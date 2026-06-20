import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const {
  buildLane12CommitPlanReport,
  buildLane12StagedOwnershipReport,
  buildLane12OwnershipReport,
  classifyStatusLines,
  readCurrentGitStatus,
  readStagedGitStatus,
  runCli,
  summarizeOwnership,
} = require("../tools/worktree/lane12-ownership.cjs");

const repoRoot = path.resolve(__dirname, "..");
const ownershipCli = path.join(repoRoot, "tools/worktree/lane12-ownership.cjs");
const ownershipRulesModule = path.join(repoRoot, "tools/worktree/lane12-ownership-rules.cjs");
const ownershipReportModule = path.join(repoRoot, "tools/worktree/lane12-ownership-report.cjs");

test("Lane 12 ownership CLI keeps rule and report code in focused modules", () => {
  assert.equal(fs.existsSync(ownershipRulesModule), true);
  assert.equal(fs.existsSync(ownershipReportModule), true);
  const cliLineCount = fs.readFileSync(ownershipCli, "utf8").split(/\r?\n/).length;
  assert.ok(cliLineCount < 180, `expected CLI wrapper below 180 lines, got ${cliLineCount}`);
});

test("Lane 12 worktree ownership preserves scope cleanup from mixed commits", () => {
  const entries = classifyStatusLines([
    " D -",
    " M DX.md",
    "?? benchmarks/dx-scope-removal-contract.test.ts",
    " M dx-www/src/dev/hot_reload_stream.rs",
    " M core/src/delivery/route_handler_ai.rs",
    " M related-crates/style/src/core/engine/mod.rs",
    "?? tools/dx-style/",
    "?? tools/worktree/lane12-ownership.cjs",
    "",
  ]);
  const byPath = new Map(entries.map((entry) => [entry.path, entry]));

  assert.equal(byPath.get("-").owner, "workspace-junk-deletion");
  assert.equal(byPath.get("-").commitPolicy, "preserve-deletion");
  assert.equal(byPath.get("-").lane12Stageable, false);

  assert.equal(byPath.get("DX.md").lane, 12);
  assert.equal(byPath.get("benchmarks/dx-scope-removal-contract.test.ts").lane, 12);
  assert.equal(byPath.get("tools/worktree/lane12-ownership.cjs").lane, 12);

  assert.equal(byPath.get("dx-www/src/dev/hot_reload_stream.rs").lane, 7);
  assert.equal(byPath.get("core/src/delivery/route_handler_ai.rs").lane, 5);
  assert.equal(byPath.get("related-crates/style/src/core/engine/mod.rs").owner, "dx-style");
  assert.equal(byPath.get("tools/dx-style/").owner, "dx-style");
  assert.equal(byPath.get("tools/dx-style/").lane12Stageable, false);

  const summary = summarizeOwnership(entries);
  assert.equal(summary.mixedCommitRisk, true);
  assert.deepEqual(summary.lane12StageablePaths, [
    "DX.md",
    "benchmarks/dx-scope-removal-contract.test.ts",
    "tools/worktree/lane12-ownership.cjs",
  ]);
  assert.deepEqual(summary.preserveOnlyPaths, ["-"]);
  assert.ok(summary.groups.some((group) => group.owner === "hot-reload-dev-server"));
  assert.ok(summary.groups.some((group) => group.owner === "route-handlers"));
  assert.ok(summary.groups.some((group) => group.owner === "dx-style"));
});

test("Lane 12 worktree ownership parses rename status lines without staging both sides", () => {
  const entries = classifyStatusLines([
    "R  docs/old-scope.md -> docs/scope-reset.md",
    "?? benchmarks/benchmark-report-scope-contract.test.ts",
  ]);

  assert.equal(entries[0].path, "docs/scope-reset.md");
  assert.equal(entries[0].originalPath, "docs/old-scope.md");
  assert.equal(entries[0].lane, 12);
  assert.equal(entries[0].lane12Stageable, true);
  assert.equal(entries[1].lane, 12);
});

test("Lane 12 strict report blocks mixed commits without hiding stageable scope files", () => {
  const report = buildLane12OwnershipReport([
    " M DX.md",
    "?? benchmarks/lane12-worktree-ownership.test.ts",
    " D -",
    " M dx-www/src/dev/axum_server.rs",
    "?? unknown-worker-output.txt",
  ]);

  assert.equal(report.schema, "dx.www.worktree.lane12Ownership");
  assert.equal(report.lane, 12);
  assert.equal(report.status, "blocked");
  assert.equal(report.blockedLane12Commit, true);
  assert.deepEqual(report.summary.lane12StageablePaths, [
    "DX.md",
    "benchmarks/lane12-worktree-ownership.test.ts",
  ]);
  assert.deepEqual(report.summary.preserveOnlyPaths, ["-"]);
  assert.deepEqual(report.blockers.map((blocker) => blocker.id), [
    "foreign-owner-dirty-files",
    "unclassified-dirty-files",
    "preserve-only-deletions",
  ]);
  assert.match(report.nextAction, /stage only summary\.lane12StageablePaths/);
});

test("Lane 12 strict CLI exits nonzero for mixed-owner status input", () => {
  const result = spawnSync(process.execPath, [ownershipCli, "--strict"], {
    cwd: repoRoot,
    encoding: "utf8",
    input: [" M DX.md", " M dx-www/src/dev/hot_reload_stream.rs", ""].join("\n"),
  });

  assert.equal(result.status, 2);
  const report = JSON.parse(result.stdout);
  assert.equal(report.status, "blocked");
  assert.equal(report.blockedLane12Commit, true);
  assert.match(result.stderr, /Lane 12 strict check blocked mixed worktree ownership/);
});

