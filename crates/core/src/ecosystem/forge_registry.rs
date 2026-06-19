//! Source-owned Forge registry primitives and optional R2 publishing.

use anyhow::{Context, Result, bail};
use bytes::Bytes;
use chrono::Utc;
use object_store::{ObjectStore, aws::AmazonS3Builder, path::Path as ObjectPath};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

use super::forge_drizzle::{DRIZZLE_SQLITE_VERSION, drizzle_sqlite_templates};
use super::forge_fumadocs::{FUMADOCS_NEXT_VERSION, fumadocs_next_templates};
use super::forge_instantdb::{INSTANTDB_REACT_VERSION, instantdb_react_templates};
use super::forge_motion::{MOTION_VERSION, motion_templates};
use super::forge_n8n_automations::{N8N_AUTOMATIONS_VERSION, n8n_automations_templates};
use super::forge_next_intl::{NEXT_INTL_VERSION, next_intl_templates};
use super::forge_react_hook_form::{REACT_HOOK_FORM_VERSION, react_hook_form_templates};
use super::forge_react_markdown::{REACT_MARKDOWN_VERSION, react_markdown_templates};
use super::forge_reactive_store::{REACTIVE_STORE_VERSION, reactive_store_templates};
use super::forge_remote_health::{
    DxForgeRemoteObjectHeadHealthEvaluation, evaluate_r2_remote_object_head_receipt_health,
};
use super::forge_root_manifest::{
    DxForgeRootPackageExport, DxForgeRootPackageManifest, load_root_dx_package_manifest,
};
use super::forge_security::{
    DxForgeAction, DxForgeAdvisoryCoverageKind, DxForgeAdvisoryMetadata, DxForgeFileMap,
    DxForgeLicenseReviewMetadata, DxForgeProvenanceMetadata, DxForgeReceipt, DxPolicyDecision,
    DxSourceFile, DxSourceKind, DxSourcePackage, DxUpdateTraffic,
};
use super::forge_stripe_js::{STRIPE_JS_VERSION, stripe_js_templates};
use super::forge_supabase::supabase_client_templates;
use super::forge_tanstack_query::{TANSTACK_QUERY_VERSION, tanstack_query_templates};
use super::forge_three_scene::{
    THREE_SCENE_OFFICIAL_PACKAGE_NAME, THREE_SCENE_PACKAGE_ID, THREE_SCENE_VERSION,
    three_scene_templates,
};
use super::forge_trpc::{TRPC_NEXT_VERSION, trpc_next_templates};
use super::forge_vercel_ai::{VERCEL_AI_VERSION, vercel_ai_templates};
use super::forge_wasm_bindgen::{WASM_BINDGEN_VERSION, wasm_bindgen_templates};
use super::forge_zod::{ZOD_VALIDATION_VERSION, zod_validation_templates};
use super::forge_zustand::{ZUSTAND_VERSION, zustand_templates};

const DEFAULT_R2_PREFIX: &str = "dx-forge/registry/v1";
const REGISTRY_INDEX: &str = "index.json";
const SHADCN_BUTTON_VERSION: &str = "0.1.0";
const SHADCN_BADGE_VERSION: &str = "0.1.0";
const SHADCN_CARD_VERSION: &str = "0.1.0";
const SHADCN_ALERT_VERSION: &str = "0.1.0";
const SHADCN_AVATAR_VERSION: &str = "0.1.0";
const SHADCN_SKELETON_VERSION: &str = "0.1.0";
const SHADCN_LABEL_VERSION: &str = "0.1.0";
const SHADCN_SEPARATOR_VERSION: &str = "0.1.0";
const SHADCN_FIELD_VERSION: &str = "0.1.0";
const SHADCN_ITEM_VERSION: &str = "0.1.0";
const SHADCN_INPUT_VERSION: &str = "0.1.0";
const SHADCN_TEXTAREA_VERSION: &str = "0.1.0";
const DX_ICON_SEARCH_VERSION: &str = "0.1.0";
const DX_AUTH_BETTER_AUTH_VERSION: &str = "1.6.11-dx.9";
const DX_SUPABASE_CLIENT_VERSION: &str = "0.1.0";
const DX_MIGRATION_STATIC_SITE_VERSION: &str = "0.1.0";

