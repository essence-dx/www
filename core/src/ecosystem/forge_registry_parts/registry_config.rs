/// Local file state for a manifest-backed remote install preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeRemoteManifestFileStatus {
    /// The selected file is not present in the target project yet.
    Missing,
    /// The target project file already matches the manifest hash.
    Matching,
    /// The target project file exists with different content.
    ConflictingLocalFile,
}

/// Per-file preview derived from a real remote package manifest fixture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRemoteManifestFilePlan {
    /// Package logical/source path when available.
    pub logical_path: String,
    /// Front-facing project path where Forge would materialize the file.
    pub materialized_path: String,
    /// Manifest-declared BLAKE3 hash.
    pub manifest_hash: String,
    /// Manifest-declared byte length.
    pub bytes: u64,
    /// Existing project file hash, when the target file exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub existing_hash: Option<String>,
    /// Conflict/match state for this target file.
    pub status: DxForgeRemoteManifestFileStatus,
}

/// Read-only install preview computed from a real manifest fixture without R2 reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRemoteManifestInstallPreview {
    /// Stable schema version for CLI, dx-check, DX-WWW, and future Zed.
    pub schema_version: String,
    /// Manifest fixture path used for this preview.
    pub manifest_path: PathBuf,
    /// Canonical package id.
    pub package_id: String,
    /// Manifest package version.
    pub version: String,
    /// Selected exports used for file filtering.
    pub selected_exports: Vec<String>,
    /// Whether this preview performed network reads.
    pub network_allowed: bool,
    /// Whether this preview performed writes.
    pub write_allowed: bool,
    /// Count of files selected from the manifest.
    pub selected_file_count: u64,
    /// Count of missing target files.
    pub missing_file_count: u64,
    /// Count of already matching target files.
    pub matching_file_count: u64,
    /// Count of target files that would conflict.
    pub conflicting_file_count: u64,
    /// Per-file front-facing plan.
    pub file_plans: Vec<DxForgeRemoteManifestFilePlan>,
    /// Honest limitations for the fixture-backed preview.
    pub warnings: Vec<String>,
}

/// Read-only remote provider plan for CLI, dx-check, DX-WWW, and future Zed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRemoteReadPlan {
    /// Stable schema version for consumers.
    pub schema_version: String,
    /// Provider kind for adapter selection.
    pub provider_kind: DxForgeRemoteProviderKind,
    /// Lifecycle intent this read plan supports.
    pub intent: DxForgeRemoteReadIntent,
    /// Canonical package id.
    pub package_id: String,
    /// Requested package version, or `None` when latest must be resolved.
    pub requested_version: Option<String>,
    /// Selected exports requested by the command.
    pub selected_exports: Vec<String>,
    /// Whether this plan is allowed to perform network reads.
    pub network_allowed: bool,
    /// Whether this plan is allowed to perform remote writes.
    pub write_allowed: bool,
    /// Honest launch boundary.
    pub boundary: String,
    /// Redacted R2 setup status.
    pub setup_status: String,
    /// Redacted R2 missing config labels.
    pub missing_config: Vec<String>,
    /// Planned remote reads.
    pub objects: Vec<DxForgeRemoteReadObject>,
    /// Optional provider object metadata plan derived from a verified manifest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_metadata_plan: Option<DxForgeRemoteObjectMetadataPlan>,
    /// Optional approval-gated object HEAD execution receipt shape.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_head_execution_receipt: Option<DxForgeRemoteObjectHeadExecutionReceipt>,
    /// Optional evaluated object HEAD health for status consumers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_head_health_evaluation: Option<DxForgeRemoteObjectHeadHealthEvaluation>,
    /// Optional fixture-backed install preview derived from a real package manifest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_install_preview: Option<DxForgeRemoteManifestInstallPreview>,
    /// Consumer-facing warnings.
    pub warnings: Vec<String>,
}

