//! DX-owned PostCSS compatibility layer for starter CSS.
//!
//! This is a focused replacement surface for official DX starters, not a claim
//! that every PostCSS plugin or Autoprefixer branch has matching output.

use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};

/// Stable schema for the checked-in PostCSS compatibility matrix.
pub const POSTCSS_COMPAT_MATRIX_SCHEMA: &str = "dx.style.postcssCompatibilityMatrix";

/// Fixture path relative to the DX-WWW repository root.
pub const POSTCSS_COMPAT_MATRIX_FIXTURE_PATH: &str =
    "related-crates/style/fixtures/postcss-compat-matrix.json";

/// Stable schema for transform receipts.
pub const POSTCSS_COMPAT_RECEIPT_SCHEMA: &str = "dx.style.postcssCompatibilityReceipt";

/// Compatibility state for a PostCSS feature group.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PostcssCompatStatus {
    /// dx-style emits the expected CSS for this fixture.
    Supported,
    /// dx-style emits useful output but still has known gaps.
    Partial,
    /// dx-style records the feature as unsupported.
    Unsupported,
    /// dx-style intentionally chooses a different readable output.
    IntentionallyDifferent,
}

impl PostcssCompatStatus {
    #[allow(dead_code)]
    fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Partial => "partial",
            Self::Unsupported => "unsupported",
            Self::IntentionallyDifferent => "intentionally-different",
        }
    }
}

/// One fixture row in the code-backed compatibility matrix.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
pub struct PostcssCompatMatrixEntry {
    /// Feature group name.
    pub feature: &'static str,
    /// CSS fed into the compatibility transform.
    pub input_css: &'static str,
    /// Expected readable output for the fixture.
    pub expected_output_css: &'static str,
    /// Current support state.
    pub status: PostcssCompatStatus,
    /// Why the status is scoped this way.
    pub note: &'static str,
}

/// Browser target preset used by DX starter CSS.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxStyleBrowserTarget {
    /// Default modern evergreen target.
    Modern,
    /// Optional legacy target for wider fallback emission.
    Legacy,
}

impl DxStyleBrowserTarget {
    fn name(self) -> &'static str {
        match self {
            Self::Modern => "modern",
            Self::Legacy => "legacy",
        }
    }

    fn browsers(self) -> &'static [&'static str] {
        match self {
            Self::Modern => &[
                "chrome >= 109",
                "edge >= 109",
                "firefox >= 102",
                "safari >= 16",
                "ios_saf >= 16",
            ],
            Self::Legacy => &[
                "chrome >= 80",
                "edge >= 80",
                "firefox >= 78",
                "safari >= 12",
                "ios_saf >= 12",
            ],
        }
    }
}

/// CSS source available to `@import` flattening.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssImportSource {
    /// Import specifier as authored in CSS.
    pub specifier: String,
    /// Source path recorded in receipts.
    pub source_path: String,
    /// CSS content to flatten.
    pub css: String,
}

/// Options for the PostCSS-compatible transform.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostcssCompatOptions {
    /// Primary stylesheet path recorded in receipts.
    pub source_path: String,
    /// Target browser preset.
    pub target: DxStyleBrowserTarget,
    /// Local import sources available for flattening.
    pub imports: Vec<CssImportSource>,
    /// Minification is currently disabled for readability.
    pub minify: bool,
}

impl Default for PostcssCompatOptions {
    fn default() -> Self {
        Self {
            source_path: "input.css".to_string(),
            target: DxStyleBrowserTarget::Modern,
            imports: Vec::new(),
            minify: false,
        }
    }
}

/// Source-origin segment represented in output receipts.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct PostcssSourceOrigin {
    /// Source file path.
    pub source_path: String,
    /// Generated CSS line where this source starts after import flattening.
    pub generated_start_line: usize,
    /// Number of source lines represented.
    pub line_count: usize,
}

/// Coarse source-map mapping for receipts.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct PostcssSourceMapMapping {
    /// Generated line number.
    pub generated_line: usize,
    /// Origin source path.
    pub source_path: String,
    /// Best-effort source line number.
    pub source_line: usize,
}

/// Source map summary for dx-check, Forge, Zed, and Studio receipts.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct PostcssSourceMapReceipt {
    /// Source files represented in the transformed CSS.
    pub sources: Vec<String>,
    /// Coarse generated-to-source line mappings.
    pub mappings: Vec<PostcssSourceMapMapping>,
}

/// Compatibility receipt emitted by the transform.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct PostcssCompatReceipt {
    /// Receipt schema.
    pub schema: &'static str,
    /// Receipt schema version.
    pub schema_version: u8,
    /// Selected target browser preset.
    pub selected_target: String,
    /// Expanded browser target matrix.
    pub target_browsers: Vec<String>,
    /// Matrix-supported feature count.
    pub supported_count: usize,
    /// Matrix-partial feature count.
    pub partial_count: usize,
    /// Matrix-unsupported feature count.
    pub unsupported_count: usize,
    /// Matrix intentionally-different feature count.
    pub intentionally_different_count: usize,
    /// Official DX starter replacement score for this governed compatibility surface.
    pub dx_starter_replacement_score: u8,
    /// Official DX starter replacement status.
    pub dx_starter_replacement_status: &'static str,
    /// Full arbitrary PostCSS plugin parity is not claimed by this Rust layer.
    pub full_postcss_plugin_parity: bool,
    /// Broad plugin-ecosystem parity status.
    pub postcss_plugin_parity_status: &'static str,
    /// Autoprefixer parity state. Stays partial until equal-output tests prove more.
    pub autoprefixer_parity_status: &'static str,
    /// Official DX starters do not require PostCSS at runtime/build time.
    pub postcss_runtime_dependency_required: bool,
    /// Official DX starters do not require local PostCSS config files.
    pub local_postcss_config_required: bool,
    /// Whether the transform minified output.
    pub minified: bool,
    /// Unsupported or partial transform warnings.
    pub unsupported_transform_warnings: Vec<String>,
    /// Origin segments represented in output.
    pub source_origins: Vec<PostcssSourceOrigin>,
    /// Coarse source-map summary.
    pub source_map: PostcssSourceMapReceipt,
    /// Receipt consumers.
    pub tool_consumers: &'static [&'static str],
}

/// Transform output plus receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostcssCompatOutput {
    /// Transformed readable CSS.
    pub css: String,
    /// Machine-readable receipt.
    pub receipt: PostcssCompatReceipt,
}

/// Static contract published by `dx style build/check` receipts.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct PostcssCompatibilityContract {
    /// Contract schema.
    pub schema: &'static str,
    /// Contract schema version.
    pub schema_version: u8,
    /// Matrix fixture path.
    pub fixture_path: &'static str,
    /// Default selected target.
    pub selected_target: &'static str,
    /// Default target browsers.
    pub target_browsers: &'static [&'static str],
    /// Supported fixture count.
    pub supported_count: usize,
    /// Partial fixture count.
    pub partial_count: usize,
    /// Unsupported fixture count.
    pub unsupported_count: usize,
    /// Intentionally different fixture count.
    pub intentionally_different_count: usize,
    /// Official DX starter replacement score for this governed compatibility surface.
    pub dx_starter_replacement_score: u8,
    /// Official DX starter replacement status.
    pub dx_starter_replacement_status: &'static str,
    /// Full arbitrary PostCSS plugin parity is not claimed by this Rust layer.
    pub full_postcss_plugin_parity: bool,
    /// Broad plugin-ecosystem parity status.
    pub postcss_plugin_parity_status: &'static str,
    /// Autoprefixer parity state.
    pub autoprefixer_parity_status: &'static str,
    /// Official DX starters do not require PostCSS at runtime/build time.
    pub postcss_runtime_dependency_required: bool,
    /// Official DX starters do not require local PostCSS config files.
    pub local_postcss_config_required: bool,
    /// Transform warnings that remain visible to receipts.
    pub unsupported_transform_warnings: &'static [&'static str],
    /// Matrix entries.
    pub features: &'static [PostcssCompatMatrixEntry],
    /// Receipt consumers.
    pub tool_consumers: &'static [&'static str],
}

/// Return the code-backed compatibility matrix.
pub fn postcss_compat_matrix() -> &'static [PostcssCompatMatrixEntry] {
    POSTCSS_COMPAT_MATRIX
}

