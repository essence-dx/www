//! Equal-output canary for fair Tailwind comparisons.
//!
//! This is a tiny checked-in contract for classes where dx-style and the named
//! Tailwind baseline should emit the same declaration fragments. It is not a
//! live Tailwind run, a speed benchmark, or universal parity proof.

use super::StyleEngine;

/// Stable schema used by the checked-in equal-output canary.
pub const TAILWIND_EQUAL_OUTPUT_CANARY_SCHEMA: &str = "dx.style.tailwindEqualOutputCanary";

/// Fixture path relative to the DX-WWW repository root.
pub const TAILWIND_EQUAL_OUTPUT_CANARY_FIXTURE_PATH: &str =
    "related-crates/style/fixtures/tailwind-equal-output-canary.json";

/// Tailwind baseline named by the checked-in canary.
pub const TAILWIND_EQUAL_OUTPUT_CANARY_BASELINE: &str =
    "tailwindcss-4.3.0-supported-subset-reference";

/// Comparison scope for this canary.
pub const TAILWIND_EQUAL_OUTPUT_CANARY_COMPARISON_SCOPE: &str =
    "generated-css-declaration-equality";

/// Source-owned class/declaration pair used by the canary.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
pub struct TailwindEqualOutputCanaryFixture {
    /// Authored class name.
    pub class_name: &'static str,
    /// Compatibility area.
    pub area: &'static str,
    /// Declarations expected from dx-style.
    pub dx_style_required_declarations: &'static [&'static str],
    /// Declarations expected from the Tailwind baseline.
    pub tailwind_required_declarations: &'static [&'static str],
}

/// One evaluated canary entry.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct TailwindEqualOutputCanaryEntry {
    /// Authored class name.
    pub class_name: &'static str,
    /// Compatibility area.
    pub area: &'static str,
    /// Declarations expected from dx-style.
    pub dx_style_required_declarations: &'static [&'static str],
    /// Declarations expected from the Tailwind baseline.
    pub tailwind_required_declarations: &'static [&'static str],
    /// Generated CSS from dx-style.
    pub dx_style_generated_css: Option<String>,
    /// Whether dx-style generated CSS for the class.
    pub supported_by_dx_style: bool,
    /// Whether the checked declaration fragments are equal and present.
    pub declarations_match: bool,
}

/// Lightweight canary contract for Check, Forge, Zed, and Friday consumers.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct TailwindEqualOutputCanaryContract {
    /// Fixture schema.
    pub schema: &'static str,
    /// Fixture schema version.
    pub schema_version: u8,
    /// Path to the checked-in fixture.
    pub fixture_path: &'static str,
    /// Tailwind baseline this canary names.
    pub baseline: &'static str,
    /// Exact comparison scope for this canary.
    pub comparison_scope: &'static str,
    /// Classes evaluated by this canary.
    pub classes: Vec<TailwindEqualOutputCanaryEntry>,
    /// Number of classes covered.
    pub class_count: usize,
    /// Number of classes with matching declaration fragments.
    pub equal_output_class_count: usize,
    /// Number of classes unsupported by dx-style.
    pub unsupported_class_count: usize,
    /// Whether this receipt was produced by executing Tailwind live.
    pub live_tailwind_execution: bool,
    /// Whether this canary proves universal Tailwind parity.
    pub full_tailwind_parity: bool,
    /// Whether this canary is a fair speed benchmark.
    pub fair_speed_benchmark: bool,
    /// Source function that produced this contract.
    pub generated_by: &'static str,
    /// Launch-policy caveat for consumers.
    pub run_policy: &'static str,
}

