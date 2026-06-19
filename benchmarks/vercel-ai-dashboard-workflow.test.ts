const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "vercel-ai");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("ai/vercel-ai has a real dashboard workflow and Forge metadata", () => {
  const upstreamIndex = read(path.join(mirror, "packages", "ai", "src", "index.ts"));
  const upstreamGenerateText = read(
    path.join(mirror, "packages", "ai", "src", "generate-text", "index.ts"),
  );
  const upstreamUi = read(path.join(mirror, "packages", "ai", "src", "ui", "index.ts"));
  const upstreamRegistry = read(path.join(mirror, "packages", "ai", "src", "registry", "index.ts"));
  const forge = read(path.join(root, "core", "src", "ecosystem", "forge_vercel_ai.rs"));
  const workflow = read(
    path.join(root, "examples", "dashboard", "src", "components", "AiLaunchAssistant.tsx"),
  );
  const workflowModel = read(
    path.join(root, "examples", "dashboard", "src", "lib", "aiLaunchAssistant.ts"),
  );
  const dashboard = read(path.join(root, "examples", "dashboard", "src", "pages", "Dashboard.tsx"));
  const readme = read(path.join(root, "examples", "dashboard", "README.md"));
  const packageDoc = read(path.join(root, "docs", "packages", "ai-vercel-ai.md"));

  assert.match(upstreamIndex, /export \* from '\.\/generate-text'/);
  assert.match(upstreamGenerateText, /streamText/);
  assert.match(upstreamUi, /convertToModelMessages/);
  assert.match(upstreamRegistry, /createProviderRegistry/);
  assert.match(upstreamIndex, /createGateway, gateway/);

  assert.match(forge, /packageId: "ai\/vercel-ai"/);
  assert.match(forge, /aliases: \["vercel-ai", "ai-sdk", "@vercel\/ai"\]/);
  assert.match(forge, /sourceMirror: "G:\/WWW\/inspirations\/vercel-ai"/);
  assert.match(forge, /provenance: \{/);
  assert.match(forge, /requiredEnv: \["AI_PROVIDER_API_KEY"\]/);
  assert.match(forge, /optionalEnv: \["AI_GATEWAY_API_KEY"\]/);
  assert.match(forge, /appOwnedBoundaries: \[/);
  assert.match(forge, /receiptPaths: \[/);
  assert.match(forge, /documentation: \{/);
  assert.match(forge, /packageDoc: "docs\/packages\/ai-vercel-ai\.md"/);
  assert.match(forge, /exportedFiles: \[/);
  assert.match(forge, /"js\/lib\/ai\/dashboard-readiness\.ts"/);
  assert.match(forge, /"js\/components\/ai\/ai-launch-assistant\.tsx"/);
  assert.match(forge, /"lib\/ai\/dashboard-readiness\.ts"/);
  assert.match(forge, /"components\/ai\/ai-launch-assistant\.tsx"/);
  assert.match(forge, /export function AiLaunchAssistant/);
  assert.match(forge, /export function createDxAiDashboardReceipt/);
  assert.match(forge, /apiKey: process\.env\.AI_PROVIDER_API_KEY/);
  assert.match(forge, /status: "missing-config"/);
  assert.match(forge, /credentialsConfigured: false/);
  assert.match(forge, /Set AI_PROVIDER_API_KEY in the app environment/);
  assert.match(forge, /data-dx-component="dashboard-ai-launch-assistant"/);
  assert.match(forge, /<dx-icon aria-label="AI" name="pack:ai" \/>/);
  assert.match(forge, /dashboardWorkflow: "components\/ai\/ai-launch-assistant\.tsx"/);
  assert.match(
    forge,
    /dashboardExample: "examples\/dashboard\/src\/components\/AiLaunchAssistant\.tsx"/,
  );

  assert.match(workflow, /data-dx-package="ai\/vercel-ai"/);
  assert.match(workflow, /data-dx-component="dashboard-ai-launch-assistant"/);
  assert.match(workflow, /data-dx-ai-dashboard-workflow="launch-risk-review"/);
  assert.match(workflow, /data-dx-ai-provider-readiness="app-owned"/);
  assert.match(workflow, /data-dx-ai-interaction="provider-picker"/);
  assert.match(workflow, /data-dx-ai-interaction="prompt-field"/);
  assert.match(workflow, /data-dx-ai-action="safe-local-preview"/);
  assert.match(workflow, /data-dx-ai-local-response/);
  assert.match(workflow, /<dx-icon name="pack:ai"/);
  assert.match(workflow, /<dx-icon name="pack:settings"/);
  assert.match(workflow, /<dx-icon name="pack:play"/);
  assert.doesNotMatch(workflow, /#[0-9a-fA-F]{3,8}|rgb\(|hsl\(/);

  assert.match(workflowModel, /streamText/);
  assert.match(workflowModel, /convertToModelMessages/);
  assert.match(workflowModel, /tool/);
  assert.match(workflowModel, /gateway/);
  assert.match(workflowModel, /createGateway/);
  assert.match(workflowModel, /createProviderRegistry/);
  assert.match(workflowModel, /AI_PROVIDER_API_KEY/);
  assert.match(workflowModel, /AI_GATEWAY_API_KEY/);
  assert.match(workflowModel, /createAiPreviewReceipt/);

  assert.match(dashboard, /import \{ AiLaunchAssistant \}/);
  assert.match(dashboard, /<AiLaunchAssistant \/>/);
  assert.match(readme, /AI SDK provider readiness/);
  assert.match(readme, /AiLaunchAssistant/);

  assert.match(packageDoc, /Package id: `ai\/vercel-ai`/);
  assert.match(packageDoc, /Official DX package name: `AI SDK`/);
  assert.match(packageDoc, /Upstream package: `ai`/);
  assert.match(packageDoc, /Aliases: `vercel-ai`, `ai-sdk`, `@vercel\/ai`/);
  assert.match(packageDoc, /Source mirror: `G:\\WWW\\inspirations\\vercel-ai`/);
  assert.match(packageDoc, /Provenance: Vercel AI SDK, Apache-2\.0/);
  assert.match(packageDoc, /`streamText`/);
  assert.match(packageDoc, /`convertToModelMessages`/);
  assert.match(packageDoc, /`createProviderRegistry`/);
  assert.match(packageDoc, /`lib\/ai\/dashboard-readiness\.ts`/);
  assert.match(packageDoc, /`components\/ai\/ai-launch-assistant\.tsx`/);
  assert.match(packageDoc, /`AI_PROVIDER_API_KEY`/);
  assert.match(packageDoc, /`AI_GATEWAY_API_KEY`/);
  assert.match(packageDoc, /data-dx-package="ai\/vercel-ai"/);
  assert.match(packageDoc, /data-dx-component="launch-ai-assistant-dashboard-workflow"/);
  assert.match(packageDoc, /data-dx-component="dashboard-ai-launch-assistant"/);
  assert.match(packageDoc, /data-dx-component="launch-ai-dashboard-workflow"/);
  assert.match(packageDoc, /data-dx-dashboard-workflow="launch-ai-assistant"/);
  assert.match(packageDoc, /data-dx-dashboard-workflow="prompt-action-provider-readiness"/);
  assert.match(packageDoc, /data-dx-ai-route-contract="\/api\/ai\/chat"/);
  assert.match(packageDoc, /data-dx-node-modules="forbidden"/);
  assert.match(packageDoc, /<dx-icon name="pack:ai" \/>/);
  assert.match(packageDoc, /dx run --test \.\\benchmarks\\vercel-ai-dashboard-workflow\.test\.ts/);
});
