use style::core::{
    StyleEngine, engine::theme_css::DEFAULT_DX_THEME_CSS, theme_layer_css_from_source,
};

fn css_for(class_name: &str) -> String {
    StyleEngine::empty()
        .css_for_class(class_name)
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
fn tailwind_v42_added_neutral_palettes_generate_across_color_families() {
    assert_css_contains(
        "bg-mauve-500",
        &["background-color: var(--color-mauve-500)"],
    );
    assert_css_contains("text-olive-500", &["color: var(--color-olive-500)"]);
    assert_css_contains("border-mist-300", &["border-color: var(--color-mist-300)"]);
    assert_css_contains(
        "ring-taupe-700",
        &["--tw-ring-color: var(--color-taupe-700)"],
    );
    assert_css_contains(
        "outline-mauve-600",
        &["outline-color: var(--color-mauve-600)"],
    );
    assert_css_contains(
        "decoration-olive-400",
        &["text-decoration-color: var(--color-olive-400)"],
    );
    assert_css_contains("fill-mist-500", &["fill: var(--color-mist-500)"]);
    assert_css_contains("stroke-taupe-950", &["stroke: var(--color-taupe-950)"]);
    assert_css_contains(
        "accent-olive-500",
        &["accent-color: var(--color-olive-500)"],
    );
    assert_css_contains("caret-mauve-500", &["caret-color: var(--color-mauve-500)"]);
    assert_css_contains(
        "placeholder-mauve-500",
        &["::placeholder", "color: var(--color-mauve-500)"],
    );
    assert_css_contains(
        "placeholder-olive-500/50",
        &[
            "::placeholder",
            "color: color-mix(in oklab, var(--color-olive-500) 50%, transparent)",
        ],
    );
    assert_css_contains(
        "placeholder-[#243c5a]",
        &["::placeholder", "color: #243c5a"],
    );
    assert_css_contains(
        "placeholder-(color:--dx-placeholder)/(--dx-alpha)",
        &[
            "::placeholder",
            "color: color-mix(in oklab, var(--dx-placeholder) var(--dx-alpha), transparent)",
        ],
    );
    assert_css_contains(
        "divide-mauve-500",
        &["border-color: var(--color-mauve-500)"],
    );
    assert_css_contains(
        "divide-olive-500/50",
        &["border-color: color-mix(in oklab, var(--color-olive-500) 50%, transparent)"],
    );
    assert_css_contains("divide-[#243c5a]", &["border-color: #243c5a"]);
    assert_css_contains(
        "divide-(color:--dx-divider)/(--dx-alpha)",
        &["border-color: color-mix(in oklab, var(--dx-divider) var(--dx-alpha), transparent)"],
    );
}

#[test]
fn tailwind_v42_added_neutral_palettes_generate_directional_border_colors() {
    assert_css_contains(
        "border-x-mauve-500",
        &["border-inline-color: var(--color-mauve-500)"],
    );
    assert_css_contains(
        "border-y-olive-500/50",
        &["border-block-color: color-mix(in oklab, var(--color-olive-500) 50%, transparent)"],
    );
    assert_css_contains(
        "border-t-mist-300",
        &["border-top-color: var(--color-mist-300)"],
    );
    assert_css_contains(
        "border-r-taupe-700",
        &["border-right-color: var(--color-taupe-700)"],
    );
    assert_css_contains(
        "border-b-mauve-950",
        &["border-bottom-color: var(--color-mauve-950)"],
    );
    assert_css_contains(
        "border-l-olive-600/[71.37%]",
        &["border-left-color: color-mix(in oklab, var(--color-olive-600) 71.37%, transparent)"],
    );
    assert_css_contains("border-t-[#243c5a]", &["border-top-color: #243c5a"]);
    assert_css_contains(
        "border-r-[#243c5a]/50",
        &["border-right-color: color-mix(in oklab, #243c5a 50%, transparent)"],
    );
    assert_css_contains(
        "border-x-(color:--dx-border)",
        &["border-inline-color: var(--dx-border)"],
    );
    assert_css_contains(
        "border-y-(color:--dx-border-alpha)/(--dx-alpha)",
        &[
            "border-block-color: color-mix(in oklab, var(--dx-border-alpha) var(--dx-alpha), transparent)",
        ],
    );
}

#[test]
fn tailwind_v42_added_neutral_palettes_support_opacity_and_gradient_stops() {
    assert_css_contains(
        "bg-mauve-500/50",
        &[concat!(
            "background-color: color-mix(in oklab, ",
            "var(--color-mauve-500) 50%, transparent)"
        )],
    );
    assert_css_contains(
        "text-taupe-500/25",
        &[concat!(
            "color: color-mix(in oklab, ",
            "var(--color-taupe-500) 25%, transparent)"
        )],
    );
    assert_css_contains(
        "bg-mist-500/[71.37%]",
        &[concat!(
            "background-color: color-mix(in oklab, ",
            "var(--color-mist-500) 71.37%, transparent)"
        )],
    );
    assert_css_contains(
        "border-taupe-500/(--dx-alpha)",
        &[concat!(
            "border-color: color-mix(in oklab, ",
            "var(--color-taupe-500) var(--dx-alpha), transparent)"
        )],
    );
    assert_css_contains(
        "text-shadow-mist-500/40",
        &[concat!(
            "--tw-text-shadow-color: color-mix(in oklab, ",
            "var(--color-mist-500) 40%, transparent)"
        )],
    );
    assert_css_contains(
        "from-mauve-500/40",
        &[
            concat!(
                "--tw-gradient-from: color-mix(in oklab, ",
                "var(--color-mauve-500) 40%, transparent)"
            ),
            "--tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to)",
        ],
    );
    assert_css_contains(
        "via-olive-500",
        &[
            "--tw-gradient-via: var(--color-olive-500)",
            "--tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-via), var(--tw-gradient-to)",
        ],
    );
    assert_css_contains(
        "to-taupe-950",
        &["--tw-gradient-to: var(--color-taupe-950)"],
    );
}

#[test]
fn tailwind_v43_neutral_palette_opacity_uses_srgb_fallback_and_oklab_supports() {
    assert_css_contains(
        "bg-mauve-500/50",
        &[
            "background-color: color-mix(in srgb, oklch(54.2% 0.034 322.5) 50%, transparent)",
            "@supports (color: color-mix(in lab, red, red))",
            "background-color: color-mix(in oklab, var(--color-mauve-500) 50%, transparent)",
        ],
    );
    assert_css_contains(
        "text-taupe-500/25",
        &[
            "color: color-mix(in srgb, oklch(54.7% 0.021 43.1) 25%, transparent)",
            "color: color-mix(in oklab, var(--color-taupe-500) 25%, transparent)",
        ],
    );
    assert_css_contains(
        "border-y-olive-500/50",
        &[
            "border-block-color: color-mix(in srgb, oklch(58% 0.031 107.3) 50%, transparent)",
            "border-block-color: color-mix(in oklab, var(--color-olive-500) 50%, transparent)",
        ],
    );
    assert_css_contains(
        "outline-mauve-700/30",
        &[
            "outline-color: color-mix(in srgb, oklch(36.4% 0.029 323.89) 30%, transparent)",
            "outline-color: color-mix(in oklab, var(--color-mauve-700) 30%, transparent)",
        ],
    );
    assert_css_contains(
        "decoration-olive-400/60",
        &[
            "text-decoration-color: color-mix(in srgb, oklch(73.7% 0.021 106.9) 60%, transparent)",
            "text-decoration-color: color-mix(in oklab, var(--color-olive-400) 60%, transparent)",
        ],
    );
    assert_css_contains(
        "fill-mist-500/45",
        &[
            "fill: color-mix(in srgb, oklch(56% 0.021 213.5) 45%, transparent)",
            "fill: color-mix(in oklab, var(--color-mist-500) 45%, transparent)",
        ],
    );
    assert_css_contains(
        "stroke-mist-700/75",
        &[
            "stroke: color-mix(in srgb, oklch(37.8% 0.015 216) 75%, transparent)",
            "stroke: color-mix(in oklab, var(--color-mist-700) 75%, transparent)",
        ],
    );
    assert_css_contains(
        "ring-taupe-400/50",
        &[
            "--tw-ring-color: color-mix(in srgb, oklch(71.4% 0.014 41.2) 50%, transparent)",
            "--tw-ring-color: color-mix(in oklab, var(--color-taupe-400) 50%, transparent)",
        ],
    );
    assert_css_contains(
        "accent-olive-500/80",
        &[
            "accent-color: color-mix(in srgb, oklch(58% 0.031 107.3) 80%, transparent)",
            "accent-color: color-mix(in oklab, var(--color-olive-500) 80%, transparent)",
        ],
    );
    assert_css_contains(
        "caret-mauve-500/20",
        &[
            "caret-color: color-mix(in srgb, oklch(54.2% 0.034 322.5) 20%, transparent)",
            "caret-color: color-mix(in oklab, var(--color-mauve-500) 20%, transparent)",
        ],
    );
    assert_css_contains(
        "placeholder-olive-500/50",
        &[
            "::placeholder",
            "color: color-mix(in srgb, oklch(58% 0.031 107.3) 50%, transparent)",
            "color: color-mix(in oklab, var(--color-olive-500) 50%, transparent)",
        ],
    );
    assert_css_contains(
        "divide-olive-500/50",
        &[
            "border-color: color-mix(in srgb, oklch(58% 0.031 107.3) 50%, transparent)",
            "border-color: color-mix(in oklab, var(--color-olive-500) 50%, transparent)",
        ],
    );
    assert_css_contains(
        "from-mauve-500/40",
        &[
            "--tw-gradient-from: color-mix(in srgb, oklch(54.2% 0.034 322.5) 40%, transparent)",
            "--tw-gradient-from: color-mix(in oklab, var(--color-mauve-500) 40%, transparent)",
            "--tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to)",
        ],
    );
    assert_css_contains(
        "via-taupe-500/40",
        &[
            "--tw-gradient-via: color-mix(in srgb, oklch(54.7% 0.021 43.1) 40%, transparent)",
            "--tw-gradient-via: color-mix(in oklab, var(--color-taupe-500) 40%, transparent)",
            "--tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-via), var(--tw-gradient-to)",
        ],
    );
    assert_css_contains(
        "to-olive-950/80",
        &[
            "--tw-gradient-to: color-mix(in srgb, oklch(15.3% 0.006 107.1) 80%, transparent)",
            "--tw-gradient-to: color-mix(in oklab, var(--color-olive-950) 80%, transparent)",
        ],
    );
    assert_css_contains(
        "scrollbar-thumb-mauve-500/60",
        &[
            "--tw-scrollbar-thumb: color-mix(in srgb, oklch(54.2% 0.034 322.5) 60%, transparent)",
            "--tw-scrollbar-thumb: color-mix(in oklab, var(--color-mauve-500) 60%, transparent)",
            "scrollbar-color: var(--tw-scrollbar-thumb) var(--tw-scrollbar-track)",
        ],
    );
    assert_css_contains(
        "scrollbar-track-taupe-100/10",
        &[
            "--tw-scrollbar-track: color-mix(in srgb, oklch(96% 0.002 17.2) 10%, transparent)",
            "--tw-scrollbar-track: color-mix(in oklab, var(--color-taupe-100) 10%, transparent)",
            "scrollbar-color: var(--tw-scrollbar-thumb) var(--tw-scrollbar-track)",
        ],
    );
}

#[test]
fn tailwind_v43_runtime_alpha_palette_hooks_use_tailwind_fallbacks() {
    assert_css_contains(
        "bg-mauve-500/(--my-alpha-value)",
        &[
            "background-color: oklch(54.2% 0.034 322.5)",
            "@supports (color: color-mix(in lab, red, red))",
            "background-color: color-mix(in oklab, var(--color-mauve-500) var(--my-alpha-value), transparent)",
        ],
    );
    assert_css_contains(
        "shadow-mauve-500/(--my-alpha-value)",
        &[
            "--tw-shadow-color: oklch(54.2% 0.034 322.5)",
            "@supports (color: color-mix(in lab, red, red))",
            "--tw-shadow-color: color-mix(in oklab, color-mix(in oklab, var(--color-mauve-500) var(--my-alpha-value), transparent) var(--tw-shadow-alpha), transparent)",
        ],
    );
    assert_css_contains(
        "drop-shadow-mauve-500/(--my-alpha-value)",
        &[
            "--tw-drop-shadow-color: oklch(54.2% 0.034 322.5)",
            "--tw-drop-shadow-color: color-mix(in oklab, color-mix(in oklab, var(--color-mauve-500) var(--my-alpha-value), transparent) var(--tw-drop-shadow-alpha), transparent)",
            "--tw-drop-shadow: var(--tw-drop-shadow-size)",
        ],
    );
    assert_css_contains(
        "inset-shadow-olive-500/(--my-alpha-value)",
        &[
            "--tw-inset-shadow-color: oklch(58% 0.031 107.3)",
            "--tw-inset-shadow-color: color-mix(in oklab, color-mix(in oklab, var(--color-olive-500) var(--my-alpha-value), transparent) var(--tw-inset-shadow-alpha), transparent)",
        ],
    );

    for class_name in [
        "shadow-mauve-500/(--my-alpha-value)",
        "drop-shadow-mauve-500/(--my-alpha-value)",
        "inset-shadow-olive-500/(--my-alpha-value)",
    ] {
        let css = css_for(class_name);
        assert!(
            !css.contains("color-mix(in srgb, oklch"),
            "{class_name} should match Tailwind's runtime-alpha base OKLCH fallback, got {css}"
        );
    }
}

#[test]
fn tailwind_v43_neutral_palettes_cover_shadow_drop_shadow_and_inset_ring_colors() {
    assert_css_contains(
        "shadow-mauve-500",
        &[
            "--tw-shadow-color: oklch(54.2% 0.034 322.5)",
            "@supports (color: color-mix(in lab, red, red))",
            "--tw-shadow-color: color-mix(in oklab, var(--color-mauve-500) var(--tw-shadow-alpha), transparent)",
        ],
    );
    assert_css_contains(
        "shadow-mauve-500/50",
        &[
            "--tw-shadow-color: color-mix(in srgb, oklch(54.2% 0.034 322.5) 50%, transparent)",
            "@supports (color: color-mix(in lab, red, red))",
            "--tw-shadow-color: color-mix(in oklab, color-mix(in oklab, var(--color-mauve-500) 50%, transparent) var(--tw-shadow-alpha), transparent)",
        ],
    );
    assert_css_contains(
        "drop-shadow-mauve-500/50",
        &[
            "--tw-drop-shadow-color: color-mix(in srgb, oklch(54.2% 0.034 322.5) 50%, transparent)",
            "--tw-drop-shadow-color: color-mix(in oklab, color-mix(in oklab, var(--color-mauve-500) 50%, transparent) var(--tw-drop-shadow-alpha), transparent)",
            "--tw-drop-shadow: var(--tw-drop-shadow-size)",
        ],
    );
    assert_css_contains(
        "inset-shadow-olive-500",
        &[
            "--tw-inset-shadow-color: oklch(58% 0.031 107.3)",
            "--tw-inset-shadow-color: color-mix(in oklab, var(--color-olive-500) var(--tw-inset-shadow-alpha), transparent)",
        ],
    );
    assert_css_contains(
        "inset-shadow-olive-500/50",
        &[
            "--tw-inset-shadow-color: color-mix(in srgb, oklch(58% 0.031 107.3) 50%, transparent)",
            "--tw-inset-shadow-color: color-mix(in oklab, color-mix(in oklab, var(--color-olive-500) 50%, transparent) var(--tw-inset-shadow-alpha), transparent)",
        ],
    );
    assert_css_contains(
        "inset-ring",
        &[
            "--tw-inset-ring-shadow: inset 0 0 0 1px var(--tw-inset-ring-color, currentcolor)",
            "box-shadow: var(--tw-inset-shadow), var(--tw-inset-ring-shadow), var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow)",
        ],
    );
    assert_css_contains(
        "inset-ring-2",
        &["--tw-inset-ring-shadow: inset 0 0 0 2px var(--tw-inset-ring-color, currentcolor)"],
    );
    assert_css_contains(
        "inset-ring-mist-500",
        &["--tw-inset-ring-color: var(--color-mist-500)"],
    );
    assert_css_contains(
        "inset-ring-mist-500/50",
        &[
            "--tw-inset-ring-color: color-mix(in srgb, oklch(56% 0.021 213.5) 50%, transparent)",
            "--tw-inset-ring-color: color-mix(in oklab, var(--color-mist-500) 50%, transparent)",
        ],
    );
    assert_css_contains(
        "ring-offset-taupe-500/40",
        &[
            "--tw-ring-offset-color: color-mix(in srgb, oklch(54.7% 0.021 43.1) 40%, transparent)",
            "--tw-ring-offset-color: color-mix(in oklab, var(--color-taupe-500) 40%, transparent)",
        ],
    );
}

#[test]
fn tailwind_v42_added_neutral_palette_full_shade_ladders_are_available() {
    for palette in ["mauve", "olive", "mist", "taupe"] {
        for shade in [
            "50", "100", "200", "300", "400", "500", "600", "700", "800", "900", "950",
        ] {
            let class_name = format!("bg-{palette}-{shade}");
            let expected = format!("background-color: var(--color-{palette}-{shade})");
            assert_css_contains(&class_name, &[expected.as_str()]);
        }
    }
}

#[test]
fn tailwind_v42_added_neutral_palette_theme_tokens_are_oklch() {
    let theme_css = theme_layer_css_from_source(DEFAULT_DX_THEME_CSS);

    for expected in [
        "--color-mauve-500: oklch(54.2% 0.034 322.5);",
        "--color-olive-500: oklch(58% 0.031 107.3);",
        "--color-mist-500: oklch(56% 0.021 213.5);",
        "--color-taupe-500: oklch(54.7% 0.021 43.1);",
    ] {
        assert!(
            theme_css.contains(expected),
            "default theme should expose {expected}, got {theme_css}"
        );
    }
}
