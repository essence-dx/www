//! DX-WWW wrappers for the vendored Next.js and Turbopack Rust snapshot.
//!
//! The vendored files live outside the Cargo workspace as reference/provenance.
//! Public DX-WWW APIs should depend on this module, not on upstream `next_*`
//! names or runtime/build adoption claims.

pub use crate::next_rust_source_map_adapter::{
    DX_NEXT_RUST_SOURCE_MAP_ADAPTER_FORMAT, DX_NEXT_RUST_SOURCE_MAP_ADAPTER_SCHEMA,
    DxNextRustSourceMapAdapter, DxNextRustSourceMapDiagnosticLocation, DxNextRustSourceMapLookup,
    DxNextRustSourceMapSegment, dx_next_rust_source_map_adapter,
    dx_next_rust_source_map_diagnostic_location, dx_next_rust_source_map_lookup,
};
pub use crate::next_rust_task_adapter::{
    DX_NEXT_RUST_TASK_GRAPH_ADAPTER_FORMAT, DX_NEXT_RUST_TASK_GRAPH_ADAPTER_SCHEMA,
    DX_NEXT_RUST_TASK_INPUT_ADAPTER_FORMAT, DX_NEXT_RUST_TASK_INPUT_ADAPTER_SCHEMA,
    DxNextRustTaskGraphAdapter, DxNextRustTaskGraphNode, DxNextRustTaskInputAdapter,
    dx_next_rust_turbo_tasks_graph_adapter, dx_next_rust_turbo_tasks_task_input_adapter,
};

/// Upstream commit imported into `vendor/next-rust`.
pub const NEXT_RUST_VENDOR_COMMIT: &str = "f3f56ecec2f3f8cefa0f0a1323ea406740251d5c";

/// Upstream branch captured at import time.
pub const NEXT_RUST_VENDOR_BRANCH: &str = "canary";

/// Date the quarantined snapshot was imported.
pub const NEXT_RUST_VENDOR_IMPORTED_ON: &str = "2026-05-23";

/// Quarantined vendor root inside the DX-WWW repository.
pub const NEXT_RUST_VENDOR_ROOT: &str = "vendor/next-rust";

/// License notice copied with the upstream snapshot.
pub const NEXT_RUST_VENDOR_LICENSE_FILE: &str = "vendor/next-rust/license.nextjs.md";

/// Local upstream checkout used when the snapshot was copied.
pub const NEXT_RUST_VENDOR_SOURCE_PATH_AT_IMPORT: &str = "G:\\WWW\\inspirations\\nextjs";

/// Real Next/Turbopack runtime or build adoption is outside the DX-WWW scope.
pub const DX_NEXT_RUST_RUNTIME_BUILD_ADOPTION: bool = false;

/// Turbopack is not the public DX-WWW architecture.
pub const DX_NEXT_RUST_PUBLIC_ARCHITECTURE: bool = false;

/// A vendored subsystem and its intended DX-WWW role.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DxNextRustCapability {
    /// Vendor-root-relative upstream Rust group path.
    pub upstream: &'static str,
    /// DX-WWW-facing subsystem name.
    pub dx_role: &'static str,
    /// Boundary that explains how DX-WWW may use the vendored code.
    pub boundary: &'static str,
    /// Whether the subsystem is retained only for source study and provenance.
    pub reference_only: bool,
}

/// DX-owned runtime or source surface that the vendored code must not replace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DxProtectedBoundary {
    /// Human-readable protected DX surface.
    pub name: &'static str,
    /// Current DX owner or subsystem.
    pub owner: &'static str,
    /// Rule that keeps Next/Turbopack imports behind the boundary.
    pub rule: &'static str,
}

/// Turbopack-core graph concept mapped onto a DX-owned receipt surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DxTurbopackCoreGraphConcept {
    /// Upstream concept name used for local source study.
    pub upstream_concept: &'static str,
    /// Vendor-root-relative files that informed this mapping.
    pub vendor_paths: &'static [&'static str],
    /// DX receipt contracts that own the public shape.
    pub dx_contracts: &'static [&'static str],
    /// DX graph node kinds covered by the concept.
    pub dx_node_kinds: &'static [&'static str],
    /// DX graph edge kinds covered by the concept.
    pub dx_edge_kinds: &'static [&'static str],
    /// Receipt fields where the concept is exposed.
    pub dx_receipt_fields: &'static [&'static str],
    /// Boundary statement that keeps Turbopack out of the public runtime.
    pub boundary: &'static str,
    /// Whether this concept requires project-local `node_modules`.
    pub node_modules_required: bool,
}

