/// balanced-page-config-exports scanner for Lane 11 Next custom-transform receipts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PageConfigExport {
    pub(super) name: String,
    pub(super) value_source: String,
    pub(super) value_kind: &'static str,
    pub(super) export_kind: &'static str,
    pub(super) compatibility_issue: Option<&'static str>,
}

const PAGE_CONFIG_EXPORT_NAMES: &[&str] = &[
    "config",
    "dynamic",
    "dynamicParams",
    "fetchCache",
    "revalidate",
    "runtime",
    "preferredRegion",
    "maxDuration",
    "experimental_ppr",
];

pub(super) fn collect_page_config_exports(source: &str) -> Vec<PageConfigExport> {
    let mut exports = Vec::new();
    collect_const_page_config_exports(source, &mut exports);
    collect_named_page_config_reexports(source, &mut exports);
    exports.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then(left.export_kind.cmp(right.export_kind))
            .then(left.value_source.cmp(&right.value_source))
    });
    exports.dedup_by(|left, right| {
        left.name == right.name
            && left.export_kind == right.export_kind
            && left.value_source == right.value_source
    });
    exports
}

fn collect_const_page_config_exports(source: &str, exports: &mut Vec<PageConfigExport>) {
    let mut cursor = 0usize;
    while let Some(export_index) = find_word(source, cursor, "export") {
        let after_export = export_index + "export".len();
        let Some(const_index) = next_non_ws(source, after_export).filter(|index| {
            source[*index..].starts_with("const")
                && word_boundary(source, *index, *index + "const".len())
        }) else {
            cursor = after_export;
            continue;
        };
        let after_const = const_index + "const".len();
        let Some((name, name_end)) = identifier_after(source, after_const) else {
            cursor = after_const;
            continue;
        };
        if !is_page_config_name(&name) {
            cursor = name_end;
            continue;
        }
        let Some(eq_index) = source[name_end..].find('=').map(|offset| name_end + offset) else {
            cursor = name_end;
            continue;
        };
        let (value_source, value_end) = read_export_value(source, eq_index + 1);
        let (value_kind, compatibility_issue) = classify_page_config_value(&name, &value_source);
        exports.push(PageConfigExport {
            name,
            value_source,
            value_kind,
            export_kind: "const",
            compatibility_issue,
        });
        cursor = value_end.max(eq_index + 1);
    }
}

fn collect_named_page_config_reexports(source: &str, exports: &mut Vec<PageConfigExport>) {
    let mut cursor = 0usize;
    while let Some(export_index) = find_word(source, cursor, "export") {
        let after_export = export_index + "export".len();
        let Some(brace_start) = next_non_ws(source, after_export).filter(|index| {
            source[*index..]
                .chars()
                .next()
                .is_some_and(|character| character == '{')
        }) else {
            cursor = after_export;
            continue;
        };
        let Some(brace_end) = find_balanced_delimiter(source, brace_start, '{', '}') else {
            cursor = brace_start + 1;
            continue;
        };
        let reexport_source = read_reexport_source(&source[brace_end + 1..]);
        for specifier in source[brace_start + 1..brace_end].split(',') {
            let Some(name) = page_config_name_from_specifier(specifier) else {
                continue;
            };
            let compatibility_issue = (name == "config").then_some("config-re-export");
            exports.push(PageConfigExport {
                name: name.to_string(),
                value_source: reexport_source
                    .as_ref()
                    .map(|module| format!("named-re-export from {module}"))
                    .unwrap_or_else(|| "named-re-export".to_string()),
                value_kind: "named-re-export",
                export_kind: "named-re-export",
                compatibility_issue,
            });
        }
        cursor = brace_end + 1;
    }
}

