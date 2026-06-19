fn source_package_with_config(
    package_id: &str,
    config: &DxForgeProjectConfig,
    variant: &str,
) -> Result<DxSourcePackage> {
    let variant = validate_source_variant(variant)?;
    let package = registry_package(package_id)?;
    let files = package
        .files
        .iter()
        .map(|file| {
            let default_path = config.materialize_path(&file.path)?;
            let path = materialized_variant_path(&file.path, &default_path, &variant)?;
            let content = file
                .content
                .as_deref()
                .map(|content| {
                    materialized_file_content(&file.path, &path, content, config, &variant)
                })
                .transpose()?;
            let hash = content
                .as_deref()
                .map(|content| hash_bytes(content.as_bytes()))
                .unwrap_or_else(|| file.hash.clone());
            let bytes = content
                .as_deref()
                .map(|content| content.len() as u64)
                .unwrap_or(file.bytes);
            Ok(DxSourceFile {
                path,
                logical_path: Some(file.path.clone()),
                hash,
                bytes,
                content,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let integrity_hash = package_integrity_hash(&files);

    let package_id = package.package_id;
    let upstream_name = registry_upstream_name(&package_id).to_string();
    let generator = registry_generator_name(&package_id).to_string();

    Ok(DxSourcePackage {
        package_id,
        upstream_name,
        version: package.version,
        generator,
        variant,
        last_accepted_update: None,
        rollback_receipt: None,
        source_kind: package.source_kind,
        integrity_hash,
        license: package.license,
        provenance: package.provenance,
        advisory_review: package.advisory_review,
        license_review: package.license_review,
        files,
    })
}

fn materialized_file_content(
    logical_path: &str,
    materialized_path: &str,
    content: &str,
    config: &DxForgeProjectConfig,
    variant: &str,
) -> Result<String> {
    if logical_path == "js/ui/button.tsx" {
        let default_utils_path = config.materialize_path("js/lib/utils.ts")?;
        let utils_path =
            materialized_variant_path("js/lib/utils.ts", &default_utils_path, variant)?;
        let utils_import = relative_module_import(materialized_path, &utils_path)?;
        return Ok(content.replace("../../lib/utils", &utils_import));
    }

    if logical_path == "js/ui/badge.tsx" {
        let default_utils_path = config.materialize_path("js/lib/utils.ts")?;
        let utils_path =
            materialized_variant_path("js/lib/utils.ts", &default_utils_path, variant)?;
        let utils_import = relative_module_import(materialized_path, &utils_path)?;
        return Ok(content.replace("../../lib/utils", &utils_import));
    }

    if logical_path == "js/ui/card.tsx" {
        let default_utils_path = config.materialize_path("js/lib/utils.ts")?;
        let utils_path =
            materialized_variant_path("js/lib/utils.ts", &default_utils_path, variant)?;
        let utils_import = relative_module_import(materialized_path, &utils_path)?;
        return Ok(content.replace("../../lib/utils", &utils_import));
    }

    if logical_path == "js/ui/alert.tsx" {
        let default_utils_path = config.materialize_path("js/lib/utils.ts")?;
        let utils_path =
            materialized_variant_path("js/lib/utils.ts", &default_utils_path, variant)?;
        let utils_import = relative_module_import(materialized_path, &utils_path)?;
        return Ok(content.replace("../../lib/utils", &utils_import));
    }

    if logical_path == "js/ui/avatar.tsx" {
        let default_utils_path = config.materialize_path("js/lib/utils.ts")?;
        let utils_path =
            materialized_variant_path("js/lib/utils.ts", &default_utils_path, variant)?;
        let utils_import = relative_module_import(materialized_path, &utils_path)?;
        return Ok(content.replace("../../lib/utils", &utils_import));
    }

    if logical_path == "js/ui/skeleton.tsx" {
        let default_utils_path = config.materialize_path("js/lib/utils.ts")?;
        let utils_path =
            materialized_variant_path("js/lib/utils.ts", &default_utils_path, variant)?;
        let utils_import = relative_module_import(materialized_path, &utils_path)?;
        return Ok(content.replace("../../lib/utils", &utils_import));
    }

    if logical_path == "js/ui/label.tsx" {
        let default_utils_path = config.materialize_path("js/lib/utils.ts")?;
        let utils_path =
            materialized_variant_path("js/lib/utils.ts", &default_utils_path, variant)?;
        let utils_import = relative_module_import(materialized_path, &utils_path)?;
        return Ok(content.replace("../../lib/utils", &utils_import));
    }

    if logical_path == "js/ui/separator.tsx" {
        let default_utils_path = config.materialize_path("js/lib/utils.ts")?;
        let utils_path =
            materialized_variant_path("js/lib/utils.ts", &default_utils_path, variant)?;
        let utils_import = relative_module_import(materialized_path, &utils_path)?;
        return Ok(content.replace("../../lib/utils", &utils_import));
    }

    if logical_path == "js/ui/field.tsx" {
        let default_utils_path = config.materialize_path("js/lib/utils.ts")?;
        let utils_path =
            materialized_variant_path("js/lib/utils.ts", &default_utils_path, variant)?;
        let utils_import = relative_module_import(materialized_path, &utils_path)?;
        return Ok(content.replace("../../lib/utils", &utils_import));
    }

    if logical_path == "js/ui/item.tsx" {
        let default_utils_path = config.materialize_path("js/lib/utils.ts")?;
        let utils_path =
            materialized_variant_path("js/lib/utils.ts", &default_utils_path, variant)?;
        let utils_import = relative_module_import(materialized_path, &utils_path)?;
        return Ok(content.replace("../../lib/utils", &utils_import));
    }

    if logical_path == "js/ui/input.tsx" {
        let default_utils_path = config.materialize_path("js/lib/utils.ts")?;
        let utils_path =
            materialized_variant_path("js/lib/utils.ts", &default_utils_path, variant)?;
        let utils_import = relative_module_import(materialized_path, &utils_path)?;
        return Ok(content.replace("../../lib/utils", &utils_import));
    }

    if logical_path == "js/ui/textarea.tsx" {
        let default_utils_path = config.materialize_path("js/lib/utils.ts")?;
        let utils_path =
            materialized_variant_path("js/lib/utils.ts", &default_utils_path, variant)?;
        let utils_import = relative_module_import(materialized_path, &utils_path)?;
        return Ok(content.replace("../../lib/utils", &utils_import));
    }

    if logical_path == "js/icons/search.tsx" {
        let default_helper_path = config.materialize_path("js/lib/icons.ts")?;
        let helper_path =
            materialized_variant_path("js/lib/icons.ts", &default_helper_path, variant)?;
        let helper_import = relative_module_import(materialized_path, &helper_path)?;
        return Ok(content.replace("../../lib/icons", &helper_import));
    }

    if logical_path == "js/app/api/trpc/[trpc]/route.ts" {
        let default_handler_path = config.materialize_path("js/lib/trpc/route-handler.ts")?;
        let handler_path = materialized_variant_path(
            "js/lib/trpc/route-handler.ts",
            &default_handler_path,
            variant,
        )?;
        let handler_import = relative_module_import(materialized_path, &handler_path)?;
        return Ok(content.replace("DX_TRPC_ROUTE_HANDLER_IMPORT", &handler_import));
    }

    if logical_path == "js/three/index.ts" {
        let default_scene_index_path = config.materialize_path("js/scene/index.ts")?;
        let scene_index_path =
            materialized_variant_path("js/scene/index.ts", &default_scene_index_path, variant)?;
        let scene_index_import = relative_module_import(materialized_path, &scene_index_path)?;
        return Ok(content.replace("DX_THREE_SCENE_INDEX_IMPORT", &scene_index_import));
    }

    Ok(content.to_string())
}

/// Validate and normalize a source-owned package variant name.
pub fn validate_source_variant(variant: &str) -> Result<String> {
    let variant = variant.trim();
    if variant.is_empty() {
        bail!("Forge package variant cannot be empty");
    }
    if variant.len() > 64 {
        bail!("Forge package variant `{variant}` is too long");
    }
    if variant == "." || variant == ".." || variant.contains("..") {
        bail!("Forge package variant `{variant}` cannot contain `..`");
    }
    if variant.contains('/') || variant.contains('\\') {
        bail!("Forge package variant `{variant}` cannot contain path separators");
    }
    if !variant
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
    {
        bail!(
            "Forge package variant `{variant}` must use ASCII letters, numbers, `-`, `_`, or `.`"
        );
    }
    Ok(variant.to_string())
}

fn materialized_variant_path(
    logical_path: &str,
    default_path: &str,
    variant: &str,
) -> Result<String> {
    let variant = validate_source_variant(variant)?;
    if variant == "default" {
        validate_project_relative_path(default_path)?;
        return Ok(default_path.to_string());
    }

    let logical_path = logical_path.replace('\\', "/");
    let default_path = default_path.replace('\\', "/");
    let (logical_prefix, namespace) =
        if logical_path == "js/ui" || logical_path.starts_with("js/ui/") {
            ("js/ui", "variants")
        } else if logical_path == "js/icons" || logical_path.starts_with("js/icons/") {
            ("js/icons", "variants")
        } else if logical_path == "js/i18n" || logical_path.starts_with("js/i18n/") {
            ("js/i18n", "forge/variants")
        } else if logical_path == "js/auth" || logical_path.starts_with("js/auth/") {
            ("js/auth", "forge/variants")
        } else if logical_path == "js/app" || logical_path.starts_with("js/app/") {
            ("js/app", "forge/variants")
        } else if logical_path == "js/components" || logical_path.starts_with("js/components/") {
            ("js/components", "forge/variants")
        } else if logical_path == "js/content" || logical_path.starts_with("js/content/") {
            ("js/content", "forge/variants")
        } else if logical_path == "js/db" || logical_path.starts_with("js/db/") {
            ("js/db", "forge/variants")
        } else if logical_path == "js/forms" || logical_path.starts_with("js/forms/") {
            ("js/forms", "forge/variants")
        } else if logical_path == "js/lib" || logical_path.starts_with("js/lib/") {
            ("js/lib", "forge/variants")
        } else if logical_path == "js/instant" || logical_path.starts_with("js/instant/") {
            ("js/instant", "forge/variants")
        } else if logical_path == "js/motion" || logical_path.starts_with("js/motion/") {
            ("js/motion", "forge/variants")
        } else if logical_path == "js/payments" || logical_path.starts_with("js/payments/") {
            ("js/payments", "forge/variants")
        } else if logical_path == "js/query" || logical_path.starts_with("js/query/") {
            ("js/query", "forge/variants")
        } else if logical_path == "js/scene" || logical_path.starts_with("js/scene/") {
            ("js/scene", "forge/variants")
        } else if logical_path == "js/three" || logical_path.starts_with("js/three/") {
            ("js/three", "forge/variants")
        } else if logical_path == "js/root" || logical_path.starts_with("js/root/") {
            ("js/root", "forge/variants")
        } else if logical_path == "js/state" || logical_path.starts_with("js/state/") {
            ("js/state", "forge/variants")
        } else if logical_path == "js/styles" || logical_path.starts_with("js/styles/") {
            ("js/styles", "forge/variants")
        } else if logical_path == "js/supabase" || logical_path.starts_with("js/supabase/") {
            ("js/supabase", "forge/variants")
        } else if logical_path == "js/validation" || logical_path.starts_with("js/validation/") {
            ("js/validation", "forge/variants")
        } else if logical_path == "js/wasm" || logical_path.starts_with("js/wasm/") {
            ("js/wasm", "forge/variants")
        } else {
            bail!("Forge variant path does not support logical file `{logical_path}`");
        };

    let suffix = logical_path
        .trim_start_matches(logical_prefix)
        .trim_start_matches('/');
    let base_len = default_path
        .len()
        .checked_sub(suffix.len())
        .context("variant path suffix is longer than default path")?;
    let base = default_path[..base_len].trim_end_matches('/');
    let variant_root = if base.is_empty() {
        namespace.to_string()
    } else {
        format!("{base}/{namespace}")
    };
    let variant_path = if suffix.is_empty() {
        format!("{variant_root}/{variant}")
    } else {
        format!("{variant_root}/{variant}/{suffix}")
    };
    validate_project_relative_path(&variant_path)?;
    Ok(variant_path)
}

fn registry_upstream_name(package_id: &str) -> &'static str {
    match canonical_package_id(package_id) {
        "shadcn/ui/button"
        | "shadcn/ui/badge"
        | "shadcn/ui/card"
        | "shadcn/ui/alert"
        | "shadcn/ui/avatar"
        | "shadcn/ui/skeleton"
        | "shadcn/ui/label"
        | "shadcn/ui/separator"
        | "shadcn/ui/field"
        | "shadcn/ui/item"
        | "shadcn/ui/input"
        | "shadcn/ui/textarea" => "shadcn-ui",
        "dx/icon/search" => "@dx/forge-icons",
        "auth/better-auth" => "better-auth",
        "animation/motion" => "motion",
        "i18n/next-intl" => "next-intl",
        "tanstack/query" => "@tanstack/react-query",
        "reactive/store" => "@tanstack/store",
        "validation/zod" => "zod",
        "forms/react-hook-form" => "react-hook-form",
        "payments/stripe-js" => "@stripe/stripe-js",
        "automations/n8n" => "n8n-nodes-base",
        "state/zustand" => "zustand",
        "ai/vercel-ai" => "ai",
        "api/trpc" => "@trpc/server",
        "content/fumadocs-next" => "fumadocs",
        "content/react-markdown" => "react-markdown",
        "supabase/client" => "@dx/forge-supabase",
        "db/drizzle-sqlite" => "drizzle-orm",
        "instantdb/react" => "@instantdb/react",
        "wasm/bindgen" => "wasm-bindgen",
        "3d/launch-scene" => "three",
        "migration/static-site" => "@dx/forge-migrations",
        _ => "@dx/forge",
    }
}

fn registry_generator_name(package_id: &str) -> &'static str {
    match canonical_package_id(package_id) {
        "shadcn/ui/button"
        | "shadcn/ui/badge"
        | "shadcn/ui/card"
        | "shadcn/ui/alert"
        | "shadcn/ui/avatar"
        | "shadcn/ui/skeleton"
        | "shadcn/ui/label"
        | "shadcn/ui/separator"
        | "shadcn/ui/field"
        | "shadcn/ui/item"
        | "shadcn/ui/input"
        | "shadcn/ui/textarea" => "dx-forge/ui-components",
        "dx/icon/search" => "dx-forge/selected-icons",
        "auth/better-auth" => "dx-forge/better-auth",
        "animation/motion" => "dx-forge/motion",
        "i18n/next-intl" => "dx-forge/next-intl",
        "tanstack/query" => "dx-forge/tanstack-query",
        "reactive/store" => "dx-forge/reactive-store",
        "validation/zod" => "dx-forge/zod",
        "forms/react-hook-form" => "dx-forge/react-hook-form",
        "payments/stripe-js" => "dx-forge/stripe-js",
        "automations/n8n" => "dx-forge/n8n-automations",
        "state/zustand" => "dx-forge/zustand",
        "ai/vercel-ai" => "dx-forge/vercel-ai",
        "api/trpc" => "dx-forge/trpc",
        "content/fumadocs-next" => "dx-forge/fumadocs",
        "content/react-markdown" => "dx-forge/react-markdown",
        "supabase/client" => "dx-forge/supabase",
        "db/drizzle-sqlite" => "dx-forge/drizzle",
        "instantdb/react" => "dx-forge/instantdb",
        "wasm/bindgen" => "dx-forge/wasm-bindgen",
        "3d/launch-scene" => "dx-forge/three-scene",
        "migration/static-site" => "dx-forge/migrations",
        _ => "dx-forge",
    }
}

fn relative_module_import(from_file: &str, to_file: &str) -> Result<String> {
    validate_project_relative_path(from_file)?;
    validate_project_relative_path(to_file)?;

    let from_parent = path_parent_parts(from_file);
    let to_module = strip_module_extension(to_file);
    let to_parts = path_parts(&to_module);
    let common = from_parent
        .iter()
        .zip(to_parts.iter())
        .take_while(|(left, right)| left == right)
        .count();

    let mut parts = Vec::new();
    parts.extend(std::iter::repeat_n(
        "..".to_string(),
        from_parent.len().saturating_sub(common),
    ));
    parts.extend(to_parts[common..].iter().cloned());

    if parts.is_empty() {
        return Ok(".".to_string());
    }

    let joined = parts.join("/");
    if joined.starts_with("..") {
        Ok(joined)
    } else {
        Ok(format!("./{joined}"))
    }
}

fn path_parent_parts(path: &str) -> Vec<String> {
    let normalized = path.replace('\\', "/");
    let Some((parent, _)) = normalized.rsplit_once('/') else {
        return Vec::new();
    };
    path_parts(parent)
}

fn path_parts(path: &str) -> Vec<String> {
    path.split('/')
        .filter(|part| !part.is_empty())
        .map(str::to_string)
        .collect()
}

fn strip_module_extension(path: &str) -> String {
    for extension in [".tsx", ".ts", ".jsx", ".js"] {
        if let Some(stripped) = path.strip_suffix(extension) {
            return stripped.to_string();
        }
    }
    path.to_string()
}

#[allow(clippy::too_many_arguments)]
fn build_registry_package(
    package_id: &str,
    aliases: Vec<String>,
    language: DxForgeLanguage,
    version: &str,
    source: DxForgeRegistrySource,
    license: &str,
    description: &str,
    source_files: Vec<(&str, &str)>,
) -> DxForgeRegistryPackage {
    let files = source_files
        .into_iter()
        .map(|(path, content)| DxSourceFile {
            path: path.to_string(),
            logical_path: Some(path.to_string()),
            hash: hash_bytes(content.as_bytes()),
            bytes: content.len() as u64,
            content: Some(content.to_string()),
        })
        .collect::<Vec<_>>();
    let integrity_hash = package_integrity_hash(&files);

    DxForgeRegistryPackage {
        package_id: package_id.to_string(),
        aliases,
        language,
        version: version.to_string(),
        source,
        source_kind: DxSourceKind::CuratedRegistry,
        license: license.to_string(),
        description: description.to_string(),
        provenance: curated_registry_provenance(package_id),
        advisory_review: curated_advisory_review(package_id),
        license_review: declared_license_review(license),
        exports: Vec::new(),
        default_exports: Vec::new(),
        allow_selective_imports: false,
        files,
        integrity_hash,
    }
}

fn reactive_store_registry_package() -> DxForgeRegistryPackage {
    let mut package = build_registry_package(
        "reactive/store",
        vec![
            "reactive-store".to_string(),
            "@tanstack/store".to_string(),
            "@tanstack/react-store".to_string(),
            "tanstack-store".to_string(),
        ],
        DxForgeLanguage::Js,
        REACTIVE_STORE_VERSION,
        DxForgeRegistrySource::Curated,
        "MIT",
        "Source-owned Reactive Store slice for Store, ReadonlyStore, createStore, createAtom, createAsyncAtom, batch, shallow comparison, React selector hooks, typed React context transport, metadata receipts, and app-owned state boundaries.",
        reactive_store_templates(),
    );
    package.allow_selective_imports = true;
    package.default_exports = vec!["full".to_string()];
    package.exports = vec![
        reactive_store_export(
            "full",
            &[
                "js/state/reactive-store/index.ts",
                "js/state/reactive-store/types.ts",
                "js/state/reactive-store/atom.ts",
                "js/state/reactive-store/store.ts",
                "js/state/reactive-store/shallow.ts",
                "js/state/reactive-store/react.ts",
                "js/state/reactive-store/context.tsx",
                "js/state/reactive-store/metadata.ts",
                "js/state/reactive-store/README.md",
            ],
        ),
        reactive_store_export(
            "core-store",
            &[
                "js/state/reactive-store/store.ts",
                "js/state/reactive-store/atom.ts",
                "js/state/reactive-store/types.ts",
                "js/state/reactive-store/metadata.ts",
                "js/state/reactive-store/README.md",
            ],
        ),
        reactive_store_export(
            "atom-graph",
            &[
                "js/state/reactive-store/atom.ts",
                "js/state/reactive-store/types.ts",
                "js/state/reactive-store/metadata.ts",
                "js/state/reactive-store/README.md",
            ],
        ),
        reactive_store_export(
            "comparison-helper",
            &[
                "js/state/reactive-store/shallow.ts",
                "js/state/reactive-store/metadata.ts",
                "js/state/reactive-store/README.md",
            ],
        ),
        reactive_store_export(
            "react-selector",
            &[
                "js/state/reactive-store/react.ts",
                "js/state/reactive-store/store.ts",
                "js/state/reactive-store/atom.ts",
                "js/state/reactive-store/types.ts",
                "js/state/reactive-store/metadata.ts",
                "js/state/reactive-store/README.md",
            ],
        ),
        reactive_store_export(
            "react-context",
            &[
                "js/state/reactive-store/context.tsx",
                "js/state/reactive-store/metadata.ts",
                "js/state/reactive-store/README.md",
            ],
        ),
    ];
    package
}

fn reactive_store_export(name: &str, files: &[&str]) -> DxForgeRegistryExport {
    DxForgeRegistryExport {
        name: name.to_string(),
        files: files.iter().map(|file| (*file).to_string()).collect(),
    }
}

fn curated_registry_provenance(package_id: &str) -> DxForgeProvenanceMetadata {
    if package_id == "shadcn/ui/button" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some(
                "shadcn-ui://apps/v4/registry/bases/radix/ui/button.tsx".to_string(),
            ),
            verified: false,
            note: "DX inspected the local shadcn-ui v4 registry entry and Radix Slot primitive for the official UI Components Button surface; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }

    if package_id == "shadcn/ui/badge" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some(
                "shadcn-ui://apps/v4/registry/bases/radix/ui/badge.tsx".to_string(),
            ),
            verified: false,
            note: "DX inspected the local shadcn-ui v4 registry entry and Radix Slot primitive for the official UI Components Badge surface; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }

    if package_id == "shadcn/ui/card" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some(
                "shadcn-ui://apps/v4/registry/bases/radix/ui/card.tsx".to_string(),
            ),
            verified: false,
            note: "DX inspected the local shadcn-ui v4 registry entry for the official UI Components Card surface; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }

    if package_id == "shadcn/ui/alert" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some(
                "shadcn-ui://apps/v4/registry/bases/radix/ui/alert.tsx".to_string(),
            ),
            verified: false,
            note: "DX inspected the local shadcn-ui v4 registry entry for the official UI Components Alert surface; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }

    if package_id == "shadcn/ui/avatar" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some(
                "shadcn-ui://apps/v4/registry/bases/radix/ui/avatar.tsx".to_string(),
            ),
            verified: false,
            note: "DX inspected the local shadcn-ui v4 registry entry for the official UI Components Avatar surface; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }

    if package_id == "shadcn/ui/skeleton" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some(
                "shadcn-ui://apps/v4/registry/bases/radix/ui/skeleton.tsx".to_string(),
            ),
            verified: false,
            note: "DX inspected the local shadcn-ui v4 registry entry for the official UI Components Skeleton surface; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }

    if package_id == "shadcn/ui/label" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some(
                "shadcn-ui://apps/v4/registry/bases/radix/ui/label.tsx".to_string(),
            ),
            verified: false,
            note: "DX inspected the local shadcn-ui v4 registry entry and Radix Label primitive for the official UI Components Label surface; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }

    if package_id == "shadcn/ui/separator" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some(
                "shadcn-ui://apps/v4/registry/bases/radix/ui/separator.tsx".to_string(),
            ),
            verified: false,
            note: "DX inspected the local shadcn-ui v4 registry entry and Radix Separator primitive for the official UI Components Separator surface; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }

    if package_id == "shadcn/ui/field" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some(
                "shadcn-ui://apps/v4/registry/bases/radix/ui/field.tsx".to_string(),
            ),
            verified: false,
            note: "DX inspected the local shadcn-ui v4 registry entry plus Radix Label and Separator primitives for the official UI Components Field surface; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }

    if package_id == "shadcn/ui/item" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some(
                "shadcn-ui://apps/v4/registry/bases/radix/ui/item.tsx".to_string(),
            ),
            verified: false,
            note: "DX inspected the local shadcn-ui v4 registry entry plus Radix Slot and Separator primitives for the official UI Components Item surface; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }

    if package_id == "shadcn/ui/input" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some(
                "shadcn-ui://apps/v4/registry/bases/radix/ui/input.tsx".to_string(),
            ),
            verified: false,
            note: "DX inspected the local shadcn-ui v4 registry entry for the official UI Components Input surface; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }

    if package_id == "shadcn/ui/textarea" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some(
                "shadcn-ui://apps/v4/registry/bases/radix/ui/textarea.tsx".to_string(),
            ),
            verified: false,
            note: "DX inspected the local shadcn-ui v4 registry entry for the official UI Components Textarea surface; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }

    if package_id == "auth/better-auth" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some("npm:better-auth@1.6.11".to_string()),
            verified: false,
            note: "DX inspected the local upstream better-auth source mirror and package export map for betterAuth(), createAuthClient(), Next.js handlers/cookies, adapters, and plugins; this is curated Authentication launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }
    if package_id == "animation/motion" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some("npm:motion@12.38.0".to_string()),
            verified: false,
            note: "DX inspected the local Motion source mirror, package export map, motion/react re-export, DOM API, React hooks, and examples before curating this launch animation slice; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }
    if package_id == "i18n/next-intl" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some("npm:next-intl@4.12.0".to_string()),
            verified: false,
            note: "DX inspected the local next-intl source mirror, package export map, App Router examples, defineRouting(), createNavigation(), getRequestConfig(), createMiddleware(), NextIntlClientProvider, useTranslations(), and getTranslations() before curating this launch i18n slice; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }
    if package_id == "tanstack/query" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some("npm:@tanstack/react-query@5.100.10".to_string()),
            verified: false,
            note: "DX inspected the local TanStack Query source mirror for QueryClient, QueryClientProvider, useQuery, queryOptions, prefetchQuery, dehydrate, HydrationBoundary, Query, Mutation, QueryState, MutationState, status types, and Updater before curating this launch query slice; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }
    if package_id == "reactive/store" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some("npm:@tanstack/store@0.11.0; npm:@tanstack/react-store@0.11.0".to_string()),
            verified: false,
            note: "DX inspected the local Reactive Store upstream mirror for Store, ReadonlyStore, createStore(), createAtom(), createAsyncAtom(), batch(), shallow(), useSelector(), useAtom(), useStore(), and createStoreContext() before curating this source-owned Reactive Store slice; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }
    if package_id == "validation/zod" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some("npm:zod@4.4.3".to_string()),
            verified: false,
            note: "DX inspected the local Zod source mirror, package export map, v4 classic external exports, schema constructors, safeParse helpers, error formatters, and JSON Schema conversion before curating this launch validation slice; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }
    if package_id == "forms/react-hook-form" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some("npm:react-hook-form@7.75.0".to_string()),
            verified: false,
            note: "DX inspected the local React Hook Form source mirror, package export map, useForm(), FormProvider, useFormContext(), Controller, useFieldArray(), useWatch(), resolver contracts, and examples before curating this launch forms slice; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }
    if package_id == "payments/stripe-js" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some("npm:@stripe/stripe-js@9.6.0".to_string()),
            verified: false,
            note: "DX inspected the local Stripe.js source mirror, package metadata, README, pure loadStripe export, Stripe type declarations, Elements options, and confirmPayment contracts before curating this launch payment slice; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }
    if package_id == "automations/n8n" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some(
                "n8n-io/n8n packages/nodes-base local source mirror".to_string(),
            ),
            verified: false,
            note: "DX inspected the copied n8n-nodes-base node and credential metadata generated under integrations/n8n-nodes-base before curating this source-owned automation bridge slice; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }
    if package_id == "state/zustand" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some("npm:zustand@5.0.13".to_string()),
            verified: false,
            note: "DX inspected the local Zustand source mirror, package export map, vanilla store, React hook, subscribeWithSelector, combine, persist, immer middleware, and shallow helpers before curating this launch state slice; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }
    if package_id == "ai/vercel-ai" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some("npm:ai@7.0.0-canary.146".to_string()),
            verified: false,
            note: "DX inspected the local Vercel AI SDK source mirror and package export map for streamText(), tool(), convertToModelMessages(), DefaultChatTransport, UIMessage, and provider boundaries; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }
    if package_id == "api/trpc" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some(
                "npm:@trpc/server@11.17.0; npm:@trpc/client@11.17.0; npm:@trpc/tanstack-react-query@11.17.0"
                    .to_string(),
            ),
            verified: false,
            note: "DX inspected the local tRPC source mirror, package export maps, fetch adapter, initTRPC root builder, errorFormatter, TRPCErrorFormatter, TRPCErrorShape, inferRouterError, getHTTPStatusCodeFromError, responseMeta, ResponseMeta, ResponseMetaFn, TRPCCombinedDataTransformer, TRPCDataTransformer, createCallerFactory server caller usage, typed router inference, typed client proxy, HTTPBatchLinkOptions, HTTPHeaders, TRPCFetch, headers, maxItems, maxURLLength, methodOverride, link transformer options, httpBatchLink, httpBatchStreamLink, loggerLink, httpSubscriptionLink, splitLink, tracked subscriptions, TanStack React Query query/mutation/infinite-query/subscription options, and Next App Router examples before curating this launch API slice; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }
    if package_id == "content/fumadocs-next" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some(
                "npm:fumadocs-core@16.8.12; npm:fumadocs-ui@16.8.12; npm:fumadocs-mdx@15.0.7; npm:fumadocs-openapi@10.8.6"
                    .to_string(),
            ),
            verified: false,
            note: "DX inspected the local Fumadocs source mirror, package export maps, package metadata, next, next-min, next-static, and openapi examples, MDX config, source loader, source plugin exports, lucideIconsPlugin(), statusBadgesPlugin(), slugsFromData(), breadcrumb/page-tree exports, getBreadcrumbItems(), flattenTree(), findNeighbour(), getPageTreePeers(), getTableOfContents(), TOCItemType, page.data.toc, llms() source export, processed Markdown getText() flow, createOpenAPI(), staticSource(), loaderPlugin(), createProxy(), proxyUrl, createAPIPage(), createCodeUsageGeneratorRegistry(), registerDefault(), defineClientConfig(), createFromSource search route, staticGET search export, useDocsSearch client preset shape, docs layouts, MDX component exports, and starter content flow before curating this launch docs slice; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }
    if package_id == "content/react-markdown" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some(
                "npm:react-markdown@10.1.0; npm:@mdx-js/mdx@3.1.1; npm:@mdx-js/react@3.1.1"
                    .to_string(),
            ),
            verified: false,
            note: "DX inspected the local Markdown & MDX Content source mirrors for react-markdown Markdown/MarkdownAsync/MarkdownHooks/defaultUrlTransform, @mdx-js/mdx compile/compileSync/createProcessor/nodeTypes/providerImportSource, and @mdx-js/react MDXProvider/useMDXComponents before curating this launch content slice; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }
    if package_id == "supabase/client" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some(
                "supabase/supabase examples/auth/nextjs and examples/user-management/nextjs-user-management"
                    .to_string(),
            ),
            verified: false,
            note: "DX inspected the local Supabase monorepo examples and package metadata for @supabase/ssr, @supabase/supabase-js, createBrowserClient(), createServerClient(), auth actions, and the profiles RLS seed; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }
    if package_id == "db/drizzle-sqlite" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some("npm:drizzle-orm@0.45.3".to_string()),
            verified: false,
            note: "DX inspected the local Drizzle ORM source mirror, package metadata, sqlite-core README, root exports, relations, SQL helpers, and better-sqlite3 driver shape; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }
    if package_id == "instantdb/react" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some(
                "instantdb/instant client/packages/react and examples/next-js-app-dir".to_string(),
            ),
            verified: false,
            note: "DX inspected the local InstantDB source mirror, @instantdb/react export map, init(), schema builder, db.useQuery(), db.transact(), db.tx, room presence, and the Next.js app-dir example; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }
    if package_id == "wasm/bindgen" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some("wasm-bindgen@0.2.121 local source mirror".to_string()),
            verified: false,
            note: "DX inspected the local wasm-bindgen source mirror for the #[wasm_bindgen] prelude, JsValue runtime surface, Closure bridge, cli-support Bindgen builder, CLI targets, and examples before curating this launch loader slice; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }
    if package_id == "3d/launch-scene" {
        return DxForgeProvenanceMetadata {
            source: "dx-forge-curated-registry".to_string(),
            upstream_reference: Some(
                "three@0.184.0; @react-three/fiber@9.6.1; @react-three/drei local mirror"
                    .to_string(),
            ),
            verified: false,
            note: "DX inspected the local Three, React Three Fiber, and Drei mirrors for WebGL engine shape, Canvas/root rendering, frame hooks, Stage, Environment, PresentationControls, Text, and MIT license boundaries before curating this source-owned Web Preview scene slice; this is curated launch metadata, not SLSA or live upstream provenance.".to_string(),
        };
    }

    DxForgeProvenanceMetadata {
        source: "dx-forge-curated-registry".to_string(),
        upstream_reference: Some(format!("dx-forge://packages/{package_id}")),
        verified: false,
        note: "Curated DX Forge source metadata is recorded, but this is not SLSA or live upstream provenance yet.".to_string(),
    }
}

fn curated_advisory_review(package_id: &str) -> DxForgeAdvisoryMetadata {
    DxForgeAdvisoryMetadata {
        coverage_kind: DxForgeAdvisoryCoverageKind::CuratedFixture,
        provider: "dx-forge-curated-advisory-fixture".to_string(),
        live_coverage: false,
        finding_count: 0,
        reviewed_at: Some("2026-05-17T00:00:00Z".to_string()),
        note: curated_advisory_note(package_id).to_string(),
    }
}

fn curated_advisory_note(package_id: &str) -> &'static str {
    match package_id {
        "shadcn/ui/badge" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this shadcn badge launch slice, but it is not a live advisory feed and does not audit application status taxonomy, labels, or tone."
        }
        "shadcn/ui/label" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this shadcn label launch slice, but it is not a live advisory feed and does not audit application form copy, accessible names, or validation relationships."
        }
        "shadcn/ui/separator" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this shadcn separator launch slice, but it is not a live advisory feed and does not audit application information hierarchy or decorative-versus-semantic divider policy."
        }
        "shadcn/ui/field" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this shadcn field launch slice, but it is not a live advisory feed and does not audit application form layout, validation copy, or error announcement policy."
        }
        "shadcn/ui/item" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this shadcn item launch slice, but it is not a live advisory feed and does not audit application list semantics, actions, or row-level authorization."
        }
        "auth/better-auth" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this Authentication launch slice based on upstream better-auth, but it is not a live advisory feed and does not audit the deployed database adapter, session policy, or OAuth provider credentials."
        }
        "animation/motion" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this Motion launch slice, but it is not a live advisory feed and does not audit application animation performance or accessibility choices."
        }
        "i18n/next-intl" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this next-intl launch slice, but it is not a live advisory feed and does not audit translated message quality, locale strategy, or deployed routing policy."
        }
        "tanstack/query" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this TanStack Query launch slice, but it is not a live advisory feed and does not audit application query functions, endpoints, or cached payload contents."
        }
        "reactive/store" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this Reactive Store slice, but it is not a live advisory feed and does not audit application state shape, mutation policy, persistence, render granularity, or sensitive state handling."
        }
        "validation/zod" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this Zod validation launch slice, but it is not a live advisory feed and does not audit application schemas, accepted inputs, or downstream authorization policy."
        }
        "forms/react-hook-form" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this React Hook Form launch slice, but it is not a live advisory feed and does not audit submitted data, validation schema quality, accessibility, or application authorization policy."
        }
        "payments/stripe-js" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this Stripe.js launch client slice, but it is not a live advisory feed and does not audit server-side payment creation, webhook verification, fulfillment, fraud policy, or PCI posture."
        }
        "automations/n8n" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this n8n automation bridge slice, but it is not a live advisory feed and does not audit live connector credentials, workflow execution, or external account policy."
        }
        "state/zustand" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this Zustand-compatible launch state slice, but it is not a live advisory feed and does not audit application persistence keys, browser storage policy, optional Immer dependency review, draft mutation conventions, or sensitive state handling."
        }
        "ai/vercel-ai" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this Vercel AI SDK launch slice, but it is not a live advisory feed and does not audit the selected provider package, API key handling, model safety policy, rate limiting, or persistence."
        }
        "api/trpc" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this tRPC launch slice, but it is not a live advisory feed and does not audit application procedures, authorization policy, request limits, request id propagation, auth token source, cross-origin header policy, proxy URL limits, fetch runtime selection, persistence, audit logging, pagination cursor semantics, serializer dependency selection, custom type registration, cache taxonomy, CDN/proxy behavior, deployed header verification, log redaction policy, JSONL/proxy compatibility, error redaction policy, public error copy, subscription fan-out, stream pacing, EventSource policy, or deployed route configuration."
        }
        "content/fumadocs-next" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this Fumadocs launch docs slice, but it is not a live advisory feed and does not audit application content, source plugin taxonomy, navigation policy, slug/canonical URL policy, OpenAPI proxy allowed origins, auth forwarding policy, search UI, multilingual/vector search policy, Next config merges, or deployment policy."
        }
        "content/react-markdown" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this react-markdown launch slice, but it is not a live advisory feed and does not audit user-generated content, plugin policy, raw HTML, or link sanitization."
        }
        "supabase/client" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this Supabase client slice, but it is not a live advisory feed and does not audit deployed RLS policy, Auth redirect configuration, or project secrets."
        }
        "db/drizzle-sqlite" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this Drizzle SQLite launch slice, but it is not a live advisory feed and does not audit deployed database files, migration policy, or application query access control."
        }
        "instantdb/react" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this InstantDB React launch slice, but it is not a live advisory feed and does not audit dashboard rules, app ids, auth policy, or deployed data access."
        }
        "wasm/bindgen" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this wasm-bindgen loader slice, but it is not a live advisory feed and does not audit generated Wasm binaries, Rust crates, or JavaScript glue emitted by local builds."
        }
        "3d/launch-scene" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this 3D launch scene slice, but it is not a live advisory feed and does not audit application WebGL shaders, GPU performance, external 3D assets, or optional Three/R3F/Drei dependencies."
        }
        "migration/static-site" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this static migration example, but it is not a live advisory feed and does not scan imported HTML."
        }
        "dx/icon/search" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this selected icon package, but it is not a live advisory feed."
        }
        "shadcn/ui/button" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this UI Components button surface, but it is not a live advisory feed."
        }
        "shadcn/ui/card" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this UI Components card surface, but it is not a live advisory feed."
        }
        "shadcn/ui/input" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this UI Components input surface, but it is not a live advisory feed."
        }
        "shadcn/ui/textarea" => {
            "Curated DX Forge advisory fixture records no known advisory findings for this UI Components textarea surface, but it is not a live advisory feed."
        }
        _ => {
            "Curated DX Forge advisory fixture records no known advisory findings for this package, but it is not a live advisory feed."
        }
    }
}

fn declared_license_review(license: &str) -> DxForgeLicenseReviewMetadata {
    DxForgeLicenseReviewMetadata {
        declared_license: license.to_string(),
        reviewed: false,
        reviewed_at: None,
        note: "License is recorded from the curated package declaration only; no formal DX legal review is claimed."
            .to_string(),
    }
}
