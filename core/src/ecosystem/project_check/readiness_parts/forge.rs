fn forge_section(root: &Path) -> DxCheckSection {
    let manifest_path = root.join(SOURCE_MANIFEST_PATH);
    let mut findings = Vec::new();
    let mut metrics = Vec::new();
    if !manifest_path.exists() {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "missing-forge-manifest",
            "Forge source manifest is not present",
            Some(SOURCE_MANIFEST_PATH.to_string()),
            "Run dx forge add for source-owned packages so receipts are tracked.",
        ));
        metrics.extend(forge_package_health_metrics(root, &mut findings));
        let mut section = section_from_findings("forge", findings);
        section.metrics = metrics;
        return section;
    }

    match fs::read(&manifest_path)
        .with_context(|| format!("read `{}`", manifest_path.display()))
        .and_then(|bytes| {
            serde_json::from_slice::<DxSourceManifest>(&bytes)
                .with_context(|| format!("parse `{}`", manifest_path.display()))
        }) {
        Ok(manifest) => {
            metrics.push(check_metric("packages", manifest.packages.len() as u64));
            metrics.extend(forge_manifest_update_metrics(
                root,
                &manifest,
                &mut findings,
            ));
            metrics.extend(forge_package_docs_metrics(root, &manifest, &mut findings));
            metrics.extend(forge_auth_package_metrics(root, &manifest, &mut findings));
            metrics.extend(forge_better_auth_package_metrics(
                root,
                &manifest,
                &mut findings,
            ));
            let (authentication_metrics, authentication_findings) =
                forge_authentication_package_metrics(root, &manifest);
            metrics.extend(authentication_metrics);
            findings.extend(authentication_findings);
            metrics.extend(forge_supabase_package_metrics(
                root,
                &manifest,
                &mut findings,
            ));
            let (backend_platform_client_metrics, backend_platform_client_findings) =
                forge_backend_platform_client_package_metrics(root, &manifest);
            metrics.extend(backend_platform_client_metrics);
            findings.extend(backend_platform_client_findings);
            let (state_management_metrics, state_management_findings) =
                forge_state_management_package_metrics(root, &manifest);
            metrics.extend(state_management_metrics);
            findings.extend(state_management_findings);
            let (data_fetching_cache_metrics, data_fetching_cache_findings) =
                forge_data_fetching_cache_package_metrics(root, &manifest);
            metrics.extend(data_fetching_cache_metrics);
            findings.extend(data_fetching_cache_findings);
            let (reactive_store_metrics, reactive_store_findings) =
                forge_reactive_store_package_metrics(root, &manifest);
            metrics.extend(reactive_store_metrics);
            findings.extend(reactive_store_findings);
            let (database_orm_metrics, database_orm_findings) =
                forge_database_orm_package_metrics(root, &manifest);
            metrics.extend(database_orm_metrics);
            findings.extend(database_orm_findings);
            let (forms_metrics, forms_findings) = forge_forms_package_metrics(root, &manifest);
            metrics.extend(forms_metrics);
            findings.extend(forms_findings);
            let (internationalization_metrics, internationalization_findings) =
                forge_internationalization_package_metrics(root, &manifest);
            metrics.extend(internationalization_metrics);
            findings.extend(internationalization_findings);
            let (type_safe_api_metrics, type_safe_api_findings) =
                forge_type_safe_api_package_metrics(root, &manifest);
            metrics.extend(type_safe_api_metrics);
            findings.extend(type_safe_api_findings);
            let (ai_sdk_metrics, ai_sdk_findings) = forge_ai_sdk_package_metrics(root, &manifest);
            metrics.extend(ai_sdk_metrics);
            findings.extend(ai_sdk_findings);
            let (markdown_mdx_content_metrics, markdown_mdx_content_findings) =
                forge_markdown_mdx_content_package_metrics(root, &manifest);
            metrics.extend(markdown_mdx_content_metrics);
            findings.extend(markdown_mdx_content_findings);
            let (documentation_system_metrics, documentation_system_findings) =
                forge_documentation_system_package_metrics(root, &manifest);
            metrics.extend(documentation_system_metrics);
            findings.extend(documentation_system_findings);
            let (payments_metrics, payments_findings) =
                forge_payments_package_metrics(root, &manifest);
            metrics.extend(payments_metrics);
            findings.extend(payments_findings);
            let (motion_animation_metrics, motion_animation_findings) =
                forge_motion_animation_package_metrics(root, &manifest);
            metrics.extend(motion_animation_metrics);
            findings.extend(motion_animation_findings);
            let (validation_schemas_metrics, validation_schemas_findings) =
                forge_validation_schemas_package_metrics(root, &manifest);
            metrics.extend(validation_schemas_metrics);
            findings.extend(validation_schemas_findings);
            let (automation_connectors_metrics, automation_connectors_findings) =
                forge_automation_connectors_package_metrics(root, &manifest);
            metrics.extend(automation_connectors_metrics);
            findings.extend(automation_connectors_findings);
            let (realtime_app_database_metrics, realtime_app_database_findings) =
                forge_realtime_app_database_package_metrics(root, &manifest);
            metrics.extend(realtime_app_database_metrics);
            findings.extend(realtime_app_database_findings);
            let (webassembly_bridge_metrics, webassembly_bridge_findings) =
                forge_webassembly_bridge_package_metrics(root, &manifest);
            metrics.extend(webassembly_bridge_metrics);
            findings.extend(webassembly_bridge_findings);
            let (three_scene_system_metrics, three_scene_system_findings) =
                forge_three_scene_system_package_metrics(root, &manifest);
            metrics.extend(three_scene_system_metrics);
            findings.extend(three_scene_system_findings);
            let (ui_components_metrics, ui_components_findings) =
                forge_ui_components_package_metrics(root, &manifest);
            metrics.extend(ui_components_metrics);
            findings.extend(ui_components_findings);
            if manifest.packages.is_empty() {
                findings.push(check_finding(
                    DxSupplyChainSeverity::Low,
                    "empty-forge-manifest",
                    "Forge manifest has no packages",
                    Some(SOURCE_MANIFEST_PATH.to_string()),
                    "Materialize at least one source-owned package or remove stale manifest state.",
                ));
            }
            match classify_forge_source_state(root) {
                Ok(report) => {
                    metrics.extend(forge_source_state_metrics(&report));
                    findings.extend(report.findings.iter().map(check_finding_from_supply_chain))
                }
                Err(error) => findings.push(check_finding(
                    DxSupplyChainSeverity::High,
                    "forge-source-state-check-failed",
                    format!("Forge source state could not be checked: {error}"),
                    Some(SOURCE_MANIFEST_PATH.to_string()),
                    "Regenerate the Forge manifest from valid receipts and re-run dx check.",
                )),
            }
            metrics.extend(forge_package_health_metrics(root, &mut findings));
        }
        Err(error) => {
            findings.push(check_finding(
                DxSupplyChainSeverity::High,
                "invalid-forge-manifest",
                format!("Forge manifest could not be parsed: {error}"),
                Some(SOURCE_MANIFEST_PATH.to_string()),
                "Regenerate the manifest from valid Forge receipts.",
            ));
        }
    }

    let receipt_dir = root.join(RECEIPT_DIR);
    let has_receipts = receipt_dir
        .read_dir()
        .map(|mut entries| entries.any(|entry| entry.is_ok()))
        .unwrap_or(false);
    if !has_receipts {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "missing-forge-receipts",
            "Forge manifest exists without reviewable receipts",
            Some(RECEIPT_DIR.to_string()),
            "Keep receipts under .dx/forge/receipts for every materialized source package.",
        ));
    }

    let findings = normalize_forge_findings(findings, &metrics);
    let mut section = section_from_findings("forge", findings);
    section.metrics = metrics;
    section
}

