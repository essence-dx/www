use style::core::{
    CssSourceDirective, StyleEngine, TAILWIND_V43_CSS_DIRECTIVE_LEDGER_BASELINE,
    TAILWIND_V43_CSS_DIRECTIVE_LEDGER_SCOPE, TAILWIND_V43_FEATURE_MATRIX_BASELINE,
    TAILWIND_V43_FEATURE_MATRIX_SCOPE, TAILWIND_V43_UTILITY_LEDGER_BASELINE,
    TAILWIND_V43_UTILITY_LEDGER_SCOPE, TailwindV43FeatureStatus, css_first_directive_diagnostics,
    css_source_directives, css_source_inline_class_tokens, tailwind_v43_css_directive_ledger,
    tailwind_v43_feature_matrix, tailwind_v43_utility_ledger,
};

#[test]
fn feature_matrix_names_latest_verified_tailwind_baseline() {
    assert_eq!(TAILWIND_V43_FEATURE_MATRIX_BASELINE, "tailwindcss-4.3.0");
    assert!(TAILWIND_V43_FEATURE_MATRIX_SCOPE.contains("not full Tailwind replacement"));
}

#[test]
fn feature_matrix_has_all_truth_status_buckets() {
    let matrix = tailwind_v43_feature_matrix();

    for status in [
        TailwindV43FeatureStatus::Supported,
        TailwindV43FeatureStatus::Partial,
        TailwindV43FeatureStatus::UnsupportedByDesign,
    ] {
        assert!(
            matrix.iter().any(|entry| entry.status == status),
            "expected at least one matrix row with status {status:?}"
        );
    }
}

