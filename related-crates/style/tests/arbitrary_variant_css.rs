#[test]
fn arbitrary_at_rule_selector_variant_generates_wrapped_css() {
    let css = style::core::css_for_class("[@media_(any-hover:hover){&:hover}]:opacity-100")
        .expect("arbitrary at-rule selector variant");

    assert!(css.contains("@media (any-hover:hover)"));
    assert!(css.contains(":hover {"));
    assert!(css.contains("opacity: 100%;"));
}

#[test]
fn arbitrary_unknown_at_rule_variant_generates_wrapped_css() {
    let css = style::core::css_for_class("[@unknown_rule]:p-4")
        .expect("unknown arbitrary at-rule variant");

    assert!(css.contains("@unknown rule"), "{css}");
    assert!(css.contains("padding: calc(var(--spacing) * 4);"), "{css}");
}

#[test]
fn not_arbitrary_at_rule_variants_emit_tailwind_negated_wrappers() {
    for (class_name, at_rule) in [
        ("not-[@media_print]:flex", "@media not print"),
        ("not-[@media_not_print]:flex", "@media print"),
        (
            "not-[@supports(display:grid)]:flex",
            "@supports not (display:grid)",
        ),
        (
            "not-[@container_(width>=32rem)]:flex",
            "@container not (width>=32rem)",
        ),
        (
            "not-[@container_card_(width>=32rem)]:flex",
            "@container card not (width>=32rem)",
        ),
    ] {
        let css = style::core::css_for_class(class_name).expect(class_name);

        assert!(
            css.contains(at_rule),
            "{class_name} missing negated at-rule {at_rule}: {css}"
        );
        assert!(
            css.contains("display: flex;"),
            "{class_name} missing flex declaration: {css}"
        );
        assert!(
            !css.contains(":not(@"),
            "{class_name} should not treat at-rules as selector :not() conditions: {css}"
        );
    }
}

#[test]
fn not_arbitrary_unknown_and_tailwind_directive_at_rules_fail_closed() {
    for class_name in [
        "not-[@unknown_rule]:flex",
        "not-[@layer_components]:flex",
        "not-[@plugin_foo]:flex",
        "not-[@config_./tailwind.config.js]:flex",
    ] {
        assert!(
            style::core::css_for_class(class_name).is_none(),
            "{class_name} should not generate unsupported negated arbitrary at-rule CSS"
        );
    }
}

#[test]
fn stacked_arbitrary_selector_variants_compose_tailwind_selectors() {
    for (class_name, fragments) in [
        ("[&.foo]:[&.bar]:flex", &[".foo.bar", "display: flex;"][..]),
        (
            "[&_p]:[&_.lead]:mt-4",
            &[" p .lead", "margin-top: calc(var(--spacing) * 4);"][..],
        ),
        (
            "not-[.is-open]:[&.dismissible]:opacity-100",
            &[":not(.is-open).dismissible", "opacity: 100%;"][..],
        ),
        (
            "[&.is-dragging]:active:cursor-grabbing",
            &[".is-dragging:active", "cursor: grabbing;"][..],
        ),
    ] {
        let css = style::core::css_for_class(class_name).expect(class_name);

        for fragment in fragments {
            assert!(
                css.contains(fragment),
                "{class_name} missing composed selector fragment {fragment}: {css}"
            );
        }
        assert!(
            !css.contains("}\n\n."),
            "{class_name} should compose stacked selector variants into one block: {css}"
        );
    }
}

#[test]
fn arbitrary_selector_lists_compose_each_branch_without_selector_leaks() {
    let css =
        style::core::css_for_class("[&.foo,&.bar]:[&>.item,&>[data-slot=control]]:opacity-100")
            .expect("stacked arbitrary selector lists");

    for fragment in [
        ".foo>.item",
        ".foo>[data-slot=control]",
        ".bar>.item",
        ".bar>[data-slot=control]",
        "opacity: 100%;",
    ] {
        assert!(
            css.contains(fragment),
            "stacked arbitrary selector list missing {fragment}: {css}"
        );
    }

    for leaked_fragment in [".foo,.bar>.item", ".foo,.bar>[data-slot=control]"] {
        assert!(
            !css.contains(leaked_fragment),
            "stacked arbitrary selector list should not leak a whole selector list into one branch: {css}"
        );
    }
}

