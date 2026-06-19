const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "vercel-ai");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("Vercel AI slice exposes real message pruning helpers from upstream API", () => {
  const upstreamDocs = read(
    path.join(
      mirror,
      "content",
      "docs",
      "07-reference",
      "02-ai-sdk-ui",
      "32-prune-messages.mdx",
    ),
  );
  const upstreamSource = read(
    path.join(mirror, "packages", "ai", "src", "generate-text", "prune-messages.ts"),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_vercel_ai.rs"));
  const launchProof = read(path.join(root, "examples", "template", "ai-chat-status.tsx"));
  const security = read(path.join(root, "core", "src", "ecosystem", "forge_security.rs"));

  assert.match(upstreamDocs, /pruneMessages/);
  assert.match(upstreamDocs, /reasoning: 'before-last-message'/);
  assert.match(upstreamDocs, /toolCalls: 'before-last-2-messages'/);
  assert.match(upstreamDocs, /emptyMessages: 'remove'/);
  assert.match(upstreamSource, /export function pruneMessages/);
  assert.match(upstreamSource, /ModelMessage\[\]/);
  assert.match(upstreamSource, /tool-approval-request/);
  assert.match(upstreamSource, /tool-approval-response/);

  assert.match(slice, /"js\/lib\/ai\/message-pruning\.ts"/);
  assert.match(slice, /pruneMessages/);
  assert.match(slice, /ModelMessage/);
  assert.match(slice, /createDxLaunchMessagePruner/);
  assert.match(slice, /messagePruning: "createDxLaunchMessagePruner/);
  assert.match(slice, /messagePruner\?/);

  assert.match(launchProof, /createDxLaunchMessagePruner/);
  assert.match(launchProof, /data-dx-ai-message-pruning/);
  assert.match(security, /message pruning/);
});
