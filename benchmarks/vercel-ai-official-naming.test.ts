const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "vercel-ai");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readMirror(relativePath) {
  return fs.readFileSync(path.join(mirror, relativePath), "utf8");
}

test("AI SDK lane uses official DX naming with Vercel AI provenance metadata", () => {
  const upstreamPackage = JSON.parse(readMirror("packages/ai/package.json"));
  const upstreamIndex = readMirror("packages/ai/src/index.ts");
  const upstreamGenerateText = readMirror("packages/ai/src/generate-text/index.ts");
  const upstreamUi = readMirror("packages/ai/src/ui/index.ts");
  const upstreamRegistry = readMirror("packages/ai/src/registry/index.ts");

  const forge = read("core/src/ecosystem/forge_vercel_ai.rs");
  const catalog = read("examples/template/package-catalog.ts");
  const packageDoc = read("docs/packages/ai-vercel-ai.md");
  const cli = read("dx-www/src/cli/mod.rs");
  const studio = read("dx-www/src/cli/studio_manifest.rs");
  const dashboardWorkflow = read("examples/dashboard/src/components/AiLaunchAssistant.tsx");
  const generatedWorkflow = read("core/src/ecosystem/forge_vercel_ai.rs");
  const frameworkStructure = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const receipt = JSON.parse(
    read("examples/template/.dx/forge/receipts/2026-05-22-ai-vercel-ai-launch-assistant.json"),
  );

  assert.equal(upstreamPackage.name, "ai");
  assert.equal(upstreamPackage.version, "7.0.0-canary.146");
  assert.match(upstreamPackage.description, /AI SDK by Vercel/);
  assert.match(upstreamIndex, /export \* from '\.\/generate-text'/);
  assert.match(upstreamGenerateText, /streamText/);
  assert.match(upstreamUi, /DefaultChatTransport/);
  assert.match(upstreamRegistry, /createProviderRegistry/);

  assert.match(forge, /officialName: "AI SDK"/);
  assert.match(forge, /officialPackageName: "AI SDK"/);
  assert.match(forge, /upstreamPackage: "ai"/);
  assert.match(forge, /upstreamVersion: "7\.0\.0-canary\.146"/);
  assert.match(forge, /inspectedSourceFiles: \[/);
  assert.match(forge, /"packages\/ai\/src\/generate-text\/index\.ts"/);
  assert.match(forge, /selectedSurfaces: \[/);
  assert.match(forge, /honestyLabel: "ADAPTER-BOUNDARY"/);

  assert.match(catalog, /packageId: "ai\/vercel-ai",\s+officialName: "AI SDK"/);
  assert.match(catalog, /officialPackageName: "AI SDK"/);
  assert.match(catalog, /upstreamPackage: "ai"/);
  assert.match(catalog, /upstreamVersion: "7\.0\.0-canary\.146"/);
  assert.match(catalog, /sourceMirror: "G:\/WWW\/inspirations\/vercel-ai"/);
  assert.match(catalog, /inspectedSourceFiles: \[/);
  assert.match(catalog, /selectedSurfaces: \[/);
  assert.match(catalog, /honestyLabel: "ADAPTER-BOUNDARY"/);
  assert.match(catalog, /dxCheckVisibility: \{/);
  assert.match(catalog, /"ai\/vercel-ai": \{\s+name: "AI SDK Launch Assistant"/);

  assert.match(cli, /"official_name": "AI SDK"/);
  assert.match(cli, /"official_dx_package_name": "AI SDK"/);
  assert.match(cli, /"upstream_package": "ai"/);
  assert.match(cli, /"upstream_version": "7\.0\.0-canary\.146"/);

  assert.match(studio, /"front_facing_name": "AI SDK Launch Assistant"/);
  assert.match(studio, /"official_dx_package_name": "AI SDK"/);
  assert.match(studio, /"upstream_package": "ai"/);
  assert.doesNotMatch(studio, /"front_facing_name": "Vercel AI Launch Assistant"/);

  assert.equal(receipt.package_name, "AI SDK");
  assert.equal(receipt.official_dx_package_name, "AI SDK");
  assert.equal(receipt.upstream_package, "ai");
  assert.equal(receipt.upstream_version, "7.0.0-canary.146");
  assert.equal(receipt.based_on, "Vercel AI SDK");

  assert.match(packageDoc, /^# AI SDK Forge Package/);
  assert.match(packageDoc, /Official DX package name: `AI SDK`/);
  assert.match(packageDoc, /Upstream package: `ai`/);
  assert.match(packageDoc, /Vercel AI SDK, Apache-2\.0/);

  assert.match(dashboardWorkflow, /<h2>AI SDK launch assistant<\/h2>/);
  assert.doesNotMatch(dashboardWorkflow, /Vercel AI SDK-shaped boundary/);
  assert.match(generatedWorkflow, /<h2>AI SDK launch assistant<\/h2>/);
  assert.doesNotMatch(generatedWorkflow, /Vercel AI SDK-shaped boundary/);
  assert.match(frameworkStructure, /Lane 16 uses the official front-facing package name `AI SDK`/);
});
