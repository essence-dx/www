import assert from "node:assert/strict";
import { createRequire } from "node:module";
import test from "node:test";

const require = createRequire(import.meta.url);
const {
  buildCommitPlan,
  buildHandoff,
  buildRiskList,
  classify,
  groupEntries,
} = require("../tools/worktree/www-agent2-ownership-map.cjs");

function entry(path: string, status = " M") {
  const domain = classify(path);
  const kind = status === "??" ? "untracked" : status.includes("D") ? "deleted" : "modified";
  return {
    status,
    path,
    displayPath: path,
    domain,
    kind,
  };
}

test("Agent 2 ownership map routes launch readiness gate source to build artifact lanes", () => {
  const launchGate = classify("tools/launch/launch-readiness-gate.js");

  assert.equal(launchGate.key, "build-smoke");
  assert.equal(launchGate.owner, "build-artifact-lanes");
});

test("Agent 2 ownership map routes launch template materializer to template lanes", () => {
  const materializer = classify("tools/launch/materialize-www-template.ts");

  assert.equal(materializer.key, "template");
  assert.equal(materializer.owner, "template-product-lanes");
});

test("Agent 2 ownership map blocks Windows-reserved root junk paths explicitly", () => {
  const groups = groupEntries([
    entry("NUL", "??"),
    entry("tools/launch/launch-readiness-gate.js"),
  ]);
  const risks = buildRiskList([], groups, []);
  const plan = buildCommitPlan(groups);

  assert.equal(classify("NUL").key, "workspace-junk");
  assert.ok(risks.some((risk: any) => risk.id === "workspace-junk-paths" && risk.severity === "blocking"));
  assert.deepEqual(
    plan.find((item: any) => item.owner === "workspace-junk-quarantine"),
    {
      owner: "workspace-junk-quarantine",
      count: 1,
      groups: ["workspace-junk"],
      disposition: "blocked",
      action: "do not stage Windows-reserved root junk paths",
    },
  );
});

test("Agent 2 handoff turns review risks into owner-routed cleanup candidates", () => {
  const entries = [
    entry("-", " D"),
    entry("dx-www/src/cli/next_parity_fixtures.rs", " D"),
    entry("benchmarks/route-handler-typed-body-alias.test.ts", "??"),
    ...Array.from({ length: 51 }, (_value, index) =>
      entry(`dx-www/src/cli/pass_three_core_${index}.rs`),
    ),
  ];
  const groups = groupEntries(entries);
  const risks = buildRiskList(entries, groups, []);
  const handoff = buildHandoff({
    generatedAt: "2026-05-24T00:00:00.000Z",
    branch: "test",
    shortstat: "",
    dirtyEntryCount: entries.length,
    blockingRiskCount: risks.filter((risk: any) => risk.severity === "blocking").length,
    groups,
    risks,
    commitPlan: buildCommitPlan(groups),
    unmerged: [],
  });

  assert.deepEqual(
    handoff.reviewCandidates.map((item: any) => ({
      riskId: item.riskId,
      owner: item.owner,
      disposition: item.disposition,
    })),
    [
      {
        riskId: "source-deletions",
        owner: "release-control",
        disposition: "review-before-staging",
      },
      {
        riskId: "untracked-legacy-js-tests",
        owner: "test-owner-lanes",
        disposition: "review-before-staging",
      },
      {
        riskId: "broad-dx-www-core",
        owner: "www-core-lanes",
        disposition: "review-before-staging",
      },
    ],
  );
});
