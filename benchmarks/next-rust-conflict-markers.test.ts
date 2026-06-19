import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const {
  CONFLICT_MARKER_SCHEMA,
  DEFAULT_CONFLICT_MARKER_TARGETS,
  scanConflictMarkers,
  scanFileForConflictMarkers,
} = require("../tools/next-rust-merge/conflict-marker-check.cjs");

test("Lane 14 conflict marker scanner detects unresolved merge blocks", () => {
  const fixtureDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-conflict-markers-"));
  const cleanPath = path.join(fixtureDir, "clean.ts");
  const conflictPath = path.join(fixtureDir, "conflict.ts");

  fs.writeFileSync(cleanPath, "export const clean = true;\n");
  fs.writeFileSync(
    conflictPath,
    [
      "export const value = 'before';",
      "<<<<<<< HEAD",
      "export const value = 'ours';",
      "=======",
      "export const value = 'theirs';",
      ">>>>>>> branch",
      "",
    ].join("\n"),
  );

  const report = scanConflictMarkers({
    cwd: fixtureDir,
    targets: ["clean.ts", "conflict.ts"],
  });

  assert.equal(report.schema, CONFLICT_MARKER_SCHEMA);
  assert.equal(report.status, "failed");
  assert.equal(report.targetCount, 2);
  assert.equal(report.scannedFileCount, 2);
  assert.deepEqual(report.missingTargets, []);
  assert.deepEqual(report.markers, [
    { file: "conflict.ts", line: 2, marker: "<<<<<<< HEAD" },
    { file: "conflict.ts", line: 4, marker: "=======" },
    { file: "conflict.ts", line: 6, marker: ">>>>>>> branch" },
  ]);
});

test("Lane 14 conflict marker scanner ignores markdown heading separators", () => {
  const fixtureDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-conflict-markers-"));
  const markdownPath = path.join(fixtureDir, "README.md");

  fs.writeFileSync(markdownPath, "Heading\n=======\n\nBody\n");

  assert.deepEqual(scanFileForConflictMarkers(markdownPath, fixtureDir), []);
});

test("Lane 14 conflict marker source guard covers merge-sensitive checked surfaces", () => {
  const report = scanConflictMarkers({ cwd: repoRoot });

  assert.equal(report.schema, CONFLICT_MARKER_SCHEMA);
  assert.equal(report.status, "passed");
  assert.equal(report.targetCount, DEFAULT_CONFLICT_MARKER_TARGETS.length);
  assert.deepEqual(report.missingTargets, []);
  assert.deepEqual(report.markers, []);
  assert.ok(
    DEFAULT_CONFLICT_MARKER_TARGETS.includes("tools/next-rust-merge"),
    "coordinator-owned merge tools should be part of the conflict scan",
  );
  assert.ok(
    DEFAULT_CONFLICT_MARKER_TARGETS.includes("dx-www/src/next_rust.rs"),
    "vendor boundary wrapper should be part of the conflict scan",
  );
});
