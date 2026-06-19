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

test("dx-style v4.3 typography/effects slice tracks font-size modifiers and theme companion tokens", () => {
  const utility = read("related-crates/style/src/core/engine/utility/mod.rs");
  const engine = read("related-crates/style/src/core/engine/mod.rs");
  const rustTest = read("related-crates/style/tests/tailwind_v43_typography_effects_css.rs");
  const ledger = read("related-crates/style/src/core/engine/utility_ledger.rs");
  const compatibility = read("related-crates/style/TAILWIND_COMPATIBILITY.md");
  const matrix = readJson("related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json");

  assert.match(
    utility,
    /fn text_size_line_height_modifier_value/,
    "utility compiler should parse Tailwind text-<size>/<line-height> modifiers",
  );
  assert.match(
    utility,
    /fn text_shadow_alpha_value/,
    "utility compiler should parse Tailwind text-shadow size and arbitrary-value opacity modifiers",
  );
  assert.match(
    utility,
    /fn font_size_value/,
    "utility compiler should support named, arbitrary, and typed custom-property font sizes",
  );
  assert.match(
    engine,
    /fn font_family_theme_css/,
    "theme token application should merge --font-* companion feature and variation settings",
  );
  assert.match(
    engine,
    /fn text_size_theme_css/,
    "theme token application should merge --text-* line-height, tracking, and weight companions",
  );
  assert.match(
    rustTest,
    /typography_font_size_modifiers_and_theme_companion_tokens_generate_css/,
    "focused Rust regression should cover the typography/effects slice",
  );
  assert.match(
    rustTest,
    /text_shadow_value_opacity_modifiers_and_typed_arbitrary_values_generate_css/,
    "focused Rust regression should cover text-shadow opacity and typed arbitrary shadow values",
  );
  assert.match(
    ledger,
    /text-shadow-lg\/20/,
    "utility ledger should list text-shadow value opacity as supported representative evidence",
  );
  assert.match(
    compatibility,
    /text-shadow-\[shadow:var\(--dx-text-shadow\)\]/,
    "compatibility receipt should mention typed arbitrary text-shadow values",
  );

  const classNames = new Set((matrix.classes ?? []).map((entry) => entry.className));
  for (const className of [
    "text-sm/6",
    "text-lg/[1.7]",
    "text-(length:--dx-text-size)/(--dx-leading)",
    "text-shadow-lg/20",
    "text-shadow-[0_35px_35px_rgb(0_0_0_/_0.25)]/50",
    "text-shadow-[10px_10px]/25",
    "text-shadow-[shadow:var(--dx-text-shadow)]",
  ]) {
    assert.ok(
      classNames.has(className),
      `official fixture matrix should track typography canary ${className}`,
    );
  }

  const textModifier = (matrix.classes ?? []).find((entry) => entry.className === "text-sm/6");
  assert.equal(textModifier?.comparisonMode, "exact-fragment-match");
  assert.deepEqual(textModifier?.dxStyleRequiredFragments, [
    "font-size: var(--text-sm)",
    "line-height: calc(var(--spacing) * 6)",
  ]);

  const textShadowAlpha = (matrix.classes ?? []).find((entry) => entry.className === "text-shadow-lg/20");
  assert.equal(textShadowAlpha?.fullTailwindParity, false);
  assert.equal(textShadowAlpha?.comparisonMode, "exact-fragment-match");
  assert.deepEqual(textShadowAlpha?.dxStyleRequiredFragments, [
    "--tw-text-shadow-alpha: 20%",
    "text-shadow: 0px 1px 2px var(--tw-text-shadow-color, oklab(from rgb(0 0 0 / 0.1) l a b / 20%))",
  ]);
  assert.equal(
    textShadowAlpha?.differenceNote,
    undefined,
    "text-shadow-lg/20 should not keep a known-different note after exact v4.3 fragment parity",
  );
});
