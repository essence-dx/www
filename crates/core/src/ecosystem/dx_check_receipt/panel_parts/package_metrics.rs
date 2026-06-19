#[allow(clippy::too_many_arguments)]
fn authentication_metric_rows(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
    dx_style_compatibility_present: u64,
    dx_style_compatibility_missing: u64,
) -> Vec<DxCheckPanelPackageLaneMetric> {
    [
        package_present,
        receipt_present,
        stale_receipt,
        missing_receipt,
        blocked_surfaces,
        unsupported_surfaces,
        hash_manifest_present,
        hash_mismatches,
        receipt_hash_refresh_current,
        receipt_hash_refresh_stale,
        receipt_hash_refresh_missing,
        dx_style_compatibility_present,
        dx_style_compatibility_missing,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, value)| DxCheckPanelPackageLaneMetric {
        name: AUTHENTICATION_METRICS[index].to_string(),
        value,
    })
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn state_management_metric_rows(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
    dx_style_compatibility_present: u64,
    dx_style_compatibility_missing: u64,
) -> Vec<DxCheckPanelPackageLaneMetric> {
    [
        package_present,
        receipt_present,
        stale_receipt,
        missing_receipt,
        blocked_surfaces,
        unsupported_surfaces,
        receipt_hash_refresh_current,
        receipt_hash_refresh_stale,
        receipt_hash_refresh_missing,
        dx_style_compatibility_present,
        dx_style_compatibility_missing,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, value)| DxCheckPanelPackageLaneMetric {
        name: STATE_MANAGEMENT_METRICS[index].to_string(),
        value,
    })
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn data_fetching_cache_metric_rows(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
    dx_style_compatibility_present: u64,
    dx_style_compatibility_missing: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
) -> Vec<DxCheckPanelPackageLaneMetric> {
    [
        package_present,
        receipt_present,
        stale_receipt,
        missing_receipt,
        blocked_surfaces,
        unsupported_surfaces,
        hash_manifest_present,
        hash_mismatches,
        dx_style_compatibility_present,
        dx_style_compatibility_missing,
        receipt_hash_refresh_current,
        receipt_hash_refresh_stale,
        receipt_hash_refresh_missing,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, value)| DxCheckPanelPackageLaneMetric {
        name: DATA_FETCHING_CACHE_METRICS[index].to_string(),
        value,
    })
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn reactive_store_metric_rows(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
) -> Vec<DxCheckPanelPackageLaneMetric> {
    [
        package_present,
        receipt_present,
        stale_receipt,
        missing_receipt,
        blocked_surfaces,
        unsupported_surfaces,
        hash_manifest_present,
        hash_mismatches,
        receipt_hash_refresh_current,
        receipt_hash_refresh_stale,
        receipt_hash_refresh_missing,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, value)| DxCheckPanelPackageLaneMetric {
        name: REACTIVE_STORE_METRICS[index].to_string(),
        value,
    })
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn database_orm_metric_rows(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
    dx_style_compatibility_present: u64,
    dx_style_compatibility_missing: u64,
) -> Vec<DxCheckPanelPackageLaneMetric> {
    [
        package_present,
        receipt_present,
        stale_receipt,
        missing_receipt,
        blocked_surfaces,
        unsupported_surfaces,
        hash_manifest_present,
        hash_mismatches,
        receipt_hash_refresh_current,
        receipt_hash_refresh_stale,
        receipt_hash_refresh_missing,
        dx_style_compatibility_present,
        dx_style_compatibility_missing,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, value)| DxCheckPanelPackageLaneMetric {
        name: DATABASE_ORM_METRICS[index].to_string(),
        value,
    })
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn forms_metric_rows(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
) -> Vec<DxCheckPanelPackageLaneMetric> {
    [
        package_present,
        receipt_present,
        stale_receipt,
        missing_receipt,
        blocked_surfaces,
        unsupported_surfaces,
        hash_manifest_present,
        hash_mismatches,
        receipt_hash_refresh_current,
        receipt_hash_refresh_stale,
        receipt_hash_refresh_missing,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, value)| DxCheckPanelPackageLaneMetric {
        name: FORMS_METRICS[index].to_string(),
        value,
    })
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn backend_platform_client_metric_rows(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
) -> Vec<DxCheckPanelPackageLaneMetric> {
    [
        package_present,
        receipt_present,
        stale_receipt,
        missing_receipt,
        blocked_surfaces,
        unsupported_surfaces,
        hash_manifest_present,
        hash_mismatches,
        receipt_hash_refresh_current,
        receipt_hash_refresh_stale,
        receipt_hash_refresh_missing,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, value)| DxCheckPanelPackageLaneMetric {
        name: BACKEND_PLATFORM_CLIENT_METRICS[index].to_string(),
        value,
    })
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn payments_metric_rows(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
    dx_style_compatibility_present: u64,
    dx_style_compatibility_missing: u64,
) -> Vec<DxCheckPanelPackageLaneMetric> {
    [
        package_present,
        receipt_present,
        stale_receipt,
        missing_receipt,
        blocked_surfaces,
        unsupported_surfaces,
        hash_manifest_present,
        hash_mismatches,
        receipt_hash_refresh_current,
        receipt_hash_refresh_stale,
        receipt_hash_refresh_missing,
        dx_style_compatibility_present,
        dx_style_compatibility_missing,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, value)| DxCheckPanelPackageLaneMetric {
        name: PAYMENTS_METRICS[index].to_string(),
        value,
    })
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn realtime_app_database_metric_rows(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
    dx_style_compatibility_present: u64,
    dx_style_compatibility_missing: u64,
) -> Vec<DxCheckPanelPackageLaneMetric> {
    [
        package_present,
        receipt_present,
        stale_receipt,
        missing_receipt,
        blocked_surfaces,
        unsupported_surfaces,
        hash_manifest_present,
        hash_mismatches,
        receipt_hash_refresh_current,
        receipt_hash_refresh_stale,
        receipt_hash_refresh_missing,
        dx_style_compatibility_present,
        dx_style_compatibility_missing,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, value)| DxCheckPanelPackageLaneMetric {
        name: REALTIME_APP_DATABASE_METRICS[index].to_string(),
        value,
    })
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn validation_schemas_metric_rows(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
) -> Vec<DxCheckPanelPackageLaneMetric> {
    [
        package_present,
        receipt_present,
        stale_receipt,
        missing_receipt,
        blocked_surfaces,
        unsupported_surfaces,
        hash_manifest_present,
        hash_mismatches,
        receipt_hash_refresh_current,
        receipt_hash_refresh_stale,
        receipt_hash_refresh_missing,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, value)| DxCheckPanelPackageLaneMetric {
        name: VALIDATION_SCHEMAS_METRICS[index].to_string(),
        value,
    })
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn type_safe_api_metric_rows(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
) -> Vec<DxCheckPanelPackageLaneMetric> {
    [
        package_present,
        receipt_present,
        stale_receipt,
        missing_receipt,
        blocked_surfaces,
        unsupported_surfaces,
        hash_manifest_present,
        hash_mismatches,
        receipt_hash_refresh_current,
        receipt_hash_refresh_stale,
        receipt_hash_refresh_missing,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, value)| DxCheckPanelPackageLaneMetric {
        name: TYPE_SAFE_API_METRICS[index].to_string(),
        value,
    })
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn motion_animation_metric_rows(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
) -> Vec<DxCheckPanelPackageLaneMetric> {
    [
        package_present,
        receipt_present,
        stale_receipt,
        missing_receipt,
        blocked_surfaces,
        unsupported_surfaces,
        hash_manifest_present,
        hash_mismatches,
        receipt_hash_refresh_current,
        receipt_hash_refresh_stale,
        receipt_hash_refresh_missing,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, value)| DxCheckPanelPackageLaneMetric {
        name: MOTION_ANIMATION_METRICS[index].to_string(),
        value,
    })
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn ui_components_metric_rows(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
) -> Vec<DxCheckPanelPackageLaneMetric> {
    [
        package_present,
        receipt_present,
        stale_receipt,
        missing_receipt,
        blocked_surfaces,
        unsupported_surfaces,
        hash_manifest_present,
        hash_mismatches,
        receipt_hash_refresh_current,
        receipt_hash_refresh_stale,
        receipt_hash_refresh_missing,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, value)| DxCheckPanelPackageLaneMetric {
        name: UI_COMPONENTS_METRICS[index].to_string(),
        value,
    })
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn internationalization_metric_rows(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
    dx_style_compatibility_present: u64,
    dx_style_compatibility_missing: u64,
) -> Vec<DxCheckPanelPackageLaneMetric> {
    [
        package_present,
        receipt_present,
        stale_receipt,
        missing_receipt,
        blocked_surfaces,
        unsupported_surfaces,
        hash_manifest_present,
        hash_mismatches,
        receipt_hash_refresh_current,
        receipt_hash_refresh_stale,
        receipt_hash_refresh_missing,
        dx_style_compatibility_present,
        dx_style_compatibility_missing,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, value)| DxCheckPanelPackageLaneMetric {
        name: INTERNATIONALIZATION_METRICS[index].to_string(),
        value,
    })
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn documentation_system_metric_rows(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
    dx_style_compatibility_present: u64,
    dx_style_compatibility_missing: u64,
) -> Vec<DxCheckPanelPackageLaneMetric> {
    [
        package_present,
        receipt_present,
        stale_receipt,
        missing_receipt,
        blocked_surfaces,
        unsupported_surfaces,
        hash_manifest_present,
        hash_mismatches,
        receipt_hash_refresh_current,
        receipt_hash_refresh_stale,
        receipt_hash_refresh_missing,
        dx_style_compatibility_present,
        dx_style_compatibility_missing,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, value)| DxCheckPanelPackageLaneMetric {
        name: DOCUMENTATION_SYSTEM_METRICS[index].to_string(),
        value,
    })
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn markdown_mdx_content_metric_rows(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
    dx_style_compatibility_present: u64,
    dx_style_compatibility_missing: u64,
    materialized_source_present: u64,
    materialized_source_missing: u64,
) -> Vec<DxCheckPanelPackageLaneMetric> {
    [
        package_present,
        receipt_present,
        stale_receipt,
        missing_receipt,
        blocked_surfaces,
        unsupported_surfaces,
        hash_manifest_present,
        hash_mismatches,
        receipt_hash_refresh_current,
        receipt_hash_refresh_stale,
        receipt_hash_refresh_missing,
        dx_style_compatibility_present,
        dx_style_compatibility_missing,
        materialized_source_present,
        materialized_source_missing,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, value)| DxCheckPanelPackageLaneMetric {
        name: MARKDOWN_MDX_CONTENT_METRICS[index].to_string(),
        value,
    })
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn ai_sdk_metric_rows(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
    dx_style_compatibility_present: u64,
    dx_style_compatibility_missing: u64,
) -> Vec<DxCheckPanelPackageLaneMetric> {
    [
        package_present,
        receipt_present,
        stale_receipt,
        missing_receipt,
        blocked_surfaces,
        unsupported_surfaces,
        hash_manifest_present,
        hash_mismatches,
        receipt_hash_refresh_current,
        receipt_hash_refresh_stale,
        receipt_hash_refresh_missing,
        dx_style_compatibility_present,
        dx_style_compatibility_missing,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, value)| DxCheckPanelPackageLaneMetric {
        name: AI_SDK_METRICS[index].to_string(),
        value,
    })
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn three_scene_system_metric_rows(
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
    dx_style_compatibility_present: u64,
    dx_style_compatibility_missing: u64,
) -> Vec<DxCheckPanelPackageLaneMetric> {
    [
        receipt_present,
        stale_receipt,
        missing_receipt,
        blocked_surfaces,
        unsupported_surfaces,
        hash_manifest_present,
        hash_mismatches,
        receipt_hash_refresh_current,
        receipt_hash_refresh_stale,
        receipt_hash_refresh_missing,
        dx_style_compatibility_present,
        dx_style_compatibility_missing,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, value)| DxCheckPanelPackageLaneMetric {
        name: THREE_SCENE_SYSTEM_METRICS[index].to_string(),
        value,
    })
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn webassembly_bridge_metric_rows(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
    dx_style_compatibility_present: u64,
    dx_style_compatibility_missing: u64,
) -> Vec<DxCheckPanelPackageLaneMetric> {
    [
        package_present,
        receipt_present,
        stale_receipt,
        missing_receipt,
        blocked_surfaces,
        unsupported_surfaces,
        hash_manifest_present,
        hash_mismatches,
        receipt_hash_refresh_current,
        receipt_hash_refresh_stale,
        receipt_hash_refresh_missing,
        dx_style_compatibility_present,
        dx_style_compatibility_missing,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, value)| DxCheckPanelPackageLaneMetric {
        name: WEBASSEMBLY_BRIDGE_METRICS[index].to_string(),
        value,
    })
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn automation_connectors_metric_rows(
    package_present: u64,
    receipt_present: u64,
    stale_receipt: u64,
    missing_receipt: u64,
    blocked_surfaces: u64,
    unsupported_surfaces: u64,
    hash_manifest_present: u64,
    hash_mismatches: u64,
    dx_style_compatibility_present: u64,
    dx_style_compatibility_missing: u64,
    upstream_runtime_boundary_present: u64,
    upstream_runtime_boundary_missing: u64,
    receipt_hash_refresh_current: u64,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
) -> Vec<DxCheckPanelPackageLaneMetric> {
    [
        package_present,
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
        receipt_hash_refresh_current,
        receipt_hash_refresh_stale,
        receipt_hash_refresh_missing,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, value)| DxCheckPanelPackageLaneMetric {
        name: AUTOMATION_CONNECTORS_METRICS[index].to_string(),
        value,
    })
    .collect()
}

