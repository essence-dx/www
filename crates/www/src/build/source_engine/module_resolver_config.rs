use std::collections::BTreeSet;
use std::path::{Component, Path, PathBuf};

use serde_json::Value;

use crate::error::{DxError, DxResult};

const MAX_CONFIG_EXTENDS_DEPTH: usize = 8;
const FORGE_SOURCE_MANIFEST_PATH: &str = ".dx/forge/source-.dx/build-cache/manifest.json";

pub(super) const RESOLVER_SOURCE_ADAPTER_BOUNDARY: &str = "adapter-boundary";
pub(super) const RESOLVER_SOURCE_BASE_URL_BOUNDARY: &str = "base-url-boundary";
pub(super) const RESOLVER_SOURCE_BASE_URL_NODE_MODULES_BOUNDARY: &str =
    "base-url-node-modules-boundary";
pub(super) const RESOLVER_SOURCE_COMPILER_INTRINSIC: &str = "compiler-intrinsic";
pub(super) const RESOLVER_SOURCE_EXTERNAL_PACKAGE_BOUNDARY: &str = "external-package-boundary";
pub(super) const RESOLVER_SOURCE_FORGE_SOURCE_MANIFEST: &str = "forge-source-manifest";
pub(super) const RESOLVER_SOURCE_JS_CONFIG_BASE_URL: &str = "jsconfig-base-url";
pub(super) const RESOLVER_SOURCE_JS_CONFIG_PATH: &str = "jsconfig-path";
pub(super) const RESOLVER_SOURCE_LOCAL_NODE_MODULES_BOUNDARY: &str = "local-node-modules-boundary";
pub(super) const RESOLVER_SOURCE_PACKAGE_EXPORT_BOUNDARY: &str = "package-export-boundary";
pub(super) const RESOLVER_SOURCE_PACKAGE_IMPORT: &str = "package-import";
pub(super) const RESOLVER_SOURCE_PACKAGE_IMPORT_BOUNDARY: &str = "package-import-boundary";
pub(super) const RESOLVER_SOURCE_PACKAGE_SELF_REFERENCE: &str = "package-self-reference";
pub(super) const RESOLVER_SOURCE_PROJECT_ROOT_ALIAS: &str = "project-root-alias";
pub(super) const RESOLVER_SOURCE_PROJECT_ROOT_ALIAS_BOUNDARY: &str = "project-root-alias-boundary";
pub(super) const RESOLVER_SOURCE_RELATIVE: &str = "relative";
pub(super) const RESOLVER_SOURCE_SRC_PROJECT_ROOT_ALIAS: &str = "src-project-root-alias";
pub(super) const RESOLVER_SOURCE_SOURCE_ALIAS_BOUNDARY: &str = "source-alias-boundary";
pub(super) const RESOLVER_SOURCE_SOURCE_ALIAS_UNRESOLVED: &str = "source-alias-unresolved";
pub(super) const RESOLVER_SOURCE_TS_CONFIG_BASE_URL: &str = "tsconfig-base-url";
pub(super) const RESOLVER_SOURCE_TS_CONFIG_PATH: &str = "tsconfig-path";

pub(super) const RESOLVER_DETAIL_BASE_URL_NODE_MODULES_BOUNDARY: &str =
    "base-url-specifier-node-modules-boundary";
pub(super) const RESOLVER_DETAIL_BASE_URL_OUTSIDE_PROJECT_BOUNDARY: &str =
    "base-url-outside-project-boundary";
pub(super) const RESOLVER_DETAIL_EXTERNAL_ADAPTER_BOUNDARY: &str = "external-adapter-boundary";
pub(super) const RESOLVER_DETAIL_EXTERNAL_PACKAGE_BOUNDARY: &str =
    "external-package-adapter-boundary";
pub(super) const RESOLVER_DETAIL_LOCAL_NODE_MODULES_BOUNDARY: &str =
    "local-import-node-modules-boundary";
pub(super) const RESOLVER_DETAIL_PACKAGE_EXPORT_BOUNDARY: &str = "package-export-adapter-boundary";
pub(super) const RESOLVER_DETAIL_PACKAGE_EXPORT_NO_SOURCE_TARGET: &str =
    "package-export-no-source-owned-target";
pub(super) const RESOLVER_DETAIL_PACKAGE_EXPORT_TARGET_NODE_MODULES_BOUNDARY: &str =
    "package-export-target-node-modules-boundary";
pub(super) const RESOLVER_DETAIL_PACKAGE_EXPORT_TARGET_OUTSIDE_PACKAGE_BOUNDARY: &str =
    "package-export-target-outside-package-boundary";
pub(super) const RESOLVER_DETAIL_PACKAGE_IMPORT_BOUNDARY: &str = "package-import-adapter-boundary";
pub(super) const RESOLVER_DETAIL_PACKAGE_IMPORT_MIXED_BOUNDARY: &str =
    "package-import-has-adapter-boundary-fallback";
pub(super) const RESOLVER_DETAIL_PACKAGE_IMPORT_NO_SOURCE_TARGET: &str =
    "package-import-no-source-owned-target";
pub(super) const RESOLVER_DETAIL_PACKAGE_IMPORT_TARGET_NODE_MODULES_BOUNDARY: &str =
    "package-import-target-node-modules-boundary";
pub(super) const RESOLVER_DETAIL_PACKAGE_IMPORT_TARGET_OUTSIDE_PACKAGE_BOUNDARY: &str =
    "package-import-target-outside-package-boundary";
pub(super) const RESOLVER_DETAIL_PROJECT_ROOT_ALIAS_NODE_MODULES_BOUNDARY: &str =
    "project-root-alias-node-modules-boundary";
pub(super) const RESOLVER_DETAIL_PROJECT_ROOT_ALIAS_OUTSIDE_PROJECT_BOUNDARY: &str =
    "project-root-alias-outside-project-boundary";
pub(super) const RESOLVER_DETAIL_SOURCE_ALIAS_BASE_URL_BOUNDARY: &str =
    "source-alias-base-url-boundary";
pub(super) const RESOLVER_DETAIL_SOURCE_ALIAS_BOUNDARY: &str = "source-alias-adapter-boundary";
pub(super) const RESOLVER_DETAIL_SOURCE_ALIAS_TARGET_NODE_MODULES_BOUNDARY: &str =
    "source-alias-target-node-modules-boundary";
pub(super) const RESOLVER_DETAIL_SOURCE_ALIAS_TARGET_OUTSIDE_PROJECT_BOUNDARY: &str =
    "source-alias-target-outside-project-boundary";
pub(super) const RESOLVER_DETAIL_SOURCE_ALIAS_UNRESOLVED: &str = "source-alias-unresolved";
pub(super) const RESOLVER_DETAIL_SOURCE_OWNED_ADAPTER_BOUNDARY: &str =
    "source-owned-adapter-boundary";

