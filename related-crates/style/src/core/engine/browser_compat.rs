//! Source contract for the Tailwind/PostCSS browser-compat fixture.
//!
//! This module intentionally describes a narrow checked-in canary fixture. It
//! is not a full autoprefixer compatibility claim.

/// Stable schema used by the checked-in browser compatibility fixture.
pub const TAILWIND_POSTCSS_BROWSER_COMPAT_SCHEMA: &str =
    "dx.style.tailwindPostcssBrowserCompatFixture";

/// Fixture path relative to the DX-WWW repository root.
pub const TAILWIND_POSTCSS_BROWSER_COMPAT_FIXTURE_PATH: &str =
    "related-crates/style/fixtures/tailwind-postcss-browser-compat.json";

/// Tailwind/PostCSS baseline named by the fixture.
pub const TAILWIND_POSTCSS_BROWSER_COMPAT_BASELINE: &str =
    "tailwindcss-4.3.0-plus-postcss-autoprefixer-reference";

/// Comparison scope for the current lightweight canary.
pub const TAILWIND_POSTCSS_BROWSER_COMPAT_COMPARISON_SCOPE: &str = "declaration-fragment-equality";

/// Comparison scope for selector-level browser compatibility canaries.
pub const TAILWIND_POSTCSS_BROWSER_COMPAT_SELECTOR_COMPARISON_SCOPE: &str =
    "selector-fragment-presence";

/// Classes currently covered by the browser compatibility canary.
pub const TAILWIND_POSTCSS_BROWSER_COMPAT_CLASSES: &[&str] = &[
    "appearance-none",
    "select-none",
    "backface-hidden",
    "break-inside-avoid",
    "backdrop-blur-md",
    "hyphens-auto",
];

/// Selector-level classes covered by the browser compatibility canary.
pub const TAILWIND_POSTCSS_BROWSER_COMPAT_SELECTOR_CLASSES: &[&str] = &["file:p-4"];

/// Lightweight fixture contract for Check, Forge, Zed, and Friday consumers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
pub struct TailwindPostcssBrowserCompatContract {
    /// Fixture schema.
    pub schema: &'static str,
    /// Fixture schema version.
    pub schema_version: u8,
    /// Path to the checked-in fixture.
    pub fixture_path: &'static str,
    /// Tailwind/PostCSS baseline this canary names.
    pub baseline: &'static str,
    /// Exact comparison scope for this canary.
    pub comparison_scope: &'static str,
    /// Classes covered by this canary.
    pub classes: &'static [&'static str],
    /// Selector comparison scope for pseudo-element/browser selector canaries.
    pub selector_comparison_scope: &'static str,
    /// Selector-level classes covered by this canary.
    pub selector_classes: &'static [&'static str],
    /// Source function that produced this contract.
    pub generated_by: &'static str,
    /// Launch-policy caveat for consumers.
    pub run_policy: &'static str,
}

/// Return the source-owned browser compatibility fixture contract.
pub fn tailwind_postcss_browser_compat_contract() -> TailwindPostcssBrowserCompatContract {
    TailwindPostcssBrowserCompatContract {
        schema: TAILWIND_POSTCSS_BROWSER_COMPAT_SCHEMA,
        schema_version: 1,
        fixture_path: TAILWIND_POSTCSS_BROWSER_COMPAT_FIXTURE_PATH,
        baseline: TAILWIND_POSTCSS_BROWSER_COMPAT_BASELINE,
        comparison_scope: TAILWIND_POSTCSS_BROWSER_COMPAT_COMPARISON_SCOPE,
        classes: TAILWIND_POSTCSS_BROWSER_COMPAT_CLASSES,
        selector_comparison_scope: TAILWIND_POSTCSS_BROWSER_COMPAT_SELECTOR_COMPARISON_SCOPE,
        selector_classes: TAILWIND_POSTCSS_BROWSER_COMPAT_SELECTOR_CLASSES,
        generated_by: "style::core::tailwind_postcss_browser_compat_contract",
        run_policy: "checked-in migration/compatibility input fixture only; no package install, server, build, broad suite, or Tailwind/PostCSS execution required",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn browser_compat_fixture_contract_is_narrow_and_discoverable() {
        let contract = tailwind_postcss_browser_compat_contract();

        assert_eq!(contract.schema, TAILWIND_POSTCSS_BROWSER_COMPAT_SCHEMA);
        assert_eq!(contract.schema_version, 1);
        assert_eq!(
            contract.fixture_path,
            TAILWIND_POSTCSS_BROWSER_COMPAT_FIXTURE_PATH
        );
        assert_eq!(
            contract.comparison_scope,
            TAILWIND_POSTCSS_BROWSER_COMPAT_COMPARISON_SCOPE
        );
        assert_eq!(
            contract.classes,
            [
                "appearance-none",
                "select-none",
                "backface-hidden",
                "break-inside-avoid",
                "backdrop-blur-md",
                "hyphens-auto"
            ]
        );
        assert_eq!(
            contract.selector_comparison_scope,
            TAILWIND_POSTCSS_BROWSER_COMPAT_SELECTOR_COMPARISON_SCOPE
        );
        assert_eq!(contract.selector_classes, ["file:p-4"]);
        assert!(
            contract.run_policy.contains("no package install"),
            "the contract must stay lightweight for repeated DX launch passes"
        );
    }
}
