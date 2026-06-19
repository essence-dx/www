use dx_compiler::delivery::parse_tsx_module;

/// balanced-font-loader-calls scanner for Lane 11 Next custom-transform receipts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::cli::app_router_execution) struct FontLoaderDetection {
    pub(in crate::cli::app_router_execution) loader: String,
    pub(in crate::cli::app_router_execution) imported: Option<String>,
    pub(in crate::cli::app_router_execution) local: String,
    pub(in crate::cli::app_router_execution) namespace_import: bool,
    pub(in crate::cli::app_router_execution) call_count: usize,
    pub(in crate::cli::app_router_execution) call_scope: &'static str,
    pub(in crate::cli::app_router_execution) module_scope: bool,
    pub(in crate::cli::app_router_execution) assigned_to_const: bool,
    pub(in crate::cli::app_router_execution) variable_names: Vec<String>,
    pub(in crate::cli::app_router_execution) css_variable_receipt: bool,
    pub(in crate::cli::app_router_execution) generated_css_import: bool,
    pub(in crate::cli::app_router_execution) compatibility_issue: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FontBinding {
    loader: String,
    imported: Option<String>,
    local: String,
    namespace_import: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FontCall {
    module_scope: bool,
    assigned_to_const: bool,
    variable_name: Option<String>,
    options_source: String,
}

pub(in crate::cli::app_router_execution) fn collect_font_loader_detections(
    source_path: &str,
    source: &str,
) -> Vec<FontLoaderDetection> {
    let mut detections = Vec::new();
    for binding in font_bindings(source_path, source) {
        let calls = font_calls(source, &binding.local);
        detections.push(FontLoaderDetection {
            loader: binding.loader,
            imported: binding.imported,
            local: binding.local,
            namespace_import: binding.namespace_import,
            call_count: calls.len(),
            call_scope: call_scope(&calls),
            module_scope: calls.iter().all(|call| call.module_scope),
            assigned_to_const: !calls.is_empty() && calls.iter().all(|call| call.assigned_to_const),
            variable_names: variable_names(&calls),
            css_variable_receipt: calls
                .iter()
                .any(|call| call.options_source.contains("variable")),
            generated_css_import: false,
            compatibility_issue: compatibility_issue(binding.namespace_import, &calls),
        });
    }
    detections.sort_by(|left, right| {
        left.loader
            .cmp(&right.loader)
            .then(left.local.cmp(&right.local))
    });
    detections.dedup_by(|left, right| left.loader == right.loader && left.local == right.local);
    detections
}

fn font_bindings(source_path: &str, source: &str) -> Vec<FontBinding> {
    let ast = parse_tsx_module(source_path, source);
    let mut bindings = Vec::new();
    for import in ast.imports {
        if !is_next_font_loader(&import.source) {
            continue;
        }
        for specifier in import
            .specifiers
            .into_iter()
            .filter(|specifier| !specifier.type_only)
        {
            bindings.push(FontBinding {
                loader: import.source.clone(),
                imported: Some(specifier.imported),
                local: specifier.local,
                namespace_import: false,
            });
        }
        if let Some(default) = import.default {
            bindings.push(FontBinding {
                loader: import.source,
                imported: None,
                local: default,
                namespace_import: false,
            });
        }
    }
    bindings.extend(namespace_font_imports(source));
    bindings
}

fn namespace_font_imports(source: &str) -> Vec<FontBinding> {
    let mut bindings = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("import * as ") {
            continue;
        }
        let Some((local_part, module_part)) = trimmed
            .trim_start_matches("import * as ")
            .split_once(" from ")
        else {
            continue;
        };
        let Some(loader) = read_string_literal(module_part.trim_end_matches(';').trim()) else {
            continue;
        };
        if !is_next_font_loader(&loader) {
            continue;
        }
        bindings.push(FontBinding {
            loader,
            imported: None,
            local: local_part.trim().to_string(),
            namespace_import: true,
        });
    }
    bindings
}

fn font_calls(source: &str, binding: &str) -> Vec<FontCall> {
    let mut calls = Vec::new();
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
        let assignment = assignment_before_call(source, binding_index);
        calls.push(FontCall {
            module_scope: brace_depth_before(source, binding_index) == 0,
            assigned_to_const: assignment
                .as_ref()
                .is_some_and(|assignment| assignment.kind == "const"),
            variable_name: assignment.map(|assignment| assignment.variable),
            options_source: read_call_arguments(source, args_start, args_end),
        });
        cursor = args_end + 1;
    }
    calls
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FontAssignment {
    kind: &'static str,
    variable: String,
}

