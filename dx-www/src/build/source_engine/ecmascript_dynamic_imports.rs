use super::graph::{
    SourceBuildEcmascriptDynamicImport, SourceBuildEcmascriptUnresolvedDynamicImport,
    SourceBuildEcmascriptUnsupportedDynamicImport,
};

#[derive(Debug, Clone, Default)]
pub struct SourceBuildEcmascriptDynamicImports {
    pub dynamic_imports: Vec<SourceBuildEcmascriptDynamicImport>,
    pub unresolved_dynamic_imports: Vec<SourceBuildEcmascriptUnresolvedDynamicImport>,
    pub unsupported_dynamic_imports: Vec<SourceBuildEcmascriptUnsupportedDynamicImport>,
}

pub fn collect_dynamic_imports(source: &str) -> SourceBuildEcmascriptDynamicImports {
    let mut imports = SourceBuildEcmascriptDynamicImports::default();

    for call in scan_dynamic_import_calls(source) {
        if let Some(dynamic_import) = call.dynamic_import {
            imports.dynamic_imports.push(dynamic_import);
        }
        if let Some(unresolved_dynamic_import) = call.unresolved_dynamic_import {
            imports
                .unresolved_dynamic_imports
                .push(unresolved_dynamic_import);
        }
        if let Some(unsupported_dynamic_import) = call.unsupported_dynamic_import {
            imports
                .unsupported_dynamic_imports
                .push(unsupported_dynamic_import);
        }
    }

    imports
}

#[derive(Debug)]
struct ClassifiedDynamicImportCall {
    dynamic_import: Option<SourceBuildEcmascriptDynamicImport>,
    unresolved_dynamic_import: Option<SourceBuildEcmascriptUnresolvedDynamicImport>,
    unsupported_dynamic_import: Option<SourceBuildEcmascriptUnsupportedDynamicImport>,
    next_index: usize,
}

fn scan_dynamic_import_calls(source: &str) -> Vec<ClassifiedDynamicImportCall> {
    let mut calls = Vec::new();
    let mut index = 0usize;

    while index < source.len() {
        let Some(byte) = source.as_bytes().get(index).copied() else {
            break;
        };

        if let Some(next) = skip_quoted_or_comment(source, index, byte) {
            index = next;
            continue;
        }

        if source[index..].starts_with("import")
            && !previous_is_identifier(source, index)
            && !next_is_identifier(source, index + "import".len())
        {
            if let Some(call) = classify_dynamic_import_call(source, index) {
                index = call.next_index;
                calls.push(call);
                continue;
            }
        }

        index += source[index..]
            .chars()
            .next()
            .map(char::len_utf8)
            .unwrap_or(1);
    }

    calls
}

fn classify_dynamic_import_call(
    source: &str,
    import_index: usize,
) -> Option<ClassifiedDynamicImportCall> {
    let open = skip_ascii_ws(source, import_index + "import".len());
    if source.as_bytes().get(open).copied() != Some(b'(') {
        return None;
    }
    let close = find_dynamic_import_close(source, open);
    let dynamic_import =
        close.and_then(|close| parse_dynamic_import(source, import_index, open, close));
    let unsupported_dynamic_import =
        parse_unsupported_dynamic_import(source, import_index, open, close);
    let unresolved_dynamic_import = if dynamic_import.is_none()
        && unsupported_dynamic_import.is_none()
    {
        close.and_then(|close| parse_unresolved_dynamic_import(source, import_index, open, close))
    } else {
        None
    };

    if dynamic_import.is_none()
        && unresolved_dynamic_import.is_none()
        && unsupported_dynamic_import.is_none()
    {
        return None;
    }

    Some(ClassifiedDynamicImportCall {
        dynamic_import,
        unresolved_dynamic_import,
        unsupported_dynamic_import,
        next_index: close
            .map(|close| close + 1)
            .unwrap_or_else(|| next_line_start(source, import_index)),
    })
}

