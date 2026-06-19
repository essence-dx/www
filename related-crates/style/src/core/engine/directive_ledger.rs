//! Tailwind v4.3 CSS directive ledger for dx-style.
//!
//! This ledger is a proof inventory for Tailwind's CSS-first directive surface.
//! It documents dx-style's supported subset and the missing directive semantics
//! without claiming full Tailwind CSS directive parity.

#![allow(dead_code)]

use super::feature_matrix::TailwindV43FeatureStatus;

/// Stable schema for CSS directive ledger consumers.
pub const TAILWIND_V43_CSS_DIRECTIVE_LEDGER_SCHEMA: &str =
    "dx.style.tailwind-v43-css-directive-ledger";

/// Tailwind baseline verified for this directive ledger.
pub const TAILWIND_V43_CSS_DIRECTIVE_LEDGER_BASELINE: &str = "tailwindcss-4.3.0";

/// Scope statement for public consumers.
pub const TAILWIND_V43_CSS_DIRECTIVE_LEDGER_SCOPE: &str =
    "Tailwind v4.3 CSS directive ledger; not full CSS directive parity";

/// One Tailwind CSS directive or CSS function area in the dx-style ledger.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
pub struct TailwindV43CssDirectiveLedgerEntry {
    /// Stable area id used by tests and docs.
    pub directive_area: &'static str,
    /// Public directive/function syntax being tracked.
    pub directive_syntax: &'static str,
    /// Current dx-style support state for this directive surface.
    pub status: TailwindV43FeatureStatus,
    /// Representative supported dx-style canaries.
    pub representative_supported_canaries: &'static [&'static str],
    /// Directive/function proof still missing for this area.
    pub unproven_or_missing_canaries: &'static [&'static str],
    /// Reason for unsupported-by-design rows.
    pub unsupported_by_design_reason: Option<&'static str>,
    /// Whether this row proves full Tailwind behavior for the directive surface.
    pub full_tailwind_parity_proven: bool,
}

/// Return the source-owned Tailwind v4.3 CSS directive ledger.
pub fn tailwind_v43_css_directive_ledger() -> &'static [TailwindV43CssDirectiveLedgerEntry] {
    TAILWIND_V43_CSS_DIRECTIVE_LEDGER
}

const NO_TAILWIND_RUNTIME: &str =
    "No Tailwind runtime or JS plugin/config execution is allowed in normal dx-style generation.";