fn assignment_before_call(source: &str, call_index: usize) -> Option<FontAssignment> {
    let statement_start = source[..call_index]
        .rfind([';', '\r', '\n'])
        .map(|index| index + 1)
        .unwrap_or(0);
    let prefix = source[statement_start..call_index].trim();
    let (left, _) = prefix.split_once('=')?;
    let left = left.trim().trim_start_matches("export ").trim();
    for kind in ["const", "let", "var"] {
        let Some(variable) = left.strip_prefix(kind).map(str::trim) else {
            continue;
        };
        if is_identifier_like(variable) {
            return Some(FontAssignment {
                kind,
                variable: variable.to_string(),
            });
        }
    }
    None
}

fn read_call_arguments(source: &str, args_start: usize, args_end: usize) -> String {
    source[args_start + 1..args_end].trim().to_string()
}

fn call_scope(calls: &[FontCall]) -> &'static str {
    if calls.is_empty() {
        "not-called"
    } else if calls.iter().all(|call| call.module_scope) {
        "module"
    } else if calls.iter().all(|call| !call.module_scope) {
        "nested"
    } else {
        "mixed"
    }
}

fn variable_names(calls: &[FontCall]) -> Vec<String> {
    let mut names = calls
        .iter()
        .filter_map(|call| call.variable_name.clone())
        .collect::<Vec<_>>();
    names.sort();
    names.dedup();
    names
}

fn compatibility_issue(namespace_import: bool, calls: &[FontCall]) -> Option<&'static str> {
    if namespace_import {
        return Some("font-loader-namespace-import");
    }
    if calls.iter().any(|call| !call.module_scope) {
        return Some("font-loader-call-outside-module-scope");
    }
    if calls.iter().any(|call| !call.assigned_to_const) {
        return Some("font-loader-call-must-be-const");
    }
    if calls.iter().any(|call| call.options_source.contains("...")) {
        return Some("font-loader-options-spread-unsupported");
    }
    None
}

fn is_next_font_loader(source: &str) -> bool {
    source == "next/font/google" || source == "next/font/local"
}

fn brace_depth_before(source: &str, end: usize) -> usize {
    let mut depth = 0usize;
    let mut cursor = 0usize;
    let mut quote = None;
    while cursor < end {
        let Some(character) = source[cursor..].chars().next() else {
            break;
        };
        if let Some(active_quote) = quote {
            if character == active_quote {
                quote = None;
            } else if character == '\\' {
                cursor += character.len_utf8();
                if cursor < end {
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
            '{' => depth += 1,
            '}' => depth = depth.saturating_sub(1),
            _ => {}
        }
        cursor += character.len_utf8();
    }
    depth
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
    fn records_module_scope_google_and_local_font_calls() {
        let fonts = collect_font_loader_detections(
            "app/layout.tsx",
            r#"import { Inter } from "next/font/google";
import localFont from "next/font/local";

export const inter = Inter({ subsets: ["latin"], variable: "--font-inter" });
const brand = localFont({ src: "./brand.woff2" });
"#,
        );

        assert_eq!(fonts.len(), 2);
        assert!(fonts.iter().any(|font| {
            font.loader == "next/font/google"
                && font.local == "Inter"
                && font.call_count == 1
                && font.call_scope == "module"
                && font.css_variable_receipt
                && font.variable_names == vec!["inter".to_string()]
        }));
        assert!(fonts.iter().any(|font| {
            font.loader == "next/font/local"
                && font.local == "localFont"
                && font.call_count == 1
                && font.variable_names == vec!["brand".to_string()]
        }));
    }

    #[test]
    fn records_namespace_and_nested_call_issues_without_generating_css() {
        let fonts = collect_font_loader_detections(
            "app/page.tsx",
            r#"import * as fonts from "next/font/google";
import { Roboto } from "next/font/google";

export default function Page() {
  const roboto = Roboto({ subsets: ["latin"] });
  return roboto.className;
}
"#,
        );

        assert!(fonts.iter().any(|font| {
            font.namespace_import
                && font.compatibility_issue == Some("font-loader-namespace-import")
        }));
        assert!(fonts.iter().any(|font| {
            font.local == "Roboto"
                && font.call_scope == "nested"
                && font.compatibility_issue == Some("font-loader-call-outside-module-scope")
                && !font.generated_css_import
        }));
    }
}
