use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::binary_compiler::{CompilerError, CompilerResult};

use super::client_boundary::{
    DxReactClientSource, analyze_react_client_boundaries, select_react_delivery_mode,
};
use super::client_island::{
    DxReactClientIsland, DxReactClientIslandInput, DxReactClientIslandManifest,
    compile_react_client_islands, react_client_island_abi_capabilities,
};
use super::contract::{
    DxComponentEdge, DxComponentGraph, DxComponentNode, DxFallbackHtml, DxPacket, DxPacketCodec,
    DxPacketKind, DxPacketSection, DxPacketSectionEncoding, DxPacketSectionKind, DxPageGraph,
    DxRouteUnit, DxStyleClass, DxStyleDelivery, DxStyleGraph, DxStyleToken,
};
use super::jsx_lowering::{
    DxReactJsxChildNode, DxReactJsxDocument, DxReactJsxElement, lower_react_jsx_source,
};
use super::route_unit::compile_route_unit;
use super::tsx_ast::parse_tsx_module;
use super::types::DxDeliveryMode;

/// React-shaped component source used by the App Router compiler slice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactComponentSource {
    /// Component export name.
    pub name: String,
    /// Project-relative source path.
    pub source_path: String,
    /// Raw TSX/JSX source.
    pub source: String,
    /// Optional source-owned package id when Forge owns the file.
    pub package_id: Option<String>,
}

/// CSS-facing source used by the App Router compiler slice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactStyleSource {
    /// Project-relative source path.
    pub source_path: String,
    /// Raw CSS source.
    pub source: String,
}

/// Next-familiar `app/` segment source used around a route.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactAppSegmentSource {
    /// Segment file kind.
    pub kind: DxReactAppSegmentKind,
    /// Project-relative segment source path.
    pub source_path: String,
    /// Raw segment TSX/JSX source.
    pub source: String,
}

/// Supported Next-familiar `app/` segment files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxReactAppSegmentKind {
    /// `layout.tsx` wrapper segment.
    Layout,
    /// `template.tsx` remounting wrapper segment.
    Template,
    /// `loading.tsx` suspense fallback segment.
    Loading,
    /// `error.tsx` client error boundary segment.
    Error,
    /// `not-found.tsx` route alternative segment.
    NotFound,
}

impl DxReactAppSegmentKind {
    fn component_name(self) -> &'static str {
        match self {
            Self::Layout => "Layout",
            Self::Template => "Template",
            Self::Loading => "Loading",
            Self::Error => "Error",
            Self::NotFound => "NotFound",
        }
    }

    fn boundary_name(self) -> Option<&'static str> {
        match self {
            Self::Layout => None,
            Self::Template => None,
            Self::Loading => Some("loading"),
            Self::Error => Some("error"),
            Self::NotFound => Some("not-found"),
        }
    }
}

/// Input for compiling one React-shaped `app/` route.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactAppRouteInput {
    /// Route path, such as `/` or `/dashboard`.
    pub route: String,
    /// Project-relative route source path.
    pub route_source_path: String,
    /// Raw route TSX/JSX source.
    pub route_source: String,
    /// Layout and boundary segments that apply to the route.
    pub segments: Vec<DxReactAppSegmentSource>,
    /// Local or Forge-owned component sources available to the route.
    pub components: Vec<DxReactComponentSource>,
    /// Style/token sources available to the route.
    pub styles: Vec<DxReactStyleSource>,
    /// Optional Forge manifest hash associated with this route build.
    pub source_manifest_hash: Option<String>,
}

/// Compiled React-shaped route proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactAppRouteProof {
    /// Route path that was compiled.
    pub route: String,
    /// Canonical compiler page graph.
    pub page_graph: DxPageGraph,
    /// Crawlable fallback HTML.
    pub fallback: DxFallbackHtml,
    /// Selected first delivery mode.
    pub delivery_mode: DxDeliveryMode,
    /// Generated CSS-facing assets required by this route.
    pub generated_styles: Vec<DxReactGeneratedStyleAsset>,
    /// Streaming/deferred rendering plan for this route.
    pub streaming: DxReactStreamingProof,
    /// Canonical browser packet proof.
    pub packet: DxReactAppRoutePacketProof,
    /// First-class route unit contract joining shell, graph, state, packet, and receipt.
    pub route_unit: DxRouteUnit,
}

/// Route-level proof that www can flush a shell before deferred work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactStreamingProof {
    /// Whether the route has shell-first streaming work.
    pub enabled: bool,
    /// Delivery strategy selected by the compiler.
    pub strategy: String,
    /// Byte length of the first HTML shell flush.
    pub shell_bytes: usize,
    /// First HTML shell flush.
    pub first_flush_html: String,
    /// Deferred HTML chunks, usually from loading/suspense boundaries.
    pub deferred_chunks: Vec<DxReactDeferredChunk>,
    /// Client islands that can resume after the shell flush.
    pub resumable_islands: Vec<DxReactResumableIsland>,
    /// Whether this plan needs `node_modules` at runtime.
    pub node_modules_required: bool,
}

/// One deferred boundary chunk.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactDeferredChunk {
    /// Stable chunk id.
    pub id: String,
    /// Boundary kind, such as `loading`.
    pub boundary: String,
    /// Source segment path.
    pub source_path: String,
    /// HTML emitted after the shell flush.
    pub html: String,
    /// Chunk byte length.
    pub bytes: usize,
    /// Flush order after the initial shell.
    pub flush_order: usize,
}

/// One client island that can resume after the shell flush.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactResumableIsland {
    /// Stable island id.
    pub id: String,
    /// Source path for the client boundary.
    pub source_path: String,
    /// Browser selector used by the runtime to resume.
    pub resume_selector: String,
    /// Runtime selected for this island.
    pub runtime: String,
    /// Public authoring style used for island hydration directives.
    pub directive_style_id: String,
    /// CamelCase hydration directives observed for this island.
    pub directives: Vec<String>,
    /// Source-owned hydration strategy selected for this island.
    pub hydration_strategy: String,
    /// Whether the island runtime is compiler/source owned.
    pub source_owned_runtime: bool,
    /// Whether the route must preserve its HTML fallback without JavaScript.
    pub no_js_fallback_required: bool,
    /// Whether this route proof claims full React hydration.
    pub full_react_hydration: bool,
    /// Browser proof status for this source-only island proof.
    pub browser_proof_status: String,
    /// Framework adapter execution status for explicit clientOnly islands.
    pub framework_adapter: String,
    /// Scope of the proof represented by this route metadata.
    pub proof_scope: String,
    /// Number of state slots that need resume metadata.
    pub state_slots: usize,
    /// Number of event slots that need listener rebinding.
    pub event_slots: usize,
}

/// Generated CSS asset emitted from source-owned style inputs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactGeneratedStyleAsset {
    /// Build-output relative path.
    pub output_path: String,
    /// Browser-facing href.
    pub href: String,
    /// Source CSS files that contributed to this asset.
    pub source_paths: Vec<String>,
    /// BLAKE3 hash of the generated CSS.
    pub content_hash: String,
    /// Generated CSS bytes.
    pub bytes: usize,
    /// Generated CSS content.
    pub css: String,
    /// CSS module exports materialized into scoped class names.
    pub module_exports: Vec<DxReactCssModuleExport>,
}

