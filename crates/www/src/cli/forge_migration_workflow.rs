use std::path::{Path, PathBuf};

use chrono::Utc;
use dx_compiler::ecosystem::{
    DxSourceKind, DxSourceManifest, DxSourcePackage, DxUpdateTraffic,
    build_forge_package_scorecard_for_project, canonical_package_id, plan_forge_add_variant,
    plan_forge_update_variant,
};

use super::{
    DxForgeMigrationCheck, DxForgeMigrationExpectation, DxForgeMigrationFile,
    DxForgeMigrationGalleryHostedArtifact, DxForgeMigrationGuideChecks,
    DxForgeMigrationGuideCommands, DxForgeMigrationGuideReport, DxForgeMigrationUpdatePreview,
    DxForgePackageGalleryAdvisory, DxForgePackageGalleryCheck, DxForgePackageGalleryFile,
    DxForgePackageGalleryHostedIndex, DxForgePackageGalleryMigrationGuide,
    DxForgePackageGalleryPackage, DxForgePackageGalleryReport, DxForgePackageGalleryUpdate,
    DxForgeVerifyCheck, FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_HTML,
    FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_JSON, FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_MD,
    FORGE_WWW_TEMPLATE_PACKAGE_IDS, build_forge_verify_package_report,
    forge_doctor_package_doc_name, markdown_table_cell, read_optional_forge_source_manifest,
    write_forge_beta_install_text_artifact,
};

