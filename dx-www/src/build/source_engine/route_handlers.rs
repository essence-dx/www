use std::path::{Path, PathBuf};

use dx_compiler::delivery::parse_tsx_module;

use crate::error::{DxError, DxResult};

use super::ecmascript_analysis::analyze_ecmascript_source;
use super::graph::{
    SourceBuildImport, SourceBuildRouteHandler, hash_bytes, normalize_path, output_snapshot_path,
    read_file, relative_path, route_from_app_page, write_file,
};

const HTTP_METHODS: [&str; 7] = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];

pub(super) fn compile_route_handlers(
    project_root: &Path,
    output_dir: &Path,
    handlers: &[PathBuf],
) -> DxResult<Vec<SourceBuildRouteHandler>> {
    let mut compiled = Vec::new();

    for handler in handlers {
        let bytes = read_file(handler)?;
        let source =
            String::from_utf8(bytes.clone()).map_err(|error| DxError::CompilationError {
                message: error.to_string(),
                file: handler.to_path_buf(),
                src: None,
                span: None,
            })?;
        let relative = relative_path(project_root, handler);
        let normalized_relative = normalize_path(&relative);
        let source_kind = handler
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or("source");
        let ast = parse_tsx_module(&normalized_relative, &source);
        let output = output_snapshot_path(output_dir, &relative);
        write_file(&output, source.as_bytes())?;

        compiled.push(SourceBuildRouteHandler {
            route: route_from_app_page(project_root, handler),
            path: normalized_relative.clone(),
            output: normalize_path(&relative_path(project_root, &output)),
            hash: hash_bytes(&bytes),
            methods: exported_methods(&source),
            imports: ast
                .imports
                .into_iter()
                .map(|import| SourceBuildImport {
                    specifier: import.source,
                    side_effect_only: import.side_effect_only,
                    type_only: import.type_only,
                })
                .collect(),
            parser_backend: ast.parser_backend.active_backend,
            diagnostics: ast.diagnostics.len(),
            execution_model: "source-owned-route-handler-contract".to_string(),
            lifecycle_scripts_executed: false,
            node_modules_required: false,
            ecmascript_analysis: analyze_ecmascript_source(
                &normalized_relative,
                source_kind,
                &source,
                &[],
            ),
        });
    }

    Ok(compiled)
}

fn exported_methods(source: &str) -> Vec<String> {
    HTTP_METHODS
        .into_iter()
        .filter(|method| {
            source.contains(&format!("export function {method}"))
                || source.contains(&format!("export async function {method}"))
                || source.contains(&format!("export const {method}"))
                || source.contains(&format!("export let {method}"))
                || named_export_clause_exports_method(source, method)
                || destructured_binding_export_exports_method(source, method)
        })
        .map(str::to_string)
        .collect()
}

fn named_export_clause_exports_method(source: &str, method: &str) -> bool {
    let mut remaining = source;

    while let Some(start) = remaining.find("export {") {
        let clause_start = start + "export {".len();
        let after_open = &remaining[clause_start..];
        let Some(end) = after_open.find('}') else {
            return false;
        };

        if export_clause_contains_method(&after_open[..end], method) {
            return true;
        }

        remaining = &after_open[end + 1..];
    }

    false
}

fn export_clause_contains_method(clause: &str, method: &str) -> bool {
    clause.split(',').any(|part| {
        let tokens = part.split_whitespace().collect::<Vec<_>>();
        matches!(tokens.as_slice(), [exported] if *exported == method)
            || matches!(tokens.as_slice(), [_, "as", exported] if *exported == method)
    })
}

fn destructured_binding_export_exports_method(source: &str, method: &str) -> bool {
    ["export const {", "export let {", "export var {"]
        .into_iter()
        .any(|prefix| destructured_binding_export_prefix_exports_method(source, prefix, method))
}

fn destructured_binding_export_prefix_exports_method(
    source: &str,
    prefix: &str,
    method: &str,
) -> bool {
    let mut remaining = source;

    while let Some(start) = remaining.find(prefix) {
        let clause_start = start + prefix.len();
        let after_open = &remaining[clause_start..];
        let Some(end) = after_open.find('}') else {
            return false;
        };

        if destructured_binding_clause_exports_method(&after_open[..end], method) {
            return true;
        }

        remaining = &after_open[end + 1..];
    }

    false
}

fn destructured_binding_clause_exports_method(clause: &str, method: &str) -> bool {
    clause.split(',').any(|part| {
        let binding = part.trim();
        if binding.is_empty() || binding.contains(':') {
            return false;
        }

        binding
            .split('=')
            .next()
            .unwrap_or(binding)
            .split_whitespace()
            .next()
            == Some(method)
    })
}

#[cfg(test)]
mod tests {
    use super::exported_methods;

    #[test]
    fn exported_methods_detects_alias_reexports() {
        let source = r#"
const getHandler = () => Response.json({ ok: true });
async function postHandler() {
    return Response.json({ ok: true });
}
export const PUT = () => Response.json({ ok: true });
const HEAD = () => new Response(null);
export {
    getHandler as GET,
    postHandler as POST,
    HEAD,
};
"#;

        assert_eq!(
            exported_methods(source),
            vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "HEAD".to_string()
            ]
        );
    }

    #[test]
    fn exported_methods_detects_destructured_helper_exports() {
        let source = r#"
import { createDxInstantRouteHandlers } from "@/lib/instant/route";

export const {
    GET,
    POST,
    DELETE: deleteHandler,
} = createDxInstantRouteHandlers();
"#;

        assert_eq!(
            exported_methods(source),
            vec!["GET".to_string(), "POST".to_string()]
        );
    }
}
