use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, bail};
use chrono::Utc;
use serde_json::{Value, json};

use crate::config::DxConfig;

use super::super::serializer_artifacts::{
    ensure_dx_machine_artifact, sr_bool, sr_number, sr_string, sr_string_array, write_sr_artifact,
};
use super::{
    PublicToolFormat, PublicToolReport, collect_files, normalize_path, normalize_relative_path,
    parse_subcommand_options, public_report, write_json_receipt,
};

const IMPORT_SCAN_EXTENSIONS: [&str; 4] = ["ts", "tsx", "jsx", "js"];
const IMPORT_USAGE_EXTENSIONS: [&str; 6] = ["ts", "tsx", "jsx", "js", "mdx", "html"];
const DEFAULT_DECLARATIONS_PATH: &str = ".dx/imports/imports.d.ts";

#[derive(Debug, Clone)]
struct ImportsToolConfig {
    map: String,
    barrel: String,
    declarations: String,
    scan_roots: Vec<String>,
    used_roots: Vec<String>,
    aliases: Vec<String>,
    used_only: bool,
}

#[derive(Debug)]
struct ImportMap {
    config: ImportsToolConfig,
    entries: Vec<ImportEntry>,
    source_hash: String,
    used_symbol_count: usize,
    typed_entry_count: usize,
    untyped_entry_count: usize,
    skipped_entries: Vec<SkippedImportEntry>,
}

#[derive(Debug, Clone)]
struct ImportEntry {
    kind: String,
    name: String,
    source: String,
    import_path: String,
    barrel_export: Option<String>,
    exports: Vec<ExportSymbol>,
    default_export_name: Option<String>,
    used_exports: Vec<String>,
    unused_exports: Vec<String>,
}

#[derive(Debug, Clone)]
struct ExportSymbol {
    name: String,
    type_only: bool,
    is_default: bool,
}

#[derive(Debug)]
struct SkippedImportEntry {
    source: String,
    reason: String,
}

pub(crate) fn run_dx_imports(project: &Path, args: &[String]) -> anyhow::Result<PublicToolReport> {
    let (command, options) = parse_subcommand_options(args, "imports", "check")?;
    ensure_dx_machine_artifact(project)?;
    match command.as_str() {
        "sync" => sync_dx_imports(project, options.format),
        "check" => check_dx_imports(project, options.format),
        other => bail!("Unknown dx imports command: {other}"),
    }
}

pub(super) fn sync_dx_imports(
    project: &Path,
    format: PublicToolFormat,
) -> anyhow::Result<PublicToolReport> {
    let project = imports_project_root(project)?;
    let imports = build_import_map(&project)?;
    write_import_map(&project, &imports)?;
    let report = imports_report(&project, &imports, true);
    write_json_receipt(&project.join(".dx/receipts/imports/sync.json"), &report)?;
    write_imports_sync_sr(&project, &imports, &report)?;

    Ok(public_report(
        format,
        "DX imports sync",
        &report,
        &format!(
            "DX imports sync\nEntries: {}\nPublic barrel exports: {}\nDeclarations: {}\nImport map: {}\n",
            imports.entries.len(),
            public_barrel_entries(&imports).len(),
            imports.config.declarations,
            imports.config.map
        ),
    ))
}

pub(super) fn check_dx_imports(
    project: &Path,
    format: PublicToolFormat,
) -> anyhow::Result<PublicToolReport> {
    let project = imports_project_root(project)?;
    let imports = build_import_map(&project)?;
    let expected_barrel = imports_barrel(&imports);
    let expected_json = serde_json::to_string_pretty(&imports_json(&project, &imports))?;
    let expected_declarations = imports_declarations(&project, &imports);

    let current_barrel = read_project_file(&project, &imports.config.barrel);
    let current_json = read_project_file(&project, &imports.config.map);
    let current_declarations = read_project_file(&project, &imports.config.declarations);

    let stale_barrel = current_barrel != expected_barrel;
    let stale_map = current_json != expected_json;
    let stale_declarations = current_declarations != expected_declarations;
    let stale_sync_receipt = sync_receipt_is_stale(&project, &imports);
    let passed = !stale_barrel && !stale_map && !stale_declarations && !stale_sync_receipt;

    let mut report = imports_report(&project, &imports, false);
    report["passed"] = json!(passed);
    report["stale_barrel"] = json!(stale_barrel);
    report["stale_import_map"] = json!(stale_map);
    report["stale_declarations"] = json!(stale_declarations);
    report["stale_sync_receipt"] = json!(stale_sync_receipt);
    report["next_command"] = json!("dx imports sync");
    report["checked_files"] = json!([
        &imports.config.barrel,
        &imports.config.map,
        &imports.config.declarations,
        ".dx/imports/sync.sr"
    ]);

    write_json_receipt(&project.join(".dx/receipts/imports/check.json"), &report)?;
    write_imports_check_sr(&project, &imports, &report)?;

    Ok(public_report(
        format,
        "DX imports check",
        &report,
        &format!(
            "DX imports check\nPassed: {passed}\nStale barrel: {stale_barrel}\nStale import map: {stale_map}\nStale declarations: {stale_declarations}\nStale sync receipt: {stale_sync_receipt}\n"
        ),
    ))
}