fn normalize_forge_findings(
    findings: Vec<DxCheckFinding>,
    metrics: &[DxCheckMetric],
) -> Vec<DxCheckFinding> {
    if !forge_package_lock_is_clean(metrics) {
        return findings;
    }

    let remote_head_health_is_dry_run =
        metric_value_from_metrics(metrics, "forge_remote_head_health_blocking_checks") > 0
            && metric_value_from_metrics(metrics, "forge_remote_head_health_missing_required") == 0
            && metric_value_from_metrics(metrics, "forge_remote_head_health_byte_mismatches") == 0
            && metric_value_from_metrics(metrics, "forge_remote_head_health_safe_receipts") == 0;

    findings
        .into_iter()
        .map(|mut finding| {
            if is_forge_package_lane_advisory(&finding.code)
                || is_forge_stale_receipt_advisory(&finding.code)
                || (remote_head_health_is_dry_run
                    && finding.code == "forge-remote-head-health-blocked")
                || (metric_value_from_metrics(metrics, "forge_vcs_missing_files") > 0
                    && finding.code == "forge-vcs-status-missing-files")
            {
                finding.severity = DxSupplyChainSeverity::Info;
            }
            finding
        })
        .collect()
}

fn forge_package_lock_is_clean(metrics: &[DxCheckMetric]) -> bool {
    metric_value_from_metrics(metrics, "forge_package_lock_integrity_valid") == 1
        && metric_value_from_metrics(metrics, "forge_package_lock_hash_mismatches") == 0
        && metric_value_from_metrics(metrics, "forge_package_lock_missing_files") == 0
        && metric_value_from_metrics(metrics, "forge_package_receipts_missing") == 0
        && metric_value_from_metrics(metrics, "forge_package_cache_missing_files") == 0
}

fn metric_value_from_metrics(metrics: &[DxCheckMetric], name: &str) -> u64 {
    metrics
        .iter()
        .find(|metric| metric.name == name)
        .map(|metric| metric.value)
        .unwrap_or(0)
}

fn is_forge_package_lane_advisory(code: &str) -> bool {
    code.ends_with("-blocked-surface")
        || code.ends_with("-unsupported-surface")
        || code.ends_with("-missing-dx-style-compatibility")
}

fn is_forge_stale_receipt_advisory(code: &str) -> bool {
    code == "forge-package-stale"
        || code.ends_with("-stale-receipt")
        || code == "supabase-client-stale-receipt"
        || code == "auth-google-stale-receipt"
        || code == "auth-better-auth-stale-receipt"
}