/// Source language for a Forge registry package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeLanguage {
    /// JavaScript or TypeScript source package.
    Js,
    /// Rust source package metadata placeholder.
    Rust,
    /// Python source package metadata placeholder.
    Python,
    /// Go source package metadata placeholder.
    Go,
}

impl DxForgeLanguage {
    /// Registry path segment for the language.
    pub fn as_segment(self) -> &'static str {
        match self {
            Self::Js => "js",
            Self::Rust => "rust",
            Self::Python => "python",
            Self::Go => "go",
        }
    }
}

/// Source location for a Forge registry package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeRegistrySource {
    /// Curated first-party source package.
    Curated,
    /// Package published from a project's root dx file.
    RootDx { project: String },
    /// Future package snapshotted from npm.
    NpmSnapshot { name: String },
    /// Future package snapshotted from another ecosystem.
    ExternalSnapshot { ecosystem: String, name: String },
}

/// Remote registry descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRegistryRemote {
    /// Remote name, for example `r2`.
    pub name: String,
    /// Remote backend kind.
    pub kind: String,
    /// Registry prefix or base path.
    pub prefix: String,
    /// Whether the remote has all required non-secret config values.
    pub configured: bool,
}

/// Remote provider kind understood by Forge planning contracts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeRemoteProviderKind {
    /// Local filesystem registry, fully proven for launch writes.
    LocalFilesystem,
    /// Git-compatible remote such as GitHub or GitLab.
    GitCompatibleRemote,
    /// S3-compatible object storage such as Cloudflare R2.
    S3CompatibleObjectStorage,
    /// Database-backed registry boundary.
    DatabaseBacked,
    /// Custom Forge remote adapter boundary.
    CustomAdapter,
}

/// Read-only remote lifecycle intent for Forge object planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeRemoteReadIntent {
    /// Plan an install from a remote package without touching the network.
    InstallDryRun,
    /// Plan an update from a remote package without touching the network.
    UpdateDryRun,
    /// Plan an uninstall against a remote package without touching the network.
    UninstallDryRun,
}

/// Single remote object a future provider adapter would read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRemoteReadObject {
    /// Object role, for example `package-manifest` or `content-blob`.
    pub intent: String,
    /// Redacted object key; never contains secret values.
    pub object_key: String,
    /// Whether this object is required to complete the lifecycle.
    pub required: bool,
}

/// Execution status for provider object metadata checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeRemoteObjectMetadataStatus {
    /// Forge planned this check, but did not perform a network request.
    PlannedNotChecked,
}

/// Single read-only provider metadata check a future adapter may execute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRemoteObjectMetadataCheck {
    /// Object role, for example `package-manifest` or `content-blob`.
    pub intent: String,
    /// Provider metadata operation. This is `head-object` for S3-compatible stores.
    pub metadata_operation: String,
    /// Redacted object key; never contains secret values.
    pub object_key: String,
    /// Whether this object is required to complete the lifecycle.
    pub required: bool,
    /// Manifest-declared content hash when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_hash: Option<String>,
    /// Manifest-declared byte length when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_bytes: Option<u64>,
    /// Whether this metadata check was actually performed.
    pub status: DxForgeRemoteObjectMetadataStatus,
}

/// Provider object metadata plan for CLI, dx-check, DX-WWW, and future Zed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRemoteObjectMetadataPlan {
    /// Stable schema version for consumers.
    pub schema_version: String,
    /// Provider kind for adapter selection.
    pub provider_kind: DxForgeRemoteProviderKind,
    /// Canonical package id.
    pub package_id: String,
    /// Manifest package version.
    pub version: String,
    /// Whether this plan performed network reads.
    pub network_allowed: bool,
    /// Whether this plan performed remote writes.
    pub write_allowed: bool,
    /// Read-only metadata checks a provider adapter would execute after approval.
    pub checks: Vec<DxForgeRemoteObjectMetadataCheck>,
    /// Honest limitations for the metadata plan.
    pub warnings: Vec<String>,
}

