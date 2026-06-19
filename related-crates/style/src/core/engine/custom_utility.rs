use super::{ThemeDefinition, theme_css::CssUtilityDefinition};

#[derive(Clone, Copy)]
pub(super) struct CustomUtilityContext<'a> {
    pub value: Option<&'a str>,
    pub modifier: Option<&'a str>,
    pub themes: &'a [ThemeDefinition],
}

pub(super) fn declarations_to_css(
    declarations: &[(String, String)],
    context: CustomUtilityContext<'_>,
) -> Option<String> {
    let mut resolved = Vec::new();
    let mut value_function_seen = false;
    let mut value_function_resolved = false;

    for (property, value) in declarations {
        let has_value_function = value.contains("--value(");
        value_function_seen |= has_value_function;
        let Some(value) = resolve_custom_utility_value(value, context) else {
            continue;
        };
        value_function_resolved |= has_value_function;
        resolved.push(format!("{property}: {value}"));
    }

    if value_function_seen && context.value.is_some() && !value_function_resolved {
        return None;
    }

    if resolved.is_empty() {
        None
    } else {
        Some(resolved.join("; "))
    }
}

pub(super) fn utility_to_css(
    utility: &CssUtilityDefinition,
    context: CustomUtilityContext<'_>,
) -> Option<String> {
    let base_css = declarations_to_css(&utility.declarations, context);
    let mut nested_css = Vec::new();

    for rule in &utility.nested_rules {
        if let Some(declarations) = declarations_to_css(&rule.declarations, context) {
            nested_css.push(format!("NEST|{}|{}", rule.selector_suffix, declarations));
        }
    }

    if nested_css.is_empty() {
        return base_css;
    }

    let mut lines = Vec::with_capacity(nested_css.len() + 1);
    if let Some(base_css) = base_css {
        lines.push(format!("BASE|{base_css}"));
    }
    lines.extend(nested_css);
    Some(lines.join("\n"))
}

pub(super) fn utility_uses_modifier(utility: &CssUtilityDefinition) -> bool {
    utility
        .declarations
        .iter()
        .chain(
            utility
                .nested_rules
                .iter()
                .flat_map(|rule| rule.declarations.iter()),
        )
        .any(|(_, value)| value.contains("--modifier("))
}

pub(super) fn split_modifier(input: &str) -> (&str, Option<&str>) {
    let Some(index) = top_level_char(input, '/') else {
        return (input, None);
    };
    let (base, modifier) = input.split_at(index);
    let modifier = &modifier[1..];
    if base.is_empty() || modifier.is_empty() {
        (input, None)
    } else {
        (base, Some(modifier))
    }
}

fn resolve_custom_utility_value(value: &str, context: CustomUtilityContext<'_>) -> Option<String> {
    let mut output = replace_function_calls(value, "--value", |argument| {
        resolve_argument_function(argument, context.value, context.themes)
    })?;
    output = replace_function_calls(&output, "--modifier", |argument| {
        resolve_argument_function(argument, context.modifier, context.themes)
    })?;

    if output.contains("--default(") {
        return None;
    }

    output = replace_function_calls(&output, "--spacing", resolve_spacing_function)?;
    output = replace_function_calls(&output, "--alpha", resolve_alpha_function)?;

    if is_custom_utility_safe_value(&output) {
        Some(output)
    } else {
        None
    }
}

fn resolve_argument_function(
    argument: &str,
    candidate: Option<&str>,
    themes: &[ThemeDefinition],
) -> Option<String> {
    let arguments = split_top_level_commas(argument);
    let candidate = candidate.filter(|value| !value.is_empty());

    if candidate.is_none() {
        return arguments
            .iter()
            .find_map(|argument| resolve_default_argument(argument));
    }

    let candidate = candidate?;
    arguments
        .iter()
        .filter(|argument| !argument.trim().starts_with("--default("))
        .find_map(|argument| resolve_match_argument(argument, candidate, themes))
}

fn resolve_default_argument(argument: &str) -> Option<String> {
    let argument = argument.trim();
    let body = argument.strip_prefix("--default(")?;
    let body = strip_function_body(body)?;
    let body = body.trim();
    let value = unquote_literal(body).unwrap_or(body);
    if is_custom_utility_safe_value(value) {
        Some(value.to_string())
    } else {
        None
    }
}