test("Lane 12 compact report keeps strict output actionable for huge dirty trees", () => {
  const report = buildLane12OwnershipReport(
    [
      " M DX.md",
      "?? benchmarks/lane12-worktree-ownership.test.ts",
      " M dx-www/src/dev/axum_server.rs",
      " M dx-www/src/dev/hot_reload_stream.rs",
      " M related-crates/style/src/core/engine/mod.rs",
      "?? unknown-worker-output.txt",
    ],
    { compact: true, sampleLimit: 1 },
  );

  assert.equal(report.compact, true);
  assert.equal("entries" in report, false);
  assert.equal(report.summary.entryCount, 6);
  assert.deepEqual(report.summary.lane12StageablePaths, [
    "DX.md",
    "benchmarks/lane12-worktree-ownership.test.ts",
  ]);
  assert.equal(report.summary.foreignOwnerCount, 3);
  assert.equal(report.summary.unclassifiedCount, 1);
  assert.deepEqual(report.summary.groups.find((group) => group.owner === "hot-reload-dev-server"), {
    owner: "hot-reload-dev-server",
    lane: 7,
    count: 2,
    samplePaths: ["dx-www/src/dev/axum_server.rs"],
  });
  assert.deepEqual(report.blockers.find((blocker) => blocker.id === "foreign-owner-dirty-files").samplePaths, [
    "dx-www/src/dev/axum_server.rs",
  ]);
});

test("Lane 12 compact strict CLI omits full entries but still blocks mixed commits", () => {
  const result = spawnSync(process.execPath, [ownershipCli, "--strict", "--compact"], {
    cwd: repoRoot,
    encoding: "utf8",
    input: [" M DX.md", " M dx-www/src/dev/hot_reload_stream.rs", ""].join("\n"),
  });

  assert.equal(result.status, 2);
  const report = JSON.parse(result.stdout);
  assert.equal(report.compact, true);
  assert.equal("entries" in report, false);
  assert.equal(report.status, "blocked");
  assert.deepEqual(report.summary.lane12StageablePaths, ["DX.md"]);
  assert.match(result.stderr, /Lane 12 strict check blocked mixed worktree ownership/);
});

test("Lane 12 staged report passes isolated Lane 12 index changes without unstaged noise", () => {
  const report = buildLane12StagedOwnershipReport(
    [
      "M\tDX.md",
      "A\ttools/worktree/lane12-ownership.cjs",
      "D\tcore/src/devtools.rs",
    ].join("\n"),
    { compact: true },
  );

  assert.equal(report.executionMode, "read-only-staged-index-classifier");
  assert.equal(report.status, "passed");
  assert.equal(report.blockedLane12Commit, false);
  assert.deepEqual(report.summary.lane12StageablePaths, [
    "DX.md",
    "tools/worktree/lane12-ownership.cjs",
    "core/src/devtools.rs",
  ]);
  assert.deepEqual(report.summary.preserveOnlyPaths, []);
  assert.equal(report.summary.foreignOwnerCount, 0);
  assert.equal(report.summary.unclassifiedCount, 0);
  assert.deepEqual(report.blockers, []);
});

test("Lane 12 staged report blocks foreign or preserve-only paths before commit", () => {
  const report = buildLane12StagedOwnershipReport(
    [
      "M\tDX.md",
      "M\tdx-www/src/dev/hot_reload_stream.rs",
      "D\t-",
      "R100\tdocs/old-scope.md\tdocs/scope-reset.md",
    ].join("\n"),
  );

  assert.equal(report.status, "blocked");
  assert.equal(report.blockedLane12Commit, true);
  assert.deepEqual(report.summary.lane12StageablePaths, ["DX.md", "docs/scope-reset.md"]);
  assert.deepEqual(report.summary.preserveOnlyPaths, ["-"]);
  assert.deepEqual(report.blockers.map((blocker) => blocker.id), [
    "foreign-owner-dirty-files",
    "preserve-only-deletions",
  ]);
  assert.equal(report.entries.find((entry) => entry.path === "docs/scope-reset.md").originalPath, "docs/old-scope.md");
});

test("Lane 12 staged report blocks generated artifacts and node_modules boundaries explicitly", () => {
  const report = buildLane12StagedOwnershipReport(
    [
      "M\tDX.md",
      "A\t.dx/receipts/build/installed-binary-smoke-latest.json",
      "A\texamples/template/.dx/build/server-data/route-.dx/build-cache/manifest.json",
      "A\texamples/template/.dx/forge/receipts/package.json",
      "A\texamples/template/node_modules/.bin/next",
    ].join("\n"),
    { compact: true },
  );

  assert.equal(report.status, "blocked");
  assert.equal(report.blockedLane12Commit, true);
  assert.deepEqual(report.summary.lane12StageablePaths, ["DX.md"]);
  assert.equal(report.summary.generatedArtifactCount, 3);
  assert.equal(report.summary.nodeModulesBoundaryCount, 1);
  assert.deepEqual(report.summary.generatedArtifactPaths, [
    ".dx/receipts/build/installed-binary-smoke-latest.json",
    "examples/template/.dx/build/server-data/route-.dx/build-cache/manifest.json",
    "examples/template/.dx/forge/receipts/package.json",
  ]);
  assert.deepEqual(report.summary.nodeModulesBoundaryPaths, [
    "examples/template/node_modules/.bin/next",
  ]);
  assert.deepEqual(report.blockers.map((blocker) => blocker.id), [
    "foreign-owner-dirty-files",
    "generated-artifact-dirty-files",
    "node-modules-boundary-dirty-files",
  ]);
});

