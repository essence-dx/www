use std::path::Path;

use chrono::Utc;

use crate::error::{DxError, DxResult};

use super::command_output::write_rendered_output_path;
use super::default_template_contract::default_www_template_architecture_contract;
use super::options::DxOutputFormat;
use super::template_options::{
    DxTemplatesCatalogOptions, DxTemplatesVerifyReadinessOptions, parse_templates_catalog_options,
    parse_templates_verify_readiness_options,
};
use super::{
    FORGE_WWW_TEMPLATE_PACKAGE_IDS, NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE,
    NEXT_FAMILIAR_LAUNCH_RECEIPT_DOC, NEXT_FAMILIAR_LAUNCH_RECEIPT_GLOB,
    NEXT_FAMILIAR_LAUNCH_RECEIPT_PACKAGE_ID, NEXT_FAMILIAR_LAUNCH_RECEIPT_VARIANT, forge_error,
    launch_adoption_report_contract, launch_companion_doc_receipts_contract,
    launch_companion_receipts_contract, launch_discovery_contract, launch_manifest_drift_contract,
    launch_readiness_bundle_contract, launch_runtime_approval_request_contract,
    launch_runtime_checklist_contract, launch_runtime_evidence_contract,
    launch_verification_lane_contract, launch_zed_template_handoff_contract, template_readiness,
    www_template_catalog_metadata,
};

pub(super) fn cmd_templates(cwd: &Path, args: &[String]) -> DxResult<()> {
    if args.first().map(String::as_str) == Some("verify-readiness") {
        return cmd_templates_verify_readiness(cwd, &args[1..]);
    }
    cmd_templates_catalog(cwd, args)
}
pub(super) fn cmd_templates_verify_readiness(cwd: &Path, args: &[String]) -> DxResult<()> {
    let options: DxTemplatesVerifyReadinessOptions =
        parse_templates_verify_readiness_options(cwd, args)?;
    let project = options.project;
    let output = options.output;
    let format = options.format;
    let quiet = options.quiet;

    let report = template_readiness::verify_template_readiness(&project).map_err(forge_error)?;
    let rendered = match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Markdown => template_readiness::template_readiness_markdown(&report),
        DxOutputFormat::Terminal => template_readiness::template_readiness_terminal(&report),
    };

    write_rendered_output_path(output, &rendered, quiet, "templates verify-readiness")?;

    if !report.passed {
        return Err(DxError::ConfigValidationError {
            message: format!("Template readiness failed with score {}", report.score),
            field: Some("templates.verify-readiness".to_string()),
        });
    }

    Ok(())
}

