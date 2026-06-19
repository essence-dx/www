use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use chrono::Utc;
use serde_json::{Value, json};
use serializer::llm::{
    DxDocument, DxLlmValue, MachineFormat, llm_to_document, machine_to_document,
};

use crate::config::DxConfig;

const DEV_EXTENSION_TOOLCHAIN_JSON_RECEIPT: &str = ".dx/receipts/run/dev-extension-toolchain.json";
const DEV_EXTENSION_TOOLCHAIN_SR_RECEIPT: &str = ".dx/run/dev-extension-toolchain.sr";

const SERIALIZER_COMMAND: DevExtensionToolchainCommand = DevExtensionToolchainCommand {
    name: "serializer",
    args: &["serializer"],
};
const STYLE_COMMAND: DevExtensionToolchainCommand = DevExtensionToolchainCommand {
    name: "style",
    args: &["style", "build", "--json"],
};
const ICONS_COMMAND: DevExtensionToolchainCommand = DevExtensionToolchainCommand {
    name: "icons",
    args: &["icons", "sync", "--json"],
};
const IMPORTS_COMMAND: DevExtensionToolchainCommand = DevExtensionToolchainCommand {
    name: "imports",
    args: &["imports", "sync", "--json"],
};

pub(super) fn run_dx_extension_toolchain_for_changed_paths(
    project_root: &Path,
    paths: &[PathBuf],
) -> bool {
    let config = DevExtensionToolchainConfig::load(project_root);
    let plan = DevExtensionToolchainPlan::from_changed_paths(project_root, paths, &config);
    if plan.is_empty() {
        return false;
    }

    let mut results = Vec::new();
    for source in &plan.serializer_sources {
        results.push(generate_serializer_machine(project_root, source));
    }
    for command in &plan.commands {
        results.push(run_dev_toolchain_command(project_root, command));
    }

    write_dev_extension_toolchain_receipt(project_root, &plan, &results);
    results
        .iter()
        .all(|result| result.get("passed").and_then(Value::as_bool) == Some(true))
}

#[derive(Debug)]
struct DevExtensionToolchainConfig {
    enabled_tools: BTreeSet<String>,
    watch_extensions: BTreeMap<String, BTreeSet<String>>,
    generated_icon_dir: String,
    generated_style_file: String,
    classname_markers: Vec<String>,
    icon_markers: Vec<String>,
}

impl DevExtensionToolchainConfig {
    fn load(project_root: &Path) -> Self {
        let dx_config = DxConfig::load_project(project_root).unwrap_or_default();
        let document = load_dx_document(project_root);

        let enabled_tools = document
            .as_ref()
            .map(enabled_tools_from_document)
            .filter(|tools| !tools.is_empty())
            .unwrap_or_else(default_enabled_tools);
        let watch_extensions = document
            .as_ref()
            .map(watch_extensions_from_document)
            .filter(|watch| !watch.is_empty())
            .unwrap_or_else(default_watch_extensions);
        let classname_markers = configured_classname_markers(&dx_config);
        let icon_markers = configured_icon_markers(&dx_config);

        Self {
            enabled_tools,
            watch_extensions,
            generated_icon_dir: dx_config.tooling.icons.generated_dir,
            generated_style_file: dx_config.tooling.dx_style.generated_css,
            classname_markers,
            icon_markers,
        }
    }

    fn tool_enabled(&self, tool: &str) -> bool {
        self.enabled_tools.contains(tool)
    }

    fn watches_extension(&self, tool: &str, extension: &str) -> bool {
        self.watch_extensions
            .get(tool)
            .is_some_and(|extensions| extensions.contains(extension))
    }

    fn is_generated_output(&self, project_root: &Path, path: &Path) -> bool {
        let normalized = normalize_dev_path(project_root, path);
        if normalized.starts_with(".dx/serializer/")
            || normalized.starts_with(".dx/receipts/")
            || normalized.starts_with(".dx/run/")
            || normalized.starts_with(".dx/www/")
            || normalized.starts_with(".dx/build/")
        {
            return true;
        }

        let generated_icon_dir = self.generated_icon_dir.trim_matches('/').replace('\\', "/");
        if !generated_icon_dir.is_empty()
            && (normalized == generated_icon_dir
                || normalized.starts_with(&format!("{generated_icon_dir}/")))
        {
            return true;
        }

        let generated_style_file = self.generated_style_file.replace('\\', "/");
        !generated_style_file.is_empty() && normalized == generated_style_file
    }
}

