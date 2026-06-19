use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

/// External ecosystem Forge can review before source materialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeImportEcosystem {
    /// npm package tarballs and metadata.
    Npm,
    /// PyPI wheels, sdists, and metadata.
    Pip,
    /// crates.io crate archives and metadata.
    Cargo,
    /// Go modules and package directories.
    Go,
    /// JSR packages for Deno-first JavaScript and TypeScript modules.
    Jsr,
    /// Dart and Flutter packages from pub.dev.
    Pub,
    /// Java and Kotlin packages from Maven-style registries.
    Maven,
    /// .NET packages from NuGet.
    Nuget,
    /// PHP packages from Packagist/Composer.
    Composer,
    /// Ruby packages from RubyGems.
    Gem,
    /// Swift packages from Swift Package Manager registries or source snapshots.
    Swift,
    /// Elixir/Erlang packages from Hex.pm source snapshots.
    Hex,
    /// R packages from CRAN source snapshots.
    Cran,
}

/// Stable order for currently modeled non-executing import surfaces.
pub const DX_FORGE_IMPORT_ECOSYSTEMS: &[DxForgeImportEcosystem] = &[
    DxForgeImportEcosystem::Npm,
    DxForgeImportEcosystem::Pip,
    DxForgeImportEcosystem::Cargo,
    DxForgeImportEcosystem::Go,
    DxForgeImportEcosystem::Jsr,
    DxForgeImportEcosystem::Pub,
    DxForgeImportEcosystem::Maven,
    DxForgeImportEcosystem::Nuget,
    DxForgeImportEcosystem::Composer,
    DxForgeImportEcosystem::Gem,
    DxForgeImportEcosystem::Swift,
    DxForgeImportEcosystem::Hex,
    DxForgeImportEcosystem::Cran,
];

/// CLI help fragment for currently modeled import surfaces.
pub const DX_FORGE_IMPORT_ECOSYSTEMS_HELP: &str =
    "npm|pip|cargo|go|jsr|pub|maven|nuget|composer|gem|swift|hex|cran";

/// Common registry aliases accepted by the CLI while preserving canonical receipt segments.
pub const DX_FORGE_IMPORT_ECOSYSTEM_ALIASES_HELP: &str = "npmjs->npm, pypi|python->pip, crates.io|rust->cargo, golang|gomod->go, deno|jsr.io->jsr, pub.dev|dart|flutter->pub, maven-central|gradle->maven, dotnet->nuget, packagist|php->composer, rubygems|ruby->gem, swiftpm|spm->swift, hex.pm|elixir->hex, rstats|r-cran->cran";

