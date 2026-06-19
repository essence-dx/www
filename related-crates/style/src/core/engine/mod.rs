//! Style Engine module
//!
//! The core CSS generation engine that transforms utility class names into CSS rules.
//! Supports precompiled styles, dynamic generation, screens, states, container queries,
//! and color theming.
//!
//! Key components:
//! - `StyleEngine`: Main engine for CSS generation
//! - `GeneratorMeta`: Metadata for dynamic CSS generators
//! - `PropertyMeta`: CSS property definitions
//! - `ThemeDefinition`: Theme token definitions

mod apply;
mod authored_css;
pub mod browser_compat;
pub mod composite;
pub mod container_queries;
mod css_functions;
mod custom_utility;
pub mod directive_ledger;
pub mod dynamic;
pub mod equal_output;
pub mod feature_matrix;
pub mod parity;
pub mod postcss_compat;
pub mod screens;
pub mod states;
pub mod theme_css;
pub mod typography;
pub mod utility;
pub mod utility_ledger;

pub use composite::expand_composite;
pub use dynamic::generate_dynamic_css;
pub use screens::{build_block, sanitize_declarations, wrap_media_queries};
pub use states::{apply_not_hover_fallback_wrappers_and_states, apply_wrappers_and_states};

#[allow(dead_code)]
pub fn init() {}

use ahash::AHashMap;
use memmap2::{Mmap, MmapOptions};
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use self::custom_utility::{
    CustomUtilityContext, split_modifier, utility_to_css, utility_uses_modifier,
};
use crate::core::color::{color::Argb, format_argb_as_oklch, theme::ThemeBuilder};
use crate::serializer::StyleConfig;

const DX_FONT_TOKENS: &[(&str, &str)] = &[
    ("font-sans", "Geist, sans-serif"),
    ("font-serif", "Georgia, serif"),
    ("font-mono", "Geist Mono, monospace"),
];

const DX_BASE_TOKENS: &[(&str, &str)] = &[("radius", "0.5rem")];

const DEFAULT_THEME_SOURCE: u32 = 0xFF6750A4;

const DEFAULT_SCREEN_TOKENS: &[(&str, &str)] = &[
    ("sm", "640px"),
    ("md", "768px"),
    ("lg", "1024px"),
    ("xl", "1280px"),
    ("2xl", "1536px"),
];

const DEFAULT_CONTAINER_QUERY_TOKENS: &[(&str, &str)] = &[
    ("@3xs", "16rem"),
    ("@2xs", "18rem"),
    ("@xs", "20rem"),
    ("@sm", "24rem"),
    ("@md", "28rem"),
    ("@lg", "32rem"),
    ("@xl", "36rem"),
    ("@2xl", "42rem"),
    ("@3xl", "48rem"),
    ("@4xl", "56rem"),
    ("@5xl", "64rem"),
    ("@6xl", "72rem"),
    ("@7xl", "80rem"),
];

const THEME_FILTER_VALUE: &str = "var(--tw-blur, ) var(--tw-brightness, ) var(--tw-contrast, ) var(--tw-grayscale, ) var(--tw-hue-rotate, ) var(--tw-invert, ) var(--tw-saturate, ) var(--tw-sepia, ) var(--tw-drop-shadow, )";

const DEFAULT_STATE_TOKENS: &[(&str, &str)] = &[
    ("hover", ":hover"),
    ("focus", ":focus"),
    ("focus-visible", ":focus-visible"),
    ("focus-within", ":focus-within"),
    ("active", ":active"),
    ("visited", ":visited"),
    ("target", ":target"),
    ("autofill", ":autofill"),
    ("default", ":default"),
    ("disabled", ":disabled"),
    ("enabled", ":enabled"),
    ("checked", ":checked"),
    ("indeterminate", ":indeterminate"),
    ("in-range", ":in-range"),
    ("out-of-range", ":out-of-range"),
    ("invalid", ":invalid"),
    ("valid", ":valid"),
    ("user-invalid", ":user-invalid"),
    ("user-valid", ":user-valid"),
    ("optional", ":optional"),
    ("required", ":required"),
    ("placeholder-shown", ":placeholder-shown"),
    ("details-content", ":details-content"),
    ("read-only", ":read-only"),
    ("read-write", ":read-write"),
    ("empty", ":empty"),
    ("first", ":first-child"),
    ("last", ":last-child"),
    ("only", ":only-child"),
    ("first-of-type", ":first-of-type"),
    ("last-of-type", ":last-of-type"),
    ("only-of-type", ":only-of-type"),
    ("before", "::before"),
    ("after", "::after"),
    ("backdrop", "::backdrop"),
    ("placeholder", "::placeholder"),
    ("file", "&::file-selector-button"),
    ("marker", "&::marker, & *::marker"),
    ("selection", "&::selection, & *::selection"),
    ("first-letter", "::first-letter"),
    ("first-line", "::first-line"),
    ("odd", ":nth-child(odd)"),
    ("even", ":nth-child(even)"),
    ("group-hover", "&:is(:where(.group):hover *)"),
    ("group-focus", "&:is(:where(.group):focus *)"),
    ("group-active", "&:is(:where(.group):active *)"),
    ("peer-hover", "&:is(:where(.peer):hover ~ *)"),
    ("peer-focus", "&:is(:where(.peer):focus ~ *)"),
    ("peer-checked", "&:is(:where(.peer):checked ~ *)"),
    ("inert", "&:is([inert], [inert] *)"),
    ("rtl", "&:where(:dir(rtl), [dir=\"rtl\"], [dir=\"rtl\"] *)"),
    ("ltr", "&:where(:dir(ltr), [dir=\"ltr\"], [dir=\"ltr\"] *)"),
    ("open", "&:is([open], :popover-open, :open)"),
];

fn map_with_defaults(defaults: &[(&str, &str)]) -> AHashMap<String, String> {
    defaults
        .iter()
        .map(|(name, value)| ((*name).to_string(), (*value).to_string()))
        .collect()
}

fn merge_defaults(map: &mut AHashMap<String, String>, defaults: &[(&str, &str)]) {
    for (name, value) in defaults {
        map.entry((*name).to_string())
            .or_insert_with(|| (*value).to_string());
    }
}

fn insert_token(map: &mut AHashMap<String, String>, name: &str, value: &str, overwrite: bool) {
    if overwrite {
        map.insert(name.to_string(), value.to_string());
    } else {
        map.entry(name.to_string())
            .or_insert_with(|| value.to_string());
    }
}

fn remove_token(map: &mut AHashMap<String, String>, name: &str, overwrite: bool) {
    if overwrite {
        map.remove(name);
    }
}

fn insert_precompiled_token(
    precompiled: &mut AHashMap<String, String>,
    class_name: &str,
    declarations: &str,
    overwrite: bool,
) {
    if overwrite {
        precompiled.insert(class_name.to_string(), declarations.to_string());
    } else {
        precompiled
            .entry(class_name.to_string())
            .or_insert_with(|| declarations.to_string());
    }
}

fn is_font_family_theme_companion_token(token_name: &str) -> bool {
    token_name.strip_prefix("--font-").is_some_and(|name| {
        name.ends_with("--font-feature-settings") || name.ends_with("--font-variation-settings")
    })
}

fn font_family_theme_css(tokens: &[(String, String)], token_name: &str) -> String {
    let mut css = format!("font-family: var({token_name})");
    for (suffix, property) in [
        ("--font-feature-settings", "font-feature-settings"),
        ("--font-variation-settings", "font-variation-settings"),
    ] {
        let companion = format!("{token_name}{suffix}");
        if theme_token_exists(tokens, &companion) {
            css.push_str("; ");
            css.push_str(property);
            css.push_str(": var(");
            css.push_str(&companion);
            css.push(')');
        }
    }
    css
}

fn is_text_size_theme_companion_token(token_name: &str) -> bool {
    token_name.strip_prefix("--text-").is_some_and(|name| {
        name.ends_with("--line-height")
            || name.ends_with("--letter-spacing")
            || name.ends_with("--font-weight")
    })
}

fn text_size_theme_css(tokens: &[(String, String)], token_name: &str) -> String {
    let mut css = format!("font-size: var({token_name})");
    for (suffix, property) in [
        ("--line-height", "line-height"),
        ("--letter-spacing", "letter-spacing"),
        ("--font-weight", "font-weight"),
    ] {
        let companion = format!("{token_name}{suffix}");
        if theme_token_exists(tokens, &companion) {
            css.push_str("; ");
            css.push_str(property);
            css.push_str(": var(");
            css.push_str(&companion);
            css.push(')');
        }
    }
    css
}

fn theme_token_exists(tokens: &[(String, String)], token_name: &str) -> bool {
    tokens.iter().any(|(name, _)| name == token_name)
}

fn insert_spacing_token(precompiled: &mut AHashMap<String, String>, name: &str, overwrite: bool) {
    let value = format!("var(--spacing-{name})");
    let negative_value = format!("calc(var(--spacing-{name}) * -1)");
    let spacing_rules: &[(&str, &[&str], bool)] = &[
        ("p-", &["padding"], false),
        ("px-", &["padding-inline"], false),
        ("py-", &["padding-block"], false),
        ("ps-", &["padding-inline-start"], false),
        ("pe-", &["padding-inline-end"], false),
        ("pbs-", &["padding-block-start"], false),
        ("pbe-", &["padding-block-end"], false),
        ("pt-", &["padding-top"], false),
        ("pr-", &["padding-right"], false),
        ("pb-", &["padding-bottom"], false),
        ("pl-", &["padding-left"], false),
        ("m-", &["margin"], true),
        ("mx-", &["margin-inline"], true),
        ("my-", &["margin-block"], true),
        ("ms-", &["margin-inline-start"], true),
        ("me-", &["margin-inline-end"], true),
        ("mbs-", &["margin-block-start"], true),
        ("mbe-", &["margin-block-end"], true),
        ("mt-", &["margin-top"], true),
        ("mr-", &["margin-right"], true),
        ("mb-", &["margin-bottom"], true),
        ("ml-", &["margin-left"], true),
        ("gap-", &["gap"], false),
        ("gap-x-", &["column-gap"], false),
        ("gap-y-", &["row-gap"], false),
        ("inset-x-", &["inset-inline"], true),
        ("inset-y-", &["inset-block"], true),
        ("inset-s-", &["inset-inline-start"], true),
        ("inset-e-", &["inset-inline-end"], true),
        ("inset-bs-", &["inset-block-start"], true),
        ("inset-be-", &["inset-block-end"], true),
        ("inset-", &["inset"], true),
        ("start-", &["inset-inline-start"], true),
        ("end-", &["inset-inline-end"], true),
        ("top-", &["top"], true),
        ("right-", &["right"], true),
        ("bottom-", &["bottom"], true),
        ("left-", &["left"], true),
    ];

    for (prefix, properties, supports_negative) in spacing_rules {
        let class_name = format!("{prefix}{name}");
        insert_precompiled_token(
            precompiled,
            &class_name,
            &join_declarations(properties, &value),
            overwrite,
        );

        if *supports_negative {
            let class_name = format!("-{prefix}{name}");
            insert_precompiled_token(
                precompiled,
                &class_name,
                &join_declarations(properties, &negative_value),
                overwrite,
            );
        }
    }
}

fn join_declarations(properties: &[&str], value: &str) -> String {
    let mut css = String::new();
    for (index, property) in properties.iter().enumerate() {
        if index > 0 {
            css.push_str("; ");
        }
        css.push_str(property);
        css.push_str(": ");
        css.push_str(value);
    }
    css
}

fn last_variant_separator(class_name: &str) -> Option<usize> {
    let mut bracket_depth = 0usize;
    let mut paren_depth = 0usize;
    let mut last_colon = None;

    for (index, byte) in class_name.bytes().enumerate() {
        match byte {
            b'[' => bracket_depth = bracket_depth.saturating_add(1),
            b']' => bracket_depth = bracket_depth.saturating_sub(1),
            b'(' if bracket_depth == 0 => paren_depth = paren_depth.saturating_add(1),
            b')' if bracket_depth == 0 => paren_depth = paren_depth.saturating_sub(1),
            b':' if bracket_depth == 0 && paren_depth == 0 => last_colon = Some(index),
            _ => {}
        }
    }

    last_colon
}

fn split_important_modifier(class_name: &str) -> (&str, bool) {
    class_name
        .strip_prefix('!')
        .map_or((class_name, false), |stripped| (stripped, true))
}

fn important_lookup_class(class_name: &str) -> (String, bool) {
    let (class_name, important) = split_important_modifier(class_name);
    if important {
        return (class_name.to_string(), true);
    }

    if let Some(idx) = last_variant_separator(class_name) {
        let base = &class_name[idx + 1..];
        if let Some(base_without_bang) = base.strip_prefix('!') {
            let mut normalized = String::with_capacity(class_name.len() - 1);
            normalized.push_str(&class_name[..=idx]);
            normalized.push_str(base_without_bang);
            return (normalized, true);
        }
    }

    (class_name.to_string(), false)
}

