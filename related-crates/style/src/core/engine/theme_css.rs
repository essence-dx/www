use ahash::AHashMap;

use super::css_functions;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ThemeCssDefinition {
    pub tokens: Vec<(String, String)>,
    pub custom_variants: Vec<CssCustomVariant>,
    pub utilities: Vec<CssUtilityDefinition>,
    pub source_directives: Vec<CssSourceDirective>,
    pub reference_directives: Vec<CssReferenceDirective>,
    pub diagnostics: Vec<CssDirectiveDiagnostic>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssCustomVariant {
    pub name: String,
    pub selector: String,
    pub media_queries: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssUtilityDefinition {
    pub name: String,
    pub declarations: Vec<(String, String)>,
    pub nested_rules: Vec<CssUtilityNestedRule>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssUtilityNestedRule {
    pub selector_suffix: String,
    pub declarations: Vec<(String, String)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CssSourceDirective {
    Scan(String),
    Inline(Vec<String>),
    InlineExclude(Vec<String>),
    Exclude(String),
    DisableAutomaticDetection,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CssSourceScanPlan {
    pub disable_automatic_detection: bool,
    pub include_paths: Vec<String>,
    pub exclude_paths: Vec<String>,
    pub inline_classes: Vec<String>,
    pub inline_exclusions: Vec<String>,
}

impl CssSourceScanPlan {
    pub fn effective_inline_classes(&self) -> Vec<String> {
        self.inline_classes
            .iter()
            .filter(|class_name| !self.inline_exclusions.contains(class_name))
            .cloned()
            .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CssReferenceDirective {
    Local(String),
    TailwindDefaultTheme,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssDirectiveDiagnostic {
    pub directive: String,
    pub reason: String,
    pub line: usize,
}

pub const DEFAULT_DX_THEME_CSS: &str = r#"
@theme {
  --color-background: hsl(var(--background));
  --color-foreground: hsl(var(--foreground));
  --color-surface: hsl(var(--surface));
  --color-muted: hsl(var(--muted));
  --color-muted-surface: hsl(var(--muted-surface));
  --color-border: hsl(var(--border));
  --color-card: hsl(var(--card));
  --color-accent: hsl(var(--accent));
  --color-accent-foreground: hsl(var(--accent-foreground));
  --color-success: hsl(var(--success));
  --color-warning: hsl(var(--warning));
  --color-danger: hsl(var(--danger));
  --color-ring: hsl(var(--ring));
  --spacing: 0.25rem;
  --radius-default: var(--radius);
  --breakpoint-sm: 640px;
  --breakpoint-md: 768px;
  --breakpoint-lg: 1024px;
  --breakpoint-xl: 1280px;
  --breakpoint-2xl: 1536px;
  --color-red-500: oklch(63.7% 0.237 25.331);
  --color-emerald-500: oklch(69.6% 0.17 162.48);
  --color-slate-950: oklch(12.9% 0.042 264.695);
  --text-xs: 0.75rem;
  --text-sm: 0.875rem;
  --text-base: 1rem;
  --text-lg: 1.125rem;
  --text-xl: 1.25rem;
  --text-2xl: 1.5rem;
  --text-3xl: 1.875rem;
  --text-4xl: 2.25rem;
  --text-5xl: 3rem;
  --text-6xl: 3.75rem;
  --text-7xl: 4.5rem;
  --text-8xl: 6rem;
  --text-9xl: 8rem;
  --container-3xs: 16rem;
  --container-2xs: 18rem;
  --container-xs: 20rem;
  --container-sm: 24rem;
  --container-md: 28rem;
  --container-lg: 32rem;
  --container-xl: 36rem;
  --container-2xl: 42rem;
  --color-mauve-50: oklch(98.5% 0 0);
  --color-mauve-100: oklch(96% 0.003 325.6);
  --color-mauve-200: oklch(92.2% 0.005 325.62);
  --color-mauve-300: oklch(86.5% 0.012 325.68);
  --color-mauve-400: oklch(71.1% 0.019 323.02);
  --color-mauve-500: oklch(54.2% 0.034 322.5);
  --color-mauve-600: oklch(43.5% 0.029 321.78);
  --color-mauve-700: oklch(36.4% 0.029 323.89);
  --color-mauve-800: oklch(26.3% 0.024 320.12);
  --color-mauve-900: oklch(21.2% 0.019 322.12);
  --color-mauve-950: oklch(14.5% 0.008 326);
  --color-olive-50: oklch(98.8% 0.003 106.5);
  --color-olive-100: oklch(96.6% 0.005 106.5);
  --color-olive-200: oklch(93% 0.007 106.5);
  --color-olive-300: oklch(88% 0.011 106.6);
  --color-olive-400: oklch(73.7% 0.021 106.9);
  --color-olive-500: oklch(58% 0.031 107.3);
  --color-olive-600: oklch(46.6% 0.025 107.3);
  --color-olive-700: oklch(39.4% 0.023 107.4);
  --color-olive-800: oklch(28.6% 0.016 107.4);
  --color-olive-900: oklch(22.8% 0.013 107.4);
  --color-olive-950: oklch(15.3% 0.006 107.1);
  --color-mist-50: oklch(98.7% 0.002 197.1);
  --color-mist-100: oklch(96.3% 0.002 197.1);
  --color-mist-200: oklch(92.5% 0.005 214.3);
  --color-mist-300: oklch(87.2% 0.007 219.6);
  --color-mist-400: oklch(72.3% 0.014 214.4);
  --color-mist-500: oklch(56% 0.021 213.5);
  --color-mist-600: oklch(45% 0.017 213.2);
  --color-mist-700: oklch(37.8% 0.015 216);
  --color-mist-800: oklch(27.5% 0.011 216.9);
  --color-mist-900: oklch(21.8% 0.008 223.9);
  --color-mist-950: oklch(14.8% 0.004 228.8);
  --color-taupe-50: oklch(98.6% 0.002 67.8);
  --color-taupe-100: oklch(96% 0.002 17.2);
  --color-taupe-200: oklch(92.2% 0.005 34.3);
  --color-taupe-300: oklch(86.8% 0.007 39.5);
  --color-taupe-400: oklch(71.4% 0.014 41.2);
  --color-taupe-500: oklch(54.7% 0.021 43.1);
  --color-taupe-600: oklch(43.8% 0.017 39.3);
  --color-taupe-700: oklch(36.7% 0.016 35.7);
  --color-taupe-800: oklch(26.8% 0.011 36.5);
  --color-taupe-900: oklch(21.4% 0.009 43.1);
  --color-taupe-950: oklch(14.7% 0.004 49.3);
}
"#;

const REGISTERED_CUSTOM_PROPERTIES: &[(&str, &str, &str, bool)] = &[
    ("--tw-gradient-from", "<color>", "transparent", false),
    ("--tw-gradient-via", "<color>", "transparent", false),
    ("--tw-gradient-to", "<color>", "transparent", false),
    ("--tw-gradient-position", "*", "initial", false),
    ("--tw-ring-color", "<color>", "currentColor", false),
    ("--tw-ring-offset-color", "<color>", "#fff", false),
    ("--tw-shadow", "*", "0 0 #0000", false),
    ("--tw-shadow-color", "*", "currentColor", false),
    ("--tw-shadow-alpha", "<percentage>", "100%", false),
    ("--tw-inset-shadow", "*", "0 0 #0000", false),
    ("--tw-inset-shadow-color", "*", "currentColor", false),
    ("--tw-inset-shadow-alpha", "<percentage>", "100%", false),
    ("--tw-inset-ring-shadow", "*", "inset 0 0 #0000", false),
    ("--tw-inset-ring-color", "<color>", "currentColor", false),
    ("--tw-drop-shadow-color", "*", "currentColor", false),
    ("--tw-drop-shadow-alpha", "<percentage>", "100%", false),
    ("--tw-drop-shadow-size", "*", "none", false),
    ("--tw-translate-x", "<length-percentage>", "0px", false),
    ("--tw-translate-y", "<length-percentage>", "0px", false),
    ("--tw-translate-z", "<length-percentage>", "0px", false),
    ("--tw-rotate", "<angle>", "0deg", false),
    ("--tw-rotate-x", "<angle>", "0deg", false),
    ("--tw-rotate-y", "<angle>", "0deg", false),
    ("--tw-rotate-z", "<angle>", "0deg", false),
    ("--tw-skew-x", "<angle>", "0deg", false),
    ("--tw-skew-y", "<angle>", "0deg", false),
    ("--tw-scale-x", "<number>", "1", false),
    ("--tw-scale-y", "<number>", "1", false),
    ("--tw-scale-z", "<number>", "1", false),
    ("--tw-border-spacing-x", "<length-percentage>", "0px", false),
    ("--tw-border-spacing-y", "<length-percentage>", "0px", false),
    ("--tw-space-x-reverse", "*", "0", false),
    ("--tw-space-y-reverse", "*", "0", false),
    ("--tw-blur", "*", "", false),
    ("--tw-brightness", "*", "", false),
    ("--tw-contrast", "*", "", false),
    ("--tw-drop-shadow", "*", "", false),
    ("--tw-grayscale", "*", "", false),
    ("--tw-hue-rotate", "*", "", false),
    ("--tw-invert", "*", "", false),
    ("--tw-saturate", "*", "", false),
    ("--tw-sepia", "*", "", false),
    ("--tw-backdrop-blur", "*", "", false),
    ("--tw-backdrop-brightness", "*", "", false),
    ("--tw-backdrop-contrast", "*", "", false),
    ("--tw-backdrop-grayscale", "*", "", false),
    ("--tw-backdrop-hue-rotate", "*", "", false),
    ("--tw-backdrop-invert", "*", "", false),
    ("--tw-backdrop-opacity", "*", "", false),
    ("--tw-backdrop-saturate", "*", "", false),
    ("--tw-backdrop-sepia", "*", "", false),
    ("--tw-pan-x", "*", "initial", false),
    ("--tw-pan-y", "*", "initial", false),
    ("--tw-pinch-zoom", "*", "initial", false),
];

pub fn parse_theme_css(source: &str) -> ThemeCssDefinition {
    let mut definition = ThemeCssDefinition::default();
    let mut rest = source;

    while let Some(theme_index) = rest.find("@theme") {
        let after_theme = &rest[theme_index + "@theme".len()..];
        let Some(open_offset) = after_theme.find('{') else {
            break;
        };
        let block_start = theme_index + "@theme".len() + open_offset + 1;
        let Some((block_end, block)) = matching_block(rest, block_start) else {
            break;
        };

        definition.tokens.extend(parse_theme_declarations(block));
        rest = &rest[block_end + 1..];
    }

    parse_line_directives(source, &mut definition);
    parse_custom_variant_directives(source, &mut definition);
    parse_utility_directives(source, &mut definition);

    definition
}

pub fn css_first_directive_diagnostics(source: &str) -> Vec<CssDirectiveDiagnostic> {
    parse_theme_css(source).diagnostics
}

pub fn css_source_directives(source: &str) -> Vec<CssSourceDirective> {
    parse_theme_css(source).source_directives
}

pub fn css_source_scan_plan(source: &str) -> CssSourceScanPlan {
    let mut plan = CssSourceScanPlan::default();

    for directive in css_source_directives(source) {
        match directive {
            CssSourceDirective::Scan(path) => push_unique(&mut plan.include_paths, path),
            CssSourceDirective::Exclude(path) => push_unique(&mut plan.exclude_paths, path),
            CssSourceDirective::Inline(classes) => plan.inline_classes.extend(classes),
            CssSourceDirective::InlineExclude(classes) => {
                plan.inline_exclusions.extend(classes);
            }
            CssSourceDirective::DisableAutomaticDetection => {
                plan.disable_automatic_detection = true;
            }
        }
    }

    plan.inline_classes.sort();
    plan.inline_classes.dedup();
    plan.inline_exclusions.sort();
    plan.inline_exclusions.dedup();
    plan
}

pub fn css_reference_directives(source: &str) -> Vec<CssReferenceDirective> {
    parse_theme_css(source).reference_directives
}

pub fn css_source_inline_class_tokens(source: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    for directive in css_source_directives(source) {
        if let CssSourceDirective::Inline(classes) = directive {
            tokens.extend(classes);
        }
    }
    tokens
}

pub fn css_source_inline_exclusion_class_tokens(source: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    for directive in css_source_directives(source) {
        if let CssSourceDirective::InlineExclude(classes) = directive {
            tokens.extend(classes);
        }
    }
    tokens
}

pub fn css_source_disables_automatic_detection(source: &str) -> bool {
    css_source_directives(source)
        .into_iter()
        .any(|directive| directive == CssSourceDirective::DisableAutomaticDetection)
}

pub fn theme_layer_css_from_source(source: &str) -> String {
    let definition = parse_theme_css(source);
    let tokens = unique_tokens(definition.tokens);
    let mut css = String::from("@layer theme, base, components, utilities;\n\n");
    css.push_str(&registered_custom_properties_css());

    let tokens: Vec<(String, String)> = tokens
        .into_iter()
        .filter(|(name, value)| is_emitted_theme_token(name, value))
        .collect();

    if !tokens.is_empty() {
        css.push_str("@layer theme {\n  :root {\n");
        for (name, value) in tokens {
            css.push_str("    ");
            css.push_str(&name);
            css.push_str(": ");
            css.push_str(&value);
            css.push_str(";\n");
        }
        css.push_str("  }\n}\n");
    }

    css
}

pub fn registered_custom_properties_css() -> String {
    let mut css = String::new();
    for (name, syntax, initial, inherits) in REGISTERED_CUSTOM_PROPERTIES {
        css.push_str("@property ");
        css.push_str(name);
        css.push_str(" {\n  syntax: \"");
        css.push_str(syntax);
        css.push_str("\";\n  inherits: ");
        css.push_str(if *inherits { "true" } else { "false" });
        css.push_str(";\n  initial-value: ");
        if initial.is_empty() {
            css.push_str("none");
        } else {
            css.push_str(initial);
        }
        css.push_str(";\n}\n\n");
    }
    css
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

fn parse_theme_declarations(block: &str) -> Vec<(String, String)> {
    split_declarations(block)
        .into_iter()
        .filter_map(|declaration| {
            let (name, value) = declaration.split_once(':')?;
            let name = name.trim();
            let value = value.trim();
            if !is_theme_token_name(name) || !is_safe_theme_value(value) {
                return None;
            }
            Some((name.to_string(), value.to_string()))
        })
        .collect()
}

fn parse_line_directives(source: &str, definition: &mut ThemeCssDefinition) {
    let mut line_start = 0usize;
    for (line_index, raw_line) in source.split_inclusive('\n').enumerate() {
        let line = raw_line.trim_end_matches('\n').trim_end_matches('\r');
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("/*") || trimmed.starts_with("//") {
            line_start += raw_line.len();
            continue;
        }
        let line_number = line_index + 1;

        if trimmed.starts_with("@plugin") {
            push_diagnostic(
                definition,
                "@plugin",
                "Tailwind JS plugin execution is not supported by dx-style; register DX-owned CSS utilities or engine support instead.",
                line_number,
            );
        } else if trimmed.starts_with("@config") {
            push_diagnostic(
                definition,
                "@config",
                "Tailwind JS config loading is not supported by dx-style; use CSS @theme tokens and DX-owned receipts instead.",
                line_number,
            );
        } else if trimmed.starts_with("@tailwind") {
            push_diagnostic(
                definition,
                "@tailwind",
                "Legacy Tailwind layer directives are not runtime inputs for dx-style; use @import \"tailwindcss\" only as migration input or authored DX CSS.",
                line_number,
            );
        } else if let Some(apply_body) = trimmed.strip_prefix("@apply") {
            let apply_body = apply_body.trim().trim_end_matches(';').trim();
            if apply_body.is_empty()
                || apply_body
                    .split_whitespace()
                    .any(|token| !is_variant_safe_apply_token(token))
            {
                push_diagnostic(
                    definition,
                    "@apply",
                    "dx-style @apply currently supports plain or variant-safe resolvable utility tokens; arbitrary at-rule, JS/runtime, and unsafe tokens remain unsupported.",
                    line_number,
                );
            }
        } else if trimmed.starts_with("@reference") {
            parse_reference_directive(trimmed, line_number, definition);
        } else if trimmed.starts_with("@source") {
            parse_source_directive(trimmed, line_number, definition);
        }

        if !trimmed.starts_with("@utility") && !is_inside_utility_block(source, line_start) {
            if trimmed.contains("--alpha(")
                && css_functions::replace_tailwind_css_functions(trimmed).is_none()
            {
                push_diagnostic(
                    definition,
                    "--alpha()",
                    "standalone authored CSS --alpha() could not be transformed safely; use color / opacity syntax that dx-style can lower.",
                    line_number,
                );
            }
            if trimmed.contains("--spacing(")
                && css_functions::replace_tailwind_css_functions(trimmed).is_none()
            {
                push_diagnostic(
                    definition,
                    "--spacing()",
                    "standalone authored CSS --spacing() could not be transformed safely; use numeric or safe CSS variable spacing arguments.",
                    line_number,
                );
            }
        }

        line_start += raw_line.len();
    }
}

fn is_inside_utility_block(source: &str, position: usize) -> bool {
    let mut search_start = 0usize;
    while let Some(relative_start) = source[search_start..].find("@utility") {
        let start = search_start + relative_start;
        let after = start + "@utility".len();
        let Some(open_offset) = source[after..].find('{') else {
            return false;
        };
        let content_start = after + open_offset + 1;
        let Some((block_end, _)) = matching_block(source, content_start) else {
            return false;
        };
        if position > content_start && position < block_end {
            return true;
        }
        search_start = block_end + 1;
    }

    false
}

fn parse_reference_directive(trimmed: &str, line: usize, definition: &mut ThemeCssDefinition) {
    let rest = trimmed
        .strip_prefix("@reference")
        .unwrap_or_default()
        .trim_start()
        .trim_end_matches(';')
        .trim();

    let Some(specifier) = quoted_argument(rest) else {
        push_diagnostic(
            definition,
            "@reference",
            "Expected @reference \"./tokens.css\" or @reference \"tailwindcss\".",
            line,
        );
        return;
    };

    if specifier == "tailwindcss" {
        definition
            .reference_directives
            .push(CssReferenceDirective::TailwindDefaultTheme);
        return;
    }

    if is_local_css_reference_specifier(&specifier) {
        definition
            .reference_directives
            .push(CssReferenceDirective::Local(specifier));
        return;
    }

    push_diagnostic(
        definition,
        "@reference",
        "dx-style @reference supports local CSS files and \"tailwindcss\" as a DX-owned default-theme reference; package, URL, and JS/runtime reference resolution is unsupported.",
        line,
    );
}

fn parse_source_directive(trimmed: &str, line: usize, definition: &mut ThemeCssDefinition) {
    let rest = trimmed
        .strip_prefix("@source")
        .unwrap_or_default()
        .trim_start()
        .trim_end_matches(';')
        .trim();

    if rest == "none" {
        definition
            .source_directives
            .push(CssSourceDirective::DisableAutomaticDetection);
        return;
    }

    if let Some(raw) = rest.strip_prefix("inline") {
        let Some(argument) = parenthesized_argument(raw.trim_start()) else {
            push_diagnostic(
                definition,
                "@source inline",
                "Expected @source inline(...) with a static safelist string.",
                line,
            );
            return;
        };
        let content = unquote(argument.trim()).unwrap_or_else(|| argument.trim().to_string());
        let classes = expand_source_inline_classes(&content);
        definition
            .source_directives
            .push(CssSourceDirective::Inline(classes));
        return;
    }

    if let Some(raw) = rest.strip_prefix("not ") {
        let raw = raw.trim();
        if let Some(raw_inline) = raw.strip_prefix("inline") {
            let Some(argument) = parenthesized_argument(raw_inline.trim_start()) else {
                push_diagnostic(
                    definition,
                    "@source not inline",
                    "Expected @source not inline(...) with a static exclusion string.",
                    line,
                );
                return;
            };
            let content = unquote(argument.trim()).unwrap_or_else(|| argument.trim().to_string());
            let classes = expand_source_inline_classes(&content);
            definition
                .source_directives
                .push(CssSourceDirective::InlineExclude(classes));
        } else if let Some(specifier) = quoted_argument(raw) {
            definition
                .source_directives
                .push(CssSourceDirective::Exclude(specifier));
        } else {
            push_diagnostic(
                definition,
                "@source not",
                "Expected @source not \"./path\" with a static local path.",
                line,
            );
        }
        return;
    }

    if let Some(specifier) = quoted_argument(rest) {
        definition
            .source_directives
            .push(CssSourceDirective::Scan(specifier));
    } else {
        push_diagnostic(
            definition,
            "@source",
            "Expected @source \"./path\", @source none, @source not \"./path\", or @source inline(\"...\").",
            line,
        );
    }
}

fn parse_custom_variant_directives(source: &str, definition: &mut ThemeCssDefinition) {
    parse_custom_variant_block_directives(source, definition);
}

fn parse_custom_variant_block_directives(source: &str, definition: &mut ThemeCssDefinition) {
    let mut search_start = 0usize;
    while let Some(relative_start) = source[search_start..].find("@custom-variant") {
        let start = search_start + relative_start;
        let after_keyword = start + "@custom-variant".len();
        let line = source[..start].lines().count() + 1;

        if !source[after_keyword..]
            .chars()
            .next()
            .is_some_and(char::is_whitespace)
        {
            search_start = after_keyword;
            continue;
        }

        if !is_top_level_position(source, start) {
            push_diagnostic(
                definition,
                "@custom-variant",
                "Nested @custom-variant directives are unsupported; place DX-owned custom variants at top level.",
                line,
            );
            search_start = after_keyword;
            continue;
        }

        let Some((name, value_start)) = custom_variant_name(source, after_keyword) else {
            push_diagnostic(
                definition,
                "@custom-variant",
                "Expected @custom-variant <name> (<selector containing &>) or @custom-variant <name> { ... @slot ... }.",
                line,
            );
            search_start = after_keyword;
            continue;
        };

        if !is_safe_variant_name(name) {
            push_diagnostic(
                definition,
                "@custom-variant",
                "Only static alphanumeric, dash, and underscore custom variant names are supported.",
                line,
            );
            search_start = value_start;
            continue;
        }

        let value = &source[value_start..];
        let leading_whitespace = value.len() - value.trim_start().len();
        let value_start = value_start + leading_whitespace;
        let value = &source[value_start..];

        if value.starts_with('(') {
            let Some((consumed, selector)) = parenthesized_argument_span(value) else {
                push_diagnostic(
                    definition,
                    "@custom-variant",
                    "Expected shorthand @custom-variant <name> (<selector containing &>).",
                    line,
                );
                search_start = value_start + 1;
                continue;
            };
            let directive_end = value_start + consumed;
            if !custom_variant_shorthand_trailing_is_safe(source, directive_end)
                || !is_safe_custom_variant_selector(selector.trim())
            {
                push_diagnostic(
                    definition,
                    "@custom-variant",
                    "Only named selector wrappers containing & are supported for dx-style custom variants.",
                    line,
                );
            } else {
                definition.custom_variants.push(CssCustomVariant {
                    name: name.to_string(),
                    selector: selector.trim().to_string(),
                    media_queries: Vec::new(),
                });
            }
            search_start = directive_end;
            continue;
        }

        if value.starts_with('{') {
            let content_start = value_start + 1;
            let Some((block_end, block)) = matching_block(source, content_start) else {
                push_diagnostic(
                    definition,
                    "@custom-variant",
                    "Unclosed @custom-variant block; dx-style only supports bounded @slot blocks.",
                    line,
                );
                search_start = content_start;
                continue;
            };

            if let Some((media_queries, selector)) = parse_custom_variant_block_body(block) {
                definition.custom_variants.push(CssCustomVariant {
                    name: name.to_string(),
                    selector,
                    media_queries,
                });
            } else {
                push_diagnostic(
                    definition,
                    "@custom-variant",
                    "Unsupported @custom-variant block; use a safe selector block containing one @slot, optionally nested in @media or @supports.",
                    line,
                );
            }
            search_start = block_end + 1;
            continue;
        }

        push_diagnostic(
            definition,
            "@custom-variant",
            "Expected @custom-variant <name> (<selector containing &>) or @custom-variant <name> { ... @slot ... }.",
            line,
        );
        search_start = value_start.saturating_add(1).min(source.len());
    }
}

fn custom_variant_name(source: &str, after_keyword: usize) -> Option<(&str, usize)> {
    let rest = &source[after_keyword..];
    let leading_whitespace = rest.len() - rest.trim_start().len();
    let name_start = after_keyword + leading_whitespace;
    let rest = &source[name_start..];
    let name_end = rest
        .char_indices()
        .find_map(|(index, ch)| (ch.is_whitespace() || ch == '(' || ch == '{').then_some(index))
        .unwrap_or(rest.len());
    if name_end == 0 {
        return None;
    }
    Some((
        &source[name_start..name_start + name_end],
        name_start + name_end,
    ))
}

fn custom_variant_shorthand_trailing_is_safe(source: &str, directive_end: usize) -> bool {
    let trailing = &source[directive_end..];
    let line_end = trailing.find('\n').unwrap_or(trailing.len());
    let line = trailing[..line_end].trim();
    line.is_empty() || line == ";"
}

fn parse_custom_variant_block_body(block: &str) -> Option<(Vec<String>, String)> {
    parse_custom_variant_block_inner(block, Vec::new(), 0)
}

fn parse_custom_variant_block_inner(
    block: &str,
    mut media_queries: Vec<String>,
    depth: usize,
) -> Option<(Vec<String>, String)> {
    if depth > 4 {
        return None;
    }

    let block = block.trim();
    if block.starts_with("@media") || block.starts_with("@supports") {
        let (at_rule, inner_block) = custom_variant_at_rule_block(block)?;
        media_queries.push(at_rule);
        return parse_custom_variant_block_inner(inner_block, media_queries, depth + 1);
    }

    let selector = custom_variant_slot_selector(block)?;
    Some((media_queries, selector))
}

fn custom_variant_at_rule_block(input: &str) -> Option<(String, &str)> {
    let open = top_level_char_from(input, '{', 0)?;
    let at_rule = input[..open].trim();
    let content_start = open + 1;
    let (block_end, block) = matching_block(input, content_start)?;
    if !input[block_end + 1..].trim().is_empty() {
        return None;
    }
    Some((safe_custom_variant_at_rule(at_rule)?, block))
}

fn safe_custom_variant_at_rule(at_rule: &str) -> Option<String> {
    let body = at_rule.strip_prefix('@')?;
    let name_end = body
        .char_indices()
        .find_map(|(index, ch)| (!(ch.is_ascii_alphanumeric() || ch == '-')).then_some(index))
        .unwrap_or(body.len());
    let name = &body[..name_end];
    if !matches!(name, "media" | "supports") {
        return None;
    }

    let prelude = body[name_end..].trim();
    let lower = prelude.to_ascii_lowercase();
    if prelude.is_empty()
        || prelude.contains('{')
        || prelude.contains('}')
        || prelude.contains(';')
        || prelude.contains('\\')
        || prelude.contains("/*")
        || prelude.contains("*/")
        || lower.contains("@import")
        || lower.contains("javascript:")
        || lower.contains("expression(")
        || lower.contains("</")
        || !balanced_custom_variant_at_rule_prelude(prelude)
    {
        return None;
    }

    Some(format!("@{name} {prelude}"))
}

fn balanced_custom_variant_at_rule_prelude(prelude: &str) -> bool {
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;

    for ch in prelude.chars() {
        match ch {
            '(' => paren_depth = paren_depth.saturating_add(1),
            ')' => {
                if paren_depth == 0 {
                    return false;
                }
                paren_depth -= 1;
            }
            '[' => bracket_depth = bracket_depth.saturating_add(1),
            ']' => {
                if bracket_depth == 0 {
                    return false;
                }
                bracket_depth -= 1;
            }
            _ => {}
        }
    }

    paren_depth == 0 && bracket_depth == 0
}

fn custom_variant_slot_selector(input: &str) -> Option<String> {
    let open = top_level_char_from(input, '{', 0)?;
    let selector = input[..open].trim();
    let content_start = open + 1;
    let (block_end, block) = matching_block(input, content_start)?;
    if !input[block_end + 1..].trim().is_empty() {
        return None;
    }
    if !matches!(block.trim(), "@slot" | "@slot;") {
        return None;
    }
    if !is_safe_custom_variant_selector(selector) {
        return None;
    }
    Some(selector.to_string())
}

fn parse_utility_directives(source: &str, definition: &mut ThemeCssDefinition) {
    let mut search_start = 0usize;
    while let Some(relative_start) = source[search_start..].find("@utility") {
        let start = search_start + relative_start;
        let line = source[..start].lines().count() + 1;
        let after = start + "@utility".len();
        if !is_top_level_position(source, start) {
            push_diagnostic(
                definition,
                "@utility",
                "Layered @utility directives are unsupported; place DX-owned @utility rules at top level.",
                line,
            );
            let Some(open_offset) = source[after..].find('{') else {
                search_start = after;
                continue;
            };
            let content_start = after + open_offset + 1;
            if let Some((block_end, _)) = matching_block(source, content_start) {
                search_start = block_end + 1;
            } else {
                search_start = content_start;
            }
            continue;
        }
        let Some(open_offset) = source[after..].find('{') else {
            push_diagnostic(
                definition,
                "@utility",
                "Expected @utility <name> { ... } with flat CSS declarations.",
                line,
            );
            break;
        };
        let name = source[after..after + open_offset].trim();
        let content_start = after + open_offset + 1;
        let Some((block_end, block)) = matching_block(source, content_start) else {
            push_diagnostic(
                definition,
                "@utility",
                "Unclosed @utility block; dx-style only supports bounded declaration blocks.",
                line,
            );
            break;
        };

        if !is_safe_utility_name(name) {
            push_diagnostic(
                definition,
                "@utility",
                "Unsupported @utility name; use a static class name or one trailing * functional segment.",
                line,
            );
            search_start = block_end + 1;
            continue;
        }

        let Some(utility_block) = parse_utility_block(block) else {
            push_diagnostic(
                definition,
                "@utility",
                "Unsupported nested @utility selector; dx-style supports one-level selectors that begin with &.",
                line,
            );
            search_start = block_end + 1;
            continue;
        };

        if utility_block.declarations.is_empty() && utility_block.nested_rules.is_empty() {
            push_diagnostic(
                definition,
                "@utility",
                "Unsupported @utility block; dx-style currently accepts safe declarations and one-level nested selectors.",
                line,
            );
        } else {
            definition.utilities.push(CssUtilityDefinition {
                name: name.to_string(),
                declarations: utility_block.declarations,
                nested_rules: utility_block.nested_rules,
            });
        }

        search_start = block_end + 1;
    }
}

struct ParsedUtilityBlock {
    declarations: Vec<(String, String)>,
    nested_rules: Vec<CssUtilityNestedRule>,
}

fn parse_utility_block(block: &str) -> Option<ParsedUtilityBlock> {
    let mut flat_source = String::new();
    let mut nested_rules = Vec::new();
    let mut search_start = 0usize;
    let mut segment_start = 0usize;

    while let Some(open) = top_level_char_from(block, '{', search_start) {
        let selector_start = previous_utility_selector_boundary(block, open);
        flat_source.push_str(&block[segment_start..selector_start]);

        let selector = block[selector_start..open].trim();
        let selector_suffix = nested_utility_selector_suffix(selector)?;
        let content_start = open + 1;
        let (block_end, nested_block) = matching_block(block, content_start)?;
        if block_contains_nested_rule(nested_block) {
            return None;
        }
        let declarations = parse_utility_declarations(nested_block);
        if declarations.is_empty() {
            return None;
        }
        nested_rules.push(CssUtilityNestedRule {
            selector_suffix,
            declarations,
        });

        search_start = block_end + 1;
        segment_start = block_end + 1;
    }

    flat_source.push_str(&block[segment_start..]);

    Some(ParsedUtilityBlock {
        declarations: parse_utility_declarations(&flat_source),
        nested_rules,
    })
}

fn parse_utility_declarations(block: &str) -> Vec<(String, String)> {
    split_declarations(block)
        .into_iter()
        .filter_map(|declaration| {
            let (property, value) = declaration.split_once(':')?;
            let property = property.trim();
            let value = value.trim();
            if !is_css_property_name(property) || !is_safe_theme_value(value) {
                return None;
            }
            Some((property.to_string(), value.to_string()))
        })
        .collect()
}

fn is_top_level_position(source: &str, position: usize) -> bool {
    let mut depth = 0usize;
    let mut quote = None;
    let mut escaped = false;

    for ch in source[..position].chars() {
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

    depth == 0
}

fn block_contains_nested_rule(block: &str) -> bool {
    let mut quote = None;
    let mut escaped = false;

    for ch in block.chars() {
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
            '{' | '}' => return true,
            _ => {}
        }
    }

    false
}

fn nested_utility_selector_suffix(selector: &str) -> Option<String> {
    let selector = selector.trim();
    let suffix = selector.strip_prefix('&')?;
    if suffix.is_empty() || suffix.contains('&') || !is_safe_nested_utility_selector(selector) {
        return None;
    }
    Some(suffix.to_string())
}

fn is_safe_nested_utility_selector(selector: &str) -> bool {
    let lower = selector.to_ascii_lowercase();
    !selector.is_empty()
        && !selector.contains('{')
        && !selector.contains('}')
        && !selector.contains(';')
        && !selector.contains(',')
        && !selector.contains('@')
        && !lower.contains("javascript:")
        && !lower.contains("expression(")
        && !lower.contains("</")
}

pub(super) fn is_plain_apply_token(token: &str) -> bool {
    let token = token.strip_prefix('!').unwrap_or(token);
    let lower = token.to_ascii_lowercase();
    !token.is_empty()
        && !token.contains(':')
        && !token.starts_with('@')
        && !token.contains('{')
        && !token.contains('}')
        && !token.contains(';')
        && !token.contains('<')
        && !token.contains('>')
        && !lower.contains("javascript:")
        && !lower.contains("expression(")
        && !lower.contains("</")
}

pub(super) fn is_variant_safe_apply_token(token: &str) -> bool {
    if is_plain_apply_token(token) {
        return true;
    }

    let Some((variant, base)) = apply_token_variant_parts(token) else {
        return false;
    };

    is_plain_apply_token(base)
        && split_apply_variant_parts(variant)
            .into_iter()
            .all(is_safe_apply_variant_part)
}

fn apply_token_variant_parts(token: &str) -> Option<(&str, &str)> {
    let split_at = last_apply_variant_separator(token)?;
    let variant = token[..split_at].trim();
    let base = token[split_at + 1..].trim();
    (!variant.is_empty() && !base.is_empty()).then_some((variant, base))
}

fn split_apply_variant_parts(input: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut bracket_depth = 0usize;
    let mut paren_depth = 0usize;

    for (index, byte) in input.bytes().enumerate() {
        match byte {
            b'[' => bracket_depth = bracket_depth.saturating_add(1),
            b']' => bracket_depth = bracket_depth.saturating_sub(1),
            b'(' if bracket_depth == 0 => paren_depth = paren_depth.saturating_add(1),
            b')' if bracket_depth == 0 => paren_depth = paren_depth.saturating_sub(1),
            b':' if bracket_depth == 0 && paren_depth == 0 => {
                parts.push(input[start..index].trim());
                start = index + 1;
            }
            _ => {}
        }
    }

    parts.push(input[start..].trim());
    parts
}

fn last_apply_variant_separator(token: &str) -> Option<usize> {
    let mut bracket_depth = 0usize;
    let mut paren_depth = 0usize;
    let mut last_colon = None;

    for (index, byte) in token.bytes().enumerate() {
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

fn is_safe_apply_variant_part(part: &str) -> bool {
    let lower = part.to_ascii_lowercase();
    !part.is_empty()
        && !part.starts_with('[')
        && !part.contains('@')
        && !part.contains('{')
        && !part.contains('}')
        && !part.contains(';')
        && !part.contains('<')
        && !part.contains('>')
        && !lower.contains("javascript:")
        && !lower.contains("expression(")
        && !lower.contains("</")
}

fn previous_utility_selector_boundary(input: &str, end: usize) -> usize {
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
            ';' | '}' if paren_depth == 0 && bracket_depth == 0 => {
                boundary = index + ch.len_utf8();
            }
            _ => {}
        }
    }

    boundary
}

fn top_level_char_from(input: &str, needle: char, start: usize) -> Option<usize> {
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
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
            '(' => paren_depth = paren_depth.saturating_add(1),
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '[' => bracket_depth = bracket_depth.saturating_add(1),
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            _ if ch == needle && paren_depth == 0 && bracket_depth == 0 => return Some(index),
            _ => {}
        }
    }

    None
}

fn push_diagnostic(
    definition: &mut ThemeCssDefinition,
    directive: &str,
    reason: &str,
    line: usize,
) {
    definition.diagnostics.push(CssDirectiveDiagnostic {
        directive: directive.to_string(),
        reason: reason.to_string(),
        line,
    });
}

fn parenthesized_argument(input: &str) -> Option<&str> {
    parenthesized_argument_span(input).map(|(_, argument)| argument)
}

fn parenthesized_argument_span(input: &str) -> Option<(usize, &str)> {
    let leading_whitespace = input.len() - input.trim_start().len();
    let input = input.trim_start();
    let input = input.strip_prefix('(')?;
    let mut depth = 1usize;
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
            '(' => depth = depth.saturating_add(1),
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some((leading_whitespace + index + 2, &input[..index]));
                }
            }
            _ => {}
        }
    }
    None
}

fn quoted_argument(input: &str) -> Option<String> {
    let input = input.trim();
    let quote = input.chars().next()?;
    if !matches!(quote, '"' | '\'') {
        return None;
    }
    let rest = &input[quote.len_utf8()..];
    let end = rest.find(quote)?;
    Some(rest[..end].to_string())
}

fn unquote(input: &str) -> Option<String> {
    let quote = input.chars().next()?;
    if !matches!(quote, '"' | '\'') || !input.ends_with(quote) {
        return None;
    }
    Some(input[quote.len_utf8()..input.len() - quote.len_utf8()].to_string())
}

fn expand_source_inline_classes(value: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    for token in value.split_whitespace() {
        for expanded in expand_brace_token(token) {
            if is_safe_inline_class_token(&expanded) {
                tokens.push(expanded);
            }
        }
    }
    tokens.sort();
    tokens.dedup();
    tokens
}

fn expand_brace_token(token: &str) -> Vec<String> {
    let Some((open, close)) = first_brace_pair(token) else {
        return vec![token.to_string()];
    };
    let prefix = &token[..open];
    let suffix = &token[close + 1..];
    let body = &token[open + 1..close];
    let mut expanded = Vec::new();
    for option in split_brace_options(body) {
        for expanded_option in expand_brace_option(option) {
            let candidate = format!("{prefix}{expanded_option}{suffix}");
            expanded.extend(expand_brace_token(&candidate));
        }
    }
    expanded
}

fn expand_brace_option(option: &str) -> Vec<String> {
    expand_numeric_range(option).unwrap_or_else(|| vec![option.to_string()])
}

fn expand_numeric_range(input: &str) -> Option<Vec<String>> {
    let mut parts = input.split("..");
    let start = parts.next()?;
    let end = parts.next()?;
    let step = parts.next();
    if parts.next().is_some() {
        return None;
    }

    let start_value = start.parse::<i32>().ok()?;
    let end_value = end.parse::<i32>().ok()?;
    let step_value = match step {
        Some(value) => value.parse::<i32>().ok()?,
        None if start_value <= end_value => 1,
        None => -1,
    };
    if step_value == 0 {
        return None;
    }

    let ascending = start_value <= end_value;
    if ascending && step_value < 0 || !ascending && step_value > 0 {
        return None;
    }

    let width = numeric_range_width(start, end);
    let mut values = Vec::new();
    let mut current = start_value;
    while if ascending {
        current <= end_value
    } else {
        current >= end_value
    } {
        if values.len() >= 2048 {
            return None;
        }
        values.push(format_numeric_range_value(current, width));
        current = current.saturating_add(step_value);
    }

    Some(values)
}

fn numeric_range_width(start: &str, end: &str) -> usize {
    let start_digits = start.trim_start_matches('-');
    let end_digits = end.trim_start_matches('-');
    if start_digits.starts_with('0') || end_digits.starts_with('0') {
        start_digits.len().max(end_digits.len())
    } else {
        0
    }
}

fn format_numeric_range_value(value: i32, width: usize) -> String {
    if width == 0 {
        return value.to_string();
    }
    if value < 0 {
        format!("-{:0width$}", value.abs(), width = width)
    } else {
        format!("{value:0width$}")
    }
}

fn first_brace_pair(token: &str) -> Option<(usize, usize)> {
    let mut open = None;
    let mut depth = 0usize;
    for (index, ch) in token.char_indices() {
        match ch {
            '{' => {
                if open.is_none() {
                    open = Some(index);
                }
                depth = depth.saturating_add(1);
            }
            '}' if depth > 0 => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some((open?, index));
                }
            }
            _ => {}
        }
    }
    None
}

fn split_brace_options(body: &str) -> Vec<&str> {
    let mut options = Vec::new();
    let mut depth = 0usize;
    let mut start = 0usize;
    for (index, ch) in body.char_indices() {
        match ch {
            '{' => depth = depth.saturating_add(1),
            '}' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                options.push(&body[start..index]);
                start = index + 1;
            }
            _ => {}
        }
    }
    options.push(&body[start..]);
    options
}

