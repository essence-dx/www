use dx_compiler::delivery::parse_tsx_module;

use crate::next_rust::{NEXT_RUST_VENDOR_COMMIT, NEXT_RUST_VENDOR_ROOT};

use super::ecmascript_dynamic_imports::{
    collect_dynamic_imports, next_is_identifier, previous_is_identifier, skip_quoted_or_comment,
};
use super::graph::{
    SourceBuildEcmascriptAnalysis, SourceBuildEcmascriptCompatibilityReference,
    SourceBuildEcmascriptDirective, SourceBuildEcmascriptDynamicImportAnalysis,
    SourceBuildEcmascriptOutputModel, SourceBuildEcmascriptRuntimeBoundaries, SourceBuildImport,
};

pub fn analyze_ecmascript_source(
    source_path: &str,
    source_kind: &str,
    source: &str,
    runtime_export_names: &[String],
) -> SourceBuildEcmascriptAnalysis {
    let ast = parse_tsx_module(source_path, source);
    let static_imports = ast
        .imports
        .into_iter()
        .map(|import| SourceBuildImport {
            specifier: import.source,
            side_effect_only: import.side_effect_only,
            type_only: import.type_only,
        })
        .collect();
    let export_names = if runtime_export_names.is_empty() {
        exported_names(source)
    } else {
        runtime_export_names.to_vec()
    };
    let dynamic_imports = collect_dynamic_imports(source);
    let dynamic_import_analysis = SourceBuildEcmascriptDynamicImportAnalysis {
        status: dynamic_import_analysis_status(
            dynamic_imports.dynamic_imports.len(),
            dynamic_imports.unresolved_dynamic_imports.len(),
            dynamic_imports.unsupported_dynamic_imports.len(),
        )
        .to_string(),
        static_count: dynamic_imports.dynamic_imports.len(),
        unresolved_count: dynamic_imports.unresolved_dynamic_imports.len(),
        unsupported_count: dynamic_imports.unsupported_dynamic_imports.len(),
        boundary:
            "source-owned dynamic import analysis; static specifiers become evidence, expressions remain unresolved, and unsupported call forms stay as adapter-boundary receipts"
                .to_string(),
    };

    SourceBuildEcmascriptAnalysis {
        schema: "dx.ecmascript.analysis".to_string(),
        schema_revision: 1,
        source_path: source_path.to_string(),
        source_kind: source_kind.to_string(),
        parser_backend: ast.parser_backend.active_backend,
        diagnostics: ast.diagnostics.len(),
        compatibility_reference: SourceBuildEcmascriptCompatibilityReference {
            upstream_crates: vec!["turbopack-ecmascript".to_string()],
            reference_only: true,
            runtime_build_adoption: false,
            public_runtime_dependency: false,
            vendor_root: NEXT_RUST_VENDOR_ROOT.to_string(),
            vendor_commit: NEXT_RUST_VENDOR_COMMIT.to_string(),
            next_transform_references: vec![
                "next-custom-transforms::track_dynamic_imports".to_string(),
                "next-custom-transforms::react_server_components".to_string(),
            ],
            copied_code: false,
        },
        output_model: SourceBuildEcmascriptOutputModel {
            contract: "dx.www.moduleGraph".to_string(),
            compiler_owns_output: true,
            public_architecture: "DX-owned source graph analysis".to_string(),
        },
        runtime_boundaries: SourceBuildEcmascriptRuntimeBoundaries {
            next_runtime_required: false,
            react_runtime_required: false,
            rsc_required: false,
            node_modules_required: false,
        },
        directives: directive_prologue(source),
        static_imports,
        dynamic_imports: dynamic_imports.dynamic_imports,
        unresolved_dynamic_imports: dynamic_imports.unresolved_dynamic_imports,
        unsupported_dynamic_imports: dynamic_imports.unsupported_dynamic_imports,
        dynamic_import_analysis,
        export_names,
        jsx: matches!(source_kind, "tsx" | "jsx") || source.contains("</") || source.contains("/>"),
        top_level_await: has_top_level_await_token(source),
        full_nextjs_parity: false,
        analysis_boundary:
            "Uses vendored Turbopack ECMAScript and selected Next transform behavior as compatibility references while emitting DX-owned source graph receipts."
                .to_string(),
    }
}

fn dynamic_import_analysis_status(
    static_count: usize,
    unresolved_count: usize,
    unsupported_count: usize,
) -> &'static str {
    match (
        static_count > 0,
        unresolved_count > 0,
        unsupported_count > 0,
    ) {
        (_, _, true) => "unsupported-observed",
        (true, true, false) => "static-and-unresolved",
        (true, false, false) => "static-only",
        (false, true, false) => "unresolved-only",
        (false, false, false) => "none-observed",
    }
}