fn resolve_match_argument(
    argument: &str,
    candidate: &str,
    themes: &[ThemeDefinition],
) -> Option<String> {
    let argument = argument.trim();

    if let Some(literal) = unquote_literal(argument) {
        if candidate == literal {
            return Some(literal.to_string());
        }
        return None;
    }

    if argument.starts_with("--") && argument.contains('*') {
        return resolve_theme_argument(argument, candidate, themes);
    }

    if let Some(inner_type) = argument
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        let arbitrary = candidate
            .strip_prefix('[')
            .and_then(|value| value.strip_suffix(']'))?;
        let value = decode_arbitrary_value(arbitrary);
        if supports_value_type(inner_type.trim(), &value) {
            return Some(value);
        }
        return None;
    }

    if supports_value_type(argument, candidate) {
        Some(candidate.to_string())
    } else {
        None
    }
}

fn resolve_theme_argument(
    pattern: &str,
    candidate: &str,
    themes: &[ThemeDefinition],
) -> Option<String> {
    if !is_theme_key_suffix(candidate) {
        return None;
    }

    let token = pattern.replace('*', candidate);
    if themes
        .iter()
        .rev()
        .any(|theme| theme.tokens.iter().any(|(name, _)| name == &token))
    {
        Some(format!("var({token})"))
    } else {
        None
    }
}

fn replace_function_calls<F>(input: &str, function_name: &str, mut resolver: F) -> Option<String>
where
    F: FnMut(&str) -> Option<String>,
{
    let needle = format!("{function_name}(");
    let mut output = String::new();
    let mut rest = input;

    while let Some(start) = rest.find(&needle) {
        output.push_str(&rest[..start]);
        let open = start + function_name.len();
        let close = matching_paren(rest, open)?;
        let argument = &rest[open + 1..close];
        output.push_str(&resolver(argument.trim())?);
        rest = &rest[close + 1..];
    }

    output.push_str(rest);
    Some(output)
}

fn resolve_spacing_function(argument: &str) -> Option<String> {
    let expression = normalize_internal_whitespace(argument.trim());
    if !is_spacing_expression(&expression) {
        return None;
    }
    Some(format!("calc(var(--spacing) * {expression})"))
}

fn resolve_alpha_function(argument: &str) -> Option<String> {
    let slash = top_level_char(argument, '/')?;
    let color = argument[..slash].trim();
    let opacity = argument[slash + 1..].trim();
    if !is_custom_utility_safe_value(color) {
        return None;
    }
    let opacity = normalize_alpha(opacity)?;
    Some(format!(
        "color-mix(in oklab, {color} {opacity}, transparent)"
    ))
}

fn normalize_alpha(value: &str) -> Option<String> {
    if is_custom_utility_percentage(value) {
        return Some(value.to_string());
    }
    if value.starts_with("var(") && value.ends_with(')') && is_custom_utility_safe_value(value) {
        return Some(value.to_string());
    }
    let number = value.parse::<f64>().ok()?;
    if !number.is_finite() || number < 0.0 {
        return None;
    }
    let percent = if number <= 1.0 {
        number * 100.0
    } else if number <= 100.0 {
        number
    } else {
        return None;
    };
    Some(format!("{}%", trim_float(percent)))
}

fn supports_value_type(value_type: &str, value: &str) -> bool {
    match value_type.trim() {
        "*" => is_custom_utility_safe_value(value),
        "integer" => is_custom_utility_integer(value),
        "number" => is_custom_utility_number(value),
        "percentage" => is_custom_utility_percentage(value),
        "length" => is_custom_utility_length(value),
        "ratio" => is_custom_utility_ratio(value),
        "angle" => is_custom_utility_angle(value),
        "line-width" => {
            matches!(value, "thin" | "medium" | "thick") || is_custom_utility_length(value)
        }
        "absolute-size" | "bg-size" | "color" | "family-name" | "generic-name" | "image"
        | "position" | "relative-size" | "url" | "vector" => is_custom_utility_safe_value(value),
        _ => false,
    }
}

fn is_custom_utility_integer(value: &str) -> bool {
    let value = value.strip_prefix('-').unwrap_or(value);
    !value.is_empty() && value.chars().all(|ch| ch.is_ascii_digit())
}

