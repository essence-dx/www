const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sourceMirror = "G:/WWW/inspirations/vercel-ai";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readMirror(relativePath) {
  return fs.readFileSync(path.join(sourceMirror, relativePath), "utf8");
}

function escaped(marker) {
  return new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
}

test("AI SDK package-lane row exposes DX check-panel style visibility", () => {
  const upstreamPackage = JSON.parse(readMirror("packages/ai/package.json"));
  const streamText = readMirror("packages/ai/src/generate-text/stream-text.ts");
  const defaultChatTransport = readMirror(
    "packages/ai/src/ui/default-chat-transport.ts",
  );
  const providerRegistry = readMirror(
    "packages/ai/src/registry/provider-registry.ts",
  );
  const launchRuntime = read("tools/launch/runtime-template/pages/index.html");
  const reader = read("core/src/ecosystem/dx_check_receipt.rs");
  const packageDoc = read("docs/packages/ai-vercel-ai.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.equal(upstreamPackage.name, "ai");
  assert.equal(upstreamPackage.version, "7.0.0-canary.146");
  assert.match(streamText, /export function streamText</);
  assert.match(defaultChatTransport, /export class DefaultChatTransport/);
  assert.match(providerRegistry, /export function createProviderRegistry/);

  for (const marker of [
    'AI_SDK_PACKAGE_ID: &str = "ai/vercel-ai"',
    'AI_SDK_OFFICIAL_NAME: &str = "AI SDK"',
    'AI_SDK_UPSTREAM_PACKAGE: &str = "ai"',
    'AI_SDK_UPSTREAM_VERSION: &str = "7.0.0-canary.146"',
    'AI_SDK_SOURCE_MIRROR: &str = "G:/WWW/inspirations/vercel-ai"',
    'AI_SDK_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json"',
    "AI_SDK_PACKAGE_RECEIPT_PATH: &str =",
    "AI_SDK_METRICS: [&str; 13]",
    "rows.extend(ai_sdk_package_lane_row(root, package_status));",
    "fn ai_sdk_package_lane_row(",
    "fn ai_sdk_missing_receipt_row(next_action: &str)",
    "fn ai_sdk_metric_rows(",
    "fn ai_sdk_status_vocabulary(",
    "fn ai_sdk_next_action(",
    "fn dx_check_latest_panel_exposes_ai_sdk_package_lane_hash_refresh_row()",
    "fn write_ai_sdk_package_status(",
    "stale_helper_ai_sdk",
    "dx_style_compatibility_missing: u64",
    "ai_sdk_hash_manifest_present",
    "ai_sdk_hash_mismatch",
    "ai_sdk_receipt_hash_refresh_current",
    "ai_sdk_receipt_hash_refresh_stale",
    "ai_sdk_receipt_hash_refresh_missing",
    "ai_sdk_dx_style_compatibility_present",
    "ai_sdk_dx_style_compatibility_missing",
    "let receipt_hash_refresh = package_lane_hash_refresh(package);",
    "let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);",
    "|| refresh_stale > 0",
    "|| refresh_missing > 0",
    "let checkable_package = ai_sdk_checkable_hash_manifest(package);",
    "count_sha256_file_hash_mismatches(root, &checkable_package)",
    'helper_stale_metric_value("ai_sdk_hash_mismatch")',
    "ai-sdk-receipt-hashes.ts --write",
    "fn ai_sdk_checkable_hash_manifest(",
    'key.starts_with("upstream:")',
    'key.starts_with("core/")',
    'key.starts_with("docs/")',
  ]) {
    assert.match(reader, escaped(marker), `missing check-panel marker ${marker}`);
  }

  for (const marker of [
    'data-dx-check-package-lane-template="ai/vercel-ai"',
    'data-dx-check-package-lane-row="ai/vercel-ai"',
    'data-dx-check-package-lane-name="AI SDK"',
    'data-dx-check-package-lane-status="missing"',
    'data-dx-check-package-lane-receipt-status="missing-receipt"',
    'data-dx-check-package-lane-upstream-package="ai"',
    'data-dx-check-package-lane-source-mirror="G:/WWW/inspirations/vercel-ai"',
    'data-dx-check-package-lane-receipt-path="examples/template/.dx/forge/receipts/2026-05-22-ai-vercel-ai-launch-assistant.json"',
    'data-dx-check-package-lane-dx-style-status="present"',
    'data-dx-style-surface="ai-sdk"',
    'data-dx-token-scope="ai/vercel-ai"',
  ]) {
    assert.match(
      launchRuntime,
      escaped(marker),
      `missing static launch package-lane marker ${marker}`,
    );
  }

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /DX Studio\/check-panel AI SDK package row/);
    assert.match(source, /static \/launch AI SDK package-lane marker/);
    assert.match(source, /ai_sdk_hash_manifest_present/);
    assert.match(source, /ai_sdk_hash_mismatch/);
    assert.match(source, /ai_sdk_receipt_hash_refresh_current/);
    assert.match(source, /ai_sdk_receipt_hash_refresh_stale/);
    assert.match(source, /ai_sdk_receipt_hash_refresh_missing/);
    assert.match(source, /ai_sdk_dx_style_compatibility_present/);
    assert.match(source, /ai_sdk_dx_style_compatibility_missing/);
    assert.match(source, /without claiming live model streaming\s+or browser proof/);
  }
});

