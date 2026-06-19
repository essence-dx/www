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

test("dx-style owns Tailwind v4.3 selector variants from the official quick reference", () => {
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const states = readRequiredFile("related-crates/style/src/core/engine/states/mod.rs");
  const rustTest = readRequiredFile("related-crates/style/tests/arbitrary_variant_css.rs");

  for (const marker of [
    '("user-valid", ":user-valid")',
    '("user-invalid", ":user-invalid")',
    '("details-content", ":details-content")',
    '("backdrop", "::backdrop")',
    '("marker", "&::marker, & *::marker")',
    '("selection", "&::selection, & *::selection")',
    '("placeholder", "::placeholder")',
    '"file",',
    '"&::file-selector-button"',
    '("first-letter", "::first-letter")',
    '("first-line", "::first-line")',
    '("inert", "&:is([inert], [inert] *)")',
    '"rtl"',
    '&:where(:dir(rtl), [dir=\\"rtl\\"], [dir=\\"rtl\\"] *)',
    '"ltr"',
    '&:where(:dir(ltr), [dir=\\"ltr\\"], [dir=\\"ltr\\"] *)',
    '("open", "&:is([open], :popover-open, :open)")',
    '("@3xs", "16rem")',
    '("@2xs", "18rem")',
    '("@xs", "20rem")',
    '("@3xl", "48rem")',
    '("@4xl", "56rem")',
    '("@5xl", "64rem")',
    '("@6xl", "72rem")',
    '("@7xl", "80rem")',
    "fn split_top_level_selector_list(selector: &str) -> Vec<&str>",
    "fn compose_selector_wrapper(selector: &str, wrapper: &str) -> String",
    "fn compose_selector_wrappers(selector: &str, wrappers: &[String]) -> String",
  ]) {
    assert.match(engine, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    '"user-valid" => Some(":user-valid")',
    '"user-invalid" => Some(":user-invalid")',
    '"details-content" => Some(":details-content")',
    '"open" => Some(":is([open], :popover-open, :open)")',
    '"starting" => return Some("@starting-style".to_string())',
    '"motion-safe" => Some("(prefers-reduced-motion: no-preference)".to_string())',
    '"landscape" => Some("(orientation: landscape)".to_string())',
    '"@media not print"',
    "fn negated_arbitrary_at_rule_variant(part: &str)",
    '"@supports not {condition}"',
    '"@container not {condition}"',
    "fn in_hover_capability_variant_selector(part: &str)",
    "fn group_peer_attribute_variant_selector(part: &str)",
    'group_peer_tailwind_wrapper(kind, name, &selector)',
    '"*" => Some(":is(& > *)")',
    '"**" => Some(":is(& *)")',
    "fn has_top_level_selector_list(selector: &str) -> bool",
  ]) {
    assert.match(states, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const className of [
    "rtl:ps-4",
    "ltr:pe-4",
    "inert:opacity-50",
    "open:bg-blue-500",
    "starting:open:opacity-0",
    "user-valid:border-green-500",
    "user-invalid:border-red-500",
    "details-content:bg-slate-100",
    "backdrop:bg-slate-950/50",
    "marker:text-red-500",
    "selection:flex",
    "placeholder:flex",
    "file:flex",
    "first-letter:uppercase",
    "first-line:uppercase",
    "motion-safe:opacity-100",
    "motion-reduce:opacity-100",
    "contrast-more:opacity-100",
    "contrast-less:opacity-100",
    "forced-colors:opacity-100",
    "inverted-colors:opacity-100",
    "pointer-none:opacity-100",
    "pointer-coarse:opacity-100",
    "pointer-fine:opacity-100",
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
    "group-hover:opacity-100",
    "peer-hover:opacity-100",
    "group-odd:bg-mauve-500",
    "group-disabled:opacity-100",
    "group-focus:text-slate-900",
    "group-active:opacity-100",
    "group-focus-visible/card:opacity-100",
    "group-focus/nav:text-slate-900",
    "group-aria-[sort=ascending]:rotate-0",
    "group-data-[state=open]/menu:block",
    "peer-invalid:visible",
    "peer-focus:opacity-100",
    "peer-checked:opacity-100",
    "peer-required/email:block",
    "peer-checked/published:opacity-100",
    "peer-disabled:opacity-100",
    "peer-aria-expanded/menu:opacity-100",
    "peer-data-[side=top]:translate-y-1",
    "*:p-4",
    "**:text-slate-900",
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
    "@3xs:grid",
    "@7xl:flex",
    "@max-3xs:hidden",
    "@max-7xl:block",
    "@3xl/main:opacity-100",
    "not-[@media_print]:flex",
    "not-[@media_not_print]:flex",
    "not-[@supports(display:grid)]:flex",
    "not-[@container_(width>=32rem)]:flex",
    "not-[@container_card_(width>=32rem)]:flex",
    "[&.foo]:[&.bar]:flex",
    "[&_p]:[&_.lead]:mt-4",
    "not-[.is-open]:[&.dismissible]:opacity-100",
    "[&.is-dragging]:active:cursor-grabbing",
    "[&.foo,&.bar]:[&>.item,&>[data-slot=control]]:opacity-100",
    "group-[.is-published]:block",
    "group-[:nth-of-type(3)_&]:block",
    "group-[&.foo,&.bar]:block",
    "group-[&:is(.foo,.bar)]:block",
    "group-[.is-open]/card:block",
    "peer-[.is-dirty]:block",
    "peer-[&.dirty,&.touched]:block",
    "peer-[:nth-of-type(3)_&]:block",
    "peer-[.is-dirty]:peer-required:block",
    "group-[.is-open]:[&.target]:opacity-100",
  ]) {
    assert.match(rustTest, new RegExp(className.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
});

test("official fixture matrix promotes selector variants and safe unknown at-rules", () => {
  const matrix = JSON.parse(
    readRequiredFile("related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json"),
  );
  const byClass = new Map(matrix.classes.map((entry) => [entry.className, entry]));

  for (const [className, fragments] of [
    [
      "rtl:ps-4",
      [":where(:dir(rtl), [dir=\"rtl\"], [dir=\"rtl\"] *)", "padding-inline-start: calc(var(--spacing) * 4)"],
    ],
    [
      "ltr:pe-4",
      [":where(:dir(ltr), [dir=\"ltr\"], [dir=\"ltr\"] *)", "padding-inline-end: calc(var(--spacing) * 4)"],
    ],
    ["inert:opacity-50", [":is([inert], [inert] *)", "opacity: 50%"]],
    ["open:bg-blue-500", [":is([open], :popover-open, :open)", "background-color"]],
    ["starting:open:opacity-0", ["@starting-style", ":is([open], :popover-open, :open)", "opacity: 0"]],
    ["user-valid:border-green-500", [":user-valid", "border-color"]],
    ["user-invalid:border-red-500", [":user-invalid", "border-color"]],
    ["details-content:bg-slate-100", [":details-content", "background-color"]],
    ["motion-safe:opacity-100", ["@media (prefers-reduced-motion: no-preference)", "opacity: 100%"]],
    ["motion-reduce:opacity-100", ["@media (prefers-reduced-motion: reduce)", "opacity: 100%"]],
    ["contrast-more:opacity-100", ["@media (prefers-contrast: more)", "opacity: 100%"]],
    ["contrast-less:opacity-100", ["@media (prefers-contrast: less)", "opacity: 100%"]],
    ["forced-colors:opacity-100", ["@media (forced-colors: active)", "opacity: 100%"]],
    ["inverted-colors:opacity-100", ["@media (inverted-colors: inverted)", "opacity: 100%"]],
    ["pointer-none:opacity-100", ["@media (pointer: none)", "opacity: 100%"]],
    ["pointer-coarse:opacity-100", ["@media (pointer: coarse)", "opacity: 100%"]],
    ["pointer-fine:opacity-100", ["@media (pointer: fine)", "opacity: 100%"]],
    ["any-pointer-none:opacity-100", ["@media (any-pointer: none)", "opacity: 100%"]],
    ["any-pointer-coarse:opacity-100", ["@media (any-pointer: coarse)", "opacity: 100%"]],
    ["any-pointer-fine:opacity-100", ["@media (any-pointer: fine)", "opacity: 100%"]],
    ["noscript:opacity-100", ["@media (scripting: none)", "opacity: 100%"]],
    ["portrait:opacity-100", ["@media (orientation: portrait)", "opacity: 100%"]],
    ["landscape:opacity-100", ["@media (orientation: landscape)", "opacity: 100%"]],
    ["print:opacity-100", ["@media print", "opacity: 100%"]],
    [
      "not-motion-safe:opacity-100",
      ["@media not (prefers-reduced-motion: no-preference)", "opacity: 100%"],
    ],
    ["not-motion-reduce:opacity-100", ["@media not (prefers-reduced-motion: reduce)", "opacity: 100%"]],
    ["not-pointer-fine:opacity-100", ["@media not (pointer: fine)", "opacity: 100%"]],
    ["not-forced-colors:opacity-100", ["@media not (forced-colors: active)", "opacity: 100%"]],
    ["not-portrait:opacity-100", ["@media not (orientation: portrait)", "opacity: 100%"]],
    ["not-landscape:opacity-100", ["@media not (orientation: landscape)", "opacity: 100%"]],
    ["not-noscript:opacity-100", ["@media not (scripting: none)", "opacity: 100%"]],
    ["not-print:opacity-100", ["@media not print", "opacity: 100%"]],
    ["[@unknown_rule]:p-4", ["@unknown rule", "padding: calc(var(--spacing) * 4)"]],
    ["marker:text-red-500", ["::marker", "*::marker", "color:"]],
    ["selection:flex", ["::selection", "*::selection", "display: flex"]],
    ["placeholder:flex", ["::placeholder", "display: flex"]],
    ["first-letter:uppercase", ["::first-letter", "text-transform: uppercase"]],
    ["first-line:uppercase", ["::first-line", "text-transform: uppercase"]],
    ["group-hover:opacity-100", ["@media (hover: hover)", ":is(:where(.group):hover *)", "opacity: 100%"]],
    ["peer-hover:opacity-100", ["@media (hover: hover)", ":is(:where(.peer):hover ~ *)", "opacity: 100%"]],
    ["group-odd:bg-mauve-500", [":is(:where(.group):nth-child(odd) *)", "background-color: var(--color-mauve-500)"]],
    ["group-disabled:opacity-100", [":is(:where(.group):disabled *)", "opacity: 100%"]],
    ["group-focus:text-slate-900", [":is(:where(.group):focus *)", "color:"]],
    ["group-active:opacity-100", [":is(:where(.group):active *)", "opacity: 100%"]],
    ["group-focus-visible/card:opacity-100", [":is(:where(.group\\/card):focus-visible *)", "opacity: 100%"]],
    ["group-focus/nav:text-slate-900", [":is(:where(.group\\/nav):focus *)", "color:"]],
    ["group-aria-[sort=ascending]:rotate-0", [":is(:where(.group)[aria-sort=\"ascending\"] *)", "rotate: 0deg"]],
    ["group-data-[state=open]/menu:block", [":is(:where(.group\\/menu)[data-state=\"open\"] *)", "display: block"]],
    ["peer-invalid:visible", [":is(:where(.peer):invalid ~ *)", "visibility: visible"]],
    ["peer-focus:opacity-100", [":is(:where(.peer):focus ~ *)", "opacity: 100%"]],
    ["peer-checked:opacity-100", [":is(:where(.peer):checked ~ *)", "opacity: 100%"]],
    ["peer-required/email:block", [":is(:where(.peer\\/email):required ~ *)", "display: block"]],
    ["peer-checked/published:opacity-100", [":is(:where(.peer\\/published):checked ~ *)", "opacity: 100%"]],
    ["peer-disabled:opacity-100", [":is(:where(.peer):disabled ~ *)", "opacity: 100%"]],
    ["peer-aria-expanded/menu:opacity-100", [":is(:where(.peer\\/menu)[aria-expanded=\"true\"] ~ *)", "opacity: 100%"]],
    [
      "peer-data-[side=top]:translate-y-1",
      [":is(:where(.peer)[data-side=\"top\"] ~ *)", "--tw-translate-y: calc(var(--spacing) * 1)"],
    ],
    ["*:p-4", [":is(", "> *", "padding: calc(var(--spacing) * 4)"]],
    ["**:text-slate-900", [":is(", " *", "color:"]],
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
    ["not-[@media_print]:flex", ["@media not print", "display: flex"]],
    ["not-[@media_not_print]:flex", ["@media print", "display: flex"]],
    ["not-[@supports(display:grid)]:flex", ["@supports not (display:grid)", "display: flex"]],
    ["not-[@container_(width>=32rem)]:flex", ["@container not (width>=32rem)", "display: flex"]],
    ["not-[@container_card_(width>=32rem)]:flex", ["@container card not (width>=32rem)", "display: flex"]],
    ["[&.foo]:[&.bar]:flex", [".foo.bar", "display: flex"]],
    ["[&_p]:[&_.lead]:mt-4", [" p .lead", "margin-top"]],
    ["not-[.is-open]:[&.dismissible]:opacity-100", [":not(.is-open).dismissible", "opacity: 100%"]],
    ["[&.is-dragging]:active:cursor-grabbing", [".is-dragging:active", "cursor: grabbing"]],
    [
      "[&.foo,&.bar]:[&>.item,&>[data-slot=control]]:opacity-100",
      [".foo>.item", ".foo>[data-slot=control]", ".bar>.item", ".bar>[data-slot=control]", "opacity: 100%"],
    ],
    ["group-[.is-published]:block", [":is(:where(.group):is(.is-published) *)", "display: block"]],
    ["group-[:nth-of-type(3)_&]:block", [":is(:nth-of-type(3) :where(.group) *)", "display: block"]],
    [
      "group-[&.foo,&.bar]:block",
      [":is(:is(:where(.group).foo,:where(.group).bar) *)", "display: block"],
    ],
    [
      "group-[&:is(.foo,.bar)]:block",
      [":is(:where(.group):is(.foo,.bar) *)", "display: block"],
    ],
    ["group-[.is-open]/card:block", [":is(:where(.group\\/card):is(.is-open) *)", "display: block"]],
    ["peer-[.is-dirty]:block", [":is(:where(.peer):is(.is-dirty) ~ *)", "display: block"]],
    [
      "peer-[&.dirty,&.touched]:block",
      [":is(:is(:where(.peer).dirty,:where(.peer).touched) ~ *)", "display: block"],
    ],
    ["peer-[:nth-of-type(3)_&]:block", [":is(:nth-of-type(3) :where(.peer) ~ *)", "display: block"]],
    [
      "peer-[.is-dirty]:peer-required:block",
      [":is(:where(.peer):is(.is-dirty) ~ *)", ":is(:where(.peer):required ~ *)", "display: block"],
    ],
    [
      "group-[.is-open]:[&.target]:opacity-100",
      [":is(:where(.group):is(.is-open) *)", ".target", "opacity: 100%"],
    ],
    ["@3xs:grid", ["@container (width >= 16rem)", "display: grid"]],
    ["@7xl:flex", ["@container (width >= 80rem)", "display: flex"]],
    ["@max-3xs:hidden", ["@container (width < 16rem)", "display: none"]],
    ["@max-7xl:block", ["@container (width < 80rem)", "display: block"]],
    ["file:flex", ["::file-selector-button", "display: flex"]],
  ]) {
    const entry = byClass.get(className);
    assert.ok(entry, `expected matrix entry for ${className}`);
    assert.equal(entry.comparisonMode, "exact-fragment-match");
    for (const fragment of fragments) {
      assert.ok(
        entry.dxStyleRequiredFragments?.some((candidate) => candidate.includes(fragment)),
        `${className} should require dx-style fragment ${fragment}`,
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
    const entry = byClass.get(className);
    assert.ok(entry, `expected matrix entry for ${className}`);
    assert.equal(entry.comparisonMode, "exact-fragment-match");
    for (const fragment of fragments) {
      assert.ok(
        entry.dxStyleRequiredFragments?.some((candidate) => candidate.includes(fragment)),
        `${className} should require dx-style fragment ${fragment}`,
      );
    }
  }

  assert.equal(matrix.fullTailwindParity, false);
});
