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

test("dx-style owns Tailwind v4.3 interaction, scrollbar, zoom, and 3D transform utilities", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const rustTest = readRequiredFile(
    "related-crates/style/tests/tailwind_v43_interaction_transform_css.rs",
  );

  for (const marker of [
    '"scrollbar-thin" => Some("scrollbar-width: thin"',
    '"scrollbar-gutter-both" => Some("scrollbar-gutter: stable both-edges"',
    'scrollbar_color_declaration("--tw-scrollbar-thumb", raw_value)',
    'scrollbar_color_declaration("--tw-scrollbar-track", raw_value)',
    'scrollbar-color: var(--tw-scrollbar-thumb) var(--tw-scrollbar-track)',
    'let raw_value = class_name.strip_prefix("zoom-")?',
    'class_name == "transform" || class_name == "transform-cpu"',
    'class_name == "transform-gpu"',
    '"transform-3d" => Some("transform-style: preserve-3d")',
    '"backface-hidden"',
    '"dramatic" => Some("--perspective-dramatic")',
    '"translate-z-"',
    '"scale-z-"',
    '"rotate-x-"',
    '"rotate-y-"',
  ]) {
    assert.match(utility, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const className of [
    "scrollbar-thumb-mauve-500",
    "scrollbar-thumb-mist-500/40",
    "scrollbar-track-taupe-100",
    "scrollbar-track-current",
    "scrollbar-thumb-transparent",
    "snap-x",
    "snap-mandatory",
    "zoom-[1.1]",
    "transform-3d",
    "perspective-dramatic",
    "rotate-x-45",
    "-rotate-y-12",
    "translate-z-4",
    "scale-z-125",
  ]) {
    assert.match(rustTest, new RegExp(className.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.doesNotMatch(
    utility,
    /scrollbar-color: var\(--tw-scrollbar-thumb,\s*currentColor\) var\(--tw-scrollbar-track,\s*transparent\)/,
    "Tailwind v4.3 scrollbar-color output should not use dx fallback vars",
  );
});

test("official fixture matrix promotes interaction and transform utilities", () => {
  const matrix = JSON.parse(
    readRequiredFile("related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json"),
  );
  const byClass = new Map(matrix.classes.map((entry) => [entry.className, entry]));

  for (const [className, comparisonMode, fragments] of [
    [
      "scrollbar-thumb-mauve-500",
      "exact-fragment-match",
      [
        "--tw-scrollbar-thumb: var(--color-mauve-500)",
        "scrollbar-color: var(--tw-scrollbar-thumb) var(--tw-scrollbar-track)",
      ],
    ],
    [
      "scrollbar-track-taupe-100",
      "exact-fragment-match",
      [
        "--tw-scrollbar-track: var(--color-taupe-100)",
        "scrollbar-color: var(--tw-scrollbar-thumb) var(--tw-scrollbar-track)",
      ],
    ],
    ["zoom-[1.1]", "exact-fragment-match", ["zoom: 1.1"]],
    ["transform-3d", "exact-fragment-match", ["transform-style: preserve-3d"]],
    [
      "perspective-dramatic",
      "exact-fragment-match",
      ["perspective: var(--perspective-dramatic)"],
    ],
    [
      "rotate-x-45",
      "exact-fragment-match",
      [
        "--tw-rotate-x: rotateX(45deg)",
        "transform: var(--tw-rotate-x,) var(--tw-rotate-y,) var(--tw-rotate-z,) var(--tw-skew-x,) var(--tw-skew-y,)",
      ],
    ],
    [
      "translate-z-4",
      "exact-fragment-match",
      [
        "--tw-translate-z: calc(var(--spacing) * 4)",
        "translate: var(--tw-translate-x) var(--tw-translate-y) var(--tw-translate-z)",
      ],
    ],
    [
      "scale-z-125",
      "exact-fragment-match",
      ["--tw-scale-z: 125%", "scale: var(--tw-scale-x) var(--tw-scale-y) var(--tw-scale-z)"],
    ],
    ["appearance-none", "exact-fragment-match", ["appearance: none"]],
    ["select-none", "exact-fragment-match", ["user-select: none"]],
    ["backface-hidden", "exact-fragment-match", ["backface-visibility: hidden"]],
    ["break-inside-avoid", "exact-fragment-match", ["break-inside: avoid"]],
    [
      "backdrop-blur-md",
      "exact-fragment-match",
      [
        "--tw-backdrop-blur: blur(var(--blur-md))",
        "backdrop-filter: var(--tw-backdrop-blur,)",
      ],
    ],
    ["hyphens-auto", "exact-fragment-match", ["hyphens: auto"]],
    ["file:p-4", "exact-fragment-match", ["::file-selector-button", "padding: calc(var(--spacing) * 4)"]],
  ]) {
    const entry = byClass.get(className);
    assert.ok(entry, `expected matrix entry for ${className}`);
    assert.equal(entry.comparisonMode, comparisonMode);
    for (const fragment of fragments) {
      assert.ok(
        entry.dxStyleRequiredFragments?.some((candidate) => candidate.includes(fragment)),
        `${className} should require dx-style fragment ${fragment}`,
      );
    }
  }

  assert.equal(matrix.fullTailwindParity, false);
});
