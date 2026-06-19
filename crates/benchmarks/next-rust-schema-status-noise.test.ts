import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const {
  DEFAULT_SCHEMA_STATUS_NOISE_FILES,
  SCHEMA_STATUS_NOISE_SCHEMA,
  runSchemaStatusNoiseCheck,
} = require("../tools/next-rust-merge/schema-status-noise-check.cjs");

test("Lane 14 schema/status noise check passes on coordinator source contracts", () => {
  const report = runSchemaStatusNoiseCheck({ cwd: repoRoot });

  assert.equal(report.schema, SCHEMA_STATUS_NOISE_SCHEMA);
  assert.equal(report.status, "passing");
  assert.equal(report.scannedFiles.length, DEFAULT_SCHEMA_STATUS_NOISE_FILES.length);
  assert.ok(
    DEFAULT_SCHEMA_STATUS_NOISE_FILES.includes(
      "tools/next-rust-merge/giant-cli-mod-check.cjs",
    ),
    "schema/status noise scan should cover the giant CLI guard source",
  );
  assert.ok(
    DEFAULT_SCHEMA_STATUS_NOISE_FILES.includes(
      "dx-www/src/cli/app_route_handler_receipt.rs",
    ),
    "schema/status noise scan should cover route-handler receipt public schemas",
  );
  assert.ok(
    DEFAULT_SCHEMA_STATUS_NOISE_FILES.includes("core/src/delivery/server_contract.rs"),
    "schema/status noise scan should cover compiler-delivery route-handler headers",
  );
  assert.ok(
    DEFAULT_SCHEMA_STATUS_NOISE_FILES.includes("docs/NEXTJS_COMPATIBILITY_MAP.md"),
    "schema/status noise scan should cover public compatibility docs",
  );
  assert.ok(
    DEFAULT_SCHEMA_STATUS_NOISE_FILES.includes(
      "integrations/n8n-nodes-base/dx-node-source-manifest.json",
    ),
    "schema/status noise scan should cover the copied n8n source manifest",
  );
  assert.deepEqual(report.violations, []);
  assert.deepEqual(report.sideEffects, []);
  const n8nManifest = JSON.parse(
    fs.readFileSync(
      path.join(repoRoot, "integrations", "n8n-nodes-base", "dx-node-source-manifest.json"),
      "utf8",
    ),
  );
  assert.equal(n8nManifest.schema, "dx.integrations.n8n_nodes_base_reference");
  assert.equal(n8nManifest.format, 1);
  assert.doesNotMatch(n8nManifest.schema, /\.v1$/);
  assert.ok(
    report.scannedFiles.every((file) => path.isAbsolute(file)),
    "report should name the exact files it scanned",
  );
});

test("Lane 14 schema/status noise check rejects public schema suffixes and overclaims", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-schema-noise-"));
  const fixturePath = path.join(tempDir, "bad-contract.cjs");
  fs.writeFileSync(
    fixturePath,
    [
      'const schema = "dx.nextRustMerge.fake.v1";',
      'const status = "full Next.js parity is complete";',
      "module.exports = { schema, status };",
      "",
    ].join("\n"),
  );

  const report = runSchemaStatusNoiseCheck({
    cwd: repoRoot,
    files: [fixturePath],
  });

  assert.equal(report.status, "failed");
  assert.deepEqual(
    report.violations.map((violation) => violation.id).sort(),
    ["full-next-parity-overclaim", "public-schema-version-suffix"],
  );
});

test("Lane 14 schema/status noise CLI emits read-only JSON", () => {
  const result = spawnSync(
    process.execPath,
    [
      path.join(
        repoRoot,
        "tools",
        "next-rust-merge",
        "schema-status-noise-check.cjs",
      ),
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
    },
  );

  assert.equal(result.status, 0, result.stderr || result.stdout);
  const report = JSON.parse(result.stdout);

  assert.equal(report.schema, SCHEMA_STATUS_NOISE_SCHEMA);
  assert.equal(report.status, "passing");
  assert.deepEqual(report.sideEffects, []);
});
