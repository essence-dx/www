use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};

use dx_compiler::delivery::parse_tsx_module;

use crate::error::{DxError, DxResult};

use super::graph::{normalize_path, relative_path};
use super::module_resolver_config::{
    RESOLVER_DETAIL_PROJECT_ROOT_ALIAS_NODE_MODULES_BOUNDARY,
    RESOLVER_DETAIL_PROJECT_ROOT_ALIAS_OUTSIDE_PROJECT_BOUNDARY,
    RESOLVER_SOURCE_PROJECT_ROOT_ALIAS, RESOLVER_SOURCE_RELATIVE,
    RESOLVER_SOURCE_SRC_PROJECT_ROOT_ALIAS, SourceAliasBase, SourceResolverConfig,
};

#[derive(Debug, Clone)]
pub(super) struct ResolvedSourceImport {
    pub path: PathBuf,
    pub resolver_source: &'static str,
}

pub(super) fn resolve_source_import(
    project_root: &Path,
    importer: &Path,
    specifier: &str,
    resolver_config: &SourceResolverConfig,
) -> Option<ResolvedSourceImport> {
    let bases = if let Some(project_path) = specifier.strip_prefix("@/") {
        let alias_boundary = resolver_config.matches_source_alias_boundary(specifier);
        let mut bases = if alias_boundary {
            Vec::new()
        } else {
            resolver_config.source_alias_bases(project_root, specifier)
        };
        if !alias_boundary {
            append_project_root_alias_fallback_bases(project_root, project_path, &mut bases);
        }
        bases
    } else if specifier.starts_with('.') {
        let importer_dir = importer.parent().unwrap_or_else(|| Path::new(""));
        vec![SourceAliasBase {
            path: importer_dir.join(specifier),
            resolver_source: RESOLVER_SOURCE_RELATIVE,
        }]
    } else if compiler_intrinsic(specifier) {
        Vec::new()
    } else {
        let alias_boundary = resolver_config.matches_source_alias_boundary(specifier)
            || resolver_config.matches_package_import_boundary(specifier)
            || resolver_config.matches_package_self_reference_boundary(specifier);
        let mut bases = if alias_boundary {
            Vec::new()
        } else {
            resolver_config.source_alias_bases(project_root, specifier)
        };
        if !alias_boundary {
            bases.extend(resolver_config.source_base_url_bases(project_root, specifier));
        }
        bases
    };

    bases
        .iter()
        .flat_map(|base| {
            source_import_candidates(&base.path)
                .into_iter()
                .map(move |candidate| ResolvedSourceImport {
                    path: candidate,
                    resolver_source: base.resolver_source,
                })
        })
        .find(|candidate| {
            candidate.path.is_file()
                && is_inside_project(project_root, &candidate.path)
                && !contains_node_modules(&candidate.path)
        })
        .map(|candidate| ResolvedSourceImport {
            path: clean_path(candidate.path),
            resolver_source: candidate.resolver_source,
        })
}

pub(super) fn canonical_project_file(project_root: &Path, candidate: &Path) -> DxResult<PathBuf> {
    let clean = clean_path(candidate.to_path_buf());
    let canonical = candidate.canonicalize().map_err(|error| DxError::IoError {
        path: Some(candidate.to_path_buf()),
        message: error.to_string(),
    })?;
    if !is_inside_project(project_root, &canonical) {
        return Err(DxError::IoError {
            path: Some(candidate.to_path_buf()),
            message: "source module is outside the project".to_string(),
        });
    }
    Ok(clean)
}

pub(super) fn project_relative(project_root: &Path, path: &Path) -> String {
    let root = clean_path(project_root.to_path_buf());
    let path = clean_path(path.to_path_buf());
    normalize_path(&relative_path(&root, &path))
}

pub(super) fn is_linkable_source(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| matches!(extension, "tsx" | "jsx" | "ts" | "js"))
}

pub(super) fn source_kind(path: &Path) -> String {
    path.extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or("source")
        .to_string()
}

pub(super) fn dependency_kind(path: &Path) -> String {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("css") => "style".to_string(),
        Some(extension) => extension.to_string(),
        None => "source".to_string(),
    }
}