fn imports_project_root(project: &Path) -> anyhow::Result<PathBuf> {
    let canonical = project.canonicalize().with_context(|| {
        format!(
            "resolve dx imports project root {}",
            normalize_path(project)
        )
    })?;
    Ok(strip_windows_verbatim_prefix(canonical))
}

fn strip_windows_verbatim_prefix(path: PathBuf) -> PathBuf {
    let raw = path.to_string_lossy();
    if let Some(rest) = raw.strip_prefix(r"\\?\UNC\") {
        return PathBuf::from(format!(r"\\{rest}"));
    }
    if let Some(rest) = raw.strip_prefix(r"\\?\") {
        return PathBuf::from(rest);
    }
    path
}

pub(crate) fn ensure_dx_imports_current_for_build(project: &Path) -> anyhow::Result<()> {
    let report = check_dx_imports(project, PublicToolFormat::Json)?.json;
    if report.get("passed").and_then(Value::as_bool) == Some(true) {
        return Ok(());
    }

    println!("Imports are stale or missing cache, automatically syncing...");
    sync_dx_imports(project, PublicToolFormat::Terminal)?;
    
    Ok(())
}

fn build_import_map(project: &Path) -> anyhow::Result<ImportMap> {
    let config = load_imports_tool_config(project);
    let usage = collect_used_symbols(project, &config)?;
    let mut entries = Vec::new();
    let mut skipped_entries = Vec::new();

    for root in &config.scan_roots {
        let root_path = project.join(root);
        if !root_path.exists() {
            continue;
        }
        for file in collect_files(&root_path, &IMPORT_SCAN_EXTENSIONS)? {
            if should_skip_import_source(project, &config, &file) {
                continue;
            }
            match import_entry_from_file(project, &config, root, &file, &usage)? {
                Some(entry) => entries.push(entry),
                None => skipped_entries.push(SkippedImportEntry {
                    source: normalize_relative_path(project, &file),
                    reason: "no public exports found".to_string(),
                }),
            }
        }
    }

    entries.extend(forge_package_entries(project));
    entries.extend(style_helper_entries(project));
    entries.sort_by(import_entry_order);
    entries.dedup_by(|left, right| {
        left.kind == right.kind
            && left.source == right.source
            && left.import_path == right.import_path
    });

    let typed_entry_count = entries
        .iter()
        .filter(|entry| !entry.exports.is_empty() || entry.default_export_name.is_some())
        .count();
    let untyped_entry_count = entries.len().saturating_sub(typed_entry_count);
    let used_symbol_count = entries
        .iter()
        .map(|entry| entry.used_exports.len())
        .sum::<usize>();
    let source_hash = import_map_source_hash(&entries, &config);

    Ok(ImportMap {
        config,
        entries,
        source_hash,
        used_symbol_count,
        typed_entry_count,
        untyped_entry_count,
        skipped_entries,
    })
}

fn load_imports_tool_config(project: &Path) -> ImportsToolConfig {
    let config = DxConfig::load_project(project).unwrap_or_default();
    let imports = config.tooling.imports;
    let mut aliases = normalize_aliases(imports.aliases);
    if aliases.is_empty() {
        aliases = vec!["#imports".to_string(), "#components".to_string()];
    }

    ImportsToolConfig {
        map: imports.map,
        barrel: imports.barrel,
        declarations: if imports.declarations.trim().is_empty() {
            DEFAULT_DECLARATIONS_PATH.to_string()
        } else {
            imports.declarations
        },
        scan_roots: normalize_roots(imports.scan_roots, &["components", "composables", "utils"]),
        used_roots: normalize_roots(
            imports.used_roots,
            &["app", "components", "lib", "server", "styles"],
        ),
        aliases,
        used_only: imports.used_only,
    }
}

fn normalize_roots(values: Vec<String>, defaults: &[&str]) -> Vec<String> {
    let source = if values.is_empty() {
        defaults.iter().map(|value| (*value).to_string()).collect()
    } else {
        values
    };
    let mut roots = source
        .into_iter()
        .flat_map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|part| !part.is_empty())
                .map(|part| part.trim_matches(['[', ']', '"']).replace('\\', "/"))
                .collect::<Vec<_>>()
        })
        .filter(|value| is_safe_project_relative(value))
        .collect::<Vec<_>>();
    roots.sort();
    roots.dedup();
    roots
}