fn authentication_status_vocabulary(package: &serde_json::Value) -> Vec<String> {
    let statuses = json_string_array(package, &["status_vocabulary"]);
    if statuses.is_empty() {
        AUTHENTICATION_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect()
    } else {
        statuses
    }
}

fn status_vocabulary(visibility: &serde_json::Value) -> Vec<String> {
    let statuses = json_string_array(visibility, &["status_vocabulary"]);
    if statuses.is_empty() {
        STATE_MANAGEMENT_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect()
    } else {
        statuses
    }
}

fn reactive_store_status_vocabulary(visibility: &serde_json::Value) -> Vec<String> {
    let statuses = json_string_array(visibility, &["status_vocabulary"]);
    if statuses.is_empty() {
        REACTIVE_STORE_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect()
    } else {
        statuses
    }
}

fn data_fetching_cache_status_vocabulary(package: &serde_json::Value) -> Vec<String> {
    let statuses = json_string_array(package, &["status_vocabulary"]);
    if statuses.is_empty() {
        DATA_FETCHING_CACHE_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect()
    } else {
        statuses
    }
}

fn forms_status_vocabulary(package: &serde_json::Value) -> Vec<String> {
    let statuses = json_string_array(package, &["status_vocabulary"]);
    if statuses.is_empty() {
        FORMS_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect()
    } else {
        statuses
    }
}

