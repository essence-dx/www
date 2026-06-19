use dx_compiler::delivery::parse_tsx_module;

const TRACK_DYNAMIC_IMPORT_HELPER: &str = "private-next-rsc-track-dynamic-import";
const LOADABLE_GENERATED_FIELD: &str = "loadableGenerated";

/// balanced-dynamic-imports scanner for Lane 11 Next custom-transform receipts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DynamicImportDetection {
    pub(super) call: &'static str,
    pub(super) specifier: Option<String>,
    pub(super) binding_name: Option<String>,
    pub(super) call_count: usize,
    pub(super) tracked_export_names: Vec<String>,
    pub(super) ssr_false: bool,
    pub(super) loadable_generated_added: bool,
    pub(super) loadable_generated_field: &'static str,
    pub(super) transition_added: bool,
    pub(super) track_helper: &'static str,
    pub(super) compatibility_issue: Option<&'static str>,
}

struct ImportCall {
    start: usize,
    specifier: Option<String>,
}

pub(super) fn collect_dynamic_import_detections(
    source_path: &str,
    source: &str,
) -> Vec<DynamicImportDetection> {
    let mut detections = Vec::new();

    for import_call in import_calls(source) {
        detections.push(DynamicImportDetection {
            call: "import()",
            specifier: import_call.specifier,
            binding_name: None,
            call_count: 1,
            tracked_export_names: tracked_export_names(source, import_call.start),
            ssr_false: false,
            loadable_generated_added: false,
            loadable_generated_field: LOADABLE_GENERATED_FIELD,
            transition_added: false,
            track_helper: TRACK_DYNAMIC_IMPORT_HELPER,
            compatibility_issue: None,
        });
    }

    for binding in next_dynamic_bindings(source_path, source) {
        detections.extend(next_dynamic_calls(source, &binding));
    }

    detections
}

fn next_dynamic_bindings(source_path: &str, source: &str) -> Vec<String> {
    let ast = parse_tsx_module(source_path, source);
    let mut bindings = ast
        .imports
        .into_iter()
        .filter(|import| import.source == "next/dynamic")
        .filter_map(|import| import.default)
        .collect::<Vec<_>>();
    bindings.sort();
    bindings.dedup();
    bindings
}

fn next_dynamic_calls(source: &str, binding: &str) -> Vec<DynamicImportDetection> {
    let mut detections = Vec::new();
    let mut cursor = 0usize;
    while let Some(binding_index) = find_word(source, cursor, binding) {
        let after_binding = binding_index + binding.len();
        let Some(args_start) = next_non_ws(source, after_binding).filter(|index| {
            source[*index..]
                .chars()
                .next()
                .is_some_and(|character| character == '(')
        }) else {
            cursor = after_binding;
            continue;
        };
        let Some(args_end) = find_balanced_delimiter(source, args_start, '(', ')') else {
            cursor = args_start + 1;
            continue;
        };
        let arguments = split_top_level_args(&source[args_start + 1..args_end]);
        let first_arg = arguments.first().copied().unwrap_or_default();
        let second_arg = arguments.get(1).copied();
        let compatibility_issue = next_dynamic_compatibility_issue(&arguments);
        detections.push(DynamicImportDetection {
            call: "next/dynamic",
            specifier: first_import_specifier(first_arg),
            binding_name: Some(binding.to_string()),
            call_count: 1,
            tracked_export_names: Vec::new(),
            ssr_false: second_arg.is_some_and(option_has_ssr_false),
            loadable_generated_added: false,
            loadable_generated_field: LOADABLE_GENERATED_FIELD,
            transition_added: false,
            track_helper: TRACK_DYNAMIC_IMPORT_HELPER,
            compatibility_issue,
        });
        cursor = args_end + 1;
    }
    detections
}

fn import_calls(source: &str) -> Vec<ImportCall> {
    let mut calls = Vec::new();
    let mut cursor = 0usize;
    while let Some(import_index) = find_word(source, cursor, "import") {
        let after_import = import_index + "import".len();
        let Some(args_start) = next_non_ws(source, after_import).filter(|index| {
            source[*index..]
                .chars()
                .next()
                .is_some_and(|character| character == '(')
        }) else {
            cursor = after_import;
            continue;
        };
        let Some(args_end) = find_balanced_delimiter(source, args_start, '(', ')') else {
            cursor = args_start + 1;
            continue;
        };
        let arguments = split_top_level_args(&source[args_start + 1..args_end]);
        calls.push(ImportCall {
            start: import_index,
            specifier: arguments
                .first()
                .and_then(|argument| read_safe_string_literal(argument)),
        });
        cursor = args_end + 1;
    }
    calls
}

fn first_import_specifier(source: &str) -> Option<String> {
    import_calls(source)
        .into_iter()
        .find_map(|call| call.specifier)
}

