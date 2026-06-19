const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "vercel-ai");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("Vercel AI slice exposes real model policy middleware helpers from upstream API", () => {
  const upstreamMiddlewareDocs = read(
    path.join(mirror, "content", "docs", "03-ai-sdk-core", "40-middleware.mdx"),
  );
  const upstreamProviderDocs = read(
    path.join(mirror, "content", "docs", "03-ai-sdk-core", "45-provider-management.mdx"),
  );
  const upstreamReference = read(
    path.join(mirror, "content", "docs", "07-reference", "01-ai-sdk-core", "68-default-settings-middleware.mdx"),
  );
  const upstreamIndex = read(path.join(mirror, "packages", "ai", "src", "index.ts"));
  const upstreamMiddlewareIndex = read(
    path.join(mirror, "packages", "ai", "src", "middleware", "index.ts"),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_vercel_ai.rs"));
  const launchProof = read(path.join(root, "examples", "template", "ai-chat-status.tsx"));
  const security = read(path.join(root, "core", "src", "ecosystem", "forge_security.rs"));

  assert.match(upstreamMiddlewareDocs, /wrapLanguageModel/);
  assert.match(upstreamMiddlewareDocs, /defaultSettingsMiddleware/);
  assert.match(upstreamProviderDocs, /wrapLanguageModel/);
  assert.match(upstreamProviderDocs, /defaultSettingsMiddleware/);
  assert.match(upstreamReference, /defaultSettingsMiddleware/);
  assert.match(upstreamIndex, /export \* from '\.\/middleware'/);
  assert.match(upstreamMiddlewareIndex, /defaultSettingsMiddleware/);
  assert.match(upstreamMiddlewareIndex, /wrapLanguageModel/);

  assert.match(slice, /"js\/lib\/ai\/model-policy\.ts"/);
  assert.match(slice, /wrapLanguageModel/);
  assert.match(slice, /defaultSettingsMiddleware/);
  assert.match(slice, /type LanguageModel/);
  assert.match(slice, /type LanguageModelMiddleware/);
  assert.match(slice, /createDxLaunchModelPolicy/);
  assert.match(slice, /withDxLaunchModelPolicy/);
  assert.match(slice, /modelPolicy: "withDxLaunchModelPolicy/);

  assert.match(launchProof, /withDxLaunchModelPolicy/);
  assert.match(launchProof, /data-dx-ai-model-policy/);
  assert.match(security, /model policy/);
});
