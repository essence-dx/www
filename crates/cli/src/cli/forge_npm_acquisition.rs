use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, bail};
use base64::Engine as _;
use chrono::Utc;
use reqwest::Url;
use serde::Serialize;
use serde_json::Value;
use sha2::Digest as _;

use dx_compiler::ecosystem::{
    DxForgeImportEcosystem, acquisition_package_slug, acquisition_plan_for_package,
    validate_import_package_name,
};

use super::forge_acquire_options::DxForgeAcquireCommandOptions;
use super::forge_import_plan::build_forge_import_plan_report_with_selection;
use super::forge_npm_archive::extract_npm_tgz;
use super::serializer_artifacts::{
    sr_bool, sr_null, sr_number, sr_string, sr_string_array, write_json_receipt_machine_alias,
    write_sr_artifact,
};

const FORGE_ACQUIRE_SCHEMA: &str = "dx.forge.package_acquisition";
const DEFAULT_NPM_REGISTRY_URL: &str = "https://registry.npmjs.org";
const MAX_NPM_TARBALL_BYTES: usize = 100 * 1024 * 1024;

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeAcquireReport {
    schema: &'static str,
    model_version: u32,
    generated_at: String,
    passed: bool,
    score: u8,
    ecosystem: String,
    package_name: String,
    package_id: String,
    requested_version: Option<String>,
    version_resolved_from: String,
    version: String,
    registry_url: String,
    metadata_url: String,
    tarball_url: String,
    tarball_integrity: Option<String>,
    tarball_sha512: Option<String>,
    integrity_verified: bool,
    cache_dir: PathBuf,
    source_dir: PathBuf,
    source_dir_ready: bool,
    packument_path: PathBuf,
    tarball_path: PathBuf,
    evidence_path: PathBuf,
    receipt_path: PathBuf,
    receipt_sr_path: PathBuf,
    receipt_machine_path: PathBuf,
    receipt_json_machine_path: PathBuf,
    import_plan_path: Option<PathBuf>,
    import_plan_sr_path: Option<PathBuf>,
    import_plan_machine_path: Option<PathBuf>,
    import_plan_json_machine_path: Option<PathBuf>,
    package_manager_execution_allowed: bool,
    package_installs_run: bool,
    lifecycle_scripts_declared: bool,
    lifecycle_script_names: Vec<String>,
    lifecycle_scripts_executed: bool,
    lifecycle_script_status: String,
    files_extracted: usize,
    bytes_extracted: u64,
    import_plan: Value,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Clone)]
struct NpmPackageSelection {
    version: String,
    version_resolved_from: String,
    tarball_url: String,
    integrity: Option<String>,
    license: Option<String>,
}