const TAILWIND_V43_CSS_DIRECTIVE_LEDGER: &[TailwindV43CssDirectiveLedgerEntry] = &[
    TailwindV43CssDirectiveLedgerEntry {
        directive_area: "theme",
        directive_syntax: "@theme",
        status: TailwindV43FeatureStatus::Supported,
        representative_supported_canaries: &[
            "@theme { --color-brand: oklch(0.7 0.18 240); }",
            "bg-brand",
        ],
        unproven_or_missing_canaries: &[
            "full Tailwind theme variable namespace sweep",
            "complete Tailwind default OKLCH/P3 palette fixture",
        ],
        unsupported_by_design_reason: None,
        full_tailwind_parity_proven: false,
    },
    TailwindV43CssDirectiveLedgerEntry {
        directive_area: "tailwind-import",
        directive_syntax: "@import \"tailwindcss\"",
        status: TailwindV43FeatureStatus::Supported,
        representative_supported_canaries: &[
            "@import \"tailwindcss\" accepted as migration input",
            "@import \"tailwindcss\" source(none) converted to @source none",
            "dx-style generated CSS replacement path",
        ],
        unproven_or_missing_canaries: &[
            "arbitrary Tailwind package import graph parity",
            "third-party PostCSS import plugin parity",
        ],
        unsupported_by_design_reason: None,
        full_tailwind_parity_proven: false,
    },
    TailwindV43CssDirectiveLedgerEntry {
        directive_area: "source-static",
        directive_syntax: "@source",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &[
            "@source \"../app\";",
            "@source \"../packages/ui\";",
            "@source not \"../legacy/**\";",
            "@source none;",
            "DX-owned scan plan captures include paths, exclude paths, @source none, and inline precedence",
        ],
        unproven_or_missing_canaries: &[
            "source graph file IO and glob expansion",
            "Tailwind glob semantics",
            "automatic project dependency scanning",
        ],
        unsupported_by_design_reason: None,
        full_tailwind_parity_proven: false,
    },
    TailwindV43CssDirectiveLedgerEntry {
        directive_area: "source-inline",
        directive_syntax: "@source inline(...)",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &[
            "@source inline(\"{hover:,focus:,}bg-brand\")",
            "@source inline(\"bg-red-{50,{100..900..100},950}\")",
            "@source inline(\"{hover:,focus:,}bg-brand p-{2..4..2} underline\")",
        ],
        unproven_or_missing_canaries: &[
            "full Tailwind brace expansion parity",
            "full inline source ordering semantics",
            "large inline fixture matrix",
        ],
        unsupported_by_design_reason: None,
        full_tailwind_parity_proven: false,
    },
    TailwindV43CssDirectiveLedgerEntry {
        directive_area: "source-exclusion",
        directive_syntax: "@source not ...",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &[
            "@source not inline(\"bg-red-500\")",
            "@source not inline(\"focus:bg-brand p-2\")",
            "@source not \"../legacy/**\"",
        ],
        unproven_or_missing_canaries: &[
            "nested graph exclusion precedence",
            "glob negation parity",
            "source include/exclude ordering fixture",
        ],
        unsupported_by_design_reason: None,
        full_tailwind_parity_proven: false,
    },
    TailwindV43CssDirectiveLedgerEntry {
        directive_area: "custom-utility",
        directive_syntax: "@utility",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &[
            "@utility content-auto { content-visibility: auto; }",
            "@utility tab-* { tab-size: --value(integer); }",
            "@utility tab-* { tab-size: --value(integer, --default(4)); }",
            "@utility text-* { line-height: --modifier(number, --default(1)); }",
            "@utility glow-* { color: --alpha(var(--color-brand) / --modifier(percentage)); }",
            "@utility scrollbar-hidden { scrollbar-width: none; &::-webkit-scrollbar { display: none; } }",
        ],
        unproven_or_missing_canaries: &[
            "selector-list and deep nested @utility selectors",
            "layered @utility cascade semantics",
            "full Tailwind typed arbitrary value grammar",
            "complete declaration cascade/sort parity",
        ],
        unsupported_by_design_reason: None,
        full_tailwind_parity_proven: false,
    },
    TailwindV43CssDirectiveLedgerEntry {
        directive_area: "custom-variant",
        directive_syntax: "@custom-variant",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &[
            "@custom-variant theme-midnight (&:where([data-theme=\"midnight\"] *))",
            r#"@custom-variant theme-midnight { &:where([data-theme="midnight"] *) { @slot; } }"#,
            "@custom-variant any-hover { @media (any-hover: hover) { &:hover { @slot; } } }",
        ],
        unproven_or_missing_canaries: &[
            "stacked custom variant ordering",
            "custom variant escaping parity",
            "multiple @slot selector-list expansion",
            "multiple @slot blocks and nested selector-list ordering",
        ],
        unsupported_by_design_reason: None,
        full_tailwind_parity_proven: false,
    },
    TailwindV43CssDirectiveLedgerEntry {
        directive_area: "css-variant",
        directive_syntax: "CSS @variant",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &[
            "@variant hover",
            "@variant dark",
            "@variant hover:focus",
            "@variant hover, focus",
            "@variant group-hover",
            "@variant md:theme-midnight",
            "@variant any-hover",
            "@variant safe arbitrary selector",
            "@variant inside safe @layer authored CSS",
        ],
        unproven_or_missing_canaries: &[
            "full CSS @variant arbitrary selector grammar",
            "full CSS @variant nested ordering parity",
            "full CSS @variant cascade layer ordering",
        ],
        unsupported_by_design_reason: None,
        full_tailwind_parity_proven: false,
    },
    TailwindV43CssDirectiveLedgerEntry {
        directive_area: "apply",
        directive_syntax: "@apply",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &[
            "@apply px-4 bg-brand",
            "@apply p-[13px] -ms-2",
            "@apply hover:bg-brand",
            "@apply md:px-4",
            "@apply dark:hover:bg-brand",
            "@apply with safe authored declarations in the same rule",
            "@apply with safe authored --alpha() and --spacing() declarations",
            "@apply inside safe nested & authored selectors",
            "@apply inside safe @layer authored CSS",
        ],
        unproven_or_missing_canaries: &[
            "@apply [@unknown_rule]:p-4",
            "full Tailwind @apply cascade/order/important parity",
        ],
        unsupported_by_design_reason: None,
        full_tailwind_parity_proven: false,
    },
    TailwindV43CssDirectiveLedgerEntry {
        directive_area: "reference",
        directive_syntax: "@reference",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &[
            "@reference \"./tokens.css\" accepted as local reference input",
            "@reference \"tailwindcss\" consumed as DX-owned default-theme reference",
        ],
        unproven_or_missing_canaries: &[
            "@reference package/subpath import semantics",
            "cross-file @reference outside the public DX style build path",
            "full Tailwind @reference module/package semantics",
        ],
        unsupported_by_design_reason: None,
        full_tailwind_parity_proven: false,
    },
    TailwindV43CssDirectiveLedgerEntry {
        directive_area: "alpha-function",
        directive_syntax: "--alpha()",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &[
            "@utility glow-* { color: --alpha(var(--color-brand) / --modifier(percentage)); }",
            "color: --alpha(var(--color-brand) / 50%)",
        ],
        unproven_or_missing_canaries: &[
            "full Tailwind color namespace resolution inside --alpha()",
            "OKLCH/P3 alpha fallback parity",
            "unsafe standalone --alpha() diagnostics",
        ],
        unsupported_by_design_reason: None,
        full_tailwind_parity_proven: false,
    },
    TailwindV43CssDirectiveLedgerEntry {
        directive_area: "spacing-function",
        directive_syntax: "--spacing()",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &[
            "@utility glow-* { padding-inline: --spacing(--value(integer)); }",
            "margin: --spacing(4)",
            "width: calc(100% - --spacing(2))",
        ],
        unproven_or_missing_canaries: &[
            "full nested calc simplification parity",
            "full Tailwind arbitrary value grammar inside --spacing()",
            "unsafe standalone --spacing() diagnostics",
        ],
        unsupported_by_design_reason: None,
        full_tailwind_parity_proven: false,
    },
    TailwindV43CssDirectiveLedgerEntry {
        directive_area: "plugin",
        directive_syntax: "@plugin",
        status: TailwindV43FeatureStatus::UnsupportedByDesign,
        representative_supported_canaries: &[],
        unproven_or_missing_canaries: &[
            "@plugin \"@tailwindcss/typography\"",
            "plugin API addUtilities/addComponents",
        ],
        unsupported_by_design_reason: Some(NO_TAILWIND_RUNTIME),
        full_tailwind_parity_proven: false,
    },
    TailwindV43CssDirectiveLedgerEntry {
        directive_area: "config",
        directive_syntax: "@config",
        status: TailwindV43FeatureStatus::UnsupportedByDesign,
        representative_supported_canaries: &[],
        unproven_or_missing_canaries: &[
            "@config \"./tailwind.config.js\"",
            "theme.extend through Tailwind JS config",
        ],
        unsupported_by_design_reason: Some(NO_TAILWIND_RUNTIME),
        full_tailwind_parity_proven: false,
    },
    TailwindV43CssDirectiveLedgerEntry {
        directive_area: "legacy-tailwind",
        directive_syntax: "@tailwind",
        status: TailwindV43FeatureStatus::UnsupportedByDesign,
        representative_supported_canaries: &[],
        unproven_or_missing_canaries: &[
            "@tailwind base",
            "@tailwind components",
            "@tailwind utilities",
        ],
        unsupported_by_design_reason: Some(NO_TAILWIND_RUNTIME),
        full_tailwind_parity_proven: false,
    },
];