#[derive(Debug, Default)]
struct DevExtensionToolchainPlan {
    triggered_paths: Vec<String>,
    serializer_sources: Vec<PathBuf>,
    commands: Vec<DevExtensionToolchainCommand>,
}

impl DevExtensionToolchainPlan {
    fn from_changed_paths(
        project_root: &Path,
        paths: &[PathBuf],
        config: &DevExtensionToolchainConfig,
    ) -> Self {
        let mut plan = Self::default();
        let mut command_names = BTreeSet::new();
        let mut serializer_sources = BTreeSet::new();

        for path in paths {
            let normalized = normalize_dev_path(project_root, path);
            let mut path_triggered = false;

            if is_serializer_source_path(project_root, path) && config.tool_enabled("serializer") {
                serializer_sources.insert(resolve_dev_path(project_root, path));
                path_triggered = true;
            }

            if !config.is_generated_output(project_root, path) {
                let extension = normalized_extension(path);
                if let Some(extension) = extension.as_deref() {
                    if config.tool_enabled("style")
                        && config.watches_extension("style", extension)
                        && file_needs_style(project_root, path, config)
                    {
                        command_names.insert(STYLE_COMMAND.name);
                        path_triggered = true;
                    }

                    if config.tool_enabled("icons")
                        && config.watches_extension("icons", extension)
                        && file_needs_icons(project_root, path, config)
                    {
                        command_names.insert(ICONS_COMMAND.name);
                        path_triggered = true;
                    }

                    if config.tool_enabled("imports")
                        && config.watches_extension("imports", extension)
                        && file_needs_imports(path)
                    {
                        command_names.insert(IMPORTS_COMMAND.name);
                        path_triggered = true;
                    }
                }
            }

            if path_triggered && !plan.triggered_paths.contains(&normalized) {
                plan.triggered_paths.push(normalized);
            }
        }

        plan.serializer_sources = serializer_sources.into_iter().collect();
        for command in [IMPORTS_COMMAND, STYLE_COMMAND, ICONS_COMMAND] {
            if command_names.contains(command.name) {
                plan.commands.push(command);
            }
        }

        plan
    }

    fn is_empty(&self) -> bool {
        self.triggered_paths.is_empty()
            && self.serializer_sources.is_empty()
            && self.commands.is_empty()
    }

    fn command_names(&self) -> Vec<&'static str> {
        let mut names = Vec::new();
        if !self.serializer_sources.is_empty() {
            names.push(SERIALIZER_COMMAND.name);
        }
        names.extend(self.commands.iter().map(|command| command.name));
        names
    }
}

#[derive(Debug, Clone, Copy)]
struct DevExtensionToolchainCommand {
    name: &'static str,
    args: &'static [&'static str],
}

fn load_dx_document(project_root: &Path) -> Option<DxDocument> {
    let dx_path = project_root.join("dx");
    if !dx_path.is_file() {
        return None;
    }

    if machine_cache_is_fresh(&dx_path, &project_root.join(".dx/serializer/dx.machine")) {
        if let Ok(bytes) = std::fs::read(project_root.join(".dx/serializer/dx.machine")) {
            let machine = MachineFormat::new(bytes);
            if let Ok(document) = machine_to_document(&machine) {
                return Some(document);
            }
        }
    }

    std::fs::read_to_string(dx_path)
        .ok()
        .and_then(|source| llm_to_document(&source).ok())
}

fn machine_cache_is_fresh(source: &Path, machine: &Path) -> bool {
    let Ok(source_modified) = std::fs::metadata(source).and_then(|metadata| metadata.modified())
    else {
        return false;
    };
    let Ok(machine_modified) = std::fs::metadata(machine).and_then(|metadata| metadata.modified())
    else {
        return false;
    };

    machine_modified >= source_modified
}