#[derive(Debug, Clone, Default)]
pub(super) struct SourceResolverConfig {
    base_urls: Vec<SourceBaseUrl>,
    path_aliases: Vec<SourcePathAlias>,
    forge_source_aliases: Vec<SourcePathAlias>,
    package_imports: Vec<SourcePathAlias>,
    package_self_references: Vec<SourcePathAlias>,
    package_self_reference_name: Option<String>,
    package_self_reference_exports_present: bool,
}

#[derive(Debug, Clone)]
struct SourceBaseUrl {
    base_url: PathBuf,
    resolver_source: &'static str,
    adapter_boundary: bool,
}

#[derive(Debug, Clone)]
struct SourcePathAlias {
    pattern: String,
    targets: Vec<String>,
    base_url: PathBuf,
    resolver_source: &'static str,
    adapter_boundary: bool,
    adapter_boundary_detail: Option<&'static str>,
}

#[derive(Debug, Clone, Default)]
struct SourcePackageImportTargets {
    targets: Vec<String>,
    adapter_boundary: bool,
    adapter_boundary_detail: Option<&'static str>,
}

#[derive(Debug)]
struct LoadedSourceConfig {
    path: PathBuf,
    source: String,
    config: Value,
    path_resolver_source: &'static str,
    base_url_resolver_source: &'static str,
}

#[derive(Debug, Clone)]
pub(super) struct SourceAliasBase {
    pub path: PathBuf,
    pub resolver_source: &'static str,
}

impl SourceResolverConfig {
    pub(super) fn load(project_root: &Path) -> DxResult<Self> {
        let mut config = Self::default();

        if let Some(config_path) = ["tsconfig.json", "jsconfig.json"]
            .into_iter()
            .map(|name| project_root.join(name))
            .find(|path| path.is_file())
        {
            let configs = read_json_config_chain(
                project_root,
                &config_path,
                path_alias_resolver_source(&config_path),
                base_url_resolver_source(&config_path),
                &mut Vec::new(),
            )?;
            for loaded in configs.into_iter().rev() {
                config.load_compiler_options(
                    project_root,
                    &loaded.path,
                    &loaded.source,
                    &loaded.config,
                    loaded.path_resolver_source,
                    loaded.base_url_resolver_source,
                )?;
            }
        }

        let package_path = project_root.join("package.json");
        if let Some((source, package_json)) = read_json_config(&package_path)? {
            config.package_imports = package_import_aliases(&package_path, &source, &package_json)?;
            config.package_self_reference_name =
                source_owned_package_name_value(&package_json).map(str::to_string);
            config.package_self_reference_exports_present = package_json.get("exports").is_some();
            config.package_self_references = package_self_reference_aliases(&package_json);
        }
        config.forge_source_aliases = forge_source_manifest_aliases(project_root)?;
        config.sort_aliases();

        Ok(config)
    }

    fn load_compiler_options(
        &mut self,
        project_root: &Path,
        config_path: &Path,
        source: &str,
        config: &Value,
        resolver_source: &'static str,
        base_url_resolver_source: &'static str,
    ) -> DxResult<()> {
        let compiler_options = config.get("compilerOptions").and_then(Value::as_object);
        let base_url = match compiler_options.and_then(|options| options.get("baseUrl")) {
            Some(Value::String(base_url)) => Some(normalize_config_base_path(
                resolve_config_relative_path(config_path, Path::new(base_url)),
            )),
            Some(_) => {
                return Err(config_parse_error(
                    config_path,
                    source,
                    "compilerOptions.baseUrl must be a string",
                ));
            }
            None => None,
        };
        if let Some(base_url) = &base_url {
            self.base_urls.push(SourceBaseUrl {
                base_url: base_url.clone(),
                resolver_source: base_url_resolver_source,
                adapter_boundary: source_path_requires_adapter_boundary(project_root, base_url),
            });
        }
        let path_alias_base_url = base_url.unwrap_or_else(|| {
            normalize_config_base_path(config_path.parent().unwrap_or_else(|| Path::new("")).into())
        });
        let path_alias_base_boundary =
            source_path_requires_adapter_boundary(project_root, &path_alias_base_url);

        match compiler_options.and_then(|options| options.get("paths")) {
            Some(Value::Object(paths)) => {
                for (pattern, targets) in paths {
                    let Some(targets) = targets.as_array() else {
                        return Err(config_parse_error(
                            config_path,
                            source,
                            "compilerOptions.paths entries must be arrays",
                        ));
                    };
                    let mut source_targets = Vec::new();
                    for target in targets {
                        let Some(target) = target.as_str() else {
                            return Err(config_parse_error(
                                config_path,
                                source,
                                "compilerOptions.paths targets must be strings",
                            ));
                        };
                        source_targets.push(target.to_string());
                    }
                    if !source_targets.is_empty() {
                        let path_alias_target_boundary = source_targets.iter().any(|target| {
                            path_alias_target_requires_adapter_boundary(
                                project_root,
                                &path_alias_base_url,
                                target,
                            )
                        });
                        self.path_aliases.push(SourcePathAlias {
                            pattern: pattern.to_string(),
                            targets: source_targets,
                            base_url: path_alias_base_url.clone(),
                            resolver_source,
                            adapter_boundary: path_alias_base_boundary
                                || path_alias_target_boundary,
                            adapter_boundary_detail: None,
                        });
                    }
                }
            }
            Some(_) => {
                return Err(config_parse_error(
                    config_path,
                    source,
                    "compilerOptions.paths must be an object",
                ));
            }
            None => {}
        }
        Ok(())
    }

    pub(super) fn source_alias_bases(
        &self,
        project_root: &Path,
        specifier: &str,
    ) -> Vec<SourceAliasBase> {
        let mut bases = Vec::new();
        let mut matched_alias_patterns = BTreeSet::new();
        for alias in self
            .path_aliases
            .iter()
            .chain(self.forge_source_aliases.iter())
            .chain(self.package_imports.iter())
            .chain(self.package_self_references.iter())
        {
            let Some(wildcard) = alias.matches(specifier) else {
                continue;
            };
            if alias.adapter_boundary {
                continue;
            }
            if !matched_alias_patterns.insert(alias.pattern.clone()) {
                continue;
            }
            for target in &alias.targets {
                bases.push(SourceAliasBase {
                    path: self.target_base(project_root, alias, target, &wildcard),
                    resolver_source: alias.resolver_source,
                });
            }
        }
        bases
    }

    pub(super) fn source_base_url_bases(
        &self,
        project_root: &Path,
        specifier: &str,
    ) -> Vec<SourceAliasBase> {
        let Some(specifier) = source_owned_base_url_specifier(specifier) else {
            return Vec::new();
        };

        self.base_urls
            .iter()
            .filter(|base_url| !base_url.adapter_boundary)
            .map(|base_url| SourceAliasBase {
                path: project_root.join(&base_url.base_url).join(specifier),
                resolver_source: base_url.resolver_source,
            })
            .collect()
    }

