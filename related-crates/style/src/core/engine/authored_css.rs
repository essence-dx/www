use super::{
    StyleEngine, apply_wrappers_and_states, build_block, css_functions, sanitize_declarations,
    wrap_media_queries,
};

struct VariantDirective<'a> {
    variants: &'a str,
    block: &'a str,
}

pub(super) fn css_authored_function_rules_from_source(source: &str) -> String {
    let mut output = String::new();
    let mut search_start = 0usize;

    while let Some(open) = top_level_char_from(source, '{', search_start) {
        let selector_start = previous_rule_boundary(source, open);
        let selector = source[selector_start..open].trim();
        let content_start = open + 1;
        let Some((block_end, block)) = matching_block(source, content_start) else {
            break;
        };
        search_start = block_end + 1;

        if selector.is_empty()
            || selector.starts_with('@')
            || !css_functions::contains_tailwind_css_function(block)
        {
            continue;
        }

        let declarations = transformed_function_declarations(block);
        if !declarations.is_empty() {
            output.push_str(&build_block(selector, &declarations.join("; ")));
        }
    }

    output
}

pub(super) fn css_variant_rules_from_source(engine: &StyleEngine, source: &str) -> String {
    let mut output = String::new();
    let mut search_start = 0usize;

    while let Some(open) = top_level_char_from(source, '{', search_start) {
        let selector_start = previous_rule_boundary(source, open);
        let selector = source[selector_start..open].trim();
        let content_start = open + 1;
        let Some((block_end, block)) = matching_block(source, content_start) else {
            break;
        };
        search_start = block_end + 1;

        if selector.starts_with("@layer") {
            if let Some(rules) = resolve_layered_variant_rules(engine, selector, block) {
                output.push_str(&rules);
            }
            continue;
        }

        if !is_safe_authored_selector(selector) || !block.contains("@variant") {
            continue;
        }

        output.push_str(&variant_rules_for_selector(engine, selector, block));
    }

    output
}

fn variant_rules_for_selector(engine: &StyleEngine, selector: &str, block: &str) -> String {
    let mut output = String::new();

    for directive in variant_directives(block) {
        let declarations = variant_directive_declarations(directive.block);
        if declarations.is_empty() {
            continue;
        }

        for variant in split_top_level_commas(directive.variants) {
            if let Some(css) = css_for_variant_rule(engine, selector, variant, &declarations) {
                output.push_str(css.trim_end());
                output.push('\n');
            }
        }
    }

    output
}

fn resolve_layered_variant_rules(
    engine: &StyleEngine,
    layer_selector: &str,
    block: &str,
) -> Option<String> {
    if !is_safe_variant_layer_selector(layer_selector) || !block.contains("@variant") {
        return None;
    }

    let mut body = String::new();
    let mut search_start = 0usize;

    while let Some(open) = top_level_char_from(block, '{', search_start) {
        let selector_start = previous_rule_boundary(block, open);
        let selector = block[selector_start..open].trim();
        let content_start = open + 1;
        let (block_end, nested_block) = matching_block(block, content_start)?;
        search_start = block_end + 1;

        if !is_safe_authored_selector(selector) || !nested_block.contains("@variant") {
            continue;
        }

        body.push_str(variant_rules_for_selector(engine, selector, nested_block).trim_end());
        if !body.ends_with('\n') {
            body.push('\n');
        }
    }

    (!body.trim().is_empty()).then(|| wrap_variant_layer_rules(layer_selector, &body))
}

fn wrap_variant_layer_rules(layer_selector: &str, body: &str) -> String {
    let mut output = String::new();
    output.push_str(layer_selector.trim());
    output.push_str(" {\n");
    output.push_str(body.trim_end());
    output.push_str("\n}\n");
    output
}

fn is_safe_variant_layer_selector(selector: &str) -> bool {
    let Some(layer_names) = selector.trim().strip_prefix("@layer") else {
        return false;
    };
    let layer_names = layer_names.trim();
    !layer_names.is_empty()
        && layer_names.split(',').all(|name| {
            let name = name.trim();
            !name.is_empty()
                && name
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' || ch == '.')
        })
}

fn css_for_variant_rule(
    engine: &StyleEngine,
    selector: &str,
    variant: &str,
    declarations: &[String],
) -> Option<String> {
    let variant = variant.trim();
    if variant.is_empty() {
        return None;
    }

    let (media_queries, pseudo_classes, wrappers) = apply_wrappers_and_states(engine, variant)?;
    let declarations = sanitize_declarations(&declarations.join("; "));
    if declarations.is_empty() {
        return None;
    }

    let mut variant_selector = String::with_capacity(selector.len() + pseudo_classes.len());
    variant_selector.push_str(selector);
    variant_selector.push_str(&pseudo_classes);
    let blocks = engine.decode_encoded_css(&declarations, &variant_selector, &wrappers);
    Some(wrap_media_queries(blocks, &media_queries))
}

