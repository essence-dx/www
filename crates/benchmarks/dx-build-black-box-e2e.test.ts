const assert = require("node:assert/strict");
const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const { inspectBinary } = require("../tools/build/installed-smoke/binary-provenance.ts");
const {
  discoverDefaultSourceFreshnessPaths,
  inspectBinarySourceFreshness,
} = require("../tools/build/installed-smoke/source-freshness.ts");

const repoRoot = path.resolve(__dirname, "..");
const dxWwwBinary = path.join(
  repoRoot,
  "target",
  "debug",
  process.platform === "win32" ? "dx-www.exe" : "dx-www",
);

function writeFixtureFile(projectRoot, relativePath, content) {
  const filePath = path.join(projectRoot, relativePath);
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, content);
}

function readJson(projectRoot, relativePath) {
  return JSON.parse(fs.readFileSync(path.join(projectRoot, relativePath), "utf8"));
}

function assertBuildFile(projectRoot, relativePath) {
  assert.ok(fs.existsSync(path.join(projectRoot, relativePath)), `${relativePath} should exist`);
}

function assertNoBuildFile(projectRoot, relativePath) {
  assert.equal(
    fs.existsSync(path.join(projectRoot, relativePath)),
    false,
    `${relativePath} should not exist`,
  );
}

function sortedRoutes(routes) {
  return routes.map((route) => route.path).sort();
}

function assertFreshDxWwwBinary() {
  const freshness = inspectBinarySourceFreshness(inspectBinary(dxWwwBinary), {
    repoRoot,
    sourcePaths: blackBoxBuildFreshnessPaths(),
  });
  assert.equal(
    freshness.fresh,
    true,
    [
      "target/debug/dx-www is stale for this black-box dx build proof",
      `binaryModifiedMs=${freshness.binaryModifiedMs}`,
      `newestSourcePath=${freshness.newestSourcePath}`,
      `newestSourceModifiedMs=${freshness.newestSourceModifiedMs}`,
      "run: cargo build -p dx-www --no-default-features --features cli --bin dx-www -j 1",
    ].join("\n"),
  );
}

function blackBoxBuildFreshnessPaths() {
  return discoverDefaultSourceFreshnessPaths(repoRoot).filter(
    (sourcePath) =>
      !sourcePath.endsWith("/tests.rs") &&
      !sourcePath.includes("/tests/") &&
      !sourcePath.includes("/src/cli/tests/"),
  );
}

function runDxBuild(projectRoot) {
  const result = spawnSync(dxWwwBinary, ["build"], {
    cwd: projectRoot,
    encoding: "utf8",
    timeout: 60000,
  });

  assert.equal(result.status, 0, result.stderr || result.stdout);
}

function findNodeModulesDirs(root) {
  const found = [];
  const stack = [root];
  while (stack.length > 0) {
    const current = stack.pop();
    for (const entry of fs.readdirSync(current, { withFileTypes: true })) {
      const fullPath = path.join(current, entry.name);
      if (!entry.isDirectory()) {
        continue;
      }
      if (entry.name === "node_modules") {
        found.push(path.relative(root, fullPath).replace(/\\/g, "/"));
        continue;
      }
      stack.push(fullPath);
    }
  }
  return found.sort();
}

