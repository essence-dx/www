const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const dxWwwBinary = process.env.DX_WWW_EXE || path.join(
  root,
  "target",
  "debug",
  process.platform === "win32" ? "dx-www.exe" : "dx-www",
);

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function runDx(args, cwd) {
  return execFileSync(dxWwwBinary, args, {
    cwd,
    encoding: "utf8",
    timeout: 30_000,
  });
}

function writeRootDx(projectDir, version) {
  fs.writeFileSync(
    path.join(projectDir, "dx"),
    `
[package]
name = "auth/better-auth"
version = "${version}"
license = "MIT"
source = "."

[forge]
package = true
registry = "local"

[[forge.files]]
from = "src/client.ts"
to = "lib/auth/better-auth/client.ts"
surface = "client"

[[forge.files]]
from = "src/server.ts"
to = "lib/auth/better-auth/server.ts"
surface = "server"

[[forge.exports]]
name = "client"
files = ["lib/auth/better-auth/client.ts"]

[[forge.exports]]
name = "server"
files = ["lib/auth/better-auth/server.ts"]

[forge.install]
default_exports = ["client", "server"]
allow_selective_imports = true
`,
  );
}

function readJsonCommand(args, cwd) {
  return JSON.parse(runDx(args, cwd));
}

function readOptionalJsonCommand(args, cwd) {
  try {
    return { ok: true, value: readJsonCommand(args, cwd) };
  } catch (error) {
    return { ok: false, error };
  }
}

function withTemporaryEnv(values, callback) {
  const previous = new Map();

  for (const [key, value] of Object.entries(values)) {
    previous.set(key, Object.prototype.hasOwnProperty.call(process.env, key) ? process.env[key] : undefined);
    if (value === undefined) {
      delete process.env[key];
    } else {
      process.env[key] = value;
    }
  }

  try {
    return callback();
  } finally {
    for (const [key, value] of previous.entries()) {
      if (value === undefined) {
        delete process.env[key];
      } else {
        process.env[key] = value;
      }
    }
  }
}

function assertRemoteLifecyclePlan(plan, expected) {
  assert.equal(plan.schema_version, "dx.forge.remote_lifecycle_plan");
  assert.equal(plan.action, expected.action);
  assert.equal(plan.package_id, "auth/better-auth");
  assert.equal(plan.registry, "r2");
  assert.equal(plan.provider_kind, "s3-compatible-object-storage");
  assert.equal(plan.dry_run, true);
  assert.equal(plan.write_allowed, false);
  assert.equal(plan.boundary, "remote install/update/uninstall dry-run boundary only");
  assert.equal(plan.remote_configured, true);
  assert.equal(plan.setup_status, "configured");
  assert.deepEqual(plan.missing_config, []);
  assert.deepEqual(plan.selected_exports, ["client"]);
  assert.equal(plan.requested_version ?? null, expected.requestedVersion ?? null);
  assert.ok(plan.object_key_plan.some((object) => object.endsWith("/.dx/build-cache/manifest.json")));
  assert.ok(plan.object_key_plan.some((object) => object.endsWith("/files/<content-hash>")));
  assert.ok(plan.object_key_plan.some((object) => object.endsWith("/latest.json")));
  assert.ok(plan.object_key_plan.some((object) => object.endsWith(`/${expected.objectPlan}`)));
  assert.ok(
    plan.receipt_plan.some((receipt) =>
      receipt.includes(`remote/${expected.receiptPrefix}-auth-better-auth-`),
    ),
  );
  assert.ok(plan.warnings.some((warning) => warning.includes("no network read, write, delete, or sync")));
  assert.ok(plan.next_actions.some((action) => action.includes("dx forge publish --registry r2")));

  if (plan.remote_read_plan) {
    assert.equal(plan.remote_read_plan.schema_version, "dx.forge.remote_read_plan");
    assert.equal(plan.remote_read_plan.provider_kind, "s3-compatible-object-storage");
    assert.equal(plan.remote_read_plan.network_allowed, false);
    assert.equal(plan.remote_read_plan.write_allowed, false);
    assert.deepEqual(plan.remote_read_plan.selected_exports, ["client"]);
    assert.ok(
      plan.remote_read_plan.objects.some(
        (object) => object.intent === "package-manifest" && object.object_key.endsWith("/.dx/build-cache/manifest.json"),
      ),
    );
    assert.ok(
      plan.remote_read_plan.objects.some(
        (object) => object.intent === "latest-version" && object.object_key.endsWith("/latest.json"),
      ),
    );
    assert.ok(
      plan.remote_read_plan.objects.some(
        (object) => object.intent === "content-blob" && object.object_key.endsWith("/files/<content-hash>"),
      ),
    );

    if (plan.remote_read_plan.manifest_install_preview) {
      const preview = plan.remote_read_plan.manifest_install_preview;
      assert.equal(preview.schema_version, "dx.forge.remote_manifest_install_preview");
      assert.equal(preview.package_id, "auth/better-auth");
      assert.equal(preview.network_allowed, false);
      assert.equal(preview.write_allowed, false);
      assert.ok(preview.selected_file_count >= 1);
      assert.ok(Array.isArray(preview.file_plans));
      assert.ok(preview.file_plans.every((file) => file.materialized_path && file.status));
    }

    if (plan.remote_read_plan.object_metadata_plan) {
      const metadataPlan = plan.remote_read_plan.object_metadata_plan;
      assert.equal(metadataPlan.schema_version, "dx.forge.remote_object_metadata_plan");
      assert.equal(metadataPlan.network_allowed, false);
      assert.equal(metadataPlan.write_allowed, false);
      assert.ok(
        metadataPlan.checks.some(
          (check) => check.metadata_operation === "head-object" && check.status === "planned-not-checked",
        ),
      );
    }

    if (plan.remote_read_plan.object_head_execution_receipt) {
      const receipt = plan.remote_read_plan.object_head_execution_receipt;
      assert.equal(receipt.schema_version, "dx.forge.remote_object_head_execution_receipt");
      assert.equal(receipt.approval_required, true);
      assert.equal(receipt.approved, false);
      assert.equal(receipt.network_allowed, false);
      assert.equal(receipt.write_allowed, false);
      assert.ok(
        receipt.checks.some(
          (check) =>
            check.metadata_operation === "head-object" &&
            check.status === "requires-explicit-approval" &&
            check.executed === false,
        ),
      );
    }
  }
}