#[test]
fn arbitrary_group_peer_selector_variants_use_tailwind_v4_wrappers_and_stack() {
    for (class_name, fragments) in [
        (
            "group-[.is-published]:block",
            &[":is(:where(.group):is(.is-published) *)", "display: block;"][..],
        ),
        (
            "group-[:nth-of-type(3)_&]:block",
            &[":is(:nth-of-type(3) :where(.group) *)", "display: block;"][..],
        ),
        (
            "group-[.is-open]/card:block",
            &[
                ":is(:where(.group\\/card):is(.is-open) *)",
                "display: block;",
            ][..],
        ),
        (
            "peer-[.is-dirty]:block",
            &[":is(:where(.peer):is(.is-dirty) ~ *)", "display: block;"][..],
        ),
        (
            "peer-[:nth-of-type(3)_&]:block",
            &[":is(:nth-of-type(3) :where(.peer) ~ *)", "display: block;"][..],
        ),
        (
            "peer-[.is-dirty]:peer-required:block",
            &[
                ":is(:where(.peer):is(.is-dirty) ~ *)",
                ":is(:where(.peer):required ~ *)",
                "display: block;",
            ][..],
        ),
        (
            "group-[.is-open]:[&.target]:opacity-100",
            &[
                ":is(:where(.group):is(.is-open) *)",
                ".target",
                "opacity: 100%;",
            ][..],
        ),
    ] {
        let css = style::core::css_for_class(class_name).expect(class_name);

        for fragment in fragments {
            assert!(
                css.contains(fragment),
                "{class_name} missing Tailwind v4 arbitrary group/peer fragment {fragment}: {css}"
            );
        }
        assert!(
            !css.contains(".group.is-") && !css.contains(".peer.is-"),
            "{class_name} should not emit the old pre-v4 group/peer selector shape: {css}"
        );
    }
}

#[test]
fn arbitrary_group_peer_selector_lists_use_tailwind_v4_is_wrappers() {
    for (class_name, fragments) in [
        (
            "group-[&.foo,&.bar]:block",
            &[
                ":is(:is(:where(.group).foo,:where(.group).bar) *)",
                "display: block;",
            ][..],
        ),
        (
            "peer-[&.dirty,&.touched]:block",
            &[
                ":is(:is(:where(.peer).dirty,:where(.peer).touched) ~ *)",
                "display: block;",
            ][..],
        ),
        (
            "group-[&:is(.foo,.bar)]:block",
            &[":is(:where(.group):is(.foo,.bar) *)", "display: block;"][..],
        ),
    ] {
        let css = style::core::css_for_class(class_name).expect(class_name);

        for fragment in fragments {
            assert!(
                css.contains(fragment),
                "{class_name} missing Tailwind v4 selector-list fragment {fragment}: {css}"
            );
        }
    }
}

#[test]
fn arbitrary_runtime_tailwind_directive_variants_fail_closed() {
    for class_name in [
        "[@plugin_foo]:p-4",
        "[@config_./tailwind.config.js]:p-4",
        "[@tailwind_utilities]:p-4",
        "[@import_\"tailwindcss\"]:p-4",
    ] {
        assert!(
            style::core::css_for_class(class_name).is_none(),
            "{class_name} should not generate runtime Tailwind directive CSS"
        );
    }
}