fn is_custom_utility_number(value: &str) -> bool {
    !value.is_empty()
        && value
            .parse::<f64>()
            .ok()
            .is_some_and(|number| number.is_finite())
}

fn is_custom_utility_length(value: &str) -> bool {
    value == "0"
        || value.starts_with("calc(") && value.ends_with(')')
        || [
            "px", "rem", "em", "vh", "vw", "vmin", "vmax", "ch", "ex", "lh", "rlh", "svh", "lvh",
            "dvh", "cqw", "cqh", "cqi", "cqb", "cqmin", "cqmax",
        ]
        .iter()
        .any(|unit| {
            value
                .strip_suffix(unit)
                .is_some_and(is_custom_utility_number)
        })
}

fn is_custom_utility_percentage(value: &str) -> bool {
    value
        .strip_suffix('%')
        .is_some_and(is_custom_utility_number)
}

fn is_custom_utility_ratio(value: &str) -> bool {
    let Some(index) = top_level_char(value, '/') else {
        return false;
    };
    is_custom_utility_number(value[..index].trim())
        && is_custom_utility_number(value[index + 1..].trim())
}

fn is_custom_utility_angle(value: &str) -> bool {
    value == "0"
        || ["deg", "rad", "grad", "turn"].iter().any(|unit| {
            value
                .strip_suffix(unit)
                .is_some_and(is_custom_utility_number)
        })
}

fn is_spacing_expression(value: &str) -> bool {
    !value.is_empty()
        && value.chars().all(|ch| {
            ch.is_ascii_digit() || matches!(ch, '.' | '+' | '-' | '*' | '/' | '(' | ')' | ' ')
        })
        && value.chars().any(|ch| ch.is_ascii_digit())
}

fn is_custom_utility_safe_value(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    !value.is_empty()
        && !value.contains('{')
        && !value.contains('}')
        && !value.contains(';')
        && !lower.contains("@import")
        && !lower.contains("javascript:")
        && !lower.contains("expression(")
        && !lower.contains("</")
}

fn is_theme_key_suffix(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
}

fn matching_paren(input: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    let mut quote = None;
    let mut escaped = false;

    for (index, ch) in input.char_indices().skip_while(|(index, _)| *index < open) {
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
            '(' => depth = depth.saturating_add(1),
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }

    None
}

fn top_level_char(input: &str, needle: char) -> Option<usize> {
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut quote = None;
    let mut escaped = false;

    for (index, ch) in input.char_indices() {
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
            '(' => paren_depth = paren_depth.saturating_add(1),
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '[' => bracket_depth = bracket_depth.saturating_add(1),
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            _ if ch == needle && paren_depth == 0 && bracket_depth == 0 => return Some(index),
            _ => {}
        }
    }

    None
}

fn split_top_level_commas(input: &str) -> Vec<&str> {
    let mut arguments = Vec::new();
    let mut start = 0usize;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut quote = None;
    let mut escaped = false;

    for (index, ch) in input.char_indices() {
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
            '(' => paren_depth = paren_depth.saturating_add(1),
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '[' => bracket_depth = bracket_depth.saturating_add(1),
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            ',' if paren_depth == 0 && bracket_depth == 0 => {
                arguments.push(input[start..index].trim());
                start = index + 1;
            }
            _ => {}
        }
    }

    arguments.push(input[start..].trim());
    arguments
}

fn strip_function_body(input: &str) -> Option<&str> {
    input.strip_suffix(')')
}

fn unquote_literal(input: &str) -> Option<&str> {
    let quote = input.chars().next()?;
    if !matches!(quote, '"' | '\'') || !input.ends_with(quote) {
        return None;
    }
    Some(&input[quote.len_utf8()..input.len() - quote.len_utf8()])
}

fn decode_arbitrary_value(input: &str) -> String {
    let mut output = String::new();
    let mut escaped = false;
    for ch in input.chars() {
        if escaped {
            output.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '_' {
            output.push(' ');
        } else {
            output.push(ch);
        }
    }
    if escaped {
        output.push('\\');
    }
    output
}

fn normalize_internal_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn trim_float(value: f64) -> String {
    let mut output = format!("{value:.6}");
    while output.contains('.') && output.ends_with('0') {
        output.pop();
    }
    if output.ends_with('.') {
        output.pop();
    }
    output
}
