use serde::{Deserialize, Serialize};

use super::types::DxDeliveryMode;

/// Optimized compiler-generated HTML plus the route profile used to serve it.
///
/// This is intentionally conservative: it targets HTML emitted by DX-WWW
/// generators, where whitespace is structural noise unless it sits inside text,
/// script, style, pre, or textarea content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxOptimizedHtml {
    html: String,
    profile: DxHtmlRouteProfile,
}

impl DxOptimizedHtml {
    /// The optimized HTML body.
    pub fn html(&self) -> &str {
        &self.html
    }

    /// Facts extracted while optimizing this route.
    pub fn profile(&self) -> &DxHtmlRouteProfile {
        &self.profile
    }
}

/// Route-level delivery facts for generated HTML.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxHtmlRouteProfile {
    /// Source bytes before optimization.
    pub original_bytes: usize,
    /// Bytes after generated-HTML optimization.
    pub optimized_bytes: usize,
    /// Bytes removed by the optimizer.
    pub saved_bytes: usize,
    /// Number of script start tags.
    pub script_count: usize,
    /// Number of style start tags.
    pub style_count: usize,
    /// Count of repeated row/card/list-like start tags.
    pub repeated_node_count: usize,
    /// Best delivery mode for this route shape.
    pub delivery_mode: DxDeliveryMode,
}

/// Optimize and profile generated HTML for route serving.
pub fn optimize_generated_html(source: &str) -> DxOptimizedHtml {
    let minified = minify_generated_html(source);
    let visible_repeated_nodes =
        count_start_tags(&minified, "article") + count_start_tags(&minified, "li");
    let should_defer_repeated_nodes =
        visible_repeated_nodes >= 128 && route_supports_repeated_visibility(&minified);
    let html = add_repeated_content_visibility_hints(minified, should_defer_repeated_nodes);
    let profile = profile_generated_html(source, &html);
    DxOptimizedHtml { html, profile }
}

/// Minify HTML emitted by DX-WWW generators without touching meaningful text.
pub fn minify_generated_html(source: &str) -> String {
    let mut output = String::with_capacity(source.len());
    let mut index = 0usize;

    while index < source.len() {
        let rest = &source[index..];

        if rest.starts_with("<!--") {
            if let Some(end) = rest.find("-->") {
                index += end + 3;
                continue;
            }
        }

        if rest.starts_with('<') {
            let Some(end) = rest.find('>') else {
                output.push_str(rest);
                break;
            };
            let raw_tag = &rest[..=end];
            let tag = minify_tag(raw_tag);
            let tag_name = opening_tag_name(&tag);
            output.push_str(&tag);
            index += end + 1;

            if let Some(name) = tag_name {
                if is_preserved_content_tag(&name) {
                    let close = format!("</{name}");
                    let remaining = &source[index..];
                    if let Some(close_index) = find_ascii_case_insensitive(remaining, &close) {
                        let content = &remaining[..close_index];
                        if name == "style" {
                            output.push_str(&minify_css_block(content));
                        } else {
                            output.push_str(content.trim());
                        }
                        index += close_index;
                    }
                }
            }
            continue;
        }

        let Some(ch) = rest.chars().next() else {
            break;
        };

        if ch.is_whitespace() {
            let next = next_non_whitespace(rest);
            let previous = output.chars().last();
            if previous.is_some_and(|value| !value.is_whitespace())
                && next.is_some()
                && !(previous == Some('>') && next == Some('<'))
            {
                output.push(' ');
            }
            index += ch.len_utf8();
            while index < source.len() {
                let next_rest = &source[index..];
                let Some(next_ch) = next_rest.chars().next() else {
                    break;
                };
                if !next_ch.is_whitespace() {
                    break;
                }
                index += next_ch.len_utf8();
            }
            continue;
        }

        output.push(ch);
        index += ch.len_utf8();
    }

    output.trim().to_string()
}

fn profile_generated_html(original: &str, optimized: &str) -> DxHtmlRouteProfile {
    let script_count = count_start_tags(optimized, "script");
    let style_count = count_start_tags(optimized, "style");
    let repeated_node_count = ["tr", "article", "li", "option"]
        .iter()
        .map(|tag| count_start_tags(optimized, tag))
        .sum();
    let delivery_mode =
        choose_html_delivery_mode(optimized.len(), script_count, repeated_node_count);

    DxHtmlRouteProfile {
        original_bytes: original.len(),
        optimized_bytes: optimized.len(),
        saved_bytes: original.len().saturating_sub(optimized.len()),
        script_count,
        style_count,
        repeated_node_count,
        delivery_mode,
    }
}

fn choose_html_delivery_mode(
    optimized_bytes: usize,
    script_count: usize,
    repeated_node_count: usize,
) -> DxDeliveryMode {
    if repeated_node_count >= 512 || optimized_bytes >= 96 * 1024 {
        return DxDeliveryMode::ColumnarSlots;
    }
    if repeated_node_count >= 128 {
        return DxDeliveryMode::TemplateSlots;
    }
    if script_count == 0 {
        return DxDeliveryMode::Static;
    }
    if script_count <= 2 && optimized_bytes <= 64 * 1024 {
        return DxDeliveryMode::MicroJs;
    }
    DxDeliveryMode::TemplateSlots
}