fn split_declarations(block: &str) -> Vec<&str> {
    let mut declarations = Vec::new();
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
                let declaration = block[start..index].trim();
                if !declaration.is_empty() {
                    declarations.push(declaration);
                }
                start = index + 1;
            }
            _ => {}
        }
    }

    let tail = block[start..].trim();
    if !tail.is_empty() {
        declarations.push(tail);
    }

    declarations
}

fn unique_tokens(tokens: Vec<(String, String)>) -> Vec<(String, String)> {
    let mut ordered = Vec::new();
    let mut seen = AHashMap::new();

    for (name, value) in tokens {
        if let Some(index) = seen.get(&name).copied() {
            ordered[index] = (name, value);
        } else {
            seen.insert(name.clone(), ordered.len());
            ordered.push((name, value));
        }
    }

    ordered
}

fn push_unique(values: &mut Vec<String>, value: String) {
    if !values.contains(&value) {
        values.push(value);
    }
}

fn is_theme_token_name(name: &str) -> bool {
    is_standard_theme_token_name(name) || is_theme_namespace_reset_token(name)
}

fn is_standard_theme_token_name(name: &str) -> bool {
    name.starts_with("--")
        && name.len() > 2
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
}

pub(super) fn is_theme_namespace_reset_token(name: &str) -> bool {
    name == "--container-*"
}

