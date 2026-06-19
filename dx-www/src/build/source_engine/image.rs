use std::{collections::BTreeMap, path::Path};

use serde::{Deserialize, Serialize};

use super::image_bmff::avif_dimensions_from_bmff;
use super::image_placeholder::{
    SourceBuildImagePlaceholder, raster_placeholder_data_url, svg_placeholder_data_url,
};

/// Source-owned metadata recorded for a public image asset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildImageMetadata {
    /// Normalized image format derived from extension and header evidence.
    pub format: String,
    /// MIME type exposed to build graph consumers.
    pub mime_type: String,
    /// Pixel width when the source format exposes it without decoding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    /// Pixel height when the source format exposes it without decoding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    /// Header or source syntax that produced the dimension evidence.
    pub dimension_source: String,
    /// Metadata extraction state for this first receipt surface.
    pub status: String,
    /// Explicit optimization boundary for this asset.
    pub optimization: SourceBuildImageOptimization,
}

/// Optimization evidence recorded without claiming an implemented image pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildImageOptimization {
    /// Current implementation state.
    pub status: String,
    /// Number of transformed image variants emitted by DX.
    pub variants_emitted: usize,
    /// Whether a resizing or encoding optimizer was invoked for this asset.
    pub optimizer_invoked: bool,
    /// Whether resize work was emitted.
    pub resize_emitted: bool,
    /// Whether re-encoding work was emitted.
    pub encoding_emitted: bool,
    /// Whether blur placeholder data was emitted.
    pub blur_placeholder_emitted: bool,
    /// Source-owned placeholder emitted for image consumers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<SourceBuildImagePlaceholder>,
    /// Stable text boundary for receipts and dashboards.
    pub boundary: String,
    /// Upstream concepts used as compatibility references only.
    pub informed_by: Vec<String>,
}

/// One source route that references a public image URL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildImageRouteReference {
    /// URL route that owns the reference.
    pub route: String,
    /// Project-relative source file where the URL was found.
    pub source_path: String,
    /// Static image URL as authored in route source.
    pub specifier: String,
    /// Reference detector used by this metadata-only surface.
    pub kind: String,
}

/// One source stylesheet that references a public image URL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildImageStyleReference {
    /// Project-relative stylesheet source path.
    pub style_path: String,
    /// CSS URL specifier as authored in the stylesheet.
    pub specifier: String,
    /// Reference detector used by the CSS adapter.
    pub kind: String,
    /// CSS URL image references do not require project-local `node_modules`.
    pub node_modules_required: bool,
}

/// Return source-owned image metadata for supported public image assets.
pub fn image_metadata_for_asset(path: &Path, bytes: &[u8]) -> Option<SourceBuildImageMetadata> {
    let format = image_format(path)?;
    let dimension_evidence = dimensions_for_format(&format, bytes);
    let width = dimension_evidence.width;
    let height = dimension_evidence.height;
    let optimization = image_optimization(&format, width, height);
    Some(SourceBuildImageMetadata {
        mime_type: mime_type(&format).to_string(),
        format,
        width,
        height,
        dimension_source: dimension_evidence.source.to_string(),
        status: if width.is_some() && height.is_some() {
            "metadata-read"
        } else {
            "metadata-format-only"
        }
        .to_string(),
        optimization,
    })
}

/// Count image metadata assets and emitted optimization variants.
pub fn image_summary(assets: &[super::graph::SourceBuildAsset]) -> (usize, usize, usize) {
    let image_assets = assets
        .iter()
        .filter(|asset| asset.image_metadata.is_some())
        .count();
    let metadata_assets = assets
        .iter()
        .filter(|asset| {
            asset
                .image_metadata
                .as_ref()
                .is_some_and(|metadata| metadata.width.is_some() && metadata.height.is_some())
        })
        .count();
    let optimized_variants = assets
        .iter()
        .filter_map(|asset| asset.image_metadata.as_ref())
        .map(|metadata| metadata.optimization.variants_emitted)
        .sum();
    (image_assets, metadata_assets, optimized_variants)
}

/// Count source-owned image placeholders emitted into receipts.
pub fn image_placeholder_count(assets: &[super::graph::SourceBuildAsset]) -> usize {
    assets
        .iter()
        .filter_map(|asset| asset.image_metadata.as_ref())
        .filter(|metadata| metadata.optimization.placeholder.is_some())
        .count()
}

