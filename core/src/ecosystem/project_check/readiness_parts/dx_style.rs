const DEFAULT_DX_STYLE_CLASSES: [&str; 10] = [
    "dx-template",
    "dx-shell",
    "dx-page-shell",
    "dx-card",
    "dx-button",
    "dx-action",
    "dx-secondary-action",
    "dx-grid",
    "dx-stack",
    "dx-muted",
];

fn dx_style_section(root: &Path) -> Result<DxCheckSection> {
    let mut findings = Vec::new();
    let mut metrics = Vec::new();
    let source_paths = dx_style_source_paths(root);
    let style_receipt = dx_style_check_receipt(root);
    let theme_path = root.join(DX_STYLE_THEME_PATH);
    let generated_path = root.join(DX_STYLE_GENERATED_PATH);
    let theme = fs::read_to_string(&theme_path).unwrap_or_default();
    let generated = fs::read_to_string(&generated_path).unwrap_or_default();
    let missing_tokens = missing_dx_style_tokens(&theme);
    let hardcoded_colors = dx_style_hardcoded_color_findings(root, &source_paths)?;
    let tailwind_leakage = style_receipt
        .as_ref()
        .map(|receipt| {
            dx_style_receipt_findings(
                receipt,
                "tailwind_leakage_findings",
                "dx-style-tailwind-leakage",
                "Move official template styling to dx-style tokens and generated CSS.",
            )
        })
        .unwrap_or_else(|| {
            dx_style_tailwind_leakage_findings(root, &source_paths).unwrap_or_default()
        });
    let class_tokens = dx_style_class_tokens(root, &source_paths)?;
    let unused_generated_classes = if dx_style_should_report_unused_generated_classes(root) {
        style_receipt
            .as_ref()
            .and_then(|receipt| dx_style_receipt_string_array(receipt, "unused_generated_classes"))
            .unwrap_or_else(|| dx_style_unused_generated_classes(&generated, &class_tokens))
    } else {
        Vec::new()
    };
    let unused_generated_classes =
        dx_style_reportable_unused_generated_classes(unused_generated_classes);
    let source_hash = style_receipt
        .as_ref()
        .and_then(|receipt| {
            receipt
                .get("source_hash")
                .and_then(serde_json::Value::as_str)
                .map(str::to_string)
        })
        .unwrap_or(dx_style_source_hash(root, &source_paths)?);
    let stale_generated_css = style_receipt
        .as_ref()
        .and_then(|receipt| {
            receipt
                .get("stale_generated_css")
                .and_then(serde_json::Value::as_bool)
        })
        .unwrap_or_else(|| !generated.contains(&format!("dx-style source-hash: {source_hash}")));
    let tailwind_parity = dx_style_tailwind_parity_summary(root);
    let tailwind_equal_output = dx_style_tailwind_equal_output_summary(root);
    let browser_compat = dx_style_browser_compat_summary(root);
    let postcss_compat = dx_style_postcss_compat_summary(root);
    let unsupported_css_directives = dx_style_unsupported_css_directives_summary(root);
    let unsupported_scanned_classes = dx_style_unsupported_scanned_classes_summary(root);
    let package_ownership = dx_style_package_ownership_summary(root);
    let rule_metadata = dx_style_rule_metadata_summary(root);

    metrics.push(check_metric(
        "dx_style_source_files",
        source_paths.len() as u64,
    ));
    metrics.push(check_metric(
        "dx_style_required_tokens_missing",
        missing_tokens.len() as u64,
    ));
    metrics.push(check_metric(
        "dx_style_hardcoded_color_findings",
        hardcoded_colors.len() as u64,
    ));
    metrics.push(check_metric(
        "dx_style_tailwind_leakage_findings",
        tailwind_leakage.len() as u64,
    ));
    metrics.push(check_metric(
        "dx_style_unused_generated_classes",
        unused_generated_classes.len() as u64,
    ));
    metrics.push(check_metric(
        "dx_style_generated_css_stale",
        u64::from(stale_generated_css),
    ));
    metrics.push(check_metric(
        "dx_style_tailwind_parity_receipt_present",
        u64::from(tailwind_parity.receipt_file_present),
    ));
    metrics.push(check_metric(
        "dx_style_tailwind_parity_contract_present",
        u64::from(tailwind_parity.contract_present),
    ));
    metrics.push(check_metric(
        "dx_style_tailwind_parity_schema_supported",
        u64::from(tailwind_parity.schema_supported),
    ));
    metrics.push(check_metric(
        "dx_style_tailwind_parity_supported_classes",
        tailwind_parity.supported_class_count,
    ));
    metrics.push(check_metric(
        "dx_style_tailwind_parity_state_alias_supported_classes",
        tailwind_parity.supported_state_alias_examples.len() as u64,
    ));
    metrics.push(check_metric(
        "dx_style_tailwind_parity_unsupported_classes",
        tailwind_parity.unsupported_class_count,
    ));
    metrics.push(check_metric(
        "dx_style_tailwind_parity_intentional_differences",
        tailwind_parity.intentionally_different_class_count,
    ));
    metrics.push(check_metric(
        "dx_style_tailwind_equal_output_receipt_present",
        u64::from(tailwind_equal_output.receipt_file_present),
    ));
    metrics.push(check_metric(
        "dx_style_tailwind_equal_output_contract_present",
        u64::from(tailwind_equal_output.contract_present),
    ));
    metrics.push(check_metric(
        "dx_style_tailwind_equal_output_schema_supported",
        u64::from(tailwind_equal_output.schema_supported),
    ));
    metrics.push(check_metric(
        "dx_style_tailwind_equal_output_class_count",
        tailwind_equal_output.class_count,
    ));
    metrics.push(check_metric(
        "dx_style_tailwind_equal_output_equal_class_count",
        tailwind_equal_output.equal_output_class_count,
    ));
    metrics.push(check_metric(
        "dx_style_tailwind_equal_output_unsupported_classes",
        tailwind_equal_output.unsupported_class_count,
    ));
    metrics.push(check_metric(
        "dx_style_tailwind_equal_output_live_tailwind_execution",
        u64::from(tailwind_equal_output.live_tailwind_execution),
    ));
    metrics.push(check_metric(
        "dx_style_tailwind_equal_output_full_tailwind_parity",
        u64::from(tailwind_equal_output.full_tailwind_parity),
    ));
    metrics.push(check_metric(
        "dx_style_tailwind_equal_output_fair_speed_benchmark",
        u64::from(tailwind_equal_output.fair_speed_benchmark),
    ));
    metrics.push(check_metric(
        "dx_style_browser_compat_receipt_present",
        u64::from(browser_compat.receipt_file_present),
    ));
    metrics.push(check_metric(
        "dx_style_browser_compat_contract_present",
        u64::from(browser_compat.contract_present),
    ));
    metrics.push(check_metric(
        "dx_style_browser_compat_schema_supported",
        u64::from(browser_compat.schema_supported),
    ));
    metrics.push(check_metric(
        "dx_style_browser_compat_class_count",
        browser_compat.class_count,
    ));
    metrics.push(check_metric(
        "dx_style_browser_compat_selector_class_count",
        browser_compat.selector_class_count,
    ));
    metrics.push(check_metric(
        "dx_style_browser_compat_full_autoprefixer_parity",
        u64::from(browser_compat.full_autoprefixer_parity),
    ));
    metrics.push(check_metric(
        "dx_style_browser_compat_full_tailwind_postcss_output_parity",
        u64::from(browser_compat.full_tailwind_postcss_output_parity),
    ));
    metrics.push(check_metric(
        "postcss_compat_supported_count",
        postcss_compat.supported_count,
    ));
    metrics.push(check_metric(
        "postcss_compat_partial_count",
        postcss_compat.partial_count,
    ));
    metrics.push(check_metric(
        "postcss_compat_unsupported_count",
        postcss_compat.unsupported_count,
    ));
    metrics.push(check_metric(
        "dx_starter_replacement_score",
        postcss_compat.dx_starter_replacement_score,
    ));
    metrics.push(check_metric(
        "full_postcss_plugin_parity",
        u64::from(postcss_compat.full_postcss_plugin_parity),
    ));
    metrics.push(check_metric(
        "postcss_runtime_dependency_required",
        u64::from(postcss_compat.postcss_runtime_dependency_required),
    ));
    metrics.push(check_metric(
        "local_postcss_config_required",
        u64::from(postcss_compat.local_postcss_config_required),
    ));
    metrics.push(check_metric(
        "unsupported_transform_warnings",
        postcss_compat.unsupported_transform_warnings.len() as u64,
    ));
    metrics.push(check_metric(
        "dx_style_unsupported_scanned_class_receipt_present",
        u64::from(unsupported_scanned_classes.receipt_file_present),
    ));
    metrics.push(check_metric(
        "dx_style_unsupported_css_directive_receipt_present",
        u64::from(unsupported_css_directives.receipt_file_present),
    ));
    metrics.push(check_metric(
        "dx_style_unsupported_css_directives",
        unsupported_css_directives.unsupported_directive_count,
    ));
    metrics.push(check_metric(
        "dx_style_unsupported_scanned_classes",
        unsupported_scanned_classes.unsupported_class_count,
    ));
    metrics.push(check_metric(
        "dx_style_package_ownership_receipt_present",
        u64::from(package_ownership.receipt_file_present),
    ));
    metrics.push(check_metric(
        "dx_style_package_ownership_package_count",
        package_ownership.package_count,
    ));
    metrics.push(check_metric(
        "dx_style_package_ownership_generated_class_count",
        package_ownership.generated_class_count,
    ));
    metrics.push(check_metric(
        "dx_style_package_ownership_unsupported_class_count",
        package_ownership.unsupported_class_count,
    ));
    metrics.push(check_metric(
        "dx_style_rule_metadata_receipt_present",
        u64::from(rule_metadata.receipt_file_present),
    ));
    metrics.push(check_metric(
        "dx_style_rule_metadata_class_count",
        rule_metadata.metadata_count,
    ));
    metrics.push(check_metric(
        "dx_style_rule_metadata_editable_class_count",
        rule_metadata.editable_class_count,
    ));
    metrics.push(check_metric(
        "dx_style_rule_metadata_visual_property_count",
        rule_metadata.visual_property_count,
    ));
    metrics.push(check_metric(
        "dx_style_rule_metadata_token_reference_count",
        rule_metadata.token_reference_count,
    ));
    metrics.push(check_metric(
        "dx_style_rule_metadata_source_file_count",
        rule_metadata.source_file_count,
    ));

    if !theme_path.exists() {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "dx-style-missing-theme-file",
            "public www projects should define dx-style tokens in `styles/theme.css`",
            Some(DX_STYLE_THEME_PATH.to_string()),
            "Run `dx style build` or create `styles/theme.css` with the public launch tokens.",
        ));
    }

    if !generated_path.exists() {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "dx-style-missing-generated-css",
            "public www projects should commit generated normal CSS at `styles/generated.css`",
            Some(DX_STYLE_GENERATED_PATH.to_string()),
            "Run `dx style build` before checking or deploying the app.",
        ));
    }

    for token in missing_tokens {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            format!("dx-style-missing-token-{}", token.trim_start_matches("--")),
            format!("dx-style theme is missing the required `{token}` token"),
            Some(DX_STYLE_THEME_PATH.to_string()),
            "Use Vercel-like public theme tokens: background, foreground, muted, border, card, accent, success, warning, and danger.",
        ));
    }

    if stale_generated_css && generated_path.exists() {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "dx-style-stale-generated-css",
            "`styles/generated.css` does not match the current TSX/theme/Forge source hash",
            Some(DX_STYLE_GENERATED_PATH.to_string()),
            "Run `dx style build` after editing routes, components, styles, or Forge style surfaces.",
        ));
    }

    if let Some(parse_error) = &tailwind_parity.parse_error {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "dx-style-tailwind-parity-receipt-malformed",
            format!("dx-style Tailwind parity receipt could not be parsed: {parse_error}"),
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Run `dx style check` again so the style receipt can be regenerated.",
        ));
    } else if tailwind_parity.receipt_file_present && !tailwind_parity.contract_present {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "dx-style-tailwind-parity-contract-missing",
            "`dx style check` receipt is missing `tailwind_parity_receipt_contract`",
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Regenerate the style check receipt with the current dx-style CLI.",
        ));
    } else if tailwind_parity.contract_present && !tailwind_parity.schema_supported {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "dx-style-tailwind-parity-schema-unsupported",
            format!(
                "dx-style Tailwind parity receipt is not `{DX_STYLE_TAILWIND_PARITY_SCHEMA}`"
            ),
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Regenerate the style check receipt with the current dx-style CLI before comparing parity.",
        ));
    }

    if tailwind_parity.unsupported_class_count > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "dx-style-tailwind-parity-unsupported-fixtures",
            format!(
                "dx-style parity receipt lists {} unsupported Tailwind fixture class(es): {}",
                tailwind_parity.unsupported_class_count,
                style_parity_examples(&tailwind_parity.unsupported_class_examples)
            ),
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Do not claim full Tailwind parity; implement these utilities or document them as unsupported.",
        ));
    }

    if tailwind_parity.intentionally_different_class_count > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Info,
            "dx-style-tailwind-parity-intentional-differences",
            format!(
                "dx-style parity receipt lists {} intentional Tailwind difference(s): {}",
                tailwind_parity.intentionally_different_class_count,
                style_parity_examples(&tailwind_parity.intentionally_different_examples)
            ),
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Keep intentional differences explicit in the compatibility matrix and launch notes.",
        ));
    }

    if let Some(parse_error) = &tailwind_equal_output.parse_error {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "dx-style-tailwind-equal-output-receipt-malformed",
            format!("dx-style equal-output canary could not be parsed: {parse_error}"),
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Run `dx style check` again so the equal-output canary can be regenerated.",
        ));
    } else if tailwind_equal_output.receipt_file_present && !tailwind_equal_output.contract_present
    {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "dx-style-tailwind-equal-output-contract-missing",
            "`dx style check` receipt is missing `tailwind_equal_output_canary_contract`",
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Regenerate the style check receipt with the current dx-style CLI.",
        ));
    } else if tailwind_equal_output.contract_present && !tailwind_equal_output.schema_supported {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "dx-style-tailwind-equal-output-schema-unsupported",
            format!(
                "dx-style equal-output canary is not `{DX_STYLE_TAILWIND_EQUAL_OUTPUT_SCHEMA}`"
            ),
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Regenerate the style check receipt with the current dx-style CLI before comparing Tailwind output.",
        ));
    }

    if tailwind_equal_output.unsupported_class_count > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "dx-style-tailwind-equal-output-unsupported-fixtures",
            format!(
                "dx-style equal-output canary has {} unsupported class(es)",
                tailwind_equal_output.unsupported_class_count
            ),
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Keep the fair-comparison fixture limited to classes that produce dx-style CSS, or implement the missing utilities.",
        ));
    }

    if tailwind_equal_output.full_tailwind_parity {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "dx-style-tailwind-equal-output-full-parity-overclaim",
            "dx-style equal-output canary claims full Tailwind parity",
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Keep full parity false until universal utility/config/plugin coverage is proven by generated-output fixtures.",
        ));
    }

    if tailwind_equal_output.fair_speed_benchmark && !tailwind_equal_output.live_tailwind_execution
    {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "dx-style-tailwind-equal-output-fair-speed-overclaim",
            "dx-style equal-output canary claims fair speed benchmarking without live Tailwind execution",
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Only set `fair_speed_benchmark` after an approved equal-output run executes both dx-style and Tailwind on the same fixture.",
        ));
    }

    if let Some(parse_error) = &browser_compat.parse_error {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "dx-style-browser-compat-receipt-malformed",
            format!("dx-style browser-compat receipt could not be parsed: {parse_error}"),
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Run `dx style check` again so browser-compat evidence can be regenerated.",
        ));
    } else if browser_compat.receipt_file_present && !browser_compat.contract_present {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "dx-style-browser-compat-contract-missing",
            "`dx style check` receipt is missing `browser_compat_receipt_contract`",
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Regenerate the style check receipt with the current dx-style CLI.",
        ));
    } else if browser_compat.contract_present && !browser_compat.schema_supported {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "dx-style-browser-compat-schema-unsupported",
            format!("dx-style browser-compat receipt is not `{DX_STYLE_BROWSER_COMPAT_SCHEMA}`"),
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Regenerate the style check receipt with the current dx-style CLI before comparing browser compatibility.",
        ));
    }

    if browser_compat.full_autoprefixer_parity {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "dx-style-browser-compat-autoprefixer-parity-overclaim",
            "dx-style browser-compat receipt claims full Autoprefixer parity without a lower dx-check proof gate",
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Keep the receipt flag false until a measured equal-output Autoprefixer fixture is wired into dx-check.",
        ));
    }

    if browser_compat.full_tailwind_postcss_output_parity {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "dx-style-browser-compat-tailwind-postcss-output-overclaim",
            "dx-style browser-compat receipt claims full Tailwind/PostCSS output parity without a lower dx-check proof gate",
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Keep the receipt flag false until a measured Tailwind/PostCSS equal-output fixture is wired into dx-check.",
        ));
    }

    if let Some(parse_error) = &postcss_compat.parse_error {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "dx-style-postcss-compat-receipt-malformed",
            format!("dx-style PostCSS compatibility receipt could not be parsed: {parse_error}"),
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Run `dx style check` again so PostCSS replacement metrics can be regenerated.",
        ));
    } else if postcss_compat.receipt_file_present && !postcss_compat.contract_present {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "dx-style-postcss-compat-contract-missing",
            "`dx style check` receipt is missing `postcss_compatibility_contract`",
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Regenerate the style check receipt with the current dx-style CLI.",
        ));
    } else if postcss_compat.contract_present && !postcss_compat.schema_supported {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "dx-style-postcss-compat-schema-unsupported",
            "dx-style PostCSS compatibility contract schema is unsupported",
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Regenerate the style check receipt with the current dx-style CLI before trusting PostCSS replacement metrics.",
        ));
    }

    if postcss_compat.postcss_runtime_dependency_required
        || postcss_compat.local_postcss_config_required
    {
        findings.push(check_finding(
            DxSupplyChainSeverity::High,
            "dx-style-postcss-compat-local-postcss-required",
            "dx-style PostCSS compatibility receipt requires a PostCSS dependency or local PostCSS config",
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Official DX starters must keep PostCSS runtime/config requirements false.",
        ));
    }

    if postcss_compat.autoprefixer_parity_status == "full" {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "dx-style-postcss-compat-autoprefixer-overclaim",
            "dx-style PostCSS compatibility receipt claims full Autoprefixer parity",
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Keep Autoprefixer parity partial until target-browser equal-output CSS tests prove the generated declarations.",
        ));
    }

    if postcss_compat.full_postcss_plugin_parity {
        findings.push(check_finding(
            DxSupplyChainSeverity::Medium,
            "dx-style-postcss-compat-full-plugin-overclaim",
            "dx-style PostCSS compatibility receipt claims full arbitrary PostCSS plugin parity",
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Keep full plugin parity false until upstream-equivalent plugin fixtures prove the whole PostCSS ecosystem surface.",
        ));
    }

    if !postcss_compat.unsupported_transform_warnings.is_empty() {
        findings.push(check_finding(
            DxSupplyChainSeverity::Info,
            "dx-style-postcss-compat-unsupported-transforms",
            format!(
                "dx-style PostCSS replacement still has {} unsupported or partial transform warning(s): {}",
                postcss_compat.unsupported_transform_warnings.len(),
                style_parity_examples(&postcss_compat.unsupported_transform_warnings)
            ),
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Keep unsupported transform warnings visible in dx-check, Forge, Zed, and Studio receipts instead of claiming broad PostCSS parity.",
        ));
    }

    if let Some(parse_error) = &unsupported_css_directives.parse_error {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "dx-style-unsupported-css-directive-receipt-malformed",
            format!("dx-style unsupported CSS directive receipt fields could not be parsed: {parse_error}"),
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Run `dx style check` again so unsupported CSS directive findings can be regenerated.",
        ));
    } else if unsupported_css_directives.unsupported_directive_count > 0 {
        let examples = unsupported_css_directives
            .findings
            .iter()
            .take(8)
            .map(|finding| {
                if let Some(line) = finding.line {
                    format!("{}:{} {}", finding.file, line, finding.directive)
                } else {
                    format!("{} {}", finding.file, finding.directive)
                }
            })
            .collect::<Vec<_>>();
        findings.push(check_finding(
            DxSupplyChainSeverity::High,
            "dx-style-unsupported-css-directive",
            format!(
                "dx-style found {} unsupported CSS-first directive(s): {}",
                unsupported_css_directives.unsupported_directive_count,
                style_parity_examples(&examples)
            ),
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Remove unsupported Tailwind JS config/plugin directives or add explicit DX-owned support before claiming compatibility.",
        ));
    }

    if let Some(parse_error) = &unsupported_scanned_classes.parse_error {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "dx-style-unsupported-scanned-class-receipt-malformed",
            format!("dx-style unsupported scanned class receipt fields could not be parsed: {parse_error}"),
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Run `dx style check` again so unsupported class findings can be regenerated.",
        ));
    } else if unsupported_scanned_classes.unsupported_class_count > 0 {
        let examples = unsupported_scanned_classes
            .findings
            .iter()
            .take(8)
            .map(|finding| finding.class_name.clone())
            .collect::<Vec<_>>();
        findings.push(check_finding(
            DxSupplyChainSeverity::High,
            "dx-style-unsupported-scanned-class",
            format!(
                "dx-style scanned {} Tailwind-like utility class(es) that generated no CSS: {}",
                unsupported_scanned_classes.unsupported_class_count,
                style_parity_examples(&examples)
            ),
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Use supported dx-style utilities, add engine support, or move semantic component styling into authored CSS before launch.",
        ));
    }

    if let Some(parse_error) = &package_ownership.parse_error {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "dx-style-package-ownership-receipt-malformed",
            format!("dx-style package ownership receipt fields could not be parsed: {parse_error}"),
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Run `dx style check` again so Forge-owned style evidence can be regenerated.",
        ));
    } else if package_ownership.unsupported_class_count > 0 {
        let examples = package_ownership
            .unsupported_classes
            .iter()
            .take(8)
            .map(|finding| format!("{}: {}", finding.package_id, finding.class_name))
            .collect::<Vec<_>>();
        findings.push(check_finding(
            DxSupplyChainSeverity::High,
            "dx-style-package-owned-unsupported-class",
            format!(
                "dx-style package ownership records {} Forge-owned class(es) without generated CSS proof: {}",
                package_ownership.unsupported_class_count,
                style_parity_examples(&examples)
            ),
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Fix the package style surface, add dx-style engine support, or mark the package unsupported before launch.",
        ));
    }

    if let Some(parse_error) = &rule_metadata.parse_error {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "dx-style-rule-metadata-receipt-malformed",
            format!("dx-style rule metadata receipt fields could not be parsed: {parse_error}"),
            Some(DX_STYLE_CHECK_RECEIPT_PATH.to_string()),
            "Run `dx style check` again so Zed/DX Studio class-to-rule metadata can be regenerated.",
        ));
    }

    for finding in hardcoded_colors {
        findings.push(finding);
    }
    for finding in tailwind_leakage {
        findings.push(finding);
    }
    for class_name in unused_generated_classes {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "dx-style-unused-generated-class",
            format!("generated selector `.{class_name}` is not used by current TSX source"),
            Some(DX_STYLE_GENERATED_PATH.to_string()),
            "Run `dx style build` to remove stale generated selectors.",
        ));
    }

    let mut section = section_from_findings("dx-style", findings);
    section.metrics = metrics;
    Ok(section)
}

