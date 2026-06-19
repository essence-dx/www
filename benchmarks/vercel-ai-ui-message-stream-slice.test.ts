const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "vercel-ai");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("Vercel AI slice exposes real UI message stream helpers", () => {
  const upstreamIndex = read(
    path.join(mirror, "packages", "ai", "src", "ui-message-stream", "index.ts"),
  );
  const upstreamCreate = read(
    path.join(
      mirror,
      "packages",
      "ai",
      "src",
      "ui-message-stream",
      "create-ui-message-stream.ts",
    ),
  );
  const upstreamResponse = read(
    path.join(
      mirror,
      "packages",
      "ai",
      "src",
      "ui-message-stream",
      "create-ui-message-stream-response.ts",
    ),
  );
  const upstreamRead = read(
    path.join(
      mirror,
      "packages",
      "ai",
      "src",
      "ui-message-stream",
      "read-ui-message-stream.ts",
    ),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_vercel_ai.rs"));
  const launchProof = read(path.join(root, "examples", "template", "ai-chat-status.tsx"));
  const security = read(path.join(root, "core", "src", "ecosystem", "forge_security.rs"));

  assert.match(upstreamIndex, /export \{ createUIMessageStream \}/);
  assert.match(upstreamIndex, /export \{ createUIMessageStreamResponse \}/);
  assert.match(upstreamIndex, /export \{ pipeUIMessageStreamToResponse \}/);
  assert.match(upstreamIndex, /export \{ readUIMessageStream \}/);
  assert.match(upstreamIndex, /UI_MESSAGE_STREAM_HEADERS/);
  assert.match(upstreamCreate, /export function createUIMessageStream/);
  assert.match(upstreamResponse, /export function createUIMessageStreamResponse/);
  assert.match(upstreamRead, /export function readUIMessageStream/);

  assert.match(slice, /"js\/lib\/ai\/ui-message-stream\.ts"/);
  assert.match(slice, /"js\/app\/api\/ai\/ui-stream\/route\.ts"/);
  assert.match(slice, /createUIMessageStream/);
  assert.match(slice, /createUIMessageStreamResponse/);
  assert.match(slice, /pipeUIMessageStreamToResponse/);
  assert.match(slice, /readUIMessageStream/);
  assert.match(slice, /UI_MESSAGE_STREAM_HEADERS/);
  assert.match(slice, /createDxLaunchUIMessageStream/);
  assert.match(slice, /createDxLaunchUIMessageStreamResponse/);
  assert.match(slice, /uiMessageStream: "createDxLaunchUIMessageStream/);

  assert.match(launchProof, /createDxLaunchUIMessageStream/);
  assert.match(launchProof, /data-dx-ai-ui-message-stream/);
  assert.match(security, /UI message stream/);
});
