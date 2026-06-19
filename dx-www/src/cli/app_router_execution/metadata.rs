use std::collections::{BTreeMap, BTreeSet};

use dx_compiler::delivery::{DxReactAppSegmentSource, parse_tsx_module};
use serde_json::{Map, Number, Value, json};

use super::directives::has_use_client_directive;

pub(super) fn metadata_sources(
    segments: &[DxReactAppSegmentSource],
    route_source_path: &str,
    route_source: &str,
    route_params: &BTreeMap<String, String>,
    search_params: &BTreeMap<String, String>,
) -> Vec<Value> {
    let mut sources = segments
        .iter()
        .filter_map(|segment| {
            metadata_source(
                &segment.source_path,
                &segment.source,
                route_params,
                search_params,
            )
        })
        .collect::<Vec<_>>();
    if let Some(page_metadata) =
        metadata_source(route_source_path, route_source, route_params, search_params)
    {
        sources.push(page_metadata);
    }
    sources
}

pub(super) fn effective_metadata(sources: &[Value]) -> Value {
    let mut title = Value::Null;
    let mut description = Value::Null;
    let mut canonical = Value::Null;
    let mut title_source = Value::Null;
    let mut title_template = Value::Null;
    let mut title_template_source = Value::Null;
    let mut title_absolute = Value::Null;
    let mut title_absolute_source = Value::Null;
    let mut description_source = Value::Null;
    let mut canonical_source = Value::Null;
    let mut open_graph = Map::new();
    let mut open_graph_field_sources = Map::new();
    let mut viewport = Map::new();
    let mut viewport_field_sources = Map::new();
    let mut merged_source_paths = Vec::new();

    for source in sources {
        if let Some(source_path) = source.get("source_path").and_then(Value::as_str) {
            merged_source_paths.push(source_path.to_string());
        }
        merge_title_metadata(
            source,
            &mut title,
            &mut title_source,
            &mut title_template,
            &mut title_template_source,
            &mut title_absolute,
            &mut title_absolute_source,
        );
        merge_metadata_field(
            source,
            "description",
            &mut description,
            &mut description_source,
        );
        merge_metadata_field(source, "canonical", &mut canonical, &mut canonical_source);
        merge_open_graph_metadata(source, &mut open_graph, &mut open_graph_field_sources);
        merge_viewport_metadata(source, &mut viewport, &mut viewport_field_sources);
    }

    let open_graph = if open_graph.is_empty() {
        Value::Null
    } else {
        Value::Object(open_graph)
    };
    let open_graph_field_sources = if open_graph_field_sources.is_empty() {
        Value::Null
    } else {
        Value::Object(open_graph_field_sources)
    };
    let viewport = if viewport.is_empty() {
        Value::Null
    } else {
        Value::Object(viewport)
    };
    let viewport_field_sources = if viewport_field_sources.is_empty() {
        Value::Null
    } else {
        Value::Object(viewport_field_sources)
    };
    let open_graph_field_sources_for_fields = open_graph_field_sources.clone();
    let viewport_field_sources_for_fields = viewport_field_sources.clone();

    json!({
        "title": title,
        "description": description,
        "canonical": canonical,
        "title_template": title_template,
        "title_absolute": title_absolute,
        "openGraph": open_graph,
        "viewport": viewport,
        "source_owned_metadata_merge": true,
        "source_owned_title_template": !title_template_source.is_null(),
        "source_owned_title_absolute": !title_absolute_source.is_null(),
        "source_owned_open_graph_metadata": true,
        "source_owned_viewport_metadata": true,
        "metadata_merge_precedence": "segment-order-parent-to-leaf",
        "source_count": sources.len(),
        "merged_source_paths": merged_source_paths,
        "open_graph_field_sources": open_graph_field_sources,
        "viewport_field_sources": viewport_field_sources,
        "field_sources": {
            "title": title_source,
            "title_template": title_template_source,
            "title_absolute": title_absolute_source,
            "description": description_source,
            "canonical": canonical_source,
            "openGraph": open_graph_field_sources_for_fields,
            "viewport": viewport_field_sources_for_fields,
        },
        "node_modules_required": false,
        "external_runtime_required": false,
        "external_runtime_executed": false,
        "full_next_metadata_runtime": false,
        "full_next_title_runtime": false,
        "full_next_open_graph_runtime": false,
        "full_next_viewport_runtime": false,
        "limits": [
            "Merges safe layout, template, and page metadata fields in App Router segment order.",
            "Applies safe title.default and title.template metadata without executing the Next.js metadata runtime.",
            "Carries safe title.absolute as an explicit template-bypassing title source.",
            "Reads safe openGraph title, description, url, siteName, and images fields.",
            "Reads safe viewport width, height, themeColor, colorScheme, interactiveWidget, scale, and userScalable fields.",
            "Leaf fields override parent fields only when the leaf field is statically resolved.",
            "Does not execute dynamic metadata, streaming metadata, parent ResolvingMetadata, image object expansion, cookies, headers, or the Next.js runtime."
        ],
    })
}

pub(super) fn metadata_head_tags(metadata: &Value) -> String {
    let mut tags = Vec::new();
    if let Some(title) = metadata.get("title").and_then(Value::as_str) {
        tags.push(format!(
            r#"<title data-dx-metadata="title">{}</title>"#,
            escape_html_text(title)
        ));
    }
    if let Some(description) = metadata.get("description").and_then(Value::as_str) {
        tags.push(format!(
            r#"<meta name="description" content="{}" data-dx-metadata="description" />"#,
            escape_html_attr(description)
        ));
    }
    if let Some(canonical) = metadata.get("canonical").and_then(Value::as_str) {
        tags.push(format!(
            r#"<link rel="canonical" href="{}" data-dx-metadata="canonical" />"#,
            escape_html_attr(canonical)
        ));
    }
    open_graph_head_tags(metadata.get("openGraph").unwrap_or(&Value::Null), &mut tags);
    viewport_head_tags(metadata.get("viewport").unwrap_or(&Value::Null), &mut tags);
    tags.join("")
}

pub(super) fn metadata_head_tag_count(tags: &str) -> usize {
    tags.matches("data-dx-metadata=").count()
}

fn open_graph_head_tags(open_graph: &Value, tags: &mut Vec<String>) {
    let Some(open_graph) = open_graph.as_object() else {
        return;
    };
    for (field, property) in [
        ("title", "og:title"),
        ("description", "og:description"),
        ("url", "og:url"),
        ("siteName", "og:site_name"),
    ] {
        let Some(value) = open_graph.get(field).and_then(Value::as_str) else {
            continue;
        };
        tags.push(format!(
            r#"<meta property="{property}" content="{}" data-dx-metadata="openGraph" />"#,
            escape_html_attr(value)
        ));
    }
    let Some(images) = open_graph.get("images") else {
        return;
    };
    if let Some(image) = images.as_str() {
        tags.push(open_graph_image_tag(image));
        return;
    }
    if let Some(images) = images.as_array() {
        for image in images.iter().filter_map(Value::as_str) {
            tags.push(open_graph_image_tag(image));
        }
    }
}

fn open_graph_image_tag(image: &str) -> String {
    format!(
        r#"<meta property="og:image" content="{}" data-dx-metadata="openGraph" />"#,
        escape_html_attr(image)
    )
}

fn viewport_head_tags(viewport: &Value, tags: &mut Vec<String>) {
    let Some(viewport) = viewport.as_object() else {
        return;
    };
    if let Some(content) = viewport_content(viewport) {
        tags.push(format!(
            r#"<meta name="viewport" content="{}" data-dx-metadata="viewport" />"#,
            escape_html_attr(&content)
        ));
    }
    if let Some(theme_color) = viewport.get("themeColor").and_then(Value::as_str) {
        tags.push(format!(
            r#"<meta name="theme-color" content="{}" data-dx-metadata="viewport" />"#,
            escape_html_attr(theme_color)
        ));
    }
    if let Some(color_scheme) = viewport.get("colorScheme").and_then(Value::as_str) {
        tags.push(format!(
            r#"<meta name="color-scheme" content="{}" data-dx-metadata="viewport" />"#,
            escape_html_attr(color_scheme)
        ));
    }
}

fn viewport_content(viewport: &Map<String, Value>) -> Option<String> {
    let mut parts = Vec::new();
    for (field, name) in [
        ("width", "width"),
        ("height", "height"),
        ("initialScale", "initial-scale"),
        ("minimumScale", "minimum-scale"),
        ("maximumScale", "maximum-scale"),
        ("interactiveWidget", "interactive-widget"),
    ] {
        let Some(value) = viewport.get(field).and_then(metadata_string_or_number) else {
            continue;
        };
        parts.push(format!("{name}={value}"));
    }
    if let Some(user_scalable) = viewport.get("userScalable").and_then(Value::as_bool) {
        parts.push(format!(
            "user-scalable={}",
            if user_scalable { "yes" } else { "no" }
        ));
    }
    (!parts.is_empty()).then(|| parts.join(","))
}

fn metadata_string_or_number(value: &Value) -> Option<String> {
    if let Some(value) = value.as_str() {
        return Some(value.to_string());
    }
    if let Some(value) = value.as_i64() {
        return Some(value.to_string());
    }
    if let Some(value) = value.as_u64() {
        return Some(value.to_string());
    }
    value.as_f64().map(|value| {
        if value.fract() == 0.0 {
            format!("{value:.0}")
        } else {
            value.to_string()
        }
    })
}

fn escape_html_attr(value: &str) -> String {
    escape_html_text(value).replace('"', "&quot;")
}

fn escape_html_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn merge_metadata_field(source: &Value, field: &str, value: &mut Value, source_path: &mut Value) {
    let Some(next_value) = source.get(field) else {
        return;
    };
    if next_value.is_null() {
        return;
    }
    *value = next_value.clone();
    *source_path = source.get("source_path").cloned().unwrap_or(Value::Null);
}

fn merge_title_metadata(
    source: &Value,
    title: &mut Value,
    title_source: &mut Value,
    title_template: &mut Value,
    title_template_source: &mut Value,
    title_absolute: &mut Value,
    title_absolute_source: &mut Value,
) {
    let source_path = source.get("source_path").cloned().unwrap_or(Value::Null);
    if let Some(next_template) = source.get("title_template").and_then(Value::as_str) {
        *title_template = Value::String(next_template.to_string());
        *title_template_source = source_path.clone();
    }

    let Some(next_title) = source.get("title").and_then(Value::as_str) else {
        return;
    };
    let title_kind = source.get("title_kind").and_then(Value::as_str);
    let is_absolute_title = matches!(title_kind, Some("title-object-absolute"));
    let rendered_title = if matches!(
        title_kind,
        Some("title-object-default" | "title-object-absolute")
    ) {
        next_title.to_string()
    } else if let Some(template) = title_template.as_str() {
        apply_title_template(template, next_title).unwrap_or_else(|| next_title.to_string())
    } else {
        next_title.to_string()
    };
    *title = Value::String(rendered_title);
    *title_source = source_path.clone();
    if is_absolute_title {
        *title_absolute = Value::String(next_title.to_string());
        *title_absolute_source = source_path;
    } else {
        *title_absolute = Value::Null;
        *title_absolute_source = Value::Null;
    }
}

fn apply_title_template(template: &str, title: &str) -> Option<String> {
    template
        .contains("%s")
        .then(|| template.replacen("%s", title, 1))
}

fn merge_open_graph_metadata(
    source: &Value,
    open_graph: &mut Map<String, Value>,
    open_graph_field_sources: &mut Map<String, Value>,
) {
    let Some(next_open_graph) = source.get("openGraph").and_then(Value::as_object) else {
        return;
    };
    let source_path = source.get("source_path").cloned().unwrap_or(Value::Null);
    for field in ["title", "description", "url", "siteName", "images"] {
        let Some(next_value) = next_open_graph.get(field) else {
            continue;
        };
        if next_value.is_null() {
            continue;
        }
        open_graph.insert(field.to_string(), next_value.clone());
        open_graph_field_sources.insert(field.to_string(), source_path.clone());
    }
}

fn merge_viewport_metadata(
    source: &Value,
    viewport: &mut Map<String, Value>,
    viewport_field_sources: &mut Map<String, Value>,
) {
    let Some(next_viewport) = source.get("viewport").and_then(Value::as_object) else {
        return;
    };
    let source_path = source.get("source_path").cloned().unwrap_or(Value::Null);
    for field in [
        "width",
        "height",
        "themeColor",
        "colorScheme",
        "interactiveWidget",
        "initialScale",
        "minimumScale",
        "maximumScale",
        "userScalable",
    ] {
        let Some(next_value) = next_viewport.get(field) else {
            continue;
        };
        if next_value.is_null() {
            continue;
        }
        viewport.insert(field.to_string(), next_value.clone());
        viewport_field_sources.insert(field.to_string(), source_path.clone());
    }
}

