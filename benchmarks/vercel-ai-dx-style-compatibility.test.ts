const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-ai-vercel-ai-launch-assistant.json";

function read(relativePath) {
  const filePath = path.join(root, relativePath);
  assert.ok(fs.existsSync(filePath), `missing ${relativePath}`);
  return fs.readFileSync(filePath, "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function escaped(marker) {
  return new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
}

test("AI SDK launch assistant declares dx-style compatibility", () => {
  const aiChatStatus = read("examples/template/ai-chat-status.tsx");
  const receipt = readJson(receiptPath);
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const readModel = read("examples/template/forge-package-status-read-model.ts");
  const packageCatalog = read("examples/template/package-catalog.ts");
  const aiDxCheck = read("core/src/ecosystem/project_check/ai_sdk_dx_check.rs");
  const forgeSlice = read("core/src/ecosystem/forge_vercel_ai.rs");
  const packageDoc = read("docs/packages/ai-vercel-ai.md");

  assert.match(aiChatStatus, /data-dx-style-surface="ai-sdk"/);
  assert.match(aiChatStatus, /data-dx-token-scope="ai\/vercel-ai"/);
  assert.doesNotMatch(aiChatStatus, /style=\{\{/);
  assert.doesNotMatch(aiChatStatus, /#[0-9a-fA-F]{3,8}/);

  const compatibility = receipt.dx_style_compatibility;
  assert.equal(compatibility.schema, "dx.forge.package.dx_style_compatibility");
  assert.equal(compatibility.status, "present");
  assert.equal(compatibility.token_source, "styles/globals.css");
  assert.equal(compatibility.generated_css, "styles/globals.css");
  assert.equal(compatibility.runtime_proof, false);
  assert.ok(
    compatibility.visible_surfaces.includes(
      "launch-ai-assistant-dashboard-workflow",
    ),
  );
  assert.ok(
    compatibility.source_files.includes(
      "examples/template/ai-chat-status.tsx",
    ),
  );
  assert.ok(
    compatibility.source_files.includes("core/src/ecosystem/forge_vercel_ai.rs"),
  );
  assert.ok(
    compatibility.data_dx_markers.includes('data-dx-style-surface="ai-sdk"'),
  );
  assert.ok(
    compatibility.data_dx_markers.includes('data-dx-token-scope="ai/vercel-ai"'),
  );
  assert.ok(
    compatibility.style_boundaries.includes("no inline style objects"),
  );
  assert.match(
    compatibility.runtime_limitations.join(" "),
    /live model streaming or browser visual proof/,
  );

  const aiVisibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "ai/vercel-ai",
  );
  assert.ok(aiVisibility, "AI SDK visibility row is missing");
  assert.equal(
    aiVisibility.dx_style_compatibility.schema,
    "dx.forge.package.dx_style_compatibility",
  );
  assert.equal(aiVisibility.dx_style_compatibility.status, "present");
  assert.equal(aiVisibility.dx_style_compatibility.token_source, "styles/globals.css");
  assert.ok(
    aiVisibility.selected_surfaces
      .find(
        (surface) =>
          surface.surface_id === "ai-launch-assistant-dashboard-workflow",
      )
      .source_markers.includes('data-dx-style-surface="ai-sdk"'),
  );

  for (const metric of [
    "ai_sdk_dx_style_compatibility_present",
    "ai_sdk_dx_style_compatibility_missing",
  ]) {
    assert.ok(
      aiVisibility.dx_check_metrics.includes(metric),
      `${metric} missing from AI SDK visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, escaped(metric));
    assert.match(aiDxCheck, escaped(metric));
  }

  for (const marker of [
    "dxStyleCompatibility: {",
    "visibleSurfaces: [\"launch-ai-assistant-dashboard-workflow\"]",
    "data-dx-style-surface=\"ai-sdk\"",
    "data-dx-token-scope=\"ai/vercel-ai\"",
    "ai-sdk-missing-dx-style-compatibility",
    "dx_style_compatibility_is_present",
    "ai_sdk_dx_style_compatibility_missing_is_reported",
  ]) {
    assert.match(readModel + aiDxCheck, escaped(marker), `missing ${marker}`);
  }

  assert.match(packageCatalog, /packageId: "ai\/vercel-ai"[\s\S]*dxStyleCompatibility: \{/);
  assert.match(forgeSlice, /data-dx-style-surface="ai-sdk"/);
  assert.match(forgeSlice, /dxStyleCompatibility/);
  assert.match(packageDoc, /## DX-Style Compatibility/);
  assert.match(packageDoc, /ai_sdk_dx_style_compatibility_present/);
  assert.match(packageDoc, /ai-sdk-missing-dx-style-compatibility/);
});