#[test]
fn pointer_capability_variants_generate_tailwind_media_queries() {
    for (class_name, media) in [
        (
            "contrast-more:opacity-100",
            "@media (prefers-contrast: more)",
        ),
        (
            "contrast-less:opacity-100",
            "@media (prefers-contrast: less)",
        ),
        (
            "forced-colors:opacity-100",
            "@media (forced-colors: active)",
        ),
        (
            "inverted-colors:opacity-100",
            "@media (inverted-colors: inverted)",
        ),
        (
            "motion-safe:opacity-100",
            "@media (prefers-reduced-motion: no-preference)",
        ),
        (
            "motion-reduce:opacity-100",
            "@media (prefers-reduced-motion: reduce)",
        ),
        ("pointer-none:opacity-100", "@media (pointer: none)"),
        ("pointer-coarse:opacity-100", "@media (pointer: coarse)"),
        ("pointer-fine:opacity-100", "@media (pointer: fine)"),
        ("any-pointer-none:opacity-100", "@media (any-pointer: none)"),
        (
            "any-pointer-coarse:opacity-100",
            "@media (any-pointer: coarse)",
        ),
        ("any-pointer-fine:opacity-100", "@media (any-pointer: fine)"),
        ("noscript:opacity-100", "@media (scripting: none)"),
        ("portrait:opacity-100", "@media (orientation: portrait)"),
        ("landscape:opacity-100", "@media (orientation: landscape)"),
        ("print:opacity-100", "@media print"),
        (
            "not-motion-safe:opacity-100",
            "@media not (prefers-reduced-motion: no-preference)",
        ),
        (
            "not-motion-reduce:opacity-100",
            "@media not (prefers-reduced-motion: reduce)",
        ),
        ("not-pointer-fine:opacity-100", "@media not (pointer: fine)"),
        (
            "not-forced-colors:opacity-100",
            "@media not (forced-colors: active)",
        ),
        (
            "not-portrait:opacity-100",
            "@media not (orientation: portrait)",
        ),
        (
            "not-landscape:opacity-100",
            "@media not (orientation: landscape)",
        ),
        ("not-noscript:opacity-100", "@media not (scripting: none)"),
        ("not-print:opacity-100", "@media not print"),
    ] {
        let css = style::core::css_for_class(class_name).expect(class_name);

        assert!(css.contains(media), "{class_name} missing {media}: {css}");
        assert!(
            css.contains("opacity: 100%;"),
            "{class_name} missing opacity: {css}"
        );
    }
}

#[test]
fn hover_variants_generate_hover_capability_media_query() {
    for (class_name, selector_fragment) in [
        ("hover:opacity-100", ":hover"),
        ("group-hover:opacity-100", ":is(:where(.group):hover *)"),
        (
            "group-hover/card:opacity-100",
            ":is(:where(.group\\/card):hover *)",
        ),
        ("peer-hover:opacity-100", ":is(:where(.peer):hover ~ *)"),
        (
            "peer-hover/card:opacity-100",
            ":is(:where(.peer\\/card):hover ~ *)",
        ),
    ] {
        let css = style::core::css_for_class(class_name).expect(class_name);

        assert!(
            css.contains("@media (hover: hover)"),
            "{class_name} missing hover media: {css}"
        );
        assert!(
            css.contains(selector_fragment),
            "{class_name} missing hover selector {selector_fragment}: {css}"
        );
        assert!(
            css.contains("opacity: 100%;"),
            "{class_name} missing opacity: {css}"
        );
    }
}

#[test]
fn in_hover_variant_uses_hover_capability_media_query() {
    let css = style::core::css_for_class("in-hover:opacity-100").expect("in-hover variant");

    assert!(css.contains(":where(*:hover)"), "{css}");
    assert!(
        css.contains("@media (hover: hover)"),
        "Tailwind v4 guards hover-capability state variants with hover media: {css}"
    );
    assert!(css.contains("opacity: 100%;"), "{css}");
}

#[test]
fn not_hover_variant_emits_selector_and_no_hover_media_fallback() {
    let css = style::core::css_for_class("not-hover:opacity-100").expect("not-hover variant");

    assert!(css.contains(":not(*:hover)"), "{css}");
    assert!(
        css.contains("@media not (hover: hover)"),
        "Tailwind emits a no-hover fallback branch for not-hover variants: {css}"
    );
    assert!(
        css.match_indices("opacity: 100%;").count() >= 2,
        "not-hover should emit the declaration in both selector and fallback branches: {css}"
    );
}

#[test]
fn file_variant_matches_tailwind_standard_file_selector_button() {
    let css = style::core::css_for_class("file:flex").expect("file variant");

    assert!(css.contains("::file-selector-button"), "{css}");
    assert!(
        !css.contains("::-webkit-file-upload-button"),
        "Tailwind v4.3 emits the standard file selector pseudo-element without the WebKit upload fallback: {css}"
    );
    assert!(css.contains("display: flex;"), "{css}");
}

