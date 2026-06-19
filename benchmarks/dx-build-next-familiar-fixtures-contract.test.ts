import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("dx build emits Next-familiar compatibility probes without parity artifact names", () => {
  const cli = read("dx-www/src/cli/mod.rs");
  const nextMigration = read("dx-www/src/cli/next_migration.rs");
  const nextMigrationPlan = read("dx-www/src/cli/next_migration_plan.rs");
  const nextAdapterFixtures = read("dx-www/src/cli/next_adapter_fixtures.rs");
  const fixtures = read("dx-www/src/cli/next_familiar_fixtures.rs");
  const hostingManifest = read("dx-www/src/cli/forge_hosting_manifest.rs");
  const hostedPreview = read("dx-www/src/cli/hosted_preview_contract.rs");

  assert.doesNotMatch(cli, /next_parity_fixtures|next-parity-fixtures/);
  assert.doesNotMatch(cli, /next_parity_evidence|next-runtime-parity-evidence|next_runtime_parity_evidence/);
  assert.doesNotMatch(cli, /next_familiar_evidence/);
  assert.doesNotMatch(nextMigration, /runtime parity/i);
  assert.doesNotMatch(nextMigration, /next-runtime-parity-evidence|next_runtime_parity_evidence|next_parity_evidence/);
  assert.doesNotMatch(nextMigrationPlan, /strict parity|parity evidence/i);
  assert.doesNotMatch(nextAdapterFixtures, /parity evidence/i);
  assert.doesNotMatch(hostingManifest, /next_parity_fixtures|next-parity-fixtures/);
  assert.doesNotMatch(hostingManifest, /next_parity_evidence|next-runtime-parity-evidence|next_runtime_parity_evidence/);
  assert.doesNotMatch(hostingManifest, /next_familiar_evidence/);
  assert.doesNotMatch(hostedPreview, /next_parity_fixtures|next-parity-fixtures/);
  assert.doesNotMatch(hostedPreview, /next_parity_evidence|next-runtime-parity-evidence|next_runtime_parity_evidence/);
  assert.doesNotMatch(hostedPreview, /next_familiar_evidence/);
  assert.equal(fs.existsSync(path.join(repoRoot, "dx-www/src/cli/next_parity_fixtures.rs")), false);

  assert.match(cli, /next_familiar_compatibility_evidence_emitted/);
  assert.match(cli, /next_familiar_compatibility_evidence/);
  assert.match(cli, /deploy_next_familiar_compatibility_contract/);
  assert.match(
    nextMigration,
    /NEXT_FAMILIAR_COMPATIBILITY_EVIDENCE_JSON:\s*&str\s*=\s*"next-familiar-compatibility-evidence\.json"/,
  );
  assert.match(nextMigration, /build_next_familiar_compatibility_evidence/);
  assert.match(nextMigration, /deploy_next_familiar_compatibility_contract/);
  assert.doesNotMatch(cli, /"next-familiar-evidence"/);
  assert.doesNotMatch(hostedPreview, /"next-familiar-evidence"/);
  assert.match(cli, /"next-familiar-compatibility-evidence"/);
  assert.match(hostedPreview, /"next-familiar-compatibility-evidence"/);
  assert.match(nextMigration, /NEXT_APP_ROOTS:\s*&\[&str\]\s*=\s*&\["app", "src\/app"\]/);
  assert.match(nextMigration, /NEXT_PAGE_FILE_NAMES:\s*&\[&str\][\s\S]*"page\.tsx"[\s\S]*"page\.jsx"[\s\S]*"page\.ts"[\s\S]*"page\.js"/);
  assert.match(nextMigration, /NEXT_ROUTE_HANDLER_FILE_NAMES:\s*&\[&str\][\s\S]*"route\.ts"[\s\S]*"route\.tsx"[\s\S]*"route\.js"[\s\S]*"route\.jsx"/);
  assert.match(nextMigration, /scan_matching_files\(project_dir, NEXT_APP_ROOTS, NEXT_PAGE_FILE_NAMES\)/);
  assert.match(nextMigration, /scan_matching_files\(project_dir, NEXT_APP_ROOTS, NEXT_ROUTE_HANDLER_FILE_NAMES\)/);
  assert.match(nextMigration, /parts\.iter\(\)\.position\(\|part\| part == "app"\)/);
  assert.match(cli, /next_familiar_fixtures_emitted/);
  assert.match(cli, /deploy_next_familiar_fixtures_contract/);
  assert.match(cli, /write_next_familiar_fixtures/);
  assert.match(fixtures, /NEXT_FAMILIAR_FIXTURES_JSON: &str = "next-familiar-fixtures\.json"/);
  assert.match(fixtures, /"fixture_family": "next-familiar-app-router-compatibility"/);
  assert.match(fixtures, /"strict_runtime_proof": \{/);
  assert.doesNotMatch(fixtures, /parity/i);
  assert.match(fixtures, /NEXT_APP_ROOTS:\s*&\[&str\]\s*=\s*&\["app", "src\/app"\]/);
  assert.match(fixtures, /NEXT_PAGE_FILE_NAMES:\s*&\[&str\][\s\S]*"page\.tsx"[\s\S]*"page\.jsx"[\s\S]*"page\.ts"[\s\S]*"page\.js"/);
  assert.match(fixtures, /NEXT_ROUTE_HANDLER_FILE_NAMES:\s*&\[&str\][\s\S]*"route\.ts"[\s\S]*"route\.tsx"[\s\S]*"route\.js"[\s\S]*"route\.jsx"/);
  assert.match(fixtures, /app_router_files\(project_dir\)/);
  assert.match(fixtures, /file_name_in\(entry\.path\(\), NEXT_PAGE_FILE_NAMES\)/);
  assert.match(fixtures, /route_handler_fixtures\(project_dir\)/);
  assert.match(fixtures, /file_name_in\(entry\.path\(\), NEXT_ROUTE_HANDLER_FILE_NAMES\)/);
  assert.match(fixtures, /"route_handlers": route_handlers/);
  assert.match(fixtures, /"route_handler_count": route_handlers\.len\(\)/);
  assert.match(fixtures, /"adapter_boundary": !is_build_safe_route_handler\(&methods\)/);
  assert.match(fixtures, /fn export_block_exports_route_handler_method/);
  assert.match(fixtures, /route_handler_methods_detect_export_aliases/);
  assert.match(fixtures, /handler as GET/);
  assert.doesNotMatch(fixtures, /source\.contains\(&format!\("\{\{ \{method\},"/);
  assert.doesNotMatch(fixtures, /source\.contains\(&format!\(", \{method\}"/);
  assert.match(fixtures, /parts\.iter\(\)\.position\(\|part\| \*part == "app"\)/);
  assert.match(hostingManifest, /next_familiar_fixtures/);
  assert.match(hostedPreview, /next_familiar_fixtures/);
});