fn next_line_start(source: &str, index: usize) -> usize {
    source[index..]
        .find('\n')
        .map(|offset| index + offset + 1)
        .unwrap_or_else(|| index + "import".len())
        .min(source.len())
}

fn parse_dynamic_import(
    source: &str,
    import_index: usize,
    open: usize,
    close: usize,
) -> Option<SourceBuildEcmascriptDynamicImport> {
    let (specifier, _literal_end) = static_dynamic_import_literal(source, open)?;
    let (line, column) = line_column(source, import_index);
    let import_options_present = dynamic_import_has_options(source, open, close);
    Some(SourceBuildEcmascriptDynamicImport {
        specifier,
        kind: "esm-dynamic-import".to_string(),
        chunking: "async-boundary-reference".to_string(),
        line,
        column,
        import_options_present,
        import_options_supported: !import_options_present,
        node_modules_required: false,
    })
}

fn parse_unresolved_dynamic_import(
    source: &str,
    import_index: usize,
    open: usize,
    close: usize,
) -> Option<SourceBuildEcmascriptUnresolvedDynamicImport> {
    let expression_start = skip_ascii_ws_and_comments(source, open + 1);
    if expression_start >= close {
        return None;
    }

    let expression = expression_preview(&source[expression_start..close]);
    if expression.is_empty() {
        return None;
    }

    let (line, column) = line_column(source, import_index);
    Some(SourceBuildEcmascriptUnresolvedDynamicImport {
        expression,
        kind: "esm-dynamic-import-unresolved".to_string(),
        reason: "non-static-dynamic-import-expression".to_string(),
        line,
        column,
        node_modules_required: false,
    })
}

fn parse_unsupported_dynamic_import(
    source: &str,
    import_index: usize,
    open: usize,
    close: Option<usize>,
) -> Option<SourceBuildEcmascriptUnsupportedDynamicImport> {
    let expression_start = skip_ascii_ws_and_comments(source, open + 1);
    let (expression, reason) = match close {
        Some(close) if expression_start >= close => {
            (String::new(), "unsupported-dynamic-import-empty-expression")
        }
        Some(close) if dynamic_import_has_options(source, open, close) => (
            dynamic_import_options_preview(source, open, close),
            "unsupported-dynamic-import-options",
        ),
        Some(_) => return None,
        None => (
            expression_preview(&source[expression_start..]),
            "unsupported-dynamic-import-unclosed-call",
        ),
    };

    let (line, column) = line_column(source, import_index);
    Some(SourceBuildEcmascriptUnsupportedDynamicImport {
        expression,
        kind: "esm-dynamic-import-unsupported".to_string(),
        reason: reason.to_string(),
        line,
        column,
        node_modules_required: false,
    })
}

fn static_dynamic_import_literal(source: &str, open: usize) -> Option<(String, usize)> {
    let specifier_start = skip_ascii_ws_and_comments(source, open + 1);
    let quote = source.as_bytes().get(specifier_start).copied()?;
    if !matches!(quote, b'"' | b'\'' | b'`') {
        return None;
    }

    let mut escaped = false;
    let mut cursor = specifier_start + 1;
    while cursor < source.len() {
        let byte = source.as_bytes()[cursor];
        if escaped {
            escaped = false;
        } else if byte == b'\\' {
            escaped = true;
        } else if byte == quote {
            let specifier = source[specifier_start + 1..cursor].to_string();
            if quote == b'`' && specifier.contains("${") {
                return None;
            }
            return Some((specifier, cursor + 1));
        }
        cursor += 1;
    }

    None
}

fn dynamic_import_has_options(source: &str, open: usize, close: usize) -> bool {
    let Some((_specifier, literal_end)) = static_dynamic_import_literal(source, open) else {
        return false;
    };
    let option_start = skip_ascii_ws_and_comments(source, literal_end);
    option_start < close && source.as_bytes().get(option_start).copied() == Some(b',')
}

