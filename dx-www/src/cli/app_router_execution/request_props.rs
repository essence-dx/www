use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Value, json};

use super::source_render::{ComponentPropBinding, LoweredSourceDocument, StaticLiteralExpression};

pub(super) fn request_prop_bindings(
    route_params: &BTreeMap<String, String>,
    search_params: &BTreeMap<String, String>,
    alias_bindings: &[RequestPropAliasBinding],
) -> Vec<ComponentPropBinding> {
    let mut bindings = Vec::new();
    let mut names = BTreeSet::new();
    for (name, value) in route_params {
        let binding_name = format!("params.{name}");
        names.insert(binding_name.clone());
        bindings.push(ComponentPropBinding {
            name: binding_name,
            value: StaticLiteralExpression::String(value.clone()),
            source_kind: "next-app-router-route-param",
            expression: Some(format!("params.{name}")),
        });
    }
    for (name, value) in search_params {
        let binding_name = format!("searchParams.{name}");
        names.insert(binding_name.clone());
        bindings.push(ComponentPropBinding {
            name: binding_name,
            value: StaticLiteralExpression::String(value.clone()),
            source_kind: "next-app-router-search-param",
            expression: Some(format!("searchParams.{name}")),
        });
    }
    for alias in alias_bindings {
        if !names.insert(alias.alias.clone()) {
            continue;
        }
        bindings.push(ComponentPropBinding {
            name: alias.alias.clone(),
            value: StaticLiteralExpression::String(alias.value.clone()),
            source_kind: alias.source_kind,
            expression: Some(alias.expression.clone()),
        });
    }
    bindings
}

pub(super) fn request_prop_bindings_manifest(
    route_params: &BTreeMap<String, String>,
    search_params: &BTreeMap<String, String>,
    alias_bindings: &[RequestPropAliasBinding],
    unresolved_alias_bindings: &[RequestPropUnresolvedAliasBinding],
    bindings: &[ComponentPropBinding],
) -> Value {
    json!({
        "schema": "dx.tsx.requestPropBindings",
        "schema_revision": 1,
        "contract_name": "App Router Request Prop Bindings",
        "mode": "safe-next-app-router-page-props",
        "adapter_boundary": "source-owned-next-app-router-request-props",
        "status": if !unresolved_alias_bindings.is_empty() {
            "next-app-router-page-prop-binding-gaps"
        } else if bindings.is_empty() {
            "no-request-props"
        } else {
            "next-app-router-page-prop-bindings"
        },
        "route_param_bindings": route_params.keys().map(|name| format!("params.{name}")).collect::<Vec<_>>(),
        "search_param_bindings": search_params.keys().map(|name| format!("searchParams.{name}")).collect::<Vec<_>>(),
        "request_prop_alias_bindings": alias_bindings.iter().map(|binding| json!({
            "alias": &binding.alias,
            "canonical_name": &binding.canonical_name,
            "source_kind": binding.source_kind,
            "expression": &binding.expression,
            "source_path": &binding.source_path,
            "value_type": "string",
        })).collect::<Vec<_>>(),
        "request_prop_unresolved_alias_bindings": unresolved_alias_bindings.iter().map(|binding| json!({
            "alias": &binding.alias,
            "canonical_name": &binding.canonical_name,
            "source_kind": binding.source_kind,
            "expression": &binding.expression,
            "source_path": &binding.source_path,
            "reason": &binding.reason,
        })).collect::<Vec<_>>(),
        "binding_count": bindings.len(),
        "alias_binding_count": alias_bindings.len(),
        "unresolved_alias_binding_count": unresolved_alias_bindings.len(),
        "supported_expressions": [
            "params.slug",
            "params[\"slug\"]",
            "params?.slug",
            "params?.[\"slug\"]",
            "const { slug } = params",
            "const { slug: postSlug } = params",
            "const { slug = \"latest\" } = params",
            "const { preview: previewMode = \"off\" } = searchParams",
            "const { slug } = await params",
            "const { slug: postSlug } = await params",
            "const { slug } = (await params)",
            "(await params)?.slug",
            "(await params)?.[\"slug\"]",
            "searchParams.query",
            "searchParams[\"query\"]",
            "searchParams?.query",
            "searchParams?.[\"query\"]",
            "const preview = searchParams.query",
            "const preview = searchParams[\"query\"]",
            "const preview = (await searchParams).query",
            "const preview = (await searchParams)?.query",
        ],
        "bindings": bindings.iter().map(|binding| json!({
            "name": &binding.name,
            "source_kind": binding.source_kind,
            "expression": &binding.expression,
            "value_type": binding.value.value_type(),
            "static_value": binding.value.to_json_value(),
        })).collect::<Vec<_>>(),
        "node_modules_required": false,
        "next_runtime_required": false,
        "react_rsc_required": false,
        "arbitrary_request_code_execution": false,
        "source_owned_request_props": true,
        "external_runtime_required": false,
        "external_runtime_executed": false,
        "limits": [
            "Resolves safe literal request values already known to the DX App Router route matcher.",
            "Supports direct params.name and searchParams.name reads, quoted bracket access, safe optional chaining page params and searchParams reads, simple destructuring aliases, quoted string destructuring defaults, simple member assignment aliases, and async page params and searchParams aliases including parenthesized page-prop objects.",
            "Records unresolved page-prop aliases as missing-request-prop-value evidence instead of executing arbitrary request code.",
            "Does not execute async Server Components, dynamic generateMetadata, cookies, headers, or arbitrary request code."
        ],
    })
}

