import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const serverContractPath = path.join(
  repoRoot,
  "core",
  "src",
  "delivery",
  "server_contract.rs",
);

test("server contract diagnostics do not expose public v1 wording", () => {
  const source = fs.readFileSync(serverContractPath, "utf8");

  assert.doesNotMatch(source, /object literal in v1/);
  assert.match(source, /stable DX server contract/);
  assert.match(source, /stable DX route contract/);
});
