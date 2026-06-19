use super::{
    StyleEngine, apply_wrappers_and_states, build_block, css_functions, theme_css,
    wrap_media_queries,
};

struct ApplyRuleOutput {
    base_declarations: Vec<String>,
    variant_rules: Vec<String>,
}

pub(super) fn css_apply_rules_from_source(engine: &StyleEngine, source: &str) -> String {
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
            if let Some(rules) = resolve_layered_apply_rules(engine, selector, block) {
                output.push_str(&rules);
            }
            continue;
        }

        if selector.is_empty() || selector.starts_with('@') || !block.contains("@apply") {
            continue;
        }

        if let Some(rules) = resolve_apply_block(engine, selector, block) {
            if !rules.base_declarations.is_empty() {
                output.push_str(&build_block(selector, &rules.base_declarations.join("; ")));
            }
            for rule in rules.variant_rules {
                output.push_str(rule.trim_end());
                if !output.ends_with('\n') {
                    output.push('\n');
                }
            }
        }
    }

    output
}

fn resolve_apply_block(
    engine: &StyleEngine,
    selector: &str,
    block: &str,
) -> Option<ApplyRuleOutput> {
    resolve_apply_block_with_depth(engine, selector, block, 0)
}

fn resolve_apply_block_with_depth(
    engine: &StyleEngine,
    selector: &str,
    block: &str,
    nested_depth: usize,
) -> Option<ApplyRuleOutput> {
    let mut base_declarations = Vec::new();
    let mut variant_rules = Vec::new();

    for statement in split_top_level_statements(block) {
        let statement = statement.trim();
        if statement.is_empty() {
            continue;
        }

        if statement.contains('{') || statement.contains('}') {
            continue;
        }

        if let Some(raw_classes) = statement.strip_prefix("@apply") {
            for class_name in raw_classes.split_whitespace() {
                let class_name = class_name.trim();
                if class_name.is_empty() || !theme_css::is_variant_safe_apply_token(class_name) {
                    return None;
                }
                if let Some((variant, base_class)) = apply_variant_token_parts(class_name) {
                    let declarations = resolve_apply_declarations(engine, base_class)?;
                    variant_rules.push(resolve_apply_variant_rule(
                        engine,
                        selector,
                        variant,
                        &declarations,
                    )?);
                } else {
                    base_declarations.extend(resolve_apply_declarations(engine, class_name)?);
                }
            }
            continue;
        }

        if statement.starts_with('@') {
            continue;
        }

        if css_functions::contains_tailwind_css_function(statement) {
            if let Some(declaration) = resolve_safe_authored_function_declaration(statement) {
                base_declarations.push(declaration);
            }
            continue;
        }

        if is_safe_authored_declaration(statement) {
            base_declarations.push(statement.to_string());
        }
    }

    if nested_depth == 0 {
        variant_rules.extend(resolve_nested_apply_blocks(engine, selector, block)?);
    }

    Some(ApplyRuleOutput {
        base_declarations,
        variant_rules,
    })
}

fn resolve_apply_declarations(engine: &StyleEngine, class_name: &str) -> Option<Vec<String>> {
    let (lookup, important) = class_name
        .strip_prefix('!')
        .map_or((class_name, false), |stripped| (stripped, true));

    if let Some(mut declarations) = apply_spacing_declarations(lookup) {
        if important {
            mark_apply_declarations_important(&mut declarations);
        }
        return Some(declarations);
    }

    let css = engine.css_for_class(class_name)?;
    extract_single_rule_declarations(&css)
}

fn resolve_apply_variant_rule(
    engine: &StyleEngine,
    selector: &str,
    variant: &str,
    declarations: &[String],
) -> Option<String> {
    build_apply_variant_rule(engine, selector, variant, declarations)
}

fn resolve_layered_apply_rules(
    engine: &StyleEngine,
    layer_selector: &str,
    block: &str,
) -> Option<String> {
    if !is_safe_apply_layer_selector(layer_selector) || !block.contains("@apply") {
        return None;
    }

    let mut body = String::new();
    let mut search_start = 0usize;

    while let Some(open) = top_level_char_from(block, '{', search_start) {
        let selector_start = previous_nested_rule_boundary(block, open);
        let selector = block[selector_start..open].trim();
        let content_start = open + 1;
        let (block_end, nested_block) = matching_block(block, content_start)?;
        search_start = block_end + 1;

        if selector.is_empty() || selector.starts_with('@') || !nested_block.contains("@apply") {
            continue;
        }
        if !is_safe_nested_apply_selector(selector) {
            return None;
        }

        if let Some(rules) = resolve_apply_block(engine, selector, nested_block) {
            if !rules.base_declarations.is_empty() {
                body.push_str(&build_block(selector, &rules.base_declarations.join("; ")));
            }
            for rule in rules.variant_rules {
                body.push_str(rule.trim_end());
                if !body.ends_with('\n') {
                    body.push('\n');
                }
            }
        }
    }

    (!body.trim().is_empty()).then(|| wrap_apply_layer_rules(layer_selector, &body))
}

