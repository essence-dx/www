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

function functionBody(source: string, name: string): string {
  const start = source.indexOf(`fn ${name}`);
  assert.notEqual(start, -1, `${name} must exist`);
  const braceStart = source.indexOf("{", start);
  assert.notEqual(braceStart, -1, `${name} must have a body`);

  let depth = 0;
  for (let index = braceStart; index < source.length; index += 1) {
    if (source[index] === "{") {
      depth += 1;
    }
    if (source[index] === "}") {
      depth -= 1;
      if (depth === 0) {
        return source.slice(braceStart, index + 1);
      }
    }
  }

  assert.fail(`${name} body did not close`);
}

test("release-readiness imports real browser receipts only after freshness validation", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");

  expectAll(readiness, [
    "--import-native-event-browser-binder-receipt",
    "--import-state-runtime-browser-receipt",
    "--import-visual-edit-browser-receipt",
    "--import-no-js-browser-receipt",
    "--import-island-browser-receipt",
    "--import-browser-page-snapshot",
    "READINESS_BROWSER_PAGE_SNAPSHOT_SCHEMA",
    "READINESS_BROWSER_RECEIPT_HARNESS",
    "READINESS_BROWSER_IMPORT_CANDIDATE_DIR",
    "import_readiness_browser_page_snapshot_receipts",
    "browser_page_snapshot_no_js_browser_candidate",
    "validate_browser_page_snapshot_candidate_receipts",
    "browser_page_snapshot_candidate_error",
    "real-page-snapshot-converted-then-validated-current-before-canonical-write",
    "JS-enabled page snapshots can refresh runtime browser receipts without inventing JS-disabled no-JS browser proof.",
    "no-js-browser-candidate-missing",
    "skipped_receipts",
    "imported_browser_receipts",
    "read_readiness_import_json",
    "resolve_readiness_import_path",
    "add_imported_browser_receipt_metadata",
    "write_readiness_json_receipt",
    "validated-current-before-canonical-write",
    "import_source_path",
    "import_source_within_project",
    "imported_by",
    "serializer_provenance",
    "READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT",
    "READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR",
    "READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_MACHINE",
    "READINESS_STATE_RUNTIME_BROWSER_RECEIPT",
    "READINESS_STATE_RUNTIME_BROWSER_RECEIPT_SR",
    "READINESS_STATE_RUNTIME_BROWSER_RECEIPT_MACHINE",
    "READINESS_ISLAND_BROWSER_RECEIPT_CONTRACT",
    "READINESS_ISLAND_BROWSER_RECEIPT",
    "READINESS_ISLAND_BROWSER_RECEIPT_SR",
    "READINESS_ISLAND_BROWSER_RECEIPT_MACHINE",
    "READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT",
    "READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_SR",
    "READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_MACHINE",
    "READINESS_NO_JS_BROWSER_RECEIPT_CONTRACT",
    "READINESS_NO_JS_BROWSER_RECEIPT",
    "READINESS_NO_JS_BROWSER_RECEIPT_SR",
    "READINESS_NO_JS_BROWSER_RECEIPT_MACHINE",
    "visual_edit_browser_workbench_receipt_value_is_current",
    "import_readiness_visual_edit_browser_receipt",
    "readiness_visual_edit_browser_workbench_sr_fields",
    "import_readiness_island_browser_receipt",
    "island_browser_receipt_is_current",
    "island_browser_stale_reason_from_receipt",
    "readiness_island_browser_sr_fields",
    "readiness_no_js_browser_receipt_is_current",
    "readiness_no_js_browser_artifact_hash_matches",
    "readiness_no_js_browser_stale_reason",
    "readiness_no_js_browser_stale_reason_from_receipt",
    "import_readiness_no_js_browser_receipt",
    "readiness_no_js_browser_sr_fields",
    "local-in-app-browser-visual-edit-workbench-replay",
    "local-in-app-browser-source-owned-island-replay",
    "source-owned-island-browser-replay-current",
    "current-local-js-disabled-browser-proof",
    "browser-workbench-replay-current-local-provider-proof-needed",
    "island-browser-receipt-missing",
    "no-js-browser-receipt-missing",
    "no-js-browser-artifact-hash-mismatch",
    "no-js-browser-execution-flags-invalid",
    "no-js-browser-static-markers-invalid",
    "no-js-browser-meaningful-html-incomplete",
    "release_ready",
    "fastest_world_claim",
    "dx www readiness --import-state-runtime-browser-receipt <browser-receipt.json> --json --full",
    "dx www readiness --import-native-event-browser-binder-receipt <browser-receipt.json> --json --full",
    "dx www readiness --import-visual-edit-browser-receipt <browser-receipt.json> --json --full",
    "dx www readiness --import-island-browser-receipt <browser-receipt.json> --json --full",
    "dx www readiness --import-no-js-browser-receipt <browser-receipt.json> --json --full",
    "dx www readiness --import-browser-page-snapshot <page-snapshot.json> --json --full",
    "node --test benchmarks/dx-www-readiness-browser-receipt-import.test.ts",
  ]);

  const snapshotImport = functionBody(readiness, "import_readiness_browser_page_snapshot_receipts");
  assert.match(snapshotImport, /READINESS_BROWSER_PAGE_SNAPSHOT_SCHEMA/);
  assert.match(snapshotImport, /Command::new\("node"\)/);
  assert.match(snapshotImport, /READINESS_BROWSER_RECEIPT_HARNESS/);
  assert.match(snapshotImport, /validate_browser_page_snapshot_candidate_receipts/);
  assert.match(snapshotImport, /browser_page_snapshot_no_js_browser_candidate/);
  assert.match(snapshotImport, /import_readiness_no_js_browser_receipt/);
  assert.ok(
    snapshotImport.indexOf("validate_browser_page_snapshot_candidate_receipts") <
      snapshotImport.indexOf("import_readiness_native_event_browser_binder_receipt"),
    "browser page snapshot import must validate every generated candidate before canonical writes",
  );
  assert.ok(
    snapshotImport.indexOf("browser_page_snapshot_no_js_browser_candidate") <
      snapshotImport.indexOf("import_readiness_no_js_browser_receipt"),
    "no-JS browser page snapshot candidate must be evaluated before optional canonical import",
  );

  const snapshotValidator = functionBody(
    readiness,
    "validate_browser_page_snapshot_candidate_receipts",
  );
  expectAll(snapshotValidator, [
    "native_event_browser_binder_receipt_is_current(&native)",
    "state_runtime_browser_receipt_is_current(&state)",
    "island_browser_receipt_is_current(&island_browser)",
    "visual_edit_browser_workbench_receipt_value_is_current(project, &visual)",
    "browser_page_snapshot_candidate_error",
  ]);

  const nativeImport = functionBody(
    readiness,
    "import_readiness_native_event_browser_binder_receipt",
  );
  assert.match(nativeImport, /native_event_browser_binder_receipt_is_current\(&receipt\)/);
  assert.match(nativeImport, /readiness_native_event_browser_binder_sr_fields\(&receipt\)/);
  assert.match(nativeImport, /write_sr_artifact\(/);
  assert.match(nativeImport, /write_readiness_json_receipt\(/);
  assert.ok(
    nativeImport.indexOf("native_event_browser_binder_receipt_is_current(&receipt)") <
      nativeImport.indexOf("write_sr_artifact("),
    "native-event browser receipt must be validated before any canonical SR or JSON write",
  );

  const nativeValidator = functionBody(readiness, "native_event_browser_binder_receipt_is_current");
  expectAll(nativeValidator, [
    "react_synthetic_events",
    "full_react_event_parity",
    "json_array_record_string_field_contains_with_bool",
    "previewed",
    "local-in-app-browser-native-event-binder-replay",
  ]);

  const stateImport = functionBody(readiness, "import_readiness_state_runtime_browser_receipt");
  assert.match(stateImport, /state_runtime_browser_receipt_is_current\(&receipt\)/);
  assert.match(stateImport, /readiness_state_runtime_browser_sr_fields\(&receipt\)/);
  assert.match(stateImport, /write_sr_artifact\(/);
  assert.match(stateImport, /write_readiness_json_receipt\(/);
  assert.ok(
    stateImport.indexOf("state_runtime_browser_receipt_is_current(&receipt)") <
      stateImport.indexOf("write_sr_artifact("),
    "state-runtime browser receipt must be validated before any canonical SR or JSON write",
  );

  const visualImport = functionBody(readiness, "import_readiness_visual_edit_browser_receipt");
  assert.match(
    visualImport,
    /visual_edit_browser_workbench_receipt_value_is_current\(project, &receipt\)/,
  );
  assert.match(visualImport, /readiness_visual_edit_browser_workbench_sr_fields\(&receipt\)/);
  assert.match(visualImport, /write_sr_artifact\(/);
  assert.match(visualImport, /write_readiness_json_receipt\(/);
  assert.ok(
    visualImport.indexOf("visual_edit_browser_workbench_receipt_value_is_current(project, &receipt)") <
      visualImport.indexOf("write_sr_artifact("),
    "visual-edit browser receipt must be validated before any canonical SR or JSON write",
  );

  const visualValidator = functionBody(
    readiness,
    "visual_edit_browser_workbench_receipt_value_is_current",
  );
  expectAll(visualValidator, [
    "browser_runtime_executed",
    "visual_replay_attempted",
    "visual_replay_status",
    "visual_replay_reason",
    "devtools_global_present",
    "browser_workbench_replay",
    "local-in-app-browser-visual-edit-workbench-replay",
    "inspected_element_present",
    "cascade_inspected",
    "preview_source_mutated",
    "apply_source_mutated",
    "undo_source_restored",
    "safe_local_source_target_known",
    "apply_receipt_written",
    "undo_receipt_written",
    "json-sr-machine-written",
    "page_url",
    "user_agent",
    "viewport",
    "inspected_selector",
    "inspected_element_fingerprint",
    "style_property",
    "style_value",
    "visual_edit_computed_style_values_are_consistent",
    "browser_snapshot_hash",
    "source_target",
    "source_root",
    "visual_edit_source_target_is_current",
    "json_snapshot_hash_is_current",
  ]);
  const visualStyleValidator = functionBody(
    readiness,
    "visual_edit_computed_style_values_are_consistent",
  );
  expectAll(visualStyleValidator, [
    "style_property",
    "style_value",
    "computed_style_before",
    "computed_style_after_preview",
    "computed_style_after_undo",
    'json_object_string_at(before, "property", style_property)',
    'json_object_string_at(after_preview, "property", style_property)',
    'json_object_string_at(after_undo, "property", style_property)',
    'json_object_string_at(after_preview, "value", style_value)',
    "after_undo",
    "before",
  ]);
  const sourceTargetValidator = functionBody(readiness, "visual_edit_source_target_is_current");
  expectAll(sourceTargetValidator, [
    "safe_visual_edit_relative_path",
    "visual_edit_source_target_path",
    "expectedText",
    "std::fs::read",
  ]);
  const sourceTargetPath = functionBody(readiness, "visual_edit_source_target_path");
  expectAll(sourceTargetPath, ["source_root", "canonical_path_within_root"]);

  const islandImport = functionBody(readiness, "import_readiness_island_browser_receipt");
  assert.match(islandImport, /island_browser_receipt_is_current\(&receipt\)/);
  assert.match(islandImport, /readiness_island_browser_sr_fields\(&receipt\)/);
  assert.match(islandImport, /write_sr_artifact\(/);
  assert.match(islandImport, /write_readiness_json_receipt\(/);
  assert.ok(
    islandImport.indexOf("island_browser_receipt_is_current(&receipt)") <
      islandImport.indexOf("write_sr_artifact("),
    "island browser receipt must be validated before any canonical SR or JSON write",
  );
  const islandValidator = functionBody(readiness, "island_browser_receipt_is_current");
  expectAll(islandValidator, [
    "READINESS_ISLAND_BROWSER_RECEIPT_CONTRACT",
    "browser_runtime_executed",
    "source_owned_bridge",
    "bridge_abi_style",
    "camelCase",
    "directive_style",
    "camelCase-jsx-props",
    "clientLoad",
    "clientVisible",
    "clientIdle",
    "clientOnly",
    "missing_core_directives",
    "event_node_count",
    "client_island_event_count",
    "missed_event_replay_count",
    "full_react_hydration",
    "node_modules_required",
    "react_synthetic_events",
    "provider_adapter_executed",
    "local-in-app-browser-source-owned-island-replay",
    "json_snapshot_hash_is_current",
  ]);

  const noJsImport = functionBody(readiness, "import_readiness_no_js_browser_receipt");
  assert.match(
    noJsImport,
    /readiness_no_js_browser_receipt_is_current\(project, &receipt\)/,
  );
  assert.match(noJsImport, /readiness_no_js_browser_sr_fields\(&receipt\)/);
  assert.match(noJsImport, /write_sr_artifact\(/);
  assert.match(noJsImport, /write_readiness_json_receipt\(/);
  assert.ok(
    noJsImport.indexOf("readiness_no_js_browser_receipt_is_current(project, &receipt)") <
      noJsImport.indexOf("write_sr_artifact("),
    "no-JS browser receipt must be validated before any canonical SR or JSON write",
  );

  const noJsValidator = functionBody(readiness, "readiness_no_js_browser_receipt_is_current");
  expectAll(noJsValidator, [
    "READINESS_NO_JS_BROWSER_RECEIPT_CONTRACT",
    "live_browser_executed",
    "javascript_disabled_browser",
    "page_javascript_enabled",
    "data_dx_output_mode_tiny_static",
    "data_dx_js_none",
    "script_tag_count",
    "semantic_landmark_present",
    "visible_text_present",
    "link_count",
    "form_count",
    "seo_title_present",
    "accessibility_signal_count",
    "release_ready",
    "fastest_world_claim",
    "readiness_no_js_browser_artifact_hash_matches",
  ]);
  const noJsHashValidator = functionBody(readiness, "readiness_no_js_browser_artifact_hash_matches");
  expectAll(noJsHashValidator, ["html_path", "artifact_html_blake3", "file_blake3_hex"]);

  const jsonWriter = functionBody(readiness, "write_readiness_json_receipt");
  assert.match(jsonWriter, /serde_json::to_string_pretty\(receipt\)/);
  assert.match(jsonWriter, /std::fs::write\(&json_path, json_text\)/);
});
