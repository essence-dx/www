import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("source-owned readiness interpreters normalize request.url before endpoint matching", () => {
  for (const relativePath of [
    "core/src/delivery/route_handler_database_orm.rs",
    "core/src/delivery/route_handler_supabase.rs",
    "core/src/delivery/route_handler_instant_readiness.rs",
  ]) {
    const source = read(relativePath);
    assert.match(
      source,
      /let path = request\.path_for_match\(\)\.trim_end_matches\('\/'\);/,
      `${relativePath} must use the shared normalized matcher path`,
    );
    assert.doesNotMatch(
      source,
      /let path = request\.path\.trim_end_matches\('\/'\);/,
      `${relativePath} must not use raw request.path for endpoint matching`,
    );
  }
});