fn mark_declaration_list_important(input: &str) -> String {
    input
        .split(';')
        .filter_map(|part| {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                return None;
            }
            if !trimmed.contains(':') || trimmed.ends_with("!important") {
                return Some(trimmed.to_string());
            }
            Some(format!("{trimmed} !important"))
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn mark_declarations_important(input: &str) -> String {
    let prefixes = [
        "BASE|",
        "STATE|",
        "CHILD|",
        "COND|",
        "DATA|",
        "NEST|",
        "NEST_SUPPORTS|",
        "CHILD_SUPPORTS|",
        "SUPPORTS|",
    ];
    let mark_line = |line: &str| -> String {
        if let Some(rest) = line.strip_prefix("INLINE_SUPPORTS|") {
            let mut parts = rest.splitn(3, '|');
            let condition = parts.next().unwrap_or("");
            let base_declarations = parts.next().unwrap_or("");
            let support_declarations = parts.next().unwrap_or("");
            return format!(
                "INLINE_SUPPORTS|{condition}|{}|{}",
                mark_declaration_list_important(base_declarations),
                mark_declaration_list_important(support_declarations)
            );
        }
        for prefix in prefixes {
            if let Some(rest) = line.strip_prefix(prefix) {
                if let Some((head, declarations)) = rest.rsplit_once('|') {
                    return format!(
                        "{prefix}{head}|{}",
                        mark_declaration_list_important(declarations)
                    );
                }
                return format!("{prefix}{}", mark_declaration_list_important(rest));
            }
        }
        mark_declaration_list_important(line)
    };

    if input.contains('\n') {
        input.lines().map(mark_line).collect::<Vec<_>>().join("\n")
    } else {
        mark_line(input)
    }
}

fn push_declaration_lines(output: &mut String, declarations: &str, indent: usize) {
    let indent = " ".repeat(indent);
    for declaration in sanitize_declarations(declarations)
        .trim()
        .trim_end_matches(';')
        .split(';')
    {
        let declaration = declaration.trim();
        if declaration.is_empty() {
            continue;
        }
        output.push_str(&indent);
        output.push_str(declaration);
        output.push_str(";\n");
    }
}

fn split_top_level_selector_list(selector: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut escaped = false;

    for (index, ch) in selector.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }

        if ch == '\\' {
            escaped = true;
            continue;
        }

        match ch {
            '(' => paren_depth = paren_depth.saturating_add(1),
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '[' => bracket_depth = bracket_depth.saturating_add(1),
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            ',' if paren_depth == 0 && bracket_depth == 0 => {
                let part = selector[start..index].trim();
                if !part.is_empty() {
                    parts.push(part);
                }
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }

    let part = selector[start..].trim();
    if !part.is_empty() {
        parts.push(part);
    }

    if parts.is_empty() {
        vec![selector]
    } else {
        parts
    }
}

fn compose_selector_wrapper(selector: &str, wrapper: &str) -> String {
    let selector_parts = split_top_level_selector_list(selector);
    let wrapper_parts = split_top_level_selector_list(wrapper);
    let mut composed = Vec::with_capacity(selector_parts.len() * wrapper_parts.len());

    for selector_part in selector_parts {
        for wrapper_part in &wrapper_parts {
            composed.push(wrapper_part.replace('&', selector_part));
        }
    }

    composed.join(",")
}

fn compose_selector_wrappers(selector: &str, wrappers: &[String]) -> String {
    wrappers
        .iter()
        .fold(selector.to_string(), |selector, wrapper| {
            compose_selector_wrapper(&selector, wrapper)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_engine_generates_tailwind_compatible_utilities() {
        let engine = StyleEngine::empty();

        let css = engine.css_for_class("p-4").expect("padding utility");
        assert!(css.contains(".p-4"));
        assert!(css.contains("padding: calc(var(--spacing) * 4);"));

        let css = engine.css_for_class("bg-blue-500").expect("color utility");
        assert!(css.contains("background-color: rgb(59 130 246);"));
    }

    #[test]
    fn empty_engine_supports_default_responsive_and_state_variants() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("md:hover:bg-blue-500")
            .expect("responsive hover utility");
        assert!(css.contains("@media (min-width: 768px)"));
        assert!(css.contains(".md\\:hover\\:bg-blue-500:hover"));
        assert!(css.contains("background-color: rgb(59 130 246);"));

        let css = engine
            .css_for_class("group-hover:text-slate-900")
            .expect("group hover utility");
        assert!(css.contains(":is(:where(.group):hover *)"));
        assert!(css.contains("color: rgb(15 23 42);"));
    }

    #[test]
    fn empty_engine_supports_common_form_and_location_state_variants() {
        let engine = StyleEngine::empty();

        let css = engine.css_for_class("target:p-4").expect("target variant");
        assert!(css.contains(":target"));
        assert!(css.contains("padding: calc(var(--spacing) * 4);"));

        let css = engine
            .css_for_class("read-only:bg-blue-500")
            .expect("read-only variant");
        assert!(css.contains(":read-only"));
        assert!(css.contains("background-color: rgb(59 130 246);"));

        let css = engine
            .css_for_class("indeterminate:opacity-100")
            .expect("indeterminate variant");
        assert!(css.contains(":indeterminate"));
        assert!(css.contains("opacity: 100%;"));
    }

    #[test]
    fn arbitrary_properties_keep_colons_inside_brackets() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("[border-collapse:collapse]")
            .expect("arbitrary property utility");

        assert!(css.contains("border-collapse: collapse"));
    }

    #[test]
    fn important_modifier_applies_to_plain_utility() {
        let engine = StyleEngine::empty();

        let css = engine.css_for_class("!p-4").expect("important padding");

        assert!(css.contains(".\\!p-4"));
        assert!(css.contains("padding: calc(var(--spacing) * 4) !important;"));
    }

    #[test]
    fn important_modifier_applies_after_variant_prefix() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("hover:!bg-blue-500")
            .expect("important hover background");

        assert!(css.contains(":hover"));
        assert!(css.contains("background-color: rgb(59 130 246) !important;"));
    }

    #[test]
    fn important_modifier_preserves_child_encoded_css() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("!space-x-4")
            .expect("important child spacing");

        assert!(css.contains("> :where(:not(:last-child))"));
        assert!(css.contains("--tw-space-x-reverse: 0 !important;"));
        assert!(css.contains(
            "margin-inline-end: calc(calc(var(--spacing) * 4) * calc(1 - var(--tw-space-x-reverse))) !important;"
        ));
    }

    #[test]
    fn data_variant_maps_to_attribute_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("data-open:bg-blue-500")
            .expect("data attribute variant");

        assert!(css.contains("[data-open]"));
        assert!(css.contains("background-color: rgb(59 130 246);"));
    }

    #[test]
    fn aria_variant_maps_to_true_attribute_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("aria-expanded:text-slate-900")
            .expect("aria attribute variant");

        assert!(css.contains("[aria-expanded=\"true\"]"));
        assert!(css.contains("color: rgb(15 23 42);"));
    }

    #[test]
    fn arbitrary_data_variant_maps_to_attribute_value_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("data-[state=open]:opacity-100")
            .expect("arbitrary data attribute variant");

        assert!(css.contains("[data-state=\"open\"]"));
        assert!(css.contains("opacity: 100%;"));
    }

    #[test]
    fn named_group_hover_variant_maps_to_named_group_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("group-hover/card:bg-blue-500")
            .expect("named group hover variant");

        assert!(css.contains(":is(:where(.group\\/card):hover *)"));
        assert!(css.contains("background-color: rgb(59 130 246);"));
    }

    #[test]
    fn named_group_focus_variant_maps_to_named_group_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("group-focus/nav:text-slate-900")
            .expect("named group focus variant");

        assert!(css.contains(":is(:where(.group\\/nav):focus *)"));
        assert!(css.contains("color: rgb(15 23 42);"));
    }

    #[test]
    fn named_peer_checked_variant_maps_to_named_peer_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("peer-checked/published:opacity-100")
            .expect("named peer checked variant");

        assert!(css.contains(":is(:where(.peer\\/published):checked ~ *)"));
        assert!(css.contains("opacity: 100%;"));
    }

    #[test]
    fn arbitrary_group_selector_variant_maps_to_group_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("group-[.is-open]:bg-blue-500")
            .expect("arbitrary group selector variant");

        assert!(css.contains(":is(:where(.group):is(.is-open) *)"));
        assert!(css.contains("background-color: rgb(59 130 246);"));
    }

    #[test]
    fn arbitrary_peer_selector_variant_maps_to_peer_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("peer-[.is-dirty]:opacity-100")
            .expect("arbitrary peer selector variant");

        assert!(css.contains(":is(:where(.peer):is(.is-dirty) ~ *)"));
        assert!(css.contains("opacity: 100%;"));
    }

    #[test]
    fn named_arbitrary_group_selector_variant_maps_to_named_group_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("group-[.is-open]/card:bg-blue-500")
            .expect("named arbitrary group selector variant");

        assert!(css.contains(":is(:where(.group\\/card):is(.is-open) *)"));
        assert!(css.contains("background-color: rgb(59 130 246);"));
    }

    #[test]
    fn named_arbitrary_peer_selector_variant_maps_to_named_peer_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("peer-[.is-dirty]/published:opacity-100")
            .expect("named arbitrary peer selector variant");

        assert!(css.contains(":is(:where(.peer\\/published):is(.is-dirty) ~ *)"));
        assert!(css.contains("opacity: 100%;"));
    }

    #[test]
    fn has_checked_variant_maps_to_has_pseudo_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("has-checked:bg-blue-500")
            .expect("has checked variant");

        assert!(css.contains(":has(*:checked)"));
        assert!(css.contains("background-color: rgb(59 130 246);"));
    }

    #[test]
    fn arbitrary_has_variant_maps_to_has_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("has-[img]:p-4")
            .expect("arbitrary has selector variant");

        assert!(css.contains(":has(img)"));
        assert!(css.contains("padding: calc(var(--spacing) * 4);"));
    }

    #[test]
    fn named_group_has_variant_maps_to_named_group_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("group-has-[a]/card:text-slate-900")
            .expect("named group has selector variant");

        assert!(css.contains(".group\\/card):has(a)"));
        assert!(css.contains("color: rgb(15 23 42);"));
    }

    #[test]
    fn named_peer_has_variant_maps_to_named_peer_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("peer-has-[input:checked]/published:opacity-100")
            .expect("named peer has selector variant");

        assert!(css.contains(".peer\\/published):has(input:checked) ~ *"));
        assert!(css.contains("opacity: 100%;"));
    }

    #[test]
    fn not_hover_variant_maps_to_not_pseudo_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("not-hover:bg-blue-500")
            .expect("not hover variant");

        assert!(css.contains(":not(*:hover)"));
        assert!(css.contains("background-color: rgb(59 130 246);"));
    }

    #[test]
    fn arbitrary_not_variant_maps_to_not_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("not-[.is-open]:p-4")
            .expect("arbitrary not selector variant");

        assert!(css.contains(":not(.is-open)"));
        assert!(css.contains("padding: calc(var(--spacing) * 4);"));
    }

    #[test]
    fn named_group_not_variant_maps_to_named_group_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("group-not-hover/card:text-slate-900")
            .expect("named group not selector variant");

        assert!(css.contains(".group\\/card):not(*:hover)"));
        assert!(css.contains("color: rgb(15 23 42);"));
    }

    #[test]
    fn named_peer_not_variant_maps_to_named_peer_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("peer-not-checked/published:opacity-100")
            .expect("named peer not selector variant");

        assert!(css.contains(".peer\\/published):not(*:checked) ~ *"));
        assert!(css.contains("opacity: 100%;"));
    }

    #[test]
    fn generic_arbitrary_child_selector_variant_maps_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("[&>svg]:bg-blue-500")
            .expect("generic arbitrary child selector variant");

        assert!(css.contains(">svg"));
        assert!(css.contains("background-color: rgb(59 130 246);"));
    }

    #[test]
    fn generic_arbitrary_state_selector_variant_maps_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("[&:nth-child(3)]:opacity-100")
            .expect("generic arbitrary state selector variant");

        assert!(css.contains(":nth-child(3)"));
        assert!(css.contains("opacity: 100%;"));
    }

    #[test]
    fn arbitrary_descendant_variant_decodes_underscore_to_space() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("[&_p]:mt-4")
            .expect("arbitrary descendant selector variant");

        assert!(css.contains(" p"));
        assert!(css.contains("margin-top: calc(var(--spacing) * 4);"));
    }

    #[test]
    fn arbitrary_adjacent_sibling_variant_maps_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("[&+*]:mt-4")
            .expect("arbitrary adjacent sibling selector variant");

        assert!(css.contains("+*"));
        assert!(css.contains("margin-top: calc(var(--spacing) * 4);"));
    }

    #[test]
    fn arbitrary_general_sibling_variant_maps_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("[&~*]:opacity-100")
            .expect("arbitrary general sibling selector variant");

        assert!(css.contains("~*"));
        assert!(css.contains("opacity: 100%;"));
    }

    #[test]
    fn arbitrary_variant_preserves_escaped_underscore() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("[&_.file\\_name]:mt-4")
            .expect("arbitrary selector variant with escaped underscore");

        assert!(css.contains(" .file_name"));
        assert!(css.contains("margin-top: calc(var(--spacing) * 4);"));
    }

    #[test]
    fn arbitrary_media_at_rule_variant_maps_to_media_query() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("[@media_(any-hover:hover)]:opacity-100")
            .expect("arbitrary media at-rule variant");

        assert!(css.contains("@media (any-hover:hover)"));
        assert!(css.contains("opacity: 100%;"));
    }

    #[test]
    fn arbitrary_supports_at_rule_variant_maps_to_supports_query() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("[@supports_(display:grid)]:grid")
            .expect("arbitrary supports at-rule variant");

        assert!(css.contains("@supports (display:grid)"));
        assert!(css.contains("display: grid;"));
    }

    #[test]
    fn arbitrary_container_at_rule_variant_maps_to_container_query() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("[@container_main_(min-width:_32rem)]:p-4")
            .expect("arbitrary container at-rule variant");

        assert!(css.contains("@container main (min-width: 32rem)"));
        assert!(css.contains("padding: calc(var(--spacing) * 4);"));
    }

    #[test]
    fn arbitrary_starting_style_at_rule_variant_maps_to_starting_style() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("[@starting-style]:opacity-0")
            .expect("arbitrary starting-style at-rule variant");

        assert!(css.contains("@starting-style"));
        assert!(css.contains("opacity: 0%;"));
    }

    #[test]
    fn arbitrary_layer_at_rule_variant_maps_to_cascade_layer() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("[@layer_components]:p-4")
            .expect("arbitrary layer at-rule variant");

        assert!(css.contains("@layer components"));
        assert!(css.contains("padding: calc(var(--spacing) * 4);"));
    }

    #[test]
    fn safe_unknown_arbitrary_at_rule_variant_generates_wrapped_css() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("[@unknown_rule]:p-4")
            .expect("safe unknown arbitrary at-rule variant");

        assert!(css.contains("@unknown rule"));
        assert!(css.contains("padding: calc(var(--spacing) * 4);"));
    }

    #[test]
    fn arbitrary_tailwind_runtime_directive_variants_fail_closed() {
        let engine = StyleEngine::empty();

        for class_name in [
            "[@plugin_foo]:p-4",
            "[@config_./tailwind.config.js]:p-4",
            "[@tailwind_utilities]:p-4",
            "[@import_\"tailwindcss\"]:p-4",
        ] {
            assert!(
                engine.css_for_class(class_name).is_none(),
                "{class_name} should not generate runtime Tailwind directive CSS"
            );
        }
    }

    #[test]
    fn direct_child_variant_maps_to_child_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("*:p-4")
            .expect("child selector variant");

        assert!(css.contains("> *"));
        assert!(css.contains("padding: calc(var(--spacing) * 4);"));
    }

    #[test]
    fn descendant_child_variant_maps_to_descendant_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("**:text-slate-900")
            .expect("descendant selector variant");

        assert!(css.contains(" *"));
        assert!(css.contains("color: rgb(15 23 42);"));
    }

    #[test]
    fn in_hover_variant_maps_to_ancestor_state_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("in-hover:bg-blue-500")
            .expect("in hover ancestor variant");

        assert!(css.contains(":where(*:hover)"));
        assert!(css.contains("background-color: rgb(59 130 246);"));
    }

    #[test]
    fn arbitrary_in_variant_maps_to_ancestor_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("in-[.is-open]:p-4")
            .expect("arbitrary in ancestor variant");

        assert!(css.contains(":where(.is-open)"));
        assert!(css.contains("padding: calc(var(--spacing) * 4);"));
    }

    #[test]
    fn has_even_variant_maps_to_even_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("has-even:bg-blue-500")
            .expect("has even selector variant");

        assert!(css.contains(":has(*:nth-child(even))"));
        assert!(css.contains("background-color: rgb(59 130 246);"));
    }

    #[test]
    fn not_visited_variant_maps_to_visited_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("not-visited:text-slate-900")
            .expect("not visited selector variant");

        assert!(css.contains(":not(*:visited)"));
        assert!(css.contains("color: rgb(15 23 42);"));
    }

    #[test]
    fn in_read_only_variant_maps_to_read_only_ancestor_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("in-read-only:p-4")
            .expect("in read-only ancestor variant");

        assert!(css.contains(":where(*:read-only)"));
        assert!(css.contains("padding: calc(var(--spacing) * 4);"));
    }

    #[test]
    fn numeric_nth_variant_maps_to_nth_child_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("nth-3:bg-blue-500")
            .expect("numeric nth child variant");

        assert!(css.contains(":nth-child(3)"));
        assert!(css.contains("background-color: rgb(59 130 246);"));
    }

    #[test]
    fn arbitrary_nth_variant_maps_to_nth_child_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("nth-[2n+1]:p-4")
            .expect("arbitrary nth child variant");

        assert!(css.contains(":nth-child(2n+1)"));
        assert!(css.contains("padding: calc(var(--spacing) * 4);"));
    }

    #[test]
    fn nth_last_variant_maps_to_nth_last_child_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("nth-last-2:bg-blue-500")
            .expect("nth last child variant");

        assert!(css.contains(":nth-last-child(2)"));
        assert!(css.contains("background-color: rgb(59 130 246);"));
    }

    #[test]
    fn arbitrary_nth_of_type_variant_maps_to_nth_of_type_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("nth-of-type-[3n+1]:p-4")
            .expect("arbitrary nth of type variant");

        assert!(css.contains(":nth-of-type(3n+1)"));
        assert!(css.contains("padding: calc(var(--spacing) * 4);"));
    }

    #[test]
    fn nth_last_of_type_variant_maps_to_nth_last_of_type_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("nth-last-of-type-2:opacity-100")
            .expect("nth last of type variant");

        assert!(css.contains(":nth-last-of-type(2)"));
        assert!(css.contains("opacity: 100%;"));
    }

    #[test]
    fn only_child_variant_maps_to_only_child_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("only:bg-blue-500")
            .expect("only child variant");

        assert!(css.contains(":only-child"));
        assert!(css.contains("background-color: rgb(59 130 246);"));
    }

    #[test]
    fn first_of_type_variant_maps_to_first_of_type_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("first-of-type:p-4")
            .expect("first of type variant");

        assert!(css.contains(":first-of-type"));
        assert!(css.contains("padding: calc(var(--spacing) * 4);"));
    }

    #[test]
    fn last_of_type_variant_maps_to_last_of_type_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("last-of-type:text-slate-900")
            .expect("last of type variant");

        assert!(css.contains(":last-of-type"));
        assert!(css.contains("color: rgb(15 23 42);"));
    }

    #[test]
    fn only_of_type_variant_maps_to_only_of_type_selector() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("only-of-type:opacity-100")
            .expect("only of type variant");

        assert!(css.contains(":only-of-type"));
        assert!(css.contains("opacity: 100%;"));
    }

    #[test]
    fn before_variant_maps_to_before_pseudo_element() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("before:bg-blue-500")
            .expect("before pseudo-element variant");

        assert!(css.contains("::before"));
        assert!(css.contains("background-color: rgb(59 130 246);"));
    }

    #[test]
    fn after_variant_maps_to_after_pseudo_element() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("after:opacity-100")
            .expect("after pseudo-element variant");

        assert!(css.contains("::after"));
        assert!(css.contains("opacity: 100%;"));
    }

    #[test]
    fn placeholder_variant_maps_to_placeholder_pseudo_element() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("placeholder:text-slate-900")
            .expect("placeholder pseudo-element variant");

        assert!(css.contains("::placeholder"));
        assert!(css.contains("color: rgb(15 23 42);"));
    }

    #[test]
    fn file_variant_maps_to_file_selector_button_pseudo_element() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("file:p-4")
            .expect("file selector button pseudo-element variant");

        assert!(css.contains("::file-selector-button"));
        assert!(!css.contains("::-webkit-file-upload-button"));
        assert!(css.contains("padding: calc(var(--spacing) * 4);"));
    }

    #[test]
    fn marker_variant_maps_to_marker_pseudo_element() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("marker:text-slate-900")
            .expect("marker pseudo-element variant");

        assert!(css.contains("::marker"));
        assert!(css.contains("color: rgb(15 23 42);"));
    }

    #[test]
    fn selection_variant_maps_to_selection_pseudo_element() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("selection:bg-blue-500")
            .expect("selection pseudo-element variant");

        assert!(css.contains("::selection"));
        assert!(css.contains("background-color: rgb(59 130 246);"));
    }

    #[test]
    fn first_letter_variant_maps_to_first_letter_pseudo_element() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("first-letter:text-slate-900")
            .expect("first-letter pseudo-element variant");

        assert!(css.contains("::first-letter"));
        assert!(css.contains("color: rgb(15 23 42);"));
    }

    #[test]
    fn first_line_variant_maps_to_first_line_pseudo_element() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("first-line:bg-blue-500")
            .expect("first-line pseudo-element variant");

        assert!(css.contains("::first-line"));
        assert!(css.contains("background-color: rgb(59 130 246);"));
    }

    #[test]
    fn before_content_variant_maps_to_before_pseudo_element() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("before:content-['New']")
            .expect("before content pseudo-element variant");

        assert!(css.contains("::before"));
        assert!(css.contains("content: 'New';"));
    }

    #[test]
    fn after_content_variable_variant_maps_to_after_pseudo_element() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("after:content-(--dx-suffix)")
            .expect("after content pseudo-element variable variant");

        assert!(css.contains("::after"));
        assert!(css.contains("content: var(--dx-suffix);"));
    }

    #[test]
    fn typography_prose_generates_nested_rules() {
        let engine = StyleEngine::empty();

        let css = engine.css_for_class("prose").expect("prose typography");

        assert!(css.contains(".prose {"));
        assert!(css.contains("max-width: 65ch;"));
        assert!(css.contains(".prose :where(h1)"));
        assert!(css.contains("font-size: 2.25em;"));
        assert!(css.contains(".prose :where(a)"));
        assert!(css.contains("text-decoration: underline;"));
        assert!(css.contains(".prose :where(pre)"));
        assert!(css.contains("overflow-x: auto;"));
    }

    #[test]
    fn typography_prose_variants_generate_css() {
        let engine = StyleEngine::empty();

        let responsive_css = engine
            .css_for_class("md:prose")
            .expect("responsive prose typography");
        assert!(responsive_css.contains("@media (min-width: 768px)"));
        assert!(responsive_css.contains(".md\\:prose :where(p)"));

        let invert_css = engine
            .css_for_class("prose-invert")
            .expect("prose invert variables");
        assert!(invert_css.contains("--tw-prose-body: rgb(209 213 219);"));

        let large_css = engine
            .css_for_class("prose-lg")
            .expect("large prose typography");
        assert!(large_css.contains("font-size: 1.125rem;"));

        let link_css = engine
            .css_for_class("prose-a:text-blue-600")
            .expect("prose link modifier");
        assert!(link_css.contains(".prose-a\\:text-blue-600 :where(a)"));
        assert!(link_css.contains("color: rgb(37 99 235);"));
    }

    #[test]
    fn token_aware_color_utilities_generate_css() {
        let engine = StyleEngine::empty();

        let background = engine
            .css_for_class("bg-token(surface)")
            .expect("surface token background");
        assert!(background.contains("background-color: hsl(var(--surface));"));

        let text = engine
            .css_for_class("text-token(foreground)")
            .expect("foreground token text");
        assert!(text.contains("color: hsl(var(--foreground));"));

        let border = engine
            .css_for_class("border-token(border)")
            .expect("border token color");
        assert!(border.contains("border-color: hsl(var(--border));"));

        let ring = engine
            .css_for_class("ring-token(ring)")
            .expect("ring token color");
        assert!(ring.contains("--tw-ring-color: hsl(var(--ring));"));

        assert!(engine.css_for_class("bg-token(does-not-exist)").is_none());
    }

    #[test]
    fn font_stretch_utilities_generate_css() {
        let engine = StyleEngine::empty();

        let condensed_css = engine
            .css_for_class("font-stretch-condensed")
            .expect("font stretch condensed");
        assert!(condensed_css.contains("font-stretch: condensed;"));

        let percentage_css = engine
            .css_for_class("font-stretch-50%")
            .expect("font stretch percentage");
        assert!(percentage_css.contains("font-stretch: 50%;"));

        let arbitrary_css = engine
            .css_for_class("font-stretch-[62.5%]")
            .expect("font stretch arbitrary percentage");
        assert!(arbitrary_css.contains("font-stretch: 62.5%;"));

        let variable_css = engine
            .css_for_class("font-stretch-(--dx-font-stretch)")
            .expect("font stretch variable");
        assert!(variable_css.contains("font-stretch: var(--dx-font-stretch);"));
    }

    #[test]
    fn numeric_font_variant_utilities_generate_css() {
        let engine = StyleEngine::empty();

        let normal_css = engine.css_for_class("normal-nums").expect("normal nums");
        assert!(normal_css.contains("font-variant-numeric: normal;"));

        let ordinal_css = engine.css_for_class("ordinal").expect("ordinal");
        assert!(ordinal_css.contains("--tw-ordinal: ordinal;"));
        assert!(ordinal_css.contains("font-variant-numeric:"));

        let tabular_css = engine.css_for_class("tabular-nums").expect("tabular nums");
        assert!(tabular_css.contains("--tw-numeric-spacing: tabular-nums;"));
        assert!(tabular_css.contains("font-variant-numeric:"));

        let fraction_css = engine
            .css_for_class("diagonal-fractions")
            .expect("diagonal fractions");
        assert!(fraction_css.contains("--tw-numeric-fraction: diagonal-fractions;"));
        assert!(fraction_css.contains("font-variant-numeric:"));
    }

    #[test]
    fn forced_color_adjust_utilities_generate_css() {
        let engine = StyleEngine::empty();

        let auto_css = engine
            .css_for_class("forced-color-adjust-auto")
            .expect("forced color adjust auto");
        assert!(auto_css.contains("forced-color-adjust: auto;"));

        let none_css = engine
            .css_for_class("forced-color-adjust-none")
            .expect("forced color adjust none");
        assert!(none_css.contains("forced-color-adjust: none;"));
    }

    #[test]
    fn outline_and_ring_edge_utilities_generate_css() {
        let engine = StyleEngine::empty();

        let outline_width = engine.css_for_class("outline-2").expect("outline width");
        assert!(outline_width.contains("outline-width: 2px;"));

        let outline_style = engine
            .css_for_class("outline-dashed")
            .expect("outline style");
        assert!(outline_style.contains("outline-style: dashed;"));

        let ring_inset = engine.css_for_class("ring-inset").expect("ring inset");
        assert!(ring_inset.contains("--tw-ring-inset: inset;"));

        let ring_offset = engine
            .css_for_class("ring-offset-2")
            .expect("ring offset width");
        assert!(ring_offset.contains("--tw-ring-offset-width: 2px;"));

        let ring = engine.css_for_class("ring-2").expect("ring width");
        assert!(ring.contains("var(--tw-ring-inset,)"));
    }

    #[test]
    fn touch_action_edge_utilities_generate_css() {
        let engine = StyleEngine::empty();

        let left = engine
            .css_for_class("touch-pan-left")
            .expect("touch pan left");
        assert!(left.contains("--tw-pan-x: pan-left;"));
        assert!(
            left.contains("touch-action: var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,);")
        );

        let right = engine
            .css_for_class("touch-pan-right")
            .expect("touch pan right");
        assert!(right.contains("--tw-pan-x: pan-right;"));
        assert!(
            right
                .contains("touch-action: var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,);")
        );

        let up = engine.css_for_class("touch-pan-up").expect("touch pan up");
        assert!(up.contains("--tw-pan-y: pan-up;"));
        assert!(
            up.contains("touch-action: var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,);")
        );

        let down = engine
            .css_for_class("touch-pan-down")
            .expect("touch pan down");
        assert!(down.contains("--tw-pan-y: pan-down;"));
        assert!(
            down.contains("touch-action: var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,);")
        );

        let pinch = engine
            .css_for_class("touch-pinch-zoom")
            .expect("touch pinch zoom");
        assert!(pinch.contains("--tw-pinch-zoom: pinch-zoom;"));
        assert!(
            pinch
                .contains("touch-action: var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,);")
        );
    }

    #[test]
    fn columns_utilities_generate_css() {
        let engine = StyleEngine::empty();

        let count_css = engine.css_for_class("columns-3").expect("column count");
        assert!(count_css.contains("columns: 3;"));

        let auto_css = engine.css_for_class("columns-auto").expect("auto columns");
        assert!(auto_css.contains("columns: auto;"));

        let width_css = engine.css_for_class("columns-lg").expect("column width");
        assert!(width_css.contains("columns: 32rem;"));

        let arbitrary_css = engine
            .css_for_class("columns-[14rem]")
            .expect("arbitrary column width");
        assert!(arbitrary_css.contains("columns: 14rem;"));

        let variable_css = engine
            .css_for_class("columns-(--dx-column-width)")
            .expect("column width variable");
        assert!(variable_css.contains("columns: var(--dx-column-width);"));
    }

    #[test]
    fn break_utilities_generate_css() {
        let engine = StyleEngine::empty();

        let before_css = engine
            .css_for_class("break-before-page")
            .expect("break before page");
        assert!(before_css.contains("break-before: page;"));

        let after_css = engine
            .css_for_class("break-after-avoid-page")
            .expect("break after avoid page");
        assert!(after_css.contains("break-after: avoid-page;"));

        let inside_css = engine
            .css_for_class("break-inside-avoid-column")
            .expect("break inside avoid column");
        assert!(inside_css.contains("break-inside: avoid-column;"));
    }

    #[test]
    fn box_decoration_utilities_generate_prefixed_css() {
        let engine = StyleEngine::empty();

        let clone_css = engine
            .css_for_class("box-decoration-clone")
            .expect("box decoration clone");
        assert!(clone_css.contains("-webkit-box-decoration-break: clone;"));
        assert!(clone_css.contains("box-decoration-break: clone;"));

        let slice_css = engine
            .css_for_class("box-decoration-slice")
            .expect("box decoration slice");
        assert!(slice_css.contains("-webkit-box-decoration-break: slice;"));
        assert!(slice_css.contains("box-decoration-break: slice;"));
    }

    #[test]
    fn browser_prefix_utilities_generate_prefixed_css() {
        let engine = StyleEngine::empty();

        let appearance_css = engine
            .css_for_class("appearance-none")
            .expect("appearance none");
        assert!(appearance_css.contains("-webkit-appearance: none;"));
        assert!(appearance_css.contains("appearance: none;"));

        let select_css = engine.css_for_class("select-none").expect("select none");
        assert!(select_css.contains("-webkit-user-select: none;"));
        assert!(select_css.contains("user-select: none;"));

        let backface_css = engine
            .css_for_class("backface-hidden")
            .expect("backface hidden");
        assert!(backface_css.contains("-webkit-backface-visibility: hidden;"));
        assert!(backface_css.contains("backface-visibility: hidden;"));

        let break_inside_css = engine
            .css_for_class("break-inside-avoid")
            .expect("break inside avoid");
        assert!(break_inside_css.contains("page-break-inside: avoid;"));
        assert!(break_inside_css.contains("break-inside: avoid;"));

        let backdrop_css = engine
            .css_for_class("backdrop-blur-md")
            .expect("backdrop blur");
        assert!(backdrop_css.contains("-webkit-backdrop-filter:"));
        assert!(backdrop_css.contains("backdrop-filter:"));
    }

    #[test]
    fn blend_mode_utilities_generate_css() {
        let engine = StyleEngine::empty();

        let background_css = engine
            .css_for_class("bg-blend-multiply")
            .expect("background blend mode");
        assert!(background_css.contains("background-blend-mode: multiply;"));

        let mix_css = engine
            .css_for_class("mix-blend-plus-lighter")
            .expect("mix blend mode");
        assert!(mix_css.contains("mix-blend-mode: plus-lighter;"));

        let arbitrary_css = engine
            .css_for_class("mix-blend-[screen]")
            .expect("arbitrary mix blend mode");
        assert!(arbitrary_css.contains("mix-blend-mode: screen;"));

        let variable_css = engine
            .css_for_class("bg-blend-(--dx-bg-blend)")
            .expect("background blend variable");
        assert!(variable_css.contains("background-blend-mode: var(--dx-bg-blend);"));
    }

    #[test]
    fn background_origin_utilities_generate_css() {
        let engine = StyleEngine::empty();

        let border_css = engine
            .css_for_class("bg-origin-border")
            .expect("background origin border");
        assert!(border_css.contains("background-origin: border-box;"));

        let padding_css = engine
            .css_for_class("bg-origin-padding")
            .expect("background origin padding");
        assert!(padding_css.contains("background-origin: padding-box;"));

        let content_css = engine
            .css_for_class("bg-origin-content")
            .expect("background origin content");
        assert!(content_css.contains("background-origin: content-box;"));

        let none_css = engine.css_for_class("bg-none").expect("background none");
        assert!(none_css.contains("background-image: none;"));

        let custom_size_css = engine
            .css_for_class("bg-size-(--dx-bg-size)")
            .expect("background custom size");
        assert!(custom_size_css.contains("background-size: var(--dx-bg-size);"));

        let custom_position_css = engine
            .css_for_class("bg-position-(--dx-bg-position)")
            .expect("background custom position");
        assert!(custom_position_css.contains("background-position: var(--dx-bg-position);"));

        let linear_css = engine
            .css_for_class("bg-linear-to-r")
            .expect("background linear right");
        assert!(
            linear_css
                .contains("background-image: linear-gradient(to right, var(--tw-gradient-stops));")
        );

        let linear_angle_css = engine
            .css_for_class("bg-linear-45")
            .expect("background linear angle");
        assert!(
            linear_angle_css
                .contains("background-image: linear-gradient(45deg, var(--tw-gradient-stops));")
        );

        let linear_custom_css = engine
            .css_for_class("bg-linear-(--dx-bg-linear)")
            .expect("background linear custom");
        assert!(linear_custom_css.contains(
            "background-image: linear-gradient(var(--dx-bg-linear), var(--tw-gradient-stops));"
        ));

        let radial_css = engine
            .css_for_class("bg-radial")
            .expect("background radial");
        assert!(
            radial_css.contains("background-image: radial-gradient(var(--tw-gradient-stops));")
        );

        let radial_position_css = engine
            .css_for_class("bg-radial-[circle_at_center]")
            .expect("background radial position");
        assert!(radial_position_css.contains(
            "background-image: radial-gradient(circle at center, var(--tw-gradient-stops));"
        ));

        let conic_css = engine
            .css_for_class("bg-conic-180")
            .expect("background conic");
        assert!(
            conic_css.contains(
                "background-image: conic-gradient(from 180deg, var(--tw-gradient-stops));"
            )
        );

        let conic_custom_css = engine
            .css_for_class("bg-conic-(--dx-bg-conic)")
            .expect("background conic custom");
        assert!(conic_custom_css.contains(
            "background-image: conic-gradient(var(--dx-bg-conic), var(--tw-gradient-stops));"
        ));

        let arbitrary_image_css = engine
            .css_for_class("bg-[url('/hero.png')]")
            .expect("background arbitrary image");
        assert!(arbitrary_image_css.contains("background-image: url('/hero.png');"));
    }

    #[test]
    fn mask_utilities_generate_prefixed_css() {
        let engine = StyleEngine::empty();

        let none_css = engine.css_for_class("mask-none").expect("mask none");
        assert!(none_css.contains("-webkit-mask-image: none;"));
        assert!(none_css.contains("mask-image: none;"));

        let alpha_css = engine.css_for_class("mask-alpha").expect("mask alpha");
        assert!(alpha_css.contains("-webkit-mask-mode: alpha;"));
        assert!(alpha_css.contains("mask-mode: alpha;"));

        let luminance_css = engine
            .css_for_class("mask-luminance")
            .expect("mask luminance");
        assert!(luminance_css.contains("-webkit-mask-mode: luminance;"));
        assert!(luminance_css.contains("mask-mode: luminance;"));

        let match_css = engine.css_for_class("mask-match").expect("mask match");
        assert!(match_css.contains("-webkit-mask-mode: match-source;"));
        assert!(match_css.contains("mask-mode: match-source;"));

        let mask_type_alpha_css = engine
            .css_for_class("mask-type-alpha")
            .expect("mask type alpha");
        assert!(mask_type_alpha_css.contains("mask-type: alpha;"));

        let mask_type_luminance_css = engine
            .css_for_class("mask-type-luminance")
            .expect("mask type luminance");
        assert!(mask_type_luminance_css.contains("mask-type: luminance;"));

        let origin_content_css = engine
            .css_for_class("mask-origin-content")
            .expect("mask origin content");
        assert!(origin_content_css.contains("-webkit-mask-origin: content-box;"));
        assert!(origin_content_css.contains("mask-origin: content-box;"));

        let origin_view_css = engine
            .css_for_class("mask-origin-view")
            .expect("mask origin view");
        assert!(origin_view_css.contains("-webkit-mask-origin: view-box;"));
        assert!(origin_view_css.contains("mask-origin: view-box;"));

        let clip_padding_css = engine
            .css_for_class("mask-clip-padding")
            .expect("mask clip padding");
        assert!(clip_padding_css.contains("-webkit-mask-clip: padding-box;"));
        assert!(clip_padding_css.contains("mask-clip: padding-box;"));

        let no_clip_css = engine.css_for_class("mask-no-clip").expect("mask no clip");
        assert!(no_clip_css.contains("-webkit-mask-clip: no-clip;"));
        assert!(no_clip_css.contains("mask-clip: no-clip;"));

        let radial_from_css = engine
            .css_for_class("mask-radial-from-50%")
            .expect("radial mask from stop");
        assert!(radial_from_css.contains("--tw-mask-radial-from: 50%;"));
        assert!(radial_from_css.contains("-webkit-mask-image: radial-gradient("));
        assert!(radial_from_css.contains("mask-image: radial-gradient("));
        assert!(radial_from_css.contains("var(--tw-mask-radial-size"));

        let radial_to_css = engine
            .css_for_class("mask-radial-to-90%")
            .expect("radial mask to stop");
        assert!(radial_to_css.contains("--tw-mask-radial-to: 90%;"));
        assert!(radial_to_css.contains(
            "var(--tw-mask-radial-to-color, transparent) var(--tw-mask-radial-to, 100%)"
        ));

        let radial_size_css = engine
            .css_for_class("mask-radial-[100%_100%]")
            .expect("arbitrary radial mask size");
        assert!(radial_size_css.contains("--tw-mask-radial-size: 100% 100%;"));
        assert!(radial_size_css.contains("-webkit-mask-image: radial-gradient("));
        assert!(radial_size_css.contains("mask-image: radial-gradient("));

        let radial_position_css = engine
            .css_for_class("mask-radial-at-[35%_35%]")
            .expect("arbitrary radial mask position");
        assert!(radial_position_css.contains("--tw-mask-radial-position: 35% 35%;"));
        assert!(!radial_position_css.contains("mask-image:"));

        let radial_shape_css = engine.css_for_class("mask-circle").expect("radial shape");
        assert!(radial_shape_css.contains("--tw-mask-radial-shape: circle;"));

        let radial_stop_color_css = engine
            .css_for_class("mask-radial-from-red-500")
            .expect("radial from color");
        assert!(
            radial_stop_color_css.contains("--tw-mask-radial-from-color: var(--color-red-500);")
        );

        let conic_from_css = engine
            .css_for_class("mask-conic-from-50%")
            .expect("conic mask from stop");
        assert!(conic_from_css.contains("--tw-mask-conic-from: 50%;"));
        assert!(conic_from_css.contains("-webkit-mask-image: conic-gradient("));
        assert!(conic_from_css.contains("mask-image: conic-gradient("));

        let conic_to_css = engine
            .css_for_class("mask-conic-to-75%")
            .expect("conic mask to stop");
        assert!(conic_to_css.contains("--tw-mask-conic-to: 75%;"));
        assert!(conic_to_css.contains("var(--tw-mask-conic-to, 100%)"));

        let conic_angle_css = engine
            .css_for_class("mask-conic-45")
            .expect("conic mask angle");
        assert!(conic_angle_css.contains("--tw-mask-conic-angle: 45deg;"));

        let left_from_css = engine
            .css_for_class("mask-l-from-50%")
            .expect("left edge mask from stop");
        assert!(left_from_css.contains("--tw-mask-left-from: 50%;"));
        assert!(left_from_css.contains("-webkit-mask-image: linear-gradient(to left,"));

        let left_to_css = engine
            .css_for_class("mask-l-to-90%")
            .expect("left edge mask to stop");
        assert!(left_to_css.contains("--tw-mask-left-to: 90%;"));
        assert!(left_to_css.contains("var(--tw-mask-left-to, 100%)"));

        let x_from_css = engine
            .css_for_class("mask-x-from-70%")
            .expect("x-axis mask from stop");
        assert!(x_from_css.contains("--tw-mask-left-from: 70%;"));
        assert!(x_from_css.contains("--tw-mask-right-from: 70%;"));
        assert!(x_from_css.contains("mask-composite: intersect;"));

        let y_to_css = engine
            .css_for_class("mask-y-to-90%")
            .expect("y-axis mask to stop");
        assert!(y_to_css.contains("--tw-mask-top-to: 90%;"));
        assert!(y_to_css.contains("--tw-mask-bottom-to: 90%;"));

        let linear_angle_css = engine
            .css_for_class("mask-linear-50")
            .expect("linear mask angle");
        assert!(linear_angle_css.contains("--tw-mask-linear-position: 50deg;"));

        let negative_linear_angle_css = engine
            .css_for_class("-mask-linear-50")
            .expect("negative linear mask angle");
        assert!(negative_linear_angle_css.contains("--tw-mask-linear-position: calc(50deg * -1);"));

        let linear_from_css = engine
            .css_for_class("mask-linear-from-60%")
            .expect("linear mask from stop");
        assert!(linear_from_css.contains("--tw-mask-linear-from: 60%;"));

        let linear_to_css = engine
            .css_for_class("mask-linear-to-80%")
            .expect("linear mask to stop");
        assert!(linear_to_css.contains("--tw-mask-linear-to: 80%;"));

        let arbitrary_linear_css = engine
            .css_for_class("mask-linear-[70deg,transparent_10%,black,transparent_80%]")
            .expect("arbitrary linear mask image");
        assert!(arbitrary_linear_css.contains(
            "-webkit-mask-image: linear-gradient(70deg,transparent 10%,black,transparent 80%);"
        ));
        assert!(
            arbitrary_linear_css.contains(
                "mask-image: linear-gradient(70deg,transparent 10%,black,transparent 80%);"
            )
        );

        let variable_linear_css = engine
            .css_for_class("mask-linear-(--launch-mask)")
            .expect("custom-property linear mask image");
        assert!(variable_linear_css.contains("-webkit-mask-image: var(--launch-mask);"));
        assert!(variable_linear_css.contains("mask-image: var(--launch-mask);"));

        let composite_css = engine.css_for_class("mask-add").expect("mask composite");
        assert!(composite_css.contains("-webkit-mask-composite: source-over;"));
        assert!(composite_css.contains("mask-composite: add;"));

        let position_css = engine
            .css_for_class("mask-position-[center_top]")
            .expect("arbitrary mask position");
        assert!(position_css.contains("-webkit-mask-position: center top;"));
        assert!(position_css.contains("mask-position: center top;"));
    }

    #[test]
    fn tailwind_container_query_variant_maps_default_token() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("@sm:bg-blue-500")
            .expect("default Tailwind-style container query variant");

        assert!(css.contains("@container (width >= 24rem)"));
        assert!(css.contains("background-color: rgb(59 130 246);"));
    }

    #[test]
    fn tailwind_container_query_variant_maps_arbitrary_min_width() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("@[475px]:text-slate-900")
            .expect("arbitrary Tailwind-style container query variant");

        assert!(css.contains("@container (width >= 475px)"));
        assert!(css.contains("color: rgb(15 23 42);"));
    }

    #[test]
    fn tailwind_container_query_variant_composes_with_state() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("@lg:hover:opacity-100")
            .expect("container query state variant");

        assert!(css.contains("@container (width >= 32rem)"));
        assert!(css.contains(":hover"));
        assert!(css.contains("opacity: 100%;"));
    }

    #[test]
    fn tailwind_named_container_query_variant_maps_token() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("@lg/main:bg-blue-500")
            .expect("named Tailwind-style container query variant");

        assert!(css.contains("@container main (width >= 32rem)"));
        assert!(css.contains("background-color: rgb(59 130 246);"));
    }

    #[test]
    fn tailwind_max_container_query_variant_maps_default_token() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("@max-md:text-slate-900")
            .expect("max-width Tailwind-style container query variant");

        assert!(css.contains("@container (width < 28rem)"));
        assert!(css.contains("color: rgb(15 23 42);"));
    }

    #[test]
    fn tailwind_arbitrary_max_container_query_variant_maps_width() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("@max-[960px]:opacity-100")
            .expect("arbitrary max-width Tailwind-style container query variant");

        assert!(css.contains("@container (width < 960px)"));
        assert!(css.contains("opacity: 100%;"));
    }

    #[test]
    fn tailwind_max_screen_variant_maps_default_token() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("max-md:text-slate-900")
            .expect("max-width screen variant");

        assert!(css.contains("@media (max-width: 767.98px)"));
        assert!(css.contains("color: rgb(15 23 42);"));
    }

    #[test]
    fn tailwind_arbitrary_media_query_variants_map_widths() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("min-[475px]:bg-blue-500")
            .expect("arbitrary min-width media variant");
        assert!(css.contains("@media (min-width: 475px)"));
        assert!(css.contains("background-color: rgb(59 130 246);"));

        let css = engine
            .css_for_class("max-[960px]:opacity-100")
            .expect("arbitrary max-width media variant");
        assert!(css.contains("@media (max-width: 960px)"));
        assert!(css.contains("opacity: 100%;"));
    }

    #[test]
    fn tailwind_environment_media_variants_map_queries() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("motion-safe:animate-spin")
            .expect("motion-safe media variant");
        assert!(css.contains("@media (prefers-reduced-motion: no-preference)"));
        assert!(css.contains("animation: dx-spin 1s linear infinite;"));

        let css = engine
            .css_for_class("motion-reduce:transition-none")
            .expect("motion-reduce media variant");
        assert!(css.contains("@media (prefers-reduced-motion: reduce)"));
        assert!(css.contains("transition-property: none;"));

        let css = engine
            .css_for_class("portrait:block")
            .expect("portrait media variant");
        assert!(css.contains("@media (orientation: portrait)"));
        assert!(css.contains("display: block;"));

        let css = engine
            .css_for_class("landscape:hidden")
            .expect("landscape media variant");
        assert!(css.contains("@media (orientation: landscape)"));
        assert!(css.contains("display: none;"));

        let css = engine.css_for_class("print:hidden").expect("print variant");
        assert!(css.contains("@media print"));
        assert!(css.contains("display: none;"));
    }

    #[test]
    fn tailwind_supports_feature_query_variant_maps_condition() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("supports-[display:grid]:grid")
            .expect("supports feature query variant");

        assert!(css.contains("@supports (display: grid)"));
        assert!(css.contains("display: grid;"));

        let css = engine
            .css_for_class("supports-[backdrop-filter:blur(0)]:backdrop-blur-md")
            .expect("supports backdrop feature query variant");

        assert!(css.contains("@supports (backdrop-filter: blur(0))"));
        assert!(css.contains("--tw-backdrop-blur: blur(var(--blur-md));"));
    }

    #[test]
    fn tailwind_not_supports_feature_query_variant_maps_condition() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("not-supports-[display:grid]:block")
            .expect("not supports feature query variant");

        assert!(css.contains("@supports not (display: grid)"));
        assert!(css.contains("display: block;"));
    }

    #[test]
    fn transition_property_utility_maps_colors() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("transition-colors")
            .expect("transition colors utility");

        assert!(css.contains(
            "transition-property: color, background-color, border-color, text-decoration-color, fill, stroke"
        ));
        assert!(css.contains("transition-duration: 150ms;"));
    }

    #[test]
    fn transition_duration_delay_and_easing_utilities_generate_css() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("duration-300")
            .expect("transition duration utility");
        assert!(css.contains("transition-duration: 300ms;"));

        let css = engine
            .css_for_class("delay-200")
            .expect("transition delay utility");
        assert!(css.contains("transition-delay: 200ms;"));

        let css = engine
            .css_for_class("ease-in-out")
            .expect("transition easing utility");
        assert!(css.contains("transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);"));
    }

    #[test]
    fn transition_arbitrary_values_generate_css() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("duration-[375ms]")
            .expect("arbitrary transition duration utility");
        assert!(css.contains("transition-duration: 375ms;"));

        let css = engine
            .css_for_class("ease-[cubic-bezier(0.2,_0,_0,_1)]")
            .expect("arbitrary transition easing utility");
        assert!(css.contains("transition-timing-function: cubic-bezier(0.2, 0, 0, 1);"));
    }

    #[test]
    fn transition_property_aliases_generate_css() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("transition-[height,opacity]")
            .expect("arbitrary transition property utility");
        assert!(css.contains("transition-property: height,opacity;"));

        let css = engine
            .css_for_class("transition-(--dx-transition-property)")
            .expect("CSS variable transition property utility");
        assert!(css.contains("transition-property: var(--dx-transition-property);"));
    }

    #[test]
    fn transition_behavior_utilities_generate_css() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("transition-discrete")
            .expect("transition discrete behavior utility");
        assert!(css.contains("transition-behavior: allow-discrete;"));

        let css = engine
            .css_for_class("transition-normal")
            .expect("transition normal behavior utility");
        assert!(css.contains("transition-behavior: normal;"));
    }

    #[test]
    fn tailwind_animation_spin_generates_keyframes() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("animate-spin")
            .expect("Tailwind-style spin animation");

        assert!(css.contains("@keyframes dx-spin"));
        assert!(css.contains("transform: rotate(360deg);"));
        assert!(css.contains("animation: dx-spin 1s linear infinite;"));
    }

    #[test]
    fn tailwind_animation_pulse_and_bounce_generate_keyframes() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("animate-pulse")
            .expect("Tailwind-style pulse animation");
        assert!(css.contains("@keyframes dx-pulse"));
        assert!(css.contains("animation: dx-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;"));

        let css = engine
            .css_for_class("animate-bounce")
            .expect("Tailwind-style bounce animation");
        assert!(css.contains("@keyframes dx-bounce"));
        assert!(css.contains("animation: dx-bounce 1s infinite;"));
    }

    #[test]
    fn tailwind_animation_none_and_arbitrary_values_generate_css() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("animate-none")
            .expect("Tailwind-style no animation utility");
        assert!(css.contains("animation: none;"));

        let css = engine
            .css_for_class("animate-[wiggle_1s_ease-in-out_infinite]")
            .expect("Tailwind-style arbitrary animation utility");
        assert!(css.contains("animation: wiggle 1s ease-in-out infinite;"));
    }

    #[test]
    fn tailwind_animation_css_variable_alias_generates_css() {
        let engine = StyleEngine::empty();

        let css = engine
            .css_for_class("animate-(--dx-animation-enter)")
            .expect("Tailwind-style CSS variable animation utility");

        assert!(css.contains("animation: var(--dx-animation-enter);"));
    }
}

