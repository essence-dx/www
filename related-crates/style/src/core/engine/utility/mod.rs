//! Tailwind-compatible utility compiler.
//!
//! This module gives dx-style a useful default utility surface even when no
//! `.dx/style/style.dxm` file is present. Project config and precompiled styles
//! still win first; this layer is the portable fallback that makes raw source
//! files immediately productive.

mod color_palette;
mod mask;
mod theme_tokens;

const TRANSFORM_VALUE: &str = "var(--tw-rotate-x,) var(--tw-rotate-y,) var(--tw-rotate-z,) var(--tw-skew-x,) var(--tw-skew-y,)";
const TRANSFORM_GPU_VALUE: &str = "translateZ(0) var(--tw-rotate-x,) var(--tw-rotate-y,) var(--tw-rotate-z,) var(--tw-skew-x,) var(--tw-skew-y,)";
const FILTER_VALUE: &str = "var(--tw-blur, ) var(--tw-brightness, ) var(--tw-contrast, ) var(--tw-grayscale, ) var(--tw-hue-rotate, ) var(--tw-invert, ) var(--tw-saturate, ) var(--tw-sepia, ) var(--tw-drop-shadow, )";
const BACKDROP_FILTER_VALUE: &str = "var(--tw-backdrop-blur, ) var(--tw-backdrop-brightness, ) var(--tw-backdrop-contrast, ) var(--tw-backdrop-grayscale, ) var(--tw-backdrop-hue-rotate, ) var(--tw-backdrop-invert, ) var(--tw-backdrop-opacity, ) var(--tw-backdrop-saturate, ) var(--tw-backdrop-sepia, )";
const SHADOW_STACK_VALUE: &str = "var(--tw-inset-shadow), var(--tw-inset-ring-shadow), var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow)";
const COLOR_MIX_SUPPORTS_CONDITION: &str = "(color: color-mix(in lab, red, red))";

#[derive(Clone, Copy)]
enum SizeAxis {
    Width,
    Height,
}

struct FontSizeValue {
    font_size: String,
    default_line_height: Option<String>,
}

#[allow(dead_code)]
struct TextShadowAlpha {
    declaration_value: String,
    fallback_value: String,
}

pub fn generate_utility_css(class_name: &str) -> Option<String> {
    static_utility(class_name)
        .map(str::to_string)
        .or_else(|| object_utility(class_name))
        .or_else(|| container_utility(class_name))
        .or_else(|| arbitrary_property(class_name))
        .or_else(|| columns_utility(class_name))
        .or_else(|| break_utility(class_name))
        .or_else(|| aspect_ratio_utility(class_name))
        .or_else(|| flex_basis_utility(class_name))
        .or_else(|| spacing_utility(class_name))
        .or_else(|| sizing_utility(class_name))
        .or_else(|| grid_utility(class_name))
        .or_else(|| grid_auto_utility(class_name))
        .or_else(|| typography_utility(class_name))
        .or_else(|| typography_detail_utility(class_name))
        .or_else(|| tab_size_utility(class_name))
        .or_else(|| numeric_font_variant_utility(class_name))
        .or_else(|| font_feature_settings_utility(class_name))
        .or_else(|| content_utility(class_name))
        .or_else(|| line_clamp_utility(class_name))
        .or_else(|| list_utility(class_name))
        .or_else(|| table_utility(class_name))
        .or_else(|| border_utility(class_name))
        .or_else(|| divide_utility(class_name))
        .or_else(|| radius_utility(class_name))
        .or_else(|| background_utility(class_name))
        .or_else(|| color_utility(class_name))
        .or_else(|| blend_utility(class_name))
        .or_else(|| effect_utility(class_name))
        .or_else(|| mask::mask_utility(class_name))
        .or_else(|| scroll_utility(class_name))
        .or_else(|| scrollbar_utility(class_name))
        .or_else(|| transition_utility(class_name))
        .or_else(|| zoom_utility(class_name))
        .or_else(|| transform_utility(class_name))
}

fn static_utility(class_name: &str) -> Option<&'static str> {
    match class_name {
        "container" => Some("width: 100%; margin-left: auto; margin-right: auto"),
        "block" => Some("display: block"),
        "inline-block" => Some("display: inline-block"),
        "inline" => Some("display: inline"),
        "table" => Some("display: table"),
        "inline-table" => Some("display: inline-table"),
        "table-caption" => Some("display: table-caption"),
        "table-cell" => Some("display: table-cell"),
        "table-column" => Some("display: table-column"),
        "table-column-group" => Some("display: table-column-group"),
        "table-footer-group" => Some("display: table-footer-group"),
        "table-header-group" => Some("display: table-header-group"),
        "table-row" => Some("display: table-row"),
        "table-row-group" => Some("display: table-row-group"),
        "table-auto" => Some("table-layout: auto"),
        "table-fixed" => Some("table-layout: fixed"),
        "caption-top" => Some("caption-side: top"),
        "caption-bottom" => Some("caption-side: bottom"),
        "border-collapse" => Some("border-collapse: collapse"),
        "border-separate" => Some("border-collapse: separate"),
        "flex" => Some("display: flex"),
        "inline-flex" => Some("display: inline-flex"),
        "grid" => Some("display: grid"),
        "inline-grid" => Some("display: inline-grid"),
        "contents" => Some("display: contents"),
        "flow-root" => Some("display: flow-root"),
        "hidden" => Some("display: none"),
        "static" => Some("position: static"),
        "fixed" => Some("position: fixed"),
        "absolute" => Some("position: absolute"),
        "relative" => Some("position: relative"),
        "sticky" => Some("position: sticky"),
        "visible" => Some("visibility: visible"),
        "invisible" => Some("visibility: hidden"),
        "collapse" => Some("visibility: collapse"),
        "box-border" => Some("box-sizing: border-box"),
        "box-content" => Some("box-sizing: content-box"),
        "isolate" => Some("isolation: isolate"),
        "isolation-auto" => Some("isolation: auto"),
        "float-start" => Some("float: inline-start"),
        "float-end" => Some("float: inline-end"),
        "float-right" => Some("float: right"),
        "float-left" => Some("float: left"),
        "float-none" => Some("float: none"),
        "clear-start" => Some("clear: inline-start"),
        "clear-end" => Some("clear: inline-end"),
        "clear-left" => Some("clear: left"),
        "clear-right" => Some("clear: right"),
        "clear-both" => Some("clear: both"),
        "clear-none" => Some("clear: none"),
        "overflow-auto" => Some("overflow: auto"),
        "overflow-hidden" => Some("overflow: hidden"),
        "overflow-clip" => Some("overflow: clip"),
        "overflow-visible" => Some("overflow: visible"),
        "overflow-scroll" => Some("overflow: scroll"),
        "overflow-x-auto" => Some("overflow-x: auto"),
        "overflow-x-hidden" => Some("overflow-x: hidden"),
        "overflow-x-clip" => Some("overflow-x: clip"),
        "overflow-x-visible" => Some("overflow-x: visible"),
        "overflow-x-scroll" => Some("overflow-x: scroll"),
        "overflow-y-auto" => Some("overflow-y: auto"),
        "overflow-y-hidden" => Some("overflow-y: hidden"),
        "overflow-y-clip" => Some("overflow-y: clip"),
        "overflow-y-visible" => Some("overflow-y: visible"),
        "overflow-y-scroll" => Some("overflow-y: scroll"),
        "flex-row" => Some("flex-direction: row"),
        "flex-row-reverse" => Some("flex-direction: row-reverse"),
        "flex-col" => Some("flex-direction: column"),
        "flex-col-reverse" => Some("flex-direction: column-reverse"),
        "flex-wrap" => Some("flex-wrap: wrap"),
        "flex-wrap-reverse" => Some("flex-wrap: wrap-reverse"),
        "flex-nowrap" => Some("flex-wrap: nowrap"),
        "flex-1" => Some("flex: 1"),
        "flex-auto" => Some("flex: auto"),
        "flex-initial" => Some("flex: 0 auto"),
        "flex-none" => Some("flex: none"),
        "grow" => Some("flex-grow: 1"),
        "grow-0" => Some("flex-grow: 0"),
        "shrink" => Some("flex-shrink: 1"),
        "shrink-0" => Some("flex-shrink: 0"),
        "items-start" => Some("align-items: flex-start"),
        "items-end" => Some("align-items: flex-end"),
        "items-center" => Some("align-items: center"),
        "items-baseline" => Some("align-items: baseline"),
        "items-baseline-last" => Some("align-items: last baseline"),
        "items-stretch" => Some("align-items: stretch"),
        "items-normal" => Some("align-items: normal"),
        "justify-normal" => Some("justify-content: normal"),
        "justify-start" => Some("justify-content: flex-start"),
        "justify-end" => Some("justify-content: flex-end"),
        "justify-center" => Some("justify-content: center"),
        "justify-between" => Some("justify-content: space-between"),
        "justify-around" => Some("justify-content: space-around"),
        "justify-evenly" => Some("justify-content: space-evenly"),
        "justify-stretch" => Some("justify-content: stretch"),
        "justify-items-normal" => Some("justify-items: normal"),
        "justify-items-start" => Some("justify-items: start"),
        "justify-items-end" => Some("justify-items: end"),
        "justify-items-center" => Some("justify-items: center"),
        "justify-items-stretch" => Some("justify-items: stretch"),
        "justify-self-auto" => Some("justify-self: auto"),
        "justify-self-start" => Some("justify-self: start"),
        "justify-self-end" => Some("justify-self: end"),
        "justify-self-center" => Some("justify-self: center"),
        "justify-self-stretch" => Some("justify-self: stretch"),
        "content-normal" => Some("align-content: normal"),
        "content-center" => Some("align-content: center"),
        "content-start" => Some("align-content: flex-start"),
        "content-end" => Some("align-content: flex-end"),
        "content-between" => Some("align-content: space-between"),
        "content-around" => Some("align-content: space-around"),
        "content-evenly" => Some("align-content: space-evenly"),
        "content-baseline" => Some("align-content: baseline"),
        "self-auto" => Some("align-self: auto"),
        "self-start" => Some("align-self: flex-start"),
        "self-end" => Some("align-self: flex-end"),
        "self-center" => Some("align-self: center"),
        "self-baseline" => Some("align-self: baseline"),
        "self-baseline-last" => Some("align-self: last baseline"),
        "self-stretch" => Some("align-self: stretch"),
        "self-normal" => Some("align-self: normal"),
        "place-items-start" => Some("place-items: start"),
        "place-items-end" => Some("place-items: end"),
        "place-items-center" => Some("place-items: center"),
        "place-items-baseline" => Some("place-items: baseline"),
        "place-items-stretch" => Some("place-items: stretch"),
        "place-content-center" => Some("place-content: center"),
        "place-content-start" => Some("place-content: start"),
        "place-content-end" => Some("place-content: end"),
        "place-content-between" => Some("place-content: space-between"),
        "place-content-around" => Some("place-content: space-around"),
        "place-content-evenly" => Some("place-content: space-evenly"),
        "place-content-stretch" => Some("place-content: stretch"),
        "place-self-auto" => Some("place-self: auto"),
        "place-self-start" => Some("place-self: start"),
        "place-self-end" => Some("place-self: end"),
        "place-self-center" => Some("place-self: center"),
        "place-self-stretch" => Some("place-self: stretch"),
        "object-contain" => Some("object-fit: contain"),
        "object-cover" => Some("object-fit: cover"),
        "object-fill" => Some("object-fit: fill"),
        "object-none" => Some("object-fit: none"),
        "object-scale-down" => Some("object-fit: scale-down"),
        "object-bottom" => Some("object-position: bottom"),
        "object-center" => Some("object-position: center"),
        "object-left" => Some("object-position: left"),
        "object-left-bottom" => Some("object-position: left bottom"),
        "object-left-top" => Some("object-position: left top"),
        "object-right" => Some("object-position: right"),
        "object-right-bottom" => Some("object-position: right bottom"),
        "object-right-top" => Some("object-position: right top"),
        "object-top" => Some("object-position: top"),
        "pointer-events-none" => Some("pointer-events: none"),
        "pointer-events-auto" => Some("pointer-events: auto"),
        "select-none" => Some("-webkit-user-select: none; user-select: none"),
        "select-text" => Some("-webkit-user-select: text; user-select: text"),
        "select-all" => Some("-webkit-user-select: all; user-select: all"),
        "select-auto" => Some("-webkit-user-select: auto; user-select: auto"),
        "resize-none" => Some("resize: none"),
        "resize-y" => Some("resize: vertical"),
        "resize-x" => Some("resize: horizontal"),
        "resize" => Some("resize: both"),
        "field-sizing-content" => Some("field-sizing: content"),
        "field-sizing-fixed" => Some("field-sizing: fixed"),
        "box-decoration-clone" => {
            Some("-webkit-box-decoration-break: clone; box-decoration-break: clone")
        }
        "box-decoration-slice" => {
            Some("-webkit-box-decoration-break: slice; box-decoration-break: slice")
        }
        "forced-color-adjust-auto" => Some("forced-color-adjust: auto"),
        "forced-color-adjust-none" => Some("forced-color-adjust: none"),
        "scheme-normal" => Some("color-scheme: normal"),
        "scheme-light" => Some("color-scheme: light"),
        "scheme-dark" => Some("color-scheme: dark"),
        "scheme-light-dark" => Some("color-scheme: light dark"),
        "scheme-only-light" => Some("color-scheme: only light"),
        "scheme-only-dark" => Some("color-scheme: only dark"),
        "antialiased" => {
            Some("-webkit-font-smoothing: antialiased; -moz-osx-font-smoothing: grayscale")
        }
        "subpixel-antialiased" => {
            Some("-webkit-font-smoothing: auto; -moz-osx-font-smoothing: auto")
        }
        "truncate" => Some("overflow: hidden; text-overflow: ellipsis; white-space: nowrap"),
        "sr-only" => Some(
            "position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border-width: 0",
        ),
        "not-sr-only" => Some(
            "position: static; width: auto; height: auto; padding: 0; margin: 0; overflow: visible; clip: auto; white-space: normal",
        ),
        "appearance-none" => Some("-webkit-appearance: none; appearance: none"),
        "cursor-auto" => Some("cursor: auto"),
        "cursor-default" => Some("cursor: default"),
        "cursor-pointer" => Some("cursor: pointer"),
        "cursor-wait" => Some("cursor: wait"),
        "cursor-text" => Some("cursor: text"),
        "cursor-move" => Some("cursor: move"),
        "cursor-help" => Some("cursor: help"),
        "cursor-not-allowed" => Some("cursor: not-allowed"),
        "cursor-grab" => Some("cursor: grab"),
        "cursor-grabbing" => Some("cursor: grabbing"),
        "touch-auto" => Some("touch-action: auto"),
        "touch-none" => Some("touch-action: none"),
        "touch-pan-x" => Some(
            "--tw-pan-x: pan-x; touch-action: var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,)",
        ),
        "touch-pan-y" => Some(
            "--tw-pan-y: pan-y; touch-action: var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,)",
        ),
        "touch-pan-left" => Some(
            "--tw-pan-x: pan-left; touch-action: var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,)",
        ),
        "touch-pan-right" => Some(
            "--tw-pan-x: pan-right; touch-action: var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,)",
        ),
        "touch-pan-up" => Some(
            "--tw-pan-y: pan-up; touch-action: var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,)",
        ),
        "touch-pan-down" => Some(
            "--tw-pan-y: pan-down; touch-action: var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,)",
        ),
        "touch-pinch-zoom" => Some(
            "--tw-pinch-zoom: pinch-zoom; touch-action: var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,)",
        ),
        "touch-manipulation" => Some("touch-action: manipulation"),
        "transform-3d" => Some("transform-style: preserve-3d"),
        "transform-flat" => Some("transform-style: flat"),
        "will-change-auto" => Some("will-change: auto"),
        "will-change-scroll" => Some("will-change: scroll-position"),
        "will-change-contents" => Some("will-change: contents"),
        "will-change-transform" => Some("will-change: transform"),
        "backface-hidden" => {
            Some("-webkit-backface-visibility: hidden; backface-visibility: hidden")
        }
        "backface-visible" => {
            Some("-webkit-backface-visibility: visible; backface-visibility: visible")
        }
        _ => None,
    }
}

fn container_utility(class_name: &str) -> Option<String> {
    let raw = class_name.strip_prefix("@container")?;
    let (container_type, name) = match raw {
        "" => ("inline-size", None),
        "-normal" => ("normal", None),
        "-size" => ("size", None),
        _ => {
            let (kind, name) = raw.split_once('/')?;
            let container_type = match kind {
                "" => "inline-size",
                "-normal" => "normal",
                "-size" => "size",
                _ => return None,
            };
            if !is_safe_container_name(name) {
                return None;
            }
            (container_type, Some(name))
        }
    };

    let mut css = format!("container-type: {container_type}");
    if let Some(name) = name {
        css.push_str("; container-name: ");
        css.push_str(name);
    }
    Some(css)
}

fn is_safe_container_name(name: &str) -> bool {
    const RESERVED: &[&str] = &[
        "none",
        "initial",
        "inherit",
        "unset",
        "revert",
        "revert-layer",
    ];

    if name.is_empty() || name.len() > 64 || RESERVED.contains(&name) {
        return false;
    }

    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
}

fn object_utility(class_name: &str) -> Option<String> {
    let raw_value = class_name.strip_prefix("object-")?;
    let value = arbitrary_or_custom_property_value(raw_value)?;
    Some(format!("object-position: {}", value))
}

fn columns_utility(class_name: &str) -> Option<String> {
    let raw_value = class_name.strip_prefix("columns-")?;
    let value = columns_value(raw_value)?;
    Some(format!("columns: {}", value))
}

fn columns_value(raw_value: &str) -> Option<String> {
    if raw_value == "auto" {
        return Some("auto".to_string());
    }
    if let Some(value) = arbitrary_value(raw_value) {
        return Some(value);
    }
    if let Some(variable) = raw_value
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))
    {
        return is_custom_property_name(variable).then(|| format!("var({variable})"));
    }
    if let Some(value) = columns_width_value(raw_value) {
        return Some(value.to_string());
    }
    let count = bounded_usize(raw_value, 1, 12)?;
    Some(count.to_string())
}

fn columns_width_value(raw_value: &str) -> Option<&'static str> {
    match raw_value {
        "3xs" => Some("16rem"),
        "2xs" => Some("18rem"),
        "xs" => Some("20rem"),
        "sm" => Some("24rem"),
        "md" => Some("28rem"),
        "lg" => Some("32rem"),
        "xl" => Some("36rem"),
        "2xl" => Some("42rem"),
        "3xl" => Some("48rem"),
        "4xl" => Some("56rem"),
        "5xl" => Some("64rem"),
        "6xl" => Some("72rem"),
        "7xl" => Some("80rem"),
        _ => None,
    }
}

fn break_utility(class_name: &str) -> Option<String> {
    if let Some(raw_value) = class_name.strip_prefix("break-before-") {
        return break_before_after_value(raw_value).map(|value| format!("break-before: {value}"));
    }
    if let Some(raw_value) = class_name.strip_prefix("break-after-") {
        return break_before_after_value(raw_value).map(|value| format!("break-after: {value}"));
    }
    if let Some(raw_value) = class_name.strip_prefix("break-inside-") {
        return break_inside_value(raw_value).map(|value| {
            if raw_value == "avoid" {
                format!("page-break-inside: {value}; break-inside: {value}")
            } else {
                format!("break-inside: {value}")
            }
        });
    }
    None
}

fn break_before_after_value(raw_value: &str) -> Option<&'static str> {
    match raw_value {
        "auto" => Some("auto"),
        "avoid" => Some("avoid"),
        "all" => Some("all"),
        "avoid-page" => Some("avoid-page"),
        "page" => Some("page"),
        "left" => Some("left"),
        "right" => Some("right"),
        "column" => Some("column"),
        _ => None,
    }
}

fn break_inside_value(raw_value: &str) -> Option<&'static str> {
    match raw_value {
        "auto" => Some("auto"),
        "avoid" => Some("avoid"),
        "avoid-page" => Some("avoid-page"),
        "avoid-column" => Some("avoid-column"),
        _ => None,
    }
}

fn aspect_ratio_utility(class_name: &str) -> Option<String> {
    let raw_value = class_name.strip_prefix("aspect-")?;
    let value = match raw_value {
        "auto" => "auto".to_string(),
        "square" => "1 / 1".to_string(),
        "video" => "16 / 9".to_string(),
        _ => arbitrary_value(raw_value).or_else(|| custom_property_var_value(raw_value))?,
    };
    Some(format!("aspect-ratio: {}", value))
}

fn arbitrary_property(class_name: &str) -> Option<String> {
    let inner = class_name.strip_prefix('[')?.strip_suffix(']')?;
    let (property, value) = inner.split_once(':')?;
    if !property
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
    {
        return None;
    }
    Some(format!(
        "{}: {}",
        property,
        safe_arbitrary_css_value(value)?
    ))
}

