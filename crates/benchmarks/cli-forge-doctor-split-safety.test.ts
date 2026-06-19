import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const modPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const doctorModulePath = path.join(repoRoot, "dx-www", "src", "cli", "forge_doctor.rs");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("Forge doctor command construction and rendering live outside cli mod.rs", () => {
  assert.equal(
    fs.existsSync(doctorModulePath),
    true,
    "expected forge_doctor.rs to own doctor reports and rendering",
  );

  const modSource = read(modPath);
  const moduleSource = read(doctorModulePath);

  assert.match(modSource, /\bmod forge_doctor;/);
  const importBlock = modSource.match(/use self::forge_doctor::\{[\s\S]*?\};/);
  assert.ok(importBlock, "expected forge_doctor import block in cli mod.rs");

  for (const helper of [
    "DxForgeDoctorRegistryCheck",
    "build_forge_doctor_report",
    "run_forge_doctor",
  ]) {
    assert.equal(importBlock[0].includes(helper), true, `${helper} should be imported`);
  }

  for (const removedFromMod of [
    "fn cmd_forge_doctor(",
    "struct DxForgeDoctorReport",
    "struct DxForgeDoctorRegistryCheck",
    "struct DxForgeDoctorPackageDoc",
    "Unknown forge doctor option",
    "DX Forge doctor failed",
    "fn build_forge_doctor_report(",
    "fn forge_doctor_registry_check(",
    "fn forge_doctor_package_docs(",
    "fn forge_doctor_package_doc_name(",
    "fn print_forge_doctor_terminal(",
    "fn forge_doctor_markdown(",
  ]) {
    assert.equal(
      modSource.includes(removedFromMod),
      false,
      `${removedFromMod} should not live in cli mod.rs`,
    );
  }

  for (const expectedInModule of [
    "pub(super) fn run_forge_doctor(",
    "pub(super) struct DxForgeDoctorReport",
    "pub(super) struct DxForgeDoctorRegistryCheck",
    "pub(super) struct DxForgeDoctorPackageDoc",
    "Unknown forge doctor option",
    "DX Forge doctor failed",
    "pub(super) fn build_forge_doctor_report(",
    "fn forge_doctor_registry_check(",
    "fn forge_doctor_package_docs(",
    "fn forge_doctor_package_doc_name(",
    "pub(super) fn print_forge_doctor_terminal(",
    "pub(super) fn forge_doctor_markdown(",
  ]) {
    assert.equal(
      moduleSource.includes(expectedInModule),
      true,
      `${expectedInModule} should live in forge_doctor.rs`,
    );
  }

  for (const contractText of [
    "DX Forge doctor",
    "# DX Forge Doctor",
    "Launch Gate",
    "Registry Integrity",
    "Package Docs",
    "source-manifest.json",
  ]) {
    assert.equal(
      moduleSource.includes(contractText),
      true,
      `doctor module should preserve ${contractText}`,
    );
  }
});
