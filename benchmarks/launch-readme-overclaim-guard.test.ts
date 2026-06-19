import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const readmePath = path.join(repoRoot, "README.md");
const launchStatusDocs = [
  "README.md",
  "DX.md",
  "TODO.md",
  "CHANGELOG.md",
];

test("launch public docs keep dx-style starter proof bounded", () => {
  const readme = fs.readFileSync(readmePath, "utf8");

  for (const relativePath of launchStatusDocs) {
    const document = fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
    assert.doesNotMatch(
      document,
      /100\/100 official-starter PostCSS replacement/i,
      `${relativePath} must not market dx-style as a 100/100 PostCSS replacement`,
    );
  }

  assert.match(
    readme,
    /dx-style CSS generation for the official starter/i,
  );
  assert.match(readme, /arbitrary plugin parity still unclaimed/i);
});
