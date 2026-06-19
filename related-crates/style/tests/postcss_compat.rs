use style::core::{
    CssImportSource, DxStyleBrowserTarget, PostcssCompatOptions, PostcssCompatStatus,
    postcss_compat_matrix, transform_postcss_compatible_css,
};

fn legacy_options() -> PostcssCompatOptions {
    PostcssCompatOptions {
        source_path: "app/styles.css".to_string(),
        target: DxStyleBrowserTarget::Legacy,
        imports: vec![CssImportSource {
            specifier: "./tokens.css".to_string(),
            source_path: "styles/tokens.css".to_string(),
            css: ":root { --space-4: 1rem; }\n".to_string(),
        }],
        minify: false,
    }
}

#[test]
fn postcss_compat_matrix_covers_required_feature_groups() {
    let matrix = postcss_compat_matrix();
    let features = matrix
        .iter()
        .map(|entry| entry.feature)
        .collect::<std::collections::BTreeSet<_>>();

    for feature in [
        "css-import-flattening",
        "nesting-transform",
        "nested-at-rule-transform",
        "custom-media",
        "compound-custom-media",
        "media-min-max-syntax",
        "custom-selectors-safe",
        "logical-property-fallbacks",
        "autoprefixer-style-prefixing",
        "preset-env-future-css",
        "place-property-fallbacks",
        "image-set-prefix-fallback",
        "custom-property-var-fallbacks",
        "hwb-color-fallbacks",
        "not-selector-list-lowering",
        "nest-at-rule-transform",
        "strict-media-range-syntax",
        "mixed-media-range-syntax",
        "logical-directional-fallbacks",
        "color-mix-fallbacks",
        "gradient-transparent-stop-fix",
        "grid-template-prefix-evidence",
        "selector-compat-diagnostics",
        "color-function-fallbacks",
        "gradient-transparency-compat",
        "page-break-fallbacks",
        "flex-grid-prefix-evidence",
        "sourcemap-source-origin",
        "minification",
    ] {
        assert!(
            features.contains(feature),
            "missing feature group {feature}"
        );
    }

    assert!(
        matrix
            .iter()
            .any(|entry| entry.status == PostcssCompatStatus::Supported)
    );
    assert!(
        matrix
            .iter()
            .any(|entry| entry.status == PostcssCompatStatus::Partial)
    );
    assert!(
        matrix.iter().all(|entry| !entry.input_css.trim().is_empty()
            && !entry.expected_output_css.trim().is_empty())
    );
}

#[test]
fn default_and_legacy_targets_are_receipted_without_postcss_dependency() {
    let input = ".button { user-select: none; appearance: none; backdrop-filter: blur(8px); }\n";
    let modern = transform_postcss_compatible_css(
        input,
        &PostcssCompatOptions {
            source_path: "app/styles.css".to_string(),
            target: DxStyleBrowserTarget::Modern,
            imports: Vec::new(),
            minify: false,
        },
    )
    .expect("modern compatibility transform");
    let legacy = transform_postcss_compatible_css(input, &legacy_options())
        .expect("legacy compatibility transform");

    assert_eq!(modern.receipt.selected_target, "modern");
    assert_eq!(legacy.receipt.selected_target, "legacy");
    assert!(
        modern
            .receipt
            .target_browsers
            .iter()
            .any(|browser| browser == "chrome >= 109")
    );
    assert!(
        legacy
            .receipt
            .target_browsers
            .iter()
            .any(|browser| browser == "safari >= 12")
    );
    assert_eq!(modern.receipt.postcss_runtime_dependency_required, false);
    assert_eq!(modern.receipt.local_postcss_config_required, false);
    assert_eq!(modern.receipt.autoprefixer_parity_status, "partial");
    assert_eq!(legacy.receipt.autoprefixer_parity_status, "partial");
}

