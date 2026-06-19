//! # Style Parser
//!
//! Parses the `<style>` section of component files.
//! Supports scoped styles, atomic CSS classes, and global styles.

#![allow(missing_docs)]

use std::path::Path;

use crate::error::DxResult;

/// Class attribute patterns that dx-style scans in app and component source files.
pub const STYLE_CLASS_ATTRIBUTE_PATTERNS: [&str; 10] = [
    "className=\"",
    "className='",
    "class=\"",
    "class='",
    "className={`",
    "className={\"",
    "className={'",
    "class={`",
    "class={\"",
    "class={'",
];

/// Static class-composition helpers that dx-style can scan without evaluating JavaScript.
pub const STYLE_STATIC_CLASS_FUNCTIONS: [&str; 7] = [
    "classes(",
    "dxClass(",
    "cn(",
    "cx(",
    "clsx(",
    "classNames(",
    "cva(",
];

/// Extract static class tokens from TSX, JSX, and HTML class attributes.
pub fn extract_class_attribute_tokens(source: &str) -> Vec<String> {
    let mut tokens = Vec::new();

    for pattern in STYLE_CLASS_ATTRIBUTE_PATTERNS {
        let quote = pattern.chars().last().unwrap_or('"');
        let mut rest = source;

        while let Some(start) = rest.find(pattern) {
            let after_pattern = &rest[start + pattern.len()..];
            let Some(end) = after_pattern.find(quote) else {
                break;
            };

            tokens.extend(expand_grouped_class_tokens(&after_pattern[..end]));
            rest = &after_pattern[end + 1..];
        }
    }

    tokens.extend(extract_static_function_class_tokens(source));

    tokens
        .into_iter()
        .filter(|token| !token.is_empty())
        .collect()
}

fn extract_static_function_class_tokens(source: &str) -> Vec<String> {
    let mut tokens = Vec::new();

    for needle in STYLE_STATIC_CLASS_FUNCTIONS {
        let mut rest = source;
        while let Some(start) = rest.find(needle) {
            let after_call_start = &rest[start + needle.len()..];
            let Some(end) = find_static_class_call_end(after_call_start) else {
                break;
            };

            tokens.extend(extract_quoted_tokens_until_call_end(
                &after_call_start[..end],
            ));
            rest = &after_call_start[end + 1..];
        }
    }

    tokens
}

pub fn find_static_class_call_end(source: &str) -> Option<usize> {
    let mut depth = 1usize;
    let mut quote = None;
    let mut escaped = false;

    for (index, ch) in source.char_indices() {
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
            '"' | '\'' | '`' => quote = Some(ch),
            '(' => depth = depth.saturating_add(1),
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }

    None
}

fn extract_quoted_tokens_until_call_end(call_source: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = call_source.chars().collect();
    let mut index = 0;

    while index < chars.len() {
        let quote = chars[index];
        if !matches!(quote, '"' | '\'' | '`') {
            index += 1;
            continue;
        }

        index += 1;
        let start = index;
        while index < chars.len() && chars[index] != quote {
            index += 1;
        }

        if start < index {
            let value: String = chars[start..index].iter().collect();
            tokens.extend(expand_grouped_class_tokens(&value));
        }

        index += 1;
    }

    tokens
}

fn clean_class_attribute_token(token: &str) -> String {
    token
        .trim_matches(|ch: char| matches!(ch, '`' | '"' | '\'' | '{' | '}' | ',' | ';'))
        .to_string()
}

fn static_class_token(token: &str) -> Option<String> {
    let cleaned = clean_class_attribute_token(token);
    if cleaned.is_empty() || cleaned.contains("${") || cleaned.contains('$') {
        None
    } else {
        Some(cleaned)
    }
}

/// Expand compact grouped classnames into normal dx-style/Tailwind-like tokens.
pub fn expand_grouped_class_tokens(value: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut prefixes = Vec::new();

    for ch in value.chars() {
        if ch.is_whitespace() {
            push_grouped_class_token(&mut tokens, &mut current, &prefixes);
            continue;
        }

        if ch == '(' {
            if let Some(prefix) = grouped_class_prefix(&current) {
                prefixes.push(prefix);
                current.clear();
                continue;
            }
        } else if ch == ')' && !prefixes.is_empty() {
            push_grouped_class_token(&mut tokens, &mut current, &prefixes);
            prefixes.pop();
            continue;
        }

        current.push(ch);
    }

    push_grouped_class_token(&mut tokens, &mut current, &prefixes);

    if let Some(prefix) = prefixes.last() {
        tokens.push(format!("dx-grouping-error:unclosed-group:{prefix}"));
    }

    tokens
}

