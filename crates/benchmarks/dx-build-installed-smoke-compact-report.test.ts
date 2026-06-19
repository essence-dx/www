const assert = require("node:assert/strict");
const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.join(__dirname, "..");
const smokePath = path.join(repoRoot, "tools", "build", "dx-build-installed-smoke.ts");

test("installed smoke reports compact command diagnostics with truncation metadata", () => {
  const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-compact-report-"));
  const fakeBinary = path.join(tempRoot, "fake-dx-www-noisy.ts");
  const receiptPath = path.join(tempRoot, "compact-report.json");
  fs.writeFileSync(
    fakeBinary,
    `const args = process.argv.slice(2);
if (args[0] === "www" && args[1] === "build" && args[2] === "--help") {
  console.error("dx www build: Run the DX source-owned build engine");
  console.error("Uses the source-owned build engine and does not install node_modules.");
  process.exit(0);
}
if (args[0] === "build") {
  process.stdout.write("stdout-start:" + "A".repeat(1800) + ":stdout-end");
  process.stderr.write("stderr-start:" + "B".repeat(1900) + ":stderr-end");
  process.exit(42);
}
process.exit(2);
`,
  );

  const jsonResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      fakeBinary,
      "--runner",
      process.execPath,
      "--json",
      "--receipt",
      receiptPath,
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
    },
  );

  assert.equal(jsonResult.status, 1, jsonResult.stdout + jsonResult.stderr);
  const report = JSON.parse(jsonResult.stdout);
  assert.equal(report.build.command.exitCode, 42);
  assert.equal(report.build.command.stdoutTruncated, true);
  assert.equal(report.build.command.stderrTruncated, true);
  assert.equal(report.build.command.stdoutLength, 1824);
  assert.equal(report.build.command.stderrLength, 1924);
  assert.ok(report.build.command.stdoutTail.length <= 1200);
  assert.ok(report.build.command.stderrTail.length <= 1200);
  assert.match(report.build.command.stdoutTail, /:stdout-end$/);
  assert.match(report.build.command.stderrTail, /:stderr-end$/);
  assert.equal(report.binarySourceFreshness.trackedSourcePaths, undefined);
  assert.equal(report.binarySourceFreshness.trackedSourcePathsTruncated, true);
  assert.ok(report.binarySourceFreshness.trackedSourcePathSample.length <= 24);
  const receiptText = fs.readFileSync(receiptPath, "utf8");
  assert.deepEqual(JSON.parse(receiptText), report);
  assert.equal(report.receiptWrite.attempted, true);
  assert.equal(report.receiptWrite.written, true);
  assert.equal(report.receiptWrite.jsonParseable, true);
  assert.equal(report.receiptWrite.matchesReport, true);
  assert.equal(report.receiptWrite.byteLength, Buffer.byteLength(receiptText));
  assert.ok(report.receiptWrite.byteLength < 40_000);

  const humanResult = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      fakeBinary,
      "--runner",
      process.execPath,
      "--receipt",
      path.join(tempRoot, "compact-report-human.json"),
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
    },
  );

  assert.equal(humanResult.status, 1, humanResult.stdout + humanResult.stderr);
  assert.match(
    humanResult.stdout,
    /Build stdout tail \(truncated to last 1200 of 1824 chars\):/,
  );
  assert.match(
    humanResult.stdout,
    /Build stderr tail \(truncated to last 1200 of 1924 chars\):/,
  );
});

test("installed smoke reports receipt write failures as parseable JSON", () => {
  const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-build-receipt-write-failure-"));
  const fakeBinary = path.join(tempRoot, "fake-dx-www.ts");
  const blockedReceiptParent = path.join(tempRoot, "blocked-receipt-parent");
  const receiptPath = path.join(blockedReceiptParent, "receipt.json");
  fs.writeFileSync(blockedReceiptParent, "not a directory");
  fs.writeFileSync(
    fakeBinary,
    `const args = process.argv.slice(2);
if (args[0] === "www" && args[1] === "build" && args[2] === "--help") {
  console.error("dx www build: Run the DX source-owned build engine");
  console.error("Uses the source-owned build engine and does not install node_modules.");
  process.exit(0);
}
if (args[0] === "build") {
  process.exit(42);
}
process.exit(2);
`,
  );
  const freshBinaryDate = new Date("2100-01-01T00:00:00.000Z");
  fs.utimesSync(fakeBinary, freshBinaryDate, freshBinaryDate);

  const result = spawnSync(
    process.execPath,
    [
      smokePath,
      "--binary",
      fakeBinary,
      "--runner",
      process.execPath,
      "--json",
      "--receipt",
      receiptPath,
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
    },
  );

  assert.equal(result.status, 1, result.stdout + result.stderr);
  assert.equal(result.stderr, "");
  const report = JSON.parse(result.stdout);
  assert.equal(report.receiptWrite.attempted, true);
  assert.equal(report.receiptWrite.written, false);
  assert.equal(report.receiptWrite.jsonParseable, false);
  assert.equal(report.receiptWrite.matchesReport, false);
  assert.equal(report.receiptWrite.byteLength, null);
  assert.equal(report.receiptWrite.path, receiptPath);
  assert.match(report.receiptWrite.error.message, /not a directory|EEXIST|ENOTDIR/i);
  assert.ok(
    report.failures.some((failure) =>
      failure.startsWith("installed smoke receipt could not be written:"),
    ),
  );
});