#[test]
fn prefix_output_is_generated_for_measured_autoprefixer_canaries() {
    let input = r#"
.panel {
  user-select: none;
  appearance: none;
  backdrop-filter: blur(12px);
  position: sticky;
  display: flex;
  break-before: page;
  break-inside: avoid;
}

.grid { display: grid; }
"#;

    let result = transform_postcss_compatible_css(input, &legacy_options())
        .expect("legacy prefix transform");

    assert!(result.css.contains("-webkit-user-select: none;"));
    assert!(result.css.contains("-moz-user-select: none;"));
    assert!(result.css.contains("-ms-user-select: none;"));
    assert!(result.css.contains("user-select: none;"));
    assert!(result.css.contains("-webkit-appearance: none;"));
    assert!(result.css.contains("-moz-appearance: none;"));
    assert!(result.css.contains("appearance: none;"));
    assert!(result.css.contains("-webkit-backdrop-filter: blur(12px);"));
    assert!(result.css.contains("backdrop-filter: blur(12px);"));
    assert!(result.css.contains("position: -webkit-sticky;"));
    assert!(result.css.contains("display: -webkit-box;"));
    assert!(result.css.contains("display: -ms-flexbox;"));
    assert!(result.css.contains("display: flex;"));
    assert!(result.css.contains("page-break-before: always;"));
    assert!(result.css.contains("break-before: page;"));
    assert!(result.css.contains("page-break-inside: avoid;"));
    assert!(result.css.contains("break-inside: avoid;"));
    assert!(
        result
            .receipt
            .unsupported_transform_warnings
            .iter()
            .any(|warning| warning.contains("grid"))
    );
}

#[test]
fn imports_nesting_custom_media_and_custom_selectors_transform_together() {
    let input = r#"
@import "./tokens.css";
@custom-media --narrow (width <= 40rem);
@custom-selector :--control button, .button;

.card {
  color: rgb(255 0 0 / 50%);
  & :--control {
    margin-inline: var(--space-4);
  }
}

@media (--narrow) {
  .card {
    & .title {
      color: blue;
    }
  }
}
"#;

    let result =
        transform_postcss_compatible_css(input, &legacy_options()).expect("compat transform");

    assert!(result.css.contains(":root {"));
    assert!(!result.css.contains("@import"));
    assert!(result.css.contains("@media (max-width: 40rem)"));
    assert!(result.css.contains(".card button, .card .button {"));
    assert!(result.css.contains("margin-left: var(--space-4);"));
    assert!(result.css.contains("margin-right: var(--space-4);"));
    assert!(result.css.contains("margin-inline: var(--space-4);"));
    assert!(result.css.contains(".card .title {"));
    assert!(result.css.contains("rgba(255, 0, 0, 0.5);"));
    assert_eq!(
        result.receipt.source_origins[0].source_path,
        "styles/tokens.css"
    );
    assert_eq!(
        result.receipt.source_origins[1].source_path,
        "app/styles.css"
    );
    assert!(
        result
            .receipt
            .source_map
            .sources
            .iter()
            .any(|source| source == "styles/tokens.css")
    );
    assert!(
        result
            .receipt
            .source_map
            .sources
            .iter()
            .any(|source| source == "app/styles.css")
    );
}

#[test]
fn compatibility_diagnostics_record_unsupported_selector_color_and_gradient_cases() {
    let input = r#"
.gallery:has(img) {
  background: linear-gradient(to right, red, transparent);
  color: oklch(62% 0.2 25);
}
"#;

    let result =
        transform_postcss_compatible_css(input, &legacy_options()).expect("diagnostic transform");

    assert!(result.css.contains(".gallery:has(img)"));
    assert!(
        result
            .receipt
            .unsupported_transform_warnings
            .iter()
            .any(|warning| warning.contains(":has()"))
    );
    assert!(
        result
            .receipt
            .unsupported_transform_warnings
            .iter()
            .any(|warning| warning.contains("oklch"))
    );
    assert!(
        result
            .receipt
            .unsupported_transform_warnings
            .iter()
            .any(|warning| warning.contains("transparent gradient"))
    );
}