fn forge_package_health_metrics(
    root: &Path,
    findings: &mut Vec<DxCheckFinding>,
) -> Vec<DxCheckMetric> {
    let mut metrics = Vec::new();
    let mut remote_keys = HashSet::new();
    let mut package_count = 0u64;
    let mut package_file_count = 0u64;
    let mut missing_package_files = 0u64;
    let mut hash_mismatches = 0u64;
    let mut unsafe_package_paths = 0u64;
    let mut package_receipts = 0u64;
    let mut missing_package_receipts = 0u64;
    let mut invalid_package_receipts = 0u64;
    let mut package_cache_files = 0u64;
    let mut missing_package_cache_files = 0u64;
    let mut remote_count = 0u64;
    let mut unsafe_remotes = 0u64;
    let mut media_assets = 0u64;
    let mut missing_media_assets = 0u64;
    let mut media_chunk_maps = 0u64;
    let mut media_status_present = 0u64;
    let mut media_restore_receipts = 0u64;
    let mut media_cache_files = 0u64;
    let mut missing_media_cache_files = 0u64;
    let mut vcs_snapshot_present = 0u64;
    let mut vcs_snapshot_receipt_present = 0u64;
    let mut vcs_tracked_files = 0u64;
    let mut vcs_missing_files = 0u64;
    let mut vcs_media_files = 0u64;
    let mut remote_status_present = 0u64;
    let mut remote_sync_plan_present = 0u64;
    let mut remote_safe_count = 0u64;
    let mut remote_boundary_count = 0u64;
    let mut remote_status_unsafe_count = 0u64;
    let mut remote_head_health_receipts = 0u64;
    let mut remote_head_health_safe_receipts = 0u64;
    let mut remote_head_health_blocking_checks = 0u64;
    let mut remote_head_health_missing_required = 0u64;
    let mut remote_head_health_missing_optional = 0u64;
    let mut remote_head_health_byte_mismatches = 0u64;

    let lock_present = root.join(PACKAGE_LOCK_PATH).is_file();
    let mut lock_json_valid = false;
    if let Some(lock) = read_optional_forge_json(
        root,
        PACKAGE_LOCK_PATH,
        "invalid-forge-package-lock",
        findings,
    ) {
        lock_json_valid = true;
        let packages = json_array_entries(&lock, &["packages"]);
        package_count = packages.len() as u64;
        if package_count == 0 {
            findings.push(check_finding(
                DxSupplyChainSeverity::Low,
                "empty-forge-package-lock",
                "Forge package lock exists but contains no packages",
                Some(PACKAGE_LOCK_PATH.to_string()),
                "Regenerate the Forge package lock after declaring source-owned packages.",
            ));
        }

        for package in packages {
            for file in json_array_entries(package, &["files"]) {
                package_file_count += 1;
                let Some(path) = json_text(file, &["path", "source_path"]) else {
                    unsafe_package_paths += 1;
                    continue;
                };
                let Some(project_path) = resolve_dx_check_relative_path(root, path) else {
                    unsafe_package_paths += 1;
                    continue;
                };
                if !project_path.is_file() {
                    missing_package_files += 1;
                    continue;
                }
                if let Some(expected_hash) =
                    json_text(file, &["expected_hash", "hash", "content_hash"])
                {
                    match hash_project_file(&project_path) {
                        Ok(actual_hash) if normalize_forge_hash(expected_hash) == actual_hash => {}
                        Ok(_) => hash_mismatches += 1,
                        Err(_) => hash_mismatches += 1,
                    }
                }
            }
            let receipt_metrics = forge_package_receipt_metrics(root, package);
            package_receipts += receipt_metrics.receipts;
            missing_package_receipts += receipt_metrics.missing_receipts;
            invalid_package_receipts += receipt_metrics.invalid_receipts;
            package_cache_files += receipt_metrics.cache_files;
            missing_package_cache_files += receipt_metrics.missing_cache_files;
            hash_mismatches += receipt_metrics.cache_hash_mismatches;
        }

        let (lock_remotes, lock_unsafe_remotes) =
            forge_remote_metrics_from_json(&lock, &mut remote_keys);
        remote_count += lock_remotes;
        unsafe_remotes += lock_unsafe_remotes;

        let (lock_media_assets, lock_missing_media, lock_chunk_maps) =
            forge_media_metrics_from_json(root, &lock);
        media_assets += lock_media_assets;
        missing_media_assets += lock_missing_media;
        media_chunk_maps += lock_chunk_maps;
    }

    if let Some(remotes) = read_optional_forge_json(
        root,
        REMOTES_CONFIG_PATH,
        "invalid-forge-remotes-config",
        findings,
    ) {
        let (configured_remotes, configured_unsafe_remotes) =
            forge_remote_metrics_from_json(&remotes, &mut remote_keys);
        remote_count += configured_remotes;
        unsafe_remotes += configured_unsafe_remotes;
    }

    if let Some(media) = read_optional_forge_json(
        root,
        MEDIA_MANIFEST_PATH,
        "invalid-forge-media-manifest",
        findings,
    ) {
        let (manifest_media_assets, manifest_missing_media, manifest_chunk_maps) =
            forge_media_metrics_from_json(root, &media);
        media_assets += manifest_media_assets;
        missing_media_assets += manifest_missing_media;
        media_chunk_maps += manifest_chunk_maps;
    }

    if let Some(media_status) = read_optional_forge_json(
        root,
        MEDIA_STATUS_PATH,
        "invalid-forge-media-status",
        findings,
    ) {
        let status_metrics = forge_media_status_metrics_from_json(root, &media_status);
        media_status_present = 1;
        media_restore_receipts = status_metrics.restore_receipts;
        media_cache_files = status_metrics.cache_files;
        missing_media_cache_files = status_metrics.missing_cache_files;
    }

    if let Some(vcs_status) =
        read_optional_forge_json(root, VCS_STATUS_PATH, "invalid-forge-vcs-status", findings)
    {
        let vcs_metrics = forge_vcs_metrics_from_json(root, &vcs_status);
        vcs_snapshot_present = 1;
        vcs_snapshot_receipt_present = vcs_metrics.snapshot_receipt_present;
        vcs_tracked_files = vcs_metrics.tracked_files;
        vcs_missing_files = vcs_metrics.missing_files;
        vcs_media_files = vcs_metrics.media_files;
    }

    if let Some(remote_status) = read_optional_forge_json(
        root,
        REMOTE_STATUS_PATH,
        "invalid-forge-remote-status",
        findings,
    ) {
        let remote_status_metrics = forge_remote_status_metrics_from_json(root, &remote_status);
        remote_status_present = 1;
        remote_sync_plan_present = remote_status_metrics.sync_plan_present;
        remote_safe_count = remote_status_metrics.safe_count;
        remote_boundary_count = remote_status_metrics.boundary_count;
        remote_status_unsafe_count = remote_status_metrics.unsafe_count;
    }

    if let Some(forge_status) = read_optional_forge_json(
        root,
        FORGE_STATUS_LATEST_RECEIPT_PATH,
        "invalid-forge-status-latest-receipt",
        findings,
    ) {
        let head_health_metrics = forge_remote_head_health_metrics_from_status_json(&forge_status);
        remote_head_health_receipts = head_health_metrics.receipts;
        remote_head_health_safe_receipts = head_health_metrics.safe_receipts;
        remote_head_health_blocking_checks = head_health_metrics.blocking_checks;
        remote_head_health_missing_required = head_health_metrics.missing_required;
        remote_head_health_missing_optional = head_health_metrics.missing_optional;
        remote_head_health_byte_mismatches = head_health_metrics.byte_mismatches;
    }

    if missing_package_files > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "forge-package-lock-missing-files",
            format!("Forge package lock references {missing_package_files} missing source file(s)"),
            Some(PACKAGE_LOCK_PATH.to_string()),
            "Regenerate the package lock after restoring or removing the missing package slices.",
        ));
    }
    if hash_mismatches > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::High,
            "forge-package-lock-hash-mismatch",
            format!("Forge package lock has {hash_mismatches} content hash mismatch(es)"),
            Some(PACKAGE_LOCK_PATH.to_string()),
            "Review the changed files and rewrite the Forge package lock from trusted source.",
        ));
    }
    if unsafe_package_paths > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::High,
            "forge-package-lock-unsafe-path",
            format!("Forge package lock contains {unsafe_package_paths} unsafe path entry(s)"),
            Some(PACKAGE_LOCK_PATH.to_string()),
            "Keep package lock paths project-relative and inside the project root.",
        ));
    }
    if missing_package_receipts > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "forge-package-receipts-missing",
            format!("Forge package lock references {missing_package_receipts} missing receipt(s)"),
            Some(PACKAGE_LOCK_PATH.to_string()),
            "Regenerate package add/status receipts or remove stale receipt paths from the lock.",
        ));
    }
    if invalid_package_receipts > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "forge-package-receipts-invalid",
            format!("Forge package lock references {invalid_package_receipts} invalid receipt(s)"),
            Some(PACKAGE_LOCK_PATH.to_string()),
            "Regenerate package receipts from Forge instead of editing them by hand.",
        ));
    }
    if missing_package_cache_files > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "forge-package-cache-missing-files",
            format!(
                "Forge package receipts reference {missing_package_cache_files} missing cache file(s)"
            ),
            Some(PACKAGE_LOCK_PATH.to_string()),
            "Regenerate the Forge package cache from trusted source slices.",
        ));
    }
    if unsafe_remotes > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "forge-unsafe-remote-config",
            format!("Forge remote config exposes {unsafe_remotes} unsafe remote definition(s)"),
            Some(REMOTES_CONFIG_PATH.to_string()),
            "Move tokens and passwords into env, keychain, or the Forge auth store.",
        ));
    }
    if missing_media_assets > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "forge-media-assets-missing",
            format!("Forge media manifest references {missing_media_assets} missing asset(s)"),
            Some(MEDIA_MANIFEST_PATH.to_string()),
            "Restore the assets or update the media manifest before release.",
        ));
    }
    if media_status_present > 0 && media_restore_receipts == 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "forge-media-restore-receipts-missing",
            "Forge media status references no present restore receipt",
            Some(MEDIA_STATUS_PATH.to_string()),
            "Regenerate media restore receipts from trusted media status data.",
        ));
    }
    if missing_media_cache_files > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "forge-media-cache-missing-files",
            format!(
                "Forge media status references {missing_media_cache_files} missing cache file(s)"
            ),
            Some(MEDIA_STATUS_PATH.to_string()),
            "Regenerate the Forge media cache from tracked assets before release.",
        ));
    }
    if vcs_missing_files > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "forge-vcs-status-missing-files",
            format!("Forge VCS status references {vcs_missing_files} missing tracked file(s)"),
            Some(VCS_STATUS_PATH.to_string()),
            "Regenerate the Forge VCS status after restoring or removing missing tracked files.",
        ));
    }
    if vcs_snapshot_present > 0 && vcs_snapshot_receipt_present == 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "forge-vcs-snapshot-receipt-missing",
            "Forge VCS status references a missing snapshot receipt",
            Some(VCS_STATUS_PATH.to_string()),
            "Regenerate the Forge VCS snapshot receipt from trusted source.",
        ));
    }
    if remote_status_present > 0 && remote_sync_plan_present == 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "forge-remote-sync-plan-receipt-missing",
            "Forge remote status references a missing sync-plan receipt",
            Some(REMOTE_STATUS_PATH.to_string()),
            "Regenerate the Forge remote sync-plan receipt from trusted source.",
        ));
    }
    if remote_status_unsafe_count > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "forge-remote-status-unsafe",
            format!(
                "Forge remote status exposes {remote_status_unsafe_count} unsafe remote definition(s)"
            ),
            Some(REMOTE_STATUS_PATH.to_string()),
            "Move tokens and passwords into env, keychain, or the Forge auth store.",
        ));
    }
    if remote_head_health_blocking_checks > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::High,
            "forge-remote-head-health-blocked",
            format!(
                "Forge remote HEAD health has {remote_head_health_blocking_checks} blocking object check(s)"
            ),
            Some(FORGE_STATUS_LATEST_RECEIPT_PATH.to_string()),
            "Repair missing or byte-mismatched remote objects and re-run approved read-only HEAD checks before remote install.",
        ));
    }

    let lock_integrity_valid = u64::from(
        lock_present && lock_json_valid && missing_package_files == 0 && hash_mismatches == 0,
    );
    metrics.push(check_metric(
        "forge_package_lock_present",
        u64::from(lock_present),
    ));
    metrics.push(check_metric("forge_package_lock_packages", package_count));
    metrics.push(check_metric("forge_package_lock_files", package_file_count));
    metrics.push(check_metric(
        "forge_package_lock_missing_files",
        missing_package_files,
    ));
    metrics.push(check_metric(
        "forge_package_lock_hash_mismatches",
        hash_mismatches,
    ));
    metrics.push(check_metric(
        "forge_package_lock_integrity_valid",
        lock_integrity_valid,
    ));
    metrics.push(check_metric("forge_package_receipts", package_receipts));
    metrics.push(check_metric(
        "forge_package_receipts_missing",
        missing_package_receipts,
    ));
    metrics.push(check_metric(
        "forge_package_receipts_invalid",
        invalid_package_receipts,
    ));
    metrics.push(check_metric(
        "forge_package_cache_files",
        package_cache_files,
    ));
    metrics.push(check_metric(
        "forge_package_cache_missing_files",
        missing_package_cache_files,
    ));
    metrics.push(check_metric("forge_remotes_configured", remote_count));
    metrics.push(check_metric("forge_remotes_unsafe", unsafe_remotes));
    metrics.push(check_metric("forge_media_assets_tracked", media_assets));
    metrics.push(check_metric(
        "forge_media_assets_missing",
        missing_media_assets,
    ));
    metrics.push(check_metric("forge_media_chunk_maps", media_chunk_maps));
    metrics.push(check_metric(
        "forge_media_status_present",
        media_status_present,
    ));
    metrics.push(check_metric(
        "forge_media_restore_receipts",
        media_restore_receipts,
    ));
    metrics.push(check_metric("forge_media_cache_files", media_cache_files));
    metrics.push(check_metric(
        "forge_media_cache_missing_files",
        missing_media_cache_files,
    ));
    metrics.push(check_metric(
        "forge_vcs_snapshot_present",
        vcs_snapshot_present,
    ));
    metrics.push(check_metric(
        "forge_vcs_snapshot_receipt_present",
        vcs_snapshot_receipt_present,
    ));
    metrics.push(check_metric("forge_vcs_tracked_files", vcs_tracked_files));
    metrics.push(check_metric("forge_vcs_missing_files", vcs_missing_files));
    metrics.push(check_metric("forge_vcs_media_files", vcs_media_files));
    metrics.push(check_metric(
        "forge_remote_status_present",
        remote_status_present,
    ));
    metrics.push(check_metric(
        "forge_remote_sync_plan_present",
        remote_sync_plan_present,
    ));
    metrics.push(check_metric("forge_remote_safe_count", remote_safe_count));
    metrics.push(check_metric(
        "forge_remote_boundary_count",
        remote_boundary_count,
    ));
    metrics.push(check_metric(
        "forge_remote_head_health_receipts",
        remote_head_health_receipts,
    ));
    metrics.push(check_metric(
        "forge_remote_head_health_safe_receipts",
        remote_head_health_safe_receipts,
    ));
    metrics.push(check_metric(
        "forge_remote_head_health_blocking_checks",
        remote_head_health_blocking_checks,
    ));
    metrics.push(check_metric(
        "forge_remote_head_health_missing_required",
        remote_head_health_missing_required,
    ));
    metrics.push(check_metric(
        "forge_remote_head_health_missing_optional",
        remote_head_health_missing_optional,
    ));
    metrics.push(check_metric(
        "forge_remote_head_health_byte_mismatches",
        remote_head_health_byte_mismatches,
    ));
    metrics
}