#[derive(Clone)]
pub struct GeneratorMeta {
    pub prefix: String,
    pub property: String,
    pub multiplier: f32,
    pub unit: String,
}

pub struct StyleEngine {
    pub(crate) precompiled: AHashMap<String, String>,
    pub(crate) _mmap: Arc<Mmap>,
    pub screens: AHashMap<String, String>,
    pub states: AHashMap<String, String>,
    pub container_queries: AHashMap<String, String>,
    pub colors: AHashMap<String, String>,
    pub generators: Option<Vec<GeneratorMeta>>,
    pub generator_map: Option<AHashMap<String, usize>>,
    pub(crate) custom_variants: AHashMap<String, theme_css::CssCustomVariant>,
    #[allow(dead_code)]
    pub properties: Vec<PropertyMeta>,
    #[allow(dead_code)]
    pub themes: Vec<ThemeDefinition>,
    pub theme_lookup: AHashMap<String, usize>,
    pub property_css: String,
    pub base_layer_raw: Option<String>,
    pub property_layer_raw: Option<String>,
    custom_utilities: Vec<theme_css::CssUtilityDefinition>,
    source_inline_classes: Vec<String>,
    source_inline_exclusions: Vec<String>,
    uses_custom_theme_css: bool,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PropertyMeta {
    pub name: String,
    pub syntax: String,
    pub inherits: bool,
    pub initial: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ThemeDefinition {
    pub name: String,
    pub tokens: Vec<(String, String)>,
}

impl StyleEngine {
    pub fn load_from_disk() -> Result<Self, Box<dyn std::error::Error>> {
        let override_path = std::env::var("DX_STYLE_DXM").ok();
        let path_buf;
        let path = if let Some(p) = override_path.as_deref() {
            path_buf = std::path::PathBuf::from(p);
            &path_buf
        } else {
            Path::new(".dx/style/style.dxm")
        };

        // Read the DX Machine format file
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        // Deserialize using DX Serializer
        let config = StyleConfig::from_binary(&mmap)
            .map_err(|e| format!("Failed to parse style.dxm: {}", e))?;

        // Build precompiled styles from static styles
        let mut precompiled = AHashMap::new();
        for (name, css) in &config.static_styles {
            if name.is_empty() || css.is_empty() {
                continue;
            }
            precompiled.insert(
                name.clone(),
                css.trim_end().trim_end_matches(';').to_string(),
            );
        }

        // Add dynamic styles to precompiled
        for (key, entry) in &config.dynamic_styles {
            for (suffix, value) in &entry.values {
                let name = if suffix.is_empty() {
                    key.clone()
                } else {
                    format!("{}-{}", key, suffix)
                };
                if !name.is_empty() {
                    let css = format!(
                        "{}: {}",
                        entry.property,
                        value.trim_end().trim_end_matches(';')
                    );
                    precompiled.insert(name, css);
                }
            }
        }

        // Convert screens
        let mut screens: AHashMap<String, String> = config.screens.into_iter().collect();
        merge_defaults(&mut screens, DEFAULT_SCREEN_TOKENS);

        // Convert states
        let mut states: AHashMap<String, String> = config.states.into_iter().collect();
        merge_defaults(&mut states, DEFAULT_STATE_TOKENS);

        // Convert container queries
        let mut container_queries: AHashMap<String, String> =
            config.container_queries.into_iter().collect();
        merge_defaults(&mut container_queries, DEFAULT_CONTAINER_QUERY_TOKENS);

        // Convert colors
        let colors: AHashMap<String, String> = config.colors.into_iter().collect();

        // Convert generators
        let generators: Option<Vec<GeneratorMeta>> = if config.generators.is_empty() {
            None
        } else {
            Some(
                config
                    .generators
                    .iter()
                    .map(|g| GeneratorMeta {
                        prefix: g.prefix.clone(),
                        property: g.property.clone(),
                        multiplier: g.multiplier,
                        unit: g.unit.clone(),
                    })
                    .collect(),
            )
        };

        let generator_map = generators.as_ref().map(|vec| {
            let mut m = AHashMap::new();
            for (i, g) in vec.iter().enumerate() {
                m.insert(g.prefix.clone(), i);
            }
            m
        });

        // Properties and themes are not yet stored in StyleConfig
        // These will be added in a future iteration
        let properties: Vec<PropertyMeta> = Vec::new();
        let themes: Vec<ThemeDefinition> = Vec::new();
        let theme_lookup: AHashMap<String, usize> = AHashMap::new();

        // Generate property CSS (empty for now since properties aren't loaded)
        let property_css = String::new();

        let base_layer_raw = if config.base_css.is_empty() {
            None
        } else {
            Some(config.base_css.clone())
        };

        let property_layer_raw = if config.property_css.is_empty() {
            None
        } else {
            Some(config.property_css.clone())
        };

        let mut engine = Self {
            precompiled,
            _mmap: Arc::new(mmap),
            screens,
            states,
            container_queries,
            colors,
            generators,
            generator_map,
            custom_variants: AHashMap::new(),
            properties,
            themes,
            theme_lookup,
            property_css,
            base_layer_raw,
            property_layer_raw,
            custom_utilities: Vec::new(),
            source_inline_classes: Vec::new(),
            source_inline_exclusions: Vec::new(),
            uses_custom_theme_css: false,
        };
        engine.apply_default_theme_css();
        Ok(engine)
    }

    pub fn empty() -> Self {
        let override_path = std::env::var("DX_STYLE_DXM").ok();
        let default_path = override_path.as_deref().unwrap_or(".dx/style/style.dxm");
        let file = File::options()
            .read(true)
            .write(false)
            .open(default_path)
            .ok();
        let mmap = file.and_then(|f| unsafe { Mmap::map(&f).ok() });
        fn anon_read_only_mmap() -> Option<Mmap> {
            let anon = MmapOptions::new().len(1).map_anon().ok()?;
            anon.make_read_only().ok()
        }
        let mut engine = StyleEngine {
            precompiled: AHashMap::new(),
            _mmap: Arc::new(mmap.unwrap_or_else(|| {
                let file = File::options()
                    .read(true)
                    .write(false)
                    .open(default_path)
                    .ok();
                if let Some(file) = file {
                    // SAFETY: Mmap::map requires unsafe but is safe when file is valid
                    unsafe { Mmap::map(&file) }
                        .ok()
                        .or_else(anon_read_only_mmap)
                        .unwrap_or_else(|| {
                            // Fallback to empty mmap if all else fails
                            MmapOptions::new()
                                .len(1)
                                .map_anon()
                                .ok()
                                .and_then(|m| m.make_read_only().ok())
                                .unwrap_or_else(|| {
                                    // Last resort: create minimal valid mmap
                                    // SAFETY: Creating zeroed Mmap is safe as fallback
                                    unsafe { std::mem::zeroed() }
                                })
                        })
                } else {
                    anon_read_only_mmap().unwrap_or_else(|| {
                        // SAFETY: Creating zeroed Mmap is safe as fallback
                        unsafe { std::mem::zeroed() }
                    })
                }
            })),
            screens: map_with_defaults(DEFAULT_SCREEN_TOKENS),
            states: map_with_defaults(DEFAULT_STATE_TOKENS),
            container_queries: map_with_defaults(DEFAULT_CONTAINER_QUERY_TOKENS),
            colors: AHashMap::new(),
            generators: None,
            generator_map: None,
            custom_variants: AHashMap::new(),
            properties: Vec::new(),
            themes: Vec::new(),
            theme_lookup: AHashMap::new(),
            property_css: String::new(),
            base_layer_raw: None,
            property_layer_raw: None,
            custom_utilities: Vec::new(),
            source_inline_classes: Vec::new(),
            source_inline_exclusions: Vec::new(),
            uses_custom_theme_css: false,
        };
        engine.apply_default_theme_css();
        engine
    }

    pub fn from_theme_css(source: &str) -> Self {
        let mut engine = Self::empty();
        engine.apply_theme_css(source);
        engine
    }

    pub fn apply_theme_css(&mut self, source: &str) {
        let definition = theme_css::parse_theme_css(source);
        self.apply_theme_definition(&definition, true);
        self.ensure_registered_custom_properties();
        self.uses_custom_theme_css = true;
    }

    fn apply_default_theme_css(&mut self) {
        let definition = theme_css::parse_theme_css(theme_css::DEFAULT_DX_THEME_CSS);
        self.apply_theme_definition(&definition, false);
        self.ensure_registered_custom_properties();
    }

    fn ensure_registered_custom_properties(&mut self) {
        if !self.property_css.contains("@property --tw-gradient-from") {
            if !self.property_css.trim().is_empty() {
                self.property_css.push('\n');
            }
            self.property_css
                .push_str(&theme_css::registered_custom_properties_css());
        }
    }

    fn apply_theme_definition(
        &mut self,
        definition: &theme_css::ThemeCssDefinition,
        overwrite: bool,
    ) {
        for variant in &definition.custom_variants {
            if overwrite {
                self.custom_variants
                    .insert(variant.name.clone(), variant.clone());
            } else {
                self.custom_variants
                    .entry(variant.name.clone())
                    .or_insert_with(|| variant.clone());
            }
        }

        if overwrite {
            for utility in &definition.utilities {
                self.custom_utilities
                    .retain(|existing| existing.name != utility.name);
            }
        }
        self.custom_utilities
            .extend(definition.utilities.iter().cloned());

        if overwrite {
            self.source_inline_classes.clear();
            self.source_inline_exclusions.clear();
        }
        for directive in &definition.source_directives {
            match directive {
                theme_css::CssSourceDirective::Inline(classes) => {
                    self.source_inline_classes.extend(classes.iter().cloned());
                }
                theme_css::CssSourceDirective::InlineExclude(classes) => {
                    self.source_inline_exclusions
                        .extend(classes.iter().cloned());
                }
                _ => {}
            }
        }
        self.source_inline_classes.sort();
        self.source_inline_classes.dedup();
        self.source_inline_exclusions.sort();
        self.source_inline_exclusions.dedup();

        if definition.tokens.is_empty() {
            return;
        }

        for (token_name, token_value) in &definition.tokens {
            if self.apply_theme_namespace_reset(token_name, token_value, overwrite) {
                continue;
            }

            if let Some(name) = token_name.strip_prefix("--color-") {
                insert_token(&mut self.colors, name, token_value, overwrite);
            } else if let Some(name) = token_name.strip_prefix("--breakpoint-") {
                insert_token(&mut self.screens, name, token_value, overwrite);
            } else if let Some(name) = token_name.strip_prefix("--container-") {
                if name == "*" {
                    continue;
                }
                let variant = format!("@{name}");
                insert_token(
                    &mut self.container_queries,
                    &variant,
                    token_value,
                    overwrite,
                );
            } else if let Some(name) = token_name.strip_prefix("--spacing-") {
                insert_spacing_token(&mut self.precompiled, name, overwrite);
            } else if let Some(name) = token_name.strip_prefix("--radius-") {
                insert_precompiled_token(
                    &mut self.precompiled,
                    &format!("rounded-{name}"),
                    &format!("border-radius: var({token_name})"),
                    overwrite,
                );
            } else if let Some(name) = token_name.strip_prefix("--shadow-") {
                insert_precompiled_token(
                    &mut self.precompiled,
                    &format!("shadow-{name}"),
                    &format!("box-shadow: var({token_name})"),
                    overwrite,
                );
            } else if let Some(name) = token_name.strip_prefix("--drop-shadow-") {
                insert_precompiled_token(
                    &mut self.precompiled,
                    &format!("drop-shadow-{name}"),
                    &format!(
                        "--tw-drop-shadow: var({token_name}); filter: {}",
                        THEME_FILTER_VALUE
                    ),
                    overwrite,
                );
            } else if let Some(name) = token_name.strip_prefix("--inset-shadow-") {
                insert_precompiled_token(
                    &mut self.precompiled,
                    &format!("inset-shadow-{name}"),
                    &format!("box-shadow: var({token_name})"),
                    overwrite,
                );
            } else if let Some(name) = token_name.strip_prefix("--animate-") {
                insert_precompiled_token(
                    &mut self.precompiled,
                    &format!("animate-{name}"),
                    &format!("animation: var({token_name})"),
                    overwrite,
                );
            } else if let Some(name) = token_name.strip_prefix("--font-weight-") {
                insert_precompiled_token(
                    &mut self.precompiled,
                    &format!("font-{name}"),
                    &format!("font-weight: var({token_name})"),
                    overwrite,
                );
            } else if is_font_family_theme_companion_token(token_name) {
                continue;
            } else if let Some(name) = token_name.strip_prefix("--font-") {
                insert_precompiled_token(
                    &mut self.precompiled,
                    &format!("font-{name}"),
                    &font_family_theme_css(&definition.tokens, token_name),
                    overwrite,
                );
            } else if is_text_size_theme_companion_token(token_name) {
                continue;
            } else if let Some(name) = token_name.strip_prefix("--text-") {
                insert_precompiled_token(
                    &mut self.precompiled,
                    &format!("text-{name}"),
                    &text_size_theme_css(&definition.tokens, token_name),
                    overwrite,
                );
            } else if let Some(name) = token_name.strip_prefix("--leading-") {
                insert_precompiled_token(
                    &mut self.precompiled,
                    &format!("leading-{name}"),
                    &format!("line-height: var({token_name})"),
                    overwrite,
                );
            } else if let Some(name) = token_name.strip_prefix("--tracking-") {
                insert_precompiled_token(
                    &mut self.precompiled,
                    &format!("tracking-{name}"),
                    &format!("letter-spacing: var({token_name})"),
                    overwrite,
                );
            } else if let Some(name) = token_name.strip_prefix("--ease-") {
                insert_precompiled_token(
                    &mut self.precompiled,
                    &format!("ease-{name}"),
                    &format!("transition-timing-function: var({token_name})"),
                    overwrite,
                );
            } else if let Some(name) = token_name.strip_prefix("--duration-") {
                insert_precompiled_token(
                    &mut self.precompiled,
                    &format!("duration-{name}"),
                    &format!("transition-duration: var({token_name})"),
                    overwrite,
                );
            } else if let Some(name) = token_name.strip_prefix("--aspect-") {
                insert_precompiled_token(
                    &mut self.precompiled,
                    &format!("aspect-{name}"),
                    &format!("aspect-ratio: var({token_name})"),
                    overwrite,
                );
            } else if let Some(name) = token_name.strip_prefix("--perspective-") {
                insert_precompiled_token(
                    &mut self.precompiled,
                    &format!("perspective-{name}"),
                    &format!("perspective: var({token_name})"),
                    overwrite,
                );
            } else if let Some(name) = token_name.strip_prefix("--blur-") {
                insert_precompiled_token(
                    &mut self.precompiled,
                    &format!("blur-{name}"),
                    &format!(
                        "--tw-blur: blur(var({token_name})); filter: {}",
                        THEME_FILTER_VALUE
                    ),
                    overwrite,
                );
            }
        }

        let theme_name = if overwrite {
            "theme-css"
        } else {
            "default-theme-css"
        };
        if let Some(index) = self.theme_lookup.get(theme_name).copied() {
            if overwrite {
                self.themes[index].tokens = definition.tokens.clone();
            }
        } else {
            let index = self.themes.len();
            self.theme_lookup.insert(theme_name.to_string(), index);
            self.themes.push(ThemeDefinition {
                name: theme_name.to_string(),
                tokens: definition.tokens.clone(),
            });
        }
    }

    fn apply_theme_namespace_reset(
        &mut self,
        token_name: &str,
        token_value: &str,
        overwrite: bool,
    ) -> bool {
        if !theme_css::is_theme_initial_reset_value(token_value) {
            return false;
        }

        if token_name == "--container-*" {
            if overwrite {
                self.container_queries.clear();
            }
            return true;
        }

        if let Some(name) = token_name.strip_prefix("--container-") {
            let variant = format!("@{name}");
            remove_token(&mut self.container_queries, &variant, overwrite);
            return true;
        }

        false
    }

    pub fn property_at_rules(&self) -> String {
        self.property_css.clone()
    }

    pub fn theme_by_name(&self, name: &str) -> Option<&ThemeDefinition> {
        self.theme_lookup
            .get(name)
            .and_then(|idx| self.themes.get(*idx))
    }

    pub fn css_apply_rules_from_source(&self, source: &str) -> String {
        apply::css_apply_rules_from_source(self, source)
    }

    pub fn css_authored_function_rules_from_source(&self, source: &str) -> String {
        authored_css::css_authored_function_rules_from_source(source)
    }

    pub fn css_variant_rules_from_source(&self, source: &str) -> String {
        authored_css::css_variant_rules_from_source(self, source)
    }

    pub fn source_inline_class_tokens(&self) -> &[String] {
        &self.source_inline_classes
    }

    pub fn source_inline_exclusion_class_tokens(&self) -> &[String] {
        &self.source_inline_exclusions
    }

    pub fn class_is_source_inline_excluded(&self, class_name: &str) -> bool {
        self.source_inline_exclusions
            .binary_search_by(|candidate| candidate.as_str().cmp(class_name))
            .is_ok()
    }

    fn custom_utility_css(&self, class_name: &str) -> Option<String> {
        for utility in self.custom_utilities.iter().rev() {
            let uses_modifier = utility_uses_modifier(utility);
            let (lookup_class, modifier) = if uses_modifier {
                split_modifier(class_name)
            } else {
                (class_name, None)
            };

            if utility.name == lookup_class {
                let context = CustomUtilityContext {
                    value: None,
                    modifier,
                    themes: &self.themes,
                };
                return utility_to_css(utility, context);
            }

            let Some(prefix) = utility.name.strip_suffix('*') else {
                continue;
            };
            if prefix.ends_with('-') && lookup_class == prefix.trim_end_matches('-') {
                let context = CustomUtilityContext {
                    value: None,
                    modifier,
                    themes: &self.themes,
                };
                if let Some(css) = utility_to_css(utility, context) {
                    return Some(css);
                }
            }

            let Some(value) = lookup_class.strip_prefix(prefix) else {
                continue;
            };
            if value.is_empty() {
                continue;
            }
            let context = CustomUtilityContext {
                value: Some(value),
                modifier,
                themes: &self.themes,
            };
            if let Some(css) = utility_to_css(utility, context) {
                return Some(css);
            }
        }

        None
    }

    pub fn compute_css(&self, class_name: &str) -> Option<String> {
        if self.uses_custom_theme_css {
            return self.compute_css_uncached(class_name);
        }

        // MEMOIZATION: Check cache first for instant return
        crate::core::lazy_gen::get_or_generate_css(class_name, || {
            self.compute_css_uncached(class_name)
        })
    }

    fn compute_css_uncached(&self, class_name: &str) -> Option<String> {
        let original_class_name = class_name;
        let (class_name, important) = important_lookup_class(class_name);
        let class_name = class_name.as_str();
        // FAST PATH: Check atomic class perfect hash first (<1µs lookup)
        if let Some(css) = crate::core::atomic::lookup_atomic_class(class_name) {
            let css = if important {
                mark_declarations_important(css)
            } else {
                css.to_string()
            };
            let mut escaped_ident = String::with_capacity(class_name.len() + 8);
            struct Acc<'a> {
                buf: &'a mut String,
            }
            impl<'a> std::fmt::Write for Acc<'a> {
                fn write_str(&mut self, s: &str) -> std::fmt::Result {
                    self.buf.push_str(s);
                    Ok(())
                }
            }
            if cssparser::serialize_identifier(
                original_class_name,
                &mut Acc {
                    buf: &mut escaped_ident,
                },
            )
            .is_err()
            {
                for ch in original_class_name.chars() {
                    match ch {
                        ':' => escaped_ident.push_str("\\:"),
                        '!' => escaped_ident.push_str("\\!"),
                        '@' => escaped_ident.push_str("\\@"),
                        '(' => escaped_ident.push_str("\\("),
                        ')' => escaped_ident.push_str("\\)"),
                        ' ' => escaped_ident.push_str("\\ "),
                        '/' => escaped_ident.push_str("\\/"),
                        '\\' => escaped_ident.push_str("\\\\"),
                        _ => escaped_ident.push(ch),
                    }
                }
            }
            return Some(format!(".{} {{ {} }}", escaped_ident, css));
        }

        // SLOW PATH: Full CSS generation for non-atomic classes
        if class_name.starts_with("from(")
            || class_name.starts_with("to(")
            || class_name.starts_with("via(")
        {
            return None;
        }
        let last_colon = last_variant_separator(class_name);
        let (prefix_segment, base_class) = if let Some(idx) = last_colon {
            (&class_name[..idx], &class_name[idx + 1..])
        } else {
            ("", class_name)
        };
        let (media_queries, pseudo_classes, wrappers) =
            crate::core::engine::apply_wrappers_and_states(self, prefix_segment)?;
        let core_css_raw = crate::core::engine::expand_composite(self, class_name)
            .or_else(|| self.precompiled.get(base_class).cloned())
            .or_else(|| self.custom_utility_css(base_class))
            .or_else(|| crate::core::color::generate_color_css(self, base_class))
            .or_else(|| {
                if class_name.contains(' ') {
                    None
                } else {
                    crate::core::animation::generate_animation_css(base_class)
                        .or_else(|| crate::core::animation::generate_animation_css(class_name))
                }
            })
            .or_else(|| crate::core::engine::generate_dynamic_css(self, base_class))
            .or_else(|| crate::core::engine::typography::generate_typography_css(base_class))
            .or_else(|| crate::core::engine::utility::generate_utility_css(base_class))
            .or_else(|| crate::core::engine::expand_composite(self, base_class));
        core_css_raw.map(|mut css| {
            css = crate::core::engine::sanitize_declarations(&css);
            let important_css = if important {
                mark_declarations_important(&css)
            } else {
                css
            };
            css = important_css;
            let mut escaped_ident = String::with_capacity(class_name.len() + 8);
            struct Acc<'a> {
                buf: &'a mut String,
            }
            impl<'a> std::fmt::Write for Acc<'a> {
                fn write_str(&mut self, s: &str) -> std::fmt::Result {
                    self.buf.push_str(s);
                    Ok(())
                }
            }
            if cssparser::serialize_identifier(
                original_class_name,
                &mut Acc {
                    buf: &mut escaped_ident,
                },
            )
            .is_err()
            {
                for ch in original_class_name.chars() {
                    match ch {
                        ':' => escaped_ident.push_str("\\:"),
                        '!' => escaped_ident.push_str("\\!"),
                        '@' => escaped_ident.push_str("\\@"),
                        '(' => escaped_ident.push_str("\\("),
                        ')' => escaped_ident.push_str("\\)"),
                        ' ' => escaped_ident.push_str("\\ "),
                        '/' => escaped_ident.push_str("\\/"),
                        '\\' => escaped_ident.push_str("\\\\"),
                        _ => escaped_ident.push(ch),
                    }
                }
            }
            let mut selector =
                String::with_capacity(escaped_ident.len() + pseudo_classes.len() + 2);
            selector.push('.');
            selector.push_str(&escaped_ident);
            selector.push_str(&pseudo_classes);
            let blocks = self.decode_encoded_css(&css, &selector, &wrappers);
            let mut output = crate::core::engine::wrap_media_queries(blocks, &media_queries);
            if let Some((fallback_media_queries, fallback_pseudo_classes, fallback_wrappers)) =
                crate::core::engine::apply_not_hover_fallback_wrappers_and_states(
                    self,
                    prefix_segment,
                )
            {
                let mut fallback_selector =
                    String::with_capacity(escaped_ident.len() + fallback_pseudo_classes.len() + 2);
                fallback_selector.push('.');
                fallback_selector.push_str(&escaped_ident);
                fallback_selector.push_str(&fallback_pseudo_classes);
                let fallback_blocks =
                    self.decode_encoded_css(&css, &fallback_selector, &fallback_wrappers);
                output.push_str(&crate::core::engine::wrap_media_queries(
                    fallback_blocks,
                    &fallback_media_queries,
                ));
            }
            output
        })
    }