#[derive(Clone)]
pub(super) struct RequestPropAliasCollection {
    pub(super) resolved: Vec<RequestPropAliasBinding>,
    pub(super) unresolved: Vec<RequestPropUnresolvedAliasBinding>,
}

#[derive(Clone)]
pub(super) struct RequestPropAliasBinding {
    pub(super) alias: String,
    pub(super) canonical_name: String,
    pub(super) value: String,
    pub(super) source_kind: &'static str,
    pub(super) expression: String,
    pub(super) source_path: String,
}

#[derive(Clone)]
pub(super) struct RequestPropUnresolvedAliasBinding {
    pub(super) alias: String,
    pub(super) canonical_name: String,
    pub(super) source_kind: &'static str,
    pub(super) expression: String,
    pub(super) source_path: String,
    pub(super) reason: &'static str,
}

pub(super) fn collect_next_app_router_page_prop_aliases(
    documents: &[LoweredSourceDocument],
    route_params: &BTreeMap<String, String>,
    search_params: &BTreeMap<String, String>,
) -> RequestPropAliasCollection {
    let mut aliases = Vec::new();
    let mut seen = BTreeSet::new();
    let mut unresolved = Vec::new();
    let mut unresolved_seen = BTreeSet::new();
    for document in documents {
        collect_destructured_request_prop_aliases(
            &document.source,
            &document.source_path,
            "params",
            route_params,
            "next-app-router-route-param-alias",
            &mut aliases,
            &mut seen,
            &mut unresolved,
            &mut unresolved_seen,
        );
        collect_destructured_request_prop_aliases(
            &document.source,
            &document.source_path,
            "searchParams",
            search_params,
            "next-app-router-search-param-alias",
            &mut aliases,
            &mut seen,
            &mut unresolved,
            &mut unresolved_seen,
        );
        collect_member_request_prop_aliases(
            &document.source,
            &document.source_path,
            "params",
            route_params,
            "next-app-router-route-param-alias",
            &mut aliases,
            &mut seen,
            &mut unresolved,
            &mut unresolved_seen,
        );
        collect_member_request_prop_aliases(
            &document.source,
            &document.source_path,
            "searchParams",
            search_params,
            "next-app-router-search-param-alias",
            &mut aliases,
            &mut seen,
            &mut unresolved,
            &mut unresolved_seen,
        );
    }
    RequestPropAliasCollection {
        resolved: aliases,
        unresolved,
    }
}

