use std::collections::BTreeMap;

use super::server_contract::{
    find_first_call_args, route_handler_body_alias_matches, route_handler_body_alias_path,
    route_handler_body_root_pattern, route_handler_normalized_await_expression,
    split_top_level_nullish,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RouteHandlerUnsupportedBodyRead {
    pub(super) root: String,
    pub(super) method: String,
}

pub(super) fn route_handler_unsupported_request_body_read(
    expression: &str,
    body_roots: &[String],
) -> Option<RouteHandlerUnsupportedBodyRead> {
    let (read, _) = split_top_level_nullish(expression);
    let read = route_handler_normalized_await_expression(read);
    for root in body_roots {
        for method in route_handler_unsupported_body_methods() {
            let getter = format!("{root}.{method}");
            if find_first_call_args(read, &[getter.as_str()]).is_some() {
                return Some(RouteHandlerUnsupportedBodyRead {
                    root: root.to_string(),
                    method: (*method).to_string(),
                });
            }
        }
    }
    None
}

pub(super) fn route_handler_unsupported_body_aliases(
    function_body: &str,
    body_roots: &[String],
) -> BTreeMap<String, RouteHandlerUnsupportedBodyRead> {
    let Some(root_pattern) = route_handler_body_root_pattern(body_roots) else {
        return BTreeMap::new();
    };
    let method_pattern = route_handler_unsupported_body_methods().join("|");
    let Ok(alias_re) = regex::Regex::new(&format!(
        r#"(?m)\b(?:const|let)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*(?::[^=;\n]+)?\s*=\s*(?:\(\s*)?(?:await\s+)?({root_pattern})\.({method_pattern})\(\)\s*\)?(?:\s+as\s+[^;\n]+)?"#
    )) else {
        return BTreeMap::new();
    };
    alias_re
        .captures_iter(function_body)
        .filter_map(|capture| {
            Some((
                capture.get(1)?.as_str().to_string(),
                RouteHandlerUnsupportedBodyRead {
                    root: capture.get(2)?.as_str().to_string(),
                    method: capture.get(3)?.as_str().to_string(),
                },
            ))
        })
        .collect()
}

pub(super) fn route_handler_unsupported_body_alias_read(
    expression: &str,
    aliases: &BTreeMap<String, RouteHandlerUnsupportedBodyRead>,
) -> Option<RouteHandlerUnsupportedBodyRead> {
    let (read, _) = split_top_level_nullish(expression);
    for (alias, body_read) in aliases {
        if route_handler_body_alias_matches(read, alias)
            || route_handler_body_alias_path(read, alias).is_some()
        {
            return Some(body_read.clone());
        }
    }
    None
}

pub(super) fn route_handler_unsupported_body_read_message(
    read: &RouteHandlerUnsupportedBodyRead,
) -> String {
    format!(
        "unsupported route handler request body reader `{}.{}()`: DX-WWW source-owned-safe-interpreter supports request.json(), request.text(), request.formData(), and request.body without executing external runtimes; binary body readers require an explicit adapter boundary (node_modules_required=false, external_runtime_executed=false).",
        read.root, read.method
    )
}

fn route_handler_unsupported_body_methods() -> &'static [&'static str] {
    &["arrayBuffer", "blob", "bytes"]
}
