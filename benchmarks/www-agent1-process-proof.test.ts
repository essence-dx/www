import assert from "node:assert/strict";
import { createRequire } from "node:module";
import test from "node:test";

const require = createRequire(import.meta.url);
const {
  classifyProcess,
  describeNextAction,
  summarizeProcessProof,
} = require("../tools/worktree/www-agent1-process-proof.cjs");

function processRow(overrides: Record<string, unknown>) {
  return {
    processId: 100,
    parentProcessId: 1,
    name: "cargo.exe",
    ageMinutes: 1,
    cpu: 0.1,
    commandLine: '"G:\\Dev\\Caches\\cargo\\bin\\cargo.exe" check -p dx-www --no-default-features --features cli --bin dx-www -j 1',
    ...overrides,
  };
}

test("Agent 1 proof waits when an active rustc is compiling dx-www", () => {
  const report = summarizeProcessProof({
    generatedAt: "2026-05-24T12:00:00.000Z",
    processes: [
      processRow({
        processId: 10,
        name: "cargo.exe",
        commandLine: '"cargo.exe" build -p dx-www --no-default-features --features cli --bin dx-www -j 1',
      }),
      processRow({
        processId: 11,
        parentProcessId: 10,
        name: "rustc.exe",
        commandLine: '"rustc.exe" --crate-name dx_www dx-www\\src\\lib.rs',
      }),
    ],
    port: { port: 3000, listeners: [] },
    binary: { exists: true, path: "target/debug/dx-www.exe", lastWriteTime: "2026-05-24T09:00:00.000Z" },
    latestSource: { path: "dx-www/src/lib.rs", lastWriteTime: "2026-05-24T12:00:00.000Z" },
  });

  assert.equal(report.recommendedAction, "wait-for-active-rustc");
  assert.equal(report.nextAction.safeToStartBuild, false);
  assert.equal(report.nextAction.command, "node tools\\worktree\\www-agent1-process-proof.cjs --sample-seconds 1");
  assert.equal(report.staleCandidates.length, 0);
  assert.ok(report.warnings.some((warning: any) => warning.id === "active-rustc"));
});

test("Agent 1 proof asks for one fresh build when the binary is stale and no cargo is active", () => {
  const report = summarizeProcessProof({
    generatedAt: "2026-05-24T12:00:00.000Z",
    processes: [],
    port: { port: 3000, listeners: [] },
    binary: { exists: true, path: "target/debug/dx-www.exe", lastWriteTime: "2026-05-24T09:00:00.000Z" },
    latestSource: { path: "dx-www/src/lib.rs", lastWriteTime: "2026-05-24T12:00:00.000Z" },
  });

  assert.equal(report.recommendedAction, "run-one-fresh-cargo-build");
  assert.equal(report.nextAction.safeToStartBuild, true);
  assert.equal(report.nextAction.command, "cargo build -p dx-www --no-default-features --features cli --bin dx-www -j 1");
  assert.ok(report.warnings.some((warning: any) => warning.id === "binary-stale"));
});

test("Agent 1 proof blocks reuse when port 3000 belongs to a non dx-www process", () => {
  const report = summarizeProcessProof({
    generatedAt: "2026-05-24T12:00:00.000Z",
    processes: [
      processRow({
        processId: 20,
        name: "node.exe",
        commandLine: '"node.exe" scripts/dev-server.js',
      }),
    ],
    port: {
      port: 3000,
      listeners: [{ state: "Listen", localAddress: "127.0.0.1", localPort: 3000, owningProcess: 20 }],
    },
    binary: { exists: true, path: "target/debug/dx-www.exe", lastWriteTime: "2026-05-24T12:00:00.000Z" },
    latestSource: { path: "dx-www/src/lib.rs", lastWriteTime: "2026-05-24T11:00:00.000Z" },
  });

  assert.equal(report.recommendedAction, "do-not-reuse-port");
  assert.equal(report.nextAction.requiresHumanReview, true);
  assert.equal(report.nextAction.safeToStartDevServer, false);
  assert.ok(report.warnings.some((warning: any) => warning.id === "port-owned-by-non-dx-www"));
});

test("Agent 1 proof reports stale candidates only after idle sampled cargo exceeds threshold", () => {
  const report = summarizeProcessProof(
    {
      generatedAt: "2026-05-24T12:00:00.000Z",
      processes: [
        processRow({
          processId: 30,
          ageMinutes: 90,
          cpuDeltaSeconds: 0,
          commandLine: '"cargo.exe" check -p dx-www --no-default-features --features cli --bin dx-www -j 1',
        }),
      ],
      port: { port: 3000, listeners: [] },
      binary: { exists: true, path: "target/debug/dx-www.exe", lastWriteTime: "2026-05-24T12:00:00.000Z" },
      latestSource: { path: "dx-www/src/lib.rs", lastWriteTime: "2026-05-24T11:00:00.000Z" },
    },
    { staleAgeMinutes: 45 },
  );

  assert.equal(report.recommendedAction, "review-stale-heavy-processes");
  assert.equal(report.nextAction.requiresHumanReview, true);
  assert.deepEqual(
    report.staleCandidates.map((candidate: any) => candidate.processId),
    [30],
  );
});

test("Agent 1 process classifier recognizes dx-www cargo proof commands", () => {
  assert.equal(
    classifyProcess(
      processRow({
        name: "cargo.exe",
        commandLine: '"cargo.exe" test -p dx-www --no-default-features --features dev-server recovery --lib --jobs 1',
      }),
    ),
    "cargo-test",
  );
  assert.equal(
    classifyProcess(
      processRow({
        name: "cargo-clippy.exe",
        commandLine: '"cargo-clippy.exe" clippy -p dx-www --no-default-features --features cli --bin dx-www -j 1',
      }),
    ),
    "cargo-clippy",
  );
});

test("Agent 1 next-action descriptors expose safe handoff commands", () => {
  assert.deepEqual(
    describeNextAction("start-dev-server-from-fresh-binary", {
      port: { port: 3000 },
    }),
    {
      safeToStartBuild: false,
      safeToStartDevServer: true,
      requiresHumanReview: false,
      command: "target\\debug\\dx-www.exe dev --host 127.0.0.1 --port 3000",
      reason: "binary is fresh and port 3000 is free",
    },
  );
});
