use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::{Component, Path, PathBuf};

use dx_compiler::delivery::{
    DxDerivedStateSlot, DxReactAppRouteProof, DxReactAppSegmentKind, DxReactAppSegmentSource,
    DxReactImport, DxReactJsxAttribute, DxReactJsxChildNode, DxReactJsxDocument, DxReactJsxElement,
    DxStateEventSlot, DxStateGraph, DxStateScope, DxStateSlot, lower_react_jsx_source,
};
use serde_json::{Value, json};

use super::next_custom_transforms::collect_font_loader_detections;
use super::next_navigation::{detects_next_navigation_not_found, next_navigation_redirect};
use super::request_props::{
    collect_next_app_router_page_prop_aliases, request_prop_bindings,
    request_prop_bindings_manifest, resolve_next_app_router_page_prop_identifier,
};

const MAX_SOURCE_IMPORT_DEPTH: usize = 2;
const MAX_SOURCE_IMPORT_DOCUMENTS: usize = 24;

#[allow(clippy::too_many_arguments)]
pub(super) fn build_tsx_source_render_surface(
    cwd: &Path,
    route: &str,
    route_source_path: &str,
    route_source: &str,
    segments: &[DxReactAppSegmentSource],
    proof: &DxReactAppRouteProof,
    route_params: &BTreeMap<String, String>,
    search_params: &BTreeMap<String, String>,
) -> Value {
    let SourceRenderDocumentSet {
        documents,
        skipped_imports,
    } = lowered_documents(cwd, route_source_path, route_source, segments);
    let request_prop_aliases =
        collect_next_app_router_page_prop_aliases(&documents, route_params, search_params);
    let request_prop_bindings =
        request_prop_bindings(route_params, search_params, &request_prop_aliases.resolved);
    let component_names = proof
        .page_graph
        .components
        .nodes
        .iter()
        .map(|node| node.name.as_str())
        .collect::<BTreeSet<_>>();
    let renderable_elements = documents
        .iter()
        .flat_map(renderable_elements)
        .collect::<Vec<_>>();
    let component_references = documents
        .iter()
        .flat_map(|document| component_references(document, &component_names))
        .collect::<Vec<_>>();
    let component_compositions = component_compositions(
        cwd,
        &documents,
        &proof.route_unit.state,
        &request_prop_bindings,
    );
    let prop_bindings = documents.iter().flat_map(prop_bindings).collect::<Vec<_>>();
    let form_surfaces = documents.iter().flat_map(form_surfaces).collect::<Vec<_>>();
    let dom_action_descriptor_sets =
        source_dom_action_descriptor_sets(&documents, &proof.route_unit.state);
    let dom_action_descriptor_count = dom_action_descriptor_sets
        .iter()
        .filter_map(|set| set.get("descriptor_count").and_then(Value::as_u64))
        .sum::<u64>() as usize;
    let dom_action_binder =
        build_dom_action_binder(&dom_action_descriptor_sets, dom_action_descriptor_count);
    let client_islands = build_client_island_manifest(
        route,
        &dom_action_descriptor_sets,
        dom_action_descriptor_count,
        &proof.route_unit.state,
    );
    let client_island_count = client_islands
        .get("island_count")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let state_dom_reflection =
        build_state_dom_reflection_plan(route, &documents, &proof.route_unit.state);
    let state_dom_reflection_count = state_dom_reflection
        .get("reflection_count")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let static_dom_snapshot = build_static_dom_snapshot(
        route,
        &documents,
        &proof.route_unit.state,
        &request_prop_bindings,
    );
    let static_dom_snapshot_elements = static_dom_snapshot
        .get("elements")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    let static_dom_snapshot_literal_expressions = static_dom_snapshot
        .get("literal_expressions")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let composed_static_dom_snapshot = build_composed_static_dom_snapshot(
        route,
        &documents,
        &component_compositions,
        &proof.route_unit.state,
        &request_prop_bindings,
    );
    let composed_static_dom_snapshot_elements = composed_static_dom_snapshot
        .get("elements")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    let component_preview_insertions = composed_static_dom_snapshot
        .get("component_preview_insertions")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let unresolved_components = component_references
        .iter()
        .filter(|reference| !reference["resolved_in_graph"].as_bool().unwrap_or(false))
        .count();
    let parser_backends = parser_backend_surfaces(&documents);
    let oxc_validated_documents = parser_backends
        .iter()
        .filter(|backend| {
            backend
                .get("active_backend")
                .and_then(Value::as_str)
                .is_some_and(|active_backend| active_backend == "oxc-parser")
        })
        .count();
    let jsx_backends = jsx_backend_surfaces(&documents);
    let oxc_jsx_documents = jsx_backends
        .iter()
        .filter(|backend| {
            backend
                .get("active_backend")
                .and_then(Value::as_str)
                .is_some_and(|active_backend| active_backend == "oxc-jsx-ast")
        })
        .count();

    json!({
        "schema": "dx.tsx.sourceRenderSurface",
        "schema_revision": 1,
        "contract_name": "TSX Source Render Surface",
        "route": route,
        "status": if unresolved_components == 0 { "source-surface-ready" } else { "source-surface-with-unresolved-components" },
        "public_authoring": "tsx",
        "full_jsx_execution": false,
        "claim": "Source-owned TSX render surface, not full JSX execution.",
        "documents": documents.iter().map(|document| json!({
            "role": document.role,
            "source_path": &document.source_path,
            "imports": document.document.imports.len(),
            "elements": document.document.elements.len(),
            "events": document.document.event_attributes.len(),
            "diagnostics": document.document.diagnostics.len(),
            "parser_backend": &document.document.parser_backend.active_backend,
            "parser_backend_status": &document.document.parser_backend.status,
            "jsx_backend": &document.document.jsx_backend.active_backend,
            "jsx_backend_status": &document.document.jsx_backend.status,
        })).collect::<Vec<_>>(),
        "counts": {
            "renderable_elements": renderable_elements.len(),
            "component_references": component_references.len(),
            "source_owned_component_compositions": component_compositions.len(),
            "prop_bindings": prop_bindings.len(),
            "page_prop_bindings": request_prop_bindings.len(),
            "page_prop_alias_bindings": request_prop_aliases.resolved.len(),
            "page_prop_unresolved_alias_bindings": request_prop_aliases.unresolved.len(),
            "form_surfaces": form_surfaces.len(),
            "unresolved_components": unresolved_components,
            "source_owned_imports_scanned": documents.iter().filter(|document| document.role == "source-owned-import").count(),
            "source_owned_imports_skipped": skipped_imports.len(),
            "static_dom_snapshot_elements": static_dom_snapshot_elements,
            "static_dom_snapshot_literal_expressions": static_dom_snapshot_literal_expressions,
            "composed_static_dom_snapshot_elements": composed_static_dom_snapshot_elements,
            "component_preview_insertions": component_preview_insertions,
            "dom_action_descriptors": dom_action_descriptor_count,
            "client_islands": client_island_count,
            "state_dom_reflections": state_dom_reflection_count,
            "oxc_validated_documents": oxc_validated_documents,
            "oxc_jsx_documents": oxc_jsx_documents,
        },
        "parser_backends": parser_backends,
        "jsx_backends": jsx_backends,
        "import_scan": {
            "mode": "bounded-source-owned-tsx",
            "max_depth": MAX_SOURCE_IMPORT_DEPTH,
            "max_documents": MAX_SOURCE_IMPORT_DOCUMENTS,
            "skipped_imports": skipped_imports,
        },
        "request_prop_bindings": request_prop_bindings_manifest(route_params, search_params, &request_prop_aliases.resolved, &request_prop_aliases.unresolved, &request_prop_bindings),
        "renderable_elements": renderable_elements,
        "component_references": component_references,
        "component_compositions": component_compositions,
        "prop_bindings": prop_bindings,
        "form_surfaces": form_surfaces,
        "dom_action_binder": dom_action_binder,
        "client_islands": client_islands,
        "state_dom_reflection": state_dom_reflection,
        "static_dom_snapshot": static_dom_snapshot,
        "composed_static_dom_snapshot": composed_static_dom_snapshot,
        "adapter_boundary_gaps": [
            "Execute general source-owned component functions beyond the bounded static preview composition surface.",
            "Expand safe TSX expression evaluation beyond literal props, route params, search params, class helpers, and static conditional/template bindings.",
            "Bind generated state/event runtime operations to the real rendered DOM for client islands.",
            "Define DX-native effect() ordering and adapter diagnostics before accepting React-specific runtime semantics."
        ],
    })
}

fn jsx_backend_surfaces(documents: &[LoweredSourceDocument]) -> Vec<Value> {
    documents
        .iter()
        .map(|document| {
            let backend = &document.document.jsx_backend;
            json!({
                "schema": &backend.schema,
                "schema_revision": backend.schema_revision,
                "source_path": &document.source_path,
                "role": document.role,
                "active_backend": &backend.active_backend,
                "status": &backend.status,
                "oxc_available": backend.oxc_available,
                "custom_scanner_fallback": backend.custom_scanner_fallback,
                "elements": backend.element_count,
                "expressions": backend.expression_count,
            })
        })
        .collect()
}

fn parser_backend_surfaces(documents: &[LoweredSourceDocument]) -> Vec<Value> {
    documents
        .iter()
        .map(|document| {
            let backend = &document.document.parser_backend;
            json!({
                "schema": &backend.schema,
                "schema_revision": backend.schema_revision,
                "source_path": &document.source_path,
                "role": document.role,
                "active_backend": &backend.active_backend,
                "status": &backend.status,
                "oxc_available": backend.oxc_available,
                "custom_scanner_active": backend.custom_scanner_active,
                "syntax_errors": backend.validation.syntax_errors,
                "source_type": &backend.validation.source_type,
                "typescript": backend.validation.typescript,
                "jsx": backend.validation.jsx,
                "module": backend.validation.module,
                "panicked": backend.validation.panicked,
            })
        })
        .collect()
}

struct SourceRenderDocumentSet {
    documents: Vec<LoweredSourceDocument>,
    skipped_imports: Vec<Value>,
}

pub(super) struct LoweredSourceDocument {
    pub(super) role: &'static str,
    pub(super) source_path: String,
    pub(super) source: String,
    pub(super) document: DxReactJsxDocument,
}

fn lowered_documents(
    cwd: &Path,
    route_source_path: &str,
    route_source: &str,
    segments: &[DxReactAppSegmentSource],
) -> SourceRenderDocumentSet {
    let mut documents = segments
        .iter()
        .map(|segment| LoweredSourceDocument {
            role: segment_role(segment.kind),
            source_path: segment.source_path.clone(),
            source: segment.source.clone(),
            document: lower_react_jsx_source(&segment.source_path, &segment.source),
        })
        .collect::<Vec<_>>();
    documents.push(LoweredSourceDocument {
        role: "page",
        source_path: route_source_path.to_string(),
        source: route_source.to_string(),
        document: lower_react_jsx_source(route_source_path, route_source),
    });
    let mut skipped_imports = Vec::new();
    collect_source_owned_imports(cwd, &mut documents, &mut skipped_imports);

    SourceRenderDocumentSet {
        documents,
        skipped_imports,
    }
}

fn segment_role(kind: DxReactAppSegmentKind) -> &'static str {
    match kind {
        DxReactAppSegmentKind::Layout => "layout",
        DxReactAppSegmentKind::Template => "template",
        DxReactAppSegmentKind::Loading => "loading",
        DxReactAppSegmentKind::Error => "error",
        DxReactAppSegmentKind::NotFound => "not-found",
    }
}

