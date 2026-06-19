const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const executionPath = path.join(root, "dx-www", "src", "cli", "app_router_execution.rs");
const receiptPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "app_router_execution",
  "next_custom_transforms.rs",
);
const conflictsPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "app_router_execution",
  "next_custom_transforms",
  "conflicts.rs",
);
const contractPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "app_router_execution",
  "next_custom_transforms",
  "contract.rs",
);
const inlineServerActionsPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "app_router_execution",
  "next_custom_transforms",
  "inline_server_actions.rs",
);
const serverActionsPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "app_router_execution",
  "next_custom_transforms",
  "server_actions.rs",
);
const rscBoundariesPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "app_router_execution",
  "next_custom_transforms",
  "rsc_boundaries.rs",
);
const pageConfigExportsPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "app_router_execution",
  "next_custom_transforms",
  "page_config_exports.rs",
);
const dynamicImportsPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "app_router_execution",
  "next_custom_transforms",
  "dynamic_imports.rs",
);
const fontLoadersPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "app_router_execution",
  "next_custom_transforms",
  "font_loaders.rs",
);
const metadataExportsPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "app_router_execution",
  "next_custom_transforms",
  "metadata_exports.rs",
);
const metadataScannerPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "app_router_execution",
  "next_custom_transforms",
  "metadata_exports",
  "scanner.rs",
);

function read(filePath) {
  assert.ok(fs.existsSync(filePath), `missing ${path.relative(root, filePath)}`);
  return fs.readFileSync(filePath, "utf8");
}