/// Count source-owned image placeholder artifacts with emitted file evidence.
pub fn image_placeholder_artifact_count(assets: &[super::graph::SourceBuildAsset]) -> usize {
    assets
        .iter()
        .filter_map(|asset| asset.image_metadata.as_ref())
        .filter_map(|metadata| metadata.optimization.placeholder.as_ref())
        .filter(|placeholder| {
            placeholder.output.is_some()
                && placeholder.hash.is_some()
                && placeholder.artifact_bytes.is_some()
        })
        .count()
}

/// Sum emitted placeholder artifact bytes recorded on source-owned image metadata.
pub fn image_placeholder_artifact_bytes(assets: &[super::graph::SourceBuildAsset]) -> usize {
    assets
        .iter()
        .filter_map(|asset| asset.image_metadata.as_ref())
        .filter_map(|metadata| metadata.optimization.placeholder.as_ref())
        .filter(|placeholder| placeholder.output.is_some() && placeholder.hash.is_some())
        .filter_map(|placeholder| placeholder.artifact_bytes)
        .sum()
}

/// List source-owned emitted placeholder artifact outputs for build consumers.
pub fn image_placeholder_artifact_outputs(
    assets: &[super::graph::SourceBuildAsset],
) -> Vec<String> {
    let mut outputs = assets
        .iter()
        .filter_map(|asset| asset.image_metadata.as_ref())
        .filter_map(|metadata| metadata.optimization.placeholder.as_ref())
        .filter(|placeholder| placeholder.hash.is_some() && placeholder.artifact_bytes.is_some())
        .filter_map(|placeholder| placeholder.output.clone())
        .collect::<Vec<_>>();
    outputs.sort();
    outputs
}

/// Count image metadata assets by normalized image format.
pub fn image_format_counts(assets: &[super::graph::SourceBuildAsset]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for metadata in assets
        .iter()
        .filter_map(|asset| asset.image_metadata.as_ref())
    {
        *counts.entry(metadata.format.clone()).or_insert(0) += 1;
    }
    counts
}

/// Count image metadata assets by the dimension evidence source.
pub fn image_dimension_source_counts(
    assets: &[super::graph::SourceBuildAsset],
) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for metadata in assets
        .iter()
        .filter_map(|asset| asset.image_metadata.as_ref())
    {
        *counts.entry(metadata.dimension_source.clone()).or_insert(0) += 1;
    }
    counts
}

/// Count static image URL references recorded on public image assets.
pub fn image_route_reference_count(assets: &[super::graph::SourceBuildAsset]) -> usize {
    assets
        .iter()
        .map(|asset| asset.referenced_by_routes.len())
        .sum()
}

/// Count static CSS URL references recorded on public image assets.
pub fn image_style_reference_count(assets: &[super::graph::SourceBuildAsset]) -> usize {
    assets
        .iter()
        .map(|asset| asset.referenced_by_styles.len())
        .sum()
}

/// Find route sources that reference a copied public image asset.
pub fn route_references_for_asset(
    project_root: &Path,
    asset_relative: &Path,
    routes: &[super::graph::SourceBuildRoute],
) -> Vec<SourceBuildImageRouteReference> {
    let candidate_urls = public_urls_for_asset(asset_relative);
    if candidate_urls.is_empty() {
        return Vec::new();
    }

    let mut references = Vec::new();
    for route in routes {
        let Ok(source) = std::fs::read_to_string(project_root.join(&route.path)) else {
            continue;
        };
        for specifier in &candidate_urls {
            if source.contains(specifier) {
                references.push(SourceBuildImageRouteReference {
                    route: route.route.clone(),
                    source_path: route.path.clone(),
                    specifier: specifier.clone(),
                    kind: "static-image-url".to_string(),
                });
            }
        }
    }

    references.sort_by(|left, right| {
        (
            left.route.as_str(),
            left.source_path.as_str(),
            left.specifier.as_str(),
        )
            .cmp(&(
                right.route.as_str(),
                right.source_path.as_str(),
                right.specifier.as_str(),
            ))
    });
    references.dedup_by(|left, right| {
        left.route == right.route
            && left.source_path == right.source_path
            && left.specifier == right.specifier
    });
    references
}