#[test]
fn feature_matrix_keeps_top_tailwind_v43_gaps_executable() {
    let matrix = tailwind_v43_feature_matrix();

    for (area, status, canary) in [
        (
            "utility-grammar",
            TailwindV43FeatureStatus::Partial,
            "Tailwind docs-table ledger",
        ),
        (
            "utility-grammar",
            TailwindV43FeatureStatus::Partial,
            "official fixture matrix utilityDocsCoverage",
        ),
        (
            "color-palette-and-modern-color-spaces",
            TailwindV43FeatureStatus::Partial,
            "Tailwind v4.3 neutral-adjacent palette fixture",
        ),
        (
            "color-palette-and-modern-color-spaces",
            TailwindV43FeatureStatus::Partial,
            "mauve/olive/mist/taupe OKLCH token-backed fixture",
        ),
        (
            "color-palette-and-modern-color-spaces",
            TailwindV43FeatureStatus::Partial,
            "display-p3 output fixture",
        ),
        (
            "variants-and-selector-grammar",
            TailwindV43FeatureStatus::Partial,
            "direction/open/inert selector fixture",
        ),
        (
            "variants-and-selector-grammar",
            TailwindV43FeatureStatus::Partial,
            "user-valid/user-invalid/details-content selector fixture",
        ),
        (
            "variants-and-selector-grammar",
            TailwindV43FeatureStatus::Partial,
            "benchmarks/dx-style-v43-variant-selector-parity.test.ts",
        ),
        (
            "variants-and-selector-grammar",
            TailwindV43FeatureStatus::Partial,
            "stacked arbitrary selector composition fixture",
        ),
        (
            "variants-and-selector-grammar",
            TailwindV43FeatureStatus::Partial,
            "selector-list arbitrary variant fixture",
        ),
        (
            "variants-and-selector-grammar",
            TailwindV43FeatureStatus::Partial,
            "stacked arbitrary group/peer selector fixture",
        ),
        (
            "variants-and-selector-grammar",
            TailwindV43FeatureStatus::Partial,
            "group/peer pseudo-class variant fixture",
        ),
        (
            "variants-and-selector-grammar",
            TailwindV43FeatureStatus::Partial,
            "safe unknown arbitrary at-rule fixture",
        ),
        (
            "variants-and-selector-grammar",
            TailwindV43FeatureStatus::Partial,
            "negated arbitrary media/supports/container at-rule fixture",
        ),
        (
            "variants-and-selector-grammar",
            TailwindV43FeatureStatus::Partial,
            "Tailwind directive arbitrary variant fail-closed fixture",
        ),
        (
            "variants-and-selector-grammar",
            TailwindV43FeatureStatus::Partial,
            "escaped arbitrary selector fixture",
        ),
        (
            "browser-fallback-parity",
            TailwindV43FeatureStatus::Partial,
            "Autoprefixer equal-output fixture",
        ),
        (
            "source-inline-and-not-directives",
            TailwindV43FeatureStatus::Partial,
            "@source inline('{hover:,focus:,}bg-brand')",
        ),
        (
            "source-inline-and-not-directives",
            TailwindV43FeatureStatus::Partial,
            r#"@import "tailwindcss" source(none)"#,
        ),
        (
            "source-inline-and-not-directives",
            TailwindV43FeatureStatus::Partial,
            "@source none",
        ),
        (
            "source-inline-and-not-directives",
            TailwindV43FeatureStatus::Partial,
            "source-not-inline-exclusion",
        ),
        (
            "custom-utility-directive",
            TailwindV43FeatureStatus::Partial,
            "@utility tab-* { tab-size: --value(integer, --default(4)); }",
        ),
        (
            "custom-utility-directive",
            TailwindV43FeatureStatus::Partial,
            "@utility text-* { line-height: --modifier(number, --default(1)); }",
        ),
        (
            "custom-utility-directive",
            TailwindV43FeatureStatus::Partial,
            "@utility glow-* { color: --alpha(var(--color-brand) / --modifier(percentage)); }",
        ),
        (
            "custom-utility-directive",
            TailwindV43FeatureStatus::Partial,
            "@utility tab-* { tab-size: --value(integer); }",
        ),
        (
            "custom-variant-directive",
            TailwindV43FeatureStatus::Partial,
            "@custom-variant theme-midnight (&:where([data-theme=\"midnight\"] *))",
        ),
        (
            "custom-variant-directive",
            TailwindV43FeatureStatus::Partial,
            "@custom-variant theme-midnight { &:where([data-theme=\"midnight\"] *) { @slot; } }",
        ),
        (
            "custom-variant-directive",
            TailwindV43FeatureStatus::Partial,
            "@custom-variant any-hover { @media (any-hover: hover) { &:hover { @slot; } } }",
        ),
        (
            "custom-variant-directive",
            TailwindV43FeatureStatus::Partial,
            "multiple @slot selector-list expansion",
        ),
        (
            "css-variant-directive",
            TailwindV43FeatureStatus::Partial,
            "@variant hover",
        ),
        (
            "css-variant-directive",
            TailwindV43FeatureStatus::Partial,
            "@variant hover:focus",
        ),
        (
            "css-variant-directive",
            TailwindV43FeatureStatus::Partial,
            "@variant hover, focus",
        ),
        (
            "css-variant-directive",
            TailwindV43FeatureStatus::Partial,
            "@variant md:theme-midnight",
        ),
        (
            "css-directive-parity-ledger",
            TailwindV43FeatureStatus::Partial,
            "cssDirectiveCanaries",
        ),
        (
            "css-directive-parity-ledger",
            TailwindV43FeatureStatus::Partial,
            "@apply",
        ),
        (
            "css-directive-parity-ledger",
            TailwindV43FeatureStatus::Partial,
            "--alpha()",
        ),
        (
            "js-config-and-plugin-ecosystem",
            TailwindV43FeatureStatus::UnsupportedByDesign,
            "tailwind.config.js plugin()",
        ),
        (
            "js-config-and-plugin-ecosystem",
            TailwindV43FeatureStatus::UnsupportedByDesign,
            "tailwind package dependency leakage",
        ),
        (
            "official-plugin-ecosystem",
            TailwindV43FeatureStatus::UnsupportedByDesign,
            "external Tailwind plugin code is out of scope",
        ),
        (
            "official-plugin-ecosystem",
            TailwindV43FeatureStatus::UnsupportedByDesign,
            "@plugin/@config unsupported diagnostics",
        ),
        (
            "official-plugin-ecosystem",
            TailwindV43FeatureStatus::UnsupportedByDesign,
            "tailwind package dependency leakage",
        ),
        (
            "official-plugin-ecosystem",
            TailwindV43FeatureStatus::UnsupportedByDesign,
            "DX-owned prose/forms/aspect behavior must be source-owned CSS",
        ),
        (
            "advanced-css-theme-token-extensions",
            TailwindV43FeatureStatus::Partial,
            "css @theme custom animation alias fixture",
        ),
        (
            "advanced-css-theme-token-extensions",
            TailwindV43FeatureStatus::Partial,
            "css @theme transition token fixture",
        ),
        (
            "advanced-css-theme-token-extensions",
            TailwindV43FeatureStatus::Partial,
            "css @theme container-query token fixture",
        ),
        (
            "advanced-css-theme-token-extensions",
            TailwindV43FeatureStatus::Partial,
            "css grid edge grammar fixture",
        ),
        (
            "class-scanning-and-diagnostics",
            TailwindV43FeatureStatus::Partial,
            "sourceScannerCanaries",
        ),
        (
            "class-scanning-and-diagnostics",
            TailwindV43FeatureStatus::Partial,
            "tsx-static-object-map",
        ),
        (
            "class-scanning-and-diagnostics",
            TailwindV43FeatureStatus::Partial,
            "tsx-static-array-and-helper-literals",
        ),
        (
            "class-scanning-and-diagnostics",
            TailwindV43FeatureStatus::Partial,
            "plain_text_extraction_reads_static_arrays_object_maps_and_helpers",
        ),
        (
            "class-scanning-and-diagnostics",
            TailwindV43FeatureStatus::Partial,
            "plain_text_extraction_rejects_dynamic_fragments_and_prose",
        ),
        (
            "class-scanning-and-diagnostics",
            TailwindV43FeatureStatus::Partial,
            "source-scan-diagnostic-receipt",
        ),
        (
            "class-scanning-and-diagnostics",
            TailwindV43FeatureStatus::Partial,
            "plain_text_diagnostics_report_dynamic_object_key_unsafe_prose_and_duplicates",
        ),
        (
            "class-scanning-and-diagnostics",
            TailwindV43FeatureStatus::Partial,
            "reports_plain_text_source_scan_diagnostics_with_locations",
        ),
        (
            "class-scanning-and-diagnostics",
            TailwindV43FeatureStatus::Partial,
            "arbitrary-value-static-string",
        ),
        (
            "official-fixture-matrix",
            TailwindV43FeatureStatus::Supported,
            "tailwind-v43-official-fixture-matrix.json",
        ),
        (
            "governed-live-tailwind-output-comparison",
            TailwindV43FeatureStatus::Supported,
            "tools/dx-style/live-tailwind-v43-compare.cjs",
        ),
        (
            "governed-live-tailwind-output-comparison",
            TailwindV43FeatureStatus::Supported,
            "benchmarks/dx-style-live-tailwind-v43-comparison.test.ts",
        ),
    ] {
        let entry = matrix
            .iter()
            .find(|entry| entry.area == area)
            .unwrap_or_else(|| panic!("missing matrix row {area}"));
        assert_eq!(entry.status, status, "{area} has the wrong status");
        let executable_claim = format!(
            "{}\n{}\n{}",
            entry.dx_style_truth,
            entry.next_test,
            entry.gap_canaries.join("\n")
        );
        assert!(
            executable_claim.contains(canary),
            "{area} should carry executable evidence {canary:?}"
        );
    }

    let advanced = matrix
        .iter()
        .find(|entry| entry.area == "advanced-css-theme-token-extensions")
        .expect("advanced CSS theme token row");
    let advanced_claim = format!(
        "{}\n{}\n{}",
        advanced.dx_style_truth,
        advanced.next_test,
        advanced.gap_canaries.join("\n")
    );
    let js_config_marker = ["theme", "extend"].join(".");
    let plugin_callback_marker = ["plugin", "theme()"].join(" ");
    assert!(
        !advanced_claim.contains(&js_config_marker)
            && !advanced_claim.contains(&plugin_callback_marker),
        "advanced fixtures should stay in dx-style CSS/token territory: {advanced_claim}"
    );
}

