import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const readmePath = path.join(repoRoot, "README.md");
const previewManifestPath = path.join(
  repoRoot,
  "examples",
  "www-template",
  "public",
  "preview-.dx/build-cache/manifest.json",
);
const launchStatusDocs = [
  "README.md",
  "DX.md",
  "TODO.md",
  "CHANGELOG.md",
];

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("launch README states current stabilization truth without runtime overclaims", () => {
  const readme = fs.readFileSync(readmePath, "utf8");
  const previewManifest = JSON.parse(fs.readFileSync(previewManifestPath, "utf8"));
  const packageReality = previewManifest.forgePackageReality;

  assert.match(readme, /## Launch Stabilization Truth/);
  assert.equal(packageReality.score, 89);
  assert.equal(packageReality.unboundedSourceScore, 93);
  assert.equal(packageReality.scoreCeilingWithoutLiveProof, 89);
  assert.match(readme, /bounded Forge\/template score is 89\/100/i);
  assert.match(readme, /unbounded source score is 93\/100/i);
  assert.match(readme, /live-proof ceiling is 89\/100/i);
  for (const relativePath of launchStatusDocs) {
    assert.doesNotMatch(
      read(relativePath),
      /source-only ceiling\s+(?:is\s+)?(?:89|93)\/100|(?:89|93)\/100 source-only ceiling/i,
      `${relativePath} must describe the live-proof ceiling separately from source score`,
    );
  }
  assert.match(readme, /20 lock-backed Forge package lanes/i);
  assert.match(readme, /30 catalog source ids/i);
  assert.match(readme, /zero live browser or provider packages are claimed/i);
  assert.match(readme, /score stays below 90/i);
  assert.match(readme, /Default `\/` template is a 3D visual landing scene/i);
  assert.match(readme, /`\/dashboard` carries the package readiness dashboard/i);
  assert.match(readme, /Next\/Turbopack Rust is quarantined vendor reference/i);
  assert.match(readme, /not the public runtime/i);
  assert.match(readme, /no template-local `node_modules`/i);
  assert.match(readme, /## Source Checkout Usage/);
  assert.match(readme, /not documented as a published `dx-www = "1\.0\.0"` crate release/i);
  assert.doesNotMatch(readme, /readiness audit baseline is 40\/100/i);
  assert.doesNotMatch(readme, /dx-style currently blocks full Rust compilation/i);
  assert.doesNotMatch(readme, /full Next\.js parity is (?:complete|proven|supported)/i);
  assert.doesNotMatch(readme, /Turbopack-powered runtime/i);
  assert.doesNotMatch(readme, /\[dependencies\]\s+dx-www = "1\.0\.0"/i);
  assert.doesNotMatch(readme, /compiler\.compile\("\.\/src\/app\.tsx", "\.\/dist"\)/i);
});

test("launch notes record non-catalog package metadata cleanup without runtime proof claims", () => {
  const statusDocs = ["DX.md", "TODO.md", "CHANGELOG.md"];

  for (const relativePath of statusDocs) {
    const source = read(relativePath);
    assert.match(
      source,
      /non-catalog package metadata cleanup/i,
      `${relativePath} should name the non-catalog package metadata cleanup`,
    );
    assert.match(
      source,
      /www\/template/i,
      `${relativePath} should name the removed template-only package id`,
    );
    assert.match(
      source,
      /dx\/icon\/search/i,
      `${relativePath} should name the catalog icon package replacement`,
    );
    assert.doesNotMatch(
      source,
      /non-catalog package metadata cleanup[\s\S]{0,500}(?:browser proof|runtime proof|live provider proof)[\s\S]{0,80}(?:complete|proven|attached|green)/i,
      `${relativePath} must not turn the source cleanup into a runtime proof claim`,
    );
  }
});

test("template-local launch docs keep source-check score separate from production readiness", () => {
  const templateDocs = [
    "examples/template/README.md",
    "examples/template/CHANGELOG.md",
  ];

  for (const relativePath of templateDocs) {
    const source = read(relativePath);
    assert.doesNotMatch(
      source,
      /89\/100\s+(?:launch readiness|source-readiness score)/i,
      `${relativePath} must not present the source-check ceiling as production readiness`,
    );
    assert.match(
      source,
      /89\/100 Forge\/template source-check ceiling/i,
      `${relativePath} should name the bounded source-check ceiling precisely`,
    );
    assert.match(
      source,
      /browser(?: route)? proof|build proof|live provider proof/i,
      `${relativePath} should keep governed production proof visible`,
    );
  }
});