/// Find stylesheet sources that reference a copied public image asset.
pub fn style_references_for_asset(
    asset_relative: &Path,
    styles: &[super::graph::SourceBuildStyle],
) -> Vec<SourceBuildImageStyleReference> {
    let asset_path = asset_relative.to_string_lossy().replace('\\', "/");
    let mut references = Vec::new();

    for style in styles {
        for reference in &style.asset_references {
            if reference.path == asset_path {
                references.push(SourceBuildImageStyleReference {
                    style_path: style.path.clone(),
                    specifier: reference.specifier.clone(),
                    kind: reference.kind.clone(),
                    node_modules_required: reference.node_modules_required,
                });
            }
        }
    }

    references.sort_by(|left, right| {
        (
            left.style_path.as_str(),
            left.specifier.as_str(),
            left.kind.as_str(),
        )
            .cmp(&(
                right.style_path.as_str(),
                right.specifier.as_str(),
                right.kind.as_str(),
            ))
    });
    references.dedup_by(|left, right| {
        left.style_path == right.style_path
            && left.specifier == right.specifier
            && left.kind == right.kind
    });
    references
}

fn image_optimization(
    format: &str,
    width: Option<u32>,
    height: Option<u32>,
) -> SourceBuildImageOptimization {
    if format == "svg" {
        if let (Some(width), Some(height)) = (width, height) {
            if width > 0 && height > 0 {
                let placeholder = svg_placeholder_data_url(width, height);
                return SourceBuildImageOptimization {
                    status: "metadata-plus-svg-placeholder".to_string(),
                    variants_emitted: 0,
                    optimizer_invoked: false,
                    resize_emitted: false,
                    encoding_emitted: false,
                    blur_placeholder_emitted: false,
                    placeholder: Some(placeholder),
                    boundary: "metadata-plus-svg-placeholder-no-resize-or-encoding".to_string(),
                    informed_by: vec![
                        "turbopack-image::process::get_meta_data".to_string(),
                        "turbopack-image::process::svg".to_string(),
                    ],
                };
            }
        }
    }

    if raster_placeholder_format(format) {
        if let (Some(width), Some(height)) = (width, height) {
            if width > 0 && height > 0 {
                let placeholder = raster_placeholder_data_url(width, height);
                return SourceBuildImageOptimization {
                    status: "metadata-plus-raster-svg-placeholder".to_string(),
                    variants_emitted: 0,
                    optimizer_invoked: false,
                    resize_emitted: false,
                    encoding_emitted: false,
                    blur_placeholder_emitted: false,
                    placeholder: Some(placeholder),
                    boundary: "metadata-plus-raster-svg-placeholder-no-resize-or-encoding"
                        .to_string(),
                    informed_by: vec!["turbopack-image::process::get_meta_data".to_string()],
                };
            }
        }
    }

    metadata_only_optimization()
}

fn metadata_only_optimization() -> SourceBuildImageOptimization {
    SourceBuildImageOptimization {
        status: "metadata-only".to_string(),
        variants_emitted: 0,
        optimizer_invoked: false,
        resize_emitted: false,
        encoding_emitted: false,
        blur_placeholder_emitted: false,
        placeholder: None,
        boundary: "metadata-only-no-resize-encoding-or-placeholder-generation".to_string(),
        informed_by: vec!["turbopack-image::process::get_meta_data".to_string()],
    }
}

fn raster_placeholder_format(format: &str) -> bool {
    matches!(
        format,
        "png" | "jpg" | "jpeg" | "gif" | "webp" | "avif" | "ico"
    )
}

struct ImageDimensionEvidence {
    width: Option<u32>,
    height: Option<u32>,
    source: &'static str,
}

impl ImageDimensionEvidence {
    fn new(width: Option<u32>, height: Option<u32>, source: &'static str) -> Self {
        Self {
            width,
            height,
            source,
        }
    }

    fn format_only() -> Self {
        Self::new(None, None, "format-only-no-dimensions")
    }
}

fn public_urls_for_asset(asset_relative: &Path) -> Vec<String> {
    let normalized = asset_relative.to_string_lossy().replace('\\', "/");
    let Some(public_path) = normalized.strip_prefix("public/") else {
        return Vec::new();
    };

    let canonical = format!("/{public_path}");
    let public_prefixed = format!("/{normalized}");
    if canonical == public_prefixed {
        vec![canonical]
    } else {
        vec![canonical, public_prefixed]
    }
}

fn image_format(path: &Path) -> Option<String> {
    let extension = path.extension()?.to_str()?.to_ascii_lowercase();
    match extension.as_str() {
        "svg" | "png" | "jpg" | "jpeg" | "gif" | "webp" | "avif" | "ico" => Some(extension),
        _ => None,
    }
}