#[derive(Default)]
struct ForgePackageReceiptMetrics {
    receipts: u64,
    missing_receipts: u64,
    invalid_receipts: u64,
    cache_files: u64,
    missing_cache_files: u64,
    cache_hash_mismatches: u64,
}

fn forge_package_receipt_metrics(
    root: &Path,
    package: &serde_json::Value,
) -> ForgePackageReceiptMetrics {
    let mut metrics = ForgePackageReceiptMetrics::default();
    for receipt in json_array_entries(package, &["receipt_paths", "receipts"]) {
        let Some(receipt_path) = receipt
            .as_str()
            .or_else(|| json_text(receipt, &["path", "receipt_path"]))
        else {
            metrics.invalid_receipts += 1;
            continue;
        };
        let Some(project_path) = resolve_dx_check_relative_path(root, receipt_path) else {
            metrics.invalid_receipts += 1;
            continue;
        };

        metrics.receipts += 1;
        if !project_path.is_file() {
            metrics.missing_receipts += 1;
            continue;
        }

        let Ok(bytes) = fs::read(&project_path) else {
            metrics.invalid_receipts += 1;
            continue;
        };
        let Ok(receipt_json) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
            metrics.invalid_receipts += 1;
            continue;
        };
        if json_text(&receipt_json, &["schema"]) == Some("forge.package_add_receipt") {
            merge_package_cache_metrics(
                &mut metrics,
                &forge_package_cache_metrics(root, &receipt_json),
            );
        }
    }
    metrics
}

fn forge_package_cache_metrics(
    root: &Path,
    receipt: &serde_json::Value,
) -> ForgePackageReceiptMetrics {
    let mut metrics = ForgePackageReceiptMetrics::default();
    let Some(cache) = receipt.get("cache") else {
        return metrics;
    };

    for cached_file in json_array_entries(cache, &["cached_files", "files"]) {
        metrics.cache_files += 1;
        let Some(cache_path) = json_text(cached_file, &["cache_path", "path"]) else {
            metrics.missing_cache_files += 1;
            continue;
        };
        let Some(project_path) = resolve_dx_check_relative_path(root, cache_path) else {
            metrics.missing_cache_files += 1;
            continue;
        };
        if !project_path.is_file() {
            metrics.missing_cache_files += 1;
            continue;
        }
        if let Some(expected_hash) = json_text(cached_file, &["content_hash", "hash"]) {
            match hash_project_file(&project_path) {
                Ok(actual_hash) if normalize_forge_hash(expected_hash) == actual_hash => {}
                Ok(_) => metrics.cache_hash_mismatches += 1,
                Err(_) => metrics.cache_hash_mismatches += 1,
            }
        }
    }
    metrics
}

fn merge_package_cache_metrics(
    target: &mut ForgePackageReceiptMetrics,
    source: &ForgePackageReceiptMetrics,
) {
    target.cache_files += source.cache_files;
    target.missing_cache_files += source.missing_cache_files;
    target.cache_hash_mismatches += source.cache_hash_mismatches;
}

pub(super) fn read_optional_forge_json(
    root: &Path,
    relative_path: &str,
    invalid_code: &str,
    findings: &mut Vec<DxCheckFinding>,
) -> Option<serde_json::Value> {
    let path = root.join(relative_path);
    if !path.is_file() {
        return None;
    }
    match fs::read(&path)
        .with_context(|| format!("read `{}`", path.display()))
        .and_then(|bytes| {
            serde_json::from_slice::<serde_json::Value>(&bytes)
                .with_context(|| format!("parse `{}`", path.display()))
        }) {
        Ok(value) => Some(value),
        Err(error) => {
            findings.push(check_finding(
                DxSupplyChainSeverity::High,
                invalid_code,
                format!("Forge JSON could not be parsed: {error}"),
                Some(relative_path.to_string()),
                "Regenerate this Forge receipt/config from a valid source.",
            ));
            None
        }
    }
}

pub(super) fn json_array_entries<'a>(
    value: &'a serde_json::Value,
    keys: &[&str],
) -> Vec<&'a serde_json::Value> {
    if let Some(array) = value.as_array() {
        return array.iter().collect();
    }
    for key in keys {
        if let Some(array) = value.get(*key).and_then(serde_json::Value::as_array) {
            return array.iter().collect();
        }
    }
    Vec::new()
}

pub(super) fn json_text<'a>(value: &'a serde_json::Value, keys: &[&str]) -> Option<&'a str> {
    keys.iter()
        .find_map(|key| value.get(*key).and_then(serde_json::Value::as_str))
}

pub(super) fn resolve_dx_check_relative_path(root: &Path, relative: &str) -> Option<PathBuf> {
    let path = Path::new(relative);
    if path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return None;
    }

    let normalized = relative.replace('\\', "/");
    let mut candidates = vec![root.join(path)];

    if let Some(stripped) = normalized.strip_prefix("examples/template/") {
        candidates.push(root.join(Path::new(stripped)));
    }

    if let Some(repo_root) = root.parent().and_then(Path::parent) {
        if normalized.starts_with("examples/template/")
            || normalized.starts_with("examples/dashboard/")
            || normalized.starts_with("tools/")
            || normalized.starts_with("docs/")
            || normalized.starts_with("core/")
            || normalized.starts_with("dx-www/")
            || normalized.starts_with("benchmarks/")
        {
            candidates.push(repo_root.join(Path::new(&normalized)));
        }
    }

    candidates
        .iter()
        .find(|candidate| candidate.is_file())
        .cloned()
        .or_else(|| candidates.into_iter().next())
}

