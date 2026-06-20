pub fn default_source_package(package_id: &str) -> Result<DxSourcePackage> {
    source_package_with_config(package_id, &DxForgeProjectConfig::default(), "default")
}

/// Build the default materialized source package for a named variant.
pub fn default_source_package_variant(package_id: &str, variant: &str) -> Result<DxSourcePackage> {
    source_package_with_config(package_id, &DxForgeProjectConfig::default(), variant)
}

/// Build a materialized source package for a project config.
pub fn source_package_for_project(
    package_id: &str,
    project: impl AsRef<Path>,
) -> Result<DxSourcePackage> {
    source_package_for_project_variant(package_id, project, "default")
}

/// Build a materialized source package for a project config and named variant.
pub fn source_package_for_project_variant(
    package_id: &str,
    project: impl AsRef<Path>,
    variant: &str,
) -> Result<DxSourcePackage> {
    let config = DxForgeProjectConfig::load(project)?;
    source_package_with_config(package_id, &config, variant)
}

/// Build the registry package for a canonical package id.
pub fn registry_package(package_id: &str) -> Result<DxForgeRegistryPackage> {
    let package = match canonical_package_id(package_id) {
        "shadcn/ui/button" => {
            let templates = shadcn_button_templates();
            build_registry_package(
                "shadcn/ui/button",
                vec!["ui/button".to_string()],
                DxForgeLanguage::Js,
                SHADCN_BUTTON_VERSION,
                DxForgeRegistrySource::Curated,
                "MIT OR Apache-2.0",
                "Source-owned UI Components button surface based on shadcn-ui v4 and Radix Slot, with local class helpers.",
                templates,
            )
        }
        "shadcn/ui/badge" => build_registry_package(
            "shadcn/ui/badge",
            vec!["ui/badge".to_string()],
            DxForgeLanguage::Js,
            SHADCN_BADGE_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT OR Apache-2.0",
            "Source-owned UI Components badge surface based on shadcn-ui v4 and Radix Slot, with local class helpers.",
            shadcn_badge_templates(),
        ),
        "shadcn/ui/card" => build_registry_package(
            "shadcn/ui/card",
            vec!["ui/card".to_string()],
            DxForgeLanguage::Js,
            SHADCN_CARD_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT OR Apache-2.0",
            "Source-owned UI Components card surface based on shadcn-ui v4, with local class helpers.",
            shadcn_card_templates(),
        ),
        "shadcn/ui/alert" => build_registry_package(
            "shadcn/ui/alert",
            vec!["ui/alert".to_string()],
            DxForgeLanguage::Js,
            SHADCN_ALERT_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT OR Apache-2.0",
            "Source-owned UI Components alert surface based on the shadcn-ui v4 registry shape, with local class helpers.",
            shadcn_alert_templates(),
        ),
        "shadcn/ui/avatar" => build_registry_package(
            "shadcn/ui/avatar",
            vec!["ui/avatar".to_string()],
            DxForgeLanguage::Js,
            SHADCN_AVATAR_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT OR Apache-2.0",
            "Source-owned UI Components avatar surface based on the shadcn-ui v4 registry shape, with local class helpers.",
            shadcn_avatar_templates(),
        ),
        "shadcn/ui/skeleton" => build_registry_package(
            "shadcn/ui/skeleton",
            vec!["ui/skeleton".to_string()],
            DxForgeLanguage::Js,
            SHADCN_SKELETON_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT OR Apache-2.0",
            "Source-owned UI Components skeleton surface based on the shadcn-ui v4 registry shape, with local class helpers.",
            shadcn_skeleton_templates(),
        ),
        "shadcn/ui/label" => build_registry_package(
            "shadcn/ui/label",
            vec!["ui/label".to_string()],
            DxForgeLanguage::Js,
            SHADCN_LABEL_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT OR Apache-2.0",
            "Source-owned UI Components label surface based on shadcn-ui v4 and the Radix Label primitive, with local class helpers.",
            shadcn_label_templates(),
        ),
        "shadcn/ui/separator" => build_registry_package(
            "shadcn/ui/separator",
            vec!["ui/separator".to_string()],
            DxForgeLanguage::Js,
            SHADCN_SEPARATOR_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT OR Apache-2.0",
            "Source-owned UI Components separator surface based on shadcn-ui v4 and the Radix Separator primitive, with local class helpers.",
            shadcn_separator_templates(),
        ),
        "shadcn/ui/field" => build_registry_package(
            "shadcn/ui/field",
            vec!["ui/field".to_string()],
            DxForgeLanguage::Js,
            SHADCN_FIELD_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT OR Apache-2.0",
            "Source-owned UI Components field primitives based on shadcn-ui v4, Radix Label, Radix Separator, and local class helpers.",
            shadcn_field_templates(),
        ),
        "shadcn/ui/item" => build_registry_package(
            "shadcn/ui/item",
            vec!["ui/item".to_string()],
            DxForgeLanguage::Js,
            SHADCN_ITEM_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT OR Apache-2.0",
            "Source-owned UI Components item primitives based on shadcn-ui v4, Radix Slot, Radix Separator, and local class helpers.",
            shadcn_item_templates(),
        ),
        "shadcn/ui/input" => build_registry_package(
            "shadcn/ui/input",
            vec!["ui/input".to_string()],
            DxForgeLanguage::Js,
            SHADCN_INPUT_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT OR Apache-2.0",
            "Source-owned UI Components input surface based on the shadcn-ui v4 registry shape.",
            shadcn_input_templates(),
        ),
        "shadcn/ui/textarea" => build_registry_package(
            "shadcn/ui/textarea",
            vec!["ui/textarea".to_string()],
            DxForgeLanguage::Js,
            SHADCN_TEXTAREA_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT OR Apache-2.0",
            "Source-owned UI Components textarea surface based on the shadcn-ui v4 registry shape.",
            shadcn_textarea_templates(),
        ),
        "dx/icon/search" => build_registry_package(
            "dx/icon/search",
            vec!["icon/search".to_string(), "icons/search".to_string()],
            DxForgeLanguage::Js,
            DX_ICON_SEARCH_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT OR Apache-2.0",
            "Source-owned selected Search icon with a tiny local icon helper.",
            icon_search_templates(),
        ),
        "auth/better-auth" => build_registry_package(
            "auth/better-auth",
            vec![
                "authentication".to_string(),
                "better-auth".to_string(),
                "auth/betterauth".to_string(),
                "auth/better-auth-next".to_string(),
                "google/auth".to_string(),
                "google-oauth".to_string(),
            ],
            DxForgeLanguage::Js,
            DX_AUTH_BETTER_AUTH_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT",
            "Source-owned Authentication launch slice based on upstream better-auth APIs for Next route handlers, server options, React client creation, social sign-in helpers, linked-account helpers, profile and email-change helpers, guarded account deletion helpers, email/password helpers, account security helpers, server and client session-management helpers, env contracts, and discovery metadata.",
            auth_better_auth_templates(),
        ),
        "animation/motion" => build_registry_package(
            "animation/motion",
            vec![
                "framer-motion".to_string(),
                "framer/motion".to_string(),
                "motion".to_string(),
                "motion-animation".to_string(),
                "motion-and-animation".to_string(),
                "motion/react".to_string(),
            ],
            DxForgeLanguage::Js,
            MOTION_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT",
            "Source-owned Motion React launch slice with MotionConfig defaults, useAnimationControls/useAnimation imperative controls, useAnimationFrame/useTime frame timing, useWillChange/WillChangeMotionValue performance hints, usePageInView document visibility, LazyMotion feature bundles, shared LayoutGroup/layoutId transitions, instant layout updates, MotionValue meters, AnimatePresence presence transitions, Reorder/useDragControls sorting, reduced-motion-safe reveal primitives, scoped useAnimate feedback, scroll progress, variants, transitions, and discovery metadata.",
            motion_templates(),
        ),
        "i18n/next-intl" => build_registry_package(
            "i18n/next-intl",
            vec![
                "next-intl".to_string(),
                "next-intl/routing".to_string(),
                "i18n/next".to_string(),
            ],
            DxForgeLanguage::Js,
            NEXT_INTL_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT",
            "Source-owned next-intl App Router launch slice for routing, navigation, request config, middleware, provider, starter messages, and discovery metadata.",
            next_intl_templates(),
        ),
        "tanstack/query" => build_registry_package(
            "tanstack/query",
            vec![
                "data-fetching-cache".to_string(),
                "data-fetching/cache".to_string(),
                "tanstack-query".to_string(),
                "tanstack/react-query".to_string(),
                "@tanstack/react-query".to_string(),
                "react-query".to_string(),
                "query/tanstack".to_string(),
            ],
            DxForgeLanguage::Js,
            TANSTACK_QUERY_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT",
            "Source-owned Data Fetching & Cache launch adapter based on upstream @tanstack/react-query public APIs for QueryClient defaults, React provider wiring, server prefetch hydration, core query/mutation state summaries, cache operations, and discovery metadata.",
            tanstack_query_templates(),
        ),
        "reactive/store" => reactive_store_registry_package(),
        "validation/zod" => build_registry_package(
            "validation/zod",
            vec![
                "zod".to_string(),
                "zod/v4".to_string(),
                "schema/zod".to_string(),
                "validation/zod/v4".to_string(),
            ],
            DxForgeLanguage::Js,
            ZOD_VALIDATION_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT",
            "Source-owned Zod validation launch adapter for typed schemas, object composition, safe parsing, display error policies, query-string coercion, JSON Schema export, experimental JSON Schema import, launch catalog validation, cross-field approval gates, form usage, and discovery metadata.",
            zod_validation_templates(),
        ),
        "forms/react-hook-form" => build_registry_package(
            "forms/react-hook-form",
            vec![
                "forms".to_string(),
                "react-hook-form".to_string(),
                "rhf".to_string(),
                "forms/rhf".to_string(),
            ],
            DxForgeLanguage::Js,
            REACT_HOOK_FORM_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT",
            "Source-owned Forms launch slice based on React Hook Form for provider wiring, registered fields, controlled fields, field arrays, watchers, resolver bridging, and discovery metadata.",
            react_hook_form_templates(),
        ),
        "payments/stripe-js" => build_registry_package(
            "payments/stripe-js",
            vec![
                "payments".to_string(),
                "stripe-js".to_string(),
                "stripe".to_string(),
                "@stripe/stripe-js".to_string(),
                "payments/stripe".to_string(),
            ],
            DxForgeLanguage::Js,
            STRIPE_JS_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT",
            "Source-owned Stripe.js launch payment slice for publishable-key config, hosted Stripe.js loading, Payment Element confirmation, checkout contact validation, server-only Checkout Sessions, env examples, and discovery metadata.",
            stripe_js_templates(),
        ),
        "automations/n8n" => build_registry_package(
            "automations/n8n",
            vec![
                "n8n".to_string(),
                "n8n-nodes-base".to_string(),
                "workflows/n8n".to_string(),
            ],
            DxForgeLanguage::Js,
            N8N_AUTOMATIONS_VERSION,
            DxForgeRegistrySource::Curated,
            "Sustainable Use License",
            "Source-owned n8n automation bridge slice for generated connector metadata, redacted credential metadata, CLI command discovery, workflow run receipts, and no-node_modules DX/Zed handoff.",
            n8n_automations_templates(),
        ),
        "state/zustand" => build_registry_package(
            "state/zustand",
            vec![
                "zustand".to_string(),
                "npm/zustand".to_string(),
                "pmndrs/zustand".to_string(),
                "state/zustand-react".to_string(),
            ],
            DxForgeLanguage::Js,
            ZUSTAND_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT",
            "Source-owned Zustand-compatible launch state slice with vanilla store, React hook, selector subscriptions, middleware mutator typing, Immer draft updates, shallow equality, and JSON persistence.",
            zustand_templates(),
        ),
        "ai/vercel-ai" => build_registry_package(
            "ai/vercel-ai",
            vec![
                "ai-sdk".to_string(),
                "npm/ai".to_string(),
                "vercel-ai".to_string(),
                "vercel/ai".to_string(),
            ],
            DxForgeLanguage::Js,
            VERCEL_AI_VERSION,
            DxForgeRegistrySource::Curated,
            "Apache-2.0",
            "Source-owned Vercel AI SDK launch slice for streaming chat routes, typed tools, UI transport, explicit model configuration, and discovery metadata.",
            vercel_ai_templates(),
        ),
        "api/trpc" => build_registry_package(
            "api/trpc",
            vec![
                "trpc".to_string(),
                "trpc/next".to_string(),
                "@trpc/server".to_string(),
                "@trpc/client".to_string(),
                "@trpc/react-query".to_string(),
                "@trpc/tanstack-react-query".to_string(),
            ],
            DxForgeLanguage::Js,
            TRPC_NEXT_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT",
            "Source-owned Type-Safe API slice based on upstream tRPC 11 public APIs for initTRPC, shared transformer wiring, typed error formatting, response metadata, fetch route handling, typed server callers, typed clients, request headers, batch/URL policy, router inference, TanStack Query provider/mutation/infinite-query/subscription wiring, HTTP subscription and streaming transports, diagnostics logging, and discovery metadata.",
            trpc_next_templates(),
        ),
        "content/fumadocs-next" => build_registry_package(
            "content/fumadocs-next",
            vec![
                "fumadocs".to_string(),
                "fumadocs-next".to_string(),
                "docs/fumadocs".to_string(),
                "docs/fumadocs-next".to_string(),
                "mdx/fumadocs".to_string(),
            ],
            DxForgeLanguage::Js,
            FUMADOCS_NEXT_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT",
            "Source-owned Fumadocs Next App Router launch docs slice with MDX config, source plugins, navigation and TOC helpers, docs routes, LLMs routes, OpenAPI virtual docs/proxy, dynamic/static Orama search wiring, starter content, and discovery metadata.",
            fumadocs_next_templates(),
        ),
        "content/react-markdown" => build_registry_package(
            "content/react-markdown",
            vec![
                "react-markdown".to_string(),
                "markdown-mdx-content".to_string(),
                "markdown/mdx".to_string(),
                "mdx/content".to_string(),
                "markdown/react".to_string(),
                "remark/react".to_string(),
            ],
            DxForgeLanguage::Js,
            REACT_MARKDOWN_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT",
            "Source-owned Markdown & MDX Content launch slice with safe Markdown rendering, component overrides, MDX provider context, server compile helpers, receipt visibility, and discovery metadata.",
            react_markdown_templates(),
        ),
        "supabase/client" => build_registry_package(
            "supabase/client",
            vec![
                "database/supabase".to_string(),
                "db/supabase".to_string(),
                "supabase/ssr".to_string(),
                "supabase-js".to_string(),
            ],
            DxForgeLanguage::Js,
            DX_SUPABASE_CLIENT_VERSION,
            DxForgeRegistrySource::Curated,
            "Apache-2.0",
            "Source-owned Supabase SSR client slice for browser/server clients, password auth actions, public env, discovery metadata, and a profiles RLS seed.",
            supabase_client_templates(),
        ),
        "db/drizzle-sqlite" => build_registry_package(
            "db/drizzle-sqlite",
            vec![
                "database/drizzle".to_string(),
                "db/drizzle".to_string(),
                "drizzle".to_string(),
                "drizzle-orm/sqlite".to_string(),
                "drizzle/sqlite".to_string(),
            ],
            DxForgeLanguage::Js,
            DRIZZLE_SQLITE_VERSION,
            DxForgeRegistrySource::Curated,
            "Apache-2.0",
            "Source-owned Drizzle ORM SQLite schema, client, and query slice using Drizzle public APIs.",
            drizzle_sqlite_templates(),
        ),
        "instantdb/react" => build_registry_package(
            "instantdb/react",
            vec![
                "@instantdb/react".to_string(),
                "instantdb".to_string(),
                "db/instantdb".to_string(),
            ],
            DxForgeLanguage::Js,
            INSTANTDB_REACT_VERSION,
            DxForgeRegistrySource::Curated,
            "Apache-2.0",
            "Source-owned Realtime App Database launch slice for typed schema, realtime query hooks, transactions, presence, Sync Table events, env config, and discovery metadata.",
            instantdb_react_templates(),
        ),
        "wasm/bindgen" => build_registry_package(
            "wasm/bindgen",
            vec![
                "webassembly-bridge".to_string(),
                "webassembly/bridge".to_string(),
                "rust/wasm-bindgen".to_string(),
                "wasm-bindgen".to_string(),
                "wasm_bindgen".to_string(),
            ],
            DxForgeLanguage::Js,
            WASM_BINDGEN_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT OR Apache-2.0",
            "Source-owned WebAssembly Bridge launch loader for generated wasm-bindgen init(input) modules, React usage, and DX/Zed discovery metadata.",
            wasm_bindgen_templates(),
        ),
        "3d/launch-scene" => build_registry_package(
            THREE_SCENE_PACKAGE_ID,
            vec![
                "3d-scene-system".to_string(),
                THREE_SCENE_OFFICIAL_PACKAGE_NAME.to_string(),
                "3d/scene".to_string(),
                "three-scene".to_string(),
                "three/r3f/drei".to_string(),
                "threejs/scene".to_string(),
                "@react-three/fiber".to_string(),
            ],
            DxForgeLanguage::Js,
            THREE_SCENE_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT",
            "Source-owned Three/R3F/Drei-inspired launch scene slice with a Web Preview-safe WebGL runtime, stable scene contract, premium preset, metadata, and React wrapper.",
            three_scene_templates(),
        ),
        "migration/static-site" => build_registry_package(
            "migration/static-site",
            vec![
                "wordpress/static-site".to_string(),
                "wp/static-site".to_string(),
                "migrations/static-site".to_string(),
                "static/site-migration".to_string(),
            ],
            DxForgeLanguage::Js,
            DX_MIGRATION_STATIC_SITE_VERSION,
            DxForgeRegistrySource::Curated,
            "MIT OR Apache-2.0",
            "Source-owned WordPress/static-site migration example for static pages, content, assets, and honest manual follow-up.",
            migration_static_site_templates(),
        ),
        other => bail!(
            "unsupported Forge package `{other}`; supported packages are `ui/button`, `ui/badge`, `ui/card`, `ui/alert`, `ui/avatar`, `ui/skeleton`, `ui/label`, `ui/separator`, `ui/field`, `ui/item`, `ui/input`, `ui/textarea`, `dx/icon/search`, `auth/better-auth`, `animation/motion`, `i18n/next-intl`, `data-fetching-cache`, `tanstack/query`, `reactive/store`, `validation/zod`, `forms/react-hook-form`, `payments/stripe-js`, `automations/n8n`, `state/zustand`, `ai/vercel-ai`, `api/trpc`, `content/fumadocs-next`, `content/react-markdown`, `supabase/client`, `db/drizzle-sqlite`, `instantdb/react`, `wasm/bindgen`, `3d/launch-scene`, and `migration/static-site`"
        ),
    };
    verify_registry_package_integrity(&package)?;
    Ok(package)
}

