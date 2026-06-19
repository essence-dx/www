import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const importerRoot = join(repoRoot, "core", "src", "ecosystem", "forge_importer");

function read(relativePath) {
  return readFileSync(join(repoRoot, relativePath), "utf8");
}

function readImporter(name) {
  return readFileSync(join(importerRoot, name), "utf8");
}

test("universal importer modules are exported and plan-only", () => {
  const modSource = readImporter("mod.rs");
  const typesSource = readImporter("types.rs");
  const capabilitySource = readImporter("capabilities.rs");

  for (const moduleName of [
    "npm",
    "pip",
    "cargo",
    "go",
    "jsr",
    "capabilities",
    "pub_package",
    "maven",
    "nuget",
    "composer",
    "gem",
    "swift",
    "hex",
    "cran",
  ]) {
    assert.match(modSource, new RegExp(`pub mod ${moduleName};`));
    assert.match(modSource, new RegExp(`pub use ${moduleName}::\\*;`));
  }

  assert.match(typesSource, /pub struct DxForgeImportPlanSurface/);
  assert.match(typesSource, /live_fetching_enabled:\s*false/);
  assert.match(typesSource, /package_manager_execution:\s*false/);
  assert.match(typesSource, /accepted_import_receipt_required:\s*true/);
  assert.match(capabilitySource, /DX_FORGE_IMPORT_CAPABILITY_MODEL_VERSION/);
  assert.match(capabilitySource, /pub fn import_capability_for_ecosystem/);
  assert.match(capabilitySource, /pub fn default_import_capabilities/);
});

