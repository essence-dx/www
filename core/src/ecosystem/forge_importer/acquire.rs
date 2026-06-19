use serde::{Deserialize, Serialize};

use super::types::{DX_FORGE_IMPORT_ECOSYSTEMS, DxForgeImportEcosystem, DxForgeImportPhase};

/// Non-executing acquisition policy for an ecosystem.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeImportAcquireRule {
    /// Ecosystem this rule applies to.
    pub ecosystem: DxForgeImportEcosystem,
    /// Phase represented by this rule.
    pub phase: DxForgeImportPhase,
    /// Metadata Forge may inspect without executing package code.
    pub metadata_inputs: Vec<String>,
    /// Artifact inputs Forge may hash or quarantine without executing.
    pub artifact_inputs: Vec<String>,
    /// Commands that remain forbidden for this import gate.
    pub forbidden_commands: Vec<String>,
    /// Whether acquisition is allowed to execute package code.
    pub executes_package_code: bool,
}

/// Deterministic acquisition plan for one external package request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeImportAcquisitionPlan {
    /// Ecosystem this acquisition plan targets.
    pub ecosystem: DxForgeImportEcosystem,
    /// Phase represented by this plan.
    pub phase: DxForgeImportPhase,
    /// Original package name requested by the user.
    pub package_name: String,
    /// Forge package id that will own any accepted source.
    pub package_id: String,
    /// Registry metadata references tools may inspect without executing package code.
    pub metadata_references: Vec<String>,
    /// Archive or source references tools may hash, unpack, or quarantine without executing.
    pub artifact_references: Vec<String>,
    /// Project-relative source directory expected after a reviewed non-executing acquire.
    pub expected_source_dir: String,
    /// Project-relative quarantine directory for untrusted acquired artifacts.
    pub quarantine_dir: String,
    /// Project-relative acquisition evidence receipt path.
    pub evidence_receipt_path: String,
    /// Whether acquisition may perform live network fetching by default.
    pub live_fetching_enabled: bool,
    /// Whether acquisition may execute package-manager commands.
    pub package_manager_execution: bool,
    /// Whether acquisition may execute package code, build scripts, or lifecycle hooks.
    pub executes_package_code: bool,
    /// Whether materialization still requires a reviewed source directory.
    pub source_dir_required_for_materialization: bool,
    /// Commands that remain forbidden for this import gate.
    pub forbidden_commands: Vec<String>,
}

/// Return the non-executing acquire rule for one ecosystem.
pub fn acquire_rule_for_ecosystem(ecosystem: DxForgeImportEcosystem) -> DxForgeImportAcquireRule {
    let (metadata_inputs, artifact_inputs) = match ecosystem {
        DxForgeImportEcosystem::Npm => (
            vec!["package.json", "npm registry metadata", "exports map"],
            vec!["package tarball"],
        ),
        DxForgeImportEcosystem::Pip => (
            vec!["PyPI project metadata", "wheel metadata", "sdist metadata"],
            vec!["wheel archive", "source distribution"],
        ),
        DxForgeImportEcosystem::Cargo => (
            vec!["crates.io metadata", "Cargo.toml", "Cargo.lock metadata"],
            vec!["crate archive"],
        ),
        DxForgeImportEcosystem::Go => (
            vec!["module metadata", "go.mod", "go.sum metadata"],
            vec!["module zip"],
        ),
        DxForgeImportEcosystem::Jsr => (
            vec![
                "jsr.io package metadata",
                "jsr.json or deno.json metadata",
                "exports metadata",
            ],
            vec!["source archive", "module source files"],
        ),
        DxForgeImportEcosystem::Pub => (
            vec!["pub.dev metadata", "pubspec.yaml", "pubspec.lock metadata"],
            vec!["package archive"],
        ),
        DxForgeImportEcosystem::Maven => (
            vec![
                "Maven Central metadata",
                "pom.xml",
                "Gradle module metadata",
            ],
            vec!["source jar archive"],
        ),
        DxForgeImportEcosystem::Nuget => (
            vec![
                "NuGet registration metadata",
                "nuspec metadata",
                "project metadata",
            ],
            vec!["nupkg archive", "symbols/source package"],
        ),
        DxForgeImportEcosystem::Composer => (
            vec![
                "Packagist metadata",
                "composer.json",
                "composer.lock metadata",
            ],
            vec!["package archive"],
        ),
        DxForgeImportEcosystem::Gem => (
            vec!["RubyGems metadata", "gemspec", "Gemfile.lock metadata"],
            vec!["gem archive"],
        ),
        DxForgeImportEcosystem::Swift => (
            vec![
                "Swift Package Index metadata",
                "Package.swift",
                "Package.resolved metadata",
            ],
            vec!["source archive"],
        ),
        DxForgeImportEcosystem::Hex => (
            vec![
                "hex.pm metadata",
                "mix.exs",
                "rebar.config metadata",
                "mix.lock metadata",
            ],
            vec!["package tarball", "source archive"],
        ),
        DxForgeImportEcosystem::Cran => (
            vec!["CRAN metadata", "DESCRIPTION", "NAMESPACE"],
            vec!["source package tarball"],
        ),
    };

    DxForgeImportAcquireRule {
        ecosystem,
        phase: DxForgeImportPhase::Acquire,
        metadata_inputs: metadata_inputs.into_iter().map(str::to_string).collect(),
        artifact_inputs: artifact_inputs.into_iter().map(str::to_string).collect(),
        forbidden_commands: ecosystem
            .blocked_commands()
            .iter()
            .map(|command| (*command).to_string())
            .collect(),
        executes_package_code: false,
    }
}