test("Lane 12 staged status reads avoid optional git index writes", () => {
  let observed = null;
  const statusText = readStagedGitStatus("G:/Dx/www", {
    spawn: (command, args, options) => {
      observed = { command, args, options };
      return { status: 0, stdout: "M\tDX.md\n", stderr: "" };
    },
  });

  assert.equal(statusText, "M\tDX.md\n");
  assert.deepEqual(observed, {
    command: "git",
    args: ["--no-optional-locks", "diff", "--cached", "--name-status", "--find-renames"],
    options: {
      cwd: "G:/Dx/www",
      encoding: "utf8",
    },
  });
});

test("Lane 12 staged CLI can validate only the index without reading dirty worktree input", () => {
  let stdoutText = "";
  let stderrText = "";
  runCli({
    argv: ["--staged", "--compact"],
    cwd: "G:/Dx/www",
    stdin: { fd: -1 },
    stdout: { write: (text) => (stdoutText += text) },
    stderr: { write: (text) => (stderrText += text) },
    readStagedStatus: () => "M\tDX.md\nA\ttools/worktree/lane12-ownership.cjs\n",
  });

  assert.equal(stderrText, "");
  const report = JSON.parse(stdoutText);
  assert.equal(report.executionMode, "read-only-staged-index-classifier");
  assert.equal(report.status, "passed");
  assert.equal(report.blockedLane12Commit, false);
  assert.deepEqual(report.summary.lane12StageablePaths, [
    "DX.md",
    "tools/worktree/lane12-ownership.cjs",
  ]);
});

test("Lane 12 stageable-only CLI prints copyable safe paths without full entries", () => {
  const result = spawnSync(process.execPath, [ownershipCli, "--stageable-only"], {
    cwd: repoRoot,
    encoding: "utf8",
    input: [
      " M DX.md",
      "?? tools/worktree/lane12-ownership.cjs",
      " D -",
      " M dx-www/src/dev/hot_reload_stream.rs",
      " M core/src/lib.rs",
      "",
    ].join("\n"),
  });

  assert.equal(result.status, 0);
  const report = JSON.parse(result.stdout);
  assert.equal(report.schema, "dx.www.worktree.lane12StageablePaths");
  assert.equal(report.lane, 12);
  assert.equal(report.status, "blocked");
  assert.equal(report.blockedLane12Commit, true);
  assert.deepEqual(report.lane12StageablePaths, [
    "DX.md",
    "tools/worktree/lane12-ownership.cjs",
  ]);
  assert.deepEqual(report.preserveOnlyPaths, ["-"]);
  assert.equal(report.foreignOwnerCount, 2);
  assert.equal(report.unclassifiedCount, 0);
  assert.equal("entries" in report, false);
  assert.equal("summary" in report, false);
  assert.match(report.nextAction, /stage only lane12StageablePaths/);
});

test("Lane 12 commit plan separates stageable, preserved, generated, and foreign work", () => {
  const report = buildLane12OwnershipReport([
    " M DX.md",
    " D core/src/devtools.rs",
    " D -",
    " M dx-www/src/dev/hot_reload_stream.rs",
    "?? .dx/receipts/build/latest.json",
    "?? unowned-output.txt",
  ]);
  const plan = buildLane12CommitPlanReport(report, { sampleLimit: 2 });

  assert.equal(plan.schema, "dx.www.worktree.commitPlan");
  assert.equal(plan.status, "blocked");
  assert.deepEqual(plan.stageGroups, [
    {
      owner: "scope-cleanup-docs-truth",
      lane: 12,
      action: "stage",
      count: 1,
      paths: ["DX.md"],
    },
    {
      owner: "scope-removal-deletion",
      lane: 12,
      action: "stage",
      count: 1,
      paths: ["core/src/devtools.rs"],
    },
  ]);
  assert.deepEqual(plan.holdGroups.find((group) => group.id === "preserve-only-deletions"), {
    id: "preserve-only-deletions",
    action: "preserve-do-not-stage",
    count: 1,
    paths: ["-"],
  });
  assert.deepEqual(plan.holdGroups.find((group) => group.id === "generated-artifacts"), {
    id: "generated-artifacts",
    action: "hold-for-artifact-owner-review",
    count: 1,
    samplePaths: [".dx/receipts/build/latest.json"],
  });
  assert.deepEqual(plan.holdGroups.find((group) => group.id === "unclassified"), {
    id: "unclassified",
    action: "assign-owner-before-staging",
    count: 1,
    samplePaths: ["unowned-output.txt"],
  });
  assert.deepEqual(plan.foreignOwnerGroups, [
    {
      owner: "hot-reload-dev-server",
      lane: 7,
      action: "leave-to-owner-lane",
      count: 1,
      samplePaths: ["dx-www/src/dev/hot_reload_stream.rs"],
    },
  ]);
  assert.deepEqual(plan.remainingBlockerIds, [
    "foreign-owner-dirty-files",
    "generated-artifact-dirty-files",
    "unclassified-dirty-files",
    "preserve-only-deletions",
  ]);
});

