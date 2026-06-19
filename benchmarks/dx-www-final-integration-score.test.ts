const assert = require("node:assert/strict");
const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const {
  collectWorktreeEvidence,
  evidenceFromReadinessGateReport,
  rustWarningCountFromText,
  scoreFinalIntegration,
  worktreeEvidenceFromCommandResults,
} = require("../tools/worktree/www-agent30-final-score.cjs");

const repoRoot = path.join(__dirname, "..");
const scorerPath = path.join(repoRoot, "tools", "worktree", "www-agent30-final-score.cjs");

test("final integration score is hard-capped by dirty worktree chaos before 95+ claims", () => {
  const report = scoreFinalIntegration({
    cargoBuildPassed: true,
    cargoCheckPassed: true,
    clippyPassed: false,
    conflictMarkerScanPassed: true,
    diffCheckPassed: true,
    dirtyEntryCount: 568,
    dxBuildPassed: true,
    focusedSuitePassed: true,
    fullCargoTestPassed: false,
    generatedArtifactsReviewed: false,
    httpProofPassed: true,
    rustWarningCount: 62,
    browserProofPassed: false,
    overlayRecoveryProofPassed: false,
  });

  assert.equal(report.schema, "dx.www.finalIntegrationScore");
  assert.equal(report.status, "blocked");
  assert.equal(report.score, 84);
  assert.ok(
    report.hardCaps.some((cap) => cap.id === "dirty-worktree-chaos" && cap.applied),
    JSON.stringify(report.hardCaps, null, 2),
  );
  assert.deepEqual(report.remainingBlockerIds.slice(0, 3), [
    "dirty-worktree-chaos",
    "generated-artifacts-unreviewed",
    "rust-warnings-present",
  ]);
  assert.match(report.nextAction, /curate or quarantine dirty entries/i);
});

test("final integration score reaches 100 only when all release proof is clean", () => {
  const report = scoreFinalIntegration({
    cargoBuildPassed: true,
    cargoCheckPassed: true,
    clippyPassed: true,
    conflictMarkerScanPassed: true,
    diffCheckPassed: true,
    dirtyEntryCount: 0,
    dxBuildPassed: true,
    focusedSuitePassed: true,
    fullCargoTestPassed: true,
    generatedArtifactsReviewed: true,
    httpProofPassed: true,
    rustWarningCount: 0,
    worktreeBlockingRiskCount: 0,
    browserProofPassed: true,
    overlayRecoveryProofPassed: true,
  });

  assert.equal(report.status, "ready");
  assert.equal(report.score, 100);
  assert.deepEqual(report.remainingBlockerIds, []);
});

test("final integration score is capped by worktree ownership blockers", () => {
  const report = scoreFinalIntegration({
    cargoBuildPassed: true,
    cargoCheckPassed: true,
    clippyPassed: true,
    conflictMarkerScanPassed: true,
    diffCheckPassed: true,
    dirtyEntryCount: 1,
    dxBuildPassed: true,
    focusedSuitePassed: true,
    fullCargoTestPassed: true,
    generatedArtifactsReviewed: true,
    httpProofPassed: true,
    rustWarningCount: 0,
    worktreeBlockingRiskCount: 1,
    browserProofPassed: true,
    overlayRecoveryProofPassed: true,
  });

  assert.equal(report.score, 83);
  assert.ok(
    report.remainingBlockerIds.includes("worktree-ownership-blockers"),
    JSON.stringify(report.remainingBlockerIds),
  );
});

