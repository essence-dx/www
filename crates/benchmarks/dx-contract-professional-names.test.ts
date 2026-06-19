const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const contractSuffix = "." + "v1";
const escapedContractSuffix = "\\." + "v1";

const publicContractFiles = [
  "examples/template/package-catalog.ts",
  "examples/template/framework-completeness.ts",
  "examples/template/forge-package-status-read-model.ts",
  "examples/template/.dx/forge/package-status.json",
  "examples/template/dx-studio-edit-contract.ts",
  "tools/launch/materialize-www-template.ts",
  "dx-www/src/cli/studio_manifest.rs",
  "dx-www/src/cli/mod.rs",
  "core/src/ecosystem/forge_registry.rs",
  "core/src/ecosystem/project_check.rs",
  "docs/DX_WWW_FRAMEWORK_STRUCTURE.md",
  "README.md",
];

const textExtensions = new Set([
  ".rs",
  ".ts",
  ".tsx",
  ".js",
  ".ts",
  ".mjs",
  ".json",
  ".md",
  ".toml",
  ".html",
  ".css",
  ".yml",
  ".yaml",
]);

const skippedDirectories = new Set([
  ".git",
  ".dx",
  ".codex-tmp",
  "benchmarks",
  "docs",
  "target",
  "node_modules",
  "integrations",
  "worker-lanes",
  "www",
]);

const skippedFiles = new Set([
  "core/src/ecosystem/dx_check_receipt.rs",
  "core/src/ecosystem/dx_check_receipt/panel.rs",
  "core/src/ecosystem/dx_check_receipt/panel_parts/tests_a.rs",
  "core/src/ecosystem/dx_check_receipt/panel_parts/tests_b.rs",
  "core/src/ecosystem/forge_package_status_machine.rs",
  "core/src/ecosystem/json_receipt_machine.rs",
  "dx-www/src/cli/dx_check_latest_receipt.rs",
  "dx-www/src/cli/readiness.rs",
  "related-crates/media-icon/src/engine.rs",
  "related-crates/media-icon/src/index.rs",
  "related-crates/media-icon/src/machine_catalog.rs",
  "related-crates/media-icon/src/machine_manifest.rs",
  "related-crates/media-icon/src/machine_pack_body.rs",
  "related-crates/media-icon/src/machine_precomputed.rs",
  "related-crates/media-icon/src/machine_readiness.rs",
]);

function readText(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

function walkTextFiles(directory, output = []) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    if (entry.isDirectory()) {
      if (!skippedDirectories.has(entry.name)) {
        walkTextFiles(path.join(directory, entry.name), output);
      }
      continue;
    }

    const absolutePath = path.join(directory, entry.name);
    if (textExtensions.has(path.extname(entry.name)) || entry.name === "dx") {
      output.push(absolutePath);
    }
  }

  return output;
}

function collectLaunchPackageIds() {
  const cli = readText("dx-www/src/cli/mod_parts/next_familiar_template.rs");
  const match = cli.match(/const FORGE_WWW_TEMPLATE_PACKAGE_IDS:[\s\S]*?=\s*\[([\s\S]*?)\];/);
  assert.ok(match, "FORGE_WWW_TEMPLATE_PACKAGE_IDS should remain declared");

  return [...match[1].matchAll(/"([^"]+)"/g)].map((entry) => entry[1]);
}

test("Forge launch package inventory stays explicit and professional", () => {
  const packageIds = collectLaunchPackageIds();

  assert.equal(packageIds.length, 32);
  assert.ok(packageIds.includes("auth/better-auth"));
  assert.ok(packageIds.includes("dx/icon/search"));
  assert.ok(packageIds.includes("shadcn/ui/alert"));
  assert.ok(packageIds.includes("shadcn/ui/avatar"));
  assert.ok(packageIds.includes("shadcn/ui/skeleton"));
  assert.ok(!packageIds.includes("lucide/react"));
  assert.ok(!packageIds.includes("auth/google"));
});

test("public DX contract names do not carry generated version suffixes", () => {
  const offenders = [];

  for (const relativePath of publicContractFiles) {
    const text = readText(relativePath);
    const lines = text.split(/\r?\n/);

    lines.forEach((line, index) => {
      const isDxContractLine = line.includes("dx.") || line.includes("dx\\.");
      if (
        isDxContractLine &&
        (line.includes(contractSuffix) || line.includes(escapedContractSuffix))
      ) {
        offenders.push(`${relativePath}:${index + 1}: ${line.trim()}`);
      }
    });
  }

  assert.deepEqual(offenders, []);
});

test("DX and Forge source surfaces have no literal generated version suffix", () => {
  const offenders = [];

  for (const absolutePath of walkTextFiles(repoRoot)) {
    const relativePath = path.relative(repoRoot, absolutePath).replaceAll(path.sep, "/");
    if (skippedFiles.has(relativePath)) {
      continue;
    }
    const lines = fs.readFileSync(absolutePath, "utf8").split(/\r?\n/);

    lines.forEach((line, index) => {
      const publicSchemaVersion =
        line.match(/"dx\.[^"]+\.v1"/) || line.match(/"dx\\\.[^"]+\\\.v1"/);
      if (publicSchemaVersion || line.includes(escapedContractSuffix)) {
        offenders.push(`${relativePath}:${index + 1}: ${line.trim()}`);
      }
    });
  }

  assert.deepEqual(offenders, []);
});

test("renamed contract identifiers are still present on launch surfaces", () => {
  const catalog = readText("examples/template/package-catalog.ts");
  const studio = readText("dx-www/src/cli/studio_manifest.rs");
  const materializer = readText("tools/launch/materialize-www-template.ts");

  assert.match(catalog, /dx\.forge\.package\.dx_check_visibility/);
  assert.match(catalog, /dx\.forge\.package\.dx_style_compatibility/);
  assert.match(studio, /dx\.studio\.preview_manifest/);
  assert.match(studio, /dx\.www\.check_panel_view_model/);
  assert.match(materializer, /dx\.forge\.package\.source_guard_runbook_fixture/);
  assert.doesNotMatch(
    catalog,
    new RegExp("dx\\.forge\\.package\\.dx_check_visibility\\" + contractSuffix),
  );
  assert.doesNotMatch(
    studio,
    new RegExp("dx\\.studio\\.preview_manifest\\" + contractSuffix),
  );
});
