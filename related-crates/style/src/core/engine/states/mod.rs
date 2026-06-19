use crate::core::engine::StyleEngine;
use smallvec::SmallVec;

fn bracketed_attribute_selector(prefix: &str, raw: &str) -> Option<String> {
    let content = raw.strip_prefix('[')?.strip_suffix(']')?.trim();
    if content.is_empty() || content.contains(']') || content.contains(';') || content.contains('{')
    {
        return None;
    }

    if let Some((name, value)) = content.split_once('=') {
        let name = name.trim();
        let value = value.trim().trim_matches('"').trim_matches('\'');
        if name.is_empty() || value.is_empty() || name.contains(' ') || value.contains('"') {
            return None;
        }
        Some(format!("&[{prefix}-{name}=\"{value}\"]"))
    } else if content.contains(' ') || content.contains('"') || content.contains('\'') {
        None
    } else {
        Some(format!("&[{prefix}-{content}]"))
    }
}

fn data_attribute_selector(part: &str) -> Option<String> {
    let raw = part.strip_prefix("data-")?;
    if raw.is_empty() {
        return None;
    }

    if raw.starts_with('[') {
        return bracketed_attribute_selector("data", raw);
    }

    Some(format!("&[data-{raw}]"))
}

fn aria_attribute_selector(part: &str) -> Option<String> {
    let raw = part.strip_prefix("aria-")?;
    if raw.is_empty() {
        return None;
    }

    if raw.starts_with('[') {
        return bracketed_attribute_selector("aria", raw);
    }

    Some(format!("&[aria-{raw}=\"true\"]"))
}

fn named_variant_fragment(name: &str) -> Option<String> {
    if name.is_empty()
        || !name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
    {
        return None;
    }

    Some(name.replace('_', "\\_"))
}

fn named_group_peer_selector(part: &str) -> Option<String> {
    group_peer_state_selector(part)
}

fn safe_container_query_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
}

fn decode_arbitrary_variant_selector(raw: &str) -> String {
    let mut decoded = String::with_capacity(raw.len());
    let mut chars = raw.chars().peekable();

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

fn sanitize_arbitrary_variant_selector(raw: &str) -> Option<String> {
    let selector = decode_arbitrary_variant_selector(raw.trim());
    let selector = selector.trim();
    if selector.is_empty()
        || selector.contains(';')
        || selector.contains('{')
        || selector.contains('}')
        || selector.contains('"')
        || selector.contains('\'')
    {
        return None;
    }

    Some(selector.to_string())
}

fn has_top_level_selector_list(selector: &str) -> bool {
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut escaped = false;

    for ch in selector.chars() {
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
            ',' if paren_depth == 0 && bracket_depth == 0 => return true,
            _ => {}
        }
    }

    false
}

fn arbitrary_variant_selector_and_name(raw: &str) -> Option<(&str, Option<&str>)> {
    if let Some((selector, name)) = raw.rsplit_once("]/") {
        if selector.is_empty() || name.is_empty() {
            return None;
        }

        return Some((selector, Some(name)));
    }

    let selector = raw.strip_suffix(']')?;
    if selector.is_empty() {
        return None;
    }

    Some((selector, None))
}

fn group_peer_base_selector(kind: &str, name: Option<&str>) -> Option<String> {
    match name {
        Some(name) => {
            let escaped_name = named_variant_fragment(name)?;
            Some(format!(".{kind}\\/{escaped_name}"))
        }
        None => Some(format!(".{kind}")),
    }
}

fn group_peer_tailwind_wrapper(kind: &str, name: Option<&str>, condition: &str) -> Option<String> {
    let base_selector = group_peer_base_selector(kind, name)?;
    match kind {
        "group" => Some(format!("&:is(:where({base_selector}){condition} *)")),
        "peer" => Some(format!("&:is(:where({base_selector}){condition} ~ *)")),
        _ => None,
    }
}

fn arbitrary_group_peer_tailwind_wrapper(
    kind: &str,
    name: Option<&str>,
    selector: &str,
) -> Option<String> {
    let base_selector = group_peer_base_selector(kind, name)?;
    let base_where = format!(":where({base_selector})");
    let owner_selector = if selector.contains('&') {
        let selector = selector.replace('&', &base_where);
        if has_top_level_selector_list(&selector) {
            format!(":is({selector})")
        } else {
            selector
        }
    } else {
        format!("{base_where}:is({selector})")
    };

    match kind {
        "group" => Some(format!("&:is({owner_selector} *)")),
        "peer" => Some(format!("&:is({owner_selector} ~ *)")),
        _ => None,
    }
}

