use std::path::Path;

use dx_compiler::delivery::DxReactServerSource;
use serde_json::json;

use crate::app_router_segments::UnsupportedAppRouteSegmentReason;
use crate::error::{DxError, DxResult};

use super::Cli;
use super::app_page_route_diagnostics;
use super::app_page_routes;
use super::app_route_diagnostics;
use super::app_router_build_output::{
    DxAppClientIslandsOutputInput, DxAppGeneratedStyleAssetsOutputInput,
    DxAppRouterExecutionOutputInput, DxAppServerDataOutputInput, DxAppStreamingPlanOutputInput,
    write_app_client_islands_contract, write_app_generated_style_assets,
    write_app_router_execution_contract, write_app_server_data_contract, write_app_streaming_plan,
};
use super::app_router_paths;
use super::app_router_runtime_command::{
    compile_app_route_proof, react_app_segment_sources, react_route_component_sources,
};
use super::app_segment_files;

const APP_ROUTE_DISCOVERY_SUMMARY_JSON: &str = ".dx/build-cache/app-route-discovery.json";

pub(super) struct DxAppRouterBuildCommandInput<'a> {
    pub(super) cwd: &'a Path,
    pub(super) output_dir: &'a Path,
    pub(super) server_sources: &'a [DxReactServerSource],
}

#[derive(Default)]
pub(super) struct DxAppRouterBuildCommandOutput {
    pub(super) app_routes_compiled: usize,
    pub(super) app_router_execution_contracts_compiled: usize,
    pub(super) client_islands_compiled: usize,
    pub(super) generated_style_assets_compiled: usize,
    pub(super) streaming_plans_compiled: usize,
    pub(super) server_data_entries_compiled: usize,
    pub(super) entrypoint_compiled: bool,
    pub(super) total_size: usize,
}

pub(super) fn compile_app_router_build_outputs(
    input: DxAppRouterBuildCommandInput<'_>,
) -> DxResult<DxAppRouterBuildCommandOutput> {
    let app_route_roots = app_segment_files::app_route_roots(input.cwd);
    if app_route_roots.is_empty() {
        remove_stale_app_route_discovery_summary(input.output_dir)?;
        return Ok(DxAppRouterBuildCommandOutput::default());
    }

    app_route_diagnostics::validate_app_route_handlers(input.cwd)?;
    let route_summaries = app_page_routes::discover_page_route_summaries(input.cwd);
    let skipped_route_summaries =
        app_page_route_diagnostics::discover_skipped_page_route_summaries(input.cwd);
    write_app_route_discovery_summary(
        input.output_dir,
        &route_summaries,
        &skipped_route_summaries,
    )?;

    let mut output = DxAppRouterBuildCommandOutput::default();

    for app_dir in app_route_roots {
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
                        let relative = Cli::relative_cli_path(input.cwd, entry.path());
                        app_page_routes::route_path_from_page_source_path(&relative).is_some()
                    }
            })
        {
            compile_app_router_page(
                input.cwd,
                input.output_dir,
                input.server_sources,
                entry.path(),
                &mut output,
            )?;
        }
    }

    output.entrypoint_compiled = is_app_router_entrypoint_compiled(input.cwd, input.output_dir);
    Ok(output)
}

fn remove_stale_app_route_discovery_summary(output_dir: &Path) -> DxResult<()> {
    let path = output_dir.join(APP_ROUTE_DISCOVERY_SUMMARY_JSON);
    match std::fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(DxError::IoError {
            path: Some(path),
            message: error.to_string(),
        }),
    }
}

