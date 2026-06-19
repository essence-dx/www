const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "vercel-ai");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("Vercel AI slice exposes real provider freedom helpers from upstream API", () => {
  const upstreamDocs = read(
    path.join(mirror, "content", "docs", "03-ai-sdk-core", "45-provider-management.mdx"),
  );
  const upstreamRegistryIndex = read(
    path.join(mirror, "packages", "ai", "src", "registry", "index.ts"),
  );
  const upstreamIndex = read(path.join(mirror, "packages", "ai", "src", "index.ts"));
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_vercel_ai.rs"));
  const launchProof = read(path.join(root, "examples", "template", "ai-chat-status.tsx"));
  const security = read(path.join(root, "core", "src", "ecosystem", "forge_security.rs"));

  assert.match(upstreamDocs, /customProvider/);
  assert.match(upstreamDocs, /createProviderRegistry/);
  assert.match(upstreamDocs, /gateway\.embeddingModel/);
  assert.match(upstreamRegistryIndex, /export \{ customProvider \}/);
  assert.match(upstreamRegistryIndex, /createProviderRegistry/);
  assert.match(upstreamIndex, /createGateway, gateway/);

  assert.match(slice, /"js\/lib\/ai\/provider-freedom\.ts"/);
  assert.match(slice, /createProviderRegistry/);
  assert.match(slice, /customProvider/);
  assert.match(slice, /gateway/);
  assert.match(slice, /createDxGatewayProvider/);
  assert.match(slice, /createDxProviderRegistry/);
  assert.match(slice, /createDxLaunchProvider/);
  assert.match(slice, /providerFreedom: "createDxProviderRegistry/);

  assert.match(launchProof, /createDxProviderRegistry/);
  assert.match(launchProof, /data-dx-ai-provider-freedom/);
  assert.match(security, /provider registry/);
});
