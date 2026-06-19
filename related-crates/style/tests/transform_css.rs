#[test]
fn transform_gpu_generates_tailwind_familiar_css() {
    let css = style::core::css_for_class("transform-gpu").expect("transform gpu utility");

    assert!(css.contains(".transform-gpu"));
    assert!(css.contains("transform: translate3d("));
    assert!(css.contains("var(--tw-translate-x, 0)"));
    assert!(css.contains("var(--tw-translate-y, 0)"));
}

#[test]
fn transform_cpu_generates_default_transform_css() {
    let css = style::core::css_for_class("transform-cpu").expect("transform cpu utility");

    assert!(css.contains(".transform-cpu"));
    assert!(css.contains("transform: translate("));
    assert!(css.contains("var(--tw-translate-x, 0)"));
    assert!(css.contains("var(--tw-translate-y, 0)"));
}

#[test]
fn three_d_transform_utilities_cover_tailwind_v43_presets_and_custom_values() {
    let dramatic =
        style::core::css_for_class("perspective-dramatic").expect("perspective preset utility");
    assert!(dramatic.contains(".perspective-dramatic"));
    assert!(dramatic.contains("perspective: var(--perspective-dramatic);"));

    let transform =
        style::core::css_for_class("transform-(--dx-transform)").expect("transform var utility");
    assert!(transform.contains(".transform-\\(--dx-transform\\)"));
    assert!(transform.contains("transform: var(--dx-transform);"));

    let arbitrary =
        style::core::css_for_class("transform-[matrix3d(1,0,0,0,0,1,0,0,0,0,1,0,24px,0,0,1)]")
            .expect("arbitrary matrix3d utility");
    assert!(arbitrary.contains("transform: matrix3d(1,0,0,0,0,1,0,0,0,0,1,0,24px,0,0,1);"));
}

#[test]
fn three_d_scale_z_utilities_support_custom_and_negative_values() {
    let custom = style::core::css_for_class("scale-z-(--dx-scale-z)").expect("scale z var utility");
    assert!(custom.contains(".scale-z-\\(--dx-scale-z\\)"));
    assert!(custom.contains("--tw-scale-z: var(--dx-scale-z);"));
    assert!(custom.contains("transform:"));
    assert!(custom.contains("scaleZ(var(--tw-scale-z, 1))"));

    let negative = style::core::css_for_class("-scale-z-125").expect("negative scale z utility");
    assert!(negative.contains(".-scale-z-125"));
    assert!(negative.contains("--tw-scale-z: -1.25;"));
    assert!(negative.contains("transform:"));
}

#[test]
fn three_d_transform_utilities_reject_unknown_or_unsafe_values() {
    assert!(style::core::css_for_class("perspective-impossible").is_none());
    assert!(style::core::css_for_class("transform-[rotate(45deg);color:red]").is_none());
}
