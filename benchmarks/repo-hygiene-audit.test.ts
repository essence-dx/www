import assert from "node:assert/strict";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

import {
  TWELVE_WORST_HYGIENE_FLAWS,
  auditRepoHygiene,
} from "../tools/hygiene/audit-repo-hygiene.ts";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

test("repo hygiene blockers stay fixed", () => {
  const result = auditRepoHygiene(repoRoot);
  assert.deepEqual(result.blockers, []);
  assert.equal(result.ok, true);
});

test("deprecated and typoed archive folders are gone from active examples", () => {
  const forbidden = [
    "examples/deprecated-1",
    "examples/deprecated-2",
    "examples/depricated-3",
    "examples/onboard-deprecated-recovery-copy",
    "artifacts",
    "target-codex-readiness-gate",
    "trash",
  ];

  for (const relativePath of forbidden) {
    assert.equal(
      fs.existsSync(path.join(repoRoot, relativePath)),
      false,
      `${relativePath} should not exist in the active tree`,
    );
  }
});

test("hygiene audit reports no scorecard debt after owned fixture and script contracts", () => {
  const result = auditRepoHygiene(repoRoot);
  const categories = new Set(result.debt.map((item) => item.category));

  assert.equal(categories.has("large-source-file"), false);
  assert.equal(categories.has("tracked-generated-surface"), false);
  assert.equal(categories.has("legacy-script-extension"), false);
  assert.deepEqual(result.debt, []);
});

test("benchmark legacy test extension migration stays closed under owned script contract", () => {
  const migratedLegacyBenchmarkTests = fs
    .readdirSync(path.join(repoRoot, "benchmarks"))
    .filter((name) => name.endsWith(".test.mjs") || name.endsWith(".test.cjs"));
  const result = auditRepoHygiene(repoRoot);
  const scorecard = new Map(result.scorecard.map((item) => [item.id, item]));

  assert.deepEqual(migratedLegacyBenchmarkTests, []);
  assert.equal(scorecard.get("legacy-script-extensions")?.status, "passing");
  assert.equal(result.metrics.legacyScripts > 0, true);
  assert.equal(result.metrics.legacyScriptsUnowned, 0);
});

test("source-visible fixtures are owned by the hygiene contract", () => {
  const result = auditRepoHygiene(repoRoot);
  const scorecard = new Map(result.scorecard.map((item) => [item.id, item]));

  assert.equal(scorecard.get("source-visible-fixtures")?.status, "passing");
  assert.equal(result.metrics.sourceVisibleFixturesOwned, 7);
  assert.equal(
    result.debt.some((item) => item.category === "tracked-generated-surface"),
    false,
  );
});

test("hygiene audit publishes the stable 12-flaw scorecard", () => {
  const result = auditRepoHygiene(repoRoot);
  const expectedIds = [
    "cli-mod-large",
    "public-framework-tools-large",
    "source-render-large",
    "dx-check-receipt-large",
    "forge-registry-large",
    "project-check-large",
    "devtools-runtime-large",
    "devtools-style-ops-large",
    "devtools-css-large",
    "source-visible-fixtures",
    "legacy-script-extensions",
    "readiness-overclaim-risk",
  ];

  assert.deepEqual(
    TWELVE_WORST_HYGIENE_FLAWS.map((flaw) => flaw.id),
    expectedIds,
  );
  assert.deepEqual(
    result.scorecard.map((flaw) => flaw.id),
    expectedIds,
  );
  assert.equal(result.scorecard.length, 12);
  assert.equal(result.readiness.readyFor100, true);
  assert.equal(result.readiness.status, "ready");
  assert.deepEqual(result.readiness.activeFlawIds, []);
});

test("hygiene readiness requires the full scorecard to be closed", () => {
  const result = auditRepoHygiene(repoRoot);

  assert.equal(result.ok, true);
  assert.equal(result.debt.length, 0);
  assert.equal(result.readiness.readyFor100, true);
  assert.equal(result.readiness.blockerFree, true);
  assert.equal(result.readiness.debtFree, true);
  assert.equal(result.readiness.scorecardTotal, 12);
  assert.equal(result.readiness.scorecardOpen, 0);
});

test("hygiene audit ok fails when scorecard debt is present", () => {
  const fixtureRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-www-hygiene-debt-"));

  try {
    fs.writeFileSync(path.join(fixtureRoot, "unowned-script.js"), "export default 1;\n");

    const result = auditRepoHygiene(fixtureRoot);

    assert.equal(result.ok, false);
    assert.equal(result.blockers.length, 0);
    assert.equal(result.debt.length, 1);
    assert.equal(result.readiness.readyFor100, false);
    assert.equal(result.readiness.status, "debt");
    assert.deepEqual(result.readiness.activeFlawIds, [
      "legacy-script-extensions",
      "readiness-overclaim-risk",
    ]);
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

test("hygiene readiness does not claim codebase, product, browser, provider, or launch readiness", () => {
  const result = auditRepoHygiene(repoRoot);

  assert.equal(result.readiness.claimScope, "repo-hygiene-scorecard");
  assert.equal(result.readiness.codebaseReady, false);
  assert.equal(result.readiness.productReady, false);
  assert.equal(result.readiness.browserReady, false);
  assert.equal(result.readiness.providerReady, false);
  assert.equal(result.readiness.launchReady, false);
  assert.match(result.readiness.message, /hygiene scorecard is closed/i);
  assert.match(result.readiness.message, /does not prove codebase, product, browser, provider, or launch readiness/i);
});

test("Devtools source splits close their large-file scorecard items", () => {
  const result = auditRepoHygiene(repoRoot);
  const scorecard = new Map(result.scorecard.map((item) => [item.id, item]));

  for (const id of [
    "devtools-runtime-large",
    "devtools-style-ops-large",
    "devtools-css-large",
  ]) {
    assert.equal(scorecard.get(id)?.status, "passing", `${id} should be closed`);
    assert.equal(scorecard.get(id)?.itemCount, 0, `${id} should not have debt items`);
  }

  assert.equal(result.readiness.readyFor100, true);
  assert.deepEqual(result.readiness.activeFlawIds, []);
});
