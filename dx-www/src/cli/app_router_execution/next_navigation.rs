use serde_json::{Value, json};

const REDIRECT_ERROR_CODE: &str = "NEXT_REDIRECT";
const HTTP_ERROR_FALLBACK_ERROR_CODE: &str = "NEXT_HTTP_ERROR_FALLBACK";
const TEMPORARY_REDIRECT: u16 = 307;
const PERMANENT_REDIRECT: u16 = 308;
const NOT_FOUND_STATUS: u16 = 404;

pub(super) fn build_next_navigation_control_flow(source_path: &str, source: &str) -> Value {
    let uses_next_navigation = has_next_navigation_import(source);
    let active_redirect = next_navigation_redirect(source);
    let redirect_detected = active_redirect.is_some();
    let not_found_detected = detects_next_navigation_not_found(source);
    let control_flow_diagnostics = control_flow_diagnostics(source_path, source);
    let status = if redirect_detected {
        "redirect-ready"
    } else if not_found_detected {
        "not-found-ready"
    } else if uses_next_navigation {
        "next-navigation-imported"
    } else {
        "no-next-navigation-control-flow"
    };

    json!({
        "schema": "dx.next.appRouterControlFlow",
        "schema_revision": 1,
        "source_path": source_path,
        "status": status,
        "upstream": {
            "source_mirror": "G:/WWW/inspirations/nextjs",
            "remote": "https://github.com/vercel/next.js.git",
            "commit": "f3f56ecec2f3f8cefa0f0a1323ea406740251d5c",
            "license": "MIT License",
            "files": [
                "packages/next/src/client/components/redirect.ts",
                "packages/next/src/client/components/redirect-error.ts",
                "packages/next/src/client/components/not-found.ts",
                "packages/next/src/client/components/http-access-fallback/http-access-fallback.ts"
            ]
        },
        "uses_next_navigation": uses_next_navigation,
        "helpers": ["redirect()", "permanentRedirect()", "notFound()"],
        "redirect_type_arguments": ["\"push\"", "\"replace\"", "RedirectType.push", "RedirectType.replace"],
        "helper_imports": {
            "redirect": next_navigation_imported_names(source, "redirect"),
            "permanentRedirect": next_navigation_imported_names(source, "permanentRedirect"),
            "notFound": next_navigation_imported_names(source, "notFound"),
            "RedirectType": next_navigation_imported_names(source, "RedirectType"),
            "namespaces": next_navigation_namespace_imports(source),
        },
        "redirect": active_redirect,
        "diagnostic_count": control_flow_diagnostics.len(),
        "diagnostics": control_flow_diagnostics,
        "not_found": {
            "detected": not_found_detected,
            "helper": "notFound()",
            "local_helpers": next_navigation_helper_local_names(source, "notFound"),
            "digest": format!("{HTTP_ERROR_FALLBACK_ERROR_CODE};{NOT_FOUND_STATUS}"),
            "status_code": NOT_FOUND_STATUS,
            "head_hint": if not_found_detected {
                Some(r#"<meta name="robots" content="noindex" />"#)
            } else {
                None
            },
        },
        "node_modules_required": false,
        "source_owned_control_flow": true,
        "external_runtime_required": false,
        "external_runtime_executed": false,
        "limits": [
            "Mirrors Next.js navigation helper names and sentinel digest semantics for source-owned App Router pages.",
            "Emits server-component-style head hints for safe literal redirect/notFound calls.",
            "Does not execute arbitrary component imports or throw/catch Next.js runtime errors through React.",
            "Does not set the actual HTTP status for App Router page rendering yet."
        ],
    })
}

fn control_flow_diagnostics(source_path: &str, source: &str) -> Vec<Value> {
    if !has_next_navigation_import(source) {
        return Vec::new();
    }

    [
        ("permanentRedirect", PERMANENT_REDIRECT),
        ("redirect", TEMPORARY_REDIRECT),
    ]
    .into_iter()
    .flat_map(|(helper, status_code)| {
        next_navigation_helper_local_names(source, helper)
            .into_iter()
            .filter_map(move |local_helper| {
                unsupported_redirect_diagnostic(
                    source_path,
                    source,
                    helper,
                    &local_helper,
                    status_code,
                )
            })
    })
    .collect()
}

fn unsupported_redirect_diagnostic(
    source_path: &str,
    source: &str,
    helper: &str,
    local_helper: &str,
    status_code: u16,
) -> Option<Value> {
    let (args_start, args_end) = navigation_helper_call_span(source, local_helper)?;
    if redirect_call_for_name(source, helper, local_helper, status_code).is_some() {
        return None;
    }
    let argument_source = source[args_start..args_end].trim();
    let argument_count = split_top_level_call_arguments(argument_source).len();
    let (line, column) = source_position(source, args_start);
    Some(json!({
        "schema": "dx.next.appRouterControlFlowDiagnostic",
        "schema_revision": 1,
        "severity": "adapter-boundary",
        "source_path": source_path,
        "line": line,
        "column": column,
        "helper": format!("{helper}()"),
        "local_helper": format!("{local_helper}()"),
        "reason": "unsupported-dynamic-redirect-arguments",
        "message": format!(
            "DX App Router kept {local_helper}() in the adapter boundary because redirect destinations and redirect types must be source-safe literals."
        ),
        "argument_count": argument_count,
        "expected_safe_arguments": ["string literal destination", "\"push\"", "\"replace\"", "RedirectType.push", "RedirectType.replace"],
        "status_code_if_static": status_code,
        "source_owned_control_flow": true,
        "node_modules_required": false,
        "external_runtime_required": false,
        "external_runtime_executed": false,
        "full_next_runtime_required": false,
    }))
}

pub(super) fn next_navigation_head_tags(control_flow: &Value) -> String {
    let mut tags = Vec::new();
    if let Some(destination) = control_flow
        .get("redirect")
        .and_then(|redirect| redirect.get("destination"))
        .and_then(Value::as_str)
    {
        tags.push(format!(
            r#"<meta http-equiv="refresh" content="0;url={}" data-dx-next-redirect="true" />"#,
            escape_attr(destination)
        ));
    }
    if control_flow
        .get("not_found")
        .and_then(|not_found| not_found.get("detected"))
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        tags.push(
            r#"<meta name="robots" content="noindex" data-dx-next-not-found="true" />"#.to_string(),
        );
    }
    tags.join("")
}

