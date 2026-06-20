const FORGE_WWW_TEMPLATE_PACKAGE_IDS: [&str; 32] = [
    "shadcn/ui/button",
    "shadcn/ui/badge",
    "shadcn/ui/card",
    "shadcn/ui/alert",
    "shadcn/ui/avatar",
    "shadcn/ui/skeleton",
    "shadcn/ui/label",
    "shadcn/ui/separator",
    "shadcn/ui/field",
    "shadcn/ui/item",
    "shadcn/ui/input",
    "shadcn/ui/textarea",
    "dx/icon/search",
    "auth/better-auth",
    "animation/motion",
    "i18n/next-intl",
    "tanstack/query",
    "validation/zod",
    "forms/react-hook-form",
    "payments/stripe-js",
    "automations/n8n",
    "state/zustand",
    "ai/vercel-ai",
    "api/trpc",
    "content/fumadocs-next",
    "content/react-markdown",
    "supabase/client",
    "db/drizzle-sqlite",
    "instantdb/react",
    "wasm/bindgen",
    "3d/launch-scene",
    "migration/static-site",
];

const FORGE_WWW_TEMPLATE_MATERIALIZED_PACKAGE_IDS: [&str; 30] = [
    "shadcn/ui/badge",
    "shadcn/ui/card",
    "shadcn/ui/alert",
    "shadcn/ui/avatar",
    "shadcn/ui/skeleton",
    "shadcn/ui/label",
    "shadcn/ui/separator",
    "shadcn/ui/field",
    "shadcn/ui/item",
    "shadcn/ui/input",
    "shadcn/ui/textarea",
    "dx/icon/search",
    "auth/better-auth",
    "animation/motion",
    "i18n/next-intl",
    "tanstack/query",
    "validation/zod",
    "forms/react-hook-form",
    "payments/stripe-js",
    "automations/n8n",
    "state/zustand",
    "ai/vercel-ai",
    "api/trpc",
    "content/fumadocs-next",
    "content/react-markdown",
    "supabase/client",
    "db/drizzle-sqlite",
    "instantdb/react",
    "wasm/bindgen",
    "3d/launch-scene",
];

const NEXT_FAMILIAR_TEMPLATE_ROUTE_CONTRACT_TS: &str =
    include_str!("../../../../../examples/onboard/template-route-contract.ts");
const NEXT_FAMILIAR_TEMPLATE_CATALOG_TS: &str = r#"export type TemplateCatalogItem = {
  packageId: string;
  surface: string;
  status: "source-owned";
};

export const templateCatalog: TemplateCatalogItem[] = [
  { packageId: "dx-www/template-shell", surface: "app-shell", status: "source-owned" },
  { packageId: "dx/icon/search", surface: "icons", status: "source-owned" },
  { packageId: "animation/motion", surface: "motion", status: "source-owned" },
];

export const templateCatalogSummary = {
  total: templateCatalog.length,
  sourceOwned: templateCatalog.length,
  generatedFrom: "dx forge",
};
"#;
const NEXT_FAMILIAR_TEMPLATE_SURFACE_REGISTRY_TS: &str =
    include_str!("../../../../../examples/onboard/template-surface-registry.ts");
const NEXT_FAMILIAR_FRAMEWORK_COMPLETENESS_TS: &str =
    include_str!("../../../../../examples/onboard/framework-completeness.ts");
const NEXT_FAMILIAR_SHADCN_DASHBOARD_CONTROLS_CONTRACT_TSX: &str =
    include_str!("../../../../../examples/onboard/shadcn-dashboard-controls-contract.tsx");
const NEXT_FAMILIAR_SHADCN_DASHBOARD_CONTROLS_TSX: &str =
    include_str!("../../../../../examples/onboard/shadcn-dashboard-controls.tsx");
const NEXT_FAMILIAR_AUTH_STATUS_TSX: &str =
    include_str!("../../../../../examples/onboard/auth-session-status.tsx");
const NEXT_FAMILIAR_BETTER_AUTH_DASHBOARD_RECEIPT_JSON: &str = "{}\n";
const NEXT_FAMILIAR_AI_CHAT_STATUS_TSX: &str =
    include_str!("../../../../../examples/onboard/ai-chat-status.tsx");
const NEXT_FAMILIAR_INSTANT_STATUS_TSX: &str =
    include_str!("../../../../../examples/onboard/instantdb-status.tsx");
const NEXT_FAMILIAR_INSTANT_DASHBOARD_RECEIPT_JSON: &str = "{}\n";
const NEXT_FAMILIAR_WASM_STATUS_TSX: &str =
    include_str!("../../../../../examples/onboard/wasm-interop-status.tsx");
const NEXT_FAMILIAR_ZOD_STATUS_TSX: &str =
    include_str!("../../../../../examples/onboard/zod-validation-status.tsx");
const NEXT_FAMILIAR_ZOD_DASHBOARD_SETTINGS_TSX: &str =
    include_str!("../../../../../examples/onboard/zod-dashboard-settings.tsx");
const NEXT_FAMILIAR_DATA_STATUS_TSX: &str =
    include_str!("../../../../../examples/onboard/data-status.tsx");
const NEXT_FAMILIAR_SUPABASE_PROFILE_WORKFLOW_STATE_TS: &str =
    include_str!("../../../../../examples/onboard/supabase-profile-workflow-state.ts");
const NEXT_FAMILIAR_SUPABASE_PROFILE_WORKFLOW_TSX: &str =
    include_str!("../../../../../examples/onboard/supabase-profile-workflow.tsx");
const NEXT_FAMILIAR_SUPABASE_DASHBOARD_RECEIPT_JSON: &str = "{}\n";
const NEXT_FAMILIAR_DRIZZLE_QUERY_PROOF_TSX: &str =
    include_str!("../../../../../examples/onboard/drizzle-query-proof.tsx");
const NEXT_FAMILIAR_PAYMENTS_STATUS_TSX: &str =
    include_str!("../../../../../examples/onboard/payments-status.tsx");
const NEXT_FAMILIAR_STRIPE_BILLING_WORKFLOW_RECEIPT_JSON: &str = "{}\n";
const NEXT_FAMILIAR_DOCS_STATUS_TSX: &str =
    include_str!("../../../../../examples/onboard/docs-status.tsx");
const NEXT_FAMILIAR_DOCS_DASHBOARD_RECEIPT_JSON: &str = "{}\n";
const NEXT_FAMILIAR_LAUNCH_SCENE_TSX: &str =
    include_str!("../../../../../examples/onboard/launch-scene.tsx");
const NEXT_FAMILIAR_SCENE_INDEX_TS: &str =
    include_str!("../../../../../examples/onboard/scene/index.ts");
const NEXT_FAMILIAR_SCENE_TYPES_TS: &str =
    include_str!("../../../../../examples/onboard/scene/types.ts");
const NEXT_FAMILIAR_SCENE_PRESET_TS: &str =
    include_str!("../../../../../examples/onboard/scene/preset.ts");
const NEXT_FAMILIAR_SCENE_INTERACTION_TS: &str =
    include_str!("../../../../../examples/onboard/scene/interaction.ts");
const NEXT_FAMILIAR_SCENE_DASHBOARD_WORKFLOW_TS: &str =
    include_str!("../../../../../examples/onboard/scene/dashboard-workflow.ts");
const NEXT_FAMILIAR_SCENE_DASHBOARD_CONTROLS_TS: &str =
    include_str!("../../../../../examples/onboard/scene/dashboard-controls.ts");
const NEXT_FAMILIAR_SCENE_FRAME_SAMPLE_TS: &str =
    include_str!("../../../../../examples/onboard/scene/frame-sample.ts");
const NEXT_FAMILIAR_SCENE_CAPABILITY_REPORT_TS: &str =
    include_str!("../../../../../examples/onboard/scene/capability-report.ts");
const NEXT_FAMILIAR_SCENE_VIEWPORT_REPORT_TS: &str =
    include_str!("../../../../../examples/onboard/scene/viewport-report.ts");
const NEXT_FAMILIAR_SCENE_BOUNDS_REPORT_TS: &str =
    include_str!("../../../../../examples/onboard/scene/bounds-report.ts");
const NEXT_FAMILIAR_SCENE_RAYCAST_REPORT_TS: &str =
    include_str!("../../../../../examples/onboard/scene/raycast-report.ts");
const NEXT_FAMILIAR_SCENE_PREVIEW_READINESS_TS: &str =
    include_str!("../../../../../examples/onboard/scene/preview-readiness.ts");
const NEXT_FAMILIAR_SCENE_PERFORMANCE_MONITOR_TS: &str =
    include_str!("../../../../../examples/onboard/scene/performance-monitor.ts");
const NEXT_FAMILIAR_SCENE_RENDERER_HANDOFF_TS: &str =
    include_str!("../../../../../examples/onboard/scene/renderer-handoff.ts");
const NEXT_FAMILIAR_SCENE_R3F_RENDERER_ADAPTER_TS: &str =
    include_str!("../../../../../examples/onboard/scene/r3f-renderer-adapter.ts");
const NEXT_FAMILIAR_SCENE_WEBGL_RUNTIME_TS: &str =
    include_str!("../../../../../examples/onboard/scene/webgl-runtime.ts");
const NEXT_FAMILIAR_SCENE_METADATA_TS: &str =
    include_str!("../../../../../examples/onboard/scene/metadata.ts");
const NEXT_FAMILIAR_SCENE_README_MD: &str =
    include_str!("../../../../../examples/onboard/scene/README.md");
const NEXT_FAMILIAR_TEMPLATE_DASHBOARD_NAV_TSX: &str =
    include_str!("../../../../../examples/onboard/template-dashboard-nav.tsx");
const NEXT_FAMILIAR_DX_STUDIO_EDIT_CONTRACT_TS: &str =
    include_str!("../../../../../examples/onboard/dx-studio-edit-contract.ts");
const NEXT_FAMILIAR_AUTOMATIONS_STATUS_TSX: &str =
    include_str!("../../../../../examples/onboard/automations-status.tsx");
const NEXT_FAMILIAR_AUTOMATION_MISSION_SUMMARY_TSX: &str =
    include_str!("../../../../../examples/onboard/automation-mission-summary.tsx");
const NEXT_FAMILIAR_AUTOMATIONS_METADATA_TS: &str =
    include_str!("../../../../../examples/onboard/automations/automations-metadata.ts");
const NEXT_FAMILIAR_AUTOMATION_CONNECTORS_LAUNCH_RECEIPT_JSON: &str = "{}\n";
const NEXT_FAMILIAR_MOTION_INTERACTION_PROOF_TSX: &str =
    include_str!("../../../../../examples/onboard/motion-interaction-proof.tsx");
const NEXT_FAMILIAR_MOTION_DASHBOARD_RECEIPT_JSON: &str = "{}\n";
const NEXT_FAMILIAR_TEMPLATE_LEAD_FORM_TSX: &str =
    include_str!("../../../../../examples/onboard/template-lead-form.tsx");
const NEXT_FAMILIAR_FORMS_DASHBOARD_RECEIPT_JSON: &str = "{}\n";
const NEXT_FAMILIAR_TEMPLATE_SHELL_TSX: &str = r#"export type TemplateShellProps = {
  route?: string;
};

export function TemplateShell({ route = "/" }: TemplateShellProps) {
  return (
    <main data-dx-template-shell data-dx-route={route}>
      <section>
        <p>DX WWW source-owned app shell.</p>
      </section>
    </main>
  );
}
"#;
const NEXT_FAMILIAR_ICON_STATUS_TSX: &str =
    include_str!("../../../../../examples/onboard/icon-status.tsx");
const NEXT_FAMILIAR_INTL_DASHBOARD_LOCALE_CONTRACT_TS: &str =
    include_str!("../../../../../examples/onboard/next-intl-dashboard-locale-contract.ts");
const NEXT_FAMILIAR_INTL_DASHBOARD_LOCALE_TSX: &str =
    include_str!("../../../../../examples/onboard/next-intl-dashboard-locale.tsx");
const NEXT_FAMILIAR_INTL_DASHBOARD_RECEIPT_JSON: &str = "{}\n";
const NEXT_FAMILIAR_INTL_STATUS_TSX: &str =
    include_str!("../../../../../examples/onboard/next-intl-status.tsx");
const NEXT_FAMILIAR_QUERY_STATUS_TSX: &str =
    include_str!("../../../../../examples/onboard/query-cache-status.tsx");
const NEXT_FAMILIAR_QUERY_DASHBOARD_READ_MODEL_TS: &str =
    include_str!("../../../../../examples/onboard/query-dashboard-read-model.ts");
const NEXT_FAMILIAR_FORGE_PACKAGE_STATUS_TS: &str =
    include_str!("../../../../../examples/onboard/forge-package-status.ts");
const NEXT_FAMILIAR_FORGE_PACKAGE_STATUS_READ_MODEL_TS: &str = r#"export type ForgePackageStatusReadModel = {
  source: ".dx/forge/package-status.json";
  authority: "read-model";
  packageCount: number;
};

export const forgePackageStatusReadModel: ForgePackageStatusReadModel = {
  source: ".dx/forge/package-status.json",
  authority: "read-model",
  packageCount: 0,
};

export function getForgePackageStatusReadModel() {
  return forgePackageStatusReadModel;
}
"#;
const NEXT_FAMILIAR_FORGE_GOLDEN_PATH_CONTRACT_TS: &str =
    include_str!("../../../../../examples/onboard/forge-golden-path-contract.ts");
const NEXT_FAMILIAR_FORGE_GOLDEN_PATH_PANEL_TSX: &str =
    include_str!("../../../../../examples/onboard/forge-golden-path-panel.tsx");
const NEXT_FAMILIAR_FORGE_SAFETY_ARCHIVE_CONTRACT_TS: &str =
    include_str!("../../../../../examples/onboard/forge-safety-archive-contract.ts");
const NEXT_FAMILIAR_FORGE_SAFETY_ARCHIVE_RUNBOOK_TS: &str =
    include_str!("../../../../../examples/onboard/forge-safety-archive-runbook.ts");
const NEXT_FAMILIAR_FORGE_SAFETY_ARCHIVE_PANEL_TSX: &str =
    include_str!("../../../../../examples/onboard/forge-safety-archive-panel.tsx");
const NEXT_FAMILIAR_FORGE_REMOTE_HEAD_HEALTH_CONTRACT_TS: &str =
    include_str!("../../../../../examples/onboard/forge-remote-head-health-contract.ts");
const NEXT_FAMILIAR_FORGE_REMOTE_HEAD_HEALTH_PANEL_TSX: &str =
    include_str!("../../../../../examples/onboard/forge-remote-head-health-panel.tsx");
const NEXT_FAMILIAR_MARKDOWN_PREVIEW_TSX: &str =
    include_str!("../../../../../examples/onboard/react-markdown-preview.tsx");
const NEXT_FAMILIAR_STATE_COUNTER_TSX: &str =
    include_str!("../../../../../examples/onboard/state-zustand-counter.tsx");
const NEXT_FAMILIAR_STATE_DASHBOARD_TSX: &str =
    include_str!("../../../../../examples/onboard/state-zustand-dashboard.tsx");
const NEXT_FAMILIAR_ZUSTAND_DASHBOARD_RECEIPT_JSON: &str = "{}\n";
const NEXT_FAMILIAR_TRPC_CONTRACT_TS: &str =
    include_str!("../../../../../examples/onboard/trpc-launch-contract.ts");
const NEXT_FAMILIAR_TRPC_HEALTH_TSX: &str =
    include_str!("../../../../../examples/onboard/trpc-launch-health.tsx");

const NEXT_FAMILIAR_TEMPLATE_CONSOLE_TSX: &str = r#""use client";

import { TemplateShell } from "./template-shell";

export function TemplateConsole() {
  return <TemplateShell route="/" />;
}
"#;

const NEXT_FAMILIAR_SERVER_TEMPLATE_CATALOG_TS: &str = r#"import { templateCatalogSummary } from "../components/template-app/package-catalog";

export async function loadTemplateCatalog() {
  return templateCatalogSummary();
}
"#;

const NEXT_FAMILIAR_LAUNCH_RECEIPT_PACKAGE_ID: &str = "dx-www/template-shell";
const NEXT_FAMILIAR_LAUNCH_RECEIPT_VARIANT: &str = "next-familiar";
const NEXT_FAMILIAR_LAUNCH_RECEIPT_GLOB: &str =
    ".dx/forge/receipts/*-dx-www-template-shell--variant-next-familiar.json";
const NEXT_FAMILIAR_LAUNCH_RECEIPT_DOC: &str =
    ".dx/forge/docs/dx-www-template-shell--variant-next-familiar.md";
const NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE: &str =
    ".dx/forge/template-readiness/launch-route.json";
const NEXT_FAMILIAR_LAUNCH_ZED_TEMPLATE_HANDOFF_FILE: &str =
    ".dx/forge/template-readiness/zed-template-handoff.json";
const NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE: &str =
    ".dx/forge/template-readiness/launch-readiness-bundle.json";
const NEXT_FAMILIAR_LAUNCH_ADOPTION_REPORT_FILE: &str =
    ".dx/forge/reports/launch-adoption-report.json";
const NEXT_FAMILIAR_LAUNCH_MANIFEST_DRIFT_FILE: &str =
    ".dx/forge/reports/launch-manifest-drift.json";
const NEXT_FAMILIAR_LAUNCH_COMPANION_DOC_RECEIPTS_FILE: &str =
    ".dx/forge/template-readiness/launch-companion-doc-receipts.json";
const NEXT_FAMILIAR_LAUNCH_RUNTIME_CHECKLIST_FILE: &str =
    ".dx/forge/template-readiness/launch-runtime-checklist.json";
const NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE: &str =
    ".dx/forge/template-readiness/launch-runtime-approval-request.json";
const NEXT_FAMILIAR_LAUNCH_RUNTIME_EVIDENCE_FILE: &str =
    ".dx/forge/template-readiness/launch-runtime-evidence.json";
const NEXT_FAMILIAR_LAUNCH_RUNTIME_FINALIZATION_RECEIPT_FILE: &str =
    ".dx/forge/runtime/final-launch-evidence-receipt.json";
const NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE: &str =
    ".dx/forge/runtime/final-launch-evidence-review.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE: &str =
    ".dx/forge/release/launch-evidence-packet.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE: &str =
    ".dx/forge/release/launch-evidence-operator-index.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_STATUS_TIMELINE_FILE: &str =
    ".dx/forge/release/launch-evidence-status-timeline.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE: &str =
    ".dx/forge/release/launch-evidence-handoff-digest.md";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE: &str =
    ".dx/forge/release/launch-evidence-release-checklist.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE: &str =
    ".dx/forge/release/launch-evidence-share-.dx/build-cache/manifest.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE: &str =
    ".dx/forge/release/launch-evidence-archive-index.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_RECEIPT_FILE: &str =
    ".dx/forge/release/launch-evidence-archive-receipt.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_LEDGER_FILE: &str =
    ".dx/forge/release/launch-evidence-archive-ledger.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_POLICY_FILE: &str =
    ".dx/forge/release/launch-evidence-retention-policy.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_REVIEW_FILE: &str =
    ".dx/forge/release/launch-evidence-retention-review.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE: &str =
    ".dx/forge/release/launch-evidence-release-seal.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_SUMMARY_FILE: &str =
    ".dx/forge/release/launch-evidence-operator-summary.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_COMPLETION_LEDGER_FILE: &str =
    ".dx/forge/release/launch-evidence-completion-ledger.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_CLOSURE_MEMO_FILE: &str =
    ".dx/forge/release/launch-evidence-closure-memo.md";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_FINAL_BRIEF_FILE: &str =
    ".dx/forge/release/launch-evidence-final-brief.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RUNBOOK_FILE: &str =
    ".dx/forge/release/launch-evidence-operator-runbook.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_CAPSULE_FILE: &str =
    ".dx/forge/release/launch-evidence-handoff-capsule.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESUMPTION_INDEX_FILE: &str =
    ".dx/forge/release/launch-evidence-resumption-index.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_RECOVERY_BRIEF_FILE: &str =
    ".dx/forge/release/launch-evidence-recovery-brief.md";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_CONTINUATION_PACKET_FILE: &str =
    ".dx/forge/release/launch-evidence-continuation-packet.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RESUME_CARD_FILE: &str =
    ".dx/forge/release/launch-evidence-operator-resume-card.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_LEDGER_FILE: &str =
    ".dx/forge/release/launch-evidence-restart-ledger.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CHECKLIST_FILE: &str =
    ".dx/forge/release/launch-evidence-restart-checklist.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_BRIEF_FILE: &str =
    ".dx/forge/release/launch-evidence-restart-brief.md";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_MANIFEST_FILE: &str =
    ".dx/forge/release/launch-evidence-restart-.dx/build-cache/manifest.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_RECEIPT_FILE: &str =
    ".dx/forge/release/launch-evidence-restart-receipt.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SUMMARY_FILE: &str =
    ".dx/forge/release/launch-evidence-restart-summary.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SNAPSHOT_FILE: &str =
    ".dx/forge/release/launch-evidence-restart-snapshot.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_DISPATCH_FILE: &str =
    ".dx/forge/release/launch-evidence-restart-dispatch.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CLOSEOUT_FILE: &str =
    ".dx/forge/release/launch-evidence-restart-closeout.md";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SIGNOFF_FILE: &str =
    ".dx/forge/release/launch-evidence-restart-signoff.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_ACCEPTANCE_INDEX_FILE: &str =
    ".dx/forge/release/launch-evidence-acceptance-index.md";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_ACCEPTANCE_DIGEST_FILE: &str =
    ".dx/forge/release/launch-evidence-acceptance-digest.json";
const NEXT_FAMILIAR_LAUNCH_EVIDENCE_FRIDAY_BATON_FILE: &str =
    ".dx/forge/release/launch-evidence-friday-baton.md";
const NEXT_FAMILIAR_LAUNCH_VERIFICATION_LANE_FILE: &str =
    ".dx/forge/template-readiness/launch-verification-lane.json";