pub(super) fn acquire_npm_package(
    options: &DxForgeAcquireCommandOptions,
) -> anyhow::Result<DxForgeAcquireReport> {
    let ecosystem = DxForgeImportEcosystem::from_segment(&options.ecosystem)
        .context("parse Forge acquire ecosystem")?;
    if ecosystem != DxForgeImportEcosystem::Npm {
        bail!(
            "dx forge acquire currently performs live registry acquisition for npm only; {} remains plan/import-gated",
            ecosystem.as_segment()
        );
    }
    validate_import_package_name(ecosystem, &options.package_name)?;

    let project = &options.project;
    let package_id = format!("npm/{}", options.package_name);
    let registry_url = validate_npm_registry_url(
        options
            .registry_url
            .as_deref()
            .unwrap_or(DEFAULT_NPM_REGISTRY_URL),
    )?;
    let metadata_url = format!(
        "{}/{}",
        registry_url,
        npm_registry_package_path(&options.package_name)
    );
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("dx-forge-acquire/1.0")
        .build()
        .context("build Forge npm registry client")?;

    let packument: Value = client
        .get(&metadata_url)
        .send()
        .with_context(|| format!("fetch npm metadata `{metadata_url}`"))?
        .error_for_status()
        .with_context(|| format!("npm metadata request failed for `{metadata_url}`"))?
        .json()
        .with_context(|| format!("decode npm metadata `{metadata_url}`"))?;

    let selection = select_npm_package_version(&packument, options.version.as_deref())?;
    validate_npm_tarball_url(&selection.tarball_url, &registry_url)?;
    let tarball_response = client
        .get(&selection.tarball_url)
        .send()
        .with_context(|| format!("fetch npm tarball `{}`", selection.tarball_url))?
        .error_for_status()
        .with_context(|| format!("npm tarball request failed for `{}`", selection.tarball_url))?;
    if let Some(content_length) = tarball_response.content_length()
        && content_length > MAX_NPM_TARBALL_BYTES as u64
    {
        bail!(
            "npm tarball for `{}` declares {} bytes, above Forge acquire limit {}",
            options.package_name,
            content_length,
            MAX_NPM_TARBALL_BYTES
        );
    }
    let tarball_bytes = tarball_response
        .bytes()
        .with_context(|| format!("read npm tarball `{}`", selection.tarball_url))?
        .to_vec();
    if tarball_bytes.len() > MAX_NPM_TARBALL_BYTES {
        bail!(
            "npm tarball for `{}` is {} bytes, above Forge acquire limit {}",
            options.package_name,
            tarball_bytes.len(),
            MAX_NPM_TARBALL_BYTES
        );
    }

    let tarball_sha512 = Some(sha512_base64(&tarball_bytes));
    let integrity_verified = verify_npm_integrity(&tarball_bytes, selection.integrity.as_deref())?;
    let package_slug = acquisition_package_slug(&options.package_name);
    let cache_dir = project.join(".dx/cache/npm").join(&package_slug);
    let source_dir = cache_dir.join("package");
    let staging_dir = cache_dir.join("package.partial");
    let packument_path = cache_dir.join("packument.json");
    let tarball_path = cache_dir.join("package.tgz");

    fs::create_dir_all(&cache_dir)
        .with_context(|| format!("create Forge npm cache `{}`", cache_dir.display()))?;
    fs::write(&packument_path, serde_json::to_vec_pretty(&packument)?)
        .with_context(|| format!("write `{}`", packument_path.display()))?;
    fs::write(&tarball_path, &tarball_bytes)
        .with_context(|| format!("write `{}`", tarball_path.display()))?;

    remove_dir_if_exists(&staging_dir)?;
    fs::create_dir_all(&staging_dir)
        .with_context(|| format!("create `{}`", staging_dir.display()))?;
    let extraction = match extract_npm_tgz(&tarball_bytes, &staging_dir) {
        Ok(extraction) => extraction,
        Err(error) => {
            let _ = remove_dir_if_exists(&staging_dir);
            return Err(error);
        }
    };
    write_npm_evidence(&staging_dir, &selection, integrity_verified)?;
    ensure_npm_package_json(&staging_dir, &options.package_name, &selection)?;
    let lifecycle = inspect_npm_lifecycle_scripts(&staging_dir)?;
    remove_dir_if_exists(&source_dir)?;
    fs::rename(&staging_dir, &source_dir).with_context(|| {
        format!(
            "promote Forge npm source cache `{}` to `{}`",
            staging_dir.display(),
            source_dir.display()
        )
    })?;

    let import_plan = build_forge_import_plan_report_with_selection(
        project,
        "npm",
        &options.package_name,
        Some(&source_dir),
        &[],
        options.fail_under,
    )?;
    let import_plan = serde_json::to_value(import_plan)?;
    let score = import_plan
        .get("score")
        .and_then(Value::as_u64)
        .and_then(|score| u8::try_from(score).ok())
        .unwrap_or_default();
    let import_plan_passed = import_plan
        .get("passed")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let source_dir_ready = source_dir.is_dir();
    let mut findings = Vec::new();
    if selection.integrity.is_none() {
        findings.push("npm metadata did not include dist.integrity; Forge recorded the tarball hash but cannot claim registry integrity verification.".to_string());
    }
    if !import_plan_passed {
        findings.push("Forge import plan did not pass after acquisition; review the embedded import plan before materialization.".to_string());
    }
    if lifecycle.declared {
        findings.push(format!(
            "npm package declares lifecycle scripts [{}]; Forge acquired source without executing them.",
            lifecycle.names.join(", ")
        ));
    }

    let acquisition_plan =
        acquisition_plan_for_package(ecosystem, &options.package_name, &package_id);
    let receipt_sr_relative_path = acquisition_plan.evidence_receipt_path.replace('\\', "/");
    let receipt_relative_path = receipt_sr_relative_path
        .strip_suffix(".sr")
        .map(|path| format!("{path}.json"))
        .unwrap_or_else(|| format!("{receipt_sr_relative_path}.json"));
    let receipt_path = project.join(&receipt_relative_path);
    let evidence_path = source_dir.join("dx-forge-evidence.sr");

    let mut report = DxForgeAcquireReport {
        schema: FORGE_ACQUIRE_SCHEMA,
        model_version: 1,
        generated_at: Utc::now().to_rfc3339(),
        passed: source_dir_ready && integrity_verified && import_plan_passed,
        score,
        ecosystem: "npm".to_string(),
        package_name: options.package_name.clone(),
        package_id,
        requested_version: options.version.clone(),
        version_resolved_from: selection.version_resolved_from,
        version: selection.version,
        registry_url,
        metadata_url,
        tarball_url: selection.tarball_url,
        tarball_integrity: selection.integrity,
        tarball_sha512,
        integrity_verified,
        cache_dir,
        source_dir,
        source_dir_ready,
        packument_path,
        tarball_path,
        evidence_path,
        receipt_path,
        receipt_sr_path: project.join(&receipt_sr_relative_path),
        receipt_machine_path: project.join(".dx/serializer/placeholder.machine"),
        receipt_json_machine_path: project.join(".dx/serializer/placeholder-json.machine"),
        import_plan_path: import_plan_path(&import_plan),
        import_plan_sr_path: import_plan_path_key(&import_plan, "import_plan_sr_path"),
        import_plan_machine_path: import_plan_path_key(&import_plan, "import_plan_machine_path"),
        import_plan_json_machine_path: import_plan_path_key(
            &import_plan,
            "import_plan_json_machine_path",
        ),
        package_manager_execution_allowed: false,
        package_installs_run: false,
        lifecycle_scripts_declared: lifecycle.declared,
        lifecycle_script_names: lifecycle.names,
        lifecycle_scripts_executed: false,
        lifecycle_script_status: "not-executed".to_string(),
        files_extracted: extraction.files,
        bytes_extracted: extraction.bytes,
        import_plan,
        findings,
        next_commands: vec![
            format!(
                "dx forge import npm {} --plan --source-dir {} --output .dx/forge/import-plans/npm-{}.json",
                options.package_name,
                ".dx/cache/npm/".to_string() + &package_slug + "/package",
                package_slug
            ),
            format!(
                "dx forge import npm {} --write --source-dir {} --from-plan .dx/forge/import-plans/npm-{}.json",
                options.package_name,
                ".dx/cache/npm/".to_string() + &package_slug + "/package",
                package_slug
            ),
        ],
    };
    write_acquire_report_artifacts(
        project,
        &receipt_relative_path,
        &receipt_sr_relative_path,
        &mut report,
    )?;
    Ok(report)
}

