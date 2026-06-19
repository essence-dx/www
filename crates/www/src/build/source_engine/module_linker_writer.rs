use std::path::Path;

use serde_json::json;

use crate::error::DxResult;

use super::graph::{SourceBuildModuleChunk, write_file};
use super::module_runtime_transform::ModuleRuntimeTransform;

pub fn write_module_chunk(
    path: &Path,
    chunk: &SourceBuildModuleChunk,
    source_text: &str,
    runtime_transform: &ModuleRuntimeTransform,
    dependency_runtime_exports: &[Vec<String>],
) -> DxResult<()> {
    let imports = chunk
        .dependencies
        .iter()
        .filter_map(|dependency| dependency.chunk_output.as_deref())
        .enumerate()
        .map(|(index, output)| {
            let file_name = Path::new(output)
                .file_name()
                .and_then(|file_name| file_name.to_str())
                .unwrap_or(output);
            format!(
                r#"import {{ dxSourceModule as dep{index}, dxRuntimeExports as dep{index}Runtime }} from "./{file_name}";"#
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let dependency_vars = chunk
        .dependencies
        .iter()
        .filter(|dependency| dependency.chunk_output.is_some())
        .enumerate()
        .map(|(index, _)| format!("dep{index}"))
        .collect::<Vec<_>>()
        .join(", ");
    let metadata = json!(chunk);
    let runtime_exports = runtime_export_object(&runtime_transform.export_names);
    let transformed = if runtime_transform.transformed_source.is_some() {
        "true"
    } else {
        "false"
    };
    let transformed_source = runtime_transform
        .transformed_source
        .as_deref()
        .unwrap_or_default();
    let runtime_bindings =
        runtime_dependency_bindings(dependency_runtime_exports, transformed_source);
    let source_text = runtime_transform
        .transformed_source
        .as_deref()
        .unwrap_or(source_text);
    let source = format!(
        "{imports}\nexport const dxSourceText = {};\nexport const dxSourceModule = Object.freeze({});\nexport const dxRuntimeModule = Object.freeze({{\n  transformed: {transformed},\n  transformKind: {},\n  exportNames: {}\n}});\n{runtime_bindings}{transformed_source}export const dxRuntimeExports = Object.freeze({runtime_exports});\nexport const dxLinkedDependencies = Object.freeze([{dependency_vars}]);\nexport default dxSourceModule;\n",
        serde_json::to_string(source_text)?,
        serde_json::to_string_pretty(&metadata)?,
        serde_json::to_string(&runtime_transform.transform_kind)?,
        serde_json::to_string(&runtime_transform.export_names)?,
    );
    write_file(path, source.as_bytes())
}

fn runtime_export_object(export_names: &[String]) -> String {
    if export_names.is_empty() {
        "{}".to_string()
    } else {
        format!("{{ {} }}", export_names.join(", "))
    }
}

fn runtime_dependency_bindings(
    dependency_exports: &[Vec<String>],
    transformed_source: &str,
) -> String {
    if transformed_source.is_empty() {
        return String::new();
    }

    let mut seen = Vec::new();
    let mut lines = Vec::new();
    for (index, export_names) in dependency_exports.iter().enumerate() {
        for export_name in export_names {
            if seen.iter().any(|seen_name| seen_name == export_name)
                || !references_identifier(transformed_source, export_name)
            {
                continue;
            }
            seen.push(export_name.clone());
            lines.push(format!(
                "const {export_name} = dep{index}Runtime.{export_name};"
            ));
        }
    }

    if lines.is_empty() {
        String::new()
    } else {
        format!("{}\n", lines.join("\n"))
    }
}

fn references_identifier(source: &str, identifier: &str) -> bool {
    source.match_indices(identifier).any(|(start, _)| {
        let before = source[..start].chars().next_back();
        let after = source[start + identifier.len()..].chars().next();
        !before.is_some_and(is_identifier_char) && !after.is_some_and(is_identifier_char)
    })
}

fn is_identifier_char(character: char) -> bool {
    character.is_ascii_alphanumeric() || matches!(character, '_' | '$')
}
