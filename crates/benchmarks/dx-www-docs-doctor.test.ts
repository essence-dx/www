import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");

const publicWwwDocs = [
  "README.md",
  "core/README.md",
  "dx-www/README.md",
  "docs/getting-started.md",
  "docs/api/README.md",
  "docs/architecture.md",
  "docs/DX_WWW_FRAMEWORK_STRUCTURE.md",
  "docs/dx-www-developer-contract.md",
  "docs/DX_WWW_CURRENT_DETAILS_2026-05-29.md",
];

function read(relativePath: string): string {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function expectAll(source: string, markers: string[]): void {
  for (const marker of markers) {
    assert.match(source, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
}

function exists(relativePath: string): boolean {
  return fs.existsSync(path.join(root, relativePath));
}

function extractReadinessReplayCommands(source: string): string[] {
  const body = source.match(
    /pub\(crate\) fn readiness_replay_commands\(\) -> Vec<&'static str> \{\s*vec!\[([\s\S]*?)\]\s*\}/,
  )?.[1];
  assert.ok(body, "readiness_replay_commands() should stay source-owned and visible");
  return Array.from(body.matchAll(/"([^"\n]+)"/g), (match) => match[1]);
}

test("public WWW docs stay on the current app router and proof workflow", () => {
  const joinedDocs = publicWwwDocs.map((doc) => `${doc}\n${read(doc)}`).join("\n\n---\n\n");
  const cliCore = read("dx-www/src/cli/mod_parts/cli_core_impl.rs");
  const docsDoctor = read("dx-www/src/cli/docs_doctor.rs");

  expectAll(joinedDocs, [
    "app/",
    "/state-runtime",
    "/islands",
    "dx new -> dx dev -> dx build -> dx check -> receipts",
    "dx new",
    "dx dev",
    "dx build",
    "dx check",
    "dx www readiness --json --full",
    "dx www agent-context --json --full",
    "dx www docs-doctor --json",
    ".dx/www/output",
    "source-owned runtime",
    "no hidden React runtime",
    "dx www readiness --import-state-runtime-browser-receipt <browser-receipt.json> --json --full",
    "dx www readiness --import-native-event-browser-binder-receipt <browser-receipt.json> --json --full",
    "dx www readiness --import-visual-edit-browser-receipt <browser-receipt.json> --json --full",
    "dx www readiness --import-no-js-browser-receipt <browser-receipt.json> --json --full",
    "Static/no-JS proof",
    "data-dx-js=\"none\"",
    "not full React or Next.js runtime parity",
  ]);
  expectAll(cliCore, [
    'Some("docs-doctor") => docs_doctor::cmd_docs_doctor',
    "www docs-doctor --json",
  ]);
  expectAll(docsDoctor, [
    "dx.www.docs_doctor",
    "www_root::discover_www_repo_root(cwd)",
    "MONITORED_DOCS",
    "REQUIRED_MARKERS",
    "REQUIRED_ORDERED_WORKFLOW_MARKERS",
    "missing-current-workflow-sequence",
    "STALE_PATTERNS",
    "dx www docs-doctor --json",
    "older-preview-caveat",
    "agent-context-active-blockers-zero",
  ]);

  for (const stalePattern of [
    /\bdx init\b/i,
    /\bsrc\/App\.tsx\b/,
    /\bHTIP binary\b/i,
    /\bdx\.config\.json\b/i,
    /\bdx serve --port\b/i,
    /\.dxob\b/i,
    /project\.contract\.folders=next-familiar/,
    /build\.output_dir=\.dx\/build/,
    /\.dx\/build\/app\/index\.html/,
    /Some older preview\/hosted\/server-action tests still expose fixture or product-readiness gaps/,
    /\bdx www agent-context --json(?: --full)?[` ]+reports [`']?active_blockers=0[`']?/i,
    /product99|product_99|Product 99/,
  ]) {
    assert.doesNotMatch(joinedDocs, stalePattern);
  }

  assert.doesNotMatch(read("README.md"), /\bG:[\\/](Dx|WWW)[\\/]/i);
});

test("docs that mention pages only do so as an unsupported legacy route tree", () => {
  const structure = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");

  assert.match(structure, /`pages\/` is not part of the WWW starter contract/);

  for (const doc of publicWwwDocs.filter((doc) => doc !== "docs/DX_WWW_FRAMEWORK_STRUCTURE.md")) {
    assert.doesNotMatch(read(doc), /`pages\/`|pages\/.*authoring|pages\/.*starter/i, doc);
  }
});

test("docs doctor separates public-doc failures from compatibility-surface warnings", () => {
  const docsDoctor = read("dx-www/src/cli/docs_doctor.rs");

  expectAll(docsDoctor, [
    "MONITORED_PUBLIC_DOCS",
    "MONITORED_COMPATIBILITY_SURFACES",
    "MONITORED_GENERATED_ARCHIVED_CLAIM_ROOTS",
    "DOCS_DOCTOR_ALLOWLISTS",
    "docs/api/README.md",
    "core/README.md",
    "examples/blog/src/data/posts.ts",
    "examples/conversion-proof/README.md",
    "docs/packages",
    "docs/superpowers/plans",
    "dx-router",
    "relative-pages-import",
    "plain-pages-html",
    "absolute-g-drive-receipt-path",
    "generated-archived-stale-claim",
    "warning-only-generated-archive-coverage",
    "compatibility_allowlists",
    "monitored_public_docs",
    "monitored_compatibility_surfaces",
    "monitored_generated_archived_claim_surfaces",
    "generated_archived_claim_surface_policy",
    "error_count",
    "warning_count",
  ]);

  assert.doesNotMatch(read("docs/api/README.md"), /src\/App\.tsx/);
  assert.doesNotMatch(read("core/README.md"), /src\/App\.tsx/);
  assert.doesNotMatch(
    read("examples/blog/src/data/posts.ts"),
    /dx\.config\.json|src\/App\.tsx|dx init/,
    "examples/blog compatibility docs surface should not revive legacy starter commands",
  );
});

test("docs doctor proves public starter score and inventory claims against current artifacts", () => {
  const docsDoctor = read("dx-www/src/cli/docs_doctor.rs");
  const readiness = read("dx-www/src/cli/readiness.rs");
  const readme = read("README.md");
  const currentReceiptPath = "examples/template/.dx/receipts/check/check-latest.json";
  const currentReceipt = JSON.parse(read(currentReceiptPath)) as {
    score: number;
    max_score: number;
    score_percent: number;
    traffic: string;
    release_ready?: boolean;
    fastest_world_claim?: boolean;
    readiness_gate_status?: {
      release_ready?: boolean;
      fastest_world_claim?: boolean;
      score_kind?: string;
      verified_from_replay_receipts?: boolean;
      receipt_freshness?: string;
      gate_summary?: { id?: string }[];
      proof_node_ids?: string[];
    };
    readiness_replay_commands?: string[];
    replay_commands?: string[];
  };

  expectAll(docsDoctor, [
    "STARTER_CHECK_RECEIPT_PATH",
    "DOCS_DOCTOR_SCORE_CLAIMS",
    "DOCS_DOCTOR_STARTER_PATH_CLAIMS",
    "starter_check_receipt",
    "starter_inventory",
    "receipt-score-mismatch",
    "missing-starter-file-claim",
    "receipt-readiness-gate-stale",
    "readiness-required-receipt-missing",
    "readiness-required-json-receipt-invalid",
    "readiness-required-json-receipt-stale",
    "readiness-required-machine-receipt-stale",
    "READINESS_REQUIRED_RECEIPTS",
    "readiness_required_receipts",
    "readiness_required_receipts",
    "readiness_release_ready",
    "readiness_release_ready",
    "release_claim_allowed",
    "readiness_native_event_catalog_json_read_model_status",
    "readiness_state_runtime_browser_json_read_model_status",
    "native-event-catalog-count-stale",
    "native-event-catalog-hash-stale",
    "runtime-global-missing",
    "full-react-hook-runtime-claimed",
    "react-api-shim-executed",
    "state-reflection-event-count-too-low",
    "derived-reflection-event-count-too-low",
    "effect-scheduled-event-count-too-low",
    "action-dispatch-count-too-low",
    "api-method-missing-",
    "missing-api-methods-present",
    "browser-snapshot-hash-missing",
    "docs_doctor_snapshot_hash_is_current",
    "expected_catalog_count",
    "expected_catalog_hash",
    "readiness::READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT",
    "readiness::READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_SR",
    "readiness::READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_MACHINE",
    "readiness::READINESS_NATIVE_EVENT_CATALOG_RECEIPT",
    "readiness::READINESS_NATIVE_EVENT_CATALOG_RECEIPT_SR",
    "readiness::READINESS_NATIVE_EVENT_CATALOG_RECEIPT_MACHINE",
    "readiness::READINESS_PRODUCTION_HTTP_RECEIPT",
    "readiness::READINESS_PRODUCTION_HTTP_RECEIPT_SR",
    "readiness::READINESS_PRODUCTION_HTTP_RECEIPT_MACHINE",
    "opaque-sr",
    "generated-machine-cache",
    "binary-machine",
    "json_parse_required",
    "json_parse_attempted",
    "json_parse_ok",
    "expected_schema",
    "actual_schema",
    "schema_current",
    "json_read_model_status",
    "json_read_model_current",
    "receipt_durability",
    "receipt_write_status",
    "source_mutated",
    "undo_receipt_status",
    "browser_workbench_replay",
    "generated_archived_claim_surface_policy",
    "generated-archived-stale-claim",
    "MONITORED_GENERATED_ARCHIVED_CLAIM_ROOTS",
    "stale_reasons",
    "style-apply-not-applied",
    "style-undo-receipt-status-not-written",
    "source_contract_path",
    "machine_fresh_against_source",
    "machine_cache_fresh_against_source",
    "modified_unix_ms",
    "content_blake3",
    "source_contract_blake3",
    "file_blake3_hex",
    "unreadable, or not provably fresh",
    "starter_check_readiness_gate",
    "metadata_current",
    "replay_verified_current",
    "static-advisory-not-release-proof",
    "missing-or-unsafe-readiness-gate-metadata",
    "readiness_replay_commands",
    "readiness_replay_commands",
    "readiness_replay_commands_required",
    "readiness::readiness_replay_commands()",
    "missing_replay_commands",
    "verified_from_replay_receipts",
    "receipt_freshness",
    "score_kind",
    "READINESS_REQUIRED_PROOF_NODE_IDS",
    "has_visual_edit_gate_summary",
    "has_visual_edit_proof_node",
    "has_native_events_gate_summary",
    "has_native_events_proof_node",
    "has_tiny_static_gate_summary",
    "has_tiny_static_proof_node",
    "missing_required_gate_summary_ids",
    "missing_required_proof_node_ids",
    "starter_check_has_readiness_gate",
    "starter_check_has_readiness_proof_node",
    "readiness::READINESS_NO_JS_ARTIFACT_RECEIPT",
    "readiness::READINESS_NO_JS_ARTIFACT_RECEIPT_SR",
    "readiness::READINESS_NO_JS_ARTIFACT_RECEIPT_MACHINE",
    "production-http-preview",
    "production-http-json-read-model",
    "production-http-serializer-receipt",
    "production-http-machine-contract",
    "no-js-artifact-json-read-model",
    "readiness_no_js_artifact_json_read_model_status",
    "schema-revision-not-1",
    "id-not-tiny-static-no-js-artifact",
    "status-not-artifact-current",
    "artifact_root",
    "artifact_source",
    "artifact_path_resolution",
    "route-unit-proof-missing",
    "meaningful-html-without-js-not-proven",
    "live-browser-executed-not-false",
    "javascript-disabled-browser-not-false",
    "live-astro-parity-receipt-not-missing",
    "examples/template/.dx/receipts/check/check-latest.json",
    "examples/template/app/page.tsx",
    "examples/template/styles/globals.css",
  ]);
  expectAll(readiness, [
    ".dx/receipts/devtools/visual-edit-latest.json",
    ".dx/receipts/devtools/visual-edit-latest.sr",
    ".dx/serializer/receipts-devtools-visual-edit-latest.machine",
    ".dx/receipts/readiness/native-events-latest.json",
    ".dx/receipts/readiness/native-events-latest.sr",
    ".dx/serializer/receipts-readiness-native-events-latest.machine",
    ".dx/receipts/readiness/no-js-artifact-latest.json",
    ".dx/receipts/readiness/no-js-artifact-latest.sr",
    ".dx/serializer/receipts-readiness-no-js-artifact-latest.machine",
    "dx.www.readiness.no_js_artifact_receipt_contract",
    "script_tag_count",
    "data_dx_output_mode_tiny_static",
    "data_dx_js_none",
    "public_packet_present",
    "meaningful_html_without_js",
    "live_browser_executed",
    "javascript_disabled_browser",
    "astro_parity_claimed",
    "live_astro_parity_receipt",
    "--write-visual-edit-replay",
    "visual_edit_replay",
    "dx www readiness --write-visual-edit-replay --json",
    "node --test benchmarks/dx-devtools-framework-integration.test.ts",
    "node --test benchmarks/dx-www-docs-doctor.test.ts",
  ]);

  assert.equal(currentReceipt.fastest_world_claim, false);
  const receiptReadinessGate = currentReceipt.readiness_gate_status;
  if (receiptReadinessGate) {
    assert.equal(receiptReadinessGate.fastest_world_claim, false);
    if (currentReceipt.release_ready) {
      assert.equal(receiptReadinessGate.release_ready, true);
      assert.equal(receiptReadinessGate.score_kind, "relative-local-proof-backed-release-ready");
      assert.equal(receiptReadinessGate.verified_from_replay_receipts, true);
      assert.equal(receiptReadinessGate.receipt_freshness, "current");
    } else {
      assert.equal(receiptReadinessGate.release_ready, false);
      assert.equal(receiptReadinessGate.verified_from_replay_receipts, false);
      assert.ok(
        ["not-evaluated-in-this-command", "local-receipts-evaluated"].includes(
          receiptReadinessGate.receipt_freshness ?? "",
        ),
        "starter check receipt should keep freshness advisory unless local proof backs release readiness",
      );
    }
    for (const gateId of [
      "visual-edit-workbench-receipts",
      "native-events",
      "tiny-static",
      "production-http-preview",
    ]) {
      assert.ok(
        receiptReadinessGate.gate_summary?.some((gate) => gate.id === gateId),
        `starter check receipt should keep the ${gateId} release-readiness gate visible`,
      );
      assert.ok(
        receiptReadinessGate.proof_node_ids?.includes(gateId),
        `starter check receipt should keep the ${gateId} proof node visible`,
      );
    }
  } else {
    assert.ok(
      docsDoctor.includes("missing-or-unsafe-readiness-gate-metadata"),
      "stale starter receipt must be flagged instead of treated as refreshed proof",
    );
  }
  if (currentReceipt.readiness_replay_commands) {
    assert.ok(
      currentReceipt.readiness_replay_commands.includes("dx www readiness --json --full"),
      "starter check receipt should carry release-readiness replay commands",
    );
    assert.ok(
      currentReceipt.readiness_replay_commands.includes(
        "node --test benchmarks/dx-devtools-framework-integration.test.ts",
      ),
      "starter check receipt should carry the Devtools release-readiness replay guard",
    );
    assert.ok(
      currentReceipt.readiness_replay_commands.includes(
        "node --test benchmarks/dx-www-docs-doctor.test.ts",
      ),
      "starter check receipt should carry the docs-doctor release-readiness replay guard",
    );
  } else {
    assert.ok(
      docsDoctor.includes("missing_replay_commands"),
      "stale starter receipt must report missing readiness replay commands",
    );
  }
  assert.ok(
    currentReceipt.replay_commands?.includes("dx www agent-context --json --full"),
    "starter check receipt should carry agent-context replay commands",
  );
  assert.ok(
    currentReceipt.replay_commands?.includes("dx www docs-doctor --json"),
    "starter check receipt should carry docs-doctor replay commands",
  );
  const readinessReplayCommands = extractReadinessReplayCommands(readiness);
  assert.ok(
    readinessReplayCommands.length >= 30,
    "release-readiness replay command list should cover the current proof surface",
  );
  const missingReadinessReplayCommands = readinessReplayCommands.filter(
    (command) => !currentReceipt.readiness_replay_commands?.includes(command),
  );
  const missingReplayCommands = readinessReplayCommands.filter(
    (command) => !currentReceipt.replay_commands?.includes(command),
  );
  assert.ok(
    missingReadinessReplayCommands.length === 0 || docsDoctor.includes("missing_replay_commands"),
    `starter readiness_replay_commands missing source-owned commands without stale detection: ${missingReadinessReplayCommands.join(", ")}`,
  );
  assert.ok(
    missingReplayCommands.length === 0 || docsDoctor.includes("missing_replay_commands"),
    `starter replay_commands missing source-owned commands without stale detection: ${missingReplayCommands.join(", ")}`,
  );

  assert.match(readme, new RegExp(`${currentReceipt.score}\\s*/\\s*${currentReceipt.max_score}`));
  assert.match(readme, new RegExp(`${currentReceipt.score_percent}%`));
  assert.match(readme, new RegExp(`traffic:\\s*${currentReceipt.traffic}`));

  for (const claimedPath of [
    "examples/template/app/page.tsx",
    "examples/template/styles/globals.css",
    "examples/template/styles/theme.css",
    "examples/template/styles/generated.css",
    "examples/template/components/icons/icon.tsx",
    "examples/template/dx",
  ]) {
    assert.equal(exists(claimedPath), true, `${claimedPath} should exist if public docs claim it`);
    assert.match(readme, new RegExp(claimedPath.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const removedClaim of [
    "examples/template/app/dashboard/page.tsx",
    "examples/template/app/api/health/route.ts",
    "examples/template/components/ui/*",
    "examples/template/public/preview-manifest.json",
  ]) {
    assert.equal(exists(removedClaim.replace(/\*$/, "")), false, `${removedClaim} is absent today`);
    for (const doc of publicWwwDocs) {
      assert.doesNotMatch(read(doc), new RegExp(removedClaim.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
    }
  }
});

test("docs doctor rejects stale config snippets and unresolved architecture placeholders", () => {
  const docsDoctor = read("dx-www/src/cli/docs_doctor.rs");
  const readiness = read("dx-www/src/cli/readiness.rs");
  const packageReadme = read("dx-www/README.md");
  const architecture = read("docs/architecture.md");

  expectAll(docsDoctor, [
    "DOCS_DOCTOR_CONFIG_SNIPPET_MARKERS",
    "DOCS_DOCTOR_UNRESOLVED_DOC_MACROS",
    "config-snippet-drift",
    "unresolved-doc-macro",
    "config_snippet_markers",
    "unresolved_doc_macros",
    "project(name=dx-www-template",
    "www(",
    "output_dir=.dx/www/output",
    "check(score_scale=500",
    "@flow",
    "@seq",
    "@tree",
  ]);

  expectAll(readiness, [
    "config-snippet-drift",
    "unresolved-doc-macro",
    "DOCS_DOCTOR_CONFIG_SNIPPET_MARKERS",
    "DOCS_DOCTOR_UNRESOLVED_DOC_MACROS",
  ]);

  for (const staleConfig of [
    /project\.name=/,
    /project\.contract\./,
    /forge\.paths\./,
    /build\.output_dir=/,
    /dev\.host=/,
    /tooling\.biome\./,
    /tooling\.dx_style\./,
  ]) {
    assert.doesNotMatch(packageReadme, staleConfig);
  }

  expectAll(packageReadme, [
    "project(name=dx-www-template",
    "www(",
    "output_dir=.dx/www/output",
    "dev(host=127.0.0.1 port=3000 hot_reload=true devtools=true)",
    "imports(",
    "aliases=#imports,#components",
    "check(score_scale=500 lighthouse=true)",
  ]);

  assert.doesNotMatch(architecture, /@(flow|seq|tree)(?::[A-Za-z0-9_-]+)?\[/);
  expectAll(architecture, [
    "System Architecture",
    "Compilation Pipeline",
    "Request Flow",
    "State Update Flow",
    "Directory Structure",
    "app/",
    "dx-www/src/cli",
    ".dx/www/output",
  ]);
});
