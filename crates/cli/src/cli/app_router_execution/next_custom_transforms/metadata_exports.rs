use dx_compiler::delivery::parse_tsx_module;
use serde_json::{Value, json};

mod scanner;

use self::scanner::{
    find_balanced_delimiter, find_word, identifier_after, next_keyword, next_non_ws,
    read_export_value, read_reexport_source, word_boundary,
};

const METADATA_CONFLICT: &str = "metadata-and-generateMetadata";
const VIEWPORT_CONFLICT: &str = "viewport-and-generateViewport";

/// balanced-metadata-exports scanner for Lane 11 Next custom-transform receipts.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct MetadataExportDetection {
    pub(super) static_metadata: bool,
    pub(super) generate_metadata: bool,
    pub(super) metadata_conflict: bool,
    pub(super) static_viewport: bool,
    pub(super) generate_viewport: bool,
    pub(super) viewport_conflict: bool,
    pub(super) parsed_static_metadata: Option<Value>,
    pub(super) static_metadata_value_source: Option<String>,
    pub(super) static_metadata_value_kind: Option<&'static str>,
    pub(super) generate_metadata_return_source: Option<String>,
    pub(super) generate_metadata_return_kind: Option<&'static str>,
    pub(super) static_viewport_value_source: Option<String>,
    pub(super) static_viewport_value_kind: Option<&'static str>,
    pub(super) generate_viewport_return_source: Option<String>,
    pub(super) generate_viewport_return_kind: Option<&'static str>,
    pub(super) exports: Vec<MetadataExportSurface>,
    pub(super) compatibility_issues: Vec<&'static str>,
    pub(super) server_component_only_enforced: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MetadataExportSurface {
    pub(super) name: String,
    pub(super) export_kind: &'static str,
    pub(super) async_export: bool,
    pub(super) value_source: Option<String>,
    pub(super) value_kind: Option<&'static str>,
    pub(super) compatibility_issue: Option<&'static str>,
}

pub(super) fn collect_metadata_export_detections(
    source_path: &str,
    source: &str,
) -> Vec<MetadataExportDetection> {
    let mut exports = Vec::new();
    collect_static_metadata_exports(source, &mut exports);
    collect_generate_metadata_exports(source, &mut exports);
    collect_named_metadata_reexports(source, &mut exports);
    normalize_exports(&mut exports);

    if exports.is_empty() {
        return Vec::new();
    }

    let static_metadata = exports.iter().any(|export| export.name == "metadata");
    let generate_metadata = exports
        .iter()
        .any(|export| export.name == "generateMetadata");
    let metadata_conflict = static_metadata && generate_metadata;
    let static_viewport = exports.iter().any(|export| export.name == "viewport");
    let generate_viewport = exports
        .iter()
        .any(|export| export.name == "generateViewport");
    let viewport_conflict = static_viewport && generate_viewport;
    let mut compatibility_issues = exports
        .iter()
        .filter_map(|export| export.compatibility_issue)
        .collect::<Vec<_>>();
    if metadata_conflict {
        compatibility_issues.push(METADATA_CONFLICT);
    }
    if viewport_conflict {
        compatibility_issues.push(VIEWPORT_CONFLICT);
    }
    compatibility_issues.sort();
    compatibility_issues.dedup();

    let ast = parse_tsx_module(source_path, source);
    let parsed_static_metadata = ast.metadata.map(|metadata| {
        json!({
            "title": metadata.title,
            "description": metadata.description,
            "canonical": metadata.canonical,
        })
    });
    let static_surface = exports.iter().find(|export| export.name == "metadata");
    let generate_surface = exports
        .iter()
        .find(|export| export.name == "generateMetadata");
    let static_viewport_surface = exports.iter().find(|export| export.name == "viewport");
    let generate_viewport_surface = exports
        .iter()
        .find(|export| export.name == "generateViewport");

    vec![MetadataExportDetection {
        static_metadata,
        generate_metadata,
        metadata_conflict,
        static_viewport,
        generate_viewport,
        viewport_conflict,
        parsed_static_metadata,
        static_metadata_value_source: static_surface.and_then(|export| export.value_source.clone()),
        static_metadata_value_kind: static_surface.and_then(|export| export.value_kind),
        generate_metadata_return_source: generate_surface
            .and_then(|export| export.value_source.as_deref())
            .and_then(read_generate_metadata_return),
        generate_metadata_return_kind: generate_surface
            .and_then(|export| export.value_source.as_deref())
            .and_then(read_generate_metadata_return)
            .as_deref()
            .map(classify_metadata_value),
        static_viewport_value_source: static_viewport_surface
            .and_then(|export| export.value_source.clone()),
        static_viewport_value_kind: static_viewport_surface.and_then(|export| export.value_kind),
        generate_viewport_return_source: generate_viewport_surface
            .and_then(|export| export.value_source.as_deref())
            .and_then(read_generate_metadata_return),
        generate_viewport_return_kind: generate_viewport_surface
            .and_then(|export| export.value_source.as_deref())
            .and_then(read_generate_metadata_return)
            .as_deref()
            .map(classify_metadata_value),
        exports,
        compatibility_issues,
        server_component_only_enforced: false,
    }]
}