fn push_grouped_class_token(tokens: &mut Vec<String>, current: &mut String, prefixes: &[String]) {
    let Some(cleaned) = static_class_token(current) else {
        current.clear();
        return;
    };

    if prefixes.is_empty() {
        tokens.push(cleaned);
    } else {
        let mut class_name = String::new();
        for prefix in prefixes {
            class_name.push_str(prefix);
            class_name.push(':');
        }
        class_name.push_str(&cleaned);
        tokens.push(class_name);
    }

    current.clear();
}

fn grouped_class_prefix(current: &str) -> Option<String> {
    let prefix = current.trim().strip_suffix(':')?;
    if prefix.is_empty()
        || prefix
            .chars()
            .any(|ch| matches!(ch, '(' | ')' | '"' | '\'' | '`' | '{' | '}' | ';' | ','))
    {
        return None;
    }

    Some(prefix.to_string())
}

// =============================================================================
// Parsed Style
// =============================================================================

/// A parsed style section.
#[derive(Debug, Clone)]
pub struct ParsedStyle {
    /// Source CSS
    pub source: String,

    /// Whether styles are scoped
    pub scoped: bool,

    /// Parsed CSS rules
    pub rules: Vec<CssRule>,

    /// Detected atomic classes
    pub atomic_classes: Vec<String>,

    /// CSS custom properties (variables)
    pub custom_properties: Vec<CustomProperty>,

    /// Import statements
    pub imports: Vec<CssImport>,
}

/// A CSS rule.
#[derive(Debug, Clone)]
pub struct CssRule {
    /// Selector
    pub selector: String,

    /// Declarations
    pub declarations: Vec<CssDeclaration>,

    /// Whether this is an at-rule
    pub at_rule: Option<AtRule>,
}

/// A CSS declaration (property: value).
#[derive(Debug, Clone)]
pub struct CssDeclaration {
    /// Property name
    pub property: String,

    /// Property value
    pub value: String,

    /// Whether this is important
    pub important: bool,
}

/// An at-rule.
#[derive(Debug, Clone)]
pub enum AtRule {
    /// @media query
    Media { query: String },
    /// @container query
    Container { query: String },
    /// @keyframes animation
    Keyframes { name: String },
    /// @import
    Import { url: String },
    /// @supports
    Supports { condition: String },
    /// @layer
    Layer { name: Option<String> },
    /// A bounded stack of nested conditional at-rules.
    Nested { rules: Vec<AtRule> },
}

/// A CSS custom property.
#[derive(Debug, Clone)]
pub struct CustomProperty {
    /// Property name (e.g., "--primary-color")
    pub name: String,
    /// Property value
    pub value: String,
}

/// A CSS import statement.
#[derive(Debug, Clone)]
pub struct CssImport {
    /// Import URL
    pub url: String,
    /// Media query (if any)
    pub media: Option<String>,
}

// =============================================================================
// Style Parser
// =============================================================================

/// Parser for style sections.
#[derive(Debug, Default)]
pub struct StyleParser {
    // Configuration can be added here
}

impl StyleParser {
    /// Create a new style parser.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse a style section.
    pub fn parse(&self, source: &str, scoped: bool, _path: &Path) -> DxResult<ParsedStyle> {
        let source = source.trim().to_string();
        let mut rules = Vec::new();
        let mut atomic_classes = Vec::new();
        let mut custom_properties = Vec::new();
        let mut imports = Vec::new();

        // Parse the CSS
        self.parse_css(
            &source,
            &mut rules,
            &mut atomic_classes,
            &mut custom_properties,
            &mut imports,
        );

        Ok(ParsedStyle {
            source,
            scoped,
            rules,
            atomic_classes,
            custom_properties,
            imports,
        })
    }

