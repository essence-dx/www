use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use crate::app_router_segments::{self, AppRouteSegmentKind};
use serde::Serialize;

use super::app_segment_files::{self, is_app_page_file_name, strip_app_page_file_name};
use super::route_request_values::{decode_path_segment, decode_path_segments, parse_search_params};

#[derive(Debug, Clone)]
pub(super) struct AppRouteMatch {
    pub(super) path: PathBuf,
    pub(super) params: BTreeMap<String, String>,
    pub(super) search_params: BTreeMap<String, String>,
    pub(super) shadowed_paths: Vec<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub(super) enum AppDiscoveredSegmentKind {
    Layout,
    Template,
    Loading,
    Error,
    NotFound,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct AppDiscoveredSegmentFile {
    pub(super) kind: AppDiscoveredSegmentKind,
    pub(super) path: PathBuf,
    pub(super) route_path: String,
    pub(super) depth: usize,
    pub(super) non_path_segment_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub(super) enum AppDiscoveredRouteSegmentKind {
    Static,
    Dynamic,
    CatchAll,
    OptionalCatchAll,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct AppDiscoveredRouteSpecificity {
    pub(super) segment_kinds: Vec<AppDiscoveredRouteSegmentKind>,
    pub(super) static_segment_count: usize,
    pub(super) dynamic_segment_count: usize,
    pub(super) catch_all_segment_count: usize,
    pub(super) optional_catch_all_segment_count: usize,
    pub(super) visible_segment_count: usize,
    pub(super) precedence_score: usize,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct AppDiscoveredPageRoute {
    pub(super) path: PathBuf,
    pub(super) route_path: String,
    pub(super) route_shape: String,
    pub(super) root_index: usize,
    pub(super) non_path_segment_count: usize,
    pub(super) segment_files: Vec<AppDiscoveredSegmentFile>,
    pub(super) shadowed_paths: Vec<PathBuf>,
    pub(super) shape_collision_paths: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct AppDiscoveredSegmentSummary {
    pub(super) kind: AppDiscoveredSegmentKind,
    pub(super) source_path: String,
    pub(super) route_path: String,
    pub(super) depth: usize,
    pub(super) non_path_segment_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct AppDiscoveredRouteSummary {
    pub(super) source_path: String,
    pub(super) route_path: String,
    pub(super) route_shape: String,
    pub(super) root_index: usize,
    pub(super) non_path_segment_count: usize,
    pub(super) specificity: AppDiscoveredRouteSpecificity,
    pub(super) segment_files: Vec<AppDiscoveredSegmentSummary>,
    pub(super) shadowed_source_paths: Vec<String>,
    pub(super) shape_collision_source_paths: Vec<String>,
}

#[derive(Debug, Clone)]
struct AppRouteCandidateMatch {
    score: usize,
    specificity: Vec<usize>,
    catch_all_used: bool,
    non_path_segment_count: usize,
    params: BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
struct ScoredAppRouteMatch {
    score: usize,
    specificity: Vec<usize>,
    catch_all_used: bool,
    non_path_segment_count: usize,
    root_index: usize,
    route_path: String,
    route_match: AppRouteMatch,
}

#[derive(Debug, Clone)]
struct AppPageRouteSource {
    app_root: PathBuf,
    path: PathBuf,
    route_path: String,
    route_shape: String,
    root_index: usize,
    non_path_segment_count: usize,
}

const APP_DISCOVERED_SEGMENT_KINDS: &[AppDiscoveredSegmentKind] = &[
    AppDiscoveredSegmentKind::Layout,
    AppDiscoveredSegmentKind::Template,
    AppDiscoveredSegmentKind::Loading,
    AppDiscoveredSegmentKind::Error,
    AppDiscoveredSegmentKind::NotFound,
];

const APP_SPECIAL_FILE_EXTENSIONS: &[&str] = &["tsx", "jsx", "ts", "js"];

pub(super) fn discover_page_routes(cwd: &Path) -> Vec<AppDiscoveredPageRoute> {
    let mut routes = app_page_route_sources(cwd)
        .into_iter()
        .map(|source| {
            let segment_files = discover_segment_files_for_page_source(cwd, &source);
            AppDiscoveredPageRoute {
                path: source.path,
                route_path: source.route_path,
                route_shape: source.route_shape,
                root_index: source.root_index,
                non_path_segment_count: source.non_path_segment_count,
                segment_files,
                shadowed_paths: Vec::new(),
                shape_collision_paths: Vec::new(),
            }
        })
        .collect::<Vec<_>>();

    routes.sort_by(|left, right| {
        left.route_path
            .cmp(&right.route_path)
            .then_with(|| left.root_index.cmp(&right.root_index))
            .then_with(|| {
                left.non_path_segment_count
                    .cmp(&right.non_path_segment_count)
            })
            .then_with(|| left.path.cmp(&right.path))
    });
    mark_shadowed_discovered_page_routes(&mut routes);
    mark_shape_collision_discovered_page_routes(&mut routes);
    routes
}

pub(super) fn discover_page_route_summaries(cwd: &Path) -> Vec<AppDiscoveredRouteSummary> {
    discover_page_routes(cwd)
        .into_iter()
        .map(|route| discovered_route_summary(cwd, route))
        .collect()
}

pub(super) fn route_match(cwd: &Path, path: &str) -> Option<AppRouteMatch> {
    let path_part = app_route_request_path_part(path).trim_end_matches('/');
    let trimmed = path_part.trim_start_matches('/').trim_end_matches('/');
    if trimmed.starts_with("api/") {
        return None;
    }
    let request_segments = if trimmed.is_empty() {
        Vec::new()
    } else {
        trimmed.split('/').collect::<Vec<_>>()
    };
    let mut matches = app_page_route_sources(cwd)
        .into_iter()
        .filter_map(|source| {
            let candidate =
                match_app_route_candidate(&source.app_root, &source.path, &request_segments)?;
            Some(ScoredAppRouteMatch {
                score: candidate.score,
                specificity: candidate.specificity,
                catch_all_used: candidate.catch_all_used,
                non_path_segment_count: candidate.non_path_segment_count,
                root_index: source.root_index,
                route_path: source.route_path,
                route_match: AppRouteMatch {
                    path: source.path,
                    params: candidate.params,
                    search_params: parse_search_params(path),
                    shadowed_paths: Vec::new(),
                },
            })
        })
        .collect::<Vec<_>>();
    matches.sort_by(|left, right| {
        right
            .specificity
            .cmp(&left.specificity)
            .then_with(|| right.score.cmp(&left.score))
            .then_with(|| left.catch_all_used.cmp(&right.catch_all_used))
            .then_with(|| left.root_index.cmp(&right.root_index))
            .then_with(|| {
                left.non_path_segment_count
                    .cmp(&right.non_path_segment_count)
            })
            .then_with(|| left.route_match.path.cmp(&right.route_match.path))
    });
    let mut matches = matches.into_iter();
    let mut best = matches.next()?;
    let best_route_path = best.route_path.clone();
    best.route_match.shadowed_paths = matches
        .filter(|candidate| candidate.route_path == best_route_path)
        .map(|candidate| candidate.route_match.path)
        .collect();
    Some(best.route_match)
}

fn app_page_route_sources(cwd: &Path) -> Vec<AppPageRouteSource> {
    let mut sources = Vec::new();
    for (root_index, app_root) in app_segment_files::app_route_roots(cwd)
        .into_iter()
        .enumerate()
    {
        sources.extend(
            walkdir::WalkDir::new(&app_root)
                .into_iter()
                .filter_map(Result::ok)
                .filter(is_app_page_file_entry)
                .filter(|entry| skips_node_modules(entry.path()))
                .filter_map(|entry| {
                    let source_path = relative_app_source_path(cwd, entry.path());
                    let route_path = route_path_from_page_source_path(&source_path)?;
                    let route_shape = route_shape_from_page_source_path(&source_path)?;
                    let segments = page_route_segments_from_source_path(&source_path)?;
                    Some(AppPageRouteSource {
                        app_root: app_root.clone(),
                        path: entry.path().to_path_buf(),
                        route_path,
                        route_shape,
                        root_index,
                        non_path_segment_count: segments
                            .iter()
                            .filter(|segment| is_app_router_non_path_segment(segment))
                            .count(),
                    })
                }),
        );
    }
    sources
}

fn discover_segment_files_for_page_source(
    _cwd: &Path,
    source: &AppPageRouteSource,
) -> Vec<AppDiscoveredSegmentFile> {
    let Some(route_dir) = source.path.parent() else {
        return Vec::new();
    };
    let Ok(relative_route_dir) = route_dir.strip_prefix(&source.app_root) else {
        return Vec::new();
    };

    let mut dirs = Vec::new();
    let mut current_dir = source.app_root.clone();
    dirs.push(current_dir.clone());
    for component in relative_route_dir.components() {
        current_dir = current_dir.join(component.as_os_str());
        dirs.push(current_dir.clone());
    }

    let mut segment_files = Vec::new();
    for dir in dirs {
        let Some((route_path, depth, non_path_segment_count)) =
            segment_route_metadata(&source.app_root, &dir)
        else {
            continue;
        };
        for kind in APP_DISCOVERED_SEGMENT_KINDS {
            let Some(path) = app_special_file_path(&dir, *kind) else {
                continue;
            };
            segment_files.push(AppDiscoveredSegmentFile {
                kind: *kind,
                path,
                route_path: route_path.clone(),
                depth,
                non_path_segment_count,
            });
        }
    }
    segment_files
}

fn discovered_route_summary(
    cwd: &Path,
    route: AppDiscoveredPageRoute,
) -> AppDiscoveredRouteSummary {
    let source_path = relative_app_source_path(cwd, &route.path);
    let route_segments = page_route_segments_from_source_path(&source_path).unwrap_or_default();
    let specificity = discovered_route_specificity_from_segments(&route_segments);
    let segment_files = route
        .segment_files
        .into_iter()
        .map(|segment| discovered_segment_summary(cwd, segment))
        .collect();
    let shadowed_source_paths = route
        .shadowed_paths
        .iter()
        .map(|path| relative_app_source_path(cwd, path))
        .collect();
    let shape_collision_source_paths = route
        .shape_collision_paths
        .iter()
        .map(|path| relative_app_source_path(cwd, path))
        .collect();

    AppDiscoveredRouteSummary {
        source_path,
        route_path: route.route_path,
        route_shape: route.route_shape,
        root_index: route.root_index,
        non_path_segment_count: route.non_path_segment_count,
        specificity,
        segment_files,
        shadowed_source_paths,
        shape_collision_source_paths,
    }
}

fn discovered_segment_summary(
    cwd: &Path,
    segment: AppDiscoveredSegmentFile,
) -> AppDiscoveredSegmentSummary {
    AppDiscoveredSegmentSummary {
        kind: segment.kind,
        source_path: relative_app_source_path(cwd, &segment.path),
        route_path: segment.route_path,
        depth: segment.depth,
        non_path_segment_count: segment.non_path_segment_count,
    }
}

fn discovered_route_specificity_from_segments(
    segments: &[String],
) -> AppDiscoveredRouteSpecificity {
    let segment_kinds = segments
        .iter()
        .filter_map(|segment| discovered_route_segment_kind(segment))
        .collect::<Vec<_>>();
    let static_segment_count = segment_kinds
        .iter()
        .filter(|kind| **kind == AppDiscoveredRouteSegmentKind::Static)
        .count();
    let dynamic_segment_count = segment_kinds
        .iter()
        .filter(|kind| **kind == AppDiscoveredRouteSegmentKind::Dynamic)
        .count();
    let catch_all_segment_count = segment_kinds
        .iter()
        .filter(|kind| **kind == AppDiscoveredRouteSegmentKind::CatchAll)
        .count();
    let optional_catch_all_segment_count = segment_kinds
        .iter()
        .filter(|kind| **kind == AppDiscoveredRouteSegmentKind::OptionalCatchAll)
        .count();
    let precedence_score = segment_kinds
        .iter()
        .map(|kind| kind.precedence_score())
        .sum();

    AppDiscoveredRouteSpecificity {
        visible_segment_count: segment_kinds.len(),
        segment_kinds,
        static_segment_count,
        dynamic_segment_count,
        catch_all_segment_count,
        optional_catch_all_segment_count,
        precedence_score,
    }
}

fn discovered_route_segment_kind(segment: &str) -> Option<AppDiscoveredRouteSegmentKind> {
    match app_router_segments::classify_app_route_segment(segment) {
        AppRouteSegmentKind::RouteGroup | AppRouteSegmentKind::ParallelSlot => None,
        AppRouteSegmentKind::OptionalCatchAll(_) => {
            Some(AppDiscoveredRouteSegmentKind::OptionalCatchAll)
        }
        AppRouteSegmentKind::RequiredCatchAll(_) => Some(AppDiscoveredRouteSegmentKind::CatchAll),
        AppRouteSegmentKind::Dynamic(_) => Some(AppDiscoveredRouteSegmentKind::Dynamic),
        AppRouteSegmentKind::Static(_) => Some(AppDiscoveredRouteSegmentKind::Static),
        AppRouteSegmentKind::Private
        | AppRouteSegmentKind::Intercepting
        | AppRouteSegmentKind::Malformed => None,
    }
}

fn segment_route_metadata(app_root: &Path, dir: &Path) -> Option<(String, usize, usize)> {
    let relative = dir.strip_prefix(app_root).ok()?;
    let segments = relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    if segments.iter().any(|segment| {
        is_private_app_folder_segment(segment) || is_intercepting_app_route_segment(segment)
    }) {
        return None;
    }
    let visible_segments = segments
        .iter()
        .filter(|segment| !is_app_router_non_path_segment(segment))
        .cloned()
        .collect::<Vec<_>>();
    let non_path_segment_count = segments
        .iter()
        .filter(|segment| is_app_router_non_path_segment(segment))
        .count();
    let route_path = if visible_segments.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", visible_segments.join("/"))
    };
    Some((route_path, visible_segments.len(), non_path_segment_count))
}

fn app_special_file_path(dir: &Path, kind: AppDiscoveredSegmentKind) -> Option<PathBuf> {
    APP_SPECIAL_FILE_EXTENSIONS
        .iter()
        .map(|extension| dir.join(format!("{}.{}", kind.file_stem(), extension)))
        .find(|path| path.is_file())
}

fn is_app_page_file_entry(entry: &walkdir::DirEntry) -> bool {
    entry.file_type().is_file()
        && entry
            .file_name()
            .to_str()
            .is_some_and(is_app_page_file_name)
}

fn skips_node_modules(path: &Path) -> bool {
    path.components()
        .all(|component| component.as_os_str().to_string_lossy() != "node_modules")
}

fn mark_shadowed_discovered_page_routes(routes: &mut [AppDiscoveredPageRoute]) {
    let mut index = 0usize;
    while index < routes.len() {
        let route_path = routes[index].route_path.clone();
        let mut end = index + 1;
        while end < routes.len() && routes[end].route_path == route_path {
            end += 1;
        }
        routes[index].shadowed_paths = routes[index + 1..end]
            .iter()
            .map(|route| route.path.clone())
            .collect();
        index = end;
    }
}

fn mark_shape_collision_discovered_page_routes(routes: &mut [AppDiscoveredPageRoute]) {
    let mut groups = BTreeMap::<String, Vec<usize>>::new();
    for (index, route) in routes.iter().enumerate() {
        groups
            .entry(route.route_shape.clone())
            .or_default()
            .push(index);
    }

    let updates = groups
        .values()
        .flat_map(|indices| {
            indices.iter().copied().filter_map(|route_index| {
                let collision_paths = shape_collision_peer_paths(routes, indices, route_index);
                if collision_paths.is_empty() {
                    None
                } else {
                    Some((route_index, collision_paths))
                }
            })
        })
        .collect::<Vec<_>>();

    for (primary, collision_paths) in updates {
        routes[primary].shape_collision_paths = collision_paths;
    }
}

fn shape_collision_peer_paths(
    routes: &[AppDiscoveredPageRoute],
    indices: &[usize],
    route_index: usize,
) -> Vec<PathBuf> {
    let route_path = routes[route_index].route_path.as_str();
    indices
        .iter()
        .copied()
        .filter(|peer_index| *peer_index != route_index)
        .filter(|peer_index| routes[*peer_index].route_path != route_path)
        .map(|peer_index| routes[peer_index].path.clone())
        .collect()
}

impl AppDiscoveredSegmentKind {
    fn file_stem(self) -> &'static str {
        match self {
            Self::Layout => "layout",
            Self::Template => "template",
            Self::Loading => "loading",
            Self::Error => "error",
            Self::NotFound => "not-found",
        }
    }
}

impl AppDiscoveredRouteSegmentKind {
    fn precedence_score(self) -> usize {
        match self {
            Self::Static => 4,
            Self::Dynamic => 2,
            Self::CatchAll => 1,
            Self::OptionalCatchAll => 0,
        }
    }
}

fn app_route_request_path_part(path: &str) -> &str {
    let query_index = path.find('?');
    let fragment_index = path.find('#');
    let end = match (query_index, fragment_index) {
        (Some(query), Some(fragment)) => query.min(fragment),
        (Some(query), None) => query,
        (None, Some(fragment)) => fragment,
        (None, None) => path.len(),
    };
    &path[..end]
}

fn relative_app_source_path(cwd: &Path, path: &Path) -> String {
    path.strip_prefix(cwd)
        .unwrap_or(path)
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn match_app_route_candidate(
    app_root: &Path,
    page_path: &Path,
    request_segments: &[&str],
) -> Option<AppRouteCandidateMatch> {
    let route_dir = page_path.parent().unwrap_or(app_root);
    let relative_route_dir = route_dir.strip_prefix(app_root).ok()?;
    let route_segments = relative_route_dir
        .components()
        .map(|component| component.as_os_str().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    let mut params = BTreeMap::new();
    let mut request_index = 0usize;
    let mut score = 0usize;
    let mut specificity = Vec::new();
    let mut catch_all_used = false;
    let mut non_path_segment_count = 0usize;

    for segment in route_segments {
        match app_router_segments::classify_app_route_segment(&segment) {
            AppRouteSegmentKind::Private
            | AppRouteSegmentKind::Intercepting
            | AppRouteSegmentKind::Malformed => return None,
            AppRouteSegmentKind::RouteGroup | AppRouteSegmentKind::ParallelSlot => {
                non_path_segment_count += 1;
                continue;
            }
            AppRouteSegmentKind::OptionalCatchAll(name) => {
                let remaining_segments = &request_segments[request_index..];
                let value = decode_path_segments(remaining_segments);
                params.insert(name.to_string(), value);
                if !remaining_segments.is_empty() {
                    specificity
                        .push(AppDiscoveredRouteSegmentKind::OptionalCatchAll.precedence_score());
                }
                request_index = request_segments.len();
                catch_all_used = true;
                break;
            }
            AppRouteSegmentKind::RequiredCatchAll(name) => {
                if request_index >= request_segments.len() {
                    return None;
                }
                let value = decode_path_segments(&request_segments[request_index..]);
                params.insert(name.to_string(), value);
                specificity.push(AppDiscoveredRouteSegmentKind::CatchAll.precedence_score());
                score += AppDiscoveredRouteSegmentKind::CatchAll.precedence_score();
                request_index = request_segments.len();
                catch_all_used = true;
                break;
            }
            AppRouteSegmentKind::Dynamic(name) => {
                let value = request_segments.get(request_index)?;
                params.insert(name.to_string(), decode_path_segment(value));
                request_index += 1;
                specificity.push(AppDiscoveredRouteSegmentKind::Dynamic.precedence_score());
                score += AppDiscoveredRouteSegmentKind::Dynamic.precedence_score();
            }
            AppRouteSegmentKind::Static(segment) => {
                let value = request_segments.get(request_index)?;
                let decoded_value = decode_path_segment(value);
                if segment != decoded_value.as_str() {
                    return None;
                }
                request_index += 1;
                specificity.push(AppDiscoveredRouteSegmentKind::Static.precedence_score());
                score += AppDiscoveredRouteSegmentKind::Static.precedence_score();
            }
        }
    }

    if request_index == request_segments.len() {
        Some(AppRouteCandidateMatch {
            score,
            specificity,
            catch_all_used,
            non_path_segment_count,
            params,
        })
    } else {
        None
    }
}

pub(super) fn is_private_app_folder_segment(segment: &str) -> bool {
    app_router_segments::is_private_app_folder_segment(segment)
}

pub(super) fn is_intercepting_app_route_segment(segment: &str) -> bool {
    app_router_segments::is_intercepting_app_route_segment(segment)
}

pub(super) fn route_path_from_page_source_path(source_path: &str) -> Option<String> {
    let visible_segments = page_route_segments_from_source_path(source_path)?
        .into_iter()
        .filter(|segment| !is_app_router_non_path_segment(segment))
        .collect::<Vec<_>>();
    if visible_segments.is_empty() {
        Some("/".to_string())
    } else {
        Some(format!("/{}", visible_segments.join("/")))
    }
}

fn route_shape_from_page_source_path(source_path: &str) -> Option<String> {
    let visible_segments = page_route_segments_from_source_path(source_path)?
        .iter()
        .filter_map(|segment| shape_segment_from_route_segment(segment))
        .collect::<Vec<_>>();
    if visible_segments.is_empty() {
        Some("/".to_string())
    } else {
        Some(format!("/{}", visible_segments.join("/")))
    }
}

fn shape_segment_from_route_segment(segment: &str) -> Option<String> {
    if is_app_router_non_path_segment(segment) {
        return None;
    }
    if optional_catchall_segment_name(segment).is_some() {
        return Some("[[...]]".to_string());
    }
    if catchall_segment_name(segment).is_some() {
        return Some("[...]".to_string());
    }
    if dynamic_segment_name(segment).is_some() {
        return Some("[]".to_string());
    }
    Some(segment.to_string())
}

pub(super) fn page_route_segments_from_source_path(source_path: &str) -> Option<Vec<String>> {
    let segments = raw_page_route_segments_from_source_path(source_path)?;
    if segments
        .iter()
        .any(|segment| is_malformed_app_route_parameter_segment(segment))
        || app_router_segments::route_segments_have_duplicate_param_names(&segments)
        || app_router_segments::route_segments_have_nonterminal_catch_all(&segments)
        || app_router_segments::has_unsupported_app_page_route_segments(&segments)
    {
        None
    } else {
        Some(segments)
    }
}

pub(super) fn raw_page_route_segments_from_source_path(source_path: &str) -> Option<Vec<String>> {
    let normalized = source_path.replace('\\', "/");
    let route = strip_app_page_file_name(&normalized)?;
    if route.is_empty() {
        Some(Vec::new())
    } else {
        Some(
            route
                .split('/')
                .filter(|segment| !segment.is_empty())
                .map(str::to_string)
                .collect::<Vec<_>>(),
        )
    }
}

pub(super) fn is_app_router_non_path_segment(segment: &str) -> bool {
    app_router_segments::is_app_router_non_path_segment(segment)
}

pub(super) fn is_parallel_route_slot_segment(segment: &str) -> bool {
    app_router_segments::is_parallel_route_slot_segment(segment)
}

fn is_malformed_app_route_parameter_segment(segment: &str) -> bool {
    app_router_segments::is_malformed_app_route_parameter_segment(segment)
}

fn dynamic_segment_name(segment: &str) -> Option<&str> {
    app_router_segments::dynamic_segment_name(segment)
}

fn catchall_segment_name(segment: &str) -> Option<&str> {
    app_router_segments::catchall_segment_name(segment)
}

fn optional_catchall_segment_name(segment: &str) -> Option<&str> {
    app_router_segments::optional_catchall_segment_name(segment)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    fn temp_project(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "dx-www-app-page-routes-{}-{}",
            name,
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("create temp project");
        root
    }

    fn write_page(root: &Path, relative_path: &str) {
        write_source(
            root,
            relative_path,
            "export default function Page() { return <main />; }",
        );
    }

    fn write_source(root: &Path, relative_path: &str, source: &str) {
        let path = root.join(relative_path);
        fs::create_dir_all(path.parent().expect("page parent")).expect("create page parent");
        fs::write(path, source).expect("write source");
    }

    fn relative_path(root: &Path, path: &Path) -> String {
        path.strip_prefix(root)
            .unwrap_or(path)
            .components()
            .map(|component| component.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/")
    }

    #[test]
    fn route_match_decodes_request_params_and_search_params() {
        let root = temp_project("decode");
        write_page(&root, "app/users/[id]/page.tsx");

        let route_match = route_match(
            &root,
            "/users/alice%20ng?tab=launch+plan&encoded=a%2Fb&bad=%zz",
        )
        .expect("match App Router page with encoded request values");

        assert_eq!(route_match.params.get("id"), Some(&"alice ng".to_string()));
        assert_eq!(
            route_match.search_params.get("tab"),
            Some(&"launch plan".to_string())
        );
        assert_eq!(
            route_match.search_params.get("encoded"),
            Some(&"a/b".to_string())
        );
        assert_eq!(
            route_match.search_params.get("bad"),
            Some(&"%zz".to_string())
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_match_ignores_query_and_fragment_suffixes() {
        let root = temp_project("query-fragment");
        write_page(&root, "app/docs/[slug]/page.tsx");

        let query_first = route_match(&root, "/docs/intro?tab=api#heading")
            .expect("match App Router page before query suffix");
        assert_eq!(query_first.path, root.join("app/docs/[slug]/page.tsx"));
        assert_eq!(query_first.params.get("slug"), Some(&"intro".to_string()));
        assert_eq!(
            query_first.search_params.get("tab"),
            Some(&"api".to_string())
        );

        let fragment_first = route_match(&root, "/docs/intro#heading?tab=api")
            .expect("match App Router page before fragment suffix");
        assert_eq!(fragment_first.path, root.join("app/docs/[slug]/page.tsx"));
        assert_eq!(
            fragment_first.params.get("slug"),
            Some(&"intro".to_string())
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_match_decodes_static_request_segments() {
        let root = temp_project("static-decode");
        write_page(&root, "app/docs/hello world/page.tsx");

        let route_match = route_match(&root, "/docs/hello%20world?tab=read")
            .expect("match encoded static request segment");
        assert_eq!(route_match.path, root.join("app/docs/hello world/page.tsx"));
        assert!(route_match.params.is_empty());
        assert_eq!(
            route_match.search_params.get("tab"),
            Some(&"read".to_string())
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_match_ignores_app_router_parallel_slots() {
        let root = temp_project("parallel-slot");
        write_page(&root, "app/@modal/settings/page.tsx");

        let route_match = route_match(&root, "/settings")
            .expect("match App Router page nested behind a parallel slot");
        assert_eq!(route_match.path, root.join("app/@modal/settings/page.tsx"));
        assert!(route_match.params.is_empty());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_match_ignores_private_app_folders() {
        let root = temp_project("private-folder");
        write_page(&root, "app/_internal/page.tsx");
        write_page(&root, "app/public/page.tsx");

        assert!(
            route_match(&root, "/_internal").is_none(),
            "private underscore folders must not become public routes"
        );
        let public_match = route_match(&root, "/public").expect("match sibling public route");
        assert_eq!(public_match.path, root.join("app/public/page.tsx"));
        assert_eq!(
            route_path_from_page_source_path("app/_internal/page.tsx"),
            None
        );
        assert_eq!(
            route_path_from_page_source_path("app/(marketing)/_drafts/page.tsx"),
            None
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_match_ignores_unsupported_intercepting_routes() {
        let root = temp_project("intercepting-route");
        write_page(&root, "app/(.)photo/page.tsx");
        write_page(&root, "app/(..)(..)feed/[id]/page.tsx");
        write_page(&root, "app/photos/page.tsx");

        assert!(
            route_match(&root, "/(.)photo").is_none(),
            "unsupported intercepting routes must not become literal public paths"
        );
        let photos_match = route_match(&root, "/photos").expect("match sibling public route");
        assert_eq!(photos_match.path, root.join("app/photos/page.tsx"));
        assert_eq!(
            route_path_from_page_source_path("app/(.)photo/page.tsx"),
            None
        );
        assert_eq!(
            route_path_from_page_source_path("app/(..)(..)feed/[id]/page.tsx"),
            None
        );
        assert!(!is_intercepting_app_route_segment("(marketing)"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn discover_page_routes_skips_malformed_parameter_segments() {
        let root = temp_project("malformed-param-discovery");
        write_page(&root, "app/[]/page.tsx");
        write_page(&root, "app/[[id]]/page.tsx");
        write_page(&root, "app/docs/[...]/page.tsx");
        write_page(&root, "app/docs/[[...]]/page.tsx");
        write_page(&root, "app/users/[id]/page.tsx");
        write_page(&root, "app/docs/[...slug]/page.tsx");
        write_page(&root, "app/files/[[...path]]/page.tsx");

        let routes = discover_page_routes(&root)
            .iter()
            .map(|route| relative_path(&root, &route.path))
            .collect::<Vec<_>>();

        assert_eq!(route_path_from_page_source_path("app/[]/page.tsx"), None);
        assert_eq!(
            route_path_from_page_source_path("app/[[id]]/page.tsx"),
            None
        );
        assert_eq!(
            route_path_from_page_source_path("app/docs/[...]/page.tsx"),
            None
        );
        assert_eq!(
            route_path_from_page_source_path("app/docs/[[...]]/page.tsx"),
            None
        );
        assert!(!routes.contains(&"app/[]/page.tsx".to_string()));
        assert!(!routes.contains(&"app/[[id]]/page.tsx".to_string()));
        assert!(!routes.contains(&"app/docs/[...]/page.tsx".to_string()));
        assert!(!routes.contains(&"app/docs/[[...]]/page.tsx".to_string()));
        assert!(routes.contains(&"app/users/[id]/page.tsx".to_string()));
        assert!(routes.contains(&"app/docs/[...slug]/page.tsx".to_string()));
        assert!(routes.contains(&"app/files/[[...path]]/page.tsx".to_string()));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_match_rejects_malformed_parameter_segments() {
        let root = temp_project("malformed-param-match");
        write_page(&root, "app/[]/page.tsx");
        write_page(&root, "app/[[id]]/page.tsx");
        write_page(&root, "app/docs/[...]/page.tsx");
        write_page(&root, "app/docs/[[...]]/page.tsx");
        write_page(&root, "app/users/[id]/page.tsx");
        write_page(&root, "app/docs/[...slug]/page.tsx");

        assert!(route_match(&root, "/literal").is_none());
        assert!(route_match(&root, "/docs/reference").is_some());
        let user_match = route_match(&root, "/users/alice").expect("valid dynamic route");
        assert_eq!(user_match.path, root.join("app/users/[id]/page.tsx"));
        assert_eq!(user_match.params.get("id"), Some(&"alice".to_string()));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn discover_page_routes_skips_malformed_non_path_segments() {
        let root = temp_project("malformed-non-path-discovery");
        write_page(&root, "app/()/page.tsx");
        write_page(&root, "app/(marketing/page.tsx");
        write_page(&root, "app/marketing)/page.tsx");
        write_page(&root, "app/@/page.tsx");
        write_page(&root, "app/(marketing)/about/page.tsx");
        write_page(&root, "app/@modal/settings/page.tsx");

        let routes = discover_page_routes(&root)
            .iter()
            .map(|route| relative_path(&root, &route.path))
            .collect::<Vec<_>>();

        assert_eq!(route_path_from_page_source_path("app/()/page.tsx"), None);
        assert_eq!(
            route_path_from_page_source_path("app/(marketing/page.tsx"),
            None
        );
        assert_eq!(
            route_path_from_page_source_path("app/marketing)/page.tsx"),
            None
        );
        assert_eq!(route_path_from_page_source_path("app/@/page.tsx"), None);
        assert!(!routes.contains(&"app/()/page.tsx".to_string()));
        assert!(!routes.contains(&"app/(marketing/page.tsx".to_string()));
        assert!(!routes.contains(&"app/marketing)/page.tsx".to_string()));
        assert!(!routes.contains(&"app/@/page.tsx".to_string()));
        assert!(routes.contains(&"app/(marketing)/about/page.tsx".to_string()));
        assert!(routes.contains(&"app/@modal/settings/page.tsx".to_string()));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_match_rejects_malformed_non_path_segments() {
        let root = temp_project("malformed-non-path-match");
        write_page(&root, "app/()/page.tsx");
        write_page(&root, "app/(marketing/page.tsx");
        write_page(&root, "app/marketing)/page.tsx");
        write_page(&root, "app/@/page.tsx");
        write_page(&root, "app/(marketing)/about/page.tsx");
        write_page(&root, "app/@modal/settings/page.tsx");

        assert!(route_match(&root, "/").is_none());
        assert!(route_match(&root, "/(marketing").is_none());
        assert!(route_match(&root, "/marketing)").is_none());
        assert!(route_match(&root, "/@").is_none());

        let about_match = route_match(&root, "/about").expect("valid route group route");
        assert_eq!(
            about_match.path,
            root.join("app/(marketing)/about/page.tsx")
        );
        let settings_match = route_match(&root, "/settings").expect("valid parallel slot route");
        assert_eq!(
            settings_match.path,
            root.join("app/@modal/settings/page.tsx")
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn discover_page_routes_skips_duplicate_parameter_names() {
        let root = temp_project("duplicate-param-discovery");
        write_page(&root, "app/[team]/[team]/page.tsx");
        write_page(&root, "app/docs/[slug]/[...slug]/page.tsx");
        write_page(&root, "app/files/[path]/[[...path]]/page.tsx");
        write_page(&root, "app/[org]/[team]/page.tsx");
        write_page(&root, "app/docs/[slug]/[...rest]/page.tsx");
        write_page(&root, "app/files/[path]/[[...rest]]/page.tsx");

        let routes = discover_page_routes(&root)
            .iter()
            .map(|route| relative_path(&root, &route.path))
            .collect::<Vec<_>>();

        assert_eq!(
            route_path_from_page_source_path("app/[team]/[team]/page.tsx"),
            None
        );
        assert_eq!(
            route_path_from_page_source_path("app/docs/[slug]/[...slug]/page.tsx"),
            None
        );
        assert_eq!(
            route_path_from_page_source_path("app/files/[path]/[[...path]]/page.tsx"),
            None
        );
        assert!(!routes.contains(&"app/[team]/[team]/page.tsx".to_string()));
        assert!(!routes.contains(&"app/docs/[slug]/[...slug]/page.tsx".to_string()));
        assert!(!routes.contains(&"app/files/[path]/[[...path]]/page.tsx".to_string()));
        assert!(routes.contains(&"app/[org]/[team]/page.tsx".to_string()));
        assert!(routes.contains(&"app/docs/[slug]/[...rest]/page.tsx".to_string()));
        assert!(routes.contains(&"app/files/[path]/[[...rest]]/page.tsx".to_string()));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_match_rejects_duplicate_parameter_names() {
        let root = temp_project("duplicate-param-match");
        write_page(&root, "app/[team]/[team]/page.tsx");
        write_page(&root, "app/docs/[slug]/[...slug]/page.tsx");
        write_page(&root, "app/docs/[slug]/[...rest]/page.tsx");

        assert!(route_match(&root, "/alpha/beta").is_none());
        let docs_match = route_match(&root, "/docs/api/reference")
            .expect("valid catch-all route with distinct parameter names");
        assert_eq!(
            docs_match.path,
            root.join("app/docs/[slug]/[...rest]/page.tsx")
        );
        assert_eq!(docs_match.params.get("slug"), Some(&"api".to_string()));
        assert_eq!(
            docs_match.params.get("rest"),
            Some(&"reference".to_string())
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn discover_page_routes_skips_nonterminal_catch_all_segments() {
        let root = temp_project("nonterminal-catchall-discovery");
        write_page(&root, "app/docs/[...slug]/details/page.tsx");
        write_page(&root, "app/files/[[...path]]/preview/page.tsx");
        write_page(&root, "app/[...slug]/[[...rest]]/page.tsx");
        write_page(&root, "app/docs/[...slug]/page.tsx");
        write_page(&root, "app/docs/[...slug]/(guide)/page.tsx");
        write_page(&root, "app/files/[category]/[[...path]]/page.tsx");

        let routes = discover_page_routes(&root)
            .iter()
            .map(|route| relative_path(&root, &route.path))
            .collect::<Vec<_>>();

        assert_eq!(
            route_path_from_page_source_path("app/docs/[...slug]/details/page.tsx"),
            None
        );
        assert_eq!(
            route_path_from_page_source_path("app/files/[[...path]]/preview/page.tsx"),
            None
        );
        assert_eq!(
            route_path_from_page_source_path("app/[...slug]/[[...rest]]/page.tsx"),
            None
        );
        assert_eq!(
            route_path_from_page_source_path("app/docs/[...slug]/(guide)/page.tsx"),
            Some("/docs/[...slug]".to_string())
        );
        assert!(!routes.contains(&"app/docs/[...slug]/details/page.tsx".to_string()));
        assert!(!routes.contains(&"app/files/[[...path]]/preview/page.tsx".to_string()));
        assert!(!routes.contains(&"app/[...slug]/[[...rest]]/page.tsx".to_string()));
        assert!(routes.contains(&"app/docs/[...slug]/page.tsx".to_string()));
        assert!(routes.contains(&"app/docs/[...slug]/(guide)/page.tsx".to_string()));
        assert!(routes.contains(&"app/files/[category]/[[...path]]/page.tsx".to_string()));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_match_rejects_nonterminal_catch_all_segments() {
        let root = temp_project("nonterminal-catchall-match");
        write_page(&root, "app/docs/[...slug]/details/page.tsx");
        write_page(&root, "app/files/[[...path]]/preview/page.tsx");

        assert!(route_match(&root, "/docs/api/details").is_none());
        assert!(route_match(&root, "/files/a/preview").is_none());
        write_page(&root, "app/files/[...path]/(modal)/page.tsx");
        let files_match =
            route_match(&root, "/files/a/b").expect("catch-all may end before a route group");
        assert_eq!(
            files_match.path,
            root.join("app/files/[...path]/(modal)/page.tsx")
        );
        assert_eq!(files_match.params.get("path"), Some(&"a/b".to_string()));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_match_accepts_next_page_file_extensions() {
        let jsx_root = temp_project("jsx-page");
        write_page(&jsx_root, "app/profile/page.jsx");

        let route_match =
            route_match(&jsx_root, "/profile").expect("match App Router page.jsx route");
        assert_eq!(route_match.path, jsx_root.join("app/profile/page.jsx"));
        let _ = fs::remove_dir_all(jsx_root);

        assert_eq!(
            route_path_from_page_source_path("app/(marketing)/about/page.js"),
            Some("/about".to_string())
        );
        assert_eq!(
            route_path_from_page_source_path("app/page.ts"),
            Some("/".to_string())
        );
    }

    #[test]
    fn route_match_accepts_src_app_root() {
        let root = temp_project("src-app");
        write_page(&root, "src/app/page.tsx");
        write_page(&root, "src/app/blog/[slug]/page.jsx");

        let root_match = route_match(&root, "/").expect("match src/app root page");
        assert_eq!(root_match.path, root.join("src/app/page.tsx"));

        let blog_match = route_match(&root, "/blog/launch").expect("match src/app blog page");
        assert_eq!(blog_match.path, root.join("src/app/blog/[slug]/page.jsx"));
        assert_eq!(blog_match.params.get("slug"), Some(&"launch".to_string()));
        assert_eq!(
            route_path_from_page_source_path("src/app/page.tsx"),
            Some("/".to_string())
        );
        assert_eq!(
            route_path_from_page_source_path("src/app/(marketing)/about/page.jsx"),
            Some("/about".to_string())
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn discover_page_routes_reports_duplicate_public_paths_before_matching() {
        let root = temp_project("discover-duplicates");
        write_page(&root, "app/blog/page.tsx");
        write_page(&root, "app/(marketing)/blog/page.tsx");
        write_page(&root, "app/@modal/blog/page.tsx");
        write_page(&root, "app/docs/[slug]/page.tsx");
        write_page(&root, "src/app/docs/[slug]/page.tsx");
        write_page(&root, "app/_private/page.tsx");
        write_page(&root, "app/(.)photo/page.tsx");

        let routes = discover_page_routes(&root);
        let route_paths = routes
            .iter()
            .map(|route| (route.route_path.as_str(), relative_path(&root, &route.path)))
            .collect::<Vec<_>>();

        assert!(route_paths.contains(&("/blog", "app/blog/page.tsx".to_string())));
        assert!(route_paths.contains(&("/blog", "app/(marketing)/blog/page.tsx".to_string())));
        assert!(route_paths.contains(&("/blog", "app/@modal/blog/page.tsx".to_string())));
        assert!(route_paths.contains(&("/docs/[slug]", "app/docs/[slug]/page.tsx".to_string())));
        assert!(
            route_paths.contains(&("/docs/[slug]", "src/app/docs/[slug]/page.tsx".to_string()))
        );
        assert!(
            !route_paths
                .iter()
                .any(|(_, path)| path == "app/_private/page.tsx")
        );
        assert!(
            !route_paths
                .iter()
                .any(|(_, path)| path == "app/(.)photo/page.tsx")
        );

        let blog = routes
            .iter()
            .find(|route| route.path == root.join("app/blog/page.tsx"))
            .expect("primary blog route");
        assert_eq!(
            blog.shadowed_paths,
            vec![
                root.join("app/(marketing)/blog/page.tsx"),
                root.join("app/@modal/blog/page.tsx")
            ]
        );

        let docs = routes
            .iter()
            .find(|route| route.path == root.join("app/docs/[slug]/page.tsx"))
            .expect("primary docs route");
        assert_eq!(
            docs.shadowed_paths,
            vec![root.join("src/app/docs/[slug]/page.tsx")]
        );

        let shadowed_blog = routes
            .iter()
            .find(|route| route.path == root.join("app/(marketing)/blog/page.tsx"))
            .expect("shadowed route group blog route is still listed");
        assert!(shadowed_blog.shadowed_paths.is_empty());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn discover_page_routes_carries_layout_template_and_boundary_files() {
        let root = temp_project("discover-segment-files");
        write_source(
            &root,
            "app/layout.tsx",
            "export default function Layout() {}",
        );
        write_source(
            &root,
            "app/template.jsx",
            "export default function Template() {}",
        );
        write_source(
            &root,
            "app/loading.ts",
            "export default function Loading() {}",
        );
        write_source(&root, "app/error.js", "export default function Error() {}");
        write_source(
            &root,
            "app/not-found.jsx",
            "export default function NotFound() {}",
        );
        write_source(
            &root,
            "app/(workspace)/layout.js",
            "export default function WorkspaceLayout() {}",
        );
        write_source(
            &root,
            "app/(workspace)/dashboard/template.tsx",
            "export default function DashboardTemplate() {}",
        );
        write_source(
            &root,
            "app/(workspace)/dashboard/settings/loading.tsx",
            "export default function SettingsLoading() {}",
        );
        write_page(&root, "app/(workspace)/dashboard/settings/page.tsx");

        let routes = discover_page_routes(&root);
        let route = routes
            .iter()
            .find(|route| route.path == root.join("app/(workspace)/dashboard/settings/page.tsx"))
            .expect("settings page route");
        let segment_files = route
            .segment_files
            .iter()
            .map(|segment| {
                (
                    segment.kind,
                    relative_path(&root, &segment.path),
                    segment.route_path.as_str(),
                    segment.depth,
                    segment.non_path_segment_count,
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(
            segment_files,
            vec![
                (
                    AppDiscoveredSegmentKind::Layout,
                    "app/layout.tsx".to_string(),
                    "/",
                    0,
                    0
                ),
                (
                    AppDiscoveredSegmentKind::Template,
                    "app/template.jsx".to_string(),
                    "/",
                    0,
                    0
                ),
                (
                    AppDiscoveredSegmentKind::Loading,
                    "app/loading.ts".to_string(),
                    "/",
                    0,
                    0
                ),
                (
                    AppDiscoveredSegmentKind::Error,
                    "app/error.js".to_string(),
                    "/",
                    0,
                    0
                ),
                (
                    AppDiscoveredSegmentKind::NotFound,
                    "app/not-found.jsx".to_string(),
                    "/",
                    0,
                    0
                ),
                (
                    AppDiscoveredSegmentKind::Layout,
                    "app/(workspace)/layout.js".to_string(),
                    "/",
                    0,
                    1
                ),
                (
                    AppDiscoveredSegmentKind::Template,
                    "app/(workspace)/dashboard/template.tsx".to_string(),
                    "/dashboard",
                    1,
                    1
                ),
                (
                    AppDiscoveredSegmentKind::Loading,
                    "app/(workspace)/dashboard/settings/loading.tsx".to_string(),
                    "/dashboard/settings",
                    2,
                    1
                ),
            ]
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn discover_page_route_summaries_are_manifest_ready_and_relative() {
        let root = temp_project("discovery-summary");
        let root_string = root.to_string_lossy().to_string();
        write_source(
            &root,
            "app/layout.tsx",
            "export default function Layout() {}",
        );
        write_source(
            &root,
            "app/blog/template.tsx",
            "export default function BlogTemplate() {}",
        );
        write_page(&root, "app/blog/page.tsx");
        write_page(&root, "app/(marketing)/blog/page.tsx");
        write_page(&root, "src/app/blog/page.tsx");

        let summaries = discover_page_route_summaries(&root);
        let summary = summaries
            .iter()
            .find(|summary| summary.source_path == "app/blog/page.tsx")
            .expect("primary blog route summary");

        assert!(!summary.source_path.contains(root_string.as_str()));
        assert_eq!(summary.route_path, "/blog");
        assert_eq!(summary.root_index, 0);
        assert_eq!(summary.non_path_segment_count, 0);
        assert_eq!(
            summary.shadowed_source_paths,
            vec![
                "app/(marketing)/blog/page.tsx".to_string(),
                "src/app/blog/page.tsx".to_string()
            ]
        );
        assert_eq!(
            summary
                .segment_files
                .iter()
                .map(|segment| {
                    assert!(!segment.source_path.contains(root_string.as_str()));
                    (
                        segment.kind,
                        segment.source_path.as_str(),
                        segment.route_path.as_str(),
                        segment.depth,
                        segment.non_path_segment_count,
                    )
                })
                .collect::<Vec<_>>(),
            vec![
                (
                    AppDiscoveredSegmentKind::Layout,
                    "app/layout.tsx",
                    "/",
                    0,
                    0
                ),
                (
                    AppDiscoveredSegmentKind::Template,
                    "app/blog/template.tsx",
                    "/blog",
                    1,
                    0
                )
            ]
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn discover_page_routes_reports_dynamic_shape_collisions() {
        let root = temp_project("dynamic-shape-collisions");
        write_page(&root, "app/users/[id]/page.tsx");
        write_page(&root, "app/users/[slug]/page.tsx");
        write_page(&root, "app/users/settings/page.tsx");
        write_page(&root, "app/docs/[...parts]/page.tsx");
        write_page(&root, "app/docs/[...slug]/page.tsx");
        write_page(&root, "app/files/[[...path]]/page.tsx");
        write_page(&root, "app/files/[[...segments]]/page.tsx");

        let routes = discover_page_routes(&root);
        let route_for = |source_path: &str| {
            routes
                .iter()
                .find(|route| route.path == root.join(source_path))
                .unwrap_or_else(|| panic!("missing route {source_path}"))
        };

        let users_id = route_for("app/users/[id]/page.tsx");
        assert_eq!(users_id.route_path, "/users/[id]");
        assert_eq!(users_id.route_shape, "/users/[]");
        assert_eq!(
            users_id.shape_collision_paths,
            vec![root.join("app/users/[slug]/page.tsx")]
        );
        let users_slug = route_for("app/users/[slug]/page.tsx");
        assert_eq!(
            users_slug.shape_collision_paths,
            vec![root.join("app/users/[id]/page.tsx")]
        );

        let users_settings = route_for("app/users/settings/page.tsx");
        assert_eq!(users_settings.route_shape, "/users/settings");
        assert!(users_settings.shape_collision_paths.is_empty());

        let docs_parts = route_for("app/docs/[...parts]/page.tsx");
        assert_eq!(docs_parts.route_shape, "/docs/[...]");
        assert_eq!(
            docs_parts.shape_collision_paths,
            vec![root.join("app/docs/[...slug]/page.tsx")]
        );
        let docs_slug = route_for("app/docs/[...slug]/page.tsx");
        assert_eq!(
            docs_slug.shape_collision_paths,
            vec![root.join("app/docs/[...parts]/page.tsx")]
        );

        let files_path = route_for("app/files/[[...path]]/page.tsx");
        assert_eq!(files_path.route_shape, "/files/[[...]]");
        assert_eq!(
            files_path.shape_collision_paths,
            vec![root.join("app/files/[[...segments]]/page.tsx")]
        );
        let files_segments = route_for("app/files/[[...segments]]/page.tsx");
        assert_eq!(
            files_segments.shape_collision_paths,
            vec![root.join("app/files/[[...path]]/page.tsx")]
        );
        assert_eq!(
            route_shape_from_page_source_path("app/(marketing)/@modal/users/[id]/page.tsx"),
            Some("/users/[]".to_string())
        );
        assert_eq!(
            route_shape_from_page_source_path("app/docs/[...slug]/page.tsx"),
            Some("/docs/[...]".to_string())
        );
        assert_eq!(
            route_shape_from_page_source_path("app/files/[[...path]]/page.tsx"),
            Some("/files/[[...]]".to_string())
        );

        let summaries = discover_page_route_summaries(&root);
        let users_summary = summaries
            .iter()
            .find(|summary| summary.source_path == "app/users/[id]/page.tsx")
            .expect("users id summary");
        assert_eq!(users_summary.route_shape, "/users/[]");
        assert_eq!(
            users_summary.shape_collision_source_paths,
            vec!["app/users/[slug]/page.tsx".to_string()]
        );
        let users_slug_summary = summaries
            .iter()
            .find(|summary| summary.source_path == "app/users/[slug]/page.tsx")
            .expect("users slug summary");
        assert_eq!(
            users_slug_summary.shape_collision_source_paths,
            vec!["app/users/[id]/page.tsx".to_string()]
        );

        let settings_summary = summaries
            .iter()
            .find(|summary| summary.source_path == "app/users/settings/page.tsx")
            .expect("settings summary");
        assert_eq!(settings_summary.route_shape, "/users/settings");
        assert!(settings_summary.shape_collision_source_paths.is_empty());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn discover_page_routes_reports_all_dynamic_shape_collision_peers() {
        let root = temp_project("all-dynamic-shape-collision-peers");
        write_page(&root, "app/(admin)/teams/[team]/page.tsx");
        write_page(&root, "app/(marketing)/teams/[slug]/page.tsx");

        let routes = discover_page_routes(&root);
        let admin = routes
            .iter()
            .find(|route| route.path == root.join("app/(admin)/teams/[team]/page.tsx"))
            .expect("admin team route");
        let marketing = routes
            .iter()
            .find(|route| route.path == root.join("app/(marketing)/teams/[slug]/page.tsx"))
            .expect("marketing team route");

        assert_eq!(admin.route_shape, "/teams/[]");
        assert_eq!(marketing.route_shape, "/teams/[]");
        assert_eq!(
            admin.shape_collision_paths,
            vec![root.join("app/(marketing)/teams/[slug]/page.tsx")]
        );
        assert_eq!(
            marketing.shape_collision_paths,
            vec![root.join("app/(admin)/teams/[team]/page.tsx")]
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn discover_page_route_summaries_carry_specificity_for_manifest_sorting() {
        let root = temp_project("discovery-specificity");
        write_page(&root, "app/docs/page.tsx");
        write_page(&root, "app/docs/[slug]/page.tsx");
        write_page(&root, "app/docs/[...slug]/page.tsx");
        write_page(&root, "app/docs/[[...slug]]/page.tsx");
        write_page(&root, "app/(shop)/@modal/products/[id]/page.tsx");

        let summaries = discover_page_route_summaries(&root);
        let specificity_for = |source_path: &str| {
            summaries
                .iter()
                .find(|summary| summary.source_path == source_path)
                .map(|summary| summary.specificity.clone())
                .expect("route summary")
        };

        assert_eq!(
            specificity_for("app/docs/page.tsx"),
            AppDiscoveredRouteSpecificity {
                segment_kinds: vec![AppDiscoveredRouteSegmentKind::Static],
                static_segment_count: 1,
                dynamic_segment_count: 0,
                catch_all_segment_count: 0,
                optional_catch_all_segment_count: 0,
                visible_segment_count: 1,
                precedence_score: 4,
            }
        );
        assert_eq!(
            specificity_for("app/docs/[slug]/page.tsx"),
            AppDiscoveredRouteSpecificity {
                segment_kinds: vec![
                    AppDiscoveredRouteSegmentKind::Static,
                    AppDiscoveredRouteSegmentKind::Dynamic,
                ],
                static_segment_count: 1,
                dynamic_segment_count: 1,
                catch_all_segment_count: 0,
                optional_catch_all_segment_count: 0,
                visible_segment_count: 2,
                precedence_score: 6,
            }
        );
        assert_eq!(
            specificity_for("app/docs/[...slug]/page.tsx"),
            AppDiscoveredRouteSpecificity {
                segment_kinds: vec![
                    AppDiscoveredRouteSegmentKind::Static,
                    AppDiscoveredRouteSegmentKind::CatchAll,
                ],
                static_segment_count: 1,
                dynamic_segment_count: 0,
                catch_all_segment_count: 1,
                optional_catch_all_segment_count: 0,
                visible_segment_count: 2,
                precedence_score: 5,
            }
        );
        assert_eq!(
            specificity_for("app/docs/[[...slug]]/page.tsx"),
            AppDiscoveredRouteSpecificity {
                segment_kinds: vec![
                    AppDiscoveredRouteSegmentKind::Static,
                    AppDiscoveredRouteSegmentKind::OptionalCatchAll,
                ],
                static_segment_count: 1,
                dynamic_segment_count: 0,
                catch_all_segment_count: 0,
                optional_catch_all_segment_count: 1,
                visible_segment_count: 2,
                precedence_score: 4,
            }
        );
        assert_eq!(
            specificity_for("app/(shop)/@modal/products/[id]/page.tsx"),
            AppDiscoveredRouteSpecificity {
                segment_kinds: vec![
                    AppDiscoveredRouteSegmentKind::Static,
                    AppDiscoveredRouteSegmentKind::Dynamic,
                ],
                static_segment_count: 1,
                dynamic_segment_count: 1,
                catch_all_segment_count: 0,
                optional_catch_all_segment_count: 0,
                visible_segment_count: 2,
                precedence_score: 6,
            }
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_match_prefers_root_app_over_src_app_on_ties() {
        let root = temp_project("dual-app-roots");
        write_page(&root, "app/page.tsx");
        write_page(&root, "src/app/page.tsx");
        write_page(&root, "app/blog/page.tsx");
        write_page(&root, "src/app/blog/page.tsx");

        let root_match = route_match(&root, "/").expect("match root app route");
        assert_eq!(root_match.path, root.join("app/page.tsx"));

        let blog_match = route_match(&root, "/blog").expect("match nested app route");
        assert_eq!(blog_match.path, root.join("app/blog/page.tsx"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_match_prefers_exact_route_over_optional_catchall() {
        let root = temp_project("exact-over-optional-catchall");
        write_page(&root, "app/docs/page.tsx");
        write_page(&root, "app/docs/[[...slug]]/page.tsx");

        let exact_match = route_match(&root, "/docs").expect("match exact docs route");
        assert_eq!(exact_match.path, root.join("app/docs/page.tsx"));
        assert!(exact_match.params.is_empty());

        let catchall_match =
            route_match(&root, "/docs/api/reference").expect("match optional catch-all route");
        assert_eq!(
            catchall_match.path,
            root.join("app/docs/[[...slug]]/page.tsx")
        );
        assert_eq!(
            catchall_match.params.get("slug"),
            Some(&"api/reference".to_string())
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_match_prefers_static_prefix_catchall_over_equal_score_dynamic_route() {
        let root = temp_project("static-prefix-catchall");
        write_page(&root, "app/shop/[category]/[item]/page.tsx");
        write_page(&root, "app/shop/sale/[...slug]/page.tsx");

        let route_match = route_match(&root, "/shop/sale/today")
            .expect("static prefix catch-all should match request");

        assert_eq!(
            route_match.path,
            root.join("app/shop/sale/[...slug]/page.tsx")
        );
        assert_eq!(route_match.params.get("slug"), Some(&"today".to_string()));
        assert!(!route_match.params.contains_key("category"));
        assert!(!route_match.params.contains_key("item"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_match_prefers_required_catchall_over_optional_catchall_with_remainder() {
        let root = temp_project("required-over-optional-catchall");
        write_page(&root, "app/docs/[...slug]/page.tsx");
        write_page(&root, "app/docs/[[...slug]]/page.tsx");

        let matched_route = route_match(&root, "/docs/api/reference")
            .expect("required catch-all should match non-empty remainder");

        assert_eq!(matched_route.path, root.join("app/docs/[...slug]/page.tsx"));
        assert_eq!(
            matched_route.params.get("slug"),
            Some(&"api/reference".to_string())
        );

        let root_only_match =
            route_match(&root, "/docs").expect("optional catch-all should match empty remainder");
        assert_eq!(
            root_only_match.path,
            root.join("app/docs/[[...slug]]/page.tsx")
        );
        assert_eq!(root_only_match.params.get("slug"), Some(&"".to_string()));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_match_prefers_visible_route_over_route_group_duplicate() {
        let root = temp_project("visible-over-route-group");
        write_page(&root, "app/blog/[slug]/page.tsx");
        write_page(&root, "app/(marketing)/blog/[slug]/page.tsx");

        let route_match = route_match(&root, "/blog/launch")
            .expect("match visible duplicate over route group duplicate");
        assert_eq!(route_match.path, root.join("app/blog/[slug]/page.tsx"));
        assert_eq!(route_match.params.get("slug"), Some(&"launch".to_string()));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_match_reports_shadowed_route_group_duplicates() {
        let root = temp_project("shadowed-route-group");
        write_page(&root, "app/blog/page.tsx");
        write_page(&root, "app/(marketing)/blog/page.tsx");

        let route_match =
            route_match(&root, "/blog").expect("match visible duplicate over route group");
        assert_eq!(route_match.path, root.join("app/blog/page.tsx"));
        assert_eq!(
            route_match.shadowed_paths,
            vec![root.join("app/(marketing)/blog/page.tsx")]
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_match_prefers_visible_route_over_parallel_slot_duplicate() {
        let root = temp_project("visible-over-parallel-slot");
        write_page(&root, "app/settings/page.tsx");
        write_page(&root, "app/@modal/settings/page.tsx");

        let route_match =
            route_match(&root, "/settings").expect("match visible duplicate over parallel slot");
        assert_eq!(route_match.path, root.join("app/settings/page.tsx"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_match_reports_shadowed_parallel_slot_duplicates() {
        let root = temp_project("shadowed-parallel-slot");
        write_page(&root, "app/settings/page.tsx");
        write_page(&root, "app/@modal/settings/page.tsx");

        let route_match =
            route_match(&root, "/settings").expect("match visible duplicate over parallel slot");
        assert_eq!(route_match.path, root.join("app/settings/page.tsx"));
        assert_eq!(
            route_match.shadowed_paths,
            vec![root.join("app/@modal/settings/page.tsx")]
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_path_from_page_source_path_skips_non_path_segments() {
        assert_eq!(
            route_path_from_page_source_path("app/@modal/settings/page.tsx"),
            Some("/settings".to_string())
        );
        assert_eq!(
            route_path_from_page_source_path("app/(workspace)/@modal/dashboard/[team]/page.tsx"),
            Some("/dashboard/[team]".to_string())
        );
        assert_eq!(
            route_path_from_page_source_path("app/page.tsx"),
            Some("/".to_string())
        );
    }
}
