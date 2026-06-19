pub(super) fn contains_tailwind_css_function(input: &str) -> bool {
    input.contains("--alpha(") || input.contains("--spacing(")
}

pub(super) fn replace_tailwind_css_functions(input: &str) -> Option<String> {
    let spaced = replace_function_calls(input, "--spacing", resolve_spacing_function)?;
    replace_function_calls(&spaced, "--alpha", resolve_alpha_function)
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
    if !is_safe_css_value(color) {
        return None;
    }
    let opacity = normalize_alpha(opacity)?;
    Some(format!(
        "color-mix(in oklab, {color} {opacity}, transparent)"
    ))
}

fn normalize_alpha(value: &str) -> Option<String> {
    if is_percentage(value) {
        return Some(value.to_string());
    }
    if value.starts_with("var(") && value.ends_with(')') && is_safe_css_value(value) {
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

fn is_spacing_expression(expression: &str) -> bool {
    is_number(expression) || is_safe_css_value(expression)
}

pub(super) fn is_safe_css_value(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    !value.trim().is_empty()
        && !value.contains('{')
        && !value.contains('}')
        && !value.contains(';')
        && !lower.contains("@import")
        && !lower.contains("javascript:")
        && !lower.contains("expression(")
        && !lower.contains("</")
}

fn is_number(value: &str) -> bool {
    value.parse::<f64>().is_ok_and(f64::is_finite)
}

fn is_percentage(value: &str) -> bool {
    value
        .strip_suffix('%')
        .is_some_and(|number| number.parse::<f64>().is_ok_and(f64::is_finite))
}

fn normalize_internal_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
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
            _ if ch == needle && paren_depth == 0 => return Some(index),
            _ => {}
        }
    }

    None
}

fn trim_float(value: f64) -> String {
    let mut output = format!("{value:.4}");
    while output.contains('.') && output.ends_with('0') {
        output.pop();
    }
    if output.ends_with('.') {
        output.pop();
    }
    output
}