/// Snapshot metadata for the quarantined Next.js/Turbopack Rust import.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxNextRustVendorSnapshot {
    /// Upstream source repository name.
    pub upstream_repo: &'static str,
    /// Upstream commit hash.
    pub commit: &'static str,
    /// Upstream branch captured at import time.
    pub branch: &'static str,
    /// Import date for this local snapshot.
    pub imported_on: &'static str,
    /// Local upstream checkout used when copying this snapshot.
    pub source_path_at_import: &'static str,
    /// Local vendor root.
    pub vendor_root: &'static str,
    /// Vendor-relative license file.
    pub license_file: &'static str,
    /// Imported capabilities.
    pub capabilities: &'static [DxNextRustCapability],
    /// Whether DX-WWW adopts Next/Turbopack runtime or build execution.
    pub runtime_build_adoption: bool,
    /// Whether Turbopack is the public DX-WWW architecture.
    pub public_architecture: bool,
    /// Runtime crate/package identifiers that remain DX-WWW-owned.
    pub protected_runtime_crates: &'static [&'static str],
    /// Runtime and source boundaries protected from vendor takeover.
    pub protected_boundaries: &'static [DxProtectedBoundary],
    /// Imported or adjacent upstream foundations that stay out of the DX core.
    pub excluded_core_foundations: &'static [&'static str],
}

/// Imported Rust capabilities that DX-WWW may wrap behind DX-owned APIs.
pub const DX_NEXT_RUST_CAPABILITIES: &[DxNextRustCapability] = &[
    DxNextRustCapability {
        upstream: "crates/next-code-frame",
        dx_role: "DX-WWW diagnostics code-frame reference",
        boundary: "reference-only: DX diagnostics own code-frame output",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "crates/next-custom-transforms",
        dx_role: "DX-WWW transform compatibility reference",
        boundary: "reference-only: detection and receipts, not runtime takeover",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbo-persistence",
        dx_role: "DX-WWW persistent build cache reference",
        boundary: "reference-only: DX receipts own cache evidence",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbo-tasks",
        dx_role: "DX-WWW incremental build task graph reference",
        boundary: "reference-only: source-owned build graph only",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbo-tasks-auto-hash-map",
        dx_role: "Support crate reference for task graph source study",
        boundary: "reference-only: no compiled support dependency in DX core",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbo-tasks-backend",
        dx_role: "Support backend reference for task graph source study",
        boundary: "reference-only: no task backend takeover",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbo-tasks-bytes",
        dx_role: "Support crate for task graph byte payloads",
        boundary: "reference-only: byte payload ideas only",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbo-tasks-env",
        dx_role: "Environment input reference for DX build graph receipts",
        boundary: "reference-only: source receipts decide environment inputs",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbo-tasks-fetch",
        dx_role: "Fetch task reference for DX build graph receipts",
        boundary: "reference-only: no default network runtime foundation",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbo-tasks-fs",
        dx_role: "Filesystem invalidation reference for DX source policy",
        boundary: "reference-only: Forge/source path policy remains authoritative",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbo-tasks-fuzz",
        dx_role: "Upstream fuzz fixtures for task graph study",
        boundary: "reference-only: not a DX runtime dependency",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbo-tasks-hash",
        dx_role: "Support crate reference for task graph hashing",
        boundary: "reference-only: source receipts remain the public hash surface",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbo-tasks-macros",
        dx_role: "Support macro reference for task graph source study",
        boundary: "reference-only: no public DX macro contract",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbo-tasks-macros-tests",
        dx_role: "Upstream macro tests for task graph study",
        boundary: "reference-only: not a DX runtime dependency",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbo-tasks-malloc",
        dx_role: "Optional allocator reference for task graph study",
        boundary: "reference-only: no allocator takeover",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbo-tasks-testing",
        dx_role: "Upstream task graph testing utilities",
        boundary: "reference-only: not a DX runtime dependency",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbopack-core",
        dx_role: "DX-WWW module and asset graph reference",
        boundary: "reference-only: dx.build.graph remains public truth",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbopack-css",
        dx_role: "DX-WWW CSS pipeline compatibility reference",
        boundary: "reference-only: dx-style remains authoritative",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbopack-dev-server",
        dx_role: "DX-WWW dev server and HMR behavior reference",
        boundary: "reference-only: Axum/hyper/DX server stays core",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbopack-ecmascript",
        dx_role: "DX-WWW TSX/ECMAScript analysis compatibility reference",
        boundary: "reference-only: DX output model remains source-owned",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbopack-ecmascript-hmr-protocol",
        dx_role: "DX-WWW hot reload protocol reference",
        boundary: "reference-only: DX dev server protocol remains branded",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbopack-image",
        dx_role: "DX-WWW image asset metadata reference",
        boundary: "reference-only: metadata receipts before pipeline claims",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbopack-mdx",
        dx_role: "DX-WWW MDX/content compatibility reference",
        boundary: "reference-only: Forge/source docs model remains authoritative",
        reference_only: true,
    },
    DxNextRustCapability {
        upstream: "turbopack/crates/turbopack-resolve",
        dx_role: "DX-WWW compatibility resolver reference",
        boundary: "reference-only: Forge/source resolver rules stay authoritative",
        reference_only: true,
    },
];

