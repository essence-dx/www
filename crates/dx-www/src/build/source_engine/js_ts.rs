use std::path::{Path, PathBuf};

use dx_compiler::delivery::parse_tsx_module;

use crate::error::{DxError, DxResult};

use super::ecmascript_analysis::analyze_ecmascript_source;
use super::graph::{
    SourceBuildImport, SourceBuildRoute, hash_bytes, normalize_path, output_snapshot_path,
    read_file, relative_path, route_from_app_page, write_file,
};

pub fn compile_routes(
    project_root: &Path,
    output_dir: &Path,
    routes: &[PathBuf],
) -> DxResult<Vec<SourceBuildRoute>> {
    let mut compiled = Vec::new();

    for route in routes {
        let bytes = read_file(route)?;
        let source =
            String::from_utf8(bytes.clone()).map_err(|error| DxError::CompilationError {
                message: error.to_string(),
                file: route.to_path_buf(),
                src: None,
                span: None,
            })?;
        let relative = relative_path(project_root, route);
        let normalized_relative = normalize_path(&relative);
        let source_kind = route
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or("source");
        let ast = parse_tsx_module(&normalized_relative, &source);
        let output = output_snapshot_path(output_dir, &relative);
        write_file(&output, source.as_bytes())?;

        compiled.push(SourceBuildRoute {
            route: route_from_app_page(project_root, route),
            path: normalized_relative.clone(),
            output: normalize_path(&relative_path(project_root, &output)),
            hash: hash_bytes(&bytes),
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
