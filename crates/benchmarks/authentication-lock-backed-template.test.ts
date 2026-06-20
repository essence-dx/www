import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { existsSync, mkdtempSync, readFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import test from "node:test";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const templateRoot = path.join(root, "examples", "template");
const forgeRoot = path.join(templateRoot, ".dx", "forge");
const packageId = "auth/better-auth";
const packageVersion = "1.6.11-dx.9";
const packageSlug = "auth-better-auth";
const appRouteSurfaceFile = "examples/template/app/api/auth/[...all]/route.ts";
const readinessRouteSurfaceFile = "examples/template/app/api/auth/readiness/route.ts";
const appServerBoundaryFile = "examples/template/server/auth/better-auth.ts";
const templateReadinessReceiptFile =
  "examples/template/.dx/forge/template-readiness/authentication.json";

const authSourceFiles = [
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
  "auth/better-auth/providers/google/config.ts",
  "auth/better-auth/providers/google/route.ts",
  "auth/better-auth/providers/google/callback.ts",
  "auth/better-auth/providers/google/.env.example",
  "auth/better-auth/providers/google/README.md",
];

function readJson<T>(filePath: string): T {
  return JSON.parse(readFileSync(filePath, "utf8")) as T;
}

function readTemplateFile(relativePath: string): string {
  return readFileSync(path.join(templateRoot, relativePath), "utf8");
}

function readRepoFile(relativePath: string): string {
  return readFileSync(path.join(root, relativePath), "utf8");
}

function assertNoUnselectedProviders(contents: string, label: string): void {
  assert.equal(contents.includes("GITHUB_"), false, `${label} must not require GitHub env`);
  assert.equal(contents.includes("provider: \"github\""), false, `${label} must not configure GitHub OAuth`);
  assert.equal(contents.includes("Continue with GitHub"), false, `${label} must not expose GitHub OAuth`);
  assert.equal(contents.includes("APPLE_"), false, `${label} must not require Apple env`);
}

test("Authentication is lock-backed, cached, and source-owned in the default template", () => {
  const status = readJson<any>(path.join(forgeRoot, "package-status.json"));
  const lock = readJson<any>(path.join(forgeRoot, "package-lock.json"));
  const sourceManifest = readJson<any>(path.join(forgeRoot, "source-.dx/build-cache/manifest.json"));
  const receipt = readJson<any>(path.join(forgeRoot, "receipts", "auth-better-auth.json"));
  const cacheManifestPath = path.join(forgeRoot, "cache", packageSlug, packageVersion, ".dx/build-cache/manifest.json");

  assert.ok(
    status.locked_package_names.includes(packageId),
    "package-status must count auth/better-auth as a locked Forge package",
  );
  assert.equal(
    status.locked_package_count,
    status.locked_package_names.length,
    "locked package count must match locked package names",
  );

  const statusRow = status.packages.find((entry: any) => entry.package_id === packageId);
  assert.ok(statusRow, "package-status must include the Authentication package row");
  assert.equal(statusRow.official_name, "Authentication");
  assert.equal(statusRow.upstream_name, "better-auth");
  assert.equal(statusRow.status, "lock-backed");
  assert.equal(
    statusRow.template_usage,
    "App Router auth routes, app-owned server boundary, readiness route, and login/logout/session/profile/missing-config gates",
  );
  assert.equal(statusRow.package_score, 87);

  const lockRow = lock.packages.find((entry: any) => entry.package_id === packageId);
  assert.ok(lockRow, "package-lock must include Authentication");
  assert.equal(lockRow.version, packageVersion);
  assert.equal(lockRow.upstream_name, "better-auth");
  assert.equal(lockRow.source_kind, "local-slice");
  assert.deepEqual(lockRow.required_env, [
    "BETTER_AUTH_SECRET",
    "BETTER_AUTH_URL",
    "GOOGLE_CLIENT_ID",
    "GOOGLE_CLIENT_SECRET",
  ]);
  assert.ok(
    lockRow.receipt_paths.includes(".dx/forge/receipts/packages/auth-better-auth.json"),
    "package lock must point at the package add receipt",
  );

  const sourceRow = sourceManifest.packages.find((entry: any) => entry.package_id === packageId);
  assert.ok(sourceRow, "source-manifest must include Authentication");
  assert.equal(sourceRow.version, packageVersion);
  assert.equal(sourceRow.upstream_name, "better-auth");
  assert.equal(sourceRow.source_kind, "curated-registry");
  assert.equal(sourceRow.provenance.source, "dx-forge-curated-registry");
  assert.equal(sourceRow.license_review.license, "MIT");

  assert.ok(existsSync(cacheManifestPath), "Authentication cache manifest must exist");
  const cacheManifest = readJson<any>(cacheManifestPath);

  for (const relativePath of authSourceFiles) {
    assert.ok(existsSync(path.join(templateRoot, relativePath)), `${relativePath} must exist in source`);
    assert.ok(
      sourceRow.files.some((entry: any) => entry.path === relativePath),
      `${relativePath} must be recorded in source-manifest`,
    );
    assert.ok(
      lockRow.files.some((entry: any) => entry.path === relativePath),
      `${relativePath} must be recorded in package-lock`,
    );
    assert.ok(
      cacheManifest.cached_files.some((entry: any) => entry.path === relativePath),
      `${relativePath} must be recorded in the cache manifest`,
    );
    assert.ok(
      existsSync(path.join(forgeRoot, "cache", packageSlug, packageVersion, relativePath)),
      `${relativePath} must be materialized in cache`,
    );
  }

  for (const relativePath of authSourceFiles) {
    assertNoUnselectedProviders(readTemplateFile(relativePath), relativePath);
  }

  const visibility = status.package_lane_visibility.find((entry: any) => entry.package_id === packageId);
  assert.ok(visibility, "package-status must include Authentication lane visibility");
  assert.ok(existsSync(path.join(root, appRouteSurfaceFile)), "template must include the Better Auth catch-all route source");
  assert.ok(receipt.file_hashes[appRouteSurfaceFile], "receipt must hash the Better Auth catch-all route source");
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface: any) =>
        surface.id === "authentication-app-route-handler" &&
        surface.file_hashes?.[appRouteSurfaceFile],
    ),
    "receipt dx-check visibility must track the Better Auth catch-all route source",
  );
  assert.ok(
    visibility.selected_surfaces.some(
      (surface: any) =>
        surface.surface_id === "authentication-app-route-handler" &&
        surface.file_hashes?.[appRouteSurfaceFile],
    ),
    "package-status selected surfaces must track the Better Auth catch-all route source",
  );
  assert.ok(
    visibility.receipt_hash_refresh.tracked_files.includes(appRouteSurfaceFile),
    "receipt hash refresh must include the Better Auth catch-all route source",
  );

  assert.ok(
    existsSync(path.join(root, readinessRouteSurfaceFile)),
    "template must include the Authentication readiness route source",
  );
  assert.ok(
    receipt.file_hashes[readinessRouteSurfaceFile],
    "receipt must hash the Authentication readiness route source",
  );
  assert.ok(
    visibility.receipt_hash_refresh.tracked_files.includes(readinessRouteSurfaceFile),
    "receipt hash refresh must include the Authentication readiness route source",
  );
  assert.ok(
    existsSync(path.join(root, appServerBoundaryFile)),
    "template must include the app-owned Better Auth server boundary source",
  );
  assert.ok(
    receipt.file_hashes[appServerBoundaryFile],
    "receipt must hash the app-owned Better Auth server boundary source",
  );
  assert.ok(
    visibility.selected_surfaces.some(
      (surface: any) =>
        surface.surface_id === "authentication-app-server-boundary" &&
        surface.file_hashes?.[appServerBoundaryFile],
    ),
    "package-status selected surfaces must track the app-owned Better Auth server boundary",
  );
  assert.ok(
    visibility.receipt_hash_refresh.tracked_files.includes(appServerBoundaryFile),
    "receipt hash refresh must include the app-owned Better Auth server boundary",
  );

  assert.ok(
    existsSync(path.join(root, templateReadinessReceiptFile)),
    "Authentication must publish a concrete template-readiness receipt instead of relying on a missing folder",
  );
  const readinessReceipt = readJson<any>(path.join(root, templateReadinessReceiptFile));
  assert.equal(readinessReceipt.schema, "dx.forge.template_readiness.package");
  assert.equal(readinessReceipt.package_id, packageId);
  assert.equal(readinessReceipt.official_package_name, "Authentication");
  assert.equal(readinessReceipt.upstream_package, "better-auth");
  assert.equal(readinessReceipt.classification, "lock-backed-adapter-boundary");
  assert.equal(readinessReceipt.runtime_proof, false);
  assert.equal(readinessReceipt.live_oauth_execution, false);
  assert.equal(readinessReceipt.readiness_route, "/api/auth/readiness");
  assert.equal(readinessReceipt.app_route_handler, "app/api/auth/[...all]/route.ts");
  assert.equal(readinessReceipt.server_boundary, "server/auth/better-auth.ts");
  assert.deepEqual(readinessReceipt.selected_provider_surfaces, ["google-oauth"]);
  assert.ok(
    readinessReceipt.blocked_until_configured.includes("database adapter"),
    "template-readiness receipt must keep the database adapter caveat explicit",
  );
  assert.ok(
    receipt.file_hashes[templateReadinessReceiptFile],
    "Authentication receipt must hash the template-readiness receipt",
  );
  assert.ok(
    visibility.selected_surfaces.some(
      (surface: any) =>
        surface.surface_id === "authentication-template-readiness-receipt" &&
        surface.file_hashes?.[templateReadinessReceiptFile],
    ),
    "package-status selected surfaces must track the Authentication template-readiness receipt",
  );
  assert.ok(
    visibility.receipt_hash_refresh.tracked_files.includes(templateReadinessReceiptFile),
    "receipt hash refresh must include the Authentication template-readiness receipt",
  );
});