fn split_variant_condition_and_name(raw: &str) -> Option<(&str, Option<&str>)> {
    if raw.is_empty() {
        return None;
    }

    let mut bracket_depth = 0usize;
    let mut paren_depth = 0usize;
    for (index, byte) in raw.bytes().enumerate() {
        match byte {
            b'[' => bracket_depth = bracket_depth.saturating_add(1),
            b']' => bracket_depth = bracket_depth.saturating_sub(1),
            b'(' if bracket_depth == 0 => paren_depth = paren_depth.saturating_add(1),
            b')' if bracket_depth == 0 => paren_depth = paren_depth.saturating_sub(1),
            b'/' if bracket_depth == 0 && paren_depth == 0 => {
                let condition = &raw[..index];
                let name = &raw[index + 1..];
                if condition.is_empty() || name.is_empty() {
                    return None;
                }
                return Some((condition, Some(name)));
            }
            _ => {}
        }
    }

    Some((raw, None))
}

fn attribute_variant_condition<'a>(kind: &str, raw: &'a str) -> Option<(String, Option<&'a str>)> {
    let (condition, name) = split_variant_condition_and_name(raw)?;
    let variant = format!("{kind}-{condition}");
    let selector = match kind {
        "aria" => aria_attribute_selector(&variant),
        "data" => data_attribute_selector(&variant),
        _ => None,
    }?;
    let condition = selector.strip_prefix('&')?;
    if condition.is_empty() || condition.contains('&') {
        return None;
    }
    Some((condition.to_string(), name))
}

fn group_peer_attribute_variant_selector(part: &str) -> Option<String> {
    let (kind, attribute_kind, raw) = if let Some(raw) = part.strip_prefix("group-aria-") {
        ("group", "aria", raw)
    } else if let Some(raw) = part.strip_prefix("group-data-") {
        ("group", "data", raw)
    } else if let Some(raw) = part.strip_prefix("peer-aria-") {
        ("peer", "aria", raw)
    } else if let Some(raw) = part.strip_prefix("peer-data-") {
        ("peer", "data", raw)
    } else {
        return None;
    };

    let (selector, name) = attribute_variant_condition(attribute_kind, raw)?;
    group_peer_tailwind_wrapper(kind, name, &selector)
}

fn has_state_selector(raw: &str) -> Option<&'static str> {
    match raw {
        "active" => Some(":active"),
        "autofill" => Some(":autofill"),
        "checked" => Some(":checked"),
        "default" => Some(":default"),
        "disabled" => Some(":disabled"),
        "empty" => Some(":empty"),
        "enabled" => Some(":enabled"),
        "even" => Some(":nth-child(even)"),
        "first" => Some(":first-child"),
        "first-of-type" => Some(":first-of-type"),
        "focus" => Some(":focus"),
        "focus-within" => Some(":focus-within"),
        "focus-visible" => Some(":focus-visible"),
        "indeterminate" => Some(":indeterminate"),
        "in-range" => Some(":in-range"),
        "hover" => Some(":hover"),
        "invalid" => Some(":invalid"),
        "last" => Some(":last-child"),
        "odd" => Some(":nth-child(odd)"),
        "last-of-type" => Some(":last-of-type"),
        "only" => Some(":only-child"),
        "only-of-type" => Some(":only-of-type"),
        "open" => Some(":is([open], :popover-open, :open)"),
        "optional" => Some(":optional"),
        "out-of-range" => Some(":out-of-range"),
        "placeholder-shown" => Some(":placeholder-shown"),
        "read-only" => Some(":read-only"),
        "read-write" => Some(":read-write"),
        "required" => Some(":required"),
        "target" => Some(":target"),
        "details-content" => Some(":details-content"),
        "user-valid" => Some(":user-valid"),
        "user-invalid" => Some(":user-invalid"),
        "valid" => Some(":valid"),
        "visited" => Some(":visited"),
        _ => None,
    }
}

fn condition_selector(raw: &str, relative_state: bool) -> Option<String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return None;
    }

    if raw.starts_with('[') {
        let selector = raw.strip_prefix('[')?.strip_suffix(']')?;
        return sanitize_arbitrary_variant_selector(selector);
    }

    has_state_selector(raw).map(|selector| {
        if relative_state {
            format!("*{selector}")
        } else {
            selector.to_string()
        }
    })
}

fn relative_condition_selector(raw: &str) -> Option<String> {
    condition_selector(raw, true)
}