#[allow(clippy::too_many_arguments)]
fn collect_destructured_request_prop_aliases(
    source: &str,
    source_path: &str,
    object_name: &str,
    values: &BTreeMap<String, String>,
    source_kind: &'static str,
    aliases: &mut Vec<RequestPropAliasBinding>,
    seen: &mut BTreeSet<String>,
    unresolved: &mut Vec<RequestPropUnresolvedAliasBinding>,
    unresolved_seen: &mut BTreeSet<String>,
) {
    for statement in source.split(';') {
        let Some(target) = safe_request_prop_assignment_target(statement, object_name) else {
            continue;
        };
        let left = &statement[..target.equals_index];
        let Some(start) = left.rfind('{') else {
            continue;
        };
        let Some(end) = left.rfind('}') else {
            continue;
        };
        if start >= end {
            continue;
        }
        let destructured = &left[start + 1..end];
        for part in destructured.split(',') {
            let part = part.trim();
            let Some(alias) = destructured_request_prop_alias(part) else {
                continue;
            };
            let canonical_name = format!("{}.{}", object_name, alias.prop_name);
            let expression = format!("const {{ {part} }} = {}", target.rhs_expression);
            let Some(value) = values
                .get(alias.prop_name)
                .cloned()
                .or_else(|| alias.fallback_literal.clone())
            else {
                push_unresolved_request_prop_alias(
                    unresolved,
                    unresolved_seen,
                    RequestPropUnresolvedAliasBinding {
                        alias: alias.alias.to_string(),
                        canonical_name,
                        source_kind,
                        expression,
                        source_path: source_path.to_string(),
                        reason: "missing-request-prop-value",
                    },
                );
                continue;
            };
            push_request_prop_alias(
                aliases,
                seen,
                RequestPropAliasBinding {
                    alias: alias.alias.to_string(),
                    canonical_name,
                    value,
                    source_kind,
                    expression,
                    source_path: source_path.to_string(),
                },
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn collect_member_request_prop_aliases(
    source: &str,
    source_path: &str,
    object_name: &str,
    values: &BTreeMap<String, String>,
    source_kind: &'static str,
    aliases: &mut Vec<RequestPropAliasBinding>,
    seen: &mut BTreeSet<String>,
    unresolved: &mut Vec<RequestPropUnresolvedAliasBinding>,
    unresolved_seen: &mut BTreeSet<String>,
) {
    for statement in source.split(';') {
        let trimmed = statement.trim();
        let Some(alias) = variable_declaration_name(trimmed) else {
            continue;
        };
        let Some(eq_index) = trimmed.find('=') else {
            continue;
        };
        let expression = trimmed[eq_index + 1..].trim();
        let Some(canonical_name) = next_app_router_page_prop_binding_name(expression) else {
            continue;
        };
        let Some(prop_name) = canonical_name.strip_prefix(&format!("{object_name}.")) else {
            continue;
        };
        let expression = format!("const {alias} = {expression}");
        let Some(value) = values.get(prop_name) else {
            push_unresolved_request_prop_alias(
                unresolved,
                unresolved_seen,
                RequestPropUnresolvedAliasBinding {
                    alias: alias.to_string(),
                    canonical_name,
                    source_kind,
                    expression,
                    source_path: source_path.to_string(),
                    reason: "missing-request-prop-value",
                },
            );
            continue;
        };
        push_request_prop_alias(
            aliases,
            seen,
            RequestPropAliasBinding {
                alias: alias.to_string(),
                canonical_name,
                value: value.clone(),
                source_kind,
                expression,
                source_path: source_path.to_string(),
            },
        );
    }
}

fn push_request_prop_alias(
    aliases: &mut Vec<RequestPropAliasBinding>,
    seen: &mut BTreeSet<String>,
    binding: RequestPropAliasBinding,
) {
    let key = format!(
        "{}::{}::{}",
        binding.source_path, binding.alias, binding.canonical_name
    );
    if seen.insert(key) {
        aliases.push(binding);
    }
}

fn push_unresolved_request_prop_alias(
    aliases: &mut Vec<RequestPropUnresolvedAliasBinding>,
    seen: &mut BTreeSet<String>,
    binding: RequestPropUnresolvedAliasBinding,
) {
    let key = format!(
        "{}::{}::{}",
        binding.source_path, binding.alias, binding.canonical_name
    );
    if seen.insert(key) {
        aliases.push(binding);
    }
}

struct RequestPropAssignmentTarget {
    equals_index: usize,
    rhs_expression: String,
}

fn safe_request_prop_assignment_target(
    statement: &str,
    object_name: &str,
) -> Option<RequestPropAssignmentTarget> {
    for (index, character) in statement.char_indices() {
        if character != '=' {
            continue;
        }
        let after = statement[index + 1..].trim_start();
        if let Some(rhs_expression) = request_prop_assignment_rhs(after, object_name) {
            return Some(RequestPropAssignmentTarget {
                equals_index: index,
                rhs_expression,
            });
        }
    }
    None
}

fn request_prop_assignment_rhs(after: &str, object_name: &str) -> Option<String> {
    let after = after.trim_start();
    if let Some(inner) = parenthesized_request_prop_assignment_rhs(after) {
        return request_prop_assignment_rhs(inner, object_name);
    }
    let (async_prefix, rest) = after
        .strip_prefix("await ")
        .map(|rest| (true, rest.trim_start()))
        .unwrap_or((false, after));
    let rest = rest.strip_prefix(object_name)?;
    if rest.chars().next().is_some_and(is_identifier_character) {
        return None;
    }
    Some(if async_prefix {
        format!("await {object_name}")
    } else {
        object_name.to_string()
    })
}

fn parenthesized_request_prop_assignment_rhs(after: &str) -> Option<&str> {
    let after = after.trim_start();
    if !(after.starts_with('(') && after.ends_with(')')) {
        return None;
    }
    let inner = &after[1..after.len() - 1];
    if has_unbalanced_static_delimiters(inner) {
        return None;
    }
    Some(inner.trim())
}

struct DestructuredRequestPropAlias<'a> {
    prop_name: &'a str,
    alias: &'a str,
    fallback_literal: Option<String>,
}

fn destructured_request_prop_alias(part: &str) -> Option<DestructuredRequestPropAlias<'_>> {
    let part = part.trim();
    if part.is_empty() || part.starts_with("...") {
        return None;
    }
    let (part, fallback_literal) = destructured_part_and_default(part)?;
    if let Some((prop_name, alias)) = part.split_once(':') {
        let prop_name = prop_name.trim();
        let alias = alias.trim();
        if is_simple_prop_identifier(prop_name) && is_simple_prop_identifier(alias) {
            return Some(DestructuredRequestPropAlias {
                prop_name,
                alias,
                fallback_literal,
            });
        }
        return None;
    }
    if is_simple_prop_identifier(part) {
        Some(DestructuredRequestPropAlias {
            prop_name: part,
            alias: part,
            fallback_literal,
        })
    } else {
        None
    }
}

fn destructured_part_and_default(part: &str) -> Option<(&str, Option<String>)> {
    if let Some((binding, fallback)) = part.split_once('=') {
        Some((
            binding.trim(),
            quoted_request_prop_default_literal(fallback.trim()),
        ))
    } else {
        Some((part.trim(), None))
    }
}

fn quoted_request_prop_default_literal(expression: &str) -> Option<String> {
    let expression = strip_static_parentheses(expression.trim());
    let quote = expression
        .chars()
        .next()
        .filter(|character| matches!(character, '"' | '\'' | '`'))?;
    if !expression.ends_with(quote) || expression.len() < quote.len_utf8() * 2 {
        return None;
    }
    let value = &expression[quote.len_utf8()..expression.len() - quote.len_utf8()];
    if value.contains('\\')
        || value.contains('\n')
        || value.contains('\r')
        || (quote == '`' && value.contains("${"))
    {
        return None;
    }
    Some(value.to_string())
}

fn variable_declaration_name(statement: &str) -> Option<&str> {
    let statement = statement.trim_start();
    let rest = statement
        .strip_prefix("const ")
        .or_else(|| statement.strip_prefix("let "))
        .or_else(|| statement.strip_prefix("var "))?
        .trim_start();
    let name = rest
        .split(|character: char| !is_identifier_character(character))
        .next()?;
    if is_simple_prop_identifier(name) {
        Some(name)
    } else {
        None
    }
}

pub(super) fn resolve_next_app_router_page_prop_identifier(
    expression: &str,
    prop_bindings: &[ComponentPropBinding],
) -> Option<StaticLiteralExpression> {
    let expression = strip_static_parentheses(expression.trim());
    let binding_name = next_app_router_page_prop_binding_name(expression)?;
    prop_bindings
        .iter()
        .find(|binding| binding.name == binding_name)
        .map(|binding| binding.value.clone())
}

pub(super) fn next_app_router_page_prop_binding_name(expression: &str) -> Option<String> {
    if let Some(name) = expression.strip_prefix("params.") {
        return simple_member_access_name(name).map(|name| format!("params.{name}"));
    }
    if let Some(name) = expression.strip_prefix("searchParams.") {
        return simple_member_access_name(name).map(|name| format!("searchParams.{name}"));
    }
    if let Some(name) = optional_request_prop_member_access_name(expression, "params") {
        return Some(format!("params.{name}"));
    }
    if let Some(name) = optional_request_prop_member_access_name(expression, "searchParams") {
        return Some(format!("searchParams.{name}"));
    }
    if let Some(name) = async_request_prop_member_access_name(expression, "params") {
        return Some(format!("params.{name}"));
    }
    if let Some(name) = async_request_prop_member_access_name(expression, "searchParams") {
        return Some(format!("searchParams.{name}"));
    }
    if let Some(name) = async_quoted_bracket_member_access_name(expression, "params") {
        return Some(format!("params.{name}"));
    }
    if let Some(name) = async_quoted_bracket_member_access_name(expression, "searchParams") {
        return Some(format!("searchParams.{name}"));
    }
    if let Some(name) = optional_quoted_bracket_member_access_name(expression, "params") {
        return Some(format!("params.{name}"));
    }
    if let Some(name) = optional_quoted_bracket_member_access_name(expression, "searchParams") {
        return Some(format!("searchParams.{name}"));
    }
    quoted_bracket_member_access_name(expression, "params")
        .map(|name| format!("params.{name}"))
        .or_else(|| {
            quoted_bracket_member_access_name(expression, "searchParams")
                .map(|name| format!("searchParams.{name}"))
        })
}

fn optional_request_prop_member_access_name<'a>(
    expression: &'a str,
    object_name: &str,
) -> Option<&'a str> {
    let rest = request_prop_access_rest(expression, object_name)?;
    let rest = rest.strip_prefix("?.")?;
    simple_member_access_name(rest)
}

