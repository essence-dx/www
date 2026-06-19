use super::*;
use std::collections::HashMap;

/// Processed media result
#[derive(Debug, Clone)]
pub struct ProcessedMedia {
    /// Original file path
    pub original: PathBuf,
    /// Generated variants
    pub variants: Vec<ImageVariant>,
    /// Blur placeholder (base64 data URL)
    pub blur_placeholder: Option<String>,
    /// Original dimensions
    pub original_dimensions: Option<(u32, u32)>,
    /// Dominant color (hex)
    pub dominant_color: Option<String>,
}

/// Image variant
#[derive(Debug, Clone)]
pub struct ImageVariant {
    /// Variant path
    pub path: PathBuf,
    /// Width
    pub width: u32,
    /// Height (calculated from aspect ratio)
    pub height: u32,
    /// Format
    pub format: String,
    /// File size estimate (bytes)
    pub size_estimate: Option<u64>,
}

/// Media processing options
#[derive(Debug, Clone)]
pub struct MediaProcessingOptions {
    /// Output directory for processed media
    pub output_dir: PathBuf,
    /// Quality for lossy formats (1-100)
    pub quality: u8,
    /// Generate blur placeholders
    pub blur_placeholder: bool,
    /// Responsive breakpoints
    pub breakpoints: Vec<u32>,
    /// Output formats
    pub formats: Vec<String>,
    /// Preserve original
    pub preserve_original: bool,
}

impl Default for MediaProcessingOptions {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("dist/images"),
            quality: 85,
            blur_placeholder: true,
            breakpoints: vec![640, 768, 1024, 1280, 1536],
            formats: vec!["webp".to_string(), "avif".to_string()],
            preserve_original: true,
        }
    }
}

/// Detect image imports in source code
pub fn detect_image_imports(source: &str) -> Vec<ImageImport> {
    let mut imports = Vec::new();

    // Match various import patterns:
    // import img from './image.png'
    // import { src } from './image.png?w=640'
    // <img src="./image.png" />
    // <Image src="./image.png" />

    // Pattern 1: ES import
    let import_regex = regex::Regex::new(
        r#"import\s+\w+\s+from\s+['"]([^'"]+\.(png|jpg|jpeg|gif|webp|avif|svg))(?:\?[^'"]*)?['"]"#,
    )
    .unwrap();

    for cap in import_regex.captures_iter(source) {
        if let Some(path) = cap.get(1) {
            imports.push(ImageImport {
                path: PathBuf::from(path.as_str()),
                import_type: ImageImportType::EsModule,
                query_params: extract_query_params(path.as_str()),
            });
        }
    }

    // Pattern 2: JSX img/Image src
    let jsx_regex = regex::Regex::new(
        r#"<(?:img|Image)\s+[^>]*src=["']([^"']+\.(png|jpg|jpeg|gif|webp|avif|svg))(?:\?[^"']*)?["']"#
    ).unwrap();

    for cap in jsx_regex.captures_iter(source) {
        if let Some(path) = cap.get(1) {
            imports.push(ImageImport {
                path: PathBuf::from(path.as_str()),
                import_type: ImageImportType::JsxSrc,
                query_params: extract_query_params(path.as_str()),
            });
        }
    }

    imports
}

/// Image import information
#[derive(Debug, Clone)]
pub struct ImageImport {
    /// Path to the image
    pub path: PathBuf,
    /// Type of import
    pub import_type: ImageImportType,
    /// Query parameters (e.g., ?w=640&format=webp)
    pub query_params: HashMap<String, String>,
}

/// Type of image import
#[derive(Debug, Clone, PartialEq)]
pub enum ImageImportType {
    /// ES module import
    EsModule,
    /// JSX src attribute
    JsxSrc,
    /// CSS url()
    CssUrl,
}

/// Extract query parameters from a path
fn extract_query_params(path: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();

    if let Some(query_start) = path.find('?') {
        let query = &path[query_start + 1..];
        for pair in query.split('&') {
            if let Some(eq_pos) = pair.find('=') {
                let key = pair[..eq_pos].to_string();
                let value = pair[eq_pos + 1..].to_string();
                params.insert(key, value);
            }
        }
    }

    params
}