/// Runtime crates that must remain DX-WWW-owned while build internals evolve.
pub const DX_PROTECTED_RUNTIME_CRATES: &[&str] = &[
    "dx-www-browser-micro",
    "dx-www-browser",
    "dx-www-packet",
    "dx-www-binary",
    "dx-www-morph",
    "dx-serializer",
    "dx-style",
    "related-crates/style",
    "dx-www-server",
];

/// DX-owned surfaces that the vendor snapshot must not replace.
pub const DX_PROTECTED_BOUNDARIES: &[DxProtectedBoundary] = &[
    DxProtectedBoundary {
        name: "browser-micro",
        owner: "dx-www-browser-micro",
        rule: "DX browser micro runtime remains the smallest trusted browser runtime.",
    },
    DxProtectedBoundary {
        name: "browser",
        owner: "dx-www-browser",
        rule: "DX browser runtime remains source-owned and does not require React/RSC.",
    },
    DxProtectedBoundary {
        name: "packet",
        owner: "dx-www-packet",
        rule: "DX packet protocol remains the application transport contract.",
    },
    DxProtectedBoundary {
        name: "binary",
        owner: "dx-www-binary",
        rule: "DX binary format remains the production output contract.",
    },
    DxProtectedBoundary {
        name: "morph",
        owner: "dx-www-morph",
        rule: "DX morphing remains runtime-owned and not React-owned.",
    },
    DxProtectedBoundary {
        name: "serializer",
        owner: "dx-serializer",
        rule: "DX serialization receipts remain source-owned.",
    },
    DxProtectedBoundary {
        name: "dx-style / related-crates/style",
        owner: "dx-style",
        rule: "dx-style remains the CSS identity; Turbopack CSS is a compatibility reference.",
    },
    DxProtectedBoundary {
        name: "Rust server",
        owner: "dx-www-server",
        rule: "The Axum/hyper/DX Rust server remains the server foundation.",
    },
    DxProtectedBoundary {
        name: "Forge/source receipts",
        owner: "DX source model",
        rule: "Forge and source receipts stay the public evidence surface.",
    },
    DxProtectedBoundary {
        name: "dx-check",
        owner: "DX verification",
        rule: "dx-check remains the source-of-truth health and receipt model.",
    },
    DxProtectedBoundary {
        name: "Zed surfaces",
        owner: "DX editor integration",
        rule: "Zed-facing receipts remain DX-branded and source-owned.",
    },
    DxProtectedBoundary {
        name: "Studio surfaces",
        owner: "DX Studio integration",
        rule: "Studio-facing receipts remain DX-branded and source-owned.",
    },
];

/// Upstream foundations that are not promoted to the DX-WWW core.
pub const DX_EXCLUDED_CORE_FOUNDATIONS: &[&str] = &[
    "next-core",
    "next-napi-bindings",
    "turbopack-nodejs",
    "React/RSC",
    "Node/NAPI",
    "Turborepo",
    "node_modules",
    "Node resolver defaults",
];