fn optional_quoted_bracket_member_access_name<'a>(
    expression: &'a str,
    object_name: &str,
) -> Option<&'a str> {
    let rest = request_prop_access_rest(expression, object_name)?;
    let rest = rest.strip_prefix("?.")?;
    quoted_bracket_access_name(rest)
}

fn async_request_prop_member_access_name<'a>(
    expression: &'a str,
    object_name: &str,
) -> Option<&'a str> {
    let rest = async_request_prop_access_rest(expression, object_name)?;
    let rest = rest.strip_prefix('.')?;
    simple_member_access_name(rest)
}

fn async_quoted_bracket_member_access_name<'a>(
    expression: &'a str,
    object_name: &str,
) -> Option<&'a str> {
    let rest = async_request_prop_access_rest(expression, object_name)?;
    quoted_bracket_access_name(rest)
}

fn async_request_prop_access_rest<'a>(expression: &'a str, object_name: &str) -> Option<&'a str> {
    let rest = expression.strip_prefix("(await ")?;
    let rest = rest.trim_start().strip_prefix(object_name)?;
    let rest = rest.trim_start().strip_prefix(')')?;
    Some(rest.trim_start())
}

fn request_prop_access_rest<'a>(expression: &'a str, object_name: &str) -> Option<&'a str> {
    direct_request_prop_access_rest(expression, object_name)
        .or_else(|| async_request_prop_access_rest(expression, object_name))
}

