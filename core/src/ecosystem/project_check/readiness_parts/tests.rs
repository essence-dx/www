#[cfg(test)]
mod tests {
    use super::super::super::dx_style_receipts::DX_STYLE_POSTCSS_COMPAT_SCHEMA;
    use super::*;
    use crate::ecosystem::{
        DxForgeLocalSourceFile, DxForgeLocalSourcePackage, write_forge_add,
        write_forge_local_source, write_forge_update,
    };
    use sha2::{Digest, Sha256};
    use tempfile::tempdir;

    fn write_dx_contract_config(root: &Path) {
        fs::write(
            root.join(DX_CONFIG_PATH),
            "project.name=\"contract\"\nproject.contract.authoring=\"react-shaped\"\nproject.contract.folders=\"next-familiar\"\nproject.contract.package_policy=\"forge-first-no-node-modules\"\ntooling.biome.version=\"2.4.15\"\ntooling.biome.formatter.enabled=true\ntooling.biome.linter.enabled=true\n",
        )
        .expect("dx config");
    }

    fn write_dx_serializer_cache(root: &Path) {
        fs::create_dir_all(root.join(".dx/serializer")).expect("serializer dir");
        fs::write(root.join(DX_SERIALIZER_MACHINE_PATH), b"dx-machine").expect("machine cache");
    }

    #[test]
    fn maintainability_skip_list_covers_windows_reserved_names() {
        assert!(is_windows_reserved_path_name("NUL"));
        assert!(is_windows_reserved_path_name("con.txt"));
        assert!(is_windows_reserved_path_name("LPT1.log"));
        assert!(!is_windows_reserved_path_name("null-state.ts"));
        assert!(!is_windows_reserved_path_name("component.tsx"));
    }

    #[test]
    fn dx_check_score_combines_sections() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("shadcn/ui/button", dir.path()).expect("forge add");

        let report = check_dx_project(dir.path()).expect("check");