impl DxForgeImportEcosystem {
    /// Stable receipt segment for paths and machine artifacts.
    pub fn as_segment(self) -> &'static str {
        match self {
            Self::Npm => "npm",
            Self::Pip => "pip",
            Self::Cargo => "cargo",
            Self::Go => "go",
            Self::Jsr => "jsr",
            Self::Pub => "pub",
            Self::Maven => "maven",
            Self::Nuget => "nuget",
            Self::Composer => "composer",
            Self::Gem => "gem",
            Self::Swift => "swift",
            Self::Hex => "hex",
            Self::Cran => "cran",
        }
    }

    /// Parse the stable CLI/source segment for one modeled ecosystem.
    pub fn from_segment(segment: &str) -> Option<Self> {
        match segment.trim().to_ascii_lowercase().as_str() {
            "npm" | "npmjs" | "npmjs.com" => Some(Self::Npm),
            "pip" | "pypi" | "pypi.org" | "python" => Some(Self::Pip),
            "cargo" | "crates" | "crates.io" | "rust" => Some(Self::Cargo),
            "go" | "golang" | "gomod" | "go-mod" | "go-module" => Some(Self::Go),
            "jsr" | "jsr.io" | "deno" => Some(Self::Jsr),
            "pub" | "pub.dev" | "dart" | "flutter" => Some(Self::Pub),
            "maven" | "maven-central" | "maven_central" | "gradle" | "java" | "kotlin" => {
                Some(Self::Maven)
            }
            "nuget" | "dotnet" | ".net" | "csharp" => Some(Self::Nuget),
            "composer" | "packagist" | "php" => Some(Self::Composer),
            "gem" | "rubygems" | "ruby" | "bundler" => Some(Self::Gem),
            "swift" | "swiftpm" | "spm" => Some(Self::Swift),
            "hex" | "hexpm" | "hex.pm" | "elixir" | "erlang" => Some(Self::Hex),
            "cran" | "r" | "rstats" | "r-cran" => Some(Self::Cran),
            _ => None,
        }
    }

    /// CLI help fragment for currently modeled import surfaces.
    pub fn supported_segments_help() -> &'static str {
        DX_FORGE_IMPORT_ECOSYSTEMS_HELP
    }

    /// CLI help fragment for accepted ecosystem aliases.
    pub fn supported_aliases_help() -> &'static str {
        DX_FORGE_IMPORT_ECOSYSTEM_ALIASES_HELP
    }

    /// Package-manager commands Forge import must not run implicitly.
    pub fn blocked_commands(self) -> &'static [&'static str] {
        match self {
            Self::Npm => &["npm install", "pnpm install", "yarn install", "bun install"],
            Self::Pip => &["pip install", "uv pip install", "python setup.py"],
            Self::Cargo => &["cargo add", "cargo build", "cargo install"],
            Self::Go => &["go get", "go install", "go generate"],
            Self::Jsr => &["deno add", "deno install", "deno task", "jsr add"],
            Self::Pub => &["dart pub get", "flutter pub get", "dart run build_runner"],
            Self::Maven => &[
                "mvn install",
                "mvn package",
                "gradle build",
                "gradle publish",
            ],
            Self::Nuget => &["dotnet restore", "dotnet build", "nuget install"],
            Self::Composer => &["composer install", "composer update", "php artisan"],
            Self::Gem => &["gem install", "bundle install", "rake"],
            Self::Swift => &["swift package resolve", "swift build", "swift test"],
            Self::Hex => &[
                "mix deps.get",
                "mix compile",
                "mix archive.install",
                "rebar3 compile",
            ],
            Self::Cran => &[
                "R CMD INSTALL",
                "install.packages",
                "pak::pkg_install",
                "renv::restore",
            ],
        }
    }
}

/// Validate a requested external package name before it becomes a Forge receipt id.
pub fn validate_import_package_name(
    ecosystem: DxForgeImportEcosystem,
    package_name: &str,
) -> Result<()> {
    let package_name = package_name.trim();
    if package_name.is_empty() {
        bail!("Forge import package name cannot be empty");
    }
    if package_name.contains('\\')
        || package_name.contains('\0')
        || package_name.contains("..")
        || package_name.contains("://")
        || package_name.starts_with('-')
        || package_name.chars().any(char::is_whitespace)
        || package_name
            .chars()
            .any(|ch| matches!(ch, ';' | '&' | '|' | '<' | '>' | '`' | '$'))
    {
        bail!(
            "Forge import package name `{package_name}` is unsafe for {} receipts",
            ecosystem.as_segment()
        );
    }

    let valid = match ecosystem {
        DxForgeImportEcosystem::Npm | DxForgeImportEcosystem::Jsr => {
            validate_javascript_package_name(package_name)
        }
        DxForgeImportEcosystem::Pip => validate_python_package_name(package_name),
        DxForgeImportEcosystem::Cargo => validate_simple_package_name(package_name, "_-"),
        DxForgeImportEcosystem::Go | DxForgeImportEcosystem::Swift => {
            validate_path_package_name(package_name, ".-_")
        }
        DxForgeImportEcosystem::Pub | DxForgeImportEcosystem::Hex => {
            validate_lower_identifier_package_name(package_name)
        }
        DxForgeImportEcosystem::Maven => validate_maven_package_name(package_name),
        DxForgeImportEcosystem::Nuget => validate_simple_package_name(package_name, ".-_"),
        DxForgeImportEcosystem::Composer => {
            validate_two_part_path_package_name(package_name, "_-.")
        }
        DxForgeImportEcosystem::Gem => validate_simple_package_name(package_name, "_-."),
        DxForgeImportEcosystem::Cran => validate_cran_package_name(package_name),
    };
    if !valid {
        bail!(
            "Forge import package name `{package_name}` does not match the {} package identity shape",
            ecosystem.as_segment()
        );
    }
    Ok(())
}