#[test]
fn stacked_hover_state_and_media_variants_preserve_tailwind_fragments() {
    for (class_name, fragments) in [
        (
            "hover:not-focus:opacity-100",
            &[
                ":hover",
                ":not(*:focus)",
                "@media (hover: hover)",
                "opacity: 100%;",
            ][..],
        ),
        (
            "not-focus:hover:opacity-100",
            &[
                ":not(*:focus)",
                ":hover",
                "@media (hover: hover)",
                "opacity: 100%;",
            ],
        ),
        (
            "group-hover:not-focus:opacity-100",
            &[
                ":is(:where(.group):hover *)",
                ":not(*:focus)",
                "@media (hover: hover)",
                "opacity: 100%;",
            ],
        ),
        (
            "in-focus:hover:opacity-100",
            &[
                ":where(*:focus)",
                ":hover",
                "@media (hover: hover)",
                "opacity: 100%;",
            ],
        ),
        (
            "not-pointer-fine:hover:opacity-100",
            &[
                "@media not (pointer: fine)",
                ":hover",
                "@media (hover: hover)",
                "opacity: 100%;",
            ],
        ),
        (
            "hover:not-pointer-fine:opacity-100",
            &[
                ":hover",
                "@media (hover: hover)",
                "@media not (pointer: fine)",
                "opacity: 100%;",
            ],
        ),
    ] {
        let css = style::core::css_for_class(class_name).expect(class_name);

        for fragment in fragments {
            assert!(
                css.contains(fragment),
                "{class_name} missing stacked variant fragment {fragment}: {css}"
            );
        }
    }
}

#[test]
fn selector_quick_reference_variants_generate_tailwind_wrappers() {
    for (class_name, fragments) in [
        (
            "rtl:ps-4",
            &[
                ":where(:dir(rtl), [dir=\"rtl\"], [dir=\"rtl\"] *)",
                "padding-inline-start: calc(var(--spacing) * 4);",
            ][..],
        ),
        (
            "ltr:pe-4",
            &[
                ":where(:dir(ltr), [dir=\"ltr\"], [dir=\"ltr\"] *)",
                "padding-inline-end: calc(var(--spacing) * 4);",
            ][..],
        ),
        (
            "inert:opacity-50",
            &[":is([inert], [inert] *)", "opacity: 50%;"][..],
        ),
        (
            "open:bg-blue-500",
            &[":is([open], :popover-open, :open)", "background-color"][..],
        ),
        (
            "starting:open:opacity-0",
            &[
                "@starting-style",
                ":is([open], :popover-open, :open)",
                "opacity: 0%;",
            ][..],
        ),
        (
            "user-valid:border-green-500",
            &[":user-valid", "border-color"][..],
        ),
        (
            "user-invalid:border-red-500",
            &[":user-invalid", "border-color"][..],
        ),
        (
            "details-content:bg-slate-100",
            &[":details-content", "background-color"][..],
        ),
        (
            "backdrop:bg-slate-950/50",
            &["::backdrop", "background-color"][..],
        ),
    ] {
        let css = style::core::css_for_class(class_name).expect(class_name);

        for fragment in fragments {
            assert!(
                css.contains(fragment),
                "{class_name} missing {fragment}: {css}"
            );
        }
    }
}

#[test]
fn official_selector_reference_variants_generate_wrapped_css() {
    for (class_name, fragments) in [
        (
            "rtl:ps-4",
            &[
                ":where(:dir(rtl), [dir=\"rtl\"], [dir=\"rtl\"] *)",
                "padding-inline-start: calc(var(--spacing) * 4);",
            ][..],
        ),
        (
            "ltr:pe-4",
            &[
                ":where(:dir(ltr), [dir=\"ltr\"], [dir=\"ltr\"] *)",
                "padding-inline-end: calc(var(--spacing) * 4);",
            ],
        ),
        (
            "inert:opacity-50",
            &[":is([inert], [inert] *)", "opacity: 50%;"],
        ),
        (
            "open:bg-blue-500",
            &[
                ":is([open], :popover-open, :open)",
                "background-color: rgb(59 130 246);",
            ],
        ),
        (
            "starting:open:opacity-0",
            &[
                "@starting-style",
                ":is([open], :popover-open, :open)",
                "opacity: 0%;",
            ],
        ),
        (
            "user-valid:border-green-500",
            &[":user-valid", "border-color:"],
        ),
        (
            "user-invalid:border-red-500",
            &[":user-invalid", "border-color:"],
        ),
        (
            "details-content:bg-slate-100",
            &[":details-content", "background-color:"],
        ),
        (
            "backdrop:bg-slate-950/50",
            &["::backdrop", "background-color:"],
        ),
    ] {
        let css = style::core::css_for_class(class_name).expect(class_name);

        for fragment in fragments {
            assert!(
                css.contains(fragment),
                "{class_name} missing {fragment:?}: {css}"
            );
        }
    }
}

