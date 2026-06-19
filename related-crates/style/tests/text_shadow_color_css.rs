use style::core::StyleEngine;

fn css_for(class_name: &str) -> String {
    StyleEngine::empty()
        .css_for_class(class_name)
        .unwrap_or_else(|| panic!("{class_name} should generate CSS"))
}

#[test]
fn text_shadow_color_utilities_generate_color_variable_css() {
    let css = css_for("text-shadow-sky-300/50");

    assert!(css.contains(".text-shadow-sky-300\\/50"));
    assert!(css.contains("--tw-text-shadow-color: rgb(125 211 252 / 0.5);"));
}

#[test]
fn text_shadow_sizes_read_the_color_variable_with_fallbacks() {
    let css = css_for("text-shadow-sm");

    assert!(css.contains("text-shadow:"));
    assert!(css.contains("var(--tw-text-shadow-color, rgb(0 0 0 / 0.075))"));
}

#[test]
fn text_shadow_arbitrary_colors_support_opacity_modifiers() {
    let css = css_for("text-shadow-[color:oklch(70%_0.17_240)]/40");

    assert!(css.contains("--tw-text-shadow-color:"));
    assert!(css.contains("color-mix(in oklab, oklch(70% 0.17 240) 40%, transparent)"));
}

#[test]
fn text_shadow_color_custom_property_shorthand_generates_variable_css() {
    let css = css_for("text-shadow-(color:--dx-text-shadow-color)");

    assert!(css.contains("--tw-text-shadow-color: var(--dx-text-shadow-color);"));
}

#[test]
fn stacked_not_variant_text_shadow_colors_generate_nested_selector_css() {
    let css = css_for("hover:not-focus:text-shadow-cyan-500/50");

    assert!(css.contains(":hover:not(:focus)"));
    assert!(css.contains("--tw-text-shadow-color: rgb(6 182 212 / 0.5);"));
}
