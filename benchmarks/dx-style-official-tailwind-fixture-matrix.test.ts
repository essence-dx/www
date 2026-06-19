import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const matrixPath = path.join(
  root,
  "related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json",
);
const inventoryPath = path.join(
  root,
  "related-crates/style/fixtures/tailwind-v43-official-candidate-inventory.json",
);

function readJson(filePath) {
  assert.ok(fs.existsSync(filePath), `expected ${path.relative(root, filePath)} to exist`);
  return JSON.parse(fs.readFileSync(filePath, "utf8"));
}

test("dx-style ingests an official Tailwind v4.3 fixture matrix with provenance", () => {
  const matrix = readJson(matrixPath);

  assert.equal(matrix.schema, "dx.style.tailwindOfficialFixtureMatrix");
  assert.equal(matrix.schemaVersion, 1);
  assert.equal(matrix.tailwindPackage.name, "tailwindcss");
  assert.equal(matrix.tailwindPackage.version, "4.3.0");
  assert.equal(matrix.tailwindCliPackage.name, "@tailwindcss/cli");
  assert.equal(matrix.tailwindCliPackage.version, "4.3.0");
  assert.equal(matrix.tailwindRuntimeDependency, false);
  assert.equal(matrix.fullTailwindParity, false);
  assert.equal(matrix.liveTailwindExecutionRequired, true);
  assert.equal(matrix.officialSource.repository, "tailwindlabs/tailwindcss");
  assert.equal(matrix.officialSource.tag, "v4.3.0");
  assert.equal(matrix.officialSource.commit, "588bd7371f4cae96426e1387819b7fd1d99765f9");
  assert.equal(
    matrix.officialSource.candidateInventory,
    "related-crates/style/fixtures/tailwind-v43-official-candidate-inventory.json",
  );
  assert.equal(
    matrix.officialSource.ingestionTool,
    "tools/dx-style/ingest-tailwind-v43-fixtures.cjs",
  );
  assert.ok(
    fs.existsSync(path.join(root, matrix.officialSource.ingestionTool)),
    "official fixture ingestion tool should exist",
  );

  assert.ok(
    matrix.sources.some((source) => source.url === "https://tailwindcss.com/blog/tailwindcss-v4-3"),
    "matrix should reference the official Tailwind v4.3 release notes",
  );
  assert.ok(
    matrix.sources.some((source) => source.url.includes("tailwindcss.com/docs")),
    "matrix should reference official Tailwind docs",
  );

  assert.ok(matrix.classes.length >= 20, "matrix should cover more than tiny smoke fixtures");

  const inventory = readJson(inventoryPath);
  assert.equal(inventory.schema, "dx.style.tailwindOfficialCandidateInventory");
  assert.equal(inventory.schemaVersion, 1);
  assert.equal(inventory.tailwindPackage.version, "4.3.0");
  assert.equal(inventory.officialSource.tag, "v4.3.0");
  assert.equal(inventory.officialSource.commit, "588bd7371f4cae96426e1387819b7fd1d99765f9");
  assert.equal(inventory.generatedBy, "tools/dx-style/ingest-tailwind-v43-fixtures.cjs");
  assert.ok(inventory.sourceFileCount >= 70, "inventory should scan Tailwind source tests");
  assert.ok(inventory.candidateCount >= 8000, "inventory should ingest Tailwind source-test candidates");
  assert.equal(inventory.candidates.length, inventory.candidateCount);
  assert.equal(
    matrix.officialFixtureTruth?.inventory,
    "related-crates/style/fixtures/tailwind-v43-official-candidate-inventory.json",
  );
  assert.equal(matrix.officialFixtureTruth?.sourceFileCount, inventory.sourceFileCount);
  assert.equal(matrix.officialFixtureTruth?.candidateCount, inventory.candidateCount);
  assert.equal(
    matrix.officialFixtureTruth?.fixtureCount,
    inventory.officialFixtureMatrix.fixtureCount,
  );
  assert.equal(
    matrix.officialFixtureTruth?.fixtureSourceFileCount,
    inventory.officialFixtureMatrix.fixtureSourceFileCount,
  );
  assert.equal(matrix.officialFixtureTruth?.snapshotOutputPolicy, "fingerprinted-not-vendored");
  assert.equal(matrix.officialFixtureTruth?.fullTailwindParity, false);
  assert.equal(
    inventory.officialFixtureMatrix?.schema,
    "dx.style.tailwindOfficialFixtureMatrix",
  );
  assert.equal(inventory.officialFixtureMatrix?.schemaVersion, 1);
  assert.ok(
    inventory.officialFixtureMatrix.fixtureCount >= 1000,
    "inventory should index official Tailwind inline-snapshot fixtures",
  );
  assert.ok(
    inventory.officialFixtureMatrix.fixtureSourceFileCount >= 40,
    "official fixture matrix should span Tailwind source test files",
  );

  const officialFixtureKinds = new Set(
    inventory.officialFixtureMatrix.entries.map((entry) => entry.kind),
  );
  for (const expectedKind of ["candidate-parser", "compiler-output", "css-directive"]) {
    assert.ok(
      officialFixtureKinds.has(expectedKind),
      `official fixture matrix should include ${expectedKind} fixtures`,
    );
  }

  const inventoryCandidates = new Set(inventory.candidates);
  for (const required of [
    "sr-only",
    "pointer-events-none",
    "inset-4",
    "bg-red-500",
    "@container-size",
    "pointer-fine:flex",
  ]) {
    assert.ok(inventoryCandidates.has(required), `inventory should include official candidate ${required}`);
  }

  const classNames = new Set(matrix.classes.map((entry) => entry.className));
  for (const required of [
    "@container-normal",
    "@container-normal/sidebar",
    "@container-size",
    "@container-size/main",
    "@min-[123px]:flex",
    "@max-[123px]:hidden",
    "@min-[456px]/name:grid",
    "@max-[456px]/name:block",
    "@max-[960px]/name:@min-[475px]/name:flex",
    "@[475px]:flex",
    "@[475px]/card:grid",
    "@[475px]:@max-[960px]:block",
    "@[475px]/card:@max-[960px]/card:hidden",
    "@min-[40rem]:@max-[70rem]:flex",
    "@min-[40rem]/main:@max-[70rem]/main:grid",
    "zoom-125",
    "tab-4",
    "bg-mauve-500",
    "bg-olive-500",
    "bg-mist-500",
    "bg-taupe-500",
    "text-olive-600/75",
    "border-mist-300",
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
    "ring-taupe-400/50",
    "outline-mauve-700",
    "decoration-olive-500",
    "from-mist-500",
    "via-taupe-500/40",
    "to-mauve-950",
    "placeholder-mauve-500",
    "placeholder-olive-500/50",
    "placeholder-[#243c5a]",
    "placeholder-(color:--dx-placeholder)/(--dx-alpha)",
    "divide-mauve-500",
    "divide-olive-500/50",
    "divide-[#243c5a]",
    "divide-(color:--dx-divider)/(--dx-alpha)",
    "pointer-fine:opacity-100",
    "motion-safe:opacity-100",
    "motion-reduce:opacity-100",
    "contrast-more:opacity-100",
    "contrast-less:opacity-100",
    "forced-colors:opacity-100",
    "inverted-colors:opacity-100",
    "pointer-none:opacity-100",
    "pointer-coarse:opacity-100",
    "any-pointer-none:opacity-100",
    "any-pointer-coarse:opacity-100",
    "any-pointer-fine:opacity-100",
    "noscript:opacity-100",
    "portrait:opacity-100",
    "landscape:opacity-100",
    "print:opacity-100",
    "not-motion-safe:opacity-100",
    "not-motion-reduce:opacity-100",
    "not-pointer-fine:opacity-100",
    "not-forced-colors:opacity-100",
    "not-portrait:opacity-100",
    "not-landscape:opacity-100",
    "not-noscript:opacity-100",
    "not-print:opacity-100",
    "rtl:ps-4",
    "ltr:pe-4",
    "inert:opacity-50",
    "open:bg-blue-500",
    "starting:open:opacity-0",
    "user-valid:border-green-500",
    "user-invalid:border-red-500",
    "details-content:bg-slate-100",
    "group-odd:bg-mauve-500",
    "group-disabled:opacity-100",
    "group-focus-visible/card:opacity-100",
    "peer-invalid:visible",
    "peer-required/email:block",
    "peer-disabled:opacity-100",
    "has-checked:opacity-100",
    "has-disabled:opacity-100",
    "not-checked:opacity-100",
    "in-checked:opacity-100",
    "in-hover:opacity-100",
    "not-hover:opacity-100",
    "hover:not-focus:opacity-100",
    "not-focus:hover:opacity-100",
    "group-hover:not-focus:opacity-100",
    "in-focus:hover:opacity-100",
    "not-pointer-fine:hover:opacity-100",
    "hover:not-pointer-fine:opacity-100",
    "group-has-checked:opacity-100",
    "group-has-disabled/card:opacity-100",
    "peer-has-checked:opacity-100",
    "peer-has-disabled/card:opacity-100",
    "group-not-disabled/card:opacity-100",
    "peer-not-checked:opacity-100",
    "peer-not-disabled/card:opacity-100",
    "[@unknown_rule]:p-4",
    "not-[@container_card_(width>=32rem)]:flex",
  ]) {
    assert.ok(classNames.has(required), `matrix should include ${required}`);
  }

  const matrixEntries = new Map(matrix.classes.map((entry) => [entry.className, entry]));
  for (const required of [
    "bg-mauve-500",
    "bg-olive-500",
    "bg-mist-500",
    "bg-taupe-500",
    "text-olive-600/75",
    "border-mist-300",
    "border-x-mauve-500",
    "border-y-olive-500/50",
    "border-t-mist-300",
    "border-r-taupe-700",
    "border-b-mauve-950",
    "border-l-olive-600/[71.37%]",
    "ring-taupe-400/50",
    "outline-mauve-700",
    "decoration-olive-500",
    "from-mist-500",
    "via-taupe-500/40",
    "to-mauve-950",
    "placeholder-mauve-500",
    "placeholder-olive-500/50",
    "divide-mauve-500",
    "divide-olive-500/50",
  ]) {
    const entry = matrixEntries.get(required);
    assert.equal(
      entry.comparisonMode,
      "exact-fragment-match",
      `${required} should no longer be tracked as a Tailwind-only palette gap`,
    );
    assert.ok(
      entry.dxStyleRequiredFragments?.some((fragment) => fragment.includes("--color-")),
      `${required} should prove dx-style emits token-backed color CSS`,
    );
  }

  const pointerFine = matrixEntries.get("pointer-fine:opacity-100");
  assert.equal(
    pointerFine?.comparisonMode,
    "exact-fragment-match",
    "pointer-fine should no longer be tracked as a Tailwind-only variant gap",
  );
  assert.ok(
    pointerFine?.dxStyleRequiredFragments?.includes("@media (pointer: fine)"),
    "pointer-fine should prove dx-style emits the pointer media query",
  );
  assert.ok(
    pointerFine?.dxStyleRequiredFragments?.includes("opacity: 100%"),
    "pointer-fine should keep its utility declaration guarded",
  );
  for (const [className, media] of [
    ["motion-safe:opacity-100", "@media (prefers-reduced-motion: no-preference)"],
    ["motion-reduce:opacity-100", "@media (prefers-reduced-motion: reduce)"],
    ["contrast-more:opacity-100", "@media (prefers-contrast: more)"],
    ["contrast-less:opacity-100", "@media (prefers-contrast: less)"],
    ["forced-colors:opacity-100", "@media (forced-colors: active)"],
    ["inverted-colors:opacity-100", "@media (inverted-colors: inverted)"],
    ["pointer-none:opacity-100", "@media (pointer: none)"],
    ["pointer-coarse:opacity-100", "@media (pointer: coarse)"],
    ["any-pointer-none:opacity-100", "@media (any-pointer: none)"],
    ["any-pointer-coarse:opacity-100", "@media (any-pointer: coarse)"],
    ["any-pointer-fine:opacity-100", "@media (any-pointer: fine)"],
    ["noscript:opacity-100", "@media (scripting: none)"],
    ["portrait:opacity-100", "@media (orientation: portrait)"],
    ["landscape:opacity-100", "@media (orientation: landscape)"],
    ["print:opacity-100", "@media print"],
    ["not-motion-safe:opacity-100", "@media not (prefers-reduced-motion: no-preference)"],
    ["not-motion-reduce:opacity-100", "@media not (prefers-reduced-motion: reduce)"],
    ["not-pointer-fine:opacity-100", "@media not (pointer: fine)"],
    ["not-forced-colors:opacity-100", "@media not (forced-colors: active)"],
    ["not-portrait:opacity-100", "@media not (orientation: portrait)"],
    ["not-landscape:opacity-100", "@media not (orientation: landscape)"],
    ["not-noscript:opacity-100", "@media not (scripting: none)"],
    ["not-print:opacity-100", "@media not print"],
  ]) {
    const entry = matrixEntries.get(className);
    assert.equal(entry?.comparisonMode, "exact-fragment-match");
    assert.ok(
      entry?.dxStyleRequiredFragments?.includes(media),
      `${className} should prove dx-style emits ${media}`,
    );
    assert.ok(
      entry?.dxStyleRequiredFragments?.includes("opacity: 100%"),
      `${className} should keep the opacity fragment guarded`,
    );
  }

  for (const [className, fragments] of [
    ["rtl:ps-4", [":where(:dir(rtl), [dir=\"rtl\"], [dir=\"rtl\"] *)", "padding-inline-start"]],
    ["ltr:pe-4", [":where(:dir(ltr), [dir=\"ltr\"], [dir=\"ltr\"] *)", "padding-inline-end"]],
    ["inert:opacity-50", [":is([inert], [inert] *)", "opacity: 50%"]],
    ["open:bg-blue-500", [":is([open], :popover-open, :open)", "background-color"]],
    ["starting:open:opacity-0", ["@starting-style", ":is([open], :popover-open, :open)", "opacity: 0%"]],
    ["user-valid:border-green-500", [":user-valid", "border-color"]],
    ["user-invalid:border-red-500", [":user-invalid", "border-color"]],
    ["details-content:bg-slate-100", [":details-content", "background-color"]],
    ["file:flex", ["::file-selector-button", "display: flex"]],
  ]) {
    const entry = matrixEntries.get(className);
    assert.equal(entry?.comparisonMode, "exact-fragment-match");
    for (const fragment of fragments) {
      assert.ok(
        entry?.dxStyleRequiredFragments?.some((candidate) => candidate.includes(fragment)),
        `${className} should prove dx-style emits ${fragment}`,
      );
    }
  }

  for (const [className, fragments] of [
    ["@3xs:grid", ["@container (width >= 16rem)", "display: grid"]],
    ["@7xl:flex", ["@container (width >= 80rem)", "display: flex"]],
    ["@max-3xs:hidden", ["@container (width < 16rem)", "display: none"]],
    ["@max-7xl:block", ["@container (width < 80rem)", "display: block"]],
    ["@[475px]:flex", ["@container (width >= 475px)", "display: flex"]],
    ["@[475px]/card:grid", ["@container card (width >= 475px)", "display: grid"]],
    [
      "@[475px]:@max-[960px]:block",
      ["@container (width >= 475px)", "@container (width < 960px)", "display: block"],
    ],
    [
      "@[475px]/card:@max-[960px]/card:hidden",
      ["@container card (width >= 475px)", "@container card (width < 960px)", "display: none"],
    ],
  ]) {
    const entry = matrixEntries.get(className);
    assert.equal(
      entry?.comparisonMode,
      "exact-fragment-match",
      `${className} should use Tailwind v4.3 range syntax directly`,
    );
    for (const fragment of fragments) {
      assert.ok(
        entry?.dxStyleRequiredFragments?.some((candidate) => candidate.includes(fragment)),
        `${className} should prove dx-style emits ${fragment}`,
      );
    }
  }

  for (const [className, fragments] of [
    [
      "backdrop:bg-slate-950/50",
      [
        "::backdrop",
        "color-mix(in srgb, oklch(12.9% 0.042 264.695) 50%, transparent)",
        "color-mix(in oklab, var(--color-slate-950) 50%, transparent)",
      ],
    ],
    ["@3xl/main:opacity-100", ["@container main (width >= 48rem)", "opacity: 100%"]],
  ]) {
    const entry = matrixEntries.get(className);
    assert.equal(
      entry?.comparisonMode,
      "exact-fragment-match",
      `${className} should prove exact governed Tailwind v4.3 fragments`,
    );
    for (const fragment of fragments) {
      assert.ok(
        entry?.dxStyleRequiredFragments?.some((candidate) => candidate.includes(fragment)),
        `${className} should prove dx-style emits ${fragment}`,
      );
    }
  }

  for (const [className, fragments] of [
    ["group-odd:bg-mauve-500", [":nth-child(odd)", "background-color: var(--color-mauve-500)"]],
    ["group-disabled:opacity-100", [":disabled", "opacity: 100%"]],
    ["group-focus:text-slate-900", [":focus", "color:"]],
    ["group-active:opacity-100", [":active", "opacity: 100%"]],
    ["group-focus-visible/card:opacity-100", [":focus-visible", "opacity: 100%"]],
    ["group-focus/nav:text-slate-900", [":focus", "color:"]],
    ["peer-invalid:visible", [":invalid", "visibility: visible"]],
    ["peer-focus:opacity-100", [":focus", "opacity: 100%"]],
    ["peer-checked:opacity-100", [":checked", "opacity: 100%"]],
    ["peer-required/email:block", [":required", "display: block"]],
    ["peer-checked/published:opacity-100", [":checked", "opacity: 100%"]],
    ["peer-disabled:opacity-100", [":disabled", "opacity: 100%"]],
    ["has-checked:opacity-100", [":has(*:checked)", "opacity: 100%"]],
    ["has-disabled:opacity-100", [":has(*:disabled)", "opacity: 100%"]],
    ["not-checked:opacity-100", [":not(*:checked)", "opacity: 100%"]],
    ["in-checked:opacity-100", [":where(*:checked)", "opacity: 100%"]],
    ["in-hover:opacity-100", [":where(*:hover)", "@media (hover: hover)", "opacity: 100%"]],
    ["not-hover:opacity-100", [":not(*:hover)", "@media not (hover: hover)", "opacity: 100%"]],
    ["hover:not-focus:opacity-100", [":hover", "@media (hover: hover)", ":not(*:focus)", "opacity: 100%"]],
    ["not-focus:hover:opacity-100", [":not(*:focus)", ":hover", "@media (hover: hover)", "opacity: 100%"]],
    [
      "group-hover:not-focus:opacity-100",
      [":is(:where(.group):hover *)", "@media (hover: hover)", ":not(*:focus)", "opacity: 100%"],
    ],
    ["in-focus:hover:opacity-100", [":where(*:focus)", ":hover", "@media (hover: hover)", "opacity: 100%"]],
    [
      "not-pointer-fine:hover:opacity-100",
      ["@media not (pointer: fine)", ":hover", "@media (hover: hover)", "opacity: 100%"],
    ],
    [
      "hover:not-pointer-fine:opacity-100",
      [":hover", "@media (hover: hover)", "@media not (pointer: fine)", "opacity: 100%"],
    ],
    ["group-has-checked:opacity-100", [":is(:where(.group):has(*:checked) *)", "opacity: 100%"]],
    [
      "group-has-disabled/card:opacity-100",
      [":is(:where(.group\\/card):has(*:disabled) *)", "opacity: 100%"],
    ],
    ["peer-has-checked:opacity-100", [":is(:where(.peer):has(*:checked) ~ *)", "opacity: 100%"]],
    [
      "peer-has-disabled/card:opacity-100",
      [":is(:where(.peer\\/card):has(*:disabled) ~ *)", "opacity: 100%"],
    ],
    [
      "group-not-disabled/card:opacity-100",
      [":is(:where(.group\\/card):not(*:disabled) *)", "opacity: 100%"],
    ],
    ["peer-not-checked:opacity-100", [":is(:where(.peer):not(*:checked) ~ *)", "opacity: 100%"]],
    [
      "peer-not-disabled/card:opacity-100",
      [":is(:where(.peer\\/card):not(*:disabled) ~ *)", "opacity: 100%"],
    ],
  ]) {
    const entry = matrixEntries.get(className);
    assert.equal(
      entry?.comparisonMode,
      "exact-fragment-match",
      `${className} should be promoted after exact Tailwind selector serialization is proven`,
    );
    for (const fragment of fragments) {
      assert.ok(
        entry?.dxStyleRequiredFragments?.some((candidate) => candidate.includes(fragment)),
        `${className} should prove dx-style emits ${fragment}`,
      );
    }
  }

  const unknownAtRule = matrixEntries.get("[@unknown_rule]:p-4");
  assert.equal(
    unknownAtRule?.comparisonMode,
    "exact-fragment-match",
    "safe unknown arbitrary at-rules should no longer be tracked as Tailwind-only gaps",
  );
  assert.ok(
    unknownAtRule?.dxStyleRequiredFragments?.includes("@unknown rule"),
    "unknown arbitrary at-rule should prove dx-style emits the decoded at-rule",
  );
  assert.ok(
    unknownAtRule?.dxStyleRequiredFragments?.includes("padding: calc(var(--spacing) * 4)"),
    "unknown arbitrary at-rule should keep its utility declaration guarded",
  );

  for (const [className, dxFragments, tailwindFragments = dxFragments] of [
    ["not-[@media_print]:flex", ["@media not print", "display: flex"]],
    ["not-[@media_not_print]:flex", ["@media print", "display: flex"]],
    ["not-[@supports(display:grid)]:flex", ["@supports not (display:grid)", "display: flex"]],
    ["not-[@container_(width>=32rem)]:flex", ["@container not (width>=32rem)", "display: flex"]],
    ["not-[@container_card_(width>=32rem)]:flex", ["@container card not (width>=32rem)", "display: flex"]],
    ["[&.foo]:[&.bar]:flex", [".foo.bar", "display: flex"], ["&.foo", "&.bar", "display: flex"]],
    ["[&_p]:[&_.lead]:mt-4", [" p .lead", "margin-top"], ["& p", "& .lead", "margin-top"]],
    [
      "not-[.is-open]:[&.dismissible]:opacity-100",
      [":not(.is-open).dismissible", "opacity: 100%"],
      ["&:not(*:is(.is-open))", "&.dismissible", "opacity: 100%"],
    ],
    [
      "[&.is-dragging]:active:cursor-grabbing",
      [".is-dragging:active", "cursor: grabbing"],
      ["&.is-dragging", "&:active", "cursor: grabbing"],
    ],
    [
      "[&.foo,&.bar]:[&>.item,&>[data-slot=control]]:opacity-100",
      [".foo>.item", ".foo>[data-slot=control]", ".bar>.item", ".bar>[data-slot=control]", "opacity: 100%"],
      ["&.foo,&.bar", "&>.item,&>[data-slot=control]", "opacity: 100%"],
    ],
    [
      "group-[.is-published]:block",
      [":is(:where(.group):is(.is-published) *)", "display: block"],
      ["&:is(:where(.group):is(.is-published) *)", "display: block"],
    ],
    [
      "group-[:nth-of-type(3)_&]:block",
      [":is(:nth-of-type(3) :where(.group) *)", "display: block"],
      ["&:is(:nth-of-type(3) :where(.group) *)", "display: block"],
    ],
    [
      "group-[&.foo,&.bar]:block",
      [":is(:is(:where(.group).foo,:where(.group).bar) *)", "display: block"],
      ["&:is(:is(:where(.group).foo,:where(.group).bar) *)", "display: block"],
    ],
    [
      "group-[&:is(.foo,.bar)]:block",
      [":is(:where(.group):is(.foo,.bar) *)", "display: block"],
      ["&:is(:where(.group):is(.foo,.bar) *)", "display: block"],
    ],
    [
      "group-[.is-open]/card:block",
      [":is(:where(.group\\/card):is(.is-open) *)", "display: block"],
      ["&:is(:where(.group\\/card):is(.is-open) *)", "display: block"],
    ],
    [
      "peer-[.is-dirty]:block",
      [":is(:where(.peer):is(.is-dirty) ~ *)", "display: block"],
      ["&:is(:where(.peer):is(.is-dirty) ~ *)", "display: block"],
    ],
    [
      "peer-[&.dirty,&.touched]:block",
      [":is(:is(:where(.peer).dirty,:where(.peer).touched) ~ *)", "display: block"],
      ["&:is(:is(:where(.peer).dirty,:where(.peer).touched) ~ *)", "display: block"],
    ],
    [
      "peer-[:nth-of-type(3)_&]:block",
      [":is(:nth-of-type(3) :where(.peer) ~ *)", "display: block"],
      ["&:is(:nth-of-type(3) :where(.peer) ~ *)", "display: block"],
    ],
    [
      "peer-[.is-dirty]:peer-required:block",
      [":is(:where(.peer):is(.is-dirty) ~ *)", ":is(:where(.peer):required ~ *)", "display: block"],
      ["&:is(:where(.peer):is(.is-dirty) ~ *)", "&:is(:where(.peer):required ~ *)", "display: block"],
    ],
    [
      "group-[.is-open]:[&.target]:opacity-100",
      [":is(:where(.group):is(.is-open) *)", ".target", "opacity: 100%"],
      ["&:is(:where(.group):is(.is-open) *)", "&.target", "opacity: 100%"],
    ],
  ]) {
    const entry = matrixEntries.get(className);
    assert.equal(
      entry?.comparisonMode,
      "exact-fragment-match",
      `${className} should be a supported negated arbitrary at-rule canary`,
    );
    assert.equal(entry?.area, "arbitrary-variants");
    for (const fragment of tailwindFragments) {
      assert.ok(
        entry?.tailwindRequiredFragments?.some((candidate) => candidate.includes(fragment)),
        `${className} should require Tailwind fragment ${fragment}`,
      );
    }
    for (const fragment of dxFragments) {
      assert.ok(
        entry?.dxStyleRequiredFragments?.some((candidate) => candidate.includes(fragment)),
        `${className} should require dx-style fragment ${fragment}`,
      );
    }
  }

  for (const entry of matrix.classes) {
    assert.ok(entry.className, "each official fixture entry needs a className");
    assert.ok(entry.area, `${entry.className} needs an area`);
    assert.ok(entry.sourceIds?.length, `${entry.className} needs official source ids`);
    assert.ok(
      ["exact-fragment-match", "known-different", "tailwind-only-gap"].includes(
        entry.comparisonMode,
      ),
      `${entry.className} has invalid comparisonMode ${entry.comparisonMode}`,
    );
    assert.ok(
      entry.tailwindRequiredFragments?.length,
      `${entry.className} needs Tailwind output fragments`,
    );
    if (entry.comparisonMode !== "tailwind-only-gap") {
      assert.ok(
        entry.dxStyleRequiredFragments?.length,
        `${entry.className} needs dx-style fragments`,
      );
    }
    if (entry.comparisonMode === "known-different") {
      assert.match(
        entry.differenceNote ?? "",
        /Tailwind v4\.3|dx-style/i,
        `${entry.className} known-different entries need an explicit reason`,
      );
    }
  }
});