/// One CSS module export lowered into a deterministic scoped class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactCssModuleExport {
    /// Source `.module.css` path.
    pub source_path: String,
    /// Local CSS module export name.
    pub local_name: String,
    /// Deterministic scoped class name.
    pub class_name: String,
}

/// Decoded canonical packet proof for a React-shaped route.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactAppRoutePacketProof {
    /// Encoded `DXPK` bytes.
    pub bytes: usize,
    /// Decoded packet kind.
    pub decoded_kind: DxPacketKind,
    /// Number of decoded sections.
    pub section_count: usize,
    /// Decoded payload bytes.
    pub payload_bytes: u32,
    /// Decoded section summaries.
    pub decoded_sections: Vec<DxReactAppRoutePacketSectionProof>,
    /// Whether the canonical packet decoded back to the emitted packet.
    pub roundtrip_matches: bool,
    /// Raw encoded packet for callers that write artifacts.
    #[serde(skip, default)]
    pub encoded: Vec<u8>,
}

/// Decoded canonical packet section summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactAppRoutePacketSectionProof {
    /// Section kind.
    pub kind: DxPacketSectionKind,
    /// Section encoding.
    pub encoding: DxPacketSectionEncoding,
    /// Raw section byte length.
    pub bytes: usize,
    /// BLAKE3 content hash.
    pub content_hash: String,
}

