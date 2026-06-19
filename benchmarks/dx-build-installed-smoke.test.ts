const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.join(__dirname, "..");
const smokePath = path.join(repoRoot, "tools", "build", "dx-build-installed-smoke.ts");
const fixturePath = path.join(repoRoot, "tools", "build", "installed-smoke", "fixture.ts");

test("installed smoke fixture is a launch-shaped tiny app without node_modules", () => {
  const { createFixtureProject } = require(fixturePath);
  const project = createFixtureProject();

  for (const relative of [
    "app/page.tsx",
    "app/layout.tsx",
    "app/api/health/route.ts",
    "app/api/checkout/route.ts",
    "components/LaunchCard.tsx",
    "server/loaders.ts",
    "server/launch-copy.ts",
    "styles/app.css",
    "public/icons/mark.svg",
    "next.config.mjs",
    "package.json",
  ]) {
    assert.ok(fs.existsSync(path.join(project, relative)), `${relative} should exist`);
  }

  assert.equal(fs.existsSync(path.join(project, "node_modules")), false);
  assert.match(fs.readFileSync(path.join(project, "app/page.tsx"), "utf8"), /LaunchCard/);
  assert.match(fs.readFileSync(path.join(project, "app/page.tsx"), "utf8"), /loadLaunchMetrics/);
  assert.match(fs.readFileSync(path.join(project, "app/page.tsx"), "utf8"), /await loadLaunchMetrics\(\)/);
  assert.match(fs.readFileSync(path.join(project, "app/layout.tsx"), "utf8"), /\.\.\/styles\/app\.css/);
  assert.match(fs.readFileSync(path.join(project, "components/LaunchCard.tsx"), "utf8"), /launchCopy/);
  assert.match(fs.readFileSync(path.join(project, "app/api/health/route.ts"), "utf8"), /export function GET/);
  assert.match(fs.readFileSync(path.join(project, "app/api/checkout/route.ts"), "utf8"), /export function POST/);
});

test("installed dx build smoke receipt carries failed build command diagnostics", () => {
  const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-failure-receipt-"));
  const fakeBinary = path.join(tempRoot, "fake-dx-www-fails.ts");
  fs.writeFileSync(
    fakeBinary,
    `const args = process.argv.slice(2);
if (args[0] === "www" && args[1] === "build" && args[2] === "--help") {
  console.error("dx www build: Run the DX source-owned build engine");
  console.error("USAGE: dx www build --target android");
  console.error("OPTIONS: --target <target>");
  console.error("Uses the source-owned build engine and does not install node_modules.");
  process.exit(0);
}
if (args[0] === "build") {
  console.log("starting source-owned fixture build");
  console.error("DX_BUILD_FIXTURE_COMPILE_FAILED: missing app/page.tsx");
  process.exit(42);
}
process.exit(2);
`,
  );
  const receiptPath = path.join(tempRoot, "failure-receipt.json");

  const result = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      fakeBinary,
      "--runner",
      process.execPath,
      "--json",
      "--receipt",
      receiptPath,
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
    },
  );

  assert.equal(result.status, 1, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  assert.equal(report.passed, false);
  assert.equal(report.build.exitCode, 42);
  assert.equal(report.build.command.exitCode, 42);
  assert.equal(report.build.command.command, process.execPath);
  assert.deepEqual(report.build.command.args, [fakeBinary, "build"]);
  assert.deepEqual(report.build.command.dxArgs, ["build"]);
  assert.match(report.build.command.stdoutTail, /starting source-owned fixture build/);
  assert.match(report.build.command.stderrTail, /DX_BUILD_FIXTURE_COMPILE_FAILED: missing app\/page\.tsx/);
  assert.equal(report.help.command.exitCode, 0);
  assert.deepEqual(report.help.command.dxArgs, ["www", "build", "--help"]);
  assert.deepEqual(JSON.parse(fs.readFileSync(receiptPath, "utf8")), report);
});

test("installed dx build smoke human output includes failed build command diagnostics", () => {
  const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-human-failure-"));
  const fakeBinary = path.join(tempRoot, "fake-dx-www-human-fails.ts");
  fs.writeFileSync(
    fakeBinary,
    `const args = process.argv.slice(2);
if (args[0] === "www" && args[1] === "build" && args[2] === "--help") {
  console.error("dx www build: Run the DX source-owned build engine");
  console.error("USAGE: dx www build --target android");
  console.error("OPTIONS: --target <target>");
  console.error("Uses the source-owned build engine and does not install node_modules.");
  process.exit(0);
}
if (args[0] === "build") {
  console.log("starting source-owned fixture build");
  console.error("DX_BUILD_FIXTURE_COMPILE_FAILED: missing app/page.tsx");
  process.exit(42);
}
process.exit(2);
`,
  );

  const result = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      fakeBinary,
      "--runner",
      process.execPath,
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
    },
  );

  assert.equal(result.status, 1, result.stdout + result.stderr);
  assert.match(result.stdout, /DX build installed-binary smoke: failed/);
  assert.match(result.stdout, /Build command:/);
  assert.match(result.stdout, /fake-dx-www-human-fails\.ts build/);
  assert.match(result.stdout, /Build exit code: 42/);
  assert.match(result.stdout, /Build stdout tail:[\s\S]*starting source-owned fixture build/);
  assert.match(result.stdout, /Build stderr tail:[\s\S]*DX_BUILD_FIXTURE_COMPILE_FAILED: missing app\/page\.tsx/);
});

test("installed dx build smoke reports missing binary as a first-class failure", () => {
  const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-missing-binary-"));
  const missingBinary = path.join(tempRoot, "missing-dx-www.exe");

  const result = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      missingBinary,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
    },
  );

  assert.equal(result.status, 1, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  assert.equal(report.binaryIdentity.present, false);
  assert.equal(report.binaryIdentity.error, "ENOENT");
  assert.deepEqual(report.failures, [
    "dx build binary is missing or not executable",
    "dx www build --help exited non-zero",
    "dx build fixture exited non-zero",
  ]);
});

test("installed dx build smoke rejects stale binaries before executing dx build", () => {
  const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-stale-binary-"));
  const { createFixtureProject } = require(fixturePath);
  const projectRoot = createFixtureProject();
  const fakeBinary = path.join(tempRoot, "fake-stale-dx-www.ts");
  const markerPath = path.join(tempRoot, "stale-binary-executed.txt");
  fs.mkdirSync(path.join(projectRoot, ".dx", "build"), { recursive: true });
  fs.writeFileSync(
    path.join(projectRoot, ".dx", "build", "source-build-manifest.json"),
    JSON.stringify({
      schema: "dx.www.sourceBuildManifest",
      styles: [
        {
          path: "styles/app.css",
          output: ".dx/build/styles/app.css",
          hash: "stale-style-hash",
        },
      ],
      assets: [],
      node_modules_required: false,
    }, null, 2),
  );
  fs.writeFileSync(
    fakeBinary,
    `const fs = require("node:fs");
fs.writeFileSync(${JSON.stringify(markerPath)}, process.argv.slice(2).join(" "));
process.exit(0);
`,
  );
  const oldDate = new Date("2000-01-01T00:00:00.000Z");
  fs.utimesSync(fakeBinary, oldDate, oldDate);

  const result = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      fakeBinary,
      "--runner",
      process.execPath,
      "--project",
      projectRoot,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
    },
  );

  assert.equal(result.status, 1, result.stdout + result.stderr);
  assert.equal(fs.existsSync(markerPath), false, "stale binary should not be executed");
  const report = JSON.parse(result.stdout);
  assert.equal(report.binaryIdentity.present, true);
  assert.equal(report.binarySourceFreshness.checked, true);
  assert.equal(report.binarySourceFreshness.fresh, false);
  assert.equal(report.binarySourceFreshness.trackedSourceCount > 0, true);
  assert.equal(typeof report.binarySourceFreshness.newestSourcePath, "string");
  assert.equal(report.build.artifactTrust.trusted, false);
  assert.equal(report.build.artifactTrust.reason, "stale-binary");
  assert.equal(report.build.artifactTrust.staleArtifactRisk, true);
  assert.equal(report.build.sourceBuild.manifest.present, true);
  assert.equal(report.proof.cssAssetOutputProof.artifactTrust.trusted, false);
  assert.equal(report.proof.cssAssetOutputProof.styleOutput.ignored, true);
  assert.equal(report.proof.cssAssetOutputProof.publicAssetOutput.ignored, true);
  assert.equal(report.proof.nodeModulesProof.artifactTrust.trusted, false);
  assert.equal(report.proof.nodeModulesProof.ignored, true);
  assert.equal(report.proof.nodeModulesProof.ignoreReason, "stale-binary");
  assert.equal(report.outputProofSummary.nodeModulesProof.ignored, true);
  assert.ok(report.outputProofSummary.missingChecks.includes("build-artifacts-trusted"));
  assert.ok(report.outputProofSummary.missingChecks.includes("style-output-ignored"));
  assert.ok(report.outputProofSummary.missingChecks.includes("public-asset-output-ignored"));
  assert.ok(report.outputProofSummary.missingChecks.includes("node-modules-proof-ignored"));
  assert.deepEqual(report.failures, [
    "dx build binary is stale relative to source changes",
  ]);
  assert.equal(report.help.command.skipped, true);
  assert.equal(report.help.command.skipReason, "stale-binary");
  assert.equal(report.build.command.skipped, true);
  assert.equal(report.build.command.skipReason, "stale-binary");
  assert.match(report.proof.nextAction, /Build a fresh dx-www binary/);
});