/// Registry package metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRegistryPackage {
    /// Canonical package id, for example `shadcn/ui/button`.
    pub package_id: String,
    /// Legacy or short aliases.
    pub aliases: Vec<String>,
    /// Source language.
    pub language: DxForgeLanguage,
    /// Package version.
    pub version: String,
    /// Source registry kind.
    pub source: DxForgeRegistrySource,
    /// Materialized package source kind.
    pub source_kind: DxSourceKind,
    /// License expression.
    pub license: String,
    /// Package description.
    pub description: String,
    /// Structured provenance metadata.
    #[serde(default)]
    pub provenance: DxForgeProvenanceMetadata,
    /// Advisory coverage metadata.
    #[serde(default)]
    pub advisory_review: DxForgeAdvisoryMetadata,
    /// License review metadata.
    #[serde(default)]
    pub license_review: DxForgeLicenseReviewMetadata,
    /// Export surfaces published by this package.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exports: Vec<DxForgeRegistryExport>,
    /// Exports installed when no selector is provided.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub default_exports: Vec<String>,
    /// Whether selected exports may be installed independently.
    #[serde(default)]
    pub allow_selective_imports: bool,
    /// Source files with logical Forge paths.
    pub files: Vec<DxSourceFile>,
    /// BLAKE3 package integrity hash.
    pub integrity_hash: String,
}

/// Named file surface exported by a registry package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRegistryExport {
    /// Export name, for example `client`, `server`, or `ui`.
    pub name: String,
    /// Package file paths included by this export.
    pub files: Vec<String>,
}

/// Registry index stored locally or in R2.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRegistryIndex {
    /// Registry schema version.
    pub version: u32,
    /// Generation timestamp.
    pub generated_at: String,
    /// Known packages.
    pub packages: Vec<DxForgeRegistryPackage>,
    /// Known remotes.
    pub remotes: Vec<DxForgeRegistryRemote>,
}

/// Forge project configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeProjectConfig {
    /// Logical package path to project-facing folder map.
    pub paths: BTreeMap<String, String>,
}

impl Default for DxForgeProjectConfig {
    fn default() -> Self {
        Self {
            paths: BTreeMap::from([
                ("js/auth".to_string(), "auth".to_string()),
                ("js/app".to_string(), "app".to_string()),
                ("js/components".to_string(), "components".to_string()),
                ("js/content".to_string(), "content".to_string()),
                ("js/db".to_string(), "db".to_string()),
                ("js/examples".to_string(), "examples".to_string()),
                ("js/forms".to_string(), "lib/forms".to_string()),
                ("js/icons".to_string(), "components/icons".to_string()),
                ("js/i18n".to_string(), "i18n".to_string()),
                ("js/instant".to_string(), "lib/instant".to_string()),
                ("js/lib".to_string(), "lib".to_string()),
                ("js/migrations".to_string(), "migrations".to_string()),
                ("js/motion".to_string(), "motion".to_string()),
                ("js/openapi".to_string(), "openapi".to_string()),
                ("js/payments".to_string(), "lib/payments".to_string()),
                ("js/query".to_string(), "lib/query".to_string()),
                ("js/root".to_string(), ".".to_string()),
                ("js/scene".to_string(), "lib/scene".to_string()),
                ("js/server".to_string(), "server".to_string()),
                ("js/state".to_string(), "lib/forge/state".to_string()),
                ("js/supabase".to_string(), "lib/supabase".to_string()),
                ("js/three".to_string(), "three".to_string()),
                ("js/styles".to_string(), "styles".to_string()),
                ("js/ui".to_string(), "components/ui".to_string()),
                ("js/validation".to_string(), "lib/validation".to_string()),
                ("js/wasm".to_string(), "wasm".to_string()),
            ]),
        }
    }
}

