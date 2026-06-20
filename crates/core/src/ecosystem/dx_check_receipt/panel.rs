//! Reader for root dx-check receipts used by DX-WWW and future editor panels.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use super::super::dx_style_receipts::{
    dx_style_package_ownership_summary, dx_style_rule_metadata_summary,
};

/// Stable location for the latest dx-check machine receipt.
pub const DX_CHECK_LATEST_RECEIPT_PATH: &str = ".dx/receipts/check/check-latest.json";
/// DX-WWW wrapper schema for a renderable check panel state.
pub const DX_WWW_CHECK_PANEL_SCHEMA_VERSION: &str = "dx.www.check_panel";
/// DX-WWW render view model schema for Studio/Web Preview and future GPUI panels.
pub const DX_WWW_CHECK_PANEL_VIEW_MODEL_SCHEMA_VERSION: &str = "dx.www.check_panel_view_model";
/// Zed-facing panel schema emitted by the root dx-check CLI.
pub const DX_CHECK_ZED_PANEL_SCHEMA_VERSION: &str = "dx.check.zed_panel.v1";
/// Launch-default scoring profile emitted by dx-check.
pub const DX_CHECK_WEIGHT_PROFILE: &str = "dx-check.launch-default";

const DX_CHECK_ZED_PANEL_LEGACY_SCHEMA_VERSION: &str = "dx.check.zed_panel";
const DX_STYLE_BROWSER_COMPAT_ROW_ID: &str = "dx-style-browser-compat";
const DX_STYLE_TAILWIND_EQUAL_OUTPUT_ROW_ID: &str = "dx-style-tailwind-equal-output";
const DX_STYLE_PACKAGE_OWNERSHIP_ROW_ID: &str = "dx-style-package-ownership";
const DX_STYLE_RULE_METADATA_ROW_ID: &str = "dx-style-rule-metadata";
const DX_STYLE_BROWSER_COMPAT_FIXTURE_PATH: &str =
    "related-crates/style/fixtures/tailwind-postcss-browser-compat.json";
const DX_STYLE_TAILWIND_EQUAL_OUTPUT_FIXTURE_PATH: &str =
    "related-crates/style/fixtures/tailwind-equal-output-canary.json";
const DX_STYLE_CHECK_RECEIPT_PATH_FOR_PANEL: &str = ".dx/receipts/style/check.json";
const DX_STYLE_PACKAGE_OWNERSHIP_FIXTURE_PATH: &str = ".dx/forge/package-status.json";
const DX_STYLE_BROWSER_COMPAT_METRICS: [&str; 16] = [
    "dx_style_browser_compat_receipt_present",
    "dx_style_browser_compat_contract_present",
    "dx_style_browser_compat_schema_supported",
    "dx_style_browser_compat_class_count",
    "dx_style_browser_compat_selector_class_count",
    "dx_style_browser_compat_full_autoprefixer_parity",
    "dx_style_browser_compat_full_tailwind_postcss_output_parity",
    "dx_style_tailwind_parity_state_alias_supported_classes",
    "postcss_compat_supported_count",
    "postcss_compat_partial_count",
    "postcss_compat_unsupported_count",
    "dx_starter_replacement_score",
    "full_postcss_plugin_parity",
    "postcss_runtime_dependency_required",
    "local_postcss_config_required",
    "unsupported_transform_warnings",
];
const DX_STYLE_BROWSER_COMPAT_SELECTOR_EXAMPLES: [&str; 1] = ["file:p-4"];
const DX_STYLE_TAILWIND_EQUAL_OUTPUT_METRICS: [&str; 9] = [
    "dx_style_tailwind_equal_output_receipt_present",
    "dx_style_tailwind_equal_output_contract_present",
    "dx_style_tailwind_equal_output_schema_supported",
    "dx_style_tailwind_equal_output_class_count",
    "dx_style_tailwind_equal_output_equal_class_count",
    "dx_style_tailwind_equal_output_unsupported_classes",
    "dx_style_tailwind_equal_output_live_tailwind_execution",
    "dx_style_tailwind_equal_output_full_tailwind_parity",
    "dx_style_tailwind_equal_output_fair_speed_benchmark",
];
const DX_STYLE_TAILWIND_PARITY_STATE_ALIAS_EXAMPLES: [&str; 6] = [
    "target:p-4",
    "read-only:bg-blue-500",
    "indeterminate:opacity-100",
    "has-even:bg-blue-500",
    "not-visited:text-slate-900",
    "in-read-only:p-4",
];
const DX_STYLE_PACKAGE_OWNERSHIP_METRICS: [&str; 4] = [
    "dx_style_package_ownership_receipt_present",
    "dx_style_package_ownership_package_count",
    "dx_style_package_ownership_generated_class_count",
    "dx_style_package_ownership_unsupported_class_count",
];
const DX_STYLE_RULE_METADATA_METRICS: [&str; 6] = [
    "dx_style_rule_metadata_receipt_present",
    "dx_style_rule_metadata_class_count",
    "dx_style_rule_metadata_editable_class_count",
    "dx_style_rule_metadata_visual_property_count",
    "dx_style_rule_metadata_token_reference_count",
    "dx_style_rule_metadata_source_file_count",
];

const AUTHENTICATION_PACKAGE_ID: &str = "auth/better-auth";
const AUTHENTICATION_OFFICIAL_NAME: &str = "Authentication";
const AUTHENTICATION_UPSTREAM_PACKAGE: &str = "better-auth";
const AUTHENTICATION_UPSTREAM_VERSION: &str = "1.6.11";
const AUTHENTICATION_SOURCE_MIRROR: &str = "G:/WWW/inspirations/better-auth";
const AUTHENTICATION_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";
const AUTHENTICATION_PACKAGE_RECEIPT_PATH: &str = ".dx/forge/receipts/auth-better-auth.json";
const AUTHENTICATION_STATUS_VOCABULARY: [&str; 5] = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
];
const AUTHENTICATION_METRICS: [&str; 13] = [
    "authentication_package_present",
    "authentication_receipt_present",
    "authentication_receipt_stale",
    "authentication_missing_receipt",
    "authentication_blocked_surface",
    "authentication_unsupported_surface",
    "authentication_hash_manifest_present",
    "authentication_hash_mismatch",
    "authentication_receipt_hash_refresh_current",
    "authentication_receipt_hash_refresh_stale",
    "authentication_receipt_hash_refresh_missing",
    "authentication_dx_style_compatibility_present",
    "authentication_dx_style_compatibility_missing",
];
const STATE_MANAGEMENT_PACKAGE_ID: &str = "state/zustand";
const STATE_MANAGEMENT_OFFICIAL_NAME: &str = "State Management";
const STATE_MANAGEMENT_UPSTREAM_PACKAGE: &str = "zustand";
const STATE_MANAGEMENT_UPSTREAM_VERSION: &str = "5.0.13";
const STATE_MANAGEMENT_SOURCE_MIRROR: &str = "G:/WWW/inspirations/zustand";
const STATE_MANAGEMENT_PACKAGE_RECEIPT_PATH: &str =
    ".dx/forge/receipts/packages/state-zustand.json";
#[cfg(test)]
#[allow(dead_code)]
const STATE_MANAGEMENT_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";
const SOURCE_MANIFEST_PATH: &str = ".dx/forge/source-.dx/build-cache/manifest.json";
const STATE_MANAGEMENT_STATUS_VOCABULARY: [&str; 5] = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
];
const STATE_MANAGEMENT_METRICS: [&str; 11] = [
    "state_management_package_present",
    "state_management_receipt_present",
    "state_management_receipt_stale",
    "state_management_missing_receipt",
    "state_management_blocked_surface",
    "state_management_unsupported_surface",
    "state_management_receipt_hash_refresh_current",
    "state_management_receipt_hash_refresh_stale",
    "state_management_receipt_hash_refresh_missing",
    "state_management_dx_style_compatibility_present",
    "state_management_dx_style_compatibility_missing",
];
const DATA_FETCHING_CACHE_PACKAGE_ID: &str = "tanstack/query";
const DATA_FETCHING_CACHE_OFFICIAL_NAME: &str = "Data Fetching & Cache";
const DATA_FETCHING_CACHE_UPSTREAM_PACKAGE: &str = "@tanstack/react-query";
const DATA_FETCHING_CACHE_UPSTREAM_VERSION: &str = "5.100.10";
const DATA_FETCHING_CACHE_SOURCE_MIRROR: &str = "G:/WWW/inspirations/tanstack-query";
#[cfg(test)]
#[allow(dead_code)]
const DATA_FETCHING_CACHE_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";
const DATA_FETCHING_CACHE_PACKAGE_RECEIPT_PATH: &str =
    ".dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json";