fn write_app_route_discovery_summary(
    output_dir: &Path,
    route_summaries: &[app_page_routes::AppDiscoveredRouteSummary],
    skipped_route_summaries: &[app_page_route_diagnostics::AppSkippedPageRouteSummary],
) -> DxResult<()> {
    let skipped_routes = skipped_route_summaries
        .iter()
        .map(|summary| {
            json!({
                "source_path": &summary.source_path,
                "reason": skipped_route_reason(summary.reason),
                "segment": &summary.segment,
            })
        })
        .collect::<Vec<_>>();
    let discovery = json!({
        "version": 1,
        "format": 1,
        "schema": "dx.app-router.route-discovery",
        "routes": route_summaries,
        "route_count": route_summaries.len(),
        "skipped_routes": skipped_routes,
        "skipped_route_count": skipped_route_summaries.len(),
    });
    let path = output_dir.join(APP_ROUTE_DISCOVERY_SUMMARY_JSON);
    let discovery_json = serde_json::to_string_pretty(&discovery).map_err(|error| {
        DxError::ConfigValidationError {
            message: error.to_string(),
            field: Some("app-router route discovery".to_string()),
        }
    })?;
    std::fs::write(&path, discovery_json).map_err(|error| DxError::IoError {
        path: Some(path),
        message: error.to_string(),
    })
}

fn skipped_route_reason(reason: UnsupportedAppRouteSegmentReason) -> &'static str {
    match reason {
        UnsupportedAppRouteSegmentReason::PrivateFolder => "private-folder",
        UnsupportedAppRouteSegmentReason::InterceptingRoute => "intercepting-route",
        UnsupportedAppRouteSegmentReason::MalformedSegment => "malformed-segment",
        UnsupportedAppRouteSegmentReason::DuplicateParamName => "duplicate-param-name",
        UnsupportedAppRouteSegmentReason::NonTerminalCatchAll => "non-terminal-catch-all",
    }
}

pub(super) fn is_app_router_entrypoint_compiled(cwd: &Path, output_dir: &Path) -> bool {
    output_dir.join("app/index.html").is_file()
        && app_segment_files::app_route_roots(cwd)
            .into_iter()
            .any(|app_root| {
                std::fs::read_dir(app_root)
                    .ok()
                    .into_iter()
                    .flat_map(|entries| entries.filter_map(Result::ok))
                    .any(|entry| {
                        entry.path().is_file()
                            && entry
                                .file_name()
                                .to_str()
                                .is_some_and(app_segment_files::is_app_page_file_name)
                    })
            })
}

