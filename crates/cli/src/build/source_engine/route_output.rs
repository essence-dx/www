use std::collections::BTreeSet;
use std::path::{Component, Path, PathBuf};

use dx_compiler::delivery::{
    DxReactAppRouteInput, DxReactAppSegmentKind, DxReactAppSegmentSource, DxReactComponentSource,
    DxReactGeneratedStyleAsset, DxReactStyleSource, compile_react_app_route,
};

use crate::error::{DxError, DxResult};

use super::graph::{
    SourceBuildRoute, SourceBuildRouteOutput, SourceBuildStyle, hash_bytes, normalize_path,
    read_file, relative_path, write_file,
};
use super::module_linker::emit_linked_route_modules;
use super::module_linker_paths::resolve_source_import;
use super::module_resolver_config::SourceResolverConfig;
use super::route_paths::{source_route_key, source_route_output_slugs, source_route_slug};

const APP_ROUTER_SOURCE_EXTENSIONS: &[&str] = &["tsx", "jsx", "ts", "js"];
const APP_ROUTER_SOURCE_ROOTS: &[&str] = &["app", "src/app"];

pub fn emit_route_outputs(
    project_root: &Path,
    output_dir: &Path,
    routes: &[SourceBuildRoute],
    styles: &[SourceBuildStyle],
) -> DxResult<Vec<SourceBuildRouteOutput>> {
    let resolver_config = SourceResolverConfig::load(project_root)?;
    let style_sources = load_style_sources(project_root, styles)?;
    let mut route_slugs = source_route_output_slugs(routes);
    let mut outputs = Vec::new();

    for route in routes {
        let route_path = project_root.join(&route.path);
        let route_source = read_utf8(&route_path)?;
        let components = load_imported_components(project_root, route, &resolver_config)?;
        let proof = compile_react_app_route(DxReactAppRouteInput {
            route: route.route.clone(),
            route_source_path: route.path.clone(),
            route_source,
            segments: app_route_segments(project_root, &route.path)?,
            components,
            styles: style_sources.clone(),
            source_manifest_hash: None,
        })
        .map_err(|error| DxError::CompilationError {
            message: error.to_string(),
            file: route_path.clone(),
            src: None,
            span: None,
        })?;

        let route_dir = output_dir.join("source-routes").join(
            route_slugs
                .remove(&source_route_key(route))
                .unwrap_or_else(|| source_route_slug(&route.route)),
        );
        let linked_modules =
            emit_linked_route_modules(project_root, &route_dir, route, &resolver_config)?;
        let fallback_hash = hash_bytes(proof.fallback.html.as_bytes());
        let html_path = route_dir.join("index.html");
        let packet_path = route_dir.join("index.dxpk");
        let page_graph_path = route_dir.join("page-graph.json");
        let route_unit_path = route_dir.join("route-unit.json");
        let shell_chunk_path = route_dir.join(format!("route-shell-{fallback_hash}.mjs"));

        write_generated_style_assets(output_dir, &proof.generated_styles)?;
        write_file(&html_path, proof.fallback.html.as_bytes())?;
        write_file(&packet_path, &proof.packet.encoded)?;
        write_json(&page_graph_path, &proof.page_graph)?;
        write_json(&route_unit_path, &proof.route_unit)?;
        write_file(
            &shell_chunk_path,
            shell_chunk(
                &route.route,
                &fallback_hash,
                &proof.fallback.html,
                linked_modules.entry_chunk_output.as_deref(),
                linked_modules.node_modules_required,
            )
            .as_bytes(),
        )?;

        outputs.push(SourceBuildRouteOutput {
            route: route.route.clone(),
            source_path: route.path.clone(),
            html_output: normalize_path(&relative_path(project_root, &html_path)),
            packet_output: normalize_path(&relative_path(project_root, &packet_path)),
            page_graph_output: normalize_path(&relative_path(project_root, &page_graph_path)),
            route_unit_output: normalize_path(&relative_path(project_root, &route_unit_path)),
            shell_chunk_output: normalize_path(&relative_path(project_root, &shell_chunk_path)),
            entry_module_chunk_output: linked_modules.entry_chunk_output,
            server_data_output: None,
            source_module_chunks: linked_modules.chunks,
            fallback_hash,
            fallback_bytes: proof.fallback.bytes,
            packet_bytes: proof.packet.bytes,
            node_modules_required: linked_modules.node_modules_required,
        });
    }

    Ok(outputs)
}

