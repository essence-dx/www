const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const outputRoot = path.resolve(__dirname, "../dx-mobile-app/.dx/www/output");
const nativeEntrypointPath = path.join(outputRoot, "index.html");
const appHtmlPath = path.join(outputRoot, "app/index.html");
const routeHtmlPath = path.join(outputRoot, "source-routes/root/index.html");
const manifestPath = path.join(outputRoot, "manifest.json");
const sourceManifestPath = path.join(outputRoot, "source-build-manifest.json");
const sourceReceiptPath = path.join(outputRoot, "source-build-receipt.json");
const sourceRuntimePath = path.resolve(__dirname, "../dx-mobile-app/public/mobile-companion-runtime.js");
const readinessPath = path.resolve(__dirname, "../dx-mobile-app/server/mobile-companion/readiness.ts");
const runtimePath = path.resolve(
  __dirname,
  "../dx-mobile-app/.dx/www/output/public/mobile-companion-runtime.js",
);

function readText(filePath) {
  assert.ok(fs.existsSync(filePath), `run \`dx www build\` before this contract: ${filePath}`);
  return fs.readFileSync(filePath, "utf8");
}

function readJson(filePath) {
  return JSON.parse(readText(filePath));
}

function countMatches(value, pattern) {
  return (value.match(pattern) ?? []).length;
}

function assertHtmlContract(html, artifactName) {
  const requiredFragments = [
    'data-dx-mobile-app="companion"',
    'data-dx-www-framework="app-router"',
    'data-dx-native-target="tauri-webview"',
    'data-dx-native-bridge="dx-native-mobile-companion"',
    'data-dx-native-bundle="com.essencefromexistence.dx.mobile"',
    'data-dx-pairing-contract="dx.mobile.pairing"',
    'data-dx-chat-contract="dx.mobile.chat_handoff"',
    'data-dx-chat-send="/ws/chat"',
    'data-dx-chat-list-endpoint="/api/sessions"',
    'data-dx-session-messages-endpoint="/api/sessions/:session_key/messages"',
    'data-dx-native-command="dx_mobile_scan_pairing_qr"',
    'data-dx-native-command="dx_mobile_clear_device_credential"',
    'name="agent_alias"',
    'src="/mobile-companion-runtime.js"',
  ];

  for (const fragment of requiredFragments) {
    assert.ok(html.includes(fragment), `${artifactName} is missing ${fragment}`);
  }

  assert.equal(countMatches(html, /<textarea\b/g), 2, `${artifactName} should render two textareas`);
  assert.equal(countMatches(html, /<\/textarea>/g), 2, `${artifactName} should close both textareas`);
  assert.equal(
    countMatches(html, /data-dx-icon="status:check"/g),
    6,
    `${artifactName} should carry six DX status check icons`,
  );
  assert.equal(
    countMatches(html, /<span\b(?=[^>]*data-dx-icon="status:check")[^>]*><\/span>/g),
    6,
    `${artifactName} should render closed DX status check icon spans`,
  );
  assert.doesNotMatch(html, /mobile-device-frame|phone-mock|iphone-frame|bezel|notch/i);
}

test("DX Mobile production manifests describe a source-built native companion", () => {
  const manifest = readJson(manifestPath);
  const sourceManifest = readJson(sourceManifestPath);

  readText(sourceReceiptPath);

  assert.equal(manifest.source_build_manifest_emitted, true);
  assert.equal(manifest.source_build_receipt_emitted, true);
  assert.equal(manifest.source_build_routes, 1);
  assert.equal(manifest.source_build_route_outputs, 1);
  assert.equal(manifest.node_modules_required, false);

  assert.equal(sourceManifest.schema, "dx.www.sourceBuildManifest");
  assert.deepEqual(
    sourceManifest.routes.map((route) => route.route),
    ["/"],
  );
  assert.deepEqual(
    sourceManifest.routes.map((route) => route.path),
    ["app/page.tsx"],
  );

  const routeOutput = sourceManifest.route_outputs.find((route) => route.route === "/");
  assert.ok(routeOutput, "source manifest should include the root route output");
  assert.equal(routeOutput.source_path, "app/page.tsx");
  assert.equal(routeOutput.html_output, ".dx/www/output/source-routes/root/index.html");
  assert.equal(routeOutput.server_data_output, ".dx/www/output/source-routes/root/server-data.json");
  assert.equal(routeOutput.node_modules_required, false);

  const runtimeAsset = sourceManifest.assets.find(
    (asset) => asset.path === "public/mobile-companion-runtime.js",
  );
  assert.ok(runtimeAsset, "source manifest should include the mobile companion runtime asset");
  assert.ok(
    runtimeAsset.referenced_by_routes.some(
      (reference) => reference.route === "/" && reference.source_path === "app/page.tsx",
    ),
    "runtime asset should be referenced by the root app route",
  );
});

test("DX Mobile production artifacts contain the real companion screen", () => {
  const routeHtml = readText(routeHtmlPath);
  const appHtml = readText(appHtmlPath);
  const nativeEntrypointHtml = readText(nativeEntrypointPath);

  assert.equal(
    nativeEntrypointHtml,
    appHtml,
    "native root entrypoint should mirror app/index.html for Tauri frontendDist loading",
  );
  assertHtmlContract(routeHtml, "canonical source route artifact");
  assertHtmlContract(appHtml, "app mirror artifact");
  assertHtmlContract(nativeEntrypointHtml, "native root entrypoint artifact");
});

test("DX Mobile production artifact carries the current companion runtime", () => {
  assert.ok(fs.existsSync(runtimePath), "run `dx www build` before this contract");
  const runtime = fs.readFileSync(runtimePath, "utf8");
  assert.match(runtime, /deviceRegistryPairEndpoint = "\/api\/pair"/);
  assert.match(runtime, /pairingState/);
  assert.match(runtime, /scannerState/);
  assert.match(runtime, /socketState/);
  assert.match(runtime, /normalizeGatewayBaseUrl/);
  assert.match(runtime, /gatewayEndpoint/);
  assert.match(runtime, /gatewayWebSocketEndpoint/);
  assert.doesNotMatch(runtime, /parsed\.pathname = ""/);
  assert.match(runtime, /connected-live-chat/);
  assert.doesNotMatch(runtime, /X-Pairing-Code/);
});

test("DX Mobile readiness does not treat stale native WWW output as current proof", () => {
  const sourceRuntime = readText(sourceRuntimePath);
  const packagedRuntime = readText(runtimePath);
  const readiness = readText(readinessPath);

  if (sourceRuntime !== packagedRuntime) {
    assert.match(readiness, /nativeWwwOutputCurrent: false/);
    assert.match(readiness, /nativeWwwOutputStatus: "stale-build-required"/);
    assert.match(readiness, /androidDebugApkSourceCurrent: false/);
    assert.match(readiness, /androidDebugApkStatus: "stale-source-build-required"/);
    assert.match(readiness, /nativeWwwOutputRefreshCommand: nativeArtifacts\.refreshCommand/);
    assert.doesNotMatch(readiness, /nativeWwwOutputCurrent: true/);
  }
});