test("Lane 12 owners CLI prints grouped ownership without full path lists", () => {
  const result = spawnSync(process.execPath, [ownershipCli, "--owners"], {
    cwd: repoRoot,
    encoding: "utf8",
    input: [
      " M DX.md",
      "?? tools/worktree/lane12-ownership.cjs",
      " D -",
      " M dx-www/src/dev/hot_reload_stream.rs",
      " M core/src/lib.rs",
      "",
    ].join("\n"),
  });

  assert.equal(result.status, 0);
  const report = JSON.parse(result.stdout);
  assert.equal(report.schema, "dx.www.worktree.ownerSummary");
  assert.equal(report.lane, 12);
  assert.equal(report.status, "blocked");
  assert.equal(report.entryCount, 5);
  assert.equal(report.foreignOwnerCount, 2);
  assert.equal(report.unclassifiedCount, 0);
  assert.equal("entries" in report, false);
  assert.equal("summary" in report, false);
  assert.deepEqual(
    report.owners.map((owner) => owner.owner),
    [
      "scope-cleanup-docs-truth",
      "workspace-junk-deletion",
      "hot-reload-dev-server",
      "shared-public-api",
    ],
  );
  assert.deepEqual(report.owners.find((owner) => owner.owner === "scope-cleanup-docs-truth"), {
    owner: "scope-cleanup-docs-truth",
    lane: 12,
    count: 2,
    samplePaths: ["DX.md", "tools/worktree/lane12-ownership.cjs"],
  });
});

test("Lane 12 commit-plan CLI emits the handoff plan without full entries", () => {
  const result = spawnSync(process.execPath, [ownershipCli, "--commit-plan"], {
    cwd: repoRoot,
    encoding: "utf8",
    input: [
      " M DX.md",
      " M dx-www/src/dev/hot_reload_stream.rs",
      "?? .dx/receipts/build/latest.json",
      "",
    ].join("\n"),
  });

  assert.equal(result.status, 0);
  const plan = JSON.parse(result.stdout);
  assert.equal(plan.schema, "dx.www.worktree.commitPlan");
  assert.equal("entries" in plan, false);
  assert.deepEqual(plan.stageGroups.map((group) => group.owner), ["scope-cleanup-docs-truth"]);
  assert.deepEqual(plan.foreignOwnerGroups.map((group) => group.owner), ["hot-reload-dev-server"]);
  assert.deepEqual(plan.holdGroups.map((group) => group.id), ["generated-artifacts"]);
});

test("Lane 12 current status reads avoid optional git index writes", () => {
  let observed = null;
  const statusText = readCurrentGitStatus("G:/Dx/www", {
    spawn: (command, args, options) => {
      observed = { command, args, options };
      return { status: 0, stdout: " M DX.md\n", stderr: "" };
    },
  });

  assert.equal(statusText, " M DX.md\n");
  assert.deepEqual(observed, {
    command: "git",
    args: ["--no-optional-locks", "status", "--short"],
    options: {
      cwd: "G:/Dx/www",
      encoding: "utf8",
    },
  });
});

test("Lane 12 CLI defaults to current status when stdin is empty", () => {
  let stdoutText = "";
  let stderrText = "";
  let currentReads = 0;
  const previousExitCode = process.exitCode;

  try {
    runCli({
      argv: ["--strict", "--compact"],
      cwd: "G:/Dx/www",
      stdin: { fd: -1, isTTY: true },
      stdout: { write: (text) => (stdoutText += text) },
      stderr: { write: (text) => (stderrText += text) },
      readCurrentStatus: () => {
        currentReads += 1;
        return [" M DX.md", " M dx-www/src/dev/hot_reload_stream.rs", ""].join("\n");
      },
    });

    assert.equal(process.exitCode, 2);
  } finally {
    process.exitCode = previousExitCode;
  }

  assert.equal(currentReads, 1);
  const report = JSON.parse(stdoutText);
  assert.equal(report.status, "blocked");
  assert.equal(report.blockedLane12Commit, true);
  assert.equal(report.summary.entryCount, 2);
  assert.deepEqual(report.summary.lane12StageablePaths, ["DX.md"]);
  assert.match(stderrText, /Lane 12 strict check blocked mixed worktree ownership/);
});

test("Lane 12 classifies launch status and scope audit files without taking runtime lanes", () => {
  const entries = classifyStatusLines([
    " M dx-www/README.md",
    " M benchmarks/measure-current-status.ts",
    " M benchmarks/measure-forge-package-update-rehearsal.ts",
    " M benchmarks/measure-real-routes.ts",
    " M tools/next-rust-merge/audit-gap-check-map.json",
    " M tools/next-rust-merge/coordinator-readiness-audit.cjs",
    " M dx-www/src/cli/launch_readiness_bundle.rs",
    " M dx-www/src/cli/public_framework_tools.rs",
    " M dx-www/src/cli/mod.rs",
    " M dx-www/src/dev/axum_server.rs",
    " M tools/build-graph/turbo-tasks-adapter.ts",
  ]);
  const byPath = new Map(entries.map((entry) => [entry.path, entry]));

  for (const lane12Path of [
    "dx-www/README.md",
    "benchmarks/measure-current-status.ts",
    "benchmarks/measure-forge-package-update-rehearsal.ts",
    "benchmarks/measure-real-routes.ts",
    "tools/next-rust-merge/audit-gap-check-map.json",
    "tools/next-rust-merge/coordinator-readiness-audit.cjs",
    "dx-www/src/cli/launch_readiness_bundle.rs",
    "dx-www/src/cli/public_framework_tools.rs",
  ]) {
    assert.equal(byPath.get(lane12Path).lane, 12);
    assert.equal(byPath.get(lane12Path).owner, "scope-status-coordination");
    assert.equal(byPath.get(lane12Path).lane12Stageable, true);
  }

  assert.equal(byPath.get("dx-www/src/cli/mod.rs").lane, 9);
  assert.equal(byPath.get("dx-www/src/dev/axum_server.rs").lane, 7);
  assert.equal(byPath.get("tools/build-graph/turbo-tasks-adapter.ts").lane, 2);

  const summary = summarizeOwnership(entries);
  assert.deepEqual(summary.lane12StageablePaths, [
    "dx-www/README.md",
    "benchmarks/measure-current-status.ts",
    "benchmarks/measure-forge-package-update-rehearsal.ts",
    "benchmarks/measure-real-routes.ts",
    "tools/next-rust-merge/audit-gap-check-map.json",
    "tools/next-rust-merge/coordinator-readiness-audit.cjs",
    "dx-www/src/cli/launch_readiness_bundle.rs",
    "dx-www/src/cli/public_framework_tools.rs",
  ]);
});