#[test]
fn group_and_peer_state_variants_cover_tailwind_pseudo_class_families() {
    let engine = StyleEngine::empty();

    for (class_name, selector_fragment) in [
        (
            "group-odd:bg-mauve-500",
            ":is(:where(.group):nth-child(odd) *)",
        ),
        (
            "group-disabled:opacity-100",
            ":is(:where(.group):disabled *)",
        ),
        (
            "group-focus-visible/card:opacity-100",
            ":is(:where(.group\\/card):focus-visible *)",
        ),
        ("peer-invalid:visible", ":is(:where(.peer):invalid ~ *)"),
        (
            "peer-required/email:block",
            ":is(:where(.peer\\/email):required ~ *)",
        ),
        (
            "peer-disabled:opacity-100",
            ":is(:where(.peer):disabled ~ *)",
        ),
    ] {
        let css = engine.css_for_class(class_name).expect(class_name);

        assert!(
            css.contains(selector_fragment),
            "{class_name} missing selector {selector_fragment}: {css}"
        );
    }
}

#[test]
fn arbitrary_group_and_peer_variants_cover_tailwind_v4_wrapper_families() {
    let engine = StyleEngine::empty();

    for (class_name, selector_fragments) in [
        (
            "group-[.is-published]:block",
            &[":is(:where(.group):is(.is-published) *)", "display: block;"][..],
        ),
        (
            "group-[:nth-of-type(3)_&]:block",
            &[":is(:nth-of-type(3) :where(.group) *)", "display: block;"][..],
        ),
        (
            "peer-[.is-dirty]:peer-required:block",
            &[
                ":is(:where(.peer):is(.is-dirty) ~ *)",
                ":is(:where(.peer):required ~ *)",
                "display: block;",
            ][..],
        ),
    ] {
        let css = engine.css_for_class(class_name).expect(class_name);

        for selector_fragment in selector_fragments {
            assert!(
                css.contains(selector_fragment),
                "{class_name} missing selector {selector_fragment}: {css}"
            );
        }
    }
}

