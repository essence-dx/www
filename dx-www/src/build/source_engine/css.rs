use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};
use crate::parser::style::StyleParser;

use super::css_asset_references::collect_flattened_css_asset_references;
use super::css_imports::flatten_local_css_imports_with_manifest;
use super::css_optimizer::{optimize_css, render_minified_css};
use super::css_source_map::{CssSourceMapSource, CssSourceMapSourceRole, write_css_source_map};
use super::css_usage::CssUsage;
use super::graph::{
    SourceBuildStyle, SourceBuildStyleImport, SourceBuildStyleRetainedImport, hash_bytes,
    normalize_path, read_file, relative_path, write_file,
};

pub fn compile_styles(
    project_root: &Path,
    output_dir: &Path,
    styles: &[PathBuf],
    css_usage: &CssUsage,
) -> DxResult<Vec<SourceBuildStyle>> {
    let parser = StyleParser::new();
    let mut compiled = Vec::new();

    for style_path in styles {
        let bytes = read_file(style_path)?;
        let source = String::from_utf8(bytes).map_err(|error| DxError::CompilationError {
            message: error.to_string(),
            file: style_path.to_path_buf(),
            src: None,
            span: None,
        })?;
        let flattened = flatten_local_css_imports_with_manifest(project_root, style_path, &source)?;
        let asset_references =
            collect_flattened_css_asset_references(project_root, style_path, &source, &flattened);
        let parsed = parser.parse(&flattened.source, false, style_path)?;
        let optimized = optimize_css(&parsed, css_usage);
        let mut css = render_minified_css(&optimized.style);
        let relative = relative_path(project_root, style_path);
        let output = output_dir.join(&relative);
        let mut source_map_sources = vec![CssSourceMapSource {
            path: &relative,
            source: &source,
            role: CssSourceMapSourceRole::EntryStyle,
        }];
        source_map_sources.extend(flattened.imports.iter().map(|import| CssSourceMapSource {
            path: &import.path,
            source: &import.source,
            role: CssSourceMapSourceRole::FlattenedImport,
        }));
        source_map_sources.extend(flattened.retained_imports.iter().filter_map(|import| {
            let (Some(path), Some(source)) = (&import.path, &import.source) else {
                return None;
            };
            Some(CssSourceMapSource {
                path: path.as_path(),
                source: source.as_str(),
                role: CssSourceMapSourceRole::RetainedImport,
            })
        }));
        let source_map = write_css_source_map(output_dir, &relative, &source_map_sources)?;
        let source_map_linked = append_source_mapping_url(&mut css, &output, &source_map.path);
        write_file(&output, css.as_bytes())?;
        let flattened_imports = flattened
            .imports
            .iter()
            .map(|import| SourceBuildStyleImport {
                specifier: import.specifier.clone(),
                path: normalize_path(&import.path),
                inlined: true,
            })
            .collect();
        let retained_imports = flattened
            .retained_imports
            .iter()
            .map(|import| SourceBuildStyleRetainedImport {
                specifier: import.specifier.clone(),
                path: import.path.as_ref().map(|path| normalize_path(path)),
                condition: import.condition.clone(),
                reason: import.reason.to_string(),
                inlined: false,
            })
            .collect();

        compiled.push(SourceBuildStyle {
            path: normalize_path(&relative),
            output: normalize_path(&relative_path(project_root, &output)),
            source_map_output: Some(normalize_path(&relative_path(
                project_root,
                &source_map.path,
            ))),
            source_map_source_count: source_map_sources.len(),
            source_map_source_hash_count: source_map.source_hash_count,
            source_map_entry_style_source_count: source_map.entry_style_source_count,
            source_map_flattened_import_source_count: source_map.flattened_import_source_count,
            source_map_retained_import_source_count: source_map.retained_import_source_count,
            source_map_segment_count: source_map.segment_count,
            source_map_exact_segment_mapping: source_map.exact_segment_mapping,
            source_map_evidence_only: source_map.evidence_only,
            source_map_linked,
            source_map_hash: Some(source_map.hash.clone()),
            hash: hash_bytes(css.as_bytes()),
            rule_count: optimized.retained_rule_count,
            original_rule_count: optimized.original_rule_count,
            retained_rule_count: optimized.retained_rule_count,
            pruned_rule_count: optimized.pruned_rule_count,
            minified: optimized.minified,
            node_modules_required: false,
            lifecycle_scripts_executed: false,
            source_owned_contract: true,
            external_runtime_required: false,
            external_runtime_executed: false,
            import_count: optimized.style.imports.len(),
            flattened_imports,
            retained_imports,
            asset_references,
        });
    }

    Ok(compiled)
}

fn append_source_mapping_url(
    css: &mut String,
    css_output: &Path,
    source_map_output: &Path,
) -> bool {
    let css_parent = css_output.parent().unwrap_or_else(|| Path::new(""));
    let source_map_parent = source_map_output.parent().unwrap_or_else(|| Path::new(""));
    if css_parent != source_map_parent {
        return false;
    }

    let Some(source_map_file_name) = source_map_output
        .file_name()
        .and_then(|value| value.to_str())
    else {
        return false;
    };

    if !css.ends_with('\n') {
        css.push('\n');
    }
    css.push_str("/*# sourceMappingURL=");
    css.push_str(source_map_file_name);
    css.push_str(" */");
    true
}