pub(super) fn collect_metadata_export_names(source_path: &str, source: &str) -> Vec<String> {
    let mut names = collect_metadata_export_detections(source_path, source)
        .into_iter()
        .flat_map(|detection| detection.exports.into_iter().map(|surface| surface.name))
        .collect::<Vec<_>>();
    names.sort();
    names.dedup();
    names
}

fn collect_static_metadata_exports(source: &str, exports: &mut Vec<MetadataExportSurface>) {
    let mut cursor = 0usize;
    while let Some(export_index) = find_word(source, cursor, "export") {
        let after_export = export_index + "export".len();
        let Some(const_index) = next_keyword(source, after_export, "const") else {
            cursor = after_export;
            continue;
        };
        let after_const = const_index + "const".len();
        let Some((name, name_end)) = identifier_after(source, after_const) else {
            cursor = after_const;
            continue;
        };
        if !matches!(name.as_str(), "metadata" | "viewport") {
            cursor = name_end;
            continue;
        }
        let Some(eq_index) = source[name_end..].find('=').map(|offset| name_end + offset) else {
            cursor = name_end;
            continue;
        };
        let (value_source, value_end) = read_export_value(source, eq_index + 1);
        exports.push(MetadataExportSurface {
            name,
            export_kind: "const",
            async_export: false,
            value_kind: Some(classify_metadata_value(&value_source)),
            value_source: Some(value_source),
            compatibility_issue: None,
        });
        cursor = value_end.max(eq_index + 1);
    }
}

fn collect_generate_metadata_exports(source: &str, exports: &mut Vec<MetadataExportSurface>) {
    let mut cursor = 0usize;
    while let Some(export_index) = find_word(source, cursor, "export") {
        let after_export = export_index + "export".len();
        if let Some((surface, end)) = exported_generate_metadata_function(source, after_export) {
            exports.push(surface);
            cursor = end;
            continue;
        }
        if let Some((surface, end)) = exported_generate_metadata_const(source, after_export) {
            exports.push(surface);
            cursor = end;
            continue;
        }
        cursor = after_export;
    }
}

fn exported_generate_metadata_function(
    source: &str,
    start: usize,
) -> Option<(MetadataExportSurface, usize)> {
    let mut cursor = next_non_ws(source, start)?;
    let async_export = if source[cursor..].starts_with("async")
        && word_boundary(source, cursor, cursor + "async".len())
    {
        cursor = next_non_ws(source, cursor + "async".len())?;
        true
    } else {
        false
    };
    if !source[cursor..].starts_with("function")
        || !word_boundary(source, cursor, cursor + "function".len())
    {
        return None;
    }
    let after_function = cursor + "function".len();
    let (name, name_end) = identifier_after(source, after_function)?;
    if !is_generate_metadata_surface_name(&name) {
        return None;
    }
    let block = function_block_after_name(source, name_end)?;
    Some((
        MetadataExportSurface {
            name,
            export_kind: "function",
            async_export,
            value_source: Some(block.0.to_string()),
            value_kind: read_generate_metadata_return(block.0)
                .as_deref()
                .map(classify_metadata_value),
            compatibility_issue: None,
        },
        block.1,
    ))
}

