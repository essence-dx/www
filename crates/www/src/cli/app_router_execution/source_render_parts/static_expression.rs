fn resolve_component_prop_identifier(
    expression: &str,
    prop_bindings: &[ComponentPropBinding],
    prop_aliases: Option<&ComponentDestructuredPropAliases>,
) -> Option<StaticLiteralExpression> {
    let expression = strip_static_parentheses(expression.trim());
    if let Some(value) = resolve_next_app_router_page_prop_identifier(expression, prop_bindings) {
        return Some(value);
    }
    if let Some(value) = prop_bindings
        .iter()
        .find(|binding| binding.name == expression)
        .map(|binding| binding.value.clone())
    {
        return Some(value);
    }
    if expression == "error?.message" {
        return prop_bindings
            .iter()
            .find(|binding| binding.name == "error.message")
            .map(|binding| binding.value.clone());
    }
    if let Some(member_name) = expression.strip_prefix("props.") {
        if !is_simple_prop_identifier(member_name) {
            return None;
        }
        return prop_bindings
            .iter()
            .find(|binding| binding.name == member_name)
            .map(|binding| binding.value.clone());
    }

    let prop_alias = prop_aliases.and_then(|aliases| aliases.alias_for_expression(expression));
    let prop_name = if let Some(alias) = prop_alias {
        alias.prop_name.as_str()
    } else {
        if !prop_alias_allows_bare_identifier(expression, prop_aliases) {
            return None;
        }
        expression
    };
    if !is_simple_prop_identifier(prop_name) {
        return None;
    }
    prop_bindings
        .iter()
        .find(|binding| binding.name == prop_name)
        .map(|binding| binding.value.clone())
        .or_else(|| prop_alias.and_then(|alias| alias.default_value.clone()))
}

fn resolve_static_template_with_prop_bindings(
    expression: &str,
    prop_bindings: &[ComponentPropBinding],
    prop_aliases: Option<&ComponentDestructuredPropAliases>,
) -> Option<StaticLiteralExpression> {
    let expression = strip_static_parentheses(expression.trim());
    let (template, trim_output) = static_template_expression_and_transform(expression)?;
    let inner = &template[1..template.len() - 1];
    let mut output = String::new();
    let mut remaining = inner;

    loop {
        let Some(interpolation_start) = remaining.find("${") else {
            output.push_str(&unescape_static_string(remaining, '`'));
            break;
        };
        output.push_str(&unescape_static_string(
            &remaining[..interpolation_start],
            '`',
        ));
        let after_start = &remaining[interpolation_start + 2..];
        let interpolation_end = find_static_template_interpolation_end(after_start)?;
        let interpolation = after_start[..interpolation_end].trim();
        if interpolation.is_empty() || interpolation.contains("${") {
            return None;
        }
        let value = static_literal_expression(interpolation).or_else(|| {
            resolve_component_prop_identifier(interpolation, prop_bindings, prop_aliases)
        })?;
        output.push_str(&value.to_text());
        remaining = &after_start[interpolation_end + 1..];
    }

    if trim_output {
        output = output.trim().to_string();
    }

    Some(StaticLiteralExpression::String(output))
}

fn resolve_static_conditional_with_prop_bindings(
    expression: &str,
    prop_bindings: &[ComponentPropBinding],
    prop_aliases: Option<&ComponentDestructuredPropAliases>,
) -> Option<StaticLiteralExpression> {
    let expression = strip_static_parentheses(expression.trim());
    let (condition, consequent, alternate) = split_static_conditional_expression(expression)?;
    let selected_branch = if evaluate_static_condition(condition, prop_bindings, prop_aliases)? {
        consequent
    } else {
        alternate
    };
    resolve_static_conditional_branch(selected_branch, prop_bindings, prop_aliases)
}

fn resolve_static_class_list_with_prop_bindings(
    expression: &str,
    prop_bindings: &[ComponentPropBinding],
    prop_aliases: Option<&ComponentDestructuredPropAliases>,
) -> Option<StaticLiteralExpression> {
    let expression = strip_static_parentheses(expression.trim());
    let array_expression = static_class_list_array_expression(expression)?;
    let items = split_static_class_list_items(array_expression)?;
    let mut classes = Vec::new();

    for item in items {
        let Some(value) = resolve_static_class_piece_with_prop_bindings(
            item,
            prop_bindings,
            prop_aliases,
            StaticClassPieceMode::AllowBareExpression,
        )?
        else {
            continue;
        };
        if static_value_is_truthy(&value) {
            classes.push(value.to_text());
        }
    }

    Some(StaticLiteralExpression::String(classes.join(" ")))
}

fn resolve_static_class_call_with_prop_bindings(
    expression: &str,
    prop_bindings: &[ComponentPropBinding],
    prop_aliases: Option<&ComponentDestructuredPropAliases>,
) -> Option<StaticLiteralExpression> {
    let expression = strip_static_parentheses(expression.trim());
    let arguments_expression = static_class_call_arguments(expression)?;
    let arguments = split_static_class_call_arguments(arguments_expression)?;
    let mut classes = Vec::new();

    for argument in arguments {
        let Some(value) = resolve_static_class_piece_with_prop_bindings(
            argument,
            prop_bindings,
            prop_aliases,
            StaticClassPieceMode::RejectStructuredExpression,
        )?
        else {
            continue;
        };
        if static_value_is_truthy(&value) {
            classes.push(value.to_text());
        }
    }

    Some(StaticLiteralExpression::String(classes.join(" ")))
}

enum StaticClassPieceMode {
    AllowBareExpression,
    RejectStructuredExpression,
}