#[test]
fn starter_preset_env_canaries_emit_fallback_css_without_postcss() {
    let input = r#"
:root {
  --brand: #1d4ed8;
}

.button:is(.primary, .secondary) {
  color: var(--brand);
  border-color: #336699cc;
  background: hsl(210 50% 40% / 75%);
  padding-top: env(safe-area-inset-top, 1rem);
  margin-block: 2rem;
  padding-block: 1rem;
}

.button:where(:focus, :focus-visible) {
  outline: 2px solid var(--brand);
}

@media (40rem <= width <= 72rem) {
  .button {
    display: block;
  }
}
"#;

    let result =
        transform_postcss_compatible_css(input, &legacy_options()).expect("preset-env canaries");

    assert!(result.css.contains(".button.primary, .button.secondary {"));
    assert!(
        result
            .css
            .contains(".button:focus, .button:focus-visible {")
    );
    assert!(result.css.contains("color: #1d4ed8;"));
    assert!(result.css.contains("color: var(--brand);"));
    assert!(result.css.contains("rgba(51, 102, 153, 0.8);"));
    assert!(result.css.contains("hsla(210, 50%, 40%, 0.75);"));
    assert!(result.css.contains("padding-top: 1rem;"));
    assert!(result.css.contains("margin-top: 2rem;"));
    assert!(result.css.contains("margin-bottom: 2rem;"));
    assert!(result.css.contains("padding-top: 1rem;"));
    assert!(result.css.contains("padding-bottom: 1rem;"));
    assert!(
        result
            .css
            .contains("@media (min-width: 40rem) and (max-width: 72rem)")
    );
}