fn metadata_source(
    source_path: &str,
    source: &str,
    route_params: &BTreeMap<String, String>,
    search_params: &BTreeMap<String, String>,
) -> Option<Value> {
    if has_use_client_directive(source) {
        return None;
    }

    let static_metadata = safe_static_metadata_source(source_path, source);
    let viewport_metadata =
        viewport_metadata_source(source_path, source, route_params, search_params);
    if let Some(mut metadata) = static_metadata {
        attach_viewport_metadata(&mut metadata, viewport_metadata.as_ref());
        return Some(metadata);
    }

    if let Some(metadata) = parse_tsx_module(source_path, source).metadata {
        let mut parsed_metadata = json!({
            "source_path": source_path,
            "source_kind": "metadata",
            "title": metadata.title,
            "title_kind": "literal",
            "title_default": null,
            "title_template": null,
            "title_absolute": null,
            "description": metadata.description,
            "canonical": metadata.canonical,
            "node_modules_required": false,
            "source_owned_metadata": true,
            "source_owned_title_template": false,
            "source_owned_title_absolute": false,
            "external_runtime_required": false,
            "external_runtime_executed": false,
        });
        attach_viewport_metadata(&mut parsed_metadata, viewport_metadata.as_ref());
        return Some(parsed_metadata);
    }
    if let Some(mut metadata) =
        safe_generate_metadata_source(source_path, source, route_params, search_params)
    {
        attach_viewport_metadata(&mut metadata, viewport_metadata.as_ref());
        return Some(metadata);
    }
    viewport_metadata
}

fn attach_viewport_metadata(target: &mut Value, viewport_metadata: Option<&Value>) {
    let Some(viewport) = viewport_metadata
        .and_then(|metadata| metadata.get("viewport"))
        .filter(|viewport| !viewport.is_null())
        .cloned()
    else {
        return;
    };
    let Some(object) = target.as_object_mut() else {
        return;
    };
    object.insert("viewport".to_string(), viewport);
    object.insert("source_owned_viewport_metadata".to_string(), json!(true));
    object.insert("full_next_viewport_runtime".to_string(), json!(false));
}

fn safe_static_metadata_source(source_path: &str, source: &str) -> Option<Value> {
    let object = static_metadata_object_literal(source)?;
    let route_params = BTreeMap::new();
    let search_params = BTreeMap::new();
    let mut request = RequestMetadataContext::new(
        &route_params,
        &search_params,
        BTreeMap::new(),
        BTreeMap::new(),
    );
    let fields = parse_generate_metadata_object(object, &mut request);
    if fields.recognized_field_count == 0 {
        return None;
    }
    let has_open_graph = fields.open_graph.is_some();
    let source_owned_title_template = fields.has_title_template();
    let source_owned_title_absolute = fields.has_title_absolute();
    Some(json!({
        "source_path": source_path,
        "source_kind": "metadata",
        "mode": "safe-static-metadata-literal-object",
        "title": fields.title,
        "title_kind": fields.title_kind,
        "title_default": fields.title_default,
        "title_template": fields.title_template,
        "title_absolute": fields.title_absolute,
        "description": fields.description,
        "canonical": fields.canonical,
        "openGraph": fields.open_graph,
        "node_modules_required": false,
        "source_owned_metadata": true,
        "source_owned_title_template": source_owned_title_template,
        "source_owned_title_absolute": source_owned_title_absolute,
        "source_owned_open_graph_metadata": has_open_graph,
        "external_runtime_required": false,
        "external_runtime_executed": false,
        "full_next_title_runtime": false,
        "full_next_open_graph_runtime": false,
    }))
}

fn safe_generate_metadata_source(
    source_path: &str,
    source: &str,
    route_params: &BTreeMap<String, String>,
    search_params: &BTreeMap<String, String>,
) -> Option<Value> {
    let function = generate_metadata_function_parts(source)?;
    let object = returned_object_literal(function.body)?;
    let mut request_root_aliases =
        collect_generate_metadata_request_root_aliases(function.parameters);
    collect_generate_metadata_request_object_aliases(
        function.parameters,
        function.body,
        &mut request_root_aliases,
    );
    let request_prop_aliases = collect_generate_metadata_request_aliases(
        function.body,
        &request_root_aliases,
        route_params,
        search_params,
    );
    let mut request = RequestMetadataContext::new(
        route_params,
        search_params,
        request_root_aliases,
        request_prop_aliases,
    );
    let fields = parse_generate_metadata_object(object, &mut request);
    if fields.recognized_field_count == 0 {
        return None;
    }
    let request_bound = request.request_bound();
    let has_open_graph = fields.open_graph.is_some();
    let source_owned_title_template = fields.has_title_template();
    let source_owned_title_absolute = fields.has_title_absolute();

    Some(json!({
        "source_path": source_path,
        "source_kind": "generateMetadata",
        "mode": if request_bound {
            "safe-generate-metadata-request-literal-return"
        } else {
            "safe-generate-metadata-literal-return"
        },
        "title": fields.title,
        "title_kind": fields.title_kind,
        "title_default": fields.title_default,
        "title_template": fields.title_template,
        "title_absolute": fields.title_absolute,
        "description": fields.description,
        "canonical": fields.canonical,
        "openGraph": fields.open_graph,
        "request_bound": request_bound,
        "request_prop_bindings": request.request_prop_bindings(),
        "request_prop_root_alias_bindings": request.request_prop_root_alias_bindings(),
        "request_prop_root_alias_count": request.request_prop_root_alias_count(),
        "request_prop_alias_bindings": request.request_prop_alias_bindings(),
        "request_prop_alias_count": request.request_prop_alias_count(),
        "supported_expressions": request.supported_expressions(),
        "supported_request_patterns": supported_request_metadata_patterns(),
        "unresolved_request_prop_bindings": request.unresolved_request_prop_bindings(),
        "node_modules_required": false,
        "arbitrary_request_code_execution": false,
        "source_owned_metadata": true,
        "source_owned_title_template": source_owned_title_template,
        "source_owned_title_absolute": source_owned_title_absolute,
        "source_owned_open_graph_metadata": has_open_graph,
        "external_runtime_required": false,
        "external_runtime_executed": false,
        "full_next_title_runtime": false,
        "full_next_open_graph_runtime": false,
        "limits": [
            "Reads safe literal fields returned directly from generateMetadata().",
            "Reads safe title.default, title.template, and title.absolute object fields.",
            "Reads alternates.canonical when it is a safe literal string.",
            "Reads safe openGraph title, description, url, siteName, and images fields.",
            "Reads params.*, searchParams.*, safe root aliases, safe optional chaining variants, and simple destructured aliases with quoted string defaults only from the DX App Router request map.",
            "Does not execute dynamic metadata code, destructuring defaults, stream metadata, openGraph image object expansion, read cookies or headers, or import the Next.js runtime."
        ],
    }))
}

fn viewport_metadata_source(
    source_path: &str,
    source: &str,
    route_params: &BTreeMap<String, String>,
    search_params: &BTreeMap<String, String>,
) -> Option<Value> {
    safe_static_viewport_source(source_path, source)
        .or_else(|| safe_generate_viewport_source(source_path, source, route_params, search_params))
}

fn safe_static_viewport_source(source_path: &str, source: &str) -> Option<Value> {
    let object = static_viewport_object_literal(source)?;
    let route_params = BTreeMap::new();
    let search_params = BTreeMap::new();
    let mut request = RequestMetadataContext::new(
        &route_params,
        &search_params,
        BTreeMap::new(),
        BTreeMap::new(),
    );
    let viewport = safe_viewport_metadata(object, &mut request)?;

    Some(json!({
        "source_path": source_path,
        "source_kind": "viewport",
        "mode": "safe-static-viewport-literal-object",
        "viewport": viewport,
        "request_bound": false,
        "node_modules_required": false,
        "arbitrary_request_code_execution": false,
        "source_owned_viewport_metadata": true,
        "external_runtime_required": false,
        "external_runtime_executed": false,
        "full_next_viewport_runtime": false,
    }))
}

fn safe_generate_viewport_source(
    source_path: &str,
    source: &str,
    route_params: &BTreeMap<String, String>,
    search_params: &BTreeMap<String, String>,
) -> Option<Value> {
    let function = generate_viewport_function_parts(source)?;
    let object = returned_object_literal(function.body)?;
    let mut request_root_aliases =
        collect_generate_metadata_request_root_aliases(function.parameters);
    collect_generate_metadata_request_object_aliases(
        function.parameters,
        function.body,
        &mut request_root_aliases,
    );
    let request_prop_aliases = collect_generate_metadata_request_aliases(
        function.body,
        &request_root_aliases,
        route_params,
        search_params,
    );
    let mut request = RequestMetadataContext::new(
        route_params,
        search_params,
        request_root_aliases,
        request_prop_aliases,
    );
    let viewport = safe_viewport_metadata(object, &mut request)?;
    let request_bound = request.request_bound();

    Some(json!({
        "source_path": source_path,
        "source_kind": "generateViewport",
        "mode": if request_bound {
            "safe-generate-viewport-request-literal-return"
        } else {
            "safe-generate-viewport-literal-return"
        },
        "viewport": viewport,
        "request_bound": request_bound,
        "request_prop_bindings": request.request_prop_bindings(),
        "request_prop_root_alias_bindings": request.request_prop_root_alias_bindings(),
        "request_prop_root_alias_count": request.request_prop_root_alias_count(),
        "request_prop_alias_bindings": request.request_prop_alias_bindings(),
        "request_prop_alias_count": request.request_prop_alias_count(),
        "supported_expressions": request.supported_expressions(),
        "supported_request_patterns": supported_request_metadata_patterns(),
        "unresolved_request_prop_bindings": request.unresolved_request_prop_bindings(),
        "node_modules_required": false,
        "arbitrary_request_code_execution": false,
        "source_owned_viewport_metadata": true,
        "external_runtime_required": false,
        "external_runtime_executed": false,
        "full_next_viewport_runtime": false,
        "limits": [
            "Reads safe literal fields returned directly from generateViewport().",
            "Reads safe viewport width, height, themeColor, colorScheme, interactiveWidget, scale, and userScalable fields.",
            "Reads params.*, searchParams.*, safe root aliases, safe optional chaining variants, and simple destructured aliases with quoted string defaults only from the DX App Router request map.",
            "Does not execute dynamic viewport code, parent ResolvingViewport, cookies, headers, or the Next.js runtime."
        ],
    }))
}

#[derive(Default)]
struct GenerateMetadataFields {
    title: Option<String>,
    title_kind: Option<&'static str>,
    title_default: Option<String>,
    title_template: Option<String>,
    title_absolute: Option<String>,
    description: Option<String>,
    canonical: Option<String>,
    open_graph: Option<Value>,
    recognized_field_count: usize,
}

impl GenerateMetadataFields {
    fn has_title_template(&self) -> bool {
        self.title_default.is_some() || self.title_template.is_some()
    }

    fn has_title_absolute(&self) -> bool {
        self.title_absolute.is_some()
    }
}

#[derive(Default)]
struct TitleMetadataFields {
    title: Option<String>,
    title_kind: Option<&'static str>,
    title_default: Option<String>,
    title_template: Option<String>,
    title_absolute: Option<String>,
}

fn parse_generate_metadata_object(
    source: &str,
    request: &mut RequestMetadataContext<'_>,
) -> GenerateMetadataFields {
    let mut fields = GenerateMetadataFields::default();
    for entry in split_top_level_entries(source) {
        let Some((key, value)) = entry.split_once(':') else {
            continue;
        };
        let key = clean_object_key(key);
        let value = value.trim();
        match key.as_str() {
            "title" => {
                if let Some(value) = safe_metadata_field_value(value, request) {
                    fields.recognized_field_count += 1;
                    fields.title = value;
                    fields.title_kind = Some("literal");
                } else if let Some(title_fields) = safe_title_metadata(value, request) {
                    fields.recognized_field_count += 1;
                    fields.title = title_fields.title;
                    fields.title_kind = title_fields.title_kind;
                    fields.title_default = title_fields.title_default;
                    fields.title_template = title_fields.title_template;
                    fields.title_absolute = title_fields.title_absolute;
                }
            }
            "description" => {
                if let Some(value) = safe_metadata_field_value(value, request) {
                    fields.recognized_field_count += 1;
                    fields.description = value;
                }
            }
            "canonical" => {
                if let Some(value) = safe_metadata_field_value(value, request) {
                    fields.recognized_field_count += 1;
                    fields.canonical = value;
                }
            }
            "alternates" => {
                if fields.canonical.is_none() {
                    if let Some(value) = strip_object_braces(value) {
                        if let Some(canonical) = alternates_canonical_literal(value, request) {
                            fields.recognized_field_count += 1;
                            fields.canonical = canonical;
                        }
                    }
                }
            }
            "openGraph" => {
                if let Some(open_graph) = safe_open_graph_metadata(value, request) {
                    fields.recognized_field_count += 1;
                    fields.open_graph = Some(open_graph);
                }
            }
            _ => {}
        }
    }
    fields
}