fn direct_request_prop_access_rest<'a>(expression: &'a str, object_name: &str) -> Option<&'a str> {
    let rest = expression.strip_prefix(object_name)?;
    if rest.chars().next().is_some_and(is_identifier_character) {
        return None;
    }
    Some(rest.trim_start())
}

fn simple_member_access_name(candidate: &str) -> Option<&str> {
    if is_simple_prop_identifier(candidate) {
        Some(candidate)
    } else {
        None
    }
}

fn quoted_bracket_member_access_name<'a>(
    expression: &'a str,
    object_name: &str,
) -> Option<&'a str> {
    let rest = expression.strip_prefix(object_name)?.trim_start();
    quoted_bracket_access_name(rest)
}

fn quoted_bracket_access_name(mut rest: &str) -> Option<&str> {
    if !(rest.starts_with('[') && rest.ends_with(']')) {
        return None;
    }
    rest = rest[1..rest.len() - 1].trim();
    let quote = rest.chars().next().filter(|ch| *ch == '"' || *ch == '\'')?;
    if !rest.ends_with(quote) || rest.len() < 2 {
        return None;
    }
    let name = &rest[quote.len_utf8()..rest.len() - quote.len_utf8()];
    if name.is_empty() || name.contains('\n') || name.contains('\r') {
        return None;
    }
    Some(name)
}

