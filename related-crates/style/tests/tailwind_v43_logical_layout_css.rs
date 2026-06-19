use style::core::StyleEngine;

fn css_for(class_name: &str) -> String {
    style::core::css_for_class(class_name)
        .unwrap_or_else(|| panic!("{class_name} should generate CSS"))
}

fn assert_css_contains(class_name: &str, fragments: &[&str]) {
    let css = css_for(class_name);
    for fragment in fragments {
        assert!(
            css.contains(fragment),
            "{class_name} should contain {fragment:?}, got {css}"
        );
    }
}

#[test]
fn tailwind_v43_numeric_logical_spacing_preserves_spacing_theme_calc() {
    for (class_name, fragments) in [
        ("px-4", &["padding-inline: calc(var(--spacing) * 4)"][..]),
        ("py-2", &["padding-block: calc(var(--spacing) * 2)"]),
        ("pbs-4", &["padding-block-start: calc(var(--spacing) * 4)"]),
        ("-mbs-2", &["margin-block-start: calc(var(--spacing) * -2)"]),
        (
            "inset-s-4",
            &["inset-inline-start: calc(var(--spacing) * 4)"],
        ),
        (
            "-inset-e-1/2",
            &["inset-inline-end: calc(calc(1 / 2 * 100%) * -1)"],
        ),
        (
            "scroll-mbs-6",
            &["scroll-margin-block-start: calc(var(--spacing) * 6)"],
        ),
        (
            "-scroll-mbe-2",
            &["scroll-margin-block-end: calc(var(--spacing) * -2)"],
        ),
        (
            "scroll-pbe-2",
            &["scroll-padding-block-end: calc(var(--spacing) * 2)"],
        ),
    ] {
        assert_css_contains(class_name, fragments);
    }
}

#[test]
fn tailwind_v43_logical_inline_aliases_match_tailwind_cli_truth() {
    for (class_name, fragment) in [
        ("ps-4", "padding-inline-start: calc(var(--spacing) * 4)"),
        ("pe-[3rem]", "padding-inline-end: 3rem"),
        ("ms-auto", "margin-inline-start: auto"),
        ("-me-2", "margin-inline-end: calc(var(--spacing) * -2)"),
        ("inset-s-1/2", "inset-inline-start: calc(1 / 2 * 100%)"),
        ("-inset-e-full", "inset-inline-end: -100%"),
        (
            "scroll-ms-6",
            "scroll-margin-inline-start: calc(var(--spacing) * 6)",
        ),
        (
            "-scroll-me-2",
            "scroll-margin-inline-end: calc(var(--spacing) * -2)",
        ),
        (
            "scroll-ps-6",
            "scroll-padding-inline-start: calc(var(--spacing) * 6)",
        ),
        (
            "scroll-pe-(--snap-inline-end)",
            "scroll-padding-inline-end: var(--snap-inline-end)",
        ),
    ] {
        assert_css_contains(class_name, &[fragment]);
    }

    for class_name in [
        "-scroll-ps-2",
        "-scroll-pe-2",
        "pis-4",
        "pie-4",
        "mis-4",
        "mie-4",
        "inset-is-4",
        "inset-ie-4",
        "scroll-mis-4",
        "scroll-pis-4",
        "scroll-pie-4",
    ] {
        assert!(
            style::core::css_for_class(class_name).is_none(),
            "{class_name} should stay unsupported because Tailwind 4.3 CLI did not emit it"
        );
    }
}

#[test]
fn tailwind_v42_logical_spacing_and_inset_block_axis_utilities_generate_css() {
    for (class_name, fragment) in [
        ("px-4", "padding-inline: calc(var(--spacing) * 4)"),
        ("py-2", "padding-block: calc(var(--spacing) * 2)"),
        ("pbs-4", "padding-block-start: calc(var(--spacing) * 4)"),
        (
            "pbe-(--panel-block-padding)",
            "padding-block-end: var(--panel-block-padding)",
        ),
        ("mx-auto", "margin-inline: auto"),
        ("-my-2", "margin-block: calc(var(--spacing) * -2)"),
        ("-mbs-2", "margin-block-start: calc(var(--spacing) * -2)"),
        ("mbe-auto", "margin-block-end: auto"),
        ("inset-s-4", "inset-inline-start: calc(var(--spacing) * 4)"),
        (
            "-inset-e-1/2",
            "inset-inline-end: calc(calc(1 / 2 * 100%) * -1)",
        ),
        ("inset-bs-full", "inset-block-start: 100%"),
        ("inset-be-auto", "inset-block-end: auto"),
    ] {
        assert_css_contains(class_name, &[fragment]);
    }
}