test("official matrix is wired to a governed live Tailwind comparison runner", () => {
  const matrix = readJson(matrixPath);
  const runnerPath = path.join(root, matrix.liveComparison.runner);

  assert.ok(fs.existsSync(runnerPath), `expected ${matrix.liveComparison.runner} to exist`);
  assert.equal(matrix.liveComparison.tailwindInstallScope, "temporary-directory-only");
  assert.match(matrix.liveComparison.command, /tailwindcss@4\.3\.0/);
  assert.match(matrix.liveComparison.command, /@tailwindcss\/cli@4\.3\.0/);
  assert.doesNotMatch(matrix.liveComparison.command, /npm install(?![^&|;]*--no-save)/);

  assert.equal(
    matrix.liveComparison.normalTest,
    "benchmarks/dx-style-live-tailwind-v43-comparison.test.ts",
  );
  assert.ok(
    fs.existsSync(path.join(root, matrix.liveComparison.normalTest)),
    "governed live comparison should be wired to a normal node --test guard",
  );
  assert.equal(matrix.liveComparison.normalTestRunsLiveTailwind, true);
  assert.match(matrix.liveComparison.normalTestCommand, /^node --test \.\\benchmarks\\/);
  assert.match(matrix.liveComparison.normalTestCommand, /dx-style-live-tailwind-v43-comparison\.test\.cjs$/);
  assert.equal(
    matrix.liveComparison.fixtureBinaryFreshnessTest,
    "benchmarks/dx-style-live-comparison-receipt-accuracy.test.ts",
  );
  assert.equal(matrix.liveComparison.receiptArtifactArgument, "--receipt <path>");
  assert.match(matrix.liveComparison.receiptArtifactPolicy ?? "", /nonzero exit code/);
  assert.equal(
    matrix.liveComparison.blockedReceiptSchema,
    "dx.style.liveTailwindComparisonBlockedReceipt",
  );
  assert.match(matrix.liveComparison.blockedReceiptPolicy ?? "", /Infrastructure failures/);
  for (const field of [
    "comparisonStatus",
    "blockerStage",
    "blockerMessage",
    "blockerCommand",
    "tailwindCssSource",
    "dxStyleCssSource",
    "comparedClassCount",
    "tailwindRuntimeDependency",
    "fullTailwindParity",
  ]) {
    assert.ok(
      matrix.liveComparison.blockedReceiptFields?.includes(field),
      `live comparison matrix should advertise blocked receipt field ${field}`,
    );
  }
  assert.equal(
    matrix.liveComparison.candidateInventoryCoverageReceiptField,
    "officialCandidateInventoryCoverage",
  );
  assert.match(
    matrix.liveComparison.candidateInventoryCoveragePolicy ?? "",
    /source-test candidate inventory/,
  );
  for (const field of [
    "matrixClassCount",
    "uniqueMatrixClassCount",
    "inventoryCandidateCount",
    "presentClassCount",
    "missingClassCount",
    "duplicateClassCount",
    "matrixClassesAllInInventory",
    "missingClassNames",
    "duplicateClassNames",
    "missingClassEntries",
    "missingSourceIdCounts",
  ]) {
    assert.ok(
      matrix.liveComparison.candidateInventoryCoverageFields?.includes(field),
      `live comparison matrix should advertise candidate inventory coverage field ${field}`,
    );
  }
  assert.equal(matrix.liveComparison.inputFingerprintReceiptField, "inputFingerprints");
  assert.match(
    matrix.liveComparison.inputFingerprintPolicy ?? "",
    /stable SHA-256/,
  );
  for (const field of [
    "fixtureMatrixClassesSha256",
    "fixtureMatrixComparisonSha256",
    "officialCandidateInventorySha256",
    "officialFixtureSnapshotsSha256",
  ]) {
    assert.ok(
      matrix.liveComparison.inputFingerprintFields?.includes(field),
      `live comparison matrix should advertise input fingerprint field ${field}`,
    );
  }
  assert.equal(matrix.liveComparison.matrixIntegrityReceiptField, "matrixIntegrity");
  assert.match(
    matrix.liveComparison.matrixIntegrityPolicy ?? "",
    /duplicate/i,
  );
  for (const field of [
    "valid",
    "classCount",
    "uniqueClassCount",
    "duplicateClassCount",
    "duplicateClassNames",
    "duplicateClassEntries",
  ]) {
    assert.ok(
      matrix.liveComparison.matrixIntegrityFields?.includes(field),
      `live comparison matrix should advertise matrix integrity field ${field}`,
    );
  }
  assert.equal(matrix.liveComparison.classificationSummaryReceiptField, "classificationSummary");
  assert.match(
    matrix.liveComparison.classificationSummaryPolicy ?? "",
    /known-different/,
  );
  for (const field of [
    "classCount",
    "exactFragmentMatchCount",
    "knownDifferentCount",
    "tailwindOnlyGapCount",
    "exactOutputParityPercent",
    "governedCompatibilityPercent",
    "byComparisonMode",
    "byOwnerLane",
  ]) {
    assert.ok(
      matrix.liveComparison.classificationSummaryFields?.includes(field),
      `live comparison matrix should advertise classification summary field ${field}`,
    );
  }
  assert.equal(matrix.liveComparison.comparisonResultSummaryReceiptField, "comparisonResultSummary");
  assert.match(
    matrix.liveComparison.comparisonResultSummaryPolicy ?? "",
    /pass\/fail|comparison mode/i,
  );
  for (const field of [
    "classCount",
    "passedCount",
    "failedCount",
    "passPercent",
    "byComparisonMode",
    "byOwnerLane",
  ]) {
    assert.ok(
      matrix.liveComparison.comparisonResultSummaryFields?.includes(field),
      `live comparison matrix should advertise comparison result summary field ${field}`,
    );
  }
  assert.equal(matrix.liveComparison.evidenceQualityReceiptField, "evidenceQuality");
  assert.match(
    matrix.liveComparison.evidenceQualityPolicy ?? "",
    /canonical live comparison/,
  );
  for (const field of [
    "canonicalLiveComparison",
    "comparisonStatus",
    "liveTailwindExecution",
    "tailwindCssSource",
    "dxStyleCssSource",
    "freshDxStyleOutput",
    "staleFixtureBinaryAllowed",
    "nonCanonicalReasons",
  ]) {
    assert.ok(
      matrix.liveComparison.evidenceQualityFields?.includes(field),
      `live comparison matrix should advertise evidence quality field ${field}`,
    );
  }
  assert.equal(matrix.liveComparison.tailwindCssFixtureArgument, "--tailwind-css <path>");
  assert.match(
    matrix.liveComparison.tailwindCssFixturePolicy ?? "",
    /Only Lane 7 tests/,
  );
  assert.match(
    matrix.liveComparison.fixtureBinaryFreshnessPolicy ?? "",
    /DX_STYLE_FIXTURE_CSS_BIN/,
  );
  assert.match(
    matrix.liveComparison.fixtureBinaryFreshnessPolicy ?? "",
    /DX_STYLE_ALLOW_STALE_FIXTURE_BIN=1/,
  );
  for (const field of [
    "failedClassNames",
    "failedUnsupportedByDxStyleClassNames",
    "failedMissingTailwindFragmentsClassNames",
    "failedMissingDxStyleFragmentsClassNames",
    "failedClassHandoffs",
    "failureLaneBuckets",
  ]) {
    assert.ok(
      matrix.liveComparison.failureBucketFields?.includes(field),
      `live comparison matrix should advertise receipt bucket ${field}`,
    );
  }

  const runnerSource = fs.readFileSync(runnerPath, "utf8");
  for (const marker of [
    "--validate-fixture-binary",
    "DX_STYLE_ALLOW_STALE_FIXTURE_BIN",
    "dxStyleCssSource",
    "tailwindCssSource",
    "--receipt",
    "--tailwind-css",
    "dxStyleFixtureBinaryFreshness",
    "failedClassNames",
    "dx.style.liveTailwindComparisonBlockedReceipt",
    "blockerStage",
    "officialCandidateInventoryCoverage",
    "inputFingerprints",
    "fixtureMatrixComparisonSha256",
    "matrixIntegrity",
    "duplicateClassNames",
    "classificationSummary",
    "governedCompatibilityPercent",
    "comparisonResultSummary",
    "byComparisonMode",
    "evidenceQuality",
    "canonicalLiveComparison",
    "nonCanonicalReasons",
  ]) {
    assert.match(runnerSource, new RegExp(marker));
  }
});

