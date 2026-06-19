use crate::error::{DxError, DxResult};
use dx_compiler::ecosystem::{DxForgeAddOutcome, public_forge_package_id};

pub(super) fn dx_add_outcome_terminal(outcome: &DxForgeAddOutcome) -> String {
    let package = &outcome.receipt.package;
    let package_id = public_forge_package_id(&package.package_id);
    let mode = if outcome.wrote_files {
        "write"
    } else {
        "dry-run"
    };
    let file_action = if outcome.wrote_files {
        "Files written"
    } else {
        "Files planned"
    };

    let mut output = format!(
        "DX add {mode}\n\nPackage: {}\nVariant: {}\nScore: {}/100\n{file_action}: {}\n",
        package_id,
        package.variant,
        outcome.receipt.risk_score,
        outcome.receipt.files_written.len()
    );

    if let Some(path) = &outcome.manifest_path {
        output.push_str(&format!("Manifest: {}\n", path.display()));
    }
    if let Some(path) = &outcome.receipt_path {
        output.push_str(&format!("Receipt: {}\n", path.display()));
    }

    if !outcome.receipt.files_written.is_empty() {
        output.push_str("\nFiles:\n");
        for file in outcome.receipt.files_written.iter().take(8) {
            output.push_str(&format!("  - {} ({} B)\n", file.path, file.bytes));
        }
        if outcome.receipt.files_written.len() > 8 {
            output.push_str(&format!(
                "  - ... {} more\n",
                outcome.receipt.files_written.len() - 8
            ));
        }
    }

    if !outcome.receipt.policy_decisions.is_empty() {
        output.push_str("\nPolicy:\n");
        for decision in outcome.receipt.policy_decisions.iter().take(5) {
            output.push_str(&format!(
                "  - {} {}: {}\n",
                decision.traffic.as_str(),
                decision.policy,
                decision.message
            ));
        }
        if outcome.receipt.policy_decisions.len() > 5 {
            output.push_str(&format!(
                "  - ... {} more policy decisions\n",
                outcome.receipt.policy_decisions.len() - 5
            ));
        }
    }

    if outcome.wrote_files {
        output.push_str(
            "\nResult: source-owned files are materialized. No node_modules were created.\n",
        );
    } else {
        let variant_flag = if package.variant == "default" {
            String::new()
        } else {
            format!(" --variant {}", package.variant)
        };
        output.push_str(
            "\nResult: dry run only. No files were written and no package scripts ran.\n",
        );
        output.push_str("Write this package:\n");
        output.push_str(&format!(
            "  dx add {}{} --write\n",
            package_id, variant_flag
        ));
        output.push_str("Machine-readable review:\n");
        output.push_str(&format!(
            "  dx add {}{} --dry-run --format json\n",
            package_id, variant_flag
        ));
    }

    output
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PublicForgeAddRequest {
    pub(super) package_ids: Vec<String>,
    pub(super) selected_exports: Vec<String>,
    pub(super) surface_packages: bool,
}

pub(super) fn parse_public_forge_add_request(
    package_spec: &str,
    only: Option<&str>,
) -> DxResult<PublicForgeAddRequest> {
    let (base, inline_exports) = package_spec
        .split_once('#')
        .map(|(base, exports)| (base.trim(), Some(exports)))
        .unwrap_or_else(|| (package_spec.trim(), None));

    if base.is_empty() {
        return Err(DxError::ConfigValidationError {
            message: "dx forge add requires a package id".to_string(),
            field: Some("forge add".to_string()),
        });
    }

    let selected_exports = parse_public_forge_export_list(inline_exports, only);
    if selected_exports.is_empty() {
        return Ok(PublicForgeAddRequest {
            package_ids: vec![base.to_string()],
            selected_exports,
            surface_packages: false,
        });
    }

    let package_ids = match base {
        "shadcn/ui" => selected_exports
            .iter()
            .map(|surface| format!("shadcn/ui/{surface}"))
            .collect(),
        "ui" => selected_exports
            .iter()
            .map(|surface| format!("ui/{surface}"))
            .collect(),
        _ => {
            return Ok(PublicForgeAddRequest {
                package_ids: vec![base.to_string()],
                selected_exports,
                surface_packages: false,
            });
        }
    };

    Ok(PublicForgeAddRequest {
        package_ids,
        selected_exports,
        surface_packages: true,
    })
}

fn parse_public_forge_export_list(inline: Option<&str>, only: Option<&str>) -> Vec<String> {
    let mut selected = Vec::new();
    for value in inline
        .into_iter()
        .chain(only)
        .flat_map(|list| list.split(','))
    {
        let value = value.trim();
        if !value.is_empty() && !selected.iter().any(|existing| existing == value) {
            selected.push(value.to_string());
        }
    }
    selected
}