const DATA_FETCHING_CACHE_STATUS_VOCABULARY: [&str; 5] = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
];
const DATA_FETCHING_CACHE_METRICS: [&str; 13] = [
    "data_fetching_cache_package_present",
    "data_fetching_cache_receipt_present",
    "data_fetching_cache_receipt_stale",
    "data_fetching_cache_missing_receipt",
    "data_fetching_cache_blocked_surface",
    "data_fetching_cache_unsupported_surface",
    "data_fetching_cache_hash_manifest_present",
    "data_fetching_cache_hash_mismatch",
    "data_fetching_cache_dx_style_compatibility_present",
    "data_fetching_cache_dx_style_compatibility_missing",
    "data_fetching_cache_receipt_hash_refresh_current",
    "data_fetching_cache_receipt_hash_refresh_stale",
    "data_fetching_cache_receipt_hash_refresh_missing",
];
const REACTIVE_STORE_PACKAGE_ID: &str = "reactive/store";
const REACTIVE_STORE_OFFICIAL_NAME: &str = "Reactive Store";
const REACTIVE_STORE_UPSTREAM_PACKAGE: &str = "@tanstack/store";
const REACTIVE_STORE_UPSTREAM_VERSION: &str = "0.11.0";
const REACTIVE_STORE_SOURCE_MIRROR: &str = "G:/WWW/inspirations/tanstack-store";
#[cfg(test)]
#[allow(dead_code)]
const REACTIVE_STORE_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";
const REACTIVE_STORE_PACKAGE_RECEIPT_PATH: &str = ".dx/forge/receipts/packages/reactive-store.json";
const REACTIVE_STORE_STATUS_VOCABULARY: [&str; 5] = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
];
const REACTIVE_STORE_METRICS: [&str; 11] = [
    "reactive_store_package_present",
    "reactive_store_receipt_present",
    "reactive_store_receipt_stale",
    "reactive_store_missing_receipt",
    "reactive_store_blocked_surface",
    "reactive_store_unsupported_surface",
    "reactive_store_hash_manifest_present",
    "reactive_store_hash_mismatch",
    "reactive_store_receipt_hash_refresh_current",
    "reactive_store_receipt_hash_refresh_stale",
    "reactive_store_receipt_hash_refresh_missing",
];
const DATABASE_ORM_PACKAGE_ID: &str = "db/drizzle-sqlite";
const DATABASE_ORM_OFFICIAL_NAME: &str = "Database ORM";
const DATABASE_ORM_UPSTREAM_PACKAGE: &str = "drizzle-orm";
const DATABASE_ORM_UPSTREAM_VERSION: &str = "0.45.3";
const DATABASE_ORM_SOURCE_MIRROR: &str = "G:/WWW/inspirations/drizzle-orm";
#[cfg(test)]
#[allow(dead_code)]
const DATABASE_ORM_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";
const DATABASE_ORM_PACKAGE_RECEIPT_PATH: &str =
    "examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json";
const DATABASE_ORM_STATUS_VOCABULARY: [&str; 5] = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
];
const DATABASE_ORM_METRICS: [&str; 13] = [
    "database_orm_package_present",
    "database_orm_receipt_present",
    "database_orm_receipt_stale",
    "database_orm_missing_receipt",
    "database_orm_blocked_surface",
    "database_orm_unsupported_surface",
    "database_orm_hash_manifest_present",
    "database_orm_hash_mismatch",
    "database_orm_receipt_hash_refresh_current",
    "database_orm_receipt_hash_refresh_stale",
    "database_orm_receipt_hash_refresh_missing",
    "database_orm_dx_style_compatibility_present",
    "database_orm_dx_style_compatibility_missing",
];
const FORMS_PACKAGE_ID: &str = "forms/react-hook-form";
const FORMS_OFFICIAL_NAME: &str = "Forms";
const FORMS_UPSTREAM_PACKAGE: &str = "react-hook-form";
const FORMS_UPSTREAM_VERSION: &str = "7.75.0";
const FORMS_SOURCE_MIRROR: &str = "G:/WWW/inspirations/react-hook-form";
#[cfg(test)]
#[allow(dead_code)]
const FORMS_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";
const FORMS_PACKAGE_RECEIPT_PATH: &str =
    ".dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json";
const FORMS_STATUS_VOCABULARY: [&str; 5] = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
];
const FORMS_METRICS: [&str; 11] = [
    "forms_package_present",
    "forms_receipt_present",
    "forms_receipt_stale",
    "forms_missing_receipt",
    "forms_blocked_surface",
    "forms_unsupported_surface",
    "forms_hash_manifest_present",
    "forms_hash_mismatch",
    "forms_receipt_hash_refresh_current",
    "forms_receipt_hash_refresh_stale",
    "forms_receipt_hash_refresh_missing",
];
const VALIDATION_SCHEMAS_PACKAGE_ID: &str = "validation/zod";
const VALIDATION_SCHEMAS_OFFICIAL_NAME: &str = "Validation & Schemas";
const VALIDATION_SCHEMAS_UPSTREAM_PACKAGE: &str = "zod";
const VALIDATION_SCHEMAS_UPSTREAM_VERSION: &str = "4.4.3";
const VALIDATION_SCHEMAS_SOURCE_MIRROR: &str = "G:/WWW/inspirations/zod";
#[cfg(test)]
#[allow(dead_code)]
const VALIDATION_SCHEMAS_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";
const VALIDATION_SCHEMAS_PACKAGE_RECEIPT_PATH: &str =
    "examples/template/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json";
const VALIDATION_SCHEMAS_STATUS_VOCABULARY: [&str; 5] = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
];
const VALIDATION_SCHEMAS_METRICS: [&str; 11] = [
    "validation_schemas_package_present",
    "validation_schemas_receipt_present",
    "validation_schemas_receipt_stale",
    "validation_schemas_missing_receipt",
    "validation_schemas_blocked_surface",
    "validation_schemas_unsupported_surface",
    "validation_schemas_hash_manifest_present",
    "validation_schemas_hash_mismatch",
    "validation_schemas_receipt_hash_refresh_current",
    "validation_schemas_receipt_hash_refresh_stale",
    "validation_schemas_receipt_hash_refresh_missing",
];
const TYPE_SAFE_API_PACKAGE_ID: &str = "api/trpc";
const TYPE_SAFE_API_OFFICIAL_NAME: &str = "Type-Safe API";
const TYPE_SAFE_API_UPSTREAM_PACKAGE: &str = "@trpc/server";
const TYPE_SAFE_API_UPSTREAM_VERSION: &str = "11.17.0";
const TYPE_SAFE_API_SOURCE_MIRROR: &str = "G:/WWW/inspirations/trpc";
#[cfg(test)]
#[allow(dead_code)]
const TYPE_SAFE_API_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";
const TYPE_SAFE_API_PACKAGE_RECEIPT_PATH: &str =
    "examples/template/.dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json";
const TYPE_SAFE_API_STATUS_VOCABULARY: [&str; 5] = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
];
const TYPE_SAFE_API_METRICS: [&str; 11] = [
    "type_safe_api_package_present",
    "type_safe_api_receipt_present",
    "type_safe_api_receipt_stale",
    "type_safe_api_missing_receipt",
    "type_safe_api_blocked_surface",
    "type_safe_api_unsupported_surface",
    "type_safe_api_hash_manifest_present",
    "type_safe_api_hash_mismatch",
    "type_safe_api_receipt_hash_refresh_current",
    "type_safe_api_receipt_hash_refresh_stale",
    "type_safe_api_receipt_hash_refresh_missing",
];
const BACKEND_PLATFORM_CLIENT_PACKAGE_ID: &str = "supabase/client";
const BACKEND_PLATFORM_CLIENT_OFFICIAL_NAME: &str = "Backend Platform Client";
const BACKEND_PLATFORM_CLIENT_UPSTREAM_PACKAGE: &str = "@supabase/ssr + @supabase/supabase-js";
const BACKEND_PLATFORM_CLIENT_UPSTREAM_VERSION: &str =
    "@supabase/ssr latest; @supabase/supabase-js ^2";
const BACKEND_PLATFORM_CLIENT_SOURCE_MIRROR: &str = "G:/WWW/inspirations/supabase";
#[cfg(test)]
#[allow(dead_code)]
const BACKEND_PLATFORM_CLIENT_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";
const BACKEND_PLATFORM_CLIENT_PACKAGE_RECEIPT_PATH: &str =
    "examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json";
const BACKEND_PLATFORM_CLIENT_STATUS_VOCABULARY: [&str; 5] = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
];
const BACKEND_PLATFORM_CLIENT_METRICS: [&str; 11] = [
    "backend_platform_client_package_present",
    "backend_platform_client_receipt_present",
    "backend_platform_client_receipt_stale",
    "backend_platform_client_missing_receipt",
    "backend_platform_client_blocked_surface",
    "backend_platform_client_unsupported_surface",
    "backend_platform_client_hash_manifest_present",
    "backend_platform_client_hash_mismatch",
    "backend_platform_client_receipt_hash_refresh_current",
    "backend_platform_client_receipt_hash_refresh_stale",
    "backend_platform_client_receipt_hash_refresh_missing",
];
const REALTIME_APP_DATABASE_PACKAGE_ID: &str = "instantdb/react";
const REALTIME_APP_DATABASE_OFFICIAL_NAME: &str = "Realtime App Database";
const REALTIME_APP_DATABASE_UPSTREAM_PACKAGE: &str = "@instantdb/react";
const REALTIME_APP_DATABASE_UPSTREAM_VERSION: &str = "0.0.0";
const REALTIME_APP_DATABASE_SOURCE_MIRROR: &str = "G:/WWW/inspirations/instantdb";
#[cfg(test)]
#[allow(dead_code)]
const REALTIME_APP_DATABASE_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";
const REALTIME_APP_DATABASE_PACKAGE_RECEIPT_PATH: &str =
    "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json";