test("installed dx build smoke freshness scans split Rust source modules", () => {
  const {
    DEFAULT_SOURCE_FRESHNESS_DIRS,
    inspectBinary,
    inspectBinarySourceFreshness,
  } = require(path.join(repoRoot, "tools", "build", "installed-smoke", "binary-provenance.ts"));
  const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-source-freshness-"));
  const fakeBinary = path.join(tempRoot, "target", "debug", "dx-www.exe");
  const trackedSource = path.join(tempRoot, "dx-www", "src", "cli", "mod.rs");
  const splitModule = path.join(tempRoot, "dx-www", "src", "cli", "dev_command.rs");
  const reactorModule = path.join(tempRoot, "reactor", "src", "lib.rs");
  const targetBuildScript = path.join(tempRoot, "target", "debug", "build.rs");
  const nodeModuleRustFile = path.join(tempRoot, "node_modules", "pkg", "index.rs");

  for (const filePath of [
    path.join(tempRoot, "Cargo.toml"),
    path.join(tempRoot, "Cargo.lock"),
    path.join(tempRoot, "dx-www", "Cargo.toml"),
    path.join(tempRoot, "dx-www", "src", "main.rs"),
    path.join(tempRoot, "dx-www", "src", "lib.rs"),
    trackedSource,
    splitModule,
    reactorModule,
    targetBuildScript,
    nodeModuleRustFile,
    fakeBinary,
  ]) {
    fs.mkdirSync(path.dirname(filePath), { recursive: true });
    fs.writeFileSync(filePath, "source");
  }

  const oldDate = new Date("2026-01-01T00:00:00.000Z");
  const binaryDate = new Date("2026-01-02T00:00:00.000Z");
  const newDate = new Date("2026-01-03T00:00:00.000Z");
  for (const filePath of [
    path.join(tempRoot, "Cargo.toml"),
    path.join(tempRoot, "Cargo.lock"),
    path.join(tempRoot, "dx-www", "Cargo.toml"),
    path.join(tempRoot, "dx-www", "src", "main.rs"),
    path.join(tempRoot, "dx-www", "src", "lib.rs"),
    trackedSource,
    reactorModule,
  ]) {
    fs.utimesSync(filePath, oldDate, oldDate);
  }
  fs.utimesSync(fakeBinary, binaryDate, binaryDate);
  fs.utimesSync(splitModule, newDate, newDate);
  fs.utimesSync(targetBuildScript, new Date("2026-01-04T00:00:00.000Z"), new Date("2026-01-04T00:00:00.000Z"));
  fs.utimesSync(nodeModuleRustFile, new Date("2026-01-05T00:00:00.000Z"), new Date("2026-01-05T00:00:00.000Z"));

  const freshness = inspectBinarySourceFreshness(inspectBinary(fakeBinary), {
    repoRoot: tempRoot,
  });

  assert.equal(freshness.checked, true);
  assert.equal(freshness.fresh, false);
  assert.equal(freshness.newestSourcePath, "dx-www/src/cli/dev_command.rs");
  assert.ok(freshness.trackedSourcePaths.includes("dx-www/src/cli/dev_command.rs"));
  assert.ok(freshness.trackedSourcePaths.includes("reactor/src/lib.rs"));
  assert.ok(DEFAULT_SOURCE_FRESHNESS_DIRS.includes("reactor/src"));
  assert.ok(DEFAULT_SOURCE_FRESHNESS_DIRS.includes("router/src"));
  assert.equal(DEFAULT_SOURCE_FRESHNESS_DIRS.includes("node_modules/pkg"), false);
  assert.equal(DEFAULT_SOURCE_FRESHNESS_DIRS.includes("target/debug"), false);
  assert.equal(freshness.trackedSourcePaths.includes("target/debug/build.rs"), false);
  assert.equal(freshness.trackedSourcePaths.includes("node_modules/pkg/index.rs"), false);
});