fn directive_prologue(source: &str) -> Vec<SourceBuildEcmascriptDirective> {
    let mut directives = Vec::new();

    for (line_index, line) in source.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }

        let leading_columns = line.len().saturating_sub(trimmed.len());
        let Some((value, rest)) = string_literal_prefix(trimmed) else {
            break;
        };
        let rest = rest.trim_start();
        if !rest.is_empty() && rest != ";" {
            break;
        }

        directives.push(SourceBuildEcmascriptDirective {
            value,
            scope: "module-prologue".to_string(),
            line: line_index + 1,
            column: leading_columns + 1,
        });
    }

    directives
}

fn string_literal_prefix(source: &str) -> Option<(String, &str)> {
    let quote = source.as_bytes().first().copied()?;
    if !matches!(quote, b'"' | b'\'') {
        return None;
    }

    let mut escaped = false;
    for (offset, byte) in source.as_bytes()[1..].iter().copied().enumerate() {
        if escaped {
            escaped = false;
            continue;
        }
        if byte == b'\\' {
            escaped = true;
            continue;
        }
        if byte == quote {
            let end = offset + 1;
            let value = &source[1..end];
            let rest = &source[end + 1..];
            return Some((value.to_string(), rest));
        }
    }

    None
}

fn has_top_level_await_token(source: &str) -> bool {
    let mut index = 0usize;
    let mut brace_depth = 0usize;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;

    while index < source.len() {
        let byte = source.as_bytes()[index];
        if let Some(next) = skip_quoted_or_comment(source, index, byte) {
            index = next;
            continue;
        }

        if brace_depth == 0
            && paren_depth == 0
            && bracket_depth == 0
            && source[index..].starts_with("await")
            && !previous_is_identifier(source, index)
            && !next_is_identifier(source, index + "await".len())
        {
            return true;
        }

        match byte {
            b'{' => brace_depth += 1,
            b'}' => brace_depth = brace_depth.saturating_sub(1),
            b'(' => paren_depth += 1,
            b')' => paren_depth = paren_depth.saturating_sub(1),
            b'[' => bracket_depth += 1,
            b']' => bracket_depth = bracket_depth.saturating_sub(1),
            _ => {}
        }
        index += 1;
    }

    false
}

fn exported_names(source: &str) -> Vec<String> {
    let mut names = Vec::new();
    collect_exported_function_names(source, "export function ", &mut names);
    collect_exported_function_names(source, "export default function ", &mut names);
    collect_exported_const_names(source, &mut names);
    names
}

fn collect_exported_function_names(source: &str, marker: &str, names: &mut Vec<String>) {
    let mut cursor = source;
    while let Some(index) = cursor.find(marker) {
        let after = &cursor[index + marker.len()..];
        let name = read_identifier(after);
        let name_len = name.len();
        if !name.is_empty() && !names.contains(&name) {
            names.push(name);
        }
        cursor = &after[name_len..];
    }
}

fn collect_exported_const_names(source: &str, names: &mut Vec<String>) {
    let mut cursor = source;
    while let Some(index) = cursor.find("export const ") {
        let after = &cursor[index + "export const ".len()..];
        let name = read_identifier(after);
        let name_len = name.len();
        if !name.is_empty() && !names.contains(&name) {
            names.push(name);
        }
        cursor = &after[name_len..];
    }
}

fn read_identifier(source: &str) -> String {
    source
        .chars()
        .take_while(|character| character.is_ascii_alphanumeric() || *character == '_')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::analyze_ecmascript_source;

    #[test]
    fn records_directives_dynamic_imports_and_top_level_await_without_runtime_takeover() {
        let analysis = analyze_ecmascript_source(
            "app/page.tsx",
            "tsx",
            r#""use client";

import type { User } from "../types";
const panel = () => import("../panel");
export const ready = await Promise.resolve(true);
export default function Page() {
  return <main />;
}
"#,
            &[],
        );

        assert_eq!(analysis.schema, "dx.ecmascript.analysis");
        assert_eq!(analysis.directives[0].value, "use client");
        assert_eq!(analysis.static_imports[0].specifier, "../types");
        assert!(analysis.static_imports[0].type_only);
        assert_eq!(analysis.dynamic_imports[0].specifier, "../panel");
        assert!(analysis.top_level_await);
        assert!(analysis.compatibility_reference.reference_only);
        assert!(!analysis.compatibility_reference.runtime_build_adoption);
        assert!(!analysis.compatibility_reference.public_runtime_dependency);
        assert!(!analysis.runtime_boundaries.next_runtime_required);
        assert!(!analysis.runtime_boundaries.react_runtime_required);
        assert!(!analysis.runtime_boundaries.rsc_required);
        assert!(!analysis.runtime_boundaries.node_modules_required);
        assert!(!analysis.full_nextjs_parity);
    }
}
