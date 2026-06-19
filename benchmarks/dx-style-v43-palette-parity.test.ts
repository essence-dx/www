import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const utilitySource = fs.readFileSync(
  path.join(root, "related-crates/style/src/core/engine/utility/mod.rs"),
  "utf8",
);
const colorPaletteSource = fs.readFileSync(
  path.join(root, "related-crates/style/src/core/engine/utility/color_palette.rs"),
  "utf8",
);
const themeCssSource = fs.readFileSync(
  path.join(root, "related-crates/style/src/core/engine/theme_css.rs"),
  "utf8",
);
const fixtureMatrix = JSON.parse(
  fs.readFileSync(
    path.join(root, "related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json"),
    "utf8",
  ),
);
const COLOR_MIX_SUPPORTS = "@supports (color: color-mix(in lab, red, red))";
const officialOpacityModifiers = [
  ["10", "10%"],
  ["20", "20%"],
  ["30", "30%"],
  ["40", "40%"],
  ["50", "50%"],
  ["60", "60%"],
  ["70", "70%"],
  ["75", "75%"],
  ["80", "80%"],
  ["90", "90%"],
  ["100", "100%"],
  ["[71.37%]", "71.37%"],
  ["(--my-alpha-value)", "var(--my-alpha-value)"],
];

const neutralAdjacentPalettes = {
  mauve: {
    50: "oklch(98.5% 0 0)",
    100: "oklch(96% 0.003 325.6)",
    200: "oklch(92.2% 0.005 325.62)",
    300: "oklch(86.5% 0.012 325.68)",
    400: "oklch(71.1% 0.019 323.02)",
    500: "oklch(54.2% 0.034 322.5)",
    600: "oklch(43.5% 0.029 321.78)",
    700: "oklch(36.4% 0.029 323.89)",
    800: "oklch(26.3% 0.024 320.12)",
    900: "oklch(21.2% 0.019 322.12)",
    950: "oklch(14.5% 0.008 326)",
  },
  olive: {
    50: "oklch(98.8% 0.003 106.5)",
    100: "oklch(96.6% 0.005 106.5)",
    200: "oklch(93% 0.007 106.5)",
    300: "oklch(88% 0.011 106.6)",
    400: "oklch(73.7% 0.021 106.9)",
    500: "oklch(58% 0.031 107.3)",
    600: "oklch(46.6% 0.025 107.3)",
    700: "oklch(39.4% 0.023 107.4)",
    800: "oklch(28.6% 0.016 107.4)",
    900: "oklch(22.8% 0.013 107.4)",
    950: "oklch(15.3% 0.006 107.1)",
  },
  mist: {
    50: "oklch(98.7% 0.002 197.1)",
    100: "oklch(96.3% 0.002 197.1)",
    200: "oklch(92.5% 0.005 214.3)",
    300: "oklch(87.2% 0.007 219.6)",
    400: "oklch(72.3% 0.014 214.4)",
    500: "oklch(56% 0.021 213.5)",
    600: "oklch(45% 0.017 213.2)",
    700: "oklch(37.8% 0.015 216)",
    800: "oklch(27.5% 0.011 216.9)",
    900: "oklch(21.8% 0.008 223.9)",
    950: "oklch(14.8% 0.004 228.8)",
  },
  taupe: {
    50: "oklch(98.6% 0.002 67.8)",
    100: "oklch(96% 0.002 17.2)",
    200: "oklch(92.2% 0.005 34.3)",
    300: "oklch(86.8% 0.007 39.5)",
    400: "oklch(71.4% 0.014 41.2)",
    500: "oklch(54.7% 0.021 43.1)",
    600: "oklch(43.8% 0.017 39.3)",
    700: "oklch(36.7% 0.016 35.7)",
    800: "oklch(26.8% 0.011 36.5)",
    900: "oklch(21.4% 0.009 43.1)",
    950: "oklch(14.7% 0.004 49.3)",
  },
};