/// Turbopack-core concepts that are mapped into DX-owned graph receipts.
pub const DX_TURBOPACK_CORE_GRAPH_CONCEPTS: &[DxTurbopackCoreGraphConcept] = &[
    DxTurbopackCoreGraphConcept {
        upstream_concept: "ModuleGraph",
        vendor_paths: &["turbopack/crates/turbopack-core/src/module_graph/mod.rs"],
        dx_contracts: &["dx.build.graph", "dx.www.moduleGraph"],
        dx_node_kinds: &[
            "route-shell-chunk",
            "source-module",
            "source-module-chunk",
            "tsx-component",
            "tsx-route",
        ],
        dx_edge_kinds: &[
            "compiled-from-source",
            "imports",
            "imports-source-module",
            "links-entry-module",
        ],
        dx_receipt_fields: &["graph.nodes", "graph.edges", "invalidation"],
        boundary: "adapter-boundary: graph concepts inform DX receipts; Turbopack is not the public architecture",
        node_modules_required: false,
    },
    DxTurbopackCoreGraphConcept {
        upstream_concept: "Module",
        vendor_paths: &["turbopack/crates/turbopack-core/src/module.rs"],
        dx_contracts: &["dx.www.moduleGraph"],
        dx_node_kinds: &[
            "source-module",
            "source-module-chunk",
            "tsx-component",
            "tsx-route",
        ],
        dx_edge_kinds: &["compiled-from-source", "imports-source-module"],
        dx_receipt_fields: &["graph.nodes", "route_outputs[].source_module_chunks"],
        boundary: "source-owned modules and emitted chunks stay DX runtime metadata, not React/RSC modules",
        node_modules_required: false,
    },
    DxTurbopackCoreGraphConcept {
        upstream_concept: "ModuleReference",
        vendor_paths: &["turbopack/crates/turbopack-core/src/reference/mod.rs"],
        dx_contracts: &["dx.build.graph", "dx.www.moduleGraph"],
        dx_node_kinds: &[],
        dx_edge_kinds: &["imports", "imports-source-module", "links-entry-module"],
        dx_receipt_fields: &["graph.edges", "source_module_chunks[].dependencies"],
        boundary: "DX resolver and Forge source rules decide references; Node resolution is compatibility-only",
        node_modules_required: false,
    },
    DxTurbopackCoreGraphConcept {
        upstream_concept: "Asset",
        vendor_paths: &["turbopack/crates/turbopack-core/src/asset.rs"],
        dx_contracts: &["dx.build.graph"],
        dx_node_kinds: &["dx-style-css", "public-asset"],
        dx_edge_kinds: &["imports"],
        dx_receipt_fields: &["styles", "assets"],
        boundary: "dx-style and hashed public assets stay source-owned receipt nodes",
        node_modules_required: false,
    },
    DxTurbopackCoreGraphConcept {
        upstream_concept: "OutputAsset",
        vendor_paths: &["turbopack/crates/turbopack-core/src/output.rs"],
        dx_contracts: &["dx.build.graph", "dx.www.moduleGraph"],
        dx_node_kinds: &["deploy-output", "route-shell-chunk"],
        dx_edge_kinds: &["emits", "emitted-from", "links-entry-module"],
        dx_receipt_fields: &["route_outputs", "graph.nodes"],
        boundary: "route shells and deploy outputs remain DX-owned outputs, not Next runtime assets",
        node_modules_required: false,
    },
    DxTurbopackCoreGraphConcept {
        upstream_concept: "ChunkingContext",
        vendor_paths: &["turbopack/crates/turbopack-core/src/chunk/mod.rs"],
        dx_contracts: &["dx.www.moduleGraph"],
        dx_node_kinds: &["route-shell-chunk", "source-module-chunk"],
        dx_edge_kinds: &["links-entry-module", "imports-source-module"],
        dx_receipt_fields: &[
            "route_outputs[].shell_chunk_output",
            "route_outputs[].source_module_chunks",
        ],
        boundary: "DX emits browser-executable metadata chunks without adopting Turbopack chunk runtime",
        node_modules_required: false,
    },
    DxTurbopackCoreGraphConcept {
        upstream_concept: "SourceMap",
        vendor_paths: &[
            "turbopack/crates/turbopack-core/src/source_map/mod.rs",
            "turbopack/crates/turbopack-core/src/source_map/source_map_asset.rs",
        ],
        dx_contracts: &["dx.diagnostics.code_frame", "dx.www.sourceMapAdapter"],
        dx_node_kinds: &["route-shell-chunk", "source-module-chunk"],
        dx_edge_kinds: &["maps-generated-to-source"],
        dx_receipt_fields: &[
            "source_maps[].segments",
            "diagnostics[].generated_position",
            "diagnostics[].source_position",
        ],
        boundary: "DX source-map lookups stay source-owned and feed diagnostics without executing Turbopack",
        node_modules_required: false,
    },
    DxTurbopackCoreGraphConcept {
        upstream_concept: "SourceOwnedInvalidation",
        vendor_paths: &["turbopack/crates/turbopack-core/src/module_graph/mod.rs"],
        dx_contracts: &["dx.build.graph"],
        dx_node_kinds: &["dx-check-receipt"],
        dx_edge_kinds: &["checks"],
        dx_receipt_fields: &[
            "invalidation.changedNodeIds",
            "invalidation.affectedNodeIds",
            "invalidation.rebuildNodeIds",
        ],
        boundary: "DX invalidation walks receipt edges and does not expose turbo-tasks as architecture",
        node_modules_required: false,
    },
    DxTurbopackCoreGraphConcept {
        upstream_concept: "ForgeSourceSurface",
        vendor_paths: &["turbopack/crates/turbopack-core/src/module_graph/mod.rs"],
        dx_contracts: &["dx.forge.sourceGraph"],
        dx_node_kinds: &["forge-surface"],
        dx_edge_kinds: &["expects-receipt", "owns-source"],
        dx_receipt_fields: &[".dx/forge/source-.dx/build-cache/manifest.json"],
        boundary: "Forge package ownership is a DX source model; Turbopack has no authority over it",
        node_modules_required: false,
    },
];

