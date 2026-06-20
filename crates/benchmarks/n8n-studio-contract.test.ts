import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const studioRoot = path.join(repoRoot, "examples", "n8n-studio");
const manifestPath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "dx-node-source-.dx/build-cache/manifest.json",
);

function readStudioFile(relativePath) {
  return fs.readFileSync(path.join(studioRoot, relativePath), "utf8");
}

function studioFiles() {
  const result = [];
  const visit = (directory) => {
    for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
      const entryPath = path.join(directory, entry.name);
      if (entry.isDirectory()) {
        visit(entryPath);
      } else {
        result.push(entryPath);
      }
    }
  };
  visit(studioRoot);
  return result;
}

test("n8n Studio follows the DX-WWW app contract", () => {
  const dx = readStudioFile("dx");
  assert.match(dx, /www\(\s*app_dir=app/s);
  assert.match(dx, /style\(\s*mode=generated-css/s);
  assert.match(dx, /icons\(component=Icon source_tag=icon runtime_tag=dx-icon/s);
  assert.match(dx, /forge\(policy=forge-first-no-node-modules\)/);
  assert.match(dx, /check\(score_scale=500 lighthouse=true\)/);

  const layout = readStudioFile(path.join("app", "layout.tsx"));
  assert.match(layout, /import "\.\.\/styles\/globals\.css"/);

  const globals = readStudioFile(path.join("styles", "globals.css"));
  assert.match(globals, /@import "\.\/theme\.css"/);
  assert.match(globals, /@import "\.\/generated\.css"/);
});

test("n8n Studio catalog stays tied to the local n8n source manifest", () => {
  const manifest = JSON.parse(fs.readFileSync(manifestPath, "utf8"));
  const catalog = readStudioFile(path.join("lib", "n8n-studio", "catalog.ts"));

  assert.equal(manifest.counts.node_files, 536);
  assert.equal(manifest.counts.credential_files, 396);
  assert.match(catalog, /nodeFileCount: 536/);
  assert.match(catalog, /credentialFileCount: 396/);

  for (const nodeName of ["ManualTrigger", "Webhook", "HttpRequest", "Slack", "Gmail", "OpenAi"]) {
    assert(
      manifest.nodes.some((node) => node.name === nodeName),
      `source manifest should include ${nodeName}`,
    );
    assert.match(catalog, new RegExp(`name: "${nodeName}"`));
  }
});

test("n8n Studio exposes the required editor surfaces", () => {
  const app = readStudioFile(path.join("components", "n8n-studio", "n8n-studio-app.tsx"));
  const receipts = readStudioFile(path.join("lib", "n8n-studio", "receipts.ts"));

  for (const surface of [
    "node-creator",
    "workflow-canvas",
    "node-parameters",
    "expression-editor",
    "credentials",
    "resource-locator",
    "pinned-data",
    "execution-debug",
    "ai-tools",
    "import-export",
    "receipts",
  ]) {
    assert.match(`${app}\n${receipts}`, new RegExp(surface));
  }
});

test("n8n Studio keeps live execution and credential secrets behind boundaries", () => {
  const allSource = studioFiles()
    .filter((file) => /\.(ts|tsx|css|md|json)$/.test(file))
    .map((file) => fs.readFileSync(file, "utf8"))
    .join("\n");

  assert.match(allSource, /providerBoundary: true/);
  assert.match(allSource, /liveProviderExecution: false/);
  assert.match(allSource, /secret-values-never-included/);
  assert.doesNotMatch(allSource, /apiKey\s*[:=]\s*["'][^"']+["']/i);
  assert.doesNotMatch(allSource, /password\s*[:=]\s*["'][^"']+["']/i);
});

test("n8n Studio does not import upstream UI runtimes or template-local packages", () => {
  const files = studioFiles();
  assert(
    files.every((file) => !/[\\/]node_modules[\\/]/.test(file)),
    "n8n Studio must not contain template-local node_modules",
  );
  assert(
    files.every((file) => !/\.(js|jsx|cjs|mjs)$/.test(file)),
    "n8n Studio source files should stay TypeScript, TSX, CSS, JSON, Markdown, or assets",
  );

  const allSource = files
    .filter((file) => /\.(ts|tsx|css|md|json)$/.test(file))
    .map((file) => fs.readFileSync(file, "utf8"))
    .join("\n");

  assert.doesNotMatch(allSource, /@vue-flow|vue|pinia|vite|reactflow|xyflow|lucide-react/);
  assert.doesNotMatch(allSource, /"dependencies"\s*:/);
});