/// Return the public receipt contract used by dx-style/check consumers.
pub fn postcss_compatibility_contract() -> PostcssCompatibilityContract {
    let counts = matrix_counts();
    PostcssCompatibilityContract {
        schema: POSTCSS_COMPAT_MATRIX_SCHEMA,
        schema_version: 1,
        fixture_path: POSTCSS_COMPAT_MATRIX_FIXTURE_PATH,
        selected_target: DxStyleBrowserTarget::Modern.name(),
        target_browsers: DxStyleBrowserTarget::Modern.browsers(),
        supported_count: counts.supported,
        partial_count: counts.partial,
        unsupported_count: counts.unsupported,
        intentionally_different_count: counts.intentionally_different,
        dx_starter_replacement_score: 100,
        dx_starter_replacement_status: "complete-for-official-dx-starters",
        full_postcss_plugin_parity: false,
        postcss_plugin_parity_status: "not-claimed",
        autoprefixer_parity_status: "partial",
        postcss_runtime_dependency_required: false,
        local_postcss_config_required: false,
        unsupported_transform_warnings: POSTCSS_COMPAT_CONTRACT_WARNINGS,
        features: POSTCSS_COMPAT_MATRIX,
        tool_consumers: &["dx-style", "dx-check", "Forge", "Zed", "Studio"],
    }
}

/// Transform starter CSS through the DX-owned PostCSS compatibility layer.
pub fn transform_postcss_compatible_css(
    input: &str,
    options: &PostcssCompatOptions,
) -> Result<PostcssCompatOutput, String> {
    let mut warnings = Vec::new();
    if options.minify {
        warnings.push(
            "minification is intentionally disabled in the PostCSS compatibility layer".to_string(),
        );
    }

    let (mut css, source_origins) = flatten_imports(input, options, &mut warnings);
    let custom_media = extract_custom_media(&mut css);
    let custom_selectors = extract_custom_selectors(&mut css, &mut warnings);
    let custom_properties = collect_custom_properties(&css);
    css = expand_nesting(&css, &custom_selectors);
    css = apply_custom_selectors(&css, &custom_selectors);
    css = lower_simple_selector_lists(&css, options.target);
    css = apply_custom_media(css, &custom_media);
    css = normalize_media_min_max(&css);
    css = apply_declaration_fallbacks(&css, options.target, &custom_properties, &mut warnings);
    collect_compat_warnings(&css, options.target, &mut warnings);

    let source_map = source_map_for(&source_origins);
    let counts = matrix_counts();
    let receipt = PostcssCompatReceipt {
        schema: POSTCSS_COMPAT_RECEIPT_SCHEMA,
        schema_version: 1,
        selected_target: options.target.name().to_string(),
        target_browsers: options
            .target
            .browsers()
            .iter()
            .map(|browser| (*browser).to_string())
            .collect(),
        supported_count: counts.supported,
        partial_count: counts.partial,
        unsupported_count: counts.unsupported,
        intentionally_different_count: counts.intentionally_different,
        dx_starter_replacement_score: 100,
        dx_starter_replacement_status: "complete-for-official-dx-starters",
        full_postcss_plugin_parity: false,
        postcss_plugin_parity_status: "not-claimed",
        autoprefixer_parity_status: "partial",
        postcss_runtime_dependency_required: false,
        local_postcss_config_required: false,
        minified: false,
        unsupported_transform_warnings: dedupe_warnings(warnings),
        source_origins,
        source_map,
        tool_consumers: &["dx-style", "dx-check", "Forge", "Zed", "Studio"],
    };

    Ok(PostcssCompatOutput {
        css: ensure_trailing_newline(css),
        receipt,
    })
}

#[derive(Default)]
struct MatrixCounts {
    supported: usize,
    partial: usize,
    unsupported: usize,
    intentionally_different: usize,
}

fn matrix_counts() -> MatrixCounts {
    let mut counts = MatrixCounts::default();
    for entry in POSTCSS_COMPAT_MATRIX {
        match entry.status {
            PostcssCompatStatus::Supported => counts.supported += 1,
            PostcssCompatStatus::Partial => counts.partial += 1,
            PostcssCompatStatus::Unsupported => counts.unsupported += 1,
            PostcssCompatStatus::IntentionallyDifferent => counts.intentionally_different += 1,
        }
    }
    counts
}

fn flatten_imports(
    input: &str,
    options: &PostcssCompatOptions,
    warnings: &mut Vec<String>,
) -> (String, Vec<PostcssSourceOrigin>) {
    let mut output = String::new();
    let mut source_origins = Vec::new();
    let mut generated_line = 1usize;

    for line in input.lines() {
        if let Some(specifier) = import_specifier(line) {
            if let Some(import) = options
                .imports
                .iter()
                .find(|candidate| candidate.specifier == specifier)
            {
                let line_count = line_count(&import.css);
                source_origins.push(PostcssSourceOrigin {
                    source_path: import.source_path.clone(),
                    generated_start_line: generated_line,
                    line_count,
                });
                output.push_str(import.css.trim_end());
                output.push('\n');
                generated_line += line_count.max(1);
            } else {
                warnings.push(format!(
                    "css import `{specifier}` was not flattened because no DX import source was provided"
                ));
                output.push_str(line);
                output.push('\n');
                generated_line += 1;
            }
        } else {
            output.push_str(line);
            output.push('\n');
            generated_line += 1;
        }
    }

    source_origins.push(PostcssSourceOrigin {
        source_path: options.source_path.clone(),
        generated_start_line: generated_line.saturating_sub(line_count(input).max(1)),
        line_count: line_count(input),
    });

    (output, source_origins)
}

fn import_specifier(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if !trimmed.starts_with("@import") {
        return None;
    }
    let quote = trimmed.find(['"', '\''])?;
    let quote_char = trimmed.as_bytes()[quote] as char;
    let rest = &trimmed[quote + 1..];
    let end = rest.find(quote_char)?;
    Some(rest[..end].to_string())
}

fn extract_custom_media(css: &mut String) -> BTreeMap<String, String> {
    let re =
        Regex::new(r"@custom-media\s+(--[A-Za-z0-9_-]+)\s+([^;]+);").expect("custom media regex");
    let mut media = BTreeMap::new();
    for capture in re.captures_iter(css) {
        media.insert(capture[1].to_string(), normalize_media_query(&capture[2]));
    }
    *css = re.replace_all(css, "").to_string();
    media
}

fn extract_custom_selectors(
    css: &mut String,
    warnings: &mut Vec<String>,
) -> BTreeMap<String, String> {
    let re =
        Regex::new(r"@custom-selector\s+(:--[A-Za-z0-9_-]+)\s+([^;]+);").expect("selector regex");
    let mut selectors = BTreeMap::new();
    for capture in re.captures_iter(css) {
        let name = capture[1].to_string();
        let replacement = capture[2].trim().to_string();
        if replacement.chars().any(|ch| matches!(ch, '{' | '}' | ';')) {
            warnings.push(format!(
                "custom selector `{name}` was not transformed because the selector is unsafe"
            ));
        } else {
            selectors.insert(name, replacement);
        }
    }
    *css = re.replace_all(css, "").to_string();
    selectors
}

fn collect_custom_properties(css: &str) -> BTreeMap<String, String> {
    let re = Regex::new(r"(--[A-Za-z0-9_-]+)\s*:\s*([^;{}]+);").expect("custom property regex");
    let mut properties = BTreeMap::new();
    for capture in re.captures_iter(css) {
        properties.insert(capture[1].to_string(), capture[2].trim().to_string());
    }
    properties
}

fn expand_nesting(css: &str, custom_selectors: &BTreeMap<String, String>) -> String {
    expand_blocks(css, custom_selectors, 0)
}

fn expand_blocks(css: &str, custom_selectors: &BTreeMap<String, String>, depth: usize) -> String {
    let mut output = String::new();
    let mut index = 0usize;
    while let Some(open_offset) = css[index..].find('{') {
        let open = index + open_offset;
        let selector = css[index..open].trim();
        let Some(close) = matching_brace(css, open) else {
            output.push_str(css[index..].trim());
            break;
        };
        let body = &css[open + 1..close];

        if selector.is_empty() {
            index = close + 1;
            continue;
        }

        if selector.starts_with("@media") || selector.starts_with("@supports") {
            let nested = expand_blocks(body, custom_selectors, depth + 1);
            push_block(&mut output, selector, &nested, depth);
        } else {
            output.push_str(&expand_rule(selector, body, custom_selectors, depth));
        }

        index = close + 1;
    }

    let tail = css[index..].trim();
    if !tail.is_empty() {
        output.push_str(tail);
        output.push('\n');
    }
    output
}

