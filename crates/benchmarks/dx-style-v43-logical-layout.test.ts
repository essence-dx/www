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

test("dx-style owns Tailwind v4.3 logical layout block-axis utilities", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const apply = readRequiredFile("related-crates/style/src/core/engine/apply.rs");
  const rustTest = readRequiredFile(
    "related-crates/style/tests/tailwind_v43_logical_layout_css.rs",
  );

  for (const marker of [
    '("px-", &["padding-inline"], false)',
    '("py-", &["padding-block"], false)',
    '("pbs-", &["padding-block-start"], false)',
    '("pbe-", &["padding-block-end"], false)',
    '("ps-", &["padding-inline-start"], false)',
    '("pe-", &["padding-inline-end"], false)',
    '("mx-", &["margin-inline"], true)',
    '("my-", &["margin-block"], true)',
    '("mbs-", &["margin-block-start"], true)',
    '("mbe-", &["margin-block-end"], true)',
    '("ms-", &["margin-inline-start"], true)',
    '("me-", &["margin-inline-end"], true)',
    '"space-x-reverse"',
    "--tw-space-x-reverse: 1",
    "margin-inline-start",
    "margin-inline-end",
    '"space-y-reverse"',
    "--tw-space-y-reverse: 1",
    "margin-block-start",
    "margin-block-end",
    'format!("calc(var(--spacing) * {multiplier})")',
    "spacing_calc_multiplier(&value)",
    '("inset-s-", &["inset-inline-start"], true)',
    '("inset-e-", &["inset-inline-end"], true)',
    '("inset-bs-", &["inset-block-start"], true)',
    '("inset-be-", &["inset-block-end"], true)',
    '"inline-", "inline-size", SizeAxis::Width, true',
    '"block-", "block-size", SizeAxis::Height, false',
    'container_scale_size_value(raw_value)',
    '("scroll-mbs-", &["scroll-margin-block-start"], true)',
    '("scroll-ms-", &["scroll-margin-inline-start"], true)',
    '("scroll-me-", &["scroll-margin-inline-end"], true)',
    '("scroll-pbs-", &["scroll-padding-block-start"], false)',
    '("scroll-ps-", &["scroll-padding-inline-start"], false)',
    '("scroll-pe-", &["scroll-padding-inline-end"], false)',
    '"border-x-"',
    '"border-inline-style"',
    '"border-inline-width"',
    '"border-y-"',
    '"border-block-style"',
    '"border-block-width"',
    '"border-bs-"',
    '"border-block-start-style"',
    '"border-block-start-width"',
    '"border-be-"',
    '"border-block-end-style"',
    '"border-block-end-width"',
    '("border-bs-", &["border-block-start-color"])',
    '("border-be-", &["border-block-end-color"])',
    '"rounded-s-"',
    '"border-start-start-radius"',
    '"border-end-start-radius"',
    '("rounded-ee-", &["border-end-end-radius"])',
    '"var(--radius-lg)"',
    '"calc(infinity * 1px)"',
  ]) {
    assert.match(
      `${utility}\n${engine}\n${apply}`,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `${marker} should be source-owned`,
    );
  }

  for (const className of [
    "px-4",
    "py-2",
    "pbs-4",
    "ps-4",
    "pe-[3rem]",
    "pbe-(--panel-block-padding)",
    "mx-auto",
    "ms-auto",
    "-my-2",
    "-mbs-2",
    "-me-2",
    "space-x-4",
    "-space-x-2",
    "space-y-3",
    "space-x-reverse",
    "mbe-auto",
    "inset-s-4",
    "inset-s-1/2",
    "-inset-e-1/2",
    "-inset-e-full",
    "inset-bs-full",
    "scroll-mbs-6",
    "scroll-ms-6",
    "-scroll-me-2",
    "scroll-ps-6",
    "scroll-pe-(--snap-inline-end)",
    "scroll-pbe-2",
    "-scroll-ps-2",
    "-scroll-pbe-2",
    "pis-4",
    "border-x",
    "border-y-4",
    "border-s-red-500",
    "border-bs-4",
    "border-bs-emerald-500",
    "border-be-(length:--dx-border-block)",
    "rounded",
    "rounded-xs",
    "rounded-lg",
    "rounded-4xl",
    "rounded-t-lg",
    "rounded-s-lg",
    "rounded-e-none",
    "rounded-ss-full",
    "rounded-se-(--dx-radius)",
    "rounded-ee-xl",
    "rounded-es-[2rem]",
    "inline-3xs",
    "min-inline-xl",
    "size-3xs",
  ]) {
    assert.match(rustTest, new RegExp(className.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
});

test("official fixture matrix includes governed logical layout coverage", () => {
  const matrix = JSON.parse(
    readRequiredFile("related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json"),
  );
  const entries = new Map(matrix.classes.map((entry) => [entry.className, entry]));

  for (const [className, area, dxFragment] of [
    ["px-4", "logical-spacing", "padding-inline: calc(var(--spacing) * 4)"],
    ["py-2", "logical-spacing", "padding-block: calc(var(--spacing) * 2)"],
    ["pbs-4", "logical-spacing", "padding-block-start: calc(var(--spacing) * 4)"],
    ["ps-4", "logical-spacing", "padding-inline-start: calc(var(--spacing) * 4)"],
    ["pe-[3rem]", "logical-spacing", "padding-inline-end: 3rem"],
    ["mx-auto", "logical-spacing", "margin-inline: auto"],
    ["ms-auto", "logical-spacing", "margin-inline-start: auto"],
    ["-mbs-2", "logical-spacing", "margin-block-start: calc(var(--spacing) * -2)"],
    ["-me-2", "logical-spacing", "margin-inline-end: calc(var(--spacing) * -2)"],
    ["space-x-4", "logical-spacing", "margin-inline-end: calc(calc(var(--spacing) * 4) * calc(1 - var(--tw-space-x-reverse)))"],
    ["space-x-reverse", "logical-spacing", "--tw-space-x-reverse: 1"],
    ["inset-s-4", "logical-inset", "inset-inline-start: calc(var(--spacing) * 4)"],
    ["inset-s-1/2", "logical-inset", "inset-inline-start: calc(1 / 2 * 100%)"],
    ["-inset-e-full", "logical-inset", "inset-inline-end: -100%"],
    ["inset-bs-full", "logical-inset", "inset-block-start: 100%"],
    ["inline-3xs", "logical-sizing", "inline-size: var(--container-3xs)"],
    ["w-1/2", "sizing", "width: calc(1 / 2 * 100%)"],
    ["scroll-mbs-6", "logical-scroll", "scroll-margin-block-start: calc(var(--spacing) * 6)"],
    ["scroll-ms-6", "logical-scroll", "scroll-margin-inline-start: calc(var(--spacing) * 6)"],
    ["-scroll-me-2", "logical-scroll", "scroll-margin-inline-end: calc(var(--spacing) * -2)"],
    ["scroll-ps-6", "logical-scroll", "scroll-padding-inline-start: calc(var(--spacing) * 6)"],
    ["scroll-pe-(--snap-inline-end)", "logical-scroll", "scroll-padding-inline-end: var(--snap-inline-end)"],
    ["scroll-pbe-2", "logical-scroll", "scroll-padding-block-end: calc(var(--spacing) * 2)"],
    ["border-x", "logical-borders", "border-inline-width: 1px"],
    ["border-y-4", "logical-borders", "border-block-width: 4px"],
    ["border-bs-4", "logical-borders", "border-block-start-width: 4px"],
    ["border-s-red-500", "logical-borders", "border-inline-start-color: var(--color-red-500)"],
    ["border-bs-emerald-500", "logical-borders", "border-block-start-color: var(--color-emerald-500)"],
    ["border-be-(length:--dx-border-block)", "logical-borders", "border-block-end-width: var(--dx-border-block)"],
    ["rounded", "logical-radius", "border-radius: 0.25rem"],
    ["rounded-xs", "logical-radius", "border-radius: var(--radius-xs)"],
    ["rounded-lg", "logical-radius", "border-radius: var(--radius-lg)"],
    ["rounded-4xl", "logical-radius", "border-radius: var(--radius-4xl)"],
    ["rounded-t-lg", "logical-radius", "border-top-left-radius: var(--radius-lg)"],
    ["rounded-s-lg", "logical-radius", "border-start-start-radius: var(--radius-lg)"],
    ["rounded-e-none", "logical-radius", "border-start-end-radius: 0"],
    ["rounded-ss-full", "logical-radius", "border-start-start-radius: calc(infinity * 1px)"],
    ["rounded-se-(--dx-radius)", "logical-radius", "border-start-end-radius: var(--dx-radius)"],
    ["rounded-ee-xl", "logical-radius", "border-end-end-radius: var(--radius-xl)"],
    ["rounded-es-[2rem]", "logical-radius", "border-end-start-radius: 2rem"],
  ]) {
    const entry = entries.get(className);
    assert.ok(entry, `fixture matrix should include ${className}`);
    assert.equal(entry.area, area);
    assert.notEqual(entry.comparisonMode, "tailwind-only-gap");
    assert.ok(
      entry.dxStyleRequiredFragments?.includes(dxFragment),
      `${className} should require dx-style fragment ${dxFragment}`,
    );
  }

  assert.equal(matrix.fullTailwindParity, false);
  assert.equal(matrix.officialFixtureTruth?.fullTailwindParity, false);
});
