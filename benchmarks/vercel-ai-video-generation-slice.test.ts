const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "vercel-ai");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("Vercel AI slice exposes real video generation helpers from upstream API", () => {
  const upstreamDocs = read(
    path.join(
      mirror,
      "content",
      "docs",
      "07-reference",
      "01-ai-sdk-core",
      "13-generate-video.mdx",
    ),
  );
  const upstreamIndex = read(
    path.join(mirror, "packages", "ai", "src", "generate-video", "index.ts"),
  );
  const upstreamSource = read(
    path.join(
      mirror,
      "packages",
      "ai",
      "src",
      "generate-video",
      "generate-video.ts",
    ),
  );
  const upstreamResult = read(
    path.join(
      mirror,
      "packages",
      "ai",
      "src",
      "generate-video",
      "generate-video-result.ts",
    ),
  );
  const upstreamExample = read(
    path.join(
      mirror,
      "examples",
      "ai-functions",
      "src",
      "generate-video",
      "fal",
      "luma.ts",
    ),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_vercel_ai.rs"));
  const launchProof = read(path.join(root, "examples", "onboard", "ai-chat-status.tsx"));
  const security = read(path.join(root, "core", "src", "ecosystem", "forge_security.rs"));

  assert.match(upstreamDocs, /experimental_generateVideo/);
  assert.match(upstreamDocs, /VideoModelV4/);
  assert.match(upstreamDocs, /GenerateVideoPrompt/);
  assert.match(upstreamDocs, /createDownload/);
  assert.match(upstreamIndex, /experimental_generateVideo/);
  assert.match(upstreamIndex, /GenerateVideoResult/);
  assert.match(upstreamSource, /export async function experimental_generateVideo/);
  assert.match(upstreamSource, /createDownload/);
  assert.match(upstreamResult, /interface GenerateVideoResult/);
  assert.match(upstreamExample, /fal\.video/);

  assert.match(slice, /"js\/lib\/ai\/video-generation\.ts"/);
  assert.match(slice, /"js\/app\/api\/ai\/video\/route\.ts"/);
  assert.match(slice, /experimental_generateVideo as generateVideo/);
  assert.match(slice, /GenerateVideoResult/);
  assert.match(slice, /GenerateVideoPrompt/);
  assert.match(slice, /VideoModel/);
  assert.match(slice, /createDownload/);
  assert.match(slice, /generateDxLaunchVideoAsset/);
  assert.match(slice, /videoGeneration: "generateDxLaunchVideoAsset/);

  assert.match(launchProof, /generateDxLaunchVideoAsset/);
  assert.match(launchProof, /data-dx-ai-video-generation/);
  assert.match(security, /video generation/);
});