fn hash_project_file(path: &Path) -> Result<String> {
    let bytes = fs::read(path).with_context(|| format!("read `{}`", path.display()))?;
    Ok(blake3::hash(&bytes).to_hex().to_string())
}

fn normalize_forge_hash(value: &str) -> String {
    value
        .trim()
        .strip_prefix("blake3:")
        .unwrap_or(value.trim())
        .to_ascii_lowercase()
}

fn forge_remote_metrics_from_json(
    value: &serde_json::Value,
    seen: &mut HashSet<String>,
) -> (u64, u64) {
    let mut remote_count = 0u64;
    let mut unsafe_remotes = 0u64;
    for remote in json_array_entries(value, &["remotes", "providers"]) {
        let name = json_text(remote, &["name"]).unwrap_or("remote");
        let kind = json_text(remote, &["kind", "provider"]).unwrap_or("unknown");
        let locator = json_text(remote, &["locator", "url", "endpoint"]).unwrap_or("");
        let key = format!("{name}|{kind}|{locator}");
        if !seen.insert(key) {
            continue;
        }
        remote_count += 1;
        let unsafe_by_receipt = remote
            .get("secrets_safe")
            .and_then(serde_json::Value::as_bool)
            == Some(false);
        let unsafe_by_text = [locator, json_text(remote, &["auth_ref"]).unwrap_or("")]
            .into_iter()
            .any(has_plaintext_secret_marker)
            || json_text(remote, &["secret_policy"]).is_some_and(has_plaintext_secret_marker);
        if unsafe_by_receipt || unsafe_by_text {
            unsafe_remotes += 1;
        }
    }
    (remote_count, unsafe_remotes)
}

fn forge_media_metrics_from_json(root: &Path, value: &serde_json::Value) -> (u64, u64, u64) {
    let mut media_assets = 0u64;
    let mut missing_media_assets = 0u64;
    let mut media_chunk_maps = 0u64;
    for asset in json_array_entries(value, &["media", "assets"]) {
        media_assets += 1;
        let exists_by_receipt = asset.get("exists").and_then(serde_json::Value::as_bool);
        if let Some(path) = json_text(asset, &["path", "source_path"]) {
            if let Some(project_path) = resolve_dx_check_relative_path(root, path) {
                if !project_path.is_file() || exists_by_receipt == Some(false) {
                    missing_media_assets += 1;
                }
            } else {
                missing_media_assets += 1;
            }
        } else if exists_by_receipt == Some(false) {
            missing_media_assets += 1;
        }

        media_chunk_maps += asset
            .get("chunk_count")
            .and_then(serde_json::Value::as_u64)
            .or_else(|| {
                asset
                    .get("chunk_map")
                    .and_then(serde_json::Value::as_array)
                    .map(|chunks| chunks.len() as u64)
            })
            .unwrap_or(0);
    }
    (media_assets, missing_media_assets, media_chunk_maps)
}

#[derive(Default)]
struct ForgeMediaStatusMetrics {
    restore_receipts: u64,
    cache_files: u64,
    missing_cache_files: u64,
}

fn forge_media_status_metrics_from_json(
    root: &Path,
    value: &serde_json::Value,
) -> ForgeMediaStatusMetrics {
    let mut metrics = ForgeMediaStatusMetrics::default();
    for asset in json_array_entries(value, &["assets", "media"]) {
        if let Some(receipt_path) = json_text(asset, &["restore_receipt_path"]) {
            if resolve_dx_check_relative_path(root, receipt_path).is_some_and(|path| path.is_file())
            {
                metrics.restore_receipts += 1;
            }
        }

        let Some(cache_path) = json_text(asset, &["cache_path"]) else {
            continue;
        };
        metrics.cache_files += 1;
        let Some(project_path) = resolve_dx_check_relative_path(root, cache_path) else {
            metrics.missing_cache_files += 1;
            continue;
        };
        if !project_path.is_file() {
            metrics.missing_cache_files += 1;
        }
    }

    metrics
}

#[derive(Default)]
struct ForgeVcsMetrics {
    snapshot_receipt_present: u64,
    tracked_files: u64,
    missing_files: u64,
    media_files: u64,
}

fn forge_vcs_metrics_from_json(root: &Path, value: &serde_json::Value) -> ForgeVcsMetrics {
    let mut metrics = ForgeVcsMetrics::default();
    if let Some(snapshot_receipt) = json_text(value, &["snapshot_receipt_path"]) {
        if resolve_dx_check_relative_path(root, snapshot_receipt).is_some_and(|path| path.is_file())
        {
            metrics.snapshot_receipt_present = 1;
        }
    }

    for file in json_array_entries(value, &["tracked_files", "files"]) {
        metrics.tracked_files += 1;
        if json_text(file, &["kind"]) == Some("media") {
            metrics.media_files += 1;
        }

        let exists_by_receipt = file.get("exists").and_then(serde_json::Value::as_bool);
        let Some(path) = json_text(file, &["path", "source_path"]) else {
            metrics.missing_files += 1;
            continue;
        };
        let Some(project_path) = resolve_dx_check_relative_path(root, path) else {
            metrics.missing_files += 1;
            continue;
        };
        if !project_path.is_file() || exists_by_receipt == Some(false) {
            metrics.missing_files += 1;
        }
    }

    metrics
}

#[derive(Default)]
struct ForgeRemoteStatusMetrics {
    sync_plan_present: u64,
    safe_count: u64,
    boundary_count: u64,
    unsafe_count: u64,
}

#[derive(Default)]
struct ForgeRemoteHeadHealthMetrics {
    receipts: u64,
    safe_receipts: u64,
    blocking_checks: u64,
    missing_required: u64,
    missing_optional: u64,
    byte_mismatches: u64,
}

fn forge_remote_head_health_metrics_from_status_json(
    value: &serde_json::Value,
) -> ForgeRemoteHeadHealthMetrics {
    let mut metrics = ForgeRemoteHeadHealthMetrics::default();
    for evaluation in remote_head_health_evaluations_from_json(value) {
        metrics.receipts += 1;
        if evaluation
            .get("safe_for_remote_install")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
        {
            metrics.safe_receipts += 1;
        }
        metrics.blocking_checks += json_u64(evaluation, "blocking_check_count");
        metrics.missing_required += json_u64(evaluation, "missing_required_count");
        metrics.missing_optional += json_u64(evaluation, "missing_optional_count");
        metrics.byte_mismatches += json_u64(evaluation, "byte_mismatch_count");
    }
    metrics
}

fn remote_head_health_evaluations_from_json(value: &serde_json::Value) -> Vec<&serde_json::Value> {
    if let Some(evaluations) = value
        .get("remote_object_head_health")
        .and_then(serde_json::Value::as_array)
    {
        return evaluations.iter().collect();
    }
    if let Some(evaluation) = value.get("health_evaluation") {
        return vec![evaluation];
    }
    if let Some(evaluation) = value.get("object_head_health_evaluation") {
        return vec![evaluation];
    }
    if let Some(evaluation) = value
        .get("remote_read_plan")
        .and_then(|plan| plan.get("object_head_health_evaluation"))
    {
        return vec![evaluation];
    }
    if let Some(evaluation) = value
        .get("remote_read_plan")
        .and_then(|plan| plan.get("health_evaluation"))
    {
        return vec![evaluation];
    }
    if value
        .get("schema_version")
        .and_then(serde_json::Value::as_str)
        == Some("dx.forge.remote_object_head_health")
    {
        return vec![value];
    }
    Vec::new()
}

