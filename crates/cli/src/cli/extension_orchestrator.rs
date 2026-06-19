use std::path::Path;

use chrono::Utc;
use serde_json::{Value, json};
use serializer::llm::{DxDocument, DxLlmValue, llm_to_document};

use crate::error::{DxError, DxResult};

use super::new_command::default_dx_project_config;
use super::public_framework_tools::{
    PublicToolReport, run_dx_doctor, run_dx_icons, run_dx_imports, run_dx_packages_check,
    run_dx_style, run_dx_web_perf_check,
};
use super::serializer_artifacts::{
    ensure_dx_machine_artifact, read_dx_machine_document, sr_bool, sr_number, sr_string,
    sr_string_array, write_sr_artifact,
};

pub(super) const DX_EXTENSION_LIST_SCHEMA: &str = "dx.extension-list";

#[derive(Debug)]
struct OrchestratorOptions {
    json: bool,
    strict: bool,
    url: String,
    device: String,
    lighthouse: bool,
    lighthouse_report: Option<String>,
}

#[derive(Debug, Clone)]
struct ExtensionListConfig {
    pipeline: Vec<String>,
    watch_on: Vec<String>,
    style_command: String,
    icons_command: String,
    imports_command: String,
    forge_command: String,
    check_command: String,
    web_perf_command: String,
    generated_style_file: String,
    generated_icon_dir: String,
}

impl Default for ExtensionListConfig {
    fn default() -> Self {
        Self {
            pipeline: vec![
                "icons".to_string(),
                "imports".to_string(),
                "style".to_string(),
                "forge".to_string(),
                "check".to_string(),
                "web_perf".to_string(),
            ],
            watch_on: vec![
                "tsx".to_string(),
                "jsx".to_string(),
                "mdx".to_string(),
                "html".to_string(),
                "css".to_string(),
            ],
            style_command: "dx style build".to_string(),
            icons_command: "dx icons sync".to_string(),
            imports_command: "dx imports sync".to_string(),
            forge_command: "dx check packages".to_string(),
            check_command: "dx doctor".to_string(),
            web_perf_command: "dx check web-perf".to_string(),
            generated_style_file: "styles/generated.css".to_string(),
            generated_icon_dir: "components/icons".to_string(),
        }
    }
}