    pub(super) fn matches_base_url_boundary(&self, specifier: &str) -> bool {
        source_owned_base_url_specifier(specifier).is_some()
            && self
                .base_urls
                .iter()
                .any(|base_url| base_url.adapter_boundary)
    }

    pub(super) fn matches_base_url_node_modules_boundary(&self, specifier: &str) -> bool {
        !self.base_urls.is_empty() && base_url_specifier_requires_adapter_boundary(specifier)
    }

    pub(super) fn matches_source_alias(&self, specifier: &str) -> bool {
        self.path_aliases
            .iter()
            .chain(self.forge_source_aliases.iter())
            .chain(self.package_imports.iter())
            .chain(self.package_self_references.iter())
            .any(|alias| alias.matches(specifier).is_some())
    }

    pub(super) fn matches_source_alias_boundary(&self, specifier: &str) -> bool {
        self.path_aliases.iter().any(|alias| {
            alias.matches(specifier).is_some()
                && !alias.targets.is_empty()
                && alias.adapter_boundary
        })
    }

    pub(super) fn matches_package_self_reference_boundary(&self, specifier: &str) -> bool {
        if !self.matches_package_self_reference_namespace(specifier) {
            return false;
        }
        self.package_self_references
            .iter()
            .find_map(|alias| alias.matches(specifier).map(|_| alias.adapter_boundary))
            .unwrap_or(true)
    }

    pub(super) fn package_self_reference_boundary_detail(
        &self,
        specifier: &str,
    ) -> Option<&'static str> {
        if !self.matches_package_self_reference_namespace(specifier) {
            return None;
        }
        self.package_self_references
            .iter()
            .find_map(|alias| {
                (alias.matches(specifier).is_some() && alias.adapter_boundary).then(|| {
                    alias
                        .adapter_boundary_detail
                        .unwrap_or(RESOLVER_DETAIL_PACKAGE_EXPORT_BOUNDARY)
                })
            })
            .or(Some(RESOLVER_DETAIL_PACKAGE_EXPORT_BOUNDARY))
    }

    pub(super) fn source_alias_boundary_detail(
        &self,
        project_root: &Path,
        specifier: &str,
    ) -> Option<&'static str> {
        self.path_aliases.iter().find_map(|alias| {
            (alias.matches(specifier).is_some()
                && !alias.targets.is_empty()
                && alias.adapter_boundary)
                .then(|| source_alias_boundary_detail(project_root, alias))
        })
    }

    pub(super) fn matches_package_import_boundary(&self, specifier: &str) -> bool {
        self.package_imports.iter().any(|alias| {
            alias.matches(specifier).is_some()
                && (alias.targets.is_empty() || alias.adapter_boundary)
        })
    }

    pub(super) fn package_import_boundary_detail(&self, specifier: &str) -> Option<&'static str> {
        self.package_imports.iter().find_map(|alias| {
            (alias.matches(specifier).is_some()
                && (alias.targets.is_empty() || alias.adapter_boundary))
                .then(|| {
                    alias.adapter_boundary_detail.unwrap_or({
                        if alias.targets.is_empty() {
                            RESOLVER_DETAIL_PACKAGE_IMPORT_NO_SOURCE_TARGET
                        } else {
                            RESOLVER_DETAIL_PACKAGE_IMPORT_MIXED_BOUNDARY
                        }
                    })
                })
        })
    }

    pub(super) fn matches_package_self_reference_namespace(&self, specifier: &str) -> bool {
        if !self.package_self_reference_exports_present {
            return false;
        }
        let Some(name) = self.package_self_reference_name.as_deref() else {
            return false;
        };
        specifier == name
            || specifier
                .strip_prefix(name)
                .is_some_and(|rest| rest.starts_with('/'))
    }

    fn target_base(
        &self,
        project_root: &Path,
        alias: &SourcePathAlias,
        target: &str,
        wildcard: &str,
    ) -> PathBuf {
        let target = target.replace('*', wildcard);
        let target_path = Path::new(&target);
        if target_path.is_absolute() {
            target_path.to_path_buf()
        } else {
            project_root.join(&alias.base_url).join(target_path)
        }
    }

    fn sort_aliases(&mut self) {
        sort_aliases(&mut self.path_aliases);
        sort_aliases(&mut self.forge_source_aliases);
        sort_aliases(&mut self.package_imports);
        sort_aliases(&mut self.package_self_references);
    }
}

impl SourcePathAlias {
    fn matches(&self, specifier: &str) -> Option<String> {
        let Some((prefix, suffix)) = self.pattern.split_once('*') else {
            return (self.pattern == specifier).then_some(String::new());
        };
        if !specifier.starts_with(prefix) || !specifier.ends_with(suffix) {
            return None;
        }
        let wildcard_end = specifier.len().checked_sub(suffix.len())?;
        if wildcard_end < prefix.len() {
            return None;
        }
        Some(specifier[prefix.len()..wildcard_end].to_string())
    }
}

fn package_import_aliases(
    package_path: &Path,
    source: &str,
    package_json: &Value,
) -> DxResult<Vec<SourcePathAlias>> {
    let Some(imports) = package_json.get("imports") else {
        return Ok(Vec::new());
    };
    let Some(imports) = imports.as_object() else {
        return Err(config_parse_error(
            package_path,
            source,
            "package.json imports must be an object",
        ));
    };

    let mut aliases = Vec::new();
    for (pattern, targets) in imports {
        if !pattern.starts_with('#') {
            continue;
        }
        let targets = source_owned_package_import_targets_with_boundary(targets);
        let adapter_boundary = targets.adapter_boundary;
        aliases.push(SourcePathAlias {
            pattern: pattern.to_string(),
            targets: targets.targets,
            base_url: PathBuf::new(),
            resolver_source: RESOLVER_SOURCE_PACKAGE_IMPORT,
            adapter_boundary,
            adapter_boundary_detail: targets.adapter_boundary_detail,
        });
    }
    sort_aliases(&mut aliases);

    Ok(aliases)
}

fn package_self_reference_aliases(package_json: &Value) -> Vec<SourcePathAlias> {
    let Some(name) = source_owned_package_name_value(package_json) else {
        return Vec::new();
    };

    if let Some(exports) = package_json.get("exports") {
        let mut aliases = package_self_reference_export_aliases(name, Some(exports));
        sort_aliases(&mut aliases);
        return aliases;
    }

    let mut aliases = Vec::new();
    push_package_self_reference_alias(&mut aliases, name.to_string(), vec![".".to_string()]);
    push_package_self_reference_alias(&mut aliases, format!("{name}/*"), vec!["*".to_string()]);
    sort_aliases(&mut aliases);
    aliases
}