fn dx_style_source_paths(root: &Path) -> Vec<String> {
    contract_source_paths(
        root,
        &["app", "components", "styles", "forge"],
        &["ts", "tsx", "js", "jsx", "css"],
    )
    .into_iter()
    .filter(|path| path != DX_STYLE_GENERATED_PATH)
    .collect()
}

fn missing_dx_style_tokens(theme: &str) -> Vec<String> {
    REQUIRED_DX_STYLE_TOKENS
        .iter()
        .filter(|token| !theme.contains(&format!("{token}:")))
        .map(|token| token.to_string())
        .collect()
}

fn dx_style_hardcoded_color_findings(
    root: &Path,
    source_paths: &[String],
) -> Result<Vec<DxCheckFinding>> {
    let mut findings = Vec::new();
    for relative in source_paths {
        if relative == DX_STYLE_THEME_PATH {
            continue;
        }
        let content = fs::read_to_string(root.join(relative)).unwrap_or_default();
        for (line_index, line) in content.lines().enumerate() {
            if contains_hardcoded_color(line) {
                findings.push(check_finding(
                    DxSupplyChainSeverity::Medium,
                    "dx-style-hardcoded-color",
                    format!("`{relative}` line {} uses a hardcoded color", line_index + 1),
                    Some(relative.clone()),
                    "Use `styles/theme.css` tokens and generated dx-style classes instead of inline hex/rgb colors.",
                ));
            }
        }
    }
    Ok(findings)
}

