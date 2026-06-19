use style::core::{
    CssReferenceDirective, CssSourceDirective, StyleEngine, css_first_directive_diagnostics,
    css_reference_directives, css_source_directives, css_source_disables_automatic_detection,
    css_source_inline_class_tokens, css_source_inline_exclusion_class_tokens, css_source_scan_plan,
    group::GroupRegistry, pipeline::css_generator::generate_css, theme_layer_css_from_source,
};

fn assert_css_contains(engine: &StyleEngine, class_name: &str, fragments: &[&str]) {
    let css = engine
        .css_for_class(class_name)
        .unwrap_or_else(|| panic!("{class_name} should generate CSS"));

    for fragment in fragments {
        assert!(
            css.contains(fragment),
            "{class_name} CSS did not contain {fragment:?}:\n{css}"
        );
    }
}

#[test]
fn css_first_theme_tokens_drive_tailwind_v4_utilities() {
    let theme_css = r#"
@import "tailwindcss";

@theme {
  --color-card: hsl(var(--card));
  --color-foreground: hsl(var(--foreground));
  --breakpoint-xs: 30rem;
  --container-dashboard: 52rem;
  --spacing-fluid: clamp(1rem, 2vw, 2rem);
  --radius-panel: 1rem;
  --shadow-soft: 0 8px 32px rgb(0 0 0 / 0.18);
  --animate-shimmer: dx-shimmer 3s linear infinite;
}
"#;

    let engine = StyleEngine::from_theme_css(theme_css);

    assert_css_contains(
        &engine,
        "bg-card/50",
        &[
            ".bg-card\\/50",
            "background-color: color-mix(in oklab, var(--color-card) 50%, transparent)",
        ],
    );
    assert_css_contains(
        &engine,
        "text-foreground",
        &["color: var(--color-foreground)"],
    );
    assert_css_contains(&engine, "gap-fluid", &["gap: var(--spacing-fluid)"]);
    assert_css_contains(
        &engine,
        "rounded-panel",
        &["border-radius: var(--radius-panel)"],
    );
    assert_css_contains(&engine, "shadow-soft", &["box-shadow: var(--shadow-soft)"]);
    assert_css_contains(
        &engine,
        "animate-shimmer",
        &["animation: var(--animate-shimmer)"],
    );
    assert_css_contains(
        &engine,
        "xs:hover:bg-card/40",
        &[
            "@media (min-width: 30rem)",
            ":hover",
            "background-color: color-mix(in oklab, var(--color-card) 40%, transparent)",
        ],
    );
    assert_css_contains(
        &engine,
        "@dashboard:flex",
        &["@container (width >= 52rem)", "display: flex"],
    );
}

#[test]
fn css_first_container_theme_namespace_reset_controls_container_variants() {
    let theme_css = r#"
@theme {
  --container-*: initial;
  --container-card: 40rem;
  --container-dashboard: 52rem;
}
"#;

    let engine = StyleEngine::from_theme_css(theme_css);

    assert_css_contains(
        &engine,
        "@card:flex",
        &["@container (width >= 40rem)", "display: flex"],
    );
    assert_css_contains(
        &engine,
        "@max-card:hidden",
        &["@container (width < 40rem)", "display: none"],
    );
    assert_css_contains(
        &engine,
        "@dashboard/main:grid",
        &["@container main (width >= 52rem)", "display: grid"],
    );
    assert!(
        engine.css_for_class("@sm:flex").is_none(),
        "--container-*: initial should remove default @sm container variant"
    );
    assert!(
        engine.css_for_class("@max-md:flex").is_none(),
        "--container-*: initial should remove default @max-md container variant"
    );

    let css = theme_layer_css_from_source(theme_css);
    assert!(css.contains("--container-card: 40rem;"));
    assert!(css.contains("--container-dashboard: 52rem;"));
    assert!(
        !css.contains("--container-*: initial"),
        "namespace reset markers are Tailwind @theme control syntax, not emitted CSS variables"
    );
}

#[test]
fn theme_layer_strips_tailwind_import_and_emits_dx_owned_layers() {
    let theme_css = r#"
@import "tailwindcss";

@theme {
  --color-card: hsl(var(--card));
  --breakpoint-xs: 30rem;
}
"#;

    let css = theme_layer_css_from_source(theme_css);

    assert!(css.contains("@layer theme, base, components, utilities;"));
    assert!(css.contains("@property --tw-gradient-from"));
    assert!(css.contains("@layer theme {"));
    assert!(css.contains("--color-card: hsl(var(--card));"));
    assert!(!css.contains("@import \"tailwindcss\""));
}

#[test]
fn css_first_directives_register_dx_owned_sources_utilities_and_variants() {
    let theme_css = r#"
@import "tailwindcss";
@plugin "@tailwindcss/forms";
@config "./tailwind.config.js";
@tailwind utilities;

@theme {
  --color-brand: hsl(var(--brand));
  --breakpoint-xs: 30rem;
}

@source inline("{hover:,focus:,}bg-brand xs:grid");
@custom-variant theme-midnight (&:where([data-theme="midnight"] *));
@utility content-auto {
  content-visibility: auto;
}
@utility tab-* {
  tab-size: --value(integer);
}
"#;

    let inline_classes = css_source_inline_class_tokens(theme_css);
    for class_name in ["bg-brand", "hover:bg-brand", "focus:bg-brand", "xs:grid"] {
        assert!(
            inline_classes.contains(&class_name.to_string()),
            "inline @source should safelist {class_name}: {inline_classes:?}"
        );
    }

    let diagnostics = css_first_directive_diagnostics(theme_css);
    for directive in ["@plugin", "@config", "@tailwind"] {
        assert!(
            diagnostics
                .iter()
                .any(|finding| finding.directive == directive),
            "{directive} should be explicitly diagnosed: {diagnostics:?}"
        );
    }

    let engine = StyleEngine::from_theme_css(theme_css);
    assert_css_contains(&engine, "content-auto", &["content-visibility: auto"]);
    assert_css_contains(&engine, "tab-4", &["tab-size: 4"]);
    assert!(engine.css_for_class("tab-auto").is_none());
    assert_css_contains(
        &engine,
        "xs:theme-midnight:hover:bg-brand/50",
        &[
            "@media (min-width: 30rem)",
            ":where([data-theme=\"midnight\"] *)",
            ":hover",
            "background-color: color-mix(in oklab, var(--color-brand) 50%, transparent)",
        ],
    );
}