fn backend_platform_client_status_vocabulary(package: &serde_json::Value) -> Vec<String> {
    let statuses = json_string_array(package, &["status_vocabulary"]);
    if statuses.is_empty() {
        BACKEND_PLATFORM_CLIENT_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect()
    } else {
        statuses
    }
}

fn payments_status_vocabulary(package: &serde_json::Value) -> Vec<String> {
    let statuses = json_string_array(package, &["status_vocabulary"]);
    if statuses.is_empty() {
        PAYMENTS_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect()
    } else {
        statuses
    }
}

fn validation_schemas_status_vocabulary(package: &serde_json::Value) -> Vec<String> {
    let statuses = json_string_array(package, &["status_vocabulary"]);
    if statuses.is_empty() {
        VALIDATION_SCHEMAS_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect()
    } else {
        statuses
    }
}

fn type_safe_api_status_vocabulary(package: &serde_json::Value) -> Vec<String> {
    let statuses = json_string_array(package, &["status_vocabulary"]);
    if statuses.is_empty() {
        TYPE_SAFE_API_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect()
    } else {
        statuses
    }
}

fn motion_animation_status_vocabulary(package: &serde_json::Value) -> Vec<String> {
    let statuses = json_string_array(package, &["status_vocabulary"]);
    if statuses.is_empty() {
        MOTION_ANIMATION_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect()
    } else {
        statuses
    }
}