pub(super) fn detects_next_navigation_not_found(source: &str) -> bool {
    has_next_navigation_import(source)
        && next_navigation_helper_local_names(source, "notFound")
            .iter()
            .any(|helper| navigation_helper_call_span(source, helper).is_some())
}

pub(super) fn next_navigation_redirect(source: &str) -> Option<Value> {
    if !has_next_navigation_import(source) {
        return None;
    }
    redirect_call(source, "permanentRedirect", PERMANENT_REDIRECT)
        .or_else(|| redirect_call(source, "redirect", TEMPORARY_REDIRECT))
}

fn has_next_navigation_import(source: &str) -> bool {
    source.contains("\"next/navigation\"") || source.contains("'next/navigation'")
}

fn redirect_call(source: &str, helper: &str, status_code: u16) -> Option<Value> {
    for local_helper in next_navigation_helper_local_names(source, helper) {
        if let Some(redirect) = redirect_call_for_name(source, helper, &local_helper, status_code) {
            return Some(redirect);
        }
    }
    None
}

fn redirect_call_for_name(
    source: &str,
    helper: &str,
    local_helper: &str,
    status_code: u16,
) -> Option<Value> {
    let arguments = safe_navigation_string_arguments(source, local_helper)?;
    let destination = arguments.first()?.to_string();
    let redirect_type = match arguments.get(1).map(String::as_str) {
        Some("push") => "push",
        Some("replace") | None => "replace",
        Some(_) => return None,
    };
    Some(json!({
        "detected": true,
        "helper": format!("{helper}()"),
        "local_helper": format!("{local_helper}()"),
        "aliased_helper": helper != local_helper,
        "destination": destination,
        "type": redirect_type,
        "status_code": status_code,
        "status_name": redirect_status_name(status_code),
        "digest_code": REDIRECT_ERROR_CODE,
        "digest_shape": format!("{REDIRECT_ERROR_CODE};{redirect_type};<url>;{status_code};"),
        "head_hint": "meta http-equiv=\"refresh\"",
    }))
}