/// Return the source-owned equal-output canary contract.
pub fn tailwind_equal_output_canary_contract() -> TailwindEqualOutputCanaryContract {
    let engine = StyleEngine::empty();
    let classes = TAILWIND_EQUAL_OUTPUT_CANARY_FIXTURES
        .iter()
        .map(|fixture| {
            let generated_css = engine.css_for_class(fixture.class_name);
            let supported_by_dx_style = generated_css.is_some();
            let declarations_match = generated_css.as_ref().is_some_and(|css| {
                same_declarations(
                    fixture.dx_style_required_declarations,
                    fixture.tailwind_required_declarations,
                ) && fixture
                    .dx_style_required_declarations
                    .iter()
                    .all(|declaration| css.contains(declaration))
            });

            TailwindEqualOutputCanaryEntry {
                class_name: fixture.class_name,
                area: fixture.area,
                dx_style_required_declarations: fixture.dx_style_required_declarations,
                tailwind_required_declarations: fixture.tailwind_required_declarations,
                dx_style_generated_css: generated_css,
                supported_by_dx_style,
                declarations_match,
            }
        })
        .collect::<Vec<_>>();

    let equal_output_class_count = classes
        .iter()
        .filter(|entry| entry.declarations_match)
        .count();
    let unsupported_class_count = classes
        .iter()
        .filter(|entry| !entry.supported_by_dx_style)
        .count();

    TailwindEqualOutputCanaryContract {
        schema: TAILWIND_EQUAL_OUTPUT_CANARY_SCHEMA,
        schema_version: 1,
        fixture_path: TAILWIND_EQUAL_OUTPUT_CANARY_FIXTURE_PATH,
        baseline: TAILWIND_EQUAL_OUTPUT_CANARY_BASELINE,
        comparison_scope: TAILWIND_EQUAL_OUTPUT_CANARY_COMPARISON_SCOPE,
        class_count: classes.len(),
        equal_output_class_count,
        unsupported_class_count,
        live_tailwind_execution: false,
        full_tailwind_parity: false,
        fair_speed_benchmark: false,
        generated_by: "style::core::tailwind_equal_output_canary_contract",
        run_policy: "checked-in migration/compatibility input fixture only; no package install, server, build, broad suite, or Tailwind execution required",
        classes,
    }
}

fn same_declarations(a: &[&str], b: &[&str]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(left, right)| left == right)
}