fn realtime_app_database_status_vocabulary(package: &serde_json::Value) -> Vec<String> {
    let statuses = json_string_array(package, &["status_vocabulary"]);
    if statuses.is_empty() {
        REALTIME_APP_DATABASE_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect()
    } else {
        statuses
    }
}

fn ui_components_status_vocabulary(package: &serde_json::Value) -> Vec<String> {
    let statuses = json_string_array(package, &["status_vocabulary"]);
    if statuses.is_empty() {
        UI_COMPONENTS_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect()
    } else {
        statuses
    }
}

fn database_orm_status_vocabulary(package: &serde_json::Value) -> Vec<String> {
    let statuses = json_string_array(package, &["status_vocabulary"]);
    if statuses.is_empty() {
        DATABASE_ORM_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect()
    } else {
        statuses
    }
}

fn internationalization_status_vocabulary(package: &serde_json::Value) -> Vec<String> {
    let statuses = json_string_array(package, &["status_vocabulary"]);
    if statuses.is_empty() {
        INTERNATIONALIZATION_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect()
    } else {
        statuses
    }
}

fn documentation_system_status_vocabulary(package: &serde_json::Value) -> Vec<String> {
    let statuses = json_string_array(package, &["status_vocabulary"]);
    if statuses.is_empty() {
        DOCUMENTATION_SYSTEM_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect()
    } else {
        statuses
    }
}