fn add_repeated_content_visibility_hints(
    html: String,
    should_defer_repeated_nodes: bool,
) -> String {
    if !should_defer_repeated_nodes || html.contains("content-visibility") {
        return html;
    }

    let Some(head_end) = find_ascii_case_insensitive(&html, "</head>") else {
        return html;
    };

    let hint = "<style>article,li{content-visibility:auto;contain-intrinsic-size:140px}</style>";
    let mut optimized = String::with_capacity(html.len() + hint.len());
    optimized.push_str(&html[..head_end]);
    optimized.push_str(hint);
    optimized.push_str(&html[head_end..]);
    optimized
}

fn route_supports_repeated_visibility(html: &str) -> bool {
    html.contains("data-search")
        || (count_start_tags(html, "input") > 0 && count_start_tags(html, "script") > 0)
}

fn minify_tag(tag: &str) -> String {
    let mut output = String::with_capacity(tag.len());
    let mut in_quote = false;
    let mut quote = '\0';
    let mut pending_space = false;

    for ch in tag.trim().chars() {
        if in_quote {
            output.push(ch);
            if ch == quote {
                in_quote = false;
            }
            continue;
        }

        match ch {
            '"' | '\'' => {
                if pending_space && should_keep_tag_space(output.chars().last()) {
                    output.push(' ');
                }
                pending_space = false;
                in_quote = true;
                quote = ch;
                output.push(ch);
            }
            '=' => {
                while output.ends_with(' ') {
                    output.pop();
                }
                output.push('=');
                pending_space = false;
            }
            value if value.is_whitespace() => {
                pending_space = true;
            }
            _ => {
                if pending_space && should_keep_tag_space(output.chars().last()) && ch != '>' {
                    output.push(' ');
                }
                pending_space = false;
                output.push(ch);
            }
        }
    }

    output
}

fn should_keep_tag_space(previous: Option<char>) -> bool {
    previous.is_some_and(|ch| !matches!(ch, '<' | '/' | '=' | ' '))
}

fn opening_tag_name(tag: &str) -> Option<String> {
    let trimmed = tag.trim_start();
    if !trimmed.starts_with('<') || trimmed.starts_with("</") {
        return None;
    }

    let name: String = trimmed[1..]
        .chars()
        .skip_while(|ch| matches!(ch, '!' | '?'))
        .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '-')
        .collect();

    if name.is_empty() {
        None
    } else {
        Some(name.to_ascii_lowercase())
    }
}

fn is_preserved_content_tag(name: &str) -> bool {
    matches!(name, "script" | "style" | "pre" | "textarea")
}

fn minify_css_block(source: &str) -> String {
    let mut output = String::with_capacity(source.len());
    let mut chars = source.trim().chars().peekable();
    let mut in_quote = false;
    let mut quote = '\0';
    let mut previous = '\0';
    let mut pending_space = false;

    while let Some(ch) = chars.next() {
        if in_quote {
            output.push(ch);
            if ch == quote && previous != '\\' {
                in_quote = false;
            }
            previous = ch;
            continue;
        }

        if previous == '/' && ch == '*' {
            output.pop();
            let mut last = '\0';
            for comment_ch in chars.by_ref() {
                if last == '*' && comment_ch == '/' {
                    break;
                }
                last = comment_ch;
            }
            previous = '\0';
            continue;
        }

        match ch {
            '"' | '\'' => {
                if pending_space && should_keep_css_space(output.chars().last(), Some(ch)) {
                    output.push(' ');
                }
                pending_space = false;
                in_quote = true;
                quote = ch;
                output.push(ch);
            }
            value if value.is_whitespace() => {
                pending_space = true;
            }
            '{' | '}' | ':' | ';' | ',' | '>' | '+' | '~' => {
                while output.ends_with(' ') {
                    output.pop();
                }
                output.push(ch);
                pending_space = false;
            }
            _ => {
                if pending_space && should_keep_css_space(output.chars().last(), Some(ch)) {
                    output.push(' ');
                }
                pending_space = false;
                output.push(ch);
            }
        }
        previous = ch;
    }

    output.trim().to_string()
}

fn should_keep_css_space(previous: Option<char>, next: Option<char>) -> bool {
    let structural = ['{', '}', ':', ';', ',', '>', '+', '~', '(', ')'];
    previous.is_some_and(|ch| !structural.contains(&ch) && ch != ' ')
        && next.is_some_and(|ch| !structural.contains(&ch))
}

fn next_non_whitespace(source: &str) -> Option<char> {
    source.chars().find(|ch| !ch.is_whitespace())
}

fn count_start_tags(source: &str, tag_name: &str) -> usize {
    let needle = format!("<{tag_name}");
    let mut count = 0usize;
    let mut rest = source;

    while let Some(index) = find_ascii_case_insensitive(rest, &needle) {
        let after = &rest[index + needle.len()..];
        if after
            .chars()
            .next()
            .is_none_or(|ch| ch.is_whitespace() || ch == '>' || ch == '/')
        {
            count += 1;
        }
        rest = &after[after.chars().next().map_or(0, char::len_utf8)..];
    }

    count
}

fn find_ascii_case_insensitive(haystack: &str, needle: &str) -> Option<usize> {
    haystack
        .to_ascii_lowercase()
        .find(&needle.to_ascii_lowercase())
}
