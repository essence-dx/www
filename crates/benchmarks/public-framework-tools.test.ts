const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function readRequiredFile(relativePath) {
  const filePath = path.join(root, relativePath);
  assert.ok(fs.existsSync(filePath), `expected ${relativePath} to exist`);
  return fs.readFileSync(filePath, "utf8");
}

function readDxStyleTools() {
  const publicTools = readRequiredFile("dx-www/src/cli/public_framework_tools.rs");
  const dxStyleTools = readRequiredFile("dx-www/src/cli/public_framework_tools/dx_style.rs");
  assert.match(publicTools, /mod dx_style;/);
  return dxStyleTools;
}

function readPublicFrameworkToolsWithDxStyle() {
  return `${readRequiredFile("dx-www/src/cli/public_framework_tools.rs")}\n${readRequiredFile("dx-www/src/cli/public_framework_tools/imports.rs")}\n${readDxStyleTools()}`;
}

function readCoreProjectCheckSources() {
  return [
    "core/src/ecosystem/project_check.rs",
    "core/src/ecosystem/project_check/readiness.rs",
    "core/src/ecosystem/project_check/readiness_parts/dx_style.rs",
    "core/src/ecosystem/project_check/readiness_parts/tests.rs",
  ]
    .map(readRequiredFile)
    .join("\n");
}

