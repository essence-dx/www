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

function cliNextRustWrapperSource() {
  const cli = read("dx-www/src/cli/mod.rs");
  const match = cli.match(
    /fn cmd_www_next_rust\(&self, args: &\[String\]\) -> DxResult<\(\)> \{\s*next_rust_status::cmd_www_next_rust\(args\)\s*\}/,
  );
  assert.ok(match, "missing next-rust CLI wrapper delegation");
  return match[0];
}

test("next-rust metadata is reference-only and cannot target runtime or build adoption", () => {
  const nextRust = read("dx-www/src/next_rust.rs");
  const cliWrapper = cliNextRustWrapperSource();
  const cliStatusModule = read("dx-www/src/cli/next_rust_status.rs");

  const capabilityEntries = nextRust.match(/\n    DxNextRustCapability \{/g) ?? [];
  assert.equal(capabilityEntries.length, 24);
  assert.equal((nextRust.match(/reference_only: true/g) ?? []).length, 24);

  assert.match(nextRust, /pub const DX_NEXT_RUST_RUNTIME_BUILD_ADOPTION: bool = false;/);
  assert.match(nextRust, /pub const DX_NEXT_RUST_PUBLIC_ARCHITECTURE: bool = false;/);
  assert.doesNotMatch(nextRust, /\bcore_candidate\b|core candidate/);
  assert.doesNotMatch(
    nextRust,
    /(?:build cache|task graph|module and asset graph|hot reload protocol) candidate/i,
  );
  assert.doesNotMatch(nextRust, /\b(?:build graph|task graph|turbo task graph) experiments\b/i);

  assert.match(cliWrapper, /next_rust_status::cmd_www_next_rust\(args\)/);
  assert.doesNotMatch(cliWrapper, /\bcore_candidate\b|core candidate/);
  assert.doesNotMatch(cliWrapper, /Imported build capabilities/);
  assert.match(cliStatusModule, /reference_only/);
  assert.match(cliStatusModule, /Imported reference capabilities/);
  assert.doesNotMatch(cliStatusModule, /\bcore_candidate\b|core candidate/);
  assert.doesNotMatch(cliStatusModule, /Imported build capabilities/);
});