fn markdown_mdx_content_status_vocabulary(package: &serde_json::Value) -> Vec<String> {
    let statuses = json_string_array(package, &["status_vocabulary"]);
    if statuses.is_empty() {
        MARKDOWN_MDX_CONTENT_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect()
    } else {
        statuses
    }
}

fn ai_sdk_status_vocabulary(package: &serde_json::Value) -> Vec<String> {
    let statuses = json_string_array(package, &["status_vocabulary"]);
    if statuses.is_empty() {
        AI_SDK_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect()
    } else {
        statuses
    }
}

fn three_scene_system_status_vocabulary(package: &serde_json::Value) -> Vec<String> {
    let statuses = json_string_array(package, &["status_vocabulary"]);
    if statuses.is_empty() {
        THREE_SCENE_SYSTEM_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect()
    } else {
        statuses
    }
}

fn webassembly_bridge_status_vocabulary(package: &serde_json::Value) -> Vec<String> {
    let statuses = json_string_array(package, &["status_vocabulary"]);
    if statuses.is_empty() {
        WEBASSEMBLY_BRIDGE_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect()
    } else {
        statuses
    }
}

fn automation_connectors_status_vocabulary(package: &serde_json::Value) -> Vec<String> {
    let statuses = json_string_array(package, &["status_vocabulary"]);
    if statuses.is_empty() {
        AUTOMATION_CONNECTORS_STATUS_VOCABULARY
            .iter()
            .map(|status| (*status).to_string())
            .collect()
    } else {
        statuses
    }
}

fn reactive_store_runtime_limitations(
    visibility: &serde_json::Value,
    package: &serde_json::Value,
) -> Vec<String> {
    let visibility_limitations = json_string_array(visibility, &["runtime_limitations"]);
    if visibility_limitations.is_empty() {
        json_string_array(package, &["runtime_limitations"])
    } else {
        visibility_limitations
    }
}

fn package_lane_hash_refresh(
    package: &serde_json::Value,
) -> Option<DxCheckPanelPackageLaneHashRefreshRow> {
    let refresh = value_at(package, &["receipt_hash_refresh"])?;
    Some(DxCheckPanelPackageLaneHashRefreshRow {
        schema: json_text(refresh, &["schema"])
            .unwrap_or("dx.forge.package.receipt_hash_refresh")
            .to_string(),
        status: json_text(refresh, &["status"])
            .unwrap_or("missing")
            .to_string(),
        helper_path: json_text(refresh, &["helper_path"])
            .unwrap_or_default()
            .to_string(),
        check_command: json_text(refresh, &["check_command"])
            .unwrap_or_default()
            .to_string(),
        write_command: json_text(refresh, &["write_command"])
            .unwrap_or_default()
            .to_string(),
        json_check_command: json_text(refresh, &["json_check_command"])
            .unwrap_or_default()
            .to_string(),
        receipt_path: json_text(refresh, &["receipt_path"])
            .unwrap_or_default()
            .to_string(),
        hash_algorithm: json_text(refresh, &["hash_algorithm"])
            .unwrap_or("sha256")
            .to_string(),
        tracked_files: json_string_array(refresh, &["tracked_files"]),
        source_guard_runbook_fixture: json_text(refresh, &["source_guard_runbook_fixture"])
            .map(str::to_string),
        preview_manifest_materializer: json_text(refresh, &["preview_manifest_materializer"])
            .map(str::to_string),
        studio_manifest_source: json_text(refresh, &["studio_manifest_source"]).map(str::to_string),
        lower_dx_check_source: json_text(refresh, &["lower_dx_check_source"]).map(str::to_string),
        check_panel_source: json_text(refresh, &["check_panel_source"]).map(str::to_string),
        tracked_file_count: json_u64(refresh, &["tracked_file_count"]).unwrap_or(0),
        stale_file_count: json_u64(refresh, &["stale_file_count"]).unwrap_or(0),
        missing_file_count: json_u64(refresh, &["missing_file_count"]).unwrap_or(0),
        current_files: json_string_array(refresh, &["current_files"]),
        stale_files: json_string_array(refresh, &["stale_files"]),
        missing_files: json_string_array(refresh, &["missing_files"]),
        stale_mirror_files: json_string_array(refresh, &["stale_mirror_files"]),
        missing_mirror_files: json_string_array(refresh, &["missing_mirror_files"]),
        mirror_problem_count: json_u64(refresh, &["mirror_problem_count"]).unwrap_or_else(|| {
            (json_string_array(refresh, &["stale_mirror_files"]).len()
                + json_string_array(refresh, &["missing_mirror_files"]).len()) as u64
        }),
        runtime_execution: json_bool(refresh, &["runtime_execution"]).unwrap_or(false),
        secret_access: json_bool(refresh, &["secret_access"]).unwrap_or(false),
        zed_visibility: json_text(refresh, &["zed_visibility"])
            .unwrap_or_default()
            .to_string(),
        runtime_limitations: json_string_array(refresh, &["runtime_limitations"]),
    })
}