pub(super) fn source_slug(relative: &str) -> String {
    relative
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

pub(super) fn import_specifiers(source_path: &str, source: &str) -> Vec<String> {
    let mut specifiers = BTreeSet::new();

    for import in parse_tsx_module(source_path, source).imports {
        if import.type_only {
            continue;
        }
        let has_value_specifier = import.default.is_some()
            || import
                .specifiers
                .iter()
                .any(|specifier| !specifier.type_only);
        if has_value_specifier || import.specifiers.is_empty() {
            specifiers.insert(import.source);
        }
    }
    specifiers.extend(re_export_specifiers(source));

    for line in source.lines().map(str::trim) {
        if !line.starts_with("import ") || line.starts_with("import type ") {
            continue;
        }
        if !fallback_import_statement_has_value(line) {
            continue;
        }
        if let Some((_, after_from)) = line.split_once(" from ") {
            if let Some(specifier) = quoted_specifier(after_from) {
                specifiers.insert(specifier);
            }
            continue;
        }
        if let Some(specifier) = quoted_specifier(line.trim_start_matches("import ")) {
            specifiers.insert(specifier);
        }
    }

    specifiers.into_iter().collect()
}

fn re_export_specifiers(source: &str) -> Vec<String> {
    let mut specifiers = BTreeSet::new();
    let mut statement = String::new();
    let mut collecting = false;

    for line in source.lines().map(str::trim) {
        if line.starts_with("export ") || collecting {
            if !collecting {
                statement.clear();
                collecting = true;
            } else {
                statement.push(' ');
            }
            statement.push_str(line);

            if line.ends_with(';') || statement.contains(" from ") {
                if let Some(specifier) = re_export_specifier(&statement) {
                    specifiers.insert(specifier);
                }
                collecting = false;
            }
        }
    }

    if collecting {
        if let Some(specifier) = re_export_specifier(&statement) {
            specifiers.insert(specifier);
        }
    }

    specifiers.into_iter().collect()
}

fn re_export_specifier(statement: &str) -> Option<String> {
    let rest = statement
        .trim()
        .trim_end_matches(';')
        .strip_prefix("export")?
        .trim_start();
    if rest
        .strip_prefix("type")
        .is_some_and(|after_type| after_type.starts_with(char::is_whitespace))
    {
        return None;
    }

    let after_export_list = if let Some(after_star) = rest.strip_prefix('*') {
        after_star
    } else if let Some(after_brace) = rest.strip_prefix('{') {
        let brace_end = after_brace.find('}')?;
        let specifier_list = &after_brace[..brace_end];
        if !named_re_export_has_value_specifier(specifier_list) {
            return None;
        }
        &after_brace[brace_end + 1..]
    } else {
        return None;
    };

    re_export_source_after_from(after_export_list)
}

fn fallback_import_statement_has_value(statement: &str) -> bool {
    let rest = statement.trim_start_matches("import ").trim_start();
    if matches!(rest.chars().next(), Some('"' | '\'')) {
        return true;
    }

    let Some((before_from, _)) = rest.split_once(" from ") else {
        return true;
    };
    let before_from = before_from.trim();
    if before_from.starts_with('{') && before_from.ends_with('}') {
        return named_specifier_list_has_value(&before_from[1..before_from.len() - 1]);
    }
    true
}

fn named_re_export_has_value_specifier(specifier_list: &str) -> bool {
    named_specifier_list_has_value(specifier_list)
}

fn named_specifier_list_has_value(specifier_list: &str) -> bool {
    specifier_list
        .split(',')
        .map(str::trim)
        .any(|specifier| !specifier.is_empty() && !specifier.starts_with("type "))
}

fn re_export_source_after_from(after_export_list: &str) -> Option<String> {
    let trimmed = after_export_list.trim_start();
    let after_from = trimmed
        .strip_prefix("from")
        .filter(|after_from| after_from.starts_with(char::is_whitespace))
        .or_else(|| {
            trimmed
                .split_once(" from ")
                .map(|(_, after_from)| after_from)
        })?;
    quoted_specifier(after_from)
}

pub(super) fn compiler_intrinsic(specifier: &str) -> bool {
    matches!(
        specifier,
        "next"
            | "react"
            | "react/jsx-runtime"
            | "react/jsx-dev-runtime"
            | "dx-www/react"
            | "next/link"
            | "next/image"
            | "next/server"
            | "next/navigation"
            | "next/headers"
            | "next/cookies"
            | "next/font/google"
            | "next/font/local"
    ) || specifier.starts_with("node:")
}

pub(super) fn source_owned_adapter_import(specifier: &str) -> bool {
    matches!(
        specifier,
        "ai" | "@ai-sdk/openai"
            | "@instantdb/react"
            | "@mdx-js/mdx"
            | "@mdx-js/react"
            | "@tanstack/react-query"
            | "mdx/types"
            | "motion/react"
            | "next-intl"
            | "react-markdown"
            | "stripe"
            | "zod"
            | "better-auth/react"
            | "collections/server"
    ) || specifier.starts_with("better-auth/")
        || specifier.starts_with("fumadocs-core/")
        || specifier.starts_with("fumadocs-openapi/")
        || specifier.starts_with("fumadocs-ui/")
        || specifier.starts_with("next-intl/")
}

fn source_import_candidates(base: &Path) -> Vec<PathBuf> {
    let mut candidates = vec![base.to_path_buf()];
    for extension in ["tsx", "jsx", "ts", "js", "css"] {
        candidates.push(base.with_extension(extension));
    }
    for extension in ["tsx", "jsx", "ts", "js"] {
        candidates.push(base.join(format!("index.{extension}")));
    }
    candidates
}

fn append_project_root_alias_fallback_bases(
    project_root: &Path,
    project_path: &str,
    bases: &mut Vec<SourceAliasBase>,
) {
    if !source_owned_project_root_alias_path(project_path) {
        return;
    }
    bases.push(SourceAliasBase {
        path: project_root.join(project_path),
        resolver_source: RESOLVER_SOURCE_PROJECT_ROOT_ALIAS,
    });
    if let Some(src_base) = src_project_root_alias_base(project_root, project_path) {
        bases.push(src_base);
    }
}

pub(super) fn project_root_alias_adapter_boundary_detail(specifier: &str) -> Option<&'static str> {
    let project_path = specifier.strip_prefix("@/")?;
    project_root_alias_path_boundary_detail(project_path)
}