const TAILWIND_EQUAL_OUTPUT_CANARY_FIXTURES: &[TailwindEqualOutputCanaryFixture] = &[
    TailwindEqualOutputCanaryFixture {
        class_name: "block",
        area: "layout",
        dx_style_required_declarations: &["display: block"],
        tailwind_required_declarations: &["display: block"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "flex",
        area: "layout",
        dx_style_required_declarations: &["display: flex"],
        tailwind_required_declarations: &["display: flex"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "grid",
        area: "layout",
        dx_style_required_declarations: &["display: grid"],
        tailwind_required_declarations: &["display: grid"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "p-4",
        area: "spacing",
        dx_style_required_declarations: &["padding: calc(var(--spacing) * 4)"],
        tailwind_required_declarations: &["padding: calc(var(--spacing) * 4)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "mt-2",
        area: "spacing",
        dx_style_required_declarations: &["margin-top: calc(var(--spacing) * 2)"],
        tailwind_required_declarations: &["margin-top: calc(var(--spacing) * 2)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "opacity-50",
        area: "effects",
        dx_style_required_declarations: &["opacity: 50%"],
        tailwind_required_declarations: &["opacity: 50%"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "ps-4",
        area: "logical-spacing",
        dx_style_required_declarations: &["padding-inline-start: calc(var(--spacing) * 4)"],
        tailwind_required_declarations: &["padding-inline-start: calc(var(--spacing) * 4)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "-ms-2",
        area: "logical-spacing",
        dx_style_required_declarations: &["margin-inline-start: calc(var(--spacing) * -2)"],
        tailwind_required_declarations: &["margin-inline-start: calc(var(--spacing) * -2)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "start-0",
        area: "logical-inset",
        dx_style_required_declarations: &["inset-inline-start: calc(var(--spacing) * 0)"],
        tailwind_required_declarations: &["inset-inline-start: calc(var(--spacing) * 0)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "bg-card/50",
        area: "theme-color-opacity",
        dx_style_required_declarations: &[
            "background-color: color-mix(in oklab, var(--color-card) 50%, transparent)",
        ],
        tailwind_required_declarations: &[
            "background-color: color-mix(in oklab, var(--color-card) 50%, transparent)",
        ],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "md:hover:bg-card/40",
        area: "variant-theme-color-opacity",
        dx_style_required_declarations: &[
            "background-color: color-mix(in oklab, var(--color-card) 40%, transparent)",
        ],
        tailwind_required_declarations: &[
            "background-color: color-mix(in oklab, var(--color-card) 40%, transparent)",
        ],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "size-8",
        area: "sizing",
        dx_style_required_declarations: &[
            "width: calc(var(--spacing) * 8)",
            "height: calc(var(--spacing) * 8)",
        ],
        tailwind_required_declarations: &[
            "width: calc(var(--spacing) * 8)",
            "height: calc(var(--spacing) * 8)",
        ],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "border-s-2",
        area: "logical-border",
        dx_style_required_declarations: &["border-inline-start-width: 2px"],
        tailwind_required_declarations: &["border-inline-start-width: 2px"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "rounded-ee-xl",
        area: "logical-radius",
        dx_style_required_declarations: &["border-end-end-radius: var(--radius-xl)"],
        tailwind_required_declarations: &["border-end-end-radius: var(--radius-xl)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "object-[25%_75%]",
        area: "object-position",
        dx_style_required_declarations: &["object-position: 25% 75%"],
        tailwind_required_declarations: &["object-position: 25% 75%"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "origin-top-left",
        area: "transform-origin",
        dx_style_required_declarations: &["transform-origin: top left"],
        tailwind_required_declarations: &["transform-origin: top left"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "flex-[2_1_0%]",
        area: "flex",
        dx_style_required_declarations: &["flex: 2 1 0%"],
        tailwind_required_declarations: &["flex: 2 1 0%"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "grid-cols-(--dx-grid-cols)",
        area: "grid",
        dx_style_required_declarations: &["grid-template-columns: var(--dx-grid-cols)"],
        tailwind_required_declarations: &["grid-template-columns: var(--dx-grid-cols)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "drop-shadow-md",
        area: "filter",
        dx_style_required_declarations: &["--tw-drop-shadow: drop-shadow("],
        tailwind_required_declarations: &["--tw-drop-shadow: drop-shadow("],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "backdrop-opacity-50",
        area: "backdrop-filter",
        dx_style_required_declarations: &["--tw-backdrop-opacity: opacity(50%)"],
        tailwind_required_declarations: &["--tw-backdrop-opacity: opacity(50%)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "table-auto",
        area: "table",
        dx_style_required_declarations: &["table-layout: auto"],
        tailwind_required_declarations: &["table-layout: auto"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "caption-bottom",
        area: "table",
        dx_style_required_declarations: &["caption-side: bottom"],
        tailwind_required_declarations: &["caption-side: bottom"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "border-spacing-2",
        area: "table",
        dx_style_required_declarations: &["border-spacing: calc(var(--spacing) * 2)"],
        tailwind_required_declarations: &["border-spacing: calc(var(--spacing) * 2)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "transform-3d",
        area: "transform-3d",
        dx_style_required_declarations: &["transform-style: preserve-3d"],
        tailwind_required_declarations: &["transform-style: preserve-3d"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "perspective-[750px]",
        area: "perspective",
        dx_style_required_declarations: &["perspective: 750px"],
        tailwind_required_declarations: &["perspective: 750px"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "perspective-origin-top-right",
        area: "perspective",
        dx_style_required_declarations: &["perspective-origin: top right"],
        tailwind_required_declarations: &["perspective-origin: top right"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "rotate-x-45",
        area: "transform-3d",
        dx_style_required_declarations: &["--tw-rotate-x: rotateX(45deg)"],
        tailwind_required_declarations: &["--tw-rotate-x: rotateX(45deg)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "translate-z-4",
        area: "transform-3d",
        dx_style_required_declarations: &["--tw-translate-z: calc(var(--spacing) * 4)"],
        tailwind_required_declarations: &["--tw-translate-z: calc(var(--spacing) * 4)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "scale-z-125",
        area: "transform-3d",
        dx_style_required_declarations: &["--tw-scale-z: 125%"],
        tailwind_required_declarations: &["--tw-scale-z: 125%"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "shadow-(--dx-shadow)",
        area: "shadow",
        dx_style_required_declarations: &["box-shadow: var(--dx-shadow)"],
        tailwind_required_declarations: &["box-shadow: var(--dx-shadow)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "box-border",
        area: "box-sizing",
        dx_style_required_declarations: &["box-sizing: border-box"],
        tailwind_required_declarations: &["box-sizing: border-box"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "box-content",
        area: "box-sizing",
        dx_style_required_declarations: &["box-sizing: content-box"],
        tailwind_required_declarations: &["box-sizing: content-box"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "bg-[#1d4ed8]/50",
        area: "arbitrary-color-opacity",
        dx_style_required_declarations: &[
            "background-color: color-mix(in oklab, #1d4ed8 50%, transparent)",
        ],
        tailwind_required_declarations: &[
            "background-color: color-mix(in oklab, #1d4ed8 50%, transparent)",
        ],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "bg-(--dx-background)",
        area: "custom-property-color",
        dx_style_required_declarations: &["background-color: var(--dx-background)"],
        tailwind_required_declarations: &["background-color: var(--dx-background)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "border-(--dx-border)",
        area: "custom-property-color",
        dx_style_required_declarations: &["border-color: var(--dx-border)"],
        tailwind_required_declarations: &["border-color: var(--dx-border)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "fill-[#0f172a]/80",
        area: "arbitrary-color-opacity",
        dx_style_required_declarations: &["fill: color-mix(in oklab, #0f172a 80%, transparent)"],
        tailwind_required_declarations: &["fill: color-mix(in oklab, #0f172a 80%, transparent)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "text-[color:var(--dx-foreground)]",
        area: "typed-arbitrary-color",
        dx_style_required_declarations: &["color: var(--dx-foreground)"],
        tailwind_required_declarations: &["color: var(--dx-foreground)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "stroke-[color:var(--dx-stroke)]",
        area: "typed-arbitrary-color",
        dx_style_required_declarations: &["stroke: var(--dx-stroke)"],
        tailwind_required_declarations: &["stroke: var(--dx-stroke)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "rounded-(--dx-radius)",
        area: "custom-property-alias",
        dx_style_required_declarations: &["border-radius: var(--dx-radius)"],
        tailwind_required_declarations: &["border-radius: var(--dx-radius)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "aspect-(--dx-aspect)",
        area: "custom-property-alias",
        dx_style_required_declarations: &["aspect-ratio: var(--dx-aspect)"],
        tailwind_required_declarations: &["aspect-ratio: var(--dx-aspect)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "leading-(--dx-leading)",
        area: "custom-property-alias",
        dx_style_required_declarations: &["line-height: var(--dx-leading)"],
        tailwind_required_declarations: &["line-height: var(--dx-leading)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "tracking-(--dx-tracking)",
        area: "custom-property-alias",
        dx_style_required_declarations: &["letter-spacing: var(--dx-tracking)"],
        tailwind_required_declarations: &["letter-spacing: var(--dx-tracking)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "items-baseline-last",
        area: "alignment",
        dx_style_required_declarations: &["align-items: last baseline"],
        tailwind_required_declarations: &["align-items: last baseline"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "self-baseline",
        area: "alignment",
        dx_style_required_declarations: &["align-self: baseline"],
        tailwind_required_declarations: &["align-self: baseline"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "place-items-baseline",
        area: "alignment",
        dx_style_required_declarations: &["place-items: baseline"],
        tailwind_required_declarations: &["place-items: baseline"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "justify-items-center",
        area: "alignment",
        dx_style_required_declarations: &["justify-items: center"],
        tailwind_required_declarations: &["justify-items: center"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "justify-self-end",
        area: "alignment",
        dx_style_required_declarations: &["justify-self: end"],
        tailwind_required_declarations: &["justify-self: end"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "content-normal",
        area: "alignment",
        dx_style_required_declarations: &["align-content: normal"],
        tailwind_required_declarations: &["align-content: normal"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "from-10%",
        area: "gradient-stop-position",
        dx_style_required_declarations: &["--tw-gradient-from-position: 10%"],
        tailwind_required_declarations: &["--tw-gradient-from-position: 10%"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "via-30%",
        area: "gradient-stop-position",
        dx_style_required_declarations: &["--tw-gradient-via-position: 30%"],
        tailwind_required_declarations: &["--tw-gradient-via-position: 30%"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "to-90%",
        area: "gradient-stop-position",
        dx_style_required_declarations: &["--tw-gradient-to-position: 90%"],
        tailwind_required_declarations: &["--tw-gradient-to-position: 90%"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "from-(--dx-gradient-from-position)",
        area: "gradient-stop-position",
        dx_style_required_declarations: &[
            "--tw-gradient-from-position: var(--dx-gradient-from-position)",
        ],
        tailwind_required_declarations: &[
            "--tw-gradient-from-position: var(--dx-gradient-from-position)",
        ],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "ring-[#2563eb]/50",
        area: "arbitrary-color-opacity",
        dx_style_required_declarations: &[
            "--tw-ring-color: color-mix(in oklab, #2563eb 50%, transparent)",
        ],
        tailwind_required_declarations: &[
            "--tw-ring-color: color-mix(in oklab, #2563eb 50%, transparent)",
        ],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "divide-(--dx-border)",
        area: "custom-property-color",
        dx_style_required_declarations: &["border-color: var(--dx-border)"],
        tailwind_required_declarations: &["border-color: var(--dx-border)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "p-(--dx-pad)",
        area: "custom-property-shorthand",
        dx_style_required_declarations: &["padding: var(--dx-pad)"],
        tailwind_required_declarations: &["padding: var(--dx-pad)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "scroll-mt-(--dx-scroll-mt)",
        area: "custom-property-shorthand",
        dx_style_required_declarations: &["scroll-margin-top: var(--dx-scroll-mt)"],
        tailwind_required_declarations: &["scroll-margin-top: var(--dx-scroll-mt)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "opacity-(--dx-opacity)",
        area: "custom-property-shorthand",
        dx_style_required_declarations: &["opacity: var(--dx-opacity)"],
        tailwind_required_declarations: &["opacity: var(--dx-opacity)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "z-(--dx-layer)",
        area: "custom-property-shorthand",
        dx_style_required_declarations: &["z-index: var(--dx-layer)"],
        tailwind_required_declarations: &["z-index: var(--dx-layer)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "order-(--dx-order)",
        area: "custom-property-shorthand",
        dx_style_required_declarations: &["order: var(--dx-order)"],
        tailwind_required_declarations: &["order: var(--dx-order)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "blur-(--dx-blur)",
        area: "custom-property-shorthand",
        dx_style_required_declarations: &["--tw-blur: blur(var(--dx-blur))"],
        tailwind_required_declarations: &["--tw-blur: blur(var(--dx-blur))"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "brightness-(--dx-brightness)",
        area: "custom-property-shorthand",
        dx_style_required_declarations: &["--tw-brightness: brightness(var(--dx-brightness))"],
        tailwind_required_declarations: &["--tw-brightness: brightness(var(--dx-brightness))"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "hue-rotate-(--dx-hue-rotate)",
        area: "custom-property-shorthand",
        dx_style_required_declarations: &["--tw-hue-rotate: hue-rotate(var(--dx-hue-rotate))"],
        tailwind_required_declarations: &["--tw-hue-rotate: hue-rotate(var(--dx-hue-rotate))"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "backdrop-opacity-(--dx-backdrop-opacity)",
        area: "custom-property-shorthand",
        dx_style_required_declarations: &[
            "--tw-backdrop-opacity: opacity(var(--dx-backdrop-opacity))",
        ],
        tailwind_required_declarations: &[
            "--tw-backdrop-opacity: opacity(var(--dx-backdrop-opacity))",
        ],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "backdrop-blur-(--dx-backdrop-blur)",
        area: "custom-property-shorthand",
        dx_style_required_declarations: &["--tw-backdrop-blur: blur(var(--dx-backdrop-blur))"],
        tailwind_required_declarations: &["--tw-backdrop-blur: blur(var(--dx-backdrop-blur))"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "outline-offset-(--dx-outline-offset)",
        area: "custom-property-shorthand",
        dx_style_required_declarations: &["outline-offset: var(--dx-outline-offset)"],
        tailwind_required_declarations: &["outline-offset: var(--dx-outline-offset)"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "outline-[3px]",
        area: "arbitrary-length-routing",
        dx_style_required_declarations: &["outline-width: 3px"],
        tailwind_required_declarations: &["outline-width: 3px"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "ring-[3px]",
        area: "arbitrary-length-routing",
        dx_style_required_declarations: &["calc(3px + var(--tw-ring-offset-width, 0px))"],
        tailwind_required_declarations: &["calc(3px + var(--tw-ring-offset-width, 0px))"],
    },
    TailwindEqualOutputCanaryFixture {
        class_name: "ring-offset-(--dx-ring-offset-width)",
        area: "custom-property-shorthand",
        dx_style_required_declarations: &["--tw-ring-offset-width: var(--dx-ring-offset-width)"],
        tailwind_required_declarations: &["--tw-ring-offset-width: var(--dx-ring-offset-width)"],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tailwind_equal_output_canary_contract_is_narrow_and_truthful() {
        let contract = tailwind_equal_output_canary_contract();

        assert_eq!(contract.schema, TAILWIND_EQUAL_OUTPUT_CANARY_SCHEMA);
        assert_eq!(contract.schema_version, 1);
        assert_eq!(
            contract.fixture_path,
            TAILWIND_EQUAL_OUTPUT_CANARY_FIXTURE_PATH
        );
        assert_eq!(
            contract.comparison_scope,
            TAILWIND_EQUAL_OUTPUT_CANARY_COMPARISON_SCOPE
        );
        assert_eq!(contract.class_count, 68);
        assert_eq!(contract.equal_output_class_count, 68);
        assert_eq!(contract.unsupported_class_count, 0);
        assert!(!contract.live_tailwind_execution);
        assert!(!contract.full_tailwind_parity);
        assert!(!contract.fair_speed_benchmark);
    }
}