fn dx_style_tailwind_leakage_findings(
    root: &Path,
    source_paths: &[String],
) -> Result<Vec<DxCheckFinding>> {
    let mut findings = Vec::new();
    for relative in source_paths {
        let content = fs::read_to_string(root.join(relative)).unwrap_or_default();
        for (line_index, line) in content.lines().enumerate() {
            let class_tokens = extract_source_class_tokens(line);
            let tailwind_tokens = class_tokens
                .iter()
                .filter(|token| looks_like_tailwind_utility(token))
                .cloned()
                .collect::<Vec<_>>();
            if line.contains("@tailwind")
                || line.contains("tailwind.config")
                || line.contains("tailwindcss")
                || line.contains("tailwind-compatible")
                || !tailwind_tokens.is_empty()
            {
                let detail = if tailwind_tokens.is_empty() {
                    "Tailwind setup/config reference".to_string()
                } else {
                    format!(
                        "Tailwind-like class token(s): {}",
                        tailwind_tokens.join(", ")
                    )
                };
                findings.push(check_finding(
                    DxSupplyChainSeverity::Medium,
                    "dx-style-tailwind-leakage",
                    format!(
                        "`{relative}` line {} leaks Tailwind styling: {detail}",
                        line_index + 1
                    ),
                    Some(relative.clone()),
                    "Move official template styling to dx-style tokens and generated CSS.",
                ));
            }
        }
    }
    Ok(findings)
}

