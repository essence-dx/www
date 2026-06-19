import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const dxRoot = path.resolve(repoRoot, "..");

function readFrom(root, relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("flow-forge stays a standalone hub-level crate outside the WWW workspace", () => {
  const rootCargo = readFrom(repoRoot, "Cargo.toml");
  const forgeCargo = readFrom(dxRoot, "forge/Cargo.toml");

  assert.doesNotMatch(rootCargo, /integrations\/flow-forge|integrations\\flow-forge/);
  assert.doesNotMatch(
    rootCargo,
    /\[workspace\][\s\S]*exclude\s*=\s*\[[\s\S]*flow-forge/,
  );
  assert.match(forgeCargo, /^name = "forge"$/m);
  assert.doesNotMatch(forgeCargo, /^\[workspace\]/m);
});