pub(super) fn run_dx_extension_orchestrator(cwd: &Path, args: &[String]) -> DxResult<()> {
    let options = parse_orchestrator_options(args)?;
    let project = cwd;
    let extension_list_created = ensure_dx_extension_list(project)?;
    let extension_list = load_extension_list_config(project)?;
    let web_perf_args = web_perf_args(&options);

    let mut steps = Vec::new();
    for step in &extension_list.pipeline {
        match step.as_str() {
            "icons" => steps.push(run_orchestrator_step(
                "icons",
                &extension_list.icons_command,
                80,
                || run_dx_icons(project, &["sync".to_string(), "--json".to_string()]),
            )),
            "imports" => steps.push(run_orchestrator_step(
                "imports",
                &extension_list.imports_command,
                80,
                || run_dx_imports(project, &["sync".to_string(), "--json".to_string()]),
            )),
            "style" => steps.push(run_orchestrator_step(
                "style",
                &extension_list.style_command,
                80,
                || run_dx_style(project, &["build".to_string(), "--json".to_string()]),
            )),
            "forge" => steps.push(run_orchestrator_step(
                "forge",
                &extension_list.forge_command,
                80,
                || run_dx_packages_check(project, &["run".to_string(), "--json".to_string()]),
            )),
            "check" => steps.push(run_orchestrator_step(
                "check",
                &extension_list.check_command,
                80,
                || run_dx_doctor(project, &["run".to_string(), "--json".to_string()]),
            )),
            "web_perf" => steps.push(run_orchestrator_step(
                "web_perf",
                &extension_list.web_perf_command,
                100,
                || run_dx_web_perf_check(project, &web_perf_args),
            )),
            unknown => steps.push(unknown_orchestrator_step(unknown)),
        }
    }

    let score = steps
        .iter()
        .filter_map(|step| step.get("score_awarded").and_then(Value::as_u64))
        .sum::<u64>()
        .min(500);
    let passed = steps.iter().all(|step| {
        step.get("status")
            .and_then(Value::as_str)
            .is_some_and(|status| status == "ok")
    });
    let lighthouse_score_source = if options.lighthouse_report.is_some() {
        "dx check web-perf --from-lighthouse"
    } else if options.lighthouse {
        "dx check web-perf --lighthouse"
    } else {
        "dx check web-perf --url"
    };
    let generated_at = Utc::now().to_rfc3339();
    let report = json!({
        "schema": "dx.run.extension_orchestrator",
        "schema_version": 1,
        "generated_at": generated_at,
        "project": project.to_string_lossy().replace('\\', "/"),
        "extension_list_schema": DX_EXTENSION_LIST_SCHEMA,
        "extension_list_file": "dx",
        "extension_list_created": extension_list_created,
        "pipeline": extension_list.pipeline.clone(),
        "tsx_change_behavior": {
            "watch_on": extension_list.watch_on.clone(),
            "style_command": extension_list.style_command.clone(),
            "icon_command": extension_list.icons_command.clone(),
            "generated_style_file": extension_list.generated_style_file.clone(),
            "generated_icon_dir": extension_list.generated_icon_dir.clone()
        },
        "score_scale": 500,
        "score": score,
        "passed": passed,
        "lighthouse_score_source": lighthouse_score_source,
        "steps": steps,
        "receipts": {
            "orchestrator": ".dx/run/orchestrator.sr",
            "style": ".dx/receipts/style/build.json",
            "icons": ".dx/receipts/icons/sync.json",
            "imports": ".dx/receipts/imports/sync.json",
            "forge": ".dx/receipts/check/packages.json",
            "check": ".dx/receipts/doctor/report.json",
            "web_perf": ".dx/check/500-points-lighthouse.sr"
        }
    });
    let sr_artifact = write_sr_artifact(
        project,
        ".dx/run/orchestrator.sr",
        &[
            ("schema", sr_string("dx.run.extension_orchestrator")),
            ("extension_list_schema", sr_string(DX_EXTENSION_LIST_SCHEMA)),
            ("extension_list_file", sr_string("dx")),
            ("passed", sr_bool(passed)),
            ("score", sr_number(score)),
            ("score_scale", sr_number(500)),
            ("pipeline", sr_string_array(&extension_list.pipeline)),
            (
                "lighthouse_score_source",
                sr_string(lighthouse_score_source),
            ),
            (
                "legacy_json",
                sr_string(".dx/receipts/run/orchestrator.json"),
            ),
            (
                "machine",
                sr_string(".dx/serializer/run-orchestrator.machine"),
            ),
        ],
    )
    .map_err(orchestrator_error)?;
    write_json_receipt(&project.join(".dx/receipts/run/orchestrator.json"), &report)?;

    if options.json {
        let output = serde_json::to_string_pretty(&report).map_err(orchestrator_error)?;
        println!("{output}");
    } else {
        println!("DX run extension orchestrator");
        println!("Score: {score}/500");
        println!("Passed: {passed}");
        println!("Extension list: dx");
        println!("TSX changes: dx style build + dx icons sync");
        println!("Receipt: {}", sr_artifact.source.display());
        println!("Machine: {}", sr_artifact.machine.display());
    }

    if options.strict && !passed {
        return Err(DxError::BuildFailed {
            message: "dx run extension orchestrator failed one or more steps".to_string(),
        });
    }

    Ok(())
}

