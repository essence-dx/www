use super::*;

/// Processed icons result
#[derive(Debug, Clone)]
pub struct ProcessedIcons {
    /// Icons used in the source
    pub used: Vec<String>,
    /// Resolved icon data
    pub resolved: Vec<ResolvedIcon>,
    /// Generated sprite sheet (if tree-shaking enabled)
    pub sprite: Option<String>,
}

/// Resolved icon data
#[derive(Debug, Clone)]
pub struct ResolvedIcon {
    /// Icon name
    pub name: String,
    /// Icon set (prefix)
    pub set: String,
    /// SVG content
    pub svg: String,
    /// Icon width
    pub width: u32,
    /// Icon height
    pub height: u32,
}

/// Process icons in source code using DX-owned icon tags/components.
pub fn process_icons(source: &str, config: &IconConfig) -> Result<ProcessedIcons> {
    let mut icons_used = Vec::new();
    let mut seen = HashSet::new();

    let icon_regex = regex::Regex::new(r#"<(?:icon|dx-icon|Icon|DxIcon)\s+([^>]*)/?>"#)?;

    for cap in icon_regex.captures_iter(source) {
        let attrs = cap.get(1).map(|m| m.as_str()).unwrap_or_default();
        let Some((final_set, final_name)) = parse_icon_attrs(attrs, config) else {
            continue;
        };
        let key = format!("{}:{}", final_set, final_name);
        if !seen.contains(&key) {
            seen.insert(key.clone());
            icons_used.push(key);
        }
    }

    // Resolve icons through dx-icon
    let resolved = resolve_icons_from_library(&icons_used, config)?;

    // Generate sprite sheet if tree-shaking is enabled
    let sprite = if config.tree_shake && !resolved.is_empty() {
        Some(generate_icon_sprite(&resolved))
    } else {
        None
    };

    // Extract just the names for the used list
    let used_names: Vec<String> = icons_used
        .iter()
        .map(|k| k.split(':').next_back().unwrap_or(k).to_string())
        .collect();

    Ok(ProcessedIcons {
        used: used_names,
        resolved,
        sprite,
    })
}

fn parse_icon_attrs(attrs: &str, config: &IconConfig) -> Option<(String, String)> {
    let set = extract_attr(attrs, "set");
    let name = extract_attr(attrs, "name")?;

    if let Some(set) = set {
        return Some((set, name));
    }

    if let Some((set, icon)) = name.split_once(':') {
        return Some((set.to_string(), icon.to_string()));
    }

    let default_set = config
        .sets
        .first()
        .cloned()
        .unwrap_or_else(|| "pack".to_string());
    Some((default_set, name))
}

fn extract_attr(attrs: &str, name: &str) -> Option<String> {
    let double = regex::Regex::new(&format!(r#"{name}="([^"]+)""#)).ok()?;
    if let Some(cap) = double.captures(attrs) {
        return cap.get(1).map(|m| m.as_str().to_string());
    }

    let single = regex::Regex::new(&format!(r#"{name}='([^']+)'"#)).ok()?;
    single
        .captures(attrs)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
}

/// Resolve icons from the dx-icon library
fn resolve_icons_from_library(
    icon_keys: &[String],
    config: &IconConfig,
) -> Result<Vec<ResolvedIcon>> {
    let mut reader = dx_icon::icons();
    let mut resolved = Vec::new();

    for key in icon_keys {
        let parts: Vec<&str> = key.splitn(2, ':').collect();
        if parts.len() != 2 {
            continue;
        }

        let (set, name) = (parts[0], parts[1]);

        // Check if this set is allowed by config
        if !config.sets.contains(&set.to_string()) && !config.sets.is_empty() {
            continue;
        }

        // Try to get the icon from dx-icon
        if let Some(icon) = reader.get(set, name) {
            let width = icon.width.unwrap_or(24);
            let height = icon.height.unwrap_or(24);

            resolved.push(ResolvedIcon {
                name: name.to_string(),
                set: set.to_string(),
                svg: icon.to_svg(24),
                width,
                height,
            });
        } else {
            // Fallback: generate placeholder SVG
            resolved.push(ResolvedIcon {
                name: name.to_string(),
                set: set.to_string(),
                svg: format!(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"><!-- {} --></svg>"#, name),
                width: 24,
                height: 24,
            });
        }
    }

    Ok(resolved)
}

/// Tree-shake icons: filter to only include icons actually used in source
pub fn tree_shake_icons(all_icons: &[ResolvedIcon], source: &str) -> Vec<ResolvedIcon> {
    let mut used_icons = Vec::new();

    for icon in all_icons {
        // Check if this icon is referenced in the source
        let patterns = [
            format!(r#"name="{}""#, icon.name),
            format!(r#"name="{}:{}""#, icon.set, icon.name),
            format!(r#"icon-{}"#, icon.name),
        ];

        if patterns.iter().any(|p| source.contains(p)) {
            used_icons.push(icon.clone());
        }
    }

    used_icons
}

/// Generate optimized icon sprite sheet
pub fn generate_icon_sprite(icons: &[ResolvedIcon]) -> String {
    let mut sprite =
        String::from(r#"<svg xmlns="http://www.w3.org/2000/svg" style="display:none">"#);

    for icon in icons {
        // Extract the inner content from the SVG
        let inner = extract_svg_inner(&icon.svg);
        sprite.push_str(&format!(
            r#"<symbol id="icon-{}-{}" viewBox="0 0 {} {}">{}</symbol>"#,
            icon.set, icon.name, icon.width, icon.height, inner
        ));
    }

    sprite.push_str("</svg>");
    sprite
}

/// Extract inner content from SVG element
fn extract_svg_inner(svg: &str) -> String {
    // Find the content between <svg ...> and </svg>
    if let Some(start) = svg.find('>') {
        if let Some(end) = svg.rfind("</svg>") {
            return svg[start + 1..end].to_string();
        }
    }
    svg.to_string()
}

/// Generate CSS for icon usage via sprite
pub fn generate_icon_css(icons: &[ResolvedIcon]) -> String {
    let mut css = String::from(
        r#"
.dx-icon {
  display: inline-block;
  width: 1em;
  height: 1em;
  vertical-align: -0.125em;
  fill: currentColor;
}
"#,
    );

    for icon in icons {
        css.push_str(&format!(
            r#"
.dx-icon-{}-{} {{
  width: {}px;
  height: {}px;
}}
"#,
            icon.set, icon.name, icon.width, icon.height
        ));
    }

    css
}

// ============================================================================
// Font Processing (dx-font integration)
// ============================================================================