pub(super) fn forge_acquire_terminal(report: &DxForgeAcquireReport) -> String {
    let findings = if report.findings.is_empty() {
        "none".to_string()
    } else {
        report.findings.join("; ")
    };
    format!(
        "DX Forge acquire\nEcosystem: {}\nPackage: {}\nVersion: {}\nPassed: {}\nScore: {} / 100\nSource dir ready: {}\nIntegrity verified: {}\nPackage installs run: {}\nLifecycle scripts declared: {}\nLifecycle status: {}\nSource dir: {}\nReceipt: {}\nFindings: {}\n",
        report.ecosystem,
        report.package_name,
        report.version,
        report.passed,
        report.score,
        report.source_dir_ready,
        report.integrity_verified,
        report.package_installs_run,
        report.lifecycle_scripts_declared,
        report.lifecycle_script_status,
        report.source_dir.display(),
        report.receipt_path.display(),
        findings
    )
}

pub(super) fn forge_acquire_markdown(report: &DxForgeAcquireReport) -> String {
    let mut markdown = String::new();
    markdown.push_str("# DX Forge Acquire\n\n");
    markdown.push_str(&format!("- Ecosystem: `{}`\n", report.ecosystem));
    markdown.push_str(&format!("- Package: `{}`\n", report.package_name));
    markdown.push_str(&format!("- Version: `{}`\n", report.version));
    markdown.push_str(&format!("- Passed: `{}`\n", report.passed));
    markdown.push_str(&format!("- Score: `{}` / `100`\n", report.score));
    markdown.push_str(&format!(
        "- Integrity verified: `{}`\n",
        report.integrity_verified
    ));
    markdown.push_str(&format!(
        "- Package installs run: `{}`\n",
        report.package_installs_run
    ));
    markdown.push_str(&format!(
        "- Lifecycle scripts declared: `{}`\n",
        report.lifecycle_scripts_declared
    ));
    if !report.lifecycle_script_names.is_empty() {
        markdown.push_str(&format!(
            "- Lifecycle script names: `{}`\n",
            report.lifecycle_script_names.join(", ")
        ));
    }
    markdown.push_str(&format!(
        "- Lifecycle scripts executed: `{}`\n",
        report.lifecycle_scripts_executed
    ));
    markdown.push_str(&format!(
        "- Source dir: `{}`\n",
        report.source_dir.display()
    ));
    markdown.push_str(&format!(
        "- Receipt: `{}`\n\n",
        report.receipt_path.display()
    ));
    if !report.findings.is_empty() {
        markdown.push_str("## Findings\n\n");
        for finding in &report.findings {
            markdown.push_str(&format!("- {finding}\n"));
        }
        markdown.push('\n');
    }
    markdown.push_str("## Next Commands\n\n");
    for command in &report.next_commands {
        markdown.push_str(&format!("- `{command}`\n"));
    }
    markdown
}

