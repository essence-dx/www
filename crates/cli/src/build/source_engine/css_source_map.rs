use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::error::DxResult;

use super::graph::{hash_bytes, normalize_path, write_file};

pub struct CssSourceMapSource<'a> {
    pub path: &'a Path,
    pub source: &'a str,
    pub role: CssSourceMapSourceRole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssSourceMapSourceRole {
    EntryStyle,
    FlattenedImport,
    RetainedImport,
}

impl CssSourceMapSourceRole {
    fn as_str(self) -> &'static str {
        match self {
            Self::EntryStyle => "entry-style",
            Self::FlattenedImport => "flattened-import",
            Self::RetainedImport => "retained-import",
        }
    }
}

pub struct CssSourceMapOutput {
    pub path: PathBuf,
    pub hash: String,
    pub source_hash_count: usize,
    pub entry_style_source_count: usize,
    pub flattened_import_source_count: usize,
    pub retained_import_source_count: usize,
    pub segment_count: usize,
    pub exact_segment_mapping: bool,
    pub evidence_only: bool,
}

#[derive(Serialize)]
struct DxCssSourceMap {
    version: u8,
    file: String,
    sources: Vec<String>,
    #[serde(rename = "sourcesContent")]
    sources_content: Vec<String>,
    names: Vec<String>,
    mappings: String,
    x_dx_css_pipeline: DxCssSourceMapMetadata,
}

#[derive(Serialize)]
struct DxCssSourceMapSourceHash {
    source: String,
    role: &'static str,
    hash: String,
}

#[derive(Serialize)]
struct DxCssSourceMapMetadata {
    owner: &'static str,
    generated_by: &'static str,
    reference: &'static str,
    mapping_status: &'static str,
    segment_mapping_available: bool,
    segment_count: usize,
    source_evidence_only: bool,
    parity_status: &'static str,
    runtime_boundary: &'static str,
    requires_node_modules: bool,
    full_lightning_css_parity: bool,
    turbopack_public_architecture: bool,
    source_count: usize,
    source_hashes: Vec<DxCssSourceMapSourceHash>,
}

pub fn write_css_source_map(
    output_dir: &Path,
    css_relative_path: &Path,
    sources: &[CssSourceMapSource<'_>],
) -> DxResult<CssSourceMapOutput> {
    let mut source_map_path = output_dir.join(css_relative_path);
    source_map_path.set_extension("css.map");
    let source_hashes = sources
        .iter()
        .map(|source| DxCssSourceMapSourceHash {
            source: normalize_path(source.path),
            role: source.role.as_str(),
            hash: hash_bytes(source.source.as_bytes()),
        })
        .collect::<Vec<_>>();
    let source_hash_count = source_hashes.len();
    let entry_style_source_count = sources
        .iter()
        .filter(|source| source.role == CssSourceMapSourceRole::EntryStyle)
        .count();
    let flattened_import_source_count = sources
        .iter()
        .filter(|source| source.role == CssSourceMapSourceRole::FlattenedImport)
        .count();
    let retained_import_source_count = sources
        .iter()
        .filter(|source| source.role == CssSourceMapSourceRole::RetainedImport)
        .count();

    let source_map = DxCssSourceMap {
        version: 3,
        file: normalize_path(css_relative_path),
        sources: sources
            .iter()
            .map(|source| normalize_path(source.path))
            .collect(),
        sources_content: sources
            .iter()
            .map(|source| source.source.to_string())
            .collect(),
        names: Vec::new(),
        mappings: String::new(),
        x_dx_css_pipeline: DxCssSourceMapMetadata {
            owner: "dx-style",
            generated_by: "dx-source-css-adapter",
            reference: "turbopack-css-source-map-asset",
            mapping_status: "source-list-hash-evidence-no-segments",
            segment_mapping_available: false,
            segment_count: 0,
            source_evidence_only: true,
            parity_status: "compatibility-evidence-only",
            runtime_boundary: "dx-style-owned-no-next-runtime",
            requires_node_modules: false,
            full_lightning_css_parity: false,
            turbopack_public_architecture: false,
            source_count: sources.len(),
            source_hashes,
        },
    };
    let json = serde_json::to_vec_pretty(&source_map)?;
    let hash = hash_bytes(&json);
    write_file(&source_map_path, &json)?;
    Ok(CssSourceMapOutput {
        path: source_map_path,
        hash,
        source_hash_count,
        entry_style_source_count,
        flattened_import_source_count,
        retained_import_source_count,
        segment_count: 0,
        exact_segment_mapping: false,
        evidence_only: true,
    })
}