fn src_project_root_alias_base(project_root: &Path, project_path: &str) -> Option<SourceAliasBase> {
    if !project_root.join("src").join("app").is_dir() || !source_owned_alias_path(project_path) {
        return None;
    }
    Some(SourceAliasBase {
        path: project_root.join("src").join(project_path),
        resolver_source: RESOLVER_SOURCE_SRC_PROJECT_ROOT_ALIAS,
    })
}

fn source_owned_project_root_alias_path(project_path: &str) -> bool {
    project_root_alias_path_boundary_detail(project_path).is_none()
}

fn project_root_alias_path_boundary_detail(project_path: &str) -> Option<&'static str> {
    let normalized = project_path.replace('\\', "/");
    let mut saw_segment = false;
    for segment in normalized.split('/') {
        if segment == "node_modules" {
            return Some(RESOLVER_DETAIL_PROJECT_ROOT_ALIAS_NODE_MODULES_BOUNDARY);
        }
        if segment.is_empty() || segment == "." || segment == ".." {
            return Some(RESOLVER_DETAIL_PROJECT_ROOT_ALIAS_OUTSIDE_PROJECT_BOUNDARY);
        }
        saw_segment = true;
    }
    if saw_segment {
        None
    } else {
        Some(RESOLVER_DETAIL_PROJECT_ROOT_ALIAS_OUTSIDE_PROJECT_BOUNDARY)
    }
}

fn source_owned_alias_path(project_path: &str) -> bool {
    if !source_owned_project_root_alias_path(project_path) {
        return false;
    }
    let normalized = project_path.replace('\\', "/");
    let mut segments = normalized.split('/');
    let Some(first) = segments.next() else {
        return false;
    };
    if first == "src" {
        return false;
    }
    true
}

fn clean_path(path: PathBuf) -> PathBuf {
    let mut clean = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                clean.pop();
            }
            _ => clean.push(component.as_os_str()),
        }
    }
    clean
}