const REALTIME_APP_DATABASE_STATUS_VOCABULARY: [&str; 5] = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
];
const REALTIME_APP_DATABASE_METRICS: [&str; 13] = [
    "realtime_app_database_package_present",
    "realtime_app_database_receipt_present",
    "realtime_app_database_receipt_stale",
    "realtime_app_database_missing_receipt",
    "realtime_app_database_blocked_surface",
    "realtime_app_database_unsupported_surface",
    "realtime_app_database_hash_manifest_present",
    "realtime_app_database_hash_mismatch",
    "realtime_app_database_receipt_hash_refresh_current",
    "realtime_app_database_receipt_hash_refresh_stale",
    "realtime_app_database_receipt_hash_refresh_missing",
    "realtime_app_database_dx_style_compatibility_present",
    "realtime_app_database_dx_style_compatibility_missing",
];
const PAYMENTS_PACKAGE_ID: &str = "payments/stripe-js";
const PAYMENTS_OFFICIAL_NAME: &str = "Payments";
const PAYMENTS_UPSTREAM_PACKAGE: &str = "@stripe/stripe-js";
const PAYMENTS_UPSTREAM_VERSION: &str = "9.6.0";
const PAYMENTS_SOURCE_MIRROR: &str = "G:/WWW/inspirations/stripe-js";
#[cfg(test)]
#[allow(dead_code)]
const PAYMENTS_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";
const PAYMENTS_PACKAGE_RECEIPT_PATH: &str =
    "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json";
const PAYMENTS_STATUS_VOCABULARY: [&str; 5] = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
];
const PAYMENTS_METRICS: [&str; 13] = [
    "payments_package_present",
    "payments_receipt_present",
    "payments_receipt_stale",
    "payments_missing_receipt",
    "payments_blocked_surface",
    "payments_unsupported_surface",
    "payments_hash_manifest_present",
    "payments_hash_mismatch",
    "payments_receipt_hash_refresh_current",
    "payments_receipt_hash_refresh_stale",
    "payments_receipt_hash_refresh_missing",
    "payments_dx_style_compatibility_present",
    "payments_dx_style_compatibility_missing",
];
const MOTION_ANIMATION_PACKAGE_ID: &str = "animation/motion";
const MOTION_ANIMATION_OFFICIAL_NAME: &str = "Motion & Animation";
const MOTION_ANIMATION_UPSTREAM_PACKAGE: &str = "motion";
const MOTION_ANIMATION_UPSTREAM_VERSION: &str = "12.38.0";
const MOTION_ANIMATION_SOURCE_MIRROR: &str = "G:/WWW/inspirations/motion";
#[cfg(test)]
#[allow(dead_code)]
const MOTION_ANIMATION_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";
const MOTION_ANIMATION_PACKAGE_RECEIPT_PATH: &str =
    "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json";
const MOTION_ANIMATION_RECEIPT_HASH_REFRESH_ZED_VISIBILITY: &str =
    "motion-animation:receipt-hash-refresh";
const MOTION_ANIMATION_STATUS_VOCABULARY: [&str; 5] = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
];
const MOTION_ANIMATION_METRICS: [&str; 11] = [
    "motion_animation_package_present",
    "motion_animation_receipt_present",
    "motion_animation_receipt_stale",
    "motion_animation_missing_receipt",
    "motion_animation_blocked_surface",
    "motion_animation_unsupported_surface",
    "motion_animation_hash_manifest_present",
    "motion_animation_hash_mismatch",
    "motion_animation_receipt_hash_refresh_current",
    "motion_animation_receipt_hash_refresh_stale",
    "motion_animation_receipt_hash_refresh_missing",
];
const UI_COMPONENTS_PACKAGE_ID: &str = "shadcn/ui/button";
const UI_COMPONENTS_OFFICIAL_NAME: &str = "UI Components";
const UI_COMPONENTS_UPSTREAM_PACKAGE: &str = "shadcn-ui";
const UI_COMPONENTS_UPSTREAM_VERSION: &str = "0.0.1";
const UI_COMPONENTS_SOURCE_MIRROR: &str =
    "G:/WWW/inspirations/shadcn-ui; G:/WWW/inspirations/radix-primitives";
#[cfg(test)]
#[allow(dead_code)]
const UI_COMPONENTS_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";
const UI_COMPONENTS_PACKAGE_RECEIPT_PATH: &str =
    "examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json";
const UI_COMPONENTS_STATUS_VOCABULARY: [&str; 5] = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
];
const UI_COMPONENTS_METRICS: [&str; 11] = [
    "ui_components_package_present",
    "ui_components_receipt_present",
    "ui_components_receipt_stale",
    "ui_components_missing_receipt",
    "ui_components_blocked_surface",
    "ui_components_unsupported_surface",
    "ui_components_hash_manifest_present",
    "ui_components_hash_mismatch",
    "ui_components_receipt_hash_refresh_current",
    "ui_components_receipt_hash_refresh_stale",
    "ui_components_receipt_hash_refresh_missing",
];
const INTERNATIONALIZATION_PACKAGE_ID: &str = "i18n/next-intl";
const INTERNATIONALIZATION_OFFICIAL_NAME: &str = "Internationalization";
const INTERNATIONALIZATION_UPSTREAM_PACKAGE: &str = "next-intl";
const INTERNATIONALIZATION_UPSTREAM_VERSION: &str = "4.12.0";
const INTERNATIONALIZATION_SOURCE_MIRROR: &str = "G:/WWW/inspirations/next-intl";
#[cfg(test)]
#[allow(dead_code)]
const INTERNATIONALIZATION_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";
const INTERNATIONALIZATION_PACKAGE_RECEIPT_PATH: &str =
    "examples/template/.dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json";
const INTERNATIONALIZATION_STATUS_VOCABULARY: [&str; 5] = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
];
const INTERNATIONALIZATION_METRICS: [&str; 13] = [
    "internationalization_package_present",
    "internationalization_receipt_present",
    "internationalization_receipt_stale",
    "internationalization_missing_receipt",
    "internationalization_blocked_surface",
    "internationalization_unsupported_surface",
    "internationalization_hash_manifest_present",
    "internationalization_hash_mismatch",
    "internationalization_receipt_hash_refresh_current",
    "internationalization_receipt_hash_refresh_stale",
    "internationalization_receipt_hash_refresh_missing",
    "internationalization_dx_style_compatibility_present",
    "internationalization_dx_style_compatibility_missing",
];
const DOCUMENTATION_SYSTEM_PACKAGE_ID: &str = "content/fumadocs-next";
const DOCUMENTATION_SYSTEM_OFFICIAL_NAME: &str = "Documentation System";
const DOCUMENTATION_SYSTEM_UPSTREAM_PACKAGE: &str = "fumadocs";
const DOCUMENTATION_SYSTEM_UPSTREAM_VERSION: &str = "16.8.12";
const DOCUMENTATION_SYSTEM_SOURCE_MIRROR: &str = "G:/WWW/inspirations/fumadocs";
#[cfg(test)]
#[allow(dead_code)]
const DOCUMENTATION_SYSTEM_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";
const DOCUMENTATION_SYSTEM_PACKAGE_RECEIPT_PATH: &str =
    "examples/template/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json";
const DOCUMENTATION_SYSTEM_STATUS_VOCABULARY: [&str; 5] = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
];
const DOCUMENTATION_SYSTEM_METRICS: [&str; 13] = [
    "documentation_system_package_present",
    "documentation_system_receipt_present",
    "documentation_system_receipt_stale",
    "documentation_system_missing_receipt",
    "documentation_system_blocked_surface",
    "documentation_system_unsupported_surface",
    "documentation_system_hash_manifest_present",
    "documentation_system_hash_mismatch",
    "documentation_system_receipt_hash_refresh_current",
    "documentation_system_receipt_hash_refresh_stale",
    "documentation_system_receipt_hash_refresh_missing",
    "documentation_system_dx_style_compatibility_present",
    "documentation_system_dx_style_compatibility_missing",
];
const MARKDOWN_MDX_CONTENT_PACKAGE_ID: &str = "content/react-markdown";
const MARKDOWN_MDX_CONTENT_OFFICIAL_NAME: &str = "Markdown & MDX Content";
const MARKDOWN_MDX_CONTENT_UPSTREAM_PACKAGE: &str = "react-markdown; @mdx-js/mdx; @mdx-js/react";
const MARKDOWN_MDX_CONTENT_UPSTREAM_VERSION: &str =
    "react-markdown@10.1.0; @mdx-js/mdx@3.1.1; @mdx-js/react@3.1.1";