pub(super) fn build_forge_migration_guide_report(
    project: &Path,
    package_id: &str,
    fail_under: u8,
) -> anyhow::Result<DxForgeMigrationGuideReport> {
    let canonical = canonical_package_id(package_id).to_string();
    let Some(spec) = forge_migration_ui_package_spec(&canonical) else {
        anyhow::bail!(
            "forge migration-guide currently supports `ui/button`, `ui/card`, `ui/input`, and `ui/textarea`; received `{canonical}`"
        );
    };
    let display_package_id = spec.forge_alias.to_string();

    let variant = "default".to_string();
    let manifest = read_optional_forge_source_manifest(project)?;
    let source_package = manifest.as_ref().and_then(|manifest| {
        manifest.packages.iter().find(|package| {
            canonical_package_id(&package.package_id) == canonical && package.variant == variant
        })
    });
    let preview = plan_forge_add_variant(&canonical, &variant, project)?;
    let verify = build_forge_verify_package_report(project, &canonical, &variant).ok();
    let update_preview = plan_forge_update_variant(&canonical, &variant, project).ok();
    let docs_path = project
        .join(".dx/forge/docs")
        .join(forge_doctor_package_doc_name(&canonical, &variant));
    let receipts = forge_migration_package_receipts(project, manifest.as_ref(), &canonical);
    let latest_receipt = receipts.last().cloned();
    let no_node_modules = !project.join("node_modules").exists();

    let file_map = if let Some(package) = source_package {
        package
            .files
            .iter()
            .map(|file| DxForgeMigrationFile {
                logical_path: file
                    .logical_path
                    .clone()
                    .unwrap_or_else(|| file.path.clone()),
                materialized_path: file.path.clone(),
                exists: project.join(&file.path).is_file(),
                bytes: file.bytes,
            })
            .collect::<Vec<_>>()
    } else {
        preview
            .receipt
            .file_map
            .iter()
            .map(|file| DxForgeMigrationFile {
                logical_path: file.logical_path.clone(),
                materialized_path: file.materialized_path.clone(),
                exists: project.join(&file.materialized_path).is_file(),
                bytes: file.bytes,
            })
            .collect::<Vec<_>>()
    };

    let materialized = source_package.is_some();
    let files_exist = !file_map.is_empty() && file_map.iter().all(|file| file.exists);
    let docs_present = docs_path.is_file();
    let verify_passed = verify.as_ref().is_some_and(|report| report.passed);
    let update = update_preview.as_ref();

    let checks = DxForgeMigrationGuideChecks {
        materialized: forge_migration_check(
            materialized,
            if materialized { 100 } else { 55 },
            if materialized {
                format!("`{display_package_id}` is tracked in `.dx/forge/source-.dx/build-cache/manifest.json`.")
            } else {
                format!(
                    "`{display_package_id}` is not materialized yet; run `dx add {} --write`.",
                    spec.forge_alias
                )
            },
            Some(project.join(".dx/forge/source-.dx/build-cache/manifest.json").display().to_string()),
        ),
        docs: forge_migration_check(
            docs_present,
            if docs_present { 100 } else { 40 },
            if docs_present {
                "Package-facing Forge docs are present.".to_string()
            } else {
                "Package-facing Forge docs are missing.".to_string()
            },
            Some(docs_path.display().to_string()),
        ),
        receipts: forge_migration_check(
            !receipts.is_empty(),
            if receipts.is_empty() { 45 } else { 100 },
            if receipts.is_empty() {
                format!("No Forge receipt for `{display_package_id}` was found.")
            } else {
                format!(
                    "{} Forge receipt(s) found for `{display_package_id}`.",
                    receipts.len()
                )
            },
            latest_receipt
                .as_ref()
                .map(|path| path.display().to_string()),
        ),
        verify_package: forge_migration_check(
            verify_passed,
            verify.as_ref().map(|report| report.score).unwrap_or(35),
            verify
                .as_ref()
                .map(|report| {
                    format!(
                        "`dx forge verify-package {}` score is {} / 100.",
                        spec.forge_alias,
                        report.score
                    )
                })
                .unwrap_or_else(|| {
                    format!(
                        "`dx forge verify-package {}` cannot pass until the package is materialized.",
                        spec.forge_alias
                    )
                }),
            verify
                .as_ref()
                .and_then(|report| report.scorecard.evidence.clone()),
        ),
        local_ownership: forge_migration_check(
            files_exist,
            if files_exist { 100 } else { 50 },
            if files_exist {
                "All source-owned UI component files exist as editable project files.".to_string()
            } else {
                "The guide can preview file ownership, but not every materialized file exists yet."
                    .to_string()
            },
            Some(spec.primary_materialized_path.to_string()),
        ),
        no_node_modules: forge_migration_check(
            no_node_modules,
            if no_node_modules { 100 } else { 0 },
            if no_node_modules {
                "No node_modules directory exists in the checked project.".to_string()
            } else {
                "node_modules exists in the checked project.".to_string()
            },
            Some(project.join("node_modules").display().to_string()),
        ),
    };

    let mut findings = Vec::new();
    for (label, check) in [
        ("materialized", &checks.materialized),
        ("docs", &checks.docs),
        ("receipts", &checks.receipts),
        ("verify-package", &checks.verify_package),
        ("local-ownership", &checks.local_ownership),
        ("no-node-modules", &checks.no_node_modules),
    ] {
        if !check.passed {
            findings.push(format!("{label}: {}", check.message));
        }
    }

    let score = [
        checks.materialized.score,
        checks.docs.score,
        checks.receipts.score,
        checks.verify_package.score,
        checks.local_ownership.score,
        checks.no_node_modules.score,
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    let passed = findings.is_empty() && score >= fail_under;

    Ok(DxForgeMigrationGuideReport {
        version: 1,
        project: project.to_path_buf(),
        generated_at: Utc::now().to_rfc3339(),
        passed,
        score,
        fail_under,
        package_id: display_package_id.clone(),
        variant,
        upstream_command: spec.upstream_command.to_string(),
        forge_commands: DxForgeMigrationGuideCommands {
            upstream: spec.upstream_command.to_string(),
            dry_run: format!("dx add {} --dry-run", spec.forge_alias),
            write: format!("dx add {} --write", spec.forge_alias),
            verify: format!("dx forge verify-package {} --project .", spec.forge_alias),
            update_preview: format!("dx update {} --dry-run", spec.forge_alias),
            package_gallery: "dx forge package-gallery --project . --format markdown".to_string(),
        },
        checks,
        expectation_map: forge_migration_expectation_map(&spec, &docs_path),
        file_map,
        docs_path,
        receipt_count: receipts.len(),
        latest_receipt,
        update_preview: DxForgeMigrationUpdatePreview {
            traffic: update
                .map(|update| update.traffic.as_str().to_string())
                .unwrap_or_else(|| "red".to_string()),
            changed_files: update.map(|update| update.files.len() as u64).unwrap_or(0),
            current_version: update.map(|update| update.current_version.clone()),
            latest_version: update
                .map(|update| update.latest_version.clone())
                .or_else(|| Some(preview.receipt.package.version.clone())),
        },
        no_node_modules,
        ownership_boundaries: vec![
            "The app owns the copied files; Forge owns the receipts, manifest, review policy, update preview, and rollback evidence.".to_string(),
            "Local edits are expected and reviewable; Forge update writes stay green-only unless a human accepts yellow review.".to_string(),
            "This is a Forge UI Components migration lane, not a universal npm replacement or proof of all package-manager coverage.".to_string(),
        ],
        findings,
        next_commands: vec![
            format!("dx add {} --dry-run", spec.forge_alias),
            format!("dx add {} --write", spec.forge_alias),
            format!("dx forge verify-package {} --project .", spec.forge_alias),
            format!("dx update {} --dry-run", spec.forge_alias),
        ],
    })
}

#[derive(Debug, Clone, Copy)]
struct DxForgeMigrationUiPackageSpec {
    component: &'static str,
    forge_alias: &'static str,
    upstream_command: &'static str,
    primary_materialized_path: &'static str,
    receipt_glob_hint: &'static str,
}

fn forge_migration_ui_package_spec(canonical: &str) -> Option<DxForgeMigrationUiPackageSpec> {
    match canonical {
        "shadcn/ui/button" => Some(DxForgeMigrationUiPackageSpec {
            component: "button",
            forge_alias: "ui/button",
            upstream_command: "npx shadcn@latest add button",
            primary_materialized_path: "components/ui/button.tsx",
            receipt_glob_hint: ".dx/forge/receipts/*-shadcn-ui-button.json",
        }),
        "shadcn/ui/card" => Some(DxForgeMigrationUiPackageSpec {
            component: "card",
            forge_alias: "ui/card",
            upstream_command: "npx shadcn@latest add card",
            primary_materialized_path: "components/ui/card.tsx",
            receipt_glob_hint: ".dx/forge/receipts/*-shadcn-ui-card.json",
        }),
        "shadcn/ui/input" => Some(DxForgeMigrationUiPackageSpec {
            component: "input",
            forge_alias: "ui/input",
            upstream_command: "npx shadcn@latest add input",
            primary_materialized_path: "components/ui/input.tsx",
            receipt_glob_hint: ".dx/forge/receipts/*-shadcn-ui-input.json",
        }),
        "shadcn/ui/textarea" => Some(DxForgeMigrationUiPackageSpec {
            component: "textarea",
            forge_alias: "ui/textarea",
            upstream_command: "npx shadcn@latest add textarea",
            primary_materialized_path: "components/ui/textarea.tsx",
            receipt_glob_hint: ".dx/forge/receipts/*-shadcn-ui-textarea.json",
        }),
        _ => None,
    }
}

fn forge_migration_check(
    passed: bool,
    score: u8,
    message: impl Into<String>,
    evidence: Option<String>,
) -> DxForgeMigrationCheck {
    DxForgeMigrationCheck {
        passed,
        score,
        message: message.into(),
        evidence,
    }
}

pub(super) fn forge_migration_package_receipts(
    project: &Path,
    manifest: Option<&DxSourceManifest>,
    canonical: &str,
) -> Vec<PathBuf> {
    let receipt_stem = canonical.replace(['/', '@'], "-");
    manifest
        .map(|manifest| {
            manifest
                .receipts
                .iter()
                .filter(|receipt| receipt.contains(&receipt_stem))
                .map(|receipt| project.join(".dx/forge/receipts").join(receipt))
                .filter(|path| path.is_file())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn forge_migration_expectation_map(
    spec: &DxForgeMigrationUiPackageSpec,
    docs_path: &Path,
) -> Vec<DxForgeMigrationExpectation> {
    vec![
        DxForgeMigrationExpectation {
            upstream_expectation: "Component source files are copied into the app.".to_string(),
            forge_behavior: format!(
                "Forge materializes the {} as editable source files and records every materialized path in `.dx/forge/source-.dx/build-cache/manifest.json`.",
                spec.component
            ),
            evidence: format!(
                "{} plus the source manifest file map",
                spec.primary_materialized_path
            ),
        },
        DxForgeMigrationExpectation {
            upstream_expectation: "The developer can edit the generated component.".to_string(),
            forge_behavior: "The app owns the copied files; Forge treats local edits as yellow review state during update previews instead of silently overwriting them.".to_string(),
            evidence: format!(
                "`dx update {} --dry-run` traffic classification",
                spec.forge_alias
            ),
        },
        DxForgeMigrationExpectation {
            upstream_expectation: "Install-time behavior should be understandable.".to_string(),
            forge_behavior: "Forge writes receipts with hashes, policy decisions, package metadata, and logical-to-materialized file maps.".to_string(),
            evidence: spec.receipt_glob_hint.to_string(),
        },
        DxForgeMigrationExpectation {
            upstream_expectation: "Package docs should explain ownership after generation.".to_string(),
            forge_behavior: "Forge writes package-facing docs that describe source ownership, update review, and rollback boundaries.".to_string(),
            evidence: docs_path.display().to_string(),
        },
        DxForgeMigrationExpectation {
            upstream_expectation: "No dependency tree should be needed for this copied component.".to_string(),
            forge_behavior: format!(
                "The curated Forge package path does not create `node_modules` for the source-owned {}.",
                spec.component
            ),
            evidence: "project `node_modules` boundary check".to_string(),
        },
    ]
}

pub(super) fn build_forge_package_gallery_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgePackageGalleryReport> {
    let generated_at = Utc::now().to_rfc3339();
    let scorecard = build_forge_package_scorecard_for_project(project)?;
    let manifest = read_optional_forge_source_manifest(project)?;
    let no_node_modules = !project.join("node_modules").exists();
    let mut packages = Vec::new();
    let mut findings = Vec::new();

    for package_id in FORGE_WWW_TEMPLATE_PACKAGE_IDS {
        let canonical = canonical_package_id(package_id).to_string();
        let scorecard_package = scorecard
            .packages
            .iter()
            .find(|package| package.package_id == canonical)
            .ok_or_else(|| anyhow::anyhow!("scorecard is missing launch package `{canonical}`"))?;
        let source_package = manifest.as_ref().and_then(|manifest| {
            manifest.packages.iter().find(|package| {
                canonical_package_id(&package.package_id) == canonical
                    && package.variant == "default"
                    && package.source_kind != DxSourceKind::Local
            })
        });

        let package = build_forge_package_gallery_package(
            project,
            &canonical,
            scorecard_package,
            source_package,
        )?;
        if !package.passed {
            if package.score < fail_under {
                findings.push(format!(
                    "`{}` gallery status is below beta review threshold: {} / 100",
                    package.package_id, package.score
                ));
            } else {
                findings.push(format!(
                    "`{}` gallery status did not pass beta review checks",
                    package.package_id
                ));
            }
        }
        packages.push(package);
    }

    if !no_node_modules {
        findings.push("node_modules exists in the package-gallery project".to_string());
    }

    let package_score = packages
        .iter()
        .map(|package| package.score)
        .min()
        .unwrap_or(0);
    let score = [
        scorecard.score,
        package_score,
        if no_node_modules { 100 } else { 0 },
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    let passed = findings.is_empty() && score >= fail_under;

    Ok(DxForgePackageGalleryReport {
        version: 1,
        project: project.to_path_buf(),
        generated_at,
        passed,
        score,
        fail_under,
        no_node_modules,
        package_count: packages.len(),
        packages,
        findings,
        hosted_index: None,
    })
}

pub(super) fn write_forge_package_gallery_hosted_index(
    out_dir: &Path,
    report: &DxForgePackageGalleryReport,
) -> anyhow::Result<DxForgePackageGalleryHostedIndex> {
    let route = "/forge/package-gallery/";
    let html_path = out_dir.join("forge/package-gallery/index.html");
    let json_path = out_dir.join("forge/package-gallery.json");
    let markdown_path = out_dir.join("forge/package-gallery.md");
    let migration_guides = forge_package_gallery_migration_guides(report);
    let migration_gallery = write_forge_migration_gallery_hosted_artifact(out_dir, report)?;
    let no_node_modules = !out_dir.join("node_modules").exists();
    let mut findings = Vec::new();

    if !report.passed {
        findings.push("source package-gallery report did not pass".to_string());
    }
    if !report.no_node_modules || !no_node_modules {
        findings.push(
            "node_modules exists in the package-gallery project or public output directory"
                .to_string(),
        );
    }
    if migration_guides.is_empty() {
        findings.push("hosted package-gallery has no migration-guide entries".to_string());
    }
    if !migration_gallery.passed {
        findings.push("hosted migration-gallery did not pass".to_string());
    }

    let passed = findings.is_empty();
    let package_value = serde_json::to_value(&report.packages)?;
    let migration_value = serde_json::to_value(&migration_guides)?;
    let migration_gallery_value = serde_json::to_value(&migration_gallery)?;
    let document = serde_json::json!({
        "version": 1,
        "route": route,
        "generated_at": report.generated_at,
        "passed": passed,
        "score": report.score,
        "no_node_modules": report.no_node_modules && no_node_modules,
        "package_count": report.package_count,
        "packages": package_value,
        "migration_guides": migration_value,
        "migration_gallery": migration_gallery_value,
        "honest_scope": [
            "Hosted package-gallery artifacts expose curated Forge launch packages for public review.",
            "Trust signals include local receipts, package docs, rollback/update status, and offline advisory metadata when available.",
            "This is not a universal npm replacement claim and not live vulnerability coverage unless an offline or live advisory source is attached."
        ],
        "findings": findings.clone(),
    });

    write_forge_beta_install_text_artifact(
        &html_path,
        &forge_package_gallery_hosted_html(report, &migration_guides, passed),
    )?;
    write_forge_beta_install_text_artifact(&json_path, &serde_json::to_string_pretty(&document)?)?;
    write_forge_beta_install_text_artifact(
        &markdown_path,
        &forge_package_gallery_hosted_markdown(report, &migration_guides, passed),
    )?;

    Ok(DxForgePackageGalleryHostedIndex {
        route: route.to_string(),
        out_dir: out_dir.to_path_buf(),
        html_path,
        json_path,
        markdown_path,
        passed,
        artifact_count: 3 + migration_gallery.artifact_count,
        package_count: report.package_count,
        migration_guides,
        migration_gallery,
        findings,
    })
}

fn write_forge_migration_gallery_hosted_artifact(
    out_dir: &Path,
    report: &DxForgePackageGalleryReport,
) -> anyhow::Result<DxForgeMigrationGalleryHostedArtifact> {
    let route = "/forge/migration-gallery/";
    let html_path = out_dir.join(FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_HTML);
    let json_path = out_dir.join(FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_JSON);
    let markdown_path = out_dir.join(FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_MD);
    let no_node_modules = report.no_node_modules && !out_dir.join("node_modules").exists();
    let package = report
        .packages
        .iter()
        .find(|package| package.package_id == "migration/static-site");
    let package_score = package.map(|package| package.score).unwrap_or_default();
    let package_evidence = package
        .map(|package| package.migration_checks.clone())
        .unwrap_or_default();
    let supported_scope = forge_migration_gallery_supported_scope();
    let manual_gaps = forge_migration_gallery_manual_gaps();
    let payload_comparison_boundaries = forge_migration_gallery_payload_boundaries();
    let next_commands = vec![
        "dx add migration/static-site --write".to_string(),
        "dx forge migration-audit --input <export.html-or-dir> --format markdown".to_string(),
        "dx forge migrated-route-benchmark --format markdown".to_string(),
        "dx forge verify-package migration/static-site --format markdown".to_string(),
        "dx forge package-gallery --public-index public --format json".to_string(),
    ];
    let mut findings = Vec::new();

    if package.is_none() {
        findings
            .push("migration/static-site package is missing from the gallery report".to_string());
    }
    if package_evidence.is_empty() {
        findings.push("migration/static-site package evidence is missing".to_string());
    }
    if package_evidence.iter().any(|check| !check.passed) {
        findings.push("migration/static-site package evidence has failing checks".to_string());
    }
    if package_score < report.fail_under {
        findings.push(format!(
            "migration/static-site package score {package_score} is below {}",
            report.fail_under
        ));
    }
    if !no_node_modules {
        findings.push(
            "node_modules exists in the project or hosted migration-gallery output".to_string(),
        );
    }

    let passed = findings.is_empty();
    let artifact = DxForgeMigrationGalleryHostedArtifact {
        route: route.to_string(),
        out_dir: out_dir.to_path_buf(),
        html_path: html_path.clone(),
        json_path: json_path.clone(),
        markdown_path: markdown_path.clone(),
        passed,
        artifact_count: 3,
        package_id: "migration/static-site".to_string(),
        score: package_score,
        no_node_modules,
        supported_scope,
        manual_gaps,
        package_evidence,
        payload_comparison_boundaries,
        next_commands,
        findings,
    };

    write_forge_beta_install_text_artifact(
        &html_path,
        &forge_migration_gallery_hosted_html(&artifact),
    )?;
    write_forge_beta_install_text_artifact(&json_path, &serde_json::to_string_pretty(&artifact)?)?;
    write_forge_beta_install_text_artifact(
        &markdown_path,
        &forge_migration_gallery_hosted_markdown(&artifact),
    )?;

    Ok(artifact)
}

fn forge_migration_gallery_supported_scope() -> Vec<String> {
    vec![
        "A scoped static WordPress or HTML page with title, description, canonical route, reviewed HTML, and editable source-owned content.".to_string(),
        "Asset source-to-target mapping notes for copied media that still need optimization, fingerprinting, and cache policy review.".to_string(),
        "Visible manual-review warnings in the rendered source component so imported HTML cannot look production-safe by accident.".to_string(),
        "Forge receipts, docs, source manifest evidence, and no node_modules proof for the migration/static-site package.".to_string(),
    ]
}

fn forge_migration_gallery_manual_gaps() -> Vec<String> {
    vec![
        "Dynamic WordPress behavior such as forms, comments, search, ecommerce, memberships, accounts, shortcodes, and plugin runtime calls remains application-owned work.".to_string(),
        "Theme parity, block-editor semantics, analytics, redirects, CMS editing, and production HTML sanitization need explicit product decisions before launch.".to_string(),
        "Media files must be copied from the original source, optimized, and reviewed against the deployment cache and accessibility policy.".to_string(),
    ]
}

fn forge_migration_gallery_payload_boundaries() -> Vec<String> {
    vec![
        "Compare only a scoped static migrated route against WordPress-style and Next.js-style fixtures with equivalent visible content.".to_string(),
        "Record decoded bytes, Brotli bytes, route timing, and static asset counts; do not claim whole-framework superiority from one fixture.".to_string(),
        "Keep security claims separate from payload claims: Forge can prove source ownership and no package installs here, not automatic sanitization of arbitrary legacy HTML.".to_string(),
        "Mark any future benchmark as a reproducible fixture, not as a replacement claim for all WordPress or Next.js applications.".to_string(),
    ]
}

fn forge_package_gallery_migration_guides(
    report: &DxForgePackageGalleryReport,
) -> Vec<DxForgePackageGalleryMigrationGuide> {
    report
        .packages
        .iter()
        .map(|package| match package.package_id.as_str() {
            "shadcn/ui/button" => DxForgePackageGalleryMigrationGuide {
                package_id: package.package_id.clone(),
                title: "UI button migration".to_string(),
                command:
                    "dx forge migration-guide --package ui/button --format markdown"
                        .to_string(),
                href: "#migration-ui-button".to_string(),
                supported: true,
                summary: "Maps `npx shadcn@latest add button` to Forge materialization, receipts, local ownership, update preview, rollback, and no node_modules proof.".to_string(),
            },
            "shadcn/ui/card" => DxForgePackageGalleryMigrationGuide {
                package_id: package.package_id.clone(),
                title: "UI card migration".to_string(),
                command:
                    "dx forge migration-guide --package ui/card --format markdown"
                        .to_string(),
                href: "#migration-ui-card".to_string(),
                supported: true,
                summary: "Maps `npx shadcn@latest add card` to Forge materialization, receipts, local ownership, update preview, rollback, and no node_modules proof.".to_string(),
            },
            "shadcn/ui/input" => DxForgePackageGalleryMigrationGuide {
                package_id: package.package_id.clone(),
                title: "UI input migration".to_string(),
                command:
                    "dx forge migration-guide --package ui/input --format markdown"
                        .to_string(),
                href: "#migration-ui-input".to_string(),
                supported: true,
                summary: "Maps the v4 radix `Input` registry source to Forge materialization, receipts, local ownership, update preview, rollback, and no node_modules proof.".to_string(),
            },
            "shadcn/ui/textarea" => DxForgePackageGalleryMigrationGuide {
                package_id: package.package_id.clone(),
                title: "UI textarea migration".to_string(),
                command:
                    "dx forge migration-guide --package ui/textarea --format markdown"
                        .to_string(),
                href: "#migration-ui-textarea".to_string(),
                supported: true,
                summary: "Maps the v4 radix `Textarea` registry source to Forge materialization, receipts, local ownership, update preview, rollback, and no node_modules proof.".to_string(),
            },
            "dx/icon/search" => DxForgePackageGalleryMigrationGuide {
                package_id: package.package_id.clone(),
                title: "selected icon adoption".to_string(),
                command: "dx add icon search --write".to_string(),
                href: "#migration-dx-icon-search".to_string(),
                supported: true,
                summary: "Shows the selected-icon package path where Forge materializes one editable icon and its helper files instead of an icon library tree.".to_string(),
            },
            "auth/better-auth" => DxForgePackageGalleryMigrationGuide {
                package_id: package.package_id.clone(),
                title: "Authentication launch slice".to_string(),
                command: "dx add auth/better-auth --write".to_string(),
                href: "#migration-auth-better-auth".to_string(),
                supported: true,
                summary: "Shows the source-owned OAuth boundary, Better Auth server factory, React client, Next route helper, env contract, discovery metadata, and app-owned production responsibilities.".to_string(),
            },
            "migration/static-site" => DxForgePackageGalleryMigrationGuide {
                package_id: package.package_id.clone(),
                title: "WordPress/static page migration seed".to_string(),
                command: "dx add migration/static-site --write".to_string(),
                href: "#migration-migration-static-site".to_string(),
                supported: true,
                summary: "Shows the source-owned static migration example for simple WordPress/static pages, including editable content, asset mapping notes, visible manual-review warnings, and no package install.".to_string(),
            },
            _ => DxForgePackageGalleryMigrationGuide {
                package_id: package.package_id.clone(),
                title: "package docs".to_string(),
                command: "dx forge package-gallery --project . --format markdown".to_string(),
                href: format!(
                    "#migration-{}",
                    package.package_id.replace(['/', '@'], "-")
                ),
                supported: false,
                summary: "Use the package-facing docs and gallery entry for this package until a focused migration guide exists.".to_string(),
            },
        })
        .collect()
}

fn build_forge_package_gallery_package(
    project: &Path,
    canonical: &str,
    scorecard_package: &dx_compiler::ecosystem::DxForgePackageScorecardEntry,
    source_package: Option<&DxSourcePackage>,
) -> anyhow::Result<DxForgePackageGalleryPackage> {
    let materialized = source_package.is_some();
    let variant = source_package
        .map(|package| package.variant.clone())
        .unwrap_or_else(|| "default".to_string());
    let file_map = if let Some(package) = source_package {
        forge_package_gallery_manifest_file_map(project, package)
    } else {
        let preview = plan_forge_add_variant(canonical, &variant, project)?;
        preview
            .receipt
            .file_map
            .iter()
            .map(|file| DxForgePackageGalleryFile {
                logical_path: file.logical_path.clone(),
                materialized_path: file.materialized_path.clone(),
                hash: file.hash.clone(),
                bytes: file.bytes,
                exists: project.join(&file.materialized_path).is_file(),
            })
            .collect()
    };

    let verify = build_forge_verify_package_report(project, canonical, &variant).ok();
    let migration_checks = verify
        .as_ref()
        .map(|report| {
            report
                .package_specific_checks
                .iter()
                .map(forge_package_gallery_check)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let docs_status = verify
        .as_ref()
        .map(|report| forge_package_gallery_check(&report.docs))
        .unwrap_or_else(|| {
            forge_package_gallery_missing_check(
                "docs",
                format!("`{canonical}` is not materialized, so package docs are not present."),
            )
        });
    let rollback_status = verify
        .as_ref()
        .map(|report| forge_package_gallery_check(&report.rollback))
        .unwrap_or_else(|| {
            forge_package_gallery_missing_check(
                "rollback",
                format!("`{canonical}` is not materialized, so rollback state is unavailable."),
            )
        });
    let scorecard_status = verify
        .as_ref()
        .map(|report| forge_package_gallery_check(&report.scorecard))
        .unwrap_or_else(|| {
            forge_package_gallery_missing_check(
                "scorecard",
                format!("`{canonical}` has no local project scorecard evidence yet."),
            )
        });
    let update_status = if let Some(report) = verify.as_ref() {
        let update = plan_forge_update_variant(canonical, &variant, project).ok();
        DxForgePackageGalleryUpdate {
            check: forge_package_gallery_check(&report.update),
            current_version: update.as_ref().map(|update| update.current_version.clone()),
            latest_version: update.as_ref().map(|update| update.latest_version.clone()),
            changed_files: update.map(|update| update.files.len() as u64).unwrap_or(0),
        }
    } else {
        DxForgePackageGalleryUpdate {
            check: forge_package_gallery_missing_check(
                "update",
                format!("`{canonical}` is not materialized, so update preview is unavailable."),
            ),
            current_version: None,
            latest_version: Some(scorecard_package.version.clone()),
            changed_files: 0,
        }
    };

    let verify_passed = verify.as_ref().is_some_and(|report| report.passed);
    let mut package_score = verify.as_ref().map(|report| report.score).unwrap_or(0);
    if verify_passed {
        package_score = package_score.max(90);
    }
    if !materialized {
        package_score = package_score.min(60);
    }
    if file_map.is_empty() {
        package_score = package_score.min(70);
    }
    let passed = materialized
        && verify_passed
        && package_score >= 90
        && docs_status.passed
        && update_status.check.passed
        && rollback_status.passed
        && scorecard_status.passed
        && migration_checks.iter().all(|check| check.passed)
        && !file_map.is_empty();

    Ok(DxForgePackageGalleryPackage {
        package_id: canonical.to_string(),
        variant,
        version: scorecard_package.version.clone(),
        description: scorecard_package.description.clone(),
        materialized,
        source_owned: scorecard_package.source_owned,
        ownership_boundary: "Forge materializes this package as editable local source; the app owns the copied files while Forge keeps receipts, docs, update previews, and rollback evidence reviewable.".to_string(),
        public_claim: scorecard_package.public_claim.clone(),
        launch_boundary: scorecard_package.launch_boundary.clone(),
        file_map,
        advisory: DxForgePackageGalleryAdvisory {
            coverage_kind: scorecard_package
                .advisory_review
                .coverage_kind
                .as_str()
                .to_string(),
            provider: scorecard_package.advisory_review.provider.clone(),
            live_coverage: scorecard_package.advisory_review.live_coverage,
            finding_count: scorecard_package.advisory_review.finding_count,
            reviewed_at: scorecard_package.advisory_review.reviewed_at.clone(),
            placeholder_present: scorecard_package.advisory_review.coverage_kind.as_str()
                == "curated-fixture"
                && scorecard_package.advisory_review.provider
                    == "dx-forge-curated-advisory-fixture"
                && !scorecard_package.advisory_review.live_coverage,
            note: scorecard_package.advisory_review.note.clone(),
        },
        docs_status,
        update_status,
        rollback_status,
        scorecard_status,
        migration_checks,
        score: package_score,
        passed,
    })
}

fn forge_package_gallery_manifest_file_map(
    project: &Path,
    package: &DxSourcePackage,
) -> Vec<DxForgePackageGalleryFile> {
    package
        .files
        .iter()
        .map(|file| DxForgePackageGalleryFile {
            logical_path: file
                .logical_path
                .clone()
                .unwrap_or_else(|| file.path.clone()),
            materialized_path: file.path.clone(),
            hash: file.hash.clone(),
            bytes: file.bytes,
            exists: project.join(&file.path).is_file(),
        })
        .collect()
}

fn forge_package_gallery_check(check: &DxForgeVerifyCheck) -> DxForgePackageGalleryCheck {
    DxForgePackageGalleryCheck {
        name: check.name.clone(),
        passed: check.passed,
        traffic: check.traffic.as_str().to_string(),
        score: check.score,
        message: check.message.clone(),
        evidence: check.evidence.clone(),
    }
}

fn forge_package_gallery_missing_check(
    name: impl Into<String>,
    message: impl Into<String>,
) -> DxForgePackageGalleryCheck {
    DxForgePackageGalleryCheck {
        name: name.into(),
        passed: false,
        traffic: DxUpdateTraffic::Red.as_str().to_string(),
        score: 0,
        message: message.into(),
        evidence: None,
    }
}

pub(super) fn forge_migration_guide_terminal(report: &DxForgeMigrationGuideReport) -> String {
    let mut output = format!(
        "DX Forge UI Components migration guide\nProject: {}\nPackage: {}\nPassed: {}\nScore: {} / 100\nUpstream command: {}\nForge write: {}\nNo node_modules: {}\n",
        report.project.display(),
        report.package_id,
        report.passed,
        report.score,
        report.upstream_command,
        report.forge_commands.write,
        report.no_node_modules
    );
    if !report.findings.is_empty() {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

pub(super) fn forge_migration_guide_markdown(report: &DxForgeMigrationGuideReport) -> String {
    let mut output = format!(
        "# DX Forge UI Components Migration Guide\n\n- Project: `{}`\n- Package: `{}`\n- Generated: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- No `node_modules`: `{}`\n\n",
        report.project.display(),
        report.package_id,
        report.generated_at,
        report.passed,
        report.score,
        report.fail_under,
        report.no_node_modules
    );

    output.push_str("## Command Mapping\n\n");
    output.push_str("| Upstream habit | Forge command | What changes |\n");
    output.push_str("| --- | --- | --- |\n");
    output.push_str(&format!(
        "| `{}` | `{}` | Preview source-owned files, receipt metadata, and policy decisions before writing. |\n",
        report.forge_commands.upstream, report.forge_commands.dry_run
    ));
    output.push_str(&format!(
        "| `{}` | `{}` | Materialize editable source and record Forge receipts instead of creating a dependency tree. |\n",
        report.forge_commands.upstream, report.forge_commands.write
    ));
    output.push_str(&format!(
        "| Update generated code manually or rerun the upstream command | `{}` | Review green/yellow/red file traffic before any write. |\n",
        report.forge_commands.update_preview
    ));
    output.push_str(&format!(
        "| Check what was generated | `{}` | Verify docs, receipts, update preview, rollback state, and scorecard evidence. |\n",
        report.forge_commands.verify
    ));

    output.push_str("\n## Migration Checks\n\n");
    output.push_str("| Check | Passed | Score | Evidence | Message |\n");
    output.push_str("| --- | --- | ---: | --- | --- |\n");
    for (label, check) in [
        ("Materialized", &report.checks.materialized),
        ("Receipts And Manifest", &report.checks.receipts),
        ("Docs", &report.checks.docs),
        ("Verify Package", &report.checks.verify_package),
        ("Local Ownership", &report.checks.local_ownership),
        ("No `node_modules`", &report.checks.no_node_modules),
    ] {
        output.push_str(&format!(
            "| {} | `{}` | `{}` | `{}` | {} |\n",
            markdown_table_cell(label),
            check.passed,
            check.score,
            markdown_table_cell(check.evidence.as_deref().unwrap_or("-")),
            markdown_table_cell(&check.message)
        ));
    }

    output.push_str("\n## Expectation Map\n\n");
    output.push_str("| Upstream expectation | Forge behavior | Evidence |\n");
    output.push_str("| --- | --- | --- |\n");
    for item in &report.expectation_map {
        output.push_str(&format!(
            "| {} | {} | {} |\n",
            markdown_table_cell(&item.upstream_expectation),
            markdown_table_cell(&item.forge_behavior),
            markdown_table_cell(&item.evidence)
        ));
    }

    output.push_str("\n## File Map\n\n");
    output.push_str("| Logical file | Materialized file | Exists | Bytes |\n");
    output.push_str("| --- | --- | --- | ---: |\n");
    for file in &report.file_map {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} |\n",
            markdown_table_cell(&file.logical_path),
            markdown_table_cell(&file.materialized_path),
            file.exists,
            file.bytes
        ));
    }

    output.push_str("\n## Receipts And Manifest\n\n");
    output.push_str(&format!(
        "- Docs path: `{}`\n- Receipt count: `{}`\n- Latest receipt: `{}`\n\n",
        report.docs_path.display(),
        report.receipt_count,
        report
            .latest_receipt
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "none".to_string())
    ));

    output.push_str("## Local Ownership\n\n");
    for boundary in &report.ownership_boundaries {
        output.push_str(&format!("- {boundary}\n"));
    }

    output.push_str("\n## Update And Rollback\n\n");
    output.push_str(&format!(
        "- Update preview command: `{}`\n- Current version: `{}`\n- Latest version: `{}`\n- Traffic: `{}`\n- Changed files in preview: `{}`\n\n",
        report.forge_commands.update_preview,
        report
            .update_preview
            .current_version
            .as_deref()
            .unwrap_or("n/a"),
        report
            .update_preview
            .latest_version
            .as_deref()
            .unwrap_or("n/a"),
        report.update_preview.traffic,
        report.update_preview.changed_files
    ));

    output.push_str("## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- No migration-guide findings for the configured threshold.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {}\n", markdown_table_cell(finding)));
        }
    }

    output.push_str("\n## Next Commands\n\n");
    for command in &report.next_commands {
        output.push_str(&format!("- `{command}`\n"));
    }

    output
}

pub(super) fn forge_migration_guide_failure_summary(
    report: &DxForgeMigrationGuideReport,
) -> String {
    if report.findings.is_empty() {
        return format!(
            "DX Forge migration-guide did not pass: score {} / 100, required {} / 100",
            report.score, report.fail_under
        );
    }

    format!(
        "DX Forge migration-guide did not pass: {}",
        report.findings.join("; ")
    )
}

pub(super) fn forge_package_gallery_terminal(report: &DxForgePackageGalleryReport) -> String {
    let mut output = format!(
        "DX Forge source-owned package gallery\nProject: {}\nGenerated: {}\nPassed: {}\nScore: {} / 100\nPackages: {}\nNo node_modules: {}\n",
        report.project.display(),
        report.generated_at,
        report.passed,
        report.score,
        report.package_count,
        report.no_node_modules
    );
    output.push_str("\nPackages:\n");
    for package in &report.packages {
        output.push_str(&format!(
            "- {}@{}: score {}, files {}, advisory {}, update {}, rollback {}\n",
            package.package_id,
            package.version,
            package.score,
            package.file_map.len(),
            package.advisory.provider,
            package.update_status.check.traffic,
            package.rollback_status.traffic
        ));
    }
    if !report.findings.is_empty() {
        output.push_str("\nFindings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

pub(super) fn forge_package_gallery_markdown(report: &DxForgePackageGalleryReport) -> String {
    let mut output = format!(
        "# DX Forge Source-Owned Package Gallery\n\n- Project: `{}`\n- Generated: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- Packages: `{}`\n- no `node_modules`: `{}`\n\n",
        report.project.display(),
        report.generated_at,
        report.passed,
        report.score,
        report.fail_under,
        report.package_count,
        report.no_node_modules
    );

    output.push_str("## Packages\n\n");
    output.push_str("| Package | Version | Materialized | Score | Files | Update | Rollback |\n");
    output.push_str("| --- | --- | --- | ---: | ---: | --- | --- |\n");
    for package in &report.packages {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} | {} | `{}` | `{}` |\n",
            package.package_id,
            package.version,
            package.materialized,
            package.score,
            package.file_map.len(),
            package.update_status.check.traffic,
            package.rollback_status.traffic
        ));
    }

    output.push_str("\n## Advisory Placeholders\n\n");
    output.push_str("| Package | Coverage | Provider | Live | Findings | Placeholder | Note |\n");
    output.push_str("| --- | --- | --- | --- | ---: | --- | --- |\n");
    for package in &report.packages {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | {} | `{}` | {} |\n",
            package.package_id,
            package.advisory.coverage_kind,
            package.advisory.provider,
            package.advisory.live_coverage,
            package.advisory.finding_count,
            package.advisory.placeholder_present,
            markdown_table_cell(&package.advisory.note)
        ));
    }

    for package in &report.packages {
        output.push_str(&format!(
            "\n## `{}`\n\n- Ownership boundary: {}\n- Public claim: {}\n- Launch boundary: {}\n- Docs: `{}` ({})\n- Scorecard: `{}` ({})\n\n",
            package.package_id,
            markdown_table_cell(&package.ownership_boundary),
            markdown_table_cell(&package.public_claim),
            markdown_table_cell(&package.launch_boundary),
            package.docs_status.traffic,
            markdown_table_cell(&package.docs_status.message),
            package.scorecard_status.traffic,
            markdown_table_cell(&package.scorecard_status.message)
        ));

        output.push_str("### File Map\n\n");
        output.push_str("| Logical path | Materialized path | Bytes | Exists |\n");
        output.push_str("| --- | --- | ---: | --- |\n");
        for file in &package.file_map {
            output.push_str(&format!(
                "| `{}` | `{}` | {} | `{}` |\n",
                markdown_table_cell(&file.logical_path),
                markdown_table_cell(&file.materialized_path),
                file.bytes,
                file.exists
            ));
        }

        output.push_str("\n### Update And Rollback\n\n");
        output.push_str("| Check | Passed | Traffic | Score | Message | Evidence |\n");
        output.push_str("| --- | --- | --- | ---: | --- | --- |\n");
        output.push_str(&format!(
            "| `update` | `{}` | `{}` | {} | {} | `{}` |\n",
            package.update_status.check.passed,
            package.update_status.check.traffic,
            package.update_status.check.score,
            markdown_table_cell(&package.update_status.check.message),
            markdown_table_cell(
                package
                    .update_status
                    .check
                    .evidence
                    .as_deref()
                    .unwrap_or("-")
            )
        ));
        output.push_str(&format!(
            "| `rollback` | `{}` | `{}` | {} | {} | `{}` |\n",
            package.rollback_status.passed,
            package.rollback_status.traffic,
            package.rollback_status.score,
            markdown_table_cell(&package.rollback_status.message),
            markdown_table_cell(package.rollback_status.evidence.as_deref().unwrap_or("-"))
        ));
    }

    output.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- `pass`: package gallery has no findings.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }

    output
}