fn mime_type(format: &str) -> &'static str {
    match format {
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "avif" => "image/avif",
        "ico" => "image/x-icon",
        _ => "application/octet-stream",
    }
}

fn dimensions_for_format(format: &str, bytes: &[u8]) -> ImageDimensionEvidence {
    match format {
        "svg" => svg_dimensions(bytes),
        "png" => png_dimensions(bytes),
        "gif" => gif_dimensions(bytes),
        "jpg" | "jpeg" => jpeg_dimensions(bytes),
        "webp" => webp_dimensions(bytes),
        "avif" => avif_dimensions(bytes),
        "ico" => ico_dimensions(bytes),
        _ => ImageDimensionEvidence::format_only(),
    }
}

fn svg_dimensions(bytes: &[u8]) -> ImageDimensionEvidence {
    let Ok(source) = std::str::from_utf8(bytes) else {
        return ImageDimensionEvidence::format_only();
    };
    let Some(root_start) = source.find("<svg") else {
        return ImageDimensionEvidence::format_only();
    };
    let root = &source[root_start..];
    let Some(root_end) = root.find('>') else {
        return ImageDimensionEvidence::format_only();
    };
    let root = &root[..root_end];
    let width = svg_attribute(root, "width").and_then(parse_svg_length);
    let height = svg_attribute(root, "height").and_then(parse_svg_length);
    if width.is_some() && height.is_some() {
        return ImageDimensionEvidence::new(width, height, "svg-root-attributes");
    }

    let view_box = svg_attribute(root, "viewBox").and_then(parse_view_box);
    match (view_box, width, height) {
        (Some((view_width, view_height)), Some(width), None) if view_height > 0.0 => {
            let ratio = view_width / view_height;
            ImageDimensionEvidence::new(
                Some(width),
                rounded_dimension(width as f64 / ratio),
                "svg-viewbox-aspect-ratio",
            )
        }
        (Some((view_width, view_height)), None, Some(height)) if view_height > 0.0 => {
            let ratio = view_width / view_height;
            ImageDimensionEvidence::new(
                rounded_dimension(height as f64 * ratio),
                Some(height),
                "svg-viewbox-aspect-ratio",
            )
        }
        (Some((view_width, view_height)), None, None) => ImageDimensionEvidence::new(
            rounded_dimension(view_width),
            rounded_dimension(view_height),
            "svg-viewbox",
        ),
        _ if width.is_some() || height.is_some() => {
            ImageDimensionEvidence::new(width, height, "svg-root-partial-attributes")
        }
        _ => ImageDimensionEvidence::format_only(),
    }
}

fn svg_attribute(root: &str, name: &str) -> Option<String> {
    for quote in ['"', '\''] {
        let marker = format!("{name}={quote}");
        let Some(start) = root.find(&marker) else {
            continue;
        };
        let value_start = start + marker.len();
        let value_end = root[value_start..].find(quote)? + value_start;
        return Some(root[value_start..value_end].to_string());
    }
    None
}

fn parse_svg_length(value: String) -> Option<u32> {
    let value = value.trim();
    if value.ends_with('%') {
        return None;
    }
    let mut number_part = value;
    let mut unit = "";
    for candidate in ["px", "in", "cm", "mm", "pt", "pc", "em", "ex", "m"] {
        if let Some(stripped) = value.strip_suffix(candidate) {
            number_part = stripped.trim();
            unit = candidate;
            break;
        }
    }
    let number = number_part.parse::<f64>().ok()?;
    let scale = match unit {
        "" | "px" => 1.0,
        "in" => 96.0,
        "cm" => 96.0 / 2.54,
        "mm" => 96.0 / 25.4,
        "pt" => 96.0 / 72.0,
        "pc" => 16.0,
        "em" => 16.0,
        "ex" => 8.0,
        "m" => 96.0 / 2.54 * 100.0,
        _ => return None,
    };
    rounded_dimension(number * scale)
}

fn parse_view_box(value: String) -> Option<(f64, f64)> {
    let parts = value
        .split(|character: char| character.is_ascii_whitespace() || character == ',')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();
    if parts.len() != 4 {
        return None;
    }
    let width = parts[2].parse::<f64>().ok()?;
    let height = parts[3].parse::<f64>().ok()?;
    Some((width, height))
}

