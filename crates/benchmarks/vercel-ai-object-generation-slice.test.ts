const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "vercel-ai");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("Vercel AI slice exposes real object generation compatibility helpers", () => {
  const upstreamIndex = read(
    path.join(mirror, "packages", "ai", "src", "generate-object", "index.ts"),
  );
  const upstreamGenerate = read(
    path.join(
      mirror,
      "packages",
      "ai",
      "src",
      "generate-object",
      "generate-object.ts",
    ),
  );
  const upstreamStream = read(
    path.join(
      mirror,
      "packages",
      "ai",
      "src",
      "generate-object",
      "stream-object.ts",
    ),
  );
  const upstreamGenerateResult = read(
    path.join(
      mirror,
      "packages",
      "ai",
      "src",
      "generate-object",
      "generate-object-result.ts",
    ),
  );
  const upstreamStreamResult = read(
    path.join(
      mirror,
      "packages",
      "ai",
      "src",
      "generate-object",
      "stream-object-result.ts",
    ),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_vercel_ai.rs"));
  const launchProof = read(path.join(root, "examples", "template", "ai-chat-status.tsx"));
  const security = read(path.join(root, "core", "src", "ecosystem", "forge_security.rs"));

  assert.match(upstreamIndex, /export \{ generateObject \}/);
  assert.match(upstreamIndex, /export \{ streamObject \}/);
  assert.match(upstreamIndex, /GenerateObjectResult/);
  assert.match(upstreamIndex, /StreamObjectResult/);
  assert.match(upstreamGenerate, /export async function generateObject/);
  assert.match(upstreamGenerate, /@deprecated Use `generateText` with an `output` setting instead/);
  assert.match(upstreamStream, /export function streamObject/);
  assert.match(upstreamGenerateResult, /interface GenerateObjectResult/);
  assert.match(upstreamStreamResult, /interface StreamObjectResult/);

  assert.match(slice, /"js\/lib\/ai\/object-generation\.ts"/);
  assert.match(slice, /"js\/app\/api\/ai\/object\/route\.ts"/);
  assert.match(slice, /generateObject/);
  assert.match(slice, /streamObject/);
  assert.match(slice, /GenerateObjectResult/);
  assert.match(slice, /StreamObjectResult/);
  assert.match(slice, /generateDxLaunchObject/);
  assert.match(slice, /streamDxLaunchObject/);
  assert.match(slice, /objectGeneration: "generateDxLaunchObject/);

  assert.match(launchProof, /generateDxLaunchObject/);
  assert.match(launchProof, /data-dx-ai-object-generation/);
  assert.match(security, /object generation compatibility/);
});
