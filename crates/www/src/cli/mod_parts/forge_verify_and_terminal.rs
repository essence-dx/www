fn write_forge_release_evidence_history(
    root: impl AsRef<Path>,
    report: &DxForgeReleaseEvidenceReport,
) -> anyhow::Result<PathBuf> {
    let history_dir = root.as_ref().join(FORGE_RELEASE_EVIDENCE_HISTORY_DIR);
    std::fs::create_dir_all(&history_dir)?;

    let snapshot_file = unique_release_evidence_snapshot_file(&history_dir, &report.generated_at);
    let snapshot_path = history_dir.join(&snapshot_file);
    let snapshot_json = serde_json::to_string_pretty(report)?;
    std::fs::write(&snapshot_path, snapshot_json)?;

    let index_path = history_dir.join("index.json");
    let mut index = read_forge_release_evidence_history_index(&index_path)?;
    let previous_check_score = index.snapshots.first().map(|snapshot| snapshot.check_score);
    let check_score_delta = previous_check_score
        .map(|score| report.check_score as i16 - score as i16)
        .unwrap_or(0);
    let registry_verified_count = report
        .registry_integrity
        .iter()
        .filter(|check| check.verified)
        .count() as u64;

    index.updated_at = Utc::now().to_rfc3339();
    index.snapshots.insert(
        0,
        DxForgeReleaseEvidenceHistoryEntry {
            generated_at: report.generated_at.clone(),
            passed: report.passed,
            check_score: report.check_score,
            previous_check_score,
            check_score_delta,
            check_traffic: report.check_traffic.clone(),
            registry_verified_count,
            registry_package_count: report.registry_integrity.len() as u64,
            package_score: report.package_scorecard.score,
            package_count: report.package_scorecard.packages.len() as u64,
            latest_benchmark_fixture_mode: report
                .latest_benchmark
                .as_ref()
                .and_then(|snapshot| snapshot.fixture_mode.clone()),
            snapshot_file,
        },
    );

    let index_json = serde_json::to_string_pretty(&index)?;
    std::fs::write(&index_path, index_json)?;
    std::fs::write(
        history_dir.join("index.md"),
        forge_release_evidence_history_markdown(&index),
    )?;

    Ok(index_path)
}

fn read_forge_release_evidence_history_index(
    path: &Path,
) -> anyhow::Result<DxForgeReleaseEvidenceHistoryIndex> {
    if !path.exists() {
        return Ok(DxForgeReleaseEvidenceHistoryIndex {
            version: 1,
            updated_at: Utc::now().to_rfc3339(),
            snapshots: Vec::new(),
        });
    }

    let bytes = std::fs::read(path)?;
    let mut index = serde_json::from_slice::<DxForgeReleaseEvidenceHistoryIndex>(&bytes)?;
    if index.version == 0 {
        index.version = 1;
    }
    Ok(index)
}

fn unique_release_evidence_snapshot_file(history_dir: &Path, generated_at: &str) -> String {
    let base = safe_release_evidence_snapshot_stem(generated_at);
    let mut candidate = format!("{base}-release-proof.json");
    let mut suffix = 1u64;
    while history_dir.join(&candidate).exists() {
        candidate = format!("{base}-release-proof-{suffix}.json");
        suffix += 1;
    }
    candidate
}

fn safe_release_evidence_snapshot_stem(value: &str) -> String {
    let stem = value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string();

    if stem.is_empty() {
        "release-proof".to_string()
    } else {
        stem
    }
}

fn build_forge_verify_package_report(
    project: &Path,
    package_id: &str,
    variant: &str,
) -> anyhow::Result<DxForgeVerifyPackageReport> {
    let canonical = canonical_package_id(package_id).to_string();
    let registry_package = registry_package(&canonical)?;
    let registry_integrity = verify_registry_package_integrity(&registry_package)?;
    let manifest = read_forge_source_manifest(project)?;
    let source_package = manifest
        .packages
        .iter()
        .find(|package| {
            canonical_package_id(&package.package_id) == canonical && package.variant == variant
        })
        .cloned()
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Forge package `{canonical}` variant `{variant}` is not tracked in `.dx/forge/source-manifest.json`"
            )
        })?;

    let docs_path = project
        .join(".dx/forge/docs")
        .join(forge_doctor_package_doc_name(&canonical, variant));
    let docs_present = std::fs::metadata(&docs_path)
        .map(|metadata| metadata.is_file() && metadata.len() > 0)
        .unwrap_or(false);

    let update = plan_forge_update_variant(&canonical, variant, project)?;
    let rollback_path = source_package
        .rollback_receipt
        .as_ref()
        .map(|receipt| project.join(".dx/forge/receipts").join(receipt));
    let rollback_present = rollback_path
        .as_ref()
        .map(|path| path.is_file())
        .unwrap_or(false);
    let scorecard = build_forge_package_scorecard_for_project(project)?;
    let scorecard_package = scorecard
        .packages
        .iter()
        .find(|package| package.package_id == canonical);
    let scorecard_present = scorecard_package
        .and_then(|package| package.project_evidence.as_ref())
        .is_some_and(|evidence| evidence.manifest_variant_count > 0);

    let registry_check = DxForgeVerifyCheck {
        name: "registry_integrity".to_string(),
        passed: true,
        traffic: DxUpdateTraffic::Green,
        score: 25,
        message: format!(
            "Registry package `{}` verified {} files with integrity `{}`.",
            registry_integrity.package_id,
            registry_integrity.verified_files,
            registry_integrity.integrity_hash
        ),
        evidence: Some(registry_integrity.integrity_hash),
    };
    let docs_check = DxForgeVerifyCheck {
        name: "docs".to_string(),
        passed: docs_present,
        traffic: if docs_present {
            DxUpdateTraffic::Green
        } else {
            DxUpdateTraffic::Red
        },
        score: if docs_present { 20 } else { 0 },
        message: if docs_present {
            "Package-facing Forge docs are present.".to_string()
        } else {
            format!("missing docs at `{}`", docs_path.display())
        },
        evidence: Some(docs_path.display().to_string()),
    };
    let update_check = DxForgeVerifyCheck {
        name: "update".to_string(),
        passed: update.traffic != DxUpdateTraffic::Red,
        traffic: update.traffic,
        score: match update.traffic {
            DxUpdateTraffic::Green => 20,
            DxUpdateTraffic::Yellow => 12,
            DxUpdateTraffic::Red => 0,
        },
        message: format!(
            "Update preview is `{}` with {} file decisions.",
            update.traffic.as_str(),
            update.files.len()
        ),
        evidence: Some(format!(
            "{} -> {}",
            update.current_version, update.latest_version
        )),
    };
    let rollback_check = DxForgeVerifyCheck {
        name: "rollback".to_string(),
        passed: rollback_present || source_package.last_accepted_update.is_none(),
        traffic: if rollback_present {
            DxUpdateTraffic::Green
        } else {
            DxUpdateTraffic::Yellow
        },
        score: if rollback_present {
            15
        } else if source_package.last_accepted_update.is_none() {
            8
        } else {
            0
        },
        message: if rollback_present {
            "Rollback receipt is present.".to_string()
        } else if source_package.last_accepted_update.is_none() {
            "No accepted update has been recorded yet, so rollback coverage is not mandatory for this fresh package.".to_string()
        } else {
            "Rollback receipt is missing for a package with accepted update history.".to_string()
        },
        evidence: rollback_path.map(|path| path.display().to_string()),
    };
    let scorecard_check = DxForgeVerifyCheck {
        name: "scorecard".to_string(),
        passed: scorecard_present,
        traffic: if scorecard_present {
            DxUpdateTraffic::Green
        } else {
            DxUpdateTraffic::Red
        },
        score: if scorecard_present { 20 } else { 0 },
        message: if scorecard_present {
            format!(
                "Project scorecard includes `{}` with local package evidence.",
                canonical
            )
        } else {
            format!("Project scorecard does not include local evidence for `{canonical}`.")
        },
        evidence: Some(format!("scorecard-score={}", scorecard.score)),
    };
    let package_specific_checks = build_forge_package_specific_verify_checks(
        project,
        &canonical,
        &source_package,
        &docs_path,
        &manifest,
    );

    let base_score = registry_check
        .score
        .saturating_add(docs_check.score)
        .saturating_add(update_check.score)
        .saturating_add(rollback_check.score)
        .saturating_add(scorecard_check.score)
        .min(100);
    let specific_score = package_specific_checks
        .iter()
        .fold(0u8, |score, check| score.saturating_add(check.score))
        .min(100);
    let score = if package_specific_checks.is_empty() {
        base_score
    } else {
        ((u16::from(base_score) + u16::from(specific_score)) / 2) as u8
    };
    let passed = registry_check.passed
        && docs_check.passed
        && update_check.passed
        && scorecard_check.passed
        && package_specific_checks.iter().all(|check| check.passed);

    Ok(DxForgeVerifyPackageReport {
        project: project.to_path_buf(),
        package_id: canonical,
        variant: variant.to_string(),
        score,
        passed,
        registry_integrity: registry_check,
        docs: docs_check,
        update: update_check,
        rollback: rollback_check,
        scorecard: scorecard_check,
        package_specific_checks,
    })
}