/// Execution status for a read-only object HEAD receipt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeRemoteObjectHeadExecutionStatus {
    /// The object check was not executed because explicit operator approval is required.
    RequiresExplicitApproval,
    /// The object check was executed by an approved provider and measured.
    Measured,
}

/// Measured object metadata returned by a read-only HEAD provider.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRemoteObjectHeadMeasurement {
    /// Whether the object exists according to the provider.
    pub exists: bool,
    /// Provider-reported byte length when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<u64>,
    /// Provider-reported ETag when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    /// Provider-reported last-modified timestamp when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
}

/// Approval record for a read-only object HEAD execution attempt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRemoteObjectHeadExecutionApproval {
    /// Operator or test harness that approved the metadata-only execution.
    pub approved_by: String,
    /// Provider mode, for example `test-provider` or a future `r2-head`.
    pub provider_mode: String,
    /// Whether this execution is allowed to touch a live network provider.
    pub network_allowed: bool,
}

/// Provider boundary for approved read-only object HEAD checks.
pub trait DxForgeRemoteObjectHeadProvider {
    /// Return metadata for a planned object check without writing or fetching blobs.
    fn head_object(
        &self,
        check: &DxForgeRemoteObjectMetadataCheck,
    ) -> Result<DxForgeRemoteObjectHeadMeasurement>;
}

/// Single object HEAD execution receipt entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRemoteObjectHeadExecutionCheck {
    /// Object role, for example `package-manifest` or `content-blob`.
    pub intent: String,
    /// Provider metadata operation. This stays `head-object` for S3-compatible stores.
    pub metadata_operation: String,
    /// Redacted object key; never contains secret values.
    pub object_key: String,
    /// Whether this object is required to complete the lifecycle.
    pub required: bool,
    /// Manifest-declared content hash when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_hash: Option<String>,
    /// Manifest-declared byte length when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_bytes: Option<u64>,
    /// Whether this individual check was approved for live remote metadata probing.
    pub approved: bool,
    /// Whether Forge executed the provider HEAD request.
    pub executed: bool,
    /// Execution status for consumers.
    pub status: DxForgeRemoteObjectHeadExecutionStatus,
    /// Measured remote existence once an approved HEAD check actually runs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measured_exists: Option<bool>,
    /// Measured remote byte length once an approved HEAD check actually runs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measured_bytes: Option<u64>,
    /// Measured provider ETag once an approved HEAD check actually runs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measured_etag: Option<String>,
    /// Measured provider last-modified value once an approved HEAD check actually runs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measured_last_modified: Option<String>,
}

/// Approval-gated receipt shape for future read-only provider HEAD execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRemoteObjectHeadExecutionReceipt {
    /// Stable schema version for CLI, dx-check, DX-WWW, and future Zed consumers.
    pub schema_version: String,
    /// Metadata plan schema this receipt shape was derived from.
    pub source_plan_schema: String,
    /// Provider kind for adapter selection.
    pub provider_kind: DxForgeRemoteProviderKind,
    /// Canonical package id.
    pub package_id: String,
    /// Manifest package version.
    pub version: String,
    /// Provider mode used to produce this receipt.
    pub provider_mode: String,
    /// Whether operator approval is required before executing remote HEAD calls.
    pub approval_required: bool,
    /// Whether explicit approval has been recorded for this receipt.
    pub approved: bool,
    /// Operator or harness that approved execution, when approved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved_by: Option<String>,
    /// Whether this receipt was produced in dry-run mode.
    pub dry_run: bool,
    /// Whether this receipt executed network reads.
    pub network_allowed: bool,
    /// Whether this receipt executed remote writes.
    pub write_allowed: bool,
    /// Per-object execution receipt entries.
    pub checks: Vec<DxForgeRemoteObjectHeadExecutionCheck>,
    /// Honest limitations and safety notes.
    pub warnings: Vec<String>,
    /// Explicit next actions for a future approved execution path.
    pub next_actions: Vec<String>,
}