fn spacing_utility(class_name: &str) -> Option<String> {
    let (class_name, negative) = class_name
        .strip_prefix('-')
        .map_or((class_name, false), |stripped| (stripped, true));

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
        let Some(raw_value) = class_name.strip_prefix(prefix) else {
            continue;
        };
        if negative && !supports_negative {
            return None;
        }
        let allow_auto = properties
            .iter()
            .all(|property| property.starts_with("margin"));
        let mut value = if properties
            .iter()
            .all(|property| is_inset_property(property))
        {
            inset_value(raw_value, true)?
        } else {
            spacing_value(raw_value, allow_auto)?
        };
        if negative && value != "auto" && value != "0px" {
            value = negate_css_value(value);
        }
        return Some(join_properties(properties, &value));
    }

    if class_name == "space-x-reverse" {
        return Some("CHILD|:where(:not(:last-child))|--tw-space-x-reverse: 1".to_string());
    }
    if class_name == "space-y-reverse" {
        return Some("CHILD|:where(:not(:last-child))|--tw-space-y-reverse: 1".to_string());
    }
    if let Some(raw_value) = class_name.strip_prefix("space-x-") {
        let mut value = spacing_value(raw_value, false)?;
        if negative && value != "0px" {
            value = negate_css_value(value);
        }
        return Some(space_axis_declarations(
            "x",
            "margin-inline-start",
            "margin-inline-end",
            &value,
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("space-y-") {
        let mut value = spacing_value(raw_value, false)?;
        if negative && value != "0px" {
            value = negate_css_value(value);
        }
        return Some(space_axis_declarations(
            "y",
            "margin-block-start",
            "margin-block-end",
            &value,
        ));
    }

    None
}

fn space_axis_declarations(
    axis: &str,
    start_property: &str,
    end_property: &str,
    value: &str,
) -> String {
    format!(
        "CHILD|:where(:not(:last-child))|--tw-space-{axis}-reverse: 0; {start_property}: calc({value} * var(--tw-space-{axis}-reverse)); {end_property}: calc({value} * calc(1 - var(--tw-space-{axis}-reverse)))"
    )
}

fn flex_basis_utility(class_name: &str) -> Option<String> {
    if let Some(raw_value) = class_name.strip_prefix("flex-") {
        let value = arbitrary_or_custom_property_value(raw_value)?;
        return Some(format!("flex: {}", value));
    }

    if let Some(raw_value) = class_name.strip_prefix("grow-") {
        let value = flex_factor_value(raw_value)?;
        return Some(format!("flex-grow: {}", value));
    }

    if let Some(raw_value) = class_name.strip_prefix("shrink-") {
        let value = flex_factor_value(raw_value)?;
        return Some(format!("flex-shrink: {}", value));
    }

    let raw_value = class_name.strip_prefix("basis-")?;
    let value = size_value(raw_value, SizeAxis::Width, true)?;
    Some(format!("flex-basis: {}", value))
}

fn flex_factor_value(raw_value: &str) -> Option<String> {
    arbitrary_value(raw_value)
        .or_else(|| custom_property_var_value(raw_value))
        .or_else(|| raw_value.parse::<f32>().ok().map(trim_float))
}

fn sizing_utility(class_name: &str) -> Option<String> {
    if let Some(raw_value) = class_name.strip_prefix("size-") {
        let value = size_value(raw_value, SizeAxis::Width, false)?;
        return Some(format!("width: {}; height: {}", value, value));
    }

    let sizing_rules: &[(&str, &str, SizeAxis, bool)] = &[
        ("min-inline-", "min-inline-size", SizeAxis::Width, true),
        ("max-inline-", "max-inline-size", SizeAxis::Width, true),
        ("inline-", "inline-size", SizeAxis::Width, true),
        ("min-block-", "min-block-size", SizeAxis::Height, false),
        ("max-block-", "max-block-size", SizeAxis::Height, false),
        ("block-", "block-size", SizeAxis::Height, false),
        ("min-w-", "min-width", SizeAxis::Width, true),
        ("max-w-", "max-width", SizeAxis::Width, true),
        ("w-", "width", SizeAxis::Width, true),
        ("min-h-", "min-height", SizeAxis::Height, false),
        ("max-h-", "max-height", SizeAxis::Height, false),
        ("h-", "height", SizeAxis::Height, false),
    ];
    for (prefix, property, axis, allow_container_scale) in sizing_rules {
        let Some(raw_value) = class_name.strip_prefix(prefix) else {
            continue;
        };
        let value = size_value(raw_value, *axis, *allow_container_scale)?;
        return Some(format!("{}: {}", property, value));
    }

    if let Some(raw_value) = class_name.strip_prefix("z-") {
        let value = if raw_value == "auto" {
            "auto".to_string()
        } else if let Some(value) = custom_property_var_value(raw_value) {
            value
        } else {
            raw_value.parse::<i32>().ok()?.to_string()
        };
        return Some(format!("z-index: {}", value));
    }

    if let Some(raw_value) = class_name.strip_prefix("-order-") {
        let value = raw_value.parse::<i32>().ok()?;
        return Some(format!("order: -{}", value.abs()));
    }

    if let Some(raw_value) = class_name.strip_prefix("order-") {
        let value = match raw_value {
            "first" => "-9999".to_string(),
            "last" => "9999".to_string(),
            "none" => "0".to_string(),
            _ => custom_property_var_value(raw_value)
                .or_else(|| raw_value.parse::<i32>().ok().map(|value| value.to_string()))?,
        };
        return Some(format!("order: {}", value));
    }

    None
}

fn grid_utility(class_name: &str) -> Option<String> {
    if let Some(raw_value) = class_name.strip_prefix("grid-cols-") {
        if raw_value == "none" {
            return Some("grid-template-columns: none".to_string());
        }
        if raw_value == "subgrid" {
            return Some("grid-template-columns: subgrid".to_string());
        }
        if let Some(value) =
            arbitrary_value(raw_value).or_else(|| custom_property_var_value(raw_value))
        {
            return Some(format!("grid-template-columns: {}", value));
        }
        let count = bounded_usize(raw_value, 1, 24)?;
        return Some(format!(
            "grid-template-columns: repeat({}, minmax(0, 1fr))",
            count
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("grid-rows-") {
        if raw_value == "none" {
            return Some("grid-template-rows: none".to_string());
        }
        if raw_value == "subgrid" {
            return Some("grid-template-rows: subgrid".to_string());
        }
        if let Some(value) =
            arbitrary_value(raw_value).or_else(|| custom_property_var_value(raw_value))
        {
            return Some(format!("grid-template-rows: {}", value));
        }
        let count = bounded_usize(raw_value, 1, 24)?;
        return Some(format!(
            "grid-template-rows: repeat({}, minmax(0, 1fr))",
            count
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("col-span-") {
        if raw_value == "full" {
            return Some("grid-column: 1 / -1".to_string());
        }
        let count = bounded_usize(raw_value, 1, 24)?;
        return Some(format!("grid-column: span {} / span {}", count, count));
    }
    if let Some(raw_value) = class_name.strip_prefix("row-span-") {
        if raw_value == "full" {
            return Some("grid-row: 1 / -1".to_string());
        }
        let count = bounded_usize(raw_value, 1, 24)?;
        return Some(format!("grid-row: span {} / span {}", count, count));
    }
    None
}

fn grid_auto_utility(class_name: &str) -> Option<String> {
    if let Some(css) = match class_name {
        "grid-flow-row" => Some("grid-auto-flow: row"),
        "grid-flow-col" => Some("grid-auto-flow: column"),
        "grid-flow-dense" => Some("grid-auto-flow: dense"),
        "grid-flow-row-dense" => Some("grid-auto-flow: row dense"),
        "grid-flow-col-dense" => Some("grid-auto-flow: column dense"),
        "col-auto" => Some("grid-column: auto"),
        "row-auto" => Some("grid-row: auto"),
        _ => None,
    } {
        return Some(css.to_string());
    }

    let auto_rules: &[(&str, &str)] = &[
        ("auto-cols-", "grid-auto-columns"),
        ("auto-rows-", "grid-auto-rows"),
    ];
    for (prefix, property) in auto_rules {
        let Some(raw_value) = class_name.strip_prefix(prefix) else {
            continue;
        };
        let value = match raw_value {
            "auto" => "auto".to_string(),
            "min" => "min-content".to_string(),
            "max" => "max-content".to_string(),
            "fr" => "minmax(0, 1fr)".to_string(),
            _ => arbitrary_value(raw_value).or_else(|| custom_property_var_value(raw_value))?,
        };
        return Some(format!("{}: {}", property, value));
    }

    let line_rules: &[(&str, &str)] = &[
        ("col-start-", "grid-column-start"),
        ("col-end-", "grid-column-end"),
        ("row-start-", "grid-row-start"),
        ("row-end-", "grid-row-end"),
    ];
    for (prefix, property) in line_rules {
        let Some(raw_value) = class_name.strip_prefix(prefix) else {
            continue;
        };
        let value = if raw_value == "auto" {
            "auto".to_string()
        } else if let Some(value) =
            arbitrary_value(raw_value).or_else(|| custom_property_var_value(raw_value))
        {
            value
        } else {
            raw_value.parse::<i32>().ok()?.to_string()
        };
        return Some(format!("{}: {}", property, value));
    }

    None
}

fn typography_utility(class_name: &str) -> Option<String> {
    if let Some(css) = match class_name {
        "text-left" => Some("text-align: left"),
        "text-center" => Some("text-align: center"),
        "text-right" => Some("text-align: right"),
        "text-justify" => Some("text-align: justify"),
        "text-start" => Some("text-align: start"),
        "text-end" => Some("text-align: end"),
        "font-thin" => Some("font-weight: 100"),
        "font-extralight" => Some("font-weight: 200"),
        "font-light" => Some("font-weight: 300"),
        "font-normal" => Some("font-weight: 400"),
        "font-medium" => Some("font-weight: 500"),
        "font-semibold" => Some("font-weight: 600"),
        "font-bold" => Some("font-weight: 700"),
        "font-extrabold" => Some("font-weight: 800"),
        "font-black" => Some("font-weight: 900"),
        "font-sans" => Some("font-family: var(--font-sans, Geist, sans-serif)"),
        "font-serif" => Some("font-family: var(--font-serif, Georgia, serif)"),
        "font-mono" => Some("font-family: var(--font-mono, 'Geist Mono', monospace)"),
        "italic" => Some("font-style: italic"),
        "not-italic" => Some("font-style: normal"),
        "uppercase" => Some("text-transform: uppercase"),
        "lowercase" => Some("text-transform: lowercase"),
        "capitalize" => Some("text-transform: capitalize"),
        "normal-case" => Some("text-transform: none"),
        "underline" => Some("text-decoration-line: underline"),
        "overline" => Some("text-decoration-line: overline"),
        "line-through" => Some("text-decoration-line: line-through"),
        "no-underline" => Some("text-decoration-line: none"),
        "whitespace-normal" => Some("white-space: normal"),
        "whitespace-nowrap" => Some("white-space: nowrap"),
        "whitespace-pre" => Some("white-space: pre"),
        "whitespace-pre-line" => Some("white-space: pre-line"),
        "whitespace-pre-wrap" => Some("white-space: pre-wrap"),
        "whitespace-break-spaces" => Some("white-space: break-spaces"),
        "break-normal" => Some("overflow-wrap: normal; word-break: normal"),
        "break-words" => Some("overflow-wrap: break-word"),
        "break-all" => Some("word-break: break-all"),
        "break-keep" => Some("word-break: keep-all"),
        "text-wrap" => Some("text-wrap: wrap"),
        "text-nowrap" => Some("text-wrap: nowrap"),
        "text-balance" => Some("text-wrap: balance"),
        "text-pretty" => Some("text-wrap: pretty"),
        "text-ellipsis" => Some("overflow: hidden; text-overflow: ellipsis; white-space: nowrap"),
        "text-clip" => Some("text-overflow: clip"),
        "hyphens-none" => Some("-webkit-hyphens: none; hyphens: none"),
        "hyphens-manual" => Some("-webkit-hyphens: manual; hyphens: manual"),
        "hyphens-auto" => Some("-webkit-hyphens: auto; hyphens: auto"),
        _ => None,
    } {
        return Some(css.to_string());
    }

    if let Some(raw_value) = class_name.strip_prefix("text-") {
        let (font_size_raw, line_height_raw) = split_text_size_line_height_modifier(raw_value);
        if let Some(value) = font_size_value(font_size_raw) {
            let line_height = match line_height_raw {
                Some(line_height_raw) => {
                    Some(text_size_line_height_modifier_value(line_height_raw)?)
                }
                None => value.default_line_height,
            };
            let mut css = format!("font-size: {}", value.font_size);
            if let Some(line_height) = line_height {
                css.push_str("; line-height: ");
                css.push_str(&line_height);
            }
            return Some(css);
        }
    }
    if let Some(raw_value) = class_name.strip_prefix("font-stretch-") {
        let value = font_stretch_value(raw_value)?;
        return Some(format!("font-stretch: {}", value));
    }
    if let Some(raw_value) = class_name.strip_prefix("leading-") {
        let value = match raw_value {
            "none" => "1".to_string(),
            "tight" => "1.25".to_string(),
            "snug" => "1.375".to_string(),
            "normal" => "1.5".to_string(),
            "relaxed" => "1.625".to_string(),
            "loose" => "2".to_string(),
            _ => arbitrary_value(raw_value)
                .or_else(|| custom_property_var_value(raw_value))
                .or_else(|| spacing_value(raw_value, false))?,
        };
        return Some(format!("line-height: {}", value));
    }
    if let Some(raw_value) = class_name.strip_prefix("tracking-") {
        let value = match raw_value {
            "tighter" => "-0.05em".to_string(),
            "tight" => "-0.025em".to_string(),
            "normal" => "0em".to_string(),
            "wide" => "0.025em".to_string(),
            "wider" => "0.05em".to_string(),
            "widest" => "0.1em".to_string(),
            _ => arbitrary_value(raw_value).or_else(|| custom_property_var_value(raw_value))?,
        };
        return Some(format!("letter-spacing: {}", value));
    }

    None
}

fn typography_detail_utility(class_name: &str) -> Option<String> {
    if let Some(css) = match class_name {
        "wrap-normal" => Some("overflow-wrap: normal"),
        "wrap-break-word" => Some("overflow-wrap: break-word"),
        "wrap-anywhere" => Some("overflow-wrap: anywhere"),
        "align-baseline" => Some("vertical-align: baseline"),
        "align-top" => Some("vertical-align: top"),
        "align-middle" => Some("vertical-align: middle"),
        "align-bottom" => Some("vertical-align: bottom"),
        "align-text-top" => Some("vertical-align: text-top"),
        "align-text-bottom" => Some("vertical-align: text-bottom"),
        "align-sub" => Some("vertical-align: sub"),
        "align-super" => Some("vertical-align: super"),
        _ => None,
    } {
        return Some(css.to_string());
    }

    let (class_name, negative) = class_name
        .strip_prefix('-')
        .map_or((class_name, false), |stripped| (stripped, true));

    if let Some(raw_value) = class_name.strip_prefix("indent-") {
        let mut value =
            spacing_value(raw_value, false).or_else(|| custom_property_var_value(raw_value))?;
        if negative {
            value = negate_css_value(value);
        }
        return Some(format!("text-indent: {}", value));
    }

    if let Some(raw_value) = class_name.strip_prefix("align-") {
        let value = arbitrary_value(raw_value).or_else(|| custom_property_var_value(raw_value))?;
        return Some(format!("vertical-align: {}", value));
    }

    if let Some(raw_value) = class_name.strip_prefix("decoration-") {
        let value = text_decoration_thickness_value(raw_value)?;
        return Some(format!("text-decoration-thickness: {}", value));
    }

    if let Some(raw_value) = class_name.strip_prefix("underline-offset-") {
        let mut value = underline_offset_value(raw_value)?;
        if negative {
            value = negate_css_value(value);
        }
        return Some(format!("text-underline-offset: {}", value));
    }

    None
}

fn text_decoration_thickness_value(raw_value: &str) -> Option<String> {
    match raw_value {
        "auto" => Some("auto".to_string()),
        "from-font" => Some("from-font".to_string()),
        _ => arbitrary_value(raw_value)
            .or_else(|| typed_custom_property_var_value(raw_value, "length"))
            .or_else(|| {
                raw_value
                    .parse::<f32>()
                    .ok()
                    .map(|value| format!("{}px", trim_float(value)))
            }),
    }
}

fn underline_offset_value(raw_value: &str) -> Option<String> {
    if raw_value == "auto" {
        return Some("auto".to_string());
    }
    arbitrary_value(raw_value)
        .or_else(|| custom_property_var_value(raw_value))
        .or_else(|| {
            raw_value
                .parse::<f32>()
                .ok()
                .map(|value| format!("{}px", trim_float(value)))
        })
}

fn negate_css_value(mut value: String) -> String {
    if value == "0" || value == "0px" {
        value
    } else if let Some(multiplier) = spacing_calc_multiplier(&value) {
        format!("calc(var(--spacing) * -{multiplier})")
    } else if value.starts_with("calc(") || value.starts_with("var(") {
        format!("calc({value} * -1)")
    } else {
        value.insert(0, '-');
        value
    }
}

fn font_stretch_value(raw_value: &str) -> Option<String> {
    if let Some(value) = font_stretch_keyword(raw_value) {
        return Some(value.to_string());
    }
    if let Some(value) = arbitrary_value(raw_value) {
        return Some(value);
    }
    if let Some(variable) = raw_value
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))
    {
        if is_custom_property_name(variable) {
            return Some(format!("var({variable})"));
        }
        return None;
    }
    percentage_value(raw_value)
}

fn font_stretch_keyword(raw_value: &str) -> Option<&'static str> {
    match raw_value {
        "ultra-condensed" => Some("ultra-condensed"),
        "extra-condensed" => Some("extra-condensed"),
        "condensed" => Some("condensed"),
        "semi-condensed" => Some("semi-condensed"),
        "normal" => Some("normal"),
        "semi-expanded" => Some("semi-expanded"),
        "expanded" => Some("expanded"),
        "extra-expanded" => Some("extra-expanded"),
        "ultra-expanded" => Some("ultra-expanded"),
        _ => None,
    }
}

fn percentage_value(raw_value: &str) -> Option<String> {
    let value = raw_value.strip_suffix('%')?;
    let numeric = value.parse::<f32>().ok()?;
    if !numeric.is_finite() || numeric <= 0.0 {
        return None;
    }
    Some(format!("{}%", trim_float(numeric)))
}

fn numeric_font_variant_utility(class_name: &str) -> Option<String> {
    const NUMERIC_VALUE: &str = "font-variant-numeric: var(--tw-ordinal,) var(--tw-slashed-zero,) var(--tw-numeric-figure,) var(--tw-numeric-spacing,) var(--tw-numeric-fraction,)";

    match class_name {
        "normal-nums" => Some("font-variant-numeric: normal".to_string()),
        "ordinal" => Some(format!("--tw-ordinal: ordinal; {NUMERIC_VALUE}")),
        "slashed-zero" => Some(format!("--tw-slashed-zero: slashed-zero; {NUMERIC_VALUE}")),
        "lining-nums" => Some(format!("--tw-numeric-figure: lining-nums; {NUMERIC_VALUE}")),
        "oldstyle-nums" => Some(format!(
            "--tw-numeric-figure: oldstyle-nums; {NUMERIC_VALUE}"
        )),
        "proportional-nums" => Some(format!(
            "--tw-numeric-spacing: proportional-nums; {NUMERIC_VALUE}"
        )),
        "tabular-nums" => Some(format!(
            "--tw-numeric-spacing: tabular-nums; {NUMERIC_VALUE}"
        )),
        "diagonal-fractions" => Some(format!(
            "--tw-numeric-fraction: diagonal-fractions; {NUMERIC_VALUE}"
        )),
        "stacked-fractions" => Some(format!(
            "--tw-numeric-fraction: stacked-fractions; {NUMERIC_VALUE}"
        )),
        _ => None,
    }
}

fn font_feature_settings_utility(class_name: &str) -> Option<String> {
    let raw_value = class_name.strip_prefix("font-features-")?;
    let value = arbitrary_or_custom_property_value(raw_value)?;
    Some(format!("font-feature-settings: {}", value))
}

fn line_clamp_utility(class_name: &str) -> Option<String> {
    let raw_value = class_name.strip_prefix("line-clamp-")?;
    if raw_value == "none" {
        return Some(
            "overflow: visible; display: block; -webkit-box-orient: horizontal; -webkit-line-clamp: unset"
                .to_string(),
        );
    }

    let value = if let Some(value) = arbitrary_value(raw_value) {
        value
    } else {
        bounded_usize(raw_value, 1, 12)?.to_string()
    };
    Some(format!(
        "overflow: hidden; display: -webkit-box; -webkit-box-orient: vertical; -webkit-line-clamp: {}",
        value
    ))
}

fn content_utility(class_name: &str) -> Option<String> {
    let raw_value = class_name.strip_prefix("content-")?;
    let value = content_value(raw_value)?;
    Some(format!("content: {}", value))
}

fn content_value(raw_value: &str) -> Option<String> {
    if raw_value == "none" {
        return Some("none".to_string());
    }

    if let Some(name) = raw_value
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))
    {
        if is_custom_property_name(name) {
            return Some(format!("var({name})"));
        }
        return None;
    }

    let inner = raw_value.strip_prefix('[')?.strip_suffix(']')?;
    if inner.is_empty()
        || inner
            .chars()
            .any(|ch| matches!(ch, '{' | '}' | ';' | '\n' | '\r'))
    {
        return None;
    }

    Some(decode_content_arbitrary_value(inner))
}

fn is_custom_property_name(value: &str) -> bool {
    value
        .strip_prefix("--")
        .is_some_and(|name| !name.is_empty() && name.chars().all(is_custom_property_char))
}

fn is_custom_property_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '-' || ch == '_'
}

fn decode_content_arbitrary_value(value: &str) -> String {
    let mut decoded = String::with_capacity(value.len());
    let mut chars = value.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' && matches!(chars.peek(), Some('_')) {
            chars.next();
            decoded.push('_');
        } else if ch == '_' {
            decoded.push(' ');
        } else {
            decoded.push(ch);
        }
    }

    decoded
}

fn list_utility(class_name: &str) -> Option<String> {
    if let Some(css) = match class_name {
        "list-none" => Some("list-style-type: none"),
        "list-disc" => Some("list-style-type: disc"),
        "list-decimal" => Some("list-style-type: decimal"),
        "list-inside" => Some("list-style-position: inside"),
        "list-outside" => Some("list-style-position: outside"),
        _ => None,
    } {
        return Some(css.to_string());
    }

    let raw_value = class_name.strip_prefix("list-")?;
    let value = arbitrary_value(raw_value)?;
    Some(format!("list-style-type: {}", value))
}

fn table_utility(class_name: &str) -> Option<String> {
    if let Some(raw_value) = class_name.strip_prefix("border-spacing-x-") {
        let value = spacing_value(raw_value, false)?;
        return Some(format!(
            "--tw-border-spacing-x: {}; border-spacing: var(--tw-border-spacing-x) var(--tw-border-spacing-y, 0)",
            value
        ));
    }

    if let Some(raw_value) = class_name.strip_prefix("border-spacing-y-") {
        let value = spacing_value(raw_value, false)?;
        return Some(format!(
            "--tw-border-spacing-y: {}; border-spacing: var(--tw-border-spacing-x, 0) var(--tw-border-spacing-y)",
            value
        ));
    }

    let raw_value = class_name.strip_prefix("border-spacing-")?;
    let value = spacing_value(raw_value, false)?;
    Some(format!("border-spacing: {}", value))
}