fn strip_static_parentheses(mut expression: &str) -> &str {
    loop {
        let trimmed = expression.trim();
        if !(trimmed.starts_with('(') && trimmed.ends_with(')')) {
            return trimmed;
        }
        let inner = &trimmed[1..trimmed.len() - 1];
        if has_unbalanced_static_delimiters(inner) {
            return trimmed;
        }
        expression = inner;
    }
}

fn has_unbalanced_static_delimiters(expression: &str) -> bool {
    let mut depth = 0isize;
    for character in expression.chars() {
        match character {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth < 0 {
                    return true;
                }
            }
            _ => {}
        }
    }
    depth != 0
}

fn is_identifier_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_' || character == '$'
}

fn is_simple_prop_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_' || first == '$') {
        return false;
    }
    chars.all(|character| character.is_ascii_alphanumeric() || character == '_' || character == '$')
}

#[cfg(test)]
mod tests {
    use super::*;
    use dx_compiler::delivery::lower_react_jsx_source;

    #[test]
    fn request_prop_manifest_records_source_owned_contract_flags() {
        let route_params = BTreeMap::from([("slug".to_string(), "acme".to_string())]);
        let search_params = BTreeMap::from([("preview".to_string(), "1".to_string())]);
        let alias_bindings = vec![RequestPropAliasBinding {
            alias: "postSlug".to_string(),
            canonical_name: "params.slug".to_string(),
            value: "acme".to_string(),
            source_kind: "next-app-router-route-param-alias",
            expression: "const { slug: postSlug } = await params".to_string(),
            source_path: "app/blog/[slug]/page.tsx".to_string(),
        }];
        let unresolved_alias_bindings = vec![RequestPropUnresolvedAliasBinding {
            alias: "missingPreview".to_string(),
            canonical_name: "searchParams.missing".to_string(),
            source_kind: "next-app-router-search-param-alias",
            expression: "const missingPreview = (await searchParams).missing".to_string(),
            source_path: "app/blog/[slug]/page.tsx".to_string(),
            reason: "missing-request-prop-value",
        }];
        let bindings = request_prop_bindings(&route_params, &search_params, &alias_bindings);

        let manifest = request_prop_bindings_manifest(
            &route_params,
            &search_params,
            &alias_bindings,
            &unresolved_alias_bindings,
            &bindings,
        );

        assert_eq!(manifest["schema"], "dx.tsx.requestPropBindings");
        assert_eq!(manifest["status"], "next-app-router-page-prop-binding-gaps");
        assert_eq!(
            manifest["adapter_boundary"],
            "source-owned-next-app-router-request-props"
        );
        assert_eq!(manifest["next_runtime_required"], false);
        assert_eq!(manifest["react_rsc_required"], false);
        assert_eq!(manifest["arbitrary_request_code_execution"], false);
        assert_eq!(manifest["source_owned_request_props"], true);
        assert_eq!(manifest["external_runtime_required"], false);
        assert_eq!(manifest["external_runtime_executed"], false);
        assert_eq!(manifest["unresolved_alias_binding_count"], 1);
        assert_eq!(
            manifest["request_prop_unresolved_alias_bindings"][0]["reason"],
            "missing-request-prop-value"
        );
    }