test("Lane 12 classifier assigns common dirty buckets without staging shared coordinator work", () => {
  const entries = classifyStatusLines([
    " M benchmarks/app-router-server-data-build-contract.test.ts",
    " M dx-www/src/cli/app_router_server_data.rs",
    "?? dx-www/src/cli/app_server_data_manifest.rs",
    " M dx-www/tests/app_router_server_data.rs",
    " M core/src/delivery/server_contract.rs",
    " M core/src/delivery/tests.rs",
    " M dx-www/src/api/mod.rs",
    " M core/src/ecosystem/forge_registry.rs",
    " M dx-www/src/cli/forge_hosting_manifest.rs",
    "?? dx-www/src/cli/forge_public_add.rs",
    " M dx-www/src/cli/forge_react_starter_benchmark.rs",
    " M dx-www/src/cli/hosted_preview_contract.rs",
    " M dx-www/src/cli/studio_manifest.rs",
    " M dx-www/src/cli/studio_manifest/hot_reload_manifest.rs",
    " M examples/conversion-proof/pages/index.html",
    " M dx-www/src/cli/dx_style_support.rs",
    "?? .dx/receipts/build/installed-binary-smoke-latest.json",
    "?? benchmarks/installed-smoke-route-output.test.ts",
    "?? benchmarks/app-router-page-discovery-collisions.test.ts",
    "?? benchmarks/dx-app-router-catch-all-semantics.test.ts",
    "?? dx-www/src/cli/build_options.rs",
    "?? dx-www/src/cli/migrate_command.rs",
    "?? dx-www/src/cli/promote_command.rs",
    "?? dx-www/src/cli/rollback_command.rs",
    "?? dx-www/src/cli/next_rust_status.rs",
    " M tools/launch-stabilize/coordinator.cjs",
  ]);
  const byPath = new Map(entries.map((entry) => [entry.path, entry]));

  for (const lane2Path of [
    "benchmarks/app-router-server-data-build-contract.test.ts",
    "dx-www/src/cli/app_router_server_data.rs",
    "dx-www/src/cli/app_server_data_manifest.rs",
    "dx-www/tests/app_router_server_data.rs",
  ]) {
    assert.equal(byPath.get(lane2Path).lane, 2);
    assert.equal(byPath.get(lane2Path).owner, "source-build-graph");
  }

  for (const lane5Path of [
    "core/src/delivery/server_contract.rs",
    "core/src/delivery/tests.rs",
    "dx-www/src/api/mod.rs",
  ]) {
    assert.equal(byPath.get(lane5Path).lane, 5);
    assert.equal(byPath.get(lane5Path).owner, "route-handlers");
  }

  for (const lane10Path of [
    "core/src/ecosystem/forge_registry.rs",
    "dx-www/src/cli/forge_hosting_manifest.rs",
    "dx-www/src/cli/forge_public_add.rs",
    "dx-www/src/cli/forge_react_starter_benchmark.rs",
    "dx-www/src/cli/hosted_preview_contract.rs",
    "dx-www/src/cli/studio_manifest.rs",
    "dx-www/src/cli/studio_manifest/hot_reload_manifest.rs",
    "examples/conversion-proof/pages/index.html",
  ]) {
    assert.equal(byPath.get(lane10Path).lane, 10);
    assert.equal(byPath.get(lane10Path).owner, "template-product-path");
  }

  assert.equal(byPath.get("dx-www/src/cli/dx_style_support.rs").owner, "dx-style");
  assert.equal(byPath.get("dx-www/src/cli/dx_style_support.rs").lane12Stageable, false);
  assert.equal(byPath.get(".dx/receipts/build/installed-binary-smoke-latest.json").owner, "generated-build-artifact");
  assert.equal(byPath.get(".dx/receipts/build/installed-binary-smoke-latest.json").sourceBoundary, "generated-artifact");
  assert.equal(byPath.get(".dx/receipts/build/installed-binary-smoke-latest.json").lane12Stageable, false);
  assert.equal(byPath.get("benchmarks/installed-smoke-route-output.test.ts").lane, 1);
  assert.equal(byPath.get("benchmarks/app-router-page-discovery-collisions.test.ts").lane, 3);
  assert.equal(byPath.get("benchmarks/dx-app-router-catch-all-semantics.test.ts").lane, 3);

  for (const lane9Path of [
    "dx-www/src/cli/build_options.rs",
    "dx-www/src/cli/migrate_command.rs",
    "dx-www/src/cli/promote_command.rs",
    "dx-www/src/cli/rollback_command.rs",
    "dx-www/src/cli/next_rust_status.rs",
  ]) {
    assert.equal(byPath.get(lane9Path).lane, 9);
    assert.equal(byPath.get(lane9Path).owner, "cli-architecture-split");
  }

  assert.equal(byPath.get("tools/launch-stabilize/coordinator.cjs").owner, "shared-launch-coordinator");
  assert.equal(byPath.get("tools/launch-stabilize/coordinator.cjs").lane, null);
  assert.equal(byPath.get("tools/launch-stabilize/coordinator.cjs").lane12Stageable, false);
  const summary = summarizeOwnership(entries);
  assert.deepEqual(summary.generatedArtifactPaths, [
    ".dx/receipts/build/installed-binary-smoke-latest.json",
  ]);
  assert.deepEqual(summary.unclassifiedPaths, []);
});