#[test]
fn tailwind_v42_scroll_logical_block_axis_utilities_generate_css() {
    for (class_name, fragment) in [
        (
            "scroll-mx-4",
            "scroll-margin-inline: calc(var(--spacing) * 4)",
        ),
        (
            "scroll-my-8",
            "scroll-margin-block: calc(var(--spacing) * 8)",
        ),
        (
            "scroll-mbs-6",
            "scroll-margin-block-start: calc(var(--spacing) * 6)",
        ),
        (
            "-scroll-mbe-2",
            "scroll-margin-block-end: calc(var(--spacing) * -2)",
        ),
        (
            "scroll-px-4",
            "scroll-padding-inline: calc(var(--spacing) * 4)",
        ),
        (
            "scroll-py-8",
            "scroll-padding-block: calc(var(--spacing) * 8)",
        ),
        (
            "scroll-pbs-(--snap-block-start)",
            "scroll-padding-block-start: var(--snap-block-start)",
        ),
        (
            "scroll-pbe-2",
            "scroll-padding-block-end: calc(var(--spacing) * 2)",
        ),
    ] {
        assert_css_contains(class_name, &[fragment]);
    }

    for class_name in [
        "-scroll-p-2",
        "-scroll-px-2",
        "-scroll-py-2",
        "-scroll-pbs-2",
        "-scroll-pbe-2",
    ] {
        assert!(
            style::core::css_for_class(class_name).is_none(),
            "{class_name} should stay unsupported because Tailwind 4.3 does not emit negative scroll-padding utilities"
        );
    }
}

#[test]
fn tailwind_v43_space_utilities_use_logical_child_margins_and_reverse_vars() {
    for (class_name, fragments) in [
        (
            "space-x-4",
            &[
                ":where(:not(:last-child))",
                "--tw-space-x-reverse: 0",
                "margin-inline-start: calc(calc(var(--spacing) * 4) * var(--tw-space-x-reverse))",
                "margin-inline-end: calc(calc(var(--spacing) * 4) * calc(1 - var(--tw-space-x-reverse)))",
            ][..],
        ),
        (
            "-space-x-2",
            &[
                ":where(:not(:last-child))",
                "--tw-space-x-reverse: 0",
                "margin-inline-start: calc(calc(var(--spacing) * -2) * var(--tw-space-x-reverse))",
                "margin-inline-end: calc(calc(var(--spacing) * -2) * calc(1 - var(--tw-space-x-reverse)))",
            ],
        ),
        (
            "space-y-3",
            &[
                ":where(:not(:last-child))",
                "--tw-space-y-reverse: 0",
                "margin-block-start: calc(calc(var(--spacing) * 3) * var(--tw-space-y-reverse))",
                "margin-block-end: calc(calc(var(--spacing) * 3) * calc(1 - var(--tw-space-y-reverse)))",
            ],
        ),
        (
            "space-x-reverse",
            &[":where(:not(:last-child))", "--tw-space-x-reverse: 1"],
        ),
        (
            "space-y-reverse",
            &[":where(:not(:last-child))", "--tw-space-y-reverse: 1"],
        ),
    ] {
        assert_css_contains(class_name, fragments);
    }
}

#[test]
fn tailwind_v43_logical_sizing_uses_container_tokens_and_fraction_calc() {
    for (class_name, fragment) in [
        ("w-1/2", "width: calc(1 / 2 * 100%)"),
        ("size-1/2", "width: calc(1 / 2 * 100%)"),
        ("size-1/2", "height: calc(1 / 2 * 100%)"),
        ("inline-3xs", "inline-size: var(--container-3xs)"),
        ("min-inline-xl", "min-inline-size: var(--container-xl)"),
        ("max-inline-3xs", "max-inline-size: var(--container-3xs)"),
    ] {
        assert_css_contains(class_name, &[fragment]);
    }

    for class_name in ["size-3xs", "h-3xs", "block-3xs", "min-block-xl"] {
        assert!(
            style::core::css_for_class(class_name).is_none(),
            "{class_name} should stay unsupported because Tailwind 4.3 does not emit container-scale block/height/size utilities"
        );
    }
}