    pub fn css_for_class(&self, class: &str) -> Option<String> {
        self.compute_css(class)
    }

    pub fn generate_color_vars_for<'a, I>(&self, classes: I) -> (String, String)
    where
        I: IntoIterator<Item = &'a String>,
    {
        use std::collections::BTreeSet;
        use std::fmt::Write as _;

        let mut needed: BTreeSet<&str> = BTreeSet::new();
        for c in classes.into_iter() {
            let base = c.rsplit(':').next().unwrap_or(c);
            if let Some(name) = base.strip_prefix("bg-") {
                needed.insert(name);
            }
            if let Some(name) = base.strip_prefix("text-") {
                needed.insert(name);
            }
        }

        let mut token_entries: Vec<(String, String)> = Vec::new();

        for name in &needed {
            if let Some(mut val) = crate::core::color::derive_color_value(self, name) {
                if let Some(oklch) = crate::core::color::normalize_color_to_oklch(&val) {
                    val = oklch;
                }
                token_entries.push(((*name).to_string(), val));
            }
        }

        let format_token_value = |raw: &str| -> String {
            let trimmed = raw.trim();
            let normalized = crate::core::color::normalize_color_to_oklch(trimmed)
                .unwrap_or_else(|| trimmed.to_string());

            let mut out = String::with_capacity(normalized.len() + 8);
            let mut prev: Option<char> = None;
            let mut iter = normalized.chars().peekable();
            while let Some(ch) = iter.next() {
                if ch == '.' {
                    let prev_is_digit = prev.is_some_and(|p| p.is_ascii_digit());
                    let next_is_digit = iter.peek().is_some_and(|n| n.is_ascii_digit());
                    if !prev_is_digit && next_is_digit {
                        out.push('0');
                    }
                }
                out.push(ch);
                prev = Some(ch);
            }
            out
        };

        if let (Some(light_theme), Some(dark_theme)) = (
            self.theme_by_name("dx.light"),
            self.theme_by_name("dx.dark"),
        ) {
            let mut root = String::from(":root {\n");
            let mut dark = String::from(".dark {\n");

            for (name, value) in &light_theme.tokens {
                let _ = writeln!(root, "  --{}: {};", name, value);
            }

            for (name, value) in &dark_theme.tokens {
                let _ = writeln!(dark, "  --{}: {};", name, value);
            }

            root.push_str("}\n");
            dark.push_str("}\n");
            return (root, dark);
        }

        let mut root = String::from(":root {\n");
        let mut dark = String::from(".dark {\n");

        for (name, value) in DX_FONT_TOKENS {
            let normalized = format_token_value(value);
            let _ = writeln!(root, "  --{}: {};", name, normalized);
        }
        for (name, value) in DX_BASE_TOKENS {
            let normalized = format_token_value(value);
            let _ = writeln!(root, "  --{}: {};", name, normalized);
        }
        let theme = ThemeBuilder::with_source(Argb::from_u32(DEFAULT_THEME_SOURCE)).build();
        let light = &theme.schemes.light;
        let dark_scheme = &theme.schemes.dark;

        let write_argb_token = |buffer: &mut String, name: &str, color: Argb| {
            let color_str = format_argb_as_oklch(color);
            let normalized = format_token_value(&color_str);
            let _ = writeln!(buffer, "  --{}: {};", name, normalized);
        };

        let write_raw_token = |buffer: &mut String, name: &str, value: &str| {
            let normalized = format_token_value(value);
            let _ = writeln!(buffer, "  --{}: {};", name, normalized);
        };

        // Surface & content tokens
        write_argb_token(&mut root, "background", light.background);
        write_argb_token(&mut dark, "background", dark_scheme.background);
        write_argb_token(&mut root, "foreground", light.on_background);
        write_argb_token(&mut dark, "foreground", dark_scheme.on_background);
        write_argb_token(&mut root, "card", light.surface);
        write_argb_token(&mut dark, "card", dark_scheme.surface);
        write_argb_token(&mut root, "card-foreground", light.on_surface);
        write_argb_token(&mut dark, "card-foreground", dark_scheme.on_surface);
        write_argb_token(&mut root, "popover", light.surface_bright);
        write_argb_token(&mut dark, "popover", dark_scheme.surface_dim);
        write_argb_token(&mut root, "popover-foreground", light.on_surface);
        write_argb_token(&mut dark, "popover-foreground", dark_scheme.on_surface);

        // Brand tokens
        write_argb_token(&mut root, "primary", light.primary);
        write_argb_token(&mut dark, "primary", dark_scheme.primary);
        write_argb_token(&mut root, "primary-foreground", light.on_primary);
        write_argb_token(&mut dark, "primary-foreground", dark_scheme.on_primary);
        write_argb_token(&mut root, "secondary", light.secondary);
        write_argb_token(&mut dark, "secondary", dark_scheme.secondary);
        write_argb_token(&mut root, "secondary-foreground", light.on_secondary);
        write_argb_token(&mut dark, "secondary-foreground", dark_scheme.on_secondary);
        write_argb_token(&mut root, "muted", light.surface_variant);
        write_argb_token(&mut dark, "muted", dark_scheme.surface_variant);
        write_argb_token(&mut root, "muted-foreground", light.on_surface_variant);
        write_argb_token(
            &mut dark,
            "muted-foreground",
            dark_scheme.on_surface_variant,
        );
        write_argb_token(&mut root, "accent", light.tertiary);
        write_argb_token(&mut dark, "accent", dark_scheme.tertiary);
        write_argb_token(&mut root, "accent-foreground", light.on_tertiary);
        write_argb_token(&mut dark, "accent-foreground", dark_scheme.on_tertiary);
        write_argb_token(&mut root, "destructive", light.error);
        write_argb_token(&mut dark, "destructive", dark_scheme.error);
        write_argb_token(&mut root, "destructive-foreground", light.on_error);
        write_argb_token(&mut dark, "destructive-foreground", dark_scheme.on_error);

        // Interaction tokens
        write_argb_token(&mut root, "border", light.outline);
        write_argb_token(&mut dark, "border", dark_scheme.outline);
        write_argb_token(&mut root, "input", light.surface_container_high);
        write_argb_token(&mut dark, "input", dark_scheme.surface_container_high);
        write_argb_token(&mut root, "ring", light.surface_tint);
        write_argb_token(&mut dark, "ring", dark_scheme.surface_tint);

        // Chart palette (shared across themes)
        let chart_1 = theme.palettes.primary.tone(60);
        let chart_2 = theme.palettes.secondary.tone(60);
        let chart_3 = theme.palettes.tertiary.tone(60);
        let chart_4 = theme.palettes.primary.tone(80);
        let chart_5 = theme.palettes.secondary.tone(80);
        for (name, color) in [
            ("chart-1", chart_1),
            ("chart-2", chart_2),
            ("chart-3", chart_3),
            ("chart-4", chart_4),
            ("chart-5", chart_5),
        ] {
            write_argb_token(&mut root, name, color);
            write_argb_token(&mut dark, name, color);
        }

        // Sidebar tokens
        write_argb_token(&mut root, "sidebar", light.surface_container_low);
        write_argb_token(&mut dark, "sidebar", dark_scheme.surface_container_low);
        write_argb_token(&mut root, "sidebar-foreground", light.on_surface);
        write_argb_token(&mut dark, "sidebar-foreground", dark_scheme.on_surface);
        write_argb_token(&mut root, "sidebar-primary", light.primary);
        write_argb_token(&mut dark, "sidebar-primary", dark_scheme.primary);
        write_argb_token(&mut root, "sidebar-primary-foreground", light.on_primary);
        write_argb_token(
            &mut dark,
            "sidebar-primary-foreground",
            dark_scheme.on_primary,
        );
        write_argb_token(&mut root, "sidebar-accent", light.secondary_container);
        write_argb_token(&mut dark, "sidebar-accent", dark_scheme.secondary_container);
        write_argb_token(
            &mut root,
            "sidebar-accent-foreground",
            light.on_secondary_container,
        );
        write_argb_token(
            &mut dark,
            "sidebar-accent-foreground",
            dark_scheme.on_secondary_container,
        );
        write_argb_token(&mut root, "sidebar-border", light.outline_variant);
        write_argb_token(&mut dark, "sidebar-border", dark_scheme.outline_variant);
        write_argb_token(&mut root, "sidebar-ring", light.surface_tint);
        write_argb_token(&mut dark, "sidebar-ring", dark_scheme.surface_tint);

        // Shadows
        write_argb_token(&mut root, "shadow-color", light.shadow);
        write_argb_token(&mut dark, "shadow-color", dark_scheme.shadow);
        for target in [&mut root, &mut dark] {
            write_raw_token(target, "shadow-opacity", "0.18");
            write_raw_token(target, "shadow-blur", "2px");
            write_raw_token(target, "shadow-spread", "0px");
            write_raw_token(target, "shadow-offset-x", "0px");
            write_raw_token(target, "shadow-offset-y", "1px");
        }

        for (name, value) in &token_entries {
            let normalized = format_token_value(value);
            let _ = writeln!(root, "  --color-{}: {};", name, normalized);
            let _ = writeln!(dark, "  --color-{}: {};", name, normalized);
        }

        root.push_str("}\n");
        dark.push_str("}\n");
        (root, dark)
    }