impl DxForgeProjectConfig {
    /// Load Forge project config from root `dx`, with `dx.config.toml` as legacy fallback.
    pub fn load(project: impl AsRef<Path>) -> Result<Self> {
        let project = project.as_ref();
        let dx_path = project.join("dx");
        if dx_path.exists() {
            let text = fs::read_to_string(&dx_path)
                .with_context(|| format!("read `{}`", dx_path.display()))?;
            return Self::load_from_dx_llm(&text)
                .with_context(|| format!("parse `{}`", dx_path.display()));
        }

        let config_path = project.join("dx.config.toml");
        if !config_path.exists() {
            return Ok(Self::default());
        }

        let text = fs::read_to_string(&config_path)
            .with_context(|| format!("read `{}`", config_path.display()))?;
        let value: toml::Value =
            toml::from_str(&text).with_context(|| format!("parse `{}`", config_path.display()))?;

        let mut config = Self::default();
        if let Some(paths) = value
            .get("forge")
            .and_then(|forge| forge.get("paths"))
            .and_then(toml::Value::as_table)
        {
            for (logical, value) in paths {
                let Some(target) = value.as_str() else {
                    bail!("forge.paths.{logical} must be a string");
                };
                validate_project_relative_path(target)
                    .with_context(|| format!("validate forge.paths.{logical}"))?;
                config.paths.insert(logical.to_string(), target.to_string());
            }
        }

        for target in config.paths.values() {
            validate_project_relative_path(target)?;
        }

        Ok(config)
    }

    fn load_from_dx_llm(text: &str) -> Result<Self> {
        let mut config = Self::default();
        for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
            let Some(rest) = line.strip_prefix("forge.paths.") else {
                continue;
            };
            let Some((logical, target)) = rest.split_once('=') else {
                continue;
            };
            let logical = logical.trim().replace(['_', '.'], "/");
            let target = target.trim().trim_matches('"');
            validate_project_relative_path(target)
                .with_context(|| format!("validate forge.paths.{logical}"))?;
            config.paths.insert(logical, target.to_string());
        }

        for target in config.paths.values() {
            validate_project_relative_path(target)?;
        }

        Ok(config)
    }

    /// Map a logical registry path to a project-facing path.
    pub fn materialize_path(&self, logical_path: &str) -> Result<String> {
        let normalized = logical_path.replace('\\', "/");
        let mut best: Option<(&str, &str)> = None;
        for (prefix, target) in &self.paths {
            if normalized == *prefix || normalized.starts_with(&format!("{prefix}/")) {
                if best.is_none_or(|(current, _)| prefix.len() > current.len()) {
                    best = Some((prefix.as_str(), target.as_str()));
                }
            }
        }

        let Some((prefix, target)) = best else {
            bail!("no Forge path mapping configured for `{logical_path}`");
        };

        let suffix = normalized
            .trim_start_matches(prefix)
            .trim_start_matches('/');
        let materialized = if suffix.is_empty() {
            target.to_string()
        } else if target == "." {
            suffix.to_string()
        } else {
            format!("{target}/{suffix}")
        };
        validate_project_relative_path(&materialized)?;
        Ok(materialized)
    }
}

/// Redacted Cloudflare R2 config status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeR2Status {
    /// Whether all required values are present.
    pub configured: bool,
    /// Redacted setup state for CLI/TUI/Zed consumers.
    pub setup_status: String,
    /// Redacted labels for required config that is still missing.
    pub missing_config: Vec<String>,
    /// Whether account id is set.
    pub account_id_set: bool,
    /// Whether an access key id is set without exposing it.
    pub access_key_id_set: bool,
    /// Whether a secret access key is set without exposing it.
    pub secret_access_key_set: bool,
    /// Whether a bucket name is set without exposing it.
    #[serde(default)]
    pub bucket_set: bool,
    /// Whether an endpoint is set or can be derived without exposing it.
    #[serde(default)]
    pub endpoint_set: bool,
    /// Whether a public base URL is set without exposing it.
    #[serde(default)]
    pub public_base_url_set: bool,
    /// Bucket name if set. Kept in memory for planning, but never serialized.
    #[serde(default, skip_serializing)]
    pub bucket: Option<String>,
    /// S3-compatible endpoint if account id is set. Kept in memory for planning, but never serialized.
    #[serde(default, skip_serializing)]
    pub endpoint: Option<String>,
    /// Public base URL if set. Kept in memory for planning, but never serialized.
    #[serde(default, skip_serializing)]
    pub public_base_url: Option<String>,
    /// Registry prefix.
    pub prefix: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxForgeR2Config {
    account_id: String,
    access_key_id: String,
    secret_access_key: String,
    bucket: String,
    endpoint: Option<String>,
    public_base_url: Option<String>,
    prefix: String,
}