fn dx_style_class_tokens(root: &Path, source_paths: &[String]) -> Result<BTreeSet<String>> {
    let mut classes = BTreeSet::new();
    for relative in source_paths {
        let content = fs::read_to_string(root.join(relative)).unwrap_or_default();
        for token in extract_source_class_tokens(&content) {
            if token.starts_with("dx-") {
                classes.insert(token);
            }
        }
    }
    Ok(classes)
}

fn dx_style_unused_generated_classes(
    generated: &str,
    class_tokens: &BTreeSet<String>,
) -> Vec<String> {
    generated
        .lines()
        .filter_map(|line| {
            line.trim()
                .strip_prefix('.')
                .and_then(|line| line.split([' ', '{', ':', ',']).next())
                .filter(|selector| selector.starts_with("dx-"))
                .map(str::to_string)
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .filter(|selector| {
            !class_tokens.contains(selector) && !dx_style_is_default_generated_class(selector)
        })
        .collect()
}

fn dx_style_should_report_unused_generated_classes(root: &Path) -> bool {
    root.join("app").is_dir()
}

fn dx_style_reportable_unused_generated_classes(classes: Vec<String>) -> Vec<String> {
    classes
        .into_iter()
        .filter(|class_name| !dx_style_is_default_generated_class(class_name))
        .collect()
}

fn dx_style_is_default_generated_class(class_name: &str) -> bool {
    DEFAULT_DX_STYLE_CLASSES.contains(&class_name)
}

fn dx_style_source_hash(root: &Path, source_paths: &[String]) -> Result<String> {
    let mut input = String::new();
    for relative in source_paths {
        input.push_str(relative);
        input.push('\n');
        input.push_str(&fs::read_to_string(root.join(relative)).unwrap_or_default());
        input.push('\n');
    }
    Ok(format!(
        "blake3:{}",
        blake3::hash(input.as_bytes()).to_hex()
    ))
}

fn dx_style_check_receipt(root: &Path) -> Option<serde_json::Value> {
    fs::read_to_string(root.join(DX_STYLE_CHECK_RECEIPT_PATH))
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
}

fn dx_style_receipt_string_array(receipt: &serde_json::Value, key: &str) -> Option<Vec<String>> {
    receipt
        .get(key)
        .and_then(serde_json::Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(str::to_string)
                .collect()
        })
}

fn dx_style_receipt_findings(
    receipt: &serde_json::Value,
    key: &str,
    default_code: &str,
    default_remediation: &str,
) -> Vec<DxCheckFinding> {
    receipt
        .get(key)
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .map(|finding| {
            let severity = finding
                .get("severity")
                .and_then(serde_json::Value::as_str)
                .map(dx_style_receipt_severity)
                .unwrap_or(DxSupplyChainSeverity::Medium);
            let code = finding
                .get("rule")
                .or_else(|| finding.get("code"))
                .and_then(serde_json::Value::as_str)
                .unwrap_or(default_code);
            let evidence_path = finding
                .get("source_file")
                .or_else(|| finding.get("evidence_path"))
                .and_then(serde_json::Value::as_str)
                .map(str::to_string);
            let message = finding
                .get("message")
                .or_else(|| finding.get("reason"))
                .and_then(serde_json::Value::as_str)
                .unwrap_or("dx-style receipt reported a style finding");
            let remediation = finding
                .get("remediation")
                .and_then(serde_json::Value::as_str)
                .unwrap_or(default_remediation);
            check_finding(
                severity,
                code.to_string(),
                message.to_string(),
                evidence_path,
                remediation.to_string(),
            )
        })
        .collect()
}

fn dx_style_receipt_severity(value: &str) -> DxSupplyChainSeverity {
    match value.to_ascii_lowercase().as_str() {
        "high" | "error" => DxSupplyChainSeverity::High,
        "low" | "warning" | "warn" => DxSupplyChainSeverity::Low,
        "info" | "note" => DxSupplyChainSeverity::Info,
        _ => DxSupplyChainSeverity::Medium,
    }
}

fn dx_style_tailwind_parity_summary(root: &Path) -> DxStyleTailwindParitySummary {
    let path = root.join(DX_STYLE_CHECK_RECEIPT_PATH);
    if !path.exists() {
        return DxStyleTailwindParitySummary::default();
    }

    let mut summary = DxStyleTailwindParitySummary {
        receipt_file_present: true,
        ..DxStyleTailwindParitySummary::default()
    };

    let content = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(error) => {
            summary.parse_error = Some(error.to_string());
            return summary;
        }
    };

    let receipt: serde_json::Value = match serde_json::from_str(&content) {
        Ok(receipt) => receipt,
        Err(error) => {
            summary.parse_error = Some(error.to_string());
            return summary;
        }
    };

    let Some(contract) = receipt.get("tailwind_parity_receipt_contract") else {
        return summary;
    };

    summary.contract_present = true;
    summary.schema_supported = contract
        .get("schema_version")
        .and_then(serde_json::Value::as_str)
        == Some(DX_STYLE_TAILWIND_PARITY_SCHEMA);
    summary.supported_class_count = json_u64(contract, "supported_class_count");
    summary.unsupported_class_count = json_u64(contract, "unsupported_class_count");
    summary.intentionally_different_class_count =
        json_u64(contract, "intentionally_different_class_count");
    summary.unsupported_class_examples = json_string_array(contract, "unsupported_class_examples");
    summary.intentionally_different_examples =
        json_string_array(contract, "intentionally_different_examples");
    summary.supported_state_alias_examples =
        tailwind_parity_supported_state_alias_examples(contract);
    summary
}

