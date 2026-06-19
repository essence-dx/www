use std::{
    collections::{BTreeSet, HashMap, HashSet},
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
    time::UNIX_EPOCH,
};

use dx_compiler::delivery::{
    DxReactAppRouteInput, DxReactAppRouteProof, DxReactAppSegmentSource, DxReactComponentSource,
    DxReactGeneratedStyleAsset, compile_react_app_route, parse_tsx_module,
};

use super::Cli;
use super::app_page_routes;
use super::app_router_execution::{
    DxAppRouterRequestValueMode, DxAppRouterRuntimeRenderInput, render_app_router_runtime,
};
use super::app_router_paths;
use super::app_segment_files;

#[derive(Debug, Clone)]
struct CachedAppRouteRender {
    cwd_key: String,
    signature: String,
    html: String,
    generated_styles: Vec<DxReactGeneratedStyleAsset>,
}

static APP_ROUTE_RENDER_CACHE: OnceLock<Mutex<HashMap<String, CachedAppRouteRender>>> =
    OnceLock::new();
static APP_ROUTE_STYLE_PREWARM_CACHE: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

pub(super) fn render_app_route(
    cwd: &PathBuf,
    app_route_match: &app_page_routes::AppRouteMatch,
) -> Result<String, String> {
    let signature = app_route_cache_signature(cwd);
    let cwd_key = app_route_cache_cwd_key(cwd);
    let route_path_key = Cli::relative_cli_path(cwd, &app_route_match.path);
    let cache_key = format!(
        "{cwd_key}\n{signature}\n{route_path_key}\n{:?}\n{:?}",
        app_route_match.params, app_route_match.search_params
    );

    if let Some(cached) = app_route_render_cache()
        .lock()
        .ok()
        .and_then(|cache| cache.get(&cache_key).cloned())
    {
        return Ok(cached.html);
    }

    let route = app_router_paths::route_from_app_path(cwd, &app_route_match.path);
    let route_source = std::fs::read_to_string(&app_route_match.path)
        .map_err(|error| format!("Failed to read app route: {error}"))?;
    let source_path = Cli::relative_cli_path(cwd, &app_route_match.path);
    let segments = react_app_segment_sources(cwd, &app_route_match.path);
    let source_manifest_hash = Cli::source_manifest_hash(cwd);
    let components = react_route_component_sources(cwd, &source_path, &route_source, &segments);
    let proof = compile_react_app_route(DxReactAppRouteInput {
        route: route.clone(),
        route_source_path: source_path.clone(),
        route_source: route_source.clone(),
        segments: segments.clone(),
        components,
        styles: Cli::react_style_sources(cwd),
        source_manifest_hash: source_manifest_hash.clone(),
    })
    .map_err(|error| format!("Failed to compile app route: {error}"))?;
    remember_app_route_generated_styles(cwd, &proof);

    let html = render_app_router_runtime(DxAppRouterRuntimeRenderInput {
        cwd,
        app_route_path: &app_route_match.path,
        route,
        source_path,
        route_source,
        segments,
        proof: &proof,
        source_manifest_hash,
        node_modules_present: cwd.join("node_modules").exists(),
        route_params: &app_route_match.params,
        search_params: &app_route_match.search_params,
        server_sources: &Cli::react_server_sources(cwd),
        request_value_mode: DxAppRouterRequestValueMode::Runtime,
    });

    remember_app_route_render(
        cache_key,
        CachedAppRouteRender {
            cwd_key,
            signature,
            html: html.clone(),
            generated_styles: proof.generated_styles.clone(),
        },
    );

    Ok(html)
}

