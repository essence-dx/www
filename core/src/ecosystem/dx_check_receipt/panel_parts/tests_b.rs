    #[test]
    fn dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_materialized_source_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let receipt_helper_path = dir.path().join("lib/markdown-mdx-content/receipt.ts");
        fs::create_dir_all(receipt_helper_path.parent().expect("receipt helper parent"))
            .expect("receipt helper dir");
        fs::write(
            &receipt_helper_path,
            "export const markdownMdxContentReceiptProbe = 'fresh';\n",
        )
        .expect("Markdown & MDX Content receipt helper source");
        let receipt_helper_hash = sha256_file(&receipt_helper_path);

        let package_receipt_path = dir
            .path()
            .join(".dx/forge/receipts/packages/content-react-markdown.json");
        fs::create_dir_all(
            package_receipt_path
                .parent()
                .expect("Markdown & MDX Content receipt parent"),
        )
        .expect("Markdown & MDX Content receipt dir");
        fs::write(&package_receipt_path, "{}").expect("Markdown & MDX Content receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        write_markdown_mdx_content_package_status(
            &package_status_path,
            &receipt_helper_hash,
            Some(serde_json::json!({
                "schema": "dx.forge.package.materialized_source",
                "source_file": "lib/markdown-mdx-content/receipt.ts",
                "materialized_file": "lib/markdown-mdx-content/receipt.ts",
                "surface": "forge-receipt-helper",
                "execution_guard": "dx run --test .\\benchmarks\\markdown-mdx-content-slice.test.ts",
                "runtime_proof": false
            })),
        );

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let markdown_mdx_content = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "content/react-markdown")
            .expect("Markdown & MDX Content row");

        assert_eq!(
            markdown_mdx_content["official_package_name"],
            "Markdown & MDX Content"
        );
        assert_eq!(
            markdown_mdx_content["upstream_package"],
            "react-markdown; @mdx-js/mdx; @mdx-js/react"
        );
        assert_eq!(
            markdown_mdx_content["upstream_version"],
            "react-markdown@10.1.0; @mdx-js/mdx@3.1.1; @mdx-js/react@3.1.1"
        );
        assert_eq!(
            markdown_mdx_content["source_mirror"],
            "G:/WWW/inspirations/react-markdown; G:/WWW/inspirations/mdx"
        );
        assert_eq!(markdown_mdx_content["status"], "present");
        assert_eq!(markdown_mdx_content["receipt_status"], "present");
        assert_eq!(
            markdown_mdx_content["package_receipt_path"],
            ".dx/forge/receipts/packages/content-react-markdown.json"
        );
        let selected_surfaces = markdown_mdx_content["selected_surfaces"]
            .as_array()
            .expect("selected surfaces");
        assert!(selected_surfaces.iter().any(|surface| {
            surface["surface_id"] == "forge-receipt-helper"
                && surface["source_markers"]
                    .as_array()
                    .expect("source markers")
                    .iter()
                    .any(|marker| marker == "createMarkdownMdxContentReceipt")
        }));
        assert!(selected_surfaces.iter().any(|surface| {
            surface["surface_id"] == "mdx-provider"
                && surface["source_markers"]
                    .as_array()
                    .expect("source markers")
                    .iter()
                    .any(|marker| marker == "data-dx-style-surface=\"markdown-mdx-content\"")
        }));

        let metric_value = |name: &str| -> u64 {
            markdown_mdx_content["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(metric_value("markdown_mdx_content_package_present"), 1);
        assert_eq!(metric_value("markdown_mdx_content_receipt_present"), 1);
        assert_eq!(metric_value("markdown_mdx_content_receipt_stale"), 0);
        assert_eq!(metric_value("markdown_mdx_content_missing_receipt"), 0);
        assert_eq!(
            metric_value("markdown_mdx_content_hash_manifest_present"),
            1
        );
        assert_eq!(metric_value("markdown_mdx_content_hash_mismatch"), 0);
        assert_eq!(
            metric_value("markdown_mdx_content_receipt_hash_refresh_current"),
            1
        );
        assert_eq!(
            metric_value("markdown_mdx_content_receipt_hash_refresh_stale"),
            0
        );
        assert_eq!(
            metric_value("markdown_mdx_content_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(
            metric_value("markdown_mdx_content_dx_style_compatibility_present"),
            1
        );
        assert_eq!(
            metric_value("markdown_mdx_content_dx_style_compatibility_missing"),
            0
        );
        assert_eq!(
            metric_value("markdown_mdx_content_materialized_source_present"),
            1
        );
        assert_eq!(
            metric_value("markdown_mdx_content_materialized_source_missing"),
            0
        );

        write_markdown_mdx_content_package_status(&package_status_path, &receipt_helper_hash, None);

        let missing_materialized_report = read_dx_check_latest_panel(dir.path());
        let missing_materialized_view_model =
            serde_json::to_value(&missing_materialized_report.view_model)
                .expect("missing materialized view model json");
        let missing_materialized_markdown = missing_materialized_view_model["package_lane_rows"]
            .as_array()
            .expect("missing materialized package lane rows")
            .iter()
            .find(|row| row["package_id"] == "content/react-markdown")
            .expect("missing materialized Markdown & MDX Content row");
        let missing_materialized_metric_value = |name: &str| -> u64 {
            missing_materialized_markdown["metrics"]
                .as_array()
                .expect("missing materialized metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from missing materialized row"))
        };

        assert_eq!(missing_materialized_markdown["status"], "present");
        assert_eq!(
            missing_materialized_metric_value("markdown_mdx_content_materialized_source_present"),
            0
        );
        assert_eq!(
            missing_materialized_metric_value("markdown_mdx_content_materialized_source_missing"),
            1
        );
        let missing_materialized_next_action = missing_materialized_markdown["next_action"]
            .as_str()
            .expect("next action");
        assert!(missing_materialized_next_action.contains("materializedSource row"));
        assert!(
            missing_materialized_next_action
                .contains("without claiming live Markdown/MDX renderer proof")
        );
    }

    #[test]
    fn dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_hash_refresh_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let receipt_helper_path = dir.path().join("lib/markdown-mdx-content/receipt.ts");
        fs::create_dir_all(receipt_helper_path.parent().expect("receipt helper parent"))
            .expect("receipt helper dir");
        fs::write(
            &receipt_helper_path,
            "export const markdownMdxContentReceiptProbe = 'fresh';\n",
        )
        .expect("Markdown & MDX Content receipt helper source");
        let receipt_helper_hash = sha256_file(&receipt_helper_path);

        let package_receipt_path = dir
            .path()
            .join(".dx/forge/receipts/packages/content-react-markdown.json");
        fs::create_dir_all(
            package_receipt_path
                .parent()
                .expect("Markdown & MDX Content receipt parent"),
        )
        .expect("Markdown & MDX Content receipt dir");
        fs::write(&package_receipt_path, "{}").expect("Markdown & MDX Content receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        write_markdown_mdx_content_package_status(
            &package_status_path,
            &receipt_helper_hash,
            Some(serde_json::json!({
                "schema": "dx.forge.package.materialized_source",
                "source_file": "lib/markdown-mdx-content/receipt.ts",
                "materialized_file": "lib/markdown-mdx-content/receipt.ts",
                "surface": "forge-receipt-helper",
                "execution_guard": "dx run --test .\\benchmarks\\markdown-mdx-content-slice.test.ts",
                "runtime_proof": false
            })),
        );

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let markdown_mdx_content = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "content/react-markdown")
            .expect("Markdown & MDX Content row");

        let metric_value = |name: &str| -> u64 {
            markdown_mdx_content["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(markdown_mdx_content["status"], "present");
        assert_eq!(markdown_mdx_content["receipt_status"], "present");
        assert_eq!(
            markdown_mdx_content["receipt_hash_refresh"]["schema"],
            "dx.forge.package.receipt_hash_refresh"
        );
        assert_eq!(
            markdown_mdx_content["receipt_hash_refresh"]["status"],
            "current"
        );
        assert_eq!(
            markdown_mdx_content["receipt_hash_refresh"]["helper_path"],
            "examples/template/markdown-mdx-content-receipt-hashes.ts"
        );
        assert_eq!(
            markdown_mdx_content["receipt_hash_refresh"]["json_check_command"],
            "node examples/template/markdown-mdx-content-receipt-hashes.ts --check --json"
        );
        assert_eq!(
            markdown_mdx_content["receipt_hash_refresh"]["zed_visibility"],
            "markdown-mdx-content:receipt-hash-refresh"
        );
        assert_eq!(
            metric_value("markdown_mdx_content_receipt_hash_refresh_current"),
            1
        );
        assert_eq!(
            metric_value("markdown_mdx_content_receipt_hash_refresh_stale"),
            0
        );
        assert_eq!(
            metric_value("markdown_mdx_content_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(metric_value("markdown_mdx_content_hash_mismatch"), 0);

        let mut stale_helper_package_status: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("package status"))
                .expect("package status json");
        stale_helper_package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["status"] =
            serde_json::json!("stale");
        stale_helper_package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_file_count"] =
            serde_json::json!(1);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&stale_helper_package_status)
                .expect("stale helper package status json"),
        )
        .expect("write stale helper package status");

        let stale_helper_report = read_dx_check_latest_panel(dir.path());
        let stale_helper_view_model =
            serde_json::to_value(&stale_helper_report.view_model).expect("view model json");
        let stale_helper_markdown = stale_helper_view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "content/react-markdown")
            .expect("stale helper Markdown & MDX Content row");
        let stale_helper_metric_value = |name: &str| -> u64 {
            stale_helper_markdown["metrics"]
                .as_array()
                .expect("stale helper metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from stale helper row"))
        };

        assert_eq!(stale_helper_markdown["status"], "stale");
        assert_eq!(stale_helper_markdown["receipt_status"], "stale");
        assert_eq!(
            stale_helper_markdown["receipt_hash_refresh"]["status"],
            "stale"
        );
        assert_eq!(
            stale_helper_metric_value("markdown_mdx_content_receipt_hash_refresh_current"),
            0
        );
        assert_eq!(
            stale_helper_metric_value("markdown_mdx_content_receipt_hash_refresh_stale"),
            1
        );
        assert_eq!(
            stale_helper_metric_value("markdown_mdx_content_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(
            stale_helper_metric_value("markdown_mdx_content_hash_mismatch"),
            0
        );
    }

    #[test]
    fn dx_check_latest_panel_exposes_ai_sdk_package_lane_hash_refresh_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let assistant_path = dir.path().join("examples/template/ai-chat-status.tsx");
        fs::create_dir_all(assistant_path.parent().expect("assistant workflow parent"))
            .expect("assistant workflow dir");
        fs::write(
            &assistant_path,
            "export const aiSdkAssistantProbe = 'fresh';\n",
        )
        .expect("assistant workflow source");
        let assistant_hash = sha256_file(&assistant_path);

        let ai_sdk_receipt_path = dir.path().join(AI_SDK_PACKAGE_RECEIPT_PATH);
        fs::create_dir_all(ai_sdk_receipt_path.parent().expect("AI SDK receipt parent"))
            .expect("AI SDK receipt dir");
        fs::write(
            &ai_sdk_receipt_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.forge.package_dashboard_workflow_receipt",
                "official_dx_package_name": AI_SDK_OFFICIAL_NAME,
                "package_id": AI_SDK_PACKAGE_ID,
                "upstream_package": AI_SDK_UPSTREAM_PACKAGE,
                "upstream_version": AI_SDK_UPSTREAM_VERSION,
                "source_mirror": AI_SDK_SOURCE_MIRROR,
                "hash_algorithm": "sha256",
                "file_hashes": {
                    "examples/template/ai-chat-status.tsx": assistant_hash
                },
                "runtime_limitations": [
                    "SOURCE-ONLY: fixture proves receipt/check-panel visibility only."
                ]
            }))
            .expect("AI SDK receipt json"),
        )
        .expect("write AI SDK package receipt");

        let package_status_path = dir.path().join(AI_SDK_PACKAGE_STATUS_PATH);
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        write_ai_sdk_package_status(&package_status_path, &assistant_hash);

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let ai_sdk = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == AI_SDK_PACKAGE_ID)
            .expect("AI SDK row");

        assert_eq!(ai_sdk["official_package_name"], "AI SDK");
        assert_eq!(ai_sdk["upstream_package"], "ai");
        assert_eq!(ai_sdk["upstream_version"], "7.0.0-canary.146");
        assert_eq!(ai_sdk["source_mirror"], "G:/WWW/inspirations/vercel-ai");
        assert_eq!(ai_sdk["status"], "present");
        assert_eq!(ai_sdk["receipt_status"], "present");
        assert_eq!(
            ai_sdk["package_receipt_path"],
            "examples/template/.dx/forge/receipts/2026-05-22-ai-vercel-ai-launch-assistant.json"
        );
        assert_eq!(
            ai_sdk["receipt_hash_refresh"]["schema"],
            "dx.forge.package.receipt_hash_refresh"
        );
        assert_eq!(ai_sdk["receipt_hash_refresh"]["status"], "current");
        assert_eq!(
            ai_sdk["receipt_hash_refresh"]["helper_path"],
            "examples/template/ai-sdk-receipt-hashes.ts"
        );
        assert_eq!(
            ai_sdk["receipt_hash_refresh"]["json_check_command"],
            "node examples/template/ai-sdk-receipt-hashes.ts --check --json"
        );
        assert_eq!(
            ai_sdk["receipt_hash_refresh"]["zed_visibility"],
            "ai-sdk:receipt-hash-refresh"
        );
        assert!(
            ai_sdk["selected_surfaces"]
                .as_array()
                .expect("selected surfaces")
                .iter()
                .any(|surface| {
                    surface["surface_id"] == "ai-launch-assistant-dashboard-workflow"
                        && surface["source_markers"]
                            .as_array()
                            .expect("source markers")
                            .iter()
                            .any(|marker| marker == "data-dx-style-surface=\"ai-sdk\"")
                })
        );

        let metric_value = |name: &str| -> u64 {
            ai_sdk["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(metric_value("ai_sdk_package_present"), 1);
        assert_eq!(metric_value("ai_sdk_receipt_present"), 1);
        assert_eq!(metric_value("ai_sdk_receipt_stale"), 0);
        assert_eq!(metric_value("ai_sdk_missing_receipt"), 0);
        assert_eq!(metric_value("ai_sdk_blocked_surface"), 0);
        assert_eq!(metric_value("ai_sdk_unsupported_surface"), 0);
        assert_eq!(metric_value("ai_sdk_hash_manifest_present"), 1);
        assert_eq!(metric_value("ai_sdk_hash_mismatch"), 0);
        assert_eq!(metric_value("ai_sdk_receipt_hash_refresh_current"), 1);
        assert_eq!(metric_value("ai_sdk_receipt_hash_refresh_stale"), 0);
        assert_eq!(metric_value("ai_sdk_receipt_hash_refresh_missing"), 0);
        assert_eq!(metric_value("ai_sdk_dx_style_compatibility_present"), 1);
        assert_eq!(metric_value("ai_sdk_dx_style_compatibility_missing"), 0);

        let mut stale_helper_ai_sdk: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("read package status"))
                .expect("stale helper AI SDK package status json");
        stale_helper_ai_sdk["package_lane_visibility"][0]["receipt_hash_refresh"]["status"] =
            serde_json::json!("stale");
        stale_helper_ai_sdk["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_file_count"] =
            serde_json::json!(1);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&stale_helper_ai_sdk)
                .expect("stale helper AI SDK package status bytes"),
        )
        .expect("write stale helper AI SDK package status");

        let stale_helper_report = read_dx_check_latest_panel(dir.path());
        let stale_helper_view_model =
            serde_json::to_value(&stale_helper_report.view_model).expect("stale view model json");
        let stale_helper_ai_sdk = stale_helper_view_model["package_lane_rows"]
            .as_array()
            .expect("stale package lane rows")
            .iter()
            .find(|row| row["package_id"] == AI_SDK_PACKAGE_ID)
            .expect("stale AI SDK row");
        let helper_stale_metric_value = |name: &str| -> u64 {
            stale_helper_ai_sdk["metrics"]
                .as_array()
                .expect("helper stale metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from helper stale row"))
        };

        assert_eq!(stale_helper_ai_sdk["status"], "stale");
        assert_eq!(stale_helper_ai_sdk["receipt_status"], "stale");
        assert_eq!(helper_stale_metric_value("ai_sdk_receipt_stale"), 1);
        assert_eq!(
            helper_stale_metric_value("ai_sdk_receipt_hash_refresh_current"),
            0
        );
        assert_eq!(
            helper_stale_metric_value("ai_sdk_receipt_hash_refresh_stale"),
            1
        );
        assert_eq!(
            helper_stale_metric_value("ai_sdk_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(helper_stale_metric_value("ai_sdk_hash_mismatch"), 0);
        assert!(
            stale_helper_ai_sdk["next_action"]
                .as_str()
                .expect("stale helper next action")
                .contains("ai-sdk-receipt-hashes.ts --write")
        );
    }

    #[test]
    fn dx_check_latest_panel_exposes_internationalization_package_lane_style_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let locale_workflow_path = dir
            .path()
            .join("examples/template/next-intl-dashboard-locale.tsx");
        fs::create_dir_all(
            locale_workflow_path
                .parent()
                .expect("locale workflow parent"),
        )
        .expect("locale workflow dir");
        fs::write(
            &locale_workflow_path,
            "export const intlWorkflowProbe = 'fresh';\n",
        )
        .expect("locale workflow source");
        let locale_workflow_hash = sha256_file(&locale_workflow_path);

        let intl_receipt_path = dir
            .path()
            .join(".dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json");
        fs::create_dir_all(intl_receipt_path.parent().expect("intl receipt parent"))
            .expect("intl receipt dir");
        fs::write(&intl_receipt_path, "{}").expect("intl receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        write_internationalization_package_status(
            &package_status_path,
            &locale_workflow_hash,
            Some(serde_json::json!({
                "schema": "dx.forge.package.dx_style_compatibility",
                "status": "present",
                "visible_surfaces": ["next-intl-dashboard-locale-workflow"]
            })),
        );

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let intl = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "i18n/next-intl")
            .expect("Internationalization row");

        assert_eq!(intl["official_package_name"], "Internationalization");
        assert_eq!(intl["upstream_package"], "next-intl");
        assert_eq!(intl["upstream_version"], "4.12.0");
        assert_eq!(intl["source_mirror"], "G:/WWW/inspirations/next-intl");
        assert_eq!(intl["status"], "present");
        assert_eq!(intl["receipt_status"], "present");
        assert_eq!(
            intl["package_receipt_path"],
            "examples/template/.dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json"
        );
        assert!(
            intl["selected_surfaces"]
                .as_array()
                .expect("selected surfaces")
                .iter()
                .any(
                    |surface| surface["surface_id"] == "next-intl-dashboard-locale-workflow"
                        && surface["source_markers"]
                            .as_array()
                            .expect("source markers")
                            .iter()
                            .any(|marker| {
                                marker == "data-dx-style-surface=\"internationalization\""
                            }),
                )
        );

        let metric_value = |name: &str| -> u64 {
            intl["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(metric_value("internationalization_package_present"), 1);
        assert_eq!(metric_value("internationalization_receipt_present"), 1);
        assert_eq!(metric_value("internationalization_receipt_stale"), 0);
        assert_eq!(metric_value("internationalization_missing_receipt"), 0);
        assert_eq!(metric_value("internationalization_blocked_surface"), 0);
        assert_eq!(metric_value("internationalization_unsupported_surface"), 0);
        assert_eq!(
            metric_value("internationalization_hash_manifest_present"),
            1
        );
        assert_eq!(metric_value("internationalization_hash_mismatch"), 0);
        assert_eq!(
            intl["receipt_hash_refresh"]["schema"],
            "dx.forge.package.receipt_hash_refresh"
        );
        assert_eq!(intl["receipt_hash_refresh"]["status"], "current");
        assert_eq!(
            intl["receipt_hash_refresh"]["helper_path"],
            "examples/template/internationalization-receipt-hashes.ts"
        );
        assert_eq!(
            intl["receipt_hash_refresh"]["json_check_command"],
            "node examples/template/internationalization-receipt-hashes.ts --check --json"
        );
        assert_eq!(
            intl["receipt_hash_refresh"]["zed_visibility"],
            "internationalization:receipt-hash-refresh"
        );
        assert_eq!(
            metric_value("internationalization_receipt_hash_refresh_current"),
            1
        );
        assert_eq!(
            metric_value("internationalization_receipt_hash_refresh_stale"),
            0
        );
        assert_eq!(
            metric_value("internationalization_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(
            metric_value("internationalization_dx_style_compatibility_present"),
            1
        );
        assert_eq!(
            metric_value("internationalization_dx_style_compatibility_missing"),
            0
        );

        let mut stale_helper_package_status: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("read package status"))
                .expect("stale helper Internationalization package status json");
        stale_helper_package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["status"] =
            serde_json::json!("stale");
        stale_helper_package_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_file_count"] =
            serde_json::json!(1);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&stale_helper_package_status)
                .expect("stale helper Internationalization package status bytes"),
        )
        .expect("write stale helper Internationalization package status");

        let stale_helper_internationalization_report = read_dx_check_latest_panel(dir.path());
        let stale_helper_internationalization_view_model =
            serde_json::to_value(&stale_helper_internationalization_report.view_model)
                .expect("stale helper Internationalization view model json");
        let stale_helper_internationalization =
            stale_helper_internationalization_view_model["package_lane_rows"]
                .as_array()
                .expect("stale helper package lane rows")
                .iter()
                .find(|row| row["package_id"] == "i18n/next-intl")
                .expect("stale helper Internationalization row");
        let helper_stale_metric_value = |name: &str| -> u64 {
            stale_helper_internationalization["metrics"]
                .as_array()
                .expect("stale helper metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from stale helper row"))
        };

        assert_eq!(stale_helper_internationalization["status"], "stale");
        assert_eq!(stale_helper_internationalization["receipt_status"], "stale");
        assert_eq!(
            helper_stale_metric_value("internationalization_receipt_hash_refresh_current"),
            0
        );
        assert_eq!(
            helper_stale_metric_value("internationalization_receipt_hash_refresh_stale"),
            1
        );
        assert_eq!(
            helper_stale_metric_value("internationalization_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(
            helper_stale_metric_value("internationalization_hash_mismatch"),
            0
        );
        assert!(
            stale_helper_internationalization["next_action"]
                .as_str()
                .expect("stale helper next action")
                .contains("internationalization-receipt-hashes.ts --write")
        );

        write_internationalization_package_status(
            &package_status_path,
            &locale_workflow_hash,
            None,
        );

        let missing_style_report = read_dx_check_latest_panel(dir.path());
        let missing_style_view_model = serde_json::to_value(&missing_style_report.view_model)
            .expect("missing style view model json");
        let missing_style_intl = missing_style_view_model["package_lane_rows"]
            .as_array()
            .expect("missing style package lane rows")
            .iter()
            .find(|row| row["package_id"] == "i18n/next-intl")
            .expect("missing style Internationalization row");
        let missing_style_metric_value = |name: &str| -> u64 {
            missing_style_intl["metrics"]
                .as_array()
                .expect("missing style metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from missing style row"))
        };

        assert_eq!(
            missing_style_metric_value("internationalization_dx_style_compatibility_present"),
            0
        );
        assert_eq!(
            missing_style_metric_value("internationalization_dx_style_compatibility_missing"),
            1
        );
        assert!(
            missing_style_intl["next_action"]
                .as_str()
                .expect("next action")
                .contains("dx-style compatibility")
        );
    }

    #[test]
    fn dx_check_latest_panel_exposes_realtime_app_database_package_lane_style_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let runtime_path = dir
            .path()
            .join("tools/launch/runtime-template/pages/index.html");
        fs::create_dir_all(runtime_path.parent().expect("runtime parent")).expect("runtime dir");
        fs::write(
            &runtime_path,
            r#"<section data-dx-package="instantdb/react" data-dx-component="instantdb-runtime-dashboard-workflow" data-dx-style-surface="realtime-app-database"></section>"#,
        )
        .expect("runtime page");
        let runtime_hash = sha256_file(&runtime_path);

        let dashboard_path = dir
            .path()
            .join("examples/dashboard/src/components/InstantDbDashboardWorkflow.tsx");
        fs::create_dir_all(dashboard_path.parent().expect("dashboard parent"))
            .expect("dashboard dir");
        fs::write(
            &dashboard_path,
            "export function InstantDbDashboardWorkflow() { return null; }\n",
        )
        .expect("dashboard component");
        let dashboard_hash = sha256_file(&dashboard_path);

        let package_receipt_path = dir.path().join(
            "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json",
        );
        fs::create_dir_all(package_receipt_path.parent().expect("receipt parent"))
            .expect("receipt dir");
        fs::write(&package_receipt_path, "{}").expect("Realtime App Database receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        write_realtime_app_database_package_status(
            &package_status_path,
            &runtime_hash,
            &dashboard_hash,
            Some(serde_json::json!({
                "schema": "dx.forge.package.dx_style_compatibility",
                "status": "present",
                "visible_surfaces": [
                    "instantdb-runtime-dashboard-workflow",
                    "dashboard-instantdb-workflow"
                ]
            })),
        );

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let realtime = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "instantdb/react")
            .expect("Realtime App Database row");

        assert_eq!(realtime["official_package_name"], "Realtime App Database");
        assert_eq!(realtime["upstream_package"], "@instantdb/react");
        assert_eq!(realtime["upstream_version"], "0.0.0");
        assert_eq!(realtime["source_mirror"], "G:/WWW/inspirations/instantdb");
        assert_eq!(realtime["status"], "present");
        assert_eq!(realtime["receipt_status"], "present");
        assert_eq!(
            realtime["package_receipt_path"],
            "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json"
        );
        assert!(
            realtime["selected_surfaces"]
                .as_array()
                .expect("selected surfaces")
                .iter()
                .any(
                    |surface| surface["surface_id"] == "instantdb-runtime-dashboard-workflow"
                        && surface["source_markers"]
                            .as_array()
                            .expect("source markers")
                            .iter()
                            .any(|marker| marker
                                == "data-dx-style-surface=\"realtime-app-database\""),
                )
        );

        let metric_value = |name: &str| -> u64 {
            realtime["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(metric_value("realtime_app_database_package_present"), 1);
        assert_eq!(metric_value("realtime_app_database_receipt_present"), 1);
        assert_eq!(metric_value("realtime_app_database_receipt_stale"), 0);
        assert_eq!(metric_value("realtime_app_database_missing_receipt"), 0);
        assert_eq!(metric_value("realtime_app_database_blocked_surface"), 0);
        assert_eq!(metric_value("realtime_app_database_unsupported_surface"), 0);
        assert_eq!(
            metric_value("realtime_app_database_hash_manifest_present"),
            1
        );
        assert_eq!(metric_value("realtime_app_database_hash_mismatch"), 0);
        assert_eq!(
            realtime["receipt_hash_refresh"]["schema"],
            "dx.forge.package.receipt_hash_refresh"
        );
        assert_eq!(realtime["receipt_hash_refresh"]["status"], "current");
        assert_eq!(
            realtime["receipt_hash_refresh"]["helper_path"],
            "examples/template/realtime-app-database-receipt-hashes.ts"
        );
        assert_eq!(
            realtime["receipt_hash_refresh"]["json_check_command"],
            "node examples/template/realtime-app-database-receipt-hashes.ts --check --json"
        );
        assert_eq!(
            realtime["receipt_hash_refresh"]["zed_visibility"],
            "realtime-app-database:receipt-hash-refresh"
        );
        assert_eq!(
            metric_value("realtime_app_database_receipt_hash_refresh_current"),
            1
        );
        assert_eq!(
            metric_value("realtime_app_database_receipt_hash_refresh_stale"),
            0
        );
        assert_eq!(
            metric_value("realtime_app_database_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(
            metric_value("realtime_app_database_dx_style_compatibility_present"),
            1
        );
        assert_eq!(
            metric_value("realtime_app_database_dx_style_compatibility_missing"),
            0
        );

        write_realtime_app_database_package_status(
            &package_status_path,
            &runtime_hash,
            &dashboard_hash,
            None,
        );

        let missing_style_report = read_dx_check_latest_panel(dir.path());
        let missing_style_view_model = serde_json::to_value(&missing_style_report.view_model)
            .expect("missing style view model json");
        let missing_style_realtime = missing_style_view_model["package_lane_rows"]
            .as_array()
            .expect("missing style package lane rows")
            .iter()
            .find(|row| row["package_id"] == "instantdb/react")
            .expect("missing style Realtime App Database row");
        let missing_style_metric_value = |name: &str| -> u64 {
            missing_style_realtime["metrics"]
                .as_array()
                .expect("missing style metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from missing style row"))
        };

        assert_eq!(
            missing_style_metric_value("realtime_app_database_dx_style_compatibility_present"),
            0
        );
        assert_eq!(
            missing_style_metric_value("realtime_app_database_dx_style_compatibility_missing"),
            1
        );
        assert!(
            missing_style_realtime["next_action"]
                .as_str()
                .expect("next action")
                .contains("hosted Instant runtime proof")
        );
    }

    #[test]
    fn dx_check_latest_panel_exposes_realtime_app_database_package_lane_hash_refresh_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let runtime_path = dir
            .path()
            .join("tools/launch/runtime-template/pages/index.html");
        fs::create_dir_all(runtime_path.parent().expect("runtime parent")).expect("runtime dir");
        fs::write(
            &runtime_path,
            r#"<section data-dx-package="instantdb/react" data-dx-component="instantdb-runtime-dashboard-workflow" data-dx-instant-action="prepare-local-schema-receipt" data-dx-style-surface="realtime-app-database"></section>"#,
        )
        .expect("runtime page");
        let runtime_hash = sha256_file(&runtime_path);

        let dashboard_path = dir
            .path()
            .join("examples/dashboard/src/components/InstantDbDashboardWorkflow.tsx");
        fs::create_dir_all(dashboard_path.parent().expect("dashboard parent"))
            .expect("dashboard dir");
        fs::write(
            &dashboard_path,
            r#"export function InstantDbDashboardWorkflow() { return <section data-dx-package="instantdb/react" data-dx-component="dashboard-instantdb-workflow" data-dx-instant-dashboard-workflow="realtime-boundary" data-dx-style-surface="realtime-app-database" />; }
"#,
        )
        .expect("dashboard component");
        let dashboard_hash = sha256_file(&dashboard_path);

        let package_receipt_path = dir.path().join(
            "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json",
        );
        fs::create_dir_all(package_receipt_path.parent().expect("receipt parent"))
            .expect("receipt dir");
        fs::write(&package_receipt_path, "{}").expect("Realtime App Database receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        write_realtime_app_database_package_status(
            &package_status_path,
            &runtime_hash,
            &dashboard_hash,
            Some(serde_json::json!({
                "schema": "dx.forge.package.dx_style_compatibility",
                "status": "present",
                "visible_surfaces": [
                    "instantdb-runtime-dashboard-workflow",
                    "dashboard-instantdb-workflow"
                ]
            })),
        );

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let realtime = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "instantdb/react")
            .expect("Realtime App Database row");
        let metric_value = |name: &str| -> u64 {
            realtime["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(realtime["status"], "present");
        assert_eq!(realtime["receipt_status"], "present");
        assert_eq!(
            realtime["receipt_hash_refresh"]["schema"],
            "dx.forge.package.receipt_hash_refresh"
        );
        assert_eq!(realtime["receipt_hash_refresh"]["status"], "current");
        assert_eq!(
            realtime["receipt_hash_refresh"]["helper_path"],
            "examples/template/realtime-app-database-receipt-hashes.ts"
        );
        assert_eq!(
            realtime["receipt_hash_refresh"]["json_check_command"],
            "node examples/template/realtime-app-database-receipt-hashes.ts --check --json"
        );
        assert_eq!(
            realtime["receipt_hash_refresh"]["zed_visibility"],
            "realtime-app-database:receipt-hash-refresh"
        );
        assert_eq!(metric_value("realtime_app_database_hash_mismatch"), 0);
        assert_eq!(
            metric_value("realtime_app_database_receipt_hash_refresh_current"),
            1
        );
        assert_eq!(
            metric_value("realtime_app_database_receipt_hash_refresh_stale"),
            0
        );
        assert_eq!(
            metric_value("realtime_app_database_receipt_hash_refresh_missing"),
            0
        );

        let mut stale_helper_realtime_app_database: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("package status bytes"))
                .expect("package status json");
        stale_helper_realtime_app_database["package_lane_visibility"][0]["receipt_hash_refresh"]
            ["status"] = serde_json::json!("stale");
        stale_helper_realtime_app_database["package_lane_visibility"][0]["receipt_hash_refresh"]
            ["stale_file_count"] = serde_json::json!(1);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&stale_helper_realtime_app_database)
                .expect("stale helper Realtime App Database package-status json"),
        )
        .expect("write stale helper Realtime App Database package status");

        let stale_helper_report = read_dx_check_latest_panel(dir.path());
        let stale_helper_view_model = serde_json::to_value(&stale_helper_report.view_model)
            .expect("stale helper view model json");
        let stale_helper_realtime = stale_helper_view_model["package_lane_rows"]
            .as_array()
            .expect("stale helper package lane rows")
            .iter()
            .find(|row| row["package_id"] == "instantdb/react")
            .expect("stale helper Realtime App Database row");
        let stale_helper_metric_value = |name: &str| -> u64 {
            stale_helper_realtime["metrics"]
                .as_array()
                .expect("stale helper metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from stale helper row"))
        };

        assert_eq!(stale_helper_realtime["status"], "stale");
        assert_eq!(stale_helper_realtime["receipt_status"], "stale");
        assert_eq!(
            stale_helper_realtime["receipt_hash_refresh"]["status"],
            "stale"
        );
        assert_eq!(
            stale_helper_realtime["receipt_hash_refresh"]["stale_file_count"]
                .as_u64()
                .expect("stale file count"),
            1
        );
        assert_eq!(
            stale_helper_metric_value("realtime_app_database_hash_mismatch"),
            0
        );
        assert_eq!(
            stale_helper_metric_value("realtime_app_database_receipt_hash_refresh_current"),
            0
        );
        assert_eq!(
            stale_helper_metric_value("realtime_app_database_receipt_hash_refresh_stale"),
            1
        );
        assert_eq!(
            stale_helper_metric_value("realtime_app_database_receipt_hash_refresh_missing"),
            0
        );
        assert!(
            stale_helper_realtime["next_action"]
                .as_str()
                .expect("next action")
                .contains("realtime-app-database-receipt-hashes.ts --write")
        );
    }

    #[test]
    fn dx_check_latest_panel_exposes_database_orm_package_lane_style_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let replica_source_path = dir.path().join("core/src/ecosystem/forge_drizzle.rs");
        fs::create_dir_all(replica_source_path.parent().expect("replica source parent"))
            .expect("replica source dir");
        fs::write(
            &replica_source_path,
            "export function createDxDrizzleReplicaSet() {}\n",
        )
        .expect("replica source");
        let replica_source_hash = sha256_file(&replica_source_path);

        let workflow_path = dir.path().join("examples/template/drizzle-query-proof.tsx");
        fs::create_dir_all(workflow_path.parent().expect("workflow parent")).expect("workflow dir");
        fs::write(
            &workflow_path,
            "export const drizzleWorkflowProbe = 'fresh';\n",
        )
        .expect("workflow source");
        let workflow_hash = sha256_file(&workflow_path);

        let package_receipt_path = dir.path().join(
            "examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json",
        );
        fs::create_dir_all(package_receipt_path.parent().expect("receipt parent"))
            .expect("receipt dir");
        fs::write(&package_receipt_path, "{}").expect("Database ORM receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        write_database_orm_package_status(
            &package_status_path,
            &replica_source_hash,
            &workflow_hash,
            Some(serde_json::json!({
                "schema": "dx.forge.package.dx_style_compatibility",
                "status": "present",
                "visible_surfaces": ["launch-drizzle-data-workflow"]
            })),
        );

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let database_orm = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "db/drizzle-sqlite")
            .expect("Database ORM row");

        assert_eq!(database_orm["official_package_name"], "Database ORM");
        assert_eq!(database_orm["upstream_package"], "drizzle-orm");
        assert_eq!(database_orm["upstream_version"], "0.45.3");
        assert_eq!(
            database_orm["source_mirror"],
            "G:/WWW/inspirations/drizzle-orm"
        );
        assert_eq!(database_orm["status"], "present");
        assert_eq!(database_orm["receipt_status"], "present");
        assert_eq!(
            database_orm["package_receipt_path"],
            "examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json"
        );
        assert!(
            database_orm["selected_surfaces"]
                .as_array()
                .expect("selected surfaces")
                .iter()
                .any(
                    |surface| surface["surface_id"] == "drizzle-launch-dashboard-workflow"
                        && surface["source_markers"]
                            .as_array()
                            .expect("source markers")
                            .iter()
                            .any(|marker| marker == "data-dx-style-surface=\"database-orm\""),
                )
        );

        let metric_value = |name: &str| -> u64 {
            database_orm["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(metric_value("database_orm_package_present"), 1);
        assert_eq!(metric_value("database_orm_receipt_present"), 1);
        assert_eq!(metric_value("database_orm_receipt_stale"), 0);
        assert_eq!(metric_value("database_orm_missing_receipt"), 0);
        assert_eq!(metric_value("database_orm_blocked_surface"), 0);
        assert_eq!(metric_value("database_orm_unsupported_surface"), 0);
        assert_eq!(metric_value("database_orm_hash_manifest_present"), 1);
        assert_eq!(metric_value("database_orm_hash_mismatch"), 0);
        assert_eq!(metric_value("database_orm_receipt_hash_refresh_current"), 1);
        assert_eq!(metric_value("database_orm_receipt_hash_refresh_stale"), 0);
        assert_eq!(metric_value("database_orm_receipt_hash_refresh_missing"), 0);
        assert_eq!(
            database_orm["receipt_hash_refresh"]["zed_visibility"],
            "database-orm:receipt-hash-refresh"
        );
        assert_eq!(
            database_orm["receipt_hash_refresh"]["json_check_command"],
            "node examples/template/database-orm-receipt-hashes.ts --check --json"
        );
        assert_eq!(
            metric_value("database_orm_dx_style_compatibility_present"),
            1
        );
        assert_eq!(
            metric_value("database_orm_dx_style_compatibility_missing"),
            0
        );

        let mut stale_helper_database_orm: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("package status bytes"))
                .expect("package status json");
        stale_helper_database_orm["package_lane_visibility"][0]["receipt_hash_refresh"]["status"] =
            serde_json::json!("stale");
        stale_helper_database_orm["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_file_count"] =
            serde_json::json!(1);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&stale_helper_database_orm)
                .expect("stale helper Database ORM package-status json"),
        )
        .expect("write stale helper Database ORM package status");

        let stale_helper_report = read_dx_check_latest_panel(dir.path());
        let stale_helper_view_model = serde_json::to_value(&stale_helper_report.view_model)
            .expect("stale helper view model json");
        let stale_helper_database_orm = stale_helper_view_model["package_lane_rows"]
            .as_array()
            .expect("stale helper package lane rows")
            .iter()
            .find(|row| row["package_id"] == "db/drizzle-sqlite")
            .expect("stale helper Database ORM row");
        let stale_helper_metric_value = |name: &str| -> u64 {
            stale_helper_database_orm["metrics"]
                .as_array()
                .expect("stale helper metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from stale helper row"))
        };

        assert_eq!(stale_helper_database_orm["status"], "stale");
        assert_eq!(stale_helper_database_orm["receipt_status"], "stale");
        assert_eq!(stale_helper_metric_value("database_orm_hash_mismatch"), 0);
        assert_eq!(
            stale_helper_metric_value("database_orm_receipt_hash_refresh_current"),
            0
        );
        assert_eq!(
            stale_helper_metric_value("database_orm_receipt_hash_refresh_stale"),
            1
        );
        assert_eq!(
            stale_helper_metric_value("database_orm_receipt_hash_refresh_missing"),
            0
        );
        assert!(
            stale_helper_database_orm["next_action"]
                .as_str()
                .expect("next action")
                .contains("database-orm-receipt-hashes.ts --write")
        );

        write_database_orm_package_status(
            &package_status_path,
            &replica_source_hash,
            &workflow_hash,
            None,
        );

        let missing_style_report = read_dx_check_latest_panel(dir.path());
        let missing_style_view_model = serde_json::to_value(&missing_style_report.view_model)
            .expect("missing style view model json");
        let missing_style_database_orm = missing_style_view_model["package_lane_rows"]
            .as_array()
            .expect("missing style package lane rows")
            .iter()
            .find(|row| row["package_id"] == "db/drizzle-sqlite")
            .expect("missing style Database ORM row");
        let missing_style_metric_value = |name: &str| -> u64 {
            missing_style_database_orm["metrics"]
                .as_array()
                .expect("missing style metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from missing style row"))
        };

        assert_eq!(
            missing_style_metric_value("database_orm_dx_style_compatibility_present"),
            0
        );
        assert_eq!(
            missing_style_metric_value("database_orm_dx_style_compatibility_missing"),
            1
        );
        assert!(
            missing_style_database_orm["next_action"]
                .as_str()
                .expect("next action")
                .contains("dx-style compatibility")
        );
    }

    #[test]
    fn dx_check_latest_panel_exposes_three_scene_system_package_lane_style_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let launch_scene_path = dir.path().join("examples/template/launch-scene.tsx");
        fs::create_dir_all(launch_scene_path.parent().expect("launch scene parent"))
            .expect("launch scene dir");
        fs::write(
            &launch_scene_path,
            "export const threeSceneSystemProbe = 'fresh';\n",
        )
        .expect("launch scene source");
        let launch_scene_hash = sha256_file(&launch_scene_path);

        let scene_receipt_path = dir
            .path()
            .join(".dx/forge/receipts/3d-launch-scene-dashboard-workflow.json");
        fs::create_dir_all(scene_receipt_path.parent().expect("3D receipt parent"))
            .expect("3D receipt dir");
        fs::write(&scene_receipt_path, "{}").expect("3D receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        write_three_scene_system_package_status(
            &package_status_path,
            &launch_scene_hash,
            Some(serde_json::json!({
                "schema": "dx.forge.package.dx_style_compatibility",
                "status": "present",
                "token_source": "examples/template/launch-scene.tsx",
                "generated_css": "tools/launch/runtime-template/assets/launch-runtime.css",
                "visible_surfaces": [
                    "launch-scene-webgl-proof",
                    "launch-scene-dashboard-workflow"
                ],
                "data_dx_markers": [
                    "data-dx-style-surface=\"launch-scene\"",
                    "data-dx-token-scope=\"3d/launch-scene\""
                ],
                "runtime_proof": false
            })),
        );

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let scene = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "3d/launch-scene")
            .expect("3D Scene System row");

        assert_eq!(scene["official_package_name"], "3D Scene System");
        assert_eq!(
            scene["upstream_package"],
            "three + @react-three/fiber + @react-three/drei"
        );
        assert_eq!(
            scene["upstream_version"],
            "three 0.184.0; @react-three/fiber 9.6.1; @react-three/drei local mirror"
        );
        assert_eq!(
            scene["source_mirror"],
            "G:/WWW/inspirations/three.js; G:/WWW/inspirations/react-three-fiber; G:/WWW/inspirations/drei"
        );
        assert_eq!(scene["status"], "present");
        assert_eq!(scene["receipt_status"], "present");
        assert_eq!(
            scene["package_receipt_path"],
            ".dx/forge/receipts/3d-launch-scene-dashboard-workflow.json"
        );
        assert_eq!(
            scene["receipt_hash_refresh"]["schema"],
            "dx.forge.package.receipt_hash_refresh"
        );
        assert_eq!(scene["receipt_hash_refresh"]["status"], "current");
        assert_eq!(
            scene["receipt_hash_refresh"]["helper_path"],
            "examples/template/3d-scene-system-receipt-hashes.ts"
        );
        assert_eq!(
            scene["receipt_hash_refresh"]["zed_visibility"],
            "3d-scene-system:receipt-hash-refresh"
        );
        assert!(
            scene["selected_surfaces"]
                .as_array()
                .expect("selected surfaces")
                .iter()
                .any(
                    |surface| surface["surface_id"] == "launch-scene-dashboard-workflow"
                        && surface["source_markers"]
                            .as_array()
                            .expect("source markers")
                            .iter()
                            .any(|marker| marker == "data-dx-style-surface=\"launch-scene\"")
                        && surface["source_markers"]
                            .as_array()
                            .expect("source markers")
                            .iter()
                            .any(|marker| marker == "data-dx-token-scope=\"3d/launch-scene\""),
                )
        );

        let metric_value = |name: &str| -> u64 {
            scene["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(metric_value("three_scene_system_receipt_present"), 1);
        assert_eq!(metric_value("three_scene_system_receipt_stale"), 0);
        assert_eq!(metric_value("three_scene_system_missing_receipt"), 0);
        assert_eq!(metric_value("three_scene_system_blocked_surface"), 0);
        assert_eq!(metric_value("three_scene_system_unsupported_surface"), 0);
        assert_eq!(metric_value("three_scene_system_hash_manifest_present"), 1);
        assert_eq!(metric_value("three_scene_system_hash_mismatch"), 0);
        assert_eq!(
            metric_value("three_scene_system_receipt_hash_refresh_current"),
            1
        );
        assert_eq!(
            metric_value("three_scene_system_receipt_hash_refresh_stale"),
            0
        );
        assert_eq!(
            metric_value("three_scene_system_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(
            metric_value("three_scene_system_dx_style_compatibility_present"),
            1
        );
        assert_eq!(
            metric_value("three_scene_system_dx_style_compatibility_missing"),
            0
        );

        write_three_scene_system_package_status(&package_status_path, &launch_scene_hash, None);

        let missing_style_report = read_dx_check_latest_panel(dir.path());
        let missing_style_view_model = serde_json::to_value(&missing_style_report.view_model)
            .expect("missing style view model json");
        let missing_style_scene = missing_style_view_model["package_lane_rows"]
            .as_array()
            .expect("missing style package lane rows")
            .iter()
            .find(|row| row["package_id"] == "3d/launch-scene")
            .expect("missing style 3D Scene System row");
        let missing_style_metric_value = |name: &str| -> u64 {
            missing_style_scene["metrics"]
                .as_array()
                .expect("missing style metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from missing style row"))
        };

        assert_eq!(
            missing_style_metric_value("three_scene_system_dx_style_compatibility_present"),
            0
        );
        assert_eq!(
            missing_style_metric_value("three_scene_system_dx_style_compatibility_missing"),
            1
        );
        assert!(
            missing_style_scene["next_action"]
                .as_str()
                .expect("next action")
                .contains("dx-style compatibility")
        );
        assert!(
            missing_style_scene["next_action"]
                .as_str()
                .expect("next action")
                .contains("without claiming live browser/WebGL proof")
        );
    }

    #[test]
    fn dx_check_latest_panel_exposes_webassembly_bridge_package_lane_style_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let wasm_status_path = dir.path().join("examples/template/wasm-interop-status.tsx");
        fs::create_dir_all(wasm_status_path.parent().expect("wasm status parent"))
            .expect("wasm status dir");
        fs::write(
            &wasm_status_path,
            "export const webAssemblyBridgeProbe = 'fresh';\n",
        )
        .expect("wasm status source");
        let wasm_status_hash = sha256_file(&wasm_status_path);

        let bridge_receipt_path = dir
            .path()
            .join(".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json");
        fs::create_dir_all(bridge_receipt_path.parent().expect("bridge receipt parent"))
            .expect("bridge receipt dir");
        fs::write(&bridge_receipt_path, "{}").expect("bridge receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        write_webassembly_bridge_package_status(
            &package_status_path,
            &wasm_status_hash,
            Some(serde_json::json!({
                "schema": "dx.forge.package.dx_style_compatibility",
                "status": "present",
                "token_source": "examples/template/wasm-interop-status.tsx",
                "generated_css": "tools/launch/runtime-template/assets/launch-runtime.css",
                "visible_surfaces": [
                    "wasm-bindgen-readiness-workflow",
                    "launch-wasm-compute-dashboard-workflow"
                ],
                "data_dx_markers": [
                    "data-dx-style-surface=\"theme-token\"",
                    "data-dx-token-scope=\"wasm/bindgen\""
                ],
                "runtime_proof": false
            })),
        );

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let bridge = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "wasm/bindgen")
            .expect("WebAssembly Bridge row");

        assert_eq!(bridge["official_package_name"], "WebAssembly Bridge");
        assert_eq!(bridge["upstream_package"], "wasm-bindgen");
        assert_eq!(bridge["upstream_version"], "0.2.121");
        assert_eq!(bridge["source_mirror"], "G:/WWW/inspirations/wasm-bindgen");
        assert_eq!(bridge["status"], "present");
        assert_eq!(bridge["receipt_status"], "present");
        assert_eq!(
            bridge["package_receipt_path"],
            ".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json"
        );
        assert!(
            bridge["selected_surfaces"]
                .as_array()
                .expect("selected surfaces")
                .iter()
                .any(
                    |surface| surface["surface_id"] == "wasm-bindgen-readiness-workflow"
                        && surface["source_markers"]
                            .as_array()
                            .expect("source markers")
                            .iter()
                            .any(|marker| marker == "data-dx-style-surface=\"theme-token\"")
                        && surface["source_markers"]
                            .as_array()
                            .expect("source markers")
                            .iter()
                            .any(|marker| marker == "data-dx-wasm-action=\"run-local-add\""),
                )
        );

        let metric_value = |name: &str| -> u64 {
            bridge["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(metric_value("webassembly_bridge_package_present"), 1);
        assert_eq!(metric_value("webassembly_bridge_receipt_present"), 1);
        assert_eq!(metric_value("webassembly_bridge_receipt_stale"), 0);
        assert_eq!(metric_value("webassembly_bridge_missing_receipt"), 0);
        assert_eq!(metric_value("webassembly_bridge_blocked_surface"), 0);
        assert_eq!(metric_value("webassembly_bridge_unsupported_surface"), 0);
        assert_eq!(metric_value("webassembly_bridge_hash_manifest_present"), 1);
        assert_eq!(metric_value("webassembly_bridge_hash_mismatch"), 0);
        assert_eq!(
            metric_value("webassembly_bridge_receipt_hash_refresh_current"),
            1
        );
        assert_eq!(
            metric_value("webassembly_bridge_receipt_hash_refresh_stale"),
            0
        );
        assert_eq!(
            metric_value("webassembly_bridge_receipt_hash_refresh_missing"),
            0
        );
        assert_eq!(
            metric_value("webassembly_bridge_dx_style_compatibility_present"),
            1
        );
        assert_eq!(
            metric_value("webassembly_bridge_dx_style_compatibility_missing"),
            0
        );

        write_webassembly_bridge_package_status(&package_status_path, &wasm_status_hash, None);

        let missing_style_report = read_dx_check_latest_panel(dir.path());
        let missing_style_view_model = serde_json::to_value(&missing_style_report.view_model)
            .expect("missing style view model json");
        let missing_style_bridge = missing_style_view_model["package_lane_rows"]
            .as_array()
            .expect("missing style package lane rows")
            .iter()
            .find(|row| row["package_id"] == "wasm/bindgen")
            .expect("missing style WebAssembly Bridge row");
        let missing_style_metric_value = |name: &str| -> u64 {
            missing_style_bridge["metrics"]
                .as_array()
                .expect("missing style metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing from missing style row"))
        };

        assert_eq!(
            missing_style_metric_value("webassembly_bridge_dx_style_compatibility_present"),
            0
        );
        assert_eq!(
            missing_style_metric_value("webassembly_bridge_dx_style_compatibility_missing"),
            1
        );
        assert!(
            missing_style_bridge["next_action"]
                .as_str()
                .expect("next action")
                .contains("dx-style compatibility")
        );
        assert!(
            missing_style_bridge["next_action"]
                .as_str()
                .expect("next action")
                .contains("without claiming live generated-Wasm or browser style proof")
        );
    }

    #[test]
    fn dx_check_latest_panel_exposes_webassembly_bridge_package_lane_hash_refresh_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let wasm_status_path = dir.path().join("examples/template/wasm-interop-status.tsx");
        fs::create_dir_all(wasm_status_path.parent().expect("wasm status parent"))
            .expect("wasm status dir");
        fs::write(
            &wasm_status_path,
            "export const webAssemblyBridgeProbe = 'fresh';\n",
        )
        .expect("wasm status source");
        let wasm_status_hash = sha256_file(&wasm_status_path);

        let bridge_receipt_path = dir
            .path()
            .join(".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json");
        fs::create_dir_all(bridge_receipt_path.parent().expect("bridge receipt parent"))
            .expect("bridge receipt dir");
        fs::write(&bridge_receipt_path, "{}").expect("bridge receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        write_webassembly_bridge_package_status(
            &package_status_path,
            &wasm_status_hash,
            Some(serde_json::json!({
                "schema": "dx.forge.package.dx_style_compatibility",
                "status": "present",
                "token_source": "examples/template/wasm-interop-status.tsx",
                "generated_css": "tools/launch/runtime-template/assets/launch-runtime.css",
                "visible_surfaces": [
                    "wasm-bindgen-readiness-workflow",
                    "launch-wasm-compute-dashboard-workflow"
                ],
                "data_dx_markers": [
                    "data-dx-style-surface=\"theme-token\"",
                    "data-dx-token-scope=\"wasm/bindgen\""
                ],
                "runtime_proof": false
            })),
        );

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let bridge = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "wasm/bindgen")
            .expect("WebAssembly Bridge row");
        let metric_value = |row: &serde_json::Value, name: &str| -> u64 {
            row["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(
            bridge["receipt_hash_refresh"]["schema"],
            "dx.forge.package.receipt_hash_refresh"
        );
        assert_eq!(bridge["receipt_hash_refresh"]["status"], "current");
        assert_eq!(
            bridge["receipt_hash_refresh"]["helper_path"],
            "examples/template/webassembly-bridge-receipt-hashes.ts"
        );
        assert_eq!(
            bridge["receipt_hash_refresh"]["json_check_command"],
            "node examples/template/webassembly-bridge-receipt-hashes.ts --check --json"
        );
        assert_eq!(
            bridge["receipt_hash_refresh"]["zed_visibility"],
            "webassembly-bridge:receipt-hash-refresh"
        );
        assert_eq!(
            bridge["receipt_hash_refresh"]["current_files"],
            serde_json::json!([
                "examples/template/wasm-interop-status.tsx",
                "tools/launch/materialize-www-template.ts",
                "docs/packages/wasm-bindgen.md",
                "dx-www/src/cli/studio_manifest.rs"
            ])
        );
        assert_eq!(
            bridge["receipt_hash_refresh"]["stale_files"],
            serde_json::json!([])
        );
        assert_eq!(
            bridge["receipt_hash_refresh"]["missing_files"],
            serde_json::json!([])
        );
        assert_eq!(
            metric_value(bridge, "webassembly_bridge_receipt_hash_refresh_current"),
            1
        );
        assert_eq!(
            metric_value(bridge, "webassembly_bridge_receipt_hash_refresh_stale"),
            0
        );
        assert_eq!(
            metric_value(bridge, "webassembly_bridge_receipt_hash_refresh_missing"),
            0
        );

        let mut stale_helper_status: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("package status bytes"))
                .expect("package status json");
        stale_helper_status["package_lane_visibility"][0]["receipt_hash_refresh"]["status"] =
            serde_json::json!("current");
        stale_helper_status["package_lane_visibility"][0]["receipt_hash_refresh"]["current_files"] =
            serde_json::json!([
                "examples/template/wasm-interop-status.tsx",
                "docs/packages/wasm-bindgen.md"
            ]);
        stale_helper_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_files"] =
            serde_json::json!(["tools/launch/materialize-www-template.ts"]);
        stale_helper_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_mirror_files"] =
            serde_json::json!(["dx-www/src/cli/studio_manifest.rs"]);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&stale_helper_status).expect("stale helper status json"),
        )
        .expect("write stale helper package status");

        let stale_report = read_dx_check_latest_panel(dir.path());
        let stale_view_model =
            serde_json::to_value(&stale_report.view_model).expect("stale view model json");
        let stale_bridge = stale_view_model["package_lane_rows"]
            .as_array()
            .expect("stale package lane rows")
            .iter()
            .find(|row| row["package_id"] == "wasm/bindgen")
            .expect("stale WebAssembly Bridge row");
        assert_eq!(
            stale_bridge["receipt_hash_refresh"]["stale_files"],
            serde_json::json!(["tools/launch/materialize-www-template.ts"])
        );
        assert_eq!(
            stale_bridge["receipt_hash_refresh"]["stale_mirror_files"],
            serde_json::json!(["dx-www/src/cli/studio_manifest.rs"])
        );
        assert_eq!(
            metric_value(
                stale_bridge,
                "webassembly_bridge_receipt_hash_refresh_current"
            ),
            0
        );
        assert_eq!(
            metric_value(
                stale_bridge,
                "webassembly_bridge_receipt_hash_refresh_stale"
            ),
            1
        );
        assert_eq!(
            metric_value(
                stale_bridge,
                "webassembly_bridge_receipt_hash_refresh_missing"
            ),
            0
        );
        assert_eq!(
            metric_value(stale_bridge, "webassembly_bridge_receipt_stale"),
            1
        );
        assert!(
            stale_bridge["next_action"]
                .as_str()
                .expect("stale next action")
                .contains("webassembly-bridge-receipt-hashes.ts --write")
        );
    }

    #[test]
    fn dx_check_latest_panel_reads_shared_contract_fixtures() {
        let fixture_dir = shared_dx_check_fixture_dir();
        let ready = fs::read_to_string(fixture_dir.join("panel-ready-warning.receipt.json"))
            .expect("ready shared fixture");
        let detected_config =
            fs::read_to_string(fixture_dir.join("panel-ready-detected-config.receipt.json"))
                .expect("detected config shared fixture");
        let malformed = fs::read_to_string(fixture_dir.join("panel-malformed-schema.receipt.json"))
            .expect("malformed shared fixture");

        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, ready).expect("ready receipt");

        let ready_report = read_dx_check_latest_panel(dir.path());
        assert_eq!(ready_report.status, DxCheckLatestPanelStatus::Ready);
        assert_eq!(
            ready_report.view_model.schema_version,
            DX_WWW_CHECK_PANEL_VIEW_MODEL_SCHEMA_VERSION
        );
        assert_eq!(ready_report.view_model.status, "ready");
        assert_eq!(
            ready_report.view_model.weight_profile,
            DX_CHECK_WEIGHT_PROFILE
        );
        assert_eq!(ready_report.view_model.scoring_config.status, "default");
        assert_eq!(
            ready_report
                .view_model
                .score_meter
                .as_ref()
                .expect("score meter")
                .value,
            410
        );
        assert_eq!(ready_report.view_model.bucket_rows[0].weight, 100);
        assert_eq!(
            ready_report.zed.as_ref().expect("zed").weight_profile,
            DX_CHECK_WEIGHT_PROFILE
        );
        assert_eq!(
            ready_report
                .zed
                .as_ref()
                .expect("zed")
                .scoring_config
                .status,
            "default"
        );
        assert_eq!(
            ready_report.view_model.quick_fix_rows[0].command.as_deref(),
            Some("dx check web-perf --url http://localhost:3000 --json")
        );

        fs::write(&receipt_path, detected_config).expect("detected config receipt");
        let detected_report = read_dx_check_latest_panel(dir.path());
        assert_eq!(detected_report.status, DxCheckLatestPanelStatus::Ready);
        assert_eq!(detected_report.view_model.status, "ready");
        assert_eq!(
            detected_report.view_model.scoring_config.status,
            "detected_not_applied"
        );
        assert!(!detected_report.view_model.scoring_config.applies_to_score);
        assert_eq!(
            detected_report
                .view_model
                .scoring_config
                .config_path
                .as_deref(),
            Some(".dx/check/config.json")
        );
        assert_eq!(
            detected_report
                .view_model
                .scoring_config
                .configured_total_weight,
            Some(500)
        );
        assert_eq!(
            detected_report
                .view_model
                .scoring_config
                .configured_bucket_weights[2]
                .weight,
            120
        );
        assert_eq!(
            detected_report.view_model.warning_rows[1].code,
            "score-config-detected-not-applied"
        );
        assert_eq!(
            detected_report.view_model.quick_fix_rows[0]
                .command
                .as_deref(),
            None
        );
        assert_eq!(
            detected_report
                .zed
                .as_ref()
                .expect("zed")
                .scoring_config
                .status,
            "detected_not_applied"
        );

        fs::write(&receipt_path, malformed).expect("malformed receipt");
        let malformed_report = read_dx_check_latest_panel(dir.path());
        assert_eq!(malformed_report.status, DxCheckLatestPanelStatus::Malformed);
        assert_eq!(malformed_report.view_model.status, "malformed");
        assert!(malformed_report.view_model.score_meter.is_none());
        assert!(
            malformed_report
                .view_model
                .empty_state
                .as_deref()
                .expect("malformed empty state")
                .contains("could not be parsed")
        );
    }

    fn sha256_file(path: &Path) -> String {
        let bytes = fs::read(path).expect("hash input");
        format!("{:x}", Sha256::digest(bytes))
    }

    fn write_realtime_app_database_package_status(
        package_status_path: &Path,
        runtime_hash: &str,
        dashboard_hash: &str,
        dx_style_compatibility: Option<serde_json::Value>,
    ) {
        let mut visibility = serde_json::json!({
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
                    "files": [
                        "tools/launch/runtime-template/pages/index.html"
                    ],
                    "source_markers": [
                        "data-dx-package=\"instantdb/react\"",
                        "data-dx-component=\"instantdb-runtime-dashboard-workflow\"",
                        "data-dx-instant-action=\"prepare-local-schema-receipt\"",
                        "data-dx-style-surface=\"realtime-app-database\""
                    ],
                    "hash_algorithm": "sha256",
                    "file_hashes": {
                        "tools/launch/runtime-template/pages/index.html": runtime_hash
                    }
                },
                {
                    "surface_id": "dashboard-instantdb-workflow",
                    "status": "present",
                    "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json",
                    "files": [
                        "examples/dashboard/src/components/InstantDbDashboardWorkflow.tsx"
                    ],
                    "source_markers": [
                        "data-dx-package=\"instantdb/react\"",
                        "data-dx-component=\"dashboard-instantdb-workflow\"",
                        "data-dx-instant-dashboard-workflow=\"realtime-boundary\"",
                        "data-dx-style-surface=\"realtime-app-database\""
                    ],
                    "hash_algorithm": "sha256",
                    "file_hashes": {
                        "examples/dashboard/src/components/InstantDbDashboardWorkflow.tsx": dashboard_hash
                    }
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
                "realtime_app_database_receipt_hash_refresh_current",
                "realtime_app_database_receipt_hash_refresh_stale",
                "realtime_app_database_receipt_hash_refresh_missing",
                "realtime_app_database_dx_style_compatibility_present",
                "realtime_app_database_dx_style_compatibility_missing"
            ],
            "runtime_limitations": [
                "SOURCE-ONLY: package-lane visibility is receipt, source-marker, hash, and style evidence; no hosted Instant runtime proof is claimed.",
                "ADAPTER-BOUNDARY: Instant app provisioning, rules, auth, storage, streams, and Sync Table runtime policy stay app-owned."
            ],
            "receipt_hash_refresh": {
                "schema": "dx.forge.package.receipt_hash_refresh",
                "status": "current",
                "helper_path": "examples/template/realtime-app-database-receipt-hashes.ts",
                "check_command": "node examples/template/realtime-app-database-receipt-hashes.ts --check",
                "write_command": "node examples/template/realtime-app-database-receipt-hashes.ts --write",
                "json_check_command": "node examples/template/realtime-app-database-receipt-hashes.ts --check --json",
                "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json",
                "hash_algorithm": "sha256",
                "tracked_file_count": 6,
                "stale_file_count": 0,
                "missing_file_count": 0,
                "runtime_execution": false,
                "secret_access": false,
                "zed_visibility": "realtime-app-database:receipt-hash-refresh",
                "runtime_limitations": [
                    "SOURCE-ONLY: this helper checks local Realtime App Database receipt hash freshness only.",
                    "ADAPTER-BOUNDARY: hosted Instant app provisioning, rules, auth policy, storage, streams, Sync Table runtime validation, dependency installation, and browser proof stay app-owned."
                ]
            }
        });

        if let Some(dx_style) = dx_style_compatibility {
            visibility["dx_style_compatibility"] = dx_style;
        }

        fs::write(
            package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [visibility]
            }))
            .expect("Realtime App Database package-status json"),
        )
        .expect("write Realtime App Database package status");
    }

    fn write_three_scene_system_package_status(
        package_status_path: &Path,
        launch_scene_hash: &str,
        dx_style_compatibility: Option<serde_json::Value>,
    ) {
        let mut visibility = serde_json::json!({
            "official_package_name": "3D Scene System",
            "package_id": "3d/launch-scene",
            "upstream_package": "three + @react-three/fiber + @react-three/drei",
            "upstream_version": "three 0.184.0; @react-three/fiber 9.6.1; @react-three/drei local mirror",
            "source_mirror": "G:/WWW/inspirations/three.js; G:/WWW/inspirations/react-three-fiber; G:/WWW/inspirations/drei",
            "status": "present",
            "receipt_status": "present",
            "package_receipt_path": ".dx/forge/receipts/3d-launch-scene-dashboard-workflow.json",
            "status_vocabulary": [
                "present",
                "stale",
                "missing-receipt",
                "blocked",
                "unsupported-surface"
            ],
            "selected_surfaces": [
                {
                    "surface_id": "launch-scene-dashboard-workflow",
                    "status": "present",
                    "receipt_path": ".dx/forge/receipts/3d-launch-scene-dashboard-workflow.json",
                    "files": [
                        "components/scene/launch-scene.tsx"
                    ],
                    "source_markers": [
                        "data-dx-package=\"3d/launch-scene\"",
                        "data-dx-component=\"launch-scene-webgl-proof\"",
                        "data-dx-component=\"launch-scene-dashboard-workflow\"",
                        "data-dx-style-surface=\"launch-scene\"",
                        "data-dx-token-scope=\"3d/launch-scene\""
                    ],
                    "hash_algorithm": "sha256",
                    "file_hashes": {
                        "examples/template/launch-scene.tsx": launch_scene_hash
                    }
                }
            ],
            "blocked_surfaces": [],
            "unsupported_surfaces": [],
            "dx_check_metrics": [
                "three_scene_system_receipt_present",
                "three_scene_system_receipt_stale",
                "three_scene_system_missing_receipt",
                "three_scene_system_blocked_surface",
                "three_scene_system_unsupported_surface",
                "three_scene_system_hash_manifest_present",
                "three_scene_system_hash_mismatch",
                "three_scene_system_receipt_hash_refresh_current",
                "three_scene_system_receipt_hash_refresh_stale",
                "three_scene_system_receipt_hash_refresh_missing",
                "three_scene_system_dx_style_compatibility_present",
                "three_scene_system_dx_style_compatibility_missing"
            ],
            "receipt_hash_refresh": {
                "schema": "dx.forge.package.receipt_hash_refresh",
                "status": "current",
                "helper_path": "examples/template/3d-scene-system-receipt-hashes.ts",
                "check_command": "node examples/template/3d-scene-system-receipt-hashes.ts --check",
                "write_command": "node examples/template/3d-scene-system-receipt-hashes.ts --write",
                "json_check_command": "node examples/template/3d-scene-system-receipt-hashes.ts --check --json",
                "receipt_path": ".dx/forge/receipts/3d-launch-scene-dashboard-workflow.json",
                "hash_algorithm": "sha256",
                "tracked_file_count": 1,
                "stale_file_count": 0,
                "missing_file_count": 0,
                "runtime_execution": false,
                "secret_access": false,
                "zed_visibility": "3d-scene-system:receipt-hash-refresh"
            },
            "runtime_limitations": [
                "SOURCE-ONLY: package-lane visibility is receipt, source-marker, hash, and style evidence; no live browser/WebGL proof is claimed."
            ]
        });

        if let Some(dx_style) = dx_style_compatibility {
            visibility["dx_style_compatibility"] = dx_style;
        }

        fs::write(
            package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [visibility]
            }))
            .expect("3D Scene System package-status json"),
        )
        .expect("write 3D Scene System package status");
    }

    fn write_webassembly_bridge_package_status(
        package_status_path: &Path,
        wasm_status_hash: &str,
        dx_style_compatibility: Option<serde_json::Value>,
    ) {
        let mut visibility = serde_json::json!({
            "official_package_name": "WebAssembly Bridge",
            "package_id": "wasm/bindgen",
            "upstream_package": "wasm-bindgen",
            "upstream_version": "0.2.121",
            "source_mirror": "G:/WWW/inspirations/wasm-bindgen",
            "status": "present",
            "receipt_status": "present",
            "package_receipt_path": ".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json",
            "status_vocabulary": [
                "present",
                "stale",
                "missing-receipt",
                "blocked",
                "unsupported-surface"
            ],
            "selected_surfaces": [
                {
                    "surface_id": "wasm-bindgen-readiness-workflow",
                    "status": "present",
                    "receipt_path": ".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json",
                    "files": [
                        "components/launch/wasm-interop-status.tsx"
                    ],
                    "source_markers": [
                        "data-dx-package=\"wasm/bindgen\"",
                        "data-dx-component=\"wasm-bindgen-readiness-workflow\"",
                        "data-dx-style-surface=\"theme-token\"",
                        "data-dx-wasm-action=\"run-local-add\""
                    ],
                    "hash_algorithm": "sha256",
                    "file_hashes": {
                        "examples/template/wasm-interop-status.tsx": wasm_status_hash
                    }
                }
            ],
            "blocked_surfaces": [],
            "unsupported_surfaces": [],
            "dx_check_metrics": [
                "webassembly_bridge_package_present",
                "webassembly_bridge_receipt_present",
                "webassembly_bridge_receipt_stale",
                "webassembly_bridge_missing_receipt",
                "webassembly_bridge_blocked_surface",
                "webassembly_bridge_unsupported_surface",
                "webassembly_bridge_hash_manifest_present",
                "webassembly_bridge_hash_mismatch",
                "webassembly_bridge_receipt_hash_refresh_current",
                "webassembly_bridge_receipt_hash_refresh_stale",
                "webassembly_bridge_receipt_hash_refresh_missing",
                "webassembly_bridge_dx_style_compatibility_present",
                "webassembly_bridge_dx_style_compatibility_missing"
            ],
            "receipt_hash_refresh": {
                "schema": "dx.forge.package.receipt_hash_refresh",
                "status": "current",
                "helper_path": "examples/template/webassembly-bridge-receipt-hashes.ts",
                "check_command": "node examples/template/webassembly-bridge-receipt-hashes.ts --check",
                "write_command": "node examples/template/webassembly-bridge-receipt-hashes.ts --write",
                "json_check_command": "node examples/template/webassembly-bridge-receipt-hashes.ts --check --json",
                "source_guard_runbook_fixture": "docs/packages/wasm-bindgen.source-guard-runbook.json",
                "preview_manifest_materializer": "tools/launch/materialize-www-template.ts",
                "receipt_path": ".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json",
                "hash_algorithm": "sha256",
                "tracked_file_count": 4,
                "tracked_files": [
                    "examples/template/wasm-interop-status.tsx",
                    "tools/launch/materialize-www-template.ts",
                    "docs/packages/wasm-bindgen.md",
                    "dx-www/src/cli/studio_manifest.rs"
                ],
                "current_files": [
                    "examples/template/wasm-interop-status.tsx",
                    "tools/launch/materialize-www-template.ts",
                    "docs/packages/wasm-bindgen.md",
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
                "zed_visibility": "webassembly-bridge:receipt-hash-refresh",
                "runtime_limitations": [
                    "SOURCE-ONLY: helper checks local WebAssembly Bridge receipt hash freshness only.",
                    "ADAPTER-BOUNDARY: Rust crate exports, wasm32 build output, generated glue import paths, CSP, MIME serving, and performance/security review stay app-owned."
                ]
            },
            "runtime_limitations": [
                "SOURCE-ONLY: package-lane visibility is receipt, source-marker, hash, and dx-style evidence; no live generated-Wasm or browser style proof is claimed.",
                "ADAPTER-BOUNDARY: Rust crate exports, wasm32 build output, generated glue import paths, CSP, MIME serving, and performance/security review stay app-owned."
            ]
        });

        if let Some(dx_style) = dx_style_compatibility {
            visibility["dx_style_compatibility"] = dx_style;
        }

        fs::write(
            package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [visibility]
            }))
            .expect("WebAssembly Bridge package-status json"),
        )
        .expect("write WebAssembly Bridge package status");
    }

    fn write_ai_sdk_package_status(package_status_path: &Path, assistant_hash: &str) {
        fs::write(
            package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [
                    {
                        "official_package_name": AI_SDK_OFFICIAL_NAME,
                        "package_id": AI_SDK_PACKAGE_ID,
                        "upstream_package": AI_SDK_UPSTREAM_PACKAGE,
                        "upstream_version": AI_SDK_UPSTREAM_VERSION,
                        "source_mirror": AI_SDK_SOURCE_MIRROR,
                        "status": "present",
                        "receipt_status": "present",
                        "package_receipt_path": AI_SDK_PACKAGE_RECEIPT_PATH,
                        "status_vocabulary": [
                            "present",
                            "stale",
                            "missing-receipt",
                            "blocked",
                            "unsupported-surface"
                        ],
                        "selected_surfaces": [
                            {
                                "surface_id": "ai-launch-assistant-dashboard-workflow",
                                "status": "present",
                                "receipt_path": AI_SDK_PACKAGE_RECEIPT_PATH,
                                "files": [
                                    "components/launch/ai-chat-status.tsx"
                                ],
                                "source_markers": [
                                    "data-dx-package=\"ai/vercel-ai\"",
                                    "data-dx-component=\"launch-ai-assistant-dashboard-workflow\"",
                                    "data-dx-ai-route-contract=\"/api/ai/chat\"",
                                    "data-dx-style-surface=\"ai-sdk\"",
                                    "data-dx-token-scope=\"ai/vercel-ai\""
                                ],
                                "hash_algorithm": "sha256",
                                "file_hashes": {
                                    "examples/template/ai-chat-status.tsx": assistant_hash
                                }
                            }
                        ],
                        "blocked_surfaces": [],
                        "unsupported_surfaces": [],
                        "dx_style_compatibility": {
                            "schema": "dx.forge.package.dx_style_compatibility",
                            "status": "present",
                            "token_source": "styles/theme.css",
                            "generated_css": "styles/generated.css",
                            "visible_surfaces": [
                                "ai-launch-assistant-dashboard-workflow"
                            ],
                            "data_dx_markers": [
                                "data-dx-style-surface=\"ai-sdk\"",
                                "data-dx-token-scope=\"ai/vercel-ai\""
                            ],
                            "runtime_proof": false
                        },
                        "dx_check_metrics": [
                            "ai_sdk_package_present",
                            "ai_sdk_receipt_present",
                            "ai_sdk_receipt_stale",
                            "ai_sdk_missing_receipt",
                            "ai_sdk_blocked_surface",
                            "ai_sdk_unsupported_surface",
                            "ai_sdk_hash_manifest_present",
                            "ai_sdk_hash_mismatch",
                            "ai_sdk_receipt_hash_refresh_current",
                            "ai_sdk_receipt_hash_refresh_stale",
                            "ai_sdk_receipt_hash_refresh_missing",
                            "ai_sdk_dx_style_compatibility_present",
                            "ai_sdk_dx_style_compatibility_missing"
                        ],
                        "receipt_hash_refresh": {
                            "schema": "dx.forge.package.receipt_hash_refresh",
                            "status": "current",
                            "helper_path": "examples/template/ai-sdk-receipt-hashes.ts",
                            "check_command": "node examples/template/ai-sdk-receipt-hashes.ts --check",
                            "write_command": "node examples/template/ai-sdk-receipt-hashes.ts --write",
                            "json_check_command": "node examples/template/ai-sdk-receipt-hashes.ts --check --json",
                            "receipt_path": AI_SDK_PACKAGE_RECEIPT_PATH,
                            "hash_algorithm": "sha256",
                            "tracked_file_count": 1,
                            "stale_file_count": 0,
                            "missing_file_count": 0,
                            "runtime_execution": false,
                            "secret_access": false,
                            "zed_visibility": "ai-sdk:receipt-hash-refresh",
                            "runtime_limitations": [
                                "SOURCE-ONLY: helper checks local AI SDK receipt hash freshness only.",
                                "ADAPTER-BOUNDARY: provider credentials, gateway routing, model safety, persistence, rate limits, and billing controls stay app-owned."
                            ]
                        },
                        "runtime_limitations": [
                            "SOURCE-ONLY: package-lane visibility is receipt, source-marker, hash, helper freshness, and dx-style evidence; no live model streaming or browser proof is claimed.",
                            "ADAPTER-BOUNDARY: provider credentials, gateway routing, model safety, persistence, rate limits, and billing controls stay app-owned."
                        ]
                    }
                ]
            }))
            .expect("AI SDK package-status json"),
        )
        .expect("write AI SDK package status");
    }

    fn write_internationalization_package_status(
        package_status_path: &Path,
        locale_workflow_hash: &str,
        dx_style_compatibility: Option<serde_json::Value>,
    ) {
        let mut visibility = serde_json::json!({
            "official_package_name": "Internationalization",
            "package_id": "i18n/next-intl",
            "upstream_package": "next-intl",
            "upstream_version": "4.12.0",
            "source_mirror": "G:/WWW/inspirations/next-intl",
            "status": "present",
            "receipt_status": "present",
            "package_receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json",
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
                    "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json",
                    "files": [
                        "components/launch/next-intl-dashboard-locale.tsx"
                    ],
                    "source_markers": [
                        "data-dx-package=\"i18n/next-intl\"",
                        "data-dx-component=\"next-intl-dashboard-locale-workflow\"",
                        "data-dx-style-surface=\"internationalization\""
                    ],
                    "hash_algorithm": "sha256",
                    "file_hashes": {
                        "examples/template/next-intl-dashboard-locale.tsx": locale_workflow_hash
                    }
                }
            ],
            "blocked_surfaces": [],
            "unsupported_surfaces": [],
            "dx_check_metrics": [
                "internationalization_receipt_present",
                "internationalization_receipt_stale",
                "internationalization_missing_receipt",
                "internationalization_blocked_surface",
                "internationalization_unsupported_surface",
                "internationalization_hash_manifest_present",
                "internationalization_hash_mismatch",
                "internationalization_receipt_hash_refresh_current",
                "internationalization_receipt_hash_refresh_stale",
                "internationalization_receipt_hash_refresh_missing",
                "internationalization_dx_style_compatibility_present",
                "internationalization_dx_style_compatibility_missing"
            ],
            "receipt_hash_refresh": {
                "schema": "dx.forge.package.receipt_hash_refresh",
                "status": "current",
                "helper_path": "examples/template/internationalization-receipt-hashes.ts",
                "check_command": "node examples/template/internationalization-receipt-hashes.ts --check",
                "write_command": "node examples/template/internationalization-receipt-hashes.ts --write",
                "json_check_command": "node examples/template/internationalization-receipt-hashes.ts --check --json",
                "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json",
                "hash_algorithm": "sha256",
                "tracked_file_count": 1,
                "stale_file_count": 0,
                "missing_file_count": 0,
                "runtime_execution": false,
                "secret_access": false,
                "zed_visibility": "internationalization:receipt-hash-refresh",
                "runtime_limitations": [
                    "SOURCE-ONLY: helper checks local Internationalization receipt hash freshness only.",
                    "ADAPTER-BOUNDARY: locale routing, translation quality, SEO alternates, middleware placement, and runtime dependency installation stay app-owned."
                ]
            },
            "runtime_limitations": [
                "SOURCE-ONLY: package-lane visibility is receipt, source-marker, hash, and style evidence; no live locale routing proof is claimed."
            ]
        });

        if let Some(dx_style) = dx_style_compatibility {
            visibility["dx_style_compatibility"] = dx_style;
        }

        fs::write(
            package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [visibility]
            }))
            .expect("Internationalization package-status json"),
        )
        .expect("write Internationalization package status");
    }

    fn write_database_orm_package_status(
        package_status_path: &Path,
        replica_source_hash: &str,
        workflow_hash: &str,
        dx_style_compatibility: Option<serde_json::Value>,
    ) {
        let mut visibility = serde_json::json!({
            "official_package_name": "Database ORM",
            "package_id": "db/drizzle-sqlite",
            "upstream_package": "drizzle-orm",
            "upstream_version": "0.45.3",
            "source_mirror": "G:/WWW/inspirations/drizzle-orm",
            "status": "present",
            "receipt_status": "present",
            "package_receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json",
            "status_vocabulary": [
                "present",
                "stale",
                "missing-receipt",
                "blocked",
                "unsupported-surface"
            ],
            "selected_surfaces": [
                {
                    "surface_id": "drizzle-replica-routing",
                    "status": "present",
                    "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json",
                    "files": [
                        "db/drizzle/replicas.ts"
                    ],
                    "source_markers": [
                        "data-dx-package=\"db/drizzle-sqlite\"",
                        "import { withReplicas } from \"drizzle-orm/sqlite-core\"",
                        "export function createDxDrizzleReplicaSet"
                    ],
                    "hash_algorithm": "sha256",
                    "file_hashes": {
                        "core/src/ecosystem/forge_drizzle.rs": replica_source_hash
                    }
                },
                {
                    "surface_id": "drizzle-launch-dashboard-workflow",
                    "status": "present",
                    "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json",
                    "files": [
                        "components/launch/drizzle-query-proof.tsx"
                    ],
                    "source_markers": [
                        "data-dx-package=\"db/drizzle-sqlite\"",
                        "data-dx-component=\"launch-drizzle-data-workflow\"",
                        "data-dx-style-surface=\"database-orm\"",
                        "data-dx-dashboard-workflow=\"sqlite-read-model\"",
                        "data-dx-drizzle-action=\"apply-read-model\"",
                        "data-dx-drizzle-receipt-path"
                    ],
                    "hash_algorithm": "sha256",
                    "file_hashes": {
                        "examples/template/drizzle-query-proof.tsx": workflow_hash
                    }
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
                "database_orm_hash_manifest_present",
                "database_orm_hash_mismatch",
                "database_orm_receipt_hash_refresh_current",
                "database_orm_receipt_hash_refresh_stale",
                "database_orm_receipt_hash_refresh_missing",
                "database_orm_dx_style_compatibility_present",
                "database_orm_dx_style_compatibility_missing"
            ],
            "receipt_hash_refresh": {
                "schema": "dx.forge.package.receipt_hash_refresh",
                "status": "current",
                "helper_path": "examples/template/database-orm-receipt-hashes.ts",
                "check_command": "node examples/template/database-orm-receipt-hashes.ts --check",
                "write_command": "node examples/template/database-orm-receipt-hashes.ts --write",
                "json_check_command": "node examples/template/database-orm-receipt-hashes.ts --check --json",
                "receipt_path": "examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json",
                "hash_algorithm": "sha256",
                "tracked_file_count": 2,
                "stale_file_count": 0,
                "missing_file_count": 0,
                "runtime_execution": false,
                "secret_access": false,
                "zed_visibility": "database-orm:receipt-hash-refresh"
            },
            "runtime_limitations": [
                "SOURCE-ONLY: package-lane visibility proves receipts, source markers, and SHA-256 hash metadata; no live SQLite read proof is claimed.",
                "ADAPTER-BOUNDARY: database files, migration rollout, replica health, authorization, and backup policy stay app-owned."
            ]
        });

        if let Some(dx_style) = dx_style_compatibility {
            visibility["dx_style_compatibility"] = dx_style;
        }

        fs::write(
            package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [visibility]
            }))
            .expect("Database ORM package-status json"),
        )
        .expect("write Database ORM package status");
    }

    fn write_markdown_mdx_content_package_status(
        package_status_path: &Path,
        receipt_helper_hash: &str,
        materialized_source: Option<serde_json::Value>,
    ) {
        let mut visibility = serde_json::json!({
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
                    "surface_id": "forge-receipt-helper",
                    "status": "present",
                    "receipt_path": ".dx/forge/receipts/packages/content-react-markdown.json",
                    "files": [
                        "lib/markdown-mdx-content/receipt.ts"
                    ],
                    "source_markers": [
                        "createMarkdownMdxContentReceipt",
                        "dx.forge.markdown_mdx_content_receipt",
                        "markdownMdxContentReceiptStatuses"
                    ],
                    "hash_algorithm": "sha256",
                    "file_hashes": {
                        "lib/markdown-mdx-content/receipt.ts": receipt_helper_hash
                    }
                },
                {
                    "surface_id": "mdx-provider",
                    "status": "present",
                    "receipt_path": ".dx/forge/receipts/packages/content-react-markdown.json",
                    "files": [
                        "components/content/mdx-provider.tsx"
                    ],
                    "source_markers": [
                        "data-dx-package=\"content/react-markdown\"",
                        "data-dx-package-name=\"Markdown & MDX Content\"",
                        "data-dx-component=\"dx-mdx-provider\"",
                        "data-dx-style-surface=\"markdown-mdx-content\""
                    ]
                }
            ],
            "blocked_surfaces": [],
            "unsupported_surfaces": [],
            "receipt_hash_refresh": {
                "schema": "dx.forge.package.receipt_hash_refresh",
                "status": "current",
                "helper_path": "examples/template/markdown-mdx-content-receipt-hashes.ts",
                "check_command": "node examples/template/markdown-mdx-content-receipt-hashes.ts --check",
                "write_command": "node examples/template/markdown-mdx-content-receipt-hashes.ts --write",
                "json_check_command": "node examples/template/markdown-mdx-content-receipt-hashes.ts --check --json",
                "receipt_path": ".dx/forge/receipts/packages/content-react-markdown.json",
                "hash_algorithm": "sha256",
                "tracked_file_count": 1,
                "tracked_files": [
                    "lib/markdown-mdx-content/receipt.ts"
                ],
                "stale_file_count": 0,
                "missing_file_count": 0,
                "runtime_execution": false,
                "secret_access": false,
                "zed_visibility": "markdown-mdx-content:receipt-hash-refresh",
                "runtime_limitations": [
                    "SOURCE-ONLY: package-lane visibility is receipt, source-marker, hash, dx-style, materialized-source, and helper-freshness evidence; no live Markdown/MDX renderer proof is claimed.",
                    "ADAPTER-BOUNDARY: runtime dependencies, remark/rehype plugins, raw HTML policy, sanitizer review, content trust, and trusted MDX execution stay app-owned."
                ]
            },
            "dx_style_compatibility": {
                "schema": "dx.forge.package.dx_style_compatibility",
                "status": "present",
                "token_source": "styles/theme.css",
                "generated_css": "styles/generated.css",
                "visible_surfaces": [
                    "mdx-provider"
                ],
                "data_dx_markers": [
                    "data-dx-style-surface=\"markdown-mdx-content\""
                ],
                "runtime_proof": false
            },
            "dx_check_metrics": [
                "markdown_mdx_content_package_present",
                "markdown_mdx_content_receipt_present",
                "markdown_mdx_content_receipt_stale",
                "markdown_mdx_content_missing_receipt",
                "markdown_mdx_content_blocked_surface",
                "markdown_mdx_content_unsupported_surface",
                "markdown_mdx_content_hash_manifest_present",
                "markdown_mdx_content_hash_mismatch",
                "markdown_mdx_content_receipt_hash_refresh_current",
                "markdown_mdx_content_receipt_hash_refresh_stale",
                "markdown_mdx_content_receipt_hash_refresh_missing",
                "markdown_mdx_content_dx_style_compatibility_present",
                "markdown_mdx_content_dx_style_compatibility_missing",
                "markdown_mdx_content_materialized_source_present",
                "markdown_mdx_content_materialized_source_missing"
            ],
            "runtime_limitations": [
                "SOURCE-ONLY: package-lane visibility is receipt, source-marker, hash, dx-style, and materialized-source evidence; no live Markdown/MDX renderer proof is claimed.",
                "ADAPTER-BOUNDARY: runtime dependencies, remark/rehype plugins, raw HTML policy, sanitizer review, content trust, and trusted MDX execution stay app-owned."
            ]
        });

        if let Some(materialized_source) = materialized_source {
            visibility["materialized_source"] = materialized_source;
        }

        fs::write(
            package_status_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema": "dx.www.template.forge_package_status",
                "package_lane_visibility": [visibility]
            }))
            .expect("Markdown & MDX Content package-status json"),
        )
        .expect("write Markdown & MDX Content package status");
    }

    fn shared_dx_check_fixture_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("www dir")
            .parent()
            .expect("dx dir")
            .join("cli")
            .join("fixtures")
            .join("dx-check")
    }

    #[test]
    fn dx_check_latest_panel_exposes_automation_connectors_package_lane_hash_refresh_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
        fs::write(&receipt_path, sample_receipt()).expect("sample receipt");

        let automation_source =
            "core/src/ecosystem/project_check/automation_connectors_dx_check.rs";
        let automation_check_panel_source = "core/src/ecosystem/dx_check_receipt.rs";
        let automation_source_path = dir.path().join(automation_source);
        fs::create_dir_all(
            automation_source_path
                .parent()
                .expect("automation source parent"),
        )
        .expect("automation source dir");
        fs::write(
            &automation_source_path,
            "pub fn forge_automation_connectors_package_metrics_probe() {}\n",
        )
        .expect("automation source");
        let automation_source_hash = sha256_file(&automation_source_path);
        let automation_check_panel_source_path = dir.path().join(automation_check_panel_source);
        fs::write(
            &automation_check_panel_source_path,
            "pub fn automation_connectors_package_lane_row_probe() {}\n",
        )
        .expect("automation check panel source");
        let automation_check_panel_source_hash = sha256_file(&automation_check_panel_source_path);

        let automation_receipt = "examples/template/.dx/forge/receipts/2026-05-22-automation-connectors-launch-workflow.json";
        let automation_receipt_path = dir.path().join(automation_receipt);
        fs::create_dir_all(
            automation_receipt_path
                .parent()
                .expect("automation receipt parent"),
        )
        .expect("automation receipt dir");
        fs::write(&automation_receipt_path, "{}").expect("automation receipt");

        let package_status_path = dir.path().join(".dx/forge/package-status.json");
        fs::create_dir_all(package_status_path.parent().expect("package status parent"))
            .expect("package status dir");
        let mut package_status = serde_json::json!({
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
                    "package_receipt_path": automation_receipt
                }
            ]
        });
        let package_entry = &mut package_status["package_lane_visibility"][0];
        package_entry["status_vocabulary"] = serde_json::json!([
            "present",
            "stale",
            "missing-receipt",
            "blocked",
            "unsupported-surface"
        ]);
        package_entry["selected_surfaces"] = serde_json::json!([
            {
                "surface_id": "automation-connectors-lower-dx-check-source",
                "status": "present",
                "files": [automation_source],
                "source_markers": [
                    "forge_automation_connectors_package_metrics",
                    "automation_connectors_receipt_hash_refresh_current",
                    "automation_connectors_receipt_hash_refresh_stale",
                    "automation_connectors_receipt_hash_refresh_missing"
                ],
                "hash_algorithm": "sha256",
                "file_hashes": {
                    automation_source: automation_source_hash
                }
            },
            {
                "surface_id": "automation-connectors-check-panel-source",
                "status": "present",
                "files": [automation_check_panel_source],
                "source_markers": [
                    "automation_connectors_package_lane_row",
                    "automation_connectors_receipt_hash_refresh_current",
                    "receipt_hash_refresh_counts",
                    "check_panel.view_model.package_lane_rows"
                ],
                "hash_algorithm": "sha256",
                "file_hashes": {
                    automation_check_panel_source: automation_check_panel_source_hash
                }
            }
        ]);
        package_entry["receipt_hash_refresh"] = serde_json::json!({
            "schema": "dx.forge.package.receipt_hash_refresh",
            "status": "current",
            "helper_path": "examples/template/automation-connectors-receipt-hashes.ts",
            "check_command": "node examples/template/automation-connectors-receipt-hashes.ts --check",
            "write_command": "node examples/template/automation-connectors-receipt-hashes.ts --write",
            "json_check_command": "node examples/template/automation-connectors-receipt-hashes.ts --check --json",
            "source_guard_runbook_fixture": "docs/packages/automation-connectors.source-guard-runbook.json",
            "preview_manifest_materializer": "tools/launch/materialize-www-template.ts",
            "studio_manifest_source": "dx-www/src/cli/studio_manifest.rs",
            "lower_dx_check_source": automation_source,
            "check_panel_source": automation_check_panel_source,
            "receipt_path": automation_receipt,
            "hash_algorithm": "sha256",
            "tracked_file_count": 15,
            "tracked_files": [
                "docs/packages/automation-connectors.source-guard-runbook.json",
                "tools/launch/materialize-www-template.ts",
                "dx-www/src/cli/studio_manifest.rs",
                automation_source,
                automation_check_panel_source
            ],
            "current_files": [
                "docs/packages/automation-connectors.source-guard-runbook.json",
                "tools/launch/materialize-www-template.ts",
                "dx-www/src/cli/studio_manifest.rs",
                automation_source,
                automation_check_panel_source
            ],
            "stale_files": [],
            "missing_files": [],
            "stale_mirror_files": [],
            "missing_mirror_files": [],
            "stale_file_count": 0,
            "missing_file_count": 0,
            "runtime_execution": false,
            "secret_access": false,
            "zed_visibility": "automation-connectors:receipt-hash-refresh",
            "runtime_limitations": [
                "SOURCE-ONLY: helper checks local Automation Connectors receipt hash freshness only.",
                "ADAPTER-BOUNDARY: live n8n workflow execution, provider credentials, and webhook registration stay app-owned."
            ]
        });
        package_entry["dx_style_compatibility"] = serde_json::json!({
            "schema": "dx.forge.package.dx_style_compatibility",
            "status": "present",
            "token_source": "related-crates/style",
            "generated_css": "tools/launch/runtime-template/assets/launch-runtime.css",
            "receipt_path": automation_receipt,
            "visible_surfaces": ["launch-automation-dashboard-workflow"],
            "source_files": ["examples/template/automations-status.tsx"]
        });
        package_entry["inspected_upstream_files"] = serde_json::json!([
            "packages/nodes-base/package.json",
            "packages/nodes-base/nodes/ManualTrigger/ManualTrigger.node.ts",
            "packages/nodes-base/nodes/Slack/Slack.node.ts",
            "packages/nodes-base/nodes/Slack/V2/SlackV2.node.ts",
            "packages/nodes-base/nodes/Webhook/Webhook.node.ts",
            "packages/nodes-base/nodes/Notion/Notion.node.ts",
            "packages/nodes-base/credentials/SlackApi.credentials.ts",
            "packages/nodes-base/credentials/SlackOAuth2Api.credentials.ts",
            "packages/nodes-base/credentials/NotionApi.credentials.ts"
        ]);
        package_entry["upstream_public_apis"] = serde_json::json!([
            "VersionedNodeType",
            "INodeType",
            "INodeTypeDescription",
            "ITriggerFunctions",
            "IExecuteFunctions",
            "IWebhookFunctions",
            "ICredentialType",
            "IAuthenticateGeneric",
            "ICredentialTestRequest"
        ]);
        package_entry["runtime_limitations"] = serde_json::json!([
            "ADAPTER-BOUNDARY: live n8n workflow execution, provider credentials, webhook registration, and browser visual proof stay app-owned."
        ]);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&package_status)
                .expect("Automation Connectors package status json"),
        )
        .expect("write Automation Connectors package status");

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let automation_connectors = view_model["package_lane_rows"]
            .as_array()
            .expect("package lane rows")
            .iter()
            .find(|row| row["package_id"] == "automations/n8n")
            .expect("Automation Connectors row");
        let metric_value = |row: &serde_json::Value, name: &str| -> u64 {
            row["metrics"]
                .as_array()
                .expect("metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(
            automation_connectors["official_package_name"],
            "Automation Connectors"
        );
        assert_eq!(automation_connectors["upstream_package"], "n8n-nodes-base");
        assert_eq!(automation_connectors["upstream_version"], "2.22.0");
        assert_eq!(automation_connectors["status"], "present");
        assert_eq!(automation_connectors["receipt_status"], "present");
        assert_eq!(
            automation_connectors["receipt_hash_refresh"]["schema"],
            "dx.forge.package.receipt_hash_refresh"
        );
        assert_eq!(
            automation_connectors["receipt_hash_refresh"]["lower_dx_check_source"],
            automation_source
        );
        assert_eq!(
            automation_connectors["receipt_hash_refresh"]["check_panel_source"],
            automation_check_panel_source
        );
        assert_eq!(
            automation_connectors["receipt_hash_refresh"]["current_files"],
            serde_json::json!([
                "docs/packages/automation-connectors.source-guard-runbook.json",
                "tools/launch/materialize-www-template.ts",
                "dx-www/src/cli/studio_manifest.rs",
                automation_source,
                automation_check_panel_source
            ])
        );
        assert_eq!(
            automation_connectors["receipt_hash_refresh"]["zed_visibility"],
            "automation-connectors:receipt-hash-refresh"
        );
        assert_eq!(
            metric_value(
                automation_connectors,
                "automation_connectors_receipt_hash_refresh_current"
            ),
            1
        );
        assert_eq!(
            metric_value(
                automation_connectors,
                "automation_connectors_receipt_hash_refresh_stale"
            ),
            0
        );
        assert_eq!(
            metric_value(
                automation_connectors,
                "automation_connectors_receipt_hash_refresh_missing"
            ),
            0
        );
        assert_eq!(
            metric_value(automation_connectors, "automation_connectors_hash_mismatch"),
            0
        );
        assert_eq!(
            metric_value(
                automation_connectors,
                "automation_connectors_upstream_runtime_boundary_present"
            ),
            1
        );

        let mut stale_helper_status: serde_json::Value =
            serde_json::from_slice(&fs::read(&package_status_path).expect("package status bytes"))
                .expect("package status json");
        stale_helper_status["package_lane_visibility"][0]["receipt_hash_refresh"]["status"] =
            serde_json::json!("current");
        stale_helper_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_files"] =
            serde_json::json!(["tools/launch/materialize-www-template.ts"]);
        stale_helper_status["package_lane_visibility"][0]["receipt_hash_refresh"]["stale_mirror_files"] =
            serde_json::json!(["examples/template/forge-package-status-read-model.ts"]);
        fs::write(
            &package_status_path,
            serde_json::to_vec_pretty(&stale_helper_status)
                .expect("stale helper Automation Connectors package-status json"),
        )
        .expect("write stale helper Automation Connectors package status");

        let stale_report = read_dx_check_latest_panel(dir.path());
        let stale_view_model =
            serde_json::to_value(&stale_report.view_model).expect("stale view model json");
        let stale_automation_connectors = stale_view_model["package_lane_rows"]
            .as_array()
            .expect("stale package lane rows")
            .iter()
            .find(|row| row["package_id"] == "automations/n8n")
            .expect("stale Automation Connectors row");
        assert_eq!(
            stale_automation_connectors["receipt_hash_refresh"]["stale_files"],
            serde_json::json!(["tools/launch/materialize-www-template.ts"])
        );
        assert_eq!(
            stale_automation_connectors["receipt_hash_refresh"]["stale_mirror_files"],
            serde_json::json!(["examples/template/forge-package-status-read-model.ts"])
        );
        assert_eq!(
            metric_value(
                stale_automation_connectors,
                "automation_connectors_receipt_hash_refresh_current"
            ),
            0
        );
        assert_eq!(
            metric_value(
                stale_automation_connectors,
                "automation_connectors_receipt_hash_refresh_stale"
            ),
            1
        );
        assert_eq!(
            stale_automation_connectors["receipt_status"],
            serde_json::json!("stale")
        );
    }

    #[test]
    fn dx_check_panel_exposes_style_browser_compat_evidence_row() {
        let dir = TempDir::new().expect("temp dir");
        let receipt_path = dx_check_latest_receipt_path(dir.path());
        fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");

        let mut receipt: serde_json::Value =
            serde_json::from_str(sample_receipt()).expect("sample receipt json");
        receipt["sections"] = serde_json::json!([
            {
                "name": "dx-style",
                "score": 100,
                "traffic": "green",
                "metrics": [
                    { "name": "dx_style_browser_compat_receipt_present", "value": 1 },
                    { "name": "dx_style_browser_compat_contract_present", "value": 1 },
                    { "name": "dx_style_browser_compat_schema_supported", "value": 1 },
                    { "name": "dx_style_browser_compat_class_count", "value": 6 },
                    { "name": "dx_style_browser_compat_selector_class_count", "value": 1 },
                    { "name": "dx_style_browser_compat_full_autoprefixer_parity", "value": 0 },
                    { "name": "dx_style_browser_compat_full_tailwind_postcss_output_parity", "value": 0 },
                    { "name": "dx_style_tailwind_parity_state_alias_supported_classes", "value": 6 },
                    { "name": "dx_style_tailwind_equal_output_receipt_present", "value": 1 },
                    { "name": "dx_style_tailwind_equal_output_contract_present", "value": 1 },
                    { "name": "dx_style_tailwind_equal_output_schema_supported", "value": 1 },
                    { "name": "dx_style_tailwind_equal_output_class_count", "value": 6 },
                    { "name": "dx_style_tailwind_equal_output_equal_class_count", "value": 6 },
                    { "name": "dx_style_tailwind_equal_output_unsupported_classes", "value": 0 },
                    { "name": "dx_style_tailwind_equal_output_live_tailwind_execution", "value": 0 },
                    { "name": "dx_style_tailwind_equal_output_full_tailwind_parity", "value": 0 },
                    { "name": "dx_style_tailwind_equal_output_fair_speed_benchmark", "value": 0 }
                ],
                "findings": []
            }
        ]);
        fs::write(
            &receipt_path,
            serde_json::to_vec_pretty(&receipt).expect("style evidence receipt json"),
        )
        .expect("sample receipt");

        let report = read_dx_check_latest_panel(dir.path());
        let view_model = serde_json::to_value(&report.view_model).expect("view model json");
        let row = view_model["style_evidence_rows"]
            .as_array()
            .expect("style evidence rows")
            .iter()
            .find(|row| row["row_id"] == DX_STYLE_BROWSER_COMPAT_ROW_ID)
            .expect("dx-style browser compat row");
        let metric_value = |name: &str| -> u64 {
            row["metrics"]
                .as_array()
                .expect("style metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(row["title"], "dx-style browser compatibility");
        assert_eq!(row["status"], "present");
        assert_eq!(row["receipt_path"], DX_STYLE_CHECK_RECEIPT_PATH_FOR_PANEL);
        assert_eq!(
            row["fixture_path"],
            "related-crates/style/fixtures/tailwind-postcss-browser-compat.json"
        );
        assert_eq!(row["receipt_present"], true);
        assert_eq!(row["contract_present"], true);
        assert_eq!(row["schema_supported"], true);
        assert_eq!(row["class_count"], 6);
        assert_eq!(row["selector_class_count"], 1);
        assert_eq!(
            row["selector_class_examples"],
            serde_json::json!(["file:p-4"])
        );
        assert_eq!(row["tailwind_parity_state_alias_supported_class_count"], 6);
        assert_eq!(
            row["tailwind_parity_supported_state_alias_examples"],
            serde_json::json!([
                "target:p-4",
                "read-only:bg-blue-500",
                "indeterminate:opacity-100",
                "has-even:bg-blue-500",
                "not-visited:text-slate-900",
                "in-read-only:p-4"
            ])
        );
        assert_eq!(row["full_autoprefixer_parity"], false);
        assert_eq!(row["full_tailwind_postcss_output_parity"], false);
        assert_eq!(row["zed_visibility"], "dx-style:browser-compat");
        assert_eq!(metric_value("dx_style_browser_compat_receipt_present"), 1);
        assert_eq!(metric_value("dx_style_browser_compat_contract_present"), 1);
        assert_eq!(metric_value("dx_style_browser_compat_schema_supported"), 1);
        assert_eq!(metric_value("dx_style_browser_compat_class_count"), 6);
        assert_eq!(
            metric_value("dx_style_browser_compat_selector_class_count"),
            1
        );
        assert_eq!(
            metric_value("dx_style_browser_compat_full_autoprefixer_parity"),
            0
        );
        assert_eq!(
            metric_value("dx_style_browser_compat_full_tailwind_postcss_output_parity"),
            0
        );
        assert_eq!(
            metric_value("dx_style_tailwind_parity_state_alias_supported_classes"),
            6
        );
        assert!(
            row["next_action"]
                .as_str()
                .expect("next action")
                .contains("tailwind-postcss-browser-compat.json")
        );

        let equal_output_row = view_model["style_evidence_rows"]
            .as_array()
            .expect("style evidence rows")
            .iter()
            .find(|row| row["row_id"] == DX_STYLE_TAILWIND_EQUAL_OUTPUT_ROW_ID)
            .expect("dx-style Tailwind equal-output row");
        let equal_output_metric_value = |name: &str| -> u64 {
            equal_output_row["metrics"]
                .as_array()
                .expect("equal-output metrics")
                .iter()
                .find(|metric| metric["name"] == name)
                .and_then(|metric| metric["value"].as_u64())
                .unwrap_or_else(|| panic!("{name} missing"))
        };

        assert_eq!(
            equal_output_row["title"],
            "dx-style Tailwind equal-output canary"
        );
        assert_eq!(equal_output_row["status"], "present");
        assert_eq!(
            equal_output_row["fixture_path"],
            DX_STYLE_TAILWIND_EQUAL_OUTPUT_FIXTURE_PATH
        );
        assert_eq!(equal_output_row["class_count"], 6);
        assert_eq!(
            equal_output_row["zed_visibility"],
            "dx-style:tailwind-equal-output"
        );
        assert_eq!(
            equal_output_metric_value("dx_style_tailwind_equal_output_equal_class_count"),
            6
        );
        assert_eq!(
            equal_output_metric_value("dx_style_tailwind_equal_output_live_tailwind_execution"),
            0
        );
        assert!(
            equal_output_row["next_action"]
                .as_str()
                .expect("equal-output next action")
                .contains("tailwind-equal-output-canary.json")
        );
    }

    fn sample_receipt() -> &'static str {
        r#"{
  "schema_version": "dx.check.receipt",
  "score": 410,
  "max_score": 500,
  "zed": {
    "panel_kind": "project-health",
    "schema_version": "dx.check.zed_panel.v1",
    "source": "dx-check",
    "weight_profile": "dx-check.launch-default",
    "scoring_config": {
      "schema_version": "dx.check.scoring_config",
      "active_profile": "dx-check.launch-default",
      "status": "default",
      "config_present": false,
      "config_paths_checked": [
        ".dx/check/config.json",
        ".dx/check/config.toml",
        "dx.check.json",
        "dx.check.toml"
      ],
      "active_bucket_weights": [
        { "id": "code-quality", "label": "Code Quality", "weight": 100 },
        { "id": "structure", "label": "Structure", "weight": 100 },
        { "id": "web-performance", "label": "Web Performance", "weight": 100 },
        { "id": "dx-framework-health", "label": "DX Framework Health", "weight": 100 },
        { "id": "test-readiness", "label": "Test and Launch Readiness", "weight": 100 }
      ],
      "configured_bucket_weights": [],
      "applies_to_score": true,
      "next_action": "No scoring config was found; dx-check is using the launch-default 500-point profile."
    },
    "score_value": 410,
    "score_max": 500,
    "score_percent": 82,
    "score_estimated": true,
    "status": "warning",
    "generated_at_unix_ms": 1770000000000,
    "bucket_count": 5,
    "blocker_count": 0,
    "warning_count": 2,
    "quick_fix_count": 1,
    "receipt_path": ".dx/receipts/check/check-latest.json",
    "refresh_command": "dx check --json",
    "detail_command": "dx check score --json",
    "blockers": [],
    "warnings": [
      {
        "severity": "warning",
        "code": "web-lighthouse-skipped",
        "message": "Lighthouse was skipped by default.",
        "next_action": "Run an approved Lighthouse adapter."
      }
    ],
    "quick_fixes": [
      {
        "id": "next-action-1",
        "label": "Run web probe",
        "next_action": "Collect a bounded web receipt.",
        "command": "dx check web-perf --url <url> --json"
      }
    ],
    "sections": [
      {
        "id": "web-performance",
        "title": "Web performance",
        "weight": 100,
        "score": 60,
        "max_score": 100,
        "estimated": true,
        "status": "warning",
        "summary": "Web checks are adapter-ready."
      }
    ]
  }
}"#
    }
