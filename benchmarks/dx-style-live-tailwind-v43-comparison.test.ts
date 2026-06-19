import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert");
const fs = require("node:fs");
const os = require("node:os");
const { spawnSync } = require("node:child_process");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sha256Pattern = /^[a-f0-9]{64}$/;

function assertBlockedReceipt(result, receipt, receiptPath) {
  assert.notEqual(result.status, 0, "blocked live comparisons must exit nonzero");
  assert.ok(fs.existsSync(receiptPath), "blocked live comparison should write a receipt artifact");
  assert.deepEqual(JSON.parse(fs.readFileSync(receiptPath, "utf8")), receipt);
  assert.equal(receipt.schema, "dx.style.liveTailwindComparisonBlockedReceipt");
  assert.equal(receipt.schemaVersion, 1);
  assert.equal(receipt.comparisonStatus, "blocked");
  assert.ok(receipt.blockerStage, "blocked receipt should name the blocked stage");
  assert.ok(receipt.blockerMessage, "blocked receipt should include the blocker message");
  assert.ok(receipt.nextAction, "blocked receipt should include a concrete next action");
  assert.equal(receipt.tailwindPackage, "tailwindcss@4.3.0");
  assert.equal(receipt.tailwindCliPackage, "@tailwindcss/cli@4.3.0");
  assert.equal(receipt.fixtureMatrixIngested, true);
  assert.equal(receipt.classCount, receipt.classificationSummary?.classCount);
  assert.equal(
    receipt.exactFragmentMatchCount,
    receipt.classificationSummary?.exactFragmentMatchCount,
  );
  assert.equal(receipt.knownDifferentCount, receipt.classificationSummary?.knownDifferentCount);
  assert.equal(receipt.tailwindOnlyGapCount, receipt.classificationSummary?.tailwindOnlyGapCount);
  assert.equal(receipt.failedCount, null);
  assert.equal(receipt.matrixIntegrity?.valid, true);
  assert.equal(receipt.matrixIntegrity?.classCount, receipt.classCount);
  assert.equal(receipt.fullTailwindParity, false);
  assert.equal(receipt.evidenceQuality?.canonicalLiveComparison, false);
  assert.equal(receipt.evidenceQuality?.comparisonStatus, "blocked");
  assert.ok(
    receipt.evidenceQuality?.nonCanonicalReasons?.includes(
      `comparison-blocked:${receipt.blockerStage}`,
    ),
  );
  for (const field of [
    "fixtureMatrixClassesSha256",
    "fixtureMatrixComparisonSha256",
    "officialCandidateInventorySha256",
    "officialFixtureSnapshotsSha256",
  ]) {
    assert.match(receipt.inputFingerprints?.[field] ?? "", sha256Pattern);
  }

  assert.fail(
    `canonical live comparison blocked at ${receipt.blockerStage}: ${receipt.blockerMessage}\nnext action: ${receipt.nextAction}`,
  );
}