fn load_extension_list_config(project: &Path) -> DxResult<ExtensionListConfig> {
    ensure_dx_machine_artifact(project).map_err(orchestrator_error)?;
    let path = project.join("dx");
    let document =
        if let Some(document) = read_dx_machine_document(project).map_err(orchestrator_error)? {
            document
        } else {
            let source = std::fs::read_to_string(&path).map_err(orchestrator_error)?;
            llm_to_document(&source).map_err(orchestrator_error)?
        };
    let mut config = ExtensionListConfig::default();

    if let Some(tools) = document.section_by_name("tools") {
        let name_index = tools.column_index("name");
        let command_index = tools.column_index("command");
        let enabled_index = tools.column_index("enabled");
        let output_index = tools.column_index("output");
        let mut pipeline = Vec::new();

        if let Some(name_index) = name_index {
            for row in &tools.rows {
                let Some(tool_name) = row.get(name_index).map(value_to_string) else {
                    continue;
                };
                let enabled = enabled_index
                    .and_then(|index| row.get(index))
                    .and_then(value_to_bool)
                    .unwrap_or(true);
                if !enabled {
                    continue;
                }

                if let Some(step_name) = orchestrator_step_name(&tool_name) {
                    pipeline.push(step_name.to_string());
                }

                let command = command_index
                    .and_then(|index| row.get(index))
                    .map(value_to_string);
                let output = output_index
                    .and_then(|index| row.get(index))
                    .map(value_to_string);
                match tool_name.as_str() {
                    "style" => {
                        if let Some(command) = command {
                            config.style_command = command;
                        }
                        if let Some(output) = output {
                            config.generated_style_file = output;
                        }
                    }
                    "icons" => {
                        if let Some(command) = command {
                            config.icons_command = command;
                        }
                        if let Some(output) = output {
                            config.generated_icon_dir = output;
                        }
                    }
                    "imports" => {
                        if let Some(command) = command {
                            config.imports_command = command;
                        }
                    }
                    "forge" => {
                        if let Some(command) = command {
                            config.forge_command = command;
                        }
                    }
                    "check" => {
                        if let Some(command) = command {
                            config.check_command = command;
                        }
                    }
                    "lighthouse" | "web_perf" => {
                        if let Some(command) = command {
                            config.web_perf_command = command;
                        }
                    }
                    _ => {}
                }
            }
        }

        if !pipeline.is_empty() {
            config.pipeline = pipeline;
        }
    }
    if let Some(style_watch) = table_value_by_key(&document, "watch", "tool", "style", "extensions")
    {
        let watch_on = split_words(&style_watch);
        if !watch_on.is_empty() {
            config.watch_on = watch_on;
        }
    }
    if let Some(path) = optional_string(&document, "style.generated_css") {
        config.generated_style_file = path;
    }
    if let Some(path) = optional_string(&document, "icons.generated_dir") {
        config.generated_icon_dir = path;
    }

    Ok(config)
}

fn optional_value<'a>(document: &'a DxDocument, key: &str) -> Option<&'a DxLlmValue> {
    document.get_path(key)
}

fn optional_string(document: &DxDocument, key: &str) -> Option<String> {
    optional_value(document, key).map(value_to_string)
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

fn table_value_by_key(
    document: &DxDocument,
    section_name: &str,
    key_column: &str,
    key: &str,
    value_column: &str,
) -> Option<String> {
    document
        .section_by_name(section_name)?
        .value_by_key(key_column, key, value_column)
        .map(value_to_string)
}

fn split_words(value: &str) -> Vec<String> {
    value
        .split_whitespace()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn orchestrator_step_name(tool_name: &str) -> Option<&'static str> {
    match tool_name {
        "serializer" => None,
        "style" => Some("style"),
        "icons" => Some("icons"),
        "imports" => Some("imports"),
        "forge" => Some("forge"),
        "check" => Some("check"),
        "lighthouse" | "web_perf" => Some("web_perf"),
        _ => Some("unknown"),
    }
}

fn parse_orchestrator_options(args: &[String]) -> DxResult<OrchestratorOptions> {
    let mut options = OrchestratorOptions {
        json: false,
        strict: false,
        url: "http://127.0.0.1:3000".to_string(),
        device: "both".to_string(),
        lighthouse: false,
        lighthouse_report: None,
    };
    let mut index = 0usize;
    while index < args.len() {
        match args[index].as_str() {
            "--json" => {
                options.json = true;
                index += 1;
            }
            "--strict" => {
                options.strict = true;
                index += 1;
            }
            "--url" => {
                options.url = required_value(args, index, "--url")?.to_string();
                index += 2;
            }
            "--device" => {
                options.device = required_value(args, index, "--device")?.to_string();
                index += 2;
            }
            "--lighthouse" => {
                options.lighthouse = true;
                index += 1;
            }
            "--from-lighthouse" => {
                options.lighthouse_report =
                    Some(required_value(args, index, "--from-lighthouse")?.to_string());
                index += 2;
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!("Unknown dx run extension-list option: {value}"),
                    field: Some("run".to_string()),
                });
            }
        }
    }

    Ok(options)
}

fn required_value<'a>(args: &'a [String], index: usize, flag: &str) -> DxResult<&'a str> {
    args.get(index + 1)
        .map(String::as_str)
        .ok_or_else(|| DxError::ConfigValidationError {
            message: format!("{flag} requires a value"),
            field: Some("run".to_string()),
        })
}