test("Lane 12 scope rules do not steal report-named files from their lane owners", () => {
  const entries = classifyStatusLines([
    "?? benchmarks/dx-build-installed-smoke-report.test.ts",
    "?? benchmarks/dx-build-installed-smoke-human-report.test.ts",
    "?? benchmarks/react-starter-benchmark-scope.test.ts",
    "?? benchmarks/next-rust-reference-only-scope.test.ts",
    "?? benchmarks/dx-scope-removal-contract.test.ts",
  ]);
  const byPath = new Map(entries.map((entry) => [entry.path, entry]));

  assert.equal(byPath.get("benchmarks/dx-build-installed-smoke-report.test.ts").lane, 1);
  assert.equal(byPath.get("benchmarks/dx-build-installed-smoke-human-report.test.ts").lane, 1);
  assert.equal(byPath.get("benchmarks/react-starter-benchmark-scope.test.ts").lane, 10);
  assert.equal(byPath.get("benchmarks/next-rust-reference-only-scope.test.ts").lane, 11);
  assert.equal(byPath.get("benchmarks/dx-scope-removal-contract.test.ts").lane, 12);
});

test("Lane 12 classifier resolves remaining live unknowns without claiming shared APIs", () => {
  const entries = classifyStatusLines([
    " M core/src/lib.rs",
    " M dx-www/src/cli/app_router_build_output.rs",
    " M dx-www/src/error.rs",
    " M dx-www/src/lib.rs",
    " M dx-www/src/next_rust.rs",
    " M dx-www/src/project.rs",
    "?? router/src/lib.rs",
    "?? benchmarks/app-router-dynamic-collision-shape.test.ts",
    "?? benchmarks/app-router-segment-file-discovery.test.ts",
    "?? benchmarks/app-router-discovery-summary.test.ts",
    "?? benchmarks/app-router-invalid-param-segments.test.ts",
    "?? benchmarks/app-router-source-owned-vocabulary.test.ts",
    "?? benchmarks/dx-router-request-normalization.test.ts",
    "?? benchmarks/next-rust-source-map-adapter.test.ts",
    "?? benchmarks/next-rust-task-input-adapter.test.ts",
    "?? dx-www/src/cli/preview_command.rs",
    "?? dx-www/src/cli/preview_contract.rs",
    "?? dx-www/src/next_rust_source_map_adapter.rs",
    "?? dx-www/src/next_rust_task_adapter.rs",
  ]);
  const byPath = new Map(entries.map((entry) => [entry.path, entry]));

  for (const sharedPath of ["core/src/lib.rs", "dx-www/src/lib.rs"]) {
    assert.equal(byPath.get(sharedPath).owner, "shared-public-api");
    assert.equal(byPath.get(sharedPath).lane, null);
    assert.equal(byPath.get(sharedPath).lane12Stageable, false);
  }

  assert.equal(byPath.get("dx-www/src/cli/app_router_build_output.rs").lane, 4);
  assert.equal(byPath.get("dx-www/src/error.rs").lane, 8);
  assert.equal(byPath.get("dx-www/src/project.rs").lane, 3);
  assert.equal(byPath.get("router/src/lib.rs").lane, 3);
  assert.equal(byPath.get("benchmarks/app-router-dynamic-collision-shape.test.ts").lane, 3);
  assert.equal(byPath.get("benchmarks/app-router-discovery-summary.test.ts").lane, 3);
  assert.equal(byPath.get("benchmarks/app-router-invalid-param-segments.test.ts").lane, 3);
  assert.equal(byPath.get("benchmarks/app-router-segment-file-discovery.test.ts").lane, 3);
  assert.equal(byPath.get("benchmarks/app-router-source-owned-vocabulary.test.ts").lane, 4);
  assert.equal(byPath.get("benchmarks/dx-router-request-normalization.test.ts").lane, 5);
  assert.equal(byPath.get("dx-www/src/cli/preview_command.rs").lane, 1);
  assert.equal(byPath.get("dx-www/src/cli/preview_contract.rs").lane, 1);

  for (const testPath of [
    "benchmarks/next-rust-source-map-adapter.test.ts",
    "benchmarks/next-rust-task-input-adapter.test.ts",
  ]) {
    assert.equal(byPath.get(testPath).lane, 11);
    assert.equal(byPath.get(testPath).owner, "behavioral-test-evidence");
  }

  for (const referencePath of [
    "dx-www/src/next_rust.rs",
    "dx-www/src/next_rust_source_map_adapter.rs",
    "dx-www/src/next_rust_task_adapter.rs",
  ]) {
    assert.equal(byPath.get(referencePath).owner, "next-rust-reference-provenance");
    assert.equal(byPath.get(referencePath).lane, null);
    assert.equal(byPath.get(referencePath).lane12Stageable, false);
  }

  assert.deepEqual(summarizeOwnership(entries).unclassifiedPaths, []);
});

