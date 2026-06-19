const assert = require("assert");
const crypto = require("crypto");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "vercel-ai");
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-ai-vercel-ai-launch-assistant.json";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function readMirror(relativePath) {
  return fs.readFileSync(path.join(mirror, relativePath), "utf8");
}

function resolveTrackedFile(trackedFile) {
  if (trackedFile.startsWith("upstream:")) {
    return path.join(mirror, trackedFile.slice("upstream:".length));
  }

  return path.join(root, trackedFile);
}

function sha256(trackedFile) {
  return crypto
    .createHash("sha256")
    .update(fs.readFileSync(resolveTrackedFile(trackedFile)))
    .digest("hex");
}

test("AI SDK receipt exposes dx-check visibility states and source hashes", () => {
  const receipt = readJson(receiptPath);
  const forge = read("core/src/ecosystem/forge_vercel_ai.rs");
  const packageCatalog = read("examples/template/package-catalog.ts");
  const packageDoc = read("docs/packages/ai-vercel-ai.md");
  const streamTextSource = readMirror("packages/ai/src/generate-text/stream-text.ts");
  const transportSource = readMirror("packages/ai/src/ui/default-chat-transport.ts");
  const registrySource = readMirror("packages/ai/src/registry/index.ts");

  assert.equal(receipt.package_id, "ai/vercel-ai");
  assert.equal(receipt.package_name, "AI SDK");
  assert.equal(receipt.official_dx_package_name, "AI SDK");
  assert.equal(receipt.upstream_package, "ai");
  assert.equal(receipt.upstream_version, "7.0.0-canary.146");
  assert.equal(receipt.honesty_label, "ADAPTER-BOUNDARY");
  assert.ok(receipt.no_runtime_execution);
  assert.ok(receipt.upstream_public_apis.includes("streamText"));
  assert.ok(receipt.upstream_public_apis.includes("DefaultChatTransport"));
  assert.ok(receipt.upstream_public_apis.includes("createProviderRegistry"));
  assert.ok(
    receipt.guards.includes(
      "dx run --test .\\benchmarks\\vercel-ai-dx-check-visibility-receipt.test.ts",
    ),
  );

  assert.equal(
    receipt.dx_check_visibility.schema,
    "dx.forge.package.dx_check_visibility",
  );
  assert.equal(receipt.dx_check_visibility.current_status, "present");
  assert.deepEqual(
    receipt.dx_check_visibility.status_legend.map((entry) => entry.status),
    ["present", "stale", "missing-receipt", "blocked", "unsupported-surface"],
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "ai-chat-route" &&
        surface.status === "present" &&
        surface.source_file === "core/src/ecosystem/forge_vercel_ai.rs" &&
        surface.materialized_file === "lib/ai/chat-route.ts",
    ),
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "ai-dashboard-readiness" &&
        surface.status === "present" &&
        surface.materialized_file === "lib/ai/dashboard-readiness.ts",
    ),
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "ai-launch-assistant-dashboard-workflow" &&
        surface.status === "present" &&
        surface.source_file === "examples/template/ai-chat-status.tsx" &&
        surface.materialized_file === "components/template-app/ai-chat-status.tsx",
    ),
  );

  assert.equal(receipt.hash_algorithm, "sha256");
  for (const trackedFile of receipt.dx_check_visibility.tracked_hash_files) {
    assert.equal(receipt.source_hashes[trackedFile], sha256(trackedFile));
  }

  assert.match(streamTextSource, /export function streamText/);
  assert.match(streamTextSource, /toUIMessageStreamResponse/);
  assert.match(transportSource, /export class DefaultChatTransport/);
  assert.match(registrySource, /createProviderRegistry/);

  assert.match(forge, /dxCheckVisibility: \{/);
  assert.match(forge, /schema: "dx\.forge\.package\.dx_check_visibility"/);
  assert.match(forge, /currentStatus: "present"/);
  assert.match(forge, /"missing-receipt"/);
  assert.match(forge, /"unsupported-surface"/);

  assert.match(packageCatalog, /packageId: "ai\/vercel-ai"/);
  assert.match(packageCatalog, /officialName: "AI SDK"/);
  assert.match(packageCatalog, /dxCheckVisibility: \{/);
  assert.match(
    packageCatalog,
    /receiptPath:\s*"examples\/template\/\.dx\/forge\/receipts\/2026-05-22-ai-vercel-ai-launch-assistant\.json"/,
  );
  assert.match(packageCatalog, /lib\/ai\/chat-route\.ts/);
  assert.match(packageCatalog, /components\/template-app\/ai-chat-status\.tsx/);

  assert.match(packageDoc, /dx-check visibility/i);
  assert.match(
    packageDoc,
    /present, stale, missing-receipt, blocked, and unsupported-surface/,
  );
});