fn dynamic_import_options_preview(source: &str, open: usize, close: usize) -> String {
    let Some((_specifier, literal_end)) = static_dynamic_import_literal(source, open) else {
        return String::new();
    };
    let option_start = skip_ascii_ws_and_comments(source, literal_end);
    expression_preview(&source[option_start..close])
}

fn find_dynamic_import_close(source: &str, open: usize) -> Option<usize> {
    let mut depth = 1usize;
    let mut index = open + 1;

    while index < source.len() {
        let byte = source.as_bytes()[index];
        if let Some(next) = skip_quoted_or_comment(source, index, byte) {
            index = next;
            continue;
        }

        match byte {
            b'(' => depth += 1,
            b')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
        index += 1;
    }

    None
}

fn expression_preview(expression: &str) -> String {
    let trimmed = expression.trim();
    const MAX_CHARS: usize = 160;
    let mut preview = trimmed.chars().take(MAX_CHARS).collect::<String>();
    if trimmed.chars().nth(MAX_CHARS).is_some() {
        preview.push_str("...");
    }
    preview
}

fn skip_ascii_ws_and_comments(source: &str, mut index: usize) -> usize {
    loop {
        index = skip_ascii_ws(source, index);
        if source.as_bytes().get(index).copied() == Some(b'/')
            && source.as_bytes().get(index + 1).copied() == Some(b'/')
        {
            index = source[index..]
                .find('\n')
                .map(|offset| index + offset + 1)
                .unwrap_or(source.len());
            continue;
        }
        if source.as_bytes().get(index).copied() == Some(b'/')
            && source.as_bytes().get(index + 1).copied() == Some(b'*')
        {
            index = source[index + 2..]
                .find("*/")
                .map(|offset| index + 2 + offset + 2)
                .unwrap_or(source.len());
            continue;
        }
        return index;
    }
}

pub(super) fn skip_quoted_or_comment(source: &str, index: usize, byte: u8) -> Option<usize> {
    match byte {
        b'"' | b'\'' | b'`' => Some(skip_quoted(source, index, byte)),
        b'/' if source.as_bytes().get(index + 1).copied() == Some(b'/') => source[index..]
            .find('\n')
            .map(|offset| index + offset + 1)
            .or(Some(source.len())),
        b'/' if source.as_bytes().get(index + 1).copied() == Some(b'*') => source[index + 2..]
            .find("*/")
            .map(|offset| index + 2 + offset + 2)
            .or(Some(source.len())),
        _ => None,
    }
}

fn skip_quoted(source: &str, start: usize, quote: u8) -> usize {
    let mut escaped = false;
    let mut index = start + 1;
    while index < source.len() {
        let byte = source.as_bytes()[index];
        if escaped {
            escaped = false;
        } else if byte == b'\\' {
            escaped = true;
        } else if byte == quote {
            return index + 1;
        }
        index += 1;
    }
    source.len()
}

fn skip_ascii_ws(source: &str, mut index: usize) -> usize {
    while source
        .as_bytes()
        .get(index)
        .is_some_and(u8::is_ascii_whitespace)
    {
        index += 1;
    }
    index
}

pub(super) fn previous_is_identifier(source: &str, index: usize) -> bool {
    source[..index]
        .bytes()
        .next_back()
        .is_some_and(is_identifier_byte)
}

pub(super) fn next_is_identifier(source: &str, index: usize) -> bool {
    source
        .as_bytes()
        .get(index)
        .copied()
        .is_some_and(is_identifier_byte)
}

fn is_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'$')
}