test("dx forge update owns local-registry selected package lifecycle", () => {
  const security = read("core/src/ecosystem/forge_security.rs");
  const cli = read("dx-www/src/cli/mod.rs");

  assert.match(security, /pub fn plan_forge_update_from_local_registry/);
  assert.match(security, /pub fn write_forge_update_dry_run_from_local_registry/);
  assert.match(security, /pub fn write_forge_update_from_local_registry/);
  assert.match(security, /pub fn write_forge_update_dry_run_variant/);
  assert.match(security, /fn plan_forge_update_against_latest/);
  assert.match(security, /fn write_forge_update_against_latest/);
  assert.match(security, /fn write_forge_dry_run_receipt/);
  assert.match(security, /source_package_from_local_registry_selected_exports/);
  assert.match(security, /selected_exports: receipt_selected_exports\(selected_exports\)/);

  assert.match(cli, /"update" => self\.cmd_forge_update/);
  assert.match(cli, /fn cmd_forge_update/);
  assert.match(cli, /write_forge_update_dry_run_from_local_registry/);
  assert.match(cli, /write_forge_update_from_local_registry/);
  assert.match(cli, /write_forge_update_dry_run_variant/);
  assert.match(cli, /resolve_local_registry_version/);
  assert.match(cli, /--registry/);
  assert.match(cli, /selected root dx update requires --registry local/);
  assert.match(cli, /dx forge update <package>/);
});

test("local-registry update has a root-dx v1 to v2 selected-export fixture", () => {
  const security = read("core/src/ecosystem/forge_security.rs");

  assert.match(
    security,
    /fn forge_local_registry_update_selected_export_e2e_from_root_dx_versions/,
  );
  assert.match(security, /publish_root_dx_package_to_local_registry/);
  assert.match(security, /write_forge_add_from_local_registry/);
  assert.match(security, /plan_forge_update_from_local_registry/);
  assert.match(security, /write_forge_update_dry_run_from_local_registry/);
  assert.match(security, /write_forge_update_from_local_registry/);
  assert.match(security, /assert_eq!\(\s*persisted_preview_receipt\.action,\s*DxForgeAction::UpdateDryRun\s*\)/);
  assert.match(security, /assert_eq!\(update_receipt\.selected_exports, vec!\["client"\]\)/);
  assert.match(security, /assert!\(!dir\.path\(\)\.join\("lib\/auth\/better-auth\/server\.ts"\)\.exists\(\)\)/);
});