fn next_dynamic_compatibility_issue(arguments: &[&str]) -> Option<&'static str> {
    if arguments.is_empty() {
        return Some("next-dynamic-missing-loader");
    }
    if arguments.len() > 2 {
        return Some("next-dynamic-too-many-args");
    }
    let options = arguments.get(1).map(|value| value.trim())?;
    if !options.starts_with('{') {
        return Some("next-dynamic-options-must-be-object");
    }
    None
}

fn option_has_ssr_false(source: &str) -> bool {
    source.contains("ssr: false")
        || source.contains("\"ssr\": false")
        || source.contains("'ssr': false")
}

fn tracked_export_names(source: &str, import_index: usize) -> Vec<String> {
    let line_start = source[..import_index]
        .rfind(['\r', '\n'])
        .map(|index| index + 1)
        .unwrap_or(0);
    let prefix = &source[line_start..import_index];
    if !prefix.contains("await") {
        return Vec::new();
    }
    let Some(eq_index) = prefix.rfind('=') else {
        return Vec::new();
    };
    let pattern = &prefix[..eq_index];
    let Some(open_index) = pattern.find('{') else {
        return Vec::new();
    };
    let Some(close_index) = pattern.rfind('}').filter(|index| *index > open_index) else {
        return Vec::new();
    };
    extract_object_pattern_names(&pattern[open_index + 1..close_index]).unwrap_or_default()
}

fn extract_object_pattern_names(pattern: &str) -> Option<Vec<String>> {
    let mut names = Vec::new();
    for part in pattern.split(',') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with("...") || trimmed.starts_with('[') {
            return None;
        }
        let name = trimmed
            .split_once(':')
            .map(|(left, _)| left.trim())
            .unwrap_or(trimmed);
        let name = name.trim_matches('"').trim_matches('\'');
        if !is_identifier_like(name) {
            return None;
        }
        names.push(name.to_string());
    }
    names.sort();
    names.dedup();
    Some(names)
}

fn split_top_level_args(source: &str) -> Vec<&str> {
    let mut args = Vec::new();
    let mut start = 0usize;
    let mut cursor = 0usize;
    let mut quote = None;
    let mut depth = 0usize;
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
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                args.push(source[start..cursor].trim());
                start = cursor + character.len_utf8();
            }
            _ => {}
        }
        cursor += character.len_utf8();
    }
    let trailing = source[start..].trim();
    if !trailing.is_empty() {
        args.push(trailing);
    }
    args
}

fn read_safe_string_literal(source: &str) -> Option<String> {
    let trimmed = source.trim();
    let quote = trimmed.chars().next()?;
    if quote != '"' && quote != '\'' && quote != '`' {
        return None;
    }
    let mut value = String::new();
    let mut escaped = false;
    for character in trimmed[quote.len_utf8()..].chars() {
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
            if quote == '`' && value.contains("${") {
                return None;
            }
            return Some(value);
        }
        value.push(character);
    }
    None
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

fn is_identifier_like(value: &str) -> bool {
    let Some(first) = value.chars().next() else {
        return false;
    };
    (first == '_' || first == '$' || first.is_ascii_alphabetic())
        && value.chars().all(is_identifier_continue)
}

fn is_identifier_continue(character: char) -> bool {
    character == '_' || character == '$' || character.is_ascii_alphanumeric()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracks_awaited_dynamic_import_exports_without_wrapping_runtime() {
        let imports = collect_dynamic_import_detections(
            "app/page.tsx",
            r#"export async function loader() {
  const { Chart, default: DefaultChart } = await import("./Chart");
  return { Chart, DefaultChart };
}"#,
        );

        assert_eq!(imports.len(), 1);
        assert_eq!(imports[0].call, "import()");
        assert_eq!(imports[0].specifier.as_deref(), Some("./Chart"));
        assert_eq!(
            imports[0].tracked_export_names,
            vec!["Chart".to_string(), "default".to_string()]
        );
        assert!(!imports[0].transition_added);
        assert!(!imports[0].loadable_generated_added);
    }

    #[test]
    fn records_next_dynamic_options_without_generating_loadable_metadata() {
        let imports = collect_dynamic_import_detections(
            "app/page.tsx",
            r#"import dynamic from "next/dynamic";
const Chart = dynamic(() => import("./Chart"), { ssr: false });
const Broken = dynamic(() => import("./Broken"), getOptions());
"#,
        );

        assert_eq!(
            imports
                .iter()
                .filter(|item| item.call == "next/dynamic")
                .count(),
            2
        );
        assert!(imports.iter().any(|item| {
            item.call == "next/dynamic"
                && item.specifier.as_deref() == Some("./Chart")
                && item.ssr_false
                && item.binding_name.as_deref() == Some("dynamic")
        }));
        assert!(imports.iter().any(|item| {
            item.call == "next/dynamic"
                && item.specifier.as_deref() == Some("./Broken")
                && item.compatibility_issue == Some("next-dynamic-options-must-be-object")
        }));
    }
}
