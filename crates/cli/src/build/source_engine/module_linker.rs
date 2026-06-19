use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::ecmascript_analysis::analyze_ecmascript_source;
use super::ecmascript_dynamic_imports::collect_dynamic_imports;
use super::graph::{
    SourceBuildModuleChunk, SourceBuildModuleDependency, SourceBuildRoute, hash_bytes, read_file,
};
use super::module_linker_paths::{
    canonical_project_file, compiler_intrinsic, dependency_kind, import_specifiers,
    is_linkable_source, project_relative, project_root_alias_adapter_boundary_detail,
    resolve_source_import, source_kind, source_owned_adapter_import, source_slug,
};
use super::module_linker_writer::write_module_chunk;
use super::module_resolver_config::{
    RESOLVER_DETAIL_BASE_URL_NODE_MODULES_BOUNDARY,
    RESOLVER_DETAIL_BASE_URL_OUTSIDE_PROJECT_BOUNDARY, RESOLVER_DETAIL_EXTERNAL_ADAPTER_BOUNDARY,
    RESOLVER_DETAIL_EXTERNAL_PACKAGE_BOUNDARY, RESOLVER_DETAIL_LOCAL_NODE_MODULES_BOUNDARY,
    RESOLVER_DETAIL_PACKAGE_EXPORT_BOUNDARY, RESOLVER_DETAIL_PACKAGE_IMPORT_BOUNDARY,
    RESOLVER_DETAIL_SOURCE_ALIAS_BOUNDARY, RESOLVER_DETAIL_SOURCE_ALIAS_UNRESOLVED,
    RESOLVER_DETAIL_SOURCE_OWNED_ADAPTER_BOUNDARY, RESOLVER_SOURCE_ADAPTER_BOUNDARY,
    RESOLVER_SOURCE_BASE_URL_BOUNDARY, RESOLVER_SOURCE_BASE_URL_NODE_MODULES_BOUNDARY,
    RESOLVER_SOURCE_COMPILER_INTRINSIC, RESOLVER_SOURCE_EXTERNAL_PACKAGE_BOUNDARY,
    RESOLVER_SOURCE_LOCAL_NODE_MODULES_BOUNDARY, RESOLVER_SOURCE_PACKAGE_EXPORT_BOUNDARY,
    RESOLVER_SOURCE_PACKAGE_IMPORT_BOUNDARY, RESOLVER_SOURCE_PROJECT_ROOT_ALIAS_BOUNDARY,
    RESOLVER_SOURCE_SOURCE_ALIAS_BOUNDARY, RESOLVER_SOURCE_SOURCE_ALIAS_UNRESOLVED,
    SourceResolverConfig,
};
use super::module_runtime_transform::transform_runtime_source;

#[derive(Debug, Clone)]
pub(super) struct LinkedRouteModules {
    pub entry_chunk_output: Option<String>,
    pub chunks: Vec<SourceBuildModuleChunk>,
    pub node_modules_required: bool,
}

pub(super) fn emit_linked_route_modules(
    project_root: &Path,
    route_dir: &Path,
    route: &SourceBuildRoute,
    resolver_config: &SourceResolverConfig,
) -> DxResult<LinkedRouteModules> {
    let mut linker = ModuleLinker {
        project_root,
        module_dir: route_dir.join("modules"),
        resolver_config,
        chunks: BTreeMap::new(),
        visiting: BTreeSet::new(),
    };
    linker.link_module(&project_root.join(&route.path))?;
    let chunks = linker.chunks.into_values().collect::<Vec<_>>();
    let entry_chunk_output = chunks
        .iter()
        .find(|chunk| chunk.source_path == route.path)
        .map(|chunk| chunk.chunk_output.clone());
    let node_modules_required = chunks.iter().any(|chunk| chunk.node_modules_required);

    Ok(LinkedRouteModules {
        entry_chunk_output,
        chunks,
        node_modules_required,
    })
}

struct ModuleLinker<'a> {
    project_root: &'a Path,
    module_dir: PathBuf,
    resolver_config: &'a SourceResolverConfig,
    chunks: BTreeMap<String, SourceBuildModuleChunk>,
    visiting: BTreeSet<String>,
}