const MARKDOWN_MDX_CONTENT_SOURCE_MIRROR: &str =
    "G:/WWW/inspirations/react-markdown; G:/WWW/inspirations/mdx";
#[cfg(test)]
#[allow(dead_code)]
const MARKDOWN_MDX_CONTENT_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";
const MARKDOWN_MDX_CONTENT_PACKAGE_RECEIPT_PATH: &str =
    ".dx/forge/receipts/packages/content-react-markdown.json";
const MARKDOWN_MDX_CONTENT_STATUS_VOCABULARY: [&str; 5] = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
];
const MARKDOWN_MDX_CONTENT_METRICS: [&str; 15] = [
    "markdown_mdx_content_package_present",
    "markdown_mdx_content_receipt_present",
    "markdown_mdx_content_receipt_stale",
    "markdown_mdx_content_missing_receipt",
    "markdown_mdx_content_blocked_surface",
    "markdown_mdx_content_unsupported_surface",
    "markdown_mdx_content_hash_manifest_present",
    "markdown_mdx_content_hash_mismatch",
    "markdown_mdx_content_receipt_hash_refresh_current",
    "markdown_mdx_content_receipt_hash_refresh_stale",
    "markdown_mdx_content_receipt_hash_refresh_missing",
    "markdown_mdx_content_dx_style_compatibility_present",
    "markdown_mdx_content_dx_style_compatibility_missing",
    "markdown_mdx_content_materialized_source_present",
    "markdown_mdx_content_materialized_source_missing",
];
const AI_SDK_PACKAGE_ID: &str = "ai/vercel-ai";
const AI_SDK_OFFICIAL_NAME: &str = "AI SDK";
const AI_SDK_UPSTREAM_PACKAGE: &str = "ai";
const AI_SDK_UPSTREAM_VERSION: &str = "7.0.0-canary.146";
const AI_SDK_SOURCE_MIRROR: &str = "G:/WWW/inspirations/vercel-ai";
#[cfg(test)]
#[allow(dead_code)]
const AI_SDK_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";
const AI_SDK_PACKAGE_RECEIPT_PATH: &str =
    "examples/template/.dx/forge/receipts/2026-05-22-ai-vercel-ai-launch-assistant.json";
const AI_SDK_STATUS_VOCABULARY: [&str; 5] = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
];
const AI_SDK_METRICS: [&str; 13] = [
    "ai_sdk_package_present",
    "ai_sdk_receipt_present",
    "ai_sdk_receipt_stale",
    "ai_sdk_missing_receipt",
    "ai_sdk_blocked_surface",
    "ai_sdk_unsupported_surface",
    "ai_sdk_hash_manifest_present",
    "ai_sdk_hash_mismatch",
    "ai_sdk_receipt_hash_refresh_current",
    "ai_sdk_receipt_hash_refresh_stale",
    "ai_sdk_receipt_hash_refresh_missing",
    "ai_sdk_dx_style_compatibility_present",
    "ai_sdk_dx_style_compatibility_missing",
];
const THREE_SCENE_SYSTEM_PACKAGE_ID: &str = "3d/launch-scene";
const THREE_SCENE_SYSTEM_OFFICIAL_NAME: &str = "3D Scene System";
const THREE_SCENE_SYSTEM_UPSTREAM_PACKAGE: &str = "three + @react-three/fiber + @react-three/drei";
const THREE_SCENE_SYSTEM_UPSTREAM_VERSION: &str =
    "three 0.184.0; @react-three/fiber 9.6.1; @react-three/drei local mirror";
const THREE_SCENE_SYSTEM_SOURCE_MIRROR: &str =
    "G:/WWW/inspirations/three.js; G:/WWW/inspirations/react-three-fiber; G:/WWW/inspirations/drei";
#[cfg(test)]
#[allow(dead_code)]
const THREE_SCENE_SYSTEM_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";
const THREE_SCENE_SYSTEM_PACKAGE_RECEIPT_PATH: &str =
    ".dx/forge/receipts/3d-launch-scene-dashboard-workflow.json";
const THREE_SCENE_SYSTEM_STATUS_VOCABULARY: [&str; 5] = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
];
const THREE_SCENE_SYSTEM_METRICS: [&str; 12] = [
    "three_scene_system_receipt_present",
    "three_scene_system_receipt_stale",
    "three_scene_system_missing_receipt",
    "three_scene_system_blocked_surface",
    "three_scene_system_unsupported_surface",
    "three_scene_system_hash_manifest_present",
    "three_scene_system_hash_mismatch",
    "three_scene_system_receipt_hash_refresh_current",
    "three_scene_system_receipt_hash_refresh_stale",
    "three_scene_system_receipt_hash_refresh_missing",
    "three_scene_system_dx_style_compatibility_present",
    "three_scene_system_dx_style_compatibility_missing",
];
const WEBASSEMBLY_BRIDGE_PACKAGE_ID: &str = "wasm/bindgen";
const WEBASSEMBLY_BRIDGE_OFFICIAL_NAME: &str = "WebAssembly Bridge";
const WEBASSEMBLY_BRIDGE_UPSTREAM_PACKAGE: &str = "wasm-bindgen";
const WEBASSEMBLY_BRIDGE_UPSTREAM_VERSION: &str = "0.2.121";
const WEBASSEMBLY_BRIDGE_SOURCE_MIRROR: &str = "G:/WWW/inspirations/wasm-bindgen";
#[cfg(test)]
#[allow(dead_code)]
const WEBASSEMBLY_BRIDGE_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";
const WEBASSEMBLY_BRIDGE_PACKAGE_RECEIPT_PATH: &str =
    ".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json";
const WEBASSEMBLY_BRIDGE_STATUS_VOCABULARY: [&str; 5] = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
];
const WEBASSEMBLY_BRIDGE_METRICS: [&str; 13] = [
    "webassembly_bridge_package_present",
    "webassembly_bridge_receipt_present",
    "webassembly_bridge_receipt_stale",
    "webassembly_bridge_missing_receipt",
    "webassembly_bridge_blocked_surface",
    "webassembly_bridge_unsupported_surface",
    "webassembly_bridge_hash_manifest_present",
    "webassembly_bridge_hash_mismatch",
    "webassembly_bridge_receipt_hash_refresh_current",
    "webassembly_bridge_receipt_hash_refresh_stale",
    "webassembly_bridge_receipt_hash_refresh_missing",
    "webassembly_bridge_dx_style_compatibility_present",
    "webassembly_bridge_dx_style_compatibility_missing",
];
const AUTOMATION_CONNECTORS_PACKAGE_ID: &str = "automations/n8n";
const AUTOMATION_CONNECTORS_OFFICIAL_NAME: &str = "Automation Connectors";
const AUTOMATION_CONNECTORS_UPSTREAM_PACKAGE: &str = "n8n-nodes-base";
const AUTOMATION_CONNECTORS_UPSTREAM_VERSION: &str = "2.22.0";
const AUTOMATION_CONNECTORS_SOURCE_MIRROR: &str = "G:/WWW/inspirations/n8n/packages/nodes-base";
#[cfg(test)]
#[allow(dead_code)]
const AUTOMATION_CONNECTORS_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";
const AUTOMATION_CONNECTORS_PACKAGE_RECEIPT_PATH: &str =
    "examples/template/.dx/forge/receipts/2026-05-22-automation-connectors-launch-workflow.json";
const AUTOMATION_CONNECTORS_STATUS_VOCABULARY: [&str; 5] = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
];
const AUTOMATION_CONNECTORS_REQUIRED_UPSTREAM_FILES: &[&str] = &[
    "packages/nodes-base/package.json",
    "packages/nodes-base/nodes/ManualTrigger/ManualTrigger.node.ts",
    "packages/nodes-base/nodes/Slack/Slack.node.ts",
    "packages/nodes-base/nodes/Slack/V2/SlackV2.node.ts",
    "packages/nodes-base/nodes/Webhook/Webhook.node.ts",
    "packages/nodes-base/nodes/Notion/Notion.node.ts",
    "packages/nodes-base/credentials/SlackApi.credentials.ts",
    "packages/nodes-base/credentials/SlackOAuth2Api.credentials.ts",
    "packages/nodes-base/credentials/NotionApi.credentials.ts",
];
const AUTOMATION_CONNECTORS_REQUIRED_UPSTREAM_PUBLIC_APIS: &[&str] = &[
    "VersionedNodeType",
    "INodeType",
    "INodeTypeDescription",
    "ITriggerFunctions",
    "IExecuteFunctions",
    "IWebhookFunctions",
    "ICredentialType",
    "IAuthenticateGeneric",
    "ICredentialTestRequest",
];
const AUTOMATION_CONNECTORS_METRICS: [&str; 15] = [
    "automation_connectors_package_present",
    "automation_connectors_receipt_present",
    "automation_connectors_receipt_stale",
    "automation_connectors_missing_receipt",
    "automation_connectors_blocked_surface",
    "automation_connectors_unsupported_surface",
    "automation_connectors_hash_manifest_present",
    "automation_connectors_hash_mismatch",
    "automation_connectors_dx_style_compatibility_present",
    "automation_connectors_dx_style_compatibility_missing",
    "automation_connectors_upstream_runtime_boundary_present",
    "automation_connectors_upstream_runtime_boundary_missing",
    "automation_connectors_receipt_hash_refresh_current",
    "automation_connectors_receipt_hash_refresh_stale",
    "automation_connectors_receipt_hash_refresh_missing",
];