test("Tailwind v4.3 neutral-adjacent palettes are complete in source-owned OKLCH tokens", () => {
  for (const [palette, shades] of Object.entries(neutralAdjacentPalettes)) {
    for (const [shade, oklch] of Object.entries(shades)) {
      const entry = `("${palette}", "${shade}") => Some("${oklch}")`;
      assert.match(
        colorPaletteSource,
        new RegExp(entry.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
        `${palette}-${shade} should be present in the v4.3 baseline OKLCH palette table`,
      );

      const themeEntry = `--color-${palette}-${shade}: ${oklch};`;
      assert.match(
        themeCssSource,
        new RegExp(themeEntry.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
        `${palette}-${shade} should be emitted as a default theme token`,
      );
    }
  }

  assert.match(utilitySource, /color_palette::tailwind_v43_oklch_color\(name\)/);
  assert.match(utilitySource, /color-mix\(in oklab/);
  assert.match(utilitySource, /tailwind_v43_palette_opacity_property_css/);
});

test("official live matrix promotes v4.3 neutral-adjacent palette gaps into exact dx-style variable output", () => {
  const classes = new Map(fixtureMatrix.classes.map((entry) => [entry.className, entry]));

  for (const [className, dxStyleFragment] of [
    ["bg-mauve-500", "background-color: var(--color-mauve-500)"],
    ["bg-olive-500", "background-color: var(--color-olive-500)"],
    ["bg-mist-500", "background-color: var(--color-mist-500)"],
    ["bg-taupe-500", "background-color: var(--color-taupe-500)"],
  ]) {
    const entry = classes.get(className);
    assert.ok(entry, `fixture matrix should include ${className}`);
    assert.equal(entry.area, "colors");
    assert.equal(entry.comparisonMode, "exact-fragment-match");
    assert.deepEqual(entry.tailwindRequiredFragments, [dxStyleFragment]);
    assert.deepEqual(entry.dxStyleRequiredFragments, [dxStyleFragment]);
  }

  assert.equal(fixtureMatrix.fullTailwindParity, false);
});

test("official live matrix covers every v4.3 neutral-adjacent shade across Lane 1 color utility families", () => {
  const classes = new Map(fixtureMatrix.classes.map((entry) => [entry.className, entry]));
  const families = [
    ["bg", "background-color"],
    ["text", "color"],
    ["border", "border-color"],
    ["ring", "--tw-ring-color"],
    ["outline", "outline-color"],
    ["decoration", "text-decoration-color"],
    ["from", "--tw-gradient-from"],
    ["via", "--tw-gradient-via"],
    ["to", "--tw-gradient-to"],
  ];

  for (const [palette, shades] of Object.entries(neutralAdjacentPalettes)) {
    for (const shade of Object.keys(shades)) {
      for (const [prefix, property] of families) {
        const className = `${prefix}-${palette}-${shade}`;
        const fragment = `${property}: var(--color-${palette}-${shade})`;
        const entry = classes.get(className);

        assert.ok(entry, `fixture matrix should include ${className}`);
        assert.equal(entry.area, "colors");
        assert.equal(entry.comparisonMode, "exact-fragment-match");
        assert.deepEqual(entry.tailwindRequiredFragments, [fragment]);
        assert.deepEqual(entry.dxStyleRequiredFragments, [fragment]);
      }
    }
  }
});

test("official live matrix covers v4.3 neutral-adjacent opacity color-mix output across Lane 1 color utility families", () => {
  const classes = new Map(fixtureMatrix.classes.map((entry) => [entry.className, entry]));
  const families = [
    ["bg", "background-color"],
    ["text", "color"],
    ["border", "border-color"],
    ["ring", "--tw-ring-color"],
    ["outline", "outline-color"],
    ["decoration", "text-decoration-color"],
    ["from", "--tw-gradient-from"],
    ["via", "--tw-gradient-via"],
    ["to", "--tw-gradient-to"],
  ];

  for (const [palette, shades] of Object.entries(neutralAdjacentPalettes)) {
    const oklch = shades[500];

    for (const [prefix, property] of families) {
      const className = `${prefix}-${palette}-500/50`;
      const fragments = [
        `${property}: color-mix(in srgb, ${oklch} 50%, transparent)`,
        COLOR_MIX_SUPPORTS,
        `${property}: color-mix(in oklab, var(--color-${palette}-500) 50%, transparent)`,
      ];
      const entry = classes.get(className);

      assert.ok(entry, `fixture matrix should include ${className}`);
      assert.equal(entry.area, "colors");
      assert.equal(entry.comparisonMode, "exact-fragment-match");
      for (const fragment of fragments) {
        assert.ok(
          entry.tailwindRequiredFragments?.includes(fragment),
          `${className} should require Tailwind fragment ${fragment}`,
        );
        assert.ok(
          entry.dxStyleRequiredFragments?.includes(fragment),
          `${className} should require dx-style fragment ${fragment}`,
        );
      }
    }
  }
});

test("official live matrix covers every v4.3 neutral-adjacent shade across SVG and form color families", () => {
  const classes = new Map(fixtureMatrix.classes.map((entry) => [entry.className, entry]));
  const families = [
    ["fill", "fill"],
    ["stroke", "stroke"],
    ["accent", "accent-color"],
    ["caret", "caret-color"],
  ];

  for (const [palette, shades] of Object.entries(neutralAdjacentPalettes)) {
    for (const shade of Object.keys(shades)) {
      for (const [prefix, property] of families) {
        const className = `${prefix}-${palette}-${shade}`;
        const fragment = `${property}: var(--color-${palette}-${shade})`;
        const entry = classes.get(className);

        assert.ok(entry, `fixture matrix should include ${className}`);
        assert.equal(entry.area, "colors");
        assert.equal(entry.comparisonMode, "exact-fragment-match");
        assert.deepEqual(entry.tailwindRequiredFragments, [fragment]);
        assert.deepEqual(entry.dxStyleRequiredFragments, [fragment]);
      }
    }
  }
});

test("official live matrix covers opacity output across SVG and form color families", () => {
  const classes = new Map(fixtureMatrix.classes.map((entry) => [entry.className, entry]));
  const families = [
    ["fill", "fill"],
    ["stroke", "stroke"],
    ["accent", "accent-color"],
    ["caret", "caret-color"],
  ];

  for (const [palette, shades] of Object.entries(neutralAdjacentPalettes)) {
    const oklch = shades[500];

    for (const [prefix, property] of families) {
      const className = `${prefix}-${palette}-500/50`;
      const fragments = [
        `${property}: color-mix(in srgb, ${oklch} 50%, transparent)`,
        COLOR_MIX_SUPPORTS,
        `${property}: color-mix(in oklab, var(--color-${palette}-500) 50%, transparent)`,
      ];
      const entry = classes.get(className);

      assert.ok(entry, `fixture matrix should include ${className}`);
      assert.equal(entry.area, "colors");
      assert.equal(entry.comparisonMode, "exact-fragment-match");
      for (const fragment of fragments) {
        assert.ok(
          entry.tailwindRequiredFragments?.includes(fragment),
          `${className} should require Tailwind fragment ${fragment}`,
        );
        assert.ok(
          entry.dxStyleRequiredFragments?.includes(fragment),
          `${className} should require dx-style fragment ${fragment}`,
        );
      }
    }
  }
});

test("official live matrix covers every v4.3 neutral-adjacent shade across nested, child, and scrollbar color families", () => {
  const classes = new Map(fixtureMatrix.classes.map((entry) => [entry.className, entry]));
  const families = [
    ["placeholder", "colors", ["::placeholder", "color"]],
    ["divide", "colors", ["border-color"]],
    [
      "scrollbar-thumb",
      "interactivity",
      ["--tw-scrollbar-thumb", "scrollbar-color: var(--tw-scrollbar-thumb) var(--tw-scrollbar-track)"],
    ],
    [
      "scrollbar-track",
      "interactivity",
      ["--tw-scrollbar-track", "scrollbar-color: var(--tw-scrollbar-thumb) var(--tw-scrollbar-track)"],
    ],
  ];

  for (const [palette, shades] of Object.entries(neutralAdjacentPalettes)) {
    for (const shade of Object.keys(shades)) {
      for (const [prefix, area, fragments] of families) {
        const className = `${prefix}-${palette}-${shade}`;
        const entry = classes.get(className);

        assert.ok(entry, `fixture matrix should include ${className}`);
        assert.equal(entry.area, area);
        assert.equal(entry.comparisonMode, "exact-fragment-match");
        for (const fragment of fragments) {
          assert.ok(
            entry.tailwindRequiredFragments?.some((candidate) => candidate.includes(fragment)),
            `${className} should require Tailwind fragment ${fragment}`,
          );
          assert.ok(
            entry.dxStyleRequiredFragments?.some((candidate) => candidate.includes(fragment)),
            `${className} should require dx-style fragment ${fragment}`,
          );
        }
        assert.ok(
          entry.tailwindRequiredFragments?.some((fragment) =>
            fragment.includes(`var(--color-${palette}-${shade})`),
          ),
          `${className} should require a Tailwind palette token fragment`,
        );
        assert.ok(
          entry.dxStyleRequiredFragments?.some((fragment) =>
            fragment.includes(`var(--color-${palette}-${shade})`),
          ),
          `${className} should require a dx-style palette token fragment`,
        );
      }
    }
  }
});

test("official live matrix covers opacity output across nested, child, and scrollbar color families", () => {
  const classes = new Map(fixtureMatrix.classes.map((entry) => [entry.className, entry]));
  const families = [
    ["placeholder", "colors", "color"],
    ["divide", "colors", "border-color"],
    ["scrollbar-thumb", "interactivity", "--tw-scrollbar-thumb"],
    ["scrollbar-track", "interactivity", "--tw-scrollbar-track"],
  ];

  for (const [palette, shades] of Object.entries(neutralAdjacentPalettes)) {
    const oklch = shades[500];

    for (const [prefix, area, property] of families) {
      const className = `${prefix}-${palette}-500/50`;
      const fragments = [
        `${property}: color-mix(in srgb, ${oklch} 50%, transparent)`,
        COLOR_MIX_SUPPORTS,
        `${property}: color-mix(in oklab, var(--color-${palette}-500) 50%, transparent)`,
      ];
      const entry = classes.get(className);

      assert.ok(entry, `fixture matrix should include ${className}`);
      assert.equal(entry.area, area);
      assert.equal(entry.comparisonMode, "exact-fragment-match");
      for (const fragment of fragments) {
        assert.ok(
          entry.tailwindRequiredFragments?.includes(fragment),
          `${className} should require Tailwind fragment ${fragment}`,
        );
        assert.ok(
          entry.dxStyleRequiredFragments?.includes(fragment),
          `${className} should require dx-style fragment ${fragment}`,
        );
      }
    }
  }
});

test("official live matrix covers every v4.3 neutral-adjacent shade across shadow and ring color hooks", () => {
  const classes = new Map(fixtureMatrix.classes.map((entry) => [entry.className, entry]));
  const colorHookFamilies = [
    ["shadow", "--tw-shadow-color", "var(--tw-shadow-alpha)"],
    ["inset-shadow", "--tw-inset-shadow-color", "var(--tw-inset-shadow-alpha)"],
    ["drop-shadow", "--tw-drop-shadow-color", "var(--tw-drop-shadow-alpha)"],
  ];
  const directHookFamilies = [
    ["inset-ring", "--tw-inset-ring-color"],
    ["ring-offset", "--tw-ring-offset-color"],
  ];

  for (const [palette, shades] of Object.entries(neutralAdjacentPalettes)) {
    for (const [shade, oklch] of Object.entries(shades)) {
      for (const [prefix, property, alphaProperty] of colorHookFamilies) {
        const className = `${prefix}-${palette}-${shade}`;
        const entry = classes.get(className);
        const supported = `${property}: color-mix(in oklab, var(--color-${palette}-${shade}) ${alphaProperty}, transparent)`;

        assert.ok(entry, `fixture matrix should include ${className}`);
        assert.equal(entry.area, "colors");
        assert.equal(entry.comparisonMode, "exact-fragment-match");
        assert.ok(
          entry.tailwindRequiredFragments?.some((fragment) =>
            fragment.includes(`${property}: ${oklch.split(" ").slice(0, 1).join(" ")}`),
          ),
          `${className} should require Tailwind OKLCH fallback output`,
        );
        assert.ok(
          entry.dxStyleRequiredFragments?.includes(`${property}: ${oklch}`),
          `${className} should require dx-style OKLCH fallback output`,
        );
        assert.ok(entry.tailwindRequiredFragments?.includes(COLOR_MIX_SUPPORTS));
        assert.ok(entry.dxStyleRequiredFragments?.includes(COLOR_MIX_SUPPORTS));
        assert.ok(entry.tailwindRequiredFragments?.includes(supported));
        assert.ok(entry.dxStyleRequiredFragments?.includes(supported));
      }

      for (const [prefix, property] of directHookFamilies) {
        const className = `${prefix}-${palette}-${shade}`;
        const fragment = `${property}: var(--color-${palette}-${shade})`;
        const entry = classes.get(className);

        assert.ok(entry, `fixture matrix should include ${className}`);
        assert.equal(entry.area, "colors");
        assert.equal(entry.comparisonMode, "exact-fragment-match");
        assert.ok(entry.tailwindRequiredFragments?.includes(fragment));
        assert.ok(entry.dxStyleRequiredFragments?.includes(fragment));
      }
    }
  }
});

test("official live matrix covers opacity output across shadow and ring color hooks", () => {
  const classes = new Map(fixtureMatrix.classes.map((entry) => [entry.className, entry]));
  const colorHookFamilies = [
    ["shadow", "--tw-shadow-color", "var(--tw-shadow-alpha)"],
    ["inset-shadow", "--tw-inset-shadow-color", "var(--tw-inset-shadow-alpha)"],
    ["drop-shadow", "--tw-drop-shadow-color", "var(--tw-drop-shadow-alpha)"],
  ];
  const directHookFamilies = [
    ["inset-ring", "--tw-inset-ring-color"],
    ["ring-offset", "--tw-ring-offset-color"],
  ];

  for (const [palette, shades] of Object.entries(neutralAdjacentPalettes)) {
    const oklch = shades[500];

    for (const [prefix, property, alphaProperty] of colorHookFamilies) {
      const className = `${prefix}-${palette}-500/50`;
      const fragments = [
        `${property}: color-mix(in srgb, ${oklch} 50%, transparent)`,
        COLOR_MIX_SUPPORTS,
        `${property}: color-mix(in oklab, color-mix(in oklab, var(--color-${palette}-500) 50%, transparent) ${alphaProperty}, transparent)`,
      ];
      const entry = classes.get(className);

      assert.ok(entry, `fixture matrix should include ${className}`);
      assert.equal(entry.area, "colors");
      assert.equal(entry.comparisonMode, "exact-fragment-match");
      for (const fragment of fragments) {
        assert.ok(
          entry.tailwindRequiredFragments?.includes(fragment),
          `${className} should require Tailwind fragment ${fragment}`,
        );
        assert.ok(
          entry.dxStyleRequiredFragments?.includes(fragment),
          `${className} should require dx-style fragment ${fragment}`,
        );
      }
    }

    for (const [prefix, property] of directHookFamilies) {
      const className = `${prefix}-${palette}-500/50`;
      const fragments = [
        `${property}: color-mix(in srgb, ${oklch} 50%, transparent)`,
        COLOR_MIX_SUPPORTS,
        `${property}: color-mix(in oklab, var(--color-${palette}-500) 50%, transparent)`,
      ];
      const entry = classes.get(className);

      assert.ok(entry, `fixture matrix should include ${className}`);
      assert.equal(entry.area, "colors");
      assert.equal(entry.comparisonMode, "exact-fragment-match");
      for (const fragment of fragments) {
        assert.ok(
          entry.tailwindRequiredFragments?.includes(fragment),
          `${className} should require Tailwind fragment ${fragment}`,
        );
        assert.ok(
          entry.dxStyleRequiredFragments?.includes(fragment),
          `${className} should require dx-style fragment ${fragment}`,
        );
      }
    }
  }
});

test("official live matrix covers docs-backed opacity syntax across v4.3 neutral-adjacent color families", () => {
  const classes = new Map(fixtureMatrix.classes.map((entry) => [entry.className, entry]));
  const directFamilies = [
    ["bg", "colors", "background-color"],
    ["text", "colors", "color"],
    ["border", "colors", "border-color"],
    ["ring", "colors", "--tw-ring-color"],
    ["outline", "colors", "outline-color"],
    ["decoration", "colors", "text-decoration-color"],
    ["from", "colors", "--tw-gradient-from"],
    ["via", "colors", "--tw-gradient-via"],
    ["to", "colors", "--tw-gradient-to"],
    ["fill", "colors", "fill"],
    ["stroke", "colors", "stroke"],
    ["accent", "colors", "accent-color"],
    ["caret", "colors", "caret-color"],
    ["inset-ring", "colors", "--tw-inset-ring-color"],
    ["ring-offset", "colors", "--tw-ring-offset-color"],
  ];
  const nestedFamilies = [
    ["placeholder", "colors", "color", ["::placeholder"]],
    ["divide", "colors", "border-color", []],
    [
      "scrollbar-thumb",
      "interactivity",
      "--tw-scrollbar-thumb",
      ["scrollbar-color: var(--tw-scrollbar-thumb) var(--tw-scrollbar-track)"],
    ],
    [
      "scrollbar-track",
      "interactivity",
      "--tw-scrollbar-track",
      ["scrollbar-color: var(--tw-scrollbar-thumb) var(--tw-scrollbar-track)"],
    ],
  ];
  const alphaHookFamilies = [
    ["shadow", "colors", "--tw-shadow-color", "var(--tw-shadow-alpha)", []],
    [
      "inset-shadow",
      "colors",
      "--tw-inset-shadow-color",
      "var(--tw-inset-shadow-alpha)",
      [],
    ],
    [
      "drop-shadow",
      "colors",
      "--tw-drop-shadow-color",
      "var(--tw-drop-shadow-alpha)",
      ["--tw-drop-shadow: var(--tw-drop-shadow-size)"],
    ],
  ];

  const fallback = (property, oklch, alpha) =>
    alpha.startsWith("var(")
      ? `${property}: ${oklch}`
      : `${property}: color-mix(in srgb, ${oklch} ${alpha}, transparent)`;
  const supported = (property, palette, alpha) =>
    `${property}: color-mix(in oklab, var(--color-${palette}-500) ${alpha}, transparent)`;
  const supportedHook = (property, palette, alpha, alphaProperty) =>
    `${property}: color-mix(in oklab, color-mix(in oklab, var(--color-${palette}-500) ${alpha}, transparent) ${alphaProperty}, transparent)`;

  for (const [palette, shades] of Object.entries(neutralAdjacentPalettes)) {
    const oklch = shades[500];
    for (const [opacityToken, alpha] of officialOpacityModifiers) {
      for (const [prefix, area, property] of directFamilies) {
        const className = `${prefix}-${palette}-500/${opacityToken}`;
        const entry = classes.get(className);
        const fragments = [fallback(property, oklch, alpha), COLOR_MIX_SUPPORTS, supported(property, palette, alpha)];

        assert.ok(entry, `fixture matrix should include ${className}`);
        assert.equal(entry.area, area);
        assert.equal(entry.comparisonMode, "exact-fragment-match");
        for (const fragment of fragments) {
          assert.ok(entry.tailwindRequiredFragments?.includes(fragment), `${className} should require Tailwind fragment ${fragment}`);
          assert.ok(entry.dxStyleRequiredFragments?.includes(fragment), `${className} should require dx-style fragment ${fragment}`);
        }
      }

      for (const [prefix, area, property, extraFragments] of nestedFamilies) {
        const className = `${prefix}-${palette}-500/${opacityToken}`;
        const entry = classes.get(className);
        const fragments = [
          ...extraFragments,
          fallback(property, oklch, alpha),
          COLOR_MIX_SUPPORTS,
          supported(property, palette, alpha),
        ];

        assert.ok(entry, `fixture matrix should include ${className}`);
        assert.equal(entry.area, area);
        assert.equal(entry.comparisonMode, "exact-fragment-match");
        for (const fragment of fragments) {
          assert.ok(entry.tailwindRequiredFragments?.includes(fragment), `${className} should require Tailwind fragment ${fragment}`);
          assert.ok(entry.dxStyleRequiredFragments?.includes(fragment), `${className} should require dx-style fragment ${fragment}`);
        }
      }

      for (const [prefix, area, property, alphaProperty, extraFragments] of alphaHookFamilies) {
        const className = `${prefix}-${palette}-500/${opacityToken}`;
        const entry = classes.get(className);
        const fragments = [
          fallback(property, oklch, alpha),
          COLOR_MIX_SUPPORTS,
          supportedHook(property, palette, alpha, alphaProperty),
          ...extraFragments,
        ];

        assert.ok(entry, `fixture matrix should include ${className}`);
        assert.equal(entry.area, area);
        assert.equal(entry.comparisonMode, "exact-fragment-match");
        for (const fragment of fragments) {
          assert.ok(entry.tailwindRequiredFragments?.includes(fragment), `${className} should require Tailwind fragment ${fragment}`);
          assert.ok(entry.dxStyleRequiredFragments?.includes(fragment), `${className} should require dx-style fragment ${fragment}`);
        }
      }
    }
  }
});

test("official live matrix captures v4.3 palette opacity fallbacks and oklab supports", () => {
  const classes = new Map(fixtureMatrix.classes.map((entry) => [entry.className, entry]));

  for (const [className, fragments] of [
    [
      "bg-mauve-500/50",
      [
        "background-color: color-mix(in srgb, oklch(54.2% 0.034 322.5) 50%, transparent)",
        COLOR_MIX_SUPPORTS,
        "background-color: color-mix(in oklab, var(--color-mauve-500) 50%, transparent)",
      ],
    ],
    [
      "text-taupe-500/25",
      [
        "color: color-mix(in srgb, oklch(54.7% 0.021 43.1) 25%, transparent)",
        COLOR_MIX_SUPPORTS,
        "color: color-mix(in oklab, var(--color-taupe-500) 25%, transparent)",
      ],
    ],
    [
      "border-y-olive-500/50",
      [
        "border-block-color: color-mix(in srgb, oklch(58% 0.031 107.3) 50%, transparent)",
        COLOR_MIX_SUPPORTS,
        "border-block-color: color-mix(in oklab, var(--color-olive-500) 50%, transparent)",
      ],
    ],
    [
      "ring-taupe-400/50",
      [
        "--tw-ring-color: color-mix(in srgb, oklch(71.4% 0.014 41.2) 50%, transparent)",
        COLOR_MIX_SUPPORTS,
        "--tw-ring-color: color-mix(in oklab, var(--color-taupe-400) 50%, transparent)",
      ],
    ],
    [
      "placeholder-olive-500/50",
      [
        "color: color-mix(in srgb, oklch(58% 0.031 107.3) 50%, transparent)",
        COLOR_MIX_SUPPORTS,
        "color: color-mix(in oklab, var(--color-olive-500) 50%, transparent)",
      ],
    ],
    [
      "divide-olive-500/50",
      [
        "border-color: color-mix(in srgb, oklch(58% 0.031 107.3) 50%, transparent)",
        COLOR_MIX_SUPPORTS,
        "border-color: color-mix(in oklab, var(--color-olive-500) 50%, transparent)",
      ],
    ],
    [
      "via-taupe-500/40",
      [
        "--tw-gradient-via: color-mix(in srgb, oklch(54.7% 0.021 43.1) 40%, transparent)",
        COLOR_MIX_SUPPORTS,
        "--tw-gradient-via: color-mix(in oklab, var(--color-taupe-500) 40%, transparent)",
      ],
    ],
    [
      "scrollbar-thumb-mauve-500/60",
      [
        "--tw-scrollbar-thumb: color-mix(in srgb, oklch(54.2% 0.034 322.5) 60%, transparent)",
        COLOR_MIX_SUPPORTS,
        "--tw-scrollbar-thumb: color-mix(in oklab, var(--color-mauve-500) 60%, transparent)",
        "scrollbar-color: var(--tw-scrollbar-thumb) var(--tw-scrollbar-track)",
      ],
    ],
  ]) {
    const entry = classes.get(className);
    assert.ok(entry, `fixture matrix should include ${className}`);
    assert.equal(entry.comparisonMode, "exact-fragment-match");
    for (const fragment of fragments) {
      assert.ok(
        entry.tailwindRequiredFragments?.includes(fragment),
        `${className} should require Tailwind fragment ${fragment}`,
      );
      assert.ok(
        entry.dxStyleRequiredFragments?.includes(fragment),
        `${className} should require dx-style fragment ${fragment}`,
      );
    }
  }
});

test("official live matrix covers physical directional border colors for v4.3 palettes", () => {
  const classes = new Map(fixtureMatrix.classes.map((entry) => [entry.className, entry]));

  for (const [className, dxStyleFragments] of [
    [
      "border-x-mauve-500",
      ["border-inline-color: var(--color-mauve-500)"],
    ],
    [
      "border-y-olive-500/50",
      ["border-block-color: color-mix(in oklab, var(--color-olive-500) 50%, transparent)"],
    ],
    ["border-t-mist-300", ["border-top-color: var(--color-mist-300)"]],
    ["border-r-taupe-700", ["border-right-color: var(--color-taupe-700)"]],
    ["border-b-mauve-950", ["border-bottom-color: var(--color-mauve-950)"]],
    [
      "border-l-olive-600/[71.37%]",
      ["border-left-color: color-mix(in oklab, var(--color-olive-600) 71.37%, transparent)"],
    ],
    ["border-t-[#243c5a]", ["border-top-color: #243c5a"]],
    [
      "border-r-[#243c5a]/50",
      ["border-right-color: color-mix(in oklab, #243c5a 50%, transparent)"],
    ],
    ["border-x-(color:--dx-border)", ["border-inline-color: var(--dx-border)"]],
    [
      "border-y-(color:--dx-border-alpha)/(--dx-alpha)",
      [
        "border-block-color: color-mix(in oklab, var(--dx-border-alpha) var(--dx-alpha), transparent)",
      ],
    ],
  ]) {
    const entry = classes.get(className);
    assert.ok(entry, `fixture matrix should include ${className}`);
    assert.equal(entry.area, "colors");
    assert.equal(entry.comparisonMode, "exact-fragment-match");
    for (const fragment of dxStyleFragments) {
      assert.ok(
        entry.dxStyleRequiredFragments?.includes(fragment),
        `${className} should require dx-style fragment ${fragment}`,
      );
    }
  }
});

test("official live matrix covers placeholder color utilities for v4.3 palettes and arbitrary colors", () => {
  const classes = new Map(fixtureMatrix.classes.map((entry) => [entry.className, entry]));

  for (const [className, dxStyleFragments] of [
    ["placeholder-mauve-500", ["::placeholder", "color: var(--color-mauve-500)"]],
    [
      "placeholder-olive-500/50",
      [
        "::placeholder",
        "color: color-mix(in oklab, var(--color-olive-500) 50%, transparent)",
      ],
    ],
    ["placeholder-[#243c5a]", ["::placeholder", "color: #243c5a"]],
    [
      "placeholder-(color:--dx-placeholder)/(--dx-alpha)",
      [
        "::placeholder",
        "color: color-mix(in oklab, var(--dx-placeholder) var(--dx-alpha), transparent)",
      ],
    ],
  ]) {
    const entry = classes.get(className);
    assert.ok(entry, `fixture matrix should include ${className}`);
    assert.equal(entry.area, "colors");
    assert.equal(entry.comparisonMode, "exact-fragment-match");
    for (const fragment of dxStyleFragments) {
      assert.ok(
        entry.dxStyleRequiredFragments?.includes(fragment),
        `${className} should require dx-style fragment ${fragment}`,
      );
      assert.ok(
        entry.tailwindRequiredFragments?.includes(fragment),
        `${className} should require Tailwind fragment ${fragment}`,
      );
    }
  }
});

test("official live matrix covers divide color utilities for v4.3 palettes and arbitrary colors", () => {
  const classes = new Map(fixtureMatrix.classes.map((entry) => [entry.className, entry]));

  for (const [className, dxStyleFragments] of [
    ["divide-mauve-500", ["border-color: var(--color-mauve-500)"]],
    [
      "divide-olive-500/50",
      ["border-color: color-mix(in oklab, var(--color-olive-500) 50%, transparent)"],
    ],
    ["divide-[#243c5a]", ["border-color: #243c5a"]],
    [
      "divide-(color:--dx-divider)/(--dx-alpha)",
      ["border-color: color-mix(in oklab, var(--dx-divider) var(--dx-alpha), transparent)"],
    ],
  ]) {
    const entry = classes.get(className);
    assert.ok(entry, `fixture matrix should include ${className}`);
    assert.equal(entry.area, "colors");
    assert.equal(entry.comparisonMode, "exact-fragment-match");
    for (const fragment of dxStyleFragments) {
      assert.ok(
        entry.dxStyleRequiredFragments?.includes(fragment),
        `${className} should require dx-style fragment ${fragment}`,
      );
      assert.ok(
        entry.tailwindRequiredFragments?.includes(fragment),
        `${className} should require Tailwind fragment ${fragment}`,
      );
    }
  }
});

test("official live matrix classifies neutral-adjacent shadow and inset ring color effects", () => {
  const classes = new Map(fixtureMatrix.classes.map((entry) => [entry.className, entry]));

  for (const [className, dxStyleFragments, tailwindFragments = dxStyleFragments] of [
    [
      "shadow-mauve-500",
      [
        "--tw-shadow-color: oklch(54.2% 0.034 322.5)",
        "--tw-shadow-color: color-mix(in oklab, var(--color-mauve-500) var(--tw-shadow-alpha), transparent)",
      ],
      [
        "--tw-shadow-color: oklch(54.2%",
        "--tw-shadow-color: color-mix(in oklab, var(--color-mauve-500) var(--tw-shadow-alpha), transparent)",
      ],
    ],
    [
      "shadow-mauve-500/50",
      [
        "--tw-shadow-color: color-mix(in srgb, oklch(54.2% 0.034 322.5) 50%, transparent)",
        "--tw-shadow-color: color-mix(in oklab, color-mix(in oklab, var(--color-mauve-500) 50%, transparent) var(--tw-shadow-alpha), transparent)",
      ],
    ],
    [
      "drop-shadow-mauve-500/50",
      [
        "--tw-drop-shadow-color: color-mix(in srgb, oklch(54.2% 0.034 322.5) 50%, transparent)",
        "--tw-drop-shadow-color: color-mix(in oklab, color-mix(in oklab, var(--color-mauve-500) 50%, transparent) var(--tw-drop-shadow-alpha), transparent)",
      ],
    ],
    [
      "inset-shadow-olive-500",
      [
        "--tw-inset-shadow-color: oklch(58% 0.031 107.3)",
        "--tw-inset-shadow-color: color-mix(in oklab, var(--color-olive-500) var(--tw-inset-shadow-alpha), transparent)",
      ],
      [
        "--tw-inset-shadow-color: oklch(58%",
        "--tw-inset-shadow-color: color-mix(in oklab, var(--color-olive-500) var(--tw-inset-shadow-alpha), transparent)",
      ],
    ],
    [
      "inset-shadow-olive-500/50",
      [
        "--tw-inset-shadow-color: color-mix(in srgb, oklch(58% 0.031 107.3) 50%, transparent)",
        "--tw-inset-shadow-color: color-mix(in oklab, color-mix(in oklab, var(--color-olive-500) 50%, transparent) var(--tw-inset-shadow-alpha), transparent)",
      ],
    ],
    [
      "inset-ring-mist-500/50",
      [
        "--tw-inset-ring-color: color-mix(in srgb, oklch(56% 0.021 213.5) 50%, transparent)",
        "--tw-inset-ring-color: color-mix(in oklab, var(--color-mist-500) 50%, transparent)",
      ],
    ],
    [
      "ring-offset-taupe-500/40",
      [
        "--tw-ring-offset-color: color-mix(in srgb, oklch(54.7% 0.021 43.1) 40%, transparent)",
        "--tw-ring-offset-color: color-mix(in oklab, var(--color-taupe-500) 40%, transparent)",
      ],
    ],
  ]) {
    const entry = classes.get(className);
    assert.ok(entry, `fixture matrix should include ${className}`);
    assert.equal(entry.area, "colors");
    assert.equal(entry.comparisonMode, "exact-fragment-match");
    for (const fragment of dxStyleFragments) {
      assert.ok(
        entry.dxStyleRequiredFragments?.includes(fragment),
        `${className} should require dx-style fragment ${fragment}`,
      );
    }
    for (const fragment of tailwindFragments) {
      assert.ok(
        entry.tailwindRequiredFragments?.includes(fragment),
        `${className} should require Tailwind fragment ${fragment}`,
      );
    }
  }
});