function createTinyBuildFixture() {
  const projectRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-black-box-e2e-"));
  writeFixtureFile(
    projectRoot,
    "dx",
    `project.name="black-box-e2e"
build.output_dir=".dx/build"
tooling.dx_style.generated_css="styles/generated.css"
`,
  );
  writeFixtureFile(
    projectRoot,
    "app/layout.tsx",
    `import "../styles/app.css";

export default function RootLayout({ children }) {
  return <html><body>{children}</body></html>;
}
`,
  );
  writeFixtureFile(
    projectRoot,
    "app/page.tsx",
    `import { MetricCard } from "../components/MetricCard";
import { loadHomeMetrics } from "@/server/loaders";

export default async function Page() {
  const metrics = await loadHomeMetrics();
  return <main><MetricCard label="routes" value={metrics.routes} /></main>;
}
`,
  );
  writeFixtureFile(
    projectRoot,
    "app/dashboard/[team]/page.tsx",
    `export default async function TeamPage({ params, searchParams }) {
  const team = params.team;
  const tab = searchParams?.tab ?? "overview";
  return <main>{team}:{tab}</main>;
}
`,
  );
  writeFixtureFile(
    projectRoot,
    "app/api/health/route.ts",
    `export function GET() {
  return Response.json({ ok: true, service: "black-box-e2e" }, { status: 200 });
}
`,
  );
  writeFixtureFile(
    projectRoot,
    "components/MetricCard.tsx",
    `export function MetricCard({ label, value }) {
  return <p>{label}: {value}</p>;
}
`,
  );
  writeFixtureFile(
    projectRoot,
    "server/loaders.ts",
    `export async function loadHomeMetrics() {
  return { routes: 2, runtime: "dx-www" };
}
`,
  );
  writeFixtureFile(projectRoot, "styles/app.css", "main { display: grid; }\n");
  writeFixtureFile(projectRoot, "public/mark.svg", '<svg viewBox="0 0 1 1"></svg>\n');
  return projectRoot;
}

