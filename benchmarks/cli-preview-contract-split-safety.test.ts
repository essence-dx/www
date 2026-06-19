import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const cliCoreImplPath = path.join(repoRoot, "dx-www", "src", "cli", "mod_parts", "cli_core_impl.rs");
const previewContractPath = path.join(repoRoot, "dx-www", "src", "cli", "preview_contract.rs");
const serverActionRuntimePath = path.join(repoRoot, "dx-www", "src", "cli", "server_action_runtime.rs");

test("dx preview production-contract wire response lives outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  const cliCoreImpl = fs.readFileSync(cliCoreImplPath, "utf8");

  assert.ok(
    fs.existsSync(previewContractPath),
    "expected dx-www/src/cli/preview_contract.rs",
  );
  assert.ok(
    fs.existsSync(serverActionRuntimePath),
    "expected dx-www/src/cli/server_action_runtime.rs",
  );

  const previewContract = fs.readFileSync(previewContractPath, "utf8");
  const serverActionRuntime = fs.readFileSync(serverActionRuntimePath, "utf8");

  assert.match(cliMod, /^mod preview_contract;$/m);
  assert.match(cliMod, /^mod server_action_runtime;$/m);
  assert.match(
    cliCoreImpl,
    /preview_command::cmd_preview\(\s*&self\.cwd,\s*args,\s*server_action_runtime::execute_production_contract_server_action,\s*\)/,
  );
  assert.doesNotMatch(cliMod, /\bfn production_contract_wire_response\b/);
  assert.doesNotMatch(cliMod, /\bfn handle_production_contract_http_request\b/);
  assert.doesNotMatch(cliMod, /\bfn execute_production_contract_server_action\b/);
  assert.doesNotMatch(cliMod, /\bfn execute_server_action_endpoint\b/);
  assert.doesNotMatch(cliMod, /\bfn read_server_action_protocols\b/);
  assert.doesNotMatch(cliMod, /\bfn read_server_action_runtime_sources\b/);
  assert.doesNotMatch(cliMod, /\bstruct DxProductionPreviewResponse\b/);
  assert.doesNotMatch(cliMod, /Content-Length: \{\}\\r\\nConnection: close\\r\\n\\r\\n/);

  assert.match(previewContract, /pub\(super\) struct DxProductionPreviewResponse/);
  assert.match(previewContract, /pub\(super\) fn production_contract_wire_response\(/);
  assert.match(previewContract, /execute_server_action: DxProductionServerActionExecutor/);
  assert.match(
    previewContract,
    /handle_production_contract_http_request\(build_dir, request, execute_server_action\)/,
  );
  assert.match(previewContract, /500 Internal Server Error/);
  assert.match(previewContract, /Content-Length: \{\}\\r\\nConnection: \{\}\\r\\n\\r\\n/);
  assert.match(previewContract, /production_contract_wire_response_cached_with_connection/);

  assert.doesNotMatch(cliMod, /\bfn read_deploy_adapter_contract\b/);
  assert.doesNotMatch(cliMod, /\bfn normalize_preview_contract_path\b/);
  assert.doesNotMatch(cliMod, /\bfn production_contract_content_type\b/);
  assert.doesNotMatch(cliMod, /\bfn safe_production_contract_file\b/);
  assert.doesNotMatch(cliMod, /\bfn production_contract_health_response\b/);
  assert.doesNotMatch(cliMod, /\bfn production_contract_server_action_response\b/);
  assert.doesNotMatch(cliMod, /contract\["server_actions"\]/);

  const devHttpImport = previewContract.match(/use super::dev_http::\{([^}]+)\};/);
  assert.ok(devHttpImport, "preview contract should import dev HTTP helpers");
  assert.match(devHttpImport[1], /\bDxCliHttpRequest\b/);
  assert.match(devHttpImport[1], /\bparse_http_request\b/);
  assert.match(previewContract, /pub\(super\) type DxProductionServerActionExecutor =/);
  assert.match(previewContract, /pub\(super\) fn read_deploy_adapter_contract\(/);
  assert.match(previewContract, /pub\(super\) fn handle_production_contract_http_request\(/);
  assert.match(previewContract, /let request = parse_http_request\(request\);/);
  assert.match(previewContract, /pub\(super\) fn handle_production_contract_request\(/);
  assert.match(previewContract, /fn production_contract_server_action_response\(/);
  assert.match(previewContract, /contract\["server_actions"\]/);
  assert.match(
    previewContract,
    /execute_server_action\(build_dir, contract, request, action_id\)/,
  );
  assert.match(previewContract, /pub\(super\) fn normalize_preview_contract_path\(/);
  assert.match(previewContract, /fn production_contract_content_type\(/);
  assert.match(previewContract, /fn safe_production_contract_file\(/);
  assert.match(previewContract, /fn production_contract_health_response\(/);

  assert.match(
    serverActionRuntime,
    /pub\(super\) fn execute_production_contract_server_action\(/,
  );
  assert.match(serverActionRuntime, /fn execute_server_action_endpoint\(/);
  assert.match(serverActionRuntime, /fn read_server_action_protocols\(/);
  assert.match(serverActionRuntime, /fn read_server_action_runtime_sources\(/);
  assert.match(serverActionRuntime, /execute_react_server_action/);
});
