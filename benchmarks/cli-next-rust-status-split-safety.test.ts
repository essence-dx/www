import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

function maybeRead(relativePath) {
  const absolute = path.join(repoRoot, relativePath);
  return fs.existsSync(absolute) ? fs.readFileSync(absolute, "utf8") : "";
}

function cliNextRustCommandSource() {
  const cli = read("dx-www/src/cli/mod.rs");
  const start = cli.indexOf("fn cmd_www_next_rust");
  const end = cli.indexOf("fn print_version", start);
  assert.notEqual(start, -1, "missing cmd_www_next_rust delegator");
  assert.notEqual(end, -1, "missing command boundary after cmd_www_next_rust");
  return cli.slice(start, end);
}

test("next-rust status command is split out of the giant CLI module", () => {
  const cli = read("dx-www/src/cli/mod.rs");
  const command = cliNextRustCommandSource();
  const module = maybeRead("dx-www/src/cli/next_rust_status.rs");

  assert.match(cli, /^mod next_rust_status;/m);
  assert.match(command, /next_rust_status::cmd_www_next_rust\(args\)/);
  assert.doesNotMatch(command, /serde_json::json!|snapshot\.capabilities|reference_only/);

  assert.match(module, /pub\(super\) fn cmd_www_next_rust\(args: &\[String\]\) -> DxResult<\(\)>/);
  assert.match(module, /reference_only/);
  assert.match(module, /Imported reference capabilities/);
  assert.doesNotMatch(module, /\bcore_candidate\b|core candidate|Imported build capabilities/);
});
