#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ReactStateBinding {
    pub name: String,
    pub setter: String,
    pub initial_source: String,
    pub initial_value: Option<i64>,
    pub value_kind: String,
}

pub(super) fn react_state_bindings(source: &str) -> Vec<ReactStateBinding> {
    let Ok(re) = regex::Regex::new(
        r#"\bconst\s*\[\s*([A-Za-z_$][A-Za-z0-9_$]*)\s*,\s*([A-Za-z_$][A-Za-z0-9_$]*)\s*\]\s*=\s*(?:React\.)?useState(?:<[^>]+>)?\s*\("#,
    ) else {
        return Vec::new();
    };
    re.captures_iter(source)
        .filter_map(|capture| {
            let full = capture.get(0)?;
            let open = full.end().checked_sub(1)?;
            let close = find_balanced(source, open, '(', ')')?;
            let initial_source = source[open + 1..close].trim().to_string();
            Some(ReactStateBinding {
                name: capture.get(1)?.as_str().to_string(),
                setter: capture.get(2)?.as_str().to_string(),
                initial_value: scalar_initial_value(&initial_source),
                value_kind: state_value_kind(&initial_source),
                initial_source,
            })
        })
        .collect()
}

pub(super) fn react_state_count(source: &str) -> usize {
    react_state_bindings(source).len()
}

pub(super) fn state_value_kind(value: &str) -> String {
    let scalar = scalar_initial_source(value);
    let trimmed = scalar.trim();
    if trimmed.parse::<i64>().is_ok() || trimmed.parse::<f64>().is_ok() {
        return "number".to_string();
    }
    if matches!(trimmed, "true" | "false") {
        return "boolean".to_string();
    }
    if trimmed.starts_with('"') || trimmed.starts_with('\'') || trimmed.starts_with('`') {
        return "string".to_string();
    }
    if trimmed.starts_with('[') {
        return "array".to_string();
    }
    if trimmed.starts_with('{') {
        return "object".to_string();
    }
    "unknown".to_string()
}

fn scalar_initial_value(value: &str) -> Option<i64> {
    let scalar = scalar_initial_source(value);
    match scalar.trim() {
        "true" => Some(1),
        "false" => Some(0),
        other => other.parse::<i64>().ok(),
    }
}

fn scalar_initial_source(value: &str) -> String {
    let trimmed = value.trim();
    let Some(arrow) = trimmed.find("=>") else {
        return trimmed.to_string();
    };
    let params = trimmed[..arrow].trim();
    if params == "()" || params == "( )" {
        return trimmed[arrow + 2..].trim().to_string();
    }
    trimmed.to_string()
}

fn find_balanced(source: &str, start: usize, open: char, close: char) -> Option<usize> {
    let mut depth = 0usize;
    let mut quote = None;
    let mut cursor = start;
    while cursor < source.len() {
        let ch = source[cursor..].chars().next()?;
        if let Some(active_quote) = quote {
            if ch == '\\' {
                cursor += ch.len_utf8();
                if cursor < source.len() {
                    cursor += source[cursor..].chars().next()?.len_utf8();
                }
                continue;
            }
            if ch == active_quote {
                quote = None;
            }
            cursor += ch.len_utf8();
            continue;
        }
        match ch {
            '"' | '\'' | '`' => quote = Some(ch),
            ch if ch == open => depth += 1,
            ch if ch == close => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(cursor);
                }
            }
            _ => {}
        }
        cursor += ch.len_utf8();
    }
    None
}