/// DX-WWW state for the latest dx-check receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckLatestPanelReport {
    /// DX-WWW wrapper schema.
    pub schema_version: String,
    /// Whether the receipt is ready to render.
    pub status: DxCheckLatestPanelStatus,
    /// Path DX-WWW attempted to read.
    pub receipt_path: PathBuf,
    /// Source system for this panel state.
    pub source: String,
    /// Compact panel summary for quick UI cards and command palettes.
    pub summary: DxCheckPanelSummary,
    /// Render-ready model for Studio/Web Preview and future GPUI panels.
    pub view_model: DxCheckPanelViewModel,
    /// The Zed-facing panel payload when a valid receipt exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zed: Option<DxCheckZedPanel>,
    /// Readiness gate state copied from the latest dx-check receipt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_gate_status: Option<Value>,
    /// Replay commands for Readiness proof gates.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub readiness_replay_commands: Vec<String>,
    /// Parse/read error when the receipt is not usable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    /// Clear remediation for the next run.
    pub next_action: String,
}

/// Status for the latest dx-check receipt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DxCheckLatestPanelStatus {
    /// The latest receipt was found and contains a supported Zed panel.
    Ready,
    /// No latest receipt exists yet.
    Missing,
    /// A receipt exists but is not valid for this contract.
    Malformed,
}

impl DxCheckLatestPanelStatus {
    /// Return the stable lowercase status string.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Missing => "missing",
            Self::Malformed => "malformed",
        }
    }
}

/// Compact panel summary for DX-WWW and editor integrations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DxCheckPanelSummary {
    /// Score value from the Zed panel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_value: Option<u16>,
    /// Maximum score from the Zed panel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_max: Option<u16>,
    /// Normalized 0-100 score percentage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_percent: Option<u8>,
    /// Whether the score includes skipped or estimated buckets.
    pub score_estimated: bool,
    /// Panel status such as ready, warning, or blocked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub panel_status: Option<String>,
    /// Bucket count exposed by the panel.
    pub bucket_count: usize,
    /// Blocker count exposed by the panel.
    pub blocker_count: u32,
    /// Warning count exposed by the panel.
    pub warning_count: u32,
    /// Quick-fix count exposed by the panel.
    pub quick_fix_count: usize,
    /// Last run time from the dx-check receipt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_unix_ms: Option<u128>,
    /// Command DX-WWW can show for refresh actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_command: Option<String>,
    /// Command DX-WWW can show for score detail actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_command: Option<String>,
}

/// Render-ready dx-check panel state for DX-WWW and editor UI consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckPanelViewModel {
    /// View-model schema.
    pub schema_version: String,
    /// Scoring weight profile used by the receipt.
    pub weight_profile: String,
    /// Scoring config discovery state used by the receipt.
    pub scoring_config: DxCheckScoringConfigReport,
    /// Stable state for panel routing.
    pub status: String,
    /// Short panel title.
    pub title: String,
    /// One-line panel explanation.
    pub subtitle: String,
    /// Stable receipt path displayed in the panel.
    pub receipt_path: String,
    /// Score meter when a valid receipt exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_meter: Option<DxCheckPanelScoreMeter>,
    /// Last run time in Unix milliseconds when known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_unix_ms: Option<u128>,
    /// Human-readable last-run state without locale formatting.
    pub last_run_label: String,
    /// Bucket rows for the score breakdown.
    pub bucket_rows: Vec<DxCheckPanelBucketRow>,
    /// Package-lane rows rendered from Forge receipts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub package_lane_rows: Vec<DxCheckPanelPackageLaneRow>,
    /// Style evidence rows rendered from lower dx-check style metrics.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style_evidence_rows: Vec<DxCheckPanelStyleEvidenceRow>,
    /// Readiness gate state copied from the latest dx-check receipt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_gate_status: Option<Value>,
    /// Replay commands for Readiness proof gates.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub readiness_replay_commands: Vec<String>,
    /// Blocking findings for the panel.
    pub blocker_rows: Vec<DxCheckPanelFindingRow>,
    /// Warning findings for the panel.
    pub warning_rows: Vec<DxCheckPanelFindingRow>,
    /// Quick fix rows for the panel.
    pub quick_fix_rows: Vec<DxCheckPanelQuickFixRow>,
    /// Primary action the UI can offer.
    pub primary_action: DxCheckPanelAction,
    /// Secondary action the UI can offer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_action: Option<DxCheckPanelAction>,
    /// Empty-state copy for missing or malformed receipts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_state: Option<String>,
}

/// Score meter rendered in the dx-check panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckPanelScoreMeter {
    /// Score value.
    pub value: u16,
    /// Maximum score.
    pub max: u16,
    /// Normalized 0-100 percentage.
    pub percent: u8,
    /// Whether the score includes estimated or skipped buckets.
    pub estimated: bool,
    /// Visual tone hint for UI renderers.
    pub tone: String,
    /// Compact label for text renderers.
    pub label: String,
}

/// Score bucket row rendered in the dx-check panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckPanelBucketRow {
    /// Stable bucket id.
    pub id: String,
    /// Bucket title.
    pub title: String,
    /// Bucket weight in the active scoring profile.
    pub weight: u16,
    /// Bucket score.
    pub score: u16,
    /// Bucket max score.
    pub max_score: u16,
    /// Whether the bucket score is estimated.
    pub estimated: bool,
    /// Bucket status.
    pub status: String,
    /// Bucket summary.
    pub summary: String,
}

/// Package-lane row rendered in the dx-check panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckPanelPackageLaneRow {
    /// Stable DX package id.
    pub package_id: String,
    /// Official DX package name.
    pub official_package_name: String,
    /// Upstream package name kept as provenance.
    pub upstream_package: String,
    /// Upstream version kept as provenance.
    pub upstream_version: String,
    /// Local source mirror used for provenance inspection.
    pub source_mirror: String,
    /// Package-lane status.
    pub status: String,
    /// Receipt-specific status.
    pub receipt_status: String,
    /// Stable package receipt path.
    pub package_receipt_path: String,
    /// Supported package-lane status vocabulary.
    pub status_vocabulary: Vec<String>,
    /// Selected package surfaces surfaced to Studio/Zed.
    pub selected_surfaces: Vec<DxCheckPanelPackageLaneSurfaceRow>,
    /// Optional package-owned helper status for keeping receipt hashes fresh.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_hash_refresh: Option<DxCheckPanelPackageLaneHashRefreshRow>,
    /// Machine metrics mirrored from dx-check package visibility.
    pub metrics: Vec<DxCheckPanelPackageLaneMetric>,
    /// Runtime limits that keep the row honest.
    pub runtime_limitations: Vec<String>,
    /// Suggested next action for the package lane.
    pub next_action: String,
}

/// Package-owned helper status rendered inside a package-lane row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckPanelPackageLaneHashRefreshRow {
    /// Helper schema.
    pub schema: String,
    /// Freshness status such as current, stale, missing, or blocked.
    pub status: String,
    /// Source helper script path.
    pub helper_path: String,
    /// Read-only freshness check command.
    pub check_command: String,
    /// Write command that refreshes reviewed receipt hashes.
    pub write_command: String,
    /// JSON-emitting check command for Studio/Zed.
    pub json_check_command: String,
    /// Receipt maintained by this helper.
    pub receipt_path: String,
    /// Hash algorithm used by the helper.
    pub hash_algorithm: String,
    /// Files tracked by the helper.
    #[serde(default)]
    pub tracked_files: Vec<String>,
    /// Package-owned source-guard runbook fixture tracked by the helper.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_guard_runbook_fixture: Option<String>,
    /// Generated preview-manifest materializer tracked by the helper.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_manifest_materializer: Option<String>,
    /// Studio manifest source tracked by the helper.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio_manifest_source: Option<String>,
    /// Lower dx-check source tracked by the helper.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_dx_check_source: Option<String>,
    /// Shared check-panel source tracked by the helper.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_panel_source: Option<String>,
    /// Number of files tracked by the helper.
    pub tracked_file_count: u64,
    /// Number of stale tracked files.
    pub stale_file_count: u64,
    /// Number of missing tracked files.
    pub missing_file_count: u64,
    /// Files whose source hashes are current.
    #[serde(default)]
    pub current_files: Vec<String>,
    /// Files whose source hashes are stale.
    #[serde(default)]
    pub stale_files: Vec<String>,
    /// Files missing from the local source tree.
    #[serde(default)]
    pub missing_files: Vec<String>,
    /// Files whose receipt/package/read-model mirrors are stale.
    #[serde(default)]
    pub stale_mirror_files: Vec<String>,
    /// Files whose receipt/package/read-model mirrors are missing.
    #[serde(default)]
    pub missing_mirror_files: Vec<String>,
    /// Number of stale or missing receipt/package/read-model mirror entries.
    #[serde(default)]
    pub mirror_problem_count: u64,
    /// Whether the helper runs package runtime code.
    pub runtime_execution: bool,
    /// Whether the helper reads app secrets.
    pub secret_access: bool,
    /// Zed/DX Studio receipt helper surface id.
    pub zed_visibility: String,
    /// Runtime limits that keep helper output honest.
    pub runtime_limitations: Vec<String>,
}