test("Authentication appears as real template UI with Google-only missing-config gates", () => {
  const files = new Map([
    ["auth-session-status", readTemplateFile("auth-session-status.tsx")],
    ["auth-pages", readTemplateFile("components/template-app/auth-pages.tsx")],
    ["dashboard-page", readTemplateFile("components/template-app/dashboard-page.tsx")],
    ["package-reality", readTemplateFile("components/template-app/package-reality.ts")],
    ["template-data", readTemplateFile("components/template-app/template-data.ts")],
    ["login-html", readRepoFile("tools/launch/runtime-template/pages/login.html")],
    ["logout-html", readRepoFile("tools/launch/runtime-template/pages/logout.html")],
    ["launch-runtime", readRepoFile("tools/launch/runtime-template/assets/launch-runtime.ts")],
  ]);

  for (const [label, contents] of files) {
    assertNoUnselectedProviders(contents, label);
  }

  const sessionStatus = files.get("auth-session-status") ?? "";
  assert.match(sessionStatus, /from "\@\/auth\/better-auth\/client"/);
  assert.match(sessionStatus, /from "\@\/auth\/better-auth\/dashboard"/);
  assert.match(sessionStatus, /from "\@\/auth\/better-auth\/profile"/);
  assert.match(sessionStatus, /data-auth-profile="ready"/);
  assert.match(sessionStatus, /provider: "google"/);

  const authPages = files.get("auth-pages") ?? "";
  assert.match(authPages, /data-dx-auth-config="missing-config"/);
  assert.match(authPages, /data-dx-auth-session-source="missing-config"/);
  assert.match(authPages, /data-dx-auth-provider="google"/);
  assert.match(authPages, /data-dx-auth-readiness-endpoint="\/api\/auth\/readiness"/);
  assert.match(authPages, /data-dx-auth-missing-config="BETTER_AUTH_SECRET,BETTER_AUTH_URL,GOOGLE_CLIENT_ID,GOOGLE_CLIENT_SECRET"/);

  const dashboardPage = files.get("dashboard-page") ?? "";
  assert.match(dashboardPage, /data-dx-component="template-auth-session-panel"/);
  assert.match(dashboardPage, /data-dx-auth-profile-gate="missing-config"/);
  assert.match(dashboardPage, /data-dx-auth-readiness-endpoint="\/api\/auth\/readiness"/);

  const packageReality = files.get("package-reality") ?? "";
  assert.match(packageReality, /packageId: "auth\/better-auth"/);
  assert.match(packageReality, /controlId: "auth-readiness"/);
  assert.match(packageReality, /score: 87/);
  assert.match(packageReality, /App Router auth routes, app-owned server boundary, readiness route, and login\/logout\/session\/profile\/missing-config gates/);

  const appServerBoundary = readTemplateFile("server/auth/better-auth.ts");
  assert.match(appServerBoundary, /createTemplateBetterAuthReadiness/);
  assert.match(appServerBoundary, /createTemplateBetterAuthRouteHandlers/);
  assert.match(appServerBoundary, /BetterAuthOptions\["database"\]/);
  assert.match(appServerBoundary, /dxTemplateBetterAuthDatabaseBoundary/);
  assert.match(appServerBoundary, /runtimeProof: false/);
  assert.match(appServerBoundary, /appOwned: true/);
  assert.match(appServerBoundary, /migrationsRequired: !readiness\.databaseAdapterConfigured/);
  assert.match(appServerBoundary, /createDxBetterAuthRouteHandlers\(input\)/);

  const authServer = readTemplateFile("auth/better-auth/server.ts");
  assert.match(authServer, /databaseAdapterConfigured/);
  assert.match(authServer, /sessionStorage: databaseAdapterConfigured \? "database-adapter" : "app-owned"/);
  assert.match(authServer, /canRunRouteHandlers: config\.configured && databaseAdapterConfigured/);
  assert.match(authServer, /database adapter, session storage, migrations/);

  const readinessRoute = readTemplateFile("app/api/auth/readiness/route.ts");
  assert.match(readinessRoute, /createTemplateBetterAuthReadiness/);
  assert.match(readinessRoute, /from "\@\/server\/auth\/better-auth"/);
  assert.match(readinessRoute, /export const runtime = "nodejs"/);
  assert.match(readinessRoute, /Response\.json/);
  assert.match(readinessRoute, /\{ status: 200 \}/);
  assert.match(readinessRoute, /liveRouteHandlersHttpStatus: readiness\.canRunRouteHandlers \? 200 : 501/);
  assert.match(readinessRoute, /adapterBoundaries/);
  assertNoUnselectedProviders(readinessRoute, "Authentication readiness route");

  const catchAllRoute = readTemplateFile("app/api/auth/[...all]/route.ts");
  assert.match(catchAllRoute, /export \{ GET, POST \} from "\@\/server\/auth\/better-auth"/);
  assertNoUnselectedProviders(catchAllRoute, "Authentication catch-all route");
});