/// Return the default acquire rules for currently modeled import ecosystems.
pub fn default_acquire_rules() -> Vec<DxForgeImportAcquireRule> {
    DX_FORGE_IMPORT_ECOSYSTEMS
        .iter()
        .copied()
        .map(acquire_rule_for_ecosystem)
        .collect()
}

/// Return a deterministic, non-executing acquisition plan for one package request.
pub fn acquisition_plan_for_package(
    ecosystem: DxForgeImportEcosystem,
    package_name: &str,
    package_id: &str,
) -> DxForgeImportAcquisitionPlan {
    let rule = acquire_rule_for_ecosystem(ecosystem);
    let package_name = package_name.trim();
    let package_slug = acquisition_package_slug(package_name);
    let package_id_slug = acquisition_package_slug(package_id);
    let ecosystem_segment = ecosystem.as_segment();

    DxForgeImportAcquisitionPlan {
        ecosystem,
        phase: DxForgeImportPhase::Acquire,
        package_name: package_name.to_string(),
        package_id: package_id.to_string(),
        metadata_references: acquisition_metadata_references(ecosystem, package_name),
        artifact_references: acquisition_artifact_references(ecosystem, package_name),
        expected_source_dir: format!(".dx/cache/{ecosystem_segment}/{package_slug}/package"),
        quarantine_dir: format!(".dx/forge/quarantine/{ecosystem_segment}/{package_slug}"),
        evidence_receipt_path: format!(".dx/forge/import-receipts/{package_id_slug}-acquire.sr"),
        live_fetching_enabled: matches!(ecosystem, DxForgeImportEcosystem::Npm),
        package_manager_execution: false,
        executes_package_code: false,
        source_dir_required_for_materialization: true,
        forbidden_commands: rule.forbidden_commands,
    }
}