fn web_perf_args(options: &OrchestratorOptions) -> Vec<String> {
    if let Some(report) = &options.lighthouse_report {
        vec![
            "--from-lighthouse".to_string(),
            report.clone(),
            "--device".to_string(),
            options.device.clone(),
            "--json".to_string(),
        ]
    } else {
        let mut args = vec![
            "--url".to_string(),
            options.url.clone(),
            "--device".to_string(),
            options.device.clone(),
            "--json".to_string(),
        ];
        if options.lighthouse {
            args.insert(args.len() - 1, "--lighthouse".to_string());
        }
        args
    }
}

fn ensure_dx_extension_list(project: &Path) -> DxResult<bool> {
    let path = project.join("dx");
    if path.exists() {
        return Ok(false);
    }

    let project_name = project
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or("dx-project");
    std::fs::write(&path, default_extension_list(project_name)).map_err(orchestrator_error)?;
    Ok(true)
}

fn default_extension_list(project_name: &str) -> String {
    default_dx_project_config(project_name)
}

fn run_orchestrator_step<F>(name: &str, command: &str, max_score: u64, operation: F) -> Value
where
    F: FnOnce() -> anyhow::Result<PublicToolReport>,
{
    match operation() {
        Ok(report) => {
            let failed = report.json.get("passed").and_then(Value::as_bool) == Some(false);
            let status = if failed { "failed" } else { "ok" };
            let score_awarded = if failed {
                0
            } else if name == "web_perf" {
                web_perf_score(&report.json, max_score)
            } else {
                max_score
            };
            json!({
                "name": name,
                "command": command,
                "status": status,
                "passed": !failed,
                "score_awarded": score_awarded,
                "score_max": max_score,
                "report": report.json
            })
        }
        Err(error) => json!({
            "name": name,
            "command": command,
            "status": "failed",
            "passed": false,
            "score_awarded": 0,
            "score_max": max_score,
            "error": error.to_string()
        }),
    }
}

fn unknown_orchestrator_step(name: &str) -> Value {
    json!({
        "name": name,
        "command": "configured in dx",
        "status": "failed",
        "passed": false,
        "score_awarded": 0,
        "score_max": 0,
        "error": format!("Unknown tools[name command enabled output] step: {name}")
    })
}

fn web_perf_score(report: &Value, max_score: u64) -> u64 {
    let complete = report
        .get("score_completeness")
        .and_then(|value| value.get("complete"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if complete {
        return report
            .get("scores")
            .and_then(|scores| scores.get("total"))
            .and_then(Value::as_u64)
            .map(|total| (total / 4).min(max_score))
            .unwrap_or(0);
    }

    0
}

fn write_json_receipt(path: &Path, value: &Value) -> DxResult<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(orchestrator_error)?;
    }
    let json = serde_json::to_string_pretty(value).map_err(orchestrator_error)?;
    std::fs::write(path, json).map_err(orchestrator_error)?;
    Ok(())
}

fn orchestrator_error(error: impl std::fmt::Display) -> DxError {
    DxError::BuildFailed {
        message: format!("dx run extension orchestrator failed: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn web_perf_score_requires_complete_measured_total_for_full_credit() {
        let complete = json!({
            "measurement_status": "measured-from-lighthouse-json",
            "score_completeness": { "complete": true },
            "scores": { "total": 376 }
        });

        assert_eq!(web_perf_score(&complete, 100), 94);
    }

    #[test]
    fn web_perf_score_rejects_url_only_source_proof_without_measurements() {
        let source_only = json!({
            "measurement_status": "not-measured-cdp-runtime-not-attached",
            "score_completeness": { "complete": false },
            "scores": { "total": null }
        });

        assert_eq!(web_perf_score(&source_only, 100), 0);
    }

    #[test]
    fn web_perf_score_rejects_partial_or_audit_only_reports_without_totals() {
        let partial = json!({
            "measurement_status": "partial-lighthouse-json-missing-score-categories",
            "score_completeness": { "complete": false },
            "scores": { "total": null }
        });
        assert_eq!(web_perf_score(&partial, 100), 0);

        let audit_only = json!({
            "measurement_status": "audit-only-rule-findings",
            "findings": ["missing Lighthouse proof"],
            "diagnostics": [{ "id": "web-perf-proof-missing" }]
        });
        assert_eq!(web_perf_score(&audit_only, 100), 0);
    }
}