fn collect_source_owned_imports(
    cwd: &Path,
    documents: &mut Vec<LoweredSourceDocument>,
    skipped_imports: &mut Vec<Value>,
) {
    let mut seen = documents
        .iter()
        .map(|document| document.source_path.clone())
        .collect::<BTreeSet<_>>();
    let mut queue = (0..documents.len())
        .map(|index| (index, 0usize))
        .collect::<VecDeque<_>>();

    while let Some((index, depth)) = queue.pop_front() {
        if depth >= MAX_SOURCE_IMPORT_DEPTH || documents.len() >= MAX_SOURCE_IMPORT_DOCUMENTS {
            continue;
        }
        let source_path = documents[index].source_path.clone();
        let imports = documents[index].document.imports.clone();
        for import in imports {
            if documents.len() >= MAX_SOURCE_IMPORT_DOCUMENTS {
                break;
            }
            let Some(resolved_path) = resolve_source_owned_import(cwd, &source_path, &import)
            else {
                continue;
            };
            let project_path = project_relative_path(cwd, &resolved_path);
            if !seen.insert(project_path.clone()) {
                continue;
            }
            if !is_render_source_file(&resolved_path) {
                skipped_imports.push(json!({
                    "importer": &source_path,
                    "specifier": &import.source,
                    "resolved_path": &project_path,
                    "reason": "non-tsx-jsx-source",
                }));
                continue;
            }
            let Ok(source) = fs::read_to_string(&resolved_path) else {
                skipped_imports.push(json!({
                    "importer": &source_path,
                    "specifier": &import.source,
                    "resolved_path": &project_path,
                    "reason": "read-failed",
                }));
                continue;
            };
            documents.push(LoweredSourceDocument {
                role: "source-owned-import",
                source_path: project_path.clone(),
                source: source.clone(),
                document: lower_react_jsx_source(&project_path, &source),
            });
            queue.push_back((documents.len() - 1, depth + 1));
        }
    }
}

fn resolve_source_owned_import(
    cwd: &Path,
    importer_source_path: &str,
    import: &DxReactImport,
) -> Option<PathBuf> {
    if import.type_only || import.side_effect_only {
        return None;
    }
    let specifier = import.source.as_str();
    let unresolved = if let Some(relative) = specifier.strip_prefix("@/") {
        cwd.join(relative)
    } else if specifier.starts_with('.') {
        cwd.join(importer_source_path).parent()?.join(specifier)
    } else {
        return None;
    };
    resolve_existing_source_path(&unresolved)
}

fn resolve_existing_source_path(path: &Path) -> Option<PathBuf> {
    if path.is_file() {
        return Some(path.to_path_buf());
    }
    for extension in ["tsx", "jsx", "ts", "js"] {
        let candidate = path.with_extension(extension);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    if path.is_dir() {
        for file_name in ["index.tsx", "index.jsx", "index.ts", "index.js"] {
            let candidate = path.join(file_name);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

fn project_relative_path(cwd: &Path, path: &Path) -> String {
    let relative = path.strip_prefix(cwd).unwrap_or(path).components().fold(
        Vec::<String>::new(),
        |mut output, component| {
            match component {
                Component::CurDir => {}
                Component::ParentDir => {
                    output.pop();
                }
                Component::Normal(part) => output.push(part.to_string_lossy().to_string()),
                Component::Prefix(prefix) => {
                    output.push(prefix.as_os_str().to_string_lossy().to_string())
                }
                Component::RootDir => {}
            }
            output
        },
    );
    relative.join("/")
}

fn is_render_source_file(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| matches!(extension, "tsx" | "jsx"))
}

fn is_renderable_intrinsic(name: &str) -> bool {
    matches!(
        name,
        "a" | "article"
            | "aside"
            | "button"
            | "circle"
            | "details"
            | "div"
            | "footer"
            | "form"
            | "g"
            | "h1"
            | "h2"
            | "h3"
            | "header"
            | "body"
            | "html"
            | "img"
            | "input"
            | "label"
            | "line"
            | "li"
            | "main"
            | "nav"
            | "p"
            | "path"
            | "polygon"
            | "polyline"
            | "rect"
            | "section"
            | "select"
            | "small"
            | "span"
            | "strong"
            | "summary"
            | "svg"
            | "title"
            | "textarea"
            | "ul"
    )
}

fn is_static_renderable_element(
    document: &LoweredSourceDocument,
    element: &DxReactJsxElement,
) -> bool {
    is_renderable_intrinsic(&element.name)
        || is_safe_external_script_element(element)
        || is_next_link_element(document, element)
        || is_next_image_element(document, element)
        || is_next_script_element(document, element)
        || is_dx_icon_element(document, element)
}

fn is_safe_external_script_element(element: &DxReactJsxElement) -> bool {
    if element.name != "script" {
        return false;
    }
    if !element.child_nodes.is_empty() || !element.text_content().trim().is_empty() {
        return false;
    }
    element
        .attribute("src")
        .is_some_and(is_safe_external_script_src)
}

fn is_safe_external_script_src(src: &str) -> bool {
    let src = src.trim();
    !src.is_empty()
        && ((src.starts_with('/') && !src.starts_with("//"))
            || src.starts_with("https://")
            || src.starts_with("http://localhost:")
            || src.starts_with("http://127.0.0.1:"))
}

fn static_html_tag_name<'a>(
    document: &LoweredSourceDocument,
    element: &'a DxReactJsxElement,
) -> &'a str {
    if is_next_link_element(document, element) {
        "a"
    } else if is_next_image_element(document, element) {
        "img"
    } else if is_next_script_element(document, element) {
        "script"
    } else if is_dx_icon_element(document, element) {
        "svg"
    } else {
        element.name.as_str()
    }
}

fn framework_static_component_name(
    document: &LoweredSourceDocument,
    element: &DxReactJsxElement,
) -> Option<&'static str> {
    if is_next_link_element(document, element) {
        Some("next/link")
    } else if is_next_image_element(document, element) {
        Some("next/image")
    } else if is_next_script_element(document, element) {
        Some("next/script")
    } else if is_dx_icon_element(document, element) {
        Some("dx/icon")
    } else {
        None
    }
}

fn framework_static_render_mode(
    document: &LoweredSourceDocument,
    element: &DxReactJsxElement,
) -> Option<&'static str> {
    if is_next_link_element(document, element) {
        Some("next-link-static-anchor")
    } else if is_next_image_element(document, element) {
        Some("next-image-static-img")
    } else if is_next_script_element(document, element) {
        Some("next-script-static-script")
    } else if is_dx_icon_element(document, element) {
        Some("dx-icon-static-svg")
    } else {
        None
    }
}

fn is_dx_icon_element(document: &LoweredSourceDocument, element: &DxReactJsxElement) -> bool {
    matches!(element.name.as_str(), "icon" | "dx-icon")
        || dx_icon_component_names(document).contains(element.name.as_str())
}

fn dx_icon_component_names(document: &LoweredSourceDocument) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for import in &document.document.imports {
        if import.type_only || import.side_effect_only || !is_dx_icon_import_source(&import.source)
        {
            continue;
        }
        if let Some(default) = &import.default {
            names.insert(default.clone());
        }
        for specifier in &import.specifiers {
            if specifier.type_only {
                continue;
            }
            if is_dx_icon_imported_name(&specifier.imported)
                || is_dx_icon_imported_name(&specifier.local)
            {
                names.insert(specifier.local.clone());
            }
        }
    }
    names
}

fn is_dx_icon_import_source(source: &str) -> bool {
    let normalized = source.replace('\\', "/");
    normalized == "dx-icons"
        || normalized == "./icon"
        || normalized == "../icons/icon"
        || normalized.ends_with("/components/icons/icon")
        || normalized.ends_with("/components/icons/icon.tsx")
}

fn is_dx_icon_imported_name(name: &str) -> bool {
    name == "Icon" || name.ends_with("Icon")
}

fn is_next_link_element(document: &LoweredSourceDocument, element: &DxReactJsxElement) -> bool {
    next_link_component_names(document).contains(element.name.as_str())
}

fn next_link_component_names(document: &LoweredSourceDocument) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for import in &document.document.imports {
        if import.type_only || import.side_effect_only || import.source != "next/link" {
            continue;
        }
        if let Some(default) = &import.default {
            names.insert(default.clone());
        }
        for specifier in &import.specifiers {
            if !specifier.type_only && matches!(specifier.imported.as_str(), "default" | "Link") {
                names.insert(specifier.local.clone());
            }
        }
    }
    names
}

fn is_next_image_element(document: &LoweredSourceDocument, element: &DxReactJsxElement) -> bool {
    next_image_component_names(document).contains(element.name.as_str())
}

fn next_image_component_names(document: &LoweredSourceDocument) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for import in &document.document.imports {
        if import.type_only || import.side_effect_only || import.source != "next/image" {
            continue;
        }
        if let Some(default) = &import.default {
            names.insert(default.clone());
        }
        for specifier in &import.specifiers {
            if !specifier.type_only && matches!(specifier.imported.as_str(), "default" | "Image") {
                names.insert(specifier.local.clone());
            }
        }
    }
    names
}

fn is_next_script_element(document: &LoweredSourceDocument, element: &DxReactJsxElement) -> bool {
    next_script_component_names(document).contains(element.name.as_str())
}

fn next_script_component_names(document: &LoweredSourceDocument) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for import in &document.document.imports {
        if import.type_only || import.side_effect_only || import.source != "next/script" {
            continue;
        }
        if let Some(default) = &import.default {
            names.insert(default.clone());
        }
        for specifier in &import.specifiers {
            if !specifier.type_only && matches!(specifier.imported.as_str(), "default" | "Script") {
                names.insert(specifier.local.clone());
            }
        }
    }
    names
}

fn renderable_elements(document: &LoweredSourceDocument) -> Vec<Value> {
    document
        .document
        .elements
        .iter()
        .enumerate()
        .filter(|(_, element)| is_static_renderable_element(document, element))
        .map(|(index, element)| {
            let framework_component = framework_static_component_name(document, element);
            let render_mode =
                framework_static_render_mode(document, element).unwrap_or("intrinsic-static-dom");
            json!({
                "source_path": &document.source_path,
                "role": document.role,
                "element_index": index,
                "parent_index": element.parent_index,
                "tag": static_html_tag_name(document, element),
                "source_tag": &element.name,
                "framework_component": framework_component.unwrap_or(""),
                "render_mode": render_mode,
                "text": element.text_content(),
                "attributes": element.attributes.iter().map(attribute_surface).collect::<Vec<_>>(),
                "child_node_count": element.child_nodes.len(),
                "child_element_indices": child_element_indices(element),
                "has_events": element.attributes.iter().any(|attribute| is_event_attribute(&attribute.name)),
                "self_closing": element.self_closing,
            })
        })
        .collect()
}

fn child_element_indices(element: &DxReactJsxElement) -> Vec<usize> {
    element
        .child_nodes
        .iter()
        .filter_map(|node| match node {
            DxReactJsxChildNode::Element { index } => Some(*index),
            _ => None,
        })
        .collect()
}

fn component_references(
    document: &LoweredSourceDocument,
    component_names: &BTreeSet<&str>,
) -> Vec<Value> {
    let next_link_names = next_link_component_names(document);
    let next_image_names = next_image_component_names(document);
    let next_script_names = next_script_component_names(document);
    document
        .document
        .elements
        .iter()
        .enumerate()
        .filter(|(_, element)| is_component_reference(&element.name))
        .map(|(index, element)| {
            let framework_component = if next_link_names.contains(element.name.as_str()) {
                Some("next/link")
            } else if next_image_names.contains(element.name.as_str()) {
                Some("next/image")
            } else if next_script_names.contains(element.name.as_str()) {
                Some("next/script")
            } else {
                None
            };
            let render_mode = if next_link_names.contains(element.name.as_str()) {
                "next-link-static-anchor"
            } else if next_image_names.contains(element.name.as_str()) {
                "next-image-static-img"
            } else if next_script_names.contains(element.name.as_str()) {
                "next-script-static-script"
            } else {
                "source-owned-component"
            };
            json!({
                "source_path": &document.source_path,
                "role": document.role,
                "element_index": index,
                "parent_index": element.parent_index,
                "name": &element.name,
                "props": element.attributes.iter().map(attribute_surface).collect::<Vec<_>>(),
                "child_node_count": element.child_nodes.len(),
                "child_element_indices": child_element_indices(element),
                "resolved_in_graph": component_names.contains(element.name.as_str()) || framework_component.is_some(),
                "framework_component": framework_component.unwrap_or(""),
                "render_mode": render_mode,
                "self_closing": element.self_closing,
            })
        })
        .collect()
}

