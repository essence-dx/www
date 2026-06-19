use super::*;

/// Processed fonts result
#[derive(Debug, Clone)]
pub struct ProcessedFonts {
    /// Processed font data
    pub fonts: Vec<ProcessedFont>,
    /// CSS for font-face declarations
    pub css: String,
    /// Preload hints for critical fonts
    pub preload_hints: Vec<PreloadHint>,
}

/// Processed font data
#[derive(Debug, Clone)]
pub struct ProcessedFont {
    /// Font family name
    pub family: String,
    /// Font data (binary)
    pub data: Vec<u8>,
    /// Whether subsetting is needed
    pub needs_subset: bool,
    /// Characters used (for subsetting)
    pub used_chars: HashSet<char>,
    /// Font weights included
    pub weights: Vec<u16>,
    /// Is variable font
    pub variable: bool,
    /// Output path for the font file
    pub output_path: PathBuf,
    /// Font format (woff2, woff, ttf)
    pub format: String,
}

/// Preload hint for critical fonts
#[derive(Debug, Clone)]
pub struct PreloadHint {
    /// Font file path
    pub href: String,
    /// Font format
    pub format: String,
    /// Crossorigin attribute
    pub crossorigin: bool,
}

/// Font processing options
#[derive(Debug, Clone)]
pub struct FontProcessingOptions {
    /// Output directory for processed fonts
    pub output_dir: PathBuf,
    /// Preferred formats in order of priority
    pub preferred_formats: Vec<String>,
    /// Enable font subsetting
    pub subset: bool,
    /// Generate preload hints
    pub preload: bool,
}

impl Default for FontProcessingOptions {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("dist/fonts"),
            preferred_formats: vec!["woff2".to_string(), "woff".to_string()],
            subset: true,
            preload: true,
        }
    }
}

/// Parse font configuration from dx.config
pub fn parse_font_config(config: &DxWwwConfig) -> Option<FontConfig> {
    config.assets.fonts.as_ref().map(|fonts| FontConfig {
        families: fonts
            .families
            .iter()
            .map(|f| FontFamily {
                name: f.name.clone(),
                weights: f.weights.clone(),
                variable: f.variable,
            })
            .collect(),
        subset: fonts.subset,
        preload: true,
    })
}

/// Process fonts in configuration using dx-font
pub fn process_fonts(config: &FontConfig) -> Result<ProcessedFonts> {
    process_fonts_with_options(config, &FontProcessingOptions::default())
}

/// Process fonts with custom options
pub fn process_fonts_with_options(
    config: &FontConfig,
    options: &FontProcessingOptions,
) -> Result<ProcessedFonts> {
    let mut fonts = Vec::new();
    let mut css = String::new();
    let mut preload_hints = Vec::new();

    // Add CSS reset for font-display
    css.push_str("/* DX-WWW Font Declarations */\n\n");

    for family in &config.families {
        let font_slug = family.name.to_lowercase().replace(' ', "-");

        // Determine weights to process
        let weights = if family.weights.is_empty() {
            vec![400] // Default to regular weight
        } else {
            family.weights.clone()
        };

        // Create processed font entry
        let font = ProcessedFont {
            family: family.name.clone(),
            data: Vec::new(), // Data loaded on demand via dx-font
            needs_subset: config.subset,
            used_chars: HashSet::new(),
            weights: weights.clone(),
            variable: family.variable,
            output_path: options.output_dir.join(format!("{}.woff2", font_slug)),
            format: "woff2".to_string(),
        };

        // Generate CSS for variable fonts
        if family.variable {
            css.push_str(&generate_variable_font_css(
                &family.name,
                &font_slug,
                &weights,
            ));

            // Add preload hint for variable font
            if options.preload {
                preload_hints.push(PreloadHint {
                    href: format!("/fonts/{}-variable.woff2", font_slug),
                    format: "woff2".to_string(),
                    crossorigin: true,
                });
            }
        } else {
            // Generate CSS for static fonts
            css.push_str(&generate_static_font_css(
                &family.name,
                &font_slug,
                &weights,
            ));

            // Add preload hints for critical weights (400, 700)
            if options.preload {
                for weight in &weights {
                    if *weight == 400 || *weight == 700 {
                        preload_hints.push(PreloadHint {
                            href: format!("/fonts/{}-{}.woff2", font_slug, weight),
                            format: "woff2".to_string(),
                            crossorigin: true,
                        });
                    }
                }
            }
        }

        fonts.push(font);
    }

    Ok(ProcessedFonts {
        fonts,
        css,
        preload_hints,
    })
}