#[test]
fn css_first_apply_expands_supported_utilities_without_tailwind_runtime() {
    let theme_css = r#"
@theme {
  --color-brand: oklch(0.65 0.16 250);
}

.card {
  @apply px-4 bg-brand;
  border-width: 1px;
  @reference "./theme.css";
  color: --alpha(var(--color-brand) / 50%);
  margin: --spacing(4);
}
"#;

    let diagnostics = css_first_directive_diagnostics(theme_css);
    assert!(
        !diagnostics
            .iter()
            .any(|finding| finding.directive == "@apply"),
        "supported @apply utilities should not be diagnosed as unsupported: {diagnostics:?}"
    );

    assert!(
        !diagnostics
            .iter()
            .any(|finding| finding.directive == "@reference"),
        "local @reference should be accepted as a DX-owned reference input: {diagnostics:?}"
    );

    let css = StyleEngine::from_theme_css(theme_css).css_apply_rules_from_source(theme_css);
    assert!(css.contains(".card"));
    assert!(css.contains("padding"));
    assert!(css.contains("calc(var(--spacing) * 4)"));
    assert!(css.contains("background-color: var(--color-brand)"));
    assert!(css.contains("border-width: 1px"));
    assert!(!css.contains("@apply"));
}

#[test]
fn css_first_apply_preserves_safe_authored_function_declarations() {
    let theme_css = r#"
@theme {
  --color-brand: oklch(0.65 0.16 250);
}

.card {
  @apply px-4;
  color: --alpha(var(--color-brand) / 35%);
  margin-inline: --spacing(3);
  width: calc(100% - --spacing(2));
}
"#;

    let css = StyleEngine::from_theme_css(theme_css).css_apply_rules_from_source(theme_css);
    assert!(css.contains(".card"));
    assert!(css.contains("padding-inline: calc(var(--spacing) * 4)"));
    assert!(css.contains("color: color-mix(in oklab, var(--color-brand) 35%, transparent)"));
    assert!(css.contains("margin-inline: calc(var(--spacing) * 3)"));
    assert!(css.contains("width: calc(100% - calc(var(--spacing) * 2))"));
    assert!(!css.contains("--alpha("), "{css}");
    assert!(!css.contains("--spacing("), "{css}");
    assert!(!css.contains("@apply"), "{css}");
}

#[test]
fn css_first_apply_expands_safe_nested_authored_selectors() {
    let theme_css = r#"
@theme {
  --color-brand: oklch(0.65 0.16 250);
}

.card {
  @apply p-4;

  &:hover {
    @apply bg-brand opacity-75;
    color: --alpha(var(--color-brand) / 40%);
  }

  & .icon {
    @apply p-2;
  }
}
"#;

    let css = StyleEngine::from_theme_css(theme_css).css_apply_rules_from_source(theme_css);
    assert!(css.contains(".card"));
    assert!(css.contains("padding: calc(var(--spacing) * 4)"));
    assert!(css.contains(".card:hover"));
    assert!(css.contains("background-color: var(--color-brand)"));
    assert!(css.contains("opacity: 75%"));
    assert!(css.contains("color: color-mix(in oklab, var(--color-brand) 40%, transparent)"));
    assert!(css.contains(".card .icon"));
    assert!(css.contains("padding: calc(var(--spacing) * 2)"));
    assert!(!css.contains("&:hover"), "{css}");
    assert!(!css.contains("@apply"), "{css}");
}

#[test]
fn css_first_apply_expands_safe_layered_authored_css() {
    let theme_css = r#"
@theme {
  --color-brand: oklch(0.65 0.16 250);
}

@layer components {
  .card {
    @apply p-4 bg-brand;
    border-width: 1px;

    &:hover {
      @apply opacity-75;
      color: --alpha(var(--color-brand) / 40%);
    }
  }
}
"#;

    let css = StyleEngine::from_theme_css(theme_css).css_apply_rules_from_source(theme_css);
    assert!(css.contains("@layer components"), "{css}");
    assert!(css.contains(".card"), "{css}");
    assert!(css.contains("padding: calc(var(--spacing) * 4)"), "{css}");
    assert!(
        css.contains("background-color: var(--color-brand)"),
        "{css}"
    );
    assert!(css.contains("border-width: 1px"), "{css}");
    assert!(css.contains(".card:hover"), "{css}");
    assert!(css.contains("opacity: 75%"), "{css}");
    assert!(css.contains("color: color-mix(in oklab, var(--color-brand) 40%, transparent)"));
    assert!(!css.contains("&:hover"), "{css}");
    assert!(!css.contains("@apply"), "{css}");
}

