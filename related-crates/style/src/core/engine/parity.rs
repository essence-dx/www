//! Tailwind-familiar generated-output parity receipts.
//!
//! This module records a small, source-owned class fixture and runs each class
//! through the real dx-style engine. It is a receipt for supported-subset
//! behavior, not a claim that dx-style implements every Tailwind class.

use super::StyleEngine;

/// Stable schema for generated-output parity receipts consumed by DX tools.
pub const TAILWIND_PARITY_RECEIPT_SCHEMA: &str = "dx.style.tailwind-parity";

/// Baseline Tailwind release used for this curated fixture set.
pub const TAILWIND_PARITY_BASELINE: &str = "tailwindcss-4.3.0-curated-fixture";

/// Public scope statement for consumers rendering the receipt.
pub const TAILWIND_PARITY_SCOPE: &str =
    "curated generated-output fixture; not full Tailwind class parity";

/// Support state for a Tailwind-familiar class in the parity receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TailwindParityStatus {
    /// dx-style generated normal CSS for the class.
    Supported,
    /// dx-style did not generate CSS for the class.
    Unsupported,
    /// dx-style generated CSS, but intentionally differs from Tailwind behavior.
    IntentionallyDifferent,
}

/// Curated input class used to build the generated-output receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TailwindParityFixture {
    /// Authored class name fed into `StyleEngine::css_for_class`.
    pub class_name: &'static str,
    /// Compatibility area used by docs and Check/Zed surfaces.
    pub area: &'static str,
    /// Expected receipt status for this fixture.
    pub expected_status: TailwindParityStatus,
    /// Human reason for the expected status.
    pub reason: &'static str,
}

/// One generated-output receipt entry.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct TailwindParityEntry {
    /// Authored class name.
    pub class_name: &'static str,
    /// Compatibility area.
    pub area: &'static str,
    /// Expected support state from the curated fixture.
    pub expected_status: TailwindParityStatus,
    /// Actual support state observed from `StyleEngine::css_for_class`.
    pub status: TailwindParityStatus,
    /// Why this fixture is classified this way.
    pub reason: &'static str,
    /// Generated normal CSS when dx-style supports the class.
    pub generated_css: Option<String>,
}

/// Generated-output parity receipt for the curated fixture set.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct TailwindParityReceipt {
    /// Receipt schema version.
    pub schema_version: &'static str,
    /// Tailwind baseline this fixture set tracks.
    pub tailwind_baseline: &'static str,
    /// Explicit compatibility scope.
    pub scope: &'static str,
    /// Receipt entries generated from the real dx-style engine.
    pub entries: Vec<TailwindParityEntry>,
}

impl TailwindParityReceipt {
    /// Count classes that generated CSS through dx-style.
    pub fn supported_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.status == TailwindParityStatus::Supported)
            .count()
    }

    /// Count classes that remain unsupported.
    pub fn unsupported_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.status == TailwindParityStatus::Unsupported)
            .count()
    }

    /// Count classes with intentional dx-style differences.
    pub fn intentionally_different_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.status == TailwindParityStatus::IntentionallyDifferent)
            .count()
    }
}

/// Curated class fixture that feeds the generated-output parity receipt.
pub fn tailwind_parity_fixtures() -> &'static [TailwindParityFixture] {
    TAILWIND_PARITY_FIXTURES
}

/// Build a generated-output parity receipt from the real dx-style engine.
pub fn build_tailwind_parity_receipt() -> TailwindParityReceipt {
    let engine = StyleEngine::empty();
    let entries = tailwind_parity_fixtures()
        .iter()
        .map(|fixture| {
            let generated_css = engine.css_for_class(fixture.class_name);
            let status = parity_status_for(fixture.expected_status, generated_css.is_some());

            TailwindParityEntry {
                class_name: fixture.class_name,
                area: fixture.area,
                expected_status: fixture.expected_status,
                status,
                reason: fixture.reason,
                generated_css,
            }
        })
        .collect();

    TailwindParityReceipt {
        schema_version: TAILWIND_PARITY_RECEIPT_SCHEMA,
        tailwind_baseline: TAILWIND_PARITY_BASELINE,
        scope: TAILWIND_PARITY_SCOPE,
        entries,
    }
}

/// Serialize the generated-output parity receipt for CLI/check consumers.
pub fn tailwind_parity_receipt_json() -> serde_json::Result<String> {
    serde_json::to_string_pretty(&build_tailwind_parity_receipt())
}

fn parity_status_for(
    expected_status: TailwindParityStatus,
    generated: bool,
) -> TailwindParityStatus {
    match (expected_status, generated) {
        (TailwindParityStatus::IntentionallyDifferent, true) => {
            TailwindParityStatus::IntentionallyDifferent
        }
        (_, true) => TailwindParityStatus::Supported,
        (_, false) => TailwindParityStatus::Unsupported,
    }
}

