const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const inspirationRoot = path.resolve(
  root,
  "..",
  "..",
  "WWW",
  "inspirations",
  "better-auth",
);

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readInspiration(relativePath) {
  return fs.readFileSync(path.join(inspirationRoot, relativePath), "utf8");
}

function readRequired(relativePath) {
  const file = path.join(root, relativePath);
  assert.ok(fs.existsSync(file), `${relativePath} should exist`);
  return fs.readFileSync(file, "utf8");
}

function sliceBetween(text, startMarker, endMarker) {
  const startIndex = text.indexOf(startMarker);
  assert.notEqual(startIndex, -1, `${startMarker} should exist`);
  const endIndex = text.indexOf(endMarker, startIndex + startMarker.length);
  assert.notEqual(endIndex, -1, `${endMarker} should exist after ${startMarker}`);
  return text.slice(startIndex, endIndex);
}

test("auth/better-auth exposes a dashboard account workflow, not only launch metadata", () => {
  const upstreamPackage = readInspiration("packages/better-auth/package.json");
  const upstreamReactClient = readInspiration(
    "packages/better-auth/src/client/react/index.ts",
  );
  const upstreamNext = readInspiration(
    "packages/better-auth/src/integrations/next-js.ts",
  );
  const forgeRegistry = read("core/src/ecosystem/forge_registry.rs");
  const catalog = read("examples/template/package-catalog.ts");
  const dashboardPage = readRequired("examples/dashboard/src/pages/Dashboard.tsx");
  const dashboardWorkflow = readRequired(
    "examples/dashboard/src/components/BetterAuthAccountWorkflow.tsx",
  );
  const dashboardPackage = readRequired(
    "examples/dashboard/src/lib/forge/auth/better-auth/dashboard.ts",
  );
  const dashboardReadme = readRequired("examples/dashboard/README.md");
  const launchRoute = read("examples/template/app/page.tsx");
  const templateRouteContract = read("examples/template/template-route-contract.ts");
  const launchShell = read("examples/template/template-shell.tsx");
  const launchAuthStatus = read("examples/template/auth-session-status.tsx");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const templateSurfaceRegistry = read("examples/template/template-surface-registry.ts");
  const launchRuntimePage = read("tools/launch/runtime-template/pages/index.html");
  const launchRuntime = read("tools/launch/runtime-template/assets/launch-runtime.ts");
  const runtimeMaterializer = read("tools/launch/materialize-www-template.ts");
  const cli = read("dx-www/src/cli/mod.rs");
  const newCommand = read("dx-www/src/cli/new_command.rs");
  const betterAuthCliMetadata = sliceBetween(
    cli,
    '"package_id": "auth/better-auth"',
    '"package_id": "animation/motion"',
  );
  const forgeReceipt = JSON.parse(
    readRequired("examples/template/.dx/forge/receipts/auth-better-auth.json"),
  );
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");
  const dx = read("DX.md");

  assert.match(upstreamPackage, /"version": "1\.6\.11"/);
  assert.match(upstreamPackage, /"\.\/react"/);
  assert.match(upstreamPackage, /"\.\/next-js"/);
  assert.match(upstreamReactClient, /export function createAuthClient/);
  assert.match(upstreamReactClient, /useSession: \(\) =>/);
  assert.match(upstreamNext, /export function toNextJsHandler/);
  assert.match(upstreamNext, /export const nextCookies/);

  assert.match(forgeRegistry, /"js\/auth\/better-auth\/dashboard\.ts"/);
  assert.match(forgeRegistry, /createDxBetterAuthDashboardProfileRequest/);
  assert.match(forgeRegistry, /createDxBetterAuthDashboardActionReceipt/);
  assert.match(forgeRegistry, /dxBetterAuthDashboardActions/);
  assert.match(forgeRegistry, /"auth\/better-auth\/dashboard\.ts"/);
  assert.match(forgeRegistry, /dashboardUsage: \{/);
  assert.match(forgeRegistry, /launchRoute: "examples\/template\/app\/page\.tsx"/);
  assert.match(forgeRegistry, /launchShell: "examples\/template\/template-shell\.tsx"/);
  assert.match(forgeRegistry, /componentMarker: "better-auth-account-dashboard-workflow"/);
  assert.match(forgeRegistry, /sessionStatusMarker: "better-auth-session-status-panel"/);
  assert.match(forgeRegistry, /emailSignUpMarker: "data-dx-auth-interaction=\\"email-sign-up\\""/);
  assert.match(forgeRegistry, /boundaryReviewMarker: "data-dx-auth-interaction=\\"mark-boundary-reviewed\\""/);
  assert.match(forgeRegistry, /networkGateMarker: "data-dx-auth-network-state"/);
  assert.match(forgeRegistry, /emailSignUpHelper: "signUpDxBetterAuthEmail\(input\)"/);
  assert.match(forgeRegistry, /"authClient\.updateUser\(\)"/);
  assert.match(forgeRegistry, /"authClient\.changeEmail\(\)"/);
  assert.match(forgeRegistry, /"authClient\.listAccounts\(\)"/);
  assert.match(forgeRegistry, /"authClient\.linkSocial\(\)"/);
  assert.match(forgeRegistry, /"authClient\.revokeOtherSessions\(\)"/);

  assert.match(catalog, /packageId: "auth\/better-auth"/);
  assert.match(
    catalog,
    /aliases: \["authentication", "better-auth", "auth\/betterauth", "auth\/better-auth-next"\]/,
  );
  assert.match(catalog, /sourceMirror: "G:\/WWW\/inspirations\/better-auth"/);
  assert.match(catalog, /"auth\/better-auth\/dashboard\.ts"/);
  assert.match(catalog, /"examples\/template\/app\/page\.tsx"/);
  assert.match(catalog, /"examples\/template\/auth-session-status\.tsx"/);
  assert.match(catalog, /"tools\/launch\/runtime-template\/pages\/index\.html"/);
  assert.match(catalog, /dashboardUsage: \{/);
  assert.match(catalog, /sourceFile: "examples\/template\/template-shell\.tsx"/);
  assert.match(catalog, /'data-dx-section="account-access-dashboard"'/);
  assert.match(catalog, /'data-dx-component="better-auth-session-status-panel"'/);
  assert.match(catalog, /'data-dx-auth-interaction="email-sign-up"'/);
  assert.match(catalog, /'data-dx-auth-interaction="mark-boundary-reviewed"'/);
  assert.match(catalog, /'data-dx-auth-network-state'/);
  assert.match(
    catalog,
    /"examples\/template\/auth-session-status\.tsx#better-auth-session-status-panel"/,
  );
  assert.match(catalog, /dxIcon: "pack:auth"/);
  assert.match(catalog, /"examples\/dashboard\/src\/components\/BetterAuthAccountWorkflow\.tsx"/);
  assert.match(catalog, /"createDxBetterAuthDashboardActionReceipt"/);

  assert.match(
    dashboardPage,
    /import \{ BetterAuthAccountWorkflow \} from '\.\.\/components\/BetterAuthAccountWorkflow';/,
  );
  assert.match(dashboardPage, /<BetterAuthAccountWorkflow \/>/);

  assert.match(dashboardPackage, /packageId: 'auth\/better-auth'/);
  assert.match(
    dashboardPackage,
    /sourceMirror: 'G:\/WWW\/inspirations\/better-auth'/,
  );
  assert.match(dashboardPackage, /requiredEnv: \[/);
  assert.match(dashboardPackage, /appOwnedBoundaries: \[/);
  assert.match(dashboardPackage, /receiptPaths: \[/);
  assert.match(dashboardPackage, /dxBetterAuthDashboardActions/);
  assert.match(dashboardPackage, /createDxBetterAuthDashboardSessionSnapshot/);
  assert.match(dashboardPackage, /createDxBetterAuthDashboardProfileRequest/);
  assert.match(dashboardPackage, /createDxBetterAuthDashboardActionReceipt/);
  assert.match(dashboardPackage, /authClient\.updateUser\(\)/);
  assert.match(dashboardPackage, /authClient\.changeEmail\(\)/);
  assert.match(dashboardPackage, /authClient\.signIn\.social\(\)/);
  assert.match(dashboardPackage, /auth\.api\.getSession\(\{ headers \}\)/);

  assert.match(dashboardWorkflow, /data-dx-package="auth\/better-auth"/);
  assert.match(
    dashboardWorkflow,
    /data-dx-component="dashboard-better-auth-account-workflow"/,
  );
  assert.match(
    dashboardWorkflow,
    /data-dx-auth-dashboard-workflow="session-profile-account"/,
  );
  assert.match(dashboardWorkflow, /data-dx-icon-search="auth:account"/);
  assert.match(dashboardWorkflow, /data-dx-node-modules="forbidden"/);
  assert.match(dashboardWorkflow, /data-dx-style-surface="theme-token-card"/);
  assert.match(dashboardWorkflow, /<dx-icon name="pack:auth"/);
  assert.match(dashboardWorkflow, /data-dx-auth-action="select-dashboard-action"/);
  assert.match(dashboardWorkflow, /data-dx-auth-action="prepare-dashboard-auth-receipt"/);
  assert.match(dashboardWorkflow, /data-dx-auth-profile-field="name"/);
  assert.match(dashboardWorkflow, /data-dx-auth-provider-option=/);
  assert.match(dashboardWorkflow, /data-dx-auth-dashboard-receipt/);
  assert.match(dashboardWorkflow, /createDxBetterAuthDashboardActionReceipt/);
  assert.doesNotMatch(dashboardWorkflow, /#[0-9a-fA-F]{3,8}/);
  assert.doesNotMatch(
    dashboardWorkflow,
    /\b(?:bg|text|border)-(?:neutral|slate|zinc|stone|gray)-/,
  );

  assert.match(dashboardReadme, /### Authentication account workflow/);
  assert.match(
    dashboardReadme,
    /examples\/template\/\.dx\/forge\/receipts\/auth-better-auth\.json/,
  );
  assert.match(launchRoute, /import \{ TemplateShell \}/);
  assert.match(launchRoute, /<TemplateShell \/>/);
  assert.match(templateRouteContract, /\.dx\/forge\/receipts\/auth-better-auth\.json/);
  assert.match(
    templateRouteContract,
    /providerEnv: \[\s*"GOOGLE_CLIENT_ID",\s*"GOOGLE_CLIENT_SECRET",?\s*\]/,
  );
  assert.doesNotMatch(templateRouteContract, /GITHUB_CLIENT_ID|GITHUB_CLIENT_SECRET/);
  assert.match(templateRouteContract, /"data-dx-auth-provider"/);
  assert.match(
    launchShell,
    /data-dx-component="better-auth-account-dashboard-workflow"/,
  );
  assert.match(launchShell, /function LaunchAccountAccessDashboard\(\)/);
  assert.match(launchShell, /<LaunchAccountAccessDashboard \/>/);
  assert.match(launchShell, /data-dx-section="account-access-dashboard"/);
  assert.match(launchShell, /data-dx-dashboard-card="account-access"/);
  assert.match(launchShell, /data-dx-edit-id="launch\.account-access-dashboard"/);
  assert.match(launchShell, /data-dx-edit-kind="dashboard-workflow"/);
  assert.match(
    launchShell,
    /selector: '\[data-dx-component="better-auth-account-dashboard-workflow"\]'/,
  );
  assert.doesNotMatch(
    launchShell,
    /id="auth"\s+data-dx-component="better-auth-account-dashboard-workflow"/,
  );
  assert.ok(
    launchShell.indexOf("<LaunchAccountAccessDashboard />") > -1 &&
      launchShell.indexOf("<LaunchAccountAccessDashboard />") <
        launchShell.indexOf('data-dx-section="proof-grid"'),
    "Better Auth account access must be a dashboard workflow before the proof grid",
  );
  assert.match(
    launchAuthStatus,
    /data-dx-component="better-auth-session-status-panel"/,
  );
  assert.match(
    launchAuthStatus,
    /data-dx-parent-component="better-auth-account-dashboard-workflow"/,
  );
  assert.match(
    launchAuthStatus,
    /data-dx-auth-dashboard-workflow="login-session-account"/,
  );
  assert.match(launchAuthStatus, /data-dx-product-surface="account-access"/);
  assert.match(launchAuthStatus, /data-dx-auth-missing-provider-state=/);
  assert.match(launchAuthStatus, /data-dx-auth-interaction="mark-boundary-reviewed"/);
  assert.match(launchAuthStatus, /data-dx-auth-session-source=/);
  assert.match(launchAuthStatus, /<dx-icon name="pack:auth"/);
  assert.match(
    launchRuntimePage,
    /data-dx-component="better-auth-account-dashboard-workflow"/,
  );
  assert.match(
    launchRuntimePage,
    /data-dx-auth-dashboard-workflow="login-session-account"/,
  );
  assert.match(launchRuntimePage, /data-dx-auth-network-state="missing-config"/);
  assert.match(launchRuntimePage, /data-dx-auth-network-disabled="true"/);
  assert.match(launchRuntimePage, /data-dx-auth-provider-option="google"/);
  assert.doesNotMatch(launchRuntimePage, /data-dx-auth-provider-option="github"/);
  assert.match(launchRuntimePage, /data-dx-auth-interaction="email-sign-up"/);
  assert.match(launchRuntimePage, /data-dx-auth-interaction="mark-boundary-reviewed"/);
  assert.match(launchRuntimePage, /data-dx-auth-session-source="boundary-review"/);
  assert.match(launchRuntimePage, /data-dx-auth-boundary-review="idle"/);
  assert.match(launchRuntimePage, /data-dx-auth-dashboard-receipt="none"/);
  assert.match(launchRuntime, /mission-auth-preview/);
  assert.match(launchRuntime, /data-dx-auth-network-state/);
  assert.match(launchRuntime, /mission-auth-boundary-review/);
  assert.match(launchRuntime, /function setBetterAuthRuntimeState\(/);
  assert.match(
    launchRuntime,
    /setBetterAuthRuntimeState\("email-sign-up-missing-config"/,
  );
  assert.match(
    launchRuntime,
    /setBetterAuthRuntimeState\("boundary-review-marked"/,
  );
  assert.match(launchRuntime, /function markBoundaryReviewed\(\)/);
  assert.match(launchRuntime, /data-dx-auth-interaction='email-sign-up'/);
  assert.match(launchRuntime, /data-dx-auth-session-source/);
  assert.match(launchRuntime, /data-dx-auth-boundary-review/);
  assert.match(
    launchRuntime,
    /Email sign-up is correctly blocked until BETTER_AUTH_SECRET/,
  );
  assert.doesNotMatch(launchRuntime, /upsertAccount|accountsKey|local-account-created/);
  assert.match(launchRuntime, /data-dx-auth-dashboard-receipt/);
  assert.match(launchRuntime, /better-auth-dashboard-link-provider/);
  assert.match(
    editContract,
    /id: "better-auth-account-dashboard-workflow",\s+selector: '\[data-dx-component="better-auth-account-dashboard-workflow"\]',\s+sourceFile: "examples\/template\/template-shell\.tsx"/,
  );
  assert.match(
    editContract,
    /id: "better-auth-session-status-panel",\s+selector: '\[data-dx-component="better-auth-session-status-panel"\]',\s+sourceFile: "examples\/template\/auth-session-status\.tsx"/,
  );
  assert.match(
    templateSurfaceRegistry,
    /receiptPaths:\s+\[\s+"\.dx\/forge\/receipts\/auth-better-auth\.json"/,
  );
  assert.match(
    templateSurfaceRegistry,
    /requiredEnv: \["BETTER_AUTH_SECRET", "BETTER_AUTH_URL"\]/,
  );
  assert.match(
    templateSurfaceRegistry,
    /providerEnv: \[\s*"GOOGLE_CLIENT_ID",\s*"GOOGLE_CLIENT_SECRET",?\s*\]/,
  );
  assert.doesNotMatch(templateSurfaceRegistry, /GITHUB_CLIENT_ID|GITHUB_CLIENT_SECRET/);
  assert.match(templateSurfaceRegistry, /"data-dx-auth-provider"/);
  assert.match(
    cli,
    /const NEXT_FAMILIAR_BETTER_AUTH_DASHBOARD_RECEIPT_JSON: &str =\s*include_str!\(/,
  );
  assert.match(
    cli,
    /"\.\.\/\.\.\/\.\.\/examples\/template\/\.dx\/forge\/receipts\/auth-better-auth\.json"/,
  );
  assert.match(
    newCommand,
    /let better_auth_dashboard_receipt = NEXT_FAMILIAR_BETTER_AUTH_DASHBOARD_RECEIPT_JSON;/,
  );
  assert.match(
    newCommand,
    /\(\s*"\.dx\/forge\/receipts\/auth-better-auth\.json",\s*better_auth_dashboard_receipt,\s*\)/,
  );
  assert.match(
    newCommand,
    /path: "\.dx\/forge\/receipts\/auth-better-auth\.json"\.to_string\(\),\s*content: NEXT_FAMILIAR_BETTER_AUTH_DASHBOARD_RECEIPT_JSON\.to_string\(\),/s,
  );
  assert.match(cli, /"\.dx\/forge\/receipts\/auth-better-auth\.json",/);
  assert.match(
    cli,
    /serde_json::json!\(\{\s*"package_id": "auth\/better-auth"[\s\S]*"dashboard_usage": "The root dashboard uses LaunchAuthSessionStatus[\s\S]*"receipt_paths": \[\s*"\.dx\/forge\/receipts\/auth-better-auth\.json",\s*"examples\/template\/\.dx\/forge\/receipts\/auth-better-auth\.json"\s*\]/,
  );
  for (const exportedFile of [
    "auth/better-auth/account-deletion.ts",
    "auth/better-auth/route.ts",
    "auth/better-auth/.env.example",
    "auth/better-auth/README.md",
  ]) {
    assert.match(
      betterAuthCliMetadata,
      new RegExp(exportedFile.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `${exportedFile} should be discoverable from Better Auth CLI metadata`,
    );
  }
  assert.match(betterAuthCliMetadata, /"provenance"/);
  assert.match(
    betterAuthCliMetadata,
    /"source": "dx-forge-curated-registry"/,
  );
  assert.match(
    betterAuthCliMetadata,
    /"provider_env": \[\s*"GOOGLE_CLIENT_ID",\s*"GOOGLE_CLIENT_SECRET"\s*\]/,
  );
  assert.doesNotMatch(betterAuthCliMetadata, /GITHUB_CLIENT_ID|GITHUB_CLIENT_SECRET/);
  assert.match(
    betterAuthCliMetadata,
    /"credential_state": "missing-config"/,
  );
  assert.match(runtimeMaterializer, /function materializeForgeReceipts/);
  assert.match(runtimeMaterializer, /\.filter\(\(name\) => name\.endsWith\("\.json"\)\)/);
  assert.equal(forgeReceipt.package_id, "auth/better-auth");
  assert.equal(forgeReceipt.source_mirror, "G:/WWW/inspirations/better-auth");
  assert.equal(forgeReceipt.dashboard_workflow, "login-session-account");
  assert.equal(forgeReceipt.node_modules_policy, "no-template-local-node_modules");
  assert.ok(forgeReceipt.source_files.includes("examples/template/auth-session-status.tsx"));
  assert.ok(forgeReceipt.source_files.includes("examples/template/template-shell.tsx"));
  assert.ok(forgeReceipt.materialized_files.includes(".dx/forge/receipts/auth-better-auth.json"));
  assert.ok(forgeReceipt.stable_markers.includes('data-dx-package="auth/better-auth"'));
  assert.ok(
    forgeReceipt.stable_markers.includes(
      'data-dx-component="better-auth-account-dashboard-workflow"',
    ),
  );
  assert.ok(
    forgeReceipt.public_apis.includes("createDxBetterAuthDashboardActionReceipt"),
  );
  assert.ok(forgeReceipt.safe_local_interaction);
  assert.equal(forgeReceipt.runtime_execution, false);
  assert.match(todo, /Authentication dashboard account workflow/);
  assert.match(changelog, /Authentication dashboard account workflow/);
  assert.match(dx, /Authentication dashboard account workflow/);
});
