const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const proofRoot = path.join(repoRoot, "examples", "conversion-proof");

const projects = [
  {
    id: "shadcn-ui",
    route: "/ui",
    page: "pages/ui.html",
    forge: "website-conversion-shadcn",
    packageId: "shadcn/ui/card",
    receipt: ".dx/forge/receipts/2026-05-21-shadcn-ui-to-ui.json",
    visualAudit: "forge/visual-audits/shadcn-ui.json",
    markers: ["source-owned", "icon registry", "button"],
  },
  {
    id: "supabase",
    route: "/database",
    page: "pages/database.html",
    forge: "website-conversion-supabase",
    packageId: "supabase/client",
    receipt: ".dx/forge/receipts/2026-05-21-supabase-to-database.json",
    visualAudit: "forge/visual-audits/supabase.json",
    markers: ["database", "schema", "realtime"],
  },
  {
    id: "convex-backend",
    route: "/backend",
    page: "pages/backend.html",
    forge: "website-conversion-convex",
    packageId: "supabase/client",
    receipt: ".dx/forge/receipts/2026-05-21-convex-to-backend.json",
    visualAudit: "forge/visual-audits/convex-backend.json",
    markers: ["route", "realtime", "schema"],
  },
];

const primitiveFiles = [
  "forge/primitives/types.ts",
  "forge/primitives/class-merge.ts",
  "forge/primitives/slot.ts",
  "forge/primitives/theme-provider.ts",
  "forge/primitives/button.ts",
  "forge/primitives/input.ts",
  "forge/primitives/badge.ts",
  "forge/primitives/card.ts",
  "forge/primitives/table.ts",
  "forge/primitives/tabs.ts",
  "forge/primitives/dialog.ts",
  "forge/primitives/dropdown.ts",
  "forge/primitives/sidebar.ts",
  "forge/primitives/index.ts",
  "forge/primitives/metadata.json",
];

const shimFiles = [
  "forge/shims/runtime-boundaries.ts",
  "forge/shims/test-adapters.ts",
  "forge/shims/launch-runtime-boundaries.json",
  "forge/shims/README.md",
];

const componentFiles = [
  "components/ConversionRouteHeader.tsx",
  "components/SourceSurfaceTable.tsx",
  "components/RuntimeBoundaryPanel.tsx",
];

const routeDiscoveryPath = "forge/route-discovery/conversion-routes.json";
const routeAcceptancePath = "forge/acceptance/no-runtime-route-acceptance.json";
const renderedProofSchemaPath = "forge/acceptance/rendered-proof-evidence.schema.json";
const renderedProofValidatorPath = "forge/acceptance/validate-rendered-proof-evidence.ts";
const blockedRenderedProofFixturePath = "forge/acceptance/fixtures/blocked-rendered-proof.sample.json";
const renderedProofImportPlanPath = "forge/acceptance/rendered-proof-import-plan.json";
const renderedProofImportToolPath = "forge/acceptance/prepare-rendered-proof-import.ts";
const renderedProofReviewToolPath = "forge/acceptance/review-rendered-proof-completeness.ts";
const renderedProofApprovalRequestPath = "forge/acceptance/rendered-proof-runtime-approval-request.json";
const renderedProofApprovalToolPath = "forge/acceptance/request-rendered-proof-runtime-approval.ts";
const renderedProofAuthoringGuidePath = "forge/acceptance/rendered-proof-evidence-authoring-guide.json";
const renderedProofAuthoringToolPath = "forge/acceptance/summarize-rendered-proof-evidence-requirements.ts";
const requiredSurfaceKinds = ["ui", "layout", "interaction", "docs", "dashboard", "brand"];
const requiredVisualSectionKinds = [
  "brand-header",
  "route-navigation",
  "primary-source-surface",
  "dashboard-workflow",
  "runtime-boundary",
  "responsive-layout",
];

