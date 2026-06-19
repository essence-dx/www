const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const runbookFixturePath = "docs/packages/authentication.source-guard-runbook.json";
const previewManifestMaterializerPath = "tools/launch/materialize-www-template.ts";
const studioManifestSourcePath = "dx-www/src/cli/studio_manifest.rs";
const appRouteHandlerPath = "examples/template/app/api/auth/[...all]/route.ts";
const readinessRouteHandlerPath = "examples/template/app/api/auth/readiness/route.ts";
const appServerBoundaryPath = "examples/template/server/auth/better-auth.ts";
const templateReadinessReceiptPath =
  "examples/template/.dx/forge/template-readiness/authentication.json";
const authenticationReceiptFiles = [
  "examples/template/template-shell.tsx",
  "examples/template/auth-session-status.tsx",
  "examples/dashboard/src/lib/forge/auth/better-auth/dashboard.ts",
  "examples/dashboard/src/components/BetterAuthAccountWorkflow.tsx",
  runbookFixturePath,
  previewManifestMaterializerPath,
  studioManifestSourcePath,
  appRouteHandlerPath,
  readinessRouteHandlerPath,
  appServerBoundaryPath,
  templateReadinessReceiptPath,
];
const statusVocabulary = [
  "present",
  "stale",
  "missing-receipt",
  "blocked",
  "unsupported-surface",
];

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