fn write_generated_style_assets(
    output_dir: &Path,
    generated_styles: &[DxReactGeneratedStyleAsset],
) -> DxResult<()> {
    for asset in generated_styles {
        let relative = safe_generated_style_output_path(&asset.output_path).ok_or_else(|| {
            DxError::CompilationError {
                message: format!("Unsafe generated style output path: {}", asset.output_path),
                file: output_dir.to_path_buf(),
                src: None,
                span: None,
            }
        })?;
        write_file(&output_dir.join(relative), asset.css.as_bytes())?;
    }
    Ok(())
}

fn safe_generated_style_output_path(output_path: &str) -> Option<PathBuf> {
    let mut relative = PathBuf::new();
    for component in Path::new(output_path.trim_start_matches('/')).components() {
        match component {
            Component::Normal(segment) => relative.push(segment),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => return None,
        }
    }
    if relative.as_os_str().is_empty() {
        None
    } else {
        Some(relative)
    }
}

fn app_route_segments(
    project_root: &Path,
    route_source_path: &str,
) -> DxResult<Vec<DxReactAppSegmentSource>> {
    let normalized_source_path = route_source_path.replace('\\', "/");
    let Some(app_root_relative) = app_root_for_source_path(&normalized_source_path) else {
        return Ok(Vec::new());
    };

    let app_root = project_root.join(app_root_relative);
    let route_path = project_root.join(route_source_path);
    let route_dir = route_path.parent().unwrap_or(app_root.as_path());
    let mut sources = Vec::new();

    for dir in app_segment_dirs(&app_root, route_dir) {
        push_app_segment_sources(project_root, &mut sources, &dir)?;
    }

    Ok(sources)
}

fn app_root_for_source_path(normalized_source_path: &str) -> Option<&'static str> {
    APP_ROUTER_SOURCE_ROOTS
        .iter()
        .copied()
        .find(|root| normalized_source_path.starts_with(&format!("{root}/")))
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

fn push_app_segment_sources(
    project_root: &Path,
    sources: &mut Vec<DxReactAppSegmentSource>,
    dir: &Path,
) -> DxResult<()> {
    push_app_segment_source(
        project_root,
        sources,
        dir,
        "layout",
        DxReactAppSegmentKind::Layout,
    )?;
    push_app_segment_source(
        project_root,
        sources,
        dir,
        "template",
        DxReactAppSegmentKind::Template,
    )?;
    push_app_segment_source(
        project_root,
        sources,
        dir,
        "loading",
        DxReactAppSegmentKind::Loading,
    )?;
    push_app_segment_source(
        project_root,
        sources,
        dir,
        "error",
        DxReactAppSegmentKind::Error,
    )?;
    push_app_segment_source(
        project_root,
        sources,
        dir,
        "not-found",
        DxReactAppSegmentKind::NotFound,
    )?;
    Ok(())
}

fn push_app_segment_source(
    project_root: &Path,
    sources: &mut Vec<DxReactAppSegmentSource>,
    dir: &Path,
    stem: &str,
    kind: DxReactAppSegmentKind,
) -> DxResult<()> {
    let Some(path) = first_existing_app_special_file(dir, stem) else {
        return Ok(());
    };
    sources.push(DxReactAppSegmentSource {
        kind,
        source_path: normalize_path(&relative_path(project_root, &path)),
        source: read_utf8(&path)?,
    });
    Ok(())
}

