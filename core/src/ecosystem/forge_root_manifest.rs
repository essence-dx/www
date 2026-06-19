//! Root `dx` package manifest support for source-owned Forge packages.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

use super::forge_security::{
    DxForgeAdvisoryCoverageKind, DxForgeAdvisoryMetadata, DxForgeLicenseReviewMetadata,
    DxForgeProvenanceMetadata, DxSourceFile, DxSourceKind, DxSourcePackage,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRootPackageManifest {
    pub package_id: String,
    pub version: String,
    pub description: String,
    pub license: String,
    pub source_root: PathBuf,
    pub visibility: String,
    pub registry: String,
    pub allow_selective_imports: bool,
    pub default_exports: Vec<String>,
    pub files: Vec<DxForgeRootPackageFile>,
    pub exports: Vec<DxForgeRootPackageExport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRootPackageFile {
    pub from: String,
    pub to: String,
    pub surface: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRootPackageExport {
    pub name: String,
    pub files: Vec<String>,
}

pub(super) fn source_package_from_root_dx_selected_exports(
    project: &Path,
    package_id: &str,
    selected_exports: &[String],
) -> Result<DxSourcePackage> {
    let manifest = load_root_dx_package_manifest(project)?.with_context(|| {
        format!(
            "root dx does not declare a Forge package; add package.*, forge.package=true, forge.files.entries, and forge.exports.<name>.files before installing `{package_id}` from selected exports"
        )
    })?;
    if manifest.package_id != package_id.trim() {
        bail!(
            "root dx package is `{}`, not `{}`",
            manifest.package_id,
            package_id
        );
    }

    let files = selected_manifest_files(&manifest, selected_exports)?;
    let mut source_files = Vec::new();
    for file in files {
        let source_path = manifest.source_root.join(&file.from);
        let content = fs::read_to_string(&source_path)
            .with_context(|| format!("read root dx package source `{}`", source_path.display()))?;
        let hash = blake3::hash(content.as_bytes()).to_hex().to_string();
        source_files.push(DxSourceFile {
            path: file.to.clone(),
            logical_path: Some(file.from.clone()),
            hash,
            bytes: content.len() as u64,
            content: Some(content),
        });
    }

    Ok(DxSourcePackage {
        package_id: manifest.package_id.clone(),
        upstream_name: manifest.description.clone(),
        version: manifest.version.clone(),
        generator: "dx-forge/root-dx".to_string(),
        variant: selected_variant_name(selected_exports),
        last_accepted_update: None,
        rollback_receipt: None,
        source_kind: DxSourceKind::Local,
        integrity_hash: package_integrity_hash(&source_files),
        license: manifest.license.clone(),
        provenance: DxForgeProvenanceMetadata {
            source: "root-dx-package-manifest".to_string(),
            upstream_reference: Some(manifest.source_root.display().to_string()),
            verified: false,
            note: "Package files were planned from the project's root dx manifest; external provenance verification is not claimed.".to_string(),
        },
        advisory_review: DxForgeAdvisoryMetadata {
            coverage_kind: DxForgeAdvisoryCoverageKind::Missing,
            provider: "none".to_string(),
            live_coverage: false,
            finding_count: 0,
            reviewed_at: None,
            note: "Root dx packages do not have live advisory coverage attached by Forge yet."
                .to_string(),
        },
        license_review: DxForgeLicenseReviewMetadata {
            declared_license: manifest.license,
            reviewed: false,
            reviewed_at: None,
            note: "License is recorded from the root dx package declaration only; no formal DX legal review is claimed.".to_string(),
        },
        files: source_files,
    })
}

pub(super) fn load_root_dx_package_manifest(
    project: &Path,
) -> Result<Option<DxForgeRootPackageManifest>> {
    let dx_path = project.join("dx");
    if !dx_path.exists() {
        return Ok(None);
    }

    let text =
        fs::read_to_string(&dx_path).with_context(|| format!("read `{}`", dx_path.display()))?;
    let value: toml::Value = match toml::from_str(&text) {
        Ok(value) => value,
        Err(_error) if looks_like_serializer_project_dx(&text) => return Ok(None),
        Err(error) => {
            return Err(error).with_context(|| format!("parse `{}`", dx_path.display()));
        }
    };
    let Some(package) = value.get("package").and_then(toml::Value::as_table) else {
        return Ok(None);
    };
    let Some(forge) = value.get("forge").and_then(toml::Value::as_table) else {
        return Ok(None);
    };
    if !toml_bool(forge.get("package")).unwrap_or(false) {
        return Ok(None);
    }

    let package_id = toml_required_string(package.get("name"), "package.name")?;
    validate_package_id(&package_id)?;
    let version = toml_required_string(package.get("version"), "package.version")?;
    let description = toml_string(package.get("description")).unwrap_or_else(|| package_id.clone());
    let license = toml_required_string(package.get("license"), "package.license")?;
    let source = toml_required_string(package.get("source"), "package.source")?;
    let source_root = resolve_package_source(project, &source);
    let visibility = toml_string(forge.get("visibility")).unwrap_or_else(|| "private".to_string());
    let registry = toml_string(forge.get("registry")).unwrap_or_else(|| "local".to_string());
    let install = forge.get("install").and_then(toml::Value::as_table);
    let allow_selective_imports = install
        .and_then(|table| toml_bool(table.get("allow_selective_imports")))
        .unwrap_or(false);
    let default_exports = install
        .and_then(|table| table.get("default_exports"))
        .map(toml_string_array)
        .transpose()?
        .unwrap_or_default();
    let files = parse_root_files(forge)?;
    if files.is_empty() {
        bail!("forge.files must contain at least one file");
    }
    let exports = parse_root_exports(forge)?;

    Ok(Some(DxForgeRootPackageManifest {
        package_id,
        version,
        description,
        license,
        source_root,
        visibility,
        registry,
        allow_selective_imports,
        default_exports,
        files,
        exports,
    }))
}

fn looks_like_serializer_project_dx(text: &str) -> bool {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .any(|line| {
            matches!(
                line.split(['(', '[']).next().unwrap_or_default(),
                "project"
                    | "contract"
                    | "runtime"
                    | "www"
                    | "paths"
                    | "dev"
                    | "build"
                    | "tools"
                    | "watch"
                    | "check"
                    | "biome"
                    | "biome_rules"
                    | "ignore"
                    | "style"
                    | "icons"
                    | "imports"
                    | "ui"
                    | "classnames"
                    | "classnames_compat"
                    | "docs"
            )
        })
}

fn parse_root_file(value: &toml::Value) -> Result<DxForgeRootPackageFile> {
    let table = value
        .as_table()
        .with_context(|| "forge.files entries must be tables")?;
    let from = toml_required_string(table.get("from"), "forge.files.from")?;
    validate_source_relative_path(&from).with_context(|| format!("validate from `{from}`"))?;
    let to = toml_required_string(table.get("to"), "forge.files.to")?;
    validate_project_relative_path(&to).with_context(|| format!("validate to `{to}`"))?;
    let surface = toml_string(table.get("surface"));
    Ok(DxForgeRootPackageFile { from, to, surface })
}

fn parse_root_files(forge: &toml::Table) -> Result<Vec<DxForgeRootPackageFile>> {
    let files = forge
        .get("files")
        .with_context(|| "forge.files must contain entries")?;
    match files {
        toml::Value::Array(files) => files.iter().map(parse_root_file).collect(),
        toml::Value::Table(table) => {
            if let Some(entries) = table.get("entries") {
                return toml_string_array(entries)?
                    .iter()
                    .map(|entry| parse_root_file_entry(entry))
                    .collect();
            }

            table
                .values()
                .filter(|value| value.as_table().is_some())
                .map(parse_root_file)
                .collect()
        }
        _ => bail!(
            "forge.files must be [[forge.files]] tables or forge.files.entries serializer strings"
        ),
    }
}

fn parse_root_file_entry(entry: &str) -> Result<DxForgeRootPackageFile> {
    let (from, rest) = entry
        .split_once("->")
        .or_else(|| entry.split_once("=>"))
        .with_context(|| "forge.files.entries items must use `from -> to | surface`")?;
    let from = from.trim().to_string();
    let mut to = rest.trim().to_string();
    let mut surface = None;

    for delimiter in ["|", "@"] {
        if let Some((path, name)) = to.split_once(delimiter) {
            surface = non_empty_string(name.trim());
            to = path.trim().to_string();
            break;
        }
    }

    if surface.is_none() && to.ends_with(')') {
        if let Some(start) = to.rfind('(') {
            let name = to[start + 1..to.len() - 1].trim().to_string();
            let path = to[..start].trim().to_string();
            if !name.is_empty() && !path.is_empty() {
                surface = Some(name);
                to = path;
            }
        }
    }

    validate_source_relative_path(&from).with_context(|| format!("validate from `{from}`"))?;
    validate_project_relative_path(&to).with_context(|| format!("validate to `{to}`"))?;
    Ok(DxForgeRootPackageFile { from, to, surface })
}

fn parse_root_export(value: &toml::Value) -> Result<DxForgeRootPackageExport> {
    let table = value
        .as_table()
        .with_context(|| "forge.exports entries must be tables")?;
    let name = toml_required_string(table.get("name"), "forge.exports.name")?;
    let files = table
        .get("files")
        .map(toml_string_array)
        .transpose()?
        .unwrap_or_default();
    for file in &files {
        validate_project_relative_path(file)
            .with_context(|| format!("validate forge.exports.{name} file `{file}`"))?;
    }
    Ok(DxForgeRootPackageExport { name, files })
}

fn parse_root_exports(forge: &toml::Table) -> Result<Vec<DxForgeRootPackageExport>> {
    let Some(exports) = forge.get("exports") else {
        return Ok(Vec::new());
    };
    match exports {
        toml::Value::Array(exports) => exports.iter().map(parse_root_export).collect(),
        toml::Value::Table(table) => table
            .iter()
            .map(|(name, value)| parse_named_root_export(name, value))
            .collect(),
        _ => bail!("forge.exports must be [[forge.exports]] tables or forge.exports.<name>.files"),
    }
}

fn parse_named_root_export(name: &str, value: &toml::Value) -> Result<DxForgeRootPackageExport> {
    let (export_name, files) = match value {
        toml::Value::Array(_) => (name.to_string(), toml_string_array(value)?),
        toml::Value::Table(table) => {
            let export_name = toml_string(table.get("name")).unwrap_or_else(|| name.to_string());
            let files = table
                .get("files")
                .map(toml_string_array)
                .transpose()?
                .unwrap_or_default();
            (export_name, files)
        }
        _ => bail!("forge.exports.{name} must be an array or table"),
    };
    for file in &files {
        validate_project_relative_path(file)
            .with_context(|| format!("validate forge.exports.{export_name} file `{file}`"))?;
    }
    Ok(DxForgeRootPackageExport {
        name: export_name,
        files,
    })
}

fn selected_manifest_files<'a>(
    manifest: &'a DxForgeRootPackageManifest,
    selected_exports: &[String],
) -> Result<Vec<&'a DxForgeRootPackageFile>> {
    let mut selected = normalize_export_names(selected_exports);
    if selected.is_empty() {
        selected = normalize_export_names(&manifest.default_exports);
    }
    if selected.is_empty() {
        return Ok(manifest.files.iter().collect());
    }
    if !manifest.allow_selective_imports && !selected_exports.is_empty() {
        bail!(
            "root dx package `{}` does not allow selective imports",
            manifest.package_id
        );
    }

    let mut selected_destinations = BTreeSet::new();
    for export_name in &selected {
        let mut matched = false;
        if let Some(export) = manifest
            .exports
            .iter()
            .find(|export| export.name == *export_name)
        {
            matched = true;
            if export.files.is_empty() {
                for file in manifest
                    .files
                    .iter()
                    .filter(|file| file.surface.as_deref() == Some(export_name.as_str()))
                {
                    selected_destinations.insert(file.to.clone());
                }
            } else {
                for export_file in &export.files {
                    selected_destinations.insert(export_file.clone());
                }
            }
        }

        for file in manifest
            .files
            .iter()
            .filter(|file| file.surface.as_deref() == Some(export_name.as_str()))
        {
            matched = true;
            selected_destinations.insert(file.to.clone());
        }

        if !matched {
            bail!(
                "root dx package `{}` does not define export `{}`",
                manifest.package_id,
                export_name
            );
        }
    }

    let selected_files = manifest
        .files
        .iter()
        .filter(|file| selected_destinations.contains(&file.to))
        .collect::<Vec<_>>();
    if selected_files.is_empty() {
        bail!(
            "root dx package `{}` selected exports `{}` did not match any forge.files entries",
            manifest.package_id,
            selected.join(",")
        );
    }
    Ok(selected_files)
}

fn normalize_export_names(values: &[String]) -> Vec<String> {
    let mut names = Vec::new();
    for value in values {
        let value = value.trim();
        if !value.is_empty() && !names.iter().any(|existing| existing == value) {
            names.push(value.to_string());
        }
    }
    names
}

fn selected_variant_name(selected_exports: &[String]) -> String {
    let selected = normalize_export_names(selected_exports);
    if selected.is_empty() {
        "default".to_string()
    } else {
        let joined = selected.join("-");
        let sanitized = joined
            .chars()
            .map(|ch| {
                if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                    ch
                } else {
                    '-'
                }
            })
            .collect::<String>();
        let sanitized = sanitized.trim_matches('-');
        if sanitized.is_empty() {
            "export-selected".to_string()
        } else {
            format!("export-{sanitized}")
        }
    }
}