#[test]
fn arbitrary_selector_list_variants_cover_branch_composition() {
    let engine = StyleEngine::empty();

    for (class_name, selector_fragments) in [
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
        (
            "group-[&.foo,&.bar]:block",
            &[
                ":is(:is(:where(.group).foo,:where(.group).bar) *)",
                "display: block;",
            ][..],
        ),
        (
            "peer-[&.dirty,&.touched]:block",
            &[
                ":is(:is(:where(.peer).dirty,:where(.peer).touched) ~ *)",
                "display: block;",
            ][..],
        ),
    ] {
        let css = engine.css_for_class(class_name).expect(class_name);

        for selector_fragment in selector_fragments {
            assert!(
                css.contains(selector_fragment),
                "{class_name} missing selector-list fragment {selector_fragment}: {css}"
            );
        }
    }
}

#[test]
fn utility_ledger_records_docs_table_coverage_without_claiming_full_grammar() {
    assert_eq!(TAILWIND_V43_UTILITY_LEDGER_BASELINE, "tailwindcss-4.3.0");
    assert!(TAILWIND_V43_UTILITY_LEDGER_SCOPE.contains("not full utility/value/modifier parity"));

    let ledger = tailwind_v43_utility_ledger();
    assert!(
        ledger.len() >= 15,
        "expected the ledger to cover Tailwind's public utility documentation areas"
    );

    for expected_area in [
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
    ] {
        assert!(
            ledger.iter().any(|entry| entry.docs_area == expected_area),
            "utility ledger should include docs area {expected_area}"
        );
    }

    for entry in ledger {
        assert!(
            !entry.docs_table.trim().is_empty(),
            "{} should name the official docs table it tracks",
            entry.docs_area
        );
        assert!(
            !entry.full_value_modifier_parity_proven,
            "{} should not claim full value/modifier parity yet",
            entry.docs_area
        );
        assert!(
            !entry.representative_supported_canaries.is_empty(),
            "{} should show at least one supported canary",
            entry.docs_area
        );
        assert!(
            !entry.unproven_or_missing_canaries.is_empty(),
            "{} should keep missing proof executable",
            entry.docs_area
        );
    }
}

