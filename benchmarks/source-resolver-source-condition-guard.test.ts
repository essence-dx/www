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

test("source resolver prefers package source conditions before generated import/default targets", () => {
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const fixture = read("dx-www/tests/source_resolver_source_condition.rs");

  assert.match(
    resolver,
    /for condition in \[\s*"dx",\s*"source",\s*"browser",\s*"development",\s*"import",\s*"default",?\s*\]/,
  );
  assert.match(
    resolver,
    /"dx" \| "source" \| "browser" \| "development" \| "import" \| "default"/,
  );
  assert.match(resolver, /fn source_runtime_condition_targets/);
  assert.match(resolver, /fn source_runtime_condition_values/);
  assert.match(
    resolver,
    /source_runtime_condition_targets\(\s*conditions,\s*source_owned_package_import_targets_with_boundary,\s*\)/,
  );
  assert.match(
    resolver,
    /source_runtime_condition_targets\(\s*conditions,\s*source_owned_package_export_targets_with_boundary,\s*\)/,
  );
  assert.match(
    resolver,
    /if !targets\.targets\.is_empty\(\) \{\s*return targets;\s*\}/,
  );
  assert.match(
    resolver,
    /boundary_fallback\.merge\(targets\)/,
  );

  assert.match(
    fixture,
    /source_build_engine_prefers_package_source_conditions_without_node_modules/,
  );
  assert.match(fixture, /"types": "\.\/node_modules\/types-only\/feature\.d\.ts"/);
  assert.match(fixture, /"node": "\.\/node_modules\/node-only\/feature\.ts"/);
  assert.match(fixture, /"production": "\.\/node_modules\/node-only\/production-feature\.ts"/);
  assert.match(fixture, /"source": "\.\/src\/feature\.ts"/);
  assert.match(fixture, /"import": "\.\/dist\/feature\.js"/);
  assert.match(fixture, /"types": "\.\/node_modules\/types-only\/public\.d\.ts"/);
  assert.match(fixture, /"node": "\.\/node_modules\/node-only\/public\.ts"/);
  assert.match(fixture, /"production": "\.\/node_modules\/node-only\/production-public\.ts"/);
  assert.match(fixture, /"source": "\.\/src\/public\.ts"/);
  assert.match(fixture, /"node": "\.\/node_modules\/node-only\/widget\.tsx"/);
  assert.match(fixture, /"production": "\.\/node_modules\/node-only\/production-widget\.tsx"/);
  assert.match(fixture, /"source": "\.\/src\/widget\.tsx"/);
  assert.match(fixture, /dependency\["specifier"\] == "#feature"/);
  assert.match(fixture, /dependency\["resolved_path"\] == "src\/feature\.ts"/);
  assert.match(fixture, /dependency\["kind"\] != "package-import-adapter-boundary"/);
  assert.match(
    fixture,
    /dependency\["specifier"\] == "dx-source-condition-fixture"/,
  );
  assert.match(fixture, /dependency\["resolved_path"\] == "src\/public\.ts"/);
  assert.match(fixture, /dependency\["kind"\] != "package-export-adapter-boundary"/);
  assert.match(
    fixture,
    /dependency\["specifier"\] == "dx-source-condition-fixture\/widget"/,
  );
  assert.match(fixture, /dependency\["resolved_path"\] == "src\/widget\.tsx"/);
  assert.match(fixture, /generated\/type target should not be linked/);
  assert.match(fixture, /assert!\(!report\.manifest\.node_modules_required\)/);
});