fn exported_generate_metadata_const(
    source: &str,
    start: usize,
) -> Option<(MetadataExportSurface, usize)> {
    let const_index = next_keyword(source, start, "const")?;
    let after_const = const_index + "const".len();
    let (name, name_end) = identifier_after(source, after_const)?;
    if !is_generate_metadata_surface_name(&name) {
        return None;
    }
    let eq_index = source[name_end..]
        .find('=')
        .map(|offset| name_end + offset)?;
    let (value_source, value_end) = read_export_value(source, eq_index + 1);
    let async_export = value_source.trim_start().starts_with("async ");
    Some((
        MetadataExportSurface {
            name,
            export_kind: "const-arrow",
            async_export,
            value_kind: read_generate_metadata_return(&value_source)
                .as_deref()
                .map(classify_metadata_value),
            value_source: Some(value_source),
            compatibility_issue: None,
        },
        value_end,
    ))
}

fn collect_named_metadata_reexports(source: &str, exports: &mut Vec<MetadataExportSurface>) {
    let mut cursor = 0usize;
    while let Some(export_index) = find_word(source, cursor, "export") {
        let after_export = export_index + "export".len();
        let Some(brace_start) = next_non_ws(source, after_export).filter(|index| {
            source[*index..]
                .chars()
                .next()
                .is_some_and(|character| character == '{')
        }) else {
            cursor = after_export;
            continue;
        };
        let Some(brace_end) = find_balanced_delimiter(source, brace_start, '{', '}') else {
            cursor = brace_start + 1;
            continue;
        };
        let reexport_source = read_reexport_source(&source[brace_end + 1..]);
        for specifier in source[brace_start + 1..brace_end].split(',') {
            let Some(name) = metadata_name_from_specifier(specifier) else {
                continue;
            };
            exports.push(MetadataExportSurface {
                name: name.to_string(),
                export_kind: "named-re-export",
                async_export: is_generate_metadata_surface_name(name),
                value_kind: Some("named-re-export"),
                value_source: Some(
                    reexport_source
                        .as_ref()
                        .map(|module| format!("named-re-export from {module}"))
                        .unwrap_or_else(|| "named-re-export".to_string()),
                ),
                compatibility_issue: Some("metadata-re-export"),
            });
        }
        cursor = brace_end + 1;
    }
}

fn normalize_exports(exports: &mut Vec<MetadataExportSurface>) {
    exports.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then(left.export_kind.cmp(right.export_kind))
            .then(left.value_source.cmp(&right.value_source))
    });
    exports.dedup_by(|left, right| {
        left.name == right.name
            && left.export_kind == right.export_kind
            && left.value_source == right.value_source
    });
}

fn function_block_after_name(source: &str, name_end: usize) -> Option<(&str, usize)> {
    let params_start = next_non_ws(source, name_end)?;
    if !source[params_start..].starts_with('(') {
        return None;
    }
    let params_end = find_balanced_delimiter(source, params_start, '(', ')')?;
    let block_start = next_non_ws(source, params_end + 1)?;
    if !source[block_start..].starts_with('{') {
        return None;
    }
    let block_end = find_balanced_delimiter(source, block_start, '{', '}')?;
    Some((&source[block_start + 1..block_end], block_end + 1))
}

fn read_generate_metadata_return(source: &str) -> Option<String> {
    let return_index = find_word(source, 0, "return");
    if let Some(return_index) = return_index {
        let object_start = source[return_index..]
            .find('{')
            .map(|offset| return_index + offset)?;
        let object_end = find_balanced_delimiter(source, object_start, '{', '}')?;
        return Some(source[object_start..=object_end].trim().to_string());
    }

    let arrow_index = source.find("=>")?;
    let after_arrow = source[arrow_index + "=>".len()..].trim_start();
    let object_start = after_arrow.find('{')?;
    let absolute_start = source.len() - after_arrow.len() + object_start;
    let object_end = find_balanced_delimiter(source, absolute_start, '{', '}')?;
    Some(source[absolute_start..=object_end].trim().to_string())
}