fn receipt_hash_refresh_counts(package: &serde_json::Value) -> (u64, u64, u64) {
    let Some(refresh) = value_at(package, &["receipt_hash_refresh"]) else {
        return (0, 0, 1);
    };
    if json_text(refresh, &["schema"]) != Some("dx.forge.package.receipt_hash_refresh") {
        return (0, 0, 1);
    }

    let stale_file_count = json_u64(refresh, &["stale_file_count"]).unwrap_or(0);
    let missing_file_count = json_u64(refresh, &["missing_file_count"]).unwrap_or(0);
    let stale_path_count = json_string_array(refresh, &["stale_files"]).len() as u64
        + json_string_array(refresh, &["stale_mirror_files"]).len() as u64;
    let missing_path_count = json_string_array(refresh, &["missing_files"]).len() as u64
        + json_string_array(refresh, &["missing_mirror_files"]).len() as u64;
    let status = json_text(refresh, &["status"]).unwrap_or("missing");
    let stale = u64::from(status == "stale" || stale_file_count > 0 || stale_path_count > 0);
    let missing =
        u64::from(status == "missing" || missing_file_count > 0 || missing_path_count > 0);
    let current = u64::from(status == "current" && stale == 0 && missing == 0);

    (current, stale, missing)
}

fn ui_components_hash_refresh_row(
    package: &serde_json::Value,
) -> Option<DxCheckPanelPackageLaneHashRefreshRow> {
    let refresh = value_at(package, &["receipt_hash_refresh"])?;
    Some(DxCheckPanelPackageLaneHashRefreshRow {
        schema: json_text(refresh, &["schema"])
            .unwrap_or("dx.forge.package.receipt_hash_refresh")
            .to_string(),
        status: json_text(refresh, &["status"])
            .unwrap_or("missing")
            .to_string(),
        helper_path: json_text(refresh, &["helper_path"])
            .unwrap_or("examples/template/ui-components-receipt-hashes.ts")
            .to_string(),
        check_command: json_text(refresh, &["check_command"])
            .unwrap_or("node examples/template/ui-components-receipt-hashes.ts --check")
            .to_string(),
        write_command: json_text(refresh, &["write_command"])
            .unwrap_or("node examples/template/ui-components-receipt-hashes.ts --write")
            .to_string(),
        json_check_command: json_text(refresh, &["json_check_command"])
            .unwrap_or("node examples/template/ui-components-receipt-hashes.ts --check --json")
            .to_string(),
        receipt_path: json_text(refresh, &["receipt_path"])
            .unwrap_or(UI_COMPONENTS_PACKAGE_RECEIPT_PATH)
            .to_string(),
        hash_algorithm: json_text(refresh, &["hash_algorithm"])
            .unwrap_or("sha256")
            .to_string(),
        tracked_files: json_string_array(refresh, &["tracked_files"]),
        source_guard_runbook_fixture: json_text(refresh, &["source_guard_runbook_fixture"])
            .map(str::to_string),
        preview_manifest_materializer: json_text(refresh, &["preview_manifest_materializer"])
            .map(str::to_string),
        studio_manifest_source: json_text(refresh, &["studio_manifest_source"]).map(str::to_string),
        lower_dx_check_source: json_text(refresh, &["lower_dx_check_source"]).map(str::to_string),
        check_panel_source: json_text(refresh, &["check_panel_source"]).map(str::to_string),
        tracked_file_count: json_u64(refresh, &["tracked_file_count"]).unwrap_or(0),
        stale_file_count: json_u64(refresh, &["stale_file_count"]).unwrap_or(0),
        missing_file_count: json_u64(refresh, &["missing_file_count"]).unwrap_or(0),
        current_files: json_string_array(refresh, &["current_files"]),
        stale_files: json_string_array(refresh, &["stale_files"]),
        missing_files: json_string_array(refresh, &["missing_files"]),
        stale_mirror_files: json_string_array(refresh, &["stale_mirror_files"]),
        missing_mirror_files: json_string_array(refresh, &["missing_mirror_files"]),
        mirror_problem_count: json_u64(refresh, &["mirror_problem_count"]).unwrap_or_else(|| {
            (json_string_array(refresh, &["stale_mirror_files"]).len()
                + json_string_array(refresh, &["missing_mirror_files"]).len()) as u64
        }),
        runtime_execution: json_bool(refresh, &["runtime_execution"]).unwrap_or(false),
        secret_access: json_bool(refresh, &["secret_access"]).unwrap_or(false),
        zed_visibility: json_text(refresh, &["zed_visibility"])
            .unwrap_or("ui-components:receipt-hash-refresh")
            .to_string(),
        runtime_limitations: json_string_array(refresh, &["runtime_limitations"]),
    })
}