/// Verify a Forge registry package manifest and any included file content.
pub fn verify_registry_package_integrity(
    package: &DxForgeRegistryPackage,
) -> Result<DxForgeRegistryIntegrityReport> {
    let canonical = canonical_package_id(&package.package_id);
    if package.package_id != canonical {
        bail!(
            "registry package id `{}` is not canonical; expected `{canonical}`",
            package.package_id
        );
    }
    if package.version.trim().is_empty() {
        bail!(
            "registry package `{}` has an empty version",
            package.package_id
        );
    }
    if !is_blake3_hex(&package.integrity_hash) {
        bail!(
            "registry package `{}` has an invalid package integrity hash",
            package.package_id
        );
    }

    let mut paths = BTreeSet::new();
    let mut verified_files = 0u64;
    for file in &package.files {
        validate_project_relative_path(&file.path)
            .with_context(|| format!("validate registry file `{}`", file.path))?;
        if let Some(logical_path) = &file.logical_path {
            validate_project_relative_path(logical_path)
                .with_context(|| format!("validate registry logical file `{logical_path}`"))?;
        }
        if !paths.insert(file.path.clone()) {
            bail!(
                "registry package `{}` declares duplicate file `{}`",
                package.package_id,
                file.path
            );
        }
        if !is_blake3_hex(&file.hash) {
            bail!(
                "registry package `{}` file `{}` has an invalid BLAKE3 hash",
                package.package_id,
                file.path
            );
        }
        if let Some(content) = &file.content {
            let actual_hash = hash_bytes(content.as_bytes());
            if actual_hash != file.hash {
                bail!(
                    "registry package `{}` file `{}` content hash mismatch",
                    package.package_id,
                    file.path
                );
            }
            let actual_bytes = content.len() as u64;
            if actual_bytes != file.bytes {
                bail!(
                    "registry package `{}` file `{}` byte count mismatch",
                    package.package_id,
                    file.path
                );
            }
            verified_files += 1;
        }
    }

    let file_paths = paths.clone();
    let mut export_names = BTreeSet::new();
    for export in &package.exports {
        if export.name.trim().is_empty() {
            bail!(
                "registry package `{}` declares an empty export name",
                package.package_id
            );
        }
        if !export_names.insert(export.name.clone()) {
            bail!(
                "registry package `{}` declares duplicate export `{}`",
                package.package_id,
                export.name
            );
        }
        for file in &export.files {
            validate_project_relative_path(file)
                .with_context(|| format!("validate registry export file `{file}`"))?;
            if !file_paths.contains(file) {
                bail!(
                    "registry package `{}` export `{}` references unknown file `{}`",
                    package.package_id,
                    export.name,
                    file
                );
            }
        }
    }
    for default_export in &package.default_exports {
        if !export_names.contains(default_export) {
            bail!(
                "registry package `{}` default export `{}` is not declared",
                package.package_id,
                default_export
            );
        }
    }

    let actual_integrity = package_integrity_hash(&package.files);
    if actual_integrity != package.integrity_hash {
        bail!(
            "registry package `{}` integrity mismatch: manifest has `{}`, computed `{actual_integrity}`",
            package.package_id,
            package.integrity_hash
        );
    }

    Ok(DxForgeRegistryIntegrityReport {
        package_id: package.package_id.clone(),
        version: package.version.clone(),
        file_count: package.files.len() as u64,
        verified_files,
        integrity_hash: package.integrity_hash.clone(),
    })
}