fn redirect_status_name(status_code: u16) -> &'static str {
    match status_code {
        TEMPORARY_REDIRECT => "TemporaryRedirect",
        PERMANENT_REDIRECT => "PermanentRedirect",
        _ => "UnknownRedirect",
    }
}

fn safe_navigation_string_arguments(source: &str, helper: &str) -> Option<Vec<String>> {
    let (args_start, args_end) = navigation_helper_call_span(source, helper)?;
    let arguments = split_top_level_call_arguments(&source[args_start..args_end]);
    if arguments.is_empty() || arguments.len() > 2 {
        return None;
    }

    let mut parsed_arguments = Vec::new();
    parsed_arguments.push(read_complete_safe_string_literal(arguments[0])?);
    if let Some(argument) = arguments.get(1) {
        parsed_arguments.push(
            read_complete_safe_string_literal(argument)
                .or_else(|| read_navigation_redirect_type_argument(source, argument))?,
        );
    }
    Some(parsed_arguments)
}

fn read_navigation_redirect_type_argument(source: &str, argument: &str) -> Option<String> {
    let argument = argument.trim();
    for type_name in next_navigation_helper_local_names(source, "RedirectType") {
        if argument == format!("{type_name}.push") {
            return Some("push".to_string());
        }
        if argument == format!("{type_name}.replace") {
            return Some("replace".to_string());
        }
    }
    None
}

fn next_navigation_helper_local_names(source: &str, exported_name: &str) -> Vec<String> {
    let mut names = next_navigation_imported_names(source, exported_name);
    for namespace in next_navigation_namespace_imports(source) {
        let namespace_helper = format!("{namespace}.{exported_name}");
        if !names.contains(&namespace_helper) {
            names.push(namespace_helper);
        }
    }
    names
}

fn next_navigation_imported_names(source: &str, exported_name: &str) -> Vec<String> {
    let mut names = Vec::new();
    for statement in next_navigation_import_statements(source) {
        let Some(open_brace) = statement.find('{') else {
            continue;
        };
        let Some(close_brace) = find_matching_brace(statement, open_brace) else {
            continue;
        };
        for specifier in split_top_level_call_arguments(&statement[open_brace + 1..close_brace]) {
            let Some((exported, local)) = named_import_specifier(specifier) else {
                continue;
            };
            if exported == exported_name && !names.contains(&local) {
                names.push(local);
            }
        }
    }
    names
}

fn next_navigation_namespace_imports(source: &str) -> Vec<String> {
    let mut namespaces = Vec::new();
    for statement in next_navigation_import_statements(source) {
        let Some(namespace) = next_navigation_namespace_import(statement) else {
            continue;
        };
        if !namespaces.contains(&namespace) {
            namespaces.push(namespace);
        }
    }
    namespaces
}

fn next_navigation_namespace_import(statement: &str) -> Option<String> {
    let rest = statement
        .trim_start()
        .strip_prefix("import")?
        .trim_start()
        .strip_prefix('*')?
        .trim_start()
        .strip_prefix("as")?;
    if rest.chars().next().is_some_and(is_identifier_char) {
        return None;
    }
    let rest = rest.trim_start();
    let namespace_end = rest
        .find(|ch: char| !is_identifier_char(ch))
        .unwrap_or(rest.len());
    let namespace = &rest[..namespace_end];
    is_identifier(namespace).then(|| namespace.to_string())
}