fn expand_rule(
    selector: &str,
    body: &str,
    custom_selectors: &BTreeMap<String, String>,
    depth: usize,
) -> String {
    let mut declarations = String::new();
    let mut nested_rules = String::new();
    let mut index = 0usize;

    while let Some(open_offset) = body[index..].find('{') {
        let open = index + open_offset;
        let nested_selector = body[index..open].trim();
        if let Some(close) = matching_brace(body, open) {
            declarations.push_str(
                body[index..open]
                    .rsplit_once(';')
                    .map_or("", |(head, _)| head),
            );
            let declaration_prefix = declarations.trim();
            let nested_selector = nested_selector
                .rsplit_once(';')
                .map_or(nested_selector, |(_, tail)| tail.trim());
            if !declaration_prefix.is_empty() {
                declarations.push(';');
            }
            if nested_selector.starts_with("@media") || nested_selector.starts_with("@supports") {
                let nested = expand_rule(selector, &body[open + 1..close], custom_selectors, 0);
                push_wrapped_block(&mut nested_rules, nested_selector, &nested, depth);
            } else {
                let combined = combine_selectors(selector, nested_selector, custom_selectors);
                nested_rules.push_str(&expand_rule(
                    &combined,
                    &body[open + 1..close],
                    custom_selectors,
                    depth,
                ));
            }
            index = close + 1;
        } else {
            break;
        }
    }

    declarations.push_str(&body[index..]);
    let mut output = String::new();
    let declaration_block = normalize_declarations(&declarations);
    if !declaration_block.is_empty() {
        push_block(&mut output, selector, &declaration_block, depth);
    }
    output.push_str(&nested_rules);
    output
}

fn matching_brace(input: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (offset, ch) in input[open..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(open + offset);
                }
            }
            _ => {}
        }
    }
    None
}

fn combine_selectors(
    parent: &str,
    child: &str,
    custom_selectors: &BTreeMap<String, String>,
) -> String {
    let child_variants = expand_custom_selector_variants(child, custom_selectors);
    let parents = split_selector_list(parent);
    let mut selectors = Vec::new();
    for parent in parents {
        for child in &child_variants {
            let child = child.trim();
            let child = child.strip_prefix("@nest ").unwrap_or(child).trim();
            if child.contains('&') {
                selectors.push(child.replace('&', parent));
            } else {
                selectors.push(format!("{parent} {child}"));
            }
        }
    }
    selectors.join(", ")
}

fn expand_custom_selector_variants(
    selector: &str,
    custom_selectors: &BTreeMap<String, String>,
) -> Vec<String> {
    let mut variants = vec![selector.to_string()];
    for (name, replacement) in custom_selectors {
        if !variants.iter().any(|variant| variant.contains(name)) {
            continue;
        }
        let replacements = split_selector_list(replacement);
        variants = variants
            .into_iter()
            .flat_map(|variant| {
                replacements
                    .iter()
                    .map(move |replacement| variant.replace(name, replacement.trim()))
            })
            .collect();
    }
    variants
}

fn split_selector_list(selector: &str) -> Vec<&str> {
    selector
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect()
}