    pub(super) fn decode_encoded_css(
        &self,
        css: &str,
        selector: &str,
        wrappers: &[String],
    ) -> String {
        use crate::core::engine::build_block;
        let is_encoded = [
            "BASE|",
            "STATE|",
            "CHILD|",
            "COND|",
            "DATA|",
            "RAW|",
            "NEST|",
            "NEST_SUPPORTS|",
            "CHILD_SUPPORTS|",
            "INLINE_SUPPORTS|",
            "SUPPORTS|",
            "ANIM|",
        ]
        .iter()
        .any(|p| css.contains(p));
        if !is_encoded {
            if wrappers.is_empty() {
                return build_block(selector, css);
            }
            let sel = compose_selector_wrappers(selector, wrappers);
            return build_block(&sel, css);
        }
        let mut out = String::new();
        let mut pending_anim: Option<crate::core::animation::PendingAnimation> = None;
        let lines: Vec<&str> = if css.contains('\n') {
            css.lines().collect()
        } else {
            vec![css]
        };
        for line in lines {
            if line.is_empty() {
                continue;
            }
            if let Some(rest) = line.strip_prefix("BASE|") {
                if wrappers.is_empty() {
                    out.push_str(&build_block(selector, rest));
                } else {
                    let sel = compose_selector_wrappers(selector, wrappers);
                    out.push_str(&build_block(&sel, rest));
                }
                out.push('\n');
            } else if let Some(rest) = line.strip_prefix("STATE|") {
                let mut parts = rest.splitn(2, '|');
                let state = parts.next().unwrap_or("");
                let decls = parts.next().unwrap_or("");
                if state == "dark" {
                    out.push_str(&build_block(&format!(".dark {}", selector), decls));
                } else if state == "light" {
                    out.push_str(&build_block(&format!(":root {}", selector), decls));
                    out.push('\n');
                    out.push_str(&build_block(&format!(".light {}", selector), decls));
                } else {
                    out.push_str(&build_block(&format!("{}:{}", selector, state), decls));
                }
                out.push('\n');
            } else if let Some(rest) = line.strip_prefix("CHILD|") {
                let mut parts = rest.splitn(2, '|');
                let child = parts.next().unwrap_or("");
                let decls = parts.next().unwrap_or("");
                out.push_str(&build_block(&format!("{} > {}", selector, child), decls));
                out.push('\n');
            } else if let Some(rest) = line.strip_prefix("DATA|") {
                let mut parts = rest.splitn(2, '|');
                let data = parts.next().unwrap_or("");
                let decls = parts.next().unwrap_or("");
                out.push_str(&build_block(&format!("{}[data-{}]", selector, data), decls));
                out.push('\n');
            } else if let Some(rest) = line.strip_prefix("NEST|") {
                let mut parts = rest.splitn(2, '|');
                let suffix = parts.next().unwrap_or("");
                let decls = parts.next().unwrap_or("");
                if wrappers.is_empty() {
                    out.push_str(&build_block(&format!("{selector}{suffix}"), decls));
                } else {
                    let sel = compose_selector_wrappers(selector, wrappers);
                    out.push_str(&build_block(&format!("{sel}{suffix}"), decls));
                }
                out.push('\n');
            } else if let Some(rest) = line.strip_prefix("NEST_SUPPORTS|") {
                let mut parts = rest.splitn(3, '|');
                let suffix = parts.next().unwrap_or("");
                let condition = parts.next().unwrap_or("");
                let decls = parts.next().unwrap_or("");
                out.push_str(&format!("@supports {condition} {{\n"));
                let sel = if wrappers.is_empty() {
                    selector.to_string()
                } else {
                    compose_selector_wrappers(selector, wrappers)
                };
                for l in build_block(&format!("{sel}{suffix}"), decls).lines() {
                    out.push_str("  ");
                    out.push_str(l);
                    out.push('\n');
                }
                out.push_str("}\n");
            } else if let Some(rest) = line.strip_prefix("COND|") {
                let mut parts = rest.splitn(2, '|');
                let cond = parts.next().unwrap_or("");
                let decls = parts.next().unwrap_or("");
                if let Some(val) = cond.strip_prefix("@container>") {
                    out.push_str(&format!("@container (min-width: {}) {{\n", val));
                    for l in build_block(selector, decls).lines() {
                        out.push_str("  ");
                        out.push_str(l);
                        out.push('\n');
                    }
                    out.push_str("}\n");
                } else if let Some(bp) = cond.strip_prefix("screen:") {
                    if let Some(v) = self.screens.get(bp) {
                        out.push_str(&format!("@media (min-width: {}) {{\n", v));
                        for l in build_block(selector, decls).lines() {
                            out.push_str("  ");
                            out.push_str(l);
                            out.push('\n');
                        }
                        out.push_str("}\n");
                    }
                }
            } else if let Some(rest) = line.strip_prefix("SUPPORTS|") {
                let mut parts = rest.splitn(2, '|');
                let condition = parts.next().unwrap_or("");
                let decls = parts.next().unwrap_or("");
                out.push_str(&format!("@supports {condition} {{\n"));
                let sel = if wrappers.is_empty() {
                    selector.to_string()
                } else {
                    compose_selector_wrappers(selector, wrappers)
                };
                for l in build_block(&sel, decls).lines() {
                    out.push_str("  ");
                    out.push_str(l);
                    out.push('\n');
                }
                out.push_str("}\n");
            } else if let Some(rest) = line.strip_prefix("INLINE_SUPPORTS|") {
                let mut parts = rest.splitn(3, '|');
                let condition = parts.next().unwrap_or("");
                let base_decls = parts.next().unwrap_or("");
                let support_decls = parts.next().unwrap_or("");
                let sel = if wrappers.is_empty() {
                    selector.to_string()
                } else {
                    compose_selector_wrappers(selector, wrappers)
                };
                out.push_str(&format!("{sel} {{\n"));
                push_declaration_lines(&mut out, base_decls, 2);
                out.push_str(&format!("  @supports {condition} {{\n"));
                push_declaration_lines(&mut out, support_decls, 4);
                out.push_str("  }\n");
                out.push_str("}\n");
            } else if let Some(rest) = line.strip_prefix("CHILD_SUPPORTS|") {
                let mut parts = rest.splitn(3, '|');
                let child = parts.next().unwrap_or("");
                let condition = parts.next().unwrap_or("");
                let decls = parts.next().unwrap_or("");
                out.push_str(&format!("@supports {condition} {{\n"));
                let sel = if wrappers.is_empty() {
                    selector.to_string()
                } else {
                    compose_selector_wrappers(selector, wrappers)
                };
                for l in build_block(&format!("{sel} > {child}"), decls).lines() {
                    out.push_str("  ");
                    out.push_str(l);
                    out.push('\n');
                }
                out.push_str("}\n");
            } else if line.starts_with("ANIM|") {
                crate::core::animation::process_anim_line(line, &mut pending_anim);
            } else if let Some(raw) = line.strip_prefix("RAW|") {
                out.push_str(raw);
                if !raw.ends_with('\n') {
                    out.push('\n');
                }
            }
        }
        crate::core::animation::decode_animation_if_pending(
            self,
            selector,
            &mut pending_anim,
            &mut out,
        );
        if out.ends_with('\n') {
            out.pop();
        }
        out
    }
}
