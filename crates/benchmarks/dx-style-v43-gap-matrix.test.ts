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

function readOptionalFile(relativePath) {
  const filePath = path.join(root, relativePath);
  return fs.existsSync(filePath) ? fs.readFileSync(filePath, "utf8") : "";
}

test("dx-style Tailwind v4.3 gap matrix is explicit and not hype", () => {
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");
  const styleReadme = readRequiredFile("related-crates/style/README.md");

  for (const marker of [
    "Latest Tailwind truth verified for this matrix: `tailwindcss@4.3.0`",
    "| CSS-first `@theme` and import flattening | Supported |",
    "| Static class scanning and unsupported diagnostics | Partial |",
    "| Full Tailwind v4.3 utility grammar | Partial |",
    "| Full OKLCH/P3/new palette parity | Partial |",
    "| `@source inline(...)` and `@source not ...` | Partial |",
    "| `@utility` CSS directive | Partial |",
    "| `@custom-variant` CSS directive | Partial |",
    "| CSS `@variant` directive | Partial |",
    "| Full CSS directive parity | Partial |",
    "| Out-of-scope JS config and plugin directives | Unsupported-by-design diagnostic boundary |",
    "| Advanced CSS theme-token extension parity | Partial |",
    "| Official Tailwind fixture matrix | Supported |",
    "| Governed live Tailwind output comparison | Supported |",
    "not a complete Tailwind v4.3 replacement",
  ]) {
    assert.match(matrix, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(styleReadme, /Tailwind v4\.3 compatibility matrix/i);
  assert.match(styleReadme, /CSS directive ledger/i);
  assert.match(styleReadme, /does not depend on Tailwind/i);
  assert.match(styleReadme, /scoped official-starter compatibility evidence/i);
  assert.match(styleReadme, /not proof of arbitrary PostCSS plugin parity/i);
  assert.match(styleReadme, /Non-CSS boundaries are separate/i);
  assert.doesNotMatch(matrix, /drop-in Tailwind v4\.3 replacement|complete Tailwind v4\.3 parity/i);
  assert.doesNotMatch(
    styleReadme,
    /is a drop-in Tailwind replacement|drop-in replacement for Tailwind|complete Tailwind replacement/i,
  );
  const riskyOfficialStarterClaim =
    String.raw`100\/100 official` + String.raw`-starter compatibility contract`;
  const riskyTailwindPostcssClaim = String.raw`Tailwind\/PostCSS ` + "perfection";
  const riskyUniversalPostcssClaim = "universal PostCSS " + "parity";
  assert.doesNotMatch(
    styleReadme,
    new RegExp(
      `${riskyOfficialStarterClaim}|${riskyTailwindPostcssClaim}|${riskyUniversalPostcssClaim}`,
      "i",
    ),
  );
});

test("dx-style gap canaries are executable source-owned tests", () => {
  const core = readRequiredFile("related-crates/style/src/core/mod.rs");
  const featureMatrix = readRequiredFile(
    "related-crates/style/src/core/engine/feature_matrix.rs",
  );
  const rustTest = readRequiredFile(
    "related-crates/style/tests/tailwind_v43_gap_matrix.rs",
  );
  const cssFirstTest = readRequiredFile(
    "related-crates/style/tests/tailwind_v4_css_first.rs",
  );

  for (const marker of [
    "tailwind_v43_feature_matrix",
    "TailwindV43FeatureStatus::UnsupportedByDesign",
    "Tailwind docs-table ledger",
    "Tailwind v4.3 neutral-adjacent palette fixture",
    "mauve/olive/mist/taupe OKLCH token-backed fixture",
    "display-p3 output fixture",
    "direction/open/inert selector fixture",
    "user-valid/user-invalid/details-content selector fixture",
    "group/peer pseudo-class variant fixture",
    "benchmarks/dx-style-v43-variant-selector-parity.test.ts",
    "safe unknown arbitrary at-rule fixture",
    "negated arbitrary media/supports/container at-rule fixture",
    "Tailwind directive arbitrary variant fail-closed fixture",
    "stacked arbitrary selector composition fixture",
    "selector-list arbitrary variant fixture",
    "stacked arbitrary group/peer selector fixture",
    "escaped arbitrary selector fixture",
    "Autoprefixer equal-output fixture",
    "class-scanning-and-diagnostics",
    "sourceScannerCanaries",
    "tsx-static-object-map",
    "tsx-static-array-and-helper-literals",
    "plain_text_extraction_reads_static_arrays_object_maps_and_helpers",
    "plain_text_extraction_rejects_dynamic_fragments_and_prose",
    "arbitrary-value-static-string",
    "template-interpolation-rejection",
    "dynamic-fragment-diagnostic",
    "source-scan-diagnostic-receipt",
    "plain_text_diagnostics_report_dynamic_object_key_unsafe_prose_and_duplicates",
    "reports_plain_text_source_scan_diagnostics_with_locations",
    "dynamic class fragment was scanned but cannot be generated statically",
    "unsupported_scan_reports_dynamic_class_fragments_as_source_boundaries",
    "source-inline-and-not-directives",
    "@import \"tailwindcss\" source(none)",
    "source-not-inline-exclusion",
    "custom-utility-directive",
    "custom-variant-directive",
    '@custom-variant theme-midnight { &:where([data-theme=\\"midnight\\"] *) { @slot; } }',
    "@custom-variant any-hover { @media (any-hover: hover) { &:hover { @slot; } } }",
    "multiple @slot selector-list expansion",
    "css-variant-directive",
    "@variant hover:focus",
    "@variant hover, focus",
    "@variant md:theme-midnight",
    "tailwind package dependency leakage",
    "official-plugin-ecosystem",
    "external Tailwind plugin code is out of scope",
    "@plugin/@config unsupported diagnostics",
    "DX-owned prose/forms/aspect behavior must be source-owned CSS",
    "advanced-css-theme-token-extensions",
    "css @theme custom animation alias fixture",
    "css @theme transition token fixture",
    "css @theme container-query token fixture",
    "css grid edge grammar fixture",
    "official-fixture-matrix",
    "tailwind-v43-official-fixture-matrix.json",
    "governed-live-tailwind-output-comparison",
    "tools/dx-style/live-tailwind-v43-compare.cjs",
    "tailwind_v43_utility_ledger",
    "TAILWIND_V43_UTILITY_LEDGER_BASELINE",
    "tailwind_v43_css_directive_ledger",
    "TAILWIND_V43_CSS_DIRECTIVE_LEDGER_BASELINE",
    "TAILWIND_V43_CSS_DIRECTIVE_LEDGER_SCOPE",
    "not full CSS directive parity",
    "@apply",
    "@reference",
    "--alpha()",
    "--spacing()",
    "TAILWIND_V43_FEATURE_MATRIX_BASELINE",
    "tailwindcss-4.3.0",
    "Tailwind v4.3 neutral-adjacent palette canaries",
  ]) {
    assert.match(
      `${core}\n${featureMatrix}\n${rustTest}`,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
    );
  }

  assert.match(rustTest, /partial_css_first_directives_are_named_as_partial_scope/);
  assert.match(rustTest, /unsupported_js_directives_emit_css_first_diagnostics/);
  assert.match(cssFirstTest, /source_inline_supports_ranges_and_explicit_inline_exclusions/);
  assert.match(cssFirstTest, /source_none_disables_automatic_detection_as_dx_owned_scan_policy/);
  assert.match(cssFirstTest, /css_source_inline_exclusion_class_tokens/);
  assert.match(cssFirstTest, /@source not inline/);
  assert.match(
    cssFirstTest,
    /css_first_custom_variant_block_directive_supports_slot_media_and_selector_forms/,
  );
  assert.match(cssFirstTest, /tailwind_v43_neutral_palettes_generate_token_backed_color_utilities/);
  assert.match(cssFirstTest, /bg-mauve-500/);
  assert.match(cssFirstTest, /ring-taupe-400\/50/);
  assert.match(rustTest, /group_and_peer_state_variants_cover_tailwind_pseudo_class_families/);
  assert.match(rustTest, /group-odd:bg-mauve-500/);
  assert.match(rustTest, /peer-required\/email:block/);
  assert.match(rustTest, /arbitrary_group_and_peer_variants_cover_tailwind_v4_wrapper_families/);
  assert.match(rustTest, /group-\[\.is-published\]:block/);
  assert.match(rustTest, /peer-\[\.is-dirty\]:peer-required:block/);
  assert.match(rustTest, /arbitrary_selector_list_variants_cover_branch_composition/);
  assert.match(rustTest, /\[\&\.foo,\&\.bar\]:\[\&>\.item,\&>\[data-slot=control\]\]:opacity-100/);
  assert.match(rustTest, /group-\[\&\.foo,\&\.bar\]:block/);
  for (const marker of [
    "--animate-shimmer",
    "animate-shimmer",
    "--container-dashboard",
    "@dashboard:flex",
    "--ease-enter",
    "duration-fast",
    "grid-cols-(--dx-grid-cols)",
  ]) {
    assert.match(
      cssFirstTest,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `${marker} should be guarded as a CSS-owned advanced theme/utility fixture`,
    );
  }
  assert.doesNotMatch(
    `${featureMatrix}\n${rustTest}`,
    /theme\.extend|plugin theme\(\)/,
    "advanced dx-style fixtures should not depend on Tailwind JS config/plugin wording",
  );
  assert.match(rustTest, /unsupported_directive_canaries_do_not_silently_generate_utility_css/);
  assert.match(rustTest, /engine\.css_for_class\(directive\)\.is_none\(\)/);
});

test("dx-style live Tailwind comparison is governed and dependency-isolated", () => {
  const harness = require(path.join(root, "tools/style/tailwind-live-comparison.cjs"));
  const source = readRequiredFile("tools/style/tailwind-live-comparison.cjs");
  const matrix = JSON.parse(
    readRequiredFile("related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json"),
  );
  const officialInventory = JSON.parse(readRequiredFile(matrix.officialSource.candidateInventory));
  const matrixRunnerSource = readRequiredFile(matrix.liveComparison.runner);
  const report = harness.runComparison({ root, live: false });

  assert.equal(report.schema, "dx.style.liveTailwindOutputComparison");
  assert.equal(report.baseline, "tailwindcss@4.3.0");
  assert.equal(report.tailwindCliPackage, "@tailwindcss/cli@4.3.0");
  assert.equal(report.fixturePath, "related-crates/style/fixtures/tailwind-equal-output-canary.json");
  assert.equal(report.liveTailwindExecuted, false);
  assert.equal(report.normalTestsRunLiveTailwind, false);
  assert.equal(report.fullTailwindParity, false);
  assert.ok(report.classCount > 0, "live harness should read checked-in equal-output canaries");
  assert.match(report.runPolicy, /DX_STYLE_RUN_LIVE_TAILWIND=1/);

  assert.match(source, /@tailwindcss\/cli@4\.3\.0/);
  assert.match(source, /tailwindcss@4\.3\.0/);
  assert.match(source, /spawnSync/);
  assert.match(source, /@source inline/);

  assert.equal(matrix.schema, "dx.style.tailwindOfficialFixtureMatrix");
  assert.equal(matrix.tailwindPackage.version, "4.3.0");
  assert.equal(matrix.tailwindCliPackage.version, "4.3.0");
  assert.equal(matrix.tailwindRuntimeDependency, false);
  assert.equal(matrix.officialSource.ingestionTool, "tools/dx-style/ingest-tailwind-v43-fixtures.cjs");
  assert.equal(officialInventory.schema, "dx.style.tailwindOfficialCandidateInventory");
  assert.ok(officialInventory.candidateCount >= 8000);
  assert.ok(officialInventory.sourceFileCount >= 70);
  assert.equal(
    officialInventory.officialFixtureMatrix?.schema,
    "dx.style.tailwindOfficialFixtureMatrix",
  );
  assert.ok(officialInventory.officialFixtureMatrix.fixtureCount >= 1000);
  assert.ok(officialInventory.officialFixtureMatrix.fixtureSourceFileCount >= 40);
  assert.equal(matrix.liveComparison.tailwindInstallScope, "temporary-directory-only");
  assert.equal(
    matrix.liveComparison.normalTest,
    "benchmarks/dx-style-live-tailwind-v43-comparison.test.ts",
  );
  assert.equal(matrix.liveComparison.normalTestRunsLiveTailwind, true);
  assert.ok(matrix.classes.length >= 20, "official fixture matrix should be non-trivial");
  assert.equal(matrix.officialFixtureTruth.inventory, matrix.officialSource.candidateInventory);
  assert.equal(matrix.officialFixtureTruth.candidateCount, officialInventory.candidateCount);
  assert.equal(
    matrix.officialFixtureTruth.fixtureCount,
    officialInventory.officialFixtureMatrix.fixtureCount,
  );
  assert.equal(matrix.officialFixtureTruth.snapshotOutputPolicy, "fingerprinted-not-vendored");
  assert.equal(matrix.officialFixtureTruth.fullTailwindParity, false);
  assert.match(matrix.liveComparison.command, /tailwindcss@4\.3\.0/);
  assert.match(matrix.liveComparison.command, /@tailwindcss\/cli@4\.3\.0/);
  assert.match(matrixRunnerSource, /npmCommand\(\)/);
  assert.match(matrixRunnerSource, /"install"/);
  assert.match(matrixRunnerSource, /"--no-save"/);
  assert.match(matrixRunnerSource, /fs\.mkdtempSync/);
  assert.match(matrixRunnerSource, /matrix\.tailwindPackage\.name/);
  assert.match(matrixRunnerSource, /matrix\.tailwindCliPackage\.name/);
  assert.match(matrixRunnerSource, /dx_style_fixture_css/);
  assert.match(matrixRunnerSource, /liveTailwindExecution:\s*true/);
});

test("dx-style manifests do not depend on Tailwind", () => {
  const manifests = [
    ["related-crates/style/Cargo.toml", readRequiredFile("related-crates/style/Cargo.toml")],
    ["related-crates/style/package.json", readOptionalFile("related-crates/style/package.json")],
  ];

  for (const [relativePath, contents] of manifests) {
    assert.doesNotMatch(
      contents,
      /(^|\n)\s*["']?tailwindcss["']?\s*[:=]/,
      `${relativePath} should not declare a Tailwind package dependency`,
    );
  }
});
