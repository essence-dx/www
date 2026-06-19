import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

test("dx-style v4.3 CSS directive slice tracks custom variant block syntax", () => {
  const themeCss = read("related-crates/style/src/core/engine/theme_css.rs");
  const rustTest = read("related-crates/style/tests/tailwind_v4_css_first.rs");
  const ledger = read("related-crates/style/src/core/engine/directive_ledger.rs");
  const matrix = readJson("related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json");

  assert.match(
    themeCss,
    /fn parse_custom_variant_directives/,
    "theme CSS parser should scan top-level @custom-variant directives, not just line shorthands",
  );
  assert.match(
    themeCss,
    /fn parse_custom_variant_block/,
    "theme CSS parser should understand block-form @custom-variant syntax with @slot",
  );
  assert.match(
    rustTest,
    /css_first_custom_variant_block_(syntax_supports_slot_and_nested_media|directive_supports_slot_media_and_selector_forms)/,
    "focused Rust regression should cover block syntax, @slot, and nested @media",
  );
  assert.match(
    ledger,
    /@custom-variant any-hover \{ @media \(any-hover: hover\) \{ &:hover \{ @slot; \} \} \}/,
    "directive ledger should name the Tailwind documented nested-media custom variant canary",
  );

  const customVariant = (matrix.cssDirectiveCanaries ?? []).find(
    (entry) => entry.directive === "@custom-variant",
  );
  assert.ok(customVariant, "official fixture matrix should contain @custom-variant canaries");
  assert.ok(
    customVariant.canaries.includes(
      "@custom-variant any-hover { @media (any-hover: hover) { &:hover { @slot; } } }",
    ),
    "official fixture matrix should track block-form nested-media @custom-variant syntax",
  );
  assert.equal(
    customVariant.fullTailwindParity,
    false,
    "custom variant receipt should stay honest until ordering/escaping/full grammar are proven",
  );
});

test("dx-style v4.3 CSS directive slice applies @source inline safelists and exclusions", () => {
  const engine = read("related-crates/style/src/core/engine/mod.rs");
  const cssGenerator = read("related-crates/style/src/core/pipeline/css_generator.rs");
  const rustTest = read("related-crates/style/tests/tailwind_v4_css_first.rs");
  const ledger = read("related-crates/style/src/core/engine/directive_ledger.rs");
  const matrix = readJson("related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json");

  for (const marker of [
    "source_inline_safelists_and_exclusions_affect_generated_css",
    '@source inline("{hover:,focus:,}bg-brand p-{2..4..2} underline")',
    '@source not inline("focus:bg-brand p-2")',
  ]) {
    assert.match(rustTest, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "source_inline_class_tokens",
    "source_inline_exclusion_class_tokens",
    "class_is_source_inline_excluded",
  ]) {
    assert.match(engine, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(cssGenerator, /source_inline_class_tokens/);
  assert.match(cssGenerator, /class_is_source_inline_excluded/);

  for (const marker of [
    '@source inline(\\"{hover:,focus:,}bg-brand p-{2..4..2} underline\\")',
    '@source not inline(\\"focus:bg-brand p-2\\")',
  ]) {
    assert.match(ledger, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  const sourceInline = (matrix.cssDirectiveCanaries ?? []).find(
    (entry) => entry.directive === "@source inline(...)",
  );
  const sourceExclusion = (matrix.cssDirectiveCanaries ?? []).find(
    (entry) => entry.directive === "@source not ...",
  );
  assert.ok(sourceInline, "official fixture matrix should contain @source inline canaries");
  assert.ok(sourceExclusion, "official fixture matrix should contain @source not canaries");
  assert.ok(
    sourceInline.canaries?.includes('@source inline("{hover:,focus:,}bg-brand p-{2..4..2} underline")'),
    "official fixture matrix should track @source inline range and variant safelisting",
  );
  assert.ok(
    sourceExclusion.canaries?.includes('@source not inline("focus:bg-brand p-2")'),
    "official fixture matrix should track @source not inline exclusion precedence",
  );
  assert.equal(sourceInline.fullTailwindParity, false);
  assert.equal(sourceExclusion.fullTailwindParity, false);
});

test("dx-style v4.3 CSS directive slice exposes a static @source scan plan", () => {
  const themeCss = read("related-crates/style/src/core/engine/theme_css.rs");
  const coreExports = read("related-crates/style/src/core/mod.rs");
  const rustTest = read("related-crates/style/tests/tailwind_v4_css_first.rs");
  const ledger = read("related-crates/style/src/core/engine/directive_ledger.rs");
  const matrix = readJson("related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json");

  for (const marker of [
    "source_scan_plan_classifies_static_paths_none_and_inline_precedence",
    "css_source_scan_plan",
    '@source "../packages/ui"',
    '@source not "../legacy/**"',
    "effective_inline_classes",
  ]) {
    assert.match(rustTest, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "CssSourceScanPlan",
    "css_source_scan_plan",
    "disable_automatic_detection",
    "include_paths",
    "exclude_paths",
    "effective_inline_classes",
  ]) {
    assert.match(themeCss, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
  assert.match(coreExports, /CssSourceScanPlan/);
  assert.match(coreExports, /css_source_scan_plan/);

  for (const marker of [
    '@source \\"../packages/ui\\";',
    '@source not \\"../legacy/**\\";',
    "@source none;",
    "DX-owned scan plan",
  ]) {
    assert.match(ledger, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  const source = (matrix.cssDirectiveCanaries ?? []).find(
    (entry) => entry.directive === "@source",
  );
  assert.ok(source, "official fixture matrix should contain @source static canaries");
  assert.ok(
    source.canaries?.includes('@source "../packages/ui";'),
    "official fixture matrix should track explicit @source include paths",
  );
  assert.ok(
    source.canaries?.includes('@source not "../legacy/**";'),
    "official fixture matrix should track explicit @source exclude paths",
  );
  assert.ok(
    source.missingOrUnproven?.includes("source graph file IO and glob expansion"),
    "matrix should keep file IO/glob source traversal as unproven",
  );
  assert.equal(source.fullTailwindParity, false);
});