fn www_template_catalog_metadata() -> Vec<serde_json::Value> {
    vec![
        serde_json::json!({"package_id": "shadcn/ui/button", "role": "ui-primitive", "command": "dx add ui/button --write", "env": [], "app_owned_boundaries": ["Button copy, disabled states, and command intent"]}),
        serde_json::json!({"package_id": "shadcn/ui/badge", "role": "ui-primitive", "command": "dx add ui/badge --write", "env": [], "app_owned_boundaries": ["Status taxonomy, labels, tone, and accessibility"]}),
        serde_json::json!({"package_id": "shadcn/ui/card", "role": "ui-primitive", "command": "dx add ui/card --write", "env": [], "app_owned_boundaries": ["Information hierarchy and responsive density"]}),
        serde_json::json!({"package_id": "shadcn/ui/alert", "role": "ui-primitive", "command": "dx add ui/alert --write", "env": [], "app_owned_boundaries": ["Severity language, recovery actions, and alert placement"]}),
        serde_json::json!({"package_id": "shadcn/ui/avatar", "role": "ui-primitive", "command": "dx add ui/avatar --write", "env": [], "app_owned_boundaries": ["Identity source, fallback initials, image policy, and profile loading"]}),
        serde_json::json!({"package_id": "shadcn/ui/skeleton", "role": "ui-primitive", "command": "dx add ui/skeleton --write", "env": [], "app_owned_boundaries": ["Loading layout, perceived-performance policy, and content reserve size"]}),
        serde_json::json!({"package_id": "shadcn/ui/label", "role": "ui-primitive", "command": "dx add ui/label --write", "env": [], "app_owned_boundaries": ["Form copy, accessible names, descriptions, and validation relationships"]}),
        serde_json::json!({"package_id": "shadcn/ui/separator", "role": "ui-primitive", "command": "dx add ui/separator --write", "env": [], "app_owned_boundaries": ["Section rhythm, information hierarchy, and decorative versus semantic divider policy"]}),
        serde_json::json!({"package_id": "shadcn/ui/field", "role": "ui-primitive", "command": "dx add ui/field --write", "env": [], "app_owned_boundaries": ["Field grouping, accessible errors, orientation, and form rhythm"]}),
        serde_json::json!({"package_id": "shadcn/ui/item", "role": "ui-primitive", "command": "dx add ui/item --write", "env": [], "app_owned_boundaries": ["List semantics, row actions, keyboard reachability, and row-level authorization"]}),
        serde_json::json!({"package_id": "shadcn/ui/input", "role": "ui-primitive", "command": "dx add ui/input --write", "env": [], "app_owned_boundaries": ["Field labels, validation copy, and accessibility"]}),
        serde_json::json!({"package_id": "shadcn/ui/textarea", "role": "ui-primitive", "command": "dx add ui/textarea --write", "env": [], "app_owned_boundaries": ["Long-form input limits and review workflow"]}),
        serde_json::json!({"package_id": "dx/icon/search", "role": "selected-asset", "command": "dx add icon/search --write", "env": [], "app_owned_boundaries": ["Icon meaning, placement, and accessible label"]}),
        serde_json::json!({
            "package_id": "auth/better-auth",
            "official_name": "Authentication",
            "aliases": ["authentication", "better-auth", "auth/betterauth", "auth/better-auth-next", "google-oauth"],
            "role": "auth",
            "command": "dx add authentication --write",
            "env": ["BETTER_AUTH_SECRET", "BETTER_AUTH_URL"],
            "required_env": ["BETTER_AUTH_SECRET", "BETTER_AUTH_URL"],
            "provider_env": [
                "GOOGLE_CLIENT_ID",
                "GOOGLE_CLIENT_SECRET"
            ],
            "docs_path": "docs/packages/authentication.md",
            "credential_state": "missing-config",
            "source_mirror": "G:/WWW/inspirations/better-auth",
            "provenance": {
                "source": "dx-forge-curated-registry",
                "upstream_package": "better-auth",
                "upstream_version": "1.6.11",
                "source_mirror": "G:/WWW/inspirations/better-auth",
                "source_subpath": "packages/better-auth",
                "note": "Authentication source surfaces use upstream better-auth APIs. DX inspected the local upstream better-auth source mirror and export map for betterAuth, createAuthClient, Next.js handlers/cookies, session helpers, email/password, social sign-in, linked accounts, profile, deletion, security, and dashboard workflow APIs."
            },
            "exported_files": [
                "auth/better-auth/options.ts",
                "auth/better-auth/server.ts",
                "auth/better-auth/client.ts",
                "auth/better-auth/email-password.ts",
                "auth/better-auth/social.ts",
                "auth/better-auth/accounts.ts",
                "auth/better-auth/profile.ts",
                "auth/better-auth/account-deletion.ts",
                "auth/better-auth/account-security.ts",
                "auth/better-auth/route.ts",
                "auth/better-auth/session.ts",
                "auth/better-auth/session-management.ts",
                "auth/better-auth/dashboard.ts",
                "auth/better-auth/metadata.ts",
                "auth/better-auth/.env.example",
                "auth/better-auth/README.md",
                "components/template-app/auth-session-status.tsx"
            ],
            "dashboard_usage": "The root dashboard uses LaunchAuthSessionStatus and data-dx-component=\"better-auth-account-dashboard-workflow\" for Authentication email sign-up missing-config readiness, provider boundary review, local demo session preview, sign-out action state, and account/profile policy receipts without a template-local node_modules workflow.",
            "receipt_paths": [
                ".dx/forge/receipts/auth-better-auth.json",
                "examples/template/.dx/forge/receipts/auth-better-auth.json"
            ],
            "dx_check_visibility": {
                "schema": "dx.forge.package.dx_check_visibility",
                "current_status": "present",
                "statuses": [
                    "present",
                    "stale",
                    "missing-receipt",
                    "blocked",
                    "unsupported-surface"
                ],
                "receipt_path": "examples/template/.dx/forge/receipts/auth-better-auth.json",
                "monitored_surfaces": [
                    "authentication-account-workflow",
                    "authentication-session-status"
                ]
            },
            "dx_icon": "pack:auth",
            "app_owned_boundaries": [
                "Session policy, trusted origins, database adapter, OAuth provider credentials, email delivery, password reset policy, account deletion policy, and production cookie domain"
            ]
        }),
        serde_json::json!({"package_id": "animation/motion", "official_package_name": "Motion & Animation", "aliases": ["motion-animation", "motion-and-animation", "motion", "framer-motion", "motion/react", "animation/motion"], "role": "animation", "command": "dx add motion-animation --write", "env": [], "required_env": [], "source_mirror": "G:/WWW/inspirations/motion", "exported_files": ["js/motion/provider.tsx", "js/motion/controls.tsx", "js/motion/lazy.tsx", "js/motion/layout.tsx", "js/motion/motion-values.tsx", "js/motion/presence.tsx", "js/motion/reorder.tsx", "js/motion/reveal.tsx", "js/motion/scoped-animate.tsx", "js/motion/scroll-progress.tsx", "js/motion/dashboard-workflow.ts", "components/template-app/motion-interaction-proof.tsx"], "dashboard_usage": "/launch uses animation/motion as data-dx-component=\"launch-motion-dashboard-workflow\" for stage advance, reorder preview, reset, reduced-motion policy preview, and safe local motion receipts without a template-local node_modules workflow.", "receipt_paths": [".dx/forge/receipts/*-animation-motion.json", "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json", ".dx/forge/docs/animation-motion.md", "docs/packages/animation-motion.md"], "dx_icon": "pack:motion", "app_owned_boundaries": ["Reduced-motion behavior, route transition choreography, gesture semantics, reorder persistence, keyboard sorting, browser QA, and motion-density performance budget"]}),
        serde_json::json!({"package_id": "forms/react-hook-form", "official_name": "Forms", "aliases": ["forms", "react-hook-form", "rhf", "forms/rhf"], "upstream_package": "react-hook-form", "upstream_version": "7.75.0", "source_mirror": "G:/WWW/inspirations/react-hook-form", "role": "forms", "command": "dx add forms --write", "env": [], "receipt_paths": ["docs/packages/forms-react-hook-form.md", "examples/template/.dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json"], "app_owned_boundaries": ["Submit handlers, spam protection, validation rules, accessibility review, persistence, authorization, dependency installation, and governed browser runtime proof"]}),
        serde_json::json!({"package_id": "i18n/next-intl", "official_name": "Internationalization", "upstream_package": "next-intl", "upstream_version": "4.12.0", "aliases": ["next-intl", "intl", "i18n/next"], "role": "i18n", "command": "dx add next-intl --write", "env": [], "required_env": [], "source_mirror": "G:/WWW/inspirations/next-intl", "exported_files": ["i18n/dashboard-copy.ts", "i18n/dashboard-locale-workflow.tsx", "components/template-app/next-intl-dashboard-locale-contract.ts", "components/template-app/next-intl-dashboard-locale.tsx", "examples/template/next-intl-dashboard-locale-contract.ts", "examples/template/next-intl-dashboard-locale.tsx", "i18n/messages/en.json", "i18n/messages/bn.json", "i18n/provider.tsx", "i18n/request.ts"], "dashboard_usage": "The root route imports TemplateDashboardIntlWorkflow for locale switching, mission copy, plan copy, route preview, formatter preview, localized plan-price preview, support SLA copy, and safe locale receipt preparation.", "receipt_paths": [".dx/forge/receipts/*-i18n-next-intl.json", ".dx/forge/docs/i18n-next-intl.md", "docs/packages/next-intl.md", "examples/template/.dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json"], "dx_icon": "pack:i18n", "app_owned_boundaries": ["Translated message quality, locale routing policy, middleware placement, SEO metadata, production route alternates, and runtime dependency installation"]}),
        serde_json::json!({"package_id": "tanstack/query", "official_name": "Data Fetching & Cache", "upstream_package": "@tanstack/react-query", "role": "server-state", "command": "dx add data-fetching-cache --write", "env": [], "app_owned_boundaries": ["Query keys, fetchers, cache invalidation, loading UI, and error UI"]}),
        serde_json::json!({"package_id": "validation/zod", "role": "validation", "command": "dx add zod --write", "env": [], "app_owned_boundaries": ["Accepted schema design, external JSON Schema trust policy, experimental z.fromJSONSchema upgrade timing, form draft policy, submit acceptance policy, query-string coercion policy, locale/error-copy policy, global-config timing, catalog item ownership, package approval policy, launch approval authority, and downstream authorization"]}),
        serde_json::json!({"package_id": "payments/stripe-js", "official_package_name": "Payments", "official_name": "Payments", "upstream_package": "@stripe/stripe-js", "upstream_version": "9.6.0", "aliases": ["payments", "stripe-js", "@stripe/stripe-js", "stripe", "payments/stripe"], "role": "payments", "command": "dx add payments --write", "env": ["NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY", "STRIPE_SECRET_KEY", "STRIPE_PRICE_ID", "STRIPE_PRICE_ID_STARTER", "STRIPE_PRICE_ID_TEAM", "STRIPE_PRICE_ID_SCALE"], "source_mirror": "G:/WWW/inspirations/stripe-js", "exported_files": ["lib/payments/stripe-js/config.ts", "lib/payments/stripe-js/client.ts", "lib/payments/stripe-js/payment.ts", "lib/payments/stripe-js/checkout.ts", "lib/payments/stripe-js/dashboard-checkout.ts", "lib/payments/stripe-js/server.ts", "app/api/checkout/route.ts", "app/api/stripe/webhook/route.ts", "lib/payments/stripe-js/metadata.ts", "components/template-app/payments-status.tsx", "examples/dashboard/src/components/StripePlanCheckout.tsx"], "receipt_paths": [".dx/forge/docs/payments-stripe-js.md", ".dx/forge/receipts/*-payments-stripe-js.json", ".dx/forge/docs/dashboard-stripe-plan-checkout.md", "docs/packages/payments-stripe-js.md", "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json"], "app_owned_boundaries": ["Stripe account, product catalog, Price IDs, plan env mapping, tax, fraud, dispute, refund policy, webhook fulfillment, entitlement mapping, and PCI boundaries"]}),
        serde_json::json!({
            "package_id": "automations/n8n",
            "official_name": "Automation Connectors",
            "official_package_name": "Automation Connectors",
            "aliases": ["automation-connectors", "@n8n/nodes-base", "n8n-nodes-base", "workflow/n8n"],
            "upstream_package": "n8n-nodes-base",
            "upstream_version": "2.22.0",
            "role": "automations",
            "command": "dx add automation-connectors --write",
            "env": [],
            "required_env": ["SLACK_BOT_TOKEN", "SLACK_SIGNING_SECRET", "NOTION_API_KEY", "DX_AUTOMATIONS_OPERATOR_APPROVAL"],
            "source_mirror": "G:/WWW/inspirations/n8n/packages/nodes-base",
            "inspected_source_files": [
                "packages/nodes-base/package.json",
                "packages/nodes-base/nodes/ManualTrigger/ManualTrigger.node.ts",
                "packages/nodes-base/nodes/Slack/Slack.node.ts",
                "packages/nodes-base/nodes/Notion/Notion.node.ts",
                "packages/nodes-base/credentials/SlackApi.credentials.ts",
                "packages/nodes-base/credentials/SlackOAuth2Api.credentials.ts",
                "packages/nodes-base/credentials/NotionApi.credentials.ts"
            ],
            "selected_surfaces": [
                "connector-catalog",
                "credential-readiness",
                "redacted-run-receipt",
                "template-dashboard-workflow",
                "starter-dashboard-workflow",
                "zed-run-handoff"
            ],
            "honesty_label": "ADAPTER-BOUNDARY",
            "dx_check_visibility": {
                "schema": "dx.forge.package.dx_check_visibility",
                "current_status": "present",
                "statuses": ["present", "stale", "missing-receipt", "blocked", "unsupported-surface"],
                "receipt_path": "G:/Dx/.dx/receipts/automations/launch-release-notification.json",
                "monitored_surfaces": [
                    "launch-automation-dashboard-workflow",
                    "launch-automation-connector-workflow",
                    "launch-automation-mission-summary",
                    "dashboard-automation-workflow",
                    "zed-run-receipt"
                ]
            },
            "exported_files": ["lib/automations/n8n/metadata.ts", "lib/automations/n8n/catalog.ts", "lib/automations/n8n/readiness.ts", "lib/automations/n8n/receipt.ts", "components/template-app/template-shell.tsx", "components/template-app/automations-status.tsx", "components/template-app/automation-mission-summary.tsx", "components/template-app/automations/automations-metadata.ts", "pages/index.html", "tools/launch/runtime-template/assets/launch-runtime.ts", "examples/dashboard/src/lib/n8nAutomationBridge.ts", "examples/dashboard/src/components/AutomationWorkflowPanel.tsx", "docs/packages/automations-n8n.md"],
            "dashboard_usage": "/launch imports LaunchAutomationBridgeStatus as data-dx-component=\"launch-automation-dashboard-workflow\" and publishes LaunchAutomationDashboardState into LaunchAutomationMissionSummary for data-dx-component=\"launch-automation-mission-summary\" release notification intent, connector readiness, missing-config credential gates, credential schema and workflow-node readiness, required env markers, data-dx-automation-dashboard-state, normalized data-dx-automation-receipt-intent and data-dx-automation-run-receipt-intent markers, redacted draft receipt handoff, visible prepare-zed-run-handoff action, and the G:/Dx/.dx/receipts/automations/run-latest.json Rust/Zed receipt path.",
            "receipt_paths": ["G:/Dx/.dx/receipts/automations/launch-release-notification.json", "G:/Dx/.dx/receipts/automations/run-latest.json", ".dx/forge/receipts/automations/run-latest.json", ".dx/forge/receipts/automations/readiness.json"],
            "app_owned_boundaries": ["Connector selection, credential approval, workflow execution, receipt retention, and secret handoff"]
        }),
        serde_json::json!({"package_id": "state/zustand", "name": "State Management", "official_name": "State Management", "upstream_package": "zustand", "upstream_version": "5.0.13", "source_mirror": "G:/WWW/inspirations/zustand", "role": "launch-state", "command": "dx add zustand --write", "env": [], "app_owned_boundaries": ["Selector granularity, equality functions, middleware mutator policy, app-owned immer dependency, DevTools availability, action taxonomy, SSR mutation safety, persist hydration lifecycle, sensitive-state policy, durable storage, and browser persistence review"]}),
        serde_json::json!({"package_id": "ai/vercel-ai", "official_name": "AI SDK", "official_dx_package_name": "AI SDK", "aliases": ["vercel-ai", "ai-sdk", "@vercel/ai"], "upstream_package": "ai", "upstream_version": "7.0.0-canary.146", "based_on": "Vercel AI SDK", "role": "ai", "command": "dx add ai-sdk --write", "env": ["AI_PROVIDER_API_KEY", "AI_GATEWAY_API_KEY"], "required_env": ["AI_PROVIDER_API_KEY"], "source_mirror": "G:/WWW/inspirations/vercel-ai", "inspected_source_files": ["packages/ai/package.json", "packages/ai/src/index.ts", "packages/ai/src/generate-text/index.ts", "packages/ai/src/ui/index.ts", "packages/ai/src/registry/index.ts"], "selected_surfaces": ["chat-route", "provider-readiness", "dashboard-workflow", "route-contract-preview"], "honesty_label": "ADAPTER-BOUNDARY", "dx_check_visibility": {"schema": "dx.forge.package.dx_check_visibility", "current_status": "present", "statuses": ["present", "stale", "missing-receipt", "blocked", "unsupported-surface"], "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-ai-vercel-ai-launch-assistant.json"}, "exported_files": ["lib/ai/model.ts", "lib/ai/chat-route.ts", "lib/ai/client-chat.tsx", "lib/ai/provider-freedom.ts", "lib/ai/dashboard-readiness.ts", "components/ai/ai-launch-assistant.tsx", "components/template-app/ai-chat-status.tsx", "lib/ai/metadata.ts"], "dashboard_usage": "/launch uses the AI SDK Launch Assistant as data-dx-component=\"launch-ai-assistant-dashboard-workflow\" for provider selection, prompt editing, /api/ai/chat route-contract preview, missing-config receipts, and DX icon markers.", "receipt_paths": [".dx/forge/docs/ai-vercel-ai.md", ".dx/forge/receipts/*-ai-vercel-ai.json", "docs/packages/ai-vercel-ai.md", "examples/template/.dx/forge/receipts/2026-05-22-ai-vercel-ai-launch-assistant.json"], "app_owned_boundaries": ["Provider keys, gateway routing policy, model safety, moderation, persistence, rate limits, billing controls, and live streaming credentials"]}),
        serde_json::json!({
            "package_id": "api/trpc",
            "official_name": "Type-Safe API",
            "official_dx_package_name": "Type-Safe API",
            "aliases": ["trpc", "trpc/next", "@trpc/server", "@trpc/client", "@trpc/tanstack-react-query"],
            "upstream_package": "@trpc/server",
            "upstream_version": "11.17.0",
            "role": "api",
            "command": "dx add trpc --write",
            "env": [],
            "required_env": [],
            "source_mirror": "G:/WWW/inspirations/trpc",
            "selected_surfaces": [
                "trpc-template-dashboard-workflow",
                "trpc-starter-dashboard-workflow",
                "trpc-route-handler"
            ],
            "honesty_label": "SOURCE-ONLY",
            "dx_check_visibility": {
                "schema": "dx.forge.package.dx_check_visibility",
                "current_status": "present",
                "statuses": ["present", "stale", "missing-receipt", "blocked", "unsupported-surface"],
                "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json",
                "monitored_surfaces": [
                    "trpc-template-dashboard-workflow",
                    "trpc-starter-dashboard-workflow",
                    "trpc-route-handler"
                ],
                "dx_check_metrics": [
                    "type_safe_api_receipt_present",
                    "type_safe_api_receipt_stale",
                    "type_safe_api_missing_receipt",
                    "type_safe_api_blocked_surface",
                    "type_safe_api_unsupported_surface",
                    "type_safe_api_hash_manifest_present",
                    "type_safe_api_hash_mismatch"
                ]
            },
            "exported_files": [
                "lib/trpc/context.ts",
                "lib/trpc/router.ts",
                "lib/trpc/route-handler.ts",
                "lib/trpc/client.ts",
                "lib/trpc/provider.tsx",
                "lib/trpc/dashboard-workflow.ts",
                "components/dashboard/trpc-dashboard-workflow.tsx",
                "components/template-app/trpc-launch-contract.ts",
                "components/template-app/trpc-launch-health.tsx"
            ],
            "receipt_paths": [
                ".dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json",
                ".dx/forge/receipts/api-trpc.json",
                "examples/template/.dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json",
                "docs/packages/api-trpc.md"
            ],
            "app_owned_boundaries": [
                "Domain routers, authorization, sessions, request limits, subscription fan-out, stream pacing, runtime dependency installation, and production observability"
            ]
        }),
        serde_json::json!({"package_id": "content/fumadocs-next", "aliases": ["fumadocs", "fumadocs-next", "docs"], "role": "docs", "command": "dx add fumadocs --write", "env": ["DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS"], "required_env": ["DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS"], "source_mirror": "G:/WWW/inspirations/fumadocs", "exported_files": ["dx", "lib/fumadocs/source.ts", "lib/fumadocs/source-plugins.tsx", "lib/fumadocs/navigation.ts", "lib/fumadocs/toc.ts", "lib/fumadocs/llms.ts", "lib/fumadocs/openapi.ts", "lib/fumadocs/search.ts", "lib/fumadocs/search-client.ts", "lib/fumadocs/readiness.ts", "lib/fumadocs/dashboard-workflow.ts", "components/dashboard/fumadocs-docs-workflow.tsx", "components/template-app/docs-status.tsx", "app/docs/readiness/route.ts"], "dashboard_usage": "The root dashboard imports LaunchDocsStatus for docs route selection, markdown rendering, OpenAPI missing-config readiness, changelog help, mission-control docs sync, and safe local route receipts.", "receipt_paths": [".dx/forge/docs/content-fumadocs-next.md", ".dx/forge/receipts/*-content-fumadocs-next.json", ".dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json", "docs/packages/content-fumadocs-next.md", "examples/template/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json"], "dx_icon": "pack:fumadocs", "app_owned_boundaries": ["Content governance, framework.www.* and framework.fumadocs.* values in dx, source plugin taxonomy, navigation policy, toc policy, slug/canonical URL policy, OpenAPI schema governance, OpenAPI proxy allowed origins, auth/cookie forwarding policy, request code sample policy, AI indexing policy, private content exclusion, search UI, static-index budget, multilingual/vector policy, and runtime verification"]}),
        serde_json::json!({"package_id": "content/react-markdown", "role": "content-rendering", "command": "dx add react-markdown --write", "env": [], "app_owned_boundaries": ["Content moderation, plugin policy, link safety, raw HTML review, and typography"]}),
        serde_json::json!({"package_id": "supabase/client", "official_name": "Backend Platform Client", "upstream_package": "@supabase/ssr + @supabase/supabase-js", "upstream_version": "@supabase/ssr latest; @supabase/supabase-js ^2", "aliases": ["db/supabase", "supabase/ssr", "backend/supabase"], "role": "backend-client", "command": "dx add supabase/client --write", "env": ["NEXT_PUBLIC_SUPABASE_URL", "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY"], "source_mirror": "G:/WWW/inspirations/supabase", "exported_files": ["lib/supabase/env.ts", "lib/supabase/browser.ts", "lib/supabase/server.ts", "lib/supabase/profiles.ts", "lib/supabase/profile-workflow.ts", "lib/supabase/metadata.ts", "components/template-app/supabase-profile-workflow.tsx"], "dashboard_usage": "/launch account-data-dashboard imports LaunchSupabaseProfileWorkflow for editable profile draft and safe upsert receipts; data-status.tsx exposes supabase-schema-query as the local profiles read-model readiness workflow; the visible DOM carries data-dx-supabase-receipt-path for the dashboard receipt handoff.", "receipt_paths": [".dx/forge/docs/supabase-client.md", ".dx/forge/receipts/*-supabase-client.json", "examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json", ".dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json", "docs/packages/supabase-client.md"], "receipt_marker": "data-dx-supabase-receipt-path", "dx_check_visibility": {"schema": "dx.forge.package.dx_check_visibility", "current_status": "present", "statuses": ["present", "stale", "missing-receipt", "blocked", "unsupported-surface"], "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json"}, "app_owned_boundaries": ["Project provisioning, Auth redirects, profile/table RLS, provider credentials, and service-role secrets"]}),
        serde_json::json!({"package_id": "db/drizzle-sqlite", "role": "database", "command": "dx add db/drizzle --write", "env": [], "app_owned_boundaries": ["Migration policy, backups, database path, and driver"]}),
        serde_json::json!({
            "package_id": "instantdb/react",
            "official_name": "Realtime App Database",
            "upstream_package": "@instantdb/react",
            "upstream_version": "0.0.0",
            "aliases": ["@instantdb/react", "instantdb", "db/instantdb"],
            "role": "realtime-data",
            "command": "dx add instantdb/react --write",
            "env": ["NEXT_PUBLIC_INSTANT_APP_ID"],
            "required_env": ["NEXT_PUBLIC_INSTANT_APP_ID"],
            "source_mirror": "G:/WWW/inspirations/instantdb",
            "inspected_source_files": [
                "client/packages/react/package.json",
                "client/packages/react/src/index.ts",
                "client/packages/core/src/index.ts",
                "client/packages/react-common/src/InstantReactAbstractDatabase.tsx",
                "client/sandbox/react-nextjs/pages/play/sync-table.tsx"
            ],
            "selected_surfaces": [
                "realtime-todos",
                "presence-room",
                "auth-storage-streams",
                "sync-table-events",
                "dashboard-workflow"
            ],
            "honesty_label": "ADAPTER-BOUNDARY",
            "dx_check_visibility": {
                "schema": "dx.forge.package.dx_check_visibility",
                "current_status": "present",
                "statuses": ["present", "stale", "missing-receipt", "blocked", "unsupported-surface"],
                "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json",
                "monitored_surfaces": [
                    "instantdb-runtime-dashboard-workflow",
                    "dashboard-instantdb-workflow",
                    "sync-table-events"
                ]
            },
            "exported_files": [
                "lib/instant/env.ts",
                "lib/instant/schema.ts",
                "lib/instant/client.ts",
                "lib/instant/next-client.tsx",
                "lib/instant/next-server.ts",
                "lib/instant/queries.ts",
                "lib/instant/status.ts",
                "lib/instant/subscriptions.ts",
                "lib/instant/pagination.ts",
                "lib/instant/diagnostics.ts",
                "lib/instant/mutations.ts",
                "lib/instant/rules.ts",
                "lib/instant/perms.ts",
                "lib/instant/auth.ts",
                "lib/instant/oauth.ts",
                "lib/instant/storage.ts",
                "lib/instant/streams.ts",
                "lib/instant/sync-table.ts",
                "lib/instant/route.ts",
                "lib/instant/metadata.ts",
                "lib/instant/dashboard-workflow.ts",
                "components/instant/instant-todos.tsx",
                "components/instant/instant-cursors.tsx",
                "components/instant/instant-auth-boundary.tsx",
                "components/dashboard/instantdb-dashboard-workflow.tsx",
                "components/template-app/instantdb-status.tsx",
                "app/api/instant/route.ts",
                "app/instant/page.tsx",
                "examples/dashboard/src/lib/instantdbDashboard.ts",
                "examples/dashboard/src/components/InstantDbDashboardWorkflow.tsx"
            ],
            "dashboard_usage": "/launch uses LaunchInstantStatus as data-dx-component=\"launch-instantdb-dashboard-workflow\" and the runtime bridge as data-dx-component=\"instantdb-runtime-dashboard-workflow\" for realtime query readiness, room presence, missing NEXT_PUBLIC_INSTANT_APP_ID state, Sync Table source-event helpers, and a safe local schema receipt without a template-local node_modules workflow; dx add instantdb/react --write also materializes lib/instant/sync-table.ts, lib/instant/dashboard-workflow.ts, and components/dashboard/instantdb-dashboard-workflow.tsx for starter dashboards.",
            "receipt_paths": [".dx/forge/docs/instantdb-react.md", ".dx/forge/receipts/*-instantdb-react.json", "docs/packages/instantdb-react.md", "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json"],
            "dx_icon": "pack:database",
            "app_owned_boundaries": ["Instant dashboard app, rules, auth policy, production schema, file access rules, stream lifecycle, topic payload policy, experimental Sync Table subscriptions, local store retention, and NEXT_PUBLIC_INSTANT_APP_ID"]
        }),
        serde_json::json!({"package_id": "wasm/bindgen", "official_name": "WebAssembly Bridge", "upstream_package": "wasm-bindgen", "upstream_version": "0.2.121", "aliases": ["webassembly-bridge", "webassembly/bridge", "wasm-bindgen", "dx-forge/wasm-bindgen"], "role": "wasm", "command": "dx add webassembly-bridge --write", "env": [], "required_env": [], "source_mirror": "G:/WWW/inspirations/wasm-bindgen", "exported_files": ["wasm/bindgen/loader.ts", "wasm/bindgen/react.tsx", "wasm/bindgen/dashboard-workflow.tsx", "wasm/bindgen/metadata.ts", "examples/dashboard/src/components/WasmBindgenWorkflow.tsx", "examples/template/wasm-interop-status.tsx"], "receipt_paths": [".dx/forge/receipts/wasm-bindgen.json", ".dx/launch/receipts/wasm-bindgen-launch.json", "examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json"], "dx_check_visibility": {"schema": "dx.forge.package.dx_check_visibility", "current_status": "present", "statuses": ["present", "stale", "missing-receipt", "blocked", "unsupported-surface"], "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json"}, "honesty_label": "ADAPTER-BOUNDARY", "app_owned_boundaries": ["Rust crate, generated glue, wasm artifact, wasm-bindgen CLI output, and browser security review"]}),
        serde_json::json!({"package_id": "3d/launch-scene", "official_name": "3D Scene System", "upstream_package": "three + @react-three/fiber + @react-three/drei", "upstream_version": "three 0.184.0; @react-three/fiber 9.6.1; @react-three/drei local mirror", "aliases": ["3d-scene-system", "three-scene", "three/r3f/drei", "@react-three/fiber", "@react-three/drei", "spline-like-scene"], "role": "scene", "command": "dx add 3d-scene-system --write", "env": [], "source_mirror": "G:/WWW/inspirations/three.js; G:/WWW/inspirations/react-three-fiber; G:/WWW/inspirations/drei", "exported_files": ["components/scene/launch-scene.tsx", "lib/scene/dashboard-workflow.ts", "lib/scene/dashboard-controls.ts", "lib/scene/frame-sample.ts", "lib/scene/capability-report.ts", "lib/scene/viewport-report.ts", "lib/scene/bounds-report.ts", "lib/scene/raycast-report.ts", "lib/scene/preset.ts", "lib/scene/renderer-handoff.ts", "lib/scene/preview-readiness.ts", "lib/scene/r3f-renderer-adapter.ts", "lib/scene/metadata.ts"], "dashboard_usage": "/launch uses the 3D Scene System as LaunchScene and launch-scene-dashboard-workflow for scene node focus, preview/cinematic quality switching, source-owned material-palette and camera-rig cycling, local canvas frame sampling, renderer capability inspection, viewport DPR measurement, bounds-fit and raycast-hit reporting, performance regression/reset, render-budget receipt preparation, and Web Preview source markers without template-local node_modules.", "receipt_paths": ["docs/packages/3d-scene-system.md", ".dx/forge/receipts/3d-launch-scene.json", ".dx/forge/receipts/3d-launch-scene-dashboard-workflow.json"], "honesty_label": "SOURCE-ONLY", "app_owned_boundaries": ["3D assets, shader review, WebGL performance budgets, WebXR permissions, dependency installation, and any full Three/R3F/Drei renderer swap"]}),
        serde_json::json!({"package_id": "migration/static-site", "role": "migration", "command": "dx add migration/static-site --write", "env": [], "app_owned_boundaries": ["Source audit, route ownership, and rollback plan"]}),
    ]
}