#[test]
fn css_directive_ledger_records_directive_parity_without_claiming_full_tailwind_css() {
    assert_eq!(
        TAILWIND_V43_CSS_DIRECTIVE_LEDGER_BASELINE,
        "tailwindcss-4.3.0"
    );
    assert!(TAILWIND_V43_CSS_DIRECTIVE_LEDGER_SCOPE.contains("not full CSS directive parity"));

    let ledger = tailwind_v43_css_directive_ledger();
    assert!(
        ledger.len() >= 14,
        "expected directive ledger to cover Tailwind's public CSS directive surface"
    );

    for expected_directive in [
        "@theme",
        "@import \"tailwindcss\"",
        "@source",
        "@source inline(...)",
        "@source not ...",
        "@utility",
        "@custom-variant",
        "CSS @variant",
        "@apply",
        "@reference",
        "--alpha()",
        "--spacing()",
        "@plugin",
        "@config",
        "@tailwind",
    ] {
        assert!(
            ledger
                .iter()
                .any(|entry| entry.directive_syntax == expected_directive),
            "directive ledger should include {expected_directive}"
        );
    }

    for (directive, status) in [
        ("@utility", TailwindV43FeatureStatus::Partial),
        ("@custom-variant", TailwindV43FeatureStatus::Partial),
        ("CSS @variant", TailwindV43FeatureStatus::Partial),
        ("@apply", TailwindV43FeatureStatus::Partial),
        ("@reference", TailwindV43FeatureStatus::Partial),
        ("--alpha()", TailwindV43FeatureStatus::Partial),
        ("--spacing()", TailwindV43FeatureStatus::Partial),
        ("@plugin", TailwindV43FeatureStatus::UnsupportedByDesign),
        ("@config", TailwindV43FeatureStatus::UnsupportedByDesign),
    ] {
        let entry = ledger
            .iter()
            .find(|entry| entry.directive_syntax == directive)
            .unwrap_or_else(|| panic!("missing directive ledger row {directive}"));
        assert_eq!(entry.status, status, "{directive} has the wrong status");
    }

    for entry in ledger {
        assert!(
            !entry.directive_area.trim().is_empty(),
            "{} should name its directive area",
            entry.directive_syntax
        );
        assert!(
            !entry.full_tailwind_parity_proven,
            "{} should not claim full Tailwind CSS directive parity",
            entry.directive_syntax
        );
        assert!(
            !entry.representative_supported_canaries.is_empty()
                || !entry.unproven_or_missing_canaries.is_empty()
                || entry.unsupported_by_design_reason.is_some(),
            "{} should keep its truth executable",
            entry.directive_syntax
        );

        match entry.status {
            TailwindV43FeatureStatus::Partial | TailwindV43FeatureStatus::Missing => assert!(
                !entry.unproven_or_missing_canaries.is_empty(),
                "{} should name the missing directive canaries",
                entry.directive_syntax
            ),
            TailwindV43FeatureStatus::UnsupportedByDesign => assert!(
                entry
                    .unsupported_by_design_reason
                    .is_some_and(|reason| reason.contains("Tailwind runtime")),
                "{} should explain the no-Tailwind-runtime boundary",
                entry.directive_syntax
            ),
            TailwindV43FeatureStatus::Supported => assert!(
                !entry.representative_supported_canaries.is_empty(),
                "{} should name the supported DX canary",
                entry.directive_syntax
            ),
        }
    }
}