test("final integration scorer CLI reads evidence JSON and exits nonzero when capped", () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-www-final-score-"));
  const evidencePath = path.join(root, "evidence.json");
  fs.writeFileSync(
    evidencePath,
    `${JSON.stringify({
      cargoBuildPassed: true,
      cargoCheckPassed: true,
      clippyPassed: true,
      conflictMarkerScanPassed: true,
      diffCheckPassed: true,
      dirtyEntryCount: 12,
      dxBuildPassed: true,
      focusedSuitePassed: true,
      fullCargoTestPassed: true,
      generatedArtifactsReviewed: true,
      httpProofPassed: true,
      rustWarningCount: 2,
      worktreeBlockingRiskCount: 0,
      browserProofPassed: true,
      overlayRecoveryProofPassed: true,
    })}\n`,
  );

  const result = spawnSync(process.execPath, [scorerPath, "--evidence", evidencePath, "--json"], {
    cwd: repoRoot,
    encoding: "utf8",
    windowsHide: true,
  });

  assert.equal(result.status, 1, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  assert.equal(report.status, "blocked");
  assert.equal(report.score, 92);
  assert.deepEqual(report.remainingBlockerIds, [
    "dirty-worktree-present",
    "rust-warnings-nonzero",
  ]);
});

test("final integration scorer derives proof-bundle evidence without treating skipped HTTP probes as proof", () => {
  const evidence = evidenceFromReadinessGateReport({
    proofBundle: {
      steps: [
        {
          id: "cargo-check-dx-www-cli",
          kind: "command",
          passed: true,
          status: "passed",
          stderrTail: [
            "warning: unreachable pattern",
            "warning: `dx-www` (lib) generated 1 warning",
          ].join("\n"),
        },
        {
          id: "focused-readiness-node-test",
          kind: "command",
          passed: true,
          status: "passed",
        },
        {
          id: "dx-build-installed-smoke",
          kind: "command",
          passed: false,
          status: "failed",
        },
        {
          id: "safe-http-root-probe",
          kind: "http-probe",
          passed: true,
          status: "skipped",
        },
      ],
    },
  });

  assert.equal(evidence.cargoCheckPassed, true);
  assert.equal(evidence.focusedSuitePassed, true);
  assert.equal(evidence.dxBuildPassed, false);
  assert.equal(evidence.httpProofPassed, false);
  assert.equal(evidence.rustWarningCount, 1);
});

test("final integration scorer CLI merges readiness report evidence with explicit worktree evidence", () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-www-final-score-report-"));
  const readinessReportPath = path.join(root, "readiness-report.json");
  const evidencePath = path.join(root, "evidence.json");

  fs.writeFileSync(
    readinessReportPath,
    `${JSON.stringify({
      proofBundle: {
        steps: [
          {
            id: "cargo-check-dx-www-cli",
            kind: "command",
            passed: true,
            status: "passed",
            stderrTail: "Finished `dev` profile",
          },
          {
            id: "focused-readiness-node-test",
            kind: "command",
            passed: true,
            status: "passed",
          },
          {
            id: "dx-build-installed-smoke",
            kind: "command",
            passed: true,
            status: "passed",
          },
          {
            id: "safe-http-root-probe",
            kind: "http-probe",
            passed: true,
            status: "passed",
          },
        ],
      },
    })}\n`,
  );
  fs.writeFileSync(
    evidencePath,
    `${JSON.stringify({
      cargoBuildPassed: true,
      clippyPassed: true,
      conflictMarkerScanPassed: true,
      diffCheckPassed: true,
      dirtyEntryCount: 12,
      fullCargoTestPassed: true,
      generatedArtifactsReviewed: true,
      worktreeBlockingRiskCount: 0,
      browserProofPassed: true,
      overlayRecoveryProofPassed: true,
    })}\n`,
  );

  const result = spawnSync(
    process.execPath,
    [scorerPath, "--readiness-report", readinessReportPath, "--evidence", evidencePath, "--json"],
    {
      cwd: repoRoot,
      encoding: "utf8",
      windowsHide: true,
    },
  );

  assert.equal(result.status, 1, result.stdout + result.stderr);
  const report = JSON.parse(result.stdout);
  assert.equal(report.score, 92);
  assert.equal(report.evidence.cargoCheckPassed, true);
  assert.equal(report.evidence.dxBuildPassed, true);
  assert.equal(report.evidence.focusedSuitePassed, true);
  assert.equal(report.evidence.httpProofPassed, true);
  assert.equal(report.evidence.rustWarningCount, 0);
  assert.deepEqual(report.remainingBlockerIds, ["dirty-worktree-present"]);
});