fn next_navigation_import_statements(source: &str) -> Vec<&str> {
    let mut statements = Vec::new();
    let mut index = 0;
    while let Some(import_start) = find_word_outside_comments_and_strings(source, "import", index) {
        let import_end = find_import_statement_end(source, import_start);
        let statement = &source[import_start..import_end];
        if statement.contains("from \"next/navigation\"")
            || statement.contains("from 'next/navigation'")
        {
            statements.push(statement);
        }
        index = import_end.max(import_start + "import".len());
    }
    statements
}

fn named_import_specifier(specifier: &str) -> Option<(String, String)> {
    let specifier = specifier
        .trim()
        .strip_prefix("type ")
        .unwrap_or(specifier.trim());
    if specifier.is_empty() {
        return None;
    }
    let parts = specifier.split_whitespace().collect::<Vec<_>>();
    let (exported, local) = match parts.as_slice() {
        [name] => (*name, *name),
        [exported, "as", local] => (*exported, *local),
        _ => return None,
    };
    if is_identifier(exported) && is_identifier(local) {
        Some((exported.to_string(), local.to_string()))
    } else {
        None
    }
}

fn find_word_outside_comments_and_strings(
    source: &str,
    word: &str,
    mut index: usize,
) -> Option<usize> {
    while index < source.len() {
        let rest = &source[index..];
        if rest.starts_with("//") {
            index = skip_line_comment(source, index);
            continue;
        }
        if rest.starts_with("/*") {
            index = skip_block_comment(source, index);
            continue;
        }

        let ch = rest.chars().next()?;
        if is_string_quote(ch) {
            index = skip_string_literal(source, index, ch);
            continue;
        }

        if rest.starts_with(word) && has_identifier_boundary(source, index, word.len()) {
            return Some(index);
        }

        index += ch.len_utf8();
    }
    None
}

fn find_import_statement_end(source: &str, import_start: usize) -> usize {
    let mut index = import_start;
    while index < source.len() {
        let rest = &source[index..];
        if rest.starts_with("//") {
            index = skip_line_comment(source, index);
            continue;
        }
        if rest.starts_with("/*") {
            index = skip_block_comment(source, index);
            continue;
        }

        let Some(ch) = rest.chars().next() else {
            break;
        };
        if is_string_quote(ch) {
            index = skip_string_literal(source, index, ch);
            continue;
        }
        if ch == ';' {
            return index + ch.len_utf8();
        }
        index += ch.len_utf8();
    }
    source.len()
}

fn find_matching_brace(source: &str, open_brace: usize) -> Option<usize> {
    let mut depth = 1usize;
    let mut index = open_brace + 1;

    while index < source.len() {
        let rest = &source[index..];
        if rest.starts_with("//") {
            index = skip_line_comment(source, index);
            continue;
        }
        if rest.starts_with("/*") {
            index = skip_block_comment(source, index);
            continue;
        }

        let ch = rest.chars().next()?;
        if is_string_quote(ch) {
            index = skip_string_literal(source, index, ch);
            continue;
        }

        match ch {
            '{' => depth += 1,
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }

        index += ch.len_utf8();
    }

    None
}

fn navigation_helper_call_span(source: &str, helper: &str) -> Option<(usize, usize)> {
    let mut index = 0;
    while index < source.len() {
        let rest = &source[index..];
        if rest.starts_with("//") {
            index = skip_line_comment(source, index);
            continue;
        }
        if rest.starts_with("/*") {
            index = skip_block_comment(source, index);
            continue;
        }

        let ch = rest.chars().next()?;
        if is_string_quote(ch) {
            index = skip_string_literal(source, index, ch);
            continue;
        }

        if rest.starts_with(helper) && has_identifier_boundary(source, index, helper.len()) {
            let helper_end = index + helper.len();
            let open_paren = skip_ascii_whitespace(source, helper_end);
            if source[open_paren..].starts_with('(') {
                let close_paren = find_matching_paren(source, open_paren)?;
                return Some((open_paren + 1, close_paren));
            }
        }

        index += ch.len_utf8();
    }

    None
}

