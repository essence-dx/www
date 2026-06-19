use style::core::StyleEngine;

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
fn typography_font_size_modifiers_and_theme_companion_tokens_generate_css() {
    let engine = StyleEngine::empty();

    assert_css_contains(
        &engine,
        "text-sm/6",
        &[
            "font-size: var(--text-sm)",
            "line-height: calc(var(--spacing) * 6)",
        ],
    );
    assert_css_contains(
        &engine,
        "text-lg/[1.7]",
        &["font-size: var(--text-lg)", "line-height: 1.7"],
    );
    assert_css_contains(
        &engine,
        "text-(length:--dx-text-size)/(--dx-leading)",
        &[
            "font-size: var(--dx-text-size)",
            "line-height: var(--dx-leading)",
        ],
    );

    let theme_css = r#"
@theme {
  --font-display: "Oswald", sans-serif;
  --font-display--font-feature-settings: "cv02", "cv03", "cv04", "cv11";
  --font-display--font-variation-settings: "opsz" 32;
  --text-tiny: 0.625rem;
  --text-tiny--line-height: 1.5;
  --text-tiny--letter-spacing: 0.025em;
  --text-tiny--font-weight: 500;
}
"#;
    let engine = StyleEngine::from_theme_css(theme_css);

    assert_css_contains(
        &engine,
        "font-display",
        &[
            "font-family: var(--font-display)",
            "font-feature-settings: var(--font-display--font-feature-settings)",
            "font-variation-settings: var(--font-display--font-variation-settings)",
        ],
    );
    assert_css_contains(
        &engine,
        "text-tiny",
        &[
            "font-size: var(--text-tiny)",
            "line-height: var(--text-tiny--line-height)",
            "letter-spacing: var(--text-tiny--letter-spacing)",
            "font-weight: var(--text-tiny--font-weight)",
        ],
    );

    assert!(
        engine
            .css_for_class("font-display--font-feature-settings")
            .is_none()
    );
    assert!(
        engine
            .css_for_class("font-display--font-variation-settings")
            .is_none()
    );
    assert!(engine.css_for_class("text-tiny--line-height").is_none());
    assert!(engine.css_for_class("text-tiny--letter-spacing").is_none());
    assert!(engine.css_for_class("text-tiny--font-weight").is_none());
}

#[test]
fn text_shadow_value_opacity_modifiers_and_typed_arbitrary_values_generate_css() {
    let engine = StyleEngine::empty();

    assert_css_contains(
        &engine,
        "text-shadow-lg/20",
        &[
            "--tw-text-shadow-alpha: 20%",
            "text-shadow: 0px 1px 2px var(--tw-text-shadow-color, oklab(from rgb(0 0 0 / 0.1) l a b / 20%))",
        ],
    );
    let named_shadow = engine
        .css_for_class("text-shadow-lg/20")
        .expect("text-shadow-lg/20 should generate CSS");
    assert!(
        !named_shadow.contains("var(--tw-text-shadow-color, var(--tw-text-shadow-color"),
        "{named_shadow}"
    );
    assert_css_contains(
        &engine,
        "text-shadow-[0_35px_35px_rgb(0_0_0_/_0.25)]/50",
        &[
            "--tw-text-shadow-alpha: 50%",
            "text-shadow: 0 35px 35px var(--tw-text-shadow-color, oklab(from rgb(0 0 0 / 0.25) l a b / 50%))",
        ],
    );
    assert_css_contains(
        &engine,
        "text-shadow-[10px_10px]/25",
        &[
            "--tw-text-shadow-alpha: 25%",
            "text-shadow: 10px 10px var(--tw-text-shadow-color, currentcolor)",
            "@supports (color: color-mix(in lab, red, red))",
            "text-shadow: 10px 10px var(--tw-text-shadow-color, color-mix(in oklab, currentcolor 25%, transparent))",
        ],
    );

    let typed_shadow = engine
        .css_for_class("text-shadow-[shadow:var(--dx-text-shadow)]")
        .expect("typed arbitrary shadow value should generate CSS");
    assert!(typed_shadow.contains("text-shadow: var(--dx-text-shadow)"));
    assert!(!typed_shadow.contains("shadow:var(--dx-text-shadow)"));
}