#[test]
fn marker_variant_generates_direct_and_descendant_marker_selectors() {
    let css = style::core::css_for_class("marker:text-red-500").expect("marker variant");

    assert!(css.contains("::marker"), "{css}");
    assert!(
        css.contains("*::marker"),
        "Tailwind's marker variant should style descendant list markers too: {css}"
    );
    assert!(css.contains("color:"), "{css}");
}

#[test]
fn pseudo_element_variants_match_tailwind_quick_reference_selectors() {
    for (class_name, fragments) in [
        (
            "selection:flex",
            &["::selection", "*::selection", "display: flex;"][..],
        ),
        ("placeholder:flex", &["::placeholder", "display: flex;"]),
        ("file:flex", &["::file-selector-button", "display: flex;"]),
        (
            "first-letter:uppercase",
            &["::first-letter", "text-transform: uppercase;"],
        ),
        (
            "first-line:uppercase",
            &["::first-line", "text-transform: uppercase;"],
        ),
    ] {
        let css = style::core::css_for_class(class_name).expect(class_name);

        for fragment in fragments {
            assert!(
                css.contains(fragment),
                "{class_name} missing {fragment}: {css}"
            );
        }
    }
}

#[test]
fn child_selector_variants_use_tailwind_is_wrappers() {
    for (class_name, selector_fragment, declaration) in [
        ("*:p-4", ":is(", "padding: calc(var(--spacing) * 4);"),
        ("**:text-slate-900", ":is(", "color:"),
    ] {
        let css = style::core::css_for_class(class_name).expect(class_name);

        assert!(
            css.contains(selector_fragment),
            "{class_name} missing {selector_fragment}: {css}"
        );
        assert!(
            css.contains(declaration),
            "{class_name} missing {declaration}: {css}"
        );
    }

    let direct_css = style::core::css_for_class("*:p-4").expect("direct child variant");
    assert!(direct_css.contains("> *"), "{direct_css}");

    let descendant_css =
        style::core::css_for_class("**:text-slate-900").expect("descendant child variant");
    assert!(descendant_css.contains(" *"), "{descendant_css}");
}

#[test]
fn full_tailwind_container_query_variant_ladder_generates_wrapped_css() {
    for (class_name, fragments) in [
        (
            "@3xs:grid",
            &["@container (width >= 16rem)", "display: grid;"][..],
        ),
        (
            "@2xs:flex",
            &["@container (width >= 18rem)", "display: flex;"][..],
        ),
        (
            "@xs:block",
            &["@container (width >= 20rem)", "display: block;"][..],
        ),
        (
            "@sm:grid",
            &["@container (width >= 24rem)", "display: grid;"][..],
        ),
        (
            "@md:flex",
            &["@container (width >= 28rem)", "display: flex;"][..],
        ),
        (
            "@lg:block",
            &["@container (width >= 32rem)", "display: block;"][..],
        ),
        (
            "@xl:grid",
            &["@container (width >= 36rem)", "display: grid;"][..],
        ),
        (
            "@2xl:flex",
            &["@container (width >= 42rem)", "display: flex;"][..],
        ),
        (
            "@3xl/block:flex",
            &["@container block (width >= 48rem)", "display: flex;"][..],
        ),
        (
            "@4xl:grid",
            &["@container (width >= 56rem)", "display: grid;"][..],
        ),
        (
            "@5xl:block",
            &["@container (width >= 64rem)", "display: block;"][..],
        ),
        (
            "@6xl:flex",
            &["@container (width >= 72rem)", "display: flex;"][..],
        ),
        (
            "@7xl:flex",
            &["@container (width >= 80rem)", "display: flex;"][..],
        ),
        (
            "@max-3xs:hidden",
            &["@container (width < 16rem)", "display: none;"][..],
        ),
        (
            "@max-2xs:block",
            &["@container (width < 18rem)", "display: block;"][..],
        ),
        (
            "@max-xs:flex",
            &["@container (width < 20rem)", "display: flex;"][..],
        ),
        (
            "@max-sm:hidden",
            &["@container (width < 24rem)", "display: none;"][..],
        ),
        (
            "@max-md:block",
            &["@container (width < 28rem)", "display: block;"][..],
        ),
        (
            "@max-lg:flex",
            &["@container (width < 32rem)", "display: flex;"][..],
        ),
        (
            "@max-xl:hidden",
            &["@container (width < 36rem)", "display: none;"][..],
        ),
        (
            "@max-2xl:block",
            &["@container (width < 42rem)", "display: block;"][..],
        ),
        (
            "@max-3xl:flex",
            &["@container (width < 48rem)", "display: flex;"][..],
        ),
        (
            "@max-4xl:hidden",
            &["@container (width < 56rem)", "display: none;"][..],
        ),
        (
            "@max-5xl:block",
            &["@container (width < 64rem)", "display: block;"][..],
        ),
        (
            "@max-6xl:flex",
            &["@container (width < 72rem)", "display: flex;"][..],
        ),
        (
            "@max-7xl:block",
            &["@container (width < 80rem)", "display: block;"][..],
        ),
        (
            "@3xl/main:opacity-100",
            &["@container main (width >= 48rem)", "opacity: 100%;"][..],
        ),
    ] {
        let css = style::core::css_for_class(class_name).expect(class_name);

        for fragment in fragments {
            assert!(
                css.contains(fragment),
                "{class_name} missing {fragment:?}: {css}"
            );
        }
    }
}

