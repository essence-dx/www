//! Source-owned Tailwind v4.3 compatibility matrix for dx-style.

/// Tailwind baseline verified for this feature matrix.
pub const TAILWIND_V43_FEATURE_MATRIX_BASELINE: &str = "tailwindcss-4.3.0";

/// Scope statement for public consumers.
pub const TAILWIND_V43_FEATURE_MATRIX_SCOPE: &str =
    "dx-style vs Tailwind v4.3 feature matrix; not full Tailwind replacement";

/// Current support state for one Tailwind v4.3 feature area.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TailwindV43FeatureStatus {
    Supported,
    Partial,
    UnsupportedByDesign,
    Missing,
}

/// One row in the explicit dx-style vs Tailwind v4.3 matrix.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
pub struct TailwindV43FeatureMatrixEntry {
    pub area: &'static str,
    pub status: TailwindV43FeatureStatus,
    pub dx_style_truth: &'static str,
    pub gap_canaries: &'static [&'static str],
    pub next_test: &'static str,
}

/// Return the source-owned Tailwind v4.3 matrix.
pub fn tailwind_v43_feature_matrix() -> &'static [TailwindV43FeatureMatrixEntry] {
    TAILWIND_V43_FEATURE_MATRIX
}

const TAILWIND_V43_FEATURE_MATRIX: &[TailwindV43FeatureMatrixEntry] = &[
    TailwindV43FeatureMatrixEntry {
        area: "css-first-theme-and-imports",
        status: TailwindV43FeatureStatus::Supported,
        dx_style_truth: "CSS-first @theme parsing, local import flattening, token receipts, and normal generated CSS are implemented for DX starters.",
        gap_canaries: &[],
        next_test: "Keep theme/import fixtures tied to dx style build and dx style check receipts.",
    },
    TailwindV43FeatureMatrixEntry {
        area: "class-scanning-and-diagnostics",
        status: TailwindV43FeatureStatus::Partial,
        dx_style_truth: "Static attribute/helper class scanning, Tailwind-style plain-text static quoted object-map, array, and helper-call scanning in the style parser, arbitrary static string candidates, generated-output canaries, explicit dynamic-fragment source-boundary diagnostics, and dx-check unsupported-class diagnostics are implemented for concrete class tokens. Full Tailwind source graph traversal, prose-token filtering parity, and every source-language shape are not proven.",
        gap_canaries: &[
            "sourceScannerCanaries",
            "tsx-static-object-map",
            "tsx-static-array-and-helper-literals",
            "plain_text_extraction_reads_static_arrays_object_maps_and_helpers",
            "plain_text_extraction_rejects_dynamic_fragments_and_prose",
            "arbitrary-value-static-string",
            "template-interpolation-rejection",
            "dynamic-fragment-diagnostic",
            "source-scan-diagnostic-receipt",
            "plain_text_diagnostics_report_dynamic_object_key_unsafe_prose_and_duplicates",
            "reports_plain_text_source_scan_diagnostics_with_locations",
            "dynamic class fragment was scanned but cannot be generated statically",
            "unsupported_scan_reports_dynamic_class_fragments_as_source_boundaries",
            "full Tailwind source graph traversal",
        ],
        next_test: "Keep adding source-scanner canaries from the official docs and candidate inventory before claiming full Tailwind plain-text scanner parity.",
    },
    TailwindV43FeatureMatrixEntry {
        area: "utility-grammar",
        status: TailwindV43FeatureStatus::Partial,
        dx_style_truth: "Many common utilities generate CSS, a docs-table ledger exists, typography now covers text-size line-height modifiers plus CSS-first font/text companion tokens, effects now cover text-shadow size/arbitrary opacity modifiers and typed arbitrary [shadow:...] values, and the official fixture matrix carries representative canaries for every public utility docs area, but complete Tailwind v4.3 utility value/modifier grammar is not implemented or proven.",
        gap_canaries: &[
            "Tailwind docs-table ledger",
            "official fixture matrix utilityDocsCoverage",
            "text-sm/6 typography modifier fixture",
            "font/text companion theme-token fixture",
            "text-shadow-lg/20 opacity modifier fixture",
            "text-shadow arbitrary opacity modifier fixture",
            "full docs-table walk",
            "uncovered arbitrary utility grammar",
        ],
        next_test: "Promote the next unsupported utility family from the official candidate inventory through a focused generated-output or live-comparison canary before implementation.",
    },
    TailwindV43FeatureMatrixEntry {
        area: "color-palette-and-modern-color-spaces",
        status: TailwindV43FeatureStatus::Partial,
        dx_style_truth: "Common palettes, Tailwind v4.3 neutral-adjacent mauve/olive/mist/taupe OKLCH theme-token palettes, generated-output Tailwind v4.3 neutral-adjacent palette canaries, token colors, alpha suffixes, and color-mix opacity work; full OKLCH/P3 color-space parity is not proven.",
        gap_canaries: &[
            "Tailwind v4.3 neutral-adjacent palette fixture",
            "mauve/olive/mist/taupe OKLCH token-backed fixture",
            "Tailwind v4.3 neutral-adjacent palette canaries",
            "display-p3 output fixture",
            "full OKLCH palette fixture",
            "P3 color output fixture",
        ],
        next_test: "Add OKLCH theme-variable and P3 equal-output fixtures before claiming modern color-space parity.",
    },
    TailwindV43FeatureMatrixEntry {
        area: "variants-and-selector-grammar",
        status: TailwindV43FeatureStatus::Partial,
        dx_style_truth: "Responsive, media/container, fixture-backed default container-query ladder tokens from @3xs through @7xl plus @max-* and named forms, Tailwind v4.3 container range syntax, reversed min/max container-query range variants that preserve Tailwind's written nesting order, arbitrary shorthand container queries such as @[475px]:* with named and stacked max-width forms, named arbitrary min/max container query variants, Tailwind v4.3 inline-size, normal, and size container marker utilities with named forms, pointer/any-pointer, contrast, forced/inverted color, noscript, hover-capability, direction selectors, open/inert wrappers, user-valid/user-invalid/details-content states, backdrop pseudo-element variants, data/aria, group/peer basics, group/peer pseudo-class state variants, safe selector arbitrary variants, stacked arbitrary selector variants, covered arbitrary selector-list branch composition, Tailwind v4 arbitrary group/peer selector wrappers including guarded & placement, selector-list branches, selector-function commas, and named forms, safe unknown/custom arbitrary at-rule variants, and Tailwind-style not-[...] arbitrary media/supports/container at-rule variants exist; Tailwind build-time directive at-rules still fail closed, unsupported negated custom at-rules fail closed, and exhaustive arbitrary selector escaping plus full grammar are not proven.",
        gap_canaries: &[
            "pointer/any-pointer media fixture",
            "hover-capability media fixture",
            "full default container query ladder fixture",
            "container query range ordering fixture",
            "container marker utility fixture",
            "direction/open/inert selector fixture",
            "user-valid/user-invalid/details-content selector fixture",
            "user-valid/user-invalid/details-content/backdrop selector fixture",
            "group/peer pseudo-class variant fixture",
            "safe unknown arbitrary at-rule fixture",
            "negated arbitrary media/supports/container at-rule fixture",
            "stacked arbitrary selector composition fixture",
            "selector-list arbitrary variant fixture",
            "benchmarks/dx-style-v43-variant-selector-parity.test.ts",
            "Tailwind directive arbitrary variant fail-closed fixture",
            "stacked arbitrary group/peer selector fixture",
            "escaped arbitrary selector fixture",
            "exact selector ordering fixture",
            "full arbitrary variant grammar fixture",
        ],
        next_test: "Promote escaped arbitrary selector edge cases, selector-list at-rule variants, and broader arbitrary group/peer selector-list fixtures before broad arbitrary variant handling or full variant parity claims.",
    },
    TailwindV43FeatureMatrixEntry {
        area: "browser-fallback-parity",
        status: TailwindV43FeatureStatus::Partial,
        dx_style_truth: "Targeted prefix and fallback canaries exist; universal Autoprefixer/browser fallback parity is not proven.",
        gap_canaries: &[
            "Autoprefixer equal-output fixture",
            "full Autoprefixer property matrix",
            "browser target fixture sweep",
        ],
        next_test: "Expand checked-in browser fallback fixtures before claiming broader parity.",
    },
    TailwindV43FeatureMatrixEntry {
        area: "source-inline-and-not-directives",
        status: TailwindV43FeatureStatus::Partial,
        dx_style_truth: "Static @source paths, @source none, Tailwind import source(none) conversion into DX-owned @source none, @source inline(...) expansion, @source not inline(...) exclusion, and a DX-owned @source scan plan are implemented; file IO, glob traversal, and full Tailwind source graph inclusion/exclusion semantics are not proven.",
        gap_canaries: &[
            "@source '../packages/ui'",
            "@source inline('{hover:,focus:,}bg-brand')",
            "@source inline('{hover:,focus:,}bg-brand p-{2..4..2} underline')",
            r#"@import "tailwindcss" source(none)"#,
            "@source none",
            "source-not-inline-exclusion",
            "@source not inline('{hover:,focus:,}bg-red-{50,{100..900..100},950}')",
            "@source not '../legacy/**'",
        ],
        next_test: "Add source graph file IO and glob fixtures before claiming full @source parity.",
    },
    TailwindV43FeatureMatrixEntry {
        area: "custom-utility-directive",
        status: TailwindV43FeatureStatus::Partial,
        dx_style_truth: "Safe @utility declarations, one trailing * functional segment, one-level top-level nested selectors beginning with &, theme-key/bare/arbitrary/literal --value(...), --modifier(...), --default(...), and @utility-local --alpha()/--spacing() helpers are supported; full Tailwind @utility grammar, cascade, selector-list nesting, and layer behavior are not proven.",
        gap_canaries: &[
            "@utility tab-* { tab-size: --value(integer); }",
            "@utility tab-* { tab-size: --value(integer, --default(4)); }",
            "@utility text-* { line-height: --modifier(number, --default(1)); }",
            "@utility glow-* { color: --alpha(var(--color-brand) / --modifier(percentage)); }",
            "@utility scrollbar-hidden { scrollbar-width: none; &::-webkit-scrollbar { display: none; } }",
            "@layer utilities { @utility layered { color: red; } }",
        ],
        next_test: "Add selector-list nesting, cascade ordering, and standalone CSS function fixtures before broadening @utility support.",
    },
    TailwindV43FeatureMatrixEntry {
        area: "custom-variant-directive",
        status: TailwindV43FeatureStatus::Partial,
        dx_style_truth: "Shorthand @custom-variant selector wrappers, block-form @slot selector wrappers, and nested @media/@supports custom variants are supported; full Tailwind custom-variant ordering, escaping, selector-list nesting, and multiple-slot grammar are not proven.",
        gap_canaries: &[
            "@custom-variant theme-midnight (&:where([data-theme=\"midnight\"] *))",
            "@custom-variant theme-midnight { &:where([data-theme=\"midnight\"] *) { @slot; } }",
            "@custom-variant any-hover { @media (any-hover: hover) { &:hover { @slot; } } }",
            "multiple @slot selector-list expansion",
            "stacked custom variant ordering fixture",
        ],
        next_test: "Add stacked custom-variant ordering and escaping fixtures before claiming broader parity.",
    },
    TailwindV43FeatureMatrixEntry {
        area: "css-variant-directive",
        status: TailwindV43FeatureStatus::Partial,
        dx_style_truth: "dx-www authored CSS supports common @variant pseudo selectors, dark wrapping, Tailwind v4.3 stacked variants, compound variants, group-hover wrappers, stacked responsive custom variants, safe arbitrary selector variants, safe @layer authored CSS wrapping, and block-form custom variants with nested media; Tailwind's full CSS @variant grammar, cascade ordering, and arbitrary selector parity are not proven.",
        gap_canaries: &[
            "@variant hover",
            "@variant hover:focus",
            "@variant hover, focus",
            "@variant group-hover",
            "@variant md:theme-midnight",
            "@variant any-hover",
            "@variant safe arbitrary selector",
            "@variant inside safe @layer authored CSS",
            "full CSS @variant arbitrary selector grammar",
        ],
        next_test: "Add authored-CSS arbitrary selector and cascade-order fixtures before claiming full CSS @variant parity.",
    },
    TailwindV43FeatureMatrixEntry {
        area: "css-directive-parity-ledger",
        status: TailwindV43FeatureStatus::Partial,
        dx_style_truth: "A source-owned Tailwind v4.3 CSS directive ledger exists for @theme, @source, @utility, @custom-variant, CSS @variant including safe arbitrary selector variants and safe @layer authored CSS wrapping, plain and safe variant-bearing @apply expansion, safe authored --alpha()/--spacing() declarations inside @apply rules, @apply inside safe one-level nested & authored selectors, @apply inside safe @layer authored CSS, local CSS @reference theme-token flattening in the public build path, standalone authored --alpha()/--spacing() transforms, @plugin, and @config, and the official fixture matrix carries CSS directive canaries, but it is not full CSS directive parity.",
        gap_canaries: &[
            "cssDirectiveCanaries",
            "@variant safe arbitrary selector",
            "@variant inside safe @layer authored CSS",
            "@apply with safe authored --alpha() and --spacing() declarations",
            "@apply inside safe nested & authored selectors",
            "@apply inside safe @layer authored CSS",
            "@apply arbitrary at-rule variants/layers/cascade",
            "@reference",
            "full --alpha() color namespace/fallback parity",
            "full --spacing() nested calc/value grammar parity",
            "selector-list/layered @utility",
            "full @custom-variant ordering",
            "full CSS @variant grammar",
        ],
        next_test: "Promote deeper @reference semantics or a variant/custom-variant grammar slice through the directive ledger and a generated-output/authored-CSS canary before implementation.",
    },
    TailwindV43FeatureMatrixEntry {
        area: "js-config-and-plugin-ecosystem",
        status: TailwindV43FeatureStatus::UnsupportedByDesign,
        dx_style_truth: "dx-style does not execute Tailwind JS config or plugin functions and should not depend on Tailwind for normal generation.",
        gap_canaries: &[
            "tailwind.config.js plugin()",
            "tailwind package dependency leakage",
        ],
        next_test: "Keep leakage checks guarding official starters against Tailwind config and package dependencies.",
    },
    TailwindV43FeatureMatrixEntry {
        area: "official-plugin-ecosystem",
        status: TailwindV43FeatureStatus::UnsupportedByDesign,
        dx_style_truth: "Official Tailwind plugin packages are external JavaScript/plugin code, not dx-style CSS parity. dx-style should keep Tailwind plugin execution out of normal generation and model any wanted prose/forms/aspect behavior as DX-owned CSS features.",
        gap_canaries: &[
            "external Tailwind plugin code is out of scope",
            "@plugin/@config unsupported diagnostics",
            "tailwind package dependency leakage",
            "DX-owned prose/forms/aspect behavior must be source-owned CSS",
        ],
        next_test: "Add DX-owned utility/prose/forms/aspect guards only when those CSS features are intentionally implemented; do not chase official plugin package execution.",
    },
    TailwindV43FeatureMatrixEntry {
        area: "advanced-css-theme-token-extensions",
        status: TailwindV43FeatureStatus::Partial,
        dx_style_truth: "CSS @theme tokens and selected generated utility families have source-owned guards for custom animation aliases, transition tokens, container-query tokens, container-query namespace resets such as --container-*: initial, and grid edge grammar. Tailwind JS config-extension and plugin-callback execution are unsupported by design because they are external JavaScript/config execution, not dx-style CSS parity.",
        gap_canaries: &[
            "css @theme custom animation alias fixture",
            "css @theme transition token fixture",
            "css @theme container-query token fixture",
            "css @theme container-query namespace reset fixture",
            "css grid edge grammar fixture",
        ],
        next_test: "Keep expanding CSS-owned animation, transition, container-query, and grid edge fixtures before broadening generated utility support; do not chase Tailwind JS config/plugin execution.",
    },
    TailwindV43FeatureMatrixEntry {
        area: "official-fixture-matrix",
        status: TailwindV43FeatureStatus::Supported,
        dx_style_truth: "A governed Tailwind v4.3 fixture matrix is ingested from related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json and backed by related-crates/style/fixtures/tailwind-v43-official-candidate-inventory.json, generated from official Tailwind v4.3 source tests, inline snapshot fixture fingerprints, and npm package metadata. This proves official fixture ingestion, not complete Tailwind parity.",
        gap_canaries: &[],
        next_test: "Keep growing comparison coverage from the official candidate inventory before claiming a full docs-table walk.",
    },
    TailwindV43FeatureMatrixEntry {
        area: "governed-live-tailwind-output-comparison",
        status: TailwindV43FeatureStatus::Supported,
        dx_style_truth: "tools/dx-style/live-tailwind-v43-compare.cjs installs tailwindcss@4.3.0 and @tailwindcss/cli@4.3.0 with --no-save inside a temporary directory, compares real Tailwind CSS against real dx-style fixture output, and emits a governed live comparison receipt. If DX_STYLE_FIXTURE_CSS_BIN is provided, the runner rejects stale fixture binaries against related-crates/style/src and the active matrix unless DX_STYLE_ALLOW_STALE_FIXTURE_BIN=1 is set for local debugging, and the receipt exposes dxStyleCssSource plus failed-class buckets. The official fixture matrix points at benchmarks/dx-style-live-tailwind-v43-comparison.test.ts as the normal focused live guard and benchmarks/dx-style-live-comparison-receipt-accuracy.test.cjs as the receipt-accuracy guard. This is test-only evidence, not a runtime dependency or full parity claim.",
        gap_canaries: &[],
        next_test: "Keep expanding live comparison entries as the official-source-backed fixture matrix grows, without adding Tailwind to dx-style manifests.",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_covers_all_status_buckets() {
        for status in [
            TailwindV43FeatureStatus::Supported,
            TailwindV43FeatureStatus::Partial,
            TailwindV43FeatureStatus::UnsupportedByDesign,
        ] {
            assert!(
                tailwind_v43_feature_matrix()
                    .iter()
                    .any(|entry| entry.status == status)
            );
        }
    }
}