fn normalize_aliases(values: Vec<String>) -> Vec<String> {
    let mut aliases = values
        .into_iter()
        .flat_map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|part| !part.is_empty())
                .map(|part| part.trim_matches(['[', ']', '"']).to_string())
                .collect::<Vec<_>>()
        })
        .filter(|value| value.starts_with('#') && value.len() > 1)
        .collect::<Vec<_>>();
    aliases.sort();
    aliases.dedup();
    aliases
}

fn import_entry_from_file(
    project: &Path,
    config: &ImportsToolConfig,
    root: &str,
    file: &Path,
    usage: &BTreeMap<String, BTreeSet<String>>,
) -> anyhow::Result<Option<ImportEntry>> {
    let source = std::fs::read_to_string(file)
        .with_context(|| format!("read auto-import source {}", file.display()))?;
    let exports = public_exports_from_source(&source);
    if exports.is_empty() {
        return Ok(None);
    }

    let source_path = normalize_relative_path(project, file);
    let mut default_export_name = exports.iter().find(|symbol| symbol.is_default).map(|_| {
        pascal_case(
            file.file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("DxImport"),
        )
    });
    let public_names = exports
        .iter()
        .filter(|symbol| !symbol.is_default)
        .map(|symbol| symbol.name.clone())
        .collect::<Vec<_>>();
    let name = public_names
        .iter()
        .find(|name| !name.ends_with("Props"))
        .cloned()
        .or_else(|| public_names.first().cloned())
        .or_else(|| default_export_name.clone())
        .unwrap_or_else(|| "DxImport".to_string());

    if default_export_name.as_deref() == Some("Default") {
        default_export_name = Some(name.clone());
    }

    let mut candidate_exports = public_names.clone();
    if let Some(default_name) = &default_export_name {
        candidate_exports.push(default_name.clone());
    }
    candidate_exports.sort();
    candidate_exports.dedup();

    let used_exports = if config.used_only {
        candidate_exports
            .iter()
            .filter(|name| usage_symbol_is_used(usage, name, &source_path))
            .cloned()
            .collect::<Vec<_>>()
    } else {
        candidate_exports.clone()
    };
    let unused_exports = candidate_exports
        .iter()
        .filter(|name| !used_exports.contains(name))
        .cloned()
        .collect::<Vec<_>>();
    let barrel_export = if used_exports.is_empty() {
        None
    } else {
        Some(relative_module_path(
            project,
            &project.join(&config.barrel),
            file,
        ))
    };

    Ok(Some(ImportEntry {
        kind: import_kind(root, &source_path).to_string(),
        name,
        source: source_path,
        import_path: format!(
            "@/{}",
            strip_known_extension(&normalize_relative_path(project, file))
        ),
        barrel_export,
        exports,
        default_export_name,
        used_exports,
        unused_exports,
    }))
}

fn import_kind(root: &str, source: &str) -> &'static str {
    if source.starts_with("components/icons/") {
        "dx-icon"
    } else if root == "components" || root.starts_with("components/") {
        "component"
    } else if root == "composables" || root.starts_with("composables/") {
        "composable"
    } else {
        "utility"
    }
}

fn collect_used_symbols(
    project: &Path,
    config: &ImportsToolConfig,
) -> anyhow::Result<BTreeMap<String, BTreeSet<String>>> {
    let mut usage = BTreeMap::<String, BTreeSet<String>>::new();
    for root in &config.used_roots {
        let root_path = project.join(root);
        if !root_path.exists() {
            continue;
        }
        for file in collect_files(&root_path, &IMPORT_USAGE_EXTENSIONS)? {
            if should_skip_usage_source(project, config, &file) {
                continue;
            }
            let source = std::fs::read_to_string(&file)
                .with_context(|| format!("read auto-import usage source {}", file.display()))?;
            let normalized = normalize_relative_path(project, &file);
            for token in identifier_tokens(&source) {
                usage.entry(token).or_default().insert(normalized.clone());
            }
        }
    }
    Ok(usage)
}

fn usage_symbol_is_used(
    usage: &BTreeMap<String, BTreeSet<String>>,
    name: &str,
    source: &str,
) -> bool {
    usage
        .get(name)
        .map(|sources| sources.iter().any(|usage_source| usage_source != source))
        .unwrap_or(false)
}

fn identifier_tokens(source: &str) -> BTreeSet<String> {
    let mut tokens = BTreeSet::new();
    let mut current = String::new();
    for ch in source.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' {
            current.push(ch);
        } else if !current.is_empty() {
            if is_identifier_like(&current) {
                tokens.insert(current.clone());
            }
            current.clear();
        }
    }
    if !current.is_empty() && is_identifier_like(&current) {
        tokens.insert(current);
    }
    tokens
}

fn is_identifier_like(value: &str) -> bool {
    value
        .chars()
        .next()
        .is_some_and(|ch| ch.is_ascii_alphabetic() || ch == '_' || ch == '$')
}