/// Load and verify a local Forge registry package manifest and content blobs.
pub fn load_local_registry_package(
    root: impl AsRef<Path>,
    package_id: &str,
    version: &str,
) -> Result<DxForgeRegistryPackage> {
    let root = root.as_ref();
    let canonical = canonical_package_id(package_id);
    let manifest_path = root.join(format!("packages/js/{canonical}/{version}/.dx/build-cache/manifest.json"));
    let bytes = fs::read(&manifest_path)
        .with_context(|| format!("read local registry manifest `{}`", manifest_path.display()))?;
    let package: DxForgeRegistryPackage =
        serde_json::from_slice(&bytes).context("parse local registry package manifest")?;
    ensure_requested_package(&package, canonical, version)?;
    let package = hydrate_local_registry_package(root, package)?;
    verify_registry_package_integrity(&package)?;
    Ok(package)
}

/// Resolve the latest known version for a package in a local registry index.
pub fn latest_local_registry_package_version(
    root: impl AsRef<Path>,
    package_id: &str,
) -> Result<String> {
    let root = root.as_ref();
    let canonical = canonical_package_id(package_id);
    let index_path = root.join(REGISTRY_INDEX);
    let bytes =
        fs::read(&index_path).with_context(|| format!("read `{}`", index_path.display()))?;
    let index: DxForgeRegistryIndex = serde_json::from_slice(&bytes)
        .with_context(|| format!("parse `{}`", index_path.display()))?;
    index
        .packages
        .iter()
        .rev()
        .find(|package| package.package_id == canonical)
        .map(|package| package.version.clone())
        .with_context(|| {
            format!(
                "local Forge registry `{}` does not contain package `{canonical}`",
                root.display()
            )
        })
}

