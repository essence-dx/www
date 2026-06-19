import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");

function read(relativePath) {
  return readFileSync(join(repoRoot, relativePath), "utf8");
}

test("three scene package owns a physical source import alias", () => {
  const forgeThree = read("core/src/ecosystem/forge_three_scene.rs");
  const registryConfig = read(
    "core/src/ecosystem/forge_registry_parts/registry_config.rs",
  );
  const packageLanes = read(
    "core/src/ecosystem/forge_registry_parts/package_lanes.rs",
  );

  assert.match(forgeThree, /"js\/three\/index\.ts"/);
  assert.match(forgeThree, /DX_THREE_SCENE_INDEX_IMPORT/);
  assert.match(registryConfig, /\("js\/three"\.to_string\(\), "three"\.to_string\(\)\)/);
  assert.match(packageLanes, /logical_path == "js\/three\/index\.ts"/);
  assert.match(packageLanes, /config\.materialize_path\("js\/scene\/index\.ts"\)/);
  assert.match(packageLanes, /relative_module_import\(materialized_path, &scene_index_path\)/);
});

test("three source alias stays source-owned and avoids node_modules", () => {
  const packageLanes = read(
    "core/src/ecosystem/forge_registry_parts/package_lanes.rs",
  );
  const sourceResolver = read(
    "dx-www/src/build/source_engine/module_resolver_config.rs",
  );

  assert.match(packageLanes, /"js\/three", "forge\/variants"/);
  assert.doesNotMatch(packageLanes, /node_modules\/three/);
  assert.match(sourceResolver, /RESOLVER_SOURCE_PROJECT_ROOT_ALIAS/);
  assert.match(sourceResolver, /project-root-alias-node-modules-boundary/);
});

test("three package docs distinguish package aliases from source aliases", () => {
  const docs = read("docs/packages/3d-scene-system.md");

  assert.match(docs, /Package ID aliases choose the Forge package/);
  assert.match(docs, /Source\s+import aliases are physical generated source files/);
  assert.match(docs, /`@\/three`/);
  assert.match(docs, /`three\/index\.ts`/);
  assert.match(docs, /does not make\s+`node_modules\/three`\s+resolvable/);
});