fn split_top_level_call_arguments(source: &str) -> Vec<&str> {
    if source.trim().is_empty() {
        return Vec::new();
    }

    let mut arguments = Vec::new();
    let mut start = 0;
    let mut depth = 0usize;
    let mut index = 0;

    while index < source.len() {
        let rest = &source[index..];
        if rest.starts_with("//") {
            index = skip_line_comment(source, index);
            continue;
        }
        if rest.starts_with("/*") {
            index = skip_block_comment(source, index);
            continue;
        }

        let Some(ch) = rest.chars().next() else {
            break;
        };
        if is_string_quote(ch) {
            index = skip_string_literal(source, index, ch);
            continue;
        }

        match ch {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                arguments.push(source[start..index].trim());
                start = index + ch.len_utf8();
            }
            _ => {}
        }

        index += ch.len_utf8();
    }

    arguments.push(source[start..].trim());
    arguments
}

fn find_matching_paren(source: &str, open_paren: usize) -> Option<usize> {
    let mut depth = 1usize;
    let mut index = open_paren + 1;

    while index < source.len() {
        let rest = &source[index..];
        if rest.starts_with("//") {
            index = skip_line_comment(source, index);
            continue;
        }
        if rest.starts_with("/*") {
            index = skip_block_comment(source, index);
            continue;
        }

        let ch = rest.chars().next()?;
        if is_string_quote(ch) {
            index = skip_string_literal(source, index, ch);
            continue;
        }

        match ch {
            '(' => depth += 1,
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }

        index += ch.len_utf8();
    }

    None
}

fn read_complete_safe_string_literal(source: &str) -> Option<String> {
    let source = source.trim();
    let quote = source.chars().next()?;
    if !is_string_quote(quote) {
        return None;
    }
    let mut value = String::new();
    let mut escaped = false;
    let mut index = quote.len_utf8();

    while index < source.len() {
        let ch = source[index..].chars().next()?;
        index += ch.len_utf8();
        if escaped {
            value.push(ch);
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == quote {
            if quote == '`' && value.contains("${") {
                return None;
            }
            return source[index..].trim().is_empty().then_some(value);
        }
        value.push(ch);
    }
    None
}

fn skip_ascii_whitespace(source: &str, mut index: usize) -> usize {
    while index < source.len() && source.as_bytes()[index].is_ascii_whitespace() {
        index += 1;
    }
    index
}

fn skip_line_comment(source: &str, index: usize) -> usize {
    source[index + 2..]
        .find('\n')
        .map(|offset| index + 2 + offset + 1)
        .unwrap_or(source.len())
}

fn skip_block_comment(source: &str, index: usize) -> usize {
    source[index + 2..]
        .find("*/")
        .map(|offset| index + 2 + offset + 2)
        .unwrap_or(source.len())
}

fn skip_string_literal(source: &str, index: usize, quote: char) -> usize {
    let mut cursor = index + quote.len_utf8();
    let mut escaped = false;

    while cursor < source.len() {
        let Some(ch) = source[cursor..].chars().next() else {
            return source.len();
        };
        cursor += ch.len_utf8();

        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == quote {
            return cursor;
        }
    }

    source.len()
}

fn has_identifier_boundary(source: &str, index: usize, len: usize) -> bool {
    let bytes = source.as_bytes();
    let before = index == 0 || (!is_identifier_byte(bytes[index - 1]) && bytes[index - 1] != b'.');
    let after_index = index + len;
    let after = after_index >= source.len() || !is_identifier_byte(bytes[after_index]);
    before && after
}

fn is_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'$'
}