fn build_forge_package_specific_verify_checks(
    project: &Path,
    canonical: &str,
    source_package: &DxSourcePackage,
    docs_path: &Path,
    manifest: &DxSourceManifest,
) -> Vec<DxForgeVerifyCheck> {
    match canonical {
        "migration/static-site" => build_forge_migration_static_site_verify_checks(
            project,
            source_package,
            docs_path,
            manifest,
        ),
        _ => Vec::new(),
    }
}

fn build_forge_migration_static_site_verify_checks(
    project: &Path,
    source_package: &DxSourcePackage,
    docs_path: &Path,
    manifest: &DxSourceManifest,
) -> Vec<DxForgeVerifyCheck> {
    let readme_path = project.join("migrations/static-site/README.md");
    let content_path = project.join("migrations/static-site/content.ts");
    let page_path = project.join("migrations/static-site/page.tsx");
    let sample_path = project.join("migrations/static-site/sample-wordpress-export.json");
    let expected_files = [&readme_path, &content_path, &page_path, &sample_path];
    let docs_text = std::fs::read_to_string(docs_path).unwrap_or_default();
    let readme_text = std::fs::read_to_string(&readme_path).unwrap_or_default();
    let content_text = std::fs::read_to_string(&content_path).unwrap_or_default();
    let page_text = std::fs::read_to_string(&page_path).unwrap_or_default();
    let sample_text = std::fs::read_to_string(&sample_path).unwrap_or_default();
    let receipts =
        forge_migration_package_receipts(project, Some(manifest), "migration/static-site");
    let receipt_ready = receipts
        .iter()
        .any(|receipt| migration_static_site_receipt_is_reviewable(receipt));
    let missing_files = expected_files
        .iter()
        .filter(|path| !path.is_file())
        .map(|path| {
            path.strip_prefix(project)
                .unwrap_or(path)
                .display()
                .to_string()
        })
        .collect::<Vec<_>>();
    let manifest_file_paths = source_package
        .files
        .iter()
        .map(|file| file.path.as_str())
        .collect::<HashSet<_>>();
    let manifest_tracks_expected_files = [
        "migrations/static-site/content.ts",
        "migrations/static-site/page.tsx",
        "migrations/static-site/sample-wordpress-export.json",
        "migrations/static-site/README.md",
    ]
    .iter()
    .all(|path| manifest_file_paths.contains(path));
    let source_files_ready = missing_files.is_empty() && manifest_tracks_expected_files;
    let docs_ready = docs_path.is_file()
        && docs_text.contains("Static Migration Contract")
        && docs_text.contains("not migrate a whole WordPress site")
        && readme_text.contains("not a full WordPress plugin or theme migration")
        && readme_text.contains("No package install is required");
    let asset_mapping_ready = content_text.contains("assets:")
        && content_text.contains("target:")
        && content_text.contains("Copy the original asset")
        && sample_text.contains("\"assets\"")
        && sample_text.contains("\"target\"")
        && readme_text.contains("Media files should be copied");
    let manual_review_ready = page_text.contains("Migration review required")
        && content_text.contains("warnings:")
        && sample_text.contains("manual_review_required")
        && readme_text.contains("Imported HTML must be reviewed");
    let no_node_modules_ready = !project.join("node_modules").exists()
        && !project.join("migrations/static-site/node_modules").exists();

    vec![
        forge_verify_check(
            "migration-static-site-docs",
            docs_ready,
            15,
            if docs_ready {
                "Forge migration docs and package README state the honest static-only scope."
                    .to_string()
            } else {
                "Forge migration docs or package README are missing required static-only scope language."
                    .to_string()
            },
            format!("{}, {}", docs_path.display(), readme_path.display()),
        ),
        forge_verify_check(
            "migration-static-site-receipts",
            receipt_ready,
            20,
            if receipt_ready {
                "Forge receipts include migration/static-site package metadata, file maps, and no-lifecycle policy evidence."
                    .to_string()
            } else {
                "No reviewable migration/static-site receipt with package metadata, file maps, and no-lifecycle policy evidence was found."
                    .to_string()
            },
            receipts
                .last()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| ".dx/forge/receipts/*-migration-static-site.json".to_string()),
        ),
        forge_verify_check(
            "migration-static-site-asset-mapping",
            asset_mapping_ready,
            20,
            if asset_mapping_ready {
                "The migration seed includes asset source/target mapping notes in source and fixture files."
                    .to_string()
            } else {
                "The migration seed is missing asset source/target mapping notes in source or fixture files."
                    .to_string()
            },
            "migrations/static-site/content.ts; migrations/static-site/sample-wordpress-export.json",
        ),
        forge_verify_check(
            "migration-static-site-manual-review",
            manual_review_ready,
            20,
            if manual_review_ready {
                "Manual-review warnings are visible in the rendered component, typed content, fixture, and README."
                    .to_string()
            } else {
                "Manual-review warnings are missing from the rendered component, typed content, fixture, or README."
                    .to_string()
            },
            "migrations/static-site/page.tsx; migrations/static-site/README.md",
        ),
        forge_verify_check(
            "migration-static-site-source-files",
            source_files_ready,
            15,
            if source_files_ready {
                "All migration/static-site source files exist and are tracked by the Forge source manifest.".to_string()
            } else if missing_files.is_empty() {
                "Migration/static-site files exist, but the source manifest does not track every expected file.".to_string()
            } else {
                format!(
                    "Missing migration/static-site source files: {}",
                    missing_files.join(", ")
                )
            },
            ".dx/forge/source-manifest.json",
        ),
        forge_verify_check(
            "migration-static-site-no-node-modules",
            no_node_modules_ready,
            10,
            if no_node_modules_ready {
                "The migration package verification path did not create a node_modules directory."
                    .to_string()
            } else {
                "A node_modules directory exists in the project or migration package path."
                    .to_string()
            },
            "node_modules; migrations/static-site/node_modules",
        ),
    ]
}