test("dx style check gates missing public tokens and unused generated classes", () => {
  const tools = readPublicFrameworkToolsWithDxStyle();
  const projectCheck = readCoreProjectCheckSources();

  assert.match(tools, /const REQUIRED_THEME_TOKENS: \[&str; 11\]/);
  for (const token of [
    "--background",
    "--foreground",
    "--surface",
    "--muted",
    "--border",
    "--card",
    "--accent",
    "--success",
    "--warning",
    "--danger",
    "--spacing",
  ]) {
    assert.match(tools, new RegExp(token));
  }

  assert.match(tools, /fn missing_theme_tokens\(project: &Path\) -> anyhow::Result<Vec<String>>/);
  assert.match(tools, /let missing_theme_tokens = missing_theme_tokens\(project\)\?/);
  assert.match(tools, /"missing_theme_tokens": missing_theme_tokens/);
  assert.match(tools, /missing_theme_tokens\.is_empty\(\)/);
  assert.match(tools, /unused_classes\.is_empty\(\)/);
  assert.match(tools, /report\.json\.get\("passed"\)\.and_then\(Value::as_bool\) == Some\(false\)/);
  assert.match(tools, /DX public tool check/);
  assert.match(tools, /"DX style check\\nPassed: \{passed\}\\nGenerated CSS stale: \{stale_generated_css\}\\nMissing theme tokens: \{\}/);

  assert.match(projectCheck, /fn dx_style_section\(root: &Path\) -> Result<DxCheckSection>/);
  assert.match(projectCheck, /const REQUIRED_DX_STYLE_TOKENS: \[&str; 9\]/);
  assert.match(projectCheck, /"dx-style"/);
  assert.match(projectCheck, /dx-style-hardcoded-color/);
  assert.match(projectCheck, /dx-style-tailwind-leakage/);
  assert.match(projectCheck, /dx-style-stale-generated-css/);
  assert.match(projectCheck, /dx-style-unused-generated-class/);
  assert.match(projectCheck, /dx_style_required_tokens_missing/);
  assert.match(projectCheck, /styles\/theme\.css/);
  assert.match(projectCheck, /styles\/generated\.css/);
});

test("core dx-check summarizes dx-style Tailwind parity receipt evidence", () => {
  const projectCheck = readCoreProjectCheckSources();
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");

  for (const marker of [
    'const DX_STYLE_CHECK_RECEIPT_PATH: &str = ".dx/receipts/style/check.json"',
    'const DX_STYLE_TAILWIND_PARITY_SCHEMA: &str = "dx.style.tailwind-parity"',
    "struct DxStyleTailwindParitySummary",
    "fn dx_style_tailwind_parity_summary(root: &Path) -> DxStyleTailwindParitySummary",
    "dx_style_tailwind_parity_receipt_present",
    "dx_style_tailwind_parity_contract_present",
    "dx_style_tailwind_parity_schema_supported",
    "dx_style_tailwind_parity_supported_classes",
    "dx_style_tailwind_parity_state_alias_supported_classes",
    "dx_style_tailwind_parity_unsupported_classes",
    "dx_style_tailwind_parity_intentional_differences",
    "supported_state_alias_examples",
    "tailwind_parity_supported_state_alias_examples",
    "dx-style-tailwind-parity-contract-missing",
    "dx-style-tailwind-parity-unsupported-fixtures",
    "dx-style-tailwind-parity-intentional-differences",
    "dx_style_section_summarizes_tailwind_parity_receipt",
  ]) {
    assert.match(projectCheck, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(projectCheck, /unsupported_class_examples[\s\S]*\[@unknown_rule\]:p-4/);
  assert.match(projectCheck, /intentionally_different_examples[\s\S]*container/);
  assert.match(projectCheck, /"class_name": "target:p-4"[\s\S]*"status": "supported"/);
  assert.match(projectCheck, /"class_name": "has-even:bg-blue-500"[\s\S]*"status": "supported"/);
  assert.match(projectCheck, /metric_value\(\s*&section,\s*"dx_style_tailwind_parity_state_alias_supported_classes"[\s\S]*Some\(6\)/);
  assert.match(projectCheck, /tailwind_parity\.supported_state_alias_examples[\s\S]*target:p-4[\s\S]*in-read-only:p-4/);
  assert.match(dx, /dx-check parity summary pass/i);
  assert.match(dx, /state alias dx-check parity summary pass/i);
  assert.match(todo, /dx-check Tailwind parity summary/i);
  assert.match(todo, /state alias dx-check parity summary/i);
  assert.match(changelog, /Summarized dx-style Tailwind parity receipts/i);
  assert.match(changelog, /state alias dx-check parity summary/i);
});

test("core dx-check surfaces unsupported dx-style scanned utility classes", () => {
  const projectCheck = readCoreProjectCheckSources();
  const styleReceipt = readRequiredFile("core/src/ecosystem/dx_style_receipts.rs");
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");

  for (const marker of [
    "struct DxStyleUnsupportedScannedClassSummary",
    "dx_style_unsupported_scanned_class_receipt_present",
    "dx_style_unsupported_scanned_classes",
    "dx-style-unsupported-scanned-class",
    "dx_style_section_summarizes_unsupported_scanned_class_receipts",
  ]) {
    assert.match(
      `${projectCheck}\n${styleReceipt}`,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
    );
  }

  assert.match(styleReceipt, /pub fn dx_style_unsupported_scanned_classes_summary[\s\S]*root: &Path/);
  assert.match(styleReceipt, /unsupported_scanned_class_findings/);
  assert.match(styleReceipt, /class_name[\s\S]*reason/);
  assert.match(dx, /dx-check unsupported scanned utility summary/i);
  assert.match(todo, /dx-check unsupported scanned utility summary/i);
  assert.match(changelog, /Surfaced unsupported dx-style scanned utilities/i);
});

test("dx-style CSS-first directive compatibility is source-owned and diagnosed", () => {
  const tools = readDxStyleTools();
  const styleCore = readRequiredFile("related-crates/style/src/core/engine/theme_css.rs");
  const featureMatrix = readRequiredFile("related-crates/style/src/core/engine/feature_matrix.rs");
  const styleExports = readRequiredFile("related-crates/style/src/core/mod.rs");
  const parser = readRequiredFile("dx-www/src/parser/style.rs");
  const projectCheck = readCoreProjectCheckSources();
  const styleReceipt = readRequiredFile("core/src/ecosystem/dx_style_receipts.rs");
  const compatibility = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");

  for (const marker of [
    "css_source_inline_class_tokens",
    "css_first_directive_diagnostics",
    "CssSourceDirective::Inline",
    "CssSourceDirective::Exclude",
    "CssSourceDirective::DisableAutomaticDetection",
    "css_source_disables_automatic_detection",
    "CssUtilityDefinition",
    "CssCustomVariant",
    "DX_STYLE_UNSUPPORTED_CSS_DIRECTIVE_RULE",
    "unsupported_css_directive_count",
    "unsupported_css_directive_findings",
    "dx_style_unsupported_css_directives_summary",
    "dx_style_unsupported_css_directives",
    "dx-style-unsupported-css-directive",
    "tailwindcss_import_policy",
    "migration-input-stripped",
    "source(none)",
    "CssDependencyDirective::Reference",
    "css_first_local_reference_feeds_theme_tokens_without_tailwind_runtime",
    "reference_directive_support",
    "local_css_files",
    "source_detection_mode",
    "tailwind_plugin_ecosystem_parity",
    "parse_variant_rule",
    "dx-style @apply currently supports plain or variant-safe resolvable utility tokens",
    "css_authored_function_rules_from_source",
    "css_variant_rules_from_source",
    "Authored CSS function transforms",
    "Authored CSS @variant transforms",
    "local CSS @reference theme-token flattening in the public build path",
    "standalone authored CSS --alpha() could not be transformed safely",
    "standalone authored CSS --spacing() could not be transformed safely",
  ]) {
    assert.match(
      `${tools}\n${styleCore}\n${featureMatrix}\n${styleExports}\n${parser}\n${projectCheck}\n${styleReceipt}`,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
    );
  }

  assert.match(compatibility, /@import "tailwindcss"[\s\S]*migration input/i);
  assert.match(compatibility, /@reference "\.\/tokens\.css"[\s\S]*without Tailwind runtime/i);
  assert.match(compatibility, /@source inline\(\.\.\.\)[\s\S]*Partial/i);
  assert.match(compatibility, /@custom-variant[\s\S]*Partial/i);
  assert.match(compatibility, /JS config[\s\S]*Unsupported-by-design/i);
  assert.match(dx, /CSS-first directive compatibility/i);
  assert.match(todo, /CSS-first directive compatibility/i);
  assert.match(changelog, /CSS-first directive compatibility/i);
});

test("public CLI advertises React TSX, dx-style, imports, web-perf, and Vercel deploy lanes", () => {
  const cli = readRequiredFile("dx-www/src/cli/mod.rs");
  const tools = readPublicFrameworkToolsWithDxStyle();

  assert.match(cli, /dx style build\|watch\|check/);
  assert.match(cli, /dx imports sync\|check/);
  assert.match(cli, /dx deploy vercel/);
  assert.match(`${cli}\n${tools}`, /dx check web-perf --from-lighthouse report\.json/);
  assert.match(tools, /"public_launch_path": "tsx-app-router"/);
  assert.match(tools, /"binary_style_output": "deprecated"/);
  assert.match(tools, /"collector": "rust-chrome-devtools-protocol"/);
  assert.match(tools, /"generated-import-maps-no-runtime-magic"/);
  assert.match(tools, /"vercel deploy --prebuilt"/);
});

test("dx imports generates a deterministic typed map instead of runtime magic", () => {
  const tools = readRequiredFile("dx-www/src/cli/public_framework_tools/imports.rs");

  for (const marker of [
    "struct ImportEntry",
    "dxAutoImportMap",
    "DxAutoImportComponent",
    "DxAutoImportForgePackage",
    "DxAutoImportStyleHelper",
    "public_exports_from_source",
    "import_map_source_hash",
    "\"generated_by\": \"dx imports sync\"",
    "\"source_hash\": &imports.source_hash",
    "\"entries\": imports.entries.iter().map(import_entry_json)",
    "\"forge-package\".to_string()",
    "\"style-helper\".to_string()",
    "\"dx-icon\"",
  ]) {
    assert.match(tools, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  const importsJsonFunction = tools.match(/fn imports_json[\s\S]*?fn imports_report/);
  assert.ok(importsJsonFunction, "imports_json function should exist");
  assert.doesNotMatch(importsJsonFunction[0], /generated_at/);
});

test("dx deploy vercel performs a guarded local preflight pipeline", () => {
  const tools = readRequiredFile("dx-www/src/cli/public_framework_tools.rs");

  for (const marker of [
    "build_dx_style(project, PublicToolFormat::Json, false)",
    "check_dx_style(project, PublicToolFormat::Json)",
    "sync_dx_imports(project, PublicToolFormat::Json)",
    "check_dx_imports(project, PublicToolFormat::Json)",
    "inspect_static_output(project, &output_dir)",
    "materialize_vercel_build_output(project, &output_dir, &output_label, &static_output)",
    "skipped_vercel_build_output_materialization(",
    "public_runtime_artifact_plan(output_dir, &static_output.upload_plan)",
    "copy_public_runtime_artifacts(output_dir, &vercel_static_dir, &public_runtime_plan.paths)",
    "use super::deploy_adapter_contract::DX_CLOUD_PROVIDER_ADAPTER_JSON;",
    "use super::readiness::READINESS_PROOF_GRAPH_RECEIPT;",
    "\"bundle_partition_source\": public_runtime_plan.source",
    "\"public_runtime_artifacts\": public_runtime_plan.paths",
    "\"evidence_artifact_count\": public_runtime_plan.evidence_artifact_count",
    "\"evidence_excluded_from_public_output\": true",
    "\"preflight_passed\": preflight_passed",
    "\"ready_for_deploy\": ready_for_deploy",
    "\"static_export\"",
    "\"upload_plan\": static_output.upload_plan",
    "\"vercel_build_output\": build_output_materialization.clone()",
    "vercel_prebuilt_deploy_contract(",
    "fn materialize_vercel_build_output(",
    "fn skipped_vercel_build_output_materialization(",
    "fn vercel_build_output_config() -> Value",
    "fn copy_public_runtime_artifacts(",
    "fn public_runtime_artifact_plan(",
    "fn normalized_public_artifact_path(path: &str) -> anyhow::Result<String>",
    "fn is_evidence_artifact_path(path: &str) -> bool",
    "normalized == \"page-graph.json\"",
    "normalized.contains(\"/page-graph.json\")",
    "normalized == READINESS_PROOF_GRAPH_RECEIPT",
    "normalized.starts_with(\".dx/\")",
    "normalized.starts_with(\".dx/build-cache/source-routes/\")",
    "fn ensure_generated_project_path(path: &Path, expected_suffix: &str) -> anyhow::Result<()>",
    "fn vercel_prebuilt_argv(prod: bool) -> Vec<&'static str>",
    "\"vercel_prebuilt_contract\": deploy_contract.clone()",
    "\"contract\": \"dx.vercel.prebuiltStatic\"",
    "\"build_output_api\"",
    "\"version\": 3",
    "\"static_dir\": \".vercel/output/static\"",
    "\"config_path\": \".vercel/output/config.json\"",
    "\"src\": \"/\"",
    "\"dest\": \"/index.html\"",
    "\"src\": \"/([^/.]+(?:/[^/.]+)*)/?\"",
    "\"dest\": \"/$1/index.html\"",
    "\"adapter_output\": \".vercel/output\"",
    "\"copy_files_to\": \".vercel/output/static\"",
    "\"preserves\": [\".vercel/project.json\", \".vercel/.gitignore\"]",
    "\"cleans_only\": \".vercel/output\"",
    "std::fs::remove_dir_all(&vercel_output_dir)",
    "&vercel_output_dir.join(\"config.json\")",
    "&vercel_build_output_config()",
    "\"argv\": vercel_prebuilt_argv(prod)",
    "\"requires_env\": [\"VERCEL_TOKEN\"]",
    "\"execution_allowed_by_this_invocation\": false",
    "\"command_contract\": \".dx/deploy/vercel-command.json\"",
    "write_json_receipt(&deploy_command_path, &deploy_contract)",
    "\"blocked_without_explicit_deploy\": !dry_run",
    "fn static_content_type(relative: &str) -> &'static str",
    "fn static_cache_policy(relative: &str) -> &'static str",
    "materialize_vercel_build_output_keeps_tiny_static_public_and_evidence_private",
    "app/page-graph.json",
    "app/app-router-execution.json",
    ".dx/build-cache/source-routes/root/route-unit.json",
  ]) {
    assert.match(tools, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.doesNotMatch(tools, /Command::new\("vercel"\)/);
  assert.doesNotMatch(tools, /npm install/);
  assert.doesNotMatch(tools, /copy_directory_contents\(output_dir, &vercel_static_dir\)/);
});

test("web performance receipts include vitals and network evidence", () => {
  const tools = readRequiredFile("dx-www/src/cli/public_framework_tools.rs");

  for (const marker of [
    "web_perf_cdp_collector_plan",
    "web_perf_device_profiles",
    "const LIGHTHOUSE_SCORE_CATEGORIES: [&str; 4]",
    "struct LighthouseCategoryScores",
    "fn lighthouse_category_scores(value: &Value) -> LighthouseCategoryScores",
    "\"partial-lighthouse-json-missing-score-categories\"",
    "\"score_completeness\"",
    "\"required_categories\": LIGHTHOUSE_SCORE_CATEGORIES",
    "\"missing_categories\": &category_scores.missing_categories",
    "\"policy\": \"URL mode does not claim Lighthouse category totals until a live CDP collection or Lighthouse JSON import exists\"",
    "\"policy\": \"do not claim a 400-point total unless every required Lighthouse category is present\"",
    "category_scores.total()",
    "lighthouse_audit_numeric_value",
    "lighthouse_request_count",
    "lighthouse_transfer_bytes",
    "total-byte-weight",
    "\"core_web_vitals\"",
    "\"first_contentful_paint_ms\"",
    "\"largest_contentful_paint_ms\"",
    "\"cumulative_layout_shift\"",
    "\"total_blocking_time_ms\"",
    "\"speed_index_ms\"",
    "\"network\"",
    "\"request_count\"",
    "\"transfer_size_bytes\"",
    "\"collector_plan\"",
    "\"launches_browser\": false",
    "\"requires_running_chrome_debug_port\": true",
    "\"discovery_endpoint\": \"http://127.0.0.1:<debug-port>/json/version\"",
    "\"websocket_endpoint_source\": \"webSocketDebuggerUrl\"",
    "\"cdp_domains\"",
    "\"Browser\"",
    "\"Target\"",
    "\"Page\"",
    "\"Performance\"",
    "\"Network\"",
    "\"Runtime\"",
    "\"Browser.getVersion\"",
    "\"Target.createTarget\"",
    "\"Performance.getMetrics\"",
    "\"Runtime.evaluate\"",
    "\"url_mode_scores\": \"null until a live CDP collection or Lighthouse JSON import exists\"",
    "\"cdp_plan\": \".dx/receipts/check/web-perf/cdp-plan.json\"",
    "write_json_receipt(",
    "\"score_estimated\"",
    "web_perf_score_estimated(report)",
    "fn web_perf_raw_lighthouse_json(report: &Value) -> String",
    "(\"raw_lighthouse_json\", web_perf_raw_lighthouse_json(report))",
    "let web_perf_proof = web_perf_receipt_proof(project)?",
    "web_perf_report_measured(&report)",
    "if !web_perf_measured",
  ]) {
    assert.match(tools, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(tools, /Core Web Vitals:/);
  assert.match(tools, /Requests:/);
  assert.match(tools, /Transfer:/);
  assert.match(tools, /CDP plan:/);
  assert.doesNotMatch(tools, /Command::new\("chrome"\)/);
  assert.doesNotMatch(tools, /lighthouse npm/i);
});