fn first_existing_app_special_file(dir: &Path, stem: &str) -> Option<PathBuf> {
    APP_ROUTER_SOURCE_EXTENSIONS
        .iter()
        .map(|extension| dir.join(format!("{stem}.{extension}")))
        .find(|path| path.is_file())
}

fn load_style_sources(
    project_root: &Path,
    styles: &[SourceBuildStyle],
) -> DxResult<Vec<DxReactStyleSource>> {
    styles
        .iter()
        .map(|style| {
            let source_path = project_root.join(&style.path);
            Ok(DxReactStyleSource {
                source_path: style.path.clone(),
                source: read_utf8(&source_path)?,
            })
        })
        .collect()
}

fn load_imported_components(
    project_root: &Path,
    route: &SourceBuildRoute,
    resolver_config: &SourceResolverConfig,
) -> DxResult<Vec<DxReactComponentSource>> {
    let mut seen = BTreeSet::new();
    let mut components = Vec::new();

    for import in &route.imports {
        if !import.specifier.starts_with('.')
            && !import.specifier.starts_with("@/")
            && !resolver_config.matches_source_alias(&import.specifier)
        {
            continue;
        }
        let importer = project_root.join(&route.path);
        let Some(component_path) =
            resolve_source_import(project_root, &importer, &import.specifier, resolver_config)
        else {
            continue;
        };
        let component_path = component_path.path;
        if !is_component_source(&component_path) {
            continue;
        }
        let relative = normalize_path(&relative_path(project_root, &component_path));
        if !seen.insert(relative.clone()) {
            continue;
        }
        components.push(DxReactComponentSource {
            name: component_name(&component_path),
            source_path: relative,
            source: read_utf8(&component_path)?,
            package_id: None,
        });
    }

    Ok(components)
}

fn is_component_source(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| matches!(extension, "tsx" | "jsx"))
}

fn component_name(path: &Path) -> String {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("Component")
        .to_string()
}

fn shell_chunk(
    route: &str,
    fallback_hash: &str,
    html: &str,
    entry_module_chunk: Option<&str>,
    node_modules_required: bool,
) -> String {
    let route = serde_json::to_string(route).expect("route string");
    let fallback_hash = serde_json::to_string(fallback_hash).expect("hash string");
    let html = serde_json::to_string(html).expect("html string");
    let entry_import = entry_module_chunk
        .and_then(|path| {
            Path::new(path)
                .file_name()
                .and_then(|file_name| file_name.to_str())
        })
        .map(|file_name| {
            format!(
                r#"import {{ dxSourceModule as dxRouteEntryModule }} from "./modules/{file_name}";
"#
            )
        })
        .unwrap_or_else(|| "const dxRouteEntryModule = null;\n".to_string());

    format!(
        r#"{entry_import}const fallbackHtml = {html};

export const dxRouteShell = Object.freeze({{
  route: {route},
  fallbackHash: {fallback_hash},
  fullReactHydration: false,
  nodeModulesRequired: {node_modules_required},
  sourceModuleEntry: dxRouteEntryModule
}});

export function mount(target) {{
  const container = typeof target === "string" ? document.querySelector(target) : target;
  if (!container) {{
    throw new Error(`DX route shell target not found for ${{dxRouteShell.route}}`);
  }}
  container.innerHTML = fallbackHtml;
  return dxRouteShell;
}}

export default dxRouteShell;
"#
    )
}

fn read_utf8(path: &Path) -> DxResult<String> {
    String::from_utf8(read_file(path)?).map_err(|error| DxError::CompilationError {
        message: error.to_string(),
        file: path.to_path_buf(),
        src: None,
        span: None,
    })
}

fn write_json<T: serde::Serialize>(path: &Path, value: &T) -> DxResult<()> {
    let json = serde_json::to_string_pretty(value)?;
    write_file(path, json.as_bytes())
}