test("Lane 12 owns deleted removed-scope targets without owning active reference files", () => {
  const entries = classifyStatusLines([
    " D core/src/devtools.rs",
    " D dx-www/src/cli/next_parity_fixtures.rs",
    " D tools/build-graph/turbo-tasks-executor.ts",
    " D tools/build-graph/turbo-tasks-execution-handoff.ts",
    " D tools/build-graph/turbo-tasks-zed-panel.ts",
    " M tools/build-graph/turbo-tasks-adapter.ts",
    " D -",
  ]);
  const byPath = new Map(entries.map((entry) => [entry.path, entry]));

  for (const removedTarget of [
    "core/src/devtools.rs",
    "dx-www/src/cli/next_parity_fixtures.rs",
    "tools/build-graph/turbo-tasks-executor.ts",
    "tools/build-graph/turbo-tasks-execution-handoff.ts",
    "tools/build-graph/turbo-tasks-zed-panel.ts",
  ]) {
    assert.equal(byPath.get(removedTarget).lane, 12);
    assert.equal(byPath.get(removedTarget).owner, "scope-removal-deletion");
    assert.equal(byPath.get(removedTarget).commitPolicy, "lane-12-scope-removal");
    assert.equal(byPath.get(removedTarget).lane12Stageable, true);
  }

  assert.equal(byPath.get("tools/build-graph/turbo-tasks-adapter.ts").owner, "source-build-graph");
  assert.equal(byPath.get("tools/build-graph/turbo-tasks-adapter.ts").lane12Stageable, false);
  assert.equal(byPath.get("-").owner, "workspace-junk-deletion");
  assert.equal(byPath.get("-").lane12Stageable, false);
});

test("Lane 12 classifier assigns live pass two owner gaps", () => {
  const entries = classifyStatusLines([
    " M dx-www/Cargo.toml",
    " M core/src/delivery/tsx_ast.rs",
    " M dx-www/src/cli/forge_launch_copy_review.rs",
    " M dx-www/src/cli/forge_static_asset_materialization.rs",
    " M tools/vendor/next-rust-boundary-check.js",
    "?? benchmarks/app-router-build-output-shared-segments.test.ts",
    "?? benchmarks/app-router-duplicate-param-names.test.ts",
    "?? benchmarks/app-router-invalid-non-path-segments.test.ts",
    "?? benchmarks/app-router-execution-shared-segments.test.ts",
    "?? benchmarks/app-router-route-precedence-vector.test.ts",
    "?? benchmarks/app-router-shape-collision-peers.test.ts",
    "?? benchmarks/app-router-shared-segment-classifier.test.ts",
    "?? benchmarks/app-router-static-segment-decoding.test.ts",
    "?? benchmarks/app-router-terminal-catch-all.test.ts",
    "?? dx-www/src/app_router_segments.rs",
    "?? dx-www/src/cli/app_router_build_command.rs",
    "?? dx-www/src/cli/app_router_runtime_command.rs",
    "?? dx-www/src/cli/app_router_style_assets.rs",
    "?? dx-www/src/cli/build_command.rs",
    "?? dx-www/src/cli/command_output.rs",
    "?? dx-www/src/cli/dev_command.rs",
    "?? dx-www/src/cli/dev_response.rs",
    "?? dx-www/src/cli/forge_doctor.rs",
    "?? dx-www/src/cli/forge_hosted_registry_smoke.rs",
    "?? dx-www/src/cli/forge_launch_page.rs",
    "?? dx-www/src/cli/forge_packages_command.rs",
    "?? dx-www/src/cli/forge_public_evidence.rs",
    "?? dx-www/src/cli/forge_provenance_command.rs",
    "?? dx-www/src/cli/forge_public_status.rs",
    "?? dx-www/src/cli/forge_release_candidate.rs",
    "?? dx-www/src/cli/forge_release_candidate_command.rs",
    "?? dx-www/src/cli/forge_release_dashboard.rs",
    "?? dx-www/src/cli/forge_release_dashboard_command.rs",
    "?? dx-www/src/cli/forge_release_proof.rs",
    "?? dx-www/src/cli/forge_remote_lifecycle.rs",
    "?? dx-www/src/cli/forge_trust_policy_command.rs",
    "?? dx-www/src/cli/forge_trust_regression_command.rs",
    "?? dx-www/src/cli/generate_command.rs",
    "?? dx-www/src/cli/studio_command.rs",
    "?? dx-www/src/cli/templates_command.rs",
    "?? dx-www/src/cli/tests.rs",
    "?? dx-www/src/cli/tests/",
    "?? librust_out.rlib",
  ]);
  const byPath = new Map(entries.map((entry) => [entry.path, entry]));

  assert.equal(byPath.get("dx-www/Cargo.toml").owner, "shared-rust-manifest");
  assert.equal(byPath.get("dx-www/Cargo.toml").lane12Stageable, false);
  assert.equal(byPath.get("core/src/delivery/tsx_ast.rs").owner, "shared-tsx-compiler-core");
  assert.equal(byPath.get("core/src/delivery/tsx_ast.rs").lane12Stageable, false);
  assert.equal(byPath.get("librust_out.rlib").owner, "generated-scratch-artifact");
  assert.equal(byPath.get("librust_out.rlib").lane12Stageable, false);

  assert.equal(byPath.get("dx-www/src/cli/app_router_style_assets.rs").owner, "source-build-graph");
  assert.equal(byPath.get("dx-www/src/cli/app_router_style_assets.rs").lane, 2);

  for (const lane3Path of [
    "benchmarks/app-router-duplicate-param-names.test.ts",
    "benchmarks/app-router-invalid-non-path-segments.test.ts",
    "benchmarks/app-router-route-precedence-vector.test.ts",
    "benchmarks/app-router-shape-collision-peers.test.ts",
    "benchmarks/app-router-shared-segment-classifier.test.ts",
    "benchmarks/app-router-static-segment-decoding.test.ts",
    "benchmarks/app-router-terminal-catch-all.test.ts",
    "dx-www/src/app_router_segments.rs",
  ]) {
    assert.equal(byPath.get(lane3Path).owner, "app-router-filesystem-routing");
    assert.equal(byPath.get(lane3Path).lane, 3);
  }

  for (const lane4Path of [
    "benchmarks/app-router-build-output-shared-segments.test.ts",
    "benchmarks/app-router-execution-shared-segments.test.ts",
    "dx-www/src/cli/app_router_build_command.rs",
    "dx-www/src/cli/app_router_runtime_command.rs",
  ]) {
    assert.equal(byPath.get(lane4Path).owner, "app-router-render-semantics");
    assert.equal(byPath.get(lane4Path).lane, 4);
  }

  for (const lane7Path of ["dx-www/src/cli/dev_response.rs"]) {
    assert.equal(byPath.get(lane7Path).owner, "hot-reload-dev-server");
    assert.equal(byPath.get(lane7Path).lane, 7);
  }

  for (const lane9Path of [
    "dx-www/src/cli/build_command.rs",
    "dx-www/src/cli/command_output.rs",
    "dx-www/src/cli/dev_command.rs",
    "dx-www/src/cli/forge_doctor.rs",
    "dx-www/src/cli/forge_hosted_registry_smoke.rs",
    "dx-www/src/cli/forge_launch_page.rs",
    "dx-www/src/cli/forge_packages_command.rs",
    "dx-www/src/cli/forge_provenance_command.rs",
    "dx-www/src/cli/forge_public_status.rs",
    "dx-www/src/cli/forge_release_candidate.rs",
    "dx-www/src/cli/forge_release_candidate_command.rs",
    "dx-www/src/cli/forge_release_dashboard.rs",
    "dx-www/src/cli/forge_release_dashboard_command.rs",
    "dx-www/src/cli/forge_release_proof.rs",
    "dx-www/src/cli/forge_remote_lifecycle.rs",
    "dx-www/src/cli/forge_trust_policy_command.rs",
    "dx-www/src/cli/forge_trust_regression_command.rs",
    "dx-www/src/cli/generate_command.rs",
    "dx-www/src/cli/templates_command.rs",
    "dx-www/src/cli/tests.rs",
    "dx-www/src/cli/tests/",
  ]) {
    assert.equal(byPath.get(lane9Path).owner, "cli-architecture-split");
    assert.equal(byPath.get(lane9Path).lane, 9);
  }

  for (const lane10Path of [
    "dx-www/src/cli/forge_launch_copy_review.rs",
    "dx-www/src/cli/forge_public_evidence.rs",
    "dx-www/src/cli/forge_static_asset_materialization.rs",
    "dx-www/src/cli/studio_command.rs",
  ]) {
    assert.equal(byPath.get(lane10Path).owner, "template-product-path");
    assert.equal(byPath.get(lane10Path).lane, 10);
  }

  assert.equal(byPath.get("tools/vendor/next-rust-boundary-check.js").owner, "scope-cleanup-docs-truth");
  assert.deepEqual(summarizeOwnership(entries).unclassifiedPaths, []);
});