impl DxForgeR2Config {
    /// Load R2 config from process env.
    pub fn from_env() -> Option<Self> {
        Self::from_lookup(|key| std::env::var(key).ok())
    }

    /// Load R2 status from process env without exposing secrets.
    pub fn status_from_env() -> DxForgeR2Status {
        Self::status_from_lookup(|key| std::env::var(key).ok())
    }

    /// Load R2 config from an arbitrary lookup function.
    pub fn from_lookup(mut get: impl FnMut(&str) -> Option<String>) -> Option<Self> {
        let account_id = first_lookup_value(
            &mut get,
            &["CLOUDFLARE_R2_ACCOUNT_ID", "CLOUDFLARE_ACCOUNT_ID"],
        )
        .unwrap_or_default();
        let access_key_id = first_lookup_value(
            &mut get,
            &["CLOUDFLARE_R2_ACCESS_KEY_ID", "AWS_ACCESS_KEY_ID"],
        )?;
        let secret_access_key = first_lookup_value(
            &mut get,
            &["CLOUDFLARE_R2_SECRET_ACCESS_KEY", "AWS_SECRET_ACCESS_KEY"],
        )?;
        let bucket = first_lookup_value(&mut get, &["CLOUDFLARE_R2_BUCKET", "DX_FORGE_R2_BUCKET"])?;
        let endpoint = first_lookup_value(&mut get, &["AWS_ENDPOINT_URL"])
            .map(|value| value.trim_end_matches('/').to_string());
        let public_base_url = first_lookup_value(
            &mut get,
            &[
                "CLOUDFLARE_R2_PUBLIC_BASE_URL",
                "DX_FORGE_R2_PUBLIC_BASE_URL",
            ],
        )
        .map(|value| value.trim_end_matches('/').to_string());
        let prefix = first_lookup_value(&mut get, &["DX_FORGE_R2_PREFIX"])
            .map(|value| value.trim_matches('/').to_string())
            .unwrap_or_else(|| DEFAULT_R2_PREFIX.to_string());

        if account_id.is_empty() && endpoint.is_none() {
            return None;
        }

        Some(Self {
            account_id,
            access_key_id,
            secret_access_key,
            bucket,
            endpoint,
            public_base_url,
            prefix,
        })
    }

    /// Return redacted status from an arbitrary lookup function.
    pub fn status_from_lookup(mut get: impl FnMut(&str) -> Option<String>) -> DxForgeR2Status {
        let account_id = first_lookup_value(
            &mut get,
            &["CLOUDFLARE_R2_ACCOUNT_ID", "CLOUDFLARE_ACCOUNT_ID"],
        )
        .unwrap_or_default();
        let access_key_id_set = first_lookup_value(
            &mut get,
            &["CLOUDFLARE_R2_ACCESS_KEY_ID", "AWS_ACCESS_KEY_ID"],
        )
        .is_some();
        let secret_access_key_set = first_lookup_value(
            &mut get,
            &["CLOUDFLARE_R2_SECRET_ACCESS_KEY", "AWS_SECRET_ACCESS_KEY"],
        )
        .is_some();
        let bucket = first_lookup_value(&mut get, &["CLOUDFLARE_R2_BUCKET", "DX_FORGE_R2_BUCKET"]);
        let endpoint_override = first_lookup_value(&mut get, &["AWS_ENDPOINT_URL"])
            .map(|value| value.trim_end_matches('/').to_string());
        let public_base_url = first_lookup_value(
            &mut get,
            &[
                "CLOUDFLARE_R2_PUBLIC_BASE_URL",
                "DX_FORGE_R2_PUBLIC_BASE_URL",
            ],
        )
        .map(|value| value.trim_end_matches('/').to_string());
        let prefix = first_lookup_value(&mut get, &["DX_FORGE_R2_PREFIX"])
            .map(|value| value.trim_matches('/').to_string())
            .unwrap_or_else(|| DEFAULT_R2_PREFIX.to_string());
        let endpoint = endpoint_override.or_else(|| {
            (!account_id.trim().is_empty())
                .then(|| format!("https://{}.r2.cloudflarestorage.com", account_id.trim()))
        });
        let configured = endpoint.is_some()
            && access_key_id_set
            && secret_access_key_set
            && bucket.as_deref().is_some_and(|value| !value.is_empty());
        let has_any_config = !account_id.trim().is_empty()
            || access_key_id_set
            || secret_access_key_set
            || bucket.as_deref().is_some_and(|value| !value.is_empty())
            || endpoint.is_some()
            || public_base_url.is_some();
        let setup_status = forge_r2_setup_status(configured, has_any_config);
        let missing_config =
            forge_r2_missing_config(&endpoint, access_key_id_set, secret_access_key_set, &bucket);

        DxForgeR2Status {
            configured,
            setup_status,
            missing_config,
            account_id_set: !account_id.trim().is_empty(),
            access_key_id_set,
            secret_access_key_set,
            bucket_set: bucket.as_deref().is_some_and(|value| !value.is_empty()),
            endpoint_set: endpoint.is_some(),
            public_base_url_set: public_base_url.is_some(),
            bucket,
            endpoint,
            public_base_url,
            prefix,
        }
    }

