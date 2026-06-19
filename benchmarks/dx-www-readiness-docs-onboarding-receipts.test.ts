import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");

function read(relativePath: string): string {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function expectAll(source: string, markers: string[]): void {
  for (const marker of markers) {
    assert.match(source, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
}

test("release-readiness docs/onboarding receipt is source-owned, serializer-backed, and honest", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");

  expectAll(readiness, [
    "READINESS_DOCS_ONBOARDING_SCHEMA",
    "dx.www.readiness.docs_onboarding",
    "READINESS_DOCS_ONBOARDING_RECEIPT_CONTRACT",
    "dx.www.readiness.docs_onboarding_receipt_contract",
    "READINESS_DOCS_ONBOARDING_RECEIPT",
    ".dx/receipts/readiness/docs-onboarding-latest.json",
    "READINESS_DOCS_ONBOARDING_RECEIPT_SR",
    ".dx/receipts/readiness/docs-onboarding-latest.sr",
    "READINESS_DOCS_ONBOARDING_RECEIPT_MACHINE",
    ".dx/serializer/receipts-readiness-docs-onboarding-latest.machine",
    "write_readiness_docs_onboarding_receipt",
    "readiness_docs_onboarding",
    "readiness_docs_onboarding_receipt_is_current",
    "readiness_docs_onboarding_sr_fields",
    "readiness_docs_onboarding_source_check",
    "developer-contract-current-model",
    "source-owned-docs-onboarding-foundation-current",
    "source-owned-docs-onboarding-receipt-current-generated-archive-clean",
    "local-source-owned-docs-onboarding-foundation",
    "docs_onboarding_receipt_current",
    "docs_doctor_command_replay_current",
    "docs-onboarding-receipt-not-current",
    "docs-onboarding-command-replay-proof-needed",
    "docs-onboarding-public-browser-provider-proof-needed",
    "source-owned-docs-onboarding-receipt-current-docs-doctor-replay-current",
    "node --test benchmarks/dx-www-readiness-docs-onboarding-receipts.test.ts",
    "source-owned docs/onboarding guardrail with generated/archive cleanup evaluation; external docs-doctor command replay and compatibility warning cleanup remain separate release-readiness gates",
    "This receipt validates the source-owned docs/onboarding guardrail and evaluates the docs-doctor report for generated/archive warning cleanup; it does not execute an external docs-doctor command or claim release readiness.",
  ]);

  expectAll(readiness, [
    "docs_doctor_report_evaluated",
    "docs_doctor_runtime_executed",
    "docs_doctor_error_count",
    "docs_doctor_warning_count",
    "public_docs_source_guarded",
    "compatibility_surfaces_warning_only",
    "generated_archived_warning_surfaces_clean",
    "generated_archived_warning_surfaces_promoted",
    "generated_archived_warning_finding_count",
    "generated_archived_warning_sample_paths",
    "release_ready",
    "readiness_release_ready",
    "fastest_world_claim",
  ]);
});

test("release-readiness docs/onboarding receipt watches the real docs-doctor surfaces", () => {
  const docsDoctor = read("dx-www/src/cli/docs_doctor.rs");
  const gettingStarted = read("docs/getting-started.md");
  const dxWwwReadme = read("dx-www/README.md");
  const developerContract = read("docs/dx-www-developer-contract.md");
  const docsTest = read("benchmarks/dx-www-docs-doctor.test.ts");

  expectAll(docsDoctor, [
    "dx.www.docs_doctor",
    "dx www docs-doctor --json",
    "dx www docs-doctor --json --write-receipt",
    "DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_CONTRACT",
    "dx.www.docs_doctor.command_replay_receipt_contract",
    ".dx/receipts/readiness/docs-doctor-command-replay-latest.json",
    ".dx/receipts/readiness/docs-doctor-command-replay-latest.sr",
    ".dx/serializer/receipts-readiness-docs-doctor-command-replay-latest.machine",
    "docs_doctor_command_replay_receipt_is_current",
    "MONITORED_PUBLIC_DOCS",
    "MONITORED_COMPATIBILITY_SURFACES",
    "MONITORED_GENERATED_ARCHIVED_CLAIM_ROOTS",
    "DOCS_DOCTOR_CONFIG_SNIPPET_MARKERS",
    "DOCS_DOCTOR_UNRESOLVED_DOC_MACROS",
    "DOCS_DOCTOR_ALLOWLISTS",
    "READINESS_REQUIRED_RECEIPTS",
    "STARTER_CHECK_RECEIPT_PATH",
    "docs_doctor_required_receipt_findings",
    "docs_doctor_config_snippet_findings",
    "docs_doctor_unresolved_doc_macro_findings",
    "generated_archived_claim_surface_policy",
    "generated-archived-stale-claim",
  ]);

  expectAll(gettingStarted, [
    "dx new",
    "dx dev",
    "dx build",
    "dx check",
    "dx www readiness --json --full",
    "dx www agent-context --json --full",
    "dx www docs-doctor --json",
    "dx www docs-doctor --json --write-receipt",
    ".dx/www/output",
  ]);

  expectAll(dxWwwReadme, [
    "project(name=dx-www-template",
    "www(",
    "output_dir=.dx/www/output",
    "dev(host=127.0.0.1 port=3000 hot_reload=true devtools=true)",
    "imports(",
    "aliases=#imports,#components",
    "check(score_scale=500 lighthouse=true)",
  ]);

  expectAll(developerContract, [
    "www should feel familiar to React and Next.js developers",
    "app/",
    "components/",
    "styles/",
    "Strict apps should keep hand-authored and forge-owned source",
    "Generated caches, opaque dependency folders, and install artifacts are not part of the strict contract.",
    "This is not a universal npm replacement claim.",
  ]);

  expectAll(docsTest, [
    "public WWW docs stay on the current app router and proof workflow",
    "docs doctor separates public-doc failures from compatibility-surface warnings",
    "docs doctor proves public starter score and inventory claims against current artifacts",
    "docs doctor rejects stale config snippets and unresolved architecture placeholders",
  ]);
});

test("Agent context exposes docs/onboarding receipt status and blockers", () => {
  const agentContext = read("dx-www/src/cli/agent_context.rs");

  expectAll(agentContext, [
    "DOCS_ONBOARDING_RECEIPT",
    "DOCS_ONBOARDING_RECEIPT_SCHEMA",
    "DOCS_ONBOARDING_RECEIPT_SR",
    "DOCS_ONBOARDING_RECEIPT_MACHINE",
    "DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT",
    "DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_SCHEMA",
    "DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_SR",
    "DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_MACHINE",
    "docs_onboarding_receipt_status",
    "docs_doctor_command_replay_receipt_status",
    "docs_onboarding_receipt_is_current",
    "\"docs_onboarding\"",
    "\"docs_doctor_command_replay\"",
    "\"docs-onboarding-doctor\"",
    "\"docs-doctor-command-replay\"",
    'receipt.get("source_check_count").and_then(Value::as_u64) == Some(5)',
    "docs_doctor_report_evaluated",
    "generated_archived_warning_finding_count",
    "docs-onboarding-receipt-missing",
    "docs-doctor-command-replay-receipt-missing",
    "readiness-docs-onboarding-machine-contract-missing",
    "readiness-docs-doctor-command-replay-machine-contract-missing",
    "Source-owned docs/onboarding release-readiness proof is missing or stale",
    "Source-owned docs/onboarding release-readiness proof is not serializer-backed yet",
    "Docs-doctor command replay proof is missing or stale",
    "Generated/archive cleanup and docs-doctor command replay remain separate gates",
    "local-source-owned-docs-onboarding-foundation",
  ]);
});