test("real dx build emits black-box app artifacts for a tiny source-owned app", (t) => {
  if (!fs.existsSync(dxWwwBinary)) {
    t.skip("target/debug/dx-www executable is required for this black-box build proof");
    return;
  }
  assertFreshDxWwwBinary();

  const projectRoot = createTinyBuildFixture();
  try {
    runDxBuild(projectRoot);

    const manifest = readJson(projectRoot, ".dx/build/manifest.json");
    assert.equal(manifest.app_routes_compiled, 2);
    assert.equal(manifest.server_data_routes_compiled, 2);
    assert.equal(manifest.server_data_route_manifest.source_build_routes, 2);
    assert.equal(manifest.server_data_route_manifest.manifest_routes, 2);
    assert.deepEqual(manifest.server_data_route_manifest.route_param_keys, ["team"]);
    assert.deepEqual(manifest.server_data_route_manifest.search_param_keys, ["tab"]);
    assert.equal(manifest.node_modules_required, false);

    const sourceBuildManifest = readJson(projectRoot, ".dx/build/source-build-manifest.json");
    assert.equal(sourceBuildManifest.route_outputs.length, 2);
    assert.ok(
      sourceBuildManifest.route_outputs.some(
        (route) =>
          route.route === "/dashboard/:team" &&
          route.source_path === "app/dashboard/[team]/page.tsx" &&
          route.server_data_output === ".dx/build/source-routes/dashboard--team/server-data.json",
      ),
      JSON.stringify(sourceBuildManifest.route_outputs, null, 2),
    );

    const styleOutput = sourceBuildManifest.styles.find((style) => style.path === "styles/app.css");
    assert.equal(styleOutput.output, ".dx/build/styles/app.css");
    assert.equal(styleOutput.source_map_output, ".dx/build/styles/app.css.map");
    assert.equal(styleOutput.source_map_linked, true);
    assert.equal(styleOutput.node_modules_required, false);

    const publicAsset = sourceBuildManifest.assets.find((asset) => asset.path === "public/mark.svg");
    assert.match(publicAsset.output, /^\.dx\/build\/public\/mark-[0-9a-f]+\.svg$/);
    assert.equal(publicAsset.source_owned_contract, true);
    assert.equal(publicAsset.node_modules_required, false);

    const rootServerData = readJson(projectRoot, ".dx/build/source-routes/root/server-data.json");
    const dynamicServerData = readJson(
      projectRoot,
      ".dx/build/source-routes/dashboard--team/server-data.json",
    );
    assert.equal(rootServerData.route, "/");
    assert.equal(rootServerData.entry_count, 1);
    assert.equal(rootServerData.node_modules_required, false);
    assert.equal(dynamicServerData.route, "/dashboard/:team");
    assert.deepEqual(dynamicServerData.request.route_params, { team: "sample-team" });
    assert.deepEqual(dynamicServerData.request.search_params, { tab: "sample-tab" });

    assertBuildFile(projectRoot, ".dx/build/source-routes/root/index.html");
    assertBuildFile(projectRoot, ".dx/build/source-routes/root/index.dxpk");
    assertBuildFile(projectRoot, ".dx/build/source-routes/dashboard--team/index.html");
    assertBuildFile(projectRoot, ".dx/build/styles/app.css");
    assertBuildFile(projectRoot, ".dx/build/styles/app.css.map");
    assertBuildFile(projectRoot, publicAsset.output);

    const routeHandlerReceipts = readJson(projectRoot, ".dx/build/route-handler-receipts.json");
    assert.ok(
      routeHandlerReceipts.receipts.some(
        (receipt) =>
          receipt.method === "GET" &&
          receipt.request_path === "/api/health" &&
          receipt.node_modules_required === false,
      ),
      JSON.stringify(routeHandlerReceipts, null, 2),
    );
    const deployAdapter = readJson(projectRoot, ".dx/build/deploy-adapter.json");
    assert.deepEqual(sortedRoutes(deployAdapter.routes), ["/", "/dashboard/[team]"]);
    assert.ok(
      deployAdapter.route_handlers.some(
        (handler) =>
          handler.path === "/api/health" &&
          handler.source_path === "app/api/health/route.ts" &&
          handler.node_modules_required === false,
      ),
      JSON.stringify(deployAdapter.route_handlers, null, 2),
    );
    assert.deepEqual(findNodeModulesDirs(projectRoot), []);

    writeFixtureFile(projectRoot, ".dx/build/stale-sentinel.txt", "must be removed by next build\n");
    fs.rmSync(path.join(projectRoot, "app/dashboard"), { recursive: true, force: true });
    runDxBuild(projectRoot);

    const rebuiltManifest = readJson(projectRoot, ".dx/build/manifest.json");
    assert.equal(rebuiltManifest.app_routes_compiled, 1);
    assert.equal(rebuiltManifest.server_data_routes_compiled, 1);
    assert.equal(rebuiltManifest.server_data_route_manifest.source_build_routes, 1);
    assert.equal(rebuiltManifest.server_data_route_manifest.manifest_routes, 1);
    assert.deepEqual(rebuiltManifest.server_data_route_manifest.route_param_keys, []);
    assert.deepEqual(rebuiltManifest.server_data_route_manifest.search_param_keys, []);

    const rebuiltSourceBuildManifest = readJson(projectRoot, ".dx/build/source-build-manifest.json");
    assert.deepEqual(
      rebuiltSourceBuildManifest.route_outputs.map((route) => route.route),
      ["/"],
    );
    assertNoBuildFile(projectRoot, ".dx/build/source-routes/dashboard--team/index.html");
    assertNoBuildFile(projectRoot, ".dx/build/source-routes/dashboard--team/server-data.json");
    assertNoBuildFile(projectRoot, ".dx/build/app/dashboard/:team/index.html");
    assertNoBuildFile(projectRoot, ".dx/build/stale-sentinel.txt");
    const rebuiltDeployAdapter = readJson(projectRoot, ".dx/build/deploy-adapter.json");
    assert.deepEqual(sortedRoutes(rebuiltDeployAdapter.routes), ["/"]);
    assert.ok(
      rebuiltDeployAdapter.immutable_assets.every(
        (asset) => !asset.path.includes("dashboard"),
      ),
      JSON.stringify(rebuiltDeployAdapter.immutable_assets, null, 2),
    );
    assert.deepEqual(findNodeModulesDirs(projectRoot), []);
  } finally {
    fs.rmSync(projectRoot, { recursive: true, force: true });
  }
});