fn classify_metadata_value(value_source: &str) -> &'static str {
    let trimmed = value_source.trim();
    if trimmed.is_empty() {
        "missing-initializer"
    } else if trimmed.starts_with('{') {
        "object-literal"
    } else if trimmed.starts_with('(')
        && trimmed
            .trim_start_matches('(')
            .trim_start()
            .starts_with('{')
    {
        "parenthesized-object-literal"
    } else {
        "identifier-or-expression"
    }
}

fn metadata_name_from_specifier(specifier: &str) -> Option<&str> {
    let trimmed = specifier
        .trim()
        .strip_prefix("type ")
        .unwrap_or(specifier)
        .trim();
    if trimmed.is_empty() {
        return None;
    }
    let mut parts = trimmed.split_whitespace();
    let original = parts.next()?;
    match (parts.next(), parts.next()) {
        (Some("as"), Some(exported)) if is_metadata_export_name(exported) => Some(exported),
        (Some("as"), Some(_)) if is_metadata_export_name(original) => Some(original),
        _ if is_metadata_export_name(original) => Some(original),
        _ => None,
    }
}

fn is_metadata_export_name(name: &str) -> bool {
    matches!(
        name,
        "metadata" | "generateMetadata" | "viewport" | "generateViewport"
    )
}

fn is_generate_metadata_surface_name(name: &str) -> bool {
    matches!(name, "generateMetadata" | "generateViewport")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_static_and_generate_metadata_without_runtime_execution() {
        let detections = collect_metadata_export_detections(
            "app/page.tsx",
            r#"export const metadata = {
  title: "Dashboard",
  alternates: { canonical: "/dashboard" },
};
export async function generateMetadata() {
  return { title: "Dynamic dashboard" };
}
"#,
        );

        assert_eq!(detections.len(), 1);
        let detection = &detections[0];
        assert!(detection.static_metadata);
        assert!(detection.generate_metadata);
        assert!(detection.metadata_conflict);
        assert_eq!(
            detection.generate_metadata_return_source.as_deref(),
            Some(r#"{ title: "Dynamic dashboard" }"#)
        );
        assert!(
            detection
                .compatibility_issues
                .contains(&"metadata-and-generateMetadata")
        );
        assert!(!detection.server_component_only_enforced);
    }

    #[test]
    fn records_metadata_reexports_as_adapter_boundary_evidence() {
        let detections = collect_metadata_export_detections(
            "app/layout.tsx",
            r#"export { pageMetadata as metadata, makeMetadata as generateMetadata } from "./seo";"#,
        );

        assert_eq!(detections.len(), 1);
        assert!(detections[0].static_metadata);
        assert!(detections[0].generate_metadata);
        assert!(
            detections[0]
                .compatibility_issues
                .contains(&"metadata-re-export")
        );
        assert!(
            detections[0]
                .exports
                .iter()
                .all(|export| export.export_kind == "named-re-export")
        );
    }

    #[test]
    fn detects_static_and_generate_viewport_without_runtime_execution() {
        let detections = collect_metadata_export_detections(
            "app/layout.tsx",
            r##"export const viewport = {
  width: "device-width",
  initialScale: 1,
};
export async function generateViewport() {
  return { themeColor: "#101827" };
}
"##,
        );

        assert_eq!(detections.len(), 1);
        let detection = &detections[0];
        assert!(detection.static_viewport);
        assert!(detection.generate_viewport);
        assert!(detection.viewport_conflict);
        assert_eq!(detection.static_viewport_value_kind, Some("object-literal"));
        assert_eq!(
            detection.generate_viewport_return_source.as_deref(),
            Some(r##"{ themeColor: "#101827" }"##)
        );
        assert!(
            detection
                .compatibility_issues
                .contains(&"viewport-and-generateViewport")
        );
    }
}