    /// Parse CSS content.
    fn parse_css(
        &self,
        source: &str,
        rules: &mut Vec<CssRule>,
        atomic_classes: &mut Vec<String>,
        custom_properties: &mut Vec<CustomProperty>,
        imports: &mut Vec<CssImport>,
    ) {
        let chars: Vec<char> = source.chars().collect();
        let mut pos = 0;

        while pos < chars.len() {
            // Skip whitespace and comments
            pos = self.skip_whitespace_and_comments(&chars, pos);
            if pos >= chars.len() {
                break;
            }

            // Check for at-rule
            if chars[pos] == '@' {
                let (at_rules, end) =
                    self.parse_at_rule(&chars, pos, atomic_classes, custom_properties, imports);
                rules.extend(at_rules);
                pos = end;
                continue;
            }

            // Parse regular rule
            let (rule, end) = self.parse_rule(&chars, pos, atomic_classes, custom_properties);
            if let Some(rule) = rule {
                rules.push(rule);
            }
            pos = end;
        }
    }

    /// Skip whitespace and comments.
    fn skip_whitespace_and_comments(&self, chars: &[char], start: usize) -> usize {
        let mut pos = start;

        while pos < chars.len() {
            // Skip whitespace
            while pos < chars.len() && chars[pos].is_whitespace() {
                pos += 1;
            }

            // Check for comment
            if pos + 1 < chars.len() && chars[pos] == '/' && chars[pos + 1] == '*' {
                pos += 2;
                while pos + 1 < chars.len() && !(chars[pos] == '*' && chars[pos + 1] == '/') {
                    pos += 1;
                }
                if pos + 1 < chars.len() {
                    pos += 2;
                }
            } else {
                break;
            }
        }

        pos
    }

    /// Parse an at-rule.
    fn parse_at_rule(
        &self,
        chars: &[char],
        start: usize,
        atomic_classes: &mut Vec<String>,
        custom_properties: &mut Vec<CustomProperty>,
        imports: &mut Vec<CssImport>,
    ) -> (Vec<CssRule>, usize) {
        let mut pos = start + 1; // Skip '@'

        // Parse at-rule name
        let mut name = String::new();
        while pos < chars.len() && chars[pos].is_alphabetic() {
            name.push(chars[pos]);
            pos += 1;
        }

        // Skip whitespace
        while pos < chars.len() && chars[pos].is_whitespace() {
            pos += 1;
        }

        match name.as_str() {
            "import" => {
                let (import, end) = self.parse_import(chars, pos);
                imports.push(import);
                return (Vec::new(), end);
            }
            "media" => {
                let (rules, end) =
                    self.parse_media_rule(chars, pos, atomic_classes, custom_properties);
                return (rules, end);
            }
            "container" => {
                let (rules, end) =
                    self.parse_container_rule(chars, pos, atomic_classes, custom_properties);
                return (rules, end);
            }
            "supports" => {
                let (rules, end) =
                    self.parse_supports_rule(chars, pos, atomic_classes, custom_properties);
                return (rules, end);
            }
            "layer" => {
                let (rules, end) =
                    self.parse_layer_rule(chars, pos, atomic_classes, custom_properties);
                return (rules, end);
            }
            "keyframes" => {
                let (rule, end) = self.parse_keyframes(chars, pos);
                return (rule.into_iter().collect(), end);
            }
            "variant" => {
                let (rules, end) =
                    self.parse_variant_rule(chars, pos, atomic_classes, custom_properties);
                return (rules, end);
            }
            "apply" => {
                // dx-style @apply-compatible directive
                let (classes, end) = self.parse_apply(chars, pos);
                atomic_classes.extend(classes);
                return (Vec::new(), end);
            }
            _ => {}
        }

        // Skip to end of rule
        let mut depth = 0;
        while pos < chars.len() {
            if chars[pos] == '{' {
                depth += 1;
            } else if chars[pos] == '}' {
                depth -= 1;
                if depth == 0 {
                    pos += 1;
                    break;
                }
            } else if chars[pos] == ';' && depth == 0 {
                pos += 1;
                break;
            }
            pos += 1;
        }

        (Vec::new(), pos)
    }

    /// Parse an @import rule.
    fn parse_import(&self, chars: &[char], start: usize) -> (CssImport, usize) {
        let mut pos = start;
        let mut url = String::new();
        let mut media = None;

        // Parse URL
        if pos < chars.len() && (chars[pos] == '"' || chars[pos] == '\'') {
            let quote = chars[pos];
            pos += 1;
            while pos < chars.len() && chars[pos] != quote {
                url.push(chars[pos]);
                pos += 1;
            }
            pos += 1;
        } else if self.starts_with(chars, pos, "url(") {
            pos += 4;
            while pos < chars.len() && chars[pos] != ')' {
                if chars[pos] != '"' && chars[pos] != '\'' {
                    url.push(chars[pos]);
                }
                pos += 1;
            }
            pos += 1;
        }

        // Skip whitespace
        while pos < chars.len() && chars[pos].is_whitespace() {
            pos += 1;
        }

        // Check for media query
        if pos < chars.len() && chars[pos] != ';' {
            let mut media_query = String::new();
            while pos < chars.len() && chars[pos] != ';' {
                media_query.push(chars[pos]);
                pos += 1;
            }
            if !media_query.trim().is_empty() {
                media = Some(media_query.trim().to_string());
            }
        }

        // Skip semicolon
        if pos < chars.len() && chars[pos] == ';' {
            pos += 1;
        }

        (CssImport { url, media }, pos)
    }