fn migration_static_site_receipt_is_reviewable(receipt: &Path) -> bool {
    let Ok(bytes) = std::fs::read(receipt) else {
        return false;
    };
    let Ok(value) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
        return false;
    };
    let package_matches = value
        .pointer("/package/package_id")
        .and_then(serde_json::Value::as_str)
        == Some("migration/static-site");
    let has_content_file = json_array_contains_path(
        value.get("files_written"),
        "migrations/static-site/content.ts",
        "path",
    );
    let has_page_map = json_array_contains_path(
        value.get("file_map"),
        "migrations/static-site/page.tsx",
        "materialized_path",
    );
    let has_no_lifecycle_policy = value
        .get("policy_decisions")
        .and_then(serde_json::Value::as_array)
        .is_some_and(|items| {
            items.iter().any(|item| {
                item.get("policy").and_then(serde_json::Value::as_str)
                    == Some("no-lifecycle-execution")
            })
        });

    package_matches && has_content_file && has_page_map && has_no_lifecycle_policy
}

fn json_array_contains_path(
    value: Option<&serde_json::Value>,
    expected: &str,
    field: &str,
) -> bool {
    value
        .and_then(serde_json::Value::as_array)
        .is_some_and(|items| {
            items.iter().any(|item| {
                item.get(field)
                    .and_then(serde_json::Value::as_str)
                    .is_some_and(|path| path == expected)
            })
        })
}

fn forge_verify_check(
    name: &str,
    passed: bool,
    score: u8,
    passed_message: impl Into<String>,
    evidence: impl Into<String>,
) -> DxForgeVerifyCheck {
    DxForgeVerifyCheck {
        name: name.to_string(),
        passed,
        traffic: if passed {
            DxUpdateTraffic::Green
        } else {
            DxUpdateTraffic::Red
        },
        score: if passed { score } else { 0 },
        message: passed_message.into(),
        evidence: Some(evidence.into()),
    }
}

fn build_forge_verify_all_packages_report(
    project: &Path,
) -> anyhow::Result<DxForgeVerifyAllPackagesReport> {
    let manifest = read_optional_forge_source_manifest(project)?;
    let mut materialized: HashMap<String, Vec<String>> = HashMap::new();
    if let Some(manifest) = &manifest {
        for package in &manifest.packages {
            if package.source_kind == DxSourceKind::Local {
                continue;
            }
            materialized
                .entry(canonical_package_id(&package.package_id).to_string())
                .or_default()
                .push(package.variant.clone());
        }
    }

    let mut targets = Vec::<(String, String)>::new();
    let mut seen = HashSet::<(String, String)>::new();
    if materialized.is_empty() {
        for package_id in FORGE_WWW_TEMPLATE_PACKAGE_IDS {
            let canonical = canonical_package_id(package_id).to_string();
            if seen.insert((canonical.clone(), "default".to_string())) {
                targets.push((canonical, "default".to_string()));
            }
        }
    }
    for (package_id, variants) in &materialized {
        for variant in variants {
            if seen.insert((package_id.clone(), variant.clone())) {
                targets.push((package_id.clone(), variant.clone()));
            }
        }
    }

    let mut packages = Vec::new();
    let mut missing_packages = Vec::new();
    for (package_id, variant) in targets {
        match build_forge_verify_package_report(project, &package_id, &variant) {
            Ok(report) => packages.push(report),
            Err(error) => missing_packages.push(DxForgeVerifyMissingPackage {
                package_id,
                variant,
                message: error.to_string(),
            }),
        }
    }

    let total = packages.len() + missing_packages.len();
    let score = if total == 0 {
        0
    } else {
        (packages
            .iter()
            .map(|package| package.score as u64)
            .sum::<u64>()
            / total as u64) as u8
    };
    let passed =
        !packages.is_empty() && missing_packages.is_empty() && packages.iter().all(|p| p.passed);

    Ok(DxForgeVerifyAllPackagesReport {
        project: project.to_path_buf(),
        generated_at: Utc::now().to_rfc3339(),
        score,
        passed,
        packages,
        missing_packages,
    })
}

fn load_latest_benchmark_snapshot(
    benchmark_history_path: &Path,
) -> anyhow::Result<Option<DxForgeBenchmarkSnapshot>> {
    Ok(load_benchmark_snapshots(benchmark_history_path)?
        .into_iter()
        .next())
}

fn load_latest_forge_route_benchmark_snapshot(
    benchmark_history_path: &Path,
) -> anyhow::Result<Option<DxForgeBenchmarkSnapshot>> {
    Ok(load_benchmark_snapshots(benchmark_history_path)?
        .into_iter()
        .find(|snapshot| snapshot.fixture_mode.as_deref() == Some("forge-site")))
}

fn load_benchmark_snapshots(
    benchmark_history_path: &Path,
) -> anyhow::Result<Vec<DxForgeBenchmarkSnapshot>> {
    if !benchmark_history_path.exists() {
        return Ok(Vec::new());
    }
    let index: DxForgeBenchmarkHistoryIndex =
        serde_json::from_slice(&std::fs::read(benchmark_history_path)?)?;
    Ok(index.snapshots)
}

fn read_forge_source_manifest(project: &Path) -> anyhow::Result<DxSourceManifest> {
    let manifest_path = project.join(".dx/forge/source-manifest.json");
    let bytes = std::fs::read(&manifest_path)
        .map_err(|error| anyhow::anyhow!("read `{}`: {error}", manifest_path.display()))?;
    serde_json::from_slice::<DxSourceManifest>(&bytes)
        .map_err(|error| anyhow::anyhow!("parse `{}`: {error}", manifest_path.display()))
}

