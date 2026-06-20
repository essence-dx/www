const assert = require("assert");
const crypto = require("crypto");
const fs = require("fs");
const os = require("os");
const path = require("path");
const test = require("node:test");

const {
  buildSnapshot,
  renderMarkdown,
  verifySnapshotProvenance,
  writeSnapshot,
} = require("./measure-forge-installability-snapshot.ts");

function writeFixtureRoot() {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-forge-installability-"));
  const releaseBundle = path.join(root, ".dx", "forge-release-bundle-adoption");
  const reportsDir = path.join(root, "benchmarks", "reports");
  fs.mkdirSync(path.join(releaseBundle, "forge", "adoption"), { recursive: true });
  fs.mkdirSync(reportsDir, { recursive: true });

  fs.writeFileSync(
    path.join(releaseBundle, "forge-release-.dx/build-cache/manifest.json"),
    `${JSON.stringify(
      {
        signed: true,
        signature_verified: true,
        artifacts: [
          "forge/adoption/index.html",
          "forge/adoption.dxp",
          "forge/adoption.claims.json",
        ],
      },
      null,
      2
    )}\n`
  );
  fs.writeFileSync(path.join(releaseBundle, "forge", "adoption", "index.html"), "<!doctype html><h1>Forge</h1>");
  fs.writeFileSync(path.join(releaseBundle, "forge", "adoption.dxp"), Buffer.alloc(384, 1));
  fs.writeFileSync(path.join(releaseBundle, "forge", "adoption.claims.json"), "{\"route\":\"/forge/adoption\"}\n");

  writeUpdateReport(root, { installMs: 1420, greenUpgradeMs: 81, yellowUpgradeMs: 117 });
  fs.writeFileSync(
    path.join(reportsDir, "forge-source-owned-package-review.json"),
    `${JSON.stringify({ passed: true, no_node_modules: true, package_count: 3 }, null, 2)}\n`
  );

  return root;
}

function writeUpdateReport(root, timings) {
  const reportsDir = path.join(root, "benchmarks", "reports");
  fs.mkdirSync(reportsDir, { recursive: true });
  fs.writeFileSync(
    path.join(reportsDir, "forge-package-update-rehearsal.json"),
    `${JSON.stringify(
      {
        no_node_modules: true,
        prepare_result: { duration_ms: timings.installMs, passed: true },
        scenarios: {
          green_update: {
            passed: true,
            no_node_modules: true,
            update: { duration_ms: timings.greenUpgradeMs },
          },
          yellow_review_accept: {
            passed: true,
            no_node_modules: true,
            update: { duration_ms: timings.yellowUpgradeMs },
          },
        },
      },
      null,
      2
    )}\n`
  );
}