fn forge_remote_status_metrics_from_json(
    root: &Path,
    value: &serde_json::Value,
) -> ForgeRemoteStatusMetrics {
    let mut metrics = ForgeRemoteStatusMetrics::default();
    if let Some(sync_plan_receipt) = json_text(value, &["sync_plan_receipt_path"]) {
        if resolve_dx_check_relative_path(root, sync_plan_receipt)
            .is_some_and(|path| path.is_file())
        {
            metrics.sync_plan_present = 1;
        }
    }

    let remotes = json_array_entries(value, &["remotes", "providers"]);
    for remote in &remotes {
        let locator = json_text(remote, &["locator", "url", "endpoint"]).unwrap_or("");
        let auth_ref = json_text(remote, &["auth_ref"]).unwrap_or("");
        let secret_policy = json_text(remote, &["secret_policy"]).unwrap_or("");
        let plaintext_secret_present = [locator, auth_ref, secret_policy]
            .into_iter()
            .any(has_plaintext_secret_marker);
        let secrets_safe = remote
            .get("secrets_safe")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(!plaintext_secret_present)
            && !plaintext_secret_present;
        if secrets_safe {
            metrics.safe_count += 1;
        } else {
            metrics.unsafe_count += 1;
        }

        let executable_now = remote
            .get("executable_now")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
        let boundary = json_text(remote, &["boundary"]).unwrap_or("");
        if !executable_now || boundary.contains("boundary") {
            metrics.boundary_count += 1;
        }
    }

    if remotes.is_empty() {
        if let Some(summary) = value.get("summary") {
            metrics.safe_count = summary
                .get("safe_remote_count")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(metrics.safe_count);
            metrics.unsafe_count = summary
                .get("unsafe_remote_count")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(metrics.unsafe_count);
            metrics.boundary_count = summary
                .get("boundary_remote_count")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(metrics.boundary_count);
        }
    }

    metrics
}

fn has_plaintext_secret_marker(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    if let Some((scheme, rest)) = lower.split_once("://") {
        let path_start = rest.find('/').unwrap_or(rest.len());
        if scheme != "file" && rest[..path_start].contains('@') {
            return true;
        }
    }

    [
        "token=",
        "secret=",
        "password=",
        "passwd=",
        "access_key=",
        "access-key=",
        "api_key=",
        "apikey=",
        "client_secret=",
    ]
    .iter()
    .any(|marker| lower.contains(marker))
}

fn forge_manifest_update_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
    findings: &mut Vec<DxCheckFinding>,
) -> Vec<DxCheckMetric> {
    let mut stale_packages = 0u64;
    let mut accepted_local_update_packages = 0u64;
    let mut rollback_covered = 0u64;
    let mut rollback_missing = 0u64;
    let mut packages_without_update = 0u64;
    let mut oldest_update_age_days = 0u64;
    let receipt_dir = root.join(RECEIPT_DIR);

    for package in &manifest.packages {
        if package.source_kind != DxSourceKind::Local {
            match source_package_for_project_variant(&package.package_id, root, &package.variant) {
                Ok(latest) => {
                    let version_is_stale = package.version != latest.version;
                    let integrity_differs = package.integrity_hash != latest.integrity_hash;
                    let accepted_same_version_local_update = !version_is_stale
                        && integrity_differs
                        && package.last_accepted_update.is_some();
                    if accepted_same_version_local_update {
                        accepted_local_update_packages += 1;
                    } else if version_is_stale || integrity_differs {
                        stale_packages += 1;
                        findings.push(check_finding(
                            DxSupplyChainSeverity::Low,
                            "forge-package-stale",
                            format!(
                                "Forge package `{}` variant `{}` is behind the current curated source",
                                package.package_id, package.variant
                            ),
                            Some(SOURCE_MANIFEST_PATH.to_string()),
                            "Run dx update with the matching --variant and review the generated change set.",
                        ));
                    }
                }
                Err(error) => findings.push(check_finding(
                    DxSupplyChainSeverity::Medium,
                    "forge-package-latest-unavailable",
                    format!(
                        "Forge could not resolve the latest source for `{}` variant `{}`: {error}",
                        package.package_id, package.variant
                    ),
                    Some(SOURCE_MANIFEST_PATH.to_string()),
                    "Check the package id, variant, registry configuration, and root `dx` path mappings.",
                )),
            }
        }

        if let Some(timestamp) = &package.last_accepted_update {
            match DateTime::parse_from_rfc3339(timestamp) {
                Ok(accepted_at) => {
                    let accepted_at = accepted_at.with_timezone(&Utc);
                    let age_days = Utc::now()
                        .signed_duration_since(accepted_at)
                        .num_days()
                        .max(0) as u64;
                    oldest_update_age_days = oldest_update_age_days.max(age_days);
                }
                Err(_) => findings.push(check_finding(
                    DxSupplyChainSeverity::Medium,
                    "forge-package-invalid-update-timestamp",
                    format!(
                        "Forge package `{}` variant `{}` has an invalid last accepted update timestamp",
                        package.package_id, package.variant
                    ),
                    Some(SOURCE_MANIFEST_PATH.to_string()),
                    "Regenerate the package manifest through a valid Forge update receipt.",
                )),
            }
        } else {
            packages_without_update += 1;
        }

        match &package.rollback_receipt {
            Some(receipt) if receipt_dir.join(receipt).exists() => rollback_covered += 1,
            Some(receipt) => {
                rollback_missing += 1;
                findings.push(check_finding(
                    DxSupplyChainSeverity::Medium,
                    "forge-package-rollback-receipt-missing",
                    format!(
                        "Forge package `{}` variant `{}` references missing rollback receipt `{}`",
                        package.package_id, package.variant, receipt
                    ),
                    Some(format!("{RECEIPT_DIR}/{receipt}")),
                    "Restore the rollback receipt or accept a new reviewed Forge update.",
                ));
            }
            None if package.last_accepted_update.is_some() => {
                rollback_missing += 1;
                findings.push(check_finding(
                    DxSupplyChainSeverity::Medium,
                    "forge-package-rollback-uncovered",
                    format!(
                        "Forge package `{}` variant `{}` has an accepted update without rollback coverage",
                        package.package_id, package.variant
                    ),
                    Some(SOURCE_MANIFEST_PATH.to_string()),
                    "Accept the next Forge update with receipts enabled so rollback coverage is restored.",
                ));
            }
            None => {}
        }
    }

    let package_count = manifest.packages.len() as u64;
    let rollback_coverage_percent = if package_count == 0 {
        100
    } else {
        rollback_covered.saturating_mul(100) / package_count
    };

    vec![
        check_metric("stale_packages", stale_packages),
        check_metric(
            "accepted_local_update_packages",
            accepted_local_update_packages,
        ),
        check_metric("packages_without_accepted_update", packages_without_update),
        check_metric("oldest_update_age_days", oldest_update_age_days),
        check_metric("rollback_covered_packages", rollback_covered),
        check_metric("rollback_missing_packages", rollback_missing),
        check_metric("rollback_coverage_percent", rollback_coverage_percent),
    ]
}

fn forge_package_docs_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
    findings: &mut Vec<DxCheckFinding>,
) -> Vec<DxCheckMetric> {
    let mut present_docs = 0u64;
    let mut missing_docs = 0u64;

    for package in &manifest.packages {
        let doc_name = forge_package_doc_name(&package.package_id, &package.variant);
        let relative_doc_path = format!("{PACKAGE_DOCS_DIR}/{doc_name}");
        let doc_path = root.join(&relative_doc_path);

        match fs::metadata(&doc_path) {
            Ok(metadata) if metadata.is_file() && metadata.len() > 0 => present_docs += 1,
            Ok(_) => {
                missing_docs += 1;
                findings.push(check_finding(
                    DxSupplyChainSeverity::Low,
                    "forge-package-doc-empty",
                    format!(
                        "Forge package `{}` variant `{}` has an empty or non-file docs entry",
                        package.package_id, package.variant
                    ),
                    Some(relative_doc_path),
                    "Regenerate Forge package docs so source ownership and update behavior are reviewable.",
                ));
            }
            Err(_) => {
                missing_docs += 1;
                findings.push(check_finding(
                    DxSupplyChainSeverity::Low,
                    "forge-package-doc-missing",
                    format!(
                        "Forge package `{}` variant `{}` is missing package-facing docs",
                        package.package_id, package.variant
                    ),
                    Some(relative_doc_path),
                    "Regenerate Forge package docs so source ownership and update behavior are reviewable.",
                ));
            }
        }
    }

    let package_count = manifest.packages.len() as u64;
    let doc_coverage_percent = if package_count == 0 {
        100
    } else {
        present_docs.saturating_mul(100) / package_count
    };

    vec![
        check_metric("package_docs_present", present_docs),
        check_metric("package_docs_missing", missing_docs),
        check_metric("package_docs_coverage_percent", doc_coverage_percent),
    ]
}