fn resolve_static_class_piece_with_prop_bindings(
    expression: &str,
    prop_bindings: &[ComponentPropBinding],
    prop_aliases: Option<&ComponentDestructuredPropAliases>,
    mode: StaticClassPieceMode,
) -> Option<Option<StaticLiteralExpression>> {
    let expression = strip_static_parentheses(expression.trim());
    if expression.is_empty() || expression.starts_with("...") {
        return None;
    }
    if matches!(mode, StaticClassPieceMode::RejectStructuredExpression)
        && (expression.starts_with('{') || expression.starts_with('['))
    {
        return None;
    }
    if let Some((condition, branch_expression)) = split_static_logical_and_expression(expression) {
        if !evaluate_static_condition(condition, prop_bindings, prop_aliases)? {
            return Some(None);
        }
        return resolve_static_conditional_branch(branch_expression, prop_bindings, prop_aliases)
            .map(Some);
    }
    resolve_static_conditional_with_prop_bindings(expression, prop_bindings, prop_aliases)
        .or_else(|| resolve_static_conditional_branch(expression, prop_bindings, prop_aliases))
        .map(Some)
}

fn resolve_static_conditional_branch(
    expression: &str,
    prop_bindings: &[ComponentPropBinding],
    prop_aliases: Option<&ComponentDestructuredPropAliases>,
) -> Option<StaticLiteralExpression> {
    static_literal_expression(expression)
        .or_else(|| {
            resolve_static_template_with_prop_bindings(expression, prop_bindings, prop_aliases)
        })
        .or_else(|| resolve_component_prop_identifier(expression, prop_bindings, prop_aliases))
}

fn static_class_list_array_expression(expression: &str) -> Option<&str> {
    const JOIN_DOUBLE_QUOTE_SPACE: &str = r#".filter(Boolean).join(" ")"#;
    const JOIN_SINGLE_QUOTE_SPACE: &str = ".filter(Boolean).join(' ')";
    let expression = strip_static_parentheses(expression.trim());
    let array_expression = expression
        .strip_suffix(JOIN_DOUBLE_QUOTE_SPACE)
        .or_else(|| expression.strip_suffix(JOIN_SINGLE_QUOTE_SPACE))?
        .trim();
    if !(array_expression.starts_with('[')
        && array_expression.ends_with(']')
        && array_expression.len() >= 2)
    {
        return None;
    }
    Some(&array_expression[1..array_expression.len() - 1])
}

fn static_class_call_arguments(expression: &str) -> Option<&str> {
    let expression = strip_static_parentheses(expression.trim());
    for callee in ["cn", "clsx"] {
        let Some(rest) = expression.strip_prefix(callee) else {
            continue;
        };
        let rest = rest.trim_start();
        if !(rest.starts_with('(') && rest.ends_with(')') && rest.len() >= 2) {
            continue;
        }
        return Some(&rest[1..rest.len() - 1]);
    }
    None
}

fn split_static_class_list_items(expression: &str) -> Option<Vec<&str>> {
    split_static_comma_separated_items(expression)
}

fn split_static_class_call_arguments(expression: &str) -> Option<Vec<&str>> {
    split_static_comma_separated_items(expression)
}

fn split_static_comma_separated_items(expression: &str) -> Option<Vec<&str>> {
    let mut scanner = StaticExpressionScanner::default();
    let mut start = 0usize;
    let mut items = Vec::new();

    for (index, character) in expression.char_indices() {
        scanner.scan(character);
        if scanner.is_top_level() && character == ',' {
            items.push(expression[start..index].trim());
            start = index + character.len_utf8();
        }
    }
    if !scanner.is_top_level() {
        return None;
    }
    items.push(expression[start..].trim());
    Some(items)
}

fn split_static_logical_and_expression(expression: &str) -> Option<(&str, &str)> {
    let expression = strip_static_parentheses(expression.trim());
    let and_index = find_top_level_operator(expression, "&&")?;
    let condition = expression[..and_index].trim();
    let branch_expression = expression[and_index + 2..].trim();
    if condition.is_empty() || branch_expression.is_empty() {
        return None;
    }
    Some((condition, branch_expression))
}

fn split_static_conditional_expression(expression: &str) -> Option<(&str, &str, &str)> {
    let expression = strip_static_parentheses(expression.trim());
    let question_index = find_top_level_character(expression, '?')?;
    let after_question = &expression[question_index + 1..];
    let colon_offset = find_top_level_character(after_question, ':')?;
    let condition = expression[..question_index].trim();
    let consequent = after_question[..colon_offset].trim();
    let alternate = after_question[colon_offset + 1..].trim();
    if condition.is_empty() || consequent.is_empty() || alternate.is_empty() {
        return None;
    }
    Some((condition, consequent, alternate))
}

fn evaluate_static_condition(
    expression: &str,
    prop_bindings: &[ComponentPropBinding],
    prop_aliases: Option<&ComponentDestructuredPropAliases>,
) -> Option<bool> {
    let expression = strip_static_parentheses(expression.trim());
    if let Some((left, operator, right)) = split_static_equality_condition(expression) {
        let left = resolve_static_condition_operand(left, prop_bindings, prop_aliases)?;
        let right = resolve_static_condition_operand(right, prop_bindings, prop_aliases)?;
        let equal = static_values_equal(&left, &right);
        return Some(if operator == "===" { equal } else { !equal });
    }
    let value = resolve_static_condition_operand(expression, prop_bindings, prop_aliases)?;
    Some(static_value_is_truthy(&value))
}

fn split_static_equality_condition(expression: &str) -> Option<(&str, &'static str, &str)> {
    if let Some(index) = find_top_level_operator(expression, "===") {
        return Some((
            expression[..index].trim(),
            "===",
            expression[index + 3..].trim(),
        ));
    }
    find_top_level_operator(expression, "!==").map(|index| {
        (
            expression[..index].trim(),
            "!==",
            expression[index + 3..].trim(),
        )
    })
}