fn wrap_apply_layer_rules(layer_selector: &str, body: &str) -> String {
    let mut output = String::new();
    output.push_str(layer_selector.trim());
    output.push_str(" {\n");
    output.push_str(body.trim_end());
    output.push_str("\n}\n");
    output
}

fn is_safe_apply_layer_selector(selector: &str) -> bool {
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

fn resolve_nested_apply_blocks(
    engine: &StyleEngine,
    parent_selector: &str,
    block: &str,
) -> Option<Vec<String>> {
    let mut rules = Vec::new();
    let mut search_start = 0usize;

    while let Some(open) = top_level_char_from(block, '{', search_start) {
        let selector_start = previous_nested_rule_boundary(block, open);
        let raw_selector = block[selector_start..open].trim();
        let content_start = open + 1;
        let (block_end, nested_block) = matching_block(block, content_start)?;
        search_start = block_end + 1;

        if !nested_block.contains("@apply") {
            continue;
        }

        let Some(nested_selector) = resolve_nested_apply_selector(parent_selector, raw_selector)
        else {
            if raw_selector.trim_start().starts_with('@') {
                continue;
            }
            return None;
        };
        let nested_rules =
            resolve_apply_block_with_depth(engine, &nested_selector, nested_block, 1)?;

        if !nested_rules.base_declarations.is_empty() {
            rules.push(build_block(
                &nested_selector,
                &nested_rules.base_declarations.join("; "),
            ));
        }
        rules.extend(nested_rules.variant_rules);
    }

    Some(rules)
}

fn resolve_nested_apply_selector(parent_selector: &str, nested_selector: &str) -> Option<String> {
    let nested_selector = nested_selector.trim();
    if !nested_selector.contains('&') || !is_safe_nested_apply_selector(nested_selector) {
        return None;
    }

    let resolved = nested_selector.replace('&', parent_selector);
    if is_safe_nested_apply_selector(&resolved) {
        Some(resolved)
    } else {
        None
    }
}

fn is_safe_nested_apply_selector(selector: &str) -> bool {
    let selector = selector.trim();
    let lower = selector.to_ascii_lowercase();
    !selector.is_empty()
        && !selector.contains('{')
        && !selector.contains('}')
        && !selector.contains(';')
        && !selector.contains(',')
        && !selector.contains('"')
        && !selector.contains('\'')
        && !selector.contains('\\')
        && !lower.contains("@import")
        && !lower.contains("javascript:")
        && !lower.contains("expression(")
        && !lower.contains("</")
}

fn build_apply_variant_rule(
    engine: &StyleEngine,
    selector: &str,
    variant: &str,
    declarations: &[String],
) -> Option<String> {
    let (media_queries, pseudo_classes, wrappers) = apply_wrappers_and_states(engine, variant)?;
    let declarations = declarations.join("; ");
    if declarations.trim().is_empty() {
        return None;
    }

    let mut variant_selector = String::with_capacity(selector.len() + pseudo_classes.len());
    variant_selector.push_str(selector);
    variant_selector.push_str(&pseudo_classes);

    let blocks = engine.decode_encoded_css(&declarations, &variant_selector, &wrappers);
    Some(wrap_media_queries(blocks, &media_queries))
}

fn apply_variant_token_parts(class_name: &str) -> Option<(&str, &str)> {
    let split_at = last_apply_variant_separator(class_name)?;
    let variant = class_name[..split_at].trim();
    let base = class_name[split_at + 1..].trim();
    (!variant.is_empty() && !base.is_empty()).then_some((variant, base))
}

fn last_apply_variant_separator(class_name: &str) -> Option<usize> {
    let mut bracket_depth = 0usize;
    let mut paren_depth = 0usize;
    let mut last_colon = None;

    for (index, byte) in class_name.bytes().enumerate() {
        match byte {
            b'[' => bracket_depth = bracket_depth.saturating_add(1),
            b']' => bracket_depth = bracket_depth.saturating_sub(1),
            b'(' if bracket_depth == 0 => paren_depth = paren_depth.saturating_add(1),
            b')' if bracket_depth == 0 => paren_depth = paren_depth.saturating_sub(1),
            b':' if bracket_depth == 0 && paren_depth == 0 => last_colon = Some(index),
            _ => {}
        }
    }

    last_colon
}

fn mark_apply_declarations_important(declarations: &mut [String]) {
    for declaration in declarations {
        if declaration.ends_with("!important") || !declaration.contains(':') {
            continue;
        }
        declaration.push_str(" !important");
    }
}

struct ApplySpacingRule {
    prefix: &'static str,
    properties: &'static [&'static str],
    supports_negative: bool,
    allows_auto: bool,
}

