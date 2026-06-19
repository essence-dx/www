import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function read(relativePath) {
  const fullPath = path.join(repoRoot, relativePath);
  assert.ok(fs.existsSync(fullPath), `expected ${relativePath} to exist`);
  return fs.readFileSync(fullPath, "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function readDevtoolsRuntimeSource() {
  return walkFiles("dx-www/src/cli/devtools/assets/runtime")
    .filter((file) => file.endsWith(".ts"))
    .map((file) => read(file))
    .join("\n");
}

function walkFiles(relativeDir) {
  const root = path.join(repoRoot, relativeDir);
  assert.ok(fs.existsSync(root), `expected ${relativeDir} to exist`);
  const files = [];
  const stack = [root];

  while (stack.length > 0) {
    const current = stack.pop();
    for (const entry of fs.readdirSync(current, { withFileTypes: true })) {
      const fullPath = path.join(current, entry.name);
      if (entry.isDirectory()) {
        stack.push(fullPath);
        continue;
      }
      if (entry.isFile()) {
        files.push(path.relative(repoRoot, fullPath).replaceAll(path.sep, "/"));
      }
    }
  }

  return files.sort();
}

function readRustSources(relativeDir) {
  const root = path.join(repoRoot, relativeDir);
  assert.ok(fs.existsSync(root), `expected ${relativeDir} to exist`);
  const sources = [];
  const stack = [root];

  while (stack.length > 0) {
    const current = stack.pop();
    for (const entry of fs.readdirSync(current, { withFileTypes: true })) {
      const fullPath = path.join(current, entry.name);
      if (entry.isDirectory()) {
        stack.push(fullPath);
        continue;
      }
      if (!entry.isFile() || !entry.name.endsWith(".rs")) {
        continue;
      }
      sources.push({
        relativePath: path.relative(repoRoot, fullPath).replaceAll(path.sep, "/"),
        source: fs.readFileSync(fullPath, "utf8"),
      });
    }
  }

  return sources.sort((left, right) => left.relativePath.localeCompare(right.relativePath));
}

function combinedSource(sources) {
  return sources
    .map(({ relativePath, source }) => `\n// FILE: ${relativePath}\n${source}`)
    .join("\n");
}

function extractBalancedBlock(source, signaturePattern, label) {
  const match = signaturePattern.exec(source);
  assert.ok(match, `expected to find ${label}`);
  const start = match.index;
  const openBrace = source.indexOf("{", start);
  assert.notEqual(openBrace, -1, `expected ${label} to have a body`);

  let depth = 0;
  let inLineComment = false;
  let inBlockComment = false;
  let inString = false;
  let escaped = false;

  for (let index = openBrace; index < source.length; index += 1) {
    const current = source[index];
    const next = source[index + 1];

    if (inLineComment) {
      if (current === "\n") inLineComment = false;
      continue;
    }
    if (inBlockComment) {
      if (current === "*" && next === "/") {
        inBlockComment = false;
        index += 1;
      }
      continue;
    }
    if (inString) {
      if (escaped) {
        escaped = false;
      } else if (current === "\\") {
        escaped = true;
      } else if (current === '"') {
        inString = false;
      }
      continue;
    }

    if (current === "/" && next === "/") {
      inLineComment = true;
      index += 1;
      continue;
    }
    if (current === "/" && next === "*") {
      inBlockComment = true;
      index += 1;
      continue;
    }
    if (current === '"') {
      inString = true;
      continue;
    }
    if (current === "{") {
      depth += 1;
    } else if (current === "}") {
      depth -= 1;
      if (depth === 0) {
        return source.slice(start, index + 1);
      }
    }
  }

  assert.fail(`expected to find the end of ${label}`);
}

function snippetsAround(source, pattern, radius = 2400) {
  const matches = [];
  const regex = pattern instanceof RegExp ? new RegExp(pattern.source, pattern.flags.includes("g") ? pattern.flags : `${pattern.flags}g`) : null;

  if (regex) {
    let match;
    while ((match = regex.exec(source))) {
      matches.push(source.slice(Math.max(0, match.index - radius), Math.min(source.length, match.index + radius)));
      if (match.index === regex.lastIndex) regex.lastIndex += 1;
    }
  } else {
    let index = source.indexOf(pattern);
    while (index !== -1) {
      matches.push(source.slice(Math.max(0, index - radius), Math.min(source.length, index + radius)));
      index = source.indexOf(pattern, index + pattern.length);
    }
  }

  return matches.join("\n");
}

function cssBlock(source, selector) {
  const escapedSelector = selector.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const match = new RegExp(`${escapedSelector}\\s*\\{([\\s\\S]*?)\\}`).exec(source);
  assert.ok(match, `expected ${selector} CSS block`);
  return match[1];
}

function zIndexValue(block, label) {
  const match = /z-index:\s*(\d+)/.exec(block);
  assert.ok(match, `${label} should declare z-index`);
  return Number(match[1]);
}

function assertNoSourceMutation(source, label) {
  assertSourceDoesNotMatch(
    source,
    /\bstd::fs::(?:write|rename|copy|remove_file|remove_dir|remove_dir_all|create_dir|create_dir_all|set_permissions)\b|\bFile::create\b|\bOpenOptions::new\b|\.write_all\(|\.write\(/,
    `${label} must not perform filesystem writes`,
  );
}

function assertSourceMatches(source, pattern, message) {
  assert.ok(pattern.test(source), message);
}

function assertSourceDoesNotMatch(source, pattern, message) {
  assert.ok(!pattern.test(source), message);
}

const rustSources = readRustSources("dx-www/src");
const allDxWwwRust = combinedSource(rustSources);
const cliMod = read("dx-www/src/cli/mod.rs");
const cliCoreImpl = read("dx-www/src/cli/mod_parts/cli_core_impl.rs");
const cmdBuildBody = extractBalancedBlock(
  cliCoreImpl,
  /pub\s+fn\s+cmd_build\s*\(&self\)\s*->\s*DxResult<\(\)>/,
  "DxCli::cmd_build",
);
const devtoolsAssetsRust = read("dx-www/src/cli/devtools/assets.rs");
const devtoolsRuntime = readDevtoolsRuntimeSource();
const devtoolsCss = read("dx-www/src/cli/devtools/assets/devtools.css");

test("DX Devtools guard and tooling stay TypeScript-only", () => {
  assert.ok(
    fs.existsSync(path.join(repoRoot, "benchmarks/dx-devtools-framework-integration.test.ts")),
    "Devtools framework guard should live as a .ts test",
  );
  assert.ok(
    fs.existsSync(path.join(repoRoot, "dx-www/src/cli/devtools/assets/runtime.ts")),
    "Devtools browser runtime should live as a source-owned runtime.ts asset",
  );
  assert.ok(
    !fs.existsSync(path.join(repoRoot, "dx-www/src/cli/devtools/assets/runtime.js")),
    "Devtools browser runtime should not keep an authored runtime.js source file",
  );
  assert.ok(
    fs.existsSync(path.join(repoRoot, "dx-www/src/cli/devtools/assets/devtools.css")),
    "Devtools CSS should live as a source-owned devtools.css asset",
  );
  assert.ok(
    !fs.existsSync(path.join(repoRoot, "benchmarks/dx-devtools-framework-integration.test.cjs")),
    "Devtools framework guard should not keep the old .cjs test file",
  );
  assert.ok(
    fs.existsSync(path.join(repoRoot, "tools/devtools/generate-mdn-css-data.ts")),
    "Devtools MDN generator should be a TypeScript tool",
  );
  assert.ok(
    !fs.existsSync(path.join(repoRoot, "tools/devtools/generate-mdn-css-data.mjs")),
    "Devtools MDN generator should not keep the old .mjs file",
  );
  const generator = read("tools/devtools/generate-mdn-css-data.ts");
  assertSourceDoesNotMatch(generator, /require\(|module\.exports/, "Devtools generator should use ESM/TS, not CommonJS");
  const devtoolsLegacyTests = fs
    .readdirSync(path.join(repoRoot, "benchmarks"))
    .filter((name) => /devtools.*\.test\.(?:cjs|mjs)$/i.test(name));
  assert.deepEqual(devtoolsLegacyTests, [], "Devtools benchmark guards should stay TypeScript-only; no .cjs/.mjs drift");
  const devtoolsBenchmarkTests = fs
    .readdirSync(path.join(repoRoot, "benchmarks"))
    .filter((name) => /devtools/i.test(name) && /\.test\./i.test(name));
  assert.deepEqual(
    devtoolsBenchmarkTests.filter((name) => !name.endsWith(".test.ts")),
    [],
    "Devtools benchmark guards should stay .test.ts only",
  );
  const devtoolsToolFiles = walkFiles("tools/devtools");
  assert.deepEqual(
    devtoolsToolFiles.filter((file) => /\.(?:cjs|mjs|js)$/i.test(file)),
    [],
    "tools/devtools should remain TypeScript-only; no JS/CJS/MJS helper drift",
  );
  assert.deepEqual(
    devtoolsToolFiles.filter((file) => !file.endsWith(".ts")),
    [],
    "tools/devtools should contain only .ts source files",
  );
  const devtoolsAssetFiles = walkFiles("dx-www/src/cli/devtools/assets");
  assert.deepEqual(
    devtoolsAssetFiles.filter((file) => /\.(?:cjs|mjs|js)$/i.test(file)),
    [],
    "dx devtools authored browser assets should not use JS/CJS/MJS source files",
  );
  assert.deepEqual(
    devtoolsAssetFiles.filter((file) => /\/runtime\.(?:cjs|mjs|js)$/i.test(file)),
    [],
    "dx devtools authored runtime source should stay runtime.ts, not runtime.js/cjs/mjs",
  );
});

test("dx dev injects DX Devtools and --no-devtools disables the injection", () => {
  const devOptions = read("dx-www/src/cli/dev_options.rs");
  const devCommand = read("dx-www/src/cli/dev_command.rs");
  const devHttp = read("dx-www/src/cli/dev_http.rs");
  const devtoolsMod = read("dx-www/src/cli/devtools/mod.rs");
  const devtoolsProtocol = read("dx-www/src/cli/devtools/protocol.rs");
  const axumServer = read("dx-www/src/dev/axum_server.rs");
  const devSurface = [devCommand, devHttp, axumServer].join("\n");
  const devtoolsProtocolResponse = extractBalancedBlock(
    devtoolsProtocol,
    /pub\(super\)\s+fn\s+devtools_protocol_response\s*\(/,
    "devtools_protocol_response",
  );

  assertSourceMatches(
    devOptions,
    /devtools:\s*bool/,
    "dx dev options should carry an explicit devtools flag",
  );
  assertSourceMatches(
    devOptions,
    /"--no-devtools"[\s\S]*devtools\s*=\s*false/,
    "--no-devtools should explicitly disable devtools injection",
  );
  assertSourceMatches(devCommand, /options\.devtools/, "cmd_dev should branch on parsed devtools state");
  assertSourceMatches(
    devSurface,
    /data-dx-devtools|DX_DEVTOOLS|\/_dx\/devtools|dx-devtools/i,
    "dx dev should inject or serve the DX Devtools framework surface",
  );
  assertSourceMatches(
    devSurface,
    /if\s+options\.devtools[\s\S]{0,1200}(inject|with|append|serve)[\s\S]{0,400}devtools/i,
    "devtools injection should be guarded by the parsed devtools flag",
  );
  assertSourceMatches(devtoolsMod, /&request\.headers/, "devtools protocol should receive request headers for write-capable safety checks");
  assertSourceMatches(devtoolsProtocolResponse, /request_headers:\s*&BTreeMap<String,\s*String>/, "devtools protocol should accept request headers");
  assertSourceMatches(
    devtoolsProtocolResponse,
    /STYLE_APPLY_ENDPOINT[\s\S]*!style_apply_local_write_allowed\(request_headers\)[\s\S]*403/,
    "style-apply should reject non-local write requests before source mutation",
  );
});

test("cmd_build remains free of devtools references", () => {
  assertSourceDoesNotMatch(
    cmdBuildBody,
    /devtools|\/_dx\/devtools|style-preview|style-apply/i,
    "production build must not reference devtools injection, routes, or style edit endpoints",
  );
});

test("DX Devtools endpoints are owned by dev-server code only", () => {
  const devRust = combinedSource(
    rustSources.filter(
      ({ relativePath }) =>
        relativePath.startsWith("dx-www/src/dev/") ||
        relativePath.startsWith("dx-www/src/cli/devtools/") ||
        relativePath === "dx-www/src/cli/dev_command.rs" ||
        relativePath === "dx-www/src/cli/dev_http.rs" ||
        relativePath === "dx-www/src/cli/dev_options.rs" ||
        relativePath === "dx-www/src/cli/dev_response.rs",
    ),
  );

  assertSourceMatches(devRust, /\/_dx\/devtools/, "dev-server code should expose a devtools route");
  assertSourceMatches(devRust, /style-preview/, "dev-server code should expose style-preview");
  assertSourceMatches(devRust, /style-apply/, "dev-server code should expose style-apply");
  assertSourceDoesNotMatch(
    cmdBuildBody,
    /\/_dx\/devtools|style-preview|style-apply/i,
    "devtools endpoints must not leak into cmd_build",
  );
});

test("DX Devtools injected runtime is source-owned and dependency-free", () => {
  const assets = devtoolsAssetsRust;
  const runtime = devtoolsRuntime;
  const cssSource = devtoolsCss;

  assertSourceMatches(assets, /DX_DEVTOOLS_RUNTIME_PATH:\s*&str\s*=\s*"\/_dx\/devtools\/runtime\.js"/, "runtime asset should be served from the DX devtools namespace");
  assertSourceMatches(assets, /DX_DEVTOOLS_CSS_PATH:\s*&str\s*=\s*"\/_dx\/devtools\/devtools\.css"/, "CSS asset should be served from the DX devtools namespace");
  assertSourceMatches(assets, /const\s+RUNTIME_JS:\s*&str\s*=\s*concat!\(/, "Rust assets should assemble the split TypeScript runtime asset");
  assertSourceMatches(assets, /include_str!\("assets\/runtime\/part-01-boot\.ts"\)/, "Rust assets should include the runtime boot fragment");
  assertSourceMatches(assets, /include_str!\("assets\/runtime\/part-05-events\.ts"\)/, "Rust assets should include the runtime event fragment");
  assertSourceDoesNotMatch(assets, /include_str!\("assets\/runtime\.js"\)/, "Rust assets should not include an authored runtime.js file");
  assertSourceMatches(assets, /include_str!\("assets\/devtools\.css"\)/, "Rust assets should include the source-owned CSS asset");
  assertSourceDoesNotMatch(assets, /const\s+RUNTIME_JS:\s*&str\s*=\s*r###"/, "Rust assets should not inline the runtime as a mega raw string");
  assertSourceDoesNotMatch(assets, /const\s+DEVTOOLS_CSS:\s*&str\s*=\s*r###"/, "Rust assets should not inline CSS as a mega raw string");
  assertSourceMatches(runtime, /data-dx-devtools-puck/, "runtime should include the draggable puck identity");
  assertSourceDoesNotMatch(runtime, /data-dx-devtools-edge/, "runtime should not include edge-hover sensors");
  assertSourceMatches(runtime, /data-dx-devtools-overlay/, "runtime should include the box model overlay");
  assertSourceMatches(runtime, /data-dx-devtools-close/, "runtime should include delegated close button markers");
  assertSourceMatches(runtime, /Inspect element/, "runtime should expose a named inspect element action");
  assertSourceMatches(runtime, /CSS_GROUPS/, "runtime should group computed CSS values for inspector panels");
  assertSourceMatches(runtime, /SOLID_PRESETS/, "runtime should include popular solid color presets");
  assertSourceMatches(runtime, /GRADIENT_PRESETS/, "runtime should include popular linear, radial, and mesh gradients");
  assertSourceMatches(cssSource, /JetBrains Mono/, "runtime should default Devtools typography to JetBrains Mono");
  assertSourceMatches(
    cssSource,
    /var\(--(?:ds|geist)-[a-z0-9-]+(?:,|\))/i,
    "DX Devtools CSS should consume Vercel/Geist theme variables instead of hardcoding the whole palette",
  );
  assertSourceDoesNotMatch(
    runtime,
    /\b(?:npm|node_modules|package\.json|ReactDOM|react-dom|from\s+["']react["']|next\/|vite|webpack|unpkg|jsdelivr|esm\.sh|cdn\.skypack|https?:\/\/)\b/i,
    "injected devtools runtime must not depend on npm, React DOM, Next, Vite, webpack, CDN URLs, or external browser UI packages",
  );
});

test("DX Devtools runtime publishes honest release-readiness visual edit replay evidence", () => {
  const runtime = devtoolsRuntime;
  const publishReplay = extractBalancedBlock(
    runtime,
    /function\s+readinessPublishVisualEditReplay\s*\(/,
    "readinessPublishVisualEditReplay",
  );
  const replayReady = extractBalancedBlock(
    runtime,
    /function\s+readinessVisualEditReady\s*\(/,
    "readinessVisualEditReady",
  );
  const preview = extractBalancedBlock(runtime, /async\s+function\s+previewStyle\s*\(/, "previewStyle");
  const apply = extractBalancedBlock(runtime, /async\s+function\s+applyStyle\s*\(/, "applyStyle");
  const undo = extractBalancedBlock(runtime, /async\s+function\s+undoStyle\s*\(/, "undoStyle");

  assertSourceMatches(runtime, /styleUndo:\s*"\/_dx\/devtools\/style-undo"/, "runtime should know the dev-only style-undo endpoint");
  assertSourceMatches(runtime, /__DX_READINESS_VISUAL_EDIT_REPLAY__/, "runtime should publish the release-readiness visual-edit browser replay global");
  assertSourceMatches(runtime, /READINESS_VISUAL_EDIT_PHASES[\s\S]*inspect[\s\S]*cascade[\s\S]*preview[\s\S]*apply[\s\S]*undo[\s\S]*receipt/, "runtime should track every release-readiness visual-edit phase");
  assertSourceMatches(runtime, /function\s+styleOperationBoxModelPayload[\s\S]*boxEdgePx[\s\S]*content[\s\S]*padding[\s\S]*border[\s\S]*margin/, "style operation requests should serialize numeric box model payloads for the Rust protocol");
  assertSourceMatches(runtime, /boxModel:\s*styleOperationBoxModelPayload\(/, "style-preview/apply requests should not send UI-only box edge objects to Rust");
  assertSourceMatches(
    publishReplay,
    /browser_workbench_replay:\s*replayCurrent\s*\?\s*READINESS_VISUAL_EDIT_REPLAY_CURRENT\s*:\s*READINESS_VISUAL_EDIT_REPLAY_MISSING/,
    "runtime should derive current replay proof from readiness rather than hardcoding current",
  );
  assertSourceMatches(
    replayReady,
    /previewProtocolOk\s*===\s*true[\s\S]*applyProtocolOk\s*===\s*true[\s\S]*undoProtocolOk\s*===\s*true[\s\S]*previewSourceMutated\s*===\s*false[\s\S]*applySourceMutated\s*===\s*true[\s\S]*undoSourceRestored\s*===\s*true[\s\S]*receiptDurability\s*===\s*"json-sr-machine-written"/,
    "runtime readiness should require successful protocol calls, preview non-mutation, safe apply, undo restoration, and durable receipts",
  );
  assertSourceMatches(preview, /readinessBeginStyleOperation\(request\)[\s\S]*readinessRecordPreview\(request,\s*result\)/, "preview should record browser workbench evidence around the real preview request");
  assertSourceMatches(apply, /ENDPOINTS\.styleApply[\s\S]*readinessRecordApply\(request,\s*result\)/, "apply should record evidence from the real style-apply response");
  assertSourceMatches(undo, /ENDPOINTS\.styleUndo[\s\S]*readinessRestoreInlinePreview\(\)[\s\S]*readinessRecordUndo\(result\)/, "undo should call the real style-undo endpoint, restore the live preview, and record undo proof");
  assertSourceMatches(
    runtime,
    /window\.__DX_DEVTOOLS_ACTIVE__\s*=\s*false;[\s\S]*readinessPublishVisualEditReplay\(\)/,
    "cleanup should republish the replay global as inactive/missing, not leave stale current proof behind",
  );
  assertSourceMatches(runtime, /__DX_DEVTOOLS_READINESS_VISUAL_EDIT_REPLAY__\s*=\s*readinessRunVisualEditReplay/, "runtime should expose an explicit dev-only browser replay entrypoint");
  assertSourceMatches(runtime, /function\s+readinessStyleRequestFromFixture[\s\S]*sourceTarget:\s*fixture\.sourceTarget/, "browser replay should use the safe release-readiness source target fixture instead of inventing a writable DOM source");
  assertSourceMatches(runtime, /function\s+readinessRecordVisualInspect[\s\S]*readinessSourceTargetIsSafe\(sourceTarget\)[\s\S]*!readinessSourceTargetIsSafe\(proof\.sourceTarget\)/, "inspect refreshes should not erase an already proven safe source target");
});

test("DX Devtools session exposes an automatable release-readiness visual edit browser replay fixture", () => {
  const devtoolsProtocol = read("dx-www/src/cli/devtools/protocol.rs");
  const sessionPayload = extractBalancedBlock(
    devtoolsProtocol,
    /fn\s+session_payload\s*\(/,
    "session_payload",
  );
  const replayFixture = extractBalancedBlock(
    devtoolsProtocol,
    /fn\s+readiness_visual_edit_browser_replay_fixture_value\s*\(/,
    "readiness_visual_edit_browser_replay_fixture_value",
  );

  assertSourceMatches(
    sessionPayload,
    /readiness_visual_edit_replay_fixture[\s\S]*readiness_visual_edit_browser_replay_fixture\(project_root\)/,
    "devtools session should expose the source-owned browser replay fixture",
  );
  assertSourceMatches(
    replayFixture,
    /dx\.devtools\.readiness\.visual_edit_browser_replay_fixture[\s\S]*status[\s\S]*ready[\s\S]*sourceTarget[\s\S]*relativePath[\s\S]*expectedText/,
    "fixture should expose a ready exact source target for browser replay",
  );
  assertSourceMatches(
    replayFixture,
    /receipt_contract[\s\S]*VISUAL_EDIT_RECEIPT_CONTRACT_ID[\s\S]*release_ready[\s\S]*false[\s\S]*fastest_world_claim[\s\S]*false/,
    "fixture should preserve the release-readiness claim boundary",
  );
});

test("DX Devtools serves MDN CSS metadata as a dev-only source-owned asset", () => {
  const assets = devtoolsAssetsRust;
  const modRs = read("dx-www/src/cli/devtools/mod.rs");
  const cssDataRs = read("dx-www/src/cli/devtools/css_data.rs");
  const cssData = readJson("dx-www/src/cli/devtools/css_data.generated.json");

  assertSourceMatches(
    assets,
    /DX_DEVTOOLS_CSS_DATA_PATH:\s*&str\s*=\s*"\/_dx\/devtools\/css-data"/,
    "devtools should define a css-data asset endpoint",
  );
  assertSourceMatches(
    assets,
    /DX_DEVTOOLS_CSS_DATA_PATH[\s\S]{0,240}application\/json[\s\S]{0,160}css_data::CSS_DATA_JSON/,
    "devtools should serve css-data as JSON from the generated MDN artifact",
  );
  assertSourceMatches(modRs, /mod\s+css_data;/, "devtools module should own css data separately");
  assertSourceMatches(
    cssDataRs,
    /include_str!\("css_data\.generated\.json"\)/,
    "generated CSS data should be embedded at compile time",
  );
  assert.equal(cssData.schema, "dx.devtools.css_data");
  assert.equal(cssData.source.repository, "https://github.com/mdn/data");
  assert.match(cssData.source.version, /^\d+\.\d+\.\d+$/);
  assert.match(cssData.source.commit, /^[0-9a-f]{40}$/);
  assert.ok(cssData.counts.properties >= 600, "MDN catalog should include the full property set");
  assert.ok(cssData.counts.selectors >= 100, "MDN catalog should include selectors");
  assert.ok(cssData.counts.propertyValueSyntaxes >= 600, "MDN catalog should include value syntaxes");
  assert.ok(cssData.counts.derivedPropertyValueHints > 1000, "DX should derive useful value hints from MDN syntax");
  assert.ok(cssData.properties.some((entry) => entry.name === "background-image" && entry.syntax));
  assert.ok(cssData.selectors.some((entry) => entry.name.includes(":has")));
});

test("DX Devtools runtime consumes the MDN catalog with fallbacks", () => {
  const assets = devtoolsRuntime;
  const styleEditorPanel = extractBalancedBlock(
    assets,
    /function\s+styleEditorPanel\s*\(/,
    "styleEditorPanel",
  );
  const catalogPropertyOptions = extractBalancedBlock(
    assets,
    /function\s+catalogPropertyOptions\s*\(/,
    "catalogPropertyOptions",
  );
  const catalogValueOptions = extractBalancedBlock(
    assets,
    /function\s+catalogValueOptions\s*\(/,
    "catalogValueOptions",
  );
  const refreshProtocol = extractBalancedBlock(
    assets,
    /async\s+function\s+refreshProtocol\s*\(/,
    "refreshProtocol",
  );

  assertSourceMatches(assets, /cssData:\s*"\/_dx\/devtools\/css-data"/, "runtime should know css-data endpoint");
  assertSourceMatches(refreshProtocol, /requestJson\(ENDPOINTS\.cssData\)\.catch/, "css-data fetch should not break Devtools boot");
  assertSourceMatches(refreshProtocol, /STATE\.cssDataByProperty\s*=\s*new Map/, "runtime should index MDN properties by name");
  assertSourceMatches(catalogPropertyOptions, /STATE\.cssData\?\.properties/, "property picker should read MDN properties");
  assertSourceMatches(catalogValueOptions, /valueHints/, "value picker should read derived MDN value hints");
  assertSourceMatches(styleEditorPanel, /catalogPropertyOptions\(current\)/, "style panel should use catalog properties");
  assertSourceMatches(styleEditorPanel, /catalogValueOptions\(draft\.property\)/, "style panel should use catalog values");
  assertSourceDoesNotMatch(
    styleEditorPanel,
    /\[\.\.\.STYLE_PROPERTY_OPTIONS,\s*\.\.\.Object\.keys\(current\)/,
    "style panel should not be limited to the old hardcoded property list",
  );
});

test("DX Devtools color and gradient picker uses real custom pointer controls", () => {
  const assets = devtoolsRuntime;
  const cssSource = devtoolsCss;
  const startTrackDrag = extractBalancedBlock(assets, /function\s+startTrackDrag\s*\(/, "startTrackDrag");
  const startAngleDialDrag = extractBalancedBlock(assets, /function\s+startAngleDialDrag\s*\(/, "startAngleDialDrag");
  const startPositionPadDrag = extractBalancedBlock(assets, /function\s+startPositionPadDrag\s*\(/, "startPositionPadDrag");
  const sliderControl = extractBalancedBlock(assets, /function\s+sliderControl\s*\(/, "sliderControl");
  const angleDialControl = extractBalancedBlock(assets, /function\s+angleDialControl\s*\(/, "angleDialControl");
  const positionPadControl = extractBalancedBlock(assets, /function\s+positionPadControl\s*\(/, "positionPadControl");
  const meshStageControl = extractBalancedBlock(assets, /function\s+meshStageControl\s*\(/, "meshStageControl");
  const solidPicker = extractBalancedBlock(assets, /function\s+solidPicker\s*\(/, "solidPicker");
  const linearPicker = extractBalancedBlock(assets, /function\s+linearPicker\s*\(/, "linearPicker");
  const radialPicker = extractBalancedBlock(assets, /function\s+radialPicker\s*\(/, "radialPicker");
  const meshPicker = extractBalancedBlock(assets, /function\s+meshPicker\s*\(/, "meshPicker");
  const pickerApplyButton = extractBalancedBlock(assets, /function\s+pickerApplyButton\s*\(/, "pickerApplyButton");
  const customTextControl = extractBalancedBlock(assets, /function\s+customTextControl\s*\(/, "customTextControl");
  const customSelectControl = extractBalancedBlock(assets, /function\s+customSelectControl\s*\(/, "customSelectControl");
  const customValueControl = extractBalancedBlock(assets, /function\s+customValueControl\s*\(/, "customValueControl");
  const customPopoverSearch = extractBalancedBlock(assets, /function\s+customPopoverSearch\s*\(/, "customPopoverSearch");
  const filteredPopoverOptions = extractBalancedBlock(assets, /function\s+filteredPopoverOptions\s*\(/, "filteredPopoverOptions");
  const onPopoverSearchKeydown = extractBalancedBlock(assets, /function\s+onPopoverSearchKeydown\s*\(/, "onPopoverSearchKeydown");
  const onSingleLineTextboxKeydown = extractBalancedBlock(assets, /function\s+onSingleLineTextboxKeydown\s*\(/, "onSingleLineTextboxKeydown");
  const onSelectKeydown = extractBalancedBlock(assets, /function\s+onSelectKeydown\s*\(/, "onSelectKeydown");
  const onPositionKeydown = extractBalancedBlock(assets, /function\s+onPositionKeydown\s*\(/, "onPositionKeydown");
  const renderAll = extractBalancedBlock(assets, /function\s+renderAll\s*\(/, "renderAll");
  const onMenuKeydown = extractBalancedBlock(assets, /function\s+onMenuKeydown\s*\(/, "onMenuKeydown");
  const inspectorPanel = extractBalancedBlock(assets, /function\s+inspectorPanel\s*\(/, "inspectorPanel");
  const pickerModeTabs = extractBalancedBlock(assets, /function\s+pickerModeTabs\s*\(/, "pickerModeTabs");

  for (const dragFunction of [startTrackDrag, startAngleDialDrag, startPositionPadDrag]) {
    assertSourceMatches(dragFunction, /preventDefault\(\)/, "pointer controls should own pointerdown default behavior");
    assertSourceMatches(dragFunction, /stopPropagation\(\)/, "pointer controls should not leak drag gestures to the app");
    assertSourceMatches(dragFunction, /setPointerCapture/, "pointer controls should capture active pointer drags");
    assertSourceMatches(dragFunction, /pointermove/, "pointer controls should listen for pointer movement");
    assertSourceMatches(dragFunction, /pointerup/, "pointer controls should stop on pointerup");
    assertSourceMatches(dragFunction, /pointercancel/, "pointer controls should stop on pointercancel");
    assertSourceMatches(dragFunction, /removeEventListener/, "pointer controls should clean up drag listeners");
    assertSourceMatches(dragFunction, /renderAll\(\)/, "pointer controls should perform one full render after drag completion");
  }
  assertSourceMatches(startTrackDrag, /pointerPercentFromRect/, "sliders should use a stable rect during drag");
  assertSourceMatches(startTrackDrag, /applySliderVisual/, "sliders should update their own DOM during drag");
  assertSourceMatches(startTrackDrag, /setColorPicker\(key,[\s\S]*min,[\s\S]*max\)/, "slider drag should update picker state");
  assertSourceMatches(startTrackDrag, /applySliderVisual\(slider,\s*key,\s*min,\s*max,\s*suffix\)/, "slider drag should update live slider DOM");
  assertSourceMatches(startAngleDialDrag, /Math\.atan2/, "angle dial should calculate real polar angle");
  assertSourceMatches(startAngleDialDrag, /setColorPicker\("linearAngle",\s*degrees,\s*0,\s*360\)/, "angle dial drag should update picker state");
  assertSourceMatches(startAngleDialDrag, /dial\.style\.setProperty\("--dx-devtools-angle"/, "angle dial drag should update live dial CSS");
  assertSourceMatches(startPositionPadDrag, /pointerPercentFromRect/, "position pads should map pointer coordinates into percentages");
  assertSourceMatches(startPositionPadDrag, /setColorPicker\(xKey,\s*percent\.x,\s*0,\s*100\)[\s\S]*setColorPicker\(yKey,\s*percent\.y,\s*0,\s*100\)/, "position drag should update both picker axes");
  assertSourceMatches(startPositionPadDrag, /handle\.style\.setProperty\("--dx-devtools-pad-x"[\s\S]*handle\.style\.setProperty\("--dx-devtools-pad-y"/, "position drag should update live pad CSS");
  assertSourceMatches(startPositionPadDrag, /aria-valuenow[\s\S]*aria-valuetext/, "position drag should update live ARIA values");
  assertSourceMatches(startPositionPadDrag, /meshGradientValue\(\)/, "mesh stage should refresh its live gradient preview");
  assertSourceDoesNotMatch(
    startTrackDrag,
    /const\s+update\s*=\s*\([^)]*\)\s*=>\s*\{[\s\S]*?renderAll\(\)[\s\S]*?\};\s*const\s+stop/,
    "slider drag update should not replace the panel subtree on every pointermove",
  );
  assertSourceDoesNotMatch(
    startAngleDialDrag,
    /const\s+update\s*=\s*\([^)]*\)\s*=>\s*\{[\s\S]*?renderAll\(\)[\s\S]*?\};\s*const\s+stop/,
    "angle drag update should not replace the panel subtree on every pointermove",
  );
  assertSourceDoesNotMatch(
    startPositionPadDrag,
    /const\s+update\s*=\s*\([^)]*\)\s*=>\s*\{[\s\S]*?renderAll\(\)[\s\S]*?\};\s*const\s+stop/,
    "position drag update should not replace the panel subtree on every pointermove",
  );

  assertSourceMatches(sliderControl, /className:\s*"dx-devtools-slider"/, "slider builder should render Devtools-owned slider chrome");
  assertSourceMatches(sliderControl, /role:\s*"slider"/, "slider builder should expose slider role");
  assertSourceMatches(sliderControl, /aria-orientation/, "slider builder should expose orientation");
  assertSourceMatches(sliderControl, /aria-valuemin[\s\S]*aria-valuemax[\s\S]*aria-valuenow[\s\S]*aria-valuetext/, "slider builder should expose readable ARIA values");
  assertSourceMatches(sliderControl, /data-dx-devtools-focus/, "slider builder should expose a stable focus token");
  assertSourceMatches(sliderControl, /data-dx-devtools-slider-suffix/, "slider builder should preserve value suffixes during drag");
  assertSourceMatches(sliderControl, /onPointerdown:\s*\(event\)\s*=>\s*startTrackDrag/, "slider builder should start a pointer drag");
  assertSourceMatches(sliderControl, /onKeydown:\s*\(event\)\s*=>\s*onSliderKeydown/, "slider builder should support keyboard adjustment");
  assertSourceMatches(sliderControl, /dx-devtools-slider-track[\s\S]*dx-devtools-slider-thumb/, "slider builder should render track and thumb parts");
  for (const expectedUse of [
    /sliderControl\("R",\s*"red"/,
    /sliderControl\("Hue",\s*"hue"/,
    /sliderControl\("Angle",\s*"linearAngle"/,
    /sliderControl\("X",\s*"radialX"/,
    /sliderControl\("X1",\s*"meshX1"/,
    /sliderControl\("Size 1",\s*"meshSize1"/,
    /sliderControl\("Pos 1",\s*"stopAOffset"/,
  ]) {
    assertSourceMatches(
      `${solidPicker}\n${linearPicker}\n${radialPicker}\n${meshPicker}`,
      expectedUse,
      `picker should use custom slider ${expectedUse}`,
    );
  }

  assertSourceMatches(angleDialControl, /className:\s*"dx-devtools-angle-dial"/, "angle builder should render a custom dial");
  assertSourceMatches(angleDialControl, /--dx-devtools-angle/, "angle builder should drive the dial with a CSS variable");
  assertSourceMatches(angleDialControl, /aria-orientation[\s\S]*aria-valuetext/, "angle dial should expose readable range semantics");
  assertSourceMatches(angleDialControl, /onPointerdown:\s*startAngleDialDrag/, "angle builder should start pointer drag");
  assertSourceMatches(angleDialControl, /onKeydown:\s*\(event\)\s*=>\s*onSliderKeydown/, "angle dial should support keyboard adjustment");
  assertSourceMatches(linearPicker, /angleDialControl\(\)/, "linear gradient picker should include the custom angle dial");
  assertSourceMatches(positionPadControl, /className:\s*"dx-devtools-position-pad"/, "radial builder should render a custom position pad");
  assertSourceMatches(positionPadControl, /data-dx-devtools-position-pad/, "position pad should expose a stable drag marker");
  assertSourceMatches(positionPadControl, /role:\s*"slider"[\s\S]*aria-valuemin[\s\S]*aria-valuemax[\s\S]*aria-valuenow[\s\S]*aria-valuetext/, "position pad should expose range semantics");
  assertSourceMatches(positionPadControl, /data-dx-devtools-focus/, "position pad should expose a focus token");
  assertSourceMatches(positionPadControl, /--dx-devtools-pad-x[\s\S]*--dx-devtools-pad-y/, "position pad should use CSS vars for position");
  assertSourceMatches(positionPadControl, /onPointerdown:\s*\(event\)\s*=>\s*startPositionPadDrag/, "position pad should start pointer drag");
  assertSourceMatches(positionPadControl, /onKeydown:\s*\(event\)\s*=>\s*onPositionKeydown/, "position pad should support keyboard movement");
  assertSourceMatches(radialPicker, /positionPadControl\("Gradient center",\s*"radialX",\s*"radialY"\)/, "radial picker should include center pad");
  assertSourceMatches(meshStageControl, /className:\s*"dx-devtools-mesh-stage"/, "mesh builder should render a mesh stage");
  assertSourceMatches(meshStageControl, /className:\s*"dx-devtools-mesh-layer"/, "mesh builder should render draggable mesh layers");
  assertSourceMatches(meshStageControl, /role:\s*"slider"[\s\S]*aria-valuetext/, "mesh layer handles should expose range semantics");
  assertSourceMatches(meshStageControl, /data-dx-devtools-focus/, "mesh layer handles should expose focus tokens");
  assertSourceMatches(meshStageControl, /onKeydown:\s*\(event\)\s*=>\s*onPositionKeydown/, "mesh layer handles should support keyboard movement");
  assertSourceMatches(meshStageControl, /meshGradientValue\(\)/, "mesh stage should preview the generated mesh gradient");
  assertSourceMatches(meshPicker, /meshStageControl\(\)/, "mesh picker should include the custom mesh stage");
  assertSourceMatches(assets, /function\s+adjustColorPicker/, "custom numeric controls should share an adjustment helper");
  assertSourceMatches(assets, /function\s+onSliderKeydown[\s\S]*ArrowRight[\s\S]*ArrowLeft[\s\S]*PageUp[\s\S]*PageDown[\s\S]*Home[\s\S]*End/, "custom controls should support arrow/page/home/end keyboard input");
  assertSourceMatches(onPositionKeydown, /PageUp[\s\S]*PageDown[\s\S]*Home[\s\S]*End/, "position controls should support page/home/end keyboard input");
  assertSourceMatches(assets, /activeColorStop:\s*"stopA"/, "color picker should track an active gradient stop");
  assertSourceMatches(
    `${linearPicker}\n${radialPicker}\n${meshPicker}`,
    /stopTargetControls\(\)[\s\S]*colorStopSwatchList\(/,
    "gradient pickers should expose active stop controls and stop swatches",
  );
  assertSourceMatches(assets, /setColorPicker\(STATE\.activeColorStop,\s*value\)/, "stop swatches should edit the active gradient stop");
  assertSourceMatches(
    pickerApplyButton,
    /onClick:\s*\(\)\s*=>\s*setStyleDraft\(property,\s*valueFactory\(\)\)/,
    "picker apply buttons should commit generated CSS values into the style draft",
  );

  assertSourceMatches(customSelectControl, /aria-controls[\s\S]*aria-activedescendant[\s\S]*aria-expanded/, "custom selects should expose combobox ownership");
  assertSourceMatches(customSelectControl, /role:\s*"option"[\s\S]*aria-selected[\s\S]*data-dx-devtools-active[\s\S]*tabIndex:\s*"-1"/, "custom select options should expose visible active-descendant state");
  assertSourceMatches(customValueControl, /aria-controls[\s\S]*aria-activedescendant[\s\S]*aria-expanded/, "custom value options should expose combobox ownership");
  assertSourceMatches(customValueControl, /role:\s*"option"[\s\S]*aria-selected[\s\S]*data-dx-devtools-active[\s\S]*tabIndex:\s*"-1"/, "custom value options should expose visible active-descendant state");
  assertSourceMatches(assets, /activePopoverIndex:\s*\{\}/, "custom select popovers should track active option separately from committed value");
  assertSourceMatches(assets, /popoverQuery:\s*\{\}/, "custom popovers should track searchable query state separately from values");
  assertSourceMatches(filteredPopoverOptions, /STATE\.popoverQuery\[openKey\]/, "custom popovers should read their per-popover query");
  assertSourceMatches(filteredPopoverOptions, /\.filter\(\(option\)\s*=>\s*option\.toLowerCase\(\)\.includes\(query\)\)/, "custom popovers should filter large CSS option sets by query");
  assertSourceMatches(customPopoverSearch, /contenteditable:\s*"plaintext-only"/, "popover search should use the custom contenteditable control, not a native input");
  assertSourceMatches(customPopoverSearch, /data-dx-devtools-popover-search/, "popover search should expose a stable marker for testing and focus");
  assertSourceMatches(customPopoverSearch, /setPopoverQuery\(openKey,\s*options,[\s\S]*renderAll\(\{\s*focusToken:\s*searchMarker\s*\}\)/, "popover search should update query state and restore caret focus");
  assertSourceMatches(onPopoverSearchKeydown, /ArrowDown[\s\S]*onSelectKeydown/, "popover search should hand option navigation to the combobox keyboard path");
  assertSourceDoesNotMatch(onPopoverSearchKeydown, /event\.key\s*===\s*" "|event\.key\s*===\s*"Space"/, "popover search should allow spaces to be typed instead of committing an option");
  assertSourceMatches(onSingleLineTextboxKeydown, /event\.key\s*===\s*"Enter"[\s\S]*preventDefault\(\)[\s\S]*event\.key\s*===\s*"Escape"[\s\S]*blur/, "single-line contenteditable controls should prevent line breaks and escape cleanly");
  assertSourceMatches(customTextControl, /"aria-multiline":\s*"false"[\s\S]*onKeydown:\s*onSingleLineTextboxKeydown/, "custom text controls should expose single-line textbox semantics");
  assertSourceMatches(customValueControl, /"aria-multiline":\s*"false"[\s\S]*onKeydown:\s*onSingleLineTextboxKeydown/, "custom CSS value controls should expose single-line textbox semantics");
  assertSourceMatches(customSelectControl, /const\s+activeDescendant\s*=\s*open\s*&&\s*visibleOptions\.length\s*\?\s*selectOptionId\(listId,\s*activeIndex\)\s*:\s*""/, "custom selects should not point aria-activedescendant at an empty result");
  assertSourceMatches(customValueControl, /const\s+activeDescendant\s*=\s*open\s*&&\s*visibleOptions\.length\s*\?\s*selectOptionId\(optionsId,\s*activeIndex\)\s*:\s*""/, "custom value options should not point aria-activedescendant at an empty result");
  assertSourceMatches(customSelectControl, /"aria-label":\s*label[\s\S]*"aria-activedescendant":\s*activeDescendant/, "custom selects should carry a contextual accessible name");
  assertSourceMatches(customValueControl, /"aria-label":\s*`\$\{label\} options`[\s\S]*"aria-activedescendant":\s*activeDescendant/, "custom value option buttons should carry contextual accessible names");
  assertSourceMatches(customSelectControl, /filteredPopoverOptions\(openKey,\s*options\)/, "custom property selectors should render the filtered option list");
  assertSourceMatches(customSelectControl, /customPopoverSearch\(`Search \$\{label\.toLowerCase\(\)\}`/, "custom property selectors should include a searchable popover field");
  assertSourceMatches(customValueControl, /customPopoverSearch\("Search values"/, "custom value selectors should include a searchable popover field");
  assertSourceMatches(onSelectKeydown, /Escape[\s\S]*STATE\[openKey\]\s*=\s*false/, "custom selects should close from Escape");
  assertSourceMatches(onSelectKeydown, /Enter[\s\S]*pickAt\(currentIndex\)/, "custom selects should commit the active option on Enter");
  assertSourceMatches(onSelectKeydown, /Space[\s\S]*pickAt\(currentIndex\)/, "custom selects should commit the active option on Space");
  assertSourceMatches(onSelectKeydown, /ArrowDown[\s\S]*moveTo\(currentIndex \+ 1\)/, "custom selects should move active option down without immediate commit");
  assertSourceMatches(onSelectKeydown, /ArrowUp[\s\S]*moveTo\(currentIndex - 1\)/, "custom selects should move active option up without immediate commit");
  assertSourceMatches(onSelectKeydown, /setActiveOptionIndex/, "custom select arrows should move active descendants without committing values");
  assertSourceMatches(renderAll, /captureFocusToken\(\)[\s\S]*replaceChildren[\s\S]*restoreFocusToken\(focusToken\)/, "renderAll should restore focus after replacing panel content");
  assertSourceMatches(inspectorPanel, /onKeydown:\s*\(event\)\s*=>\s*onTabKeydown[\s\S]*focusToken:\s*`inspector-tab-\$\{next\}`/, "inspector tabs should support keyboard navigation and focus restoration");
  assertSourceMatches(pickerModeTabs, /onKeydown:\s*\(event\)\s*=>\s*onTabKeydown[\s\S]*focusToken:\s*`color-tab-\$\{next\}`/, "picker tabs should support keyboard navigation and focus restoration");
  assertSourceMatches(onMenuKeydown, /stopPropagation/, "puck menu should contain Escape handling");
  assertSourceMatches(onMenuKeydown, /ArrowDown[\s\S]*ArrowUp[\s\S]*Home[\s\S]*End/, "puck menu should support arrow/home/end navigation");

  for (const selector of [
    ".dx-devtools-slider",
    ".dx-devtools-angle-dial",
    ".dx-devtools-position-pad,\n.dx-devtools-mesh-stage",
  ]) {
    assertSourceMatches(cssBlock(cssSource, selector), /touch-action:\s*none/, `${selector} should own touch gestures`);
  }
  assertSourceMatches(cssBlock(cssSource, ".dx-devtools-slider-thumb"), /--dx-devtools-slider-fill/, "slider thumb should follow the slider fill variable");
  assertSourceMatches(cssBlock(cssSource, ".dx-devtools-angle-dial::after"), /--dx-devtools-angle/, "angle dial needle should follow the angle variable");
  const handleCss = cssBlock(cssSource, ".dx-devtools-position-dot,\n.dx-devtools-mesh-layer");
  assertSourceMatches(handleCss, /--dx-devtools-pad-x/, "position handles should follow horizontal pad CSS variable");
  assertSourceMatches(handleCss, /--dx-devtools-pad-y/, "position handles should follow vertical pad CSS variable");
  assertSourceMatches(cssBlock(cssSource, ".dx-devtools-input-shell"), /min-height:\s*44px/, "custom text controls should keep a 44px effective hit target");
  assertSourceMatches(cssBlock(cssSource, ".dx-devtools-input-shell [role=\"textbox\"]"), /min-height:\s*42px/, "custom textboxes should fill the themed input shell");
  assertSourceMatches(cssBlock(cssSource, ".dx-devtools-popover-search"), /display:\s*grid/, "popover search should have a stable custom layout");
  assertSourceMatches(cssBlock(cssSource, ".dx-devtools-option-list"), /display:\s*grid/, "filtered option lists should use stable Devtools-owned layout");
  assertSourceMatches(cssBlock(cssSource, ".dx-devtools-popover button[data-dx-devtools-active=\"true\"]"), /box-shadow:\s*inset/, "custom selects should visibly mark the active descendant");
  assertSourceDoesNotMatch(
    assets,
    /(?:el|document\.createElement)\(\s*["'](?:input|select|textarea)["']|setAttribute\(\s*["']type["']\s*,\s*["']range["']\s*\)|type\s*:\s*["']range["']|type=["']range["']|<\s*(?:input|select|textarea)\b|HTML(?:Input|Select|TextArea)Element/i,
    "Devtools picker should not sneak in native form controls or range inputs",
  );
});

test("DX Devtools inspect and runtime listeners are throttled and cleanable", () => {
  const assets = devtoolsRuntime;
  const captureElement = extractBalancedBlock(assets, /function\s+captureElement\s*\(/, "captureElement");
  const onDocumentPointerMove = extractBalancedBlock(assets, /function\s+onDocumentPointerMove\s*\(/, "onDocumentPointerMove");

  assertSourceMatches(assets, /const\s+controller\s*=\s*new AbortController\(\)/, "runtime should own listener cleanup with AbortController");
  assertSourceMatches(assets, /window\.__DX_DEVTOOLS_CLEANUP__\s*=/, "runtime should expose a cleanup hook for reinjection/HMR");
  assertSourceMatches(assets, /documentElement\.classList\.remove\("dx-devtools-inspecting"\)/, "cleanup should remove inspect mode class");
  assertSourceMatches(assets, /addEventListener[\s\S]{0,180}signal:\s*listenerSignal/, "runtime listeners should be attached with an abort signal");
  assertSourceMatches(assets, /runtimeIssues\.length\s*>\s*20/, "runtime issue list should be capped");
  assertSourceMatches(onDocumentPointerMove, /target\s*===\s*inspectedTarget/, "inspect hover should skip repeated target renders");
  assertSourceMatches(captureElement, /sourceLookupKey[\s\S]*lastSourceLookupKey/, "inspect capture should avoid repeated source-map requests for the same source");
  assertSourceMatches(captureElement, /scheduleRenderAll\(\)/, "inspect capture should schedule a single render frame");
});

test("DX Devtools close buttons are wired to close the active panel", () => {
  const assets = devtoolsRuntime;
  const closePanel = extractBalancedBlock(
    assets,
    /function\s+closePanel\s*\(/,
    "closePanel",
  );
  const openPanel = extractBalancedBlock(
    assets,
    /function\s+openPanel\s*\(/,
    "openPanel",
  );
  const buildRoot = extractBalancedBlock(
    assets,
    /function\s+buildRoot\s*\(/,
    "buildRoot",
  );
  const panel = extractBalancedBlock(
    assets,
    /function\s+panel\s*\(/,
    "panel",
  );

  assertSourceMatches(
    closePanel,
    /STATE\.openPanels\.delete\(edge\)/,
    "closePanel should remove the current edge from openPanels",
  );
  assertSourceMatches(closePanel, /renderAll\(\)/, "closePanel should re-render after closing");
  assertSourceMatches(openPanel, /panelInCloseCooldown\(edge\)/, "openPanel should respect the close cooldown");
  assertSourceMatches(openPanel, /options\?\.force/, "openPanel should allow explicit future force-open flows");
  assertSourceMatches(assets, /performance\.now\(\)\s*<\s*STATE\.edgeCooldownUntil/, "close cooldown should be time-bound by performance.now");
  assertSourceDoesNotMatch(assets, /EDGE_HOLD_MS|beginEdgeHold|cancelEdgeHold|edgeHoldTimers|pendingEdge/, "edge-hover hold machinery should not remain in the circle-driven runtime");
  assertSourceMatches(buildRoot, /root\.append\(panelLayer,\s*puck,\s*menu,\s*overlay\)/, "root should mount panels, puck, menu, and overlay without edge sensors");
  assertSourceMatches(
    panel,
    /data-dx-devtools-close[\s\S]*aria-label[\s\S]*closePanel\(edge\)[\s\S]*text:\s*`Close \$\{title\}`/,
    "each panel header should render a labeled Close button wired to closePanel(edge)",
  );
  assertSourceMatches(panel, /aria-hidden[\s\S]*hidden:\s*!isOpen[\s\S]*inert:\s*!isOpen/, "closed panels should leave the accessibility and tab order");
});

test("DX Devtools exposes an inspect/select element control", () => {
  const assets = devtoolsRuntime;
  const previewPanel = extractBalancedBlock(
    assets,
    /function\s+previewPanel\s*\(/,
    "previewPanel",
  );
  const inspectMode = extractBalancedBlock(
    assets,
    /function\s+setInspectMode\s*\(/,
    "setInspectMode",
  );

  assertSourceMatches(
    previewPanel,
    /aria-pressed[\s\S]{0,160}STATE\.inspectMode/,
    "inspect/select element button should expose pressed state",
  );
  assertSourceMatches(
    previewPanel,
    /onClick:\s*\(\)\s*=>\s*setInspectMode\(!STATE\.inspectMode\)/,
    "inspect/select element button should toggle inspect mode",
  );
  assertSourceMatches(
    previewPanel,
    /Inspecting\.\.\.|Inspect element/,
    "inspect/select element button should have visible select-state text",
  );
  assertSourceMatches(
    inspectMode,
    /dx-devtools-inspecting/,
    "inspect mode should toggle a document-level inspecting class",
  );
});

test("DX Devtools style panel exposes color and gradient picker strings", () => {
  const assets = devtoolsRuntime;
  const styleEditorPanel = extractBalancedBlock(
    assets,
    /function\s+styleEditorPanel\s*\(/,
    "styleEditorPanel",
  );
  const colorPopoverControl = extractBalancedBlock(
    assets,
    /function\s+colorPopoverControl\s*\(/,
    "colorPopoverControl",
  );
  const pickerSnippet = [
    styleEditorPanel,
    colorPopoverControl,
    extractBalancedBlock(assets, /function\s+solidPicker\s*\(/, "solidPicker"),
    extractBalancedBlock(assets, /function\s+linearPicker\s*\(/, "linearPicker"),
    extractBalancedBlock(assets, /function\s+radialPicker\s*\(/, "radialPicker"),
    extractBalancedBlock(assets, /function\s+meshPicker\s*\(/, "meshPicker"),
    extractBalancedBlock(assets, /function\s+swatchList\s*\(/, "swatchList"),
  ].join("\n");

  for (const marker of [
    "Quick solid colors",
    "Quick gradients",
    "Use RGB",
    "Use HSL",
    "Use linear gradient",
    "Use radial gradient",
    "Use mesh gradient",
  ]) {
    assertSourceMatches(
      pickerSnippet,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `style panel should expose ${marker}`,
    );
  }
  assertSourceMatches(
    pickerSnippet,
    /SOLID_PRESETS/,
    "style panel should keep solid color presets as shortcuts",
  );
  assertSourceMatches(
    pickerSnippet,
    /linearGradientValue|radialGradientValue|meshGradientValue/,
    "style panel should call real linear, radial, and mesh gradient builders",
  );
  assertSourceMatches(
    colorPopoverControl,
    /"aria-haspopup":\s*"dialog"[\s\S]*"aria-controls":\s*"dx-devtools-color-popover"/,
    "color picker triggers should announce the controlled dialog",
  );
  assertSourceMatches(
    colorPopoverControl,
    /id:\s*"dx-devtools-color-popover"[\s\S]*role:\s*"dialog"[\s\S]*"aria-label":\s*"Color picker"/,
    "opened color picker should expose real dialog semantics",
  );
  assertSourceMatches(assets, /function\s+linearGradientValue[\s\S]*linearAngle[\s\S]*linear-gradient/, "linear picker should build from angle and stops");
  assertSourceMatches(assets, /function\s+radialGradientValue[\s\S]*radialShape[\s\S]*radialX[\s\S]*radial-gradient/, "radial picker should build from shape, position, and stops");
  assertSourceMatches(assets, /function\s+meshGradientValue[\s\S]*meshX1[\s\S]*radial-gradient/, "mesh picker should build layered radial gradients");
  assertSourceDoesNotMatch(
    styleEditorPanel,
    /el\("input"/,
    "style editor should use Devtools-owned custom controls instead of native input elements",
  );
});

test("DX Devtools captures all computed CSS and keeps box values visible", () => {
  const assets = devtoolsRuntime;
  const computedPayload = extractBalancedBlock(
    assets,
    /function\s+computedPayload\s*\(/,
    "computedPayload",
  );
  const boxPanel = extractBalancedBlock(
    assets,
    /function\s+boxPanel\s*\(/,
    "boxPanel",
  );

  assertSourceMatches(
    computedPayload,
    /window\.getComputedStyle\(target\)/,
    "computed payload should read the browser computed style object",
  );
  assertSourceMatches(
    computedPayload,
    /Array\.from\(styles\)\.sort\(\)/,
    "computed payload should iterate every property exposed by getComputedStyle, not only a curated allowlist",
  );
  assertSourceMatches(
    computedPayload,
    /for\s*\(\s*const\s+property\s+of\s+names\s*\)\s*\{[\s\S]{0,180}styles\.getPropertyValue\(property\)/,
    "computed payload should serialize each computed property value by name",
  );
  assertSourceMatches(
    computedPayload,
    /Object\.entries\(CSS_GROUPS\)/,
    "computed payload should still project grouped readouts from the full computed property map",
  );
  assertSourceMatches(
    boxPanel,
    /boxSize\(model\.contentBox\s*\|\|\s*model\.content\)/,
    "box model panel should render visible content dimensions",
  );
  assertSourceMatches(
    boxPanel,
    /\["Margin",\s*edgeSummary\(model\.margin\)\][\s\S]*\["Border",\s*edgeSummary\(model\.border\)\][\s\S]*\["Padding",\s*edgeSummary\(model\.padding\)\]/,
    "box model panel should expose real margin, border, and padding values in metrics",
  );
  assertSourceDoesNotMatch(
    boxPanel,
    /boxLayer\(|dx-devtools-box-readout/,
    "box model panel should not render decorative margin/padding/border cards",
  );
});

test("DX Devtools source guards reject native form controls in the injected UI", () => {
  const assets = devtoolsRuntime;
  const cssSource = devtoolsCss;

  assertSourceDoesNotMatch(
    assets,
    /(?:el|document\.createElement)\(\s*["'](?:input|select|textarea)["']|setAttribute\(\s*["']type["']\s*,\s*["']range["']\s*\)|type\s*:\s*["']range["']|<\s*(?:input|select|textarea)\b|HTML(?:Input|Select|TextArea)Element/i,
    "injected Devtools UI should not create native input, select, or textarea controls",
  );
  assertSourceDoesNotMatch(
    cssSource,
    /(?:^|[,{]\s*)(?:input|select|textarea)(?:\s|[.#:[,{>+~])/im,
    "Devtools CSS should not target native input, select, or textarea controls",
  );
});

test("DX Devtools chrome avoids the red-dot indicator and keeps panels above app chrome", () => {
  const assets = `${devtoolsRuntime}\n${devtoolsCss}`;
  const cssSource = devtoolsCss;
  const rootZ = zIndexValue(cssBlock(cssSource, "#dx-devtools-root"), "root");
  const panelZ = zIndexValue(cssBlock(cssSource, ".dx-devtools-panel"), "panel");
  const overlayZ = zIndexValue(cssBlock(cssSource, ".dx-devtools-box-overlay"), "box overlay");
  const puckZ = zIndexValue(cssBlock(cssSource, ".dx-devtools-puck"), "puck");
  const menuZ = zIndexValue(cssBlock(cssSource, ".dx-devtools-puck-menu"), "puck menu");

  assertSourceDoesNotMatch(
    assets,
    /dx-devtools-puck-dot|\.dx-devtools-puck-dot\b/,
    "Devtools chrome should not render a red dot indicator on the puck",
  );
  assertSourceDoesNotMatch(
    assets,
    /el\("span",\s*\{\s*text:\s*\{\s*left:\s*"Inspector"|\.dx-devtools-edge span/,
    "edge sensors should not render visible Inspector/Preview/Box badges",
  );
  assertSourceMatches(cssSource, /@keyframes\s+dx-devtools-puck-bounce/, "puck should define a smooth bounce keyframe");
  assertSourceMatches(cssBlock(cssSource, ".dx-devtools-puck"), /animation:\s*dx-devtools-puck-bounce/, "puck should use bounce animation");
  assertSourceMatches(
    snippetsAround(cssSource, /@media\s*\(prefers-reduced-motion:\s*reduce\)/, 1200),
    /dx-devtools-puck[\s\S]{0,260}animation:\s*none/,
    "reduced motion should disable puck animation",
  );
  assertSourceDoesNotMatch(
    cssBlock(cssSource, ".dx-devtools-inspecting *"),
    /^cursor:\s*crosshair\s*!important;\s*$/m,
    "inspect cursor should not be the plain oversized crosshair only",
  );
  assert.ok(rootZ >= 2147483000, "Devtools root should sit above normal app chrome");
  assert.ok(panelZ > rootZ, "Devtools panels should layer above the injected root");
  assert.ok(overlayZ > rootZ, "box overlay should layer above normal app chrome");
  assert.ok(puckZ > rootZ, "puck should stay reachable above normal app chrome");
  assert.ok(menuZ > rootZ, "puck menu should layer above normal app chrome");
  assert.ok(panelZ > overlayZ, "Devtools sheets should keep the highest z-index");
  assert.ok(panelZ > puckZ, "Devtools sheets should stay above the draggable puck");
  assert.ok(panelZ > menuZ, "Devtools sheets should stay above puck popovers");
});

test("DX Devtools color swatches are circular source-owned controls", () => {
  const cssSource = devtoolsCss;
  const swatchCss = cssBlock(cssSource, ".dx-devtools-swatch");
  const swatchPreviewCss = cssBlock(cssSource, ".dx-devtools-swatch::before");

  assertSourceMatches(
    swatchCss,
    /aspect-ratio:\s*1\s*\/\s*1/,
    "color swatches should reserve a square footprint before circular rounding",
  );
  assertSourceMatches(
    swatchCss,
    /border-radius:\s*(?:999px|50%)/,
    "color swatch buttons should be circular",
  );
  assertSourceMatches(
    swatchPreviewCss,
    /border-radius:\s*(?:999px|50%)/,
    "color swatch inner previews should stay circular",
  );
});

test("DX Devtools source resolution accepts nested source targets", () => {
  const sourceMap = read("dx-www/src/cli/devtools/source_map.rs");
  const requestSourcePath = extractBalancedBlock(
    sourceMap,
    /fn\s+request_source_path\s*\(/,
    "request_source_path",
  );

  assertSourceMatches(
    requestSourcePath,
    /sourceTarget|source_target/,
    "source resolution should inspect structured style request sourceTarget payloads",
  );
  assertSourceMatches(
    requestSourcePath,
    /relativePath|relative_path/,
    "source resolution should accept nested relativePath fields from the browser runtime",
  );
});

test("error overlay remains present in the dev hot reload client", () => {
  const client = read("dx-www/src/cli/dev_hot_reload_client.rs");
  const overlay = read("dx-www/src/dev/error_overlay.rs");

  assertSourceMatches(client, /data-dx-www-error-overlay/, "dev hot reload client should render an overlay shell");
  assertSourceMatches(client, /window\.__DX_SHOW_ERROR__/, "dev hot reload client should expose show-error hook");
  assertSourceMatches(client, /window\.__DX_HIDE_ERROR__/, "dev hot reload client should expose hide-error hook");
  assertSourceMatches(client, /window\.addEventListener\("error"/, "runtime errors should surface in the overlay");
  assertSourceMatches(client, /window\.addEventListener\("unhandledrejection"/, "promise rejections should surface in the overlay");
  assertSourceMatches(client, /DX-WWW error overlay/, "overlay title should remain present");
  assertSourceMatches(overlay, /DX_FEEDBACK_ERRORS_ENDPOINT = '\/_dx\/feedback\/errors'/, "feedback errors endpoint should remain wired");
});

test("hot reload protocol constants remain stable", () => {
  const protocol = read("dx-www/src/hot_reload_protocol.rs");

  assertSourceMatches(protocol, /DX_HOT_RELOAD_PROTOCOL:\s*&str\s*=\s*"dx\.hot-reload\.poll"/, "poll protocol name should remain stable");
  assertSourceMatches(protocol, /DX_HOT_RELOAD_VERSION_ENDPOINT:\s*&str\s*=\s*"\/_dx\/hot-reload\/version"/, "version endpoint should remain stable");
  assertSourceMatches(protocol, /DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT:\s*&str\s*=\s*"\/_dx\/hot-reload\/events"/, "event stream endpoint should remain stable");
  assertSourceMatches(protocol, /DX_HOT_RELOAD_EVENT_NAME:\s*&str\s*=\s*"dx-hot-reload"/, "SSE event name should remain stable");
  assertSourceMatches(protocol, /DX_HOT_RELOAD_STYLE_REFRESH_INSTRUCTION:\s*&str\s*=\s*"refresh-style"/, "style refresh instruction should remain stable");
  assertSourceMatches(protocol, /DX_HOT_RELOAD_ASSET_REFRESH_INSTRUCTION:\s*&str\s*=\s*"refresh-asset"/, "asset refresh instruction should remain stable");
  assertSourceMatches(protocol, /DX_HOT_RELOAD_ISSUE_MODE:\s*&str\s*=\s*"diagnostic-overlay"/, "diagnostic overlay mode should remain stable");
  assertSourceMatches(protocol, /DX_HOT_RELOAD_DISABLED_INSTRUCTION:\s*&str\s*=\s*"disabled"/, "disabled instruction should remain stable");
});

test("style-preview is read-only and cannot mutate project source", () => {
  const devtoolsProtocol = read("dx-www/src/cli/devtools/protocol.rs");
  const styleOps = read("dx-www/src/cli/devtools/style_ops.rs");
  const previewProtocol = extractBalancedBlock(
    devtoolsProtocol,
    /fn\s+style_preview_payload\s*\(/,
    "style_preview_payload",
  );
  const previewOperation = extractBalancedBlock(
    styleOps,
    /pub\(crate\)\s+fn\s+preview_style_change\s*\(/,
    "preview_style_change",
  );
  const previewSnippet = `${previewProtocol}\n${previewOperation}`;

  assertSourceMatches(previewSnippet, /style-preview|style_preview/i, "style-preview endpoint or handler should exist");
  assertSourceMatches(
    previewSnippet,
    /writesSource["']?\s*[:=]\s*false|writes_source["']?\s*[:=]\s*false|mutates_source["']?\s*[:=]\s*false|read_only["']?\s*[:=]\s*true/i,
    "style-preview should advertise a read-only/no-source-mutation contract",
  );
  assertNoSourceMutation(previewSnippet, "style-preview");
});

test("DX Devtools visual edit responses expose release-readiness receipt contract candidates", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");
  const devtoolsProtocol = read("dx-www/src/cli/devtools/protocol.rs");
  const styleOps = read("dx-www/src/cli/devtools/style_ops.rs");
  const devtoolsSource = `${devtoolsProtocol}\n${styleOps}`;

  assertSourceMatches(
    readiness,
    /dx\.www\.readiness\.visual_edit_workbench_receipt_contract/,
    "release-readiness should name the visual edit workbench receipt contract schema",
  );
  assertSourceMatches(
    readiness,
    /\.dx\/receipts\/devtools\/visual-edit-latest\.json/,
    "release-readiness should publish the latest visual-edit receipt path as a required receipt",
  );
  assertSourceMatches(
    devtoolsSource,
    /receipt_contract/,
    "framework-owned devtools responses should expose a receipt_contract surface",
  );
  assertSourceMatches(
    devtoolsSource,
    /receipt_candidate/,
    "framework-owned devtools responses should expose a receipt_candidate surface",
  );
  assertSourceMatches(
    devtoolsSource,
    /dx\.www\.readiness\.visual_edit_workbench_receipt_contract/,
    "devtools receipt candidates should carry the release-readiness visual edit receipt contract schema",
  );
  assertSourceMatches(
    devtoolsSource,
    /\.dx\/receipts\/devtools\/visual-edit-latest\.json/,
    "devtools receipt candidates should point at the latest visual-edit receipt path",
  );
  assertSourceMatches(
    devtoolsSource,
    /workbench_phases[\s\S]*inspect[\s\S]*cascade[\s\S]*preview[\s\S]*apply[\s\S]*undo[\s\S]*receipt/,
    "visual edit receipt contracts should name every release-readiness workbench phase",
  );
  assertSourceMatches(
    devtoolsSource,
    /missing_release_phases[\s\S]*browser-workbench-replay/,
    "visual edit receipt contracts should keep browser replay as the remaining release gate",
  );
  assertSourceMatches(
    devtoolsSource,
    /STYLE_UNDO_ENDPOINT[\s\S]*\/_dx\/devtools\/style-undo[\s\S]*style_undo/,
    "devtools should expose the source-owned style-undo endpoint in the protocol session",
  );
  assertSourceMatches(
    devtoolsSource,
    /receipt_durability["']?\s*:\s*"candidate-only-not-written"/,
    "preview and rejected devtools receipt candidates should not claim durable receipt writes",
  );
  assertSourceMatches(
    devtoolsSource,
    /visual_edit_apply_receipts["']?\s*:\s*true[\s\S]*visual_edit_apply_receipt_formats["']?\s*:\s*\["json",\s*"sr",\s*"machine"\][\s\S]*visual_edit_undo_receipts["']?\s*:\s*true/,
    "devtools session should advertise durable apply and undo receipts only for safe local mutations",
  );
  assertSourceMatches(
    devtoolsSource,
    /fn\s+write_visual_edit_receipt_artifacts\s*\([\s\S]*write_sr_artifact[\s\S]*serde_json::to_vec_pretty/,
    "safe style-apply should persist both serializer .sr/.machine artifacts and the JSON read model",
  );
  assertSourceMatches(
    devtoolsSource,
    /VISUAL_EDIT_RECEIPT_SR_PATH[\s\S]*\.dx\/receipts\/devtools\/visual-edit-latest\.sr[\s\S]*VISUAL_EDIT_RECEIPT_MACHINE_PATH[\s\S]*\.dx\/serializer\/receipts-devtools-visual-edit-latest\.machine/,
    "visual edit receipt paths should include the durable serializer and generated machine contracts",
  );
  assertSourceMatches(
    devtoolsSource,
    /write_readiness_visual_edit_replay_receipt[\s\S]*STYLE_PREVIEW_ENDPOINT[\s\S]*STYLE_APPLY_ENDPOINT[\s\S]*STYLE_UNDO_ENDPOINT/,
    "release-readiness visual edit replay should exercise preview, apply, and undo through the framework protocol",
  );
  assertSourceMatches(
    devtoolsSource,
    /READINESS_VISUAL_EDIT_STARTER_SOURCE[\s\S]*examples\/template\/styles\/theme\.css[\s\S]*READINESS_VISUAL_EDIT_REPLAY_PROPERTY[\s\S]*--ring/,
    "release-readiness visual edit replay should target a known authored CSS declaration in the canonical starter",
  );
  assertSourceMatches(
    devtoolsSource,
    /source_restored[\s\S]*browser_workbench_replay[\s\S]*missing[\s\S]*claim_boundary/,
    "release-readiness visual edit replay should prove safe source restoration without inventing browser workbench proof",
  );
  assertSourceMatches(
    readiness,
    /--write-visual-edit-replay[\s\S]*devtools::write_readiness_visual_edit_replay_receipt[\s\S]*visual_edit_replay/,
    "release-readiness CLI should expose an explicit visual edit replay writer instead of hiding it in --write-receipts",
  );
  assertSourceMatches(
    readiness,
    /workbench_phases[\s\S]*inspect[\s\S]*cascade[\s\S]*preview[\s\S]*apply[\s\S]*undo[\s\S]*receipt/,
    "release-readiness proof graph should track every visual edit workbench phase",
  );
  assertSourceMatches(
    readiness,
    /missing_release_phases[\s\S]*browser-workbench-replay/,
    "release-readiness proof graph should keep browser replay as the remaining visual edit release gate",
  );
});

test("DX Devtools visual edit receipt candidates keep undo proof honest and durable", () => {
  const devtoolsProtocol = read("dx-www/src/cli/devtools/protocol.rs");
  const styleOps = read("dx-www/src/cli/devtools/style_ops.rs");
  const sessionPayload = extractBalancedBlock(
    devtoolsProtocol,
    /fn\s+session_payload\s*\(/,
    "session_payload",
  );
  const visualReceiptContract = extractBalancedBlock(
    devtoolsProtocol,
    /fn\s+visual_edit_receipt_contract\s*\(/,
    "visual_edit_receipt_contract",
  );
  const visualReceiptCandidate = extractBalancedBlock(
    devtoolsProtocol,
    /fn\s+visual_edit_receipt_candidate\s*\(/,
    "visual_edit_receipt_candidate",
  );
  const nonLocalReceiptCandidate = extractBalancedBlock(
    devtoolsProtocol,
    /fn\s+visual_edit_non_local_receipt_candidate\s*\(/,
    "visual_edit_non_local_receipt_candidate",
  );
  const undoPayload = extractBalancedBlock(
    devtoolsProtocol,
    /fn\s+style_undo_payload\s*\(/,
    "style_undo_payload",
  );
  const undoReceiptCandidate = extractBalancedBlock(
    devtoolsProtocol,
    /fn\s+visual_edit_undo_receipt_candidate\s*\(/,
    "visual_edit_undo_receipt_candidate",
  );
  const undoStatus = extractBalancedBlock(
    devtoolsProtocol,
    /fn\s+visual_edit_undo_status\s*\(/,
    "visual_edit_undo_status",
  );

  assertSourceMatches(
    sessionPayload,
    /visual_edit_undo_receipts["']?\s*:\s*true[\s\S]*fake_writable_source_claims["']?\s*:\s*false/,
    "devtools session capabilities should claim real undo receipts while still rejecting fake writable source proof",
  );
  assertSourceMatches(
    visualReceiptContract,
    /undo_receipt_required["']?\s*:\s*true[\s\S]*STYLE_UNDO_ENDPOINT[\s\S]*VISUAL_EDIT_RECEIPT_WRITE_STATUS_WRITTEN[\s\S]*STYLE_APPLY_ENDPOINT[\s\S]*VISUAL_EDIT_UNDO_STATUS_PENDING/,
    "visual edit receipt contract should distinguish written undo receipts from pending apply undo proof",
  );
  assertSourceMatches(
    visualReceiptContract,
    /receipt_write_status["']?[\s\S]*writes-only-after-safe-source-mutation[\s\S]*VISUAL_EDIT_RECEIPT_WRITE_STATUS_MISSING/,
    "visual edit receipt contract should distinguish safe apply receipt writes from preview-only non-writes",
  );
  assertSourceMatches(
    visualReceiptCandidate,
    /undo_supported["']?\s*:\s*source_mutated/,
    "apply receipt candidates should expose undo support only after source mutation is true",
  );
  assertSourceMatches(
    visualReceiptCandidate,
    /undo_receipt_status["']?\s*:\s*visual_edit_undo_status\(applied,\s*source_mutated\)/,
    "visual edit receipt candidates should derive undo status from actual apply/source mutation facts",
  );
  assertSourceMatches(
    visualReceiptCandidate,
    /fake_writable_source_claim["']?\s*:\s*false/,
    "visual edit receipt candidates should explicitly reject fake writable source claims",
  );
  assertSourceMatches(
    nonLocalReceiptCandidate,
    /undo_receipt_status["']?\s*:\s*VISUAL_EDIT_UNDO_STATUS_MISSING/,
    "non-local style-apply rejections should not claim pending undo proof",
  );
  assertSourceMatches(
    undoStatus,
    /applied\s*&&\s*source_mutated[\s\S]*VISUAL_EDIT_UNDO_STATUS_PENDING[\s\S]*VISUAL_EDIT_UNDO_STATUS_MISSING/,
    "undo status should become pending only when apply actually mutates source",
  );
  assertSourceMatches(
    undoPayload,
    /write_visual_edit_undo_receipt[\s\S]*visual_edit_undo_receipt_candidate[\s\S]*undone/,
    "style-undo should write a durable undo receipt only after an exact undo mutation succeeds",
  );
  assertSourceMatches(
    undoReceiptCandidate,
    /operation["']?\s*:\s*style_ops::STYLE_UNDO_OPERATION[\s\S]*receipt_written[\s\S]*json-sr-machine-written/,
    "style-undo receipt candidates should carry the undo operation and durable receipt state",
  );
  assertSourceMatches(
    styleOps,
    /pub\(crate\)\s+fn\s+undo_style_change_json[\s\S]*expected_text_after[\s\S]*restore_text_before[\s\S]*undo-source-range-mismatch/,
    "style-undo should restore only the exact authored CSS range captured in the prior receipt",
  );
  assertSourceMatches(
    visualReceiptCandidate,
    /receipt_written[\s\S]*receipt_durability[\s\S]*json-sr-machine-written[\s\S]*candidate-only-not-written/,
    "receipt candidates should claim durable receipt writes only after the write succeeds",
  );
});

test("style-apply mutates only the expected source file after safety checks", () => {
  const devtoolsProtocol = read("dx-www/src/cli/devtools/protocol.rs");
  const styleOps = read("dx-www/src/cli/devtools/style_ops.rs");
  const devtoolsProtocolResponse = extractBalancedBlock(
    devtoolsProtocol,
    /pub\(super\)\s+fn\s+devtools_protocol_response\s*\(/,
    "devtools_protocol_response",
  );
  const localWriteGate = extractBalancedBlock(
    devtoolsProtocol,
    /fn\s+style_apply_local_write_allowed\s*\(/,
    "style_apply_local_write_allowed",
  );
  const nonLocalPayload = extractBalancedBlock(
    devtoolsProtocol,
    /fn\s+style_apply_non_local_payload\s*\(/,
    "style_apply_non_local_payload",
  );
  const loopbackAuthority = extractBalancedBlock(
    devtoolsProtocol,
    /fn\s+is_loopback_authority\s*\(/,
    "is_loopback_authority",
  );
  const applyOperation = extractBalancedBlock(
    styleOps,
    /pub\(crate\)\s+fn\s+apply_style_change\s*\(/,
    "apply_style_change",
  );
  const validatePath = extractBalancedBlock(
    styleOps,
    /fn\s+validate_relative_source_path\s*\(/,
    "validate_relative_source_path",
  );
  const safeProjectPath = extractBalancedBlock(
    styleOps,
    /fn\s+safe_project_path\s*\(/,
    "safe_project_path",
  );
  const sourceEligibility = extractBalancedBlock(
    styleOps,
    /pub\(crate\)\s+fn\s+source_write_eligibility\s*\(/,
    "source_write_eligibility",
  );
  const authoredCssPath = extractBalancedBlock(
    styleOps,
    /fn\s+is_authored_css_path\s*\(/,
    "is_authored_css_path",
  );
  const cssDeclaration = extractBalancedBlock(
    styleOps,
    /fn\s+looks_like_css_declaration\s*\(/,
    "looks_like_css_declaration",
  );
  const declarationPropertyMatch = extractBalancedBlock(
    styleOps,
    /fn\s+declaration_property_matches_validation\s*\(/,
    "declaration_property_matches_validation",
  );
  const applySnippet = `${applyOperation}\n${validatePath}\n${safeProjectPath}\n${sourceEligibility}\n${authoredCssPath}\n${cssDeclaration}\n${declarationPropertyMatch}`;

  assertSourceMatches(applySnippet, /style-apply|style_apply|apply_style_change/i, "style-apply endpoint or handler should exist");
  assertSourceMatches(
    devtoolsProtocolResponse,
    /STYLE_APPLY_ENDPOINT[\s\S]*!style_apply_local_write_allowed\(request_headers\)[\s\S]*style_apply_non_local_payload\(request_headers\)/,
    "style-apply should gate source writes behind a local request check",
  );
  assertSourceMatches(
    localWriteGate,
    /headers\.get\("host"\)[\s\S]*is_loopback_authority\(host\)[\s\S]*for\s+header\s+in\s+\["origin",\s*"referer"\]/,
    "style-apply local gate should require loopback Host and validate Origin/Referer when present",
  );
  assertSourceMatches(
    loopbackAuthority,
    /parse::<std::net::IpAddr>\(\)[\s\S]*is_loopback\(\)/,
    "style-apply local gate should use parsed loopback IPs instead of hostname prefixes",
  );
  assertSourceDoesNotMatch(loopbackAuthority, /starts_with\("127\."\)/, "style-apply local gate should reject 127.* lookalike hostnames");
  assertSourceMatches(nonLocalPayload, /non-local-devtools-write/, "style-apply non-local rejections should be explicit");
  assertSourceMatches(
    applySnippet,
    /canonicalize|strip_prefix|ancestors\(\)|components\(\)|Component::Normal/,
    "style-apply should validate the requested file is inside the project source boundary",
  );
  assertSourceMatches(
    applySnippet,
    /expected_source|source_path|target_file|target_path|allowed_source/i,
    "style-apply should name and constrain the exact source file it will mutate",
  );
  assertSourceMatches(
    sourceEligibility,
    /target\.kind\s*!=\s*DxSourceTargetKind::AuthoredCss/,
    "style-apply should not treat arbitrary source target kinds as writable",
  );
  assertSourceMatches(
    sourceEligibility,
    /!is_authored_css_path\(&relative_path\)/,
    "style-apply should only write authored CSS-like source paths",
  );
  assertSourceMatches(
    sourceEligibility,
    /!looks_like_css_declaration\(&range\.expected_text\)/,
    "style-apply should require a declaration-shaped expected source range",
  );
  assertSourceMatches(
    applyOperation,
    /declaration_property_matches_validation\(&range\.expected_text,\s*&validation\)/,
    "style-apply should reject exact ranges whose declaration property differs from the requested property",
  );
  assertSourceMatches(
    applySnippet,
    /\b(?:std::fs::write|fs::write)\b|\.write_all\(|\bFile::create\b|\bOpenOptions::new\b/,
    "style-apply should perform one explicit source write when the request is safe",
  );
  assertSourceDoesNotMatch(
    applySnippet,
    /\bremove_file\b|\bremove_dir\b|\bremove_dir_all\b|\brename\b|\bcopy\b|\bcreate_dir_all\b/,
    "style-apply should not perform broad filesystem mutation",
  );
});
