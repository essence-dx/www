use serde::{Deserialize, Serialize};

use super::types::{DxForgeImportEcosystem, DxForgeImportRiskFlag};

/// Manual-review trigger that can keep an import in quarantine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeImportManualReviewTrigger {
    /// Ecosystem where the trigger applies.
    pub ecosystem: DxForgeImportEcosystem,
    /// Risk flag raised by the trigger.
    pub risk: DxForgeImportRiskFlag,
    /// Stable trigger name used in receipts.
    pub name: String,
    /// Human-readable trigger description.
    pub detail: String,
}

/// Manual-review triggers for one ecosystem.
pub fn manual_review_triggers_for_ecosystem(
    ecosystem: DxForgeImportEcosystem,
) -> Vec<DxForgeImportManualReviewTrigger> {
    let shared = [
        (
            DxForgeImportRiskFlag::MissingLicense,
            "missing-license",
            "Package license metadata is missing or has not been reviewed.",
        ),
        (
            DxForgeImportRiskFlag::MissingIntegrity,
            "missing-integrity",
            "Package artifact or file integrity is missing.",
        ),
        (
            DxForgeImportRiskFlag::PlaintextSecret,
            "plaintext-secret",
            "Metadata or source contains credential-like plaintext.",
        ),
    ];

    let ecosystem_specific: Vec<(DxForgeImportRiskFlag, &'static str, &'static str)> =
        match ecosystem {
            DxForgeImportEcosystem::Npm => vec![
                (
                    DxForgeImportRiskFlag::LifecycleScript,
                    "npm-lifecycle-script",
                    "package.json contains preinstall, install, postinstall, prepare, or prepublish.",
                ),
                (
                    DxForgeImportRiskFlag::DynamicImport,
                    "npm-dynamic-require",
                    "Source uses dynamic require/import that prevents precise slicing.",
                ),
                (
                    DxForgeImportRiskFlag::NativeBinary,
                    "npm-native-addon",
                    "Package contains native addons or prebuilt binaries.",
                ),
            ],
            DxForgeImportEcosystem::Pip => vec![
                (
                    DxForgeImportRiskFlag::DynamicExecution,
                    "pip-setup-execution",
                    "Package requires setup.py or build backend execution.",
                ),
                (
                    DxForgeImportRiskFlag::NativeBinary,
                    "pip-native-extension",
                    "Wheel or source distribution contains native extensions.",
                ),
            ],
            DxForgeImportEcosystem::Cargo => vec![
                (
                    DxForgeImportRiskFlag::InstallHook,
                    "cargo-build-script",
                    "Crate contains build.rs or native link requirements.",
                ),
                (
                    DxForgeImportRiskFlag::DynamicExecution,
                    "cargo-proc-macro",
                    "Crate requires proc macro execution before use.",
                ),
            ],
            DxForgeImportEcosystem::Go => vec![
                (
                    DxForgeImportRiskFlag::InstallHook,
                    "go-generate",
                    "Module requires go:generate or command execution.",
                ),
                (
                    DxForgeImportRiskFlag::NativeBinary,
                    "go-cgo",
                    "Module uses cgo or platform-native bindings.",
                ),
            ],
            DxForgeImportEcosystem::Jsr => vec![
                (
                    DxForgeImportRiskFlag::InstallHook,
                    "jsr-deno-task",
                    "Package requires deno task, generator, or install-time command execution.",
                ),
                (
                    DxForgeImportRiskFlag::DynamicImport,
                    "jsr-dynamic-import",
                    "Source uses dynamic import boundaries that prevent precise source slicing.",
                ),
                (
                    DxForgeImportRiskFlag::RuntimeMismatch,
                    "jsr-deno-permission-boundary",
                    "Package depends on Deno permissions, unstable APIs, or runtime-specific globals.",
                ),
            ],
            DxForgeImportEcosystem::Pub => vec![
                (
                    DxForgeImportRiskFlag::InstallHook,
                    "pub-build-runner",
                    "Package requires build_runner or generated source execution.",
                ),
                (
                    DxForgeImportRiskFlag::NativeBinary,
                    "pub-native-platform-channel",
                    "Package requires Flutter platform channels, FFI, or native assets.",
                ),
            ],
            DxForgeImportEcosystem::Maven => vec![
                (
                    DxForgeImportRiskFlag::InstallHook,
                    "maven-gradle-plugin",
                    "Package requires Maven or Gradle plugin/build script execution.",
                ),
                (
                    DxForgeImportRiskFlag::NativeBinary,
                    "maven-jni-native-library",
                    "Package includes JNI, native libraries, or platform-specific runtime assets.",
                ),
            ],
            DxForgeImportEcosystem::Nuget => vec![
                (
                    DxForgeImportRiskFlag::InstallHook,
                    "nuget-msbuild-target",
                    "Package contains MSBuild targets, props, analyzers, or source generators.",
                ),
                (
                    DxForgeImportRiskFlag::NativeBinary,
                    "nuget-native-runtime-asset",
                    "Package includes native runtime assets or platform-specific binaries.",
                ),
            ],
            DxForgeImportEcosystem::Composer => vec![
                (
                    DxForgeImportRiskFlag::InstallHook,
                    "composer-script",
                    "Package declares Composer scripts or install-time PHP execution.",
                ),
                (
                    DxForgeImportRiskFlag::SideEffectImport,
                    "composer-autoload-side-effect",
                    "Package relies on autoloaded side-effect files.",
                ),
            ],
            DxForgeImportEcosystem::Gem => vec![
                (
                    DxForgeImportRiskFlag::InstallHook,
                    "gem-native-extension",
                    "Gem builds native extensions or runs install hooks.",
                ),
                (
                    DxForgeImportRiskFlag::DynamicImport,
                    "gem-dynamic-require",
                    "Gem uses dynamic require/load boundaries that prevent precise slicing.",
                ),
            ],
            DxForgeImportEcosystem::Swift => vec![
                (
                    DxForgeImportRiskFlag::InstallHook,
                    "swift-package-plugin",
                    "Package requires SwiftPM plugin, macro, or generated source execution.",
                ),
                (
                    DxForgeImportRiskFlag::NativeBinary,
                    "swift-binary-target",
                    "Package contains a binary target or system library dependency.",
                ),
            ],
            DxForgeImportEcosystem::Hex => vec![
                (
                    DxForgeImportRiskFlag::InstallHook,
                    "hex-mix-compiler",
                    "Package requires Mix compiler hooks, Rebar hooks, or generated source execution.",
                ),
                (
                    DxForgeImportRiskFlag::NativeBinary,
                    "hex-native-nif",
                    "Package includes NIF, port-driver, or platform-native code.",
                ),
            ],
            DxForgeImportEcosystem::Cran => vec![
                (
                    DxForgeImportRiskFlag::InstallHook,
                    "cran-configure-script",
                    "Package declares configure, cleanup, or install-time R package scripts.",
                ),
                (
                    DxForgeImportRiskFlag::NativeBinary,
                    "cran-compiled-native-code",
                    "Package includes compiled native code under src/ or platform-specific libraries.",
                ),
            ],
        };

    shared
        .into_iter()
        .chain(ecosystem_specific)
        .map(|(risk, name, detail)| DxForgeImportManualReviewTrigger {
            ecosystem,
            risk,
            name: name.to_string(),
            detail: detail.to_string(),
        })
        .collect()
}