fn component_compositions(
    cwd: &Path,
    documents: &[LoweredSourceDocument],
    state_graph: &DxStateGraph,
    request_prop_bindings: &[ComponentPropBinding],
) -> Vec<Value> {
    documents
        .iter()
        .flat_map(|document| {
            document
                .document
                .elements
                .iter()
                .enumerate()
                .filter(|(_, element)| is_component_reference(&element.name))
                .filter(|(_, element)| framework_static_component_name(document, element).is_none())
                .filter_map(|(element_index, element)| {
                    source_owned_component_composition(
                        cwd,
                        documents,
                        state_graph,
                        request_prop_bindings,
                        document,
                        element_index,
                        element,
                    )
                })
        })
        .collect()
}

fn source_owned_component_composition(
    cwd: &Path,
    documents: &[LoweredSourceDocument],
    state_graph: &DxStateGraph,
    request_prop_bindings: &[ComponentPropBinding],
    importer: &LoweredSourceDocument,
    importer_element_index: usize,
    element: &DxReactJsxElement,
) -> Option<Value> {
    for import in &importer.document.imports {
        let Some(import_match) = matching_component_import(import, &element.name) else {
            continue;
        };
        let Some(resolved_path) = resolve_source_owned_import(cwd, &importer.source_path, import)
        else {
            continue;
        };
        let source_path = project_relative_path(cwd, &resolved_path);
        let imported_document = documents
            .iter()
            .find(|document| document.source_path == source_path)?;
        let snapshot = static_document_snapshot(imported_document, state_graph, &[]);
        let snapshot_html = snapshot.html;
        let snapshot_element_count = snapshot.element_count;
        let snapshot_literal_expressions = snapshot.literal_expressions;
        let snapshot_skipped_attributes = snapshot.skipped_attributes;
        let snapshot_skipped_child_expressions = snapshot.skipped_child_expressions;
        let invocation_inputs = component_invocation_inputs(element, request_prop_bindings);
        let return_preview = component_return_preview(
            imported_document,
            importer,
            element,
            state_graph,
            request_prop_bindings,
        );
        return Some(json!({
            "component": &element.name,
            "importer": &importer.source_path,
            "importer_element_index": importer_element_index,
            "parent_index": element.parent_index,
            "source_path": &imported_document.source_path,
            "import_kind": import_match.kind,
            "imported_symbol": import_match.imported,
            "mode": "source-owned-import-static-composition",
            "status": if snapshot_element_count == 0 { "empty-static-snapshot" } else { "static-composition-ready" },
            "full_component_execution": false,
            "invocation_inputs": invocation_inputs,
            "return_preview": return_preview,
            "static_snapshot": {
                "html": snapshot_html,
                "element_count": snapshot_element_count,
                "literal_expressions": snapshot_literal_expressions,
                "skipped_attributes": snapshot_skipped_attributes,
                "skipped_child_expressions": snapshot_skipped_child_expressions,
            },
            "limits": [
                "Matches component JSX usages to bounded source-owned local imports.",
                "Preserves literal usage props and direct literal children as renderer inputs.",
                "Previews static return JSX with direct children placeholder insertion when possible.",
                "Serializes the imported component file's safe intrinsic JSX skeleton.",
                "Does not execute the component function body, hooks, props logic, effects, or runtime data reads."
            ],
        }));
    }
    None
}

struct ComponentImportMatch {
    kind: &'static str,
    imported: String,
}

fn matching_component_import(
    import: &DxReactImport,
    component_name: &str,
) -> Option<ComponentImportMatch> {
    if import.default.as_deref() == Some(component_name) {
        return Some(ComponentImportMatch {
            kind: "default",
            imported: "default".to_string(),
        });
    }
    import
        .specifiers
        .iter()
        .find(|specifier| !specifier.type_only && specifier.local == component_name)
        .map(|specifier| ComponentImportMatch {
            kind: "named",
            imported: specifier.imported.clone(),
        })
}

fn component_invocation_inputs(
    element: &DxReactJsxElement,
    request_prop_bindings: &[ComponentPropBinding],
) -> Value {
    let props = component_invocation_props(element, request_prop_bindings);
    let children = component_invocation_children(element);
    let prop_values = props.values;
    let literal_props = props.literal_props;
    let dynamic_props = props.dynamic_props;
    let event_handler_props = props.event_handler_props;
    let spread_props = props.spread_props;
    let child_values = children.values;
    let text_children = children.text_children;
    let literal_child_expressions = children.literal_child_expressions;
    let skipped_child_expressions = children.skipped_child_expressions;
    json!({
        "schema": "dx.tsx.componentInvocationInputs",
        "schema_revision": 1,
        "contract_name": "Component Invocation Inputs",
        "component": &element.name,
        "self_closing": element.self_closing,
        "props": prop_values,
        "children": child_values,
        "counts": {
            "literal_props": literal_props,
            "dynamic_props": dynamic_props,
            "event_handler_props": event_handler_props,
            "spread_props": spread_props,
            "text_children": text_children,
            "literal_child_expressions": literal_child_expressions,
            "skipped_child_expressions": skipped_child_expressions,
        },
        "lowering": {
            "mode": "static-invocation-inputs",
            "children_order_exact": false,
            "full_prop_evaluation": false,
            "event_handlers_attached": false,
        }
    })
}

struct ComponentInvocationProps {
    values: Vec<Value>,
    literal_props: usize,
    dynamic_props: usize,
    event_handler_props: usize,
    spread_props: usize,
}

fn component_invocation_props(
    element: &DxReactJsxElement,
    request_prop_bindings: &[ComponentPropBinding],
) -> ComponentInvocationProps {
    let mut props = Vec::new();
    let mut literal_props = 0usize;
    let mut dynamic_props = 0usize;
    let mut event_handler_props = 0usize;
    let mut spread_props = 0usize;

    for attribute in &element.attributes {
        let prop = component_invocation_prop(attribute, request_prop_bindings);
        match prop["kind"].as_str().unwrap_or("unknown") {
            "literal" | "literal-expression" | "boolean-bare" | "next-app-router-page-prop" => {
                literal_props += 1
            }
            "event-handler" => event_handler_props += 1,
            "spread" => spread_props += 1,
            _ => dynamic_props += 1,
        }
        props.push(prop);
    }

    ComponentInvocationProps {
        values: props,
        literal_props,
        dynamic_props,
        event_handler_props,
        spread_props,
    }
}

fn component_invocation_prop(
    attribute: &DxReactJsxAttribute,
    request_prop_bindings: &[ComponentPropBinding],
) -> Value {
    if is_jsx_spread_attribute(&attribute.name) {
        return json!({
            "name": &attribute.name,
            "kind": "spread",
            "static_value": null,
            "expression": &attribute.expression,
            "executable_in_static_renderer": false,
        });
    }
    if is_event_attribute(&attribute.name) {
        return json!({
            "name": &attribute.name,
            "kind": "event-handler",
            "static_value": null,
            "expression": attribute.expression.as_deref().or(attribute.value.as_deref()),
            "executable_in_static_renderer": false,
        });
    }
    if let Some(value) = attribute.value.as_deref() {
        return json!({
            "name": &attribute.name,
            "kind": "literal",
            "value_type": "string",
            "static_value": value,
            "expression": null,
            "executable_in_static_renderer": true,
        });
    }
    if let Some(expression) = attribute.expression.as_deref() {
        if let Some(value) = static_literal_expression(expression) {
            return json!({
                "name": &attribute.name,
                "kind": "literal-expression",
                "value_type": value.value_type(),
                "static_value": value.to_json_value(),
                "expression": expression,
                "executable_in_static_renderer": true,
            });
        }
        if let Some(value) =
            resolve_next_app_router_page_prop_identifier(expression, request_prop_bindings)
        {
            return json!({
                "name": &attribute.name,
                "kind": "next-app-router-page-prop",
                "value_type": value.value_type(),
                "static_value": value.to_json_value(),
                "expression": expression,
                "executable_in_static_renderer": true,
            });
        }
        return json!({
            "name": &attribute.name,
            "kind": "dynamic-expression",
            "static_value": null,
            "expression": expression,
            "executable_in_static_renderer": false,
        });
    }
    json!({
        "name": &attribute.name,
        "kind": "boolean-bare",
        "value_type": "boolean",
        "static_value": true,
        "expression": null,
        "executable_in_static_renderer": true,
    })
}

struct ComponentInvocationChildren {
    values: Value,
    text_children: usize,
    literal_child_expressions: usize,
    skipped_child_expressions: usize,
}

fn component_invocation_children(element: &DxReactJsxElement) -> ComponentInvocationChildren {
    let literal_expressions = element
        .child_expressions
        .iter()
        .filter_map(|expression| {
            static_literal_expression(expression).map(|value| {
                json!({
                    "expression": expression,
                    "value_type": value.value_type(),
                    "static_value": value.to_json_value(),
                })
            })
        })
        .collect::<Vec<_>>();
    let skipped_expressions = element
        .child_expressions
        .iter()
        .filter(|expression| static_literal_expression(expression).is_none())
        .cloned()
        .collect::<Vec<_>>();
    let literal_child_expression_count = literal_expressions.len();
    let skipped_child_expression_count = skipped_expressions.len();

    ComponentInvocationChildren {
        values: json!({
            "mode": "direct-literal-children",
            "text": element.text_content(),
            "text_segments": &element.child_text,
            "literal_expressions": literal_expressions,
            "skipped_expressions": skipped_expressions,
            "order_exact": false,
        }),
        text_children: element.child_text.len(),
        literal_child_expressions: literal_child_expression_count,
        skipped_child_expressions: skipped_child_expression_count,
    }
}

fn is_jsx_spread_attribute(name: &str) -> bool {
    name.starts_with("{...") || name.starts_with("...")
}

fn component_return_preview(
    imported_document: &LoweredSourceDocument,
    invocation_document: &LoweredSourceDocument,
    invocation_element: &DxReactJsxElement,
    state_graph: &DxStateGraph,
    request_prop_bindings: &[ComponentPropBinding],
) -> Value {
    let invocation_children = component_invocation_child_html(
        invocation_document,
        invocation_element,
        state_graph,
        request_prop_bindings,
    );
    let prop_binding_context =
        component_prop_binding_context(invocation_element, request_prop_bindings);
    let destructured_prop_aliases = component_destructured_prop_aliases(imported_document);
    let preview = static_document_preview_with_children(
        imported_document,
        &invocation_children.html,
        &prop_binding_context.bindings,
        &destructured_prop_aliases,
    );
    let runtime_binding = component_runtime_binding_plan(imported_document, state_graph);
    let invocation_children_available = !invocation_children.html.is_empty();
    let invocation_children_html = invocation_children.html;
    let prop_identifier_bindings = component_prop_identifier_bindings(
        &prop_binding_context,
        preview.prop_identifier_bindings,
        &destructured_prop_aliases,
    );
    let preview_html = preview.html;
    let preview_elements = preview.elements;
    json!({
        "schema": "dx.tsx.componentReturnPreview",
        "schema_revision": 1,
        "contract_name": "Component Return Preview",
        "mode": "static-return-preview-with-children",
        "status": if preview.element_count == 0 {
            "empty-static-return-preview"
        } else if preview.children_placeholder_count > 0 {
            "static-return-preview-with-children-placeholders"
        } else {
            "static-return-preview-ready"
        },
        "full_component_execution": false,
        "children_insertion": {
            "available": invocation_children_available,
            "html": invocation_children_html,
            "text_children": invocation_children.text_children,
            "literal_child_expressions": invocation_children.literal_child_expressions,
            "skipped_child_expressions": invocation_children.skipped_child_expressions,
        },
        "prop_identifier_bindings": prop_identifier_bindings,
        "destructured_prop_aliases": destructured_prop_aliases.to_json(),
        "html": preview_html,
        "element_count": preview.element_count,
        "children_placeholder_count": preview.children_placeholder_count,
        "children_insertions": preview.children_insertions,
        "prop_identifier_binding_count": preview.prop_identifier_bindings,
        "literal_expressions": preview.literal_expressions,
        "skipped_attributes": preview.skipped_attributes,
        "skipped_child_expressions": preview.skipped_child_expressions,
        "runtime_binding": runtime_binding,
        "elements": preview_elements,
        "limits": [
            "Substitutes direct children placeholders such as {children} and {props.children}.",
            "Binds simple prop identifiers such as {title} and {props.title} from literal caller props.",
            "Recognizes simple destructured prop aliases from function and arrow component parameters.",
            "Uses only direct text and literal expression children from the component usage.",
            "Does not preserve nested child JSX order until the JSX graph has parent-child edges.",
            "Does not execute component functions, resolve rest/spread props, run hooks, or attach events."
        ],
    })
}

