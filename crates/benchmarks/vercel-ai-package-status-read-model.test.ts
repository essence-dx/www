const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-ai-vercel-ai-launch-assistant.json";
const statusVocabulary = [
  "present",
  "stale",
  "missing-receipt",
  "blocked",
  "unsupported-surface",
];
const aiSdkMetrics = [
  "ai_sdk_receipt_present",
  "ai_sdk_receipt_stale",
  "ai_sdk_missing_receipt",
  "ai_sdk_blocked_surface",
  "ai_sdk_unsupported_surface",
  "ai_sdk_hash_manifest_present",
  "ai_sdk_hash_mismatch",
  "ai_sdk_receipt_hash_refresh_current",
  "ai_sdk_receipt_hash_refresh_stale",
  "ai_sdk_receipt_hash_refresh_missing",
  "ai_sdk_dx_style_compatibility_present",
  "ai_sdk_dx_style_compatibility_missing",
];

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

test("AI SDK receipt visibility is consumed by the shared package-status read model", () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const receipt = readJson(receiptPath);
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const statusSource = read("examples/template/forge-package-status.ts");
  const packageDoc = read("docs/packages/ai-vercel-ai.md");

  const aiVisibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "ai/vercel-ai",
  );

  assert.ok(aiVisibility, "AI SDK visibility row is missing");
  assert.equal(aiVisibility.official_package_name, "AI SDK");
  assert.equal(aiVisibility.upstream_package, "ai");
  assert.equal(aiVisibility.upstream_version, "7.0.0-canary.146");
  assert.equal(aiVisibility.source_mirror, "G:/WWW/inspirations/vercel-ai");
  assert.equal(aiVisibility.status, "present");
  assert.equal(aiVisibility.receipt_status, "present");
  assert.equal(aiVisibility.package_receipt_path, receiptPath);
  assert.deepEqual(aiVisibility.status_vocabulary, statusVocabulary);
  assert.deepEqual(
    receipt.dx_check_visibility.status_legend.map((entry) => entry.status),
    statusVocabulary,
  );
  assert.equal(receipt.honesty_label, "ADAPTER-BOUNDARY");
  assert.equal(receipt.hash_algorithm, "sha256");

  for (const surfaceId of [
    "ai-chat-route",
    "ai-dashboard-readiness",
    "ai-dashboard-assistant-component",
    "ai-launch-assistant-dashboard-workflow",
  ]) {
    assert.ok(
      aiVisibility.selected_surfaces.some(
        (surface) =>
          surface.surface_id === surfaceId &&
          surface.receipt_path === receiptPath,
      ),
      `${surfaceId} missing from AI SDK visibility row`,
    );
  }

  const markers = aiVisibility.selected_surfaces.flatMap((surface) =>
    surface.source_markers.concat(surface.files),
  );
  for (const marker of [
    'data-dx-package="ai/vercel-ai"',
    'data-dx-component="launch-ai-assistant-dashboard-workflow"',
    'data-dx-component="dashboard-ai-launch-assistant"',
    'data-dx-ai-route-contract="/api/ai/chat"',
    "streamText",
    "convertToModelMessages",
    "DefaultChatTransport",
    "createProviderRegistry",
    "AI_PROVIDER_API_KEY",
  ]) {
    assert.ok(markers.includes(marker), `${marker} missing from AI SDK row`);
  }

  for (const metric of aiSdkMetrics) {
    assert.ok(
      aiVisibility.dx_check_metrics.includes(metric),
      `${metric} missing from AI SDK visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
  }

  assert.match(readModel, /export const aiSdkPackageVisibility/);
  assert.match(
    readModel,
    /packageLaneVisibility:\s*\[[\s\S]*aiSdkPackageVisibility[\s\S]*\]/,
  );
  assert.match(statusSource, /aiSdkPackageVisibility/);
  assert.match(statusSource, /aiSdkVisibility: aiSdkPackageVisibility/);
  assert.ok(status.zed_receipt_surfaces.includes("ai-sdk:chat-route"));
  assert.ok(status.zed_receipt_surfaces.includes("ai-sdk:dashboard-readiness"));
  assert.ok(
    status.zed_receipt_surfaces.includes(
      "ai-sdk:launch-assistant-dashboard-workflow",
    ),
  );
  assert.match(packageDoc, /shared package-status read model/i);
});