/// Process media asset with full optimization
pub fn process_media(path: &Path, config: &MediaConfig) -> Result<ProcessedMedia> {
    process_media_with_options(
        path,
        &MediaProcessingOptions {
            quality: config.quality,
            blur_placeholder: config.blur_placeholder,
            breakpoints: config.breakpoints.clone(),
            formats: config.image_formats.clone(),
            ..Default::default()
        },
    )
}

/// Process media asset with custom options
pub fn process_media_with_options(
    path: &Path,
    options: &MediaProcessingOptions,
) -> Result<ProcessedMedia> {
    let mut variants = Vec::new();
    let file_stem = path.file_stem().unwrap_or_default().to_string_lossy();

    // Estimate original dimensions (placeholder - would use image library)
    let original_dimensions = estimate_image_dimensions(path);
    let aspect_ratio = original_dimensions
        .map(|(w, h)| w as f64 / h as f64)
        .unwrap_or(16.0 / 9.0);

    // Generate responsive variants
    for &width in &options.breakpoints {
        // Skip breakpoints larger than original
        if let Some((orig_w, _)) = original_dimensions {
            if width > orig_w {
                continue;
            }
        }

        let height = (width as f64 / aspect_ratio).round() as u32;

        for format in &options.formats {
            let variant_name = format!("{}-{}w.{}", file_stem, width, format);
            let variant_path = options.output_dir.join(&variant_name);

            // Estimate file size based on format and dimensions
            let size_estimate = estimate_file_size(width, height, format, options.quality);

            variants.push(ImageVariant {
                path: variant_path,
                width,
                height,
                format: format.clone(),
                size_estimate: Some(size_estimate),
            });
        }
    }

    // Generate blur placeholder
    let blur_placeholder = if options.blur_placeholder {
        Some(generate_blur_placeholder(path, original_dimensions))
    } else {
        None
    };

    // Extract dominant color (placeholder)
    let dominant_color = extract_dominant_color(path);

    Ok(ProcessedMedia {
        original: path.to_path_buf(),
        variants,
        blur_placeholder,
        original_dimensions,
        dominant_color,
    })
}

/// Estimate image dimensions from file (placeholder)
fn estimate_image_dimensions(path: &Path) -> Option<(u32, u32)> {
    // In production, this would read image headers
    // For now, return a reasonable default based on common image sizes
    let ext = path.extension()?.to_str()?;
    match ext {
        "png" | "jpg" | "jpeg" | "webp" => Some((1920, 1080)),
        "gif" => Some((800, 600)),
        "svg" => Some((100, 100)), // SVG is scalable
        _ => Some((1920, 1080)),
    }
}

/// Estimate file size based on format and dimensions
fn estimate_file_size(width: u32, height: u32, format: &str, quality: u8) -> u64 {
    let pixels = width as u64 * height as u64;
    let quality_factor = quality as f64 / 100.0;

    // Rough estimates based on format compression ratios
    let bytes_per_pixel = match format {
        "avif" => 0.1 * quality_factor,
        "webp" => 0.15 * quality_factor,
        "jpg" | "jpeg" => 0.2 * quality_factor,
        "png" => 0.5, // PNG is lossless
        _ => 0.3,
    };

    (pixels as f64 * bytes_per_pixel) as u64
}

/// Generate blur placeholder (LQIP - Low Quality Image Placeholder)
pub(crate) fn generate_blur_placeholder(_path: &Path, dimensions: Option<(u32, u32)>) -> String {
    let (w, h) = dimensions.unwrap_or((16, 9));
    let aspect = w as f64 / h as f64;

    // Generate a simple SVG blur placeholder
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" preserveAspectRatio="none">
  <filter id="b" color-interpolation-filters="sRGB">
    <feGaussianBlur stdDeviation="20"/>
  </filter>
  <rect width="100%" height="100%" fill="#e0e0e0" filter="url(#b)"/>
</svg>"##,
        (aspect * 10.0).round() as u32,
        10
    );

    // Encode as base64 data URL
    let encoded = base64_encode(svg.as_bytes());
    format!("data:image/svg+xml;base64,{}", encoded)
}