#[derive(Clone)]
pub(super) struct ComponentPropBinding {
    pub(super) name: String,
    pub(super) value: StaticLiteralExpression,
    pub(super) source_kind: &'static str,
    pub(super) expression: Option<String>,
}

struct ComponentPropBindingContext {
    bindings: Vec<ComponentPropBinding>,
    skipped_dynamic_props: usize,
}

fn component_prop_binding_context(
    element: &DxReactJsxElement,
    request_prop_bindings: &[ComponentPropBinding],
) -> ComponentPropBindingContext {
    let mut bindings = Vec::new();
    let mut skipped_dynamic_props = 0usize;

    for attribute in &element.attributes {
        if is_event_attribute(&attribute.name)
            || is_jsx_spread_attribute(&attribute.name)
            || !is_simple_prop_identifier(&attribute.name)
        {
            skipped_dynamic_props += 1;
            continue;
        }
        if let Some(value) = attribute.value.as_deref() {
            bindings.push(ComponentPropBinding {
                name: attribute.name.clone(),
                value: StaticLiteralExpression::String(value.to_string()),
                source_kind: "literal",
                expression: None,
            });
            continue;
        }
        if let Some(expression) = attribute.expression.as_deref() {
            if let Some(value) = static_literal_expression(expression) {
                bindings.push(ComponentPropBinding {
                    name: attribute.name.clone(),
                    value,
                    source_kind: "literal-expression",
                    expression: Some(expression.to_string()),
                });
            } else if let Some(value) =
                resolve_next_app_router_page_prop_identifier(expression, request_prop_bindings)
            {
                bindings.push(ComponentPropBinding {
                    name: attribute.name.clone(),
                    value,
                    source_kind: "next-app-router-page-prop",
                    expression: Some(expression.to_string()),
                });
            } else {
                skipped_dynamic_props += 1;
            }
            continue;
        }
        bindings.push(ComponentPropBinding {
            name: attribute.name.clone(),
            value: StaticLiteralExpression::Boolean(true),
            source_kind: "boolean-bare",
            expression: None,
        });
    }

    ComponentPropBindingContext {
        bindings,
        skipped_dynamic_props,
    }
}

fn component_prop_identifier_bindings(
    context: &ComponentPropBindingContext,
    resolved_count: usize,
    destructured_prop_aliases: &ComponentDestructuredPropAliases,
) -> Value {
    json!({
        "schema": "dx.tsx.componentPropIdentifierBindings",
        "schema_revision": 1,
        "contract_name": "Component Prop Identifier Bindings",
        "status": if context.bindings.is_empty() {
            "no-static-caller-props"
        } else if resolved_count == 0 {
            "static-caller-props-without-simple-identifier-reads"
        } else {
            "simple-prop-identifier-bindings"
        },
        "mode": "simple-prop-identifier-bindings",
        "full_prop_evaluation": false,
        "resolved_count": resolved_count,
        "available_prop_count": context.bindings.len(),
        "skipped_dynamic_props": context.skipped_dynamic_props,
        "destructured_prop_alias_count": destructured_prop_aliases.aliases.len(),
        "destructured_prop_aliases": destructured_prop_aliases.to_json(),
        "template_literal_prop_bindings": true,
        "conditional_expression_prop_bindings": true,
        "class_list_prop_bindings": true,
        "class_call_prop_bindings": true,
        "supported_expressions": [
            "propName",
            "props.propName",
            "static-template-prop-interpolation",
            "static-ternary-prop-branches",
            "static-class-list-prop-bindings",
            "static-class-call-prop-bindings",
            "`${propName}`",
            "`${props.propName}`",
            "`${propName}`.trim()",
            "propName ? literalA : literalB",
            "propName === literal ? literalA : literalB",
            "[\"base\", active && \"active\"].filter(Boolean).join(\" \")",
            "cn(\"base\", active && \"active\")",
            "clsx(\"base\", active && \"active\")"
        ],
        "available_props": context.bindings.iter().map(|binding| json!({
            "name": &binding.name,
            "source_kind": binding.source_kind,
            "expression": &binding.expression,
            "value_type": binding.value.value_type(),
            "static_value": binding.value.to_json_value(),
        })).collect::<Vec<_>>(),
        "limits": [
            "Uses only literal caller props and literal expression props from the component invocation.",
            "Resolves direct identifier reads and props.identifier reads inside safe intrinsic return previews.",
            "Direct identifier reads are bounded by simple destructured prop aliases when the component signature exposes them.",
            "Resolves bounded static template literals when every interpolation is a literal or a resolved prop binding.",
            "Resolves bounded class-list arrays only when every item is a literal, resolved prop binding, template binding, or static condition.",
            "Resolves bounded cn/clsx class calls only when every argument is a literal, resolved prop binding, template binding, or static condition.",
            "Does not evaluate spreads, rest props, computed member expressions, function calls, hooks, or arbitrary JavaScript."
        ],
    })
}

struct ComponentDestructuredPropAliases {
    aliases: Vec<ComponentPropAlias>,
    signature_patterns: Vec<&'static str>,
    skipped_patterns: usize,
}

impl ComponentDestructuredPropAliases {
    fn to_json(&self) -> Value {
        json!({
            "schema": "dx.tsx.componentDestructuredPropAliases",
            "schema_revision": 1,
            "contract_name": "Component Destructured Prop Aliases",
            "mode": "safe-destructured-prop-aliases",
            "status": if self.aliases.is_empty() {
                "no-simple-destructured-prop-aliases"
            } else {
                "simple-destructured-prop-aliases"
            },
            "alias_count": self.aliases.len(),
            "aliases": self.aliases.iter().map(ComponentPropAlias::to_json).collect::<Vec<_>>(),
            "signature_patterns": &self.signature_patterns,
            "supported_signatures": [
                "function-parameter-object-pattern",
                "arrow-parameter-object-pattern",
                "renamed-alias-object-pattern",
                "default-value-object-pattern",
            ],
            "skipped_patterns": self.skipped_patterns,
            "full_parameter_evaluation": false,
            "limits": [
                "Recognizes simple object-pattern component parameters only.",
                "Supports function components and arrow function components with direct destructured props.",
                "Supports simple renamed aliases and static literal default values.",
                "Skips rest props, nested patterns, computed keys, non-literal defaults, and arbitrary TypeScript/JavaScript parameter logic."
            ],
        })
    }
}

#[derive(Clone)]
struct ComponentPropAlias {
    prop_name: String,
    alias: String,
    default_value: Option<StaticLiteralExpression>,
    pattern_kind: &'static str,
}

impl ComponentPropAlias {
    fn to_json(&self) -> Value {
        json!({
            "prop_name": &self.prop_name,
            "alias": &self.alias,
            "pattern_kind": self.pattern_kind,
            "default_value": self.default_value.as_ref().map(StaticLiteralExpression::to_json_value),
            "default_value_type": self.default_value.as_ref().map(StaticLiteralExpression::value_type),
        })
    }
}

fn component_destructured_prop_aliases(
    document: &LoweredSourceDocument,
) -> ComponentDestructuredPropAliases {
    let mut aliases = Vec::new();
    let mut signature_patterns = Vec::new();
    let mut skipped_patterns = 0usize;

    collect_function_parameter_object_pattern_aliases(
        &document.source,
        &mut aliases,
        &mut signature_patterns,
        &mut skipped_patterns,
    );
    collect_arrow_parameter_object_pattern_aliases(
        &document.source,
        &mut aliases,
        &mut signature_patterns,
        &mut skipped_patterns,
    );

    ComponentDestructuredPropAliases {
        aliases,
        signature_patterns,
        skipped_patterns,
    }
}

fn empty_component_prop_aliases() -> ComponentDestructuredPropAliases {
    ComponentDestructuredPropAliases {
        aliases: Vec::new(),
        signature_patterns: Vec::new(),
        skipped_patterns: 0,
    }
}

fn collect_function_parameter_object_pattern_aliases(
    source: &str,
    aliases: &mut Vec<ComponentPropAlias>,
    signature_patterns: &mut Vec<&'static str>,
    skipped_patterns: &mut usize,
) {
    for (index, _) in source.match_indices("function") {
        if !is_word_boundary(source, index, "function".len()) {
            continue;
        }
        let after_function = index + "function".len();
        let Some(open_paren_offset) = source[after_function..].find('(') else {
            continue;
        };
        let open_paren = after_function + open_paren_offset;
        let open_brace = skip_ascii_whitespace(source, open_paren + 1);
        if !source[open_brace..].starts_with('{') {
            continue;
        }
        let Some(close_brace) = find_matching_delimiter(source, open_brace, '{', '}') else {
            *skipped_patterns += 1;
            continue;
        };
        if end_of_object_pattern_parameter(source, close_brace).is_none() {
            *skipped_patterns += 1;
            continue;
        }
        push_unique_static(signature_patterns, "function-parameter-object-pattern");
        collect_object_pattern_aliases(
            &source[open_brace + 1..close_brace],
            aliases,
            signature_patterns,
            skipped_patterns,
        );
    }
}

fn collect_arrow_parameter_object_pattern_aliases(
    source: &str,
    aliases: &mut Vec<ComponentPropAlias>,
    signature_patterns: &mut Vec<&'static str>,
    skipped_patterns: &mut usize,
) {
    for (open_paren, _) in source.match_indices("({") {
        let open_brace = open_paren + 1;
        let Some(close_brace) = find_matching_delimiter(source, open_brace, '{', '}') else {
            *skipped_patterns += 1;
            continue;
        };
        let Some(parameter_end) = end_of_object_pattern_parameter(source, close_brace) else {
            *skipped_patterns += 1;
            continue;
        };
        let arrow = skip_ascii_whitespace(source, parameter_end);
        if !source[arrow..].starts_with("=>") {
            continue;
        }
        push_unique_static(signature_patterns, "arrow-parameter-object-pattern");
        collect_object_pattern_aliases(
            &source[open_brace + 1..close_brace],
            aliases,
            signature_patterns,
            skipped_patterns,
        );
    }
}

fn collect_object_pattern_aliases(
    object_pattern: &str,
    aliases: &mut Vec<ComponentPropAlias>,
    signature_patterns: &mut Vec<&'static str>,
    skipped_patterns: &mut usize,
) {
    for raw_part in object_pattern.split(',') {
        let part = raw_part.trim();
        if part.is_empty() {
            continue;
        }
        if part.starts_with("...") || part.contains('{') || part.contains('[') {
            *skipped_patterns += 1;
            continue;
        }
        if let Some(alias) = component_prop_alias_from_object_pattern_part(part) {
            push_unique_static(signature_patterns, alias.pattern_kind);
            push_unique_component_prop_alias(aliases, alias);
        } else {
            *skipped_patterns += 1;
        }
    }
}