fn enabled_tools_from_document(document: &DxDocument) -> BTreeSet<String> {
    let Some(tools) = document.section_by_name("tools") else {
        return default_enabled_tools();
    };
    let Some(name_index) = tools.column_index("name") else {
        return default_enabled_tools();
    };
    let enabled_index = tools.column_index("enabled");
    let mut enabled_tools = BTreeSet::new();

    for row in &tools.rows {
        let Some(tool) = row.get(name_index).map(value_to_string) else {
            continue;
        };
        let enabled = enabled_index
            .and_then(|index| row.get(index))
            .and_then(value_to_bool)
            .unwrap_or(true);
        if enabled {
            enabled_tools.insert(normalize_tool_name(&tool));
        }
    }

    enabled_tools
}

fn watch_extensions_from_document(document: &DxDocument) -> BTreeMap<String, BTreeSet<String>> {
    let Some(watch) = document.section_by_name("watch") else {
        return default_watch_extensions();
    };
    let Some(tool_index) = watch.column_index("tool") else {
        return default_watch_extensions();
    };
    let Some(extensions_index) = watch.column_index("extensions") else {
        return default_watch_extensions();
    };
    let mut by_tool = BTreeMap::new();

    for row in &watch.rows {
        let Some(tool) = row.get(tool_index).map(value_to_string) else {
            continue;
        };
        let Some(extensions) = row.get(extensions_index).map(value_to_string) else {
            continue;
        };
        let extension_set = split_words(&extensions)
            .into_iter()
            .map(|extension| extension.trim_start_matches('.').to_ascii_lowercase())
            .filter(|extension| !extension.is_empty())
            .collect::<BTreeSet<_>>();
        if !extension_set.is_empty() {
            by_tool.insert(normalize_tool_name(&tool), extension_set);
        }
    }

    by_tool
}

fn default_enabled_tools() -> BTreeSet<String> {
    ["serializer", "style", "icons", "imports"]
        .into_iter()
        .map(ToOwned::to_owned)
        .collect()
}

fn default_watch_extensions() -> BTreeMap<String, BTreeSet<String>> {
    [
        ("style", ["tsx", "jsx", "mdx", "html", "css"].as_slice()),
        ("icons", ["tsx"].as_slice()),
        ("imports", ["tsx"].as_slice()),
    ]
    .into_iter()
    .map(|(tool, extensions)| {
        (
            tool.to_string(),
            extensions
                .iter()
                .map(|extension| (*extension).to_string())
                .collect(),
        )
    })
    .collect()
}

fn normalize_tool_name(tool: &str) -> String {
    match tool {
        "lighthouse" => "web_perf".to_string(),
        value => value.to_string(),
    }
}

fn configured_classname_markers(config: &DxConfig) -> Vec<String> {
    let mut markers = vec![
        "className=".to_string(),
        "className:".to_string(),
        "class=".to_string(),
        "class:".to_string(),
        " class ".to_string(),
    ];
    for helper in std::iter::once(&config.tooling.classnames.helper)
        .chain(config.tooling.classnames.compat_helpers.iter())
    {
        if !helper.trim().is_empty() {
            markers.push(format!("{}(", helper.trim()));
        }
    }
    markers.sort();
    markers.dedup();
    markers
}

fn configured_icon_markers(config: &DxConfig) -> Vec<String> {
    let mut markers = [
        config.tooling.icons.source_tag.as_str(),
        config.tooling.icons.runtime_tag.as_str(),
        config.tooling.icons.component.as_str(),
    ]
    .into_iter()
    .filter(|tag| !tag.trim().is_empty())
    .map(|tag| format!("<{}", tag.trim()))
    .collect::<Vec<_>>();
    markers.sort();
    markers.dedup();
    markers
}