fn is_inside_project(project_root: &Path, candidate: &Path) -> bool {
    let Ok(root) = project_root.canonicalize() else {
        return false;
    };
    let Ok(candidate) = candidate.canonicalize() else {
        return false;
    };
    candidate.starts_with(root)
}

fn contains_node_modules(path: &Path) -> bool {
    path.components()
        .any(|component| component.as_os_str() == OsStr::new("node_modules"))
}

fn quoted_specifier(value: &str) -> Option<String> {
    let mut chars = value.char_indices();
    let (start, quote) = chars.find(|(_, character)| matches!(character, '"' | '\''))?;
    let end = value[start + quote.len_utf8()..]
        .find(quote)
        .map(|offset| start + quote.len_utf8() + offset)?;
    Some(value[start + quote.len_utf8()..end].to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use serde_json::json;

    use super::super::module_resolver_config::RESOLVER_SOURCE_FORGE_SOURCE_MANIFEST;
    use super::*;

    #[test]
    fn import_specifiers_keep_value_edges_and_skip_type_only_edges() {
        let source = r#"
import type { Config } from "./types";
import { type Shape } from "./shape";
import "./side-effect";
import value from "./value";
export type { PublicConfig } from "./public-types";
export { type Meta } from "./meta";
export { createStore } from "./store";
export * from "./all";
"#;

        let specifiers = import_specifiers("app/page.tsx", source);

        assert_eq!(
            specifiers,
            vec!["./all", "./side-effect", "./store", "./value"]
        );
    }

    #[test]
    fn resolve_source_import_uses_forge_source_manifest_bare_package_aliases() {
        let project = tempfile::tempdir().expect("temp project");
        let root = project.path();
        let page_path = root.join("app/page.tsx");
        let index_path = root.join("lib/forge/npm/three/index.ts");
        let vector_path = root.join("lib/forge/npm/three/math/vector3.ts");
        fs::create_dir_all(page_path.parent().expect("page dir")).expect("page dir");
        fs::create_dir_all(index_path.parent().expect("index dir")).expect("index dir");
        fs::create_dir_all(vector_path.parent().expect("vector dir")).expect("vector dir");
        fs::create_dir_all(root.join(".dx/forge")).expect("manifest dir");
        fs::write(&page_path, "import { Vector3 } from \"three\";\n").expect("page");
        fs::write(&index_path, "export { Vector3 } from \"./math/vector3\";\n").expect("index");
        fs::write(&vector_path, "export class Vector3 {}\n").expect("vector");
        fs::write(
            root.join(".dx/forge/source-manifest.json"),
            serde_json::to_vec_pretty(&json!({
                "version": 1,
                "packages": [{
                    "package_id": "npm/three",
                    "upstream_name": "npm:three",
                    "files": [
                        {
                            "path": "lib/forge/npm/three/index.ts",
                            "hash": "index-hash",
                            "bytes": 43
                        },
                        {
                            "path": "lib/forge/npm/three/math/vector3.ts",
                            "hash": "vector-hash",
                            "bytes": 24
                        }
                    ]
                }],
                "receipts": ["2026-06-09-three.json"]
            }))
            .expect("manifest json"),
        )
        .expect("manifest");

        let resolver_config = SourceResolverConfig::load(root).expect("resolver config");
        let root_import = resolve_source_import(root, &page_path, "three", &resolver_config)
            .expect("root package import");
        let subpath_import =
            resolve_source_import(root, &page_path, "three/math/vector3", &resolver_config)
                .expect("package subpath import");

        assert_eq!(root_import.path, index_path);
        assert_eq!(
            root_import.resolver_source,
            RESOLVER_SOURCE_FORGE_SOURCE_MANIFEST
        );
        assert_eq!(subpath_import.path, vector_path);
        assert_eq!(
            subpath_import.resolver_source,
            RESOLVER_SOURCE_FORGE_SOURCE_MANIFEST
        );
        assert!(
            resolve_source_import(root, &page_path, "lodash", &resolver_config).is_none(),
            "untracked bare packages must remain outside the source-owned resolver"
        );
    }
}
