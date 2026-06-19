    use super::*;
    use sha2::{Digest, Sha256};
    use std::fs;
    use std::path::{Path, PathBuf};
    use tempfile::TempDir;

    #[test]
    fn dx_check_latest_panel_reads_zed_receipt() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let report = read_dx_check_latest_panel(dir.path());

        assert_eq!(report.schema_version, DX_WWW_CHECK_PANEL_SCHEMA_VERSION);
        assert_eq!(report.status, DxCheckLatestPanelStatus::Ready);
        assert_eq!(report.receipt_path, receipt_path);
        assert_eq!(report.summary.score_value, Some(410));
        assert_eq!(report.summary.score_max, Some(500));
        assert_eq!(report.summary.score_percent, Some(82));
        assert_eq!(report.summary.bucket_count, 5);
        assert_eq!(report.summary.warning_count, 2);
        assert_eq!(report.summary.quick_fix_count, 1);
        assert_eq!(
            report.view_model.schema_version,
            DX_WWW_CHECK_PANEL_VIEW_MODEL_SCHEMA_VERSION
        );
        assert_eq!(report.view_model.status, "ready");
        assert_eq!(report.view_model.weight_profile, DX_CHECK_WEIGHT_PROFILE);
        assert_eq!(report.view_model.scoring_config.status, "default");
        assert!(report.view_model.scoring_config.applies_to_score);
        assert_eq!(
            report
                .view_model
                .score_meter
                .as_ref()
                .expect("score meter")
                .percent,
            82
        );
        assert!(report.view_model.score_meter.as_ref().unwrap().estimated);
        assert_eq!(report.view_model.bucket_rows.len(), 1);
        assert_eq!(report.view_model.bucket_rows[0].id, "web-performance");
        assert_eq!(report.view_model.bucket_rows[0].weight, 100);
        assert_eq!(
            report.view_model.warning_rows[0].code,
            "web-lighthouse-skipped"
        );
        assert_eq!(
            report.view_model.quick_fix_rows[0].command.as_deref(),
            Some("dx check web-perf --url <url> --json")
        );
        assert_eq!(
            report.view_model.quick_fix_rows[0].risk_level,
            "receipt-write"
        );
        assert!(report.view_model.quick_fix_rows[0].writes_receipts);
        assert!(!report.view_model.quick_fix_rows[0].requires_user_approval);
        assert_eq!(report.view_model.primary_action.command, "dx check --json");
        assert_eq!(
            report
                .view_model
                .secondary_action
                .as_ref()
                .expect("secondary action")
                .command,
            "dx check score --json"
        );
        assert_eq!(report.view_model.empty_state, None);
        assert_eq!(
            report.summary.refresh_command.as_deref(),
            Some("dx check --json")
        );
        let zed = report.zed.expect("zed panel");
        assert_eq!(zed.schema_version, DX_CHECK_ZED_PANEL_SCHEMA_VERSION);
        assert_eq!(zed.weight_profile, DX_CHECK_WEIGHT_PROFILE);
        assert_eq!(zed.scoring_config.status, "default");
        assert_eq!(zed.sections[0].id, "web-performance");
        assert_eq!(zed.sections[0].weight, 100);
        assert_eq!(zed.warnings[0].code, "web-lighthouse-skipped");
        assert_eq!(
            zed.quick_fixes[0].command.as_deref(),
            Some("dx check web-perf --url <url> --json")
        );
        assert_eq!(zed.quick_fixes[0].risk_level, "receipt-write");
        assert!(zed.quick_fixes[0].writes_receipts);
        assert!(!zed.quick_fixes[0].requires_user_approval);
    }

    #[test]
    fn dx_check_latest_panel_still_reads_legacy_unversioned_zed_receipt() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(
            &receipt_path,
            sample_receipt().replace("dx.check.zed_panel.v1", "dx.check.zed_panel"),
        )
        .expect("legacy receipt");

        let report = read_dx_check_latest_panel(dir.path());

        assert_eq!(report.status, DxCheckLatestPanelStatus::Ready);
        assert_eq!(
            report.zed.expect("legacy zed panel").schema_version,
            "dx.check.zed_panel"
        );
    }

    #[test]
    fn dx_check_latest_panel_forwards_readiness_gate_status() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(
            &receipt_path,
            sample_receipt().replace(
                "\n  \"zed\": {",
                r#"
  "readiness_gate_status": {
    "schema": "dx.www.readiness.gate_status",
    "release_ready": false,
    "fastest_world_claim": false
  },
  "readiness_replay_commands": [
    "dx www readiness --json --full",
    "dx check --latest-receipt --json"
  ],
  "zed": {"#,
            ),
        )
        .expect("Readiness receipt");

        let report = read_dx_check_latest_panel(dir.path());

        assert_eq!(report.status, DxCheckLatestPanelStatus::Ready);
        let gate_status = report
            .readiness_gate_status
            .expect("Readiness gate status");
        assert_eq!(gate_status["release_ready"], false);
        assert_eq!(gate_status["fastest_world_claim"], false);
        assert!(report
            .readiness_replay_commands
            .iter()
            .any(|command| command == "dx www readiness --json --full"));
        let view_gate_status = report
            .view_model
            .readiness_gate_status
            .expect("view-model Readiness gate status");
        assert_eq!(view_gate_status["release_ready"], false);
        assert!(report
            .view_model
            .readiness_replay_commands
            .iter()
            .any(|command| command == "dx check --latest-receipt --json"));
    }

    #[test]
    fn dx_check_latest_panel_accepts_local_proof_backed_readiness_receipt() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(
            &receipt_path,
            sample_receipt()
                .replace("dx.check.receipt", "dx.check.latest.v1")
                .replace(
                    "\n  \"zed\": {",
                    r#"
  "release_ready": true,
  "fastest_world_claim": false,
  "readiness_gate_status": {
    "schema": "dx.www.readiness.gate_status",
    "release_ready": true,
    "relative_release_ready": true,
    "release_ready_scope": "local-proof-backed-www-release",
    "release_claim_allowed": true,
    "global_speed_claim_allowed": false,
    "fastest_world_claim": false,
    "score_kind": "relative-local-proof-backed-release-ready",
    "verified_from_replay_receipts": true,
    "receipt_freshness": "current"
  },
  "readiness_replay_commands": [
    "dx www readiness --json --full"
  ],
  "replay_commands": [
    "dx www agent-context --json --full",
    "dx www docs-doctor --json"
  ],
  "zed": {"#,
                ),
        )
        .expect("local proof-backed receipt");

        let report = read_dx_check_latest_panel(dir.path());

        assert_eq!(report.status, DxCheckLatestPanelStatus::Ready);
        let gate_status = report
            .readiness_gate_status
            .expect("Readiness gate status");
        assert_eq!(gate_status["release_ready"], true);
        assert_eq!(
            gate_status["score_kind"],
            "relative-local-proof-backed-release-ready"
        );
        assert!(report
            .view_model
            .readiness_replay_commands
            .iter()
            .any(|command| command == "dx www readiness --json --full"));
    }

    #[test]
    fn dx_check_latest_panel_rejects_current_v1_receipt_without_readiness_gate() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(
            &receipt_path,
            sample_receipt().replace("dx.check.receipt", "dx.check.latest.v1"),
        )
        .expect("stale current receipt");

        let report = read_dx_check_latest_panel(dir.path());

        assert_eq!(report.status, DxCheckLatestPanelStatus::Malformed);
        assert!(report.zed.is_none());
        assert!(
            report
                .last_error
                .expect("readiness gate error")
                .contains("readiness release-gate")
        );
    }

    #[test]
    fn dx_check_latest_panel_reports_missing_and_malformed_receipts() {
        let dir = TempDir::new().expect("temp dir");
        let missing = read_dx_check_latest_panel(dir.path());
        assert_eq!(missing.status, DxCheckLatestPanelStatus::Missing);
        assert!(missing.zed.is_none());
        assert!(missing.next_action.contains("dx check --json"));
        assert_eq!(missing.view_model.status, "missing");
        assert_eq!(missing.view_model.weight_profile, DX_CHECK_WEIGHT_PROFILE);
        assert_eq!(missing.view_model.scoring_config.status, "default");
        assert!(missing.view_model.score_meter.is_none());
        assert!(missing.view_model.bucket_rows.is_empty());
        assert_eq!(missing.view_model.primary_action.command, "dx check --json");
        assert!(
            missing
                .view_model
                .empty_state
                .as_deref()
                .expect("missing empty state")
                .contains("No dx-check receipt")
        );

        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, "{not-json").expect("bad receipt");

        let malformed = read_dx_check_latest_panel(dir.path());
        assert_eq!(malformed.status, DxCheckLatestPanelStatus::Malformed);
        assert!(malformed.zed.is_none());
        assert!(malformed.last_error.expect("error").contains("parse"));
        assert_eq!(malformed.view_model.status, "malformed");
        assert_eq!(malformed.view_model.weight_profile, DX_CHECK_WEIGHT_PROFILE);
        assert_eq!(malformed.view_model.scoring_config.status, "default");
        assert!(malformed.view_model.score_meter.is_none());
        assert_eq!(
            malformed.view_model.primary_action.command,
            "dx check --json"
        );
        assert!(
            malformed
                .view_model
                .empty_state
                .as_deref()
                .expect("malformed empty state")
                .contains("could not be parsed")
        );
    }

    #[test]
    fn receipt_hash_refresh_counts_rejects_public_v1_schema_suffix() {
        let legacy_suffix_schema = format!("{}.{}", "dx.forge.package.receipt_hash_refresh", "v1");
        let stable_package = serde_json::json!({
            "receipt_hash_refresh": {
                "schema": "dx.forge.package.receipt_hash_refresh",
                "status": "current",
            },
        });
        let legacy_suffix_package = serde_json::json!({
            "receipt_hash_refresh": {
                "schema": legacy_suffix_schema,
                "status": "current",
            },
        });

        assert_eq!(receipt_hash_refresh_counts(&stable_package), (1, 0, 0));
        assert_eq!(
            receipt_hash_refresh_counts(&legacy_suffix_package),
            (0, 0, 1)
        );
    }

    #[test]
    fn dx_check_latest_panel_exposes_authentication_package_lane_hash_refresh_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let template_shell_path = dir.path().join("examples/template/template-shell.tsx");
        fs::create_dir_all(template_shell_path.parent().expect("launch shell parent"))
            .expect("launch shell dir");
        fs::write(
            &template_shell_path,
            "export const authenticationAccountWorkflowProbe = 'fresh';\n",
        )
        .expect("Authentication account workflow source");
        let template_shell_hash = sha256_file(&template_shell_path);

        let session_status_path = dir.path().join("examples/template/auth-session-status.tsx");
        fs::create_dir_all(session_status_path.parent().expect("session status parent"))
            .expect("session status dir");
        fs::write(
            &session_status_path,
            "export const authenticationSessionStatusProbe = 'fresh';\n",
        )
        .expect("Authentication session status source");
        let session_status_hash = sha256_file(&session_status_path);

        let authentication_receipt_path = dir.path().join(AUTHENTICATION_PACKAGE_RECEIPT_PATH);
        fs::create_dir_all(
            authentication_receipt_path
                .parent()
                .expect("Authentication receipt parent"),
        )
        .expect("Authentication receipt dir");
        fs::write(
            &authentication_receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.authentication_receipt",
                "official_package_name": AUTHENTICATION_OFFICIAL_NAME,
                "package_id": AUTHENTICATION_PACKAGE_ID,
                "upstream_package": AUTHENTICATION_UPSTREAM_PACKAGE,
                "upstream_version": AUTHENTICATION_UPSTREAM_VERSION,
                "source_mirror": AUTHENTICATION_SOURCE_MIRROR,
                "selected_surfaces": [
                    "authentication-account-workflow",
                    "authentication-session-status"
                ],
                "hash_algorithm": "sha256",
                "file_hashes": {
                    "examples/template/template-shell.tsx": template_shell_hash.clone(),
                    "examples/template/auth-session-status.tsx": session_status_hash.clone()
                },
                "runtime_limitations": [
                    "ADAPTER-BOUNDARY: credentials, callbacks, cookies, database adapters, email delivery, and hosted sessions stay app-owned."
                ]
            }))
            .expect("Authentication receipt json"),
        )
        .expect("write Authentication package receipt");

        let package_status_path = dir.path().join(AUTHENTICATION_PACKAGE_STATUS_PATH);
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": AUTHENTICATION_OFFICIAL_NAME,
                        "package_id": AUTHENTICATION_PACKAGE_ID,
                        "upstream_package": AUTHENTICATION_UPSTREAM_PACKAGE,
                        "upstream_version": AUTHENTICATION_UPSTREAM_VERSION,
                        "source_mirror": AUTHENTICATION_SOURCE_MIRROR,
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": AUTHENTICATION_PACKAGE_RECEIPT_PATH,
                        "receipt_hash_refresh": {
                            "schema": "dx.forge.package.receipt_hash_refresh",
                            "status": "current",
                            "helper_path": "examples/template/authentication-receipt-hashes.ts",
                            "check_command": "node examples/template/authentication-receipt-hashes.ts --check",
                            "write_command": "node examples/template/authentication-receipt-hashes.ts --write",
                            "json_check_command": "node examples/template/authentication-receipt-hashes.ts --check --json",
                            "receipt_path": AUTHENTICATION_PACKAGE_RECEIPT_PATH,
                            "hash_algorithm": "sha256",
                            "tracked_file_count": 2,
                            "stale_file_count": 0,
                            "missing_file_count": 0,
                            "runtime_execution": false,
                            "secret_access": false,
                            "zed_visibility": "authentication:receipt-hash-refresh",
                            "runtime_limitations": [
                                "SOURCE-ONLY: this helper checks local Authentication receipt hash freshness only.",
                                "ADAPTER-BOUNDARY: live OAuth, cookies, credentials, and hosted session proof stay app-owned."
                            ]
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
                                "surface_id": "authentication-account-workflow",
                                "status": "present",
                                "receipt_path": AUTHENTICATION_PACKAGE_RECEIPT_PATH,
                                "files": [
                                    "components/template-app/template-shell.tsx"
                                ],
                                "source_markers": [
                                    "data-dx-package=\"auth/better-auth\"",
                                    "data-dx-style-surface=\"authentication-account-workflow\""
                                ],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "examples/template/template-shell.tsx": template_shell_hash.clone()
                                }
                            },
                            {
                                "surface_id": "authentication-session-status",
                                "status": "present",
                                "receipt_path": AUTHENTICATION_PACKAGE_RECEIPT_PATH,
                                "files": [
                                    "components/launch/auth-session-status.tsx"
                                ],
                                "source_markers": [
                                    "data-dx-package=\"auth/better-auth\"",
                                    "data-dx-style-surface=\"authentication-session-status\""
                                ],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "examples/template/auth-session-status.tsx": session_status_hash.clone()
                                }
                            }
                        ],
                        "source_hashes": {
                            "algorithm": "sha256",
                            "files": {
                                "examples/template/template-shell.tsx": template_shell_hash.clone(),
                                "examples/template/auth-session-status.tsx": session_status_hash.clone()
                            }
                        },
                        "dx_style_compatibility": {
                            "schema": "dx.forge.package.dx_style_compatibility",
                            "status": "present",
                            "token_source": "styles/theme.css",
                            "generated_css": "styles/generated.css",
                            "surfaces": [
                                "authentication-account-workflow",
                                "authentication-session-status"
                            ]
                        },
                        "blocked_surfaces": [],
                        "unsupported_surfaces": [],
                        "runtime_limitations": [
                            "ADAPTER-BOUNDARY: package-lane visibility is receipt, source-marker, hash, helper, and dx-style evidence; live Authentication runtime proof is app-owned."
                        ]
                    }
                ]
            }))
            .expect("Authentication package-status json"),
        )
        .expect("write Authentication package status");

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let authentication = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == AUTHENTICATION_PACKAGE_ID)
            .expect("Authentication row");

        assert_eq!(
            authentication["official_package_name"],
            AUTHENTICATION_OFFICIAL_NAME
        );
        assert_eq!(
            authentication["upstream_package"],
            AUTHENTICATION_UPSTREAM_PACKAGE
        );
        assert_eq!(
            authentication["upstream_version"],
            AUTHENTICATION_UPSTREAM_VERSION
        );
        assert_eq!(
            authentication["source_mirror"],
            AUTHENTICATION_SOURCE_MIRROR
        );
        assert_eq!(authentication["status"], "present");
        assert_eq!(authentication["receipt_status"], "present");
        assert_eq!(
            authentication["receipt_hash_refresh"]["schema"],
            "dx.forge.package.receipt_hash_refresh"
        );
        assert_eq!(
            authentication["receipt_hash_refresh"]["zed_visibility"],
            "authentication:receipt-hash-refresh"
        );
        assert_eq!(
            authentication["receipt_hash_refresh"]["json_check_command"],
            "node examples/template/authentication-receipt-hashes.ts --check --json"
        );
        assert_eq!(
            authentication["receipt_hash_refresh"]["runtime_execution"],
            false
        );
        assert_eq!(
            authentication["receipt_hash_refresh"]["secret_access"],
            false
        );

        let metric_value = |name: &str| -> u64 {
            authentication["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(metric_value("authentication_package_present"), 1);
        assert_eq!(metric_value("authentication_receipt_present"), 1);
        assert_eq!(metric_value("authentication_receipt_stale"), 0);
        assert_eq!(metric_value("authentication_missing_receipt"), 0);
        assert_eq!(metric_value("authentication_blocked_surface"), 0);
        assert_eq!(metric_value("authentication_unsupported_surface"), 0);
        assert_eq!(metric_value("authentication_hash_manifest_present"), 1);
        assert_eq!(metric_value("authentication_hash_mismatch"), 0);
        assert_eq!(
            metric_value("authentication_receipt_hash_refresh_current"),
            1
        );
        assert_eq!(metric_value("authentication_receipt_hash_refresh_stale"), 0);
        assert_eq!(
            metric_value("authentication_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(
            metric_value("authentication_dx_style_compatibility_present"),
            1
        );
        assert_eq!(
            metric_value("authentication_dx_style_compatibility_missing"),
            0
        );

        let mut stale_helper_authentication: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("read package status"))
                .expect("stale helper Authentication package status json");
        stale_helper_authentication["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_file_count"] =
            serde_json::json!(1);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&stale_helper_authentication)
                .expect("stale helper Authentication package status bytes"),
        )
        .expect("write stale helper Authentication package status");

        let helper_stale_report = read_dx_check_latest_panel(dir.path());
        let helper_stale_view_model = serde_json::to_value(&helper_stale_report.view_model)
            .expect("helper stale view model json");
        let helper_stale_authentication = helper_stale_view_model["package_lane_rows"]
            .as_array()
            .expect("helper stale package lane rows")
            .iter()
            .find(|row| row["package_id"] == AUTHENTICATION_PACKAGE_ID)
            .expect("helper stale Authentication row");
        let helper_stale_metric_value = |name: &str| -> u64 {
            helper_stale_authentication["metrics"]
                .as_array()
                .expect("helper stale metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from helper stale row"))
        };

        assert_eq!(helper_stale_authentication["status"], "stale");
        assert_eq!(helper_stale_authentication["receipt_status"], "stale");
        assert_eq!(
            helper_stale_authentication["receipt_hash_refresh"]["stale_file_count"],
            1
        );
        assert_eq!(
            helper_stale_metric_value("authentication_receipt_hash_refresh_current"),
            0
        );
        assert_eq!(
            helper_stale_metric_value("authentication_receipt_hash_refresh_stale"),
            1
        );
        assert_eq!(
            helper_stale_metric_value("authentication_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(helper_stale_metric_value("authentication_hash_mismatch"), 0);
        assert_eq!(
            helper_stale_authentication["next_action"],
            "Run node examples/template/authentication-receipt-hashes.ts --write after reviewing changed Authentication account or session source surfaces."
        );
    }

    #[test]
    fn dx_check_latest_panel_exposes_state_management_package_lane_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let package_receipt_path = dir
            .path()
            .join(".dx/forge/receipts/packages/state-zustand.json");
        fs::create_dir_all(
            package_receipt_path
                .parent()
                .expect("package receipt parent"),
        )
        .expect("package receipt dir");
        fs::write(
            &package_receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "forge.package_add_receipt",
                "package": {
                    "official_package_name": "State Management",
                    "package_id": "state/zustand",
                    "upstream_package": "zustand",
                    "upstream_version": "5.0.13",
                    "source_mirror": "G:/WWW/inspirations/zustand",
                    "receipt_hash_refresh": {
                        "schema": "dx.forge.package.receipt_hash_refresh",
                        "status": "current",
                        "helper_path": "examples/template/state-management-receipt-hashes.ts",
                        "check_command": "node examples/template/state-management-receipt-hashes.ts --check",
                        "write_command": "node examples/template/state-management-receipt-hashes.ts --write",
                        "json_check_command": "node examples/template/state-management-receipt-hashes.ts --check --json",
                        "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json",
                        "hash_algorithm": "sha256",
                        "tracked_file_count": 7,
                        "stale_file_count": 0,
                        "missing_file_count": 0,
                        "current_files": [
                            "examples/template/state-zustand-dashboard.tsx",
                            "examples/template/template-shell.tsx",
                            "tools/launch/runtime-template/assets/launch-runtime.ts",
                            "tools/launch/runtime-template/pages/index.html",
                            "docs/packages/state-zustand.md",
                            "docs/packages/state-zustand.source-guard-runbook.json",
                            "tools/launch/materialize-www-template.ts"
                        ],
                        "stale_files": [],
                        "missing_files": [],
                        "stale_mirror_files": [],
                        "missing_mirror_files": [],
                        "runtime_execution": false,
                        "secret_access": false,
                        "zed_visibility": "state-management:receipt-hash-refresh",
                        "runtime_limitations": [
                            "SOURCE-ONLY: this helper checks local State Management receipt hash freshness only."
                        ]
                    },
                    "dx_check_visibility": {
                        "status": "present",
                        "receipt_status": "present",
                        "status_vocabulary": [
                            "present",
                            "stale",
                            "missing-receipt",
                            "blocked",
                            "unsupported-surface"
                        ],
                        "selected_surfaces": [
                            {
                                "surface_id": "launch-dashboard-state-workflow",
                                "status": "present",
                                "source_markers": [
                                    "data-dx-package=\"state/zustand\"",
                                    "data-dx-component=\"launch-dashboard-state-workflow\""
                                ],
                                "files": [
                                    "components/launch/state-zustand-dashboard.tsx"
                                ]
                            }
                        ],
                        "metrics": [
                            "state_management_receipt_present",
                            "state_management_receipt_stale",
                            "state_management_missing_receipt",
                            "state_management_blocked_surface",
                            "state_management_unsupported_surface",
                            "state_management_receipt_hash_refresh_current",
                            "state_management_receipt_hash_refresh_stale",
                            "state_management_receipt_hash_refresh_missing",
                            "state_management_dx_style_compatibility_present",
                            "state_management_dx_style_compatibility_missing"
                        ],
                        "dx_style_compatibility": {
                            "schema": "dx.forge.package.dx_style_compatibility",
                            "status": "present",
                            "token_source": "styles/theme.css",
                            "generated_css": "styles/generated.css",
                            "visible_surfaces": [
                                "launch-dashboard-state-workflow",
                                "launch-dashboard-state-shell"
                            ],
                            "data_dx_markers": [
                                "data-dx-style-surface=\"state-management\""
                            ],
                            "runtime_proof": false
                        },
                        "runtime_limitations": [
                            "SOURCE-ONLY: package-lane visibility is receipt and source-marker evidence."
                        ]
                    }
                }
            }))
            .expect("state management receipt json"),
        )
        .expect("write state management package receipt");

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let package_lane_rows = view_model
            .get("package_lane_rows")
            .and_then(|rows| rows.as_array())
            .expect("package lane rows");
        let state_management = package_lane_rows
            .iter()
            .find(|row| row["package_id"] == "state/zustand")
            .expect("State Management row");

        assert_eq!(
            state_management["official_package_name"],
            "State Management"
        );
        assert_eq!(state_management["upstream_package"], "zustand");
        assert_eq!(state_management["upstream_version"], "5.0.13");
        assert_eq!(
            state_management["source_mirror"],
            "G:/WWW/inspirations/zustand"
        );
        assert_eq!(state_management["status"], "present");
        assert_eq!(state_management["receipt_status"], "present");
        assert_eq!(
            state_management["package_receipt_path"],
            ".dx/forge/receipts/packages/state-zustand.json"
        );
        assert!(
            state_management["selected_surfaces"]
                .as_array()
                .expect("selected surfaces")
                .iter()
                .any(|surface| surface["surface_id"] == "launch-dashboard-state-workflow")
        );

        let metric_value = |name: &str| -> u64 {
            state_management["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(metric_value("state_management_package_present"), 1);
        assert_eq!(metric_value("state_management_receipt_present"), 1);
        assert_eq!(metric_value("state_management_receipt_stale"), 0);
        assert_eq!(metric_value("state_management_missing_receipt"), 0);
        assert_eq!(metric_value("state_management_blocked_surface"), 0);
        assert_eq!(metric_value("state_management_unsupported_surface"), 0);
        assert_eq!(
            metric_value("state_management_receipt_hash_refresh_current"),
            1
        );
        assert_eq!(
            metric_value("state_management_receipt_hash_refresh_stale"),
            0
        );
        assert_eq!(
            metric_value("state_management_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(
            metric_value("state_management_dx_style_compatibility_present"),
            1
        );
        assert_eq!(
            metric_value("state_management_dx_style_compatibility_missing"),
            0
        );
        assert_eq!(
            state_management["receipt_hash_refresh"]["helper_path"],
            "examples/template/state-management-receipt-hashes.ts"
        );
        assert_eq!(
            state_management["receipt_hash_refresh"]["json_check_command"],
            "node examples/template/state-management-receipt-hashes.ts --check --json"
        );
        assert_eq!(
            state_management["receipt_hash_refresh"]["zed_visibility"],
            "state-management:receipt-hash-refresh"
        );
        assert!(
            state_management["receipt_hash_refresh"]["current_files"]
                .as_array()
                .expect("current files")
                .iter()
                .any(|file| file == "tools/launch/materialize-www-template.ts")
        );
        assert_eq!(
            state_management["receipt_hash_refresh"]["stale_files"],
            serde_json::json!([])
        );
        assert_eq!(
            state_management["receipt_hash_refresh"]["missing_files"],
            serde_json::json!([])
        );

        let mut missing_style_receipt: serde_json::Value = serde_json::from_slice(
            &fs::read(&package_receipt_path).expect("read state management receipt"),
        )
        .expect("state management receipt json");
        missing_style_receipt["package"]["dx_check_visibility"]
            .as_object_mut()
            .expect("dx-check visibility")
            .remove("dx_style_compatibility");
        fs::write(
            &package_receipt_path,
            serde_json::to_vec_pretty(&missing_style_receipt).expect("missing style receipt json"),
        )
        .expect("write missing style receipt");

        let missing_style_report = read_dx_check_latest_panel(dir.path());
        let missing_style_view_model =
            serde_json::to_value(&missing_style_report.view_model).expect("view model json");
        let missing_style_state_management = missing_style_view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "state/zustand")
            .expect("State Management row");
        let missing_style_metric_value = |name: &str| -> u64 {
            missing_style_state_management["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(
            missing_style_metric_value("state_management_dx_style_compatibility_present"),
            0
        );
        assert_eq!(
            missing_style_metric_value("state_management_dx_style_compatibility_missing"),
            1
        );
        assert!(
            missing_style_state_management["next_action"]
                .as_str()
                .expect("next action")
                .contains("dx-style compatibility row")
        );
    }

    #[test]
    fn dx_check_latest_panel_exposes_state_management_stale_helper_file_attribution() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let package_receipt_path = dir
            .path()
            .join(".dx/forge/receipts/packages/state-zustand.json");
        fs::create_dir_all(
            package_receipt_path
                .parent()
                .expect("package receipt parent"),
        )
        .expect("package receipt dir");
        fs::write(&package_receipt_path, "{}").expect("state management package receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        let state_management_source_files = serde_json::json!([
            "examples/template/state-zustand-dashboard.tsx",
            "examples/template/template-shell.tsx",
            "tools/launch/runtime-template/assets/launch-runtime.ts",
            "tools/launch/runtime-template/pages/index.html",
            "docs/packages/state-zustand.md",
            "docs/packages/state-zustand.source-guard-runbook.json"
        ]);
        let mut stale_helper_state_management = serde_json::json!({
            "schema": "dx.www.template.forge_package_status",
            "package_lane_visibility": [
                {
                    "official_package_name": "State Management",
                    "package_id": "state/zustand",
                    "upstream_package": "zustand",
                    "upstream_version": "5.0.13",
                    "source_mirror": "G:/WWW/inspirations/zustand",
                    "status": "present",
                    "receipt_status": "present",
                    "package_receipt_path": ".dx/forge/receipts/packages/state-zustand.json",
                    "status_vocabulary": [
                        "present",
                        "stale",
                        "missing-receipt",
                        "blocked",
                        "unsupported-surface"
                    ],
                    "selected_surfaces": [
                        {
                            "surface_id": "launch-dashboard-state-workflow",
                            "status": "present",
                            "source_markers": [
                                "data-dx-package=\"state/zustand\"",
                                "data-dx-component=\"launch-dashboard-state-workflow\"",
                                "data-dx-style-surface=\"state-management\""
                            ],
                            "files": [
                                "components/launch/state-zustand-dashboard.tsx"
                            ]
                        }
                    ],
                    "receipt_hash_refresh": {
                        "schema": "dx.forge.package.receipt_hash_refresh",
                        "status": "current",
                        "helper_path": "examples/template/state-management-receipt-hashes.ts",
                        "check_command": "node examples/template/state-management-receipt-hashes.ts --check",
                        "write_command": "node examples/template/state-management-receipt-hashes.ts --write",
                        "json_check_command": "node examples/template/state-management-receipt-hashes.ts --check --json",
                        "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json",
                        "hash_algorithm": "sha256",
                        "tracked_file_count": 7,
                        "tracked_files": [
                            "examples/template/state-zustand-dashboard.tsx",
                            "examples/template/template-shell.tsx",
                            "tools/launch/runtime-template/assets/launch-runtime.ts",
                            "tools/launch/runtime-template/pages/index.html",
                            "docs/packages/state-zustand.md",
                            "docs/packages/state-zustand.source-guard-runbook.json",
                            "tools/launch/materialize-www-template.ts"
                        ],
                        "current_files": [],
                        "stale_files": [],
                        "missing_files": [],
                        "stale_mirror_files": [],
                        "missing_mirror_files": [],
                        "stale_file_count": 0,
                        "missing_file_count": 0,
                        "runtime_execution": false,
                        "secret_access": false,
                        "zed_visibility": "state-management:receipt-hash-refresh",
                        "runtime_limitations": [
                            "SOURCE-ONLY: this helper checks local State Management receipt hash freshness only."
                        ]
                    },
                    "dx_style_compatibility": {
                        "schema": "dx.forge.package.dx_style_compatibility",
                        "status": "present",
                        "visible_surfaces": [
                            "launch-dashboard-state-workflow",
                            "launch-dashboard-state-shell"
                        ],
                        "runtime_proof": false
                    },
                    "runtime_limitations": [
                        "SOURCE-ONLY: package-lane visibility is receipt and source-marker evidence."
                    ]
                }
            ]
        });
        stale_helper_state_management["package_lane_visibility"][0]["receipt_hash_refresh"]["status"] =
            serde_json::json!("stale");
        stale_helper_state_management["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_file_count"] =
            serde_json::json!(1);
        stale_helper_state_management["package_lane_visibility"][0]["receipt_hash_refresh"]["current_files"] =
            state_management_source_files.clone();
        stale_helper_state_management["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_files"] =
            serde_json::json!(["tools/launch/materialize-www-template.ts"]);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&stale_helper_state_management)
                .expect("state management stale helper package-status json"),
        )
        .expect("write stale helper state management package status");

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let state_management = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "state/zustand")
            .expect("State Management row");
        let metric_value = |name: &str| -> u64 {
            state_management["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(state_management["status"], "stale");
        assert_eq!(state_management["receipt_status"], "stale");
        assert_eq!(state_management["receipt_hash_refresh"]["status"], "stale");
        assert_eq!(
            state_management["receipt_hash_refresh"]["current_files"],
            state_management_source_files
        );
        assert_eq!(
            state_management["receipt_hash_refresh"]["stale_files"],
            serde_json::json!(["tools/launch/materialize-www-template.ts"])
        );
        assert_eq!(
            state_management["receipt_hash_refresh"]["missing_files"],
            serde_json::json!([])
        );
        assert_eq!(
            state_management["receipt_hash_refresh"]["stale_file_count"],
            1
        );
        assert_eq!(
            metric_value("state_management_receipt_hash_refresh_current"),
            0
        );
        assert_eq!(
            metric_value("state_management_receipt_hash_refresh_stale"),
            1
        );
        assert_eq!(
            metric_value("state_management_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(metric_value("state_management_receipt_stale"), 1);
        assert_eq!(
            metric_value("state_management_dx_style_compatibility_present"),
            1
        );
        assert_eq!(
            metric_value("state_management_dx_style_compatibility_missing"),
            0
        );
        assert_eq!(
            state_management["next_action"],
            "Run node examples/template/state-management-receipt-hashes.ts --write after reviewing the changed State Management source surfaces."
        );
    }

    #[test]
    fn dx_check_latest_panel_exposes_reactive_store_package_lane_hash_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let context_path = dir
            .path()
            .join("lib/forge/state/reactive-store/context.tsx");
        fs::create_dir_all(context_path.parent().expect("context parent")).expect("context dir");
        fs::write(
            &context_path,
            "export const reactiveStoreContextProbe = 'fresh';\n",
        )
        .expect("context source");
        let context_hash = sha256_file(&context_path);

        let package_receipt_path = dir
            .path()
            .join(".dx/forge/receipts/packages/reactive-store.json");
        fs::create_dir_all(
            package_receipt_path
                .parent()
                .expect("package receipt parent"),
        )
        .expect("package receipt dir");
        fs::write(
            &package_receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.reactive_store_receipt",
                "official_package_name": "Reactive Store",
                "package_id": "reactive/store",
                "upstream_package": "@tanstack/store",
                "based_on": "@tanstack/react-store",
                "upstream_version": "0.11.0",
                "source_mirror": "G:/WWW/inspirations/tanstack-store",
                "selected_surfaces": ["react-context"],
                "files": [
                    "lib/forge/state/reactive-store/context.tsx"
                ],
                "hash_algorithm": "sha256",
                "file_hashes": {
                    "lib/forge/state/reactive-store/context.tsx": context_hash
                },
                "status": "present",
                "dx_check_visibility": {
                    "schema": "dx.forge.package.dx_check_visibility",
                    "current_status": "present",
                    "receipt_status": "present",
                    "status_vocabulary": [
                        "present",
                        "stale",
                        "missing-receipt",
                        "blocked",
                        "unsupported-surface"
                    ],
                    "monitored_surfaces": ["react-context"],
                    "metrics": [
                        "reactive_store_receipt_present",
                        "reactive_store_receipt_stale",
                        "reactive_store_missing_receipt",
                        "reactive_store_blocked_surface",
                        "reactive_store_unsupported_surface",
                        "reactive_store_hash_manifest_present",
                        "reactive_store_hash_mismatch"
                    ]
                },
                "runtime_limitations": [
                    "SOURCE-ONLY: selected files and hashes are materialized; browser runtime proof is not claimed."
                ]
            }))
            .expect("reactive store receipt json"),
        )
        .expect("write reactive store package receipt");

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let package_lane_rows = view_model
            .get("package_lane_rows")
            .and_then(|rows| rows.as_array())
            .expect("package lane rows");
        let reactive_store = package_lane_rows
            .iter()
            .find(|row| row["package_id"] == "reactive/store")
            .expect("Reactive Store row");

        assert_eq!(reactive_store["official_package_name"], "Reactive Store");
        assert_eq!(reactive_store["upstream_package"], "@tanstack/store");
        assert_eq!(reactive_store["upstream_version"], "0.11.0");
        assert_eq!(
            reactive_store["source_mirror"],
            "G:/WWW/inspirations/tanstack-store"
        );
        assert_eq!(reactive_store["status"], "present");
        assert_eq!(reactive_store["receipt_status"], "present");
        assert_eq!(
            reactive_store["package_receipt_path"],
            ".dx/forge/receipts/packages/reactive-store.json"
        );
        assert!(
            reactive_store["selected_surfaces"]
                .as_array()
                .expect("selected surfaces")
                .iter()
                .any(|surface| surface["surface_id"] == "react-context")
        );

        let metric_value = |name: &str| -> u64 {
            reactive_store["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(metric_value("reactive_store_package_present"), 1);
        assert_eq!(metric_value("reactive_store_receipt_present"), 1);
        assert_eq!(metric_value("reactive_store_receipt_stale"), 0);
        assert_eq!(metric_value("reactive_store_missing_receipt"), 0);
        assert_eq!(metric_value("reactive_store_blocked_surface"), 0);
        assert_eq!(metric_value("reactive_store_unsupported_surface"), 0);
        assert_eq!(metric_value("reactive_store_hash_manifest_present"), 1);
        assert_eq!(metric_value("reactive_store_hash_mismatch"), 0);

        fs::write(
            &context_path,
            "export const reactiveStoreContextProbe = 'stale';\n",
        )
        .expect("stale context source");

        let stale_report = read_dx_check_latest_panel(dir.path());
        let stale_view_model =
            serde_json::to_value(&stale_report.view_model).expect("stale view model json");
        let stale_reactive_store = stale_view_model["package_lane_rows"]
            .as_array()
            .expect("stale package lane rows")
            .iter()
            .find(|row| row["package_id"] == "reactive/store")
            .expect("stale Reactive Store row");
        let stale_metric_value = |name: &str| -> u64 {
            stale_reactive_store["metrics"]
                .as_array()
                .expect("stale metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from stale row"))
        };

        assert_eq!(stale_reactive_store["status"], "stale");
        assert_eq!(stale_metric_value("reactive_store_receipt_stale"), 1);
        assert_eq!(stale_metric_value("reactive_store_hash_mismatch"), 1);
    }

    #[test]
    fn dx_check_latest_panel_exposes_reactive_store_package_lane_hash_refresh_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let context_path = dir
            .path()
            .join("lib/forge/state/reactive-store/context.tsx");
        fs::create_dir_all(context_path.parent().expect("context parent")).expect("context dir");
        fs::write(
            &context_path,
            "export const reactiveStoreContextProbe = 'fresh';\n",
        )
        .expect("context source");
        let context_hash = sha256_file(&context_path);

        let package_receipt_path = dir.path().join(REACTIVE_STORE_PACKAGE_RECEIPT_PATH);
        fs::create_dir_all(
            package_receipt_path
                .parent()
                .expect("package receipt parent"),
        )
        .expect("package receipt dir");
        fs::write(
            &package_receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.reactive_store_receipt",
                "official_package_name": "Reactive Store",
                "package_id": REACTIVE_STORE_PACKAGE_ID,
                "upstream_package": REACTIVE_STORE_UPSTREAM_PACKAGE,
                "based_on": "@tanstack/react-store",
                "upstream_version": REACTIVE_STORE_UPSTREAM_VERSION,
                "source_mirror": REACTIVE_STORE_SOURCE_MIRROR,
                "selected_surfaces": ["react-context"],
                "files": [
                    "lib/forge/state/reactive-store/context.tsx"
                ],
                "hash_algorithm": "sha256",
                "file_hashes": {
                    "lib/forge/state/reactive-store/context.tsx": context_hash
                },
                "status": "present",
                "dx_check_visibility": {
                    "schema": "dx.forge.package.dx_check_visibility",
                    "current_status": "present",
                    "receipt_status": "present",
                    "status_vocabulary": [
                        "present",
                        "stale",
                        "missing-receipt",
                        "blocked",
                        "unsupported-surface"
                    ],
                    "monitored_surfaces": ["react-context"]
                },
                "runtime_limitations": [
                    "SOURCE-ONLY: selected files and hashes are materialized; browser runtime proof is not claimed."
                ]
            }))
            .expect("Reactive Store receipt json"),
        )
        .expect("write Reactive Store package receipt");

        let package_status_path = dir.path().join(REACTIVE_STORE_PACKAGE_STATUS_PATH);
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": REACTIVE_STORE_OFFICIAL_NAME,
                        "package_id": REACTIVE_STORE_PACKAGE_ID,
                        "upstream_package": REACTIVE_STORE_UPSTREAM_PACKAGE,
                        "upstream_version": REACTIVE_STORE_UPSTREAM_VERSION,
                        "source_mirror": REACTIVE_STORE_SOURCE_MIRROR,
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": REACTIVE_STORE_PACKAGE_RECEIPT_PATH,
                        "receipt_hash_refresh": {
                            "schema": "dx.forge.package.receipt_hash_refresh",
                            "status": "current",
                            "helper_path": "examples/template/reactive-store-receipt-hashes.ts",
                            "check_command": "node examples/template/reactive-store-receipt-hashes.ts --check",
                            "write_command": "node examples/template/reactive-store-receipt-hashes.ts --write",
                            "json_check_command": "node examples/template/reactive-store-receipt-hashes.ts --check --json",
                            "receipt_path": REACTIVE_STORE_PACKAGE_RECEIPT_PATH,
                            "hash_algorithm": "sha256",
                            "tracked_file_count": 1,
                            "stale_file_count": 0,
                            "missing_file_count": 0,
                            "runtime_execution": false,
                            "secret_access": false,
                            "zed_visibility": "reactive-store:receipt-hash-refresh",
                            "runtime_limitations": [
                                "SOURCE-ONLY: this helper checks local Reactive Store receipt hash freshness only."
                            ]
                        },
                        "selected_surfaces": [
                            {
                                "surface_id": "react-context",
                                "status": "present",
                                "receipt_path": REACTIVE_STORE_PACKAGE_RECEIPT_PATH,
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "lib/forge/state/reactive-store/context.tsx": context_hash
                                }
                            }
                        ],
                        "runtime_limitations": [
                            "SOURCE-ONLY: package-lane visibility is receipt and source-marker evidence; no live React runtime proof is claimed."
                        ]
                    }
                ]
            }))
            .expect("Reactive Store package-status json"),
        )
        .expect("write Reactive Store package status");

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let reactive_store = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == REACTIVE_STORE_PACKAGE_ID)
            .expect("Reactive Store row");

        assert_eq!(
            reactive_store["receipt_hash_refresh"]["zed_visibility"],
            "reactive-store:receipt-hash-refresh"
        );
        assert_eq!(
            reactive_store["receipt_hash_refresh"]["json_check_command"],
            "node examples/template/reactive-store-receipt-hashes.ts --check --json"
        );
        assert_eq!(reactive_store["status"], "present");
        assert_eq!(reactive_store["receipt_status"], "present");

        let metric_value = |name: &str| -> u64 {
            reactive_store["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(metric_value("reactive_store_hash_mismatch"), 0);
        assert_eq!(
            metric_value("reactive_store_receipt_hash_refresh_current"),
            1
        );
        assert_eq!(metric_value("reactive_store_receipt_hash_refresh_stale"), 0);
        assert_eq!(
            metric_value("reactive_store_receipt_hash_refresh_missing"),
            0
        );

        let mut stale_helper_reactive_store: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("read package status"))
                .expect("stale helper package status json");
        stale_helper_reactive_store["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_file_count"] =
            serde_json::json!(1);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&stale_helper_reactive_store)
                .expect("stale helper package status bytes"),
        )
        .expect("write stale helper package status");

        let helper_stale_report = read_dx_check_latest_panel(dir.path());
        let helper_stale_view_model = serde_json::to_value(&helper_stale_report.view_model)
            .expect("helper stale view model json");
        let helper_stale_reactive_store = helper_stale_view_model["package_lane_rows"]
            .as_array()
            .expect("helper stale package lane rows")
            .iter()
            .find(|row| row["package_id"] == REACTIVE_STORE_PACKAGE_ID)
            .expect("helper stale Reactive Store row");
        let helper_stale_metric_value = |name: &str| -> u64 {
            helper_stale_reactive_store["metrics"]
                .as_array()
                .expect("helper stale metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from helper stale row"))
        };

        assert_eq!(helper_stale_reactive_store["status"], "stale");
        assert_eq!(helper_stale_reactive_store["receipt_status"], "stale");
        assert_eq!(helper_stale_metric_value("reactive_store_receipt_stale"), 1);
        assert_eq!(
            helper_stale_metric_value("reactive_store_receipt_hash_refresh_current"),
            0
        );
        assert_eq!(
            helper_stale_metric_value("reactive_store_receipt_hash_refresh_stale"),
            1
        );
        assert_eq!(
            helper_stale_metric_value("reactive_store_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(helper_stale_metric_value("reactive_store_hash_mismatch"), 0);
    }

    #[test]
    fn dx_check_latest_panel_exposes_data_fetching_cache_package_lane_hash_refresh_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let query_status_path = dir.path().join("examples/template/query-cache-status.tsx");
        fs::create_dir_all(query_status_path.parent().expect("query status parent"))
            .expect("query status dir");
        fs::write(
            &query_status_path,
            "export const dataFetchingCacheProbe = 'fresh';\n",
        )
        .expect("query status source");
        let query_status_hash = sha256_file(&query_status_path);

        let data_fetching_receipt_path = dir.path().join(DATA_FETCHING_CACHE_PACKAGE_RECEIPT_PATH);
        fs::create_dir_all(
            data_fetching_receipt_path
                .parent()
                .expect("Data Fetching & Cache receipt parent"),
        )
        .expect("Data Fetching & Cache receipt dir");
        fs::write(&data_fetching_receipt_path, "{}").expect("Data Fetching & Cache receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": "Data Fetching & Cache",
                        "package_id": DATA_FETCHING_CACHE_PACKAGE_ID,
                        "upstream_package": DATA_FETCHING_CACHE_UPSTREAM_PACKAGE,
                        "upstream_version": DATA_FETCHING_CACHE_UPSTREAM_VERSION,
                        "source_mirror": DATA_FETCHING_CACHE_SOURCE_MIRROR,
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": DATA_FETCHING_CACHE_PACKAGE_RECEIPT_PATH,
                        "receipt_hash_refresh": {
                            "schema": "dx.forge.package.receipt_hash_refresh",
                            "status": "current",
                            "helper_path": "examples/template/data-fetching-cache-receipt-hashes.ts",
                            "check_command": "node examples/template/data-fetching-cache-receipt-hashes.ts --check",
                            "write_command": "node examples/template/data-fetching-cache-receipt-hashes.ts --write",
                            "json_check_command": "node examples/template/data-fetching-cache-receipt-hashes.ts --check --json",
                            "receipt_path": DATA_FETCHING_CACHE_PACKAGE_RECEIPT_PATH,
                            "hash_algorithm": "sha256",
                            "tracked_file_count": 1,
                            "stale_file_count": 0,
                            "missing_file_count": 0,
                            "runtime_execution": false,
                            "secret_access": false,
                            "zed_visibility": "data-fetching-cache:receipt-hash-refresh",
                            "runtime_limitations": [
                                "SOURCE-ONLY: this helper checks local Data Fetching & Cache receipt hash freshness only."
                            ]
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
                                "surface_id": "data-fetching-cache-query-dashboard-workflow",
                                "status": "present",
                                "receipt_path": DATA_FETCHING_CACHE_PACKAGE_RECEIPT_PATH,
                                "files": [
                                    "components/launch/query-cache-status.tsx"
                                ],
                                "source_markers": [
                                    "data-dx-package=\"tanstack/query\"",
                                    "data-dx-component=\"tanstack-query-dashboard-data-workflow\""
                                ],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "examples/template/query-cache-status.tsx": query_status_hash
                                }
                            }
                        ],
                        "blocked_surfaces": [],
                        "unsupported_surfaces": [],
                        "dx_check_metrics": [
                            "data_fetching_cache_receipt_present",
                            "data_fetching_cache_receipt_stale",
                            "data_fetching_cache_missing_receipt",
                            "data_fetching_cache_blocked_surface",
                            "data_fetching_cache_unsupported_surface",
                            "data_fetching_cache_hash_manifest_present",
                            "data_fetching_cache_hash_mismatch",
                            "data_fetching_cache_receipt_hash_refresh_current",
                            "data_fetching_cache_receipt_hash_refresh_stale",
                            "data_fetching_cache_receipt_hash_refresh_missing"
                        ],
                        "dx_style_compatibility": {
                            "schema": "dx.forge.package.dx_style_compatibility",
                            "status": "present",
                            "token_source": "styles/theme.css",
                            "generated_css": "styles/generated.css",
                            "source_files": [
                                "examples/template/query-cache-status.tsx"
                            ],
                            "source_markers": [
                                "data-dx-style-surface=\"data-fetching-cache\""
                            ],
                            "runtime_proof": false
                        },
                        "runtime_limitations": [
                            "SOURCE-ONLY: package-lane visibility is receipt and source-marker evidence; no live QueryClient execution is claimed."
                        ]
                    }
                ]
            }))
            .expect("Data Fetching & Cache package-status json"),
        )
        .expect("write Data Fetching & Cache package status");

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let data_fetching = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == DATA_FETCHING_CACHE_PACKAGE_ID)
            .expect("Data Fetching & Cache row");

        assert_eq!(
            data_fetching["official_package_name"],
            DATA_FETCHING_CACHE_OFFICIAL_NAME
        );
        assert_eq!(
            data_fetching["upstream_package"],
            DATA_FETCHING_CACHE_UPSTREAM_PACKAGE
        );
        assert_eq!(
            data_fetching["upstream_version"],
            DATA_FETCHING_CACHE_UPSTREAM_VERSION
        );
        assert_eq!(
            data_fetching["source_mirror"],
            DATA_FETCHING_CACHE_SOURCE_MIRROR
        );
        assert_eq!(data_fetching["status"], "present");
        assert_eq!(data_fetching["receipt_status"], "present");
        assert_eq!(
            data_fetching["receipt_hash_refresh"]["zed_visibility"],
            "data-fetching-cache:receipt-hash-refresh"
        );
        assert_eq!(
            data_fetching["receipt_hash_refresh"]["runtime_execution"],
            false
        );
        assert!(
            data_fetching["next_action"]
                .as_str()
                .expect("next action")
                .contains("without claiming live QueryClient execution")
        );
        assert!(
            data_fetching["selected_surfaces"]
                .as_array()
                .expect("selected surfaces")
                .iter()
                .any(|surface| {
                    surface["surface_id"] == "data-fetching-cache-query-dashboard-workflow"
                })
        );

        let metric_value = |name: &str| -> u64 {
            data_fetching["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(metric_value("data_fetching_cache_package_present"), 1);
        assert_eq!(metric_value("data_fetching_cache_receipt_present"), 1);
        assert_eq!(metric_value("data_fetching_cache_receipt_stale"), 0);
        assert_eq!(metric_value("data_fetching_cache_missing_receipt"), 0);
        assert_eq!(metric_value("data_fetching_cache_hash_manifest_present"), 1);
        assert_eq!(metric_value("data_fetching_cache_hash_mismatch"), 0);
        assert_eq!(
            metric_value("data_fetching_cache_dx_style_compatibility_present"),
            1
        );
        assert_eq!(
            metric_value("data_fetching_cache_dx_style_compatibility_missing"),
            0
        );
        assert_eq!(
            metric_value("data_fetching_cache_receipt_hash_refresh_current"),
            1
        );
        assert_eq!(
            metric_value("data_fetching_cache_receipt_hash_refresh_stale"),
            0
        );
        assert_eq!(
            metric_value("data_fetching_cache_receipt_hash_refresh_missing"),
            0
        );

        let mut stale_helper_package_status: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("read package status"))
                .expect("stale helper package status json");
        stale_helper_package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_file_count"] =
            serde_json::json!(2);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&stale_helper_package_status)
                .expect("stale helper package status bytes"),
        )
        .expect("write stale helper package status");

        let helper_stale_report = read_dx_check_latest_panel(dir.path());
        let helper_stale_view_model = serde_json::to_value(&helper_stale_report.view_model)
            .expect("helper stale view model json");
        let helper_stale_data_fetching = helper_stale_view_model["package_lane_rows"]
            .as_array()
            .expect("helper stale package lane rows")
            .iter()
            .find(|row| row["package_id"] == DATA_FETCHING_CACHE_PACKAGE_ID)
            .expect("helper stale Data Fetching & Cache row");
        let helper_stale_metric_value = |name: &str| -> u64 {
            helper_stale_data_fetching["metrics"]
                .as_array()
                .expect("helper stale metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from helper stale row"))
        };

        assert_eq!(helper_stale_data_fetching["status"], "stale");
        assert_eq!(helper_stale_data_fetching["receipt_status"], "stale");
        assert_eq!(
            helper_stale_metric_value("data_fetching_cache_receipt_stale"),
            1
        );
        assert_eq!(
            helper_stale_metric_value("data_fetching_cache_receipt_hash_refresh_current"),
            0
        );
        assert_eq!(
            helper_stale_metric_value("data_fetching_cache_receipt_hash_refresh_stale"),
            1
        );
        assert_eq!(
            helper_stale_metric_value("data_fetching_cache_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(
            helper_stale_metric_value("data_fetching_cache_hash_mismatch"),
            0
        );
    }

    #[test]
    fn dx_check_latest_panel_exposes_forms_package_lane_hash_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let form_path = dir.path().join("examples/template/template-lead-form.tsx");
        fs::create_dir_all(form_path.parent().expect("form parent")).expect("form dir");
        fs::write(&form_path, "export const formsProbe = 'fresh';\n").expect("form source");
        let form_hash = sha256_file(&form_path);

        let forms_receipt_path = dir
            .path()
            .join(".dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json");
        fs::create_dir_all(forms_receipt_path.parent().expect("forms receipt parent"))
            .expect("forms receipt dir");
        fs::write(&forms_receipt_path, "{}").expect("forms receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": "Forms",
                        "package_id": "forms/react-hook-form",
                        "upstream_package": "react-hook-form",
                        "upstream_version": "7.75.0",
                        "source_mirror": "G:/WWW/inspirations/react-hook-form",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": ".dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json",
                        "receipt_hash_refresh": {
                            "schema": "dx.forge.package.receipt_hash_refresh",
                            "status": "current",
                            "helper_path": "examples/template/forms-receipt-hashes.ts",
                            "check_command": "node examples/template/forms-receipt-hashes.ts --check",
                            "write_command": "node examples/template/forms-receipt-hashes.ts --write",
                            "json_check_command": "node examples/template/forms-receipt-hashes.ts --check --json",
                            "receipt_path": ".dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json",
                            "hash_algorithm": "sha256",
                            "tracked_file_count": 2,
                            "tracked_files": [
                                "examples/template/template-lead-form.tsx",
                                "docs/packages/forms-react-hook-form.md"
                            ],
                            "current_files": [
                                "examples/template/template-lead-form.tsx",
                                "docs/packages/forms-react-hook-form.md"
                            ],
                            "stale_file_count": 0,
                            "stale_files": [],
                            "missing_file_count": 0,
                            "missing_files": [],
                            "stale_mirror_files": [],
                            "missing_mirror_files": [],
                            "runtime_execution": false,
                            "secret_access": false,
                            "zed_visibility": "forms:receipt-hash-refresh",
                            "runtime_limitations": [
                                "SOURCE-ONLY: this helper checks local Forms receipt hash freshness only."
                            ]
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
                                "surface_id": "template-lead-form",
                                "status": "present",
                                "receipt_path": ".dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json",
                                "files": [
                                    "components/launch/template-lead-form.tsx"
                                ],
                                "source_markers": [
                                    "data-dx-package=\"forms/react-hook-form\"",
                                    "data-dx-component=\"template-lead-form\""
                                ],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "examples/template/template-lead-form.tsx": form_hash
                                }
                            }
                        ],
                        "blocked_surfaces": [],
                        "unsupported_surfaces": [],
                        "dx_check_metrics": [
                            "forms_receipt_present",
                            "forms_receipt_stale",
                            "forms_missing_receipt",
                            "forms_blocked_surface",
                            "forms_unsupported_surface",
                            "forms_hash_manifest_present",
                            "forms_hash_mismatch"
                        ],
                        "runtime_limitations": [
                            "SOURCE-ONLY: package-lane visibility is receipt and source-marker evidence; no browser form submission proof is claimed."
                        ]
                    }
                ]
            }))
            .expect("forms package-status json"),
        )
        .expect("write forms package status");

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let forms = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "forms/react-hook-form")
            .expect("Forms row");

        assert_eq!(forms["official_package_name"], "Forms");
        assert_eq!(forms["upstream_package"], "react-hook-form");
        assert_eq!(forms["upstream_version"], "7.75.0");
        assert_eq!(
            forms["source_mirror"],
            "G:/WWW/inspirations/react-hook-form"
        );
        assert_eq!(forms["status"], "present");
        assert_eq!(forms["receipt_status"], "present");
        assert_eq!(
            forms["package_receipt_path"],
            ".dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json"
        );
        assert!(
            forms["selected_surfaces"]
                .as_array()
                .expect("selected surfaces")
                .iter()
                .any(|surface| surface["surface_id"] == "template-lead-form")
        );

        let metric_value = |name: &str| -> u64 {
            forms["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(metric_value("forms_package_present"), 1);
        assert_eq!(metric_value("forms_receipt_present"), 1);
        assert_eq!(metric_value("forms_receipt_stale"), 0);
        assert_eq!(metric_value("forms_missing_receipt"), 0);
        assert_eq!(metric_value("forms_blocked_surface"), 0);
        assert_eq!(metric_value("forms_unsupported_surface"), 0);
        assert_eq!(metric_value("forms_hash_manifest_present"), 1);
        assert_eq!(metric_value("forms_hash_mismatch"), 0);
        assert_eq!(metric_value("forms_receipt_hash_refresh_current"), 1);
        assert_eq!(metric_value("forms_receipt_hash_refresh_stale"), 0);
        assert_eq!(metric_value("forms_receipt_hash_refresh_missing"), 0);
        assert_eq!(
            forms["receipt_hash_refresh"]["current_files"],
            serde_json::json!([
                "examples/template/template-lead-form.tsx",
                "docs/packages/forms-react-hook-form.md"
            ])
        );
        assert_eq!(
            forms["receipt_hash_refresh"]["stale_files"],
            serde_json::json!([])
        );

        let mut stale_helper_package_status: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("read package status"))
                .expect("stale helper package status json");
        stale_helper_package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_file_count"] =
            serde_json::json!(0);
        stale_helper_package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_files"] =
            serde_json::json!(["docs/packages/forms-react-hook-form.md"]);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&stale_helper_package_status)
                .expect("stale helper package status bytes"),
        )
        .expect("write stale helper package status");

        let helper_stale_report = read_dx_check_latest_panel(dir.path());
        let helper_stale_view_model = serde_json::to_value(&helper_stale_report.view_model)
            .expect("helper stale view model json");
        let helper_stale_forms = helper_stale_view_model["package_lane_rows"]
            .as_array()
            .expect("helper stale package lane rows")
            .iter()
            .find(|row| row["package_id"] == "forms/react-hook-form")
            .expect("helper stale Forms row");
        let helper_stale_metric_value = |name: &str| -> u64 {
            helper_stale_forms["metrics"]
                .as_array()
                .expect("helper stale metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from helper stale row"))
        };

        assert_eq!(helper_stale_forms["status"], "stale");
        assert_eq!(helper_stale_forms["receipt_status"], "stale");
        assert_eq!(
            helper_stale_forms["receipt_hash_refresh"]["stale_files"][0],
            "docs/packages/forms-react-hook-form.md"
        );
        assert_eq!(
            helper_stale_forms["receipt_hash_refresh"]["stale_file_count"],
            0
        );
        assert_eq!(helper_stale_metric_value("forms_receipt_stale"), 1);
        assert_eq!(
            helper_stale_metric_value("forms_receipt_hash_refresh_current"),
            0
        );
        assert_eq!(
            helper_stale_metric_value("forms_receipt_hash_refresh_stale"),
            1
        );
        assert_eq!(
            helper_stale_metric_value("forms_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(helper_stale_metric_value("forms_hash_mismatch"), 0);

        fs::write(&form_path, "export const formsProbe = 'stale';\n").expect("stale form source");

        let stale_report = read_dx_check_latest_panel(dir.path());
        let stale_view_model =
            serde_json::to_value(&stale_report.view_model).expect("stale view model json");
        let stale_forms = stale_view_model["package_lane_rows"]
            .as_array()
            .expect("stale package lane rows")
            .iter()
            .find(|row| row["package_id"] == "forms/react-hook-form")
            .expect("stale Forms row");
        let stale_metric_value = |name: &str| -> u64 {
            stale_forms["metrics"]
                .as_array()
                .expect("stale metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from stale row"))
        };

        assert_eq!(stale_forms["status"], "stale");
        assert_eq!(stale_forms["receipt_status"], "stale");
        assert_eq!(stale_metric_value("forms_receipt_stale"), 1);
        assert_eq!(stale_metric_value("forms_hash_mismatch"), 1);
    }

    #[test]
    fn dx_check_latest_panel_exposes_type_safe_api_package_lane_hash_refresh_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let health_path = dir.path().join("examples/template/trpc-launch-health.tsx");
        fs::create_dir_all(health_path.parent().expect("health parent")).expect("health dir");
        fs::write(&health_path, "export const typeSafeApiProbe = 'fresh';\n")
            .expect("Type-Safe API health source");
        let health_hash = sha256_file(&health_path);

        let package_receipt_path = dir.path().join(TYPE_SAFE_API_PACKAGE_RECEIPT_PATH);
        fs::create_dir_all(package_receipt_path.parent().expect("receipt parent"))
            .expect("Type-Safe API receipt dir");
        fs::write(&package_receipt_path, "{}").expect("Type-Safe API receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": TYPE_SAFE_API_OFFICIAL_NAME,
                        "package_id": TYPE_SAFE_API_PACKAGE_ID,
                        "upstream_package": TYPE_SAFE_API_UPSTREAM_PACKAGE,
                        "upstream_version": TYPE_SAFE_API_UPSTREAM_VERSION,
                        "source_mirror": TYPE_SAFE_API_SOURCE_MIRROR,
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": TYPE_SAFE_API_PACKAGE_RECEIPT_PATH,
                        "receipt_hash_refresh": {
                            "schema": "dx.forge.package.receipt_hash_refresh",
                            "status": "current",
                            "helper_path": "examples/template/type-safe-api-receipt-hashes.ts",
                            "check_command": "node examples/template/type-safe-api-receipt-hashes.ts --check",
                            "write_command": "node examples/template/type-safe-api-receipt-hashes.ts --write",
                            "json_check_command": "node examples/template/type-safe-api-receipt-hashes.ts --check --json",
                            "receipt_path": TYPE_SAFE_API_PACKAGE_RECEIPT_PATH,
                            "hash_algorithm": "sha256",
                            "tracked_file_count": 1,
                            "stale_file_count": 0,
                            "missing_file_count": 0,
                            "tracked_files": [
                                "examples/template/trpc-launch-health.tsx"
                            ],
                            "current_files": [
                                "examples/template/trpc-launch-health.tsx"
                            ],
                            "stale_files": [],
                            "missing_files": [],
                            "stale_mirror_files": [],
                            "missing_mirror_files": [],
                            "runtime_execution": false,
                            "secret_access": false,
                            "zed_visibility": "type-safe-api:receipt-hash-refresh",
                            "runtime_limitations": [
                                "SOURCE-ONLY: this helper checks local Type-Safe API receipt hash freshness only."
                            ]
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
                                "surface_id": "trpc-launch-dashboard-workflow",
                                "status": "present",
                                "receipt_path": TYPE_SAFE_API_PACKAGE_RECEIPT_PATH,
                                "files": [
                                    "components/launch/trpc-launch-health.tsx"
                                ],
                                "source_markers": [
                                    "data-dx-package=\"api/trpc\"",
                                    "data-dx-component=\"launch-trpc-api-dashboard-workflow\"",
                                    "data-dx-trpc-action=\"check-health\""
                                ],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "examples/template/trpc-launch-health.tsx": health_hash
                                }
                            }
                        ],
                        "blocked_surfaces": [],
                        "unsupported_surfaces": [],
                        "runtime_limitations": [
                            "SOURCE-ONLY: package-lane visibility is receipt, source-marker, source-hash, and helper evidence without claiming live tRPC route execution."
                        ]
                    }
                ]
            }))
            .expect("Type-Safe API package-status json"),
        )
        .expect("write Type-Safe API package status");

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let type_safe_api = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == TYPE_SAFE_API_PACKAGE_ID)
            .expect("Type-Safe API row");

        assert_eq!(
            type_safe_api["official_package_name"],
            TYPE_SAFE_API_OFFICIAL_NAME
        );
        assert_eq!(
            type_safe_api["upstream_package"],
            TYPE_SAFE_API_UPSTREAM_PACKAGE
        );
        assert_eq!(
            type_safe_api["upstream_version"],
            TYPE_SAFE_API_UPSTREAM_VERSION
        );
        assert_eq!(type_safe_api["source_mirror"], TYPE_SAFE_API_SOURCE_MIRROR);
        assert_eq!(type_safe_api["status"], "present");
        assert_eq!(type_safe_api["receipt_status"], "present");
        assert_eq!(
            type_safe_api["package_receipt_path"],
            TYPE_SAFE_API_PACKAGE_RECEIPT_PATH
        );
        assert_eq!(
            type_safe_api["receipt_hash_refresh"]["zed_visibility"],
            "type-safe-api:receipt-hash-refresh"
        );
        assert_eq!(
            type_safe_api["receipt_hash_refresh"]["json_check_command"],
            "node examples/template/type-safe-api-receipt-hashes.ts --check --json"
        );
        assert_eq!(
            type_safe_api["receipt_hash_refresh"]["current_files"],
            serde_json::json!(["examples/template/trpc-launch-health.tsx"])
        );
        assert_eq!(
            type_safe_api["receipt_hash_refresh"]["stale_files"],
            serde_json::json!([])
        );
        assert_eq!(
            type_safe_api["receipt_hash_refresh"]["missing_files"],
            serde_json::json!([])
        );
        assert_eq!(
            type_safe_api["receipt_hash_refresh"]["stale_mirror_files"],
            serde_json::json!([])
        );
        assert_eq!(
            type_safe_api["receipt_hash_refresh"]["missing_mirror_files"],
            serde_json::json!([])
        );
        assert!(
            type_safe_api["selected_surfaces"]
                .as_array()
                .expect("selected surfaces")
                .iter()
                .any(|surface| surface["surface_id"] == "trpc-launch-dashboard-workflow")
        );

        let metric_value = |name: &str| -> u64 {
            type_safe_api["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(metric_value("type_safe_api_package_present"), 1);
        assert_eq!(metric_value("type_safe_api_receipt_present"), 1);
        assert_eq!(metric_value("type_safe_api_receipt_stale"), 0);
        assert_eq!(metric_value("type_safe_api_missing_receipt"), 0);
        assert_eq!(metric_value("type_safe_api_blocked_surface"), 0);
        assert_eq!(metric_value("type_safe_api_unsupported_surface"), 0);
        assert_eq!(metric_value("type_safe_api_hash_manifest_present"), 1);
        assert_eq!(metric_value("type_safe_api_hash_mismatch"), 0);
        assert_eq!(
            metric_value("type_safe_api_receipt_hash_refresh_current"),
            1
        );
        assert_eq!(metric_value("type_safe_api_receipt_hash_refresh_stale"), 0);
        assert_eq!(
            metric_value("type_safe_api_receipt_hash_refresh_missing"),
            0
        );

        let mut stale_helper_package_status: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("read package status"))
                .expect("stale helper package status json");
        stale_helper_package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_file_count"] =
            serde_json::json!(1);
        stale_helper_package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["current_files"] =
            serde_json::json!([]);
        stale_helper_package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_files"] =
            serde_json::json!(["docs/packages/api-trpc.source-guard-runbook.json"]);
        stale_helper_package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_mirror_files"] =
            serde_json::json!(["examples/template/forge-package-status-read-model.ts"]);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&stale_helper_package_status)
                .expect("stale helper package status bytes"),
        )
        .expect("write stale helper package status");

        let stale_report = read_dx_check_latest_panel(dir.path());
        let stale_view_model =
            serde_json::to_value(&stale_report.view_model).expect("stale view model json");
        let stale_type_safe_api = stale_view_model["package_lane_rows"]
            .as_array()
            .expect("stale package lane rows")
            .iter()
            .find(|row| row["package_id"] == TYPE_SAFE_API_PACKAGE_ID)
            .expect("stale Type-Safe API row");
        let stale_metric_value = |name: &str| -> u64 {
            stale_type_safe_api["metrics"]
                .as_array()
                .expect("stale metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from stale row"))
        };

        assert_eq!(stale_type_safe_api["status"], "stale");
        assert_eq!(stale_type_safe_api["receipt_status"], "stale");
        assert_eq!(
            stale_type_safe_api["receipt_hash_refresh"]["current_files"],
            serde_json::json!([])
        );
        assert_eq!(
            stale_type_safe_api["receipt_hash_refresh"]["stale_files"],
            serde_json::json!(["docs/packages/api-trpc.source-guard-runbook.json"])
        );
        assert_eq!(
            stale_type_safe_api["receipt_hash_refresh"]["stale_mirror_files"],
            serde_json::json!(["examples/template/forge-package-status-read-model.ts"])
        );
        assert_eq!(stale_metric_value("type_safe_api_receipt_stale"), 1);
        assert_eq!(stale_metric_value("type_safe_api_hash_mismatch"), 0);
        assert_eq!(
            stale_metric_value("type_safe_api_receipt_hash_refresh_current"),
            0
        );
        assert_eq!(
            stale_metric_value("type_safe_api_receipt_hash_refresh_stale"),
            1
        );
        assert_eq!(
            stale_metric_value("type_safe_api_receipt_hash_refresh_missing"),
            0
        );
        assert!(
            stale_type_safe_api["next_action"]
                .as_str()
                .expect("stale next action")
                .contains("type-safe-api-receipt-hashes.ts --write")
        );
    }

    #[test]
    fn dx_check_latest_panel_exposes_backend_platform_client_package_lane_hash_refresh_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let workflow_path = dir
            .path()
            .join("examples/template/supabase-profile-workflow.tsx");
        fs::create_dir_all(workflow_path.parent().expect("workflow parent")).expect("workflow dir");
        fs::write(
            &workflow_path,
            "export const supabaseProfileWorkflowProbe = 'fresh';\n",
        )
        .expect("Backend Platform Client workflow source");
        let workflow_hash = sha256_file(&workflow_path);

        let package_receipt_path = dir
            .path()
            .join(BACKEND_PLATFORM_CLIENT_PACKAGE_RECEIPT_PATH);
        fs::create_dir_all(package_receipt_path.parent().expect("receipt parent"))
            .expect("Backend Platform Client receipt dir");
        fs::write(&package_receipt_path, "{}").expect("Backend Platform Client receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": "Backend Platform Client",
                        "package_id": "supabase/client",
                        "upstream_package": "@supabase/ssr + @supabase/supabase-js",
                        "upstream_version": "@supabase/ssr latest; @supabase/supabase-js ^2",
                        "source_mirror": "G:/WWW/inspirations/supabase",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": BACKEND_PLATFORM_CLIENT_PACKAGE_RECEIPT_PATH,
                        "receipt_hash_refresh": {
                            "schema": "dx.forge.package.receipt_hash_refresh",
                            "status": "current",
                            "helper_path": "examples/template/backend-platform-client-receipt-hashes.ts",
                            "check_command": "node examples/template/backend-platform-client-receipt-hashes.ts --check",
                            "write_command": "node examples/template/backend-platform-client-receipt-hashes.ts --write",
                            "json_check_command": "node examples/template/backend-platform-client-receipt-hashes.ts --check --json",
                            "receipt_path": BACKEND_PLATFORM_CLIENT_PACKAGE_RECEIPT_PATH,
                            "hash_algorithm": "sha256",
                            "tracked_file_count": 1,
                            "stale_file_count": 0,
                            "missing_file_count": 0,
                            "runtime_execution": false,
                            "secret_access": false,
                            "zed_visibility": "backend-platform-client:receipt-hash-refresh"
                        },
                        "selected_surfaces": [
                            {
                                "surface_id": "supabase-profile-workflow",
                                "status": "present",
                                "receipt_path": BACKEND_PLATFORM_CLIENT_PACKAGE_RECEIPT_PATH,
                                "source_markers": [
                                    "data-dx-package=\"supabase/client\"",
                                    "data-dx-component=\"supabase-profile-workflow\""
                                ],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "examples/template/supabase-profile-workflow.tsx": workflow_hash
                                }
                            }
                        ],
                        "blocked_surfaces": [],
                        "unsupported_surfaces": [],
                        "runtime_limitations": [
                            "ADAPTER-BOUNDARY: hosted Supabase credentials, RLS, database writes, Storage, Realtime, and Edge Functions stay app-owned."
                        ]
                    }
                ]
            }))
            .expect("Backend Platform Client package-status json"),
        )
        .expect("write Backend Platform Client package status");

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let backend_platform_client = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "supabase/client")
            .expect("Backend Platform Client row");

        assert_eq!(
            backend_platform_client["official_package_name"],
            "Backend Platform Client"
        );
        assert_eq!(backend_platform_client["status"], "present");
        assert_eq!(
            backend_platform_client["receipt_hash_refresh"]["zed_visibility"],
            "backend-platform-client:receipt-hash-refresh"
        );
        assert_eq!(
            backend_platform_client["receipt_hash_refresh"]["json_check_command"],
            "node examples/template/backend-platform-client-receipt-hashes.ts --check --json"
        );

        let metric_value = |name: &str| -> u64 {
            backend_platform_client["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(metric_value("backend_platform_client_package_present"), 1);
        assert_eq!(metric_value("backend_platform_client_receipt_present"), 1);
        assert_eq!(metric_value("backend_platform_client_receipt_stale"), 0);
        assert_eq!(
            metric_value("backend_platform_client_hash_manifest_present"),
            1
        );
        assert_eq!(metric_value("backend_platform_client_hash_mismatch"), 0);
        assert_eq!(
            metric_value("backend_platform_client_receipt_hash_refresh_current"),
            1
        );
        assert_eq!(
            metric_value("backend_platform_client_receipt_hash_refresh_stale"),
            0
        );
        assert_eq!(
            metric_value("backend_platform_client_receipt_hash_refresh_missing"),
            0
        );

        let mut stale_helper_backend_platform_client: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("read package status"))
                .expect("package-status json");
        stale_helper_backend_platform_client["package_lane_visibility"][0]["receipt_hash_refresh"]
            ["status"] = serde_json::Value::String("stale".to_string());
        stale_helper_backend_platform_client["package_lane_visibility"][0]["receipt_hash_refresh"]
            ["stale_file_count"] = serde_json::Value::from(1);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&stale_helper_backend_platform_client)
                .expect("stale helper package-status json"),
        )
        .expect("write stale helper package status");

        let helper_stale_report = read_dx_check_latest_panel(dir.path());
        let helper_stale_view_model =
            serde_json::to_value(&helper_stale_report.view_model).expect("view model json");
        let helper_stale_backend_platform_client = helper_stale_view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "supabase/client")
            .expect("Backend Platform Client helper-stale row");
        assert_eq!(helper_stale_backend_platform_client["status"], "stale");
        assert_eq!(
            helper_stale_backend_platform_client["receipt_status"],
            "stale"
        );

        let helper_stale_metric_value = |name: &str| -> u64 {
            helper_stale_backend_platform_client["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(
            helper_stale_metric_value("backend_platform_client_receipt_hash_refresh_current"),
            0
        );
        assert_eq!(
            helper_stale_metric_value("backend_platform_client_receipt_hash_refresh_stale"),
            1
        );
        assert_eq!(
            helper_stale_metric_value("backend_platform_client_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(
            helper_stale_metric_value("backend_platform_client_hash_mismatch"),
            0
        );
    }

    #[test]
    fn dx_check_latest_panel_exposes_validation_schemas_package_lane_hash_refresh_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let settings_path = dir
            .path()
            .join("examples/template/zod-dashboard-settings.tsx");
        fs::create_dir_all(settings_path.parent().expect("settings parent")).expect("settings dir");
        fs::write(
            &settings_path,
            "export const validationSchemasProbe = 'fresh';\n",
        )
        .expect("settings source");
        let settings_hash = sha256_file(&settings_path);

        let validation_receipt_path = dir.path().join(
            "examples/template/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json",
        );
        fs::create_dir_all(
            validation_receipt_path
                .parent()
                .expect("validation receipt parent"),
        )
        .expect("validation receipt dir");
        fs::write(&validation_receipt_path, "{}").expect("validation receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": "Validation & Schemas",
                        "package_id": "validation/zod",
                        "upstream_package": "zod",
                        "upstream_version": "4.4.3",
                        "source_mirror": "G:/WWW/inspirations/zod",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json",
                        "status_vocabulary": [
                            "present",
                            "stale",
                            "missing-receipt",
                            "blocked",
                            "unsupported-surface"
                        ],
                        "selected_surfaces": [
                            {
                                "surface_id": "dashboard-settings-validation",
                                "status": "present",
                                "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json",
                                "files": [
                                    "components/launch/zod-dashboard-settings.tsx"
                                ],
                                "source_markers": [
                                    "data-dx-package=\"validation/zod\"",
                                    "data-dx-zod-field-errors-api=\"z.flattenError\""
                                ],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "examples/template/zod-dashboard-settings.tsx": settings_hash
                                }
                            }
                        ],
                        "source_hashes": {
                            "algorithm": "sha256",
                            "files": {
                                "examples/template/zod-dashboard-settings.tsx": settings_hash
                            }
                        },
                        "receipt_hash_refresh": {
                            "schema": "dx.forge.package.receipt_hash_refresh",
                            "status": "current",
                            "helper_path": "examples/template/validation-schemas-receipt-hashes.ts",
                            "check_command": "node examples/template/validation-schemas-receipt-hashes.ts --check",
                            "write_command": "node examples/template/validation-schemas-receipt-hashes.ts --write",
                            "json_check_command": "node examples/template/validation-schemas-receipt-hashes.ts --check --json",
                            "source_guard_runbook_fixture": "docs/packages/validation-schemas.source-guard-runbook.json",
                            "preview_manifest_materializer": "tools/launch/materialize-www-template.ts",
                            "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json",
                            "hash_algorithm": "sha256",
                            "tracked_file_count": 3,
                            "tracked_files": [
                                "examples/template/zod-dashboard-settings.tsx",
                                "tools/launch/materialize-www-template.ts",
                                "docs/packages/validation-schemas.source-guard-runbook.json"
                            ],
                            "stale_file_count": 0,
                            "current_files": [
                                "examples/template/zod-dashboard-settings.tsx",
                                "tools/launch/materialize-www-template.ts",
                                "docs/packages/validation-schemas.source-guard-runbook.json"
                            ],
                            "stale_files": [],
                            "missing_file_count": 0,
                            "missing_files": [],
                            "stale_mirror_files": [],
                            "missing_mirror_files": [],
                            "runtime_execution": false,
                            "secret_access": false,
                            "zed_visibility": "validation-schemas:receipt-hash-refresh"
                        },
                        "blocked_surfaces": [],
                        "unsupported_surfaces": [],
                        "runtime_limitations": [
                            "SOURCE-ONLY: package-lane visibility is receipt, source-marker, hash, and helper evidence; no live Validation & Schemas runtime proof is claimed."
                        ]
                    }
                ]
            }))
            .expect("Validation & Schemas package-status json"),
        )
        .expect("write Validation & Schemas package status");

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let validation = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "validation/zod")
            .expect("Validation & Schemas row");

        assert_eq!(validation["official_package_name"], "Validation & Schemas");
        assert_eq!(validation["upstream_package"], "zod");
        assert_eq!(validation["upstream_version"], "4.4.3");
        assert_eq!(validation["source_mirror"], "G:/WWW/inspirations/zod");
        assert_eq!(validation["status"], "present");
        assert_eq!(validation["receipt_status"], "present");
        assert_eq!(
            validation["receipt_hash_refresh"]["zed_visibility"],
            "validation-schemas:receipt-hash-refresh"
        );
        assert_eq!(
            validation["receipt_hash_refresh"]["tracked_files"],
            serde_json::json!([
                "examples/template/zod-dashboard-settings.tsx",
                "tools/launch/materialize-www-template.ts",
                "docs/packages/validation-schemas.source-guard-runbook.json"
            ])
        );
        assert_eq!(
            validation["receipt_hash_refresh"]["current_files"],
            serde_json::json!([
                "examples/template/zod-dashboard-settings.tsx",
                "tools/launch/materialize-www-template.ts",
                "docs/packages/validation-schemas.source-guard-runbook.json"
            ])
        );
        assert_eq!(
            validation["receipt_hash_refresh"]["stale_files"],
            serde_json::json!([])
        );
        assert_eq!(
            validation["receipt_hash_refresh"]["missing_files"],
            serde_json::json!([])
        );
        assert_eq!(
            validation["receipt_hash_refresh"]["source_guard_runbook_fixture"],
            "docs/packages/validation-schemas.source-guard-runbook.json"
        );
        assert_eq!(
            validation["receipt_hash_refresh"]["preview_manifest_materializer"],
            "tools/launch/materialize-www-template.ts"
        );

        let metric_value = |name: &str| -> u64 {
            validation["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(metric_value("validation_schemas_package_present"), 1);
        assert_eq!(metric_value("validation_schemas_receipt_present"), 1);
        assert_eq!(metric_value("validation_schemas_receipt_stale"), 0);
        assert_eq!(metric_value("validation_schemas_hash_manifest_present"), 1);
        assert_eq!(metric_value("validation_schemas_hash_mismatch"), 0);
        assert_eq!(
            metric_value("validation_schemas_receipt_hash_refresh_current"),
            1
        );
        assert_eq!(
            metric_value("validation_schemas_receipt_hash_refresh_stale"),
            0
        );
        assert_eq!(
            metric_value("validation_schemas_receipt_hash_refresh_missing"),
            0
        );

        let mut stale_helper_validation: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("read package status"))
                .expect("stale helper Validation & Schemas package status json");
        stale_helper_validation["package_lane_visibility"][0]["receipt_hash_refresh"]["status"] =
            serde_json::json!("stale");
        stale_helper_validation["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_file_count"] =
            serde_json::json!(1);
        stale_helper_validation["package_lane_visibility"][0]["receipt_hash_refresh"]["current_files"] =
            serde_json::json!(["examples/template/zod-dashboard-settings.tsx"]);
        stale_helper_validation["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_files"] =
            serde_json::json!(["tools/launch/materialize-www-template.ts"]);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&stale_helper_validation)
                .expect("stale helper Validation & Schemas package status bytes"),
        )
        .expect("write stale helper Validation & Schemas package status");

        let helper_stale_report = read_dx_check_latest_panel(dir.path());
        let helper_stale_view_model = serde_json::to_value(&helper_stale_report.view_model)
            .expect("helper stale view model json");
        let helper_stale_validation = helper_stale_view_model["package_lane_rows"]
            .as_array()
            .expect("helper stale package lane rows")
            .iter()
            .find(|row| row["package_id"] == "validation/zod")
            .expect("helper stale Validation & Schemas row");
        let helper_stale_metric_value = |name: &str| -> u64 {
            helper_stale_validation["metrics"]
                .as_array()
                .expect("helper stale metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from helper stale row"))
        };

        assert_eq!(helper_stale_validation["status"], "stale");
        assert_eq!(helper_stale_validation["receipt_status"], "stale");
        assert_eq!(
            helper_stale_validation["receipt_hash_refresh"]["current_files"],
            serde_json::json!(["examples/template/zod-dashboard-settings.tsx"])
        );
        assert_eq!(
            helper_stale_validation["receipt_hash_refresh"]["stale_files"],
            serde_json::json!(["tools/launch/materialize-www-template.ts"])
        );
        assert_eq!(
            helper_stale_validation["receipt_hash_refresh"]["missing_files"],
            serde_json::json!([])
        );
        assert_eq!(
            helper_stale_validation["receipt_hash_refresh"]["preview_manifest_materializer"],
            "tools/launch/materialize-www-template.ts"
        );
        assert_eq!(
            helper_stale_metric_value("validation_schemas_receipt_hash_refresh_current"),
            0
        );
        assert_eq!(
            helper_stale_metric_value("validation_schemas_receipt_hash_refresh_stale"),
            1
        );
        assert_eq!(
            helper_stale_metric_value("validation_schemas_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(
            helper_stale_metric_value("validation_schemas_hash_mismatch"),
            0
        );
    }

    #[test]
    fn dx_check_latest_panel_exposes_motion_animation_package_lane_hash_refresh_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let template_shell_path = dir.path().join("examples/template/template-shell.tsx");
        fs::create_dir_all(template_shell_path.parent().expect("launch shell parent"))
            .expect("launch shell dir");
        fs::write(
            &template_shell_path,
            "export const motionLaunchProbe = 'fresh';\n",
        )
        .expect("motion launch source");
        let template_shell_hash = sha256_file(&template_shell_path);

        let motion_receipt_path = dir.path().join(
            "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
        );
        fs::create_dir_all(motion_receipt_path.parent().expect("motion receipt parent"))
            .expect("motion receipt dir");
        fs::write(&motion_receipt_path, "{}").expect("motion receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": "Motion & Animation",
                        "package_id": "animation/motion",
                        "upstream_package": "motion",
                        "upstream_version": "12.38.0",
                        "source_mirror": "G:/WWW/inspirations/motion",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
                        "receipt_hash_refresh": {
                            "schema": "dx.forge.package.receipt_hash_refresh",
                            "status": "current",
                            "helper_path": "examples/template/motion-receipt-hashes.ts",
                            "check_command": "node examples/template/motion-receipt-hashes.ts --check",
                            "write_command": "node examples/template/motion-receipt-hashes.ts --write",
                            "json_check_command": "node examples/template/motion-receipt-hashes.ts --check --json",
                            "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
                            "hash_algorithm": "sha256",
                            "tracked_file_count": 1,
                            "stale_file_count": 0,
                            "missing_file_count": 0,
                            "runtime_execution": false,
                            "secret_access": false,
                            "zed_visibility": "motion-animation:receipt-hash-refresh",
                            "runtime_limitations": [
                                "SOURCE-ONLY: this helper checks local Motion & Animation receipt hash freshness only.",
                                "ADAPTER-BOUNDARY: browser animation runtime proof stays app-owned."
                            ]
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
                                "surface_id": "motion-dashboard-workflow",
                                "status": "present",
                                "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
                                "files": [
                                    "components/template-app/template-shell.tsx"
                                ],
                                "source_markers": [
                                    "data-dx-package=\"animation/motion\"",
                                    "data-dx-component=\"launch-motion-dashboard-workflow\""
                                ],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "examples/template/template-shell.tsx": template_shell_hash.clone()
                                }
                            }
                        ],
                        "source_hashes": {
                            "algorithm": "sha256",
                            "files": {
                                "examples/template/template-shell.tsx": template_shell_hash
                            }
                        },
                        "blocked_surfaces": [],
                        "unsupported_surfaces": [],
                        "runtime_limitations": [
                            "SOURCE-ONLY: package-lane visibility is receipt, source-marker, hash, and helper evidence; no live Motion browser animation proof is claimed."
                        ]
                    }
                ]
            }))
            .expect("motion package-status json"),
        )
        .expect("write motion package status");

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let motion_animation = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "animation/motion")
            .expect("Motion & Animation row");

        assert_eq!(
            motion_animation["official_package_name"],
            "Motion & Animation"
        );
        assert_eq!(motion_animation["upstream_package"], "motion");
        assert_eq!(motion_animation["upstream_version"], "12.38.0");
        assert_eq!(
            motion_animation["source_mirror"],
            "G:/WWW/inspirations/motion"
        );
        assert_eq!(motion_animation["status"], "present");
        assert_eq!(motion_animation["receipt_status"], "present");
        assert_eq!(
            motion_animation["package_receipt_path"],
            "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json"
        );
        assert_eq!(
            motion_animation["receipt_hash_refresh"]["schema"],
            "dx.forge.package.receipt_hash_refresh"
        );
        assert_eq!(
            motion_animation["receipt_hash_refresh"]["status"],
            "current"
        );
        assert_eq!(
            motion_animation["receipt_hash_refresh"]["helper_path"],
            "examples/template/motion-receipt-hashes.ts"
        );
        assert_eq!(
            motion_animation["receipt_hash_refresh"]["json_check_command"],
            "node examples/template/motion-receipt-hashes.ts --check --json"
        );
        assert_eq!(
            motion_animation["receipt_hash_refresh"]["zed_visibility"],
            "motion-animation:receipt-hash-refresh"
        );
        assert_eq!(
            motion_animation["receipt_hash_refresh"]["runtime_execution"],
            false
        );
        assert_eq!(
            motion_animation["receipt_hash_refresh"]["secret_access"],
            false
        );

        let metric_value = |name: &str| -> u64 {
            motion_animation["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(metric_value("motion_animation_package_present"), 1);
        assert_eq!(metric_value("motion_animation_receipt_present"), 1);
        assert_eq!(metric_value("motion_animation_receipt_stale"), 0);
        assert_eq!(metric_value("motion_animation_missing_receipt"), 0);
        assert_eq!(metric_value("motion_animation_blocked_surface"), 0);
        assert_eq!(metric_value("motion_animation_unsupported_surface"), 0);
        assert_eq!(metric_value("motion_animation_hash_manifest_present"), 1);
        assert_eq!(metric_value("motion_animation_hash_mismatch"), 0);
        assert_eq!(
            metric_value("motion_animation_receipt_hash_refresh_current"),
            1
        );
        assert_eq!(
            metric_value("motion_animation_receipt_hash_refresh_stale"),
            0
        );
        assert_eq!(
            metric_value("motion_animation_receipt_hash_refresh_missing"),
            0
        );

        let mut stale_helper_motion_animation: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("read package status"))
                .expect("stale helper Motion package status json");
        stale_helper_motion_animation["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_file_count"] =
            serde_json::json!(2);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&stale_helper_motion_animation)
                .expect("stale helper Motion package status bytes"),
        )
        .expect("write stale helper Motion package status");

        let helper_stale_report = read_dx_check_latest_panel(dir.path());
        let helper_stale_view_model = serde_json::to_value(&helper_stale_report.view_model)
            .expect("helper stale view model json");
        let helper_stale_motion_animation = helper_stale_view_model["package_lane_rows"]
            .as_array()
            .expect("helper stale package lane rows")
            .iter()
            .find(|row| row["package_id"] == "animation/motion")
            .expect("helper stale Motion & Animation row");
        let helper_stale_metric_value = |name: &str| -> u64 {
            helper_stale_motion_animation["metrics"]
                .as_array()
                .expect("helper stale metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from helper stale row"))
        };

        assert_eq!(helper_stale_motion_animation["status"], "stale");
        assert_eq!(helper_stale_motion_animation["receipt_status"], "stale");
        assert_eq!(
            helper_stale_metric_value("motion_animation_receipt_hash_refresh_current"),
            0
        );
        assert_eq!(
            helper_stale_metric_value("motion_animation_receipt_hash_refresh_stale"),
            1
        );
        assert_eq!(
            helper_stale_metric_value("motion_animation_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(
            helper_stale_metric_value("motion_animation_hash_mismatch"),
            0
        );
        assert_eq!(
            helper_stale_motion_animation["next_action"],
            "Run node examples/template/motion-receipt-hashes.ts --write after reviewing changed Motion & Animation source files, then rerun dx-check."
        );
    }

    #[test]
    fn dx_check_latest_panel_exposes_type_safe_api_unsupported_surface_context() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let package_receipt_path = dir.path().join(TYPE_SAFE_API_PACKAGE_RECEIPT_PATH);
        fs::create_dir_all(package_receipt_path.parent().expect("receipt parent"))
            .expect("Type-Safe API receipt dir");
        fs::write(&package_receipt_path, "{}").expect("Type-Safe API receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": TYPE_SAFE_API_OFFICIAL_NAME,
                        "package_id": TYPE_SAFE_API_PACKAGE_ID,
                        "upstream_package": TYPE_SAFE_API_UPSTREAM_PACKAGE,
                        "upstream_version": TYPE_SAFE_API_UPSTREAM_VERSION,
                        "source_mirror": TYPE_SAFE_API_SOURCE_MIRROR,
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": TYPE_SAFE_API_PACKAGE_RECEIPT_PATH,
                        "status_vocabulary": [
                            "present",
                            "stale",
                            "missing-receipt",
                            "blocked",
                            "unsupported-surface"
                        ],
                        "selected_surfaces": [
                            {
                                "surface_id": "trpc-route-handler",
                                "status": "present",
                                "receipt_path": TYPE_SAFE_API_PACKAGE_RECEIPT_PATH,
                                "files": [
                                    "app/api/trpc/[trpc]/route.ts"
                                ],
                                "source_markers": [
                                    "fetchRequestHandler"
                                ]
                            }
                        ],
                        "blocked_surfaces": [],
                        "unsupported_surfaces": [
                            {
                                "surface_id": "trpc-websocket-subscriptions",
                                "status": "unsupported-surface",
                                "reason": "WebSocket subscriptions require a concrete transport/runtime surface; the Type-Safe API starter only materializes fetch route-handler and dashboard workflow surfaces.",
                                "app_owned_boundary": "Subscription transport, connection authorization, stream fan-out, retry policy, and hosted runtime limits stay app-owned."
                            }
                        ],
                        "runtime_limitations": [
                            "ADAPTER-BOUNDARY: live tRPC WebSocket subscriptions stay app-owned until a real upstream-backed surface is selected."
                        ]
                    }
                ]
            }))
            .expect("Type-Safe API unsupported package-status json"),
        )
        .expect("write Type-Safe API unsupported package status");

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let type_safe_api = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == TYPE_SAFE_API_PACKAGE_ID)
            .expect("Type-Safe API row");
        let metric_value = |name: &str| -> u64 {
            type_safe_api["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(type_safe_api["status"], "unsupported-surface");
        assert_eq!(metric_value("type_safe_api_unsupported_surface"), 1);
        let unsupported_surface = type_safe_api["selected_surfaces"]
            .as_array()
            .expect("selected surfaces")
            .iter()
            .find(|surface| surface["surface_id"] == "trpc-websocket-subscriptions")
            .expect("unsupported surface row");
        assert_eq!(unsupported_surface["status"], "unsupported-surface");
        assert_eq!(
            unsupported_surface["reason"],
            "WebSocket subscriptions require a concrete transport/runtime surface; the Type-Safe API starter only materializes fetch route-handler and dashboard workflow surfaces."
        );
        assert_eq!(
            unsupported_surface["app_owned_boundary"],
            "Subscription transport, connection authorization, stream fan-out, retry policy, and hosted runtime limits stay app-owned."
        );
        assert!(
            type_safe_api["next_action"]
                .as_str()
                .expect("unsupported next action")
                .contains("Request only supported Type-Safe API")
        );
    }

    #[test]
    fn dx_check_latest_panel_exposes_ui_components_package_lane_hash_refresh_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let button_path = dir
            .path()
            .join("examples/template/components/ui/button.tsx");
        fs::create_dir_all(button_path.parent().expect("button parent")).expect("button dir");
        fs::write(&button_path, "export const buttonProbe = 'fresh';\n").expect("button source");
        let button_hash = sha256_file(&button_path);

        let ui_receipt_path = dir
            .path()
            .join("examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json");
        fs::create_dir_all(ui_receipt_path.parent().expect("ui receipt parent"))
            .expect("ui receipt dir");
        fs::write(&ui_receipt_path, "{}").expect("ui receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        fs::write(
            &package_status_path,
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
                            "helper_path": "examples/template/ui-components-receipt-hashes.ts",
                            "check_command": "node examples/template/ui-components-receipt-hashes.ts --check",
                            "write_command": "node examples/template/ui-components-receipt-hashes.ts --write",
                            "json_check_command": "node examples/template/ui-components-receipt-hashes.ts --check --json",
                            "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json",
                            "hash_algorithm": "sha256",
                            "tracked_file_count": 1,
                            "stale_file_count": 0,
                            "missing_file_count": 0,
                            "runtime_execution": false,
                            "secret_access": false,
                            "zed_visibility": "ui-components:receipt-hash-refresh",
                            "runtime_limitations": [
                                "SOURCE-ONLY: this helper checks local UI Components receipt hash freshness only.",
                                "ADAPTER-BOUNDARY: browser UI runtime proof stays app-owned."
                            ]
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
                                "files": [
                                    "components/ui/button.tsx"
                                ],
                                "source_markers": [
                                    "data-slot=\"button\"",
                                    "data-dx-package=\"shadcn/ui/button\""
                                ],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "examples/template/components/ui/button.tsx": button_hash
                                }
                            }
                        ],
                        "source_hashes": {
                            "algorithm": "sha256",
                            "files": {
                                "examples/template/components/ui/button.tsx": button_hash
                            }
                        },
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
                        ],
                        "runtime_limitations": [
                            "SOURCE-ONLY: package-lane visibility is receipt and source-marker evidence; no browser UI runtime proof is claimed."
                        ]
                    }
                ]
            }))
            .expect("ui components package-status json"),
        )
        .expect("write ui components package status");

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let ui_components = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "shadcn/ui/button")
            .expect("UI Components row");

        assert_eq!(ui_components["official_package_name"], "UI Components");
        assert_eq!(ui_components["upstream_package"], "shadcn-ui");
        assert_eq!(ui_components["upstream_version"], "0.0.1");
        assert_eq!(
            ui_components["source_mirror"],
            "G:/WWW/inspirations/shadcn-ui; G:/WWW/inspirations/radix-primitives"
        );
        assert_eq!(ui_components["status"], "present");
        assert_eq!(ui_components["receipt_status"], "present");
        assert_eq!(
            ui_components["package_receipt_path"],
            "examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json"
        );
        assert_eq!(
            ui_components["receipt_hash_refresh"]["schema"],
            "dx.forge.package.receipt_hash_refresh"
        );
        assert_eq!(ui_components["receipt_hash_refresh"]["status"], "current");
        assert_eq!(
            ui_components["receipt_hash_refresh"]["helper_path"],
            "examples/template/ui-components-receipt-hashes.ts"
        );
        assert_eq!(
            ui_components["receipt_hash_refresh"]["json_check_command"],
            "node examples/template/ui-components-receipt-hashes.ts --check --json"
        );
        assert_eq!(
            ui_components["receipt_hash_refresh"]["zed_visibility"],
            "ui-components:receipt-hash-refresh"
        );
        assert_eq!(
            ui_components["receipt_hash_refresh"]["runtime_execution"],
            false
        );
        assert_eq!(
            ui_components["receipt_hash_refresh"]["secret_access"],
            false
        );

        let metric_value = |name: &str| -> u64 {
            ui_components["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(metric_value("ui_components_package_present"), 1);
        assert_eq!(metric_value("ui_components_receipt_present"), 1);
        assert_eq!(metric_value("ui_components_receipt_stale"), 0);
        assert_eq!(metric_value("ui_components_missing_receipt"), 0);
        assert_eq!(metric_value("ui_components_blocked_surface"), 0);
        assert_eq!(metric_value("ui_components_unsupported_surface"), 0);
        assert_eq!(metric_value("ui_components_hash_manifest_present"), 1);
        assert_eq!(metric_value("ui_components_hash_mismatch"), 0);
        assert_eq!(
            metric_value("ui_components_receipt_hash_refresh_current"),
            1
        );
        assert_eq!(metric_value("ui_components_receipt_hash_refresh_stale"), 0);
        assert_eq!(
            metric_value("ui_components_receipt_hash_refresh_missing"),
            0
        );

        let mut stale_helper_package_status: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("read package status"))
                .expect("stale helper package status json");
        stale_helper_package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_file_count"] =
            serde_json::json!(2);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&stale_helper_package_status)
                .expect("stale helper package status bytes"),
        )
        .expect("write stale helper package status");

        let helper_stale_report = read_dx_check_latest_panel(dir.path());
        let helper_stale_view_model = serde_json::to_value(&helper_stale_report.view_model)
            .expect("helper stale view model json");
        let helper_stale_ui_components = helper_stale_view_model["package_lane_rows"]
            .as_array()
            .expect("helper stale package lane rows")
            .iter()
            .find(|row| row["package_id"] == "shadcn/ui/button")
            .expect("helper stale UI Components row");
        let helper_stale_metric_value = |name: &str| -> u64 {
            helper_stale_ui_components["metrics"]
                .as_array()
                .expect("helper stale metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from helper stale row"))
        };

        assert_eq!(helper_stale_ui_components["status"], "stale");
        assert_eq!(helper_stale_ui_components["receipt_status"], "stale");
        assert_eq!(
            helper_stale_metric_value("ui_components_receipt_hash_refresh_current"),
            0
        );
        assert_eq!(
            helper_stale_metric_value("ui_components_receipt_hash_refresh_stale"),
            1
        );
        assert_eq!(
            helper_stale_metric_value("ui_components_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(helper_stale_metric_value("ui_components_hash_mismatch"), 0);

        fs::write(&button_path, "export const buttonProbe = 'stale';\n")
            .expect("stale button source");

        let stale_report = read_dx_check_latest_panel(dir.path());
        let stale_view_model =
            serde_json::to_value(&stale_report.view_model).expect("stale view model json");
        let stale_ui_components = stale_view_model["package_lane_rows"]
            .as_array()
            .expect("stale package lane rows")
            .iter()
            .find(|row| row["package_id"] == "shadcn/ui/button")
            .expect("stale UI Components row");
        let stale_metric_value = |name: &str| -> u64 {
            stale_ui_components["metrics"]
                .as_array()
                .expect("stale metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from stale row"))
        };

        assert_eq!(stale_ui_components["status"], "stale");
        assert_eq!(stale_ui_components["receipt_status"], "stale");
        assert_eq!(stale_metric_value("ui_components_receipt_stale"), 1);
        assert_eq!(stale_metric_value("ui_components_hash_mismatch"), 2);
    }

    #[test]
    fn dx_check_latest_panel_exposes_payments_package_lane_hash_refresh_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let forge_source_path = dir.path().join("core/src/ecosystem/forge_stripe_js.rs");
        fs::create_dir_all(forge_source_path.parent().expect("forge parent")).expect("forge dir");
        fs::write(
            &forge_source_path,
            "export const paymentsForgeProbe = 'loadStripe';\n",
        )
        .expect("payments forge source");
        let forge_source_hash = sha256_file(&forge_source_path);

        let billing_status_path = dir.path().join("examples/template/payments-status.tsx");
        fs::create_dir_all(billing_status_path.parent().expect("billing status parent"))
            .expect("billing status dir");
        fs::write(
            &billing_status_path,
            "export const paymentsBillingProbe = 'fresh';\n",
        )
        .expect("payments billing status source");
        let billing_status_hash = sha256_file(&billing_status_path);

        let checkout_route_path = dir
            .path()
            .join("examples/template/app/api/payments/checkout/route.ts");
        fs::create_dir_all(checkout_route_path.parent().expect("checkout route parent"))
            .expect("checkout route dir");
        fs::write(
            &checkout_route_path,
            "export const paymentsCheckoutRouteProbe = 'fresh';\n",
        )
        .expect("payments checkout route source");
        let checkout_route_hash = sha256_file(&checkout_route_path);

        let webhook_route_path = dir
            .path()
            .join("examples/template/app/api/payments/webhook/route.ts");
        fs::create_dir_all(webhook_route_path.parent().expect("webhook route parent"))
            .expect("webhook route dir");
        fs::write(
            &webhook_route_path,
            "export const paymentsWebhookRouteProbe = 'fresh';\n",
        )
        .expect("payments webhook route source");
        let webhook_route_hash = sha256_file(&webhook_route_path);

        let payments_receipt_path = dir.path().join(
            "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json",
        );
        fs::create_dir_all(
            payments_receipt_path
                .parent()
                .expect("Payments receipt parent"),
        )
        .expect("Payments receipt dir");
        fs::write(&payments_receipt_path, "{}").expect("Payments receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": "Payments",
                        "package_id": "payments/stripe-js",
                        "upstream_package": "@stripe/stripe-js",
                        "upstream_version": "9.6.0",
                        "source_mirror": "G:/WWW/inspirations/stripe-js",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json",
                        "status_vocabulary": [
                            "present",
                            "stale",
                            "missing-receipt",
                            "blocked",
                            "unsupported-surface"
                        ],
                        "selected_surfaces": [
                            {
                                "surface_id": "payments-launch-billing-checkout-workflow",
                                "status": "present",
                                "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json",
                                "files": [
                                    "components/launch/payments-status.tsx"
                                ],
                                "source_markers": [
                                    "data-dx-package=\"payments/stripe-js\"",
                                    "data-dx-component=\"launch-billing-checkout-workflow\"",
                                    "data-dx-style-surface=\"payments\"",
                                    "data-dx-stripe-action=\"request-checkout-intent\""
                                ],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "examples/template/payments-status.tsx": billing_status_hash
                                }
                            },
                            {
                                "surface_id": "payments-checkout-session-route",
                                "status": "present",
                                "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json",
                                "files": [
                                    "app/api/payments/checkout/route.ts"
                                ],
                                "source_markers": [
                                    "data-dx-package=\"payments/stripe-js\"",
                                    "stripe.checkout.sessions.create",
                                    "stripe.confirmPayment"
                                ],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "examples/template/app/api/payments/checkout/route.ts": checkout_route_hash
                                }
                            },
                            {
                                "surface_id": "payments-webhook-route",
                                "status": "present",
                                "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json",
                                "files": [
                                    "app/api/payments/webhook/route.ts"
                                ],
                                "source_markers": [
                                    "data-dx-package=\"payments/stripe-js\"",
                                    "stripe.webhooks.constructEvent",
                                    "checkout.session.completed"
                                ],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "examples/template/app/api/payments/webhook/route.ts": webhook_route_hash
                                }
                            }
                        ],
                        "source_hashes": {
                            "algorithm": "sha256",
                            "files": {
                                "core/src/ecosystem/forge_stripe_js.rs": forge_source_hash
                            }
                        },
                        "receipt_hash_refresh": {
                            "schema": "dx.forge.package.receipt_hash_refresh",
                            "status": "current",
                            "helper_path": "examples/template/payments-receipt-hashes.ts",
                            "check_command": "node examples/template/payments-receipt-hashes.ts --check",
                            "write_command": "node examples/template/payments-receipt-hashes.ts --write",
                            "json_check_command": "node examples/template/payments-receipt-hashes.ts --check --json",
                            "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json",
                            "hash_algorithm": "sha256",
                            "tracked_file_count": 4,
                            "stale_file_count": 0,
                            "missing_file_count": 0,
                            "runtime_execution": false,
                            "secret_access": false,
                            "zed_visibility": "payments:receipt-hash-refresh",
                            "runtime_limitations": [
                                "SOURCE-ONLY: this helper checks local Payments receipt hash freshness only.",
                                "ADAPTER-BOUNDARY: live Stripe Checkout, webhook delivery, and billing fulfillment stay app-owned."
                            ]
                        },
                        "dx_style_compatibility": {
                            "schema": "dx.forge.package.dx_style_compatibility",
                            "status": "present",
                            "token_source": "examples/template/payments-status.tsx",
                            "visible_surfaces": [
                                "payments-launch-billing-checkout-workflow"
                            ],
                            "data_dx_markers": [
                                "data-dx-style-surface=\"payments\""
                            ],
                            "runtime_proof": false
                        },
                        "blocked_surfaces": [],
                        "unsupported_surfaces": [],
                        "dx_check_metrics": [
                            "payments_package_present",
                            "payments_receipt_present",
                            "payments_receipt_stale",
                            "payments_missing_receipt",
                            "payments_blocked_surface",
                            "payments_unsupported_surface",
                            "payments_hash_manifest_present",
                            "payments_hash_mismatch",
                            "payments_receipt_hash_refresh_current",
                            "payments_receipt_hash_refresh_stale",
                            "payments_receipt_hash_refresh_missing",
                            "payments_dx_style_compatibility_present",
                            "payments_dx_style_compatibility_missing"
                        ],
                        "runtime_limitations": [
                            "SOURCE-ONLY: package-lane visibility is receipt, source-marker, source-hash, helper, and dx-style evidence without claiming live Stripe Checkout or webhook runtime proof.",
                            "ADAPTER-BOUNDARY: Stripe credentials, Price IDs, customer lookup, webhook fulfillment, and entitlement policy stay app-owned."
                        ]
                    }
                ]
            }))
            .expect("Payments package-status json"),
        )
        .expect("write Payments package status");

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let payments = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "payments/stripe-js")
            .expect("Payments row");

        assert_eq!(payments["official_package_name"], "Payments");
        assert_eq!(payments["upstream_package"], "@stripe/stripe-js");
        assert_eq!(payments["upstream_version"], "9.6.0");
        assert_eq!(payments["source_mirror"], "G:/WWW/inspirations/stripe-js");
        assert_eq!(payments["status"], "present");
        assert_eq!(payments["receipt_status"], "present");
        assert_eq!(
            payments["package_receipt_path"],
            "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json"
        );
        assert!(
            payments["selected_surfaces"]
                .as_array()
                .expect("selected surfaces")
                .iter()
                .any(|surface| surface["surface_id"]
                    == "payments-launch-billing-checkout-workflow"
                    && surface["source_markers"]
                        .as_array()
                        .expect("source markers")
                        .iter()
                        .any(|marker| marker == "data-dx-style-surface=\"payments\""))
        );
        assert_eq!(
            payments["receipt_hash_refresh"]["schema"],
            "dx.forge.package.receipt_hash_refresh"
        );
        assert_eq!(payments["receipt_hash_refresh"]["status"], "current");
        assert_eq!(
            payments["receipt_hash_refresh"]["helper_path"],
            "examples/template/payments-receipt-hashes.ts"
        );
        assert_eq!(
            payments["receipt_hash_refresh"]["zed_visibility"],
            "payments:receipt-hash-refresh"
        );
        assert_eq!(payments["receipt_hash_refresh"]["runtime_execution"], false);
        assert_eq!(payments["receipt_hash_refresh"]["secret_access"], false);
        assert!(
            payments["next_action"]
                .as_str()
                .expect("next action")
                .contains("without claiming live Stripe Checkout or webhook runtime proof")
        );

        let metric_value = |name: &str| -> u64 {
            payments["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(metric_value("payments_package_present"), 1);
        assert_eq!(metric_value("payments_receipt_present"), 1);
        assert_eq!(metric_value("payments_receipt_stale"), 0);
        assert_eq!(metric_value("payments_missing_receipt"), 0);
        assert_eq!(metric_value("payments_blocked_surface"), 0);
        assert_eq!(metric_value("payments_unsupported_surface"), 0);
        assert_eq!(metric_value("payments_hash_manifest_present"), 1);
        assert_eq!(metric_value("payments_hash_mismatch"), 0);
        assert_eq!(metric_value("payments_receipt_hash_refresh_current"), 1);
        assert_eq!(metric_value("payments_receipt_hash_refresh_stale"), 0);
        assert_eq!(metric_value("payments_receipt_hash_refresh_missing"), 0);
        assert_eq!(metric_value("payments_dx_style_compatibility_present"), 1);
        assert_eq!(metric_value("payments_dx_style_compatibility_missing"), 0);

        fs::write(
            &billing_status_path,
            "export const paymentsBillingProbe = 'stale';\n",
        )
        .expect("stale Payments billing status source");

        let stale_report = read_dx_check_latest_panel(dir.path());
        let stale_view_model =
            serde_json::to_value(&stale_report.view_model).expect("stale view model json");
        let stale_payments = stale_view_model["package_lane_rows"]
            .as_array()
            .expect("stale package lane rows")
            .iter()
            .find(|row| row["package_id"] == "payments/stripe-js")
            .expect("stale Payments row");
        let stale_metric_value = |name: &str| -> u64 {
            stale_payments["metrics"]
                .as_array()
                .expect("stale metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from stale row"))
        };

        assert_eq!(stale_payments["status"], "stale");
        assert_eq!(stale_payments["receipt_status"], "stale");
        assert_eq!(stale_metric_value("payments_receipt_stale"), 1);
        assert_eq!(stale_metric_value("payments_hash_mismatch"), 1);
        assert_eq!(
            stale_metric_value("payments_receipt_hash_refresh_current"),
            1
        );
        assert_eq!(stale_metric_value("payments_receipt_hash_refresh_stale"), 0);
        assert_eq!(
            stale_metric_value("payments_receipt_hash_refresh_missing"),
            0
        );
    }

    #[test]
    fn dx_check_latest_panel_exposes_payments_stale_helper_without_source_hash_drift() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let billing_status_path = dir.path().join("examples/template/payments-status.tsx");
        fs::create_dir_all(billing_status_path.parent().expect("billing status parent"))
            .expect("billing status dir");
        fs::write(
            &billing_status_path,
            "export const paymentsBillingProbe = 'fresh';\n",
        )
        .expect("payments billing status source");
        let billing_status_hash = sha256_file(&billing_status_path);

        let payments_receipt_path = dir.path().join(
            "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json",
        );
        fs::create_dir_all(
            payments_receipt_path
                .parent()
                .expect("Payments receipt parent"),
        )
        .expect("Payments receipt dir");
        fs::write(&payments_receipt_path, "{}").expect("Payments receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": "Payments",
                        "package_id": "payments/stripe-js",
                        "upstream_package": "@stripe/stripe-js",
                        "upstream_version": "9.6.0",
                        "source_mirror": "G:/WWW/inspirations/stripe-js",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json",
                        "status_vocabulary": [
                            "present",
                            "stale",
                            "missing-receipt",
                            "blocked",
                            "unsupported-surface"
                        ],
                        "selected_surfaces": [
                            {
                                "surface_id": "payments-launch-billing-checkout-workflow",
                                "status": "present",
                                "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json",
                                "files": [
                                    "components/launch/payments-status.tsx"
                                ],
                                "source_markers": [
                                    "data-dx-package=\"payments/stripe-js\"",
                                    "data-dx-style-surface=\"payments\""
                                ],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "examples/template/payments-status.tsx": billing_status_hash
                                }
                            }
                        ],
                        "receipt_hash_refresh": {
                            "schema": "dx.forge.package.receipt_hash_refresh",
                            "status": "stale",
                            "helper_path": "examples/template/payments-receipt-hashes.ts",
                            "check_command": "node examples/template/payments-receipt-hashes.ts --check",
                            "write_command": "node examples/template/payments-receipt-hashes.ts --write",
                            "json_check_command": "node examples/template/payments-receipt-hashes.ts --check --json",
                            "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json",
                            "hash_algorithm": "sha256",
                            "tracked_file_count": 4,
                            "stale_file_count": 1,
                            "missing_file_count": 0,
                            "runtime_execution": false,
                            "secret_access": false,
                            "zed_visibility": "payments:receipt-hash-refresh"
                        },
                        "dx_style_compatibility": {
                            "schema": "dx.forge.package.dx_style_compatibility",
                            "status": "present",
                            "visible_surfaces": [
                                "payments-launch-billing-checkout-workflow"
                            ],
                            "runtime_proof": false
                        },
                        "blocked_surfaces": [],
                        "unsupported_surfaces": []
                    }
                ]
            }))
            .expect("Payments package-status json"),
        )
        .expect("write Payments package status");

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let payments = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "payments/stripe-js")
            .expect("Payments row");
        let metric_value = |name: &str| -> u64 {
            payments["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(payments["status"], "stale");
        assert_eq!(payments["receipt_status"], "stale");
        assert_eq!(metric_value("payments_receipt_stale"), 1);
        assert_eq!(metric_value("payments_hash_mismatch"), 0);
        assert_eq!(metric_value("payments_receipt_hash_refresh_current"), 0);
        assert_eq!(metric_value("payments_receipt_hash_refresh_stale"), 1);
        assert_eq!(metric_value("payments_receipt_hash_refresh_missing"), 0);
        assert!(
            payments["next_action"]
                .as_str()
                .expect("next action")
                .contains("payments-receipt-hashes.ts --write")
        );
    }

    #[test]
    fn dx_check_latest_panel_exposes_documentation_system_package_lane_hash_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let docs_path = dir.path().join("examples/template/content/docs/index.mdx");
        fs::create_dir_all(docs_path.parent().expect("docs parent")).expect("docs dir");
        fs::write(
            &docs_path,
            "# Launch Docs\n\nFresh documentation content.\n",
        )
        .expect("docs source");
        let docs_hash = sha256_file(&docs_path);

        let docs_status_path = dir.path().join("examples/template/docs-status.tsx");
        fs::create_dir_all(docs_status_path.parent().expect("status parent")).expect("status dir");
        fs::write(
            &docs_status_path,
            "export const documentationSystemStatusProbe = 'fresh';\n",
        )
        .expect("docs status source");
        let docs_status_hash = sha256_file(&docs_status_path);

        let package_receipt_path = dir.path().join(
            "examples/template/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json",
        );
        fs::create_dir_all(
            package_receipt_path
                .parent()
                .expect("documentation receipt parent"),
        )
        .expect("documentation receipt dir");
        fs::write(&package_receipt_path, "{}").expect("documentation receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": "Documentation System",
                        "package_id": "content/fumadocs-next",
                        "upstream_package": "fumadocs",
                        "upstream_version": "16.8.12",
                        "source_mirror": "G:/WWW/inspirations/fumadocs",
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json",
                        "status_vocabulary": [
                            "present",
                            "stale",
                            "missing-receipt",
                            "blocked",
                            "unsupported-surface"
                        ],
                        "selected_surfaces": [
                            {
                                "surface_id": "docs-app-router",
                                "status": "present",
                                "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json",
                                "files": [
                                    "content/docs/index.mdx"
                                ],
                                "source_markers": [
                                    "loader",
                                    "data-dx-package=\"content/fumadocs-next\""
                                ],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "examples/template/content/docs/index.mdx": docs_hash
                                }
                            }
                        ],
                        "source_hashes": {
                            "algorithm": "sha256",
                            "files": {
                                "examples/template/docs-status.tsx": docs_status_hash
                            }
                        },
                        "dx_style_compatibility": {
                            "schema": "dx.forge.package.dx_style_compatibility",
                            "status": "present",
                            "token_source": "styles/theme.css",
                            "generated_css": "styles/generated.css",
                            "visible_surfaces": [
                                "docs-app-router"
                            ],
                            "runtime_proof": false
                        },
                        "receipt_hash_refresh": {
                            "schema": "dx.forge.package.receipt_hash_refresh",
                            "status": "current",
                            "helper_path": "examples/template/documentation-system-receipt-hashes.ts",
                            "check_command": "node examples/template/documentation-system-receipt-hashes.ts --check",
                            "write_command": "node examples/template/documentation-system-receipt-hashes.ts --write",
                            "json_check_command": "node examples/template/documentation-system-receipt-hashes.ts --check --json",
                            "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json",
                            "hash_algorithm": "sha256",
                            "tracked_file_count": 2,
                            "stale_file_count": 0,
                            "missing_file_count": 0,
                            "runtime_execution": false,
                            "secret_access": false,
                            "zed_visibility": "documentation-system:receipt-hash-refresh",
                            "runtime_limitations": [
                                "SOURCE-ONLY: helper checks local Documentation System receipt hash freshness only."
                            ]
                        },
                        "blocked_surfaces": [],
                        "unsupported_surfaces": [],
                        "dx_check_metrics": [
                            "documentation_system_receipt_present",
                            "documentation_system_receipt_stale",
                            "documentation_system_missing_receipt",
                            "documentation_system_blocked_surface",
                            "documentation_system_unsupported_surface",
                            "documentation_system_hash_manifest_present",
                            "documentation_system_hash_mismatch",
                            "documentation_system_receipt_hash_refresh_current",
                            "documentation_system_receipt_hash_refresh_stale",
                            "documentation_system_receipt_hash_refresh_missing",
                            "documentation_system_dx_style_compatibility_present",
                            "documentation_system_dx_style_compatibility_missing"
                        ],
                        "runtime_limitations": [
                            "SOURCE-ONLY: package-lane visibility is receipt, source-marker, source-hash, and dx-style evidence; no live Fumadocs renderer proof is claimed."
                        ]
                    }
                ]
            }))
            .expect("documentation system package-status json"),
        )
        .expect("write documentation system package status");

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let documentation_system = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "content/fumadocs-next")
            .expect("Documentation System row");

        assert_eq!(
            documentation_system["official_package_name"],
            "Documentation System"
        );
        assert_eq!(documentation_system["upstream_package"], "fumadocs");
        assert_eq!(documentation_system["upstream_version"], "16.8.12");
        assert_eq!(
            documentation_system["source_mirror"],
            "G:/WWW/inspirations/fumadocs"
        );
        assert_eq!(documentation_system["status"], "present");
        assert_eq!(documentation_system["receipt_status"], "present");
        assert_eq!(
            documentation_system["package_receipt_path"],
            "examples/template/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json"
        );
        assert!(
            documentation_system["selected_surfaces"]
                .as_array()
                .expect("selected surfaces")
                .iter()
                .any(|surface| surface["surface_id"] == "docs-app-router")
        );

        let metric_value = |name: &str| -> u64 {
            documentation_system["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(metric_value("documentation_system_package_present"), 1);
        assert_eq!(metric_value("documentation_system_receipt_present"), 1);
        assert_eq!(metric_value("documentation_system_receipt_stale"), 0);
        assert_eq!(metric_value("documentation_system_missing_receipt"), 0);
        assert_eq!(metric_value("documentation_system_blocked_surface"), 0);
        assert_eq!(metric_value("documentation_system_unsupported_surface"), 0);
        assert_eq!(
            metric_value("documentation_system_hash_manifest_present"),
            1
        );
        assert_eq!(metric_value("documentation_system_hash_mismatch"), 0);
        assert_eq!(
            documentation_system["receipt_hash_refresh"]["schema"],
            "dx.forge.package.receipt_hash_refresh"
        );
        assert_eq!(
            documentation_system["receipt_hash_refresh"]["status"],
            "current"
        );
        assert_eq!(
            documentation_system["receipt_hash_refresh"]["helper_path"],
            "examples/template/documentation-system-receipt-hashes.ts"
        );
        assert_eq!(
            documentation_system["receipt_hash_refresh"]["zed_visibility"],
            "documentation-system:receipt-hash-refresh"
        );
        assert_eq!(
            metric_value("documentation_system_receipt_hash_refresh_current"),
            1
        );
        assert_eq!(
            metric_value("documentation_system_receipt_hash_refresh_stale"),
            0
        );
        assert_eq!(
            metric_value("documentation_system_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(
            metric_value("documentation_system_dx_style_compatibility_present"),
            1
        );
        assert_eq!(
            metric_value("documentation_system_dx_style_compatibility_missing"),
            0
        );

        let mut stale_helper_package_status: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("read package status"))
                .expect("stale helper Documentation System package status json");
        stale_helper_package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["status"] =
            serde_json::json!("stale");
        stale_helper_package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_file_count"] =
            serde_json::json!(1);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&stale_helper_package_status)
                .expect("stale helper Documentation System package status bytes"),
        )
        .expect("write stale helper Documentation System package status");

        let helper_stale_report = read_dx_check_latest_panel(dir.path());
        let helper_stale_view_model = serde_json::to_value(&helper_stale_report.view_model)
            .expect("helper stale view model json");
        let helper_stale_documentation_system = helper_stale_view_model["package_lane_rows"]
            .as_array()
            .expect("helper stale package lane rows")
            .iter()
            .find(|row| row["package_id"] == "content/fumadocs-next")
            .expect("helper stale Documentation System row");
        let helper_stale_metric_value = |name: &str| -> u64 {
            helper_stale_documentation_system["metrics"]
                .as_array()
                .expect("helper stale metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from helper stale row"))
        };

        assert_eq!(helper_stale_documentation_system["status"], "stale");
        assert_eq!(helper_stale_documentation_system["receipt_status"], "stale");
        assert_eq!(
            helper_stale_metric_value("documentation_system_receipt_hash_refresh_current"),
            0
        );
        assert_eq!(
            helper_stale_metric_value("documentation_system_receipt_hash_refresh_stale"),
            1
        );
        assert_eq!(
            helper_stale_metric_value("documentation_system_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(
            helper_stale_metric_value("documentation_system_hash_mismatch"),
            0
        );

        stale_helper_package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["status"] =
            serde_json::json!("current");
        stale_helper_package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_file_count"] =
            serde_json::json!(0);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&stale_helper_package_status)
                .expect("fresh helper Documentation System package status bytes"),
        )
        .expect("restore fresh helper Documentation System package status");

        fs::write(
            &docs_path,
            "# Launch Docs\n\nStale documentation content.\n",
        )
        .expect("stale docs source");

        let stale_report = read_dx_check_latest_panel(dir.path());
        let stale_view_model =
            serde_json::to_value(&stale_report.view_model).expect("stale view model json");
        let stale_documentation_system = stale_view_model["package_lane_rows"]
            .as_array()
            .expect("stale package lane rows")
            .iter()
            .find(|row| row["package_id"] == "content/fumadocs-next")
            .expect("stale Documentation System row");
        let stale_metric_value = |name: &str| -> u64 {
            stale_documentation_system["metrics"]
                .as_array()
                .expect("stale metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from stale row"))
        };

        assert_eq!(stale_documentation_system["status"], "stale");
        assert_eq!(stale_documentation_system["receipt_status"], "stale");
        assert_eq!(stale_metric_value("documentation_system_receipt_stale"), 1);
        assert_eq!(stale_metric_value("documentation_system_hash_mismatch"), 1);
        assert_eq!(
            stale_metric_value("documentation_system_receipt_hash_refresh_current"),
            1
        );
        assert_eq!(
            stale_metric_value("documentation_system_dx_style_compatibility_present"),
            1
        );
    }