fn classify_page_config_value(
    name: &str,
    value_source: &str,
) -> (&'static str, Option<&'static str>) {
    let trimmed = value_source.trim();
    let value_kind = if trimmed.is_empty() {
        "missing-initializer"
    } else if trimmed.starts_with('{') {
        "object-literal"
    } else if trimmed.starts_with('[') {
        "array-literal"
    } else if is_string_literal(trimmed) {
        "string-literal"
    } else if trimmed == "true" || trimmed == "false" {
        "boolean-literal"
    } else if trimmed == "null" {
        "null-literal"
    } else if trimmed.parse::<f64>().is_ok() {
        "number-literal"
    } else {
        "identifier-or-expression"
    };

    let compatibility_issue = if name != "config" {
        None
    } else if value_kind != "object-literal" {
        Some("config-must-be-object")
    } else if contains_unquoted_spread(trimmed) {
        Some("config-object-spread-unsupported")
    } else {
        None
    };

    (value_kind, compatibility_issue)
}

fn page_config_name_from_specifier(specifier: &str) -> Option<&str> {
    let trimmed = specifier
        .trim()
        .strip_prefix("type ")
        .unwrap_or(specifier)
        .trim();
    if trimmed.is_empty() {
        return None;
    }
    let mut parts = trimmed.split_whitespace();
    let original = parts.next()?;
    match (parts.next(), parts.next()) {
        (Some("as"), Some(exported)) if is_page_config_name(exported) => Some(exported),
        (Some("as"), Some(_)) if is_page_config_name(original) => Some(original),
        _ if is_page_config_name(original) => Some(original),
        _ => None,
    }
}

fn read_export_value(source: &str, start: usize) -> (String, usize) {
    let Some(value_start) = next_non_ws(source, start) else {
        return (String::new(), start);
    };
    let mut cursor = value_start;
    let mut value_end = value_start;
    let mut quote = None;
    let mut depth = 0usize;
    let mut saw_value = false;

    while cursor < source.len() {
        let Some(character) = source[cursor..].chars().next() else {
            break;
        };
        if let Some(active_quote) = quote {
            if character == active_quote {
                quote = None;
            } else if character == '\\' {
                cursor += character.len_utf8();
                if cursor < source.len() {
                    cursor += source[cursor..]
                        .chars()
                        .next()
                        .map(char::len_utf8)
                        .unwrap_or_default();
                    value_end = cursor;
                    continue;
                }
            }
            cursor += character.len_utf8();
            value_end = cursor;
            continue;
        }

        match character {
            '"' | '\'' | '`' => {
                quote = Some(character);
                saw_value = true;
            }
            '(' | '[' | '{' => {
                depth += 1;
                saw_value = true;
            }
            ')' | ']' | '}' => {
                depth = depth.saturating_sub(1);
                saw_value = true;
            }
            ';' if depth == 0 => break,
            '\r' | '\n' if depth == 0 && saw_value => break,
            character if !character.is_whitespace() => saw_value = true,
            _ => {}
        }

        cursor += character.len_utf8();
        value_end = cursor;
    }

    (source[value_start..value_end].trim().to_string(), cursor)
}

fn read_reexport_source(after_brace: &str) -> Option<String> {
    let trimmed = after_brace.trim_start();
    let after_from = trimmed.strip_prefix("from")?.trim_start();
    let module = read_string_literal(after_from)?;
    Some(module)
}

fn read_string_literal(source: &str) -> Option<String> {
    let quote = source.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    let mut value = String::new();
    let mut escaped = false;
    for character in source[quote.len_utf8()..].chars() {
        if escaped {
            value.push(character);
            escaped = false;
            continue;
        }
        if character == '\\' {
            escaped = true;
            continue;
        }
        if character == quote {
            return Some(value);
        }
        value.push(character);
    }
    None
}

fn is_string_literal(source: &str) -> bool {
    read_string_literal(source)
        .map(|literal| literal.len() + 2 == source.len())
        .unwrap_or(false)
}

fn contains_unquoted_spread(source: &str) -> bool {
    let mut cursor = 0usize;
    let mut quote = None;
    while cursor < source.len() {
        let Some(character) = source[cursor..].chars().next() else {
            break;
        };
        if let Some(active_quote) = quote {
            if character == active_quote {
                quote = None;
            } else if character == '\\' {
                cursor += character.len_utf8();
                if cursor < source.len() {
                    cursor += source[cursor..]
                        .chars()
                        .next()
                        .map(char::len_utf8)
                        .unwrap_or_default();
                    continue;
                }
            }
            cursor += character.len_utf8();
            continue;
        }
        match character {
            '"' | '\'' | '`' => quote = Some(character),
            '.' if source[cursor..].starts_with("...") => return true,
            _ => {}
        }
        cursor += character.len_utf8();
    }
    false
}