fn resolve_static_condition_operand(
    expression: &str,
    prop_bindings: &[ComponentPropBinding],
    prop_aliases: Option<&ComponentDestructuredPropAliases>,
) -> Option<StaticLiteralExpression> {
    static_literal_expression(expression)
        .or_else(|| {
            resolve_static_template_with_prop_bindings(expression, prop_bindings, prop_aliases)
        })
        .or_else(|| resolve_component_prop_identifier(expression, prop_bindings, prop_aliases))
}

fn static_values_equal(left: &StaticLiteralExpression, right: &StaticLiteralExpression) -> bool {
    match (left, right) {
        (StaticLiteralExpression::Boolean(left), StaticLiteralExpression::Boolean(right)) => {
            left == right
        }
        (StaticLiteralExpression::Nullish, StaticLiteralExpression::Nullish) => true,
        (StaticLiteralExpression::String(left), StaticLiteralExpression::String(right))
        | (StaticLiteralExpression::Number(left), StaticLiteralExpression::Number(right)) => {
            left == right
        }
        _ => false,
    }
}

fn static_value_is_truthy(value: &StaticLiteralExpression) -> bool {
    match value {
        StaticLiteralExpression::String(value) => !value.is_empty(),
        StaticLiteralExpression::Number(value) => value != "0" && value != "-0",
        StaticLiteralExpression::Boolean(value) => *value,
        StaticLiteralExpression::Nullish => false,
    }
}

fn static_template_expression_and_transform(expression: &str) -> Option<(&str, bool)> {
    let trimmed = expression.trim();
    let (template, trim_output) = if let Some(template) = trimmed.strip_suffix(".trim()") {
        (strip_static_parentheses(template.trim()), true)
    } else {
        (trimmed, false)
    };
    if !(template.starts_with('`') && template.ends_with('`') && template.len() >= 2) {
        return None;
    }
    if !template.contains("${") {
        return None;
    }
    Some((template, trim_output))
}

fn find_static_template_interpolation_end(expression: &str) -> Option<usize> {
    let mut quote = None;
    let mut escaped = false;
    let mut paren_depth = 0usize;

    for (index, character) in expression.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if character == '\\' {
            escaped = true;
            continue;
        }
        if let Some(active_quote) = quote {
            if character == active_quote {
                quote = None;
            }
            continue;
        }
        match character {
            '"' | '\'' | '`' => quote = Some(character),
            '(' | '[' => paren_depth += 1,
            ')' | ']' => {
                paren_depth = paren_depth.saturating_sub(1);
            }
            '}' if paren_depth == 0 => return Some(index),
            _ => {}
        }
    }

    None
}

fn find_top_level_operator(expression: &str, operator: &str) -> Option<usize> {
    let mut scanner = StaticExpressionScanner::default();
    for (index, character) in expression.char_indices() {
        scanner.scan(character);
        if scanner.is_top_level() && expression[index..].strip_prefix(operator).is_some() {
            return Some(index);
        }
    }
    None
}

fn find_top_level_character(expression: &str, target: char) -> Option<usize> {
    let mut scanner = StaticExpressionScanner::default();
    for (index, character) in expression.char_indices() {
        scanner.scan(character);
        if scanner.is_top_level() && character == target {
            return Some(index);
        }
    }
    None
}

#[derive(Default)]
struct StaticExpressionScanner {
    quote: Option<char>,
    escaped: bool,
    paren_depth: usize,
}

impl StaticExpressionScanner {
    fn scan(&mut self, character: char) {
        if self.escaped {
            self.escaped = false;
            return;
        }
        if character == '\\' {
            self.escaped = true;
            return;
        }
        if let Some(active_quote) = self.quote {
            if character == active_quote {
                self.quote = None;
            }
            return;
        }
        match character {
            '"' | '\'' | '`' => self.quote = Some(character),
            '(' | '[' | '{' => self.paren_depth += 1,
            ')' | ']' | '}' => self.paren_depth = self.paren_depth.saturating_sub(1),
            _ => {}
        }
    }

    fn is_top_level(&self) -> bool {
        self.quote.is_none() && self.paren_depth == 0
    }
}

impl ComponentDestructuredPropAliases {
    fn alias_for_expression(&self, expression: &str) -> Option<&ComponentPropAlias> {
        self.aliases.iter().find(|alias| alias.alias == expression)
    }
}