fn forge_package_gallery_hosted_markdown(
    report: &DxForgePackageGalleryReport,
    migration_guides: &[DxForgePackageGalleryMigrationGuide],
    passed: bool,
) -> String {
    let mut output = format!(
        "# DX Forge Hosted Package Gallery\n\n- Route: `/forge/package-gallery/`\n- Generated: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Packages: `{}`\n- no `node_modules`: `{}`\n\n",
        report.generated_at, passed, report.score, report.package_count, report.no_node_modules
    );

    output.push_str("## Trust Signals\n\n");
    output.push_str("| Package | Score | Advisory | Docs | Update | Rollback |\n");
    output.push_str("| --- | ---: | --- | --- | --- | --- |\n");
    for package in &report.packages {
        output.push_str(&format!(
            "| `{}` | {} | `{}` via `{}` | `{}` | `{}` | `{}` |\n",
            package.package_id,
            package.score,
            package.advisory.coverage_kind,
            package.advisory.provider,
            package.docs_status.traffic,
            package.update_status.check.traffic,
            package.rollback_status.traffic
        ));
    }

    output.push_str("\n## Migration Guides\n\n");
    for guide in migration_guides {
        output.push_str(&format!(
            "- `{}`: {}. Command: `{}`\n",
            guide.package_id,
            markdown_table_cell(&guide.summary),
            guide.command
        ));
    }
    if let Some(hosted) = report.hosted_index.as_ref() {
        output.push_str(&format!(
            "\n## Migration Gallery\n\n- Route: `{}`\n- Package: `{}`\n- Score: `{}` / `100`\n\n",
            hosted.migration_gallery.route,
            hosted.migration_gallery.package_id,
            hosted.migration_gallery.score
        ));
    } else {
        output.push_str("\n## Migration Gallery\n\n- Route: `/forge/migration-gallery/`\n- Package: `migration/static-site`\n");
    }

    output.push_str("\n## Honest Scope\n\n");
    output.push_str("- Hosted package-gallery artifacts are public review material for the curated Forge beta package set.\n");
    output.push_str("- This is not a universal npm replacement claim and not live vulnerability coverage unless a real advisory source is attached.\n");
    output.push_str("- The public route proves source-owned files, receipts, docs, update/rollback status, and no node_modules evidence for the checked project.\n");

    output
}

