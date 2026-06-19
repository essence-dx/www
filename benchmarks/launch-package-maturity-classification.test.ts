import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

const allowedMaturityKinds = new Set([
  "lock-backed-package",
  "source-owned-limited-proof",
  "adapter-boundary-readiness",
  "provider-gated",
  "source-guard-only",
]);

const allowedRealityLevelIds = new Set([
  "real-lock-backed",
  "lock-backed-adapter-boundary",
  "source-owned-not-runtime-proven",
  "adapter-boundary",
  "docs-receipt-source-guard-only",
]);

function readJson(relativePath: string) {
  return JSON.parse(fs.readFileSync(path.join(root, relativePath), "utf8"));
}

test("package-status rows expose honest launch maturity classifications", async () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const readModel = await import(
    "../examples/template/forge-package-status-read-model.ts"
  );
  const reality = await import(
    "../examples/template/components/template-app/package-reality.ts"
  );

  const statusRows = status.package_lane_visibility ?? [];
  const readModelRows = readModel.readLaunchForgePackageStatus().packageLaneVisibility;
  const realityRowsById = new Map(
    reality.forgeRealityRows.map((row) => [row.packageId, row]),
  );

  assert.equal(statusRows.length, 20);
  assert.equal(readModelRows.length, statusRows.length);

  for (const row of statusRows) {
    const classification = row.launch_classification;
    const readModelRow = readModelRows.find(
      (entry) => entry.packageId === row.package_id,
    );
    const realityRow = realityRowsById.get(row.package_id);

    assert.ok(classification, `${row.package_id} is missing launch_classification`);
    assert.ok(readModelRow, `${row.package_id} is missing from typed read model`);
    assert.ok(realityRow, `${row.package_id} is missing from package reality rows`);

    assert.equal(classification.schema, "dx.forge.package.launch_classification");
    assert.equal(classification.package_id, row.package_id);
    assert.ok(
      allowedMaturityKinds.has(classification.maturity_kind),
      `${row.package_id} uses unsupported maturity ${classification.maturity_kind}`,
    );
    assert.ok(
      allowedRealityLevelIds.has(classification.reality_level_id),
      `${row.package_id} uses unsupported reality ${classification.reality_level_id}`,
    );
    assert.equal(classification.maturity_kind, realityRow.maturityKind);
    assert.equal(classification.reality_level_id, realityRow.realityLevelId);
    assert.equal(classification.runtime_proof, false);
    assert.equal(classification.browser_proof, false);
    assert.equal(classification.live_provider_proof, false);
    assert.equal(
      classification.classification_source,
      "examples/template/components/template-app/package-reality.ts",
    );
    assert.match(classification.classification_summary, /\S/);
    assert.match(classification.remaining_proof, /\S/);

    assert.deepEqual(readModelRow.launchClassification, {
      schema: classification.schema,
      packageId: classification.package_id,
      maturityKind: classification.maturity_kind,
      realityLevelId: classification.reality_level_id,
      runtimeProof: classification.runtime_proof,
      browserProof: classification.browser_proof,
      liveProviderProof: classification.live_provider_proof,
      classificationSource: classification.classification_source,
      classificationSummary: classification.classification_summary,
      remainingProof: classification.remaining_proof,
    });
  }
});