fn tailwind_parity_supported_state_alias_examples(contract: &serde_json::Value) -> Vec<String> {
    let Some(entries) = contract
        .get("entries")
        .and_then(serde_json::Value::as_array)
    else {
        return Vec::new();
    };

    DX_STYLE_TAILWIND_PARITY_STATE_ALIAS_CLASSES
        .iter()
        .filter(|class_name| {
            entries.iter().any(|entry| {
                entry.get("class_name").and_then(serde_json::Value::as_str) == Some(*class_name)
                    && entry.get("status").and_then(serde_json::Value::as_str) == Some("supported")
            })
        })
        .map(|class_name| (*class_name).to_string())
        .collect()
}

fn json_u64(value: &serde_json::Value, key: &str) -> u64 {
    value
        .get(key)
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0)
}

fn json_string_array(value: &serde_json::Value, key: &str) -> Vec<String> {
    value
        .get(key)
        .and_then(serde_json::Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn style_parity_examples(examples: &[String]) -> String {
    if examples.is_empty() {
        return "none recorded".to_string();
    }

    examples.join(", ")
}

fn contains_hardcoded_color(line: &str) -> bool {
    line.contains("rgb(") || line.contains("rgba(") || has_css_hex_color(line)
}

fn has_css_hex_color(line: &str) -> bool {
    let bytes = line.as_bytes();
    for index in 0..bytes.len() {
        if bytes[index] != b'#' {
            continue;
        }
        let hex_len = line[index + 1..]
            .chars()
            .take_while(|ch| ch.is_ascii_hexdigit())
            .count();
        if matches!(hex_len, 3 | 4 | 6 | 8) {
            return true;
        }
    }
    false
}

fn extract_source_class_tokens(content: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    for marker in ["className=\"", "className='", "class=\"", "class='"] {
        let quote = marker.chars().last().unwrap_or('"');
        let mut rest = content;
        while let Some(start) = rest.find(marker) {
            let after = &rest[start + marker.len()..];
            let Some(end) = after.find(quote) else {
                break;
            };
            tokens.extend(after[..end].split_whitespace().map(clean_style_token));
            rest = &after[end + 1..];
        }
    }
    tokens
        .into_iter()
        .filter(|token| !token.is_empty())
        .collect()
}

fn clean_style_token(token: &str) -> String {
    token
        .trim_matches(|ch: char| {
            matches!(
                ch,
                '`' | '"' | '\'' | '{' | '}' | '(' | ')' | '[' | ']' | ',' | ';'
            )
        })
        .to_string()
}

fn looks_like_tailwind_utility(token: &str) -> bool {
    if token.starts_with("dx-") || token.starts_with("data-") || token.starts_with("aria-") {
        return false;
    }
    let token = token
        .rsplit(':')
        .next()
        .unwrap_or(token)
        .trim_start_matches('!');
    let prefixes = [
        "bg-", "text-", "border-", "rounded-", "shadow", "p-", "px-", "py-", "pt-", "pb-", "pl-",
        "pr-", "m-", "mx-", "my-", "mt-", "mb-", "ml-", "mr-", "gap-", "w-", "h-", "min-", "max-",
        "grid", "flex", "items-", "justify-", "font-", "leading-",
    ];
    prefixes.iter().any(|prefix| token.starts_with(prefix))
}

fn is_barrel_file(relative: &str, content: &str) -> bool {
    let normalized = relative.replace('\\', "/");
    let Some(file_name) = normalized.rsplit('/').next() else {
        return false;
    };
    if !matches!(
        file_name,
        "index.ts" | "index.tsx" | "index.js" | "index.jsx"
    ) {
        return false;
    }
    let meaningful_lines = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .collect::<Vec<_>>();
    !meaningful_lines.is_empty()
        && meaningful_lines
            .iter()
            .all(|line| line.starts_with("export ") && line.contains(" from "))
}

fn is_public_package_boundary(relative: &str) -> bool {
    let normalized = relative.replace('\\', "/");
    normalized.starts_with("components/ui/")
        || normalized.starts_with("components/packages/")
        || normalized.starts_with("forge/")
}

fn contains_dynamic_import(content: &str) -> bool {
    content.contains("import(")
}

fn declares_client_component(content: &str) -> bool {
    content.lines().take(5).map(str::trim).any(|line| {
        matches!(
            line,
            r#""use client";"# | r#""use client""# | "'use client';" | "'use client'"
        )
    })
}

fn imports_server_boundary(content: &str) -> bool {
    content.contains(" from \"../server")
        || content.contains(" from '../../server")
        || content.contains(" from \"@/server")
        || content.contains(" from '@/server")
        || content.contains("import(\"../server")
        || content.contains("import(\"../../server")
        || content.contains("import(\"@/server")
        || content.contains("import('../server")
        || content.contains("import('../../server")
        || content.contains("import('@/server")
}

fn has_source_extension(path: &str) -> bool {
    path.rsplit('.').next().is_some_and(|extension| {
        matches!(extension, "ts" | "tsx" | "js" | "jsx" | "pg" | "cp" | "lyt")
    })
}