fn forge_source_manifest_aliases(project_root: &Path) -> DxResult<Vec<SourcePathAlias>> {
    let manifest_path = project_root.join(FORGE_SOURCE_MANIFEST_PATH);
    let Some((source, manifest)) = read_json_config(&manifest_path)? else {
        return Ok(Vec::new());
    };
    let Some(packages) = manifest.get("packages").and_then(Value::as_array) else {
        return Err(config_parse_error(
            &manifest_path,
            &source,
            "Forge source manifest packages must be an array",
        ));
    };

    let mut aliases = Vec::new();
    for package in packages {
        let Some(import_name) = forge_manifest_package_import_name(package) else {
            continue;
        };
        let Some(files) = package.get("files") else {
            continue;
        };
        let Some(files) = files.as_array() else {
            return Err(config_parse_error(
                &manifest_path,
                &source,
                "Forge source manifest package files must be arrays",
            ));
        };

        for source_root in forge_manifest_package_roots(project_root, files) {
            push_forge_source_manifest_alias(
                &mut aliases,
                import_name.clone(),
                source_root.clone(),
            );
            push_forge_source_manifest_alias(
                &mut aliases,
                format!("{import_name}/*"),
                format!("{source_root}/*"),
            );
        }
    }
    sort_aliases(&mut aliases);
    Ok(aliases)
}

fn forge_manifest_package_import_name(package: &Value) -> Option<String> {
    ["upstream_name", "package_id"]
        .into_iter()
        .filter_map(|field| package.get(field).and_then(Value::as_str))
        .filter_map(forge_manifest_import_name_from_value)
        .find(|name| source_owned_package_name(name))
}

fn forge_manifest_import_name_from_value(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    if let Some((ecosystem, package_name)) = value.split_once(':') {
        return forge_manifest_import_ecosystem(ecosystem)
            .then(|| package_name.trim().to_string())
            .filter(|package_name| !package_name.is_empty());
    }
    for prefix in [
        "npm/", "jsr/", "pip/", "cargo/", "go/", "pub/", "hex/", "cran/",
    ] {
        if let Some(package_name) = value.strip_prefix(prefix) {
            return (!package_name.trim().is_empty()).then(|| package_name.trim().to_string());
        }
    }
    Some(value.to_string())
}

fn forge_manifest_import_ecosystem(ecosystem: &str) -> bool {
    matches!(
        ecosystem,
        "npm" | "jsr" | "pip" | "cargo" | "go" | "pub" | "hex" | "cran"
    )
}

fn forge_manifest_package_roots(project_root: &Path, files: &[Value]) -> Vec<String> {
    let mut roots = BTreeSet::new();
    for path in files
        .iter()
        .filter_map(|file| file.get("path").and_then(Value::as_str))
    {
        let path = clean_source_path_for_boundary(Path::new(path));
        if path.is_absolute()
            || source_path_requires_adapter_boundary(project_root, &project_root.join(&path))
        {
            continue;
        }
        let normalized = normalize_source_path_for_boundary(&path);
        let segments = normalized.split('/').collect::<Vec<_>>();
        if segments.len() < 5
            || segments[0] != "lib"
            || segments[1] != "forge"
            || segments[2].is_empty()
            || segments[3].is_empty()
            || segments.contains(&"node_modules")
        {
            continue;
        }
        let root = segments[..4].join("/");
        if source_path_requires_adapter_boundary(project_root, &project_root.join(&root)) {
            continue;
        }
        roots.insert(root);
    }
    roots.into_iter().collect()
}

fn push_forge_source_manifest_alias(
    aliases: &mut Vec<SourcePathAlias>,
    pattern: String,
    target: String,
) {
    if let Some(alias) = aliases.iter_mut().find(|alias| alias.pattern == pattern) {
        if !alias.targets.iter().any(|existing| existing == &target) {
            alias.targets.push(target);
        }
        return;
    }
    aliases.push(SourcePathAlias {
        pattern,
        targets: vec![target],
        base_url: PathBuf::new(),
        resolver_source: RESOLVER_SOURCE_FORGE_SOURCE_MANIFEST,
        adapter_boundary: false,
        adapter_boundary_detail: None,
    });
}

fn package_self_reference_export_aliases(
    name: &str,
    exports: Option<&Value>,
) -> Vec<SourcePathAlias> {
    let mut aliases = Vec::new();
    let Some(exports) = exports else {
        return aliases;
    };

    match exports {
        Value::Object(exports) if exports.keys().any(|key| key.starts_with('.')) => {
            for (key, targets) in exports {
                let Some(pattern) = package_export_key_to_self_reference_pattern(name, key) else {
                    continue;
                };
                push_package_self_reference_alias_targets(
                    &mut aliases,
                    pattern,
                    source_owned_package_export_targets_with_boundary(targets),
                );
            }
        }
        _ => push_package_self_reference_alias_targets(
            &mut aliases,
            name.to_string(),
            source_owned_package_export_targets_with_boundary(exports),
        ),
    }

    aliases
}

fn push_package_self_reference_alias(
    aliases: &mut Vec<SourcePathAlias>,
    pattern: String,
    targets: Vec<String>,
) {
    push_package_self_reference_alias_targets(
        aliases,
        pattern,
        SourcePackageImportTargets {
            targets,
            adapter_boundary: false,
            adapter_boundary_detail: None,
        },
    );
}

fn push_package_self_reference_alias_targets(
    aliases: &mut Vec<SourcePathAlias>,
    pattern: String,
    targets: SourcePackageImportTargets,
) {
    if targets.targets.is_empty() && !targets.adapter_boundary {
        return;
    }

    if let Some(alias) = aliases.iter_mut().find(|alias| alias.pattern == pattern) {
        alias.adapter_boundary |= targets.adapter_boundary;
        alias.adapter_boundary_detail = preferred_package_export_boundary_detail(
            alias.adapter_boundary_detail,
            targets.adapter_boundary_detail,
        );
        for target in targets.targets {
            if !alias.targets.iter().any(|existing| existing == &target) {
                alias.targets.push(target);
            }
        }
        return;
    }

    let adapter_boundary = targets.adapter_boundary;
    aliases.push(SourcePathAlias {
        pattern,
        targets: targets.targets,
        base_url: PathBuf::new(),
        resolver_source: RESOLVER_SOURCE_PACKAGE_SELF_REFERENCE,
        adapter_boundary,
        adapter_boundary_detail: targets.adapter_boundary_detail,
    });
}

fn package_export_key_to_self_reference_pattern(name: &str, key: &str) -> Option<String> {
    if key == "." {
        return Some(name.to_string());
    }

    key.strip_prefix("./")
        .filter(|subpath| !subpath.is_empty())
        .map(|subpath| format!("{name}/{subpath}"))
}

fn path_alias_resolver_source(config_path: &Path) -> &'static str {
    if config_path
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name == "jsconfig.json")
    {
        RESOLVER_SOURCE_JS_CONFIG_PATH
    } else {
        RESOLVER_SOURCE_TS_CONFIG_PATH
    }
}

fn base_url_resolver_source(config_path: &Path) -> &'static str {
    if config_path
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name == "jsconfig.json")
    {
        RESOLVER_SOURCE_JS_CONFIG_BASE_URL
    } else {
        RESOLVER_SOURCE_TS_CONFIG_BASE_URL
    }
}