fn ui_components_runtime_limitations(package: &serde_json::Value) -> Vec<String> {
    let mut limitations = json_string_array(package, &["runtime_limitations"]);
    if let Some(refresh) = value_at(package, &["receipt_hash_refresh"]) {
        for limitation in json_string_array(refresh, &["runtime_limitations"]) {
            if !limitations.iter().any(|existing| existing == &limitation) {
                limitations.push(limitation);
            }
        }
    }
    limitations
}

fn data_fetching_cache_runtime_limitations(package: &serde_json::Value) -> Vec<String> {
    let mut limitations = json_string_array(package, &["runtime_limitations"]);
    if let Some(refresh) = value_at(package, &["receipt_hash_refresh"]) {
        for limitation in json_string_array(refresh, &["runtime_limitations"]) {
            if !limitations.iter().any(|existing| existing == &limitation) {
                limitations.push(limitation);
            }
        }
    }
    limitations
}

fn ai_sdk_checkable_hash_manifest(value: &serde_json::Value) -> serde_json::Value {
    let mut checkable = value.clone();
    prune_ai_sdk_provenance_hashes(&mut checkable);
    checkable
}

fn prune_ai_sdk_provenance_hashes(value: &mut serde_json::Value) {
    let Some(object) = value.as_object_mut() else {
        return;
    };

    if let Some(file_hashes) = object
        .get_mut("file_hashes")
        .and_then(|hashes| hashes.as_object_mut())
    {
        file_hashes.retain(|key, _| {
            !(key.starts_with("upstream:") || key.starts_with("core/") || key.starts_with("docs/"))
        });
    }

    if let Some(surfaces) = object
        .get_mut("selected_surfaces")
        .and_then(|surfaces| surfaces.as_array_mut())
    {
        for surface in surfaces {
            prune_ai_sdk_provenance_hashes(surface);
        }
    }
}

fn has_sha256_file_hashes(value: &serde_json::Value) -> bool {
    json_text(value, &["hash_algorithm"]) == Some("sha256")
        && value_at(value, &["file_hashes"])
            .and_then(|hashes| hashes.as_object())
            .is_some_and(|hashes| !hashes.is_empty())
        || value_at(value, &["selected_surfaces"])
            .and_then(|surfaces| surfaces.as_array())
            .is_some_and(|surfaces| surfaces.iter().any(has_sha256_file_hashes))
}

fn has_sha256_source_hashes(value: &serde_json::Value) -> bool {
    json_text(value, &["source_hashes", "algorithm"]) == Some("sha256")
        && value_at(value, &["source_hashes", "files"])
            .and_then(|hashes| hashes.as_object())
            .is_some_and(|hashes| !hashes.is_empty())
}