test("dx-style runs a governed live Tailwind v4.3 output comparison", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-style-live-tailwind-v43-"));
  const receiptPath = path.join(tempDir, "live-tailwind-v43-comparison-receipt.json");
  const result = spawnSync(
    process.execPath,
    [
      "tools/dx-style/live-tailwind-v43-compare.cjs",
      "--matrix",
      "related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json",
      "--receipt",
      receiptPath,
      "--json",
    ],
    {
      cwd: root,
      encoding: "utf8",
      maxBuffer: 10 * 1024 * 1024,
      timeout: 600_000,
    },
  );

  try {
    assert.match(
      result.stdout.trim(),
      /^\{/,
      result.stderr || result.stdout || "live comparison did not emit a JSON receipt",
    );
    const receipt = JSON.parse(result.stdout);

    if (receipt.comparisonStatus === "blocked") {
      assertBlockedReceipt(result, receipt, receiptPath);
    }

    assert.equal(
      result.status,
      receipt.failedCount > 0 ? 1 : 0,
      `runner exit code should mirror receipt.failedCount; stderr:\n${result.stderr}`,
    );
    assert.ok(fs.existsSync(receiptPath), "live comparison should write a receipt artifact");
    assert.deepEqual(JSON.parse(fs.readFileSync(receiptPath, "utf8")), receipt);

    assert.equal(receipt.schema, "dx.style.liveTailwindComparisonReceipt");
    assert.equal(receipt.schemaVersion, 1);
    assert.equal(receipt.comparisonStatus, "compared");
  assert.equal(receipt.tailwindPackage, "tailwindcss@4.3.0");
  assert.equal(receipt.tailwindCliPackage, "@tailwindcss/cli@4.3.0");
  assert.equal(receipt.liveTailwindExecution, true);
  assert.match(receipt.dxStyleCssSource, /^(cargo-run|provided-binary)$/);
  assert.equal(
    receipt.dxStyleFixtureBinaryFreshness === null ||
      receipt.dxStyleFixtureBinaryFreshness?.schema === "dx.style.fixtureBinaryFreshnessReceipt",
    true,
  );
  assert.equal(receipt.tailwindRuntimeDependency, false);
  assert.equal(receipt.packageManifestMutation, false);
  assert.equal(receipt.fullTailwindParity, false);
  assert.equal(typeof receipt.evidenceQuality?.canonicalLiveComparison, "boolean");
  assert.equal(receipt.evidenceQuality?.comparisonStatus, "compared");
  assert.equal(
    receipt.evidenceQuality?.liveTailwindExecution,
    receipt.liveTailwindExecution,
  );
  assert.equal(receipt.evidenceQuality?.tailwindCssSource, receipt.tailwindCssSource);
  assert.equal(receipt.evidenceQuality?.dxStyleCssSource, receipt.dxStyleCssSource);
  assert.ok(
    Array.isArray(receipt.evidenceQuality?.nonCanonicalReasons),
    "live receipt should explain why a run is not canonical parity evidence",
  );
  assert.equal(receipt.fixtureMatrixIngested, true);
  assert.equal(receipt.officialSourceCommit, "588bd7371f4cae96426e1387819b7fd1d99765f9");
  assert.equal(
    receipt.officialCandidateInventory,
    "related-crates/style/fixtures/tailwind-v43-official-candidate-inventory.json",
  );
  assert.ok(receipt.officialCandidateCount >= 8000);
  for (const field of [
    "fixtureMatrixClassesSha256",
    "fixtureMatrixComparisonSha256",
    "officialCandidateInventorySha256",
    "officialFixtureSnapshotsSha256",
  ]) {
    assert.match(receipt.inputFingerprints?.[field] ?? "", sha256Pattern);
  }
  assert.equal(receipt.matrixIntegrity?.valid, true);
  assert.equal(receipt.matrixIntegrity?.classCount, receipt.classCount);
  assert.equal(receipt.matrixIntegrity?.duplicateClassCount, 0);
  assert.deepEqual(receipt.matrixIntegrity?.duplicateClassNames, []);
  assert.equal(
    receipt.officialCandidateInventoryCoverage?.matrixClassCount,
    receipt.classCount,
  );
  assert.equal(
    receipt.officialCandidateInventoryCoverage?.inventoryCandidateCount,
    receipt.officialCandidateCount,
  );
  assert.equal(receipt.classificationSummary?.classCount, receipt.classCount);
  assert.equal(
    receipt.classificationSummary?.exactFragmentMatchCount,
    receipt.exactFragmentMatchCount,
  );
  assert.equal(
    receipt.classificationSummary?.knownDifferentCount,
    receipt.knownDifferentCount,
  );
  assert.equal(
    receipt.classificationSummary?.tailwindOnlyGapCount,
    receipt.tailwindOnlyGapCount,
  );
  assert.ok(
    Array.isArray(receipt.classificationSummary?.byOwnerLane),
    "live receipt should summarize matrix classifications by owning lane",
  );
  assert.equal(receipt.comparisonResultSummary?.classCount, receipt.classCount);
  assert.equal(receipt.comparisonResultSummary?.failedCount, receipt.failedCount);
  assert.ok(
    Array.isArray(receipt.comparisonResultSummary?.byComparisonMode),
    "live receipt should summarize comparison results by comparison mode",
  );
  assert.ok(
    Array.isArray(receipt.comparisonResultSummary?.byOwnerLane),
    "live receipt should summarize comparison results by owning lane",
  );
  assert.ok(
    Array.isArray(receipt.officialCandidateInventoryCoverage?.missingClassEntries),
    "live receipt should expose matrix class coverage against the official candidate inventory",
  );
  assert.ok(receipt.officialSourceFileCount >= 70);
  assert.ok(receipt.officialFixtureCount >= 1000);
  assert.ok(receipt.officialFixtureSourceFileCount >= 40);
  assert.ok(receipt.classCount >= 20);
  assert.equal(receipt.exactFragmentMatchCount, receipt.classCount);
  assert.equal(receipt.knownDifferentCount, 0);
  assert.equal(receipt.tailwindOnlyGapCount, 0);
  assert.equal(receipt.failedClassNames.length, receipt.failedCount);
  assert.ok(Array.isArray(receipt.failedClassHandoffs));
  assert.ok(Array.isArray(receipt.failureLaneBuckets));
  if (receipt.failedCount === 0) {
    assert.deepEqual(receipt.failedClassNames, []);
    assert.deepEqual(receipt.failedUnsupportedByDxStyleClassNames, []);
    assert.deepEqual(receipt.failedMissingTailwindFragmentsClassNames, []);
    assert.deepEqual(receipt.failedMissingDxStyleFragmentsClassNames, []);
    assert.deepEqual(receipt.failedClassHandoffs, []);
    assert.deepEqual(receipt.failureLaneBuckets, []);
  } else {
    assert.equal(receipt.failedClassHandoffs.length, receipt.failedCount);
    assert.ok(
      receipt.failureLaneBuckets.length > 0,
      "failing live comparisons must be grouped by owning lane",
    );
    assert.equal(
      receipt.failureLaneBuckets.reduce((sum, bucket) => sum + bucket.failedCount, 0),
      receipt.failedCount,
    );
    for (const handoff of receipt.failedClassHandoffs) {
      assert.ok(receipt.failedClassNames.includes(handoff.className));
      assert.ok(handoff.area, `${handoff.className} needs an area`);
      assert.ok(handoff.comparisonMode, `${handoff.className} needs a comparison mode`);
      assert.ok(handoff.ownerLaneName, `${handoff.className} needs an owning lane label`);
      assert.ok(
        handoff.failureReasons?.length,
        `${handoff.className} needs at least one failure reason`,
      );
    }
  }

  const byClass = new Map(receipt.entries.map((entry) => [entry.className, entry]));
  assert.equal(byClass.get("@container-normal")?.ownerLaneNumber, 4);
  assert.equal(byClass.get("@container-normal/sidebar")?.ownerLaneNumber, 4);
  assert.equal(byClass.get("@container-size")?.ownerLaneNumber, 4);
  assert.equal(byClass.get("pointer-fine:opacity-100")?.ownerLaneNumber, 3);
  assert.equal(byClass.get("[@unknown_rule]:p-4")?.ownerLaneNumber, 5);
  assert.equal(byClass.get("bg-mauve-500")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("bg-mauve-500")?.ownerLaneNumber, 1);
  assert.equal(byClass.get("bg-olive-500")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("bg-mist-500")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("bg-taupe-500")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("divide-mauve-500")?.ownerLaneNumber, 1);
  assert.equal(byClass.get("divide-mauve-500")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("divide-olive-500/50")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("divide-[#243c5a]")?.comparisonMode, "exact-fragment-match");
  assert.equal(
    byClass.get("divide-(color:--dx-divider)/(--dx-alpha)")?.comparisonMode,
    "exact-fragment-match",
  );
  assert.equal(byClass.get("pointer-fine:opacity-100")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("backdrop:bg-slate-950/50")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("@3xs:grid")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("[@unknown_rule]:p-4")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("not-[@media_print]:flex")?.ownerLaneNumber, 5);
  assert.equal(byClass.get("not-[@media_print]:flex")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("not-[@supports(display:grid)]:flex")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("not-[@container_(width>=32rem)]:flex")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("not-[@container_card_(width>=32rem)]:flex")?.ownerLaneNumber, 5);
  assert.equal(byClass.get("not-[@container_card_(width>=32rem)]:flex")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("[&.foo]:[&.bar]:flex")?.ownerLaneNumber, 5);
  assert.equal(byClass.get("[&.foo]:[&.bar]:flex")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("[&_p]:[&_.lead]:mt-4")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("not-[.is-open]:[&.dismissible]:opacity-100")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("[&.is-dragging]:active:cursor-grabbing")?.comparisonMode, "exact-fragment-match");
  assert.equal(
    byClass.get("[&.foo,&.bar]:[&>.item,&>[data-slot=control]]:opacity-100")?.comparisonMode,
    "exact-fragment-match",
  );
  assert.equal(byClass.get("group-[.is-published]:block")?.ownerLaneNumber, 5);
  assert.equal(byClass.get("group-[.is-published]:block")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("group-[:nth-of-type(3)_&]:block")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("group-[&.foo,&.bar]:block")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("group-[&:is(.foo,.bar)]:block")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("group-[.is-open]/card:block")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("peer-[.is-dirty]:block")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("peer-[&.dirty,&.touched]:block")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("peer-[:nth-of-type(3)_&]:block")?.comparisonMode, "exact-fragment-match");
  assert.equal(byClass.get("peer-[.is-dirty]:peer-required:block")?.comparisonMode, "exact-fragment-match");
    assert.equal(byClass.get("group-[.is-open]:[&.target]:opacity-100")?.comparisonMode, "exact-fragment-match");
  } finally {
    fs.rmSync(tempDir, { recursive: true, force: true });
  }
});
