use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::next_rust::DX_TURBOPACK_CORE_GRAPH_CONCEPTS;

use super::ecosystem::relative_project_path;
use super::ecosystem_invalidation::build_graph_invalidation;
use super::graph::{
    SourceBuildAsset, SourceBuildContentDocument, SourceBuildManifest, SourceBuildModuleChunk,
    SourceBuildRoute, SourceBuildRouteHandler, SourceBuildRouteOutput, SourceBuildStyle,
    normalize_path,
};
use super::module_resolver_config::RESOLVER_SOURCE_ADAPTER_BOUNDARY;
use super::server_data::SourceBuildServerDataRoute;

pub fn build_graph_receipt(
    project_root: &Path,
    graph_receipt_path: &Path,
    manifest: &SourceBuildManifest,
    changed_paths: &[PathBuf],
) -> Value {
    let nodes = graph_nodes(
        &manifest.routes,
        &manifest.route_handlers,
        &manifest.route_outputs,
        &manifest.server_data_routes,
        &manifest.styles,
        &manifest.content_documents,
        &manifest.assets,
    );
    let edges = graph_edges(
        project_root,
        &manifest.routes,
        &manifest.route_handlers,
        &manifest.route_outputs,
        &manifest.server_data_routes,
        &manifest.styles,
        &manifest.assets,
    );
    let invalidation = build_graph_invalidation(project_root, &nodes, &edges, changed_paths);

    json!({
        "schema": "dx.build.graph",
        "format": 1,
        "generatedAt": chrono::Utc::now().to_rfc3339(),
        "projectRoot": manifest.project_root,
        "receiptPath": normalize_path(graph_receipt_path),
        "names": {
            "buildGraph": "dx.build.graph",
            "wwwModuleGraph": "dx.www.moduleGraph",
            "forgeSourceGraph": "dx.forge.sourceGraph"
        },
        "positioning": {
            "rolldownPublicDependency": false,
            "turbopackPublicDependency": false,
            "turbopackCoreAdapterBoundary": true,
            "dxSourceOwnedBuildEngine": true,
            "nodeModulesRequired": false
        },
        "consumers": {
            "dxCli": "dx build and future dx graph --json",
            "dxWww": ".dx/receipts/build/latest.json",
            "zedPreview": "read graph nodes, route shell chunks, and receipt paths without executing node_modules"
        },
        "graph": {
            "nodes": nodes,
            "edges": edges
        },
        "invalidation": invalidation,
        "coreConceptMap": turbopack_core_concept_map(),
        "provenance": manifest_provenance()
    })
}