        assert!(report.score >= 90);
        assert_eq!(report.sections.len(), 5);
        assert!(
            report
                .sections
                .iter()
                .any(|section| section.name == "forge")
        );
    }

    #[test]
    fn dx_check_allows_local_forge_source_without_curated_latest_lookup() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("components/ui")).expect("components");
        let source = "export function Button() { return <button />; }\n";
        fs::write(dir.path().join("components/ui/button.tsx"), source).expect("button");
        write_forge_local_source(
            DxForgeLocalSourcePackage {
                package_id: "www/starter-ui".to_string(),
                variant: "default".to_string(),
                upstream_name: "www/source-owned-starter-ui".to_string(),
                version: "0.1.0".to_string(),
                license: "MIT".to_string(),
                files: vec![DxForgeLocalSourceFile {
                    path: "components/ui/button.tsx".to_string(),
                    content: source.to_string(),
                }],
            },
            dir.path(),
        )
        .expect("track local source");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(forge.score, 100);
        assert!(
            !forge
                .findings
                .iter()
                .any(|finding| finding.code == "forge-package-latest-unavailable")
        );
    }

    #[test]
    fn dx_check_reports_forge_package_lock_remote_and_media_health() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("shadcn/ui/button", dir.path()).expect("forge add");

        let source_path = dir.path().join("components/ui/button.tsx");
        let source_hash = blake3::hash(&fs::read(&source_path).expect("source bytes"))
            .to_hex()
            .to_string();
        fs::create_dir_all(dir.path().join("public")).expect("public dir");
        let media_bytes = b"forge package preview media";
        fs::write(dir.path().join("public/preview.bin"), media_bytes).expect("media");
        let media_hash = blake3::hash(media_bytes).to_hex().to_string();
        fs::create_dir_all(dir.path().join(".dx/forge/receipts/packages")).expect("receipt dir");
        fs::create_dir_all(
            dir.path()
                .join(".dx/forge/cache/shadcn-ui-button/0.1.0/components/ui"),
        )
        .expect("cache dir");
        fs::copy(
            &source_path,
            dir.path()
                .join(".dx/forge/cache/shadcn-ui-button/0.1.0/components/ui/button.tsx"),
        )
        .expect("cache package source");
        let package_receipt = serde_json::json!({
            "schema": "forge.package_add_receipt",
            "package": {
                "name": "shadcn/ui/button",
                "version": "0.1.0"
            },
            "cache": {
                "cache_path": ".dx/forge/cache/shadcn-ui-button/0.1.0",
                "cached_files": [
                    {
                        "path": "components/ui/button.tsx",
                        "cache_path": ".dx/forge/cache/shadcn-ui-button/0.1.0/components/ui/button.tsx",
                        "content_hash": source_hash
                    }
                ]
            }
        });
        fs::write(
            dir.path()
                .join(".dx/forge/receipts/packages/shadcn-ui-button.json"),
            serde_json::to_vec_pretty(&package_receipt).expect("receipt json"),
        )
        .expect("write package receipt");

        let lock = serde_json::json!({
            "schema": "forge.package_lock",
            "packages": [
                {
                    "name": "shadcn/ui/button",
                    "version": "0.1.0",
                    "source_kind": "local-slice",
                    "source_locator": "components/ui/button.tsx",
                    "files": [
                        {
                            "path": "components/ui/button.tsx",
                            "content_hash": source_hash
                        }
                    ],
                    "receipt_paths": [
                        ".dx/forge/receipts/packages/shadcn-ui-button.json"
                    ]
                }
            ],
            "remotes": [
                {
                    "name": "local-cache",
                    "kind": "local-filesystem",
                    "locator": "file://.dx/forge/cache",
                    "auth_ref": "none",
                    "secret_policy": "no-plaintext-secrets",
                    "secrets_safe": true
                },
                {
                    "name": "release-r2",
                    "kind": "s3-compatible",
                    "locator": "s3://dx-forge/releases",
                    "auth_ref": "env:CLOUDFLARE_R2_ACCESS_KEY_ID",
                    "secret_policy": "env-only",
                    "secrets_safe": true
                }
            ],
            "media": [
                {
                    "asset_id": "preview",
                    "path": "public/preview.bin",
                    "content_hash": media_hash,
                    "chunk_count": 1,
                    "chunk_map": [
                        {
                            "index": 0,
                            "offset": 0,
                            "length": media_bytes.len(),
                            "hash": media_hash
                        }
                    ]
                }
            ]
        });
        fs::write(
            dir.path().join(PACKAGE_LOCK_PATH),
            serde_json::to_vec_pretty(&lock).expect("lock json"),
        )
        .expect("write lock");
        fs::create_dir_all(dir.path().join(".dx/forge/receipts/vcs")).expect("vcs receipt dir");
        let vcs_receipt_path = ".dx/forge/receipts/vcs/snapshot.json";
        let vcs_status = serde_json::json!({
            "schema": "dx.www.template.forge_vcs_status",
            "status": "clean",
            "snapshot_id": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "snapshot_receipt_path": vcs_receipt_path,
            "tracked_files": [
                {
                    "path": "components/ui/button.tsx",
                    "exists": true,
                    "kind": "code"
                },
                {
                    "path": "public/preview.bin",
                    "exists": true,
                    "kind": "media"
                }
            ]
        });
        fs::write(
            dir.path().join(VCS_STATUS_PATH),
            serde_json::to_vec_pretty(&vcs_status).expect("vcs status json"),
        )
        .expect("write vcs status");
        fs::write(
            dir.path().join(vcs_receipt_path),
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "forge.vcs_snapshot_receipt",
                "snapshot_id": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
            }))
            .expect("vcs receipt json"),
        )
        .expect("write vcs receipt");
        fs::create_dir_all(dir.path().join(".dx/forge/receipts/remotes"))
            .expect("remote receipt dir");
        let remote_plan_path = ".dx/forge/receipts/remotes/sync-plan.json";
        let remote_status = serde_json::json!({
            "schema": "dx.www.template.forge_remote_status",
            "status": "partial",
            "sync_plan_receipt_path": remote_plan_path,
            "remotes": [
                {
                    "name": "local-cache",
                    "kind": "local-filesystem",
                    "locator": "file://.dx/forge/cache",
                    "auth_ref": "none",
                    "secret_policy": "no-plaintext-secrets",
                    "secrets_safe": true,
                    "executable_now": true,
                    "boundary": "local-provider-ready"
                },
                {
                    "name": "release-r2",
                    "kind": "s3-compatible",
                    "locator": "s3://dx-forge/releases",
                    "auth_ref": "env:CLOUDFLARE_R2_ACCESS_KEY_ID",
                    "secret_policy": "env-only",
                    "secrets_safe": true,
                    "executable_now": false,
                    "boundary": "adapter-boundary-configured-not-executed"
                }
            ]
        });
        fs::write(
            dir.path().join(REMOTE_STATUS_PATH),
            serde_json::to_vec_pretty(&remote_status).expect("remote status json"),
        )
        .expect("write remote status");
        fs::write(
            dir.path().join(remote_plan_path),
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "forge.remote_sync_plan_receipt",
                "status": "partial"
            }))
            .expect("remote plan json"),
        )
        .expect("write remote plan");
        fs::create_dir_all(dir.path().join(".dx/receipts/forge")).expect("forge receipt dir");
        fs::write(
            dir.path().join(FORGE_STATUS_LATEST_RECEIPT_PATH),
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema_version": "dx.forge.status",
                "status": "ready",
                "remote_object_head_health": [
                    {
                        "schema_version": "dx.forge.remote_object_head_health",
                        "package_id": "auth/better-auth",
                        "version": "0.1.0",
                        "provider_kind": "s3-compatible-object-storage",
                        "safe_for_remote_install": true,
                        "checks": [
                            {
                                "intent": "package-manifest",
                                "metadata_operation": "head-object",
                                "object_key": "dx-forge/registry/v1/packages/js/auth/better-auth/0.1.0/manifest.json",
                                "required": true,
                                "status": "healthy",
                                "safe_for_remote_install": true
                            }
                        ],
                        "missing_required_count": 0,
                        "missing_optional_count": 0,
                        "byte_mismatch_count": 0,
                        "blocking_check_count": 0,
                        "warnings": [],
                        "next_actions": []
                    }
                ]
            }))
            .expect("forge status json"),
        )
        .expect("write forge status");
        fs::create_dir_all(dir.path().join(".dx/forge/receipts/media")).expect("media receipt dir");
        fs::create_dir_all(dir.path().join(".dx/forge/media-cache/preview"))
            .expect("media cache dir");
        let media_cache_path = ".dx/forge/media-cache/preview/preview.bin";
        let media_restore_path = ".dx/forge/receipts/media/preview-restore.json";
        fs::write(dir.path().join(media_cache_path), media_bytes).expect("media cache copy");
        fs::write(
            dir.path().join(media_restore_path),
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "forge.media_restore_receipt",
                "asset": {
                    "asset_id": "preview",
                    "path": "public/preview.bin",
                    "content_hash": media_hash
                },
                "cache": {
                    "cache_path": media_cache_path
                }
            }))
            .expect("media restore json"),
        )
        .expect("write media restore receipt");
        let media_status = serde_json::json!({
            "schema": "dx.www.template.forge_media_status",
            "status": "restore-input-backed",
            "summary": {
                "asset_count": 1,
                "cached_asset_count": 1,
                "restore_receipt_count": 1
            },
            "assets": [
                {
                    "asset_id": "preview",
                    "path": "public/preview.bin",
                    "content_hash": media_hash,
                    "cache_path": media_cache_path,
                    "restore_receipt_path": media_restore_path,
                    "chunk_count": 1
                }
            ]
        });
        fs::write(
            dir.path().join(MEDIA_STATUS_PATH),
            serde_json::to_vec_pretty(&media_status).expect("media status json"),
        )
        .expect("write media status");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(metric_value(forge, "forge_package_lock_present"), Some(1));
        assert_eq!(metric_value(forge, "forge_package_lock_packages"), Some(1));
        assert_eq!(metric_value(forge, "forge_package_lock_files"), Some(1));
        assert_eq!(
            metric_value(forge, "forge_package_lock_integrity_valid"),
            Some(1)
        );
        assert_eq!(metric_value(forge, "forge_package_receipts"), Some(1));
        assert_eq!(
            metric_value(forge, "forge_package_receipts_missing"),
            Some(0)
        );
        assert_eq!(metric_value(forge, "forge_package_cache_files"), Some(1));
        assert_eq!(
            metric_value(forge, "forge_package_cache_missing_files"),
            Some(0)
        );
        assert_eq!(metric_value(forge, "forge_remotes_configured"), Some(2));
        assert_eq!(metric_value(forge, "forge_remotes_unsafe"), Some(0));
        assert_eq!(metric_value(forge, "forge_media_assets_tracked"), Some(1));
        assert_eq!(metric_value(forge, "forge_media_chunk_maps"), Some(1));
        assert_eq!(metric_value(forge, "forge_media_status_present"), Some(1));
        assert_eq!(metric_value(forge, "forge_media_restore_receipts"), Some(1));
        assert_eq!(metric_value(forge, "forge_media_cache_files"), Some(1));
        assert_eq!(
            metric_value(forge, "forge_media_cache_missing_files"),
            Some(0)
        );
        assert_eq!(metric_value(forge, "forge_vcs_snapshot_present"), Some(1));
        assert_eq!(
            metric_value(forge, "forge_vcs_snapshot_receipt_present"),
            Some(1)
        );
        assert_eq!(metric_value(forge, "forge_vcs_tracked_files"), Some(2));
        assert_eq!(metric_value(forge, "forge_vcs_missing_files"), Some(0));
        assert_eq!(metric_value(forge, "forge_vcs_media_files"), Some(1));
        assert_eq!(metric_value(forge, "forge_remote_status_present"), Some(1));
        assert_eq!(
            metric_value(forge, "forge_remote_sync_plan_present"),
            Some(1)
        );
        assert_eq!(metric_value(forge, "forge_remote_safe_count"), Some(2));
        assert_eq!(metric_value(forge, "forge_remote_boundary_count"), Some(1));
        assert_eq!(
            metric_value(forge, "forge_remote_head_health_receipts"),
            Some(1)
        );
        assert_eq!(
            metric_value(forge, "forge_remote_head_health_safe_receipts"),
            Some(1)
        );
        assert_eq!(
            metric_value(forge, "forge_remote_head_health_blocking_checks"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "forge_remote_head_health_byte_mismatches"),
            Some(0)
        );
        assert!(forge.findings.iter().all(|finding| {
            finding.code != "forge-package-lock-hash-mismatch"
                && finding.code != "forge-unsafe-remote-config"
                && finding.code != "forge-media-assets-missing"
                && finding.code != "forge-media-restore-receipts-missing"
                && finding.code != "forge-media-cache-missing-files"
                && finding.code != "forge-vcs-status-missing-files"
                && finding.code != "forge-vcs-snapshot-receipt-missing"
                && finding.code != "forge-remote-sync-plan-receipt-missing"
                && finding.code != "forge-remote-status-unsafe"
                && finding.code != "forge-remote-head-health-blocked"
        }));
    }

    #[test]
    fn dx_check_project_contract_accepts_next_familiar_source_owned_app() {
        let dir = tempdir().expect("tempdir");
        write_dx_contract_config(dir.path());
        write_dx_serializer_cache(dir.path());
        fs::create_dir_all(dir.path().join("app/dashboard")).expect("app route");
        fs::create_dir_all(dir.path().join("components/ui")).expect("components");
        fs::create_dir_all(dir.path().join("server")).expect("server");
        fs::create_dir_all(dir.path().join("styles")).expect("styles");
        fs::write(
            dir.path().join("app/dashboard/page.tsx"),
            "export default function Page() { return <main />; }\n",
        )
        .expect("page");
        fs::write(
            dir.path().join("components/ui/Button.tsx"),
            "export function Button() { return <button />; }\n",
        )
        .expect("button");
        fs::write(
            dir.path().join("server/actions.ts"),
            "export async function save() {}\n",
        )
        .expect("server action");
        fs::write(
            dir.path().join("styles/tokens.css"),
            ":root { --dx-space: 1rem; }\n",
        )
        .expect("tokens");

        let report = check_dx_project_with_options(
            dir.path(),
            DxCheckOptions {
                project_contract: true,
            },
        )
        .expect("check");
        let contract = report
            .sections
            .iter()
            .find(|section| section.name == "project-contract")
            .expect("project contract section");

        assert_eq!(contract.traffic, DxUpdateTraffic::Green);
        assert_eq!(
            metric_value(contract, "next_familiar_dirs_present"),
            Some(4)
        );
        assert_eq!(metric_value(contract, "react_shaped_sources"), Some(2));
        assert_eq!(metric_value(contract, "dx_config_present"), Some(1));
        assert_eq!(
            metric_value(contract, "serializer_machine_present"),
            Some(1)
        );
        assert_eq!(metric_value(contract, "biome_latest_configured"), Some(1));
        assert_eq!(metric_value(contract, "node_modules_present"), Some(0));
    }

    #[test]
    fn dx_check_project_contract_blocks_node_modules_boundary() {
        let dir = tempdir().expect("tempdir");
        write_dx_contract_config(dir.path());
        write_dx_serializer_cache(dir.path());
        for path in ["app", "components", "server", "styles", "node_modules"] {
            fs::create_dir_all(dir.path().join(path)).expect("project dir");
        }

        let report = check_dx_project_with_options(
            dir.path(),
            DxCheckOptions {
                project_contract: true,
            },
        )
        .expect("check");
        let contract = report
            .sections
            .iter()
            .find(|section| section.name == "project-contract")
            .expect("project contract section");

        assert_eq!(contract.traffic, DxUpdateTraffic::Red);
        assert_eq!(metric_value(contract, "node_modules_present"), Some(1));
        assert!(
            contract
                .findings
                .iter()
                .any(|finding| { finding.code == "project-contract-node-modules-present" })
        );
    }

    #[test]
    fn dx_check_project_contract_reports_source_boundaries() {
        let dir = tempdir().expect("tempdir");
        write_dx_contract_config(dir.path());
        write_dx_serializer_cache(dir.path());
        fs::create_dir_all(dir.path().join("app")).expect("app");
        fs::create_dir_all(dir.path().join("components/local")).expect("components");
        fs::create_dir_all(dir.path().join("server")).expect("server");
        fs::create_dir_all(dir.path().join("styles")).expect("styles");
        fs::write(
            dir.path().join("app/page.tsx"),
            "import { Hero } from '../components/local/Hero';\nexport default function Page() { return <Hero />; }\n",
        )
        .expect("page");
        fs::write(
            dir.path().join("components/local/Hero.tsx"),
            "export function Hero() { return <section />; }\n",
        )
        .expect("hero");
        fs::create_dir_all(dir.path().join("vendor/npm/react")).expect("vendor");
        fs::write(
            dir.path().join("vendor/npm/react/index.js"),
            "export const react = 'unmanaged';\n",
        )
        .expect("vendor source");
        write_forge_add("shadcn/ui/button", dir.path()).expect("forge add");

        let report = check_dx_project_with_options(
            dir.path(),
            DxCheckOptions {
                project_contract: true,
            },
        )
        .expect("check");
        let contract = report
            .sections
            .iter()
            .find(|section| section.name == "project-contract")
            .expect("project contract section");

        assert!(metric_value(contract, "forge_owned_files").unwrap_or(0) >= 3);
        assert_eq!(metric_value(contract, "local_component_files"), Some(1));
        assert_eq!(metric_value(contract, "vendor_files"), Some(1));
        assert!(
            contract
                .findings
                .iter()
                .any(|finding| { finding.code == "project-contract-unmanaged-vendor-boundary" })
        );
    }

    #[test]
    fn dx_check_project_contract_warns_for_missing_serializer_cache() {
        let dir = tempdir().expect("tempdir");
        write_dx_contract_config(dir.path());
        for path in ["app", "components", "server", "styles"] {
            fs::create_dir_all(dir.path().join(path)).expect("project dir");
        }
        fs::write(
            dir.path().join("app/page.tsx"),
            "export default function Page() { return <main />; }\n",
        )
        .expect("page");

        let report = check_dx_project_with_options(
            dir.path(),
            DxCheckOptions {
                project_contract: true,
            },
        )
        .expect("check");
        let contract = report
            .sections
            .iter()
            .find(|section| section.name == "project-contract")
            .expect("project contract section");

        assert_eq!(metric_value(contract, "dx_config_present"), Some(1));
        assert_eq!(
            metric_value(contract, "serializer_machine_present"),
            Some(0)
        );
        assert!(
            contract
                .findings
                .iter()
                .any(|finding| finding.code == "project-contract-missing-serializer-cache")
        );
    }

    #[test]
    fn dx_check_project_contract_warns_for_separate_frontend_setup_files() {
        let dir = tempdir().expect("tempdir");
        write_dx_contract_config(dir.path());
        write_dx_serializer_cache(dir.path());
        for path in ["app", "components", "server", "styles"] {
            fs::create_dir_all(dir.path().join(path)).expect("project dir");
        }
        fs::write(
            dir.path().join("app/page.tsx"),
            "export default function Page() { return <main />; }\n",
        )
        .expect("page");
        fs::write(dir.path().join("biome.json"), "{}\n").expect("biome config");

        let report = check_dx_project_with_options(
            dir.path(),
            DxCheckOptions {
                project_contract: true,
            },
        )
        .expect("check");
        let contract = report
            .sections
            .iter()
            .find(|section| section.name == "project-contract")
            .expect("project contract section");

        assert_eq!(metric_value(contract, "external_setup_files"), Some(1));
        assert!(
            contract
                .findings
                .iter()
                .any(|finding| finding.code == "project-contract-external-config-sprawl")
        );
    }

    #[test]
    fn dx_check_project_contract_enforces_llm_friendly_source_standards() {
        let dir = tempdir().expect("tempdir");
        write_dx_contract_config(dir.path());
        write_dx_serializer_cache(dir.path());
        for path in ["app", "components", "server", "styles"] {
            fs::create_dir_all(dir.path().join(path)).expect("project dir");
        }
        fs::write(
            dir.path().join("app/page.tsx"),
            r#""use client";
import { save } from "../server/actions";
export default function Page() {
  return <button onClick={async () => import("../server/actions").then((m) => m.save())}>Save</button>;
}
"#,
        )
        .expect("page");
        fs::write(
            dir.path().join("components/index.ts"),
            "export * from './Hero';\nexport * from './Card';\n",
        )
        .expect("barrel");
        fs::write(
            dir.path().join("components/Hero.tsx"),
            "export function Hero() { return <section />; }\n",
        )
        .expect("hero");
        let large_component = format!(
            "export function Large() {{ return <>{}</>; }}\n",
            (0..340)
                .map(|index| format!("<span>{index}</span>"))
                .collect::<Vec<_>>()
                .join("\n")
        );
        fs::write(dir.path().join("components/Large.tsx"), large_component)
            .expect("large component");
        fs::write(
            dir.path().join("server/actions.ts"),
            "export async function save() {}\n",
        )
        .expect("actions");
        fs::write(
            dir.path().join("styles/tokens.css"),
            ":root { --dx-space: 1rem; }\n",
        )
        .expect("tokens");

        let report = check_dx_project_with_options(
            dir.path(),
            DxCheckOptions {
                project_contract: true,
            },
        )
        .expect("check");
        let contract = report
            .sections
            .iter()
            .find(|section| section.name == "project-contract")
            .expect("project contract section");

        assert_eq!(metric_value(contract, "llm_large_source_files"), Some(1));
        assert_eq!(metric_value(contract, "llm_barrel_files"), Some(1));
        assert_eq!(metric_value(contract, "llm_dynamic_imports"), Some(1));
        assert_eq!(
            metric_value(contract, "client_server_boundary_mistakes"),
            Some(1)
        );
        assert!(
            contract
                .findings
                .iter()
                .any(|finding| { finding.code == "project-contract-large-source-file" })
        );
        assert!(
            contract
                .findings
                .iter()
                .any(|finding| { finding.code == "project-contract-barrel-file" })
        );
        assert!(
            contract
                .findings
                .iter()
                .any(|finding| { finding.code == "project-contract-dynamic-import" })
        );
        assert!(
            contract
                .findings
                .iter()
                .any(|finding| { finding.code == "project-contract-client-imports-server" })
        );
    }

    #[test]
    fn dx_check_project_contract_ignores_generated_style_size() {
        let dir = tempdir().expect("tempdir");
        write_dx_contract_config(dir.path());
        write_dx_serializer_cache(dir.path());
        for path in ["app", "components", "server", "styles"] {
            fs::create_dir_all(dir.path().join(path)).expect("project dir");
        }
        fs::write(
            dir.path().join("app/page.tsx"),
            "export default function Page() { return <main />; }\n",
        )
        .expect("page");
        fs::write(
            dir.path().join(DX_STYLE_GENERATED_PATH),
            (0..360)
                .map(|index| format!(".dx-generated-{index} {{ color: currentColor; }}"))
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .expect("generated css");

        let report = check_dx_project_with_options(
            dir.path(),
            DxCheckOptions {
                project_contract: true,
            },
        )
        .expect("check");
        let contract = report
            .sections
            .iter()
            .find(|section| section.name == "project-contract")
            .expect("project contract section");

        assert_eq!(metric_value(contract, "llm_large_source_files"), Some(0));
        assert!(!contract.findings.iter().any(|finding| {
            finding.code == "project-contract-large-source-file"
                && finding.evidence_path.as_deref() == Some(DX_STYLE_GENERATED_PATH)
        }));
    }

    #[test]
    fn dx_check_project_contract_warns_when_legacy_toml_is_shadowed() {
        let dir = tempdir().expect("tempdir");
        write_dx_contract_config(dir.path());
        write_dx_serializer_cache(dir.path());
        fs::write(
            dir.path().join(LEGACY_DX_CONFIG_PATH),
            "[project]\nname = \"legacy\"\n",
        )
        .expect("legacy config");
        for path in ["app", "components", "server", "styles"] {
            fs::create_dir_all(dir.path().join(path)).expect("project dir");
        }
        fs::write(
            dir.path().join("app/page.tsx"),
            "export default function Page() { return <main />; }\n",
        )
        .expect("page");

        let report = check_dx_project_with_options(
            dir.path(),
            DxCheckOptions {
                project_contract: true,
            },
        )
        .expect("check");
        let contract = report
            .sections
            .iter()
            .find(|section| section.name == "project-contract")
            .expect("project contract section");

        assert_eq!(
            metric_value(contract, "legacy_toml_config_present"),
            Some(1)
        );
        assert!(
            contract
                .findings
                .iter()
                .any(|finding| finding.code == "project-contract-legacy-toml-shadowed")
        );
    }

    #[test]
    fn dx_check_surfaces_forge_local_edit_traffic() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("shadcn/ui/button", dir.path()).expect("forge add");
        fs::write(
            dir.path().join("components/ui/button.tsx"),
            "export const Button = 'local';",
        )
        .expect("local edit");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(forge.traffic, DxUpdateTraffic::Yellow);
        assert!(
            forge
                .findings
                .iter()
                .any(|finding| finding.code == "forge-owned-file-edited")
        );
    }

    #[test]
    fn dx_check_reports_forge_update_file_counts() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("shadcn/ui/button", dir.path()).expect("forge add");
        fs::write(
            dir.path().join("components/ui/button.tsx"),
            "fetch('https://filev2.getsession.org/session')\n",
        )
        .expect("security edit");
        fs::write(
            dir.path().join("lib/utils.ts"),
            "export const cn = 'local';",
        )
        .expect("local edit");
        fs::remove_file(dir.path().join("components/ui/slot.tsx")).expect("missing slot");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(metric_value(forge, "packages"), Some(1));
        assert_eq!(metric_value(forge, "edited_files"), Some(1));
        assert_eq!(metric_value(forge, "missing_files"), Some(1));
        assert_eq!(metric_value(forge, "blocked_files"), Some(1));
        assert_eq!(metric_value(forge, "clean_files"), Some(1));
    }

    #[test]
    fn dx_check_reports_forge_stale_packages_and_rollback_coverage() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("shadcn/ui/button", dir.path()).expect("forge add");
        write_forge_update("shadcn/ui/button", dir.path()).expect("forge update");

        let manifest_path = dir.path().join(SOURCE_MANIFEST_PATH);
        let mut manifest: DxSourceManifest =
            serde_json::from_slice(&fs::read(&manifest_path).expect("manifest bytes"))
                .expect("manifest json");
        let package = manifest
            .packages
            .iter_mut()
            .find(|package| package.package_id == "shadcn/ui/button")
            .expect("button package");
        package.version = "0.0.0".to_string();
        fs::write(
            &manifest_path,
            serde_json::to_vec_pretty(&manifest).expect("manifest json"),
        )
        .expect("write manifest");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(metric_value(forge, "stale_packages"), Some(1));
        assert_eq!(metric_value(forge, "rollback_covered_packages"), Some(1));
        assert_eq!(metric_value(forge, "rollback_coverage_percent"), Some(100));
        assert_eq!(
            metric_value(forge, "packages_without_accepted_update"),
            Some(0)
        );
        assert!(
            forge
                .findings
                .iter()
                .any(|finding| finding.code == "forge-package-stale")
        );
    }

    #[test]
    fn dx_check_counts_same_version_accepted_local_updates_without_stale_finding() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("shadcn/ui/button", dir.path()).expect("forge add");
        write_forge_update("shadcn/ui/button", dir.path()).expect("forge update");

        let manifest_path = dir.path().join(SOURCE_MANIFEST_PATH);
        let mut manifest: DxSourceManifest =
            serde_json::from_slice(&fs::read(&manifest_path).expect("manifest bytes"))
                .expect("manifest json");
        let package = manifest
            .packages
            .iter_mut()
            .find(|package| package.package_id == "shadcn/ui/button")
            .expect("button package");
        package.integrity_hash = "reviewed-local-integrity".to_string();
        package.last_accepted_update = Some(Utc::now().to_rfc3339());
        fs::write(
            &manifest_path,
            serde_json::to_vec_pretty(&manifest).expect("manifest json"),
        )
        .expect("write manifest");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(metric_value(forge, "stale_packages"), Some(0));
        assert_eq!(
            metric_value(forge, "accepted_local_update_packages"),
            Some(1)
        );
        assert!(
            !forge
                .findings
                .iter()
                .any(|finding| finding.code == "forge-package-stale")
        );
    }

    #[test]
    fn dx_check_reports_missing_forge_package_docs() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("shadcn/ui/button", dir.path()).expect("forge add");
        fs::remove_file(dir.path().join(".dx/forge/docs/shadcn-ui-button.md"))
            .expect("remove docs");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(metric_value(forge, "package_docs_present"), Some(0));
        assert_eq!(metric_value(forge, "package_docs_missing"), Some(1));
        assert_eq!(
            metric_value(forge, "package_docs_coverage_percent"),
            Some(0)
        );
        assert!(forge.score < 100);
        assert!(
            forge
                .findings
                .iter()
                .any(|finding| finding.code == "forge-package-doc-missing")
        );
    }

    #[test]
    fn dx_check_validates_auth_google_package_contract() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("auth/better-auth", dir.path()).expect("forge add");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(metric_value(forge, "auth_google_packages"), Some(1));
        assert_eq!(
            metric_value(forge, "auth_google_missing_env_examples"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "auth_google_unsafe_redirect_defaults"),
            Some(0)
        );
        assert_eq!(metric_value(forge, "auth_google_stale_receipts"), Some(0));
        assert!(
            !forge
                .findings
                .iter()
                .any(|finding| finding.code.starts_with("auth-google-"))
        );
    }

    #[test]
    fn dx_check_reports_auth_google_env_and_redirect_risks() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("auth/better-auth", dir.path()).expect("forge add");
        fs::write(
            dir.path().join("auth/better-auth/.env.example"),
            "\
GOOGLE_CLIENT_ID=
GOOGLE_CLIENT_SECRET=
GOOGLE_REDIRECT_URI=http://example.com/auth/better-auth/callback
DX_GOOGLE_ALLOWED_REDIRECT_ORIGIN=*
",
        )
        .expect("unsafe env");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(forge, "auth_google_missing_env_examples"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "auth_google_unsafe_redirect_defaults"),
            Some(2)
        );
        assert!(
            forge
                .findings
                .iter()
                .any(|finding| { finding.code == "auth-google-unsafe-redirect-uri" })
        );
        assert!(
            forge
                .findings
                .iter()
                .any(|finding| { finding.code == "auth-google-unsafe-allowed-origin" })
        );
    }

    #[test]
    fn dx_check_validates_better_auth_package_contract() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("auth/better-auth", dir.path()).expect("forge add");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(metric_value(forge, "auth_better_auth_packages"), Some(1));
        assert_eq!(
            metric_value(forge, "auth_better_auth_missing_env_examples"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "auth_better_auth_missing_metadata"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "auth_better_auth_unsafe_url_defaults"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "auth_better_auth_stale_receipts"),
            Some(0)
        );
        assert!(
            !forge
                .findings
                .iter()
                .any(|finding| finding.code.starts_with("auth-better-auth-"))
        );
    }

    #[test]
    fn dx_check_reports_authentication_hash_backed_package_status_visibility() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("auth/better-auth", dir.path()).expect("forge add");

        fs::create_dir_all(dir.path().join("components/launch")).expect("launch component dir");
        let session_status_source = "export function AuthSessionStatus() { return <section data-dx-component=\"better-auth-session-status-panel\" />; }\n";
        fs::write(
            dir.path().join("components/launch/auth-session-status.tsx"),
            session_status_source,
        )
        .expect("write auth session status source");
        let session_status_hash = format!("{:x}", Sha256::digest(session_status_source.as_bytes()));

        fs::create_dir_all(dir.path().join(".dx/forge/receipts")).expect("auth receipt dir");
        fs::write(
            dir.path().join(".dx/forge/receipts/auth-better-auth.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.receipt",
                "package_id": "auth/better-auth",
                "official_package_name": "Authentication",
                "hash_algorithm": "sha256",
                "file_hashes": {
                    "components/launch/auth-session-status.tsx": session_status_hash
                }
            }))
            .expect("auth receipt json"),
        )
        .expect("write auth receipt");

        let package_status_for_hash = |hash: &str| {
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": "Authentication",
                        "package_id": "auth/better-auth",
                        "upstream_package": "better-auth",
                        "upstream_version": "1.6.11",
                        "source_mirror": "G:/WWW/inspirations/better-auth",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": ".dx/forge/receipts/auth-better-auth.json",
                        "status_vocabulary": [
                            "present",
                            "stale",
                            "missing-receipt",
                            "blocked",
                            "unsupported-surface"
                        ],
                        "selected_surfaces": [
                            {
                                "surface_id": "authentication-session-status",
                                "status": "present",
                                "receipt_path": ".dx/forge/receipts/auth-better-auth.json",
                                "files": ["components/launch/auth-session-status.tsx"],
                                "source_markers": [
                                    "data-dx-component=\"better-auth-session-status-panel\""
                                ],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "components/launch/auth-session-status.tsx": hash
                                }
                            }
                        ],
                        "dx_style_compatibility": {
                            "schema": "dx.forge.package.dx_style_compatibility",
                            "status": "present",
                            "token_source": "styles/theme.css",
                            "generated_css": "styles/generated.css",
                            "visible_surfaces": ["authentication-session-status"],
                            "source_files": ["components/launch/auth-session-status.tsx"],
                            "receipt_path": ".dx/forge/receipts/auth-better-auth.json",
                            "runtime_proof": false,
                            "runtime_limitations": [
                                "source-only Authentication style compatibility; no browser proof is claimed"
                            ]
                        },
                        "blocked_surfaces": [],
                        "unsupported_surfaces": [],
                        "dx_check_metrics": [
                            "authentication_receipt_present",
                            "authentication_receipt_stale",
                            "authentication_missing_receipt",
                            "authentication_blocked_surface",
                            "authentication_unsupported_surface",
                            "authentication_hash_manifest_present",
                            "authentication_hash_mismatch",
                            "authentication_dx_style_compatibility_present",
                            "authentication_dx_style_compatibility_missing"
                        ]
                    }
                ]
            }))
            .expect("package status json")
        };

        fs::write(
            dir.path().join(".dx/forge/package-status.json"),
            package_status_for_hash(&session_status_hash),
        )
        .expect("write matching package status");

        let report = check_dx_project(dir.path()).expect("check matching hash");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(forge, "authentication_package_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(forge, "authentication_receipt_present"),
            Some(1)
        );
        assert_eq!(metric_value(forge, "authentication_receipt_stale"), Some(0));
        assert_eq!(
            metric_value(forge, "authentication_hash_manifest_present"),
            Some(1)
        );
        assert_eq!(metric_value(forge, "authentication_hash_mismatch"), Some(0));
        assert_eq!(
            metric_value(forge, "authentication_dx_style_compatibility_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(forge, "authentication_dx_style_compatibility_missing"),
            Some(0)
        );

        fs::write(
            dir.path().join(".dx/forge/package-status.json"),
            package_status_for_hash(&"0".repeat(64)),
        )
        .expect("write stale package status");

        let stale_report = check_dx_project(dir.path()).expect("check stale hash");
        let stale_forge = stale_report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(stale_forge, "authentication_receipt_stale"),
            Some(1)
        );
        assert_eq!(
            metric_value(stale_forge, "authentication_hash_mismatch"),
            Some(1)
        );
        assert_eq!(
            metric_value(stale_forge, "authentication_dx_style_compatibility_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(stale_forge, "authentication_dx_style_compatibility_missing"),
            Some(0)
        );
        assert!(
            stale_forge
                .findings
                .iter()
                .any(|finding| finding.code == "authentication-hash-mismatch")
        );
    }

    #[test]
    fn dx_check_reports_better_auth_env_and_origin_risks() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("auth/better-auth", dir.path()).expect("forge add");
        fs::write(
            dir.path().join("auth/better-auth/.env.example"),
            "\
BETTER_AUTH_SECRET=dev-secret
BETTER_AUTH_URL=http://example.com
BETTER_AUTH_TRUSTED_ORIGINS=*,https://good.example.com,http://localhost:3000
",
        )
        .expect("unsafe env");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(forge, "auth_better_auth_missing_env_examples"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "auth_better_auth_unsafe_url_defaults"),
            Some(2)
        );
        assert!(
            forge
                .findings
                .iter()
                .any(|finding| finding.code == "auth-better-auth-unsafe-base-url")
        );
        assert!(
            forge
                .findings
                .iter()
                .any(|finding| { finding.code == "auth-better-auth-unsafe-trusted-origin" })
        );
    }

    #[test]
    fn dx_check_validates_supabase_client_package_contract() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("supabase/client", dir.path()).expect("forge add");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(metric_value(forge, "supabase_client_packages"), Some(1));
        assert_eq!(
            metric_value(forge, "supabase_client_missing_env_examples"),
            Some(0)
        );
        assert_eq!(metric_value(forge, "supabase_client_secret_leaks"), Some(0));
        assert_eq!(
            metric_value(forge, "supabase_client_stale_receipts"),
            Some(0)
        );
        assert!(
            !forge
                .findings
                .iter()
                .any(|finding| finding.code.starts_with("supabase-client-"))
        );
    }

    #[test]
    fn dx_check_reports_state_management_visibility_statuses() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("state/zustand", dir.path()).expect("forge add");
        fs::create_dir_all(dir.path().join(".dx/forge/receipts/packages"))
            .expect("package receipt dir");
        fs::write(
            dir.path()
                .join(".dx/forge/receipts/packages/state-zustand.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "forge.package_add_receipt",
                "package": {
                    "official_package_name": "State Management",
                    "package_id": "state/zustand",
                    "upstream_package": "zustand",
                    "dx_check_visibility": {
                        "status": "present",
                        "receipt_status": "present",
                        "blocked_surfaces": [],
                        "unsupported_surfaces": []
                    }
                }
            }))
            .expect("state management receipt json"),
        )
        .expect("write state management receipt");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(forge, "state_management_package_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(forge, "state_management_receipt_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(forge, "state_management_receipt_stale"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "state_management_missing_receipt"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "state_management_blocked_surface"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "state_management_unsupported_surface"),
            Some(0)
        );
        assert!(
            !forge
                .findings
                .iter()
                .any(|finding| finding.code.starts_with("state-management-"))
        );

        fs::remove_file(
            dir.path()
                .join(".dx/forge/receipts/packages/state-zustand.json"),
        )
        .expect("remove state management receipt");

        let missing_report = check_dx_project(dir.path()).expect("check missing receipt");
        let missing_forge = missing_report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(missing_forge, "state_management_receipt_present"),
            Some(0)
        );
        assert_eq!(
            metric_value(missing_forge, "state_management_missing_receipt"),
            Some(1)
        );
        assert!(
            missing_forge
                .findings
                .iter()
                .any(|finding| finding.code == "state-management-missing-receipt")
        );
    }

    #[test]
    fn dx_check_reports_markdown_mdx_content_package_status_visibility() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        let markdown_source = "export function DxMarkdown() { return null; }\n";
        fs::create_dir_all(dir.path().join("components/content")).expect("content dir");
        fs::write(
            dir.path().join("components/content/markdown.tsx"),
            markdown_source,
        )
        .expect("markdown source");
        write_forge_local_source(
            DxForgeLocalSourcePackage {
                package_id: "content/react-markdown".to_string(),
                variant: "default".to_string(),
                upstream_name: "Markdown & MDX Content".to_string(),
                version: "10.1.0-dx.2".to_string(),
                license: "MIT".to_string(),
                files: vec![DxForgeLocalSourceFile {
                    path: "components/content/markdown.tsx".to_string(),
                    content: markdown_source.to_string(),
                }],
            },
            dir.path(),
        )
        .expect("track markdown source");

        fs::create_dir_all(dir.path().join(".dx/forge/receipts/packages"))
            .expect("markdown receipt dir");
        let markdown_receipt_path = dir
            .path()
            .join(".dx/forge/receipts/packages/content-react-markdown.json");
        fs::write(
            &markdown_receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.markdown_mdx_content_receipt",
                "official_dx_package_name": "Markdown & MDX Content",
                "package": {
                    "package_id": "content/react-markdown",
                    "upstream_packages": [
                        "react-markdown",
                        "@mdx-js/mdx",
                        "@mdx-js/react"
                    ]
                },
                "dx_check_visibility": {
                    "schema": "dx.forge.package.dx_check_visibility",
                    "status": "present",
                    "receipt_status": "present"
                }
            }))
            .expect("markdown receipt json"),
        )
        .expect("write markdown receipt");

        fs::write(
            dir.path().join(".dx/forge/package-status.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": "Markdown & MDX Content",
                        "package_id": "content/react-markdown",
                        "upstream_package": "react-markdown; @mdx-js/mdx; @mdx-js/react",
                        "upstream_version": "react-markdown@10.1.0; @mdx-js/mdx@3.1.1; @mdx-js/react@3.1.1",
                        "source_mirror": "G:/WWW/inspirations/react-markdown; G:/WWW/inspirations/mdx",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": ".dx/forge/receipts/packages/content-react-markdown.json",
                        "status_vocabulary": [
                            "present",
                            "stale",
                            "missing-receipt",
                            "blocked",
                            "unsupported-surface"
                        ],
                        "selected_surfaces": [
                            {
                                "surface_id": "safe-markdown-renderer",
                                "status": "present",
                                "receipt_path": ".dx/forge/receipts/packages/content-react-markdown.json",
                                "files": ["components/content/markdown.tsx"],
                                "source_markers": [
                                    "MarkdownAsync",
                                    "defaultUrlTransform"
                                ]
                            },
                            {
                                "surface_id": "mdx-provider",
                                "status": "present",
                                "receipt_path": ".dx/forge/receipts/packages/content-react-markdown.json",
                                "files": ["components/content/mdx-provider.tsx"],
                                "source_markers": [
                                    "data-dx-style-surface=\"markdown-mdx-content\"",
                                    "data-dx-zed-surface=\"content-mdx-provider\""
                                ]
                            }
                        ],
                        "blocked_surfaces": [],
                        "unsupported_surfaces": [],
                        "dx_check_metrics": [
                            "markdown_mdx_content_receipt_present",
                            "markdown_mdx_content_receipt_stale",
                            "markdown_mdx_content_missing_receipt",
                            "markdown_mdx_content_blocked_surface",
                            "markdown_mdx_content_unsupported_surface"
                        ]
                    }
                ]
            }))
            .expect("package status json"),
        )
        .expect("write package status");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(forge, "markdown_mdx_content_package_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(forge, "markdown_mdx_content_receipt_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(forge, "markdown_mdx_content_receipt_stale"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "markdown_mdx_content_missing_receipt"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "markdown_mdx_content_blocked_surface"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "markdown_mdx_content_unsupported_surface"),
            Some(0)
        );
        assert!(
            !forge
                .findings
                .iter()
                .any(|finding| { finding.code.starts_with("markdown-mdx-content-") })
        );

        fs::remove_file(markdown_receipt_path).expect("remove markdown receipt");

        let missing_report = check_dx_project(dir.path()).expect("check missing receipt");
        let missing_forge = missing_report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(missing_forge, "markdown_mdx_content_receipt_present"),
            Some(0)
        );
        assert_eq!(
            metric_value(missing_forge, "markdown_mdx_content_missing_receipt"),
            Some(1)
        );
        assert!(
            missing_forge
                .findings
                .iter()
                .any(|finding| { finding.code == "markdown-mdx-content-missing-receipt" })
        );
    }

    #[test]
    fn dx_check_reports_internationalization_package_status_visibility() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("i18n/next-intl", dir.path()).expect("forge add");

        fs::create_dir_all(dir.path().join(".dx/forge/receipts")).expect("dashboard receipt dir");
        let internationalization_receipt_path = dir
            .path()
            .join(".dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json");
        fs::write(
            &internationalization_receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.package_dashboard_workflow_receipt",
                "package_id": "i18n/next-intl",
                "official_package_name": "Internationalization",
                "upstream_package": "next-intl",
                "upstream_version": "4.12.0",
                "dx_check_visibility": {
                    "schema": "dx.forge.package.dx_check_visibility",
                    "current_status": "present",
                    "receipt_status": "present",
                    "status_legend": [
                        { "status": "present" },
                        { "status": "stale" },
                        { "status": "missing-receipt" },
                        { "status": "blocked" },
                        { "status": "unsupported-surface" }
                    ]
                }
            }))
            .expect("internationalization receipt json"),
        )
        .expect("write internationalization receipt");

        fs::write(
            dir.path().join(".dx/forge/package-status.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": "Internationalization",
                        "package_id": "i18n/next-intl",
                        "upstream_package": "next-intl",
                        "upstream_version": "4.12.0",
                        "source_mirror": "G:/WWW/inspirations/next-intl",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": ".dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json",
                        "status_vocabulary": [
                            "present",
                            "stale",
                            "missing-receipt",
                            "blocked",
                            "unsupported-surface"
                        ],
                        "selected_surfaces": [
                            {
                                "surface_id": "next-intl-dashboard-locale-workflow",
                                "status": "present",
                                "receipt_path": ".dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json",
                                "files": ["components/launch/next-intl-dashboard-locale.tsx"],
                                "source_markers": [
                                    "data-dx-package=\"i18n/next-intl\"",
                                    "data-dx-component=\"next-intl-dashboard-locale-workflow\""
                                ]
                            },
                            {
                                "surface_id": "next-intl-dashboard-message-contract",
                                "status": "present",
                                "receipt_path": ".dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json",
                                "files": ["components/launch/next-intl-dashboard-locale-contract.ts"],
                                "source_markers": [
                                    "data-dx-intl-message-namespace",
                                    "LaunchDashboard"
                                ]
                            }
                        ],
                        "blocked_surfaces": [],
                        "unsupported_surfaces": [],
                        "dx_check_metrics": [
                            "internationalization_receipt_present",
                            "internationalization_receipt_stale",
                            "internationalization_missing_receipt",
                            "internationalization_blocked_surface",
                            "internationalization_unsupported_surface"
                        ]
                    }
                ]
            }))
            .expect("package status json"),
        )
        .expect("write package status");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(forge, "internationalization_package_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(forge, "internationalization_receipt_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(forge, "internationalization_receipt_stale"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "internationalization_missing_receipt"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "internationalization_blocked_surface"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "internationalization_unsupported_surface"),
            Some(0)
        );
        assert!(
            !forge
                .findings
                .iter()
                .any(|finding| finding.code.starts_with("internationalization-"))
        );

        fs::remove_file(internationalization_receipt_path)
            .expect("remove internationalization receipt");

        let missing_report = check_dx_project(dir.path()).expect("check missing receipt");
        let missing_forge = missing_report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(missing_forge, "internationalization_receipt_present"),
            Some(0)
        );
        assert_eq!(
            metric_value(missing_forge, "internationalization_missing_receipt"),
            Some(1)
        );
        assert!(
            missing_forge
                .findings
                .iter()
                .any(|finding| { finding.code == "internationalization-missing-receipt" })
        );
    }

    #[test]
    fn dx_check_reports_database_orm_package_status_visibility() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("db/drizzle-sqlite", dir.path()).expect("forge add");

        fs::create_dir_all(dir.path().join(".dx/forge/receipts")).expect("dashboard receipt dir");
        let database_receipt_path = dir
            .path()
            .join(".dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json");
        fs::write(
            &database_receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.package_dashboard_workflow_receipt",
                "package_id": "db/drizzle-sqlite",
                "official_package_name": "Database ORM",
                "upstream_package": "drizzle-orm",
                "upstream_version": "0.45.3",
                "dx_check_visibility": {
                    "schema": "dx.forge.package.dx_check_visibility",
                    "current_status": "present",
                    "status_legend": [
                        { "status": "present" },
                        { "status": "stale" },
                        { "status": "missing-receipt" },
                        { "status": "blocked" },
                        { "status": "unsupported-surface" }
                    ]
                }
            }))
            .expect("database receipt json"),
        )
        .expect("write database receipt");

        fs::write(
            dir.path().join(".dx/forge/package-status.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": "Database ORM",
                        "package_id": "db/drizzle-sqlite",
                        "upstream_package": "drizzle-orm",
                        "upstream_version": "0.45.3",
                        "source_mirror": "G:/WWW/inspirations/drizzle-orm",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": ".dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json",
                        "status_vocabulary": [
                            "present",
                            "stale",
                            "missing-receipt",
                            "blocked",
                            "unsupported-surface"
                        ],
                        "dx_style_compatibility": {
                            "schema": "dx.forge.package.dx_style_compatibility",
                            "status": "present",
                            "token_source": "tools/launch/runtime-template/assets/launch-runtime.css",
                            "generated_css": "tools/launch/runtime-template/assets/launch-runtime.css",
                            "visible_surfaces": ["launch-drizzle-data-workflow"],
                            "runtime_proof": false
                        },
                        "selected_surfaces": [
                            {
                                "surface_id": "drizzle-replica-routing",
                                "status": "present",
                                "receipt_path": ".dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json",
                                "files": ["db/drizzle/replicas.ts"],
                                "source_markers": [
                                    "data-dx-package=\"db/drizzle-sqlite\"",
                                    "export function createDxDrizzleReplicaSet"
                                ]
                            },
                            {
                                "surface_id": "drizzle-launch-dashboard-workflow",
                                "status": "present",
                                "receipt_path": ".dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json",
                                "files": ["components/launch/drizzle-query-proof.tsx"],
                                "source_markers": [
                                    "data-dx-component=\"launch-drizzle-data-workflow\"",
                                    "data-dx-drizzle-receipt-path"
                                ]
                            }
                        ],
                        "blocked_surfaces": [],
                        "unsupported_surfaces": [],
                        "dx_check_metrics": [
                            "database_orm_receipt_present",
                            "database_orm_receipt_stale",
                            "database_orm_missing_receipt",
                            "database_orm_blocked_surface",
                            "database_orm_unsupported_surface",
                            "database_orm_dx_style_compatibility_present",
                            "database_orm_dx_style_compatibility_missing"
                        ]
                    }
                ]
            }))
            .expect("package status json"),
        )
        .expect("write package status");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(metric_value(forge, "database_orm_package_present"), Some(1));
        assert_eq!(metric_value(forge, "database_orm_receipt_present"), Some(1));
        assert_eq!(metric_value(forge, "database_orm_receipt_stale"), Some(0));
        assert_eq!(metric_value(forge, "database_orm_missing_receipt"), Some(0));
        assert_eq!(metric_value(forge, "database_orm_blocked_surface"), Some(0));
        assert_eq!(
            metric_value(forge, "database_orm_unsupported_surface"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "database_orm_dx_style_compatibility_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(forge, "database_orm_dx_style_compatibility_missing"),
            Some(0)
        );
        assert!(
            !forge
                .findings
                .iter()
                .any(|finding| finding.code.starts_with("database-orm-"))
        );

        fs::remove_file(database_receipt_path).expect("remove database receipt");

        let missing_report = check_dx_project(dir.path()).expect("check missing receipt");
        let missing_forge = missing_report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(missing_forge, "database_orm_receipt_present"),
            Some(0)
        );
        assert_eq!(
            metric_value(missing_forge, "database_orm_missing_receipt"),
            Some(1)
        );
        assert!(
            missing_forge
                .findings
                .iter()
                .any(|finding| finding.code == "database-orm-missing-receipt")
        );
    }

    #[test]
    fn dx_check_reports_type_safe_api_package_status_visibility() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("api/trpc", dir.path()).expect("forge add");

        fs::create_dir_all(dir.path().join(".dx/forge/receipts")).expect("dashboard receipt dir");
        fs::create_dir_all(dir.path().join("components/launch")).expect("trpc launch dir");
        fs::write(
            dir.path().join("components/launch/trpc-launch-health.tsx"),
            "export const trpcHealth = 'present';\n",
        )
        .expect("write trpc launch health");
        let type_safe_api_receipt_path = dir
            .path()
            .join(".dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json");
        fs::write(
            &type_safe_api_receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.package_dashboard_workflow_receipt",
                "package_id": "api/trpc",
                "official_dx_package_name": "Type-Safe API",
                "upstream_package": "@trpc/server",
                "upstream_version": "11.17.0",
                "honesty_label": "ADAPTER-BOUNDARY",
                "dx_check_visibility": {
                    "schema": "dx.forge.package.dx_check_visibility",
                    "current_status": "present",
                    "status_legend": [
                        { "status": "present" },
                        { "status": "stale" },
                        { "status": "missing-receipt" },
                        { "status": "blocked" },
                        { "status": "unsupported-surface" }
                    ]
                }
            }))
            .expect("type-safe api receipt json"),
        )
        .expect("write type-safe api receipt");

        fs::write(
            dir.path().join(".dx/forge/package-status.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": "Type-Safe API",
                        "package_id": "api/trpc",
                        "upstream_package": "@trpc/server",
                        "upstream_version": "11.17.0",
                        "source_mirror": "G:/WWW/inspirations/trpc",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": ".dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json",
                        "status_vocabulary": [
                            "present",
                            "stale",
                            "missing-receipt",
                            "blocked",
                            "unsupported-surface"
                        ],
                        "selected_surfaces": [
                            {
                                "surface_id": "trpc-launch-dashboard-workflow",
                                "status": "present",
                                "receipt_path": ".dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json",
                                "files": ["components/launch/trpc-launch-health.tsx"],
                                "source_markers": [
                                    "data-dx-package=\"api/trpc\"",
                                    "data-dx-trpc-action=\"check-health\""
                                ],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "components/launch/trpc-launch-health.tsx": "0000000000000000000000000000000000000000000000000000000000000000"
                                }
                            },
                            {
                                "surface_id": "trpc-starter-dashboard-workflow",
                                "status": "blocked",
                                "receipt_path": ".dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json",
                                "files": ["components/dashboard/trpc-dashboard-workflow.tsx"],
                                "source_markers": ["data-dx-component=\"dashboard-trpc-workflow\""]
                            },
                            {
                                "surface_id": "trpc-route-handler",
                                "status": "unsupported-surface",
                                "receipt_path": ".dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json",
                                "files": ["app/api/trpc/[trpc]/route.ts"],
                                "source_markers": ["fetchRequestHandler"]
                            }
                        ],
                        "blocked_surfaces": [
                            "production authorization and session context"
                        ],
                        "unsupported_surfaces": [
                            "legacy pages router helper"
                        ],
                        "dx_check_metrics": [
                            "type_safe_api_receipt_present",
                            "type_safe_api_receipt_stale",
                            "type_safe_api_missing_receipt",
                            "type_safe_api_blocked_surface",
                            "type_safe_api_unsupported_surface",
                            "type_safe_api_hash_manifest_present",
                            "type_safe_api_hash_mismatch"
                        ]
                    }
                ]
            }))
            .expect("package status json"),
        )
        .expect("write package status");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(forge, "type_safe_api_package_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(forge, "type_safe_api_receipt_present"),
            Some(1)
        );
        assert_eq!(metric_value(forge, "type_safe_api_receipt_stale"), Some(1));
        assert_eq!(
            metric_value(forge, "type_safe_api_missing_receipt"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "type_safe_api_blocked_surface"),
            Some(2)
        );
        assert_eq!(
            metric_value(forge, "type_safe_api_unsupported_surface"),
            Some(2)
        );
        assert_eq!(
            metric_value(forge, "type_safe_api_hash_manifest_present"),
            Some(1)
        );
        assert_eq!(metric_value(forge, "type_safe_api_hash_mismatch"), Some(1));
        assert!(
            forge
                .findings
                .iter()
                .any(|finding| finding.code == "type-safe-api-stale-receipt")
        );
        assert!(
            forge
                .findings
                .iter()
                .any(|finding| finding.code == "type-safe-api-blocked-surface")
        );
        assert!(
            forge
                .findings
                .iter()
                .any(|finding| finding.code == "type-safe-api-unsupported-surface")
        );
        assert!(
            forge
                .findings
                .iter()
                .any(|finding| finding.code == "type-safe-api-hash-mismatch")
        );

        fs::remove_file(type_safe_api_receipt_path).expect("remove type-safe api receipt");

        let missing_report = check_dx_project(dir.path()).expect("check missing receipt");
        let missing_forge = missing_report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(missing_forge, "type_safe_api_receipt_present"),
            Some(0)
        );
        assert_eq!(
            metric_value(missing_forge, "type_safe_api_missing_receipt"),
            Some(1)
        );
        assert!(
            missing_forge
                .findings
                .iter()
                .any(|finding| finding.code == "type-safe-api-missing-receipt")
        );
    }

    #[test]
    fn dx_check_reports_ai_sdk_package_status_visibility() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("ai/vercel-ai", dir.path()).expect("forge add");

        fs::create_dir_all(dir.path().join(".dx/forge/receipts")).expect("dashboard receipt dir");
        fs::create_dir_all(dir.path().join("lib/ai")).expect("ai lib dir");
        fs::create_dir_all(dir.path().join("components/launch")).expect("launch components dir");
        let chat_route_source =
            "import { streamText } from \"ai\";\nexport const marker = streamText;\n";
        let chat_route_hash = format!("{:x}", Sha256::digest(chat_route_source.as_bytes()));
        fs::write(dir.path().join("lib/ai/chat-route.ts"), chat_route_source)
            .expect("write chat route");
        let launch_assistant_source = "export const marker = 'data-dx-package=\"ai/vercel-ai\"';\n";
        let launch_assistant_hash =
            format!("{:x}", Sha256::digest(launch_assistant_source.as_bytes()));
        fs::write(
            dir.path().join("components/launch/ai-chat-status.tsx"),
            launch_assistant_source,
        )
        .expect("write launch assistant");

        let ai_sdk_receipt_path = dir
            .path()
            .join(".dx/forge/receipts/2026-05-22-ai-vercel-ai-launch-assistant.json");
        fs::write(
            &ai_sdk_receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.package_dashboard_workflow_receipt",
                "package_id": "ai/vercel-ai",
                "official_dx_package_name": "AI SDK",
                "upstream_package": "ai",
                "upstream_version": "7.0.0-canary.146",
                "honesty_label": "ADAPTER-BOUNDARY",
                "dx_check_visibility": {
                    "schema": "dx.forge.package.dx_check_visibility",
                    "current_status": "present",
                    "status_legend": [
                        { "status": "present" },
                        { "status": "stale" },
                        { "status": "missing-receipt" },
                        { "status": "blocked" },
                        { "status": "unsupported-surface" }
                    ]
                }
            }))
            .expect("ai sdk receipt json"),
        )
        .expect("write ai sdk receipt");

        fs::write(
            dir.path().join(".dx/forge/package-status.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": "AI SDK",
                        "package_id": "ai/vercel-ai",
                        "upstream_package": "ai",
                        "upstream_version": "7.0.0-canary.146",
                        "source_mirror": "G:/WWW/inspirations/vercel-ai",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": ".dx/forge/receipts/2026-05-22-ai-vercel-ai-launch-assistant.json",
                        "status_vocabulary": [
                            "present",
                            "stale",
                            "missing-receipt",
                            "blocked",
                            "unsupported-surface"
                        ],
                        "selected_surfaces": [
                            {
                                "surface_id": "ai-chat-route",
                                "status": "present",
                                "receipt_path": ".dx/forge/receipts/2026-05-22-ai-vercel-ai-launch-assistant.json",
                                "files": ["lib/ai/chat-route.ts"],
                                "source_markers": [
                                    "streamText",
                                    "convertToModelMessages",
                                    "toUIMessageStreamResponse"
                                ],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "lib/ai/chat-route.ts": chat_route_hash,
                                    "upstream:packages/ai/src/generate-text/stream-text.ts": "upstream-fixture-hash"
                                }
                            },
                            {
                                "surface_id": "ai-dashboard-assistant-component",
                                "status": "blocked",
                                "receipt_path": ".dx/forge/receipts/2026-05-22-ai-vercel-ai-launch-assistant.json",
                                "files": ["components/ai/ai-launch-assistant.tsx"],
                                "source_markers": [
                                    "DefaultChatTransport",
                                    "createProviderRegistry"
                                ],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "components/ai/ai-launch-assistant.tsx": "missing-fixture-hash"
                                }
                            },
                            {
                                "surface_id": "ai-launch-assistant-dashboard-workflow",
                                "status": "unsupported-surface",
                                "receipt_path": ".dx/forge/receipts/2026-05-22-ai-vercel-ai-launch-assistant.json",
                                "files": ["components/launch/ai-chat-status.tsx"],
                                "source_markers": [
                                    "data-dx-package=\"ai/vercel-ai\"",
                                    "data-dx-ai-route-contract=\"/api/ai/chat\""
                                ],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "components/launch/ai-chat-status.tsx": launch_assistant_hash
                                }
                            }
                        ],
                        "blocked_surfaces": [
                            "credential-backed model streaming"
                        ],
                        "unsupported_surfaces": [
                            "unselected media generation surfaces"
                        ],
                        "dx_check_metrics": [
                            "ai_sdk_receipt_present",
                            "ai_sdk_receipt_stale",
                            "ai_sdk_missing_receipt",
                            "ai_sdk_blocked_surface",
                            "ai_sdk_unsupported_surface",
                            "ai_sdk_hash_manifest_present",
                            "ai_sdk_hash_mismatch"
                        ]
                    }
                ]
            }))
            .expect("package status json"),
        )
        .expect("write package status");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(metric_value(forge, "ai_sdk_package_present"), Some(1));
        assert_eq!(metric_value(forge, "ai_sdk_receipt_present"), Some(1));
        assert_eq!(metric_value(forge, "ai_sdk_receipt_stale"), Some(1));
        assert_eq!(metric_value(forge, "ai_sdk_missing_receipt"), Some(0));
        assert_eq!(metric_value(forge, "ai_sdk_blocked_surface"), Some(2));
        assert_eq!(metric_value(forge, "ai_sdk_unsupported_surface"), Some(2));
        assert_eq!(metric_value(forge, "ai_sdk_hash_manifest_present"), Some(1));
        assert_eq!(metric_value(forge, "ai_sdk_hash_mismatch"), Some(1));
        assert!(
            forge
                .findings
                .iter()
                .any(|finding| finding.code == "ai-sdk-stale-receipt")
        );
        assert!(
            forge
                .findings
                .iter()
                .any(|finding| finding.code == "ai-sdk-blocked-surface")
        );
        assert!(
            forge
                .findings
                .iter()
                .any(|finding| finding.code == "ai-sdk-unsupported-surface")
        );
        assert!(
            forge
                .findings
                .iter()
                .any(|finding| finding.code == "ai-sdk-hash-mismatch")
        );

        fs::remove_file(ai_sdk_receipt_path).expect("remove ai sdk receipt");

        let missing_report = check_dx_project(dir.path()).expect("check missing receipt");
        let missing_forge = missing_report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(missing_forge, "ai_sdk_receipt_present"),
            Some(0)
        );
        assert_eq!(
            metric_value(missing_forge, "ai_sdk_missing_receipt"),
            Some(1)
        );
        assert!(
            missing_forge
                .findings
                .iter()
                .any(|finding| finding.code == "ai-sdk-missing-receipt")
        );
    }

    #[test]
    fn dx_check_reports_realtime_app_database_package_status_visibility() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("instantdb/react", dir.path()).expect("forge add");

        fs::create_dir_all(dir.path().join("components/launch")).expect("launch component dir");
        let realtime_status_source = r#"export function InstantDbStatus() { return <section data-dx-package="instantdb/react" data-dx-style-surface="realtime-app-database" />; }"#;
        let realtime_status_hash =
            format!("{:x}", Sha256::digest(realtime_status_source.as_bytes()));
        fs::write(
            dir.path().join("components/launch/instantdb-status.tsx"),
            realtime_status_source,
        )
        .expect("write instantdb status component");
        fs::create_dir_all(dir.path().join(".dx/forge/receipts")).expect("dashboard receipt dir");
        let realtime_receipt_path = dir
            .path()
            .join(".dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json");
        fs::write(
            &realtime_receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.package_dashboard_workflow_receipt",
                "package_id": "instantdb/react",
                "package_name": "Realtime App Database",
                "upstream_package": "@instantdb/react",
                "upstream_version": "0.0.0",
                "honesty_label": "ADAPTER-BOUNDARY",
                "dx_check_visibility": {
                    "schema": "dx.forge.package.dx_check_visibility",
                    "current_status": "present",
                    "receipt_status": "present",
                    "status_legend": [
                        { "status": "present" },
                        { "status": "stale" },
                        { "status": "missing-receipt" },
                        { "status": "blocked" },
                        { "status": "unsupported-surface" }
                    ]
                }
            }))
            .expect("realtime receipt json"),
        )
        .expect("write realtime receipt");

        fs::write(
            dir.path().join(".dx/forge/package-status.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": "Realtime App Database",
                        "package_id": "instantdb/react",
                        "upstream_package": "@instantdb/react",
                        "upstream_version": "0.0.0",
                        "source_mirror": "G:/WWW/inspirations/instantdb",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json",
                        "status_vocabulary": [
                            "present",
                            "stale",
                            "missing-receipt",
                            "blocked",
                            "unsupported-surface"
                        ],
                        "selected_surfaces": [
                            {
                                "surface_id": "instantdb-runtime-dashboard-workflow",
                                "status": "present",
                                "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json",
                                "files": ["components/launch/instantdb-status.tsx"],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "components/launch/instantdb-status.tsx": realtime_status_hash
                                },
                                "source_markers": [
                                    "data-dx-component=\"instantdb-runtime-dashboard-workflow\"",
                                    "data-dx-package=\"instantdb/react\"",
                                    "data-dx-style-surface=\"realtime-app-database\""
                                ]
                            },
                            {
                                "surface_id": "sync-table-events",
                                "status": "present",
                                "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json",
                                "files": ["lib/instant/dashboard-workflow.ts"],
                                "source_markers": [
                                    "SyncTableCallbackEventType",
                                    "db.core._syncTableExperimental"
                                ]
                            }
                        ],
                        "blocked_surfaces": [],
                        "unsupported_surfaces": [],
                        "dx_check_metrics": [
                            "realtime_app_database_receipt_present",
                            "realtime_app_database_receipt_stale",
                            "realtime_app_database_missing_receipt",
                            "realtime_app_database_blocked_surface",
                            "realtime_app_database_unsupported_surface",
                            "realtime_app_database_hash_manifest_present",
                            "realtime_app_database_hash_mismatch",
                            "realtime_app_database_dx_style_compatibility_present",
                            "realtime_app_database_dx_style_compatibility_missing"
                        ],
                        "dx_style_compatibility": {
                            "schema": "dx.forge.package.dx_style_compatibility",
                            "status": "present",
                            "token_source": "tools/launch/runtime-template/assets/launch-runtime.css",
                            "generated_css": "tools/launch/runtime-template/assets/launch-runtime.css",
                            "visible_surfaces": [
                                "instantdb-runtime-dashboard-workflow"
                            ],
                            "source_files": [
                                "components/launch/instantdb-status.tsx"
                            ],
                            "data_dx_markers": [
                                "data-dx-style-surface=\"realtime-app-database\""
                            ],
                            "runtime_proof": false,
                            "runtime_limitations": [
                                "SOURCE-ONLY: Realtime App Database style evidence is source-visible; no live browser style proof was run."
                            ]
                        }
                    }
                ]
            }))
            .expect("package status json"),
        )
        .expect("write package status");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(forge, "realtime_app_database_package_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(forge, "realtime_app_database_receipt_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(forge, "realtime_app_database_receipt_stale"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "realtime_app_database_missing_receipt"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "realtime_app_database_blocked_surface"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "realtime_app_database_unsupported_surface"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "realtime_app_database_hash_manifest_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(forge, "realtime_app_database_hash_mismatch"),
            Some(0)
        );
        assert_eq!(
            metric_value(
                forge,
                "realtime_app_database_dx_style_compatibility_present"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(
                forge,
                "realtime_app_database_dx_style_compatibility_missing"
            ),
            Some(0)
        );
        assert!(
            !forge
                .findings
                .iter()
                .any(|finding| finding.code.starts_with("realtime-app-database-"))
        );

        fs::remove_file(realtime_receipt_path).expect("remove realtime receipt");

        let missing_report = check_dx_project(dir.path()).expect("check missing receipt");
        let missing_forge = missing_report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(missing_forge, "realtime_app_database_receipt_present"),
            Some(0)
        );
        assert_eq!(
            metric_value(missing_forge, "realtime_app_database_missing_receipt"),
            Some(1)
        );
        assert!(
            missing_forge
                .findings
                .iter()
                .any(|finding| { finding.code == "realtime-app-database-missing-receipt" })
        );
    }

    #[test]
    fn realtime_app_database_hash_mismatch_flips_when_selected_file_changes() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("instantdb/react", dir.path()).expect("forge add");

        let selected_path = "components/launch/instantdb-status.tsx";
        let selected_source = r#"export function InstantDbStatus() { return <section data-dx-package="instantdb/react" />; }"#;
        fs::create_dir_all(dir.path().join("components/launch")).expect("launch component dir");
        fs::write(dir.path().join(selected_path), selected_source)
            .expect("write realtime selected source");
        let selected_hash = format!("{:x}", Sha256::digest(selected_source.as_bytes()));

        let receipt_path = ".dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json";
        fs::create_dir_all(dir.path().join(".dx/forge/receipts")).expect("receipt dir");
        fs::write(
            dir.path().join(receipt_path),
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.package_dashboard_workflow_receipt",
                "package_id": "instantdb/react",
                "package_name": "Realtime App Database",
                "upstream_package": "@instantdb/react",
                "upstream_version": "0.0.0",
                "hash_algorithm": "sha256",
                "file_hashes": {
                    selected_path: selected_hash
                }
            }))
            .expect("realtime receipt json"),
        )
        .expect("write realtime receipt");

        fs::write(
            dir.path().join(".dx/forge/package-status.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": "Realtime App Database",
                        "package_id": "instantdb/react",
                        "upstream_package": "@instantdb/react",
                        "upstream_version": "0.0.0",
                        "source_mirror": "G:/WWW/inspirations/instantdb",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": receipt_path,
                        "status_vocabulary": [
                            "present",
                            "stale",
                            "missing-receipt",
                            "blocked",
                            "unsupported-surface"
                        ],
                        "selected_surfaces": [
                            {
                                "surface_id": "instantdb-runtime-dashboard-workflow",
                                "status": "present",
                                "receipt_path": receipt_path,
                                "files": [selected_path],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    selected_path: selected_hash
                                },
                                "source_markers": [
                                    "data-dx-component=\"instantdb-runtime-dashboard-workflow\"",
                                    "data-dx-package=\"instantdb/react\""
                                ]
                            }
                        ],
                        "blocked_surfaces": [],
                        "unsupported_surfaces": []
                    }
                ]
            }))
            .expect("package status json"),
        )
        .expect("write package status");

        let fresh_report = check_dx_project(dir.path()).expect("fresh check");
        let fresh_forge = fresh_report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");
        assert_eq!(
            metric_value(fresh_forge, "realtime_app_database_hash_manifest_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(fresh_forge, "realtime_app_database_hash_mismatch"),
            Some(0)
        );
        assert_eq!(
            metric_value(fresh_forge, "realtime_app_database_receipt_stale"),
            Some(0)
        );

        fs::write(
            dir.path().join(selected_path),
            r#"export function InstantDbStatus() { return <section data-dx-package="instantdb/react" data-state="changed" />; }"#,
        )
        .expect("mutate realtime selected source");

        let stale_report = check_dx_project(dir.path()).expect("stale check");
        let stale_forge = stale_report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");
        assert_eq!(
            metric_value(stale_forge, "realtime_app_database_hash_manifest_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(stale_forge, "realtime_app_database_hash_mismatch"),
            Some(1)
        );
        assert_eq!(
            metric_value(stale_forge, "realtime_app_database_receipt_stale"),
            Some(1)
        );
        assert!(
            stale_forge
                .findings
                .iter()
                .any(|finding| { finding.code == "realtime-app-database-hash-mismatch" })
        );
        assert!(
            stale_forge
                .findings
                .iter()
                .any(|finding| { finding.code == "realtime-app-database-stale-receipt" })
        );
    }

    #[test]
    fn dx_check_reports_automation_connectors_package_status_visibility() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("automations/n8n", dir.path()).expect("forge add");

        let automations_source = "export const automationStatus = 'ready';\n";
        fs::write(
            dir.path().join("automations-status.tsx"),
            automations_source,
        )
        .expect("automation status source");
        let automations_source_hash =
            format!("{:x}", Sha256::digest(automations_source.as_bytes()));

        fs::create_dir_all(dir.path().join(".dx/forge/receipts")).expect("dashboard receipt dir");
        let automations_receipt_path = dir
            .path()
            .join(".dx/forge/receipts/2026-05-22-automation-connectors-launch-workflow.json");
        fs::write(
            &automations_receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.package_dashboard_workflow_receipt",
                "package_id": "automations/n8n",
                "official_package_name": "Automation Connectors",
                "upstream_package": "n8n-nodes-base",
                "upstream_version": "2.22.0",
                "hash_algorithm": "sha256",
                "file_hashes": {
                    "examples/template/automations-status.tsx": automations_source_hash
                },
                "dx_check_visibility": {
                    "schema": "dx.forge.package.dx_check_visibility",
                    "current_status": "present",
                    "status_legend": [
                        { "status": "present" },
                        { "status": "stale" },
                        { "status": "missing-receipt" },
                        { "status": "blocked" },
                        { "status": "unsupported-surface" }
                    ]
                }
            }))
            .expect("automation receipt json"),
        )
        .expect("write automation receipt");

        fs::write(
            dir.path().join(".dx/forge/package-status.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": "Automation Connectors",
                        "package_id": "automations/n8n",
                        "upstream_package": "n8n-nodes-base",
                        "upstream_version": "2.22.0",
                        "source_mirror": "G:/WWW/inspirations/n8n/packages/nodes-base",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": ".dx/forge/receipts/2026-05-22-automation-connectors-launch-workflow.json",
                        "status_vocabulary": [
                            "present",
                            "stale",
                            "missing-receipt",
                            "blocked",
                            "unsupported-surface"
                        ],
                        "receipt_hash_refresh": {
                            "schema": "dx.forge.package.receipt_hash_refresh",
                            "status": "current",
                            "helper_path": "examples/template/automation-connectors-receipt-hashes.ts",
                            "check_command": "node examples/template/automation-connectors-receipt-hashes.ts --check",
                            "write_command": "node examples/template/automation-connectors-receipt-hashes.ts --write",
                            "json_check_command": "node examples/template/automation-connectors-receipt-hashes.ts --check --json",
                            "source_guard_runbook_fixture": "docs/packages/automation-connectors.source-guard-runbook.json",
                            "preview_manifest_materializer": "tools/launch/materialize-www-template.ts",
                            "studio_manifest_source": "dx-www/src/cli/studio_manifest.rs",
                            "receipt_path": ".dx/forge/receipts/2026-05-22-automation-connectors-launch-workflow.json",
                            "hash_algorithm": "sha256",
                            "tracked_file_count": 13,
                            "tracked_files": [
                                "examples/template/automations-status.tsx",
                                "tools/launch/materialize-www-template.ts",
                                "dx-www/src/cli/studio_manifest.rs"
                            ],
                            "current_files": [
                                "examples/template/automations-status.tsx",
                                "tools/launch/materialize-www-template.ts",
                                "dx-www/src/cli/studio_manifest.rs"
                            ],
                            "stale_files": [],
                            "missing_files": [],
                            "stale_mirror_files": [],
                            "missing_mirror_files": [],
                            "stale_file_count": 0,
                            "missing_file_count": 0,
                            "runtime_execution": false,
                            "secret_access": false,
                            "zed_visibility": "automation-connectors:receipt-hash-refresh"
                        },
                        "selected_surfaces": [
                            {
                                "surface_id": "automation-launch-dashboard-workflow",
                                "status": "present",
                                "receipt_path": ".dx/forge/receipts/2026-05-22-automation-connectors-launch-workflow.json",
                                "files": ["components/launch/automations-status.tsx"],
                                "source_markers": [
                                    "data-dx-component=\"launch-automation-dashboard-workflow\"",
                                    "data-dx-dashboard-workflow=\"automation-release-receipt\""
                                ],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "examples/template/automations-status.tsx": automations_source_hash
                                }
                            },
                            {
                                "surface_id": "automation-zed-run-handoff",
                                "status": "present",
                                "receipt_path": ".dx/forge/receipts/2026-05-22-automation-connectors-launch-workflow.json",
                                "files": ["components/launch/automations-status.tsx"],
                                "source_markers": [
                                    "data-dx-automation-safe-action=\"prepare-zed-run-handoff\"",
                                    "data-dx-automation-handoff=\"zed-run-receipt\""
                                ]
                            }
                        ],
                        "blocked_surfaces": [],
                        "unsupported_surfaces": [],
                        "dx_style_compatibility": {
                            "schema": "dx.forge.package.dx_style_compatibility",
                            "status": "present",
                            "token_source": "styles/theme.css",
                            "generated_css": "styles/generated.css",
                            "visible_surfaces": ["automation-connectors"],
                            "source_files": ["examples/template/automations-status.tsx"],
                            "receipt_path": ".dx/forge/receipts/2026-05-22-automation-connectors-launch-workflow.json"
                        },
                        "inspected_upstream_files": [
                            "packages/nodes-base/package.json",
                            "packages/nodes-base/nodes/ManualTrigger/ManualTrigger.node.ts",
                            "packages/nodes-base/nodes/Slack/Slack.node.ts",
                            "packages/nodes-base/nodes/Slack/V2/SlackV2.node.ts",
                            "packages/nodes-base/nodes/Webhook/Webhook.node.ts",
                            "packages/nodes-base/nodes/Notion/Notion.node.ts",
                            "packages/nodes-base/credentials/SlackApi.credentials.ts",
                            "packages/nodes-base/credentials/SlackOAuth2Api.credentials.ts",
                            "packages/nodes-base/credentials/NotionApi.credentials.ts"
                        ],
                        "upstream_public_apis": [
                            "VersionedNodeType",
                            "INodeType",
                            "INodeTypeDescription",
                            "ITriggerFunctions",
                            "IExecuteFunctions",
                            "IWebhookFunctions",
                            "ICredentialType",
                            "IAuthenticateGeneric",
                            "ICredentialTestRequest"
                        ],
                        "dx_check_metrics": [
                            "automation_connectors_receipt_present",
                            "automation_connectors_receipt_stale",
                            "automation_connectors_missing_receipt",
                            "automation_connectors_blocked_surface",
                            "automation_connectors_unsupported_surface",
                            "automation_connectors_hash_manifest_present",
                            "automation_connectors_hash_mismatch",
                            "automation_connectors_dx_style_compatibility_present",
                            "automation_connectors_dx_style_compatibility_missing",
                            "automation_connectors_upstream_runtime_boundary_present",
                            "automation_connectors_upstream_runtime_boundary_missing",
                            "automation_connectors_receipt_hash_refresh_current",
                            "automation_connectors_receipt_hash_refresh_stale",
                            "automation_connectors_receipt_hash_refresh_missing"
                        ]
                    }
                ]
            }))
            .expect("package status json"),
        )
        .expect("write package status");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(forge, "automation_connectors_package_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(forge, "automation_connectors_receipt_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(forge, "automation_connectors_receipt_stale"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "automation_connectors_missing_receipt"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "automation_connectors_blocked_surface"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "automation_connectors_unsupported_surface"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "automation_connectors_hash_manifest_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(forge, "automation_connectors_hash_mismatch"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "automation_connectors_receipt_hash_refresh_current"),
            Some(1)
        );
        assert_eq!(
            metric_value(forge, "automation_connectors_receipt_hash_refresh_stale"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "automation_connectors_receipt_hash_refresh_missing"),
            Some(0)
        );
        assert!(
            !forge
                .findings
                .iter()
                .any(|finding| finding.code.starts_with("automation-connectors-"))
        );

        fs::write(
            dir.path().join("automations-status.tsx"),
            "export const automationStatus = 'changed';\n",
        )
        .expect("stale automation status source");

        let stale_report = check_dx_project(dir.path()).expect("check stale hash");
        let stale_forge = stale_report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(stale_forge, "automation_connectors_receipt_stale"),
            Some(1)
        );
        assert_eq!(
            metric_value(stale_forge, "automation_connectors_hash_mismatch"),
            Some(1)
        );
        assert!(
            stale_forge
                .findings
                .iter()
                .any(|finding| finding.code == "automation-connectors-hash-mismatch")
        );

        fs::remove_file(automations_receipt_path).expect("remove automation receipt");

        let missing_report = check_dx_project(dir.path()).expect("check missing receipt");
        let missing_forge = missing_report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(missing_forge, "automation_connectors_receipt_present"),
            Some(0)
        );
        assert_eq!(
            metric_value(missing_forge, "automation_connectors_missing_receipt"),
            Some(1)
        );
        assert!(
            missing_forge
                .findings
                .iter()
                .any(|finding| finding.code == "automation-connectors-missing-receipt")
        );
    }

    #[test]
    fn dx_check_reports_ui_components_package_status_visibility() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("shadcn/ui/button", dir.path()).expect("forge add");

        let button_source = "export function Button() {}\nexport function buttonVariants() {}\nexport const Slot = \"Root\";\n";
        let button_hash = format!("{:x}", Sha256::digest(button_source.as_bytes()));
        fs::create_dir_all(dir.path().join("components/ui")).expect("ui components dir");
        fs::write(dir.path().join("components/ui/button.tsx"), button_source)
            .expect("write button source");

        let dashboard_controls_source =
            "export const marker = 'data-dx-component=\"shadcn-dashboard-controls\"';\n";
        let dashboard_controls_hash =
            format!("{:x}", Sha256::digest(dashboard_controls_source.as_bytes()));
        fs::create_dir_all(dir.path().join("components/launch")).expect("launch components dir");
        fs::write(
            dir.path()
                .join("components/launch/shadcn-dashboard-controls.tsx"),
            dashboard_controls_source,
        )
        .expect("write dashboard controls source");

        fs::create_dir_all(dir.path().join(".dx/forge/receipts")).expect("dashboard receipt dir");
        let ui_components_receipt_path = dir
            .path()
            .join(".dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json");
        fs::write(
            &ui_components_receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.package_dashboard_workflow_receipt",
                "package_id": "shadcn/ui/button",
                "official_package_name": "UI Components",
                "upstream_package": "shadcn-ui",
                "upstream_version": "0.0.1",
                "based_on": "shadcn-ui v4 registry plus Radix Primitives",
                "hash_algorithm": "sha256",
                "file_hashes": {
                    "components/ui/button.tsx": &button_hash,
                    "components/launch/shadcn-dashboard-controls.tsx": &dashboard_controls_hash
                },
                "dx_check_visibility": {
                    "schema": "dx.forge.package.dx_check_visibility",
                    "current_status": "present",
                    "status_legend": [
                        { "status": "present" },
                        { "status": "stale" },
                        { "status": "missing-receipt" },
                        { "status": "blocked" },
                        { "status": "unsupported-surface" }
                    ]
                },
                "runtime_limitations": [
                    "SOURCE-ONLY: browser UI runtime proof deferred"
                ]
            }))
            .expect("ui components receipt json"),
        )
        .expect("write ui components receipt");

        fs::write(
            dir.path().join(".dx/forge/package-status.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": "UI Components",
                        "package_id": "shadcn/ui/button",
                        "upstream_package": "shadcn-ui",
                        "upstream_version": "0.0.1",
                        "source_mirror": "G:/WWW/inspirations/shadcn-ui; G:/WWW/inspirations/radix-primitives",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json",
                        "receipt_hash_refresh": {
                            "schema": "dx.forge.package.receipt_hash_refresh",
                            "status": "current",
                            "stale_file_count": 0,
                            "missing_file_count": 0
                        },
                        "status_vocabulary": [
                            "present",
                            "stale",
                            "missing-receipt",
                            "blocked",
                            "unsupported-surface"
                        ],
                        "selected_surfaces": [
                            {
                                "surface_id": "ui-components-source-primitives",
                                "status": "present",
                                "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json",
                                "files": ["components/ui/button.tsx"],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "components/ui/button.tsx": &button_hash
                                },
                                "source_markers": [
                                    "Button",
                                    "buttonVariants",
                                    "Slot.Root"
                                ]
                            },
                            {
                                "surface_id": "ui-components-dashboard-controls",
                                "status": "present",
                                "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json",
                                "files": ["components/launch/shadcn-dashboard-controls.tsx"],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "components/launch/shadcn-dashboard-controls.tsx": &dashboard_controls_hash
                                },
                                "source_markers": [
                                    "data-dx-component=\"shadcn-dashboard-controls\"",
                                    "data-dx-package=\"shadcn/ui/button\""
                                ]
                            }
                        ],
                        "blocked_surfaces": [],
                        "unsupported_surfaces": [],
                        "dx_check_metrics": [
                            "ui_components_receipt_present",
                            "ui_components_receipt_stale",
                            "ui_components_missing_receipt",
                            "ui_components_blocked_surface",
                            "ui_components_unsupported_surface",
                            "ui_components_hash_manifest_present",
                            "ui_components_hash_mismatch"
                        ]
                    }
                ]
            }))
            .expect("package status json"),
        )
        .expect("write package status");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(forge, "ui_components_package_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(forge, "ui_components_receipt_present"),
            Some(1)
        );
        assert_eq!(metric_value(forge, "ui_components_receipt_stale"), Some(0));
        assert_eq!(
            metric_value(forge, "ui_components_missing_receipt"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "ui_components_blocked_surface"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "ui_components_unsupported_surface"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "ui_components_hash_manifest_present"),
            Some(1)
        );
        assert_eq!(metric_value(forge, "ui_components_hash_mismatch"), Some(0));
        assert_eq!(
            metric_value(forge, "ui_components_receipt_hash_refresh_current"),
            Some(1)
        );
        assert_eq!(
            metric_value(forge, "ui_components_receipt_hash_refresh_stale"),
            Some(0)
        );
        assert_eq!(
            metric_value(forge, "ui_components_receipt_hash_refresh_missing"),
            Some(0)
        );
        assert!(
            !forge
                .findings
                .iter()
                .any(|finding| { finding.code.starts_with("ui-components-") })
        );

        fs::write(
            dir.path()
                .join("components/launch/shadcn-dashboard-controls.tsx"),
            "export const marker = 'changed-dashboard-controls';\n",
        )
        .expect("mutate dashboard controls source");

        let stale_report = check_dx_project(dir.path()).expect("check stale receipt");
        let stale_forge = stale_report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");
        assert_eq!(
            metric_value(stale_forge, "ui_components_receipt_stale"),
            Some(1)
        );
        assert_eq!(
            metric_value(stale_forge, "ui_components_hash_mismatch"),
            Some(1)
        );
        assert!(
            stale_forge
                .findings
                .iter()
                .any(|finding| { finding.code == "ui-components-hash-mismatch" })
        );

        fs::remove_file(ui_components_receipt_path).expect("remove ui components receipt");

        let missing_report = check_dx_project(dir.path()).expect("check missing receipt");
        let missing_forge = missing_report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(missing_forge, "ui_components_receipt_present"),
            Some(0)
        );
        assert_eq!(
            metric_value(missing_forge, "ui_components_missing_receipt"),
            Some(1)
        );
        assert!(
            missing_forge
                .findings
                .iter()
                .any(|finding| { finding.code == "ui-components-missing-receipt" })
        );
    }

    #[test]
    fn dx_check_reports_supabase_env_secret_leaks() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("supabase/client", dir.path()).expect("forge add");
        fs::write(
            dir.path().join("lib/supabase/.env.example"),
            "\
NEXT_PUBLIC_SUPABASE_URL=https://example.supabase.co
NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY=publishable-key
SUPABASE_SERVICE_ROLE_KEY=must-not-live-in-client-template
",
        )
        .expect("unsafe env");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(metric_value(forge, "supabase_client_secret_leaks"), Some(1));
        assert!(
            forge
                .findings
                .iter()
                .any(|finding| finding.code == "supabase-client-secret-env-leak")
        );
    }

    #[test]
    fn dx_check_reports_missing_auth_google_env_example_and_stale_receipt() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("auth/better-auth", dir.path()).expect("forge add");
        fs::remove_file(dir.path().join("auth/better-auth/.env.example")).expect("remove env");

        let manifest_path = dir.path().join(SOURCE_MANIFEST_PATH);
        let mut manifest: DxSourceManifest =
            serde_json::from_slice(&fs::read(&manifest_path).expect("manifest bytes"))
                .expect("manifest json");
        manifest.packages[0].version = "0.0.0".to_string();
        fs::write(
            &manifest_path,
            serde_json::to_vec_pretty(&manifest).expect("manifest json"),
        )
        .expect("write manifest");

        let report = check_dx_project(dir.path()).expect("check");
        let forge = report
            .sections
            .iter()
            .find(|section| section.name == "forge")
            .expect("forge section");

        assert_eq!(
            metric_value(forge, "auth_google_missing_env_examples"),
            Some(1)
        );
        assert_eq!(metric_value(forge, "auth_google_stale_receipts"), Some(1));
        assert!(
            forge
                .findings
                .iter()
                .any(|finding| finding.code == "auth-google-env-example-missing")
        );
        assert!(
            forge
                .findings
                .iter()
                .any(|finding| finding.code == "auth-google-stale-receipt")
        );
    }

    #[test]
    fn forge_launch_gate_passes_for_clean_source_owned_package() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("shadcn/ui/button", dir.path()).expect("forge add");

        let report = check_dx_project(dir.path()).expect("check");

        assert!(forge_launch_gate_findings(&report).is_empty());
    }

    #[test]
    fn forge_launch_gate_fails_for_stale_receipts() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("shadcn/ui/button", dir.path()).expect("forge add");

        let manifest_path = dir.path().join(SOURCE_MANIFEST_PATH);
        let mut manifest: DxSourceManifest =
            serde_json::from_slice(&fs::read(&manifest_path).expect("manifest bytes"))
                .expect("manifest json");
        manifest.packages[0].version = "0.0.0".to_string();
        fs::write(
            &manifest_path,
            serde_json::to_vec_pretty(&manifest).expect("manifest json"),
        )
        .expect("write manifest");

        let report = check_dx_project(dir.path()).expect("check");
        let findings = forge_launch_gate_findings(&report);

        assert!(
            findings
                .iter()
                .any(|finding| { finding.code == "forge-launch-gate-stale-receipts" })
        );
    }

    #[test]
    fn forge_launch_gate_fails_for_missing_rollback_coverage() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("shadcn/ui/button", dir.path()).expect("forge add");

        let manifest_path = dir.path().join(SOURCE_MANIFEST_PATH);
        let mut manifest: DxSourceManifest =
            serde_json::from_slice(&fs::read(&manifest_path).expect("manifest bytes"))
                .expect("manifest json");
        manifest.packages[0].last_accepted_update = Some(Utc::now().to_rfc3339());
        manifest.packages[0].rollback_receipt = None;
        fs::write(
            &manifest_path,
            serde_json::to_vec_pretty(&manifest).expect("manifest json"),
        )
        .expect("write manifest");

        let report = check_dx_project(dir.path()).expect("check");
        let findings = forge_launch_gate_findings(&report);

        assert!(
            findings
                .iter()
                .any(|finding| { finding.code == "forge-launch-gate-missing-rollback" })
        );
    }

    #[test]
    fn forge_launch_gate_fails_for_red_package_traffic() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("pages")).expect("pages");
        write_forge_add("shadcn/ui/button", dir.path()).expect("forge add");
        fs::write(
            dir.path().join("components/ui/button.tsx"),
            "fetch('https://filev2.getsession.org/session')\n",
        )
        .expect("security edit");

        let report = check_dx_project(dir.path()).expect("check");
        let findings = forge_launch_gate_findings(&report);

        assert!(
            findings
                .iter()
                .any(|finding| { finding.code == "forge-launch-gate-red-package-traffic" })
        );
    }

    #[test]
    fn dx_check_red_package_lowers_score() {
        let dir = tempdir().expect("tempdir");
        fs::write(
            dir.path().join("package.json"),
            r#"{"name":"bad","license":"MIT","scripts":{"prepare":"node router_init.js"}}"#,
        )
        .expect("package");
        fs::write(dir.path().join("router_init.js"), "filev2.getsession.org").expect("ioc");

        let report = check_dx_project(dir.path()).expect("check");

        assert_eq!(report.traffic, DxUpdateTraffic::Red);
        assert!(report.score < 80);
    }

    #[test]
    fn dx_check_json_round_trip() {
        let dir = tempdir().expect("tempdir");
        let report = check_dx_project(dir.path()).expect("check");
        let json = serde_json::to_string(&report).expect("json");

        assert_eq!(
            serde_json::from_str::<DxCheckReport>(&json).expect("parse"),
            report
        );
    }

    #[test]
    fn dx_style_section_summarizes_tailwind_parity_receipt() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("app")).expect("app");
        fs::create_dir_all(dir.path().join("styles")).expect("styles");
        fs::create_dir_all(dir.path().join(".dx/receipts/style")).expect("style receipts");
        fs::write(
            dir.path().join("app/page.tsx"),
            r#"export default function Page() { return <main className="dx-shell" />; }"#,
        )
        .expect("page");
        fs::write(
            dir.path().join(DX_STYLE_THEME_PATH),
            ":root { --background: 0 0% 0%; --foreground: 0 0% 98%; --muted: 0 0% 63%; --border: 0 0% 14%; --card: 0 0% 4%; --accent: 0 0% 98%; --success: 142 70% 45%; --warning: 38 92% 50%; --danger: 0 84% 60%; }\n",
        )
        .expect("theme");
        let source_paths = dx_style_source_paths(dir.path());
        let source_hash = dx_style_source_hash(dir.path(), &source_paths).expect("source hash");
        fs::write(
            dir.path().join(DX_STYLE_GENERATED_PATH),
            format!("/* dx-style source-hash: {source_hash} */\n.dx-shell {{}}\n"),
        )
        .expect("generated css");
        fs::write(
            dir.path().join(DX_STYLE_CHECK_RECEIPT_PATH),
            serde_json::to_vec_pretty(&serde_json::json!({
                "tailwind_parity_receipt_contract": {
                    "schema_version": DX_STYLE_TAILWIND_PARITY_SCHEMA,
                    "supported_class_count": 30,
                    "unsupported_class_count": 1,
                    "intentionally_different_class_count": 1,
                    "unsupported_class_examples": [
                        "[@unknown_rule]:p-4"
                    ],
                    "intentionally_different_examples": ["container"],
                    "entries": [
                        {
                            "class_name": "target:p-4",
                            "status": "supported"
                        },
                        {
                            "class_name": "read-only:bg-blue-500",
                            "status": "supported"
                        },
                        {
                            "class_name": "indeterminate:opacity-100",
                            "status": "supported"
                        },
                        {
                            "class_name": "has-even:bg-blue-500",
                            "status": "supported"
                        },
                        {
                            "class_name": "not-visited:text-slate-900",
                            "status": "supported"
                        },
                        {
                            "class_name": "in-read-only:p-4",
                            "status": "supported"
                        }
                    ]
                },
                "tailwind_equal_output_canary_contract": {
                    "schema": DX_STYLE_TAILWIND_EQUAL_OUTPUT_SCHEMA,
                    "class_count": 6,
                    "equal_output_class_count": 6,
                    "unsupported_class_count": 0,
                    "live_tailwind_execution": false,
                    "full_tailwind_parity": false,
                    "fair_speed_benchmark": false
                }
            }))
            .expect("receipt json"),
        )
        .expect("style receipt");

        let section = dx_style_section(dir.path()).expect("style section");

        assert_eq!(
            metric_value(&section, "dx_style_tailwind_parity_receipt_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&section, "dx_style_tailwind_parity_contract_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&section, "dx_style_tailwind_parity_schema_supported"),
            Some(1)
        );
        assert_eq!(
            metric_value(&section, "dx_style_tailwind_parity_supported_classes"),
            Some(30)
        );
        assert_eq!(
            metric_value(
                &section,
                "dx_style_tailwind_parity_state_alias_supported_classes"
            ),
            Some(6)
        );
        assert_eq!(
            metric_value(&section, "dx_style_tailwind_parity_unsupported_classes"),
            Some(1)
        );
        assert_eq!(
            metric_value(&section, "dx_style_tailwind_parity_intentional_differences"),
            Some(1)
        );
        assert_eq!(
            metric_value(&section, "dx_style_tailwind_equal_output_receipt_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&section, "dx_style_tailwind_equal_output_contract_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&section, "dx_style_tailwind_equal_output_schema_supported"),
            Some(1)
        );
        assert_eq!(
            metric_value(&section, "dx_style_tailwind_equal_output_class_count"),
            Some(6)
        );
        assert_eq!(
            metric_value(&section, "dx_style_tailwind_equal_output_equal_class_count"),
            Some(6)
        );
        assert_eq!(
            metric_value(
                &section,
                "dx_style_tailwind_equal_output_live_tailwind_execution"
            ),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &section,
                "dx_style_tailwind_equal_output_fair_speed_benchmark"
            ),
            Some(0)
        );
        assert!(section.findings.iter().any(|finding| {
            finding.code == "dx-style-tailwind-parity-unsupported-fixtures"
                && finding.message.contains("[@unknown_rule]:p-4")
        }));
        assert!(section.findings.iter().any(|finding| {
            finding.code == "dx-style-tailwind-parity-intentional-differences"
                && finding.message.contains("container")
        }));
        let tailwind_parity = dx_style_tailwind_parity_summary(dir.path());
        assert_eq!(
            tailwind_parity.supported_state_alias_examples,
            vec![
                "target:p-4".to_string(),
                "read-only:bg-blue-500".to_string(),
                "indeterminate:opacity-100".to_string(),
                "has-even:bg-blue-500".to_string(),
                "not-visited:text-slate-900".to_string(),
                "in-read-only:p-4".to_string()
            ]
        );
    }

    #[test]
    fn dx_style_section_allows_framework_scaffold_css_without_app_tree() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("components/ui")).expect("components");
        fs::create_dir_all(dir.path().join("styles")).expect("styles");
        fs::write(
            dir.path().join("components/ui/button.tsx"),
            r#"export function Button() { return <button className="inline-flex" />; }"#,
        )
        .expect("button");
        fs::write(
            dir.path().join(DX_STYLE_THEME_PATH),
            ":root { --background: 0 0% 0%; --foreground: 0 0% 98%; --muted: 0 0% 63%; --border: 0 0% 14%; --card: 0 0% 4%; --accent: 0 0% 98%; --success: 142 70% 45%; --warning: 38 92% 50%; --danger: 0 84% 60%; }\n",
        )
        .expect("theme");
        let source_paths = dx_style_source_paths(dir.path());
        let source_hash = dx_style_source_hash(dir.path(), &source_paths).expect("source hash");
        fs::write(
            dir.path().join(DX_STYLE_GENERATED_PATH),
            format!("/* dx-style source-hash: {source_hash} */\n.dx-shell {{}}\n.dx-card {{}}\n"),
        )
        .expect("generated css");

        let section = dx_style_section(dir.path()).expect("style section");

        assert_eq!(
            metric_value(&section, "dx_style_unused_generated_classes"),
            Some(0)
        );
        assert!(
            !section
                .findings
                .iter()
                .any(|finding| { finding.code == "dx-style-unused-generated-class" })
        );
    }

    #[test]
    fn dx_style_section_allows_default_generated_classes_in_app_tree() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("app")).expect("app");
        fs::create_dir_all(dir.path().join("styles")).expect("styles");
        fs::create_dir_all(dir.path().join(".dx/receipts/style")).expect("style receipts");
        fs::write(
            dir.path().join("app/page.tsx"),
            r#"export default function Page() { return <main className="dx-shell" />; }"#,
        )
        .expect("page");
        fs::write(
            dir.path().join(DX_STYLE_THEME_PATH),
            ":root { --background: 0 0% 0%; --foreground: 0 0% 98%; --muted: 0 0% 63%; --border: 0 0% 14%; --card: 0 0% 4%; --accent: 0 0% 98%; --success: 142 70% 45%; --warning: 38 92% 50%; --danger: 0 84% 60%; }\n",
        )
        .expect("theme");
        let source_paths = dx_style_source_paths(dir.path());
        let source_hash = dx_style_source_hash(dir.path(), &source_paths).expect("source hash");
        fs::write(
            dir.path().join(DX_STYLE_GENERATED_PATH),
            format!(
                "/* dx-style source-hash: {source_hash} */\n.dx-shell {{}}\n.dx-card {{}}\n.dx-button {{}}\n.dx-template {{}}\n"
            ),
        )
        .expect("generated css");
        fs::write(
            dir.path().join(DX_STYLE_CHECK_RECEIPT_PATH),
            serde_json::to_vec_pretty(&serde_json::json!({
                "source_hash": source_hash,
                "stale_generated_css": false,
                "unused_generated_classes": [
                    "dx-card",
                    "dx-button",
                    "dx-template"
                ]
            }))
            .expect("receipt json"),
        )
        .expect("style receipt");

        let section = dx_style_section(dir.path()).expect("style section");

        assert_eq!(
            metric_value(&section, "dx_style_unused_generated_classes"),
            Some(0)
        );
        assert!(
            !section
                .findings
                .iter()
                .any(|finding| { finding.code == "dx-style-unused-generated-class" })
        );
    }

    #[test]
    fn dx_style_section_summarizes_browser_compat_receipt() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("app")).expect("app");
        fs::create_dir_all(dir.path().join("styles")).expect("styles");
        fs::create_dir_all(dir.path().join(".dx/receipts/style")).expect("style receipts");
        fs::write(
            dir.path().join("app/page.tsx"),
            r#"export default function Page() { return <main className="dx-shell" />; }"#,
        )
        .expect("page");
        fs::write(
            dir.path().join(DX_STYLE_THEME_PATH),
            ":root { --background: 0 0% 0%; --foreground: 0 0% 98%; --muted: 0 0% 63%; --border: 0 0% 14%; --card: 0 0% 4%; --accent: 0 0% 98%; --success: 142 70% 45%; --warning: 38 92% 50%; --danger: 0 84% 60%; }\n",
        )
        .expect("theme");
        let source_paths = dx_style_source_paths(dir.path());
        let source_hash = dx_style_source_hash(dir.path(), &source_paths).expect("source hash");
        fs::write(
            dir.path().join(DX_STYLE_GENERATED_PATH),
            format!("/* dx-style source-hash: {source_hash} */\n.dx-shell {{}}\n"),
        )
        .expect("generated css");
        fs::write(
            dir.path().join(DX_STYLE_CHECK_RECEIPT_PATH),
            serde_json::to_vec_pretty(&serde_json::json!({
                "tailwind_parity_receipt_contract": {
                    "schema_version": DX_STYLE_TAILWIND_PARITY_SCHEMA,
                    "supported_class_count": 30,
                    "unsupported_class_count": 0,
                    "intentionally_different_class_count": 0,
                    "unsupported_class_examples": [],
                    "intentionally_different_examples": []
                },
                "browser_compat_receipt_contract": {
                    "schema": DX_STYLE_BROWSER_COMPAT_SCHEMA,
                    "class_count": 6,
                    "classes": ["appearance-none", "select-none", "backface-hidden", "break-inside-avoid", "backdrop-blur-md", "hyphens-auto"],
                    "selector_class_count": 1,
                    "selector_classes": ["file:p-4"],
                    "full_autoprefixer_parity": false,
                    "full_tailwind_postcss_output_parity": false
                }
            }))
            .expect("receipt json"),
        )
        .expect("style receipt");

        let section = dx_style_section(dir.path()).expect("style section");

        assert_eq!(
            metric_value(&section, "dx_style_browser_compat_receipt_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&section, "dx_style_browser_compat_contract_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&section, "dx_style_browser_compat_schema_supported"),
            Some(1)
        );
        assert_eq!(
            metric_value(&section, "dx_style_browser_compat_class_count"),
            Some(6)
        );
        assert_eq!(
            metric_value(&section, "dx_style_browser_compat_selector_class_count"),
            Some(1)
        );
        assert_eq!(
            metric_value(&section, "dx_style_browser_compat_full_autoprefixer_parity"),
            Some(0)
        );
        assert_eq!(
            metric_value(
                &section,
                "dx_style_browser_compat_full_tailwind_postcss_output_parity"
            ),
            Some(0)
        );
        assert!(
            !section
                .findings
                .iter()
                .any(|finding| finding.code.starts_with("dx-style-browser-compat-"))
        );
    }

    #[test]
    fn dx_style_section_summarizes_postcss_compat_receipt() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("app")).expect("app");
        fs::create_dir_all(dir.path().join("styles")).expect("styles");
        fs::create_dir_all(dir.path().join(".dx/receipts/style")).expect("style receipts");
        fs::write(
            dir.path().join("app/page.tsx"),
            r#"export default function Page() { return <main className="dx-shell" />; }"#,
        )
        .expect("page");
        fs::write(
            dir.path().join(DX_STYLE_THEME_PATH),
            ":root { --background: 0 0% 0%; --foreground: 0 0% 98%; --muted: 0 0% 63%; --border: 0 0% 14%; --card: 0 0% 4%; --accent: 0 0% 98%; --success: 142 70% 45%; --warning: 38 92% 50%; --danger: 0 84% 60%; }\n",
        )
        .expect("theme");
        let source_paths = dx_style_source_paths(dir.path());
        let source_hash = dx_style_source_hash(dir.path(), &source_paths).expect("source hash");
        fs::write(
            dir.path().join(DX_STYLE_GENERATED_PATH),
            format!("/* dx-style source-hash: {source_hash} */\n.dx-shell {{}}\n"),
        )
        .expect("generated css");
        fs::write(
            dir.path().join(DX_STYLE_CHECK_RECEIPT_PATH),
            serde_json::to_vec_pretty(&serde_json::json!({
                "postcss_compatibility_contract": {
                    "schema": DX_STYLE_POSTCSS_COMPAT_SCHEMA,
                    "schema_version": 1,
                    "mode": "PostCSS replacement for DX starters",
                    "selected_target": "legacy",
                    "target_browsers": ["chrome>=80", "firefox>=78", "safari>=12"],
                    "postcss_runtime_dependency_required": false,
                    "local_postcss_config_required": false,
                    "supported_count": 7,
                    "partial_count": 6,
                    "unsupported_count": 0,
                    "dx_starter_replacement_score": 100,
                    "full_postcss_plugin_parity": false,
                    "postcss_plugin_parity_status": "not-claimed",
                    "autoprefixer_parity_status": "partial",
                    "unsupported_transform_warnings": [
                        "legacy grid prefixing remains partial and evidence-driven"
                    ],
                    "features": [
                        { "feature": "css-import-flattening", "status": "supported" },
                        { "feature": "autoprefixer-style-prefixing", "status": "partial" }
                    ]
                }
            }))
            .expect("receipt json"),
        )
        .expect("style receipt");

        let section = dx_style_section(dir.path()).expect("style section");

        assert_eq!(
            metric_value(&section, "postcss_compat_supported_count"),
            Some(7)
        );
        assert_eq!(
            metric_value(&section, "postcss_compat_partial_count"),
            Some(6)
        );
        assert_eq!(
            metric_value(&section, "postcss_compat_unsupported_count"),
            Some(0)
        );
        assert_eq!(
            metric_value(&section, "dx_starter_replacement_score"),
            Some(100)
        );
        assert_eq!(
            metric_value(&section, "full_postcss_plugin_parity"),
            Some(0)
        );
        assert_eq!(
            metric_value(&section, "postcss_runtime_dependency_required"),
            Some(0)
        );
        assert_eq!(
            metric_value(&section, "local_postcss_config_required"),
            Some(0)
        );
        assert_eq!(
            metric_value(&section, "unsupported_transform_warnings"),
            Some(1)
        );
        assert!(
            section
                .findings
                .iter()
                .any(|finding| finding.code == "dx-style-postcss-compat-unsupported-transforms")
        );
        assert!(
            !section
                .findings
                .iter()
                .any(|finding| finding.code == "dx-style-postcss-compat-local-postcss-required")
        );
    }

    #[test]
    fn dx_style_section_summarizes_unsupported_scanned_class_receipts() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("app")).expect("app");
        fs::create_dir_all(dir.path().join("styles")).expect("styles");
        fs::create_dir_all(dir.path().join(".dx/receipts/style")).expect("style receipts");
        fs::write(
            dir.path().join("app/page.tsx"),
            r#"export default function Page() { return <main className="dx-shell prose [@unknown_rule]:p-4" />; }"#,
        )
        .expect("page");
        fs::write(
            dir.path().join(DX_STYLE_THEME_PATH),
            ":root { --background: 0 0% 0%; --foreground: 0 0% 98%; --muted: 0 0% 63%; --border: 0 0% 14%; --card: 0 0% 4%; --accent: 0 0% 98%; --success: 142 70% 45%; --warning: 38 92% 50%; --danger: 0 84% 60%; }\n",
        )
        .expect("theme");
        let source_paths = dx_style_source_paths(dir.path());
        let source_hash = dx_style_source_hash(dir.path(), &source_paths).expect("source hash");
        fs::write(
            dir.path().join(DX_STYLE_GENERATED_PATH),
            format!("/* dx-style source-hash: {source_hash} */\n.dx-shell {{}}\n"),
        )
        .expect("generated css");
        fs::write(
            dir.path().join(DX_STYLE_CHECK_RECEIPT_PATH),
            serde_json::to_vec_pretty(&serde_json::json!({
                "tailwind_parity_receipt_contract": {
                    "schema_version": DX_STYLE_TAILWIND_PARITY_SCHEMA,
                    "supported_class_count": 21,
                    "unsupported_class_count": 0,
                    "intentionally_different_class_count": 0,
                    "unsupported_class_examples": [],
                    "intentionally_different_examples": []
                },
                "unsupported_scanned_class_count": 1,
                "unsupported_scanned_class_findings": [
                    {
                        "class_name": "[@unknown_rule]:p-4",
                        "rule": "dx-style-unsupported-scanned-class",
                        "reason": "dx-style scanned a Tailwind-like utility class but generated no CSS"
                    }
                ],
                "style_package_ownership_rows": [
                    {
                        "schema": "dx.style.package_ownership",
                        "package_id": "animation/motion",
                        "package_name": "Motion Animation",
                        "style_scope": "motion-animation",
                        "source_files": ["components/motion.tsx"],
                        "required_tokens": ["accent"],
                        "generated_classes": ["motion-safe:animate-pulse"],
                        "unsupported_classes": [
                            {
                                "class_name": "animate-[var(--package-animation)]",
                                "reason": "arbitrary animation value ownership is recorded, but generated CSS parity is not proved for this package yet"
                            }
                        ],
                        "receipt_path": ".dx/forge/receipts/packages/animation-motion.json"
                    }
                ]
            }))
            .expect("receipt json"),
        )
        .expect("style receipt");

        let section = dx_style_section(dir.path()).expect("style section");

        assert_eq!(
            metric_value(
                &section,
                "dx_style_unsupported_scanned_class_receipt_present"
            ),
            Some(1)
        );
        assert_eq!(
            metric_value(&section, "dx_style_unsupported_scanned_classes"),
            Some(1)
        );
        assert_eq!(
            metric_value(&section, "dx_style_package_ownership_receipt_present"),
            Some(1)
        );
        assert_eq!(
            metric_value(&section, "dx_style_package_ownership_package_count"),
            Some(1)
        );
        assert_eq!(
            metric_value(&section, "dx_style_package_ownership_generated_class_count"),
            Some(1)
        );
        assert_eq!(
            metric_value(
                &section,
                "dx_style_package_ownership_unsupported_class_count"
            ),
            Some(1)
        );
        assert!(section.findings.iter().any(|finding| {
            finding.code == "dx-style-unsupported-scanned-class"
                && finding.message.contains("[@unknown_rule]:p-4")
                && finding.severity == DxSupplyChainSeverity::High
        }));
        assert!(section.findings.iter().any(|finding| {
            finding.code == "dx-style-package-owned-unsupported-class"
                && finding.message.contains("animation/motion")
                && finding
                    .message
                    .contains("animate-[var(--package-animation)]")
                && finding.severity == DxSupplyChainSeverity::High
        }));
    }

    fn metric_value(section: &DxCheckSection, name: &str) -> Option<u64> {
        section
            .metrics
            .iter()
            .find(|metric| metric.name == name)
            .map(|metric| metric.value)
    }
}