/// Compile one React-shaped `app/` route into the canonical DX page graph.
pub fn compile_react_app_route(
    input: DxReactAppRouteInput,
) -> CompilerResult<DxReactAppRouteProof> {
    let delivery_mode = react_delivery_mode(&input);
    let style_plan = react_style_plan(&input.styles);
    let streaming = react_streaming_proof(&input, delivery_mode);
    let page_graph = react_page_graph(&input, &style_plan);
    let fallback = DxFallbackHtml::crawlable(render_react_fallback(
        &input,
        delivery_mode,
        &style_plan.generated_styles,
        &streaming,
    ));
    let packet = compile_react_app_packet(&page_graph, &fallback, delivery_mode, &streaming)?;
    let route_unit = compile_route_unit(
        &input,
        &page_graph,
        &fallback,
        delivery_mode,
        &style_plan.generated_styles,
        &streaming,
        &packet,
    );

    Ok(DxReactAppRouteProof {
        route: input.route,
        page_graph,
        fallback,
        delivery_mode,
        generated_styles: style_plan.generated_styles,
        streaming,
        packet,
        route_unit,
    })
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct DxReactStylePlan {
    graph: DxStyleGraph,
    generated_styles: Vec<DxReactGeneratedStyleAsset>,
}

fn react_page_graph(input: &DxReactAppRouteInput, style_plan: &DxReactStylePlan) -> DxPageGraph {
    let route_doc = lower_react_jsx_source(&input.route_source_path, &input.route_source);
    let segment_docs = input
        .segments
        .iter()
        .map(|segment| {
            (
                segment,
                lower_react_jsx_source(&segment.source_path, &segment.source),
            )
        })
        .collect::<Vec<_>>();
    let wrapper_nodes = input
        .segments
        .iter()
        .filter(|segment| is_wrapper_segment(segment.kind))
        .map(segment_node_id)
        .collect::<Vec<_>>();
    let root_component_id = wrapper_nodes
        .first()
        .cloned()
        .unwrap_or_else(|| "app/page".to_string());
    let mut nodes = vec![DxComponentNode {
        id: "app/page".to_string(),
        name: "Page".to_string(),
        package_id: None,
        content_hash: content_hash(&input.route_source),
    }];
    let mut edges = Vec::new();

    for segment in &input.segments {
        nodes.push(DxComponentNode {
            id: segment_node_id(segment),
            name: segment.kind.component_name().to_string(),
            package_id: None,
            content_hash: content_hash(&segment.source),
        });
    }

    edges.extend(segment_composition_edges(input));

    for component in &input.components {
        let component_id = format!("component/{}", component.name);
        nodes.push(DxComponentNode {
            id: component_id.clone(),
            name: component.name.clone(),
            package_id: component.package_id.clone(),
            content_hash: content_hash(&component.source),
        });
        if route_references_component(&route_doc, &component.name) {
            edges.push(DxComponentEdge {
                from: "app/page".to_string(),
                to: component_id.clone(),
            });
        }
        for (segment, segment_doc) in &segment_docs {
            if route_references_component(segment_doc, &component.name) {
                edges.push(DxComponentEdge {
                    from: segment_node_id(segment),
                    to: component_id.clone(),
                });
            }
        }
    }

    DxPageGraph {
        route_id: input.route.clone(),
        source_path: Some(input.route_source_path.clone()),
        root_component_id,
        components: DxComponentGraph { nodes, edges },
        styles: style_plan.graph.clone(),
        source_manifest_hash: input.source_manifest_hash.clone(),
    }
}

fn segment_composition_edges(input: &DxReactAppRouteInput) -> Vec<DxComponentEdge> {
    let wrappers = input
        .segments
        .iter()
        .filter(|segment| is_wrapper_segment(segment.kind))
        .collect::<Vec<_>>();
    let mut edges = Vec::new();
    for pair in wrappers.windows(2) {
        edges.push(DxComponentEdge {
            from: segment_node_id(pair[0]),
            to: segment_node_id(pair[1]),
        });
    }

    if let Some(page_parent) = nearest_wrapper_parent(&wrappers, &input.route_source_path) {
        edges.push(DxComponentEdge {
            from: page_parent,
            to: "app/page".to_string(),
        });
    }

    for segment in input.segments.iter().filter(|segment| {
        segment.kind != DxReactAppSegmentKind::Layout
            && segment.kind != DxReactAppSegmentKind::Template
    }) {
        let boundary_parent = nearest_wrapper_parent(&wrappers, &segment.source_path)
            .unwrap_or_else(|| "app/page".to_string());
        edges.push(DxComponentEdge {
            from: boundary_parent,
            to: segment_node_id(segment),
        });
    }

    edges
}

fn is_wrapper_segment(kind: DxReactAppSegmentKind) -> bool {
    matches!(
        kind,
        DxReactAppSegmentKind::Layout | DxReactAppSegmentKind::Template
    )
}

fn nearest_wrapper_parent(
    wrappers: &[&DxReactAppSegmentSource],
    child_source_path: &str,
) -> Option<String> {
    let child_dir = segment_source_dir(child_source_path);
    wrappers
        .iter()
        .rev()
        .find(|wrapper| {
            let wrapper_dir = segment_source_dir(&wrapper.source_path);
            source_dir_contains(&wrapper_dir, &child_dir)
        })
        .map(|segment| segment_node_id(segment))
}

fn source_dir_contains(parent: &str, child: &str) -> bool {
    child == parent
        || child
            .strip_prefix(parent)
            .is_some_and(|remaining| remaining.starts_with('/'))
}

fn segment_source_dir(source_path: &str) -> String {
    source_path
        .rsplit_once('/')
        .map(|(dir, _)| dir.to_string())
        .unwrap_or_default()
}

fn react_style_plan(styles: &[DxReactStyleSource]) -> DxReactStylePlan {
    let mut tokens = Vec::new();
    let mut classes = Vec::new();
    let mut css_blocks = Vec::new();
    let mut module_exports = Vec::new();

    for style in styles {
        let source_tokens = css_tokens(&style.source);
        tokens.extend(source_tokens);
        if is_css_module(&style.source_path) {
            let module_classes = css_classes(&style.source);
            for class in module_classes {
                let class_name = scoped_css_module_class(&style.source_path, &class.name);
                module_exports.push(DxReactCssModuleExport {
                    source_path: style.source_path.clone(),
                    local_name: class.name,
                    class_name: class_name.clone(),
                });
                classes.push(DxStyleClass {
                    name: class_name.clone(),
                    rule: class.rule.clone(),
                    source_hash: class.source_hash.clone(),
                });
                css_blocks.push(format!(
                    ".{}{{{}}}",
                    class_name,
                    minify_css_rule(&class.rule)
                ));
            }
        } else {
            classes.extend(css_classes(&style.source));
            let css = minify_css_source(&style.source);
            if !css.is_empty() {
                css_blocks.push(css);
            }
        }
    }

    dedupe_style_tokens(&mut tokens);
    dedupe_style_classes(&mut classes);
    module_exports.sort_by(|left, right| {
        left.source_path
            .cmp(&right.source_path)
            .then_with(|| left.local_name.cmp(&right.local_name))
    });
    let graph = DxStyleGraph {
        tokens,
        classes,
        delivery: DxStyleDelivery::GeneratedCss,
    };
    let css = css_blocks
        .into_iter()
        .filter(|block| !block.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    let generated_styles = if css.is_empty() {
        Vec::new()
    } else {
        let hash = content_hash(&css);
        let short_hash = hash.chars().take(16).collect::<String>();
        let output_path = format!("_dx/styles/react-route-{short_hash}.css");
        vec![DxReactGeneratedStyleAsset {
            href: format!("/{output_path}"),
            output_path,
            source_paths: styles
                .iter()
                .map(|style| style.source_path.clone())
                .collect(),
            content_hash: hash,
            bytes: css.len(),
            css,
            module_exports,
        }]
    };

    DxReactStylePlan {
        graph,
        generated_styles,
    }
}

fn react_streaming_proof(
    input: &DxReactAppRouteInput,
    delivery_mode: DxDeliveryMode,
) -> DxReactStreamingProof {
    let deferred_chunks = input
        .segments
        .iter()
        .filter_map(|segment| {
            let boundary = segment.kind.boundary_name()?.to_string();
            if boundary != "loading" {
                return None;
            }
            let id = format!("defer-{}", stable_source_id(&segment.source_path));
            let html = deferred_segment_html(segment);
            Some(DxReactDeferredChunk {
                id,
                boundary,
                source_path: segment.source_path.clone(),
                bytes: html.len(),
                html,
                flush_order: 1,
            })
        })
        .collect::<Vec<_>>();
    let client_island_manifest = react_client_island_manifest(input, delivery_mode);
    let capabilities = react_client_island_abi_capabilities();
    let resumable_islands = analyze_react_client_boundaries(&react_client_sources(input))
        .into_iter()
        .filter(|boundary| {
            boundary.use_client || boundary.state_vars > 0 || boundary.event_handlers > 0
        })
        .map(|boundary| {
            let id = format!("island-{}", stable_source_id(&boundary.source_path));
            let client_island =
                client_island_for_source(&client_island_manifest, &boundary.source_path);
            let directives = resumable_island_directives(client_island);
            let hydration_strategy = client_island
                .map(|island| island.hydration.strategy.clone())
                .unwrap_or_else(|| boundary.delivery_mode.as_str().to_string());
            DxReactResumableIsland {
                resume_selector: format!(r#"[data-dx-island="{id}"]"#),
                runtime: boundary.delivery_mode.as_str().to_string(),
                directive_style_id: capabilities.directive_style_id.clone(),
                directives: directives.clone(),
                hydration_strategy,
                source_owned_runtime: capabilities.source_owned_runtime,
                no_js_fallback_required: capabilities.no_js_fallback_required,
                full_react_hydration: capabilities.full_react_hydration,
                browser_proof_status: capabilities.browser_proof_status.clone(),
                framework_adapter: client_island_framework_adapter_status(&directives).to_string(),
                proof_scope: "local-source-owned-island-abi-foundation".to_string(),
                state_slots: boundary.state_vars,
                event_slots: boundary.event_handlers,
                source_path: boundary.source_path,
                id,
            }
        })
        .collect::<Vec<_>>();
    let enabled = !deferred_chunks.is_empty()
        || !resumable_islands.is_empty()
        || input.route_source.contains("await ");
    let strategy = if enabled {
        "shell-first-deferred-boundaries"
    } else {
        "static-single-flush"
    }
    .to_string();
    let first_flush_html = if enabled {
        first_stream_flush_html(
            input,
            delivery_mode,
            deferred_chunks.len(),
            resumable_islands.len(),
        )
    } else {
        String::new()
    };
    DxReactStreamingProof {
        enabled,
        strategy,
        shell_bytes: first_flush_html.len(),
        first_flush_html,
        deferred_chunks,
        resumable_islands,
        node_modules_required: false,
    }
}

fn react_client_island_manifest(
    input: &DxReactAppRouteInput,
    delivery_mode: DxDeliveryMode,
) -> DxReactClientIslandManifest {
    compile_react_client_islands(DxReactClientIslandInput {
        route: input.route.clone(),
        route_source_path: input.route_source_path.clone(),
        route_source: input.route_source.clone(),
        segments: input.segments.clone(),
        components: input.components.clone(),
        route_delivery_mode: delivery_mode,
    })
}

fn client_island_for_source<'a>(
    manifest: &'a DxReactClientIslandManifest,
    source_path: &str,
) -> Option<&'a DxReactClientIsland> {
    manifest
        .islands
        .iter()
        .find(|island| island.source_path == source_path)
}

fn resumable_island_directives(island: Option<&DxReactClientIsland>) -> Vec<String> {
    island
        .into_iter()
        .flat_map(|island| island.hydration.directives.iter())
        .map(|directive| directive.name.clone())
        .collect()
}

fn client_island_framework_adapter_status(directives: &[String]) -> &'static str {
    if directives.iter().any(|directive| directive == "clientOnly") {
        "preview-only"
    } else {
        "not-requested"
    }
}

fn first_stream_flush_html(
    input: &DxReactAppRouteInput,
    delivery_mode: DxDeliveryMode,
    deferred_chunks: usize,
    resumable_islands: usize,
) -> String {
    let docs = lowered_react_sources(input);
    let heading = first_raw_element_text(input, "h1")
        .or_else(|| first_element_text(&docs, "h1"))
        .unwrap_or_else(|| "www route".to_string());
    format!(
        r#"<!doctype html><html lang="en"><head><meta charset="utf-8"></head><body><main class="dx-shell" data-dx-stream-shell="true" data-dx-route="{}" data-dx-runtime="{}" data-dx-deferred-chunks="{}" data-dx-resumable-islands="{}"><h1>{}</h1></main>"#,
        escape_html(&input.route),
        delivery_mode.as_str(),
        deferred_chunks,
        resumable_islands,
        escape_html(&heading)
    )
}