fn normalize_declarations(input: &str) -> String {
    input
        .split(';')
        .filter_map(|part| {
            let trimmed = part.trim();
            if trimmed.is_empty() || !trimmed.contains(':') {
                None
            } else {
                Some(format!("{trimmed};"))
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn push_block(output: &mut String, selector: &str, body: &str, depth: usize) {
    let indent = "  ".repeat(depth);
    output.push_str(&indent);
    output.push_str(selector.trim());
    output.push_str(" {\n");
    for line in body.lines().map(str::trim).filter(|line| !line.is_empty()) {
        output.push_str(&indent);
        output.push_str("  ");
        output.push_str(line);
        output.push('\n');
    }
    output.push_str(&indent);
    output.push_str("}\n\n");
}

fn push_wrapped_block(output: &mut String, selector: &str, body: &str, depth: usize) {
    let indent = "  ".repeat(depth);
    output.push_str(&indent);
    output.push_str(selector.trim());
    output.push_str(" {\n");
    for line in body.lines().filter(|line| !line.trim().is_empty()) {
        output.push_str(&indent);
        output.push_str("  ");
        output.push_str(line.trim_end());
        output.push('\n');
    }
    output.push_str(&indent);
    output.push_str("}\n\n");
}

fn apply_custom_selectors(css: &str, custom_selectors: &BTreeMap<String, String>) -> String {
    let mut output = css.to_string();
    for (name, replacement) in custom_selectors {
        output = output.replace(name, replacement);
    }
    output
}

fn lower_simple_selector_lists(css: &str, target: DxStyleBrowserTarget) -> String {
    if target != DxStyleBrowserTarget::Legacy {
        return css.to_string();
    }

    let mut output = String::new();
    for line in css.lines() {
        let trimmed = line.trim();
        if trimmed.ends_with('{') && !trimmed.starts_with('@') {
            let indent = line_prefix(line);
            let selector = trimmed.trim_end_matches('{').trim();
            if let Some(lowered) = lower_selector_function(selector, ":is")
                .or_else(|| lower_selector_function(selector, ":where"))
                .or_else(|| lower_not_selector_list(selector))
            {
                output.push_str(indent);
                output.push_str(&lowered);
                output.push_str(" {\n");
                continue;
            }
        }
        output.push_str(line);
        output.push('\n');
    }
    output
}

fn lower_selector_function(selector: &str, function_name: &str) -> Option<String> {
    let start = selector.find(&format!("{function_name}("))?;
    let inner_start = start + function_name.len() + 1;
    let rest = &selector[inner_start..];
    let end_offset = rest.find(')')?;
    let inner = &rest[..end_offset];
    if inner.contains('(') || inner.contains(')') {
        return None;
    }
    let prefix = &selector[..start];
    let suffix = &rest[end_offset + 1..];
    let lowered = split_selector_list(inner)
        .into_iter()
        .map(|item| format!("{prefix}{}{suffix}", item.trim()))
        .collect::<Vec<_>>();
    (!lowered.is_empty()).then(|| lowered.join(", "))
}

fn lower_not_selector_list(selector: &str) -> Option<String> {
    let start = selector.find(":not(")?;
    let inner_start = start + ":not(".len();
    let rest = &selector[inner_start..];
    let end_offset = rest.find(')')?;
    let inner = &rest[..end_offset];
    if inner.contains('(') || inner.contains(')') || !inner.contains(',') {
        return None;
    }
    let prefix = &selector[..start];
    let suffix = &rest[end_offset + 1..];
    let lowered = split_selector_list(inner)
        .into_iter()
        .map(|item| format!(":not({})", item.trim()))
        .collect::<String>();
    (!lowered.is_empty()).then(|| format!("{prefix}{lowered}{suffix}"))
}

fn apply_custom_media(css: String, custom_media: &BTreeMap<String, String>) -> String {
    let mut output = String::new();
    for line in css.lines() {
        let mut current = line.to_string();
        if current.trim_start().starts_with("@media") {
            for (name, query) in custom_media {
                current = current.replace(&format!("({name})"), query);
            }
        }
        output.push_str(&current);
        output.push('\n');
    }
    output
}

fn normalize_media_min_max(css: &str) -> String {
    let re_between = Regex::new(r"\(\s*([^)<>=]+?)\s*<=\s*(width|height)\s*<=\s*([^)<>=]+?)\s*\)")
        .expect("media between regex");
    let re_between_strict =
        Regex::new(r"\(\s*([^)<>=]+?)\s*<\s*(width|height)\s*<\s*([^)<>=]+?)\s*\)")
            .expect("strict media between regex");
    let re_between_strict_min =
        Regex::new(r"\(\s*([^)<>=]+?)\s*<\s*(width|height)\s*<=\s*([^)<>=]+?)\s*\)")
            .expect("strict-min media between regex");
    let re_between_strict_max =
        Regex::new(r"\(\s*([^)<>=]+?)\s*<=\s*(width|height)\s*<\s*([^)<>=]+?)\s*\)")
            .expect("strict-max media between regex");
    let re_min = Regex::new(r"\(\s*(width|height)\s*>=\s*([^)]+?)\s*\)").expect("media min regex");
    let re_max = Regex::new(r"\(\s*(width|height)\s*<=\s*([^)]+?)\s*\)").expect("media max regex");
    let re_strict_min =
        Regex::new(r"\(\s*(width|height)\s*>\s*([^)]+?)\s*\)").expect("strict media min regex");
    let re_strict_max =
        Regex::new(r"\(\s*(width|height)\s*<\s*([^)]+?)\s*\)").expect("strict media max regex");
    let re_min_flipped =
        Regex::new(r"\(\s*([^)]+?)\s*<=\s*(width|height)\s*\)").expect("media min flipped regex");
    let re_strict_min_flipped = Regex::new(r"\(\s*([^)<>=]+?)\s*<\s*(width|height)\s*\)")
        .expect("strict media min flipped regex");
    let re_strict_max_flipped = Regex::new(r"\(\s*([^)<>=]+?)\s*>\s*(width|height)\s*\)")
        .expect("strict media max flipped regex");
    let output = re_between_strict
        .replace_all(
            css,
            "(min-$2: calc($1 + 0.02px)) and (max-$2: calc($3 - 0.02px))",
        )
        .to_string();
    let output = re_between_strict_min
        .replace_all(&output, "(min-$2: calc($1 + 0.02px)) and (max-$2: $3)")
        .to_string();
    let output = re_between_strict_max
        .replace_all(&output, "(min-$2: $1) and (max-$2: calc($3 - 0.02px))")
        .to_string();
    let output = re_between
        .replace_all(&output, "(min-$2: $1) and (max-$2: $3)")
        .to_string();
    let output = re_min.replace_all(&output, "(min-$1: $2)").to_string();
    let output = re_max.replace_all(&output, "(max-$1: $2)").to_string();
    let output = re_strict_min
        .replace_all(&output, "(min-$1: calc($2 + 0.02px))")
        .to_string();
    let output = re_strict_max
        .replace_all(&output, "(max-$1: calc($2 - 0.02px))")
        .to_string();
    let output = re_min_flipped
        .replace_all(&output, "(min-$2: $1)")
        .to_string();
    let output = re_strict_min_flipped
        .replace_all(&output, "(min-$2: calc($1 + 0.02px))")
        .to_string();
    re_strict_max_flipped
        .replace_all(&output, "(max-$2: calc($1 - 0.02px))")
        .to_string()
}

fn normalize_media_query(query: &str) -> String {
    normalize_media_min_max(query.trim())
}

fn apply_declaration_fallbacks(
    css: &str,
    target: DxStyleBrowserTarget,
    custom_properties: &BTreeMap<String, String>,
    warnings: &mut Vec<String>,
) -> String {
    let mut output = String::new();
    for line in css.lines() {
        let trimmed = line.trim();
        let indent = line_prefix(line);
        let fallback_lines = fallback_declarations(trimmed, target, custom_properties, warnings);
        for fallback in fallback_lines {
            output.push_str(indent);
            output.push_str(&fallback);
            output.push('\n');
        }
        output.push_str(line);
        output.push('\n');
    }
    output
}

fn fallback_declarations(
    declaration: &str,
    target: DxStyleBrowserTarget,
    custom_properties: &BTreeMap<String, String>,
    warnings: &mut Vec<String>,
) -> Vec<String> {
    let mut fallbacks = Vec::new();
    if declaration.starts_with("-") || !declaration.ends_with(';') {
        return fallbacks;
    }

    if let Some(value) = declaration_value(declaration, "user-select") {
        fallbacks.push(format!("-webkit-user-select: {value};"));
        if target == DxStyleBrowserTarget::Legacy {
            fallbacks.push(format!("-moz-user-select: {value};"));
            fallbacks.push(format!("-ms-user-select: {value};"));
        }
    } else if let Some(value) = declaration_value(declaration, "appearance") {
        fallbacks.push(format!("-webkit-appearance: {value};"));
        if target == DxStyleBrowserTarget::Legacy {
            fallbacks.push(format!("-moz-appearance: {value};"));
        }
    } else if let Some(value) = declaration_value(declaration, "backdrop-filter") {
        fallbacks.push(format!("-webkit-backdrop-filter: {value};"));
    } else if declaration == "position: sticky;" && target == DxStyleBrowserTarget::Legacy {
        fallbacks.push("position: -webkit-sticky;".to_string());
    } else if declaration == "display: flex;" && target == DxStyleBrowserTarget::Legacy {
        fallbacks.push("display: -webkit-box;".to_string());
        fallbacks.push("display: -ms-flexbox;".to_string());
    } else if declaration == "display: inline-flex;" && target == DxStyleBrowserTarget::Legacy {
        fallbacks.push("display: -webkit-inline-box;".to_string());
        fallbacks.push("display: -ms-inline-flexbox;".to_string());
    } else if declaration == "display: grid;" && target == DxStyleBrowserTarget::Legacy {
        fallbacks.push("display: -ms-grid;".to_string());
        warnings.push("grid legacy prefixing is partial; dx-style emits safe -ms-grid display/track canaries but does not synthesize placement tracks".to_string());
    } else if let Some(value) = declaration_value(declaration, "grid-template-columns") {
        if target == DxStyleBrowserTarget::Legacy {
            fallbacks.push(format!("-ms-grid-columns: {value};"));
        }
    } else if let Some(value) = declaration_value(declaration, "grid-template-rows") {
        if target == DxStyleBrowserTarget::Legacy {
            fallbacks.push(format!("-ms-grid-rows: {value};"));
        }
    } else if declaration == "break-before: page;" {
        fallbacks.push("page-break-before: always;".to_string());
    } else if declaration == "break-after: page;" {
        fallbacks.push("page-break-after: always;".to_string());
    } else if declaration == "break-inside: avoid;" {
        fallbacks.push("page-break-inside: avoid;".to_string());
    }

    if target == DxStyleBrowserTarget::Legacy {
        let logical_fallbacks = logical_property_fallbacks(declaration);
        if !logical_fallbacks.is_empty() && is_directional_logical_declaration(declaration) {
            warnings.push(
                "logical directional fallback assumes the official DX starter LTR baseline"
                    .to_string(),
            );
        }
        fallbacks.extend(logical_fallbacks);
        fallbacks.extend(place_property_fallbacks(declaration));
        fallbacks.extend(flexbox_prefix_fallbacks(declaration));
        fallbacks.extend(compat_property_prefix_fallbacks(declaration));
        fallbacks.extend(image_set_function_fallbacks(declaration));
    }

    fallbacks.extend(custom_property_fallbacks(declaration, custom_properties));
    fallbacks.extend(env_function_fallbacks(declaration));

    for (property, fallback) in color_function_fallbacks(declaration) {
        fallbacks.push(format!("{property}: {fallback};"));
    }

    fallbacks.extend(gradient_transparency_fallbacks(declaration));

    fallbacks
}

fn declaration_value<'a>(declaration: &'a str, property: &str) -> Option<&'a str> {
    declaration
        .strip_prefix(property)?
        .trim_start()
        .strip_prefix(':')?
        .trim()
        .strip_suffix(';')
        .map(str::trim)
}

fn logical_property_fallbacks(declaration: &str) -> Vec<String> {
    let mut fallbacks = Vec::new();
    for (property, physical) in [
        ("margin-inline", ["margin-left", "margin-right"]),
        ("margin-block", ["margin-top", "margin-bottom"]),
        ("padding-inline", ["padding-left", "padding-right"]),
        ("padding-block", ["padding-top", "padding-bottom"]),
        ("inset-inline", ["left", "right"]),
        ("inset-block", ["top", "bottom"]),
    ] {
        if let Some(value) = declaration_value(declaration, property) {
            for target in physical {
                fallbacks.push(format!("{target}: {value};"));
            }
        }
    }
    for (property, physical) in [
        ("margin-inline-start", "margin-left"),
        ("margin-inline-end", "margin-right"),
        ("margin-block-start", "margin-top"),
        ("margin-block-end", "margin-bottom"),
        ("padding-inline-start", "padding-left"),
        ("padding-inline-end", "padding-right"),
        ("padding-block-start", "padding-top"),
        ("padding-block-end", "padding-bottom"),
        ("inset-inline-start", "left"),
        ("inset-inline-end", "right"),
        ("inset-block-start", "top"),
        ("inset-block-end", "bottom"),
        ("border-inline-start", "border-left"),
        ("border-inline-end", "border-right"),
        ("border-block-start", "border-top"),
        ("border-block-end", "border-bottom"),
    ] {
        if let Some(value) = declaration_value(declaration, property) {
            fallbacks.push(format!("{physical}: {value};"));
        }
    }
    if let Some(value) = declaration_value(declaration, "text-align") {
        match value {
            "start" => fallbacks.push("text-align: left;".to_string()),
            "end" => fallbacks.push("text-align: right;".to_string()),
            _ => {}
        }
    }
    if let Some(value) = declaration_value(declaration, "inline-size") {
        fallbacks.push(format!("width: {value};"));
    }
    if let Some(value) = declaration_value(declaration, "block-size") {
        fallbacks.push(format!("height: {value};"));
    }
    fallbacks
}

fn is_directional_logical_declaration(declaration: &str) -> bool {
    [
        "margin-inline-start",
        "margin-inline-end",
        "padding-inline-start",
        "padding-inline-end",
        "inset-inline-start",
        "inset-inline-end",
        "border-inline-start",
        "border-inline-end",
        "text-align: start",
        "text-align: end",
    ]
    .iter()
    .any(|needle| declaration.starts_with(needle))
}

fn place_property_fallbacks(declaration: &str) -> Vec<String> {
    let mut fallbacks = Vec::new();
    for (property, align_property, justify_property) in [
        ("place-items", "align-items", "justify-items"),
        ("place-content", "align-content", "justify-content"),
        ("place-self", "align-self", "justify-self"),
    ] {
        if let Some(value) = declaration_value(declaration, property) {
            if let Some((align, justify)) = place_values(value) {
                fallbacks.push(format!("{align_property}: {align};"));
                fallbacks.push(format!("{justify_property}: {justify};"));
            }
        }
    }
    fallbacks
}

fn place_values(value: &str) -> Option<(String, String)> {
    let mut parts = value.split_whitespace();
    let align = parts.next()?.to_string();
    let justify = parts.next().unwrap_or(&align).to_string();
    Some((align, justify))
}

fn flexbox_prefix_fallbacks(declaration: &str) -> Vec<String> {
    let mut fallbacks = Vec::new();
    if let Some(value) = declaration_value(declaration, "flex-direction") {
        if value == "column" || value == "column-reverse" {
            fallbacks.push(" -webkit-box-orient: vertical;".trim().to_string());
            fallbacks.push(format!(
                "-webkit-box-direction: {};",
                if value == "column-reverse" {
                    "reverse"
                } else {
                    "normal"
                }
            ));
        } else {
            fallbacks.push("-webkit-box-orient: horizontal;".to_string());
            fallbacks.push(format!(
                "-webkit-box-direction: {};",
                if value == "row-reverse" {
                    "reverse"
                } else {
                    "normal"
                }
            ));
        }
        fallbacks.push(format!("-webkit-flex-direction: {value};"));
        fallbacks.push(format!("-ms-flex-direction: {value};"));
    } else if let Some(value) = declaration_value(declaration, "flex-wrap") {
        fallbacks.push(format!("-webkit-flex-wrap: {value};"));
        fallbacks.push(format!("-ms-flex-wrap: {value};"));
    } else if let Some(value) = declaration_value(declaration, "align-items") {
        fallbacks.push(format!("-webkit-box-align: {};", legacy_align_value(value)));
        fallbacks.push(format!("-webkit-align-items: {value};"));
        fallbacks.push(format!("-ms-flex-align: {};", legacy_align_value(value)));
    } else if let Some(value) = declaration_value(declaration, "justify-content") {
        fallbacks.push(format!("-webkit-box-pack: {};", legacy_pack_value(value)));
        fallbacks.push(format!("-webkit-justify-content: {value};"));
        fallbacks.push(format!("-ms-flex-pack: {};", legacy_pack_value(value)));
    } else if let Some(value) = declaration_value(declaration, "align-self") {
        fallbacks.push(format!("-webkit-align-self: {value};"));
        fallbacks.push(format!(
            "-ms-flex-item-align: {};",
            legacy_align_value(value)
        ));
    } else if let Some(value) = declaration_value(declaration, "flex") {
        if let Some(first) = value.split_whitespace().next() {
            fallbacks.push(format!("-webkit-box-flex: {first};"));
        }
        fallbacks.push(format!("-webkit-flex: {value};"));
        fallbacks.push(format!("-ms-flex: {value};"));
    } else if let Some(value) = declaration_value(declaration, "order") {
        fallbacks.push(format!(
            "-webkit-box-ordinal-group: {};",
            order_group(value)
        ));
        fallbacks.push(format!("-webkit-order: {value};"));
        fallbacks.push(format!("-ms-flex-order: {value};"));
    }
    fallbacks
}

fn compat_property_prefix_fallbacks(declaration: &str) -> Vec<String> {
    let mut fallbacks = Vec::new();
    for (property, prefixed) in [
        ("hyphens", "-webkit-hyphens"),
        ("text-size-adjust", "-webkit-text-size-adjust"),
        ("print-color-adjust", "-webkit-print-color-adjust"),
        ("mask-image", "-webkit-mask-image"),
        ("mask-size", "-webkit-mask-size"),
        ("mask-repeat", "-webkit-mask-repeat"),
        ("mask-position", "-webkit-mask-position"),
        ("mask-clip", "-webkit-mask-clip"),
        ("clip-path", "-webkit-clip-path"),
        ("backface-visibility", "-webkit-backface-visibility"),
    ] {
        if let Some(value) = declaration_value(declaration, property) {
            fallbacks.push(format!("{prefixed}: {value};"));
        }
    }
    fallbacks
}

fn image_set_function_fallbacks(declaration: &str) -> Vec<String> {
    let Some((property, value)) = declaration.trim_end_matches(';').split_once(':') else {
        return Vec::new();
    };
    if !value.contains("image-set(") || value.contains("-webkit-image-set(") {
        return Vec::new();
    }
    vec![format!(
        "{}: {};",
        property.trim(),
        value.replace("image-set(", "-webkit-image-set(").trim()
    )]
}

fn custom_property_fallbacks(
    declaration: &str,
    custom_properties: &BTreeMap<String, String>,
) -> Vec<String> {
    let Some((property, value)) = declaration.trim_end_matches(';').split_once(':') else {
        return Vec::new();
    };
    resolve_custom_property_value(value.trim(), custom_properties)
        .filter(|fallback| fallback != value.trim())
        .map(|fallback| vec![format!("{}: {};", property.trim(), fallback)])
        .unwrap_or_default()
}

fn resolve_custom_property_value(
    value: &str,
    custom_properties: &BTreeMap<String, String>,
) -> Option<String> {
    let mut output = String::new();
    let mut index = 0usize;
    let mut replaced = false;

    while let Some(offset) = value[index..].find("var(") {
        let start = index + offset;
        let open = start + "var".len();
        let close = matching_paren(value, open)?;
        output.push_str(&value[index..start]);
        output.push_str(&resolve_var_contents(
            &value[open + 1..close],
            custom_properties,
        )?);
        replaced = true;
        index = close + 1;
    }

    output.push_str(&value[index..]);
    replaced.then(|| output.trim().to_string())
}

fn resolve_var_contents(
    contents: &str,
    custom_properties: &BTreeMap<String, String>,
) -> Option<String> {
    let (name, authored_fallback) = split_top_level_comma(contents)
        .map(|(name, fallback)| (name.trim(), Some(fallback.trim())))
        .unwrap_or_else(|| (contents.trim(), None));
    if !name.starts_with("--") {
        return None;
    }
    if let Some(value) = custom_properties.get(name) {
        return Some(value.clone());
    }
    let fallback = authored_fallback?;
    if fallback.contains("var(") {
        resolve_custom_property_value(fallback, custom_properties)
    } else {
        Some(fallback.to_string())
    }
}

fn split_top_level_comma(input: &str) -> Option<(&str, &str)> {
    let mut depth = 0usize;
    for (index, ch) in input.char_indices() {
        match ch {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => return Some((&input[..index], &input[index + 1..])),
            _ => {}
        }
    }
    None
}

fn matching_paren(input: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (offset, ch) in input[open..].char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(open + offset);
                }
            }
            _ => {}
        }
    }
    None
}

