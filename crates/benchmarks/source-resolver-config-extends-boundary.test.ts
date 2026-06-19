import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function read(relativePath: string) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function captureFunction(source: string, signature: string) {
  const startIndex = source.indexOf(signature);
  assert.notEqual(startIndex, -1, `missing ${signature}`);
  const nextFunctionIndex = source.indexOf("\nfn ", startIndex + signature.length);
  assert.notEqual(nextFunctionIndex, -1, `missing next function after ${signature}`);
  return source.slice(startIndex, nextFunctionIndex);
}

test("config extends keeps external files at the source resolver boundary", () => {
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const fixture = read("dx-www/tests/source_resolver_config_extends.rs");
  const extendedConfigPath = captureFunction(
    resolver,
    "fn source_owned_extended_config_path(",
  );

  assert.match(extendedConfigPath, /source_path_requires_adapter_boundary\(project_root, &candidate\)/);
  assert.match(extendedConfigPath, /source_path_requires_adapter_boundary\(&project_root, &canonical\)/);
  assert.match(extendedConfigPath, /return Ok\(None\);/);
  assert.doesNotMatch(extendedConfigPath, /extended config is outside the project/);

  const boundaryCheckIndex = extendedConfigPath.indexOf(
    "source_path_requires_adapter_boundary(project_root, &candidate)",
  );
  const canonicalizeIndex = extendedConfigPath.indexOf("candidate.canonicalize()");
  assert.ok(
    boundaryCheckIndex > -1 && canonicalizeIndex > -1 && boundaryCheckIndex < canonicalizeIndex,
    "external extends must be rejected before canonicalizing or reading the outside config",
  );

  assert.match(
    fixture,
    /source_build_engine_ignores_external_jsconfig_extends_without_reading_outside_project/,
  );
  assert.match(fixture, /"extends": "\.\.\/shared-config\/jsconfig\.base\.json"/);
  assert.match(fixture, /"@external\/\*": \["external\/\*"\]/);
  assert.match(fixture, /dependency\["specifier"\] == "@external\/Secret"/);
  assert.match(fixture, /dependency\["resolver_source"\] == "external-package-boundary"/);

  assert.match(
    resolver,
    /source_owned_extended_config_path_keeps_local_config_before_canonical_boundary/,
  );
  assert.match(
    resolver,
    /source_owned_extended_config_path_ignores_missing_external_config_before_canonicalize/,
  );
});