fn has_condition_selector_and_name(raw: &str) -> Option<(String, Option<&str>)> {
    if raw.starts_with('[') {
        let raw = raw.strip_prefix('[')?;
        let (selector, name) = arbitrary_variant_selector_and_name(raw)?;
        return Some((sanitize_arbitrary_variant_selector(selector)?, name));
    }

    if let Some((condition, name)) = raw.split_once('/') {
        if condition.is_empty() || name.is_empty() {
            return None;
        }

        return Some((relative_condition_selector(condition)?, Some(name)));
    }

    Some((relative_condition_selector(raw)?, None))
}

fn has_variant_selector(part: &str) -> Option<String> {
    if let Some(raw) = part.strip_prefix("has-") {
        let selector = relative_condition_selector(raw)?;
        return Some(format!("&:has({selector})"));
    }

    if let Some(raw) = part.strip_prefix("group-has-") {
        let (selector, name) = has_condition_selector_and_name(raw)?;
        return group_peer_tailwind_wrapper("group", name, &format!(":has({selector})"));
    }

    if let Some(raw) = part.strip_prefix("peer-has-") {
        let (selector, name) = has_condition_selector_and_name(raw)?;
        return group_peer_tailwind_wrapper("peer", name, &format!(":has({selector})"));
    }

    None
}

fn not_condition_selector(raw: &str) -> Option<String> {
    let selector = relative_condition_selector(raw)?;
    if selector.trim_start().starts_with('@') {
        return None;
    }
    if selector.contains('&') {
        return None;
    }
    Some(selector)
}

fn not_condition_selector_and_name(raw: &str) -> Option<(String, Option<&str>)> {
    if raw.starts_with('[') {
        let raw = raw.strip_prefix('[')?;
        let (selector, name) = arbitrary_variant_selector_and_name(raw)?;
        let selector = sanitize_arbitrary_variant_selector(selector)?;
        if selector.contains('&') {
            return None;
        }
        return Some((selector, name));
    }

    if let Some((condition, name)) = raw.split_once('/') {
        if condition.is_empty() || name.is_empty() {
            return None;
        }

        return Some((not_condition_selector(condition)?, Some(name)));
    }

    Some((not_condition_selector(raw)?, None))
}

fn not_variant_selector(part: &str) -> Option<String> {
    if let Some(raw) = part.strip_prefix("not-") {
        let selector = not_condition_selector(raw)?;
        return Some(format!("&:not({selector})"));
    }

    if let Some(raw) = part.strip_prefix("group-not-") {
        let (selector, name) = not_condition_selector_and_name(raw)?;
        return group_peer_tailwind_wrapper("group", name, &format!(":not({selector})"));
    }

    if let Some(raw) = part.strip_prefix("peer-not-") {
        let (selector, name) = not_condition_selector_and_name(raw)?;
        return group_peer_tailwind_wrapper("peer", name, &format!(":not({selector})"));
    }

    None
}

fn arbitrary_group_peer_selector(part: &str) -> Option<String> {
    if let Some(raw) = part.strip_prefix("group-[") {
        let (raw_selector, name) = arbitrary_variant_selector_and_name(raw)?;
        let selector = sanitize_arbitrary_variant_selector(raw_selector)?;
        return arbitrary_group_peer_tailwind_wrapper("group", name, &selector);
    }

    if let Some(raw) = part.strip_prefix("peer-[") {
        let (raw_selector, name) = arbitrary_variant_selector_and_name(raw)?;
        let selector = sanitize_arbitrary_variant_selector(raw_selector)?;
        return arbitrary_group_peer_tailwind_wrapper("peer", name, &selector);
    }

    None
}

fn arbitrary_selector_variant(part: &str) -> Option<String> {
    let selector = sanitize_arbitrary_variant_selector(part.strip_prefix('[')?.strip_suffix(']')?)?;
    if !selector.contains('&') {
        return None;
    }
    Some(selector)
}

fn child_selector_variant(part: &str) -> Option<&'static str> {
    match part {
        "*" => Some(":is(& > *)"),
        "**" => Some(":is(& *)"),
        _ => None,
    }
}

fn in_condition_selector(raw: &str) -> Option<String> {
    let selector = relative_condition_selector(raw)?;
    if selector.contains('&') {
        return None;
    }
    Some(selector)
}

fn in_variant_selector(part: &str) -> Option<String> {
    let raw = part.strip_prefix("in-")?;
    let selector = in_condition_selector(raw)?;
    Some(format!(":where({selector}) &"))
}

fn sanitize_nth_expression(raw: &str) -> Option<String> {
    let expr = raw.trim().replace('_', " ");
    if expr.is_empty()
        || expr.contains(';')
        || expr.contains('{')
        || expr.contains('}')
        || expr.contains('"')
        || expr.contains('\'')
        || expr.contains('&')
    {
        return None;
    }

    if !expr
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '+' | '-' | ' ' | '(' | ')'))
    {
        return None;
    }

    Some(expr)
}