test("DX App Router records source-owned Next custom transform receipts", () => {
  const execution = read(executionPath);
  const receipt = read(receiptPath);
  const conflicts = read(conflictsPath);
  const contract = read(contractPath);
  const inlineServerActions = read(inlineServerActionsPath);
  const serverActions = read(serverActionsPath);
  const rscBoundaries = read(rscBoundariesPath);
  const pageConfigExports = read(pageConfigExportsPath);
  const dynamicImports = read(dynamicImportsPath);
  const fontLoaders = read(fontLoadersPath);
  const metadataExports = read(metadataExportsPath);
  const metadataScanner = read(metadataScannerPath);

  assert.match(execution, /mod next_custom_transforms;/);
  assert.match(execution, /build_next_custom_transform_receipt/);
  assert.match(execution, /"next_custom_transform_receipt"/);
  assert.match(receipt, /mod contract;/);
  assert.match(receipt, /next_custom_transform_adapter_contract/);
  assert.match(receipt, /next_custom_transform_upstream_evidence/);
  assert.match(receipt, /next_custom_transform_limits/);
  assert.match(receipt, /next_custom_transform_contract_booleans/);
  assert.match(receipt, /next_custom_transform_runtime_generation_contract/);
  assert.match(
    receipt,
    /next_custom_transform_runtime_generation_contract\(\s*&rsc_boundaries,\s*&server_actions,\s*&dynamic_imports,\s*&font_loaders,\s*\)/,
  );
  assert.match(receipt, /mod conflicts;/);
  assert.match(receipt, /collect_next_custom_transform_conflicts/);
  assert.match(receipt, /mod inline_server_actions;/);
  assert.match(receipt, /mod server_actions;/);
  assert.match(receipt, /mod rsc_boundaries;/);
  assert.match(receipt, /mod page_config_exports;/);
  assert.match(receipt, /mod dynamic_imports;/);
  assert.match(receipt, /mod font_loaders;/);
  assert.match(receipt, /mod metadata_exports;/);
  assert.match(metadataExports, /mod scanner;/);
  assert.match(receipt, /collect_server_action_detections/);
  assert.match(receipt, /collect_rsc_boundary_detections/);
  assert.match(receipt, /collect_page_config_export_descriptors/);
  assert.match(receipt, /collect_dynamic_import_detections/);
  assert.match(receipt, /collect_font_loader_detections/);
  assert.match(receipt, /collect_metadata_export_detections/);

  for (const token of [
    "NEXT_CUSTOM_TRANSFORM_SCHEMA",
    "NEXT_CUSTOM_TRANSFORM_CONTRACT_NAME",
    "node_modules_required",
    "full_nextjs_runtime_parity",
    "source_owned_receipt",
    "contract_booleans",
    "runtime_generation",
    "rsc_boundaries",
    "server_actions",
    "page_config_exports",
    "dynamic_imports",
    "font_loaders",
    "metadata_exports",
    "conflicts",
  ]) {
    assert.match(receipt, new RegExp(token.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const token of [
    "adapter-boundary-contract",
    "dx.next.customTransformReceipt",
    "NEXT_CUSTOM_TRANSFORM_SCHEMA",
    "Next Custom Transforms Compatibility Receipt",
    "next-custom-transforms-adapter",
    "react_server_components.rs",
    "server_actions.rs",
    "page_config.rs",
    "dynamic.rs",
    "track_dynamic_imports.rs",
    "fonts/font_functions_collector.rs",
    "source-owned-detection-only",
    "runtime_takeover",
    "react_required",
    "rsc_required",
    "node_required",
    "swc_transform_execution",
    "node_modules_required",
    "full_nextjs_runtime_parity",
    "source_owned_receipt",
    "does_not_claim_nextjs_parity",
    "does_not_require_react_or_rsc",
    "does_not_require_node_modules",
    "next_custom_transform_runtime_generation_contract",
    "runtime_generation_contract",
    "runtime_generation_surface_counts",
    "detected_generation_attempts",
    "runtime_generation_detected",
    "count_true_field",
    "source_rewrite_performed",
    "next_proxy_generated",
    "rsc_runtime_proxy_generated",
    "font_css_import_generated",
    "dynamic_loadable_generated",
    "NEXT_CUSTOM_TRANSFORM_LIMITS",
  ]) {
    assert.match(contract, new RegExp(token.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const token of [
    "adapter-boundary-conflicts",
    "collect_next_custom_transform_conflicts",
    "diagnostic_status",
    "dx-check-receipt-only",
    "client-and-server-directives",
    "page-config-re-export",
    "next-dynamic-ssr-false",
    "font-loader-call-outside-module-scope",
    "metadata-and-generateMetadata",
  ]) {
    assert.match(conflicts, new RegExp(token.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const token of [
    "balanced-inline-server-actions",
    "InlineServerAction",
    "collect_inline_server_actions",
    "find_balanced_delimiter",
    "server_directive_in_body",
    "return { ok: true",
    "misplaced directive",
  ]) {
    assert.match(
      inlineServerActions,
      new RegExp(token.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
    );
  }

  for (const token of [
    "balanced-server-actions",
    "ServerActionDetection",
    "collect_server_action_detections",
    "collect_module_directive_exports",
    "collect_inline_server_actions",
    "module-directive-export",
    "inline-function-directive",
    "action_id_strategy",
    "next_proxy_generated",
  ]) {
    assert.match(serverActions, new RegExp(token.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const token of [
    "balanced-rsc-boundaries",
    "RscBoundaryDetection",
    "collect_rsc_boundary_detections",
    "collect_top_level_directives",
    "use_cache",
    "client_entry_reasons",
    "metadata_export_in_client",
    "error_file_requires_client",
    "runtime_proxy_generated",
    "source_owned_boundary",
  ]) {
    assert.match(rscBoundaries, new RegExp(token.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const token of [
    "balanced-page-config-exports",
    "PageConfigExport",
    "collect_page_config_exports",
    "classify_page_config_value",
    "named-re-export",
    "object-literal",
    "config-re-export",
    "config-object-spread-unsupported",
  ]) {
    assert.match(pageConfigExports, new RegExp(token.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const token of [
    "balanced-dynamic-imports",
    "DynamicImportDetection",
    "collect_dynamic_import_detections",
    "tracked_export_names",
    "private-next-rsc-track-dynamic-import",
    "loadableGenerated",
    "next-dynamic-options-must-be-object",
    "transition_added",
    "find_balanced_delimiter",
  ]) {
    assert.match(dynamicImports, new RegExp(token.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const token of [
    "balanced-font-loader-calls",
    "FontLoaderDetection",
    "collect_font_loader_detections",
    "next/font/google",
    "next/font/local",
    "module_scope",
    "call_scope",
    "font-loader-call-outside-module-scope",
    "font-loader-namespace-import",
    "css_variable_receipt",
  ]) {
    assert.match(fontLoaders, new RegExp(token.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const token of [
    "balanced-metadata-exports",
    "MetadataExportDetection",
    "collect_metadata_export_detections",
    "static_metadata",
    "generate_metadata",
    "metadata-and-generateMetadata",
    "metadata-re-export",
    "server_component_only_enforced",
    "parsed_static_metadata",
    "read_generate_metadata_return",
  ]) {
    assert.match(metadataExports, new RegExp(token.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const token of [
    "read_export_value",
    "read_reexport_source",
    "find_balanced_delimiter",
    "identifier_after",
  ]) {
    assert.match(metadataScanner, new RegExp(token.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.doesNotMatch(receipt, /next_core::/);
  assert.doesNotMatch(receipt, /next_runtime/);
  assert.doesNotMatch(receipt, /node_modules: true/);
  assert.doesNotMatch(receipt, /"runtime_takeover": true/);
  assert.doesNotMatch(receipt, /"react_required": true/);
  assert.doesNotMatch(receipt, /"rsc_required": true/);
  assert.doesNotMatch(receipt, /"node_required": true/);
  assert.doesNotMatch(receipt, /"node_modules_required": true/);
  assert.doesNotMatch(receipt, /"full_nextjs_runtime_parity": true/);
  assert.doesNotMatch(contract, /"runtime_takeover": true/);
  assert.doesNotMatch(contract, /"react_required": true/);
  assert.doesNotMatch(contract, /"rsc_required": true/);
  assert.doesNotMatch(contract, /"node_required": true/);
  assert.doesNotMatch(contract, /"node_modules_required": true/);
  assert.doesNotMatch(contract, /"full_nextjs_runtime_parity": true/);
  assert.doesNotMatch(contract, /"source_rewrite_performed": true/);
  assert.doesNotMatch(contract, /"next_proxy_generated": true/);
  assert.doesNotMatch(contract, /"rsc_runtime_proxy_generated": true/);
  assert.doesNotMatch(contract, /"font_css_import_generated": true/);
  assert.doesNotMatch(contract, /"dynamic_loadable_generated": true/);
  assert.doesNotMatch(receipt, /INLINE_FUNCTION_SERVER_RE|INLINE_ARROW_SERVER_RE/);
  assert.doesNotMatch(
    receipt,
    /EXPORT_FUNCTION_RE|EXPORT_CONST_CALLABLE_RE|ExportedCallable|fn exported_callables|fn push_server_action/,
  );
  assert.doesNotMatch(receipt, /EXPORT_CONST_RE/);
  assert.doesNotMatch(receipt, /fn dynamic_import_specifiers|fn next_dynamic_bindings/);
  assert.doesNotMatch(receipt, /import\.source == "next\/font\/google"/);
  assert.doesNotMatch(receipt, /source\.source\.contains\("export const metadata"\)/);
  assert.doesNotMatch(
    receipt,
    /fn source_has_hook_or_event|fn top_level_directives|fn quoted_statement_value/,
  );
  assert.doesNotMatch(receipt, /fn collect_conflicts|fn conflict\(|fn grouped_names/);
});