    /// Parse a @media rule.
    fn parse_media_rule(
        &self,
        chars: &[char],
        start: usize,
        atomic_classes: &mut Vec<String>,
        custom_properties: &mut Vec<CustomProperty>,
    ) -> (Vec<CssRule>, usize) {
        let (query, mut nested_rules, pos) =
            self.parse_nested_at_rule_block(chars, start, atomic_classes, custom_properties);
        let query = query.trim().to_string();
        wrap_rules_with_at_rule(&mut nested_rules, AtRule::Media { query });

        (nested_rules, pos)
    }

    /// Parse a @container rule.
    fn parse_container_rule(
        &self,
        chars: &[char],
        start: usize,
        atomic_classes: &mut Vec<String>,
        custom_properties: &mut Vec<CustomProperty>,
    ) -> (Vec<CssRule>, usize) {
        let (query, mut nested_rules, pos) =
            self.parse_nested_at_rule_block(chars, start, atomic_classes, custom_properties);
        let query = query.trim().to_string();
        wrap_rules_with_at_rule(&mut nested_rules, AtRule::Container { query });

        (nested_rules, pos)
    }

    /// Parse a @supports rule.
    fn parse_supports_rule(
        &self,
        chars: &[char],
        start: usize,
        atomic_classes: &mut Vec<String>,
        custom_properties: &mut Vec<CustomProperty>,
    ) -> (Vec<CssRule>, usize) {
        let (condition, mut nested_rules, pos) =
            self.parse_nested_at_rule_block(chars, start, atomic_classes, custom_properties);
        let condition = condition.trim().to_string();
        wrap_rules_with_at_rule(&mut nested_rules, AtRule::Supports { condition });

        (nested_rules, pos)
    }

    /// Parse a block-form @layer rule.
    fn parse_layer_rule(
        &self,
        chars: &[char],
        start: usize,
        atomic_classes: &mut Vec<String>,
        custom_properties: &mut Vec<CustomProperty>,
    ) -> (Vec<CssRule>, usize) {
        let mut pos = start;
        while pos < chars.len() && chars[pos] != '{' && chars[pos] != ';' {
            pos += 1;
        }

        if pos < chars.len() && chars[pos] == ';' {
            return (Vec::new(), pos + 1);
        }

        let (name, mut nested_rules, pos) =
            self.parse_nested_at_rule_block(chars, start, atomic_classes, custom_properties);
        let name = name.trim();
        let name = (!name.is_empty()).then(|| name.to_string());
        wrap_rules_with_at_rule(&mut nested_rules, AtRule::Layer { name });

        (nested_rules, pos)
    }

    fn parse_variant_rule(
        &self,
        chars: &[char],
        start: usize,
        atomic_classes: &mut Vec<String>,
        custom_properties: &mut Vec<CustomProperty>,
    ) -> (Vec<CssRule>, usize) {
        let (variant, mut nested_rules, pos) =
            self.parse_nested_at_rule_block(chars, start, atomic_classes, custom_properties);
        let variant = variant.trim();
        if apply_css_variant_to_rules(&mut nested_rules, variant) {
            (nested_rules, pos)
        } else {
            (Vec::new(), pos)
        }
    }