fn prop_alias_allows_bare_identifier(
    expression: &str,
    prop_aliases: Option<&ComponentDestructuredPropAliases>,
) -> bool {
    let Some(prop_aliases) = prop_aliases else {
        return true;
    };
    prop_aliases.aliases.is_empty() || prop_aliases.alias_for_expression(expression).is_some()
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

fn apply_next_image_static_attributes(
    document: &LoweredSourceDocument,
    element: &DxReactJsxElement,
    attributes: &mut Vec<String>,
) {
    if !is_next_image_element(document, element) {
        return;
    }

    attributes.retain(|attribute| {
        !attribute.starts_with("fill")
            && !attribute.starts_with("priority")
            && !attribute.starts_with("quality")
            && !attribute.starts_with("placeholder")
            && !attribute.starts_with("blurDataURL")
            && !attribute.starts_with("unoptimized")
    });

    if !has_static_html_attribute(attributes, "decoding") {
        attributes.push(r#"decoding="async""#.to_string());
    }

    let priority = static_boolean_attribute(element, "priority");
    if !has_static_html_attribute(attributes, "loading") {
        attributes.push(format!(
            r#"loading="{}""#,
            if priority { "eager" } else { "lazy" }
        ));
    }
    if priority && !has_static_html_attribute(attributes, "fetchpriority") {
        attributes.push(r#"fetchpriority="high""#.to_string());
    }

    let fill = static_boolean_attribute(element, "fill");
    if fill {
        attributes.retain(|attribute| {
            !attribute.starts_with("width=") && !attribute.starts_with("height=")
        });
        if !has_static_html_attribute(attributes, "style") {
            attributes.push(
                r#"style="position:absolute;height:100%;width:100%;inset:0;color:transparent;object-fit:cover""#
                    .to_string(),
            );
        }
        attributes.push(r#"data-nimg="fill""#.to_string());
    } else {
        attributes.push(r#"data-nimg="1""#.to_string());
    }

    if !has_static_html_attribute(attributes, "data-dx-image-boundary") {
        attributes
            .push(r#"data-dx-image-boundary="next-image-static-optimized-metadata""#.to_string());
    }
}

fn apply_next_script_static_attributes(
    document: &LoweredSourceDocument,
    element: &DxReactJsxElement,
    attributes: &mut Vec<String>,
) {
    if !is_next_script_element(document, element) {
        return;
    }

    let strategy = static_attribute_literal_attr_value(element, "strategy")
        .unwrap_or_else(|| "afterInteractive".to_string());
    attributes.retain(|attribute| !attribute.starts_with("strategy="));

    if strategy != "beforeInteractive"
        && !has_static_html_attribute(attributes, "async")
        && !has_static_html_attribute(attributes, "defer")
    {
        attributes.push("defer".to_string());
    }
    if !has_static_html_attribute(attributes, "data-dx-next-script-strategy") {
        attributes.push(format!(
            r#"data-dx-next-script-strategy="{}""#,
            escape_html_attr(&strategy)
        ));
    }
    if !has_static_html_attribute(attributes, "data-dx-script-boundary") {
        attributes
            .push(r#"data-dx-script-boundary="next-script-static-script-metadata""#.to_string());
    }
}

fn apply_next_font_static_attributes(
    document: &LoweredSourceDocument,
    element: &DxReactJsxElement,
    attributes: &mut Vec<String>,
) {
    let Some(class_expression) = attribute_expression(element, "className") else {
        return;
    };
    let Some(font_binding) = next_font_binding_for_expression(document, class_expression) else {
        return;
    };

    if !has_static_html_attribute(attributes, "class") {
        attributes.push(format!(
            r#"class="{}""#,
            escape_html_attr(&font_binding.class_name)
        ));
    }
    if !has_static_html_attribute(attributes, "style") {
        attributes.push(format!(
            r#"style="{}""#,
            escape_html_attr(&font_binding.style)
        ));
    }
    attributes.push(format!(
        r#"data-dx-next-font="{}""#,
        escape_html_attr(&font_binding.expression)
    ));
}

#[derive(Debug, Clone)]
struct NextFontBinding {
    expression: String,
    class_name: String,
    style: String,
}

fn next_font_binding_for_expression(
    document: &LoweredSourceDocument,
    expression: &str,
) -> Option<NextFontBinding> {
    let expression = strip_static_parentheses(expression.trim());
    let (variable, member) = expression.split_once('.')?;
    if !matches!(member, "className" | "variable") {
        return None;
    }
    let detections = collect_font_loader_detections(&document.source_path, &document.source);
    let detection = detections
        .iter()
        .find(|detection| detection.variable_names.iter().any(|name| name == variable))?;
    let family = detection
        .imported
        .as_deref()
        .unwrap_or(variable)
        .replace('_', " ");
    let slug = css_identifier_slug(variable);
    let quoted_family = format!("'{}'", family.replace('\'', ""));
    let style = if member == "variable" {
        format!("--font-{slug}: {quoted_family}, system-ui, sans-serif")
    } else {
        format!("font-family: {quoted_family}, system-ui, sans-serif")
    };
    Some(NextFontBinding {
        expression: expression.to_string(),
        class_name: format!("__dx-font-{slug}"),
        style,
    })
}

fn css_identifier_slug(value: &str) -> String {
    let mut slug = String::new();
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }
    slug.trim_matches('-').to_string()
}

fn attribute_expression<'a>(element: &'a DxReactJsxElement, name: &str) -> Option<&'a str> {
    element
        .attributes
        .iter()
        .find(|attribute| attribute.name == name)
        .and_then(|attribute| attribute.expression.as_deref())
}

fn static_boolean_attribute(element: &DxReactJsxElement, name: &str) -> bool {
    element
        .attributes
        .iter()
        .find(|attribute| attribute.name == name)
        .is_some_and(|attribute| {
            attribute.value.is_none() && attribute.expression.is_none()
                || attribute
                    .expression
                    .as_deref()
                    .and_then(static_literal_expression)
                    .is_some_and(|value| matches!(value, StaticLiteralExpression::Boolean(true)))
        })
}

fn static_attribute_literal_attr_value(element: &DxReactJsxElement, name: &str) -> Option<String> {
    let attribute = element
        .attributes
        .iter()
        .find(|attribute| attribute.name == name)?;
    if let Some(value) = attribute.value.as_deref() {
        return Some(value.to_string());
    }
    attribute
        .expression
        .as_deref()
        .and_then(static_literal_expression)
        .map(|value| value.to_attr_value())
}

fn has_static_html_attribute(attributes: &[String], name: &str) -> bool {
    attributes.iter().any(|attribute| {
        attribute == name
            || attribute
                .strip_prefix(name)
                .is_some_and(|rest| rest.starts_with('='))
    })
}

struct StaticDxIconElementHtml {
    html: String,
    skipped_attributes: usize,
    literal_expressions: usize,
    prop_identifier_bindings: usize,
}

fn static_dx_icon_element_html(
    element: &DxReactJsxElement,
    prop_bindings: &[ComponentPropBinding],
    prop_aliases: Option<&ComponentDestructuredPropAliases>,
) -> StaticDxIconElementHtml {
    let mut literal_expressions = 0usize;
    let mut prop_identifier_bindings = 0usize;
    let mut supported_attributes = BTreeSet::from([
        "name",
        "class",
        "className",
        "title",
        "aria-label",
        "aria-hidden",
    ]);

    let mut take = |name: &str| {
        let value = static_dx_icon_attribute_value(element, name, prop_bindings, prop_aliases);
        if value.is_some() {
            supported_attributes.remove(name);
        }
        if let Some(value) = value {
            if value.literal_expression {
                literal_expressions += 1;
            }
            if value.prop_identifier_binding {
                prop_identifier_bindings += 1;
            }
            Some(value.value)
        } else {
            None
        }
    };

    let icon_name = take("name").unwrap_or_else(|| "pack:logo".to_string());
    let class_name = take("className").or_else(|| take("class"));
    let title = take("title");
    let aria_label = take("aria-label");
    let aria_hidden = take("aria-hidden").unwrap_or_else(|| {
        if aria_label.is_some() || title.is_some() {
            "false".to_string()
        } else {
            "true".to_string()
        }
    });
    let class_attr = match class_name.as_deref() {
        Some(value)
            if !value.trim().is_empty()
                && value.split_whitespace().any(|part| part == "dx-icon") =>
        {
            value.trim().to_string()
        }
        Some(value) if !value.trim().is_empty() => format!("dx-icon {}", value.trim()),
        _ => "dx-icon".to_string(),
    };
    let (set, icon) = split_dx_icon_name(&icon_name);
    let title_html = title
        .as_deref()
        .map(|value| format!("<title>{}</title>", escape_html_text(value)))
        .unwrap_or_default();
    let label_attr = aria_label
        .as_deref()
        .map(|value| format!(r#" aria-label="{}""#, escape_html_attr(value)))
        .unwrap_or_default();
    let body = dx_icon_svg_body(&set, &icon, &icon_name);
    let skipped_attributes = element
        .attributes
        .iter()
        .filter(|attribute| !supported_attributes.contains(attribute.name.as_str()))
        .filter(|attribute| {
            !matches!(
                attribute.name.as_str(),
                "name" | "class" | "className" | "title" | "aria-label" | "aria-hidden"
            )
        })
        .count();

    StaticDxIconElementHtml {
        html: format!(
            r#"<svg class="{}" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="{}" data-icon-source="dx-icons" data-dx-icon="{}" data-dx-icon-set="{}" data-dx-icon-name="{}"{}>{}{}</svg>"#,
            escape_html_attr(&class_attr),
            escape_html_attr(&aria_hidden),
            escape_html_attr(&icon_name),
            escape_html_attr(&set),
            escape_html_attr(&icon),
            label_attr,
            title_html,
            body
        ),
        skipped_attributes,
        literal_expressions,
        prop_identifier_bindings,
    }
}

struct StaticDxIconAttributeValue {
    value: String,
    literal_expression: bool,
    prop_identifier_binding: bool,
}

fn static_dx_icon_attribute_value(
    element: &DxReactJsxElement,
    name: &str,
    prop_bindings: &[ComponentPropBinding],
    prop_aliases: Option<&ComponentDestructuredPropAliases>,
) -> Option<StaticDxIconAttributeValue> {
    let attribute = element
        .attributes
        .iter()
        .find(|attribute| attribute.name == name)?;
    if let Some(value) = attribute.value.as_deref() {
        return Some(StaticDxIconAttributeValue {
            value: value.to_string(),
            literal_expression: false,
            prop_identifier_binding: false,
        });
    }
    let expression = attribute.expression.as_deref()?;
    if let Some(value) = static_literal_expression(expression) {
        return Some(StaticDxIconAttributeValue {
            value: value.to_attr_value(),
            literal_expression: true,
            prop_identifier_binding: false,
        });
    }
    resolve_static_class_call_with_prop_bindings(expression, prop_bindings, prop_aliases)
        .or_else(|| {
            resolve_static_class_list_with_prop_bindings(expression, prop_bindings, prop_aliases)
        })
        .or_else(|| {
            resolve_static_conditional_with_prop_bindings(expression, prop_bindings, prop_aliases)
        })
        .or_else(|| {
            resolve_static_template_with_prop_bindings(expression, prop_bindings, prop_aliases)
        })
        .or_else(|| resolve_component_prop_identifier(expression, prop_bindings, prop_aliases))
        .map(|value| StaticDxIconAttributeValue {
            value: value.to_attr_value(),
            literal_expression: false,
            prop_identifier_binding: true,
        })
}

fn split_dx_icon_name(name: &str) -> (String, String) {
    name.split_once(':')
        .map(|(set, icon)| (set.to_string(), icon.to_string()))
        .unwrap_or_else(|| ("pack".to_string(), name.to_string()))
}

fn dx_icon_svg_body(set: &str, icon: &str, original_name: &str) -> String {
    let mut reader = dx_icon::icons();
    reader
        .get(set, icon)
        .map(|icon| dx_icon_svg_inner(&icon.to_svg(24)))
        .unwrap_or_else(|| {
            format!(
                r#"<path d="M4 4h16v16H4z" data-dx-icon-missing="{}"/>"#,
                escape_html_attr(original_name)
            )
        })
}

fn dx_icon_svg_inner(svg: &str) -> String {
    let Some(start) = svg.find('>') else {
        return svg.to_string();
    };
    let Some(end) = svg.rfind("</svg>") else {
        return svg[start + 1..].to_string();
    };
    svg[start + 1..end].to_string()
}

struct StaticAttributeListSnapshot {
    html: Vec<String>,
    skipped_attributes: usize,
    literal_expressions: usize,
    prop_identifier_bindings: usize,
}

fn static_attribute_list_snapshot_with_bindings(
    element: &DxReactJsxElement,
    prop_bindings: &[ComponentPropBinding],
    prop_aliases: Option<&ComponentDestructuredPropAliases>,
) -> StaticAttributeListSnapshot {
    let mut html = Vec::new();
    let mut skipped_attributes = 0usize;
    let mut literal_expressions = 0usize;
    let mut prop_identifier_bindings = 0usize;
    for attribute in &element.attributes {
        match static_html_attribute_with_bindings(attribute, prop_bindings, prop_aliases) {
            StaticAttributeSnapshot::Rendered {
                html: attribute_html,
                literal_expression,
                prop_identifier_binding,
            } => {
                if literal_expression {
                    literal_expressions += 1;
                }
                if prop_identifier_binding {
                    prop_identifier_bindings += 1;
                }
                html.push(attribute_html);
            }
            StaticAttributeSnapshot::OmittedLiteralExpression {
                literal_expression,
                prop_identifier_binding,
            } => {
                if literal_expression {
                    literal_expressions += 1;
                }
                if prop_identifier_binding {
                    prop_identifier_bindings += 1;
                }
            }
            StaticAttributeSnapshot::Skipped => skipped_attributes += 1,
        }
    }
    StaticAttributeListSnapshot {
        html,
        skipped_attributes,
        literal_expressions,
        prop_identifier_bindings,
    }
}

enum StaticAttributeSnapshot {
    Rendered {
        html: String,
        literal_expression: bool,
        prop_identifier_binding: bool,
    },
    OmittedLiteralExpression {
        literal_expression: bool,
        prop_identifier_binding: bool,
    },
    Skipped,
}

fn static_html_attribute_with_bindings(
    attribute: &DxReactJsxAttribute,
    prop_bindings: &[ComponentPropBinding],
    prop_aliases: Option<&ComponentDestructuredPropAliases>,
) -> StaticAttributeSnapshot {
    if let Some(snapshot) = static_motion_attribute_snapshot(attribute) {
        return snapshot;
    }
    if is_event_attribute(&attribute.name) {
        if let Some(snapshot) = static_event_class_attribute_snapshot(attribute) {
            return snapshot;
        }
        return StaticAttributeSnapshot::Skipped;
    }
    let Some(name) = static_html_attribute_name(&attribute.name) else {
        return StaticAttributeSnapshot::Skipped;
    };
    if let Some(expression) = attribute.expression.as_deref() {
        return match static_literal_expression(expression) {
            Some(StaticLiteralExpression::Boolean(value))
                if is_enumerated_boolean_html_attribute(name) =>
            {
                StaticAttributeSnapshot::Rendered {
                    html: format!(r#"{name}="{value}""#),
                    literal_expression: true,
                    prop_identifier_binding: false,
                }
            }
            Some(StaticLiteralExpression::Boolean(false) | StaticLiteralExpression::Nullish) => {
                StaticAttributeSnapshot::OmittedLiteralExpression {
                    literal_expression: true,
                    prop_identifier_binding: false,
                }
            }
            Some(StaticLiteralExpression::Boolean(true)) if is_boolean_html_attribute(name) => {
                StaticAttributeSnapshot::Rendered {
                    html: name.to_string(),
                    literal_expression: true,
                    prop_identifier_binding: false,
                }
            }
            Some(value) => StaticAttributeSnapshot::Rendered {
                html: format!(r#"{name}="{}""#, escape_html_attr(&value.to_attr_value())),
                literal_expression: true,
                prop_identifier_binding: false,
            },
            None => resolve_static_class_call_with_prop_bindings(
                expression,
                prop_bindings,
                prop_aliases,
            )
            .or_else(|| {
                resolve_static_class_list_with_prop_bindings(
                    expression,
                    prop_bindings,
                    prop_aliases,
                )
            })
            .or_else(|| {
                resolve_static_conditional_with_prop_bindings(
                    expression,
                    prop_bindings,
                    prop_aliases,
                )
            })
            .or_else(|| {
                resolve_static_template_with_prop_bindings(expression, prop_bindings, prop_aliases)
            })
            .or_else(|| resolve_component_prop_identifier(expression, prop_bindings, prop_aliases))
            .map(|value| static_html_attribute_from_resolved_value(name, &value))
            .unwrap_or(StaticAttributeSnapshot::Skipped),
        };
    }
    match attribute.value.as_deref() {
        Some(value) => StaticAttributeSnapshot::Rendered {
            html: format!(r#"{name}="{}""#, escape_html_attr(value)),
            literal_expression: false,
            prop_identifier_binding: false,
        },
        None if is_boolean_html_attribute(name) => StaticAttributeSnapshot::Rendered {
            html: name.to_string(),
            literal_expression: false,
            prop_identifier_binding: false,
        },
        None => StaticAttributeSnapshot::Skipped,
    }
}

fn static_event_class_attribute_snapshot(
    attribute: &DxReactJsxAttribute,
) -> Option<StaticAttributeSnapshot> {
    if attribute.expression.is_some() {
        return None;
    }
    let value = attribute.value.as_deref()?.trim();
    let dom_event = dom_event_name_from_react_attribute(&attribute.name)?;
    let classes = safe_interaction_class_value(value)?;
    let suffix = data_attribute_suffix_for_event(&dom_event);
    Some(StaticAttributeSnapshot::Rendered {
        html: format!(
            r#"data-dx-on-{suffix}-class="{}""#,
            escape_html_attr(&classes)
        ),
        literal_expression: false,
        prop_identifier_binding: false,
    })
}

fn static_motion_attribute_snapshot(
    attribute: &DxReactJsxAttribute,
) -> Option<StaticAttributeSnapshot> {
    if !matches!(attribute.name.as_str(), "motion" | "dxMotion") {
        return None;
    }
    let (motion_source, literal_expression) = if let Some(value) = attribute.value.as_deref() {
        (value.trim().to_string(), false)
    } else {
        let expression = attribute.expression.as_deref()?;
        let value = static_literal_expression(expression)?;
        (value.to_attr_value(), true)
    };
    let motion_source = safe_interaction_class_value(&motion_source)?;
    let group_syntax = if motion_source.contains('(') && motion_source.contains(')') {
        "true"
    } else {
        "false"
    };
    let generator_syntax = if motion_source.contains("animation-")
        || motion_source.contains("animate:")
        || motion_source.contains("from(")
        || motion_source.contains("via(")
        || motion_source.contains("to(")
    {
        "advanced-animation"
    } else {
        "utility-or-group"
    };
    Some(StaticAttributeSnapshot::Rendered {
        html: format!(
            r#"data-dx-motion="dx-style" data-dx-motion-engine="dx-style-animation-generator" data-dx-motion-class="{}" data-dx-motion-group-syntax="{group_syntax}" data-dx-motion-syntax="{generator_syntax}""#,
            escape_html_attr(&motion_source)
        ),
        literal_expression,
        prop_identifier_binding: false,
    })
}

fn data_attribute_suffix_for_event(event_name: &str) -> String {
    event_name
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn safe_interaction_class_value(value: &str) -> Option<String> {
    let tokens = value
        .split_whitespace()
        .filter(|token| !token.is_empty())
        .collect::<Vec<_>>();
    if tokens.is_empty() {
        return None;
    }
    tokens
        .iter()
        .all(|token| token.chars().all(is_safe_static_interaction_class_char))
        .then(|| tokens.join(" "))
}

fn is_safe_static_interaction_class_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric()
        || matches!(
            ch,
            '-' | '_'
                | ':'
                | '/'
                | '['
                | ']'
                | '('
                | ')'
                | '.'
                | '%'
                | '#'
                | '@'
                | '!'
                | '&'
                | '*'
                | '+'
                | '~'
                | ','
        )
}

fn static_html_attribute_from_resolved_value(
    name: &str,
    value: &StaticLiteralExpression,
) -> StaticAttributeSnapshot {
    match value {
        StaticLiteralExpression::Boolean(value) if is_enumerated_boolean_html_attribute(name) => {
            StaticAttributeSnapshot::Rendered {
                html: format!(r#"{name}="{value}""#),
                literal_expression: false,
                prop_identifier_binding: true,
            }
        }
        StaticLiteralExpression::Boolean(false) | StaticLiteralExpression::Nullish => {
            StaticAttributeSnapshot::OmittedLiteralExpression {
                literal_expression: false,
                prop_identifier_binding: true,
            }
        }
        StaticLiteralExpression::Boolean(true) if is_boolean_html_attribute(name) => {
            StaticAttributeSnapshot::Rendered {
                html: name.to_string(),
                literal_expression: false,
                prop_identifier_binding: true,
            }
        }
        _ => StaticAttributeSnapshot::Rendered {
            html: format!(r#"{name}="{}""#, escape_html_attr(&value.to_attr_value())),
            literal_expression: false,
            prop_identifier_binding: true,
        },
    }
}

struct StaticChildSnapshot {
    html: String,
    literal_expressions: usize,
    skipped_expressions: usize,
    component_preview_insertions: usize,
    component_prop_identifier_bindings: usize,
    skipped_component_references: usize,
}

fn render_static_child_nodes(
    document: &LoweredSourceDocument,
    element: &DxReactJsxElement,
    state_graph: &DxStateGraph,
    request_prop_bindings: &[ComponentPropBinding],
    component_compositions: Option<&[Value]>,
) -> StaticChildSnapshot {
    if element.child_nodes.is_empty() {
        return static_child_snapshot_legacy(element, request_prop_bindings);
    }

    let mut html = String::new();
    let mut literal_expressions = 0usize;
    let mut skipped_expressions = 0usize;
    let mut component_preview_insertions = 0usize;
    let mut component_prop_identifier_bindings = 0usize;
    let mut skipped_component_references = 0usize;

    for child in &element.child_nodes {
        match child {
            DxReactJsxChildNode::Text { value } => html.push_str(&escape_html_text(value)),
            DxReactJsxChildNode::Expression { expression } => {
                match static_literal_expression(expression) {
                    Some(StaticLiteralExpression::Nullish) => literal_expressions += 1,
                    Some(value) => {
                        literal_expressions += 1;
                        html.push_str(&escape_html_text(&value.to_text()));
                    }
                    None => {
                        if let Some(value) = resolve_component_prop_identifier(
                            expression,
                            request_prop_bindings,
                            None,
                        ) {
                            literal_expressions += 1;
                            component_prop_identifier_bindings += 1;
                            html.push_str(&escape_html_text(&value.to_text()));
                        } else {
                            skipped_expressions += 1;
                        }
                    }
                }
            }
            DxReactJsxChildNode::Element { index } => {
                let Some(child_element) = document.document.elements.get(*index) else {
                    skipped_component_references += 1;
                    continue;
                };
                if is_static_renderable_element(document, child_element) {
                    let snapshot = static_element_snapshot_with_components(
                        document,
                        *index,
                        child_element,
                        state_graph,
                        request_prop_bindings,
                        component_compositions,
                    );
                    html.push_str(&snapshot.html);
                    literal_expressions += snapshot.literal_expressions;
                    skipped_expressions += snapshot.skipped_child_expressions;
                    component_preview_insertions += snapshot.component_preview_insertions;
                    component_prop_identifier_bindings +=
                        snapshot.component_prop_identifier_bindings;
                    skipped_component_references += snapshot.skipped_component_references;
                    continue;
                }
                if !is_component_reference(&child_element.name) {
                    skipped_component_references += 1;
                    continue;
                }
                let Some(component_compositions) = component_compositions else {
                    skipped_component_references += 1;
                    continue;
                };
                let Some(composition) = matching_component_composition_for_element(
                    component_compositions,
                    &document.source_path,
                    *index,
                    &child_element.name,
                ) else {
                    skipped_component_references += 1;
                    continue;
                };
                let Some(return_preview) = composition.get("return_preview") else {
                    skipped_component_references += 1;
                    continue;
                };
                let preview_html = return_preview
                    .get("html")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                if preview_html.is_empty() {
                    skipped_component_references += 1;
                    continue;
                }
                html.push_str(preview_html);
                component_preview_insertions += 1;
                component_prop_identifier_bindings += return_preview
                    .get("prop_identifier_binding_count")
                    .and_then(Value::as_u64)
                    .unwrap_or(0) as usize;
                literal_expressions += return_preview
                    .get("literal_expressions")
                    .and_then(Value::as_u64)
                    .unwrap_or(0) as usize;
                skipped_expressions += return_preview
                    .get("skipped_child_expressions")
                    .and_then(Value::as_u64)
                    .unwrap_or(0) as usize;
            }
        }
    }

    StaticChildSnapshot {
        html,
        literal_expressions,
        skipped_expressions,
        component_preview_insertions,
        component_prop_identifier_bindings,
        skipped_component_references,
    }
}

fn static_child_snapshot_legacy(
    element: &DxReactJsxElement,
    request_prop_bindings: &[ComponentPropBinding],
) -> StaticChildSnapshot {
    let mut html = escape_html_text(&element.text_content());
    let mut literal_expressions = 0usize;
    let mut skipped_expressions = 0usize;
    let mut component_prop_identifier_bindings = 0usize;
    for expression in &element.child_expressions {
        match static_literal_expression(expression) {
            Some(StaticLiteralExpression::Nullish) => literal_expressions += 1,
            Some(value) => {
                literal_expressions += 1;
                html.push_str(&escape_html_text(&value.to_text()));
            }
            None => {
                if let Some(value) =
                    resolve_component_prop_identifier(expression, request_prop_bindings, None)
                {
                    literal_expressions += 1;
                    component_prop_identifier_bindings += 1;
                    html.push_str(&escape_html_text(&value.to_text()));
                } else {
                    skipped_expressions += 1;
                }
            }
        }
    }
    StaticChildSnapshot {
        html,
        literal_expressions,
        skipped_expressions,
        component_preview_insertions: 0,
        component_prop_identifier_bindings,
        skipped_component_references: 0,
    }
}

#[derive(Clone)]
pub(super) enum StaticLiteralExpression {
    String(String),
    Number(String),
    Boolean(bool),
    Nullish,
}

impl StaticLiteralExpression {
    fn to_attr_value(&self) -> String {
        match self {
            StaticLiteralExpression::String(value) | StaticLiteralExpression::Number(value) => {
                value.clone()
            }
            StaticLiteralExpression::Boolean(value) => value.to_string(),
            StaticLiteralExpression::Nullish => String::new(),
        }
    }

    fn to_text(&self) -> String {
        self.to_attr_value()
    }

    pub(super) fn to_json_value(&self) -> Value {
        match self {
            StaticLiteralExpression::String(value) | StaticLiteralExpression::Number(value) => {
                json!(value)
            }
            StaticLiteralExpression::Boolean(value) => json!(value),
            StaticLiteralExpression::Nullish => Value::Null,
        }
    }

    pub(super) fn value_type(&self) -> &'static str {
        match self {
            StaticLiteralExpression::String(_) => "string",
            StaticLiteralExpression::Number(_) => "number-literal",
            StaticLiteralExpression::Boolean(_) => "boolean",
            StaticLiteralExpression::Nullish => "nullish",
        }
    }
}

fn static_literal_expression(expression: &str) -> Option<StaticLiteralExpression> {
    let expression = expression.trim();
    let expression = strip_static_parentheses(expression);
    if matches!(expression, "null" | "undefined") {
        return Some(StaticLiteralExpression::Nullish);
    }
    if expression == "true" {
        return Some(StaticLiteralExpression::Boolean(true));
    }
    if expression == "false" {
        return Some(StaticLiteralExpression::Boolean(false));
    }
    if let Some(value) = quoted_static_string(expression, '"') {
        return Some(StaticLiteralExpression::String(value));
    }
    if let Some(value) = quoted_static_string(expression, '\'') {
        return Some(StaticLiteralExpression::String(value));
    }
    if let Some(value) = static_template_literal(expression) {
        return Some(StaticLiteralExpression::String(value));
    }
    if is_static_number_literal(expression) {
        return Some(StaticLiteralExpression::Number(expression.to_string()));
    }
    None
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

fn quoted_static_string(expression: &str, quote: char) -> Option<String> {
    if !(expression.starts_with(quote) && expression.ends_with(quote) && expression.len() >= 2) {
        return None;
    }
    let inner = &expression[quote.len_utf8()..expression.len() - quote.len_utf8()];
    if inner.contains('\n') || inner.contains('\r') {
        return None;
    }
    Some(unescape_static_string(inner, quote))
}

fn static_template_literal(expression: &str) -> Option<String> {
    if !(expression.starts_with('`') && expression.ends_with('`') && expression.len() >= 2) {
        return None;
    }
    let inner = &expression[1..expression.len() - 1];
    if inner.contains("${") {
        return None;
    }
    Some(unescape_static_string(inner, '`'))
}

fn unescape_static_string(value: &str, quote: char) -> String {
    let mut output = String::new();
    let mut chars = value.chars();
    while let Some(character) = chars.next() {
        if character != '\\' {
            output.push(character);
            continue;
        }
        match chars.next() {
            Some('n') => output.push('\n'),
            Some('r') => output.push('\r'),
            Some('t') => output.push('\t'),
            Some('\\') => output.push('\\'),
            Some(escaped) if escaped == quote => output.push(escaped),
            Some(escaped) => {
                output.push('\\');
                output.push(escaped);
            }
            None => output.push('\\'),
        }
    }
    output
}

fn is_static_number_literal(expression: &str) -> bool {
    if expression.is_empty() {
        return false;
    }
    let unsigned = expression.strip_prefix('-').unwrap_or(expression);
    if unsigned.is_empty() {
        return false;
    }
    let mut dot_count = 0usize;
    let mut digit_count = 0usize;
    for character in unsigned.chars() {
        match character {
            '0'..='9' => digit_count += 1,
            '_' => {}
            '.' if dot_count == 0 => dot_count += 1,
            _ => return false,
        }
    }
    digit_count > 0
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