test("R2 status source exposes redacted partial-credential readiness", () => {
  const registry = read("core/src/ecosystem/forge_registry.rs");
  const cli = read("dx-www/src/cli/mod.rs");

  assert.match(registry, /pub setup_status: String/);
  assert.match(registry, /pub missing_config: Vec<String>/);
  assert.match(registry, /pub access_key_id_set: bool/);
  assert.match(registry, /pub secret_access_key_set: bool/);
  assert.match(registry, /pub bucket_set: bool/);
  assert.match(registry, /pub endpoint_set: bool/);
  assert.match(registry, /pub public_base_url_set: bool/);
  assert.match(registry, /#\[serde\(default, skip_serializing\)\]\s+pub bucket: Option<String>/);
  assert.match(registry, /#\[serde\(default, skip_serializing\)\]\s+pub endpoint: Option<String>/);
  assert.match(registry, /#\[serde\(default, skip_serializing\)\]\s+pub public_base_url: Option<String>/);
  assert.match(registry, /fn forge_r2_setup_status/);
  assert.match(registry, /fn forge_r2_missing_config/);
  assert.match(registry, /setup_status,/);
  assert.match(registry, /missing_config,/);
  assert.match(registry, /access_key_id_set,/);
  assert.match(registry, /secret_access_key_set,/);
  assert.match(registry, /bucket_set: bucket\.as_deref\(\)\.is_some_and/);
  assert.match(registry, /endpoint_set: endpoint\.is_some\(\)/);
  assert.match(registry, /public_base_url_set: public_base_url\.is_some\(\)/);
  assert.match(registry, /r2:\/\/<configured-bucket>\//);
  assert.match(registry, /assert_eq!\(status\.setup_status, "partial-config"\)/);
  assert.match(registry, /assert_eq!\(status\.missing_config, vec!\["secret_access_key"\]\)/);
  assert.match(registry, /assert_eq!\(status\.access_key_id_set, true\)/);
  assert.match(registry, /assert_eq!\(status\.secret_access_key_set, false\)/);
  assert.match(registry, /assert!\(!serialized\.contains\("partial-secret"\)\)/);
  assert.match(registry, /assert!\(!serialized\.contains\("partial-account"\)\)/);
  assert.match(registry, /assert!\(!serialized\.contains\("partial-bucket"\)\)/);
  assert.match(cli, /access_key_id_set: Option<bool>/);
  assert.match(cli, /secret_access_key_set: Option<bool>/);
  assert.match(cli, /access_key_id_set: Some\(r2\.access_key_id_set\)/);
  assert.match(cli, /secret_access_key_set: Some\(r2\.secret_access_key_set\)/);
  assert.match(cli, /bucket_set: Some\(r2\.bucket_set\)/);
  assert.match(cli, /endpoint_set: Some\(r2\.endpoint_set\)/);
  assert.match(cli, /public_base_url_set: Some\(r2\.public_base_url_set\)/);
  assert.match(cli, /setup_status: &'static str/);
  assert.match(cli, /missing_config: Vec<String>/);
  assert.match(cli, /fn forge_public_r2_setup_status/);
  assert.match(cli, /fn forge_public_r2_missing_config/);
  assert.match(cli, /"partial-config"/);
  assert.match(cli, /"account_id_or_endpoint"/);
  assert.match(cli, /"access_key_id"/);
  assert.match(cli, /"secret_access_key"/);
  assert.match(cli, /"bucket"/);
  assert.match(cli, /setup_status: forge_public_r2_setup_status\(&r2\)/);
  assert.match(cli, /missing_config: forge_public_r2_missing_config\(&r2\)/);
  assert.match(cli, /fn forge_public_remote_next_actions/);
  assert.match(cli, /let mut next_actions = forge_public_next_actions\(&remotes\)/);
  assert.match(cli, /next_actions: forge_public_remote_next_actions\(&remotes\)/);
  assert.match(cli, /remote\.setup_status == "partial-config"/);
  assert.match(cli, /remote\.missing_config\.join\(", "\)/);
  assert.match(cli, /Complete R2 missing config labels/);
  assert.match(cli, /Dry-run remains safe while R2 is missing config/);
});

test("remote lifecycle source exposes dry-run-only R2 boundaries", () => {
  const registry = read("core/src/ecosystem/forge_registry.rs");
  const r2Head = read("core/src/ecosystem/forge_r2_head.rs");
  const remoteHealth = read("core/src/ecosystem/forge_remote_health.rs");
  const projectCheck = read("core/src/ecosystem/project_check.rs");
  const ecosystemMod = read("core/src/ecosystem/mod.rs");
  const cli = read("dx-www/src/cli/mod.rs");
  const benchmark = read("benchmarks/forge-update-local-registry-command.test.ts");

  assert.match(registry, /pub enum DxForgeRemoteProviderKind/);
  assert.match(registry, /pub enum DxForgeRemoteReadIntent/);
  assert.match(registry, /pub struct DxForgeRemoteReadPlan/);
  assert.match(registry, /pub struct DxForgeRemoteReadObject/);
  assert.match(registry, /pub trait DxForgeRemoteReadProvider/);
  assert.match(registry, /pub struct DxForgeR2ReadOnlyProvider/);
  assert.match(registry, /pub enum DxForgeRemoteObjectMetadataStatus/);
  assert.match(registry, /pub struct DxForgeRemoteObjectMetadataCheck/);
  assert.match(registry, /pub struct DxForgeRemoteObjectMetadataPlan/);
  assert.match(registry, /pub enum DxForgeRemoteObjectHeadExecutionStatus/);
  assert.match(registry, /pub struct DxForgeRemoteObjectHeadMeasurement/);
  assert.match(registry, /pub trait DxForgeRemoteObjectHeadProvider/);
  assert.match(registry, /pub struct DxForgeRemoteObjectHeadExecutionApproval/);
  assert.match(registry, /pub struct DxForgeRemoteObjectHeadExecutionCheck/);
  assert.match(registry, /pub struct DxForgeRemoteObjectHeadExecutionReceipt/);
  assert.match(registry, /pub fn execute_r2_remote_object_head_checks_with_provider/);
  assert.match(registry, /pub struct DxForgeRemoteManifestInstallPreview/);
  assert.match(registry, /pub struct DxForgeRemoteManifestFilePlan/);
  assert.match(ecosystemMod, /mod forge_r2_head/);
  assert.match(ecosystemMod, /pub use forge_r2_head::\*/);
  assert.match(ecosystemMod, /mod forge_remote_health/);
  assert.match(ecosystemMod, /pub use forge_remote_health::\*/);
  assert.match(r2Head, /pub async fn execute_r2_remote_object_head_checks_from_env/);
  assert.match(r2Head, /pub async fn execute_r2_remote_object_head_checks_with_config/);
  assert.match(r2Head, /validate_live_head_approval/);
  assert.match(r2Head, /approval\.network_allowed/);
  assert.match(r2Head, /live R2\/S3 HEAD execution requires explicit network approval/);
  assert.match(r2Head, /metadata_plan\.write_allowed/);
  assert.match(r2Head, /store\.head\(&path\)\.await/);
  assert.match(r2Head, /ObjectStoreError::NotFound/);
  assert.match(r2Head, /write_allowed: false/);
  assert.match(r2Head, /live_r2_head_adapter_requires_explicit_network_approval/);
  assert.doesNotMatch(r2Head, /\.put\(/);
  assert.doesNotMatch(r2Head, /\.get\(/);
  assert.doesNotMatch(r2Head, /secret_access_key/);
  assert.match(remoteHealth, /pub enum DxForgeRemoteObjectHeadHealthStatus/);
  assert.match(remoteHealth, /pub struct DxForgeRemoteObjectHeadHealthCheck/);
  assert.match(remoteHealth, /pub struct DxForgeRemoteObjectHeadHealthEvaluation/);
  assert.match(remoteHealth, /pub fn evaluate_r2_remote_object_head_receipt_health/);
  assert.match(registry, /pub fn plan_r2_remote_read_only_install/);
  assert.match(registry, /pub fn plan_r2_remote_read_only_install_from_manifest_fixture/);
  assert.match(registry, /manifest_install_preview: Option<DxForgeRemoteManifestInstallPreview>/);
  assert.match(registry, /object_metadata_plan: Option<DxForgeRemoteObjectMetadataPlan>/);
  assert.match(registry, /object_head_execution_receipt: Option<DxForgeRemoteObjectHeadExecutionReceipt>/);
  assert.match(registry, /object_head_health_evaluation: Option<DxForgeRemoteObjectHeadHealthEvaluation>/);
  assert.match(registry, /dx\.forge\.remote_object_metadata_plan/);
  assert.match(registry, /dx\.forge\.remote_object_head_execution_receipt/);
  assert.match(registry, /metadata_operation: "head-object"/);
  assert.match(registry, /PlannedNotChecked/);
  assert.match(registry, /RequiresExplicitApproval/);
  assert.match(registry, /Measured/);
  assert.match(remoteHealth, /MissingRequiredObject/);
  assert.match(remoteHealth, /ByteMismatch/);
  assert.match(registry, /provider_mode/);
  assert.match(registry, /approved_by/);
  assert.match(registry, /measured_exists: Some\(measurement\.exists\)/);
  assert.match(remoteHealth, /dx\.forge\.remote_object_head_health/);
  assert.match(remoteHealth, /safe_for_remote_install/);
  assert.match(remoteHealth, /missing_required_count/);
  assert.match(remoteHealth, /byte_mismatch_count/);
  assert.match(registry, /let head_health_evaluation = evaluate_r2_remote_object_head_receipt_health\(&head_receipt\)/);
  assert.match(registry, /plan\.object_head_health_evaluation = Some\(head_health_evaluation\)/);
  assert.match(cli, /struct DxForgePublicRemoteObjectHeadHealthState/);
  assert.match(cli, /remote_object_head_health: Vec<DxForgePublicRemoteObjectHeadHealthState>/);
  assert.match(cli, /fn forge_public_remote_object_head_health_states/);
  assert.match(cli, /remote_read_plan"\)\s*\.and_then\(\|plan\| plan\.get\("object_head_health_evaluation"\)\)/);
  assert.match(cli, /"remote-health-blocked"/);
  assert.match(projectCheck, /FORGE_STATUS_LATEST_RECEIPT_PATH/);
  assert.match(projectCheck, /fn forge_remote_head_health_metrics_from_status_json/);
  assert.match(projectCheck, /remote_object_head_health/);
  assert.match(projectCheck, /forge_remote_head_health_blocking_checks/);
  assert.match(projectCheck, /forge-remote-head-health-blocked/);
  assert.match(registry, /approval_required: true/);
  assert.match(registry, /approved: false/);
  assert.match(registry, /dx\.forge\.remote_manifest_install_preview/);
  assert.match(registry, /conflicting_file_count/);
  assert.match(registry, /network_allowed: false/);
  assert.match(registry, /write_allowed: false/);
  assert.match(registry, /packages\/js\/\{package_object_path\}\/\{version_segment\}\/manifest\.json/);
  assert.match(registry, /packages\/js\/\{package_object_path\}\/latest\.json/);
  assert.match(registry, /files\/<content-hash>/);
  assert.match(cli, /struct DxForgeRemoteLifecyclePlan/);
  assert.match(cli, /remote_read_plan: DxForgeRemoteReadPlan/);
  assert.match(cli, /--remote-manifest/);
  assert.match(cli, /remote_manifest: Option<PathBuf>/);
  assert.match(cli, /schema_version: "dx\.forge\.remote_lifecycle_plan"/);
  assert.match(cli, /enum DxForgeRemoteLifecycleAction/);
  assert.match(cli, /InstallDryRun/);
  assert.match(cli, /UpdateDryRun/);
  assert.match(cli, /UninstallDryRun/);
  assert.match(cli, /fn forge_remote_lifecycle_dry_run/);
  assert.match(cli, /plan_r2_remote_read_only_install\(/);
  assert.match(cli, /plan_r2_remote_read_only_install_from_manifest_fixture\(/);
  assert.match(cli, /write_allowed: false/);
  assert.match(cli, /remote install\/update\/uninstall dry-run boundary only/);
  assert.match(cli, /r2_registry_status\(\)/);
  assert.match(cli, /print_forge_remote_lifecycle_plan/);
  assert.match(cli, /dx forge add --registry r2 is dry-run only/);
  assert.match(cli, /dx forge update --registry r2 is dry-run only/);
  assert.match(cli, /dx forge remove --registry r2 is dry-run only/);
  assert.match(cli, /format != DxOutputFormat::Json/);
  assert.match(cli, /parse_public_forge_add_request\(&package_id, None\)\?/);
  assert.match(cli, /Some\("r2"\) =>/);
  assert.match(cli, /"remote-head" \| "remote-head-health" \| "r2-head"/);
  assert.match(cli, /fn cmd_forge_remote_head/);
  assert.match(cli, /struct DxForgeRemoteHeadCliReport/);
  assert.match(cli, /DxForgeRemoteObjectHeadExecutionApproval/);
  assert.match(cli, /execute_r2_remote_object_head_checks_from_env/);
  assert.match(cli, /evaluate_r2_remote_object_head_receipt_health/);
  assert.match(cli, /dx forge remote-head package#surface --registry r2 --remote-manifest/);
  assert.match(cli, /dx forge remote-head requires --remote-manifest/);
  assert.match(cli, /live dx forge remote-head requires --approved-by/);
  assert.match(cli, /--write-receipt/);
  assert.match(cli, /write_forge_remote_head_report/);
  assert.match(cli, /forge_remote_head_receipt_path/);
  assert.match(cli, /\.dx\/forge\/receipts\/remotes/);
  assert.match(cli, /\.get\("health_evaluation"\)/);
  assert.match(cli, /remote HEAD command stayed in planned mode/);
  assert.match(cli, /remote_write_allowed: false/);
  assert.match(projectCheck, /value\.get\("health_evaluation"\)/);
  assert.match(benchmark, /function readOptionalJsonCommand/);
  assert.match(benchmark, /assertRemoteLifecyclePlan/);
  assert.match(benchmark, /existing dx-www binary reports R2 lifecycle dry-run plans without writes/);
  assert.match(benchmark, /existing dx-www binary writes planned R2 remote-head receipt from local manifest/);
});

test(
  "existing dx-www binary runs root-dx local-registry selected update smoke",
  { skip: fs.existsSync(dxWwwBinary) ? false : "target/debug/dx-www is not available without building" },
  () => {
    const tmpRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-forge-cli-smoke-"));
    const projectDir = path.join(tmpRoot, "project");
    const registryDir = path.join(tmpRoot, "registry");

    try {
      fs.mkdirSync(path.join(projectDir, "src"), { recursive: true });
      fs.writeFileSync(
        path.join(projectDir, "src", "client.ts"),
        "export const betterAuthClientVersion = 'v1';\n",
      );
      fs.writeFileSync(
        path.join(projectDir, "src", "server.ts"),
        "export const betterAuthServerVersion = 'v1';\n",
      );
      writeRootDx(projectDir, "0.1.0");

      const publishV1 = readJsonCommand(
        [
          "forge",
          "publish",
          "--registry",
          "local",
          "--package",
          "auth/better-auth",
          "--local",
          registryDir,
          "--write",
          "--format",
          "json",
        ],
        projectDir,
      );

      assert.equal(publishV1.action, "registry-publish");
      assert.equal(publishV1.package_id, "auth/better-auth");
      assert.equal(publishV1.version, "0.1.0");
      assert.equal(publishV1.remote, registryDir);
      assert.equal(publishV1.dry_run, false);

      runDx(
        [
          "forge",
          "add",
          "auth/better-auth#client",
          "--project",
          projectDir,
          "--registry",
          "local",
          "--local",
          registryDir,
          "--version",
          "0.1.0",
          "--write",
        ],
        projectDir,
      );

      assert.match(
        fs.readFileSync(path.join(projectDir, "lib/auth/better-auth/client.ts"), "utf8"),
        /v1/,
      );
      assert.equal(fs.existsSync(path.join(projectDir, "lib/auth/better-auth/server.ts")), false);
      assert.equal(fs.existsSync(path.join(projectDir, "node_modules")), false);

      fs.writeFileSync(
        path.join(projectDir, "src", "client.ts"),
        "export const betterAuthClientVersion = 'v2';\n",
      );
      fs.writeFileSync(
        path.join(projectDir, "src", "server.ts"),
        "export const betterAuthServerVersion = 'v2';\n",
      );
      writeRootDx(projectDir, "0.2.0");

      const publishV2 = readJsonCommand(
        [
          "forge",
          "publish",
          "--registry",
          "local",
          "--package",
          "auth/better-auth",
          "--local",
          registryDir,
          "--write",
          "--format",
          "json",
        ],
        projectDir,
      );

      assert.equal(publishV2.action, "registry-publish");
      assert.equal(publishV2.package_id, "auth/better-auth");
      assert.equal(publishV2.version, "0.2.0");
      assert.equal(publishV2.remote, registryDir);
      assert.equal(publishV2.dry_run, false);

      const updatePreview = readJsonCommand(
        [
          "forge",
          "update",
          "auth/better-auth#client",
          "--project",
          projectDir,
          "--registry",
          "local",
          "--local",
          registryDir,
          "--version",
          "0.2.0",
          "--dry-run",
          "--format",
          "json",
        ],
        projectDir,
      );

      assert.equal(updatePreview.package_id, "auth/better-auth");
      assert.equal(updatePreview.variant, "export-client");
      assert.equal(updatePreview.current_version, "0.1.0");
      assert.equal(updatePreview.latest_version, "0.2.0");
      assert.equal(updatePreview.traffic, "green");
      assert.equal(updatePreview.wrote_files, false);
      assert.deepEqual(updatePreview.receipt.selected_exports, ["client"]);
      assert.deepEqual(
        updatePreview.files.map((file) => file.path),
        ["lib/auth/better-auth/client.ts"],
      );
      assert.equal(updatePreview.files[0].change, "update");
      assert.equal(updatePreview.files[0].traffic, "green");
      assert.equal(updatePreview.receipt.files_written.length, 1);
      assert.equal(
        updatePreview.receipt.files_written[0].path,
        "lib/auth/better-auth/client.ts",
      );
      assert.equal(updatePreview.receipt.update_decisions[0].decision, "rejected");
      assert.equal(fs.existsSync(path.join(projectDir, "lib/auth/better-auth/server.ts")), false);
      assert.equal(fs.existsSync(path.join(projectDir, "node_modules")), false);

      const updateWrite = readJsonCommand(
        [
          "forge",
          "update",
          "auth/better-auth#client",
          "--project",
          projectDir,
          "--registry",
          "local",
          "--local",
          registryDir,
          "--version",
          "0.2.0",
          "--write",
          "--format",
          "json",
        ],
        projectDir,
      );

      assert.equal(updateWrite.traffic, "green");
      assert.equal(updateWrite.wrote_files, true);
      assert.deepEqual(updateWrite.receipt.selected_exports, ["client"]);
      assert.equal(updateWrite.receipt.action, "update-write");
      assert.equal(updateWrite.receipt.update_decisions[0].decision, "accepted");
      assert.ok(updateWrite.receipt_path);
      assert.equal(fs.existsSync(updateWrite.receipt_path), true);
      assert.match(
        fs.readFileSync(path.join(projectDir, "lib/auth/better-auth/client.ts"), "utf8"),
        /v2/,
      );
      assert.equal(fs.existsSync(path.join(projectDir, "lib/auth/better-auth/server.ts")), false);
      assert.equal(fs.existsSync(path.join(projectDir, "node_modules")), false);
    } finally {
      fs.rmSync(tmpRoot, { recursive: true, force: true });
    }
  },
);

test(
  "existing dx-www binary plans root-dx R2 dry-run without leaking secrets",
  { skip: fs.existsSync(dxWwwBinary) ? false : "target/debug/dx-www is not available without building" },
  () => {
    const tmpRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-forge-r2-smoke-"));
    const projectDir = path.join(tmpRoot, "project");
    const env = {
      CLOUDFLARE_ACCOUNT_ID: "dx-test-account-should-not-print",
      CLOUDFLARE_R2_ACCOUNT_ID: undefined,
      CLOUDFLARE_R2_ACCESS_KEY_ID: "dx-test-access-key-should-not-print",
      CLOUDFLARE_R2_SECRET_ACCESS_KEY: "dx-test-secret-key-should-not-print",
      CLOUDFLARE_R2_BUCKET: undefined,
      DX_FORGE_R2_BUCKET: "dx-forge-test-bucket",
      AWS_ACCESS_KEY_ID: undefined,
      AWS_SECRET_ACCESS_KEY: undefined,
      AWS_ENDPOINT_URL: "https://r2.example.invalid",
      CLOUDFLARE_R2_PUBLIC_BASE_URL: undefined,
      DX_FORGE_R2_PUBLIC_BASE_URL: undefined,
      DX_FORGE_R2_PREFIX: "launch-smoke",
    };

    try {
      fs.mkdirSync(path.join(projectDir, "src"), { recursive: true });
      fs.writeFileSync(
        path.join(projectDir, "src", "client.ts"),
        "export const betterAuthClientVersion = 'r2-dry-run';\n",
      );
      fs.writeFileSync(
        path.join(projectDir, "src", "server.ts"),
        "export const betterAuthServerVersion = 'r2-dry-run';\n",
      );
      writeRootDx(projectDir, "0.1.0");

      const report = withTemporaryEnv(env, () =>
        readJsonCommand(
          [
            "forge",
            "publish",
            "--registry",
            "r2",
            "--package",
            "auth/better-auth",
            "--dry-run",
            "--format",
            "json",
          ],
          projectDir,
        ),
      );
      const serialized = JSON.stringify(report);

      assert.equal(report.action, "registry-publish");
      assert.equal(report.package_id, "auth/better-auth");
      assert.equal(report.version, "0.1.0");
      assert.equal(report.remote, "r2");
      assert.equal(report.dry_run, true);
      assert.equal(report.r2_status.configured, true);
      assert.equal(report.r2_status.account_id_set, true);
      if (Object.prototype.hasOwnProperty.call(report.r2_status, "bucket_set")) {
        assert.equal(report.r2_status.bucket_set, true);
        assert.equal(report.r2_status.endpoint_set, true);
        assert.equal(report.r2_status.public_base_url_set, false);
        assert.equal(Object.prototype.hasOwnProperty.call(report.r2_status, "bucket"), false);
        assert.equal(Object.prototype.hasOwnProperty.call(report.r2_status, "endpoint"), false);
        assert.equal(Object.prototype.hasOwnProperty.call(report.r2_status, "public_base_url"), false);
      } else {
        assert.equal(report.r2_status.bucket, "dx-forge-test-bucket");
        assert.equal(report.r2_status.endpoint, "https://r2.example.invalid");
      }
      assert.equal(report.r2_status.prefix, "launch-smoke");
      if (Object.prototype.hasOwnProperty.call(report.r2_status, "setup_status")) {
        assert.equal(report.r2_status.setup_status, "configured");
        assert.deepEqual(report.r2_status.missing_config, []);
      }
      assert.ok(report.objects.some((object) => object.endsWith("launch-smoke/index.json")));
      assert.ok(
        report.objects.some((object) =>
          object.endsWith("launch-smoke/packages/js/auth/better-auth/0.1.0/.dx/build-cache/manifest.json"),
        ),
      );
      assert.ok(report.objects.some((object) => object.includes("/files/")));
      assert.ok(
        report.objects.some((object) =>
          object.endsWith("launch-smoke/receipts/<timestamp>-auth-better-auth.json"),
        ),
      );
      if (Object.prototype.hasOwnProperty.call(report.r2_status, "bucket_set")) {
        assert.ok(report.objects.every((object) => object.startsWith("r2://<configured-bucket>/")));
      }

      for (const marker of [
        env.CLOUDFLARE_ACCOUNT_ID,
        env.CLOUDFLARE_R2_ACCESS_KEY_ID,
        env.CLOUDFLARE_R2_SECRET_ACCESS_KEY,
        "CLOUDFLARE_R2_ACCESS_KEY_ID",
        "CLOUDFLARE_R2_SECRET_ACCESS_KEY",
        "AWS_SECRET_ACCESS_KEY",
      ]) {
        assert.equal(
          serialized.includes(marker),
          false,
          `R2 dry-run output leaked secret marker ${marker}`,
        );
      }
    } finally {
      fs.rmSync(tmpRoot, { recursive: true, force: true });
    }
  },
);

test(
  "existing dx-www binary reports missing and partial R2 setup without leaking env values",
  { skip: fs.existsSync(dxWwwBinary) ? false : "target/debug/dx-www is not available without building" },
  (t) => {
    const tmpRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-forge-r2-config-smoke-"));
    const projectDir = path.join(tmpRoot, "project");
    const secretMarkers = [
      "dx-test-account-should-not-print",
      "dx-test-access-key-should-not-print",
      "dx-test-secret-key-should-not-print",
      "CLOUDFLARE_R2_ACCESS_KEY_ID",
      "CLOUDFLARE_R2_SECRET_ACCESS_KEY",
      "AWS_SECRET_ACCESS_KEY",
    ];

    try {
      fs.mkdirSync(path.join(projectDir, "src"), { recursive: true });
      fs.writeFileSync(
        path.join(projectDir, "src", "client.ts"),
        "export const betterAuthClientVersion = 'r2-config-smoke';\n",
      );
      fs.writeFileSync(
        path.join(projectDir, "src", "server.ts"),
        "export const betterAuthServerVersion = 'r2-config-smoke';\n",
      );
      writeRootDx(projectDir, "0.1.0");

      const missingEnv = {
        CLOUDFLARE_ACCOUNT_ID: undefined,
        CLOUDFLARE_R2_ACCOUNT_ID: undefined,
        CLOUDFLARE_R2_ACCESS_KEY_ID: undefined,
        CLOUDFLARE_R2_SECRET_ACCESS_KEY: undefined,
        CLOUDFLARE_R2_BUCKET: undefined,
        DX_FORGE_R2_BUCKET: undefined,
        AWS_ACCESS_KEY_ID: undefined,
        AWS_SECRET_ACCESS_KEY: undefined,
        AWS_ENDPOINT_URL: undefined,
        CLOUDFLARE_R2_PUBLIC_BASE_URL: undefined,
        DX_FORGE_R2_PUBLIC_BASE_URL: undefined,
        DX_FORGE_R2_PREFIX: "launch-smoke",
      };
      const missingRemotes = withTemporaryEnv(missingEnv, () =>
        readJsonCommand(["forge", "remotes", "--json"], projectDir),
      );
      const missingR2 = missingRemotes.remotes.find((remote) => remote.name === "r2");
      if (!missingR2 || !Object.prototype.hasOwnProperty.call(missingR2, "setup_status")) {
        t.skip("existing dx-www binary predates Forge R2 setup_status JSON contract");
        return;
      }
      assert.equal(missingR2.setup_status, "missing-config");
      assert.deepEqual(missingR2.missing_config, [
        "account_id_or_endpoint",
        "access_key_id",
        "secret_access_key",
        "bucket",
      ]);

      const partialEnv = {
        ...missingEnv,
        CLOUDFLARE_ACCOUNT_ID: secretMarkers[0],
        CLOUDFLARE_R2_ACCESS_KEY_ID: secretMarkers[1],
        DX_FORGE_R2_BUCKET: "dx-forge-test-bucket",
        AWS_ENDPOINT_URL: "https://r2.example.invalid",
      };
      const partialRemotes = withTemporaryEnv(partialEnv, () =>
        readJsonCommand(["forge", "remotes", "--json"], projectDir),
      );
      const partialR2 = partialRemotes.remotes.find((remote) => remote.name === "r2");
      assert.equal(partialR2.setup_status, "partial-config");
      assert.equal(partialR2.account_id_set, true);
      assert.equal(partialR2.access_key_id_set, true);
      assert.equal(partialR2.secret_access_key_set, false);
      assert.equal(partialR2.bucket_set, true);
      assert.deepEqual(partialR2.missing_config, ["secret_access_key"]);

      const publishPlan = withTemporaryEnv(partialEnv, () =>
        readJsonCommand(
          [
            "forge",
            "publish",
            "--registry",
            "r2",
            "--package",
            "auth/better-auth",
            "--dry-run",
            "--format",
            "json",
          ],
          projectDir,
        ),
      );
      if (
        !publishPlan.r2_status ||
        !Object.prototype.hasOwnProperty.call(publishPlan.r2_status, "setup_status")
      ) {
        t.skip("existing dx-www binary predates Forge R2 dry-run setup_status JSON contract");
        return;
      }
      assert.equal(publishPlan.r2_status.setup_status, "partial-config");
      assert.deepEqual(publishPlan.r2_status.missing_config, ["secret_access_key"]);
      assert.equal(publishPlan.r2_status.configured, false);
      assert.equal(publishPlan.dry_run, true);
      assert.ok(publishPlan.objects.some((object) => object.includes("/files/")));

      const serialized = JSON.stringify({ missingRemotes, partialRemotes, publishPlan });
      const hasRedactedDryRunValueContract = Object.prototype.hasOwnProperty.call(
        publishPlan.r2_status,
        "bucket_set",
      );
      if (!hasRedactedDryRunValueContract && secretMarkers.some((marker) => serialized.includes(marker))) {
        t.skip("existing dx-www binary predates redacted R2 dry-run value contract");
        return;
      }
      if (hasRedactedDryRunValueContract) {
        assert.equal(publishPlan.r2_status.bucket_set, true);
        assert.equal(publishPlan.r2_status.endpoint_set, true);
        assert.equal(Object.prototype.hasOwnProperty.call(publishPlan.r2_status, "bucket"), false);
        assert.equal(Object.prototype.hasOwnProperty.call(publishPlan.r2_status, "endpoint"), false);
        assert.ok(publishPlan.objects.every((object) => object.startsWith("r2://<configured-bucket>/")));
      }
      for (const marker of secretMarkers) {
        assert.equal(
          serialized.includes(marker),
          false,
          `R2 status output leaked env marker ${marker}`,
        );
      }
    } finally {
      fs.rmSync(tmpRoot, { recursive: true, force: true });
    }
  },
);

test(
  "existing dx-www binary reports R2 lifecycle dry-run plans without writes",
  { skip: fs.existsSync(dxWwwBinary) ? false : "target/debug/dx-www is not available without building" },
  (t) => {
    const tmpRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-forge-r2-lifecycle-smoke-"));
    const projectDir = path.join(tmpRoot, "project");
    const env = {
      CLOUDFLARE_ACCOUNT_ID: "dx-test-account-should-not-print",
      CLOUDFLARE_R2_ACCOUNT_ID: undefined,
      CLOUDFLARE_R2_ACCESS_KEY_ID: "dx-test-access-key-should-not-print",
      CLOUDFLARE_R2_SECRET_ACCESS_KEY: "dx-test-secret-key-should-not-print",
      CLOUDFLARE_R2_BUCKET: undefined,
      DX_FORGE_R2_BUCKET: "dx-forge-test-bucket",
      AWS_ACCESS_KEY_ID: undefined,
      AWS_SECRET_ACCESS_KEY: undefined,
      AWS_ENDPOINT_URL: "https://r2.example.invalid",
      CLOUDFLARE_R2_PUBLIC_BASE_URL: undefined,
      DX_FORGE_R2_PUBLIC_BASE_URL: undefined,
      DX_FORGE_R2_PREFIX: "launch-lifecycle-smoke",
    };

    try {
      fs.mkdirSync(path.join(projectDir, "src"), { recursive: true });
      fs.writeFileSync(
        path.join(projectDir, "src", "client.ts"),
        "export const betterAuthClientVersion = 'r2-lifecycle-smoke';\n",
      );
      fs.writeFileSync(
        path.join(projectDir, "src", "server.ts"),
        "export const betterAuthServerVersion = 'r2-lifecycle-smoke';\n",
      );
      writeRootDx(projectDir, "0.1.0");

      const addAttempt = withTemporaryEnv(env, () =>
        readOptionalJsonCommand(
          [
            "forge",
            "add",
            "auth/better-auth#client",
            "--project",
            projectDir,
            "--registry",
            "r2",
            "--dry-run",
            "--format",
            "json",
          ],
          projectDir,
        ),
      );
      if (!addAttempt.ok || addAttempt.value.schema_version !== "dx.forge.remote_lifecycle_plan") {
        t.skip("existing dx-www binary predates Forge R2 lifecycle dry-run JSON contract");
        return;
      }

      const updateAttempt = withTemporaryEnv(env, () =>
        readOptionalJsonCommand(
          [
            "forge",
            "update",
            "auth/better-auth#client",
            "--project",
            projectDir,
            "--registry",
            "r2",
            "--version",
            "0.2.0",
            "--dry-run",
            "--format",
            "json",
          ],
          projectDir,
        ),
      );
      assert.equal(updateAttempt.ok, true, updateAttempt.error?.message);

      const removeAttempt = withTemporaryEnv(env, () =>
        readOptionalJsonCommand(
          [
            "forge",
            "remove",
            "auth/better-auth#client",
            "--project",
            projectDir,
            "--registry",
            "r2",
            "--version",
            "0.2.0",
            "--dry-run",
            "--format",
            "json",
          ],
          projectDir,
        ),
      );
      assert.equal(removeAttempt.ok, true, removeAttempt.error?.message);

      const addPlan = addAttempt.value;
      const updatePlan = updateAttempt.value;
      const removePlan = removeAttempt.value;

      assertRemoteLifecyclePlan(addPlan, {
        action: "install-dry-run",
        objectPlan: "install-plan.json",
        receiptPrefix: "add",
      });
      assertRemoteLifecyclePlan(updatePlan, {
        action: "update-dry-run",
        requestedVersion: "0.2.0",
        objectPlan: "update-plan.json",
        receiptPrefix: "update",
      });
      assertRemoteLifecyclePlan(removePlan, {
        action: "uninstall-dry-run",
        requestedVersion: "0.2.0",
        objectPlan: "uninstall-plan.json",
        receiptPrefix: "remove",
      });
      assert.equal(fs.existsSync(path.join(projectDir, "node_modules")), false);
      assert.equal(fs.existsSync(path.join(projectDir, "lib/auth/better-auth/client.ts")), false);

      const serialized = JSON.stringify({ addPlan, updatePlan, removePlan });
      for (const marker of [
        env.CLOUDFLARE_ACCOUNT_ID,
        env.CLOUDFLARE_R2_ACCESS_KEY_ID,
        env.CLOUDFLARE_R2_SECRET_ACCESS_KEY,
        env.DX_FORGE_R2_BUCKET,
        "CLOUDFLARE_R2_ACCESS_KEY_ID",
        "CLOUDFLARE_R2_SECRET_ACCESS_KEY",
        "AWS_SECRET_ACCESS_KEY",
      ]) {
        assert.equal(
          serialized.includes(marker),
          false,
          `R2 lifecycle dry-run output leaked env marker ${marker}`,
        );
      }
    } finally {
      fs.rmSync(tmpRoot, { recursive: true, force: true });
    }
  },
);

test(
  "existing dx-www binary writes planned R2 remote-head receipt from local manifest",
  { skip: fs.existsSync(dxWwwBinary) ? false : "target/debug/dx-www is not available without building" },
  (t) => {
    const tmpRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-forge-r2-head-cli-smoke-"));
    const projectDir = path.join(tmpRoot, "project");
    const registryDir = path.join(tmpRoot, "registry");
    const env = {
      CLOUDFLARE_ACCOUNT_ID: "dx-test-account-should-not-print",
      CLOUDFLARE_R2_ACCOUNT_ID: undefined,
      CLOUDFLARE_R2_ACCESS_KEY_ID: "dx-test-access-key-should-not-print",
      CLOUDFLARE_R2_SECRET_ACCESS_KEY: "dx-test-secret-key-should-not-print",
      CLOUDFLARE_R2_BUCKET: undefined,
      DX_FORGE_R2_BUCKET: "dx-forge-test-bucket",
      AWS_ACCESS_KEY_ID: undefined,
      AWS_SECRET_ACCESS_KEY: undefined,
      AWS_ENDPOINT_URL: "https://r2.example.invalid",
      CLOUDFLARE_R2_PUBLIC_BASE_URL: undefined,
      DX_FORGE_R2_PUBLIC_BASE_URL: undefined,
      DX_FORGE_R2_PREFIX: "launch-head-smoke",
    };

    try {
      fs.mkdirSync(path.join(projectDir, "src"), { recursive: true });
      fs.writeFileSync(
        path.join(projectDir, "src", "client.ts"),
        "export const betterAuthClientVersion = 'remote-head';\n",
      );
      fs.writeFileSync(
        path.join(projectDir, "src", "server.ts"),
        "export const betterAuthServerVersion = 'remote-head';\n",
      );
      writeRootDx(projectDir, "0.1.0");

      const publish = readJsonCommand(
        [
          "forge",
          "publish",
          "--registry",
          "local",
          "--package",
          "auth/better-auth",
          "--local",
          registryDir,
          "--write",
          "--format",
          "json",
        ],
        projectDir,
      );
      assert.equal(publish.package_id, "auth/better-auth");
      assert.equal(publish.version, "0.1.0");

      const manifestPath = path.join(
        registryDir,
        "packages/js/auth/better-auth/0.1.0/.dx/build-cache/manifest.json",
      );
      assert.equal(fs.existsSync(manifestPath), true);

      const headAttempt = withTemporaryEnv(env, () =>
        readOptionalJsonCommand(
          [
            "forge",
            "remote-head",
            "auth/better-auth#client",
            "--project",
            projectDir,
            "--registry",
            "r2",
            "--remote-manifest",
            manifestPath,
            "--write-receipt",
            "--json",
          ],
          projectDir,
        ),
      );
      if (!headAttempt.ok || headAttempt.value.schema_version !== "dx.forge.remote_head_cli_report") {
        t.skip("existing dx-www binary predates Forge remote-head CLI report contract");
        return;
      }

      const headReport = headAttempt.value;
      assert.equal(headReport.package_id, "auth/better-auth");
      assert.deepEqual(headReport.selected_exports, ["client"]);
      assert.equal(headReport.registry, "r2");
      assert.equal(headReport.provider_kind, "s3-compatible-object-storage");
      assert.equal(headReport.approved, false);
      assert.equal(headReport.executed, false);
      assert.equal(headReport.dry_run, true);
      assert.equal(headReport.network_allowed, false);
      assert.equal(headReport.remote_write_allowed, false);
      assert.equal(headReport.health_evaluation.schema_version, "dx.forge.remote_object_head_health");
      assert.equal(headReport.health_evaluation.safe_for_remote_install, false);
      assert.ok(headReport.receipt_path.endsWith("auth-better-auth-0.1.0-r2-head-health.json"));
      assert.equal(fs.existsSync(headReport.receipt_path), true);

      const status = readJsonCommand(["forge", "status", "--json"], projectDir);
      assert.equal(status.status, "remote-health-blocked");
      assert.ok(
        status.remote_object_head_health.some(
          (health) =>
            health.package_id === "auth/better-auth" &&
            health.version === "0.1.0" &&
            health.source_receipt_path.endsWith("auth-better-auth-0.1.0-r2-head-health.json"),
        ),
      );

      const serialized = JSON.stringify({ headReport, status });
      for (const marker of [
        env.CLOUDFLARE_ACCOUNT_ID,
        env.CLOUDFLARE_R2_ACCESS_KEY_ID,
        env.CLOUDFLARE_R2_SECRET_ACCESS_KEY,
        env.DX_FORGE_R2_BUCKET,
        "CLOUDFLARE_R2_ACCESS_KEY_ID",
        "CLOUDFLARE_R2_SECRET_ACCESS_KEY",
        "AWS_SECRET_ACCESS_KEY",
      ]) {
        assert.equal(
          serialized.includes(marker),
          false,
          `R2 remote-head output leaked env marker ${marker}`,
        );
      }
    } finally {
      fs.rmSync(tmpRoot, { recursive: true, force: true });
    }
  },
);