fn validate_javascript_package_name(package_name: &str) -> bool {
    if let Some(rest) = package_name.strip_prefix('@') {
        let Some((scope, name)) = rest.split_once('/') else {
            return false;
        };
        return validate_simple_package_name(scope, "-_.")
            && validate_simple_package_name(name, "-_.")
            && !name.starts_with('.');
    }
    !package_name.contains('/') && validate_simple_package_name(package_name, "-_.")
}

fn validate_python_package_name(package_name: &str) -> bool {
    validate_simple_package_name(package_name, "._-")
        && package_name
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_alphanumeric())
        && package_name
            .chars()
            .last()
            .is_some_and(|ch| ch.is_ascii_alphanumeric())
}

fn validate_maven_package_name(package_name: &str) -> bool {
    let separator = if package_name.contains(':') { ':' } else { '/' };
    let parts = package_name.split(separator).collect::<Vec<_>>();
    parts.len() == 2
        && validate_dotted_package_path(parts[0])
        && validate_simple_package_name(parts[1], "._-")
}

fn validate_two_part_path_package_name(package_name: &str, extra: &str) -> bool {
    let parts = package_name.split('/').collect::<Vec<_>>();
    parts.len() == 2
        && parts
            .iter()
            .all(|part| validate_simple_package_name(part, extra))
}

fn validate_path_package_name(package_name: &str, extra: &str) -> bool {
    !package_name.starts_with('/')
        && !package_name.ends_with('/')
        && package_name
            .split('/')
            .all(|part| validate_simple_package_name(part, extra))
}

fn validate_dotted_package_path(package_name: &str) -> bool {
    package_name
        .split('.')
        .all(|part| validate_simple_package_name(part, "_-"))
}

fn validate_lower_identifier_package_name(package_name: &str) -> bool {
    let mut chars = package_name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first.is_ascii_lowercase() || first == '_')
        && chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
}

fn validate_cran_package_name(package_name: &str) -> bool {
    let mut chars = package_name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    first.is_ascii_alphabetic() && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '.')
}

fn validate_simple_package_name(package_name: &str, extra: &str) -> bool {
    !package_name.is_empty()
        && package_name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || extra.chars().any(|allowed| allowed == ch))
}

/// Review phase in the Forge import firewall for modeled external ecosystems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeImportPhase {
    /// Build a policy plan without network or filesystem writes.
    Plan,
    /// Acquire metadata or archives without executing package code.
    Acquire,
    /// Copy source into a non-importable review area.
    Quarantine,
    /// Inspect usage, metadata, entrypoints, and risk signals.
    Analyze,
    /// Select only the reviewed files/symbols that should become source-owned.
    Slice,
    /// Write accepted source-owned files with receipts.
    Materialize,
    /// Rewrite app imports only after accepted receipts exist.
    Rewrite,
}

impl DxForgeImportPhase {
    /// Whether this phase may create app-importable source.
    pub fn may_materialize_source(self) -> bool {
        matches!(self, Self::Materialize | Self::Rewrite)
    }
}