fn forge_migration_gallery_hosted_markdown(
    artifact: &DxForgeMigrationGalleryHostedArtifact,
) -> String {
    let mut output = format!(
        "# DX Forge Migration Gallery\n\n- Route: `{}`\n- Package: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- no `node_modules`: `{}`\n\n",
        artifact.route,
        artifact.package_id,
        artifact.passed,
        artifact.score,
        artifact.no_node_modules
    );

    output.push_str("## Supported Scope\n\n");
    for item in &artifact.supported_scope {
        output.push_str(&format!("- {}\n", markdown_table_cell(item)));
    }

    output.push_str("\n## Manual Gaps\n\n");
    for item in &artifact.manual_gaps {
        output.push_str(&format!("- {}\n", markdown_table_cell(item)));
    }

    output.push_str("\n## Package Evidence\n\n");
    output.push_str("| Check | Passed | Traffic | Score | Evidence | Message |\n");
    output.push_str("| --- | --- | --- | ---: | --- | --- |\n");
    for check in &artifact.package_evidence {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} | `{}` | {} |\n",
            check.name,
            check.passed,
            check.traffic,
            check.score,
            markdown_table_cell(check.evidence.as_deref().unwrap_or("-")),
            markdown_table_cell(&check.message)
        ));
    }

    output.push_str("\n## Payload Comparison Boundaries\n\n");
    for item in &artifact.payload_comparison_boundaries {
        output.push_str(&format!("- {}\n", markdown_table_cell(item)));
    }

    output.push_str("\n## Next Commands\n\n");
    for command in &artifact.next_commands {
        output.push_str(&format!("- `{}`\n", markdown_table_cell(command)));
    }

    output.push_str("\n## Findings\n\n");
    if artifact.findings.is_empty() {
        output.push_str("- `pass`: hosted migration-gallery has no findings.\n");
    } else {
        for finding in &artifact.findings {
            output.push_str(&format!("- {}\n", markdown_table_cell(finding)));
        }
    }

    output
}