/// Return the current vendored snapshot metadata.
#[must_use]
pub fn dx_next_rust_vendor_snapshot() -> DxNextRustVendorSnapshot {
    DxNextRustVendorSnapshot {
        upstream_repo: "vercel/next.js",
        commit: NEXT_RUST_VENDOR_COMMIT,
        branch: NEXT_RUST_VENDOR_BRANCH,
        imported_on: NEXT_RUST_VENDOR_IMPORTED_ON,
        source_path_at_import: NEXT_RUST_VENDOR_SOURCE_PATH_AT_IMPORT,
        vendor_root: NEXT_RUST_VENDOR_ROOT,
        license_file: NEXT_RUST_VENDOR_LICENSE_FILE,
        capabilities: DX_NEXT_RUST_CAPABILITIES,
        runtime_build_adoption: DX_NEXT_RUST_RUNTIME_BUILD_ADOPTION,
        public_architecture: DX_NEXT_RUST_PUBLIC_ARCHITECTURE,
        protected_runtime_crates: DX_PROTECTED_RUNTIME_CRATES,
        protected_boundaries: DX_PROTECTED_BOUNDARIES,
        excluded_core_foundations: DX_EXCLUDED_CORE_FOUNDATIONS,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_keeps_dx_runtime_authoritative() {
        let snapshot = dx_next_rust_vendor_snapshot();
        assert_eq!(snapshot.commit, NEXT_RUST_VENDOR_COMMIT);
        assert!(
            snapshot
                .protected_runtime_crates
                .contains(&"dx-www-browser-micro")
        );
        assert!(
            snapshot
                .capabilities
                .iter()
                .any(
                    |capability| capability.upstream == "turbopack/crates/turbo-tasks"
                        && capability.reference_only
                )
        );
        assert_eq!(snapshot.runtime_build_adoption, false);
        assert_eq!(snapshot.public_architecture, false);
        assert!(
            snapshot
                .capabilities
                .iter()
                .all(|capability| capability.reference_only)
        );
        assert!(
            snapshot
                .protected_boundaries
                .iter()
                .any(|boundary| boundary.name == "Forge/source receipts")
        );
        assert!(snapshot.excluded_core_foundations.contains(&"React/RSC"));
    }

    #[test]
    fn snapshot_records_exact_quarantined_vendor_paths() {
        let snapshot = dx_next_rust_vendor_snapshot();
        let actual_paths: Vec<_> = snapshot
            .capabilities
            .iter()
            .map(|capability| capability.upstream)
            .collect();
        let expected_paths = vec![
            "crates/next-code-frame",
            "crates/next-custom-transforms",
            "turbopack/crates/turbo-persistence",
            "turbopack/crates/turbo-tasks",
            "turbopack/crates/turbo-tasks-auto-hash-map",
            "turbopack/crates/turbo-tasks-backend",
            "turbopack/crates/turbo-tasks-bytes",
            "turbopack/crates/turbo-tasks-env",
            "turbopack/crates/turbo-tasks-fetch",
            "turbopack/crates/turbo-tasks-fs",
            "turbopack/crates/turbo-tasks-fuzz",
            "turbopack/crates/turbo-tasks-hash",
            "turbopack/crates/turbo-tasks-macros",
            "turbopack/crates/turbo-tasks-macros-tests",
            "turbopack/crates/turbo-tasks-malloc",
            "turbopack/crates/turbo-tasks-testing",
            "turbopack/crates/turbopack-core",
            "turbopack/crates/turbopack-css",
            "turbopack/crates/turbopack-dev-server",
            "turbopack/crates/turbopack-ecmascript",
            "turbopack/crates/turbopack-ecmascript-hmr-protocol",
            "turbopack/crates/turbopack-image",
            "turbopack/crates/turbopack-mdx",
            "turbopack/crates/turbopack-resolve",
        ];
        assert_eq!(actual_paths, expected_paths);

        let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("dx-www crate should live under the workspace root");
        let vendor_root = repo_root.join(snapshot.vendor_root);
        assert!(vendor_root.join("license.nextjs.md").is_file());

        for path in actual_paths {
            assert!(
                vendor_root.join(path).join("Cargo.toml").is_file(),
                "vendored Rust group should exist and expose Cargo metadata: {path}"
            );
        }
    }

    #[test]
    fn vendor_readme_matches_snapshot_metadata() {
        let snapshot = dx_next_rust_vendor_snapshot();
        let readme = include_str!("../../../vendor/next-rust/README.md");
        assert!(readme.contains(&format!("- Commit: `{}`", snapshot.commit)));
        assert!(readme.contains("- License file: `license.nextjs.md`"));

        for capability in snapshot.capabilities {
            assert!(
                readme.contains(&format!("- `{}`", capability.upstream)),
                "vendor README should list imported Rust group `{}`",
                capability.upstream
            );
        }

        for boundary in snapshot.protected_boundaries {
            assert!(
                readme.contains(boundary.name),
                "vendor README should list protected boundary `{}`",
                boundary.name
            );
        }

        for excluded in snapshot.excluded_core_foundations {
            assert!(
                readme.contains(excluded),
                "vendor README should list excluded core foundation `{excluded}`"
            );
        }
    }

    #[test]
    fn turbopack_core_graph_map_stays_dx_owned() {
        let concepts = DX_TURBOPACK_CORE_GRAPH_CONCEPTS;
        assert!(
            concepts
                .iter()
                .any(|concept| concept.upstream_concept == "ModuleGraph"
                    && concept.dx_contracts.contains(&"dx.www.moduleGraph"))
        );
        assert!(
            concepts
                .iter()
                .any(|concept| concept.upstream_concept == "Module"
                    && concept.dx_node_kinds.contains(&"source-module")
                    && concept.dx_node_kinds.contains(&"source-module-chunk"))
        );
        assert!(
            concepts
                .iter()
                .any(|concept| concept.upstream_concept == "Module"
                    && concept.dx_edge_kinds.contains(&"compiled-from-source"))
        );
        assert!(
            concepts
                .iter()
                .any(|concept| concept.upstream_concept == "ModuleReference"
                    && concept.dx_edge_kinds.contains(&"imports-source-module"))
        );
        assert!(
            concepts
                .iter()
                .any(|concept| concept.upstream_concept == "Asset"
                    && concept.dx_node_kinds.contains(&"dx-style-css")
                    && concept.dx_node_kinds.contains(&"public-asset"))
        );
        assert!(
            concepts
                .iter()
                .any(|concept| concept.upstream_concept == "OutputAsset"
                    && concept.dx_node_kinds.contains(&"route-shell-chunk"))
        );
        assert!(
            concepts
                .iter()
                .any(|concept| concept.upstream_concept == "SourceMap"
                    && concept.dx_contracts.contains(&"dx.www.sourceMapAdapter")
                    && concept.dx_edge_kinds.contains(&"maps-generated-to-source"))
        );
        assert!(
            concepts
                .iter()
                .any(|concept| concept.upstream_concept == "ForgeSourceSurface"
                    && concept.dx_contracts.contains(&"dx.forge.sourceGraph")
                    && concept.dx_node_kinds.contains(&"forge-surface"))
        );
        assert!(concepts.iter().all(|concept| {
            concept.node_modules_required == false
                && !concept.boundary.is_empty()
                && !concept.boundary.contains("public architecture switch")
        }));
    }
}