fn forge_package_discovery_public_api(package_id: &str) -> Vec<&'static str> {
    match package_id {
        "shadcn/ui/button" => vec!["Button"],
        "shadcn/ui/badge" => vec!["Badge", "badgeVariants"],
        "shadcn/ui/card" => vec!["Card", "CardHeader", "CardContent"],
        "shadcn/ui/alert" => vec!["Alert", "AlertTitle", "AlertDescription"],
        "shadcn/ui/avatar" => vec!["Avatar", "AvatarImage", "AvatarFallback"],
        "shadcn/ui/skeleton" => vec!["Skeleton"],
        "shadcn/ui/label" => vec!["Label"],
        "shadcn/ui/separator" => vec!["Separator"],
        "shadcn/ui/field" => vec![
            "Field",
            "FieldSet",
            "FieldLegend",
            "FieldGroup",
            "FieldLabel",
            "FieldDescription",
            "FieldSeparator",
            "FieldError",
        ],
        "shadcn/ui/item" => vec![
            "Item",
            "ItemMedia",
            "ItemContent",
            "ItemActions",
            "ItemGroup",
            "ItemSeparator",
            "ItemTitle",
            "ItemDescription",
            "ItemHeader",
            "ItemFooter",
        ],
        "shadcn/ui/input" => vec!["Input"],
        "shadcn/ui/textarea" => vec!["Textarea"],
        "dx/icon/search" => vec!["SearchIcon"],
        "auth/better-auth" => vec!["createBetterAuthClient", "createGoogleOAuthProvider"],
        "animation/motion" => vec!["MotionReveal"],
        "i18n/next-intl" => vec![
            "DxIntlProvider",
            "loadDxMessages",
            "TemplateDashboardIntlWorkflow",
            "createDxDashboardIntlReceipt",
            "createDashboardIntlFormatPreview",
            "createDxDashboardIntlNumberPreview",
            "createDxDashboardIntlFormatPreview",
            "createDxDashboardIntlNumberPreview",
            "getDxDashboardLocaleAlternateLinks",
            "getDashboardLocaleAlternateLinks",
            "getDxDashboardLocaleRoutePreview",
            "getDashboardLocaleRoutePreview",
        ],
        "tanstack/query" => vec!["createDxQueryClient"],
        "validation/zod" => vec!["z"],
        "forms/react-hook-form" => vec!["DxHookForm", "DxInputField"],
        "payments/stripe-js" => vec![
            "getDxStripe",
            "confirmDxStripePayment",
            "submitDxStripeCheckoutContact",
            "createDxStripeDashboardCheckoutRequest",
            "createDxStripeDashboardMissingConfigReceipt",
            "createDxStripeCheckoutSession",
        ],
        "automations/n8n" => vec![
            "automationRoutes",
            "automationSummary",
            "connectorMetadata",
            "credentialMetadata",
            "normalizeDxN8nConnector",
            "filterDxN8nConnectors",
            "buildDxN8nCredentialReadiness",
            "requiredEnvForDxN8nConnector",
            "buildDxN8nWorkflowDraft",
            "createDxN8nRunReceipt",
            "LaunchAutomationBridgeStatus",
            "LaunchAutomationMissionSummary",
            "LaunchAutomationDashboardState",
        ],
        "state/zustand" => vec![
            "create",
            "createStore",
            "Mutate",
            "StoreMutators",
            "StoreMutatorIdentifier",
            "SubscribeWithSelector",
            "StoreSubscribeWithSelector",
            "WithSelectorSubscribe",
            "Write",
            "persist",
            "createJSONStorage",
            "PersistApi",
            "immer",
            "redux",
            "createWithEqualityFn",
            "unstable_ssrSafe",
            "devtools",
        ],
        "ai/vercel-ai" => vec![
            "DxAIClientChat",
            "streamText",
            "convertToModelMessages",
            "tool",
            "gateway",
            "createGateway",
            "createProviderRegistry",
            "LaunchAiChatStatus",
        ],
        "api/trpc" => vec![
            "initTRPC.context().create()",
            "fetchRequestHandler",
            "createDxTrpcRouteHandler",
            "createDxTrpcClient",
            "createDxTrpcStreamingClient",
            "createDxTrpcSubscriptionClient",
            "createDxTrpcServerCaller",
            "createDxTrpcResponseMeta",
            "createDxTrpcHttpLinkOptions",
            "TrpcDashboardWorkflow",
            "trpcLaunchContract",
            "createLocalHealthCheck",
            "createLocalLaunchEvent",
        ],
        "content/fumadocs-next" => vec![
            "DocsPage",
            "source",
            "dxFumadocsRouteContract",
            "dxFumadocsSourcePluginContract",
            "dxFumadocsNavigationContract",
            "getDxFumadocsNavigationSnapshot",
            "dxFumadocsTocContract",
            "getDxFumadocsPageTocSummary",
            "dxFumadocsLLMsContract",
            "createDxFumadocsLLMsIndex",
            "dxFumadocsOpenAPIContract",
            "dxFumadocsOpenAPICodeUsageContract",
            "dxFumadocsOpenAPI",
            "readDxFumadocsOpenAPIAllowedOrigins",
            "createDxFumadocsSearchApi",
            "dxFumadocsSearchClientContract",
        ],
        "content/react-markdown" => vec!["DxMarkdown"],
        "supabase/client" => vec![
            "readSupabasePublicConfig",
            "createDxSupabaseBrowserClient",
            "createDxSupabaseServerClient",
            "getDxSupabaseCurrentProfile",
            "upsertDxSupabaseProfile",
            "dxSupabaseForgePackage",
            "readDxSupabaseProfilesReadModel",
            "supabase-schema-query",
        ],
        "db/drizzle-sqlite" => vec![
            "createDxDrizzleConnection",
            "readDrizzleDashboardOverview",
            "readDrizzleDashboardQueryPlan",
        ],
        "instantdb/react" => vec![
            "createDxInstantClient",
            "subscribeInstantLaunchSyncTable",
            "SyncTableCallbackEventType",
        ],
        "wasm/bindgen" => vec![
            "WebAssembly Bridge",
            "#[wasm_bindgen]",
            "useWasmBindgenModule",
            "WasmBindgenFactory",
            "default init(input)",
            "initSync(input)",
        ],
        "3d/launch-scene" => vec![
            "createDxLaunchScenePreset",
            "dxSceneQualityProfiles",
            "dxSceneMaterialPalettes",
            "resolveDxSceneMaterialPalette",
            "mountDxSceneWithRenderer",
            "createDxSceneDashboardWorkflow",
            "createDxSceneDashboardReceipt",
            "dxSceneDashboardCameraRigs",
            "resolveDxSceneDashboardCameraRig",
        ],
        "migration/static-site" => vec!["staticRouteManifest"],
        _ => Vec::new(),
    }
}

