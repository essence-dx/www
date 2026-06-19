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

test("dx-style owns a code-backed PostCSS compatibility matrix", () => {
  const fixture = JSON.parse(
    readRequiredFile("related-crates/style/fixtures/postcss-compat-matrix.json"),
  );
  const postcssCompat = readRequiredFile(
    "related-crates/style/src/core/engine/postcss_compat.rs",
  );
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const core = readRequiredFile("related-crates/style/src/core/mod.rs");

  assert.equal(fixture.schema, "dx.style.postcssCompatibilityMatrix");
  assert.equal(fixture.schemaVersion, 1);
  assert.equal(fixture.postcssRuntimeDependencyRequired, false);
  assert.equal(fixture.localPostcssConfigRequired, false);
  assert.equal(fixture.dxStarterReplacementScore, 100);
  assert.equal(fixture.dxStarterReplacementStatus, "complete-for-official-dx-starters");
  assert.equal(fixture.fullPostcssPluginParity, false);
  assert.equal(fixture.postcssPluginParityStatus, "not-claimed");
  assert.equal(fixture.autoprefixerParityStatus, "partial");
  assert.ok(Array.isArray(fixture.features));

  for (const feature of [
    "css-import-flattening",
    "nesting-transform",
    "nested-at-rule-transform",
    "nest-at-rule-transform",
    "custom-media",
    "compound-custom-media",
    "media-min-max-syntax",
    "strict-media-range-syntax",
    "mixed-media-range-syntax",
    "custom-selectors-safe",
    "logical-property-fallbacks",
    "logical-directional-fallbacks",
    "autoprefixer-style-prefixing",
    "expanded-prefix-families",
    "preset-env-future-css",
    "place-property-fallbacks",
    "image-set-prefix-fallback",
    "custom-property-env-fallbacks",
    "custom-property-var-fallbacks",
    "simple-selector-list-lowering",
    "not-selector-list-lowering",
    "color-mix-fallbacks",
    "hwb-color-fallbacks",
    "gradient-transparent-stop-fix",
    "grid-template-prefix-evidence",
    "selector-compat-diagnostics",
    "color-function-fallbacks",
    "gradient-transparency-compat",
    "page-break-fallbacks",
    "flex-grid-prefix-evidence",
    "sourcemap-source-origin",
    "minification",
  ]) {
    assert.ok(
      fixture.features.some((entry) => entry.feature === feature),
      `fixture missing ${feature}`,
    );
    assert.match(postcssCompat, new RegExp(feature.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "pub mod postcss_compat;",
    "pub struct PostcssCompatMatrixEntry",
    "pub enum PostcssCompatStatus",
    "pub enum DxStyleBrowserTarget",
    "pub struct PostcssCompatOptions",
    "pub struct PostcssCompatReceipt",
    "pub fn postcss_compat_matrix()",
    "pub fn transform_postcss_compatible_css(",
    "postcss_runtime_dependency_required: false",
    "local_postcss_config_required: false",
    "dx_starter_replacement_score: 100",
    'dx_starter_replacement_status: "complete-for-official-dx-starters"',
    "full_postcss_plugin_parity: false",
    'postcss_plugin_parity_status: "not-claimed"',
    'autoprefixer_parity_status: "partial"',
  ]) {
    assert.match(
      `${postcssCompat}\n${engine}\n${core}`,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
    );
  }
});

test("dx style receipts expose PostCSS replacement summary without local PostCSS config", () => {
  const publicTools = readRequiredFile("dx-www/src/cli/public_framework_tools.rs");
  const tools = readRequiredFile("dx-www/src/cli/public_framework_tools/dx_style.rs");
  const receipts = readRequiredFile("core/src/ecosystem/dx_style_receipts.rs");
  const projectCheck = [
    readRequiredFile("core/src/ecosystem/project_check/readiness_parts/dx_style.rs"),
    readRequiredFile("core/src/ecosystem/project_check/readiness_parts/tests.rs"),
  ].join("\n");
  const checkReceipt = readRequiredFile("core/src/ecosystem/dx_check_receipt/panel.rs");

  assert.match(publicTools, /mod dx_style;/);

  for (const marker of [
    '"postcss_compatibility_contract": dx_style_postcss_compatibility_contract()',
    '"postcss_compat_supported_count": postcss_compat.supported_count',
    '"postcss_compat_partial_count": postcss_compat.partial_count',
    '"dx_starter_replacement_score": postcss_compat.dx_starter_replacement_score',
    '"full_postcss_plugin_parity": postcss_compat.full_postcss_plugin_parity',
    '"postcss_plugin_parity_status": postcss_compat.postcss_plugin_parity_status',
    '"unsupported_transform_warnings": postcss_compat.unsupported_transform_warnings',
    '"local_postcss_config_required": false',
    '"postcss_runtime_dependency_required": false',
    "style::core::postcss_compatibility_contract()",
  ]) {
    assert.match(tools, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "DxStylePostcssCompatSummary",
    "dx_style_postcss_compat_summary",
    "postcss_compatibility_contract",
    "autoprefixer_parity_status",
    "dx_starter_replacement_score",
    "full_postcss_plugin_parity",
    "postcss_plugin_parity_status",
    "unsupported_transform_warnings",
  ]) {
    assert.match(receipts, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "let postcss_compat = dx_style_postcss_compat_summary(root);",
    "postcss_compat_supported_count",
    "postcss_compat_partial_count",
    "dx_starter_replacement_score",
    "full_postcss_plugin_parity",
    "dx-style-postcss-compat-full-plugin-overclaim",
    "unsupported_transform_warnings",
    "dx-style-postcss-compat-unsupported-transforms",
    "dx_style_section_summarizes_postcss_compat_receipt",
  ]) {
    assert.match(projectCheck, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "postcss_compat_supported_count",
    "postcss_compat_partial_count",
    "postcss_compat_unsupported_count",
    "dx_starter_replacement_score",
    "full_postcss_plugin_parity",
    "postcss_runtime_dependency_required",
    "local_postcss_config_required",
    "unsupported_transform_warnings",
  ]) {
    assert.match(checkReceipt, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
});

test("public wording stays scoped to DX starters instead of broad PostCSS parity", () => {
  const readme = readRequiredFile("related-crates/style/README.md");
  const rootReadme = readRequiredFile("README.md");

  assert.match(`${readme}\n${rootReadme}`, /PostCSS replacement for DX starters/);
  assert.doesNotMatch(`${readme}\n${rootReadme}`, /full PostCSS parity/i);
  assert.doesNotMatch(`${readme}\n${rootReadme}`, /full Autoprefixer parity/i);
});