fn safe_metadata_field_value(
    value: &str,
    request: &mut RequestMetadataContext<'_>,
) -> Option<Option<String>> {
    let expression = safe_metadata_expression(value, request)?;
    request.record_expression(&expression);
    Some(expression.value)
}

fn safe_title_metadata(
    source: &str,
    request: &mut RequestMetadataContext<'_>,
) -> Option<TitleMetadataFields> {
    let source = strip_object_braces(source)?;
    let mut fields = TitleMetadataFields::default();
    let mut recognized = false;
    for entry in split_top_level_entries(source) {
        let Some((key, value)) = entry.split_once(':') else {
            continue;
        };
        let key = clean_object_key(key);
        let value = value.trim();
        match key.as_str() {
            "default" => {
                if let Some(value) = safe_metadata_field_value(value, request) {
                    recognized = true;
                    fields.title_default = value.clone();
                    if fields.title.is_none() {
                        fields.title = value;
                        fields.title_kind = Some("title-object-default");
                    }
                }
            }
            "template" => {
                if let Some(Some(value)) = safe_metadata_field_value(value, request) {
                    if value.contains("%s") {
                        recognized = true;
                        fields.title_template = Some(value);
                    }
                }
            }
            "absolute" => {
                if let Some(value) = safe_metadata_field_value(value, request) {
                    recognized = true;
                    fields.title_absolute = value.clone();
                    fields.title = value;
                    fields.title_kind = Some("title-object-absolute");
                }
            }
            _ => {}
        }
    }
    recognized.then_some(fields)
}

fn alternates_canonical_literal(
    source: &str,
    request: &mut RequestMetadataContext<'_>,
) -> Option<Option<String>> {
    split_top_level_entries(source)
        .into_iter()
        .find_map(|entry| {
            let (key, value) = entry.split_once(':')?;
            (clean_object_key(key) == "canonical")
                .then(|| {
                    let expression = safe_metadata_expression(value.trim(), request)?;
                    request.record_expression(&expression);
                    Some(expression.value)
                })
                .flatten()
        })
}

fn safe_open_graph_metadata(
    source: &str,
    request: &mut RequestMetadataContext<'_>,
) -> Option<Value> {
    let source = strip_object_braces(source)?;
    let mut fields = Map::new();
    for entry in split_top_level_entries(source) {
        let Some((key, value)) = entry.split_once(':') else {
            continue;
        };
        let key = clean_object_key(key);
        let value = value.trim();
        match key.as_str() {
            "title" | "description" | "url" | "siteName" => {
                if let Some(value) = safe_open_graph_field_value(value, request) {
                    fields.insert(key, value);
                }
            }
            "images" => {
                if let Some(value) = safe_open_graph_images(value, request) {
                    fields.insert(key, value);
                }
            }
            _ => {}
        }
    }
    (!fields.is_empty()).then_some(Value::Object(fields))
}

fn safe_open_graph_field_value(
    value: &str,
    request: &mut RequestMetadataContext<'_>,
) -> Option<Value> {
    safe_metadata_field_value(value, request)
        .map(|value| value.map(Value::String).unwrap_or(Value::Null))
}

fn safe_open_graph_images(value: &str, request: &mut RequestMetadataContext<'_>) -> Option<Value> {
    let value = clean_expression_tail(value);
    if value.starts_with('[') {
        let end = find_balanced_delimiter(value, 0, '[', ']')?;
        if !value[end + ']'.len_utf8()..].trim().is_empty() {
            return None;
        }
        let mut images = Vec::new();
        for entry in split_top_level_entries(&value['['.len_utf8()..end]) {
            let image = safe_metadata_field_value(entry, request)??;
            images.push(Value::String(image));
        }
        return Some(Value::Array(images));
    }
    safe_metadata_field_value(value, request).map(|value| {
        value
            .map(|image| Value::Array(vec![Value::String(image)]))
            .unwrap_or(Value::Null)
    })
}

fn safe_viewport_metadata(source: &str, request: &mut RequestMetadataContext<'_>) -> Option<Value> {
    let mut fields = Map::new();
    for entry in split_top_level_entries(source) {
        let Some((key, value)) = entry.split_once(':') else {
            continue;
        };
        let key = clean_object_key(key);
        let value = value.trim();
        match key.as_str() {
            "width" | "height" => {
                if let Some(value) = safe_viewport_string_or_number_value(value, request) {
                    fields.insert(key, value);
                }
            }
            "themeColor" | "colorScheme" | "interactiveWidget" => {
                if let Some(value) = safe_viewport_string_field_value(value, request) {
                    fields.insert(key, value);
                }
            }
            "initialScale" | "minimumScale" | "maximumScale" => {
                if let Some(value) = safe_viewport_number_field_value(value) {
                    fields.insert(key, value);
                }
            }
            "userScalable" => {
                if let Some(value) = safe_viewport_boolean_field_value(value) {
                    fields.insert(key, value);
                }
            }
            _ => {}
        }
    }
    (!fields.is_empty()).then_some(Value::Object(fields))
}

fn safe_viewport_string_or_number_value(
    value: &str,
    request: &mut RequestMetadataContext<'_>,
) -> Option<Value> {
    safe_viewport_string_field_value(value, request)
        .or_else(|| safe_viewport_number_field_value(value))
}

fn safe_viewport_string_field_value(
    value: &str,
    request: &mut RequestMetadataContext<'_>,
) -> Option<Value> {
    safe_metadata_field_value(value, request)
        .map(|value| value.map(Value::String).unwrap_or(Value::Null))
}

fn safe_viewport_number_field_value(value: &str) -> Option<Value> {
    let value = clean_expression_tail(value);
    let number = value.parse::<f64>().ok()?;
    if !number.is_finite() {
        return None;
    }
    Number::from_f64(number).map(Value::Number)
}

fn safe_viewport_boolean_field_value(value: &str) -> Option<Value> {
    match clean_expression_tail(value) {
        "true" => Some(Value::Bool(true)),
        "false" => Some(Value::Bool(false)),
        _ => None,
    }
}

fn static_metadata_object_literal(source: &str) -> Option<&str> {
    static_object_literal(source, "metadata")
}

fn static_viewport_object_literal(source: &str) -> Option<&str> {
    static_object_literal(source, "viewport")
}

fn static_object_literal<'a>(source: &'a str, marker: &str) -> Option<&'a str> {
    let mut search_start = 0usize;
    while let Some(offset) = source[search_start..].find(marker) {
        let name_start = search_start + offset;
        let name_end = name_start + marker.len();
        search_start = name_end;

        if source[..name_start]
            .chars()
            .last()
            .is_some_and(is_identifier_character)
            || source[name_end..]
                .chars()
                .next()
                .is_some_and(is_identifier_character)
        {
            continue;
        }

        let declaration_prefix = source[..name_start].trim_end();
        if !has_export_const_declaration_prefix(declaration_prefix) {
            continue;
        }

        let Some(equals_offset) = source[name_end..].find('=') else {
            continue;
        };
        let cursor = skip_whitespace(source, name_end + equals_offset + 1);
        if !source[cursor..].starts_with('{') {
            continue;
        }
        let object_end = find_balanced_delimiter(source, cursor, '{', '}')?;
        return Some(&source[cursor + '{'.len_utf8()..object_end]);
    }
    None
}

fn has_export_const_declaration_prefix(declaration_prefix: &str) -> bool {
    let declaration_tokens = declaration_prefix.split_whitespace().collect::<Vec<_>>();
    declaration_tokens.ends_with(&["export", "const"])
}

fn has_export_function_declaration_prefix(declaration_prefix: &str) -> bool {
    let declaration_tokens = declaration_prefix.split_whitespace().collect::<Vec<_>>();
    declaration_tokens.ends_with(&["export"]) || declaration_tokens.ends_with(&["export", "async"])
}

struct GenerateMetadataFunctionParts<'a> {
    parameters: &'a str,
    body: &'a str,
}

fn generate_metadata_function_parts(source: &str) -> Option<GenerateMetadataFunctionParts<'_>> {
    generate_exported_named_function_parts(source, "generateMetadata")
        .or_else(|| generate_metadata_const_arrow_parts(source))
}

fn generate_viewport_function_parts(source: &str) -> Option<GenerateMetadataFunctionParts<'_>> {
    generate_exported_named_function_parts(source, "generateViewport")
        .or_else(|| generate_const_arrow_function_parts(source, "generateViewport"))
}

fn generate_metadata_const_arrow_parts(source: &str) -> Option<GenerateMetadataFunctionParts<'_>> {
    generate_const_arrow_function_parts(source, "generateMetadata")
}

fn generate_exported_named_function_parts<'source>(
    source: &'source str,
    function_name: &str,
) -> Option<GenerateMetadataFunctionParts<'source>> {
    let function_marker = format!("function {function_name}");
    let mut search_start = 0usize;
    while let Some(offset) = source[search_start..].find(&function_marker) {
        let function_index = search_start + offset;
        let name_end = function_index + function_marker.len();
        search_start = name_end;
        if source[..function_index]
            .chars()
            .last()
            .is_some_and(is_identifier_character)
            || source[name_end..]
                .chars()
                .next()
                .is_some_and(is_identifier_character)
            || !has_export_function_declaration_prefix(source[..function_index].trim_end())
        {
            continue;
        }
        let parameters_start = source[function_index..]
            .find('(')
            .map(|offset| function_index + offset)?;
        let parameters_end = find_balanced_delimiter(source, parameters_start, '(', ')')?;
        let block_start = source[parameters_end..]
            .find('{')
            .map(|offset| parameters_end + offset)?;
        let block_end = find_balanced_delimiter(source, block_start, '{', '}')?;
        return Some(GenerateMetadataFunctionParts {
            parameters: &source[parameters_start + 1..parameters_end],
            body: &source[block_start + 1..block_end],
        });
    }
    None
}

fn generate_const_arrow_function_parts<'source>(
    source: &'source str,
    marker: &str,
) -> Option<GenerateMetadataFunctionParts<'source>> {
    let mut search_start = 0usize;
    while let Some(offset) = source[search_start..].find(marker) {
        let name_start = search_start + offset;
        let name_end = name_start + marker.len();
        search_start = name_end;

        if source[..name_start]
            .chars()
            .last()
            .is_some_and(is_identifier_character)
            || source[name_end..]
                .chars()
                .next()
                .is_some_and(is_identifier_character)
        {
            continue;
        }

        let declaration_prefix = source[..name_start].trim_end();
        if !has_export_const_declaration_prefix(declaration_prefix) {
            continue;
        }

        let Some(equals_offset) = source[name_end..].find('=') else {
            continue;
        };
        let mut cursor = skip_whitespace(source, name_end + equals_offset + 1);
        if source[cursor..].starts_with("async") {
            let async_end = cursor + "async".len();
            if source[async_end..]
                .chars()
                .next()
                .is_some_and(is_identifier_character)
            {
                continue;
            }
            cursor = skip_whitespace(source, async_end);
        }

        cursor = skip_const_arrow_type_parameters(source, cursor)?;
        let Some((parameters, parameters_end)) = const_arrow_parameters(source, cursor) else {
            continue;
        };
        cursor = skip_whitespace(source, parameters_end);
        cursor = skip_const_arrow_return_type_annotation(source, cursor)?;
        if !source[cursor..].starts_with("=>") {
            continue;
        }
        cursor = skip_whitespace(source, cursor + "=>".len());
        if !source[cursor..].starts_with('{') {
            if let Some((body, _end)) = const_arrow_expression_body(source, cursor) {
                return Some(GenerateMetadataFunctionParts { parameters, body });
            }
            continue;
        }

        let block_start = cursor;
        let block_end = find_balanced_delimiter(source, block_start, '{', '}')?;
        return Some(GenerateMetadataFunctionParts {
            parameters,
            body: &source[block_start + 1..block_end],
        });
    }
    None
}

fn skip_const_arrow_type_parameters(source: &str, cursor: usize) -> Option<usize> {
    let cursor = skip_whitespace(source, cursor);
    if !source[cursor..].starts_with('<') {
        return Some(cursor);
    }
    let type_parameters_end = find_type_parameter_list_end(source, cursor)?;
    let after_type_parameters = skip_whitespace(source, type_parameters_end + '>'.len_utf8());
    if source[after_type_parameters..].starts_with('(')
        || source[after_type_parameters..]
            .chars()
            .next()
            .is_some_and(|character| {
                character.is_ascii_alphabetic() || character == '_' || character == '$'
            })
    {
        return Some(after_type_parameters);
    }
    None
}