test("Lane 12 classifier assigns live pass three source-boundary gaps", () => {
  const entries = classifyStatusLines([
    " M core/src/delivery/mod.rs",
    "?? dx-www/src/cli/app_page_route_diagnostics.rs",
    "?? dx-www/src/cli/app_router_paths.rs",
    "?? dx-www/src/cli/deploy_adapter_contract.rs",
    "?? dx-www/src/cli/dev_bridge.rs",
    "?? dx-www/src/cli/forge_release_history.rs",
    "?? dx-www/src/cli/server_action_runtime.rs",
    "?? examples/template/.dx/receipts/graph/",
    "?? examples/template/node_modules/.bin/next",
  ]);
  const byPath = new Map(entries.map((entry) => [entry.path, entry]));

  assert.equal(byPath.get("core/src/delivery/mod.rs").owner, "route-handlers");
  assert.equal(byPath.get("core/src/delivery/mod.rs").lane, 5);
  assert.equal(byPath.get("dx-www/src/cli/app_page_route_diagnostics.rs").owner, "app-router-filesystem-routing");
  assert.equal(byPath.get("dx-www/src/cli/app_page_route_diagnostics.rs").lane, 3);
  assert.equal(byPath.get("dx-www/src/cli/app_router_paths.rs").owner, "app-router-filesystem-routing");
  assert.equal(byPath.get("dx-www/src/cli/app_router_paths.rs").lane, 3);
  assert.equal(byPath.get("dx-www/src/cli/deploy_adapter_contract.rs").owner, "template-product-path");
  assert.equal(byPath.get("dx-www/src/cli/deploy_adapter_contract.rs").lane, 10);
  assert.equal(byPath.get("dx-www/src/cli/dev_bridge.rs").owner, "hot-reload-dev-server");
  assert.equal(byPath.get("dx-www/src/cli/dev_bridge.rs").lane, 7);
  assert.equal(byPath.get("dx-www/src/cli/forge_release_history.rs").owner, "cli-architecture-split");
  assert.equal(byPath.get("dx-www/src/cli/forge_release_history.rs").lane, 9);
  assert.equal(byPath.get("dx-www/src/cli/server_action_runtime.rs").owner, "app-router-render-semantics");
  assert.equal(byPath.get("dx-www/src/cli/server_action_runtime.rs").lane, 4);
  assert.equal(byPath.get("examples/template/.dx/receipts/graph/").owner, "generated-build-artifact");
  assert.equal(byPath.get("examples/template/.dx/receipts/graph/").sourceBoundary, "generated-artifact");
  assert.equal(byPath.get("examples/template/node_modules/.bin/next").owner, "forbidden-node-modules");
  assert.equal(byPath.get("examples/template/node_modules/.bin/next").sourceBoundary, "node-modules-boundary");

  const summary = summarizeOwnership(entries);
  assert.deepEqual(summary.unclassifiedPaths, []);
  assert.deepEqual(summary.generatedArtifactPaths, ["examples/template/.dx/receipts/graph/"]);
  assert.deepEqual(summary.nodeModulesBoundaryPaths, ["examples/template/node_modules/.bin/next"]);
});