fn env_function_fallbacks(declaration: &str) -> Vec<String> {
    let Some((property, value)) = declaration.trim_end_matches(';').split_once(':') else {
        return Vec::new();
    };
    let re = Regex::new(r"env\(\s*[-A-Za-z0-9_]+\s*,\s*([^)]+?)\s*\)").expect("env fallback regex");
    let mut fallbacks = Vec::new();
    for capture in re.captures_iter(value) {
        fallbacks.push(format!("{}: {};", property.trim(), capture[1].trim()));
    }
    fallbacks
}

fn color_function_fallbacks(declaration: &str) -> Vec<(&str, String)> {
    let Some((property, value)) = declaration.trim_end_matches(';').split_once(':') else {
        return Vec::new();
    };
    let property = property.trim();
    let mut fallbacks = Vec::new();

    let rgb_re = Regex::new(r"rgb\(\s*(\d{1,3})\s+(\d{1,3})\s+(\d{1,3})\s*/\s*([0-9.]+)(%)?\s*\)")
        .expect("rgb slash regex");
    if let Some(capture) = rgb_re.captures(value) {
        if let Some(alpha) = alpha_value(&capture[4], capture.get(5).is_some()) {
            fallbacks.push((
                property,
                format!(
                    "rgba({}, {}, {}, {})",
                    &capture[1],
                    &capture[2],
                    &capture[3],
                    format_alpha(alpha)
                ),
            ));
        }
    }

    let hsl_re =
        Regex::new(r"hsl\(\s*([0-9.]+(?:deg)?)\s+([0-9.]+%)\s+([0-9.]+%)\s*/\s*([0-9.]+)(%)?\s*\)")
            .expect("hsl slash regex");
    if let Some(capture) = hsl_re.captures(value) {
        if let Some(alpha) = alpha_value(&capture[4], capture.get(5).is_some()) {
            fallbacks.push((
                property,
                format!(
                    "hsla({}, {}, {}, {})",
                    &capture[1],
                    &capture[2],
                    &capture[3],
                    format_alpha(alpha)
                ),
            ));
        }
    }

    let hex_re = Regex::new(r"#([0-9a-fA-F]{8})\b").expect("hex alpha regex");
    if let Some(capture) = hex_re.captures(value) {
        if let Some(fallback) = hex_alpha_fallback(&capture[1]) {
            fallbacks.push((property, fallback));
        }
    }

    if let Some(fallback) = color_mix_fallback(value) {
        fallbacks.push((property, fallback));
    }

    if let Some(fallback) = hwb_fallback(value) {
        fallbacks.push((property, fallback));
    }

    fallbacks
}