fn is_serializer_source_path(project_root: &Path, path: &Path) -> bool {
    let relative = path.strip_prefix(project_root).unwrap_or(path);
    let parts = relative
        .components()
        .filter_map(|component| component.as_os_str().to_str())
        .map(|part| part.replace('\\', "/"))
        .collect::<Vec<_>>();

    if parts.len() >= 2
        && parts[0] == ".dx"
        && matches!(
            parts[1].as_str(),
            "serializer" | "run" | "receipts" | "www" | "build"
        )
    {
        return false;
    }

    if path.file_name().and_then(|name| name.to_str()) == Some("dx") {
        return true;
    }

    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("sr"))
}

fn file_needs_style(
    project_root: &Path,
    path: &Path,
    config: &DevExtensionToolchainConfig,
) -> bool {
    if !path.exists() {
        return true;
    }
    let extension = normalized_extension(path);
    if extension.as_deref().is_some_and(|extension| {
        matches!(extension, "css" | "html" | "mdx")
            && !config.is_generated_output(project_root, path)
    }) {
        return true;
    }

    std::fs::read_to_string(path)
        .ok()
        .is_some_and(|source| contains_classname_marker(&source, config))
}

fn file_needs_icons(
    _project_root: &Path,
    path: &Path,
    config: &DevExtensionToolchainConfig,
) -> bool {
    if !path.exists() {
        return true;
    }

    std::fs::read_to_string(path)
        .ok()
        .is_some_and(|source| contains_icon_marker(&source, &config.icon_markers))
}

fn file_needs_imports(path: &Path) -> bool {
    !path.exists()
        || std::fs::read_to_string(path)
            .ok()
            .is_some_and(|source| source.contains("import ") || source.contains("from "))
}

fn contains_classname_marker(source: &str, config: &DevExtensionToolchainConfig) -> bool {
    config
        .classname_markers
        .iter()
        .any(|marker| source.contains(marker))
}

fn contains_icon_marker(source: &str, markers: &[String]) -> bool {
    markers.iter().any(|marker| source.contains(marker))
}

fn generate_serializer_machine(project_root: &Path, source: &Path) -> Value {
    let serializer_config = serializer::SerializerOutputConfig::new()
        .with_output_dir(project_root.join(".dx/serializer"))
        .with_llm(false)
        .with_machine(true);
    let serializer = serializer::SerializerOutput::with_config(serializer_config);
    match serializer.process_file(source) {
        Ok(result) => json!({
            "name": SERIALIZER_COMMAND.name,
            "command": format!("dx serializer {}", normalize_dev_path(project_root, source)),
            "passed": result.machine_generated,
            "source": normalize_dev_path(project_root, &result.paths.source),
            "machine": normalize_dev_path(project_root, &result.paths.machine),
            "machine_size": result.machine_size
        }),
        Err(error) => json!({
            "name": SERIALIZER_COMMAND.name,
            "command": format!("dx serializer {}", normalize_dev_path(project_root, source)),
            "passed": false,
            "error": error.to_string()
        }),
    }
}

fn run_dev_toolchain_command(project_root: &Path, command: &DevExtensionToolchainCommand) -> Value {
    let executable = match std::env::current_exe() {
        Ok(executable) => executable,
        Err(error) => {
            return json!({
                "name": command.name,
                "passed": false,
                "error": format!("resolve current dx executable: {error}")
            });
        }
    };
    let output = Command::new(&executable)
        .args(command.args)
        .current_dir(project_root)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output();
    match output {
        Ok(output) => {
            let passed = output.status.success();
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            json!({
                "name": command.name,
                "command": format!("dx {}", command.args.join(" ")),
                "passed": passed,
                "status": output.status.code(),
                "stderr": stderr
            })
        }
        Err(error) => json!({
            "name": command.name,
            "command": format!("dx {}", command.args.join(" ")),
            "passed": false,
            "error": error.to_string()
        }),
    }
}