/// Build a registry package from the project's root `dx` Forge package manifest.
pub fn root_dx_registry_package(project: impl AsRef<Path>) -> Result<DxForgeRegistryPackage> {
    let project = project.as_ref();
    let manifest = load_root_dx_package_manifest(project)?.with_context(|| {
        format!(
            "root dx in `{}` does not declare a Forge package",
            project.display()
        )
    })?;
    root_dx_registry_package_from_manifest(project, &manifest)
}

/// Publish the root `dx` package manifest into a local filesystem registry.
pub fn publish_root_dx_package_to_local_registry(
    project: impl AsRef<Path>,
    root: impl AsRef<Path>,
    dry_run: bool,
) -> Result<DxForgeRegistryOperationReport> {
    let root = root.as_ref();
    let package = root_dx_registry_package(project)?;
    verify_registry_package_integrity(&package)?;
    let planned = local_registry_object_paths(root, &package, true);

    if dry_run {
        return Ok(DxForgeRegistryOperationReport {
            action: "registry-publish-plan".to_string(),
            package_id: Some(package.package_id),
            version: Some(package.version),
            remote: root.display().to_string(),
            dry_run: true,
            r2_status: None,
            objects: planned,
        });
    }

    fs::create_dir_all(root).with_context(|| format!("create `{}`", root.display()))?;
    let index = merged_local_registry_index(root, &package)?;
    let mut objects = write_registry_package_to_local(root, &package, Some(&index))?;
    let receipt_path = write_local_registry_publish_receipt(root, &package)?;
    objects.push(receipt_path.display().to_string());

    Ok(DxForgeRegistryOperationReport {
        action: "registry-publish".to_string(),
        package_id: Some(package.package_id),
        version: Some(package.version),
        remote: root.display().to_string(),
        dry_run: false,
        r2_status: None,
        objects,
    })
}

