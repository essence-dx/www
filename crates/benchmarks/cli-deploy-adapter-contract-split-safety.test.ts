import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const cliCorePath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "mod_parts",
  "cli_core_impl.rs",
);
const deployAdapterPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "deploy_adapter_contract.rs",
);
const serverActionRuntimePath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "server_action_runtime.rs",
);

function sliceRustMethod(source: string, signature: string): string {
  const start = source.indexOf(signature);
  assert.notEqual(start, -1, `expected to find ${signature}`);
  const nextMethod = source.indexOf("\n    pub fn ", start + signature.length);
  return source.slice(start, nextMethod === -1 ? undefined : nextMethod);
}

test("deploy adapter contract generation is owned outside the cli dispatcher", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  const cliCore = fs.readFileSync(cliCorePath, "utf8");
  const deployAdapter = fs.readFileSync(deployAdapterPath, "utf8");
  const serverActionRuntime = fs.readFileSync(serverActionRuntimePath, "utf8");
  const cmdBuild = sliceRustMethod(cliCore, "pub fn cmd_build(&self) -> DxResult<()>");

  assert.match(cliMod, /^mod deploy_adapter_contract;$/m);
  assert.match(
    cmdBuild,
    /deploy_adapter_contract::write_deploy_adapter_contract\(\s*&self\.cwd,\s*&output_dir,\s*&server_sources,\s*manifest_json,\s*\)/,
  );
  assert.doesNotMatch(`${cliMod}\n${cliCore}`, /fn write_deploy_adapter_contract\(/);
  assert.doesNotMatch(`${cliMod}\n${cliCore}`, /fn write_provider_adapter_fixture\(/);
  assert.doesNotMatch(`${cliMod}\n${cliCore}`, /fn provider_adapter_upload_plan\(/);
  assert.doesNotMatch(`${cliMod}\n${cliCore}`, /const DX_CLOUD_PROVIDER_ADAPTER_JSON/);

  assert.match(
    deployAdapter,
    /pub\(super\) fn write_deploy_adapter_contract\(\s*project_dir: &Path,\s*output_dir: &Path,\s*server_sources: &\[DxReactServerSource\],\s*manifest_json: &str,\s*\) -> DxResult<\(\)>/,
  );
  assert.match(deployAdapter, /"no_node_modules_required": true/);
  assert.match(deployAdapter, /if !path\.contains\("node_modules"\)/);
  assert.match(
    deployAdapter,
    /fn deploy_artifact_bundle\(path: &str, requested_bundle: &str\) -> &'static str/,
  );
  assert.match(
    deployAdapter,
    /fn deploy_artifact_evidence_only_path\(path: &str\) -> bool/,
  );
  assert.match(
    deployAdapter,
    /fn deploy_artifact_cache_control\(requested_cache_control: &str, bundle: &str\) -> String/,
  );
  assert.match(deployAdapter, /deploy_artifact_bundle\(path, bundle\)/);
  assert.match(deployAdapter, /deploy_artifact_cache_control\(cache_control, bundle\)/);
  assert.match(deployAdapter, /normalized\.starts_with\("\.dx\/"\)/);
  assert.match(deployAdapter, /normalized\.starts_with\("source-routes\/"\)/);
  assert.match(deployAdapter, /normalized\.ends_with\("\.sr"\)/);
  assert.match(deployAdapter, /"lifecycle_scripts_executed": false/);
  assert.match(deployAdapter, /"review_before_materialization"/);
  assert.match(deployAdapter, /write_production_observability_contract\(output_dir, &deploy, &manifest_hash\)/);
  assert.match(deployAdapter, /write_forge_hosting_manifest\(output_dir, &deploy, &manifest_hash\)/);
  assert.match(deployAdapter, /write_hosted_preview_contract\(project_dir, output_dir, &deploy, &manifest_hash\)/);
  assert.match(deployAdapter, /deploy_next_familiar_compatibility_contract\(output_dir\)/);
  assert.match(deployAdapter, /app_route_handler_deploy_metadata\(server_sources\)/);
  assert.match(deployAdapter, /SERVER_ACTION_REPLAY_LEDGER_JSON/);
  assert.match(deployAdapter, /write_server_action_replay_ledger_contract\(/);
  assert.match(deployAdapter, /"server-action-replay-ledger"/);
  assert.match(deployAdapter, /"replay_ledger": SERVER_ACTION_REPLAY_LEDGER_JSON/);
  assert.match(serverActionRuntime, /pub\(super\) const SERVER_ACTION_REPLAY_LEDGER_JSON/);
  assert.match(serverActionRuntime, /"local-preview-hash-ledger"/);
  assert.match(serverActionRuntime, /"distributed": false/);
  assert.match(serverActionRuntime, /"provider_hosted": false/);
  assert.match(serverActionRuntime, /raw payload, session, csrf, and idempotency values are not persisted/);
});