#[test]
fn reference_directive_accepts_local_and_tailwind_default_without_runtime_dependency() {
    let theme_css = r#"
@reference "./theme.css";
@reference "../tokens.css";
@reference "tailwindcss";
@reference "@tailwindcss/forms";
@reference "https://cdn.tailwindcss.com/theme.css";
"#;

    let references = css_reference_directives(theme_css);
    assert!(
        references.contains(&CssReferenceDirective::Local("./theme.css".to_string())),
        "local @reference should be retained for the DX style build path: {references:?}"
    );
    assert!(
        references.contains(&CssReferenceDirective::Local("../tokens.css".to_string())),
        "relative parent @reference should be retained for the DX style build path: {references:?}"
    );
    assert!(
        references.contains(&CssReferenceDirective::TailwindDefaultTheme),
        "@reference \"tailwindcss\" should map to dx-style's owned default theme, not a package runtime: {references:?}"
    );

    let diagnostics = css_first_directive_diagnostics(theme_css);
    assert!(
        !diagnostics
            .iter()
            .any(|finding| finding.directive == "@reference" && finding.line <= 4),
        "supported local/default @reference directives should not be diagnosed: {diagnostics:?}"
    );
    assert!(
        diagnostics.iter().any(|finding| {
            finding.directive == "@reference"
                && finding.line >= 5
                && finding
                    .reason
                    .contains("package, URL, and JS/runtime reference resolution")
        }),
        "package and URL @reference inputs should stay diagnosed without Tailwind runtime support: {diagnostics:?}"
    );
}

#[test]
fn css_first_standalone_functions_transform_authored_rules() {
    let theme_css = r#"
@theme {
  --color-brand: oklch(0.65 0.16 250);
}

.badge {
  color: --alpha(var(--color-brand) / 50%);
  margin-inline: --spacing(4);
  width: calc(100% - --spacing(2));
}
"#;

    let diagnostics = css_first_directive_diagnostics(theme_css);
    for directive in ["--alpha()", "--spacing()"] {
        assert!(
            !diagnostics
                .iter()
                .any(|finding| finding.directive == directive),
            "supported standalone {directive} should not be diagnosed: {diagnostics:?}"
        );
    }

    let css =
        StyleEngine::from_theme_css(theme_css).css_authored_function_rules_from_source(theme_css);
    assert!(css.contains(".badge"));
    assert!(css.contains("color: color-mix(in oklab, var(--color-brand) 50%, transparent)"));
    assert!(css.contains("margin-inline: calc(var(--spacing) * 4)"));
    assert!(css.contains("width: calc(100% - calc(var(--spacing) * 2))"));
    assert!(!css.contains("--alpha("));
    assert!(!css.contains("--spacing("));
}

#[test]
fn css_first_variant_directive_supports_stacked_compound_and_custom_variants() {
    let theme_css = r#"
@theme {
  --color-brand: oklch(0.65 0.16 250);
}

@custom-variant theme-midnight (&:where([data-theme="midnight"] *));

.button {
  background: white;
  @variant hover:focus {
    background: black;
  }
  @variant hover, focus {
    color: red;
  }
  @variant md:theme-midnight {
    border-color: var(--color-brand);
  }
  @variant group-hover {
    opacity: 0.8;
  }
}
"#;

    let diagnostics = css_first_directive_diagnostics(theme_css);
    assert!(
        !diagnostics
            .iter()
            .any(|finding| finding.directive == "CSS @variant"),
        "supported CSS @variant syntax should not be diagnosed: {diagnostics:?}"
    );

    let css = StyleEngine::from_theme_css(theme_css).css_variant_rules_from_source(theme_css);
    assert!(css.contains("@media (hover: hover)"), "{css}");
    assert!(css.contains(".button:hover:focus"), "{css}");
    assert!(css.contains("background: black"), "{css}");
    assert!(css.contains(".button:hover"), "{css}");
    assert!(css.contains(".button:focus"), "{css}");
    assert!(css.contains("color: red"), "{css}");
    assert!(css.contains("@media (min-width: 768px)"), "{css}");
    assert!(css.contains(":where([data-theme=\"midnight\"] *)"), "{css}");
    assert!(css.contains("border-color: var(--color-brand)"), "{css}");
    assert!(css.contains(".button:is(:where(.group):hover *)"), "{css}");
    assert!(css.contains("opacity: 0.8"), "{css}");
}

#[test]
fn css_first_variant_directive_expands_safe_layered_arbitrary_authored_css() {
    let theme_css = r#"
@layer components {
  .button {
    color: red;
    @variant hover, focus {
      color: blue;
    }
    @variant md:[&>.icon] {
      opacity: 0.5;
    }
  }
}
"#;

    let diagnostics = css_first_directive_diagnostics(theme_css);
    assert!(
        !diagnostics
            .iter()
            .any(|finding| finding.directive == "CSS @variant"),
        "safe layered arbitrary CSS @variant syntax should not be diagnosed: {diagnostics:?}"
    );

    let css = StyleEngine::from_theme_css(theme_css).css_variant_rules_from_source(theme_css);
    assert!(css.contains("@layer components"), "{css}");
    assert!(css.contains("@media (hover: hover)"), "{css}");
    assert!(css.contains(".button:hover"), "{css}");
    assert!(css.contains(".button:focus"), "{css}");
    assert!(css.contains("color: blue"), "{css}");
    assert!(css.contains("@media (min-width: 768px)"), "{css}");
    assert!(css.contains(".button>.icon"), "{css}");
    assert!(css.contains("opacity: 0.5"), "{css}");
    assert!(!css.contains("@variant"), "{css}");
}

#[test]
fn css_first_custom_variant_block_directive_supports_slot_media_and_selector_forms() {
    let theme_css = r#"
@theme {
  --color-brand: oklch(0.65 0.16 250);
}

@custom-variant theme-midnight {
  &:where([data-theme="midnight"] *) {
    @slot;
  }
}

@custom-variant any-hover {
  @media (any-hover: hover) {
    &:hover {
      @slot;
    }
  }
}

.button {
  @variant any-hover {
    color: red;
  }
}
"#;

    let diagnostics = css_first_directive_diagnostics(theme_css);
    for directive in ["@custom-variant", "CSS @variant"] {
        assert!(
            !diagnostics
                .iter()
                .any(|finding| finding.directive == directive),
            "supported {directive} block syntax should not be diagnosed: {diagnostics:?}"
        );
    }

    let engine = StyleEngine::from_theme_css(theme_css);
    assert_css_contains(
        &engine,
        "theme-midnight:bg-brand",
        &[
            ":where([data-theme=\"midnight\"] *)",
            "background-color: var(--color-brand)",
        ],
    );
    assert_css_contains(
        &engine,
        "any-hover:bg-brand",
        &[
            "@media (any-hover: hover)",
            ":hover",
            "background-color: var(--color-brand)",
        ],
    );

    let css = engine.css_variant_rules_from_source(theme_css);
    assert!(css.contains("@media (any-hover: hover)"), "{css}");
    assert!(css.contains(".button:hover"), "{css}");
    assert!(css.contains("color: red"), "{css}");
}