fn component_prop_alias_from_object_pattern_part(part: &str) -> Option<ComponentPropAlias> {
    let (left, right, renamed) = part
        .split_once(':')
        .map(|(left, right)| (left.trim(), right.trim(), true))
        .unwrap_or((part.trim(), part.trim(), false));
    if !is_simple_prop_identifier(left) {
        return None;
    }

    let (alias_source, default_value) =
        if let Some((alias, default_expression)) = right.split_once('=') {
            (
                alias.trim(),
                Some(prop_alias_default_value(default_expression.trim())?),
            )
        } else {
            (right, None)
        };
    if !is_simple_prop_identifier(alias_source) {
        return None;
    }

    let pattern_kind = match (renamed, default_value.is_some()) {
        (true, _) => "renamed-alias-object-pattern",
        (false, true) => "default-value-object-pattern",
        (false, false) => "simple-alias-object-pattern",
    };

    Some(ComponentPropAlias {
        prop_name: left.to_string(),
        alias: alias_source.to_string(),
        default_value,
        pattern_kind,
    })
}

fn prop_alias_default_value(expression: &str) -> Option<StaticLiteralExpression> {
    static_literal_expression(expression)
}

fn end_of_object_pattern_parameter(source: &str, close_brace: usize) -> Option<usize> {
    let mut cursor = skip_ascii_whitespace(source, close_brace + 1);
    if source[cursor..].starts_with(':') {
        let close_paren_offset = source[cursor..].find(')')?;
        cursor += close_paren_offset;
    }
    cursor = skip_ascii_whitespace(source, cursor);
    if source[cursor..].starts_with(')') {
        Some(cursor + 1)
    } else {
        None
    }
}

fn find_matching_delimiter(
    source: &str,
    open_index: usize,
    open: char,
    close: char,
) -> Option<usize> {
    let mut depth = 0isize;
    let mut quote: Option<char> = None;
    let mut escaped = false;
    for (offset, character) in source[open_index..].char_indices() {
        let index = open_index + offset;
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
        if matches!(character, '"' | '\'' | '`') {
            quote = Some(character);
            continue;
        }
        if character == open {
            depth += 1;
            continue;
        }
        if character == close {
            depth -= 1;
            if depth == 0 {
                return Some(index);
            }
        }
    }
    None
}

fn skip_ascii_whitespace(source: &str, mut index: usize) -> usize {
    while index < source.len() && source.as_bytes()[index].is_ascii_whitespace() {
        index += 1;
    }
    index
}

fn is_word_boundary(source: &str, index: usize, len: usize) -> bool {
    let before = if index == 0 {
        None
    } else {
        source[..index].chars().next_back()
    };
    let after = source[index + len..].chars().next();
    before.is_none_or(|character| !is_identifier_character(character))
        && after.is_none_or(|character| !is_identifier_character(character))
}

fn is_identifier_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_' || character == '$'
}

fn push_unique_static(values: &mut Vec<&'static str>, value: &'static str) {
    if !values.contains(&value) {
        values.push(value);
    }
}

fn push_unique_component_prop_alias(
    aliases: &mut Vec<ComponentPropAlias>,
    alias: ComponentPropAlias,
) {
    if let Some(existing) = aliases
        .iter_mut()
        .find(|existing| existing.alias == alias.alias)
    {
        *existing = alias;
    } else {
        aliases.push(alias);
    }
}

include!("source_render_parts/client_component.rs");
include!("source_render_parts/static_markup.rs");
fn push_unique(values: &mut Vec<String>, value: &str) {
    if !values.iter().any(|existing| existing == value) {
        values.push(value.to_string());
    }
}

fn expression_refs_state_slot(expression: &str, slot_name: &str) -> bool {
    strip_static_parentheses(expression.trim()) == slot_name
}

include!("source_render_parts/static_expression.rs");
fn static_html_attribute_name(name: &str) -> Option<&str> {
    match name {
        "className" => Some("class"),
        "htmlFor" => Some("for"),
        "inputMode" => Some("inputmode"),
        "autoComplete" => Some("autocomplete"),
        "autoCapitalize" => Some("autocapitalize"),
        "enterKeyHint" => Some("enterkeyhint"),
        "spellCheck" => Some("spellcheck"),
        "tabIndex" => Some("tabindex"),
        "contentEditable" => Some("contenteditable"),
        "playsInline" => Some("playsinline"),
        "allowFullScreen" => Some("allowfullscreen"),
        "readOnly" => Some("readonly"),
        "srcSet" => Some("srcset"),
        "fetchPriority" => Some("fetchpriority"),
        "crossOrigin" => Some("crossorigin"),
        "referrerPolicy" => Some("referrerpolicy"),
        "noModule" => Some("nomodule"),
        "strokeWidth" => Some("stroke-width"),
        "strokeLinecap" => Some("stroke-linecap"),
        "strokeLinejoin" => Some("stroke-linejoin"),
        "class" | "id" | "href" | "type" | "name" | "value" | "placeholder" | "title" | "role"
        | "alt" | "src" | "sizes" | "width" | "height" | "loading" | "decoding" | "target"
        | "rel" | "method" | "action" | "inputmode" | "autocomplete" | "autocapitalize"
        | "enterkeyhint" | "spellcheck" | "tabindex" | "contenteditable" | "draggable" | "rows"
        | "cols" | "async" | "defer" | "nonce" | "integrity" | "crossorigin" | "referrerpolicy"
        | "nomodule" | "required" | "disabled" | "checked" | "selected" | "multiple"
        | "readonly" | "hidden" | "playsinline" | "allowfullscreen" | "viewBox" | "fill"
        | "stroke" | "stroke-width" | "stroke-linecap" | "stroke-linejoin" | "d" | "x" | "y"
        | "x1" | "y1" | "x2" | "y2" | "cx" | "cy" | "r" | "rx" | "ry" | "points" => Some(name),
        _ if name.starts_with("data-") || name.starts_with("aria-") => Some(name),
        _ => None,
    }
}

fn is_boolean_html_attribute(name: &str) -> bool {
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

fn is_enumerated_boolean_html_attribute(name: &str) -> bool {
    matches!(name, "spellcheck" | "contenteditable" | "draggable")
}

fn is_void_element(name: &str) -> bool {
    matches!(name, "img" | "input")
}

fn escape_html_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_html_attr(value: &str) -> String {
    escape_html_text(value).replace('"', "&quot;")
}

fn attribute_surface(attribute: &DxReactJsxAttribute) -> Value {
    json!({
        "name": &attribute.name,
        "kind": attribute_kind(attribute),
        "value": &attribute.value,
        "expression": &attribute.expression,
        "event": is_event_attribute(&attribute.name),
    })
}

fn attribute_kind(attribute: &DxReactJsxAttribute) -> &'static str {
    if attribute.expression.is_some() {
        "expression"
    } else if attribute.value.is_some() {
        "literal"
    } else {
        "boolean"
    }
}

fn is_component_reference(name: &str) -> bool {
    name.chars()
        .next()
        .is_some_and(|character| character.is_ascii_uppercase())
}

fn is_event_attribute(name: &str) -> bool {
    let Some(rest) = name.strip_prefix("on") else {
        return false;
    };
    rest.chars()
        .next()
        .is_some_and(|character| character.is_ascii_uppercase())
}

#[cfg(test)]
mod tests {
    use super::super::request_props::next_app_router_page_prop_binding_name;
    use super::*;

    fn string_prop(name: &str, value: &str) -> ComponentPropBinding {
        ComponentPropBinding {
            name: name.to_string(),
            value: StaticLiteralExpression::String(value.to_string()),
            source_kind: "literal",
            expression: None,
        }
    }

    fn empty_aliases() -> ComponentDestructuredPropAliases {
        ComponentDestructuredPropAliases {
            aliases: Vec::new(),
            signature_patterns: Vec::new(),
            skipped_patterns: 0,
        }
    }

    #[test]
    fn not_found_boundary_replaces_page_leaf_in_app_router_shell() {
        let not_found_source = r#"export default function NotFound() {
  return <section><h1>Missing route</h1></section>;
}
"#;
        let page_source = r#"import { notFound } from "next/navigation";

export default function Page() {
  notFound();
  return <section>Secret page</section>;
}
"#;
        let documents = vec![
            LoweredSourceDocument {
                role: "not-found",
                source_path: "app/blog/not-found.tsx".to_string(),
                source: not_found_source.to_string(),
                document: lower_react_jsx_source("app/blog/not-found.tsx", not_found_source),
            },
            LoweredSourceDocument {
                role: "page",
                source_path: "app/blog/[slug]/page.tsx".to_string(),
                source: page_source.to_string(),
                document: lower_react_jsx_source("app/blog/[slug]/page.tsx", page_source),
            },
        ];

        let shell = compose_app_router_static_shell(&documents, &[], &DxStateGraph::default(), &[]);
        let shell_json = shell.to_json();

        assert_eq!(shell.status, "not-found-boundary-ready");
        assert_eq!(shell.leaf_role, "not-found");
        assert!(shell.not_found_boundary_selected);
        assert!(shell.html.contains("Missing route"));
        assert!(!shell.html.contains("Secret page"));
        assert_eq!(shell_json["selected_leaf"]["role"], "not-found");
        assert_eq!(
            shell_json["selected_leaf"]["not_found_boundary_selected"],
            true
        );
    }

    #[test]
    fn not_found_boundary_uses_nearest_route_scoped_boundary() {
        let root_not_found_source = r#"export default function NotFound() {
  return <section><h1>Root missing</h1></section>;
}
"#;
        let blog_not_found_source = r#"export default function NotFound() {
  return <section><h1>Blog missing</h1></section>;
}
"#;
        let admin_not_found_source = r#"export default function NotFound() {
  return <section><h1>Admin missing</h1></section>;
}
"#;
        let page_source = r#"import { notFound } from "next/navigation";

export default function Page() {
  notFound();
  return <section>Private blog page</section>;
}
"#;
        let documents = vec![
            LoweredSourceDocument {
                role: "not-found",
                source_path: "app/not-found.tsx".to_string(),
                source: root_not_found_source.to_string(),
                document: lower_react_jsx_source("app/not-found.tsx", root_not_found_source),
            },
            LoweredSourceDocument {
                role: "not-found",
                source_path: "app/blog/not-found.tsx".to_string(),
                source: blog_not_found_source.to_string(),
                document: lower_react_jsx_source("app/blog/not-found.tsx", blog_not_found_source),
            },
            LoweredSourceDocument {
                role: "not-found",
                source_path: "app/admin/not-found.tsx".to_string(),
                source: admin_not_found_source.to_string(),
                document: lower_react_jsx_source("app/admin/not-found.tsx", admin_not_found_source),
            },
            LoweredSourceDocument {
                role: "page",
                source_path: "app/blog/[slug]/page.tsx".to_string(),
                source: page_source.to_string(),
                document: lower_react_jsx_source("app/blog/[slug]/page.tsx", page_source),
            },
        ];

        let shell = compose_app_router_static_shell(&documents, &[], &DxStateGraph::default(), &[]);
        let shell_json = shell.to_json();

        assert_eq!(shell.status, "not-found-boundary-ready");
        assert_eq!(shell.leaf_role, "not-found");
        assert_eq!(
            shell.leaf_source_path.as_deref(),
            Some("app/blog/not-found.tsx")
        );
        assert!(shell.not_found_boundary_selected);
        assert!(shell.html.contains("Blog missing"));
        assert!(!shell.html.contains("Root missing"));
        assert!(!shell.html.contains("Admin missing"));
        assert!(!shell.html.contains("Private blog page"));
        assert_eq!(
            shell_json["selected_leaf"]["source_path"],
            "app/blog/not-found.tsx"
        );
    }

    #[test]
    fn not_found_boundary_does_not_use_child_layouts_below_selected_boundary() {
        let root_layout_source = r#"export default function Layout({ children }) {
  return <main><h1>Root shell</h1>{children}</main>;
}
"#;
        let blog_not_found_source = r#"export default function NotFound() {
  return <section><h2>Missing post</h2></section>;
}
"#;
        let child_layout_source = r#"export default function Layout({ children }) {
  return <aside><h3>Child slug shell</h3>{children}</aside>;
}
"#;
        let page_source = r#"import { notFound } from "next/navigation";