test("final integration scorer warning parser honors cargo generated-warning summaries", () => {
  assert.equal(rustWarningCountFromText("warning: one\nwarning: two\n"), 2);
  assert.equal(rustWarningCountFromText("warning: first\nwarning: `dx-www` generated 6 warnings\n"), 6);
});

test("final integration scorer derives cheap worktree evidence from command results", () => {
  assert.deepEqual(
    worktreeEvidenceFromCommandResults({
      conflictScanStatus: 1,
      diffCheckStatus: 0,
      statusStatus: 0,
      statusStdout: " M dx-www/src/error.rs\n?? tools/worktree/www-agent30-final-score.cjs\n",
    }),
    {
      conflictMarkerScanPassed: true,
      diffCheckPassed: true,
      dirtyEntryCount: 2,
    },
  );
  assert.equal(
    worktreeEvidenceFromCommandResults({
      conflictScanStatus: 0,
      diffCheckStatus: 1,
      statusStatus: 0,
      statusStdout: "",
    }).conflictMarkerScanPassed,
    false,
  );
});

test("final integration score includes an evidence cap table for score-audit handoff", () => {
  const report = scoreFinalIntegration({
    cargoBuildPassed: false,
    cargoCheckPassed: false,
    conflictMarkerScanPassed: true,
    diffCheckPassed: true,
    dirtyEntryCount: 17,
    generatedArtifactsReviewed: false,
  });

  assert.ok(Array.isArray(report.scoreTable));
  assert.ok(report.scoreTable.length >= report.hardCaps.length);
  assert.deepEqual(report.scoreTable.slice(0, 4), [
    {
      id: "dirty-worktree-chaos",
      status: "clear",
      cap: 84,
      scoreImpact: "none",
      issue: "dirty worktree has 500 or more entries",
      nextAction: "Curate or quarantine dirty entries before raising the final score.",
    },
    {
      id: "worktree-ownership-blockers",
      status: "clear",
      cap: 83,
      scoreImpact: "none",
      issue: "worktree ownership map still has blocking risks",
      nextAction: "Resolve unclassified or blocking ownership risks before raising the final score.",
    },
    {
      id: "dirty-worktree-present",
      status: "applied",
      cap: 92,
      scoreImpact: "caps at 92",
      issue: "dirty worktree still has uncommitted entries",
      nextAction: "Reduce the dirty worktree to reviewed release-control slices.",
    },
    {
      id: "generated-artifacts-unreviewed",
      status: "applied",
      cap: 88,
      scoreImpact: "caps at 88",
      issue: "generated artifacts have not been reviewed or quarantined",
      nextAction: "Review generated .dx, receipt, report, and temp artifacts.",
    },
  ]);
});

test("final integration scorer conflict scan ignores target star build directories", () => {
  const calls = [];
  collectWorktreeEvidence("G:/Dx/www", (command, args, options) => {
    calls.push({ command, args, cwd: options.cwd });
    if (command === "git" && args[0] === "status") {
      return { status: 0, stdout: "" };
    }
    if (command === "git" && args[0] === "diff") {
      return { status: 0, stdout: "" };
    }
    if (command === "rg") {
      return { status: 1, stdout: "" };
    }
    throw new Error(`unexpected command: ${command}`);
  });

  const conflictScan = calls.find((call) => call.command === "rg");
  assert.ok(conflictScan, JSON.stringify(calls, null, 2));
  assert.ok(
    conflictScan.args.includes("!target-*/**"),
    `conflict scan args: ${JSON.stringify(conflictScan.args)}`,
  );
});
