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

test("source resolver keeps unexported package self-reference subpaths at the exports boundary", () => {
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const linker = read("dx-www/src/build/source_engine/module_linker.rs");
  const fixture = read("dx-www/tests/source_resolver_compat.rs");

  assert.match(resolver, /RESOLVER_SOURCE_PACKAGE_EXPORT_BOUNDARY/);
  assert.match(resolver, /matches_package_self_reference_namespace/);
  assert.match(resolver, /package_self_reference_exports_present/);
  assert.match(linker, /package-export-adapter-boundary/);
  assert.match(linker, /RESOLVER_SOURCE_PACKAGE_EXPORT_BOUNDARY/);

  assert.match(
    fixture,
    /source_build_engine_keeps_unexported_package_self_reference_subpaths_at_exports_boundary_without_node_modules/,
  );
  assert.match(fixture, /"exports": \{/);
  assert.match(fixture, /internal\/Secret\.ts/);
  assert.match(fixture, /dx-source-resolver-fixture\/internal\/Secret/);
  assert.match(fixture, /dependency\["kind"\] == "package-export-adapter-boundary"/);
  assert.match(fixture, /dependency\["resolver_source"\] == "package-export-boundary"/);
  assert.match(fixture, /chunk\["source_path"\] != "internal\/Secret\.ts"/);

  assert.doesNotMatch(linker, /project_root\.join\("node_modules"\)/);
});