fn source_owned_package_name_value(package_json: &Value) -> Option<&str> {
    package_json
        .get("name")
        .and_then(Value::as_str)
        .filter(|name| source_owned_package_name(name))
}

fn source_owned_package_name(name: &str) -> bool {
    if name.is_empty() || name.contains('\\') || compiler_reserved_package_name(name) {
        return false;
    }
    if let Some(scoped_name) = name.strip_prefix('@') {
        let mut segments = scoped_name.split('/');
        let Some(scope) = segments.next() else {
            return false;
        };
        let Some(package) = segments.next() else {
            return false;
        };
        return segments.next().is_none()
            && source_owned_package_name_segment(scope)
            && source_owned_package_name_segment(package);
    }
    !name.contains('/') && source_owned_package_name_segment(name)
}

fn source_owned_package_name_segment(segment: &str) -> bool {
    !segment.is_empty()
        && !segment.contains('@')
        && segment != "."
        && segment != ".."
        && segment != "node_modules"
}

fn compiler_reserved_package_name(name: &str) -> bool {
    matches!(name, "dx-www" | "next" | "react") || name.starts_with("node:")
}

fn source_owned_base_url_specifier(specifier: &str) -> Option<&str> {
    if specifier.is_empty()
        || specifier.starts_with('.')
        || specifier.starts_with('/')
        || specifier.starts_with('\\')
        || specifier.starts_with('#')
        || specifier.starts_with('@')
        || specifier.contains('\\')
    {
        return None;
    }
    if specifier.split('/').any(|segment| {
        segment.is_empty() || segment == "." || segment == ".." || segment == "node_modules"
    }) {
        return None;
    }
    Some(specifier)
}

fn base_url_specifier_requires_adapter_boundary(specifier: &str) -> bool {
    if specifier.is_empty()
        || specifier.starts_with('.')
        || specifier.starts_with('/')
        || specifier.starts_with('\\')
        || specifier.starts_with('#')
        || specifier.starts_with('@')
    {
        return false;
    }
    specifier
        .replace('\\', "/")
        .split('/')
        .any(|segment| segment == "node_modules")
}

fn source_path_requires_adapter_boundary(project_root: &Path, target: &Path) -> bool {
    let target = clean_source_path_for_boundary(target);
    let project_root = clean_source_path_for_boundary(project_root);
    let target = normalize_source_path_for_boundary(&target);
    let project_root = normalize_source_path_for_boundary(&project_root);
    !source_path_is_inside_project(&project_root, &target)
        || target.split('/').any(|segment| segment == "node_modules")
}

fn clean_source_path_for_boundary(path: &Path) -> PathBuf {
    let mut clean = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !clean.pop() {
                    clean.push("..");
                }
            }
            _ => clean.push(component.as_os_str()),
        }
    }
    clean
}

fn normalize_source_path_for_boundary(path: &Path) -> String {
    let normalized = path
        .to_string_lossy()
        .replace('\\', "/")
        .trim_end_matches('/')
        .to_string();
    normalized
        .strip_prefix("//?/")
        .unwrap_or(&normalized)
        .to_string()
}

fn source_path_is_inside_project(project_root: &str, target: &str) -> bool {
    !project_root.is_empty()
        && (target == project_root
            || target
                .strip_prefix(project_root)
                .is_some_and(|rest| rest.starts_with('/')))
}

fn path_alias_target_requires_adapter_boundary(
    project_root: &Path,
    base_url: &Path,
    target: &str,
) -> bool {
    path_alias_target_has_node_modules_boundary(base_url, target)
        || path_alias_target_has_outside_boundary(project_root, base_url, target)
}

fn source_alias_boundary_detail(project_root: &Path, alias: &SourcePathAlias) -> &'static str {
    if source_path_requires_adapter_boundary(project_root, &alias.base_url) {
        return RESOLVER_DETAIL_SOURCE_ALIAS_BASE_URL_BOUNDARY;
    }
    if alias
        .targets
        .iter()
        .any(|target| path_alias_target_has_node_modules_boundary(&alias.base_url, target))
    {
        return RESOLVER_DETAIL_SOURCE_ALIAS_TARGET_NODE_MODULES_BOUNDARY;
    }
    if alias
        .targets
        .iter()
        .any(|target| path_alias_target_has_outside_boundary(project_root, &alias.base_url, target))
    {
        return RESOLVER_DETAIL_SOURCE_ALIAS_TARGET_OUTSIDE_PROJECT_BOUNDARY;
    }
    RESOLVER_DETAIL_SOURCE_ALIAS_BOUNDARY
}

fn path_alias_target_has_outside_boundary(
    project_root: &Path,
    base_url: &Path,
    target: &str,
) -> bool {
    let target = path_alias_target_path(base_url, target);
    let target = normalize_source_path_for_boundary(&target);
    let project_root =
        normalize_source_path_for_boundary(&clean_source_path_for_boundary(project_root));
    !source_path_is_inside_project(&project_root, &target)
}

fn path_alias_target_has_node_modules_boundary(base_url: &Path, target: &str) -> bool {
    normalize_source_path_for_boundary(&path_alias_target_path(base_url, target))
        .split('/')
        .any(|segment| segment == "node_modules")
}

fn path_alias_target_path(base_url: &Path, target: &str) -> PathBuf {
    let target = target.replace('\\', "/").replace('*', "__dx_wildcard__");
    let target_path = Path::new(&target);
    if target_path.is_absolute() {
        clean_source_path_for_boundary(target_path)
    } else {
        clean_source_path_for_boundary(&base_url.join(target_path))
    }
}

fn source_owned_package_import_targets_with_boundary(value: &Value) -> SourcePackageImportTargets {
    let mut targets = SourcePackageImportTargets::default();
    match value {
        Value::String(target) => match source_owned_package_import_target_or_boundary(target) {
            Ok(target) => targets.push(target),
            Err(detail) => targets.mark_boundary(Some(detail)),
        },
        Value::Array(values) => {
            for value in values {
                targets.merge(source_owned_package_import_targets_with_boundary(value));
            }
        }
        Value::Object(conditions) => {
            targets.merge(source_owned_package_import_condition_targets(conditions));
        }
        _ => {
            targets.mark_boundary(Some(RESOLVER_DETAIL_PACKAGE_IMPORT_NO_SOURCE_TARGET));
        }
    }
    targets
}

fn source_owned_package_import_condition_targets(
    conditions: &serde_json::Map<String, Value>,
) -> SourcePackageImportTargets {
    source_runtime_condition_targets(
        conditions,
        source_owned_package_import_targets_with_boundary,
    )
}