fn find_type_parameter_list_end(source: &str, mut cursor: usize) -> Option<usize> {
    if !source[cursor..].starts_with('<') {
        return None;
    }
    let mut quote = None;
    let mut paren_depth = 0usize;
    let mut brace_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut angle_depth = 0usize;
    while cursor < source.len() {
        let character = source[cursor..].chars().next()?;
        if let Some(active_quote) = quote {
            if character == active_quote {
                quote = None;
            } else if character == '\\' {
                cursor += character.len_utf8();
                if cursor < source.len() {
                    cursor += source[cursor..].chars().next()?.len_utf8();
                    continue;
                }
            }
            cursor += character.len_utf8();
            continue;
        }
        match character {
            '"' | '\'' | '`' => quote = Some(character),
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '{' => brace_depth += 1,
            '}' => brace_depth = brace_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '<' if paren_depth == 0 && brace_depth == 0 && bracket_depth == 0 => angle_depth += 1,
            '>' if paren_depth == 0 && brace_depth == 0 && bracket_depth == 0 => {
                angle_depth = angle_depth.saturating_sub(1);
                if angle_depth == 0 {
                    return Some(cursor);
                }
            }
            _ => {}
        }
        cursor += character.len_utf8();
    }
    None
}

fn const_arrow_parameters(source: &str, cursor: usize) -> Option<(&str, usize)> {
    let cursor = skip_whitespace(source, cursor);
    if source[cursor..].starts_with('(') {
        let parameters_end = find_balanced_delimiter(source, cursor, '(', ')')?;
        return Some((&source[cursor + 1..parameters_end], parameters_end + 1));
    }
    const_arrow_bare_parameter_parts(source, cursor)
}

fn const_arrow_bare_parameter_parts(source: &str, cursor: usize) -> Option<(&str, usize)> {
    let cursor = skip_whitespace(source, cursor);
    let first = source[cursor..].chars().next()?;
    if !(first.is_ascii_alphabetic() || first == '_' || first == '$') {
        return None;
    }
    let mut end = cursor + first.len_utf8();
    while end < source.len() {
        let character = source[end..].chars().next()?;
        if !is_identifier_character(character) {
            break;
        }
        end += character.len_utf8();
    }
    let after_parameter = skip_whitespace(source, end);
    if !source[after_parameter..].starts_with("=>") {
        return None;
    }
    Some((&source[cursor..end], end))
}

fn skip_const_arrow_return_type_annotation(source: &str, cursor: usize) -> Option<usize> {
    let cursor = skip_whitespace(source, cursor);
    if source[cursor..].starts_with("=>") {
        return Some(cursor);
    }
    if !source[cursor..].starts_with(':') {
        return Some(cursor);
    }
    find_top_level_arrow(source, cursor + ':'.len_utf8())
}

fn find_top_level_arrow(source: &str, mut cursor: usize) -> Option<usize> {
    let mut quote = None;
    let mut paren_depth = 0usize;
    let mut brace_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut angle_depth = 0usize;
    while cursor < source.len() {
        if quote.is_none()
            && paren_depth == 0
            && brace_depth == 0
            && bracket_depth == 0
            && angle_depth == 0
            && source[cursor..].starts_with("=>")
        {
            return Some(cursor);
        }
        let character = source[cursor..].chars().next()?;
        if let Some(active_quote) = quote {
            if character == active_quote {
                quote = None;
            } else if character == '\\' {
                cursor += character.len_utf8();
                if cursor < source.len() {
                    cursor += source[cursor..].chars().next()?.len_utf8();
                    continue;
                }
            }
            cursor += character.len_utf8();
            continue;
        }
        match character {
            '"' | '\'' | '`' => quote = Some(character),
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '{' => brace_depth += 1,
            '}' => brace_depth = brace_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '<' => angle_depth += 1,
            '>' => angle_depth = angle_depth.saturating_sub(1),
            _ => {}
        }
        cursor += character.len_utf8();
    }
    None
}

fn const_arrow_expression_body(source: &str, cursor: usize) -> Option<(&str, usize)> {
    let cursor = skip_whitespace(source, cursor);
    if !source[cursor..].starts_with('(') {
        return None;
    }
    let expression_end = find_balanced_delimiter(source, cursor, '(', ')')?;
    let body = &source[cursor + 1..expression_end];
    arrow_expression_body_object(body)?;
    Some((body, expression_end + 1))
}

fn returned_object_literal(function_body: &str) -> Option<&str> {
    let Some(return_index) = function_body.find("return") else {
        return arrow_expression_body_object(function_body);
    };
    let object_start = function_body[return_index..]
        .find('{')
        .map(|offset| return_index + offset)?;
    let object_end = find_balanced_delimiter(function_body, object_start, '{', '}')?;
    Some(&function_body[object_start + 1..object_end])
}

fn arrow_expression_body_object(source: &str) -> Option<&str> {
    let source = source.trim().trim_end_matches(';').trim();
    if !source.starts_with('{') {
        return None;
    }
    let object_end = find_balanced_delimiter(source, 0, '{', '}')?;
    Some(&source[1..object_end])
}

fn strip_object_braces(source: &str) -> Option<&str> {
    let source = source.trim();
    let object_start = source.find('{')?;
    let object_end = find_balanced_delimiter(source, object_start, '{', '}')?;
    Some(&source[object_start + 1..object_end])
}

fn safe_metadata_expression(
    source: &str,
    request: &RequestMetadataContext<'_>,
) -> Option<MetadataExpression> {
    let source = clean_expression_tail(source);
    safe_metadata_template_literal(source, request)
        .or_else(|| safe_static_string_literal(source).map(MetadataExpression::literal))
        .or_else(|| safe_request_value_expression(source, request))
}

fn safe_static_string_literal(source: &str) -> Option<String> {
    let source = source
        .trim()
        .trim_end_matches(',')
        .trim_end_matches(';')
        .trim();
    let quote = source.chars().next()?;
    if !matches!(quote, '"' | '\'' | '`') {
        return None;
    }
    let (value, end) = parse_quoted_value(source, quote)?;
    if quote == '`' && value.contains("${") {
        return None;
    }
    source[end..].trim().is_empty().then_some(value)
}

fn safe_metadata_template_literal(
    source: &str,
    request: &RequestMetadataContext<'_>,
) -> Option<MetadataExpression> {
    let source = clean_expression_tail(source);
    if !source.starts_with('`') {
        return None;
    }

    let mut output = String::new();
    let mut cursor = '`'.len_utf8();
    let mut escaped = false;
    let mut result = MetadataExpression::default();
    let mut interpolation_seen = false;
    while cursor < source.len() {
        let character = source[cursor..].chars().next()?;
        if escaped {
            output.push(match character {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                escaped => escaped,
            });
            escaped = false;
            cursor += character.len_utf8();
            continue;
        }
        if character == '\\' {
            escaped = true;
            cursor += character.len_utf8();
            continue;
        }
        if character == '`' {
            let rest = &source[cursor + character.len_utf8()..];
            if !rest.trim().is_empty() {
                return None;
            }
            if interpolation_seen {
                result.supported_expressions.insert(source.to_string());
            }
            result.value = result
                .unresolved_request_prop_bindings
                .is_empty()
                .then_some(output);
            return Some(result);
        }
        if character == '$' {
            let next_cursor = cursor + character.len_utf8();
            if source[next_cursor..].starts_with('{') {
                let expression_start = next_cursor + '{'.len_utf8();
                let expression_end = find_balanced_delimiter(source, next_cursor, '{', '}')?;
                let expression_source = source[expression_start..expression_end].trim();
                let expression = safe_request_value_expression(expression_source, request)?;
                interpolation_seen = true;
                if let Some(value) = &expression.value {
                    output.push_str(value);
                }
                result.merge(expression);
                cursor = expression_end + '}'.len_utf8();
                continue;
            }
        }
        output.push(character);
        cursor += character.len_utf8();
    }

    interpolation_seen.then_some(result)
}

fn safe_request_value_expression(
    source: &str,
    request: &RequestMetadataContext<'_>,
) -> Option<MetadataExpression> {
    safe_request_prop_access(source, request)
        .or_else(|| safe_request_prop_alias_access(source, request))
}

fn safe_request_prop_access(
    source: &str,
    request: &RequestMetadataContext<'_>,
) -> Option<MetadataExpression> {
    let source = clean_expression_tail(source);
    let (root, key, expression, root_alias) = parse_request_prop_access(source)
        .map(|(root, key, expression)| (root, key, expression, None))
        .or_else(|| parse_request_root_alias_prop_access(source, request))?;
    let values = match root {
        "params" => request.route_params,
        "searchParams" => request.search_params,
        _ => return None,
    };
    let binding = format!("{root}.{key}");
    let value = values.get(&key).cloned();
    let mut result = MetadataExpression {
        value: value.clone(),
        ..MetadataExpression::default()
    };
    result.request_prop_bindings.insert(binding.clone());
    result.supported_expressions.insert(expression);
    if let Some(root_alias) = root_alias {
        result.request_prop_root_alias_bindings.insert(root_alias);
    }
    if value.is_none() {
        result.unresolved_request_prop_bindings.insert(binding);
    }
    Some(result)
}

fn parse_request_root_alias_prop_access(
    source: &str,
    request: &RequestMetadataContext<'_>,
) -> Option<(&'static str, String, String, Option<String>)> {
    for alias in request.request_prop_root_aliases.values() {
        let Some(normalized) =
            rewrite_request_root_alias(source, &alias.alias, alias.canonical_root)
        else {
            continue;
        };
        if let Some((root, key, _)) = parse_request_prop_access(&normalized) {
            return Some((root, key, source.to_string(), Some(alias.alias.clone())));
        }
    }
    None
}

fn rewrite_request_root_alias(source: &str, alias: &str, canonical_root: &str) -> Option<String> {
    let source = clean_expression_tail(source);
    if let Some(rest) = source.strip_prefix(alias) {
        if rest.chars().next().is_some_and(is_identifier_character) {
            return None;
        }
        return Some(format!("{canonical_root}{rest}"));
    }

    let rest = source.strip_prefix("(await")?.trim_start();
    let rest = rest.strip_prefix(alias)?;
    if rest.chars().next().is_some_and(is_identifier_character) {
        return None;
    }
    Some(format!("(await {canonical_root}{rest}"))
}

fn safe_request_prop_alias_access(
    source: &str,
    request: &RequestMetadataContext<'_>,
) -> Option<MetadataExpression> {
    let alias_name = clean_expression_tail(source);
    if !is_simple_request_identifier(alias_name) {
        return None;
    }
    let alias = request.request_prop_aliases.get(alias_name)?;
    let mut result = MetadataExpression {
        value: alias.value.clone(),
        ..MetadataExpression::default()
    };
    result
        .request_prop_bindings
        .insert(alias.canonical_name.clone());
    result.supported_expressions.insert(alias_name.to_string());
    result
        .supported_expressions
        .insert(alias.expression.clone());
    result
        .request_prop_alias_bindings
        .insert(alias.alias.clone());
    if alias.value.is_none() {
        result
            .unresolved_request_prop_bindings
            .insert(alias.canonical_name.clone());
    }
    Some(result)
}

fn collect_generate_metadata_request_root_aliases(
    parameters: &str,
) -> BTreeMap<String, MetadataRequestRootAlias> {
    let mut aliases = BTreeMap::new();
    let Some(parameters) = strip_object_braces(parameters) else {
        return aliases;
    };
    for part in split_top_level_entries(parameters) {
        let Some(alias) = metadata_request_root_alias(part) else {
            continue;
        };
        aliases.entry(alias.alias.clone()).or_insert(alias);
    }
    aliases
}

fn metadata_request_root_alias(part: &str) -> Option<MetadataRequestRootAlias> {
    let part = part.trim();
    if part.is_empty() || part.starts_with("...") || part.contains('=') {
        return None;
    }
    let (root, alias) = part.split_once(':')?;
    let canonical_root = metadata_canonical_request_root(root.trim())?;
    let alias = alias.trim();
    if !is_simple_request_identifier(alias) || alias == canonical_root {
        return None;
    }
    Some(MetadataRequestRootAlias {
        alias: alias.to_string(),
        canonical_root,
        expression: format!("generateMetadata({{ {canonical_root}: {alias} }})"),
    })
}

fn collect_generate_metadata_request_object_aliases(
    parameters: &str,
    function_body: &str,
    aliases: &mut BTreeMap<String, MetadataRequestRootAlias>,
) {
    let parameter = simple_parameter_binding_name(parameters);
    if !is_simple_request_identifier(parameter) {
        return;
    }

    for statement in statement_candidates(function_body) {
        let Some(target) = safe_request_prop_assignment_target(statement, parameter) else {
            continue;
        };
        let left = &statement[..target.equals_index];
        if !is_variable_destructuring_declaration(left) {
            continue;
        }
        let Some(start) = left.rfind('{') else {
            continue;
        };
        let Some(end) = left.rfind('}') else {
            continue;
        };
        if start >= end {
            continue;
        }

        for part in split_top_level_entries(&left[start + 1..end]) {
            let Some(alias) = metadata_request_object_root_alias(part, &target.rhs_expression)
            else {
                continue;
            };
            aliases.entry(alias.alias.clone()).or_insert(alias);
        }
    }
}