fn read_optional_forge_source_manifest(project: &Path) -> anyhow::Result<Option<DxSourceManifest>> {
    let manifest_path = project.join(".dx/forge/source-manifest.json");
    match std::fs::read(&manifest_path) {
        Ok(bytes) => serde_json::from_slice::<DxSourceManifest>(&bytes)
            .map(Some)
            .map_err(|error| anyhow::anyhow!("parse `{}`: {error}", manifest_path.display())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(anyhow::anyhow!(
            "read `{}`: {error}",
            manifest_path.display()
        )),
    }
}

fn check_section_metric(section: &dx_compiler::ecosystem::DxCheckSection, name: &str) -> u64 {
    section
        .metrics
        .iter()
        .find(|metric| metric.name == name)
        .map(|metric| metric.value)
        .unwrap_or(0)
}

fn ensure_r2_remote(remote: Option<&str>, command: &str) -> DxResult<()> {
    match remote {
        Some("r2") => Ok(()),
        Some(other) => Err(DxError::ConfigValidationError {
            message: format!("dx forge registry {command} supports only --remote r2, got {other}"),
            field: Some("remote".to_string()),
        }),
        None => Err(DxError::ConfigValidationError {
            message: format!("dx forge registry {command} requires --remote r2"),
            field: Some("remote".to_string()),
        }),
    }
}

fn block_on_registry<F, T>(future: F) -> DxResult<T>
where
    F: std::future::Future<Output = anyhow::Result<T>>,
{
    let runtime = tokio::runtime::Runtime::new().map_err(|error| DxError::InternalError {
        message: format!("create Tokio runtime for Forge registry: {error}"),
    })?;
    runtime.block_on(future).map_err(forge_error)
}

fn forge_release_notes_markdown(report: &DxForgeReleaseNotesReport) -> String {
    let readiness = &report.ci_readiness;
    let scorecard = &report.package_scorecard;
    let mut output = format!(
        "# DX Forge Release Notes\n\n- Project: `{}`\n- Generated: `{}`\n- Release readiness: `{}` (`{}` / `100`)\n- Passed: `{}`\n- No `node_modules`: `{}`\n\n",
        report.project.display(),
        report.generated_at,
        report.status,
        report.score,
        report.passed,
        readiness.no_node_modules
    );

    output.push_str("## CI Readiness\n\n");
    output.push_str(&format!(
        "- Smoke: `{}` (`{}` / `100`)\n- Release evidence: `{}` (`{}` / `100`)\n- Launch page quality: `{}` / `100`\n- Doctor: `{}`\n- Verify packages: `{}`\n\n",
        readiness.smoke_passed,
        readiness.smoke_score,
        readiness.release_evidence_passed,
        readiness.release_evidence_score,
        readiness.launch_page_quality_score,
        readiness.doctor_passed,
        readiness.verify_passed
    ));

    output.push_str("## Package Scorecard\n\n");
    output.push_str(&format!(
        "- Score: `{}` / `100`\n- Packages: `{}`\n- Verified packages: `{}`\n- Source-owned packages: `{}`\n- `node_modules` packages: `{}`\n\n",
        scorecard.score,
        scorecard.package_count,
        scorecard.verified_packages,
        scorecard.source_owned_packages,
        scorecard.node_modules_packages
    ));
    output.push_str("| Package | Version | Files | Verified | Source-owned | Scripts blocked | node_modules |\n");
    output.push_str("| --- | --- | ---: | --- | --- | --- | --- |\n");
    for package in &scorecard.packages {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            package.package_id,
            package.version,
            package.file_count,
            package.integrity_verified,
            package.source_owned,
            package.install_scripts_blocked,
            package.node_modules_created
        ));
    }

    output.push_str("\n## Route Measurements\n\n");
    output.push_str(&format!(
        "- Benchmark history: `{}`\n",
        report.route_measurements.benchmark_history_path.display()
    ));
    if let Some(snapshot) = &report.route_measurements.latest_forge_route_benchmark {
        output.push_str(&format!(
            "- Fixture: `{}`\n- Delivery: `{}`\n- decoded bytes: `{}`\n- Brotli bytes: `{}`\n- HTTP median: `{}` ms\n- Chrome load event: `{}` ms\n\n",
            optional_string(snapshot.fixture_mode.as_deref()),
            optional_string(snapshot.route_delivery.as_deref()),
            optional_u64(snapshot.decoded_bytes),
            optional_u64(snapshot.brotli_bytes),
            optional_f64(snapshot.http_route_median_ms),
            optional_f64(snapshot.chrome_load_event_ms)
        ));
    } else {
        output.push_str("- Latest /forge benchmark: `missing`\n\n");
    }

    output.push_str("## Honest Launch Limitations\n\n");
    for limitation in &report.honest_launch_limitations {
        output.push_str(&format!("- {limitation}\n"));
    }

    output.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- No release-blocking findings for the configured threshold.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }

    output
}

fn forge_scorecard_cli_markdown(
    report: &DxForgePackageScorecardReport,
    latest_forge_route_benchmark: Option<&DxForgeBenchmarkSnapshot>,
) -> String {
    let mut output = forge_package_scorecard_markdown(report);
    output.push_str("\n## Latest /forge Payload And Browser Timing\n\n");
    if let Some(snapshot) = latest_forge_route_benchmark {
        output.push_str(&forge_benchmark_snapshot_markdown(snapshot));
    } else {
        output.push_str("- No `/forge` benchmark snapshot was found.\n");
    }
    output
}