// Build the approval-gated HEAD execution receipt shape without touching the network.
include!("forge_registry_parts/remote_head_execution.rs");
include!("forge_registry_parts/registry_config.rs");
include!("forge_registry_parts/forge_ui_registry_schema.rs");
/// Resolve package aliases to canonical ids.
pub fn canonical_package_id(package_id: &str) -> &str {
    match package_id {
        "ai-sdk" | "npm/ai" | "vercel-ai" | "vercel/ai" => "ai/vercel-ai",
        "authentication" | "better-auth" | "auth/betterauth" | "auth/better-auth-next" => {
            "auth/better-auth"
        }
        "google/auth" | "google-oauth" | "google-oauth-provider" => "auth/better-auth",
        "docs/fumadocs" | "docs/fumadocs-next" | "fumadocs" | "fumadocs-next" | "mdx/fumadocs" => {
            "content/fumadocs-next"
        }
        "markdown-mdx-content" => "content/react-markdown",
        "mdx/content" => "content/react-markdown",
        "markdown/mdx" | "markdown/react" | "react-markdown" | "remark/react" => {
            "content/react-markdown"
        }
        "database/drizzle" | "db/drizzle" | "drizzle" | "drizzle-orm/sqlite" | "drizzle/sqlite" => {
            "db/drizzle-sqlite"
        }
        "@instantdb/react" | "instantdb" | "instantdb/react" | "db/instantdb" => "instantdb/react",
        "@tanstack/react-query"
        | "data-fetching-cache"
        | "data-fetching/cache"
        | "react-query"
        | "tanstack-query"
        | "tanstack/react-query"
        | "query/tanstack" => "tanstack/query",
        "reactive-store" | "@tanstack/store" | "@tanstack/react-store" | "tanstack-store" => {
            "reactive/store"
        }
        "zustand" | "npm/zustand" | "pmndrs/zustand" | "state/zustand-react" => "state/zustand",
        "icon/search" | "icons/search" => "dx/icon/search",
        "i18n/next" | "i18n/next-intl" | "next-intl" | "next-intl/routing" => "i18n/next-intl",
        "forms" | "forms/rhf" | "forms/react-hook-form" | "react-hook-form" | "rhf" => {
            "forms/react-hook-form"
        }
        "payments" | "@stripe/stripe-js" | "payments/stripe" | "payments/stripe-js" | "stripe"
        | "stripe-js" => "payments/stripe-js",
        "framer-motion"
        | "framer/motion"
        | "motion"
        | "motion-animation"
        | "motion-and-animation"
        | "motion/react" => "animation/motion",
        "automations/n8n" | "n8n" | "n8n-nodes-base" | "workflows/n8n" => "automations/n8n",
        "@trpc/client"
        | "@trpc/react-query"
        | "@trpc/server"
        | "@trpc/tanstack-react-query"
        | "trpc"
        | "trpc/next" => "api/trpc",
        "migrations/static-site"
        | "static/site-migration"
        | "wordpress/static-site"
        | "wp/static-site" => "migration/static-site",
        "database/supabase" | "db/supabase" | "supabase/ssr" | "supabase-js" => "supabase/client",
        "webassembly-bridge" | "webassembly/bridge" | "rust/wasm-bindgen" | "wasm-bindgen"
        | "wasm_bindgen" => "wasm/bindgen",
        "3d/launch-scene" | "3d-scene-system" | "3d/scene" | "three-scene" | "three/r3f/drei"
        | "threejs/scene" | "@react-three/fiber" => "3d/launch-scene",
        "ui/button" => "shadcn/ui/button",
        "ui/badge" => "shadcn/ui/badge",
        "ui/card" => "shadcn/ui/card",
        "ui/alert" => "shadcn/ui/alert",
        "ui/avatar" => "shadcn/ui/avatar",
        "ui/skeleton" => "shadcn/ui/skeleton",
        "ui/label" => "shadcn/ui/label",
        "ui/separator" => "shadcn/ui/separator",
        "ui/field" => "shadcn/ui/field",
        "ui/item" => "shadcn/ui/item",
        "ui/input" => "shadcn/ui/input",
        "ui/textarea" => "shadcn/ui/textarea",
        "schema/zod" | "validation/zod/v4" | "zod" | "zod/v4" => "validation/zod",
        other => other,
    }
}

