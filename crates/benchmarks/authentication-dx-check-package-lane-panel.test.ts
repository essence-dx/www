const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function readRepoFile(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

function readRepoJson(relativePath) {
  return JSON.parse(readRepoFile(relativePath));
}

function escaped(marker) {
  return new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
}

test("Authentication is a first-class dx-check panel package-lane row", () => {
  const receiptReader = readRepoFile("core/src/ecosystem/dx_check_receipt.rs");

  [
    'const AUTHENTICATION_PACKAGE_ID: &str = "auth/better-auth";',
    'const AUTHENTICATION_OFFICIAL_NAME: &str = "Authentication";',
    'const AUTHENTICATION_UPSTREAM_PACKAGE: &str = "better-auth";',
    'const AUTHENTICATION_UPSTREAM_VERSION: &str = "1.6.11";',
    'const AUTHENTICATION_SOURCE_MIRROR: &str = "G:/WWW/inspirations/better-auth";',
    'const AUTHENTICATION_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";',
    'const AUTHENTICATION_PACKAGE_RECEIPT_PATH: &str = ".dx/forge/receipts/auth-better-auth.json";',
    "const AUTHENTICATION_METRICS: [&str; 13] = [",
    "rows.extend(authentication_package_lane_row(root, package_status));",
    "fn authentication_package_lane_row(",
    "package_status: Option<&ForgePackageStatusReadModel>,",
    "fn authentication_metric_rows(",
    "fn authentication_next_action(",
    "authentication_hash_manifest_present",
    "authentication_hash_mismatch",
    "authentication_receipt_hash_refresh_current",
    "authentication_receipt_hash_refresh_stale",
    "authentication_receipt_hash_refresh_missing",
    "let receipt_hash_refresh = package_lane_hash_refresh(package);",
    "let (refresh_current, refresh_stale, refresh_missing) =",
    "receipt_hash_refresh: receipt_hash_refresh.clone(),",
    "authentication_next_action(\n            status,",
    "refresh_missing,\n            dx_style_compatibility_missing,",
    "authentication_dx_style_compatibility_present",
    "authentication_dx_style_compatibility_missing",
  ].forEach((fragment) => {
    assert.ok(
      receiptReader.includes(fragment),
      `dx_check_receipt.rs should include ${fragment}`,
    );
  });

  assert.ok(
    receiptReader.indexOf("rows.extend(authentication_package_lane_row(root, package_status));") <
      receiptReader.indexOf("rows.extend(state_management_package_lane_row(root, package_status));"),
    "Authentication should be listed before later package lanes in package_lane_rows",
  );
});

test("Authentication docs describe the check-panel row contract honestly", () => {
  const docs = readRepoFile("docs/packages/authentication.md");

  [
    "## DX Studio / Check-Panel Row",
    "`view_model.package_lane_rows`",
    "`auth/better-auth`",
    "`authentication_hash_manifest_present`",
    "`authentication_receipt_hash_refresh_current`",
    "`authentication_receipt_hash_refresh_stale`",
    "`authentication_receipt_hash_refresh_missing`",
    "`receipt_hash_refresh`",
    "`authentication_dx_style_compatibility_present`",
    "does not claim live OAuth, deployed cookies, or hosted session runtime proof",
  ].forEach((fragment) => {
    assert.ok(
      docs.includes(fragment),
      `docs/packages/authentication.md should include ${fragment}`,
    );
  });
});

test("Authentication publishes a package-owned source-guard runbook fixture", () => {
  const fixturePath = "docs/packages/authentication.source-guard-runbook.json";
  const fixtureAbsolutePath = path.join(repoRoot, fixturePath);

  assert.ok(
    fs.existsSync(fixtureAbsolutePath),
    "Authentication should publish a package-owned source-guard runbook JSON fixture",
  );

  const fixture = readRepoJson(fixturePath);
  const docs = readRepoFile("docs/packages/authentication.md");
  const frameworkDocs = readRepoFile("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const studioManifest = readRepoFile("dx-www/src/cli/studio_manifest.rs");
  const materializer = readRepoFile("tools/launch/materialize-www-template.ts");
  const authenticationStudioGuardStart = studioManifest.indexOf(
    '"authentication-package-lane-panel"',
  );
  const authenticationStudioGuardEnd = studioManifest.indexOf(
    "studio_source_guard",
    authenticationStudioGuardStart + 1,
  );
  const authenticationStudioGuard = studioManifest.slice(
    authenticationStudioGuardStart,
    authenticationStudioGuardEnd === -1
      ? undefined
      : authenticationStudioGuardEnd,
  );

  assert.equal(
    fixture.schema,
    "dx.forge.package.source_guard_runbook_fixture",
  );
  assert.equal(fixture.route, "/");
  assert.equal(fixture.package.official_package_name, "Authentication");
  assert.equal(fixture.package.package_id, "auth/better-auth");
  assert.equal(fixture.package.upstream_package, "better-auth");
  assert.equal(fixture.package.upstream_version, "1.6.11");
  assert.equal(fixture.package.source_mirror, "G:/WWW/inspirations/better-auth");
  assert.equal(fixture.guard.id, "authentication-package-lane-panel");
  assert.equal(
    fixture.guard.command,
    "dx run --test .\\benchmarks\\authentication-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(fixture.guard.execution_policy, "source-only");
  assert.equal(fixture.guard.starts_server, false);
  assert.equal(fixture.guard.runs_package_install, false);
  assert.equal(fixture.guard.runs_full_build, false);
  assert.equal(fixture.receipt.hash_helper, "examples/template/authentication-receipt-hashes.ts");
  assert.equal(
    fixture.receipt.hash_helper_json_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/authentication-receipt-hashes.ts --check --json",
  );
  assert.equal(fixture.receipt.zed_visibility, "authentication:receipt-hash-refresh");
  assert.equal(fixture.receipt.source_guard_runbook_fixture, fixturePath);
  for (const marker of [
    "data-dx-check-package-lane-hash-refresh-current-file-list",
    "data-dx-check-package-lane-hash-refresh-stale-file-list",
    "data-dx-check-package-lane-hash-refresh-missing-file-list",
    "data-dx-check-package-lane-hash-refresh-stale-mirror-file-list",
    "data-dx-check-package-lane-hash-refresh-missing-mirror-file-list",
  ]) {
    assert.ok(
      fixture.zed_dx_studio_markers.includes(marker),
      `Authentication runbook fixture should publish ${marker}`,
    );
  }
  assert.notEqual(
    authenticationStudioGuardStart,
    -1,
    "DX Studio manifest should expose the Authentication package-lane guard",
  );
  for (const fragment of [
    "data-dx-check-package-lane-hash-refresh-current-file-list",
    "data-dx-check-package-lane-hash-refresh-stale-file-list",
    "data-dx-check-package-lane-hash-refresh-missing-file-list",
    "data-dx-check-package-lane-hash-refresh-stale-mirror-file-list",
    "data-dx-check-package-lane-hash-refresh-missing-mirror-file-list",
    "authentication_receipt_hash_refresh_current",
    "authentication_receipt_hash_refresh_stale",
    "authentication_receipt_hash_refresh_missing",
  ]) {
    assert.ok(
      authenticationStudioGuard.includes(fragment),
      `Authentication Studio source guard should publish ${fragment}`,
    );
  }
  assert.deepEqual(fixture.preview_manifest, {
    generated_file: "public/preview-.dx/build-cache/manifest.json",
    materializer: "tools/launch/materialize-www-template.ts",
    field: "sourceGuardRunbookFixtures",
    route_field: "routes[].sourceGuardRunbookFixtures",
    fixture_path: fixturePath,
    route: "/",
    guard_id: "authentication-package-lane-panel",
    runtime_proof: false,
    zed_visibility: "authentication:receipt-hash-refresh",
  });
  assert.equal(fixture.honesty_label, "ADAPTER-BOUNDARY");
  assert.equal(fixture.runtime_proof, false);
  assert.ok(
    fixture.runtime_limitations.some((limitation) =>
      limitation.includes("does not claim live OAuth"),
    ),
  );

  [
    fixturePath,
    "authentication-package-lane-panel",
    "authentication:receipt-hash-refresh",
  ].forEach((fragment) => {
    assert.ok(
      docs.includes(fragment),
      `Authentication docs should include ${fragment}`,
    );
    assert.ok(
      frameworkDocs.includes(fragment),
      `DX framework docs should include ${fragment}`,
    );
    assert.ok(
      studioManifest.includes(fragment),
      `DX Studio manifest should include ${fragment}`,
    );
  });

  [
    "AUTHENTICATION_SOURCE_GUARD_RUNBOOK_FIXTURE",
    "sourceGuardRunbookFixtures",
    fixturePath,
  ].forEach((fragment) => {
    assert.ok(
      materializer.includes(fragment),
      `materializer should include ${fragment}`,
    );
  });

  const command =
    "dx run --test .\\benchmarks\\authentication-dx-check-package-lane-panel.test.ts";
  assert.ok(
    docs.includes(command),
    `Authentication docs should include ${command}`,
  );
  assert.ok(
    frameworkDocs.includes(command),
    `DX framework docs should include ${command}`,
  );
  assert.ok(
    studioManifest.includes(
      "dx run --test .\\\\benchmarks\\\\authentication-dx-check-package-lane-panel.test.ts",
    ),
    `DX Studio manifest should include ${command}`,
  );
});

test("Authentication has a targeted helper-stale-only Rust panel fixture", () => {
  const receiptReader = readRepoFile("core/src/ecosystem/dx_check_receipt.rs");
  const docs = readRepoFile("docs/packages/authentication.md");

  [
    "fn dx_check_latest_panel_exposes_authentication_package_lane_hash_refresh_row()",
    'find(|row| row["package_id"] == AUTHENTICATION_PACKAGE_ID)',
    '"authentication:receipt-hash-refresh"',
    "authentication_receipt_hash_refresh_current",
    "authentication_receipt_hash_refresh_stale",
    "authentication_receipt_hash_refresh_missing",
    "authentication_hash_mismatch",
    "stale_helper_authentication[\"package_lane_visibility\"][0][\"receipt_hash_refresh\"][\"stale_file_count\"]",
    'assert_eq!(helper_stale_authentication["status"], "stale");',
    'helper_stale_metric_value("authentication_hash_mismatch")',
  ].forEach((fragment) => {
    assert.ok(
      receiptReader.includes(fragment),
      `dx_check_receipt.rs should include ${fragment}`,
    );
  });

  assert.ok(
    docs.includes(
      "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_authentication_package_lane_hash_refresh_row --lib",
    ),
    "Authentication docs should publish the targeted helper-stale Rust fixture command",
  );
});

test("Authentication static launch fixture exposes receipt-less package-lane markers", () => {
  const launchPage = readRepoFile(
    "tools/launch/runtime-template/pages/index.html",
  );
  const launchShell = readRepoFile("examples/template/template-shell.tsx");
  const normalizedTemplateShell = launchShell.replace(/\r\n/g, "\n");

  [
    'data-dx-check-package-lane-template="auth/better-auth"',
    'data-dx-check-package-lane-row="auth/better-auth"',
    'data-dx-check-package-lane-name="Authentication"',
    'data-dx-check-package-lane-status="missing"',
    'data-dx-check-package-lane-receipt-status="missing-receipt"',
    'data-dx-check-package-lane-upstream-package="better-auth"',
    'data-dx-check-package-lane-source-mirror="G:/WWW/inspirations/better-auth"',
    'data-dx-check-package-lane-receipt-path=".dx/forge/receipts/auth-better-auth.json"',
    'data-dx-check-package-lane-dx-style-status="present"',
    'data-dx-check-package-lane-hash-refresh-status="current"',
    'data-dx-check-package-lane-hash-refresh-helper="examples/template/authentication-receipt-hashes.ts"',
    'data-dx-check-package-lane-hash-refresh-json-command="node tools/launch/run-template-receipt-helper.js examples/template/authentication-receipt-hashes.ts --check --json"',
    'data-dx-check-package-lane-hash-refresh-zed="authentication:receipt-hash-refresh"',
    'data-dx-check-package-lane-hash-refresh-tracked-files="6"',
    'data-dx-check-package-lane-hash-refresh-stale-files="0"',
    'data-dx-check-package-lane-hash-refresh-missing-files="0"',
    'data-dx-check-package-lane-hash-refresh-current-file-list="examples/template/template-shell.tsx|examples/template/auth-session-status.tsx|examples/dashboard/src/lib/forge/auth/better-auth/dashboard.ts|examples/dashboard/src/components/BetterAuthAccountWorkflow.tsx|docs/packages/authentication.source-guard-runbook.json|tools/launch/materialize-www-template.ts"',
    'data-dx-check-package-lane-hash-refresh-stale-file-list=""',
    'data-dx-check-package-lane-hash-refresh-missing-file-list=""',
    'data-dx-check-package-lane-hash-refresh-stale-mirror-file-list=""',
    'data-dx-check-package-lane-hash-refresh-missing-mirror-file-list=""',
    'data-dx-check-package-lane-hash-refresh-current-metric="authentication_receipt_hash_refresh_current"',
    'data-dx-check-package-lane-hash-refresh-stale-metric="authentication_receipt_hash_refresh_stale"',
    'data-dx-check-package-lane-hash-refresh-missing-metric="authentication_receipt_hash_refresh_missing"',
    'data-dx-style-surface="authentication-account-workflow"',
    'data-dx-token-scope="auth/better-auth"',
  ].forEach((fragment) => {
    assert.ok(
      launchPage.includes(fragment),
      `tools/launch/runtime-template/pages/index.html should include ${fragment}`,
    );
  });

  [
    'package_id: "auth/better-auth"',
    'official_package_name: "Authentication"',
    'upstream_package: "better-auth"',
    'upstream_version: "1.6.11"',
    'source_mirror: "G:/WWW/inspirations/better-auth"',
    'package_receipt_path: ".dx/forge/receipts/auth-better-auth.json"',
    'hash_refresh_status: "current"',
    'hash_refresh_helper:\n      "examples/template/authentication-receipt-hashes.ts"',
    'hash_refresh_json_command:\n      "node tools/launch/run-template-receipt-helper.js examples/template/authentication-receipt-hashes.ts --check --json"',
    'hash_refresh_zed: "authentication:receipt-hash-refresh"',
    "hash_refresh_tracked_files: 6",
    "hash_refresh_stale_files: 0",
    "hash_refresh_missing_files: 0",
    'hash_refresh_current_file_list:\n      "examples/template/template-shell.tsx|examples/template/auth-session-status.tsx|examples/dashboard/src/lib/forge/auth/better-auth/dashboard.ts|examples/dashboard/src/components/BetterAuthAccountWorkflow.tsx|docs/packages/authentication.source-guard-runbook.json|tools/launch/materialize-www-template.ts"',
    'hash_refresh_stale_file_list: ""',
    'hash_refresh_missing_file_list: ""',
    'hash_refresh_stale_mirror_file_list: ""',
    'hash_refresh_missing_mirror_file_list: ""',
    'hash_refresh_metric_current: "authentication_receipt_hash_refresh_current"',
    'hash_refresh_metric_stale: "authentication_receipt_hash_refresh_stale"',
    'hash_refresh_metric_missing: "authentication_receipt_hash_refresh_missing"',
    'dx_style_surface: "authentication-account-workflow"',
    'token_scope: "auth/better-auth"',
  ].forEach((fragment) => {
    assert.ok(
      normalizedTemplateShell.includes(fragment),
      `template-shell.tsx should include ${fragment}`,
    );
  });

  assert.ok(
    launchPage.indexOf('data-dx-check-package-lane-row="auth/better-auth"') <
      launchPage.indexOf('data-dx-component="better-auth-account-dashboard-workflow"'),
    "Authentication package-lane marker should be discoverable before the account workflow card",
  );
});

test("Authentication runbook fixture is exposed in generated preview-manifest metadata", () => {
  const fixturePath = "docs/packages/authentication.source-guard-runbook.json";
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-auth-preview-manifest-"));
  const materializer = path.join(
    repoRoot,
    "tools",
    "launch",
    "materialize-www-template.ts",
  );

  try {
    const result = JSON.parse(
      execFileSync(process.execPath, [materializer, dir], {
        cwd: repoRoot,
        encoding: "utf8",
      }),
    );
    const launch = fs.readFileSync(path.join(dir, "pages", "index.html"), "utf8");
    const manifest = JSON.parse(
      fs.readFileSync(path.join(dir, "public", "preview-.dx/build-cache/manifest.json"), "utf8"),
    );

    assert.equal(result.ok, true);
    assert.equal(result.noNodeModules, true);
    assert.ok(!fs.existsSync(path.join(dir, "node_modules")));

    for (const marker of [
      'data-dx-check-package-lane-template="auth/better-auth"',
      'data-dx-check-package-lane-row="auth/better-auth"',
      'data-dx-check-package-lane-name="Authentication"',
      'data-dx-style-surface="authentication-account-workflow"',
      'data-dx-token-scope="auth/better-auth"',
      'data-dx-package="auth/better-auth"',
      'data-dx-check-package-lane-hash-refresh-current-file-list="examples/template/template-shell.tsx|examples/template/auth-session-status.tsx|examples/dashboard/src/lib/forge/auth/better-auth/dashboard.ts|examples/dashboard/src/components/BetterAuthAccountWorkflow.tsx|docs/packages/authentication.source-guard-runbook.json|tools/launch/materialize-www-template.ts"',
      'data-dx-check-package-lane-hash-refresh-stale-file-list=""',
      'data-dx-check-package-lane-hash-refresh-missing-file-list=""',
    ]) {
      assert.match(launch, escaped(marker), `missing generated marker ${marker}`);
    }

    assert.ok(
      Array.isArray(manifest.sourceGuardRunbookFixtures),
      "generated preview manifest must expose source-guard runbook fixtures",
    );
    const authenticationRunbookFixture =
      manifest.sourceGuardRunbookFixtures.find(
        (fixture) => fixture.packageId === "auth/better-auth",
      );
    assert.ok(
      authenticationRunbookFixture,
      "generated preview manifest must expose the Authentication source-guard runbook fixture",
    );
    assert.equal(authenticationRunbookFixture.officialPackageName, "Authentication");
    assert.equal(authenticationRunbookFixture.upstreamPackage, "better-auth");
    assert.equal(authenticationRunbookFixture.upstreamVersion, "1.6.11");
    assert.equal(
      authenticationRunbookFixture.sourceMirror,
      "G:/WWW/inspirations/better-auth",
    );
    assert.equal(authenticationRunbookFixture.fixture, fixturePath);
    assert.equal(
      authenticationRunbookFixture.guardId,
      "authentication-package-lane-panel",
    );
    assert.equal(authenticationRunbookFixture.route, "/");
    assert.equal(authenticationRunbookFixture.honestyLabel, "ADAPTER-BOUNDARY");
    assert.equal(authenticationRunbookFixture.runtimeProof, false);
    assert.equal(
      authenticationRunbookFixture.zedVisibility,
      "authentication:receipt-hash-refresh",
    );

    const launchRoute = manifest.routes.find((entry) => entry.route === "/");
    assert.ok(launchRoute, "expected generated / route metadata");
    assert.ok(
      launchRoute.forgePackages.includes("auth/better-auth"),
      "generated / route package scope must include Authentication",
    );
    assert.ok(
      launchRoute.sourceGuardRunbookFixtures.includes(fixturePath),
      "generated / route must link the Authentication source-guard runbook fixture",
    );

    assert.ok(
      manifest.editContract.editableSurfaces.some(
        (surface) => surface.id === "launch-runtime-dx-check-panel",
      ),
      "expected generated dx-check panel edit surface",
    );
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});