fn variant_directives(block: &str) -> Vec<VariantDirective<'_>> {
    let mut directives = Vec::new();
    let mut search_start = 0usize;

    while let Some(relative_start) = block[search_start..].find("@variant") {
        let start = search_start + relative_start;
        let after_keyword = start + "@variant".len();
        if !is_top_level_position(block, start)
            || !block[after_keyword..]
                .chars()
                .next()
                .is_some_and(char::is_whitespace)
        {
            search_start = after_keyword;
            continue;
        }

        let Some(open) = top_level_char_from(block, '{', after_keyword) else {
            break;
        };
        let variants = block[after_keyword..open].trim();
        let Some((end, body)) = matching_block(block, open + 1) else {
            break;
        };
        if !variants.is_empty() {
            directives.push(VariantDirective {
                variants,
                block: body,
            });
        }
        search_start = end + 1;
    }

    directives
}

fn variant_directive_declarations(block: &str) -> Vec<String> {
    split_top_level_statements(block)
        .into_iter()
        .filter_map(|statement| variant_directive_declaration(statement.trim()))
        .collect()
}

fn variant_directive_declaration(statement: &str) -> Option<String> {
    if statement.is_empty()
        || statement.starts_with('@')
        || statement.contains('{')
        || statement.contains('}')
    {
        return None;
    }

    if css_functions::contains_tailwind_css_function(statement) {
        return transformed_function_declaration(statement);
    }

    let (property, value) = statement.split_once(':')?;
    let property = property.trim();
    let value = value.trim();
    if !is_safe_property(property) || !css_functions::is_safe_css_value(value) {
        return None;
    }

    Some(format!("{property}: {value}"))
}

fn transformed_function_declarations(block: &str) -> Vec<String> {
    split_top_level_statements(block)
        .into_iter()
        .filter_map(|statement| transformed_function_declaration(statement.trim()))
        .collect()
}

fn transformed_function_declaration(statement: &str) -> Option<String> {
    if statement.is_empty()
        || statement.starts_with('@')
        || !css_functions::contains_tailwind_css_function(statement)
    {
        return None;
    }

    let (property, value) = statement.split_once(':')?;
    let property = property.trim();
    let value = value.trim();
    if !is_safe_property(property) {
        return None;
    }

    let transformed = css_functions::replace_tailwind_css_functions(value)?;
    if !css_functions::is_safe_css_value(&transformed) {
        return None;
    }

    Some(format!("{property}: {transformed}"))
}

fn is_safe_authored_selector(selector: &str) -> bool {
    !selector.is_empty()
        && !selector.starts_with('@')
        && !selector
            .chars()
            .any(|ch| ch.is_control() || matches!(ch, '{' | '}' | ';' | '"' | '\'' | '\\'))
}

fn is_safe_property(property: &str) -> bool {
    !property.is_empty()
        && property
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
}

fn split_top_level_commas(input: &str) -> Vec<&str> {
    let mut parts = Vec::new();
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
                parts.push(input[start..index].trim());
                start = index + 1;
            }
            _ => {}
        }
    }

    let tail = input[start..].trim();
    if !tail.is_empty() {
        parts.push(tail);
    }
    parts
}

fn split_top_level_statements(block: &str) -> Vec<&str> {
    let mut statements = Vec::new();
    let mut start = 0usize;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut quote = None;
    let mut escaped = false;

    for (index, ch) in block.char_indices() {
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
            ';' if paren_depth == 0 && bracket_depth == 0 => {
                statements.push(&block[start..index]);
                start = index + 1;
            }
            _ => {}
        }
    }

    let tail = block[start..].trim();
    if !tail.is_empty() {
        statements.push(tail);
    }

    statements
}

fn is_top_level_position(input: &str, position: usize) -> bool {
    let mut depth = 0usize;
    let mut quote = None;
    let mut escaped = false;

    for (index, ch) in input.char_indices() {
        if index >= position {
            break;
        }

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
            '{' => depth = depth.saturating_add(1),
            '}' => depth = depth.saturating_sub(1),
            _ => {}
        }
    }

    depth == 0 && quote.is_none()
}

fn previous_rule_boundary(input: &str, end: usize) -> usize {
    let mut boundary = 0usize;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut quote = None;
    let mut escaped = false;

    for (index, ch) in input[..end].char_indices() {
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
            '{' | '}' => boundary = index + ch.len_utf8(),
            ';' if paren_depth == 0 && bracket_depth == 0 => {
                boundary = index + ch.len_utf8();
            }
            _ => {}
        }
    }

    boundary
}

fn matching_block(source: &str, content_start: usize) -> Option<(usize, &str)> {
    let mut depth = 1usize;
    let mut quote = None;
    let mut escaped = false;

    for (offset, ch) in source[content_start..].char_indices() {
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
            '{' => depth = depth.saturating_add(1),
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    let end = content_start + offset;
                    return Some((end, &source[content_start..end]));
                }
            }
            _ => {}
        }
    }

    None
}

fn top_level_char_from(input: &str, needle: char, start: usize) -> Option<usize> {
    let mut depth = 0usize;
    let mut quote = None;
    let mut escaped = false;

    for (index, ch) in input.char_indices().skip_while(|(index, _)| *index < start) {
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
            '{' => {
                if ch == needle && depth == 0 {
                    return Some(index);
                }
                depth = depth.saturating_add(1);
            }
            '}' => depth = depth.saturating_sub(1),
            _ if ch == needle && depth == 0 => return Some(index),
            _ => {}
        }
    }

    None
}
