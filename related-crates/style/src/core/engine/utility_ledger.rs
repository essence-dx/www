//! Tailwind v4.3 utility docs-table ledger for dx-style.
//!
//! This ledger is proof inventory, not proof of complete utility parity. Each
//! row keeps at least one supported canary and at least one still-unproven
//! value/modifier family so future work starts from an executable gap.

use super::feature_matrix::TailwindV43FeatureStatus;

/// Stable schema for utility docs-table ledger consumers.
pub const TAILWIND_V43_UTILITY_LEDGER_SCHEMA: &str = "dx.style.tailwind-v43-utility-ledger";

/// Tailwind baseline verified for this utility ledger.
pub const TAILWIND_V43_UTILITY_LEDGER_BASELINE: &str = "tailwindcss-4.3.0";

/// Scope statement for public consumers.
pub const TAILWIND_V43_UTILITY_LEDGER_SCOPE: &str =
    "official Tailwind docs-table ledger; not full utility/value/modifier parity";

/// One Tailwind utility documentation area in the dx-style ledger.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
pub struct TailwindV43UtilityLedgerEntry {
    /// Stable area id used by tests and docs.
    pub docs_area: &'static str,
    /// Human docs table label.
    pub docs_table: &'static str,
    /// Current dx-style support state for this area.
    pub status: TailwindV43FeatureStatus,
    /// Representative supported dx-style canaries.
    pub representative_supported_canaries: &'static [&'static str],
    /// Utility/value/modifier proof still missing for this area.
    pub unproven_or_missing_canaries: &'static [&'static str],
    /// Whether this row proves every value and modifier in Tailwind v4.3.
    pub full_value_modifier_parity_proven: bool,
}

/// Return the source-owned Tailwind v4.3 utility docs-table ledger.
pub fn tailwind_v43_utility_ledger() -> &'static [TailwindV43UtilityLedgerEntry] {
    TAILWIND_V43_UTILITY_LEDGER
}

