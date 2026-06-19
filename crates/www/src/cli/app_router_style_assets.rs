use std::{
    collections::HashSet,
    path::Path,
    sync::{Mutex, OnceLock},
};

use serde_json::json;

use super::Cli;
use super::app_page_routes;
use super::app_router_runtime_command::{
    app_route_cache_signature, cached_app_route_generated_style, compile_app_route_proof,
    has_cached_app_route_generated_styles, remember_app_route_generated_styles,
};
use super::app_segment_files;

static GENERATED_STYLE_MISS_CACHE: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

pub(super) struct DxAppRouterStyleAssetError {
    message: String,
    request_path: String,
    normalized_output_path: String,
    app_roots: Vec<String>,
    routes_scanned: usize,
    route_sources_scanned: Vec<String>,
    source_path: Option<String>,
}

impl DxAppRouterStyleAssetError {
    pub(super) fn to_response_body(&self) -> String {
        json!({
            "error": "generated-style-not-found",
            "message": self.message,
            "request_path": self.request_path,
            "normalized_output_path": self.normalized_output_path,
            "app_roots": self.app_roots,
            "routes_scanned": self.routes_scanned,
            "route_sources_scanned": self.route_sources_scanned,
            "source_path": self.source_path,
            "source_owned_contract": true,
            "node_modules_required": false,
            "lifecycle_scripts_executed": false
        })
        .to_string()
    }
}

pub(super) fn render_generated_style_asset(
    cwd: &Path,
    request_path: &str,
) -> Result<String, Box<DxAppRouterStyleAssetError>> {
    let normalized_output_path = Cli::dev_lookup_path(request_path)
        .trim_start_matches('/')
        .replace('\\', "/");
    let miss_cache_key = style_asset_miss_cache_key(cwd, &normalized_output_path);

    if let Some(css) = cached_app_route_generated_style(cwd, &normalized_output_path) {
        return Ok(css);
    }

    if has_cached_app_route_generated_styles(cwd) {
        let _ = super::app_router_runtime_command::prewarm_app_route_generated_styles(cwd);
        if let Some(css) = cached_app_route_generated_style(cwd, &normalized_output_path) {
            return Ok(css);
        }
    }

    if generated_style_miss_cache()
        .lock()
        .ok()
        .is_some_and(|cache| cache.contains(&miss_cache_key))
    {
        return Err(Box::new(style_asset_error(
            request_path,
            &normalized_output_path,
            Vec::new(),
            0,
            Vec::new(),
            None,
            format!("No source-owned App Router generated style asset matched {request_path}."),
        )));
    }

    let app_roots = app_segment_files::app_route_roots(cwd);
    let app_root_labels = app_roots
        .iter()
        .map(|root| Cli::relative_cli_path(cwd, root))
        .collect::<Vec<_>>();

    if app_roots.is_empty() {
        return Err(Box::new(style_asset_error(
            request_path,
            &normalized_output_path,
            app_root_labels,
            0,
            Vec::new(),
            None,
            "No App Router source root was found; expected app or src/app before serving generated DX style assets.",
        )));
    }

    let mut routes_scanned = 0;
    let mut route_sources_scanned = Vec::new();
    for app_dir in app_roots {
        for entry in walkdir::WalkDir::new(&app_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| {
                entry.file_type().is_file()
                    && entry
                        .file_name()
                        .to_str()
                        .is_some_and(app_segment_files::is_app_page_file_name)
                    && entry
                        .path()
                        .components()
                        .all(|component| component.as_os_str().to_string_lossy() != "node_modules")
                    && {
                        let relative = Cli::relative_cli_path(cwd, entry.path());
                        app_page_routes::route_path_from_page_source_path(&relative).is_some()
                    }
            })
        {
            routes_scanned += 1;
            let source_path = Cli::relative_cli_path(cwd, entry.path());
            route_sources_scanned.push(source_path.clone());
            let proof = compile_app_route_proof(cwd, entry.path()).map_err(|error| {
                Box::new(style_asset_error(
                    request_path,
                    &normalized_output_path,
                    app_root_labels.clone(),
                    routes_scanned,
                    route_sources_scanned.clone(),
                    Some(source_path.clone()),
                    format!(
                        "Failed to compile App Router source while resolving generated DX style asset {request_path}: {error}"
                    ),
                ))
            })?;
            remember_app_route_generated_styles(cwd, &proof);
            if let Some(asset) = proof
                .generated_styles
                .into_iter()
                .find(|asset| asset.output_path == normalized_output_path)
            {
                return Ok(asset.css);
            }
        }
    }

    remember_generated_style_miss(miss_cache_key);
    Err(Box::new(style_asset_error(
        request_path,
        &normalized_output_path,
        app_root_labels,
        routes_scanned,
        route_sources_scanned,
        None,
        format!("No source-owned App Router generated style asset matched {request_path}."),
    )))
}

fn generated_style_miss_cache() -> &'static Mutex<HashSet<String>> {
    GENERATED_STYLE_MISS_CACHE.get_or_init(|| Mutex::new(HashSet::new()))
}

fn remember_generated_style_miss(cache_key: String) {
    if let Ok(mut cache) = generated_style_miss_cache().lock() {
        if cache.len() > 256 {
            cache.clear();
        }
        cache.insert(cache_key);
    }
}

fn style_asset_miss_cache_key(cwd: &Path, normalized_output_path: &str) -> String {
    let cwd_key = cwd
        .canonicalize()
        .unwrap_or_else(|_| cwd.to_path_buf())
        .to_string_lossy()
        .replace('\\', "/");
    format!(
        "{cwd_key}\n{}\n{normalized_output_path}",
        app_route_cache_signature(cwd)
    )
}

fn style_asset_error(
    request_path: &str,
    normalized_output_path: &str,
    app_roots: Vec<String>,
    routes_scanned: usize,
    route_sources_scanned: Vec<String>,
    source_path: Option<String>,
    message: impl Into<String>,
) -> DxAppRouterStyleAssetError {
    DxAppRouterStyleAssetError {
        message: message.into(),
        request_path: request_path.to_string(),
        normalized_output_path: normalized_output_path.to_string(),
        app_roots,
        routes_scanned,
        route_sources_scanned,
        source_path,
    }
}