#[test]
fn css_first_standalone_functions_diagnose_unsafe_values() {
    let theme_css = r#"
.bad {
  color: --alpha(url(javascript:alert(1)) / 50%);
  margin: --spacing(bad; value);
}
"#;

    let diagnostics = css_first_directive_diagnostics(theme_css);
    for directive in ["--alpha()", "--spacing()"] {
        assert!(
            diagnostics.iter().any(|finding| {
                finding.directive == directive
                    && finding.reason.contains("could not be transformed safely")
            }),
            "unsafe standalone {directive} should stay diagnosed: {diagnostics:?}"
        );
    }

    assert!(
        StyleEngine::from_theme_css(theme_css)
            .css_authored_function_rules_from_source(theme_css)
            .is_empty(),
        "unsafe standalone functions should not emit misleading CSS"
    );
}

#[test]
fn css_first_apply_expands_variant_tokens_without_tailwind_runtime() {
    let theme_css = r#"
@theme {
  --color-brand: oklch(0.65 0.16 250);
}

@custom-variant theme-midnight (&:where([data-theme="midnight"] *));

.card {
  @apply hover:bg-brand focus:opacity-100 md:px-4 dark:hover:bg-brand theme-midnight:opacity-50;
}
"#;

    let diagnostics = css_first_directive_diagnostics(theme_css);
    assert!(
        !diagnostics
            .iter()
            .any(|finding| finding.directive == "@apply"),
        "supported variant-bearing @apply should not be diagnosed: {diagnostics:?}"
    );

    let css = StyleEngine::from_theme_css(theme_css).css_apply_rules_from_source(theme_css);
    assert!(css.contains("@media (hover: hover)"), "{css}");
    assert!(css.contains(".card:hover"), "{css}");
    assert!(
        css.contains("background-color: var(--color-brand)"),
        "{css}"
    );
    assert!(css.contains(".card:focus"), "{css}");
    assert!(css.contains("opacity: 100%"), "{css}");
    assert!(css.contains("@media (min-width: 768px)"), "{css}");
    assert!(
        css.contains("padding-inline: calc(var(--spacing) * 4)"),
        "{css}"
    );
    assert!(css.contains(".dark .card:hover"), "{css}");
    assert!(css.contains(":where([data-theme=\"midnight\"] *)"), "{css}");
    assert!(css.contains("opacity: 50%"), "{css}");
    assert!(!css.contains("@apply"), "{css}");
}

#[test]
fn css_first_apply_keeps_unsafe_variant_tokens_diagnosed() {
    let theme_css = r#"
.card {
  @apply [@unknown_rule]:p-4 bg-[url(javascript:alert(1))];
}
"#;

    let diagnostics = css_first_directive_diagnostics(theme_css);
    assert!(
        diagnostics.iter().any(|finding| {
            finding.directive == "@apply" && finding.reason.contains("variant-safe")
        }),
        "unsafe @apply variants should stay explicitly unsupported: {diagnostics:?}"
    );
    assert!(
        StyleEngine::from_theme_css(theme_css)
            .css_apply_rules_from_source(theme_css)
            .is_empty(),
        "unsafe @apply variants should not emit misleading CSS"
    );
}

#[test]
fn source_inline_supports_ranges_and_explicit_inline_exclusions() {
    let theme_css = r#"
@source inline("{hover:,focus:,}bg-brand bg-red-{50,{100..300..100},950}");
@source not inline("{focus:,}bg-brand bg-red-{100..200..100}");
"#;

    let inline_classes = css_source_inline_class_tokens(theme_css);
    for class_name in [
        "bg-brand",
        "hover:bg-brand",
        "focus:bg-brand",
        "bg-red-50",
        "bg-red-100",
        "bg-red-200",
        "bg-red-300",
        "bg-red-950",
    ] {
        assert!(
            inline_classes.contains(&class_name.to_string()),
            "inline @source should brace-expand {class_name}: {inline_classes:?}"
        );
    }

    let excluded_classes = css_source_inline_exclusion_class_tokens(theme_css);
    for class_name in ["bg-brand", "focus:bg-brand", "bg-red-100", "bg-red-200"] {
        assert!(
            excluded_classes.contains(&class_name.to_string()),
            "@source not inline should exclude {class_name}: {excluded_classes:?}"
        );
    }

    assert!(
        !excluded_classes.contains(&"hover:bg-brand".to_string()),
        "empty brace option should only exclude the base class"
    );
}

