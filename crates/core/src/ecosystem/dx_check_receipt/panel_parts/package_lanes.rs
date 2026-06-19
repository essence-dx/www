fn package_lane_rows(root: &Path) -> Vec<DxCheckPanelPackageLaneRow> {
    let package_status = read_forge_package_status(root);
    let package_status = package_status.as_ref();
    let mut rows = Vec::new();
    rows.extend(authentication_package_lane_row(root, package_status));
    rows.extend(state_management_package_lane_row(root, package_status));
    rows.extend(data_fetching_cache_package_lane_row(root, package_status));
    rows.extend(reactive_store_package_lane_row(root, package_status));
    rows.extend(database_orm_package_lane_row(root, package_status));
    rows.extend(forms_package_lane_row(root, package_status));
    rows.extend(backend_platform_client_package_lane_row(root, package_status));
    rows.extend(realtime_app_database_package_lane_row(root, package_status));
    rows.extend(payments_package_lane_row(root, package_status));
    rows.extend(motion_animation_package_lane_row(root, package_status));
    rows.extend(validation_schemas_package_lane_row(root, package_status));
    rows.extend(type_safe_api_package_lane_row(root, package_status));
    rows.extend(ui_components_package_lane_row(root, package_status));
    rows.extend(internationalization_package_lane_row(root, package_status));
    rows.extend(documentation_system_package_lane_row(root, package_status));
    rows.extend(markdown_mdx_content_package_lane_row(root, package_status));
    rows.extend(ai_sdk_package_lane_row(root, package_status));
    rows.extend(three_scene_system_package_lane_row(root, package_status));
    rows.extend(webassembly_bridge_package_lane_row(root, package_status));
    rows.extend(automation_connectors_package_lane_row(root, package_status));
    rows
}

struct ForgePackageStatusReadModel {
    package_lane_visibility: Vec<serde_json::Value>,
    package_lane_visibility_by_id: std::collections::BTreeMap<String, usize>,
}

impl ForgePackageStatusReadModel {
    fn new(value: serde_json::Value) -> Self {
        let package_lane_visibility = value_at(&value, &["package_lane_visibility"])
            .and_then(|entries| entries.as_array())
            .cloned()
            .unwrap_or_default();

        Self::from_visibility_entries(package_lane_visibility)
    }

    fn from_visibility_entries(package_lane_visibility: Vec<serde_json::Value>) -> Self {
        let mut package_lane_visibility_by_id = std::collections::BTreeMap::new();
        for (index, entry) in package_lane_visibility.iter().enumerate() {
            if let Some(package_id) = json_text(entry, &["package_id"]) {
                package_lane_visibility_by_id
                    .entry(package_id.to_string())
                    .or_insert(index);
            }
        }

        Self {
            package_lane_visibility,
            package_lane_visibility_by_id,
        }
    }

    fn visibility_entry(&self, package_id: &str) -> Option<&serde_json::Value> {
        let index = *self.package_lane_visibility_by_id.get(package_id)?;
        self.package_lane_visibility.get(index)
    }
}

fn read_forge_package_status(root: &Path) -> Option<ForgePackageStatusReadModel> {
    const FORGE_PACKAGE_STATUS_MACHINE_PATH: &str = ".dx/www/forge-package-status.machine";

    let package_status_path = forge_package_status_path();
    let source_path = root.join(package_status_path);
    if !source_path.exists() {
        return None;
    }

    if let Some(package_status) =
        super::super::forge_package_status_machine::read_forge_package_status_machine_cache(root)
    {
        return Some(ForgePackageStatusReadModel::from_visibility_entries(
            package_status,
        ));
    }

    if let Some(package_status) = super::super::json_receipt_machine::read_json_receipt_machine_alias(
        root,
        package_status_path,
        FORGE_PACKAGE_STATUS_MACHINE_PATH,
    ) {
        return Some(ForgePackageStatusReadModel::new(package_status));
    }

    fs::read(source_path)
        .ok()
        .and_then(|bytes| serde_json::from_slice::<serde_json::Value>(&bytes).ok())
        .map(ForgePackageStatusReadModel::new)
}

fn forge_package_status_path() -> &'static str {
    AUTHENTICATION_PACKAGE_STATUS_PATH
}