fn forge_init_app_terminal(report: &DxForgeInitAppReport) -> String {
    let mut output = format!(
        "DX Forge init app\nProject: {}\nGenerated: {}\nMode: {}\nPassed: {}\nScore: {} / 100\nNo node_modules: {}\nPackages: {}\nPlanned files: {}\n",
        report.project.display(),
        report.generated_at,
        report.mode,
        report.passed,
        report.score,
        report.no_node_modules,
        report.package_ids.len(),
        report.planned_files.len()
    );
    if let Some(score) = report.check_score {
        output.push_str(&format!(
            "DX check: {} / 100 ({})\n",
            score,
            report.check_traffic.as_deref().unwrap_or("n/a")
        ));
        output.push_str(&format!(
            "Strict Forge gate: {}\n",
            report.strict_forge_passed
        ));
    }
    output.push_str("\nPackages:\n");
    for package in &report.packages {
        output.push_str(&format!(
            "- {}@{}: {} files, score {}, wrote {}\n",
            package.package_id,
            package.variant,
            package.files,
            package.risk_score,
            package.wrote_files
        ));
    }
    output.push_str("\nNext commands:\n");
    for command in &report.next_commands {
        output.push_str(&format!("- {command}\n"));
    }
    if !report.findings.is_empty() {
        output.push_str("\nFindings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

fn forge_init_app_markdown(report: &DxForgeInitAppReport) -> String {
    let mut output = format!(
        "# DX Forge Init App\n\n- Project: `{}`\n- Generated: `{}`\n- Mode: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- No `node_modules`: `{}`\n- Launch packages: `{}`\n- Planned files: `{}`\n\n",
        report.project.display(),
        report.generated_at,
        report.mode,
        report.passed,
        report.score,
        report.no_node_modules,
        report.package_ids.len(),
        report.planned_files.len()
    );

    output.push_str("## Packages\n\n");
    output.push_str("| Package | Variant | Files | Score | Wrote |\n");
    output.push_str("| --- | --- | ---: | ---: | --- |\n");
    for package in &report.packages {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            package.package_id,
            package.variant,
            package.files,
            package.risk_score,
            package.wrote_files
        ));
    }

    output.push_str("\n## Artifacts\n\n");
    output.push_str(&format!(
        "- Source manifest: `{}`\n",
        report.source_manifest_path.display()
    ));
    if let Some(path) = &report.scorecard_report_path {
        output.push_str(&format!("- Scorecard: `{}`\n", path.display()));
    }
    if let Some(path) = &report.dx_check_report_path {
        output.push_str(&format!("- Strict check: `{}`\n", path.display()));
    }
    if let Some(score) = report.check_score {
        output.push_str(&format!(
            "- DX check: `{}` / `100` (`{}`)\n- Strict Forge gate: `{}`\n",
            score,
            report.check_traffic.as_deref().unwrap_or("n/a"),
            report.strict_forge_passed
        ));
    }

    output.push_str("\n## Next Commands\n\n");
    for command in &report.next_commands {
        output.push_str(&format!("- `{command}`\n"));
    }

    output.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- `pass`: Forge init-app is beta-ready.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }

    output
}

fn forge_init_app_failure_summary(report: &DxForgeInitAppReport) -> String {
    if report.findings.is_empty() {
        return format!("Forge init-app failed with score {}", report.score);
    }
    format!("Forge init-app failed: {}", report.findings.join("; "))
}

fn forge_release_evidence_history_markdown(index: &DxForgeReleaseEvidenceHistoryIndex) -> String {
    let mut output = format!(
        "# DX Forge Release Proof History\n\n- Updated: `{}`\n- Snapshots: `{}`\n\n",
        index.updated_at,
        index.snapshots.len()
    );
    output.push_str("| Generated | Passed | Check | Delta | Registry | Package score | Packages | Benchmark | Snapshot |\n");
    output.push_str("| --- | --- | ---: | ---: | --- | ---: | ---: | --- | --- |\n");
    for snapshot in &index.snapshots {
        output.push_str(&format!(
            "| `{}` | {} | {} | {:+} | {}/{} | {} | {} | {} | `{}` |\n",
            snapshot.generated_at,
            if snapshot.passed { "yes" } else { "no" },
            snapshot.check_score,
            snapshot.check_score_delta,
            snapshot.registry_verified_count,
            snapshot.registry_package_count,
            snapshot.package_score,
            snapshot.package_count,
            snapshot
                .latest_benchmark_fixture_mode
                .as_deref()
                .unwrap_or("missing"),
            snapshot.snapshot_file
        ));
    }
    output
}

