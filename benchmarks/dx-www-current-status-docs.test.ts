import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function read(relativePath: string) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

function readJson(relativePath: string) {
  return JSON.parse(read(relativePath));
}

test("public docs expose current source/check score separately from historical release readiness", () => {
  const readme = read("README.md");
  const handoff = read("docs/DX_WWW_MANAGER_HANDOFF.md");
  const docs = `${readme}\n${handoff}`;

  assert.match(readme, /Verified Results/);
  assert.match(readme, /490\/500/);
  assert.match(readme, /readiness score of `99`/);
  assert.match(readme, /Performance Leadership/);
  assert.match(readme, /2398\.95/);
  assert.match(readme, /2515\.26/);
  assert.match(readme, /1142\.21/);
  assert.match(readme, /474` bytes/);
  assert.match(readme, /smallest captured route payload/i);
  assert.match(readme, /Controlled local benchmark/);
  assert.match(readme, /Performance Leadership/);
  assert.match(readme, /repository quality gates/i);
  assert.match(handoff, /Current 30-Agent Worker Checkpoint/);
  assert.match(docs, /84\/100, not (?:a )?release-ready/i);
  assert.match(docs, /62 Rust warnings/i);
  assert.match(docs, /browser screenshot/i);
  assert.match(docs, /overlay recovery proof/i);
  assert.match(docs, /full `cargo test` \/ `cargo clippy`/i);
  assert.doesNotMatch(docs, /current (?:verified )?(?:integration )?score is 100\/100/i);
  assert.doesNotMatch(docs, /release-ready score is 100\/100/i);
});

test("current launch docs mark cosmetic polish outside the active score cap", () => {
  const docs = [
    read("README.md"),
    read("dx-www/README.md"),
    read("examples/template/README.md"),
  ].join("\n");

  assert.match(docs, /Deferred Polish, Not Current Score Caps|Deferred Polish Scope|Deferred polish, not current score caps/);
  assert.match(docs, /Speed Index/i);
  assert.match(docs, /Clippy/i);
  assert.match(docs, /old extra worktrees?/i);
  assert.match(docs, /20,000\+ line/i);
  assert.match(docs, /dx-www\/src\/cli\/mod\.rs/i);
  assert.match(docs, /\.js`, `.cjs`, and `.mjs`/i);
  assert.match(docs, /new scripts prefer `\.ts`/i);
  assert.match(docs, /not (?:active )?score caps?/i);
  assert.match(read("README.md"), /Engineering roadmap:/i);
});

test("historical benchmark reports are marked as non-current readiness snapshots", () => {
  const frameworkMarkdown = read("benchmarks/reports/framework-scorecard.md");
  const currentMarkdown = read("benchmarks/reports/current-status.md");
  const latestMarkdown = read("benchmarks/reports/latest.md");
  const markdown = `${frameworkMarkdown}\n${currentMarkdown}\n${latestMarkdown}`;
  const scorecard = readJson("benchmarks/reports/framework-scorecard.json");
  const currentStatus = readJson("benchmarks/reports/current-status.json");
  const latest = readJson("benchmarks/reports/latest.json");
  const latestCompression = readJson("benchmarks/reports/latest-compression.json");
  const expectedBlockers = [
    "62 Rust warnings",
    "chaotic worktree",
    "source-guard-heavy evidence",
    "missing browser screenshot proof",
    "missing overlay recovery proof",
    "generated artifact curation",
    "full cargo test and clippy not rerun after curation",
  ];

  assert.match(markdown, /Historical benchmark snapshot, not the current release-readiness score/);
  assert.match(markdown, /current 30-agent worker checkpoint is 84\/100/);
  assert.equal(scorecard.snapshot_status.not_current_release_readiness, true);
  assert.equal(scorecard.snapshot_status.current_worker_checkpoint_score, "84/100");
  assert.equal(currentStatus.snapshot_status.not_current_release_readiness, true);
  assert.equal(latest.snapshot_status.not_current_release_readiness, true);
  assert.equal(latestCompression.snapshot_status.not_current_release_readiness, true);
  assert.deepEqual(scorecard.snapshot_status.current_worker_checkpoint_blockers, expectedBlockers);
  assert.deepEqual(currentStatus.snapshot_status.current_worker_checkpoint_blockers, expectedBlockers);
  assert.deepEqual(latest.snapshot_status.current_worker_checkpoint_blockers, expectedBlockers);
  assert.deepEqual(latestCompression.snapshot_status.current_worker_checkpoint_blockers, expectedBlockers);
});