test("AI SDK stale-helper fixture is published through Studio source-guard runbooks", () => {
  const runbookFixturePath = "docs/packages/ai-sdk.source-guard-runbook.json";
  const guardId = "ai-sdk-check-panel-helper-freshness";
  const command =
    "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_ai_sdk_package_lane_hash_refresh_row --lib";

  const fixture = JSON.parse(read(runbookFixturePath));
  const fixtureText = [
    JSON.stringify(fixture),
    ...(fixture.zed_dx_studio_markers ?? []),
    ...(fixture.runtime_limitations ?? []),
    ...(fixture.upstream_public_apis ?? []),
    ...(fixture.selected_surfaces ?? []),
  ].join("\n");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const frameworkStructure = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const packageDoc = read("docs/packages/ai-vercel-ai.md");

  assert.equal(
    fixture.schema,
    "dx.forge.package.source_guard_runbook_fixture",
  );
  assert.equal(fixture.route, "/");
  assert.equal(fixture.package.official_package_name, "AI SDK");
  assert.equal(fixture.package.package_id, "ai/vercel-ai");
  assert.equal(fixture.package.upstream_package, "ai");
  assert.equal(fixture.package.upstream_version, "7.0.0-canary.146");
  assert.deepEqual(fixture.package.source_mirrors, [sourceMirror]);
  assert.equal(fixture.guard.id, guardId);
  assert.equal(fixture.guard.guard_file, "core/src/ecosystem/dx_check_receipt.rs");
  assert.equal(fixture.guard.command, command);
  assert.equal(fixture.guard.fixture_path, runbookFixturePath);
  assert.equal(fixture.honesty_label, "SOURCE-ONLY");
  assert.equal(fixture.runtime_proof, false);
  assert.deepEqual(fixture.runbook.fixture_paths, [
    {
      source_guard_id: guardId,
      package_id: "ai/vercel-ai",
      fixture_path: runbookFixturePath,
      schema: "dx.forge.package.source_guard_runbook_fixture",
    },
  ]);
  assert.equal(fixture.runbook.contract.id, guardId);
  assert.equal(fixture.runbook.contract.fixture_path, runbookFixturePath);
  assert.equal(fixture.runbook.command.command, command);
  assert.equal(fixture.runbook.command.fixture_path, runbookFixturePath);
  assert.equal(fixture.receipt.source_guard_runbook_fixture, runbookFixturePath);
  assert.equal(fixture.receipt.tracked_by_receipt_hash_helper, true);
  assert.equal(fixture.receipt.tracked_file_count, 7);
  assert.equal(fixture.preview_manifest.generated_file, "public/preview-.dx/build-cache/manifest.json");
  assert.equal(
    fixture.preview_manifest.materializer,
    "tools/launch/materialize-www-template.ts",
  );
  assert.equal(fixture.preview_manifest.root_field, "sourceGuardRunbookFixtures");
  assert.equal(
    fixture.preview_manifest.route_field,
    "routes[].sourceGuardRunbookFixtures",
  );
  assert.equal(fixture.preview_manifest.fixture, runbookFixturePath);
  assert.equal(fixture.preview_manifest.tracked_by_receipt_hash_helper, true);
  assert.equal(fixture.preview_manifest.runtime_proof, false);

  for (const marker of [
    "launch-assistant-dashboard-workflow",
    "receipt-hash-refresh",
    "check-panel-helper-freshness",
    "streamText",
    "convertToModelMessages",
    "DefaultChatTransport",
    "tool",
    "gateway",
    "createGateway",
    "createProviderRegistry",
    "ai_sdk_receipt_hash_refresh_current",
    "ai_sdk_receipt_hash_refresh_stale",
    "ai_sdk_receipt_hash_refresh_missing",
    "ai_sdk_hash_mismatch",
    "ai_sdk_dx_style_compatibility_present",
    'data-dx-check-package-lane-row="ai/vercel-ai"',
    'data-dx-style-surface="ai-sdk"',
    'data-dx-token-scope="ai/vercel-ai"',
    "ai-sdk:receipt-hash-refresh",
    "without claiming live model streaming proof",
  ]) {
    assert.match(
      fixtureText,
      escaped(marker),
      `missing runbook fixture marker ${marker}`,
    );
  }

  for (const marker of [guardId, runbookFixturePath, command]) {
    assert.match(
      studioManifest,
      escaped(marker),
      `missing Studio source guard marker ${marker}`,
    );
    assert.match(
      frameworkStructure,
      escaped(marker),
      `missing framework source guard marker ${marker}`,
    );
    assert.match(
      packageDoc,
      escaped(marker),
      `missing package doc source guard marker ${marker}`,
    );
  }

  assert.match(
    studioManifest,
    /fn source_guard_fixture_paths_for_route\(route: &str\)[\s\S]*"source_guard_id": "ai-sdk-check-panel-helper-freshness"[\s\S]*"fixture_path": "docs\/packages\/ai-sdk\.source-guard-runbook\.json"/,
    "Studio source_guard_runbook_index fixture_paths must link the AI SDK runbook fixture",
  );
  assert.match(
    studioManifest,
    /fn source_guard_ids_for_route\(route: &str\)[\s\S]*"ai-sdk-check-panel-helper-freshness"/,
    "Studio source guard ids must include the AI SDK helper-freshness guard on /launch",
  );
});