fn source_owned_package_export_targets_with_boundary(value: &Value) -> SourcePackageImportTargets {
    let mut targets = SourcePackageImportTargets::default();
    match value {
        Value::String(target) => match source_owned_package_export_target_or_boundary(target) {
            Ok(target) => targets.push(target),
            Err(detail) => targets.mark_boundary(Some(detail)),
        },
        Value::Array(values) => {
            for value in values {
                targets.merge(source_owned_package_export_targets_with_boundary(value));
            }
        }
        Value::Object(conditions) => {
            targets.merge(source_owned_package_export_condition_targets(conditions));
        }
        _ => {
            targets.mark_boundary(Some(RESOLVER_DETAIL_PACKAGE_EXPORT_NO_SOURCE_TARGET));
        }
    }
    targets
}

fn source_owned_package_export_condition_targets(
    conditions: &serde_json::Map<String, Value>,
) -> SourcePackageImportTargets {
    source_runtime_condition_targets(
        conditions,
        source_owned_package_export_targets_with_boundary,
    )
}

impl SourcePackageImportTargets {
    fn push(&mut self, target: String) {
        if !self.targets.iter().any(|existing| existing == &target) {
            self.targets.push(target);
        }
    }

    fn merge(&mut self, targets: SourcePackageImportTargets) {
        self.adapter_boundary |= targets.adapter_boundary;
        self.adapter_boundary_detail = preferred_package_export_boundary_detail(
            self.adapter_boundary_detail,
            targets.adapter_boundary_detail,
        );
        for target in targets.targets {
            self.push(target);
        }
    }

    fn mark_boundary(&mut self, detail: Option<&'static str>) {
        self.adapter_boundary = true;
        self.adapter_boundary_detail =
            preferred_package_export_boundary_detail(self.adapter_boundary_detail, detail);
    }
}

fn source_runtime_package_import_condition(condition: &str) -> bool {
    !matches!(condition, "types" | "node" | "node-addons" | "require")
}

fn source_runtime_condition_targets(
    conditions: &serde_json::Map<String, Value>,
    value_targets: fn(&Value) -> SourcePackageImportTargets,
) -> SourcePackageImportTargets {
    let mut boundary_fallback = SourcePackageImportTargets::default();
    for value in source_runtime_condition_values(conditions) {
        let targets = value_targets(value);
        if !targets.targets.is_empty() {
            return targets;
        }
        boundary_fallback.merge(targets);
    }
    boundary_fallback
}

fn source_owned_package_import_target_or_boundary(target: &str) -> Result<String, &'static str> {
    let target = target.replace('\\', "/");
    if !target.starts_with("./") {
        return Err(RESOLVER_DETAIL_PACKAGE_IMPORT_NO_SOURCE_TARGET);
    }
    let target = target
        .strip_prefix("./")
        .ok_or(RESOLVER_DETAIL_PACKAGE_IMPORT_NO_SOURCE_TARGET)?;
    if target.is_empty() {
        return Err(RESOLVER_DETAIL_PACKAGE_IMPORT_NO_SOURCE_TARGET);
    }

    let target = normalize_source_path_for_boundary(&clean_package_import_target_path(target));
    if target.is_empty() {
        return Err(RESOLVER_DETAIL_PACKAGE_IMPORT_NO_SOURCE_TARGET);
    }
    if target == ".." || target.starts_with("../") {
        return Err(RESOLVER_DETAIL_PACKAGE_IMPORT_TARGET_OUTSIDE_PACKAGE_BOUNDARY);
    }
    if target.split('/').any(|segment| segment == "node_modules") {
        return Err(RESOLVER_DETAIL_PACKAGE_IMPORT_TARGET_NODE_MODULES_BOUNDARY);
    }

    Ok(target)
}

fn clean_package_import_target_path(target: &str) -> PathBuf {
    clean_source_path_for_boundary(Path::new(target))
}

fn source_owned_package_export_target_or_boundary(target: &str) -> Result<String, &'static str> {
    let target = target.replace('\\', "/");
    if !target.starts_with("./") {
        return Err(RESOLVER_DETAIL_PACKAGE_EXPORT_NO_SOURCE_TARGET);
    }
    let target = target
        .strip_prefix("./")
        .ok_or(RESOLVER_DETAIL_PACKAGE_EXPORT_NO_SOURCE_TARGET)?;
    if target.is_empty() {
        return Err(RESOLVER_DETAIL_PACKAGE_EXPORT_NO_SOURCE_TARGET);
    }

    let target = normalize_source_path_for_boundary(&clean_package_export_target_path(target));
    if target.is_empty() {
        return Err(RESOLVER_DETAIL_PACKAGE_EXPORT_NO_SOURCE_TARGET);
    }
    if target == ".." || target.starts_with("../") {
        return Err(RESOLVER_DETAIL_PACKAGE_EXPORT_TARGET_OUTSIDE_PACKAGE_BOUNDARY);
    }
    if target.split('/').any(|segment| segment == "node_modules") {
        return Err(RESOLVER_DETAIL_PACKAGE_EXPORT_TARGET_NODE_MODULES_BOUNDARY);
    }

    Ok(target)
}

fn clean_package_export_target_path(target: &str) -> PathBuf {
    clean_source_path_for_boundary(Path::new(target))
}

fn source_runtime_condition_values(conditions: &serde_json::Map<String, Value>) -> Vec<&Value> {
    let mut values = Vec::new();
    for condition in [
        "dx",
        "source",
        "browser",
        "development",
        "import",
        "default",
    ] {
        if let Some(value) = conditions.get(condition) {
            values.push(value);
        }
    }

    let mut custom_conditions = conditions
        .keys()
        .filter(|condition| {
            !matches!(
                condition.as_str(),
                "dx" | "source" | "browser" | "development" | "import" | "default"
            ) && source_runtime_package_import_condition(condition)
        })
        .collect::<Vec<_>>();
    custom_conditions.sort();
    for condition in custom_conditions {
        if let Some(value) = conditions.get(condition) {
            values.push(value);
        }
    }

    values
}

fn preferred_package_export_boundary_detail(
    current: Option<&'static str>,
    next: Option<&'static str>,
) -> Option<&'static str> {
    match (current, next) {
        (None, detail) | (detail, None) => detail,
        (Some(current), Some(next))
            if package_export_boundary_detail_rank(next)
                > package_export_boundary_detail_rank(current) =>
        {
            Some(next)
        }
        (Some(current), Some(_)) => Some(current),
    }
}

fn package_export_boundary_detail_rank(detail: &str) -> u8 {
    match detail {
        RESOLVER_DETAIL_PACKAGE_EXPORT_TARGET_NODE_MODULES_BOUNDARY
        | RESOLVER_DETAIL_PACKAGE_IMPORT_TARGET_NODE_MODULES_BOUNDARY => 3,
        RESOLVER_DETAIL_PACKAGE_EXPORT_TARGET_OUTSIDE_PACKAGE_BOUNDARY
        | RESOLVER_DETAIL_PACKAGE_IMPORT_TARGET_OUTSIDE_PACKAGE_BOUNDARY => 2,
        RESOLVER_DETAIL_PACKAGE_EXPORT_NO_SOURCE_TARGET
        | RESOLVER_DETAIL_PACKAGE_IMPORT_NO_SOURCE_TARGET => 1,
        _ => 0,
    }
}