impl ModuleLinker<'_> {
    fn link_module(&mut self, path: &Path) -> DxResult<Option<String>> {
        let path = canonical_project_file(self.project_root, path)?;
        if !is_linkable_source(&path) {
            return Ok(None);
        }

        let relative = project_relative(self.project_root, &path);
        if let Some(chunk) = self.chunks.get(&relative) {
            return Ok(Some(chunk.chunk_output.clone()));
        }

        let bytes = read_file(&path)?;
        let source =
            String::from_utf8(bytes.clone()).map_err(|error| DxError::CompilationError {
                message: error.to_string(),
                file: path.clone(),
                src: None,
                span: None,
            })?;
        let hash = hash_bytes(&bytes);
        let chunk_path = self.chunk_path(&relative, &hash);
        let chunk_output = project_relative(self.project_root, &chunk_path);

        if !self.visiting.insert(relative.clone()) {
            return Ok(Some(chunk_output));
        }

        let mut dependencies = Vec::new();
        for specifier in module_dependency_specifiers(&relative, &source) {
            dependencies.push(self.dependency(
                &path,
                &relative,
                &source,
                &specifier.specifier,
                specifier.dynamic_import,
            )?);
        }
        self.visiting.remove(&relative);

        let node_modules_required = dependencies
            .iter()
            .any(|dependency| dependency.node_modules_required);
        let dependency_runtime_exports = self.dependency_runtime_exports(&dependencies);
        let kind = source_kind(&path);
        let runtime_transform = transform_runtime_source(&kind, &source);
        let ecmascript_analysis =
            analyze_ecmascript_source(&relative, &kind, &source, &runtime_transform.export_names);
        let chunk = SourceBuildModuleChunk {
            source_path: relative.clone(),
            chunk_output: chunk_output.clone(),
            kind,
            hash,
            dependencies,
            browser_executable: true,
            source_transformed: runtime_transform.transformed_source.is_some(),
            transform_kind: runtime_transform.transform_kind.clone(),
            runtime_exports: runtime_transform.export_names.clone(),
            ecmascript_analysis,
            node_modules_required,
        };
        write_module_chunk(
            &chunk_path,
            &chunk,
            &source,
            &runtime_transform,
            &dependency_runtime_exports,
        )?;
        self.chunks.insert(relative, chunk);

        Ok(Some(chunk_output))
    }

    fn dependency(
        &mut self,
        importer: &Path,
        importer_relative: &str,
        importer_source: &str,
        specifier: &str,
        dynamic_import: bool,
    ) -> DxResult<SourceBuildModuleDependency> {
        if let Some(resolved) =
            resolve_source_import(self.project_root, importer, specifier, self.resolver_config)
        {
            let resolved_path_source = resolved.resolver_source.to_string();
            let resolved = canonical_project_file(self.project_root, &resolved.path)?;
            let resolved_path = project_relative(self.project_root, &resolved);
            let chunk_output = self.link_module(&resolved)?;
            return Ok(SourceBuildModuleDependency {
                specifier: specifier.to_string(),
                resolved_path: Some(resolved_path),
                chunk_output,
                kind: dependency_kind(&resolved),
                resolver_source: resolved_path_source,
                resolver_detail: String::new(),
                node_modules_required: false,
            });
        }

        if !specifier.starts_with('.') && !specifier.starts_with("@/") {
            let compiler_intrinsic = compiler_intrinsic(specifier);
            let source_owned_adapter = source_owned_adapter_import(specifier);
            let package_import_boundary = self
                .resolver_config
                .matches_package_import_boundary(specifier);
            let source_alias_boundary = self
                .resolver_config
                .matches_source_alias_boundary(specifier);
            let package_self_reference_boundary = self
                .resolver_config
                .matches_package_self_reference_boundary(specifier);
            let source_alias = self.resolver_config.matches_source_alias(specifier);
            let base_url_node_modules_boundary = self
                .resolver_config
                .matches_base_url_node_modules_boundary(specifier);
            let base_url_boundary = self.resolver_config.matches_base_url_boundary(specifier);
            let package_export_boundary = self
                .resolver_config
                .matches_package_self_reference_namespace(specifier);
            let external_package_boundary = external_package_import(specifier);
            let (kind, resolver_source, resolver_detail) = if compiler_intrinsic {
                ("compiler-intrinsic", RESOLVER_SOURCE_COMPILER_INTRINSIC, "")
            } else if package_import_boundary {
                (
                    "package-import-adapter-boundary",
                    RESOLVER_SOURCE_PACKAGE_IMPORT_BOUNDARY,
                    self.resolver_config
                        .package_import_boundary_detail(specifier)
                        .unwrap_or(RESOLVER_DETAIL_PACKAGE_IMPORT_BOUNDARY),
                )
            } else if source_alias_boundary {
                (
                    "source-alias-adapter-boundary",
                    RESOLVER_SOURCE_SOURCE_ALIAS_BOUNDARY,
                    self.resolver_config
                        .source_alias_boundary_detail(self.project_root, specifier)
                        .unwrap_or(RESOLVER_DETAIL_SOURCE_ALIAS_BOUNDARY),
                )
            } else if package_self_reference_boundary {
                (
                    "package-export-adapter-boundary",
                    RESOLVER_SOURCE_PACKAGE_EXPORT_BOUNDARY,
                    self.resolver_config
                        .package_self_reference_boundary_detail(specifier)
                        .unwrap_or(RESOLVER_DETAIL_PACKAGE_EXPORT_BOUNDARY),
                )
            } else if source_alias {
                (
                    "unresolved-source-alias",
                    RESOLVER_SOURCE_SOURCE_ALIAS_UNRESOLVED,
                    RESOLVER_DETAIL_SOURCE_ALIAS_UNRESOLVED,
                )
            } else if base_url_node_modules_boundary {
                (
                    "base-url-node-modules-adapter-boundary",
                    RESOLVER_SOURCE_BASE_URL_NODE_MODULES_BOUNDARY,
                    RESOLVER_DETAIL_BASE_URL_NODE_MODULES_BOUNDARY,
                )
            } else if base_url_boundary {
                (
                    "base-url-adapter-boundary",
                    RESOLVER_SOURCE_BASE_URL_BOUNDARY,
                    RESOLVER_DETAIL_BASE_URL_OUTSIDE_PROJECT_BOUNDARY,
                )
            } else if package_export_boundary {
                (
                    "package-export-adapter-boundary",
                    RESOLVER_SOURCE_PACKAGE_EXPORT_BOUNDARY,
                    self.resolver_config
                        .package_self_reference_boundary_detail(specifier)
                        .unwrap_or(RESOLVER_DETAIL_PACKAGE_EXPORT_BOUNDARY),
                )
            } else if source_owned_adapter {
                (
                    "source-owned-adapter-boundary",
                    RESOLVER_SOURCE_ADAPTER_BOUNDARY,
                    RESOLVER_DETAIL_SOURCE_OWNED_ADAPTER_BOUNDARY,
                )
            } else if external_package_boundary {
                (
                    "external-package-adapter-boundary",
                    RESOLVER_SOURCE_EXTERNAL_PACKAGE_BOUNDARY,
                    RESOLVER_DETAIL_EXTERNAL_PACKAGE_BOUNDARY,
                )
            } else {
                (
                    "external-adapter-boundary",
                    RESOLVER_SOURCE_ADAPTER_BOUNDARY,
                    RESOLVER_DETAIL_EXTERNAL_ADAPTER_BOUNDARY,
                )
            };
            return Ok(SourceBuildModuleDependency {
                specifier: specifier.to_string(),
                resolved_path: None,
                chunk_output: None,
                kind: kind.to_string(),
                resolver_source: resolver_source.to_string(),
                resolver_detail: resolver_detail.to_string(),
                node_modules_required: false,
            });
        }

        if let Some(project_root_alias_detail) =
            project_root_alias_adapter_boundary_detail(specifier)
        {
            return Ok(SourceBuildModuleDependency {
                specifier: specifier.to_string(),
                resolved_path: None,
                chunk_output: None,
                kind: "project-root-alias-adapter-boundary".to_string(),
                resolver_source: RESOLVER_SOURCE_PROJECT_ROOT_ALIAS_BOUNDARY.to_string(),
                resolver_detail: project_root_alias_detail.to_string(),
                node_modules_required: false,
            });
        }

        if specifier.starts_with("@/") {
            if self
                .resolver_config
                .matches_source_alias_boundary(specifier)
            {
                return Ok(SourceBuildModuleDependency {
                    specifier: specifier.to_string(),
                    resolved_path: None,
                    chunk_output: None,
                    kind: "source-alias-adapter-boundary".to_string(),
                    resolver_source: RESOLVER_SOURCE_SOURCE_ALIAS_BOUNDARY.to_string(),
                    resolver_detail: self
                        .resolver_config
                        .source_alias_boundary_detail(self.project_root, specifier)
                        .unwrap_or(RESOLVER_DETAIL_SOURCE_ALIAS_BOUNDARY)
                        .to_string(),
                    node_modules_required: false,
                });
            }

            if self.resolver_config.matches_source_alias(specifier) {
                return Ok(SourceBuildModuleDependency {
                    specifier: specifier.to_string(),
                    resolved_path: None,
                    chunk_output: None,
                    kind: "unresolved-source-alias".to_string(),
                    resolver_source: RESOLVER_SOURCE_SOURCE_ALIAS_UNRESOLVED.to_string(),
                    resolver_detail: RESOLVER_DETAIL_SOURCE_ALIAS_UNRESOLVED.to_string(),
                    node_modules_required: false,
                });
            }
        }

        if local_import_requires_adapter_boundary(specifier) {
            return Ok(SourceBuildModuleDependency {
                specifier: specifier.to_string(),
                resolved_path: None,
                chunk_output: None,
                kind: "local-node-modules-adapter-boundary".to_string(),
                resolver_source: RESOLVER_SOURCE_LOCAL_NODE_MODULES_BOUNDARY.to_string(),
                resolver_detail: RESOLVER_DETAIL_LOCAL_NODE_MODULES_BOUNDARY.to_string(),
                node_modules_required: false,
            });
        }

        if dynamic_import && (specifier.starts_with('.') || specifier.starts_with("@/")) {
            return Ok(SourceBuildModuleDependency {
                specifier: specifier.to_string(),
                resolved_path: None,
                chunk_output: None,
                kind: "dynamic-import-adapter-boundary".to_string(),
                resolver_source: RESOLVER_SOURCE_ADAPTER_BOUNDARY.to_string(),
                resolver_detail: "dynamic-import-target-unresolved".to_string(),
                node_modules_required: false,
            });
        }

        Err(unresolved_local_import_error(
            importer_relative,
            importer_source,
            specifier,
        ))
    }

    fn chunk_path(&self, relative: &str, hash: &str) -> PathBuf {
        let file_name = format!("{}-{hash}.mjs", source_slug(relative));
        self.module_dir.join(file_name)
    }

    fn dependency_runtime_exports(
        &self,
        dependencies: &[SourceBuildModuleDependency],
    ) -> Vec<Vec<String>> {
        dependencies
            .iter()
            .filter(|dependency| dependency.chunk_output.is_some())
            .map(|dependency| {
                dependency
                    .resolved_path
                    .as_ref()
                    .and_then(|resolved_path| self.chunks.get(resolved_path))
                    .map(|chunk| chunk.runtime_exports.clone())
                    .unwrap_or_default()
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ModuleDependencySpecifier {
    specifier: String,
    dynamic_import: bool,
}

fn module_dependency_specifiers(source_path: &str, source: &str) -> Vec<ModuleDependencySpecifier> {
    let mut specifiers = BTreeMap::new();

    for specifier in import_specifiers(source_path, source) {
        specifiers.insert(specifier, false);
    }
    for dynamic_import in collect_dynamic_imports(source).dynamic_imports {
        if dynamic_import.import_options_present && !dynamic_import.import_options_supported {
            continue;
        }
        specifiers.entry(dynamic_import.specifier).or_insert(true);
    }

    specifiers
        .into_iter()
        .map(|(specifier, dynamic_import)| ModuleDependencySpecifier {
            specifier,
            dynamic_import,
        })
        .collect()
}

fn local_import_requires_adapter_boundary(specifier: &str) -> bool {
    if !(specifier.starts_with('.') || specifier.starts_with('/') || specifier.starts_with('\\')) {
        return false;
    }
    specifier
        .replace('\\', "/")
        .split('/')
        .any(|segment| segment == "node_modules")
}

fn external_package_import(specifier: &str) -> bool {
    if specifier.is_empty()
        || specifier.starts_with('.')
        || specifier.starts_with('/')
        || specifier.starts_with('\\')
        || specifier.starts_with('#')
        || specifier.starts_with("@/")
        || specifier.contains('\\')
    {
        return false;
    }

    let mut segments = specifier.split('/');
    let Some(package) = segments.next() else {
        return false;
    };

    if let Some(scope) = package.strip_prefix('@') {
        let Some(scoped_package) = segments.next() else {
            return false;
        };
        source_owned_package_segment(scope) && source_owned_package_segment(scoped_package)
    } else {
        source_owned_package_segment(package)
    }
}

fn source_owned_package_segment(segment: &str) -> bool {
    !segment.is_empty() && segment != "." && segment != ".." && segment != "node_modules"
}

fn unresolved_local_import_error(
    importer_relative: &str,
    importer_source: &str,
    specifier: &str,
) -> DxError {
    let (offset, length) = import_specifier_span(importer_source, specifier).unwrap_or((0, 1));
    DxError::compilation_error_with_context(
        format!("Cannot resolve local source import `{specifier}` from {importer_relative}."),
        PathBuf::from(importer_relative),
        importer_source,
        offset,
        length,
    )
}

fn import_specifier_span(source: &str, specifier: &str) -> Option<(usize, usize)> {
    for quote in ['"', '\''] {
        let quoted = format!("{quote}{specifier}{quote}");
        if let Some(offset) = source.find(&quoted) {
            return Some((offset + quote.len_utf8(), specifier.len().max(1)));
        }
    }

    source
        .find(specifier)
        .map(|offset| (offset, specifier.len().max(1)))
}
