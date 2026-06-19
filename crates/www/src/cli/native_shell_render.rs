use crate::error::DxResult;

use super::forge_error;
use super::native_shell_plan::NativeShellReport;
use super::options::DxOutputFormat;

pub(super) fn render_native_shell_report(
    report: &NativeShellReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(native_shell_markdown(report)),
        DxOutputFormat::Terminal => Ok(native_shell_terminal(report)),
    }
}

fn native_shell_terminal(report: &NativeShellReport) -> String {
    let mut lines = vec![
        format!("DX WWW native shell: {}", report.status),
        format!("Project: {}", report.project.name),
        format!("Target: {}", report.target),
        format!("Native shell: {}", report.native.shell),
        format!("Bridge: {}", report.native.bridge),
        format!("Dev URL: {}", report.native.dev_url),
        format!("Frontend dist: {}", report.native.frontend_dist),
        "Materialized files:".to_string(),
    ];
    lines.extend(
        report
            .materialized_files
            .iter()
            .map(|path| format!("  - {path}")),
    );
    if !report.blocked_files.is_empty() {
        lines.push("Existing files requiring --force for --write:".to_string());
        lines.extend(
            report
                .blocked_files
                .iter()
                .map(|path| format!("  - {path}")),
        );
    }
    lines.push(
        "Runtime proof: source only; native build and WebView run are not claimed.".to_string(),
    );
    append_terminal_next_commands(&mut lines, &report.next_commands);
    lines.join("\n")
}

fn native_shell_markdown(report: &NativeShellReport) -> String {
    let mut lines = vec![
        "# DX WWW Native Shell".to_string(),
        String::new(),
        format!("- Status: `{}`", report.status),
        format!("- Project: `{}`", report.project.name),
        format!("- Target: `{}`", report.target),
        format!("- Native shell: `{}`", report.native.shell),
        format!("- Bridge: `{}`", report.native.bridge),
        format!("- Dev URL: `{}`", report.native.dev_url),
        format!("- Frontend dist: `{}`", report.native.frontend_dist),
        String::new(),
        "## Materialized Files".to_string(),
    ];
    lines.extend(
        report
            .materialized_files
            .iter()
            .map(|path| format!("- `{path}`")),
    );
    if !report.blocked_files.is_empty() {
        lines.push(String::new());
        lines.push("## Existing Files".to_string());
        lines.extend(
            report
                .blocked_files
                .iter()
                .map(|path| format!("- `{path}`")),
        );
    }
    lines.push(String::new());
    lines.push(
        "Runtime proof remains source-only until `dx www build` and a native check/run complete."
            .to_string(),
    );
    append_markdown_next_commands(&mut lines, &report.next_commands);
    lines.join("\n")
}

fn append_terminal_next_commands(lines: &mut Vec<String>, commands: &[String]) {
    if commands.is_empty() {
        return;
    }

    lines.push("Next commands:".to_string());
    lines.extend(commands.iter().map(|command| format!("  - {command}")));
}

fn append_markdown_next_commands(lines: &mut Vec<String>, commands: &[String]) {
    if commands.is_empty() {
        return;
    }

    lines.push(String::new());
    lines.push("## Next Commands".to_string());
    lines.extend(
        commands
            .iter()
            .map(|command| format!("- {}", markdown_inline_code(command))),
    );
}

fn markdown_inline_code(value: &str) -> String {
    if value.contains('`') {
        format!("`` {value} ``")
    } else {
        format!("`{value}`")
    }
}

#[cfg(test)]
mod tests {
    use super::super::native_shell_plan::{
        NativeShellNative, NativeShellProject, NativeShellReport, NativeShellRuntimeProof,
    };
    use super::*;

    fn report_with_next_commands(next_commands: Vec<String>) -> NativeShellReport {
        NativeShellReport {
            schema: "dx.www.native_shell.receipt",
            command: "dx www native-shell",
            mode: "plan",
            status: "planned",
            target: "tauri",
            project: NativeShellProject {
                name: "dx-mobile-app".to_string(),
                root: "G:/Dx/www/dx-mobile-app".to_string(),
                dx_config: "dx".to_string(),
                www_output: ".dx/www/output",
            },
            native: NativeShellNative {
                bridge: "dx-native-mobile-companion".to_string(),
                shell: "tauri-webview",
                product_name: "DX Mobile App".to_string(),
                identifier: "com.essencefromexistence.dx.mobile".to_string(),
                dev_url: "http://127.0.0.1:3000".to_string(),
                dev_port: 3000,
                frontend_dist: "../.dx/www/output",
                native_root: "G:/Dx/www/native".to_string(),
                tauri_crate: "../../../native/crates/tauri".to_string(),
                tauri_build_crate: "../../../native/crates/tauri-build".to_string(),
                receipt_path: ".dx/native/native-shell-receipt.json",
            },
            materialized_files: vec!["src-tauri/tauri.conf.json".to_string()],
            blocked_files: Vec::new(),
            runtime_proof: NativeShellRuntimeProof {
                source_materialized: false,
                native_build: false,
                webview_run: false,
                production_output_present: false,
                node_modules_created: false,
            },
            next_commands,
        }
    }

    #[test]
    fn terminal_render_surfaces_next_commands_after_runtime_proof() {
        let report = report_with_next_commands(vec![
            "dx www build".to_string(),
            "dx www build --target android".to_string(),
        ]);

        let rendered = render_native_shell_report(&report, DxOutputFormat::Terminal).unwrap();

        let runtime_index = rendered
            .find("Runtime proof: source only; native build and WebView run are not claimed.")
            .expect("runtime proof line");
        let commands_index = rendered
            .find("Next commands:\n  - dx www build\n  - dx www build --target android")
            .expect("next command lines");
        assert!(
            runtime_index < commands_index,
            "runtime proof should stay outside the next-command section"
        );
    }

    #[test]
    fn markdown_render_surfaces_next_commands_after_runtime_proof() {
        let report = report_with_next_commands(vec![
            "dx www build".to_string(),
            "echo `quoted`".to_string(),
        ]);

        let rendered = render_native_shell_report(&report, DxOutputFormat::Markdown).unwrap();

        let runtime_index = rendered
            .find("Runtime proof remains source-only until `dx www build`")
            .expect("runtime proof paragraph");
        let commands_index = rendered
            .find("## Next Commands\n- `dx www build`\n- `` echo `quoted` ``")
            .expect("markdown next command list");
        assert!(
            runtime_index < commands_index,
            "runtime proof should not render under the Next Commands heading"
        );
    }
}
