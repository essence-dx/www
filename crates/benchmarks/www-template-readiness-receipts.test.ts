import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

const requiredTemplateReadinessReceipts = [
  "ai-sdk.json",
  "authentication.json",
  "automation-connectors.json",
  "database-api.json",
  "launch-companion-doc-receipts.json",
  "launch-readiness-bundle.json",
  "launch-route.json",
  "launch-runtime-approval-request.json",
  "launch-runtime-checklist.json",
  "launch-runtime-evidence.json",
  "launch-scene-readiness.json",
  "launch-verification-lane.json",
  "payments.json",
  "zed-template-handoff.json",
] as const;

const referenceRoots = [
  "examples/template",
  "docs/packages",
  "dx-www/src/cli",
  "tools/launch",
  ".dx/template-app-browser-preview",
] as const;

const generatedLaunchReportPaths = [
  ".dx/forge/reports/launch-adoption-report.json",
  ".dx/forge/reports/launch-manifest-drift.json",
] as const;

const templateReadinessPrefix = ".dx/forge/template-readiness/";

const generatedLaunchReportFileNames = [
  "launch-adoption-report.json",
  "launch-manifest-drift.json",
] as const;

function read(relativePath: string) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath: string) {
  return JSON.parse(read(relativePath));
}

function escapeRegExp(value: string) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function listFiles(directory: string): string[] {
  const resolved = path.join(root, directory);
  const files: string[] = [];

  if (!fs.existsSync(resolved)) {
    return files;
  }

  function visit(current: string) {
    for (const entry of fs.readdirSync(current, { withFileTypes: true })) {
      const entryPath = path.join(current, entry.name);
      if (entry.isDirectory()) {
        visit(entryPath);
      } else if (/\.(?:html|json|md|ts|tsx)$/.test(entry.name)) {
        files.push(path.relative(root, entryPath).replaceAll("\\", "/"));
      }
    }
  }

  visit(resolved);
  return files;
}

function referencedTemplateReadinessReceipts() {
  const refs = new Set<string>();
  for (const sourcePath of referenceRoots.flatMap((directory) => listFiles(directory))) {
    const source = read(sourcePath);
    for (const match of source.matchAll(
      /\.dx\/forge\/template-readiness\/[A-Za-z0-9._-]+\.json/g,
    )) {
      refs.add(match[0]);
    }
  }
  return [...refs].sort();
}

function assertHonestReceipt(receipt: Record<string, unknown>, receiptPath: string) {
  assert.notEqual(receipt.runtime_proof, true, `${receiptPath} must not claim runtime proof`);
  assert.notEqual(
    receipt.live_provider_execution,
    true,
    `${receiptPath} must not claim live provider execution`,
  );
  assert.notEqual(receipt.network_calls, true, `${receiptPath} must not claim network calls`);
  assert.notEqual(receipt.fake_proof, true, `${receiptPath} must not carry fake proof`);
  if (Array.isArray(receipt.secret_values)) {
    assert.deepEqual(receipt.secret_values, [], `${receiptPath} must not expose secrets`);
  }
}

test("template-readiness receipts referenced by launch surfaces exist and stay no-execution", () => {
  const sourceRoot = "examples/template/.dx/forge/template-readiness";
  const previewRoot = ".dx/template-app-browser-preview/.dx/forge/template-readiness";
  const expectedRefs = requiredTemplateReadinessReceipts.map(
    (name) => `.dx/forge/template-readiness/${name}`,
  );

  assert.deepEqual(referencedTemplateReadinessReceipts(), expectedRefs);

  for (const name of requiredTemplateReadinessReceipts) {
    for (const directory of [sourceRoot, previewRoot]) {
      const receiptPath = `${directory}/${name}`;
      assert.ok(fs.existsSync(path.join(root, receiptPath)), `${receiptPath} should exist`);
      const receipt = readJson(receiptPath);
      assert.equal(typeof receipt.schema, "string", `${receiptPath} should name a schema`);
      assertHonestReceipt(receipt, receiptPath);
    }
  }
});

test("preview manifest lists every copied template-readiness receipt without lifting runtime proof", () => {
  const manifest = readJson("examples/template/public/preview-.dx/build-cache/manifest.json");
  const previewManifest = readJson(
    ".dx/template-app-browser-preview/public/preview-.dx/build-cache/manifest.json",
  );
  const expectedRefs = requiredTemplateReadinessReceipts.map(
    (name) => `.dx/forge/template-readiness/${name}`,
  );

  for (const preview of [manifest, previewManifest]) {
    const reality = preview.forgePackageReality;
    assert.deepEqual(
      reality.templateReadinessReceipts.map(
        (receipt: { readinessReceipt: string }) => receipt.readinessReceipt,
      ),
      expectedRefs,
    );
    assert.ok(
      reality.templateReadinessReceipts.every(
        (receipt: { runtimeProof: boolean; secretValues: readonly string[] }) =>
          receipt.runtimeProof === false && receipt.secretValues.length === 0,
      ),
      "template-readiness receipts must not lift the browser/provider score gate",
    );
    assert.equal(reality.scoreGate.browserRuntimeProof, false);
    assert.equal(reality.scoreGate.liveProviderProof, false);
  }
});

test("generated launch reports are not advertised as template-readiness receipts", () => {
  const cliSource = read("dx-www/src/cli/mod.rs");

  for (const reportPath of generatedLaunchReportPaths) {
    assert.match(
      cliSource,
      new RegExp(escapeRegExp(reportPath)),
      `${reportPath} should be a generated report contract`,
    );
  }

  for (const fileName of generatedLaunchReportFileNames) {
    const receiptPath = `${templateReadinessPrefix}${fileName}`;
    assert.doesNotMatch(
      cliSource,
      new RegExp(escapeRegExp(receiptPath)),
      `${receiptPath} would make generated reports look like source readiness receipts`,
    );
  }
});