/// The result category for a reviewed package slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeImportSliceKind {
    /// Entire package source snapshot after review.
    FullPackage,
    /// Reviewed source subset copied from an external package.
    SourceSlice,
    /// A named export or symbol-level source slice.
    SymbolSlice,
    /// Small compatibility adapter written as reviewed source.
    Adapter,
    /// Direct source subset copied after review.
    SourceCopy,
    /// Asset-only subset such as icons, media, or static data.
    AssetSlice,
    /// WASM-compatible source or compiled module boundary.
    WasmSlice,
    /// CLI, formatter, code generator, or build-side tool boundary.
    ToolBridge,
    /// Checksummed binary or prebuilt artifact boundary.
    BinaryBridge,
    /// Runtime, native, hosted, or package-manager boundary.
    RuntimeBridge,
    /// Metadata is recorded, but no importable source is written.
    MetadataOnly,
    /// Policy blocked the import.
    Blocked,
}

/// Supply-chain or compatibility signal collected during import review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeImportRiskFlag {
    /// Package lifecycle hooks are present or required.
    LifecycleScript,
    /// Package manager install hooks are required.
    InstallHook,
    /// Native binaries or compiled extensions are present.
    NativeBinary,
    /// Code appears minified, obfuscated, or intentionally hard to review.
    ObfuscatedBlob,
    /// Dynamic code execution such as eval, Function, or setup.py execution.
    DynamicExecution,
    /// Dependency graph is too large for automatic source slicing.
    HugeDependencyGraph,
    /// Path is not safe to place in quarantine or materialized source.
    UnsafePath,
    /// Symlink or filesystem indirection requires manual review.
    Symlink,
    /// Candidate path can escape the intended project or quarantine root.
    ProjectEscape,
    /// License metadata is missing or unreviewed.
    MissingLicense,
    /// Integrity metadata or file hashes are missing.
    MissingIntegrity,
    /// Advisory coverage is missing or only fixture-backed.
    AdvisoryCoverageMissing,
    /// Browser/server/runtime target does not match the route surface.
    RuntimeMismatch,
    /// Dynamic import/require prevents precise source ownership.
    DynamicImport,
    /// Side-effect import cannot be proven safe automatically.
    SideEffectImport,
    /// Plaintext secret or credential-like value appears in metadata.
    PlaintextSecret,
}

impl DxForgeImportRiskFlag {
    /// Stable receipt reason code.
    pub fn as_reason_code(self) -> &'static str {
        match self {
            Self::LifecycleScript => "lifecycle-script",
            Self::InstallHook => "install-hook",
            Self::NativeBinary => "native-binary",
            Self::ObfuscatedBlob => "obfuscated-blob",
            Self::DynamicExecution => "dynamic-execution",
            Self::HugeDependencyGraph => "huge-dependency-graph",
            Self::UnsafePath => "unsafe-path",
            Self::Symlink => "symlink",
            Self::ProjectEscape => "project-escape",
            Self::MissingLicense => "missing-license",
            Self::MissingIntegrity => "missing-integrity",
            Self::AdvisoryCoverageMissing => "advisory-coverage-missing",
            Self::RuntimeMismatch => "runtime-mismatch",
            Self::DynamicImport => "dynamic-import",
            Self::SideEffectImport => "side-effect-import",
            Self::PlaintextSecret => "plaintext-secret",
        }
    }

    /// Whether this finding blocks automatic materialization.
    pub fn blocks_materialization(self) -> bool {
        matches!(
            self,
            Self::LifecycleScript
                | Self::InstallHook
                | Self::NativeBinary
                | Self::ObfuscatedBlob
                | Self::DynamicExecution
                | Self::HugeDependencyGraph
                | Self::UnsafePath
                | Self::Symlink
                | Self::ProjectEscape
                | Self::MissingIntegrity
                | Self::RuntimeMismatch
                | Self::DynamicImport
                | Self::SideEffectImport
                | Self::PlaintextSecret
        )
    }
}