#[test]
fn feature_matrix_missing_and_partial_rows_stay_actionable_without_hype() {
    for entry in tailwind_v43_feature_matrix() {
        if matches!(
            entry.status,
            TailwindV43FeatureStatus::Partial | TailwindV43FeatureStatus::Missing
        ) {
            assert!(
                !entry.gap_canaries.is_empty(),
                "{} should name at least one executable gap canary",
                entry.area
            );
            assert!(
                !entry.next_test.trim().is_empty(),
                "{} should name the next focused test",
                entry.area
            );
        }

        let truth = entry.dx_style_truth.to_ascii_lowercase();
        assert!(
            !truth.contains("drop-in") && !truth.contains("complete parity"),
            "{} should not use replacement/parity hype: {}",
            entry.area,
            entry.dx_style_truth
        );
    }
}

#[test]
fn unsupported_js_directives_emit_css_first_diagnostics() {
    let diagnostics = css_first_directive_diagnostics(
        r#"
@plugin "./tailwind-plugin.js";
@config "./tailwind.config.js";
@tailwind utilities;
"#,
    );

    for expected in ["@plugin", "@config", "@tailwind"] {
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.directive == expected),
            "expected {expected} diagnostic in {diagnostics:?}"
        );
    }
}

#[test]
fn partial_css_first_directives_are_named_as_partial_scope() {
    let theme_css = r#"
@theme {
  --color-brand: hsl(var(--brand));
}

@source inline("{hover:,focus:,}bg-brand");
@source not "../legacy/**";
@custom-variant theme-midnight (&:where([data-theme="midnight"] *));
@utility content-auto {
  content-visibility: auto;
}
@utility tab-* {
  tab-size: --value(integer);
}
"#;

    let inline_tokens = css_source_inline_class_tokens(theme_css);
    for expected in ["bg-brand", "hover:bg-brand", "focus:bg-brand"] {
        assert!(
            inline_tokens.contains(&expected.to_string()),
            "inline @source should include {expected}: {inline_tokens:?}"
        );
    }

    let source_directives = css_source_directives(theme_css);
    assert!(
        source_directives
            .iter()
            .any(|directive| matches!(directive, CssSourceDirective::Exclude(path) if path == "../legacy/**")),
        "@source not should be parsed as an exclusion directive: {source_directives:?}"
    );

    let engine = StyleEngine::from_theme_css(theme_css);
    assert!(
        engine
            .css_for_class("content-auto")
            .is_some_and(|css| css.contains("content-visibility: auto"))
    );
    assert!(
        engine
            .css_for_class("tab-4")
            .is_some_and(|css| css.contains("tab-size: 4"))
    );
    assert!(engine.css_for_class("tab-auto").is_none());
    assert!(
        engine
            .css_for_class("theme-midnight:bg-brand")
            .is_some_and(|css| css.contains(":where([data-theme=\"midnight\"] *)"))
    );
}

#[test]
fn unsupported_directive_canaries_do_not_silently_generate_utility_css() {
    let engine = StyleEngine::empty();

    for directive in [
        "@variant hover { .card { color: red; } }",
        "@plugin './tailwind-plugin.js'",
        "tailwind.config.js plugin()",
    ] {
        assert!(
            engine.css_for_class(directive).is_none(),
            "directive gap canary {directive:?} unexpectedly generated CSS"
        );
    }
}
