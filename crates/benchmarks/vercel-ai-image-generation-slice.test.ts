const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "vercel-ai");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("Vercel AI slice exposes real image generation helpers from upstream API", () => {
  const upstreamDocs = read(
    path.join(
      mirror,
      "content",
      "docs",
      "07-reference",
      "01-ai-sdk-core",
      "10-generate-image.mdx",
    ),
  );
  const upstreamSource = read(
    path.join(mirror, "packages", "ai", "src", "generate-image", "generate-image.ts"),
  );
  const upstreamResult = read(
    path.join(
      mirror,
      "packages",
      "ai",
      "src",
      "generate-image",
      "generate-image-result.ts",
    ),
  );
  const upstreamExample = read(
    path.join(
      mirror,
      "examples",
      "ai-functions",
      "src",
      "generate-image",
      "openai",
      "gpt-image.ts",
    ),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_vercel_ai.rs"));
  const launchProof = read(path.join(root, "examples", "template", "ai-chat-status.tsx"));
  const security = read(path.join(root, "core", "src", "ecosystem", "forge_security.rs"));

  assert.match(upstreamDocs, /generateImage/);
  assert.match(upstreamDocs, /ImageModelV4/);
  assert.match(upstreamDocs, /GeneratedFile/);
  assert.match(upstreamDocs, /providerMetadata/);
  assert.match(upstreamSource, /export async function generateImage/);
  assert.match(upstreamResult, /interface GenerateImageResult/);
  assert.match(upstreamExample, /openai\.image/);

  assert.match(slice, /"js\/lib\/ai\/image-generation\.ts"/);
  assert.match(slice, /"js\/app\/api\/ai\/image\/route\.ts"/);
  assert.match(slice, /generateImage/);
  assert.match(slice, /ImageModel/);
  assert.match(slice, /GenerateImageResult/);
  assert.match(slice, /generateDxLaunchImageAsset/);
  assert.match(slice, /imageGeneration: "generateDxLaunchImageAsset/);

  assert.match(launchProof, /generateDxLaunchImageAsset/);
  assert.match(launchProof, /data-dx-ai-image-generation/);
  assert.match(security, /image generation/);
});