fn launch_companion_doc_receipts() -> Vec<serde_json::Value> {
    vec![
        launch_companion_doc_receipt(
            "auth/better-auth",
            "auth",
            "auth-session-status",
            "dx add better-auth --write",
            "examples/template/auth-session-status.tsx",
            "components/template-app/auth-session-status.tsx",
            ".dx/forge/docs/auth-better-auth.md",
            "LaunchAuthSessionStatus",
            &["useSession", "signOut", "createBetterAuthClient"],
        ),
        launch_companion_doc_receipt(
            "ai/vercel-ai",
            "ai",
            "ai-chat-status",
            "dx add ai-sdk --write",
            "examples/template/ai-chat-status.tsx",
            "components/template-app/ai-chat-status.tsx",
            ".dx/forge/docs/ai-vercel-ai.md",
            "LaunchAiChatStatus",
            &[
                "DxAIClientChat",
                "LaunchAiChatStatus",
                "streamText",
                "convertToModelMessages",
                "tool",
                "gateway",
                "createProviderRegistry",
            ],
        ),
        launch_companion_doc_receipt(
            "db/drizzle-sqlite",
            "database",
            "drizzle-query-proof",
            "dx add db/drizzle --write",
            "examples/template/drizzle-query-proof.tsx",
            "components/template-app/drizzle-query-proof.tsx",
            ".dx/forge/docs/db-drizzle-sqlite.md",
            "LaunchDrizzleDashboardData",
            &[
                "dxDrizzlePackage",
                "readDrizzleDashboardOverview",
                "readDrizzleDashboardQueryPlan",
                "mission-control database card",
            ],
        ),
        launch_companion_doc_receipt(
            "supabase/client",
            "database",
            "supabase-profile-workflow",
            "dx add supabase/client --write",
            "examples/template/supabase-profile-workflow.tsx",
            "components/template-app/supabase-profile-workflow.tsx",
            ".dx/forge/docs/supabase-client.md",
            "LaunchSupabaseProfileWorkflow",
            &[
                "readSupabasePublicConfig",
                "getDxSupabaseCurrentProfile",
                "upsertDxSupabaseProfile",
                "readDxSupabaseProfileConfigStatus",
                "createDxSupabaseProfilePreview",
                "createDxSupabaseProfileUpsertReceipt",
            ],
        ),
        launch_companion_doc_receipt(
            "payments/stripe-js",
            "payments",
            "payments-status",
            "dx add payments --write",
            "examples/template/payments-status.tsx",
            "components/template-app/payments-status.tsx",
            ".dx/forge/docs/payments-stripe-js.md",
            "LaunchPaymentStatus",
            &[
                "readDxStripeClientConfig",
                "submitDxStripeCheckoutContact",
                "createDxStripeCheckoutSession",
            ],
        ),
        launch_companion_doc_receipt(
            "content/fumadocs-next",
            "docs",
            "docs-status",
            "dx add content/fumadocs-next --write",
            "examples/template/docs-status.tsx",
            "components/template-app/docs-status.tsx",
            ".dx/forge/docs/content-fumadocs-next.md",
            "LaunchDocsStatus",
            &[
                "dxFumadocsRouteContract",
                "dxFumadocsSourcePluginContract",
                "dxFumadocsNavigationContract",
                "dxFumadocsTocContract",
                "dxFumadocsOpenAPIContract",
                "dxFumadocsOpenAPICodeUsageContract",
                "openApiProxyRoute",
                "DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS",
                "DxMarkdown",
            ],
        ),
        launch_companion_doc_receipt(
            "validation/zod",
            "validation",
            "validation-status",
            "dx add zod --write",
            "examples/template/zod-validation-status.tsx",
            "components/template-app/zod-validation-status.tsx",
            ".dx/forge/docs/validation-zod.md",
            "LaunchZodValidationStatus",
            &[
                "dxLaunchSignupSchemaWithMetadata",
                "validateDxInput",
                "dxToJsonSchema",
                "readDxLaunchSchemaMetadata",
                "parseDxLaunchEnvFlags",
                "parseDxLaunchRoutePath",
                "safeParseDxLaunchAssetFile",
                "parseDxLaunchScoreInput",
                "summarizeDxLaunchPackageCatalog",
                "safeParseDxLaunchApprovalGate",
                "formatDxLaunchApprovalIssues",
                "configureDxZodEnglishLocale",
                "safeParseDxLaunchSignupForDisplay",
                "parseDxLaunchSearchParams",
                "safeParseDxLaunchSignupSubmission",
                "safeParseDxLaunchExternalPackage",
            ],
        ),
        launch_companion_doc_receipt(
            "instantdb/react",
            "realtime-data",
            "realtime-data-status",
            "dx add instantdb/react --write",
            "examples/template/instantdb-status.tsx",
            "components/template-app/instantdb-status.tsx",
            ".dx/forge/docs/instantdb-react.md",
            "LaunchInstantStatus",
            &["createDxInstantClient", "db", "launchRoom"],
        ),
        launch_companion_doc_receipt(
            "i18n/next-intl",
            "i18n",
            "next-intl-dashboard-locale",
            "dx add next-intl --write",
            "examples/template/next-intl-dashboard-locale.tsx",
            "components/template-app/next-intl-dashboard-locale.tsx",
            ".dx/forge/docs/i18n-next-intl.md",
            "TemplateDashboardIntlWorkflow",
            &[
                "useTranslations",
                "useLocale",
                "useFormatter",
                "createDxDashboardIntlReceipt",
            ],
        ),
        launch_companion_doc_receipt(
            "tanstack/query",
            "server-state",
            "query-cache-status",
            "dx add data-fetching-cache --write",
            "examples/template/query-cache-status.tsx",
            "components/template-app/query-cache-status.tsx",
            ".dx/forge/docs/tanstack-query.md",
            "LaunchQueryCacheStatus",
            &["dxQueryOptions", "useQuery", "createDxQueryClient"],
        ),
        launch_companion_doc_receipt(
            "wasm/bindgen",
            "wasm",
            "wasm-interop-status",
            "dx add webassembly-bridge --write",
            "examples/template/wasm-interop-status.tsx",
            "components/template-app/wasm-interop-status.tsx",
            ".dx/forge/docs/wasm-bindgen.md",
            "LaunchWasmInteropStatus",
            &[
                "useWasmBindgenModule",
                "WasmBindgenFactory",
                "default init(input)",
            ],
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn launch_companion_doc_receipt(
    package_id: &str,
    role: &str,
    kind: &str,
    command: &str,
    source_file: &str,
    materialized_file: &str,
    docs_file: &str,
    proof_export: &str,
    public_api: &[&str],
) -> serde_json::Value {
    serde_json::json!({
        "package_id": package_id,
        "role": role,
        "kind": kind,
        "command": command,
        "source_file": source_file,
        "materialized_file": materialized_file,
        "docs_file": docs_file,
        "proof_export": proof_export,
        "public_api": public_api,
        "open_files": [
            {
                "kind": "source-proof",
                "path": source_file
            },
            {
                "kind": "materialized-proof",
                "path": materialized_file
            },
            {
                "kind": "package-doc",
                "path": docs_file
            }
        ],
        "no_execution": true
    })
}

fn launch_companion_doc_receipts_contract() -> serde_json::Value {
    let companions = launch_companion_doc_receipts();
    let companion_count = companions.len();
    serde_json::json!({
        "schema": "dx.launch.companion_doc_receipts",
        "owner": "dx-www",
        "template_id": "next-familiar-www-template",
        "route": "/",
        "file": NEXT_FAMILIAR_LAUNCH_COMPANION_DOC_RECEIPTS_FILE,
        "companion_count": companion_count,
        "companions": companions,
        "open_file_kinds": ["source-proof", "materialized-proof", "package-doc"],
        "no_execution": true
    })
}

fn launch_companion_receipts_contract() -> serde_json::Value {
    let receipts = launch_companion_doc_receipts()
        .into_iter()
        .map(|receipt| {
            let kind = receipt["kind"].as_str().unwrap_or("unknown");
            let docs_file = launch_companion_docs_file(kind);
            serde_json::json!({
                "package_id": receipt["package_id"],
                "role": receipt["role"],
                "kind": receipt["kind"],
                "command": receipt["command"],
                "source_file": receipt["source_file"],
                "materialized_file": receipt["materialized_file"],
                "docs_file": docs_file.clone(),
                "package_docs_file": receipt["docs_file"],
                "proof_export": receipt["proof_export"],
                "public_api": receipt["public_api"],
                "open_files": [
                    {
                        "kind": "source-proof",
                        "path": receipt["source_file"]
                    },
                    {
                        "kind": "materialized-proof",
                        "path": receipt["materialized_file"]
                    },
                    {
                        "kind": "companion-doc",
                        "path": docs_file
                    },
                    {
                        "kind": "package-doc",
                        "path": receipt["docs_file"]
                    }
                ],
                "no_execution": true
            })
        })
        .collect::<Vec<_>>();
    let receipt_count = receipts.len();
    serde_json::json!({
        "schema": "dx.launch.companion_receipts",
        "owner": "dx-www",
        "template_id": "next-familiar-www-template",
        "route": "/",
        "receipt_source": NEXT_FAMILIAR_LAUNCH_COMPANION_DOC_RECEIPTS_FILE,
        "receipt_count": receipt_count,
        "receipts": receipts,
        "open_file_kinds": ["source-proof", "materialized-proof", "companion-doc", "package-doc"],
        "no_execution": true
    })
}

fn launch_runtime_approval_request_contract() -> serde_json::Value {
    serde_json::json!({
        "schema": "dx.launch.runtime_approval_request",
        "owner": "dx-www",
        "template_id": "next-familiar-www-template",
        "route": "/",
        "file": NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE,
        "command": "dx forge launch-runtime-approval-request --project <path> --json",
        "approval_record": {
            "status": "pending-explicit-approval",
            "approved": false,
            "approved_by": null,
            "approved_at": null,
            "approval_note": null,
            "scope": "build-preview-runtime-evidence"
        },
        "requested_commands": [
            {
                "id": "runtime-build",
                "command": "dx build",
                "approved": false,
                "requires_explicit_permission": true,
                "approval_status": "pending",
                "expected_evidence": "production-contract-route-proof"
            },
            {
                "id": "production-preview",
                "command": "dx preview --production-contract",
                "approved": false,
                "requires_explicit_permission": true,
                "approval_status": "pending",
                "expected_evidence": "governed-runtime-route-response"
            }
        ],
        "requested_evidence": {
            "items": [
                "governed-runtime-route-response",
                "production-contract-route-proof",
                "final-launch-evidence-receipt"
            ],
            "receipt": NEXT_FAMILIAR_LAUNCH_RUNTIME_EVIDENCE_FILE
        },
        "source_artifacts": [
            ".dx/forge/template-.dx/build-cache/manifest.json",
            NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE,
            NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE,
            NEXT_FAMILIAR_LAUNCH_RUNTIME_CHECKLIST_FILE
        ],
        "blocked_until_approved": true,
        "no_execution": true
    })
}

fn launch_runtime_evidence_contract() -> serde_json::Value {
    serde_json::json!({
        "schema": "dx.launch.runtime_evidence",
        "owner": "dx-www",
        "template_id": "next-familiar-www-template",
        "route": "/",
        "file": NEXT_FAMILIAR_LAUNCH_RUNTIME_EVIDENCE_FILE,
        "command": "dx forge launch-runtime-evidence --project <path> --json",
        "import_plan_command": "dx forge launch-runtime-evidence-import-plan --project <path> --build-log <path> --route-response <path> --preview-proof <path> --json",
        "completeness_command": "dx forge launch-runtime-evidence-completeness --project <path> --import-plan <path> --json",
        "finalization_command": "dx forge launch-runtime-evidence-finalize --project <path> --import-plan <path> --write --json",
        "review_command": "dx forge launch-runtime-evidence-review --project <path> --json",
        "packet_command": "dx forge launch-evidence-packet --project <path> --json",
        "operator_index_command": "dx forge launch-evidence-operator-index --project <path> --json",
        "status_timeline_command": "dx forge launch-evidence-status-timeline --project <path> --json",
        "handoff_digest_command": "dx forge launch-evidence-handoff-digest --project <path> --write",
        "release_checklist_command": "dx forge launch-evidence-release-checklist --project <path> --write",
        "share_manifest_command": "dx forge launch-evidence-share-manifest --project <path> --write",
        "archive_index_command": "dx forge launch-evidence-archive-index --project <path> --write",
        "archive_receipt_command": "dx forge launch-evidence-archive-receipt --project <path> --write",
        "finalization_receipt": NEXT_FAMILIAR_LAUNCH_RUNTIME_FINALIZATION_RECEIPT_FILE,
        "review_report": NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE,
        "packet_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
        "operator_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE,
        "status_timeline": NEXT_FAMILIAR_LAUNCH_EVIDENCE_STATUS_TIMELINE_FILE,
        "handoff_digest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
        "release_checklist": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE,
        "share_manifest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE,
        "archive_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE,
        "archive_receipt": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_RECEIPT_FILE,
        "status": "awaiting-approved-runtime-run",
        "fake_proof": false,
        "approval_required": true,
        "approval_request": NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE,
        "runtime_checklist": NEXT_FAMILIAR_LAUNCH_RUNTIME_CHECKLIST_FILE,
        "required_evidence": [
            {
                "id": "governed-runtime-route-response",
                "kind": "route-response",
                "status": "not-collected",
                "required": true,
                "artifact_path": ".dx/forge/runtime/launch-route-response.json",
                "source_command": "dx preview --production-contract"
            },
            {
                "id": "production-contract-route-proof",
                "kind": "production-preview",
                "status": "not-collected",
                "required": true,
                "artifact_path": ".dx/forge/runtime/production-contract-route-proof.json",
                "source_command": "dx build"
            },
            {
                "id": "final-launch-evidence-receipt",
                "kind": "final-receipt",
                "status": "not-collected",
                "required": true,
                "artifact_path": NEXT_FAMILIAR_LAUNCH_RUNTIME_FINALIZATION_RECEIPT_FILE,
                "source_command": "dx forge launch-runtime-evidence-finalize --project <path> --import-plan <path> --write --json"
            }
        ],
        "collected_evidence": {
            "present": 0,
            "artifacts": []
        },
        "source_artifacts": [
            ".dx/forge/template-.dx/build-cache/manifest.json",
            NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE,
            NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE,
            NEXT_FAMILIAR_LAUNCH_RUNTIME_CHECKLIST_FILE,
            NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE
        ],
        "blocked_until_approved": true,
        "no_execution": true
    })
}

fn launch_runtime_checklist_contract() -> serde_json::Value {
    serde_json::json!({
        "schema": "dx.launch.runtime_checklist",
        "owner": "dx-www",
        "template_id": "next-familiar-www-template",
        "route": "/",
        "file": NEXT_FAMILIAR_LAUNCH_RUNTIME_CHECKLIST_FILE,
        "command": "dx forge launch-runtime-checklist --project <path> --json",
        "approval": {
            "status": "requires-explicit-permission",
            "requires_explicit_permission": true,
            "default_action": "skip-runtime-build-preview"
        },
        "commands": [
            {
                "id": "build",
                "command": "dx build",
                "purpose": "compile the generated starter before production preview",
                "requires_explicit_approval": true,
                "default_action": "skip",
                "expected_evidence": "production-contract-build-log"
            },
            {
                "id": "production-preview",
                "command": "dx preview --production-contract",
                "purpose": "serve the compiled launch route through the production contract preview",
                "requires_explicit_approval": true,
                "default_action": "skip",
                "expected_evidence": "production-contract-route-proof"
            }
        ],
        "expected_evidence": [
            "governed-runtime-route-response",
            "production-contract-route-proof",
            "final-launch-evidence-receipt"
        ],
        "approval_request": {
            "schema": "dx.launch.runtime_approval_request",
            "file": NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE,
            "command": "dx forge launch-runtime-approval-request --project <path> --json"
        },
        "runtime_evidence": {
            "schema": "dx.launch.runtime_evidence",
            "file": NEXT_FAMILIAR_LAUNCH_RUNTIME_EVIDENCE_FILE,
            "command": "dx forge launch-runtime-evidence --project <path> --json",
            "status": "awaiting-approved-runtime-run"
        },
        "blocked_without_permission": ["dev-server", "full-build", "production-preview"],
        "no_execution": true
    })
}

fn launch_verification_lane_contract() -> serde_json::Value {
    serde_json::json!({
        "schema": "dx.launch.verification_lane",
        "owner": "dx-www",
        "template_id": "next-familiar-www-template",
        "route": "/",
        "file": NEXT_FAMILIAR_LAUNCH_VERIFICATION_LANE_FILE,
        "command": "dx forge launch-verification-lane --project <path> --json",
        "lane_id": "governed-runtime-verification",
        "label": "Launch runtime verification lane",
        "operator_focus": "review-checklist-record-approval-collect-evidence",
        "requires_explicit_permission": true,
        "runtime_approved": false,
        "runtime_artifacts": {
            "checklist": NEXT_FAMILIAR_LAUNCH_RUNTIME_CHECKLIST_FILE,
            "approval_request": NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE,
            "runtime_evidence": NEXT_FAMILIAR_LAUNCH_RUNTIME_EVIDENCE_FILE,
            "finalization_receipt": NEXT_FAMILIAR_LAUNCH_RUNTIME_FINALIZATION_RECEIPT_FILE
        },
        "operator_steps": [
            {
                "id": "review-runtime-checklist",
                "label": "Review runtime checklist",
                "file": NEXT_FAMILIAR_LAUNCH_RUNTIME_CHECKLIST_FILE,
                "command": "dx forge launch-runtime-checklist --project <path> --json",
                "status": "requires-explicit-permission"
            },
            {
                "id": "record-runtime-approval",
                "label": "Record explicit runtime approval",
                "file": NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE,
                "command": "dx forge launch-runtime-approval-request --project <path> --json",
                "status": "pending-explicit-approval"
            },
            {
                "id": "collect-runtime-evidence",
                "label": "Collect approved runtime evidence",
                "file": NEXT_FAMILIAR_LAUNCH_RUNTIME_EVIDENCE_FILE,
                "command": "dx forge launch-runtime-evidence --project <path> --json",
                "status": "awaiting-approved-runtime-run"
            },
            {
                "id": "finalize-runtime-evidence",
                "label": "Finalize complete runtime evidence",
                "file": NEXT_FAMILIAR_LAUNCH_RUNTIME_FINALIZATION_RECEIPT_FILE,
                "command": "dx forge launch-runtime-evidence-finalize --project <path> --import-plan <path> --write --json",
                "status": "blocked-until-completeness-passes"
            },
            {
                "id": "review-final-runtime-evidence",
                "label": "Review final runtime evidence",
                "file": NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE,
                "command": "dx forge launch-runtime-evidence-review --project <path> --json",
                "status": "blocked-until-final-receipt"
            },
            {
                "id": "packet-launch-evidence",
                "label": "Packet launch evidence for handoff",
                "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
                "command": "dx forge launch-evidence-packet --project <path> --json",
                "status": "blocked-until-final-review"
            },
            {
                "id": "index-launch-evidence",
                "label": "Index launch evidence operator status",
                "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE,
                "command": "dx forge launch-evidence-operator-index --project <path> --json",
                "status": "available-after-template-materialization"
            },
            {
                "id": "timeline-launch-evidence",
                "label": "Summarize launch evidence status timeline",
                "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_STATUS_TIMELINE_FILE,
                "command": "dx forge launch-evidence-status-timeline --project <path> --json",
                "status": "available-after-template-materialization"
            },
            {
                "id": "digest-launch-evidence",
                "label": "Write launch evidence handoff digest",
                "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
                "command": "dx forge launch-evidence-handoff-digest --project <path> --write",
                "status": "blocked-until-status-timeline"
            },
            {
                "id": "checklist-launch-evidence",
                "label": "Write launch evidence release checklist",
                "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE,
                "command": "dx forge launch-evidence-release-checklist --project <path> --write",
                "status": "blocked-until-handoff-digest"
            },
            {
                "id": "share-launch-evidence",
                "label": "Write launch evidence share manifest",
                "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE,
                "command": "dx forge launch-evidence-share-manifest --project <path> --write",
                "status": "blocked-until-release-checklist"
            },
            {
                "id": "archive-launch-evidence",
                "label": "Write launch evidence archive index",
                "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE,
                "command": "dx forge launch-evidence-archive-index --project <path> --write",
                "status": "blocked-until-share-manifest"
            },
            {
                "id": "receipt-launch-evidence-archive",
                "label": "Write launch evidence archive receipt",
                "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_RECEIPT_FILE,
                "command": "dx forge launch-evidence-archive-receipt --project <path> --write",
                "status": "blocked-until-archive-index"
            }
        ],
        "open_files": [
            {
                "kind": "runtime-checklist",
                "path": NEXT_FAMILIAR_LAUNCH_RUNTIME_CHECKLIST_FILE
            },
            {
                "kind": "runtime-approval-request",
                "path": NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE
            },
            {
                "kind": "runtime-evidence",
                "path": NEXT_FAMILIAR_LAUNCH_RUNTIME_EVIDENCE_FILE
            },
            {
                "kind": "runtime-evidence-finalization-receipt",
                "path": NEXT_FAMILIAR_LAUNCH_RUNTIME_FINALIZATION_RECEIPT_FILE
            },
            {
                "kind": "runtime-evidence-review",
                "path": NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE
            },
            {
                "kind": "launch-evidence-packet",
                "path": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE
            },
            {
                "kind": "launch-evidence-operator-index",
                "path": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE
            },
            {
                "kind": "launch-evidence-status-timeline",
                "path": NEXT_FAMILIAR_LAUNCH_EVIDENCE_STATUS_TIMELINE_FILE
            },
            {
                "kind": "launch-evidence-handoff-digest",
                "path": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE
            },
            {
                "kind": "launch-evidence-release-checklist",
                "path": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE
            },
            {
                "kind": "launch-evidence-share-manifest",
                "path": NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE
            },
            {
                "kind": "launch-evidence-archive-index",
                "path": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE
            },
            {
                "kind": "launch-evidence-archive-receipt",
                "path": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_RECEIPT_FILE
            }
        ],
        "blocked_without_permission": ["dev-server", "full-build", "production-preview"],
        "no_execution": true
    })
}

fn launch_companion_docs_file(kind: &str) -> String {
    format!(".dx/forge/docs/launch-companions/{kind}.md")
}

fn launch_zed_template_handoff_contract() -> serde_json::Value {
    let primary_route = DEFAULT_TEMPLATE_APP_ROUTE_SOURCES[0];

    serde_json::json!({
        "schema": "dx.zed.template_handoff",
        "handoff_kind": "app-router-template",
        "owner": "dx-www",
        "template_id": "next-familiar-www-template",
        "architecture_contract": default_www_template_architecture_contract(),
        "file": NEXT_FAMILIAR_LAUNCH_ZED_TEMPLATE_HANDOFF_FILE,
        "materialized_file": NEXT_FAMILIAR_LAUNCH_ZED_TEMPLATE_HANDOFF_FILE,
        "route": primary_route.route,
        "route_aliases": primary_route.aliases,
        "entrypoint_file": primary_route.materialized_file,
        "source_entrypoint_file": primary_route.source_file,
        "entrypoint_role": primary_route.role,
        "readiness_receipt": NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE,
        "readiness_bundle": NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE,
        "launch_verification_lane": NEXT_FAMILIAR_LAUNCH_VERIFICATION_LANE_FILE,
        "source_smoke_command": "dx run --test .\\benchmarks\\template-shell.test.ts",
        "safe_source_checks": [
            "dx run --test .\\benchmarks\\template-shell.test.ts",
            "rustfmt --edition 2024 --check dx-www\\src\\cli\\mod.rs",
            "git diff --check"
        ],
        "open_files": [
            {
                "kind": "route",
                "path": "app/page.tsx"
            },
            {
                "kind": "route-contract",
                "path": "components/template-app/template-route-contract.ts"
            },
            {
                "kind": "template-shell",
                "path": "components/template-app/template-shell.tsx"
            },
            {
                "kind": "package-catalog",
                "path": "components/template-app/package-catalog.ts"
            },
            {
                "kind": "template-surface-registry",
                "path": "components/template-app/template-surface-registry.ts"
            },
            {
                "kind": "framework-completeness",
                "path": "components/template-app/framework-completeness.ts"
            },
            {
                "kind": "readiness-receipt",
                "path": NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE
            },
            {
                "kind": "zed-template-handoff",
                "path": NEXT_FAMILIAR_LAUNCH_ZED_TEMPLATE_HANDOFF_FILE
            },
            {
                "kind": "readiness-bundle",
                "path": NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE
            },
            {
                "kind": "companion-doc-receipts",
                "path": NEXT_FAMILIAR_LAUNCH_COMPANION_DOC_RECEIPTS_FILE
            },
            {
                "kind": "runtime-checklist",
                "path": NEXT_FAMILIAR_LAUNCH_RUNTIME_CHECKLIST_FILE
            },
            {
                "kind": "runtime-approval-request",
                "path": NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE
            },
            {
                "kind": "runtime-evidence",
                "path": NEXT_FAMILIAR_LAUNCH_RUNTIME_EVIDENCE_FILE
            },
            {
                "kind": "runtime-evidence-finalization-receipt",
                "path": NEXT_FAMILIAR_LAUNCH_RUNTIME_FINALIZATION_RECEIPT_FILE
            },
            {
                "kind": "launch-verification-lane",
                "path": NEXT_FAMILIAR_LAUNCH_VERIFICATION_LANE_FILE
            }
        ],
        "runtime_actions": {
            "requires_explicit_permission": true,
            "launch_verification_lane": NEXT_FAMILIAR_LAUNCH_VERIFICATION_LANE_FILE,
            "approval_request": NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE,
            "runtime_evidence": NEXT_FAMILIAR_LAUNCH_RUNTIME_EVIDENCE_FILE,
            "runtime_evidence_import_plan": "dx forge launch-runtime-evidence-import-plan --project <path> --build-log <path> --route-response <path> --preview-proof <path> --json",
            "runtime_evidence_completeness": "dx forge launch-runtime-evidence-completeness --project <path> --import-plan <path> --json",
            "runtime_evidence_finalization": "dx forge launch-runtime-evidence-finalize --project <path> --import-plan <path> --write --json",
            "runtime_evidence_finalization_receipt": NEXT_FAMILIAR_LAUNCH_RUNTIME_FINALIZATION_RECEIPT_FILE,
            "runtime_evidence_review": "dx forge launch-runtime-evidence-review --project <path> --json",
            "runtime_evidence_review_report": NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE,
            "launch_evidence_packet": "dx forge launch-evidence-packet --project <path> --json",
            "launch_evidence_packet_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "launch_evidence_operator_index": "dx forge launch-evidence-operator-index --project <path> --json",
            "launch_evidence_operator_index_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE,
            "launch_evidence_status_timeline": "dx forge launch-evidence-status-timeline --project <path> --json",
            "launch_evidence_status_timeline_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_STATUS_TIMELINE_FILE,
            "launch_evidence_handoff_digest": "dx forge launch-evidence-handoff-digest --project <path> --write",
            "launch_evidence_handoff_digest_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
            "launch_evidence_release_checklist": "dx forge launch-evidence-release-checklist --project <path> --write",
            "launch_evidence_release_checklist_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE,
            "launch_evidence_share_manifest": "dx forge launch-evidence-share-manifest --project <path> --write",
            "launch_evidence_share_manifest_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE,
            "launch_evidence_archive_index": "dx forge launch-evidence-archive-index --project <path> --write",
            "launch_evidence_archive_index_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE,
            "launch_evidence_archive_receipt": "dx forge launch-evidence-archive-receipt --project <path> --write",
            "launch_evidence_archive_receipt_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_RECEIPT_FILE,
            "launch_evidence_archive_ledger": "dx forge launch-evidence-archive-ledger --project <path> --write",
            "launch_evidence_archive_ledger_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_LEDGER_FILE,
            "launch_evidence_retention_policy": "dx forge launch-evidence-retention-policy --project <path> --write",
            "launch_evidence_retention_policy_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_POLICY_FILE,
            "launch_evidence_retention_review": "dx forge launch-evidence-retention-review --project <path> --write",
            "launch_evidence_retention_review_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_REVIEW_FILE,
            "launch_evidence_release_seal": "dx forge launch-evidence-release-seal --project <path> --write",
            "launch_evidence_release_seal_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE,
            "launch_evidence_operator_summary": "dx forge launch-evidence-operator-summary --project <path> --write",
            "launch_evidence_operator_summary_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_SUMMARY_FILE,
            "launch_evidence_completion_ledger": "dx forge launch-evidence-completion-ledger --project <path> --write",
            "launch_evidence_completion_ledger_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_COMPLETION_LEDGER_FILE,
            "launch_evidence_closure_memo": "dx forge launch-evidence-closure-memo --project <path> --write",
            "launch_evidence_closure_memo_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_CLOSURE_MEMO_FILE,
            "launch_evidence_final_brief": "dx forge launch-evidence-final-brief --project <path> --write",
            "launch_evidence_final_brief_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_FINAL_BRIEF_FILE,
            "launch_evidence_operator_runbook": "dx forge launch-evidence-operator-runbook --project <path> --write",
            "launch_evidence_operator_runbook_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RUNBOOK_FILE,
            "launch_evidence_handoff_capsule": "dx forge launch-evidence-handoff-capsule --project <path> --write",
            "launch_evidence_handoff_capsule_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_CAPSULE_FILE,
            "launch_evidence_resumption_index": "dx forge launch-evidence-resumption-index --project <path> --write",
            "launch_evidence_resumption_index_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESUMPTION_INDEX_FILE,
            "launch_evidence_recovery_brief": "dx forge launch-evidence-recovery-brief --project <path> --write",
            "launch_evidence_recovery_brief_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RECOVERY_BRIEF_FILE,
            "launch_evidence_continuation_packet": "dx forge launch-evidence-continuation-packet --project <path> --write",
            "launch_evidence_continuation_packet_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_CONTINUATION_PACKET_FILE,
            "launch_evidence_operator_resume_card": "dx forge launch-evidence-operator-resume-card --project <path> --write",
            "launch_evidence_operator_resume_card_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RESUME_CARD_FILE,
            "launch_evidence_restart_ledger": "dx forge launch-evidence-restart-ledger --project <path> --write",
            "launch_evidence_restart_ledger_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_LEDGER_FILE,
            "launch_evidence_restart_checklist": "dx forge launch-evidence-restart-checklist --project <path> --write",
            "launch_evidence_restart_checklist_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CHECKLIST_FILE,
            "launch_evidence_restart_brief": "dx forge launch-evidence-restart-brief --project <path> --write",
            "launch_evidence_restart_brief_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_BRIEF_FILE,
            "launch_evidence_restart_manifest": "dx forge launch-evidence-restart-manifest --project <path> --write",
            "launch_evidence_restart_manifest_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_MANIFEST_FILE,
            "launch_evidence_restart_receipt": "dx forge launch-evidence-restart-receipt --project <path> --write",
            "launch_evidence_restart_receipt_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_RECEIPT_FILE,
            "launch_evidence_restart_summary": "dx forge launch-evidence-restart-summary --project <path> --write",
            "launch_evidence_restart_summary_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SUMMARY_FILE,
            "launch_evidence_restart_snapshot": "dx forge launch-evidence-restart-snapshot --project <path> --write",
            "launch_evidence_restart_snapshot_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SNAPSHOT_FILE,
            "launch_evidence_restart_dispatch": "dx forge launch-evidence-restart-dispatch --project <path> --write",
            "launch_evidence_restart_dispatch_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_DISPATCH_FILE,
            "launch_evidence_restart_closeout": "dx forge launch-evidence-restart-closeout --project <path> --write",
            "launch_evidence_restart_closeout_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CLOSEOUT_FILE,
            "launch_evidence_restart_signoff": "dx forge launch-evidence-restart-signoff --project <path> --write",
            "launch_evidence_restart_signoff_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SIGNOFF_FILE,
            "launch_evidence_acceptance_index": "dx forge launch-evidence-acceptance-index --project <path> --write",
            "launch_evidence_acceptance_index_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ACCEPTANCE_INDEX_FILE,
            "launch_evidence_acceptance_digest": "dx forge launch-evidence-acceptance-digest --project <path> --write",
            "launch_evidence_acceptance_digest_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ACCEPTANCE_DIGEST_FILE,
            "launch_evidence_friday_baton": "dx forge launch-evidence-friday-baton --project <path> --write",
            "launch_evidence_friday_baton_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_FRIDAY_BATON_FILE,
            "blocked_without_permission": ["dev-server", "full-build", "production-preview"]
        },
        "no_execution": true
    })
}

fn launch_readiness_bundle_contract() -> serde_json::Value {
    serde_json::json!({
        "schema": "dx.launch.readiness_bundle",
        "owner": "dx-www",
        "template_id": "next-familiar-www-template",
        "architecture_contract": default_www_template_architecture_contract(),
        "route": "/",
        "file": NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE,
        "materialized_file": NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE,
        "metadata_commands": {
            "templates": "dx templates --json",
            "template_readiness": "dx templates verify-readiness --project <path> --json",
            "forge_template_readiness": "dx forge template-readiness --project <path> --json",
            "launch_adoption_report": "dx forge launch-adoption-report --project <path> --json",
            "launch_manifest_drift": "dx forge launch-manifest-drift --project <path> --json",
            "launch_companion_receipts": "dx forge launch-companion-receipts --project <path> --json",
            "launch_runtime_checklist": "dx forge launch-runtime-checklist --project <path> --json",
            "launch_runtime_approval_request": "dx forge launch-runtime-approval-request --project <path> --json",
            "launch_runtime_evidence": "dx forge launch-runtime-evidence --project <path> --json",
            "launch_runtime_evidence_import_plan": "dx forge launch-runtime-evidence-import-plan --project <path> --build-log <path> --route-response <path> --preview-proof <path> --json",
            "launch_runtime_evidence_completeness": "dx forge launch-runtime-evidence-completeness --project <path> --import-plan <path> --json",
            "launch_runtime_evidence_finalization": "dx forge launch-runtime-evidence-finalize --project <path> --import-plan <path> --write --json",
            "launch_runtime_evidence_review": "dx forge launch-runtime-evidence-review --project <path> --json",
            "launch_evidence_packet": "dx forge launch-evidence-packet --project <path> --json",
            "launch_evidence_operator_index": "dx forge launch-evidence-operator-index --project <path> --json",
            "launch_evidence_status_timeline": "dx forge launch-evidence-status-timeline --project <path> --json",
            "launch_evidence_handoff_digest": "dx forge launch-evidence-handoff-digest --project <path> --write",
            "launch_evidence_release_checklist": "dx forge launch-evidence-release-checklist --project <path> --write",
            "launch_evidence_share_manifest": "dx forge launch-evidence-share-manifest --project <path> --write",
            "launch_evidence_archive_index": "dx forge launch-evidence-archive-index --project <path> --write",
            "launch_evidence_archive_receipt": "dx forge launch-evidence-archive-receipt --project <path> --write",
            "launch_evidence_archive_ledger": "dx forge launch-evidence-archive-ledger --project <path> --write",
            "launch_evidence_retention_policy": "dx forge launch-evidence-retention-policy --project <path> --write",
            "launch_evidence_retention_review": "dx forge launch-evidence-retention-review --project <path> --write",
            "launch_evidence_release_seal": "dx forge launch-evidence-release-seal --project <path> --write",
            "launch_evidence_operator_summary": "dx forge launch-evidence-operator-summary --project <path> --write",
            "launch_evidence_completion_ledger": "dx forge launch-evidence-completion-ledger --project <path> --write",
            "launch_evidence_closure_memo": "dx forge launch-evidence-closure-memo --project <path> --write",
            "launch_evidence_final_brief": "dx forge launch-evidence-final-brief --project <path> --write",
            "launch_evidence_operator_runbook": "dx forge launch-evidence-operator-runbook --project <path> --write",
            "launch_evidence_handoff_capsule": "dx forge launch-evidence-handoff-capsule --project <path> --write",
            "launch_evidence_resumption_index": "dx forge launch-evidence-resumption-index --project <path> --write",
            "launch_evidence_recovery_brief": "dx forge launch-evidence-recovery-brief --project <path> --write",
            "launch_evidence_continuation_packet": "dx forge launch-evidence-continuation-packet --project <path> --write",
            "launch_evidence_operator_resume_card": "dx forge launch-evidence-operator-resume-card --project <path> --write",
            "launch_evidence_restart_ledger": "dx forge launch-evidence-restart-ledger --project <path> --write",
            "launch_evidence_restart_checklist": "dx forge launch-evidence-restart-checklist --project <path> --write",
            "launch_evidence_restart_brief": "dx forge launch-evidence-restart-brief --project <path> --write",
            "launch_evidence_restart_manifest": "dx forge launch-evidence-restart-manifest --project <path> --write",
            "launch_evidence_restart_receipt": "dx forge launch-evidence-restart-receipt --project <path> --write",
            "launch_evidence_restart_summary": "dx forge launch-evidence-restart-summary --project <path> --write",
            "launch_evidence_restart_snapshot": "dx forge launch-evidence-restart-snapshot --project <path> --write",
            "launch_evidence_restart_dispatch": "dx forge launch-evidence-restart-dispatch --project <path> --write",
            "launch_evidence_restart_closeout": "dx forge launch-evidence-restart-closeout --project <path> --write",
            "launch_evidence_restart_signoff": "dx forge launch-evidence-restart-signoff --project <path> --write",
            "launch_evidence_acceptance_index": "dx forge launch-evidence-acceptance-index --project <path> --write",
            "launch_evidence_acceptance_digest": "dx forge launch-evidence-acceptance-digest --project <path> --write",
            "launch_evidence_friday_baton": "dx forge launch-evidence-friday-baton --project <path> --write",
            "launch_verification_lane": "dx forge launch-verification-lane --project <path> --json",
            "forge_packages": "dx forge packages --json"
        },
        "readiness_receipts": {
            "template_readiness": NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE,
            "forge_receipt_glob": NEXT_FAMILIAR_LAUNCH_RECEIPT_GLOB,
            "forge_docs_file": NEXT_FAMILIAR_LAUNCH_RECEIPT_DOC
        },
        "companion_documentation_receipts": launch_companion_doc_receipts_contract(),
        "runtime_verification_checklist": launch_runtime_checklist_contract(),
        "runtime_approval_request": launch_runtime_approval_request_contract(),
        "runtime_evidence": launch_runtime_evidence_contract(),
        "runtime_evidence_review": {
            "schema": "dx.launch.runtime_evidence_review",
            "file": NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE,
            "command": "dx forge launch-runtime-evidence-review --project <path> --json",
            "finalization_receipt": NEXT_FAMILIAR_LAUNCH_RUNTIME_FINALIZATION_RECEIPT_FILE,
            "runtime_evidence": NEXT_FAMILIAR_LAUNCH_RUNTIME_EVIDENCE_FILE,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_packet": {
            "schema": "dx.launch.evidence_packet",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "command": "dx forge launch-evidence-packet --project <path> --json",
            "readiness_bundle": NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE,
            "runtime_evidence_review": NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_operator_index": {
            "schema": "dx.launch.evidence_operator_index",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE,
            "command": "dx forge launch-evidence-operator-index --project <path> --json",
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_status_timeline": {
            "schema": "dx.launch.evidence_status_timeline",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_STATUS_TIMELINE_FILE,
            "command": "dx forge launch-evidence-status-timeline --project <path> --json",
            "operator_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE,
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "freshness_source": "filesystem-metadata",
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_handoff_digest": {
            "schema": "dx.launch.evidence_handoff_digest",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
            "command": "dx forge launch-evidence-handoff-digest --project <path> --write",
            "status_timeline": NEXT_FAMILIAR_LAUNCH_EVIDENCE_STATUS_TIMELINE_FILE,
            "operator_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE,
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "format": "markdown",
            "zed_openable": true,
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_release_checklist": {
            "schema": "dx.launch.evidence_release_checklist",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE,
            "command": "dx forge launch-evidence-release-checklist --project <path> --write",
            "handoff_digest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
            "status_timeline": NEXT_FAMILIAR_LAUNCH_EVIDENCE_STATUS_TIMELINE_FILE,
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_share_manifest": {
            "schema": "dx.launch.evidence_share_manifest",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE,
            "command": "dx forge launch-evidence-share-manifest --project <path> --write",
            "release_checklist": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE,
            "handoff_digest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
            "status_timeline": NEXT_FAMILIAR_LAUNCH_EVIDENCE_STATUS_TIMELINE_FILE,
            "operator_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE,
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "export_target": "dx-cli-zed",
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_archive_index": {
            "schema": "dx.launch.evidence_archive_index",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE,
            "command": "dx forge launch-evidence-archive-index --project <path> --write",
            "share_manifest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE,
            "release_checklist": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE,
            "handoff_digest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "archive_target": "long-term-launch-handoff",
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_archive_receipt": {
            "schema": "dx.launch.evidence_archive_receipt",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_RECEIPT_FILE,
            "command": "dx forge launch-evidence-archive-receipt --project <path> --write",
            "archive_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE,
            "operator_handoff_target": "dx-cli-zed-archive",
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_archive_ledger": {
            "schema": "dx.launch.evidence_archive_ledger",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_LEDGER_FILE,
            "command": "dx forge launch-evidence-archive-ledger --project <path> --write",
            "archive_receipt": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_RECEIPT_FILE,
            "archive_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE,
            "share_manifest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE,
            "release_checklist": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE,
            "handoff_digest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_retention_policy": {
            "schema": "dx.launch.evidence_retention_policy",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_POLICY_FILE,
            "command": "dx forge launch-evidence-retention-policy --project <path> --write",
            "archive_ledger": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_LEDGER_FILE,
            "archive_receipt": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_RECEIPT_FILE,
            "archive_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE,
            "share_manifest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE,
            "release_checklist": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE,
            "handoff_digest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_retention_review": {
            "schema": "dx.launch.evidence_retention_review",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_REVIEW_FILE,
            "command": "dx forge launch-evidence-retention-review --project <path> --write",
            "retention_policy": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_POLICY_FILE,
            "archive_ledger": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_LEDGER_FILE,
            "archive_receipt": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_RECEIPT_FILE,
            "archive_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE,
            "share_manifest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE,
            "release_checklist": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE,
            "handoff_digest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_release_seal": {
            "schema": "dx.launch.evidence_release_seal",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE,
            "command": "dx forge launch-evidence-release-seal --project <path> --write",
            "retention_review": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_REVIEW_FILE,
            "retention_policy": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_POLICY_FILE,
            "archive_ledger": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_LEDGER_FILE,
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_operator_summary": {
            "schema": "dx.launch.evidence_operator_summary",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_SUMMARY_FILE,
            "command": "dx forge launch-evidence-operator-summary --project <path> --write",
            "release_seal": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE,
            "retention_review": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_REVIEW_FILE,
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "final_runtime_review": NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE,
            "summary_target": "terminal-friendly-launch-handoff",
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_completion_ledger": {
            "schema": "dx.launch.evidence_completion_ledger",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_COMPLETION_LEDGER_FILE,
            "command": "dx forge launch-evidence-completion-ledger --project <path> --write",
            "operator_summary": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_SUMMARY_FILE,
            "release_seal": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE,
            "retention_review": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_REVIEW_FILE,
            "final_runtime_review": NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE,
            "completion_target": "final-launch-evidence-completion-map",
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_closure_memo": {
            "schema": "dx.launch.evidence_closure_memo",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_CLOSURE_MEMO_FILE,
            "command": "dx forge launch-evidence-closure-memo --project <path> --write",
            "completion_ledger": NEXT_FAMILIAR_LAUNCH_EVIDENCE_COMPLETION_LEDGER_FILE,
            "operator_summary": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_SUMMARY_FILE,
            "release_seal": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE,
            "final_runtime_review": NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE,
            "memo_target": "human-readable-launch-release-closeout",
            "format": "markdown",
            "zed_openable": true,
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_final_brief": {
            "schema": "dx.launch.evidence_final_brief",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_FINAL_BRIEF_FILE,
            "command": "dx forge launch-evidence-final-brief --project <path> --write",
            "closure_memo": NEXT_FAMILIAR_LAUNCH_EVIDENCE_CLOSURE_MEMO_FILE,
            "completion_ledger": NEXT_FAMILIAR_LAUNCH_EVIDENCE_COMPLETION_LEDGER_FILE,
            "operator_summary": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_SUMMARY_FILE,
            "release_seal": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE,
            "brief_target": "dx-cli-zed-launch-closeout-pointer",
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_operator_runbook": {
            "schema": "dx.launch.evidence_operator_runbook",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RUNBOOK_FILE,
            "command": "dx forge launch-evidence-operator-runbook --project <path> --write",
            "final_brief": NEXT_FAMILIAR_LAUNCH_EVIDENCE_FINAL_BRIEF_FILE,
            "closure_memo": NEXT_FAMILIAR_LAUNCH_EVIDENCE_CLOSURE_MEMO_FILE,
            "completion_ledger": NEXT_FAMILIAR_LAUNCH_EVIDENCE_COMPLETION_LEDGER_FILE,
            "operator_summary": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_SUMMARY_FILE,
            "release_seal": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE,
            "runbook_target": "restartable-dx-worker-checklist",
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_handoff_capsule": {
            "schema": "dx.launch.evidence_handoff_capsule",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_CAPSULE_FILE,
            "command": "dx forge launch-evidence-handoff-capsule --project <path> --write",
            "operator_runbook": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RUNBOOK_FILE,
            "final_brief": NEXT_FAMILIAR_LAUNCH_EVIDENCE_FINAL_BRIEF_FILE,
            "closure_memo": NEXT_FAMILIAR_LAUNCH_EVIDENCE_CLOSURE_MEMO_FILE,
            "completion_ledger": NEXT_FAMILIAR_LAUNCH_EVIDENCE_COMPLETION_LEDGER_FILE,
            "capsule_target": "dx-cli-zed-restart-artifact",
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_resumption_index": {
            "schema": "dx.launch.evidence_resumption_index",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESUMPTION_INDEX_FILE,
            "command": "dx forge launch-evidence-resumption-index --project <path> --write",
            "handoff_capsule": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_CAPSULE_FILE,
            "operator_runbook": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RUNBOOK_FILE,
            "final_brief": NEXT_FAMILIAR_LAUNCH_EVIDENCE_FINAL_BRIEF_FILE,
            "resumption_target": "ordered-dx-cli-zed-restart-lanes",
            "lanes": ["source-only", "runtime-approved", "release-closeout"],
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_recovery_brief": {
            "schema": "dx.launch.evidence_recovery_brief",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RECOVERY_BRIEF_FILE,
            "command": "dx forge launch-evidence-recovery-brief --project <path> --write",
            "resumption_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESUMPTION_INDEX_FILE,
            "handoff_capsule": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_CAPSULE_FILE,
            "operator_runbook": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RUNBOOK_FILE,
            "final_brief": NEXT_FAMILIAR_LAUNCH_EVIDENCE_FINAL_BRIEF_FILE,
            "recovery_target": "human-readable-dx-worker-restart-brief",
            "format": "markdown",
            "zed_openable": true,
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_continuation_packet": {
            "schema": "dx.launch.evidence_continuation_packet",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_CONTINUATION_PACKET_FILE,
            "command": "dx forge launch-evidence-continuation-packet --project <path> --write",
            "recovery_brief": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RECOVERY_BRIEF_FILE,
            "resumption_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESUMPTION_INDEX_FILE,
            "handoff_capsule": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_CAPSULE_FILE,
            "operator_runbook": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RUNBOOK_FILE,
            "continuation_target": "dx-cli-zed-continuation-packet",
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_operator_resume_card": {
            "schema": "dx.launch.evidence_operator_resume_card",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RESUME_CARD_FILE,
            "command": "dx forge launch-evidence-operator-resume-card --project <path> --write",
            "continuation_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_CONTINUATION_PACKET_FILE,
            "recovery_brief": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RECOVERY_BRIEF_FILE,
            "resume_target": "terminal-first-dx-resume-card",
            "display_mode": "terminal-first",
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_restart_ledger": {
            "schema": "dx.launch.evidence_restart_ledger",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_LEDGER_FILE,
            "command": "dx forge launch-evidence-restart-ledger --project <path> --write",
            "operator_resume_card": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RESUME_CARD_FILE,
            "continuation_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_CONTINUATION_PACKET_FILE,
            "recovery_brief": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RECOVERY_BRIEF_FILE,
            "resumption_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESUMPTION_INDEX_FILE,
            "ledger_target": "durable-dx-restart-ledger",
            "display_mode": "terminal-first",
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_restart_checklist": {
            "schema": "dx.launch.evidence_restart_checklist",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CHECKLIST_FILE,
            "command": "dx forge launch-evidence-restart-checklist --project <path> --write",
            "restart_ledger": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_LEDGER_FILE,
            "operator_resume_card": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RESUME_CARD_FILE,
            "checklist_target": "dx-cli-zed-restart-next-actions",
            "lanes": ["source-only", "runtime-approved", "release-closeout"],
            "display_mode": "terminal-first",
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_restart_brief": {
            "schema": "dx.launch.evidence_restart_brief",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_BRIEF_FILE,
            "command": "dx forge launch-evidence-restart-brief --project <path> --write",
            "restart_checklist": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CHECKLIST_FILE,
            "restart_ledger": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_LEDGER_FILE,
            "brief_target": "zed-openable-dx-restart-brief",
            "format": "markdown",
            "zed_openable": true,
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_restart_manifest": {
            "schema": "dx.launch.evidence_restart_manifest",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_MANIFEST_FILE,
            "command": "dx forge launch-evidence-restart-manifest --project <path> --write",
            "restart_brief": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_BRIEF_FILE,
            "restart_checklist": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CHECKLIST_FILE,
            "restart_ledger": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_LEDGER_FILE,
            "operator_resume_card": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RESUME_CARD_FILE,
            "manifest_target": "dx-cli-zed-indexable-restart-manifest",
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_restart_receipt": {
            "schema": "dx.launch.evidence_restart_receipt",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_RECEIPT_FILE,
            "command": "dx forge launch-evidence-restart-receipt --project <path> --write",
            "restart_manifest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_MANIFEST_FILE,
            "receipt_target": "latest-resumable-dx-zed-handoff",
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_restart_summary": {
            "schema": "dx.launch.evidence_restart_summary",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SUMMARY_FILE,
            "command": "dx forge launch-evidence-restart-summary --project <path> --write",
            "restart_receipt": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_RECEIPT_FILE,
            "restart_manifest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_MANIFEST_FILE,
            "restart_brief": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_BRIEF_FILE,
            "restart_checklist": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CHECKLIST_FILE,
            "summary_target": "terminal-friendly-dx-zed-restart-handoff",
            "display_mode": "terminal-first",
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_restart_snapshot": {
            "schema": "dx.launch.evidence_restart_snapshot",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SNAPSHOT_FILE,
            "command": "dx forge launch-evidence-restart-snapshot --project <path> --write",
            "restart_summary": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SUMMARY_FILE,
            "restart_receipt": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_RECEIPT_FILE,
            "restart_manifest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_MANIFEST_FILE,
            "restart_brief": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_BRIEF_FILE,
            "snapshot_target": "latest-openable-dx-zed-restart-file",
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_restart_dispatch": {
            "schema": "dx.launch.evidence_restart_dispatch",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_DISPATCH_FILE,
            "command": "dx forge launch-evidence-restart-dispatch --project <path> --write",
            "restart_snapshot": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SNAPSHOT_FILE,
            "restart_summary": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SUMMARY_FILE,
            "restart_receipt": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_RECEIPT_FILE,
            "restart_manifest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_MANIFEST_FILE,
            "dispatch_target": "one-command-next-worker-dispatch-card",
            "display_mode": "next-worker-card",
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_restart_closeout": {
            "schema": "dx.launch.evidence_restart_closeout",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CLOSEOUT_FILE,
            "command": "dx forge launch-evidence-restart-closeout --project <path> --write",
            "restart_dispatch": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_DISPATCH_FILE,
            "closeout_target": "final-friday-essencefromexistence-closeout-actions",
            "format": "markdown",
            "zed_openable": true,
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_restart_signoff": {
            "schema": "dx.launch.evidence_restart_signoff",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SIGNOFF_FILE,
            "command": "dx forge launch-evidence-restart-signoff --project <path> --write",
            "restart_closeout": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CLOSEOUT_FILE,
            "signoff_target": "friday-essencefromexistence-acceptance-receipt",
            "acceptance_status": "reviewable",
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_acceptance_index": {
            "schema": "dx.launch.evidence_acceptance_index",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ACCEPTANCE_INDEX_FILE,
            "command": "dx forge launch-evidence-acceptance-index --project <path> --write",
            "restart_signoff": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SIGNOFF_FILE,
            "restart_closeout": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CLOSEOUT_FILE,
            "restart_dispatch": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_DISPATCH_FILE,
            "restart_snapshot": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SNAPSHOT_FILE,
            "acceptance_target": "friday-final-handoff-index",
            "format": "markdown",
            "zed_openable": true,
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_acceptance_digest": {
            "schema": "dx.launch.evidence_acceptance_digest",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ACCEPTANCE_DIGEST_FILE,
            "command": "dx forge launch-evidence-acceptance-digest --project <path> --write",
            "acceptance_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ACCEPTANCE_INDEX_FILE,
            "digest_target": "friday-terminal-final-status-line",
            "format": "json",
            "display_mode": "terminal-first-final-status",
            "zed_openable": true,
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_evidence_friday_baton": {
            "schema": "dx.launch.evidence_friday_baton",
            "file": NEXT_FAMILIAR_LAUNCH_EVIDENCE_FRIDAY_BATON_FILE,
            "command": "dx forge launch-evidence-friday-baton --project <path> --write",
            "acceptance_digest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ACCEPTANCE_DIGEST_FILE,
            "acceptance_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ACCEPTANCE_INDEX_FILE,
            "restart_signoff": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SIGNOFF_FILE,
            "launch_verification_lane": NEXT_FAMILIAR_LAUNCH_VERIFICATION_LANE_FILE,
            "baton_target": "friday-orchestrator-final-handoff",
            "format": "markdown",
            "zed_openable": true,
            "reads_runtime_artifact_contents": false,
            "requires_runtime_execution": false,
            "no_execution": true
        },
        "launch_verification_lane": launch_verification_lane_contract(),
        "source_guards": [
            {
                "kind": "source_level_template_guard",
                "command": "dx run --test .\\benchmarks\\template-shell.test.ts",
                "status": "source-guard-required"
            },
            {
                "kind": "motion_launch_proof_guard",
                "command": "dx run --test .\\benchmarks\\motion-launch-proof.test.ts",
                "status": "source-guard-required"
            },
            {
                "kind": "source_level_package_guard",
                "command": "dx run --test .\\benchmarks\\launch-package-slices.test.ts",
                "status": "source-guard-required"
            }
        ],
        "package_receipts": {
            "registry_command": "dx forge packages --json",
            "required_packages": FORGE_WWW_TEMPLATE_PACKAGE_IDS
        },
        "zed_template_handoff": launch_zed_template_handoff_contract(),
        "runtime_gate": {
            "status": "pending-governed-runtime-pass",
            "requires_explicit_permission": true,
            "blocked_without_permission": ["dev-server", "full-build", "production-preview"]
        },
        "no_execution": true
    })
}

fn launch_adoption_report_contract() -> serde_json::Value {
    serde_json::json!({
        "schema": "dx.launch.adoption_report",
        "owner": "dx-www",
        "template_id": "next-familiar-www-template",
        "route": "/",
        "file": NEXT_FAMILIAR_LAUNCH_ADOPTION_REPORT_FILE,
        "command": "dx forge launch-adoption-report --project <path> --json",
        "summaries": [
            "materialized_launch_companion_files",
            "app_owned_package_dependencies",
            "permission_gated_runtime_proofs"
        ],
        "input_artifacts": [
            ".dx/forge/template-.dx/build-cache/manifest.json",
            NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE,
            NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE,
            NEXT_FAMILIAR_LAUNCH_COMPANION_DOC_RECEIPTS_FILE,
            NEXT_FAMILIAR_LAUNCH_RUNTIME_CHECKLIST_FILE,
            NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE,
            NEXT_FAMILIAR_LAUNCH_RUNTIME_EVIDENCE_FILE,
            NEXT_FAMILIAR_LAUNCH_VERIFICATION_LANE_FILE
        ],
        "follow_up_commands": {
            "manifest_drift": "dx forge launch-manifest-drift --project <path> --json"
        },
        "runtime_gate": {
            "status": "pending-governed-runtime-pass",
            "requires_explicit_permission": true
        },
        "no_execution": true
    })
}

fn next_familiar_launch_generated_file_paths() -> Vec<&'static str> {
    vec![
        "app/page.tsx",
        "components/template-app/template-route-contract.ts",
        "components/template-app/template-shell.tsx",
        "components/template-app/template-dashboard-nav.tsx",
        "components/template-app/dx-studio-edit-contract.ts",
        "components/template-app/shadcn-dashboard-controls-contract.tsx",
        "components/template-app/shadcn-dashboard-controls.tsx",
        "components/template-app/automations-status.tsx",
        "components/template-app/automation-mission-summary.tsx",
        "components/template-app/automations/automations-metadata.ts",
        "components/template-app/motion-interaction-proof.tsx",
        "components/template-app/template-lead-form.tsx",
        ".dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json",
        "components/template-app/auth-session-status.tsx",
        ".dx/forge/receipts/auth-better-auth.json",
        "components/template-app/ai-chat-status.tsx",
        "components/template-app/data-status.tsx",
        "lib/supabase/profile-workflow.ts",
        "components/template-app/supabase-profile-workflow.tsx",
        ".dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json",
        "components/template-app/payments-status.tsx",
        ".dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json",
        "components/template-app/docs-status.tsx",
        "components/template-app/instantdb-status.tsx",
        ".dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json",
        "components/template-app/wasm-interop-status.tsx",
        "components/template-app/zod-validation-status.tsx",
        "components/template-app/zod-dashboard-settings.tsx",
        "components/template-app/icon-status.tsx",
        "components/template-app/next-intl-status.tsx",
        "components/template-app/query-cache-status.tsx",
        "components/template-app/query-dashboard-read-model.ts",
        "components/template-app/dx-check-style-evidence-read-model.ts",
        "components/template-app/template-shell-evidence-loader.ts",
        "components/template-app/template-shell-style-evidence-drift.ts",
        "components/template-app/preview-style-evidence-read-model.ts",
        "components/template-app/forge-package-status.ts",
        "components/template-app/forge-package-status-read-model.ts",
        "components/template-app/preview-style-package-panel-read-model.ts",
        "components/template-app/preview-style-package-ownership-read-model.ts",
        "components/template-app/forge-golden-path-contract.ts",
        "components/template-app/forge-golden-path-panel.tsx",
        "components/template-app/forge-safety-archive-contract.ts",
        "components/template-app/forge-safety-archive-runbook.ts",
        "components/template-app/forge-safety-archive-panel.tsx",
        "components/template-app/forge-remote-head-health-contract.ts",
        "components/template-app/forge-remote-head-health-panel.tsx",
        "components/template-app/package-catalog.ts",
        "components/template-app/react-markdown-preview.tsx",
        "components/template-app/state-zustand-counter.tsx",
        "components/template-app/state-zustand-dashboard.tsx",
        ".dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json",
        "components/template-app/trpc-launch-contract.ts",
        "components/template-app/trpc-launch-health.tsx",
        "components/scene/launch-scene.tsx",
        "lib/scene/index.ts",
        "lib/scene/types.ts",
        "lib/scene/preset.ts",
        "lib/scene/frame-sample.ts",
        "lib/scene/capability-report.ts",
        "lib/scene/viewport-report.ts",
        "lib/scene/bounds-report.ts",
        "lib/scene/raycast-report.ts",
        "lib/scene/preview-readiness.ts",
        "lib/scene/webgl-runtime.ts",
        "lib/scene/metadata.ts",
        "lib/scene/README.md",
        "components/template-app/template-console.tsx",
        "server/templateCatalog.ts",
        NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE,
        NEXT_FAMILIAR_LAUNCH_ZED_TEMPLATE_HANDOFF_FILE,
        NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE,
        NEXT_FAMILIAR_LAUNCH_COMPANION_DOC_RECEIPTS_FILE,
        NEXT_FAMILIAR_LAUNCH_RUNTIME_CHECKLIST_FILE,
        NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE,
        NEXT_FAMILIAR_LAUNCH_RUNTIME_EVIDENCE_FILE,
        NEXT_FAMILIAR_LAUNCH_VERIFICATION_LANE_FILE,
    ]
}

fn next_familiar_launch_component_materialized_files() -> Vec<&'static str> {
    vec![
        "components/template-app/template-shell.tsx",
        "components/template-app/template-dashboard-nav.tsx",
        "components/template-app/dx-studio-edit-contract.ts",
        "components/template-app/shadcn-dashboard-controls-contract.tsx",
        "components/template-app/shadcn-dashboard-controls.tsx",
        "components/template-app/automations-status.tsx",
        "components/template-app/automation-mission-summary.tsx",
        "components/template-app/automations/automations-metadata.ts",
        "components/template-app/motion-interaction-proof.tsx",
        "components/template-app/template-lead-form.tsx",
        "components/template-app/template-route-contract.ts",
        "components/template-app/auth-session-status.tsx",
        "components/template-app/ai-chat-status.tsx",
        "components/template-app/data-status.tsx",
        "lib/supabase/profile-workflow.ts",
        "components/template-app/supabase-profile-workflow.tsx",
        "components/template-app/payments-status.tsx",
        "components/template-app/docs-status.tsx",
        "components/template-app/instantdb-status.tsx",
        "components/template-app/icon-status.tsx",
        "components/template-app/next-intl-status.tsx",
        "components/template-app/query-cache-status.tsx",
        "components/template-app/query-dashboard-read-model.ts",
        "components/template-app/forge-package-status.ts",
        "components/template-app/forge-package-status-read-model.ts",
        "components/template-app/preview-style-package-panel-read-model.ts",
        "components/template-app/preview-style-package-ownership-read-model.ts",
        "components/template-app/forge-golden-path-contract.ts",
        "components/template-app/forge-golden-path-panel.tsx",
        "components/template-app/forge-safety-archive-contract.ts",
        "components/template-app/forge-safety-archive-runbook.ts",
        "components/template-app/forge-safety-archive-panel.tsx",
        "components/template-app/forge-remote-head-health-contract.ts",
        "components/template-app/forge-remote-head-health-panel.tsx",
        "components/template-app/wasm-interop-status.tsx",
        "components/template-app/zod-validation-status.tsx",
        "components/template-app/zod-dashboard-settings.tsx",
        "components/template-app/package-catalog.ts",
        "components/template-app/template-surface-registry.ts",
        "components/template-app/framework-completeness.ts",
        "components/template-app/state-zustand-counter.tsx",
        "components/template-app/state-zustand-dashboard.tsx",
        "components/template-app/react-markdown-preview.tsx",
        "components/template-app/trpc-launch-contract.ts",
        "components/template-app/trpc-launch-health.tsx",
        "components/scene/launch-scene.tsx",
        "lib/scene/index.ts",
        "lib/scene/types.ts",
        "lib/scene/preset.ts",
        "lib/scene/interaction.ts",
        "lib/scene/dashboard-workflow.ts",
        "lib/scene/dashboard-controls.ts",
        "lib/scene/frame-sample.ts",
        "lib/scene/capability-report.ts",
        "lib/scene/viewport-report.ts",
        "lib/scene/bounds-report.ts",
        "lib/scene/raycast-report.ts",
        "lib/scene/preview-readiness.ts",
        "lib/scene/performance-monitor.ts",
        "lib/scene/renderer-handoff.ts",
        "lib/scene/r3f-renderer-adapter.ts",
        "lib/scene/webgl-runtime.ts",
        "lib/scene/metadata.ts",
        "lib/scene/README.md",
        "components/template-app/template-console.tsx",
        "server/templateCatalog.ts",
    ]
}

fn launch_manifest_drift_source_contract() -> serde_json::Value {
    serde_json::json!({
        "schema": "dx.templates.launch_manifest_source",
        "source_command": "dx templates --json",
        "template_id": "next-familiar-www-template",
        "route": "/",
        "package_ids": FORGE_WWW_TEMPLATE_PACKAGE_IDS,
        "generated_files": next_familiar_launch_generated_file_paths(),
        "component_materialized_files": next_familiar_launch_component_materialized_files(),
        "no_execution": true
    })
}

fn launch_manifest_drift_contract() -> serde_json::Value {
    serde_json::json!({
        "schema": "dx.launch.manifest_drift",
        "owner": "dx-www",
        "template_id": "next-familiar-www-template",
        "route": "/",
        "file": NEXT_FAMILIAR_LAUNCH_MANIFEST_DRIFT_FILE,
        "command": "dx forge launch-manifest-drift --project <path> --json",
        "compares": [
            "dx_templates_json_source_contract",
            ".dx/forge/template-.dx/build-cache/manifest.json",
            ".dx/forge/template-readiness/launch-route.json",
            ".dx/forge/template-readiness/launch-companion-doc-receipts.json",
            ".dx/forge/template-readiness/launch-runtime-checklist.json",
            ".dx/forge/template-readiness/launch-runtime-approval-request.json",
            ".dx/forge/template-readiness/launch-runtime-evidence.json",
            ".dx/forge/template-readiness/launch-verification-lane.json",
            "launch_companion_file_coverage"
        ],
        "runtime_gate": {
            "status": "pending-governed-runtime-pass",
            "requires_explicit_permission": true
        },
        "no_execution": true
    })
}

fn launch_discovery_contract() -> serde_json::Value {
    serde_json::json!({
        "schema": "dx.discovery",
        "consumers": ["dx-cli", "zed"],
        "template_id": "next-familiar-www-template",
        "template_schema": "dx.www.templates",
        "packages_schema": "dx.forge.packages",
        "metadata_commands": {
            "templates": "dx templates --json",
            "packages": "dx forge packages --json",
            "launch_adoption_report": "dx forge launch-adoption-report --project <path> --json",
            "launch_manifest_drift": "dx forge launch-manifest-drift --project <path> --json",
            "launch_companion_receipts": "dx forge launch-companion-receipts --project <path> --json",
            "launch_runtime_checklist": "dx forge launch-runtime-checklist --project <path> --json",
            "launch_runtime_approval_request": "dx forge launch-runtime-approval-request --project <path> --json",
            "launch_runtime_evidence": "dx forge launch-runtime-evidence --project <path> --json",
            "launch_runtime_evidence_import_plan": "dx forge launch-runtime-evidence-import-plan --project <path> --build-log <path> --route-response <path> --preview-proof <path> --json",
            "launch_runtime_evidence_completeness": "dx forge launch-runtime-evidence-completeness --project <path> --import-plan <path> --json",
            "launch_runtime_evidence_finalization": "dx forge launch-runtime-evidence-finalize --project <path> --import-plan <path> --write --json",
            "launch_runtime_evidence_review": "dx forge launch-runtime-evidence-review --project <path> --json",
            "launch_evidence_packet": "dx forge launch-evidence-packet --project <path> --json",
            "launch_evidence_operator_index": "dx forge launch-evidence-operator-index --project <path> --json",
            "launch_evidence_status_timeline": "dx forge launch-evidence-status-timeline --project <path> --json",
            "launch_evidence_handoff_digest": "dx forge launch-evidence-handoff-digest --project <path> --write",
            "launch_evidence_release_checklist": "dx forge launch-evidence-release-checklist --project <path> --write",
            "launch_evidence_share_manifest": "dx forge launch-evidence-share-manifest --project <path> --write",
            "launch_evidence_archive_index": "dx forge launch-evidence-archive-index --project <path> --write",
            "launch_evidence_archive_receipt": "dx forge launch-evidence-archive-receipt --project <path> --write",
            "launch_evidence_archive_ledger": "dx forge launch-evidence-archive-ledger --project <path> --write",
            "launch_evidence_retention_policy": "dx forge launch-evidence-retention-policy --project <path> --write",
            "launch_evidence_retention_review": "dx forge launch-evidence-retention-review --project <path> --write",
            "launch_evidence_release_seal": "dx forge launch-evidence-release-seal --project <path> --write",
            "launch_evidence_operator_summary": "dx forge launch-evidence-operator-summary --project <path> --write",
            "launch_evidence_completion_ledger": "dx forge launch-evidence-completion-ledger --project <path> --write",
            "launch_evidence_closure_memo": "dx forge launch-evidence-closure-memo --project <path> --write",
            "launch_evidence_final_brief": "dx forge launch-evidence-final-brief --project <path> --write",
            "launch_evidence_operator_runbook": "dx forge launch-evidence-operator-runbook --project <path> --write",
            "launch_evidence_handoff_capsule": "dx forge launch-evidence-handoff-capsule --project <path> --write",
            "launch_evidence_resumption_index": "dx forge launch-evidence-resumption-index --project <path> --write",
            "launch_evidence_recovery_brief": "dx forge launch-evidence-recovery-brief --project <path> --write",
            "launch_evidence_continuation_packet": "dx forge launch-evidence-continuation-packet --project <path> --write",
            "launch_evidence_operator_resume_card": "dx forge launch-evidence-operator-resume-card --project <path> --write",
            "launch_evidence_restart_ledger": "dx forge launch-evidence-restart-ledger --project <path> --write",
            "launch_evidence_restart_checklist": "dx forge launch-evidence-restart-checklist --project <path> --write",
            "launch_evidence_restart_brief": "dx forge launch-evidence-restart-brief --project <path> --write",
            "launch_evidence_restart_manifest": "dx forge launch-evidence-restart-manifest --project <path> --write",
            "launch_evidence_restart_receipt": "dx forge launch-evidence-restart-receipt --project <path> --write",
            "launch_evidence_restart_summary": "dx forge launch-evidence-restart-summary --project <path> --write",
            "launch_evidence_restart_snapshot": "dx forge launch-evidence-restart-snapshot --project <path> --write",
            "launch_evidence_restart_dispatch": "dx forge launch-evidence-restart-dispatch --project <path> --write",
            "launch_evidence_restart_closeout": "dx forge launch-evidence-restart-closeout --project <path> --write",
            "launch_evidence_restart_signoff": "dx forge launch-evidence-restart-signoff --project <path> --write",
            "launch_evidence_acceptance_index": "dx forge launch-evidence-acceptance-index --project <path> --write",
            "launch_evidence_acceptance_digest": "dx forge launch-evidence-acceptance-digest --project <path> --write",
            "launch_evidence_friday_baton": "dx forge launch-evidence-friday-baton --project <path> --write",
            "launch_verification_lane": "dx forge launch-verification-lane --project <path> --json",
            "project": "dx new <name>"
        },
        "indexable_fields": [
            "id",
            "label",
            "description",
            "forge_packages",
            "www_package_catalog",
            "launch_companion_doc_receipts",
            "launch_companion_receipts",
            "app_router_entrypoint",
            "zed_template_handoff",
            "launch_readiness_bundle",
            "launch_adoption_report",
            "launch_manifest_drift",
            "launch_runtime_checklist",
            "launch_runtime_approval_request",
            "launch_runtime_evidence",
            "launch_runtime_evidence_finalization",
            "launch_runtime_evidence_review",
            "launch_evidence_packet",
            "launch_evidence_operator_index",
            "launch_evidence_status_timeline",
            "launch_evidence_handoff_digest",
            "launch_evidence_release_checklist",
            "launch_evidence_share_manifest",
            "launch_evidence_archive_index",
            "launch_evidence_archive_receipt",
            "launch_evidence_archive_ledger",
            "launch_evidence_retention_policy",
            "launch_evidence_retention_review",
            "launch_evidence_release_seal",
            "launch_evidence_operator_summary",
            "launch_evidence_completion_ledger",
            "launch_evidence_closure_memo",
            "launch_evidence_final_brief",
            "launch_evidence_operator_runbook",
            "launch_evidence_handoff_capsule",
            "launch_evidence_resumption_index",
            "launch_evidence_recovery_brief",
            "launch_evidence_continuation_packet",
            "launch_evidence_operator_resume_card",
            "launch_evidence_restart_ledger",
            "launch_evidence_restart_checklist",
            "launch_evidence_restart_brief",
            "launch_evidence_restart_manifest",
            "launch_evidence_restart_receipt",
            "launch_evidence_restart_summary",
            "launch_evidence_restart_snapshot",
            "launch_evidence_restart_dispatch",
            "launch_evidence_restart_closeout",
            "launch_evidence_restart_signoff",
            "launch_evidence_acceptance_index",
            "launch_evidence_acceptance_digest",
            "launch_evidence_friday_baton",
            "launch_verification_lane",
            "generated_template_files",
            "usage_examples"
        ],
        "entrypoints": [
            {
                "kind": "app-router-route",
                "route": "/",
                "route_aliases": [],
                "source_file": "examples/template/app/page.tsx",
                "materialized_file": "app/page.tsx",
                "readiness_receipt": NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE,
                "readiness_bundle": NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE,
                "runtime_verification": "pending-governed-runtime-pass",
                "runtime_verification_requires_explicit_permission": true
            }
        ],
        "zed_template_handoff": launch_zed_template_handoff_contract(),
        "launch_readiness_bundle": launch_readiness_bundle_contract(),
        "launch_adoption_report": launch_adoption_report_contract(),
        "launch_manifest_drift": launch_manifest_drift_contract(),
        "launch_companion_doc_receipts": launch_companion_doc_receipts_contract(),
        "launch_companion_receipts": launch_companion_receipts_contract(),
        "launch_runtime_checklist": launch_runtime_checklist_contract(),
        "launch_runtime_approval_request": launch_runtime_approval_request_contract(),
        "launch_runtime_evidence": launch_runtime_evidence_contract(),
        "launch_runtime_evidence_finalization": {
            "schema": "dx.launch.runtime_evidence_finalization",
            "command": "dx forge launch-runtime-evidence-finalize --project <path> --import-plan <path> --write --json",
            "receipt": NEXT_FAMILIAR_LAUNCH_RUNTIME_FINALIZATION_RECEIPT_FILE,
            "no_execution": true
        },
        "launch_runtime_evidence_review": {
            "schema": "dx.launch.runtime_evidence_review",
            "command": "dx forge launch-runtime-evidence-review --project <path> --json",
            "report": NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE,
            "finalization_receipt": NEXT_FAMILIAR_LAUNCH_RUNTIME_FINALIZATION_RECEIPT_FILE,
            "no_execution": true
        },
        "launch_evidence_packet": {
            "schema": "dx.launch.evidence_packet",
            "command": "dx forge launch-evidence-packet --project <path> --json",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "runtime_evidence_review": NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE,
            "no_execution": true
        },
        "launch_evidence_operator_index": {
            "schema": "dx.launch.evidence_operator_index",
            "command": "dx forge launch-evidence-operator-index --project <path> --json",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE,
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_status_timeline": {
            "schema": "dx.launch.evidence_status_timeline",
            "command": "dx forge launch-evidence-status-timeline --project <path> --json",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_STATUS_TIMELINE_FILE,
            "operator_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE,
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "freshness_source": "filesystem-metadata",
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_handoff_digest": {
            "schema": "dx.launch.evidence_handoff_digest",
            "command": "dx forge launch-evidence-handoff-digest --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
            "status_timeline": NEXT_FAMILIAR_LAUNCH_EVIDENCE_STATUS_TIMELINE_FILE,
            "operator_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE,
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "format": "markdown",
            "zed_openable": true,
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_release_checklist": {
            "schema": "dx.launch.evidence_release_checklist",
            "command": "dx forge launch-evidence-release-checklist --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE,
            "handoff_digest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
            "status_timeline": NEXT_FAMILIAR_LAUNCH_EVIDENCE_STATUS_TIMELINE_FILE,
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_share_manifest": {
            "schema": "dx.launch.evidence_share_manifest",
            "command": "dx forge launch-evidence-share-manifest --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE,
            "release_checklist": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE,
            "handoff_digest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
            "status_timeline": NEXT_FAMILIAR_LAUNCH_EVIDENCE_STATUS_TIMELINE_FILE,
            "operator_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE,
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "export_target": "dx-cli-zed",
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_archive_index": {
            "schema": "dx.launch.evidence_archive_index",
            "command": "dx forge launch-evidence-archive-index --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE,
            "share_manifest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE,
            "release_checklist": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE,
            "handoff_digest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "archive_target": "long-term-launch-handoff",
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_archive_receipt": {
            "schema": "dx.launch.evidence_archive_receipt",
            "command": "dx forge launch-evidence-archive-receipt --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_RECEIPT_FILE,
            "archive_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE,
            "operator_handoff_target": "dx-cli-zed-archive",
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_archive_ledger": {
            "schema": "dx.launch.evidence_archive_ledger",
            "command": "dx forge launch-evidence-archive-ledger --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_LEDGER_FILE,
            "archive_receipt": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_RECEIPT_FILE,
            "archive_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE,
            "share_manifest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE,
            "release_checklist": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE,
            "handoff_digest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_retention_policy": {
            "schema": "dx.launch.evidence_retention_policy",
            "command": "dx forge launch-evidence-retention-policy --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_POLICY_FILE,
            "archive_ledger": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_LEDGER_FILE,
            "archive_receipt": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_RECEIPT_FILE,
            "archive_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE,
            "share_manifest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE,
            "release_checklist": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE,
            "handoff_digest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_retention_review": {
            "schema": "dx.launch.evidence_retention_review",
            "command": "dx forge launch-evidence-retention-review --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_REVIEW_FILE,
            "retention_policy": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_POLICY_FILE,
            "archive_ledger": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_LEDGER_FILE,
            "archive_receipt": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_RECEIPT_FILE,
            "archive_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE,
            "share_manifest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE,
            "release_checklist": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE,
            "handoff_digest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_release_seal": {
            "schema": "dx.launch.evidence_release_seal",
            "command": "dx forge launch-evidence-release-seal --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE,
            "retention_review": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_REVIEW_FILE,
            "retention_policy": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_POLICY_FILE,
            "archive_ledger": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_LEDGER_FILE,
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_operator_summary": {
            "schema": "dx.launch.evidence_operator_summary",
            "command": "dx forge launch-evidence-operator-summary --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_SUMMARY_FILE,
            "release_seal": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE,
            "retention_review": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_REVIEW_FILE,
            "release_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
            "final_runtime_review": NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE,
            "summary_target": "terminal-friendly-launch-handoff",
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_completion_ledger": {
            "schema": "dx.launch.evidence_completion_ledger",
            "command": "dx forge launch-evidence-completion-ledger --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_COMPLETION_LEDGER_FILE,
            "operator_summary": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_SUMMARY_FILE,
            "release_seal": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE,
            "retention_review": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_REVIEW_FILE,
            "final_runtime_review": NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE,
            "completion_target": "final-launch-evidence-completion-map",
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_closure_memo": {
            "schema": "dx.launch.evidence_closure_memo",
            "command": "dx forge launch-evidence-closure-memo --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_CLOSURE_MEMO_FILE,
            "completion_ledger": NEXT_FAMILIAR_LAUNCH_EVIDENCE_COMPLETION_LEDGER_FILE,
            "operator_summary": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_SUMMARY_FILE,
            "release_seal": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE,
            "final_runtime_review": NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE,
            "memo_target": "human-readable-launch-release-closeout",
            "format": "markdown",
            "zed_openable": true,
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_final_brief": {
            "schema": "dx.launch.evidence_final_brief",
            "command": "dx forge launch-evidence-final-brief --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_FINAL_BRIEF_FILE,
            "closure_memo": NEXT_FAMILIAR_LAUNCH_EVIDENCE_CLOSURE_MEMO_FILE,
            "completion_ledger": NEXT_FAMILIAR_LAUNCH_EVIDENCE_COMPLETION_LEDGER_FILE,
            "operator_summary": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_SUMMARY_FILE,
            "release_seal": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE,
            "brief_target": "dx-cli-zed-launch-closeout-pointer",
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_operator_runbook": {
            "schema": "dx.launch.evidence_operator_runbook",
            "command": "dx forge launch-evidence-operator-runbook --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RUNBOOK_FILE,
            "final_brief": NEXT_FAMILIAR_LAUNCH_EVIDENCE_FINAL_BRIEF_FILE,
            "closure_memo": NEXT_FAMILIAR_LAUNCH_EVIDENCE_CLOSURE_MEMO_FILE,
            "completion_ledger": NEXT_FAMILIAR_LAUNCH_EVIDENCE_COMPLETION_LEDGER_FILE,
            "operator_summary": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_SUMMARY_FILE,
            "release_seal": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE,
            "runbook_target": "restartable-dx-worker-checklist",
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_handoff_capsule": {
            "schema": "dx.launch.evidence_handoff_capsule",
            "command": "dx forge launch-evidence-handoff-capsule --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_CAPSULE_FILE,
            "operator_runbook": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RUNBOOK_FILE,
            "final_brief": NEXT_FAMILIAR_LAUNCH_EVIDENCE_FINAL_BRIEF_FILE,
            "closure_memo": NEXT_FAMILIAR_LAUNCH_EVIDENCE_CLOSURE_MEMO_FILE,
            "completion_ledger": NEXT_FAMILIAR_LAUNCH_EVIDENCE_COMPLETION_LEDGER_FILE,
            "capsule_target": "dx-cli-zed-restart-artifact",
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_resumption_index": {
            "schema": "dx.launch.evidence_resumption_index",
            "command": "dx forge launch-evidence-resumption-index --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESUMPTION_INDEX_FILE,
            "handoff_capsule": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_CAPSULE_FILE,
            "operator_runbook": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RUNBOOK_FILE,
            "final_brief": NEXT_FAMILIAR_LAUNCH_EVIDENCE_FINAL_BRIEF_FILE,
            "resumption_target": "ordered-dx-cli-zed-restart-lanes",
            "lanes": ["source-only", "runtime-approved", "release-closeout"],
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_recovery_brief": {
            "schema": "dx.launch.evidence_recovery_brief",
            "command": "dx forge launch-evidence-recovery-brief --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RECOVERY_BRIEF_FILE,
            "resumption_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESUMPTION_INDEX_FILE,
            "handoff_capsule": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_CAPSULE_FILE,
            "operator_runbook": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RUNBOOK_FILE,
            "final_brief": NEXT_FAMILIAR_LAUNCH_EVIDENCE_FINAL_BRIEF_FILE,
            "recovery_target": "human-readable-dx-worker-restart-brief",
            "format": "markdown",
            "zed_openable": true,
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_continuation_packet": {
            "schema": "dx.launch.evidence_continuation_packet",
            "command": "dx forge launch-evidence-continuation-packet --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_CONTINUATION_PACKET_FILE,
            "recovery_brief": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RECOVERY_BRIEF_FILE,
            "resumption_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESUMPTION_INDEX_FILE,
            "handoff_capsule": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_CAPSULE_FILE,
            "operator_runbook": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RUNBOOK_FILE,
            "continuation_target": "dx-cli-zed-continuation-packet",
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_operator_resume_card": {
            "schema": "dx.launch.evidence_operator_resume_card",
            "command": "dx forge launch-evidence-operator-resume-card --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RESUME_CARD_FILE,
            "continuation_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_CONTINUATION_PACKET_FILE,
            "recovery_brief": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RECOVERY_BRIEF_FILE,
            "resume_target": "terminal-first-dx-resume-card",
            "display_mode": "terminal-first",
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_restart_ledger": {
            "schema": "dx.launch.evidence_restart_ledger",
            "command": "dx forge launch-evidence-restart-ledger --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_LEDGER_FILE,
            "operator_resume_card": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RESUME_CARD_FILE,
            "continuation_packet": NEXT_FAMILIAR_LAUNCH_EVIDENCE_CONTINUATION_PACKET_FILE,
            "recovery_brief": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RECOVERY_BRIEF_FILE,
            "resumption_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESUMPTION_INDEX_FILE,
            "ledger_target": "durable-dx-restart-ledger",
            "display_mode": "terminal-first",
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_restart_checklist": {
            "schema": "dx.launch.evidence_restart_checklist",
            "command": "dx forge launch-evidence-restart-checklist --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CHECKLIST_FILE,
            "restart_ledger": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_LEDGER_FILE,
            "operator_resume_card": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RESUME_CARD_FILE,
            "checklist_target": "dx-cli-zed-restart-next-actions",
            "lanes": ["source-only", "runtime-approved", "release-closeout"],
            "display_mode": "terminal-first",
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_restart_brief": {
            "schema": "dx.launch.evidence_restart_brief",
            "command": "dx forge launch-evidence-restart-brief --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_BRIEF_FILE,
            "restart_checklist": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CHECKLIST_FILE,
            "restart_ledger": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_LEDGER_FILE,
            "brief_target": "zed-openable-dx-restart-brief",
            "format": "markdown",
            "zed_openable": true,
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_restart_manifest": {
            "schema": "dx.launch.evidence_restart_manifest",
            "command": "dx forge launch-evidence-restart-manifest --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_MANIFEST_FILE,
            "restart_brief": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_BRIEF_FILE,
            "restart_checklist": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CHECKLIST_FILE,
            "restart_ledger": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_LEDGER_FILE,
            "operator_resume_card": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RESUME_CARD_FILE,
            "manifest_target": "dx-cli-zed-indexable-restart-manifest",
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_restart_receipt": {
            "schema": "dx.launch.evidence_restart_receipt",
            "command": "dx forge launch-evidence-restart-receipt --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_RECEIPT_FILE,
            "restart_manifest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_MANIFEST_FILE,
            "receipt_target": "latest-resumable-dx-zed-handoff",
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_restart_summary": {
            "schema": "dx.launch.evidence_restart_summary",
            "command": "dx forge launch-evidence-restart-summary --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SUMMARY_FILE,
            "restart_receipt": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_RECEIPT_FILE,
            "restart_manifest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_MANIFEST_FILE,
            "restart_brief": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_BRIEF_FILE,
            "restart_checklist": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CHECKLIST_FILE,
            "summary_target": "terminal-friendly-dx-zed-restart-handoff",
            "display_mode": "terminal-first",
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_restart_snapshot": {
            "schema": "dx.launch.evidence_restart_snapshot",
            "command": "dx forge launch-evidence-restart-snapshot --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SNAPSHOT_FILE,
            "restart_summary": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SUMMARY_FILE,
            "restart_receipt": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_RECEIPT_FILE,
            "restart_manifest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_MANIFEST_FILE,
            "restart_brief": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_BRIEF_FILE,
            "snapshot_target": "latest-openable-dx-zed-restart-file",
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_restart_dispatch": {
            "schema": "dx.launch.evidence_restart_dispatch",
            "command": "dx forge launch-evidence-restart-dispatch --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_DISPATCH_FILE,
            "restart_snapshot": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SNAPSHOT_FILE,
            "restart_summary": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SUMMARY_FILE,
            "restart_receipt": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_RECEIPT_FILE,
            "restart_manifest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_MANIFEST_FILE,
            "dispatch_target": "one-command-next-worker-dispatch-card",
            "display_mode": "next-worker-card",
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_restart_closeout": {
            "schema": "dx.launch.evidence_restart_closeout",
            "command": "dx forge launch-evidence-restart-closeout --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CLOSEOUT_FILE,
            "restart_dispatch": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_DISPATCH_FILE,
            "closeout_target": "final-friday-essencefromexistence-closeout-actions",
            "format": "markdown",
            "zed_openable": true,
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_restart_signoff": {
            "schema": "dx.launch.evidence_restart_signoff",
            "command": "dx forge launch-evidence-restart-signoff --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SIGNOFF_FILE,
            "restart_closeout": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CLOSEOUT_FILE,
            "signoff_target": "friday-essencefromexistence-acceptance-receipt",
            "acceptance_status": "reviewable",
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_acceptance_index": {
            "schema": "dx.launch.evidence_acceptance_index",
            "command": "dx forge launch-evidence-acceptance-index --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ACCEPTANCE_INDEX_FILE,
            "restart_signoff": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SIGNOFF_FILE,
            "restart_closeout": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CLOSEOUT_FILE,
            "restart_dispatch": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_DISPATCH_FILE,
            "restart_snapshot": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SNAPSHOT_FILE,
            "acceptance_target": "friday-final-handoff-index",
            "format": "markdown",
            "zed_openable": true,
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_acceptance_digest": {
            "schema": "dx.launch.evidence_acceptance_digest",
            "command": "dx forge launch-evidence-acceptance-digest --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ACCEPTANCE_DIGEST_FILE,
            "acceptance_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ACCEPTANCE_INDEX_FILE,
            "digest_target": "friday-terminal-final-status-line",
            "display_mode": "terminal-first-final-status",
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_evidence_friday_baton": {
            "schema": "dx.launch.evidence_friday_baton",
            "command": "dx forge launch-evidence-friday-baton --project <path> --write",
            "report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_FRIDAY_BATON_FILE,
            "acceptance_digest": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ACCEPTANCE_DIGEST_FILE,
            "acceptance_index": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ACCEPTANCE_INDEX_FILE,
            "restart_signoff": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SIGNOFF_FILE,
            "launch_verification_lane": NEXT_FAMILIAR_LAUNCH_VERIFICATION_LANE_FILE,
            "baton_target": "friday-orchestrator-final-handoff",
            "reads_runtime_artifact_contents": false,
            "no_execution": true
        },
        "launch_verification_lane": launch_verification_lane_contract(),
        "no_execution": true
    })
}
