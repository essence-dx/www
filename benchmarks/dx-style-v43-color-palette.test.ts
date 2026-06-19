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

test("dx-style owns Tailwind v4.3 neutral-adjacent palette values", () => {
  const utility = readRequiredFile(
    "related-crates/style/src/core/engine/utility/mod.rs",
  );
  const colorPalette = readRequiredFile(
    "related-crates/style/src/core/engine/utility/color_palette.rs",
  );
  const themeCss = readRequiredFile(
    "related-crates/style/src/core/engine/theme_css.rs",
  );
  const rustTest = readRequiredFile(
    "related-crates/style/tests/tailwind_v43_color_palette_css.rs",
  );

  for (const marker of [
    "mod color_palette",
    "tailwind_v43_neutral_palette_variable",
    "color_palette::tailwind_v43_oklch_color(name)",
    "var(--color-{name})",
    "color-mix(in oklab",
  ]) {
    assert.match(utility, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "tailwind_v43_oklch_color",
    '("mauve", "500") => Some("oklch(54.2% 0.034 322.5)")',
    '("olive", "500") => Some("oklch(58% 0.031 107.3)")',
    '("mist", "500") => Some("oklch(56% 0.021 213.5)")',
    '("taupe", "500") => Some("oklch(54.7% 0.021 43.1)")',
  ]) {
    assert.match(colorPalette, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "--color-mauve-500: oklch(54.2% 0.034 322.5);",
    "--color-olive-500: oklch(58% 0.031 107.3);",
    "--color-mist-500: oklch(56% 0.021 213.5);",
    "--color-taupe-500: oklch(54.7% 0.021 43.1);",
  ]) {
    assert.match(themeCss, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const className of [
    "bg-mauve-500",
    "text-olive-500",
    "border-mist-300",
    "ring-taupe-700",
    "outline-mauve-600",
    "decoration-olive-400",
    "placeholder-mauve-500",
    "placeholder-olive-500/50",
    "placeholder-[#243c5a]",
    "placeholder-(color:--dx-placeholder)/(--dx-alpha)",
    "divide-mauve-500",
    "divide-olive-500/50",
    "divide-[#243c5a]",
    "divide-(color:--dx-divider)/(--dx-alpha)",
    "shadow-mauve-500",
    "drop-shadow-mauve-500/50",
    "inset-shadow-olive-500",
    "inset-ring-mist-500/50",
    "ring-offset-taupe-500/40",
    "text-shadow-mist-500/40",
    "bg-mauve-500/50",
    "text-taupe-500/25",
    "bg-mist-500/[71.37%]",
    "border-taupe-500/(--dx-alpha)",
    "from-mauve-500/40",
    "via-taupe-500/40",
    "to-olive-950/80",
    "to-taupe-950",
    "scrollbar-thumb-mauve-500/60",
    "scrollbar-track-taupe-100/10",
    "border-x-mauve-500",
    "border-y-olive-500/50",
    "border-t-mist-300",
    "border-r-taupe-700",
    "border-b-mauve-950",
    "border-l-olive-600/[71.37%]",
    "border-t-[#243c5a]",
    "border-r-[#243c5a]/50",
    "border-x-(color:--dx-border)",
    "border-y-(color:--dx-border-alpha)/(--dx-alpha)",
  ]) {
    assert.match(rustTest, new RegExp(className.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "directional_border_color_utility",
    "fn divide_utility",
    '"divide-"',
    'child_color_property_css("* + *", "border-color", raw_value)',
    '"border-x-"',
    '"border-y-"',
    '"border-inline-color"',
    '"border-block-color"',
    '"placeholder-"',
    'nested_color_property_css("::placeholder", "color", raw_value)',
    '"border-top-color"',
    '"border-right-color"',
    '"border-bottom-color"',
    '"border-left-color"',
    "tailwind_v43_palette_opacity_property_css",
    'typed_custom_property_var_value(name, "color")',
  ]) {
    assert.match(utility, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
});

test("official Tailwind v4.3 fixture matrix no longer classifies neutral-adjacent palettes as Tailwind-only gaps", () => {
  const matrix = JSON.parse(
    readRequiredFile(
      "related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json",
    ),
  );
  const byClass = new Map(matrix.classes.map((entry) => [entry.className, entry]));

  for (const [className, expectedFragment] of [
    ["bg-mauve-500", "background-color: var(--color-mauve-500)"],
    ["bg-olive-500", "background-color: var(--color-olive-500)"],
    ["bg-mist-500", "background-color: var(--color-mist-500)"],
    ["bg-taupe-500", "background-color: var(--color-taupe-500)"],
    ["text-olive-600/75", "color: color-mix(in oklab, var(--color-olive-600) 75%, transparent)"],
    ["border-mist-300", "border-color: var(--color-mist-300)"],
    ["ring-taupe-400/50", "--tw-ring-color: color-mix(in oklab, var(--color-taupe-400) 50%, transparent)"],
    ["decoration-olive-500", "text-decoration-color: var(--color-olive-500)"],
    ["from-mist-500", "--tw-gradient-from: var(--color-mist-500)"],
    ["via-taupe-500/40", "--tw-gradient-via: color-mix(in oklab, var(--color-taupe-500) 40%, transparent)"],
    ["to-mauve-950", "--tw-gradient-to: var(--color-mauve-950)"],
    ["placeholder-mauve-500", "color: var(--color-mauve-500)"],
    ["placeholder-olive-500/50", "color: color-mix(in oklab, var(--color-olive-500) 50%, transparent)"],
    ["placeholder-[#243c5a]", "color: #243c5a"],
    ["placeholder-(color:--dx-placeholder)/(--dx-alpha)", "color: color-mix(in oklab, var(--dx-placeholder) var(--dx-alpha), transparent)"],
    ["divide-mauve-500", "border-color: var(--color-mauve-500)"],
    ["divide-olive-500/50", "border-color: color-mix(in oklab, var(--color-olive-500) 50%, transparent)"],
    ["divide-[#243c5a]", "border-color: #243c5a"],
    ["divide-(color:--dx-divider)/(--dx-alpha)", "border-color: color-mix(in oklab, var(--dx-divider) var(--dx-alpha), transparent)"],
    ["border-x-mauve-500", "border-inline-color: var(--color-mauve-500)"],
    ["border-y-olive-500/50", "border-block-color: color-mix(in oklab, var(--color-olive-500) 50%, transparent)"],
    ["border-t-mist-300", "border-top-color: var(--color-mist-300)"],
    ["border-r-taupe-700", "border-right-color: var(--color-taupe-700)"],
    ["border-b-mauve-950", "border-bottom-color: var(--color-mauve-950)"],
    ["border-l-olive-600/[71.37%]", "border-left-color: color-mix(in oklab, var(--color-olive-600) 71.37%, transparent)"],
    ["border-t-[#243c5a]", "border-top-color: #243c5a"],
    ["border-r-[#243c5a]/50", "border-right-color: color-mix(in oklab, #243c5a 50%, transparent)"],
    ["border-x-(color:--dx-border)", "border-inline-color: var(--dx-border)"],
    ["border-y-(color:--dx-border-alpha)/(--dx-alpha)", "border-block-color: color-mix(in oklab, var(--dx-border-alpha) var(--dx-alpha), transparent)"],
  ]) {
    const entry = byClass.get(className);
    assert.ok(entry, `expected matrix entry for ${className}`);
    assert.notEqual(entry.comparisonMode, "tailwind-only-gap");
    assert.ok(
      ["exact-fragment-match", "known-different"].includes(entry.comparisonMode),
      `${className} should be an explained comparison entry`,
    );
    assert.ok(
      entry.dxStyleRequiredFragments?.includes(expectedFragment),
      `${className} should require dx-style fragment ${expectedFragment}`,
    );
  }
});
