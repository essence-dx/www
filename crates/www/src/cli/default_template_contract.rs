pub(crate) fn default_www_template_architecture_contract() -> serde_json::Value {
    serde_json::json!({
        "schema": "dx.www.default_template.architecture_contract",
        "lane": 13,
        "template_id": "next-familiar-www-template",
        "runtime_model": {
            "foundation": "dx-www",
            "public_runtime_layers": [
                "foundation",
                "html",
                "javascript",
                "wasm",
                "browser",
                "protocol",
                "server"
            ],
            "protected_runtime_crates": [
                "dx-www-browser-micro",
                "dx-www-browser",
                "dx-www-packet",
                "dx-www-binary",
                "dx-www-morph",
                "dx-serializer",
                "dx-style",
                "dx-www-server"
            ],
            "react_required": false,
            "rsc_required": false,
            "node_required": false,
            "napi_required": false,
            "node_resolver_default": false
        },
        "build_layer": {
            "dx_source_build": true,
            "external_bundler_runtime_executed": false,
            "external_bundler_runtime_required": false,
            "next_rust_reference_scope": "reference-provenance-only",
            "reference_next_rust_groups": [
                "turbo-tasks",
                "turbo-persistence",
                "turbopack-core",
                "turbopack-ecmascript",
                "turbopack-css",
                "turbopack-image",
                "turbopack-mdx",
                "turbopack-resolve",
                "next-code-frame",
                "next-custom-transforms"
            ],
            "public_architecture": false
        },
        "developer_experience": {
            "next_familiar_authoring": true,
            "next_familiar_app_files": true,
            "app_router_file_shape": true,
            "react_compatible_authoring": "optional-adapter-boundary",
            "rsc_core_model": false,
            "node_modules_required": false
        },
        "evidence_surfaces": {
            "forge_receipts": true,
            "dx_style": true,
            "dx_check": true,
            "zed_template_handoff": true,
            "studio_edit_contract": true
        },
        "source_boundaries": [
            "Do not replace DX runtime with Next runtime.",
            "Do not make React or RSC the core app model.",
            "Do not make Node, npm, NAPI, Turborepo, or node_modules the foundation.",
            "Keep Next/Turbopack materials as reference/provenance only.",
            "Keep external bundlers out of DX build/dev execution.",
            "Keep Forge/source-owned resolver rules authoritative."
        ]
    })
}