test("official matrix backs utility grammar and CSS directive gaps with executable canaries", () => {
  const matrix = readJson(matrixPath);
  const classAreas = new Set(matrix.classes.map((entry) => entry.area));
  const matrixEntries = new Map(matrix.classes.map((entry) => [entry.className, entry]));

  for (const expectedArea of [
    "layout",
    "flexbox-grid",
    "spacing",
    "sizing",
    "typography",
    "backgrounds",
    "borders",
    "effects",
    "filters",
    "tables",
    "transitions-animation",
    "transforms",
    "interactivity",
    "svg",
    "accessibility",
    "masks",
  ]) {
    assert.ok(
      classAreas.has(expectedArea),
      `official fixture matrix should include a utility canary for ${expectedArea}`,
    );
  }

  const classNames = new Set(matrix.classes.map((entry) => entry.className));
  for (const expectedClass of [
    "block",
    "inline-grid",
    "flow-root",
    "contents",
    "hidden",
    "inline-flex",
    "table-cell",
    "grid-cols-3",
    "flex-1",
    "flex-auto",
    "flex-initial",
    "flex-none",
    "grow",
    "grow-0",
    "shrink",
    "shrink-0",
    "basis-full",
    "basis-1/2",
    "grid-cols-subgrid",
    "auto-rows-min",
    "auto-cols-max",
    "col-start-auto",
    "col-end-auto",
    "row-start-auto",
    "row-end-auto",
    "place-content-evenly",
    "justify-items-normal",
    "self-baseline",
    "place-self-stretch",
    "p-4",
    "w-1/2",
    "border-s-2",
    "blur-sm",
    "table-auto",
    "translate-y-4",
    "stroke-[color:var(--dx-stroke)]",
    "forced-color-adjust-none",
  ]) {
    assert.ok(
      classNames.has(expectedClass),
      `official fixture matrix should include utility grammar canary ${expectedClass}`,
    );
  }

  for (const [className, fragments] of [
    ["inline-grid", ["display: inline-grid"]],
    ["flow-root", ["display: flow-root"]],
    ["contents", ["display: contents"]],
    ["hidden", ["display: none"]],
    ["inline-flex", ["display: inline-flex"]],
    ["table-cell", ["display: table-cell"]],
    ["flex-1", ["flex: 1"]],
    ["flex-auto", ["flex: auto"]],
    ["flex-initial", ["flex: 0 auto"]],
    ["flex-none", ["flex: none"]],
    ["grow", ["flex-grow: 1"]],
    ["grow-0", ["flex-grow: 0"]],
    ["shrink", ["flex-shrink: 1"]],
    ["shrink-0", ["flex-shrink: 0"]],
    ["basis-full", ["flex-basis: 100%"]],
    ["basis-1/2", ["flex-basis: calc(1 / 2 * 100%)"]],
    ["grid-cols-subgrid", ["grid-template-columns: subgrid"]],
    ["auto-rows-min", ["grid-auto-rows: min-content"]],
    ["auto-cols-max", ["grid-auto-columns: max-content"]],
    ["col-start-auto", ["grid-column-start: auto"]],
    ["col-end-auto", ["grid-column-end: auto"]],
    ["row-start-auto", ["grid-row-start: auto"]],
    ["row-end-auto", ["grid-row-end: auto"]],
    ["place-content-evenly", ["place-content: space-evenly"]],
    ["justify-items-normal", ["justify-items: normal"]],
    ["self-baseline", ["align-self: baseline"]],
    ["place-self-stretch", ["place-self: stretch"]],
  ]) {
    const entry = matrixEntries.get(className);
    assert.equal(
      entry?.comparisonMode,
      "exact-fragment-match",
      `${className} should prove exact Tailwind v4.3 core utility output`,
    );
    for (const fragment of fragments) {
      assert.ok(
        entry?.tailwindRequiredFragments?.includes(fragment),
        `${className} should require Tailwind fragment ${fragment}`,
      );
      assert.ok(
        entry?.dxStyleRequiredFragments?.includes(fragment),
        `${className} should require dx-style fragment ${fragment}`,
      );
    }
  }

  assert.equal(matrix.utilityDocsCoverage?.fullValueModifierParity, false);
  assert.match(matrix.utilityDocsCoverage?.scope ?? "", /not full value\/modifier parity/);
  const utilityCoverage = new Map(
    (matrix.utilityDocsCoverage?.areas ?? []).map((entry) => [entry.docsArea, entry]),
  );
  for (const expectedArea of classAreas) {
    if (
      expectedArea === "container-query" ||
      expectedArea === "colors" ||
      expectedArea === "variants" ||
      expectedArea === "arbitrary-variants"
    ) {
      continue;
    }
    const coverage = utilityCoverage.get(expectedArea);
    assert.ok(coverage, `utilityDocsCoverage should include ${expectedArea}`);
    assert.ok(
      classNames.has(coverage.className),
      `utilityDocsCoverage ${expectedArea} should point at a matrix class`,
    );
  }

  assert.ok(
    matrix.sources.some(
      (source) => source.url === "https://tailwindcss.com/docs/detecting-classes-in-source-files",
    ),
    "official fixture matrix should cite Tailwind source detection docs",
  );
  assert.equal(matrix.sourceScannerCanaries?.fullTailwindSourceDetectionParity, false);
  const sourceScannerCanaries = new Set(
    (matrix.sourceScannerCanaries?.canaries ?? []).map((entry) => entry.id),
  );
  for (const expectedCanary of [
    "tsx-static-object-map",
    "arbitrary-value-static-string",
    "template-interpolation-rejection",
    "css-source-inline-bridge",
  ]) {
    assert.ok(
      sourceScannerCanaries.has(expectedCanary),
      `official fixture matrix should include source scanner canary ${expectedCanary}`,
    );
  }

  const directives = new Map(
    (matrix.cssDirectiveCanaries ?? []).map((entry) => [entry.directive, entry]),
  );

  for (const [directive, status] of [
    ["@theme", "supported"],
    ["@source", "partial"],
    ["@source inline(...)", "partial"],
    ["@source not ...", "partial"],
    ["@utility", "partial"],
    ["@custom-variant", "partial"],
    ["CSS @variant", "partial"],
    ["@apply", "partial"],
    ["@reference", "partial"],
    ["--alpha()", "partial"],
    ["--spacing()", "partial"],
    ["@plugin", "unsupported-by-design"],
    ["@config", "unsupported-by-design"],
  ]) {
    const entry = directives.get(directive);
    assert.ok(entry, `official fixture matrix should include CSS directive canary ${directive}`);
    assert.equal(entry.dxStyleStatus, status);
    assert.equal(entry.fullTailwindParity, false);
    assert.ok(entry.canaries?.length, `${directive} should name executable canaries`);
    if (status === "unsupported-by-design") {
      assert.match(
        entry.unsupportedByDesignReason ?? "",
        /No Tailwind runtime/,
        `${directive} should explain the no-Tailwind-runtime boundary`,
      );
    }
  }

  const cssVariant = directives.get("CSS @variant");
  const themeDirective = directives.get("@theme");
  assert.ok(
    themeDirective?.canaries?.includes("@theme { --container-*: initial; --container-card: 40rem; }"),
    "@theme canaries should include container namespace reset behavior",
  );

  assert.ok(
    cssVariant?.canaries?.includes("@variant hover:focus"),
    "CSS @variant canaries should include Tailwind v4.3 stacked variant syntax",
  );
  assert.ok(
    cssVariant?.canaries?.includes("@variant hover, focus"),
    "CSS @variant canaries should include Tailwind v4.3 compound variant syntax",
  );
  assert.ok(
    cssVariant?.canaries?.includes("@variant group-hover"),
    "CSS @variant canaries should include grouped state variants",
  );
  assert.ok(
    cssVariant?.canaries?.includes("@variant md:theme-midnight"),
    "CSS @variant canaries should include stacked responsive custom variants",
  );

  const applyDirective = directives.get("@apply");
  for (const marker of [
    "@apply hover:bg-brand",
    "@apply md:px-4",
    "@apply dark:hover:bg-brand",
  ]) {
    assert.ok(
      applyDirective?.canaries?.includes(marker),
      `@apply canaries should include supported variant token ${marker}`,
    );
    assert.ok(
      !(applyDirective?.missingOrUnproven ?? []).includes(marker),
      `@apply should not keep ${marker} classified as an unproven gap after the variant slice`,
    );
  }
  assert.ok(
    applyDirective?.missingOrUnproven?.includes("@apply [@unknown_rule]:p-4"),
    "@apply should keep arbitrary at-rule variants as an explicit remaining gap",
  );

  const referenceDirective = directives.get("@reference");
  for (const marker of ['@reference "tailwindcss"', '@reference "./tokens.css"']) {
    assert.ok(
      referenceDirective?.canaries?.includes(marker),
      `@reference canaries should include supported reference token ${marker}`,
    );
  }
  assert.ok(
    referenceDirective?.missingOrUnproven?.includes("@reference package/subpath import semantics"),
    "@reference should keep package/subpath resolution as an explicit remaining gap",
  );
});