fn deferred_segment_html(segment: &DxReactAppSegmentSource) -> String {
    let doc = lower_react_jsx_source(&segment.source_path, &segment.source);
    let text = first_element_text(&[doc], "p")
        .or_else(|| raw_element_text(&segment.source, "p"))
        .unwrap_or_else(|| segment.kind.component_name().to_string());
    format!(
        r#"<template data-dx-deferred-boundary="{}" data-dx-source="{}"><p>{}</p></template>"#,
        segment.kind.boundary_name().unwrap_or("segment"),
        escape_html(&segment.source_path),
        escape_html(&text)
    )
}

fn compile_react_app_packet(
    page_graph: &DxPageGraph,
    fallback: &DxFallbackHtml,
    delivery_mode: DxDeliveryMode,
    streaming: &DxReactStreamingProof,
) -> CompilerResult<DxReactAppRoutePacketProof> {
    let fallback_hash = content_hash(&fallback.html);
    let fallback_ref = serde_json::json!({
        "route": page_graph.route_id,
        "html_hash": fallback_hash,
        "html_bytes": fallback.bytes,
        "delivery_mode": delivery_mode.as_str(),
        "crawlable": fallback.crawlable,
    });
    let template_slots = serde_json::json!({
        "root_component_id": page_graph.root_component_id,
        "component_count": page_graph.components.nodes.len(),
        "edge_count": page_graph.components.edges.len(),
    });
    let source_manifest = serde_json::json!({
        "source_path": page_graph.source_path,
        "source_manifest_hash": page_graph.source_manifest_hash,
    });

    let sections = vec![
        DxPacketSection::new(
            DxPacketSectionKind::FallbackHtmlRef,
            DxPacketSectionEncoding::Json,
            serde_json::to_vec(&fallback_ref)
                .map_err(|error| CompilerError::Parse(error.to_string()))?,
        ),
        DxPacketSection::new(
            DxPacketSectionKind::TemplateSlots,
            DxPacketSectionEncoding::Json,
            serde_json::to_vec(&template_slots)
                .map_err(|error| CompilerError::Parse(error.to_string()))?,
        ),
        DxPacketSection::new(
            DxPacketSectionKind::StyleGraph,
            DxPacketSectionEncoding::Json,
            serde_json::to_vec(&page_graph.styles)
                .map_err(|error| CompilerError::Parse(error.to_string()))?,
        ),
        DxPacketSection::new(
            DxPacketSectionKind::StreamingPlan,
            DxPacketSectionEncoding::Json,
            serde_json::to_vec(streaming)
                .map_err(|error| CompilerError::Parse(error.to_string()))?,
        ),
        DxPacketSection::new(
            DxPacketSectionKind::SourceManifest,
            DxPacketSectionEncoding::Json,
            serde_json::to_vec(&source_manifest)
                .map_err(|error| CompilerError::Parse(error.to_string()))?,
        ),
    ];
    let packet = DxPacket::new(DxPacketKind::Route, sections);
    let encoded =
        DxPacketCodec::encode(&packet).map_err(|error| CompilerError::Parse(error.to_string()))?;
    let decoded =
        DxPacketCodec::decode(&encoded).map_err(|error| CompilerError::Parse(error.to_string()))?;

    Ok(DxReactAppRoutePacketProof {
        bytes: encoded.len(),
        decoded_kind: decoded.header.kind,
        section_count: decoded.sections.len(),
        payload_bytes: decoded.header.payload_len,
        decoded_sections: decoded
            .sections
            .iter()
            .map(|section| DxReactAppRoutePacketSectionProof {
                kind: section.kind,
                encoding: section.encoding,
                bytes: section.bytes.len(),
                content_hash: section.content_hash.clone(),
            })
            .collect(),
        roundtrip_matches: decoded == packet,
        encoded,
    })
}

