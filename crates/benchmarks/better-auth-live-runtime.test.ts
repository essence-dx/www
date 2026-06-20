const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const materializer = path.join(root, "tools", "launch", "materialize-www-template.ts");

function read(file) {
  return fs.readFileSync(file, "utf8");
}

test("auth/better-auth is visible and interactive in the materialized /launch runtime", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-better-auth-launch-"));

  const output = execFileSync(process.execPath, [materializer, dir], {
    cwd: root,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  });
  const result = JSON.parse(output);

  assert.equal(result.ok, true);
  assert.equal(result.noNodeModules, true);
  assert.equal(fs.existsSync(path.join(dir, "node_modules")), false);

  const launch = read(path.join(dir, "pages", "index.html"));
  assert.match(launch, /data-dx-package="auth\/better-auth"/);
  assert.match(launch, /data-dx-component="better-auth-boundary-review"/);
  assert.match(launch, /data-dx-component="auth-session-card"/);
  assert.match(launch, /data-dx-auth-boundary-surface="session-readiness"/);
  assert.match(launch, /data-dx-auth-config="missing-config"/);
  assert.match(launch, /data-dx-auth-local-session="not-created"/);
  assert.match(launch, /data-dx-auth-safe-action-state="idle"/);
  assert.match(launch, /data-dx-auth-storage-key="dx-better-auth-boundary-review"/);
  assert.match(launch, /data-dx-auth-readiness-card="missing-config"/);
  assert.match(
    launch,
    /data-dx-auth-missing-config="BETTER_AUTH_SECRET,BETTER_AUTH_URL,GOOGLE_CLIENT_ID,GOOGLE_CLIENT_SECRET"/,
  );
  assert.match(launch, /data-dx-auth-provider="google"/);
  assert.doesNotMatch(launch, /data-dx-auth-provider="github"/);
  assert.doesNotMatch(launch, /GITHUB_CLIENT_ID|GITHUB_CLIENT_SECRET/);
  assert.match(launch, /data-dx-auth-provider-state="missing-config"/);
  assert.match(launch, /id="session-boundary-email"/);
  assert.match(launch, /data-dx-auth-interaction="boundary-email-field"/);
  assert.match(launch, /data-dx-auth-boundary-email-input="true"/);
  assert.match(launch, /data-dx-auth-interaction="safe-sign-in-preview"/);
  assert.match(launch, /data-dx-auth-interaction="clear-boundary-review"/);
  assert.match(launch, /data-dx-auth-boundary-review-state="idle"/);
  assert.match(launch, /data-dx-auth-boundary-review-user="none"/);
  assert.match(launch, /data-dx-auth-session-endpoint="\/api\/auth\/session"/);
  assert.match(launch, /Authentication provider credentials are not configured/);

  const runtime = read(path.join(dir, "public", "launch-runtime.js"));
  assert.match(runtime, /function bindSession\(\)/);
  assert.match(runtime, /better-auth-boundary-review/);
  assert.match(runtime, /data-dx-auth-local-session/);
  assert.match(runtime, /data-dx-auth-safe-action-state/);
  assert.match(runtime, /data-dx-auth-boundary-review-state/);
  assert.match(runtime, /dx-better-auth-boundary-review/);
  assert.match(runtime, /sessionStorage\.setItem/);
  assert.match(runtime, /data-dx-auth-boundary-email/);
  assert.match(runtime, /data-dx-auth-boundary-review-user/);
  assert.match(runtime, /fetch\("\/api\/auth\/session"\)/);

  const route = read(path.join(dir, "app", "api", "auth", "session", "route.ts"));
  assert.match(route, /adapter: "better-auth"/);
  assert.match(route, /credentialsConfigured: false/);
  assert.match(route, /appOwnedBoundary:/);

  const catchAllRoute = read(path.join(dir, "app", "api", "auth", "[...all]", "route.ts"));
  assert.match(catchAllRoute, /export const runtime = "nodejs";/);
  assert.match(catchAllRoute, /export \{ GET, POST \} from "\@\/server\/auth\/better-auth";/);
  assert.doesNotMatch(catchAllRoute, /GITHUB_|APPLE_|auth\/google/);

  const readinessRoute = read(path.join(dir, "app", "api", "auth", "readiness", "route.ts"));
  assert.match(readinessRoute, /createTemplateBetterAuthReadiness/);
  assert.match(readinessRoute, /databaseAdapterConfigured/);
  assert.match(readinessRoute, /\{ status: 200 \}/);
  assert.match(readinessRoute, /liveRouteHandlersHttpStatus: readiness\.canRunRouteHandlers \? 200 : 501/);
  assert.doesNotMatch(readinessRoute, /GITHUB_|APPLE_|auth\/google/);

  const serverBoundary = read(path.join(dir, "server", "auth", "better-auth.ts"));
  assert.match(serverBoundary, /dxTemplateBetterAuthDatabaseBoundary/);
  assert.match(serverBoundary, /createTemplateBetterAuthRouteHandlers/);
  assert.match(serverBoundary, /runtimeProof: false/);
  assert.match(serverBoundary, /\{ status: 501 \}/);
  assert.doesNotMatch(serverBoundary, /GITHUB_|APPLE_|auth\/google/);

  const manifest = JSON.parse(read(path.join(dir, "public", "preview-.dx/build-cache/manifest.json")));
  assert.equal(manifest.noNodeModulesRequired, true);
  assert.ok(
    manifest.routes.some(
      (route) => route.route === "/" && route.forgePackages.includes("auth/better-auth"),
    ),
  );
});