fn forge_migration_gallery_hosted_html(artifact: &DxForgeMigrationGalleryHostedArtifact) -> String {
    let supported_scope = html_list(&artifact.supported_scope);
    let manual_gaps = html_list(&artifact.manual_gaps);
    let payload_boundaries = html_list(&artifact.payload_comparison_boundaries);
    let next_commands = artifact
        .next_commands
        .iter()
        .map(|command| {
            format!(
                "<li><code>{}</code></li>",
                forge_package_gallery_html_escape(command)
            )
        })
        .collect::<String>();
    let package_evidence = artifact
        .package_evidence
        .iter()
        .map(|check| {
            format!(
                r#"<tr><td>{name}</td><td>{passed}</td><td>{traffic}</td><td>{score}</td><td>{message}</td></tr>"#,
                name = forge_package_gallery_html_escape(&check.name),
                passed = check.passed,
                traffic = forge_package_gallery_html_escape(&check.traffic),
                score = check.score,
                message = forge_package_gallery_html_escape(&check.message)
            )
        })
        .collect::<String>();

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>DX Forge Migration Gallery</title>
  <style>
    :root {{ color-scheme: light; font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; background: #f8fafc; color: #16181d; }}
    body {{ margin: 0; }}
    main {{ max-width: 1080px; margin: 0 auto; padding: 40px 20px 56px; }}
    header, section {{ display: grid; gap: 12px; }}
    header {{ margin-bottom: 24px; }}
    h1 {{ margin: 0; font-size: 38px; line-height: 1.08; letter-spacing: 0; }}
    h2 {{ margin: 0; font-size: 20px; letter-spacing: 0; }}
    p, li {{ line-height: 1.6; color: #3b414d; }}
    .summary {{ display: grid; grid-template-columns: repeat(4, minmax(0, 1fr)); gap: 12px; margin: 22px 0; }}
    .metric, .panel {{ border: 1px solid #d9dde5; background: #fff; border-radius: 8px; padding: 16px; }}
    .metric span {{ display: block; color: #5a6270; font-size: 13px; }}
    .metric strong {{ display: block; margin-top: 6px; font-size: 21px; }}
    .grid {{ display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 14px; }}
    ul {{ margin: 0; padding-left: 20px; }}
    code {{ border-radius: 6px; background: #eef2f7; padding: 3px 6px; overflow-wrap: anywhere; }}
    table {{ width: 100%; border-collapse: collapse; font-size: 14px; }}
    th, td {{ border-bottom: 1px solid #e1e5ec; padding: 10px 8px; text-align: left; vertical-align: top; }}
    th {{ color: #5a6270; font-weight: 700; }}
    @media (max-width: 820px) {{ .summary, .grid {{ grid-template-columns: 1fr; }} main {{ padding: 28px 16px 44px; }} h1 {{ font-size: 30px; }} table {{ display: block; overflow-x: auto; }} }}
  </style>
</head>
<body>
  <main>
    <header>
      <p>DX Forge public beta</p>
      <h1>DX Forge Migration Gallery</h1>
      <p>Hosted conversion workflow for <strong>{package_id}</strong>. This is not a full WordPress plugin or theme migration; it is a reviewable static-page lane with source-owned files, package evidence, and no node_modules.</p>
    </header>
    <section class="summary" aria-label="Migration gallery summary">
      <div class="metric"><span>Status</span><strong>{passed}</strong></div>
      <div class="metric"><span>Score</span><strong>{score}/100</strong></div>
      <div class="metric"><span>Package</span><strong>{package_id}</strong></div>
      <div class="metric"><span>Dependency boundary</span><strong>no node_modules</strong></div>
    </section>
    <section class="grid">
      <div class="panel"><h2>Supported scope</h2><ul>{supported_scope}</ul></div>
      <div class="panel"><h2>Manual gaps</h2><ul>{manual_gaps}</ul></div>
    </section>
    <section class="panel" style="margin-top: 14px;">
      <h2>Package evidence</h2>
      <table><thead><tr><th>Check</th><th>Passed</th><th>Traffic</th><th>Score</th><th>Message</th></tr></thead><tbody>{package_evidence}</tbody></table>
    </section>
    <section class="panel" style="margin-top: 14px;">
      <h2>Payload comparison boundaries</h2>
      <ul>{payload_boundaries}</ul>
    </section>
    <section class="panel" style="margin-top: 14px;">
      <h2>Next commands</h2>
      <ul>{next_commands}</ul>
    </section>
  </main>
</body>
</html>
"#,
        package_id = forge_package_gallery_html_escape(&artifact.package_id),
        passed = artifact.passed,
        score = artifact.score,
        supported_scope = supported_scope,
        manual_gaps = manual_gaps,
        package_evidence = package_evidence,
        payload_boundaries = payload_boundaries,
        next_commands = next_commands
    )
}

fn forge_package_gallery_hosted_html(
    report: &DxForgePackageGalleryReport,
    migration_guides: &[DxForgePackageGalleryMigrationGuide],
    passed: bool,
) -> String {
    let mut packages = String::new();
    for package in &report.packages {
        packages.push_str(&format!(
            r#"<article class="package" id="package-{id}">
  <div class="package-header">
    <h2>{package_id}</h2>
    <span class="score">{score}/100</span>
  </div>
  <p>{description}</p>
  <dl>
    <div><dt>Version</dt><dd>{version}</dd></div>
    <div><dt>Files</dt><dd>{files}</dd></div>
    <div><dt>Docs</dt><dd>{docs}</dd></div>
    <div><dt>Update</dt><dd>{update}</dd></div>
    <div><dt>Rollback</dt><dd>{rollback}</dd></div>
    <div><dt>Advisory</dt><dd>{advisory} via {provider}</dd></div>
  </dl>
  <p><strong>Ownership:</strong> {ownership}</p>
  <p><strong>Boundary:</strong> {boundary}</p>
</article>
"#,
            id = forge_package_gallery_anchor(&package.package_id),
            package_id = forge_package_gallery_html_escape(&package.package_id),
            score = package.score,
            description = forge_package_gallery_html_escape(&package.description),
            version = forge_package_gallery_html_escape(&package.version),
            files = package.file_map.len(),
            docs = forge_package_gallery_html_escape(&package.docs_status.traffic),
            update = forge_package_gallery_html_escape(&package.update_status.check.traffic),
            rollback = forge_package_gallery_html_escape(&package.rollback_status.traffic),
            advisory = forge_package_gallery_html_escape(&package.advisory.coverage_kind),
            provider = forge_package_gallery_html_escape(&package.advisory.provider),
            ownership = forge_package_gallery_html_escape(&package.ownership_boundary),
            boundary = forge_package_gallery_html_escape(&package.launch_boundary)
        ));
    }

    let mut migrations = String::new();
    for guide in migration_guides {
        migrations.push_str(&format!(
            r#"<li id="{href}"><strong>{title}</strong><span>{package_id}</span><code>{command}</code><p>{summary}</p></li>"#,
            href = guide.href.trim_start_matches('#'),
            title = forge_package_gallery_html_escape(&guide.title),
            package_id = forge_package_gallery_html_escape(&guide.package_id),
            command = forge_package_gallery_html_escape(&guide.command),
            summary = forge_package_gallery_html_escape(&guide.summary)
        ));
    }

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>DX Forge Package Gallery</title>
  <style>
    :root {{ color-scheme: light; font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; background: #f7f8fa; color: #16181d; }}
    body {{ margin: 0; }}
    main {{ max-width: 1120px; margin: 0 auto; padding: 40px 20px 56px; }}
    header {{ display: grid; gap: 12px; margin-bottom: 28px; }}
    h1 {{ font-size: 38px; line-height: 1.08; margin: 0; letter-spacing: 0; }}
    h2 {{ font-size: 20px; margin: 0; letter-spacing: 0; }}
    h3 {{ font-size: 16px; margin: 0 0 10px; letter-spacing: 0; }}
    p {{ margin: 0; line-height: 1.6; color: #3b414d; }}
    .summary {{ display: grid; grid-template-columns: repeat(4, minmax(0, 1fr)); gap: 12px; margin: 24px 0; }}
    .metric, .package, .panel {{ border: 1px solid #d9dde5; background: #fff; border-radius: 8px; padding: 16px; }}
    .metric span {{ display: block; color: #5a6270; font-size: 13px; }}
    .metric strong {{ display: block; margin-top: 6px; font-size: 22px; }}
    .grid {{ display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 14px; }}
    .package-header {{ display: flex; align-items: center; justify-content: space-between; gap: 12px; margin-bottom: 8px; }}
    .score {{ font-weight: 700; color: #0f766e; }}
    dl {{ display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 8px 14px; margin: 14px 0; }}
    dt {{ color: #5a6270; font-size: 12px; text-transform: uppercase; }}
    dd {{ margin: 2px 0 0; font-weight: 600; }}
    code {{ display: block; white-space: normal; overflow-wrap: anywhere; border-radius: 6px; background: #eef2f7; padding: 8px; margin: 8px 0; }}
    ul {{ margin: 0; padding-left: 20px; }}
    li {{ margin: 12px 0; }}
    li span {{ display: block; color: #5a6270; margin-top: 2px; }}
    @media (max-width: 820px) {{ .summary, .grid, dl {{ grid-template-columns: 1fr; }} main {{ padding: 28px 16px 44px; }} h1 {{ font-size: 30px; }} }}
  </style>
</head>
<body>
  <main>
    <header>
      <p>DX Forge public beta</p>
      <h1>DX Forge Package Gallery</h1>
      <p>Hosted source-owned package review for adopters. Trust signals include package docs, receipts, update and rollback status, advisory metadata, and no node_modules evidence.</p>
    </header>
    <section class="summary" aria-label="Gallery summary">
      <div class="metric"><span>Status</span><strong>{passed}</strong></div>
      <div class="metric"><span>Score</span><strong>{score}/100</strong></div>
      <div class="metric"><span>Packages</span><strong>{package_count}</strong></div>
      <div class="metric"><span>Dependency boundary</span><strong>no node_modules</strong></div>
    </section>
    <section class="panel">
      <h2>Trust signals</h2>
      <p>Advisory rows show curated placeholders or offline snapshots honestly. This is not a universal npm replacement claim and not live vulnerability coverage unless a real advisory source is attached.</p>
    </section>
    <section class="grid" aria-label="Packages">
      {packages}
    </section>
    <section class="panel" style="margin-top: 14px;">
      <h2>Migration guides</h2>
      <ul>{migrations}</ul>
    </section>
    <section class="panel" style="margin-top: 14px;">
      <h2>Migration gallery</h2>
      <p>The dedicated <code>/forge/migration-gallery/</code> route shows supported scope, manual gaps, package evidence, and payload comparison boundaries for <code>migration/static-site</code>.</p>
    </section>
  </main>
</body>
</html>
"#,
        passed = passed,
        score = report.score,
        package_count = report.package_count,
        packages = packages,
        migrations = migrations
    )
}

fn html_list(items: &[String]) -> String {
    items
        .iter()
        .map(|item| format!("<li>{}</li>", forge_package_gallery_html_escape(item)))
        .collect()
}

fn forge_package_gallery_anchor(package_id: &str) -> String {
    package_id.replace(['/', '@'], "-")
}

fn forge_package_gallery_html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub(super) fn forge_package_gallery_failure_summary(
    report: &DxForgePackageGalleryReport,
) -> String {
    if report.findings.is_empty() {
        return format!(
            "DX Forge package gallery failed with score {}",
            report.score
        );
    }
    format!(
        "DX Forge package gallery failed: {}",
        report.findings.join("; ")
    )
}
