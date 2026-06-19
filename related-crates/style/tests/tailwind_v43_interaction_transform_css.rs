fn css_for(class_name: &str) -> String {
    style::core::css_for_class(class_name)
        .unwrap_or_else(|| panic!("{class_name} should generate CSS"))
}

#[test]
fn tailwind_v43_scrollbar_color_utilities_match_direct_var_output() {
    for (class_name, fragments) in [
        (
            "scrollbar-thumb-mauve-500",
            &[
                "--tw-scrollbar-thumb: var(--color-mauve-500);",
                "scrollbar-color: var(--tw-scrollbar-thumb) var(--tw-scrollbar-track);",
            ][..],
        ),
        (
            "scrollbar-thumb-mist-500/40",
            &[
                "--tw-scrollbar-thumb: color-mix(in oklab, var(--color-mist-500) 40%, transparent);",
                "scrollbar-color: var(--tw-scrollbar-thumb) var(--tw-scrollbar-track);",
            ],
        ),
        (
            "scrollbar-track-taupe-100",
            &[
                "--tw-scrollbar-track: var(--color-taupe-100);",
                "scrollbar-color: var(--tw-scrollbar-thumb) var(--tw-scrollbar-track);",
            ],
        ),
        (
            "scrollbar-track-current",
            &[
                "--tw-scrollbar-track: currentColor;",
                "scrollbar-color: var(--tw-scrollbar-thumb) var(--tw-scrollbar-track);",
            ],
        ),
        (
            "scrollbar-thumb-transparent",
            &[
                "--tw-scrollbar-thumb: transparent;",
                "scrollbar-color: var(--tw-scrollbar-thumb) var(--tw-scrollbar-track);",
            ],
        ),
    ] {
        let css = css_for(class_name);

        for fragment in fragments {
            assert!(
                css.contains(fragment),
                "{class_name} missing {fragment}: {css}"
            );
        }
        assert!(
            !css.contains("var(--tw-scrollbar-thumb,")
                && !css.contains("var(--tw-scrollbar-track,"),
            "{class_name} should not emit fallbacked scrollbar-color vars: {css}"
        );
    }
}

#[test]
fn tailwind_v43_scroll_snap_zoom_and_3d_transform_utilities_stay_owned() {
    for (class_name, fragments) in [
        (
            "snap-x",
            &["scroll-snap-type: x var(--tw-scroll-snap-strictness);"][..],
        ),
        (
            "snap-mandatory",
            &["--tw-scroll-snap-strictness: mandatory;"],
        ),
        ("zoom-[1.1]", &["zoom: 1.1;"]),
        ("zoom-(--dx-zoom)", &["zoom: var(--dx-zoom);"]),
        ("transform-3d", &["transform-style: preserve-3d;"]),
        (
            "backface-hidden",
            &[
                "-webkit-backface-visibility: hidden;",
                "backface-visibility: hidden;",
            ],
        ),
        (
            "perspective-dramatic",
            &["perspective: var(--perspective-dramatic);"],
        ),
        ("perspective-[750px]", &["perspective: 750px;"]),
        (
            "rotate-x-45",
            &[
                "--tw-rotate-x: rotateX(45deg);",
                "transform: var(--tw-rotate-x,) var(--tw-rotate-y,) var(--tw-rotate-z,) var(--tw-skew-x,) var(--tw-skew-y,);",
            ],
        ),
        (
            "-rotate-y-12",
            &[
                "--tw-rotate-y: rotateY(calc(12deg * -1));",
                "transform: var(--tw-rotate-x,) var(--tw-rotate-y,) var(--tw-rotate-z,) var(--tw-skew-x,) var(--tw-skew-y,);",
            ],
        ),
        (
            "translate-z-4",
            &[
                "--tw-translate-z: calc(var(--spacing) * 4);",
                "translate: var(--tw-translate-x) var(--tw-translate-y) var(--tw-translate-z);",
            ],
        ),
        (
            "scale-z-125",
            &[
                "--tw-scale-z: 125%;",
                "scale: var(--tw-scale-x) var(--tw-scale-y) var(--tw-scale-z);",
            ],
        ),
    ] {
        let css = css_for(class_name);

        for fragment in fragments {
            assert!(
                css.contains(fragment),
                "{class_name} missing {fragment}: {css}"
            );
        }
    }
}

#[test]
fn tailwind_v43_interaction_transform_unsafe_values_fail_closed() {
    for class_name in [
        "scrollbar-thumb-[red;color:blue]",
        "zoom-[1.1;color:red]",
        "transform-[rotate(45deg);color:red]",
        "perspective-origin-[top;left]",
    ] {
        assert!(
            style::core::css_for_class(class_name).is_none(),
            "{class_name} should not generate unsafe CSS"
        );
    }
}