#[test]
fn custom_property_var_fallbacks_substitute_full_declaration_values() {
    let input = r#"
:root {
  --brand: #1d4ed8;
  --shadow: 0 1px 2px rgb(0 0 0 / 20%);
}

.card {
  color: var(--missing-fg, #111827);
  border: 1px solid var(--brand);
  box-shadow: var(--shadow, none);
  padding: var(--space, calc(1rem + 2px));
}
"#;

    let result = transform_postcss_compatible_css(input, &legacy_options()).expect("var fallbacks");

    assert!(result.css.contains("color: #111827;"));
    assert!(result.css.contains("color: var(--missing-fg, #111827);"));
    assert!(result.css.contains("border: 1px solid #1d4ed8;"));
    assert!(result.css.contains("border: 1px solid var(--brand);"));
    assert!(
        result
            .css
            .contains("box-shadow: 0 1px 2px rgb(0 0 0 / 20%);")
    );
    assert!(result.css.contains("box-shadow: var(--shadow, none);"));
    assert!(result.css.contains("padding: calc(1rem + 2px);"));
    assert!(
        result
            .css
            .contains("padding: var(--space, calc(1rem + 2px));")
    );
    assert!(!result.css.contains("border: #1d4ed8;"));
}

#[test]
fn hwb_color_fallbacks_emit_legacy_rgb_css() {
    let input = r#"
.swatch {
  color: hwb(210 20% 30% / 75%);
}
"#;

    let result = transform_postcss_compatible_css(input, &legacy_options()).expect("hwb fallback");

    assert!(result.css.contains("color: rgba(51, 115, 179, 0.75);"));
    assert!(result.css.contains("color: hwb(210 20% 30% / 75%);"));
}

#[test]
fn deeper_postcss_compat_canaries_emit_safe_fallback_css_without_plugins() {
    let input = r#"
.button:not(.disabled, [aria-disabled="true"]) {
  background-color: color-mix(in srgb, #000000 25%, #ffffff);
  background-image: linear-gradient(to right, red, transparent);
}

.layout {
  display: grid;
  grid-template-columns: 12rem 1fr;
  grid-template-rows: auto 1fr;
}
"#;

    let result =
        transform_postcss_compatible_css(input, &legacy_options()).expect("deeper canaries");

    assert!(
        result
            .css
            .contains(r#".button:not(.disabled):not([aria-disabled="true"]) {"#)
    );
    assert!(result.css.contains("background-color: rgb(191, 191, 191);"));
    assert!(
        result
            .css
            .contains("background-color: color-mix(in srgb, #000000 25%, #ffffff);")
    );
    assert!(
        result
            .css
            .contains("background-image: linear-gradient(to right, red, rgba(255, 0, 0, 0));")
    );
    assert!(
        result
            .css
            .contains("background-image: linear-gradient(to right, red, transparent);")
    );
    assert!(result.css.contains("display: -ms-grid;"));
    assert!(result.css.contains("-ms-grid-columns: 12rem 1fr;"));
    assert!(result.css.contains("-ms-grid-rows: auto 1fr;"));
    assert!(
        result
            .receipt
            .unsupported_transform_warnings
            .iter()
            .any(|warning| warning.contains("grid"))
    );
}

#[test]
fn nesting_strict_media_and_logical_directional_canaries_emit_fallback_css() {
    let input = r#"
.card {
  color: red;
  @nest .theme & {
    color: blue;
  }
}

@media (width > 48rem) {
  .card {
    display: block;
  }
}

@media (height < 40rem) {
  .card {
    display: none;
  }
}

.logical {
  margin-inline-start: 1rem;
  padding-inline-end: 2rem;
  inset-inline-start: 0;
  border-inline-start: 1px solid red;
  text-align: start;
}
"#;

    let result =
        transform_postcss_compatible_css(input, &legacy_options()).expect("logical canaries");

    assert!(result.css.contains(".theme .card {"));
    assert!(result.css.contains("color: blue;"));
    assert!(
        result
            .css
            .contains("@media (min-width: calc(48rem + 0.02px))")
    );
    assert!(
        result
            .css
            .contains("@media (max-height: calc(40rem - 0.02px))")
    );
    assert!(result.css.contains("margin-left: 1rem;"));
    assert!(result.css.contains("margin-inline-start: 1rem;"));
    assert!(result.css.contains("padding-right: 2rem;"));
    assert!(result.css.contains("padding-inline-end: 2rem;"));
    assert!(result.css.contains("left: 0;"));
    assert!(result.css.contains("inset-inline-start: 0;"));
    assert!(result.css.contains("border-left: 1px solid red;"));
    assert!(result.css.contains("border-inline-start: 1px solid red;"));
    assert!(result.css.contains("text-align: left;"));
    assert!(result.css.contains("text-align: start;"));
    assert!(
        result
            .receipt
            .unsupported_transform_warnings
            .iter()
            .any(|warning| warning.contains("logical directional fallback"))
    );
}

#[test]
fn mixed_media_range_syntax_emits_min_max_queries() {
    let input = r#"
@media (48rem < width <= 72rem) {
  .card {
    display: block;
  }
}

@media (30rem <= height < 50rem) {
  .card {
    display: none;
  }
}
"#;

    let result =
        transform_postcss_compatible_css(input, &legacy_options()).expect("mixed media ranges");

    assert!(
        result
            .css
            .contains("@media (min-width: calc(48rem + 0.02px)) and (max-width: 72rem)")
    );
    assert!(
        result
            .css
            .contains("@media (min-height: 30rem) and (max-height: calc(50rem - 0.02px))")
    );
    assert!(!result.css.contains("48rem < width <= 72rem"));
    assert!(!result.css.contains("30rem <= height < 50rem"));
}

#[test]
fn compound_custom_media_place_and_image_set_canaries_emit_css() {
    let input = r#"
@custom-media --wide (width >= 64rem);

@media screen and (--wide) {
  .hero {
    place-items: center start;
    place-content: space-between center;
    place-self: stretch end;
    background-image: image-set(url("hero.avif") type("image/avif") 1x, url("hero.png") type("image/png") 1x);
  }
}
"#;

    let result =
        transform_postcss_compatible_css(input, &legacy_options()).expect("preset canaries");

    assert!(result.css.contains("@media screen and (min-width: 64rem)"));
    assert!(result.css.contains("align-items: center;"));
    assert!(result.css.contains("justify-items: start;"));
    assert!(result.css.contains("place-items: center start;"));
    assert!(result.css.contains("align-content: space-between;"));
    assert!(result.css.contains("justify-content: center;"));
    assert!(result.css.contains("place-content: space-between center;"));
    assert!(result.css.contains("align-self: stretch;"));
    assert!(result.css.contains("justify-self: end;"));
    assert!(result.css.contains("place-self: stretch end;"));
    assert!(
        result
            .css
            .contains("background-image: -webkit-image-set(url(\"hero.avif\") type(\"image/avif\") 1x, url(\"hero.png\") type(\"image/png\") 1x);")
    );
    assert!(
        result
            .css
            .contains("background-image: image-set(url(\"hero.avif\") type(\"image/avif\") 1x, url(\"hero.png\") type(\"image/png\") 1x);")
    );
}

#[test]
fn nested_media_and_supports_rules_inside_selectors_emit_wrapped_css() {
    let input = r#"
.card {
  color: red;

  @media (width >= 48rem) {
    color: blue;

    & .title {
      color: green;
    }
  }

  @supports (display: grid) {
    display: grid;
  }
}
"#;

    let result =
        transform_postcss_compatible_css(input, &legacy_options()).expect("nested at-rules");

    assert!(result.css.contains(".card {\n  color: red;\n}"));
    assert!(result.css.contains("@media (min-width: 48rem) {"));
    assert!(result.css.contains("  .card {\n    color: blue;\n  }"));
    assert!(
        result
            .css
            .contains("  .card .title {\n    color: green;\n  }")
    );
    assert!(result.css.contains("@supports (display: grid) {"));
    assert!(
        result
            .css
            .contains("  .card {\n    display: -ms-grid;\n    display: grid;\n  }")
    );
    assert!(!result.css.contains(".card @media"));
    assert!(!result.css.contains(".card @supports"));
}

#[test]
fn autoprefixer_style_legacy_matrix_expands_more_prefix_families() {
    let input = r#"
.panel {
  display: inline-flex;
  flex-direction: column;
  flex-wrap: wrap;
  align-items: center;
  justify-content: center;
  align-self: stretch;
  flex: 1 1 auto;
  order: 2;
  hyphens: auto;
  text-size-adjust: 100%;
  print-color-adjust: exact;
  mask-image: linear-gradient(black, transparent);
  clip-path: inset(0 round 8px);
  backface-visibility: hidden;
}
"#;

    let result =
        transform_postcss_compatible_css(input, &legacy_options()).expect("prefix family canaries");

    assert!(result.css.contains("display: -webkit-inline-box;"));
    assert!(result.css.contains("display: -ms-inline-flexbox;"));
    assert!(result.css.contains("-webkit-flex-direction: column;"));
    assert!(result.css.contains("-ms-flex-direction: column;"));
    assert!(result.css.contains("-webkit-flex-wrap: wrap;"));
    assert!(result.css.contains("-ms-flex-wrap: wrap;"));
    assert!(result.css.contains("-webkit-align-items: center;"));
    assert!(result.css.contains("-ms-flex-align: center;"));
    assert!(result.css.contains("-webkit-justify-content: center;"));
    assert!(result.css.contains("-ms-flex-pack: center;"));
    assert!(result.css.contains("-webkit-align-self: stretch;"));
    assert!(result.css.contains("-ms-flex-item-align: stretch;"));
    assert!(result.css.contains("-webkit-flex: 1 1 auto;"));
    assert!(result.css.contains("-ms-flex: 1 1 auto;"));
    assert!(result.css.contains("-webkit-order: 2;"));
    assert!(result.css.contains("-ms-flex-order: 2;"));
    assert!(result.css.contains("-webkit-hyphens: auto;"));
    assert!(result.css.contains("-webkit-text-size-adjust: 100%;"));
    assert!(result.css.contains("-webkit-print-color-adjust: exact;"));
    assert!(
        result
            .css
            .contains("-webkit-mask-image: linear-gradient(black, transparent);")
    );
    assert!(
        result
            .css
            .contains("-webkit-clip-path: inset(0 round 8px);")
    );
    assert!(result.css.contains("-webkit-backface-visibility: hidden;"));
}

#[test]
fn starter_replacement_score_is_complete_without_full_postcss_overclaim() {
    let result = transform_postcss_compatible_css(".button { color: red; }\n", &legacy_options())
        .expect("receipt");

    assert_eq!(result.receipt.dx_starter_replacement_score, 100);
    assert_eq!(
        result.receipt.dx_starter_replacement_status,
        "complete-for-official-dx-starters"
    );
    assert_eq!(result.receipt.full_postcss_plugin_parity, false);
    assert_eq!(result.receipt.postcss_plugin_parity_status, "not-claimed");
    assert_eq!(result.receipt.autoprefixer_parity_status, "partial");
}