test("installed dx build smoke validates help, manifest summaries, readiness, Zed handoff, and receipt output", () => {
  assert.ok(fs.existsSync(smokePath), "installed dx build smoke harness is missing");

  const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-smoke-test-"));
  const fakeBinary = path.join(tempRoot, "fake-dx-www.ts");
  fs.writeFileSync(
    fakeBinary,
    `const fs = require("node:fs");
const path = require("node:path");
const args = process.argv.slice(2);
if (args[0] === "www" && args[1] === "build" && args[2] === "--help") {
  console.error("dx www build: Run the DX source-owned build engine");
  console.error("USAGE: dx www build --target android");
  console.error("OPTIONS: --target <target>");
  console.error("Uses the source-owned build engine and does not install node_modules.");
  process.exit(0);
}
if (args[0] !== "build") {
  console.error("unexpected command " + args.join(" "));
  process.exit(2);
}
fs.mkdirSync(path.join(process.cwd(), ".dx", "build"), { recursive: true });
fs.mkdirSync(path.join(process.cwd(), ".dx", "build", "app"), { recursive: true });
fs.mkdirSync(path.join(process.cwd(), ".dx", "build", "public", "icons"), { recursive: true });
fs.mkdirSync(path.join(process.cwd(), ".dx", "build", "source-routes", "root", "modules"), { recursive: true });
fs.mkdirSync(path.join(process.cwd(), ".dx", "build", "styles"), { recursive: true });
fs.mkdirSync(path.join(process.cwd(), ".dx", "receipts", "build"), { recursive: true });
fs.mkdirSync(path.join(process.cwd(), ".dx", "receipts", "graph"), { recursive: true });
fs.writeFileSync(path.join(process.cwd(), ".dx", "build", "app", "index.html"), "<main>DX tiny app</main>");
fs.writeFileSync(path.join(process.cwd(), ".dx", "build", "app", "index.dxpk"), "packet");
fs.writeFileSync(path.join(process.cwd(), ".dx", "build", "app", "page-graph.json"), "{}");
fs.writeFileSync(path.join(process.cwd(), ".dx", "build", "source-routes", "root", "index.html"), "<main>DX tiny app</main>");
fs.writeFileSync(path.join(process.cwd(), ".dx", "build", "source-routes", "root", "index.dxpk"), "packet");
fs.writeFileSync(path.join(process.cwd(), ".dx", "build", "source-routes", "root", "page-graph.json"), "{}");
fs.writeFileSync(path.join(process.cwd(), ".dx", "build", "source-routes", "root", "modules", "app-page-tsx-abc.mjs"), "export {};");
fs.writeFileSync(path.join(process.cwd(), ".dx", "build", "public", "icons", "mark-e261696593b3dbad.svg"), "<svg />");
fs.writeFileSync(path.join(process.cwd(), ".dx", "build", "styles", "app.css"), ".hero{display:grid}\\n/*# sourceMappingURL=app.css.map */\\n");
fs.writeFileSync(path.join(process.cwd(), ".dx", "build", "styles", "app.css.map"), JSON.stringify({
  version: 3,
  file: "app.css",
  sources: ["styles/app.css"],
  names: [],
  mappings: ""
}, null, 2));
fs.writeFileSync(path.join(process.cwd(), ".dx", "build", "app", "app-router-execution.json"), JSON.stringify({
  route: process.env.DX_BAD_APP_ROUTER_EXECUTION_ROUTE === "1" ? "/wrong" : "/",
  source_path: "app/page.tsx",
  node_modules_present: false,
  runtime_boundary: {
    source_owned: true,
    external_runtime_required: false,
    external_runtime_executed: false
  }
}, null, 2));
const serverDataContract = {
  route: process.env.DX_BAD_SERVER_DATA_ROUTE === "1" ? "/wrong" : "/",
  route_source_path: "app/page.tsx",
  status: "compiled",
  entry_count: 1,
  execution_model: "source-owned-safe-interpreter",
  node_modules_required: false,
  lifecycle_scripts_executed: false,
  source_owned_contract: true,
  external_runtime_required: false,
  external_runtime_executed: false,
  entries: [
    {
      binding: "metrics",
      export_name: "loadLaunchMetrics",
      source_path: "server/loaders.ts",
      execution_model: "source-owned-safe-interpreter",
      lifecycle_scripts_executed: false,
      value: { routeHandlers: 2 }
    }
  ]
};
fs.writeFileSync(path.join(process.cwd(), ".dx", "build", "app", "server-data.json"), JSON.stringify(serverDataContract, null, 2));
fs.writeFileSync(path.join(process.cwd(), ".dx", "build", "source-routes", "root", "server-data.json"), JSON.stringify(serverDataContract, null, 2));
fs.writeFileSync(path.join(process.cwd(), ".dx", "build", "server-contracts.json"), JSON.stringify([
  {
    kind: "route-handler",
    source_path: "app/api/health/route.ts",
    endpoint: "/api/health",
    lifecycle_scripts_executed: false
  },
  ...(process.env.DX_OMIT_CHECKOUT_ROUTE_HANDLER === "1" ? [] : [{
    kind: "route-handler",
    source_path: "app/api/checkout/route.ts",
    endpoint: "/api/checkout",
    lifecycle_scripts_executed: false
  }]),
  {
    kind: "loader",
    source_path: "server/loaders.ts",
    endpoint: null,
    lifecycle_scripts_executed: false
  }
], null, 2));
const nextFamiliarCompatibilityEvidence = {
  version: 1,
  source_framework: "nextjs-app-router",
  evidence_kind: "next-familiar-compatibility",
  evidence_mode: "next-familiar-source-output-readiness",
  next_familiar_inventory: {
    page_route_count: 1,
    route_handler_count: 2,
    client_component_count: 1,
    server_action_count: 0
  },
  dx_www_output: {
    app_routes_compiled: 1,
    server_contracts_compiled: process.env.DX_OMIT_CHECKOUT_ROUTE_HANDLER === "1" ? 2 : 3
  },
  compatibility_dimensions: {
    routes: { score: 100 },
    bytes: { score: 100 },
    hydration: { score: 100 },
    server_actions: { score: 100 },
    security: {
      node_modules_present: false,
      package_installs_executed: false,
      lifecycle_scripts_executed: false,
      score: 100
    }
  },
  score: 100,
  verdict: "passes-current-source-owned-next-familiar-compatibility-gate"
};
if (process.env.DX_OMIT_NEXT_FAMILIAR_COMPATIBILITY_EVIDENCE !== "1") {
  fs.writeFileSync(
    path.join(process.cwd(), ".dx", "build", "next-familiar-compatibility-evidence.json"),
    JSON.stringify(nextFamiliarCompatibilityEvidence, null, 2)
  );
}
if (process.env.DX_WRITE_OLD_NEXT_PARITY_EVIDENCE === "1") {
  fs.writeFileSync(
    path.join(process.cwd(), ".dx", "build", "next-runtime-parity-evidence.json"),
    JSON.stringify({ comparison_mode: "removed-runtime-parity" }, null, 2)
  );
}
fs.writeFileSync(path.join(process.cwd(), ".dx", "build", "deploy-adapter.json"), JSON.stringify({
  no_node_modules_required: true,
  ...(process.env.DX_OMIT_NEXT_FAMILIAR_COMPATIBILITY_EVIDENCE === "1" ? {} : {
    next_familiar_compatibility_evidence: {
      path: "next-familiar-compatibility-evidence.json",
      evidence_kind: "next-familiar-compatibility",
      evidence_mode: "next-familiar-source-output-readiness",
      score: 100,
      verdict: "passes-current-source-owned-next-familiar-compatibility-gate"
    }
  }),
  routes: [{ path: "/", server_data: "app/server-data.json" }],
  health_checks: [{ path: "/api/health", source_path: "app/api/health/route.ts", method: "GET" }],
  route_handlers: [
    {
      path: "/api/health",
      source_path: "app/api/health/route.ts",
      methods: ["GET"],
      safe_build_methods: ["GET"],
      skipped_build_methods: [],
      build_execution: "safe-requestless-receipt",
      receipt: "route-handler-receipts.json",
      node_modules_required: false,
      runtime_boundary: {
        source_owned: true,
        external_runtime_required: false,
        external_runtime_executed: false
      }
    },
    ...(process.env.DX_OMIT_CHECKOUT_ROUTE_HANDLER === "1" ? [] : [{
      path: "/api/checkout",
      source_path: "app/api/checkout/route.ts",
      methods: ["POST"],
      safe_build_methods: [],
      skipped_build_methods: ["POST"],
      build_execution: "skipped-build-execution",
      receipt: "route-handler-receipts.json",
      node_modules_required: false,
      runtime_boundary: {
        source_owned: true,
        external_runtime_required: false,
        external_runtime_executed: false
      }
    }])
  ]
}, null, 2));
const manifestServerDataRoutes = [{
  route: process.env.DX_BAD_MANIFEST_SERVER_DATA_ROUTE === "1" ? "/wrong" : "/",
  output: process.env.DX_BAD_MANIFEST_SERVER_DATA_OUTPUT === "1"
    ? ".dx/build/source-routes/root/missing-server-data.json"
    : ".dx/build/source-routes/root/server-data.json",
  route_source_path: "app/page.tsx",
  status: "compiled",
  entry_count: 1,
  execution_model: "source-owned-safe-interpreter",
  request: {
    mode: "static-route-contract-inputs",
    build_time_contract_inputs: true
  },
  node_modules_required: false,
  lifecycle_scripts_executed: false,
  source_owned_contract: true,
  external_runtime_required: false,
  external_runtime_executed: false
}];
fs.writeFileSync(path.join(process.cwd(), ".dx", "build", "manifest.json"), JSON.stringify({
  source_build_css_original_rules: 3,
  source_build_css_retained_rules: 2,
  source_build_css_pruned_rules: 1,
  source_build_css_minified_styles: 1,
  source_build_zed_handoff_emitted: true,
  server_data_entries_compiled: 1,
  ...(process.env.DX_OMIT_MANIFEST_SERVER_DATA_ROUTES === "1" ? {} : {
    server_data_routes_compiled: process.env.DX_BAD_MANIFEST_SERVER_DATA_COMPILED === "1"
      ? 0
      : manifestServerDataRoutes.length,
    server_data_routes: manifestServerDataRoutes,
    server_data_route_manifest: {
      source_build_routes: 1,
      manifest_routes: manifestServerDataRoutes.length,
      source_build_entries: 1,
      manifest_entries: 1,
      routes_with_route_params: 0,
      routes_with_search_params: 0,
      route_param_keys: [],
      search_param_keys: [],
      manifest_includes_source_build_routes: true,
      missing_source_build_routes: []
    },
  }),
  server_contracts_compiled: process.env.DX_OMIT_CHECKOUT_ROUTE_HANDLER === "1" ? 2 : 3,
  route_handler_receipts_compiled: process.env.DX_BAD_MANIFEST_ROUTE_HANDLER_RECEIPT_COUNT === "1" ? 0 : 1,
  deploy_adapter_emitted: true,
  next_familiar_compatibility_evidence_emitted: process.env.DX_OMIT_NEXT_FAMILIAR_COMPATIBILITY_EVIDENCE !== "1",
  ...(process.env.DX_WRITE_OLD_NEXT_PARITY_EVIDENCE === "1" ? {
    next_runtime_parity_evidence_emitted: true
  } : {}),
  node_modules_required: false
}, null, 2));
const sourceBuildReceipt = {
  schema: process.env.DX_WRONG_SOURCE_RECEIPT_SCHEMA === "1"
    ? "dx.www.sourceBuildReceipt.experimental"
    : "dx.www.sourceBuildReceipt",
  schema_revision: 1,
  node_modules_required: false,
  summary: {
    routes: 1,
    route_handlers: 2,
    route_outputs: 1,
    styles: 1,
    assets: 1,
    css_minified_styles: 1
  }
};
const badSourceModuleResolver = process.env.DX_BAD_SOURCE_MODULE_RESOLVER === "1";
fs.writeFileSync(path.join(process.cwd(), ".dx", "build", "source-build-manifest.json"), JSON.stringify({
  schema: process.env.DX_WRONG_SOURCE_MANIFEST_SCHEMA === "1"
    ? "dx.www.sourceBuildManifest.experimental"
    : "dx.www.sourceBuildManifest",
  schema_revision: 1,
  routes: [{ route: "/", path: "app/page.tsx" }],
  route_handlers: [
    {
      route: "/api/health",
      path: "app/api/health/route.ts",
      output: ".dx/build/source/app/api/health/route.ts",
      hash: "healthroutehash1",
      methods: ["GET"],
      imports: [],
      parser_backend: "oxc",
      diagnostics: 0,
      execution_model: "source-owned-route-handler-contract",
      lifecycle_scripts_executed: false,
      node_modules_required: false
    },
    ...(process.env.DX_OMIT_CHECKOUT_ROUTE_HANDLER === "1" ? [] : [{
      route: "/api/checkout",
      path: "app/api/checkout/route.ts",
      output: ".dx/build/source/app/api/checkout/route.ts",
      hash: "checkoutroutehash1",
      methods: ["POST"],
      imports: [],
      parser_backend: "oxc",
      diagnostics: 0,
      execution_model: "source-owned-route-handler-contract",
      lifecycle_scripts_executed: false,
      node_modules_required: false
    }])
  ],
  route_outputs: [{
    route: "/",
    source_path: "app/page.tsx",
    html_output: ".dx/build/source-routes/root/index.html",
    packet_output: ".dx/build/source-routes/root/index.dxpk",
    page_graph_output: ".dx/build/source-routes/root/page-graph.json",
    entry_module_chunk_output: ".dx/build/source-routes/root/modules/app-page-tsx-abc.mjs",
    ...(process.env.DX_OMIT_ROUTE_OUTPUT_SERVER_DATA === "1" ? {} : {
      server_data_output: ".dx/build/source-routes/root/server-data.json",
    }),
    source_module_chunks: [
      {
        source_path: "app/page.tsx",
        dependencies: [
          { specifier: "../components/LaunchCard", resolved_path: "components/LaunchCard.tsx", node_modules_required: false },
          { specifier: "../server/launch-copy", resolved_path: "server/launch-copy.ts", node_modules_required: false },
          { specifier: "../server/loaders", resolved_path: "server/loaders.ts", node_modules_required: false },
          ...(badSourceModuleResolver ? [
            { specifier: "@vendor/widget", resolved_path: "node_modules/@vendor/widget/index.js", node_modules_required: false }
          ] : [])
        ],
        diagnostics: badSourceModuleResolver ? 2 : 0,
        node_modules_required: badSourceModuleResolver
      },
      { source_path: "components/LaunchCard.tsx", dependencies: [], node_modules_required: false },
      { source_path: "server/loaders.ts", dependencies: [], node_modules_required: false },
      { source_path: "server/launch-copy.ts", dependencies: [], node_modules_required: false }
    ],
    node_modules_required: false
  }],
  server_data_routes: [{
    route: "/",
    route_source_path: "app/page.tsx",
    output: ".dx/build/source-routes/root/server-data.json",
    entry_count: 1
  }],
  styles: [{
    path: "styles/app.css",
    output: ".dx/build/styles/app.css",
    hash: "stylehash123",
    source_map_output: ".dx/build/styles/app.css.map",
    source_map_linked: true,
    source_map_hash: "stylemaphash123",
    source_map_source_count: 1,
    node_modules_required: false,
    lifecycle_scripts_executed: false,
    source_owned_contract: true,
    external_runtime_required: false,
    external_runtime_executed: false
  }],
  assets: [{
    path: "public/icons/mark.svg",
    output: ".dx/build/public/icons/mark-e261696593b3dbad.svg",
    hash: "e261696593b3dbad",
    size: 7,
    node_modules_required: false,
    lifecycle_scripts_executed: false,
    source_owned_contract: true,
    external_runtime_required: false,
    external_runtime_executed: false
  }],
  node_modules_required: false
}, null, 2));
fs.writeFileSync(path.join(process.cwd(), ".dx", "build", "source-build-receipt.json"), JSON.stringify(sourceBuildReceipt, null, 2));
fs.writeFileSync(path.join(process.cwd(), ".dx", "receipts", "build", "latest.json"), JSON.stringify(sourceBuildReceipt, null, 2));
const routeReceiptHasBadResponse = process.env.DX_BAD_ROUTE_HANDLER_RESPONSE === "1";
const routeReceiptMissingAdapterBoundary = process.env.DX_MISSING_ROUTE_HANDLER_ADAPTER_BOUNDARY === "1";
const routeReceiptResponse = {
  status: routeReceiptHasBadResponse ? 500 : 200,
  content_type: routeReceiptHasBadResponse ? "text/plain" : "application/json; charset=utf-8",
  header_count: routeReceiptHasBadResponse ? 0 : 1
};
if (process.env.DX_OMIT_ROUTE_HANDLER_RECEIPT !== "1") {
  fs.writeFileSync(path.join(process.cwd(), ".dx", "build", "route-handler-receipts.json"), JSON.stringify({
    schema: process.env.DX_WRONG_ROUTE_HANDLER_COLLECTION_SCHEMA === "1"
      ? "dx.next.appRouteHandlerBuildReceipts.experimental"
      : "dx.next.appRouteHandlerBuildReceipts",
    format: 1,
    receipt_count: 1,
    skipped_count: process.env.DX_OMIT_CHECKOUT_ROUTE_HANDLER === "1" ? 0 : 1,
    node_modules_required: false,
    node_modules_present: false,
    lifecycle_scripts_executed: false,
    receipts: [{
      schema: "dx.next.appRouteHandlerReceipt",
      format: 1,
      source_path: "app/api/health/route.ts",
      method: "GET",
      request_path: "/api/health",
      route_params: {},
      search_params: {},
      route_param_count: 0,
      search_param_count: 0,
      response: routeReceiptResponse,
      response_header_count: routeReceiptResponse.header_count,
      execution_model: "source-owned-route-handler-contract",
      lifecycle_scripts_executed: false,
      node_modules_required: false,
      node_modules_present: false,
      runtime_boundary: {
        source_owned: true,
        external_runtime_required: false,
        external_runtime_executed: false
      },
      adapter_boundary: routeReceiptMissingAdapterBoundary ? [] : [
        "Does not import Next.js Route Handler runtime.",
        "Does not require React/RSC, Node/NAPI, npm, or node_modules.",
        "Does not claim unbounded route-handler runtime coverage."
      ]
    }],
    skipped: process.env.DX_OMIT_CHECKOUT_ROUTE_HANDLER === "1" ? [] : [{
      source_path: "app/api/checkout/route.ts",
      method: "POST",
      request_path: "/api/checkout",
      reason: "build receipts execute only safe requestless GET/HEAD route handlers"
    }]
  }, null, 2));
}
const graphNodes = [{ kind: "app-route", path: "app/page.tsx" }];
const graphEdges = [];
if (process.env.DX_OMIT_GRAPH_ROUTE_HANDLER !== "1") {
  graphNodes.push({
    kind: "app-route-handler",
    path: "app/api/health/route.ts",
    route: "/api/health",
    methods: ["GET"],
    execution_model: "source-owned-route-handler-contract",
    lifecycle_scripts_executed: false,
    node_modules_required: false
  });
}
if (process.env.DX_OMIT_CHECKOUT_ROUTE_HANDLER !== "1") {
  graphNodes.push({
    kind: "app-route-handler",
    path: "app/api/checkout/route.ts",
    route: "/api/checkout",
    methods: ["POST"],
    execution_model: "source-owned-route-handler-contract",
    lifecycle_scripts_executed: false,
    node_modules_required: false
  });
}
if (process.env.DX_OMIT_GRAPH_SOURCE_MODULES !== "1") {
  graphNodes.push(
    {
      id: "source-module:server/loaders.ts",
      kind: "source-module",
      path: "server/loaders.ts",
      contract: "dx.www.moduleGraph",
      node_modules_required: false
    },
    {
      id: "source-module:server/launch-copy.ts",
      kind: "source-module",
      path: "server/launch-copy.ts",
      contract: "dx.www.moduleGraph",
      node_modules_required: false
    },
    {
      id: "source-module-chunk:.dx/build/source-routes/root/modules/server-loaders-ts-abc.mjs",
      kind: "source-module-chunk",
      path: ".dx/build/source-routes/root/modules/server-loaders-ts-abc.mjs",
      source_path: "server/loaders.ts",
      node_modules_required: false
    },
    {
      id: "source-module-chunk:.dx/build/source-routes/root/modules/server-launch-copy-ts-abc.mjs",
      kind: "source-module-chunk",
      path: ".dx/build/source-routes/root/modules/server-launch-copy-ts-abc.mjs",
      source_path: "server/launch-copy.ts",
      node_modules_required: false
    }
  );
}
if (process.env.DX_OMIT_GRAPH_COMPILED_FROM_SOURCE_EDGES !== "1") {
  graphEdges.push(
    {
      from: "source-module-chunk:.dx/build/source-routes/root/modules/server-loaders-ts-abc.mjs",
      to: "source-module:server/loaders.ts",
      kind: "compiled-from-source"
    },
    {
      from: "source-module-chunk:.dx/build/source-routes/root/modules/server-launch-copy-ts-abc.mjs",
      to: "source-module:server/launch-copy.ts",
      kind: "compiled-from-source"
    }
  );
}
const graphSchema = process.env.DX_WRONG_GRAPH_SCHEMA === "1" ? "dx.build.graph.experimental" : "dx.build.graph";
fs.writeFileSync(path.join(process.cwd(), ".dx", "receipts", "graph", "latest.json"), JSON.stringify({
  schema: graphSchema,
  graph: { nodes: graphNodes, edges: graphEdges }
}, null, 2));
if (process.env.DX_OMIT_GRAPH_CONSUMER_SNAPSHOT !== "1") {
  const badGraphConsumerSnapshot = process.env.DX_BAD_GRAPH_CONSUMER_SNAPSHOT === "1";
  fs.writeFileSync(path.join(process.cwd(), ".dx", "receipts", "graph", "consumer-snapshot.json"), JSON.stringify({
    schema: badGraphConsumerSnapshot ? "dx.build.graph.consumerSnapshot.experimental" : "dx.build.graph.consumerSnapshot",
    graph: {
      nodeKindCounts: badGraphConsumerSnapshot
        ? { "source-module-chunk": 2 }
        : { "source-module": 2, "source-module-chunk": 2 },
      edgeCount: graphEdges.length
    },
    coreConceptMap: {
      coveredNodeKinds: badGraphConsumerSnapshot ? ["source-module-chunk"] : ["source-module", "source-module-chunk"],
      coveredEdgeKinds: badGraphConsumerSnapshot ? ["imports-source-module"] : ["compiled-from-source", "imports-source-module"]
    },
    consumers: {
      zedPreview: {
        sourceModuleKind: badGraphConsumerSnapshot ? null : "source-module",
        sourceModuleChunkKind: "source-module-chunk"
      }
    }
  }, null, 2));
}
fs.writeFileSync(path.join(process.cwd(), ".dx", "receipts", "build", "zed-handoff.json"), JSON.stringify({
  schema: process.env.DX_WRONG_ZED_HANDOFF_SCHEMA === "1"
    ? "dx.build.zedHandoff.experimental"
    : "dx.build.zedHandoff",
  build_readiness: ".dx/receipts/build/readiness.json",
  installed_binary_smoke_receipt: ".dx/receipts/build/installed-binary-smoke-latest.json",
  style_optimization: {
    style_node_count: 1,
    original_rule_count: 3,
    retained_rule_count: 2,
    pruned_rule_count: 1,
    minified_style_count: 1
  }
}, null, 2));
fs.writeFileSync(path.join(process.cwd(), ".dx", "receipts", "build", "readiness.json"), JSON.stringify({
  schema: "dx.build.readiness",
  source_ready: true,
  source_score: 100,
  product_ready: false,
  product_score: 82,
  graph: {
    route_handler_receipt_output: process.env.DX_BAD_READINESS_ROUTE_HANDLER_RECEIPT_OUTPUT === "1"
      ? ".dx/build/stale-route-handler-receipts.json"
      : ".dx/build/route-handler-receipts.json",
    route_handler_receipts_executed: process.env.DX_BAD_READINESS_ROUTE_HANDLER_RECEIPT_COUNTS === "1" ? 0 : 1,
    route_handler_receipts_skipped: process.env.DX_BAD_READINESS_ROUTE_HANDLER_RECEIPT_COUNTS === "1" ? 0 : (process.env.DX_OMIT_CHECKOUT_ROUTE_HANDLER === "1" ? 0 : 1),
    ...(process.env.DX_OMIT_READINESS_ROUTE_HANDLER_RECEIPT_RUNTIME_GUARDS === "1" ? {} : {
      route_handler_receipts_node_modules_required: process.env.DX_BAD_READINESS_ROUTE_HANDLER_RECEIPT_NODE_MODULES === "1",
      route_handler_receipts_lifecycle_scripts_executed: process.env.DX_BAD_READINESS_ROUTE_HANDLER_RECEIPT_LIFECYCLE === "1"
    })
  },
  installed_binary_smoke: {
    required: true,
    receipt: ".dx/receipts/build/installed-binary-smoke-latest.json",
    status: "pending-governed-refresh"
  },
  receipts: {
    installed_binary_smoke: ".dx/receipts/build/installed-binary-smoke-latest.json"
  }
}, null, 2));
process.exit(0);
`,
  );
  const freshBinaryDate = new Date("2100-01-01T00:00:00.000Z");
  fs.utimesSync(fakeBinary, freshBinaryDate, freshBinaryDate);

  const result = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
    },
  );

  assert.equal(result.status, 0, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  assert.equal(report.schema, "dx.build.installedBinarySmoke");
  assert.equal(report.passed, true);
  assert.equal(report.binary, fakeBinary);
  assert.equal(report.binaryRole, "candidate-override");
  assert.equal(report.binaryIdentity.present, true);
  assert.equal(report.binaryIdentity.path, fakeBinary);
  assert.equal(report.binaryIdentity.byteLength, fs.statSync(fakeBinary).size);
  assert.equal(report.binaryIdentity.sha256, sha256File(fakeBinary));
  assert.equal(report.proof.scope, "candidate-override");
  assert.equal(report.proof.productEligible, false);
  assert.equal(report.proof.installedDefaultRequired, true);
  assert.match(report.proof.nextAction, /rerun without --binary/);
  assert.equal(report.fixture.hasAppPage, true);
  assert.equal(report.fixture.hasAppLayout, true);
  assert.equal(report.fixture.hasRouteHandler, true);
  assert.equal(report.fixture.hasCheckoutRouteHandler, true);
  assert.equal(report.fixture.hasComponent, true);
  assert.equal(report.fixture.hasServerLoader, true);
  assert.equal(report.fixture.hasServerModule, true);
  assert.equal(report.fixture.hasStyleSource, true);
  assert.equal(report.fixture.hasPublicAsset, true);
  assert.equal(report.help.readOnly, true);
  assert.equal(report.help.sourceOwnedContractVisible, true);
  assert.equal(report.build.appRouter.rootHtmlPresent, true);
  assert.equal(report.build.appRouter.rootPacketPresent, true);
  assert.equal(report.build.appRouter.pageGraphPresent, true);
  assert.equal(report.build.appRouter.executionContractPresent, true);
  assert.equal(report.build.appRouter.execution?.route, "/");
  assert.equal(report.build.appRouter.execution?.routeSourcePath, "app/page.tsx");
  assert.equal(report.build.appRouter.execution?.hasRootRouteContract, true);
  assert.equal(report.build.appRouter.execution?.declaresNoNodeModules, true);
  assert.equal(report.build.appRouter.execution?.nodeModulesPresent, false);
  assert.equal(report.build.appRouter.execution?.sourceOwnedRuntimeBoundary, true);
  assert.equal(report.build.appRouter.execution?.externalRuntimeRequired, false);
  assert.equal(report.build.appRouter.execution?.externalRuntimeExecuted, false);
  assert.equal(report.build.appRouter.serverDataPresent, true);
  assert.equal(report.build.appRouter.serverData.route, "/");
  assert.equal(report.build.appRouter.serverData.routeSourcePath, "app/page.tsx");
  assert.equal(report.build.appRouter.serverData.hasRootRouteContract, true);
  assert.equal(report.build.appRouter.serverData.hasLoaderEntry, true);
  assert.equal(report.build.appRouter.serverData.hasLoaderValue, true);
  assert.equal(report.build.appRouter.serverData.declaresNoNodeModules, true);
  assert.equal(report.build.manifest.hasCssSummary, true);
  assert.equal(report.build.manifest.hasServerDataSummary, true);
  assert.equal(report.build.manifest.routeHandlerReceiptsCompiled, 1);
  assert.equal(report.build.manifest.routeHandlerReceiptsCompiledMatchesCollection, true);
  assert.equal(report.build.manifest.nextFamiliarCompatibilityEvidenceEmitted, true);
  assert.equal(report.build.manifest.oldNextRuntimeParityEvidenceFlagPresent, false);
  assert.equal(report.build.manifest.serverDataRoutes.present, true);
  assert.equal(report.build.manifest.serverDataRoutes.compiledCount, 1);
  assert.equal(report.build.manifest.serverDataRoutes.routeCount, 1);
  assert.equal(report.build.manifest.serverDataRoutes.hasRootRoute, true);
  assert.equal(
    report.build.manifest.serverDataRoutes.rootRoute.output.path,
    ".dx/build/source-routes/root/server-data.json",
  );
  assert.equal(report.build.manifest.serverDataRoutes.rootRoute.output.present, true);
  assert.equal(report.build.manifest.serverDataRoutes.rootRoute.matchesSourceBuildRouteOutput, true);
  assert.equal(report.build.manifest.serverDataRoutes.rootRoute.declaresNoNodeModules, true);
  assert.equal(report.build.manifest.serverDataRoutes.rootRoute.lifecycleScriptsExecuted, false);
  assert.equal(report.build.manifest.serverDataRoutes.rootRoute.sourceOwnedContract, true);
  assert.equal(report.build.manifest.serverDataRoutes.rootRoute.externalRuntimeRequired, false);
  assert.equal(report.build.manifest.serverDataRoutes.rootRoute.externalRuntimeExecuted, false);
  assert.equal(report.build.serverContracts.present, true);
  assert.equal(report.build.serverContracts.hasHealthRouteHandler, true);
  assert.equal(report.build.serverContracts.hasCheckoutRouteHandler, true);
  assert.equal(report.build.serverContracts.hasServerLoader, true);
  assert.equal(report.build.serverContracts.lifecycleScriptsExecuted, false);
  assert.equal(report.build.deployAdapter.present, true);
  assert.equal(report.build.deployAdapter.noNodeModulesRequired, true);
  assert.equal(report.build.deployAdapter.hasRootServerDataRoute, true);
  assert.equal(report.build.deployAdapter.hasHealthCheck, true);
  assert.equal(report.build.deployAdapter.hasCheckoutRouteHandler, true);
  assert.equal(report.build.deployAdapter.hasNextFamiliarCompatibilityEvidence, true);
  assert.equal(report.build.deployAdapter.hasRemovedParityDeployArtifact, false);
  assert.equal(report.build.nextFamiliarCompatibilityEvidence.present, true);
  assert.equal(report.build.nextFamiliarCompatibilityEvidence.evidenceKind, "next-familiar-compatibility");
  assert.equal(report.build.nextFamiliarCompatibilityEvidence.evidenceMode, "next-familiar-source-output-readiness");
  assert.equal(report.build.nextFamiliarCompatibilityEvidence.score, 100);
  assert.equal(report.build.nextFamiliarCompatibilityEvidence.declaresNoNodeModules, true);
  assert.equal(report.build.nextFamiliarCompatibilityEvidence.oldRuntimeParityArtifactPresent, false);
  assert.equal(report.build.sourceBuild.manifest.present, true);
  assert.equal(report.build.sourceBuild.manifest.hasRootRoute, true);
  assert.equal(report.build.sourceBuild.manifest.hasRouteHandler, true);
  assert.deepEqual(report.build.sourceBuild.manifest.routeHandlerMethods, ["GET"]);
  assert.equal(report.build.sourceBuild.manifest.routeHandlerDeclaresNoNodeModules, true);
  assert.equal(report.build.sourceBuild.manifest.hasCheckoutRouteHandler, true);
  assert.deepEqual(report.build.sourceBuild.manifest.checkoutRouteHandlerMethods, ["POST"]);
  assert.equal(report.build.sourceBuild.manifest.checkoutRouteHandlerDeclaresNoNodeModules, true);
  assert.equal(report.build.sourceBuild.manifest.rootRouteOutput?.present, true);
  assert.equal(
    report.build.sourceBuild.manifest.rootRouteOutput?.html.path,
    ".dx/build/source-routes/root/index.html",
  );
  assert.equal(
    report.build.sourceBuild.manifest.rootRouteOutput?.serverData?.path,
    ".dx/build/source-routes/root/server-data.json",
  );
  assert.equal(report.build.sourceBuild.manifest.rootRouteOutput?.serverData.present, true);
  assert.equal(report.build.sourceBuild.manifest.hasStyle, true);
  assert.equal(report.build.sourceBuild.manifest.hasStyleOutput, true);
  assert.equal(report.build.sourceBuild.manifest.styleOutputPath, ".dx/build/styles/app.css");
  assert.equal(report.build.sourceBuild.manifest.hasPublicAsset, true);
  assert.equal(report.build.sourceBuild.manifest.hasPublicAssetOutput, true);
  assert.equal(
    report.build.sourceBuild.manifest.publicAssetOutputPath,
    ".dx/build/public/icons/mark-e261696593b3dbad.svg",
  );
  assert.equal(report.build.sourceBuild.manifest.hasLinkedComponent, true);
  assert.equal(report.build.sourceBuild.manifest.hasLinkedServerLoader, true);
  assert.equal(report.build.sourceBuild.manifest.hasLinkedServerModule, true);
  assert.equal(report.build.sourceBuild.manifest.declaresNoNodeModules, true);
  assert.equal(report.build.sourceBuild.manifest.nodeModulesRequired, false);
  assert.equal(report.build.sourceBuild.receipt.present, true);
  assert.equal(report.build.sourceBuild.receipt.declaresNoNodeModules, true);
  assert.equal(report.build.sourceBuild.receipt.nodeModulesRequired, false);
  assert.equal(report.build.sourceBuild.receipt.routeHandlers, 2);
  assert.equal(report.build.sourceBuild.canonicalReceipt.present, true);
  assert.equal(report.build.sourceBuild.graphReceipt.present, true);
  assert.equal(report.build.sourceBuild.graphReceipt.schema, "dx.build.graph");
  assert.equal(report.build.sourceBuild.graphReceipt.hasRouteHandlerNode, true);
  assert.deepEqual(report.build.sourceBuild.graphReceipt.routeHandlerMethods, ["GET"]);
  assert.equal(report.build.sourceBuild.graphReceipt.routeHandlerDeclaresNoNodeModules, true);
  assert.equal(report.build.sourceBuild.graphReceipt.hasCheckoutRouteHandlerNode, true);
  assert.deepEqual(report.build.sourceBuild.graphReceipt.checkoutRouteHandlerMethods, ["POST"]);
  assert.equal(report.build.sourceBuild.graphReceipt.checkoutRouteHandlerDeclaresNoNodeModules, true);
  assert.equal(report.build.sourceBuild.graphReceipt.hasServerLoaderSourceModule, true);
  assert.equal(report.build.sourceBuild.graphReceipt.serverLoaderSourceModuleDeclaresNoNodeModules, true);
  assert.equal(report.build.sourceBuild.graphReceipt.serverLoaderSourceModuleCompiledFromSource, true);
  assert.equal(report.build.sourceBuild.graphReceipt.hasServerModuleSourceModule, true);
  assert.equal(report.build.sourceBuild.graphReceipt.serverModuleSourceModuleDeclaresNoNodeModules, true);
  assert.equal(report.build.sourceBuild.graphReceipt.serverModuleSourceModuleCompiledFromSource, true);
  assert.equal(report.build.sourceBuild.graphConsumerSnapshot.present, true);
  assert.equal(report.build.sourceBuild.graphConsumerSnapshot.schema, "dx.build.graph.consumerSnapshot");
  assert.equal(report.build.sourceBuild.graphConsumerSnapshot.sourceModuleCount, 2);
  assert.equal(report.build.sourceBuild.graphConsumerSnapshot.coversSourceModuleKind, true);
  assert.equal(report.build.sourceBuild.graphConsumerSnapshot.coversCompiledFromSourceEdge, true);
  assert.equal(report.build.sourceBuild.graphConsumerSnapshot.zedPreviewSourceModuleKind, "source-module");
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.present, true);
  assert.equal(
    report.build.sourceBuild.routeHandlerReceipt.collectionSchema,
    "dx.next.appRouteHandlerBuildReceipts",
  );
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.collectionFormat, 1);
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.collectionDeclaresNoNodeModules, true);
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.collectionNodeModulesPresent, false);
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.collectionLifecycleScriptsExecuted, false);
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.receiptCount, 1);
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.requiredReceiptCount, 1);
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.skippedCount, 1);
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.requiredSkipCount, 1);
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.hasHealthGetReceipt, true);
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.hasCheckoutPostSkipped, true);
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.schema, "dx.next.appRouteHandlerReceipt");
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.format, 1);
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.declaresNoNodeModules, true);
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.nodeModulesRequired, false);
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.lifecycleScriptsExecuted, false);
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.sourceOwnedRuntimeBoundary, true);
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.externalRuntimeRequired, false);
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.externalRuntimeExecuted, false);
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.responseStatus, 200);
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.responseContentType, "application/json; charset=utf-8");
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.responseHeaderCount, 1);
  assert.equal(report.build.sourceBuild.routeHandlerReceipt.hasAdapterBoundary, true);
  assert.equal(report.build.zedHandoff.hasStyleOptimization, true);
  assert.equal(report.build.zedHandoff.hasBuildReadinessPointer, true);
  assert.equal(report.build.zedHandoff.hasInstalledBinarySmokeReceiptPointer, true);
  assert.equal(report.build.readiness.present, true);
  assert.equal(report.build.readiness.sourceReady, true);
  assert.equal(report.build.readiness.sourceScore, 100);
  assert.equal(report.build.readiness.productReady, false);
  assert.equal(report.build.readiness.productScore, 82);
  assert.equal(report.build.readiness.routeHandlerReceiptOutput, ".dx/build/route-handler-receipts.json");
  assert.equal(report.build.readiness.routeHandlerReceiptOutputMatchesActual, true);
  assert.equal(report.build.readiness.routeHandlerReceiptsExecuted, 1);
  assert.equal(report.build.readiness.routeHandlerReceiptsSkipped, 1);
  assert.equal(report.build.readiness.routeHandlerReceiptsNodeModulesRequired, false);
  assert.equal(report.build.readiness.routeHandlerReceiptsLifecycleScriptsExecuted, false);
  assert.equal(report.build.readiness.hasInstalledBinarySmokeReceipt, true);
  const receiptPath = path.join(report.projectRoot, ".dx", "receipts", "build", "installed-binary-smoke-latest.json");
  assert.equal(report.receiptPath, receiptPath);
  assert.deepEqual(JSON.parse(fs.readFileSync(receiptPath, "utf8")), report);
  assert.equal(report.build.nodeModulesCreated, false);

  const missingNextFamiliarEvidenceResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_OMIT_NEXT_FAMILIAR_COMPATIBILITY_EVIDENCE: "1" },
    },
  );
  assert.equal(
    missingNextFamiliarEvidenceResult.status,
    1,
    missingNextFamiliarEvidenceResult.stdout + missingNextFamiliarEvidenceResult.stderr,
  );
  const missingNextFamiliarEvidenceReport = JSON.parse(missingNextFamiliarEvidenceResult.stdout);
  assert.equal(missingNextFamiliarEvidenceReport.build.nextFamiliarCompatibilityEvidence.present, false);
  assert.equal(missingNextFamiliarEvidenceReport.build.deployAdapter.hasNextFamiliarCompatibilityEvidence, false);
  assert.ok(
    missingNextFamiliarEvidenceReport.failures.includes(
      "dx build did not write next-familiar compatibility evidence",
    ),
    JSON.stringify(missingNextFamiliarEvidenceReport.failures, null, 2),
  );

  const oldNextParityEvidenceResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_WRITE_OLD_NEXT_PARITY_EVIDENCE: "1" },
    },
  );
  assert.equal(
    oldNextParityEvidenceResult.status,
    1,
    oldNextParityEvidenceResult.stdout + oldNextParityEvidenceResult.stderr,
  );
  const oldNextParityEvidenceReport = JSON.parse(oldNextParityEvidenceResult.stdout);
  assert.equal(oldNextParityEvidenceReport.build.nextFamiliarCompatibilityEvidence.oldRuntimeParityArtifactPresent, true);
  assert.equal(oldNextParityEvidenceReport.build.manifest.oldNextRuntimeParityEvidenceFlagPresent, true);
  assert.equal(oldNextParityEvidenceReport.build.deployAdapter.hasRemovedParityDeployArtifact, false);
  assert.ok(
    oldNextParityEvidenceReport.failures.includes("dx build wrote removed next-runtime-parity evidence"),
    JSON.stringify(oldNextParityEvidenceReport.failures, null, 2),
  );

  const missingGraphResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_OMIT_GRAPH_ROUTE_HANDLER: "1" },
    },
  );
  assert.equal(missingGraphResult.status, 1, missingGraphResult.stdout + missingGraphResult.stderr);
  const missingGraphReport = JSON.parse(missingGraphResult.stdout);
  assert.equal(missingGraphReport.build.sourceBuild.graphReceipt.hasRouteHandlerNode, false);
  assert.ok(
    missingGraphReport.failures.includes("dx.build.graph is missing the app/api/health/route.ts route-handler node"),
    JSON.stringify(missingGraphReport.failures, null, 2),
  );

  const missingGraphSourceModulesResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_OMIT_GRAPH_SOURCE_MODULES: "1" },
    },
  );
  assert.equal(
    missingGraphSourceModulesResult.status,
    1,
    missingGraphSourceModulesResult.stdout + missingGraphSourceModulesResult.stderr,
  );
  const missingGraphSourceModulesReport = JSON.parse(missingGraphSourceModulesResult.stdout);
  assert.equal(missingGraphSourceModulesReport.build.sourceBuild.graphReceipt.hasServerLoaderSourceModule, false);
  assert.equal(missingGraphSourceModulesReport.build.sourceBuild.graphReceipt.hasServerModuleSourceModule, false);
  assert.ok(
    missingGraphSourceModulesReport.failures.includes(
      "dx.build.graph is missing the server/loaders.ts source-module node",
    ),
    JSON.stringify(missingGraphSourceModulesReport.failures, null, 2),
  );
  assert.ok(
    missingGraphSourceModulesReport.failures.includes(
      "dx.build.graph is missing the server/launch-copy.ts source-module node",
    ),
    JSON.stringify(missingGraphSourceModulesReport.failures, null, 2),
  );

  const missingCompiledFromSourceResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_OMIT_GRAPH_COMPILED_FROM_SOURCE_EDGES: "1" },
    },
  );
  assert.equal(
    missingCompiledFromSourceResult.status,
    1,
    missingCompiledFromSourceResult.stdout + missingCompiledFromSourceResult.stderr,
  );
  const missingCompiledFromSourceReport = JSON.parse(missingCompiledFromSourceResult.stdout);
  assert.equal(missingCompiledFromSourceReport.build.sourceBuild.graphReceipt.serverLoaderSourceModuleCompiledFromSource, false);
  assert.equal(missingCompiledFromSourceReport.build.sourceBuild.graphReceipt.serverModuleSourceModuleCompiledFromSource, false);
  assert.ok(
    missingCompiledFromSourceReport.failures.includes(
      "dx.build.graph is missing compiled-from-source edge for server/loaders.ts",
    ),
    JSON.stringify(missingCompiledFromSourceReport.failures, null, 2),
  );
  assert.ok(
    missingCompiledFromSourceReport.failures.includes(
      "dx.build.graph is missing compiled-from-source edge for server/launch-copy.ts",
    ),
    JSON.stringify(missingCompiledFromSourceReport.failures, null, 2),
  );

  const missingGraphConsumerSnapshotResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_OMIT_GRAPH_CONSUMER_SNAPSHOT: "1" },
    },
  );
  assert.equal(
    missingGraphConsumerSnapshotResult.status,
    1,
    missingGraphConsumerSnapshotResult.stdout + missingGraphConsumerSnapshotResult.stderr,
  );
  const missingGraphConsumerSnapshotReport = JSON.parse(missingGraphConsumerSnapshotResult.stdout);
  assert.equal(missingGraphConsumerSnapshotReport.build.sourceBuild.graphConsumerSnapshot.present, false);
  assert.ok(
    missingGraphConsumerSnapshotReport.failures.includes(
      "dx build did not write .dx/receipts/graph/consumer-snapshot.json",
    ),
    JSON.stringify(missingGraphConsumerSnapshotReport.failures, null, 2),
  );

  const badGraphConsumerSnapshotResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_BAD_GRAPH_CONSUMER_SNAPSHOT: "1" },
    },
  );
  assert.equal(
    badGraphConsumerSnapshotResult.status,
    1,
    badGraphConsumerSnapshotResult.stdout + badGraphConsumerSnapshotResult.stderr,
  );
  const badGraphConsumerSnapshotReport = JSON.parse(badGraphConsumerSnapshotResult.stdout);
  assert.equal(badGraphConsumerSnapshotReport.build.sourceBuild.graphConsumerSnapshot.sourceModuleCount, 0);
  assert.equal(badGraphConsumerSnapshotReport.build.sourceBuild.graphConsumerSnapshot.coversCompiledFromSourceEdge, false);
  assert.ok(
    badGraphConsumerSnapshotReport.failures.includes(
      "dx.build.graph consumer snapshot is missing source-module counts",
    ),
    JSON.stringify(badGraphConsumerSnapshotReport.failures, null, 2),
  );
  assert.ok(
    badGraphConsumerSnapshotReport.failures.includes(
      "dx.build.graph consumer snapshot is missing compiled-from-source coverage",
    ),
    JSON.stringify(badGraphConsumerSnapshotReport.failures, null, 2),
  );

  const missingCheckoutRouteHandlerResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_OMIT_CHECKOUT_ROUTE_HANDLER: "1" },
    },
  );
  assert.equal(
    missingCheckoutRouteHandlerResult.status,
    1,
    missingCheckoutRouteHandlerResult.stdout + missingCheckoutRouteHandlerResult.stderr,
  );
  const missingCheckoutRouteHandlerReport = JSON.parse(missingCheckoutRouteHandlerResult.stdout);
  assert.ok(
    missingCheckoutRouteHandlerReport.failures.includes(
      "source-build manifest is missing app/api/checkout/route.ts route-handler evidence",
    ),
    JSON.stringify(missingCheckoutRouteHandlerReport.failures, null, 2),
  );
  assert.ok(
    missingCheckoutRouteHandlerReport.failures.includes(
      ".dx/build/route-handler-receipts.json is missing app/api/checkout/route.ts POST skipped evidence",
    ),
    JSON.stringify(missingCheckoutRouteHandlerReport.failures, null, 2),
  );
  assert.ok(
    missingCheckoutRouteHandlerReport.failures.includes(
      "dx.build.graph is missing the app/api/checkout/route.ts route-handler node",
    ),
    JSON.stringify(missingCheckoutRouteHandlerReport.failures, null, 2),
  );
  assert.ok(
    missingCheckoutRouteHandlerReport.failures.includes(
      "server contracts are missing app/api/checkout/route.ts",
    ),
    JSON.stringify(missingCheckoutRouteHandlerReport.failures, null, 2),
  );
  assert.ok(
    missingCheckoutRouteHandlerReport.failures.includes(
      "deploy adapter is missing /api/checkout route handler",
    ),
    JSON.stringify(missingCheckoutRouteHandlerReport.failures, null, 2),
  );

  const missingRouteReceiptResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_OMIT_ROUTE_HANDLER_RECEIPT: "1" },
    },
  );
  assert.equal(missingRouteReceiptResult.status, 1, missingRouteReceiptResult.stdout + missingRouteReceiptResult.stderr);
  const missingRouteReceiptReport = JSON.parse(missingRouteReceiptResult.stdout);
  assert.equal(missingRouteReceiptReport.build.sourceBuild.routeHandlerReceipt.present, false);
  assert.ok(
    missingRouteReceiptReport.failures.includes("dx build did not write .dx/build/route-handler-receipts.json"),
    JSON.stringify(missingRouteReceiptReport.failures, null, 2),
  );

  const badManifestRouteHandlerReceiptCountResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_BAD_MANIFEST_ROUTE_HANDLER_RECEIPT_COUNT: "1" },
    },
  );
  assert.equal(
    badManifestRouteHandlerReceiptCountResult.status,
    1,
    badManifestRouteHandlerReceiptCountResult.stdout + badManifestRouteHandlerReceiptCountResult.stderr,
  );
  const badManifestRouteHandlerReceiptCountReport = JSON.parse(
    badManifestRouteHandlerReceiptCountResult.stdout,
  );
  assert.equal(badManifestRouteHandlerReceiptCountReport.build.manifest.routeHandlerReceiptsCompiled, 0);
  assert.equal(
    badManifestRouteHandlerReceiptCountReport.build.manifest.routeHandlerReceiptsCompiledMatchesCollection,
    false,
  );
  assert.ok(
    badManifestRouteHandlerReceiptCountReport.failures.includes(
      "manifest route_handler_receipts_compiled does not match route-handler receipts",
    ),
    JSON.stringify(badManifestRouteHandlerReceiptCountReport.failures, null, 2),
  );

  const badReadinessRouteHandlerReceiptOutputResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_BAD_READINESS_ROUTE_HANDLER_RECEIPT_OUTPUT: "1" },
    },
  );
  assert.equal(
    badReadinessRouteHandlerReceiptOutputResult.status,
    1,
    badReadinessRouteHandlerReceiptOutputResult.stdout + badReadinessRouteHandlerReceiptOutputResult.stderr,
  );
  const badReadinessRouteHandlerReceiptOutputReport = JSON.parse(
    badReadinessRouteHandlerReceiptOutputResult.stdout,
  );
  assert.equal(
    badReadinessRouteHandlerReceiptOutputReport.build.readiness.routeHandlerReceiptOutput,
    ".dx/build/stale-route-handler-receipts.json",
  );
  assert.equal(
    badReadinessRouteHandlerReceiptOutputReport.build.readiness.routeHandlerReceiptOutputMatchesActual,
    false,
  );
  assert.ok(
    badReadinessRouteHandlerReceiptOutputReport.failures.includes(
      "build readiness route_handler_receipt_output does not match route-handler receipts path",
    ),
    JSON.stringify(badReadinessRouteHandlerReceiptOutputReport.failures, null, 2),
  );

  const badReadinessRouteHandlerReceiptContractResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: {
        ...process.env,
        DX_BAD_READINESS_ROUTE_HANDLER_RECEIPT_COUNTS: "1",
        DX_BAD_READINESS_ROUTE_HANDLER_RECEIPT_NODE_MODULES: "1",
        DX_BAD_READINESS_ROUTE_HANDLER_RECEIPT_LIFECYCLE: "1",
      },
    },
  );
  assert.equal(
    badReadinessRouteHandlerReceiptContractResult.status,
    1,
    badReadinessRouteHandlerReceiptContractResult.stdout + badReadinessRouteHandlerReceiptContractResult.stderr,
  );
  const badReadinessRouteHandlerReceiptContractReport = JSON.parse(
    badReadinessRouteHandlerReceiptContractResult.stdout,
  );
  assert.equal(
    badReadinessRouteHandlerReceiptContractReport.build.readiness.routeHandlerReceiptsExecuted,
    0,
  );
  assert.equal(
    badReadinessRouteHandlerReceiptContractReport.build.readiness.routeHandlerReceiptsSkipped,
    0,
  );
  assert.equal(
    badReadinessRouteHandlerReceiptContractReport.build.readiness.routeHandlerReceiptsNodeModulesRequired,
    true,
  );
  assert.equal(
    badReadinessRouteHandlerReceiptContractReport.build.readiness.routeHandlerReceiptsLifecycleScriptsExecuted,
    true,
  );
  for (const expectedFailure of [
    "build readiness route-handler receipt executed count does not match receipt collection",
    "build readiness route-handler receipt skipped count does not match receipt collection",
    "build readiness route-handler receipts require node_modules",
    "build readiness route-handler receipts executed lifecycle scripts",
  ]) {
    assert.ok(
      badReadinessRouteHandlerReceiptContractReport.failures.includes(expectedFailure),
      JSON.stringify(badReadinessRouteHandlerReceiptContractReport.failures, null, 2),
    );
  }

  const missingReadinessRouteHandlerReceiptRuntimeGuardsResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: {
        ...process.env,
        DX_OMIT_READINESS_ROUTE_HANDLER_RECEIPT_RUNTIME_GUARDS: "1",
      },
    },
  );
  assert.equal(
    missingReadinessRouteHandlerReceiptRuntimeGuardsResult.status,
    1,
    missingReadinessRouteHandlerReceiptRuntimeGuardsResult.stdout +
      missingReadinessRouteHandlerReceiptRuntimeGuardsResult.stderr,
  );
  const missingReadinessRouteHandlerReceiptRuntimeGuardsReport = JSON.parse(
    missingReadinessRouteHandlerReceiptRuntimeGuardsResult.stdout,
  );
  assert.equal(
    missingReadinessRouteHandlerReceiptRuntimeGuardsReport.build.readiness.routeHandlerReceiptsDeclareNoNodeModules,
    false,
  );
  assert.equal(
    missingReadinessRouteHandlerReceiptRuntimeGuardsReport.build.readiness.routeHandlerReceiptsDeclareNoLifecycleScripts,
    false,
  );
  for (const expectedFailure of [
    "build readiness route-handler receipts do not declare node_modules_required=false",
    "build readiness route-handler receipts do not declare lifecycle_scripts_executed=false",
  ]) {
    assert.ok(
      missingReadinessRouteHandlerReceiptRuntimeGuardsReport.failures.includes(expectedFailure),
      JSON.stringify(missingReadinessRouteHandlerReceiptRuntimeGuardsReport.failures, null, 2),
    );
  }

  const wrongRouteHandlerCollectionSchemaResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_WRONG_ROUTE_HANDLER_COLLECTION_SCHEMA: "1" },
    },
  );
  assert.equal(
    wrongRouteHandlerCollectionSchemaResult.status,
    1,
    wrongRouteHandlerCollectionSchemaResult.stdout + wrongRouteHandlerCollectionSchemaResult.stderr,
  );
  const wrongRouteHandlerCollectionSchemaReport = JSON.parse(
    wrongRouteHandlerCollectionSchemaResult.stdout,
  );
  assert.equal(
    wrongRouteHandlerCollectionSchemaReport.build.sourceBuild.routeHandlerReceipt.collectionSchema,
    "dx.next.appRouteHandlerBuildReceipts.experimental",
  );
  assert.ok(
    wrongRouteHandlerCollectionSchemaReport.failures.includes(
      "route-handler receipt collection has an unexpected schema",
    ),
    JSON.stringify(wrongRouteHandlerCollectionSchemaReport.failures, null, 2),
  );

  const wrongGraphSchemaResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_WRONG_GRAPH_SCHEMA: "1" },
    },
  );
  assert.equal(wrongGraphSchemaResult.status, 1, wrongGraphSchemaResult.stdout + wrongGraphSchemaResult.stderr);
  const wrongGraphSchemaReport = JSON.parse(wrongGraphSchemaResult.stdout);
  assert.equal(wrongGraphSchemaReport.build.sourceBuild.graphReceipt.schema, "dx.build.graph.experimental");
  assert.ok(
    wrongGraphSchemaReport.failures.includes("dx.build.graph receipt has an unexpected schema"),
    JSON.stringify(wrongGraphSchemaReport.failures, null, 2),
  );

  const wrongManifestSchemaResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_WRONG_SOURCE_MANIFEST_SCHEMA: "1" },
    },
  );
  assert.equal(wrongManifestSchemaResult.status, 1, wrongManifestSchemaResult.stdout + wrongManifestSchemaResult.stderr);
  const wrongManifestSchemaReport = JSON.parse(wrongManifestSchemaResult.stdout);
  assert.equal(wrongManifestSchemaReport.build.sourceBuild.manifest.schema, "dx.www.sourceBuildManifest.experimental");
  assert.ok(
    wrongManifestSchemaReport.failures.includes("source-build manifest has an unexpected schema"),
    JSON.stringify(wrongManifestSchemaReport.failures, null, 2),
  );

  const wrongReceiptSchemaResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_WRONG_SOURCE_RECEIPT_SCHEMA: "1" },
    },
  );
  assert.equal(wrongReceiptSchemaResult.status, 1, wrongReceiptSchemaResult.stdout + wrongReceiptSchemaResult.stderr);
  const wrongReceiptSchemaReport = JSON.parse(wrongReceiptSchemaResult.stdout);
  assert.equal(wrongReceiptSchemaReport.build.sourceBuild.receipt.schema, "dx.www.sourceBuildReceipt.experimental");
  assert.equal(
    wrongReceiptSchemaReport.build.sourceBuild.canonicalReceipt.schema,
    "dx.www.sourceBuildReceipt.experimental",
  );
  assert.ok(
    wrongReceiptSchemaReport.failures.includes("source-build receipt has an unexpected schema"),
    JSON.stringify(wrongReceiptSchemaReport.failures, null, 2),
  );
  assert.ok(
    wrongReceiptSchemaReport.failures.includes("canonical source-build receipt has an unexpected schema"),
    JSON.stringify(wrongReceiptSchemaReport.failures, null, 2),
  );

  const wrongZedSchemaResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_WRONG_ZED_HANDOFF_SCHEMA: "1" },
    },
  );
  assert.equal(wrongZedSchemaResult.status, 1, wrongZedSchemaResult.stdout + wrongZedSchemaResult.stderr);
  const wrongZedSchemaReport = JSON.parse(wrongZedSchemaResult.stdout);
  assert.equal(wrongZedSchemaReport.build.zedHandoff.schema, "dx.build.zedHandoff.experimental");
  assert.ok(
    wrongZedSchemaReport.failures.includes("Zed handoff receipt has an unexpected schema"),
    JSON.stringify(wrongZedSchemaReport.failures, null, 2),
  );

  const badServerDataRouteResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_BAD_SERVER_DATA_ROUTE: "1" },
    },
  );
  assert.equal(badServerDataRouteResult.status, 1, badServerDataRouteResult.stdout + badServerDataRouteResult.stderr);
  const badServerDataRouteReport = JSON.parse(badServerDataRouteResult.stdout);
  assert.equal(badServerDataRouteReport.build.appRouter.serverData.route, "/wrong");
  assert.equal(badServerDataRouteReport.build.appRouter.serverData.hasRootRouteContract, false);
  assert.ok(
    badServerDataRouteReport.failures.includes("server-data.json does not describe the root app route"),
    JSON.stringify(badServerDataRouteReport.failures, null, 2),
  );

  const missingManifestServerDataRoutesResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_OMIT_MANIFEST_SERVER_DATA_ROUTES: "1" },
    },
  );
  assert.equal(
    missingManifestServerDataRoutesResult.status,
    1,
    missingManifestServerDataRoutesResult.stdout + missingManifestServerDataRoutesResult.stderr,
  );
  const missingManifestServerDataRoutesReport = JSON.parse(
    missingManifestServerDataRoutesResult.stdout,
  );
  assert.equal(missingManifestServerDataRoutesReport.build.manifest.serverDataRoutes.present, false);
  assert.ok(
    missingManifestServerDataRoutesReport.failures.includes(
      "manifest is missing server_data_routes",
    ),
    JSON.stringify(missingManifestServerDataRoutesReport.failures, null, 2),
  );

  const badManifestServerDataOutputResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_BAD_MANIFEST_SERVER_DATA_OUTPUT: "1" },
    },
  );
  assert.equal(
    badManifestServerDataOutputResult.status,
    1,
    badManifestServerDataOutputResult.stdout + badManifestServerDataOutputResult.stderr,
  );
  const badManifestServerDataOutputReport = JSON.parse(
    badManifestServerDataOutputResult.stdout,
  );
  assert.equal(
    badManifestServerDataOutputReport.build.manifest.serverDataRoutes.rootRoute.output.present,
    false,
  );
  assert.equal(
    badManifestServerDataOutputReport.build.manifest.serverDataRoutes.rootRoute.matchesSourceBuildRouteOutput,
    false,
  );
  assert.ok(
    badManifestServerDataOutputReport.failures.includes(
      "manifest server_data_routes / app/page.tsx output does not match source-build route output",
    ),
    JSON.stringify(badManifestServerDataOutputReport.failures, null, 2),
  );

  const missingRouteOutputServerDataResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_OMIT_ROUTE_OUTPUT_SERVER_DATA: "1" },
    },
  );
  assert.equal(
    missingRouteOutputServerDataResult.status,
    1,
    missingRouteOutputServerDataResult.stdout + missingRouteOutputServerDataResult.stderr,
  );
  const missingRouteOutputServerDataReport = JSON.parse(
    missingRouteOutputServerDataResult.stdout,
  );
  assert.equal(
    missingRouteOutputServerDataReport.build.sourceBuild.manifest.rootRouteOutput.serverData.present,
    false,
  );
  assert.ok(
    missingRouteOutputServerDataReport.failures.includes(
      "source-build root route output is missing manifest-declared server-data output",
    ),
    JSON.stringify(missingRouteOutputServerDataReport.failures, null, 2),
  );

  const badSourceModuleResolverResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_BAD_SOURCE_MODULE_RESOLVER: "1" },
    },
  );
  assert.equal(
    badSourceModuleResolverResult.status,
    1,
    badSourceModuleResolverResult.stdout + badSourceModuleResolverResult.stderr,
  );
  const badSourceModuleResolverReport = JSON.parse(badSourceModuleResolverResult.stdout);
  assert.equal(badSourceModuleResolverReport.build.sourceBuild.manifest.resolverEvidence.diagnosticCount, 2);
  assert.equal(badSourceModuleResolverReport.build.sourceBuild.manifest.resolverEvidence.nodeModuleModuleCount, 1);
  assert.equal(
    badSourceModuleResolverReport.build.sourceBuild.manifest.resolverEvidence.nodeModuleDependencyCount,
    1,
  );
  for (const expectedFailure of [
    "source-build source module app/page.tsx has 2 resolver diagnostics",
    "source-build source module app/page.tsx requires node_modules",
    "source-build source module app/page.tsx dependency @vendor/widget requires node_modules",
  ]) {
    assert.ok(
      badSourceModuleResolverReport.failures.includes(expectedFailure),
      JSON.stringify(badSourceModuleResolverReport.failures, null, 2),
    );
  }

  const badAppRouterExecutionRouteResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_BAD_APP_ROUTER_EXECUTION_ROUTE: "1" },
    },
  );
  assert.equal(
    badAppRouterExecutionRouteResult.status,
    1,
    badAppRouterExecutionRouteResult.stdout + badAppRouterExecutionRouteResult.stderr,
  );
  const badAppRouterExecutionRouteReport = JSON.parse(badAppRouterExecutionRouteResult.stdout);
  assert.equal(badAppRouterExecutionRouteReport.build.appRouter.execution.route, "/wrong");
  assert.equal(badAppRouterExecutionRouteReport.build.appRouter.execution.hasRootRouteContract, false);
  assert.ok(
    badAppRouterExecutionRouteReport.failures.includes("app-router-execution.json does not describe the root app route"),
    JSON.stringify(badAppRouterExecutionRouteReport.failures, null, 2),
  );

  const badRouteHandlerResponseResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_BAD_ROUTE_HANDLER_RESPONSE: "1" },
    },
  );
  assert.equal(
    badRouteHandlerResponseResult.status,
    1,
    badRouteHandlerResponseResult.stdout + badRouteHandlerResponseResult.stderr,
  );
  const badRouteHandlerResponseReport = JSON.parse(badRouteHandlerResponseResult.stdout);
  assert.equal(badRouteHandlerResponseReport.build.sourceBuild.routeHandlerReceipt.responseStatus, 500);
  assert.equal(badRouteHandlerResponseReport.build.sourceBuild.routeHandlerReceipt.responseContentType, "text/plain");
  assert.equal(badRouteHandlerResponseReport.build.sourceBuild.routeHandlerReceipt.responseHeaderCount, 0);
  assert.ok(
    badRouteHandlerResponseReport.failures.includes(
      "route-handler receipt does not describe a 200 JSON response",
    ),
    JSON.stringify(badRouteHandlerResponseReport.failures, null, 2),
  );
  assert.ok(
    badRouteHandlerResponseReport.failures.includes(
      "route-handler receipt is missing response header evidence",
    ),
    JSON.stringify(badRouteHandlerResponseReport.failures, null, 2),
  );

  const missingRouteHandlerBoundaryResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, DX_MISSING_ROUTE_HANDLER_ADAPTER_BOUNDARY: "1" },
    },
  );
  assert.equal(
    missingRouteHandlerBoundaryResult.status,
    1,
    missingRouteHandlerBoundaryResult.stdout + missingRouteHandlerBoundaryResult.stderr,
  );
  const missingRouteHandlerBoundaryReport = JSON.parse(missingRouteHandlerBoundaryResult.stdout);
  assert.equal(missingRouteHandlerBoundaryReport.build.sourceBuild.routeHandlerReceipt.hasAdapterBoundary, false);
  assert.equal(missingRouteHandlerBoundaryReport.build.sourceBuild.routeHandlerReceipt.adapterBoundaryCount, 0);
  assert.ok(
    missingRouteHandlerBoundaryReport.failures.includes(
      "route-handler receipt is missing adapter-boundary evidence",
    ),
    JSON.stringify(missingRouteHandlerBoundaryReport.failures, null, 2),
  );

  const productReceipt = path.join(tempRoot, "product-proof.json");
  const productResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      path.relative(repoRoot, fakeBinary),
      "--runner",
      process.execPath,
      "--json",
      "--require-product",
      "--receipt",
      productReceipt,
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
    },
  );

  assert.equal(productResult.status, 1, productResult.stdout + productResult.stderr);
  const productReport = JSON.parse(productResult.stdout);
  assert.equal(productReport.passed, true);
  assert.equal(productReport.productProofRequired, true);
  assert.equal(productReport.productProofPassed, false);
  assert.equal(productReport.proof.scope, "candidate-override");
  assert.equal(productReport.proof.productEligible, false);
  assert.deepEqual(JSON.parse(fs.readFileSync(productReceipt, "utf8")), productReport);
});