fn forge_package_entries(project: &Path) -> Vec<ImportEntry> {
    let mut entries = Vec::new();
    let manifest_path = project.join(".dx/forge/source-manifest.json");
    let Ok(bytes) = std::fs::read(&manifest_path) else {
        return entries;
    };
    let Ok(value) = serde_json::from_slice::<Value>(&bytes) else {
        return entries;
    };
    let Some(packages) = value.get("packages").and_then(Value::as_array) else {
        return entries;
    };

    for package in packages {
        let Some(package_id) = package.get("package_id").and_then(Value::as_str) else {
            continue;
        };
        let source = package
            .get("source")
            .and_then(Value::as_str)
            .map(str::to_string)
            .unwrap_or_else(|| format!("forge/{package_id}"));
        let exports = package
            .get("exports")
            .and_then(Value::as_array)
            .map(|exports| {
                exports
                    .iter()
                    .filter_map(Value::as_str)
                    .map(|name| ExportSymbol {
                        name: name.to_string(),
                        type_only: false,
                        is_default: false,
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        entries.push(ImportEntry {
            kind: "forge-package".to_string(),
            name: package_id.to_string(),
            source,
            import_path: format!("@/forge/{package_id}"),
            barrel_export: None,
            exports,
            default_export_name: None,
            used_exports: Vec::new(),
            unused_exports: Vec::new(),
        });
    }
    entries
}

fn style_helper_entries(project: &Path) -> Vec<ImportEntry> {
    ["styles/theme.css", "styles/generated.css"]
        .into_iter()
        .filter(|path| project.join(path).is_file())
        .map(|path| ImportEntry {
            kind: "style-helper".to_string(),
            name: path.rsplit('/').next().unwrap_or(path).to_string(),
            source: path.to_string(),
            import_path: format!("@/{path}"),
            barrel_export: None,
            exports: Vec::new(),
            default_export_name: None,
            used_exports: Vec::new(),
            unused_exports: Vec::new(),
        })
        .collect()
}

fn write_import_map(project: &Path, imports: &ImportMap) -> anyhow::Result<()> {
    write_project_file(project, &imports.config.barrel, &imports_barrel(imports))?;
    write_project_file(
        project,
        &imports.config.map,
        &serde_json::to_string_pretty(&imports_json(project, imports))?,
    )?;
    write_project_file(
        project,
        &imports.config.declarations,
        &imports_declarations(project, imports),
    )?;
    Ok(())
}

fn imports_barrel(imports: &ImportMap) -> String {
    let mut output = String::from("/* Generated by dx imports sync. */\n");
    output
        .push_str("/* Keep source files editable; regenerate after moves or export changes. */\n");
    for entry in public_barrel_entries(imports) {
        if let Some(export_path) = &entry.barrel_export {
            output.push_str(&format!("export * from \"{export_path}\";\n"));
        }
    }
    output.push_str("\nexport const dxAutoImportMap = ");
    output.push_str(
        &serde_json::to_string_pretty(&imports_barrel_metadata(imports)).unwrap_or_default(),
    );
    output.push_str(" as const;\n\n");
    output.push_str(
        "export type DxAutoImportComponent = typeof dxAutoImportMap.components[number][\"name\"];\n",
    );
    output.push_str(
        "export type DxAutoImportComposable = typeof dxAutoImportMap.composables[number][\"name\"];\n",
    );
    output.push_str(
        "export type DxAutoImportUtility = typeof dxAutoImportMap.utilities[number][\"name\"];\n",
    );
    output.push_str(
        "export type DxAutoImportForgePackage = typeof dxAutoImportMap.forgePackages[number][\"packageId\"];\n",
    );
    output.push_str(
        "export type DxAutoImportStyleHelper = typeof dxAutoImportMap.styleHelpers[number][\"name\"];\n",
    );
    output
}

fn imports_declarations(project: &Path, imports: &ImportMap) -> String {
    let declaration_file = project.join(&imports.config.declarations);
    let mut output = String::from("/* Generated by dx imports sync. */\n");
    output.push_str("/* Source-owned IDE contract for DX WWW auto-imports. */\n\n");
    for alias in &imports.config.aliases {
        output.push_str(&format!("declare module \"{alias}\" {{\n"));
        for entry in public_barrel_entries(imports) {
            write_declaration_exports(
                project,
                &declaration_file,
                entry,
                &mut output,
                alias == "#components",
            );
        }
        output.push_str("}\n\n");
    }

    output.push_str("declare global {\n");
    for entry in public_barrel_entries(imports) {
        let module_path =
            relative_module_path(project, &declaration_file, &project.join(&entry.source));
        for symbol in runtime_export_symbols(entry) {
            output.push_str(&format!(
                "  const {}: typeof import(\"{}\")[\"{}\"];\n",
                symbol.name, module_path, symbol.name
            ));
        }
        if let Some(default_name) = &entry.default_export_name {
            if entry.used_exports.contains(default_name) {
                output.push_str(&format!(
                    "  const {}: typeof import(\"{}\")[\"default\"];\n",
                    default_name, module_path
                ));
            }
        }
    }
    output.push_str("}\n\nexport {};\n");
    output
}

fn write_declaration_exports(
    project: &Path,
    declaration_file: &Path,
    entry: &ImportEntry,
    output: &mut String,
    components_only: bool,
) {
    if components_only && !matches!(entry.kind.as_str(), "component" | "dx-icon") {
        return;
    }
    let module_path = relative_module_path(project, declaration_file, &project.join(&entry.source));
    let runtime_names = runtime_export_symbols(entry)
        .into_iter()
        .filter(|symbol| entry.used_exports.contains(&symbol.name))
        .map(|symbol| symbol.name)
        .collect::<Vec<_>>();
    let type_names = entry
        .exports
        .iter()
        .filter(|symbol| symbol.type_only && entry.used_exports.contains(&symbol.name))
        .map(|symbol| symbol.name.clone())
        .collect::<Vec<_>>();

    if !runtime_names.is_empty() {
        output.push_str(&format!(
            "  export {{ {} }} from \"{}\";\n",
            runtime_names.join(", "),
            module_path
        ));
    }
    if !type_names.is_empty() {
        output.push_str(&format!(
            "  export type {{ {} }} from \"{}\";\n",
            type_names.join(", "),
            module_path
        ));
    }
    if let Some(default_name) = &entry.default_export_name {
        if entry.used_exports.contains(default_name) {
            output.push_str(&format!(
                "  export {{ default as {} }} from \"{}\";\n",
                default_name, module_path
            ));
        }
    }
}

fn imports_json(project: &Path, imports: &ImportMap) -> Value {
    json!({
        "tool": "dx imports",
        "version": 2,
        "generated_by": "dx imports sync",
        "project": normalize_path(project),
        "source_hash": &imports.source_hash,
        "policy": "generated-import-maps-no-runtime-magic",
        "used_only": imports.config.used_only,
        "scan_roots": &imports.config.scan_roots,
        "used_roots": &imports.config.used_roots,
        "aliases": &imports.config.aliases,
        "files": {
            "barrel": &imports.config.barrel,
            "import_map": &imports.config.map,
            "declarations": &imports.config.declarations
        },
        "entry_count": imports.entries.len(),
        "used_symbol_count": imports.used_symbol_count,
        "typed_entry_count": imports.typed_entry_count,
        "untyped_entry_count": imports.untyped_entry_count,
        "entries": imports.entries.iter().map(import_entry_json).collect::<Vec<_>>(),
        "public_barrel_entries": public_barrel_entries(imports).iter().map(|entry| &entry.source).collect::<Vec<_>>(),
        "unused_entries": imports.entries.iter().filter(|entry| entry.barrel_export.is_none() && matches!(entry.kind.as_str(), "component" | "dx-icon" | "composable" | "utility")).map(|entry| &entry.source).collect::<Vec<_>>(),
        "skipped_entries": imports.skipped_entries.iter().map(skipped_entry_json).collect::<Vec<_>>(),
        "component_exports": public_barrel_entries(imports).iter().filter_map(|entry| entry.barrel_export.as_ref()).collect::<Vec<_>>(),
        "components": imports.entries.iter().filter(|entry| matches!(entry.kind.as_str(), "component" | "dx-icon")).map(import_entry_barrel_json).collect::<Vec<_>>(),
        "composables": imports.entries.iter().filter(|entry| entry.kind == "composable").map(import_entry_barrel_json).collect::<Vec<_>>(),
        "utilities": imports.entries.iter().filter(|entry| entry.kind == "utility").map(import_entry_barrel_json).collect::<Vec<_>>(),
        "forge_packages": imports.entries.iter().filter(|entry| entry.kind == "forge-package").map(|entry| &entry.name).collect::<Vec<_>>(),
        "style_helpers": imports.entries.iter().filter(|entry| entry.kind == "style-helper").map(|entry| &entry.source).collect::<Vec<_>>()
    })
}

fn imports_barrel_metadata(imports: &ImportMap) -> Value {
    json!({
        "components": imports.entries.iter().filter(|entry| matches!(entry.kind.as_str(), "component" | "dx-icon")).map(import_entry_json).collect::<Vec<_>>(),
        "composables": imports.entries.iter().filter(|entry| entry.kind == "composable").map(import_entry_json).collect::<Vec<_>>(),
        "utilities": imports.entries.iter().filter(|entry| entry.kind == "utility").map(import_entry_json).collect::<Vec<_>>(),
        "forgePackages": imports.entries.iter().filter(|entry| entry.kind == "forge-package").map(|entry| json!({
            "packageId": &entry.name,
            "source": &entry.source,
            "importPath": &entry.import_path,
            "exports": export_names(&entry.exports)
        })).collect::<Vec<_>>(),
        "styleHelpers": imports.entries.iter().filter(|entry| entry.kind == "style-helper").map(|entry| json!({
            "name": &entry.name,
            "source": &entry.source,
            "importPath": &entry.import_path
        })).collect::<Vec<_>>(),
        "sourceHash": imports.source_hash
    })
}

fn imports_report(project: &Path, imports: &ImportMap, wrote: bool) -> Value {
    json!({
        "tool": if wrote { "dx imports sync" } else { "dx imports check" },
        "version": 2,
        "generated_at": Utc::now().to_rfc3339(),
        "project": normalize_path(project),
        "passed": true,
        "wrote_files": wrote,
        "source_hash": &imports.source_hash,
        "entry_count": imports.entries.len(),
        "public_barrel_entry_count": public_barrel_entries(imports).len(),
        "used_symbol_count": imports.used_symbol_count,
        "typed_entry_count": imports.typed_entry_count,
        "untyped_entry_count": imports.untyped_entry_count,
        "skipped_entry_count": imports.skipped_entries.len(),
        "scan_roots": &imports.config.scan_roots,
        "used_roots": &imports.config.used_roots,
        "aliases": &imports.config.aliases,
        "used_only": imports.config.used_only,
        "files": {
            "barrel": &imports.config.barrel,
            "import_map": &imports.config.map,
            "declarations": &imports.config.declarations
        },
        "receipts": {
            "sync_sr": ".dx/imports/sync.sr",
            "check_sr": ".dx/imports/check.sr",
            "legacy_json": if wrote { ".dx/receipts/imports/sync.json" } else { ".dx/receipts/imports/check.json" }
        }
    })
}

fn write_imports_sync_sr(
    project: &Path,
    imports: &ImportMap,
    report: &Value,
) -> anyhow::Result<()> {
    write_sr_artifact(
        project,
        ".dx/imports/sync.sr",
        &[
            ("tool", sr_string("dx imports")),
            ("command", sr_string("sync")),
            ("passed", sr_bool(true)),
            ("source_hash", sr_string(&imports.source_hash)),
            ("entry_count", sr_number(imports.entries.len())),
            (
                "public_barrel_entry_count",
                sr_number(public_barrel_entries(imports).len()),
            ),
            ("used_symbol_count", sr_number(imports.used_symbol_count)),
            ("typed_entry_count", sr_number(imports.typed_entry_count)),
            (
                "untyped_entry_count",
                sr_number(imports.untyped_entry_count),
            ),
            (
                "skipped_entry_count",
                sr_number(imports.skipped_entries.len()),
            ),
            ("scan_roots", sr_string_array(&imports.config.scan_roots)),
            ("used_roots", sr_string_array(&imports.config.used_roots)),
            ("aliases", sr_string_array(&imports.config.aliases)),
            ("barrel", sr_string(&imports.config.barrel)),
            ("import_map", sr_string(&imports.config.map)),
            ("declarations", sr_string(&imports.config.declarations)),
            (
                "legacy_json",
                sr_string(
                    report["receipts"]["legacy_json"]
                        .as_str()
                        .unwrap_or(".dx/receipts/imports/sync.json"),
                ),
            ),
        ],
    )?;
    Ok(())
}

fn write_imports_check_sr(
    project: &Path,
    imports: &ImportMap,
    report: &Value,
) -> anyhow::Result<()> {
    write_sr_artifact(
        project,
        ".dx/imports/check.sr",
        &[
            ("tool", sr_string("dx imports")),
            ("command", sr_string("check")),
            (
                "passed",
                sr_bool(report["passed"].as_bool().unwrap_or(false)),
            ),
            ("source_hash", sr_string(&imports.source_hash)),
            (
                "stale_barrel",
                sr_bool(report["stale_barrel"].as_bool().unwrap_or(false)),
            ),
            (
                "stale_import_map",
                sr_bool(report["stale_import_map"].as_bool().unwrap_or(false)),
            ),
            (
                "stale_declarations",
                sr_bool(report["stale_declarations"].as_bool().unwrap_or(false)),
            ),
            (
                "stale_sync_receipt",
                sr_bool(report["stale_sync_receipt"].as_bool().unwrap_or(false)),
            ),
            ("barrel", sr_string(&imports.config.barrel)),
            ("import_map", sr_string(&imports.config.map)),
            ("declarations", sr_string(&imports.config.declarations)),
            ("legacy_json", sr_string(".dx/receipts/imports/check.json")),
        ],
    )?;
    Ok(())
}

fn sync_receipt_is_stale(project: &Path, imports: &ImportMap) -> bool {
    let content = read_project_file(project, ".dx/imports/sync.sr");
    content.is_empty()
        || !content.contains(&format!("source_hash={}", sr_string(&imports.source_hash)))
        || !content.contains(&format!("entry_count={}", imports.entries.len()))
        || !content.contains(&format!(
            "public_barrel_entry_count={}",
            public_barrel_entries(imports).len()
        ))
}

fn import_entry_json(entry: &ImportEntry) -> Value {
    json!({
        "kind": &entry.kind,
        "name": &entry.name,
        "source": &entry.source,
        "import_path": &entry.import_path,
        "barrel_export": entry.barrel_export.as_ref(),
        "exports": export_names(&entry.exports),
        "default_export_name": entry.default_export_name.as_ref(),
        "used_exports": &entry.used_exports,
        "unused_exports": &entry.unused_exports,
        "type_exports": entry.exports.iter().filter(|symbol| symbol.type_only).map(|symbol| &symbol.name).collect::<Vec<_>>()
    })
}

fn import_entry_barrel_json(entry: &ImportEntry) -> Value {
    json!({
        "kind": &entry.kind,
        "name": &entry.name,
        "source": &entry.source,
        "importPath": &entry.import_path,
        "barrelExport": entry.barrel_export.as_ref(),
        "exports": export_names(&entry.exports),
        "defaultExportName": entry.default_export_name.as_ref(),
        "usedExports": &entry.used_exports,
        "unusedExports": &entry.unused_exports,
        "typeExports": entry.exports.iter().filter(|symbol| symbol.type_only).map(|symbol| &symbol.name).collect::<Vec<_>>()
    })
}

fn skipped_entry_json(entry: &SkippedImportEntry) -> Value {
    json!({
        "source": &entry.source,
        "reason": &entry.reason
    })
}

fn public_barrel_entries(imports: &ImportMap) -> Vec<&ImportEntry> {
    imports
        .entries
        .iter()
        .filter(|entry| entry.barrel_export.is_some())
        .collect()
}

fn runtime_export_symbols(entry: &ImportEntry) -> Vec<ExportSymbol> {
    entry
        .exports
        .iter()
        .filter(|symbol| !symbol.type_only && !symbol.is_default)
        .cloned()
        .collect()
}

fn export_names(exports: &[ExportSymbol]) -> Vec<String> {
    exports.iter().map(|symbol| symbol.name.clone()).collect()
}

fn public_exports_from_source(source: &str) -> Vec<ExportSymbol> {
    let mut exports = BTreeMap::<String, ExportSymbol>::new();
    for raw_line in source.lines() {
        let line = raw_line.trim_start();
        if let Some(name) = exported_identifier(line, "export function ") {
            insert_export(&mut exports, name, false, false);
        } else if let Some(name) = exported_identifier(line, "export async function ") {
            insert_export(&mut exports, name, false, false);
        } else if let Some(name) = exported_identifier(line, "export const ") {
            insert_export(&mut exports, name, false, false);
        } else if let Some(name) = exported_identifier(line, "export let ") {
            insert_export(&mut exports, name, false, false);
        } else if let Some(name) = exported_identifier(line, "export class ") {
            insert_export(&mut exports, name, false, false);
        } else if let Some(name) = exported_identifier(line, "export type ") {
            insert_export(&mut exports, name, true, false);
        } else if let Some(name) = exported_identifier(line, "export interface ") {
            insert_export(&mut exports, name, true, false);
        } else if let Some(name) = exported_identifier(line, "export enum ") {
            insert_export(&mut exports, name, false, false);
        } else if let Some(name) = exported_identifier(line, "export default function ") {
            insert_export(&mut exports, name, false, true);
        } else if let Some(name) = exported_identifier(line, "export default class ") {
            insert_export(&mut exports, name, false, true);
        } else if line.starts_with("export default ") {
            insert_export(&mut exports, "default".to_string(), false, true);
        } else if line.starts_with("export {") {
            for symbol in named_export_list(line) {
                insert_export(&mut exports, symbol.name, symbol.type_only, false);
            }
        }
    }
    exports.into_values().collect()
}

fn insert_export(
    exports: &mut BTreeMap<String, ExportSymbol>,
    name: String,
    type_only: bool,
    is_default: bool,
) {
    exports.insert(
        name.clone(),
        ExportSymbol {
            name,
            type_only,
            is_default,
        },
    );
}

fn exported_identifier(line: &str, prefix: &str) -> Option<String> {
    let tail = line.strip_prefix(prefix)?.trim_start();
    let identifier = tail
        .chars()
        .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_' || *ch == '$')
        .collect::<String>();
    if identifier.is_empty() {
        None
    } else {
        Some(identifier)
    }
}

fn named_export_list(line: &str) -> Vec<ExportSymbol> {
    let Some(start) = line.find('{') else {
        return Vec::new();
    };
    let Some(end) = line[start + 1..].find('}') else {
        return Vec::new();
    };
    line[start + 1..start + 1 + end]
        .split(',')
        .filter_map(|part| {
            let token = part.trim();
            if token.is_empty() {
                return None;
            }
            let (type_only, token) = token
                .strip_prefix("type ")
                .map(|value| (true, value.trim()))
                .unwrap_or((false, token));
            let public_name = token
                .rsplit_once(" as ")
                .map(|(_, alias)| alias.trim())
                .unwrap_or(token)
                .split_whitespace()
                .next()
                .unwrap_or_default();
            if public_name.is_empty() {
                None
            } else {
                Some(ExportSymbol {
                    name: public_name.to_string(),
                    type_only,
                    is_default: false,
                })
            }
        })
        .collect()
}

fn import_map_source_hash(entries: &[ImportEntry], config: &ImportsToolConfig) -> String {
    let mut input = String::new();
    let aliases = config.aliases.join(",");
    let scan_roots = config.scan_roots.join(",");
    let used_roots = config.used_roots.join(",");
    for value in [
        config.map.as_str(),
        config.barrel.as_str(),
        config.declarations.as_str(),
        aliases.as_str(),
        scan_roots.as_str(),
        used_roots.as_str(),
        if config.used_only {
            "used-only"
        } else {
            "catalog"
        },
    ] {
        input.push_str(value);
        input.push('\n');
    }
    for entry in entries {
        input.push_str(&entry.kind);
        input.push('\n');
        input.push_str(&entry.name);
        input.push('\n');
        input.push_str(&entry.source);
        input.push('\n');
        input.push_str(&entry.import_path);
        input.push('\n');
        input.push_str(&entry.used_exports.join(","));
        input.push('\n');
        for export in &entry.exports {
            input.push_str(&export.name);
            input.push(':');
            input.push_str(if export.type_only { "type" } else { "runtime" });
            input.push(':');
            input.push_str(if export.is_default {
                "default"
            } else {
                "named"
            });
            input.push('\n');
        }
    }
    format!(
        "blake3:{}",
        blake3::hash(input.as_bytes()).to_hex().as_str()
    )
}

fn import_entry_order(left: &ImportEntry, right: &ImportEntry) -> std::cmp::Ordering {
    (
        left.kind.as_str(),
        left.source.as_str(),
        left.name.as_str(),
        left.import_path.as_str(),
    )
        .cmp(&(
            right.kind.as_str(),
            right.source.as_str(),
            right.name.as_str(),
            right.import_path.as_str(),
        ))
}

fn should_skip_import_source(project: &Path, config: &ImportsToolConfig, file: &Path) -> bool {
    let relative = normalize_relative_path(project, file);
    relative == config.barrel
        || relative == config.map
        || relative == config.declarations
        || relative.ends_with(".d.ts")
}

fn should_skip_usage_source(project: &Path, config: &ImportsToolConfig, file: &Path) -> bool {
    should_skip_import_source(project, config, file)
        || normalize_relative_path(project, file).starts_with(".dx/")
}

fn read_project_file(project: &Path, relative: &str) -> String {
    std::fs::read_to_string(project.join(relative)).unwrap_or_default()
}

fn write_project_file(project: &Path, relative: &str, content: &str) -> anyhow::Result<()> {
    let path = project.join(relative);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, content)?;
    Ok(())
}

fn relative_module_path(project: &Path, from_file: &Path, target_file: &Path) -> String {
    let from_dir = from_file.parent().unwrap_or(project);
    let target_no_ext = PathBuf::from(strip_known_extension(&normalize_relative_path(
        project,
        target_file,
    )));
    let from_rel = from_dir.strip_prefix(project).unwrap_or(from_dir);
    let from_components = normalized_components(from_rel);
    let target_components = normalized_components(&target_no_ext);
    let mut common = 0usize;
    while common < from_components.len()
        && common < target_components.len()
        && from_components[common] == target_components[common]
    {
        common += 1;
    }

    let mut parts = Vec::new();
    for _ in common..from_components.len() {
        parts.push("..".to_string());
    }
    parts.extend(target_components[common..].iter().cloned());
    if parts.is_empty() {
        return ".".to_string();
    }
    let mut value = parts.join("/");
    if !value.starts_with('.') {
        value = format!("./{value}");
    }
    value
}

fn normalized_components(path: &Path) -> Vec<String> {
    path.components()
        .filter_map(|component| match component {
            Component::Normal(value) => Some(value.to_string_lossy().replace('\\', "/")),
            Component::ParentDir => Some("..".to_string()),
            _ => None,
        })
        .collect()
}

fn strip_known_extension(path: &str) -> String {
    for extension in [".tsx", ".ts", ".jsx", ".js"] {
        if let Some(stripped) = path.strip_suffix(extension) {
            return stripped.to_string();
        }
    }
    path.to_string()
}

fn is_safe_project_relative(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('/')
        && !value.contains(':')
        && !value
            .split('/')
            .any(|segment| segment.is_empty() || segment == "." || segment == "..")
}

fn pascal_case(value: &str) -> String {
    let mut output = String::new();
    let mut upper_next = true;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            if upper_next {
                output.push(ch.to_ascii_uppercase());
                upper_next = false;
            } else {
                output.push(ch);
            }
        } else {
            upper_next = true;
        }
    }
    if output.is_empty() {
        "DxImport".to_string()
    } else {
        output
    }
}