pub fn build_graph_consumer_snapshot(graph_receipt: &Value) -> Value {
    let nodes = graph_receipt["graph"]["nodes"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let edges = graph_receipt["graph"]["edges"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let mut node_kind_counts = BTreeMap::new();
    for node in &nodes {
        let kind = node["kind"].as_str().unwrap_or("unknown").to_string();
        *node_kind_counts.entry(kind).or_insert(0usize) += 1;
    }
    let style_optimization = style_optimization_summary(&nodes);
    let image_optimization = image_optimization_summary(&nodes, &edges);
    let server_data = server_data_summary(&nodes, &edges);
    let content_pipeline = content_pipeline_summary(&nodes);
    let ecmascript_analysis = ecmascript_analysis_summary(&nodes);

    json!({
        "schema": "dx.build.graph.consumerSnapshot",
        "sourceSchema": graph_receipt["schema"],
        "generatedAt": chrono::Utc::now().to_rfc3339(),
        "coreConceptMap": summarize_core_concept_map(graph_receipt["coreConceptMap"].as_array()),
        "graph": {
            "nodeCount": nodes.len(),
            "edgeCount": edges.len(),
            "nodeKindCounts": node_kind_counts,
            "styleOptimization": style_optimization,
            "imageOptimization": image_optimization,
            "serverData": server_data,
            "contentPipeline": content_pipeline,
            "ecmascriptAnalysis": ecmascript_analysis
        },
        "invalidation": graph_receipt["invalidation"],
        "consumers": {
            "zedPreview": {
                "primaryField": "invalidation.affectedNodeIds",
                "fallbackField": "graph.nodeKindCounts",
                "routeShellChunkKind": "route-shell-chunk",
                "sourceModuleKind": "source-module",
                "sourceModuleChunkKind": "source-module-chunk"
            },
            "dxCli": {
                "primaryField": "graph.nodeKindCounts"
            }
        }
    })
}

fn image_optimization_summary(nodes: &[Value], edges: &[Value]) -> Value {
    let mut image_asset_count = 0usize;
    let mut metadata_asset_count = 0usize;
    let mut optimized_variant_count = 0u64;
    let mut placeholder_count = 0usize;
    let mut placeholder_artifact_count = 0usize;
    let mut placeholder_artifact_bytes = 0u64;
    let mut placeholder_artifact_outputs = Vec::new();
    let mut route_reference_count = 0usize;
    let mut style_reference_count = 0usize;
    let mut format_counts = BTreeMap::new();
    let mut dimension_source_counts = BTreeMap::new();

    for node in nodes {
        if node["kind"] == "image-placeholder-asset" {
            placeholder_artifact_count += 1;
            placeholder_artifact_bytes += node["bytes"].as_u64().unwrap_or_default();
            if let Some(path) = node["path"].as_str() {
                placeholder_artifact_outputs.push(path.to_string());
            }
        }

        let Some(metadata) = node["image_metadata"].as_object() else {
            continue;
        };
        image_asset_count += 1;
        if let Some(format) = metadata.get("format").and_then(Value::as_str) {
            *format_counts.entry(format.to_string()).or_insert(0usize) += 1;
        }
        if let Some(dimension_source) = metadata.get("dimension_source").and_then(Value::as_str) {
            *dimension_source_counts
                .entry(dimension_source.to_string())
                .or_insert(0usize) += 1;
        }
        if metadata.get("width").and_then(Value::as_u64).is_some()
            && metadata.get("height").and_then(Value::as_u64).is_some()
        {
            metadata_asset_count += 1;
        }
        optimized_variant_count += metadata
            .get("optimization")
            .and_then(|optimization| optimization.get("variants_emitted"))
            .and_then(Value::as_u64)
            .unwrap_or_default();
        if metadata
            .get("optimization")
            .and_then(|optimization| optimization.get("placeholder"))
            .is_some()
        {
            placeholder_count += 1;
        }
        route_reference_count += node["referenced_by_routes"].as_array().map_or(0, Vec::len);
        style_reference_count += node["referenced_by_styles"].as_array().map_or(0, Vec::len);
    }
    placeholder_artifact_outputs.sort();
    let placeholder_artifact_edge_count = edges
        .iter()
        .filter(|edge| edge["kind"] == "emits-placeholder")
        .count();

    let status = if placeholder_count > 0 {
        "metadata-plus-placeholder-artifacts-boundary"
    } else {
        "metadata-only"
    };

    json!({
        "imageAssetCount": image_asset_count,
        "metadataAssetCount": metadata_asset_count,
        "optimizedVariantCount": optimized_variant_count,
        "placeholderCount": placeholder_count,
        "placeholderArtifactCount": placeholder_artifact_count,
        "placeholderArtifactBytes": placeholder_artifact_bytes,
        "placeholderArtifactOutputs": placeholder_artifact_outputs,
        "placeholderArtifactEdgeCount": placeholder_artifact_edge_count,
        "routeReferenceCount": route_reference_count,
        "styleReferenceCount": style_reference_count,
        "formatCounts": format_counts,
        "dimensionSourceCounts": dimension_source_counts,
        "status": status,
        "fullPipelineParity": false
    })
}

fn server_data_summary(nodes: &[Value], edges: &[Value]) -> Value {
    let mut route_count = 0usize;
    let mut entry_count = 0usize;
    let mut status_counts = BTreeMap::new();
    let mut node_modules_required = false;
    let mut lifecycle_scripts_executed = false;
    let mut source_owned_contract = true;
    let mut external_runtime_required = false;
    let mut external_runtime_executed = false;
    let mut routes_with_route_params = 0usize;
    let mut routes_with_search_params = 0usize;
    let mut route_param_keys = BTreeSet::new();
    let mut search_param_keys = BTreeSet::new();

    for node in nodes {
        if node["kind"] != "server-data-route" {
            continue;
        }
        route_count += 1;
        entry_count += node["entry_count"].as_u64().unwrap_or_default() as usize;
        if let Some(status) = node["status"].as_str() {
            *status_counts.entry(status.to_string()).or_insert(0usize) += 1;
        }
        node_modules_required |= node["node_modules_required"].as_bool().unwrap_or_default();
        lifecycle_scripts_executed |= node["lifecycle_scripts_executed"]
            .as_bool()
            .unwrap_or_default();
        source_owned_contract &= node["source_owned_contract"].as_bool().unwrap_or_default();
        external_runtime_required |= node["external_runtime_required"]
            .as_bool()
            .unwrap_or_default();
        external_runtime_executed |= node["external_runtime_executed"]
            .as_bool()
            .unwrap_or_default();
        if let Some(params) = node["request"]["route_params"].as_object() {
            if !params.is_empty() {
                routes_with_route_params += 1;
            }
            route_param_keys.extend(params.keys().cloned());
        }
        if let Some(params) = node["request"]["search_params"].as_object() {
            if !params.is_empty() {
                routes_with_search_params += 1;
            }
            search_param_keys.extend(params.keys().cloned());
        }
    }

    let emits_edge_count = edges
        .iter()
        .filter(|edge| edge["kind"] == "emits-server-data")
        .count();
    let links_edge_count = edges
        .iter()
        .filter(|edge| edge["kind"] == "links-server-data")
        .count();

    json!({
        "routeCount": route_count,
        "entryCount": entry_count,
        "statusCounts": status_counts,
        "emitsEdgeCount": emits_edge_count,
        "linksEdgeCount": links_edge_count,
        "routesWithRouteParams": routes_with_route_params,
        "routesWithSearchParams": routes_with_search_params,
        "routeParamKeys": route_param_keys.into_iter().collect::<Vec<_>>(),
        "searchParamKeys": search_param_keys.into_iter().collect::<Vec<_>>(),
        "nodeModulesRequired": node_modules_required,
        "lifecycleScriptsExecuted": lifecycle_scripts_executed,
        "sourceOwnedContract": route_count > 0 && source_owned_contract,
        "externalRuntimeRequired": external_runtime_required,
        "externalRuntimeExecuted": external_runtime_executed
    })
}

fn content_pipeline_summary(nodes: &[Value]) -> Value {
    let mut document_count = 0usize;
    let mut mdx_document_count = 0usize;
    let mut node_modules_required = false;

    for node in nodes {
        if node["kind"] != "dx-content-document" {
            continue;
        }
        document_count += 1;
        if node["content_kind"] == "mdx" {
            mdx_document_count += 1;
        }
        if node["node_modules_required"].as_bool().unwrap_or_default() {
            node_modules_required = true;
        }
    }

    json!({
        "documentCount": document_count,
        "mdxDocumentCount": mdx_document_count,
        "nodeModulesRequired": node_modules_required,
        "runtimeProof": false
    })
}

fn ecmascript_analysis_summary(nodes: &[Value]) -> Value {
    let mut source_module_count = 0usize;
    let mut dynamic_import_count = 0usize;
    let mut dynamic_import_option_boundary_count = 0usize;
    let mut dynamic_import_option_unsupported_count = 0usize;
    let mut unresolved_dynamic_import_count = 0usize;
    let mut unsupported_dynamic_import_count = 0usize;
    let mut unsupported_dynamic_import_reason_counts = BTreeMap::new();
    let mut dynamic_import_analysis_status_counts = BTreeMap::new();
    let mut client_boundary_count = 0usize;
    let mut server_boundary_count = 0usize;
    let mut top_level_await_count = 0usize;
    let mut next_runtime_required = false;
    let mut react_runtime_required = false;
    let mut rsc_required = false;
    let mut node_modules_required = false;

    for node in nodes {
        if node["kind"] != "source-module-chunk" {
            continue;
        }
        let analysis = &node["ecmascript_analysis"];
        if !analysis.is_object() {
            continue;
        }

        source_module_count += 1;
        dynamic_import_count += analysis["dynamic_imports"].as_array().map_or(0, Vec::len);
        for dynamic_import in analysis["dynamic_imports"].as_array().into_iter().flatten() {
            if dynamic_import["import_options_present"]
                .as_bool()
                .unwrap_or_default()
            {
                dynamic_import_option_boundary_count += 1;
                if !dynamic_import["import_options_supported"]
                    .as_bool()
                    .unwrap_or(true)
                {
                    dynamic_import_option_unsupported_count += 1;
                }
            }
        }
        unresolved_dynamic_import_count += analysis["unresolved_dynamic_imports"]
            .as_array()
            .map_or(0, Vec::len);
        for unsupported_import in analysis["unsupported_dynamic_imports"]
            .as_array()
            .into_iter()
            .flatten()
        {
            let reason = unsupported_import["reason"]
                .as_str()
                .unwrap_or("unknown")
                .to_string();
            *unsupported_dynamic_import_reason_counts
                .entry(reason)
                .or_insert(0usize) += 1;
        }
        let dynamic_import_analysis = &analysis["dynamic_import_analysis"];
        if dynamic_import_analysis.is_object() {
            unsupported_dynamic_import_count += dynamic_import_analysis["unsupported_count"]
                .as_u64()
                .unwrap_or_default() as usize;
            let status = dynamic_import_analysis["status"]
                .as_str()
                .unwrap_or("unknown")
                .to_string();
            *dynamic_import_analysis_status_counts
                .entry(status)
                .or_insert(0usize) += 1;
        } else {
            *dynamic_import_analysis_status_counts
                .entry("legacy-raw-arrays".to_string())
                .or_insert(0usize) += 1;
        }
        if analysis["top_level_await"].as_bool().unwrap_or_default() {
            top_level_await_count += 1;
        }
        for directive in analysis["directives"].as_array().into_iter().flatten() {
            match directive["value"].as_str().unwrap_or_default() {
                "use client" => client_boundary_count += 1,
                "use server" => server_boundary_count += 1,
                _ => {}
            }
        }

        let runtime = &analysis["runtime_boundaries"];
        next_runtime_required |= runtime["next_runtime_required"]
            .as_bool()
            .unwrap_or_default();
        react_runtime_required |= runtime["react_runtime_required"]
            .as_bool()
            .unwrap_or_default();
        rsc_required |= runtime["rsc_required"].as_bool().unwrap_or_default();
        node_modules_required |= runtime["node_modules_required"]
            .as_bool()
            .unwrap_or_default();
    }

    json!({
        "sourceModuleCount": source_module_count,
        "dynamicImportCount": dynamic_import_count,
        "dynamicImportOptionBoundaryCount": dynamic_import_option_boundary_count,
        "dynamicImportOptionUnsupportedCount": dynamic_import_option_unsupported_count,
        "unresolvedDynamicImportCount": unresolved_dynamic_import_count,
        "unsupportedDynamicImportCount": unsupported_dynamic_import_count,
        "unsupportedDynamicImportReasonCounts": unsupported_dynamic_import_reason_counts,
        "dynamicImportAnalysisStatusCounts": dynamic_import_analysis_status_counts,
        "clientBoundaryCount": client_boundary_count,
        "serverBoundaryCount": server_boundary_count,
        "topLevelAwaitCount": top_level_await_count,
        "nextRuntimeRequired": next_runtime_required,
        "reactRuntimeRequired": react_runtime_required,
        "rscRequired": rsc_required,
        "nodeModulesRequired": node_modules_required,
        "fullNextjsParity": false
    })
}

fn turbopack_core_concept_map() -> Vec<Value> {
    DX_TURBOPACK_CORE_GRAPH_CONCEPTS
        .iter()
        .map(|concept| {
            json!({
                "upstreamConcept": concept.upstream_concept,
                "vendorPaths": concept.vendor_paths,
                "dxContracts": concept.dx_contracts,
                "dxNodeKinds": concept.dx_node_kinds,
                "dxEdgeKinds": concept.dx_edge_kinds,
                "dxReceiptFields": concept.dx_receipt_fields,
                "boundary": concept.boundary,
                "nodeModulesRequired": concept.node_modules_required
            })
        })
        .collect()
}

fn summarize_core_concept_map(concepts: Option<&Vec<Value>>) -> Value {
    let mut covered_node_kinds = BTreeSet::new();
    let mut covered_edge_kinds = BTreeSet::new();
    let mut node_modules_required = false;
    let concepts = concepts.cloned().unwrap_or_default();

    for concept in &concepts {
        if concept["nodeModulesRequired"].as_bool().unwrap_or_default() {
            node_modules_required = true;
        }
        for kind in concept["dxNodeKinds"].as_array().into_iter().flatten() {
            if let Some(kind) = kind.as_str() {
                covered_node_kinds.insert(kind.to_string());
            }
        }
        for kind in concept["dxEdgeKinds"].as_array().into_iter().flatten() {
            if let Some(kind) = kind.as_str() {
                covered_edge_kinds.insert(kind.to_string());
            }
        }
    }

    json!({
        "conceptCount": concepts.len(),
        "coveredNodeKinds": covered_node_kinds.into_iter().collect::<Vec<_>>(),
        "coveredEdgeKinds": covered_edge_kinds.into_iter().collect::<Vec<_>>(),
        "nodeModulesRequired": node_modules_required,
        "adapterBoundary": true,
        "publicArchitecture": false
    })
}

fn style_optimization_summary(nodes: &[Value]) -> Value {
    let mut style_node_count = 0usize;
    let mut original_rule_count = 0u64;
    let mut retained_rule_count = 0u64;
    let mut pruned_rule_count = 0u64;
    let mut minified_style_count = 0usize;
    let mut source_map_count = 0usize;
    let mut source_map_source_count = 0u64;
    let mut source_map_source_hash_count = 0u64;
    let mut source_map_entry_style_source_count = 0u64;
    let mut source_map_flattened_import_source_count = 0u64;
    let mut source_map_retained_import_source_count = 0u64;
    let mut source_map_segment_count = 0u64;
    let mut source_map_exact_segment_map_count = 0usize;
    let mut source_map_evidence_only_count = 0usize;
    let mut source_map_link_count = 0usize;
    let mut source_map_hash_count = 0usize;
    let mut source_map_artifact_count = 0usize;
    let mut flattened_import_count = 0u64;
    let mut retained_import_count = 0u64;
    let mut asset_reference_count = 0u64;
    let mut entry_style_asset_reference_count = 0usize;
    let mut flattened_import_asset_reference_count = 0usize;
    let mut retained_import_asset_reference_count = 0usize;

    for node in nodes {
        if node["kind"] == "dx-style-source-map" {
            source_map_artifact_count += 1;
            continue;
        }
        if node["kind"] != "dx-style-css" {
            continue;
        }
        style_node_count += 1;
        original_rule_count += node["original_rule_count"].as_u64().unwrap_or_default();
        retained_rule_count += node["retained_rule_count"].as_u64().unwrap_or_default();
        pruned_rule_count += node["pruned_rule_count"].as_u64().unwrap_or_default();
        if node["minified"].as_bool().unwrap_or_default() {
            minified_style_count += 1;
        }
        if node["source_map_output"].as_str().is_some() {
            source_map_count += 1;
        }
        source_map_source_count += node["source_map_source_count"].as_u64().unwrap_or_default();
        source_map_source_hash_count += node["source_map_source_hash_count"]
            .as_u64()
            .unwrap_or_default();
        source_map_entry_style_source_count += node["source_map_entry_style_source_count"]
            .as_u64()
            .unwrap_or_default();
        source_map_flattened_import_source_count +=
            node["source_map_flattened_import_source_count"]
                .as_u64()
                .unwrap_or_default();
        source_map_retained_import_source_count += node["source_map_retained_import_source_count"]
            .as_u64()
            .unwrap_or_default();
        source_map_segment_count += node["source_map_segment_count"]
            .as_u64()
            .unwrap_or_default();
        if node["source_map_exact_segment_mapping"]
            .as_bool()
            .unwrap_or_default()
        {
            source_map_exact_segment_map_count += 1;
        }
        if node["source_map_evidence_only"]
            .as_bool()
            .unwrap_or_default()
        {
            source_map_evidence_only_count += 1;
        }
        if node["source_map_linked"].as_bool().unwrap_or_default() {
            source_map_link_count += 1;
        }
        if node["source_map_hash"].as_str().is_some() {
            source_map_hash_count += 1;
        }
        flattened_import_count += node["flattened_import_count"].as_u64().unwrap_or_default();
        retained_import_count += node["retained_import_count"].as_u64().unwrap_or_default();
        asset_reference_count += node["asset_reference_count"].as_u64().unwrap_or_default();
        for reference in node["asset_references"].as_array().into_iter().flatten() {
            match reference["source_role"].as_str().unwrap_or_default() {
                "entry-style" => entry_style_asset_reference_count += 1,
                "flattened-import" => flattened_import_asset_reference_count += 1,
                "retained-import" => retained_import_asset_reference_count += 1,
                _ => {}
            }
        }
    }

    json!({
        "styleNodeCount": style_node_count,
        "originalRuleCount": original_rule_count,
        "retainedRuleCount": retained_rule_count,
        "prunedRuleCount": pruned_rule_count,
        "minifiedStyleCount": minified_style_count,
        "sourceMapCount": source_map_count,
        "sourceMapSourceCount": source_map_source_count,
        "sourceMapSourceHashCount": source_map_source_hash_count,
        "sourceMapEntryStyleSourceCount": source_map_entry_style_source_count,
        "sourceMapFlattenedImportSourceCount": source_map_flattened_import_source_count,
        "sourceMapRetainedImportSourceCount": source_map_retained_import_source_count,
        "sourceMapSegmentCount": source_map_segment_count,
        "sourceMapExactSegmentMapCount": source_map_exact_segment_map_count,
        "sourceMapEvidenceOnlyCount": source_map_evidence_only_count,
        "sourceMapLinkCount": source_map_link_count,
        "sourceMapHashCount": source_map_hash_count,
        "sourceMapArtifactCount": source_map_artifact_count,
        "flattenedImportCount": flattened_import_count,
        "retainedImportCount": retained_import_count,
        "assetReferenceCount": asset_reference_count,
        "entryStyleAssetReferenceCount": entry_style_asset_reference_count,
        "flattenedImportAssetReferenceCount": flattened_import_asset_reference_count,
        "retainedImportAssetReferenceCount": retained_import_asset_reference_count
    })
}

fn graph_nodes(
    routes: &[SourceBuildRoute],
    route_handlers: &[SourceBuildRouteHandler],
    route_outputs: &[SourceBuildRouteOutput],
    server_data_routes: &[SourceBuildServerDataRoute],
    styles: &[SourceBuildStyle],
    content_documents: &[SourceBuildContentDocument],
    assets: &[SourceBuildAsset],
) -> Vec<Value> {
    let mut nodes = Vec::new();
    let mut source_module_nodes = BTreeMap::new();
    let mut css_import_source_nodes = BTreeMap::new();
    for route in routes {
        nodes.push(json!({
            "id": node_id("tsx-route", &route.path),
            "kind": "tsx-route",
            "path": route.path,
            "route": route.route,
            "contract": "dx.www.moduleGraph",
            "hash": route.hash,
            "parser_backend": route.parser_backend,
            "ecmascript_analysis": &route.ecmascript_analysis
        }));
    }
    for handler in route_handlers {
        nodes.push(json!({
            "id": node_id("app-route-handler", &handler.path),
            "kind": "app-route-handler",
            "path": handler.path,
            "route": handler.route,
            "methods": handler.methods,
            "contract": "dx.www.moduleGraph",
            "hash": handler.hash,
            "parser_backend": handler.parser_backend,
            "execution_model": handler.execution_model,
            "lifecycle_scripts_executed": handler.lifecycle_scripts_executed,
            "ecmascript_analysis": &handler.ecmascript_analysis,
            "node_modules_required": handler.node_modules_required
        }));
    }
    for output in route_outputs {
        nodes.push(json!({
            "id": node_id("route-shell-chunk", &output.shell_chunk_output),
            "kind": "route-shell-chunk",
            "path": output.shell_chunk_output,
            "route": output.route,
            "contract": "dx.www.moduleGraph",
            "fallback_hash": output.fallback_hash,
            "node_modules_required": output.node_modules_required
        }));
        for chunk in &output.source_module_chunks {
            if is_support_source_module(&chunk.source_path) {
                record_source_module_node(&mut source_module_nodes, chunk, &output.route);
            }
            nodes.push(json!({
                "id": node_id("source-module-chunk", &chunk.chunk_output),
                "kind": "source-module-chunk",
                "path": chunk.chunk_output,
                "source_path": chunk.source_path,
                "route": output.route,
                "source_kind": chunk.kind,
                "contract": "dx.www.moduleGraph",
                "hash": chunk.hash,
                "browser_executable": chunk.browser_executable,
                "source_transformed": chunk.source_transformed,
                "transform_kind": chunk.transform_kind,
                "runtime_exports": chunk.runtime_exports,
                "ecmascript_analysis": &chunk.ecmascript_analysis,
                "node_modules_required": chunk.node_modules_required
            }));
            for dependency in &chunk.dependencies {
                if !is_adapter_boundary_dependency(dependency.resolver_source.as_str()) {
                    continue;
                }
                nodes.push(json!({
                    "id": node_id("adapter-boundary-import", &dependency.specifier),
                    "kind": "adapter-boundary-import",
                    "specifier": dependency.specifier.as_str(),
                    "dependency_kind": dependency.kind.as_str(),
                    "resolver_source": dependency.resolver_source.as_str(),
                    "resolver_detail": dependency.resolver_detail.as_str(),
                    "contract": "dx.www.moduleGraph",
                    "node_modules_required": dependency.node_modules_required,
                    "public_architecture": false
                }));
            }
        }
    }
    for server_data in server_data_routes {
        nodes.push(json!({
            "id": node_id("server-data-route", &server_data.output),
            "kind": "server-data-route",
            "route": server_data.route,
            "path": server_data.output,
            "output": server_data.output,
            "route_source_path": server_data.source_path,
            "contract": "dx.appRouter.serverData",
            "status": server_data.status,
            "entry_count": server_data.entry_count,
            "entry_source_paths": server_data.entry_source_paths,
            "request": server_data.request,
            "execution_model": server_data.execution_model,
            "node_modules_required": server_data.node_modules_required,
            "lifecycle_scripts_executed": server_data.lifecycle_scripts_executed,
            "source_owned_contract": server_data.source_owned_contract,
            "external_runtime_required": server_data.external_runtime_required,
            "external_runtime_executed": server_data.external_runtime_executed
        }));
    }
    nodes.extend(source_module_nodes.into_values());
    for style in styles {
        for import in &style.flattened_imports {
            record_css_import_source_node(&mut css_import_source_nodes, style, import);
        }
        nodes.push(json!({
            "id": node_id("dx-style-css", &style.path),
            "kind": "dx-style-css",
            "path": style.path,
            "output": style.output,
            "contract": "dx.build.graph",
            "hash": style.hash,
            "original_rule_count": style.original_rule_count,
            "retained_rule_count": style.retained_rule_count,
            "pruned_rule_count": style.pruned_rule_count,
            "minified": style.minified,
            "source_map_output": style.source_map_output.clone(),
            "source_map_source_count": style.source_map_source_count,
            "source_map_source_hash_count": style.source_map_source_hash_count,
            "source_map_entry_style_source_count": style.source_map_entry_style_source_count,
            "source_map_flattened_import_source_count": style.source_map_flattened_import_source_count,
            "source_map_retained_import_source_count": style.source_map_retained_import_source_count,
            "source_map_segment_count": style.source_map_segment_count,
            "source_map_exact_segment_mapping": style.source_map_exact_segment_mapping,
            "source_map_evidence_only": style.source_map_evidence_only,
            "source_map_linked": style.source_map_linked,
            "source_map_hash": style.source_map_hash.clone(),
            "node_modules_required": style.node_modules_required,
            "lifecycle_scripts_executed": style.lifecycle_scripts_executed,
            "source_owned_contract": style.source_owned_contract,
            "external_runtime_required": style.external_runtime_required,
            "external_runtime_executed": style.external_runtime_executed,
            "flattened_import_count": style.flattened_imports.len(),
            "flattened_imports": style.flattened_imports.clone(),
            "retained_import_count": style.retained_imports.len(),
            "retained_imports": style.retained_imports.clone(),
            "asset_reference_count": style.asset_references.len(),
            "asset_references": style.asset_references.clone()
        }));
        if let Some(source_map_output) = style.source_map_output.as_deref() {
            nodes.push(json!({
                "id": node_id("dx-style-source-map", source_map_output),
                "kind": "dx-style-source-map",
                "path": source_map_output,
                "output": source_map_output,
                "style_path": style.path.as_str(),
                "contract": "dx.build.graph",
                "source_map_source_count": style.source_map_source_count,
                "source_map_source_hash_count": style.source_map_source_hash_count,
                "source_map_entry_style_source_count": style.source_map_entry_style_source_count,
                "source_map_flattened_import_source_count": style.source_map_flattened_import_source_count,
                "source_map_retained_import_source_count": style.source_map_retained_import_source_count,
                "source_map_segment_count": style.source_map_segment_count,
                "source_map_exact_segment_mapping": style.source_map_exact_segment_mapping,
                "source_map_evidence_only": style.source_map_evidence_only,
                "source_map_linked": style.source_map_linked,
                "source_map_hash": style.source_map_hash.clone(),
                "node_modules_required": style.node_modules_required,
                "lifecycle_scripts_executed": style.lifecycle_scripts_executed,
                "source_owned_contract": style.source_owned_contract,
                "external_runtime_required": style.external_runtime_required,
                "external_runtime_executed": style.external_runtime_executed
            }));
        }
    }
    nodes.extend(css_import_source_nodes.into_values());
    for document in content_documents {
        nodes.push(json!({
            "id": node_id("dx-content-document", &document.path),
            "kind": "dx-content-document",
            "path": document.path,
            "content_kind": document.kind,
            "contract": "dx.forge.sourceGraph",
            "hash": document.hash,
            "bytes": document.size,
            "frontmatter": document.frontmatter,
            "heading_count": document.heading_count,
            "code_block_count": document.code_block_count,
            "mdx_options": document.mdx_options,
            "node_modules_required": document.node_modules_required,
            "runtime_proof": document.runtime_proof,
            "adapter_boundary": document.adapter_boundary
        }));
    }
    for asset in assets {
        let mut node = json!({
            "id": node_id("public-asset", &asset.path),
            "kind": "public-asset",
            "path": asset.path,
            "output": asset.output,
            "contract": "dx.build.graph",
            "hash": asset.hash,
            "bytes": asset.size,
            "node_modules_required": asset.node_modules_required,
            "lifecycle_scripts_executed": asset.lifecycle_scripts_executed,
            "source_owned_contract": asset.source_owned_contract,
            "external_runtime_required": asset.external_runtime_required,
            "external_runtime_executed": asset.external_runtime_executed
        });
        if let Some(metadata) = &asset.image_metadata {
            node["image_metadata"] = json!(metadata);
            node["referenced_by_routes"] = json!(asset.referenced_by_routes);
            node["referenced_by_styles"] = json!(asset.referenced_by_styles);
        }
        nodes.push(node);
        if let Some(placeholder_node) = image_placeholder_node(asset) {
            nodes.push(placeholder_node);
        }
    }
    nodes.sort_by_key(|node| node["id"].as_str().unwrap_or_default().to_string());
    nodes
}

fn graph_edges(
    project_root: &Path,
    routes: &[SourceBuildRoute],
    route_handlers: &[SourceBuildRouteHandler],
    route_outputs: &[SourceBuildRouteOutput],
    server_data_routes: &[SourceBuildServerDataRoute],
    styles: &[SourceBuildStyle],
    assets: &[SourceBuildAsset],
) -> Vec<Value> {
    let style_paths = styles
        .iter()
        .map(|style| style.path.as_str())
        .collect::<BTreeSet<_>>();
    let asset_paths = assets
        .iter()
        .map(|asset| asset.path.as_str())
        .collect::<BTreeSet<_>>();
    let mut edges = Vec::new();

    for route in routes {
        let route_id = node_id("tsx-route", &route.path);
        if let Some(output) = route_outputs
            .iter()
            .find(|output| output.route == route.route)
        {
            edges.push(json!({
                "from": route_id,
                "to": node_id("route-shell-chunk", &output.shell_chunk_output),
                "kind": "emits"
            }));
            if let Some(entry_chunk) = output.entry_module_chunk_output.as_deref() {
                edges.push(json!({
                    "from": route_id,
                    "to": node_id("source-module-chunk", entry_chunk),
                    "kind": "imports-source-module"
                }));
                edges.push(json!({
                    "from": node_id("route-shell-chunk", &output.shell_chunk_output),
                    "to": node_id("source-module-chunk", entry_chunk),
                    "kind": "links-entry-module"
                }));
            }
            if let Some(server_data) = server_data_routes.iter().find(|server_data| {
                server_data.route == output.route && server_data.source_path == output.source_path
            }) {
                let server_data_node = node_id("server-data-route", &server_data.output);
                edges.push(json!({
                    "from": route_id,
                    "to": server_data_node.as_str(),
                    "kind": "emits-server-data",
                    "route": server_data.route,
                    "route_source_path": server_data.source_path,
                    "output": server_data.output,
                    "node_modules_required": server_data.node_modules_required,
                    "lifecycle_scripts_executed": server_data.lifecycle_scripts_executed,
                    "source_owned_contract": server_data.source_owned_contract,
                    "external_runtime_required": server_data.external_runtime_required,
                    "external_runtime_executed": server_data.external_runtime_executed
                }));
                edges.push(json!({
                    "from": node_id("route-shell-chunk", &output.shell_chunk_output),
                    "to": server_data_node.as_str(),
                    "kind": "links-server-data",
                    "route": server_data.route,
                    "route_source_path": server_data.source_path,
                    "output": server_data.output,
                    "node_modules_required": server_data.node_modules_required,
                    "lifecycle_scripts_executed": server_data.lifecycle_scripts_executed,
                    "source_owned_contract": server_data.source_owned_contract,
                    "external_runtime_required": server_data.external_runtime_required,
                    "external_runtime_executed": server_data.external_runtime_executed
                }));
                for source_path in &server_data.entry_source_paths {
                    edges.push(json!({
                        "from": server_data_node.as_str(),
                        "to": node_id("source-module", source_path),
                        "kind": "uses-server-loader",
                        "route": server_data.route,
                        "route_source_path": server_data.source_path,
                        "source_path": source_path,
                        "output": server_data.output,
                        "node_modules_required": server_data.node_modules_required,
                        "lifecycle_scripts_executed": server_data.lifecycle_scripts_executed,
                        "source_owned_contract": server_data.source_owned_contract,
                        "external_runtime_required": server_data.external_runtime_required,
                        "external_runtime_executed": server_data.external_runtime_executed
                    }));
                }
            }
            for chunk in &output.source_module_chunks {
                if is_support_source_module(&chunk.source_path) {
                    edges.push(json!({
                        "from": node_id("source-module-chunk", &chunk.chunk_output),
                        "to": node_id("source-module", &chunk.source_path),
                        "kind": "compiled-from-source",
                        "source_path": chunk.source_path.as_str(),
                        "chunk_output": chunk.chunk_output.as_str(),
                        "node_modules_required": chunk.node_modules_required
                    }));
                }
                for dependency in &chunk.dependencies {
                    if let Some(dependency_chunk) = dependency.chunk_output.as_deref() {
                        edges.push(json!({
                            "from": node_id("source-module-chunk", &chunk.chunk_output),
                            "to": node_id("source-module-chunk", dependency_chunk),
                            "kind": "imports-source-module",
                            "specifier": dependency.specifier.as_str(),
                            "resolver_source": dependency.resolver_source.as_str()
                        }));
                    } else if is_adapter_boundary_dependency(dependency.resolver_source.as_str()) {
                        edges.push(json!({
                            "from": node_id("source-module-chunk", &chunk.chunk_output),
                            "to": node_id("adapter-boundary-import", &dependency.specifier),
                            "kind": "adapter-boundary",
                            "specifier": dependency.specifier.as_str(),
                            "dependency_kind": dependency.kind.as_str(),
                            "resolver_source": dependency.resolver_source.as_str(),
                            "resolver_detail": dependency.resolver_detail.as_str(),
                            "node_modules_required": dependency.node_modules_required,
                            "public_architecture": false
                        }));
                    }
                }
            }
        }
        for import in &route.imports {
            let Some(resolved) =
                resolve_relative_import(project_root, &route.path, &import.specifier)
            else {
                continue;
            };
            if style_paths.contains(resolved.as_str()) {
                edges.push(json!({
                    "from": route_id,
                    "to": node_id("dx-style-css", &resolved),
                    "kind": "imports",
                    "specifier": import.specifier
                }));
            }
        }
    }

    for handler in route_handlers {
        let handler_id = node_id("app-route-handler", &handler.path);
        for import in &handler.imports {
            let Some(resolved) =
                resolve_relative_import(project_root, &handler.path, &import.specifier)
            else {
                continue;
            };
            if style_paths.contains(resolved.as_str()) {
                edges.push(json!({
                    "from": handler_id,
                    "to": node_id("dx-style-css", &resolved),
                    "kind": "imports",
                    "specifier": import.specifier
                }));
            } else if asset_paths.contains(resolved.as_str()) {
                edges.push(json!({
                    "from": handler_id,
                    "to": node_id("public-asset", &resolved),
                    "kind": "references",
                    "specifier": import.specifier
                }));
            }
        }
    }

    for style in styles {
        if let Some(source_map_output) = style.source_map_output.as_deref() {
            edges.push(json!({
                "from": node_id("dx-style-css", &style.path),
                "to": node_id("dx-style-source-map", source_map_output),
                "kind": "emits-css-source-map",
                "source_path": style.path.as_str(),
                "output": source_map_output,
                "source_map_hash": style.source_map_hash.clone(),
                "source_map_linked": style.source_map_linked,
                "node_modules_required": style.node_modules_required,
                "lifecycle_scripts_executed": style.lifecycle_scripts_executed,
                "source_owned_contract": style.source_owned_contract,
                "external_runtime_required": style.external_runtime_required,
                "external_runtime_executed": style.external_runtime_executed
            }));
        }
        for import in &style.flattened_imports {
            edges.push(json!({
                "from": node_id("dx-style-css", &style.path),
                "to": node_id("dx-style-import-source", &import.path),
                "kind": "flattens-css-import",
                "specifier": import.specifier.as_str(),
                "source_path": import.path.as_str(),
                "source_role": "flattened-import",
                "inlined": import.inlined,
                "node_modules_required": style.node_modules_required,
                "lifecycle_scripts_executed": style.lifecycle_scripts_executed,
                "source_owned_contract": style.source_owned_contract,
                "external_runtime_required": style.external_runtime_required,
                "external_runtime_executed": style.external_runtime_executed
            }));
        }
        for reference in &style.asset_references {
            if !asset_paths.contains(reference.path.as_str()) {
                continue;
            }
            edges.push(json!({
                "from": node_id("dx-style-css", &style.path),
                "to": node_id("public-asset", &reference.path),
                "kind": "imports",
                "specifier": reference.specifier.as_str(),
                "reference_source": reference.kind.as_str(),
                "source_path": reference.source_path.as_str(),
                "source_role": reference.source_role.as_str(),
                "import_specifier": reference.import_specifier.as_deref(),
                "node_modules_required": reference.node_modules_required
            }));
        }
    }

    for asset in assets {
        if let Some(placeholder_edge) = image_placeholder_edge(asset) {
            edges.push(placeholder_edge);
        }
    }

    edges.sort_by_key(|edge| {
        format!(
            "{}\0{}\0{}",
            edge["from"].as_str().unwrap_or_default(),
            edge["to"].as_str().unwrap_or_default(),
            edge["kind"].as_str().unwrap_or_default()
        )
    });
    edges
}

fn image_placeholder_node(asset: &SourceBuildAsset) -> Option<Value> {
    let metadata = asset.image_metadata.as_ref()?;
    let placeholder = metadata.optimization.placeholder.as_ref()?;
    let output = placeholder.output.as_deref()?;
    let hash = placeholder.hash.as_deref()?;
    let artifact_bytes = placeholder.artifact_bytes?;

    Some(json!({
        "id": node_id("image-placeholder-asset", output),
        "kind": "image-placeholder-asset",
        "path": output,
        "contract": "dx.build.graph",
        "source_image": asset.path,
        "source_asset": node_id("public-asset", &asset.path),
        "placeholder_kind": placeholder.kind.as_str(),
        "source": placeholder.source.as_str(),
        "width": placeholder.width,
        "height": placeholder.height,
        "data_url_bytes": placeholder.bytes,
        "hash": hash,
        "bytes": artifact_bytes,
        "node_modules_required": asset.node_modules_required,
        "lifecycle_scripts_executed": asset.lifecycle_scripts_executed,
        "source_owned_contract": asset.source_owned_contract,
        "external_runtime_required": asset.external_runtime_required,
        "external_runtime_executed": asset.external_runtime_executed,
        "optimizer_invoked": metadata.optimization.optimizer_invoked,
        "resize_emitted": metadata.optimization.resize_emitted,
        "encoding_emitted": metadata.optimization.encoding_emitted,
        "blur_placeholder_emitted": metadata.optimization.blur_placeholder_emitted,
        "full_pipeline_parity": false
    }))
}

fn image_placeholder_edge(asset: &SourceBuildAsset) -> Option<Value> {
    let metadata = asset.image_metadata.as_ref()?;
    let placeholder = metadata.optimization.placeholder.as_ref()?;
    let output = placeholder.output.as_deref()?;

    Some(json!({
        "from": node_id("public-asset", &asset.path),
        "to": node_id("image-placeholder-asset", output),
        "kind": "emits-placeholder",
        "source_image": asset.path,
        "output": output,
        "placeholder_kind": placeholder.kind.as_str(),
        "node_modules_required": asset.node_modules_required,
        "lifecycle_scripts_executed": asset.lifecycle_scripts_executed,
        "source_owned_contract": asset.source_owned_contract,
        "external_runtime_required": asset.external_runtime_required,
        "external_runtime_executed": asset.external_runtime_executed,
        "optimizer_invoked": metadata.optimization.optimizer_invoked,
        "resize_emitted": metadata.optimization.resize_emitted,
        "encoding_emitted": metadata.optimization.encoding_emitted,
        "blur_placeholder_emitted": metadata.optimization.blur_placeholder_emitted,
        "full_pipeline_parity": false
    }))
}

fn resolve_relative_import(project_root: &Path, importer: &str, specifier: &str) -> Option<String> {
    if !specifier.starts_with('.') {
        return None;
    }
    let project_root = project_root
        .canonicalize()
        .unwrap_or_else(|_| project_root.to_path_buf());
    let importer_dir = Path::new(importer)
        .parent()
        .unwrap_or_else(|| Path::new(""));
    let base = project_root.join(importer_dir).join(specifier);
    let candidates = [
        base.clone(),
        base.with_extension("css"),
        base.with_extension("tsx"),
        base.with_extension("ts"),
        base.with_extension("jsx"),
        base.with_extension("js"),
    ];
    candidates
        .into_iter()
        .find(|candidate| candidate.is_file())
        .and_then(|candidate| candidate.canonicalize().ok())
        .map(|candidate| relative_project_path(&project_root, &candidate))
}

fn is_adapter_boundary_dependency(resolver_source: &str) -> bool {
    resolver_source == RESOLVER_SOURCE_ADAPTER_BOUNDARY || resolver_source.ends_with("-boundary")
}

fn is_support_source_module(source_path: &str) -> bool {
    source_path.starts_with("lib/")
        || source_path.starts_with("server/")
        || source_path.starts_with("src/")
}

fn record_source_module_node(
    source_module_nodes: &mut BTreeMap<String, Value>,
    chunk: &SourceBuildModuleChunk,
    route: &str,
) {
    let id = node_id("source-module", &chunk.source_path);
    if let Some(node) = source_module_nodes.get_mut(&id) {
        append_source_module_route(node, route);
        return;
    }

    source_module_nodes.insert(
        id.clone(),
        json!({
            "id": id,
            "kind": "source-module",
            "path": chunk.source_path,
            "route": route,
            "routes": [route],
            "source_kind": chunk.kind,
            "contract": "dx.www.moduleGraph",
            "hash": chunk.hash,
            "chunk_output": chunk.chunk_output,
            "browser_executable": chunk.browser_executable,
            "source_transformed": chunk.source_transformed,
            "transform_kind": chunk.transform_kind,
            "runtime_exports": chunk.runtime_exports,
            "ecmascript_analysis": &chunk.ecmascript_analysis,
            "node_modules_required": chunk.node_modules_required
        }),
    );
}

fn append_source_module_route(node: &mut Value, route: &str) {
    let Some(routes) = node["routes"].as_array_mut() else {
        return;
    };
    if routes.iter().any(|existing| existing == route) {
        return;
    }
    routes.push(json!(route));
    routes.sort_by_key(|route| route.as_str().unwrap_or_default().to_string());
}

fn record_css_import_source_node(
    css_import_source_nodes: &mut BTreeMap<String, Value>,
    style: &SourceBuildStyle,
    import: &super::graph::SourceBuildStyleImport,
) {
    let id = node_id("dx-style-import-source", &import.path);
    if let Some(node) = css_import_source_nodes.get_mut(&id) {
        append_json_string_array(node, "style_paths", &style.path);
        append_json_string_array(node, "specifiers", &import.specifier);
        return;
    }

    css_import_source_nodes.insert(
        id.clone(),
        json!({
            "id": id,
            "kind": "dx-style-import-source",
            "path": import.path,
            "style_path": style.path,
            "style_paths": [style.path],
            "specifier": import.specifier,
            "specifiers": [import.specifier],
            "source_role": "flattened-import",
            "contract": "dx.build.graph",
            "inlined": import.inlined,
            "node_modules_required": false,
            "source_owned_contract": true,
            "external_runtime_required": false,
            "external_runtime_executed": false
        }),
    );
}

fn append_json_string_array(node: &mut Value, field: &str, value: &str) {
    let Some(values) = node[field].as_array_mut() else {
        return;
    };
    if values.iter().any(|existing| existing == value) {
        return;
    }
    values.push(json!(value));
    values.sort_by_key(|value| value.as_str().unwrap_or_default().to_string());
}

fn manifest_provenance() -> Vec<Value> {
    vec![
        json!({
            "name": "Rolldown source study",
            "upstream": "rolldown/rolldown",
            "license": "MIT",
            "copiedCode": false
        }),
        json!({
            "name": "Oxc parser boundary",
            "upstream": "oxc-project/oxc",
            "license": "MIT",
            "copiedCode": false
        }),
    ]
}

fn node_id(kind: &str, path: &str) -> String {
    format!("{kind}:{path}")
}