pub(super) fn is_theme_initial_reset_value(value: &str) -> bool {
    value.trim().eq_ignore_ascii_case("initial")
}

fn is_emitted_theme_token(name: &str, value: &str) -> bool {
    !(is_theme_namespace_reset_token(name) && is_theme_initial_reset_value(value))
}

fn is_safe_variant_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
}

fn is_safe_custom_variant_selector(selector: &str) -> bool {
    let lower = selector.to_ascii_lowercase();
    selector.contains('&')
        && !selector.contains('{')
        && !selector.contains('}')
        && !selector.contains(';')
        && !lower.contains("@import")
        && !lower.contains("javascript:")
        && !lower.contains("expression(")
        && !lower.contains("</")
}

fn is_safe_utility_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    let star_count = name.chars().filter(|ch| *ch == '*').count();
    star_count <= 1
        && (star_count == 0 || name.ends_with('*'))
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '*'))
}

fn is_css_property_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
}

fn is_safe_inline_class_token(token: &str) -> bool {
    !token.is_empty()
        && !token.contains("${")
        && !token.contains('$')
        && !token.contains('{')
        && !token.contains('}')
        && !token.contains(';')
        && !token.contains('<')
        && !token.contains('>')
}

fn is_local_css_reference_specifier(specifier: &str) -> bool {
    if specifier.is_empty()
        || specifier.starts_with("http://")
        || specifier.starts_with("https://")
        || specifier.starts_with("data:")
        || specifier.starts_with("node:")
        || specifier.starts_with('@')
        || specifier.starts_with('#')
        || specifier.contains('\\')
        || specifier.contains('\0')
        || specifier.contains("..\\")
    {
        return false;
    }

    specifier.ends_with(".css")
        && specifier
            .chars()
            .all(|ch| !ch.is_control() && !matches!(ch, '"' | '\'' | '<' | '>' | '|' | ';'))
}

fn is_safe_theme_value(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    !value.is_empty()
        && !value.contains('{')
        && !value.contains('}')
        && !lower.contains("@import")
        && !lower.contains("javascript:")
        && !lower.contains("expression(")
        && !lower.contains("</")
}