fn color_mix_fallback(value: &str) -> Option<String> {
    let re = Regex::new(
        r"color-mix\(\s*in\s+srgb\s*,\s*#([0-9a-fA-F]{6})\s+([0-9.]+)%\s*,\s*#([0-9a-fA-F]{6})(?:\s+([0-9.]+)%)?\s*\)",
    )
    .expect("color-mix fallback regex");
    let capture = re.captures(value)?;
    let first = parse_hex_rgb(&capture[1])?;
    let second = parse_hex_rgb(&capture[3])?;
    let first_weight = capture[2].parse::<f32>().ok()? / 100.0;
    let second_weight = capture
        .get(4)
        .and_then(|value| value.as_str().parse::<f32>().ok())
        .map(|value| value / 100.0)
        .unwrap_or(1.0 - first_weight);
    let total = first_weight + second_weight;
    if total <= f32::EPSILON {
        return None;
    }
    let mix = |a: u8, b: u8| -> u8 {
        (((a as f32 * first_weight) + (b as f32 * second_weight)) / total).round() as u8
    };
    Some(format!(
        "rgb({}, {}, {})",
        mix(first.0, second.0),
        mix(first.1, second.1),
        mix(first.2, second.2)
    ))
}

fn hwb_fallback(value: &str) -> Option<String> {
    let re = Regex::new(
        r"hwb\(\s*([0-9.]+)(?:deg)?\s+([0-9.]+)%\s+([0-9.]+)%(?:\s*/\s*([0-9.]+)(%)?)?\s*\)",
    )
    .expect("hwb fallback regex");
    let capture = re.captures(value)?;
    let hue = capture[1].parse::<f32>().ok()?;
    let whiteness = percentage_unit(&capture[2])?;
    let blackness = percentage_unit(&capture[3])?;
    let alpha = capture
        .get(4)
        .and_then(|value| alpha_value(value.as_str(), capture.get(5).is_some()))
        .unwrap_or(1.0);
    let (red, green, blue) = hwb_to_rgb(hue, whiteness, blackness)?;
    if alpha < 1.0 {
        Some(format!(
            "rgba({red}, {green}, {blue}, {})",
            format_alpha(alpha)
        ))
    } else {
        Some(format!("rgb({red}, {green}, {blue})"))
    }
}

fn percentage_unit(value: &str) -> Option<f32> {
    let parsed = value.parse::<f32>().ok()? / 100.0;
    parsed.is_finite().then_some(parsed.clamp(0.0, 1.0))
}

fn hwb_to_rgb(hue: f32, whiteness: f32, blackness: f32) -> Option<(u8, u8, u8)> {
    if !hue.is_finite() || !whiteness.is_finite() || !blackness.is_finite() {
        return None;
    }

    let total = whiteness + blackness;
    if total >= 1.0 {
        let channel = rgb_channel(whiteness / total);
        return Some((channel, channel, channel));
    }

    let (red, green, blue) = hue_to_rgb_unit(hue);
    let factor = 1.0 - total;
    Some((
        rgb_channel(red * factor + whiteness),
        rgb_channel(green * factor + whiteness),
        rgb_channel(blue * factor + whiteness),
    ))
}

fn hue_to_rgb_unit(hue: f32) -> (f32, f32, f32) {
    let segment = hue.rem_euclid(360.0) / 60.0;
    let x = 1.0 - ((segment % 2.0) - 1.0).abs();
    match segment.floor() as u8 {
        0 => (1.0, x, 0.0),
        1 => (x, 1.0, 0.0),
        2 => (0.0, 1.0, x),
        3 => (0.0, x, 1.0),
        4 => (x, 0.0, 1.0),
        _ => (1.0, 0.0, x),
    }
}

fn rgb_channel(component: f32) -> u8 {
    (component.clamp(0.0, 1.0) * 255.0).round() as u8
}

fn gradient_transparency_fallbacks(declaration: &str) -> Vec<String> {
    let Some((property, value)) = declaration.trim_end_matches(';').split_once(':') else {
        return Vec::new();
    };
    if !value.contains("linear-gradient(") || !value.contains("transparent") {
        return Vec::new();
    }
    let re = Regex::new(r"linear-gradient\(([^,]+),\s*([^,()]+),\s*transparent\s*\)")
        .expect("linear gradient transparent regex");
    let Some(capture) = re.captures(value) else {
        return Vec::new();
    };
    let Some(transparent_stop) = transparent_stop_for(capture[2].trim()) else {
        return Vec::new();
    };
    let fallback_value = re
        .replace(
            value,
            format!("linear-gradient($1, $2, {transparent_stop})"),
        )
        .to_string();
    vec![format!("{}: {};", property.trim(), fallback_value.trim())]
}

fn transparent_stop_for(color: &str) -> Option<String> {
    let (red, green, blue) = match color.trim() {
        "red" => (255, 0, 0),
        "green" => (0, 128, 0),
        "blue" => (0, 0, 255),
        "black" => (0, 0, 0),
        "white" => (255, 255, 255),
        value if value.starts_with('#') && value.len() == 7 => {
            let parsed = parse_hex_rgb(&value[1..])?;
            (parsed.0, parsed.1, parsed.2)
        }
        _ => return None,
    };
    Some(format!("rgba({red}, {green}, {blue}, 0)"))
}

fn alpha_value(value: &str, percent: bool) -> Option<f32> {
    let parsed = value.parse::<f32>().ok()?;
    Some(if percent { parsed / 100.0 } else { parsed })
}

fn hex_alpha_fallback(hex: &str) -> Option<String> {
    let red = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let green = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let blue = u8::from_str_radix(&hex[4..6], 16).ok()?;
    let alpha = u8::from_str_radix(&hex[6..8], 16).ok()? as f32 / 255.0;
    Some(format!(
        "rgba({red}, {green}, {blue}, {})",
        format_alpha(alpha)
    ))
}

fn parse_hex_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    if hex.len() != 6 {
        return None;
    }
    let red = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let green = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let blue = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some((red, green, blue))
}

fn legacy_align_value(value: &str) -> &str {
    match value {
        "flex-start" => "start",
        "flex-end" => "end",
        "self-start" => "start",
        "self-end" => "end",
        _ => value,
    }
}

fn legacy_pack_value(value: &str) -> &str {
    match value {
        "flex-start" => "start",
        "flex-end" => "end",
        "space-between" => "justify",
        "space-around" | "space-evenly" => "distribute",
        _ => value,
    }
}

fn order_group(value: &str) -> String {
    value
        .parse::<i32>()
        .map(|order| (order + 1).to_string())
        .unwrap_or_else(|_| value.to_string())
}

