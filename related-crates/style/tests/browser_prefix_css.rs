#[test]
fn hyphens_auto_generates_prefixed_and_standard_css() {
    let css = style::core::css_for_class("hyphens-auto").expect("hyphens auto utility");

    assert!(css.contains(".hyphens-auto"));
    assert!(css.contains("-webkit-hyphens: auto"));
    assert!(css.contains("hyphens: auto"));
}

#[test]
fn backface_hidden_generates_prefixed_and_standard_css() {
    let css = style::core::css_for_class("backface-hidden").expect("backface hidden utility");

    assert!(css.contains(".backface-hidden"));
    assert!(css.contains("-webkit-backface-visibility: hidden"));
    assert!(css.contains("backface-visibility: hidden"));
}

#[test]
fn break_inside_avoid_generates_legacy_and_standard_css() {
    let css = style::core::css_for_class("break-inside-avoid").expect("break inside utility");

    assert!(css.contains(".break-inside-avoid"));
    assert!(css.contains("page-break-inside: avoid"));
    assert!(css.contains("break-inside: avoid"));
}

#[test]
fn file_variant_generates_tailwind_standard_file_button_selector() {
    let css = style::core::css_for_class("file:p-4").expect("file button utility");

    assert!(css.contains("::file-selector-button"));
    assert!(!css.contains("::-webkit-file-upload-button"));
    assert!(css.contains("padding: calc(var(--spacing) * 4);"));
}