#[test]
fn tailwind_container_query_range_variants_preserve_tailwind_variant_order() {
    for (class_name, first_fragment, second_fragment, declaration) in [
        (
            "@max-md:@sm:flex",
            "@container (width < 28rem)",
            "@container (width >= 24rem)",
            "display: flex;",
        ),
        (
            "@sm:@max-md:flex",
            "@container (width >= 24rem)",
            "@container (width < 28rem)",
            "display: flex;",
        ),
        (
            "@max-md/main:@sm/main:grid",
            "@container main (width < 28rem)",
            "@container main (width >= 24rem)",
            "display: grid;",
        ),
        (
            "@max-[960px]:@min-[475px]:hidden",
            "@container (width < 960px)",
            "@container (width >= 475px)",
            "display: none;",
        ),
        (
            "@max-[960px]/name:@min-[475px]/name:flex",
            "@container name (width < 960px)",
            "@container name (width >= 475px)",
            "display: flex;",
        ),
        (
            "@[475px]:@max-[960px]:block",
            "@container (width >= 475px)",
            "@container (width < 960px)",
            "display: block;",
        ),
        (
            "@[475px]/card:@max-[960px]/card:hidden",
            "@container card (width >= 475px)",
            "@container card (width < 960px)",
            "display: none;",
        ),
        (
            "@min-[40rem]:@max-[70rem]:flex",
            "@container (width >= 40rem)",
            "@container (width < 70rem)",
            "display: flex;",
        ),
        (
            "@min-[40rem]/main:@max-[70rem]/main:grid",
            "@container main (width >= 40rem)",
            "@container main (width < 70rem)",
            "display: grid;",
        ),
    ] {
        let css = style::core::css_for_class(class_name).expect(class_name);
        let first_index = css
            .find(first_fragment)
            .unwrap_or_else(|| panic!("{class_name} missing {first_fragment}: {css}"));
        let second_index = css
            .find(second_fragment)
            .unwrap_or_else(|| panic!("{class_name} missing {second_fragment}: {css}"));

        assert!(
            first_index < second_index,
            "{class_name} should preserve Tailwind's written container query order: {css}"
        );
        assert!(
            css.contains(declaration),
            "{class_name} missing {declaration}: {css}"
        );
    }

    for (class_name, fragments) in [
        (
            "@min-[123px]:flex",
            &["@container (width >= 123px)", "display: flex;"][..],
        ),
        (
            "@max-[123px]:hidden",
            &["@container (width < 123px)", "display: none;"][..],
        ),
        (
            "@min-[456px]/name:grid",
            &["@container name (width >= 456px)", "display: grid;"][..],
        ),
        (
            "@max-[456px]/name:block",
            &["@container name (width < 456px)", "display: block;"][..],
        ),
        (
            "@[475px]:flex",
            &["@container (width >= 475px)", "display: flex;"][..],
        ),
        (
            "@[475px]/card:grid",
            &["@container card (width >= 475px)", "display: grid;"][..],
        ),
    ] {
        let css = style::core::css_for_class(class_name).expect(class_name);
        for fragment in fragments {
            assert!(
                css.contains(fragment),
                "{class_name} missing {fragment}: {css}"
            );
        }
    }
}