    #[test]
    fn request_prop_identifier_resolves_optional_chaining_reads() {
        assert_eq!(
            next_app_router_page_prop_binding_name("params?.slug").as_deref(),
            Some("params.slug")
        );
        assert_eq!(
            next_app_router_page_prop_binding_name(r#"params?.["slug"]"#).as_deref(),
            Some("params.slug")
        );
        assert_eq!(
            next_app_router_page_prop_binding_name("(await params)?.slug").as_deref(),
            Some("params.slug")
        );
        assert_eq!(
            next_app_router_page_prop_binding_name(r#"(await params)?.["slug"]"#).as_deref(),
            Some("params.slug")
        );
        assert_eq!(
            next_app_router_page_prop_binding_name("searchParams?.query").as_deref(),
            Some("searchParams.query")
        );
        assert_eq!(
            next_app_router_page_prop_binding_name(r#"(await searchParams)?.["query"]"#).as_deref(),
            Some("searchParams.query")
        );
        assert_eq!(
            next_app_router_page_prop_binding_name("paramsExtra?.slug").as_deref(),
            None
        );
        assert_eq!(
            next_app_router_page_prop_binding_name("params?.[dynamic]").as_deref(),
            None
        );
    }

    #[test]
    fn destructured_request_prop_alias_defaults_to_quoted_strings() {
        let slug =
            destructured_request_prop_alias(r#"slug = "latest""#).expect("defaulted slug alias");
        assert_eq!(slug.prop_name, "slug");
        assert_eq!(slug.alias, "slug");
        assert_eq!(slug.fallback_literal.as_deref(), Some("latest"));

        let preview = destructured_request_prop_alias(r#"preview: previewMode = "off""#)
            .expect("defaulted preview alias");
        assert_eq!(preview.prop_name, "preview");
        assert_eq!(preview.alias, "previewMode");
        assert_eq!(preview.fallback_literal.as_deref(), Some("off"));

        let unsafe_default = destructured_request_prop_alias(r#"preview = computeDefault()"#)
            .expect("keeps alias but ignores executable default");
        assert_eq!(unsafe_default.prop_name, "preview");
        assert_eq!(unsafe_default.alias, "preview");
        assert_eq!(unsafe_default.fallback_literal, None);
    }

    #[test]
    fn destructured_request_prop_aliases_accept_parenthesized_safe_rhs() {
        let source = r#"
            export default async function Page({ params, searchParams }) {
                const { slug } = (await params);
                const { preview: previewMode = "off" } = (searchParams);
                return <p>{slug}:{previewMode}</p>;
            }
        "#;
        let route_params = BTreeMap::from([("slug".to_string(), "acme".to_string())]);
        let search_params = BTreeMap::new();
        let documents = vec![LoweredSourceDocument {
            role: "page",
            source_path: "app/blog/[slug]/page.tsx".to_string(),
            source: source.to_string(),
            document: lower_react_jsx_source("app/blog/[slug]/page.tsx", source),
        }];

        let aliases =
            collect_next_app_router_page_prop_aliases(&documents, &route_params, &search_params);

        assert!(aliases.resolved.iter().any(|binding| {
            binding.alias == "slug"
                && binding.canonical_name == "params.slug"
                && binding.expression == "const { slug } = await params"
                && binding.value == "acme"
        }));
        assert!(aliases.resolved.iter().any(|binding| {
            binding.alias == "previewMode"
                && binding.canonical_name == "searchParams.preview"
                && binding.expression == "const { preview: previewMode = \"off\" } = searchParams"
                && binding.value == "off"
        }));
        assert!(aliases.unresolved.is_empty());
    }
}
