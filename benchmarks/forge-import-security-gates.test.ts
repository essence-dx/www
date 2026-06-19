import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = process.cwd();
const importerRoot = join(repoRoot, "core", "src", "ecosystem", "forge_importer");

function readImporterFile(name) {
  return readFileSync(join(importerRoot, name), "utf8");
}

test("forge importer defines the universal firewall module set", () => {
  const modSource = readImporterFile("mod.rs");

  for (const moduleName of [
    "acquire",
    "analyze",
    "quarantine",
    "receipts",
    "slice",
    "types",
  ]) {
    assert.match(modSource, new RegExp(`pub mod ${moduleName};`));
  }
});

test("forge importer supports modeled popular ecosystems without install execution", () => {
  const typesSource = readImporterFile("types.rs");
  const acquireSource = readImporterFile("acquire.rs");

  for (const ecosystem of [
    "Npm",
    "Pip",
    "Cargo",
    "Go",
    "Pub",
    "Maven",
    "Nuget",
    "Composer",
    "Gem",
    "Swift",
  ]) {
    assert.match(typesSource, new RegExp(`\\b${ecosystem}\\b`));
  }

  for (const blockedCommand of [
    "npm install",
    "pip install",
    "cargo add",
    "go get",
    "dart pub get",
    "mvn install",
    "dotnet restore",
    "composer install",
    "gem install",
    "swift build",
  ]) {
    assert.match(typesSource, new RegExp(blockedCommand));
  }

  assert.match(acquireSource, /executes_package_code:\s*false/);
});

test("forge importer rejects unsafe package paths before quarantine", () => {
  const quarantineSource = readImporterFile("quarantine.rs");

  for (const guard of [
    "node_modules",
    "ParentDir",
    "RootDir",
    "Prefix",
    "is_absolute",
  ]) {
    assert.match(quarantineSource, new RegExp(guard.replaceAll("\\", "\\\\")));
  }

  assert.ok(
    quarantineSource.includes("trimmed.contains('\\\\')"),
    "backslash paths must be rejected before quarantine",
  );
  assert.match(quarantineSource, /validate_import_target_path/);
  assert.match(quarantineSource, /symlink_metadata/);
  assert.match(quarantineSource, /ProjectEscape/);
});

test("forge importer blocks importable source for lifecycle and native risks", () => {
  const typesSource = readImporterFile("types.rs");
  const sliceSource = readImporterFile("slice.rs");

  for (const risk of [
    "LifecycleScript",
    "InstallHook",
    "NativeBinary",
    "DynamicExecution",
    "ObfuscatedBlob",
    "ProjectEscape",
    "PlaintextSecret",
  ]) {
    assert.match(typesSource, new RegExp(`\\b${risk}\\b`));
  }

  assert.match(sliceSource, /writes_importable_source/);
  assert.match(sliceSource, /DxForgeImportSliceKind::Blocked/);
  assert.match(sliceSource, /DxForgeImportDecision::Block/);
});
