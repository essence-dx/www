import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

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

test("dx-style owns Tailwind v4.3 custom-variant block directive canaries", () => {
  const cssFirstTest = readRequiredFile("related-crates/style/tests/tailwind_v4_css_first.rs");
  const themeCss = readRequiredFile("related-crates/style/src/core/engine/theme_css.rs");
  const states = readRequiredFile("related-crates/style/src/core/engine/states/mod.rs");
  const directiveLedger = readRequiredFile(
    "related-crates/style/src/core/engine/directive_ledger.rs",
  );

  for (const marker of [
    "css_first_custom_variant_block_directive_supports_slot_media_and_selector_forms",
    "@custom-variant theme-midnight",
    "@custom-variant any-hover",
    "@slot",
    "any-hover:bg-brand",
    "theme-midnight:bg-brand",
  ]) {
    assert.match(cssFirstTest, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "parse_custom_variant_directives",
    "parse_custom_variant_block",
    "CssCustomVariant",
    "media_queries",
  ]) {
    assert.match(themeCss, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(states, /custom_variant\.media_queries/);
  assert.match(states, /custom_variant\.selector/);

  for (const marker of [
    "@custom-variant theme-midnight { &:where([data-theme=\"midnight\"] *) { @slot; } }",
    "@custom-variant any-hover { @media (any-hover: hover) { &:hover { @slot; } } }",
  ]) {
    assert.match(directiveLedger, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
});

test("official fixture matrix classifies custom-variant block directive coverage", () => {
  const matrix = JSON.parse(
    readRequiredFile("related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json"),
  );
  const customVariant = (matrix.cssDirectiveCanaries ?? []).find(
    (entry) => entry.directive === "@custom-variant",
  );

  assert.ok(customVariant, "missing @custom-variant directive canary row");
  assert.equal(customVariant.dxStyleStatus, "partial");
  assert.equal(customVariant.fullTailwindParity, false);

  for (const marker of [
    "@custom-variant theme-midnight { &:where([data-theme=\"midnight\"] *) { @slot; } }",
    "@custom-variant any-hover { @media (any-hover: hover) { &:hover { @slot; } } }",
  ]) {
    assert.ok(
      customVariant.canaries?.includes(marker),
      `@custom-variant canaries should include ${marker}`,
    );
  }

  assert.ok(
    customVariant.missingOrUnproven?.includes("multiple @slot selector-list expansion"),
    "@custom-variant should keep selector-list expansion as an explicit remaining gap",
  );
});

test("dx-style expands CSS @variant inside safe layered authored CSS", () => {
  const authoredCss = readRequiredFile("related-crates/style/src/core/engine/authored_css.rs");
  const cssFirstTest = readRequiredFile("related-crates/style/tests/tailwind_v4_css_first.rs");
  const directiveLedger = readRequiredFile(
    "related-crates/style/src/core/engine/directive_ledger.rs",
  );
  const matrix = JSON.parse(
    readRequiredFile("related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json"),
  );

  for (const marker of [
    "css_first_variant_directive_expands_safe_layered_arbitrary_authored_css",
    "@variant hover, focus",
    "@variant md:[&>.icon]",
    "@layer components",
  ]) {
    assert.match(cssFirstTest, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "resolve_layered_variant_rules",
    "wrap_variant_layer_rules",
    "is_safe_variant_layer_selector",
  ]) {
    assert.match(authoredCss, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(directiveLedger, /@variant inside safe @layer authored CSS/);

  const cssVariant = (matrix.cssDirectiveCanaries ?? []).find(
    (entry) => entry.directive === "CSS @variant",
  );
  assert.ok(cssVariant, "missing CSS @variant directive canary row");
  assert.ok(
    cssVariant.canaries?.includes("@variant safe arbitrary selector"),
    "CSS @variant row should track safe arbitrary selector support",
  );
  assert.ok(
    cssVariant.canaries?.includes("@variant inside safe @layer authored CSS"),
    "CSS @variant row should track safe layered authored CSS support",
  );
  assert.ok(
    !(cssVariant.missingOrUnproven ?? []).includes("@variant arbitrary selector"),
    "safe arbitrary selector @variant support should no longer be a blanket missing item",
  );
});

test("dx-style owns a variant-bearing @apply directive slice", () => {
  const applyEngine = readRequiredFile("related-crates/style/src/core/engine/apply.rs");
  const themeCss = readRequiredFile("related-crates/style/src/core/engine/theme_css.rs");
  const cssFirstTest = readRequiredFile("related-crates/style/tests/tailwind_v4_css_first.rs");
  const directiveLedger = readRequiredFile(
    "related-crates/style/src/core/engine/directive_ledger.rs",
  );

  for (const marker of [
    "css_first_apply_expands_variant_tokens_without_tailwind_runtime",
    "@apply hover:bg-brand focus:opacity-100 md:px-4 dark:hover:bg-brand theme-midnight:opacity-50",
    "css_first_apply_keeps_unsafe_variant_tokens_diagnosed",
  ]) {
    assert.match(cssFirstTest, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "resolve_apply_variant_rule",
    "apply_variant_token_parts",
    "build_apply_variant_rule",
  ]) {
    assert.match(applyEngine, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(
    themeCss,
    /is_variant_safe_apply_token/,
    "theme CSS diagnostics should recognize safe variant-bearing @apply tokens",
  );

  for (const marker of ["@apply hover:bg-brand", "@apply md:px-4", "@apply dark:hover:bg-brand"]) {
    assert.match(directiveLedger, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
});

test("dx-style preserves safe authored CSS functions inside @apply rules", () => {
  const applyEngine = readRequiredFile("related-crates/style/src/core/engine/apply.rs");
  const cssFirstTest = readRequiredFile("related-crates/style/tests/tailwind_v4_css_first.rs");
  const directiveLedger = readRequiredFile(
    "related-crates/style/src/core/engine/directive_ledger.rs",
  );
  const matrix = JSON.parse(
    readRequiredFile("related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json"),
  );

  for (const marker of [
    "css_first_apply_preserves_safe_authored_function_declarations",
    "color: --alpha(var(--color-brand) / 35%)",
    "margin-inline: --spacing(3)",
    "width: calc(100% - --spacing(2))",
  ]) {
    assert.match(cssFirstTest, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "resolve_safe_authored_function_declaration",
    "css_functions::replace_tailwind_css_functions",
  ]) {
    assert.match(applyEngine, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(
    directiveLedger,
    /@apply with safe authored --alpha\(\) and --spacing\(\) declarations/,
  );

  const apply = (matrix.cssDirectiveCanaries ?? []).find(
    (entry) => entry.directive === "@apply",
  );
  assert.ok(apply, "missing @apply directive canary row");
  assert.ok(
    apply.canaries?.includes("@apply with safe authored --alpha() and --spacing() declarations"),
    "@apply row should track safe authored CSS function preservation",
  );
});

test("dx-style expands @apply inside safe nested authored selectors", () => {
  const applyEngine = readRequiredFile("related-crates/style/src/core/engine/apply.rs");
  const cssFirstTest = readRequiredFile("related-crates/style/tests/tailwind_v4_css_first.rs");
  const directiveLedger = readRequiredFile(
    "related-crates/style/src/core/engine/directive_ledger.rs",
  );
  const matrix = JSON.parse(
    readRequiredFile("related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json"),
  );

  for (const marker of [
    "css_first_apply_expands_safe_nested_authored_selectors",
    "&:hover",
    "& .icon",
    "color: --alpha(var(--color-brand) / 40%)",
  ]) {
    assert.match(cssFirstTest, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "resolve_nested_apply_blocks",
    "resolve_nested_apply_selector",
    "is_safe_nested_apply_selector",
  ]) {
    assert.match(applyEngine, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(directiveLedger, /@apply inside safe nested & authored selectors/);

  const apply = (matrix.cssDirectiveCanaries ?? []).find(
    (entry) => entry.directive === "@apply",
  );
  assert.ok(apply, "missing @apply directive canary row");
  assert.ok(
    apply.canaries?.includes("@apply inside safe nested & authored selectors"),
    "@apply row should track safe nested authored selector expansion",
  );
  assert.ok(
    apply.missingOrUnproven?.includes("full Tailwind @apply cascade/order/important parity"),
    "@apply row should keep full cascade/order/important parity honest as still unproven",
  );
  assert.ok(
    !(apply.missingOrUnproven ?? []).includes("@apply inside layered or nested authored CSS"),
    "nested authored selectors should no longer be lumped into the layered gap once implemented",
  );
});

test("dx-style expands @apply inside safe layered authored CSS", () => {
  const applyEngine = readRequiredFile("related-crates/style/src/core/engine/apply.rs");
  const cssFirstTest = readRequiredFile("related-crates/style/tests/tailwind_v4_css_first.rs");
  const directiveLedger = readRequiredFile(
    "related-crates/style/src/core/engine/directive_ledger.rs",
  );
  const matrix = JSON.parse(
    readRequiredFile("related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json"),
  );

  for (const marker of [
    "css_first_apply_expands_safe_layered_authored_css",
    "@layer components",
    "border-width: 1px",
    "color: --alpha(var(--color-brand) / 40%)",
  ]) {
    assert.match(cssFirstTest, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "resolve_layered_apply_rules",
    "wrap_apply_layer_rules",
    "is_safe_apply_layer_selector",
  ]) {
    assert.match(applyEngine, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(directiveLedger, /@apply inside safe @layer authored CSS/);

  const apply = (matrix.cssDirectiveCanaries ?? []).find(
    (entry) => entry.directive === "@apply",
  );
  assert.ok(apply, "missing @apply directive canary row");
  assert.ok(
    apply.canaries?.includes("@apply inside safe @layer authored CSS"),
    "@apply row should track safe layered authored CSS expansion",
  );
  assert.ok(
    !(apply.missingOrUnproven ?? []).includes("@apply inside layered authored CSS"),
    "safe layered authored CSS should be removed from the missing list once implemented",
  );
});

test("dx-style owns safe @reference directive inputs without Tailwind runtime", () => {
  const themeCss = readRequiredFile("related-crates/style/src/core/engine/theme_css.rs");
  const coreExports = readRequiredFile("related-crates/style/src/core/mod.rs");
  const cssFirstTest = readRequiredFile("related-crates/style/tests/tailwind_v4_css_first.rs");
  const directiveLedger = readRequiredFile(
    "related-crates/style/src/core/engine/directive_ledger.rs",
  );
  const matrix = JSON.parse(
    readRequiredFile("related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json"),
  );

  for (const marker of [
    "CssReferenceDirective",
    "css_reference_directives",
    "TailwindDefaultTheme",
    "is_local_css_reference_specifier",
    "package, URL, and JS/runtime reference resolution",
  ]) {
    assert.match(themeCss, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(coreExports, /css_reference_directives/);
  assert.match(
    cssFirstTest,
    /reference_directive_accepts_local_and_tailwind_default_without_runtime_dependency/,
  );

  for (const marker of [
    '@reference \\"./tokens.css\\" accepted as local reference input',
    '@reference \\"tailwindcss\\" consumed as DX-owned default-theme reference',
  ]) {
    assert.match(directiveLedger, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  const reference = (matrix.cssDirectiveCanaries ?? []).find(
    (entry) => entry.directive === "@reference",
  );
  assert.ok(reference, "missing @reference directive canary row");
  assert.equal(reference.dxStyleStatus, "partial");
  assert.equal(reference.fullTailwindParity, false);
  assert.ok(
    reference.canaries?.includes('@reference "tailwindcss"'),
    '@reference row should track Tailwind default-theme reference syntax',
  );
  assert.ok(
    reference.canaries?.includes('@reference "./tokens.css"'),
    "@reference row should track local CSS reference syntax",
  );
  assert.ok(
    reference.missingOrUnproven?.includes("@reference package/subpath import semantics"),
    "@reference should keep package/subpath imports as a remaining explicit gap",
  );
});