/// Selected package surface row rendered in package-lane status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckPanelPackageLaneSurfaceRow {
    /// Stable selected surface id.
    pub surface_id: String,
    /// Surface status.
    pub status: String,
    /// Files owned by the selected surface.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    /// Source markers for Zed/DX Studio selection.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_markers: Vec<String>,
    /// Optional surface receipt path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_path: Option<String>,
    /// Optional reason for blocked or unsupported requested surfaces.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Optional app-owned boundary that must be resolved before claiming proof.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_owned_boundary: Option<String>,
}

/// Package-lane metric rendered in the dx-check panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckPanelPackageLaneMetric {
    /// Metric name from dx-check.
    pub name: String,
    /// Metric value for the current package row.
    pub value: u64,
}

/// Style evidence row rendered in the dx-check panel for Forge/Zed consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckPanelStyleEvidenceRow {
    /// Stable row id.
    pub row_id: String,
    /// Short UI title.
    pub title: String,
    /// Evidence status such as present, missing-receipt, missing-contract, unsupported-schema, or overclaimed.
    pub status: String,
    /// Style receipt path that fed dx-check.
    pub receipt_path: String,
    /// Source fixture backing this evidence row.
    pub fixture_path: String,
    /// Whether `.dx/receipts/style/check.json` was present when dx-check ran.
    pub receipt_present: bool,
    /// Whether the browser compatibility contract was present in the style receipt.
    pub contract_present: bool,
    /// Whether the contract schema matched the current reader.
    pub schema_supported: bool,
    /// Number of classes covered by the browser compatibility canary.
    pub class_count: u64,
    /// Number of selector-level classes covered by the browser compatibility canary.
    pub selector_class_count: u64,
    /// Receipt-backed selector class examples for Zed/DX Studio display.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selector_class_examples: Vec<String>,
    /// Count of receipt-proven Tailwind state-alias classes summarized by lower dx-check.
    pub tailwind_parity_state_alias_supported_class_count: u64,
    /// Receipt-backed state-alias class examples for Zed/DX Studio display.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tailwind_parity_supported_state_alias_examples: Vec<String>,
    /// Number of package-owned style rows summarized for Forge/Zed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_ownership_package_count: Option<u64>,
    /// Number of generated classes attributed to package-owned style rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_ownership_generated_class_count: Option<u64>,
    /// Number of package-owned classes without generated CSS proof.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_ownership_unsupported_class_count: Option<u64>,
    /// Package ids represented by package-owned style evidence.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub package_ownership_package_ids: Vec<String>,
    /// Package-owned unsupported class examples for diagnostics.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub package_ownership_unsupported_class_examples: Vec<String>,
    /// Rule metadata visual property buckets for Zed/DX Studio editing.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rule_metadata_visual_properties: Vec<String>,
    /// Rule metadata source files for class-to-component mapping.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rule_metadata_source_files: Vec<String>,
    /// Rule metadata theme token references for visual token editing.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rule_metadata_token_references: Vec<String>,
    /// Number of rule metadata rows editable by Zed/DX Studio.
    #[serde(default)]
    pub rule_metadata_editable_class_count: u64,
    /// Whether every rule metadata class in this row is Zed/DX Studio editable.
    #[serde(default)]
    pub rule_metadata_zed_studio_editable: bool,
    /// Whether dx-style claims full Autoprefixer parity.
    pub full_autoprefixer_parity: bool,
    /// Whether dx-style claims full Tailwind/PostCSS generated-output parity.
    pub full_tailwind_postcss_output_parity: bool,
    /// Machine metrics mirrored from the lower dx-check style section.
    pub metrics: Vec<DxCheckPanelStyleEvidenceMetric>,
    /// Editor surface id used by Zed/DX Studio.
    pub zed_visibility: String,
    /// Runtime limits that keep the row honest.
    pub runtime_limitations: Vec<String>,
    /// Suggested next action for the dx-style browser compatibility lane.
    pub next_action: String,
}

/// Style evidence metric rendered in the dx-check panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckPanelStyleEvidenceMetric {
    /// Metric name from dx-check.
    pub name: String,
    /// Metric value for the current style evidence row.
    pub value: u64,
}

/// Finding row rendered in the dx-check panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckPanelFindingRow {
    /// Finding severity.
    pub severity: String,
    /// Stable finding code.
    pub code: String,
    /// Finding message.
    pub message: String,
    /// Optional evidence path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_path: Option<String>,
    /// Suggested next action.
    pub next_action: String,
}

/// Quick-fix row rendered in the dx-check panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckPanelQuickFixRow {
    /// Stable quick-fix id.
    pub id: String,
    /// Short UI label.
    pub label: String,
    /// Suggested next action.
    pub next_action: String,
    /// Operator risk classification for the quick-fix action.
    #[serde(default = "default_quick_fix_risk_level")]
    pub risk_level: String,
    /// Whether running the action requires explicit user approval.
    #[serde(default)]
    pub requires_user_approval: bool,
    /// Whether the action is expected to write dx-check receipts.
    #[serde(default)]
    pub writes_receipts: bool,
    /// Optional command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
}

/// UI action rendered in the dx-check panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckPanelAction {
    /// Stable action id.
    pub id: String,
    /// Button or menu label.
    pub label: String,
    /// CLI command the user can run.
    pub command: String,
}

/// Zed-facing panel payload emitted by dx-check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckZedPanel {
    /// Panel kind for editor routing.
    pub panel_kind: String,
    /// Zed panel schema.
    pub schema_version: String,
    /// Source system.
    pub source: String,
    /// Scoring weight profile used by this panel.
    #[serde(default = "default_weight_profile")]
    pub weight_profile: String,
    /// Scoring config discovery state used by this panel.
    #[serde(default)]
    pub scoring_config: DxCheckScoringConfigReport,
    /// Current score.
    pub score_value: u16,
    /// Maximum score.
    pub score_max: u16,
    /// Score percentage.
    pub score_percent: u8,
    /// Whether the score is estimated.
    pub score_estimated: bool,
    /// Panel status.
    pub status: String,
    /// Generation time in Unix milliseconds.
    pub generated_at_unix_ms: u128,
    /// Number of score buckets.
    pub bucket_count: usize,
    /// Number of blocking findings.
    pub blocker_count: u32,
    /// Number of warning findings.
    pub warning_count: u32,
    /// Number of quick fixes.
    pub quick_fix_count: usize,
    /// Receipt path emitted by dx-check.
    pub receipt_path: String,
    /// Refresh command for UI actions.
    pub refresh_command: String,
    /// Detail command for UI actions.
    pub detail_command: String,
    /// Blocking findings.
    #[serde(default)]
    pub blockers: Vec<DxCheckZedFinding>,
    /// Warning findings.
    #[serde(default)]
    pub warnings: Vec<DxCheckZedFinding>,
    /// Suggested next actions.
    #[serde(default)]
    pub quick_fixes: Vec<DxCheckZedQuickFix>,
    /// Score bucket sections.
    #[serde(default)]
    pub sections: Vec<DxCheckZedSection>,
}

/// Finding rendered in the dx-check panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckZedFinding {
    /// Finding severity.
    pub severity: String,
    /// Stable finding code.
    pub code: String,
    /// Human-readable finding message.
    pub message: String,
    /// Optional evidence path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_path: Option<String>,
    /// Suggested next action.
    pub next_action: String,
}

/// Quick-fix command rendered in the dx-check panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckZedQuickFix {
    /// Stable quick-fix id.
    pub id: String,
    /// Short UI label.
    pub label: String,
    /// Suggested next action.
    pub next_action: String,
    /// Operator risk classification for the quick-fix action.
    #[serde(default = "default_quick_fix_risk_level")]
    pub risk_level: String,
    /// Whether running the action requires explicit user approval.
    #[serde(default)]
    pub requires_user_approval: bool,
    /// Whether the action is expected to write dx-check receipts.
    #[serde(default)]
    pub writes_receipts: bool,
    /// Optional command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
}