fn forge_package_doc_name(package_id: &str, variant: &str) -> String {
    let package = package_id.replace('/', "-");
    if variant == "default" {
        format!("{package}.md")
    } else {
        format!("{package}--variant-{}.md", variant.replace('.', "-"))
    }
}

fn forge_auth_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
    findings: &mut Vec<DxCheckFinding>,
) -> Vec<DxCheckMetric> {
    let mut auth_packages = 0u64;
    let mut missing_env_examples = 0u64;
    let mut unsafe_redirect_defaults = 0u64;
    let mut stale_auth_receipts = 0u64;

    for package in manifest
        .packages
        .iter()
        .filter(|package| package.package_id == "auth/better-auth")
    {
        auth_packages += 1;

        if source_package_for_project_variant(&package.package_id, root, &package.variant)
            .map(|latest| {
                latest.version != package.version || latest.integrity_hash != package.integrity_hash
            })
            .unwrap_or(false)
        {
            stale_auth_receipts += 1;
            findings.push(check_finding(
                DxSupplyChainSeverity::Medium,
                "auth-google-stale-receipt",
                format!(
                    "auth/better-auth variant `{}` is behind the current curated source",
                    package.variant
                ),
                Some(SOURCE_MANIFEST_PATH.to_string()),
                "Run dx update auth/better-auth, review the receipt, and keep the OAuth package contract current.",
            ));
        }

        let Some(env_file) = package
            .files
            .iter()
            .find(|file| file.logical_path.as_deref() == Some("js/auth/better-auth/.env.example"))
        else {
            missing_env_examples += 1;
            findings.push(check_finding(
                DxSupplyChainSeverity::Medium,
                "auth-google-env-example-untracked",
                format!(
                    "auth/better-auth variant `{}` does not track its `.env.example` file",
                    package.variant
                ),
                Some(SOURCE_MANIFEST_PATH.to_string()),
                "Regenerate auth/better-auth so Forge receipts include the OAuth env example contract.",
            ));
            continue;
        };

        let env_path = root.join(&env_file.path);
        let relative_env_path = env_file.path.clone();
        let content = match fs::read_to_string(&env_path) {
            Ok(content) if !content.trim().is_empty() => content,
            _ => {
                missing_env_examples += 1;
                findings.push(check_finding(
                    DxSupplyChainSeverity::Medium,
                    "auth-google-env-example-missing",
                    format!(
                        "auth/better-auth variant `{}` is missing a usable `.env.example`",
                        package.variant
                    ),
                    Some(relative_env_path),
                    "Restore auth/better-auth/.env.example from the Forge receipt so required OAuth variables are visible.",
                ));
                continue;
            }
        };

        let env = parse_env_example(&content);
        for key in [
            "GOOGLE_CLIENT_ID",
            "GOOGLE_CLIENT_SECRET",
            "GOOGLE_REDIRECT_URI",
        ] {
            if !env.contains_key(key) {
                missing_env_examples += 1;
                findings.push(check_finding(
                    DxSupplyChainSeverity::Medium,
                    "auth-google-env-key-missing",
                    format!("auth/better-auth `.env.example` is missing `{key}`"),
                    Some(relative_env_path.clone()),
                    "Restore the complete auth/better-auth env example before launch.",
                ));
            }
        }

        if let Some(redirect_uri) = env.get("GOOGLE_REDIRECT_URI") {
            if !is_safe_google_redirect_uri(redirect_uri) {
                unsafe_redirect_defaults += 1;
                findings.push(check_finding(
                    DxSupplyChainSeverity::High,
                    "auth-google-unsafe-redirect-uri",
                    "auth/better-auth `.env.example` contains an unsafe GOOGLE_REDIRECT_URI default",
                    Some(relative_env_path.clone()),
                    "Use HTTPS for production redirects, keep local HTTP limited to localhost, and route callbacks through /auth/better-auth/callback.",
                ));
            }
        }

        if let Some(origin) = env.get("DX_GOOGLE_ALLOWED_REDIRECT_ORIGIN") {
            if !origin.trim().is_empty() && !is_safe_google_redirect_origin(origin) {
                unsafe_redirect_defaults += 1;
                findings.push(check_finding(
                    DxSupplyChainSeverity::High,
                    "auth-google-unsafe-allowed-origin",
                    "auth/better-auth `.env.example` contains an unsafe DX_GOOGLE_ALLOWED_REDIRECT_ORIGIN default",
                    Some(relative_env_path),
                    "Use an exact HTTPS production origin or a localhost origin for local development; never use wildcard redirect origins.",
                ));
            }
        }
    }

    vec![
        check_metric("auth_google_packages", auth_packages),
        check_metric("auth_google_missing_env_examples", missing_env_examples),
        check_metric(
            "auth_google_unsafe_redirect_defaults",
            unsafe_redirect_defaults,
        ),
        check_metric("auth_google_stale_receipts", stale_auth_receipts),
    ]
}

fn forge_better_auth_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
    findings: &mut Vec<DxCheckFinding>,
) -> Vec<DxCheckMetric> {
    let mut packages = 0u64;
    let mut missing_env_examples = 0u64;
    let mut unsafe_url_defaults = 0u64;
    let mut missing_metadata_files = 0u64;
    let mut stale_receipts = 0u64;

    for package in manifest
        .packages
        .iter()
        .filter(|package| package.package_id == "auth/better-auth")
    {
        packages += 1;

        if source_package_for_project_variant(&package.package_id, root, &package.variant)
            .map(|latest| {
                latest.version != package.version || latest.integrity_hash != package.integrity_hash
            })
            .unwrap_or(false)
        {
            stale_receipts += 1;
            findings.push(check_finding(
                DxSupplyChainSeverity::Medium,
                "auth-better-auth-stale-receipt",
                format!(
                    "auth/better-auth variant `{}` is behind the current curated source",
                    package.variant
                ),
                Some(SOURCE_MANIFEST_PATH.to_string()),
                "Run dx update auth/better-auth, review the receipt, and keep the Better Auth package contract current.",
            ));
        }

        if !package
            .files
            .iter()
            .any(|file| file.logical_path.as_deref() == Some("js/auth/better-auth/metadata.ts"))
        {
            missing_metadata_files += 1;
            findings.push(check_finding(
                DxSupplyChainSeverity::Medium,
                "auth-better-auth-metadata-untracked",
                format!(
                    "auth/better-auth variant `{}` does not track its discovery metadata file",
                    package.variant
                ),
                Some(SOURCE_MANIFEST_PATH.to_string()),
                "Regenerate auth/better-auth so DX CLI and host UIs can discover the package through metadata.ts.",
            ));
        }

        let Some(env_file) = package
            .files
            .iter()
            .find(|file| file.logical_path.as_deref() == Some("js/auth/better-auth/.env.example"))
        else {
            missing_env_examples += 1;
            findings.push(check_finding(
                DxSupplyChainSeverity::Medium,
                "auth-better-auth-env-example-untracked",
                format!(
                    "auth/better-auth variant `{}` does not track its `.env.example` file",
                    package.variant
                ),
                Some(SOURCE_MANIFEST_PATH.to_string()),
                "Regenerate auth/better-auth so Forge receipts include the Better Auth env contract.",
            ));
            continue;
        };

        let env_path = root.join(&env_file.path);
        let relative_env_path = env_file.path.clone();
        let content = match fs::read_to_string(&env_path) {
            Ok(content) if !content.trim().is_empty() => content,
            _ => {
                missing_env_examples += 1;
                findings.push(check_finding(
                    DxSupplyChainSeverity::Medium,
                    "auth-better-auth-env-example-missing",
                    format!(
                        "auth/better-auth variant `{}` is missing a usable `.env.example`",
                        package.variant
                    ),
                    Some(relative_env_path),
                    "Restore auth/better-auth/.env.example from the Forge receipt so required Better Auth variables are visible.",
                ));
                continue;
            }
        };

        let env = parse_env_example(&content);
        for key in ["BETTER_AUTH_SECRET", "BETTER_AUTH_URL"] {
            if !env.contains_key(key) {
                missing_env_examples += 1;
                findings.push(check_finding(
                    DxSupplyChainSeverity::Medium,
                    "auth-better-auth-env-key-missing",
                    format!("auth/better-auth `.env.example` is missing `{key}`"),
                    Some(relative_env_path.clone()),
                    "Restore the complete auth/better-auth env example before launch.",
                ));
            }
        }

        if let Some(base_url) = env.get("BETTER_AUTH_URL") {
            if !is_safe_auth_origin(base_url) {
                unsafe_url_defaults += 1;
                findings.push(check_finding(
                    DxSupplyChainSeverity::High,
                    "auth-better-auth-unsafe-base-url",
                    "auth/better-auth `.env.example` contains an unsafe BETTER_AUTH_URL default",
                    Some(relative_env_path.clone()),
                    "Use HTTPS for production, keep local HTTP limited to localhost, and avoid wildcard or credential-bearing auth origins.",
                ));
            }
        }

        if let Some(origins) = env.get("BETTER_AUTH_TRUSTED_ORIGINS") {
            for origin in origins
                .split([',', ' '])
                .filter(|origin| !origin.trim().is_empty())
            {
                if !is_safe_auth_origin(origin) {
                    unsafe_url_defaults += 1;
                    findings.push(check_finding(
                        DxSupplyChainSeverity::High,
                        "auth-better-auth-unsafe-trusted-origin",
                        "auth/better-auth `.env.example` contains an unsafe BETTER_AUTH_TRUSTED_ORIGINS default",
                        Some(relative_env_path.clone()),
                        "Use exact HTTPS production origins or localhost origins for local development; never use wildcard trusted origins.",
                    ));
                }
            }
        }
    }

    vec![
        check_metric("auth_better_auth_packages", packages),
        check_metric(
            "auth_better_auth_missing_env_examples",
            missing_env_examples,
        ),
        check_metric("auth_better_auth_missing_metadata", missing_metadata_files),
        check_metric("auth_better_auth_unsafe_url_defaults", unsafe_url_defaults),
        check_metric("auth_better_auth_stale_receipts", stale_receipts),
    ]
}