export default function Page() {
  notFound();
  return <section>Secret post</section>;
}
"#;
        let documents = vec![
            LoweredSourceDocument {
                role: "layout",
                source_path: "app/layout.tsx".to_string(),
                source: root_layout_source.to_string(),
                document: lower_react_jsx_source("app/layout.tsx", root_layout_source),
            },
            LoweredSourceDocument {
                role: "not-found",
                source_path: "app/blog/not-found.tsx".to_string(),
                source: blog_not_found_source.to_string(),
                document: lower_react_jsx_source("app/blog/not-found.tsx", blog_not_found_source),
            },
            LoweredSourceDocument {
                role: "layout",
                source_path: "app/blog/[slug]/layout.tsx".to_string(),
                source: child_layout_source.to_string(),
                document: lower_react_jsx_source("app/blog/[slug]/layout.tsx", child_layout_source),
            },
            LoweredSourceDocument {
                role: "page",
                source_path: "app/blog/[slug]/page.tsx".to_string(),
                source: page_source.to_string(),
                document: lower_react_jsx_source("app/blog/[slug]/page.tsx", page_source),
            },
        ];

        let shell = compose_app_router_static_shell(&documents, &[], &DxStateGraph::default(), &[]);

        assert_eq!(shell.status, "not-found-boundary-ready");
        assert_eq!(shell.leaf_role, "not-found");
        assert!(shell.html.contains("Root shell"));
        assert!(shell.html.contains("Missing post"));
        assert!(!shell.html.contains("Child slug shell"));
        assert!(!shell.html.contains("Secret post"));
    }

    #[test]
    fn not_found_boundary_selects_aliased_not_found_import() {
        let not_found_source = r#"export default function NotFound() {
  return <section><h1>Missing account</h1></section>;
}
"#;
        let page_source = r#"import { notFound as missing } from "next/navigation";

export default function Page() {
  missing();
  return <section>Private account</section>;
}
"#;
        let documents = vec![
            LoweredSourceDocument {
                role: "not-found",
                source_path: "app/account/not-found.tsx".to_string(),
                source: not_found_source.to_string(),
                document: lower_react_jsx_source("app/account/not-found.tsx", not_found_source),
            },
            LoweredSourceDocument {
                role: "page",
                source_path: "app/account/page.tsx".to_string(),
                source: page_source.to_string(),
                document: lower_react_jsx_source("app/account/page.tsx", page_source),
            },
        ];

        let shell = compose_app_router_static_shell(&documents, &[], &DxStateGraph::default(), &[]);
        let shell_json = shell.to_json();

        assert_eq!(shell.status, "not-found-boundary-ready");
        assert_eq!(shell.leaf_role, "not-found");
        assert!(shell.not_found_boundary_selected);
        assert!(shell.html.contains("Missing account"));
        assert!(!shell.html.contains("Private account"));
        assert_eq!(shell_json["selected_leaf"]["role"], "not-found");
        assert_eq!(
            shell_json["selected_leaf"]["not_found_boundary_selected"],
            true
        );
    }

    #[test]
    fn not_found_boundary_selects_namespace_not_found_import() {
        let not_found_source = r#"export default function NotFound() {
  return <section><h1>Missing settings</h1></section>;
}
"#;
        let page_source = r#"import * as navigation from "next/navigation";