    fn parse_nested_at_rule_block(
        &self,
        chars: &[char],
        start: usize,
        atomic_classes: &mut Vec<String>,
        custom_properties: &mut Vec<CustomProperty>,
    ) -> (String, Vec<CssRule>, usize) {
        let mut pos = start;

        let mut prelude = String::new();
        while pos < chars.len() && chars[pos] != '{' {
            prelude.push(chars[pos]);
            pos += 1;
        }

        if pos >= chars.len() {
            return (prelude, Vec::new(), pos);
        }

        pos += 1;
        let mut depth = 1;
        let content_start = pos;

        while pos < chars.len() && depth > 0 {
            if chars[pos] == '{' {
                depth += 1;
            } else if chars[pos] == '}' {
                depth -= 1;
            }
            pos += 1;
        }

        let content_end = pos.saturating_sub(1).max(content_start);
        let content: String = chars[content_start..content_end].iter().collect();
        let mut nested_rules = Vec::new();
        self.parse_css(
            &content,
            &mut nested_rules,
            atomic_classes,
            custom_properties,
            &mut Vec::new(),
        );

        (prelude, nested_rules, pos)
    }

    /// Parse a @keyframes rule.
    fn parse_keyframes(&self, chars: &[char], start: usize) -> (Option<CssRule>, usize) {
        let mut pos = start;

        // Parse animation name
        let mut name = String::new();
        while pos < chars.len() && chars[pos] != '{' && !chars[pos].is_whitespace() {
            name.push(chars[pos]);
            pos += 1;
        }

        // Skip to end of rule
        let mut depth = 0;
        while pos < chars.len() {
            if chars[pos] == '{' {
                depth += 1;
            } else if chars[pos] == '}' {
                depth -= 1;
                if depth == 0 {
                    pos += 1;
                    break;
                }
            }
            pos += 1;
        }

        (
            Some(CssRule {
                selector: format!("@keyframes {name}"),
                declarations: Vec::new(),
                at_rule: Some(AtRule::Keyframes {
                    name: name.trim().to_string(),
                }),
            }),
            pos,
        )
    }

    /// Parse @apply directive (Tailwind-style).
    fn parse_apply(&self, chars: &[char], start: usize) -> (Vec<String>, usize) {
        let mut pos = start;
        let mut classes = Vec::new();

        while pos < chars.len() && chars[pos] != ';' {
            // Skip whitespace
            while pos < chars.len() && chars[pos].is_whitespace() {
                pos += 1;
            }

            // Parse class name
            let mut class = String::new();
            while pos < chars.len() && !chars[pos].is_whitespace() && chars[pos] != ';' {
                class.push(chars[pos]);
                pos += 1;
            }

            if !class.is_empty() {
                classes.push(class);
            }
        }

        // Skip semicolon
        if pos < chars.len() && chars[pos] == ';' {
            pos += 1;
        }

        (classes, pos)
    }

    /// Parse a regular CSS rule.
    fn parse_rule(
        &self,
        chars: &[char],
        start: usize,
        atomic_classes: &mut Vec<String>,
        custom_properties: &mut Vec<CustomProperty>,
    ) -> (Option<CssRule>, usize) {
        let mut pos = start;

        // Parse selector
        let mut selector = String::new();
        while pos < chars.len() && chars[pos] != '{' {
            selector.push(chars[pos]);
            pos += 1;
        }

        let selector = selector.trim().to_string();
        if selector.is_empty() {
            return (None, pos);
        }

        // Extract class names from selector
        for word in selector.split_whitespace() {
            if word.starts_with('.') {
                let class = word.trim_start_matches('.').split([':', '[']).next();
                if let Some(class) = class {
                    if self.is_atomic_class(class) {
                        atomic_classes.push(class.to_string());
                    }
                }
            }
        }

        // Skip opening brace
        if pos < chars.len() {
            pos += 1;
        }

        // Parse declarations
        let mut declarations = Vec::new();
        while pos < chars.len() && chars[pos] != '}' {
            // Skip whitespace
            pos = self.skip_whitespace_and_comments(chars, pos);
            if pos >= chars.len() || chars[pos] == '}' {
                break;
            }

            // Parse property
            let mut property = String::new();
            while pos < chars.len() && chars[pos] != ':' && chars[pos] != '}' {
                property.push(chars[pos]);
                pos += 1;
            }

            let property = property.trim().to_string();
            if property.is_empty() || pos >= chars.len() || chars[pos] == '}' {
                break;
            }

            // Skip colon
            pos += 1;

            // Parse value
            let mut value = String::new();
            let mut important = false;
            while pos < chars.len() && chars[pos] != ';' && chars[pos] != '}' {
                value.push(chars[pos]);
                pos += 1;
            }

            let value = value.trim().to_string();
            if value.ends_with("!important") {
                important = true;
            }
            let value = value.trim_end_matches("!important").trim().to_string();

            // Check for custom property
            if property.starts_with("--") {
                custom_properties.push(CustomProperty {
                    name: property.clone(),
                    value: value.clone(),
                });
            }

            declarations.push(CssDeclaration {
                property,
                value,
                important,
            });

            // Skip semicolon
            if pos < chars.len() && chars[pos] == ';' {
                pos += 1;
            }
        }

        // Skip closing brace
        if pos < chars.len() && chars[pos] == '}' {
            pos += 1;
        }

        (
            Some(CssRule {
                selector,
                declarations,
                at_rule: None,
            }),
            pos,
        )
    }