/// Score bucket rendered in the dx-check panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckZedSection {
    /// Stable section id.
    pub id: String,
    /// Human-readable title.
    pub title: String,
    /// Section weight in the active scoring profile.
    #[serde(default = "default_bucket_weight")]
    pub weight: u16,
    /// Section score.
    pub score: u16,
    /// Section max score.
    pub max_score: u16,
    /// Whether the section score is estimated.
    pub estimated: bool,
    /// Section status.
    pub status: String,
    /// Section summary.
    pub summary: String,
}

/// dx-check scoring config discovery state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckScoringConfigReport {
    /// Scoring config schema.
    pub schema_version: String,
    /// Active profile used by the receipt score.
    pub active_profile: String,
    /// Config status such as default, detected_not_applied, or invalid_not_applied.
    pub status: String,
    /// Whether a config file was present.
    pub config_present: bool,
    /// Config paths checked by the CLI.
    #[serde(default)]
    pub config_paths_checked: Vec<String>,
    /// Config path used when present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_path: Option<String>,
    /// Profile declared by a detected config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_profile: Option<String>,
    /// Active bucket weights used for the current score.
    #[serde(default)]
    pub active_bucket_weights: Vec<DxCheckPanelBucketWeight>,
    /// Configured bucket weights when a future config is present.
    #[serde(default)]
    pub configured_bucket_weights: Vec<DxCheckPanelBucketWeight>,
    /// Configured weight total when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_total_weight: Option<u32>,
    /// Whether the detected config affected scoring.
    pub applies_to_score: bool,
    /// Why a detected config did not apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_reason: Option<String>,
    /// Suggested next action.
    pub next_action: String,
}

impl Default for DxCheckScoringConfigReport {
    fn default() -> Self {
        default_scoring_config_report()
    }
}

/// Bucket weight row in the scoring config report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckPanelBucketWeight {
    /// Stable bucket id.
    pub id: String,
    /// Bucket label.
    pub label: String,
    /// Bucket weight.
    pub weight: u16,
}

#[derive(Debug, Deserialize)]
struct DxCheckLatestReceipt {
    #[serde(default)]
    schema_version: Option<String>,
    #[serde(default)]
    release_ready: Option<bool>,
    #[serde(default)]
    fastest_world_claim: Option<bool>,
    zed: DxCheckZedPanel,
    #[serde(default)]
    sections: Vec<DxCheckReceiptSection>,
    #[serde(default)]
    readiness_gate_status: Option<Value>,
    #[serde(default)]
    readiness_replay_commands: Vec<String>,
    #[serde(default)]
    replay_commands: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct DxCheckReceiptSection {
    name: String,
    #[serde(default)]
    metrics: Vec<DxCheckReceiptMetric>,
}

#[derive(Debug, Deserialize)]
struct DxCheckReceiptMetric {
    name: String,
    value: u64,
}

/// Return the stable latest dx-check receipt path for a project root.
pub fn dx_check_latest_receipt_path(root: impl AsRef<Path>) -> PathBuf {
    root.as_ref().join(DX_CHECK_LATEST_RECEIPT_PATH)
}

/// Read the latest dx-check receipt as a compact DX-WWW panel state.
pub fn read_dx_check_latest_panel(root: impl AsRef<Path>) -> DxCheckLatestPanelReport {
    let root = root.as_ref();
    let receipt_path = dx_check_latest_receipt_path(root);
    let body = match fs::read_to_string(&receipt_path) {
        Ok(body) => body,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return missing_report(receipt_path);
        }
        Err(error) => {
            return malformed_report(
                receipt_path,
                format!("Failed to read dx-check receipt: {error}"),
            );
        }
    };

    let receipt = match serde_json::from_str::<DxCheckLatestReceipt>(&body) {
        Ok(receipt) => receipt,
        Err(error) => {
            return malformed_report(
                receipt_path,
                format!("Failed to parse dx-check receipt: {error}"),
            );
        }
    };

    if !is_supported_zed_panel_schema(&receipt.zed.schema_version) {
        return malformed_report(
            receipt_path,
            format!(
                "Unsupported dx-check Zed panel schema: {}",
                receipt.zed.schema_version
            ),
        );
    }

    if receipt.zed.panel_kind != "project-health" {
        return malformed_report(
            receipt_path,
            format!(
                "Unsupported dx-check panel kind: {}",
                receipt.zed.panel_kind
            ),
        );
    }

    if receipt.schema_version.as_deref() == Some("dx.check.latest.v1")
        && !latest_receipt_readiness_gate_metadata_current(&receipt)
    {
        let stale_reasons = latest_receipt_readiness_gate_stale_reasons(&receipt);
        return malformed_report(
            receipt_path,
            format!(
                "Current dx-check latest receipt is missing safe readiness release-gate metadata or replay commands: {}.",
                stale_reasons.join(", ")
            ),
        );
    }

    ready_report(
        root,
        receipt_path,
        normalize_zed_panel(receipt.zed),
        receipt.sections,
        receipt.readiness_gate_status,
        receipt.readiness_replay_commands,
    )
}

fn latest_receipt_readiness_gate_metadata_current(receipt: &DxCheckLatestReceipt) -> bool {
    latest_receipt_readiness_gate_stale_reasons(receipt).is_empty()
}

fn latest_receipt_readiness_gate_stale_reasons(receipt: &DxCheckLatestReceipt) -> Vec<String> {
    let mut stale_reasons = Vec::new();
    let gate = receipt.readiness_gate_status.as_ref();

    if !latest_receipt_static_advisory_claim_current(receipt)
        && !latest_receipt_local_proof_claim_current(receipt)
    {
        if receipt.release_ready != Some(false) {
            stale_reasons.push("release-ready-claim-unsafe-for-static-advisory".to_string());
        }
        if receipt.fastest_world_claim != Some(false) {
            stale_reasons.push("fastest-world-claim-unsafe-for-static-advisory".to_string());
        }
        if gate_bool(gate, "release_ready") != Some(false) {
            stale_reasons.push("gate-release-ready-claim-unsafe-for-static-advisory".to_string());
        }
        if gate_bool(gate, "fastest_world_claim") != Some(false) {
            stale_reasons.push("gate-fastest-world-claim-unsafe-for-static-advisory".to_string());
        }
        if gate_str(gate, "score_kind") != Some("static-advisory-not-release-proof") {
            stale_reasons.push("score-kind-unsafe-for-static-advisory".to_string());
        }
        if gate_bool(gate, "verified_from_replay_receipts") != Some(false) {
            stale_reasons
                .push("verified-from-replay-receipts-unsafe-for-static-advisory".to_string());
        }
        if !static_advisory_receipt_freshness_safe(gate_str(gate, "receipt_freshness")) {
            stale_reasons.push("receipt-freshness-unsafe-for-static-advisory".to_string());
        }
    }
    if !receipt
        .readiness_replay_commands
        .iter()
        .any(|command| command == "dx www readiness --json --full")
    {
        stale_reasons.push("missing-replay-command: dx www readiness --json --full".to_string());
    }
    if !receipt
        .replay_commands
        .iter()
        .any(|command| command == "dx www agent-context --json --full")
    {
        stale_reasons
            .push("missing-replay-command: dx www agent-context --json --full".to_string());
    }
    if !receipt
        .replay_commands
        .iter()
        .any(|command| command == "dx www docs-doctor --json")
    {
        stale_reasons.push("missing-replay-command: dx www docs-doctor --json".to_string());
    }

    stale_reasons
}

fn latest_receipt_static_advisory_claim_current(receipt: &DxCheckLatestReceipt) -> bool {
    let gate = receipt.readiness_gate_status.as_ref();
    receipt.release_ready == Some(false)
        && receipt.fastest_world_claim == Some(false)
        && gate_bool(gate, "release_ready") == Some(false)
        && gate_bool(gate, "fastest_world_claim") == Some(false)
        && gate_str(gate, "score_kind") == Some("static-advisory-not-release-proof")
        && gate_bool(gate, "verified_from_replay_receipts") == Some(false)
        && static_advisory_receipt_freshness_safe(gate_str(gate, "receipt_freshness"))
}

fn latest_receipt_local_proof_claim_current(receipt: &DxCheckLatestReceipt) -> bool {
    let gate = receipt.readiness_gate_status.as_ref();
    receipt.release_ready == Some(true)
        && receipt.fastest_world_claim == Some(false)
        && gate_bool(gate, "release_ready") == Some(true)
        && gate_bool(gate, "relative_release_ready") == Some(true)
        && gate_bool(gate, "release_claim_allowed") == Some(true)
        && gate_bool(gate, "global_speed_claim_allowed") == Some(false)
        && gate_bool(gate, "fastest_world_claim") == Some(false)
        && gate_str(gate, "release_ready_scope") == Some("local-proof-backed-www-release")
        && gate_str(gate, "score_kind") == Some("relative-local-proof-backed-release-ready")
        && gate_bool(gate, "verified_from_replay_receipts") == Some(true)
        && gate_str(gate, "receipt_freshness") == Some("current")
}