fn write_acquire_report_artifacts(
    project: &Path,
    receipt_relative_path: &str,
    receipt_sr_relative_path: &str,
    report: &mut DxForgeAcquireReport,
) -> anyhow::Result<()> {
    if let Some(parent) = report.receipt_path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create `{}`", parent.display()))?;
    }
    let report_value = serde_json::to_value(&*report)?;
    fs::write(
        &report.receipt_path,
        serde_json::to_vec_pretty(&report_value)?,
    )
    .with_context(|| format!("write `{}`", report.receipt_path.display()))?;
    let sr = write_sr_artifact(
        project,
        receipt_sr_relative_path,
        &[
            ("schema", sr_string(report.schema)),
            ("model_version", sr_number(report.model_version)),
            ("generated_at", sr_string(&report.generated_at)),
            ("passed", sr_bool(report.passed)),
            ("score", sr_number(report.score)),
            ("ecosystem", sr_string(&report.ecosystem)),
            ("package_name", sr_string(&report.package_name)),
            ("package_id", sr_string(&report.package_id)),
            ("package_version", sr_string(&report.version)),
            ("registry_url", sr_string(&report.registry_url)),
            ("metadata_url", sr_string(&report.metadata_url)),
            ("tarball_url", sr_string(&report.tarball_url)),
            (
                "tarball_integrity",
                report
                    .tarball_integrity
                    .as_ref()
                    .map(sr_string)
                    .unwrap_or_else(sr_null),
            ),
            ("integrity_verified", sr_bool(report.integrity_verified)),
            ("source_dir", sr_string(report.source_dir.to_string_lossy())),
            ("source_dir_ready", sr_bool(report.source_dir_ready)),
            (
                "package_manager_execution_allowed",
                sr_bool(report.package_manager_execution_allowed),
            ),
            ("package_installs_run", sr_bool(report.package_installs_run)),
            (
                "lifecycle_scripts_declared",
                sr_bool(report.lifecycle_scripts_declared),
            ),
            (
                "lifecycle_script_names",
                sr_string_array(&report.lifecycle_script_names),
            ),
            (
                "lifecycle_scripts_executed",
                sr_bool(report.lifecycle_scripts_executed),
            ),
            (
                "lifecycle_script_status",
                sr_string(&report.lifecycle_script_status),
            ),
            ("files_extracted", sr_number(report.files_extracted)),
            ("bytes_extracted", sr_number(report.bytes_extracted)),
        ],
    )?;
    report.receipt_sr_path = sr.source;
    report.receipt_machine_path = sr.machine;
    fs::write(&report.receipt_path, serde_json::to_vec_pretty(&*report)?)
        .with_context(|| format!("refresh `{}`", report.receipt_path.display()))?;
    let preliminary_report_value = serde_json::to_value(&*report)?;
    let machine_path = write_json_receipt_machine_alias(
        project,
        &format!(
            "forge-acquire-npm-{}",
            acquisition_package_slug(&report.package_name)
        ),
        receipt_relative_path,
        &preliminary_report_value,
    )?;
    report.receipt_json_machine_path = machine_path;
    fs::write(&report.receipt_path, serde_json::to_vec_pretty(&*report)?)
        .with_context(|| format!("refresh `{}`", report.receipt_path.display()))?;
    let final_report_value = serde_json::to_value(&*report)?;
    let machine_path = write_json_receipt_machine_alias(
        project,
        &format!(
            "forge-acquire-npm-{}",
            acquisition_package_slug(&report.package_name)
        ),
        receipt_relative_path,
        &final_report_value,
    )?;
    report.receipt_json_machine_path = machine_path;
    Ok(())
}

