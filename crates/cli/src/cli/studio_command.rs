use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use chrono::Utc;

use crate::config::DxConfig;
use crate::error::DxResult;
use crate::project::Project;

use super::app_page_routes::{
    self, AppDiscoveredRouteSegmentKind, AppDiscoveredRouteSummary, AppDiscoveredSegmentKind,
};
use super::forge_error;
use super::options::DxOutputFormat;
use super::studio_json_surface::{
    DxStudioJsonSurfaceArgs, parse_studio_json_surface_args, write_or_print_studio_json_surface,
};
use super::studio_manifest::{
    build_studio_preview_manifest, build_www_routes_report, studio_preview_manifest_markdown,
    studio_preview_manifest_terminal, www_routes_markdown, www_routes_terminal,
};

pub(super) fn cmd_routes(cwd: &Path, args: &[String]) -> DxResult<()> {
    let DxStudioJsonSurfaceArgs {
        format,
        output,
        quiet,
    } = parse_studio_json_surface_args(cwd, args, "routes")?;
    let mut report = build_www_routes_report(&Utc::now().to_rfc3339());
    attach_local_www_routes(cwd, &mut report)?;
    let rendered = match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Terminal => www_routes_terminal(&report),
        DxOutputFormat::Markdown => www_routes_markdown(&report),
    };
    write_or_print_studio_json_surface(output, &rendered, quiet)
}

pub(super) fn cmd_preview_manifest(cwd: &Path, args: &[String]) -> DxResult<()> {
    let DxStudioJsonSurfaceArgs {
        format,
        output,
        quiet,
    } = parse_studio_json_surface_args(cwd, args, "preview-manifest")?;
    let report = build_studio_preview_manifest(&Utc::now().to_rfc3339());
    let rendered = match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Terminal => studio_preview_manifest_terminal(&report),
        DxOutputFormat::Markdown => studio_preview_manifest_markdown(&report),
    };
    write_or_print_studio_json_surface(output, &rendered, quiet)
}

pub(super) fn attach_local_www_routes(cwd: &Path, report: &mut serde_json::Value) -> DxResult<()> {
    let config = DxConfig::load_project(cwd)?;
    if !cwd.join("app").exists() && !cwd.join(&config.routing.pages_dir).exists() {
        return Ok(());
    }

    let project = Project::scan(cwd, config.clone())?;
    let local_routes = local_www_routes_report_entries(&project, &config);
    if local_routes.is_empty() {
        return Ok(());
    }

    let existing_routes = report
        .get_mut("routes")
        .and_then(|value| value.as_array_mut())
        .map(std::mem::take)
        .unwrap_or_default();
    let mut merged_routes = BTreeMap::new();

    for route in existing_routes {
        if let Some(route_key) = route.get("route").and_then(|value| value.as_str()) {
            merged_routes.insert(route_key.to_string(), route);
        }
    }

    for route in local_routes {
        if let Some(route_key) = route.get("route").and_then(|value| value.as_str()) {
            merged_routes.insert(route_key.to_string(), route);
        }
    }

    let routes: Vec<_> = merged_routes.into_values().collect();
    report["source"] = serde_json::json!("dx-www-project-scan+dx-studio-preview-manifest");
    report["local_project"] = serde_json::json!({
        "root": cwd.display().to_string(),
        "app_dir_present": cwd.join("app").exists(),
        "pages_dir": config.routing.pages_dir,
    });
    report["route_count"] = serde_json::json!(routes.len());
    report["routes"] = serde_json::json!(routes);
    Ok(())
}

fn local_www_routes_report_entries(project: &Project, config: &DxConfig) -> Vec<serde_json::Value> {
    let base_url = config.dev.server_url().trim_end_matches('/').to_string();
    let app_router_summaries = app_router_route_summaries_by_source(&project.root);
    let app_router_boundary_sources = app_router_boundary_source_paths(&app_router_summaries);
    let mut routes = Vec::new();

    for page in &project.pages {
        let page_source_path = project_relative_slash_path(&project.root, &page.path);
        if app_router_boundary_sources.contains(&page_source_path) {
            continue;
        }

        let mut route = serde_json::json!({
            "route": page.route_path,
            "label": page.route_path,
            "role": if page.is_special { "special-page" } else { "app-page" },
            "source": "dx-www-project-scan",
            "source_files": [page_source_path.clone()],
            "dynamic": page.is_dynamic,
            "catch_all": page.is_catch_all,
            "params": page.params,
            "preview": {
                "url": route_preview_url(&base_url, &page.route_path),
                "hot_reload_target": format!("route:{}", page.route_path),
            },
        });
        if let Some(summary) = app_router_summaries.get(&page_source_path) {
            route["app_router"] = app_router_summary_json(summary);
        }
        routes.push(route);
    }

    for api in &project.api_routes {
        routes.push(serde_json::json!({
            "route": api.endpoint,
            "label": api.endpoint,
            "role": "route-handler",
            "source": "dx-www-project-scan",
            "source_files": [project_relative_slash_path(&project.root, &api.path)],
            "dynamic": api.is_dynamic,
            "catch_all": false,
            "params": api.params,
            "language": api.language,
            "preview": {
                "url": route_preview_url(&base_url, &api.endpoint),
                "hot_reload_target": format!("route:{}", api.endpoint),
            },
        }));
    }

    routes
}