fn write_dev_extension_toolchain_receipt(
    project_root: &Path,
    plan: &DevExtensionToolchainPlan,
    results: &[Value],
) {
    let passed = results
        .iter()
        .all(|result| result.get("passed").and_then(Value::as_bool) == Some(true));
    let receipt = json!({
        "schema": "dx.dev.extension_toolchain",
        "version": 2,
        "generated_at": Utc::now().to_rfc3339(),
        "config_source": "dx",
        "serializer_cache": ".dx/serializer",
        "triggered_paths": plan.triggered_paths,
        "commands": results,
        "planned_commands": plan.command_names(),
        "passed": passed,
        "policy": "dx config driven TSX/.sr/dx changes run the minimum serializer, imports, style, and icon tools before the hot reload event is published"
    });

    let json_receipt_path = project_root.join(DEV_EXTENSION_TOOLCHAIN_JSON_RECEIPT);
    if let Some(parent) = json_receipt_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(bytes) = serde_json::to_vec_pretty(&receipt) {
        let _ = std::fs::write(json_receipt_path, bytes);
    }

    write_dev_extension_toolchain_sr_receipt(project_root, plan, passed);
}

fn write_dev_extension_toolchain_sr_receipt(
    project_root: &Path,
    plan: &DevExtensionToolchainPlan,
    passed: bool,
) {
    let source = project_root.join(DEV_EXTENSION_TOOLCHAIN_SR_RECEIPT);
    if let Some(parent) = source.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let fields = [
        ("schema", sr_string("dx.dev.extension_toolchain")),
        ("version", sr_number(2)),
        ("generated_at", sr_string(Utc::now().to_rfc3339())),
        ("config_source", sr_string("dx")),
        ("passed", sr_bool(passed)),
        ("triggered_paths", sr_string_array(&plan.triggered_paths)),
        ("planned_commands", sr_string_array(&plan.command_names())),
        ("serializer_cache", sr_string(".dx/serializer")),
        (
            "policy",
            sr_string("dx config driven minimal toolchain before hot reload publish"),
        ),
        (
            "legacy_json",
            sr_string(".dx/receipts/run/dev-extension-toolchain.json"),
        ),
    ];
    let mut content = String::new();
    for (key, value) in fields {
        content.push_str(key);
        content.push('=');
        content.push_str(&value);
        content.push('\n');
    }
    if std::fs::write(&source, content).is_ok() {
        let serializer_config = serializer::SerializerOutputConfig::new()
            .with_output_dir(project_root.join(".dx/serializer"))
            .with_llm(false)
            .with_machine(true);
        let _ = serializer::SerializerOutput::with_config(serializer_config).process_file(&source);
    }
}

fn resolve_dev_path(project_root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        project_root.join(path)
    }
}

fn normalize_dev_path(project_root: &Path, path: &Path) -> String {
    path.strip_prefix(project_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn normalized_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_ascii_lowercase())
}

fn value_to_string(value: &DxLlmValue) -> String {
    match value {
        DxLlmValue::Str(value) | DxLlmValue::Ref(value) => value.clone(),
        DxLlmValue::Num(value) if value.fract() == 0.0 => (*value as i64).to_string(),
        DxLlmValue::Num(value) => value.to_string(),
        DxLlmValue::Bool(value) => value.to_string(),
        DxLlmValue::Null => "null".to_string(),
        DxLlmValue::Arr(_) | DxLlmValue::Obj(_) => value.to_string(),
    }
}

fn value_to_bool(value: &DxLlmValue) -> Option<bool> {
    match value {
        DxLlmValue::Bool(value) => Some(*value),
        DxLlmValue::Str(value) if value.eq_ignore_ascii_case("true") => Some(true),
        DxLlmValue::Str(value) if value.eq_ignore_ascii_case("false") => Some(false),
        _ => None,
    }
}

fn split_words(value: &str) -> Vec<String> {
    value
        .split_whitespace()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn sr_string(value: impl AsRef<str>) -> String {
    let value = value
        .as_ref()
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace(['\r', '\n'], " ");
    format!("\"{value}\"")
}

fn sr_bool(value: bool) -> String {
    value.to_string()
}

fn sr_number(value: impl std::fmt::Display) -> String {
    value.to_string()
}

fn sr_string_array<T: AsRef<str>>(values: &[T]) -> String {
    let values = values.iter().map(sr_string).collect::<Vec<_>>().join(", ");
    format!("[{values}]")
}