fn authentication_package_lane_row(
    root: &Path,
    package_status: Option<&ForgePackageStatusReadModel>,
) -> Option<DxCheckPanelPackageLaneRow> {
    let manifest_package_present = source_manifest_has_package(root, AUTHENTICATION_PACKAGE_ID);
    let Some(package_status) = package_status else {
        return manifest_package_present.then(|| {
            authentication_missing_receipt_row(
                "Restore the Authentication package-status row and auth-better-auth receipt so Studio and Zed can render source-owned package-lane status.",
            )
        });
    };
    let package = package_lane_visibility_entry(package_status, AUTHENTICATION_PACKAGE_ID)?;
    let package_id = json_text(package, &["package_id"]).unwrap_or(AUTHENTICATION_PACKAGE_ID);
    if package_id != AUTHENTICATION_PACKAGE_ID && !manifest_package_present {
        return None;
    }

    let visibility_status = json_text(package, &["status"])
        .or_else(|| json_text(package, &["current_status"]))
        .unwrap_or("present");
    let package_receipt_path = json_text(package, &["package_receipt_path"])
        .unwrap_or(AUTHENTICATION_PACKAGE_RECEIPT_PATH);
    let receipt_present = u64::from(project_file_path(root, package_receipt_path).is_some());
    let missing_receipt = u64::from(receipt_present == 0);
    let receipt_status = if receipt_present == 0 {
        "missing-receipt"
    } else {
        json_text(package, &["receipt_status"]).unwrap_or(visibility_status)
    };
    let selected_surfaces = selected_package_surfaces(package);
    let blocked_surfaces = u64::from(matches!(visibility_status, "blocked"))
        + u64::from(matches!(receipt_status, "blocked"))
        + json_array_len(package, &["blocked_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "blocked")
            .count() as u64;
    let unsupported_surfaces = u64::from(matches!(visibility_status, "unsupported-surface"))
        + u64::from(matches!(receipt_status, "unsupported-surface"))
        + json_array_len(package, &["unsupported_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "unsupported-surface")
            .count() as u64;
    let hash_manifest_present = u64::from(has_sha256_file_hashes(package));
    let hash_mismatches = count_sha256_file_hash_mismatches(root, package);
    let mut receipt_hash_refresh = package_lane_hash_refresh(package);
    if let Some(refresh) = receipt_hash_refresh.as_mut() {
        if refresh.zed_visibility.is_empty() {
            refresh.zed_visibility =
                MOTION_ANIMATION_RECEIPT_HASH_REFRESH_ZED_VISIBILITY.to_string();
        }
    }
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);
    let stale_receipt = u64::from(
        matches!(visibility_status, "stale")
            || matches!(receipt_status, "stale")
            || hash_mismatches > 0
            || refresh_stale > 0,
    );
    let dx_style_compatibility_present = u64::from(dx_style_compatibility_is_present(package));
    let dx_style_compatibility_missing = u64::from(dx_style_compatibility_present == 0);
    let status = if missing_receipt > 0 {
        "missing-receipt"
    } else {
        state_management_effective_status(
            visibility_status,
            stale_receipt,
            blocked_surfaces,
            unsupported_surfaces,
        )
    };

    Some(DxCheckPanelPackageLaneRow {
        package_id: AUTHENTICATION_PACKAGE_ID.to_string(),
        official_package_name: json_text(package, &["official_package_name"])
            .unwrap_or(AUTHENTICATION_OFFICIAL_NAME)
            .to_string(),
        upstream_package: json_text(package, &["upstream_package"])
            .unwrap_or(AUTHENTICATION_UPSTREAM_PACKAGE)
            .to_string(),
        upstream_version: json_text(package, &["upstream_version"])
            .unwrap_or(AUTHENTICATION_UPSTREAM_VERSION)
            .to_string(),
        source_mirror: json_text(package, &["source_mirror"])
            .unwrap_or(AUTHENTICATION_SOURCE_MIRROR)
            .to_string(),
        status: status.to_string(),
        receipt_status: if stale_receipt > 0 {
            "stale".to_string()
        } else {
            receipt_status.to_string()
        },
        package_receipt_path: package_receipt_path.to_string(),
        status_vocabulary: authentication_status_vocabulary(package),
        selected_surfaces,
        receipt_hash_refresh: receipt_hash_refresh.clone(),
        metrics: authentication_metric_rows(
            1,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
            refresh_current,
            refresh_stale,
            refresh_missing,
            dx_style_compatibility_present,
            dx_style_compatibility_missing,
        ),
        runtime_limitations: json_string_array(package, &["runtime_limitations"]),
        next_action: authentication_next_action(
            status,
            refresh_stale,
            refresh_missing,
            dx_style_compatibility_missing,
        )
        .to_string(),
    })
}

fn authentication_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow {
    DxCheckPanelPackageLaneRow {
        package_id: AUTHENTICATION_PACKAGE_ID.to_string(),
        official_package_name: AUTHENTICATION_OFFICIAL_NAME.to_string(),
        upstream_package: AUTHENTICATION_UPSTREAM_PACKAGE.to_string(),
        upstream_version: AUTHENTICATION_UPSTREAM_VERSION.to_string(),
        source_mirror: AUTHENTICATION_SOURCE_MIRROR.to_string(),
        status: "missing-receipt".to_string(),
        receipt_status: "missing-receipt".to_string(),
        package_receipt_path: AUTHENTICATION_PACKAGE_RECEIPT_PATH.to_string(),
        status_vocabulary: AUTHENTICATION_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect(),
        selected_surfaces: Vec::new(),
        receipt_hash_refresh: None,
        metrics: authentication_metric_rows(1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1),
        runtime_limitations: vec![
            "SOURCE-ONLY: missing package-status or auth-better-auth receipt blocks Authentication package-lane visibility.".to_string(),
        ],
        next_action: next_action.to_string(),
    }
}

fn state_management_package_lane_row(
    root: &Path,
    package_status: Option<&ForgePackageStatusReadModel>,
) -> Option<DxCheckPanelPackageLaneRow> {
    let manifest_package_present = source_manifest_has_package(root, STATE_MANAGEMENT_PACKAGE_ID);
    let package_status_entry = package_status.and_then(|package_status| {
        package_lane_visibility_entry(package_status, STATE_MANAGEMENT_PACKAGE_ID).cloned()
    });
    let package = if let Some(package) = package_status_entry {
        package
    } else {
        let package_receipt = root.join(STATE_MANAGEMENT_PACKAGE_RECEIPT_PATH);
        if !package_receipt.is_file() {
            return manifest_package_present.then(|| {
                state_management_missing_receipt_row(
                    "Restore the State Management package-status row and package receipt so Studio and Zed can render helper freshness.",
                )
            });
        }

        let receipt = fs::read(&package_receipt)
            .ok()
            .and_then(|bytes| serde_json::from_slice::<serde_json::Value>(&bytes).ok())?;
        receipt.get("package").unwrap_or(&receipt).clone()
    };
    let package_id = json_text(&package, &["package_id"]).unwrap_or(STATE_MANAGEMENT_PACKAGE_ID);
    if package_id != STATE_MANAGEMENT_PACKAGE_ID && !manifest_package_present {
        return None;
    }

    let visibility = package.get("dx_check_visibility").unwrap_or(&package);
    let visibility_status = json_text(visibility, &["status"])
        .or_else(|| json_text(visibility, &["current_status"]))
        .unwrap_or("present");
    let package_receipt_path = json_text(&package, &["package_receipt_path"])
        .unwrap_or(STATE_MANAGEMENT_PACKAGE_RECEIPT_PATH);
    let receipt_present = u64::from(project_file_path(root, package_receipt_path).is_some());
    let missing_receipt = u64::from(receipt_present == 0);
    let receipt_status = if receipt_present == 0 {
        "missing-receipt"
    } else {
        json_text(visibility, &["receipt_status"]).unwrap_or(visibility_status)
    };
    let selected_surfaces = selected_package_surfaces(visibility);
    let blocked_surfaces = u64::from(matches!(visibility_status, "blocked"))
        + u64::from(matches!(receipt_status, "blocked"))
        + json_array_len(visibility, &["blocked_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "blocked")
            .count() as u64;
    let unsupported_surfaces = u64::from(matches!(visibility_status, "unsupported-surface"))
        + u64::from(matches!(receipt_status, "unsupported-surface"))
        + json_array_len(visibility, &["unsupported_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "unsupported-surface")
            .count() as u64;
    let stale_receipt =
        u64::from(matches!(visibility_status, "stale") || matches!(receipt_status, "stale"));
    let dx_style_compatibility_present = u64::from(dx_style_compatibility_is_present(visibility));
    let dx_style_compatibility_missing = u64::from(dx_style_compatibility_present == 0);
    let receipt_hash_refresh = package_lane_hash_refresh(&package);
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(&package);
    let stale_receipt = u64::from(
        stale_receipt > 0
            || matches!(visibility_status, "stale")
            || matches!(receipt_status, "stale")
            || refresh_stale > 0,
    );
    let status = if missing_receipt > 0 {
        "missing-receipt"
    } else {
        state_management_effective_status(
            visibility_status,
            stale_receipt,
            blocked_surfaces,
            unsupported_surfaces,
        )
    };

    Some(DxCheckPanelPackageLaneRow {
        package_id: STATE_MANAGEMENT_PACKAGE_ID.to_string(),
        official_package_name: json_text(&package, &["official_package_name"])
            .unwrap_or(STATE_MANAGEMENT_OFFICIAL_NAME)
            .to_string(),
        upstream_package: json_text(&package, &["upstream_package"])
            .unwrap_or(STATE_MANAGEMENT_UPSTREAM_PACKAGE)
            .to_string(),
        upstream_version: json_text(&package, &["upstream_version"])
            .or_else(|| json_text(&package, &["provenance", "upstream_reference"]))
            .map(|value| value.strip_prefix("npm:zustand@").unwrap_or(value))
            .unwrap_or(STATE_MANAGEMENT_UPSTREAM_VERSION)
            .to_string(),
        source_mirror: json_text(&package, &["source_mirror"])
            .unwrap_or(STATE_MANAGEMENT_SOURCE_MIRROR)
            .to_string(),
        status: status.to_string(),
        receipt_status: if stale_receipt > 0 {
            "stale".to_string()
        } else {
            receipt_status.to_string()
        },
        package_receipt_path: package_receipt_path.to_string(),
        status_vocabulary: status_vocabulary(visibility),
        selected_surfaces,
        metrics: state_management_metric_rows(
            1,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            refresh_current,
            refresh_stale,
            refresh_missing,
            dx_style_compatibility_present,
            dx_style_compatibility_missing,
        ),
        receipt_hash_refresh,
        runtime_limitations: json_string_array(visibility, &["runtime_limitations"]),
        next_action: state_management_next_action(
            status,
            refresh_stale,
            refresh_missing,
            dx_style_compatibility_missing,
        )
        .to_string(),
    })
}

fn state_management_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow {
    DxCheckPanelPackageLaneRow {
        package_id: STATE_MANAGEMENT_PACKAGE_ID.to_string(),
        official_package_name: STATE_MANAGEMENT_OFFICIAL_NAME.to_string(),
        upstream_package: STATE_MANAGEMENT_UPSTREAM_PACKAGE.to_string(),
        upstream_version: STATE_MANAGEMENT_UPSTREAM_VERSION.to_string(),
        source_mirror: STATE_MANAGEMENT_SOURCE_MIRROR.to_string(),
        status: "missing-receipt".to_string(),
        receipt_status: "missing-receipt".to_string(),
        package_receipt_path: STATE_MANAGEMENT_PACKAGE_RECEIPT_PATH.to_string(),
        status_vocabulary: STATE_MANAGEMENT_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect(),
        selected_surfaces: Vec::new(),
        metrics: state_management_metric_rows(1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1),
        receipt_hash_refresh: None,
        runtime_limitations: vec![
            "SOURCE-ONLY: missing package receipt blocks package-lane visibility.".to_string(),
        ],
        next_action: next_action.to_string(),
    }
}

fn data_fetching_cache_package_lane_row(
    root: &Path,
    package_status: Option<&ForgePackageStatusReadModel>,
) -> Option<DxCheckPanelPackageLaneRow> {
    let manifest_package_present =
        source_manifest_has_package(root, DATA_FETCHING_CACHE_PACKAGE_ID);
    let Some(package_status) = package_status else {
        return manifest_package_present.then(|| {
            data_fetching_cache_missing_receipt_row(
                "Restore the Data Fetching & Cache package-status row and dashboard workflow receipt so Studio and Zed can render helper freshness.",
            )
        });
    };
    let package = package_lane_visibility_entry(package_status, DATA_FETCHING_CACHE_PACKAGE_ID)?;
    let package_id = json_text(package, &["package_id"]).unwrap_or(DATA_FETCHING_CACHE_PACKAGE_ID);
    if package_id != DATA_FETCHING_CACHE_PACKAGE_ID && !manifest_package_present {
        return None;
    }

    let visibility_status = json_text(package, &["status"])
        .or_else(|| json_text(package, &["current_status"]))
        .unwrap_or("present");
    let package_receipt_path = json_text(package, &["package_receipt_path"])
        .unwrap_or(DATA_FETCHING_CACHE_PACKAGE_RECEIPT_PATH);
    let receipt_present = u64::from(project_file_path(root, package_receipt_path).is_some());
    let missing_receipt = u64::from(receipt_present == 0);
    let receipt_status = if receipt_present == 0 {
        "missing-receipt"
    } else {
        json_text(package, &["receipt_status"]).unwrap_or(visibility_status)
    };
    let selected_surfaces = selected_package_surfaces(package);
    let blocked_surfaces = u64::from(matches!(visibility_status, "blocked"))
        + u64::from(matches!(receipt_status, "blocked"))
        + json_array_len(package, &["blocked_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "blocked")
            .count() as u64;
    let unsupported_surfaces = u64::from(matches!(visibility_status, "unsupported-surface"))
        + u64::from(matches!(receipt_status, "unsupported-surface"))
        + json_array_len(package, &["unsupported_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "unsupported-surface")
            .count() as u64;
    let hash_manifest_present = u64::from(has_sha256_file_hashes(package));
    let hash_mismatches = count_sha256_file_hash_mismatches(root, package);
    let dx_style_compatibility_present = u64::from(dx_style_compatibility_is_present(package));
    let dx_style_compatibility_missing = u64::from(dx_style_compatibility_present == 0);
    let receipt_hash_refresh = package_lane_hash_refresh(package);
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);
    let stale_receipt = u64::from(
        matches!(visibility_status, "stale")
            || matches!(receipt_status, "stale")
            || hash_mismatches > 0
            || refresh_stale > 0,
    );
    let status = if missing_receipt > 0 {
        "missing-receipt"
    } else {
        state_management_effective_status(
            visibility_status,
            stale_receipt,
            blocked_surfaces,
            unsupported_surfaces,
        )
    };

    Some(DxCheckPanelPackageLaneRow {
        package_id: DATA_FETCHING_CACHE_PACKAGE_ID.to_string(),
        official_package_name: json_text(package, &["official_package_name"])
            .unwrap_or(DATA_FETCHING_CACHE_OFFICIAL_NAME)
            .to_string(),
        upstream_package: json_text(package, &["upstream_package"])
            .unwrap_or(DATA_FETCHING_CACHE_UPSTREAM_PACKAGE)
            .to_string(),
        upstream_version: json_text(package, &["upstream_version"])
            .unwrap_or(DATA_FETCHING_CACHE_UPSTREAM_VERSION)
            .to_string(),
        source_mirror: json_text(package, &["source_mirror"])
            .unwrap_or(DATA_FETCHING_CACHE_SOURCE_MIRROR)
            .to_string(),
        status: status.to_string(),
        receipt_status: if stale_receipt > 0 {
            "stale".to_string()
        } else {
            receipt_status.to_string()
        },
        package_receipt_path: package_receipt_path.to_string(),
        status_vocabulary: data_fetching_cache_status_vocabulary(package),
        selected_surfaces,
        receipt_hash_refresh,
        metrics: data_fetching_cache_metric_rows(
            1,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
            dx_style_compatibility_present,
            dx_style_compatibility_missing,
            refresh_current,
            refresh_stale,
            refresh_missing,
        ),
        runtime_limitations: data_fetching_cache_runtime_limitations(package),
        next_action: data_fetching_cache_next_action(status, refresh_stale, refresh_missing)
            .to_string(),
    })
}

fn data_fetching_cache_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow {
    DxCheckPanelPackageLaneRow {
        package_id: DATA_FETCHING_CACHE_PACKAGE_ID.to_string(),
        official_package_name: DATA_FETCHING_CACHE_OFFICIAL_NAME.to_string(),
        upstream_package: DATA_FETCHING_CACHE_UPSTREAM_PACKAGE.to_string(),
        upstream_version: DATA_FETCHING_CACHE_UPSTREAM_VERSION.to_string(),
        source_mirror: DATA_FETCHING_CACHE_SOURCE_MIRROR.to_string(),
        status: "missing-receipt".to_string(),
        receipt_status: "missing-receipt".to_string(),
        package_receipt_path: DATA_FETCHING_CACHE_PACKAGE_RECEIPT_PATH.to_string(),
        status_vocabulary: DATA_FETCHING_CACHE_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect(),
        selected_surfaces: Vec::new(),
        metrics: data_fetching_cache_metric_rows(1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1),
        receipt_hash_refresh: None,
        runtime_limitations: vec![
            "SOURCE-ONLY: missing package-status or dashboard workflow receipt blocks Data Fetching & Cache package-lane visibility.".to_string(),
        ],
        next_action: next_action.to_string(),
    }
}

fn reactive_store_package_lane_row(
    root: &Path,
    package_status: Option<&ForgePackageStatusReadModel>,
) -> Option<DxCheckPanelPackageLaneRow> {
    let manifest_package_present = source_manifest_has_package(root, REACTIVE_STORE_PACKAGE_ID);
    let package_receipt = root.join(REACTIVE_STORE_PACKAGE_RECEIPT_PATH);
    if !package_receipt.is_file() {
        return manifest_package_present.then(|| {
            reactive_store_missing_receipt_row(
                "Restore the Reactive Store package receipt so Studio and Zed can render hash-backed package-lane status.",
            )
        });
    }

    let receipt = fs::read(&package_receipt)
        .ok()
        .and_then(|bytes| serde_json::from_slice::<serde_json::Value>(&bytes).ok())?;
    let package = receipt.get("package").unwrap_or(&receipt);
    let package_status_visibility = package_status
        .and_then(|status| package_lane_visibility_entry(status, REACTIVE_STORE_PACKAGE_ID));
    let hash_refresh_source = package_status_visibility.unwrap_or(package);
    let package_id = json_text(package, &["package_id"]).unwrap_or(REACTIVE_STORE_PACKAGE_ID);
    if package_id != REACTIVE_STORE_PACKAGE_ID && !manifest_package_present {
        return None;
    }

    let visibility = package.get("dx_check_visibility").unwrap_or(package);
    let visibility_status = json_text(visibility, &["status"])
        .or_else(|| json_text(visibility, &["current_status"]))
        .or_else(|| json_text(package, &["status"]))
        .unwrap_or("present");
    let receipt_status = json_text(visibility, &["receipt_status"]).unwrap_or(visibility_status);
    let selected_surfaces = reactive_store_selected_surfaces(visibility, package);
    let blocked_surfaces = u64::from(matches!(visibility_status, "blocked"))
        + u64::from(matches!(receipt_status, "blocked"))
        + json_array_len(visibility, &["blocked_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "blocked")
            .count() as u64;
    let unsupported_surfaces = u64::from(matches!(visibility_status, "unsupported-surface"))
        + u64::from(matches!(receipt_status, "unsupported-surface"))
        + json_array_len(visibility, &["unsupported_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "unsupported-surface")
            .count() as u64;
    let hash_manifest_present = u64::from(has_sha256_file_hashes(package));
    let hash_mismatches = count_sha256_file_hash_mismatches(root, package);
    let receipt_hash_refresh = package_lane_hash_refresh(hash_refresh_source);
    let (refresh_current, refresh_stale, refresh_missing) =
        receipt_hash_refresh_counts(hash_refresh_source);
    let stale_receipt = u64::from(
        matches!(visibility_status, "stale")
            || matches!(receipt_status, "stale")
            || hash_mismatches > 0
            || refresh_stale > 0,
    );
    let status = state_management_effective_status(
        visibility_status,
        stale_receipt,
        blocked_surfaces,
        unsupported_surfaces,
    );

    Some(DxCheckPanelPackageLaneRow {
        package_id: REACTIVE_STORE_PACKAGE_ID.to_string(),
        official_package_name: json_text(package, &["official_package_name"])
            .unwrap_or(REACTIVE_STORE_OFFICIAL_NAME)
            .to_string(),
        upstream_package: json_text(package, &["upstream_package"])
            .or_else(|| json_text(package, &["provenance", "upstream_package"]))
            .unwrap_or(REACTIVE_STORE_UPSTREAM_PACKAGE)
            .to_string(),
        upstream_version: json_text(package, &["upstream_version"])
            .or_else(|| json_text(package, &["provenance", "upstream_version"]))
            .unwrap_or(REACTIVE_STORE_UPSTREAM_VERSION)
            .to_string(),
        source_mirror: json_text(package, &["source_mirror"])
            .or_else(|| json_text(package, &["provenance", "source_mirror"]))
            .unwrap_or(REACTIVE_STORE_SOURCE_MIRROR)
            .to_string(),
        status: status.to_string(),
        receipt_status: if stale_receipt > 0 {
            "stale".to_string()
        } else {
            receipt_status.to_string()
        },
        package_receipt_path: REACTIVE_STORE_PACKAGE_RECEIPT_PATH.to_string(),
        status_vocabulary: reactive_store_status_vocabulary(visibility),
        selected_surfaces,
        receipt_hash_refresh,
        metrics: reactive_store_metric_rows(
            1,
            1,
            stale_receipt,
            0,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
            refresh_current,
            refresh_stale,
            refresh_missing,
        ),
        runtime_limitations: reactive_store_runtime_limitations(visibility, package),
        next_action: reactive_store_next_action(status, refresh_stale, refresh_missing).to_string(),
    })
}

fn reactive_store_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow {
    DxCheckPanelPackageLaneRow {
        package_id: REACTIVE_STORE_PACKAGE_ID.to_string(),
        official_package_name: REACTIVE_STORE_OFFICIAL_NAME.to_string(),
        upstream_package: REACTIVE_STORE_UPSTREAM_PACKAGE.to_string(),
        upstream_version: REACTIVE_STORE_UPSTREAM_VERSION.to_string(),
        source_mirror: REACTIVE_STORE_SOURCE_MIRROR.to_string(),
        status: "missing-receipt".to_string(),
        receipt_status: "missing-receipt".to_string(),
        package_receipt_path: REACTIVE_STORE_PACKAGE_RECEIPT_PATH.to_string(),
        status_vocabulary: REACTIVE_STORE_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect(),
        selected_surfaces: Vec::new(),
        metrics: reactive_store_metric_rows(1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1),
        receipt_hash_refresh: None,
        runtime_limitations: vec![
            "SOURCE-ONLY: missing package receipt blocks Reactive Store package-lane visibility."
                .to_string(),
        ],
        next_action: next_action.to_string(),
    }
}

fn database_orm_package_lane_row(
    root: &Path,
    package_status: Option<&ForgePackageStatusReadModel>,
) -> Option<DxCheckPanelPackageLaneRow> {
    let manifest_package_present = source_manifest_has_package(root, DATABASE_ORM_PACKAGE_ID);
    let Some(package_status) = package_status else {
        return manifest_package_present.then(|| {
            database_orm_missing_receipt_row(
                "Restore the Database ORM package-status row and dashboard workflow receipt so Studio and Zed can render package-lane style visibility.",
            )
        });
    };
    let package = package_lane_visibility_entry(package_status, DATABASE_ORM_PACKAGE_ID)?;
    let package_id = json_text(package, &["package_id"]).unwrap_or(DATABASE_ORM_PACKAGE_ID);
    if package_id != DATABASE_ORM_PACKAGE_ID && !manifest_package_present {
        return None;
    }

    let visibility_status = json_text(package, &["status"])
        .or_else(|| json_text(package, &["current_status"]))
        .unwrap_or("present");
    let package_receipt_path =
        json_text(package, &["package_receipt_path"]).unwrap_or(DATABASE_ORM_PACKAGE_RECEIPT_PATH);
    let receipt_present = u64::from(project_file_path(root, package_receipt_path).is_some());
    let missing_receipt = u64::from(receipt_present == 0);
    let receipt_status = if receipt_present == 0 {
        "missing-receipt"
    } else {
        json_text(package, &["receipt_status"]).unwrap_or(visibility_status)
    };
    let selected_surfaces = selected_package_surfaces(package);
    let blocked_surfaces = u64::from(matches!(visibility_status, "blocked"))
        + u64::from(matches!(receipt_status, "blocked"))
        + json_array_len(package, &["blocked_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "blocked")
            .count() as u64;
    let unsupported_surfaces = u64::from(matches!(visibility_status, "unsupported-surface"))
        + u64::from(matches!(receipt_status, "unsupported-surface"))
        + json_array_len(package, &["unsupported_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "unsupported-surface")
            .count() as u64;
    let hash_manifest_present =
        u64::from(has_sha256_file_hashes(package) || has_sha256_source_hashes(package));
    let hash_mismatches = count_sha256_file_hash_mismatches(root, package)
        + count_sha256_source_hash_mismatches(root, package);
    let receipt_hash_refresh = package_lane_hash_refresh(package);
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);
    let stale_receipt = u64::from(
        matches!(visibility_status, "stale")
            || matches!(receipt_status, "stale")
            || hash_mismatches > 0
            || refresh_stale > 0,
    );
    let dx_style_compatibility_present = u64::from(dx_style_compatibility_is_present(package));
    let dx_style_compatibility_missing = u64::from(dx_style_compatibility_present == 0);
    let status = if missing_receipt > 0 {
        "missing-receipt"
    } else {
        state_management_effective_status(
            visibility_status,
            stale_receipt,
            blocked_surfaces,
            unsupported_surfaces,
        )
    };

    Some(DxCheckPanelPackageLaneRow {
        package_id: DATABASE_ORM_PACKAGE_ID.to_string(),
        official_package_name: json_text(package, &["official_package_name"])
            .unwrap_or(DATABASE_ORM_OFFICIAL_NAME)
            .to_string(),
        upstream_package: json_text(package, &["upstream_package"])
            .unwrap_or(DATABASE_ORM_UPSTREAM_PACKAGE)
            .to_string(),
        upstream_version: json_text(package, &["upstream_version"])
            .unwrap_or(DATABASE_ORM_UPSTREAM_VERSION)
            .to_string(),
        source_mirror: json_text(package, &["source_mirror"])
            .unwrap_or(DATABASE_ORM_SOURCE_MIRROR)
            .to_string(),
        status: status.to_string(),
        receipt_status: if stale_receipt > 0 {
            "stale".to_string()
        } else {
            receipt_status.to_string()
        },
        package_receipt_path: package_receipt_path.to_string(),
        status_vocabulary: database_orm_status_vocabulary(package),
        selected_surfaces,
        metrics: database_orm_metric_rows(
            1,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
            refresh_current,
            refresh_stale,
            refresh_missing,
            dx_style_compatibility_present,
            dx_style_compatibility_missing,
        ),
        receipt_hash_refresh: receipt_hash_refresh.clone(),
        runtime_limitations: json_string_array(package, &["runtime_limitations"]),
        next_action: database_orm_next_action(
            status,
            receipt_hash_refresh.as_ref(),
            dx_style_compatibility_missing,
        )
        .to_string(),
    })
}

fn database_orm_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow {
    DxCheckPanelPackageLaneRow {
        package_id: DATABASE_ORM_PACKAGE_ID.to_string(),
        official_package_name: DATABASE_ORM_OFFICIAL_NAME.to_string(),
        upstream_package: DATABASE_ORM_UPSTREAM_PACKAGE.to_string(),
        upstream_version: DATABASE_ORM_UPSTREAM_VERSION.to_string(),
        source_mirror: DATABASE_ORM_SOURCE_MIRROR.to_string(),
        status: "missing-receipt".to_string(),
        receipt_status: "missing-receipt".to_string(),
        package_receipt_path: DATABASE_ORM_PACKAGE_RECEIPT_PATH.to_string(),
        status_vocabulary: DATABASE_ORM_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect(),
        selected_surfaces: Vec::new(),
        metrics: database_orm_metric_rows(1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1),
        receipt_hash_refresh: None,
        runtime_limitations: vec![
            "SOURCE-ONLY: missing package-status or dashboard workflow receipt blocks Database ORM package-lane visibility.".to_string(),
        ],
        next_action: next_action.to_string(),
    }
}

fn forms_package_lane_row(
    root: &Path,
    package_status: Option<&ForgePackageStatusReadModel>,
) -> Option<DxCheckPanelPackageLaneRow> {
    let manifest_package_present = source_manifest_has_package(root, FORMS_PACKAGE_ID);
    let Some(package_status) = package_status else {
        return manifest_package_present.then(|| {
            forms_missing_receipt_row(
                "Restore the Forms package-status row and dashboard workflow receipt so Studio and Zed can render package-lane status.",
            )
        });
    };
    let package = package_lane_visibility_entry(package_status, FORMS_PACKAGE_ID)?;
    let package_id = json_text(package, &["package_id"]).unwrap_or(FORMS_PACKAGE_ID);
    if package_id != FORMS_PACKAGE_ID && !manifest_package_present {
        return None;
    }

    let visibility_status = json_text(package, &["status"])
        .or_else(|| json_text(package, &["current_status"]))
        .unwrap_or("present");
    let package_receipt_path =
        json_text(package, &["package_receipt_path"]).unwrap_or(FORMS_PACKAGE_RECEIPT_PATH);
    let receipt_present = u64::from(project_file_path(root, package_receipt_path).is_some());
    let missing_receipt = u64::from(receipt_present == 0);
    let receipt_status = if receipt_present == 0 {
        "missing-receipt"
    } else {
        json_text(package, &["receipt_status"]).unwrap_or(visibility_status)
    };
    let selected_surfaces = selected_package_surfaces(package);
    let blocked_surfaces = u64::from(matches!(visibility_status, "blocked"))
        + u64::from(matches!(receipt_status, "blocked"))
        + json_array_len(package, &["blocked_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "blocked")
            .count() as u64;
    let unsupported_surfaces = u64::from(matches!(visibility_status, "unsupported-surface"))
        + u64::from(matches!(receipt_status, "unsupported-surface"))
        + json_array_len(package, &["unsupported_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "unsupported-surface")
            .count() as u64;
    let hash_manifest_present = u64::from(has_sha256_file_hashes(package));
    let hash_mismatches = count_sha256_file_hash_mismatches(root, package);
    let receipt_hash_refresh = package_lane_hash_refresh(package);
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);
    let stale_receipt = u64::from(
        matches!(visibility_status, "stale")
            || matches!(receipt_status, "stale")
            || hash_mismatches > 0
            || refresh_stale > 0,
    );
    let status = if missing_receipt > 0 {
        "missing-receipt"
    } else {
        state_management_effective_status(
            visibility_status,
            stale_receipt,
            blocked_surfaces,
            unsupported_surfaces,
        )
    };

    Some(DxCheckPanelPackageLaneRow {
        package_id: FORMS_PACKAGE_ID.to_string(),
        official_package_name: json_text(package, &["official_package_name"])
            .unwrap_or(FORMS_OFFICIAL_NAME)
            .to_string(),
        upstream_package: json_text(package, &["upstream_package"])
            .unwrap_or(FORMS_UPSTREAM_PACKAGE)
            .to_string(),
        upstream_version: json_text(package, &["upstream_version"])
            .unwrap_or(FORMS_UPSTREAM_VERSION)
            .to_string(),
        source_mirror: json_text(package, &["source_mirror"])
            .unwrap_or(FORMS_SOURCE_MIRROR)
            .to_string(),
        status: status.to_string(),
        receipt_status: if stale_receipt > 0 {
            "stale".to_string()
        } else {
            receipt_status.to_string()
        },
        package_receipt_path: package_receipt_path.to_string(),
        status_vocabulary: forms_status_vocabulary(package),
        selected_surfaces,
        receipt_hash_refresh,
        metrics: forms_metric_rows(
            1,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
            refresh_current,
            refresh_stale,
            refresh_missing,
        ),
        runtime_limitations: json_string_array(package, &["runtime_limitations"]),
        next_action: forms_next_action(status, refresh_stale, refresh_missing).to_string(),
    })
}

fn forms_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow {
    DxCheckPanelPackageLaneRow {
        package_id: FORMS_PACKAGE_ID.to_string(),
        official_package_name: FORMS_OFFICIAL_NAME.to_string(),
        upstream_package: FORMS_UPSTREAM_PACKAGE.to_string(),
        upstream_version: FORMS_UPSTREAM_VERSION.to_string(),
        source_mirror: FORMS_SOURCE_MIRROR.to_string(),
        status: "missing-receipt".to_string(),
        receipt_status: "missing-receipt".to_string(),
        package_receipt_path: FORMS_PACKAGE_RECEIPT_PATH.to_string(),
        status_vocabulary: FORMS_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect(),
        selected_surfaces: Vec::new(),
        metrics: forms_metric_rows(1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1),
        receipt_hash_refresh: None,
        runtime_limitations: vec![
            "SOURCE-ONLY: missing package-status or dashboard workflow receipt blocks Forms package-lane visibility.".to_string(),
        ],
        next_action: next_action.to_string(),
    }
}

fn backend_platform_client_package_lane_row(
    root: &Path,
    package_status: Option<&ForgePackageStatusReadModel>,
) -> Option<DxCheckPanelPackageLaneRow> {
    let manifest_package_present =
        source_manifest_has_package(root, BACKEND_PLATFORM_CLIENT_PACKAGE_ID);
    let Some(package_status) = package_status else {
        return (manifest_package_present
            || project_file_path(root, BACKEND_PLATFORM_CLIENT_PACKAGE_RECEIPT_PATH).is_some())
        .then(|| {
            backend_platform_client_missing_receipt_row(
                "Restore the Backend Platform Client package-status row and dashboard workflow receipt so Studio and Zed can render package-lane status.",
            )
        });
    };
    let package = package_lane_visibility_entry(package_status, BACKEND_PLATFORM_CLIENT_PACKAGE_ID)?;
    let package_id =
        json_text(package, &["package_id"]).unwrap_or(BACKEND_PLATFORM_CLIENT_PACKAGE_ID);
    if package_id != BACKEND_PLATFORM_CLIENT_PACKAGE_ID && !manifest_package_present {
        return None;
    }

    let visibility_status = json_text(package, &["status"])
        .or_else(|| json_text(package, &["current_status"]))
        .unwrap_or("present");
    let package_receipt_path = json_text(package, &["package_receipt_path"])
        .unwrap_or(BACKEND_PLATFORM_CLIENT_PACKAGE_RECEIPT_PATH);
    let receipt_present = u64::from(project_file_path(root, package_receipt_path).is_some());
    let missing_receipt = u64::from(receipt_present == 0);
    let receipt_status = if receipt_present == 0 {
        "missing-receipt"
    } else {
        json_text(package, &["receipt_status"]).unwrap_or(visibility_status)
    };
    let selected_surfaces = selected_package_surfaces(package);
    let blocked_surfaces = u64::from(matches!(visibility_status, "blocked"))
        + u64::from(matches!(receipt_status, "blocked"))
        + json_array_len(package, &["blocked_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "blocked")
            .count() as u64;
    let unsupported_surfaces = u64::from(matches!(visibility_status, "unsupported-surface"))
        + u64::from(matches!(receipt_status, "unsupported-surface"))
        + json_array_len(package, &["unsupported_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "unsupported-surface")
            .count() as u64;
    let hash_manifest_present = u64::from(has_sha256_file_hashes(package));
    let hash_mismatches = count_sha256_file_hash_mismatches(root, package);
    let receipt_hash_refresh = package_lane_hash_refresh(package);
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);
    let helper_stale = u64::from(refresh_stale > 0 || refresh_missing > 0);
    let stale_receipt = u64::from(
        matches!(visibility_status, "stale")
            || matches!(receipt_status, "stale")
            || hash_mismatches > 0
            || helper_stale > 0,
    );
    let status = if missing_receipt > 0 {
        "missing-receipt"
    } else {
        state_management_effective_status(
            visibility_status,
            stale_receipt,
            blocked_surfaces,
            unsupported_surfaces,
        )
    };

    Some(DxCheckPanelPackageLaneRow {
        package_id: BACKEND_PLATFORM_CLIENT_PACKAGE_ID.to_string(),
        official_package_name: json_text(package, &["official_package_name"])
            .unwrap_or(BACKEND_PLATFORM_CLIENT_OFFICIAL_NAME)
            .to_string(),
        upstream_package: json_text(package, &["upstream_package"])
            .unwrap_or(BACKEND_PLATFORM_CLIENT_UPSTREAM_PACKAGE)
            .to_string(),
        upstream_version: json_text(package, &["upstream_version"])
            .unwrap_or(BACKEND_PLATFORM_CLIENT_UPSTREAM_VERSION)
            .to_string(),
        source_mirror: json_text(package, &["source_mirror"])
            .unwrap_or(BACKEND_PLATFORM_CLIENT_SOURCE_MIRROR)
            .to_string(),
        status: status.to_string(),
        receipt_status: if stale_receipt > 0 {
            "stale".to_string()
        } else {
            receipt_status.to_string()
        },
        package_receipt_path: package_receipt_path.to_string(),
        status_vocabulary: backend_platform_client_status_vocabulary(package),
        selected_surfaces,
        receipt_hash_refresh,
        metrics: backend_platform_client_metric_rows(
            1,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
            refresh_current,
            refresh_stale,
            refresh_missing,
        ),
        runtime_limitations: json_string_array(package, &["runtime_limitations"]),
        next_action: backend_platform_client_next_action(status).to_string(),
    })
}

fn backend_platform_client_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow {
    DxCheckPanelPackageLaneRow {
        package_id: BACKEND_PLATFORM_CLIENT_PACKAGE_ID.to_string(),
        official_package_name: BACKEND_PLATFORM_CLIENT_OFFICIAL_NAME.to_string(),
        upstream_package: BACKEND_PLATFORM_CLIENT_UPSTREAM_PACKAGE.to_string(),
        upstream_version: BACKEND_PLATFORM_CLIENT_UPSTREAM_VERSION.to_string(),
        source_mirror: BACKEND_PLATFORM_CLIENT_SOURCE_MIRROR.to_string(),
        status: "missing-receipt".to_string(),
        receipt_status: "missing-receipt".to_string(),
        package_receipt_path: BACKEND_PLATFORM_CLIENT_PACKAGE_RECEIPT_PATH.to_string(),
        status_vocabulary: BACKEND_PLATFORM_CLIENT_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect(),
        selected_surfaces: Vec::new(),
        receipt_hash_refresh: None,
        metrics: backend_platform_client_metric_rows(1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1),
        runtime_limitations: vec![
            "SOURCE-ONLY: missing package-status or dashboard workflow receipt blocks Backend Platform Client package-lane visibility.".to_string(),
        ],
        next_action: next_action.to_string(),
    }
}

fn realtime_app_database_package_lane_row(
    root: &Path,
    package_status: Option<&ForgePackageStatusReadModel>,
) -> Option<DxCheckPanelPackageLaneRow> {
    let manifest_package_present =
        source_manifest_has_package(root, REALTIME_APP_DATABASE_PACKAGE_ID);
    let Some(package_status) = package_status else {
        return (manifest_package_present
            || project_file_path(root, REALTIME_APP_DATABASE_PACKAGE_RECEIPT_PATH).is_some())
        .then(|| {
            realtime_app_database_missing_receipt_row(
                "Restore the Realtime App Database package-status row and dashboard workflow receipt so Studio and Zed can render package-lane style visibility.",
            )
        });
    };
    let package = package_lane_visibility_entry(package_status, REALTIME_APP_DATABASE_PACKAGE_ID)?;
    let package_id =
        json_text(package, &["package_id"]).unwrap_or(REALTIME_APP_DATABASE_PACKAGE_ID);
    if package_id != REALTIME_APP_DATABASE_PACKAGE_ID && !manifest_package_present {
        return None;
    }

    let visibility_status = json_text(package, &["status"])
        .or_else(|| json_text(package, &["current_status"]))
        .unwrap_or("present");
    let package_receipt_path = json_text(package, &["package_receipt_path"])
        .unwrap_or(REALTIME_APP_DATABASE_PACKAGE_RECEIPT_PATH);
    let receipt_present = u64::from(project_file_path(root, package_receipt_path).is_some());
    let missing_receipt = u64::from(receipt_present == 0);
    let receipt_status = if receipt_present == 0 {
        "missing-receipt"
    } else {
        json_text(package, &["receipt_status"]).unwrap_or(visibility_status)
    };
    let selected_surfaces = selected_package_surfaces(package);
    let blocked_surfaces = u64::from(matches!(visibility_status, "blocked"))
        + u64::from(matches!(receipt_status, "blocked"))
        + json_array_len(package, &["blocked_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "blocked")
            .count() as u64;
    let unsupported_surfaces = u64::from(matches!(visibility_status, "unsupported-surface"))
        + u64::from(matches!(receipt_status, "unsupported-surface"))
        + json_array_len(package, &["unsupported_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "unsupported-surface")
            .count() as u64;
    let hash_manifest_present =
        u64::from(has_sha256_file_hashes(package) || has_sha256_source_hashes(package));
    let hash_mismatches = count_sha256_file_hash_mismatches(root, package)
        + count_sha256_source_hash_mismatches(root, package);
    let receipt_hash_refresh = package_lane_hash_refresh(package);
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);
    let stale_receipt = u64::from(
        matches!(visibility_status, "stale")
            || matches!(receipt_status, "stale")
            || hash_mismatches > 0
            || refresh_stale > 0
            || refresh_missing > 0,
    );
    let dx_style_compatibility_present = u64::from(dx_style_compatibility_is_present(package));
    let dx_style_compatibility_missing = u64::from(dx_style_compatibility_present == 0);
    let status = if missing_receipt > 0 {
        "missing-receipt"
    } else {
        state_management_effective_status(
            visibility_status,
            stale_receipt,
            blocked_surfaces,
            unsupported_surfaces,
        )
    };

    Some(DxCheckPanelPackageLaneRow {
        package_id: REALTIME_APP_DATABASE_PACKAGE_ID.to_string(),
        official_package_name: json_text(package, &["official_package_name"])
            .unwrap_or(REALTIME_APP_DATABASE_OFFICIAL_NAME)
            .to_string(),
        upstream_package: json_text(package, &["upstream_package"])
            .unwrap_or(REALTIME_APP_DATABASE_UPSTREAM_PACKAGE)
            .to_string(),
        upstream_version: json_text(package, &["upstream_version"])
            .unwrap_or(REALTIME_APP_DATABASE_UPSTREAM_VERSION)
            .to_string(),
        source_mirror: json_text(package, &["source_mirror"])
            .unwrap_or(REALTIME_APP_DATABASE_SOURCE_MIRROR)
            .to_string(),
        status: status.to_string(),
        receipt_status: if stale_receipt > 0 {
            "stale".to_string()
        } else {
            receipt_status.to_string()
        },
        package_receipt_path: package_receipt_path.to_string(),
        status_vocabulary: realtime_app_database_status_vocabulary(package),
        selected_surfaces,
        receipt_hash_refresh,
        metrics: realtime_app_database_metric_rows(
            1,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
            refresh_current,
            refresh_stale,
            refresh_missing,
            dx_style_compatibility_present,
            dx_style_compatibility_missing,
        ),
        runtime_limitations: json_string_array(package, &["runtime_limitations"]),
        next_action: realtime_app_database_next_action(
            status,
            refresh_stale,
            refresh_missing,
            dx_style_compatibility_missing,
        )
        .to_string(),
    })
}

fn realtime_app_database_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow {
    DxCheckPanelPackageLaneRow {
        package_id: REALTIME_APP_DATABASE_PACKAGE_ID.to_string(),
        official_package_name: REALTIME_APP_DATABASE_OFFICIAL_NAME.to_string(),
        upstream_package: REALTIME_APP_DATABASE_UPSTREAM_PACKAGE.to_string(),
        upstream_version: REALTIME_APP_DATABASE_UPSTREAM_VERSION.to_string(),
        source_mirror: REALTIME_APP_DATABASE_SOURCE_MIRROR.to_string(),
        status: "missing-receipt".to_string(),
        receipt_status: "missing-receipt".to_string(),
        package_receipt_path: REALTIME_APP_DATABASE_PACKAGE_RECEIPT_PATH.to_string(),
        status_vocabulary: REALTIME_APP_DATABASE_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect(),
        selected_surfaces: Vec::new(),
        receipt_hash_refresh: None,
        metrics: realtime_app_database_metric_rows(1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1),
        runtime_limitations: vec![
            "SOURCE-ONLY: missing package-status or dashboard workflow receipt blocks Realtime App Database package-lane visibility.".to_string(),
        ],
        next_action: next_action.to_string(),
    }
}

fn payments_package_lane_row(
    root: &Path,
    package_status: Option<&ForgePackageStatusReadModel>,
) -> Option<DxCheckPanelPackageLaneRow> {
    let manifest_package_present = source_manifest_has_package(root, PAYMENTS_PACKAGE_ID);
    let Some(package_status) = package_status else {
        return (manifest_package_present
            || project_file_path(root, PAYMENTS_PACKAGE_RECEIPT_PATH).is_some())
        .then(|| {
            payments_missing_receipt_row(
                "Restore the Payments package-status row and billing workflow receipt so Studio and Zed can render package-lane status.",
            )
        });
    };
    let package = package_lane_visibility_entry(package_status, PAYMENTS_PACKAGE_ID)?;
    let package_id = json_text(package, &["package_id"]).unwrap_or(PAYMENTS_PACKAGE_ID);
    if package_id != PAYMENTS_PACKAGE_ID && !manifest_package_present {
        return None;
    }

    let visibility_status = json_text(package, &["status"])
        .or_else(|| json_text(package, &["current_status"]))
        .unwrap_or("present");
    let package_receipt_path =
        json_text(package, &["package_receipt_path"]).unwrap_or(PAYMENTS_PACKAGE_RECEIPT_PATH);
    let receipt_present = u64::from(project_file_path(root, package_receipt_path).is_some());
    let missing_receipt = u64::from(receipt_present == 0);
    let receipt_status = if receipt_present == 0 {
        "missing-receipt"
    } else {
        json_text(package, &["receipt_status"]).unwrap_or(visibility_status)
    };
    let selected_surfaces = selected_package_surfaces(package);
    let blocked_surfaces = u64::from(matches!(visibility_status, "blocked"))
        + u64::from(matches!(receipt_status, "blocked"))
        + json_array_len(package, &["blocked_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "blocked")
            .count() as u64;
    let unsupported_surfaces = u64::from(matches!(visibility_status, "unsupported-surface"))
        + u64::from(matches!(receipt_status, "unsupported-surface"))
        + json_array_len(package, &["unsupported_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "unsupported-surface")
            .count() as u64;
    let hash_manifest_present = u64::from(has_sha256_file_hashes(package));
    let hash_mismatches = count_sha256_file_hash_mismatches(root, package);
    let receipt_hash_refresh = package_lane_hash_refresh(package);
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);
    let dx_style_compatibility_present = u64::from(dx_style_compatibility_is_present(package));
    let dx_style_compatibility_missing = u64::from(dx_style_compatibility_present == 0);
    let stale_receipt = u64::from(
        matches!(visibility_status, "stale")
            || matches!(receipt_status, "stale")
            || hash_mismatches > 0
            || refresh_stale > 0
            || refresh_missing > 0,
    );
    let status = if missing_receipt > 0 {
        "missing-receipt"
    } else {
        state_management_effective_status(
            visibility_status,
            stale_receipt,
            blocked_surfaces,
            unsupported_surfaces,
        )
    };

    Some(DxCheckPanelPackageLaneRow {
        package_id: PAYMENTS_PACKAGE_ID.to_string(),
        official_package_name: json_text(package, &["official_package_name"])
            .unwrap_or(PAYMENTS_OFFICIAL_NAME)
            .to_string(),
        upstream_package: json_text(package, &["upstream_package"])
            .unwrap_or(PAYMENTS_UPSTREAM_PACKAGE)
            .to_string(),
        upstream_version: json_text(package, &["upstream_version"])
            .unwrap_or(PAYMENTS_UPSTREAM_VERSION)
            .to_string(),
        source_mirror: json_text(package, &["source_mirror"])
            .unwrap_or(PAYMENTS_SOURCE_MIRROR)
            .to_string(),
        status: status.to_string(),
        receipt_status: if stale_receipt > 0 {
            "stale".to_string()
        } else {
            receipt_status.to_string()
        },
        package_receipt_path: package_receipt_path.to_string(),
        status_vocabulary: payments_status_vocabulary(package),
        selected_surfaces,
        receipt_hash_refresh: receipt_hash_refresh.clone(),
        metrics: payments_metric_rows(
            1,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
            refresh_current,
            refresh_stale,
            refresh_missing,
            dx_style_compatibility_present,
            dx_style_compatibility_missing,
        ),
        runtime_limitations: json_string_array(package, &["runtime_limitations"]),
        next_action: payments_next_action(
            status,
            receipt_hash_refresh.as_ref(),
            dx_style_compatibility_missing,
        )
        .to_string(),
    })
}

fn payments_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow {
    DxCheckPanelPackageLaneRow {
        package_id: PAYMENTS_PACKAGE_ID.to_string(),
        official_package_name: PAYMENTS_OFFICIAL_NAME.to_string(),
        upstream_package: PAYMENTS_UPSTREAM_PACKAGE.to_string(),
        upstream_version: PAYMENTS_UPSTREAM_VERSION.to_string(),
        source_mirror: PAYMENTS_SOURCE_MIRROR.to_string(),
        status: "missing-receipt".to_string(),
        receipt_status: "missing-receipt".to_string(),
        package_receipt_path: PAYMENTS_PACKAGE_RECEIPT_PATH.to_string(),
        status_vocabulary: PAYMENTS_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect(),
        selected_surfaces: Vec::new(),
        receipt_hash_refresh: None,
        metrics: payments_metric_rows(1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1),
        runtime_limitations: vec![
            "SOURCE-ONLY: missing package-status or billing workflow receipt blocks Payments package-lane visibility.".to_string(),
        ],
        next_action: next_action.to_string(),
    }
}

fn motion_animation_package_lane_row(
    root: &Path,
    package_status: Option<&ForgePackageStatusReadModel>,
) -> Option<DxCheckPanelPackageLaneRow> {
    let manifest_package_present = source_manifest_has_package(root, MOTION_ANIMATION_PACKAGE_ID);
    let Some(package_status) = package_status else {
        return (manifest_package_present
            || project_file_path(root, MOTION_ANIMATION_PACKAGE_RECEIPT_PATH).is_some())
        .then(|| {
            motion_animation_missing_receipt_row(
                "Restore the Motion & Animation package-status row and dashboard workflow receipt so Studio and Zed can render package-lane helper freshness.",
            )
        });
    };
    let package = package_lane_visibility_entry(package_status, MOTION_ANIMATION_PACKAGE_ID)?;
    let package_id = json_text(package, &["package_id"]).unwrap_or(MOTION_ANIMATION_PACKAGE_ID);
    if package_id != MOTION_ANIMATION_PACKAGE_ID && !manifest_package_present {
        return None;
    }

    let visibility_status = json_text(package, &["status"])
        .or_else(|| json_text(package, &["current_status"]))
        .unwrap_or("present");
    let package_receipt_path = json_text(package, &["package_receipt_path"])
        .unwrap_or(MOTION_ANIMATION_PACKAGE_RECEIPT_PATH);
    let receipt_present = u64::from(project_file_path(root, package_receipt_path).is_some());
    let missing_receipt = u64::from(receipt_present == 0);
    let receipt_status = if receipt_present == 0 {
        "missing-receipt"
    } else {
        json_text(package, &["receipt_status"]).unwrap_or(visibility_status)
    };
    let selected_surfaces = selected_package_surfaces(package);
    let blocked_surfaces = u64::from(matches!(visibility_status, "blocked"))
        + u64::from(matches!(receipt_status, "blocked"))
        + json_array_len(package, &["blocked_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "blocked")
            .count() as u64;
    let unsupported_surfaces = u64::from(matches!(visibility_status, "unsupported-surface"))
        + u64::from(matches!(receipt_status, "unsupported-surface"))
        + json_array_len(package, &["unsupported_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "unsupported-surface")
            .count() as u64;
    let hash_manifest_present =
        u64::from(has_sha256_file_hashes(package) || has_sha256_source_hashes(package));
    let hash_mismatches = count_sha256_file_hash_mismatches(root, package)
        + count_sha256_source_hash_mismatches(root, package);
    let receipt_hash_refresh = package_lane_hash_refresh(package);
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);
    let stale_receipt = u64::from(
        matches!(visibility_status, "stale")
            || matches!(receipt_status, "stale")
            || hash_mismatches > 0
            || refresh_stale > 0,
    );
    let status = if missing_receipt > 0 {
        "missing-receipt"
    } else {
        state_management_effective_status(
            visibility_status,
            stale_receipt,
            blocked_surfaces,
            unsupported_surfaces,
        )
    };

    Some(DxCheckPanelPackageLaneRow {
        package_id: MOTION_ANIMATION_PACKAGE_ID.to_string(),
        official_package_name: json_text(package, &["official_package_name"])
            .unwrap_or(MOTION_ANIMATION_OFFICIAL_NAME)
            .to_string(),
        upstream_package: json_text(package, &["upstream_package"])
            .unwrap_or(MOTION_ANIMATION_UPSTREAM_PACKAGE)
            .to_string(),
        upstream_version: json_text(package, &["upstream_version"])
            .unwrap_or(MOTION_ANIMATION_UPSTREAM_VERSION)
            .to_string(),
        source_mirror: json_text(package, &["source_mirror"])
            .unwrap_or(MOTION_ANIMATION_SOURCE_MIRROR)
            .to_string(),
        status: status.to_string(),
        receipt_status: if stale_receipt > 0 {
            "stale".to_string()
        } else {
            receipt_status.to_string()
        },
        package_receipt_path: package_receipt_path.to_string(),
        status_vocabulary: motion_animation_status_vocabulary(package),
        selected_surfaces,
        receipt_hash_refresh,
        metrics: motion_animation_metric_rows(
            1,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
            refresh_current,
            refresh_stale,
            refresh_missing,
        ),
        runtime_limitations: json_string_array(package, &["runtime_limitations"]),
        next_action: motion_animation_next_action(status, refresh_stale, refresh_missing)
            .to_string(),
    })
}

fn motion_animation_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow {
    DxCheckPanelPackageLaneRow {
        package_id: MOTION_ANIMATION_PACKAGE_ID.to_string(),
        official_package_name: MOTION_ANIMATION_OFFICIAL_NAME.to_string(),
        upstream_package: MOTION_ANIMATION_UPSTREAM_PACKAGE.to_string(),
        upstream_version: MOTION_ANIMATION_UPSTREAM_VERSION.to_string(),
        source_mirror: MOTION_ANIMATION_SOURCE_MIRROR.to_string(),
        status: "missing-receipt".to_string(),
        receipt_status: "missing-receipt".to_string(),
        package_receipt_path: MOTION_ANIMATION_PACKAGE_RECEIPT_PATH.to_string(),
        status_vocabulary: MOTION_ANIMATION_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect(),
        selected_surfaces: Vec::new(),
        receipt_hash_refresh: None,
        metrics: motion_animation_metric_rows(1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1),
        runtime_limitations: vec![
            "SOURCE-ONLY: missing package-status or dashboard workflow receipt blocks Motion & Animation package-lane visibility.".to_string(),
        ],
        next_action: next_action.to_string(),
    }
}

fn validation_schemas_package_lane_row(
    root: &Path,
    package_status: Option<&ForgePackageStatusReadModel>,
) -> Option<DxCheckPanelPackageLaneRow> {
    let manifest_package_present = source_manifest_has_package(root, VALIDATION_SCHEMAS_PACKAGE_ID);
    let Some(package_status) = package_status else {
        return manifest_package_present.then(|| {
            validation_schemas_missing_receipt_row(
                "Restore the Validation & Schemas package-status row and dashboard settings receipt so Studio and Zed can render package-lane status.",
            )
        });
    };
    let package = package_lane_visibility_entry(package_status, VALIDATION_SCHEMAS_PACKAGE_ID)?;
    let package_id = json_text(package, &["package_id"]).unwrap_or(VALIDATION_SCHEMAS_PACKAGE_ID);
    if package_id != VALIDATION_SCHEMAS_PACKAGE_ID && !manifest_package_present {
        return None;
    }

    let visibility_status = json_text(package, &["status"])
        .or_else(|| json_text(package, &["current_status"]))
        .unwrap_or("present");
    let package_receipt_path = json_text(package, &["package_receipt_path"])
        .unwrap_or(VALIDATION_SCHEMAS_PACKAGE_RECEIPT_PATH);
    let receipt_present = u64::from(project_file_path(root, package_receipt_path).is_some());
    let missing_receipt = u64::from(receipt_present == 0);
    let receipt_status = if receipt_present == 0 {
        "missing-receipt"
    } else {
        json_text(package, &["receipt_status"]).unwrap_or(visibility_status)
    };
    let selected_surfaces = selected_package_surfaces(package);
    let blocked_surfaces = u64::from(matches!(visibility_status, "blocked"))
        + u64::from(matches!(receipt_status, "blocked"))
        + json_array_len(package, &["blocked_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "blocked")
            .count() as u64;
    let unsupported_surfaces = u64::from(matches!(visibility_status, "unsupported-surface"))
        + u64::from(matches!(receipt_status, "unsupported-surface"))
        + json_array_len(package, &["unsupported_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "unsupported-surface")
            .count() as u64;
    let hash_manifest_present =
        u64::from(has_sha256_file_hashes(package) || has_sha256_source_hashes(package));
    let hash_mismatches = count_sha256_file_hash_mismatches(root, package)
        + count_sha256_source_hash_mismatches(root, package);
    let receipt_hash_refresh = package_lane_hash_refresh(package);
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);
    let stale_receipt = u64::from(
        matches!(visibility_status, "stale")
            || matches!(receipt_status, "stale")
            || hash_mismatches > 0
            || refresh_stale > 0,
    );
    let status = if missing_receipt > 0 {
        "missing-receipt"
    } else {
        state_management_effective_status(
            visibility_status,
            stale_receipt,
            blocked_surfaces,
            unsupported_surfaces,
        )
    };

    Some(DxCheckPanelPackageLaneRow {
        package_id: VALIDATION_SCHEMAS_PACKAGE_ID.to_string(),
        official_package_name: json_text(package, &["official_package_name"])
            .unwrap_or(VALIDATION_SCHEMAS_OFFICIAL_NAME)
            .to_string(),
        upstream_package: json_text(package, &["upstream_package"])
            .unwrap_or(VALIDATION_SCHEMAS_UPSTREAM_PACKAGE)
            .to_string(),
        upstream_version: json_text(package, &["upstream_version"])
            .unwrap_or(VALIDATION_SCHEMAS_UPSTREAM_VERSION)
            .to_string(),
        source_mirror: json_text(package, &["source_mirror"])
            .unwrap_or(VALIDATION_SCHEMAS_SOURCE_MIRROR)
            .to_string(),
        status: status.to_string(),
        receipt_status: if stale_receipt > 0 {
            "stale".to_string()
        } else {
            receipt_status.to_string()
        },
        package_receipt_path: package_receipt_path.to_string(),
        status_vocabulary: validation_schemas_status_vocabulary(package),
        selected_surfaces,
        receipt_hash_refresh,
        metrics: validation_schemas_metric_rows(
            1,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
            refresh_current,
            refresh_stale,
            refresh_missing,
        ),
        runtime_limitations: json_string_array(package, &["runtime_limitations"]),
        next_action: validation_schemas_next_action(status, refresh_stale, refresh_missing)
            .to_string(),
    })
}

fn validation_schemas_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow {
    DxCheckPanelPackageLaneRow {
        package_id: VALIDATION_SCHEMAS_PACKAGE_ID.to_string(),
        official_package_name: VALIDATION_SCHEMAS_OFFICIAL_NAME.to_string(),
        upstream_package: VALIDATION_SCHEMAS_UPSTREAM_PACKAGE.to_string(),
        upstream_version: VALIDATION_SCHEMAS_UPSTREAM_VERSION.to_string(),
        source_mirror: VALIDATION_SCHEMAS_SOURCE_MIRROR.to_string(),
        status: "missing-receipt".to_string(),
        receipt_status: "missing-receipt".to_string(),
        package_receipt_path: VALIDATION_SCHEMAS_PACKAGE_RECEIPT_PATH.to_string(),
        status_vocabulary: VALIDATION_SCHEMAS_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect(),
        selected_surfaces: Vec::new(),
        receipt_hash_refresh: None,
        metrics: validation_schemas_metric_rows(1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1),
        runtime_limitations: vec![
            "SOURCE-ONLY: missing package-status or dashboard settings receipt blocks Validation & Schemas package-lane visibility.".to_string(),
        ],
        next_action: next_action.to_string(),
    }
}

fn type_safe_api_package_lane_row(
    root: &Path,
    package_status: Option<&ForgePackageStatusReadModel>,
) -> Option<DxCheckPanelPackageLaneRow> {
    let manifest_package_present = source_manifest_has_package(root, TYPE_SAFE_API_PACKAGE_ID);
    let Some(package_status) = package_status else {
        return manifest_package_present.then(|| {
            type_safe_api_missing_receipt_row(
                "Restore the Type-Safe API package-status row and dashboard workflow receipt so Studio and Zed can render package-lane helper freshness.",
            )
        });
    };
    let package = package_lane_visibility_entry(package_status, TYPE_SAFE_API_PACKAGE_ID)?;
    let package_id = json_text(package, &["package_id"]).unwrap_or(TYPE_SAFE_API_PACKAGE_ID);
    if package_id != TYPE_SAFE_API_PACKAGE_ID && !manifest_package_present {
        return None;
    }

    let visibility_status = json_text(package, &["status"])
        .or_else(|| json_text(package, &["current_status"]))
        .unwrap_or("present");
    let package_receipt_path =
        json_text(package, &["package_receipt_path"]).unwrap_or(TYPE_SAFE_API_PACKAGE_RECEIPT_PATH);
    let receipt_present = u64::from(project_file_path(root, package_receipt_path).is_some());
    let missing_receipt = u64::from(receipt_present == 0);
    let receipt_status = if receipt_present == 0 {
        "missing-receipt"
    } else {
        json_text(package, &["receipt_status"]).unwrap_or(visibility_status)
    };
    let selected_surfaces = type_safe_api_selected_surfaces(package);
    let blocked_surfaces = u64::from(matches!(visibility_status, "blocked"))
        + u64::from(matches!(receipt_status, "blocked"))
        + json_array_len(package, &["blocked_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "blocked")
            .count() as u64;
    let unsupported_surfaces = u64::from(matches!(visibility_status, "unsupported-surface"))
        + u64::from(matches!(receipt_status, "unsupported-surface"))
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "unsupported-surface")
            .count() as u64;
    let hash_manifest_present =
        u64::from(has_sha256_file_hashes(package) || has_sha256_source_hashes(package));
    let hash_mismatches = count_sha256_file_hash_mismatches(root, package)
        + count_sha256_source_hash_mismatches(root, package);
    let receipt_hash_refresh = package_lane_hash_refresh(package);
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);
    let stale_receipt = u64::from(
        matches!(visibility_status, "stale")
            || matches!(receipt_status, "stale")
            || hash_mismatches > 0
            || refresh_stale > 0,
    );
    let status = if missing_receipt > 0 {
        "missing-receipt"
    } else {
        state_management_effective_status(
            visibility_status,
            stale_receipt,
            blocked_surfaces,
            unsupported_surfaces,
        )
    };

    Some(DxCheckPanelPackageLaneRow {
        package_id: TYPE_SAFE_API_PACKAGE_ID.to_string(),
        official_package_name: json_text(package, &["official_package_name"])
            .unwrap_or(TYPE_SAFE_API_OFFICIAL_NAME)
            .to_string(),
        upstream_package: json_text(package, &["upstream_package"])
            .unwrap_or(TYPE_SAFE_API_UPSTREAM_PACKAGE)
            .to_string(),
        upstream_version: json_text(package, &["upstream_version"])
            .unwrap_or(TYPE_SAFE_API_UPSTREAM_VERSION)
            .to_string(),
        source_mirror: json_text(package, &["source_mirror"])
            .unwrap_or(TYPE_SAFE_API_SOURCE_MIRROR)
            .to_string(),
        status: status.to_string(),
        receipt_status: if stale_receipt > 0 {
            "stale".to_string()
        } else {
            receipt_status.to_string()
        },
        package_receipt_path: package_receipt_path.to_string(),
        status_vocabulary: type_safe_api_status_vocabulary(package),
        selected_surfaces,
        receipt_hash_refresh,
        metrics: type_safe_api_metric_rows(
            1,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
            refresh_current,
            refresh_stale,
            refresh_missing,
        ),
        runtime_limitations: json_string_array(package, &["runtime_limitations"]),
        next_action: type_safe_api_next_action(status, refresh_stale, refresh_missing).to_string(),
    })
}

fn type_safe_api_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow {
    DxCheckPanelPackageLaneRow {
        package_id: TYPE_SAFE_API_PACKAGE_ID.to_string(),
        official_package_name: TYPE_SAFE_API_OFFICIAL_NAME.to_string(),
        upstream_package: TYPE_SAFE_API_UPSTREAM_PACKAGE.to_string(),
        upstream_version: TYPE_SAFE_API_UPSTREAM_VERSION.to_string(),
        source_mirror: TYPE_SAFE_API_SOURCE_MIRROR.to_string(),
        status: "missing-receipt".to_string(),
        receipt_status: "missing-receipt".to_string(),
        package_receipt_path: TYPE_SAFE_API_PACKAGE_RECEIPT_PATH.to_string(),
        status_vocabulary: TYPE_SAFE_API_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect(),
        selected_surfaces: Vec::new(),
        receipt_hash_refresh: None,
        metrics: type_safe_api_metric_rows(1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1),
        runtime_limitations: vec![
            "SOURCE-ONLY: missing package-status or dashboard workflow receipt blocks Type-Safe API package-lane visibility.".to_string(),
        ],
        next_action: next_action.to_string(),
    }
}

fn ui_components_package_lane_row(
    root: &Path,
    package_status: Option<&ForgePackageStatusReadModel>,
) -> Option<DxCheckPanelPackageLaneRow> {
    let manifest_package_present = source_manifest_has_package(root, UI_COMPONENTS_PACKAGE_ID);
    let Some(package_status) = package_status else {
        return manifest_package_present.then(|| {
            ui_components_missing_receipt_row(
                "Restore the UI Components package-status row and dashboard controls receipt so Studio and Zed can render package-lane helper freshness.",
            )
        });
    };
    let package = package_lane_visibility_entry(package_status, UI_COMPONENTS_PACKAGE_ID)?;
    let package_id = json_text(package, &["package_id"]).unwrap_or(UI_COMPONENTS_PACKAGE_ID);
    if package_id != UI_COMPONENTS_PACKAGE_ID && !manifest_package_present {
        return None;
    }

    let visibility_status = json_text(package, &["status"])
        .or_else(|| json_text(package, &["current_status"]))
        .unwrap_or("present");
    let package_receipt_path =
        json_text(package, &["package_receipt_path"]).unwrap_or(UI_COMPONENTS_PACKAGE_RECEIPT_PATH);
    let receipt_present = u64::from(project_file_path(root, package_receipt_path).is_some());
    let missing_receipt = u64::from(receipt_present == 0);
    let receipt_status = if receipt_present == 0 {
        "missing-receipt"
    } else {
        json_text(package, &["receipt_status"]).unwrap_or(visibility_status)
    };
    let selected_surfaces = selected_package_surfaces(package);
    let blocked_surfaces = u64::from(matches!(visibility_status, "blocked"))
        + u64::from(matches!(receipt_status, "blocked"))
        + json_array_len(package, &["blocked_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "blocked")
            .count() as u64;
    let unsupported_surfaces = u64::from(matches!(visibility_status, "unsupported-surface"))
        + u64::from(matches!(receipt_status, "unsupported-surface"))
        + json_array_len(package, &["unsupported_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "unsupported-surface")
            .count() as u64;
    let hash_manifest_present =
        u64::from(has_sha256_file_hashes(package) || has_sha256_source_hashes(package));
    let hash_mismatches = count_sha256_file_hash_mismatches(root, package)
        + count_sha256_source_hash_mismatches(root, package);
    let receipt_hash_refresh = ui_components_hash_refresh_row(package);
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);
    let stale_receipt = u64::from(
        matches!(visibility_status, "stale")
            || matches!(receipt_status, "stale")
            || hash_mismatches > 0
            || refresh_stale > 0
            || refresh_missing > 0,
    );
    let status = if missing_receipt > 0 {
        "missing-receipt"
    } else {
        state_management_effective_status(
            visibility_status,
            stale_receipt,
            blocked_surfaces,
            unsupported_surfaces,
        )
    };
    let next_action = ui_components_next_action(status, receipt_hash_refresh.as_ref()).to_string();

    Some(DxCheckPanelPackageLaneRow {
        package_id: UI_COMPONENTS_PACKAGE_ID.to_string(),
        official_package_name: json_text(package, &["official_package_name"])
            .unwrap_or(UI_COMPONENTS_OFFICIAL_NAME)
            .to_string(),
        upstream_package: json_text(package, &["upstream_package"])
            .unwrap_or(UI_COMPONENTS_UPSTREAM_PACKAGE)
            .to_string(),
        upstream_version: json_text(package, &["upstream_version"])
            .unwrap_or(UI_COMPONENTS_UPSTREAM_VERSION)
            .to_string(),
        source_mirror: json_text(package, &["source_mirror"])
            .unwrap_or(UI_COMPONENTS_SOURCE_MIRROR)
            .to_string(),
        status: status.to_string(),
        receipt_status: if stale_receipt > 0 {
            "stale".to_string()
        } else {
            receipt_status.to_string()
        },
        package_receipt_path: package_receipt_path.to_string(),
        status_vocabulary: ui_components_status_vocabulary(package),
        selected_surfaces,
        receipt_hash_refresh,
        metrics: ui_components_metric_rows(
            1,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
            refresh_current,
            refresh_stale,
            refresh_missing,
        ),
        runtime_limitations: ui_components_runtime_limitations(package),
        next_action,
    })
}

fn ui_components_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow {
    DxCheckPanelPackageLaneRow {
        package_id: UI_COMPONENTS_PACKAGE_ID.to_string(),
        official_package_name: UI_COMPONENTS_OFFICIAL_NAME.to_string(),
        upstream_package: UI_COMPONENTS_UPSTREAM_PACKAGE.to_string(),
        upstream_version: UI_COMPONENTS_UPSTREAM_VERSION.to_string(),
        source_mirror: UI_COMPONENTS_SOURCE_MIRROR.to_string(),
        status: "missing-receipt".to_string(),
        receipt_status: "missing-receipt".to_string(),
        package_receipt_path: UI_COMPONENTS_PACKAGE_RECEIPT_PATH.to_string(),
        status_vocabulary: UI_COMPONENTS_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect(),
        selected_surfaces: Vec::new(),
        receipt_hash_refresh: None,
        metrics: ui_components_metric_rows(1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1),
        runtime_limitations: vec![
            "SOURCE-ONLY: missing package-status or dashboard controls receipt blocks UI Components package-lane helper freshness.".to_string(),
        ],
        next_action: next_action.to_string(),
    }
}

fn documentation_system_package_lane_row(
    root: &Path,
    package_status: Option<&ForgePackageStatusReadModel>,
) -> Option<DxCheckPanelPackageLaneRow> {
    let manifest_package_present =
        source_manifest_has_package(root, DOCUMENTATION_SYSTEM_PACKAGE_ID);
    let Some(package_status) = package_status else {
        return manifest_package_present.then(|| {
            documentation_system_missing_receipt_row(
                "Restore the Documentation System package-status row and dashboard workflow receipt so Studio and Zed can render package-lane status.",
            )
        });
    };
    let package = package_lane_visibility_entry(package_status, DOCUMENTATION_SYSTEM_PACKAGE_ID)?;
    let package_id = json_text(package, &["package_id"]).unwrap_or(DOCUMENTATION_SYSTEM_PACKAGE_ID);
    if package_id != DOCUMENTATION_SYSTEM_PACKAGE_ID && !manifest_package_present {
        return None;
    }

    let visibility_status = json_text(package, &["status"])
        .or_else(|| json_text(package, &["current_status"]))
        .unwrap_or("present");
    let package_receipt_path = json_text(package, &["package_receipt_path"])
        .unwrap_or(DOCUMENTATION_SYSTEM_PACKAGE_RECEIPT_PATH);
    let receipt_present = u64::from(project_file_path(root, package_receipt_path).is_some());
    let missing_receipt = u64::from(receipt_present == 0);
    let receipt_status = if receipt_present == 0 {
        "missing-receipt"
    } else {
        json_text(package, &["receipt_status"]).unwrap_or(visibility_status)
    };
    let selected_surfaces = selected_package_surfaces(package);
    let blocked_surfaces = u64::from(matches!(visibility_status, "blocked"))
        + u64::from(matches!(receipt_status, "blocked"))
        + json_array_len(package, &["blocked_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "blocked")
            .count() as u64;
    let unsupported_surfaces = u64::from(matches!(visibility_status, "unsupported-surface"))
        + u64::from(matches!(receipt_status, "unsupported-surface"))
        + json_array_len(package, &["unsupported_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "unsupported-surface")
            .count() as u64;
    let hash_manifest_present =
        u64::from(has_sha256_file_hashes(package) || has_sha256_source_hashes(package));
    let hash_mismatches = count_sha256_file_hash_mismatches(root, package)
        + count_sha256_source_hash_mismatches(root, package);
    let receipt_hash_refresh = package_lane_hash_refresh(package);
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);
    let stale_receipt = u64::from(
        matches!(visibility_status, "stale")
            || matches!(receipt_status, "stale")
            || hash_mismatches > 0
            || refresh_stale > 0,
    );
    let dx_style_compatibility_present = u64::from(dx_style_compatibility_is_present(package));
    let dx_style_compatibility_missing = u64::from(dx_style_compatibility_present == 0);
    let status = if missing_receipt > 0 {
        "missing-receipt"
    } else {
        state_management_effective_status(
            visibility_status,
            stale_receipt,
            blocked_surfaces,
            unsupported_surfaces,
        )
    };

    Some(DxCheckPanelPackageLaneRow {
        package_id: DOCUMENTATION_SYSTEM_PACKAGE_ID.to_string(),
        official_package_name: json_text(package, &["official_package_name"])
            .unwrap_or(DOCUMENTATION_SYSTEM_OFFICIAL_NAME)
            .to_string(),
        upstream_package: json_text(package, &["upstream_package"])
            .unwrap_or(DOCUMENTATION_SYSTEM_UPSTREAM_PACKAGE)
            .to_string(),
        upstream_version: json_text(package, &["upstream_version"])
            .unwrap_or(DOCUMENTATION_SYSTEM_UPSTREAM_VERSION)
            .to_string(),
        source_mirror: json_text(package, &["source_mirror"])
            .unwrap_or(DOCUMENTATION_SYSTEM_SOURCE_MIRROR)
            .to_string(),
        status: status.to_string(),
        receipt_status: if stale_receipt > 0 {
            "stale".to_string()
        } else {
            receipt_status.to_string()
        },
        package_receipt_path: package_receipt_path.to_string(),
        status_vocabulary: documentation_system_status_vocabulary(package),
        selected_surfaces,
        receipt_hash_refresh,
        metrics: documentation_system_metric_rows(
            1,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
            refresh_current,
            refresh_stale,
            refresh_missing,
            dx_style_compatibility_present,
            dx_style_compatibility_missing,
        ),
        runtime_limitations: json_string_array(package, &["runtime_limitations"]),
        next_action: documentation_system_next_action(status).to_string(),
    })
}

fn documentation_system_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow {
    DxCheckPanelPackageLaneRow {
        package_id: DOCUMENTATION_SYSTEM_PACKAGE_ID.to_string(),
        official_package_name: DOCUMENTATION_SYSTEM_OFFICIAL_NAME.to_string(),
        upstream_package: DOCUMENTATION_SYSTEM_UPSTREAM_PACKAGE.to_string(),
        upstream_version: DOCUMENTATION_SYSTEM_UPSTREAM_VERSION.to_string(),
        source_mirror: DOCUMENTATION_SYSTEM_SOURCE_MIRROR.to_string(),
        status: "missing-receipt".to_string(),
        receipt_status: "missing-receipt".to_string(),
        package_receipt_path: DOCUMENTATION_SYSTEM_PACKAGE_RECEIPT_PATH.to_string(),
        status_vocabulary: DOCUMENTATION_SYSTEM_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect(),
        selected_surfaces: Vec::new(),
        metrics: documentation_system_metric_rows(1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1),
        receipt_hash_refresh: None,
        runtime_limitations: vec![
            "SOURCE-ONLY: missing package-status or dashboard workflow receipt blocks Documentation System package-lane visibility.".to_string(),
        ],
        next_action: next_action.to_string(),
    }
}

fn markdown_mdx_content_package_lane_row(
    root: &Path,
    package_status: Option<&ForgePackageStatusReadModel>,
) -> Option<DxCheckPanelPackageLaneRow> {
    let manifest_package_present =
        source_manifest_has_package(root, MARKDOWN_MDX_CONTENT_PACKAGE_ID);
    let Some(package_status) = package_status else {
        return manifest_package_present.then(|| {
            markdown_mdx_content_missing_receipt_row(
                "Restore the Markdown & MDX Content package-status row and package receipt so Studio and Zed can render materialized-source visibility.",
            )
        });
    };
    let package = package_lane_visibility_entry(package_status, MARKDOWN_MDX_CONTENT_PACKAGE_ID)?;
    let package_id = json_text(package, &["package_id"]).unwrap_or(MARKDOWN_MDX_CONTENT_PACKAGE_ID);
    if package_id != MARKDOWN_MDX_CONTENT_PACKAGE_ID && !manifest_package_present {
        return None;
    }

    let visibility_status = json_text(package, &["status"])
        .or_else(|| json_text(package, &["current_status"]))
        .unwrap_or("present");
    let package_receipt_path = json_text(package, &["package_receipt_path"])
        .unwrap_or(MARKDOWN_MDX_CONTENT_PACKAGE_RECEIPT_PATH);
    let receipt_present = u64::from(project_file_path(root, package_receipt_path).is_some());
    let missing_receipt = u64::from(receipt_present == 0);
    let receipt_status = if receipt_present == 0 {
        "missing-receipt"
    } else {
        json_text(package, &["receipt_status"]).unwrap_or(visibility_status)
    };
    let selected_surfaces = selected_package_surfaces(package);
    let blocked_surfaces = u64::from(matches!(visibility_status, "blocked"))
        + u64::from(matches!(receipt_status, "blocked"))
        + json_array_len(package, &["blocked_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "blocked")
            .count() as u64;
    let unsupported_surfaces = u64::from(matches!(visibility_status, "unsupported-surface"))
        + u64::from(matches!(receipt_status, "unsupported-surface"))
        + json_array_len(package, &["unsupported_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "unsupported-surface")
            .count() as u64;
    let hash_manifest_present =
        u64::from(has_sha256_file_hashes(package) || has_sha256_source_hashes(package));
    let hash_mismatches = count_sha256_file_hash_mismatches(root, package)
        + count_sha256_source_hash_mismatches(root, package);
    let receipt_hash_refresh = package_lane_hash_refresh(package);
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);
    let helper_stale = u64::from(refresh_stale > 0 || refresh_missing > 0);
    let stale_receipt = u64::from(
        matches!(visibility_status, "stale")
            || matches!(receipt_status, "stale")
            || hash_mismatches > 0
            || helper_stale > 0,
    );
    let dx_style_compatibility_present = u64::from(dx_style_compatibility_is_present(package));
    let dx_style_compatibility_missing = u64::from(dx_style_compatibility_present == 0);
    let materialized_source_present =
        u64::from(markdown_mdx_content_materialized_source_is_present(package));
    let materialized_source_missing = u64::from(materialized_source_present == 0);
    let status = if missing_receipt > 0 {
        "missing-receipt"
    } else {
        state_management_effective_status(
            visibility_status,
            stale_receipt,
            blocked_surfaces,
            unsupported_surfaces,
        )
    };

    Some(DxCheckPanelPackageLaneRow {
        package_id: MARKDOWN_MDX_CONTENT_PACKAGE_ID.to_string(),
        official_package_name: json_text(package, &["official_package_name"])
            .unwrap_or(MARKDOWN_MDX_CONTENT_OFFICIAL_NAME)
            .to_string(),
        upstream_package: json_text(package, &["upstream_package"])
            .unwrap_or(MARKDOWN_MDX_CONTENT_UPSTREAM_PACKAGE)
            .to_string(),
        upstream_version: json_text(package, &["upstream_version"])
            .unwrap_or(MARKDOWN_MDX_CONTENT_UPSTREAM_VERSION)
            .to_string(),
        source_mirror: json_text(package, &["source_mirror"])
            .unwrap_or(MARKDOWN_MDX_CONTENT_SOURCE_MIRROR)
            .to_string(),
        status: status.to_string(),
        receipt_status: if stale_receipt > 0 {
            "stale".to_string()
        } else {
            receipt_status.to_string()
        },
        package_receipt_path: package_receipt_path.to_string(),
        status_vocabulary: markdown_mdx_content_status_vocabulary(package),
        selected_surfaces,
        receipt_hash_refresh,
        metrics: markdown_mdx_content_metric_rows(
            1,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
            refresh_current,
            refresh_stale,
            refresh_missing,
            dx_style_compatibility_present,
            dx_style_compatibility_missing,
            materialized_source_present,
            materialized_source_missing,
        ),
        runtime_limitations: json_string_array(package, &["runtime_limitations"]),
        next_action: markdown_mdx_content_next_action(
            status,
            dx_style_compatibility_missing,
            materialized_source_missing,
        )
        .to_string(),
    })
}

fn markdown_mdx_content_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow {
    DxCheckPanelPackageLaneRow {
        package_id: MARKDOWN_MDX_CONTENT_PACKAGE_ID.to_string(),
        official_package_name: MARKDOWN_MDX_CONTENT_OFFICIAL_NAME.to_string(),
        upstream_package: MARKDOWN_MDX_CONTENT_UPSTREAM_PACKAGE.to_string(),
        upstream_version: MARKDOWN_MDX_CONTENT_UPSTREAM_VERSION.to_string(),
        source_mirror: MARKDOWN_MDX_CONTENT_SOURCE_MIRROR.to_string(),
        status: "missing-receipt".to_string(),
        receipt_status: "missing-receipt".to_string(),
        package_receipt_path: MARKDOWN_MDX_CONTENT_PACKAGE_RECEIPT_PATH.to_string(),
        status_vocabulary: MARKDOWN_MDX_CONTENT_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect(),
        selected_surfaces: Vec::new(),
        metrics: markdown_mdx_content_metric_rows(1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1),
        receipt_hash_refresh: None,
        runtime_limitations: vec![
            "SOURCE-ONLY: missing package-status or package receipt blocks Markdown & MDX Content materialized-source visibility.".to_string(),
        ],
        next_action: next_action.to_string(),
    }
}

fn ai_sdk_package_lane_row(
    root: &Path,
    package_status: Option<&ForgePackageStatusReadModel>,
) -> Option<DxCheckPanelPackageLaneRow> {
    let manifest_package_present = source_manifest_has_package(root, AI_SDK_PACKAGE_ID);
    let Some(package_status) = package_status else {
        return manifest_package_present.then(|| {
            ai_sdk_missing_receipt_row(
                "Restore the AI SDK package-status row and launch assistant receipt so Studio and Zed can render package-lane style visibility.",
            )
        });
    };
    let package = package_lane_visibility_entry(package_status, AI_SDK_PACKAGE_ID)?;
    let package_id = json_text(package, &["package_id"]).unwrap_or(AI_SDK_PACKAGE_ID);
    if package_id != AI_SDK_PACKAGE_ID && !manifest_package_present {
        return None;
    }

    let visibility_status = json_text(package, &["status"])
        .or_else(|| json_text(package, &["current_status"]))
        .unwrap_or("present");
    let package_receipt_path =
        json_text(package, &["package_receipt_path"]).unwrap_or(AI_SDK_PACKAGE_RECEIPT_PATH);
    let receipt_present = u64::from(project_file_path(root, package_receipt_path).is_some());
    let missing_receipt = u64::from(receipt_present == 0);
    let receipt_status = if receipt_present == 0 {
        "missing-receipt"
    } else {
        json_text(package, &["receipt_status"]).unwrap_or(visibility_status)
    };
    let selected_surfaces = selected_package_surfaces(package);
    let blocked_surfaces = u64::from(matches!(visibility_status, "blocked"))
        + u64::from(matches!(receipt_status, "blocked"))
        + json_array_len(package, &["blocked_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "blocked")
            .count() as u64;
    let unsupported_surfaces = u64::from(matches!(visibility_status, "unsupported-surface"))
        + u64::from(matches!(receipt_status, "unsupported-surface"))
        + json_array_len(package, &["unsupported_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "unsupported-surface")
            .count() as u64;
    let checkable_package = ai_sdk_checkable_hash_manifest(package);
    let hash_manifest_present = u64::from(has_sha256_file_hashes(&checkable_package));
    let hash_mismatches = count_sha256_file_hash_mismatches(root, &checkable_package);
    let receipt_hash_refresh = package_lane_hash_refresh(package);
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);
    let stale_receipt = u64::from(
        matches!(visibility_status, "stale")
            || matches!(receipt_status, "stale")
            || hash_mismatches > 0
            || refresh_stale > 0
            || refresh_missing > 0,
    );
    let dx_style_compatibility_present = u64::from(dx_style_compatibility_is_present(package));
    let dx_style_compatibility_missing = u64::from(dx_style_compatibility_present == 0);
    let status = if missing_receipt > 0 {
        "missing-receipt"
    } else {
        state_management_effective_status(
            visibility_status,
            stale_receipt,
            blocked_surfaces,
            unsupported_surfaces,
        )
    };

    Some(DxCheckPanelPackageLaneRow {
        package_id: AI_SDK_PACKAGE_ID.to_string(),
        official_package_name: json_text(package, &["official_package_name"])
            .unwrap_or(AI_SDK_OFFICIAL_NAME)
            .to_string(),
        upstream_package: json_text(package, &["upstream_package"])
            .unwrap_or(AI_SDK_UPSTREAM_PACKAGE)
            .to_string(),
        upstream_version: json_text(package, &["upstream_version"])
            .unwrap_or(AI_SDK_UPSTREAM_VERSION)
            .to_string(),
        source_mirror: json_text(package, &["source_mirror"])
            .unwrap_or(AI_SDK_SOURCE_MIRROR)
            .to_string(),
        status: status.to_string(),
        receipt_status: if stale_receipt > 0 {
            "stale".to_string()
        } else {
            receipt_status.to_string()
        },
        package_receipt_path: package_receipt_path.to_string(),
        status_vocabulary: ai_sdk_status_vocabulary(package),
        selected_surfaces,
        receipt_hash_refresh,
        metrics: ai_sdk_metric_rows(
            1,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
            refresh_current,
            refresh_stale,
            refresh_missing,
            dx_style_compatibility_present,
            dx_style_compatibility_missing,
        ),
        runtime_limitations: json_string_array(package, &["runtime_limitations"]),
        next_action: ai_sdk_next_action(
            status,
            refresh_stale,
            refresh_missing,
            dx_style_compatibility_missing,
        )
        .to_string(),
    })
}

fn ai_sdk_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow {
    DxCheckPanelPackageLaneRow {
        package_id: AI_SDK_PACKAGE_ID.to_string(),
        official_package_name: AI_SDK_OFFICIAL_NAME.to_string(),
        upstream_package: AI_SDK_UPSTREAM_PACKAGE.to_string(),
        upstream_version: AI_SDK_UPSTREAM_VERSION.to_string(),
        source_mirror: AI_SDK_SOURCE_MIRROR.to_string(),
        status: "missing-receipt".to_string(),
        receipt_status: "missing-receipt".to_string(),
        package_receipt_path: AI_SDK_PACKAGE_RECEIPT_PATH.to_string(),
        status_vocabulary: AI_SDK_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect(),
        selected_surfaces: Vec::new(),
        metrics: ai_sdk_metric_rows(1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1),
        receipt_hash_refresh: None,
        runtime_limitations: vec![
            "SOURCE-ONLY: missing package-status or launch assistant receipt blocks AI SDK package-lane visibility.".to_string(),
        ],
        next_action: next_action.to_string(),
    }
}

fn internationalization_package_lane_row(
    root: &Path,
    package_status: Option<&ForgePackageStatusReadModel>,
) -> Option<DxCheckPanelPackageLaneRow> {
    let manifest_package_present =
        source_manifest_has_package(root, INTERNATIONALIZATION_PACKAGE_ID);
    let Some(package_status) = package_status else {
        return manifest_package_present.then(|| {
            internationalization_missing_receipt_row(
                "Restore the Internationalization package-status row and dashboard workflow receipt so Studio and Zed can render package-lane style visibility.",
            )
        });
    };
    let package = package_lane_visibility_entry(package_status, INTERNATIONALIZATION_PACKAGE_ID)?;
    let package_id = json_text(package, &["package_id"]).unwrap_or(INTERNATIONALIZATION_PACKAGE_ID);
    if package_id != INTERNATIONALIZATION_PACKAGE_ID && !manifest_package_present {
        return None;
    }

    let visibility_status = json_text(package, &["status"])
        .or_else(|| json_text(package, &["current_status"]))
        .unwrap_or("present");
    let package_receipt_path = json_text(package, &["package_receipt_path"])
        .unwrap_or(INTERNATIONALIZATION_PACKAGE_RECEIPT_PATH);
    let receipt_present = u64::from(project_file_path(root, package_receipt_path).is_some());
    let missing_receipt = u64::from(receipt_present == 0);
    let receipt_status = if receipt_present == 0 {
        "missing-receipt"
    } else {
        json_text(package, &["receipt_status"]).unwrap_or(visibility_status)
    };
    let selected_surfaces = selected_package_surfaces(package);
    let blocked_surfaces = u64::from(matches!(visibility_status, "blocked"))
        + u64::from(matches!(receipt_status, "blocked"))
        + json_array_len(package, &["blocked_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "blocked")
            .count() as u64;
    let unsupported_surfaces = u64::from(matches!(visibility_status, "unsupported-surface"))
        + u64::from(matches!(receipt_status, "unsupported-surface"))
        + json_array_len(package, &["unsupported_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "unsupported-surface")
            .count() as u64;
    let hash_manifest_present = u64::from(has_sha256_file_hashes(package));
    let hash_mismatches = count_sha256_file_hash_mismatches(root, package);
    let receipt_hash_refresh = package_lane_hash_refresh(package);
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);
    let stale_receipt = u64::from(
        matches!(visibility_status, "stale")
            || matches!(receipt_status, "stale")
            || hash_mismatches > 0
            || refresh_stale > 0,
    );
    let dx_style_compatibility_present = u64::from(dx_style_compatibility_is_present(package));
    let dx_style_compatibility_missing = u64::from(dx_style_compatibility_present == 0);
    let status = if missing_receipt > 0 {
        "missing-receipt"
    } else {
        state_management_effective_status(
            visibility_status,
            stale_receipt,
            blocked_surfaces,
            unsupported_surfaces,
        )
    };

    Some(DxCheckPanelPackageLaneRow {
        package_id: INTERNATIONALIZATION_PACKAGE_ID.to_string(),
        official_package_name: json_text(package, &["official_package_name"])
            .unwrap_or(INTERNATIONALIZATION_OFFICIAL_NAME)
            .to_string(),
        upstream_package: json_text(package, &["upstream_package"])
            .unwrap_or(INTERNATIONALIZATION_UPSTREAM_PACKAGE)
            .to_string(),
        upstream_version: json_text(package, &["upstream_version"])
            .unwrap_or(INTERNATIONALIZATION_UPSTREAM_VERSION)
            .to_string(),
        source_mirror: json_text(package, &["source_mirror"])
            .unwrap_or(INTERNATIONALIZATION_SOURCE_MIRROR)
            .to_string(),
        status: status.to_string(),
        receipt_status: if stale_receipt > 0 {
            "stale".to_string()
        } else {
            receipt_status.to_string()
        },
        package_receipt_path: package_receipt_path.to_string(),
        status_vocabulary: internationalization_status_vocabulary(package),
        selected_surfaces,
        receipt_hash_refresh: receipt_hash_refresh.clone(),
        metrics: internationalization_metric_rows(
            1,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
            refresh_current,
            refresh_stale,
            refresh_missing,
            dx_style_compatibility_present,
            dx_style_compatibility_missing,
        ),
        runtime_limitations: json_string_array(package, &["runtime_limitations"]),
        next_action: internationalization_next_action(
            status,
            refresh_stale,
            refresh_missing,
            dx_style_compatibility_missing,
        )
        .to_string(),
    })
}

fn internationalization_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow {
    DxCheckPanelPackageLaneRow {
        package_id: INTERNATIONALIZATION_PACKAGE_ID.to_string(),
        official_package_name: INTERNATIONALIZATION_OFFICIAL_NAME.to_string(),
        upstream_package: INTERNATIONALIZATION_UPSTREAM_PACKAGE.to_string(),
        upstream_version: INTERNATIONALIZATION_UPSTREAM_VERSION.to_string(),
        source_mirror: INTERNATIONALIZATION_SOURCE_MIRROR.to_string(),
        status: "missing-receipt".to_string(),
        receipt_status: "missing-receipt".to_string(),
        package_receipt_path: INTERNATIONALIZATION_PACKAGE_RECEIPT_PATH.to_string(),
        status_vocabulary: INTERNATIONALIZATION_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect(),
        selected_surfaces: Vec::new(),
        metrics: internationalization_metric_rows(1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1),
        receipt_hash_refresh: None,
        runtime_limitations: vec![
            "SOURCE-ONLY: missing package-status or dashboard workflow receipt blocks Internationalization package-lane visibility.".to_string(),
        ],
        next_action: next_action.to_string(),
    }
}

fn three_scene_system_package_lane_row(
    root: &Path,
    package_status: Option<&ForgePackageStatusReadModel>,
) -> Option<DxCheckPanelPackageLaneRow> {
    let manifest_package_present = source_manifest_has_package(root, THREE_SCENE_SYSTEM_PACKAGE_ID);
    let Some(package_status) = package_status else {
        return manifest_package_present.then(|| {
            three_scene_system_missing_receipt_row(
                "Restore the 3D Scene System package-status row and dashboard workflow receipt so Studio and Zed can render package-lane style visibility.",
            )
        });
    };
    let package = package_lane_visibility_entry(package_status, THREE_SCENE_SYSTEM_PACKAGE_ID)?;
    let package_id = json_text(package, &["package_id"]).unwrap_or(THREE_SCENE_SYSTEM_PACKAGE_ID);
    if package_id != THREE_SCENE_SYSTEM_PACKAGE_ID && !manifest_package_present {
        return None;
    }

    let visibility_status = json_text(package, &["status"])
        .or_else(|| json_text(package, &["current_status"]))
        .unwrap_or("present");
    let package_receipt_path = json_text(package, &["package_receipt_path"])
        .unwrap_or(THREE_SCENE_SYSTEM_PACKAGE_RECEIPT_PATH);
    let receipt_present = u64::from(project_file_path(root, package_receipt_path).is_some());
    let missing_receipt = u64::from(receipt_present == 0);
    let receipt_status = if receipt_present == 0 {
        "missing-receipt"
    } else {
        json_text(package, &["receipt_status"]).unwrap_or(visibility_status)
    };
    let selected_surfaces = selected_package_surfaces(package);
    let blocked_surfaces = u64::from(matches!(visibility_status, "blocked"))
        + u64::from(matches!(receipt_status, "blocked"))
        + json_array_len(package, &["blocked_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "blocked")
            .count() as u64;
    let unsupported_surfaces = u64::from(matches!(visibility_status, "unsupported-surface"))
        + u64::from(matches!(receipt_status, "unsupported-surface"))
        + json_array_len(package, &["unsupported_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "unsupported-surface")
            .count() as u64;
    let hash_manifest_present = u64::from(has_sha256_file_hashes(package));
    let hash_mismatches = count_sha256_file_hash_mismatches(root, package);
    let receipt_hash_refresh = package_lane_hash_refresh(package);
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);
    let stale_receipt = u64::from(
        matches!(visibility_status, "stale")
            || matches!(receipt_status, "stale")
            || hash_mismatches > 0
            || refresh_stale > 0
            || refresh_missing > 0,
    );
    let dx_style_compatibility_present = u64::from(dx_style_compatibility_is_present(package));
    let dx_style_compatibility_missing = u64::from(dx_style_compatibility_present == 0);
    let status = if missing_receipt > 0 {
        "missing-receipt"
    } else {
        state_management_effective_status(
            visibility_status,
            stale_receipt,
            blocked_surfaces,
            unsupported_surfaces,
        )
    };

    Some(DxCheckPanelPackageLaneRow {
        package_id: THREE_SCENE_SYSTEM_PACKAGE_ID.to_string(),
        official_package_name: json_text(package, &["official_package_name"])
            .unwrap_or(THREE_SCENE_SYSTEM_OFFICIAL_NAME)
            .to_string(),
        upstream_package: json_text(package, &["upstream_package"])
            .unwrap_or(THREE_SCENE_SYSTEM_UPSTREAM_PACKAGE)
            .to_string(),
        upstream_version: json_text(package, &["upstream_version"])
            .unwrap_or(THREE_SCENE_SYSTEM_UPSTREAM_VERSION)
            .to_string(),
        source_mirror: json_text(package, &["source_mirror"])
            .unwrap_or(THREE_SCENE_SYSTEM_SOURCE_MIRROR)
            .to_string(),
        status: status.to_string(),
        receipt_status: if stale_receipt > 0 {
            "stale".to_string()
        } else {
            receipt_status.to_string()
        },
        package_receipt_path: package_receipt_path.to_string(),
        status_vocabulary: three_scene_system_status_vocabulary(package),
        selected_surfaces,
        receipt_hash_refresh,
        metrics: three_scene_system_metric_rows(
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
            refresh_current,
            refresh_stale,
            refresh_missing,
            dx_style_compatibility_present,
            dx_style_compatibility_missing,
        ),
        runtime_limitations: json_string_array(package, &["runtime_limitations"]),
        next_action: three_scene_system_next_action(
            status,
            refresh_stale,
            refresh_missing,
            dx_style_compatibility_missing,
        )
        .to_string(),
    })
}

fn three_scene_system_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow {
    DxCheckPanelPackageLaneRow {
        package_id: THREE_SCENE_SYSTEM_PACKAGE_ID.to_string(),
        official_package_name: THREE_SCENE_SYSTEM_OFFICIAL_NAME.to_string(),
        upstream_package: THREE_SCENE_SYSTEM_UPSTREAM_PACKAGE.to_string(),
        upstream_version: THREE_SCENE_SYSTEM_UPSTREAM_VERSION.to_string(),
        source_mirror: THREE_SCENE_SYSTEM_SOURCE_MIRROR.to_string(),
        status: "missing-receipt".to_string(),
        receipt_status: "missing-receipt".to_string(),
        package_receipt_path: THREE_SCENE_SYSTEM_PACKAGE_RECEIPT_PATH.to_string(),
        status_vocabulary: THREE_SCENE_SYSTEM_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect(),
        selected_surfaces: Vec::new(),
        metrics: three_scene_system_metric_rows(0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1),
        receipt_hash_refresh: None,
        runtime_limitations: vec![
            "SOURCE-ONLY: missing package-status or dashboard workflow receipt blocks 3D Scene System package-lane visibility.".to_string(),
        ],
        next_action: next_action.to_string(),
    }
}

fn webassembly_bridge_package_lane_row(
    root: &Path,
    package_status: Option<&ForgePackageStatusReadModel>,
) -> Option<DxCheckPanelPackageLaneRow> {
    let manifest_package_present = source_manifest_has_package(root, WEBASSEMBLY_BRIDGE_PACKAGE_ID);
    let Some(package_status) = package_status else {
        return manifest_package_present.then(|| {
            webassembly_bridge_missing_receipt_row(
                "Restore the WebAssembly Bridge package-status row and dashboard workflow receipt so Studio and Zed can render package-lane style visibility.",
            )
        });
    };
    let package = package_lane_visibility_entry(package_status, WEBASSEMBLY_BRIDGE_PACKAGE_ID)?;
    let package_id = json_text(package, &["package_id"]).unwrap_or(WEBASSEMBLY_BRIDGE_PACKAGE_ID);
    if package_id != WEBASSEMBLY_BRIDGE_PACKAGE_ID && !manifest_package_present {
        return None;
    }

    let visibility_status = json_text(package, &["status"])
        .or_else(|| json_text(package, &["current_status"]))
        .unwrap_or("present");
    let package_receipt_path = json_text(package, &["package_receipt_path"])
        .unwrap_or(WEBASSEMBLY_BRIDGE_PACKAGE_RECEIPT_PATH);
    let receipt_present = u64::from(project_file_path(root, package_receipt_path).is_some());
    let missing_receipt = u64::from(receipt_present == 0);
    let receipt_status = if receipt_present == 0 {
        "missing-receipt"
    } else {
        json_text(package, &["receipt_status"]).unwrap_or(visibility_status)
    };
    let selected_surfaces = selected_package_surfaces(package);
    let blocked_surfaces = u64::from(matches!(visibility_status, "blocked"))
        + u64::from(matches!(receipt_status, "blocked"))
        + json_array_len(package, &["blocked_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "blocked")
            .count() as u64;
    let unsupported_surfaces = u64::from(matches!(visibility_status, "unsupported-surface"))
        + u64::from(matches!(receipt_status, "unsupported-surface"))
        + json_array_len(package, &["unsupported_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "unsupported-surface")
            .count() as u64;
    let hash_manifest_present = u64::from(has_sha256_file_hashes(package));
    let hash_mismatches = count_sha256_file_hash_mismatches(root, package);
    let receipt_hash_refresh = package_lane_hash_refresh(package);
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);
    let stale_receipt = u64::from(
        matches!(visibility_status, "stale")
            || matches!(receipt_status, "stale")
            || hash_mismatches > 0
            || refresh_stale > 0
            || refresh_missing > 0,
    );
    let dx_style_compatibility_present = u64::from(dx_style_compatibility_is_present(package));
    let dx_style_compatibility_missing = u64::from(dx_style_compatibility_present == 0);
    let status = if missing_receipt > 0 {
        "missing-receipt"
    } else {
        state_management_effective_status(
            visibility_status,
            stale_receipt,
            blocked_surfaces,
            unsupported_surfaces,
        )
    };

    Some(DxCheckPanelPackageLaneRow {
        package_id: WEBASSEMBLY_BRIDGE_PACKAGE_ID.to_string(),
        official_package_name: json_text(package, &["official_package_name"])
            .unwrap_or(WEBASSEMBLY_BRIDGE_OFFICIAL_NAME)
            .to_string(),
        upstream_package: json_text(package, &["upstream_package"])
            .unwrap_or(WEBASSEMBLY_BRIDGE_UPSTREAM_PACKAGE)
            .to_string(),
        upstream_version: json_text(package, &["upstream_version"])
            .unwrap_or(WEBASSEMBLY_BRIDGE_UPSTREAM_VERSION)
            .to_string(),
        source_mirror: json_text(package, &["source_mirror"])
            .unwrap_or(WEBASSEMBLY_BRIDGE_SOURCE_MIRROR)
            .to_string(),
        status: status.to_string(),
        receipt_status: if stale_receipt > 0 {
            "stale".to_string()
        } else {
            receipt_status.to_string()
        },
        package_receipt_path: package_receipt_path.to_string(),
        status_vocabulary: webassembly_bridge_status_vocabulary(package),
        selected_surfaces,
        receipt_hash_refresh,
        metrics: webassembly_bridge_metric_rows(
            1,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
            refresh_current,
            refresh_stale,
            refresh_missing,
            dx_style_compatibility_present,
            dx_style_compatibility_missing,
        ),
        runtime_limitations: json_string_array(package, &["runtime_limitations"]),
        next_action: webassembly_bridge_next_action(
            status,
            refresh_stale,
            refresh_missing,
            dx_style_compatibility_missing,
        )
        .to_string(),
    })
}

fn webassembly_bridge_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow {
    DxCheckPanelPackageLaneRow {
        package_id: WEBASSEMBLY_BRIDGE_PACKAGE_ID.to_string(),
        official_package_name: WEBASSEMBLY_BRIDGE_OFFICIAL_NAME.to_string(),
        upstream_package: WEBASSEMBLY_BRIDGE_UPSTREAM_PACKAGE.to_string(),
        upstream_version: WEBASSEMBLY_BRIDGE_UPSTREAM_VERSION.to_string(),
        source_mirror: WEBASSEMBLY_BRIDGE_SOURCE_MIRROR.to_string(),
        status: "missing-receipt".to_string(),
        receipt_status: "missing-receipt".to_string(),
        package_receipt_path: WEBASSEMBLY_BRIDGE_PACKAGE_RECEIPT_PATH.to_string(),
        status_vocabulary: WEBASSEMBLY_BRIDGE_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect(),
        selected_surfaces: Vec::new(),
        receipt_hash_refresh: None,
        metrics: webassembly_bridge_metric_rows(1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1),
        runtime_limitations: vec![
            "SOURCE-ONLY: missing package-status or dashboard workflow receipt blocks WebAssembly Bridge package-lane visibility.".to_string(),
        ],
        next_action: next_action.to_string(),
    }
}

fn automation_connectors_package_lane_row(
    root: &Path,
    package_status: Option<&ForgePackageStatusReadModel>,
) -> Option<DxCheckPanelPackageLaneRow> {
    let manifest_package_present =
        source_manifest_has_package(root, AUTOMATION_CONNECTORS_PACKAGE_ID);
    let Some(package_status) = package_status else {
        return manifest_package_present.then(|| {
            automation_connectors_missing_receipt_row(
                "Restore the Automation Connectors package-status row and dashboard workflow receipt so Studio and Zed can render helper freshness, dx-style, and runtime-boundary visibility.",
            )
        });
    };
    let package = package_lane_visibility_entry(package_status, AUTOMATION_CONNECTORS_PACKAGE_ID)?;
    let package_id =
        json_text(package, &["package_id"]).unwrap_or(AUTOMATION_CONNECTORS_PACKAGE_ID);
    if package_id != AUTOMATION_CONNECTORS_PACKAGE_ID && !manifest_package_present {
        return None;
    }

    let visibility_status = json_text(package, &["status"])
        .or_else(|| json_text(package, &["current_status"]))
        .unwrap_or("present");
    let package_receipt_path = json_text(package, &["package_receipt_path"])
        .unwrap_or(AUTOMATION_CONNECTORS_PACKAGE_RECEIPT_PATH);
    let receipt_present = u64::from(project_file_path(root, package_receipt_path).is_some());
    let missing_receipt = u64::from(receipt_present == 0);
    let receipt_status = if receipt_present == 0 {
        "missing-receipt"
    } else {
        json_text(package, &["receipt_status"]).unwrap_or(visibility_status)
    };
    let selected_surfaces = selected_package_surfaces(package);
    let blocked_surfaces = u64::from(matches!(visibility_status, "blocked"))
        + u64::from(matches!(receipt_status, "blocked"))
        + json_array_len(package, &["blocked_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "blocked")
            .count() as u64;
    let unsupported_surfaces = u64::from(matches!(visibility_status, "unsupported-surface"))
        + u64::from(matches!(receipt_status, "unsupported-surface"))
        + json_array_len(package, &["unsupported_surfaces"])
        + selected_surfaces
            .iter()
            .filter(|surface| surface.status == "unsupported-surface")
            .count() as u64;
    let hash_manifest_present = u64::from(has_sha256_file_hashes(package));
    let hash_mismatches = count_sha256_file_hash_mismatches(root, package);
    let receipt_hash_refresh = package_lane_hash_refresh(package);
    let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);
    let dx_style_compatibility_present = u64::from(dx_style_compatibility_is_present(package));
    let dx_style_compatibility_missing = u64::from(dx_style_compatibility_present == 0);
    let upstream_runtime_boundary_present =
        u64::from(automation_connectors_has_upstream_runtime_boundary(package));
    let upstream_runtime_boundary_missing = u64::from(upstream_runtime_boundary_present == 0);
    let stale_receipt = u64::from(
        matches!(visibility_status, "stale")
            || matches!(receipt_status, "stale")
            || hash_mismatches > 0
            || refresh_stale > 0
            || refresh_missing > 0,
    );
    let status = if missing_receipt > 0 {
        "missing-receipt"
    } else {
        state_management_effective_status(
            visibility_status,
            stale_receipt,
            blocked_surfaces,
            unsupported_surfaces,
        )
    };

    Some(DxCheckPanelPackageLaneRow {
        package_id: AUTOMATION_CONNECTORS_PACKAGE_ID.to_string(),
        official_package_name: json_text(package, &["official_package_name"])
            .unwrap_or(AUTOMATION_CONNECTORS_OFFICIAL_NAME)
            .to_string(),
        upstream_package: json_text(package, &["upstream_package"])
            .unwrap_or(AUTOMATION_CONNECTORS_UPSTREAM_PACKAGE)
            .to_string(),
        upstream_version: json_text(package, &["upstream_version"])
            .unwrap_or(AUTOMATION_CONNECTORS_UPSTREAM_VERSION)
            .to_string(),
        source_mirror: json_text(package, &["source_mirror"])
            .unwrap_or(AUTOMATION_CONNECTORS_SOURCE_MIRROR)
            .to_string(),
        status: status.to_string(),
        receipt_status: if stale_receipt > 0 {
            "stale".to_string()
        } else {
            receipt_status.to_string()
        },
        package_receipt_path: package_receipt_path.to_string(),
        status_vocabulary: automation_connectors_status_vocabulary(package),
        selected_surfaces,
        receipt_hash_refresh,
        metrics: automation_connectors_metric_rows(
            1,
            receipt_present,
            stale_receipt,
            missing_receipt,
            blocked_surfaces,
            unsupported_surfaces,
            hash_manifest_present,
            hash_mismatches,
            dx_style_compatibility_present,
            dx_style_compatibility_missing,
            upstream_runtime_boundary_present,
            upstream_runtime_boundary_missing,
            refresh_current,
            refresh_stale,
            refresh_missing,
        ),
        runtime_limitations: json_string_array(package, &["runtime_limitations"]),
        next_action: automation_connectors_next_action(
            status,
            refresh_stale,
            refresh_missing,
            dx_style_compatibility_missing,
            upstream_runtime_boundary_missing,
        )
        .to_string(),
    })
}

fn automation_connectors_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow {
    DxCheckPanelPackageLaneRow {
        package_id: AUTOMATION_CONNECTORS_PACKAGE_ID.to_string(),
        official_package_name: AUTOMATION_CONNECTORS_OFFICIAL_NAME.to_string(),
        upstream_package: AUTOMATION_CONNECTORS_UPSTREAM_PACKAGE.to_string(),
        upstream_version: AUTOMATION_CONNECTORS_UPSTREAM_VERSION.to_string(),
        source_mirror: AUTOMATION_CONNECTORS_SOURCE_MIRROR.to_string(),
        status: "missing-receipt".to_string(),
        receipt_status: "missing-receipt".to_string(),
        package_receipt_path: AUTOMATION_CONNECTORS_PACKAGE_RECEIPT_PATH.to_string(),
        status_vocabulary: AUTOMATION_CONNECTORS_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect(),
        selected_surfaces: Vec::new(),
        receipt_hash_refresh: None,
        metrics: automation_connectors_metric_rows(1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1),
        runtime_limitations: vec![
            "SOURCE-ONLY: missing package-status or dashboard workflow receipt blocks Automation Connectors package-lane visibility.".to_string(),
        ],
        next_action: next_action.to_string(),
    }
}

fn package_lane_visibility_entry<'a>(
    package_status: &'a ForgePackageStatusReadModel,
    package_id: &str,
) -> Option<&'a serde_json::Value> {
    package_status.visibility_entry(package_id)
}

fn source_manifest_has_package(root: &Path, package_id: &str) -> bool {
    fs::read(root.join(SOURCE_MANIFEST_PATH))
        .ok()
        .and_then(|bytes| serde_json::from_slice::<serde_json::Value>(&bytes).ok())
        .and_then(|manifest| {
            manifest
                .get("packages")
                .and_then(|packages| packages.as_array())
                .map(|packages| {
                    packages.iter().any(|package| {
                        json_text(package, &["package_id"]).is_some_and(|id| id == package_id)
                    })
                })
        })
        .unwrap_or(false)
}

fn selected_package_surfaces(
    visibility: &serde_json::Value,
) -> Vec<DxCheckPanelPackageLaneSurfaceRow> {
    value_at(visibility, &["selected_surfaces"])
        .and_then(|surfaces| surfaces.as_array())
        .map(|surfaces| {
            surfaces
                .iter()
                .filter_map(|surface| {
                    if let Some(surface_id) = surface.as_str() {
                        return Some(DxCheckPanelPackageLaneSurfaceRow {
                            surface_id: surface_id.to_string(),
                            status: "present".to_string(),
                            files: Vec::new(),
                            source_markers: Vec::new(),
                            receipt_path: None,
                            reason: None,
                            app_owned_boundary: None,
                        });
                    }

                    let surface_id = json_text(surface, &["surface_id"])
                        .or_else(|| json_text(surface, &["id"]))?;
                    Some(DxCheckPanelPackageLaneSurfaceRow {
                        surface_id: surface_id.to_string(),
                        status: json_text(surface, &["status"])
                            .unwrap_or("present")
                            .to_string(),
                        files: json_string_array(surface, &["files"]),
                        source_markers: json_string_array(surface, &["source_markers"]),
                        receipt_path: json_text(surface, &["receipt_path"]).map(str::to_string),
                        reason: json_text(surface, &["reason"]).map(str::to_string),
                        app_owned_boundary: json_text(surface, &["app_owned_boundary"])
                            .or_else(|| json_text(surface, &["app_owned_boundary_context"]))
                            .map(str::to_string),
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

fn type_safe_api_selected_surfaces(
    visibility: &serde_json::Value,
) -> Vec<DxCheckPanelPackageLaneSurfaceRow> {
    let mut surfaces = selected_package_surfaces(visibility);
    for unsupported in value_at(visibility, &["unsupported_surfaces"])
        .and_then(|surfaces| surfaces.as_array())
        .into_iter()
        .flatten()
    {
        let surface_id = if let Some(surface_id) = unsupported.as_str() {
            surface_id
        } else if let Some(surface_id) = json_text(unsupported, &["surface_id"])
            .or_else(|| json_text(unsupported, &["id"]))
            .or_else(|| json_text(unsupported, &["surface"]))
        {
            surface_id
        } else {
            continue;
        };

        if surfaces
            .iter()
            .any(|surface| surface.surface_id == surface_id)
        {
            continue;
        }

        surfaces.push(DxCheckPanelPackageLaneSurfaceRow {
            surface_id: surface_id.to_string(),
            status: json_text(unsupported, &["status"])
                .unwrap_or("unsupported-surface")
                .to_string(),
            files: json_string_array(unsupported, &["files"]),
            source_markers: json_string_array(unsupported, &["source_markers"]),
            receipt_path: json_text(unsupported, &["receipt_path"]).map(str::to_string),
            reason: json_text(unsupported, &["reason"]).map(str::to_string),
            app_owned_boundary: json_text(unsupported, &["app_owned_boundary"])
                .or_else(|| json_text(unsupported, &["app_owned_boundary_context"]))
                .map(str::to_string),
        });
    }
    surfaces
}

fn reactive_store_selected_surfaces(
    visibility: &serde_json::Value,
    package: &serde_json::Value,
) -> Vec<DxCheckPanelPackageLaneSurfaceRow> {
    let selected_surfaces = selected_package_surfaces(visibility);
    if !selected_surfaces.is_empty() {
        return selected_surfaces;
    }

    let files = json_string_array(package, &["files"]);
    value_at(package, &["selected_surfaces"])
        .and_then(|surfaces| surfaces.as_array())
        .map(|surfaces| {
            surfaces
                .iter()
                .filter_map(|surface| surface.as_str())
                .map(|surface_id| DxCheckPanelPackageLaneSurfaceRow {
                    surface_id: surface_id.to_string(),
                    status: "present".to_string(),
                    files: files.clone(),
                    source_markers: Vec::new(),
                    receipt_path: Some(REACTIVE_STORE_PACKAGE_RECEIPT_PATH.to_string()),
                    reason: None,
                    app_owned_boundary: None,
                })
                .collect()
        })
        .unwrap_or_default()
}

fn state_management_effective_status(
    receipt_status: &str,
    stale_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
) -> &'static str {
    if unsupported_surfaces > 0 {
        "unsupported-surface"
    } else if blocked_surfaces > 0 {
        "blocked"
    } else if stale_receipt > 0 || receipt_status == "stale" {
        "stale"
    } else {
        "present"
    }
}
