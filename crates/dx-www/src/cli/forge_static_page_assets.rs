use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct DxForgeStaticPageAssetManifest {
    pub(super) path: PathBuf,
    pub(super) project_relative_path: String,
    pub(super) write_state: String,
    pub(super) asset_count: u64,
    pub(super) resolved_asset_count: u64,
    pub(super) unresolved_media_gap_count: u64,
    pub(super) assets: Vec<DxForgeStaticPageAsset>,
    pub(super) unresolved_media_gaps: Vec<DxForgeStaticPageMediaGap>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct DxForgeStaticPageAsset {
    pub(super) kind: String,
    pub(super) source_url: String,
    pub(super) copied_target_path: String,
    pub(super) source_file_path: Option<PathBuf>,
    pub(super) hash: Option<String>,
    pub(super) byte_size: Option<u64>,
    pub(super) cache_hint: String,
    pub(super) alt_text_review_state: String,
    pub(super) unresolved: bool,
    pub(super) note: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct DxForgeStaticPageMediaGap {
    pub(super) source_url: String,
    pub(super) copied_target_path: String,
    pub(super) reason: String,
    pub(super) alt_text_review_state: String,
}

pub(super) fn extract_static_page_assets(
    text: &str,
    slug: &str,
    export_root: &Path,
    selected_file_dir: &Path,
) -> Vec<DxForgeStaticPageAsset> {
    let mut assets = Vec::new();
    let mut seen = BTreeSet::new();
    let Ok(tag_regex) = Regex::new(r#"(?is)<([a-z0-9:-]+)\b([^>]*)>"#) else {
        return assets;
    };

    for capture in tag_regex.captures_iter(text) {
        let Some(tag) = capture
            .get(1)
            .map(|value| value.as_str().to_ascii_lowercase())
        else {
            continue;
        };
        let attrs = capture.get(2).map(|value| value.as_str()).unwrap_or("");
        let Some((kind, source)) = asset_source_from_tag(&tag, attrs) else {
            continue;
        };
        if !is_copyable_static_asset_source(&source) || !seen.insert(source.clone()) {
            continue;
        }

        let copied_target_path = migrated_asset_target(slug, &source, assets.len() + 1);
        let local_path = local_asset_candidate(export_root, selected_file_dir, &source);
        let local_bytes = local_path
            .as_ref()
            .filter(|path| path.is_file())
            .and_then(|path| fs::read(path).ok());
        let (source_file_path, hash, byte_size, unresolved) = if let Some(bytes) = local_bytes {
            let path = local_path.expect("local path exists when bytes are present");
            (
                Some(path),
                Some(format!("blake3:{}", blake3::hash(&bytes).to_hex())),
                Some(bytes.len() as u64),
                false,
            )
        } else {
            (None, None, None, true)
        };

        assets.push(DxForgeStaticPageAsset {
            kind,
            source_url: source,
            copied_target_path,
            source_file_path,
            hash,
            byte_size,
            cache_hint: "copy-optimize-cache-long".to_string(),
            alt_text_review_state: alt_text_review_state(&tag, attrs),
            unresolved,
            note: "Copy the original asset into the target path, optimize it, and verify alt text before shipping.".to_string(),
        });
    }

    assets
}

pub(super) fn build_static_page_asset_manifest(
    project: &Path,
    path: &Path,
    write_state: &str,
    assets: &[DxForgeStaticPageAsset],
) -> DxForgeStaticPageAssetManifest {
    let unresolved_media_gaps = assets
        .iter()
        .filter(|asset| asset.unresolved)
        .map(|asset| DxForgeStaticPageMediaGap {
            source_url: asset.source_url.clone(),
            copied_target_path: asset.copied_target_path.clone(),
            reason: "source asset file was not found in the export folder".to_string(),
            alt_text_review_state: asset.alt_text_review_state.clone(),
        })
        .collect::<Vec<_>>();
    let resolved_asset_count = assets.iter().filter(|asset| !asset.unresolved).count() as u64;

    DxForgeStaticPageAssetManifest {
        path: path.to_path_buf(),
        project_relative_path: project_relative_path(project, path),
        write_state: write_state.to_string(),
        asset_count: assets.len() as u64,
        resolved_asset_count,
        unresolved_media_gap_count: unresolved_media_gaps.len() as u64,
        assets: assets.to_vec(),
        unresolved_media_gaps,
    }
}

pub(super) fn static_page_asset_manifest_json(
    manifest: &DxForgeStaticPageAssetManifest,
) -> anyhow::Result<String> {
    serde_json::to_string_pretty(manifest).context("serialize static page asset manifest")
}

fn asset_source_from_tag(tag: &str, attrs: &str) -> Option<(String, String)> {
    let source = attribute_value(attrs, "src").or_else(|| attribute_value(attrs, "href"))?;
    let kind = match tag {
        "img" | "picture" | "source" => "image",
        "script" => "script",
        "link" => "linked-resource",
        "video" | "audio" | "track" => "media",
        "iframe" | "embed" | "object" => "embed",
        other => other,
    };
    Some((kind.to_string(), source))
}

fn attribute_value(attrs: &str, name: &str) -> Option<String> {
    let escaped = regex::escape(name);
    let double_quoted = format!(r#"(?is)\b{escaped}\s*=\s*"([^"]*)""#);
    let single_quoted = format!(r#"(?is)\b{escaped}\s*=\s*'([^']*)'"#);
    Regex::new(&double_quoted)
        .ok()
        .and_then(|regex| regex.captures(attrs))
        .and_then(|captures| {
            captures
                .get(1)
                .map(|value| value.as_str().trim().to_string())
        })
        .or_else(|| {
            Regex::new(&single_quoted)
                .ok()
                .and_then(|regex| regex.captures(attrs))
                .and_then(|captures| {
                    captures
                        .get(1)
                        .map(|value| value.as_str().trim().to_string())
                })
        })
}

fn alt_text_review_state(tag: &str, attrs: &str) -> String {
    if tag != "img" {
        return "not-applicable".to_string();
    }
    match attribute_value(attrs, "alt") {
        Some(value) if !value.trim().is_empty() => "present-review-required".to_string(),
        Some(_) => "empty".to_string(),
        None => "missing".to_string(),
    }
}

fn is_copyable_static_asset_source(source: &str) -> bool {
    let lower = source.to_ascii_lowercase();
    !(source.is_empty()
        || source.starts_with('#')
        || lower.starts_with("mailto:")
        || lower.starts_with("tel:")
        || lower.starts_with("javascript:")
        || lower.starts_with("data:"))
}

fn local_asset_candidate(
    export_root: &Path,
    selected_file_dir: &Path,
    source: &str,
) -> Option<PathBuf> {
    let path_part = asset_source_path_part(source)?;
    let without_leading = path_part.trim_start_matches('/');
    if path_part.starts_with('/') || source.contains("://") {
        Some(export_root.join(without_leading))
    } else {
        Some(selected_file_dir.join(without_leading))
    }
}

fn asset_source_path_part(source: &str) -> Option<String> {
    let without_fragment = source.split('#').next().unwrap_or(source);
    let without_query = without_fragment
        .split('?')
        .next()
        .unwrap_or(without_fragment);
    if without_query.trim().is_empty() {
        return None;
    }

    if let Some((_, after_scheme)) = without_query.split_once("://") {
        let path_start = after_scheme.find('/')?;
        return Some(after_scheme[path_start..].to_string());
    }

    Some(without_query.to_string())
}

fn migrated_asset_target(slug: &str, source: &str, index: usize) -> String {
    let without_query = source
        .split(['?', '#'])
        .next()
        .unwrap_or(source)
        .trim_end_matches('/');
    let raw_name = without_query
        .rsplit('/')
        .find(|part| !part.trim().is_empty())
        .unwrap_or("");
    let file_name = sanitize_asset_file_name(raw_name)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| format!("asset-{index}"));
    format!("/assets/migrated/{slug}/{file_name}")
}

fn sanitize_asset_file_name(value: &str) -> Option<String> {
    let mut output = String::new();
    let mut last_dash = false;
    for character in value.chars().flat_map(|character| character.to_lowercase()) {
        if character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-') {
            output.push(character);
            last_dash = false;
        } else if !last_dash {
            output.push('-');
            last_dash = true;
        }
    }
    let output = output.trim_matches('-').to_string();
    (!output.is_empty()).then_some(output)
}

fn project_relative_path(project: &Path, path: &Path) -> String {
    path.strip_prefix(project)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