fn png_dimensions(bytes: &[u8]) -> ImageDimensionEvidence {
    const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";
    if bytes.len() < 24 || &bytes[..8] != PNG_SIGNATURE {
        return ImageDimensionEvidence::format_only();
    }
    ImageDimensionEvidence::new(
        Some(u32::from_be_bytes([
            bytes[16], bytes[17], bytes[18], bytes[19],
        ])),
        Some(u32::from_be_bytes([
            bytes[20], bytes[21], bytes[22], bytes[23],
        ])),
        "png-ihdr",
    )
}

fn gif_dimensions(bytes: &[u8]) -> ImageDimensionEvidence {
    if bytes.len() < 10 || (!bytes.starts_with(b"GIF87a") && !bytes.starts_with(b"GIF89a")) {
        return ImageDimensionEvidence::format_only();
    }
    let width = u16::from_le_bytes([bytes[6], bytes[7]]) as u32;
    let height = u16::from_le_bytes([bytes[8], bytes[9]]) as u32;
    ImageDimensionEvidence::new(Some(width), Some(height), "gif-logical-screen")
}

fn jpeg_dimensions(bytes: &[u8]) -> ImageDimensionEvidence {
    if bytes.len() < 4 || bytes[0] != 0xff || bytes[1] != 0xd8 {
        return ImageDimensionEvidence::format_only();
    }

    let mut index = 2usize;
    while index + 9 < bytes.len() {
        if bytes[index] != 0xff {
            index += 1;
            continue;
        }
        while index < bytes.len() && bytes[index] == 0xff {
            index += 1;
        }
        if index >= bytes.len() {
            break;
        }
        let marker = bytes[index];
        index += 1;
        if marker == 0xd9 || marker == 0xda {
            break;
        }
        if index + 2 > bytes.len() {
            break;
        }
        let segment_length = u16::from_be_bytes([bytes[index], bytes[index + 1]]) as usize;
        if segment_length < 2 || index + segment_length > bytes.len() {
            break;
        }
        if is_jpeg_start_of_frame(marker) && segment_length >= 7 {
            let height = u16::from_be_bytes([bytes[index + 3], bytes[index + 4]]) as u32;
            let width = u16::from_be_bytes([bytes[index + 5], bytes[index + 6]]) as u32;
            return ImageDimensionEvidence::new(Some(width), Some(height), "jpeg-sof");
        }
        index += segment_length;
    }

    ImageDimensionEvidence::format_only()
}

fn webp_dimensions(bytes: &[u8]) -> ImageDimensionEvidence {
    if bytes.len() < 20 || &bytes[..4] != b"RIFF" || &bytes[8..12] != b"WEBP" {
        return ImageDimensionEvidence::format_only();
    }

    match &bytes[12..16] {
        b"VP8X" if bytes.len() >= 30 => {
            let width = read_u24_le(&bytes[24..27]) + 1;
            let height = read_u24_le(&bytes[27..30]) + 1;
            ImageDimensionEvidence::new(Some(width), Some(height), "webp-vp8x")
        }
        b"VP8L" if bytes.len() >= 25 && bytes[20] == 0x2f => {
            let width = ((bytes[22] as u32 & 0x3f) << 8) | bytes[21] as u32;
            let height = ((bytes[24] as u32 & 0x0f) << 10)
                | ((bytes[23] as u32) << 2)
                | ((bytes[22] as u32 & 0xc0) >> 6);
            ImageDimensionEvidence::new(Some(width + 1), Some(height + 1), "webp-vp8l")
        }
        b"VP8 " if bytes.len() >= 30 && bytes[23..26] == [0x9d, 0x01, 0x2a] => {
            let width = u16::from_le_bytes([bytes[26], bytes[27]]) as u32 & 0x3fff;
            let height = u16::from_le_bytes([bytes[28], bytes[29]]) as u32 & 0x3fff;
            ImageDimensionEvidence::new(Some(width), Some(height), "webp-vp8")
        }
        _ => ImageDimensionEvidence::format_only(),
    }
}

fn avif_dimensions(bytes: &[u8]) -> ImageDimensionEvidence {
    let Some((width, height)) = avif_dimensions_from_bmff(bytes) else {
        return ImageDimensionEvidence::format_only();
    };
    ImageDimensionEvidence::new(Some(width), Some(height), "avif-ispe")
}