test("benchmark report generators preserve current checkpoint snapshot markers", () => {
  const snapshotStatusHelper = read("benchmarks/report-snapshot-status.js");
  const currentStatusGenerator = read("benchmarks/measure-current-status.ts");
  const scorecardGenerator = read("benchmarks/measure-framework-scorecard.ts");
  const generatorSources = `${currentStatusGenerator}\n${scorecardGenerator}`;

  assert.match(snapshotStatusHelper, /generated artifact curation/);
  assert.match(generatorSources, /report-snapshot-status/);
  assert.match(currentStatusGenerator, /buildHistoricalBenchmarkSnapshotStatus\(\)/);
  assert.match(scorecardGenerator, /buildHistoricalBenchmarkSnapshotStatus\(\)/);
  assert.match(scorecardGenerator, /snapshotStatusMarkdownBlock\(\)/);
});

test("historical root roadmap docs do not read as current release readiness", () => {
  const rootStatus = read("docs/root-workspace-status.md");
  const rootTodo = read("docs/root-workspace-todo.md");
  const docs = `${rootStatus}\n${rootTodo}`;

  assert.match(rootStatus, /Historical workspace status, not the current 30-agent release-readiness score/i);
  assert.match(rootTodo, /Historical roadmap, not the current 30-agent release-readiness score/i);
  assert.match(rootStatus, /100\/100 labels.*source\/contract slice\s+scores, not current live browser, runtime, or full-release proof/is);
  assert.match(rootTodo, /100\/100 labels.*scoped source\/contract\s+completion notes, not current live browser, runtime, or full-release proof/is);
  assert.match(docs, /current 30-agent worker checkpoint is 84\/100/i);
  assert.match(docs, /62 Rust warnings/i);
  assert.match(docs, /browser screenshot\s*>?\s*and overlay recovery proof/i);
  assert.match(docs, /generated artifact curation/i);
});

test("current release-readiness details keep source proof separate from browser and hosted claims", () => {
  const details = read("docs/DX_WWW_CURRENT_DETAILS_2026-05-30.md");

  assert.match(details, /Framework Status/);
  assert.match(details, /99 \/ 100/);
  assert.match(details, /local receipt-backed release readiness/i);
  assert.match(details, /Hosted\/provider benchmark publication is tracked separately/i);
  assert.match(details, /Source-owned runtime behavior for supported state slots, derived reads, effects, actions, DOM events, and client islands/i);
  assert.match(details, /no hidden React runtime claim/i);
  assert.match(details, /\/state-runtime/);
  assert.match(details, /\/islands/);
  assert.match(details, /their presence does not clear browser\/provider proof gates/i);
  assert.match(details, /State runtime browser replay has a receipt contract, freshness status, and stale reasons/i);
  assert.match(details, /Server-action replay ledger has a release-readiness receipt contract/i);
  assert.match(details, /Browser-receipt import bridge status:/i);
  assert.match(details, /source-owned import is now guarded/i);
  assert.match(details, /TypeScript harness can convert a real page snapshot into import candidates/i);
  assert.match(details, /validates imported real browser JSON before canonical JSON\/SR\/machine writes/i);
  assert.match(details, /dx www readiness --import-state-runtime-browser-receipt <browser-receipt\.json> --json --full/i);
  assert.match(details, /dx www readiness --import-native-event-browser-binder-receipt <browser-receipt\.json> --json --full/i);
  assert.match(details, /dx www readiness --import-visual-edit-browser-receipt <browser-receipt\.json> --json --full/i);
  assert.match(details, /dx www readiness --import-no-js-browser-receipt <browser-receipt\.json> --json --full/i);
  assert.match(details, /do not themselves run Browser\/Chrome or convert local receipts into hosted or release proof/i);
  assert.match(details, /dx-www-readiness-browser-receipt-import\.test\.ts/i);
  assert.match(details, /dx-www-readiness-browser-receipt-harness\.test\.ts/i);
  assert.match(details, /dx-www-readiness-browser-receipt-harness\.ts/i);
  assert.match(details, /import current browser receipts through the guarded source bridge/i);
  assert.match(details, /Local preview is separate from provider-hosted validation/i);
  assert.match(details, /Astro tiny-static parity is proven for the current controlled local/i);
  assert.match(details, /Static\/no-JS proof remains bounded to source\/output evidence/i);
  assert.match(details, /`tiny-static`, `data-dx-js="none"`, and no-JS artifact receipts are separate from hosted\/provider parity proof/i);
  assert.match(details, /React compatibility remains bounded/i);
  assert.match(details, /Browser, Lighthouse, release build, hosted provider, and broader runtime benchmark proof remain post-release hardening tracks/i);
  assert.doesNotMatch(details, /product99|product_99|Product 99/);
  assert.doesNotMatch(details, /\bv1\b/i);
  assert.doesNotMatch(details, /current claim (?:is|=) 100 \/ 100/i);
});