fn select_npm_package_version(
    packument: &Value,
    requested_version: Option<&str>,
) -> anyhow::Result<NpmPackageSelection> {
    let versions = packument
        .get("versions")
        .and_then(Value::as_object)
        .context("npm packument missing versions object")?;
    let (version, version_resolved_from) = if let Some(version) = requested_version {
        (version.to_string(), "requested-version".to_string())
    } else {
        let latest = packument
            .get("dist-tags")
            .and_then(|tags| tags.get("latest"))
            .and_then(Value::as_str)
            .context("npm packument missing dist-tags.latest")?;
        (latest.to_string(), "dist-tags.latest".to_string())
    };
    let selected = versions
        .get(&version)
        .with_context(|| format!("npm packument does not contain version `{version}`"))?;
    let dist = selected
        .get("dist")
        .and_then(Value::as_object)
        .context("npm version metadata missing dist object")?;
    let tarball_url = dist
        .get("tarball")
        .and_then(Value::as_str)
        .context("npm version metadata missing dist.tarball")?
        .to_string();
    let integrity = dist
        .get("integrity")
        .and_then(Value::as_str)
        .map(str::to_string);
    let license = selected
        .get("license")
        .and_then(Value::as_str)
        .or_else(|| packument.get("license").and_then(Value::as_str))
        .map(str::to_string);
    Ok(NpmPackageSelection {
        version,
        version_resolved_from,
        tarball_url,
        integrity,
        license,
    })
}

fn validate_npm_registry_url(registry_url: &str) -> anyhow::Result<String> {
    let registry = Url::parse(registry_url).context("parse npm registry URL")?;
    validate_allowed_npm_url("registry", &registry)?;
    if !registry.username().is_empty() || registry.password().is_some() {
        bail!("npm registry URL must not contain credentials");
    }
    Ok(registry.as_str().trim_end_matches('/').to_string())
}

fn validate_npm_tarball_url(tarball_url: &str, registry_url: &str) -> anyhow::Result<()> {
    let registry = Url::parse(registry_url).context("parse npm registry URL")?;
    let tarball = Url::parse(tarball_url).context("parse npm tarball URL")?;
    validate_allowed_npm_url("tarball", &tarball)?;
    if url_origin(&registry) != url_origin(&tarball) {
        bail!("npm tarball URL origin must match the configured registry origin");
    }
    Ok(())
}

fn validate_allowed_npm_url(label: &str, url: &Url) -> anyhow::Result<()> {
    match url.scheme() {
        "https" => {}
        "http" if url.host_str().is_some_and(is_loopback_host) => {}
        scheme => {
            bail!(
                "npm {label} URL must use https, or http for loopback test registries; got `{scheme}`"
            )
        }
    }
    if url.host_str().is_none() {
        bail!("npm {label} URL must include a host");
    }
    Ok(())
}

fn url_origin(url: &Url) -> (String, String, Option<u16>) {
    (
        url.scheme().to_string(),
        url.host_str().unwrap_or_default().to_ascii_lowercase(),
        url.port_or_known_default(),
    )
}

fn is_loopback_host(host: &str) -> bool {
    matches!(host, "localhost" | "127.0.0.1" | "::1")
}

fn verify_npm_integrity(bytes: &[u8], integrity: Option<&str>) -> anyhow::Result<bool> {
    let Some(integrity) = integrity else {
        return Ok(false);
    };
    let Some((algorithm, expected)) = integrity.split_once('-') else {
        bail!("npm dist.integrity is not an SRI value");
    };
    if algorithm != "sha512" {
        bail!("npm dist.integrity algorithm `{algorithm}` is not supported yet");
    }
    let digest = sha512_base64(bytes);
    if digest != expected {
        bail!("npm dist.integrity did not match downloaded tarball");
    }
    Ok(true)
}