/// Build a dry-run R2 upload plan for the root `dx` package manifest.
pub fn publish_root_dx_package_to_r2_dry_run(
    project: impl AsRef<Path>,
) -> Result<DxForgeRegistryOperationReport> {
    let package = root_dx_registry_package(project)?;
    verify_registry_package_integrity(&package)?;
    let status = DxForgeR2Config::status_from_env();
    let objects = registry_object_keys(&status.prefix, &package, true)
        .into_iter()
        .map(|key| object_url_from_status(&status, &key))
        .collect();

    Ok(DxForgeRegistryOperationReport {
        action: "registry-publish".to_string(),
        package_id: Some(package.package_id),
        version: Some(package.version),
        remote: "r2".to_string(),
        dry_run: true,
        r2_status: Some(status),
        objects,
    })
}

/// Load a local registry package and return only the requested export surfaces.
pub fn source_package_from_local_registry_selected_exports(
    root: impl AsRef<Path>,
    package_id: &str,
    version: &str,
    selected_exports: &[String],
    project: impl AsRef<Path>,
) -> Result<DxSourcePackage> {
    let package = load_local_registry_package(root, package_id, version)?;
    source_package_from_registry_selected_exports(&package, selected_exports, project.as_ref())
}

/// Create a local registry layout.
pub fn init_local_registry(path: impl AsRef<Path>) -> Result<DxForgeRegistryOperationReport> {
    let root = path.as_ref();
    fs::create_dir_all(root).with_context(|| format!("create `{}`", root.display()))?;

    let packages = vec![
        registry_package("shadcn/ui/button")?,
        registry_package("shadcn/ui/badge")?,
        registry_package("shadcn/ui/card")?,
        registry_package("shadcn/ui/alert")?,
        registry_package("shadcn/ui/avatar")?,
        registry_package("shadcn/ui/skeleton")?,
        registry_package("shadcn/ui/label")?,
        registry_package("shadcn/ui/separator")?,
        registry_package("shadcn/ui/field")?,
        registry_package("shadcn/ui/item")?,
        registry_package("shadcn/ui/input")?,
        registry_package("shadcn/ui/textarea")?,
        registry_package("dx/icon/search")?,
        registry_package("auth/better-auth")?,
        registry_package("animation/motion")?,
        registry_package("i18n/next-intl")?,
        registry_package("tanstack/query")?,
        registry_package("reactive/store")?,
        registry_package("validation/zod")?,
        registry_package("forms/react-hook-form")?,
        registry_package("payments/stripe-js")?,
        registry_package("automations/n8n")?,
        registry_package("state/zustand")?,
        registry_package("ai/vercel-ai")?,
        registry_package("content/fumadocs-next")?,
        registry_package("content/react-markdown")?,
        registry_package("supabase/client")?,
        registry_package("db/drizzle-sqlite")?,
        registry_package("instantdb/react")?,
        registry_package("wasm/bindgen")?,
        registry_package("3d/launch-scene")?,
        registry_package("migration/static-site")?,
    ];
    let registry_index = DxForgeRegistryIndex {
        version: 1,
        generated_at: Utc::now().to_rfc3339(),
        packages: packages
            .iter()
            .map(DxForgeRegistryPackage::clone_without_content)
            .collect(),
        remotes: Vec::new(),
    };
    let mut objects = Vec::new();
    for (package_index, package) in packages.iter().enumerate() {
        objects.extend(write_registry_package_to_local(
            root,
            package,
            (package_index == 0).then_some(&registry_index),
        )?);
    }

    Ok(DxForgeRegistryOperationReport {
        action: "registry-init".to_string(),
        package_id: None,
        version: None,
        remote: root.display().to_string(),
        dry_run: false,
        r2_status: None,
        objects,
    })
}

