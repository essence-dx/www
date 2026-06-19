const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json";
const sourceGuardRunbookFixturePath =
  "docs/packages/wasm-bindgen.source-guard-runbook.json";
const sourceGuardRunbookSurfaceId =
  "webassembly-bridge-source-guard-runbook";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function sha256(relativePath) {
  return crypto.createHash("sha256").update(read(relativePath)).digest("hex");
}

test("WebAssembly Bridge dashboard receipt exposes hash-backed dx-check freshness", () => {
  const receipt = readJson(receiptPath);
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const readModel = read("examples/template/forge-package-status-read-model.ts");
  const wasmDxCheck = read("core/src/ecosystem/project_check/wasm_bindgen_dx_check.rs");
  const packageDoc = read("docs/packages/wasm-bindgen.md");
  const launchShell = read("examples/template/template-shell.tsx");
  const dashboardWorkflow = read("examples/dashboard/src/components/WasmBindgenWorkflow.tsx");

  assert.equal(receipt.package_name, "WebAssembly Bridge");
  assert.equal(receipt.package_id, "wasm/bindgen");
  assert.equal(receipt.hash_algorithm, "sha256");
  assert.ok(receipt.files.length >= 6, "receipt must track selected source files");
  assert.equal(
    receipt.source_guard_runbook_fixture,
    sourceGuardRunbookFixturePath,
  );
  assert.ok(
    receipt.selected_surfaces.includes(sourceGuardRunbookSurfaceId),
    "receipt selected_surfaces should include the WebAssembly Bridge source-guard runbook fixture",
  );
  assert.ok(
    receipt.files.includes(sourceGuardRunbookFixturePath),
    "receipt files should track the package-owned source-guard runbook fixture",
  );
  assert.equal(
    receipt.file_hashes[sourceGuardRunbookFixturePath],
    sha256(sourceGuardRunbookFixturePath),
    "WebAssembly Bridge source-guard runbook fixture hash is stale",
  );
  assert.equal(
    receipt.source_guard_runbook_surface.file_hashes[
      sourceGuardRunbookFixturePath
    ],
    receipt.file_hashes[sourceGuardRunbookFixturePath],
    "WebAssembly Bridge receipt source_guard_runbook_surface hash should mirror the top-level file_hashes manifest",
  );

  assert.match(launchShell, /data-dx-style-surface="theme-token"/);
  assert.match(dashboardWorkflow, /data-dx-style-surface="theme-token-card"/);

  const styleCompatibility = receipt.dx_style_compatibility;
  assert.equal(styleCompatibility.schema, "dx.forge.package.dx_style_compatibility");
  assert.equal(styleCompatibility.status, "present");
  assert.equal(styleCompatibility.token_source, "styles/globals.css");
  assert.equal(styleCompatibility.generated_css, "styles/globals.css");
  assert.equal(styleCompatibility.runtime_proof, false);
  assert.deepEqual(styleCompatibility.visible_surfaces, [
    "dashboard-wasm-bindgen-workflow",
    "launch-wasm-compute-dashboard-workflow",
    "wasm-bindgen-readiness-workflow",
  ]);
  assert.ok(
    styleCompatibility.source_files.includes(
      "examples/template/template-shell.tsx",
    ),
  );
  assert.ok(
    styleCompatibility.source_files.includes(
      "examples/dashboard/src/components/WasmBindgenWorkflow.tsx",
    ),
  );
  assert.match(
    styleCompatibility.runtime_limitations.join(" "),
    /no live governed browser style proof/,
  );

  for (const filePath of receipt.files) {
    assert.equal(
      receipt.file_hashes[filePath],
      sha256(filePath),
      `${filePath} hash is stale in WebAssembly Bridge receipt`,
    );
  }

  const bridgeVisibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "wasm/bindgen",
  );
  assert.ok(bridgeVisibility, "WebAssembly Bridge visibility row is missing");
  assert.equal(
    bridgeVisibility.source_guard_runbook_fixture,
    sourceGuardRunbookFixturePath,
  );

  for (const surface of bridgeVisibility.selected_surfaces) {
    assert.equal(surface.hash_algorithm, "sha256");
    for (const filePath of surface.files) {
      if (receipt.file_hashes[filePath]) {
        assert.equal(
          surface.file_hashes[filePath],
          receipt.file_hashes[filePath],
          `${surface.surface_id} should mirror the receipt hash for ${filePath}`,
        );
      }
    }
  }

  const sourceGuardRunbookSurface = bridgeVisibility.selected_surfaces.find(
    (surface) => surface.surface_id === sourceGuardRunbookSurfaceId,
  );
  assert.ok(
    sourceGuardRunbookSurface,
    "WebAssembly Bridge source-guard runbook selected surface is missing",
  );
  assert.equal(
    sourceGuardRunbookSurface.surface_type,
    "source_guard_runbook_fixture",
  );
  assert.deepEqual(sourceGuardRunbookSurface.files, [
    sourceGuardRunbookFixturePath,
  ]);
  assert.equal(
    sourceGuardRunbookSurface.file_hashes[sourceGuardRunbookFixturePath],
    receipt.file_hashes[sourceGuardRunbookFixturePath],
  );
  assert.ok(
    sourceGuardRunbookSurface.source_markers.includes(
      "source_guard_runbook_index",
    ),
  );
  assert.ok(
    sourceGuardRunbookSurface.source_markers.includes(
      "sourceGuardRunbookFixtures",
    ),
  );
  assert.ok(
    sourceGuardRunbookSurface.source_markers.includes(
      sourceGuardRunbookFixturePath,
    ),
  );
  assert.match(
    sourceGuardRunbookSurface.runtime_limitations.join(" "),
    /SOURCE-ONLY/,
  );
  assert.match(
    readModel,
    /sourceGuardRunbookFixture:\s*"docs\/packages\/wasm-bindgen\.source-guard-runbook\.json"/,
  );
  assert.match(
    readModel,
    /surfaceId:\s*"webassembly-bridge-source-guard-runbook"/,
  );
  assert.match(readModel, /surfaceType:\s*"source_guard_runbook_fixture"/);
  assert.match(readModel, /source_guard_runbook_index/);
  assert.match(readModel, /sourceGuardRunbookFixtures/);

  for (const metric of [
    "webassembly_bridge_hash_manifest_present",
    "webassembly_bridge_hash_mismatch",
    "webassembly_bridge_dx_style_compatibility_present",
    "webassembly_bridge_dx_style_compatibility_missing",
  ]) {
    assert.ok(
      bridgeVisibility.dx_check_metrics.includes(metric),
      `${metric} missing from WebAssembly Bridge visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
    assert.match(wasmDxCheck, new RegExp(metric));
  }

  assert.equal(
    bridgeVisibility.dx_style_compatibility.schema,
    "dx.forge.package.dx_style_compatibility",
  );
  assert.equal(bridgeVisibility.dx_style_compatibility.status, "present");
  assert.ok(
    bridgeVisibility.dx_style_compatibility.visible_surfaces.includes(
      "launch-wasm-compute-dashboard-workflow",
    ),
  );

  for (const marker of [
    "mod file_hashes;",
    "pub(super) fn count_sha256_file_hash_mismatches",
    "Sha256::digest",
    "use super::file_hashes::count_sha256_file_hash_mismatches;",
    "count_sha256_file_hash_mismatches(root, surface)",
    "webassembly_bridge_hash_mismatch_metric_and_finding_are_byte_derived",
    "webassembly_bridge_dx_style_missing_metric_and_finding_flip",
    "forge_webassembly_bridge_package_metrics(dir.path(), &manifest)",
    'metric_value(&metrics, "webassembly_bridge_hash_mismatch")',
    "webassembly_bridge_dx_style_compatibility_missing",
    "webassembly-bridge-hash-mismatch",
    "hash_manifest_present",
    "hash_mismatches",
    "dx_style_compatibility_is_present",
    "webassembly-bridge-missing-dx-style-compatibility",
  ]) {
    const source = marker === "mod file_hashes;"
      ? read("core/src/ecosystem/project_check.rs")
      : marker.includes("count_sha256_file_hash_mismatches") || marker.includes("Sha256")
        ? read("core/src/ecosystem/project_check/file_hashes.rs") + wasmDxCheck
      : wasmDxCheck;
    assert.match(source, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
  assert.doesNotMatch(wasmDxCheck, /fn count_hash_mismatches\(/);

  assert.match(packageDoc, /hash_algorithm: sha256/);
  assert.match(packageDoc, /file_hashes/);
  assert.match(packageDoc, /webassembly_bridge_hash_mismatch/);
  assert.match(packageDoc, /byte-level SHA-256 helper/);
  assert.match(packageDoc, /## DX-Style Compatibility/);
  assert.match(packageDoc, /webassembly_bridge_dx_style_compatibility_present/);
  assert.match(packageDoc, /webassembly-bridge-missing-dx-style-compatibility/);
  assert.match(packageDoc, /webassembly-bridge-source-guard-runbook/);
  assert.match(packageDoc, /source_guard_runbook_fixture/);
});