fn read_json_config(path: &Path) -> DxResult<Option<(String, Value)>> {
    if !path.is_file() {
        return Ok(None);
    }
    let source = std::fs::read_to_string(path).map_err(|error| DxError::IoError {
        path: Some(path.to_path_buf()),
        message: error.to_string(),
    })?;
    let json_source = jsonc_to_json(&source);
    let config = serde_json::from_str(&json_source).map_err(|error| DxError::ConfigParseError {
        file: Some(path.to_path_buf()),
        message: format!("{}: {}", path.display(), error),
        src: Some(source.clone()),
        span: None,
    })?;
    Ok(Some((source, config)))
}

fn jsonc_to_json(source: &str) -> String {
    strip_jsonc_trailing_commas(&strip_jsonc_comments(source))
}

fn strip_jsonc_comments(source: &str) -> String {
    let mut output = String::with_capacity(source.len());
    let mut chars = source.char_indices().peekable();
    let mut in_string = false;
    let mut escaped = false;

    while let Some((_, character)) = chars.next() {
        if in_string {
            output.push(character);
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
            continue;
        }

        if character == '"' {
            in_string = true;
            output.push(character);
            continue;
        }

        if character == '/' {
            match chars.peek().map(|(_, next)| *next) {
                Some('/') => {
                    chars.next();
                    for (_, comment_character) in chars.by_ref() {
                        if comment_character == '\n' {
                            output.push('\n');
                            break;
                        }
                    }
                }
                Some('*') => {
                    chars.next();
                    while let Some((_, comment_character)) = chars.next() {
                        if comment_character == '\n' {
                            output.push('\n');
                        } else if comment_character == '*'
                            && chars.peek().map(|(_, next)| *next) == Some('/')
                        {
                            chars.next();
                            break;
                        }
                    }
                }
                _ => output.push(character),
            }
            continue;
        }

        output.push(character);
    }

    output
}

fn strip_jsonc_trailing_commas(source: &str) -> String {
    let mut output = String::with_capacity(source.len());
    let mut chars = source.char_indices().peekable();
    let mut in_string = false;
    let mut escaped = false;

    while let Some((_, character)) = chars.next() {
        if in_string {
            output.push(character);
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
            continue;
        }

        if character == '"' {
            in_string = true;
            output.push(character);
            continue;
        }

        if character == ',' {
            let mut lookahead = chars.clone();
            let next = lookahead
                .find(|(_, next)| !next.is_whitespace())
                .map(|(_, next)| next);
            if matches!(next, Some('}' | ']')) {
                continue;
            }
        }

        output.push(character);
    }

    output
}

fn read_json_config_chain(
    project_root: &Path,
    config_path: &Path,
    path_resolver_source: &'static str,
    base_url_resolver_source: &'static str,
    stack: &mut Vec<PathBuf>,
) -> DxResult<Vec<LoadedSourceConfig>> {
    if stack.len() >= MAX_CONFIG_EXTENDS_DEPTH {
        let source = std::fs::read_to_string(config_path).unwrap_or_default();
        return Err(config_parse_error(
            config_path,
            &source,
            "config extends chain is too deep",
        ));
    }

    let canonical = config_path
        .canonicalize()
        .map_err(|error| DxError::IoError {
            path: Some(config_path.to_path_buf()),
            message: error.to_string(),
        })?;
    if stack.iter().any(|path| path == &canonical) {
        let source = std::fs::read_to_string(config_path).unwrap_or_default();
        return Err(config_parse_error(
            config_path,
            &source,
            "config extends cycle detected",
        ));
    }

    let (source, config) = read_json_config(config_path)?.expect("config exists");
    stack.push(canonical);

    let mut configs = Vec::new();
    match config.get("extends") {
        Some(Value::String(extended)) => {
            append_source_owned_extended_config(
                &mut configs,
                project_root,
                config_path,
                extended,
                path_resolver_source,
                base_url_resolver_source,
                stack,
            )?;
        }
        Some(Value::Array(extended_configs)) => {
            for extended in extended_configs {
                let Some(extended) = extended.as_str() else {
                    stack.pop();
                    return Err(config_parse_error(
                        config_path,
                        &source,
                        "extends entries must be strings",
                    ));
                };
                append_source_owned_extended_config(
                    &mut configs,
                    project_root,
                    config_path,
                    extended,
                    path_resolver_source,
                    base_url_resolver_source,
                    stack,
                )?;
            }
        }
        Some(_) => {
            stack.pop();
            return Err(config_parse_error(
                config_path,
                &source,
                "extends must be a string or array of strings",
            ));
        }
        None => {}
    }

    stack.pop();
    configs.push(LoadedSourceConfig {
        path: config_path.to_path_buf(),
        source,
        config,
        path_resolver_source,
        base_url_resolver_source,
    });
    Ok(configs)
}

fn append_source_owned_extended_config(
    configs: &mut Vec<LoadedSourceConfig>,
    project_root: &Path,
    config_path: &Path,
    extended: &str,
    path_resolver_source: &'static str,
    base_url_resolver_source: &'static str,
    stack: &mut Vec<PathBuf>,
) -> DxResult<()> {
    if let Some(extended_path) =
        source_owned_extended_config_path(project_root, config_path, extended)?
    {
        configs.extend(read_json_config_chain(
            project_root,
            &extended_path,
            path_resolver_source,
            base_url_resolver_source,
            stack,
        )?);
    }
    Ok(())
}

fn source_owned_extended_config_path(
    project_root: &Path,
    config_path: &Path,
    extended: &str,
) -> DxResult<Option<PathBuf>> {
    let extended = extended.replace('\\', "/");
    if extended.is_empty() || extended.split('/').any(|segment| segment == "node_modules") {
        return Ok(None);
    }

    let extended_path = Path::new(&extended);
    if !extended_path.is_absolute() && !extended.starts_with("./") && !extended.starts_with("../") {
        return Ok(None);
    }

    let mut candidate = resolve_config_relative_path(config_path, extended_path);
    if candidate.extension().is_none() {
        candidate.set_extension("json");
    }

    if source_path_requires_adapter_boundary(project_root, &candidate) {
        return Ok(None);
    }

    let project_root = project_root
        .canonicalize()
        .map_err(|error| DxError::IoError {
            path: Some(project_root.to_path_buf()),
            message: error.to_string(),
        })?;
    let canonical = candidate.canonicalize().map_err(|error| DxError::IoError {
        path: Some(candidate.clone()),
        message: error.to_string(),
    })?;
    if source_path_requires_adapter_boundary(&project_root, &canonical) {
        return Ok(None);
    }
    Ok(Some(clean_source_path_for_boundary(&candidate)))
}

fn resolve_config_relative_path(config_path: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        config_path
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .join(path)
    }
}