#[test]
fn group_and_peer_state_variants_cover_tailwind_pseudo_class_families() {
    for (class_name, selector_fragment) in [
        (
            "group-odd:bg-mauve-500",
            ":is(:where(.group):nth-child(odd) *)",
        ),
        (
            "group-disabled:opacity-100",
            ":is(:where(.group):disabled *)",
        ),
        ("group-focus:text-slate-900", ":is(:where(.group):focus *)"),
        ("group-active:opacity-100", ":is(:where(.group):active *)"),
        (
            "group-focus-visible/card:opacity-100",
            ":is(:where(.group\\/card):focus-visible *)",
        ),
        (
            "group-focus/nav:text-slate-900",
            ":is(:where(.group\\/nav):focus *)",
        ),
        ("peer-invalid:visible", ":is(:where(.peer):invalid ~ *)"),
        ("peer-focus:opacity-100", ":is(:where(.peer):focus ~ *)"),
        ("peer-checked:opacity-100", ":is(:where(.peer):checked ~ *)"),
        (
            "peer-required/email:block",
            ":is(:where(.peer\\/email):required ~ *)",
        ),
        (
            "peer-checked/published:opacity-100",
            ":is(:where(.peer\\/published):checked ~ *)",
        ),
        (
            "peer-disabled:opacity-100",
            ":is(:where(.peer):disabled ~ *)",
        ),
    ] {
        let css = style::core::css_for_class(class_name).expect(class_name);

        assert!(
            css.contains(selector_fragment),
            "{class_name} missing selector {selector_fragment}: {css}"
        );
    }
}

#[test]
fn relational_state_variants_use_tailwind_relative_selectors_and_wrappers() {
    for (class_name, selector_fragment) in [
        ("has-checked:opacity-100", ":has(*:checked)"),
        ("has-disabled:opacity-100", ":has(*:disabled)"),
        ("not-checked:opacity-100", ":not(*:checked)"),
        ("in-checked:opacity-100", ":where(*:checked)"),
        (
            "group-has-checked:opacity-100",
            ":is(:where(.group):has(*:checked) *)",
        ),
        (
            "group-has-disabled/card:opacity-100",
            ":is(:where(.group\\/card):has(*:disabled) *)",
        ),
        (
            "peer-has-checked:opacity-100",
            ":is(:where(.peer):has(*:checked) ~ *)",
        ),
        (
            "peer-has-disabled/card:opacity-100",
            ":is(:where(.peer\\/card):has(*:disabled) ~ *)",
        ),
        (
            "group-not-disabled/card:opacity-100",
            ":is(:where(.group\\/card):not(*:disabled) *)",
        ),
        (
            "peer-not-checked:opacity-100",
            ":is(:where(.peer):not(*:checked) ~ *)",
        ),
        (
            "peer-not-disabled/card:opacity-100",
            ":is(:where(.peer\\/card):not(*:disabled) ~ *)",
        ),
    ] {
        let css = style::core::css_for_class(class_name).expect(class_name);

        assert!(
            css.contains(selector_fragment),
            "{class_name} missing selector {selector_fragment}: {css}"
        );
        assert!(
            css.contains("opacity: 100%;"),
            "{class_name} missing opacity declaration: {css}"
        );
    }
}

#[test]
fn group_and_peer_attribute_variants_cover_tailwind_aria_and_data_families() {
    for (class_name, fragments) in [
        (
            "group-aria-[sort=ascending]:rotate-0",
            &[
                ":is(:where(.group)[aria-sort=\"ascending\"] *)",
                "rotate: 0deg;",
            ][..],
        ),
        (
            "group-data-[state=open]/menu:block",
            &[
                ":is(:where(.group\\/menu)[data-state=\"open\"] *)",
                "display: block;",
            ],
        ),
        (
            "peer-aria-expanded/menu:opacity-100",
            &[
                ":is(:where(.peer\\/menu)[aria-expanded=\"true\"] ~ *)",
                "opacity: 100%;",
            ],
        ),
        (
            "peer-data-[side=top]:translate-y-1",
            &[
                ":is(:where(.peer)[data-side=\"top\"] ~ *)",
                "--tw-translate-y: calc(var(--spacing) * 1);",
            ],
        ),
    ] {
        let css = style::core::css_for_class(class_name).expect(class_name);

        for fragment in fragments {
            assert!(
                css.contains(fragment),
                "{class_name} missing {fragment:?}: {css}"
            );
        }
    }
}