/// Simple base64 encoding
pub(crate) fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = String::new();
    let mut i = 0;

    while i < data.len() {
        let b0 = data[i] as usize;
        let b1 = if i + 1 < data.len() {
            data[i + 1] as usize
        } else {
            0
        };
        let b2 = if i + 2 < data.len() {
            data[i + 2] as usize
        } else {
            0
        };

        result.push(ALPHABET[b0 >> 2] as char);
        result.push(ALPHABET[((b0 & 0x03) << 4) | (b1 >> 4)] as char);

        if i + 1 < data.len() {
            result.push(ALPHABET[((b1 & 0x0f) << 2) | (b2 >> 6)] as char);
        } else {
            result.push('=');
        }

        if i + 2 < data.len() {
            result.push(ALPHABET[b2 & 0x3f] as char);
        } else {
            result.push('=');
        }

        i += 3;
    }

    result
}

/// Extract dominant color from image (placeholder)
fn extract_dominant_color(_path: &Path) -> Option<String> {
    // In production, this would analyze the image
    // Return a neutral gray as placeholder
    Some("#808080".to_string())
}

/// Generate srcset attribute for responsive images
pub fn generate_srcset(variants: &[ImageVariant]) -> String {
    variants
        .iter()
        .map(|v| format!("{} {}w", v.path.display(), v.width))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Generate sizes attribute for responsive images
pub fn generate_sizes(breakpoints: &[(u32, &str)]) -> String {
    // breakpoints: [(max_width, size), ...]
    // e.g., [(640, "100vw"), (1024, "50vw"), (0, "33vw")]
    breakpoints
        .iter()
        .map(|(max_width, size)| {
            if *max_width == 0 {
                size.to_string()
            } else {
                format!("(max-width: {}px) {}", max_width, size)
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// Generate picture element HTML for responsive images
pub fn generate_picture_html(media: &ProcessedMedia, alt: &str) -> String {
    let mut html = String::from("<picture>\n");

    // Group variants by format
    let mut by_format: HashMap<&str, Vec<&ImageVariant>> = HashMap::new();
    for variant in &media.variants {
        by_format.entry(&variant.format).or_default().push(variant);
    }

    // Generate source elements for each format (prefer modern formats first)
    for format in &["avif", "webp"] {
        if let Some(variants) = by_format.get(format as &str) {
            let srcset = variants
                .iter()
                .map(|v| format!("{} {}w", v.path.display(), v.width))
                .collect::<Vec<_>>()
                .join(", ");

            html.push_str(&format!(
                "  <source type=\"image/{}\" srcset=\"{}\">\n",
                format, srcset
            ));
        }
    }

    // Fallback img element
    let fallback = media
        .variants
        .first()
        .map(|v| v.path.display().to_string())
        .unwrap_or_else(|| media.original.display().to_string());

    // Add blur placeholder as style if available
    let style = media
        .blur_placeholder
        .as_ref()
        .map(|p| {
            format!(
                " style=\"background-image: url('{}'); background-size: cover;\"",
                p
            )
        })
        .unwrap_or_default();

    html.push_str(&format!(
        "  <img src=\"{}\" alt=\"{}\" loading=\"lazy\"{}>\n",
        fallback, alt, style
    ));

    html.push_str("</picture>");
    html
}

/// Optimize all media in a directory
pub fn optimize_media_directory(dir: &Path, config: &MediaConfig) -> Result<Vec<ProcessedMedia>> {
    let mut results = Vec::new();

    // Supported image extensions
    let extensions = ["png", "jpg", "jpeg", "gif", "webp", "avif"];

    // Walk directory
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if extensions.contains(&ext.to_lowercase().as_str()) {
                        let processed = process_media(&path, config)?;
                        results.push(processed);
                    }
                }
            }
        }
    }

    Ok(results)
}

// ============================================================================
// DXM Content Processing
// ============================================================================