fn structural_nth_expression(part: &str, prefix: &str) -> Option<String> {
    let raw = part.strip_prefix(prefix)?;
    if let Some(raw) = raw.strip_prefix('[').and_then(|raw| raw.strip_suffix(']')) {
        return sanitize_nth_expression(raw);
    }

    if raw.is_empty() || !raw.chars().all(|ch| ch.is_ascii_digit()) {
        return None;
    }

    Some(raw.to_string())
}

fn structural_nth_variant(part: &str) -> Option<String> {
    for (prefix, pseudo) in [
        ("nth-last-of-type-", "nth-last-of-type"),
        ("nth-of-type-", "nth-of-type"),
        ("nth-last-", "nth-last-child"),
        ("nth-", "nth-child"),
    ] {
        if let Some(expr) = structural_nth_expression(part, prefix) {
            return Some(format!("&:{pseudo}({expr})"));
        }
    }

    None
}

fn selector_suffix(selector: &str) -> Option<String> {
    if let Some(selector) = selector.strip_prefix('&') {
        return Some(selector.to_string());
    }

    if selector.starts_with(':') || selector.starts_with('[') {
        return Some(selector.to_string());
    }

    None
}

fn group_peer_state_selector(part: &str) -> Option<String> {
    let (kind, raw_state) = if let Some(raw_state) = part.strip_prefix("group-") {
        ("group", raw_state)
    } else if let Some(raw_state) = part.strip_prefix("peer-") {
        ("peer", raw_state)
    } else {
        return None;
    };

    if raw_state.starts_with("has-") || raw_state.starts_with("not-") || raw_state.starts_with('[')
    {
        return None;
    }

    let (state, name) = if let Some((state, name)) = raw_state.rsplit_once('/') {
        if state.is_empty() || name.is_empty() {
            return None;
        }
        (state, Some(name))
    } else {
        (raw_state, None)
    };

    let state_selector = has_state_selector(state).map(str::to_string).or_else(|| {
        structural_nth_variant(state).and_then(|selector| selector_suffix(&selector))
    })?;

    group_peer_tailwind_wrapper(kind, name, &state_selector)
}