/// Return redacted R2 registry status.
pub fn r2_registry_status() -> DxForgeRegistryOperationReport {
    let status = DxForgeR2Config::status_from_env();
    DxForgeRegistryOperationReport {
        action: "registry-status".to_string(),
        package_id: None,
        version: None,
        remote: "r2".to_string(),
        dry_run: true,
        r2_status: Some(status),
        objects: Vec::new(),
    }
}

/// Plan R2/S3 remote reads for install/update/remove without touching the network.
pub fn plan_r2_remote_read_only_install(
    intent: DxForgeRemoteReadIntent,
    package_id: &str,
    selected_exports: &[String],
    requested_version: Option<&str>,
) -> DxForgeRemoteReadPlan {
    let status = DxForgeR2Config::status_from_env();
    let package_object_path = package_id.trim_matches('/');
    let version_segment = requested_version.unwrap_or("<version>");
    let manifest_key = format!(
        "{}/packages/js/{package_object_path}/{version_segment}/.dx/build-cache/manifest.json",
        status.prefix
    );
    let content_key = format!(
        "{}/packages/js/{package_object_path}/{version_segment}/files/<content-hash>",
        status.prefix
    );
    let latest_key = format!(
        "{}/packages/js/{package_object_path}/latest.json",
        status.prefix
    );
    let mut objects = vec![
        DxForgeRemoteReadObject {
            intent: "latest-version".to_string(),
            object_key: latest_key,
            required: requested_version.is_none(),
        },
        DxForgeRemoteReadObject {
            intent: "package-manifest".to_string(),
            object_key: manifest_key,
            required: true,
        },
        DxForgeRemoteReadObject {
            intent: "content-blob".to_string(),
            object_key: content_key,
            required: true,
        },
    ];
    if matches!(intent, DxForgeRemoteReadIntent::UninstallDryRun) {
        objects.push(DxForgeRemoteReadObject {
            intent: "tracked-source-manifest".to_string(),
            object_key: ".dx/forge/source-.dx/build-cache/manifest.json".to_string(),
            required: true,
        });
    }

    let mut warnings = vec![
        "remote manifest fetch/read-only materialization planning; no network call is performed"
            .to_string(),
        "R2/S3 read support is a provider boundary in this launch pass; use local registry for real materialize/update/remove writes.".to_string(),
    ];
    if !status.configured {
        warnings.push(format!(
            "R2 is {}; missing redacted config labels: {}",
            status.setup_status,
            if status.missing_config.is_empty() {
                "none".to_string()
            } else {
                status.missing_config.join(", ")
            }
        ));
    }

    DxForgeRemoteReadPlan {
        schema_version: "dx.forge.remote_read_plan".to_string(),
        provider_kind: DxForgeRemoteProviderKind::S3CompatibleObjectStorage,
        intent,
        package_id: package_id.to_string(),
        requested_version: requested_version.map(str::to_string),
        selected_exports: selected_exports.to_vec(),
        network_allowed: false,
        write_allowed: false,
        boundary:
            "remote manifest fetch/read-only materialization planning; no network call is performed"
                .to_string(),
        setup_status: status.setup_status,
        missing_config: status.missing_config,
        objects,
        object_metadata_plan: None,
        object_head_execution_receipt: None,
        object_head_health_evaluation: None,
        manifest_install_preview: None,
        warnings,
    }
}