fn metadata_request_object_root_alias(
    part: &str,
    rhs_expression: &str,
) -> Option<MetadataRequestRootAlias> {
    let part = part.trim();
    if part.is_empty() || part.starts_with("...") || part.contains('=') {
        return None;
    }

    let (root, alias) = part
        .split_once(':')
        .map(|(root, alias)| (root.trim(), alias.trim()))
        .unwrap_or((part, part));
    let canonical_root = metadata_canonical_request_root(root)?;
    if !is_simple_request_identifier(alias) || alias == canonical_root {
        return None;
    }

    Some(MetadataRequestRootAlias {
        alias: alias.to_string(),
        canonical_root,
        expression: format!("const {{ {part} }} = {rhs_expression}"),
    })
}

fn metadata_canonical_request_root(root: &str) -> Option<&'static str> {
    match root {
        "params" => Some("params"),
        "searchParams" => Some("searchParams"),
        _ => None,
    }
}

fn collect_generate_metadata_request_aliases(
    function_body: &str,
    request_root_aliases: &BTreeMap<String, MetadataRequestRootAlias>,
    route_params: &BTreeMap<String, String>,
    search_params: &BTreeMap<String, String>,
) -> BTreeMap<String, MetadataRequestPropAlias> {
    let mut aliases = BTreeMap::new();
    collect_generate_metadata_destructured_request_aliases(
        function_body,
        "params",
        "params",
        route_params,
        "generate-metadata-route-param-alias",
        &mut aliases,
    );
    collect_generate_metadata_destructured_request_aliases(
        function_body,
        "searchParams",
        "searchParams",
        search_params,
        "generate-metadata-search-param-alias",
        &mut aliases,
    );
    for root_alias in request_root_aliases.values() {
        match root_alias.canonical_root {
            "params" => collect_generate_metadata_destructured_request_aliases(
                function_body,
                &root_alias.alias,
                "params",
                route_params,
                "generate-metadata-route-param-alias",
                &mut aliases,
            ),
            "searchParams" => collect_generate_metadata_destructured_request_aliases(
                function_body,
                &root_alias.alias,
                "searchParams",
                search_params,
                "generate-metadata-search-param-alias",
                &mut aliases,
            ),
            _ => {}
        }
    }
    aliases
}

fn collect_generate_metadata_destructured_request_aliases(
    source: &str,
    rhs_object_name: &str,
    canonical_object_name: &'static str,
    values: &BTreeMap<String, String>,
    source_kind: &'static str,
    aliases: &mut BTreeMap<String, MetadataRequestPropAlias>,
) {
    for statement in statement_candidates(source) {
        let Some(target) = safe_request_prop_assignment_target(statement, rhs_object_name) else {
            continue;
        };
        let left = &statement[..target.equals_index];
        if !is_variable_destructuring_declaration(left) {
            continue;
        }
        let Some(start) = left.rfind('{') else {
            continue;
        };
        let Some(end) = left.rfind('}') else {
            continue;
        };
        if start >= end {
            continue;
        }
        let destructured = &left[start + 1..end];
        for part in split_top_level_entries(destructured) {
            let Some(alias) = metadata_destructured_request_prop_alias(part) else {
                continue;
            };
            let canonical_name = format!("{}.{}", canonical_object_name, alias.prop_name);
            let expression = format!("const {{ {part} }} = {}", target.rhs_expression);
            aliases
                .entry(alias.alias.to_string())
                .or_insert_with(|| MetadataRequestPropAlias {
                    alias: alias.alias.to_string(),
                    canonical_name,
                    expression,
                    value: values
                        .get(alias.prop_name)
                        .cloned()
                        .or_else(|| alias.fallback_literal.clone()),
                    source_kind,
                });
        }
    }
}

fn statement_candidates(source: &str) -> Vec<&str> {
    let mut statements = Vec::new();
    for statement in source.split(';') {
        let statement = statement.trim();
        if statement.is_empty() {
            continue;
        }
        for line in statement.lines() {
            let line = line.trim();
            if !line.is_empty() {
                statements.push(line);
            }
        }
    }
    statements
}

struct MetadataRequestPropAssignmentTarget {
    equals_index: usize,
    rhs_expression: String,
}

fn safe_request_prop_assignment_target(
    statement: &str,
    object_name: &str,
) -> Option<MetadataRequestPropAssignmentTarget> {
    for (index, character) in statement.char_indices() {
        if character != '=' {
            continue;
        }
        let after = statement[index + 1..].trim_start();
        if let Some(rhs_expression) = request_prop_assignment_rhs(after, object_name) {
            return Some(MetadataRequestPropAssignmentTarget {
                equals_index: index,
                rhs_expression,
            });
        }
    }
    None
}

fn request_prop_assignment_rhs(after: &str, object_name: &str) -> Option<String> {
    let after = after.trim_start();
    let (async_prefix, rest) = after
        .strip_prefix("await ")
        .map(|rest| (true, rest.trim_start()))
        .unwrap_or((false, after));
    let rest = rest.strip_prefix(object_name)?;
    if rest.chars().next().is_some_and(is_identifier_character) {
        return None;
    }
    Some(if async_prefix {
        format!("await {object_name}")
    } else {
        object_name.to_string()
    })
}

fn is_variable_destructuring_declaration(left: &str) -> bool {
    let left = left.trim_start();
    left.starts_with("const ") || left.starts_with("let ") || left.starts_with("var ")
}

struct MetadataDestructuredRequestPropAlias<'a> {
    prop_name: &'a str,
    alias: &'a str,
    fallback_literal: Option<String>,
}

fn metadata_destructured_request_prop_alias(
    part: &str,
) -> Option<MetadataDestructuredRequestPropAlias<'_>> {
    let part = part.trim();
    if part.is_empty() || part.starts_with("...") {
        return None;
    }
    let (part, fallback_literal) = metadata_destructured_part_and_default(part);
    if let Some((prop_name, alias)) = part.split_once(':') {
        let prop_name = prop_name.trim();
        let alias = alias.trim();
        if is_simple_request_identifier(prop_name) && is_simple_request_identifier(alias) {
            return Some(MetadataDestructuredRequestPropAlias {
                prop_name,
                alias,
                fallback_literal,
            });
        }
        return None;
    }
    if is_simple_request_identifier(part) {
        Some(MetadataDestructuredRequestPropAlias {
            prop_name: part,
            alias: part,
            fallback_literal,
        })
    } else {
        None
    }
}

fn metadata_destructured_part_and_default(part: &str) -> (&str, Option<String>) {
    if let Some((binding, fallback)) = part.split_once('=') {
        (
            binding.trim(),
            quoted_request_prop_default_literal(fallback.trim()),
        )
    } else {
        (part.trim(), None)
    }
}

fn quoted_request_prop_default_literal(expression: &str) -> Option<String> {
    let expression = expression.trim();
    let quote = expression
        .chars()
        .next()
        .filter(|character| matches!(character, '"' | '\'' | '`'))?;
    let (value, end) = parse_quoted_value(expression, quote)?;
    if quote == '`' && value.contains("${") {
        return None;
    }
    expression[end..].trim().is_empty().then_some(value)
}

fn parse_request_prop_access(source: &str) -> Option<(&'static str, String, String)> {
    parse_dot_request_prop_access(source, "params")
        .or_else(|| parse_bracket_request_prop_access(source, "params"))
        .or_else(|| parse_optional_request_prop_access(source, "params"))
        .or_else(|| parse_dot_request_prop_access(source, "searchParams"))
        .or_else(|| parse_bracket_request_prop_access(source, "searchParams"))
        .or_else(|| parse_optional_request_prop_access(source, "searchParams"))
}

fn parse_dot_request_prop_access(
    source: &str,
    root: &'static str,
) -> Option<(&'static str, String, String)> {
    let rest = source.strip_prefix(root)?.strip_prefix('.')?;
    let key = rest
        .chars()
        .take_while(|character| {
            character.is_ascii_alphanumeric() || *character == '_' || *character == '-'
        })
        .collect::<String>();
    if !is_safe_request_key(&key) || !rest[key.len()..].trim().is_empty() {
        return None;
    }
    Some((root, key.clone(), format!("{root}.{key}")))
}

fn parse_bracket_request_prop_access(
    source: &str,
    root: &'static str,
) -> Option<(&'static str, String, String)> {
    let rest = source.strip_prefix(root)?.strip_prefix('[')?.trim_start();
    let quote = rest
        .chars()
        .next()
        .filter(|quote| *quote == '"' || *quote == '\'')?;
    let (key, end) = parse_quoted_value(rest, quote)?;
    if !is_safe_request_key(&key) {
        return None;
    }
    let rest = rest[end..].trim_start();
    let rest = rest.strip_prefix(']')?.trim();
    if !rest.is_empty() {
        return None;
    }
    Some((root, key.clone(), format!("{root}[\"{key}\"]")))
}

fn parse_optional_request_prop_access(
    source: &str,
    root: &'static str,
) -> Option<(&'static str, String, String)> {
    let rest = parse_request_prop_access_rest(source, root)?;
    let rest = rest.strip_prefix("?.")?;
    let key =
        parse_request_prop_member_key(rest).or_else(|| parse_request_prop_bracket_key(rest))?;
    Some((root, key, source.to_string()))
}

fn parse_request_prop_access_rest<'a>(source: &'a str, root: &str) -> Option<&'a str> {
    parse_direct_request_prop_access_rest(source, root)
        .or_else(|| parse_awaited_request_prop_access_rest(source, root))
}

fn parse_direct_request_prop_access_rest<'a>(source: &'a str, root: &str) -> Option<&'a str> {
    let rest = source.strip_prefix(root)?;
    if rest.chars().next().is_some_and(is_identifier_character) {
        return None;
    }
    Some(rest.trim_start())
}

fn parse_awaited_request_prop_access_rest<'a>(source: &'a str, root: &str) -> Option<&'a str> {
    let rest = source.strip_prefix("(await")?.trim_start();
    let rest = rest.strip_prefix(root)?.trim_start();
    let rest = rest.strip_prefix(')')?;
    Some(rest.trim_start())
}

fn parse_request_prop_member_key(source: &str) -> Option<String> {
    let key = source
        .chars()
        .take_while(|character| {
            character.is_ascii_alphanumeric() || *character == '_' || *character == '-'
        })
        .collect::<String>();
    if !is_safe_request_key(&key) || !source[key.len()..].trim().is_empty() {
        return None;
    }
    Some(key)
}

fn parse_request_prop_bracket_key(source: &str) -> Option<String> {
    let rest = source.strip_prefix('[')?.trim_start();
    let quote = rest
        .chars()
        .next()
        .filter(|quote| *quote == '"' || *quote == '\'')?;
    let (key, end) = parse_quoted_value(rest, quote)?;
    if !is_safe_request_key(&key) {
        return None;
    }
    let rest = rest[end..].trim_start();
    let rest = rest.strip_prefix(']')?.trim();
    rest.is_empty().then_some(key)
}

fn is_identifier_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_' || character == '$'
}

fn is_simple_request_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_' || first == '$') {
        return false;
    }
    chars.all(is_identifier_character)
}

fn is_safe_request_key(key: &str) -> bool {
    !key.is_empty()
        && key.chars().all(|character| {
            character.is_ascii_alphanumeric() || character == '_' || character == '-'
        })
}

fn clean_expression_tail(source: &str) -> &str {
    source
        .trim()
        .trim_end_matches(',')
        .trim_end_matches(';')
        .trim()
}

fn simple_parameter_binding_name(source: &str) -> &str {
    let source = clean_expression_tail(source);
    let Some(colon_index) = top_level_type_annotation_colon(source) else {
        return source;
    };
    clean_expression_tail(&source[..colon_index])
}

fn top_level_type_annotation_colon(source: &str) -> Option<usize> {
    let mut cursor = 0usize;
    let mut quote = None;
    let mut paren_depth = 0usize;
    let mut brace_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut angle_depth = 0usize;
    while cursor < source.len() {
        let character = source[cursor..].chars().next()?;
        if let Some(active_quote) = quote {
            if character == active_quote {
                quote = None;
            } else if character == '\\' {
                cursor += character.len_utf8();
                if cursor < source.len() {
                    cursor += source[cursor..].chars().next()?.len_utf8();
                    continue;
                }
            }
            cursor += character.len_utf8();
            continue;
        }
        match character {
            '"' | '\'' | '`' => quote = Some(character),
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '{' => brace_depth += 1,
            '}' => brace_depth = brace_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '<' => angle_depth += 1,
            '>' => angle_depth = angle_depth.saturating_sub(1),
            ':' if paren_depth == 0
                && brace_depth == 0
                && bracket_depth == 0
                && angle_depth == 0 =>
            {
                return Some(cursor);
            }
            _ => {}
        }
        cursor += character.len_utf8();
    }
    None
}

fn skip_whitespace(source: &str, mut cursor: usize) -> usize {
    while cursor < source.len() {
        let Some(character) = source[cursor..].chars().next() else {
            break;
        };
        if !character.is_whitespace() {
            break;
        }
        cursor += character.len_utf8();
    }
    cursor
}

