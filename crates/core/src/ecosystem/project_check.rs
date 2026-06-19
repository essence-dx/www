//! Project-wide DX quality scoring.

mod ai_sdk_dx_check;
mod authentication_dx_check;
mod automation_connectors_dx_check;
mod backend_platform_client_dx_check;
mod data_fetching_cache_dx_check;
mod database_orm_dx_check;
mod documentation_system_dx_check;
mod file_hashes;
mod forms_dx_check;
mod internationalization_dx_check;
mod markdown_mdx_content_dx_check;
mod motion_animation_dx_check;
mod payments_dx_check;
mod reactive_store_dx_check;
mod readiness;
mod realtime_app_database_dx_check;
mod state_management_dx_check;
mod three_scene_system_dx_check;
mod type_safe_api_dx_check;
mod ui_components_dx_check;
mod validation_schemas_dx_check;
mod wasm_bindgen_dx_check;

pub use readiness::{
    DxCheckFinding, DxCheckMetric, DxCheckOptions, DxCheckReport, DxCheckSection, check_dx_project,
    check_dx_project_with_options, dx_check_report_markdown, forge_launch_gate_findings,
};

use readiness::{
    SOURCE_MANIFEST_PATH, check_finding, check_metric, json_array_entries, json_text,
    read_optional_forge_json, resolve_dx_check_relative_path,
};