test("ecosystem capabilities separate modeled breadth from direct WWW package imports", () => {
  const capabilitySource = readImporter("capabilities.rs");
  const planSource = read("dx-www/src/cli/forge_import_plan.rs");
  const helpSource = read("dx-www/src/cli/help_text.rs");
  const commandSource = read("dx-www/src/cli/mod_parts/cli_forge_commands_a.rs");

  assert.match(capabilitySource, /pub enum DxForgeImportCapabilityTier/);
  assert.match(capabilitySource, /ReviewedJavascriptAdapter/);
  assert.match(capabilitySource, /SourceSnapshot/);
  assert.match(capabilitySource, /direct_www_bare_import/);
  assert.match(capabilitySource, /package_score_can_reach_100/);
  assert.match(capabilitySource, /score_100_requirements/);
  assert.match(capabilitySource, /universal_package_compatibility_claim/);
  assert.match(
    capabilitySource,
    /live_registry_fetching:\s*matches!\(ecosystem, DxForgeImportEcosystem::Npm\)/,
  );
  assert.match(capabilitySource, /package_manager_execution:\s*false/);
  assert.match(capabilitySource, /universal_package_compatibility_claim:\s*false/);
  assert.match(
    capabilitySource,
    /DxForgeImportEcosystem::Npm \| DxForgeImportEcosystem::Jsr/,
  );
  assert.match(
    capabilitySource,
    /direct WWW bare imports require a reviewed adapter or explicit bridge/,
  );
  assert.match(
    capabilitySource,
    /native, toolchain, server, or language runtime behavior stays behind a bridge/,
  );
  for (const requirement of [
    "verified package provenance",
    "source bill of materials receipt",
    "reviewer-accepted license evidence",
    "live or reviewed advisory coverage",
    "no package-manager installs",
  ]) {
    assert.match(
      capabilitySource,
      new RegExp(requirement.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
    );
  }

  for (const marker of [
    "ecosystem_capability: DxForgeImportEcosystemCapability",
    "import_capability_for_ecosystem(ecosystem)",
    "Capability tier:",
    "Direct WWW bare import:",
    "Universal package compatibility claim:",
    "Clean package can score 100:",
    "import_capability_model_version",
    "import_capability_direct_www_bare_import",
    "import_capability_universal_package_compatibility_claim",
    "import_capability_supported_slice_kinds",
    "import_capability_bridge_kinds",
    "import_capability_score_100_requirements",
  ]) {
    assert.match(planSource, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(helpSource, /Ecosystem support means modeled review surfaces, not universal or live package-manager compatibility/);
  assert.match(commandSource, /Ecosystem support means modeled review surfaces, not universal or live package-manager compatibility/);
  assert.match(helpSource, /clean package-name imports require a compatible reviewed adapter or bridge/);
  assert.match(commandSource, /clean package-name imports require a compatible reviewed adapter or bridge/);
});

test("ecosystem surfaces forbid package manager execution", () => {
  const typesSource = readImporter("types.rs");
  const cases = [
    ["npm.rs", ["postinstall lifecycle script"]],
    ["pip.rs", ["setup.py execution required"]],
    ["cargo.rs", ["build.rs script"]],
    ["go.rs", ["cgo requirement"]],
    ["jsr.rs", ["Deno permission or unstable API boundary"]],
    ["pub_package.rs", ["build_runner or generated source requirement"]],
    ["maven.rs", ["Gradle build script execution"]],
    ["nuget.rs", ["MSBuild target or props execution"]],
    ["composer.rs", ["Composer script execution"]],
    ["gem.rs", ["native extension build"]],
    ["swift.rs", ["Swift package plugin execution"]],
    ["hex.rs", ["NIF or port-driver native code"]],
    ["cran.rs", ["configure or cleanup script execution"]],
  ];

  for (const [fileName, phrases] of cases) {
    const source = readImporter(fileName);
    assert.match(source, /DxForgeImportPlanSurface::non_executing/);
    for (const phrase of phrases) {
      assert.match(source, new RegExp(phrase.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
    }
  }

  for (const command of [
    "npm install",
    "pnpm install",
    "pip install",
    "python setup.py",
    "cargo add",
    "cargo build",
    "go get",
    "go generate",
    "deno add",
    "jsr add",
    "dart pub get",
    "flutter pub get",
    "mvn install",
    "gradle build",
    "dotnet restore",
    "nuget install",
    "composer install",
    "gem install",
    "bundle install",
    "swift package resolve",
    "swift build",
    "mix deps.get",
    "rebar3 compile",
    "R CMD INSTALL",
    "install.packages",
  ]) {
    assert.match(typesSource, new RegExp(command.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
});

test("ecosystem parser accepts common registry aliases but keeps canonical segments", () => {
  const typesSource = readImporter("types.rs");
  const docs = read("docs/forge-universal-importers.md");
  const compactDocs = docs.replace(/\s+/g, " ");
  const planSource = read("dx-www/src/cli/forge_import_plan.rs");
  const helpSource = read("dx-www/src/cli/help_text.rs");
  const forgeCommandSource = read("dx-www/src/cli/mod_parts/cli_forge_commands_a.rs");

  assert.match(typesSource, /DX_FORGE_IMPORT_ECOSYSTEM_ALIASES_HELP/);
  assert.match(typesSource, /pub fn supported_aliases_help\(\)/);
  assert.match(typesSource, /"pypi" \| "pypi\.org" \| "python" => Some\(Self::Pip\)/);
  assert.match(typesSource, /"crates" \| "crates\.io" \| "rust" => Some\(Self::Cargo\)/);
  assert.match(typesSource, /"golang" \| "gomod" \| "go-mod" \| "go-module" => Some\(Self::Go\)/);
  assert.match(typesSource, /"pub\.dev" \| "dart" \| "flutter" => Some\(Self::Pub\)/);
  assert.match(typesSource, /"packagist" \| "php" => Some\(Self::Composer\)/);
  assert.match(typesSource, /"rubygems" \| "ruby" \| "bundler" => Some\(Self::Gem\)/);
  assert.match(typesSource, /"dotnet" \| "\.net" \| "csharp" => Some\(Self::Nuget\)/);
  assert.match(planSource, /aliases: \{\}/);
  assert.match(planSource, /DxForgeImportEcosystem::supported_aliases_help\(\)/);
  assert.match(helpSource, /DX_FORGE_IMPORT_ECOSYSTEM_ALIASES_HELP/);
  assert.match(helpSource, /Ecosystem aliases normalize into canonical Forge receipt paths/);
  assert.match(forgeCommandSource, /DX_FORGE_IMPORT_ECOSYSTEM_ALIASES_HELP/);
  assert.match(forgeCommandSource, /Ecosystem aliases normalize into canonical Forge receipt paths/);
  assert.match(helpSource, /Package names are validated per ecosystem/);
  assert.match(forgeCommandSource, /Package names are validated per ecosystem/);

  for (const phrase of [
    "The CLI accepts common registry aliases",
    "machine artifacts keep canonical Forge ecosystem segments",
    "`pypi` normalizes to `pip`",
    "`crates.io` normalizes to `cargo`",
    "`golang` normalizes to `go`",
    "`packagist` normalizes to `composer`",
    "`rubygems` normalizes to `gem`",
    "`dotnet` normalizes to `nuget`",
    "Package identities are validated before they become package ids",
    "Maven accepts `group:artifact` or `group/artifact`",
    "URLs, whitespace, traversal, shell metacharacters, and package-manager commands are rejected",
  ]) {
    assert.match(compactDocs, new RegExp(phrase.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
  assert.match(compactDocs, /`pub\.dev` normalizes to `pub`/);
});

test("forge import validates package identity before writing receipt paths", () => {
  const typesSource = readImporter("types.rs");
  const planSource = read("dx-www/src/cli/forge_import_plan.rs");
  const acquireSource = readImporter("acquire.rs");

  assert.match(typesSource, /pub fn validate_import_package_name/);
  assert.match(typesSource, /validate_javascript_package_name/);
  assert.match(typesSource, /validate_python_package_name/);
  assert.match(typesSource, /validate_maven_package_name/);
  assert.match(typesSource, /validate_two_part_path_package_name/);
  assert.match(typesSource, /validate_path_package_name/);
  assert.match(typesSource, /validate_lower_identifier_package_name/);
  assert.match(typesSource, /validate_cran_package_name/);
  assert.match(typesSource, /package_name\.contains\(":\/\/"\)/);
  assert.match(typesSource, /package_name\.chars\(\)\.any\(char::is_whitespace\)/);
  assert.match(
    typesSource,
    /matches!\(ch, ';' \| '&' \| '\|' \| '<' \| '>' \| '`' \| '\$'\)/,
  );
  assert.match(planSource, /validate_import_package_name\(ecosystem, package_name\)\?/);
  assert.match(acquireSource, /let package_id_slug = acquisition_package_slug\(package_id\)/);
  assert.doesNotMatch(acquireSource, /package_id\.replace\(\['\/', '@'\], "-"\)/);
});

test("universal importer docs keep dx add gated behind accepted receipts", () => {
  const docs = read("docs/forge-universal-importers.md");
  const compactDocs = docs.replace(/\s+/g, " ");

  for (const phrase of [
    "Default import planning is non-executing",
    "Quarantine is not importable app source",
    "Live fetching stays behind explicit acquisition commands and receipts",
    "npm is the first implemented live fetcher",
    "Materialization requires an accepted import receipt",
    "declaration-only marker files stay below green",
    "dx forge acquire npm <package>",
    "dx forge add npm <package>",
    "dx add pip/foo",
    "dx add cargo/foo",
    "dx add go/foo",
    "dx add jsr/foo",
    "dx add pub/foo",
    "dx add maven/foo",
    "dx add nuget/foo",
    "dx add composer/foo",
    "dx add gem/foo",
    "dx add swift/foo",
    "dx add hex/foo",
    "dx add cran/foo",
    "Hex",
    "CRAN",
    "remain unsupported unless an accepted import receipt exists",
    "not universal package compatibility",
  ]) {
    assert.match(compactDocs, new RegExp(phrase.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
});

test("forge import plans expose acquisition policy to tools", () => {
  const planSource = read("dx-www/src/cli/forge_import_plan.rs");
  const acquireSource = read("core/src/ecosystem/forge_importer/acquire.rs");

  for (const field of [
    "acquisition_metadata_inputs",
    "acquisition_artifact_inputs",
    "acquisition_plan",
    "forbidden_commands",
    "live_fetching_enabled",
    "package_manager_execution_allowed",
    "accepted_import_receipt_required",
    "unsupported_dx_add_form",
  ]) {
    assert.match(planSource, new RegExp(field));
  }

  assert.match(planSource, /surface\.metadata_inputs\.clone\(\)/);
  assert.match(planSource, /surface\.artifact_inputs\.clone\(\)/);
  assert.match(planSource, /acquisition_plan_for_package\(ecosystem, package_name, &package_id\)/);
  assert.match(planSource, /surface\.forbidden_commands\.clone\(\)/);
  assert.match(planSource, /Acquisition metadata inputs:/);
  assert.match(planSource, /Forbidden commands:/);
  assert.match(planSource, /sr_string_array\(&report\.acquisition_metadata_inputs\)/);
  assert.match(
    planSource,
    /sr_string_array\(&report\.acquisition_plan\.metadata_references\)/,
  );
  assert.match(planSource, /sr_string\(&report\.acquisition_plan\.expected_source_dir\)/);
  assert.match(planSource, /sr_string\(&report\.acquisition_plan\.quarantine_dir\)/);
  assert.match(
    planSource,
    /sr_string\(&report\.acquisition_plan\.evidence_receipt_path\)/,
  );
  assert.match(planSource, /sr_string_array\(&report\.forbidden_commands\)/);

  assert.match(acquireSource, /pub struct DxForgeImportAcquisitionPlan/);
  assert.match(acquireSource, /pub fn acquisition_plan_for_package/);
  assert.match(
    acquireSource,
    /live_fetching_enabled:\s*matches!\(ecosystem, DxForgeImportEcosystem::Npm\)/,
  );
  assert.match(acquireSource, /package_manager_execution:\s*false/);
  assert.match(acquireSource, /executes_package_code:\s*false/);
  assert.match(acquireSource, /source_dir_required_for_materialization:\s*true/);
  assert.match(acquireSource, /\.dx\/cache\/\{ecosystem_segment\}\/\{package_slug\}\/package/);
  assert.match(acquireSource, /\.dx\/forge\/quarantine\/\{ecosystem_segment\}\/\{package_slug\}/);
  assert.match(acquireSource, /\.dx\/forge\/import-receipts\/\{package_id_slug\}-acquire\.sr/);
});

test("reviewed JavaScript adapters include npm and jsr source slices", () => {
  const planSource = read("dx-www/src/cli/forge_import_plan.rs");

  assert.match(planSource, /fn reviewed_javascript_adapter_ecosystem/);
  assert.match(planSource, /DxForgeImportEcosystem::Npm => Some\("npm"\)/);
  assert.match(planSource, /DxForgeImportEcosystem::Jsr => Some\("jsr"\)/);
  assert.match(planSource, /fn reviewed_javascript_adapter_specifier/);
  assert.match(
    planSource,
    /format!\("lib\/forge\/\{ecosystem_segment\}\/\{package_slug\}\/"\)/,
  );
  assert.doesNotMatch(
    planSource,
    /if ecosystem != DxForgeImportEcosystem::Npm\s*\{/,
  );
});

test("CLI import resolver consumes npm and jsr reviewed JavaScript adapters", () => {
  const cliSource = read("dx-www/src/cli/mod_parts/cli_core_impl.rs");

  assert.match(cliSource, /reviewed_javascript_adapters_for_package/);
  assert.match(cliSource, /reviewed_javascript_adapter_origin/);
  assert.match(cliSource, /javascript_package_name_from_reference/);
  assert.match(cliSource, /"npm" => package\.source_kind == DxSourceKind::NpmSnapshot/);
  assert.match(cliSource, /"jsr" => package\.source_kind == DxSourceKind::ExternalSnapshot/);
  assert.match(cliSource, /format!\("lib\/forge\/\{ecosystem_segment\}\/"\)/);
  assert.doesNotMatch(cliSource, /reviewed_npm_adapters_for_package/);
  assert.doesNotMatch(cliSource, /reviewed_npm_subpath_specifier/);
});

test("import resolution types do not describe reviewed adapters as npm-only", () => {
  const resolutionSource = read("core/src/delivery/import_resolution.rs");

  assert.match(resolutionSource, /Reviewed JavaScript-family adapter/);
  assert.match(resolutionSource, /Import specifier exposed by the adapter/);
  assert.match(resolutionSource, /Reviewed adapter for source-owned package compatibility/);
  assert.doesNotMatch(resolutionSource, /npm-origin/);
  assert.doesNotMatch(resolutionSource, /Bare npm package name/);
});