/// Traffic-light decision for a single import gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeImportDecision {
    /// Safe to continue automatically inside the current phase.
    Accept,
    /// Keep quarantined until a reviewer accepts the risk.
    ManualReview,
    /// Do not materialize or rewrite.
    Block,
}

/// Specific import policy gate recorded in receipts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeImportPolicyGate {
    /// Package manager commands were not executed.
    NoPackageInstall,
    /// Lifecycle/setup hooks were not executed.
    NoLifecycleExecution,
    /// Candidate path is relative, slash-normalized, and project-safe.
    SafePath,
    /// Quarantine source is not importable app code.
    NonImportableQuarantine,
    /// Integrity/hash evidence is present.
    IntegrityPresent,
    /// Declared license metadata is present.
    LicenseDeclared,
    /// Advisory coverage state is declared honestly.
    AdvisoryDeclared,
    /// Slice scope is small and explicit.
    SliceScope,
    /// Materialization receipts are required.
    ReceiptRequired,
    /// Import rewrites require accepted materialization receipts.
    RewriteRequiresReceipt,
}

/// A specific policy decision recorded in an import receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeImportPolicyDecision {
    /// Phase where this decision was made.
    pub phase: DxForgeImportPhase,
    /// Gate evaluated by this decision.
    pub gate: DxForgeImportPolicyGate,
    /// Decision result.
    pub decision: DxForgeImportDecision,
    /// Stable reason code.
    pub code: String,
    /// Risk flags connected to the decision.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub risk_flags: Vec<DxForgeImportRiskFlag>,
    /// Optional evidence path referenced by the decision.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_path: Option<String>,
    /// Human-readable detail for receipts and reports.
    pub detail: String,
    /// Concrete remediation when the gate does not pass cleanly.
    pub remediation: String,
}

/// Plan-only import surface for one external package ecosystem.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeImportPlanSurface {
    /// Ecosystem represented by this surface.
    pub ecosystem: DxForgeImportEcosystem,
    /// Metadata Forge may inspect without executing package code.
    pub metadata_inputs: Vec<String>,
    /// Archives or source bundles Forge may hash or quarantine.
    pub artifact_inputs: Vec<String>,
    /// Conditions that force manual review before slicing.
    pub manual_review_triggers: Vec<String>,
    /// Package-manager commands that remain forbidden in this plan surface.
    pub forbidden_commands: Vec<String>,
    /// Whether the surface performs live fetching by default.
    pub live_fetching_enabled: bool,
    /// Whether the surface executes package-manager or package code.
    pub package_manager_execution: bool,
    /// Whether materialization requires an accepted import receipt.
    pub accepted_import_receipt_required: bool,
    /// Unsupported direct `dx add` form until an accepted import receipt exists.
    pub unsupported_dx_add_form: String,
}

impl DxForgeImportPlanSurface {
    /// Create a non-executing, receipt-gated plan surface.
    pub fn non_executing(
        ecosystem: DxForgeImportEcosystem,
        metadata_inputs: &[&str],
        artifact_inputs: &[&str],
        manual_review_triggers: &[&str],
        unsupported_dx_add_form: &str,
    ) -> Self {
        Self {
            ecosystem,
            metadata_inputs: metadata_inputs
                .iter()
                .map(|input| (*input).to_string())
                .collect(),
            artifact_inputs: artifact_inputs
                .iter()
                .map(|input| (*input).to_string())
                .collect(),
            manual_review_triggers: manual_review_triggers
                .iter()
                .map(|trigger| (*trigger).to_string())
                .collect(),
            forbidden_commands: ecosystem
                .blocked_commands()
                .iter()
                .map(|command| (*command).to_string())
                .collect(),
            live_fetching_enabled: false,
            package_manager_execution: false,
            accepted_import_receipt_required: true,
            unsupported_dx_add_form: unsupported_dx_add_form.to_string(),
        }
    }
}