fn is_identifier_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '$'
}

fn is_identifier(value: &str) -> bool {
    let mut bytes = value.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == b'_' || first == b'$') && bytes.all(is_identifier_byte)
}

fn is_string_quote(ch: char) -> bool {
    ch == '"' || ch == '\'' || ch == '`'
}

fn source_position(source: &str, index: usize) -> (usize, usize) {
    let mut line = 1usize;
    let mut column = 1usize;
    for ch in source[..index.min(source.len())].chars() {
        if ch == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    (line, column)
}

fn escape_attr(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigation_control_flow_ignores_comments_strings_and_dynamic_redirects() {
        let source = r#"
            import { redirect, notFound } from "next/navigation";

            const text = "notFound() redirect(\"/fake\")";
            // notFound()
            /* redirect("/old") */

            export default function Page() {
                const suffix = "next";
                redirect("/login" + suffix);
                return null;
            }
        "#;

        let control_flow = build_next_navigation_control_flow("app/page.tsx", source);

        assert_eq!(control_flow["status"], "next-navigation-imported");
        assert!(control_flow["redirect"].is_null());
        assert_eq!(control_flow["not_found"]["detected"], false);
    }

    #[test]
    fn navigation_control_flow_reports_adapter_boundary_for_dynamic_redirects() {
        let source = r#"
            import { redirect } from "next/navigation";

            export default function Page({ params }) {
                redirect(`/login/${params.slug}`);
                return null;
            }
        "#;

        let control_flow =
            build_next_navigation_control_flow("app/account/[slug]/page.tsx", source);

        assert_eq!(control_flow["status"], "next-navigation-imported");
        assert!(control_flow["redirect"].is_null());
        assert_eq!(control_flow["diagnostic_count"], 1);
        assert_eq!(
            control_flow["diagnostics"][0]["reason"],
            "unsupported-dynamic-redirect-arguments"
        );
        assert_eq!(control_flow["diagnostics"][0]["helper"], "redirect()");
        assert_eq!(control_flow["diagnostics"][0]["local_helper"], "redirect()");
        assert_eq!(
            control_flow["diagnostics"][0]["severity"],
            "adapter-boundary"
        );
        assert_eq!(
            control_flow["diagnostics"][0]["source_owned_control_flow"],
            true
        );
        assert_eq!(
            control_flow["diagnostics"][0]["node_modules_required"],
            false
        );
        assert_eq!(
            control_flow["diagnostics"][0]["external_runtime_executed"],
            false
        );
    }

    #[test]
    fn navigation_control_flow_accepts_whitespace_helpers_and_push_redirects() {
        let source = r#"
            import { redirect, notFound } from "next/navigation";

            export default function Page() {
                redirect ( "/target", "push" );
                notFound ( );
                return null;
            }
        "#;

        let control_flow = build_next_navigation_control_flow("app/page.tsx", source);

        assert_eq!(control_flow["status"], "redirect-ready");
        assert_eq!(control_flow["redirect"]["destination"], "/target");
        assert_eq!(control_flow["redirect"]["type"], "push");
        assert_eq!(control_flow["redirect"]["status_code"], TEMPORARY_REDIRECT);
        assert_eq!(control_flow["not_found"]["detected"], true);
    }

    #[test]
    fn navigation_control_flow_accepts_redirect_type_enum_arguments() {
        let source = r#"
            import { redirect, RedirectType } from "next/navigation";

            export default function Page() {
                redirect("/settings", RedirectType.push);
                return null;
            }
        "#;

        let control_flow = build_next_navigation_control_flow("app/settings/page.tsx", source);

        assert_eq!(control_flow["status"], "redirect-ready");
        assert_eq!(control_flow["redirect"]["destination"], "/settings");
        assert_eq!(control_flow["redirect"]["type"], "push");
        assert_eq!(control_flow["redirect"]["status_code"], TEMPORARY_REDIRECT);
    }

    #[test]
    fn navigation_control_flow_accepts_aliased_next_navigation_helpers() {
        let source = r#"
            import {
                redirect as go,
                notFound as missing,
                RedirectType as Mode,
            } from "next/navigation";

            export default function Page() {
                go("/dashboard", Mode.replace);
                missing();
                return null;
            }
        "#;

        let control_flow = build_next_navigation_control_flow("app/dashboard/page.tsx", source);

        assert_eq!(control_flow["status"], "redirect-ready");
        assert_eq!(control_flow["redirect"]["destination"], "/dashboard");
        assert_eq!(control_flow["redirect"]["type"], "replace");
        assert_eq!(control_flow["redirect"]["helper"], "redirect()");
        assert_eq!(control_flow["redirect"]["local_helper"], "go()");
        assert_eq!(control_flow["redirect"]["aliased_helper"], true);
        assert_eq!(control_flow["not_found"]["detected"], true);
        assert_eq!(control_flow["helper_imports"]["redirect"][0], "go");
        assert_eq!(control_flow["helper_imports"]["notFound"][0], "missing");
        assert_eq!(control_flow["helper_imports"]["RedirectType"][0], "Mode");
    }

    #[test]
    fn navigation_control_flow_accepts_namespace_next_navigation_helpers() {
        let source = r#"
            import * as navigation from "next/navigation";

            export default function Page() {
                navigation.redirect("/settings", navigation.RedirectType.push);
                navigation.notFound();
                return null;
            }
        "#;

        let control_flow = build_next_navigation_control_flow("app/settings/page.tsx", source);

        assert_eq!(control_flow["status"], "redirect-ready");
        assert_eq!(control_flow["redirect"]["destination"], "/settings");
        assert_eq!(control_flow["redirect"]["type"], "push");
        assert_eq!(control_flow["redirect"]["helper"], "redirect()");
        assert_eq!(
            control_flow["redirect"]["local_helper"],
            "navigation.redirect()"
        );
        assert_eq!(control_flow["redirect"]["aliased_helper"], true);
        assert_eq!(control_flow["not_found"]["detected"], true);
        assert_eq!(
            control_flow["helper_imports"]["namespaces"][0],
            "navigation"
        );
        assert_eq!(
            control_flow["not_found"]["local_helpers"][0],
            "navigation.notFound"
        );
    }

    #[test]
    fn navigation_control_flow_ignores_property_chains_that_contain_helper_names() {
        let source = r#"
            import { redirect } from "next/navigation";
            import * as navigation from "next/navigation";

            const shell = { navigation: { redirect() {} } };

            export default function Page() {
                shell.navigation.redirect("/local-only");
                return null;
            }
        "#;

        let control_flow = build_next_navigation_control_flow("app/page.tsx", source);

        assert_eq!(control_flow["status"], "next-navigation-imported");
        assert!(control_flow["redirect"].is_null());
        assert_eq!(control_flow["not_found"]["detected"], false);
    }

    #[test]
    fn navigation_control_flow_ignores_unimported_local_helper_names() {
        let source = r#"
            import { notFound } from "next/navigation";

            function redirect(destination) {
                return destination;
            }

            export default function Page() {
                redirect("/local-only");
                return null;
            }
        "#;

        let control_flow = build_next_navigation_control_flow("app/page.tsx", source);

        assert_eq!(control_flow["status"], "next-navigation-imported");
        assert!(control_flow["redirect"].is_null());
        assert_eq!(control_flow["not_found"]["detected"], false);
        assert!(
            control_flow["helper_imports"]["redirect"]
                .as_array()
                .unwrap()
                .is_empty()
        );
        assert_eq!(control_flow["helper_imports"]["notFound"][0], "notFound");
    }
}