test("Authentication receipt visibility is consumed by the shared package-status read model", () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const receipt = readJson(
    "examples/template/.dx/forge/receipts/auth-better-auth.json",
  );
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const statusSource = read("examples/template/forge-package-status.ts");
  const packageLock = read("examples/template/forge-package-lock.ts");
  const launchShell = read("examples/template/template-shell.tsx");
  const sessionStatus = read("examples/template/auth-session-status.tsx");

  const authVisibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "auth/better-auth",
  );

  assert.ok(authVisibility, "Authentication visibility row is missing");
  assert.equal(authVisibility.official_package_name, "Authentication");
  assert.equal(authVisibility.upstream_package, "better-auth");
  assert.equal(authVisibility.upstream_version, "1.6.11");
  assert.equal(authVisibility.status, "present");
  assert.equal(authVisibility.receipt_status, "present");
  assert.equal(
    authVisibility.package_receipt_path,
    ".dx/forge/receipts/auth-better-auth.json",
  );
  assert.deepEqual(authVisibility.status_vocabulary, statusVocabulary);
  assert.deepEqual(
    receipt.dx_check_visibility.status_legend.map((entry) => entry.status),
    statusVocabulary,
  );
  assert.equal(receipt.hash_algorithm, "sha256");
  assert.ok(
    receipt.file_hashes["examples/template/template-shell.tsx"],
    "Authentication receipt is missing the launch shell hash",
  );
  assert.ok(
    receipt.file_hashes["examples/template/auth-session-status.tsx"],
    "Authentication receipt is missing the session status hash",
  );
  assert.ok(
    receipt.file_hashes[runbookFixturePath],
    "Authentication receipt is missing the source-guard runbook fixture hash",
  );
  assert.ok(
    receipt.file_hashes[studioManifestSourcePath],
    "Authentication receipt is missing the Studio manifest source hash",
  );
  assert.ok(
    receipt.file_hashes[appRouteHandlerPath],
    "Authentication receipt is missing the App Router auth route hash",
  );
  assert.ok(
    receipt.file_hashes[readinessRouteHandlerPath],
    "Authentication receipt is missing the Authentication readiness route hash",
  );
  assert.ok(
    receipt.file_hashes[appServerBoundaryPath],
    "Authentication receipt is missing the app-owned server boundary hash",
  );
  assert.ok(
    receipt.file_hashes[templateReadinessReceiptPath],
    "Authentication receipt is missing the template-readiness receipt hash",
  );
  assert.ok(
    receipt.source_files.includes(studioManifestSourcePath),
    "Authentication receipt must list the Studio manifest source",
  );
  assert.ok(
    /^[a-f0-9]{64}$/.test(
      receipt.file_hashes["examples/template/template-shell.tsx"],
    ),
    "Authentication receipt launch shell hash must be a SHA-256 hex digest",
  );

  assert.ok(
    authVisibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "authentication-account-workflow" &&
        surface.receipt_path === ".dx/forge/receipts/auth-better-auth.json" &&
        surface.files.includes("components/template-app/template-shell.tsx") &&
        surface.hash_algorithm === "sha256" &&
        surface.file_hashes["examples/template/template-shell.tsx"] ===
          receipt.file_hashes["examples/template/template-shell.tsx"] &&
        surface.source_markers.includes(
          'data-dx-component="better-auth-account-dashboard-workflow"',
        ),
    ),
    "Authentication account workflow surface is missing",
  );
  assert.match(
    launchShell,
    /data-dx-style-surface="authentication-account-workflow"/,
  );
  assert.ok(
    authVisibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "authentication-session-status" &&
        surface.files.includes("components/template-app/auth-session-status.tsx") &&
        surface.hash_algorithm === "sha256" &&
        surface.file_hashes["examples/template/auth-session-status.tsx"] ===
          receipt.file_hashes["examples/template/auth-session-status.tsx"] &&
        surface.source_markers.includes(
          'data-dx-component="better-auth-session-status-panel"',
        ),
    ),
    "Authentication session status surface is missing",
  );
  assert.ok(
    authVisibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "authentication-source-guard-runbook" &&
        surface.files.includes(runbookFixturePath) &&
        surface.hash_algorithm === "sha256" &&
        surface.file_hashes[runbookFixturePath] ===
          receipt.file_hashes[runbookFixturePath] &&
        surface.source_markers.includes(runbookFixturePath),
    ),
    "Authentication source-guard runbook surface is missing",
  );
  assert.ok(
    authVisibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "authentication-studio-manifest-source" &&
        surface.files.includes(studioManifestSourcePath) &&
        surface.hash_algorithm === "sha256" &&
        surface.file_hashes[studioManifestSourcePath] ===
          receipt.file_hashes[studioManifestSourcePath] &&
        surface.source_markers.includes("authentication-package-lane-panel"),
    ),
    "Authentication Studio manifest source surface is missing",
  );
  assert.ok(
    authVisibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "authentication-app-route-handler" &&
        surface.files.includes(appRouteHandlerPath) &&
        surface.hash_algorithm === "sha256" &&
        surface.file_hashes[appRouteHandlerPath] ===
          receipt.file_hashes[appRouteHandlerPath] &&
        surface.source_markers.includes("app/api/auth/[...all]/route.ts") &&
        surface.source_markers.includes('export const runtime = "nodejs"'),
    ),
    "Authentication App Router route handler surface is missing",
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "authentication-app-route-handler" &&
        surface.source_file === appRouteHandlerPath &&
        surface.materialized_file === "app/api/auth/[...all]/route.ts",
    ),
    "Authentication receipt is missing the App Router route monitored surface",
  );
  assert.ok(
    authVisibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "authentication-readiness-route-handler" &&
        surface.files.includes(readinessRouteHandlerPath) &&
        surface.hash_algorithm === "sha256" &&
        surface.file_hashes[readinessRouteHandlerPath] ===
          receipt.file_hashes[readinessRouteHandlerPath] &&
        surface.source_markers.includes("app/api/auth/readiness/route.ts") &&
        surface.source_markers.includes("createTemplateBetterAuthReadiness"),
    ),
    "Authentication readiness route handler surface is missing",
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "authentication-readiness-route-handler" &&
        surface.source_file === readinessRouteHandlerPath &&
        surface.materialized_file === "app/api/auth/readiness/route.ts",
    ),
    "Authentication receipt is missing the readiness route monitored surface",
  );
  assert.ok(
    authVisibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "authentication-app-server-boundary" &&
        surface.files.includes(appServerBoundaryPath) &&
        surface.hash_algorithm === "sha256" &&
        surface.file_hashes[appServerBoundaryPath] ===
          receipt.file_hashes[appServerBoundaryPath] &&
        surface.source_markers.includes("dxTemplateBetterAuthDatabaseBoundary") &&
        surface.source_markers.includes("createTemplateBetterAuthRouteHandlers"),
    ),
    "Authentication app-owned server boundary surface is missing",
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "authentication-app-server-boundary" &&
        surface.source_file === appServerBoundaryPath &&
        surface.materialized_file === "server/auth/better-auth.ts",
    ),
    "Authentication receipt is missing the app-owned server boundary monitored surface",
  );
  assert.ok(
    authVisibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "authentication-template-readiness-receipt" &&
        surface.files.includes(templateReadinessReceiptPath) &&
        surface.hash_algorithm === "sha256" &&
        surface.file_hashes[templateReadinessReceiptPath] ===
          receipt.file_hashes[templateReadinessReceiptPath] &&
        surface.source_markers.includes("dx.forge.template_readiness.package"),
    ),
    "Authentication template-readiness receipt surface is missing",
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "authentication-template-readiness-receipt" &&
        surface.source_file === templateReadinessReceiptPath &&
        surface.materialized_file === ".dx/forge/template-readiness/authentication.json",
    ),
    "Authentication receipt is missing the template-readiness monitored surface",
  );
  assert.match(
    sessionStatus,
    /data-dx-style-surface="authentication-session-status"/,
  );

  assert.equal(
    receipt.dx_style_compatibility.schema,
    "dx.forge.package.dx_style_compatibility",
  );
  assert.equal(receipt.dx_style_compatibility.status, "present");
  assert.equal(receipt.dx_style_compatibility.token_source, "styles/globals.css");
  assert.equal(receipt.dx_style_compatibility.generated_css, "styles/globals.css");
  assert.equal(receipt.dx_style_compatibility.runtime_proof, false);
  assert.ok(
    receipt.dx_style_compatibility.visible_surfaces.includes(
      "authentication-account-workflow",
    ),
  );
  assert.ok(
    receipt.dx_style_compatibility.visible_surfaces.includes(
      "authentication-session-status",
    ),
  );
  assert.ok(
    receipt.dx_style_compatibility.data_dx_markers.includes(
      'data-dx-style-surface="authentication-account-workflow"',
    ),
  );
  assert.ok(
    receipt.dx_style_compatibility.data_dx_markers.includes(
      'data-dx-style-surface="authentication-session-status"',
    ),
  );
  assert.equal(
    authVisibility.dx_style_compatibility.schema,
    "dx.forge.package.dx_style_compatibility",
  );
  assert.equal(authVisibility.dx_style_compatibility.status, "present");
  assert.deepEqual(
    authVisibility.dx_style_compatibility.visible_surfaces,
    ["authentication-account-workflow", "authentication-session-status"],
  );
  assert.ok(
    authVisibility.dx_style_compatibility.source_files.includes(
      "examples/template/template-shell.tsx",
    ),
  );
  assert.deepEqual(authVisibility.receipt_hash_refresh, {
    schema: "dx.forge.package.receipt_hash_refresh",
    status: "current",
    helper_path: "examples/template/authentication-receipt-hashes.ts",
    check_command:
      "node tools/launch/run-template-receipt-helper.js examples/template/authentication-receipt-hashes.ts --check",
    write_command:
      "node tools/launch/run-template-receipt-helper.js examples/template/authentication-receipt-hashes.ts --write",
    json_check_command:
      "node tools/launch/run-template-receipt-helper.js examples/template/authentication-receipt-hashes.ts --check --json",
    source_guard_runbook_fixture: runbookFixturePath,
    studio_manifest_source: studioManifestSourcePath,
    preview_manifest_materializer: previewManifestMaterializerPath,
    receipt_path: "examples/template/.dx/forge/receipts/auth-better-auth.json",
    hash_algorithm: "sha256",
    tracked_file_count: authenticationReceiptFiles.length,
    tracked_files: authenticationReceiptFiles,
    current_files: authenticationReceiptFiles,
    stale_files: [],
    missing_files: [],
    stale_mirror_files: [],
    missing_mirror_files: [],
    mirror_problem_count: 0,
    stale_file_count: 0,
    missing_file_count: 0,
    runtime_execution: false,
    secret_access: false,
    zed_visibility: "authentication:receipt-hash-refresh",
    runtime_limitations: [
      "SOURCE-ONLY: this helper checks local Authentication receipt hash freshness only.",
      "ADAPTER-BOUNDARY: Better Auth credentials, provider callbacks, cookies, database adapters, email delivery, and hosted sessions stay app-owned.",
    ],
  });

  for (const metric of [
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
  ]) {
    assert.ok(
      authVisibility.dx_check_metrics.includes(metric),
      `${metric} missing from Authentication visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
  }

  assert.match(readModel, /export const authenticationPackageVisibility/);
  assert.match(readModel, /packageLaneVisibility: \[[\s\S]*packageId: "auth\/better-auth"/);
  assert.match(statusSource, /authenticationPackageVisibility/);
  assert.match(statusSource, /authenticationVisibility: authenticationPackageVisibility/);
  assert.match(packageLock, /checkAuthenticationPackageVisibility/);
  assert.match(packageLock, /authentication_receipt_present/);
  assert.match(readModel, /dxStyleCompatibility: \{/);
  assert.match(readModel, /receiptHashRefresh: \{/);
  assert.match(
    readModel,
    /sourceGuardRunbookFixture:\s*"docs\/packages\/authentication\.source-guard-runbook\.json"/,
  );
  assert.match(
    readModel,
    /studioManifestSource:\s*"dx-www\/src\/cli\/studio_manifest\.rs"/,
  );
  assert.match(readModel, /authentication:receipt-hash-refresh/);
  assert.match(readModel, /visibleSurfaces: \[\s*"authentication-account-workflow"/);
  assert.match(packageLock, /authentication_dx_style_compatibility_present/);
  assert.ok(status.zed_receipt_surfaces.includes("authentication-package-visibility"));
  assert.ok(
    status.zed_receipt_surfaces.includes("authentication:receipt-hash-refresh"),
  );
});