fn forge_supabase_package_metrics(
    root: &Path,
    manifest: &DxSourceManifest,
    findings: &mut Vec<DxCheckFinding>,
) -> Vec<DxCheckMetric> {
    let mut packages = 0u64;
    let mut missing_env_examples = 0u64;
    let mut secret_leaks = 0u64;
    let mut stale_receipts = 0u64;

    for package in manifest
        .packages
        .iter()
        .filter(|package| package.package_id == "supabase/client")
    {
        packages += 1;

        if source_package_for_project_variant(&package.package_id, root, &package.variant)
            .map(|latest| {
                latest.version != package.version || latest.integrity_hash != package.integrity_hash
            })
            .unwrap_or(false)
        {
            stale_receipts += 1;
            findings.push(check_finding(
                DxSupplyChainSeverity::Medium,
                "supabase-client-stale-receipt",
                format!(
                    "supabase/client variant `{}` is behind the current curated source",
                    package.variant
                ),
                Some(SOURCE_MANIFEST_PATH.to_string()),
                "Run dx update supabase/client, review the receipt, and keep the Supabase package contract current.",
            ));
        }

        let Some(env_file) = package
            .files
            .iter()
            .find(|file| file.logical_path.as_deref() == Some("js/supabase/.env.example"))
        else {
            missing_env_examples += 1;
            findings.push(check_finding(
                DxSupplyChainSeverity::Medium,
                "supabase-client-env-example-untracked",
                format!(
                    "supabase/client variant `{}` does not track its `.env.example` file",
                    package.variant
                ),
                Some(SOURCE_MANIFEST_PATH.to_string()),
                "Regenerate supabase/client so Forge receipts include the public Supabase env contract.",
            ));
            continue;
        };

        let env_path = root.join(&env_file.path);
        let relative_env_path = env_file.path.clone();
        let content = match fs::read_to_string(&env_path) {
            Ok(content) if !content.trim().is_empty() => content,
            _ => {
                missing_env_examples += 1;
                findings.push(check_finding(
                    DxSupplyChainSeverity::Medium,
                    "supabase-client-env-example-missing",
                    format!(
                        "supabase/client variant `{}` is missing a usable `.env.example`",
                        package.variant
                    ),
                    Some(relative_env_path),
                    "Restore lib/supabase/.env.example from the Forge receipt so required public Supabase variables are visible.",
                ));
                continue;
            }
        };

        let env = parse_env_example(&content);
        for key in [
            "NEXT_PUBLIC_SUPABASE_URL",
            "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY",
        ] {
            if !env.contains_key(key) {
                missing_env_examples += 1;
                findings.push(check_finding(
                    DxSupplyChainSeverity::Medium,
                    "supabase-client-env-key-missing",
                    format!("supabase/client `.env.example` is missing `{key}`"),
                    Some(relative_env_path.clone()),
                    "Restore the complete supabase/client env example before launch.",
                ));
            }
        }

        for key in ["SUPABASE_SERVICE_ROLE_KEY", "SUPABASE_SERVICE_KEY"] {
            if env.contains_key(key) {
                secret_leaks += 1;
                findings.push(check_finding(
                    DxSupplyChainSeverity::High,
                    "supabase-client-secret-env-leak",
                    format!("supabase/client `.env.example` exposes `{key}`"),
                    Some(relative_env_path.clone()),
                    "Keep service-role secrets in server-only deployment secret stores, never in public Supabase client templates.",
                ));
            }
        }
    }

    vec![
        check_metric("supabase_client_packages", packages),
        check_metric("supabase_client_missing_env_examples", missing_env_examples),
        check_metric("supabase_client_secret_leaks", secret_leaks),
        check_metric("supabase_client_stale_receipts", stale_receipts),
    ]
}

fn parse_env_example(content: &str) -> BTreeMap<String, String> {
    content
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                return None;
            }
            let (key, value) = line.split_once('=')?;
            Some((key.trim().to_string(), value.trim().to_string()))
        })
        .collect()
}

fn is_safe_google_redirect_uri(value: &str) -> bool {
    is_safe_google_url(value)
        && url_path(value).is_some_and(|path| path.starts_with("/auth/better-auth/callback"))
}

fn is_safe_google_redirect_origin(value: &str) -> bool {
    is_safe_google_url(value)
        && match url_path(value) {
            Some(path) => path == "/" || path.is_empty(),
            None => true,
        }
}

fn is_safe_google_url(value: &str) -> bool {
    is_safe_auth_origin(value)
}

fn is_safe_auth_origin(value: &str) -> bool {
    let value = value.trim();
    if value.is_empty() || value.contains('*') {
        return false;
    }
    let Some((scheme, rest)) = value.split_once("://") else {
        return false;
    };
    if rest.contains('@') {
        return false;
    }
    let scheme = scheme.to_ascii_lowercase();
    match scheme.as_str() {
        "https" => true,
        "http" => url_host(value).is_some_and(is_localhost),
        _ => false,
    }
}

fn url_host(value: &str) -> Option<&str> {
    let (_, rest) = value.trim().split_once("://")?;
    let host_port = rest.split(['/', '?', '#']).next().unwrap_or_default();
    if host_port.starts_with('[') {
        return host_port
            .strip_prefix('[')
            .and_then(|host| host.split_once(']').map(|(host, _)| host));
    }
    Some(host_port.split(':').next().unwrap_or(host_port))
}

fn url_path(value: &str) -> Option<&str> {
    let (_, rest) = value.trim().split_once("://")?;
    rest.find('/').map(|index| &rest[index..])
}