fn render_react_fallback(
    input: &DxReactAppRouteInput,
    delivery_mode: DxDeliveryMode,
    generated_styles: &[DxReactGeneratedStyleAsset],
    streaming: &DxReactStreamingProof,
) -> String {
    let docs = lowered_react_sources(input);
    let metadata = react_route_metadata(input);
    let layout_count = input
        .segments
        .iter()
        .filter(|segment| segment.kind == DxReactAppSegmentKind::Layout)
        .count();
    let template_count = input
        .segments
        .iter()
        .filter(|segment| segment.kind == DxReactAppSegmentKind::Template)
        .count();
    let boundaries = input
        .segments
        .iter()
        .filter_map(|segment| segment.kind.boundary_name())
        .collect::<Vec<_>>()
        .join(",");
    let page_graph_ref = input
        .segments
        .iter()
        .find(|segment| segment.kind == DxReactAppSegmentKind::Layout)
        .map(|segment| segment.source_path.as_str())
        .unwrap_or(input.route_source_path.as_str());
    let page_graph_ref = page_graph_ref
        .trim_end_matches(".tsx")
        .trim_end_matches(".jsx")
        .to_string();
    let title = metadata
        .title
        .clone()
        .or_else(|| first_element_text(&docs, "h1"))
        .unwrap_or_else(|| "www route".to_string());
    let heading = first_raw_element_text(input, "h1")
        .or_else(|| first_element_text(&docs, "h1"))
        .unwrap_or_else(|| title.clone());
    let description = metadata
        .description
        .as_deref()
        .map(|description| {
            format!(
                r#"<meta name="description" content="{}">"#,
                escape_html(description)
            )
        })
        .unwrap_or_default();
    let canonical = metadata
        .canonical
        .as_deref()
        .map(|canonical| {
            format!(
                r#"<link rel="canonical" href="{}">"#,
                escape_html(canonical)
            )
        })
        .unwrap_or_default();
    let depth = input.route.split('/').filter(|s| !s.is_empty()).count();
    let relative_prefix = if delivery_mode == DxDeliveryMode::Static {
        if depth == 0 {
            "./".to_string()
        } else {
            "../".repeat(depth)
        }
    } else {
        "/".to_string()
    };

    let style_links = react_style_links(&input.styles, generated_styles, &relative_prefix);
    let asset_refs = react_asset_refs(&docs).join(",");
    let asset_attr = if asset_refs.is_empty() {
        String::new()
    } else {
        format!(r#" data-dx-assets="{}""#, escape_html(&asset_refs))
    };
    let streaming_attr = if streaming.enabled {
        format!(
            r#" data-dx-streaming="shell-first" data-dx-deferred-chunks="{}" data-dx-resumable-islands="{}""#,
            streaming.deferred_chunks.len(),
            streaming.resumable_islands.len()
        )
    } else {
        String::new()
    };
    let conditional_count = docs
        .iter()
        .map(|doc| doc.conditional_branches.len())
        .sum::<usize>();
    let list_count = docs
        .iter()
        .map(|doc| doc.list_iterations.len())
        .sum::<usize>();
    let key_count = docs
        .iter()
        .map(|doc| doc.keyed_update_hints.len())
        .sum::<usize>();
    let paragraphs = element_texts(&docs, "p")
        .into_iter()
        .filter(|text| {
            !matches!(
                text.as_str(),
                "www starter" | "Dx WWW" | "Loading" | "Route error" | "404" | "No-JS proof target"
            ) && !text.starts_with("Local interactions")
        })
        .take(1)
        .collect::<Vec<_>>();
    let paragraph_html = paragraphs
        .iter()
        .map(|text| format!("<p>{}</p>", escape_html(text)))
        .collect::<Vec<_>>()
        .join("");
    if delivery_mode == DxDeliveryMode::Static && !streaming.enabled {
        return render_tiny_static_fallback(
            input,
            &docs,
            &title,
            &description,
            &canonical,
            &style_links,
            &asset_attr,
            &relative_prefix,
        );
    }
    format!(
        r#"<!DOCTYPE html><html lang="en" class="dark"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><link rel="icon" href="{}favicon.svg" type="image/svg+xml"><link rel="apple-touch-icon" href="{}icon.svg">{}{}{}<title>{}</title></head><body class="dx-template"><main class="starter-shell" data-dx-template="minimal" data-dx-runtime="{}" data-dx-page-graph="{}" data-dx-packet-sections="{}" data-dx-layouts="{}" data-dx-templates="{}" data-dx-boundaries="{}" data-dx-conditionals="{}" data-dx-lists="{}" data-dx-keys="{}"{}{}><section class="starter-card" aria-labelledby="starter-title"><img class="starter-logo" src="{}logo.svg" alt="Dx WWW" width="40" height="40"><p class="starter-kicker">Dx WWW</p><h1 id="starter-title">{}</h1>{}<div class="starter-actions" data-dx-proof-links="state-runtime islands" aria-label="Starter proof routes"><a class="starter-link" href="{}state-runtime">Open state runtime proof</a><a class="starter-link" href="{}islands">Open island proof</a></div><form class="starter-form" action="{}state-runtime" method="get" data-dx-no-js-fallback="preserved"><p class="starter-form-label" id="starter-proof-route-label">No-JS proof target</p><div class="starter-form-panel"><input class="starter-input" name="note" aria-labelledby="starter-proof-route-label" placeholder="Optional proof note"><button class="starter-action-button" id="starter-proof-route" aria-describedby="starter-proof-route-label" type="submit">Continue without JavaScript</button></div></form></section></main></body></html>"#,
        relative_prefix,
        relative_prefix,
        description,
        canonical,
        style_links,
        escape_html(&title),
        delivery_mode.as_str(),
        escape_html(&page_graph_ref),
        5,
        layout_count,
        template_count,
        escape_html(&boundaries),
        conditional_count,
        list_count,
        key_count,
        asset_attr,
        streaming_attr,
        relative_prefix,
        escape_html(&heading),
        paragraph_html,
        relative_prefix,
        relative_prefix,
        relative_prefix
    )
}

fn render_tiny_static_fallback(
    input: &DxReactAppRouteInput,
    docs: &[DxReactJsxDocument],
    title: &str,
    description: &str,
    canonical: &str,
    style_links: &str,
    asset_attr: &str,
    relative_prefix: &str,
) -> String {
    let main_attrs = tiny_static_root_main_attributes(docs, relative_prefix);
    let content = tiny_static_content_html(docs, relative_prefix).unwrap_or_else(|| {
        let heading = first_raw_element_text(input, "h1")
            .or_else(|| first_element_text(docs, "h1"))
            .unwrap_or_else(|| title.to_string());
        let paragraphs = element_texts(docs, "p")
            .into_iter()
            .take(2)
            .map(|text| format!("<p>{}</p>", escape_html(&text)))
            .collect::<String>();
        format!("<h1>{}</h1>{paragraphs}", escape_html(&heading))
    });

    format!(
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><link rel="icon" href="{}favicon.svg" type="image/svg+xml"><link rel="apple-touch-icon" href="{}icon.svg">{}{}{}<title>{}</title></head><body><main class="{}" data-dx-runtime="static" data-dx-output-mode="tiny-static" data-dx-js="none" data-dx-route="{}"{}{}>{}</main></body></html>"#,
        relative_prefix,
        relative_prefix,
        description,
        canonical,
        style_links,
        escape_html(title),
        escape_html(&main_attrs.class_name),
        escape_html(&input.route),
        asset_attr,
        main_attrs.extra_attrs,
        content
    )
}

struct TinyStaticRootMainAttributes {
    class_name: String,
    extra_attrs: String,
}

fn tiny_static_root_main_attributes(
    docs: &[DxReactJsxDocument],
    relative_prefix: &str,
) -> TinyStaticRootMainAttributes {
    let mut class_name = "starter-shell".to_string();
    let mut extra_attrs = String::new();
    let Some(main) = docs.iter().find_map(root_static_main_element) else {
        return TinyStaticRootMainAttributes {
            class_name,
            extra_attrs,
        };
    };

    for mut attribute in main.attributes.iter().filter_map(static_html_attribute) {
        match attribute.name.as_str() {
            "class" => {
                if let Some(value) = attribute
                    .value
                    .clone()
                    .filter(|value| !value.trim().is_empty())
                {
                    class_name = value;
                }
            }
            "data-dx-route" | "data-dx-runtime" | "data-dx-output-mode" | "data-dx-js" => {}
            _ => {
                if matches!(
                    attribute.name.as_str(),
                    "src" | "href" | "poster" | "data-dx-assets"
                ) {
                    if let Some(value) = attribute.value.as_mut() {
                        if value.starts_with('/') {
                            let path = value.strip_prefix('/').unwrap_or(value);
                            *value = format!("{}{}", relative_prefix, path);
                        }
                    }
                }
                extra_attrs.push_str(&format_static_html_attribute(&attribute));
            }
        }
    }

    TinyStaticRootMainAttributes {
        class_name,
        extra_attrs,
    }
}

fn root_static_main_element(doc: &DxReactJsxDocument) -> Option<&DxReactJsxElement> {
    doc.elements.iter().find(|element| {
        element.parent_index.is_none()
            && element.name == "main"
            && is_static_intrinsic_element(&element.name)
    })
}

fn tiny_static_content_html(docs: &[DxReactJsxDocument], relative_prefix: &str) -> Option<String> {
    docs.iter().find_map(|doc| {
        doc.elements
            .iter()
            .enumerate()
            .find(|(_, element)| {
                element.parent_index.is_none() && is_static_intrinsic_element(&element.name)
            })
            .and_then(|(index, element)| {
                let html = if element.name == "main" {
                    render_static_child_nodes(doc, element, relative_prefix)
                } else {
                    render_static_intrinsic_element(doc, index, relative_prefix)
                };
                (!html.trim().is_empty()).then_some(html)
            })
    })
}

fn render_static_intrinsic_element(
    doc: &DxReactJsxDocument,
    index: usize,
    relative_prefix: &str,
) -> String {
    let Some(element) = doc.elements.get(index) else {
        return String::new();
    };
    if !is_static_intrinsic_element(&element.name) {
        return String::new();
    }

    if element.name == "Icon" {
        let name = element
            .attributes
            .iter()
            .find(|a| a.name == "name")
            .and_then(|a| a.value.as_deref())
            .unwrap_or("");
        let class_name = element
            .attributes
            .iter()
            .find(|a| a.name == "className")
            .and_then(|a| a.value.as_deref())
            .unwrap_or("");
        let mut extra_attrs = String::new();
        for attribute in element.attributes.iter() {
            if matches!(attribute.name.as_str(), "name" | "className" | "class") {
                continue;
            }
            if let Some(static_attr) = static_html_attribute(attribute) {
                extra_attrs.push_str(&format_static_html_attribute(&static_attr));
            }
        }
        return format!(
            r#"<span data-dx-icon="{}" class="{}" aria-hidden="true" data-icon-source="dx-icons"{}></span>"#,
            escape_html(name),
            escape_html(class_name),
            extra_attrs
        );
    }

    let attrs = static_attribute_html(element, relative_prefix);
    if is_void_html_element(&element.name) {
        return format!("<{}{}>", element.name, attrs);
    }

    let children = render_static_child_nodes(doc, element, relative_prefix);
    format!("<{}{}>{}</{}>", element.name, attrs, children, element.name)
}

fn render_static_child_nodes(
    doc: &DxReactJsxDocument,
    element: &DxReactJsxElement,
    relative_prefix: &str,
) -> String {
    element
        .child_nodes
        .iter()
        .filter_map(|child| match child {
            DxReactJsxChildNode::Text { value } => {
                meaningful_static_text(value).map(|text| escape_html(&text))
            }
            DxReactJsxChildNode::Expression { expression } => {
                static_string_expression(expression).map(|text| escape_html(&text))
            }
            DxReactJsxChildNode::Element { index } => {
                let html = render_static_intrinsic_element(doc, *index, relative_prefix);
                (!html.is_empty()).then_some(html)
            }
        })
        .collect::<String>()
}

fn static_attribute_html(element: &DxReactJsxElement, relative_prefix: &str) -> String {
    element
        .attributes
        .iter()
        .filter_map(static_html_attribute)
        .map(|mut attribute| {
            if matches!(
                attribute.name.as_str(),
                "src" | "href" | "poster" | "data-dx-assets"
            ) {
                if let Some(value) = attribute.value.as_mut() {
                    if value.starts_with('/') {
                        let path = value.strip_prefix('/').unwrap_or(value);
                        *value = format!("{}{}", relative_prefix, path);
                    }
                }
            }
            format_static_html_attribute(&attribute)
        })
        .collect()
}

struct StaticHtmlAttribute {
    name: String,
    value: Option<String>,
}

enum StaticHtmlAttributeExpression {
    Bare,
    Value(String),
}

fn static_html_attribute(
    attribute: &super::jsx_lowering::DxReactJsxAttribute,
) -> Option<StaticHtmlAttribute> {
    if attribute.name.starts_with("on") || attribute.name == "key" {
        return None;
    }
    let name = static_html_attribute_name(&attribute.name);
    if !is_static_html_attribute_name(name) {
        return None;
    }
    let value = if let Some(expression) = attribute.expression.as_deref() {
        match static_attribute_expression_value(name, expression)? {
            StaticHtmlAttributeExpression::Bare => None,
            StaticHtmlAttributeExpression::Value(value) => Some(value),
        }
    } else {
        attribute.value.clone()
    };
    if value
        .as_deref()
        .is_some_and(|value| value.trim().is_empty())
    {
        return None;
    }
    Some(StaticHtmlAttribute {
        name: name.to_string(),
        value,
    })
}

fn format_static_html_attribute(attribute: &StaticHtmlAttribute) -> String {
    match attribute.value.as_deref() {
        Some(value) => format!(r#" {}="{}""#, attribute.name, escape_html(value)),
        None => format!(" {}", attribute.name),
    }
}

fn static_attribute_expression_value(
    name: &str,
    expression: &str,
) -> Option<StaticHtmlAttributeExpression> {
    if name == "class" {
        return static_class_expression(expression).map(StaticHtmlAttributeExpression::Value);
    }
    let expression = strip_static_parentheses(expression.trim());
    if matches!(expression, "null" | "undefined") {
        return None;
    }
    if expression == "true" {
        return Some(if is_boolean_static_html_attribute(name) {
            StaticHtmlAttributeExpression::Bare
        } else {
            StaticHtmlAttributeExpression::Value("true".to_string())
        });
    }
    if expression == "false" {
        return if is_boolean_static_html_attribute(name) {
            None
        } else {
            Some(StaticHtmlAttributeExpression::Value("false".to_string()))
        };
    }
    if let Some(value) = static_quoted_string_literal(expression) {
        return Some(StaticHtmlAttributeExpression::Value(value));
    }
    if is_static_number_literal(expression) {
        return Some(StaticHtmlAttributeExpression::Value(expression.to_string()));
    }
    None
}

fn static_class_expression(expression: &str) -> Option<String> {
    let expression = strip_static_parentheses(expression.trim());
    static_quoted_string_literal(expression).or_else(|| static_class_call_expression(expression))
}

fn static_class_call_expression(expression: &str) -> Option<String> {
    let arguments = static_class_call_arguments(expression)?;
    let mut classes = Vec::new();
    for argument in split_static_comma_separated_items(arguments)? {
        let argument = strip_static_parentheses(argument.trim());
        if matches!(argument, "" | "false" | "true" | "null" | "undefined") {
            continue;
        }
        let class_name = static_quoted_string_literal(argument)?;
        if !class_name.trim().is_empty() {
            classes.push(class_name);
        }
    }
    Some(classes.join(" "))
}

fn static_class_call_arguments(expression: &str) -> Option<&str> {
    let expression = strip_static_parentheses(expression.trim());
    for callee in ["classes", "classNames", "clsx", "cn", "cx", "dxClass"] {
        let Some(rest) = expression.strip_prefix(callee) else {
            continue;
        };
        let rest = rest.trim_start();
        if rest.starts_with('(') && rest.ends_with(')') && rest.len() >= 2 {
            return Some(&rest[1..rest.len() - 1]);
        }
    }
    None
}

fn split_static_comma_separated_items(expression: &str) -> Option<Vec<&str>> {
    let mut items = Vec::new();
    let mut start = 0usize;
    let mut quote: Option<char> = None;
    let mut escaped = false;
    let mut depth = 0i32;

    for (index, character) in expression.char_indices() {
        if let Some(active_quote) = quote {
            if escaped {
                escaped = false;
                continue;
            }
            if character == '\\' {
                escaped = true;
                continue;
            }
            if character == active_quote {
                quote = None;
            }
            continue;
        }

        match character {
            '"' | '\'' | '`' => quote = Some(character),
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => {
                depth -= 1;
                if depth < 0 {
                    return None;
                }
            }
            ',' if depth == 0 => {
                items.push(expression[start..index].trim());
                start = index + character.len_utf8();
            }
            _ => {}
        }
    }

    if quote.is_some() || escaped || depth != 0 {
        return None;
    }
    items.push(expression[start..].trim());
    Some(items)
}

fn static_quoted_string_literal(expression: &str) -> Option<String> {
    let expression = strip_static_parentheses(expression.trim());
    let quote = expression.chars().next()?;
    if !matches!(quote, '"' | '\'' | '`') {
        return None;
    }
    let value = expression.strip_prefix(quote)?.strip_suffix(quote)?;
    if quote == '`' && value.contains("${") {
        return None;
    }
    Some(unescape_static_string(value, quote))
}

fn is_static_number_literal(expression: &str) -> bool {
    if expression.is_empty() {
        return false;
    }
    let unsigned = expression.strip_prefix('-').unwrap_or(expression);
    if unsigned.is_empty() {
        return false;
    }
    let mut dot_count = 0usize;
    let mut digit_count = 0usize;
    for character in unsigned.chars() {
        match character {
            '0'..='9' => digit_count += 1,
            '_' => {}
            '.' if dot_count == 0 => dot_count += 1,
            _ => return false,
        }
    }
    digit_count > 0
}

fn static_html_attribute_name(name: &str) -> &str {
    match name {
        "className" => "class",
        "htmlFor" => "for",
        "inputMode" => "inputmode",
        "autoComplete" => "autocomplete",
        "autoCapitalize" => "autocapitalize",
        "enterKeyHint" => "enterkeyhint",
        "spellCheck" => "spellcheck",
        "tabIndex" => "tabindex",
        "contentEditable" => "contenteditable",
        "playsInline" => "playsinline",
        "allowFullScreen" => "allowfullscreen",
        "readOnly" => "readonly",
        "srcSet" => "srcset",
        "fetchPriority" => "fetchpriority",
        "crossOrigin" => "crossorigin",
        "referrerPolicy" => "referrerpolicy",
        "noModule" => "nomodule",
        other => other,
    }
}

fn is_boolean_static_html_attribute(name: &str) -> bool {
    matches!(
        name,
        "async"
            | "defer"
            | "nomodule"
            | "required"
            | "disabled"
            | "checked"
            | "selected"
            | "multiple"
            | "readonly"
            | "hidden"
            | "playsinline"
            | "allowfullscreen"
    )
}

fn unescape_static_string(value: &str, quote: char) -> String {
    let mut output = String::with_capacity(value.len());
    let mut escaped = false;
    for character in value.chars() {
        if escaped {
            match character {
                'n' => output.push('\n'),
                'r' => output.push('\r'),
                't' => output.push('\t'),
                '\\' => output.push('\\'),
                '"' if quote == '"' => output.push('"'),
                '\'' if quote == '\'' => output.push('\''),
                '`' if quote == '`' => output.push('`'),
                other => {
                    output.push('\\');
                    output.push(other);
                }
            }
            escaped = false;
        } else if character == '\\' {
            escaped = true;
        } else {
            output.push(character);
        }
    }
    if escaped {
        output.push('\\');
    }
    output
}

fn strip_static_parentheses(expression: &str) -> &str {
    let mut current = expression.trim();
    while current.starts_with('(')
        && current.ends_with(')')
        && static_outer_parentheses_wrap_expression(current)
    {
        current = current[1..current.len() - 1].trim();
    }
    current
}

fn static_outer_parentheses_wrap_expression(expression: &str) -> bool {
    let mut depth = 0i32;
    let mut quote: Option<char> = None;
    let mut escaped = false;

    for (index, character) in expression.char_indices() {
        if let Some(active_quote) = quote {
            if escaped {
                escaped = false;
                continue;
            }
            if character == '\\' {
                escaped = true;
                continue;
            }
            if character == active_quote {
                quote = None;
            }
            continue;
        }

        match character {
            '"' | '\'' | '`' => quote = Some(character),
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 && index + character.len_utf8() < expression.len() {
                    return false;
                }
                if depth < 0 {
                    return false;
                }
            }
            _ => {}
        }
    }

    depth == 0 && quote.is_none() && !escaped
}

fn static_string_expression(expression: &str) -> Option<String> {
    let expression = expression.trim();
    let quote = expression.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    expression
        .strip_prefix(quote)
        .and_then(|value| value.strip_suffix(quote))
        .map(str::to_string)
        .and_then(|text| meaningful_static_text(&text))
}

fn is_static_intrinsic_element(name: &str) -> bool {
    if name == "Icon" {
        return true;
    }
    name.chars()
        .next()
        .is_some_and(|first| first.is_ascii_lowercase())
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-')
}

fn is_static_html_attribute_name(name: &str) -> bool {
    name.chars()
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || matches!(first, '_' | ':'))
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | ':' | '.'))
}

