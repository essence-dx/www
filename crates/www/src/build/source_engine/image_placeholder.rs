use std::{
    fmt::Write as _,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::error::{DxError, DxResult};

/// Source-owned image placeholder emitted into receipts and manifests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildImagePlaceholder {
    /// Placeholder encoding contract.
    pub kind: String,
    /// Data URL payload that consumers can render without another generated file.
    pub data_url: String,
    /// Placeholder width in CSS pixels.
    pub width: u32,
    /// Placeholder height in CSS pixels.
    pub height: u32,
    /// Data URL byte length.
    pub bytes: usize,
    /// DX-owned producer for this placeholder.
    pub source: String,
    /// Project-relative emitted placeholder artifact path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// Stable BLAKE3 hash of the emitted placeholder artifact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    /// Emitted placeholder artifact byte count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_bytes: Option<usize>,
}

pub(super) fn write_image_placeholder_artifact(
    project_root: &Path,
    output_dir: &Path,
    asset_relative: &Path,
    placeholder: &mut SourceBuildImagePlaceholder,
) -> DxResult<()> {
    if !matches!(
        placeholder.kind.as_str(),
        "svg-placeholder-data-url" | "raster-svg-placeholder-artifact"
    ) {
        return Ok(());
    }

    let bytes = svg_placeholder_bytes(placeholder.width, placeholder.height);
    let hash = placeholder_hash(&bytes);
    let output = placeholder_output_path(output_dir, asset_relative, &hash);
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent).map_err(|error| DxError::IoError {
            path: Some(parent.to_path_buf()),
            message: error.to_string(),
        })?;
    }
    std::fs::write(&output, &bytes).map_err(|error| DxError::IoError {
        path: Some(output.clone()),
        message: error.to_string(),
    })?;

    placeholder.output = Some(normalized_relative_path(project_root, &output));
    placeholder.hash = Some(hash);
    placeholder.artifact_bytes = Some(bytes.len());
    Ok(())
}

pub(super) fn svg_placeholder_data_url(width: u32, height: u32) -> SourceBuildImagePlaceholder {
    let svg = svg_placeholder_markup(width, height);
    let data_url = format!("data:image/svg+xml,{}", percent_encode_svg(&svg));
    SourceBuildImagePlaceholder {
        kind: "svg-placeholder-data-url".to_string(),
        bytes: data_url.len(),
        data_url,
        width,
        height,
        source: "dx-source-owned-svg-placeholder".to_string(),
        output: None,
        hash: None,
        artifact_bytes: None,
    }
}

pub(super) fn raster_placeholder_data_url(width: u32, height: u32) -> SourceBuildImagePlaceholder {
    let svg = svg_placeholder_markup(width, height);
    let data_url = format!("data:image/svg+xml,{}", percent_encode_svg(&svg));
    SourceBuildImagePlaceholder {
        kind: "raster-svg-placeholder-artifact".to_string(),
        bytes: data_url.len(),
        data_url,
        width,
        height,
        source: "dx-source-owned-raster-svg-placeholder".to_string(),
        output: None,
        hash: None,
        artifact_bytes: None,
    }
}

fn svg_placeholder_bytes(width: u32, height: u32) -> Vec<u8> {
    svg_placeholder_markup(width, height).into_bytes()
}

fn svg_placeholder_markup(width: u32, height: u32) -> String {
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}" preserveAspectRatio="none"><rect width="100%" height="100%" fill="#f3f4f6"/></svg>"##
    )
}

fn percent_encode_svg(source: &str) -> String {
    let mut encoded = String::with_capacity(source.len());
    for byte in source.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char)
            }
            _ => {
                let _ = write!(&mut encoded, "%{byte:02X}");
            }
        }
    }
    encoded
}

fn placeholder_hash(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex()[..16].to_string()
}

fn placeholder_output_path(output_dir: &Path, asset_relative: &Path, hash: &str) -> PathBuf {
    let mut output = output_dir.join(".dx/build-cache/image-placeholders").join(asset_relative);
    let stem = output
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("image");
    output.set_file_name(format!("{stem}-{hash}.placeholder.svg"));
    output
}

fn normalized_relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
