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

test("source resolver supports local config extends without node_modules lookup", () => {
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const fixture = read("dx-www/tests/source_resolver_config_extends.rs");

  assert.match(resolver, /fn read_json_config_chain\(/);
  assert.match(resolver, /fn jsonc_to_json\(/);
  assert.match(resolver, /fn strip_jsonc_comments\(/);
  assert.match(resolver, /fn strip_jsonc_trailing_commas\(/);
  assert.match(resolver, /MAX_CONFIG_EXTENDS_DEPTH/);
  assert.match(resolver, /fn source_owned_extended_config_path\(/);
  assert.match(resolver, /Some\(Value::Array\(extended_configs\)\)/);
  assert.match(resolver, /extends entries must be strings/);
  assert.match(resolver, /matched_alias_patterns/);
  assert.match(resolver, /matched_alias_patterns\.insert\(alias\.pattern\.clone\(\)\)/);
  assert.match(resolver, /segment == "node_modules"/);
  assert.match(resolver, /!extended\.starts_with\("\.\/"\)/);
  assert.match(resolver, /base_url_resolver_source/);
  assert.match(resolver, /configs\.into_iter\(\)\.rev\(\)/);

  assert.match(fixture, /"extends": "\.\/config\/jsconfig\.base\.json"/);
  assert.match(fixture, /\/\/ JSONC comments are valid in source-owned TS configs/);
  assert.match(fixture, /"@commented\/\*": \["shared\/commented\/\*"\],/);
  assert.match(fixture, /"extends": \[\s*"\.\/config\/jsconfig\.base-a\.json"/);
  assert.match(fixture, /"\.\/config\/jsconfig\.base-b\.json"/);
  assert.match(fixture, /"next\/tsconfig"/);
  assert.match(fixture, /"baseUrl": "\.\."/);
  assert.match(fixture, /"@array-shared\/\*": \["shared\/array-a\/\*"\]/);
  assert.match(fixture, /"@array-shared\/\*": \["shared\/array-b\/\*"\]/);
  assert.match(fixture, /"@shared\/\*": \["shared\/\*"\]/);
  assert.match(fixture, /"~\/\*": \["src\/\*"\]/);
  assert.match(fixture, /"@override\/\*": \["shared\/overrides\/\*"\]/);
  assert.match(fixture, /"@override\/\*": \["shared\/base\/\*"\]/);
  assert.match(fixture, /"@missing-override\/\*": \["missing\/overrides\/\*"\]/);
  assert.match(fixture, /dependency\["specifier"\] == "@missing-override\/Panel"/);
  assert.match(fixture, /dependency\["kind"\] == "unresolved-source-alias"/);
  assert.match(fixture, /"source_path"\] == "shared\/overrides\/Panel\.tsx"/);
  assert.match(fixture, /"source_path"\] == "shared\/array-b\/Panel\.tsx"/);
  assert.match(fixture, /dependency\["specifier"\] != "@array-shared\/Panel"/);
  assert.match(fixture, /dependency\["resolved_path"\] != "shared\/array-a\/Panel\.tsx"/);
  assert.match(fixture, /"source_path"\] != "shared\/base\/Panel\.tsx"/);
  assert.match(fixture, /"node_modules\/shared\/trap"/);
  assert.match(fixture, /"resolver_source"\] == "jsconfig-path"/);
  assert.match(fixture, /"resolver_source"\] == "base-url-node-modules-boundary"/);
  assert.match(fixture, /assert!\(!report\.manifest\.node_modules_required\)/);
});