pub(super) fn compile_app_route_proof(
    cwd: &Path,
    app_route_path: &Path,
) -> Result<DxReactAppRouteProof, String> {
    let route_source = std::fs::read_to_string(app_route_path)
        .map_err(|error| format!("Failed to read app route: {error}"))?;
    let route_source_path = Cli::relative_cli_path(cwd, app_route_path);
    let segments = react_app_segment_sources(cwd, app_route_path);
    let components =
        react_route_component_sources(cwd, &route_source_path, &route_source, &segments);
    compile_react_app_route(DxReactAppRouteInput {
        route: app_router_paths::route_from_app_path(cwd, app_route_path),
        route_source_path,
        route_source,
        segments,
        components,
        styles: Cli::react_style_sources(cwd),
        source_manifest_hash: Cli::source_manifest_hash(cwd),
    })
    .map_err(|error| format!("Failed to compile app route: {error}"))
}

pub(super) fn react_route_component_sources(
    cwd: &Path,
    route_source_path: &str,
    route_source: &str,
    segments: &[DxReactAppSegmentSource],
) -> Vec<DxReactComponentSource> {
    let all_components = Cli::react_component_sources(cwd);
    if all_components.is_empty() {
        return Vec::new();
    }

    let components_by_path = all_components
        .into_iter()
        .map(|component| (normalize_component_path(&component.source_path), component))
        .collect::<HashMap<_, _>>();
    let mut selected = BTreeSet::new();
    let mut queue = Vec::new();

    enqueue_component_imports(
        route_source_path,
        route_source,
        &components_by_path,
        &mut selected,
        &mut queue,
    );
    for segment in segments {
        enqueue_component_imports(
            &segment.source_path,
            &segment.source,
            &components_by_path,
            &mut selected,
            &mut queue,
        );
    }

    while let Some(source_path) = queue.pop() {
        let Some(component) = components_by_path.get(&source_path) else {
            continue;
        };
        enqueue_component_imports(
            &component.source_path,
            &component.source,
            &components_by_path,
            &mut selected,
            &mut queue,
        );
    }

    selected
        .into_iter()
        .filter_map(|source_path| components_by_path.get(&source_path).cloned())
        .collect()
}

fn enqueue_component_imports(
    importer_source_path: &str,
    source: &str,
    components_by_path: &HashMap<String, DxReactComponentSource>,
    selected: &mut BTreeSet<String>,
    queue: &mut Vec<String>,
) {
    for specifier in route_value_import_specifiers(importer_source_path, source) {
        let Some(source_path) =
            resolve_component_import(importer_source_path, &specifier, components_by_path)
        else {
            continue;
        };
        if selected.insert(source_path.clone()) {
            queue.push(source_path);
        }
    }
}

fn route_value_import_specifiers(source_path: &str, source: &str) -> Vec<String> {
    let mut specifiers = BTreeSet::new();
    for import in parse_tsx_module(source_path, source).imports {
        if import.type_only {
            continue;
        }
        let has_value_specifier = import.default.is_some()
            || import
                .specifiers
                .iter()
                .any(|specifier| !specifier.type_only);
        if has_value_specifier || import.specifiers.is_empty() {
            specifiers.insert(import.source);
        }
    }
    specifiers.into_iter().collect()
}

fn resolve_component_import(
    importer_source_path: &str,
    specifier: &str,
    components_by_path: &HashMap<String, DxReactComponentSource>,
) -> Option<String> {
    if !specifier.starts_with('.') && !specifier.starts_with("@/") {
        return None;
    }

    let base = if let Some(project_path) = specifier.strip_prefix("@/") {
        PathBuf::from(project_path)
    } else {
        let importer_dir = Path::new(importer_source_path)
            .parent()
            .unwrap_or_else(|| Path::new(""));
        importer_dir.join(specifier)
    };

    component_import_candidates(&base)
        .into_iter()
        .find(|candidate| components_by_path.contains_key(candidate))
}

fn component_import_candidates(base: &Path) -> Vec<String> {
    let mut candidates = Vec::new();
    if base.extension().is_some() {
        candidates.push(normalize_component_path(base));
        return candidates;
    }

    for extension in ["tsx", "jsx", "ts"] {
        candidates.push(normalize_component_path(base.with_extension(extension)));
    }
    for extension in ["tsx", "jsx", "ts"] {
        candidates.push(normalize_component_path(
            base.join("index").with_extension(extension),
        ));
    }
    candidates
}