    /// Check if a class name looks like an atomic class.
    fn is_atomic_class(&self, class: &str) -> bool {
        // Common atomic class patterns
        let patterns = [
            "flex",
            "grid",
            "block",
            "inline",
            "hidden",
            "w-",
            "h-",
            "m-",
            "p-",
            "gap-",
            "text-",
            "font-",
            "bg-",
            "border-",
            "rounded",
            "shadow",
            "opacity",
            "justify-",
            "items-",
            "self-",
            "absolute",
            "relative",
            "fixed",
            "sticky",
            "top-",
            "right-",
            "bottom-",
            "left-",
            "z-",
            "overflow-",
        ];

        patterns.iter().any(|p| class.starts_with(p) || class == *p)
    }

    /// Check if chars starting at pos match the pattern.
    fn starts_with(&self, chars: &[char], pos: usize, pattern: &str) -> bool {
        let pattern_chars: Vec<char> = pattern.chars().collect();
        if pos + pattern_chars.len() > chars.len() {
            return false;
        }
        for (i, &c) in pattern_chars.iter().enumerate() {
            if chars[pos + i] != c {
                return false;
            }
        }
        true
    }
}

fn wrap_rules_with_at_rule(rules: &mut [CssRule], wrapper: AtRule) {
    for rule in rules {
        wrap_rule_with_at_rule(rule, wrapper.clone());
    }
}

fn wrap_rule_with_at_rule(rule: &mut CssRule, wrapper: AtRule) {
    rule.at_rule = Some(match rule.at_rule.take() {
        Some(AtRule::Nested { mut rules }) => {
            rules.insert(0, wrapper);
            AtRule::Nested { rules }
        }
        Some(existing) => AtRule::Nested {
            rules: vec![wrapper, existing],
        },
        None => wrapper,
    });
}

fn apply_css_variant_to_rules(rules: &mut [CssRule], variant: &str) -> bool {
    let Some(transform) = css_variant_transform(variant) else {
        return false;
    };
    for rule in rules {
        apply_css_variant_to_rule(rule, transform);
    }
    true
}

fn apply_css_variant_to_rule(rule: &mut CssRule, transform: CssVariantTransform) {
    rule.selector = match transform {
        CssVariantTransform::Suffix(suffix) => selector_list_suffix(&rule.selector, suffix),
        CssVariantTransform::Prefix(prefix) => selector_list_prefix(&rule.selector, prefix),
    };
}

