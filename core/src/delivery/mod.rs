//! Adaptive delivery planning and compact DX packet encoders.
//!
//! This module is the first production slice of the www meaning-aware
//! compiler. It keeps the existing HTIP pipeline intact while adding a planner
//! and concrete packet encoders for static output, generated JS, template
//! slots, columnar slots, semantic codecs, patch streams, and server fragments.

mod app_route;
mod client_boundary;
mod client_island;
mod conformance;
mod contract;
mod dom_events;
mod encoding;
mod global_store;
mod html;
mod import_resolution;
mod jsx_lowering;
mod micro_js;
mod plan;
mod react_state;
mod route_handler_ai;
mod route_handler_automations;
mod route_handler_body_boundary;
mod route_handler_compat;
mod route_handler_database_orm;
mod route_handler_fumadocs;
mod route_handler_http_json;
mod route_handler_instant_readiness;
mod route_handler_payments;
mod route_handler_supabase;
mod route_unit;
mod samples;
mod server_contract;
mod tsx_ast;
mod types;
mod vertical;
mod vertical_interaction;
mod vertical_render;

pub use app_route::{
    DxReactAppRouteInput, DxReactAppRoutePacketProof, DxReactAppRoutePacketSectionProof,
    DxReactAppRouteProof, DxReactAppSegmentKind, DxReactAppSegmentSource, DxReactComponentSource,
    DxReactCssModuleExport, DxReactDeferredChunk, DxReactGeneratedStyleAsset,
    DxReactResumableIsland, DxReactStreamingProof, DxReactStyleSource, compile_react_app_route,
};
pub use client_boundary::{
    DxReactClientBoundary, DxReactClientSource, analyze_react_client_boundaries,
    select_react_delivery_mode,
};
pub use client_island::{
    DxReactClientIsland, DxReactClientIslandAbi, DxReactClientIslandAbiCapabilities,
    DxReactClientIslandDirective, DxReactClientIslandDynamicImport, DxReactClientIslandEvent,
    DxReactClientIslandForm, DxReactClientIslandFormField, DxReactClientIslandHydration,
    DxReactClientIslandHydrationEvent, DxReactClientIslandHydrationRuntime,
    DxReactClientIslandInput, DxReactClientIslandKeyedUpdate, DxReactClientIslandManifest,
    DxReactClientIslandMicroJs, DxReactClientIslandProp, DxReactClientIslandState,
    compile_react_client_islands, react_client_island_abi_capabilities,
    react_client_island_micro_js_bundle,
};
pub use conformance::{
    DxNextConformanceFixture, DxNextConformanceReport, next_app_router_conformance_report,
};
pub use contract::{
    DX_PACKET_MAGIC, DX_PACKET_VERSION, DxAdaptiveRuntimeReport, DxComponentEdge, DxComponentGraph,
    DxComponentNode, DxDerivedStateSlot, DxFallbackHtml, DxGlobalStore, DxGlobalStoreAction,
    DxPacket, DxPacketCodec, DxPacketCodecError, DxPacketHeader, DxPacketKind, DxPacketSection,
    DxPacketSectionEncoding, DxPacketSectionKind, DxPageGraph, DxRouteClientIslandAbiReceipt,
    DxRoutePacketRef, DxRouteReceipt, DxRouteShell, DxRouteUnit, DxRuntimeCandidate,
    DxRuntimeRejection, DxServerActionEdge, DxSourceManifest, DxStateEffectSlot, DxStateEventSlot,
    DxStateGraph, DxStateScope, DxStateSlot, DxStyleClass, DxStyleDelivery, DxStyleGraph,
    DxStyleToken,
};
pub use dom_events::{
    NativeDomEventCatalogIntegrity, native_dom_event_catalog_integrity, native_dom_event_names,
    react_style_event_attribute_to_dom_event,
};
pub use encoding::DxPacketEncoder;
pub use html::{
    DxHtmlRouteProfile, DxOptimizedHtml, minify_generated_html, optimize_generated_html,
};
pub use import_resolution::{
    DxReactForgeOwnedFile, DxReactImportAlias, DxReactImportResolutionKind,
    DxReactImportResolverConfig, DxReactResolvedImport, DxReactReviewedAdapter,
    resolve_react_imports,
};
pub use jsx_lowering::{
    DxReactEventAttribute, DxReactImport, DxReactImportSpecifier, DxReactJsxAttribute,
    DxReactJsxChildNode, DxReactJsxConditionalBranch, DxReactJsxDocument, DxReactJsxElement,
    DxReactJsxKeyHint, DxReactJsxListIteration, lower_react_jsx_source,
};
pub use micro_js::DxMicroJsEmitter;
pub use plan::plan_delivery;
pub use samples::{
    sample_counter_micro_program, sample_dashboard_column_batch, sample_dashboard_patch_ops,
    template_dictionary_terms,
};
pub use server_contract::{
    DxReactRouteHandlerRequest, DxReactRouteHandlerResponse, DxReactServerActionProtocol,
    DxReactServerActionReceipt, DxReactServerActionRequest, DxReactServerActionResponse,
    DxReactServerActionSchema, DxReactServerActionSchemaField, DxReactServerContract,
    DxReactServerDataEntry, DxReactServerDataManifest, DxReactServerExport,
    DxReactServerLoaderResponse, DxReactServerSource, DxReactServerSourceKind,
    compile_react_server_action_protocols, compile_react_server_contracts,
    compile_react_server_data_manifest, execute_react_route_handler, execute_react_server_action,
    execute_react_server_loader,
};
pub use tsx_ast::{
    DxTsxDiagnostic, DxTsxModuleAst, DxTsxParserBackend, DxTsxParserBackendValidation,
    DxTsxRouteMetadata, DxTsxSpan, parse_tsx_module,
};
pub use types::{
    DxColumn, DxColumnBatch, DxColumnData, DxDeliveryEstimates, DxDeliveryMode, DxDeliveryPlan,
    DxMicroJsAction, DxMicroJsOp, DxMicroJsProgram, DxPatchOp, DxSemanticSequence, DxShapeStats,
    DxSlotKind,
};
pub use vertical::{
    DxVerticalBrowserPacketProof, DxVerticalBrowserPacketSectionProof, DxVerticalComponentSource,
    DxVerticalDecodedBinding, DxVerticalDecodedSlot, DxVerticalDecodedTemplate,
    DxVerticalPacketProof, DxVerticalSliceInput, DxVerticalSliceProof, compile_vertical_slice,
};
pub use vertical_interaction::DxVerticalInteractionProof;

#[cfg(test)]
mod tests;
