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

test("dx-style preserves Tailwind container-query range nesting order", () => {
  const states = readRequiredFile("related-crates/style/src/core/engine/states/mod.rs");
  const rustTest = readRequiredFile("related-crates/style/tests/arbitrary_variant_css.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");

  for (const marker of [
    "ContainerQueryVariant",
    "container_query_range_condition",
    "@max-md:@sm:flex",
    "@sm:@max-md:flex",
    "@max-md/main:@sm/main:grid",
    "@min-[456px]/name:grid",
    "@max-[456px]/name:block",
    "@max-[960px]:@min-[475px]:hidden",
    "@max-[960px]/name:@min-[475px]/name:flex",
    "tailwind_container_query_range_variants_preserve_tailwind_variant_order",
  ]) {
    assert.match(states + rustTest + parity, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.doesNotMatch(
    states,
    /sort_container_query_range_wrappers\(&mut media_queries\)/,
    "container query wrappers should preserve Tailwind's written variant order instead of canonical sorting",
  );
});

test("official fixture matrix promotes container ranges and arbitrary named queries to exact fragments", () => {
  const matrix = JSON.parse(
    readRequiredFile("related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json"),
  );
  const byClass = new Map(matrix.classes.map((entry) => [entry.className, entry]));

  for (const [className, fragments] of [
    [
      "@max-md:@sm:flex",
      ["@container (width < 28rem)", "@container (width >= 24rem)", "display: flex"],
    ],
    [
      "@sm:@max-md:flex",
      ["@container (width >= 24rem)", "@container (width < 28rem)", "display: flex"],
    ],
    [
      "@max-md/main:@sm/main:grid",
      [
        "@container main (width < 28rem)",
        "@container main (width >= 24rem)",
        "display: grid",
      ],
    ],
    [
      "@max-[960px]:@min-[475px]:hidden",
      ["@container (width < 960px)", "@container (width >= 475px)", "display: none"],
    ],
    [
      "@min-[456px]/name:grid",
      ["@container name (width >= 456px)", "display: grid"],
    ],
    [
      "@max-[456px]/name:block",
      ["@container name (width < 456px)", "display: block"],
    ],
    [
      "@max-[960px]/name:@min-[475px]/name:flex",
      ["@container name (width < 960px)", "@container name (width >= 475px)", "display: flex"],
    ],
  ]) {
    const entry = byClass.get(className);
    assert.ok(entry, `expected matrix entry for ${className}`);
    assert.equal(entry.area, "container-query");
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

test("official fixture matrix covers the full default container query ladder", () => {
  const matrix = JSON.parse(
    readRequiredFile("related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json"),
  );
  const byClass = new Map(matrix.classes.map((entry) => [entry.className, entry]));

  for (const [className, fragments] of [
    ["@3xs:grid", ["@container (width >= 16rem)", "display: grid"]],
    ["@2xs:flex", ["@container (width >= 18rem)", "display: flex"]],
    ["@xs:block", ["@container (width >= 20rem)", "display: block"]],
    ["@sm:grid", ["@container (width >= 24rem)", "display: grid"]],
    ["@md:flex", ["@container (width >= 28rem)", "display: flex"]],
    ["@lg:block", ["@container (width >= 32rem)", "display: block"]],
    ["@xl:grid", ["@container (width >= 36rem)", "display: grid"]],
    ["@2xl:flex", ["@container (width >= 42rem)", "display: flex"]],
    ["@3xl/block:flex", ["@container block (width >= 48rem)", "display: flex"]],
    ["@4xl:grid", ["@container (width >= 56rem)", "display: grid"]],
    ["@5xl:block", ["@container (width >= 64rem)", "display: block"]],
    ["@6xl:flex", ["@container (width >= 72rem)", "display: flex"]],
    ["@7xl:flex", ["@container (width >= 80rem)", "display: flex"]],
    ["@max-3xs:hidden", ["@container (width < 16rem)", "display: none"]],
    ["@max-2xs:block", ["@container (width < 18rem)", "display: block"]],
    ["@max-xs:flex", ["@container (width < 20rem)", "display: flex"]],
    ["@max-sm:hidden", ["@container (width < 24rem)", "display: none"]],
    ["@max-md:block", ["@container (width < 28rem)", "display: block"]],
    ["@max-lg:flex", ["@container (width < 32rem)", "display: flex"]],
    ["@max-xl:hidden", ["@container (width < 36rem)", "display: none"]],
    ["@max-2xl:block", ["@container (width < 42rem)", "display: block"]],
    ["@max-3xl:flex", ["@container (width < 48rem)", "display: flex"]],
    ["@max-4xl:hidden", ["@container (width < 56rem)", "display: none"]],
    ["@max-5xl:block", ["@container (width < 64rem)", "display: block"]],
    ["@max-6xl:flex", ["@container (width < 72rem)", "display: flex"]],
    ["@max-7xl:block", ["@container (width < 80rem)", "display: block"]],
  ]) {
    const entry = byClass.get(className);
    assert.ok(entry, `expected matrix entry for ${className}`);
    assert.equal(entry.area, "container-query");
    assert.equal(entry.comparisonMode, "exact-fragment-match");
    for (const fragment of fragments) {
      assert.ok(
        entry.dxStyleRequiredFragments?.some((candidate) => candidate.includes(fragment)),
        `${className} should require dx-style fragment ${fragment}`,
      );
    }
  }
});

test("official fixture matrix covers arbitrary shorthand container query variants", () => {
  const matrix = JSON.parse(
    readRequiredFile("related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json"),
  );
  const byClass = new Map(matrix.classes.map((entry) => [entry.className, entry]));

  for (const [className, fragments] of [
    ["@[475px]:flex", ["@container (width >= 475px)", "display: flex"]],
    ["@[475px]/card:grid", ["@container card (width >= 475px)", "display: grid"]],
    [
      "@[475px]:@max-[960px]:block",
      ["@container (width >= 475px)", "@container (width < 960px)", "display: block"],
    ],
    [
      "@[475px]/card:@max-[960px]/card:hidden",
      [
        "@container card (width >= 475px)",
        "@container card (width < 960px)",
        "display: none",
      ],
    ],
    [
      "@min-[40rem]:@max-[70rem]:flex",
      ["@container (width >= 40rem)", "@container (width < 70rem)", "display: flex"],
    ],
    [
      "@min-[40rem]/main:@max-[70rem]/main:grid",
      ["@container main (width >= 40rem)", "@container main (width < 70rem)", "display: grid"],
    ],
  ]) {
    const entry = byClass.get(className);
    assert.ok(entry, `expected matrix entry for ${className}`);
    assert.equal(entry.area, "container-query");
    assert.equal(entry.comparisonMode, "exact-fragment-match");
    for (const fragment of fragments) {
      assert.ok(
        entry.dxStyleRequiredFragments?.some((candidate) => candidate.includes(fragment)),
        `${className} should require dx-style fragment ${fragment}`,
      );
    }
  }
});

test("official fixture matrix covers multi-depth named container query stacks", () => {
  const matrix = JSON.parse(
    readRequiredFile("related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json"),
  );
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const byClass = new Map(matrix.classes.map((entry) => [entry.className, entry]));

  for (const [className, fragments] of [
    [
      "@max-7xl:@3xs:flex",
      ["@container (width < 80rem)", "@container (width >= 16rem)", "display: flex"],
    ],
    [
      "@3xs:@max-7xl:flex",
      ["@container (width >= 16rem)", "@container (width < 80rem)", "display: flex"],
    ],
    [
      "@max-7xl/main:@3xs/main:grid",
      ["@container main (width < 80rem)", "@container main (width >= 16rem)", "display: grid"],
    ],
    [
      "@3xs:@md:@max-7xl:flex",
      [
        "@container (width >= 16rem)",
        "@container (width >= 28rem)",
        "@container (width < 80rem)",
        "display: flex",
      ],
    ],
    [
      "@max-7xl:@md:@3xs:grid",
      [
        "@container (width < 80rem)",
        "@container (width >= 28rem)",
        "@container (width >= 16rem)",
        "display: grid",
      ],
    ],
    [
      "@min-[30rem]/rail:@max-[50rem]/rail:block",
      ["@container rail (width >= 30rem)", "@container rail (width < 50rem)", "display: block"],
    ],
    [
      "@max-[50rem]/rail:@min-[30rem]/rail:hidden",
      ["@container rail (width < 50rem)", "@container rail (width >= 30rem)", "display: none"],
    ],
  ]) {
    assert.match(parity, new RegExp(className.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));

    const entry = byClass.get(className);
    assert.ok(entry, `expected matrix entry for ${className}`);
    assert.equal(entry.area, "container-query");
    assert.equal(entry.comparisonMode, "exact-fragment-match");
    for (const fragment of fragments) {
      assert.ok(
        entry.dxStyleRequiredFragments?.some((candidate) => candidate.includes(fragment)),
        `${className} should require dx-style fragment ${fragment}`,
      );
    }
  }
});

test("CSS-first container theme namespace resets are guarded", () => {
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const themeCss = readRequiredFile("related-crates/style/src/core/engine/theme_css.rs");
  const rustTest = readRequiredFile("related-crates/style/tests/tailwind_v4_css_first.rs");
  const matrix = JSON.parse(
    readRequiredFile("related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json"),
  );

  for (const marker of [
    "apply_theme_namespace_reset",
    "is_theme_namespace_reset_token",
    "--container-*: initial",
    "@card:flex",
    "@max-card:hidden",
    "@dashboard/main:grid",
    "css_first_container_theme_namespace_reset_controls_container_variants",
  ]) {
    assert.match(
      engine + themeCss + rustTest,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
    );
  }

  const themeCanary = matrix.cssDirectiveCanaries.find((entry) => entry.directive === "@theme");
  assert.ok(themeCanary, "fixture matrix should carry @theme canaries");
  assert.ok(
    themeCanary.canaries.includes("@theme { --container-*: initial; --container-card: 40rem; }"),
    "@theme canaries should include container namespace reset behavior",
  );
});

test("official fixture matrix carries CSS-context custom container token canaries", () => {
  const matrix = JSON.parse(
    readRequiredFile("related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json"),
  );

  assert.ok(Array.isArray(matrix.containerQueryThemeCanaries));

  const customThemeCanary = matrix.containerQueryThemeCanaries.find(
    (entry) => entry.id === "css-theme-custom-container-tokens",
  );
  assert.ok(customThemeCanary, "expected a CSS-context custom container token canary");
  assert.equal(customThemeCanary.tailwindRuntimeDependency, false);
  assert.match(customThemeCanary.themeCss, /--container-card:\s*40rem/);
  assert.match(customThemeCanary.themeCss, /--container-dashboard:\s*52rem/);

  for (const [className, fragments] of [
    ["@card:flex", ["@container (width >= 40rem)", "display: flex"]],
    ["@max-card:hidden", ["@container (width < 40rem)", "display: none"]],
    ["@dashboard/main:grid", ["@container main (width >= 52rem)", "display: grid"]],
    ["@min-card:block", ["@container (width >= 40rem)", "display: block"]],
    ["@min-dashboard/main:flex", ["@container main (width >= 52rem)", "display: flex"]],
    ["@max-dashboard/main:hidden", ["@container main (width < 52rem)", "display: none"]],
  ]) {
    const sample = customThemeCanary.classes.find((entry) => entry.className === className);
    assert.ok(sample, `expected custom container canary for ${className}`);
    assert.equal(sample.comparisonMode, "exact-fragment-match");
    for (const fragment of fragments) {
      assert.ok(
        sample.dxStyleRequiredFragments?.some((candidate) => candidate.includes(fragment)),
        `${className} should require dx-style fragment ${fragment}`,
      );
    }
  }

  const resetCanary = matrix.containerQueryThemeCanaries.find(
    (entry) => entry.id === "css-theme-container-namespace-reset",
  );
  assert.ok(resetCanary, "expected a CSS-context container namespace reset canary");
  assert.equal(resetCanary.removesDefaultContainerScale, true);
  assert.match(resetCanary.themeCss, /--container-\*:\s*initial/);
});

test("Tailwind v4.3 normal container marker utilities are guarded", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = JSON.parse(
    readRequiredFile("related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json"),
  );

  for (const marker of [
    '"@container"',
    '"@container/sidebar"',
    '"@container-normal"',
    '"@container-normal/sidebar"',
    '"@container-size/sidebar"',
    '"container-type: inline-size"',
    '"container-type: normal"',
    '"container-type: size"',
    "Tailwind v4.3 normal container utilities generate normal CSS",
  ]) {
    assert.match(
      utility + parity,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
    );
  }

  const byClass = new Map(matrix.classes.map((entry) => [entry.className, entry]));
  for (const [className, fragments] of [
    ["@container", ["container-type: inline-size"]],
    ["@container/sidebar", ["container-type: inline-size", "container-name: sidebar"]],
    ["@container-normal", ["container-type: normal"]],
    ["@container-normal/sidebar", ["container-type: normal", "container-name: sidebar"]],
    ["@container-size/sidebar", ["container-type: size", "container-name: sidebar"]],
  ]) {
    const entry = byClass.get(className);
    assert.ok(entry, `expected matrix entry for ${className}`);
    assert.equal(entry.area, "container-query");
    assert.equal(entry.comparisonMode, "exact-fragment-match");
    for (const fragment of fragments) {
      assert.ok(
        entry.dxStyleRequiredFragments?.some((candidate) => candidate.includes(fragment)),
        `${className} should require dx-style fragment ${fragment}`,
      );
    }
  }
});