fn named_container_query_variant(part: &str) -> Option<(&str, &str)> {
    let (query, name) = part.split_once('/')?;
    if !safe_container_query_name(name) {
        return None;
    }
    Some((query, name))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ContainerQueryAxis {
    Min,
    Max,
}

impl ContainerQueryAxis {
    fn range_operator(self) -> &'static str {
        match self {
            Self::Min => ">=",
            Self::Max => "<",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ContainerQueryVariant {
    at_rule: String,
}

impl ContainerQueryVariant {
    fn new(query: String, container_name: &str) -> Self {
        let at_rule = if container_name.is_empty() {
            format!("@container {query}")
        } else {
            format!("@container {container_name} {query}")
        };

        Self { at_rule }
    }
}

fn container_query_range_condition(value: &str, axis: ContainerQueryAxis) -> Option<String> {
    let value = value.trim();
    if value.is_empty()
        || value.contains(';')
        || value.contains('{')
        || value.contains('}')
        || value.contains('"')
        || value.contains('\'')
    {
        return None;
    }
    Some(format!("(width {} {value})", axis.range_operator()))
}

fn tailwind_container_query_variant(
    part: &str,
    container_name: &str,
) -> Option<ContainerQueryVariant> {
    let raw = part.strip_prefix("@[")?.strip_suffix(']')?.trim();
    let axis = ContainerQueryAxis::Min;
    let query = container_query_range_condition(raw, axis)?;
    Some(ContainerQueryVariant::new(query, container_name))
}

fn tailwind_container_query_variant_for_engine(
    engine: &StyleEngine,
    part: &str,
) -> Option<ContainerQueryVariant> {
    let (query_part, container_name) =
        named_container_query_variant(part).map_or((part, ""), |(query, name)| (query, name));

    let query = if let Some(raw) = query_part
        .strip_prefix("@max-[")
        .and_then(|raw| raw.strip_suffix(']'))
    {
        container_query_range_condition(raw, ContainerQueryAxis::Max)?
    } else if let Some(raw) = query_part
        .strip_prefix("@min-[")
        .and_then(|raw| raw.strip_suffix(']'))
    {
        container_query_range_condition(raw, ContainerQueryAxis::Min)?
    } else if let Some(raw) = query_part.strip_prefix("@max-") {
        let token = format!("@{raw}");
        let value = engine.container_queries.get(&token)?;
        container_query_range_condition(value, ContainerQueryAxis::Max)?
    } else if let Some(raw) = query_part.strip_prefix("@min-") {
        let token = format!("@{raw}");
        let value = engine.container_queries.get(&token)?;
        container_query_range_condition(value, ContainerQueryAxis::Min)?
    } else if let Some(value) = engine.container_queries.get(query_part) {
        container_query_range_condition(value, ContainerQueryAxis::Min)?
    } else {
        return tailwind_container_query_variant(query_part, container_name);
    };

    Some(ContainerQueryVariant::new(query, container_name))
}

fn trim_media_decimal(value: f32) -> String {
    let mut out = format!("{value:.4}");
    while out.contains('.') && out.ends_with('0') {
        out.pop();
    }
    if out.ends_with('.') {
        out.pop();
    }
    out
}

fn exclusive_max_screen_width(value: &str) -> Option<String> {
    let raw = value.trim().strip_suffix("px")?;
    let width = raw.parse::<f32>().ok()?;
    if width <= 0.02 {
        return None;
    }
    Some(format!("{}px", trim_media_decimal(width - 0.02)))
}

fn media_query_width(value: &str, axis: &str, exclusive_max: bool) -> Option<String> {
    if !matches!(axis, "min" | "max") {
        return None;
    }
    let value = value.trim();
    if value.is_empty()
        || value.contains(';')
        || value.contains('{')
        || value.contains('}')
        || value.contains('"')
        || value.contains('\'')
    {
        return None;
    }

    let value = if exclusive_max {
        exclusive_max_screen_width(value).unwrap_or_else(|| value.to_string())
    } else {
        value.to_string()
    };
    Some(format!("({axis}-width: {value})"))
}

fn tailwind_media_query_variant_for_engine(engine: &StyleEngine, part: &str) -> Option<String> {
    let query = match part {
        "starting" => return Some("@starting-style".to_string()),
        "print" => return Some("@media print".to_string()),
        _ => {
            if let Some(query) = tailwind_media_condition_for_variant(part) {
                query
            } else if let Some(raw) = part
                .strip_prefix("min-[")
                .and_then(|raw| raw.strip_suffix(']'))
            {
                media_query_width(raw, "min", false)?
            } else if let Some(raw) = part
                .strip_prefix("max-[")
                .and_then(|raw| raw.strip_suffix(']'))
            {
                media_query_width(raw, "max", false)?
            } else if let Some(raw) = part.strip_prefix("min-") {
                let value = engine.screens.get(raw)?;
                media_query_width(value, "min", false)?
            } else if let Some(raw) = part.strip_prefix("max-") {
                let value = engine.screens.get(raw)?;
                media_query_width(value, "max", true)?
            } else {
                return None;
            }
        }
    };

    Some(format!("@media {query}"))
}

fn tailwind_media_condition_for_variant(part: &str) -> Option<String> {
    match part {
        "motion-safe" => Some("(prefers-reduced-motion: no-preference)".to_string()),
        "motion-reduce" => Some("(prefers-reduced-motion: reduce)".to_string()),
        "contrast-more" => Some("(prefers-contrast: more)".to_string()),
        "contrast-less" => Some("(prefers-contrast: less)".to_string()),
        "forced-colors" => Some("(forced-colors: active)".to_string()),
        "inverted-colors" => Some("(inverted-colors: inverted)".to_string()),
        "pointer-none" => Some("(pointer: none)".to_string()),
        "pointer-coarse" => Some("(pointer: coarse)".to_string()),
        "pointer-fine" => Some("(pointer: fine)".to_string()),
        "any-pointer-none" => Some("(any-pointer: none)".to_string()),
        "any-pointer-coarse" => Some("(any-pointer: coarse)".to_string()),
        "any-pointer-fine" => Some("(any-pointer: fine)".to_string()),
        "any-hover" => Some("(any-hover: hover)".to_string()),
        "portrait" => Some("(orientation: portrait)".to_string()),
        "landscape" => Some("(orientation: landscape)".to_string()),
        "noscript" => Some("(scripting: none)".to_string()),
        _ => None,
    }
}

fn tailwind_negated_media_query_variant(part: &str) -> Option<String> {
    let inner = part.strip_prefix("not-")?;
    if inner == "print" {
        return Some("@media not print".to_string());
    }
    if inner.starts_with("supports-") {
        return None;
    }
    let query = tailwind_media_condition_for_variant(inner)?;
    Some(format!("@media not {query}"))
}

fn hover_capability_variant_selector(part: &str) -> Option<Option<String>> {
    match part {
        "hover" => return Some(None),
        "group-hover" | "peer-hover" => return group_peer_state_selector(part).map(Some),
        _ => {}
    }

    if part.starts_with("group-hover/") || part.starts_with("peer-hover/") {
        return named_group_peer_selector(part).map(Some);
    }

    None
}

fn in_hover_capability_variant_selector(part: &str) -> Option<String> {
    (part == "in-hover").then(|| ":where(*:hover) &".to_string())
}

fn supports_condition(raw: &str) -> Option<String> {
    let condition = raw.trim().replace('_', " ");
    if condition.is_empty()
        || condition.contains(';')
        || condition.contains('{')
        || condition.contains('}')
        || condition.contains('"')
        || condition.contains('\'')
    {
        return None;
    }

    if condition.starts_with('(') && condition.ends_with(')') {
        return Some(condition);
    }

    if let Some((property, value)) = condition.split_once(':') {
        let property = property.trim();
        let value = value.trim();
        if property.is_empty()
            || value.is_empty()
            || !property
                .chars()
                .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
        {
            return None;
        }
        return Some(format!("({property}: {value})"));
    }

    Some(format!("({condition})"))
}

fn supports_query_variant(part: &str) -> Option<String> {
    if let Some(raw) = part
        .strip_prefix("supports-[")
        .and_then(|raw| raw.strip_suffix(']'))
    {
        return supports_condition(raw).map(|query| format!("@supports {query}"));
    }

    if let Some(raw) = part
        .strip_prefix("not-supports-[")
        .and_then(|raw| raw.strip_suffix(']'))
    {
        return supports_condition(raw).map(|query| format!("@supports not {query}"));
    }

    None
}

fn arbitrary_at_rule_from_decoded(decoded: &str) -> Option<String> {
    let decoded = decoded.trim();
    if decoded.is_empty()
        || decoded.contains(';')
        || decoded.contains('{')
        || decoded.contains('}')
        || decoded.contains('"')
        || decoded.contains('\'')
        || decoded.contains('&')
    {
        return None;
    }

    if decoded == "@starting-style" {
        return Some("@starting-style".to_string());
    }

    if let Some(layer) = decoded.strip_prefix("@layer ") {
        let layer = layer.trim();
        if safe_cascade_layer_name(layer) {
            return Some(format!("@layer {layer}"));
        }
        return None;
    }

    for at_rule in ["@media", "@supports", "@container"] {
        if let Some(condition) = decoded.strip_prefix(at_rule) {
            let condition = condition.trim();
            if condition.is_empty() || condition.contains('@') {
                return None;
            }
            return Some(format!("{at_rule} {condition}"));
        }
    }

    if let Some(at_rule) = safe_custom_at_rule_from_decoded(decoded) {
        return Some(at_rule);
    }

    None
}

fn arbitrary_at_rule_variant(part: &str) -> Option<String> {
    let raw = part.strip_prefix('[')?.strip_suffix(']')?;
    let decoded = decode_arbitrary_variant_selector(raw.trim());
    arbitrary_at_rule_from_decoded(&decoded)
}

fn negated_arbitrary_at_rule_variant(part: &str) -> Option<String> {
    let raw = part.strip_prefix("not-[")?.strip_suffix(']')?;
    let decoded = decode_arbitrary_variant_selector(raw.trim());
    let at_rule = arbitrary_at_rule_from_decoded(&decoded)?;

    if let Some(condition) = at_rule.strip_prefix("@media ") {
        let condition = condition.trim();
        let negated = condition
            .strip_prefix("not ")
            .map(str::trim)
            .filter(|condition| !condition.is_empty())
            .map(str::to_string)
            .unwrap_or_else(|| format!("not {condition}"));
        return Some(format!("@media {negated}"));
    }

    if let Some(condition) = at_rule.strip_prefix("@supports ") {
        let condition = condition.trim();
        if !condition.is_empty() {
            return Some(format!("@supports not {condition}"));
        }
    }

    if let Some(condition) = at_rule.strip_prefix("@container ") {
        return negated_container_at_rule(condition.trim());
    }

    None
}

fn split_named_container_condition(condition: &str) -> Option<(&str, &str)> {
    let split_at = condition
        .char_indices()
        .find_map(|(index, ch)| ch.is_whitespace().then_some(index))?;
    let name = &condition[..split_at];
    let query = condition[split_at..].trim();

    if safe_container_query_name(name) && !query.is_empty() {
        Some((name, query))
    } else {
        None
    }
}

fn negated_container_at_rule(condition: &str) -> Option<String> {
    if condition.is_empty() {
        return None;
    }

    if let Some(non_negated) = condition
        .strip_prefix("not ")
        .map(str::trim)
        .filter(|query| !query.is_empty())
    {
        return Some(format!("@container {non_negated}"));
    }

    if let Some((name, query)) = split_named_container_condition(condition) {
        let negated = query
            .strip_prefix("not ")
            .map(str::trim)
            .filter(|query| !query.is_empty())
            .map(str::to_string)
            .unwrap_or_else(|| format!("not {query}"));
        return Some(format!("@container {name} {negated}"));
    }

    Some(format!("@container not {condition}"))
}

fn arbitrary_at_rule_selector_variant(part: &str) -> Option<(String, String)> {
    let raw = part.strip_prefix('[')?.strip_suffix(']')?;
    let (at_rule_raw, selector_raw) = raw.rsplit_once('{')?;
    let selector_raw = selector_raw.strip_suffix('}')?;
    if at_rule_raw.contains('{')
        || selector_raw.contains('{')
        || selector_raw.contains('}')
        || selector_raw.is_empty()
    {
        return None;
    }

    let at_rule = decode_arbitrary_variant_selector(at_rule_raw.trim());
    let selector = sanitize_arbitrary_variant_selector(selector_raw)?;
    if !selector.contains('&') {
        return None;
    }

    Some((arbitrary_at_rule_from_decoded(&at_rule)?, selector))
}

fn safe_cascade_layer_name(layer: &str) -> bool {
    if layer.is_empty() || layer.contains(',') {
        return false;
    }

    layer.split('.').all(|segment| {
        !segment.is_empty()
            && segment
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
    })
}

fn balanced_custom_at_rule_prelude(prelude: &str) -> bool {
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;

    for ch in prelude.chars() {
        match ch {
            '(' => paren_depth = paren_depth.saturating_add(1),
            ')' => {
                if paren_depth == 0 {
                    return false;
                }
                paren_depth -= 1;
            }
            '[' => bracket_depth = bracket_depth.saturating_add(1),
            ']' => {
                if bracket_depth == 0 {
                    return false;
                }
                bracket_depth -= 1;
            }
            _ => {}
        }
    }

    paren_depth == 0 && bracket_depth == 0
}

fn safe_custom_at_rule_name(name: &str) -> bool {
    let mut chars = name.chars();
    let first = chars.next();
    matches!(first, Some(ch) if ch.is_ascii_alphabetic() || ch == '-')
        && name.chars().any(|ch| ch.is_ascii_alphabetic())
        && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '-')
}

fn safe_custom_at_rule_from_decoded(decoded: &str) -> Option<String> {
    let body = decoded.strip_prefix('@')?;
    if body.contains('@') {
        return None;
    }

    let name_end = body
        .char_indices()
        .find_map(|(index, ch)| (!(ch.is_ascii_alphanumeric() || ch == '-')).then_some(index))
        .unwrap_or(body.len());
    let name = &body[..name_end];
    if !safe_custom_at_rule_name(name) {
        return None;
    }

    let lower_name = name.to_ascii_lowercase();
    if matches!(
        lower_name.as_str(),
        "apply"
            | "charset"
            | "config"
            | "custom-variant"
            | "import"
            | "namespace"
            | "plugin"
            | "reference"
            | "source"
            | "tailwind"
            | "theme"
            | "utility"
            | "variant"
    ) {
        return None;
    }

    let prelude = body[name_end..].trim();
    if prelude.contains('\\')
        || prelude.contains("/*")
        || prelude.contains("*/")
        || !balanced_custom_at_rule_prelude(prelude)
    {
        return None;
    }

    if prelude.is_empty() {
        Some(format!("@{name}"))
    } else {
        Some(format!("@{name} {prelude}"))
    }
}

fn split_variant_parts(prefix_segment: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut bracket_depth = 0usize;
    let mut paren_depth = 0usize;

    for (index, byte) in prefix_segment.bytes().enumerate() {
        match byte {
            b'[' => bracket_depth = bracket_depth.saturating_add(1),
            b']' => bracket_depth = bracket_depth.saturating_sub(1),
            b'(' if bracket_depth == 0 => paren_depth = paren_depth.saturating_add(1),
            b')' if bracket_depth == 0 => paren_depth = paren_depth.saturating_sub(1),
            b':' if bracket_depth == 0 && paren_depth == 0 => {
                parts.push(&prefix_segment[start..index]);
                start = index + 1;
            }
            _ => {}
        }
    }

    parts.push(&prefix_segment[start..]);
    parts
}

type AppliedVariantState = (SmallVec<[String; 4]>, String, SmallVec<[String; 2]>);

pub fn apply_wrappers_and_states(
    engine: &StyleEngine,
    prefix_segment: &str,
) -> Option<AppliedVariantState> {
    apply_wrappers_and_states_with_not_hover_mode(engine, prefix_segment, false)
}

pub fn apply_not_hover_fallback_wrappers_and_states(
    engine: &StyleEngine,
    prefix_segment: &str,
) -> Option<AppliedVariantState> {
    apply_wrappers_and_states_with_not_hover_mode(engine, prefix_segment, true)
}

fn apply_wrappers_and_states_with_not_hover_mode(
    engine: &StyleEngine,
    prefix_segment: &str,
    not_hover_fallback: bool,
) -> Option<AppliedVariantState> {
    let mut media_queries: SmallVec<[String; 4]> = SmallVec::new();
    let pseudo_classes = String::new();
    let mut wrappers: SmallVec<[String; 2]> = SmallVec::new();
    let mut saw_not_hover = false;
    if !prefix_segment.is_empty() {
        for part in split_variant_parts(prefix_segment) {
            if let Some(custom_variant) = engine.custom_variants.get(part) {
                media_queries.extend(custom_variant.media_queries.iter().cloned());
                wrappers.push(custom_variant.selector.clone());
            } else if let Some(screen_value) = engine.screens.get(part) {
                media_queries.push(format!("@media (min-width: {})", screen_value));
            } else if not_hover_fallback && part == "not-hover" {
                saw_not_hover = true;
                media_queries.push("@media not (hover: hover)".to_string());
            } else if let Some(selector) = hover_capability_variant_selector(part) {
                media_queries.push("@media (hover: hover)".to_string());
                if let Some(selector) = selector {
                    wrappers.push(selector);
                } else {
                    wrappers.push("&:hover".to_string());
                }
            } else if let Some(selector) = in_hover_capability_variant_selector(part) {
                media_queries.push("@media (hover: hover)".to_string());
                wrappers.push(selector);
            } else if let Some(media_query) = tailwind_media_query_variant_for_engine(engine, part)
            {
                media_queries.push(media_query);
            } else if let Some(media_query) = tailwind_negated_media_query_variant(part) {
                media_queries.push(media_query);
            } else if let Some(supports_query) = supports_query_variant(part) {
                media_queries.push(supports_query);
            } else if let Some((at_rule, selector)) = arbitrary_at_rule_selector_variant(part) {
                media_queries.push(at_rule);
                wrappers.push(selector);
            } else if let Some(at_rule) = arbitrary_at_rule_variant(part) {
                media_queries.push(at_rule);
            } else if let Some(cq_query) = tailwind_container_query_variant_for_engine(engine, part)
            {
                media_queries.push(cq_query.at_rule);
            } else if let Some(state_value) = engine.states.get(part) {
                if state_value.contains('&') {
                    wrappers.push(state_value.to_string());
                } else {
                    wrappers.push(format!("&{state_value}"));
                }
            } else if part == "dark" {
                wrappers.push(".dark &".to_string());
            } else if part == "light" {
                wrappers.push(":root &".to_string());
            } else if let Some(selector) = data_attribute_selector(part) {
                wrappers.push(selector);
            } else if let Some(selector) = aria_attribute_selector(part) {
                wrappers.push(selector);
            } else if let Some(selector) = has_variant_selector(part) {
                wrappers.push(selector);
            } else if let Some(at_rule) = negated_arbitrary_at_rule_variant(part) {
                media_queries.push(at_rule);
            } else if let Some(selector) = not_variant_selector(part) {
                wrappers.push(selector);
            } else if let Some(selector) = group_peer_attribute_variant_selector(part) {
                wrappers.push(selector);
            } else if let Some(selector) = named_group_peer_selector(part) {
                wrappers.push(selector);
            } else if let Some(selector) = arbitrary_group_peer_selector(part) {
                wrappers.push(selector);
            } else if let Some(selector) = arbitrary_selector_variant(part) {
                wrappers.push(selector);
            } else if let Some(selector) = child_selector_variant(part) {
                wrappers.push(selector.to_string());
            } else if let Some(selector) = in_variant_selector(part) {
                wrappers.push(selector);
            } else if let Some(selector) = structural_nth_variant(part) {
                wrappers.push(selector);
            } else if let Some(selector) =
                crate::core::engine::typography::typography_element_selector(part)
            {
                wrappers.push(format!("&{selector}"));
            } else {
                return None;
            }
        }
    }
    if not_hover_fallback && !saw_not_hover {
        return None;
    }
    Some((media_queries, pseudo_classes, wrappers))
}
