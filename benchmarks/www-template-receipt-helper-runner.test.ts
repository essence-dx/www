import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const templateRoot = path.join(root, "examples", "template");

function read(relativePath: string) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("launch receipt hash helpers run under the template type-module boundary", () => {
  const packageJson = JSON.parse(read("examples/template/package.json"));
  const helpers = fs
    .readdirSync(templateRoot)
    .filter((fileName) => fileName.endsWith("-receipt-hashes.ts"))
    .sort();

  assert.equal(packageJson.type, "module");
  assert.ok(helpers.length >= 20, "expected package receipt helper coverage");

  for (const helper of helpers) {
    const relativePath = `examples/template/${helper}`;
    const source = read(relativePath);
    const output = spawnSync(process.execPath, [relativePath, "--help"], {
      cwd: root,
      encoding: "utf8",
    });
    const combinedOutput = `${output.stdout}\n${output.stderr}`;

    if (source.includes('require("node:')) {
      assert.match(source, /createRequire\(import\.meta\.url\)/, relativePath);
      assert.match(source, /fileURLToPath\(import\.meta\.url\)/, relativePath);
      assert.doesNotMatch(source, /require\.main === module/, relativePath);
    }

    assert.equal(output.status, 0, combinedOutput);
    assert.match(combinedOutput, /Usage: node examples\/template\/.*-receipt-hashes\.ts/);
    assert.doesNotMatch(
      combinedOutput,
      /ReferenceError: require is not defined|ReferenceError: module is not defined|ReferenceError: __dirname is not defined/,
      relativePath,
    );
  }
});

test("package docs publish receipt helper commands through the DX runner", () => {
  const docsDir = path.join(root, "docs", "packages");
  const packageDocs = fs
    .readdirSync(docsDir)
    .filter((fileName) => fileName.endsWith(".md"))
    .sort();

  for (const packageDoc of packageDocs) {
    const relativePath = `docs/packages/${packageDoc}`;
    const source = read(relativePath);

    if (!source.includes("-receipt-hashes.ts")) {
      continue;
    }

    assert.doesNotMatch(
      source,
      /node examples[\\/]+www-template[\\/]+[^`\s]+-receipt-hashes\.ts --/i,
      `${relativePath} should route receipt helper commands through tools/launch/run-template-receipt-helper.js`,
    );
  }
});
