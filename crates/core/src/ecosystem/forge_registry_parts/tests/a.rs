    use super::super::forge_remote_health::{
        DxForgeRemoteObjectHeadHealthStatus, evaluate_r2_remote_object_head_receipt_health,
    };
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn registry_index_and_package_round_trip_json() {
        let package = registry_package("shadcn/ui/button").expect("package");
        let index = DxForgeRegistryIndex {
            version: 1,
            generated_at: "2026-05-16T00:00:00Z".to_string(),
            packages: vec![package.clone_without_content()],
            remotes: Vec::new(),
        };

        let package_json = serde_json::to_string(&package).expect("package json");
        let index_json = serde_json::to_string(&index).expect("index json");

        assert_eq!(
            serde_json::from_str::<DxForgeRegistryPackage>(&package_json).expect("package parse"),
            package.clone_without_content()
        );
        assert_eq!(
            serde_json::from_str::<DxForgeRegistryIndex>(&index_json).expect("index parse"),
            index
        );
    }

    #[test]
    fn registry_and_source_package_metadata_exposes_curated_non_live_advisory_fixtures() {
        let dir = tempdir().expect("tempdir");
        let registry_package = registry_package("shadcn/ui/button").expect("registry package");
        let source_package =
            source_package_for_project("shadcn/ui/button", dir.path()).expect("source package");
        let registry_json = serde_json::to_value(&registry_package).expect("registry json");
        let source_json = serde_json::to_value(&source_package).expect("source json");

        assert_eq!(
            registry_json["provenance"]["source"],
            "dx-forge-curated-registry"
        );
        assert_eq!(registry_json["provenance"]["verified"], false);
        assert!(
            registry_json["provenance"]["note"]
                .as_str()
                .expect("provenance note")
                .contains("not SLSA")
        );
        assert_eq!(
            registry_json["advisory_review"]["coverage_kind"],
            "curated-fixture"
        );
        assert_eq!(registry_json["advisory_review"]["live_coverage"], false);
        assert_eq!(
            registry_json["advisory_review"]["provider"],
            "dx-forge-curated-advisory-fixture"
        );
        assert_eq!(registry_json["advisory_review"]["finding_count"], 0);
        assert!(registry_json["advisory_review"]["reviewed_at"].is_string());
        assert!(
            registry_json["advisory_review"]["note"]
                .as_str()
                .expect("advisory note")
                .contains("not a live advisory feed")
        );
        assert_eq!(
            registry_json["license_review"]["declared_license"],
            registry_package.license
        );
        assert_eq!(registry_json["license_review"]["reviewed"], false);

        assert_eq!(source_json["provenance"], registry_json["provenance"]);
        assert_eq!(
            source_json["advisory_review"],
            registry_json["advisory_review"]
        );
        assert_eq!(
            source_json["license_review"],
            registry_json["license_review"]
        );
    }

    #[test]
    fn registry_package_integrity_verifies_manifest_and_file_content() {
        let package = registry_package("shadcn/ui/button").expect("package");

        let report = verify_registry_package_integrity(&package).expect("integrity");

        assert_eq!(report.package_id, "shadcn/ui/button");
        assert_eq!(report.file_count, package.files.len() as u64);
        assert_eq!(report.verified_files, package.files.len() as u64);
        assert_eq!(report.integrity_hash, package.integrity_hash);
    }

    #[test]
    fn registry_package_integrity_rejects_tampered_content() {
        let mut package = registry_package("shadcn/ui/button").expect("package");
        package.files[0].content = Some("tampered".to_string());

        assert!(verify_registry_package_integrity(&package).is_err());
    }

    #[test]
    fn local_registry_package_loader_hydrates_verified_blobs() {
        let dir = tempdir().expect("tempdir");
        init_local_registry(dir.path()).expect("init");

        let package =
            load_local_registry_package(dir.path(), "shadcn/ui/button", "0.1.0").expect("load");
        let report = verify_registry_package_integrity(&package).expect("integrity");

        assert_eq!(package.package_id, "shadcn/ui/button");
        assert!(package.files.iter().all(|file| file.content.is_some()));
        assert_eq!(report.verified_files, report.file_count);
    }

    #[test]
    fn local_registry_package_loader_rejects_tampered_blob() {
        let dir = tempdir().expect("tempdir");
        init_local_registry(dir.path()).expect("init");
        let package = registry_package("shadcn/ui/button").expect("package");
        let first_blob = dir
            .path()
            .join(relative_registry_file(&package, &package.files[0].hash));
        fs::write(&first_blob, "tampered").expect("tamper blob");

        assert!(load_local_registry_package(dir.path(), "shadcn/ui/button", "0.1.0").is_err());
    }

    #[test]
    fn root_dx_package_publishes_to_local_registry_with_export_map() {
        let project = tempdir().expect("project");
        let registry = tempdir().expect("registry");
        fs::create_dir_all(project.path().join("src/state")).expect("source dirs");
        fs::write(
            project.path().join("src/state/store.ts"),
            "export const createCounterStore = () => ({ count: 0 });\n",
        )
        .expect("store source");
        fs::write(
            project.path().join("src/state/provider.tsx"),
            "export function ZustandProvider() { return null; }\n",
        )
        .expect("provider source");
        fs::write(
            project.path().join("dx"),
            r#"
[package]
name = "state/zustand"
version = "0.1.0"
description = "Source-owned Zustand front-facing package"
license = "MIT"
source = "."

[forge]
package = true
visibility = "public"
registry = "local"

[[forge.files]]
from = "src/state/store.ts"
to = "lib/state/zustand/store.ts"
surface = "store"

[[forge.files]]
from = "src/state/provider.tsx"
to = "components/state/zustand-provider.tsx"
surface = "ui"

[[forge.exports]]
name = "store"
files = ["lib/state/zustand/store.ts"]

[[forge.exports]]
name = "ui"
files = ["components/state/zustand-provider.tsx"]

[forge.install]
default_exports = ["store", "ui"]
allow_selective_imports = true
"#,
        )
        .expect("dx manifest");

        let report =
            publish_root_dx_package_to_local_registry(project.path(), registry.path(), false)
                .expect("publish root dx package");
        let package =
            load_local_registry_package(registry.path(), "state/zustand", "0.1.0").expect("load");

        assert_eq!(report.action, "registry-publish");
        assert_eq!(report.package_id.as_deref(), Some("state/zustand"));
        assert_eq!(report.version.as_deref(), Some("0.1.0"));
        assert!(registry.path().join("index.json").exists());
        assert!(
            registry
                .path()
                .join("packages/js/state/zustand/0.1.0/manifest.json")
                .exists()
        );
        assert_eq!(package.source_kind, DxSourceKind::Local);
        assert_eq!(package.exports.len(), 2);
        assert_eq!(package.default_exports, vec!["store", "ui"]);
        assert!(package.allow_selective_imports);

        let selected = source_package_from_local_registry_selected_exports(
            registry.path(),
            "state/zustand",
            "0.1.0",
            &["store".to_string()],
            project.path(),
        )
        .expect("selected source package");

        assert_eq!(selected.package_id, "state/zustand");
        assert_eq!(selected.variant, "export-store");
        assert_eq!(selected.files.len(), 1);
        assert_eq!(selected.files[0].path, "lib/state/zustand/store.ts");
        assert!(
            selected.files[0]
                .content
                .as_deref()
                .unwrap_or_default()
                .contains("createCounterStore")
        );
    }

    #[test]
    fn root_dx_package_publish_dry_run_plans_without_writing_registry_files() {
        let project = tempdir().expect("project");
        let registry = tempdir().expect("registry");
        fs::create_dir_all(project.path().join("src")).expect("source dirs");
        fs::write(
            project.path().join("src/client.ts"),
            "export const client = true;\n",
        )
        .expect("client source");
        fs::write(
            project.path().join("dx"),
            r#"
[package]
name = "auth/better-auth"
version = "0.1.0"
license = "MIT"
source = "."

[forge]
package = true

[[forge.files]]
from = "src/client.ts"
to = "lib/auth/better-auth/client.ts"
surface = "client"

[[forge.exports]]
name = "client"
files = ["lib/auth/better-auth/client.ts"]

[forge.install]
default_exports = ["client"]
allow_selective_imports = true
"#,
        )
        .expect("dx manifest");

        let report =
            publish_root_dx_package_to_local_registry(project.path(), registry.path(), true)
                .expect("dry-run root dx package publish");

        assert_eq!(report.action, "registry-publish-plan");
        assert!(report.dry_run);
        assert_eq!(report.package_id.as_deref(), Some("auth/better-auth"));
        assert!(
            report
                .objects
                .iter()
                .any(|object| object.ends_with("packages/js/auth/better-auth/0.1.0/manifest.json"))
        );
        assert!(!registry.path().join("index.json").exists());
    }

    #[test]
    fn root_dx_package_r2_dry_run_plans_object_keys_without_secrets() {
        let project = tempdir().expect("project");
        fs::create_dir_all(project.path().join("src")).expect("source dirs");
        fs::write(
            project.path().join("src/client.ts"),
            "export const client = true;\n",
        )
        .expect("client source");
        fs::write(
            project.path().join("dx"),
            r#"
[package]
name = "auth/better-auth"
version = "0.1.0"
license = "MIT"
source = "."

[forge]
package = true

[[forge.files]]
from = "src/client.ts"
to = "lib/auth/better-auth/client.ts"
surface = "client"

[[forge.exports]]
name = "client"
files = ["lib/auth/better-auth/client.ts"]

[forge.install]
default_exports = ["client"]
allow_selective_imports = true
"#,
        )
        .expect("dx manifest");

        let report =
            publish_root_dx_package_to_r2_dry_run(project.path()).expect("r2 dry-run plan");
        let serialized = serde_json::to_string(&report).expect("report json");

        assert_eq!(report.action, "registry-publish");
        assert!(report.dry_run);
        assert_eq!(report.remote, "r2");
        assert_eq!(report.package_id.as_deref(), Some("auth/better-auth"));
        assert!(
            report
                .objects
                .iter()
                .any(|object| object.contains("packages/js/auth/better-auth/0.1.0/manifest.json"))
        );
        assert!(!serialized.contains("SECRET_ACCESS_KEY"));
        assert!(!serialized.contains("ACCESS_KEY_ID"));
    }

    #[test]
    fn r2_config_parses_status_without_secrets() {
        let lookup = |key: &str| match key {
            "CLOUDFLARE_R2_ACCOUNT_ID" => Some("abc".to_string()),
            "CLOUDFLARE_R2_BUCKET" => Some("dx".to_string()),
            "CLOUDFLARE_R2_ACCESS_KEY_ID" => Some("access".to_string()),
            "CLOUDFLARE_R2_SECRET_ACCESS_KEY" => Some("secret".to_string()),
            "DX_FORGE_R2_PREFIX" => Some("custom/prefix".to_string()),
            _ => None,
        };

        let status = DxForgeR2Config::status_from_lookup(lookup);

        assert!(status.configured);
        assert_eq!(status.setup_status, "configured");
        assert!(status.missing_config.is_empty());
        assert_eq!(status.bucket.as_deref(), Some("dx"));
        assert_eq!(status.prefix, "custom/prefix");
        assert_eq!(status.access_key_id_set, true);
        assert_eq!(status.secret_access_key_set, true);
        assert_eq!(status.bucket_set, true);
        assert_eq!(status.endpoint_set, true);
        assert_eq!(status.public_base_url_set, false);
        let serialized = serde_json::to_string(&status).expect("status json");
        assert!(!serialized.contains("\"secret\""));
        assert!(!serialized.contains("\"access\""));
        assert!(!serialized.contains("abc.r2.cloudflarestorage.com"));
    }

    #[test]
    fn r2_config_missing_secret_is_not_configured() {
        let status = DxForgeR2Config::status_from_lookup(|key| match key {
            "CLOUDFLARE_R2_ACCOUNT_ID" => Some("abc".to_string()),
            "CLOUDFLARE_R2_BUCKET" => Some("dx".to_string()),
            _ => None,
        });

        assert!(!status.configured);
        assert_eq!(status.setup_status, "partial-config");
        assert_eq!(
            status.missing_config,
            vec!["access_key_id", "secret_access_key"]
        );
        assert!(status.account_id_set);
        assert_eq!(status.access_key_id_set, false);
        assert_eq!(status.secret_access_key_set, false);
        assert_eq!(status.bucket_set, true);
        assert_eq!(status.endpoint_set, true);
        assert_eq!(status.public_base_url_set, false);
    }

    #[test]
    fn r2_config_reports_partial_credentials_without_secret_values() {
        let status = DxForgeR2Config::status_from_lookup(|key| match key {
            "CLOUDFLARE_R2_ACCOUNT_ID" => Some("partial-account".to_string()),
            "CLOUDFLARE_R2_BUCKET" => Some("partial-bucket".to_string()),
            "CLOUDFLARE_R2_ACCESS_KEY_ID" => Some("partial-access".to_string()),
            _ => None,
        });

        assert!(!status.configured);
        assert_eq!(status.setup_status, "partial-config");
        assert_eq!(status.missing_config, vec!["secret_access_key"]);
        assert_eq!(status.account_id_set, true);
        assert_eq!(status.access_key_id_set, true);
        assert_eq!(status.secret_access_key_set, false);
        assert_eq!(status.bucket_set, true);
        assert_eq!(status.endpoint_set, true);
        assert_eq!(status.public_base_url_set, false);
        assert_eq!(status.bucket.as_deref(), Some("partial-bucket"));

        let serialized = serde_json::to_string(&status).expect("status json");
        assert!(!serialized.contains("partial-access"));
        assert!(!serialized.contains("partial-secret"));
        assert!(!serialized.contains("partial-account"));
        assert!(!serialized.contains("partial-bucket"));
    }

    #[test]
    fn r2_config_accepts_s3_compatible_fallback_env_without_secrets() {
        let lookup = |key: &str| match key {
            "CLOUDFLARE_ACCOUNT_ID" => Some("cloudflare-account".to_string()),
            "DX_FORGE_R2_BUCKET" => Some("dx-forge".to_string()),
            "AWS_ACCESS_KEY_ID" => Some("aws-access".to_string()),
            "AWS_SECRET_ACCESS_KEY" => Some("aws-secret".to_string()),
            "AWS_ENDPOINT_URL" => Some("https://example-r2.invalid".to_string()),
            _ => None,
        };

        let status = DxForgeR2Config::status_from_lookup(lookup);

        assert!(status.configured);
        assert_eq!(status.setup_status, "configured");
        assert!(status.missing_config.is_empty());
        assert!(status.account_id_set);
        assert_eq!(status.access_key_id_set, true);
        assert_eq!(status.secret_access_key_set, true);
        assert_eq!(status.bucket_set, true);
        assert_eq!(status.endpoint_set, true);
        assert_eq!(status.public_base_url_set, false);
        assert_eq!(status.bucket.as_deref(), Some("dx-forge"));
        assert_eq!(
            status.endpoint.as_deref(),
            Some("https://example-r2.invalid")
        );
        let serialized = serde_json::to_string(&status).expect("status json");
        assert!(!serialized.contains("aws-secret"));
        assert!(!serialized.contains("aws-access"));
        assert!(!serialized.contains("cloudflare-account"));
        assert!(!serialized.contains("\"bucket\":\"dx-forge\""));
        assert!(!serialized.contains("\"endpoint\":\"https://example-r2.invalid\""));
    }

    #[test]
    fn r2_remote_read_only_plan_keeps_network_and_writes_disabled() {
        let selected_exports = vec!["client".to_string()];
        let plan = plan_r2_remote_read_only_install(
            DxForgeRemoteReadIntent::InstallDryRun,
            "auth/better-auth",
            &selected_exports,
            Some("0.1.0"),
        );

        assert_eq!(plan.schema_version, "dx.forge.remote_read_plan");
        assert_eq!(
            plan.provider_kind,
            DxForgeRemoteProviderKind::S3CompatibleObjectStorage
        );
        assert_eq!(plan.intent, DxForgeRemoteReadIntent::InstallDryRun);
        assert_eq!(plan.package_id, "auth/better-auth");
        assert_eq!(plan.requested_version.as_deref(), Some("0.1.0"));
        assert_eq!(plan.selected_exports, selected_exports);
        assert!(!plan.network_allowed);
        assert!(!plan.write_allowed);
        assert!(plan.boundary.contains("no network call is performed"));
        assert!(plan.objects.iter().any(|object| {
            object.intent == "package-manifest"
                && object
                    .object_key
                    .contains("packages/js/auth/better-auth/0.1.0/manifest.json")
                && object.required
        }));
        assert!(plan.objects.iter().any(|object| {
            object.intent == "latest-version"
                && object
                    .object_key
                    .contains("packages/js/auth/better-auth/latest.json")
                && !object.required
        }));
        assert!(plan.objects.iter().any(|object| {
            object.intent == "content-blob"
                && object
                    .object_key
                    .contains("packages/js/auth/better-auth/0.1.0/files/<content-hash>")
                && object.required
        }));
    }

    #[test]
    fn r2_remote_manifest_fixture_preview_reports_front_facing_conflicts() {
        let project = tempdir().expect("project");
        fs::create_dir_all(project.path().join("src")).expect("source dirs");
        fs::write(
            project.path().join("src/client.ts"),
            "export const client = true;\n",
        )
        .expect("client source");
        fs::write(
            project.path().join("dx"),
            r#"
[package]
name = "auth/better-auth"
version = "0.1.0"
license = "MIT"
source = "."

[forge]
package = true

[[forge.files]]
from = "src/client.ts"
to = "lib/auth/better-auth/client.ts"
surface = "client"

[[forge.exports]]
name = "client"
files = ["lib/auth/better-auth/client.ts"]

[forge.install]
default_exports = ["client"]
allow_selective_imports = true
"#,
        )
        .expect("dx manifest");

        let package = root_dx_registry_package(project.path()).expect("root dx registry package");
        let manifest_dir = project.path().join(".dx/remote-fixtures");
        fs::create_dir_all(&manifest_dir).expect("manifest dir");
        let manifest_path = manifest_dir.join("manifest.json");
        fs::write(
            &manifest_path,
            serde_json::to_vec_pretty(&package.clone_without_content()).expect("manifest json"),
        )
        .expect("write manifest fixture");

        let installed_path = project.path().join("lib/auth/better-auth/client.ts");
        fs::create_dir_all(installed_path.parent().expect("parent")).expect("install parent");
        fs::write(&installed_path, "export const client = false;\n").expect("conflict source");

        let plan = plan_r2_remote_read_only_install_from_manifest_fixture(
            DxForgeRemoteReadIntent::InstallDryRun,
            "auth/better-auth",
            &["client".to_string()],
            None,
            &manifest_path,
            project.path(),
        )
        .expect("fixture-backed remote read plan");

        assert_eq!(plan.requested_version.as_deref(), Some("0.1.0"));
        assert!(!plan.network_allowed);
        assert!(!plan.write_allowed);
        assert!(plan.objects.iter().any(|object| {
            object.intent == "package-manifest"
                && object
                    .object_key
                    .contains("packages/js/auth/better-auth/0.1.0/manifest.json")
        }));

        let preview = plan
            .manifest_install_preview
            .expect("manifest install preview");
        assert_eq!(
            preview.schema_version,
            "dx.forge.remote_manifest_install_preview"
        );
        assert_eq!(preview.package_id, "auth/better-auth");
        assert_eq!(preview.version, "0.1.0");
        assert_eq!(preview.selected_exports, vec!["client"]);
        assert!(!preview.network_allowed);
        assert!(!preview.write_allowed);
        assert_eq!(preview.selected_file_count, 1);
        assert_eq!(preview.conflicting_file_count, 1);
        assert_eq!(preview.matching_file_count, 0);
        assert_eq!(preview.missing_file_count, 0);
        assert_eq!(preview.file_plans.len(), 1);
        assert_eq!(
            preview.file_plans[0].status,
            DxForgeRemoteManifestFileStatus::ConflictingLocalFile
        );
        assert_eq!(
            preview.file_plans[0].materialized_path,
            "lib/auth/better-auth/client.ts"
        );
        assert!(preview.file_plans[0].existing_hash.is_some());
        assert!(
            preview
                .warnings
                .iter()
                .any(|warning| warning.contains("no R2/S3 object was fetched"))
        );
    }

    #[test]
    fn r2_remote_manifest_fixture_plans_object_metadata_without_network() {
        let project = tempdir().expect("project");
        fs::create_dir_all(project.path().join("src")).expect("source dirs");
        fs::write(
            project.path().join("src/client.ts"),
            "export const client = true;\n",
        )
        .expect("client source");
        fs::write(
            project.path().join("dx"),
            r#"
[package]
name = "auth/better-auth"
version = "0.1.0"
license = "MIT"
source = "."

[forge]
package = true

[[forge.files]]
from = "src/client.ts"
to = "lib/auth/better-auth/client.ts"
surface = "client"

[[forge.exports]]
name = "client"
files = ["lib/auth/better-auth/client.ts"]

[forge.install]
default_exports = ["client"]
allow_selective_imports = true
"#,
        )
        .expect("dx manifest");

        let package = root_dx_registry_package(project.path()).expect("root dx registry package");
        let manifest_dir = project.path().join(".dx/remote-fixtures");
        fs::create_dir_all(&manifest_dir).expect("manifest dir");
        let manifest_path = manifest_dir.join("manifest.json");
        fs::write(
            &manifest_path,
            serde_json::to_vec_pretty(&package.clone_without_content()).expect("manifest json"),
        )
        .expect("write manifest fixture");

        let plan = plan_r2_remote_read_only_install_from_manifest_fixture(
            DxForgeRemoteReadIntent::InstallDryRun,
            "auth/better-auth",
            &["client".to_string()],
            None,
            &manifest_path,
            project.path(),
        )
        .expect("fixture-backed remote read plan");
        let metadata_plan = plan.object_metadata_plan.expect("metadata plan");

        assert_eq!(
            metadata_plan.schema_version,
            "dx.forge.remote_object_metadata_plan"
        );
        assert_eq!(
            metadata_plan.provider_kind,
            DxForgeRemoteProviderKind::S3CompatibleObjectStorage
        );
        assert_eq!(metadata_plan.package_id, "auth/better-auth");
        assert_eq!(metadata_plan.version, "0.1.0");
        assert!(!metadata_plan.network_allowed);
        assert!(!metadata_plan.write_allowed);
        assert!(metadata_plan.checks.iter().any(|check| {
            check.intent == "package-manifest"
                && check.metadata_operation == "head-object"
                && check
                    .object_key
                    .contains("packages/js/auth/better-auth/0.1.0/manifest.json")
                && check.required
                && check.status == DxForgeRemoteObjectMetadataStatus::PlannedNotChecked
        }));
        assert!(metadata_plan.checks.iter().any(|check| {
            check.intent == "content-blob"
                && check.object_key.contains(&package.files[0].hash)
                && check.expected_hash.as_deref() == Some(package.files[0].hash.as_str())
                && check.expected_bytes == Some(package.files[0].bytes)
                && check.status == DxForgeRemoteObjectMetadataStatus::PlannedNotChecked
        }));
        assert!(
            metadata_plan
                .warnings
                .iter()
                .any(|warning| warning.contains("no HEAD or GET request was performed"))
        );
    }

    #[test]
    fn r2_remote_head_execution_receipt_requires_approval_without_network() {
        let project = tempdir().expect("project");
        fs::create_dir_all(project.path().join("src")).expect("source dirs");
        fs::write(
            project.path().join("src/client.ts"),
            "export const client = true;\n",
        )
        .expect("client source");
        fs::write(
            project.path().join("dx"),
            r#"
[package]
name = "auth/better-auth"
version = "0.1.0"
license = "MIT"
source = "."

[forge]
package = true

[[forge.files]]
from = "src/client.ts"
to = "lib/auth/better-auth/client.ts"
surface = "client"

[[forge.exports]]
name = "client"
files = ["lib/auth/better-auth/client.ts"]

[forge.install]
default_exports = ["client"]
allow_selective_imports = true
"#,
        )
        .expect("dx manifest");

        let package = root_dx_registry_package(project.path()).expect("root dx registry package");
        let manifest_dir = project.path().join(".dx/remote-fixtures");
        fs::create_dir_all(&manifest_dir).expect("manifest dir");
        let manifest_path = manifest_dir.join("manifest.json");
        fs::write(
            &manifest_path,
            serde_json::to_vec_pretty(&package.clone_without_content()).expect("manifest json"),
        )
        .expect("write manifest fixture");

        let plan = plan_r2_remote_read_only_install_from_manifest_fixture(
            DxForgeRemoteReadIntent::InstallDryRun,
            "auth/better-auth",
            &["client".to_string()],
            None,
            &manifest_path,
            project.path(),
        )
        .expect("fixture-backed remote read plan");
        let health = plan
            .object_head_health_evaluation
            .as_ref()
            .expect("HEAD health evaluation");
        let receipt = plan
            .object_head_execution_receipt
            .as_ref()
            .expect("HEAD execution receipt");

        assert_eq!(
            receipt.schema_version,
            "dx.forge.remote_object_head_execution_receipt"
        );
        assert_eq!(
            receipt.source_plan_schema,
            "dx.forge.remote_object_metadata_plan"
        );
        assert!(receipt.approval_required);
        assert!(!receipt.approved);
        assert!(receipt.dry_run);
        assert!(!receipt.network_allowed);
        assert!(!receipt.write_allowed);
        assert!(receipt.checks.iter().any(|check| {
            check.intent == "content-blob"
                && check.metadata_operation == "head-object"
                && check.status == DxForgeRemoteObjectHeadExecutionStatus::RequiresExplicitApproval
                && !check.executed
                && check.expected_hash.as_deref() == Some(package.files[0].hash.as_str())
                && check.expected_bytes == Some(package.files[0].bytes)
                && check.measured_exists.is_none()
                && check.measured_bytes.is_none()
        }));
        assert!(
            receipt
                .warnings
                .iter()
                .any(|warning| warning.contains("no HEAD request was executed"))
        );
        assert!(
            receipt
                .next_actions
                .iter()
                .any(|action| action.contains("explicit operator approval"))
        );
        assert_eq!(health.schema_version, "dx.forge.remote_object_head_health");
        assert!(!health.safe_for_remote_install);
        assert!(health.blocking_check_count >= 1);
        assert!(
            health
                .next_actions
                .iter()
                .any(|action| action.contains("Block remote install"))
        );
    }

    struct FakeHeadProvider;

    impl DxForgeRemoteObjectHeadProvider for FakeHeadProvider {
        fn head_object(
            &self,
            check: &DxForgeRemoteObjectMetadataCheck,
        ) -> Result<DxForgeRemoteObjectHeadMeasurement> {
            Ok(DxForgeRemoteObjectHeadMeasurement {
                exists: true,
                bytes: check.expected_bytes,
                etag: Some(format!("fake-etag-{}", check.intent)),
                last_modified: Some("2026-05-22T00:00:00Z".to_string()),
            })
        }
    }

    #[test]
    fn r2_remote_head_execution_harness_populates_fake_provider_measurements() {
        let project = tempdir().expect("project");
        fs::create_dir_all(project.path().join("src")).expect("source dirs");
        fs::write(
            project.path().join("src/client.ts"),
            "export const client = true;\n",
        )
        .expect("client source");
        fs::write(
            project.path().join("dx"),
            r#"
[package]
name = "auth/better-auth"
version = "0.1.0"
license = "MIT"
source = "."

[forge]
package = true

[[forge.files]]
from = "src/client.ts"
to = "lib/auth/better-auth/client.ts"
surface = "client"

[[forge.exports]]
name = "client"
files = ["lib/auth/better-auth/client.ts"]

[forge.install]
default_exports = ["client"]
allow_selective_imports = true
"#,
        )
        .expect("dx manifest");

        let package = root_dx_registry_package(project.path()).expect("root dx registry package");
        let metadata_plan =
            DxForgeR2ReadOnlyProvider::from_status(&DxForgeR2Config::status_from_env())
                .object_metadata_plan(&package);
        let receipt = execute_r2_remote_object_head_checks_with_provider(
            &metadata_plan,
            DxForgeRemoteObjectHeadExecutionApproval {
                approved_by: "test-operator".to_string(),
                provider_mode: "test-provider".to_string(),
                network_allowed: false,
            },
            &FakeHeadProvider,
        )
        .expect("fake provider HEAD receipt");

        assert_eq!(
            receipt.schema_version,
            "dx.forge.remote_object_head_execution_receipt"
        );
        assert!(receipt.approval_required);
        assert!(receipt.approved);
        assert!(!receipt.dry_run);
        assert_eq!(receipt.approved_by.as_deref(), Some("test-operator"));
        assert_eq!(receipt.provider_mode, "test-provider");
        assert!(!receipt.network_allowed);
        assert!(!receipt.write_allowed);
        assert!(receipt.checks.iter().any(|check| {
            check.intent == "content-blob"
                && check.metadata_operation == "head-object"
                && check.status == DxForgeRemoteObjectHeadExecutionStatus::Measured
                && check.approved
                && check.executed
                && check.expected_hash.as_deref() == Some(package.files[0].hash.as_str())
                && check.measured_exists == Some(true)
                && check.measured_bytes == Some(package.files[0].bytes)
                && check.measured_etag.as_deref() == Some("fake-etag-content-blob")
        }));
        assert!(
            receipt
                .warnings
                .iter()
                .any(|warning| warning.contains("test provider"))
        );
        assert!(
            receipt
                .warnings
                .iter()
                .any(|warning| warning.contains("no live R2/S3 request was performed"))
        );
    }

    struct MismatchedHeadProvider;

    impl DxForgeRemoteObjectHeadProvider for MismatchedHeadProvider {
        fn head_object(
            &self,
            check: &DxForgeRemoteObjectMetadataCheck,
        ) -> Result<DxForgeRemoteObjectHeadMeasurement> {
            if check.intent == "latest-version" {
                return Ok(DxForgeRemoteObjectHeadMeasurement {
                    exists: false,
                    bytes: None,
                    etag: None,
                    last_modified: None,
                });
            }

            let measured_bytes = check.expected_bytes.map(|bytes| bytes.saturating_add(7));

            Ok(DxForgeRemoteObjectHeadMeasurement {
                exists: true,
                bytes: measured_bytes,
                etag: Some(format!("fake-mismatch-etag-{}", check.intent)),
                last_modified: Some("2026-05-22T00:00:00Z".to_string()),
            })
        }
    }

    #[test]
    fn r2_remote_head_health_evaluation_blocks_missing_required_and_byte_mismatch() {
        let project = tempdir().expect("project");
        fs::create_dir_all(project.path().join("src")).expect("source dirs");
        fs::write(
            project.path().join("src/client.ts"),
            "export const client = true;\n",
        )
        .expect("client source");
        fs::write(
            project.path().join("dx"),
            r#"
[package]
name = "auth/better-auth"
version = "0.1.0"
license = "MIT"
source = "."

[forge]
package = true

[[forge.files]]
from = "src/client.ts"
to = "lib/auth/better-auth/client.ts"
surface = "client"

[[forge.exports]]
name = "client"
files = ["lib/auth/better-auth/client.ts"]

[forge.install]
default_exports = ["client"]
allow_selective_imports = true
"#,
        )
        .expect("dx manifest");

        let package = root_dx_registry_package(project.path()).expect("root dx registry package");
        let metadata_plan =
            DxForgeR2ReadOnlyProvider::from_status(&DxForgeR2Config::status_from_env())
                .object_metadata_plan(&package);
        let receipt = execute_r2_remote_object_head_checks_with_provider(
            &metadata_plan,
            DxForgeRemoteObjectHeadExecutionApproval {
                approved_by: "test-operator".to_string(),
                provider_mode: "test-provider".to_string(),
                network_allowed: false,
            },
            &MismatchedHeadProvider,
        )
        .expect("fake provider HEAD receipt");

        let evaluation = evaluate_r2_remote_object_head_receipt_health(&receipt);

        assert_eq!(
            evaluation.schema_version,
            "dx.forge.remote_object_head_health"
        );
        assert_eq!(evaluation.source_receipt_schema, receipt.schema_version);
        assert_eq!(evaluation.package_id, "auth/better-auth");
        assert!(!evaluation.safe_for_remote_install);
        assert_eq!(evaluation.missing_required_count, 0);
        assert_eq!(evaluation.missing_optional_count, 1);
        assert_eq!(evaluation.byte_mismatch_count, 1);
        assert_eq!(evaluation.blocking_check_count, 1);
        assert!(evaluation.checks.iter().any(|check| {
            check.intent == "latest-version"
                && check.status == DxForgeRemoteObjectHeadHealthStatus::MissingOptionalObject
                && check.safe_for_remote_install
        }));
        assert!(evaluation.checks.iter().any(|check| {
            check.intent == "content-blob"
                && check.status == DxForgeRemoteObjectHeadHealthStatus::ByteMismatch
                && !check.safe_for_remote_install
                && check.expected_bytes == Some(package.files[0].bytes)
                && check.measured_bytes == Some(package.files[0].bytes + 7)
        }));
        assert!(
            evaluation
                .warnings
                .iter()
                .any(|warning| warning.contains("byte-mismatched"))
        );
        assert!(
            evaluation
                .next_actions
                .iter()
                .any(|action| action.contains("Block remote install"))
        );
    }

    #[test]
    fn path_mapping_rejects_unsafe_paths() {
        assert!(validate_project_relative_path("components/ui").is_ok());
        assert!(validate_project_relative_path("../outside").is_err());
        assert!(validate_project_relative_path("/absolute").is_err());
        assert!(validate_project_relative_path("components\\ui").is_err());
    }

    #[test]
    fn project_config_maps_front_facing_paths() {
        let dir = tempdir().expect("tempdir");
        fs::write(
            dir.path().join("dx"),
            "forge.paths.js_ui=src/components/ui\nforge.paths.js_lib=src/lib\nforge.paths.js_icons=src/icons\n",
        )
        .expect("write config");

        let config = DxForgeProjectConfig::load(dir.path()).expect("config");

        assert_eq!(
            config.materialize_path("js/ui/button.tsx").expect("path"),
            "src/components/ui/button.tsx"
        );
        assert_eq!(
            config.materialize_path("js/lib/utils.ts").expect("path"),
            "src/lib/utils.ts"
        );
        assert_eq!(
            config
                .materialize_path("js/icons/search.tsx")
                .expect("path"),
            "src/icons/search.tsx"
        );
        assert_eq!(
            config
                .materialize_path("js/auth/better-auth/providers/google/config.ts")
                .expect("path"),
            "auth/better-auth/providers/google/config.ts"
        );
        assert_eq!(
            config
                .materialize_path("js/db/drizzle/schema.ts")
                .expect("path"),
            "db/drizzle/schema.ts"
        );
        assert_eq!(
            config
                .materialize_path("js/examples/template/demo.tsx")
                .expect("path"),
            "examples/template/demo.tsx"
        );
        assert_eq!(
            config
                .materialize_path("js/openapi/dx-launch.yaml")
                .expect("path"),
            "openapi/dx-launch.yaml"
        );
        assert_eq!(
            config
                .materialize_path("js/server/content/mdx.ts")
                .expect("path"),
            "server/content/mdx.ts"
        );
    }

    #[test]
    fn ui_button_alias_resolves_to_canonical_package() {
        assert_eq!(canonical_package_id("ui/button"), "shadcn/ui/button");
        let package = default_source_package("ui/button").expect("package");
        assert_eq!(package.package_id, "shadcn/ui/button");
    }

    #[test]
    fn ui_badge_alias_materializes_real_upstream_badge_component() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("ui/badge"), "shadcn/ui/badge");

        let package = source_package_for_project("ui/badge", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        let badge = package
            .files
            .iter()
            .find(|file| file.path == "components/ui/badge.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("badge component");

        assert_eq!(package.package_id, "shadcn/ui/badge");
        assert_eq!(package.upstream_name, "shadcn-ui");
        assert!(paths.contains(&"components/ui/badge.tsx"));
        assert!(paths.contains(&"components/ui/slot.tsx"));
        assert!(paths.contains(&"components/ui/README.md"));
        assert!(paths.contains(&"lib/utils.ts"));
        assert!(badge.contains("badgeVariants"));
        assert!(badge.contains(r#"data-slot="badge""#));
        assert!(badge.contains("data-variant={variant}"));
        assert!(badge.contains(r#"asChild ? Slot.Root : "span""#));
        assert!(badge.contains("export { Badge, badgeVariants }"));
        assert!(badge.contains(r#"import { cn } from "../../lib/utils";"#));

        let registry = registry_package("shadcn/ui/badge").expect("registry package");
        assert_eq!(
            registry.provenance.upstream_reference.as_deref(),
            Some("shadcn-ui://apps/v4/registry/bases/radix/ui/badge.tsx")
        );
    }

    #[test]
    fn ui_card_alias_materializes_source_owned_component() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("ui/card"), "shadcn/ui/card");

        let package = source_package_for_project("ui/card", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();

        assert_eq!(package.package_id, "shadcn/ui/card");
        assert_eq!(package.upstream_name, "shadcn-ui");
        assert!(paths.contains(&"components/ui/card.tsx"));
        assert!(paths.contains(&"lib/utils.ts"));
        assert!(paths.contains(&"components/ui/README.md"));
        assert_eq!(
            paths
                .iter()
                .filter(|path| path.ends_with("card.tsx"))
                .count(),
            1
        );

        let registry = registry_package("shadcn/ui/card").expect("registry package");
        assert_eq!(
            registry.provenance.upstream_reference.as_deref(),
            Some("shadcn-ui://apps/v4/registry/bases/radix/ui/card.tsx")
        );

        let card = package
            .files
            .iter()
            .find(|file| file.path == "components/ui/card.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("card source");
        assert!(card.contains(r#"data-slot="card""#));
        assert!(card.contains("data-size={size}"));
        assert!(card.contains(r#"data-slot="card-header""#));
        assert!(card.contains(r#"data-slot="card-action""#));
        assert!(card.contains("function CardAction"));
        assert!(card.contains("CardAction,"));
    }

    #[test]
    fn ui_alert_alias_materializes_real_upstream_alert_component() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("ui/alert"), "shadcn/ui/alert");

        let package = source_package_for_project("ui/alert", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        let alert = package
            .files
            .iter()
            .find(|file| file.path == "components/ui/alert.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("alert component");

        assert_eq!(package.package_id, "shadcn/ui/alert");
        assert_eq!(package.upstream_name, "shadcn-ui");
        assert!(paths.contains(&"components/ui/alert.tsx"));
        assert!(paths.contains(&"components/ui/README.md"));
        assert!(paths.contains(&"lib/utils.ts"));
        assert!(alert.contains("function Alert"));
        assert!(alert.contains("function AlertTitle"));
        assert!(alert.contains("function AlertDescription"));
        assert!(alert.contains(r#"data-slot="alert""#));
        assert!(alert.contains("data-variant={variant}"));
        assert!(alert.contains(r#"role="alert""#));
        assert!(alert.contains("export { Alert, AlertTitle, AlertDescription }"));
        assert!(alert.contains(r#"import { cn } from "../../lib/utils";"#));

        let registry = registry_package("shadcn/ui/alert").expect("registry package");
        assert_eq!(
            registry.provenance.upstream_reference.as_deref(),
            Some("shadcn-ui://apps/v4/registry/bases/radix/ui/alert.tsx")
        );
    }

    #[test]
    fn ui_avatar_alias_materializes_real_upstream_avatar_component() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("ui/avatar"), "shadcn/ui/avatar");

        let package = source_package_for_project("ui/avatar", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        let avatar = package
            .files
            .iter()
            .find(|file| file.path == "components/ui/avatar.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("avatar component");

        assert_eq!(package.package_id, "shadcn/ui/avatar");
        assert_eq!(package.upstream_name, "shadcn-ui");
        assert!(paths.contains(&"components/ui/avatar.tsx"));
        assert!(paths.contains(&"components/ui/README.md"));
        assert!(paths.contains(&"lib/utils.ts"));
        assert!(avatar.contains("function Avatar"));
        assert!(avatar.contains("function AvatarImage"));
        assert!(avatar.contains("function AvatarFallback"));
        assert!(avatar.contains(r#"data-slot="avatar""#));
        assert!(avatar.contains(r#"data-slot="avatar-image""#));
        assert!(avatar.contains(r#"data-slot="avatar-fallback""#));
        assert!(avatar.contains("export { Avatar, AvatarImage, AvatarFallback }"));
        assert!(avatar.contains(r#"import { cn } from "../../lib/utils";"#));

        let registry = registry_package("shadcn/ui/avatar").expect("registry package");
        assert_eq!(
            registry.provenance.upstream_reference.as_deref(),
            Some("shadcn-ui://apps/v4/registry/bases/radix/ui/avatar.tsx")
        );
    }

    #[test]
    fn ui_skeleton_alias_materializes_real_upstream_skeleton_component() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("ui/skeleton"), "shadcn/ui/skeleton");

        let package = source_package_for_project("ui/skeleton", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        let skeleton = package
            .files
            .iter()
            .find(|file| file.path == "components/ui/skeleton.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("skeleton component");

        assert_eq!(package.package_id, "shadcn/ui/skeleton");
        assert_eq!(package.upstream_name, "shadcn-ui");
        assert!(paths.contains(&"components/ui/skeleton.tsx"));
        assert!(paths.contains(&"components/ui/README.md"));
        assert!(paths.contains(&"lib/utils.ts"));
        assert!(skeleton.contains("function Skeleton"));
        assert!(skeleton.contains(r#"data-slot="skeleton""#));
        assert!(skeleton.contains("cn-skeleton animate-pulse"));
        assert!(skeleton.contains("export { Skeleton }"));
        assert!(skeleton.contains(r#"import { cn } from "../../lib/utils";"#));

        let registry = registry_package("shadcn/ui/skeleton").expect("registry package");
        assert_eq!(
            registry.provenance.upstream_reference.as_deref(),
            Some("shadcn-ui://apps/v4/registry/bases/radix/ui/skeleton.tsx")
        );
    }

    #[test]
    fn ui_label_alias_materializes_real_upstream_label_component() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("ui/label"), "shadcn/ui/label");

        let package = source_package_for_project("ui/label", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        let label = package
            .files
            .iter()
            .find(|file| file.path == "components/ui/label.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("label component");

        assert_eq!(package.package_id, "shadcn/ui/label");
        assert_eq!(package.upstream_name, "shadcn-ui");
        assert!(paths.contains(&"components/ui/label.tsx"));
        assert!(paths.contains(&"components/ui/README.md"));
        assert!(paths.contains(&"lib/utils.ts"));
        assert!(label.contains("function Label"));
        assert!(label.contains(r#"data-slot="label""#));
        assert!(label.contains("cn-label flex items-center select-none"));
        assert!(label.contains(r#"React.ComponentProps<"label">"#));
        assert!(label.contains("export { Label }"));

        let registry = registry_package("shadcn/ui/label").expect("registry package");
        assert_eq!(
            registry.provenance.upstream_reference.as_deref(),
            Some("shadcn-ui://apps/v4/registry/bases/radix/ui/label.tsx")
        );
    }

    #[test]
    fn ui_separator_alias_materializes_real_upstream_separator_component() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("ui/separator"), "shadcn/ui/separator");

        let package = source_package_for_project("ui/separator", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        let separator = package
            .files
            .iter()
            .find(|file| file.path == "components/ui/separator.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("separator component");

        assert_eq!(package.package_id, "shadcn/ui/separator");
        assert_eq!(package.upstream_name, "shadcn-ui");
        assert!(paths.contains(&"components/ui/separator.tsx"));
        assert!(paths.contains(&"components/ui/README.md"));
        assert!(paths.contains(&"lib/utils.ts"));
        assert!(separator.contains("function Separator"));
        assert!(separator.contains(r#"data-slot="separator""#));
        assert!(separator.contains(r#"orientation = "horizontal""#));
        assert!(separator.contains("decorative = true"));
        assert!(separator.contains("data-orientation={orientation}"));
        assert!(separator.contains("data-horizontal:h-px data-horizontal:w-full"));
        assert!(separator.contains("export { Separator }"));

        let registry = registry_package("shadcn/ui/separator").expect("registry package");
        assert_eq!(
            registry.provenance.upstream_reference.as_deref(),
            Some("shadcn-ui://apps/v4/registry/bases/radix/ui/separator.tsx")
        );
    }

    #[test]
    fn ui_field_alias_materializes_real_upstream_field_component() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("ui/field"), "shadcn/ui/field");

        let package = source_package_for_project("ui/field", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        let field = package
            .files
            .iter()
            .find(|file| file.path == "components/ui/field.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("field component");

        assert_eq!(package.package_id, "shadcn/ui/field");
        assert_eq!(package.upstream_name, "shadcn-ui");
        assert!(paths.contains(&"components/ui/field.tsx"));
        assert!(paths.contains(&"components/ui/label.tsx"));
        assert!(paths.contains(&"components/ui/separator.tsx"));
        assert!(paths.contains(&"components/ui/README.md"));
        assert!(paths.contains(&"lib/utils.ts"));
        assert!(field.contains("function FieldSet"));
        assert!(field.contains("function FieldLegend"));
        assert!(field.contains("function FieldGroup"));
        assert!(field.contains("fieldVariants"));
        assert!(field.contains(r#"data-slot="field""#));
        assert!(field.contains("data-orientation={orientation}"));
        assert!(field.contains("function FieldLabel"));
        assert!(field.contains("function FieldDescription"));
        assert!(field.contains("function FieldSeparator"));
        assert!(field.contains("function FieldError"));
        assert!(field.contains("uniqueErrors"));
        assert!(field.contains("FieldError,"));

        let registry = registry_package("shadcn/ui/field").expect("registry package");
        assert_eq!(
            registry.provenance.upstream_reference.as_deref(),
            Some("shadcn-ui://apps/v4/registry/bases/radix/ui/field.tsx")
        );
    }

    #[test]
    fn ui_item_alias_materializes_real_upstream_item_component() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("ui/item"), "shadcn/ui/item");

        let package = source_package_for_project("ui/item", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        let item = package
            .files
            .iter()
            .find(|file| file.path == "components/ui/item.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("item component");

        assert_eq!(package.package_id, "shadcn/ui/item");
        assert_eq!(package.upstream_name, "shadcn-ui");
        assert!(paths.contains(&"components/ui/item.tsx"));
        assert!(paths.contains(&"components/ui/slot.tsx"));
        assert!(paths.contains(&"components/ui/separator.tsx"));
        assert!(paths.contains(&"components/ui/README.md"));
        assert!(paths.contains(&"lib/utils.ts"));
        assert!(item.contains("function ItemGroup"));
        assert!(item.contains("function ItemSeparator"));
        assert!(item.contains("itemVariants"));
        assert!(item.contains(r#"data-slot="item""#));
        assert!(item.contains("data-variant={variant}"));
        assert!(item.contains("data-size={size}"));
        assert!(item.contains(r#"asChild ? Slot.Root : "div""#));
        assert!(item.contains("function ItemMedia"));
        assert!(item.contains("function ItemContent"));
        assert!(item.contains("function ItemActions"));
        assert!(item.contains("function ItemHeader"));
        assert!(item.contains("function ItemFooter"));
        assert!(item.contains("ItemFooter,"));

        let registry = registry_package("shadcn/ui/item").expect("registry package");
        assert_eq!(
            registry.provenance.upstream_reference.as_deref(),
            Some("shadcn-ui://apps/v4/registry/bases/radix/ui/item.tsx")
        );
    }

    #[test]
    fn zustand_alias_materializes_real_launch_state_slice() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("zustand"), "state/zustand");

        let package = source_package_for_project("zustand", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        let react = package
            .files
            .iter()
            .find(|file| file.path == "lib/forge/state/zustand/react.ts")
            .and_then(|file| file.content.as_deref())
            .expect("react source");
        let persist = package
            .files
            .iter()
            .find(|file| file.path == "lib/forge/state/zustand/persist.ts")
            .and_then(|file| file.content.as_deref())
            .expect("persist source");

        assert_eq!(package.package_id, "state/zustand");
        assert_eq!(package.upstream_name, "zustand");
        assert_eq!(package.version, "5.0.13-dx.0");
        assert!(paths.contains(&"lib/forge/state/zustand/index.ts"));
        assert!(paths.contains(&"lib/forge/state/zustand/vanilla.ts"));
        assert!(paths.contains(&"lib/forge/state/zustand/react.ts"));
        assert!(paths.contains(&"lib/forge/state/zustand/middleware.ts"));
        assert!(paths.contains(&"lib/forge/state/zustand/shallow.ts"));
        assert!(paths.contains(&"lib/forge/state/zustand/persist.ts"));
        assert!(paths.contains(&"lib/forge/state/zustand/README.md"));
        assert!(react.contains("useSyncExternalStore"));
        assert!(react.contains("Object.assign(useBoundStore, api)"));
        assert!(persist.contains("createJSONStorage"));
        assert!(paths.iter().all(|path| !path.contains("node_modules")));
    }

    #[test]
    fn ui_input_alias_materializes_real_upstream_input_component() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("ui/input"), "shadcn/ui/input");

        let package = source_package_for_project("ui/input", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        let input = package
            .files
            .iter()
            .find(|file| file.path == "components/ui/input.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("input component");

        assert_eq!(package.package_id, "shadcn/ui/input");
        assert_eq!(package.upstream_name, "shadcn-ui");
        assert!(paths.contains(&"components/ui/input.tsx"));
        assert!(paths.contains(&"lib/utils.ts"));
        assert!(input.contains(r#"data-slot="input""#));
        assert!(input.contains("cn-input w-full min-w-0 outline-none"));
        assert!(input.contains(r#"import { cn } from "../../lib/utils";"#));
    }

    #[test]
    fn ui_textarea_alias_materializes_real_upstream_textarea_component() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("ui/textarea"), "shadcn/ui/textarea");

        let package = source_package_for_project("ui/textarea", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        let textarea = package
            .files
            .iter()
            .find(|file| file.path == "components/ui/textarea.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("textarea component");

        assert_eq!(package.package_id, "shadcn/ui/textarea");
        assert_eq!(package.upstream_name, "shadcn-ui");
        assert!(paths.contains(&"components/ui/textarea.tsx"));
        assert!(paths.contains(&"lib/utils.ts"));
        assert!(textarea.contains(r#"data-slot="textarea""#));
        assert!(
            textarea.contains("cn-textarea flex field-sizing-content min-h-16 w-full outline-none")
        );
        assert!(textarea.contains(r#"import { cn } from "../../lib/utils";"#));
    }

    #[test]
    fn icon_search_alias_resolves_to_selected_icon_package() {
        assert_eq!(canonical_package_id("icon/search"), "dx/icon/search");
        let package = default_source_package("icon/search").expect("package");
        assert_eq!(package.package_id, "dx/icon/search");
        assert_eq!(package.upstream_name, "@dx/forge-icons");
    }

    #[test]
    fn authentication_google_provider_aliases_resolve_to_authentication() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("google/auth"), "auth/better-auth");
        assert_eq!(canonical_package_id("google-oauth"), "auth/better-auth");

        let package = source_package_for_project("google-oauth", dir.path()).expect("package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();

        assert_eq!(package.package_id, "auth/better-auth");
        assert_eq!(package.upstream_name, "better-auth");
        assert!(
            paths
                .iter()
                .any(|path| path.starts_with("auth/better-auth/"))
        );
    }

    #[test]
    fn drizzle_sqlite_alias_materializes_real_drizzle_api_slice() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("drizzle/sqlite"), "db/drizzle-sqlite");
        assert_eq!(canonical_package_id("drizzle"), "db/drizzle-sqlite");

        let package =
            source_package_for_project("drizzle/sqlite", dir.path()).expect("drizzle package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        let schema = package
            .files
            .iter()
            .find(|file| file.path == "db/drizzle/schema.ts")
            .and_then(|file| file.content.as_deref())
            .expect("schema source");
        let queries = package
            .files
            .iter()
            .find(|file| file.path == "db/drizzle/queries.ts")
            .and_then(|file| file.content.as_deref())
            .expect("query source");
        let readme = package
            .files
            .iter()
            .find(|file| file.path == "db/drizzle/README.md")
            .and_then(|file| file.content.as_deref())
            .expect("readme source");

        assert_eq!(package.package_id, "db/drizzle-sqlite");
        assert_eq!(package.upstream_name, "drizzle-orm");
        assert!(paths.contains(&"db/drizzle/client.ts"));
        assert!(paths.contains(&"db/drizzle/schema.ts"));
        assert!(paths.contains(&"db/drizzle/queries.ts"));
        assert!(paths.contains(&"db/drizzle/metadata.ts"));
        assert!(paths.contains(&"db/drizzle/README.md"));
        assert_eq!(paths.len(), 5);
        assert!(schema.contains(r#"from "drizzle-orm/sqlite-core";"#));
        assert!(schema.contains(r#"from "drizzle-orm";"#));
        assert!(schema.contains("sqliteTable"));
        assert!(schema.contains("relations"));
        assert!(schema.contains("InferInsertModel"));
        assert!(schema.contains("InferSelectModel"));
        assert!(queries.contains("eq("));
        assert!(queries.contains("and("));
        assert!(queries.contains("sql<number>"));
        assert!(readme.contains("metadata.ts"));
        assert!(readme.contains("drizzle-orm 0.45.3"));
    }

    #[test]
    fn drizzle_sqlite_materializes_under_custom_db_path() {
        let dir = tempdir().expect("tempdir");
        fs::write(dir.path().join("dx"), "forge.paths.js_db=src/server/db\n")
            .expect("write config");

        let package =
            source_package_for_project("db/drizzle", dir.path()).expect("drizzle package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();

        assert!(paths.contains(&"src/server/db/drizzle/client.ts"));
        assert!(paths.contains(&"src/server/db/drizzle/schema.ts"));
        assert!(paths.contains(&"src/server/db/drizzle/queries.ts"));
        assert!(paths.contains(&"src/server/db/drizzle/metadata.ts"));
        assert!(paths.contains(&"src/server/db/drizzle/README.md"));
    }

    #[test]
    fn supabase_client_materializes_ssr_auth_starter() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("db/supabase"), "supabase/client");
        assert_eq!(canonical_package_id("supabase/ssr"), "supabase/client");

        let package =
            source_package_for_project("supabase/client", dir.path()).expect("supabase package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        let server = package
            .files
            .iter()
            .find(|file| file.path == "lib/supabase/server.ts")
            .and_then(|file| file.content.as_deref())
            .expect("server client");
        let env = package
            .files
            .iter()
            .find(|file| file.path == "lib/supabase/.env.example")
            .and_then(|file| file.content.as_deref())
            .expect("env example");

        assert_eq!(package.package_id, "supabase/client");
        assert_eq!(package.upstream_name, "@dx/forge-supabase");
        assert!(paths.contains(&"lib/supabase/browser.ts"));
        assert!(paths.contains(&"lib/supabase/env.ts"));
        assert!(paths.contains(&"lib/supabase/server.ts"));
        assert!(paths.contains(&"lib/supabase/auth-actions.ts"));
        assert!(paths.contains(&"lib/supabase/metadata.ts"));
        assert!(paths.contains(&"lib/supabase/schema.sql"));
        assert!(paths.contains(&"lib/supabase/.env.example"));
        assert!(paths.contains(&"lib/supabase/README.md"));
        assert!(server.contains("createServerClient"));
        assert!(server.contains("NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY"));
        assert!(env.contains("NEXT_PUBLIC_SUPABASE_URL"));
        assert!(env.contains("NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY"));
        assert!(!env.contains("SUPABASE_SERVICE_ROLE_KEY"));
        assert_eq!(paths.len(), 8);
    }

    #[test]
    fn trpc_next_alias_materializes_typed_app_router_slice() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("trpc"), "api/trpc");
        assert_eq!(canonical_package_id("@trpc/server"), "api/trpc");

        let package = source_package_for_project("trpc", dir.path()).expect("trpc package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        let server = package
            .files
            .iter()
            .find(|file| file.path == "lib/trpc/server.ts")
            .and_then(|file| file.content.as_deref())
            .expect("server source");
        let errors = package
            .files
            .iter()
            .find(|file| file.path == "lib/trpc/errors.ts")
            .and_then(|file| file.content.as_deref())
            .expect("errors source");
        let transformer = package
            .files
            .iter()
            .find(|file| file.path == "lib/trpc/transformer.ts")
            .and_then(|file| file.content.as_deref())
            .expect("transformer source");
        let response_meta = package
            .files
            .iter()
            .find(|file| file.path == "lib/trpc/response-meta.ts")
            .and_then(|file| file.content.as_deref())
            .expect("response metadata source");
        let router_source = package
            .files
            .iter()
            .find(|file| file.path == "lib/trpc/router.ts")
            .and_then(|file| file.content.as_deref())
            .expect("router source");
        let route = package
            .files
            .iter()
            .find(|file| file.path == "app/api/trpc/[trpc]/route.ts")
            .and_then(|file| file.content.as_deref())
            .expect("app route source");
        let http = package
            .files
            .iter()
            .find(|file| file.path == "lib/trpc/http.ts")
            .and_then(|file| file.content.as_deref())
            .expect("http source");
        let client = package
            .files
            .iter()
            .find(|file| file.path == "lib/trpc/client.ts")
            .and_then(|file| file.content.as_deref())
            .expect("client source");
        let server_caller = package
            .files
            .iter()
            .find(|file| file.path == "lib/trpc/server-caller.ts")
            .and_then(|file| file.content.as_deref())
            .expect("server caller source");
        let subscriptions = package
            .files
            .iter()
            .find(|file| file.path == "lib/trpc/subscriptions.ts")
            .and_then(|file| file.content.as_deref())
            .expect("subscription source");
        let streaming_client = package
            .files
            .iter()
            .find(|file| file.path == "lib/trpc/streaming-client.ts")
            .and_then(|file| file.content.as_deref())
            .expect("streaming client source");
        let provider = package
            .files
            .iter()
            .find(|file| file.path == "lib/trpc/provider.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("provider source");
        let launch_example = package
            .files
            .iter()
            .find(|file| file.path == "examples/template/trpc-launch-health.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("launch example source");
        let launch_server_example = package
            .files
            .iter()
            .find(|file| file.path == "examples/template/trpc-server-readiness.ts")
            .and_then(|file| file.content.as_deref())
            .expect("server launch example source");
        let launch_subscription_example = package
            .files
            .iter()
            .find(|file| file.path == "examples/template/trpc-subscription-status.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("subscription launch example source");
        let launch_error_example = package
            .files
            .iter()
            .find(|file| file.path == "examples/template/trpc-error-status.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("error launch example source");
        let launch_streaming_example = package
            .files
            .iter()
            .find(|file| file.path == "examples/template/trpc-streaming-client-status.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("streaming launch example source");
        let launch_response_meta_example = package
            .files
            .iter()
            .find(|file| file.path == "examples/template/trpc-response-meta.ts")
            .and_then(|file| file.content.as_deref())
            .expect("response metadata launch example source");
        let launch_infinite_feed_example = package
            .files
            .iter()
            .find(|file| file.path == "examples/template/trpc-infinite-feed.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("infinite feed launch example source");
        let launch_transformer_example = package
            .files
            .iter()
            .find(|file| file.path == "examples/template/trpc-transformer-status.ts")
            .and_then(|file| file.content.as_deref())
            .expect("transformer launch example source");
        let launch_request_policy_example = package
            .files
            .iter()
            .find(|file| file.path == "examples/template/trpc-request-policy.ts")
            .and_then(|file| file.content.as_deref())
            .expect("request policy launch example source");
        let metadata = package
            .files
            .iter()
            .find(|file| file.path == "lib/trpc/metadata.ts")
            .and_then(|file| file.content.as_deref())
            .expect("metadata source");

        assert_eq!(package.package_id, "api/trpc");
        assert_eq!(package.upstream_name, "@trpc/server");
        assert_eq!(package.version, "11.17.0-dx.10");
        assert!(paths.contains(&"lib/trpc/context.ts"));
        assert!(paths.contains(&"lib/trpc/transformer.ts"));
        assert!(paths.contains(&"lib/trpc/server.ts"));
        assert!(paths.contains(&"lib/trpc/errors.ts"));
        assert!(paths.contains(&"lib/trpc/response-meta.ts"));
        assert!(paths.contains(&"lib/trpc/router.ts"));
        assert!(paths.contains(&"lib/trpc/route-handler.ts"));
        assert!(paths.contains(&"lib/trpc/server-caller.ts"));
        assert!(paths.contains(&"app/api/trpc/[trpc]/route.ts"));
        assert!(paths.contains(&"lib/trpc/http.ts"));
        assert!(paths.contains(&"lib/trpc/client.ts"));
        assert!(paths.contains(&"lib/trpc/subscriptions.ts"));
        assert!(paths.contains(&"lib/trpc/streaming-client.ts"));
        assert!(paths.contains(&"lib/trpc/provider.tsx"));
        assert!(paths.contains(&"lib/trpc/dashboard-workflow.ts"));
        assert!(paths.contains(&"components/dashboard/trpc-dashboard-workflow.tsx"));
        assert!(paths.contains(&"lib/trpc/metadata.ts"));
        assert!(paths.contains(&"lib/trpc/README.md"));
        assert!(paths.contains(&"examples/template/trpc-launch-contract.ts"));
        assert!(paths.contains(&"examples/template/trpc-launch-health.tsx"));
        assert!(paths.contains(&"examples/template/trpc-server-readiness.ts"));
        assert!(paths.contains(&"examples/template/trpc-subscription-status.tsx"));
        assert!(paths.contains(&"examples/template/trpc-error-status.tsx"));
        assert!(paths.contains(&"examples/template/trpc-streaming-client-status.tsx"));
        assert!(paths.contains(&"examples/template/trpc-response-meta.ts"));
        assert!(paths.contains(&"examples/template/trpc-infinite-feed.tsx"));
        assert!(paths.contains(&"examples/template/trpc-transformer-status.ts"));
        assert!(paths.contains(&"examples/template/trpc-request-policy.ts"));
        assert!(server.contains(r#"import { initTRPC, TRPCError } from "@trpc/server";"#));
        assert!(server.contains("errorFormatter: formatDxTrpcError"));
        assert!(server.contains("transformer: dxTrpcTransformer"));
        assert!(server.contains("createCallerFactory"));
        assert!(transformer.contains("TRPCCombinedDataTransformer"));
        assert!(transformer.contains("TRPCDataTransformer"));
        assert!(transformer.contains("createDxTrpcTransformer"));
        assert!(transformer.contains("dxTrpcIdentityDataTransformer"));
        assert!(transformer.contains("dxTrpcTransformerPolicy"));
        assert!(errors.contains("TRPCErrorFormatter"));
        assert!(errors.contains("TRPCErrorShape"));
        assert!(errors.contains("getHTTPStatusCodeFromError"));
        assert!(errors.contains("createDxTrpcError"));
        assert!(response_meta.contains("ResponseMetaFn"));
        assert!(response_meta.contains("createDxTrpcResponseMeta"));
        assert!(response_meta.contains("dxTrpcPublicCacheResponseMeta"));
        assert!(response_meta.contains("dxTrpcNoStoreResponseMeta"));
        assert!(router_source.contains("launchEvents: publicProcedure"));
        assert!(router_source.contains("cursor: z.number().int().min(0).default(0)"));
        assert!(router_source.contains("nextCursor"));
        assert!(
            router_source
                .contains(r#"export type LaunchEventsInput = AppRouterInputs["launchEvents"];"#)
        );
        assert!(route.contains(r#"from "../../../../lib/trpc/route-handler";"#));
        assert!(route.contains("responseMeta: options.responseMeta ?? createDxTrpcResponseMeta()"));
        assert!(http.contains("HTTPBatchLinkOptions"));
        assert!(http.contains("HTTPHeaders"));
        assert!(http.contains("TRPCFetch"));
        assert!(http.contains("createDxTrpcRequestHeaders"));
        assert!(http.contains("createDxTrpcHttpLinkOptions"));
        assert!(http.contains("maxItems: options.maxItems ?? dxTrpcHttpBatchPolicy.maxItems"));
        assert!(
            http.contains(
                "maxURLLength: options.maxURLLength ?? dxTrpcHttpBatchPolicy.maxURLLength"
            )
        );
        assert!(http.contains("methodOverride: options.methodOverride"));
        assert!(client.contains("...createDxTrpcHttpLinkOptions(options)"));
        assert!(server_caller.contains("createDxTrpcServerCaller"));
        assert!(server_caller.contains("readDxTrpcLaunchReadiness"));
        assert!(server_caller.contains("createCaller(await createDxTrpcContext({"));
        assert!(subscriptions.contains("httpSubscriptionLink"));
        assert!(subscriptions.contains("splitLink"));
        assert!(subscriptions.contains("createDxTrpcSubscriptionClient"));
        assert!(subscriptions.contains("transformer: options.transformer ?? dxTrpcTransformer"));
        assert!(streaming_client.contains("httpBatchStreamLink"));
        assert!(streaming_client.contains("loggerLink"));
        assert!(streaming_client.contains("createDxTrpcStreamingClient"));
        assert!(streaming_client.contains("createDxTrpcLoggerLink"));
        assert!(streaming_client.contains("transformer: options.transformer ?? dxTrpcTransformer"));
        assert!(
            provider.contains(r#"import { createTRPCContext } from "@trpc/tanstack-react-query";"#)
        );
        assert!(provider.contains("enableSubscriptions"));
        assert!(provider.contains(r#"transport?: "batch" | "stream" | "subscription";"#));
        assert!(provider.contains("transport === \"stream\""));
        assert!(provider.contains("QueryClientProvider"));
        assert!(launch_example.contains(r#"import * as React from "react";"#));
        assert!(launch_example.contains(r#"data-dx-package="api/trpc""#));
        assert!(launch_example.contains(r#"data-dx-component="trpc-launch-health-workflow""#));
        assert!(launch_example.contains(r#"data-trpc-workflow="template-visible""#));
        assert!(launch_example.contains(r#"data-trpc-interaction="local-launch-event-mutation""#));
        assert!(launch_example.contains(r#"data-trpc-node-modules="not-required-for-workflow""#));
        assert!(launch_example.contains("setResult(createLocalLaunchEvent(nextSequence))"));
        assert!(launch_example.contains("trpc.health.queryOptions()"));
        assert!(launch_example.contains("trpc.launchEvent.mutationOptions"));
        assert!(launch_example.contains("trpc.health.queryFilter()"));
        assert!(!launch_example.contains(r#"from "@trpc/client""#));
        assert!(!launch_example.contains(r#"from "@tanstack/react-query""#));
        assert!(!launch_example.contains(r#"from "@/lib/trpc/provider""#));
        assert!(launch_server_example.contains("readDxTrpcLaunchReadiness"));
        assert!(launch_server_example.contains("www-template"));
        assert!(launch_subscription_example.contains("useSubscription"));
        assert!(launch_subscription_example.contains("trpc.launchFeed.subscriptionOptions"));
        assert!(launch_error_example.contains("TRPCClientError"));
        assert!(launch_error_example.contains("AppRouterError"));
        assert!(launch_error_example.contains("data-trpc-error-code"));
        assert!(launch_streaming_example.contains("DxTrpcProvider"));
        assert!(launch_streaming_example.contains("transport=\"stream\""));
        assert!(launch_streaming_example.contains("data-trpc-transport=\"httpBatchStreamLink\""));
        assert!(launch_response_meta_example.contains("createDxTrpcResponseMeta"));
        assert!(launch_response_meta_example.contains("createDxTrpcRouteHandler"));
        assert!(launch_response_meta_example.contains("publicPathPrefix: \"health\""));
        assert!(launch_infinite_feed_example.contains("useInfiniteQuery"));
        assert!(launch_infinite_feed_example.contains("trpc.launchEvents.infiniteQueryOptions"));
        assert!(launch_infinite_feed_example.contains("trpc.launchEvents.infiniteQueryKey"));
        assert!(launch_infinite_feed_example.contains("trpc.launchEvents.infiniteQueryFilter"));
        assert!(launch_infinite_feed_example.contains("data-trpc-infinite-feed=\"ready\""));
        assert!(launch_transformer_example.contains("createDxTrpcTransformer"));
        assert!(launch_transformer_example.contains("dxTrpcIdentityDataTransformer"));
        assert!(launch_transformer_example.contains("trpcTransformerReadiness"));
        assert!(launch_request_policy_example.contains("createDxTrpcRequestHeaders"));
        assert!(launch_request_policy_example.contains("createDxTrpcHttpLinkOptions"));
        assert!(launch_request_policy_example.contains("x-dx-www-template"));
        assert!(launch_request_policy_example.contains("trpcRequestPolicyReadiness"));
        assert!(metadata.contains(r#"packageId: "api/trpc""#));
        assert!(metadata.contains(r#"officialDxPackageName: "Type-Safe API""#));
        assert!(metadata.contains("@trpc/server"));
        assert!(metadata.contains("dxTrpcTransformer"));
        assert!(metadata.contains("formatDxTrpcError"));
        assert!(metadata.contains("AppRouterError"));
        assert!(metadata.contains("createDxTrpcResponseMeta"));
        assert!(metadata.contains("createDxTrpcStreamingClient"));
        assert!(metadata.contains("createDxTrpcLoggerLink"));
        assert!(metadata.contains("inferRouterInputs"));
        assert!(metadata.contains("mutationOptions"));
        assert!(metadata.contains("infiniteQueryOptions"));
        assert!(metadata.contains("subscriptionOptions"));
        assert!(metadata.contains("createDxTrpcHttpLinkOptions"));
        assert!(metadata.contains("createCallerFactory server callers"));
        assert!(metadata.contains("fetchRequestHandler"));
        assert_eq!(paths.len(), 28);
    }

    #[test]
    fn instantdb_react_alias_materializes_realtime_launch_slice() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(canonical_package_id("@instantdb/react"), "instantdb/react");
        assert_eq!(canonical_package_id("instantdb"), "instantdb/react");

        let package =
            source_package_for_project("@instantdb/react", dir.path()).expect("instantdb package");
        let paths = package
            .files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>();
        let client = package
            .files
            .iter()
            .find(|file| file.path == "lib/instant/client.ts")
            .and_then(|file| file.content.as_deref())
            .expect("instant client");
        let schema = package
            .files
            .iter()
            .find(|file| file.path == "lib/instant/schema.ts")
            .and_then(|file| file.content.as_deref())
            .expect("instant schema");
        let next_client = package
            .files
            .iter()
            .find(|file| file.path == "lib/instant/next-client.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("instant next client");
        let next_server = package
            .files
            .iter()
            .find(|file| file.path == "lib/instant/next-server.ts")
            .and_then(|file| file.content.as_deref())
            .expect("instant next server");
        let queries = package
            .files
            .iter()
            .find(|file| file.path == "lib/instant/queries.ts")
            .and_then(|file| file.content.as_deref())
            .expect("instant queries");
        let status = package
            .files
            .iter()
            .find(|file| file.path == "lib/instant/status.ts")
            .and_then(|file| file.content.as_deref())
            .expect("instant status");
        let subscriptions = package
            .files
            .iter()
            .find(|file| file.path == "lib/instant/subscriptions.ts")
            .and_then(|file| file.content.as_deref())
            .expect("instant subscriptions");
        let pagination = package
            .files
            .iter()
            .find(|file| file.path == "lib/instant/pagination.ts")
            .and_then(|file| file.content.as_deref())
            .expect("instant pagination");
        let diagnostics = package
            .files
            .iter()
            .find(|file| file.path == "lib/instant/diagnostics.ts")
            .and_then(|file| file.content.as_deref())
            .expect("instant diagnostics");
        let env = package
            .files
            .iter()
            .find(|file| file.path == "lib/instant/env.ts")
            .and_then(|file| file.content.as_deref())
            .expect("instant env");
        let todos = package
            .files
            .iter()
            .find(|file| file.path == "components/instant/instant-todos.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("instant todos component");
        let cursors = package
            .files
            .iter()
            .find(|file| file.path == "components/instant/instant-cursors.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("instant cursors component");
        let auth_boundary = package
            .files
            .iter()
            .find(|file| file.path == "components/instant/instant-auth-boundary.tsx")
            .and_then(|file| file.content.as_deref())
            .expect("instant auth boundary component");
        let mutations = package
            .files
            .iter()
            .find(|file| file.path == "lib/instant/mutations.ts")
            .and_then(|file| file.content.as_deref())
            .expect("instant mutations");
        let rules = package
            .files
            .iter()
            .find(|file| file.path == "lib/instant/rules.ts")
            .and_then(|file| file.content.as_deref())
            .expect("instant rules");
        let perms = package
            .files
            .iter()
            .find(|file| file.path == "lib/instant/perms.ts")
            .and_then(|file| file.content.as_deref())
            .expect("instant perms");
        let auth = package
            .files
            .iter()
            .find(|file| file.path == "lib/instant/auth.ts")
            .and_then(|file| file.content.as_deref())
            .expect("instant auth");
        let oauth = package
            .files
            .iter()
            .find(|file| file.path == "lib/instant/oauth.ts")
            .and_then(|file| file.content.as_deref())
            .expect("instant oauth");
        let storage = package
            .files
            .iter()
            .find(|file| file.path == "lib/instant/storage.ts")
            .and_then(|file| file.content.as_deref())
            .expect("instant storage");
        let streams = package
            .files
            .iter()
            .find(|file| file.path == "lib/instant/streams.ts")
            .and_then(|file| file.content.as_deref())
            .expect("instant streams");
        let sync_table = package
            .files
            .iter()
            .find(|file| file.path == "lib/instant/sync-table.ts")
            .and_then(|file| file.content.as_deref())
            .expect("instant sync table");
        let route = package
            .files
            .iter()
            .find(|file| file.path == "lib/instant/route.ts")
            .and_then(|file| file.content.as_deref())
            .expect("instant route");
        let api_route = package
            .files
            .iter()
            .find(|file| file.path == "app/api/instant/route.ts")
            .and_then(|file| file.content.as_deref())
            .expect("instant api route");
        let metadata = package
            .files
            .iter()
            .find(|file| file.path == "lib/instant/metadata.ts")
            .and_then(|file| file.content.as_deref())
            .expect("metadata");

        assert_eq!(package.package_id, "instantdb/react");
        assert_eq!(package.upstream_name, "@instantdb/react");
        assert_eq!(package.version, "0.0.0-dx.0");
        assert!(paths.contains(&"lib/instant/env.ts"));
        assert!(paths.contains(&"lib/instant/schema.ts"));
        assert!(paths.contains(&"lib/instant/client.ts"));
        assert!(paths.contains(&"lib/instant/next-client.tsx"));
        assert!(paths.contains(&"lib/instant/next-server.ts"));
        assert!(paths.contains(&"lib/instant/queries.ts"));
        assert!(paths.contains(&"lib/instant/status.ts"));
        assert!(paths.contains(&"lib/instant/subscriptions.ts"));
        assert!(paths.contains(&"lib/instant/pagination.ts"));
        assert!(paths.contains(&"lib/instant/diagnostics.ts"));
        assert!(paths.contains(&"lib/instant/mutations.ts"));
        assert!(paths.contains(&"lib/instant/rules.ts"));
        assert!(paths.contains(&"lib/instant/perms.ts"));
        assert!(paths.contains(&"lib/instant/auth.ts"));
        assert!(paths.contains(&"lib/instant/oauth.ts"));
        assert!(paths.contains(&"lib/instant/storage.ts"));
        assert!(paths.contains(&"lib/instant/streams.ts"));
        assert!(paths.contains(&"lib/instant/sync-table.ts"));
        assert!(paths.contains(&"lib/instant/route.ts"));
        assert!(paths.contains(&"lib/instant/metadata.ts"));
        assert!(paths.contains(&"components/instant/instant-todos.tsx"));
        assert!(paths.contains(&"components/instant/instant-cursors.tsx"));
        assert!(paths.contains(&"components/instant/instant-auth-boundary.tsx"));
        assert!(paths.contains(&"app/api/instant/route.ts"));
        assert!(paths.contains(&"app/instant/page.tsx"));
        assert!(paths.contains(&"lib/instant/README.md"));
        assert_eq!(paths.len(), 26);
        assert!(client.contains(r#"import { init } from "@instantdb/react";"#));
        assert!(client.contains("readInstantConfig"));
        assert!(schema.contains("labels: i.entity"));
        assert!(schema.contains("details: i.json().optional()"));
        assert!(schema.contains("todoLabels:"));
        assert!(schema.contains(r#"label: "labels""#));
        assert!(next_client.contains(
            r#"import { init, InstantSuspenseProvider } from "@instantdb/react/nextjs";"#
        ));
        assert!(next_client.contains("createDxInstantNextClient"));
        assert!(next_client.contains("InstantLaunchSuspenseProvider"));
        assert!(next_client.contains("nextDb.useSuspenseQuery"));
        assert!(next_server.contains("getUnverifiedUserFromInstantCookie"));
        assert!(next_server.contains("getInstantLaunchSsrUser"));
        assert!(queries.contains("queryInstantLaunchTodosSnapshot"));
        assert!(queries.contains("db.queryOnce"));
        assert!(queries.contains("db.useLocalId"));
        assert!(queries.contains("labels: {}"));
        assert!(status.contains("useInstantLaunchConnectionStatus"));
        assert!(status.contains("db.useConnectionStatus"));
        assert!(status.contains("db.getLocalId"));
        assert!(subscriptions.contains("subscribeInstantLaunchTodos"));
        assert!(subscriptions.contains("db.core.subscribeQuery"));
        assert!(subscriptions.contains("db.core.subscribeAuth"));
        assert!(pagination.contains("useInstantLaunchTodosInfinite"));
        assert!(pagination.contains("db.useInfiniteQuery"));
        assert!(pagination.contains("db.core.subscribeInfiniteQuery"));
        assert!(diagnostics.contains("InstantAPIError"));
        assert!(diagnostics.contains("setInstantWarningsEnabled"));
        assert!(diagnostics.contains("formatInstantLaunchError"));
        assert!(env.contains("NEXT_PUBLIC_INSTANT_APP_ID"));
        assert!(env.contains("NEXT_PUBLIC_INSTANT_API_URI"));
        assert!(env.contains("NEXT_PUBLIC_INSTANT_DEVTOOL"));
        assert!(env.contains("NEXT_PUBLIC_INSTANT_DISABLE_VALIDATION"));
        assert!(env.contains("NEXT_PUBLIC_INSTANT_FIRST_PARTY_PATH"));
        assert!(env.contains("NEXT_PUBLIC_INSTANT_QUERY_CACHE_LIMIT"));
        assert!(env.contains("NEXT_PUBLIC_INSTANT_WEBSOCKET_URI"));
        assert!(env.contains("NEXT_PUBLIC_INSTANT_VERBOSE"));
        assert!(env.contains("optionalBooleanEnv"));
        assert!(env.contains("optionalNumberEnv"));
        assert!(todos.contains("db.useQuery(instantLaunchTodosQuery)"));
        assert!(todos.contains("addInstantTodo"));
        assert!(cursors.contains(r#"import { Cursors } from "@instantdb/react";"#));
        assert!(cursors.contains("InstantLaunchCursors"));
        assert!(cursors.contains(r#"spaceId = "dx-launch-cursors""#));
        assert!(todos.contains("db.rooms.useSyncPresence"));
        assert!(todos.contains("presenceName"));
        assert!(auth_boundary.contains("<db.SignedIn>"));
        assert!(auth_boundary.contains("<db.SignedOut>"));
        assert!(auth_boundary.contains("db.useUser()"));
        assert!(auth_boundary.contains("InstantLaunchUserBadge"));
        assert!(mutations.contains("db.transact"));
        assert!(mutations.contains("id()"));
        assert!(mutations.contains(".create({"));
        assert!(mutations.contains("{ upsert: false }"));
        assert!(mutations.contains("toggleAllInstantTodos"));
        assert!(mutations.contains("lookup"));
        assert!(mutations.contains("labelInstantTodoForLaunch"));
        assert!(mutations.contains(".link({"));
        assert!(mutations.contains("unlabelInstantTodoForLaunch"));
        assert!(mutations.contains(".unlink({"));
        assert!(mutations.contains("mergeInstantTodoLaunchDetails"));
        assert!(mutations.contains(".merge({"));
        assert!(mutations.contains("touchedAt: Date.now()"));
        assert!(mutations.contains("clearCompletedInstantTodos"));
        assert!(mutations.contains("db.transact(chunks)"));
        assert!(rules.contains("queryInstantLaunchTodosWithRuleParams"));
        assert!(rules.contains("db.tx.todos[todoId].ruleParams(ruleParams)"));
        assert!(rules.contains(r#"lookup("text", value)"#));
        assert!(perms.contains(r#"import type { InstantRules } from "@instantdb/react";"#));
        assert!(perms.contains("satisfies InstantRules"));
        assert!(perms.contains("todos:"));
        assert!(perms.contains("$files:"));
        assert!(auth.contains("sendInstantLaunchMagicCode"));
        assert!(auth.contains("db.auth.sendMagicCode"));
        assert!(auth.contains("db.auth.signInWithMagicCode"));
        assert!(auth.contains("getInstantLaunchAuth"));
        assert!(auth.contains("db.getAuth"));
        assert!(auth.contains("requireInstantLaunchUser"));
        assert!(oauth.contains("createInstantLaunchAuthorizationUrl"));
        assert!(oauth.contains("db.auth.createAuthorizationURL"));
        assert!(oauth.contains("db.auth.signInWithIdToken"));
        assert!(storage.contains("uploadInstantLaunchFile"));
        assert!(storage.contains("db.storage.uploadFile"));
        assert!(storage.contains("queryInstantLaunchFile"));
        assert!(storage.contains("db.queryOnce"));
        assert!(storage.contains(r#"lookup("path", path)"#));
        assert!(storage.contains("fileClient().transact"));
        assert!(!storage.contains("db.storage.delete"));
        assert!(!storage.contains("db.storage.getDownloadUrl"));
        assert!(streams.contains("createInstantLaunchWriteStream"));
        assert!(streams.contains("db.streams.createWriteStream"));
        assert!(sync_table.contains("SyncTableCallbackEventType"));
        assert!(sync_table.contains("type SyncTableCallbackEvent"));
        assert!(sync_table.contains("type StoreInterfaceStoreName"));
        assert!(sync_table.contains("subscribeInstantLaunchSyncTable"));
        assert!(sync_table.contains("db.core._syncTableExperimental"));
        assert!(sync_table.contains("summarizeInstantLaunchSyncTableEvent"));
        assert!(route.contains("createInstantRouteHandler"));
        assert!(route.contains("createDxInstantRouteHandlers"));
        assert!(
            api_route
                .contains(r#"import { createDxInstantRouteHandlers } from "@/lib/instant/route";"#)
        );
        assert!(api_route.contains("export const { GET, POST } = createDxInstantRouteHandlers();"));
        assert!(metadata.contains(r#"packageId: "instantdb/react""#));
        assert!(metadata.contains(r#"upstreamPackage: "@instantdb/react""#));
    }