fn cmd_templates_catalog(cwd: &Path, args: &[String]) -> DxResult<()> {
    let DxTemplatesCatalogOptions {
        output,
        format,
        quiet,
    } = parse_templates_catalog_options(cwd, args)?;

    let www_package_catalog = www_template_catalog_metadata();
    let discovery = launch_discovery_contract();
    let report = serde_json::json!({
        "schema": "dx.www.templates",
        "generated_at": Utc::now().to_rfc3339(),
        "source": "dx-www",
        "discovery": discovery,
        "architecture_contract": default_www_template_architecture_contract(),
        "zed_template_handoff": launch_zed_template_handoff_contract(),
        "launch_readiness_bundle": launch_readiness_bundle_contract(),
        "launch_adoption_report": launch_adoption_report_contract(),
        "launch_manifest_drift": launch_manifest_drift_contract(),
        "launch_companion_doc_receipts": launch_companion_doc_receipts_contract(),
        "launch_companion_receipts": launch_companion_receipts_contract(),
        "launch_runtime_checklist": launch_runtime_checklist_contract(),
        "launch_runtime_approval_request": launch_runtime_approval_request_contract(),
        "launch_runtime_evidence": launch_runtime_evidence_contract(),
        "launch_verification_lane": launch_verification_lane_contract(),
        "templates": [
            {
                "id": "next-familiar-www-template",
                "label": "Next-familiar DX-WWW app",
                "description": "Source-owned App Router shaped starter with Forge packages, Forge UI folders, and no node_modules requirement.",
                "discovery": launch_discovery_contract(),
                "architecture_contract": default_www_template_architecture_contract(),
                "zed_template_handoff": launch_zed_template_handoff_contract(),
                "launch_readiness_bundle": launch_readiness_bundle_contract(),
                "launch_adoption_report": launch_adoption_report_contract(),
                "launch_manifest_drift": launch_manifest_drift_contract(),
                "launch_companion_doc_receipts": launch_companion_doc_receipts_contract(),
                "launch_companion_receipts": launch_companion_receipts_contract(),
                "launch_runtime_checklist": launch_runtime_checklist_contract(),
                "launch_runtime_approval_request": launch_runtime_approval_request_contract(),
                "launch_runtime_evidence": launch_runtime_evidence_contract(),
                "launch_verification_lane": launch_verification_lane_contract(),
                "forge_packages": FORGE_WWW_TEMPLATE_PACKAGE_IDS,
                "primary_slice": "shadcn/ui/input",
                "recommended_commands": [
                    "dx new <name>",
                    "dx add ui/badge --write",
                    "dx add ui/card --write",
                    "dx add ui/alert --write",
                    "dx add ui/avatar --write",
                    "dx add ui/skeleton --write",
                    "dx add ui/label --write",
                    "dx add ui/separator --write",
                    "dx add ui/field --write",
                    "dx add ui/item --write",
                    "dx add ui/input --write",
                    "dx add ui/textarea --write",
                    "dx add better-auth --write",
                    "dx add next-intl --write",
                    "dx add forms --write",
                    "dx add zod --write",
                    "dx add trpc --write",
                    "dx add ai-sdk --write",
                    "dx add payments --write",
                    "dx add motion-animation --write",
                    "dx add instantdb/react --write",
                    "dx add webassembly-bridge --write",
                    "dx add supabase/client --write",
                    "dx add content/fumadocs-next --write",
                    "dx add react-markdown --write",
                    "dx forge packages --json"
                ],
                "usage_example": {
                    "file": "components/local/LeadCapture.tsx",
                    "imports": ["import { Input } from \"@/components/ui/input\";"],
                    "tsx": "<Input type=\"email\" placeholder=\"you@example.com\" />"
                },
                "usage_examples": [
                    {
                        "package": "shadcn/ui/button",
                        "file": "components/template-app/shadcn-dashboard-controls.tsx",
                        "imports": [
                            "import { LaunchShadcnDashboardControls } from \"@/components/template-app/shadcn-dashboard-controls\";",
                            "import { shadcnTemplateDashboardMetadata } from \"@/components/template-app/shadcn-dashboard-controls-contract\";"
                        ],
                        "tsx": "<LaunchShadcnDashboardControls />",
                        "selector": "[data-dx-component=\"shadcn-dashboard-controls\"]",
                        "receipt": "examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json",
                        "workflow": "operator-dashboard-controls",
                        "keyboard": "arrow-roving-focus",
                        "target_focus": "mission-control-card",
                        "contract": "components/template-app/shadcn-dashboard-controls-contract.tsx"
                    },
                    {
                        "package": "shadcn/ui/badge",
                        "file": "components/local/LaunchStatus.tsx",
                        "imports": ["import { Badge } from \"@/components/ui/badge\";"],
                        "tsx": "<Badge variant=\"secondary\">Source-owned</Badge>"
                    },
                    {
                        "package": "shadcn/ui/alert",
                        "file": "components/local/LaunchNotice.tsx",
                        "imports": ["import { Alert, AlertDescription, AlertTitle } from \"@/components/ui/alert\";"],
                        "tsx": "<Alert><AlertTitle>Source-owned</AlertTitle><AlertDescription>Forge materialized this UI source.</AlertDescription></Alert>"
                    },
                    {
                        "package": "shadcn/ui/avatar",
                        "file": "components/local/AuthorAvatar.tsx",
                        "imports": ["import { Avatar, AvatarFallback, AvatarImage } from \"@/components/ui/avatar\";"],
                        "tsx": "<Avatar><AvatarImage src=\"/avatar.png\" alt=\"\" /><AvatarFallback>DX</AvatarFallback></Avatar>"
                    },
                    {
                        "package": "shadcn/ui/skeleton",
                        "file": "components/local/LoadingPreview.tsx",
                        "imports": ["import { Skeleton } from \"@/components/ui/skeleton\";"],
                        "tsx": "<Skeleton className=\"h-8 w-full\" />"
                    },
                    {
                        "package": "shadcn/ui/label",
                        "file": "components/local/LeadCapture.tsx",
                        "imports": ["import { Label } from \"@/components/ui/label\";"],
                        "tsx": "<Label htmlFor=\"launch-notes\">Launch notes</Label>"
                    },
                    {
                        "package": "shadcn/ui/separator",
                        "file": "components/template-app/template-shell.tsx",
                        "imports": ["import { Separator } from \"@/components/ui/separator\";"],
                        "tsx": "<Separator decorative />"
                    },
                    {
                        "package": "shadcn/ui/field",
                        "file": "components/template-app/template-lead-form.tsx",
                        "imports": ["import { Field, FieldDescription, FieldError, FieldGroup, FieldLabel } from \"@/components/ui/field\";"],
                        "tsx": "<Field><FieldLabel htmlFor=\"launch-notes\">Launch notes</FieldLabel><FieldDescription>Optional launch context.</FieldDescription><FieldError errors={[form.formState.errors.notes]} /></Field>"
                    },
                    {
                        "package": "shadcn/ui/item",
                        "file": "components/template-app/template-shell.tsx",
                        "imports": ["import { Item, ItemContent, ItemTitle } from \"@/components/ui/item\";"],
                        "tsx": "<Item><ItemContent><ItemTitle>shadcn/ui/item</ItemTitle></ItemContent></Item>"
                    },
                    {
                        "package": "shadcn/ui/input",
                        "file": "components/local/LeadCapture.tsx",
                        "imports": ["import { Input } from \"@/components/ui/input\";"],
                        "tsx": "<Input type=\"email\" placeholder=\"you@example.com\" />"
                    },
                    {
                        "package": "shadcn/ui/textarea",
                        "file": "components/local/LeadCapture.tsx",
                        "imports": ["import { Textarea } from \"@/components/ui/textarea\";"],
                        "tsx": "<Textarea placeholder=\"Launch notes\" />"
                    },
                    {
                        "package": "auth/better-auth",
                        "file": "examples/template/auth-session-status.tsx",
                        "imports": ["import { signOut, useSession } from \"@/auth/better-auth/client\";"],
                        "tsx": "const { data, isPending } = useSession();"
                    },
                    {
                        "package": "forms/react-hook-form",
                        "file": "components/local/LeadCapture.tsx",
                        "imports": [
                            "import { DxHookForm } from \"@/lib/forms/react-hook-form/form\";",
                            "import { DxInputField } from \"@/lib/forms/react-hook-form/fields\";",
                            "import { createDxZodResolver } from \"@/lib/forms/react-hook-form/resolver\";"
                        ],
                        "tsx": "<DxHookForm options={{ resolver: createDxZodResolver(schema) }} onSubmit={saveLead}><DxInputField name=\"email\" type=\"email\" label=\"Email\" /></DxHookForm>"
                    },
                    {
                        "package": "validation/zod",
                        "file": "examples/template/zod-validation-status.tsx",
                        "imports": ["import { dxLaunchSignupSchemaWithMetadata, readDxLaunchSchemaMetadata } from \"@/lib/validation/zod/registry\";", "import { validateDxInput } from \"@/lib/validation/zod/parse\";", "import { dxToJsonSchema } from \"@/lib/validation/zod/json-schema\";", "import { safeParseDxLaunchExternalPackage } from \"@/lib/validation/zod/json-schema-import\";", "import { decodeDxIsoDate, safeEncodeDxIsoDate } from \"@/lib/validation/zod/codecs\";", "import { parseDxLaunchSearchParams } from \"@/lib/validation/zod/coerce\";", "import { parseDxLaunchEnvFlags } from \"@/lib/validation/zod/env\";", "import { configureDxZodEnglishLocale, safeParseDxLaunchSignupForDisplay } from \"@/lib/validation/zod/errors\";", "import { createDxLaunchAssetFileProbe, safeParseDxLaunchAssetFile } from \"@/lib/validation/zod/files\";", "import { safeParseDxLaunchSignupSubmission } from \"@/lib/validation/zod/objects\";", "import { parseDxLaunchRoutePath } from \"@/lib/validation/zod/patterns\";", "import { parseDxLaunchScoreInput } from \"@/lib/validation/zod/transforms\";", "import { parseDxLaunchPackageCatalog, summarizeDxLaunchPackageCatalog } from \"@/lib/validation/zod/catalog\";", "import { formatDxLaunchApprovalIssues, safeParseDxLaunchApprovalGate } from \"@/lib/validation/zod/refinements\";", "import { createDxDashboardSettingsReceipt, dxDashboardSettingsExample, safeParseDxDashboardSettingsForm } from \"@/lib/validation/zod/dashboard-settings\";"],
                        "tsx": "validateDxInput(dxLaunchSignupSchemaWithMetadata, input); dxToJsonSchema(dxLaunchSignupSchemaWithMetadata); readDxLaunchSchemaMetadata(dxLaunchSignupSchemaWithMetadata); safeEncodeDxIsoDate(decodeDxIsoDate(checkedAt)); safeParseDxLaunchExternalPackage(externalPackage); parseDxLaunchSearchParams(searchParams); parseDxLaunchEnvFlags(env); configureDxZodEnglishLocale(); safeParseDxLaunchSignupForDisplay(input); safeParseDxLaunchSignupSubmission(input); safeParseDxLaunchAssetFile(file); createDxLaunchAssetFileProbe(); parseDxLaunchRoutePath(\"/launch\"); parseDxLaunchScoreInput(score); parseDxLaunchPackageCatalog(catalog); summarizeDxLaunchPackageCatalog(catalog); safeParseDxLaunchApprovalGate(gate); formatDxLaunchApprovalIssues(gate); safeParseDxDashboardSettingsForm(dxDashboardSettingsExample); createDxDashboardSettingsReceipt(dxDashboardSettingsExample)"
                    },
                    {
                        "package": "payments/stripe-js",
                        "file": "examples/template/payments-status.tsx",
                        "imports": [
                            "import { readDxStripeClientConfig } from \"@/lib/payments/stripe-js/config\";",
                            "import { createDxStripeDashboardCheckoutRequest, createDxStripeDashboardMissingConfigReceipt, dxStripeDashboardPlans } from \"@/lib/payments/stripe-js/dashboard-checkout\";"
                        ],
                        "tsx": "const request = createDxStripeDashboardCheckoutRequest({ planId: dxStripeDashboardPlans[0].id, checkoutMode: \"hosted\", contact }); createDxStripeDashboardMissingConfigReceipt(request); readDxStripeClientConfig();"
                    },
                    {
                        "package": "animation/motion",
                        "file": "components/template-app/motion-interaction-proof.tsx",
                        "imports": ["import { LaunchMotionInteractionProof } from \"@/components/template-app/motion-interaction-proof\";"],
                        "tsx": "<LaunchMotionInteractionProof />",
                        "selector": "[data-dx-component=\"motion-interaction-proof\"]",
                        "receipt": "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
                        "workflow": "motion-panel-orchestration"
                    },
                    {
                        "package": "i18n/next-intl",
                        "file": "examples/template/next-intl-status.tsx",
                        "imports": ["import { useTranslations } from \"next-intl\";"],
                        "tsx": "const t = useTranslations(\"Launch\");"
                    },
                    {
                        "package": "api/trpc",
                        "file": "examples/template/trpc-launch-health.tsx",
                        "imports": ["import { useTRPC } from \"@/lib/trpc/provider\";"],
                        "tsx": "useQuery(trpc.health.queryOptions())"
                    },
                    {
                        "package": "tanstack/query",
                        "file": "examples/template/query-cache-status.tsx",
                        "imports": ["import { dxQueryOptions } from \"@/lib/query/client\";", "import { useQuery } from \"@tanstack/react-query\";"],
                        "tsx": "useQuery(dxQueryOptions({ queryKey, queryFn, staleTime: 60_000 }))"
                    },
                    {
                        "package": "ai/vercel-ai",
                        "file": "examples/template/ai-chat-status.tsx",
                        "imports": ["import { DxAIClientChat } from \"@/lib/ai/client-chat\";"],
                        "tsx": "<DxAIClientChat api=\"/api/ai/chat\" initialMessages={launchAiInitialMessages} />"
                    },
                    {
                        "package": "instantdb/react",
                        "file": "examples/template/instantdb-status.tsx",
                        "imports": ["import { db, launchRoom } from \"@/lib/instant/client\";", "import { LaunchInstantStatus } from \"./instantdb-status\";"],
                        "tsx": "<LaunchInstantStatus db={db} room={launchRoom} />"
                    },
                    {
                        "package": "wasm/bindgen",
                        "file": "examples/template/wasm-interop-status.tsx",
                        "imports": ["import { useWasmBindgenModule } from \"@/wasm/bindgen/react\";", "import type { WasmBindgenFactory, WasmBindgenInput } from \"@/wasm/bindgen/loader\";"],
                        "tsx": "useWasmBindgenModule({ cacheKey: \"dx-launch-wasm-status\", importModule, input, enabled: Boolean(importModule) })"
                    },
                    {
                        "package": "3d/launch-scene",
                        "file": "examples/template/launch-scene.tsx",
                        "imports": ["import { createDxLaunchScenePreset, dxSceneQualityProfiles } from \"@/lib/scene/preset\";", "import { createDxSceneDashboardWorkflow, createDxSceneDashboardReceipt } from \"@/lib/scene/dashboard-workflow\";", "import { cycleDxSceneCameraRig, cycleDxSceneMaterialPalette, cycleDxSceneQualityProfile } from \"@/lib/scene/dashboard-controls\";", "import { captureDxSceneFrameSample } from \"@/lib/scene/frame-sample\";", "import { createDxSceneCapabilityReport } from \"@/lib/scene/capability-report\";", "import { createDxSceneViewportReport } from \"@/lib/scene/viewport-report\";", "import { createDxSceneBoundsReport } from \"@/lib/scene/bounds-report\";", "import { createDxSceneRaycastReport } from \"@/lib/scene/raycast-report\";", "import { mountDxSceneWithRenderer } from \"@/lib/scene/renderer-handoff\";"],
                        "tsx": "<LaunchScene />"
                    },
                    {
                        "package": "supabase/client",
                        "file": "examples/template/supabase-profile-workflow.tsx",
                        "imports": [
                            "import { readDxSupabaseProfileConfigStatus, createDxSupabaseProfilePreview, createDxSupabaseProfileUpsertReceipt } from \"@/lib/supabase/profile-workflow\";",
                            "import { getDxSupabaseCurrentProfile, upsertDxSupabaseProfile } from \"@/lib/supabase/profiles\";"
                        ],
                        "tsx": "<LaunchSupabaseProfileWorkflow />"
                    },
                    {
                        "package": "db/drizzle-sqlite",
                        "file": "examples/template/drizzle-query-proof.tsx",
                        "imports": [
                            "import { dxDrizzlePackage } from \"@/db/drizzle/metadata\";",
                            "import { readDrizzleDashboardOverview, readDrizzleDashboardQueryPlan } from \"@/db/drizzle/dashboard-workflow\";",
                            "import { LaunchDrizzleDashboardData } from \"@/components/template-app/drizzle-query-proof\";"
                        ],
                        "tsx": "<LaunchDrizzleDashboardData />"
                    },
                    {
                        "package": "content/fumadocs-next",
                        "file": "examples/template/docs-status.tsx",
                        "imports": [
                            "import { dxFumadocsLLMsContract } from \"@/lib/fumadocs/llms\";",
                            "import { dxFumadocsOpenAPICodeUsageContract } from \"@/lib/fumadocs/openapi-code-usage\";",
                            "import { dxFumadocsOpenAPIContract } from \"@/lib/fumadocs/openapi\";",
                            "import { dxFumadocsNavigationContract } from \"@/lib/fumadocs/navigation\";",
                            "import { dxFumadocsRouteContract } from \"@/lib/fumadocs/route-contract\";",
                            "import { dxFumadocsSearchClientContract } from \"@/lib/fumadocs/search-client\";",
                            "import { dxFumadocsSourcePluginContract } from \"@/lib/fumadocs/source-plugins\";",
                            "import { dxFumadocsTocContract } from \"@/lib/fumadocs/toc\";"
                        ],
                        "tsx": "`${dxFumadocsRouteContract.docsRoute} ${dxFumadocsRouteContract.openApiDocsRoute} ${dxFumadocsRouteContract.openApiProxyRoute} ${dxFumadocsOpenAPIContract.allowedOriginsEnv} ${dxFumadocsOpenAPICodeUsageContract.customGenerators[0]} ${dxFumadocsSourcePluginContract.frontmatterFields.join(\",\")} ${dxFumadocsNavigationContract.surfaces.join(\",\")} ${dxFumadocsTocContract.surfaces.join(\",\")} ${dxFumadocsRouteContract.llmsIndexRoute} ${dxFumadocsRouteContract.llmsFullRoute} ${dxFumadocsOpenAPIContract.upstreamApis[0]} ${dxFumadocsLLMsContract.upstreamApi} ${dxFumadocsRouteContract.searchRoute} ${dxFumadocsRouteContract.staticSearchRoute} ${dxFumadocsSearchClientContract.staticPreset}`"
                    },
                    {
                        "package": "content/react-markdown",
                        "file": "components/local/MarkdownPreview.tsx",
                        "imports": ["import { DxMarkdown } from \"@/components/markdown\";"],
                        "tsx": "<DxMarkdown>{markdown}</DxMarkdown>"
                    }
                ],
                "example_files": [
                    "examples/template/app/page.tsx",
                    "examples/template/ai-chat-status.tsx",
                    "examples/template/auth-session-status.tsx",
                    "examples/template/data-status.tsx",
                    "examples/template/supabase-profile-workflow-state.ts",
                    "examples/template/supabase-profile-workflow.tsx",
                    "examples/template/payments-status.tsx",
                    "examples/template/docs-status.tsx",
                    "examples/template/template-route-contract.ts",
                    "examples/template/template-shell.tsx",
                    "examples/template/template-dashboard-nav.tsx",
                    "examples/template/dx-studio-edit-contract.ts",
                    "examples/template/shadcn-dashboard-controls-contract.tsx",
                    "examples/template/shadcn-dashboard-controls.tsx",
                    "examples/template/automations-status.tsx",
                    "examples/template/automation-mission-summary.tsx",
                    "examples/template/automations/automations-metadata.ts",
                    "examples/template/motion-interaction-proof.tsx",
                    "examples/template/template-lead-form.tsx",
                    "examples/template/launch-scene.tsx",
                    "examples/template/scene/types.ts",
                    "examples/template/scene/preset.ts",
                    "examples/template/scene/webgl-runtime.ts",
                    "examples/template/scene/metadata.ts",
                    "examples/template/scene/README.md",
                    "examples/template/instantdb-status.tsx",
                    "examples/template/icon-status.tsx",
                    "examples/template/next-intl-status.tsx",
                    "examples/template/package-catalog.ts",
                    "examples/template/query-cache-status.tsx",
                    "examples/template/query-dashboard-read-model.ts",
                    "examples/template/forge-package-status.ts",
                    "examples/template/forge-package-status-read-model.ts",
                    "examples/template/preview-style-package-panel-read-model.ts",
                    "examples/template/preview-style-package-ownership-read-model.ts",
                    "examples/template/forge-golden-path-contract.ts",
                    "examples/template/forge-golden-path-panel.tsx",
                    "examples/template/forge-safety-archive-contract.ts",
                    "examples/template/forge-safety-archive-runbook.ts",
                    "examples/template/forge-safety-archive-panel.tsx",
                    "examples/template/forge-remote-head-health-contract.ts",
                    "examples/template/forge-remote-head-health-panel.tsx",
                    "examples/template/react-markdown-preview.tsx",
                    "examples/template/state-zustand-counter.tsx",
                    "examples/template/state-zustand-dashboard.tsx",
                    "examples/template/trpc-launch-contract.ts",
                    "examples/template/trpc-launch-health.tsx",
                    "examples/template/wasm-interop-status.tsx",
                    "examples/template/zod-validation-status.tsx",
                    "examples/template/zod-dashboard-settings.tsx"
                ],
                "www_package_catalog": www_package_catalog,
                "app_router_entrypoint": {
                    "route": "/",
                    "route_aliases": [],
                    "source_file": "examples/template/app/page.tsx",
                    "materialized_file": "app/page.tsx",
                    "runtime_component_materialized_file": "components/template-app/template-console.tsx",
                    "runtime_catalog_materialized_file": "server/templateCatalog.ts",
                    "contract_source_file": "examples/template/template-route-contract.ts",
                    "contract_materialized_file": "components/template-app/template-route-contract.ts",
                    "source_smoke_command": "dx run --test .\\benchmarks\\template-shell.test.ts",
                    "runtime_verification": "pending-governed-runtime-pass",
                    "runtime_verification_requires_explicit_permission": true,
                    "runtime_verification_request": {
                        "approval_status": "requires-explicit-permission",
                        "automation_default": "skip-runtime-build-preview",
                        "expected_evidence": [
                            "governed-runtime-route-response",
                            "production-contract-route-proof",
                            "final-launch-evidence-receipt"
                        ],
                        "no_execution": true
                    },
                    "template_readiness_receipt": {
                        "package": NEXT_FAMILIAR_LAUNCH_RECEIPT_PACKAGE_ID,
                        "variant": NEXT_FAMILIAR_LAUNCH_RECEIPT_VARIANT,
                        "file": NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE,
                        "receipt_glob": NEXT_FAMILIAR_LAUNCH_RECEIPT_GLOB,
                        "docs_file": NEXT_FAMILIAR_LAUNCH_RECEIPT_DOC,
                        "status": "source-materialized-runtime-pending",
                        "source_smoke_command": "dx run --test .\\benchmarks\\template-shell.test.ts",
                        "runtime_verification": "pending-governed-runtime-pass",
                        "runtime_verification_requires_explicit_permission": true,
                        "runtime_verification_request": {
                            "approval_status": "requires-explicit-permission",
                            "automation_default": "skip-runtime-build-preview",
                            "expected_evidence": [
                                "governed-runtime-route-response",
                                "production-contract-route-proof",
                                "final-launch-evidence-receipt"
                            ],
                            "no_execution": true
                        }
                    },
                    "companion_documentation_receipts": launch_companion_doc_receipts_contract(),
                    "generated_template_files": [
                        {
                            "kind": "app-router-default-route",
                            "source_file": "examples/template/app/page.tsx",
                            "materialized_file": "app/page.tsx"
                        },
                        {
                            "kind": "route-contract",
                            "source_file": "examples/template/template-route-contract.ts",
                            "materialized_file": "components/template-app/template-route-contract.ts"
                        },
                        {
                            "kind": "template-shell",
                            "source_file": "examples/template/template-shell.tsx",
                            "materialized_file": "components/template-app/template-shell.tsx"
                        },
                        {
                            "kind": "template-dashboard-nav",
                            "source_file": "examples/template/template-dashboard-nav.tsx",
                            "materialized_file": "components/template-app/template-dashboard-nav.tsx"
                        },
                        {
                            "kind": "dx-studio-edit-contract",
                            "source_file": "examples/template/dx-studio-edit-contract.ts",
                            "materialized_file": "components/template-app/dx-studio-edit-contract.ts"
                        },
                        {
                            "kind": "shadcn-dashboard-controls-contract",
                            "source_file": "examples/template/shadcn-dashboard-controls-contract.tsx",
                            "materialized_file": "components/template-app/shadcn-dashboard-controls-contract.tsx"
                        },
                        {
                            "kind": "shadcn-dashboard-controls",
                            "source_file": "examples/template/shadcn-dashboard-controls.tsx",
                            "materialized_file": "components/template-app/shadcn-dashboard-controls.tsx"
                        },
                        {
                            "kind": "automations-status",
                            "source_file": "examples/template/automations-status.tsx",
                            "materialized_file": "components/template-app/automations-status.tsx"
                        },
                        {
                            "kind": "automation-mission-summary",
                            "source_file": "examples/template/automation-mission-summary.tsx",
                            "materialized_file": "components/template-app/automation-mission-summary.tsx"
                        },
                        {
                            "kind": "automations-metadata",
                            "source_file": "examples/template/automations/automations-metadata.ts",
                            "materialized_file": "components/template-app/automations/automations-metadata.ts"
                        },
                        {
                            "kind": "motion-interaction-proof",
                            "source_file": "examples/template/motion-interaction-proof.tsx",
                            "materialized_file": "components/template-app/motion-interaction-proof.tsx"
                        },
                        {
                            "kind": "template-lead-form",
                            "source_file": "examples/template/template-lead-form.tsx",
                            "materialized_file": "components/template-app/template-lead-form.tsx"
                        },
                        {
                            "kind": "auth-session-status",
                            "source_file": "examples/template/auth-session-status.tsx",
                            "materialized_file": "components/template-app/auth-session-status.tsx"
                        },
                        {
                            "kind": "ai-chat-status",
                            "source_file": "examples/template/ai-chat-status.tsx",
                            "materialized_file": "components/template-app/ai-chat-status.tsx"
                        },
                        {
                            "kind": "data-status",
                            "source_file": "examples/template/data-status.tsx",
                            "materialized_file": "components/template-app/data-status.tsx"
                        },
                        {
                            "kind": "supabase-profile-workflow-state",
                            "source_file": "examples/template/supabase-profile-workflow-state.ts",
                            "materialized_file": "lib/supabase/profile-workflow.ts"
                        },
                        {
                            "kind": "supabase-profile-workflow",
                            "source_file": "examples/template/supabase-profile-workflow.tsx",
                            "materialized_file": "components/template-app/supabase-profile-workflow.tsx"
                        },
                        {
                            "kind": "supabase-dashboard-workflow-receipt",
                            "source_file": "examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json",
                            "materialized_file": ".dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json"
                        },
                        {
                            "kind": "payments-status",
                            "source_file": "examples/template/payments-status.tsx",
                            "materialized_file": "components/template-app/payments-status.tsx"
                        },
                        {
                            "kind": "docs-status",
                            "source_file": "examples/template/docs-status.tsx",
                            "materialized_file": "components/template-app/docs-status.tsx"
                        },
                        {
                            "kind": "realtime-data-status",
                            "source_file": "examples/template/instantdb-status.tsx",
                            "materialized_file": "components/template-app/instantdb-status.tsx"
                        },
                        {
                            "kind": "wasm-interop-status",
                            "source_file": "examples/template/wasm-interop-status.tsx",
                            "materialized_file": "components/template-app/wasm-interop-status.tsx"
                        },
                        {
                            "kind": "validation-status",
                            "source_file": "examples/template/zod-validation-status.tsx",
                            "materialized_file": "components/template-app/zod-validation-status.tsx"
                        },
                        {
                            "kind": "validation-dashboard-settings",
                            "source_file": "examples/template/zod-dashboard-settings.tsx",
                            "materialized_file": "components/template-app/zod-dashboard-settings.tsx"
                        },
                        {
                            "kind": "package-catalog",
                            "source_file": "examples/template/package-catalog.ts",
                            "materialized_file": "components/template-app/package-catalog.ts"
                        },
                        {
                            "kind": "state-preview",
                            "source_file": "examples/template/state-zustand-counter.tsx",
                            "materialized_file": "components/template-app/state-zustand-counter.tsx"
                        },
                        {
                            "kind": "state-dashboard-workflow",
                            "source_file": "examples/template/state-zustand-dashboard.tsx",
                            "materialized_file": "components/template-app/state-zustand-dashboard.tsx"
                        },
                        {
                            "kind": "state-dashboard-workflow-receipt",
                            "source_file": "examples/template/.dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json",
                            "materialized_file": ".dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json"
                        },
                        {
                            "kind": "content-preview",
                            "source_file": "examples/template/react-markdown-preview.tsx",
                            "materialized_file": "components/template-app/react-markdown-preview.tsx"
                        },
                        {
                            "kind": "typed-api-contract",
                            "source_file": "examples/template/trpc-launch-contract.ts",
                            "materialized_file": "components/template-app/trpc-launch-contract.ts"
                        },
                        {
                            "kind": "typed-api-health",
                            "source_file": "examples/template/trpc-launch-health.tsx",
                            "materialized_file": "components/template-app/trpc-launch-health.tsx"
                        },
                        {
                            "kind": "source-owned-scene",
                            "source_file": "examples/template/launch-scene.tsx",
                            "materialized_file": "components/scene/launch-scene.tsx"
                        },
                        {
                            "kind": "source-owned-scene-entrypoint",
                            "source_file": "examples/template/scene/index.ts",
                            "materialized_file": "lib/scene/index.ts"
                        },
                        {
                            "kind": "source-owned-scene-types",
                            "source_file": "examples/template/scene/types.ts",
                            "materialized_file": "lib/scene/types.ts"
                        },
                        {
                            "kind": "source-owned-scene-preset",
                            "source_file": "examples/template/scene/preset.ts",
                            "materialized_file": "lib/scene/preset.ts"
                        },
                        {
                            "kind": "source-owned-scene-preview-readiness",
                            "source_file": "examples/template/scene/preview-readiness.ts",
                            "materialized_file": "lib/scene/preview-readiness.ts"
                        },
                        {
                            "kind": "source-owned-scene-runtime",
                            "source_file": "examples/template/scene/webgl-runtime.ts",
                            "materialized_file": "lib/scene/webgl-runtime.ts"
                        },
                        {
                            "kind": "source-owned-scene-metadata",
                            "source_file": "examples/template/scene/metadata.ts",
                            "materialized_file": "lib/scene/metadata.ts"
                        },
                        {
                            "kind": "source-owned-scene-readme",
                            "source_file": "examples/template/scene/README.md",
                            "materialized_file": "lib/scene/README.md"
                        },
                        {
                            "kind": "icon-status",
                            "source_file": "examples/template/icon-status.tsx",
                            "materialized_file": "components/template-app/icon-status.tsx"
                        },
                        {
                            "kind": "i18n-status",
                            "source_file": "examples/template/next-intl-status.tsx",
                            "materialized_file": "components/template-app/next-intl-status.tsx"
                        },
                        {
                            "kind": "query-cache-status",
                            "source_file": "examples/template/query-cache-status.tsx",
                            "materialized_file": "components/template-app/query-cache-status.tsx"
                        },
                        {
                            "kind": "query-dashboard-read-model",
                            "source_file": "examples/template/query-dashboard-read-model.ts",
                            "materialized_file": "components/template-app/query-dashboard-read-model.ts"
                        },
                        {
                            "kind": "forge-package-status",
                            "source_file": "examples/template/forge-package-status.ts",
                            "materialized_file": "components/template-app/forge-package-status.ts"
                        },
                        {
                            "kind": "forge-package-status-read-model",
                            "source_file": "examples/template/forge-package-status-read-model.ts",
                            "materialized_file": "components/template-app/forge-package-status-read-model.ts"
                        },
                        {
                            "kind": "forge-golden-path-contract",
                            "source_file": "examples/template/forge-golden-path-contract.ts",
                            "materialized_file": "components/template-app/forge-golden-path-contract.ts"
                        },
                        {
                            "kind": "forge-golden-path-panel",
                            "source_file": "examples/template/forge-golden-path-panel.tsx",
                            "materialized_file": "components/template-app/forge-golden-path-panel.tsx"
                        },
                        {
                            "kind": "forge-safety-archive-contract",
                            "source_file": "examples/template/forge-safety-archive-contract.ts",
                            "materialized_file": "components/template-app/forge-safety-archive-contract.ts"
                        },
                        {
                            "kind": "forge-safety-archive-runbook",
                            "source_file": "examples/template/forge-safety-archive-runbook.ts",
                            "materialized_file": "components/template-app/forge-safety-archive-runbook.ts"
                        },
                        {
                            "kind": "forge-safety-archive-panel",
                            "source_file": "examples/template/forge-safety-archive-panel.tsx",
                            "materialized_file": "components/template-app/forge-safety-archive-panel.tsx"
                        },
                        {
                            "kind": "forge-remote-head-health-contract",
                            "source_file": "examples/template/forge-remote-head-health-contract.ts",
                            "materialized_file": "components/template-app/forge-remote-head-health-contract.ts"
                        },
                        {
                            "kind": "forge-remote-head-health-panel",
                            "source_file": "examples/template/forge-remote-head-health-panel.tsx",
                            "materialized_file": "components/template-app/forge-remote-head-health-panel.tsx"
                        },
                        {
                            "kind": "runtime-console",
                            "source_file": null,
                            "materialized_file": "components/template-app/template-console.tsx"
                        },
                        {
                            "kind": "runtime-catalog-loader",
                            "source_file": null,
                            "materialized_file": "server/templateCatalog.ts"
                        },
                        {
                            "kind": "template-readiness-receipt",
                            "source_file": null,
                            "materialized_file": ".dx/forge/template-readiness/launch-route.json"
                        },
                        {
                            "kind": "launch-readiness-bundle",
                            "source_file": null,
                            "materialized_file": ".dx/forge/template-readiness/launch-readiness-bundle.json"
                        },
                        {
                            "kind": "launch-companion-doc-receipts",
                            "source_file": null,
                            "materialized_file": ".dx/forge/template-readiness/launch-companion-doc-receipts.json"
                        }
                    ],
                    "component_source_file": "examples/template/template-shell.tsx",
                    "component_materialized_file": "components/template-app/template-shell.tsx",
                    "component_source_files": [
                        "examples/template/template-shell.tsx",
                        "examples/template/template-dashboard-nav.tsx",
                        "examples/template/template-lead-form.tsx",
                        "examples/template/template-route-contract.ts",
                        "examples/template/dx-studio-edit-contract.ts",
                        "examples/template/shadcn-dashboard-controls-contract.tsx",
                        "examples/template/shadcn-dashboard-controls.tsx",
                        "examples/template/automations-status.tsx",
                        "examples/template/automation-mission-summary.tsx",
                        "examples/template/automations/automations-metadata.ts",
                        "examples/template/motion-interaction-proof.tsx",
                        "examples/template/ai-chat-status.tsx",
                        "examples/template/auth-session-status.tsx",
                        "examples/template/data-status.tsx",
                        "examples/template/supabase-profile-workflow-state.ts",
                        "examples/template/supabase-profile-workflow.tsx",
                        "examples/template/payments-status.tsx",
                        "examples/template/docs-status.tsx",
                        "examples/template/launch-scene.tsx",
                        "examples/template/instantdb-status.tsx",
                        "examples/template/icon-status.tsx",
                        "examples/template/next-intl-status.tsx",
                        "examples/template/query-cache-status.tsx",
                        "examples/template/query-dashboard-read-model.ts",
                        "examples/template/forge-package-status.ts",
                        "examples/template/forge-package-status-read-model.ts",
                        "examples/template/preview-style-package-panel-read-model.ts",
                        "examples/template/preview-style-package-ownership-read-model.ts",
                        "examples/template/forge-golden-path-contract.ts",
                        "examples/template/forge-golden-path-panel.tsx",
                        "examples/template/forge-safety-archive-contract.ts",
                        "examples/template/forge-safety-archive-runbook.ts",
                        "examples/template/forge-safety-archive-panel.tsx",
                        "examples/template/forge-remote-head-health-contract.ts",
                        "examples/template/forge-remote-head-health-panel.tsx",
                        "examples/template/wasm-interop-status.tsx",
                        "examples/template/zod-validation-status.tsx",
                        "examples/template/zod-dashboard-settings.tsx",
                        "examples/template/package-catalog.ts",
                        "examples/template/state-zustand-counter.tsx",
                        "examples/template/state-zustand-dashboard.tsx",
                        "examples/template/react-markdown-preview.tsx",
                        "examples/template/trpc-launch-contract.ts",
                        "examples/template/trpc-launch-health.tsx",
                        "examples/template/scene/types.ts",
                        "examples/template/scene/preset.ts",
                        "examples/template/scene/webgl-runtime.ts",
                        "examples/template/scene/metadata.ts",
                        "examples/template/scene/README.md"
                    ],
                    "component_materialized_files": [
                        "components/template-app/template-shell.tsx",
                        "components/template-app/template-dashboard-nav.tsx",
                        "components/template-app/template-lead-form.tsx",
                        "components/template-app/template-route-contract.ts",
                        "components/template-app/dx-studio-edit-contract.ts",
                        "components/template-app/automations-status.tsx",
                        "components/template-app/automation-mission-summary.tsx",
                        "components/template-app/automations/automations-metadata.ts",
                        "components/template-app/motion-interaction-proof.tsx",
                        "components/template-app/ai-chat-status.tsx",
                        "components/template-app/auth-session-status.tsx",
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
                        "components/template-app/state-zustand-counter.tsx",
                        "components/template-app/state-zustand-dashboard.tsx",
                        "components/template-app/react-markdown-preview.tsx",
                        "components/template-app/trpc-launch-contract.ts",
                        "components/template-app/trpc-launch-health.tsx",
                        "components/scene/launch-scene.tsx",
                        "lib/scene/index.ts",
                        "lib/scene/types.ts",
                        "lib/scene/preset.ts",
                        "lib/scene/preview-readiness.ts",
                        "lib/scene/webgl-runtime.ts",
                        "lib/scene/metadata.ts",
                        "lib/scene/README.md"
                    ],
                    "provider_package": "i18n/next-intl",
                    "required_packages": [
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
                        "forms/react-hook-form",
                        "validation/zod",
                        "animation/motion",
                        "3d/launch-scene",
                        "state/zustand",
                        "tanstack/query",
                        "i18n/next-intl",
                        "content/react-markdown"
                    ]
                },
                "no_execution": true
            }
        ]
    });

    let rendered = match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Terminal => {
            let mut output = format!(
                "DX-WWW templates\nSchema: {}\nTemplates: {}\n",
                report["schema"].as_str().unwrap_or("dx.www.templates"),
                report["templates"]
                    .as_array()
                    .map(|templates| templates.len())
                    .unwrap_or_default()
            );
            output.push_str("- next-familiar-www-template: dx new <name>, dx add ui/badge --write, dx add ui/card --write, dx add ui/alert --write, dx add ui/avatar --write, dx add ui/skeleton --write, dx add ui/label --write, dx add ui/separator --write, dx add ui/field --write, dx add ui/item --write, dx add ui/input --write, dx add ui/textarea --write, dx add forms --write, dx add payments --write, dx add automation-connectors --write, dx add motion-animation --write, dx add 3d-scene-system --write, dx add supabase/client --write, dx add content/fumadocs-next --write, dx add react-markdown --write\n");
            output
        }
        DxOutputFormat::Markdown => {
            "# DX-WWW Templates\n\n| Template | Primary Slice | Commands |\n| --- | --- | --- |\n| `next-familiar-www-template` | `shadcn/ui/badge`, `shadcn/ui/card`, `shadcn/ui/alert`, `shadcn/ui/avatar`, `shadcn/ui/skeleton`, `shadcn/ui/label`, `shadcn/ui/separator`, `shadcn/ui/field`, `shadcn/ui/item`, `shadcn/ui/input`, `forms/react-hook-form`, `payments/stripe-js`, `automations/n8n`, `animation/motion`, `3d/launch-scene`, `supabase/client`, `content/fumadocs-next`, `content/react-markdown` | `dx new <name>`, `dx add ui/badge --write`, `dx add ui/card --write`, `dx add ui/alert --write`, `dx add ui/avatar --write`, `dx add ui/skeleton --write`, `dx add ui/label --write`, `dx add ui/field --write`, `dx add ui/item --write`, `dx add ui/input --write`, `dx add ui/textarea --write`, `dx add forms --write`, `dx add payments --write`, `dx add automation-connectors --write`, `dx add motion-animation --write`, `dx add 3d-scene-system --write`, `dx add supabase/client --write`, `dx add content/fumadocs-next --write`, `dx add react-markdown --write` |\n".to_string()
        }
    };

    write_rendered_output_path(output, &rendered, quiet, "templates")?;

    Ok(())
}
