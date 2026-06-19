use super::*;

/// Compiled DXM content
#[derive(Debug, Clone)]
pub struct CompiledContent {
    /// Frontmatter metadata
    pub frontmatter: ContentFrontmatter,
    /// Compiled binary content
    pub binary: Vec<u8>,
    /// HTML output
    pub html: String,
}

/// Content frontmatter
#[derive(Debug, Clone, Default)]
pub struct ContentFrontmatter {
    /// Page title
    pub title: Option<String>,
    /// Page description
    pub description: Option<String>,
    /// Layout to use
    pub layout: Option<String>,
}

/// Compile DXM content to component
pub fn compile_dxm_content(path: &Path) -> Result<CompiledContent> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read DXM file: {}", path.display()))?;

    // Parse frontmatter
    let (frontmatter, body) = parse_frontmatter(&content);

    // Convert to HTML (placeholder - would use dx-markdown)
    let html = markdown_to_html(&body);

    // Generate binary (placeholder)
    let binary = body.as_bytes().to_vec();

    Ok(CompiledContent {
        frontmatter,
        binary,
        html,
    })
}

/// Parse frontmatter from content
pub(crate) fn parse_frontmatter(content: &str) -> (ContentFrontmatter, String) {
    let mut frontmatter = ContentFrontmatter::default();
    let mut body = content.to_string();

    // Check for YAML frontmatter (--- ... ---)
    if let Some(after_start) = content.strip_prefix("---") {
        if let Some(end) = after_start.find("---") {
            let fm_content = &after_start[..end];
            body = after_start[end + 3..].trim().to_string();

            // Parse simple key: value pairs
            for line in fm_content.lines() {
                let line = line.trim();
                if let Some(colon_pos) = line.find(':') {
                    let key = line[..colon_pos].trim();
                    let value = line[colon_pos + 1..].trim().trim_matches('"');

                    match key {
                        "title" => frontmatter.title = Some(value.to_string()),
                        "description" => frontmatter.description = Some(value.to_string()),
                        "layout" => frontmatter.layout = Some(value.to_string()),
                        _ => {}
                    }
                }
            }
        }
    }

    (frontmatter, body)
}

/// Convert markdown to HTML (simple implementation)
pub(crate) fn markdown_to_html(markdown: &str) -> String {
    let mut html = String::new();
    let mut in_code_block = false;

    for line in markdown.lines() {
        if let Some(lang_part) = line.strip_prefix("```") {
            if in_code_block {
                html.push_str("</code></pre>\n");
                in_code_block = false;
            } else {
                let code_lang = lang_part.trim();
                html.push_str(&format!(
                    "<pre><code class=\"language-{}\">",
                    if code_lang.is_empty() {
                        "text"
                    } else {
                        code_lang
                    }
                ));
                in_code_block = true;
            }
            continue;
        }

        if in_code_block {
            html.push_str(&html_escape(line));
            html.push('\n');
            continue;
        }

        // Headers
        if let Some(content) = line.strip_prefix("# ") {
            html.push_str(&format!("<h1>{}</h1>\n", content));
        } else if let Some(content) = line.strip_prefix("## ") {
            html.push_str(&format!("<h2>{}</h2>\n", content));
        } else if let Some(content) = line.strip_prefix("### ") {
            html.push_str(&format!("<h3>{}</h3>\n", content));
        } else if let Some(content) = line.strip_prefix("#### ") {
            html.push_str(&format!("<h4>{}</h4>\n", content));
        } else if let Some(content) = line.strip_prefix("- ") {
            html.push_str(&format!("<li>{}</li>\n", content));
        } else if let Some(content) = line.strip_prefix("* ") {
            html.push_str(&format!("<li>{}</li>\n", content));
        } else if line.is_empty() {
            html.push_str("<br>\n");
        } else {
            html.push_str(&format!("<p>{}</p>\n", line));
        }
    }

    html
}

/// Escape HTML special characters
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

// ============================================================================
// Legacy Ecosystem Features (from original implementation)
// ============================================================================