fn parse_quoted_value(source: &str, quote: char) -> Option<(String, usize)> {
    let mut value = String::new();
    let mut cursor = quote.len_utf8();
    let mut escaped = false;
    while cursor < source.len() {
        let character = source[cursor..].chars().next()?;
        cursor += character.len_utf8();
        if escaped {
            value.push(match character {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                escaped => escaped,
            });
            escaped = false;
            continue;
        }
        if character == '\\' {
            escaped = true;
            continue;
        }
        if character == quote {
            return Some((value, cursor));
        }
        value.push(character);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_request_props_resolve_safe_optional_chaining_reads() {
        let route_params = BTreeMap::from([("slug".to_string(), "post-1".to_string())]);
        let search_params = BTreeMap::from([("preview".to_string(), "draft".to_string())]);
        let mut request = RequestMetadataContext::new(
            &route_params,
            &search_params,
            BTreeMap::new(),
            BTreeMap::new(),
        );

        assert_eq!(
            safe_metadata_field_value("params?.slug", &mut request)
                .flatten()
                .as_deref(),
            Some("post-1")
        );
        assert_eq!(
            safe_metadata_field_value(r#"params?.["slug"]"#, &mut request)
                .flatten()
                .as_deref(),
            Some("post-1")
        );
        assert_eq!(
            safe_metadata_field_value("(await searchParams)?.preview", &mut request)
                .flatten()
                .as_deref(),
            Some("draft")
        );
        assert_eq!(
            safe_metadata_field_value(
                "`/blog/${params?.slug}?preview=${(await searchParams)?.preview}`",
                &mut request,
            )
            .flatten()
            .as_deref(),
            Some("/blog/post-1?preview=draft")
        );
        assert_eq!(
            safe_metadata_field_value("params?.missing", &mut request),
            Some(None)
        );
        assert!(safe_request_prop_access("paramsExtra?.slug", &request).is_none());
        assert!(safe_request_prop_access("params?.[dynamic]", &request).is_none());
        assert!(
            request
                .supported_expressions()
                .iter()
                .any(|expression| expression == "params?.slug")
        );
        assert!(
            request
                .unresolved_request_prop_bindings()
                .iter()
                .any(|binding| binding == "params.missing")
        );
    }

    #[test]
    fn metadata_request_prop_alias_defaults_resolve_quoted_strings() {
        let route_params = BTreeMap::new();
        let search_params = BTreeMap::new();
        let source = r#"
            export async function generateMetadata({ params, searchParams }) {
                const { slug = "latest" } = params;
                const { preview: previewMode = "off" } = searchParams;
                return {
                    title: `${slug}:${previewMode}`,
                    canonical: `/blog/${slug}?preview=${previewMode}`,
                };
            }
        "#;

        let metadata = safe_generate_metadata_source(
            "app/blog/[slug]/page.tsx",
            source,
            &route_params,
            &search_params,
        )
        .expect("safe generateMetadata defaults should produce metadata");

        assert_eq!(metadata["title"], "latest:off");
        assert_eq!(metadata["canonical"], "/blog/latest?preview=off");
        assert_eq!(metadata["request_bound"], true);
        assert_eq!(metadata["arbitrary_request_code_execution"], false);
        assert_eq!(metadata["request_prop_alias_count"], 2);
        let alias_expressions = metadata["request_prop_alias_bindings"]
            .as_array()
            .expect("metadata alias bindings")
            .iter()
            .filter_map(|binding| binding["expression"].as_str())
            .collect::<BTreeSet<_>>();
        assert!(alias_expressions.contains(r#"const { slug = "latest" } = params"#));
        assert!(
            alias_expressions.contains(r#"const { preview: previewMode = "off" } = searchParams"#)
        );
    }

    #[test]
    fn metadata_request_parameter_aliases_resolve_known_values() {
        let route_params = BTreeMap::from([("slug".to_string(), "post-1".to_string())]);
        let search_params = BTreeMap::from([("preview".to_string(), "draft".to_string())]);
        let source = r#"
            export async function generateMetadata({ params: routeParams, searchParams: queryParams }) {
                return {
                    title: `${routeParams.slug}:${queryParams?.preview}`,
                    canonical: `/blog/${routeParams["slug"]}`,
                };
            }
        "#;

        let metadata = safe_generate_metadata_source(
            "app/blog/[slug]/page.tsx",
            source,
            &route_params,
            &search_params,
        )
        .expect("safe generateMetadata parameter aliases should produce metadata");

        assert_eq!(metadata["title"], "post-1:draft");
        assert_eq!(metadata["canonical"], "/blog/post-1");
        assert_eq!(metadata["request_bound"], true);
        assert_eq!(metadata["node_modules_required"], false);
        assert_eq!(metadata["arbitrary_request_code_execution"], false);
        assert_eq!(metadata["request_prop_root_alias_count"], 2);
        let root_aliases = metadata["request_prop_root_alias_bindings"]
            .as_array()
            .expect("metadata root alias bindings")
            .iter()
            .filter_map(|binding| binding["alias"].as_str())
            .collect::<BTreeSet<_>>();
        assert!(root_aliases.contains("routeParams"));
        assert!(root_aliases.contains("queryParams"));
    }

    #[test]
    fn metadata_request_const_arrow_exports_resolve_known_values() {
        let route_params = BTreeMap::from([("slug".to_string(), "post-1".to_string())]);
        let search_params = BTreeMap::from([("preview".to_string(), "draft".to_string())]);
        let source = r#"
            export const generateMetadata = async ({ params, searchParams }) => {
                return {
                    title: `${params.slug}:${searchParams?.preview}`,
                    canonical: `/blog/${params["slug"]}`,
                };
            };
        "#;

        let metadata = safe_generate_metadata_source(
            "app/blog/[slug]/page.tsx",
            source,
            &route_params,
            &search_params,
        )
        .expect("safe const-arrow generateMetadata should produce metadata");

        assert_eq!(metadata["title"], "post-1:draft");
        assert_eq!(metadata["canonical"], "/blog/post-1");
        assert_eq!(metadata["request_bound"], true);
        assert_eq!(metadata["node_modules_required"], false);
        assert_eq!(metadata["arbitrary_request_code_execution"], false);
        assert_eq!(metadata["external_runtime_required"], false);
        assert_eq!(metadata["external_runtime_executed"], false);
    }

    #[test]
    fn metadata_request_const_arrow_expression_body_exports_resolve_known_values() {
        let route_params = BTreeMap::from([("slug".to_string(), "post-1".to_string())]);
        let search_params = BTreeMap::from([("preview".to_string(), "draft".to_string())]);
        let source = r#"
            export const generateMetadata = ({ params, searchParams }) => ({
                title: `${params.slug}:${searchParams?.preview}`,
                canonical: `/blog/${params["slug"]}`,
            });
        "#;

        let metadata = safe_generate_metadata_source(
            "app/blog/[slug]/page.tsx",
            source,
            &route_params,
            &search_params,
        )
        .expect("safe expression-bodied const-arrow generateMetadata should produce metadata");

        assert_eq!(metadata["title"], "post-1:draft");
        assert_eq!(metadata["canonical"], "/blog/post-1");
        assert_eq!(metadata["request_bound"], true);
        assert_eq!(metadata["node_modules_required"], false);
        assert_eq!(metadata["arbitrary_request_code_execution"], false);
        assert_eq!(metadata["external_runtime_required"], false);
        assert_eq!(metadata["external_runtime_executed"], false);
    }

    #[test]
    fn metadata_request_const_arrow_typed_props_exports_resolve_known_values() {
        let route_params = BTreeMap::from([("slug".to_string(), "post-1".to_string())]);
        let search_params = BTreeMap::from([("preview".to_string(), "draft".to_string())]);
        let source = r#"
            type MetadataProps = {
                params: { slug: string };
                searchParams: { preview?: string };
            };

            export const generateMetadata = async (props: MetadataProps): Promise<Metadata> => {
                const { params: routeParams, searchParams: queryParams } = props;
                return {
                    title: `${routeParams.slug}:${queryParams?.preview}`,
                    canonical: `/blog/${routeParams["slug"]}`,
                };
            };
        "#;

        let metadata = safe_generate_metadata_source(
            "app/blog/[slug]/page.tsx",
            source,
            &route_params,
            &search_params,
        )
        .expect("safe typed props const-arrow generateMetadata should produce metadata");

        assert_eq!(metadata["title"], "post-1:draft");
        assert_eq!(metadata["canonical"], "/blog/post-1");
        assert_eq!(metadata["request_bound"], true);
        assert_eq!(metadata["node_modules_required"], false);
        assert_eq!(metadata["arbitrary_request_code_execution"], false);
        assert_eq!(metadata["external_runtime_required"], false);
        assert_eq!(metadata["external_runtime_executed"], false);
        assert_eq!(metadata["request_prop_root_alias_count"], 2);
    }

    #[test]
    fn metadata_request_const_arrow_typed_destructure_exports_resolve_known_values() {
        let route_params = BTreeMap::from([("slug".to_string(), "post-1".to_string())]);
        let search_params = BTreeMap::from([("preview".to_string(), "draft".to_string())]);
        let source = r#"
            type MetadataProps = {
                params: { slug: string };
                searchParams: { preview?: string };
            };

            export const generateMetadata = ({ params, searchParams }: MetadataProps): Metadata => ({
                title: `${params.slug}:${searchParams?.preview}`,
                canonical: `/blog/${params["slug"]}`,
            });
        "#;

        let metadata = safe_generate_metadata_source(
            "app/blog/[slug]/page.tsx",
            source,
            &route_params,
            &search_params,
        )
        .expect("safe typed destructured const-arrow generateMetadata should produce metadata");

        assert_eq!(metadata["title"], "post-1:draft");
        assert_eq!(metadata["canonical"], "/blog/post-1");
        assert_eq!(metadata["request_bound"], true);
        assert_eq!(metadata["node_modules_required"], false);
        assert_eq!(metadata["arbitrary_request_code_execution"], false);
        assert_eq!(metadata["external_runtime_required"], false);
        assert_eq!(metadata["external_runtime_executed"], false);
    }

    #[test]
    fn metadata_request_const_arrow_bare_props_exports_resolve_known_values() {
        let route_params = BTreeMap::from([("slug".to_string(), "post-1".to_string())]);
        let search_params = BTreeMap::from([("preview".to_string(), "draft".to_string())]);
        let source = r#"
            export const generateMetadata = async props => {
                const { params: routeParams, searchParams: queryParams } = props;
                return {
                    title: `${routeParams.slug}:${queryParams?.preview}`,
                    canonical: `/blog/${routeParams["slug"]}`,
                };
            };
        "#;

        let metadata = safe_generate_metadata_source(
            "app/blog/[slug]/page.tsx",
            source,
            &route_params,
            &search_params,
        )
        .expect("safe bare props const-arrow generateMetadata should produce metadata");

        assert_eq!(metadata["title"], "post-1:draft");
        assert_eq!(metadata["canonical"], "/blog/post-1");
        assert_eq!(metadata["request_bound"], true);
        assert_eq!(metadata["node_modules_required"], false);
        assert_eq!(metadata["arbitrary_request_code_execution"], false);
        assert_eq!(metadata["external_runtime_required"], false);
        assert_eq!(metadata["external_runtime_executed"], false);
        assert_eq!(metadata["request_prop_root_alias_count"], 2);
    }

    #[test]
    fn metadata_request_const_arrow_generic_props_exports_resolve_known_values() {
        let route_params = BTreeMap::from([("slug".to_string(), "post-1".to_string())]);
        let search_params = BTreeMap::from([("preview".to_string(), "draft".to_string())]);
        let source = r#"
            type MetadataProps = {
                params: { slug: string };
                searchParams: { preview?: string };
            };

            export const generateMetadata = async <T extends MetadataProps>(props: T): Promise<Metadata> => {
                const { params: routeParams, searchParams: queryParams } = props;
                return {
                    title: `${routeParams.slug}:${queryParams?.preview}`,
                    canonical: `/blog/${routeParams["slug"]}`,
                };
            };
        "#;

        let metadata = safe_generate_metadata_source(
            "app/blog/[slug]/page.tsx",
            source,
            &route_params,
            &search_params,
        )
        .expect("safe generic props const-arrow generateMetadata should produce metadata");

        assert_eq!(metadata["title"], "post-1:draft");
        assert_eq!(metadata["canonical"], "/blog/post-1");
        assert_eq!(metadata["request_bound"], true);
        assert_eq!(metadata["node_modules_required"], false);
        assert_eq!(metadata["arbitrary_request_code_execution"], false);
        assert_eq!(metadata["external_runtime_required"], false);
        assert_eq!(metadata["external_runtime_executed"], false);
        assert_eq!(metadata["request_prop_root_alias_count"], 2);
    }

    #[test]
    fn metadata_request_object_parameter_aliases_resolve_known_values() {
        let route_params = BTreeMap::from([("slug".to_string(), "post-1".to_string())]);
        let search_params = BTreeMap::from([("preview".to_string(), "draft".to_string())]);
        let source = r#"
            export const generateMetadata = async (props) => {
                const { params: routeParams, searchParams: queryParams } = props;
                return {
                    title: `${routeParams.slug}:${queryParams?.preview}`,
                    canonical: `/blog/${routeParams["slug"]}`,
                };
            };
        "#;

        let metadata = safe_generate_metadata_source(
            "app/blog/[slug]/page.tsx",
            source,
            &route_params,
            &search_params,
        )
        .expect("safe props-object generateMetadata should produce metadata");

        assert_eq!(metadata["title"], "post-1:draft");
        assert_eq!(metadata["canonical"], "/blog/post-1");
        assert_eq!(metadata["request_bound"], true);
        assert_eq!(metadata["node_modules_required"], false);
        assert_eq!(metadata["arbitrary_request_code_execution"], false);
        assert_eq!(metadata["external_runtime_required"], false);
        assert_eq!(metadata["external_runtime_executed"], false);
        assert_eq!(metadata["request_prop_root_alias_count"], 2);
        let root_aliases = metadata["request_prop_root_alias_bindings"]
            .as_array()
            .expect("metadata root alias bindings")
            .iter()
            .filter_map(|binding| binding["alias"].as_str())
            .collect::<BTreeSet<_>>();
        assert!(root_aliases.contains("routeParams"));
        assert!(root_aliases.contains("queryParams"));
    }

    #[test]
    fn static_metadata_requires_export_const_surface() {
        let route_params = BTreeMap::new();
        let search_params = BTreeMap::new();
        let local_metadata_source = r#"
            export default function Page() {
                const metadata = {
                    title: "Local implementation detail",
                    description: "This object is not an App Router metadata export",
                };
                return <main>{metadata.title}</main>;
            }
        "#;
        let exported_metadata_source = r#"
            export const metadata = {
                title: "Exported metadata",
                description: "This object is an App Router metadata export",
            };
        "#;
        let local_viewport_source = r#"
            export default function Page() {
                const viewport = {
                    width: "device-width",
                    initialScale: 1,
                };
                return <main>Viewport is local</main>;
            }
        "#;
        let exported_viewport_source = r#"
            export const viewport = {
                width: "device-width",
                initialScale: 1,
            };
        "#;

        assert!(
            metadata_source(
                "app/page.tsx",
                local_metadata_source,
                &route_params,
                &search_params,
            )
            .is_none()
        );

        let metadata = metadata_source(
            "app/page.tsx",
            exported_metadata_source,
            &route_params,
            &search_params,
        )
        .expect("exported static metadata should be extracted");

        assert_eq!(metadata["title"], "Exported metadata");
        assert_eq!(metadata["mode"], "safe-static-metadata-literal-object");
        assert_eq!(metadata["source_owned_metadata"], true);

        assert!(
            metadata_source(
                "app/page.tsx",
                local_viewport_source,
                &route_params,
                &search_params,
            )
            .is_none()
        );

        let viewport = metadata_source(
            "app/page.tsx",
            exported_viewport_source,
            &route_params,
            &search_params,
        )
        .expect("exported static viewport should be extracted");

        assert_eq!(viewport["source_kind"], "viewport");
        assert_eq!(viewport["viewport"]["width"], "device-width");
        assert_eq!(viewport["source_owned_viewport_metadata"], true);
    }

    #[test]
    fn generate_metadata_requires_exported_surface() {
        let route_params = BTreeMap::from([("slug".to_string(), "notes".to_string())]);
        let search_params = BTreeMap::new();
        let local_generate_metadata_source = r#"
            export default function Page() {
                function generateMetadata() {
                    return { title: "Local generateMetadata helper" };
                }
                return <main>{generateMetadata().title}</main>;
            }
        "#;
        let local_generate_viewport_source = r#"
            export default function Page() {
                const generateViewport = () => ({
                    width: "device-width",
                    themeColor: "Local generateViewport helper",
                });
                return <main>{generateViewport().width}</main>;
            }
        "#;

        assert!(
            metadata_source(
                "app/page.tsx",
                local_generate_metadata_source,
                &route_params,
                &search_params,
            )
            .is_none()
        );
        assert!(
            metadata_source(
                "app/page.tsx",
                local_generate_viewport_source,
                &route_params,
                &search_params,
            )
            .is_none()
        );

        let exported_generate_metadata_source = r#"
            export default function Page() {
                function generateMetadata() {
                    return { title: "Local generateMetadata helper" };
                }
                return <main>Page</main>;
            }

            export async function generateMetadata({ params }) {
                return {
                    title: "Exported generateMetadata",
                    canonical: `/launch/${params.slug}`,
                };
            }
        "#;
        let exported_generate_viewport_source = r#"
            export default function Page() {
                const generateViewport = () => ({
                    width: "local-helper",
                    themeColor: "Local generateViewport helper",
                });
                return <main>Page</main>;
            }

            export const generateViewport = async () => ({
                width: "device-width",
                themeColor: "Exported generateViewport",
            });
        "#;

        let metadata = metadata_source(
            "app/[slug]/page.tsx",
            exported_generate_metadata_source,
            &route_params,
            &search_params,
        )
        .expect("exported generateMetadata should be extracted after local helpers are skipped");

        assert_eq!(metadata["source_kind"], "generateMetadata");
        assert_eq!(metadata["title"], "Exported generateMetadata");
        assert_eq!(metadata["canonical"], "/launch/notes");
        assert_eq!(metadata["source_owned_metadata"], true);

        let viewport = metadata_source(
            "app/page.tsx",
            exported_generate_viewport_source,
            &route_params,
            &search_params,
        )
        .expect("exported generateViewport should be extracted after local helpers are skipped");

        assert_eq!(viewport["source_kind"], "generateViewport");
        assert_eq!(viewport["viewport"]["width"], "device-width");
        assert_eq!(
            viewport["viewport"]["themeColor"],
            "Exported generateViewport"
        );
        assert_eq!(viewport["source_owned_viewport_metadata"], true);
    }

    #[test]
    fn client_metadata_exports_are_diagnostic_only() {
        let route_params = BTreeMap::new();
        let search_params = BTreeMap::new();
        let client_source = r##"
            /* app shell */
            "use strict";
            "use client";

            export const metadata = {
                title: "Client metadata",
                description: "Should be reported by diagnostics, not extracted",
            };

            export async function generateMetadata() {
                return { title: "Client generateMetadata" };
            }

            export const viewport = {
                width: "device-width",
                themeColor: "#111827",
            };

            export const generateViewport = () => ({
                themeColor: "#2563eb",
            });
        "##;
        let server_source = r#"
            "use strict";
            export const metadata = {
                title: "Server metadata",
            };
        "#;

        assert!(has_use_client_directive(client_source));
        assert!(
            metadata_source(
                "app/client/page.tsx",
                client_source,
                &route_params,
                &search_params,
            )
            .is_none()
        );

        assert!(!has_use_client_directive(server_source));
        let metadata = metadata_source(
            "app/server/page.tsx",
            server_source,
            &route_params,
            &search_params,
        )
        .expect("server metadata should still be extracted");

        assert_eq!(metadata["title"], "Server metadata");
        assert_eq!(metadata["source_owned_metadata"], true);
    }

    #[test]
    fn effective_metadata_merges_layout_defaults_with_page_overrides() {
        let sources = vec![
            json!({
                "source_path": "app/layout.tsx",
                "source_kind": "metadata",
                "title": "DX",
                "description": "Source-owned DX default",
                "canonical": "/",
            }),
            json!({
                "source_path": "app/blog/layout.tsx",
                "source_kind": "metadata",
                "title": null,
                "description": "Blog default",
                "canonical": "/blog",
            }),
            json!({
                "source_path": "app/blog/[slug]/page.tsx",
                "source_kind": "generateMetadata",
                "title": "Launch notes",
                "description": null,
                "canonical": null,
            }),
        ];

        let effective = effective_metadata(&sources);

        assert_eq!(effective["title"], "Launch notes");
        assert_eq!(effective["description"], "Blog default");
        assert_eq!(effective["canonical"], "/blog");
        assert_eq!(effective["source_owned_metadata_merge"], true);
        assert_eq!(
            effective["metadata_merge_precedence"],
            "segment-order-parent-to-leaf"
        );
        assert_eq!(
            effective["field_sources"]["title"],
            "app/blog/[slug]/page.tsx"
        );
        assert_eq!(
            effective["field_sources"]["description"],
            "app/blog/layout.tsx"
        );
        assert_eq!(
            effective["field_sources"]["canonical"],
            "app/blog/layout.tsx"
        );
        assert_eq!(effective["full_next_metadata_runtime"], false);
        assert_eq!(effective["external_runtime_executed"], false);
    }

    #[test]
    fn metadata_title_templates_apply_parent_template_to_leaf_title() {
        let route_params = BTreeMap::new();
        let search_params = BTreeMap::new();
        let layout_source = r#"
            export const metadata = {
                title: {
                    default: "DX",
                    template: "%s | DX",
                },
            };
        "#;
        let page_source = r#"
            export const metadata = {
                title: "Launch notes",
            };
        "#;

        let layout_metadata = metadata_source(
            "app/layout.tsx",
            layout_source,
            &route_params,
            &search_params,
        )
        .expect("layout metadata");
        let page_metadata =
            metadata_source("app/page.tsx", page_source, &route_params, &search_params)
                .expect("page metadata");
        let effective = effective_metadata(&[layout_metadata.clone(), page_metadata]);
        let layout_only = effective_metadata(&[layout_metadata]);

        assert_eq!(layout_only["title"], "DX");
        assert_eq!(layout_only["title_template"], "%s | DX");
        assert_eq!(layout_only["source_owned_title_template"], true);
        assert_eq!(effective["title"], "Launch notes | DX");
        assert_eq!(effective["title_template"], "%s | DX");
        assert_eq!(effective["source_owned_title_template"], true);
        assert_eq!(effective["field_sources"]["title"], "app/page.tsx");
        assert_eq!(
            effective["field_sources"]["title_template"],
            "app/layout.tsx"
        );
        assert_eq!(effective["full_next_title_runtime"], false);
    }

    #[test]
    fn metadata_title_absolute_bypasses_parent_template() {
        let route_params = BTreeMap::new();
        let search_params = BTreeMap::new();
        let layout_source = r#"
            export const metadata = {
                title: {
                    default: "DX",
                    template: "%s | DX",
                },
            };
        "#;
        let page_source = r#"
            export const metadata = {
                title: {
                    absolute: "Standalone launch",
                },
            };
        "#;

        let layout_metadata = metadata_source(
            "app/layout.tsx",
            layout_source,
            &route_params,
            &search_params,
        )
        .expect("layout metadata");
        let page_metadata =
            metadata_source("app/page.tsx", page_source, &route_params, &search_params)
                .expect("page metadata");
        let effective = effective_metadata(&[layout_metadata, page_metadata]);

        assert_eq!(effective["title"], "Standalone launch");
        assert_eq!(effective["title_absolute"], "Standalone launch");
        assert_eq!(effective["title_template"], "%s | DX");
        assert_eq!(effective["source_owned_title_absolute"], true);
        assert_eq!(effective["source_owned_title_template"], true);
        assert_eq!(effective["field_sources"]["title"], "app/page.tsx");
        assert_eq!(effective["field_sources"]["title_absolute"], "app/page.tsx");
        assert_eq!(
            effective["field_sources"]["title_template"],
            "app/layout.tsx"
        );
        assert_eq!(effective["full_next_title_runtime"], false);
    }

    #[test]
    fn metadata_open_graph_merges_parent_defaults_with_page_overrides() {
        let route_params = BTreeMap::from([("slug".to_string(), "launch".to_string())]);
        let search_params = BTreeMap::new();
        let layout_source = r#"
            export const metadata = {
                title: "DX",
                openGraph: {
                    title: "DX",
                    description: "Source-owned framework",
                    url: "/",
                    siteName: "DX WWW",
                    images: ["/og-default.png"],
                },
            };
        "#;
        let page_source = r#"
            export async function generateMetadata({ params }) {
                return {
                    title: `Post ${params.slug}`,
                    openGraph: {
                        title: `Post ${params.slug}`,
                        url: `/blog/${params.slug}`,
                    },
                };
            }
        "#;

        let layout_metadata = metadata_source(
            "app/layout.tsx",
            layout_source,
            &route_params,
            &search_params,
        )
        .expect("layout metadata");
        let page_metadata = metadata_source(
            "app/blog/[slug]/page.tsx",
            page_source,
            &route_params,
            &search_params,
        )
        .expect("page metadata");
        let effective = effective_metadata(&[layout_metadata, page_metadata]);

        assert_eq!(effective["openGraph"]["title"], "Post launch");
        assert_eq!(
            effective["openGraph"]["description"],
            "Source-owned framework"
        );
        assert_eq!(effective["openGraph"]["url"], "/blog/launch");
        assert_eq!(effective["openGraph"]["siteName"], "DX WWW");
        assert_eq!(effective["openGraph"]["images"], json!(["/og-default.png"]));
        assert_eq!(
            effective["open_graph_field_sources"]["title"],
            "app/blog/[slug]/page.tsx"
        );
        assert_eq!(
            effective["open_graph_field_sources"]["description"],
            "app/layout.tsx"
        );
        assert_eq!(effective["source_owned_open_graph_metadata"], true);
        assert_eq!(effective["full_next_open_graph_runtime"], false);
        assert_eq!(effective["external_runtime_executed"], false);
    }

    #[test]
    fn metadata_viewport_merges_parent_defaults_with_page_overrides() {
        let route_params = BTreeMap::new();
        let search_params = BTreeMap::from([("theme".to_string(), "#2563eb".to_string())]);
        let layout_source = r##"
            export const viewport = {
                width: "device-width",
                initialScale: 1,
                themeColor: "#0f172a",
                colorScheme: "dark light",
                userScalable: false,
            };
        "##;
        let page_source = r##"
            export const generateViewport = async ({ searchParams }) => {
                return {
                    themeColor: searchParams.theme,
                    maximumScale: 1,
                    interactiveWidget: "resizes-content",
                };
            };
        "##;

        let layout_metadata = metadata_source(
            "app/layout.tsx",
            layout_source,
            &route_params,
            &search_params,
        )
        .expect("layout viewport metadata");
        let page_metadata =
            metadata_source("app/page.tsx", page_source, &route_params, &search_params)
                .expect("page viewport metadata");
        let effective = effective_metadata(&[layout_metadata, page_metadata]);

        assert_eq!(effective["viewport"]["width"], "device-width");
        assert_eq!(effective["viewport"]["themeColor"], "#2563eb");
        assert_eq!(effective["viewport"]["colorScheme"], "dark light");
        assert_eq!(
            effective["viewport"]["interactiveWidget"],
            "resizes-content"
        );
        assert_eq!(effective["viewport"]["initialScale"].as_f64(), Some(1.0));
        assert_eq!(effective["viewport"]["maximumScale"].as_f64(), Some(1.0));
        assert_eq!(effective["viewport"]["userScalable"], false);
        assert_eq!(
            effective["viewport_field_sources"]["themeColor"],
            "app/page.tsx"
        );
        assert_eq!(
            effective["viewport_field_sources"]["width"],
            "app/layout.tsx"
        );
        assert_eq!(effective["source_owned_viewport_metadata"], true);
        assert_eq!(effective["full_next_viewport_runtime"], false);
        assert_eq!(effective["external_runtime_executed"], false);
    }

    #[test]
    fn metadata_head_tags_renders_safe_metadata_and_viewport_tags() {
        let metadata = json!({
            "title": "DX <Launch>",
            "description": "Source-owned \"metadata\"",
            "canonical": "/",
            "openGraph": {
                "title": "DX Launch",
                "description": "Preview-ready",
                "url": "/",
                "siteName": "DX WWW",
                "images": ["/og.png"],
            },
            "viewport": {
                "width": "device-width",
                "initialScale": 1,
                "maximumScale": 1,
                "themeColor": "#111827",
                "colorScheme": "dark light",
                "userScalable": false,
            },
        });

        let tags = metadata_head_tags(&metadata);

        assert!(tags.contains(r#"<title data-dx-metadata="title">DX &lt;Launch&gt;</title>"#));
        assert!(
            tags.contains(
                r#"<meta name="description" content="Source-owned &quot;metadata&quot;" data-dx-metadata="description" />"#
            )
        );
        assert!(tags.contains(
            r#"<meta property="og:title" content="DX Launch" data-dx-metadata="openGraph" />"#
        ));
        assert!(
            tags.contains(
                r#"<meta name="viewport" content="width=device-width,initial-scale=1,maximum-scale=1,user-scalable=no" data-dx-metadata="viewport" />"#
            )
        );
        assert!(tags.contains(
            r##"<meta name="theme-color" content="#111827" data-dx-metadata="viewport" />"##
        ));
        assert_eq!(metadata_head_tag_count(&tags), 11);
    }
}

#[derive(Default)]
struct MetadataExpression {
    value: Option<String>,
    request_prop_bindings: BTreeSet<String>,
    request_prop_root_alias_bindings: BTreeSet<String>,
    request_prop_alias_bindings: BTreeSet<String>,
    supported_expressions: BTreeSet<String>,
    unresolved_request_prop_bindings: BTreeSet<String>,
}

impl MetadataExpression {
    fn literal(value: String) -> Self {
        Self {
            value: Some(value),
            request_prop_bindings: BTreeSet::new(),
            request_prop_root_alias_bindings: BTreeSet::new(),
            request_prop_alias_bindings: BTreeSet::new(),
            supported_expressions: BTreeSet::new(),
            unresolved_request_prop_bindings: BTreeSet::new(),
        }
    }

    fn merge(&mut self, expression: Self) {
        self.request_prop_bindings
            .extend(expression.request_prop_bindings);
        self.request_prop_root_alias_bindings
            .extend(expression.request_prop_root_alias_bindings);
        self.request_prop_alias_bindings
            .extend(expression.request_prop_alias_bindings);
        self.supported_expressions
            .extend(expression.supported_expressions);
        self.unresolved_request_prop_bindings
            .extend(expression.unresolved_request_prop_bindings);
    }
}

#[derive(Clone)]
struct MetadataRequestRootAlias {
    alias: String,
    canonical_root: &'static str,
    expression: String,
}

#[derive(Clone)]
struct MetadataRequestPropAlias {
    alias: String,
    canonical_name: String,
    expression: String,
    value: Option<String>,
    source_kind: &'static str,
}

struct RequestMetadataContext<'a> {
    route_params: &'a BTreeMap<String, String>,
    search_params: &'a BTreeMap<String, String>,
    request_prop_root_aliases: BTreeMap<String, MetadataRequestRootAlias>,
    request_prop_aliases: BTreeMap<String, MetadataRequestPropAlias>,
    request_prop_bindings: BTreeSet<String>,
    request_prop_root_alias_bindings: BTreeSet<String>,
    request_prop_alias_bindings: BTreeSet<String>,
    supported_expressions: BTreeSet<String>,
    unresolved_request_prop_bindings: BTreeSet<String>,
}

impl<'a> RequestMetadataContext<'a> {
    fn new(
        route_params: &'a BTreeMap<String, String>,
        search_params: &'a BTreeMap<String, String>,
        request_prop_root_aliases: BTreeMap<String, MetadataRequestRootAlias>,
        request_prop_aliases: BTreeMap<String, MetadataRequestPropAlias>,
    ) -> Self {
        Self {
            route_params,
            search_params,
            request_prop_root_aliases,
            request_prop_aliases,
            request_prop_bindings: BTreeSet::new(),
            request_prop_root_alias_bindings: BTreeSet::new(),
            request_prop_alias_bindings: BTreeSet::new(),
            supported_expressions: BTreeSet::new(),
            unresolved_request_prop_bindings: BTreeSet::new(),
        }
    }

    fn request_bound(&self) -> bool {
        !self.request_prop_bindings.is_empty()
    }

    fn record_expression(&mut self, expression: &MetadataExpression) {
        self.request_prop_bindings
            .extend(expression.request_prop_bindings.iter().cloned());
        self.request_prop_root_alias_bindings
            .extend(expression.request_prop_root_alias_bindings.iter().cloned());
        self.request_prop_alias_bindings
            .extend(expression.request_prop_alias_bindings.iter().cloned());
        self.supported_expressions
            .extend(expression.supported_expressions.iter().cloned());
        self.unresolved_request_prop_bindings
            .extend(expression.unresolved_request_prop_bindings.iter().cloned());
    }

    fn request_prop_bindings(&self) -> Vec<String> {
        self.request_prop_bindings.iter().cloned().collect()
    }

    fn request_prop_root_alias_bindings(&self) -> Vec<Value> {
        self.request_prop_root_alias_bindings
            .iter()
            .filter_map(|alias| self.request_prop_root_aliases.get(alias))
            .map(|binding| {
                json!({
                    "alias": &binding.alias,
                    "canonical_root": binding.canonical_root,
                    "expression": &binding.expression,
                })
            })
            .collect()
    }

    fn request_prop_root_alias_count(&self) -> usize {
        self.request_prop_root_alias_bindings.len()
    }

    fn request_prop_alias_bindings(&self) -> Vec<Value> {
        self.request_prop_alias_bindings
            .iter()
            .filter_map(|alias| self.request_prop_aliases.get(alias))
            .map(|binding| {
                json!({
                    "alias": &binding.alias,
                    "canonical_name": &binding.canonical_name,
                    "source_kind": binding.source_kind,
                    "expression": &binding.expression,
                    "value_resolved": binding.value.is_some(),
                    "value_type": "string",
                })
            })
            .collect()
    }

    fn request_prop_alias_count(&self) -> usize {
        self.request_prop_alias_bindings.len()
    }

    fn supported_expressions(&self) -> Vec<String> {
        self.supported_expressions.iter().cloned().collect()
    }

    fn unresolved_request_prop_bindings(&self) -> Vec<String> {
        self.unresolved_request_prop_bindings
            .iter()
            .cloned()
            .collect()
    }
}

fn supported_request_metadata_patterns() -> Vec<&'static str> {
    vec![
        "params.slug",
        "params[\"slug\"]",
        "params?.slug",
        "params?.[\"slug\"]",
        "(await params)?.slug",
        "searchParams.preview",
        "searchParams[\"preview\"]",
        "searchParams?.preview",
        "searchParams?.[\"preview\"]",
        "(await searchParams)?.preview",
        "title: { default: \"DX\", template: \"%s | DX\" }",
        "title: { absolute: \"Standalone\" }",
        "generateMetadata({ params: routeParams, searchParams: queryParams })",
        "export const generateMetadata = async ({ params, searchParams }) => { ... }",
        "export const generateMetadata = ({ params, searchParams }) => ({ ... })",
        "export const generateMetadata = async (props: MetadataProps): Promise<Metadata> => { ... }",
        "export const generateMetadata = ({ params, searchParams }: MetadataProps): Metadata => ({ ... })",
        "export const generateMetadata = async props => { ... }",
        "export const generateMetadata = async <T extends MetadataProps>(props: T): Promise<Metadata> => { ... }",
        "generateMetadata(props)",
        "const { params: routeParams, searchParams: queryParams } = props",
        "routeParams.slug",
        "queryParams?.preview",
        "const { slug = \"latest\" } = params",
        "const { preview: previewMode = \"off\" } = searchParams",
        "`/blog/${params.slug}`",
        "`/blog/${params?.slug}`",
    ]
}

fn clean_object_key(source: &str) -> String {
    source
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .to_string()
}

fn split_top_level_entries(source: &str) -> Vec<&str> {
    let mut entries = Vec::new();
    let mut start = 0usize;
    let mut cursor = 0usize;
    let mut quote = None;
    let mut depth = 0usize;
    while cursor < source.len() {
        let Some(character) = source[cursor..].chars().next() else {
            break;
        };
        if let Some(active_quote) = quote {
            if character == active_quote {
                quote = None;
            } else if character == '\\' {
                cursor += character.len_utf8();
                if cursor < source.len() {
                    cursor += source[cursor..]
                        .chars()
                        .next()
                        .unwrap_or_default()
                        .len_utf8();
                    continue;
                }
            }
            cursor += character.len_utf8();
            continue;
        }
        match character {
            '"' | '\'' | '`' => quote = Some(character),
            '{' | '[' | '(' => depth += 1,
            '}' | ']' | ')' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                let entry = source[start..cursor].trim();
                if !entry.is_empty() {
                    entries.push(entry);
                }
                start = cursor + character.len_utf8();
            }
            _ => {}
        }
        cursor += character.len_utf8();
    }
    let entry = source[start..].trim();
    if !entry.is_empty() {
        entries.push(entry);
    }
    entries
}

fn find_balanced_delimiter(
    source: &str,
    mut cursor: usize,
    open: char,
    close: char,
) -> Option<usize> {
    let mut quote = None;
    let mut depth = 0usize;
    while cursor < source.len() {
        let character = source[cursor..].chars().next()?;
        if let Some(active_quote) = quote {
            if character == active_quote {
                quote = None;
            } else if character == '\\' {
                cursor += character.len_utf8();
                if cursor < source.len() {
                    cursor += source[cursor..].chars().next()?.len_utf8();
                    continue;
                }
            }
            cursor += character.len_utf8();
            continue;
        }
        match character {
            '"' | '\'' | '`' => quote = Some(character),
            _ if character == open => depth += 1,
            _ if character == close => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(cursor);
                }
            }
            _ => {}
        }
        cursor += character.len_utf8();
    }
    None
}