fn border_utility(class_name: &str) -> Option<String> {
    if class_name == "border" {
        return Some("border-style: var(--tw-border-style); border-width: 1px".to_string());
    }
    if let Some(css) = match class_name {
        "border-x" => Some("border-inline-style: var(--tw-border-style); border-inline-width: 1px"),
        "border-y" => Some("border-block-style: var(--tw-border-style); border-block-width: 1px"),
        "border-t" => Some("border-top-style: var(--tw-border-style); border-top-width: 1px"),
        "border-r" => Some("border-right-style: var(--tw-border-style); border-right-width: 1px"),
        "border-b" => Some("border-bottom-style: var(--tw-border-style); border-bottom-width: 1px"),
        "border-l" => Some("border-left-style: var(--tw-border-style); border-left-width: 1px"),
        "border-s" => Some(
            "border-inline-start-style: var(--tw-border-style); border-inline-start-width: 1px",
        ),
        "border-e" => {
            Some("border-inline-end-style: var(--tw-border-style); border-inline-end-width: 1px")
        }
        "border-bs" => {
            Some("border-block-start-style: var(--tw-border-style); border-block-start-width: 1px")
        }
        "border-be" => {
            Some("border-block-end-style: var(--tw-border-style); border-block-end-width: 1px")
        }
        "border-solid" => Some("border-style: solid"),
        "border-dashed" => Some("border-style: dashed"),
        "border-dotted" => Some("border-style: dotted"),
        "border-double" => Some("border-style: double"),
        "border-hidden" => Some("border-style: hidden"),
        "border-none" => Some("border-style: none"),
        "divide-x" => Some("CHILD|* + *|border-left-width: 1px"),
        "divide-y" => Some("CHILD|* + *|border-top-width: 1px"),
        _ => None,
    } {
        return Some(css.to_string());
    }
    let border_width_rules: &[(&str, &[&str], &[&str])] = &[
        (
            "border-x-",
            &["border-inline-style"],
            &["border-inline-width"],
        ),
        (
            "border-y-",
            &["border-block-style"],
            &["border-block-width"],
        ),
        ("border-t-", &["border-top-style"], &["border-top-width"]),
        (
            "border-r-",
            &["border-right-style"],
            &["border-right-width"],
        ),
        (
            "border-b-",
            &["border-bottom-style"],
            &["border-bottom-width"],
        ),
        ("border-l-", &["border-left-style"], &["border-left-width"]),
        (
            "border-s-",
            &["border-inline-start-style"],
            &["border-inline-start-width"],
        ),
        (
            "border-e-",
            &["border-inline-end-style"],
            &["border-inline-end-width"],
        ),
        (
            "border-bs-",
            &["border-block-start-style"],
            &["border-block-start-width"],
        ),
        (
            "border-be-",
            &["border-block-end-style"],
            &["border-block-end-width"],
        ),
        ("border-", &["border-style"], &["border-width"]),
    ];
    for (prefix, style_properties, width_properties) in border_width_rules {
        let Some(raw_value) = class_name.strip_prefix(prefix) else {
            continue;
        };
        if is_border_color_arbitrary_value(raw_value) {
            continue;
        }
        let value = border_size_value(raw_value)?;
        let style = join_properties(style_properties, "var(--tw-border-style)");
        let width = join_properties(width_properties, &value);
        return Some(format!("{style}; {width}"));
    }
    None
}

fn divide_utility(class_name: &str) -> Option<String> {
    if let Some(css) = match class_name {
        "divide-x" => Some("CHILD|* + *|border-left-width: 1px"),
        "divide-y" => Some("CHILD|* + *|border-top-width: 1px"),
        "divide-solid" => Some("CHILD|* + *|border-style: solid"),
        "divide-dashed" => Some("CHILD|* + *|border-style: dashed"),
        "divide-dotted" => Some("CHILD|* + *|border-style: dotted"),
        "divide-double" => Some("CHILD|* + *|border-style: double"),
        "divide-none" => Some("CHILD|* + *|border-style: none"),
        _ => None,
    } {
        return Some(css.to_string());
    }

    let divide_width_rules: &[(&str, &str)] = &[
        ("divide-x-", "border-left-width"),
        ("divide-y-", "border-top-width"),
    ];
    for (prefix, property) in divide_width_rules {
        let Some(raw_value) = class_name.strip_prefix(prefix) else {
            continue;
        };
        let value = border_size_value(raw_value)?;
        return Some(format!("CHILD|* + *|{}: {}", property, value));
    }

    let raw_value = class_name.strip_prefix("divide-")?;
    child_color_property_css("* + *", "border-color", raw_value)
}

fn radius_utility(class_name: &str) -> Option<String> {
    let radius_rules: &[(&str, &[&str])] = &[
        (
            "rounded-t-",
            &["border-top-left-radius", "border-top-right-radius"],
        ),
        (
            "rounded-r-",
            &["border-top-right-radius", "border-bottom-right-radius"],
        ),
        (
            "rounded-b-",
            &["border-bottom-right-radius", "border-bottom-left-radius"],
        ),
        (
            "rounded-l-",
            &["border-top-left-radius", "border-bottom-left-radius"],
        ),
        (
            "rounded-s-",
            &["border-start-start-radius", "border-end-start-radius"],
        ),
        (
            "rounded-e-",
            &["border-start-end-radius", "border-end-end-radius"],
        ),
        ("rounded-ss-", &["border-start-start-radius"]),
        ("rounded-se-", &["border-start-end-radius"]),
        ("rounded-ee-", &["border-end-end-radius"]),
        ("rounded-es-", &["border-end-start-radius"]),
        ("rounded-tl-", &["border-top-left-radius"]),
        ("rounded-tr-", &["border-top-right-radius"]),
        ("rounded-br-", &["border-bottom-right-radius"]),
        ("rounded-bl-", &["border-bottom-left-radius"]),
        ("rounded-", &["border-radius"]),
    ];
    if class_name == "rounded" {
        return Some("border-radius: 0.25rem".to_string());
    }
    for (prefix, properties) in radius_rules {
        let Some(raw_value) = class_name.strip_prefix(prefix) else {
            continue;
        };
        let value = radius_value(raw_value)?;
        return Some(join_properties(properties, &value));
    }
    None
}

fn color_utility(class_name: &str) -> Option<String> {
    if let Some(raw_value) = class_name.strip_prefix("drop-shadow-") {
        let css = color_hook_property_css(
            "--tw-drop-shadow-color",
            raw_value,
            Some("var(--tw-drop-shadow-alpha)"),
        )?;
        return Some(append_color_hook_base_declaration(
            css,
            "--tw-drop-shadow: var(--tw-drop-shadow-size)",
        ));
    }

    for (prefix, property, alpha_property) in [
        (
            "shadow-",
            "--tw-shadow-color",
            Some("var(--tw-shadow-alpha)"),
        ),
        (
            "inset-shadow-",
            "--tw-inset-shadow-color",
            Some("var(--tw-inset-shadow-alpha)"),
        ),
        ("inset-ring-", "--tw-inset-ring-color", None),
        ("ring-offset-", "--tw-ring-offset-color", None),
    ] {
        let Some(raw_value) = class_name.strip_prefix(prefix) else {
            continue;
        };
        if matches!(prefix, "ring-offset-") && is_length_like_color_ambiguous_value(raw_value) {
            continue;
        }
        return color_hook_property_css(property, raw_value, alpha_property);
    }

    if let Some(css) = directional_border_color_utility(class_name) {
        return Some(css);
    }

    if let Some(raw_value) = class_name.strip_prefix("placeholder-") {
        return nested_color_property_css("::placeholder", "color", raw_value);
    }

    let color_rules: &[(&str, &str)] = &[
        ("bg-", "background-color"),
        ("text-", "color"),
        ("border-", "border-color"),
        ("decoration-", "text-decoration-color"),
        ("accent-", "accent-color"),
        ("caret-", "caret-color"),
        ("fill-", "fill"),
        ("stroke-", "stroke"),
        ("outline-", "outline-color"),
        ("ring-", "--tw-ring-color"),
    ];
    for (prefix, property) in color_rules {
        let Some(raw_value) = class_name.strip_prefix(prefix) else {
            continue;
        };
        if matches!(*prefix, "outline-" | "ring-offset-" | "ring-")
            && is_length_like_color_ambiguous_value(raw_value)
        {
            continue;
        }
        return color_property_css(property, raw_value);
    }

    if let Some(raw_value) = class_name.strip_prefix("from-") {
        if let Some(value) = gradient_stop_position_value(raw_value) {
            return Some(format!("--tw-gradient-from-position: {}", value));
        }
        return gradient_color_stop_css(
            "--tw-gradient-from",
            raw_value,
            "--tw-gradient-to: rgb(255 255 255 / 0); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to)",
        );
    }
    if let Some(raw_value) = class_name.strip_prefix("via-") {
        if let Some(value) = gradient_stop_position_value(raw_value) {
            return Some(format!("--tw-gradient-via-position: {}", value));
        }
        return gradient_color_stop_css(
            "--tw-gradient-via",
            raw_value,
            "--tw-gradient-to: rgb(255 255 255 / 0); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-via), var(--tw-gradient-to)",
        );
    }
    if let Some(raw_value) = class_name.strip_prefix("to-") {
        if let Some(value) = gradient_stop_position_value(raw_value) {
            return Some(format!("--tw-gradient-to-position: {}", value));
        }
        return color_property_css("--tw-gradient-to", raw_value);
    }

    None
}

fn color_property_css(property: &str, raw_value: &str) -> Option<String> {
    if let Some(css) = tailwind_v43_palette_opacity_property_css(property, raw_value) {
        return Some(css);
    }

    let value = color_value(raw_value)?;
    Some(format!("{property}: {value}"))
}

pub(crate) fn tailwind_v43_palette_opacity_property_css(
    property: &str,
    raw_value: &str,
) -> Option<String> {
    let (fallback, supported) = tailwind_v43_palette_opacity_fallback_and_supported(raw_value)?;
    Some(format!(
        "BASE|{property}: {fallback}\nSUPPORTS|{COLOR_MIX_SUPPORTS_CONDITION}|{property}: {supported}"
    ))
}

fn nested_color_property_css(suffix: &str, property: &str, raw_value: &str) -> Option<String> {
    if let Some((fallback, supported)) =
        tailwind_v43_palette_opacity_fallback_and_supported(raw_value)
    {
        return Some(format!(
            "NEST|{suffix}|{property}: {fallback}\nNEST_SUPPORTS|{suffix}|{COLOR_MIX_SUPPORTS_CONDITION}|{property}: {supported}"
        ));
    }

    let value = color_value(raw_value)?;
    Some(format!("NEST|{suffix}|{property}: {value}"))
}

fn child_color_property_css(child: &str, property: &str, raw_value: &str) -> Option<String> {
    if let Some((fallback, supported)) =
        tailwind_v43_palette_opacity_fallback_and_supported(raw_value)
    {
        return Some(format!(
            "CHILD|{child}|{property}: {fallback}\nCHILD_SUPPORTS|{child}|{COLOR_MIX_SUPPORTS_CONDITION}|{property}: {supported}"
        ));
    }

    let value = color_value(raw_value)?;
    Some(format!("CHILD|{child}|{property}: {value}"))
}

fn gradient_color_stop_css(
    property: &str,
    raw_value: &str,
    stops_declaration: &str,
) -> Option<String> {
    let css = color_property_css(property, raw_value)?;
    Some(append_color_hook_base_declaration(css, stops_declaration))
}

fn color_hook_property_css(
    property: &str,
    raw_value: &str,
    alpha_property: Option<&str>,
) -> Option<String> {
    if let Some((fallback, supported)) =
        tailwind_v43_color_hook_fallback_and_supported(raw_value, alpha_property)
    {
        return Some(format!(
            "BASE|{property}: {fallback}\nSUPPORTS|{COLOR_MIX_SUPPORTS_CONDITION}|{property}: {supported}"
        ));
    }

    let value = color_hook_value(raw_value)?;
    Some(format!("{property}: {value}"))
}

fn append_color_hook_base_declaration(css: String, declaration: &str) -> String {
    if css.contains("BASE|") || css.contains("SUPPORTS|") {
        format!("{css}\nBASE|{declaration}")
    } else {
        format!("{css}; {declaration}")
    }
}

fn tailwind_v43_color_hook_fallback_and_supported(
    raw_value: &str,
    alpha_property: Option<&str>,
) -> Option<(String, String)> {
    let (name, alpha) = split_color_alpha(raw_value)?;
    let fallback = tailwind_v43_color_hook_fallback(name, alpha.as_deref())?;
    let variable = format!("var(--color-{name})");
    let color_value = if let Some(alpha) = alpha.as_deref() {
        color_mix_opacity_value(&variable, alpha)
    } else {
        variable
    };
    let supported = if let Some(alpha_property) = alpha_property {
        color_mix_opacity_value(&color_value, alpha_property)
    } else {
        color_value
    };
    Some((fallback, supported))
}

fn tailwind_v43_color_hook_fallback(name: &str, alpha: Option<&str>) -> Option<String> {
    let value = color_palette::tailwind_v43_oklch_color(name)?;
    if let Some(alpha) = alpha {
        if is_runtime_alpha_value(alpha) {
            return Some(value.to_string());
        }
        return Some(color_mix_srgb_opacity_value(value, alpha));
    }
    Some(value.to_string())
}

fn tailwind_v43_palette_opacity_fallback_and_supported(
    raw_value: &str,
) -> Option<(String, String)> {
    let (name, alpha) = split_color_alpha(raw_value)?;
    let alpha = alpha?;
    let oklch = color_palette::tailwind_v43_oklch_color(name)?;
    let fallback = if is_runtime_alpha_value(&alpha) {
        oklch.to_string()
    } else {
        color_mix_srgb_opacity_value(oklch, &alpha)
    };
    let variable = format!("var(--color-{name})");
    let supported = color_mix_opacity_value(&variable, &alpha);
    Some((fallback, supported))
}

fn is_runtime_alpha_value(alpha: &str) -> bool {
    let alpha = alpha.trim();
    alpha.starts_with("var(") || alpha.starts_with("calc(") || alpha.starts_with("clamp(")
}

fn color_mix_srgb_opacity_value(value: &str, alpha: &str) -> String {
    let alpha = alpha.trim();
    let percent = if alpha.ends_with('%')
        || alpha.starts_with("var(")
        || alpha.starts_with("calc(")
        || alpha.starts_with("clamp(")
    {
        alpha.to_string()
    } else {
        alpha
            .parse::<f32>()
            .ok()
            .map(|value| format!("{}%", trim_float(value * 100.0)))
            .unwrap_or_else(|| alpha.to_string())
    };
    format!("color-mix(in srgb, {value} {percent}, transparent)")
}

fn directional_border_color_utility(class_name: &str) -> Option<String> {
    let border_color_rules: &[(&str, &[&str])] = &[
        ("border-x-", &["border-inline-color"]),
        ("border-y-", &["border-block-color"]),
        ("border-t-", &["border-top-color"]),
        ("border-r-", &["border-right-color"]),
        ("border-b-", &["border-bottom-color"]),
        ("border-l-", &["border-left-color"]),
        ("border-s-", &["border-inline-start-color"]),
        ("border-e-", &["border-inline-end-color"]),
        ("border-bs-", &["border-block-start-color"]),
        ("border-be-", &["border-block-end-color"]),
    ];

    for (prefix, properties) in border_color_rules {
        let Some(raw_value) = class_name.strip_prefix(prefix) else {
            continue;
        };
        if properties.len() == 1 {
            return color_property_css(properties[0], raw_value);
        }
        if let Some((fallback, supported)) =
            tailwind_v43_palette_opacity_fallback_and_supported(raw_value)
        {
            return Some(format!(
                "BASE|{}\nSUPPORTS|{COLOR_MIX_SUPPORTS_CONDITION}|{}",
                join_properties(properties, &fallback),
                join_properties(properties, &supported)
            ));
        }
        let value = color_value(raw_value)?;
        return Some(join_properties(properties, &value));
    }

    None
}

fn is_border_color_arbitrary_value(raw_value: &str) -> bool {
    typed_custom_property_var_value(raw_value, "color").is_some()
        || custom_property_var_value(raw_value).is_some()
        || is_arbitrary_color_hook_value(raw_value)
        || arbitrary_value(raw_value)
            .as_deref()
            .is_some_and(theme_tokens::is_color_like_custom_property_reference)
}

fn gradient_stop_position_value(raw_value: &str) -> Option<String> {
    if let Some(value) = arbitrary_value(raw_value) {
        return is_gradient_stop_position_value(&value).then_some(value);
    }
    if let Some(variable) = raw_value
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))
    {
        return (is_custom_property_name(variable)
            && (variable.contains("position") || variable.contains("stop")))
        .then(|| format!("var({variable})"));
    }
    percentage_position_value(raw_value)
}

fn is_gradient_stop_position_value(value: &str) -> bool {
    percentage_position_value(value).is_some()
        || value.starts_with("var(")
        || value.starts_with("calc(")
}

fn percentage_position_value(raw_value: &str) -> Option<String> {
    let value = raw_value.strip_suffix('%')?.parse::<f32>().ok()?;
    value.is_finite().then(|| format!("{}%", trim_float(value)))
}

