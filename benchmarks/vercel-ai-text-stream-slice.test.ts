const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "vercel-ai");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("Vercel AI slice exposes real text stream response helpers", () => {
  const upstreamIndex = read(
    path.join(mirror, "packages", "ai", "src", "text-stream", "index.ts"),
  );
  const upstreamCreate = read(
    path.join(
      mirror,
      "packages",
      "ai",
      "src",
      "text-stream",
      "create-text-stream-response.ts",
    ),
  );
  const upstreamPipe = read(
    path.join(
      mirror,
      "packages",
      "ai",
      "src",
      "text-stream",
      "pipe-text-stream-to-response.ts",
    ),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_vercel_ai.rs"));
  const launchProof = read(path.join(root, "examples", "template", "ai-chat-status.tsx"));
  const security = read(path.join(root, "core", "src", "ecosystem", "forge_security.rs"));

  assert.match(upstreamIndex, /export \{ createTextStreamResponse \}/);
  assert.match(upstreamIndex, /export \{ pipeTextStreamToResponse \}/);
  assert.match(upstreamCreate, /export function createTextStreamResponse/);
  assert.match(upstreamCreate, /text\/plain; charset=utf-8/);
  assert.match(upstreamPipe, /export function pipeTextStreamToResponse/);

  assert.match(slice, /"js\/lib\/ai\/text-stream\.ts"/);
  assert.match(slice, /"js\/app\/api\/ai\/text-stream\/route\.ts"/);
  assert.match(slice, /createTextStreamResponse/);
  assert.match(slice, /pipeTextStreamToResponse/);
  assert.match(slice, /createDxLaunchTextStream/);
  assert.match(slice, /createDxLaunchTextStreamResponse/);
  assert.match(slice, /textStream: "createDxLaunchTextStream/);

  assert.match(launchProof, /createDxLaunchTextStream/);
  assert.match(launchProof, /data-dx-ai-text-stream/);
  assert.match(security, /text stream/);
});