    fn endpoint(&self) -> String {
        self.endpoint
            .clone()
            .unwrap_or_else(|| format!("https://{}.r2.cloudflarestorage.com", self.account_id))
    }

    fn object_url(&self, key: &str) -> String {
        if let Some(base) = &self.public_base_url {
            format!(
                "{}/{}",
                base,
                key.split('/')
                    .map(url_escape_segment)
                    .collect::<Vec<_>>()
                    .join("/")
            )
        } else {
            format!("r2://{}/{}", self.bucket, key)
        }
    }

    pub(crate) fn store(&self) -> Result<object_store::aws::AmazonS3> {
        AmazonS3Builder::new()
            .with_bucket_name(&self.bucket)
            .with_endpoint(self.endpoint())
            .with_access_key_id(&self.access_key_id)
            .with_secret_access_key(&self.secret_access_key)
            .with_region("auto")
            .build()
            .context("create R2 object store")
    }
}

fn forge_r2_setup_status(configured: bool, has_any_config: bool) -> String {
    if configured {
        "configured".to_string()
    } else if has_any_config {
        "partial-config".to_string()
    } else {
        "missing-config".to_string()
    }
}

fn forge_r2_missing_config(
    endpoint: &Option<String>,
    access_key_id_set: bool,
    secret_access_key_set: bool,
    bucket: &Option<String>,
) -> Vec<String> {
    let mut missing = Vec::new();
    if endpoint.is_none() {
        missing.push("account_id_or_endpoint".to_string());
    }
    if !access_key_id_set {
        missing.push("access_key_id".to_string());
    }
    if !secret_access_key_set {
        missing.push("secret_access_key".to_string());
    }
    if bucket.as_deref().is_none_or(|value| value.is_empty()) {
        missing.push("bucket".to_string());
    }
    missing
}

fn first_lookup_value(
    get: &mut impl FnMut(&str) -> Option<String>,
    keys: &[&str],
) -> Option<String> {
    keys.iter()
        .find_map(|key| get(key))
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

/// Result for registry init/publish/pull/status commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRegistryOperationReport {
    /// Operation name.
    pub action: String,
    /// Package id when package-specific.
    pub package_id: Option<String>,
    /// Version when package-specific.
    pub version: Option<String>,
    /// Remote name or local path.
    pub remote: String,
    /// Whether the command avoided writes.
    pub dry_run: bool,
    /// Redacted R2 status if relevant.
    pub r2_status: Option<DxForgeR2Status>,
    /// Object or file paths planned, written, or pulled.
    pub objects: Vec<String>,
}

/// Integrity verification summary for a Forge registry package manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRegistryIntegrityReport {
    /// Canonical package id.
    pub package_id: String,
    /// Package version.
    pub version: String,
    /// Number of files declared by the manifest.
    pub file_count: u64,
    /// Number of file contents verified against the manifest.
    pub verified_files: u64,
    /// Verified package integrity hash.
    pub integrity_hash: String,
}