fn sha512_base64(bytes: &[u8]) -> String {
    let digest = sha2::Sha512::digest(bytes);
    base64::engine::general_purpose::STANDARD.encode(digest)
}

fn write_npm_evidence(
    source_dir: &Path,
    selection: &NpmPackageSelection,
    integrity_verified: bool,
) -> anyhow::Result<()> {
    let license = evidence_marker_value(selection.license.as_deref().unwrap_or("unreviewed"));
    let provenance_verified = integrity_verified;
    let tarball_integrity =
        evidence_marker_value(selection.integrity.as_deref().unwrap_or("missing"));
    let evidence = format!(
        "integrity={integrity_verified}\nregistry_integrity_verified={integrity_verified}\nprovenance_verified={provenance_verified}\nlicense={license}\nlicense_reviewed=false\nadvisory=false\nadvisory_reviewed=false\npopularity=false\nsbom=false\nregistry_source=\"npm\"\ntarball_integrity=\"{tarball_integrity}\"\n",
    );
    fs::write(source_dir.join("dx-forge-evidence.sr"), evidence).with_context(|| {
        format!(
            "write `{}`",
            source_dir.join("dx-forge-evidence.sr").display()
        )
    })
}

fn evidence_marker_value(value: &str) -> String {
    value
        .chars()
        .map(|character| match character {
            '\n' | '\r' | '\t' | '=' => ' ',
            _ if character.is_control() => ' ',
            _ => character,
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn ensure_npm_package_json(
    source_dir: &Path,
    package_name: &str,
    selection: &NpmPackageSelection,
) -> anyhow::Result<()> {
    let package_json = source_dir.join("package.json");
    if package_json.is_file() {
        return Ok(());
    }
    let metadata = serde_json::json!({
        "name": package_name,
        "version": selection.version,
        "license": selection.license.as_deref().unwrap_or("UNLICENSED")
    });
    fs::write(&package_json, serde_json::to_vec_pretty(&metadata)?)
        .with_context(|| format!("write `{}`", package_json.display()))
}

#[derive(Debug, Clone)]
struct NpmLifecycleInspection {
    declared: bool,
    names: Vec<String>,
}

fn inspect_npm_lifecycle_scripts(source_dir: &Path) -> anyhow::Result<NpmLifecycleInspection> {
    let package_json = source_dir.join("package.json");
    if !package_json.is_file() {
        return Ok(NpmLifecycleInspection {
            declared: false,
            names: Vec::new(),
        });
    }
    let metadata: Value = serde_json::from_slice(
        &fs::read(&package_json).with_context(|| format!("read `{}`", package_json.display()))?,
    )
    .with_context(|| format!("decode `{}`", package_json.display()))?;
    let Some(scripts) = metadata.get("scripts").and_then(Value::as_object) else {
        return Ok(NpmLifecycleInspection {
            declared: false,
            names: Vec::new(),
        });
    };
    let mut names = scripts
        .keys()
        .filter(|name| is_npm_lifecycle_script(name))
        .cloned()
        .collect::<Vec<_>>();
    names.sort();
    Ok(NpmLifecycleInspection {
        declared: !names.is_empty(),
        names,
    })
}

fn is_npm_lifecycle_script(name: &str) -> bool {
    matches!(
        name,
        "preinstall"
            | "install"
            | "postinstall"
            | "prepublish"
            | "prepare"
            | "prepack"
            | "postpack"
            | "prepublishOnly"
    )
}

fn import_plan_path(import_plan: &Value) -> Option<PathBuf> {
    import_plan_path_key(import_plan, "import_plan_path")
}

fn import_plan_path_key(import_plan: &Value, key: &str) -> Option<PathBuf> {
    import_plan
        .get(key)
        .and_then(Value::as_str)
        .map(PathBuf::from)
}

fn npm_registry_package_path(package_name: &str) -> String {
    package_name
        .bytes()
        .flat_map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                vec![char::from(byte)]
            }
            other => format!("%{other:02X}").chars().collect(),
        })
        .collect()
}

fn remove_dir_if_exists(path: &Path) -> anyhow::Result<()> {
    match fs::remove_dir_all(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).with_context(|| format!("remove `{}`", path.display())),
    }
}