export default function Page() {
  navigation.notFound();
  return <section>Private settings</section>;
}
"#;
        let documents = vec![
            LoweredSourceDocument {
                role: "not-found",
                source_path: "app/settings/not-found.tsx".to_string(),
                source: not_found_source.to_string(),
                document: lower_react_jsx_source("app/settings/not-found.tsx", not_found_source),
            },
            LoweredSourceDocument {
                role: "page",
                source_path: "app/settings/page.tsx".to_string(),
                source: page_source.to_string(),
                document: lower_react_jsx_source("app/settings/page.tsx", page_source),
            },
        ];

        let shell = compose_app_router_static_shell(&documents, &[], &DxStateGraph::default(), &[]);
        let shell_json = shell.to_json();

        assert_eq!(shell.status, "not-found-boundary-ready");
        assert_eq!(shell.leaf_role, "not-found");
        assert!(shell.not_found_boundary_selected);
        assert!(shell.html.contains("Missing settings"));
        assert!(!shell.html.contains("Private settings"));
        assert_eq!(shell_json["selected_leaf"]["role"], "not-found");
        assert_eq!(
            shell_json["selected_leaf"]["not_found_boundary_selected"],
            true
        );
    }

    #[test]
    fn redirect_control_flow_replaces_page_leaf_with_static_marker() {
        let page_source = r#"import { redirect } from "next/navigation";

export default function Page() {
  redirect("/login", "push");
  return <section>Secret page</section>;
}
"#;
        let documents = vec![LoweredSourceDocument {
            role: "page",
            source_path: "app/account/page.tsx".to_string(),
            source: page_source.to_string(),
            document: lower_react_jsx_source("app/account/page.tsx", page_source),
        }];

        let shell = compose_app_router_static_shell(&documents, &[], &DxStateGraph::default(), &[]);
        let shell_json = shell.to_json();

        assert_eq!(shell.status, "redirect-boundary-ready");
        assert_eq!(shell.leaf_role, "redirect");
        assert!(shell.redirect_boundary_selected);
        assert!(!shell.not_found_boundary_selected);
        assert!(shell.html.contains(r#"data-dx-app-router-redirect="true""#));
        assert!(
            shell
                .html
                .contains(r#"data-dx-next-redirect-destination="/login""#)
        );
        assert!(shell.html.contains(r#"data-dx-next-redirect-type="push""#));
        assert!(!shell.html.contains("Secret page"));
        assert_eq!(shell_json["selected_leaf"]["role"], "redirect");
        assert_eq!(shell_json["selected_leaf"]["redirect_selected"], true);
        assert_eq!(shell_json["redirect"]["destination"], "/login");
    }

    #[test]
    fn redirect_control_flow_replaces_aliased_redirect_page_leaf() {
        let page_source = r#"import { redirect as go, RedirectType as Mode } from "next/navigation";

export default function Page() {
  go("/dashboard", Mode.push);
  return <section>Private dashboard</section>;
}
"#;
        let documents = vec![LoweredSourceDocument {
            role: "page",
            source_path: "app/dashboard/page.tsx".to_string(),
            source: page_source.to_string(),
            document: lower_react_jsx_source("app/dashboard/page.tsx", page_source),
        }];

        let shell = compose_app_router_static_shell(&documents, &[], &DxStateGraph::default(), &[]);
        let shell_json = shell.to_json();

        assert_eq!(shell.status, "redirect-boundary-ready");
        assert_eq!(shell.leaf_role, "redirect");
        assert!(shell.redirect_boundary_selected);
        assert!(shell.html.contains(r#"data-dx-app-router-redirect="true""#));
        assert!(
            shell
                .html
                .contains(r#"data-dx-next-redirect-destination="/dashboard""#)
        );
        assert!(shell.html.contains(r#"data-dx-next-redirect-type="push""#));
        assert!(!shell.html.contains("Private dashboard"));
        assert_eq!(shell_json["redirect"]["destination"], "/dashboard");
        assert_eq!(shell_json["redirect"]["local_helper"], "go()");
        assert_eq!(shell_json["redirect"]["aliased_helper"], true);
    }

    #[test]
    fn redirect_control_flow_replaces_namespace_redirect_page_leaf() {
        let page_source = r#"import * as navigation from "next/navigation";

export default function Page() {
  navigation.redirect("/settings", navigation.RedirectType.push);
  return <section>Private settings</section>;
}
"#;
        let documents = vec![LoweredSourceDocument {
            role: "page",
            source_path: "app/settings/page.tsx".to_string(),
            source: page_source.to_string(),
            document: lower_react_jsx_source("app/settings/page.tsx", page_source),
        }];

        let shell = compose_app_router_static_shell(&documents, &[], &DxStateGraph::default(), &[]);
        let shell_json = shell.to_json();

        assert_eq!(shell.status, "redirect-boundary-ready");
        assert_eq!(shell.leaf_role, "redirect");
        assert!(shell.redirect_boundary_selected);
        assert!(shell.html.contains(r#"data-dx-app-router-redirect="true""#));
        assert!(
            shell
                .html
                .contains(r#"data-dx-next-redirect-destination="/settings""#)
        );
        assert!(shell.html.contains(r#"data-dx-next-redirect-type="push""#));
        assert!(!shell.html.contains("Private settings"));
        assert_eq!(shell_json["redirect"]["destination"], "/settings");
        assert_eq!(
            shell_json["redirect"]["local_helper"],
            "navigation.redirect()"
        );
        assert_eq!(shell_json["redirect"]["aliased_helper"], true);
    }

    #[test]
    fn page_named_redirect_function_is_not_navigation_control_flow_without_import() {
        let page_source = r#"import { notFound } from "next/navigation";

function redirect(destination) {
  return destination;
}

export default function Page() {
  redirect("/local-only");
  return <section>Local redirect function rendered</section>;
}
"#;
        let documents = vec![LoweredSourceDocument {
            role: "page",
            source_path: "app/local/page.tsx".to_string(),
            source: page_source.to_string(),
            document: lower_react_jsx_source("app/local/page.tsx", page_source),
        }];

        let shell = compose_app_router_static_shell(&documents, &[], &DxStateGraph::default(), &[]);
        let shell_json = shell.to_json();

        assert_eq!(shell.status, "app-router-shell-ready");
        assert_eq!(shell.leaf_role, "page");
        assert!(!shell.redirect_boundary_selected);
        assert!(shell.html.contains("Local redirect function rendered"));
        assert!(!shell.html.contains(r#"data-dx-app-router-redirect="true""#));
        assert!(shell_json["redirect"].is_null());
        assert_eq!(shell_json["selected_leaf"]["role"], "page");
    }

    #[test]
    fn error_boundary_replaces_page_leaf_for_static_throw() {
        let error_source = r#"export default function ErrorBoundary() {
  return <section><h1>Route failed</h1></section>;
}
"#;
        let page_source = r#"export default function Page() {
  throw new Error("route failed");
  return <section>Secret page</section>;
}
"#;
        let documents = vec![
            LoweredSourceDocument {
                role: "error",
                source_path: "app/blog/error.tsx".to_string(),
                source: error_source.to_string(),
                document: lower_react_jsx_source("app/blog/error.tsx", error_source),
            },
            LoweredSourceDocument {
                role: "page",
                source_path: "app/blog/[slug]/page.tsx".to_string(),
                source: page_source.to_string(),
                document: lower_react_jsx_source("app/blog/[slug]/page.tsx", page_source),
            },
        ];

        let shell = compose_app_router_static_shell(&documents, &[], &DxStateGraph::default(), &[]);
        let shell_json = shell.to_json();

        assert_eq!(shell.status, "error-boundary-ready");
        assert_eq!(shell.leaf_role, "error");
        assert!(shell.error_boundary_selected);
        assert!(!shell.not_found_boundary_selected);
        assert!(shell.html.contains("Route failed"));
        assert!(!shell.html.contains("Secret page"));
        assert_eq!(shell_json["selected_leaf"]["role"], "error");
        assert_eq!(shell_json["selected_leaf"]["error_boundary_selected"], true);
    }

    #[test]
    fn error_boundary_does_not_use_child_layouts_below_selected_boundary_and_records_skip() {
        let root_layout_source = r#"export default function Layout({ children }) {
  return <main><h1>Root shell</h1>{children}</main>;
}
"#;
        let error_source = r#"export default function ErrorBoundary({ error }) {
  return <section><h2>{error.message}</h2></section>;
}
"#;
        let child_layout_source = r#"export default function Layout({ children }) {
  return <aside><h3>Child slug shell</h3>{children}</aside>;
}
"#;
        let page_source = r#"export default function Page() {
  throw new Error("route exploded");
  return <section>Secret post</section>;
}
"#;
        let documents = vec![
            LoweredSourceDocument {
                role: "layout",
                source_path: "app/layout.tsx".to_string(),
                source: root_layout_source.to_string(),
                document: lower_react_jsx_source("app/layout.tsx", root_layout_source),
            },
            LoweredSourceDocument {
                role: "error",
                source_path: "app/blog/error.tsx".to_string(),
                source: error_source.to_string(),
                document: lower_react_jsx_source("app/blog/error.tsx", error_source),
            },
            LoweredSourceDocument {
                role: "layout",
                source_path: "app/blog/[slug]/layout.tsx".to_string(),
                source: child_layout_source.to_string(),
                document: lower_react_jsx_source("app/blog/[slug]/layout.tsx", child_layout_source),
            },
            LoweredSourceDocument {
                role: "page",
                source_path: "app/blog/[slug]/page.tsx".to_string(),
                source: page_source.to_string(),
                document: lower_react_jsx_source("app/blog/[slug]/page.tsx", page_source),
            },
        ];

        let shell = compose_app_router_static_shell(&documents, &[], &DxStateGraph::default(), &[]);
        let shell_json = shell.to_json();

        assert_eq!(shell.status, "error-boundary-ready");
        assert_eq!(shell.leaf_role, "error");
        assert!(shell.html.contains("Root shell"));
        assert!(shell.html.contains("route exploded"));
        assert!(!shell.html.contains("Child slug shell"));
        assert!(!shell.html.contains("Secret post"));
        assert_eq!(shell.scope_skipped_wrappers.len(), 1);
        assert_eq!(shell_json["scope_skipped_wrapper_count"], 1);
        assert_eq!(
            shell_json["scope_skipped_wrappers"][0]["source_path"],
            "app/blog/[slug]/layout.tsx"
        );
        assert_eq!(
            shell_json["scope_skipped_wrappers"][0]["reason"],
            "below-selected-boundary-scope"
        );
        assert_eq!(
            shell_json["scope_skipped_wrappers"][0]["selected_leaf_source_path"],
            "app/blog/error.tsx"
        );
    }

    #[test]
    fn error_boundary_records_error_and_reset_props_without_client_runtime() {
        let error_source = r#"export default function ErrorBoundary({ error, reset }) {
  return <section><h1>{error.message}</h1><button onClick={reset}>Try again</button></section>;
}
"#;
        let page_source = r#"export default function Page() {
  throw new Error("route failed from page");
  return <section>Secret page</section>;
}
"#;
        let documents = vec![
            LoweredSourceDocument {
                role: "error",
                source_path: "app/blog/error.tsx".to_string(),
                source: error_source.to_string(),
                document: lower_react_jsx_source("app/blog/error.tsx", error_source),
            },
            LoweredSourceDocument {
                role: "page",
                source_path: "app/blog/page.tsx".to_string(),
                source: page_source.to_string(),
                document: lower_react_jsx_source("app/blog/page.tsx", page_source),
            },
        ];

        let shell = compose_app_router_static_shell(&documents, &[], &DxStateGraph::default(), &[]);
        let shell_json = shell.to_json();
        let props = shell_json["error_boundary_props"]
            .as_object()
            .expect("error boundary props");

        assert_eq!(shell.status, "error-boundary-ready");
        assert_eq!(shell.leaf_role, "error");
        assert!(shell.html.contains("route failed from page"));
        assert!(shell.html.contains("Try again"));
        assert_eq!(props["source_owned_error_boundary_props"], true);
        assert_eq!(props["error"]["message"], "route failed from page");
        assert_eq!(props["error"]["message_binding"], "error.message");
        assert_eq!(props["reset"]["binding"], "reset");
        assert_eq!(props["reset"]["reset_invocable"], false);
        assert_eq!(props["full_client_error_runtime"], false);
        assert_eq!(props["full_react_error_boundary_runtime"], false);
    }

    #[test]
    fn static_page_error_throw_detector_ignores_comments_and_strings() {
        assert!(!detects_static_page_error_throw(
            r#"const text = "throw new Error(\"fake\")";
// throw new Error("comment")
/* throw Error("comment") */"#
        ));
        assert!(detects_static_page_error_throw(
            r#"export default function Page() {
  throw Error("route failed");
}"#
        ));
    }

    #[test]
    fn loading_boundary_reuses_static_loading_segment_for_deferred_page() {
        let loading_source = r#"export default function Loading() {
  return <section><p>Loading article</p></section>;
}
"#;
        let page_source = r#"export default async function Page() {
  const article = await loadArticle();
  return <section>Article page</section>;
}
"#;
        let documents = vec![
            LoweredSourceDocument {
                role: "loading",
                source_path: "app/blog/loading.tsx".to_string(),
                source: loading_source.to_string(),
                document: lower_react_jsx_source("app/blog/loading.tsx", loading_source),
            },
            LoweredSourceDocument {
                role: "page",
                source_path: "app/blog/page.tsx".to_string(),
                source: page_source.to_string(),
                document: lower_react_jsx_source("app/blog/page.tsx", page_source),
            },
        ];

        let shell = compose_app_router_static_shell(&documents, &[], &DxStateGraph::default(), &[]);
        let shell_json = shell.to_json();
        let loading_boundary = shell_json["loading_boundary"]
            .as_object()
            .expect("loading boundary preview");

        assert_eq!(shell.status, "app-router-shell-ready-with-loading-boundary");
        assert_eq!(shell.leaf_role, "page");
        assert!(shell.html.contains("Article page"));
        assert_eq!(loading_boundary["status"], "loading-boundary-ready");
        assert!(
            loading_boundary["html"]
                .as_str()
                .expect("loading html")
                .contains("Loading article")
        );
        assert_eq!(loading_boundary["source_owned_loading_boundary"], true);
        assert_eq!(loading_boundary["full_streaming_runtime"], false);
        assert_eq!(loading_boundary["full_react_suspense_runtime"], false);
    }

    #[test]
    fn deferred_page_detector_ignores_comments_and_strings() {
        assert!(!detects_deferred_page_render(
            r#"const text = "await loadArticle()";
// await loadArticle()
/* await loadArticle() */"#
        ));
        assert!(detects_deferred_page_render(
            r#"export default async function Page() {
  const article = await loadArticle();
  return <section>{article.title}</section>;
}"#
        ));
    }

    #[test]
    fn template_boundary_records_remount_semantics_without_runtime_parity() {
        let layout_source = r#"export default function Layout({ children }) {
  return <html><body>{children}</body></html>;
}
"#;
        let template_source = r#"export default function Template({ children }) {
  return <section data-template="blog"><h2>Template shell</h2>{children}</section>;
}
"#;
        let page_source = r#"export default function Page() {
  return <article>Page body</article>;
}
"#;
        let documents = vec![
            LoweredSourceDocument {
                role: "layout",
                source_path: "app/blog/layout.tsx".to_string(),
                source: layout_source.to_string(),
                document: lower_react_jsx_source("app/blog/layout.tsx", layout_source),
            },
            LoweredSourceDocument {
                role: "template",
                source_path: "app/blog/template.tsx".to_string(),
                source: template_source.to_string(),
                document: lower_react_jsx_source("app/blog/template.tsx", template_source),
            },
            LoweredSourceDocument {
                role: "page",
                source_path: "app/blog/page.tsx".to_string(),
                source: page_source.to_string(),
                document: lower_react_jsx_source("app/blog/page.tsx", page_source),
            },
        ];

        let shell = compose_app_router_static_shell(&documents, &[], &DxStateGraph::default(), &[]);
        let shell_json = shell.to_json();
        let boundaries = shell_json["template_boundaries"]
            .as_array()
            .expect("template boundaries");
        let boundary = boundaries.first().expect("one template boundary");

        assert_eq!(boundaries.len(), 1);
        assert_eq!(shell.template_boundaries.len(), 1);
        assert_eq!(shell.status, "app-router-shell-ready");
        assert!(shell.html.contains("Template shell"));
        assert!(shell.html.contains("Page body"));
        assert_eq!(boundary["status"], "template-boundary-ready");
        assert_eq!(boundary["source_path"], "app/blog/template.tsx");
        assert_eq!(boundary["source_owned_template_boundary"], true);
        assert_eq!(boundary["remount_on_navigation"], true);
        assert_eq!(boundary["persistent_across_navigation"], false);
        assert_eq!(boundary["full_react_template_runtime"], false);
    }

    #[test]
    fn app_router_shell_keeps_safe_external_scripts_visible() {
        let layout_source = r#"export default function Layout({ children }) {
  return <html><body>{children}</body></html>;
}
"#;
        let page_source = r#"export default function Page() {
  return <main data-whiteboard-route="direct-renderable-workbench">
    <p>Board ready</p>
    <script type="module" src="/whiteboard-runtime.ts" data-whiteboard-runtime="source-owned"></script>
  </main>;
}
"#;
        let documents = vec![
            LoweredSourceDocument {
                role: "layout",
                source_path: "app/layout.tsx".to_string(),
                source: layout_source.to_string(),
                document: lower_react_jsx_source("app/layout.tsx", layout_source),
            },
            LoweredSourceDocument {
                role: "page",
                source_path: "app/page.tsx".to_string(),
                source: page_source.to_string(),
                document: lower_react_jsx_source("app/page.tsx", page_source),
            },
        ];

        let shell = compose_app_router_static_shell(&documents, &[], &DxStateGraph::default(), &[]);

        assert_eq!(shell.status, "app-router-shell-ready");
        assert!(shell.html.contains("Board ready"));
        assert!(shell.html.contains(r#"<script type="module" src="/whiteboard-runtime.ts" data-whiteboard-runtime="source-owned"></script>"#));
    }

    #[test]
    fn app_router_shell_closes_self_closing_non_void_static_elements() {
        let page_source = r#"export default function Page() {
  return <main>
    <div data-dx-contract="dx-mobile-companion" hidden />
    <script src="/mobile-companion-runtime.js" defer />
  </main>;
}
"#;
        let documents = vec![LoweredSourceDocument {
            role: "page",
            source_path: "app/page.tsx".to_string(),
            source: page_source.to_string(),
            document: lower_react_jsx_source("app/page.tsx", page_source),
        }];

        let shell = compose_app_router_static_shell(&documents, &[], &DxStateGraph::default(), &[]);

        assert!(
            shell
                .html
                .contains(r#"<div data-dx-contract="dx-mobile-companion" hidden></div>"#),
            "{}",
            shell.html
        );
        assert!(
            shell
                .html
                .contains(r#"<script src="/mobile-companion-runtime.js" defer></script>"#),
            "{}",
            shell.html
        );
    }

    #[test]
    fn app_router_shell_preserves_mobile_form_attributes() {
        let page_source = r#"export default function Page() {
  return <main>
    <input
      id="dx-mobile-pairing-code"
      name="pairing_code"
      type="text"
      inputMode="numeric"
      autoComplete="one-time-code"
      autoCapitalize="none"
      enterKeyHint="done"
      required
    />
    <textarea
      id="dx-mobile-pairing-payload"
      name="pairing_payload"
      rows={4}
      spellCheck={false}
    ></textarea>
  </main>;
}
"#;
        let documents = vec![LoweredSourceDocument {
            role: "page",
            source_path: "app/page.tsx".to_string(),
            source: page_source.to_string(),
            document: lower_react_jsx_source("app/page.tsx", page_source),
        }];

        let shell = compose_app_router_static_shell(&documents, &[], &DxStateGraph::default(), &[]);

        assert!(
            shell.html.contains(r#"<input id="dx-mobile-pairing-code" name="pairing_code" type="text" inputmode="numeric" autocomplete="one-time-code" autocapitalize="none" enterkeyhint="done" required>"#),
            "{}",
            shell.html
        );
        assert!(
            shell.html.contains(r#"<textarea id="dx-mobile-pairing-payload" name="pairing_payload" rows="4" spellcheck="false"></textarea>"#),
            "{}",
            shell.html
        );
    }

    #[test]
    fn app_router_shell_skips_inline_scripts() {
        let page_source = r#"export default function Page() {
  return <main>
    <script>window.__DX_UNSAFE_INLINE_SCRIPT__ = true;</script>
    <p>Safe content</p>
  </main>;
}
"#;
        let documents = vec![LoweredSourceDocument {
            role: "page",
            source_path: "app/page.tsx".to_string(),
            source: page_source.to_string(),
            document: lower_react_jsx_source("app/page.tsx", page_source),
        }];

        let shell = compose_app_router_static_shell(&documents, &[], &DxStateGraph::default(), &[]);

        assert!(shell.html.contains("Safe content"));
        assert!(!shell.html.contains("__DX_UNSAFE_INLINE_SCRIPT__"));
        assert!(!shell.html.contains("<script>"));
    }

    #[test]
    fn static_dom_reflects_exact_global_store_reads_across_component_files() {
        let component_source = r#"export function CounterBadge() {
  return (
    <section>
      <span>{counterStore.count}</span>
      <input value={counterStore.count} aria-label="Count" />
    </section>
  );
}
"#;
        let documents = vec![LoweredSourceDocument {
            role: "source-owned-import",
            source_path: "components/counter-badge.tsx".to_string(),
            source: component_source.to_string(),
            document: lower_react_jsx_source("components/counter-badge.tsx", component_source),
        }];
        let mut state_graph = DxStateGraph::default();
        state_graph.slots.push(DxStateSlot {
            id: "store-counter-state-count".to_string(),
            name: "counterStore.count".to_string(),
            setter: None,
            scope: DxStateScope::Global,
            source_path: "lib/stores/counter.ts".to_string(),
            initial_source: "1".to_string(),
            value_kind: "number".to_string(),
        });

        let reflection_plan = build_state_dom_reflection_plan("/counter", &documents, &state_graph);
        let snapshot = build_static_dom_snapshot("/counter", &documents, &state_graph, &[]);
        let html = snapshot["html"].as_str().expect("snapshot html");

        assert_eq!(reflection_plan["reflection_count"], 2);
        assert!(html.contains(r#"data-dx-state-read="counterStore.count""#));
        assert!(html.contains(r#"data-dx-state-value="counterStore.count""#));
    }

    #[test]
    fn records_safe_async_next_app_router_page_prop_aliases() {
        let source = r#"export default async function Page({ params, searchParams }) {
  const { slug, locale: region, missing } = await params;
  const preview = (await searchParams).query;
  const missingPreview = (await searchParams).missing;
  return <main>{slug}{region}{preview}</main>;
}
"#;
        let documents = vec![LoweredSourceDocument {
            role: "page",
            source_path: "app/blog/[slug]/page.tsx".to_string(),
            source: source.to_string(),
            document: lower_react_jsx_source("app/blog/[slug]/page.tsx", source),
        }];
        let route_params = BTreeMap::from([
            ("slug".to_string(), "acme".to_string()),
            ("locale".to_string(), "en".to_string()),
        ]);
        let search_params = BTreeMap::from([("query".to_string(), "draft".to_string())]);

        let alias_collection =
            collect_next_app_router_page_prop_aliases(&documents, &route_params, &search_params);
        assert!(alias_collection.resolved.iter().any(|binding| {
            binding.alias == "slug"
                && binding.canonical_name == "params.slug"
                && binding.expression == "const { slug } = await params"
        }));
        assert!(alias_collection.resolved.iter().any(|binding| {
            binding.alias == "region"
                && binding.canonical_name == "params.locale"
                && binding.expression == "const { locale: region } = await params"
        }));
        assert!(alias_collection.resolved.iter().any(|binding| {
            binding.alias == "preview"
                && binding.canonical_name == "searchParams.query"
                && binding.expression == "const preview = (await searchParams).query"
        }));
        assert!(alias_collection.unresolved.iter().any(|binding| {
            binding.alias == "missing"
                && binding.canonical_name == "params.missing"
                && binding.reason == "missing-request-prop-value"
        }));
        assert!(alias_collection.unresolved.iter().any(|binding| {
            binding.alias == "missingPreview"
                && binding.canonical_name == "searchParams.missing"
                && binding.expression == "const missingPreview = (await searchParams).missing"
        }));

        let bindings =
            request_prop_bindings(&route_params, &search_params, &alias_collection.resolved);
        assert_eq!(
            resolve_component_prop_identifier("slug", &bindings, None)
                .expect("slug alias")
                .to_text(),
            "acme"
        );
        assert_eq!(
            resolve_component_prop_identifier("preview", &bindings, None)
                .expect("preview alias")
                .to_text(),
            "draft"
        );
        assert_eq!(
            next_app_router_page_prop_binding_name("(await params).slug").as_deref(),
            Some("params.slug")
        );
        assert_eq!(
            next_app_router_page_prop_binding_name("(await searchParams)[\"query\"]").as_deref(),
            Some("searchParams.query")
        );
    }

    #[test]
    fn resolves_trimmed_static_template_literal_prop_binding() {
        let bindings = vec![string_prop("className", "primary")];
        let aliases = empty_aliases();

        let value = resolve_static_template_with_prop_bindings(
            "` card ${className} `.trim()",
            &bindings,
            Some(&aliases),
        )
        .expect("template literal should resolve from literal caller prop");

        assert_eq!(value.to_text(), "card primary");
    }

    #[test]
    fn resolves_props_member_template_and_rejects_function_calls() {
        let bindings = vec![string_prop("name", "Friday")];
        let aliases = empty_aliases();

        let value = resolve_static_template_with_prop_bindings(
            "`Hello ${props.name}`",
            &bindings,
            Some(&aliases),
        )
        .expect("props.name interpolation should resolve from caller prop");

        assert_eq!(value.to_text(), "Hello Friday");
        assert!(
            resolve_static_template_with_prop_bindings(
                "`Hello ${formatName(name)}`",
                &bindings,
                Some(&aliases),
            )
            .is_none(),
            "template interpolation must not execute function calls"
        );
    }

    #[test]
    fn resolves_static_conditional_prop_branch() {
        let bindings = vec![
            string_prop("variant", "primary"),
            ComponentPropBinding {
                name: "active".to_string(),
                value: StaticLiteralExpression::Boolean(false),
                source_kind: "literal-expression",
                expression: Some("false".to_string()),
            },
        ];
        let aliases = empty_aliases();

        let value = resolve_static_conditional_with_prop_bindings(
            r#"variant === "primary" ? `button ${variant}` : "button""#,
            &bindings,
            Some(&aliases),
        )
        .expect("static equality condition should choose the matching branch");

        assert_eq!(value.to_text(), "button primary");
        let inactive = resolve_static_conditional_with_prop_bindings(
            r#"active ? "enabled" : "disabled""#,
            &bindings,
            Some(&aliases),
        )
        .expect("boolean prop condition should choose the false branch");

        assert_eq!(inactive.to_text(), "disabled");
    }

    #[test]
    fn resolves_static_class_list_prop_bindings() {
        let bindings = vec![
            string_prop("variant", "primary"),
            ComponentPropBinding {
                name: "active".to_string(),
                value: StaticLiteralExpression::Boolean(true),
                source_kind: "literal-expression",
                expression: Some("true".to_string()),
            },
            ComponentPropBinding {
                name: "disabled".to_string(),
                value: StaticLiteralExpression::Boolean(false),
                source_kind: "literal-expression",
                expression: Some("false".to_string()),
            },
        ];
        let aliases = empty_aliases();

        let value = resolve_static_class_list_with_prop_bindings(
            r#"["button", active && "is-active", disabled && "is-disabled", variant === "primary" && `button-${variant}`].filter(Boolean).join(" ")"#,
            &bindings,
            Some(&aliases),
        )
        .expect("static class-list expression should resolve from literal caller props");

        assert_eq!(value.to_text(), "button is-active button-primary");
        assert!(
            resolve_static_class_list_with_prop_bindings(
                r#"[formatName(variant)].filter(Boolean).join(" ")"#,
                &bindings,
                Some(&aliases),
            )
            .is_none(),
            "class-list lowering must not execute function calls"
        );
    }

    #[test]
    fn resolves_static_class_call_prop_bindings() {
        let bindings = vec![
            string_prop("variant", "primary"),
            ComponentPropBinding {
                name: "active".to_string(),
                value: StaticLiteralExpression::Boolean(true),
                source_kind: "literal-expression",
                expression: Some("true".to_string()),
            },
            ComponentPropBinding {
                name: "disabled".to_string(),
                value: StaticLiteralExpression::Boolean(false),
                source_kind: "literal-expression",
                expression: Some("false".to_string()),
            },
        ];
        let aliases = empty_aliases();

        let value = resolve_static_class_call_with_prop_bindings(
            r#"cn("button", active && "is-active", disabled && "is-disabled", variant === "primary" && `button-${variant}`)"#,
            &bindings,
            Some(&aliases),
        )
        .expect("static cn expression should resolve from literal caller props");

        assert_eq!(value.to_text(), "button is-active button-primary");
        let clsx_value = resolve_static_class_call_with_prop_bindings(
            r#"clsx("button", variant === "primary" ? "is-primary" : "is-secondary")"#,
            &bindings,
            Some(&aliases),
        )
        .expect("static clsx ternary expression should resolve from literal caller props");

        assert_eq!(clsx_value.to_text(), "button is-primary");
        assert!(
            resolve_static_class_call_with_prop_bindings(
                r#"cn(formatName(variant))"#,
                &bindings,
                Some(&aliases),
            )
            .is_none(),
            "class-call lowering must not execute function calls"
        );
        assert!(
            resolve_static_class_call_with_prop_bindings(
                r#"cn({ active })"#,
                &bindings,
                Some(&aliases),
            )
            .is_none(),
            "class-call lowering must not treat object arguments as static proof"
        );
    }

    #[test]
    fn renders_dx_icon_components_as_static_svg() {
        let source = r#"import { Icon } from "@/components/icons/icon";

export default function Page() {
  return (
    <main>
      <Icon name="status:check" className="size-4" title="Ready" />
      <dx-icon name="action:menu" className="size-5" aria-hidden="true" />
    </main>
  );
}
"#;
        let documents = vec![LoweredSourceDocument {
            role: "page",
            source_path: "app/counter/page.tsx".to_string(),
            source: source.to_string(),
            document: lower_react_jsx_source("app/counter/page.tsx", source),
        }];

        let snapshot =
            composed_static_document_snapshot(&documents[0], &[], &DxStateGraph::default(), &[]);

        assert!(snapshot.html.contains(r#"data-icon-source="dx-icons""#));
        assert!(snapshot.html.contains(r#"data-dx-icon="status:check""#));
        assert!(snapshot.html.contains(r#"data-dx-icon-set="status""#));
        assert!(snapshot.html.contains(r#"data-dx-icon-name="check""#));
        assert!(snapshot.html.contains(r#"class="dx-icon size-4""#));
        assert!(snapshot.html.contains("<title>Ready</title>"));
        assert!(snapshot.html.contains(r#"data-dx-icon="action:menu""#));
        assert!(snapshot.html.contains(r#"class="dx-icon size-5""#));
        assert!(
            snapshot
                .html
                .contains(r#"<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>"#)
                && snapshot.html.contains(r#"<path d="m9 11 3 3L22 4"/>"#),
            "known DX icons should render their real SVG body"
        );
        assert!(
            !snapshot.html.contains(r#"M4 4h16v16H4z"#),
            "known DX icons should not render the generic square fallback"
        );
        assert!(
            !snapshot.html.contains("data-dx-icon-missing"),
            "known DX icons should render real SVG bodies"
        );
    }
}