fn normalize_component_path(path: impl AsRef<Path>) -> String {
    let mut segments = Vec::new();
    for component in path.as_ref().components() {
        match component {
            std::path::Component::Normal(segment) => {
                segments.push(segment.to_string_lossy().replace('\\', "/"));
            }
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                segments.pop();
            }
            std::path::Component::RootDir | std::path::Component::Prefix(_) => {}
        }
    }
    segments.join("/")
}

pub(super) fn remember_app_route_generated_styles(cwd: &Path, proof: &DxReactAppRouteProof) {
    let cwd_key = app_route_cache_cwd_key(cwd);
    let signature = app_route_cache_signature(cwd);
    remember_app_route_render(
        format!("{cwd_key}\n{signature}\n__styles_only__\n{}", proof.route),
        CachedAppRouteRender {
            cwd_key,
            signature,
            html: String::new(),
            generated_styles: proof.generated_styles.clone(),
        },
    );
}

pub(super) fn prewarm_app_route_generated_styles(cwd: &Path) -> Result<usize, String> {
    let cwd_key = app_route_cache_cwd_key(cwd);
    let signature = app_route_cache_signature(cwd);
    let prewarm_key = format!("{cwd_key}\n{signature}");
    if app_route_style_prewarm_cache()
        .lock()
        .ok()
        .is_some_and(|cache| cache.contains(&prewarm_key))
    {
        return Ok(0);
    }

    let mut warmed_asset_count = 0usize;
    for app_dir in app_segment_files::app_route_roots(cwd) {
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
            let proof = compile_app_route_proof(cwd, entry.path())?;
            warmed_asset_count = warmed_asset_count.saturating_add(proof.generated_styles.len());
            remember_app_route_generated_styles(cwd, &proof);
        }
    }

    if let Ok(mut cache) = app_route_style_prewarm_cache().lock() {
        if cache.len() > 64 {
            cache.clear();
        }
        cache.insert(prewarm_key);
    }

    Ok(warmed_asset_count)
}

pub(super) fn cached_app_route_generated_style(
    cwd: &Path,
    normalized_output_path: &str,
) -> Option<String> {
    let cwd_key = app_route_cache_cwd_key(cwd);
    let signature = app_route_cache_signature(cwd);
    app_route_render_cache().lock().ok().and_then(|cache| {
        cache.values().find_map(|cached| {
            if cached.cwd_key != cwd_key || cached.signature != signature {
                return None;
            }
            cached
                .generated_styles
                .iter()
                .find(|asset| asset.output_path == normalized_output_path)
                .map(|asset| asset.css.clone())
        })
    })
}

pub(super) fn has_cached_app_route_generated_styles(cwd: &Path) -> bool {
    let cwd_key = app_route_cache_cwd_key(cwd);
    let signature = app_route_cache_signature(cwd);
    app_route_render_cache().lock().ok().is_some_and(|cache| {
        cache.values().any(|cached| {
            cached.cwd_key == cwd_key
                && cached.signature == signature
                && !cached.generated_styles.is_empty()
        })
    })
}

pub(super) fn react_app_segment_sources(
    cwd: &Path,
    app_route_path: &Path,
) -> Vec<DxReactAppSegmentSource> {
    let app_root = app_segment_files::app_root_for_route(cwd, app_route_path)
        .unwrap_or_else(|| cwd.join("app"));
    let route_dir = app_route_path.parent().unwrap_or(app_root.as_path());
    let mut dirs = app_segment_dirs(&app_root, route_dir);
    dirs.sort();

    let mut sources = Vec::new();
    for dir in &dirs {
        app_segment_files::push_app_segment_sources(cwd, &mut sources, dir);
    }
    sources
}