fn app_router_route_summaries_by_source(
    project_root: &Path,
) -> BTreeMap<String, AppDiscoveredRouteSummary> {
    app_page_routes::discover_page_route_summaries(project_root)
        .into_iter()
        .map(|summary| (summary.source_path.clone(), summary))
        .collect()
}

fn app_router_boundary_source_paths(
    summaries: &BTreeMap<String, AppDiscoveredRouteSummary>,
) -> BTreeSet<String> {
    summaries
        .values()
        .flat_map(|summary| summary.segment_files.iter())
        .map(|segment| segment.source_path.clone())
        .collect()
}

fn app_router_summary_json(summary: &AppDiscoveredRouteSummary) -> serde_json::Value {
    serde_json::json!({
        "schema": "dx.app-router.route-report",
        "format": 1,
        "source": "dx-www-app-page-route-discovery",
        "source_owned_app_router": true,
        "node_modules_required": false,
        "full_next_runtime": false,
        "next_internals_required": false,
        "route_path": summary.route_path,
        "route_shape": summary.route_shape,
        "root_index": summary.root_index,
        "non_path_segment_count": summary.non_path_segment_count,
        "specificity": {
            "segment_kinds": summary
                .specificity
                .segment_kinds
                .iter()
                .map(|kind| app_discovered_route_segment_kind_label(*kind))
                .collect::<Vec<_>>(),
            "static_segment_count": summary.specificity.static_segment_count,
            "dynamic_segment_count": summary.specificity.dynamic_segment_count,
            "catch_all_segment_count": summary.specificity.catch_all_segment_count,
            "optional_catch_all_segment_count": summary
                .specificity
                .optional_catch_all_segment_count,
            "visible_segment_count": summary.specificity.visible_segment_count,
            "precedence_score": summary.specificity.precedence_score,
        },
        "segment_file_count": summary.segment_files.len(),
        "segment_files": summary
            .segment_files
            .iter()
            .map(|segment| {
                serde_json::json!({
                    "kind": app_discovered_segment_kind_label(segment.kind),
                    "source_path": segment.source_path,
                    "route_path": segment.route_path,
                    "depth": segment.depth,
                    "non_path_segment_count": segment.non_path_segment_count,
                })
            })
            .collect::<Vec<_>>(),
        "shadowed_source_paths": summary.shadowed_source_paths,
        "shape_collision_source_paths": summary.shape_collision_source_paths,
    })
}

fn app_discovered_segment_kind_label(kind: AppDiscoveredSegmentKind) -> &'static str {
    match kind {
        AppDiscoveredSegmentKind::Layout => "layout",
        AppDiscoveredSegmentKind::Template => "template",
        AppDiscoveredSegmentKind::Loading => "loading",
        AppDiscoveredSegmentKind::Error => "error",
        AppDiscoveredSegmentKind::NotFound => "not-found",
    }
}

fn app_discovered_route_segment_kind_label(kind: AppDiscoveredRouteSegmentKind) -> &'static str {
    match kind {
        AppDiscoveredRouteSegmentKind::Static => "static",
        AppDiscoveredRouteSegmentKind::Dynamic => "dynamic",
        AppDiscoveredRouteSegmentKind::CatchAll => "catch-all",
        AppDiscoveredRouteSegmentKind::OptionalCatchAll => "optional-catch-all",
    }
}

fn route_preview_url(base_url: &str, route: &str) -> String {
    if route == "/" {
        format!("{base_url}/")
    } else {
        format!("{base_url}{route}")
    }
}

fn project_relative_slash_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
