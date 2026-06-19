use style::core::StyleEngine;

fn css_for(class_name: &str) -> String {
    StyleEngine::empty()
        .css_for_class(class_name)
        .unwrap_or_else(|| panic!("{class_name} should generate CSS"))
}

#[test]
fn linear_direction_gradients_support_interpolation_modifiers() {
    let oklch = css_for("bg-linear-to-r/oklch");
    assert!(oklch.contains(
        "background-image: linear-gradient(to right in oklch, var(--tw-gradient-stops));"
    ));

    let longer = css_for("bg-linear-to-r/longer");
    assert!(longer.contains(
        "background-image: linear-gradient(to right in oklch longer hue, var(--tw-gradient-stops));"
    ));
}

#[test]
fn linear_angle_gradients_support_interpolation_modifiers() {
    let srgb = css_for("bg-linear-45/srgb");
    assert!(
        srgb.contains(
            "background-image: linear-gradient(45deg in srgb, var(--tw-gradient-stops));"
        )
    );

    let hsl = css_for("-bg-linear-30/hsl");
    assert!(
        hsl.contains("background-image: linear-gradient(-30deg in hsl, var(--tw-gradient-stops));")
    );
}

#[test]
fn radial_and_conic_gradients_support_interpolation_modifiers() {
    let radial = css_for("bg-radial/oklch");
    assert!(
        radial.contains("background-image: radial-gradient(in oklch, var(--tw-gradient-stops));")
    );

    let conic = css_for("bg-conic/decreasing");
    assert!(conic.contains(
        "background-image: conic-gradient(in oklch decreasing hue, var(--tw-gradient-stops));"
    ));

    let conic_angle = css_for("bg-conic-180/shorter");
    assert!(conic_angle.contains(
        "background-image: conic-gradient(from 180deg in oklch shorter hue, var(--tw-gradient-stops));"
    ));
}

#[test]
fn gradient_interpolation_supports_safe_arbitrary_methods() {
    let css = css_for("bg-conic/[in_hsl_longer_hue]");

    assert!(css.contains(
        "background-image: conic-gradient(in hsl longer hue, var(--tw-gradient-stops));"
    ));
}

#[test]
fn gradient_interpolation_rejects_unknown_or_unsafe_modifiers() {
    let engine = StyleEngine::empty();

    assert!(engine.css_for_class("bg-linear-to-r/banana").is_none());
    assert!(engine.css_for_class("bg-conic/[bad;value]").is_none());
}
