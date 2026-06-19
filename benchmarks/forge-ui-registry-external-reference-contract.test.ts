import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = process.cwd();
const schemaPath = join(
  repoRoot,
  "core",
  "src",
  "ecosystem",
  "forge_registry_parts",
  "forge_ui_registry_schema.rs",
);

const source = readFileSync(schemaPath, "utf8");

test("forge ui registry plans external references as bridge evidence", () => {
  assert.match(
    source,
    /external_registry_references: Vec<DxForgeUiRegistryExternalReferenceEvidence>/,
  );
  assert.match(source, /struct DxForgeUiRegistryExternalReferenceEvidence/);
  assert.match(source, /enum DxForgeUiRegistryExternalReferenceKind/);
  assert.match(source, /RemoteRegistryUrl/);
  assert.match(source, /NamespacedRegistry/);
  assert.match(source, /GithubSourceRegistry/);
  assert.match(source, /LocalRegistryFile/);
  assert.match(source, /requires_bridge: true/);
  assert.match(source, /no_network_request: true/);
});

test("forge ui registry classifies official reference forms without fetching", () => {
  assert.match(source, /classify_forge_ui_registry_external_reference/);
  assert.match(source, /forge_ui_registry_external_reference_kind/);
  assert.match(
    source,
    /reference\.starts_with\("http:\/\/"\) \|\| reference\.starts_with\("https:\/\/"\)/,
  );
  assert.match(
    source,
    /reference\.starts_with\("\.\/"\) \|\| reference\.starts_with\("\.\.\/"\)/,
  );
  assert.match(source, /reference\.starts_with\('@'\) && reference\.contains\('\/'\)/);
  assert.match(source, /forge_ui_registry_registry_dependency_is_github_address\(reference\)/);
  assert.doesNotMatch(source, /reqwest::/);
  assert.doesNotMatch(source, /ureq::/);
});

test("forge ui registry rejects command-shaped registry dependencies", () => {
  assert.match(source, /validate_forge_ui_registry_registry_dependency_reference/);
  assert.match(
    source,
    /registry dependencies must be item names, namespaced items, GitHub registry addresses, local registry files, or reviewed registry URLs/,
  );
  for (const command of [
    'lower.starts_with("npm ")',
    'lower.starts_with("pnpm ")',
    'lower.starts_with("yarn ")',
    'lower.starts_with("bun ")',
    'lower.starts_with("npx ")',
    'lower.starts_with("cargo ")',
    'lower.starts_with("pip ")',
    'lower.starts_with("uv ")',
    'lower.starts_with("go get ")',
    'lower.starts_with("dart ")',
  ]) {
    assert.ok(source.includes(command), `missing command guard ${command}`);
  }
});