const TAILWIND_V43_UTILITY_LEDGER: &[TailwindV43UtilityLedgerEntry] = &[
    TailwindV43UtilityLedgerEntry {
        docs_area: "layout",
        docs_table: "Tailwind docs: Layout",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &["block", "columns-3", "aspect-video", "inset-s-4"],
        unproven_or_missing_canaries: &[
            "complete layout docs-table walk",
            "all display/position/object value aliases",
            "full arbitrary modifier sweep",
        ],
        full_value_modifier_parity_proven: false,
    },
    TailwindV43UtilityLedgerEntry {
        docs_area: "flexbox-grid",
        docs_table: "Tailwind docs: Flexbox & Grid",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &["flex", "grid-cols-3", "col-span-2"],
        unproven_or_missing_canaries: &[
            "complete flexbox and grid docs-table walk",
            "grid edge grammar and placement synthesis",
            "full subgrid/value/modifier parity",
        ],
        full_value_modifier_parity_proven: false,
    },
    TailwindV43UtilityLedgerEntry {
        docs_area: "spacing",
        docs_table: "Tailwind docs: Spacing",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &[
            "p-4",
            "px-4",
            "py-2",
            "mx-auto",
            "-mt-2",
            "space-x-4",
            "space-x-reverse",
            "pbs-4",
            "-mbs-2",
        ],
        unproven_or_missing_canaries: &[
            "complete spacing docs-table walk",
            "remaining arbitrary/custom negative spacing calc edge cases",
            "full logical spacing modifier sweep",
        ],
        full_value_modifier_parity_proven: false,
    },
    TailwindV43UtilityLedgerEntry {
        docs_area: "sizing",
        docs_table: "Tailwind docs: Sizing",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &[
            "w-1/2",
            "size-8",
            "inline-1/2",
            "inline-3xs",
            "min-inline-xl",
        ],
        unproven_or_missing_canaries: &[
            "complete sizing docs-table walk",
            "all min/max logical sizing edge values",
            "complete block/height over-generation rejection proof",
            "container scale and arbitrary modifier sweep",
        ],
        full_value_modifier_parity_proven: false,
    },
    TailwindV43UtilityLedgerEntry {
        docs_area: "typography",
        docs_table: "Tailwind docs: Typography",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &[
            "text-sm",
            "text-sm/6",
            "text-(length:--dx-text-size)/(--dx-leading)",
            "wrap-anywhere",
            "font-features-(--dx-font-features)",
            "@theme --font-display companion feature/variation tokens",
            "@theme --text-tiny companion line-height/tracking/weight tokens",
        ],
        unproven_or_missing_canaries: &[
            "complete typography docs-table walk",
            "official typography plugin behavior",
            "full typography value/modifier parity",
        ],
        full_value_modifier_parity_proven: false,
    },
    TailwindV43UtilityLedgerEntry {
        docs_area: "backgrounds",
        docs_table: "Tailwind docs: Backgrounds",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &["bg-blue-500", "bg-linear-to-r", "bg-conic-180"],
        unproven_or_missing_canaries: &[
            "complete backgrounds docs-table walk",
            "full gradient stop grammar",
            "full arbitrary background-image grammar",
        ],
        full_value_modifier_parity_proven: false,
    },
    TailwindV43UtilityLedgerEntry {
        docs_area: "borders",
        docs_table: "Tailwind docs: Borders",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &[
            "border",
            "border-s-2",
            "border-bs-4",
            "border-x",
            "border-y-4",
            "border-s-red-500",
            "border-bs-emerald-500",
            "border-x-mauve-500",
            "rounded-s-lg",
            "rounded-ss-full",
            "rounded-ee-xl",
        ],
        unproven_or_missing_canaries: &[
            "complete borders docs-table walk",
            "full divide/ring/radius modifier sweep",
            "full logical directional border color sweep",
            "all color opacity and arbitrary modifier combinations",
        ],
        full_value_modifier_parity_proven: false,
    },
    TailwindV43UtilityLedgerEntry {
        docs_area: "effects",
        docs_table: "Tailwind docs: Effects",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &[
            "shadow-md",
            "text-shadow-sm",
            "text-shadow-lg/20",
            "text-shadow-[0_35px_35px_rgb(0_0_0_/_0.25)]/50",
            "text-shadow-[shadow:var(--dx-text-shadow)]",
            "opacity-50",
        ],
        unproven_or_missing_canaries: &[
            "complete effects docs-table walk",
            "full shadow/ring variable algebra",
            "complete text-shadow @supports/property-registration fallback parity",
        ],
        full_value_modifier_parity_proven: false,
    },
    TailwindV43UtilityLedgerEntry {
        docs_area: "filters",
        docs_table: "Tailwind docs: Filters",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &["blur-sm", "brightness-125", "backdrop-blur-md"],
        unproven_or_missing_canaries: &[
            "complete filters docs-table walk",
            "all filter variable composition edge cases",
            "universal browser fallback parity",
        ],
        full_value_modifier_parity_proven: false,
    },
    TailwindV43UtilityLedgerEntry {
        docs_area: "tables",
        docs_table: "Tailwind docs: Tables",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &["table-auto", "caption-bottom", "border-spacing-2"],
        unproven_or_missing_canaries: &[
            "complete tables docs-table walk",
            "Tailwind border-spacing variable algebra",
            "all border-spacing arbitrary/custom-property forms",
            "selector ordering parity with table variants",
        ],
        full_value_modifier_parity_proven: false,
    },
    TailwindV43UtilityLedgerEntry {
        docs_area: "transitions-animation",
        docs_table: "Tailwind docs: Transitions & Animation",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &["transition-colors", "duration-300", "animate-spin"],
        unproven_or_missing_canaries: &[
            "complete transition and animation docs-table walk",
            "custom animation theme extension parity",
            "full transition property/theme extension parity",
        ],
        full_value_modifier_parity_proven: false,
    },
    TailwindV43UtilityLedgerEntry {
        docs_area: "transforms",
        docs_table: "Tailwind docs: Transforms",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &["translate-y-4", "rotate-x-45", "zoom-125"],
        unproven_or_missing_canaries: &[
            "complete transforms docs-table walk",
            "full 3d transform variable algebra",
            "all arbitrary transform value combinations",
        ],
        full_value_modifier_parity_proven: false,
    },
    TailwindV43UtilityLedgerEntry {
        docs_area: "interactivity",
        docs_table: "Tailwind docs: Interactivity",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &[
            "pointer-events-none",
            "scheme-light-dark",
            "scrollbar-thin",
            "scroll-mbs-6",
        ],
        unproven_or_missing_canaries: &[
            "complete interactivity docs-table walk",
            "full browser behavior parity",
            "all scrollbar/color-scheme edge combinations",
        ],
        full_value_modifier_parity_proven: false,
    },
    TailwindV43UtilityLedgerEntry {
        docs_area: "svg",
        docs_table: "Tailwind docs: SVG",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &[
            "fill-[#0f172a]/80",
            "stroke-[color:var(--dx-stroke)]",
        ],
        unproven_or_missing_canaries: &[
            "complete SVG docs-table walk",
            "full fill/stroke palette and opacity parity",
            "mask SVG/theme edge grammar",
        ],
        full_value_modifier_parity_proven: false,
    },
    TailwindV43UtilityLedgerEntry {
        docs_area: "accessibility",
        docs_table: "Tailwind docs: Accessibility",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &[
            "forced-color-adjust-auto",
            "forced-color-adjust-none",
        ],
        unproven_or_missing_canaries: &[
            "complete accessibility docs-table walk",
            "browser forced-colors behavior parity",
            "variant and fallback matrix for accessibility utilities",
        ],
        full_value_modifier_parity_proven: false,
    },
    TailwindV43UtilityLedgerEntry {
        docs_area: "masks",
        docs_table: "Tailwind docs: Masking",
        status: TailwindV43FeatureStatus::Partial,
        representative_supported_canaries: &["mask-none", "mask-alpha", "mask-radial-[100%_100%]"],
        unproven_or_missing_canaries: &[
            "complete masking docs-table walk",
            "all mask gradient/value/modifier combinations",
            "full WebKit fallback parity",
        ],
        full_value_modifier_parity_proven: false,
    },
];
