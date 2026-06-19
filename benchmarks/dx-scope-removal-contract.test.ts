import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

function walkFiles(root, visitor) {
  for (const entry of fs.readdirSync(root, { withFileTypes: true })) {
    const absolutePath = path.join(root, entry.name);
    if (entry.isDirectory()) {
      if ([".git", "target", "node_modules", "vendor"].includes(entry.name)) {
        continue;
      }
      walkFiles(absolutePath, visitor);
      continue;
    }
    visitor(absolutePath);
  }
}

test("DX-WWW active scope removes Next DevTools clone and Turbopack adoption targets", () => {
  const activeStatusFiles = [
    "README.md",
    "DX.md",
    "TODO.md",
    "CHANGELOG.md",
    "docs/NEXTJS_COMPATIBILITY_MAP.md",
  ];
  const activeStatusText = activeStatusFiles
    .map((file) => `--- ${file}\n${read(file)}`)
    .join("\n");
  const devFeedback = read("dx-www/src/dev/dev_feedback.rs");
  const hotReloadProtocol = read("dx-www/src/hot_reload_protocol.rs");
  const buildReceipt = read("dx-www/src/build/source_engine/receipt.rs");

  for (const forbidden of [
    "DX-WWW DevTools",
    "Next DevTools",
    "next-devtools",
    "dev-overlay",
    "/_dx/devtools",
    "DevTools/build diagnostics",
    "source-safe DevTools code frames",
    "external DevTools runtime",
  ]) {
    assert.ok(
      !activeStatusText.includes(forbidden),
      `active status docs must not keep ${forbidden} as a target or comparison`,
    );
  }
  assert.doesNotMatch(
    activeStatusText,
    /Next\s+DevTools/,
    "active status docs must not preserve split Next DevTools wording",
  );

  assert.ok(
    !fs.existsSync(path.join(repoRoot, "dx-www/src/dev/devtools.rs")),
    "Next DevTools clone module name must not stay active",
  );
  assert.ok(
    !fs.existsSync(path.join(repoRoot, "core/src/devtools.rs")),
    "full DevTools extension surface must not stay active in the compiler crate",
  );
  assert.doesNotMatch(
    read("core/src/lib.rs"),
    /pub\s+mod\s+devtools\b/,
    "compiler public API must not export the removed full DevTools extension surface",
  );
  assert.match(devFeedback, /DX_DEV_FEEDBACK_ROOT_ENDPOINT: &str = "\/_dx\/feedback"/);
  assert.match(devFeedback, /"schema": "dx\.dev_feedback\.hmr"/);
  assert.match(devFeedback, /"next_runtime": false/);
  assert.match(devFeedback, /"turbopack_hmr": false/);
  assert.match(hotReloadProtocol, /DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT/);

  assert.doesNotMatch(activeStatusText, /Turbopack powers|powers `dx build`|powers DX/i);
  assert.doesNotMatch(
    activeStatusText,
    /(?:Turbopack|external\s+bundler)\s+(?:runtime\s+)?(?:execution|adoption)\s+(?:proof|remain[s]?\s+unclaimed)/i,
    "active status docs must not keep external bundler execution/adoption as a pending proof target",
  );
  assert.doesNotMatch(
    activeStatusText,
    /(?:Turbopack|external\s+bundler)[^\n.]*runtime[^\n.]*adoption[^\n.]*remain[s]?\s+unclaimed/i,
    "active status docs must not include external bundler runtime adoption in remaining-proof lists",
  );
  assert.match(buildReceipt, /source snapshot reference only/);
  assert.match(buildReceipt, /adapter-boundary/);
  assert.match(buildReceipt, /informed_by/);
  assert.match(buildReceipt, /outside DX-WWW runtime\/build scope/);
  assert.doesNotMatch(buildReceipt, /Turbopack powers|powers `dx build`|powers DX/i);
  assert.doesNotMatch(buildReceipt, /Turbopack Image, or Next\.js runtime parity remain explicit governed follow-ups/);
});

test("raw Turbopack chunk URLs are removed from benchmark artifacts", () => {
  const offenders = [];
  const turbopackChunkMarkers = [
    "_next/static/chunks/" + "turbopack",
    "_next\\\\/static\\\\/chunks\\\\/" + "turbopack",
    "_next\\/static\\/chunks\\/" + "turbopack",
  ];
  walkFiles(path.join(repoRoot, "benchmarks"), (absolutePath) => {
    const relativePath = path.relative(repoRoot, absolutePath).replaceAll(path.sep, "/");
    const source = fs.readFileSync(absolutePath, "utf8");
    if (turbopackChunkMarkers.some((marker) => source.includes(marker))) {
      offenders.push(relativePath);
    }
  });

  assert.deepEqual(offenders, []);
});