test("source build readiness and Zed handoff expose the installed smoke receipt path", () => {
  const readinessSource = fs.readFileSync(
    path.join(repoRoot, "dx-www", "src", "build", "source_engine", "readiness.rs"),
    "utf8",
  );
  const handoffSource = fs.readFileSync(
    path.join(repoRoot, "dx-www", "src", "build", "source_engine", "ecosystem_handoff.rs"),
    "utf8",
  );
  const sourceBuildTest = fs.readFileSync(
    path.join(repoRoot, "dx-www", "tests", "source_build_engine.rs"),
    "utf8",
  );

  for (const source of [readinessSource, handoffSource, sourceBuildTest]) {
    assert.match(source, /\.dx\/receipts\/build\/installed-binary-smoke-latest\.json/);
  }
  assert.match(readinessSource, /"installed_binary_smoke": relative_project_path/);
  assert.match(readinessSource, /"receipt": relative_project_path/);
  assert.match(handoffSource, /"installed_binary_smoke_receipt": relative_project_path/);
  assert.match(sourceBuildTest, /zed_handoff\["installed_binary_smoke_receipt"\]/);
  assert.match(sourceBuildTest, /build_readiness\["receipts"\]\["installed_binary_smoke"\]/);
});

test("installed smoke harness stays split into small professional modules", () => {
  const sourceFiles = [
    "tools/build/dx-build-installed-smoke.ts",
    "tools/build/installed-smoke/args.ts",
    "tools/build/installed-smoke/app-router-execution.ts",
    "tools/build/installed-smoke/artifact-hash.ts",
    "tools/build/installed-smoke/binary-provenance.ts",
    "tools/build/installed-smoke/blake3-16.ts",
    "tools/build/installed-smoke/build-receipt-failures.ts",
    "tools/build/installed-smoke/cli.ts",
    "tools/build/installed-smoke/command-summary.ts",
    "tools/build/installed-smoke/constants.ts",
    "tools/build/installed-smoke/fixture.ts",
    "tools/build/installed-smoke/fixture-paths.ts",
    "tools/build/installed-smoke/graph-consumer-snapshot.ts",
    "tools/build/installed-smoke/help-summary.ts",
    "tools/build/installed-smoke/human-binary-summary.ts",
    "tools/build/installed-smoke/human-command-diagnostics.ts",
    "tools/build/installed-smoke/human-node-modules-boundary.ts",
    "tools/build/installed-smoke/human-report.ts",
    "tools/build/installed-smoke/human-report-proof.ts",
    "tools/build/installed-smoke/io.ts",
    "tools/build/installed-smoke/manifest-summary.ts",
    "tools/build/installed-smoke/no-node-modules.ts",
    "tools/build/installed-smoke/manifest-server-data-artifact.ts",
    "tools/build/installed-smoke/manifest-server-data-route-failures.ts",
    "tools/build/installed-smoke/manifest-server-data-route-keys.ts",
    "tools/build/installed-smoke/manifest-server-data-request-props.ts",
    "tools/build/installed-smoke/manifest-server-data-route-manifest.ts",
    "tools/build/installed-smoke/manifest-server-data-route-summary.ts",
    "tools/build/installed-smoke/manifest-server-data-routes.ts",
    "tools/build/installed-smoke/manifest-server-data-source-routes.ts",
    "tools/build/installed-smoke/manifest-asset-output.ts",
    "tools/build/installed-smoke/manifest-output.ts",
    "tools/build/installed-smoke/manifest-output-paths.ts",
    "tools/build/installed-smoke/manifest-output-source-map.ts",
    "tools/build/installed-smoke/manifest-style-output.ts",
    "tools/build/installed-smoke/next-familiar-compatibility.ts",
    "tools/build/installed-smoke/output-proof-failures.ts",
    "tools/build/installed-smoke/proof.ts",
    "tools/build/installed-smoke/proof-css-assets.ts",
    "tools/build/installed-smoke/proof-css-style-output.ts",
    "tools/build/installed-smoke/proof-node-modules.ts",
    "tools/build/installed-smoke/proof-output-node-modules.ts",
    "tools/build/installed-smoke/proof-output-public-assets.ts",
    "tools/build/installed-smoke/proof-output-styles.ts",
    "tools/build/installed-smoke/proof-output-summary.ts",
    "tools/build/installed-smoke/proof-public-assets.ts",
    "tools/build/installed-smoke/readiness.ts",
    "tools/build/installed-smoke/receipt-writer.ts",
    "tools/build/installed-smoke/report.ts",
    "tools/build/installed-smoke/report-diagnostics.ts",
    "tools/build/installed-smoke/route-handler-graph-failures.ts",
    "tools/build/installed-smoke/route-handler-graph.ts",
    "tools/build/installed-smoke/route-handler-manifest-failures.ts",
    "tools/build/installed-smoke/route-handler-manifest.ts",
    "tools/build/installed-smoke/route-handler-receipt-failures.ts",
    "tools/build/installed-smoke/route-handler-receipt-summary.ts",
    "tools/build/installed-smoke/route-handler-receipts.ts",
    "tools/build/installed-smoke/route-handler-requirements.ts",
    "tools/build/installed-smoke/route-handler-unexpected-evidence.ts",
    "tools/build/installed-smoke/route-output.ts",
    "tools/build/installed-smoke/runner.ts",
    "tools/build/installed-smoke/server-artifacts.ts",
    "tools/build/installed-smoke/source-freshness.ts",
    "tools/build/installed-smoke/source-build-app-router.ts",
    "tools/build/installed-smoke/source-build-fixture.ts",
    "tools/build/installed-smoke/source-build.ts",
    "tools/build/installed-smoke/source-build-failures.ts",
    "tools/build/installed-smoke/source-module-resolver.ts",
  ];

  const installedSmokeSource = sourceFiles
    .map((relative) => fs.readFileSync(path.join(repoRoot, relative), "utf8"))
    .join("\n");
  for (const removedVocabulary of [
    new RegExp("next_familiar_" + "evidence"),
    new RegExp("next-familiar-" + "evidence"),
    new RegExp("nextFamiliar" + "Evidence"),
    new RegExp("next_" + "parity_evidence"),
    new RegExp("full_nextjs_runtime_" + "parity"),
    new RegExp("full_nextjs_route_handler_" + "parity"),
    new RegExp("claimsFullNext" + "Parity"),
    new RegExp("claimsFullNextRuntime" + "Parity"),
  ]) {
    assert.doesNotMatch(installedSmokeSource, removedVocabulary);
    assert.doesNotMatch(fs.readFileSync(__filename, "utf8"), removedVocabulary);
  }

  for (const relative of sourceFiles) {
    const source = fs.readFileSync(path.join(repoRoot, relative), "utf8");
    assert.ok(source.split(/\r?\n/).length <= 180, `${relative} is too large`);
  }
});