fn is_void_html_element(name: &str) -> bool {
    matches!(
        name,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}

#[derive(Debug, Default)]
struct DxReactRouteMetadata {
    title: Option<String>,
    description: Option<String>,
    canonical: Option<String>,
}

fn react_route_metadata(input: &DxReactAppRouteInput) -> DxReactRouteMetadata {
    let sources = std::iter::once(input.route_source.as_str())
        .chain(input.segments.iter().map(|segment| segment.source.as_str()))
        .collect::<Vec<_>>();
    let metadata = sources.iter().enumerate().find_map(|(index, source)| {
        parse_tsx_module(&format!("metadata-source-{index}.tsx"), source).metadata
    });
    metadata
        .map(|metadata| DxReactRouteMetadata {
            title: metadata.title,
            description: metadata.description,
            canonical: metadata.canonical,
        })
        .unwrap_or_default()
}

fn react_style_links(
    styles: &[DxReactStyleSource],
    generated_styles: &[DxReactGeneratedStyleAsset],
    relative_prefix: &str,
) -> String {
    let mut links = styles
        .iter()
        .filter(|style| style.source_path.ends_with(".css") && !is_css_module(&style.source_path))
        .map(|style| {
            let path = style.source_path.replace('\\', "/");
            let path = path.strip_prefix('/').unwrap_or(&path);
            format!(
                r#"<link rel="stylesheet" href="{}{}">"#,
                relative_prefix,
                escape_html(path)
            )
        })
        .collect::<Vec<_>>();
    links.extend(generated_styles.iter().map(|asset| {
        let path = asset.href.strip_prefix('/').unwrap_or(&asset.href);
        format!(
            r#"<link rel="stylesheet" href="{}{}" data-dx-generated="true">"#,
            relative_prefix,
            escape_html(path)
        )
    }));
    links.join("")
}

fn react_asset_refs(docs: &[DxReactJsxDocument]) -> Vec<String> {
    let mut assets = docs
        .iter()
        .flat_map(|doc| doc.elements.iter())
        .filter_map(|element| element.attribute("src"))
        .filter(|src| src.starts_with('/'))
        .map(str::to_string)
        .collect::<Vec<_>>();
    assets.sort();
    assets.dedup();
    assets
}

fn react_delivery_mode(input: &DxReactAppRouteInput) -> DxDeliveryMode {
    let sources = react_client_sources(input);
    let boundaries = analyze_react_client_boundaries(&sources);
    select_react_delivery_mode(&boundaries)
}

fn lowered_react_sources(input: &DxReactAppRouteInput) -> Vec<DxReactJsxDocument> {
    let mut docs = vec![lower_react_jsx_source(
        &input.route_source_path,
        &input.route_source,
    )];
    docs.extend(
        input
            .segments
            .iter()
            .map(|segment| lower_react_jsx_source(&segment.source_path, &segment.source)),
    );
    docs.extend(
        input
            .components
            .iter()
            .map(|component| lower_react_jsx_source(&component.source_path, &component.source)),
    );
    docs
}

fn react_client_sources(input: &DxReactAppRouteInput) -> Vec<DxReactClientSource> {
    let mut sources = vec![DxReactClientSource {
        source_path: input.route_source_path.clone(),
        source: input.route_source.clone(),
    }];
    let referenced_components = route_scoped_component_names(input);
    sources.extend(
        input
            .components
            .iter()
            .filter(|&component| referenced_components.contains(&component.name))
            .map(|component| DxReactClientSource {
                source_path: component.source_path.clone(),
                source: component.source.clone(),
            }),
    );
    sources.extend(input.segments.iter().map(|segment| DxReactClientSource {
        source_path: segment.source_path.clone(),
        source: segment.source.clone(),
    }));
    sources
}

fn route_scoped_component_names(input: &DxReactAppRouteInput) -> HashSet<String> {
    let route_doc = lower_react_jsx_source(&input.route_source_path, &input.route_source);
    let segment_docs = input
        .segments
        .iter()
        .map(|segment| lower_react_jsx_source(&segment.source_path, &segment.source))
        .collect::<Vec<_>>();
    let component_docs = input
        .components
        .iter()
        .map(|component| {
            (
                component.name.as_str(),
                lower_react_jsx_source(&component.source_path, &component.source),
            )
        })
        .collect::<Vec<_>>();
    let mut referenced = HashSet::new();

    for component in &input.components {
        if route_references_component(&route_doc, &component.name)
            || segment_docs
                .iter()
                .any(|segment_doc| route_references_component(segment_doc, &component.name))
        {
            referenced.insert(component.name.clone());
        }
    }

    let mut changed = true;
    while changed {
        changed = false;
        for (component_name, component_doc) in &component_docs {
            if !referenced.contains(*component_name) {
                continue;
            }
            for child in &input.components {
                if !referenced.contains(&child.name)
                    && route_references_component(component_doc, &child.name)
                {
                    referenced.insert(child.name.clone());
                    changed = true;
                }
            }
        }
    }

    referenced
}

fn route_references_component(route_doc: &DxReactJsxDocument, component_name: &str) -> bool {
    let element_names = route_doc
        .elements
        .iter()
        .map(|element| element.name.as_str())
        .collect::<Vec<_>>();
    if element_names.contains(&component_name) {
        return true;
    }

    route_doc.imports.iter().any(|import| {
        import.specifiers.iter().any(|specifier| {
            !specifier.type_only
                && specifier.imported == component_name
                && element_names.contains(&specifier.local.as_str())
        }) || (!import.type_only
            && import
                .default
                .as_deref()
                .is_some_and(|local| local == component_name && element_names.contains(&local)))
    })
}

fn segment_node_id(segment: &DxReactAppSegmentSource) -> String {
    let path = segment.source_path.replace('\\', "/");
    let path = path
        .strip_prefix("app/")
        .unwrap_or(path.as_str())
        .trim_end_matches(".tsx")
        .trim_end_matches(".jsx");
    format!("app/{path}")
}

fn first_element_text(docs: &[DxReactJsxDocument], name: &str) -> Option<String> {
    element_texts(docs, name).into_iter().next()
}

fn first_raw_element_text(input: &DxReactAppRouteInput, name: &str) -> Option<String> {
    std::iter::once(input.route_source.as_str())
        .chain(
            input
                .components
                .iter()
                .map(|component| component.source.as_str()),
        )
        .chain(input.segments.iter().map(|segment| segment.source.as_str()))
        .find_map(|source| raw_element_text(source, name))
}

fn raw_element_text(source: &str, name: &str) -> Option<String> {
    let open = format!("<{name}");
    let close = format!("</{name}>");
    let start = source.find(&open)?;
    let after_open = source[start..].find('>').map(|offset| start + offset + 1)?;
    let end = source[after_open..]
        .find(&close)
        .map(|offset| after_open + offset)?;
    let text = source[after_open..end]
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    meaningful_static_text(&text)
}

fn element_texts(docs: &[DxReactJsxDocument], name: &str) -> Vec<String> {
    docs.iter()
        .flat_map(|doc| {
            doc.elements
                .iter()
                .filter(move |element| element.name == name)
                .filter_map(element_visible_text)
        })
        .collect()
}

fn element_visible_text(element: &DxReactJsxElement) -> Option<String> {
    let text = element.text_content();
    meaningful_static_text(&text)
}

fn meaningful_static_text(text: &str) -> Option<String> {
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if text.is_empty() {
        return None;
    }

    let lower = text.to_ascii_lowercase();
    let looks_dynamic = text.contains('{')
        || text.contains('}')
        || lower.contains("children")
        || lower.contains("props.")
        || lower.contains("error.message")
        || lower.contains("=>");
    if looks_dynamic { None } else { Some(text) }
}

fn css_tokens(source: &str) -> Vec<DxStyleToken> {
    let Ok(re) = regex::Regex::new(r#"(--[A-Za-z0-9_-]+)\s*:\s*([^;}{]+)"#) else {
        return Vec::new();
    };
    re.captures_iter(source)
        .filter_map(|capture| {
            Some(DxStyleToken {
                name: capture.get(1)?.as_str().trim().to_string(),
                value: capture.get(2)?.as_str().trim().to_string(),
            })
        })
        .collect()
}

fn css_classes(source: &str) -> Vec<DxStyleClass> {
    let Ok(re) = regex::Regex::new(r#"\.([A-Za-z0-9_-]+)\s*\{([^}]*)\}"#) else {
        return Vec::new();
    };
    re.captures_iter(source)
        .filter_map(|capture| {
            let rule = capture
                .get(2)?
                .as_str()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            Some(DxStyleClass {
                name: capture.get(1)?.as_str().to_string(),
                source_hash: Some(content_hash(&rule)),
                rule,
            })
        })
        .collect()
}

fn is_css_module(source_path: &str) -> bool {
    source_path.replace('\\', "/").ends_with(".module.css")
}

fn scoped_css_module_class(source_path: &str, local_name: &str) -> String {
    let hash = content_hash(&format!("{source_path}:{local_name}"));
    let short_hash = hash.chars().take(8).collect::<String>();
    format!("{local_name}__{short_hash}")
}

fn stable_source_id(source_path: &str) -> String {
    content_hash(source_path).chars().take(12).collect()
}

fn minify_css_source(source: &str) -> String {
    let without_comments = strip_css_comments(source);
    let mut css = without_comments
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    for (from, to) in [
        (" {", "{"),
        ("{ ", "{"),
        (" }", "}"),
        ("} ", "}"),
        (": ", ":"),
        ("; ", ";"),
        (", ", ","),
    ] {
        css = css.replace(from, to);
    }
    css.trim().trim_end_matches(';').to_string()
}

fn minify_css_rule(rule: &str) -> String {
    minify_css_source(rule)
        .trim()
        .trim_start_matches('{')
        .trim_end_matches('}')
        .trim_end_matches(';')
        .to_string()
}

fn strip_css_comments(source: &str) -> String {
    let Ok(comment_re) = regex::Regex::new(r#"(?s)/\*.*?\*/"#) else {
        return source.to_string();
    };
    comment_re.replace_all(source, "").to_string()
}

fn dedupe_style_tokens(tokens: &mut Vec<DxStyleToken>) {
    tokens.sort_by(|left, right| left.name.cmp(&right.name));
    tokens.dedup_by(|left, right| left.name == right.name);
}

fn dedupe_style_classes(classes: &mut Vec<DxStyleClass>) {
    classes.sort_by(|left, right| left.name.cmp(&right.name));
    classes.dedup_by(|left, right| left.name == right.name);
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn content_hash(value: &str) -> String {
    blake3::hash(value.as_bytes()).to_hex().to_string()
}