fn background_utility(class_name: &str) -> Option<String> {
    if let Some(css) = match class_name {
        "bg-fixed" => Some("background-attachment: fixed"),
        "bg-local" => Some("background-attachment: local"),
        "bg-scroll" => Some("background-attachment: scroll"),
        "bg-clip-border" => Some("background-clip: border-box"),
        "bg-clip-padding" => Some("background-clip: padding-box"),
        "bg-clip-content" => Some("background-clip: content-box"),
        "bg-clip-text" => Some("background-clip: text"),
        "bg-origin-border" => Some("background-origin: border-box"),
        "bg-origin-padding" => Some("background-origin: padding-box"),
        "bg-origin-content" => Some("background-origin: content-box"),
        "bg-bottom" => Some("background-position: bottom"),
        "bg-center" => Some("background-position: center"),
        "bg-left" => Some("background-position: left"),
        "bg-left-bottom" => Some("background-position: left bottom"),
        "bg-left-top" => Some("background-position: left top"),
        "bg-right" => Some("background-position: right"),
        "bg-right-bottom" => Some("background-position: right bottom"),
        "bg-right-top" => Some("background-position: right top"),
        "bg-top" => Some("background-position: top"),
        "bg-auto" => Some("background-size: auto"),
        "bg-cover" => Some("background-size: cover"),
        "bg-contain" => Some("background-size: contain"),
        "bg-repeat" => Some("background-repeat: repeat"),
        "bg-no-repeat" => Some("background-repeat: no-repeat"),
        "bg-repeat-x" => Some("background-repeat: repeat-x"),
        "bg-repeat-y" => Some("background-repeat: repeat-y"),
        "bg-repeat-round" => Some("background-repeat: round"),
        "bg-repeat-space" => Some("background-repeat: space"),
        "bg-none" => Some("background-image: none"),
        _ => None,
    } {
        return Some(css.to_string());
    }

    if let Some(raw_value) = class_name.strip_prefix("bg-size-") {
        let value = arbitrary_or_custom_property_value(raw_value)?;
        return Some(format!("background-size: {}", value));
    }
    if let Some(raw_value) = class_name.strip_prefix("bg-position-") {
        let value = arbitrary_or_custom_property_value(raw_value)?;
        return Some(format!("background-position: {}", value));
    }
    if let Some(raw_value) = class_name.strip_prefix("bg-linear-to-") {
        let (raw_value, interpolation) = split_gradient_interpolation_modifier(raw_value)?;
        let direction = linear_gradient_direction(raw_value)?;
        return Some(format!(
            "background-image: linear-gradient({}{}, var(--tw-gradient-stops))",
            direction,
            gradient_interpolation_suffix(interpolation.as_deref())
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("bg-linear-") {
        return linear_gradient_image(raw_value, false);
    }
    if let Some(raw_value) = class_name.strip_prefix("-bg-linear-") {
        return linear_gradient_image(raw_value, true);
    }
    if let Some(raw_value) = class_name.strip_prefix("bg-radial/") {
        let interpolation = gradient_interpolation_modifier(raw_value)?;
        return Some(format!(
            "background-image: radial-gradient(in {}, var(--tw-gradient-stops))",
            interpolation
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("bg-radial-") {
        let (raw_value, interpolation) = split_gradient_interpolation_modifier(raw_value)?;
        let value = gradient_image_value(raw_value)?;
        return Some(format!(
            "background-image: radial-gradient({}{}, var(--tw-gradient-stops))",
            value,
            gradient_interpolation_suffix(interpolation.as_deref())
        ));
    }
    if class_name == "bg-radial" {
        return Some("--tw-gradient-position: in oklab; background-image: radial-gradient(var(--tw-gradient-stops))".to_string());
    }
    if let Some(raw_value) = class_name.strip_prefix("bg-conic/") {
        let interpolation = gradient_interpolation_modifier(raw_value)?;
        return Some(format!(
            "background-image: conic-gradient(in {}, var(--tw-gradient-stops))",
            interpolation
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("bg-conic-") {
        return conic_gradient_image(raw_value, false);
    }
    if let Some(raw_value) = class_name.strip_prefix("-bg-conic-") {
        return conic_gradient_image(raw_value, true);
    }
    if class_name == "bg-conic" {
        return Some("--tw-gradient-position: in oklab; background-image: conic-gradient(var(--tw-gradient-stops))".to_string());
    }
    if let Some(raw_value) = class_name.strip_prefix("bg-") {
        let value = background_image_arbitrary_value(raw_value)?;
        return Some(format!("background-image: {value}"));
    }

    None
}

fn linear_gradient_image(raw_value: &str, negative: bool) -> Option<String> {
    let (raw_value, interpolation) = split_gradient_interpolation_modifier(raw_value)?;
    if let Some(value) = gradient_image_value(raw_value) {
        return Some(format!(
            "background-image: linear-gradient({}{}, var(--tw-gradient-stops))",
            value,
            gradient_interpolation_suffix(interpolation.as_deref())
        ));
    }

    let angle = gradient_angle_value(raw_value, negative)?;
    Some(format!(
        "background-image: linear-gradient({}{}, var(--tw-gradient-stops))",
        angle,
        gradient_interpolation_suffix(interpolation.as_deref())
    ))
}

fn gradient_image_value(raw_value: &str) -> Option<String> {
    arbitrary_or_custom_property_value(raw_value)
}

fn arbitrary_or_custom_property_value(raw_value: &str) -> Option<String> {
    if let Some(value) = arbitrary_value(raw_value) {
        return Some(value);
    }
    if let Some(variable) = raw_value
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))
    {
        return is_custom_property_name(variable).then(|| format!("var({variable})"));
    }
    None
}

fn conic_gradient_image(raw_value: &str, negative: bool) -> Option<String> {
    let (raw_value, interpolation) = split_gradient_interpolation_modifier(raw_value)?;
    if let Some(value) = gradient_image_value(raw_value) {
        return Some(format!(
            "background-image: conic-gradient({}{}, var(--tw-gradient-stops))",
            value,
            gradient_interpolation_suffix(interpolation.as_deref())
        ));
    }

    let angle = gradient_angle_value(raw_value, negative)?;
    Some(format!(
        "background-image: conic-gradient(from {}{}, var(--tw-gradient-stops))",
        angle,
        gradient_interpolation_suffix(interpolation.as_deref())
    ))
}

fn split_gradient_interpolation_modifier(raw_value: &str) -> Option<(&str, Option<String>)> {
    let mut bracket_depth = 0usize;
    let mut paren_depth = 0usize;
    let mut slash_index = None;

    for (index, byte) in raw_value.bytes().enumerate() {
        match byte {
            b'[' => bracket_depth = bracket_depth.saturating_add(1),
            b']' => bracket_depth = bracket_depth.saturating_sub(1),
            b'(' if bracket_depth == 0 => paren_depth = paren_depth.saturating_add(1),
            b')' if bracket_depth == 0 => paren_depth = paren_depth.saturating_sub(1),
            b'/' if bracket_depth == 0 && paren_depth == 0 => slash_index = Some(index),
            _ => {}
        }
    }

    let Some(index) = slash_index else {
        return Some((raw_value, None));
    };
    let base = &raw_value[..index];
    let modifier = &raw_value[index + 1..];
    if base.is_empty() || modifier.is_empty() {
        return None;
    }

    Some((base, Some(gradient_interpolation_modifier(modifier)?)))
}

fn gradient_interpolation_suffix(interpolation: Option<&str>) -> String {
    interpolation
        .map(|value| format!(" in {value}"))
        .unwrap_or_default()
}

fn gradient_interpolation_modifier(raw_value: &str) -> Option<String> {
    match raw_value {
        "srgb" | "hsl" | "oklab" | "oklch" => Some(raw_value.to_string()),
        "longer" | "shorter" | "increasing" | "decreasing" => {
            Some(format!("oklch {raw_value} hue"))
        }
        _ => arbitrary_gradient_interpolation_modifier(raw_value),
    }
}

fn arbitrary_gradient_interpolation_modifier(raw_value: &str) -> Option<String> {
    let value = arbitrary_value(raw_value)?;
    let value = value.trim();
    let interpolation = value.strip_prefix("in ")?.trim();
    if interpolation.is_empty() || !is_safe_css_value(interpolation) {
        return None;
    }
    Some(interpolation.to_string())
}

fn gradient_angle_value(raw_value: &str, negative: bool) -> Option<String> {
    let value = arbitrary_value(raw_value).unwrap_or_else(|| raw_value.to_string());
    let angle = if is_angle_value(&value) {
        value
    } else {
        let parsed = value.parse::<f32>().ok()?;
        format!("{}deg", trim_float(parsed))
    };

    if negative {
        Some(format!("-{angle}"))
    } else {
        Some(angle)
    }
}

fn is_angle_value(value: &str) -> bool {
    let number = value
        .strip_suffix("deg")
        .or_else(|| value.strip_suffix("rad"))
        .or_else(|| value.strip_suffix("grad"))
        .or_else(|| value.strip_suffix("turn"));
    number.is_some_and(|raw| raw.parse::<f32>().is_ok())
}

fn background_image_arbitrary_value(raw_value: &str) -> Option<String> {
    if let Some(variable) = raw_value
        .strip_prefix("(image:")
        .and_then(|value| value.strip_suffix(')'))
    {
        return is_custom_property_name(variable).then(|| format!("var({variable})"));
    }

    let value = arbitrary_value(raw_value)?;
    if !is_safe_css_value(&value) {
        return None;
    }
    let lower = value.trim_start().to_ascii_lowercase();
    let image_function = [
        "url(",
        "image(",
        "image-set(",
        "cross-fade(",
        "element(",
        "linear-gradient(",
        "radial-gradient(",
        "conic-gradient(",
        "repeating-linear-gradient(",
        "repeating-radial-gradient(",
        "repeating-conic-gradient(",
    ]
    .iter()
    .any(|prefix| lower.starts_with(prefix));

    image_function.then_some(value)
}

fn linear_gradient_direction(raw_value: &str) -> Option<&'static str> {
    match raw_value {
        "t" => Some("to top"),
        "tr" => Some("to top right"),
        "r" => Some("to right"),
        "br" => Some("to bottom right"),
        "b" => Some("to bottom"),
        "bl" => Some("to bottom left"),
        "l" => Some("to left"),
        "tl" => Some("to top left"),
        _ => None,
    }
}

fn blend_utility(class_name: &str) -> Option<String> {
    if let Some(raw_value) = class_name.strip_prefix("bg-blend-") {
        let value = blend_mode_value(raw_value, false)?;
        return Some(format!("background-blend-mode: {}", value));
    }
    if let Some(raw_value) = class_name.strip_prefix("mix-blend-") {
        let value = blend_mode_value(raw_value, true)?;
        return Some(format!("mix-blend-mode: {}", value));
    }
    None
}

fn blend_mode_value(raw_value: &str, allow_plus_modes: bool) -> Option<String> {
    if let Some(value) = arbitrary_value(raw_value) {
        return Some(value);
    }
    if let Some(variable) = raw_value
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))
    {
        return is_custom_property_name(variable).then(|| format!("var({variable})"));
    }

    let value = match raw_value {
        "normal" => "normal",
        "multiply" => "multiply",
        "screen" => "screen",
        "overlay" => "overlay",
        "darken" => "darken",
        "lighten" => "lighten",
        "color-dodge" => "color-dodge",
        "color-burn" => "color-burn",
        "hard-light" => "hard-light",
        "soft-light" => "soft-light",
        "difference" => "difference",
        "exclusion" => "exclusion",
        "hue" => "hue",
        "saturation" => "saturation",
        "color" => "color",
        "luminosity" => "luminosity",
        "plus-darker" if allow_plus_modes => "plus-darker",
        "plus-lighter" if allow_plus_modes => "plus-lighter",
        _ => return None,
    };
    Some(value.to_string())
}

fn effect_utility(class_name: &str) -> Option<String> {
    let (class_name, negative) = class_name
        .strip_prefix('-')
        .map_or((class_name, false), |stripped| (stripped, true));

    if class_name == "filter" {
        return Some(format_filter_css("filter", FILTER_VALUE));
    }
    if class_name == "backdrop-filter" {
        return Some(backdrop_filter_declaration(BACKDROP_FILTER_VALUE));
    }

    if let Some(css) = match class_name {
        "shadow-sm" => Some("box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05)"),
        "shadow" => {
            Some("box-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)")
        }
        "shadow-md" => {
            Some("box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)")
        }
        "shadow-lg" => {
            Some("box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)")
        }
        "shadow-xl" => {
            Some("box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)")
        }
        "shadow-2xl" => Some("box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25)"),
        "shadow-inner" => Some("box-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05)"),
        "shadow-none" => Some("box-shadow: 0 0 #0000"),
        "outline-none" => Some("outline: 2px solid transparent; outline-offset: 2px"),
        "outline" => Some("outline-style: solid"),
        "outline-solid" => Some("outline-style: solid"),
        "outline-dashed" => Some("outline-style: dashed"),
        "outline-dotted" => Some("outline-style: dotted"),
        "outline-double" => Some("outline-style: double"),
        "outline-hidden" => Some("outline-style: hidden"),
        "ring-inset" => Some("--tw-ring-inset: inset"),
        "blur-none" => Some("filter: blur(0)"),
        "grayscale" => Some("filter: grayscale(100%)"),
        "grayscale-0" => Some("filter: grayscale(0)"),
        "invert" => Some("filter: invert(100%)"),
        "invert-0" => Some("filter: invert(0)"),
        "filter" => Some(
            "filter: var(--tw-blur, ) var(--tw-brightness, ) var(--tw-contrast, ) var(--tw-grayscale, ) var(--tw-hue-rotate, ) var(--tw-invert, ) var(--tw-saturate, ) var(--tw-sepia, ) var(--tw-drop-shadow, )",
        ),
        "filter-none" => Some("filter: none"),
        "backdrop-filter" => Some(
            "-webkit-backdrop-filter: var(--tw-backdrop-blur, ) var(--tw-backdrop-brightness, ) var(--tw-backdrop-contrast, ) var(--tw-backdrop-grayscale, ) var(--tw-backdrop-hue-rotate, ) var(--tw-backdrop-invert, ) var(--tw-backdrop-opacity, ) var(--tw-backdrop-saturate, ) var(--tw-backdrop-sepia, ); backdrop-filter: var(--tw-backdrop-blur, ) var(--tw-backdrop-brightness, ) var(--tw-backdrop-contrast, ) var(--tw-backdrop-grayscale, ) var(--tw-backdrop-hue-rotate, ) var(--tw-backdrop-invert, ) var(--tw-backdrop-opacity, ) var(--tw-backdrop-saturate, ) var(--tw-backdrop-sepia, )",
        ),
        "backdrop-filter-none" => Some("-webkit-backdrop-filter: none; backdrop-filter: none"),
        "bg-gradient-to-t" => {
            Some("background-image: linear-gradient(to top, var(--tw-gradient-stops))")
        }
        "bg-gradient-to-tr" => {
            Some("background-image: linear-gradient(to top right, var(--tw-gradient-stops))")
        }
        "bg-gradient-to-r" => {
            Some("background-image: linear-gradient(to right, var(--tw-gradient-stops))")
        }
        "bg-gradient-to-br" => {
            Some("background-image: linear-gradient(to bottom right, var(--tw-gradient-stops))")
        }
        "bg-gradient-to-b" => {
            Some("background-image: linear-gradient(to bottom, var(--tw-gradient-stops))")
        }
        "bg-gradient-to-bl" => {
            Some("background-image: linear-gradient(to bottom left, var(--tw-gradient-stops))")
        }
        "bg-gradient-to-l" => {
            Some("background-image: linear-gradient(to left, var(--tw-gradient-stops))")
        }
        "bg-gradient-to-tl" => {
            Some("background-image: linear-gradient(to top left, var(--tw-gradient-stops))")
        }
        _ => None,
    } {
        return Some(css.to_string());
    }

    if let Some(raw_value) = class_name.strip_prefix("opacity-") {
        let value = opacity_value(raw_value)?;
        return Some(format!("opacity: {}", value));
    }
    if let Some(raw_value) = class_name.strip_prefix("shadow-") {
        let value = shadow_value(raw_value)?;
        return Some(format!("box-shadow: {}", value));
    }
    if class_name == "inset-shadow" {
        return Some("box-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05)".to_string());
    }
    if let Some(raw_value) = class_name.strip_prefix("inset-shadow-") {
        let value = inset_shadow_value(raw_value)?;
        return Some(format!("box-shadow: {}", value));
    }
    if let Some(css) = text_shadow_utility(class_name) {
        return Some(css);
    }
    if let Some(raw_value) = class_name.strip_prefix("ring-offset-") {
        let width = border_size_value(raw_value)?;
        return Some(format!("--tw-ring-offset-width: {}", width));
    }
    if let Some(raw_value) = class_name.strip_prefix("ring-") {
        let width = border_size_value(raw_value)?;
        return Some(format!(
            "--tw-ring-offset-width: 0px; box-shadow: var(--tw-ring-inset,) 0 0 0 calc({} + var(--tw-ring-offset-width, 0px)) var(--tw-ring-color, rgb(59 130 246 / 0.5))",
            width
        ));
    }
    if class_name == "inset-ring" {
        return Some(inset_ring_shadow_css("1px"));
    }
    if let Some(raw_value) = class_name.strip_prefix("inset-ring-") {
        let width = border_size_value(raw_value)?;
        return Some(inset_ring_shadow_css(&width));
    }
    if let Some(raw_value) = class_name.strip_prefix("outline-offset-") {
        let value = border_size_value(raw_value)?;
        return Some(format!("outline-offset: {}", value));
    }
    if let Some(raw_value) = class_name.strip_prefix("outline-") {
        let value = border_size_value(raw_value)?;
        return Some(format!("outline-width: {}", value));
    }
    if let Some(raw_value) = class_name.strip_prefix("blur-") {
        let value = blur_value(raw_value)?;
        return Some(format!(
            "--tw-blur: blur({}); filter: {}",
            value, FILTER_VALUE
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("brightness-") {
        let value = filter_numeric_value(raw_value)?;
        return Some(format!(
            "--tw-brightness: brightness({}); filter: {}",
            value, FILTER_VALUE
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("contrast-") {
        let value = filter_numeric_value(raw_value)?;
        return Some(format!(
            "--tw-contrast: contrast({}); filter: {}",
            value, FILTER_VALUE
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("saturate-") {
        let value = filter_numeric_value(raw_value)?;
        return Some(format!(
            "--tw-saturate: saturate({}); filter: {}",
            value, FILTER_VALUE
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("sepia-") {
        let value = filter_numeric_value(raw_value)?;
        return Some(format!(
            "--tw-sepia: sepia({}); filter: {}",
            value, FILTER_VALUE
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("hue-rotate-") {
        let mut value = angle_value(raw_value)?;
        if negative {
            value.insert(0, '-');
        }
        return Some(format!(
            "--tw-hue-rotate: hue-rotate({}); filter: {}",
            value, FILTER_VALUE
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("backdrop-blur-") {
        let value = blur_value(raw_value)?;
        return Some(format!(
            "--tw-backdrop-blur: blur({}); {}",
            value,
            backdrop_filter_declaration(BACKDROP_FILTER_VALUE)
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("backdrop-brightness-") {
        let value = filter_numeric_value(raw_value)?;
        return Some(format!(
            "--tw-backdrop-brightness: brightness({}); {}",
            value,
            backdrop_filter_declaration(BACKDROP_FILTER_VALUE)
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("backdrop-contrast-") {
        let value = filter_numeric_value(raw_value)?;
        return Some(format!(
            "--tw-backdrop-contrast: contrast({}); {}",
            value,
            backdrop_filter_declaration(BACKDROP_FILTER_VALUE)
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("backdrop-saturate-") {
        let value = filter_numeric_value(raw_value)?;
        return Some(format!(
            "--tw-backdrop-saturate: saturate({}); {}",
            value,
            backdrop_filter_declaration(BACKDROP_FILTER_VALUE)
        ));
    }
    if class_name == "drop-shadow" {
        return Some(format!(
            "--tw-drop-shadow: {}; filter: {}",
            drop_shadow_value("")?,
            FILTER_VALUE
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("drop-shadow-") {
        let value = drop_shadow_value(raw_value)?;
        return Some(format!(
            "--tw-drop-shadow: {}; filter: {}",
            value, FILTER_VALUE
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("backdrop-opacity-") {
        let value = filter_numeric_value(raw_value)?;
        return Some(format!(
            "--tw-backdrop-opacity: opacity({}); {}",
            value,
            backdrop_filter_declaration(BACKDROP_FILTER_VALUE)
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("backdrop-invert") {
        let value = suffix_or_default_filter_value(raw_value, "100%")?;
        return Some(format!(
            "--tw-backdrop-invert: invert({}); {}",
            value,
            backdrop_filter_declaration(BACKDROP_FILTER_VALUE)
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("backdrop-grayscale") {
        let value = suffix_or_default_filter_value(raw_value, "100%")?;
        return Some(format!(
            "--tw-backdrop-grayscale: grayscale({}); {}",
            value,
            backdrop_filter_declaration(BACKDROP_FILTER_VALUE)
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("backdrop-sepia") {
        let value = suffix_or_default_filter_value(raw_value, "100%")?;
        return Some(format!(
            "--tw-backdrop-sepia: sepia({}); {}",
            value,
            backdrop_filter_declaration(BACKDROP_FILTER_VALUE)
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("backdrop-hue-rotate-") {
        let mut value = angle_value(raw_value)?;
        if negative {
            value.insert(0, '-');
        }
        return Some(format!(
            "--tw-backdrop-hue-rotate: hue-rotate({}); {}",
            value,
            backdrop_filter_declaration(BACKDROP_FILTER_VALUE)
        ));
    }

    None
}

fn scroll_utility(class_name: &str) -> Option<String> {
    let (class_name, negative) = class_name
        .strip_prefix('-')
        .map_or((class_name, false), |stripped| (stripped, true));

    if let Some(css) = match class_name {
        "scroll-auto" => Some("scroll-behavior: auto"),
        "scroll-smooth" => Some("scroll-behavior: smooth"),
        "overscroll-auto" => Some("overscroll-behavior: auto"),
        "overscroll-contain" => Some("overscroll-behavior: contain"),
        "overscroll-none" => Some("overscroll-behavior: none"),
        "overscroll-x-auto" => Some("overscroll-behavior-x: auto"),
        "overscroll-x-contain" => Some("overscroll-behavior-x: contain"),
        "overscroll-x-none" => Some("overscroll-behavior-x: none"),
        "overscroll-y-auto" => Some("overscroll-behavior-y: auto"),
        "overscroll-y-contain" => Some("overscroll-behavior-y: contain"),
        "overscroll-y-none" => Some("overscroll-behavior-y: none"),
        "snap-none" => Some("scroll-snap-type: none"),
        "snap-x" => Some("scroll-snap-type: x var(--tw-scroll-snap-strictness)"),
        "snap-y" => Some("scroll-snap-type: y var(--tw-scroll-snap-strictness)"),
        "snap-both" => Some("scroll-snap-type: both var(--tw-scroll-snap-strictness)"),
        "snap-mandatory" => Some("--tw-scroll-snap-strictness: mandatory"),
        "snap-proximity" => Some("--tw-scroll-snap-strictness: proximity"),
        "snap-start" => Some("scroll-snap-align: start"),
        "snap-end" => Some("scroll-snap-align: end"),
        "snap-center" => Some("scroll-snap-align: center"),
        "snap-align-none" => Some("scroll-snap-align: none"),
        "snap-normal" => Some("scroll-snap-stop: normal"),
        "snap-always" => Some("scroll-snap-stop: always"),
        _ => None,
    } {
        return Some(css.to_string());
    }

    let scroll_spacing_rules: &[(&str, &[&str], bool)] = &[
        ("scroll-m-", &["scroll-margin"], true),
        ("scroll-mx-", &["scroll-margin-inline"], true),
        ("scroll-my-", &["scroll-margin-block"], true),
        ("scroll-ms-", &["scroll-margin-inline-start"], true),
        ("scroll-me-", &["scroll-margin-inline-end"], true),
        ("scroll-mbs-", &["scroll-margin-block-start"], true),
        ("scroll-mbe-", &["scroll-margin-block-end"], true),
        ("scroll-mt-", &["scroll-margin-top"], true),
        ("scroll-mr-", &["scroll-margin-right"], true),
        ("scroll-mb-", &["scroll-margin-bottom"], true),
        ("scroll-ml-", &["scroll-margin-left"], true),
        ("scroll-p-", &["scroll-padding"], false),
        ("scroll-px-", &["scroll-padding-inline"], false),
        ("scroll-py-", &["scroll-padding-block"], false),
        ("scroll-ps-", &["scroll-padding-inline-start"], false),
        ("scroll-pe-", &["scroll-padding-inline-end"], false),
        ("scroll-pbs-", &["scroll-padding-block-start"], false),
        ("scroll-pbe-", &["scroll-padding-block-end"], false),
        ("scroll-pt-", &["scroll-padding-top"], false),
        ("scroll-pr-", &["scroll-padding-right"], false),
        ("scroll-pb-", &["scroll-padding-bottom"], false),
        ("scroll-pl-", &["scroll-padding-left"], false),
    ];
    for (prefix, properties, supports_negative) in scroll_spacing_rules {
        let Some(raw_value) = class_name.strip_prefix(prefix) else {
            continue;
        };
        if negative && !supports_negative {
            return None;
        }
        let mut value = spacing_value(raw_value, false)?;
        if negative && value != "0px" {
            value = negate_css_value(value);
        }
        return Some(join_properties(properties, &value));
    }

    None
}

fn scrollbar_utility(class_name: &str) -> Option<String> {
    if let Some(css) = match class_name {
        "scrollbar-auto" => Some("scrollbar-width: auto".to_string()),
        "scrollbar-thin" => Some("scrollbar-width: thin".to_string()),
        "scrollbar-none" => Some("scrollbar-width: none".to_string()),
        "scrollbar-gutter-auto" => Some("scrollbar-gutter: auto".to_string()),
        "scrollbar-gutter-stable" => Some("scrollbar-gutter: stable".to_string()),
        "scrollbar-gutter-both" => Some("scrollbar-gutter: stable both-edges".to_string()),
        _ => None,
    } {
        return Some(css);
    }

    if let Some(raw_value) = class_name.strip_prefix("scrollbar-thumb-") {
        return scrollbar_color_declaration("--tw-scrollbar-thumb", raw_value);
    }
    if let Some(raw_value) = class_name.strip_prefix("scrollbar-track-") {
        return scrollbar_color_declaration("--tw-scrollbar-track", raw_value);
    }

    None
}

fn scrollbar_color_declaration(role: &str, raw_value: &str) -> Option<String> {
    let css = color_property_css(role, raw_value)?;
    Some(append_color_hook_base_declaration(
        css,
        "scrollbar-color: var(--tw-scrollbar-thumb) var(--tw-scrollbar-track)",
    ))
}

fn format_filter_css(property: &str, value: &str) -> String {
    format!("{}: {}", property, value)
}

fn backdrop_filter_declaration(value: &str) -> String {
    format!(
        "-webkit-backdrop-filter: {}; backdrop-filter: {}",
        value, value
    )
}

fn blur_value(raw_value: &str) -> Option<String> {
    if let Some(value) = arbitrary_value(raw_value).or_else(|| custom_property_var_value(raw_value))
    {
        return Some(value);
    }
    match raw_value {
        "none" => Some("0".to_string()),
        "sm" => Some("var(--blur-sm)".to_string()),
        "" => Some("8px".to_string()),
        "md" => Some("var(--blur-md)".to_string()),
        "lg" => Some("var(--blur-lg)".to_string()),
        "xl" => Some("var(--blur-xl)".to_string()),
        "2xl" => Some("var(--blur-2xl)".to_string()),
        "3xl" => Some("var(--blur-3xl)".to_string()),
        _ => None,
    }
}

fn filter_numeric_value(raw_value: &str) -> Option<String> {
    if let Some(value) = arbitrary_value(raw_value) {
        return Some(value);
    }
    if let Some(value) = custom_property_var_value(raw_value) {
        return Some(value);
    }
    let value = raw_value.parse::<f32>().ok()?;
    Some(format!("{}%", trim_float(value)))
}

fn suffix_or_default_filter_value(raw_value: &str, default: &str) -> Option<String> {
    if raw_value.is_empty() {
        return Some(default.to_string());
    }
    let raw_value = raw_value.strip_prefix('-')?;
    filter_numeric_value(raw_value)
}

fn drop_shadow_value(raw_value: &str) -> Option<String> {
    let value = match raw_value {
        "none" => return Some(String::new()),
        "sm" => "drop-shadow(0 1px 1px rgb(0 0 0 / 0.05))".to_string(),
        "" => "drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06))"
            .to_string(),
        "md" => "drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06))"
            .to_string(),
        "lg" => "drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1))"
            .to_string(),
        "xl" => {
            "drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08))"
                .to_string()
        }
        "2xl" => "drop-shadow(0 25px 25px rgb(0 0 0 / 0.15))".to_string(),
        _ => {
            let value = arbitrary_or_custom_property_value(raw_value)?;
            if value.starts_with("drop-shadow(") {
                value
            } else {
                format!("drop-shadow({value})")
            }
        }
    };
    Some(value)
}

fn shadow_value(raw_value: &str) -> Option<String> {
    arbitrary_or_custom_property_value(raw_value)
}

fn inset_ring_shadow_css(width: &str) -> String {
    format!(
        "--tw-inset-ring-shadow: inset 0 0 0 {} var(--tw-inset-ring-color, currentcolor); box-shadow: {}",
        width, SHADOW_STACK_VALUE
    )
}

fn inset_shadow_value(raw_value: &str) -> Option<String> {
    let value = match raw_value {
        "none" => "inset 0 0 #0000".to_string(),
        "sm" => "inset 0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string(),
        _ => arbitrary_or_custom_property_value(raw_value)?,
    };
    Some(value)
}

fn transition_utility(class_name: &str) -> Option<String> {
    const DEFAULT_EASING: &str = "cubic-bezier(0.4, 0, 0.2, 1)";
    const DEFAULT_DURATION: &str = "150ms";

    let transition = |property: &str| {
        format!(
            "transition-property: {}; transition-timing-function: {}; transition-duration: {}",
            property, DEFAULT_EASING, DEFAULT_DURATION
        )
    };

    if let Some(css) = match class_name {
        "transition-none" => Some("transition-property: none".to_string()),
        "transition-all" => Some(transition("all")),
        "transition" => Some(transition(
            "color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter",
        )),
        "transition-colors" => Some(transition(
            "color, background-color, border-color, text-decoration-color, fill, stroke",
        )),
        "transition-opacity" => Some(transition("opacity")),
        "transition-shadow" => Some(transition("box-shadow")),
        "transition-transform" => Some(transition("transform")),
        "transition-discrete" => Some("transition-behavior: allow-discrete".to_string()),
        "transition-normal" => Some("transition-behavior: normal".to_string()),
        _ => None,
    } {
        return Some(css);
    }

    if let Some(raw_value) = class_name.strip_prefix("duration-") {
        let value = transition_time_value(raw_value)?;
        return Some(format!("transition-duration: {}", value));
    }
    if let Some(raw_value) = class_name.strip_prefix("delay-") {
        let value = transition_time_value(raw_value)?;
        return Some(format!("transition-delay: {}", value));
    }
    if let Some(raw_value) = class_name.strip_prefix("ease-") {
        let value = match raw_value {
            "linear" => "linear".to_string(),
            "in" => "cubic-bezier(0.4, 0, 1, 1)".to_string(),
            "out" => "cubic-bezier(0, 0, 0.2, 1)".to_string(),
            "in-out" => DEFAULT_EASING.to_string(),
            _ => arbitrary_value(raw_value)?,
        };
        return Some(format!("transition-timing-function: {}", value));
    }
    if let Some(raw_value) = class_name.strip_prefix("transition-") {
        let value = transition_property_value(raw_value)?;
        return Some(transition(&value));
    }

    None
}

fn transition_time_value(raw_value: &str) -> Option<String> {
    if let Some(value) = arbitrary_value(raw_value) {
        return Some(value);
    }
    let value = raw_value.parse::<f32>().ok()?;
    Some(format!("{}ms", trim_float(value)))
}

fn transition_property_value(raw_value: &str) -> Option<String> {
    if let Some(value) = arbitrary_value(raw_value) {
        return Some(value);
    }
    if let Some(variable) = raw_value
        .strip_prefix('(')
        .and_then(|raw| raw.strip_suffix(')'))
    {
        if variable.starts_with("--")
            && variable
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_'))
        {
            return Some(format!("var({})", variable));
        }
    }
    None
}

fn zoom_utility(class_name: &str) -> Option<String> {
    let raw_value = class_name.strip_prefix("zoom-")?;
    let value = arbitrary_value(raw_value)
        .or_else(|| custom_property_var_value(raw_value))
        .or_else(|| zoom_percent_value(raw_value))?;
    Some(format!("zoom: {}", value))
}

fn zoom_percent_value(raw_value: &str) -> Option<String> {
    let value = raw_value.parse::<f32>().ok()?;
    Some(format!("{}%", trim_float(value)))
}

fn transform_utility(class_name: &str) -> Option<String> {
    let (class_name, negative) = class_name
        .strip_prefix('-')
        .map_or((class_name, false), |stripped| (stripped, true));

    if class_name == "transform" || class_name == "transform-cpu" {
        return Some(format!("transform: {}", TRANSFORM_VALUE));
    }
    if class_name == "transform-gpu" {
        return Some(format!("transform: {}", TRANSFORM_GPU_VALUE));
    }
    if class_name == "transform-none" {
        return Some("transform: none".to_string());
    }
    if let Some(raw_value) = class_name.strip_prefix("transform-") {
        let value = arbitrary_or_custom_property_value(raw_value)?;
        return Some(format!("transform: {}", value));
    }
    if let Some(value) = perspective_origin_value(class_name) {
        return Some(format!("perspective-origin: {}", value));
    }
    if let Some(raw_value) = class_name.strip_prefix("perspective-") {
        let value = perspective_value(raw_value)?;
        return Some(format!("perspective: {}", value));
    }
    if let Some(value) = transform_origin_value(class_name) {
        return Some(format!("transform-origin: {}", value));
    }

    let transform_spacing_rules: &[(&str, &str, &str)] = &[
        (
            "translate-x-",
            "--tw-translate-x",
            "var(--tw-translate-x) var(--tw-translate-y)",
        ),
        (
            "translate-y-",
            "--tw-translate-y",
            "var(--tw-translate-x) var(--tw-translate-y)",
        ),
        (
            "translate-z-",
            "--tw-translate-z",
            "var(--tw-translate-x) var(--tw-translate-y) var(--tw-translate-z)",
        ),
    ];
    for (prefix, variable, translate_value) in transform_spacing_rules {
        let Some(raw_value) = class_name.strip_prefix(prefix) else {
            continue;
        };
        let mut value = spacing_value(raw_value, false)?;
        if negative {
            value = negate_css_value(value);
        }
        return Some(format!(
            "{}: {}; translate: {}",
            variable, value, translate_value
        ));
    }

    if let Some(raw_value) = class_name.strip_prefix("scale-x-") {
        let value = signed_scale_value(raw_value, negative)?;
        return Some(format!(
            "--tw-scale-x: {}; scale: var(--tw-scale-x) var(--tw-scale-y)",
            value
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("scale-y-") {
        let value = signed_scale_value(raw_value, negative)?;
        return Some(format!(
            "--tw-scale-y: {}; scale: var(--tw-scale-x) var(--tw-scale-y)",
            value
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("scale-z-") {
        let value = signed_scale_value(raw_value, negative)?;
        return Some(format!(
            "--tw-scale-z: {}; scale: var(--tw-scale-x) var(--tw-scale-y) var(--tw-scale-z)",
            value
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("scale-") {
        let value = signed_scale_value(raw_value, negative)?;
        return Some(format!(
            "--tw-scale-x: {}; --tw-scale-y: {}; --tw-scale-z: {}; scale: var(--tw-scale-x) var(--tw-scale-y)",
            value, value, value
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("rotate-x-") {
        let value = transform_function_angle_value("rotateX", raw_value, negative)?;
        return Some(format!(
            "--tw-rotate-x: {}; transform: {}",
            value, TRANSFORM_VALUE
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("rotate-y-") {
        let value = transform_function_angle_value("rotateY", raw_value, negative)?;
        return Some(format!(
            "--tw-rotate-y: {}; transform: {}",
            value, TRANSFORM_VALUE
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("rotate-z-") {
        let value = transform_function_angle_value("rotateZ", raw_value, negative)?;
        return Some(format!(
            "--tw-rotate-z: {}; transform: {}",
            value, TRANSFORM_VALUE
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("rotate-") {
        let value = signed_angle_value(raw_value, negative)?;
        return Some(format!("rotate: {}", value));
    }
    if let Some(raw_value) = class_name.strip_prefix("skew-x-") {
        let value = transform_function_angle_value("skewX", raw_value, negative)?;
        return Some(format!(
            "--tw-skew-x: {}; transform: {}",
            value, TRANSFORM_VALUE
        ));
    }
    if let Some(raw_value) = class_name.strip_prefix("skew-y-") {
        let value = transform_function_angle_value("skewY", raw_value, negative)?;
        return Some(format!(
            "--tw-skew-y: {}; transform: {}",
            value, TRANSFORM_VALUE
        ));
    }

    None
}

fn transform_origin_value(class_name: &str) -> Option<String> {
    let raw_value = class_name.strip_prefix("origin-")?;
    position_value(raw_value)
}

fn perspective_origin_value(class_name: &str) -> Option<String> {
    let raw_value = class_name.strip_prefix("perspective-origin-")?;
    position_value(raw_value)
}

fn position_value(raw_value: &str) -> Option<String> {
    Some(match raw_value {
        "center" => "center".to_string(),
        "top" => "top".to_string(),
        "top-right" => "top right".to_string(),
        "right" => "right".to_string(),
        "bottom-right" => "bottom right".to_string(),
        "bottom" => "bottom".to_string(),
        "bottom-left" => "bottom left".to_string(),
        "left" => "left".to_string(),
        "top-left" => "top left".to_string(),
        _ => arbitrary_or_custom_property_value(raw_value)?,
    })
}

fn perspective_value(raw_value: &str) -> Option<String> {
    if raw_value == "none" {
        return Some("none".to_string());
    }
    if let Some(name) = match raw_value {
        "dramatic" => Some("--perspective-dramatic"),
        "near" => Some("--perspective-near"),
        "normal" => Some("--perspective-normal"),
        "midrange" => Some("--perspective-midrange"),
        "distant" => Some("--perspective-distant"),
        _ => None,
    } {
        return Some(format!("var({name})"));
    }
    arbitrary_or_custom_property_value(raw_value).or_else(|| {
        raw_value
            .parse::<f32>()
            .ok()
            .map(|value| format!("{}px", trim_float(value)))
    })
}

fn angle_value(raw_value: &str) -> Option<String> {
    if let Some(value) = arbitrary_value(raw_value).or_else(|| custom_property_var_value(raw_value))
    {
        return Some(value);
    }
    if is_angle_value(raw_value) {
        return Some(raw_value.to_string());
    }
    Some(format!("{}deg", trim_float(raw_value.parse::<f32>().ok()?)))
}

fn signed_angle_value(raw_value: &str, negative: bool) -> Option<String> {
    let value = angle_value(raw_value)?;
    if negative {
        Some(format!("calc({value} * -1)"))
    } else {
        Some(value)
    }
}

fn transform_function_angle_value(
    function_name: &str,
    raw_value: &str,
    negative: bool,
) -> Option<String> {
    let value = signed_angle_value(raw_value, negative)?;
    Some(format!("{function_name}({value})"))
}

fn spacing_value(raw_value: &str, allow_auto: bool) -> Option<String> {
    if allow_auto && raw_value == "auto" {
        return Some("auto".to_string());
    }
    if let Some(value) = arbitrary_value(raw_value) {
        return Some(value);
    }
    if let Some(value) = custom_property_var_value(raw_value) {
        return Some(value);
    }
    if raw_value == "px" {
        return Some("1px".to_string());
    }
    spacing_multiplier(raw_value).map(|multiplier| format!("calc(var(--spacing) * {multiplier})"))
}

fn spacing_multiplier(raw_value: &str) -> Option<String> {
    let value = raw_value.parse::<f32>().ok()?;
    value.is_finite().then(|| trim_float(value))
}

fn spacing_calc_multiplier(value: &str) -> Option<&str> {
    value
        .strip_prefix("calc(var(--spacing) * ")
        .and_then(|value| value.strip_suffix(')'))
}

fn is_inset_property(property: &str) -> bool {
    matches!(
        property,
        "inset"
            | "inset-inline"
            | "inset-block"
            | "inset-inline-start"
            | "inset-inline-end"
            | "inset-block-start"
            | "inset-block-end"
            | "top"
            | "right"
            | "bottom"
            | "left"
    )
}

fn inset_value(raw_value: &str, allow_auto: bool) -> Option<String> {
    if allow_auto && raw_value == "auto" {
        return Some("auto".to_string());
    }
    if raw_value == "full" {
        return Some("100%".to_string());
    }
    if let Some((numerator, denominator)) = raw_value.split_once('/') {
        let numerator = numerator.parse::<f32>().ok()?;
        let denominator = denominator.parse::<f32>().ok()?;
        if denominator == 0.0 || !numerator.is_finite() || !denominator.is_finite() {
            return None;
        }
        return Some(format!(
            "calc({} / {} * 100%)",
            trim_float(numerator),
            trim_float(denominator)
        ));
    }
    spacing_value(raw_value, allow_auto)
}

fn size_value(raw_value: &str, axis: SizeAxis, allow_container_scale: bool) -> Option<String> {
    if let Some(value) = arbitrary_value(raw_value) {
        return Some(value);
    }
    if let Some(value) = custom_property_size_value(raw_value) {
        return Some(value);
    }
    if let Some((numerator, denominator)) = raw_value.split_once('/') {
        let numerator = numerator.parse::<f32>().ok()?;
        let denominator = denominator.parse::<f32>().ok()?;
        if denominator == 0.0 {
            return None;
        }
        return Some(format!("calc({numerator} / {denominator} * 100%)"));
    }
    if allow_container_scale {
        if let Some(value) = container_scale_size_value(raw_value) {
            return Some(value);
        }
        if raw_value == "prose" {
            return Some("65ch".to_string());
        }
    }
    match raw_value {
        "auto" => Some("auto".to_string()),
        "full" => Some("100%".to_string()),
        "screen" => Some(
            match axis {
                SizeAxis::Width => "100vw",
                SizeAxis::Height => "100vh",
            }
            .to_string(),
        ),
        "svh" => Some("100svh".to_string()),
        "lvh" => Some("100lvh".to_string()),
        "dvh" => Some("100dvh".to_string()),
        "svw" => Some("100svw".to_string()),
        "lvw" => Some("100lvw".to_string()),
        "dvw" => Some("100dvw".to_string()),
        "min" => Some("min-content".to_string()),
        "max" => Some("max-content".to_string()),
        "fit" => Some("fit-content".to_string()),
        "none" => Some("none".to_string()),
        _ => spacing_value(raw_value, false),
    }
}

fn container_scale_size_value(raw_value: &str) -> Option<String> {
    match raw_value {
        "3xs" | "2xs" | "xs" | "sm" | "md" | "lg" | "xl" | "2xl" | "3xl" | "4xl" | "5xl"
        | "6xl" | "7xl" => Some(format!("var(--container-{raw_value})")),
        _ => None,
    }
}

fn custom_property_size_value(raw_value: &str) -> Option<String> {
    custom_property_var_value(raw_value)
}

fn custom_property_var_value(raw_value: &str) -> Option<String> {
    let variable = raw_value.strip_prefix('(')?.strip_suffix(')')?;
    is_custom_property_name(variable).then(|| format!("var({variable})"))
}

fn typed_custom_property_var_value(raw_value: &str, type_hint: &str) -> Option<String> {
    let inner = raw_value.strip_prefix('(')?.strip_suffix(')')?;
    let variable = inner.strip_prefix(type_hint)?.strip_prefix(':')?;
    is_custom_property_name(variable).then(|| format!("var({variable})"))
}

fn tab_size_utility(class_name: &str) -> Option<String> {
    let raw_value = class_name.strip_prefix("tab-")?;
    let value = arbitrary_value(raw_value)
        .or_else(|| custom_property_var_value(raw_value))
        .or_else(|| raw_value.parse::<f32>().ok().map(trim_float))?;
    Some(format!("tab-size: {}", value))
}

fn split_text_size_line_height_modifier(raw_value: &str) -> (&str, Option<&str>) {
    let mut bracket_depth = 0usize;
    let mut paren_depth = 0usize;
    let mut quote = None;
    let mut escaped = false;

    for (index, ch) in raw_value.char_indices() {
        if let Some(active_quote) = quote {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == active_quote {
                quote = None;
            }
            continue;
        }

        match ch {
            '"' | '\'' => quote = Some(ch),
            '[' => bracket_depth = bracket_depth.saturating_add(1),
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '(' if bracket_depth == 0 => paren_depth = paren_depth.saturating_add(1),
            ')' if bracket_depth == 0 => paren_depth = paren_depth.saturating_sub(1),
            '/' if bracket_depth == 0 && paren_depth == 0 => {
                let base = &raw_value[..index];
                let modifier = &raw_value[index + 1..];
                if base.is_empty() || modifier.is_empty() {
                    return (raw_value, None);
                }
                return (base, Some(modifier));
            }
            _ => {}
        }
    }

    (raw_value, None)
}

fn font_size_value(raw_value: &str) -> Option<FontSizeValue> {
    if let Some((font_size, default_line_height)) = default_font_size_value(raw_value) {
        return Some(FontSizeValue {
            font_size: font_size.to_string(),
            default_line_height: Some(default_line_height.to_string()),
        });
    }

    if let Some(value) = typed_custom_property_var_value(raw_value, "length") {
        return Some(FontSizeValue {
            font_size: value,
            default_line_height: None,
        });
    }

    let value = arbitrary_value(raw_value)?;
    let value = value.trim();
    if value.starts_with("color:") {
        return None;
    }
    if theme_tokens::is_color_like_custom_property_reference(value) {
        return None;
    }
    let value = value.strip_prefix("length:").unwrap_or(value).trim();
    (!value.is_empty()).then(|| FontSizeValue {
        font_size: value.to_string(),
        default_line_height: None,
    })
}

fn text_size_line_height_modifier_value(raw_value: &str) -> Option<String> {
    arbitrary_value(raw_value)
        .or_else(|| custom_property_var_value(raw_value))
        .or_else(|| {
            raw_value
                .parse::<f32>()
                .ok()
                .filter(|value| value.is_finite() && *value >= 0.0)
                .map(|value| format!("calc(var(--spacing) * {})", trim_float(value)))
        })
}

fn default_font_size_value(raw_value: &str) -> Option<(&'static str, &'static str)> {
    match raw_value {
        "xs" => Some(("var(--text-xs)", "1rem")),
        "sm" => Some(("var(--text-sm)", "1.25rem")),
        "base" => Some(("var(--text-base)", "1.5rem")),
        "lg" => Some(("var(--text-lg)", "1.75rem")),
        "xl" => Some(("var(--text-xl)", "1.75rem")),
        "2xl" => Some(("var(--text-2xl)", "2rem")),
        "3xl" => Some(("var(--text-3xl)", "2.25rem")),
        "4xl" => Some(("var(--text-4xl)", "2.5rem")),
        "5xl" => Some(("var(--text-5xl)", "1")),
        "6xl" => Some(("var(--text-6xl)", "1")),
        "7xl" => Some(("var(--text-7xl)", "1")),
        "8xl" => Some(("var(--text-8xl)", "1")),
        "9xl" => Some(("var(--text-9xl)", "1")),
        _ => None,
    }
}

fn radius_value(raw_value: &str) -> Option<String> {
    if let Some(value) = arbitrary_value(raw_value) {
        return Some(value);
    }
    if let Some(value) = custom_property_var_value(raw_value) {
        return Some(value);
    }
    Some(match raw_value {
        "none" => "0".to_string(),
        "xs" => "var(--radius-xs)".to_string(),
        "sm" => "var(--radius-sm)".to_string(),
        "" | "DEFAULT" => "0.25rem".to_string(),
        "md" => "var(--radius-md)".to_string(),
        "lg" => "var(--radius-lg)".to_string(),
        "xl" => "var(--radius-xl)".to_string(),
        "2xl" => "var(--radius-2xl)".to_string(),
        "3xl" => "var(--radius-3xl)".to_string(),
        "4xl" => "var(--radius-4xl)".to_string(),
        "full" => "calc(infinity * 1px)".to_string(),
        _ => return None,
    })
}

fn color_value(raw_value: &str) -> Option<String> {
    if let Some(value) = token_color_value(raw_value) {
        return Some(value);
    }
    let (name, alpha) = split_color_alpha(raw_value)?;
    if let Some(value) = arbitrary_color_value(name, alpha.as_deref()) {
        return Some(value);
    }
    if let Some(value) = typed_custom_property_var_value(name, "color") {
        return if let Some(alpha) = alpha {
            Some(color_mix_opacity_value(&value, &alpha))
        } else {
            Some(value)
        };
    }
    if let Some(value) = custom_property_var_value(name) {
        return if let Some(alpha) = alpha {
            Some(color_mix_opacity_value(&value, &alpha))
        } else {
            Some(value)
        };
    }
    match name {
        "inherit" => return Some("inherit".to_string()),
        "current" => {
            return if let Some(alpha) = alpha {
                Some(color_mix_opacity_value("currentColor", &alpha))
            } else {
                Some("currentColor".to_string())
            };
        }
        "transparent" => return Some("transparent".to_string()),
        "black" => return Some(rgb("0 0 0", alpha)),
        "white" => return Some(rgb("255 255 255", alpha)),
        _ => {}
    }
    if is_hex_color(name) {
        return hex_rgb(name, alpha);
    }
    if let Some(token) = theme_tokens::theme_color_token_name(name) {
        return if let Some(alpha) = alpha {
            Some(format!("hsl(var(--{}) / {})", token, alpha))
        } else {
            Some(format!("hsl(var(--{}))", token))
        };
    }
    if let Some(value) = tailwind_v43_neutral_palette_variable(name, alpha.as_deref()) {
        return Some(value);
    }
    tailwind_color_rgb(name).map(|channels| rgb(channels, alpha))
}

fn color_hook_value(raw_value: &str) -> Option<String> {
    let (name, alpha) = split_color_alpha(raw_value)?;
    if let Some(value) = typed_custom_property_var_value(name, "color") {
        return if let Some(alpha) = alpha {
            Some(color_mix_opacity_value(&value, &alpha))
        } else {
            Some(value)
        };
    }
    if name.starts_with('(') {
        return None;
    }
    if name.starts_with('[') && !is_arbitrary_color_hook_value(name) {
        return None;
    }
    color_value(raw_value)
}

fn is_arbitrary_color_hook_value(raw_value: &str) -> bool {
    let Some(value) = arbitrary_value(raw_value) else {
        return false;
    };
    let value = value.trim();
    let lower = value.to_ascii_lowercase();
    value.starts_with("color:")
        || value.starts_with('#')
        || lower.starts_with("rgb(")
        || lower.starts_with("rgba(")
        || lower.starts_with("hsl(")
        || lower.starts_with("hsla(")
        || lower.starts_with("oklab(")
        || lower.starts_with("oklch(")
        || lower.starts_with("color-mix(")
}

fn split_color_alpha(raw_value: &str) -> Option<(&str, Option<String>)> {
    raw_value
        .split_once('/')
        .map_or(Some((raw_value, None)), |(name, alpha)| {
            if is_opaque_color_alpha_token(alpha) {
                return Some((name, None));
            }
            color_alpha_value(alpha).map(|alpha| (name, Some(alpha)))
        })
}

fn arbitrary_color_value(raw_value: &str, alpha: Option<&str>) -> Option<String> {
    let value = arbitrary_value(raw_value)?;
    let value = value
        .strip_prefix("color:")
        .unwrap_or(value.as_str())
        .trim()
        .to_string();

    if value.starts_with('#') {
        return if let Some(alpha) = alpha {
            Some(color_mix_opacity_value(&value, alpha))
        } else {
            Some(value)
        };
    }

    if let Some(alpha) = alpha {
        return Some(color_mix_opacity_value(&value, alpha));
    }

    Some(value)
}

fn color_mix_opacity_value(value: &str, alpha: &str) -> String {
    let alpha = alpha.trim();
    let percent = if alpha.ends_with('%')
        || alpha.starts_with("var(")
        || alpha.starts_with("calc(")
        || alpha.starts_with("clamp(")
    {
        alpha.to_string()
    } else {
        alpha
            .parse::<f32>()
            .ok()
            .map(|value| format!("{}%", trim_float(value * 100.0)))
            .unwrap_or_else(|| alpha.to_string())
    };
    format!("color-mix(in oklab, {value} {percent}, transparent)")
}

fn is_length_like_color_ambiguous_value(raw_value: &str) -> bool {
    if let Some(value) = arbitrary_value(raw_value) {
        return is_length_like_css_value(&value);
    }
    if let Some(variable) = raw_value
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))
    {
        return is_custom_property_name(variable)
            && variable
                .trim_start_matches("--")
                .split('-')
                .any(|part| matches!(part, "width" | "size" | "length" | "offset"));
    }
    false
}

fn is_length_like_css_value(value: &str) -> bool {
    let value = value.trim().to_ascii_lowercase();
    value == "0"
        || value.starts_with("var(")
        || value.starts_with("calc(")
        || value.starts_with("min(")
        || value.starts_with("max(")
        || value.starts_with("clamp(")
        || value.ends_with("px")
        || value.ends_with("rem")
        || value.ends_with("em")
        || value.ends_with("ch")
        || value.ends_with("ex")
        || value.ends_with("cap")
        || value.ends_with("ic")
        || value.ends_with("lh")
        || value.ends_with("rlh")
        || value.ends_with("vw")
        || value.ends_with("vh")
        || value.ends_with("vi")
        || value.ends_with("vb")
        || value.ends_with("vmin")
        || value.ends_with("vmax")
        || value.ends_with("svw")
        || value.ends_with("svh")
        || value.ends_with("lvw")
        || value.ends_with("lvh")
        || value.ends_with("dvw")
        || value.ends_with("dvh")
}

fn token_color_value(raw_value: &str) -> Option<String> {
    let (raw_token, alpha) =
        raw_value
            .split_once('/')
            .map_or((raw_value, None), |(name, alpha)| {
                if is_opaque_color_alpha_token(alpha) {
                    (name, None)
                } else {
                    (name, color_alpha_value(alpha))
                }
            });
    let token = raw_token.strip_prefix("token(")?.strip_suffix(')')?.trim();
    let token = theme_tokens::theme_color_token_name(token)?;
    if let Some(alpha) = alpha {
        Some(format!("hsl(var(--{}) / {})", token, alpha))
    } else {
        Some(format!("hsl(var(--{}))", token))
    }
}

fn tailwind_v43_neutral_palette_variable(name: &str, alpha: Option<&str>) -> Option<String> {
    color_palette::tailwind_v43_oklch_color(name)?;
    let variable = format!("var(--color-{name})");
    if let Some(alpha) = alpha {
        Some(color_mix_opacity_value(&variable, alpha))
    } else {
        Some(variable)
    }
}

fn tailwind_color_rgb(name: &str) -> Option<&'static str> {
    let (palette, shade) = name.rsplit_once('-')?;
    match (palette, shade) {
        ("slate", "50") => Some("248 250 252"),
        ("slate", "100") => Some("241 245 249"),
        ("slate", "200") => Some("226 232 240"),
        ("slate", "300") => Some("203 213 225"),
        ("slate", "400") => Some("148 163 184"),
        ("slate", "500") => Some("100 116 139"),
        ("slate", "600") => Some("71 85 105"),
        ("slate", "700") => Some("51 65 85"),
        ("slate", "800") => Some("30 41 59"),
        ("slate", "900") => Some("15 23 42"),
        ("slate", "950") => Some("2 6 23"),
        ("gray", "50") => Some("249 250 251"),
        ("gray", "100") => Some("243 244 246"),
        ("gray", "200") => Some("229 231 235"),
        ("gray", "300") => Some("209 213 219"),
        ("gray", "400") => Some("156 163 175"),
        ("gray", "500") => Some("107 114 128"),
        ("gray", "600") => Some("75 85 99"),
        ("gray", "700") => Some("55 65 81"),
        ("gray", "800") => Some("31 41 55"),
        ("gray", "900") => Some("17 24 39"),
        ("gray", "950") => Some("3 7 18"),
        ("zinc", "50") => Some("250 250 250"),
        ("zinc", "100") => Some("244 244 245"),
        ("zinc", "200") => Some("228 228 231"),
        ("zinc", "300") => Some("212 212 216"),
        ("zinc", "400") => Some("161 161 170"),
        ("zinc", "500") => Some("113 113 122"),
        ("zinc", "600") => Some("82 82 91"),
        ("zinc", "700") => Some("63 63 70"),
        ("zinc", "800") => Some("39 39 42"),
        ("zinc", "900") => Some("24 24 27"),
        ("zinc", "950") => Some("9 9 11"),
        ("neutral", "50") => Some("250 250 250"),
        ("neutral", "100") => Some("245 245 245"),
        ("neutral", "200") => Some("229 229 229"),
        ("neutral", "300") => Some("212 212 212"),
        ("neutral", "400") => Some("163 163 163"),
        ("neutral", "500") => Some("115 115 115"),
        ("neutral", "600") => Some("82 82 82"),
        ("neutral", "700") => Some("64 64 64"),
        ("neutral", "800") => Some("38 38 38"),
        ("neutral", "900") => Some("23 23 23"),
        ("neutral", "950") => Some("10 10 10"),
        ("stone", "50") => Some("250 250 249"),
        ("stone", "100") => Some("245 245 244"),
        ("stone", "200") => Some("231 229 228"),
        ("stone", "300") => Some("214 211 209"),
        ("stone", "400") => Some("168 162 158"),
        ("stone", "500") => Some("120 113 108"),
        ("stone", "600") => Some("87 83 78"),
        ("stone", "700") => Some("68 64 60"),
        ("stone", "800") => Some("41 37 36"),
        ("stone", "900") => Some("28 25 23"),
        ("stone", "950") => Some("12 10 9"),
        ("red", "50") => Some("254 242 242"),
        ("red", "100") => Some("254 226 226"),
        ("red", "200") => Some("254 202 202"),
        ("red", "300") => Some("252 165 165"),
        ("red", "400") => Some("248 113 113"),
        ("red", "500") => Some("239 68 68"),
        ("red", "600") => Some("220 38 38"),
        ("red", "700") => Some("185 28 28"),
        ("red", "800") => Some("153 27 27"),
        ("red", "900") => Some("127 29 29"),
        ("red", "950") => Some("69 10 10"),
        ("orange", "50") => Some("255 247 237"),
        ("orange", "100") => Some("255 237 213"),
        ("orange", "200") => Some("254 215 170"),
        ("orange", "300") => Some("253 186 116"),
        ("orange", "400") => Some("251 146 60"),
        ("orange", "500") => Some("249 115 22"),
        ("orange", "600") => Some("234 88 12"),
        ("orange", "700") => Some("194 65 12"),
        ("orange", "800") => Some("154 52 18"),
        ("orange", "900") => Some("124 45 18"),
        ("orange", "950") => Some("67 20 7"),
        ("amber", "50") => Some("255 251 235"),
        ("amber", "100") => Some("254 243 199"),
        ("amber", "200") => Some("253 230 138"),
        ("amber", "300") => Some("252 211 77"),
        ("amber", "400") => Some("251 191 36"),
        ("amber", "500") => Some("245 158 11"),
        ("amber", "600") => Some("217 119 6"),
        ("amber", "700") => Some("180 83 9"),
        ("amber", "800") => Some("146 64 14"),
        ("amber", "900") => Some("120 53 15"),
        ("amber", "950") => Some("69 26 3"),
        ("yellow", "50") => Some("254 252 232"),
        ("yellow", "100") => Some("254 249 195"),
        ("yellow", "200") => Some("254 240 138"),
        ("yellow", "300") => Some("253 224 71"),
        ("yellow", "400") => Some("250 204 21"),
        ("yellow", "500") => Some("234 179 8"),
        ("yellow", "600") => Some("202 138 4"),
        ("yellow", "700") => Some("161 98 7"),
        ("yellow", "800") => Some("133 77 14"),
        ("yellow", "900") => Some("113 63 18"),
        ("yellow", "950") => Some("66 32 6"),
        ("lime", "50") => Some("247 254 231"),
        ("lime", "100") => Some("236 252 203"),
        ("lime", "200") => Some("217 249 157"),
        ("lime", "300") => Some("190 242 100"),
        ("lime", "400") => Some("163 230 53"),
        ("lime", "500") => Some("132 204 22"),
        ("lime", "600") => Some("101 163 13"),
        ("lime", "700") => Some("77 124 15"),
        ("lime", "800") => Some("63 98 18"),
        ("lime", "900") => Some("54 83 20"),
        ("lime", "950") => Some("26 46 5"),
        ("green", "50") => Some("240 253 244"),
        ("green", "100") => Some("220 252 231"),
        ("green", "200") => Some("187 247 208"),
        ("green", "300") => Some("134 239 172"),
        ("green", "400") => Some("74 222 128"),
        ("green", "500") => Some("34 197 94"),
        ("green", "600") => Some("22 163 74"),
        ("green", "700") => Some("21 128 61"),
        ("green", "800") => Some("22 101 52"),
        ("green", "900") => Some("20 83 45"),
        ("green", "950") => Some("5 46 22"),
        ("emerald", "50") => Some("236 253 245"),
        ("emerald", "100") => Some("209 250 229"),
        ("emerald", "200") => Some("167 243 208"),
        ("emerald", "300") => Some("110 231 183"),
        ("emerald", "400") => Some("52 211 153"),
        ("emerald", "500") => Some("16 185 129"),
        ("emerald", "600") => Some("5 150 105"),
        ("emerald", "700") => Some("4 120 87"),
        ("emerald", "800") => Some("6 95 70"),
        ("emerald", "900") => Some("6 78 59"),
        ("emerald", "950") => Some("2 44 34"),
        ("teal", "50") => Some("240 253 250"),
        ("teal", "100") => Some("204 251 241"),
        ("teal", "200") => Some("153 246 228"),
        ("teal", "300") => Some("94 234 212"),
        ("teal", "400") => Some("45 212 191"),
        ("teal", "500") => Some("20 184 166"),
        ("teal", "600") => Some("13 148 136"),
        ("teal", "700") => Some("15 118 110"),
        ("teal", "800") => Some("17 94 89"),
        ("teal", "900") => Some("19 78 74"),
        ("teal", "950") => Some("4 47 46"),
        ("cyan", "50") => Some("236 254 255"),
        ("cyan", "100") => Some("207 250 254"),
        ("cyan", "200") => Some("165 243 252"),
        ("cyan", "300") => Some("103 232 249"),
        ("cyan", "400") => Some("34 211 238"),
        ("cyan", "500") => Some("6 182 212"),
        ("cyan", "600") => Some("8 145 178"),
        ("cyan", "700") => Some("14 116 144"),
        ("cyan", "800") => Some("21 94 117"),
        ("cyan", "900") => Some("22 78 99"),
        ("cyan", "950") => Some("8 51 68"),
        ("sky", "50") => Some("240 249 255"),
        ("sky", "100") => Some("224 242 254"),
        ("sky", "200") => Some("186 230 253"),
        ("sky", "300") => Some("125 211 252"),
        ("sky", "400") => Some("56 189 248"),
        ("sky", "500") => Some("14 165 233"),
        ("sky", "600") => Some("2 132 199"),
        ("sky", "700") => Some("3 105 161"),
        ("sky", "800") => Some("7 89 133"),
        ("sky", "900") => Some("12 74 110"),
        ("sky", "950") => Some("8 47 73"),
        ("blue", "50") => Some("239 246 255"),
        ("blue", "100") => Some("219 234 254"),
        ("blue", "200") => Some("191 219 254"),
        ("blue", "300") => Some("147 197 253"),
        ("blue", "400") => Some("96 165 250"),
        ("blue", "500") => Some("59 130 246"),
        ("blue", "600") => Some("37 99 235"),
        ("blue", "700") => Some("29 78 216"),
        ("blue", "800") => Some("30 64 175"),
        ("blue", "900") => Some("30 58 138"),
        ("blue", "950") => Some("23 37 84"),
        ("indigo", "50") => Some("238 242 255"),
        ("indigo", "100") => Some("224 231 255"),
        ("indigo", "200") => Some("199 210 254"),
        ("indigo", "300") => Some("165 180 252"),
        ("indigo", "400") => Some("129 140 248"),
        ("indigo", "500") => Some("99 102 241"),
        ("indigo", "600") => Some("79 70 229"),
        ("indigo", "700") => Some("67 56 202"),
        ("indigo", "800") => Some("55 48 163"),
        ("indigo", "900") => Some("49 46 129"),
        ("indigo", "950") => Some("30 27 75"),
        ("violet", "50") => Some("245 243 255"),
        ("violet", "100") => Some("237 233 254"),
        ("violet", "200") => Some("221 214 254"),
        ("violet", "300") => Some("196 181 253"),
        ("violet", "400") => Some("167 139 250"),
        ("violet", "500") => Some("139 92 246"),
        ("violet", "600") => Some("124 58 237"),
        ("violet", "700") => Some("109 40 217"),
        ("violet", "800") => Some("91 33 182"),
        ("violet", "900") => Some("76 29 149"),
        ("violet", "950") => Some("46 16 101"),
        ("purple", "50") => Some("250 245 255"),
        ("purple", "100") => Some("243 232 255"),
        ("purple", "200") => Some("233 213 255"),
        ("purple", "300") => Some("216 180 254"),
        ("purple", "400") => Some("192 132 252"),
        ("purple", "500") => Some("168 85 247"),
        ("purple", "600") => Some("147 51 234"),
        ("purple", "700") => Some("126 34 206"),
        ("purple", "800") => Some("107 33 168"),
        ("purple", "900") => Some("88 28 135"),
        ("purple", "950") => Some("59 7 100"),
        ("fuchsia", "50") => Some("253 244 255"),
        ("fuchsia", "100") => Some("250 232 255"),
        ("fuchsia", "200") => Some("245 208 254"),
        ("fuchsia", "300") => Some("240 171 252"),
        ("fuchsia", "400") => Some("232 121 249"),
        ("fuchsia", "500") => Some("217 70 239"),
        ("fuchsia", "600") => Some("192 38 211"),
        ("fuchsia", "700") => Some("162 28 175"),
        ("fuchsia", "800") => Some("134 25 143"),
        ("fuchsia", "900") => Some("112 26 117"),
        ("fuchsia", "950") => Some("74 4 78"),
        ("pink", "50") => Some("253 242 248"),
        ("pink", "100") => Some("252 231 243"),
        ("pink", "200") => Some("251 207 232"),
        ("pink", "300") => Some("249 168 212"),
        ("pink", "400") => Some("244 114 182"),
        ("pink", "500") => Some("236 72 153"),
        ("pink", "600") => Some("219 39 119"),
        ("pink", "700") => Some("190 24 93"),
        ("pink", "800") => Some("157 23 77"),
        ("pink", "900") => Some("131 24 67"),
        ("pink", "950") => Some("80 7 36"),
        ("rose", "50") => Some("255 241 242"),
        ("rose", "100") => Some("255 228 230"),
        ("rose", "200") => Some("254 205 211"),
        ("rose", "300") => Some("253 164 175"),
        ("rose", "400") => Some("251 113 133"),
        ("rose", "500") => Some("244 63 94"),
        ("rose", "600") => Some("225 29 72"),
        ("rose", "700") => Some("190 18 60"),
        ("rose", "800") => Some("159 18 57"),
        ("rose", "900") => Some("136 19 55"),
        ("rose", "950") => Some("76 5 25"),
        _ => None,
    }
}

fn opacity_value(raw_value: &str) -> Option<String> {
    if let Some(value) = arbitrary_value(raw_value) {
        return Some(value);
    }
    if let Some(value) = custom_property_var_value(raw_value) {
        return Some(value);
    }
    let value = raw_value.parse::<f32>().ok()?;
    if !(0.0..=100.0).contains(&value) {
        return None;
    }
    Some(format!("{}%", trim_float(value)))
}

fn color_alpha_value(raw_value: &str) -> Option<String> {
    if let Some(value) = arbitrary_value(raw_value) {
        return Some(value);
    }
    if let Some(value) = custom_property_var_value(raw_value) {
        return Some(value);
    }
    let value = raw_value.parse::<f32>().ok()?;
    if !(0.0..=100.0).contains(&value) {
        return None;
    }
    Some(trim_float(value / 100.0))
}

fn is_opaque_color_alpha_token(raw_value: &str) -> bool {
    raw_value
        .trim()
        .parse::<f32>()
        .is_ok_and(|value| (value - 100.0).abs() < f32::EPSILON)
}

fn border_size_value(raw_value: &str) -> Option<String> {
    if let Some(value) = arbitrary_value(raw_value) {
        return Some(value);
    }
    if let Some(value) = typed_custom_property_var_value(raw_value, "length") {
        return Some(value);
    }
    if let Some(value) = custom_property_var_value(raw_value) {
        return Some(value);
    }
    match raw_value {
        "0" | "1" | "2" | "4" | "8" => Some(format!("{}px", raw_value)),
        _ => None,
    }
}

fn text_shadow_utility(class_name: &str) -> Option<String> {
    let raw_value = class_name.strip_prefix("text-shadow")?;
    if let Some(value) = text_shadow_color_value(raw_value) {
        return Some(format!("--tw-text-shadow-color: {}", value));
    }

    let (raw_value, alpha_modifier) = split_text_shadow_opacity_modifier(raw_value);
    let alpha = match alpha_modifier {
        Some(raw_alpha) => Some(text_shadow_alpha_value(raw_alpha)?),
        None => None,
    };
    let value = text_shadow_value(raw_value, alpha.as_ref())?;
    if let Some(alpha) = alpha {
        if value.contains("BASE|") || value.contains("SUPPORTS|") {
            return Some(value);
        }
        return Some(format!(
            "--tw-text-shadow-alpha: {}; text-shadow: {}",
            alpha.declaration_value, value
        ));
    }

    Some(format!("text-shadow: {}", value))
}

fn text_shadow_color_value(raw_value: &str) -> Option<String> {
    let raw_value = raw_value.strip_prefix('-')?;

    if let Some(value) = typed_custom_property_var_value(raw_value, "color") {
        return Some(value);
    }

    if raw_value.starts_with("[color:") {
        return color_value(raw_value);
    }

    if raw_value.starts_with('[') || raw_value.starts_with('(') {
        return None;
    }

    color_value(raw_value)
}

fn split_text_shadow_opacity_modifier(raw_value: &str) -> (&str, Option<&str>) {
    split_text_size_line_height_modifier(raw_value)
}

fn text_shadow_alpha_value(raw_value: &str) -> Option<TextShadowAlpha> {
    if let Some(value) = arbitrary_value(raw_value) {
        let value = value.trim();
        if value.is_empty() {
            return None;
        }
        return Some(TextShadowAlpha {
            declaration_value: value.to_string(),
            fallback_value: text_shadow_alpha_percentage(value)?,
        });
    }

    if let Some(value) = custom_property_var_value(raw_value) {
        return Some(TextShadowAlpha {
            declaration_value: value.clone(),
            fallback_value: value,
        });
    }

    if let Some(percent) = raw_value.strip_suffix('%') {
        let value = percent.parse::<f32>().ok()?;
        if !(0.0..=100.0).contains(&value) {
            return None;
        }
        let value = format!("{}%", trim_float(value));
        return Some(TextShadowAlpha {
            declaration_value: value.clone(),
            fallback_value: value,
        });
    }

    let value = raw_value.parse::<f32>().ok()?;
    if !(0.0..=100.0).contains(&value) {
        return None;
    }
    let value = format!("{}%", trim_float(value));
    Some(TextShadowAlpha {
        declaration_value: value.clone(),
        fallback_value: value,
    })
}

fn text_shadow_alpha_percentage(raw_value: &str) -> Option<String> {
    let raw_value = raw_value.trim();
    if raw_value.starts_with("var(")
        || raw_value.starts_with("calc(")
        || raw_value.starts_with("clamp(")
    {
        return Some(raw_value.to_string());
    }

    if let Some(percent) = raw_value.strip_suffix('%') {
        let value = percent.parse::<f32>().ok()?;
        return (0.0..=100.0)
            .contains(&value)
            .then(|| format!("{}%", trim_float(value)));
    }

    let value = raw_value.parse::<f32>().ok()?;
    if !(0.0..=100.0).contains(&value) {
        return None;
    }
    let percent = if value <= 1.0 { value * 100.0 } else { value };
    Some(format!("{}%", trim_float(percent)))
}

fn text_shadow_color_fallback(fallback: &str, alpha: Option<&TextShadowAlpha>) -> String {
    match alpha {
        Some(alpha) => text_shadow_alpha_color(fallback, &alpha.fallback_value),
        None => format!("var(--tw-text-shadow-color, {fallback})"),
    }
}

fn text_shadow_value(raw_value: &str, alpha: Option<&TextShadowAlpha>) -> Option<String> {
    if raw_value.is_empty() {
        return Some(format!(
            "0px 1px 2px {}",
            text_shadow_color_fallback("rgb(0 0 0 / 0.15)", alpha)
        ));
    }

    let raw_value = raw_value.strip_prefix('-')?;
    if let Some(value) = arbitrary_value(raw_value) {
        let value = value
            .trim()
            .strip_prefix("shadow:")
            .unwrap_or(value.trim())
            .trim();
        if value.is_empty() {
            return None;
        }
        return match alpha {
            Some(alpha) => text_shadow_arbitrary_value_with_alpha(value, alpha),
            None => Some(value.to_string()),
        };
    }
    if let Some(variable) = raw_value
        .strip_prefix('(')
        .and_then(|raw| raw.strip_suffix(')'))
    {
        if is_custom_property_name(variable) {
            return Some(format!("var({})", variable));
        }
        return None;
    }

    Some(match raw_value {
        "2xs" => format!(
            "0px 1px 0px {}",
            text_shadow_color_fallback("rgb(0 0 0 / 0.15)", alpha)
        ),
        "xs" => format!(
            "0px 1px 1px {}",
            text_shadow_color_fallback("rgb(0 0 0 / 0.2)", alpha)
        ),
        "sm" => {
            let color = text_shadow_color_fallback("rgb(0 0 0 / 0.075)", alpha);
            format!("0px 1px 0px {color}, 0px 1px 1px {color}, 0px 2px 2px {color}")
        }
        "md" => {
            let color = text_shadow_color_fallback("rgb(0 0 0 / 0.1)", alpha);
            format!("0px 1px 1px {color}, 0px 2px 2px {color}, 0px 4px 4px {color}")
        }
        "lg" => {
            let color = text_shadow_color_fallback("rgb(0 0 0 / 0.1)", alpha);
            format!("0px 1px 2px {color}, 0px 3px 2px {color}, 0px 4px 8px {color}")
        }
        "none" => "none".to_string(),
        _ => return None,
    })
}

fn text_shadow_arbitrary_value_with_alpha(value: &str, alpha: &TextShadowAlpha) -> Option<String> {
    let tokens = split_top_level_whitespace(value);
    if tokens.is_empty() {
        return None;
    }

    if tokens.len() >= 2 {
        if let Some(color) = tokens
            .last()
            .filter(|token| is_text_shadow_color_token(token))
        {
            let offsets = tokens[..tokens.len() - 1].join(" ");
            return Some(format!(
                "{} {}",
                offsets,
                text_shadow_alpha_color(color, &alpha.fallback_value)
            ));
        }
    }

    Some(format!(
        "INLINE_SUPPORTS|{}|--tw-text-shadow-alpha: {}; text-shadow: {} var(--tw-text-shadow-color, currentcolor)|text-shadow: {} var(--tw-text-shadow-color, color-mix(in oklab, currentcolor {}, transparent))",
        COLOR_MIX_SUPPORTS_CONDITION, alpha.declaration_value, value, value, alpha.fallback_value
    ))
}

fn text_shadow_alpha_color(color: &str, alpha: &str) -> String {
    format!("var(--tw-text-shadow-color, oklab(from {color} l a b / {alpha}))")
}

fn is_text_shadow_color_token(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower == "currentcolor"
        || lower == "transparent"
        || lower == "inherit"
        || value.starts_with('#')
        || lower.starts_with("var(")
        || lower.starts_with("rgb(")
        || lower.starts_with("rgba(")
        || lower.starts_with("hsl(")
        || lower.starts_with("hsla(")
        || lower.starts_with("oklab(")
        || lower.starts_with("oklch(")
        || lower.starts_with("color-mix(")
}

fn scale_value(raw_value: &str) -> Option<String> {
    if let Some(value) = arbitrary_value(raw_value) {
        return Some(value);
    }
    if let Some(value) = custom_property_var_value(raw_value) {
        return Some(value);
    }
    let value = raw_value.parse::<f32>().ok()?;
    Some(format!("{}%", trim_float(value)))
}

fn signed_scale_value(raw_value: &str, negative: bool) -> Option<String> {
    let value = scale_value(raw_value)?;
    if negative && value != "0" {
        return Some(negative_scale_value(&value));
    }
    Some(value)
}

fn negative_scale_value(value: &str) -> String {
    if value.ends_with('%') {
        return format!("calc({value} * -1)");
    }
    value
        .parse::<f32>()
        .ok()
        .map(|value| trim_float(-value))
        .unwrap_or_else(|| format!("calc({value} * -1)"))
}

fn arbitrary_value(raw_value: &str) -> Option<String> {
    let inner = raw_value.strip_prefix('[')?.strip_suffix(']')?;
    safe_arbitrary_css_value(inner)
}

fn safe_arbitrary_css_value(raw_value: &str) -> Option<String> {
    let decoded = decode_arbitrary(raw_value);
    is_safe_css_value(&decoded).then_some(decoded)
}

fn is_safe_css_value(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    !value.is_empty()
        && !value.chars().any(|ch| matches!(ch, '{' | '}' | ';'))
        && !lower.contains("javascript:")
        && !lower.contains("expression(")
        && !lower.contains("@import")
        && !lower.contains("</")
}

fn decode_arbitrary(value: &str) -> String {
    value.replace('_', " ")
}

fn split_top_level_whitespace(value: &str) -> Vec<&str> {
    let mut tokens = Vec::new();
    let mut start = None;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut quote = None;
    let mut escaped = false;

    for (index, ch) in value.char_indices() {
        if let Some(active_quote) = quote {
            if start.is_none() {
                start = Some(index);
            }
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == active_quote {
                quote = None;
            }
            continue;
        }

        match ch {
            '"' | '\'' => {
                if start.is_none() {
                    start = Some(index);
                }
                quote = Some(ch);
            }
            '(' => {
                paren_depth = paren_depth.saturating_add(1);
                if start.is_none() {
                    start = Some(index);
                }
            }
            ')' => {
                paren_depth = paren_depth.saturating_sub(1);
                if start.is_none() {
                    start = Some(index);
                }
            }
            '[' => {
                bracket_depth = bracket_depth.saturating_add(1);
                if start.is_none() {
                    start = Some(index);
                }
            }
            ']' => {
                bracket_depth = bracket_depth.saturating_sub(1);
                if start.is_none() {
                    start = Some(index);
                }
            }
            ch if ch.is_whitespace() && paren_depth == 0 && bracket_depth == 0 => {
                if let Some(token_start) = start.take() {
                    if token_start < index {
                        tokens.push(&value[token_start..index]);
                    }
                }
            }
            _ => {
                if start.is_none() {
                    start = Some(index);
                }
            }
        }
    }

    if let Some(token_start) = start {
        if token_start < value.len() {
            tokens.push(&value[token_start..]);
        }
    }

    tokens
}

fn join_properties(properties: &[&str], value: &str) -> String {
    let mut css = String::new();
    for (idx, property) in properties.iter().enumerate() {
        if idx > 0 {
            css.push_str("; ");
        }
        css.push_str(property);
        css.push_str(": ");
        css.push_str(value);
    }
    css
}

fn bounded_usize(raw_value: &str, min: usize, max: usize) -> Option<usize> {
    let value = raw_value.parse::<usize>().ok()?;
    (min..=max).contains(&value).then_some(value)
}

fn trim_float(value: f32) -> String {
    let mut out = format!("{:.4}", value);
    while out.contains('.') && out.ends_with('0') {
        out.pop();
    }
    if out.ends_with('.') {
        out.pop();
    }
    out
}

fn rgb(channels: &str, alpha: Option<String>) -> String {
    match alpha {
        Some(alpha) => format!("rgb({} / {})", channels, alpha),
        None => format!("rgb({})", channels),
    }
}

fn is_hex_color(raw_value: &str) -> bool {
    matches!(raw_value.len(), 3 | 4 | 6 | 8) && raw_value.chars().all(|ch| ch.is_ascii_hexdigit())
}

fn hex_rgb(raw_value: &str, alpha: Option<String>) -> Option<String> {
    let hex = raw_value.strip_prefix('#').unwrap_or(raw_value);
    if !is_hex_color(hex) {
        return None;
    }
    let parse_pair = |pair: &str| u8::from_str_radix(pair, 16).ok();
    let (r, g, b, a) = match hex.len() {
        3 | 4 => {
            let r = parse_pair(&hex[0..1].repeat(2))?;
            let g = parse_pair(&hex[1..2].repeat(2))?;
            let b = parse_pair(&hex[2..3].repeat(2))?;
            let a = if hex.len() == 4 {
                Some(parse_pair(&hex[3..4].repeat(2))?)
            } else {
                None
            };
            (r, g, b, a)
        }
        6 | 8 => {
            let r = parse_pair(&hex[0..2])?;
            let g = parse_pair(&hex[2..4])?;
            let b = parse_pair(&hex[4..6])?;
            let a = if hex.len() == 8 {
                Some(parse_pair(&hex[6..8])?)
            } else {
                None
            };
            (r, g, b, a)
        }
        _ => return None,
    };
    let alpha = alpha.or_else(|| a.map(|a| trim_float(a as f32 / 255.0)));
    Some(rgb(&format!("{} {} {}", r, g, b), alpha))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_spacing_and_layout_utilities() {
        assert_eq!(
            generate_utility_css("p-4").unwrap(),
            "padding: calc(var(--spacing) * 4)"
        );
        assert_eq!(
            generate_utility_css("mx-auto").unwrap(),
            "margin-inline: auto"
        );
        assert_eq!(
            generate_utility_css("-mt-2").unwrap(),
            "margin-top: calc(var(--spacing) * -2)"
        );
        assert_eq!(
            generate_utility_css("grid-cols-3").unwrap(),
            "grid-template-columns: repeat(3, minmax(0, 1fr))"
        );
    }

    #[test]
    fn generates_colors_and_arbitrary_values() {
        assert_eq!(
            generate_utility_css("bg-blue-500").unwrap(),
            "background-color: rgb(59 130 246)"
        );
        assert_eq!(
            generate_utility_css("text-slate-900/80").unwrap(),
            "color: rgb(15 23 42 / 0.8)"
        );
        assert_eq!(
            generate_utility_css("w-[min(100%,_42rem)]").unwrap(),
            "width: min(100%, 42rem)"
        );
        assert_eq!(
            generate_utility_css("[scroll-margin-top:4rem]").unwrap(),
            "scroll-margin-top: 4rem"
        );
    }

    #[test]
    fn tailwind_v43_neutral_adjacent_palettes_feed_shared_color_utilities() {
        let cases = [
            ("bg-mauve-500", "background-color: var(--color-mauve-500)"),
            (
                "text-mauve-500/50",
                "color: color-mix(in oklab, var(--color-mauve-500) 50%, transparent)",
            ),
            ("border-olive-500", "border-color: var(--color-olive-500)"),
            ("outline-taupe-700", "outline-color: var(--color-taupe-700)"),
            ("ring-mist-400", "--tw-ring-color: var(--color-mist-400)"),
            (
                "decoration-mauve-300",
                "text-decoration-color: var(--color-mauve-300)",
            ),
            ("fill-olive-600", "fill: var(--color-olive-600)"),
            (
                "stroke-mist-700/75",
                "stroke: color-mix(in oklab, var(--color-mist-700) 75%, transparent)",
            ),
            (
                "placeholder-mauve-500",
                "NEST|::placeholder|color: var(--color-mauve-500)",
            ),
            (
                "placeholder-olive-500/50",
                "color: color-mix(in oklab, var(--color-olive-500) 50%, transparent)",
            ),
            ("placeholder-[#243c5a]", "NEST|::placeholder|color: #243c5a"),
            (
                "placeholder-(color:--dx-placeholder)/(--dx-alpha)",
                "NEST|::placeholder|color: color-mix(in oklab, var(--dx-placeholder) var(--dx-alpha), transparent)",
            ),
            (
                "from-taupe-200",
                "--tw-gradient-from: var(--color-taupe-200); --tw-gradient-to: rgb(255 255 255 / 0); --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to)",
            ),
            (
                "via-mauve-400/40",
                "--tw-gradient-via: color-mix(in oklab, var(--color-mauve-400) 40%, transparent)",
            ),
            ("to-olive-950", "--tw-gradient-to: var(--color-olive-950)"),
            (
                "scrollbar-thumb-mist-500",
                "--tw-scrollbar-thumb: var(--color-mist-500); scrollbar-color: var(--tw-scrollbar-thumb) var(--tw-scrollbar-track)",
            ),
            (
                "scrollbar-track-taupe-100",
                "--tw-scrollbar-track: var(--color-taupe-100); scrollbar-color: var(--tw-scrollbar-thumb) var(--tw-scrollbar-track)",
            ),
            (
                "text-shadow-mauve-500/50",
                "--tw-text-shadow-color: color-mix(in oklab, var(--color-mauve-500) 50%, transparent)",
            ),
            (
                "shadow-mauve-500",
                "--tw-shadow-color: color-mix(in oklab, var(--color-mauve-500) var(--tw-shadow-alpha), transparent)",
            ),
            (
                "shadow-mauve-500/50",
                "--tw-shadow-color: color-mix(in oklab, color-mix(in oklab, var(--color-mauve-500) 50%, transparent) var(--tw-shadow-alpha), transparent)",
            ),
            (
                "drop-shadow-mauve-500/50",
                "--tw-drop-shadow-color: color-mix(in oklab, color-mix(in oklab, var(--color-mauve-500) 50%, transparent) var(--tw-drop-shadow-alpha), transparent)",
            ),
            (
                "inset-shadow-olive-500",
                "--tw-inset-shadow-color: color-mix(in oklab, var(--color-olive-500) var(--tw-inset-shadow-alpha), transparent)",
            ),
            (
                "inset-shadow-olive-500/50",
                "--tw-inset-shadow-color: color-mix(in oklab, color-mix(in oklab, var(--color-olive-500) 50%, transparent) var(--tw-inset-shadow-alpha), transparent)",
            ),
            (
                "inset-ring",
                "--tw-inset-ring-shadow: inset 0 0 0 1px var(--tw-inset-ring-color, currentcolor); box-shadow: var(--tw-inset-shadow), var(--tw-inset-ring-shadow), var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow)",
            ),
            (
                "inset-ring-2",
                "--tw-inset-ring-shadow: inset 0 0 0 2px var(--tw-inset-ring-color, currentcolor); box-shadow: var(--tw-inset-shadow), var(--tw-inset-ring-shadow), var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow)",
            ),
            (
                "inset-ring-mist-500/50",
                "--tw-inset-ring-color: color-mix(in oklab, var(--color-mist-500) 50%, transparent)",
            ),
            (
                "ring-offset-taupe-500/40",
                "--tw-ring-offset-color: color-mix(in oklab, var(--color-taupe-500) 40%, transparent)",
            ),
            (
                "bg-mist-500/[71.37%]",
                "background-color: color-mix(in oklab, var(--color-mist-500) 71.37%, transparent)",
            ),
            (
                "border-taupe-500/(--dx-alpha)",
                "border-color: color-mix(in oklab, var(--color-taupe-500) var(--dx-alpha), transparent)",
            ),
        ];

        for (class_name, expected_css) in cases {
            let css = generate_utility_css(class_name).unwrap_or_else(|| {
                panic!("{class_name} should resolve through the Tailwind v4.3 palette table")
            });
            assert!(
                css.contains(expected_css),
                "{class_name} should contain {expected_css:?}:\n{css}"
            );
        }

        for palette in ["mauve", "olive", "mist", "taupe"] {
            for shade in ["50", "500", "950"] {
                let class_name = format!("bg-{palette}-{shade}");
                assert!(
                    generate_utility_css(&class_name).is_some(),
                    "{class_name} should be present in the complete v4.3 neutral-adjacent palette"
                );
            }
        }
    }

    #[test]
    fn tailwind_v43_neutral_adjacent_palettes_feed_directional_border_color_utilities() {
        let cases = [
            (
                "border-x-mauve-500",
                "border-inline-color: var(--color-mauve-500)",
            ),
            (
                "border-y-olive-500/50",
                "border-block-color: color-mix(in oklab, var(--color-olive-500) 50%, transparent)",
            ),
            (
                "border-t-mist-300",
                "border-top-color: var(--color-mist-300)",
            ),
            (
                "border-r-taupe-700",
                "border-right-color: var(--color-taupe-700)",
            ),
            (
                "border-b-mauve-950",
                "border-bottom-color: var(--color-mauve-950)",
            ),
            (
                "border-l-olive-600/[71.37%]",
                "border-left-color: color-mix(in oklab, var(--color-olive-600) 71.37%, transparent)",
            ),
            ("border-t-[#243c5a]", "border-top-color: #243c5a"),
            (
                "border-r-[#243c5a]/50",
                "border-right-color: color-mix(in oklab, #243c5a 50%, transparent)",
            ),
            (
                "border-x-(color:--dx-border)",
                "border-inline-color: var(--dx-border)",
            ),
            (
                "border-y-(color:--dx-border-alpha)/(--dx-alpha)",
                "border-block-color: color-mix(in oklab, var(--dx-border-alpha) var(--dx-alpha), transparent)",
            ),
        ];

        for (class_name, expected_css) in cases {
            let css = generate_utility_css(class_name).unwrap_or_else(|| {
                panic!("{class_name} should resolve as a directional border color utility")
            });
            assert!(
                css.contains(expected_css),
                "{class_name} should contain {expected_css:?}:\n{css}"
            );
        }
    }

    #[test]
    fn generates_token_aware_color_utilities() {
        assert_eq!(
            generate_utility_css("bg-background").unwrap(),
            "background-color: hsl(var(--background))"
        );
        assert_eq!(
            generate_utility_css("text-muted-foreground").unwrap(),
            "color: hsl(var(--muted-foreground))"
        );
        assert_eq!(
            generate_utility_css("border-border").unwrap(),
            "border-color: hsl(var(--border))"
        );
        assert_eq!(
            generate_utility_css("border-input").unwrap(),
            "border-color: hsl(var(--input))"
        );
        assert_eq!(
            generate_utility_css("ring-ring").unwrap(),
            "--tw-ring-color: hsl(var(--ring))"
        );
        assert_eq!(
            generate_utility_css("bg-primary/90").unwrap(),
            "background-color: hsl(var(--primary) / 0.9)"
        );
        assert_eq!(
            generate_utility_css("placeholder-muted-foreground").unwrap(),
            "NEST|::placeholder|color: hsl(var(--muted-foreground))"
        );
        assert_eq!(
            generate_utility_css("text-current/70").unwrap(),
            "color: color-mix(in oklab, currentColor 70%, transparent)"
        );
        assert_eq!(
            generate_utility_css("border-current/20").unwrap(),
            "border-color: color-mix(in oklab, currentColor 20%, transparent)"
        );
        assert_eq!(
            generate_utility_css("bg-[var(--dx-scene-chip)]").unwrap(),
            "background-color: var(--dx-scene-chip)"
        );
        assert_eq!(
            generate_utility_css("text-[color:var(--dx-scene-muted)]").unwrap(),
            "color: var(--dx-scene-muted)"
        );
        assert_eq!(
            generate_utility_css("text-[var(--dx-scene-muted)]").unwrap(),
            "color: var(--dx-scene-muted)"
        );
        assert_eq!(
            generate_utility_css("border-[var(--dx-scene-border)]").unwrap(),
            "border-color: var(--dx-scene-border)"
        );
        assert_eq!(
            generate_utility_css("border-[var(--dx-border-width)]").unwrap(),
            "border-style: var(--tw-border-style); border-width: var(--dx-border-width)"
        );
        assert_eq!(
            generate_utility_css("shadow-[var(--dx-scene-shadow)]").unwrap(),
            "box-shadow: var(--dx-scene-shadow)"
        );
        assert_eq!(
            generate_utility_css("bg-token(surface)").unwrap(),
            "background-color: hsl(var(--surface))"
        );
        assert_eq!(
            generate_utility_css("text-token(foreground)").unwrap(),
            "color: hsl(var(--foreground))"
        );
        assert_eq!(
            generate_utility_css("border-token(border)").unwrap(),
            "border-color: hsl(var(--border))"
        );
        assert_eq!(
            generate_utility_css("ring-token(ring)").unwrap(),
            "--tw-ring-color: hsl(var(--ring))"
        );
        assert_eq!(
            generate_utility_css("bg-token(surface)/50").unwrap(),
            "background-color: hsl(var(--surface) / 0.5)"
        );
        assert_eq!(
            generate_utility_css("bg-chart-1").unwrap(),
            "background-color: hsl(var(--chart-1))"
        );
        assert_eq!(
            generate_utility_css("stroke-chart-5/50").unwrap(),
            "stroke: hsl(var(--chart-5) / 0.5)"
        );
        assert_eq!(
            generate_utility_css("bg-sidebar").unwrap(),
            "background-color: hsl(var(--sidebar))"
        );
        assert_eq!(
            generate_utility_css("text-sidebar-foreground").unwrap(),
            "color: hsl(var(--sidebar-foreground))"
        );
        assert_eq!(
            generate_utility_css("border-sidebar-border").unwrap(),
            "border-color: hsl(var(--sidebar-border))"
        );
        assert_eq!(
            generate_utility_css("ring-sidebar-ring").unwrap(),
            "--tw-ring-color: hsl(var(--sidebar-ring))"
        );
        assert!(generate_utility_css("bg-token(does-not-exist)").is_none());
    }

    #[test]
    fn generates_numeric_font_variant_utilities() {
        assert_eq!(
            generate_utility_css("normal-nums").unwrap(),
            "font-variant-numeric: normal"
        );
        assert!(
            generate_utility_css("ordinal")
                .unwrap()
                .contains("--tw-ordinal: ordinal; font-variant-numeric:")
        );
        assert!(
            generate_utility_css("tabular-nums")
                .unwrap()
                .contains("--tw-numeric-spacing: tabular-nums; font-variant-numeric:")
        );
        assert!(
            generate_utility_css("diagonal-fractions")
                .unwrap()
                .contains("--tw-numeric-fraction: diagonal-fractions; font-variant-numeric:")
        );
    }

    #[test]
    fn tailwind_v42_font_feature_utilities_generate_css() {
        assert_eq!(
            generate_utility_css("font-features-['smcp','onum']").unwrap(),
            "font-feature-settings: 'smcp','onum'"
        );
        assert_eq!(
            generate_utility_css("font-features-(--dx-font-features)").unwrap(),
            "font-feature-settings: var(--dx-font-features)"
        );
        assert!(generate_utility_css("font-features-[bad;value]").is_none());
        assert!(generate_utility_css("font-features-smcp").is_none());
    }

    #[test]
    fn generates_border_side_shorthands_before_width_prefixes() {
        assert_eq!(
            generate_utility_css("border-b").unwrap(),
            "border-bottom-style: var(--tw-border-style); border-bottom-width: 1px"
        );
        assert_eq!(
            generate_utility_css("border-x").unwrap(),
            "border-inline-style: var(--tw-border-style); border-inline-width: 1px"
        );
    }

    #[test]
    fn generates_effect_and_transform_utilities() {
        assert!(
            generate_utility_css("shadow-lg")
                .unwrap()
                .contains("box-shadow")
        );
        assert!(
            generate_utility_css("text-shadow-sm")
                .unwrap()
                .contains("text-shadow")
        );
        assert_eq!(
            generate_utility_css("text-shadow-none").unwrap(),
            "text-shadow: none"
        );
        assert_eq!(
            generate_utility_css("text-shadow-(--dx-copy-shadow)").unwrap(),
            "text-shadow: var(--dx-copy-shadow)"
        );
        assert!(
            generate_utility_css("ring-2")
                .unwrap()
                .contains("--tw-ring-offset-width")
        );
        assert_eq!(
            generate_utility_css("outline-2").unwrap(),
            "outline-width: 2px"
        );
        assert_eq!(
            generate_utility_css("outline-dashed").unwrap(),
            "outline-style: dashed"
        );
        assert_eq!(
            generate_utility_css("ring-inset").unwrap(),
            "--tw-ring-inset: inset"
        );
        assert_eq!(
            generate_utility_css("ring-offset-2").unwrap(),
            "--tw-ring-offset-width: 2px"
        );
        assert!(
            generate_utility_css("ring-2")
                .unwrap()
                .contains("var(--tw-ring-inset,)")
        );
        assert!(
            generate_utility_css("-translate-y-1")
                .unwrap()
                .contains("--tw-translate-y: calc(var(--spacing) * -1)")
        );
        assert!(
            generate_utility_css("transform-gpu")
                .unwrap()
                .contains("transform: translateZ(0)")
        );
        assert!(
            generate_utility_css("transform-cpu")
                .unwrap()
                .contains("transform: var(--tw-rotate-x,)")
        );
    }

    #[test]
    fn high_usage_tailwind_layout_utilities_generate_css() {
        assert_eq!(
            generate_utility_css("inline-table").unwrap(),
            "display: inline-table"
        );
        assert_eq!(generate_utility_css("float-right").unwrap(), "float: right");
        assert_eq!(
            generate_utility_css("aspect-video").unwrap(),
            "aspect-ratio: 16 / 9"
        );
        assert_eq!(
            generate_utility_css("basis-1/2").unwrap(),
            "flex-basis: calc(1 / 2 * 100%)"
        );
        assert_eq!(generate_utility_css("flex-1").unwrap(), "flex: 1");
        assert_eq!(generate_utility_css("flex-auto").unwrap(), "flex: auto");
        assert_eq!(
            generate_utility_css("flex-initial").unwrap(),
            "flex: 0 auto"
        );
        assert_eq!(generate_utility_css("w-screen").unwrap(), "width: 100vw");
        assert_eq!(generate_utility_css("h-screen").unwrap(), "height: 100vh");
        assert_eq!(
            generate_utility_css("inline-1/2").unwrap(),
            "inline-size: calc(1 / 2 * 100%)"
        );
        assert_eq!(
            generate_utility_css("inline-(--dx-inline-size)").unwrap(),
            "inline-size: var(--dx-inline-size)"
        );
        assert_eq!(
            generate_utility_css("min-inline-3xs").unwrap(),
            "min-inline-size: var(--container-3xs)"
        );
        assert_eq!(
            generate_utility_css("max-inline-[42rem]").unwrap(),
            "max-inline-size: 42rem"
        );
        assert_eq!(
            generate_utility_css("block-screen").unwrap(),
            "block-size: 100vh"
        );
        assert_eq!(
            generate_utility_css("max-block-dvh").unwrap(),
            "max-block-size: 100dvh"
        );
        assert!(generate_utility_css("size-3xs").is_none());
        assert!(generate_utility_css("block-3xs").is_none());
        assert!(generate_utility_css("h-3xs").is_none());
        assert_eq!(
            generate_utility_css("auto-cols-fr").unwrap(),
            "grid-auto-columns: minmax(0, 1fr)"
        );
        assert_eq!(
            generate_utility_css("col-start-2").unwrap(),
            "grid-column-start: 2"
        );
        assert_eq!(
            generate_utility_css("overscroll-y-contain").unwrap(),
            "overscroll-behavior-y: contain"
        );
        assert_eq!(
            generate_utility_css("touch-pan-left").unwrap(),
            "--tw-pan-x: pan-left; touch-action: var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,)"
        );
        assert_eq!(
            generate_utility_css("touch-pinch-zoom").unwrap(),
            "--tw-pinch-zoom: pinch-zoom; touch-action: var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,)"
        );
        assert_eq!(
            generate_utility_css("scroll-mt-4").unwrap(),
            "scroll-margin-top: calc(var(--spacing) * 4)"
        );
        assert_eq!(
            generate_utility_css("field-sizing-content").unwrap(),
            "field-sizing: content"
        );
        assert_eq!(
            generate_utility_css("field-sizing-fixed").unwrap(),
            "field-sizing: fixed"
        );
        assert_eq!(generate_utility_css("columns-3").unwrap(), "columns: 3");
        assert_eq!(
            generate_utility_css("columns-auto").unwrap(),
            "columns: auto"
        );
        assert_eq!(
            generate_utility_css("columns-lg").unwrap(),
            "columns: 32rem"
        );
        assert_eq!(
            generate_utility_css("columns-[14rem]").unwrap(),
            "columns: 14rem"
        );
        assert_eq!(
            generate_utility_css("columns-(--dx-column-width)").unwrap(),
            "columns: var(--dx-column-width)"
        );
        assert!(generate_utility_css("columns-13").is_none());
        assert_eq!(
            generate_utility_css("break-before-page").unwrap(),
            "break-before: page"
        );
        assert_eq!(
            generate_utility_css("break-after-avoid-page").unwrap(),
            "break-after: avoid-page"
        );
        assert_eq!(
            generate_utility_css("break-inside-avoid-column").unwrap(),
            "break-inside: avoid-column"
        );
        assert_eq!(
            generate_utility_css("break-inside-avoid").unwrap(),
            "page-break-inside: avoid; break-inside: avoid"
        );
        assert!(generate_utility_css("break-inside-left").is_none());
        assert_eq!(
            generate_utility_css("box-decoration-clone").unwrap(),
            "-webkit-box-decoration-break: clone; box-decoration-break: clone"
        );
        assert_eq!(
            generate_utility_css("box-decoration-slice").unwrap(),
            "-webkit-box-decoration-break: slice; box-decoration-break: slice"
        );
    }

    #[test]
    fn tailwind_v43_interactivity_additions_generate_css() {
        assert_eq!(
            generate_utility_css("scheme-light-dark").unwrap(),
            "color-scheme: light dark"
        );
        assert_eq!(
            generate_utility_css("scheme-only-dark").unwrap(),
            "color-scheme: only dark"
        );
        assert_eq!(
            generate_utility_css("scrollbar-thin").unwrap(),
            "scrollbar-width: thin"
        );
        assert_eq!(
            generate_utility_css("scrollbar-gutter-both").unwrap(),
            "scrollbar-gutter: stable both-edges"
        );
        let scrollbar_thumb = generate_utility_css("scrollbar-thumb-red-500").unwrap();
        assert!(scrollbar_thumb.contains("--tw-scrollbar-thumb: var(--color-red-500)"));
        let scrollbar_track = generate_utility_css("scrollbar-track-slate-200").unwrap();
        assert!(scrollbar_track.contains("--tw-scrollbar-track: rgb(226 232 240)"));
        assert_eq!(generate_utility_css("zoom-125").unwrap(), "zoom: 125%");
        assert_eq!(
            generate_utility_css("zoom-(--dx-zoom)").unwrap(),
            "zoom: var(--dx-zoom)"
        );
        assert_eq!(generate_utility_css("tab-4").unwrap(), "tab-size: 4");
        assert_eq!(
            generate_utility_css("tab-(--dx-tab-size)").unwrap(),
            "tab-size: var(--dx-tab-size)"
        );
    }

    #[test]
    fn tailwind_v43_container_type_utilities_generate_css() {
        assert_eq!(
            generate_utility_css("@container").unwrap(),
            "container-type: inline-size"
        );
        assert_eq!(
            generate_utility_css("@container/main").unwrap(),
            "container-type: inline-size; container-name: main"
        );
        assert_eq!(
            generate_utility_css("@container-normal").unwrap(),
            "container-type: normal"
        );
        assert_eq!(
            generate_utility_css("@container-normal/sidebar").unwrap(),
            "container-type: normal; container-name: sidebar"
        );
        assert_eq!(
            generate_utility_css("@container-size").unwrap(),
            "container-type: size"
        );
        assert_eq!(
            generate_utility_css("@container-size/main").unwrap(),
            "container-type: size; container-name: main"
        );
        assert!(generate_utility_css("@container-size/[bad]").is_none());
        assert!(generate_utility_css("@container-size/1main").is_none());
        assert!(generate_utility_css("-@container-normal").is_none());
    }

    #[test]
    fn tailwind_v43_typography_detail_utilities_generate_css() {
        assert_eq!(
            generate_utility_css("wrap-anywhere").unwrap(),
            "overflow-wrap: anywhere"
        );
        assert_eq!(
            generate_utility_css("wrap-break-word").unwrap(),
            "overflow-wrap: break-word"
        );
        assert_eq!(
            generate_utility_css("indent-8").unwrap(),
            "text-indent: calc(var(--spacing) * 8)"
        );
        assert_eq!(
            generate_utility_css("-indent-8").unwrap(),
            "text-indent: calc(var(--spacing) * -8)"
        );
        assert_eq!(
            generate_utility_css("indent-(--dx-indent)").unwrap(),
            "text-indent: var(--dx-indent)"
        );
        assert_eq!(
            generate_utility_css("align-middle").unwrap(),
            "vertical-align: middle"
        );
        assert_eq!(
            generate_utility_css("align-(--dx-vertical-align)").unwrap(),
            "vertical-align: var(--dx-vertical-align)"
        );
        assert_eq!(
            generate_utility_css("decoration-4").unwrap(),
            "text-decoration-thickness: 4px"
        );
        assert_eq!(
            generate_utility_css("decoration-auto").unwrap(),
            "text-decoration-thickness: auto"
        );
        assert_eq!(
            generate_utility_css("decoration-from-font").unwrap(),
            "text-decoration-thickness: from-font"
        );
        assert_eq!(
            generate_utility_css("decoration-(length:--dx-decoration-thickness)").unwrap(),
            "text-decoration-thickness: var(--dx-decoration-thickness)"
        );
        assert_eq!(
            generate_utility_css("underline-offset-4").unwrap(),
            "text-underline-offset: 4px"
        );
        assert_eq!(
            generate_utility_css("-underline-offset-2").unwrap(),
            "text-underline-offset: -2px"
        );
        assert_eq!(
            generate_utility_css("underline-offset-(--dx-underline-offset)").unwrap(),
            "text-underline-offset: var(--dx-underline-offset)"
        );
        assert_eq!(
            generate_utility_css("decoration-red-500").unwrap(),
            "text-decoration-color: var(--color-red-500)"
        );
    }

    #[test]
    fn browser_prefix_tailwind_utilities_generate_css() {
        assert_eq!(
            generate_utility_css("appearance-none").unwrap(),
            "-webkit-appearance: none; appearance: none"
        );
        assert_eq!(
            generate_utility_css("select-none").unwrap(),
            "-webkit-user-select: none; user-select: none"
        );
        assert_eq!(
            generate_utility_css("backface-hidden").unwrap(),
            "-webkit-backface-visibility: hidden; backface-visibility: hidden"
        );
        assert_eq!(
            generate_utility_css("break-inside-avoid").unwrap(),
            "page-break-inside: avoid; break-inside: avoid"
        );
        assert_eq!(
            generate_utility_css("backdrop-filter-none").unwrap(),
            "-webkit-backdrop-filter: none; backdrop-filter: none"
        );

        let backdrop_blur_css = generate_utility_css("backdrop-blur-md").unwrap();
        assert!(backdrop_blur_css.contains("--tw-backdrop-blur: blur(var(--blur-md))"));
        assert!(backdrop_blur_css.contains("-webkit-backdrop-filter:"));
        assert!(backdrop_blur_css.contains("backdrop-filter:"));
    }

    #[test]
    fn high_usage_tailwind_background_and_list_utilities_generate_css() {
        assert_eq!(
            generate_utility_css("bg-cover").unwrap(),
            "background-size: cover"
        );
        assert_eq!(
            generate_utility_css("bg-size-(--dx-bg-size)").unwrap(),
            "background-size: var(--dx-bg-size)"
        );
        assert_eq!(
            generate_utility_css("bg-position-(--dx-bg-position)").unwrap(),
            "background-position: var(--dx-bg-position)"
        );
        assert_eq!(
            generate_utility_css("bg-no-repeat").unwrap(),
            "background-repeat: no-repeat"
        );
        assert_eq!(
            generate_utility_css("bg-left-top").unwrap(),
            "background-position: left top"
        );
        assert_eq!(
            generate_utility_css("bg-origin-border").unwrap(),
            "background-origin: border-box"
        );
        assert_eq!(
            generate_utility_css("bg-origin-padding").unwrap(),
            "background-origin: padding-box"
        );
        assert_eq!(
            generate_utility_css("bg-origin-content").unwrap(),
            "background-origin: content-box"
        );
        assert_eq!(
            generate_utility_css("bg-none").unwrap(),
            "background-image: none"
        );
        assert_eq!(
            generate_utility_css("bg-linear-to-r").unwrap(),
            "background-image: linear-gradient(to right, var(--tw-gradient-stops))"
        );
        assert_eq!(
            generate_utility_css("bg-linear-to-tl").unwrap(),
            "background-image: linear-gradient(to top left, var(--tw-gradient-stops))"
        );
        assert_eq!(
            generate_utility_css("bg-linear-45").unwrap(),
            "background-image: linear-gradient(45deg, var(--tw-gradient-stops))"
        );
        assert_eq!(
            generate_utility_css("-bg-linear-45").unwrap(),
            "background-image: linear-gradient(-45deg, var(--tw-gradient-stops))"
        );
        assert_eq!(
            generate_utility_css("bg-linear-[25deg]").unwrap(),
            "background-image: linear-gradient(25deg, var(--tw-gradient-stops))"
        );
        assert_eq!(
            generate_utility_css("bg-linear-(--dx-bg-linear)").unwrap(),
            "background-image: linear-gradient(var(--dx-bg-linear), var(--tw-gradient-stops))"
        );
        assert_eq!(
            generate_utility_css("bg-radial").unwrap(),
            "--tw-gradient-position: in oklab; background-image: radial-gradient(var(--tw-gradient-stops))"
        );
        assert_eq!(
            generate_utility_css("bg-radial-[circle_at_center]").unwrap(),
            "background-image: radial-gradient(circle at center, var(--tw-gradient-stops))"
        );
        assert_eq!(
            generate_utility_css("bg-radial-(--dx-bg-radial)").unwrap(),
            "background-image: radial-gradient(var(--dx-bg-radial), var(--tw-gradient-stops))"
        );
        assert_eq!(
            generate_utility_css("bg-conic").unwrap(),
            "--tw-gradient-position: in oklab; background-image: conic-gradient(var(--tw-gradient-stops))"
        );
        assert_eq!(
            generate_utility_css("bg-conic-180").unwrap(),
            "background-image: conic-gradient(from 180deg, var(--tw-gradient-stops))"
        );
        assert_eq!(
            generate_utility_css("-bg-conic-45").unwrap(),
            "background-image: conic-gradient(from -45deg, var(--tw-gradient-stops))"
        );
        assert_eq!(
            generate_utility_css("bg-conic-[from_45deg_at_center]").unwrap(),
            "background-image: conic-gradient(from 45deg at center, var(--tw-gradient-stops))"
        );
        assert_eq!(
            generate_utility_css("bg-conic-(--dx-bg-conic)").unwrap(),
            "background-image: conic-gradient(var(--dx-bg-conic), var(--tw-gradient-stops))"
        );
        assert_eq!(
            generate_utility_css("bg-[url('/hero.png')]").unwrap(),
            "background-image: url('/hero.png')"
        );
        assert_eq!(
            generate_utility_css("bg-(image:--dx-bg-image)").unwrap(),
            "background-image: var(--dx-bg-image)"
        );
        assert_eq!(
            generate_utility_css("list-disc").unwrap(),
            "list-style-type: disc"
        );
        assert_eq!(
            generate_utility_css("list-inside").unwrap(),
            "list-style-position: inside"
        );
        assert_eq!(
            generate_utility_css("divide-red-500").unwrap(),
            "CHILD|* + *|border-color: var(--color-red-500)"
        );
    }

    #[test]
    fn high_usage_tailwind_filter_utilities_generate_css() {
        let brightness_css = generate_utility_css("brightness-125").unwrap();
        assert!(brightness_css.contains("--tw-brightness: brightness(125%)"));
        assert!(
            generate_utility_css("-hue-rotate-30")
                .unwrap()
                .contains("--tw-hue-rotate: hue-rotate(-30deg)")
        );
        let backdrop_blur_css = generate_utility_css("backdrop-blur-md").unwrap();
        assert!(backdrop_blur_css.contains("--tw-backdrop-blur: blur(var(--blur-md))"));
        assert!(backdrop_blur_css.contains("-webkit-backdrop-filter:"));
        assert!(
            generate_utility_css("backdrop-saturate-150")
                .unwrap()
                .contains("--tw-backdrop-saturate: saturate(150%)")
        );
        assert_eq!(
            generate_utility_css("bg-blend-multiply").unwrap(),
            "background-blend-mode: multiply"
        );
        assert_eq!(
            generate_utility_css("mix-blend-plus-lighter").unwrap(),
            "mix-blend-mode: plus-lighter"
        );
        assert_eq!(
            generate_utility_css("mix-blend-[screen]").unwrap(),
            "mix-blend-mode: screen"
        );
        assert_eq!(
            generate_utility_css("bg-blend-(--dx-bg-blend)").unwrap(),
            "background-blend-mode: var(--dx-bg-blend)"
        );
        assert!(generate_utility_css("bg-blend-plus-lighter").is_none());
    }

    #[test]
    fn text_flow_and_line_clamp_utilities_generate_css() {
        assert_eq!(
            generate_utility_css("line-clamp-3").unwrap(),
            "overflow: hidden; display: -webkit-box; -webkit-box-orient: vertical; -webkit-line-clamp: 3"
        );
        assert_eq!(
            generate_utility_css("line-clamp-none").unwrap(),
            "overflow: visible; display: block; -webkit-box-orient: horizontal; -webkit-line-clamp: unset"
        );
        assert_eq!(
            generate_utility_css("line-clamp-[7]").unwrap(),
            "overflow: hidden; display: -webkit-box; -webkit-box-orient: vertical; -webkit-line-clamp: 7"
        );
        assert_eq!(
            generate_utility_css("hyphens-auto").unwrap(),
            "-webkit-hyphens: auto; hyphens: auto"
        );
        assert_eq!(
            generate_utility_css("text-balance").unwrap(),
            "text-wrap: balance"
        );
        assert_eq!(
            generate_utility_css("text-ellipsis").unwrap(),
            "overflow: hidden; text-overflow: ellipsis; white-space: nowrap"
        );
        assert_eq!(
            generate_utility_css("text-clip").unwrap(),
            "text-overflow: clip"
        );
    }

    #[test]
    fn font_stretch_utilities_generate_css() {
        assert_eq!(
            generate_utility_css("font-stretch-condensed").unwrap(),
            "font-stretch: condensed"
        );
        assert_eq!(
            generate_utility_css("font-stretch-semi-expanded").unwrap(),
            "font-stretch: semi-expanded"
        );
        assert_eq!(
            generate_utility_css("font-stretch-50%").unwrap(),
            "font-stretch: 50%"
        );
        assert_eq!(
            generate_utility_css("font-stretch-[62.5%]").unwrap(),
            "font-stretch: 62.5%"
        );
        assert_eq!(
            generate_utility_css("font-stretch-(--dx-font-stretch)").unwrap(),
            "font-stretch: var(--dx-font-stretch)"
        );
        assert!(generate_utility_css("font-stretch-wide").is_none());
    }

    #[test]
    fn forced_color_adjust_utilities_generate_css() {
        assert_eq!(
            generate_utility_css("forced-color-adjust-auto").unwrap(),
            "forced-color-adjust: auto"
        );
        assert_eq!(
            generate_utility_css("forced-color-adjust-none").unwrap(),
            "forced-color-adjust: none"
        );
    }

    #[test]
    fn content_utilities_generate_css() {
        assert_eq!(
            generate_utility_css("content-none").unwrap(),
            "content: none"
        );
        assert_eq!(
            generate_utility_css("content-['hello_world']").unwrap(),
            "content: 'hello world'"
        );
        assert_eq!(
            generate_utility_css("content-['hello\\_world']").unwrap(),
            "content: 'hello_world'"
        );
        assert_eq!(
            generate_utility_css("content-[attr(data-label)]").unwrap(),
            "content: attr(data-label)"
        );
        assert_eq!(
            generate_utility_css("content-(--dx-label)").unwrap(),
            "content: var(--dx-label)"
        );
        assert!(generate_utility_css("content-[bad;value]").is_none());
        assert!(generate_utility_css("content-(label)").is_none());
    }

    #[test]
    fn transition_property_utility_maps_colors() {
        let css = generate_utility_css("transition-colors").unwrap();
        assert!(css.contains(
            "transition-property: color, background-color, border-color, text-decoration-color, fill, stroke"
        ));
        assert!(css.contains("transition-duration: 150ms"));
    }

    #[test]
    fn transition_duration_delay_and_easing_utilities_generate_css() {
        assert_eq!(
            generate_utility_css("duration-300").unwrap(),
            "transition-duration: 300ms"
        );
        assert_eq!(
            generate_utility_css("delay-200").unwrap(),
            "transition-delay: 200ms"
        );
        assert_eq!(
            generate_utility_css("ease-in-out").unwrap(),
            "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1)"
        );
    }

    #[test]
    fn transition_arbitrary_values_generate_css() {
        assert_eq!(
            generate_utility_css("duration-[375ms]").unwrap(),
            "transition-duration: 375ms"
        );
        assert_eq!(
            generate_utility_css("ease-[cubic-bezier(0.2,_0,_0,_1)]").unwrap(),
            "transition-timing-function: cubic-bezier(0.2, 0, 0, 1)"
        );
    }

    #[test]
    fn transition_property_aliases_generate_css() {
        assert!(
            generate_utility_css("transition-[height,opacity]")
                .unwrap()
                .contains("transition-property: height,opacity")
        );
        assert!(
            generate_utility_css("transition-(--dx-transition-property)")
                .unwrap()
                .contains("transition-property: var(--dx-transition-property)")
        );
    }

    #[test]
    fn transition_behavior_utilities_generate_css() {
        assert_eq!(
            generate_utility_css("transition-discrete").unwrap(),
            "transition-behavior: allow-discrete"
        );
        assert_eq!(
            generate_utility_css("transition-normal").unwrap(),
            "transition-behavior: normal"
        );
    }
}
