const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

test("Authentication lane uses the official DX package name and keeps better-auth as provenance", () => {
  const docs = read("docs/packages/authentication.md");
  const registry = read("core/src/ecosystem/forge_registry.rs");
  const catalog = read("examples/template/package-catalog.ts");
  const receipt = readJson(
    "examples/template/.dx/forge/receipts/auth-better-auth.json",
  );
  const dashboardPackage = read(
    "examples/dashboard/src/lib/forge/auth/better-auth/dashboard.ts",
  );
  const dashboardWorkflow = read(
    "examples/dashboard/src/components/BetterAuthAccountWorkflow.tsx",
  );
  const dashboardReadme = read("examples/dashboard/README.md");
  const cli = read("dx-www/src/cli/mod.rs");
  const launchRuntime = read("tools/launch/runtime-template/assets/launch-runtime.ts");
  const launchRuntimePage = read("tools/launch/runtime-template/pages/index.html");

  assert.match(docs, /^# Authentication/m);
  assert.match(docs, /official DX package name: `Authentication`/);
  assert.match(docs, /upstream_package: `better-auth`/);
  assert.match(docs, /source_mirror: `G:\/WWW\/inspirations\/better-auth`/);
  assert.match(docs, /`betterAuth`/);
  assert.match(docs, /`createAuthClient`/);
  assert.match(docs, /`toNextJsHandler`/);
  assert.match(docs, /`nextCookies`/);

  assert.equal(receipt.package_name, "Authentication");
  assert.equal(receipt.official_package_name, "Authentication");
  assert.equal(receipt.upstream_package, "better-auth");
  assert.equal(receipt.source_mirror, "G:/WWW/inspirations/better-auth");
  assert.ok(receipt.source_files.includes("docs/packages/authentication.md"));

  assert.match(catalog, /officialName: "Authentication"/);
  assert.match(
    catalog,
    /aliases: \["authentication", "better-auth", "auth\/betterauth", "auth\/better-auth-next"\]/,
  );
  assert.match(catalog, /upstreamPackage: "better-auth"/);
  assert.match(catalog, /docsPath: "docs\/packages\/authentication\.md"/);
  assert.match(catalog, /name: "Authentication"/);
  assert.match(catalog, /command: "dx add authentication --write"/);
  assert.doesNotMatch(catalog, /command: "dx add better-auth --write"/);
  assert.match(docs, /Use `dx add authentication --write`/);

  assert.match(registry, /officialName: "Authentication"/);
  assert.match(registry, /docsPath: "docs\/packages\/authentication\.md"/);
  assert.match(
    registry,
    /"authentication" \| "better-auth" \| "auth\/betterauth" \| "auth\/better-auth-next" => \{\s+"auth\/better-auth"\s+\}/,
  );
  assert.match(
    registry,
    /"authentication"\.to_string\(\),\s+"better-auth"\.to_string\(\),\s+"auth\/betterauth"\.to_string\(\),\s+"auth\/better-auth-next"\.to_string\(\),/,
  );
  assert.match(registry, /dxAdd: "dx add authentication --write"/);
  assert.doesNotMatch(registry, /dxAdd: "dx add auth\/better-auth --write"/);
  assert.match(
    registry,
    /Source-owned Authentication launch slice based on upstream better-auth/,
  );

  assert.match(dashboardPackage, /officialName: 'Authentication'/);
  assert.match(dashboardPackage, /upstreamPackage: 'better-auth'/);
  assert.match(dashboardPackage, /docsPath: 'docs\/packages\/authentication\.md'/);
  assert.match(dashboardWorkflow, /<h2>Authentication account workflow<\/h2>/);
  assert.doesNotMatch(dashboardWorkflow, /<h2>Better Auth account workflow<\/h2>/);
  assert.match(dashboardReadme, /### Authentication account workflow/);
  assert.doesNotMatch(dashboardReadme, /### Better Auth account workflow/);

  assert.match(cli, /"official_name": "Authentication"/);
  assert.match(cli, /"upstream_package": "better-auth"/);
  assert.match(cli, /"docs_path": "docs\/packages\/authentication\.md"/);
  assert.match(
    cli,
    /Authentication source surfaces use upstream better-auth APIs/,
  );

  assert.match(launchRuntime, /displayName: "Authentication"/);
  assert.match(launchRuntime, /command: "dx add authentication --write"/);
  assert.doesNotMatch(launchRuntime, /command: "dx add better-auth --write"/);
  assert.match(launchRuntimePage, /data-dx-query-package-name="Authentication"/);
  assert.match(
    launchRuntimePage,
    /data-dx-query-package-command="dx add authentication --write"/,
  );
  assert.match(launchRuntimePage, /<strong>Authentication<\/strong>/);
  assert.doesNotMatch(
    launchRuntimePage,
    /data-dx-query-package-command="dx add better-auth --write"/,
  );
});