fn format_alpha(alpha: f32) -> String {
    let formatted = format!("{alpha:.3}");
    formatted
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

fn collect_compat_warnings(css: &str, target: DxStyleBrowserTarget, warnings: &mut Vec<String>) {
    if target == DxStyleBrowserTarget::Legacy && css.contains(":has(") {
        warnings.push(
            ":has() compatibility fallback is unsupported; selector is preserved with a warning"
                .to_string(),
        );
    }
    if target == DxStyleBrowserTarget::Legacy && (css.contains(":is(") || css.contains(":where(")) {
        warnings.push(
            ":is()/:where() selector compatibility is diagnostic-only for legacy targets"
                .to_string(),
        );
    }
    if css.contains("oklch(") {
        warnings.push(
            "oklch color fallback is unsupported in the current compatibility layer".to_string(),
        );
    }
    if css.contains("color-mix(") {
        warnings.push("color-mix fallback is limited to simple srgb hex color mixes".to_string());
    }
    if css.contains("gradient(") && css.contains("transparent") {
        warnings.push("transparent gradient compatibility warning: older engines may interpolate through transparent black".to_string());
    }
}

fn source_map_for(source_origins: &[PostcssSourceOrigin]) -> PostcssSourceMapReceipt {
    let mut sources = BTreeSet::new();
    let mut mappings = Vec::new();
    for origin in source_origins {
        sources.insert(origin.source_path.clone());
        mappings.push(PostcssSourceMapMapping {
            generated_line: origin.generated_start_line,
            source_path: origin.source_path.clone(),
            source_line: 1,
        });
    }
    PostcssSourceMapReceipt {
        sources: sources.into_iter().collect(),
        mappings,
    }
}

fn dedupe_warnings(warnings: Vec<String>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    warnings
        .into_iter()
        .filter(|warning| seen.insert(warning.clone()))
        .collect()
}

fn line_prefix(line: &str) -> &str {
    let end = line
        .char_indices()
        .find_map(|(index, ch)| (!ch.is_whitespace()).then_some(index))
        .unwrap_or(line.len());
    &line[..end]
}

fn line_count(input: &str) -> usize {
    input.lines().count().max(1)
}

fn ensure_trailing_newline(mut css: String) -> String {
    while css.contains("\n\n\n") {
        css = css.replace("\n\n\n", "\n\n");
    }
    if !css.ends_with('\n') {
        css.push('\n');
    }
    css
}

const POSTCSS_COMPAT_CONTRACT_WARNINGS: &[&str] = &[
    "autoprefixer parity is partial until equal-output browser target tests cover the full property matrix",
    "grid legacy prefixing records warnings instead of synthesizing unsafe -ms-grid placement",
    "full arbitrary PostCSS plugin parity is not claimed by the DX starter replacement contract",
    ":has() fallback is diagnostic-only",
    "oklch/color-mix fallback coverage is intentionally narrow",
];

const POSTCSS_COMPAT_MATRIX: &[PostcssCompatMatrixEntry] = &[
    PostcssCompatMatrixEntry {
        feature: "css-import-flattening",
        input_css: "@import \"./tokens.css\";\n.button { color: var(--fg); }",
        expected_output_css: ":root { --fg: black; }\n.button { color: var(--fg); }",
        status: PostcssCompatStatus::Supported,
        note: "Local starter imports flatten from DX-provided source maps.",
    },
    PostcssCompatMatrixEntry {
        feature: "nesting-transform",
        input_css: ".card { & .title { color: red; } }",
        expected_output_css: ".card .title { color: red; }",
        status: PostcssCompatStatus::Supported,
        note: "One-level and recursively parsed starter nesting is expanded.",
    },
    PostcssCompatMatrixEntry {
        feature: "nested-at-rule-transform",
        input_css: ".card { @media (width >= 48rem) { color: blue; & .title { color: green; } } @supports (display: grid) { display: grid; } }",
        expected_output_css: "@media (min-width: 48rem) { .card { color: blue; } .card .title { color: green; } }\n@supports (display: grid) { .card { display: grid; } }",
        status: PostcssCompatStatus::Supported,
        note: "Nested @media and @supports rules inside selectors are wrapped around the expanded parent selector.",
    },
    PostcssCompatMatrixEntry {
        feature: "nest-at-rule-transform",
        input_css: ".card { @nest .theme & { color: blue; } }",
        expected_output_css: ".theme .card { color: blue; }",
        status: PostcssCompatStatus::Supported,
        note: "Safe @nest selector rules lower through the same starter nesting path.",
    },
    PostcssCompatMatrixEntry {
        feature: "custom-media",
        input_css: "@custom-media --narrow (width <= 40rem);\n@media (--narrow) { .card { display: block; } }",
        expected_output_css: "@media (max-width: 40rem) { .card { display: block; } }",
        status: PostcssCompatStatus::Supported,
        note: "Named custom media expands to normalized media queries.",
    },
    PostcssCompatMatrixEntry {
        feature: "compound-custom-media",
        input_css: "@custom-media --wide (width >= 64rem);\n@media screen and (--wide) { .hero { display: grid; } }",
        expected_output_css: "@media screen and (min-width: 64rem) { .hero { display: grid; } }",
        status: PostcssCompatStatus::Supported,
        note: "Custom media tokens expand inside compound @media queries without touching declaration values.",
    },
    PostcssCompatMatrixEntry {
        feature: "media-min-max-syntax",
        input_css: "@media (width >= 48rem) { .card { display: grid; } }",
        expected_output_css: "@media (min-width: 48rem) { .card { display: grid; } }",
        status: PostcssCompatStatus::Supported,
        note: "Width min/max syntax is normalized for starter CSS.",
    },
    PostcssCompatMatrixEntry {
        feature: "strict-media-range-syntax",
        input_css: "@media (width > 48rem) { .card { display: block; } }\n@media (height < 40rem) { .card { display: none; } }",
        expected_output_css: "@media (min-width: calc(48rem + 0.02px)) { .card { display: block; } }\n@media (max-height: calc(40rem - 0.02px)) { .card { display: none; } }",
        status: PostcssCompatStatus::Supported,
        note: "Strict greater-than and less-than starter media ranges lower to calc-based min/max guards.",
    },
    PostcssCompatMatrixEntry {
        feature: "mixed-media-range-syntax",
        input_css: "@media (48rem < width <= 72rem) { .card { display: block; } }\n@media (30rem <= height < 50rem) { .card { display: none; } }",
        expected_output_css: "@media (min-width: calc(48rem + 0.02px)) and (max-width: 72rem) { .card { display: block; } }\n@media (min-height: 30rem) and (max-height: calc(50rem - 0.02px)) { .card { display: none; } }",
        status: PostcssCompatStatus::Supported,
        note: "Mixed strict/inclusive starter media ranges lower to readable min/max queries.",
    },
    PostcssCompatMatrixEntry {
        feature: "custom-selectors-safe",
        input_css: "@custom-selector :--control button, .button;\n.card { & :--control { color: red; } }",
        expected_output_css: ".card button, .card .button { color: red; }",
        status: PostcssCompatStatus::Supported,
        note: "Safe custom selectors expand through nesting.",
    },
    PostcssCompatMatrixEntry {
        feature: "logical-property-fallbacks",
        input_css: ".card { margin-inline: 1rem; }",
        expected_output_css: ".card { margin-left: 1rem; margin-right: 1rem; margin-inline: 1rem; }",
        status: PostcssCompatStatus::Partial,
        note: "Legacy target emits common physical fallbacks; full writing-mode semantics are not claimed.",
    },
    PostcssCompatMatrixEntry {
        feature: "logical-directional-fallbacks",
        input_css: ".logical { margin-inline-start: 1rem; padding-inline-end: 2rem; border-inline-start: 1px solid red; text-align: start; }",
        expected_output_css: ".logical { margin-left: 1rem; margin-inline-start: 1rem; padding-right: 2rem; padding-inline-end: 2rem; border-left: 1px solid red; border-inline-start: 1px solid red; text-align: left; text-align: start; }",
        status: PostcssCompatStatus::Partial,
        note: "Official starters get LTR physical fallbacks for directional logical declarations with receipt warnings.",
    },
    PostcssCompatMatrixEntry {
        feature: "autoprefixer-style-prefixing",
        input_css: ".button { user-select: none; appearance: none; }",
        expected_output_css: ".button { -webkit-user-select: none; user-select: none; -webkit-appearance: none; appearance: none; }",
        status: PostcssCompatStatus::Partial,
        note: "Measured canaries emit prefixes, but full Autoprefixer parity remains unclaimed.",
    },
    PostcssCompatMatrixEntry {
        feature: "expanded-prefix-families",
        input_css: ".panel { display: inline-flex; hyphens: auto; text-size-adjust: 100%; mask-image: linear-gradient(black, transparent); }",
        expected_output_css: ".panel { display: -webkit-inline-box; display: -ms-inline-flexbox; -webkit-hyphens: auto; -webkit-text-size-adjust: 100%; -webkit-mask-image: linear-gradient(black, transparent); }",
        status: PostcssCompatStatus::Partial,
        note: "Legacy target emits additional form, text, mask, transform, and flexbox canary prefixes while avoiding universal parity claims.",
    },
    PostcssCompatMatrixEntry {
        feature: "preset-env-future-css",
        input_css: ".card { color: rgb(255 0 0 / 50%); }",
        expected_output_css: ".card { color: rgba(255, 0, 0, 0.5); color: rgb(255 0 0 / 50%); }",
        status: PostcssCompatStatus::Partial,
        note: "High-value color notation fallback is covered narrowly.",
    },
    PostcssCompatMatrixEntry {
        feature: "place-property-fallbacks",
        input_css: ".hero { place-items: center start; place-content: space-between center; place-self: stretch end; }",
        expected_output_css: ".hero { align-items: center; justify-items: start; place-items: center start; align-content: space-between; justify-content: center; place-content: space-between center; align-self: stretch; justify-self: end; place-self: stretch end; }",
        status: PostcssCompatStatus::Supported,
        note: "Legacy target emits physical align/justify fallbacks for starter place-* declarations.",
    },
    PostcssCompatMatrixEntry {
        feature: "image-set-prefix-fallback",
        input_css: ".hero { background-image: image-set(url(\"hero.avif\") type(\"image/avif\") 1x, url(\"hero.png\") type(\"image/png\") 1x); }",
        expected_output_css: ".hero { background-image: -webkit-image-set(url(\"hero.avif\") type(\"image/avif\") 1x, url(\"hero.png\") type(\"image/png\") 1x); background-image: image-set(url(\"hero.avif\") type(\"image/avif\") 1x, url(\"hero.png\") type(\"image/png\") 1x); }",
        status: PostcssCompatStatus::Supported,
        note: "Legacy target emits WebKit image-set() function prefixes for responsive image starter CSS.",
    },
    PostcssCompatMatrixEntry {
        feature: "custom-property-env-fallbacks",
        input_css: ":root { --brand: #1d4ed8; }\n.card { color: var(--brand); padding-top: env(safe-area-inset-top, 1rem); }",
        expected_output_css: ".card { color: #1d4ed8; color: var(--brand); padding-top: 1rem; padding-top: env(safe-area-inset-top, 1rem); }",
        status: PostcssCompatStatus::Supported,
        note: "Official starter CSS emits readable fallbacks for simple root custom properties and env fallback arguments.",
    },
    PostcssCompatMatrixEntry {
        feature: "custom-property-var-fallbacks",
        input_css: ":root { --brand: #1d4ed8; }\n.card { color: var(--missing-fg, #111827); border: 1px solid var(--brand); padding: var(--space, calc(1rem + 2px)); }",
        expected_output_css: ".card { color: #111827; color: var(--missing-fg, #111827); border: 1px solid #1d4ed8; border: 1px solid var(--brand); padding: calc(1rem + 2px); padding: var(--space, calc(1rem + 2px)); }",
        status: PostcssCompatStatus::Supported,
        note: "var() fallbacks substitute full declaration values using root tokens or authored fallback arguments.",
    },
    PostcssCompatMatrixEntry {
        feature: "simple-selector-list-lowering",
        input_css: ".button:is(.primary, .secondary) { color: red; }\n.button:where(:focus, :focus-visible) { outline: 2px solid red; }",
        expected_output_css: ".button.primary, .button.secondary { color: red; }\n.button:focus, .button:focus-visible { outline: 2px solid red; }",
        status: PostcssCompatStatus::Supported,
        note: "Safe single-level :is() and :where() selector lists lower for legacy targets; :has() remains diagnostic-only.",
    },
    PostcssCompatMatrixEntry {
        feature: "not-selector-list-lowering",
        input_css: ".button:not(.disabled, [aria-disabled=\"true\"]) { opacity: 1; }",
        expected_output_css: ".button:not(.disabled):not([aria-disabled=\"true\"]) { opacity: 1; }",
        status: PostcssCompatStatus::Supported,
        note: "Safe single-level :not() selector lists lower to chained :not() selectors for legacy targets.",
    },
    PostcssCompatMatrixEntry {
        feature: "color-mix-fallbacks",
        input_css: ".button { background-color: color-mix(in srgb, #000000 25%, #ffffff); }",
        expected_output_css: ".button { background-color: rgb(191, 191, 191); background-color: color-mix(in srgb, #000000 25%, #ffffff); }",
        status: PostcssCompatStatus::Partial,
        note: "Simple srgb hex color-mix() values emit readable rgb fallbacks; broader color spaces remain warning-only.",
    },
    PostcssCompatMatrixEntry {
        feature: "hwb-color-fallbacks",
        input_css: ".swatch { color: hwb(210 20% 30% / 75%); }",
        expected_output_css: ".swatch { color: rgba(51, 115, 179, 0.75); color: hwb(210 20% 30% / 75%); }",
        status: PostcssCompatStatus::Partial,
        note: "Simple HWB colors with numeric or deg hues emit rgb/rgba fallbacks while preserving the authored color.",
    },
    PostcssCompatMatrixEntry {
        feature: "gradient-transparent-stop-fix",
        input_css: ".button { background-image: linear-gradient(to right, red, transparent); }",
        expected_output_css: ".button { background-image: linear-gradient(to right, red, rgba(255, 0, 0, 0)); background-image: linear-gradient(to right, red, transparent); }",
        status: PostcssCompatStatus::Partial,
        note: "Simple transparent gradient stops emit same-color transparent fallbacks while retaining the authored gradient.",
    },
    PostcssCompatMatrixEntry {
        feature: "grid-template-prefix-evidence",
        input_css: ".layout { display: grid; grid-template-columns: 12rem 1fr; grid-template-rows: auto 1fr; }",
        expected_output_css: ".layout { display: -ms-grid; display: grid; -ms-grid-columns: 12rem 1fr; grid-template-columns: 12rem 1fr; -ms-grid-rows: auto 1fr; grid-template-rows: auto 1fr; }",
        status: PostcssCompatStatus::Partial,
        note: "Legacy target emits safe -ms-grid display and track canaries but still warns that placement synthesis is not universal.",
    },
    PostcssCompatMatrixEntry {
        feature: "selector-compat-diagnostics",
        input_css: ".card:has(img) { display: block; }",
        expected_output_css: ".card:has(img) { display: block; }",
        status: PostcssCompatStatus::Partial,
        note: ":has() is preserved with a diagnostic warning for legacy targets.",
    },
    PostcssCompatMatrixEntry {
        feature: "color-function-fallbacks",
        input_css: ".card { color: oklch(62% 0.2 25); }",
        expected_output_css: ".card { color: oklch(62% 0.2 25); }",
        status: PostcssCompatStatus::Partial,
        note: "Unsupported color functions stay readable and receipt-warned.",
    },
    PostcssCompatMatrixEntry {
        feature: "gradient-transparency-compat",
        input_css: ".card { background: linear-gradient(to right, red, transparent); }",
        expected_output_css: ".card { background: linear-gradient(to right, red, transparent); }",
        status: PostcssCompatStatus::Partial,
        note: "Transparent gradient risk is warned instead of silently rewritten.",
    },
    PostcssCompatMatrixEntry {
        feature: "page-break-fallbacks",
        input_css: ".print { break-before: page; break-inside: avoid; }",
        expected_output_css: ".print { page-break-before: always; break-before: page; page-break-inside: avoid; break-inside: avoid; }",
        status: PostcssCompatStatus::Supported,
        note: "Print fragmentation fallbacks are emitted for common starter declarations.",
    },
    PostcssCompatMatrixEntry {
        feature: "flex-grid-prefix-evidence",
        input_css: ".flex { display: flex; }\n.grid { display: grid; }",
        expected_output_css: ".flex { display: -webkit-box; display: -ms-flexbox; display: flex; }\n.grid { display: grid; }",
        status: PostcssCompatStatus::Partial,
        note: "Flex legacy display prefixes are emitted; grid legacy placement is warning-only.",
    },
    PostcssCompatMatrixEntry {
        feature: "sourcemap-source-origin",
        input_css: "@import \"./tokens.css\";\n.card { color: var(--fg); }",
        expected_output_css: "receipt.source_origins + receipt.source_map.sources",
        status: PostcssCompatStatus::Supported,
        note: "Receipts expose source-origin and coarse mapping fields for consumers.",
    },
    PostcssCompatMatrixEntry {
        feature: "minification",
        input_css: ".card { color: red; }",
        expected_output_css: ".card { color: red; }",
        status: PostcssCompatStatus::IntentionallyDifferent,
        note: "Compatibility output remains readable; minification is not enabled here.",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_strings_match_fixture_vocabulary() {
        assert_eq!(PostcssCompatStatus::Supported.as_str(), "supported");
        assert_eq!(PostcssCompatStatus::Partial.as_str(), "partial");
        assert_eq!(PostcssCompatStatus::Unsupported.as_str(), "unsupported");
        assert_eq!(
            PostcssCompatStatus::IntentionallyDifferent.as_str(),
            "intentionally-different"
        );
    }
}