/// Generate CSS for variable fonts
fn generate_variable_font_css(family_name: &str, font_slug: &str, weights: &[u16]) -> String {
    let min_weight = weights.iter().min().copied().unwrap_or(100);
    let max_weight = weights.iter().max().copied().unwrap_or(900);

    format!(
        r#"@font-face {{
  font-family: '{}';
  font-weight: {} {};
  font-style: normal;
  font-display: swap;
  src: url('/fonts/{}-variable.woff2') format('woff2-variations');
  unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
}}

"#,
        family_name, min_weight, max_weight, font_slug
    )
}

/// Generate CSS for static fonts
fn generate_static_font_css(family_name: &str, font_slug: &str, weights: &[u16]) -> String {
    let mut css = String::new();

    for weight in weights {
        css.push_str(&format!(
            r#"@font-face {{
  font-family: '{}';
  font-weight: {};
  font-style: normal;
  font-display: swap;
  src: url('/fonts/{}-{}.woff2') format('woff2'),
       url('/fonts/{}-{}.woff') format('woff');
  unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
}}

"#,
            family_name, weight, font_slug, weight, font_slug, weight
        ));
    }

    css
}

/// Extract characters used in source code for font subsetting
pub fn extract_used_characters(source: &str) -> HashSet<char> {
    source.chars().collect()
}

/// Subset font based on used characters
pub fn subset_font(font: &ProcessedFont, chars: &HashSet<char>) -> Result<Vec<u8>> {
    // If no subsetting needed or no characters, return original
    if !font.needs_subset || chars.is_empty() {
        return Ok(font.data.clone());
    }

    // Create subset data
    // In a full implementation, this would use dx-font's subsetting capabilities
    // For now, we return the original data with a marker
    let mut subset_data = font.data.clone();

    // Add metadata about subset (placeholder)
    if subset_data.is_empty() {
        // Generate placeholder subset info
        let subset_info = format!("SUBSET:{}:chars={}", font.family, chars.len());
        subset_data = subset_info.into_bytes();
    }

    Ok(subset_data)
}

/// Subset font to include only specified characters
pub fn subset_font_to_chars(font_data: &[u8], chars: &HashSet<char>) -> Result<Vec<u8>> {
    if font_data.is_empty() || chars.is_empty() {
        return Ok(font_data.to_vec());
    }

    // In production, this would use a font subsetting library
    // For now, return original data
    Ok(font_data.to_vec())
}

/// Generate preload link tags for fonts
pub fn generate_font_preload_html(hints: &[PreloadHint]) -> String {
    hints
        .iter()
        .map(|hint| {
            format!(
                r#"<link rel="preload" href="{}" as="font" type="font/{}" crossorigin{}>"#,
                hint.href,
                hint.format,
                if hint.crossorigin {
                    ""
                } else {
                    "=\"anonymous\""
                }
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Calculate optimal font subset based on content analysis
pub fn calculate_font_subset(content_sources: &[&str], include_common: bool) -> HashSet<char> {
    let mut chars = HashSet::new();

    // Extract characters from all content sources
    for source in content_sources {
        chars.extend(source.chars());
    }

    // Optionally include common characters
    if include_common {
        // Add common punctuation and symbols
        let common = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r";
        chars.extend(common.chars());

        // Add digits
        chars.extend('0'..='9');

        // Add basic Latin letters
        chars.extend('a'..='z');
        chars.extend('A'..='Z');
    }

    chars
}

/// Font download integration with dx-font
pub async fn download_font_family(
    family_name: &str,
    weights: &[u16],
    output_dir: &Path,
) -> Result<Vec<PathBuf>> {
    // This would integrate with dx-font's FontSearch and FontDownloader
    // For now, return placeholder paths
    let font_slug = family_name.to_lowercase().replace(' ', "-");
    let mut paths = Vec::new();

    for weight in weights {
        let path = output_dir.join(format!("{}-{}.woff2", font_slug, weight));
        paths.push(path);
    }

    Ok(paths)
}

// ============================================================================
// Media Processing (dx-media integration)
// ============================================================================