test("installability snapshot compares Forge install and upgrade against skipped npm and shadcn baselines", () => {
  const root = writeFixtureRoot();
  const outDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-forge-installability-out-"));
  const { privateKey } = crypto.generateKeyPairSync("ed25519");
  const privateKeyPem = privateKey.export({ type: "pkcs8", format: "pem" });
  const report = buildSnapshot({
    generatedAt: "2026-05-18T00:00:00.000Z",
    rootDir: root,
  });
  const markdown = renderMarkdown(report);
  const written = writeSnapshot(report, {
    outDir,
    provenancePrivateKeyPem: privateKeyPem,
    provenanceSigner: "essencefromexistence",
  });

  assert.equal(report.score, 100);
  assert.equal(report.passed, true);
  assert.equal(report.scope.no_package_installs_run, true);
  assert.equal(report.scope.not_live_npm_or_shadcn_benchmark, true);
  assert.equal(report.checks.no_package_installs.passed, true);
  assert.equal(report.checks.forge_installability.passed, true);
  assert.equal(report.checks.artifact_size.passed, true);
  assert.equal(report.checks.baseline_scope.passed, true);

  const rowsById = Object.fromEntries(report.rows.map((row) => [row.id, row]));
  assert.ok(rowsById["dx-forge-beta-install"].time_ms > 0);
  assert.ok(rowsById["dx-forge-beta-upgrade"].time_ms > 0);
  assert.equal(rowsById["dx-forge-beta-install"].package_install_ran, false);
  assert.equal(rowsById["dx-forge-beta-upgrade"].package_install_ran, false);
  assert.equal(rowsById["npm-install-baseline"].package_install_ran, false);
  assert.equal(rowsById["shadcn-add-baseline"].package_install_ran, false);
  assert.equal(rowsById["npm-install-baseline"].baseline_kind, "static-reference");
  assert.equal(rowsById["shadcn-add-baseline"].baseline_kind, "static-reference");

  assert.match(markdown, /DX Forge Installability Snapshot/);
  assert.match(markdown, /not a live npm\/shadcn install benchmark/);
  assert.match(markdown, /dx-forge-beta-install/);
  assert.match(markdown, /shadcn-add-baseline/);
  assert.ok(fs.existsSync(written.jsonPath));
  assert.ok(fs.existsSync(written.mdPath));
  assert.ok(fs.existsSync(written.provenanceJsonPath));
  assert.ok(fs.existsSync(written.provenanceMdPath));
  const provenance = JSON.parse(fs.readFileSync(written.provenanceJsonPath, "utf8"));
  assert.equal(provenance.signed, true);
  assert.equal(provenance.signature_verified, true);
  assert.equal(provenance.signer, "essencefromexistence");
  assert.equal(provenance.artifact_count, 2);
  assert.ok(provenance.artifacts.some((artifact) => artifact.path === "forge-installability-snapshot.json"));
  assert.ok(provenance.artifacts.some((artifact) => artifact.path === "forge-installability-snapshot.md"));
  assert.equal(verifySnapshotProvenance(written.provenanceJsonPath).passed, true);
  assert.equal(fs.existsSync(path.join(root, "node_modules")), false);
  assert.equal(fs.existsSync(path.join(outDir, "node_modules")), false);
});

test("installability snapshot writes trend history with release-to-release deltas", () => {
  const root = writeFixtureRoot();
  const outDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-forge-installability-history-"));

  const first = buildSnapshot({
    generatedAt: "2026-05-18T00:00:00.000Z",
    rootDir: root,
  });
  writeSnapshot(first, { outDir });

  writeUpdateReport(root, { installMs: 1000, greenUpgradeMs: 66, yellowUpgradeMs: 90 });
  const second = buildSnapshot({
    generatedAt: "2026-05-19T00:00:00.000Z",
    rootDir: root,
  });
  const written = writeSnapshot(second, { outDir });

  assert.ok(fs.existsSync(written.historyJsonPath));
  assert.ok(fs.existsSync(written.historyMdPath));
  const history = JSON.parse(fs.readFileSync(written.historyJsonPath, "utf8"));

  assert.equal(history.snapshot_count, 2);
  assert.equal(history.latest.generated_at, "2026-05-19T00:00:00.000Z");
  assert.equal(history.previous.generated_at, "2026-05-18T00:00:00.000Z");
  assert.equal(history.latest.beta_install_time_ms, 1000);
  assert.equal(history.latest.beta_upgrade_time_ms, 90);
  assert.equal(history.latest.delta.install_time_ms, -420);
  assert.equal(history.latest.delta.upgrade_time_ms, -27);
  assert.equal(history.latest.trend, "improved");
  assert.ok(fs.existsSync(path.join(outDir, "forge-installability-history", "snapshots", "2026-05-19T00-00-00-000Z.json")));

  const historyMarkdown = fs.readFileSync(written.historyMdPath, "utf8");
  assert.match(historyMarkdown, /DX Forge Installability Trend History/);
  assert.match(historyMarkdown, /2026-05-19T00:00:00.000Z/);
  assert.match(historyMarkdown, /-420 ms/);
  assert.equal(fs.existsSync(path.join(outDir, "node_modules")), false);
});