#[test]
fn tailwind_v42_logical_border_block_width_utilities_generate_css() {
    for (class_name, fragments) in [
        (
            "border-x",
            &[
                "border-inline-style: var(--tw-border-style)",
                "border-inline-width: 1px",
            ][..],
        ),
        (
            "border-y-4",
            &[
                "border-block-style: var(--tw-border-style)",
                "border-block-width: 4px",
            ],
        ),
        (
            "border-s-4",
            &[
                "border-inline-start-style: var(--tw-border-style)",
                "border-inline-start-width: 4px",
            ],
        ),
        (
            "border-e-4",
            &[
                "border-inline-end-style: var(--tw-border-style)",
                "border-inline-end-width: 4px",
            ],
        ),
        (
            "border-bs",
            &[
                "border-block-start-style: var(--tw-border-style)",
                "border-block-start-width: 1px",
            ],
        ),
        (
            "border-be",
            &[
                "border-block-end-style: var(--tw-border-style)",
                "border-block-end-width: 1px",
            ],
        ),
        (
            "border-bs-4",
            &[
                "border-block-start-style: var(--tw-border-style)",
                "border-block-start-width: 4px",
            ],
        ),
        ("border-be-[3px]", &["border-block-end-width: 3px"]),
        (
            "border-be-(length:--dx-border-block)",
            &["border-block-end-width: var(--dx-border-block)"],
        ),
    ] {
        assert_css_contains(class_name, fragments);
    }
}

#[test]
fn tailwind_v43_logical_border_color_utilities_generate_css() {
    for (class_name, fragment) in [
        (
            "border-x-red-500",
            "border-inline-color: var(--color-red-500)",
        ),
        ("border-y-blue-500", "border-block-color: rgb(59 130 246)"),
        (
            "border-s-red-500",
            "border-inline-start-color: var(--color-red-500)",
        ),
        (
            "border-e-blue-500",
            "border-inline-end-color: rgb(59 130 246)",
        ),
        (
            "border-bs-emerald-500",
            "border-block-start-color: var(--color-emerald-500)",
        ),
        (
            "border-be-purple-500",
            "border-block-end-color: rgb(168 85 247)",
        ),
    ] {
        assert_css_contains(class_name, &[fragment]);
    }
}

#[test]
fn tailwind_v43_logical_radius_utilities_preserve_tailwind_theme_tokens() {
    for (class_name, fragments) in [
        ("rounded", &["border-radius: 0.25rem"][..]),
        ("rounded-xs", &["border-radius: var(--radius-xs)"]),
        ("rounded-lg", &["border-radius: var(--radius-lg)"]),
        ("rounded-4xl", &["border-radius: var(--radius-4xl)"]),
        (
            "rounded-t-lg",
            &[
                "border-top-left-radius: var(--radius-lg)",
                "border-top-right-radius: var(--radius-lg)",
            ],
        ),
        (
            "rounded-s-lg",
            &[
                "border-start-start-radius: var(--radius-lg)",
                "border-end-start-radius: var(--radius-lg)",
            ],
        ),
        (
            "rounded-e-none",
            &["border-start-end-radius: 0", "border-end-end-radius: 0"],
        ),
        (
            "rounded-ss-full",
            &["border-start-start-radius: calc(infinity * 1px)"],
        ),
        (
            "rounded-se-(--dx-radius)",
            &["border-start-end-radius: var(--dx-radius)"],
        ),
        (
            "rounded-ee-xl",
            &["border-end-end-radius: var(--radius-xl)"],
        ),
        ("rounded-es-[2rem]", &["border-end-start-radius: 2rem"]),
    ] {
        assert_css_contains(class_name, fragments);
    }
}

#[test]
fn tailwind_v42_logical_spacing_theme_tokens_precompile_block_axis_aliases() {
    let engine = StyleEngine::from_theme_css(
        r#"
@theme {
  --spacing-panel: 2rem;
}
"#,
    );

    for (class_name, fragment) in [
        ("px-panel", "padding-inline: var(--spacing-panel)"),
        ("mx-panel", "margin-inline: var(--spacing-panel)"),
        ("pbs-panel", "padding-block-start: var(--spacing-panel)"),
        (
            "-mbe-panel",
            "margin-block-end: calc(var(--spacing-panel) * -1)",
        ),
        ("inset-bs-panel", "inset-block-start: var(--spacing-panel)"),
    ] {
        let css = engine
            .css_for_class(class_name)
            .unwrap_or_else(|| panic!("{class_name} should generate CSS"));
        assert!(
            css.contains(fragment),
            "{class_name} should contain {fragment:?}, got {css}"
        );
    }
}