test("installed smoke route-handler counts come from the fixture contract", () => {
  const source = fs.readFileSync(
    path.join(repoRoot, "tools", "build", "installed-smoke", "source-build-failures.ts"),
    "utf8",
  );

  assert.match(source, /EXPECTED_ROUTE_HANDLER_COUNT/);
  assert.doesNotMatch(source, /routeHandlers !== 2/);
});

test("installed smoke source-build summary keeps artifact ownership split", () => {
  const sourceBuild = fs.readFileSync(
    path.join(repoRoot, "tools", "build", "installed-smoke", "source-build.ts"),
    "utf8",
  );
  const appRouterSummary = fs.readFileSync(
    path.join(repoRoot, "tools", "build", "installed-smoke", "source-build-app-router.ts"),
    "utf8",
  );
  const fixtureSummary = fs.readFileSync(
    path.join(repoRoot, "tools", "build", "installed-smoke", "source-build-fixture.ts"),
    "utf8",
  );

  assert.ok(
    sourceBuild.split(/\r?\n/).length <= 135,
    "source-build.ts should orchestrate artifact summaries instead of owning every summary helper",
  );
  assert.doesNotMatch(sourceBuild, /function summarizeServerData/);
  assert.doesNotMatch(sourceBuild, /function summarizeFixture/);
  assert.match(appRouterSummary, /function summarizeAppRouterOutputs/);
  assert.match(appRouterSummary, /function summarizeServerData/);
  assert.match(fixtureSummary, /function summarizeFixture/);
});

function sha256File(filePath) {
  return crypto.createHash("sha256").update(fs.readFileSync(filePath)).digest("hex");
}