fn compile_app_router_page(
    cwd: &Path,
    output_dir: &Path,
    server_sources: &[DxReactServerSource],
    page_path: &Path,
    output: &mut DxAppRouterBuildCommandOutput,
) -> DxResult<()> {
    app_route_diagnostics::validate_app_route_source(cwd, page_path)?;
    let proof = compile_app_route_proof(cwd, page_path)
        .map_err(|error| app_route_diagnostics::app_route_compile_error(cwd, page_path, error))?;
    let app_output_dir = output_dir.join(app_router_paths::app_build_output_dir(cwd, page_path));
    std::fs::create_dir_all(&app_output_dir).map_err(|error| DxError::IoError {
        path: Some(app_output_dir.clone()),
        message: error.to_string(),
    })?;

    let generated_style_bytes =
        write_app_generated_style_assets(DxAppGeneratedStyleAssetsOutputInput {
            output_dir,
            proof: &proof,
        })?;
    output.generated_style_assets_compiled += proof.generated_styles.len();

    let route = app_router_paths::route_from_app_path(cwd, page_path);
    let html_path = app_output_dir.join("index.html");
    std::fs::write(&html_path, &proof.fallback.html).map_err(|error| DxError::IoError {
        path: Some(html_path),
        message: error.to_string(),
    })?;

    let packet_path = app_output_dir.join("index.dxpk");
    let packet_bytes = if route_public_packet_required(&proof) {
        std::fs::write(&packet_path, &proof.packet.encoded).map_err(|error| DxError::IoError {
            path: Some(packet_path),
            message: error.to_string(),
        })?;
        proof.packet.encoded.len()
    } else {
        remove_stale_route_packet(&packet_path)?;
        0
    };

    let graph_path = app_output_dir.join("page-graph.json");
    let page_graph_json = serde_json::to_string_pretty(&proof.page_graph).map_err(|error| {
        DxError::ConfigValidationError {
            message: error.to_string(),
            field: Some("app-router page graph".to_string()),
        }
    })?;
    std::fs::write(&graph_path, page_graph_json).map_err(|error| DxError::IoError {
        path: Some(graph_path),
        message: error.to_string(),
    })?;

    let streaming_plan_written = write_app_streaming_plan(DxAppStreamingPlanOutputInput {
        app_output_dir: &app_output_dir,
        proof: &proof,
    })?;
    if !streaming_plan_written {
        remove_stale_route_artifact(&app_output_dir.join("streaming-plan.json"))?;
    } else {
        output.streaming_plans_compiled += 1;
    }

    let route_source_path = Cli::relative_cli_path(cwd, page_path);
    let route_segments = react_app_segment_sources(cwd, page_path);
    let route_source = std::fs::read_to_string(page_path).map_err(|error| DxError::IoError {
        path: Some(page_path.to_path_buf()),
        message: error.to_string(),
    })?;
    let route_components =
        react_route_component_sources(cwd, &route_source_path, &route_source, &route_segments);

    write_app_router_execution_contract(DxAppRouterExecutionOutputInput {
        cwd,
        app_route_path: page_path,
        app_output_dir: &app_output_dir,
        route: &route,
        route_source_path: &route_source_path,
        segments: route_segments.clone(),
        proof: &proof,
        source_manifest_hash: Cli::source_manifest_hash(cwd),
        node_modules_present: cwd.join("node_modules").exists(),
        server_sources,
    })?;
    output.app_router_execution_contracts_compiled += 1;

    let islands_compiled = write_app_client_islands_contract(DxAppClientIslandsOutputInput {
        app_route_path: page_path,
        app_output_dir: &app_output_dir,
        route: &route,
        route_source_path: &route_source_path,
        segments: route_segments,
        components: route_components,
        proof: &proof,
    })?;
    if islands_compiled == 0 {
        remove_stale_route_artifacts(
            &app_output_dir,
            &["client-islands.json", "client-islands.js"],
        )?;
    }
    output.client_islands_compiled += islands_compiled;

    output.server_data_entries_compiled +=
        write_app_server_data_contract(DxAppServerDataOutputInput {
            app_route_path: page_path,
            app_output_dir: &app_output_dir,
            route: &route,
            route_source_path: &route_source_path,
            server_sources,
        })?;

    write_native_root_entrypoint_if_root_route(output_dir, &app_output_dir, &route)?;

    let route_size = proof.fallback.bytes + packet_bytes + generated_style_bytes;
    output.total_size += route_size;
    output.app_routes_compiled += 1;
    eprintln!("{} Compiled {} {}", console::style("✓").green(), console::style(&route).cyan(), console::style(format!("({} bytes)", route_size)).dim());

    Ok(())
}

fn write_native_root_entrypoint_if_root_route(
    output_dir: &Path,
    app_output_dir: &Path,
    route: &str,
) -> DxResult<()> {
    if route != "/" {
        return Ok(());
    }

    let app_entrypoint = app_output_dir.join("index.html");
    let native_entrypoint = output_dir.join("index.html");
    std::fs::copy(&app_entrypoint, &native_entrypoint).map_err(|error| DxError::IoError {
        path: Some(native_entrypoint),
        message: error.to_string(),
    })?;

    Ok(())
}

fn route_public_packet_required(proof: &dx_compiler::delivery::DxReactAppRouteProof) -> bool {
    !proof
        .route_unit
        .runtime_report
        .tiny_static_route_proof
        .no_js_capable
}

fn remove_stale_route_packet(packet_path: &Path) -> DxResult<()> {
    remove_stale_route_artifact(packet_path)
}

fn remove_stale_route_artifacts(app_output_dir: &Path, file_names: &[&str]) -> DxResult<()> {
    for file_name in file_names {
        remove_stale_route_artifact(&app_output_dir.join(file_name))?;
    }
    Ok(())
}

fn remove_stale_route_artifact(path: &Path) -> DxResult<()> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(DxError::IoError {
            path: Some(path.to_path_buf()),
            message: error.to_string(),
        }),
    }
}
