const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "vercel-ai");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("Vercel AI slice exposes real ToolLoopAgent route helpers from upstream API", () => {
  const upstreamAgentDocs = read(
    path.join(mirror, "content", "docs", "07-reference", "01-ai-sdk-core", "16-tool-loop-agent.mdx"),
  );
  const upstreamResponseDocs = read(
    path.join(
      mirror,
      "content",
      "docs",
      "07-reference",
      "01-ai-sdk-core",
      "18-create-agent-ui-stream-response.mdx",
    ),
  );
  const upstreamNextRoute = read(
    path.join(mirror, "examples", "next-agent", "app", "api", "chat", "route.ts"),
  );
  const upstreamNextAgent = read(
    path.join(mirror, "examples", "next-agent", "agent", "weather-agent.ts"),
  );
  const upstreamAgentIndex = read(path.join(mirror, "packages", "ai", "src", "agent", "index.ts"));
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_vercel_ai.rs"));
  const launchProof = read(path.join(root, "examples", "template", "ai-chat-status.tsx"));
  const security = read(path.join(root, "core", "src", "ecosystem", "forge_security.rs"));

  assert.match(upstreamAgentDocs, /ToolLoopAgent/);
  assert.match(upstreamAgentDocs, /instructions/);
  assert.match(upstreamAgentDocs, /stopWhen/);
  assert.match(upstreamResponseDocs, /createAgentUIStreamResponse/);
  assert.match(upstreamNextRoute, /createAgentUIStreamResponse/);
  assert.match(upstreamNextAgent, /new ToolLoopAgent/);
  assert.match(upstreamNextAgent, /InferAgentUIMessage/);
  assert.match(upstreamAgentIndex, /ToolLoopAgent/);
  assert.match(upstreamAgentIndex, /InferAgentUIMessage/);
  assert.match(upstreamAgentIndex, /createAgentUIStreamResponse/);

  assert.match(slice, /"js\/lib\/ai\/agent\.ts"/);
  assert.match(slice, /"js\/app\/api\/ai\/agent\/route\.ts"/);
  assert.match(slice, /ToolLoopAgent/);
  assert.match(slice, /InferAgentUIMessage/);
  assert.match(slice, /createAgentUIStreamResponse/);
  assert.match(slice, /createDxLaunchAgent/);
  assert.match(slice, /createDxAgentRoute/);
  assert.match(slice, /agentRoute: "createDxAgentRoute/);

  assert.match(launchProof, /createDxLaunchAgent/);
  assert.match(launchProof, /data-dx-ai-agent/);
  assert.match(security, /agent route/);
});
