const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "vercel-ai");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("Vercel AI slice exposes real tool approval policy helpers from upstream API", () => {
  const upstreamApprovalConfig = read(
    path.join(
      mirror,
      "packages",
      "ai",
      "src",
      "generate-text",
      "tool-approval-configuration.ts",
    ),
  );
  const upstreamApprovalRequest = read(
    path.join(
      mirror,
      "packages",
      "ai",
      "src",
      "generate-text",
      "tool-approval-request-output.ts",
    ),
  );
  const upstreamApprovalResponse = read(
    path.join(
      mirror,
      "packages",
      "ai",
      "src",
      "generate-text",
      "tool-approval-response-output.ts",
    ),
  );
  const upstreamExample = read(
    path.join(
      mirror,
      "examples",
      "ai-functions",
      "src",
      "generate-text",
      "openai",
      "tool-approval.ts",
    ),
  );
  const upstreamIndex = read(path.join(mirror, "packages", "ai", "src", "index.ts"));
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_vercel_ai.rs"));
  const launchProof = read(path.join(root, "examples", "template", "ai-chat-status.tsx"));
  const security = read(path.join(root, "core", "src", "ecosystem", "forge_security.rs"));

  assert.match(upstreamApprovalConfig, /ToolApprovalConfiguration/);
  assert.match(upstreamApprovalConfig, /ToolApprovalStatus/);
  assert.match(upstreamApprovalConfig, /user-approval/);
  assert.match(upstreamApprovalRequest, /ToolApprovalRequestOutput/);
  assert.match(upstreamApprovalResponse, /ToolApprovalResponseOutput/);
  assert.match(upstreamExample, /toolApproval/);
  assert.match(upstreamExample, /ToolApprovalResponse/);
  assert.match(upstreamIndex, /type ToolApprovalRequest/);
  assert.match(upstreamIndex, /type ToolApprovalResponse/);

  assert.match(slice, /"js\/lib\/ai\/tool-approval\.ts"/);
  assert.match(slice, /ToolApprovalConfiguration/);
  assert.match(slice, /ToolApprovalRequest/);
  assert.match(slice, /ToolApprovalResponse/);
  assert.match(slice, /createDxLaunchToolApproval/);
  assert.match(slice, /createDxToolApprovalResponse/);
  assert.match(slice, /toolApproval: "createDxLaunchToolApproval/);

  assert.match(launchProof, /createDxLaunchToolApproval/);
  assert.match(launchProof, /data-dx-ai-tool-approval/);
  assert.match(security, /tool approval/);
});