fn find_word(source: &str, start: usize, word: &str) -> Option<usize> {
    let mut cursor = start;
    while let Some(offset) = source[cursor..].find(word) {
        let index = cursor + offset;
        let end = index + word.len();
        if word_boundary(source, index, end) {
            return Some(index);
        }
        cursor = end;
    }
    None
}

fn find_balanced_delimiter(
    source: &str,
    mut cursor: usize,
    open: char,
    close: char,
) -> Option<usize> {
    let mut quote = None;
    let mut depth = 0usize;
    while cursor < source.len() {
        let character = source[cursor..].chars().next()?;
        if let Some(active_quote) = quote {
            if character == active_quote {
                quote = None;
            } else if character == '\\' {
                cursor += character.len_utf8();
                if cursor < source.len() {
                    cursor += source[cursor..].chars().next()?.len_utf8();
                    continue;
                }
            }
            cursor += character.len_utf8();
            continue;
        }
        match character {
            '"' | '\'' | '`' => quote = Some(character),
            _ if character == open => depth += 1,
            _ if character == close => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(cursor);
                }
            }
            _ => {}
        }
        cursor += character.len_utf8();
    }
    None
}

fn identifier_after(source: &str, start: usize) -> Option<(String, usize)> {
    let mut cursor = next_non_ws(source, start)?;
    let first = source[cursor..].chars().next()?;
    if !is_identifier_start(first) {
        return None;
    }
    let mut name = String::new();
    name.push(first);
    cursor += first.len_utf8();
    while cursor < source.len() {
        let Some(character) = source[cursor..].chars().next() else {
            break;
        };
        if !is_identifier_continue(character) {
            break;
        }
        name.push(character);
        cursor += character.len_utf8();
    }
    Some((name, cursor))
}

fn next_non_ws(source: &str, mut cursor: usize) -> Option<usize> {
    while cursor < source.len() {
        let character = source[cursor..].chars().next()?;
        if !character.is_whitespace() {
            return Some(cursor);
        }
        cursor += character.len_utf8();
    }
    None
}

fn word_boundary(source: &str, start: usize, end: usize) -> bool {
    let before = source[..start]
        .chars()
        .last()
        .is_none_or(|character| !is_identifier_continue(character));
    let after = source[end..]
        .chars()
        .next()
        .is_none_or(|character| !is_identifier_continue(character));
    before && after
}

fn is_page_config_name(name: &str) -> bool {
    PAGE_CONFIG_EXPORT_NAMES.contains(&name)
}

fn is_identifier_start(character: char) -> bool {
    character == '_' || character == '$' || character.is_ascii_alphabetic()
}

fn is_identifier_continue(character: char) -> bool {
    is_identifier_start(character) || character.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_multiline_config_object_and_route_literals() {
        let exports = collect_page_config_exports(
            r#"export const config = {
  runtime: "edge",
  unstable_runtimeJS: false
};
export const revalidate = 60
export const dynamic = "force-dynamic";
"#,
        );

        assert_eq!(exports.len(), 3);
        assert!(
            exports
                .iter()
                .any(|export| export.name == "config" && export.value_kind == "object-literal")
        );
        assert!(
            exports
                .iter()
                .any(|export| export.name == "revalidate" && export.value_kind == "number-literal")
        );
        assert!(
            exports
                .iter()
                .any(|export| export.name == "dynamic" && export.value_kind == "string-literal")
        );
    }

    #[test]
    fn records_next_page_config_reexport_and_spread_issues() {
        let exports = collect_page_config_exports(
            r#"const shared = { amp: true };
export const config = { ...shared };
export { pageConfig as config } from "./config";
"#,
        );

        assert!(exports.iter().any(|export| {
            export.name == "config"
                && export.value_kind == "object-literal"
                && export.compatibility_issue == Some("config-object-spread-unsupported")
        }));
        assert!(exports.iter().any(|export| {
            export.name == "config"
                && export.value_kind == "named-re-export"
                && export.compatibility_issue == Some("config-re-export")
        }));
    }
}