/// Return the user-facing Forge package id for command output.
pub fn public_forge_package_id(package_id: &str) -> &str {
    match canonical_package_id(package_id) {
        "shadcn/ui/button" => "ui/button",
        "shadcn/ui/badge" => "ui/badge",
        "shadcn/ui/card" => "ui/card",
        "shadcn/ui/alert" => "ui/alert",
        "shadcn/ui/avatar" => "ui/avatar",
        "shadcn/ui/skeleton" => "ui/skeleton",
        "shadcn/ui/label" => "ui/label",
        "shadcn/ui/separator" => "ui/separator",
        "shadcn/ui/field" => "ui/field",
        "shadcn/ui/item" => "ui/item",
        "shadcn/ui/input" => "ui/input",
        "shadcn/ui/textarea" => "ui/textarea",
        other => other,
    }
}

// Build the default materialized source package.
include!("forge_registry_parts/registry_operations.rs");
pub fn registry_operation_markdown(report: &DxForgeRegistryOperationReport) -> String {
    let mut output = format!(
        "# DX Forge Registry\n\n- Action: `{}`\n- Remote: `{}`\n- Dry run: `{}`\n",
        report.action, report.remote, report.dry_run
    );
    if let Some(package_id) = &report.package_id {
        output.push_str(&format!(
            "- Package: `{}`\n",
            public_forge_package_id(package_id)
        ));
    }
    if let Some(version) = &report.version {
        output.push_str(&format!("- Version: `{version}`\n"));
    }
    if let Some(status) = &report.r2_status {
        output.push_str(&format!(
            "- R2 configured: `{}`\n- R2 bucket: `{}`\n- R2 prefix: `{}`\n",
            status.configured,
            status.bucket.as_deref().unwrap_or("-"),
            status.prefix
        ));
    }
    if !report.objects.is_empty() {
        output.push_str("\n## Objects\n\n");
        for object in &report.objects {
            output.push_str(&format!("- `{object}`\n"));
        }
    }
    output
}

impl DxForgeRegistryPackage {
    fn clone_without_content(&self) -> Self {
        let mut package = self.clone();
        package.files = package
            .files
            .iter()
            .map(|file| DxSourceFile {
                path: file.path.clone(),
                logical_path: file.logical_path.clone(),
                hash: file.hash.clone(),
                bytes: file.bytes,
                content: None,
            })
            .collect();
        package
    }
}

include!("forge_registry_parts/package_lanes.rs");
include!("forge_registry_parts/package_templates.rs");
fn migration_static_site_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "js/migrations/static-site/content.ts",
            r#"export type DxStaticMigrationAsset = {
  source: string;
  target: string;
  bytes?: number;
  note?: string;
};

export type DxStaticMigrationPage = {
  sourceKind: "wordpress-export" | "static-html";
  sourceUrl?: string;
  slug: string;
  title: string;
  description?: string;
  publishedAt?: string;
  html: string;
  assets: DxStaticMigrationAsset[];
  warnings: string[];
};

export const sampleStaticMigrationPage: DxStaticMigrationPage = {
  sourceKind: "wordpress-export",
  sourceUrl: "https://example.test/hello-world/",
  slug: "hello-world",
  title: "Hello world",
  description: "Sample content exported from a simple WordPress page.",
  publishedAt: "2026-05-18T00:00:00Z",
  html:
    "<p>This sample page shows the static-content lane for DX Forge migration.</p>" +
    "<p>Forms, comments, search, ecommerce, memberships, and plugin behavior need manual product work.</p>",
  assets: [
    {
      source: "https://example.test/wp-content/uploads/hero.jpg",
      target: "/assets/migrated/hello-world/hero.jpg",
      note: "Copy the original asset, optimize it, and replace the source URL after review.",
    },
  ],
  warnings: [
    "Static content only. Plugin behavior, dynamic blocks, forms, comments, search, ecommerce, and accounts are out of scope.",
    "Imported HTML must be reviewed before production because Forge does not sanitize arbitrary remote markup here.",
    "Redirects, metadata, media optimization, analytics, and CMS editing flows remain application-owned work.",
  ],
};