fn app_segment_dirs(app_root: &Path, route_dir: &Path) -> Vec<PathBuf> {
    let mut dirs = vec![app_root.to_path_buf()];
    let Ok(relative_route_dir) = route_dir.strip_prefix(app_root) else {
        return dirs;
    };
    let mut current = app_root.to_path_buf();
    for component in relative_route_dir.components() {
        current.push(component.as_os_str());
        dirs.push(current.clone());
    }
    dirs
}

pub(super) fn app_route_cache_signature(cwd: &Path) -> String {
    let mut latest_modified = 0u128;
    let mut file_count = 0u64;
    let mut byte_fingerprint = 0u64;

    for root in ["app", "src/app", "components", "server", "styles", "public"] {
        let root = cwd.join(root);
        if !root.exists() {
            continue;
        }

        for entry in walkdir::WalkDir::new(root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
        {
            let normalized = entry.path().to_string_lossy().replace('\\', "/");
            if normalized.contains("/node_modules/")
                || normalized.ends_with("/styles/generated.css")
                || normalized.contains("/components/icons/")
            {
                continue;
            }
            let Ok(metadata) = entry.metadata() else {
                continue;
            };
            file_count = file_count.saturating_add(1);
            byte_fingerprint = byte_fingerprint
                .wrapping_mul(16777619)
                .wrapping_add(metadata.len());

            let Ok(modified) = metadata.modified() else {
                continue;
            };
            let Ok(since_epoch) = modified.duration_since(UNIX_EPOCH) else {
                continue;
            };
            latest_modified = latest_modified.max(since_epoch.as_nanos());
        }
    }

    format!(
        "{}:{file_count:x}-{latest_modified:x}-{byte_fingerprint:x}",
        Cli::source_manifest_hash(cwd).unwrap_or_else(|| "no-manifest".to_string())
    )
}

fn app_route_render_cache() -> &'static Mutex<HashMap<String, CachedAppRouteRender>> {
    APP_ROUTE_RENDER_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn app_route_style_prewarm_cache() -> &'static Mutex<HashSet<String>> {
    APP_ROUTE_STYLE_PREWARM_CACHE.get_or_init(|| Mutex::new(HashSet::new()))
}

fn remember_app_route_render(cache_key: String, render: CachedAppRouteRender) {
    if let Ok(mut cache) = app_route_render_cache().lock() {
        if cache.len() > 256 {
            cache.clear();
        }
        cache.insert(cache_key, render);
    }
}

fn app_route_cache_cwd_key(cwd: &Path) -> String {
    cwd.canonicalize()
        .unwrap_or_else(|_| cwd.to_path_buf())
        .to_string_lossy()
        .replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn route_component_sources_do_not_include_unimported_components() {
        let project = tempdir().expect("tempdir");
        fs::create_dir_all(project.path().join("components")).expect("components dir");
        fs::write(
            project.path().join("components").join("Unused.tsx"),
            r#"export function Unused() { const [count] = useState(0); return <p>{count}</p>; }"#,
        )
        .expect("unused component");

        let components = react_route_component_sources(
            project.path(),
            "app/page.tsx",
            "export default function Page() { return <main>Static</main>; }",
            &[],
        );

        assert!(components.is_empty());
    }

    #[test]
    fn route_component_sources_include_direct_and_transitive_imports() {
        let project = tempdir().expect("tempdir");
        fs::create_dir_all(project.path().join("components")).expect("components dir");
        fs::write(
            project.path().join("components").join("Counter.tsx"),
            r#"import { Nested } from "./Nested"; export function Counter() { return <Nested />; }"#,
        )
        .expect("counter component");
        fs::write(
            project.path().join("components").join("Nested.tsx"),
            r#"export function Nested() { return <span>Nested</span>; }"#,
        )
        .expect("nested component");

        let components = react_route_component_sources(
            project.path(),
            "app/page.tsx",
            r#"import { Counter } from "@/components/Counter"; export default function Page() { return <Counter />; }"#,
            &[],
        );
        let source_paths = components
            .iter()
            .map(|component| component.source_path.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            source_paths,
            ["components/Counter.tsx", "components/Nested.tsx"]
        );
    }
}