test("AI SDK runbook fixture is exposed from generated preview manifest", () => {
  const runbookFixturePath = "docs/packages/ai-sdk.source-guard-runbook.json";
  const guardId = "ai-sdk-check-panel-helper-freshness";
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-ai-sdk-preview-manifest-"));
  const materializer = path.join(
    root,
    "tools",
    "launch",
    "materialize-www-template.ts",
  );

  const result = JSON.parse(
    execFileSync(process.execPath, [materializer, dir], {
      cwd: root,
      encoding: "utf8",
    }),
  );
  const manifest = JSON.parse(
    fs.readFileSync(path.join(dir, "public", "preview-.dx/build-cache/manifest.json"), "utf8"),
  );
  const launch = fs.readFileSync(path.join(dir, "pages", "index.html"), "utf8");

  assert.equal(result.ok, true);
  assert.equal(result.noNodeModules, true);
  assert.ok(
    Array.isArray(manifest.sourceGuardRunbookFixtures),
    "generated preview manifest must expose source-guard runbook fixtures",
  );
  const aiSdkFixture = manifest.sourceGuardRunbookFixtures.find(
    (fixture) => fixture.packageId === "ai/vercel-ai",
  );
  assert.ok(
    aiSdkFixture,
    "generated preview manifest must expose the AI SDK runbook fixture",
  );
  assert.equal(aiSdkFixture.officialPackageName, "AI SDK");
  assert.equal(aiSdkFixture.upstreamPackage, "ai");
  assert.equal(aiSdkFixture.upstreamVersion, "7.0.0-canary.146");
  assert.equal(aiSdkFixture.sourceMirror, "G:/WWW/inspirations/vercel-ai");
  assert.equal(aiSdkFixture.fixture, runbookFixturePath);
  assert.equal(aiSdkFixture.guardId, guardId);
  assert.equal(aiSdkFixture.schema, "dx.forge.package.source_guard_runbook_fixture");
  assert.equal(aiSdkFixture.route, "/");
  assert.equal(aiSdkFixture.honestyLabel, "SOURCE-ONLY");
  assert.equal(aiSdkFixture.runtimeProof, false);
  assert.equal(aiSdkFixture.zedVisibility, "ai-sdk:receipt-hash-refresh");

  const homeRoute = manifest.routes.find((entry) => entry.route === "/");
  assert.ok(homeRoute, "expected generated / route metadata");
  assert.ok(
    homeRoute.forgePackages.includes("ai/vercel-ai"),
    "generated / route package scope must include AI SDK",
  );

  const launchPackageRoute = manifest.routes.find(
    (entry) => entry.route === "/",
  );
  assert.ok(launchPackageRoute, "expected generated / route metadata");
  assert.ok(
    launchPackageRoute.forgePackages.includes("ai/vercel-ai"),
    "generated /launch route package scope must include AI SDK",
  );

  const launchRoute = manifest.routes.find((entry) => entry.route === "/");
  assert.ok(
    launchRoute.sourceGuardRunbookFixtures.includes(runbookFixturePath),
    "generated /launch route must link the AI SDK source-guard runbook fixture",
  );
  assert.match(launch, /data-dx-check-package-lane-template="ai\/vercel-ai"/);
  assert.match(launch, /data-dx-check-package-lane-row="ai\/vercel-ai"/);
  assert.match(launch, /data-dx-check-package-lane-name="AI SDK"/);
  assert.match(launch, /data-dx-style-surface="ai-sdk"/);
  assert.match(launch, /data-dx-token-scope="ai\/vercel-ai"/);

  const checkPanel = manifest.editContract.editableSurfaces.find(
    (surface) => surface.id === "launch-runtime-dx-check-panel",
  );
  assert.ok(checkPanel, "expected generated dx-check panel edit surface");
  assert.equal(checkPanel.sourceFile, "pages/index.html");
  assert.ok(
    checkPanel.packageIds.includes("ai/vercel-ai"),
    "generated dx-check panel package scope must include AI SDK",
  );
});