/// Plan an R2/S3 install from a real package manifest fixture without touching the network.
pub fn plan_r2_remote_read_only_install_from_manifest_fixture(
    intent: DxForgeRemoteReadIntent,
    package_id: &str,
    selected_exports: &[String],
    requested_version: Option<&str>,
    manifest_path: impl AsRef<Path>,
    project: impl AsRef<Path>,
) -> Result<DxForgeRemoteReadPlan> {
    let manifest_path = manifest_path.as_ref();
    let project = project.as_ref();
    let bytes = fs::read(manifest_path)
        .with_context(|| format!("read remote manifest fixture `{}`", manifest_path.display()))?;
    let package: DxForgeRegistryPackage = serde_json::from_slice(&bytes).with_context(|| {
        format!(
            "parse remote manifest fixture `{}`",
            manifest_path.display()
        )
    })?;
    let canonical = canonical_package_id(package_id);
    if let Some(version) = requested_version {
        ensure_requested_package(&package, canonical, version)?;
    } else if package.package_id != canonical {
        bail!(
            "registry manifest package id mismatch: expected `{canonical}`, found `{}`",
            package.package_id
        );
    }
    verify_registry_package_integrity(&package)?;

    let mut plan = plan_r2_remote_read_only_install(
        intent,
        canonical,
        selected_exports,
        Some(&package.version),
    );
    let preview =
        remote_manifest_install_preview(&package, selected_exports, manifest_path, project)?;
    let provider_status = DxForgeR2Config::status_from_env();
    let provider = DxForgeR2ReadOnlyProvider::from_status(&provider_status);
    let metadata_plan = provider.object_metadata_plan(&package);
    let head_receipt = plan_r2_remote_object_head_execution_receipt(&metadata_plan);
    let head_health_evaluation = evaluate_r2_remote_object_head_receipt_health(&head_receipt);
    plan.object_metadata_plan = Some(metadata_plan);
    plan.object_head_execution_receipt = Some(head_receipt);
    plan.object_head_health_evaluation = Some(head_health_evaluation);
    plan.manifest_install_preview = Some(preview);
    plan.warnings.push(
        "remote manifest fixture was parsed from local disk; no R2/S3 object was fetched"
            .to_string(),
    );
    Ok(plan)
}

