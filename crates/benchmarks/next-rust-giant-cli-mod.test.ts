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
  DEFAULT_GIANT_CLI_MOD_FILE,
  DEFAULT_GIANT_CLI_MOD_LIMITS,
  GIANT_CLI_MOD_SCHEMA,
  runGiantCliModCheck,
} = require("../tools/next-rust-merge/giant-cli-mod-check.cjs");

test("Lane 14 giant CLI guard reports the current risk without editing cli/mod.rs", () => {
  const report = runGiantCliModCheck({ cwd: repoRoot });

  assert.equal(report.schema, GIANT_CLI_MOD_SCHEMA);
  assert.equal(report.lane, 14);
  assert.equal(report.featureImplementation, false);
  assert.equal(report.file, path.join(repoRoot, DEFAULT_GIANT_CLI_MOD_FILE));
  assert.equal(report.limits.maxLineCount, DEFAULT_GIANT_CLI_MOD_LIMITS.maxLineCount);
  assert.equal(report.status, "risk-open");
  assert.ok(
    report.metrics.lineCount > report.limits.maxLineCount,
    "cli/mod.rs should remain flagged until it is split below the guard threshold",
  );
  assert.ok(report.metrics.declarationCount > report.limits.maxDeclarationCount);
  assert.deepEqual(report.sideEffects, []);
});

test("Lane 14 giant CLI guard accepts a small extracted module fixture", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-cli-size-ok-"));
  const fixturePath = path.join(tempDir, "mod.rs");
  fs.writeFileSync(
    fixturePath,
    [
      "mod app_router;",
      "mod diagnostics;",
      "",
      "pub fn run() {",
      "    app_router::start();",
      "}",
      "",
    ].join("\n"),
  );

  const report = runGiantCliModCheck({
    cwd: repoRoot,
    file: fixturePath,
  });

  assert.equal(report.status, "passing");
  assert.deepEqual(report.violations, []);
});

test("Lane 14 giant CLI guard rejects a large fixture by metrics", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-cli-size-large-"));
  const fixturePath = path.join(tempDir, "mod.rs");
  const lines = Array.from({ length: DEFAULT_GIANT_CLI_MOD_LIMITS.maxLineCount + 1 }, (_, index) =>
    index % 3 === 0 ? `fn generated_${index}() {}` : `// generated line ${index}`,
  );
  fs.writeFileSync(fixturePath, `${lines.join("\n")}\n`);

  const report = runGiantCliModCheck({
    cwd: repoRoot,
    file: fixturePath,
  });

  assert.equal(report.status, "risk-open");
  assert.ok(report.violations.some((violation) => violation.id === "line-count"));
});
