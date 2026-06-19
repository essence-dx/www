use std::path::Path;

use chrono::Utc;
use dx_compiler::ecosystem::public_forge_package_id;

use crate::error::DxResult;

use super::forge_packages_options::{DxForgePackagesCommandOptions, parse_forge_packages_options};
use super::options::DxOutputFormat;
use super::{
    forge_error, forge_package_discovery_public_api, launch_discovery_contract,
    www_template_catalog_metadata,
};

pub(super) fn run_forge_packages(cwd: &Path, args: &[String]) -> DxResult<()> {
    let DxForgePackagesCommandOptions {
        output,
        format,
        quiet,
    } = parse_forge_packages_options(cwd, args)?;

    let discovery = launch_discovery_contract();
    let packages = www_template_catalog_metadata()
        .into_iter()
        .map(|package| {
            let canonical_package_id = package["package_id"].as_str().unwrap_or("unknown");
            let package_id = public_forge_package_id(canonical_package_id);
            serde_json::json!({
                "package_id": package_id,
                "discoverable": true,
                "discoverable_by": ["dx-cli", "zed"],
                "metadata_command": "dx forge packages --json",
                "cli_add": package["command"],
                "template_role": package["role"],
                "public_api": forge_package_discovery_public_api(canonical_package_id),
                "env": package["env"],
                "app_owned_boundaries": package["app_owned_boundaries"],
            })
        })
        .collect::<Vec<_>>();
    let report = serde_json::json!({
        "schema": "dx.forge.packages",
        "generated_at": Utc::now().to_rfc3339(),
        "source": "dx-www",
        "discovery": discovery,
        "packages": packages,
    });

    let rendered = match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Terminal => forge_packages_terminal(&report),
        DxOutputFormat::Markdown => forge_packages_markdown(&report),
    };

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent).map_err(forge_error)?;
        }
        std::fs::write(&output, &rendered).map_err(forge_error)?;
    }

    if !quiet {
        println!("{rendered}");
    }

    Ok(())
}

fn forge_packages_terminal(report: &serde_json::Value) -> String {
    format!(
        "DX Forge packages\nSchema: {}\nPackages: {}\n",
        report["schema"].as_str().unwrap_or("dx.forge.packages"),
        report["packages"]
            .as_array()
            .map(|packages| packages.len())
            .unwrap_or(0)
    )
}

fn forge_packages_markdown(report: &serde_json::Value) -> String {
    let mut output =
        String::from("# DX Forge Packages\n\n| Package | Command | Role |\n| --- | --- | --- |\n");
    for package in report["packages"].as_array().into_iter().flatten() {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` |\n",
            package["package_id"].as_str().unwrap_or("unknown"),
            package["cli_add"]
                .as_str()
                .unwrap_or("dx forge packages --json"),
            package["template_role"].as_str().unwrap_or("supporting")
        ));
    }
    output
}