fn acquisition_metadata_references(
    ecosystem: DxForgeImportEcosystem,
    package_name: &str,
) -> Vec<String> {
    match ecosystem {
        DxForgeImportEcosystem::Npm => vec![
            format!("npm registry packument for `{package_name}`"),
            format!("package.json from npm tarball `{package_name}`"),
        ],
        DxForgeImportEcosystem::Pip => vec![
            format!("PyPI JSON metadata for `{package_name}`"),
            format!("METADATA from wheel or sdist `{package_name}`"),
        ],
        DxForgeImportEcosystem::Cargo => vec![
            format!("crates.io metadata for `{package_name}`"),
            format!("Cargo.toml from crate archive `{package_name}`"),
        ],
        DxForgeImportEcosystem::Go => vec![
            format!("Go module metadata for `{package_name}`"),
            format!("go.mod from module zip `{package_name}`"),
        ],
        DxForgeImportEcosystem::Jsr => vec![
            format!("JSR package metadata for `{package_name}`"),
            format!("jsr.json or deno.json from source archive `{package_name}`"),
        ],
        DxForgeImportEcosystem::Pub => vec![
            format!("pub.dev metadata for `{package_name}`"),
            format!("pubspec.yaml from package archive `{package_name}`"),
        ],
        DxForgeImportEcosystem::Maven => vec![
            format!("Maven metadata for `{package_name}`"),
            format!("pom.xml or module metadata for `{package_name}`"),
        ],
        DxForgeImportEcosystem::Nuget => vec![
            format!("NuGet registration metadata for `{package_name}`"),
            format!("nuspec metadata for `{package_name}`"),
        ],
        DxForgeImportEcosystem::Composer => vec![
            format!("Packagist metadata for `{package_name}`"),
            format!("composer.json from package archive `{package_name}`"),
        ],
        DxForgeImportEcosystem::Gem => vec![
            format!("RubyGems metadata for `{package_name}`"),
            format!("gemspec metadata for `{package_name}`"),
        ],
        DxForgeImportEcosystem::Swift => vec![
            format!("Swift package registry metadata for `{package_name}`"),
            format!("Package.swift from source archive `{package_name}`"),
        ],
        DxForgeImportEcosystem::Hex => vec![
            format!("Hex.pm metadata for `{package_name}`"),
            format!("mix.exs or rebar.config metadata for `{package_name}`"),
        ],
        DxForgeImportEcosystem::Cran => vec![
            format!("CRAN metadata for `{package_name}`"),
            format!("DESCRIPTION and NAMESPACE for `{package_name}`"),
        ],
    }
}

fn acquisition_artifact_references(
    ecosystem: DxForgeImportEcosystem,
    package_name: &str,
) -> Vec<String> {
    match ecosystem {
        DxForgeImportEcosystem::Npm => vec![format!(
            "npm dist.tarball for `{package_name}` after integrity verification"
        )],
        DxForgeImportEcosystem::Pip => vec![format!(
            "PyPI wheel or sdist for `{package_name}` after hash verification"
        )],
        DxForgeImportEcosystem::Cargo => vec![format!(
            "crates.io crate archive for `{package_name}` after checksum verification"
        )],
        DxForgeImportEcosystem::Go => vec![format!(
            "Go module zip for `{package_name}` after sumdb verification"
        )],
        DxForgeImportEcosystem::Jsr => vec![format!(
            "JSR source archive for `{package_name}` after integrity verification"
        )],
        DxForgeImportEcosystem::Pub => vec![format!(
            "pub.dev package archive for `{package_name}` after hash verification"
        )],
        DxForgeImportEcosystem::Maven => vec![format!(
            "Maven source jar for `{package_name}` after checksum verification"
        )],
        DxForgeImportEcosystem::Nuget => vec![format!(
            "NuGet source package for `{package_name}` after hash verification"
        )],
        DxForgeImportEcosystem::Composer => vec![format!(
            "Composer package archive for `{package_name}` after dist hash verification"
        )],
        DxForgeImportEcosystem::Gem => vec![format!(
            "Ruby gem archive for `{package_name}` after checksum verification"
        )],
        DxForgeImportEcosystem::Swift => vec![format!(
            "Swift package source archive for `{package_name}` after revision verification"
        )],
        DxForgeImportEcosystem::Hex => vec![format!(
            "Hex package tarball for `{package_name}` after checksum verification"
        )],
        DxForgeImportEcosystem::Cran => vec![format!(
            "CRAN source package tarball for `{package_name}` after checksum verification"
        )],
    }
}

pub fn acquisition_package_slug(package_name: &str) -> String {
    let mut slug = String::new();
    for character in package_name.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
        } else if matches!(character, '-' | '_' | '.') {
            slug.push(character);
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }
    let slug = slug.trim_matches('-');
    if slug.is_empty() {
        "package".to_string()
    } else {
        slug.to_string()
    }
}