const TAILWIND_PARITY_FIXTURES: &[TailwindParityFixture] = &[
    TailwindParityFixture {
        class_name: "p-4",
        area: "spacing",
        expected_status: TailwindParityStatus::Supported,
        reason: "core spacing utility should emit padding CSS",
    },
    TailwindParityFixture {
        class_name: "-mt-2",
        area: "spacing",
        expected_status: TailwindParityStatus::Supported,
        reason: "negative spacing is part of the supported subset",
    },
    TailwindParityFixture {
        class_name: "text-red-500",
        area: "colors",
        expected_status: TailwindParityStatus::Supported,
        reason: "common Tailwind color tokens are generated",
    },
    TailwindParityFixture {
        class_name: "bg-blue-500/50",
        area: "colors",
        expected_status: TailwindParityStatus::Supported,
        reason: "alpha suffix colors are generated",
    },
    TailwindParityFixture {
        class_name: "bg-mauve-500",
        area: "tailwind-v4.3-palettes",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 mauve palette colors are generated through theme tokens",
    },
    TailwindParityFixture {
        class_name: "bg-olive-500",
        area: "tailwind-v4.3-palettes",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 olive palette colors are generated through theme tokens",
    },
    TailwindParityFixture {
        class_name: "bg-mist-500",
        area: "tailwind-v4.3-palettes",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 mist palette colors are generated through theme tokens",
    },
    TailwindParityFixture {
        class_name: "bg-taupe-500",
        area: "tailwind-v4.3-palettes",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 taupe palette colors are generated through theme tokens",
    },
    TailwindParityFixture {
        class_name: "text-olive-600/75",
        area: "tailwind-v4.3-palettes",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 palette opacity modifiers are generated through color-mix",
    },
    TailwindParityFixture {
        class_name: "border-mist-300",
        area: "tailwind-v4.3-palettes",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 palette border colors are generated through theme tokens",
    },
    TailwindParityFixture {
        class_name: "ring-taupe-400/50",
        area: "tailwind-v4.3-palettes",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 palette ring colors are generated through color-mix",
    },
    TailwindParityFixture {
        class_name: "outline-mauve-700",
        area: "tailwind-v4.3-palettes",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 palette outline colors are generated through theme tokens",
    },
    TailwindParityFixture {
        class_name: "decoration-olive-500",
        area: "tailwind-v4.3-palettes",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 palette decoration colors are generated through theme tokens",
    },
    TailwindParityFixture {
        class_name: "from-mist-500",
        area: "tailwind-v4.3-palettes",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 palette gradient-from colors are generated through theme tokens",
    },
    TailwindParityFixture {
        class_name: "via-taupe-500/40",
        area: "tailwind-v4.3-palettes",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 palette gradient-via colors are generated through color-mix",
    },
    TailwindParityFixture {
        class_name: "to-mauve-950",
        area: "tailwind-v4.3-palettes",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 palette gradient-to colors are generated through theme tokens",
    },
    TailwindParityFixture {
        class_name: "shadow-mauve-500",
        area: "tailwind-v4.3-palettes",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 palette shadow colors are generated through theme tokens",
    },
    TailwindParityFixture {
        class_name: "drop-shadow-mauve-500/50",
        area: "tailwind-v4.3-palettes",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 palette drop-shadow colors are generated through color-mix",
    },
    TailwindParityFixture {
        class_name: "inset-shadow-olive-500",
        area: "tailwind-v4.3-palettes",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 palette inset-shadow colors are generated through theme tokens",
    },
    TailwindParityFixture {
        class_name: "inset-ring-mist-500/50",
        area: "tailwind-v4.3-palettes",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 palette inset-ring colors are generated through color-mix",
    },
    TailwindParityFixture {
        class_name: "ring-offset-taupe-500/40",
        area: "tailwind-v4.3-palettes",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 palette ring-offset colors are generated through color-mix",
    },
    TailwindParityFixture {
        class_name: "bg-token(surface)",
        area: "theme-tokens",
        expected_status: TailwindParityStatus::Supported,
        reason: "DX token-aware background classes emit theme CSS variables",
    },
    TailwindParityFixture {
        class_name: "text-token(foreground)",
        area: "theme-tokens",
        expected_status: TailwindParityStatus::Supported,
        reason: "DX token-aware text classes emit theme CSS variables",
    },
    TailwindParityFixture {
        class_name: "border-token(border)",
        area: "theme-tokens",
        expected_status: TailwindParityStatus::Supported,
        reason: "DX token-aware border classes emit theme CSS variables",
    },
    TailwindParityFixture {
        class_name: "ring-token(ring)",
        area: "theme-tokens",
        expected_status: TailwindParityStatus::Supported,
        reason: "DX token-aware ring classes emit theme CSS variables",
    },
    TailwindParityFixture {
        class_name: "grid-cols-3",
        area: "grid",
        expected_status: TailwindParityStatus::Supported,
        reason: "numeric grid templates are generated",
    },
    TailwindParityFixture {
        class_name: "w-1/2",
        area: "sizing",
        expected_status: TailwindParityStatus::Supported,
        reason: "fractional sizing is generated",
    },
    TailwindParityFixture {
        class_name: "inline-1/2",
        area: "logical-sizing",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 logical inline-size utilities are generated",
    },
    TailwindParityFixture {
        class_name: "inline-3xs",
        area: "logical-sizing",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 logical inline-size container scale utilities are generated",
    },
    TailwindParityFixture {
        class_name: "min-inline-xl",
        area: "logical-sizing",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 logical min-inline-size container scale utilities are generated",
    },
    TailwindParityFixture {
        class_name: "inline-(--dx-inline-size)",
        area: "logical-sizing",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 logical inline-size custom-property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "block-screen",
        area: "logical-sizing",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 logical block-size viewport utilities are generated",
    },
    TailwindParityFixture {
        class_name: "scheme-light-dark",
        area: "tailwind-v4.3-interactivity",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 color-scheme helpers are generated",
    },
    TailwindParityFixture {
        class_name: "scrollbar-thin",
        area: "tailwind-v4.3-interactivity",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 scrollbar-width helpers are generated",
    },
    TailwindParityFixture {
        class_name: "scrollbar-gutter-both",
        area: "tailwind-v4.3-interactivity",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 scrollbar-gutter helpers are generated",
    },
    TailwindParityFixture {
        class_name: "scrollbar-thumb-red-500",
        area: "tailwind-v4.3-interactivity",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 scrollbar-color thumb helpers are generated",
    },
    TailwindParityFixture {
        class_name: "zoom-125",
        area: "tailwind-v4.3-interactivity",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 zoom percentage helpers are generated",
    },
    TailwindParityFixture {
        class_name: "tab-4",
        area: "tailwind-v4.3-interactivity",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 tab-size helpers are generated",
    },
    TailwindParityFixture {
        class_name: "wrap-anywhere",
        area: "tailwind-v4.3-typography",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 overflow-wrap helpers are generated",
    },
    TailwindParityFixture {
        class_name: "indent-8",
        area: "tailwind-v4.3-typography",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 text-indent spacing helpers are generated",
    },
    TailwindParityFixture {
        class_name: "align-middle",
        area: "tailwind-v4.3-typography",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 vertical-align helpers are generated",
    },
    TailwindParityFixture {
        class_name: "decoration-4",
        area: "tailwind-v4.3-typography",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 text-decoration-thickness helpers are generated",
    },
    TailwindParityFixture {
        class_name: "underline-offset-4",
        area: "tailwind-v4.3-typography",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 text-underline-offset helpers are generated",
    },
    TailwindParityFixture {
        class_name: "font-bold",
        area: "typography",
        expected_status: TailwindParityStatus::Supported,
        reason: "font-weight utilities are generated",
    },
    TailwindParityFixture {
        class_name: "rounded-lg",
        area: "radius",
        expected_status: TailwindParityStatus::Supported,
        reason: "radius scale utilities are generated",
    },
    TailwindParityFixture {
        class_name: "shadow-md",
        area: "effects",
        expected_status: TailwindParityStatus::Supported,
        reason: "common shadow utilities are generated",
    },
    TailwindParityFixture {
        class_name: "opacity-50",
        area: "effects",
        expected_status: TailwindParityStatus::Supported,
        reason: "numeric opacity utilities are generated",
    },
    TailwindParityFixture {
        class_name: "translate-y-4",
        area: "transforms",
        expected_status: TailwindParityStatus::Supported,
        reason: "translate utilities are generated",
    },
    TailwindParityFixture {
        class_name: "transform-gpu",
        area: "transforms",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind transform-gpu composition",
    },
    TailwindParityFixture {
        class_name: "transition-colors",
        area: "transitions",
        expected_status: TailwindParityStatus::Supported,
        reason: "common transition-property utilities are generated",
    },
    TailwindParityFixture {
        class_name: "animate-spin",
        area: "animation",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind-style animation utilities are generated with dx keyframe names",
    },
    TailwindParityFixture {
        class_name: "blur-sm",
        area: "filters",
        expected_status: TailwindParityStatus::Supported,
        reason: "filter utilities are generated",
    },
    TailwindParityFixture {
        class_name: "md:hover:bg-blue-500",
        area: "responsive-state",
        expected_status: TailwindParityStatus::Supported,
        reason: "responsive state variants compose around supported utilities",
    },
    TailwindParityFixture {
        class_name: "dark:text-white",
        area: "dark-mode",
        expected_status: TailwindParityStatus::Supported,
        reason: "dark wrapper variants are generated",
    },
    TailwindParityFixture {
        class_name: "data-[state=open]:opacity-100",
        area: "data-aria-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "safe arbitrary data variants are generated",
    },
    TailwindParityFixture {
        class_name: "group-hover/card:text-slate-900",
        area: "group-peer-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "named group variants are generated",
    },
    TailwindParityFixture {
        class_name: "group-odd:bg-mauve-500",
        area: "group-peer-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "group pseudo-class state variants compose with supported utilities",
    },
    TailwindParityFixture {
        class_name: "group-disabled:opacity-100",
        area: "group-peer-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "group form state variants compose with supported utilities",
    },
    TailwindParityFixture {
        class_name: "group-focus-visible/card:opacity-100",
        area: "group-peer-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "named group pseudo-class state variants are generated",
    },
    TailwindParityFixture {
        class_name: "peer-invalid:visible",
        area: "group-peer-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "peer validation state variants compose with supported utilities",
    },
    TailwindParityFixture {
        class_name: "peer-required/email:block",
        area: "group-peer-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "named peer pseudo-class state variants are generated",
    },
    TailwindParityFixture {
        class_name: "peer-disabled:opacity-100",
        area: "group-peer-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "peer form state variants compose with supported utilities",
    },
    TailwindParityFixture {
        class_name: "target:p-4",
        area: "state-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "common location state aliases generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "read-only:bg-blue-500",
        area: "state-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "common form state aliases generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "indeterminate:opacity-100",
        area: "state-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "common control state aliases generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "user-valid:border-green-500",
        area: "state-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind user-valid selector variants generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "user-invalid:border-red-500",
        area: "state-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind user-invalid selector variants generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "details-content:bg-slate-100",
        area: "state-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind details-content selector variants generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "rtl:ps-4",
        area: "direction-and-attribute-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind rtl direction variants generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "ltr:pe-4",
        area: "direction-and-attribute-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind ltr direction variants generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "inert:opacity-50",
        area: "direction-and-attribute-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind inert selector variants generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "open:bg-blue-500",
        area: "direction-and-attribute-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind open selector variants generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "starting:open:opacity-0",
        area: "direction-and-attribute-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind starting and open variants compose in normal CSS",
    },
    TailwindParityFixture {
        class_name: "backdrop:bg-slate-950/50",
        area: "pseudo-elements",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind backdrop pseudo-element variants generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "has-even:bg-blue-500",
        area: "conditional-state-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "common state aliases compose through has-* variants",
    },
    TailwindParityFixture {
        class_name: "not-visited:text-slate-900",
        area: "conditional-state-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "common state aliases compose through not-* variants",
    },
    TailwindParityFixture {
        class_name: "in-read-only:p-4",
        area: "conditional-state-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "common state aliases compose through in-* ancestor variants",
    },
    TailwindParityFixture {
        class_name: "before:content-['New']",
        area: "pseudo-elements",
        expected_status: TailwindParityStatus::Supported,
        reason: "safe content utilities compose with before pseudo-elements",
    },
    TailwindParityFixture {
        class_name: "[@media_(any-hover:hover)]:opacity-100",
        area: "arbitrary-at-rules",
        expected_status: TailwindParityStatus::Supported,
        reason: "safe arbitrary media at-rules are generated",
    },
    TailwindParityFixture {
        class_name: "pointer-fine:opacity-100",
        area: "media-capability-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind pointer media variants generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "any-pointer-coarse:opacity-100",
        area: "media-capability-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind any-pointer media variants generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "forced-colors:opacity-100",
        area: "media-capability-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind forced-colors media variants generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "noscript:opacity-100",
        area: "media-capability-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind scripting media variants generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "not-pointer-fine:opacity-100",
        area: "media-capability-variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind negated media variants generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "@max-md:text-slate-900",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "max-width container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@3xs:grid",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 3xs container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@2xs:flex",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 2xs container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@xs:block",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 xs container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@sm:grid",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 sm container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@md:flex",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 md container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@lg:block",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 lg container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@xl:grid",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 xl container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@2xl:flex",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 2xl container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@3xl/block:flex",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 named 3xl container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@4xl:grid",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 4xl container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@5xl:block",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 5xl container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@6xl:flex",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 6xl container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@7xl:flex",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 7xl container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@max-3xs:hidden",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 max-3xs container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@max-2xs:block",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 max-2xs container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@max-xs:flex",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 max-xs container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@max-sm:hidden",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 max-sm container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@max-md:block",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 max-md container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@max-lg:flex",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 max-lg container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@max-xl:hidden",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 max-xl container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@max-2xl:block",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 max-2xl container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@max-3xl:flex",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 max-3xl container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@max-4xl:hidden",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 max-4xl container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@max-5xl:block",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 max-5xl container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@max-6xl:flex",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 max-6xl container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@max-7xl:block",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 max-7xl container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@3xl/main:opacity-100",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 named 3xl container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@max-md:@sm:flex",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind container query range variants preserve written nesting order",
    },
    TailwindParityFixture {
        class_name: "@sm:@max-md:flex",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind container query range variants preserve min-first written nesting order",
    },
    TailwindParityFixture {
        class_name: "@max-md/main:@sm/main:grid",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Named Tailwind container query range variants preserve written nesting order",
    },
    TailwindParityFixture {
        class_name: "@max-[960px]:@min-[475px]:hidden",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Arbitrary Tailwind container query range variants preserve written nesting order",
    },
    TailwindParityFixture {
        class_name: "@min-[123px]:flex",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Arbitrary Tailwind min-width container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@max-[123px]:hidden",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Arbitrary Tailwind max-width container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@min-[456px]/name:grid",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Named arbitrary Tailwind min-width container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@max-[456px]/name:block",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Named arbitrary Tailwind max-width container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@max-[960px]/name:@min-[475px]/name:flex",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Named arbitrary Tailwind container query range variants preserve written nesting order",
    },
    TailwindParityFixture {
        class_name: "@[475px]:flex",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind arbitrary shorthand container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@[475px]/card:grid",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Named Tailwind arbitrary shorthand container query variants are generated",
    },
    TailwindParityFixture {
        class_name: "@[475px]:@max-[960px]:block",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind arbitrary shorthand container query ranges preserve written nesting order",
    },
    TailwindParityFixture {
        class_name: "@[475px]/card:@max-[960px]/card:hidden",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Named Tailwind arbitrary shorthand container query ranges preserve written nesting order",
    },
    TailwindParityFixture {
        class_name: "@min-[40rem]:@max-[70rem]:flex",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind arbitrary min/max container query ranges preserve written nesting order",
    },
    TailwindParityFixture {
        class_name: "@min-[40rem]/main:@max-[70rem]/main:grid",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Named Tailwind arbitrary min/max container query ranges preserve written nesting order",
    },
    TailwindParityFixture {
        class_name: "@max-7xl:@3xs:flex",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind container query range variants preserve written nesting order across edge tokens",
    },
    TailwindParityFixture {
        class_name: "@3xs:@max-7xl:flex",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind container query range variants preserve reversed written nesting order across edge tokens",
    },
    TailwindParityFixture {
        class_name: "@max-7xl/main:@3xs/main:grid",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Named Tailwind container query range variants preserve written nesting order across edge tokens",
    },
    TailwindParityFixture {
        class_name: "@3xs:@md:@max-7xl:flex",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind multi-depth container query stacks preserve written nesting order",
    },
    TailwindParityFixture {
        class_name: "@max-7xl:@md:@3xs:grid",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind reversed multi-depth container query stacks preserve written nesting order",
    },
    TailwindParityFixture {
        class_name: "@min-[30rem]/rail:@max-[50rem]/rail:block",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Named Tailwind arbitrary min/max container query ranges preserve written nesting order",
    },
    TailwindParityFixture {
        class_name: "@max-[50rem]/rail:@min-[30rem]/rail:hidden",
        area: "container-queries",
        expected_status: TailwindParityStatus::Supported,
        reason: "Named Tailwind arbitrary max/min container query ranges preserve written nesting order",
    },
    TailwindParityFixture {
        class_name: "@container",
        area: "tailwind-v4.3-container-type",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 inline-size container utilities generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "@container/sidebar",
        area: "tailwind-v4.3-container-type",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 named inline-size container utilities generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "@container-normal",
        area: "tailwind-v4.3-container-type",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 normal container utilities generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "@container-normal/sidebar",
        area: "tailwind-v4.3-container-type",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 named normal container utilities generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "@container-size",
        area: "tailwind-v4.3-container-type",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 size container utilities generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "@container-size/main",
        area: "tailwind-v4.3-container-type",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 named size container utilities generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "@container-size/sidebar",
        area: "tailwind-v4.3-container-type",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 named size container utilities generate normal CSS",
    },
    TailwindParityFixture {
        class_name: "prose",
        area: "typography-plugin",
        expected_status: TailwindParityStatus::Supported,
        reason: "baseline Tailwind typography plugin class generates nested prose CSS",
    },
    TailwindParityFixture {
        class_name: "prose-a:text-blue-600",
        area: "typography-plugin",
        expected_status: TailwindParityStatus::Supported,
        reason: "typography element variants generate nested prose element CSS",
    },
    TailwindParityFixture {
        class_name: "text-shadow-sm",
        area: "effects",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for text-shadow utility classes",
    },
    TailwindParityFixture {
        class_name: "text-shadow-cyan-500/50",
        area: "effects",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits text-shadow color variables with opacity modifiers",
    },
    TailwindParityFixture {
        class_name: "hover:not-focus:text-shadow-sky-300/50",
        area: "effects",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits stacked variant text-shadow color utilities",
    },
    TailwindParityFixture {
        class_name: "field-sizing-content",
        area: "forms",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for field-sizing utility classes",
    },
    TailwindParityFixture {
        class_name: "mask-radial-from-50%",
        area: "effects",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits prefixed normal CSS for radial mask utility classes",
    },
    TailwindParityFixture {
        class_name: "mask-conic-from-50%",
        area: "effects",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits prefixed normal CSS for conic mask utility classes",
    },
    TailwindParityFixture {
        class_name: "mask-l-from-50%",
        area: "effects",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits prefixed normal CSS for linear edge mask utility classes",
    },
    TailwindParityFixture {
        class_name: "mask-linear-from-60%",
        area: "effects",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits prefixed normal CSS for angled linear mask stops",
    },
    TailwindParityFixture {
        class_name: "mask-linear-[70deg,transparent_10%,black,transparent_80%]",
        area: "effects",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits prefixed normal CSS for safe arbitrary linear mask-image shorthands",
    },
    TailwindParityFixture {
        class_name: "container",
        area: "layout",
        expected_status: TailwindParityStatus::IntentionallyDifferent,
        reason: "dx-style keeps container minimal instead of Tailwind responsive container defaults",
    },
    TailwindParityFixture {
        class_name: "mask-radial-[100%_100%]",
        area: "effects",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits prefixed normal CSS for arbitrary radial mask sizing shorthands",
    },
    TailwindParityFixture {
        class_name: "mask-alpha",
        area: "effects",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits prefixed normal CSS for Tailwind mask-mode utilities",
    },
    TailwindParityFixture {
        class_name: "mask-origin-content",
        area: "effects",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits prefixed normal CSS for Tailwind mask-origin utilities",
    },
    TailwindParityFixture {
        class_name: "mask-type-alpha",
        area: "effects",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind SVG mask-type utilities",
    },
    TailwindParityFixture {
        class_name: "font-stretch-condensed",
        area: "typography",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind font-stretch utilities",
    },
    TailwindParityFixture {
        class_name: "font-features-['smcp','onum']",
        area: "tailwind-v4.2-font-features",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind font-feature-settings arbitrary values",
    },
    TailwindParityFixture {
        class_name: "font-features-(--dx-font-features)",
        area: "tailwind-v4.2-font-features",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind font-feature-settings custom properties",
    },
    TailwindParityFixture {
        class_name: "tabular-nums",
        area: "typography",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind numeric font-variant utilities",
    },
    TailwindParityFixture {
        class_name: "ordinal",
        area: "typography",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind OpenType ordinal numeric utilities",
    },
    TailwindParityFixture {
        class_name: "forced-color-adjust-auto",
        area: "accessibility",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind forced-color-adjust utilities",
    },
    TailwindParityFixture {
        class_name: "outline-2",
        area: "effects",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind outline width utilities",
    },
    TailwindParityFixture {
        class_name: "ring-inset",
        area: "effects",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind ring inset helpers",
    },
    TailwindParityFixture {
        class_name: "ring-offset-2",
        area: "effects",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind ring offset width helpers",
    },
    TailwindParityFixture {
        class_name: "touch-pan-left",
        area: "interactivity",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind touch-action directional helpers",
    },
    TailwindParityFixture {
        class_name: "touch-pinch-zoom",
        area: "interactivity",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind touch-action pinch zoom helpers",
    },
    TailwindParityFixture {
        class_name: "columns-3",
        area: "layout",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind columns utilities",
    },
    TailwindParityFixture {
        class_name: "break-before-page",
        area: "layout",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind break utilities",
    },
    TailwindParityFixture {
        class_name: "box-decoration-clone",
        area: "layout",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits prefixed normal CSS for Tailwind box-decoration-break utilities",
    },
    TailwindParityFixture {
        class_name: "bg-blend-multiply",
        area: "effects",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind blend mode utilities",
    },
    TailwindParityFixture {
        class_name: "bg-origin-border",
        area: "backgrounds",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind background-origin utilities",
    },
    TailwindParityFixture {
        class_name: "bg-none",
        area: "backgrounds",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind background image reset utility",
    },
    TailwindParityFixture {
        class_name: "bg-linear-to-r",
        area: "backgrounds",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind v4 linear background gradient aliases",
    },
    TailwindParityFixture {
        class_name: "bg-linear-to-r/oklch",
        area: "backgrounds",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind v4 gradient interpolation modifiers",
    },
    TailwindParityFixture {
        class_name: "bg-linear-to-r/longer",
        area: "backgrounds",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind v4 hue interpolation modifiers",
    },
    TailwindParityFixture {
        class_name: "bg-radial",
        area: "backgrounds",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind radial background gradient utilities",
    },
    TailwindParityFixture {
        class_name: "bg-radial-[circle_at_center]",
        area: "backgrounds",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for safe arbitrary Tailwind radial background gradient positions",
    },
    TailwindParityFixture {
        class_name: "bg-radial-(--dx-bg-radial)",
        area: "backgrounds",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind custom-property radial background gradient aliases",
    },
    TailwindParityFixture {
        class_name: "bg-conic",
        area: "backgrounds",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind conic background gradient utilities",
    },
    TailwindParityFixture {
        class_name: "bg-conic/decreasing",
        area: "backgrounds",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind conic gradient interpolation modifiers",
    },
    TailwindParityFixture {
        class_name: "bg-conic-180",
        area: "backgrounds",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind conic background gradient utilities",
    },
    TailwindParityFixture {
        class_name: "bg-conic-180/shorter",
        area: "backgrounds",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind angle plus interpolation modifiers",
    },
    TailwindParityFixture {
        class_name: "-bg-conic-45",
        area: "backgrounds",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for negative-angle Tailwind conic background gradient utilities",
    },
    TailwindParityFixture {
        class_name: "bg-conic-[from_45deg_at_center]",
        area: "backgrounds",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for safe arbitrary Tailwind conic background gradient positions",
    },
    TailwindParityFixture {
        class_name: "bg-conic/[in_hsl_longer_hue]",
        area: "backgrounds",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for safe arbitrary Tailwind gradient interpolation modifiers",
    },
    TailwindParityFixture {
        class_name: "bg-conic-(--dx-bg-conic)",
        area: "backgrounds",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind custom-property conic background gradient aliases",
    },
    TailwindParityFixture {
        class_name: "bg-linear-45",
        area: "backgrounds",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind linear angle background gradient utilities",
    },
    TailwindParityFixture {
        class_name: "bg-[url('/hero.png')]",
        area: "backgrounds",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for safe arbitrary background image values",
    },
    TailwindParityFixture {
        class_name: "bg-size-(--dx-bg-size)",
        area: "backgrounds",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for Tailwind custom-property background-size aliases",
    },
    TailwindParityFixture {
        class_name: "size-8",
        area: "sizing",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind size shorthand emits width and height declarations",
    },
    TailwindParityFixture {
        class_name: "px-4",
        area: "logical-spacing",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4 logical inline padding shorthand emits padding-inline",
    },
    TailwindParityFixture {
        class_name: "py-2",
        area: "logical-spacing",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4 logical block padding shorthand emits padding-block",
    },
    TailwindParityFixture {
        class_name: "pbs-4",
        area: "logical-spacing",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4 logical block-start padding emits calc(var(--spacing) * n)",
    },
    TailwindParityFixture {
        class_name: "mx-auto",
        area: "logical-spacing",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4 logical inline margin shorthand emits margin-inline",
    },
    TailwindParityFixture {
        class_name: "-mbs-2",
        area: "logical-spacing",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4 negative logical block-start margin emits calc(var(--spacing) * -n)",
    },
    TailwindParityFixture {
        class_name: "space-x-4",
        area: "logical-spacing",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4 logical horizontal space utilities emit logical child margins",
    },
    TailwindParityFixture {
        class_name: "space-x-reverse",
        area: "logical-spacing",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4 logical horizontal space reverse utility emits the reverse variable",
    },
    TailwindParityFixture {
        class_name: "inset-s-4",
        area: "logical-inset",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4 logical inset start emits calc(var(--spacing) * n)",
    },
    TailwindParityFixture {
        class_name: "-inset-e-1/2",
        area: "logical-inset",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4 negative logical inset fractions preserve nested calc serialization",
    },
    TailwindParityFixture {
        class_name: "border-x",
        area: "logical-border",
        expected_status: TailwindParityStatus::Supported,
        reason: "logical border inline width utilities include Tailwind border-style scaffolding",
    },
    TailwindParityFixture {
        class_name: "border-y-4",
        area: "logical-border",
        expected_status: TailwindParityStatus::Supported,
        reason: "logical border block width utilities include Tailwind border-style scaffolding",
    },
    TailwindParityFixture {
        class_name: "border-s-2",
        area: "logical-border",
        expected_status: TailwindParityStatus::Supported,
        reason: "logical border start width utilities are generated",
    },
    TailwindParityFixture {
        class_name: "border-s-red-500",
        area: "logical-border",
        expected_status: TailwindParityStatus::Supported,
        reason: "logical border inline-start color utilities are generated",
    },
    TailwindParityFixture {
        class_name: "border-bs-emerald-500",
        area: "logical-border",
        expected_status: TailwindParityStatus::Supported,
        reason: "logical border block-start color utilities are generated",
    },
    TailwindParityFixture {
        class_name: "rounded-ee-xl",
        area: "logical-radius",
        expected_status: TailwindParityStatus::Supported,
        reason: "logical corner radius utilities are generated",
    },
    TailwindParityFixture {
        class_name: "scroll-ms-4",
        area: "logical-scroll",
        expected_status: TailwindParityStatus::Supported,
        reason: "logical scroll margin utilities are generated",
    },
    TailwindParityFixture {
        class_name: "scroll-mbs-6",
        area: "logical-scroll",
        expected_status: TailwindParityStatus::Supported,
        reason: "logical scroll block-start margin utilities preserve --spacing serialization",
    },
    TailwindParityFixture {
        class_name: "scroll-pe-2",
        area: "logical-scroll",
        expected_status: TailwindParityStatus::Supported,
        reason: "logical scroll padding utilities are generated",
    },
    TailwindParityFixture {
        class_name: "scroll-pbe-2",
        area: "logical-scroll",
        expected_status: TailwindParityStatus::Supported,
        reason: "logical scroll block-end padding utilities preserve --spacing serialization",
    },
    TailwindParityFixture {
        class_name: "object-[25%_75%]",
        area: "object-position",
        expected_status: TailwindParityStatus::Supported,
        reason: "safe arbitrary object-position utilities are generated",
    },
    TailwindParityFixture {
        class_name: "object-(--dx-object-position)",
        area: "object-position",
        expected_status: TailwindParityStatus::Supported,
        reason: "object-position custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "origin-top-left",
        area: "transform-origin",
        expected_status: TailwindParityStatus::Supported,
        reason: "transform-origin placement utilities are generated",
    },
    TailwindParityFixture {
        class_name: "skew-x-6",
        area: "transforms",
        expected_status: TailwindParityStatus::Supported,
        reason: "skew transform utilities are generated",
    },
    TailwindParityFixture {
        class_name: "-skew-y-3",
        area: "transforms",
        expected_status: TailwindParityStatus::Supported,
        reason: "negative skew transform utilities are generated",
    },
    TailwindParityFixture {
        class_name: "flex-[2_1_0%]",
        area: "flex",
        expected_status: TailwindParityStatus::Supported,
        reason: "safe arbitrary flex shorthand utilities are generated",
    },
    TailwindParityFixture {
        class_name: "grow-[2]",
        area: "flex",
        expected_status: TailwindParityStatus::Supported,
        reason: "safe arbitrary flex-grow utilities are generated",
    },
    TailwindParityFixture {
        class_name: "shrink-[3]",
        area: "flex",
        expected_status: TailwindParityStatus::Supported,
        reason: "safe arbitrary flex-shrink utilities are generated",
    },
    TailwindParityFixture {
        class_name: "-order-1",
        area: "layout",
        expected_status: TailwindParityStatus::Supported,
        reason: "negative order utilities are generated",
    },
    TailwindParityFixture {
        class_name: "grid-cols-(--dx-grid-cols)",
        area: "grid",
        expected_status: TailwindParityStatus::Supported,
        reason: "grid template custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "auto-rows-(--dx-auto-rows)",
        area: "grid",
        expected_status: TailwindParityStatus::Supported,
        reason: "grid auto-row custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "drop-shadow-md",
        area: "filters",
        expected_status: TailwindParityStatus::Supported,
        reason: "drop-shadow filter utilities are generated",
    },
    TailwindParityFixture {
        class_name: "backdrop-opacity-50",
        area: "backdrop-filters",
        expected_status: TailwindParityStatus::Supported,
        reason: "backdrop opacity filter utilities are generated",
    },
    TailwindParityFixture {
        class_name: "backdrop-invert",
        area: "backdrop-filters",
        expected_status: TailwindParityStatus::Supported,
        reason: "backdrop invert filter utilities are generated",
    },
    TailwindParityFixture {
        class_name: "table-auto",
        area: "table",
        expected_status: TailwindParityStatus::Supported,
        reason: "table layout utilities are generated",
    },
    TailwindParityFixture {
        class_name: "table-fixed",
        area: "table",
        expected_status: TailwindParityStatus::Supported,
        reason: "table layout utilities are generated",
    },
    TailwindParityFixture {
        class_name: "caption-bottom",
        area: "table",
        expected_status: TailwindParityStatus::Supported,
        reason: "table caption side utilities are generated",
    },
    TailwindParityFixture {
        class_name: "border-collapse",
        area: "table",
        expected_status: TailwindParityStatus::Supported,
        reason: "table border-collapse utilities are generated",
    },
    TailwindParityFixture {
        class_name: "border-spacing-2",
        area: "table",
        expected_status: TailwindParityStatus::Supported,
        reason: "table border-spacing utilities are generated",
    },
    TailwindParityFixture {
        class_name: "border-spacing-x-4",
        area: "table",
        expected_status: TailwindParityStatus::Supported,
        reason: "logical table border-spacing axis utilities are generated",
    },
    TailwindParityFixture {
        class_name: "transform-3d",
        area: "transforms",
        expected_status: TailwindParityStatus::Supported,
        reason: "3d transform-style utilities are generated",
    },
    TailwindParityFixture {
        class_name: "perspective-dramatic",
        area: "transforms",
        expected_status: TailwindParityStatus::Supported,
        reason: "Tailwind v4.3 named perspective presets are generated",
    },
    TailwindParityFixture {
        class_name: "perspective-[750px]",
        area: "transforms",
        expected_status: TailwindParityStatus::Supported,
        reason: "safe arbitrary perspective utilities are generated",
    },
    TailwindParityFixture {
        class_name: "perspective-origin-top-right",
        area: "transforms",
        expected_status: TailwindParityStatus::Supported,
        reason: "perspective origin position utilities are generated",
    },
    TailwindParityFixture {
        class_name: "transform-(--dx-transform)",
        area: "transforms",
        expected_status: TailwindParityStatus::Supported,
        reason: "transform custom-property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "rotate-x-45",
        area: "transforms",
        expected_status: TailwindParityStatus::Supported,
        reason: "3d rotation utilities are generated",
    },
    TailwindParityFixture {
        class_name: "-rotate-y-12",
        area: "transforms",
        expected_status: TailwindParityStatus::Supported,
        reason: "negative 3d rotation utilities are generated",
    },
    TailwindParityFixture {
        class_name: "translate-z-4",
        area: "transforms",
        expected_status: TailwindParityStatus::Supported,
        reason: "3d translation utilities are generated",
    },
    TailwindParityFixture {
        class_name: "scale-z-125",
        area: "transforms",
        expected_status: TailwindParityStatus::Supported,
        reason: "3d scale utilities are generated",
    },
    TailwindParityFixture {
        class_name: "scale-z-(--dx-scale-z)",
        area: "transforms",
        expected_status: TailwindParityStatus::Supported,
        reason: "3d scale custom-property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "-scale-z-125",
        area: "transforms",
        expected_status: TailwindParityStatus::Supported,
        reason: "negative 3d scale utilities are generated",
    },
    TailwindParityFixture {
        class_name: "inset-shadow-[inset_0_1px_2px_rgb(0_0_0_/_0.1)]",
        area: "shadows",
        expected_status: TailwindParityStatus::Supported,
        reason: "safe arbitrary inset shadow utilities are generated",
    },
    TailwindParityFixture {
        class_name: "shadow-(--dx-shadow)",
        area: "shadows",
        expected_status: TailwindParityStatus::Supported,
        reason: "box-shadow custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "box-border",
        area: "box-sizing",
        expected_status: TailwindParityStatus::Supported,
        reason: "box sizing utilities are generated",
    },
    TailwindParityFixture {
        class_name: "box-content",
        area: "box-sizing",
        expected_status: TailwindParityStatus::Supported,
        reason: "box sizing utilities are generated",
    },
    TailwindParityFixture {
        class_name: "bg-[#1d4ed8]/50",
        area: "colors",
        expected_status: TailwindParityStatus::Supported,
        reason: "safe arbitrary hex colors with opacity are generated",
    },
    TailwindParityFixture {
        class_name: "bg-(--dx-background)",
        area: "colors",
        expected_status: TailwindParityStatus::Supported,
        reason: "color custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "border-(--dx-border)",
        area: "colors",
        expected_status: TailwindParityStatus::Supported,
        reason: "border color custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "fill-[#0f172a]/80",
        area: "svg",
        expected_status: TailwindParityStatus::Supported,
        reason: "safe arbitrary SVG fill colors with opacity are generated",
    },
    TailwindParityFixture {
        class_name: "text-[color:var(--dx-foreground)]",
        area: "colors",
        expected_status: TailwindParityStatus::Supported,
        reason: "typed arbitrary text colors are routed to color utilities",
    },
    TailwindParityFixture {
        class_name: "stroke-[color:var(--dx-stroke)]",
        area: "svg",
        expected_status: TailwindParityStatus::Supported,
        reason: "typed arbitrary SVG stroke colors are generated",
    },
    TailwindParityFixture {
        class_name: "rounded-(--dx-radius)",
        area: "borders",
        expected_status: TailwindParityStatus::Supported,
        reason: "border-radius custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "aspect-(--dx-aspect)",
        area: "layout",
        expected_status: TailwindParityStatus::Supported,
        reason: "aspect-ratio custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "leading-(--dx-leading)",
        area: "typography",
        expected_status: TailwindParityStatus::Supported,
        reason: "line-height custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "tracking-(--dx-tracking)",
        area: "typography",
        expected_status: TailwindParityStatus::Supported,
        reason: "letter-spacing custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "items-baseline-last",
        area: "alignment",
        expected_status: TailwindParityStatus::Supported,
        reason: "last-baseline item alignment utilities are generated",
    },
    TailwindParityFixture {
        class_name: "self-baseline",
        area: "alignment",
        expected_status: TailwindParityStatus::Supported,
        reason: "baseline self-alignment utilities are generated",
    },
    TailwindParityFixture {
        class_name: "place-items-baseline",
        area: "alignment",
        expected_status: TailwindParityStatus::Supported,
        reason: "baseline place-items utilities are generated",
    },
    TailwindParityFixture {
        class_name: "justify-items-center",
        area: "alignment",
        expected_status: TailwindParityStatus::Supported,
        reason: "justify-items utilities are generated",
    },
    TailwindParityFixture {
        class_name: "justify-self-end",
        area: "alignment",
        expected_status: TailwindParityStatus::Supported,
        reason: "justify-self utilities are generated",
    },
    TailwindParityFixture {
        class_name: "content-normal",
        area: "alignment",
        expected_status: TailwindParityStatus::Supported,
        reason: "normal align-content utilities are generated",
    },
    TailwindParityFixture {
        class_name: "from-10%",
        area: "gradients",
        expected_status: TailwindParityStatus::Supported,
        reason: "gradient from-position utilities are generated",
    },
    TailwindParityFixture {
        class_name: "via-30%",
        area: "gradients",
        expected_status: TailwindParityStatus::Supported,
        reason: "gradient via-position utilities are generated",
    },
    TailwindParityFixture {
        class_name: "to-90%",
        area: "gradients",
        expected_status: TailwindParityStatus::Supported,
        reason: "gradient to-position utilities are generated",
    },
    TailwindParityFixture {
        class_name: "from-(--dx-gradient-from-position)",
        area: "gradients",
        expected_status: TailwindParityStatus::Supported,
        reason: "gradient stop custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "ring-[#2563eb]/50",
        area: "colors",
        expected_status: TailwindParityStatus::Supported,
        reason: "safe arbitrary ring colors with opacity are generated",
    },
    TailwindParityFixture {
        class_name: "divide-(--dx-border)",
        area: "borders",
        expected_status: TailwindParityStatus::Supported,
        reason: "divide color custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "p-(--dx-pad)",
        area: "custom-property-shorthand",
        expected_status: TailwindParityStatus::Supported,
        reason: "spacing custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "scroll-mt-(--dx-scroll-mt)",
        area: "custom-property-shorthand",
        expected_status: TailwindParityStatus::Supported,
        reason: "scroll spacing custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "opacity-(--dx-opacity)",
        area: "custom-property-shorthand",
        expected_status: TailwindParityStatus::Supported,
        reason: "opacity custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "z-(--dx-layer)",
        area: "custom-property-shorthand",
        expected_status: TailwindParityStatus::Supported,
        reason: "z-index custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "order-(--dx-order)",
        area: "custom-property-shorthand",
        expected_status: TailwindParityStatus::Supported,
        reason: "order custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "blur-(--dx-blur)",
        area: "custom-property-shorthand",
        expected_status: TailwindParityStatus::Supported,
        reason: "filter custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "brightness-(--dx-brightness)",
        area: "custom-property-shorthand",
        expected_status: TailwindParityStatus::Supported,
        reason: "filter numeric custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "hue-rotate-(--dx-hue-rotate)",
        area: "custom-property-shorthand",
        expected_status: TailwindParityStatus::Supported,
        reason: "filter angle custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "backdrop-opacity-(--dx-backdrop-opacity)",
        area: "custom-property-shorthand",
        expected_status: TailwindParityStatus::Supported,
        reason: "backdrop filter custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "backdrop-blur-(--dx-backdrop-blur)",
        area: "custom-property-shorthand",
        expected_status: TailwindParityStatus::Supported,
        reason: "backdrop blur custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "outline-offset-(--dx-outline-offset)",
        area: "custom-property-shorthand",
        expected_status: TailwindParityStatus::Supported,
        reason: "outline offset custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "outline-[3px]",
        area: "arbitrary-length-routing",
        expected_status: TailwindParityStatus::Supported,
        reason: "arbitrary outline lengths route to outline width",
    },
    TailwindParityFixture {
        class_name: "ring-[3px]",
        area: "arbitrary-length-routing",
        expected_status: TailwindParityStatus::Supported,
        reason: "arbitrary ring lengths route to ring width",
    },
    TailwindParityFixture {
        class_name: "ring-offset-(--dx-ring-offset-width)",
        area: "custom-property-shorthand",
        expected_status: TailwindParityStatus::Supported,
        reason: "ring offset width custom property aliases are generated",
    },
    TailwindParityFixture {
        class_name: "appearance-none",
        area: "browser-compat",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits prefixed normal CSS for Tailwind appearance utilities",
    },
    TailwindParityFixture {
        class_name: "select-none",
        area: "browser-compat",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits prefixed normal CSS for Tailwind user-select utilities",
    },
    TailwindParityFixture {
        class_name: "backface-hidden",
        area: "browser-compat",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits prefixed normal CSS for Tailwind backface-visibility utilities",
    },
    TailwindParityFixture {
        class_name: "break-inside-avoid",
        area: "browser-compat",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits legacy page-break and standard CSS for Tailwind break-inside avoidance",
    },
    TailwindParityFixture {
        class_name: "backdrop-blur-md",
        area: "browser-compat",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits prefixed normal CSS for Tailwind backdrop-filter utilities",
    },
    TailwindParityFixture {
        class_name: "hyphens-auto",
        area: "browser-compat",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits prefixed normal CSS for Tailwind hyphenation utilities",
    },
    TailwindParityFixture {
        class_name: "file:p-4",
        area: "variants",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits Tailwind v4 standard file selector button pseudo-element variants",
    },
    TailwindParityFixture {
        class_name: "[@starting-style]:opacity-0",
        area: "arbitrary-at-rules",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for safe Tailwind @starting-style arbitrary variants",
    },
    TailwindParityFixture {
        class_name: "[@layer_components]:p-4",
        area: "arbitrary-at-rules",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for safe Tailwind @layer arbitrary variants",
    },
    TailwindParityFixture {
        class_name: "[@media_(any-hover:hover){&:hover}]:opacity-100",
        area: "arbitrary-at-rules",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for safe arbitrary at-rule variants with nested selectors",
    },
    TailwindParityFixture {
        class_name: "[@unknown_rule]:p-4",
        area: "arbitrary-at-rules",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits normal CSS for safe unknown arbitrary at-rule variants while build-time Tailwind directives still fail closed",
    },
    TailwindParityFixture {
        class_name: "[&.foo]:[&.bar]:flex",
        area: "arbitrary-selector-stacking",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style composes stacked arbitrary selector variants into one Tailwind-style selector",
    },
    TailwindParityFixture {
        class_name: "[&_p]:[&_.lead]:mt-4",
        area: "arbitrary-selector-stacking",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style composes stacked arbitrary descendant selectors instead of emitting independent blocks",
    },
    TailwindParityFixture {
        class_name: "not-[.is-open]:[&.dismissible]:opacity-100",
        area: "arbitrary-selector-stacking",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style preserves left-to-right selector composition across not-[...] and arbitrary selector variants",
    },
    TailwindParityFixture {
        class_name: "[&.is-dragging]:active:cursor-grabbing",
        area: "arbitrary-selector-stacking",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style composes arbitrary selector variants with pseudo-class variants in Tailwind order",
    },
    TailwindParityFixture {
        class_name: "[&.foo,&.bar]:[&>.item,&>[data-slot=control]]:opacity-100",
        area: "arbitrary-selector-stacking",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style composes stacked arbitrary selector lists branch-by-branch instead of leaking whole lists into one branch",
    },
    TailwindParityFixture {
        class_name: "group-[.is-published]:block",
        area: "arbitrary-group-peer-selector-stacking",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits Tailwind v4 :is/:where wrappers for arbitrary group selector variants",
    },
    TailwindParityFixture {
        class_name: "group-[:nth-of-type(3)_&]:block",
        area: "arbitrary-group-peer-selector-stacking",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style preserves explicit & placement in arbitrary group selector variants",
    },
    TailwindParityFixture {
        class_name: "group-[&.foo,&.bar]:block",
        area: "arbitrary-group-peer-selector-stacking",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits Tailwind v4 inner :is(...) wrappers for arbitrary group selector lists",
    },
    TailwindParityFixture {
        class_name: "group-[&:is(.foo,.bar)]:block",
        area: "arbitrary-group-peer-selector-stacking",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style preserves selector-function commas separately from top-level arbitrary group selector lists",
    },
    TailwindParityFixture {
        class_name: "group-[.is-open]/card:block",
        area: "arbitrary-group-peer-selector-stacking",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits Tailwind v4 wrappers for named arbitrary group selector variants",
    },
    TailwindParityFixture {
        class_name: "peer-[.is-dirty]:block",
        area: "arbitrary-group-peer-selector-stacking",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits Tailwind v4 :is/:where wrappers for arbitrary peer selector variants",
    },
    TailwindParityFixture {
        class_name: "peer-[&.dirty,&.touched]:block",
        area: "arbitrary-group-peer-selector-stacking",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits Tailwind v4 inner :is(...) wrappers for arbitrary peer selector lists",
    },
    TailwindParityFixture {
        class_name: "peer-[:nth-of-type(3)_&]:block",
        area: "arbitrary-group-peer-selector-stacking",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style preserves explicit & placement in arbitrary peer selector variants",
    },
    TailwindParityFixture {
        class_name: "peer-[.is-dirty]:peer-required:block",
        area: "arbitrary-group-peer-selector-stacking",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style composes arbitrary peer selector variants with peer state variants",
    },
    TailwindParityFixture {
        class_name: "group-[.is-open]:[&.target]:opacity-100",
        area: "arbitrary-group-peer-selector-stacking",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style composes arbitrary group selector variants with arbitrary selector variants",
    },
    TailwindParityFixture {
        class_name: "not-[@media_print]:flex",
        area: "arbitrary-at-rules",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits Tailwind-style negated media wrappers for not-[...] arbitrary at-rule variants",
    },
    TailwindParityFixture {
        class_name: "not-[@media_not_print]:flex",
        area: "arbitrary-at-rules",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style cancels media not conditions for Tailwind-style not-[...] arbitrary at-rule variants",
    },
    TailwindParityFixture {
        class_name: "not-[@supports(display:grid)]:flex",
        area: "arbitrary-at-rules",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits Tailwind-style negated supports wrappers for not-[...] arbitrary at-rule variants",
    },
    TailwindParityFixture {
        class_name: "not-[@container_(width>=32rem)]:flex",
        area: "arbitrary-at-rules",
        expected_status: TailwindParityStatus::Supported,
        reason: "dx-style emits Tailwind-style negated container wrappers for not-[...] arbitrary at-rule variants",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parity_receipt_records_supported_and_unsupported_classes() {
        let receipt = build_tailwind_parity_receipt();

        assert_eq!(receipt.schema_version, TAILWIND_PARITY_RECEIPT_SCHEMA);
        assert_eq!(receipt.tailwind_baseline, TAILWIND_PARITY_BASELINE);
        assert!(receipt.supported_count() > 10);
        assert_eq!(receipt.unsupported_count(), 0);

        let supported = entry_for(&receipt, "p-4");
        assert_eq!(supported.status, TailwindParityStatus::Supported);
        assert!(
            supported
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("padding: calc(var(--spacing) * 4);"))
        );

        for (class_name, fragment) in [
            ("bg-mauve-500", "background-color: var(--color-mauve-500);"),
            ("bg-olive-500", "background-color: var(--color-olive-500);"),
            ("bg-mist-500", "background-color: var(--color-mist-500);"),
            ("bg-taupe-500", "background-color: var(--color-taupe-500);"),
            (
                "text-olive-600/75",
                "color: color-mix(in oklab, var(--color-olive-600) 75%, transparent);",
            ),
            ("border-mist-300", "border-color: var(--color-mist-300);"),
            (
                "ring-taupe-400/50",
                "--tw-ring-color: color-mix(in oklab, var(--color-taupe-400) 50%, transparent);",
            ),
            (
                "outline-mauve-700",
                "outline-color: var(--color-mauve-700);",
            ),
            (
                "decoration-olive-500",
                "text-decoration-color: var(--color-olive-500);",
            ),
            (
                "from-mist-500",
                "--tw-gradient-from: var(--color-mist-500);",
            ),
            (
                "via-taupe-500/40",
                "color-mix(in oklab, var(--color-taupe-500) 40%, transparent)",
            ),
            ("to-mauve-950", "--tw-gradient-to: var(--color-mauve-950);"),
            (
                "shadow-mauve-500",
                "--tw-shadow-color: color-mix(in oklab, var(--color-mauve-500) var(--tw-shadow-alpha), transparent);",
            ),
            (
                "drop-shadow-mauve-500/50",
                "--tw-drop-shadow-color: color-mix(in oklab, color-mix(in oklab, var(--color-mauve-500) 50%, transparent) var(--tw-drop-shadow-alpha), transparent);",
            ),
            (
                "inset-shadow-olive-500",
                "--tw-inset-shadow-color: color-mix(in oklab, var(--color-olive-500) var(--tw-inset-shadow-alpha), transparent);",
            ),
            (
                "inset-ring-mist-500/50",
                "--tw-inset-ring-color: color-mix(in oklab, var(--color-mist-500) 50%, transparent);",
            ),
            (
                "ring-offset-taupe-500/40",
                "--tw-ring-offset-color: color-mix(in oklab, var(--color-taupe-500) 40%, transparent);",
            ),
        ] {
            let entry = entry_for(&receipt, class_name);
            assert_eq!(entry.status, TailwindParityStatus::Supported);
            assert!(
                entry
                    .generated_css
                    .as_deref()
                    .is_some_and(|css| css.contains(fragment)),
                "{class_name} should emit {fragment}"
            );
        }

        let unknown_at_rule = entry_for(&receipt, "[@unknown_rule]:p-4");
        assert_eq!(unknown_at_rule.status, TailwindParityStatus::Supported);
        assert!(unknown_at_rule.generated_css.as_deref().is_some_and(|css| {
            css.contains("@unknown rule") && css.contains("padding: calc(var(--spacing) * 4);")
        }));

        for (class_name, fragments) in [
            ("[&.foo]:[&.bar]:flex", &[".foo.bar", "display: flex;"][..]),
            (
                "[&_p]:[&_.lead]:mt-4",
                &[" p .lead", "margin-top: calc(var(--spacing) * 4);"][..],
            ),
            (
                "not-[.is-open]:[&.dismissible]:opacity-100",
                &[":not(.is-open).dismissible", "opacity: 100%;"][..],
            ),
            (
                "[&.is-dragging]:active:cursor-grabbing",
                &[".is-dragging:active", "cursor: grabbing;"][..],
            ),
            (
                "[&.foo,&.bar]:[&>.item,&>[data-slot=control]]:opacity-100",
                &[
                    ".foo>.item",
                    ".foo>[data-slot=control]",
                    ".bar>.item",
                    ".bar>[data-slot=control]",
                    "opacity: 100%;",
                ][..],
            ),
        ] {
            let entry = entry_for(&receipt, class_name);
            assert_eq!(entry.status, TailwindParityStatus::Supported);
            let css = entry.generated_css.as_deref().unwrap_or_default();
            for fragment in fragments {
                assert!(
                    css.contains(fragment),
                    "{class_name} should emit composed selector fragment {fragment}: {css}"
                );
            }
            assert!(
                !css.contains("}\n\n."),
                "{class_name} should compose stacked selector wrappers into one block: {css}"
            );
        }

        for (class_name, fragments) in [
            (
                "group-[.is-published]:block",
                &[":is(:where(.group):is(.is-published) *)", "display: block;"][..],
            ),
            (
                "group-[:nth-of-type(3)_&]:block",
                &[":is(:nth-of-type(3) :where(.group) *)", "display: block;"][..],
            ),
            (
                "group-[&.foo,&.bar]:block",
                &[
                    ":is(:is(:where(.group).foo,:where(.group).bar) *)",
                    "display: block;",
                ][..],
            ),
            (
                "group-[&:is(.foo,.bar)]:block",
                &[":is(:where(.group):is(.foo,.bar) *)", "display: block;"][..],
            ),
            (
                "group-[.is-open]/card:block",
                &[
                    ":is(:where(.group\\/card):is(.is-open) *)",
                    "display: block;",
                ][..],
            ),
            (
                "peer-[.is-dirty]:block",
                &[":is(:where(.peer):is(.is-dirty) ~ *)", "display: block;"][..],
            ),
            (
                "peer-[&.dirty,&.touched]:block",
                &[
                    ":is(:is(:where(.peer).dirty,:where(.peer).touched) ~ *)",
                    "display: block;",
                ][..],
            ),
            (
                "peer-[:nth-of-type(3)_&]:block",
                &[":is(:nth-of-type(3) :where(.peer) ~ *)", "display: block;"][..],
            ),
            (
                "peer-[.is-dirty]:peer-required:block",
                &[
                    ":is(:where(.peer):is(.is-dirty) ~ *)",
                    ":is(:where(.peer):required ~ *)",
                    "display: block;",
                ][..],
            ),
            (
                "group-[.is-open]:[&.target]:opacity-100",
                &[
                    ":is(:where(.group):is(.is-open) *)",
                    ".target",
                    "opacity: 100%;",
                ][..],
            ),
        ] {
            let entry = entry_for(&receipt, class_name);
            assert_eq!(entry.status, TailwindParityStatus::Supported);
            let css = entry.generated_css.as_deref().unwrap_or_default();
            for fragment in fragments {
                assert!(
                    css.contains(fragment),
                    "{class_name} should emit Tailwind v4 arbitrary group/peer fragment {fragment}: {css}"
                );
            }
            assert!(
                !css.contains(".group.is-") && !css.contains(".peer.is-"),
                "{class_name} should not emit the old pre-v4 group/peer selector shape: {css}"
            );
        }

        for (class_name, fragments) in [
            (
                "not-[@media_print]:flex",
                &["@media not print", "display: flex;"][..],
            ),
            (
                "not-[@media_not_print]:flex",
                &["@media print", "display: flex;"][..],
            ),
            (
                "not-[@supports(display:grid)]:flex",
                &["@supports not (display:grid)", "display: flex;"][..],
            ),
            (
                "not-[@container_(width>=32rem)]:flex",
                &["@container not (width>=32rem)", "display: flex;"][..],
            ),
        ] {
            let entry = entry_for(&receipt, class_name);
            assert_eq!(entry.status, TailwindParityStatus::Supported);
            let css = entry.generated_css.as_deref().unwrap_or_default();
            for fragment in fragments {
                assert!(
                    css.contains(fragment),
                    "{class_name} should emit {fragment}: {css}"
                );
            }
            assert!(
                !css.contains(":not(@"),
                "{class_name} should not treat at-rules as selector conditions: {css}"
            );
        }

        for (class_name, fragments) in [
            (
                "group-odd:bg-mauve-500",
                &[
                    ":is(:where(.group):nth-child(odd) *)",
                    "background-color: var(--color-mauve-500);",
                ][..],
            ),
            (
                "group-disabled:opacity-100",
                &[":is(:where(.group):disabled *)", "opacity: 100%;"][..],
            ),
            (
                "group-focus-visible/card:opacity-100",
                &[
                    ":is(:where(.group\\/card):focus-visible *)",
                    "opacity: 100%;",
                ][..],
            ),
            (
                "peer-invalid:visible",
                &[":is(:where(.peer):invalid ~ *)", "visibility: visible;"][..],
            ),
            (
                "peer-required/email:block",
                &[":is(:where(.peer\\/email):required ~ *)", "display: block;"][..],
            ),
            (
                "peer-disabled:opacity-100",
                &[":is(:where(.peer):disabled ~ *)", "opacity: 100%;"][..],
            ),
        ] {
            let entry = entry_for(&receipt, class_name);
            assert_eq!(entry.status, TailwindParityStatus::Supported);
            for fragment in fragments {
                assert!(
                    entry
                        .generated_css
                        .as_deref()
                        .is_some_and(|css| css.contains(fragment)),
                    "{class_name} should emit {fragment}"
                );
            }
        }

        let surface_token = entry_for(&receipt, "bg-token(surface)");
        assert_eq!(surface_token.status, TailwindParityStatus::Supported);
        assert!(
            surface_token
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("background-color: hsl(var(--surface));"))
        );

        let foreground_token = entry_for(&receipt, "text-token(foreground)");
        assert_eq!(foreground_token.status, TailwindParityStatus::Supported);
        assert!(
            foreground_token
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("color: hsl(var(--foreground));"))
        );

        let border_token = entry_for(&receipt, "border-token(border)");
        assert_eq!(border_token.status, TailwindParityStatus::Supported);
        assert!(
            border_token
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("border-color: hsl(var(--border));"))
        );

        let ring_token = entry_for(&receipt, "ring-token(ring)");
        assert_eq!(ring_token.status, TailwindParityStatus::Supported);
        assert!(
            ring_token
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("--tw-ring-color: hsl(var(--ring));"))
        );

        let layer = entry_for(&receipt, "[@layer_components]:p-4");
        assert_eq!(layer.status, TailwindParityStatus::Supported);
        assert!(layer.generated_css.as_deref().is_some_and(|css| {
            css.contains("@layer components") && css.contains("padding: calc(var(--spacing) * 4);")
        }));

        let nested_at_rule = entry_for(&receipt, "[@media_(any-hover:hover){&:hover}]:opacity-100");
        assert_eq!(nested_at_rule.status, TailwindParityStatus::Supported);
        assert!(nested_at_rule.generated_css.as_deref().is_some_and(|css| {
            css.contains("@media (any-hover:hover)")
                && css.contains(":hover")
                && css.contains("opacity: 100%;")
        }));

        let prose = entry_for(&receipt, "prose");
        assert_eq!(prose.status, TailwindParityStatus::Supported);
        assert!(
            prose
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains(".prose :where(h1)"))
        );

        let text_shadow = entry_for(&receipt, "text-shadow-sm");
        assert_eq!(text_shadow.status, TailwindParityStatus::Supported);
        assert!(
            text_shadow
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("text-shadow:"))
        );

        let text_shadow_color = entry_for(&receipt, "text-shadow-cyan-500/50");
        assert_eq!(text_shadow_color.status, TailwindParityStatus::Supported);
        assert!(
            text_shadow_color
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("--tw-text-shadow-color: rgb(6 182 212 / 0.5);"))
        );

        let stacked_text_shadow_color =
            entry_for(&receipt, "hover:not-focus:text-shadow-sky-300/50");
        assert_eq!(
            stacked_text_shadow_color.status,
            TailwindParityStatus::Supported
        );
        assert!(
            stacked_text_shadow_color
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains(":hover:not(*:focus)")
                    && css.contains("--tw-text-shadow-color:"))
        );

        let transform_gpu = entry_for(&receipt, "transform-gpu");
        assert_eq!(transform_gpu.status, TailwindParityStatus::Supported);
        assert!(
            transform_gpu
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("transform: translateZ(0)"))
        );

        let hyphens_auto = entry_for(&receipt, "hyphens-auto");
        assert_eq!(hyphens_auto.status, TailwindParityStatus::Supported);
        assert!(hyphens_auto.generated_css.as_deref().is_some_and(|css| {
            css.contains("-webkit-hyphens: auto") && css.contains("hyphens: auto;")
        }));

        let backface_hidden = entry_for(&receipt, "backface-hidden");
        assert_eq!(backface_hidden.status, TailwindParityStatus::Supported);
        assert!(backface_hidden.generated_css.as_deref().is_some_and(|css| {
            css.contains("-webkit-backface-visibility: hidden")
                && css.contains("backface-visibility: hidden;")
        }));

        let break_inside = entry_for(&receipt, "break-inside-avoid");
        assert_eq!(break_inside.status, TailwindParityStatus::Supported);
        assert!(break_inside.generated_css.as_deref().is_some_and(|css| {
            css.contains("page-break-inside: avoid;") && css.contains("break-inside: avoid;")
        }));

        let file_button = entry_for(&receipt, "file:p-4");
        assert_eq!(file_button.status, TailwindParityStatus::Supported);
        assert!(file_button.generated_css.as_deref().is_some_and(|css| {
            css.contains("::file-selector-button")
                && !css.contains("::-webkit-file-upload-button")
                && css.contains("padding: calc(var(--spacing) * 4);")
        }));

        let field_sizing = entry_for(&receipt, "field-sizing-content");
        assert_eq!(field_sizing.status, TailwindParityStatus::Supported);
        assert!(
            field_sizing
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("field-sizing: content"))
        );

        let scheme = entry_for(&receipt, "scheme-light-dark");
        assert_eq!(scheme.status, TailwindParityStatus::Supported);
        assert!(
            scheme
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("color-scheme: light dark;"))
        );

        let scrollbar_thumb = entry_for(&receipt, "scrollbar-thumb-red-500");
        assert_eq!(scrollbar_thumb.status, TailwindParityStatus::Supported);
        assert!(
            scrollbar_thumb
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("--tw-scrollbar-thumb: var(--color-red-500);"))
        );

        let zoom = entry_for(&receipt, "zoom-125");
        assert_eq!(zoom.status, TailwindParityStatus::Supported);
        assert!(
            zoom.generated_css
                .as_deref()
                .is_some_and(|css| css.contains("zoom: 125%;"))
        );

        let tab_size = entry_for(&receipt, "tab-4");
        assert_eq!(tab_size.status, TailwindParityStatus::Supported);
        assert!(
            tab_size
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("tab-size: 4;"))
        );

        let container = entry_for(&receipt, "@container");
        assert_eq!(container.status, TailwindParityStatus::Supported);
        assert!(
            container
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("container-type: inline-size;"))
        );

        let named_container = entry_for(&receipt, "@container/sidebar");
        assert_eq!(named_container.status, TailwindParityStatus::Supported);
        assert!(named_container.generated_css.as_deref().is_some_and(|css| {
            css.contains("container-type: inline-size;") && css.contains("container-name: sidebar;")
        }));

        let container_normal = entry_for(&receipt, "@container-normal");
        assert_eq!(container_normal.status, TailwindParityStatus::Supported);
        assert!(
            container_normal
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("container-type: normal;"))
        );

        let named_container_normal = entry_for(&receipt, "@container-normal/sidebar");
        assert_eq!(
            named_container_normal.status,
            TailwindParityStatus::Supported
        );
        assert!(
            named_container_normal
                .generated_css
                .as_deref()
                .is_some_and(|css| {
                    css.contains("container-type: normal;")
                        && css.contains("container-name: sidebar;")
                })
        );

        let container_size = entry_for(&receipt, "@container-size");
        assert_eq!(container_size.status, TailwindParityStatus::Supported);
        assert!(
            container_size
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("container-type: size;"))
        );

        let named_container_size = entry_for(&receipt, "@container-size/main");
        assert_eq!(named_container_size.status, TailwindParityStatus::Supported);
        assert!(
            named_container_size
                .generated_css
                .as_deref()
                .is_some_and(|css| {
                    css.contains("container-type: size;") && css.contains("container-name: main;")
                })
        );

        let named_container_size_sidebar = entry_for(&receipt, "@container-size/sidebar");
        assert_eq!(
            named_container_size_sidebar.status,
            TailwindParityStatus::Supported
        );
        assert!(
            named_container_size_sidebar
                .generated_css
                .as_deref()
                .is_some_and(|css| {
                    css.contains("container-type: size;")
                        && css.contains("container-name: sidebar;")
                })
        );

        let wrap_anywhere = entry_for(&receipt, "wrap-anywhere");
        assert_eq!(wrap_anywhere.status, TailwindParityStatus::Supported);
        assert!(
            wrap_anywhere
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("overflow-wrap: anywhere;"))
        );

        let indent = entry_for(&receipt, "indent-8");
        assert_eq!(indent.status, TailwindParityStatus::Supported);
        assert!(
            indent
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("text-indent: calc(var(--spacing) * 8);"))
        );

        let align_middle = entry_for(&receipt, "align-middle");
        assert_eq!(align_middle.status, TailwindParityStatus::Supported);
        assert!(
            align_middle
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("vertical-align: middle;"))
        );

        let decoration_thickness = entry_for(&receipt, "decoration-4");
        assert_eq!(decoration_thickness.status, TailwindParityStatus::Supported);
        assert!(
            decoration_thickness
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("text-decoration-thickness: 4px;"))
        );

        let underline_offset = entry_for(&receipt, "underline-offset-4");
        assert_eq!(underline_offset.status, TailwindParityStatus::Supported);
        assert!(
            underline_offset
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("text-underline-offset: 4px;"))
        );

        let mask_radial = entry_for(&receipt, "mask-radial-from-50%");
        assert_eq!(mask_radial.status, TailwindParityStatus::Supported);
        assert!(
            mask_radial
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("-webkit-mask-image:"))
        );

        let mask_conic = entry_for(&receipt, "mask-conic-from-50%");
        assert_eq!(mask_conic.status, TailwindParityStatus::Supported);
        assert!(
            mask_conic
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("conic-gradient("))
        );

        let mask_linear_edge = entry_for(&receipt, "mask-l-from-50%");
        assert_eq!(mask_linear_edge.status, TailwindParityStatus::Supported);
        assert!(
            mask_linear_edge
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("linear-gradient(to left"))
        );

        let mask_linear_stop = entry_for(&receipt, "mask-linear-from-60%");
        assert_eq!(mask_linear_stop.status, TailwindParityStatus::Supported);
        assert!(
            mask_linear_stop
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("--tw-mask-linear-from: 60%;"))
        );

        let mask_linear_arbitrary = entry_for(
            &receipt,
            "mask-linear-[70deg,transparent_10%,black,transparent_80%]",
        );
        assert_eq!(
            mask_linear_arbitrary.status,
            TailwindParityStatus::Supported
        );
        assert!(
            mask_linear_arbitrary
                .generated_css
                .as_deref()
                .is_some_and(|css| css
                    .contains("linear-gradient(70deg,transparent 10%,black,transparent 80%)"))
        );

        let mask_radial_arbitrary = entry_for(&receipt, "mask-radial-[100%_100%]");
        assert_eq!(
            mask_radial_arbitrary.status,
            TailwindParityStatus::Supported
        );
        assert!(
            mask_radial_arbitrary
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("--tw-mask-radial-size: 100% 100%;"))
        );

        let mask_mode = entry_for(&receipt, "mask-alpha");
        assert_eq!(mask_mode.status, TailwindParityStatus::Supported);
        assert!(
            mask_mode
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("mask-mode: alpha;"))
        );

        let mask_origin = entry_for(&receipt, "mask-origin-content");
        assert_eq!(mask_origin.status, TailwindParityStatus::Supported);
        assert!(
            mask_origin
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("mask-origin: content-box;"))
        );

        let mask_type = entry_for(&receipt, "mask-type-alpha");
        assert_eq!(mask_type.status, TailwindParityStatus::Supported);
        assert!(
            mask_type
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("mask-type: alpha;"))
        );

        let font_stretch = entry_for(&receipt, "font-stretch-condensed");
        assert_eq!(font_stretch.status, TailwindParityStatus::Supported);
        assert!(
            font_stretch
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("font-stretch: condensed;"))
        );

        let font_features = entry_for(&receipt, "font-features-['smcp','onum']");
        assert_eq!(font_features.status, TailwindParityStatus::Supported);
        assert!(
            font_features
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("font-feature-settings: 'smcp','onum';"))
        );

        let font_features_var = entry_for(&receipt, "font-features-(--dx-font-features)");
        assert_eq!(font_features_var.status, TailwindParityStatus::Supported);
        assert!(
            font_features_var
                .generated_css
                .as_deref()
                .is_some_and(|css| {
                    css.contains("font-feature-settings: var(--dx-font-features);")
                })
        );

        let tabular_nums = entry_for(&receipt, "tabular-nums");
        assert_eq!(tabular_nums.status, TailwindParityStatus::Supported);
        assert!(tabular_nums.generated_css.as_deref().is_some_and(|css| {
            css.contains("--tw-numeric-spacing: tabular-nums;")
                && css.contains("font-variant-numeric:")
        }));

        let ordinal = entry_for(&receipt, "ordinal");
        assert_eq!(ordinal.status, TailwindParityStatus::Supported);
        assert!(ordinal.generated_css.as_deref().is_some_and(|css| {
            css.contains("--tw-ordinal: ordinal;") && css.contains("font-variant-numeric:")
        }));

        let forced_color_adjust = entry_for(&receipt, "forced-color-adjust-auto");
        assert_eq!(forced_color_adjust.status, TailwindParityStatus::Supported);
        assert!(
            forced_color_adjust
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("forced-color-adjust: auto;"))
        );

        let outline_width = entry_for(&receipt, "outline-2");
        assert_eq!(outline_width.status, TailwindParityStatus::Supported);
        assert!(
            outline_width
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("outline-width: 2px;"))
        );

        let ring_inset = entry_for(&receipt, "ring-inset");
        assert_eq!(ring_inset.status, TailwindParityStatus::Supported);
        assert!(
            ring_inset
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("--tw-ring-inset: inset;"))
        );

        let ring_offset = entry_for(&receipt, "ring-offset-2");
        assert_eq!(ring_offset.status, TailwindParityStatus::Supported);
        assert!(
            ring_offset
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("--tw-ring-offset-width: 2px;"))
        );

        let touch_pan_left = entry_for(&receipt, "touch-pan-left");
        assert_eq!(touch_pan_left.status, TailwindParityStatus::Supported);
        assert!(touch_pan_left.generated_css.as_deref().is_some_and(|css| {
            css.contains("--tw-pan-x: pan-left;")
                && css.contains(
                    "touch-action: var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,);",
                )
        }));

        let touch_pinch_zoom = entry_for(&receipt, "touch-pinch-zoom");
        assert_eq!(touch_pinch_zoom.status, TailwindParityStatus::Supported);
        assert!(
            touch_pinch_zoom
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("--tw-pinch-zoom: pinch-zoom;")
                    && css.contains(
                        "touch-action: var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,);"
                    ))
        );

        let columns = entry_for(&receipt, "columns-3");
        assert_eq!(columns.status, TailwindParityStatus::Supported);
        assert!(
            columns
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("columns: 3;"))
        );

        let break_before = entry_for(&receipt, "break-before-page");
        assert_eq!(break_before.status, TailwindParityStatus::Supported);
        assert!(
            break_before
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("break-before: page;"))
        );

        let box_decoration = entry_for(&receipt, "box-decoration-clone");
        assert_eq!(box_decoration.status, TailwindParityStatus::Supported);
        assert!(
            box_decoration
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("box-decoration-break: clone;"))
        );

        let blend = entry_for(&receipt, "bg-blend-multiply");
        assert_eq!(blend.status, TailwindParityStatus::Supported);
        assert!(
            blend
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("background-blend-mode: multiply;"))
        );

        let background_origin = entry_for(&receipt, "bg-origin-border");
        assert_eq!(background_origin.status, TailwindParityStatus::Supported);
        assert!(
            background_origin
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("background-origin: border-box;"))
        );

        let background_none = entry_for(&receipt, "bg-none");
        assert_eq!(background_none.status, TailwindParityStatus::Supported);
        assert!(
            background_none
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("background-image: none;"))
        );

        let linear_background = entry_for(&receipt, "bg-linear-to-r");
        assert_eq!(linear_background.status, TailwindParityStatus::Supported);
        assert!(
            linear_background.generated_css.as_deref().is_some_and(
                |css| css.contains("linear-gradient(to right, var(--tw-gradient-stops));")
            )
        );

        let linear_oklch_background = entry_for(&receipt, "bg-linear-to-r/oklch");
        assert_eq!(
            linear_oklch_background.status,
            TailwindParityStatus::Supported
        );
        assert!(
            linear_oklch_background
                .generated_css
                .as_deref()
                .is_some_and(|css| css
                    .contains("linear-gradient(to right in oklch, var(--tw-gradient-stops));"))
        );

        let linear_longer_background = entry_for(&receipt, "bg-linear-to-r/longer");
        assert_eq!(
            linear_longer_background.status,
            TailwindParityStatus::Supported
        );
        assert!(
            linear_longer_background
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains(
                    "linear-gradient(to right in oklch longer hue, var(--tw-gradient-stops));"
                ))
        );

        let radial_background = entry_for(&receipt, "bg-radial");
        assert_eq!(radial_background.status, TailwindParityStatus::Supported);
        assert!(
            radial_background
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("radial-gradient(var(--tw-gradient-stops));"))
        );

        let radial_position_background = entry_for(&receipt, "bg-radial-[circle_at_center]");
        assert_eq!(
            radial_position_background.status,
            TailwindParityStatus::Supported
        );
        assert!(
            radial_position_background
                .generated_css
                .as_deref()
                .is_some_and(|css| css
                    .contains("radial-gradient(circle at center, var(--tw-gradient-stops));"))
        );

        let radial_custom_background = entry_for(&receipt, "bg-radial-(--dx-bg-radial)");
        assert_eq!(
            radial_custom_background.status,
            TailwindParityStatus::Supported
        );
        assert!(
            radial_custom_background
                .generated_css
                .as_deref()
                .is_some_and(|css| css
                    .contains("radial-gradient(var(--dx-bg-radial), var(--tw-gradient-stops));"))
        );

        let conic_plain_background = entry_for(&receipt, "bg-conic");
        assert_eq!(
            conic_plain_background.status,
            TailwindParityStatus::Supported
        );
        assert!(
            conic_plain_background
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("conic-gradient(var(--tw-gradient-stops));"))
        );

        let conic_decreasing_background = entry_for(&receipt, "bg-conic/decreasing");
        assert_eq!(
            conic_decreasing_background.status,
            TailwindParityStatus::Supported
        );
        assert!(
            conic_decreasing_background
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains(
                    "conic-gradient(in oklch decreasing hue, var(--tw-gradient-stops));"
                ))
        );

        let conic_background = entry_for(&receipt, "bg-conic-180");
        assert_eq!(conic_background.status, TailwindParityStatus::Supported);
        assert!(
            conic_background.generated_css.as_deref().is_some_and(
                |css| css.contains("conic-gradient(from 180deg, var(--tw-gradient-stops));")
            )
        );

        let conic_shorter_background = entry_for(&receipt, "bg-conic-180/shorter");
        assert_eq!(
            conic_shorter_background.status,
            TailwindParityStatus::Supported
        );
        assert!(
            conic_shorter_background
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains(
                    "conic-gradient(from 180deg in oklch shorter hue, var(--tw-gradient-stops));"
                ))
        );

        let conic_negative_background = entry_for(&receipt, "-bg-conic-45");
        assert_eq!(
            conic_negative_background.status,
            TailwindParityStatus::Supported
        );
        assert!(
            conic_negative_background
                .generated_css
                .as_deref()
                .is_some_and(
                    |css| css.contains("conic-gradient(from -45deg, var(--tw-gradient-stops));")
                )
        );

        let conic_arbitrary_background = entry_for(&receipt, "bg-conic-[from_45deg_at_center]");
        assert_eq!(
            conic_arbitrary_background.status,
            TailwindParityStatus::Supported
        );
        assert!(
            conic_arbitrary_background
                .generated_css
                .as_deref()
                .is_some_and(|css| css
                    .contains("conic-gradient(from 45deg at center, var(--tw-gradient-stops));"))
        );

        let conic_arbitrary_interpolation = entry_for(&receipt, "bg-conic/[in_hsl_longer_hue]");
        assert_eq!(
            conic_arbitrary_interpolation.status,
            TailwindParityStatus::Supported
        );
        assert!(
            conic_arbitrary_interpolation
                .generated_css
                .as_deref()
                .is_some_and(|css| css
                    .contains("conic-gradient(in hsl longer hue, var(--tw-gradient-stops));"))
        );

        let conic_custom_background = entry_for(&receipt, "bg-conic-(--dx-bg-conic)");
        assert_eq!(
            conic_custom_background.status,
            TailwindParityStatus::Supported
        );
        assert!(
            conic_custom_background
                .generated_css
                .as_deref()
                .is_some_and(|css| css
                    .contains("conic-gradient(var(--dx-bg-conic), var(--tw-gradient-stops));"))
        );

        let linear_angle_background = entry_for(&receipt, "bg-linear-45");
        assert_eq!(
            linear_angle_background.status,
            TailwindParityStatus::Supported
        );
        assert!(
            linear_angle_background
                .generated_css
                .as_deref()
                .is_some_and(
                    |css| css.contains("linear-gradient(45deg, var(--tw-gradient-stops));")
                )
        );

        let arbitrary_image_background = entry_for(&receipt, "bg-[url('/hero.png')]");
        assert_eq!(
            arbitrary_image_background.status,
            TailwindParityStatus::Supported
        );
        assert!(
            arbitrary_image_background
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("background-image: url('/hero.png');"))
        );

        let background_size = entry_for(&receipt, "bg-size-(--dx-bg-size)");
        assert_eq!(background_size.status, TailwindParityStatus::Supported);
        assert!(
            background_size
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("background-size: var(--dx-bg-size);"))
        );

        let appearance = entry_for(&receipt, "appearance-none");
        assert_eq!(appearance.status, TailwindParityStatus::Supported);
        assert!(
            appearance
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("-webkit-appearance: none;"))
        );

        let select = entry_for(&receipt, "select-none");
        assert_eq!(select.status, TailwindParityStatus::Supported);
        assert!(
            select
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("-webkit-user-select: none;"))
        );

        let backface = entry_for(&receipt, "backface-hidden");
        assert_eq!(backface.status, TailwindParityStatus::Supported);
        assert!(backface.generated_css.as_deref().is_some_and(|css| {
            css.contains("-webkit-backface-visibility: hidden;")
                && css.contains("backface-visibility: hidden;")
        }));

        let break_inside = entry_for(&receipt, "break-inside-avoid");
        assert_eq!(break_inside.status, TailwindParityStatus::Supported);
        assert!(break_inside.generated_css.as_deref().is_some_and(|css| {
            css.contains("page-break-inside: avoid;") && css.contains("break-inside: avoid;")
        }));

        let backdrop = entry_for(&receipt, "backdrop-blur-md");
        assert_eq!(backdrop.status, TailwindParityStatus::Supported);
        assert!(
            backdrop
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("-webkit-backdrop-filter:"))
        );

        let starting_style = entry_for(&receipt, "[@starting-style]:opacity-0");
        assert_eq!(starting_style.status, TailwindParityStatus::Supported);
        assert!(
            starting_style
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("@starting-style") && css.contains("opacity: 0%;"))
        );
    }

    #[test]
    fn parity_receipt_keeps_intentional_differences_visible() {
        let receipt = build_tailwind_parity_receipt();
        let container = entry_for(&receipt, "container");

        assert_eq!(
            container.status,
            TailwindParityStatus::IntentionallyDifferent
        );
        assert!(
            container
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains("width: 100%;"))
        );
        assert_eq!(receipt.intentionally_different_count(), 1);
    }

    #[test]
    fn parity_receipt_records_common_state_alias_generated_css() {
        let receipt = build_tailwind_parity_receipt();

        let target = entry_for(&receipt, "target:p-4");
        assert_eq!(target.status, TailwindParityStatus::Supported);
        assert!(target.generated_css.as_deref().is_some_and(|css| {
            css.contains(":target") && css.contains("padding: calc(var(--spacing) * 4);")
        }));

        let read_only = entry_for(&receipt, "read-only:bg-blue-500");
        assert_eq!(read_only.status, TailwindParityStatus::Supported);
        assert!(read_only.generated_css.as_deref().is_some_and(|css| {
            css.contains(":read-only") && css.contains("background-color: rgb(59 130 246);")
        }));

        let indeterminate = entry_for(&receipt, "indeterminate:opacity-100");
        assert_eq!(indeterminate.status, TailwindParityStatus::Supported);
        assert!(indeterminate.generated_css.as_deref().is_some_and(|css| {
            css.contains(":indeterminate") && css.contains("opacity: 100%;")
        }));

        let user_valid = entry_for(&receipt, "user-valid:border-green-500");
        assert_eq!(user_valid.status, TailwindParityStatus::Supported);
        assert!(
            user_valid
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains(":user-valid") && css.contains("border-color:"))
        );

        let user_invalid = entry_for(&receipt, "user-invalid:border-red-500");
        assert_eq!(user_invalid.status, TailwindParityStatus::Supported);
        assert!(
            user_invalid.generated_css.as_deref().is_some_and(|css| {
                css.contains(":user-invalid") && css.contains("border-color:")
            })
        );

        let details_content = entry_for(&receipt, "details-content:bg-slate-100");
        assert_eq!(details_content.status, TailwindParityStatus::Supported);
        assert!(details_content.generated_css.as_deref().is_some_and(|css| {
            css.contains(":details-content") && css.contains("background-color:")
        }));

        let rtl = entry_for(&receipt, "rtl:ps-4");
        assert_eq!(rtl.status, TailwindParityStatus::Supported);
        assert!(rtl.generated_css.as_deref().is_some_and(|css| {
            css.contains(":where(:dir(rtl), [dir=\"rtl\"], [dir=\"rtl\"] *)")
                && css.contains("padding-inline-start: calc(var(--spacing) * 4);")
        }));

        let ltr = entry_for(&receipt, "ltr:pe-4");
        assert_eq!(ltr.status, TailwindParityStatus::Supported);
        assert!(ltr.generated_css.as_deref().is_some_and(|css| {
            css.contains(":where(:dir(ltr), [dir=\"ltr\"], [dir=\"ltr\"] *)")
                && css.contains("padding-inline-end: calc(var(--spacing) * 4);")
        }));

        let inert = entry_for(&receipt, "inert:opacity-50");
        assert_eq!(inert.status, TailwindParityStatus::Supported);
        assert!(inert.generated_css.as_deref().is_some_and(|css| {
            css.contains(":is([inert], [inert] *)") && css.contains("opacity: 50%;")
        }));

        let open = entry_for(&receipt, "open:bg-blue-500");
        assert_eq!(open.status, TailwindParityStatus::Supported);
        assert!(open.generated_css.as_deref().is_some_and(|css| {
            css.contains(":is([open], :popover-open, :open)") && css.contains("background-color:")
        }));

        let starting_open = entry_for(&receipt, "starting:open:opacity-0");
        assert_eq!(starting_open.status, TailwindParityStatus::Supported);
        assert!(starting_open.generated_css.as_deref().is_some_and(|css| {
            css.contains("@starting-style")
                && css.contains(":is([open], :popover-open, :open)")
                && css.contains("opacity: 0%;")
        }));

        let backdrop = entry_for(&receipt, "backdrop:bg-slate-950/50");
        assert_eq!(backdrop.status, TailwindParityStatus::Supported);
        assert!(backdrop.generated_css.as_deref().is_some_and(|css| {
            css.contains("::backdrop") && css.contains("background-color:")
        }));

        let has_even = entry_for(&receipt, "has-even:bg-blue-500");
        assert_eq!(has_even.status, TailwindParityStatus::Supported);
        assert!(
            has_even
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains(":has(*:nth-child(even))"))
        );

        let not_visited = entry_for(&receipt, "not-visited:text-slate-900");
        assert_eq!(not_visited.status, TailwindParityStatus::Supported);
        assert!(
            not_visited
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains(":not(*:visited)"))
        );

        let in_read_only = entry_for(&receipt, "in-read-only:p-4");
        assert_eq!(in_read_only.status, TailwindParityStatus::Supported);
        assert!(
            in_read_only
                .generated_css
                .as_deref()
                .is_some_and(|css| css.contains(":where(*:read-only)"))
        );
    }

    #[test]
    fn parity_receipt_records_tailwind_v4_container_query_ladder() {
        let receipt = build_tailwind_parity_receipt();

        for (class_name, fragments) in [
            (
                "@3xs:grid",
                &["@container (width >= 16rem)", "display: grid;"][..],
            ),
            (
                "@2xs:flex",
                &["@container (width >= 18rem)", "display: flex;"][..],
            ),
            (
                "@xs:block",
                &["@container (width >= 20rem)", "display: block;"][..],
            ),
            (
                "@sm:grid",
                &["@container (width >= 24rem)", "display: grid;"][..],
            ),
            (
                "@md:flex",
                &["@container (width >= 28rem)", "display: flex;"][..],
            ),
            (
                "@lg:block",
                &["@container (width >= 32rem)", "display: block;"][..],
            ),
            (
                "@xl:grid",
                &["@container (width >= 36rem)", "display: grid;"][..],
            ),
            (
                "@2xl:flex",
                &["@container (width >= 42rem)", "display: flex;"][..],
            ),
            (
                "@3xl/block:flex",
                &["@container block (width >= 48rem)", "display: flex;"][..],
            ),
            (
                "@4xl:grid",
                &["@container (width >= 56rem)", "display: grid;"][..],
            ),
            (
                "@5xl:block",
                &["@container (width >= 64rem)", "display: block;"][..],
            ),
            (
                "@6xl:flex",
                &["@container (width >= 72rem)", "display: flex;"][..],
            ),
            (
                "@7xl:flex",
                &["@container (width >= 80rem)", "display: flex;"][..],
            ),
            (
                "@max-3xs:hidden",
                &["@container (width < 16rem)", "display: none;"][..],
            ),
            (
                "@max-2xs:block",
                &["@container (width < 18rem)", "display: block;"][..],
            ),
            (
                "@max-xs:flex",
                &["@container (width < 20rem)", "display: flex;"][..],
            ),
            (
                "@max-sm:hidden",
                &["@container (width < 24rem)", "display: none;"][..],
            ),
            (
                "@max-md:block",
                &["@container (width < 28rem)", "display: block;"][..],
            ),
            (
                "@max-lg:flex",
                &["@container (width < 32rem)", "display: flex;"][..],
            ),
            (
                "@max-xl:hidden",
                &["@container (width < 36rem)", "display: none;"][..],
            ),
            (
                "@max-2xl:block",
                &["@container (width < 42rem)", "display: block;"][..],
            ),
            (
                "@max-3xl:flex",
                &["@container (width < 48rem)", "display: flex;"][..],
            ),
            (
                "@max-4xl:hidden",
                &["@container (width < 56rem)", "display: none;"][..],
            ),
            (
                "@max-5xl:block",
                &["@container (width < 64rem)", "display: block;"][..],
            ),
            (
                "@max-6xl:flex",
                &["@container (width < 72rem)", "display: flex;"][..],
            ),
            (
                "@max-7xl:block",
                &["@container (width < 80rem)", "display: block;"][..],
            ),
            (
                "@3xl/main:opacity-100",
                &["@container main (width >= 48rem)", "opacity: 100%;"][..],
            ),
            (
                "@max-md:@sm:flex",
                &[
                    "@container (width < 28rem)",
                    "@container (width >= 24rem)",
                    "display: flex;",
                ][..],
            ),
            (
                "@sm:@max-md:flex",
                &[
                    "@container (width >= 24rem)",
                    "@container (width < 28rem)",
                    "display: flex;",
                ][..],
            ),
            (
                "@max-md/main:@sm/main:grid",
                &[
                    "@container main (width < 28rem)",
                    "@container main (width >= 24rem)",
                    "display: grid;",
                ][..],
            ),
            (
                "@max-[960px]:@min-[475px]:hidden",
                &[
                    "@container (width < 960px)",
                    "@container (width >= 475px)",
                    "display: none;",
                ][..],
            ),
            (
                "@min-[123px]:flex",
                &["@container (width >= 123px)", "display: flex;"][..],
            ),
            (
                "@max-[123px]:hidden",
                &["@container (width < 123px)", "display: none;"][..],
            ),
            (
                "@min-[456px]/name:grid",
                &["@container name (width >= 456px)", "display: grid;"][..],
            ),
            (
                "@max-[456px]/name:block",
                &["@container name (width < 456px)", "display: block;"][..],
            ),
            (
                "@max-[960px]/name:@min-[475px]/name:flex",
                &[
                    "@container name (width < 960px)",
                    "@container name (width >= 475px)",
                    "display: flex;",
                ][..],
            ),
            (
                "@[475px]:flex",
                &["@container (width >= 475px)", "display: flex;"][..],
            ),
            (
                "@[475px]/card:grid",
                &["@container card (width >= 475px)", "display: grid;"][..],
            ),
            (
                "@[475px]:@max-[960px]:block",
                &[
                    "@container (width >= 475px)",
                    "@container (width < 960px)",
                    "display: block;",
                ][..],
            ),
            (
                "@[475px]/card:@max-[960px]/card:hidden",
                &[
                    "@container card (width >= 475px)",
                    "@container card (width < 960px)",
                    "display: none;",
                ][..],
            ),
            (
                "@min-[40rem]:@max-[70rem]:flex",
                &[
                    "@container (width >= 40rem)",
                    "@container (width < 70rem)",
                    "display: flex;",
                ][..],
            ),
            (
                "@min-[40rem]/main:@max-[70rem]/main:grid",
                &[
                    "@container main (width >= 40rem)",
                    "@container main (width < 70rem)",
                    "display: grid;",
                ][..],
            ),
            (
                "@max-7xl:@3xs:flex",
                &[
                    "@container (width < 80rem)",
                    "@container (width >= 16rem)",
                    "display: flex;",
                ][..],
            ),
            (
                "@3xs:@max-7xl:flex",
                &[
                    "@container (width >= 16rem)",
                    "@container (width < 80rem)",
                    "display: flex;",
                ][..],
            ),
            (
                "@max-7xl/main:@3xs/main:grid",
                &[
                    "@container main (width < 80rem)",
                    "@container main (width >= 16rem)",
                    "display: grid;",
                ][..],
            ),
            (
                "@3xs:@md:@max-7xl:flex",
                &[
                    "@container (width >= 16rem)",
                    "@container (width >= 28rem)",
                    "@container (width < 80rem)",
                    "display: flex;",
                ][..],
            ),
            (
                "@max-7xl:@md:@3xs:grid",
                &[
                    "@container (width < 80rem)",
                    "@container (width >= 28rem)",
                    "@container (width >= 16rem)",
                    "display: grid;",
                ][..],
            ),
            (
                "@min-[30rem]/rail:@max-[50rem]/rail:block",
                &[
                    "@container rail (width >= 30rem)",
                    "@container rail (width < 50rem)",
                    "display: block;",
                ][..],
            ),
            (
                "@max-[50rem]/rail:@min-[30rem]/rail:hidden",
                &[
                    "@container rail (width < 50rem)",
                    "@container rail (width >= 30rem)",
                    "display: none;",
                ][..],
            ),
        ] {
            let entry = entry_for(&receipt, class_name);
            assert_eq!(entry.status, TailwindParityStatus::Supported);
            let css = entry.generated_css.as_deref().expect(class_name);

            for fragment in fragments {
                assert!(
                    css.contains(fragment),
                    "{class_name} missing {fragment:?}: {css}"
                );
            }
        }
    }

    #[test]
    fn parity_receipt_records_tailwind_v4_second_pass_utility_canaries() {
        let receipt = build_tailwind_parity_receipt();

        for (class_name, fragment) in [
            ("size-8", "width: calc(var(--spacing) * 8);"),
            ("w-1/2", "width: calc(1 / 2 * 100%);"),
            ("inline-1/2", "inline-size: calc(1 / 2 * 100%);"),
            ("inline-3xs", "inline-size: var(--container-3xs);"),
            ("min-inline-xl", "min-inline-size: var(--container-xl);"),
            ("px-4", "padding-inline: calc(var(--spacing) * 4);"),
            ("py-2", "padding-block: calc(var(--spacing) * 2);"),
            ("pbs-4", "padding-block-start: calc(var(--spacing) * 4);"),
            ("mx-auto", "margin-inline: auto;"),
            ("-mbs-2", "margin-block-start: calc(var(--spacing) * -2);"),
            ("space-x-4", "--tw-space-x-reverse: 0;"),
            (
                "space-x-4",
                "margin-inline-end: calc(calc(var(--spacing) * 4) * calc(1 - var(--tw-space-x-reverse)));",
            ),
            ("space-x-reverse", "--tw-space-x-reverse: 1;"),
            ("inset-s-4", "inset-inline-start: calc(var(--spacing) * 4);"),
            (
                "-inset-e-1/2",
                "inset-inline-end: calc(calc(1 / 2 * 100%) * -1);",
            ),
            ("border-x", "border-inline-style: var(--tw-border-style);"),
            ("border-x", "border-inline-width: 1px;"),
            ("border-y-4", "border-block-style: var(--tw-border-style);"),
            ("border-y-4", "border-block-width: 4px;"),
            ("border-s-2", "border-inline-start-width: 2px;"),
            (
                "border-s-red-500",
                "border-inline-start-color: var(--color-red-500);",
            ),
            (
                "border-bs-emerald-500",
                "border-block-start-color: var(--color-emerald-500);",
            ),
            ("rounded-ee-xl", "border-end-end-radius: var(--radius-xl);"),
            (
                "scroll-ms-4",
                "scroll-margin-inline-start: calc(var(--spacing) * 4);",
            ),
            (
                "scroll-mbs-6",
                "scroll-margin-block-start: calc(var(--spacing) * 6);",
            ),
            (
                "scroll-pe-2",
                "scroll-padding-inline-end: calc(var(--spacing) * 2);",
            ),
            (
                "scroll-pbe-2",
                "scroll-padding-block-end: calc(var(--spacing) * 2);",
            ),
            ("object-[25%_75%]", "object-position: 25% 75%;"),
            (
                "object-(--dx-object-position)",
                "object-position: var(--dx-object-position);",
            ),
            ("origin-top-left", "transform-origin: top left;"),
            ("skew-x-6", "--tw-skew-x: skewX(6deg);"),
            ("-skew-y-3", "--tw-skew-y: skewY(calc(3deg * -1));"),
            ("flex-[2_1_0%]", "flex: 2 1 0%;"),
            ("grow-[2]", "flex-grow: 2;"),
            ("shrink-[3]", "flex-shrink: 3;"),
            ("-order-1", "order: -1;"),
            (
                "grid-cols-(--dx-grid-cols)",
                "grid-template-columns: var(--dx-grid-cols);",
            ),
            (
                "auto-rows-(--dx-auto-rows)",
                "grid-auto-rows: var(--dx-auto-rows);",
            ),
            ("drop-shadow-md", "--tw-drop-shadow: drop-shadow("),
            (
                "backdrop-opacity-50",
                "--tw-backdrop-opacity: opacity(50%);",
            ),
            ("backdrop-invert", "--tw-backdrop-invert: invert(100%);"),
        ] {
            let entry = entry_for(&receipt, class_name);
            assert_eq!(entry.status, TailwindParityStatus::Supported);
            assert!(
                entry
                    .generated_css
                    .as_deref()
                    .is_some_and(|css| css.contains(fragment)),
                "{class_name} generated CSS did not contain {fragment:?}: {:?}",
                entry.generated_css
            );
        }
    }

    #[test]
    fn parity_receipt_records_tailwind_v4_third_pass_utility_canaries() {
        let receipt = build_tailwind_parity_receipt();

        for (class_name, fragment) in [
            ("table-auto", "table-layout: auto;"),
            ("table-fixed", "table-layout: fixed;"),
            ("caption-bottom", "caption-side: bottom;"),
            ("border-collapse", "border-collapse: collapse;"),
            (
                "border-spacing-2",
                "border-spacing: calc(var(--spacing) * 2);",
            ),
            (
                "border-spacing-x-4",
                "--tw-border-spacing-x: calc(var(--spacing) * 4);",
            ),
            ("transform-3d", "transform-style: preserve-3d;"),
            ("perspective-[750px]", "perspective: 750px;"),
            (
                "perspective-origin-top-right",
                "perspective-origin: top right;",
            ),
            ("rotate-x-45", "--tw-rotate-x: rotateX(45deg);"),
            ("-rotate-y-12", "--tw-rotate-y: rotateY(calc(12deg * -1));"),
            (
                "translate-z-4",
                "--tw-translate-z: calc(var(--spacing) * 4);",
            ),
            ("scale-z-125", "--tw-scale-z: 125%;"),
            (
                "inset-shadow-[inset_0_1px_2px_rgb(0_0_0_/_0.1)]",
                "box-shadow: inset 0 1px 2px rgb(0 0 0 / 0.1);",
            ),
            ("shadow-(--dx-shadow)", "box-shadow: var(--dx-shadow);"),
        ] {
            let entry = entry_for(&receipt, class_name);
            assert_eq!(entry.status, TailwindParityStatus::Supported);
            assert!(
                entry
                    .generated_css
                    .as_deref()
                    .is_some_and(|css| css.contains(fragment)),
                "{class_name} generated CSS did not contain {fragment:?}: {:?}",
                entry.generated_css
            );
        }
    }

    #[test]
    fn parity_receipt_records_tailwind_v4_fourth_pass_utility_canaries() {
        let receipt = build_tailwind_parity_receipt();

        for (class_name, fragment) in [
            ("box-border", "box-sizing: border-box;"),
            ("box-content", "box-sizing: content-box;"),
            (
                "bg-[#1d4ed8]/50",
                "background-color: color-mix(in oklab, #1d4ed8 50%, transparent);",
            ),
            (
                "bg-(--dx-background)",
                "background-color: var(--dx-background);",
            ),
            ("border-(--dx-border)", "border-color: var(--dx-border);"),
            (
                "fill-[#0f172a]/80",
                "fill: color-mix(in oklab, #0f172a 80%, transparent);",
            ),
            (
                "text-[color:var(--dx-foreground)]",
                "color: var(--dx-foreground);",
            ),
            (
                "stroke-[color:var(--dx-stroke)]",
                "stroke: var(--dx-stroke);",
            ),
            ("rounded-(--dx-radius)", "border-radius: var(--dx-radius);"),
            ("aspect-(--dx-aspect)", "aspect-ratio: var(--dx-aspect);"),
            ("leading-(--dx-leading)", "line-height: var(--dx-leading);"),
            (
                "tracking-(--dx-tracking)",
                "letter-spacing: var(--dx-tracking);",
            ),
        ] {
            let entry = entry_for(&receipt, class_name);
            assert_eq!(entry.status, TailwindParityStatus::Supported);
            assert!(
                entry
                    .generated_css
                    .as_deref()
                    .is_some_and(|css| css.contains(fragment)),
                "{class_name} generated CSS did not contain {fragment:?}: {:?}",
                entry.generated_css
            );
        }
    }

    #[test]
    fn parity_receipt_records_tailwind_v4_fifth_pass_utility_canaries() {
        let receipt = build_tailwind_parity_receipt();

        for (class_name, fragment) in [
            ("items-baseline-last", "align-items: last baseline;"),
            ("self-baseline", "align-self: baseline;"),
            ("place-items-baseline", "place-items: baseline;"),
            ("justify-items-center", "justify-items: center;"),
            ("justify-self-end", "justify-self: end;"),
            ("content-normal", "align-content: normal;"),
            ("from-10%", "--tw-gradient-from-position: 10%;"),
            ("via-30%", "--tw-gradient-via-position: 30%;"),
            ("to-90%", "--tw-gradient-to-position: 90%;"),
            (
                "from-(--dx-gradient-from-position)",
                "--tw-gradient-from-position: var(--dx-gradient-from-position);",
            ),
            (
                "ring-[#2563eb]/50",
                "--tw-ring-color: color-mix(in oklab, #2563eb 50%, transparent);",
            ),
            ("divide-(--dx-border)", "border-color: var(--dx-border);"),
        ] {
            let entry = entry_for(&receipt, class_name);
            assert_eq!(entry.status, TailwindParityStatus::Supported);
            assert!(
                entry
                    .generated_css
                    .as_deref()
                    .is_some_and(|css| css.contains(fragment)),
                "{class_name} generated CSS did not contain {fragment:?}: {:?}",
                entry.generated_css
            );
        }
    }

    #[test]
    fn parity_receipt_records_tailwind_v4_sixth_pass_utility_canaries() {
        let receipt = build_tailwind_parity_receipt();

        for (class_name, fragment) in [
            ("p-(--dx-pad)", "padding: var(--dx-pad);"),
            (
                "scroll-mt-(--dx-scroll-mt)",
                "scroll-margin-top: var(--dx-scroll-mt);",
            ),
            ("opacity-(--dx-opacity)", "opacity: var(--dx-opacity);"),
            ("z-(--dx-layer)", "z-index: var(--dx-layer);"),
            ("order-(--dx-order)", "order: var(--dx-order);"),
            ("blur-(--dx-blur)", "--tw-blur: blur(var(--dx-blur));"),
            (
                "brightness-(--dx-brightness)",
                "--tw-brightness: brightness(var(--dx-brightness));",
            ),
            (
                "hue-rotate-(--dx-hue-rotate)",
                "--tw-hue-rotate: hue-rotate(var(--dx-hue-rotate));",
            ),
            (
                "backdrop-opacity-(--dx-backdrop-opacity)",
                "--tw-backdrop-opacity: opacity(var(--dx-backdrop-opacity));",
            ),
            (
                "backdrop-blur-(--dx-backdrop-blur)",
                "--tw-backdrop-blur: blur(var(--dx-backdrop-blur));",
            ),
            (
                "outline-offset-(--dx-outline-offset)",
                "outline-offset: var(--dx-outline-offset);",
            ),
            ("outline-[3px]", "outline-width: 3px;"),
            ("ring-[3px]", "calc(3px + var(--tw-ring-offset-width, 0px))"),
            (
                "ring-offset-(--dx-ring-offset-width)",
                "--tw-ring-offset-width: var(--dx-ring-offset-width);",
            ),
            (
                "perspective-dramatic",
                "perspective: var(--perspective-dramatic);",
            ),
            (
                "transform-(--dx-transform)",
                "transform: var(--dx-transform);",
            ),
            ("scale-z-(--dx-scale-z)", "--tw-scale-z: var(--dx-scale-z);"),
            ("-scale-z-125", "--tw-scale-z: calc(125% * -1);"),
        ] {
            let entry = entry_for(&receipt, class_name);
            assert_eq!(entry.status, TailwindParityStatus::Supported);
            assert!(
                entry
                    .generated_css
                    .as_deref()
                    .is_some_and(|css| css.contains(fragment)),
                "{class_name} generated CSS did not contain {fragment:?}: {:?}",
                entry.generated_css
            );
        }
    }

    fn entry_for<'a>(
        receipt: &'a TailwindParityReceipt,
        class_name: &str,
    ) -> &'a TailwindParityEntry {
        receipt
            .entries
            .iter()
            .find(|entry| entry.class_name == class_name)
            .expect("fixture entry")
    }
}