fn line_column(source: &str, index: usize) -> (usize, usize) {
    let mut line = 1usize;
    let mut column = 1usize;
    for byte in source[..index].bytes() {
        if byte == b'\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    (line, column)
}

#[cfg(test)]
mod tests {
    use super::collect_dynamic_imports;

    #[test]
    fn records_dynamic_imports_with_leading_comments_and_static_templates() {
        let imports = collect_dynamic_imports(
            r#"
const lazyPanel = () => import(/* webpackChunkName: "panel" */ "../panel");
const lazyTemplate = () => import(`../template-panel`);
"#,
        );

        let specifiers = imports
            .dynamic_imports
            .iter()
            .map(|import| import.specifier.as_str())
            .collect::<Vec<_>>();
        assert_eq!(specifiers, vec!["../panel", "../template-panel"]);
        assert!(
            imports
                .dynamic_imports
                .iter()
                .all(|import| import.chunking == "async-boundary-reference")
        );
    }

    #[test]
    fn records_dynamic_imports_after_utf8_bom_without_byte_boundary_panic() {
        let imports =
            collect_dynamic_imports("\u{feff}const lazyPanel = () => import(\"../panel\");\n");

        assert_eq!(imports.dynamic_imports.len(), 1);
        assert_eq!(imports.dynamic_imports[0].specifier, "../panel");
    }

    #[test]
    fn records_unresolved_dynamic_imports_without_static_specifiers() {
        let imports = collect_dynamic_imports(
            r#"
const path = "../panels/" + name;
const byVariable = () => import(path);
const byTemplate = () => import(`../panels/${name}`);
const staticPanel = () => import("../panels/static-panel");
"#,
        );

        assert_eq!(imports.dynamic_imports.len(), 1);
        assert_eq!(
            imports.dynamic_imports[0].specifier,
            "../panels/static-panel"
        );
        let unresolved = imports
            .unresolved_dynamic_imports
            .iter()
            .map(|import| (import.expression.as_str(), import.reason.as_str()))
            .collect::<Vec<_>>();
        assert_eq!(
            unresolved,
            vec![
                ("path", "non-static-dynamic-import-expression"),
                (
                    "`../panels/${name}`",
                    "non-static-dynamic-import-expression"
                )
            ]
        );
        assert!(
            imports
                .unresolved_dynamic_imports
                .iter()
                .all(|import| !import.node_modules_required)
        );
    }

    #[test]
    fn records_unsupported_dynamic_import_calls() {
        let imports = collect_dynamic_imports(
            r#"
const emptyImport = () => import();
const withOptions = () => import("../panels/options", { with: { type: "json" } });
const brokenImport = () => import("../missing";
const staticPanel = () => import("../panels/static-panel");
"#,
        );

        let specifiers = imports
            .dynamic_imports
            .iter()
            .map(|import| import.specifier.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            specifiers,
            vec!["../panels/options", "../panels/static-panel"]
        );
        let options_import = imports
            .dynamic_imports
            .iter()
            .find(|import| import.specifier == "../panels/options")
            .expect("options import");
        assert_eq!(options_import.import_options_present, true);
        assert_eq!(options_import.import_options_supported, false);
        let static_import = imports
            .dynamic_imports
            .iter()
            .find(|import| import.specifier == "../panels/static-panel")
            .expect("static import");
        assert_eq!(static_import.import_options_present, false);
        assert_eq!(static_import.import_options_supported, true);
        assert!(imports.unresolved_dynamic_imports.is_empty());
        let unsupported = imports
            .unsupported_dynamic_imports
            .iter()
            .map(|import| (import.expression.as_str(), import.reason.as_str()))
            .collect::<Vec<_>>();
        assert_eq!(
            unsupported,
            vec![
                ("", "unsupported-dynamic-import-empty-expression"),
                (
                    ", { with: { type: \"json\" } }",
                    "unsupported-dynamic-import-options"
                ),
                (
                    "\"../missing\";\nconst staticPanel = () => import(\"../panels/static-panel\");",
                    "unsupported-dynamic-import-unclosed-call"
                )
            ]
        );
        assert!(
            imports
                .unsupported_dynamic_imports
                .iter()
                .all(|import| !import.node_modules_required)
        );
    }
}