function readProof(relativePath) {
  return fs.readFileSync(path.join(proofRoot, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(readProof(relativePath));
}

test("conversion proof owns DX-WWW routes without Next, TanStack, or React Router launch routing", () => {
  for (const project of projects) {
    const source = readProof(project.page);
    assert.match(source, /<main\b/);
    assert.match(source, new RegExp(`data-dx-route="${project.route}"`));
    assert.match(source, new RegExp(`data-dx-forge="${project.forge}"`));
    assert.match(source, new RegExp(`data-dx-package="${project.packageId}"`));

    for (const marker of project.markers) {
      assert.match(source.toLowerCase(), new RegExp(marker));
    }

    assert.doesNotMatch(
      source,
      /next\/router|next\/link|@tanstack\/router|react-router|createBrowserRouter|getServerSideProps|NextPageWithLayout|<Route\s/,
      `${project.route} must use DX-WWW routing as canonical`,
    );
  }
});

test("conversion proof uses source-owned DX-WWW components for route surfaces", () => {
  for (const componentPath of componentFiles) {
    const source = readProof(componentPath);

    assert.match(source, /export function /);
    assert.match(source, /children/);
    assert.match(source, /DX-WWW conversion proof|source surface|runtime boundary/i);
    assert.doesNotMatch(
      source,
      /require\(["']react["']\)|next\/router|next\/link|@tanstack\/router|react-router/,
      `${componentPath} must stay DX-WWW-owned and dependency-free`,
    );
  }

  for (const project of projects) {
    const page = readProof(project.page);
    assert.match(page, /<main\b/);
    assert.match(page, new RegExp(`data-dx-route="${project.route}"`));
    assert.match(page, /data-dx-package-set=/);
  }

  for (const project of projects) {
    const manifest = readJson(`forge/conversion-manifests/${project.id}.json`);
    for (const componentPath of componentFiles) {
      assert.ok(manifest.converted_files.includes(componentPath));
    }

    const receipt = readJson(project.receipt);
    for (const componentPath of componentFiles) {
      assert.ok(receipt.converted_files.includes(componentPath));
    }
  }
});

test("conversion proof maps source UI, layout, interaction, docs, dashboard, and brand surfaces", () => {
  for (const project of projects) {
    const surfaceMapPath = `forge/source-surfaces/${project.id}.json`;
    const surfaceMap = readJson(surfaceMapPath);

    assert.equal(surfaceMap.schema, "dx.www.source_surface_map");
    assert.equal(surfaceMap.source_project, project.id);
    assert.equal(surfaceMap.converted_route, project.route);
    assert.ok(surfaceMap.source_commit.length >= 12);

    for (const surfaceKind of requiredSurfaceKinds) {
      const surface = surfaceMap.surfaces[surfaceKind];
      assert.ok(surface, `${project.id} missing ${surfaceKind} surface`);
      assert.ok(surface.label.length > 0);
      assert.ok(surface.source_files.length >= 1);
      assert.ok(surface.converted_files.includes(project.page));
      assert.ok(surface.launch_value.length > 0);
      assert.ok(surface.runtime_boundary.length > 0);
    }

    const page = readProof(project.page);
    assert.match(page, new RegExp(`data-surface-map="${surfaceMapPath}"`));

    const manifest = readJson(`forge/conversion-manifests/${project.id}.json`);
    assert.equal(manifest.source_surface_map, surfaceMapPath);

    const receipt = readJson(project.receipt);
    assert.equal(receipt.source_surface_map, surfaceMapPath);
  }
});

test("conversion proof records visual route audits for real upstream surfaces", () => {
  const sourceManifest = readJson(".dx/forge/source-.dx/build-cache/manifest.json");
  assert.ok(Array.isArray(sourceManifest.visual_audits), "source manifest must list visual audit files");
  assert.equal(sourceManifest.visual_audits.length, 3);

  for (const project of projects) {
    const visualAudit = readJson(project.visualAudit);

    assert.equal(visualAudit.schema, "dx.www.route_visual_audit");
    assert.equal(visualAudit.source_project, project.id);
    assert.equal(visualAudit.converted_route, project.route);
    assert.equal(visualAudit.source_surface_map, `forge/source-surfaces/${project.id}.json`);
    assert.equal(visualAudit.no_fake_cards, true);
    assert.ok(visualAudit.score_out_of_100 >= 91);

    const sectionKinds = visualAudit.route_sections.map((section) => section.kind);
    for (const sectionKind of requiredVisualSectionKinds) {
      assert.ok(sectionKinds.includes(sectionKind), `${project.id} missing ${sectionKind} route section`);
    }

    for (const section of visualAudit.route_sections) {
      assert.ok(section.label.length > 0);
      assert.ok(section.source_files.length >= 1);
      assert.ok(section.converted_files.includes(project.page));
      assert.ok(section.launch_value.length > 0);
    }

    assert.ok(visualAudit.visual_assets.length >= 1);
    assert.ok(visualAudit.responsive_constraints.length >= 3);
    assert.ok(visualAudit.accessibility_notes.length >= 3);
    assert.ok(visualAudit.launch_state.real.length >= 2);
    assert.ok(visualAudit.launch_state.partial.length >= 1);
    assert.ok(visualAudit.launch_state.blocked.length >= 1);

    const page = readProof(project.page);
    assert.match(page, new RegExp(`data-visual-audit="${project.visualAudit}"`));

    const manifest = readJson(`forge/conversion-manifests/${project.id}.json`);
    assert.equal(manifest.visual_audit, project.visualAudit);

    const receipt = readJson(project.receipt);
    assert.equal(receipt.visual_audit, project.visualAudit);
  }
});

test("conversion proof exposes one canonical route discovery manifest for DX and Zed", () => {
  const routeDiscovery = readJson(routeDiscoveryPath);

  assert.equal(routeDiscovery.schema, "dx.www.conversion_route_discovery");
  assert.equal(routeDiscovery.status.score_out_of_100, 99);
  assert.equal(routeDiscovery.routing.canonical, "dx-www");
  assert.equal(routeDiscovery.routing.no_next_router, true);
  assert.equal(routeDiscovery.routing.no_tanstack_router, true);
  assert.equal(routeDiscovery.routing.no_react_router, true);
  assert.deepEqual(routeDiscovery.discovery_consumers, ["dx-www", "dx-cli", "zed"]);
  assert.equal(routeDiscovery.routes.length, 4);

  const landingRoute = routeDiscovery.routes.find((candidate) => candidate.route === "/");
  assert.ok(landingRoute, "DX landing route missing from route discovery");
  assert.equal(landingRoute.source_project, "dx-website-startpage");
  assert.equal(landingRoute.page, "pages/index.html");
  assert.equal(landingRoute.manifest, "forge/conversion-manifests/dx-landing.json");
  assert.equal(landingRoute.receipt, ".dx/forge/receipts/2026-05-21-dx-landing-to-index.json");
  assert.equal(landingRoute.source_surface_map, "forge/source-surfaces/dx-landing.json");
  assert.equal(landingRoute.visual_audit, "forge/visual-audits/dx-landing.json");
  assert.match(landingRoute.runtime_state, /real source recreation/);
  assert.ok(landingRoute.assets.length >= 1);
  assert.ok(landingRoute.dx_www_entrypoints.includes("pages/index.html"));

  const sourceManifest = readJson(".dx/forge/source-.dx/build-cache/manifest.json");
  assert.equal(sourceManifest.route_discovery, routeDiscoveryPath);
  assert.equal(sourceManifest.route_acceptance_checklist, routeAcceptancePath);
  assert.equal(sourceManifest.rendered_proof_receipt_schema, renderedProofSchemaPath);
  assert.equal(sourceManifest.rendered_proof_validator, renderedProofValidatorPath);
  assert.equal(sourceManifest.blocked_rendered_proof_fixture, blockedRenderedProofFixturePath);
  assert.equal(sourceManifest.rendered_proof_import_plan, renderedProofImportPlanPath);
  assert.equal(sourceManifest.rendered_proof_import_tool, renderedProofImportToolPath);
  assert.equal(sourceManifest.rendered_proof_review_tool, renderedProofReviewToolPath);
  assert.equal(sourceManifest.rendered_proof_runtime_approval_request, renderedProofApprovalRequestPath);
  assert.equal(sourceManifest.rendered_proof_runtime_approval_tool, renderedProofApprovalToolPath);
  assert.equal(sourceManifest.rendered_proof_authoring_guide, renderedProofAuthoringGuidePath);
  assert.equal(sourceManifest.rendered_proof_authoring_tool, renderedProofAuthoringToolPath);
  assert.equal(routeDiscovery.acceptance_checklist, routeAcceptancePath);
  assert.equal(routeDiscovery.rendered_proof_receipt_schema, renderedProofSchemaPath);
  assert.equal(routeDiscovery.rendered_proof_validator, renderedProofValidatorPath);
  assert.equal(routeDiscovery.blocked_rendered_proof_fixture, blockedRenderedProofFixturePath);
  assert.equal(routeDiscovery.rendered_proof_import_plan, renderedProofImportPlanPath);
  assert.equal(routeDiscovery.rendered_proof_import_tool, renderedProofImportToolPath);
  assert.equal(routeDiscovery.rendered_proof_review_tool, renderedProofReviewToolPath);
  assert.equal(routeDiscovery.rendered_proof_runtime_approval_request, renderedProofApprovalRequestPath);
  assert.equal(routeDiscovery.rendered_proof_runtime_approval_tool, renderedProofApprovalToolPath);
  assert.equal(routeDiscovery.rendered_proof_authoring_guide, renderedProofAuthoringGuidePath);
  assert.equal(routeDiscovery.rendered_proof_authoring_tool, renderedProofAuthoringToolPath);

  for (const project of projects) {
    const route = routeDiscovery.routes.find((candidate) => candidate.route === project.route);
    assert.ok(route, `${project.route} missing from route discovery`);
    assert.equal(route.source_project, project.id);
    assert.equal(route.page, project.page);
    assert.equal(route.manifest, `forge/conversion-manifests/${project.id}.json`);
    assert.equal(route.receipt, project.receipt);
    assert.equal(route.source_surface_map, `forge/source-surfaces/${project.id}.json`);
    assert.equal(route.visual_audit, project.visualAudit);
    assert.match(route.runtime_state, /real source conversion/);
    assert.ok(route.assets.length >= 1);
    assert.ok(route.provenance.license.length > 0);
    assert.ok(route.provenance.source_commit.length >= 12);
    assert.ok(route.dx_www_entrypoints.includes(project.page));
    assert.ok(route.operator_next_action.length > 0);
  }
});

test("conversion proof publishes a no-runtime route acceptance checklist", () => {
  const acceptance = readJson(routeAcceptancePath);

  assert.equal(acceptance.schema, "dx.www.no_runtime_route_acceptance");
  assert.equal(acceptance.status.score_out_of_100, 99);
  assert.equal(acceptance.rendered_proof_receipt_schema, renderedProofSchemaPath);
  assert.equal(acceptance.rendered_proof_validator, renderedProofValidatorPath);
  assert.equal(acceptance.blocked_rendered_proof_fixture, blockedRenderedProofFixturePath);
  assert.equal(acceptance.rendered_proof_import_plan, renderedProofImportPlanPath);
  assert.equal(acceptance.rendered_proof_import_tool, renderedProofImportToolPath);
  assert.equal(acceptance.rendered_proof_review_tool, renderedProofReviewToolPath);
  assert.equal(acceptance.rendered_proof_runtime_approval_request, renderedProofApprovalRequestPath);
  assert.equal(acceptance.rendered_proof_runtime_approval_tool, renderedProofApprovalToolPath);
  assert.equal(acceptance.rendered_proof_authoring_guide, renderedProofAuthoringGuidePath);
  assert.equal(acceptance.rendered_proof_authoring_tool, renderedProofAuthoringToolPath);
  assert.equal(acceptance.rendered_proof_receipt_output, ".dx/forge/runtime-evidence/conversion-proof/{route}.rendered-proof.json");
  assert.deepEqual(acceptance.consumers, ["studio", "zed", "dx-cli", "dx-www"]);
  assert.equal(acceptance.policy.no_local_server, true);
  assert.equal(acceptance.policy.no_build, true);
  assert.equal(acceptance.policy.no_node_modules, true);
  assert.equal(acceptance.policy.no_live_credentials, true);
  assert.match(acceptance.approval_gate, /explicit runtime approval/i);

  const requiredEvidenceIds = [
    "renderer-snapshot",
    "asset-rendering",
    "responsive-review",
    "accessibility-review",
    "interaction-boundary-review",
    "provenance-review",
  ];

  for (const project of projects) {
    const route = acceptance.routes.find((candidate) => candidate.route === project.route);

    assert.ok(route, `${project.route} missing from acceptance checklist`);
    assert.equal(route.source_project, project.id);
    assert.equal(route.page, project.page);
    assert.equal(route.current_state, "source-proof-ready");
    assert.equal(route.target_state, "rendered-proof-ready");
    assert.equal(route.studio_preview_selector, `[data-dx-route="${project.route}"]`);
    assert.equal(route.visual_audit, project.visualAudit);
    assert.equal(route.route_discovery, routeDiscoveryPath);
    assert.equal(route.rendered_proof_receipt_schema, renderedProofSchemaPath);
    assert.equal(route.rendered_proof_receipt_target, `.dx/forge/runtime-evidence/conversion-proof/${project.route.slice(1)}.rendered-proof.json`);
    assert.ok(route.open_files.includes(project.page));
    assert.ok(route.open_files.includes(route.visual_audit));
    assert.ok(route.blocked_until.length > 0);

    const evidenceIds = route.required_evidence.map((item) => item.id);
    for (const evidenceId of requiredEvidenceIds) {
      assert.ok(evidenceIds.includes(evidenceId), `${project.route} missing ${evidenceId}`);
    }

    for (const evidence of route.required_evidence) {
      assert.match(evidence.status, /pending|blocked/);
      assert.ok(evidence.owner.length > 0);
      assert.ok(evidence.acceptance_note.length > 0);
    }
  }
});

test("conversion proof defines the rendered-proof evidence receipt schema for runtime lanes", () => {
  const schema = readJson(renderedProofSchemaPath);

  assert.equal(schema.schema, "dx.www.rendered_proof_evidence_schema");
  assert.equal(schema.$schema, "https://json-schema.org/draft/2020-12/schema");
  assert.equal(schema.status.score_out_of_100, 99);
  assert.equal(schema.validator, renderedProofValidatorPath);
  assert.equal(schema.blocked_receipt_fixture, blockedRenderedProofFixturePath);
  assert.equal(schema.import_plan, renderedProofImportPlanPath);
  assert.equal(schema.import_tool, renderedProofImportToolPath);
  assert.equal(schema.review_tool, renderedProofReviewToolPath);
  assert.equal(schema.runtime_approval_request, renderedProofApprovalRequestPath);
  assert.equal(schema.runtime_approval_tool, renderedProofApprovalToolPath);
  assert.equal(schema.authoring_guide, renderedProofAuthoringGuidePath);
  assert.equal(schema.authoring_tool, renderedProofAuthoringToolPath);
  assert.equal(schema.runtime_lane, "runtime-verification-lane");
  assert.equal(schema.no_source_proof_mutation, true);
  assert.equal(schema.receipt_output_pattern, ".dx/forge/runtime-evidence/conversion-proof/{route}.rendered-proof.json");
  assert.deepEqual(schema.source_inputs, [
    routeDiscoveryPath,
    routeAcceptancePath,
    ".dx/forge/source-.dx/build-cache/manifest.json",
  ]);

  for (const requiredField of [
    "route",
    "source_project",
    "approval",
    "checks",
    "provenance_snapshot",
    "result",
    "no_source_proof_mutation",
  ]) {
    assert.ok(schema.required.includes(requiredField), `schema missing ${requiredField}`);
  }

  const properties = schema.properties;
  assert.deepEqual(properties.route.enum, projects.map((project) => project.route));
  assert.ok(properties.artifacts.required.includes("renderer_snapshot"));
  assert.ok(properties.artifacts.required.includes("asset_rendering"));
  assert.ok(properties.artifacts.required.includes("responsive_review"));
  assert.ok(properties.artifacts.required.includes("accessibility_review"));
  assert.equal(properties.blocked_reason.type, "string");
  assert.equal(properties.missing_artifacts.type, "array");
  assert.ok(properties.checks.required.includes("interaction_boundary_review"));
  assert.ok(properties.checks.required.includes("provenance_review"));
  assert.ok(properties.approval.required.includes("status"));
  assert.ok(properties.approval.required.includes("approval_reference"));
  assert.ok(properties.result.enum.includes("rendered-proof-ready"));
  assert.ok(properties.result.enum.includes("blocked"));

  const readyRule = schema.allOf.find((rule) => rule.if?.properties?.result?.const === "rendered-proof-ready");
  const blockedRule = schema.allOf.find((rule) => rule.if?.properties?.result?.const === "blocked");

  assert.ok(readyRule.then.required.includes("artifacts"));
  assert.ok(readyRule.then.properties.approval.required.includes("approved_by"));
  assert.ok(readyRule.then.properties.approval.required.includes("approved_at"));
  assert.ok(blockedRule.then.required.includes("blocked_reason"));
  assert.ok(blockedRule.then.required.includes("missing_artifacts"));
  assert.equal(blockedRule.then.properties.approval.properties.status.const, "not-approved");
  assert.ok(blockedRule.then.not.required.includes("artifacts"));
});

test("conversion proof defines an approval-gated rendered-proof import plan", () => {
  const importPlan = readJson(renderedProofImportPlanPath);

  assert.equal(importPlan.schema, "dx.www.rendered_proof_import_plan");
  assert.equal(importPlan.status.score_out_of_100, 99);
  assert.equal(importPlan.import_tool, renderedProofImportToolPath);
  assert.equal(importPlan.validator, renderedProofValidatorPath);
  assert.equal(importPlan.receipt_schema, renderedProofSchemaPath);
  assert.equal(importPlan.acceptance_checklist, routeAcceptancePath);
  assert.equal(importPlan.blocked_fixture, blockedRenderedProofFixturePath);
  assert.equal(importPlan.no_source_proof_mutation, true);
  assert.equal(importPlan.policy.dry_run_default, true);
  assert.equal(importPlan.policy.no_write_without_approval, true);
  assert.equal(importPlan.policy.no_fake_artifacts, true);
  assert.equal(importPlan.policy.no_local_server, true);
  assert.equal(importPlan.policy.no_build, true);
  assert.equal(importPlan.routes.length, 3);

  for (const project of projects) {
    const route = importPlan.routes.find((candidate) => candidate.route === project.route);

    assert.ok(route, `${project.route} missing from import plan`);
    assert.equal(route.source_project, project.id);
    assert.equal(route.target_receipt, `.dx/forge/runtime-evidence/conversion-proof/${project.route.slice(1)}.rendered-proof.json`);
    assert.equal(route.input_receipt, `external-runtime-evidence/conversion-proof/${project.route.slice(1)}.rendered-proof.json`);
    assert.equal(route.approval_required, true);
    assert.ok(route.required_source_files.includes(project.page));
    assert.ok(route.required_source_files.includes(project.visualAudit));
    assert.ok(route.required_source_files.includes(`forge/conversion-manifests/${project.id}.json`));
    assert.ok(route.required_source_files.includes(project.receipt));
    assert.ok(route.import_checks.includes("receipt-schema-validation"));
    assert.ok(route.import_checks.includes("artifact-hash-review"));
    assert.ok(route.import_checks.includes("provenance-snapshot-review"));
  }
});

test("conversion proof publishes a no-runtime rendered-proof approval request", () => {
  const approvalRequest = readJson(renderedProofApprovalRequestPath);

  assert.equal(approvalRequest.schema, "dx.www.rendered_proof_runtime_approval_request");
  assert.equal(approvalRequest.status.score_out_of_100, 99);
  assert.equal(approvalRequest.approval_state, "approval_required");
  assert.equal(approvalRequest.requested_scope, "runtime-rendered-proof-capture");
  assert.equal(approvalRequest.no_source_proof_mutation, true);
  assert.equal(approvalRequest.request_tool, renderedProofApprovalToolPath);
  assert.equal(approvalRequest.import_plan, renderedProofImportPlanPath);
  assert.equal(approvalRequest.import_tool, renderedProofImportToolPath);
  assert.equal(approvalRequest.review_tool, renderedProofReviewToolPath);
  assert.equal(approvalRequest.validator, renderedProofValidatorPath);
  assert.equal(approvalRequest.authoring_guide, renderedProofAuthoringGuidePath);
  assert.equal(approvalRequest.authoring_tool, renderedProofAuthoringToolPath);
  assert.equal(approvalRequest.policy.no_local_server_without_approval, true);
  assert.equal(approvalRequest.policy.no_build_without_approval, true);
  assert.equal(approvalRequest.policy.no_live_credentials_without_approval, true);
  assert.equal(approvalRequest.policy.no_write_before_approval, true);
  assert.equal(approvalRequest.routes.length, 3);

  for (const project of projects) {
    const route = approvalRequest.routes.find((candidate) => candidate.route === project.route);

    assert.ok(route, `${project.route} missing from approval request`);
    assert.equal(route.source_project, project.id);
    assert.equal(route.page, project.page);
    assert.equal(route.target_receipt, `.dx/forge/runtime-evidence/conversion-proof/${project.route.slice(1)}.rendered-proof.json`);
    assert.equal(route.approval_required, true);
    assert.ok(route.required_runtime_actions.includes("renderer-snapshot"));
    assert.ok(route.required_runtime_actions.includes("responsive-review"));
    assert.ok(route.required_runtime_actions.includes("accessibility-review"));
    assert.ok(route.required_runtime_actions.includes("artifact-hash-review"));
    assert.match(route.approval_question, /approve/i);
    assert.match(route.approval_question, new RegExp(project.route.replace("/", "\\/")));
  }

  assert.ok(approvalRequest.post_approval_commands.includes(`node .\\${renderedProofImportToolPath} --json --approval <approval-reference>`));
  assert.ok(approvalRequest.post_approval_commands.includes(`node .\\${renderedProofReviewToolPath} --json`));
  assert.ok(approvalRequest.post_approval_commands.includes(`node .\\${renderedProofValidatorPath} --json`));
});

test("conversion proof publishes a rendered-proof evidence authoring guide without fake artifacts", () => {
  const guide = readJson(renderedProofAuthoringGuidePath);

  assert.equal(guide.schema, "dx.www.rendered_proof_evidence_authoring_guide");
  assert.equal(guide.status.score_out_of_100, 99);
  assert.equal(guide.guidance_state, "source-only");
  assert.equal(guide.no_source_proof_mutation, true);
  assert.equal(guide.writes_runtime_evidence, false);
  assert.equal(guide.receipt_schema, renderedProofSchemaPath);
  assert.equal(guide.approval_request, renderedProofApprovalRequestPath);
  assert.equal(guide.approval_tool, renderedProofApprovalToolPath);
  assert.equal(guide.import_plan, renderedProofImportPlanPath);
  assert.equal(guide.import_tool, renderedProofImportToolPath);
  assert.equal(guide.review_tool, renderedProofReviewToolPath);
  assert.equal(guide.validator, renderedProofValidatorPath);
  assert.ok(guide.forbidden_fake_artifacts.includes("synthetic_screenshot"));
  assert.ok(guide.forbidden_fake_artifacts.includes("placeholder_hash"));
  assert.ok(guide.forbidden_fake_artifacts.includes("generated_success_status"));
  assert.equal(guide.routes.length, 3);

  for (const project of projects) {
    const route = guide.routes.find((candidate) => candidate.route === project.route);

    assert.ok(route, `${project.route} missing from authoring guide`);
    assert.equal(route.source_project, project.id);
    assert.equal(route.target_receipt, `.dx/forge/runtime-evidence/conversion-proof/${project.route.slice(1)}.rendered-proof.json`);
    assert.equal(route.output_intent, "authoring-guide-only");
    assert.equal(route.approval_required, true);
    assert.ok(route.source_files.includes(project.page));
    assert.ok(route.source_files.includes(project.visualAudit));
    assert.ok(route.required_artifacts.includes("renderer_snapshot"));
    assert.ok(route.required_artifacts.includes("asset_rendering"));
    assert.ok(route.required_artifacts.includes("responsive_review"));
    assert.ok(route.required_artifacts.includes("accessibility_review"));
    assert.ok(route.required_checks.includes("interaction_boundary_review"));
    assert.ok(route.required_checks.includes("provenance_review"));
    assert.ok(route.disallowed_artifacts.includes("fake-renderer-snapshot"));
    assert.ok(route.disallowed_artifacts.includes("placeholder-sha256"));
    assert.ok(route.capture_fields.every((field) => field.value === null));
  }
});

test("rendered-proof evidence authoring reporter is source-only and non-writing", () => {
  const authoringToolSource = readProof(renderedProofAuthoringToolPath);
  assert.doesNotMatch(
    authoringToolSource,
    /require\(["']ajv["']\)|from\s+["']ajv["']|node_modules|playwright|puppeteer|next\/|@supabase|convex|fs\.writeFileSync|fs\.copyFileSync/,
    "authoring reporter must stay source-only, dependency-free, and non-writing",
  );

  const output = execFileSync(process.execPath, [path.join(proofRoot, renderedProofAuthoringToolPath), "--json"], {
    cwd: proofRoot,
    encoding: "utf8",
  });
  const report = JSON.parse(output);

  assert.equal(report.schema, "dx.www.rendered_proof_evidence_authoring_report");
  assert.equal(report.status, "approval_required");
  assert.equal(report.source_readiness, "pass");
  assert.equal(report.write_mode, false);
  assert.equal(report.wrote_files, false);
  assert.equal(report.no_source_proof_mutation, true);
  assert.equal(report.authoring_guide, renderedProofAuthoringGuidePath);
  assert.equal(report.approval_request, renderedProofApprovalRequestPath);
  assert.equal(report.receipt_schema, renderedProofSchemaPath);
  assert.equal(report.summary.routes, 3);
  assert.equal(report.summary.required_artifacts_per_route, 4);
  assert.equal(report.summary.runtime_receipts_present, 0);
  assert.equal(report.routes.length, 3);

  for (const project of projects) {
    const route = report.routes.find((candidate) => candidate.route === project.route);

    assert.ok(route, `${project.route} missing from authoring report`);
    assert.equal(route.source_project, project.id);
    assert.equal(route.status, "waiting_for_runtime_approval");
    assert.equal(route.target_receipt, `.dx/forge/runtime-evidence/conversion-proof/${project.route.slice(1)}.rendered-proof.json`);
    assert.equal(route.runtime_receipt_present, false);
    assert.equal(route.fake_artifacts_allowed, false);
    assert.equal(route.capture_fields_pending, 4);
  }
});

test("rendered-proof approval request reporter is source-only and non-writing", () => {
  const approvalToolSource = readProof(renderedProofApprovalToolPath);
  assert.doesNotMatch(
    approvalToolSource,
    /require\(["']ajv["']\)|from\s+["']ajv["']|node_modules|playwright|puppeteer|next\/|@supabase|convex|fs\.writeFileSync|fs\.copyFileSync/,
    "approval request reporter must stay source-only, dependency-free, and non-writing",
  );

  const output = execFileSync(process.execPath, [path.join(proofRoot, renderedProofApprovalToolPath), "--json"], {
    cwd: proofRoot,
    encoding: "utf8",
  });
  const report = JSON.parse(output);

  assert.equal(report.schema, "dx.www.rendered_proof_runtime_approval_report");
  assert.equal(report.status, "approval_required");
  assert.equal(report.source_readiness, "pass");
  assert.equal(report.approval_granted, false);
  assert.equal(report.write_mode, false);
  assert.equal(report.wrote_files, false);
  assert.equal(report.no_source_proof_mutation, true);
  assert.equal(report.request, renderedProofApprovalRequestPath);
  assert.equal(report.import_plan, renderedProofImportPlanPath);
  assert.equal(report.import_tool, renderedProofImportToolPath);
  assert.equal(report.review_tool, renderedProofReviewToolPath);
  assert.equal(report.validator, renderedProofValidatorPath);
  assert.equal(report.summary.routes, 3);
  assert.equal(report.summary.approval_required, 3);
  assert.equal(report.summary.runtime_receipts_present, 0);
  assert.equal(report.routes.length, 3);

  for (const project of projects) {
    const route = report.routes.find((candidate) => candidate.route === project.route);

    assert.ok(route, `${project.route} missing from approval report`);
    assert.equal(route.source_project, project.id);
    assert.equal(route.status, "waiting_for_runtime_approval");
    assert.equal(route.approval_required, true);
    assert.equal(route.write_allowed, false);
    assert.equal(route.target_receipt, `.dx/forge/runtime-evidence/conversion-proof/${project.route.slice(1)}.rendered-proof.json`);
  }
});

test("rendered-proof completeness reviewer keeps the proof at 99 until real receipts exist", () => {
  const reviewToolSource = readProof(renderedProofReviewToolPath);
  assert.doesNotMatch(
    reviewToolSource,
    /require\(["']ajv["']\)|from\s+["']ajv["']|node_modules|playwright|puppeteer|next\/|@supabase|convex|fs\.writeFileSync|fs\.copyFileSync/,
    "completeness reviewer must stay source-only, dependency-free, and non-writing",
  );

  const output = execFileSync(process.execPath, [path.join(proofRoot, renderedProofReviewToolPath), "--json"], {
    cwd: proofRoot,
    encoding: "utf8",
  });
  const report = JSON.parse(output);

  assert.equal(report.schema, "dx.www.rendered_proof_review_report");
  assert.equal(report.status, "runtime_evidence_incomplete");
  assert.equal(report.score_out_of_100, 99);
  assert.equal(report.source_readiness, "pass");
  assert.equal(report.complete, false);
  assert.equal(report.ready_for_100, false);
  assert.equal(report.no_source_proof_mutation, true);
  assert.equal(report.wrote_files, false);
  assert.equal(report.receipt_schema, renderedProofSchemaPath);
  assert.equal(report.validator, renderedProofValidatorPath);
  assert.equal(report.import_plan, renderedProofImportPlanPath);
  assert.equal(report.import_tool, renderedProofImportToolPath);
  assert.equal(report.summary.routes, 3);
  assert.equal(report.summary.ready_receipts, 0);
  assert.equal(report.summary.missing_runtime_receipts, 3);
  assert.equal(report.summary.ready_to_import, 0);
  assert.equal(report.summary.approval_required, 3);
  assert.ok(report.blockers.includes("real-rendered-proof-receipts"));
  assert.ok(report.blockers.includes("runtime-approval-reference"));
  assert.ok(report.blockers.includes("renderer-snapshots"));
  assert.ok(report.blockers.includes("responsive-accessibility-artifacts"));

  for (const project of projects) {
    const route = report.routes.find((candidate) => candidate.route === project.route);

    assert.ok(route, `${project.route} missing from completeness review`);
    assert.equal(route.source_project, project.id);
    assert.equal(route.completion_state, "blocked-runtime-evidence");
    assert.equal(route.validator_status, "missing_runtime_evidence");
    assert.equal(route.import_status, "approval_required");
    assert.equal(route.target_receipt, `.dx/forge/runtime-evidence/conversion-proof/${project.route.slice(1)}.rendered-proof.json`);
    assert.match(route.next_action, /approved runtime/i);
    assert.match(route.next_action, /rendered-proof receipt/i);
  }
});

test("rendered-proof validator reports missing runtime evidence without failing source readiness", () => {
  const validatorSource = readProof(renderedProofValidatorPath);
  assert.doesNotMatch(
    validatorSource,
    /require\(["']ajv["']\)|from\s+["']ajv["']|node_modules|playwright|puppeteer|next\/|@supabase|convex/,
    "validator must stay source-only and dependency-free",
  );

  const output = execFileSync(process.execPath, [path.join(proofRoot, renderedProofValidatorPath), "--json"], {
    cwd: proofRoot,
    encoding: "utf8",
  });
  const report = JSON.parse(output);

  assert.equal(report.schema, "dx.www.rendered_proof_validator");
  assert.equal(report.source_readiness, "pass");
  assert.equal(report.status, "missing_runtime_evidence");
  assert.equal(report.no_source_proof_mutation, true);
  assert.equal(report.receipt_schema, renderedProofSchemaPath);
  assert.equal(report.acceptance_checklist, routeAcceptancePath);
  assert.equal(report.blocked_fixture, blockedRenderedProofFixturePath);
  assert.equal(report.import_plan, renderedProofImportPlanPath);
  assert.equal(report.import_tool, renderedProofImportToolPath);
  assert.equal(report.receipt_output_pattern, ".dx/forge/runtime-evidence/conversion-proof/{route}.rendered-proof.json");
  assert.equal(report.summary.expected_receipts, 3);
  assert.equal(report.summary.missing_receipts, 3);
  assert.equal(report.summary.ready_receipts, 0);
  assert.equal(report.routes.length, 3);

  for (const project of projects) {
    const route = report.routes.find((candidate) => candidate.route === project.route);

    assert.ok(route, `${project.route} missing from validator report`);
    assert.equal(route.source_project, project.id);
    assert.equal(route.status, "missing_runtime_evidence");
    assert.equal(route.expected_receipt, `.dx/forge/runtime-evidence/conversion-proof/${project.route.slice(1)}.rendered-proof.json`);
    assert.equal(route.source_ready, true);
    assert.ok(route.required_evidence.includes("renderer-snapshot"));
    assert.ok(route.required_evidence.includes("provenance-review"));
  }
});

test("rendered-proof import reporter is source-only and approval-gated", () => {
  const importToolSource = readProof(renderedProofImportToolPath);
  assert.doesNotMatch(
    importToolSource,
    /require\(["']ajv["']\)|from\s+["']ajv["']|node_modules|playwright|puppeteer|next\/|@supabase|convex|fs\.writeFileSync|fs\.copyFileSync/,
    "import reporter must stay source-only, dependency-free, and non-writing",
  );

  const output = execFileSync(process.execPath, [path.join(proofRoot, renderedProofImportToolPath), "--json"], {
    cwd: proofRoot,
    encoding: "utf8",
  });
  const report = JSON.parse(output);

  assert.equal(report.schema, "dx.www.rendered_proof_import_report");
  assert.equal(report.status, "approval_required");
  assert.equal(report.source_readiness, "pass");
  assert.equal(report.approval_required, true);
  assert.equal(report.approval_reference, null);
  assert.equal(report.write_mode, false);
  assert.equal(report.wrote_files, false);
  assert.equal(report.no_source_proof_mutation, true);
  assert.equal(report.import_plan, renderedProofImportPlanPath);
  assert.equal(report.receipt_schema, renderedProofSchemaPath);
  assert.equal(report.validator, renderedProofValidatorPath);
  assert.equal(report.summary.planned_routes, 3);
  assert.equal(report.summary.missing_input_receipts, 3);
  assert.equal(report.summary.ready_to_import, 0);
  assert.equal(report.routes.length, 3);

  for (const project of projects) {
    const route = report.routes.find((candidate) => candidate.route === project.route);

    assert.ok(route, `${project.route} missing from import report`);
    assert.equal(route.source_project, project.id);
    assert.equal(route.status, "approval_required");
    assert.equal(route.target_receipt, `.dx/forge/runtime-evidence/conversion-proof/${project.route.slice(1)}.rendered-proof.json`);
    assert.equal(route.input_receipt, `external-runtime-evidence/conversion-proof/${project.route.slice(1)}.rendered-proof.json`);
    assert.equal(route.approved, false);
    assert.equal(route.write_allowed, false);
  }
});

test("conversion proof includes an honest blocked rendered-proof sample receipt", () => {
  const fixture = readJson(blockedRenderedProofFixturePath);

  assert.equal(fixture.schema, "dx.www.rendered_proof_evidence");
  assert.equal(fixture.fixture_kind, "blocked-rendered-proof-sample");
  assert.equal(fixture.route, "/ui");
  assert.equal(fixture.source_project, "shadcn-ui");
  assert.equal(fixture.result, "blocked");
  assert.equal(fixture.no_source_proof_mutation, true);
  assert.equal(fixture.approval.status, "not-approved");
  assert.match(fixture.blocked_reason, /runtime approval/i);
  assert.match(fixture.blocked_reason, /renderer snapshot/i);
  assert.deepEqual(fixture.missing_artifacts, [
    "renderer_snapshot",
    "asset_rendering",
    "responsive_review",
    "accessibility_review",
  ]);
  assert.equal(fixture.artifacts, undefined, "blocked fixture must not invent artifact paths or hashes");
  assert.equal(fixture.checks.interaction_boundary_review.status, "blocked");
  assert.equal(fixture.checks.provenance_review.status, "pass");
  assert.equal(fixture.provenance_snapshot.route_discovery, routeDiscoveryPath);
  assert.equal(fixture.provenance_snapshot.acceptance_checklist, routeAcceptancePath);
  assert.equal(fixture.provenance_snapshot.source_manifest, ".dx/forge/source-.dx/build-cache/manifest.json");
  assert.equal(fixture.provenance_snapshot.visual_audit, "forge/visual-audits/shadcn-ui.json");
  assert.equal(fixture.provenance_snapshot.conversion_manifest, "forge/conversion-manifests/shadcn-ui.json");
  assert.equal(fixture.provenance_snapshot.receipt, ".dx/forge/receipts/2026-05-21-shadcn-ui-to-ui.json");
});

test("conversion routes expose Studio preview selectors for source selection", () => {
  const routeDiscovery = readJson(routeDiscoveryPath);

  assert.equal(routeDiscovery.studio_preview.schema, "dx.studio.preview_route_markers");
  assert.deepEqual(routeDiscovery.studio_preview.marker_attributes, [
    "data-dx-route",
    "data-dx-source",
    "data-dx-forge",
    "data-dx-package",
    "data-dx-hot-reload-target",
    "data-surface-map",
    "data-visual-audit",
  ]);

  for (const project of projects) {
    const page = readProof(project.page);
    const route = routeDiscovery.routes.find((candidate) => candidate.route === project.route);

    assert.match(page, new RegExp(`data-dx-route="${project.route}"`));
    assert.match(page, new RegExp(`data-dx-source="${project.id}"`));
    assert.match(page, new RegExp(`data-dx-forge="${project.forge}"`));
    assert.match(page, new RegExp(`data-dx-package="${project.packageId}"`));
    assert.match(page, new RegExp(`data-dx-hot-reload-target="route:${project.route}"`));

    assert.equal(route.studio_preview.selector, `[data-dx-route="${project.route}"]`);
    assert.equal(route.studio_preview.forge_selector, `[data-dx-forge="${project.forge}"]`);
    assert.equal(route.studio_preview.package_selector, `[data-dx-package="${project.packageId}"]`);
    assert.equal(route.studio_preview.hot_reload_target, `route:${project.route}`);
    assert.equal(route.studio_preview.open_files[0], project.page);
    assert.ok(route.studio_preview.open_files.includes(route.source_surface_map));
    assert.ok(route.studio_preview.open_files.includes(route.visual_audit));
  }
});

test("source-owned Forge primitives cover the launch-useful UI/theme layer without node_modules", () => {
  for (const primitivePath of primitiveFiles) {
    const source = readProof(primitivePath);

    assert.doesNotMatch(
      source,
      /from\s+["']react["']|require\(["']react["']\)|from\s+["']@radix-ui|from\s+["']next-themes|from\s+["']clsx|from\s+["']tailwind-merge/,
      `${primitivePath} must stay source-owned and node_modules-free`,
    );
  }

  const indexSource = readProof("forge/primitives/index.ts");
  for (const exportName of [
    "cn",
    "createSlot",
    "createThemeProvider",
    "buttonPrimitive",
    "inputPrimitive",
    "badgePrimitive",
    "cardPrimitive",
    "tablePrimitive",
    "tabsPrimitive",
    "dialogPrimitive",
    "dropdownPrimitive",
    "sidebarPrimitive",
  ]) {
    assert.match(indexSource, new RegExp(exportName));
  }

  const metadata = readJson("forge/primitives/metadata.json");
  assert.equal(metadata.schema, "dx.forge.primitive_package");
  assert.equal(metadata.dependency_kind, "source-owned-forge-primitives");
  assert.deepEqual(metadata.routes, ["/ui", "/database", "/backend"]);
  assert.ok(metadata.primitives.length >= 11);
});

test("heavy dependencies are launch shims with explicit missing-runtime boundaries", () => {
  for (const shimPath of shimFiles) {
    const source = readProof(shimPath);
    assert.doesNotMatch(
      source,
      /from\s+["']@supabase|from\s+["']convex|from\s+["']next|from\s+["']@tanstack|from\s+["']react-router/,
      `${shimPath} must not import heavy runtime packages`,
    );
  }

  const boundaries = readJson("forge/shims/launch-runtime-boundaries.json");
  assert.equal(boundaries.schema, "dx.forge.launch_runtime_boundaries");
  assert.ok(boundaries.boundaries.length >= 6);

  for (const boundary of boundaries.boundaries) {
    assert.notEqual(boundary.status, "implemented");
    assert.match(boundary.status, /shim|blocked|missing-runtime/);
    assert.ok(boundary.future_work.length > 0);
  }

  for (const project of projects) {
    const page = readProof(project.page);
    assert.match(page, /data-dx-package-set=/);
    assert.match(page, /data-dx-hot-reload-target=/);
  }
});

test("conversion manifests preserve source provenance, license, assets, and unsupported runtime notes", () => {
  for (const project of projects) {
    const manifest = readJson(`forge/conversion-manifests/${project.id}.json`);

    assert.equal(manifest.schema, "dx.www.conversion_manifest");
    assert.equal(manifest.project, project.id);
    assert.equal(manifest.converted_dx_www_route_target, project.route);
    assert.ok(manifest.source_repo.url.startsWith("https://github.com/"));
    assert.ok(manifest.source_repo.local_path.startsWith("G:\\WWW\\inspirations\\"));
    assert.ok(manifest.source_repo.commit.length >= 12);
    assert.ok(manifest.source_repo.license);
    assert.ok(manifest.source_repo.license_files.length >= 1);
    assert.ok(manifest.source_route_component_files.length >= 5);
    assert.ok(manifest.converted_files.includes(project.page));
    assert.ok(manifest.assets_copied.length >= 1);
    assert.ok(manifest.forge_primitives.length >= 3);
    assert.ok(manifest.launch_shims.length >= 1);
    assert.ok(manifest.unsupported_runtime_features.length >= 3);
  }
});

test("Forge receipts and copied assets make the conversion auditable", () => {
  const sourceManifest = readJson(".dx/forge/source-.dx/build-cache/manifest.json");
  assert.equal(sourceManifest.schema, "dx.forge.source_manifest");
  assert.equal(sourceManifest.routes.length, 3);
  assert.equal(sourceManifest.status.score_out_of_100, 99);

  for (const project of projects) {
    const receipt = readJson(project.receipt);

    assert.equal(receipt.schema, "dx.forge.conversion_receipt");
    assert.equal(receipt.converted_route, project.route);
    assert.equal(receipt.source_project, project.id);
    assert.equal(receipt.license_notice_preserved, true);
    assert.ok(receipt.converted_source_files.length >= 5);
    assert.ok(receipt.copied_assets.length >= 1);
    assert.ok(receipt.forge_primitives.length >= 3);
    assert.ok(receipt.launch_shims.length >= 1);
    assert.ok(receipt.unsupported_runtime_features.length >= 3);
    assert.equal(receipt.no_runtime_execution, true);
  }

  for (const assetPath of [
    "public/vendor/shadcn-favicon-32x32.png",
    "public/vendor/supabase-logo.svg",
    "public/vendor/convex-logo.svg",
  ]) {
    assert.ok(fs.existsSync(path.join(proofRoot, assetPath)), `${assetPath} is copied`);
  }
});

test("status docs report real, partial, and blocked route state out of 100", () => {
  const docs = [
    readProof("README.md"),
    readProof("DX.md"),
    readProof("TODO.md"),
    readProof("CHANGELOG.md"),
  ].join("\n");

  assert.match(docs, /99 \/ 100/);
  assert.match(docs.toLowerCase(), /visual audit/);
  assert.match(docs.toLowerCase(), /route discovery/);
  assert.match(docs.toLowerCase(), /studio preview/);
  assert.match(docs.toLowerCase(), /acceptance checklist/);
  assert.match(docs.toLowerCase(), /rendered-proof evidence/);
  assert.match(docs.toLowerCase(), /validator/);
  assert.match(docs.toLowerCase(), /blocked rendered-proof sample/);
  assert.match(docs.toLowerCase(), /rendered-proof import/);
  assert.match(docs.toLowerCase(), /rendered-proof completeness/);
  assert.match(docs.toLowerCase(), /runtime approval request/);
  assert.match(docs.toLowerCase(), /evidence authoring/);
  assert.match(docs.toLowerCase(), /real/);
  assert.match(docs.toLowerCase(), /partial/);
  assert.match(docs.toLowerCase(), /blocked/);
  assert.match(docs, /\/ui/);
  assert.match(docs, /\/database/);
  assert.match(docs, /\/backend/);
});

test("root DX-WWW status docs report the conversion proof out of 100", () => {
  const docs = [
    fs.readFileSync(path.join(repoRoot, "TODO.md"), "utf8"),
    fs.readFileSync(path.join(repoRoot, "CHANGELOG.md"), "utf8"),
    fs.readFileSync(path.join(repoRoot, "DX.md"), "utf8"),
  ].join("\n");

  assert.match(docs, /DX-WWW website conversion proof/);
  assert.match(docs, /99\/100|99 \/ 100/);
  assert.match(docs.toLowerCase(), /studio preview/);
  assert.match(docs.toLowerCase(), /acceptance checklist/);
  assert.match(docs.toLowerCase(), /rendered-proof evidence/);
  assert.match(docs.toLowerCase(), /validator/);
  assert.match(docs.toLowerCase(), /blocked rendered-proof sample/);
  assert.match(docs.toLowerCase(), /rendered-proof import/);
  assert.match(docs.toLowerCase(), /rendered-proof completeness/);
  assert.match(docs.toLowerCase(), /runtime approval request/);
  assert.match(docs.toLowerCase(), /evidence authoring/);
  assert.match(docs, /\/ui/);
  assert.match(docs, /\/database/);
  assert.match(docs, /\/backend/);
});