#[derive(Clone, Copy)]
enum CssVariantTransform {
    Suffix(&'static str),
    Prefix(&'static str),
}

fn css_variant_transform(variant: &str) -> Option<CssVariantTransform> {
    match variant {
        "hover" => Some(CssVariantTransform::Suffix(":hover")),
        "focus" => Some(CssVariantTransform::Suffix(":focus")),
        "focus-visible" => Some(CssVariantTransform::Suffix(":focus-visible")),
        "focus-within" => Some(CssVariantTransform::Suffix(":focus-within")),
        "active" => Some(CssVariantTransform::Suffix(":active")),
        "disabled" => Some(CssVariantTransform::Suffix(":disabled")),
        "checked" => Some(CssVariantTransform::Suffix(":checked")),
        "visited" => Some(CssVariantTransform::Suffix(":visited")),
        "dark" => Some(CssVariantTransform::Prefix(".dark ")),
        _ => None,
    }
}

fn selector_list_suffix(selector: &str, suffix: &str) -> String {
    selector
        .split(',')
        .map(|part| format!("{}{suffix}", part.trim()))
        .collect::<Vec<_>>()
        .join(", ")
}

fn selector_list_prefix(selector: &str, prefix: &str) -> String {
    selector
        .split(',')
        .map(|part| format!("{prefix}{}", part.trim()))
        .collect::<Vec<_>>()
        .join(", ")
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_simple_style() {
        let parser = StyleParser::new();
        let source = r#"
            .container {
                max-width: 1200px;
                margin: 0 auto;
            }
        "#;

        let result = parser.parse(source, false, &PathBuf::from("test.html"));
        assert!(result.is_ok());

        let style = result.unwrap();
        assert_eq!(style.rules.len(), 1);
        assert_eq!(style.rules[0].declarations.len(), 2);
    }

    #[test]
    fn test_parse_scoped_style() {
        let parser = StyleParser::new();
        let source = "h1 { color: blue; }";

        let result = parser.parse(source, true, &PathBuf::from("test.html"));
        assert!(result.is_ok());

        let style = result.unwrap();
        assert!(style.scoped);
    }

    #[test]
    fn test_parse_import() {
        let parser = StyleParser::new();
        let source = r#"
            @import "other.css";
            .test { color: red; }
        "#;

        let result = parser.parse(source, false, &PathBuf::from("test.html"));
        assert!(result.is_ok());

        let style = result.unwrap();
        assert_eq!(style.imports.len(), 1);
        assert_eq!(style.imports[0].url, "other.css");
    }

    #[test]
    fn test_detect_atomic_classes() {
        let parser = StyleParser::new();
        let source = r#"
            .flex { display: flex; }
            .w-full { width: 100%; }
            .bg-blue-500 { background-color: blue; }
        "#;

        let result = parser.parse(source, false, &PathBuf::from("test.html"));
        assert!(result.is_ok());

        let style = result.unwrap();
        assert!(style.atomic_classes.contains(&"flex".to_string()));
        assert!(style.atomic_classes.contains(&"w-full".to_string()));
        assert!(style.atomic_classes.contains(&"bg-blue-500".to_string()));
    }

    #[test]
    fn test_parse_custom_properties() {
        let parser = StyleParser::new();
        let source = r#"
            :root {
                --primary-color: blue;
                --spacing: 1rem;
            }
        "#;

        let result = parser.parse(source, false, &PathBuf::from("test.html"));
        assert!(result.is_ok());

        let style = result.unwrap();
        assert_eq!(style.custom_properties.len(), 2);
    }

    #[test]
    fn extract_class_attribute_tokens_reads_class_and_class_name_attributes() {
        let source = r#"
            <main className="dx-shell p-4 hover:bg-blue-500">
                <section class='dx-card text-sm'></section>
                <div class="dx-grid"></div>
            </main>
        "#;

        let tokens = extract_class_attribute_tokens(source);

        assert!(tokens.contains(&"dx-shell".to_string()));
        assert!(tokens.contains(&"p-4".to_string()));
        assert!(tokens.contains(&"hover:bg-blue-500".to_string()));
        assert!(tokens.contains(&"dx-card".to_string()));
        assert!(tokens.contains(&"text-sm".to_string()));
        assert!(tokens.contains(&"dx-grid".to_string()));
    }

    #[test]
    fn extract_class_attribute_tokens_reads_static_tsx_expression_strings() {
        let source = r#"
            <main className={`dx-template text-lg`}>
                <section className={"dx-panel gap-4"}></section>
                <div class={'dx-inline'}></div>
                <span class={`dx-chip`}></span>
            </main>
        "#;

        let tokens = extract_class_attribute_tokens(source);

        assert!(tokens.contains(&"dx-template".to_string()));
        assert!(tokens.contains(&"text-lg".to_string()));
        assert!(tokens.contains(&"dx-panel".to_string()));
        assert!(tokens.contains(&"gap-4".to_string()));
        assert!(tokens.contains(&"dx-inline".to_string()));
        assert!(tokens.contains(&"dx-chip".to_string()));
    }

    #[test]
    fn extract_class_attribute_tokens_reads_static_helper_function_strings() {
        let source = r#"
            const panel = classes("dx-panel", compact && "gap-2");
            const shell = dxClass("dx-shell", ready && "opacity-100");
            const button = cn("dx-button px-4", active && "dx-active");
            const icon = cx("dx-icon", disabled && "opacity-50");
            const card = clsx('dx-card', selected ? 'dx-selected' : undefined);
            const row = classNames("dx-row", selected && "dx-row-selected");
            const badge = cva(`dx-badge text-sm`);
        "#;

        let tokens = extract_class_attribute_tokens(source);

        assert!(tokens.contains(&"dx-panel".to_string()));
        assert!(tokens.contains(&"gap-2".to_string()));
        assert!(tokens.contains(&"dx-shell".to_string()));
        assert!(tokens.contains(&"opacity-100".to_string()));
        assert!(tokens.contains(&"dx-button".to_string()));
        assert!(tokens.contains(&"px-4".to_string()));
        assert!(tokens.contains(&"dx-active".to_string()));
        assert!(tokens.contains(&"dx-icon".to_string()));
        assert!(tokens.contains(&"opacity-50".to_string()));
        assert!(tokens.contains(&"dx-card".to_string()));
        assert!(tokens.contains(&"dx-selected".to_string()));
        assert!(tokens.contains(&"dx-row".to_string()));
        assert!(tokens.contains(&"dx-row-selected".to_string()));
        assert!(tokens.contains(&"dx-badge".to_string()));
        assert!(tokens.contains(&"text-sm".to_string()));
    }

    #[test]
    fn extract_class_attribute_tokens_skips_dynamic_template_placeholders() {
        let source = r#"
            <main className={`dx-card dx-${variant} text-sm`}></main>
            const button = cn(`dx-button dx-${size}`, "dx-static");
        "#;

        let tokens = extract_class_attribute_tokens(source);

        assert!(tokens.contains(&"dx-card".to_string()));
        assert!(tokens.contains(&"text-sm".to_string()));
        assert!(tokens.contains(&"dx-button".to_string()));
        assert!(tokens.contains(&"dx-static".to_string()));
        assert!(!tokens.iter().any(|token| token.contains("${")));
        assert!(!tokens.iter().any(|token| token.contains('$')));
    }

    #[test]
    fn extract_class_attribute_tokens_preserves_parenthesized_dx_token_classes() {
        let source = r#"
            <main className="bg-token(surface) text-token(foreground) border-token(border) ring-token(ring)">
                <section className={"bg-size-(--dx-bg-size)"}></section>
            </main>
        "#;

        let tokens = extract_class_attribute_tokens(source);

        assert!(tokens.contains(&"bg-token(surface)".to_string()));
        assert!(tokens.contains(&"text-token(foreground)".to_string()));
        assert!(tokens.contains(&"border-token(border)".to_string()));
        assert!(tokens.contains(&"ring-token(ring)".to_string()));
        assert!(tokens.contains(&"bg-size-(--dx-bg-size)".to_string()));
    }

    #[test]
    fn extract_class_attribute_tokens_expands_grouped_classnames() {
        let source = r#"
            <main className="hover:(bg-accent text-accent-foreground shadow-sm) md:(grid grid-cols-2 gap-4) dark:hover:(bg-card text-foreground) group-hover:(opacity-100 translate-y-0) bg-token(surface)">
            </main>
            const classes = cn("hover:(bg-accent text-accent-foreground shadow-sm)", `md:(grid grid-cols-2 gap-4)`);
        "#;

        let tokens = extract_class_attribute_tokens(source);

        for class_name in [
            "hover:bg-accent",
            "hover:text-accent-foreground",
            "hover:shadow-sm",
            "md:grid",
            "md:grid-cols-2",
            "md:gap-4",
            "dark:hover:bg-card",
            "dark:hover:text-foreground",
            "group-hover:opacity-100",
            "group-hover:translate-y-0",
            "bg-token(surface)",
        ] {
            assert!(tokens.contains(&class_name.to_string()));
        }
    }

    #[test]
    fn extract_class_attribute_tokens_reports_invalid_grouped_classnames() {
        let source = r#"
            <main className="md:(grid grid-cols-2 gap-4"></main>
        "#;

        let tokens = extract_class_attribute_tokens(source);

        assert!(tokens.contains(&"md:grid".to_string()));
        assert!(tokens.contains(&"md:grid-cols-2".to_string()));
        assert!(tokens.contains(&"md:gap-4".to_string()));
        assert!(
            tokens
                .iter()
                .any(|token| token.starts_with("dx-grouping-error:unclosed-group:md"))
        );
    }

    #[test]
    fn parse_variant_rule_expands_common_css_variants() {
        let parser = StyleParser::new();
        let source = r#"
            @variant hover {
                .button { color: red; }
            }
            @variant dark {
                .card { display: grid; }
            }
        "#;

        let style = parser
            .parse(source, false, &PathBuf::from("variant.css"))
            .expect("parse variant CSS");

        assert_eq!(style.rules.len(), 2);
        assert_eq!(style.rules[0].selector, ".button:hover");
        assert_eq!(style.rules[1].selector, ".dark .card");
    }
}