test("materializer emits Authentication source, App Router auth route, and no fake OAuth", () => {
  const outputRoot = mkdtempSync(path.join(tmpdir(), "dx-auth-materialized-"));

  execFileSync(
    "node",
    [
      path.join(root, "tools", "launch", "materialize-www-template.ts"),
      outputRoot,
    ],
    { cwd: templateRoot, stdio: "pipe" },
  );

  const emittedAuthSource = path.join(outputRoot, "auth", "better-auth", "providers", "google", "config.ts");
  assert.ok(existsSync(emittedAuthSource), "materializer must emit the selected Google provider source");
  assert.equal(existsSync(path.join(outputRoot, "auth", "google")), false, "materializer must not create auth/google");
  assert.equal(
    existsSync(path.join(outputRoot, "auth", "better-auth", "providers", "github")),
    false,
    "materializer must not emit an unselected GitHub provider",
  );

  const login = readFileSync(path.join(outputRoot, "pages", "login.html"), "utf8");
  assert.match(login, /data-dx-auth-provider="google"/);
  assert.match(login, /data-dx-auth-config="missing-config"/);
  assertNoUnselectedProviders(login, "materialized login page");

  const dashboard = readFileSync(path.join(outputRoot, "pages", "dashboard.html"), "utf8");
  assert.match(dashboard, /data-dx-component="template-auth-session-panel"/);
  assert.match(dashboard, /data-dx-auth-profile-gate="missing-config"/);
  assertNoUnselectedProviders(dashboard, "materialized dashboard page");

  const routeHandler = readFileSync(path.join(outputRoot, "app", "api", "auth", "session", "route.ts"), "utf8");
  assert.match(routeHandler, /Response\.json/);
  assert.match(routeHandler, /adapter: "better-auth"/);
  assert.match(routeHandler, /credentialsConfigured: false/);
  assert.match(routeHandler, /GOOGLE_CLIENT_ID/);
  assertNoUnselectedProviders(routeHandler, "materialized auth route handler");

  const catchAllRoute = readFileSync(
    path.join(outputRoot, "app", "api", "auth", "[...all]", "route.ts"),
    "utf8",
  );
  assert.match(catchAllRoute, /export const runtime = "nodejs";/);
  assert.match(catchAllRoute, /export \{ GET, POST \} from "\@\/server\/auth\/better-auth";/);
  assertNoUnselectedProviders(catchAllRoute, "materialized Better Auth catch-all route");

  const readinessRoute = readFileSync(
    path.join(outputRoot, "app", "api", "auth", "readiness", "route.ts"),
    "utf8",
  );
  assert.match(readinessRoute, /createTemplateBetterAuthReadiness/);
  assert.match(readinessRoute, /from "\@\/server\/auth\/better-auth"/);
  assert.match(readinessRoute, /\{ status: 200 \}/);
  assert.match(readinessRoute, /liveRouteHandlersHttpStatus: readiness\.canRunRouteHandlers \? 200 : 501/);
  assert.match(readinessRoute, /databaseAdapterConfigured/);
  assertNoUnselectedProviders(readinessRoute, "materialized Authentication readiness route");

  const serverBoundary = readFileSync(
    path.join(outputRoot, "server", "auth", "better-auth.ts"),
    "utf8",
  );
  assert.match(serverBoundary, /dxTemplateBetterAuthDatabaseBoundary/);
  assert.match(serverBoundary, /createTemplateBetterAuthRouteHandlers/);
  assert.match(serverBoundary, /runtimeProof: false/);
  assert.match(serverBoundary, /\{ status: 501 \}/);
  assertNoUnselectedProviders(serverBoundary, "materialized app-owned Better Auth server boundary");
});