fn gate_bool<'a>(gate: Option<&'a Value>, field: &str) -> Option<bool> {
    gate.and_then(|gate| gate.get(field))
        .and_then(Value::as_bool)
}

fn gate_str<'a>(gate: Option<&'a Value>, field: &str) -> Option<&'a str> {
    gate.and_then(|gate| gate.get(field))
        .and_then(Value::as_str)
}

fn static_advisory_receipt_freshness_safe(value: Option<&str>) -> bool {
    matches!(
        value,
        Some("not-evaluated-in-this-command" | "local-receipts-evaluated")
    )
}

fn is_supported_zed_panel_schema(schema_version: &str) -> bool {
    matches!(
        schema_version,
        DX_CHECK_ZED_PANEL_SCHEMA_VERSION | DX_CHECK_ZED_PANEL_LEGACY_SCHEMA_VERSION
    )
}

fn ready_report(
    root: &Path,
    receipt_path: PathBuf,
    zed: DxCheckZedPanel,
    receipt_sections: Vec<DxCheckReceiptSection>,
    readiness_gate_status: Option<Value>,
    readiness_replay_commands: Vec<String>,
) -> DxCheckLatestPanelReport {
    let summary = DxCheckPanelSummary {
        score_value: Some(zed.score_value),
        score_max: Some(zed.score_max),
        score_percent: Some(zed.score_percent),
        score_estimated: zed.score_estimated,
        panel_status: Some(zed.status.clone()),
        bucket_count: zed.bucket_count,
        blocker_count: zed.blocker_count,
        warning_count: zed.warning_count,
        quick_fix_count: zed.quick_fix_count,
        last_run_unix_ms: Some(zed.generated_at_unix_ms),
        refresh_command: Some(zed.refresh_command.clone()),
        detail_command: Some(zed.detail_command.clone()),
    };
    let view_model = ready_view_model(
        root,
        &zed,
        &receipt_sections,
        readiness_gate_status.clone(),
        readiness_replay_commands.clone(),
    );

    DxCheckLatestPanelReport {
        schema_version: DX_WWW_CHECK_PANEL_SCHEMA_VERSION.to_string(),
        status: DxCheckLatestPanelStatus::Ready,
        receipt_path,
        source: "dx-check".to_string(),
        summary,
        view_model,
        zed: Some(zed),
        readiness_gate_status,
        readiness_replay_commands,
        last_error: None,
        next_action: "Render the dx-check panel from the embedded Zed payload.".to_string(),
    }
}

fn missing_report(receipt_path: PathBuf) -> DxCheckLatestPanelReport {
    let next_action =
        "Run `dx check --json` from the project root to create check-latest.json.".to_string();

    DxCheckLatestPanelReport {
        schema_version: DX_WWW_CHECK_PANEL_SCHEMA_VERSION.to_string(),
        status: DxCheckLatestPanelStatus::Missing,
        receipt_path,
        source: "dx-check".to_string(),
        summary: DxCheckPanelSummary::default(),
        view_model: missing_view_model(),
        zed: None,
        readiness_gate_status: None,
        readiness_replay_commands: Vec::new(),
        last_error: None,
        next_action,
    }
}

fn malformed_report(receipt_path: PathBuf, error: impl Into<String>) -> DxCheckLatestPanelReport {
    let error = error.into();
    let next_action =
        "Re-run `dx check --json`; if this persists, update the DX-WWW receipt reader.".to_string();

    DxCheckLatestPanelReport {
        schema_version: DX_WWW_CHECK_PANEL_SCHEMA_VERSION.to_string(),
        status: DxCheckLatestPanelStatus::Malformed,
        receipt_path,
        source: "dx-check".to_string(),
        summary: DxCheckPanelSummary::default(),
        view_model: malformed_view_model(&error),
        zed: None,
        readiness_gate_status: None,
        readiness_replay_commands: Vec::new(),
        last_error: Some(error),
        next_action,
    }
}

fn ready_view_model(
    root: &Path,
    zed: &DxCheckZedPanel,
    receipt_sections: &[DxCheckReceiptSection],
    readiness_gate_status: Option<Value>,
    readiness_replay_commands: Vec<String>,
) -> DxCheckPanelViewModel {
    DxCheckPanelViewModel {
        schema_version: DX_WWW_CHECK_PANEL_VIEW_MODEL_SCHEMA_VERSION.to_string(),
        weight_profile: zed.weight_profile.clone(),
        scoring_config: zed.scoring_config.clone(),
        status: "ready".to_string(),
        title: "dx-check project health".to_string(),
        subtitle: "Latest receipt loaded from the root dx-check engine.".to_string(),
        receipt_path: zed.receipt_path.clone(),
        score_meter: Some(DxCheckPanelScoreMeter {
            value: zed.score_value,
            max: zed.score_max,
            percent: zed.score_percent,
            estimated: zed.score_estimated,
            tone: panel_tone(&zed.status, zed.blocker_count, zed.warning_count),
            label: format!(
                "{}/{} ({}%){}",
                zed.score_value,
                zed.score_max,
                zed.score_percent,
                if zed.score_estimated {
                    " estimated"
                } else {
                    ""
                }
            ),
        }),
        last_run_unix_ms: Some(zed.generated_at_unix_ms),
        last_run_label: format!("Last run Unix ms: {}", zed.generated_at_unix_ms),
        bucket_rows: zed.sections.iter().map(bucket_row).collect(),
        package_lane_rows: package_lane_rows(root),
        style_evidence_rows: dx_style_panel_rows(root, receipt_sections),
        readiness_gate_status,
        readiness_replay_commands,
        blocker_rows: zed.blockers.iter().map(finding_row).collect(),
        warning_rows: zed.warnings.iter().map(finding_row).collect(),
        quick_fix_rows: zed.quick_fixes.iter().map(quick_fix_row).collect(),
        primary_action: DxCheckPanelAction {
            id: "refresh-dx-check".to_string(),
            label: "Refresh dx-check".to_string(),
            command: zed.refresh_command.clone(),
        },
        secondary_action: Some(DxCheckPanelAction {
            id: "open-dx-check-score".to_string(),
            label: "Open score details".to_string(),
            command: zed.detail_command.clone(),
        }),
        empty_state: None,
    }
}

fn missing_view_model() -> DxCheckPanelViewModel {
    empty_view_model(
        "missing",
        "dx-check receipt missing",
        "No latest receipt is available yet.",
        "No dx-check receipt found at .dx/receipts/check/check-latest.json. Run the fast default check to create one.",
    )
}

fn malformed_view_model(error: &str) -> DxCheckPanelViewModel {
    empty_view_model(
        "malformed",
        "dx-check receipt malformed",
        "The latest receipt exists but cannot be rendered safely.",
        &format!(
            "The latest dx-check receipt could not be parsed or validated: {error}. Re-run dx-check before trusting this panel."
        ),
    )
}

fn empty_view_model(
    status: &'static str,
    title: &'static str,
    subtitle: &'static str,
    empty_state: &str,
) -> DxCheckPanelViewModel {
    DxCheckPanelViewModel {
        schema_version: DX_WWW_CHECK_PANEL_VIEW_MODEL_SCHEMA_VERSION.to_string(),
        weight_profile: DX_CHECK_WEIGHT_PROFILE.to_string(),
        scoring_config: default_scoring_config_report(),
        status: status.to_string(),
        title: title.to_string(),
        subtitle: subtitle.to_string(),
        receipt_path: DX_CHECK_LATEST_RECEIPT_PATH.to_string(),
        score_meter: None,
        last_run_unix_ms: None,
        last_run_label: "No dx-check run has been loaded.".to_string(),
        bucket_rows: Vec::new(),
        package_lane_rows: Vec::new(),
        style_evidence_rows: Vec::new(),
        readiness_gate_status: None,
        readiness_replay_commands: Vec::new(),
        blocker_rows: Vec::new(),
        warning_rows: Vec::new(),
        quick_fix_rows: Vec::new(),
        primary_action: DxCheckPanelAction {
            id: "run-dx-check".to_string(),
            label: "Run dx-check".to_string(),
            command: "dx check --json".to_string(),
        },
        secondary_action: None,
        empty_state: Some(empty_state.to_string()),
    }
}

fn bucket_row(section: &DxCheckZedSection) -> DxCheckPanelBucketRow {
    DxCheckPanelBucketRow {
        id: section.id.clone(),
        title: section.title.clone(),
        weight: section.weight,
        score: section.score,
        max_score: section.max_score,
        estimated: section.estimated,
        status: section.status.clone(),
        summary: section.summary.clone(),
    }
}
include!("panel_parts/style_evidence.rs");
include!("panel_parts/package_lanes.rs");
include!("panel_parts/package_metrics.rs");
include!("panel_parts/status_actions.rs");

#[cfg(test)]
mod tests {
    include!("panel_parts/tests_a.rs");
    include!("panel_parts/tests_b.rs");
}