fn ico_dimensions(bytes: &[u8]) -> ImageDimensionEvidence {
    const ICON_DIR_BYTES: usize = 6;
    const ICON_DIR_ENTRY_BYTES: usize = 16;
    const FIRST_ENTRY: usize = ICON_DIR_BYTES;
    const MIN_ICO_BYTES: usize = ICON_DIR_BYTES + ICON_DIR_ENTRY_BYTES;

    if bytes.len() < MIN_ICO_BYTES {
        return ImageDimensionEvidence::format_only();
    }

    let reserved = u16::from_le_bytes([bytes[0], bytes[1]]);
    let image_type = u16::from_le_bytes([bytes[2], bytes[3]]);
    let image_count = u16::from_le_bytes([bytes[4], bytes[5]]);
    if reserved != 0 || !matches!(image_type, 1 | 2) || image_count == 0 {
        return ImageDimensionEvidence::format_only();
    }
    let Some(directory_bytes) = (image_count as usize)
        .checked_mul(ICON_DIR_ENTRY_BYTES)
        .and_then(|entries| ICON_DIR_BYTES.checked_add(entries))
    else {
        return ImageDimensionEvidence::format_only();
    };
    if bytes.len() < directory_bytes {
        return ImageDimensionEvidence::format_only();
    }

    let payload_size = read_u32_le(&bytes[FIRST_ENTRY + 8..FIRST_ENTRY + 12]);
    let payload_offset = read_u32_le(&bytes[FIRST_ENTRY + 12..FIRST_ENTRY + 16]);
    let Some(payload_end) = payload_offset.checked_add(payload_size) else {
        return ImageDimensionEvidence::format_only();
    };
    if payload_size == 0
        || payload_offset < directory_bytes as u32
        || payload_end as usize > bytes.len()
    {
        return ImageDimensionEvidence::format_only();
    }

    let width = ico_dimension_byte(bytes[FIRST_ENTRY]);
    let height = ico_dimension_byte(bytes[FIRST_ENTRY + 1]);
    ImageDimensionEvidence::new(Some(width), Some(height), "ico-directory-entry")
}

fn ico_dimension_byte(value: u8) -> u32 {
    if value == 0 { 256 } else { value as u32 }
}

fn is_jpeg_start_of_frame(marker: u8) -> bool {
    matches!(
        marker,
        0xc0 | 0xc1 | 0xc2 | 0xc3 | 0xc5 | 0xc6 | 0xc7 | 0xc9 | 0xca | 0xcb | 0xcd | 0xce | 0xcf
    )
}

fn rounded_dimension(value: f64) -> Option<u32> {
    if !value.is_finite() || value < 0.0 {
        return None;
    }
    Some(value.round() as u32)
}

fn read_u24_le(bytes: &[u8]) -> u32 {
    bytes[0] as u32 | ((bytes[1] as u32) << 8) | ((bytes[2] as u32) << 16)
}

fn read_u32_le(bytes: &[u8]) -> u32 {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::image_metadata_for_asset;

    #[test]
    fn records_webp_vp8x_dimensions_with_raster_placeholder_without_optimizer() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"RIFF");
        bytes.extend_from_slice(&[22, 0, 0, 0]);
        bytes.extend_from_slice(b"WEBP");
        bytes.extend_from_slice(b"VP8X");
        bytes.extend_from_slice(&[10, 0, 0, 0]);
        bytes.push(0);
        bytes.extend_from_slice(&[0, 0, 0]);
        bytes.extend_from_slice(&[0x7f, 0x02, 0x00]);
        bytes.extend_from_slice(&[0x67, 0x01, 0x00]);

        let metadata = image_metadata_for_asset(Path::new("public/images/hero.webp"), &bytes)
            .expect("webp metadata");

        assert_eq!(metadata.format, "webp");
        assert_eq!(metadata.mime_type, "image/webp");
        assert_eq!(metadata.width, Some(640));
        assert_eq!(metadata.height, Some(360));
        assert_eq!(metadata.dimension_source, "webp-vp8x");
        assert_eq!(
            metadata.optimization.status,
            "metadata-plus-raster-svg-placeholder"
        );
        assert_eq!(metadata.optimization.variants_emitted, 0);
        assert!(!metadata.optimization.optimizer_invoked);
        assert_eq!(
            metadata
                .optimization
                .placeholder
                .as_ref()
                .expect("webp placeholder")
                .kind,
            "raster-svg-placeholder-artifact"
        );
        assert!(
            !metadata
                .optimization
                .informed_by
                .iter()
                .any(|reference| reference == "turbopack-image::process::optimize")
        );
    }
}
