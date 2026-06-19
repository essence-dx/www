use serde::{Deserialize, Serialize};

use super::jsx_lowering::lower_react_jsx_source;

/// Import resolver settings for strict React-shaped DX-WWW source.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactImportResolverConfig {
    /// Local alias prefixes such as `@/`.
    pub aliases: Vec<DxReactImportAlias>,
    /// Forge-owned files recorded by package manifests.
    pub forge_files: Vec<DxReactForgeOwnedFile>,
    /// Reviewed adapters produced by `dx forge import`.
    pub reviewed_adapters: Vec<DxReactReviewedAdapter>,
    /// Whether unresolved bare imports should be blocked from `node_modules`.
    pub strict_no_node_modules: bool,
}

/// One local import alias.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactImportAlias {
    /// Import prefix.
    pub prefix: String,
    /// Project-relative target root.
    pub target_root: String,
}

/// One Forge-owned importable file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactForgeOwnedFile {
    /// Import specifier exposed by the package or adapter.
    pub import_specifier: String,
    /// Project-relative materialized source path.
    pub source_path: String,
    /// Forge package id.
    pub package_id: String,
}

/// Reviewed JavaScript-family adapter that can satisfy an import without `node_modules`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactReviewedAdapter {
    /// Import specifier exposed by the adapter.
    pub package_name: String,
    /// Project-relative adapter path.
    pub adapter_path: String,
    /// Forge package id.
    pub package_id: String,
    /// Whether the adapter passed review.
    pub reviewed: bool,
}

/// Resolved import category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxReactImportResolutionKind {
    /// Relative local source file.
    RelativeLocal,
    /// Local source through a configured alias.
    AliasLocal,
    /// Forge-owned source file.
    ForgeOwned,
    /// Reviewed adapter for source-owned package compatibility.
    ReviewedAdapter,
    /// Compiler-owned intrinsic such as React authoring primitives.
    CompilerIntrinsic,
    /// Bare import blocked in strict mode because it would require `node_modules`.
    BlockedNodeModules,
    /// Import could not be resolved by the strict resolver.
    Unresolved,
}

/// One import resolution result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactResolvedImport {
    /// Importing file.
    pub importer_path: String,
    /// Original import specifier.
    pub specifier: String,
    /// Resolution category.
    pub kind: DxReactImportResolutionKind,
    /// Project-relative resolved file path, when known.
    pub resolved_path: Option<String>,
    /// Owning Forge package id, when known.
    pub package_id: Option<String>,
    /// Whether this import would require a `node_modules` lookup.
    pub requires_node_modules: bool,
}

/// Resolve React-shaped imports under the strict DX-WWW project contract.
pub fn resolve_react_imports(
    source_path: &str,
    source: &str,
    config: DxReactImportResolverConfig,
) -> Vec<DxReactResolvedImport> {
    lower_react_jsx_source(source_path, source)
        .imports
        .into_iter()
        .map(|import| resolve_import(source_path, &import.source, &config))
        .collect()
}

fn resolve_import(
    importer_path: &str,
    specifier: &str,
    config: &DxReactImportResolverConfig,
) -> DxReactResolvedImport {
    if specifier.starts_with('.') {
        return resolved(
            importer_path,
            specifier,
            DxReactImportResolutionKind::RelativeLocal,
            Some(normalize_relative_import(importer_path, specifier)),
            None,
            false,
        );
    }

    if let Some(alias) = config
        .aliases
        .iter()
        .find(|alias| specifier.starts_with(&alias.prefix))
    {
        let suffix = specifier.trim_start_matches(&alias.prefix);
        return resolved(
            importer_path,
            specifier,
            DxReactImportResolutionKind::AliasLocal,
            Some(join_project_path(&alias.target_root, suffix)),
            None,
            false,
        );
    }

    if let Some(file) = config
        .forge_files
        .iter()
        .find(|file| file.import_specifier == specifier)
    {
        return resolved(
            importer_path,
            specifier,
            DxReactImportResolutionKind::ForgeOwned,
            Some(file.source_path.clone()),
            Some(file.package_id.clone()),
            false,
        );
    }

    if let Some(adapter) = config
        .reviewed_adapters
        .iter()
        .find(|adapter| adapter.reviewed && adapter.package_name == specifier)
    {
        return resolved(
            importer_path,
            specifier,
            DxReactImportResolutionKind::ReviewedAdapter,
            Some(adapter.adapter_path.clone()),
            Some(adapter.package_id.clone()),
            false,
        );
    }

    if compiler_intrinsic(specifier) {
        return resolved(
            importer_path,
            specifier,
            DxReactImportResolutionKind::CompilerIntrinsic,
            None,
            None,
            false,
        );
    }

    let kind = if config.strict_no_node_modules {
        DxReactImportResolutionKind::BlockedNodeModules
    } else {
        DxReactImportResolutionKind::Unresolved
    };
    resolved(importer_path, specifier, kind, None, None, true)
}

fn resolved(
    importer_path: &str,
    specifier: &str,
    kind: DxReactImportResolutionKind,
    resolved_path: Option<String>,
    package_id: Option<String>,
    requires_node_modules: bool,
) -> DxReactResolvedImport {
    DxReactResolvedImport {
        importer_path: importer_path.to_string(),
        specifier: specifier.to_string(),
        kind,
        resolved_path,
        package_id,
        requires_node_modules,
    }
}

fn normalize_relative_import(importer_path: &str, specifier: &str) -> String {
    let importer = importer_path.replace('\\', "/");
    let parent = importer
        .rsplit_once('/')
        .map(|(parent, _)| parent)
        .unwrap_or("");
    join_project_path(parent, specifier)
}

fn join_project_path(prefix: &str, suffix: &str) -> String {
    let mut parts = Vec::new();
    let combined = if prefix.is_empty() {
        suffix.replace('\\', "/")
    } else {
        format!("{}/{}", prefix.trim_matches('/'), suffix.replace('\\', "/"))
    };
    for part in combined.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            value => parts.push(value),
        }
    }
    parts.join("/")
}

fn compiler_intrinsic(specifier: &str) -> bool {
    matches!(
        specifier,
        "react"
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
    )
}