fn toml_required_string(value: Option<&toml::Value>, field: &str) -> Result<String> {
    toml_string(value).with_context(|| format!("{field} is required and must be a string"))
}

fn toml_string(value: Option<&toml::Value>) -> Option<String> {
    value
        .and_then(toml::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn non_empty_string(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn toml_bool(value: Option<&toml::Value>) -> Option<bool> {
    value.and_then(toml::Value::as_bool)
}

fn toml_string_array(value: &toml::Value) -> Result<Vec<String>> {
    let Some(values) = value.as_array() else {
        bail!("expected an array of strings");
    };
    values
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned)
                .with_context(|| "expected a non-empty string")
        })
        .collect()
}

fn resolve_package_source(project: &Path, source: &str) -> PathBuf {
    let path = PathBuf::from(source);
    if path.is_absolute() {
        path
    } else {
        project.join(path)
    }
}

fn validate_package_id(package_id: &str) -> Result<()> {
    if package_id.trim().is_empty() {
        bail!("package id cannot be empty");
    }
    if package_id.contains('\\') || package_id.contains("..") {
        bail!("package id must use safe `/`-separated segments");
    }
    Ok(())
}

fn validate_source_relative_path(path: &str) -> Result<()> {
    validate_relative_slash_path(path)
}

fn validate_project_relative_path(path: &str) -> Result<()> {
    validate_relative_slash_path(path)
}

fn validate_relative_slash_path(path: &str) -> Result<()> {
    if path.trim().is_empty() {
        bail!("path cannot be empty");
    }
    if path.contains('\\') {
        bail!("path must use `/` separators");
    }
    let path = Path::new(path);
    if path.is_absolute() {
        bail!("path must be relative");
    }
    for component in path.components() {
        if matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        ) {
            bail!("path cannot escape the configured root");
        }
    }
    Ok(())
}

fn package_integrity_hash(files: &[DxSourceFile]) -> String {
    let mut hasher = blake3::Hasher::new();
    for file in files {
        hasher.update(file.path.as_bytes());
        hasher.update(file.hash.as_bytes());
    }
    hasher.finalize().to_hex().to_string()
}