fn forge_adoption_smoke_terminal(report: &DxForgeAdoptionSmokeReport) -> String {
    let mut output = format!(
        "DX Forge adoption smoke\nProject: {}\nGenerated: {}\nPassed: {}\nScore: {}\nNo node_modules: {}\nPackages: {}\nRoutes: {}\nRelease bundle: {} / {}\n",
        report.project.display(),
        report.generated_at,
        report.passed,
        report.score,
        report.no_node_modules,
        report.package_count,
        report.route_count,
        report.release_bundle_score,
        report.fail_under
    );
    output.push_str("\nPublic routes:\n");
    for route in &report.routes {
        output.push_str(&format!(
            "- {}: {} ({})\n",
            route.route,
            route.exists,
            route.html_path.display()
        ));
    }
    output.push_str("\nArtifacts:\n");
    output.push_str(&format!(
        "- Smoke: {}\n- Release bundle: {}\n- Manifest: {}\n- Public: {}\n",
        report.smoke_report_path.display(),
        report.release_bundle_dir.display(),
        report.release_bundle_manifest_path.display(),
        report.public_dir.display()
    ));
    if !report.findings.is_empty() {
        output.push_str("\nFindings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

fn forge_adoption_smoke_markdown(report: &DxForgeAdoptionSmokeReport) -> String {
    let mut output = format!(
        "# DX Forge Adoption Smoke\n\n- Project: `{}`\n- Generated: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- No `node_modules`: `{}`\n- Packages: `{}`\n- Public routes: `{}`\n\n",
        report.project.display(),
        report.generated_at,
        report.passed,
        report.score,
        report.fail_under,
        report.no_node_modules,
        report.package_count,
        report.route_count
    );

    output.push_str("## Routes\n\n");
    output.push_str("| Route | Exists | HTML | Proof |\n");
    output.push_str("| --- | --- | --- | --- |\n");
    for route in &report.routes {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` |\n",
            route.route,
            route.exists,
            route.html_path.display(),
            route.proof_path.display()
        ));
    }

    output.push_str("\n## Artifacts\n\n");
    output.push_str(&format!(
        "- Smoke report: `{}`\n- Adoption artifacts: `{}`\n- Release bundle: `{}`\n- Release manifest: `{}`\n- Source manifest: `{}`\n- Route comparison: `{}`\n- Release history: `{}`\n\n",
        report.smoke_report_path.display(),
        report.adoption_artifacts_dir.display(),
        report.release_bundle_dir.display(),
        report.release_bundle_manifest_path.display(),
        report.source_manifest_path.display(),
        report.route_comparison_path.display(),
        report.release_history_path.display()
    ));

    output.push_str("## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- `pass`: no adoption-smoke findings.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }

    output
}

fn forge_adoption_report_terminal(report: &DxForgeAdoptionReport) -> String {
    let mut output = format!(
        "DX Forge adoption report\nProject: {}\nGenerated: {}\nPassed: {}\nScore: {} / 100\nNo node_modules: {}\nPackages: {} (docs {}/{}, receipts {})\nDX check: {} / 100 ({}, release gate {}, strict Forge {})\nRelease bundle: {} / 100 ({})\n",
        report.project.display(),
        report.generated_at,
        report.passed,
        report.score,
        report.no_node_modules,
        report.package_count,
        report.package_docs_present,
        report.package_count,
        report.receipt_count,
        report.dx_check.score,
        report.dx_check.traffic,
        report.dx_check.release_gate_score,
        report.dx_check.strict_forge_passed,
        report.release_bundle.score,
        report.release_bundle.passed
    );
    output.push_str("\nProject structure:\n");
    output.push_str(&format!(
        "- dx: {}\n- pages: {}\n- components: {}\n- adoption route: {} ({})\n",
        report.project_structure.dx_config_exists,
        report.project_structure.pages_dir_exists,
        report.project_structure.components_dir_exists,
        report.project_structure.app_route_exists,
        report.project_structure.app_route_path.display()
    ));
    output.push_str("\nPackages:\n");
    for package in &report.packages {
        output.push_str(&format!(
            "- {}@{}: {} files, docs {}\n",
            package.package_id, package.version, package.file_count, package.docs_exists
        ));
    }
    output.push_str("\nPublic routes:\n");
    for route in &report.public_routes {
        output.push_str(&format!(
            "- {}: {} ({})\n",
            route.route,
            route.passed,
            route.html_path.display()
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

fn forge_adoption_report_markdown(report: &DxForgeAdoptionReport) -> String {
    let mut output = format!(
        "# DX Forge Adoption Report\n\n- Project: `{}`\n- Generated: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- No `node_modules`: `{}`\n- Packages: `{}`\n- Receipts: `{}`\n- Package docs: `{}` present / `{}` missing\n- DX check: `{}` / `100` (`{}`)\n- Release gate: `{}` / `100`\n- Strict Forge gate: `{}`\n- Release bundle: `{}` / `100` (`{}`)\n\n",
        report.project.display(),
        report.generated_at,
        report.passed,
        report.score,
        report.fail_under,
        report.no_node_modules,
        report.package_count,
        report.receipt_count,
        report.package_docs_present,
        report.package_docs_missing,
        report.dx_check.score,
        report.dx_check.traffic,
        report.dx_check.release_gate_score,
        report.dx_check.strict_forge_passed,
        report.release_bundle.score,
        report.release_bundle.passed
    );

    output.push_str("## Project Structure\n\n");
    output.push_str("| Item | Exists | Path |\n");
    output.push_str("| --- | --- | --- |\n");
    output.push_str(&format!(
        "| `dx` | `{}` | `{}` |\n",
        report.project_structure.dx_config_exists,
        report.project_structure.dx_config_path.display()
    ));
    output.push_str(&format!(
        "| `pages` | `{}` | `{}` |\n",
        report.project_structure.pages_dir_exists,
        report.project_structure.pages_dir.display()
    ));
    output.push_str(&format!(
        "| `components` | `{}` | `{}` |\n",
        report.project_structure.components_dir_exists,
        report.project_structure.components_dir.display()
    ));
    output.push_str(&format!(
        "| `pages/forge-adoption.html` | `{}` | `{}` |\n",
        report.project_structure.app_route_exists,
        report.project_structure.app_route_path.display()
    ));

    output.push_str("\n## Packages\n\n");
    output.push_str("| Package | Variant | Version | Files | Docs |\n");
    output.push_str("| --- | --- | --- | ---: | --- |\n");
    for package in &report.packages {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            package.package_id,
            package.variant,
            package.version,
            package.file_count,
            package.docs_path.display()
        ));
    }

    output.push_str("\n## Public Routes\n\n");
    output.push_str("| Route | Passed | HTML | Packet | Proof |\n");
    output.push_str("| --- | --- | --- | --- | --- |\n");
    for route in &report.public_routes {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            route.route,
            route.passed,
            route.html_path.display(),
            route.packet_path.display(),
            route.proof_path.display()
        ));
    }

    output.push_str("\n## Evidence Paths\n\n");
    output.push_str(&format!(
        "- Source manifest: `{}`\n- Receipts: `{}`\n- Package docs: `{}`\n- Public dir: `{}`\n- Release bundle: `{}`\n\n",
        report.source_manifest_path.display(),
        report.receipt_dir.display(),
        report.package_docs_dir.display(),
        report.public_dir.display(),
        report.release_bundle.bundle_dir.display()
    ));

    output.push_str("## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- `pass`: no adoption-report findings.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }

    output.push_str("\n## Honest Scope\n\n");
    for boundary in &report.honest_scope {
        output.push_str(&format!("- {boundary}\n"));
    }

    output
}

fn forge_adoption_report_failure_summary(report: &DxForgeAdoptionReport) -> String {
    if report.findings.is_empty() {
        return format!(
            "Forge adoption report did not pass: score {} / 100, required {} / 100",
            report.score, report.fail_under
        );
    }
    format!(
        "Forge adoption report did not pass: {}",
        report.findings.join("; ")
    )
}

fn forge_adoption_smoke_failure_summary(report: &DxForgeAdoptionSmokeReport) -> String {
    if report.findings.is_empty() {
        return format!(
            "Forge adoption smoke did not pass: score {} / 100, required {} / 100",
            report.score, report.fail_under
        );
    }
    format!(
        "Forge adoption smoke did not pass: {}",
        report.findings.join("; ")
    )
}

fn forge_smoke_terminal(report: &DxForgeSmokeReport) -> String {
    let mut output = format!(
        "DX Forge smoke\nProject: {}\nGenerated: {}\nPassed: {}\nScore: {}\nNo node_modules: {}\nRelease gate: {}\nDX check: {} / {}\nDoctor: {} / {}\nVerify packages: {} / {}\nScorecard: {}\nLaunch page quality: {} / {}\n",
        report.project.display(),
        report.generated_at,
        report.passed,
        report.score,
        report.no_node_modules,
        report.release_gate_score,
        report.check_score,
        report.check_traffic,
        report.doctor_score,
        report.doctor_passed,
        report.verify_score,
        report.verify_passed,
        report.scorecard_score,
        report.launch_page_quality.score,
        report.launch_page_quality.passed
    );

    output.push_str("\nPackages:\n");
    for package in &report.packages {
        output.push_str(&format!(
            "- {} variant={} files={} risk={} receipt={}\n",
            package.package_id,
            package.variant,
            package.files_written,
            package.risk_score,
            package
                .receipt_path
                .as_ref()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "-".to_string())
        ));
    }

    output.push_str("\nLaunch artifacts:\n");
    for (label, path) in forge_smoke_artifact_paths(&report.launch_artifacts) {
        output.push_str(&format!("- {label}: {}\n", path.display()));
    }

    output.push_str("\nLaunch page quality:\n");
    for (label, check) in forge_launch_page_quality_checks(&report.launch_page_quality) {
        output.push_str(&format!(
            "- {label}: passed={} evidence={}\n",
            check.passed, check.evidence
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

fn forge_smoke_markdown(report: &DxForgeSmokeReport) -> String {
    let mut output = format!(
        "# DX Forge Smoke\n\n- Project: `{}`\n- Generated: `{}`\n- Passed: `{}`\n- Score: `{}`\n- No `node_modules`: `{}`\n- Release gate: `{}`\n- DX check: `{}` / `{}`\n- Doctor: `{}` / `{}`\n- Verify packages: `{}` / `{}`\n- Scorecard: `{}`\n- Launch page quality: `{}` / `{}`\n\n",
        report.project.display(),
        report.generated_at,
        report.passed,
        report.score,
        report.no_node_modules,
        report.release_gate_score,
        report.check_score,
        report.check_traffic,
        report.doctor_score,
        report.doctor_passed,
        report.verify_score,
        report.verify_passed,
        report.scorecard_score,
        report.launch_page_quality.score,
        report.launch_page_quality.passed
    );

    output.push_str("## Packages\n\n");
    output.push_str("| Package | Variant | Files | Risk | Receipt |\n");
    output.push_str("| --- | --- | ---: | ---: | --- |\n");
    for package in &report.packages {
        output.push_str(&format!(
            "| `{}` | `{}` | {} | {} | `{}` |\n",
            package.package_id,
            package.variant,
            package.files_written,
            package.risk_score,
            package
                .receipt_path
                .as_ref()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "-".to_string())
        ));
    }

    output.push_str("\n## Launch Artifacts\n\n");
    output.push_str("| Artifact | Path |\n");
    output.push_str("| --- | --- |\n");
    for (label, path) in forge_smoke_artifact_paths(&report.launch_artifacts) {
        output.push_str(&format!("| {label} | `{}` |\n", path.display()));
    }

    output.push_str("\n## Launch Page Quality\n\n");
    output.push_str("| Check | Passed | Evidence |\n");
    output.push_str("| --- | --- | --- |\n");
    for (label, check) in forge_launch_page_quality_checks(&report.launch_page_quality) {
        output.push_str(&format!(
            "| `{label}` | `{}` | `{}` |\n",
            check.passed,
            markdown_table_cell(&check.evidence)
        ));
    }

    output.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- `pass`: no smoke findings.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }

    output
}

fn forge_smoke_failure_summary(report: &DxForgeSmokeReport) -> String {
    if report.findings.is_empty() {
        format!("Forge smoke failed with score {}", report.score)
    } else {
        report.findings.join("; ")
    }
}

fn forge_readiness_badge_failure_summary(report: &DxForgeReadinessBadge) -> String {
    if report.findings.is_empty() {
        format!("Forge readiness badge failed with score {}", report.score)
    } else {
        report.findings.join("; ")
    }
}

fn print_forge_verify_package_terminal(report: &DxForgeVerifyPackageReport) {
    println!("DX Forge verify-package");
    println!("Project: {}", report.project.display());
    println!("Package: {}", report.package_id);
    println!("Variant: {}", report.variant);
    println!("Passed: {}", report.passed);
    println!("Score: {}", report.score);
    println!();
    println!("Checks:");
    for check in forge_verify_package_checks(report) {
        println!(
            "- {} pass={} score={} traffic={} evidence={}",
            check.name,
            check.passed,
            check.score,
            check.traffic.as_str(),
            check.evidence.as_deref().unwrap_or("-")
        );
        println!("  {}", check.message);
    }
}

fn print_forge_verify_all_packages_terminal(report: &DxForgeVerifyAllPackagesReport) {
    println!("DX Forge verify-package --all");
    println!("Project: {}", report.project.display());
    println!("Generated: {}", report.generated_at);
    println!("Passed: {}", report.passed);
    println!("Score: {}", report.score);
    println!("Packages: {}", report.packages.len());
    println!("Missing: {}", report.missing_packages.len());
    println!();
    for package in &report.packages {
        println!(
            "- {} variant={} passed={} score={}",
            package.package_id, package.variant, package.passed, package.score
        );
    }
    for package in &report.missing_packages {
        println!(
            "- {} variant={} missing: {}",
            package.package_id, package.variant, package.message
        );
    }
}

fn forge_verify_package_markdown(report: &DxForgeVerifyPackageReport) -> String {
    let mut output = format!(
        "# DX Forge Verify Package\n\n- Project: `{}`\n- Package: `{}`\n- Variant: `{}`\n- Passed: `{}`\n- Score: `{}`\n\n",
        report.project.display(),
        report.package_id,
        report.variant,
        report.passed,
        report.score
    );

    output.push_str("| Check | Passed | Score | Traffic | Evidence | Message |\n");
    output.push_str("| --- | --- | ---: | --- | --- | --- |\n");
    for check in forge_verify_package_checks(report) {
        output.push_str(&format!(
            "| `{}` | `{}` | {} | `{}` | `{}` | {} |\n",
            check.name,
            check.passed,
            check.score,
            check.traffic.as_str(),
            markdown_table_cell(check.evidence.as_deref().unwrap_or("-")),
            markdown_table_cell(&check.message)
        ));
    }

    output
}

fn forge_verify_all_packages_markdown(report: &DxForgeVerifyAllPackagesReport) -> String {
    let mut output = format!(
        "# DX Forge Verify All Packages\n\n- Project: `{}`\n- Generated: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Packages: `{}`\n- Missing packages: `{}`\n\n",
        report.project.display(),
        report.generated_at,
        report.passed,
        report.score,
        report.packages.len(),
        report.missing_packages.len()
    );

    output.push_str(
        "| Package | Variant | Passed | Score | Update | Docs | Rollback | Scorecard |\n",
    );
    output.push_str("| --- | --- | --- | ---: | --- | --- | --- | --- |\n");
    for package in &report.packages {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} | `{}` | `{}` | `{}` | `{}` |\n",
            package.package_id,
            package.variant,
            package.passed,
            package.score,
            package.update.traffic.as_str(),
            package.docs.passed,
            package.rollback.passed,
            package.scorecard.passed
        ));
    }

    if !report.missing_packages.is_empty() {
        output.push_str("\n## Missing Packages\n\n");
        for package in &report.missing_packages {
            output.push_str(&format!(
                "- `{}` variant `{}`: {}\n",
                package.package_id, package.variant, package.message
            ));
        }
    }

    output
}

fn forge_verify_package_failure_summary(report: &DxForgeVerifyPackageReport) -> String {
    forge_verify_package_checks(report)
        .into_iter()
        .filter(|check| !check.passed)
        .map(|check| check.message.as_str())
        .collect::<Vec<_>>()
        .join("; ")
}

fn forge_verify_package_checks(report: &DxForgeVerifyPackageReport) -> Vec<&DxForgeVerifyCheck> {
    let mut checks = vec![
        &report.registry_integrity,
        &report.docs,
        &report.update,
        &report.rollback,
        &report.scorecard,
    ];
    checks.extend(report.package_specific_checks.iter());
    checks
}

fn print_forge_audit_terminal(report: &DxForgeAuditReport) {
    println!("DX Forge audit");
    println!("Path: {}", report.path.display());
    println!("Score: {}", report.risk_score);
    println!("Traffic: {}", report.traffic.as_str());

    if !report.packages.is_empty() {
        println!();
        println!("Packages:");
        for package in &report.packages {
            println!(
                "- {} ({}) score={} traffic={} findings={}",
                package.package_name,
                if package.path.is_empty() {
                    "."
                } else {
                    &package.path
                },
                package.risk_score,
                package.traffic.as_str(),
                package.finding_count
            );
        }
    }

    if report.findings.is_empty() {
        println!("No supply-chain findings detected.");
        return;
    }

    println!();
    println!("Findings:");
    for finding in &report.findings {
        println!(
            "- {:?} {}: {}",
            finding.severity, finding.code, finding.message
        );
        if let Some(path) = &finding.evidence_path {
            println!("  evidence: {path}");
        }
        println!("  remediation: {}", finding.remediation);
    }
}

fn strict_forge_failure_summary(findings: &[DxCheckFinding]) -> String {
    findings
        .iter()
        .map(|finding| format!("{} ({:?})", finding.code, finding.severity))
        .collect::<Vec<_>>()
        .join(", ")
}

fn project_contract_failure_summary(section: Option<&DxCheckSection>) -> String {
    let Some(section) = section else {
        return "project-contract section missing".to_string();
    };
    section
        .findings
        .iter()
        .map(|finding| format!("{} ({:?})", finding.code, finding.severity))
        .collect::<Vec<_>>()
        .join(", ")
}

fn print_dx_check_terminal(report: &DxCheckReport) {
    println!("DX check");
    println!("Path: {}", report.path.display());
    println!("Score: {}", report.score);
    println!("Traffic: {}", report.traffic.as_str());
    println!();
    println!("Sections:");
    for section in &report.sections {
        let metrics = section
            .metrics
            .iter()
            .map(|metric| format!("{}={}", metric.name, metric.value))
            .collect::<Vec<_>>()
            .join(", ");
        if metrics.is_empty() {
            println!(
                "- {} score={} traffic={} findings={}",
                section.name,
                section.score,
                section.traffic.as_str(),
                section.findings.len()
            );
        } else {
            println!(
                "- {} score={} traffic={} findings={} metrics={}",
                section.name,
                section.score,
                section.traffic.as_str(),
                section.findings.len(),
                metrics
            );
        }
        for finding in &section.findings {
            println!(
                "  - {:?} {}: {}",
                finding.severity, finding.code, finding.message
            );
            if let Some(path) = &finding.evidence_path {
                println!("    evidence: {path}");
            }
            println!("    remediation: {}", finding.remediation);
        }
    }
}

fn print_dx_check_latest_panel_terminal(report: &DxCheckLatestPanelReport) {
    println!("DX check latest receipt");
    println!("Receipt: {}", report.receipt_path.display());
    println!("Status: {}", report.status.as_str());
    if let Some(error) = &report.last_error {
        println!("Error: {error}");
    }
    println!("Next: {}", report.next_action);

    let Some(zed) = &report.zed else {
        return;
    };

    println!();
    println!(
        "Score: {}/{} ({}%){}",
        zed.score_value,
        zed.score_max,
        zed.score_percent,
        if zed.score_estimated {
            " estimated"
        } else {
            ""
        }
    );
    println!("Panel: {}", zed.status);
    println!(
        "Findings: blockers={} warnings={} quick_fixes={}",
        zed.blocker_count, zed.warning_count, zed.quick_fix_count
    );

    if !zed.sections.is_empty() {
        println!();
        println!("Buckets:");
        for section in &zed.sections {
            println!(
                "- {} {}/{} status={}{}",
                section.id,
                section.score,
                section.max_score,
                section.status,
                if section.estimated { " estimated" } else { "" }
            );
            println!("  {}", section.summary);
        }
    }

    if !zed.blockers.is_empty() {
        println!();
        println!("Blockers:");
        for finding in &zed.blockers {
            println!("- {}: {}", finding.code, finding.message);
            println!("  next: {}", finding.next_action);
        }
    }

    if !zed.warnings.is_empty() {
        println!();
        println!("Warnings:");
        for finding in &zed.warnings {
            println!("- {}: {}", finding.code, finding.message);
            println!("  next: {}", finding.next_action);
        }
    }

    if !zed.quick_fixes.is_empty() {
        println!();
        println!("Quick fixes:");
        for fix in &zed.quick_fixes {
            match &fix.command {
                Some(command) => println!("- {}: {} ({command})", fix.id, fix.label),
                None => println!("- {}: {}", fix.id, fix.label),
            }
            println!("  next: {}", fix.next_action);
        }
    }
}

fn dx_check_latest_panel_markdown(report: &DxCheckLatestPanelReport) -> String {
    let mut output = format!(
        "# DX Check Latest Receipt\n\n- Receipt: `{}`\n- Status: `{}`\n- Next: {}\n",
        report.receipt_path.display(),
        report.status.as_str(),
        report.next_action
    );

    if let Some(error) = &report.last_error {
        output.push_str(&format!("- Error: `{error}`\n"));
    }

    let Some(zed) = &report.zed else {
        return output;
    };

    output.push_str(&format!(
        "- Score: `{}/{}` (`{}%`{})\n- Panel: `{}`\n- Blockers: `{}`\n- Warnings: `{}`\n- Quick fixes: `{}`\n\n",
        zed.score_value,
        zed.score_max,
        zed.score_percent,
        if zed.score_estimated { ", estimated" } else { "" },
        zed.status,
        zed.blocker_count,
        zed.warning_count,
        zed.quick_fix_count
    ));

    if !zed.sections.is_empty() {
        output.push_str("| Bucket | Score | Status | Summary |\n");
        output.push_str("| --- | --- | --- | --- |\n");
        for section in &zed.sections {
            output.push_str(&format!(
                "| `{}` | `{}/{}` | `{}`{} | {} |\n",
                section.id,
                section.score,
                section.max_score,
                section.status,
                if section.estimated { " estimated" } else { "" },
                section.summary
            ));
        }
        output.push('\n');
    }

    if !zed.blockers.is_empty() {
        output.push_str("## Blockers\n\n");
        for finding in &zed.blockers {
            output.push_str(&format!(
                "- `{}`: {} Next: {}\n",
                finding.code, finding.message, finding.next_action
            ));
        }
        output.push('\n');
    }

    if !zed.warnings.is_empty() {
        output.push_str("## Warnings\n\n");
        for finding in &zed.warnings {
            output.push_str(&format!(
                "- `{}`: {} Next: {}\n",
                finding.code, finding.message, finding.next_action
            ));
        }
        output.push('\n');
    }

    if !zed.quick_fixes.is_empty() {
        output.push_str("## Quick Fixes\n\n");
        for fix in &zed.quick_fixes {
            let command = fix
                .command
                .as_ref()
                .map(|command| format!(" Command: `{command}`."))
                .unwrap_or_default();
            output.push_str(&format!(
                "- `{}`: {}. Next: {}{}\n",
                fix.id, fix.label, fix.next_action, command
            ));
        }
    }

    output
}

// =============================================================================
// Tests
// =============================================================================