const APPLY_SPACING_RULES: &[ApplySpacingRule] = &[
    ApplySpacingRule {
        prefix: "scroll-mx-",
        properties: &["scroll-margin-inline"],
        supports_negative: true,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "scroll-my-",
        properties: &["scroll-margin-block"],
        supports_negative: true,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "scroll-ms-",
        properties: &["scroll-margin-inline-start"],
        supports_negative: true,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "scroll-me-",
        properties: &["scroll-margin-inline-end"],
        supports_negative: true,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "scroll-mbs-",
        properties: &["scroll-margin-block-start"],
        supports_negative: true,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "scroll-mbe-",
        properties: &["scroll-margin-block-end"],
        supports_negative: true,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "scroll-mt-",
        properties: &["scroll-margin-top"],
        supports_negative: true,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "scroll-mr-",
        properties: &["scroll-margin-right"],
        supports_negative: true,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "scroll-mb-",
        properties: &["scroll-margin-bottom"],
        supports_negative: true,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "scroll-ml-",
        properties: &["scroll-margin-left"],
        supports_negative: true,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "scroll-m-",
        properties: &["scroll-margin"],
        supports_negative: true,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "scroll-px-",
        properties: &["scroll-padding-inline"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "scroll-py-",
        properties: &["scroll-padding-block"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "scroll-ps-",
        properties: &["scroll-padding-inline-start"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "scroll-pe-",
        properties: &["scroll-padding-inline-end"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "scroll-pbs-",
        properties: &["scroll-padding-block-start"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "scroll-pbe-",
        properties: &["scroll-padding-block-end"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "scroll-pt-",
        properties: &["scroll-padding-top"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "scroll-pr-",
        properties: &["scroll-padding-right"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "scroll-pb-",
        properties: &["scroll-padding-bottom"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "scroll-pl-",
        properties: &["scroll-padding-left"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "scroll-p-",
        properties: &["scroll-padding"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "inset-x-",
        properties: &["inset-inline"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "inset-y-",
        properties: &["inset-block"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "inset-s-",
        properties: &["inset-inline-start"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "inset-e-",
        properties: &["inset-inline-end"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "inset-bs-",
        properties: &["inset-block-start"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "inset-be-",
        properties: &["inset-block-end"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "inset-",
        properties: &["inset"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "start-",
        properties: &["inset-inline-start"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "end-",
        properties: &["inset-inline-end"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "right-",
        properties: &["right"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "left-",
        properties: &["left"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "top-",
        properties: &["top"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "bottom-",
        properties: &["bottom"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "gap-x-",
        properties: &["column-gap"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "gap-y-",
        properties: &["row-gap"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "gap-",
        properties: &["gap"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "px-",
        properties: &["padding-inline"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "py-",
        properties: &["padding-block"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "ps-",
        properties: &["padding-inline-start"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "pe-",
        properties: &["padding-inline-end"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "pbs-",
        properties: &["padding-block-start"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "pbe-",
        properties: &["padding-block-end"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "pt-",
        properties: &["padding-top"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "pr-",
        properties: &["padding-right"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "pb-",
        properties: &["padding-bottom"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "pl-",
        properties: &["padding-left"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "p-",
        properties: &["padding"],
        supports_negative: false,
        allows_auto: false,
    },
    ApplySpacingRule {
        prefix: "mx-",
        properties: &["margin-inline"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "my-",
        properties: &["margin-block"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "ms-",
        properties: &["margin-inline-start"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "me-",
        properties: &["margin-inline-end"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "mbs-",
        properties: &["margin-block-start"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "mbe-",
        properties: &["margin-block-end"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "mt-",
        properties: &["margin-top"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "mr-",
        properties: &["margin-right"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "mb-",
        properties: &["margin-bottom"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "ml-",
        properties: &["margin-left"],
        supports_negative: true,
        allows_auto: true,
    },
    ApplySpacingRule {
        prefix: "m-",
        properties: &["margin"],
        supports_negative: true,
        allows_auto: true,
    },
];

fn apply_spacing_declarations(class_name: &str) -> Option<Vec<String>> {
    let (negative, lookup) = class_name
        .strip_prefix('-')
        .map_or((false, class_name), |stripped| (true, stripped));

    for rule in APPLY_SPACING_RULES {
        let Some(raw_value) = lookup.strip_prefix(rule.prefix) else {
            continue;
        };
        if raw_value.is_empty() || (negative && !rule.supports_negative) {
            return None;
        }
        let value = apply_spacing_value(raw_value, negative, rule.allows_auto)?;
        return Some(
            rule.properties
                .iter()
                .map(|property| format!("{property}: {value}"))
                .collect(),
        );
    }

    None
}

fn apply_spacing_value(raw_value: &str, negative: bool, allows_auto: bool) -> Option<String> {
    if raw_value == "auto" {
        return (allows_auto && !negative).then(|| "auto".to_string());
    }

    if let Some(custom_property) = custom_property_shorthand(raw_value) {
        let value = format!("var({custom_property})");
        return Some(if negative {
            format!("calc({value} * -1)")
        } else {
            value
        });
    }

    if let Some(value) = arbitrary_spacing_value(raw_value) {
        return Some(if negative {
            format!("calc({value} * -1)")
        } else {
            value
        });
    }

    if !is_numeric_spacing_value(raw_value) {
        return None;
    }

    let multiplier = if negative {
        format!("-{raw_value}")
    } else {
        raw_value.to_string()
    };
    Some(format!("calc(var(--spacing) * {multiplier})"))
}

fn custom_property_shorthand(raw_value: &str) -> Option<&str> {
    let custom_property = raw_value.strip_prefix('(')?.strip_suffix(')')?;
    if !custom_property.starts_with("--")
        || custom_property.len() <= 2
        || !custom_property
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
    {
        return None;
    }
    Some(custom_property)
}

fn arbitrary_spacing_value(raw_value: &str) -> Option<String> {
    let inner = raw_value.strip_prefix('[')?.strip_suffix(']')?;
    let value = inner.replace('_', " ");
    if is_safe_css_value(&value) {
        Some(value)
    } else {
        None
    }
}

fn is_numeric_spacing_value(raw_value: &str) -> bool {
    let mut dot_count = 0usize;
    let mut digit_count = 0usize;

    for ch in raw_value.chars() {
        if ch.is_ascii_digit() {
            digit_count += 1;
        } else if ch == '.' {
            dot_count += 1;
            if dot_count > 1 {
                return false;
            }
        } else {
            return false;
        }
    }

    digit_count > 0 && !raw_value.starts_with('.') && !raw_value.ends_with('.')
}

fn extract_single_rule_declarations(css: &str) -> Option<Vec<String>> {
    let css = css.trim();
    if css.starts_with('@') {
        return None;
    }
    let open = css.find('{')?;
    let close = css.rfind('}')?;
    if close <= open || css[open + 1..close].contains('{') {
        return None;
    }

    let body = &css[open + 1..close];
    let declarations = split_top_level_statements(body)
        .into_iter()
        .map(str::trim)
        .filter(|statement| !statement.is_empty())
        .filter(|statement| is_safe_authored_declaration(statement))
        .map(str::to_string)
        .collect::<Vec<_>>();

    if declarations.is_empty() {
        None
    } else {
        Some(declarations)
    }
}

fn resolve_safe_authored_function_declaration(statement: &str) -> Option<String> {
    let (property, value) = statement.split_once(':')?;
    let property = property.trim();
    let value = value.trim();
    if !is_safe_authored_property(property) {
        return None;
    }

    let transformed = css_functions::replace_tailwind_css_functions(value)?;
    if !is_safe_css_value(&transformed) {
        return None;
    }

    Some(format!("{property}: {transformed}"))
}

fn is_safe_css_value(value: &str) -> bool {
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

fn is_safe_authored_declaration(statement: &str) -> bool {
    let Some((property, value)) = statement.split_once(':') else {
        return false;
    };
    let property = property.trim();
    let value = value.trim();
    is_safe_authored_property(property) && is_safe_css_value(value)
}

fn is_safe_authored_property(property: &str) -> bool {
    !property.is_empty()
        && property
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
}

fn split_top_level_statements(block: &str) -> Vec<&str> {
    let mut statements = Vec::new();
    let mut start = 0usize;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut brace_depth = 0usize;
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
            '{' => brace_depth = brace_depth.saturating_add(1),
            '}' => brace_depth = brace_depth.saturating_sub(1),
            ';' if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
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

fn previous_nested_rule_boundary(input: &str, end: usize) -> usize {
    let mut boundary = 0usize;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut brace_depth = 0usize;
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
            '{' => brace_depth = brace_depth.saturating_add(1),
            '}' => {
                brace_depth = brace_depth.saturating_sub(1);
                if brace_depth == 0 {
                    boundary = index + ch.len_utf8();
                }
            }
            ';' if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
                boundary = index + ch.len_utf8();
            }
            _ => {}
        }
    }

    boundary
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