#[test]
fn source_inline_safelists_and_exclusions_affect_generated_css() {
    use ahash::AHashSet;

    let theme_css = r#"
@theme {
  --color-brand: oklch(0.65 0.16 250);
}

@source inline("{hover:,focus:,}bg-brand p-{2..4..2} underline");
@source not inline("focus:bg-brand p-2");
"#;

    let engine = StyleEngine::from_theme_css(theme_css);
    let mut detected_classes = AHashSet::new();
    detected_classes.insert("focus:bg-brand".to_string());
    detected_classes.insert("p-2".to_string());
    detected_classes.insert("flex".to_string());

    let mut registry = GroupRegistry::default();
    let generated = generate_css(&detected_classes, &mut registry, &engine);
    let css = generated
        .rules
        .iter()
        .map(|rule| rule.css.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    assert!(css.contains(".bg-brand"), "{css}");
    assert!(css.contains(".hover\\:bg-brand:hover"), "{css}");
    assert!(css.contains(".p-4"), "{css}");
    assert!(css.contains(".underline"), "{css}");
    assert!(css.contains(".flex"), "{css}");

    assert!(!css.contains(".focus\\:bg-brand"), "{css}");
    assert!(!css.contains(".p-2"), "{css}");
}

#[test]
fn source_none_disables_automatic_detection_as_dx_owned_scan_policy() {
    let theme_css = r#"
@source none;
@source "../extra";
@source inline("bg-brand");
"#;

    assert!(css_source_disables_automatic_detection(theme_css));

    let directives = css_source_directives(theme_css);
    assert!(
        directives
            .iter()
            .any(|directive| matches!(directive, CssSourceDirective::DisableAutomaticDetection)),
        "@source none should preserve an explicit no-auto-scan directive: {directives:?}"
    );
    assert!(
        directives.iter().any(
            |directive| matches!(directive, CssSourceDirective::Scan(path) if path == "../extra")
        ),
        "explicit @source paths should still be honored when auto detection is disabled: {directives:?}"
    );
    assert_eq!(
        css_source_inline_class_tokens(theme_css),
        vec!["bg-brand".to_string()]
    );
}

#[test]
fn source_scan_plan_classifies_static_paths_none_and_inline_precedence() {
    let theme_css = r#"
@source none;
@source "../app";
@source "../packages/ui";
@source not "../legacy/**";
@source inline("{hover:,focus:,}underline");
@source not inline("focus:underline");
"#;

    let plan = css_source_scan_plan(theme_css);

    assert!(
        plan.disable_automatic_detection,
        "@source none should disable automatic source detection: {plan:?}"
    );
    assert_eq!(
        plan.include_paths,
        vec!["../app".to_string(), "../packages/ui".to_string()]
    );
    assert_eq!(plan.exclude_paths, vec!["../legacy/**".to_string()]);
    assert_eq!(
        plan.inline_classes,
        vec![
            "focus:underline".to_string(),
            "hover:underline".to_string(),
            "underline".to_string()
        ]
    );
    assert_eq!(plan.inline_exclusions, vec!["focus:underline".to_string()]);
    assert_eq!(
        plan.effective_inline_classes(),
        vec!["hover:underline".to_string(), "underline".to_string()]
    );
}

#[test]
fn css_first_functional_utilities_support_v43_defaults_modifiers_and_helpers() {
    let theme_css = r#"
@theme {
  --tab-size-github: 8;
  --color-brand: oklch(0.65 0.16 250);
}

@utility tab-* {
  tab-size: --value(--tab-size-*, integer, [integer], --default(4));
  line-height: --modifier(number, --default(1));
}

@utility glow-* {
  padding-inline: --spacing(--value(integer));
  color: --alpha(var(--color-brand) / --modifier(percentage));
}

@utility cascade-* {
  visibility: --value("inherit", "initial", "unset");
}
"#;

    let engine = StyleEngine::from_theme_css(theme_css);

    assert_css_contains(&engine, "tab", &["tab-size: 4", "line-height: 1"]);
    assert_css_contains(&engine, "tab-2/1.5", &["tab-size: 2", "line-height: 1.5"]);
    assert_css_contains(
        &engine,
        "tab-github",
        &["tab-size: var(--tab-size-github)", "line-height: 1"],
    );
    assert_css_contains(&engine, "tab-[8]", &["tab-size: 8", "line-height: 1"]);
    assert_css_contains(
        &engine,
        "glow-4/45%",
        &[
            "padding-inline: calc(var(--spacing) * 4)",
            "color: color-mix(in oklab, var(--color-brand) 45%, transparent)",
        ],
    );
    assert_css_contains(&engine, "cascade-inherit", &["visibility: inherit"]);

    assert!(engine.css_for_class("tab-auto").is_none());
    let glow_without_modifier = engine
        .css_for_class("glow-4")
        .expect("modifier-dependent declarations should be omitted without dropping value matches");
    assert!(glow_without_modifier.contains("padding-inline: calc(var(--spacing) * 4)"));
    assert!(!glow_without_modifier.contains("color:"));
    assert!(engine.css_for_class("glow-auto/45%").is_none());
    assert!(engine.css_for_class("cascade-revert-layer").is_none());
}

#[test]
fn css_first_supports_nested_utility_selectors_and_reports_layered_blocks() {
    let theme_css = r#"
@theme {
  --color-brand: oklch(62% 0.18 250);
}

@layer utilities {
  @utility layered {
    color: red;
  }
}

@utility scrollbar-hidden {
  scrollbar-width: none;
  &::-webkit-scrollbar {
    display: none;
  }
}

@utility scrollbar-brand {
  scrollbar-color: --alpha(var(--color-brand) / --modifier(percentage)) transparent;
  &::-webkit-scrollbar-thumb {
    background-color: --alpha(var(--color-brand) / --modifier(percentage));
  }
}
"#;

    let diagnostics = css_first_directive_diagnostics(theme_css);

    assert!(
        diagnostics.iter().any(|finding| {
            finding.directive == "@utility"
                && finding.reason.contains("Layered @utility directives")
        }),
        "layered @utility should be diagnosed precisely: {diagnostics:?}"
    );
    assert!(
        !diagnostics
            .iter()
            .any(|finding| finding.reason.contains("Nested @utility rule blocks")),
        "top-level nested @utility selectors should be supported: {diagnostics:?}"
    );

    let engine = StyleEngine::from_theme_css(theme_css);
    assert!(engine.css_for_class("layered").is_none());
    assert_css_contains(
        &engine,
        "scrollbar-hidden",
        &[
            "scrollbar-width: none",
            ".scrollbar-hidden::-webkit-scrollbar",
            "display: none",
        ],
    );
    assert_css_contains(
        &engine,
        "scrollbar-brand/45%",
        &[
            "scrollbar-color: color-mix(in oklab, var(--color-brand) 45%, transparent) transparent",
            ".scrollbar-brand\\/45\\%::-webkit-scrollbar-thumb",
            "background-color: color-mix(in oklab, var(--color-brand) 45%, transparent)",
        ],
    );
}

#[test]
fn logical_spacing_utilities_generate_rtl_friendly_properties() {
    assert_css_contains(
        &StyleEngine::empty(),
        "ps-4",
        &["padding-inline-start: calc(var(--spacing) * 4)"],
    );
    assert_css_contains(
        &StyleEngine::empty(),
        "-ms-2",
        &["margin-inline-start: calc(var(--spacing) * -2)"],
    );
    assert_css_contains(
        &StyleEngine::empty(),
        "start-0",
        &["inset-inline-start: calc(var(--spacing) * 0)"],
    );
}

#[test]
fn unsafe_arbitrary_values_are_rejected_with_safe_values_still_supported() {
    let engine = StyleEngine::empty();

    assert!(
        engine
            .css_for_class("bg-[url(javascript:alert(1))]")
            .is_none()
    );
    assert!(
        engine
            .css_for_class("[background:url(javascript:alert(1))]")
            .is_none()
    );
    assert!(
        engine
            .css_for_class("[color:color-mix(in_oklab,red_50%,transparent)]")
            .is_some()
    );
}

#[test]
fn theme_namespaces_cover_text_transition_filter_and_aspect_tokens() {
    let theme_css = r#"
@theme {
  --text-display: clamp(2rem, 5vw, 4rem);
  --leading-display: 0.95;
  --tracking-tight-ui: -0.02em;
  --ease-enter: cubic-bezier(0.16, 1, 0.3, 1);
  --duration-fast: 120ms;
  --aspect-card: 4 / 3;
  --blur-soft: 18px;
}
"#;
    let engine = StyleEngine::from_theme_css(theme_css);

    assert_css_contains(&engine, "text-display", &["font-size: var(--text-display)"]);
    assert_css_contains(
        &engine,
        "leading-display",
        &["line-height: var(--leading-display)"],
    );
    assert_css_contains(
        &engine,
        "tracking-tight-ui",
        &["letter-spacing: var(--tracking-tight-ui)"],
    );
    assert_css_contains(
        &engine,
        "ease-enter",
        &["transition-timing-function: var(--ease-enter)"],
    );
    assert_css_contains(
        &engine,
        "duration-fast",
        &["transition-duration: var(--duration-fast)"],
    );
    assert_css_contains(
        &engine,
        "aspect-card",
        &["aspect-ratio: var(--aspect-card)"],
    );
    assert_css_contains(&engine, "blur-soft", &["--tw-blur: blur(var(--blur-soft))"]);
}

#[test]
fn high_impact_tailwind_v4_utility_gap_canaries_generate_css() {
    let engine = StyleEngine::empty();

    assert_css_contains(
        &engine,
        "size-8",
        &[
            "width: calc(var(--spacing) * 8)",
            "height: calc(var(--spacing) * 8)",
        ],
    );
    assert_css_contains(&engine, "object-[25%_75%]", &["object-position: 25% 75%"]);
    assert_css_contains(
        &engine,
        "object-(--dx-object-position)",
        &["object-position: var(--dx-object-position)"],
    );
    assert_css_contains(&engine, "origin-top-left", &["transform-origin: top left"]);
    assert_css_contains(
        &engine,
        "skew-x-6",
        &["--tw-skew-x: skewX(6deg)", "transform:"],
    );
    assert_css_contains(
        &engine,
        "-skew-y-3",
        &["--tw-skew-y: skewY(calc(3deg * -1))", "transform:"],
    );
    assert_css_contains(&engine, "flex-[2_1_0%]", &["flex: 2 1 0%"]);
    assert_css_contains(&engine, "grow-[2]", &["flex-grow: 2"]);
    assert_css_contains(&engine, "shrink-[3]", &["flex-shrink: 3"]);
    assert_css_contains(&engine, "-order-1", &["order: -1"]);
    assert_css_contains(
        &engine,
        "grid-cols-(--dx-grid-cols)",
        &["grid-template-columns: var(--dx-grid-cols)"],
    );
    assert_css_contains(
        &engine,
        "auto-rows-(--dx-auto-rows)",
        &["grid-auto-rows: var(--dx-auto-rows)"],
    );
}

#[test]
fn logical_border_radius_and_scroll_utilities_generate_css() {
    let engine = StyleEngine::empty();

    assert_css_contains(&engine, "border-s-2", &["border-inline-start-width: 2px"]);
    assert_css_contains(&engine, "border-e", &["border-inline-end-width: 1px"]);
    assert_css_contains(
        &engine,
        "rounded-s-lg",
        &[
            "border-start-start-radius: var(--radius-lg)",
            "border-end-start-radius: var(--radius-lg)",
        ],
    );
    assert_css_contains(
        &engine,
        "rounded-ee-xl",
        &["border-end-end-radius: var(--radius-xl)"],
    );
    assert_css_contains(
        &engine,
        "scroll-ms-4",
        &["scroll-margin-inline-start: calc(var(--spacing) * 4)"],
    );
    assert_css_contains(
        &engine,
        "scroll-pe-2",
        &["scroll-padding-inline-end: calc(var(--spacing) * 2)"],
    );
}

#[test]
fn filter_and_backdrop_completion_utilities_generate_css() {
    let engine = StyleEngine::empty();

    assert_css_contains(
        &engine,
        "drop-shadow-md",
        &["--tw-drop-shadow: drop-shadow("],
    );
    assert_css_contains(
        &engine,
        "drop-shadow-none",
        &["--tw-drop-shadow:", "filter:"],
    );
    assert_css_contains(
        &engine,
        "backdrop-opacity-50",
        &["--tw-backdrop-opacity: opacity(50%)", "backdrop-filter:"],
    );
    assert_css_contains(
        &engine,
        "backdrop-invert",
        &["--tw-backdrop-invert: invert(100%)"],
    );
    assert_css_contains(
        &engine,
        "backdrop-grayscale",
        &["--tw-backdrop-grayscale: grayscale(100%)"],
    );
    assert_css_contains(
        &engine,
        "backdrop-sepia",
        &["--tw-backdrop-sepia: sepia(100%)"],
    );
}

#[test]
fn third_pass_theme_namespaces_cover_weight_shadow_and_perspective_tokens() {
    let theme_css = r#"
@theme {
  --font-weight-display: 850;
  --drop-shadow-glow: 0 0 20px rgb(59 130 246 / 0.45);
  --inset-shadow-panel: inset 0 1px 0 rgb(255 255 255 / 0.12);
  --perspective-stage: 1200px;
}
"#;
    let engine = StyleEngine::from_theme_css(theme_css);

    assert_css_contains(
        &engine,
        "font-display",
        &["font-weight: var(--font-weight-display)"],
    );
    assert_css_contains(
        &engine,
        "drop-shadow-glow",
        &["--tw-drop-shadow: var(--drop-shadow-glow)", "filter:"],
    );
    assert_css_contains(
        &engine,
        "inset-shadow-panel",
        &["box-shadow: var(--inset-shadow-panel)"],
    );
    assert_css_contains(
        &engine,
        "perspective-stage",
        &["perspective: var(--perspective-stage)"],
    );
}

#[test]
fn third_pass_layout_table_and_3d_transform_utilities_generate_css() {
    let engine = StyleEngine::empty();

    assert_css_contains(&engine, "table-auto", &["table-layout: auto"]);
    assert_css_contains(&engine, "table-fixed", &["table-layout: fixed"]);
    assert_css_contains(&engine, "caption-bottom", &["caption-side: bottom"]);
    assert_css_contains(&engine, "border-collapse", &["border-collapse: collapse"]);
    assert_css_contains(
        &engine,
        "border-spacing-2",
        &["border-spacing: calc(var(--spacing) * 2)"],
    );
    assert_css_contains(
        &engine,
        "border-spacing-x-4",
        &[
            "--tw-border-spacing-x: calc(var(--spacing) * 4)",
            "border-spacing:",
        ],
    );
    assert_css_contains(&engine, "transform-3d", &["transform-style: preserve-3d"]);
    assert_css_contains(&engine, "perspective-[750px]", &["perspective: 750px"]);
    assert_css_contains(
        &engine,
        "perspective-origin-top-right",
        &["perspective-origin: top right"],
    );
    assert_css_contains(
        &engine,
        "rotate-x-45",
        &["--tw-rotate-x: rotateX(45deg)", "transform:"],
    );
    assert_css_contains(
        &engine,
        "-rotate-y-12",
        &["--tw-rotate-y: rotateY(calc(12deg * -1))", "transform:"],
    );
    assert_css_contains(
        &engine,
        "translate-z-4",
        &[
            "--tw-translate-z: calc(var(--spacing) * 4)",
            "translate: var(--tw-translate-x) var(--tw-translate-y) var(--tw-translate-z)",
        ],
    );
    assert_css_contains(
        &engine,
        "scale-z-125",
        &[
            "--tw-scale-z: 125%",
            "scale: var(--tw-scale-x) var(--tw-scale-y) var(--tw-scale-z)",
        ],
    );
    assert_css_contains(
        &engine,
        "inset-shadow-[inset_0_1px_2px_rgb(0_0_0_/_0.1)]",
        &["box-shadow: inset 0 1px 2px rgb(0 0 0 / 0.1)"],
    );
    assert_css_contains(
        &engine,
        "shadow-(--dx-shadow)",
        &["box-shadow: var(--dx-shadow)"],
    );
}

#[test]
fn fourth_pass_arbitrary_color_box_and_custom_property_aliases_generate_css() {
    let engine = StyleEngine::empty();

    assert_css_contains(&engine, "box-border", &["box-sizing: border-box"]);
    assert_css_contains(&engine, "box-content", &["box-sizing: content-box"]);
    assert_css_contains(
        &engine,
        "bg-[#1d4ed8]/50",
        &["background-color: color-mix(in oklab, #1d4ed8 50%, transparent)"],
    );
    assert_css_contains(
        &engine,
        "bg-(--dx-background)",
        &["background-color: var(--dx-background)"],
    );
    assert_css_contains(
        &engine,
        "border-(--dx-border)",
        &["border-color: var(--dx-border)"],
    );
    assert_css_contains(
        &engine,
        "fill-[#0f172a]/80",
        &["fill: color-mix(in oklab, #0f172a 80%, transparent)"],
    );
    assert_css_contains(
        &engine,
        "text-[color:var(--dx-foreground)]",
        &["color: var(--dx-foreground)"],
    );
    assert_css_contains(
        &engine,
        "stroke-[color:var(--dx-stroke)]",
        &["stroke: var(--dx-stroke)"],
    );
    assert_css_contains(
        &engine,
        "rounded-(--dx-radius)",
        &["border-radius: var(--dx-radius)"],
    );
    assert_css_contains(
        &engine,
        "aspect-(--dx-aspect)",
        &["aspect-ratio: var(--dx-aspect)"],
    );
    assert_css_contains(
        &engine,
        "leading-(--dx-leading)",
        &["line-height: var(--dx-leading)"],
    );
    assert_css_contains(
        &engine,
        "tracking-(--dx-tracking)",
        &["letter-spacing: var(--dx-tracking)"],
    );
}

#[test]
fn fifth_pass_alignment_gradient_stop_and_color_alias_utilities_generate_css() {
    let engine = StyleEngine::empty();

    assert_css_contains(
        &engine,
        "items-baseline-last",
        &["align-items: last baseline"],
    );
    assert_css_contains(&engine, "self-baseline", &["align-self: baseline"]);
    assert_css_contains(&engine, "place-items-baseline", &["place-items: baseline"]);
    assert_css_contains(&engine, "justify-items-center", &["justify-items: center"]);
    assert_css_contains(&engine, "justify-self-end", &["justify-self: end"]);
    assert_css_contains(&engine, "content-normal", &["align-content: normal"]);
    assert_css_contains(&engine, "from-10%", &["--tw-gradient-from-position: 10%"]);
    assert_css_contains(&engine, "via-30%", &["--tw-gradient-via-position: 30%"]);
    assert_css_contains(&engine, "to-90%", &["--tw-gradient-to-position: 90%"]);
    assert_css_contains(
        &engine,
        "from-(--dx-gradient-from-position)",
        &["--tw-gradient-from-position: var(--dx-gradient-from-position)"],
    );
    assert_css_contains(
        &engine,
        "ring-[#2563eb]/50",
        &["--tw-ring-color: color-mix(in oklab, #2563eb 50%, transparent)"],
    );
    assert_css_contains(
        &engine,
        "divide-(--dx-border)",
        &["border-color: var(--dx-border)"],
    );
}

#[test]
fn tailwind_v43_neutral_palettes_generate_token_backed_color_utilities() {
    let engine = StyleEngine::empty();
    let default_theme = engine
        .theme_by_name("default-theme-css")
        .expect("default dx-style theme should be registered");

    for (token, value) in [
        ("--color-mauve-500", "oklch(54.2% 0.034 322.5)"),
        ("--color-olive-500", "oklch(58% 0.031 107.3)"),
        ("--color-mist-500", "oklch(56% 0.021 213.5)"),
        ("--color-taupe-500", "oklch(54.7% 0.021 43.1)"),
    ] {
        assert!(
            default_theme
                .tokens
                .iter()
                .any(|(name, token_value)| name == token && token_value == value),
            "default theme should expose {token}: {value}"
        );
    }

    assert_css_contains(
        &engine,
        "bg-mauve-500",
        &["background-color: var(--color-mauve-500)"],
    );
    assert_css_contains(
        &engine,
        "text-olive-600/75",
        &["color: color-mix(in oklab, var(--color-olive-600) 75%, transparent)"],
    );
    assert_css_contains(
        &engine,
        "border-mist-300",
        &["border-color: var(--color-mist-300)"],
    );
    assert_css_contains(
        &engine,
        "ring-taupe-400/50",
        &["--tw-ring-color: color-mix(in oklab, var(--color-taupe-400) 50%, transparent)"],
    );
    assert_css_contains(
        &engine,
        "outline-mauve-700",
        &["outline-color: var(--color-mauve-700)"],
    );
    assert_css_contains(
        &engine,
        "decoration-olive-500",
        &["text-decoration-color: var(--color-olive-500)"],
    );
    assert_css_contains(
        &engine,
        "from-mist-500",
        &["--tw-gradient-from: var(--color-mist-500)"],
    );
    assert_css_contains(
        &engine,
        "via-taupe-500/40",
        &["color-mix(in oklab, var(--color-taupe-500) 40%, transparent)"],
    );
    assert_css_contains(
        &engine,
        "to-mauve-950",
        &["--tw-gradient-to: var(--color-mauve-950)"],
    );
}

#[test]
fn sixth_pass_custom_property_shorthand_utilities_generate_css() {
    let engine = StyleEngine::empty();

    assert_css_contains(&engine, "p-(--dx-pad)", &["padding: var(--dx-pad)"]);
    assert_css_contains(
        &engine,
        "scroll-mt-(--dx-scroll-mt)",
        &["scroll-margin-top: var(--dx-scroll-mt)"],
    );
    assert_css_contains(
        &engine,
        "opacity-(--dx-opacity)",
        &["opacity: var(--dx-opacity)"],
    );
    assert_css_contains(&engine, "z-(--dx-layer)", &["z-index: var(--dx-layer)"]);
    assert_css_contains(&engine, "order-(--dx-order)", &["order: var(--dx-order)"]);
    assert_css_contains(
        &engine,
        "blur-(--dx-blur)",
        &["--tw-blur: blur(var(--dx-blur))", "filter:"],
    );
    assert_css_contains(
        &engine,
        "brightness-(--dx-brightness)",
        &[
            "--tw-brightness: brightness(var(--dx-brightness))",
            "filter:",
        ],
    );
    assert_css_contains(
        &engine,
        "hue-rotate-(--dx-hue-rotate)",
        &[
            "--tw-hue-rotate: hue-rotate(var(--dx-hue-rotate))",
            "filter:",
        ],
    );
    assert_css_contains(
        &engine,
        "backdrop-opacity-(--dx-backdrop-opacity)",
        &[
            "--tw-backdrop-opacity: opacity(var(--dx-backdrop-opacity))",
            "backdrop-filter:",
        ],
    );
    assert_css_contains(
        &engine,
        "backdrop-blur-(--dx-backdrop-blur)",
        &[
            "--tw-backdrop-blur: blur(var(--dx-backdrop-blur))",
            "backdrop-filter:",
        ],
    );
    assert_css_contains(
        &engine,
        "outline-offset-(--dx-outline-offset)",
        &["outline-offset: var(--dx-outline-offset)"],
    );
    assert_css_contains(&engine, "outline-[3px]", &["outline-width: 3px"]);
    assert_css_contains(
        &engine,
        "ring-[3px]",
        &["calc(3px + var(--tw-ring-offset-width, 0px))"],
    );
    assert_css_contains(
        &engine,
        "ring-offset-(--dx-ring-offset-width)",
        &["--tw-ring-offset-width: var(--dx-ring-offset-width)"],
    );
}
