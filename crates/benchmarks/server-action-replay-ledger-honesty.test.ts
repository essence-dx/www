import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function read(relativePath: string) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("server-action replay ledger keeps hosted-provider proof boundary explicit", () => {
  const runtime = read("dx-www/src/cli/server_action_runtime.rs");
  const readiness = read("dx-www/src/cli/readiness.rs");
  const agentContext = read("dx-www/src/cli/agent_context.rs");

  assert.match(runtime, /"mode": "local-preview-hash-ledger"/);
  assert.match(runtime, /"release_ready": false/);
  assert.match(runtime, /"provider_hosted": false/);
  assert.match(runtime, /"hosted_provider_proof": false/);
  assert.match(runtime, /"provider_proof_status": "not-run-local-preview-only"/);
  assert.match(runtime, /"production_proof_scope": "local-production-preview-only"/);
  assert.match(runtime, /"provider_hosted_replay_required": true/);
  assert.match(runtime, /SERVER_ACTION_REPLAY_LEDGER_PROVIDER_PROOF_GAP_IDS/);
  assert.match(runtime, /SERVER_ACTION_REPLAY_LEDGER_RECEIPT_HINT_FIELDS/);
  assert.match(runtime, /SERVER_ACTION_REPLAY_LEDGER_LOCAL_REPLAY_HINT_STEPS/);
  assert.match(runtime, /"provider_proof_gap_ids": SERVER_ACTION_REPLAY_LEDGER_PROVIDER_PROOF_GAP_IDS/);
  assert.match(runtime, /"receipt_hint_fields": SERVER_ACTION_REPLAY_LEDGER_RECEIPT_HINT_FIELDS/);
  assert.match(runtime, /"local_replay_hint_steps": SERVER_ACTION_REPLAY_LEDGER_LOCAL_REPLAY_HINT_STEPS/);
  assert.match(
    runtime,
    /ledger\["provider_proof_gap_ids"\]\s*=\s*serde_json::json!\(SERVER_ACTION_REPLAY_LEDGER_PROVIDER_PROOF_GAP_IDS\)/,
  );
  assert.match(
    runtime,
    /ledger\["receipt_hint_fields"\]\s*=\s*serde_json::json!\(SERVER_ACTION_REPLAY_LEDGER_RECEIPT_HINT_FIELDS\)/,
  );
  assert.match(
    runtime,
    /ledger\["local_replay_hint_steps"\]\s*=\s*serde_json::json!\(SERVER_ACTION_REPLAY_LEDGER_LOCAL_REPLAY_HINT_STEPS\)/,
  );
  assert.match(runtime, /dx preview --production/);
  assert.match(runtime, /POST the compiled server action endpoint/);
  assert.match(runtime, /not hosted or distributed provider proof/);
  assert.match(runtime, /"distributed-idempotency-store"/);
  assert.match(runtime, /"provider-hosted-csrf-session-integration"/);
  assert.match(runtime, /"cross-process-replay-consistency"/);
  assert.match(runtime, /"durable-provider-kv-sql-replay-retention"/);
  assert.match(runtime, /"provider-request-cancellation-replay"/);
  assert.match(runtime, /provider request cancellation replay/);
  assert.match(runtime, /"entries\[\]\.receipt_id"/);
  assert.match(runtime, /"entries\[\]\.replay_key_hash"/);
  assert.match(runtime, /"entries\[\]\.payload_hash"/);
  assert.match(runtime, /"entries\[\]\.response_hash"/);
  assert.match(runtime, /"last_recorded_unix_ms"/);
  assert.match(runtime, /"conflict_count": conflict_count/);
  assert.match(runtime, /"duplicate_replay_count": duplicate_replay_count/);
  assert.match(
    runtime,
    /same action\/session\/idempotency with different payload is recorded as a local conflict/,
  );
  assert.match(
    runtime,
    /This ledger is local production-preview evidence only; it is not a distributed or provider-hosted idempotency store\./,
  );

  assert.match(readiness, /dx\.www\.readiness\.server_action_replay_ledger_receipt_contract/);
  assert.match(readiness, /\.dx\/receipts\/readiness\/server-action-replay-ledger-latest\.json/);
  assert.match(readiness, /\.dx\/receipts\/readiness\/server-action-replay-ledger-latest\.sr/);
  assert.match(
    readiness,
    /\.dx\/serializer\/receipts-readiness-server-action-replay-ledger-latest\.machine/,
  );
  assert.match(readiness, /write_readiness_server_action_replay_ledger_receipt/);
  assert.match(readiness, /server_action_replay_ledger_receipt_is_current/);
  assert.match(readiness, /readiness_server_action_replay_ledger_stale_reason/);
  assert.match(readiness, /server_action_replay_ledger_stale_reason_from_receipt/);
  assert.match(readiness, /server_action_replay_ledger_stale_reason/);
  assert.match(readiness, /server-action-replay-ledger-receipt-missing/);
  assert.match(readiness, /server-action-replay-ledger-proof-boundary-invalid/);
  assert.match(readiness, /server-action-provider-distributed-proof-missing/);
  assert.match(readiness, /READINESS_SERVER_ACTION_PROVIDER_GAP_IDS/);
  assert.match(readiness, /server_action_replay_ledger_has_provider_gap_ids/);
  assert.match(readiness, /"provider-request-cancellation-replay"/);
  assert.match(readiness, /"provider_proof_gap_ids": ledger\.as_ref\(\)/);
  assert.match(readiness, /"stale_reason": server_action_replay_ledger_stale_reason/);
  assert.match(readiness, /local-replay-ledger-current-provider-proof-needed/);
  assert.match(readiness, /hosted provider proof remains separate/);
  assert.match(readiness, /server_action_replay_ledger_current/);
  assert.match(readiness, /refresh_readiness_local_read_models/);
  assert.match(
    readiness,
    /write_readiness_local_receipts\(&project_root\)\?;[\s\S]*refresh_readiness_local_read_models\(&mut report, &project_root\)/,
  );

  assert.match(agentContext, /READINESS_SERVER_ACTION_PROVIDER_GAP_IDS/);
  assert.match(agentContext, /remaining_provider_gap_ids/);
  assert.match(agentContext, /provider_proof_gap_ids/);
  assert.match(agentContext, /missing-provider-proof-gap-/);
});