fn normalize_config_base_path(path: PathBuf) -> PathBuf {
    clean_source_path_for_boundary(&path)
}

fn sort_aliases(aliases: &mut [SourcePathAlias]) {
    aliases.sort_by(|left, right| {
        right
            .pattern
            .len()
            .cmp(&left.pattern.len())
            .then_with(|| left.pattern.cmp(&right.pattern))
    });
}

#[cfg(test)]
mod tests {
    use std::fs;

    use serde_json::json;

    use super::*;

    #[test]
    fn package_self_reference_aliases_respect_exports_without_generic_fallback() {
        let package_json = json!({
            "name": "dx-source-resolver-fixture",
            "exports": {
                ".": "./src/public.ts",
                "./feature/*": "./src/features/*.tsx",
                "./blocked/*": "./node_modules/not-source/*"
            }
        });

        let aliases = package_self_reference_aliases(&package_json);

        assert_eq!(aliases.len(), 3);
        assert!(aliases.iter().any(|alias| {
            alias.pattern == "dx-source-resolver-fixture"
                && alias.targets.len() == 1
                && alias.targets[0] == "src/public.ts"
                && alias.resolver_source == RESOLVER_SOURCE_PACKAGE_SELF_REFERENCE
        }));
        assert!(aliases.iter().any(|alias| {
            alias.pattern == "dx-source-resolver-fixture/feature/*"
                && alias.targets.len() == 1
                && alias.targets[0] == "src/features/*.tsx"
                && alias.resolver_source == RESOLVER_SOURCE_PACKAGE_SELF_REFERENCE
        }));
        assert!(aliases.iter().any(|alias| {
            alias.pattern == "dx-source-resolver-fixture/blocked/*"
                && alias.targets.is_empty()
                && alias.adapter_boundary
                && alias.adapter_boundary_detail
                    == Some(RESOLVER_DETAIL_PACKAGE_EXPORT_TARGET_NODE_MODULES_BOUNDARY)
                && alias.resolver_source == RESOLVER_SOURCE_PACKAGE_SELF_REFERENCE
        }));
        assert!(
            aliases
                .iter()
                .all(|alias| alias.pattern != "dx-source-resolver-fixture/*")
        );
    }

    #[test]
    fn package_self_reference_aliases_include_root_and_subpath_without_exports() {
        let package_json = json!({
            "name": "dx-source-resolver-fixture"
        });

        let aliases = package_self_reference_aliases(&package_json);

        assert_eq!(aliases.len(), 2);
        assert!(aliases.iter().any(|alias| {
            alias.pattern == "dx-source-resolver-fixture"
                && alias.targets.len() == 1
                && alias.targets[0] == "."
                && alias.resolver_source == RESOLVER_SOURCE_PACKAGE_SELF_REFERENCE
        }));
        assert!(aliases.iter().any(|alias| {
            alias.pattern == "dx-source-resolver-fixture/*"
                && alias.targets.len() == 1
                && alias.targets[0] == "*"
                && alias.resolver_source == RESOLVER_SOURCE_PACKAGE_SELF_REFERENCE
        }));
    }

    #[test]
    fn package_self_reference_aliases_skip_compiler_reserved_names() {
        for name in ["dx-www", "next", "react", "node:fs"] {
            let package_json = json!({ "name": name });

            assert!(
                package_self_reference_aliases(&package_json).is_empty(),
                "{name} must stay a compiler intrinsic, not a package self-reference"
            );
        }
    }

    #[test]
    fn package_self_reference_aliases_skip_invalid_package_names() {
        for name in [
            "@scope",
            "@@scope/pkg",
            "@scope/@pkg",
            "@scope/pkg/extra",
            "plain/pkg",
            "node_modules/pkg",
        ] {
            let package_json = json!({
                "name": name,
                "exports": {
                    ".": "./src/public.ts"
                }
            });

            assert!(
                package_self_reference_aliases(&package_json).is_empty(),
                "{name} must stay outside the package self-reference namespace"
            );
        }
    }

    #[test]
    fn jsonc_to_json_strips_comments_and_trailing_commas_without_touching_strings() {
        let source = r#"{
  // line comment
  "url": "https://example.test/path//not-comment",
  "glob": "shared/*",
  "paths": {
    "@ui/*": ["components/*",],
  },
  /* block
     comment */
}
"#;

        let config: Value = serde_json::from_str(&jsonc_to_json(source)).expect("valid json");

        assert_eq!(config["url"], "https://example.test/path//not-comment");
        assert_eq!(config["glob"], "shared/*");
        assert_eq!(config["paths"]["@ui/*"][0], "components/*");
    }

    #[test]
    fn source_path_boundary_rejects_project_prefix_siblings() {
        let project_root = Path::new("G:/Dx/www");

        assert!(!source_path_requires_adapter_boundary(
            project_root,
            Path::new("G:/Dx/www")
        ));
        assert!(!source_path_requires_adapter_boundary(
            project_root,
            Path::new("G:/Dx/www/src")
        ));
        assert!(!source_path_requires_adapter_boundary(
            project_root,
            Path::new("G:/Dx/www/config/..")
        ));
        assert!(source_path_requires_adapter_boundary(
            project_root,
            Path::new("G:/Dx/www-other/src")
        ));
        assert!(source_path_requires_adapter_boundary(
            project_root,
            Path::new("G:/Dx/www/node_modules/trap")
        ));
        assert!(source_path_requires_adapter_boundary(
            project_root,
            Path::new("G:/Dx/www/../outside")
        ));
    }

    #[test]
    fn source_owned_extended_config_path_keeps_local_config_before_canonical_boundary() {
        let project = tempfile::tempdir().expect("temp project");
        let root = project.path();
        let config_dir = root.join("config");
        fs::create_dir_all(&config_dir).expect("config dir");
        let local_config = config_dir.join("base.json");
        fs::write(&local_config, "{}\n").expect("local config");

        let resolved =
            source_owned_extended_config_path(root, &root.join("jsconfig.json"), "./config/base")
                .expect("extended config path")
                .expect("local config is source-owned");

        assert_eq!(resolved, clean_source_path_for_boundary(&local_config));
    }

    #[test]
    fn source_owned_extended_config_path_ignores_missing_external_config_before_canonicalize() {
        let project = tempfile::tempdir().expect("temp project");
        let root = project.path().join("www");
        fs::create_dir_all(&root).expect("project root");

        let resolved = source_owned_extended_config_path(
            &root,
            &root.join("jsconfig.json"),
            "../shared-config/missing",
        )
        .expect("external path should be ignored, not canonicalized");

        assert!(resolved.is_none());
    }
}

fn config_parse_error(config_path: &Path, source: &str, message: &str) -> DxError {
    DxError::ConfigParseError {
        file: Some(config_path.to_path_buf()),
        message: format!("{}: {}", config_path.display(), message),
        src: Some(source.to_string()),
        span: None,
    }
}