/// Publish a package to R2.
pub async fn publish_registry_package_to_r2(
    package_id: &str,
    dry_run: bool,
) -> Result<DxForgeRegistryOperationReport> {
    let package = registry_package(package_id)?;

    if dry_run {
        let status = DxForgeR2Config::status_from_env();
        let objects = registry_object_keys(&status.prefix, &package, true)
            .into_iter()
            .map(|key| object_url_from_status(&status, &key))
            .collect();

        return Ok(DxForgeRegistryOperationReport {
            action: "registry-publish".to_string(),
            package_id: Some(package.package_id),
            version: Some(package.version),
            remote: "r2".to_string(),
            dry_run: true,
            r2_status: Some(status),
            objects,
        });
    }

    let config = DxForgeR2Config::from_env()
        .context("Cloudflare R2 is not configured for DX Forge registry publishing")?;
    let index = DxForgeRegistryIndex {
        version: 1,
        generated_at: Utc::now().to_rfc3339(),
        packages: vec![package.clone_without_content()],
        remotes: vec![DxForgeRegistryRemote {
            name: "r2".to_string(),
            kind: "cloudflare-r2".to_string(),
            prefix: config.prefix.clone(),
            configured: true,
        }],
    };
    let objects = registry_object_keys(&config.prefix, &package, true);

    if !dry_run {
        let store = config.store()?;
        put_json(
            &store,
            &format!("{}/{}", config.prefix, REGISTRY_INDEX),
            &index,
        )
        .await?;
        put_json(
            &store,
            &package_manifest_key(&config.prefix, &package),
            &package.clone_without_content(),
        )
        .await?;
        for file in &package.files {
            let content = file
                .content
                .as_deref()
                .context("registry package file missing content")?;
            put_bytes(
                &store,
                &package_file_key(&config.prefix, &package, &file.hash),
                content.as_bytes(),
            )
            .await?;
        }
        let receipt = registry_publish_receipt(&package);
        let receipt_key = format!(
            "{}/receipts/{}-{}.json",
            config.prefix,
            Utc::now().format("%Y%m%dT%H%M%SZ"),
            package.package_id.replace('/', "-")
        );
        put_json(&store, &receipt_key, &receipt).await?;
    }

    Ok(DxForgeRegistryOperationReport {
        action: "registry-publish".to_string(),
        package_id: Some(package.package_id),
        version: Some(package.version),
        remote: "r2".to_string(),
        dry_run: false,
        r2_status: Some(DxForgeR2Config::status_from_env()),
        objects: objects
            .into_iter()
            .map(|key| config.object_url(&key))
            .collect(),
    })
}

/// Pull package metadata from R2.
pub async fn pull_registry_package_from_r2(
    package_id: &str,
    version: &str,
    dry_run: bool,
) -> Result<DxForgeRegistryOperationReport> {
    let canonical = canonical_package_id(package_id);
    if dry_run {
        let status = DxForgeR2Config::status_from_env();
        let manifest_key = format!(
            "{}/packages/js/{canonical}/{version}/.dx/build-cache/manifest.json",
            status.prefix
        );
        let content_key = format!(
            "{}/packages/js/{canonical}/{version}/files/<content-hash>",
            status.prefix
        );

        return Ok(DxForgeRegistryOperationReport {
            action: "registry-pull".to_string(),
            package_id: Some(canonical.to_string()),
            version: Some(version.to_string()),
            remote: "r2".to_string(),
            dry_run: true,
            r2_status: Some(status.clone()),
            objects: vec![
                object_url_from_status(&status, &manifest_key),
                object_url_from_status(&status, &content_key),
            ],
        });
    }

    let config = DxForgeR2Config::from_env()
        .context("Cloudflare R2 is not configured for DX Forge registry pulls")?;
    let manifest_key = format!(
        "{}/packages/js/{}/{}/.dx/build-cache/manifest.json",
        config.prefix, canonical, version
    );
    let store = config.store()?;
    let bytes = store
        .get(&ObjectPath::from(manifest_key.as_str()))
        .await
        .with_context(|| format!("pull `{manifest_key}`"))?
        .bytes()
        .await
        .context("read pulled registry package")?;
    let package: DxForgeRegistryPackage =
        serde_json::from_slice(&bytes).context("parse pulled registry package")?;
    ensure_requested_package(&package, canonical, version)?;
    let package = hydrate_r2_registry_package(&store, &config.prefix, package).await?;
    verify_registry_package_integrity(&package)?;
    let objects = std::iter::once(manifest_key)
        .chain(
            package
                .files
                .iter()
                .map(|file| package_file_key(&config.prefix, &package, &file.hash)),
        )
        .map(|key| config.object_url(&key))
        .collect::<Vec<_>>();
    let package_id = package.package_id.clone();
    let version = package.version.clone();

    Ok(DxForgeRegistryOperationReport {
        action: "registry-pull".to_string(),
        package_id: Some(package_id),
        version: Some(version),
        remote: "r2".to_string(),
        dry_run: false,
        r2_status: Some(DxForgeR2Config::status_from_env()),
        objects,
    })
}