export function normalizeStaticSlug(value: string): string {
  return value
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
}

export function staticMigrationRoute(page: DxStaticMigrationPage): string {
  const slug = normalizeStaticSlug(page.slug || page.title);
  return `/migrated/${slug || "page"}`;
}
"#,
        ),
        (
            "js/migrations/static-site/page.tsx",
            r#"import * as React from "react";

import {
  sampleStaticMigrationPage,
  staticMigrationRoute,
  type DxStaticMigrationPage,
} from "./content";

export type StaticSiteMigrationPageProps = {
  page?: DxStaticMigrationPage;
  className?: string;
};

export function StaticSiteMigrationPage({
  page = sampleStaticMigrationPage,
  className,
}: StaticSiteMigrationPageProps) {
  return (
    <article className={["mx-auto grid max-w-3xl gap-6 py-10", className].filter(Boolean).join(" ")}>
      <header className="grid gap-2">
        <p className="text-sm font-medium uppercase text-neutral-500">Migrated static page</p>
        <h1 className="text-4xl font-semibold tracking-normal text-neutral-950">{page.title}</h1>
        {page.description ? <p className="text-base text-neutral-600">{page.description}</p> : null}
      </header>

      <section
        className="prose prose-neutral max-w-none"
        dangerouslySetInnerHTML={{ __html: page.html }}
      />

      <section className="grid gap-3 rounded-md border border-amber-200 bg-amber-50 p-4 text-sm text-amber-950">
        <h2 className="text-base font-semibold">Migration review required</h2>
        <ul className="grid gap-2">
          {page.warnings.map((warning) => (
            <li key={warning}>{warning}</li>
          ))}
        </ul>
      </section>
    </article>
  );
}

export function staticMigrationMetadata(page = sampleStaticMigrationPage) {
  return {
    title: page.title,
    description: page.description,
    alternates: { canonical: staticMigrationRoute(page) },
  };
}
"#,
        ),
        (
            "js/migrations/static-site/sample-wordpress-export.json",
            r#"{
  "source_kind": "wordpress-export",
  "source_url": "https://example.test/hello-world/",
  "slug": "hello-world",
  "title": "Hello world",
  "description": "Sample content exported from a simple WordPress page.",
  "html": "<p>This sample page shows the static-content lane for DX Forge migration.</p><p>Dynamic WordPress behavior is intentionally out of scope.</p>",
  "assets": [
    {
      "source": "https://example.test/wp-content/uploads/hero.jpg",
      "target": "/assets/migrated/hello-world/hero.jpg",
      "note": "Copy, optimize, and review before production."
    }
  ],
  "manual_review_required": [
    "Shortcodes and dynamic blocks",
    "Plugin-owned forms, comments, search, ecommerce, memberships, and accounts",
    "Redirects, metadata, analytics, asset optimization, and HTML sanitization"
  ]
}
"#,
        ),
        (
            "js/migrations/static-site/README.md",
            r#"# DX Forge Static Site Migration Example

This source-owned package gives a small, editable starting point for migrating a simple WordPress or static HTML page into a DX-owned route.

No package install is required. Forge writes only local source files, sample content, and review notes into your project.

## Honest scope

- This is not a full WordPress plugin or theme migration.
- It does not migrate comments, forms, search, ecommerce, memberships, accounts, shortcode behavior, block editor semantics, analytics, redirects, or CMS editing flows.
- Imported HTML must be reviewed and sanitized before production.
- Media files should be copied, optimized, and checked against your deployment cache policy.

## Owned files

- `migrations/static-site/content.ts` defines the typed static page model, sample content, slug normalization, and route helper.
- `migrations/static-site/page.tsx` renders a reviewable static page component with visible migration warnings.
- `migrations/static-site/sample-wordpress-export.json` records the fixture shape for a simple imported page.

Use this package as a scoped migration seed: copy content, review every dynamic requirement, then replace the sample fixture with source your app owns.
"#,
        ),
    ]
}

include!("forge_registry_parts/registry_receipts.rs");

#[cfg(test)]
include!("forge_registry_parts/tests.rs");