fn count_sha256_file_hash_mismatches(root: &Path, value: &serde_json::Value) -> u64 {
    let own_mismatches = if json_text(value, &["hash_algorithm"]) == Some("sha256") {
        value_at(value, &["file_hashes"])
            .and_then(|hashes| hashes.as_object())
            .map(|hashes| {
                hashes
                    .iter()
                    .filter(|(relative_path, expected_hash)| {
                        let Some(expected_hash) = expected_hash.as_str() else {
                            return true;
                        };
                        match sha256_project_file(root, relative_path) {
                            Some(actual_hash) => {
                                normalize_sha256_hash(expected_hash) != actual_hash
                            }
                            None => true,
                        }
                    })
                    .count() as u64
            })
            .unwrap_or(0)
    } else {
        0
    };

    own_mismatches
        + value_at(value, &["selected_surfaces"])
            .and_then(|surfaces| surfaces.as_array())
            .map(|surfaces| {
                surfaces
                    .iter()
                    .map(|surface| count_sha256_file_hash_mismatches(root, surface))
                    .sum::<u64>()
            })
            .unwrap_or(0)
}

fn dx_style_compatibility_is_present(value: &serde_json::Value) -> bool {
    let Some(dx_style) = value_at(value, &["dx_style_compatibility"]) else {
        return false;
    };

    json_text(dx_style, &["schema"]) == Some("dx.forge.package.dx_style_compatibility")
        && json_text(dx_style, &["status"]).unwrap_or("present") == "present"
}

fn automation_connectors_has_upstream_runtime_boundary(value: &serde_json::Value) -> bool {
    json_array_contains_all(
        value,
        &["inspected_upstream_files"],
        AUTOMATION_CONNECTORS_REQUIRED_UPSTREAM_FILES,
    ) && json_array_contains_all(
        value,
        &["upstream_public_apis"],
        AUTOMATION_CONNECTORS_REQUIRED_UPSTREAM_PUBLIC_APIS,
    )
}

fn markdown_mdx_content_materialized_source_is_present(value: &serde_json::Value) -> bool {
    let Some(materialized_source) = value_at(value, &["materialized_source"]) else {
        return false;
    };
    let execution_guard = json_text(materialized_source, &["execution_guard"]).unwrap_or("");

    json_text(materialized_source, &["schema"]) == Some("dx.forge.package.materialized_source")
        && json_text(materialized_source, &["source_file"])
            == Some("lib/markdown-mdx-content/receipt.ts")
        && json_text(materialized_source, &["materialized_file"])
            == Some("lib/markdown-mdx-content/receipt.ts")
        && json_text(materialized_source, &["surface"]) == Some("forge-receipt-helper")
        && execution_guard.contains("markdown-mdx-content-slice.test.ts")
        && json_bool(materialized_source, &["runtime_proof"]) == Some(false)
}

fn count_sha256_source_hash_mismatches(root: &Path, value: &serde_json::Value) -> u64 {
    if json_text(value, &["source_hashes", "algorithm"]) != Some("sha256") {
        return 0;
    }

    value_at(value, &["source_hashes", "files"])
        .and_then(|hashes| hashes.as_object())
        .map(|hashes| {
            hashes
                .iter()
                .filter(|(relative_path, expected_hash)| {
                    let Some(expected_hash) = expected_hash.as_str() else {
                        return true;
                    };
                    match sha256_project_file(root, relative_path) {
                        Some(actual_hash) => normalize_sha256_hash(expected_hash) != actual_hash,
                        None => true,
                    }
                })
                .count() as u64
        })
        .unwrap_or(0)
}

fn sha256_project_file(root: &Path, relative_path: &str) -> Option<String> {
    let bytes = fs::read(project_file_path(root, relative_path)?).ok()?;
    Some(format!("{:x}", Sha256::digest(bytes)))
}

fn project_file_path(root: &Path, relative_path: &str) -> Option<PathBuf> {
    let path = Path::new(relative_path);
    if path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return None;
    }

    let normalized = relative_path.replace('\\', "/");
    let mut candidates = vec![root.join(path)];

    if let Some(template_relative) = normalized.strip_prefix("examples/template/") {
        candidates.push(root.join(Path::new(template_relative)));
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

    candidates.into_iter().find(|path| path.is_file())
}
