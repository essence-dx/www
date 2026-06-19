fn dx_style_browser_compat_panel_rows(
    receipt_sections: &[DxCheckReceiptSection],
) -> Vec<DxCheckPanelStyleEvidenceRow> {
    let Some(section) = receipt_sections
        .iter()
        .find(|section| section.name == "dx-style")
    else {
        return Vec::new();
    };

    let receipt_present = receipt_metric(section, "dx_style_browser_compat_receipt_present") > 0;
    let contract_present = receipt_metric(section, "dx_style_browser_compat_contract_present") > 0;
    let schema_supported = receipt_metric(section, "dx_style_browser_compat_schema_supported") > 0;
    let class_count = receipt_metric(section, "dx_style_browser_compat_class_count");
    let selector_class_count =
        receipt_metric(section, "dx_style_browser_compat_selector_class_count");
    let tailwind_parity_state_alias_supported_class_count = receipt_metric(
        section,
        "dx_style_tailwind_parity_state_alias_supported_classes",
    );
    let full_autoprefixer_parity =
        receipt_metric(section, "dx_style_browser_compat_full_autoprefixer_parity") > 0;
    let full_tailwind_postcss_output_parity = receipt_metric(
        section,
        "dx_style_browser_compat_full_tailwind_postcss_output_parity",
    ) > 0;

    Vec::from([DxCheckPanelStyleEvidenceRow {
        row_id: DX_STYLE_BROWSER_COMPAT_ROW_ID.to_string(),
        title: "dx-style browser compatibility".to_string(),
        status: browser_compat_panel_status(
            receipt_present,
            contract_present,
            schema_supported,
            full_autoprefixer_parity,
            full_tailwind_postcss_output_parity,
        )
        .to_string(),
        receipt_path: DX_STYLE_CHECK_RECEIPT_PATH_FOR_PANEL.to_string(),
        fixture_path: DX_STYLE_BROWSER_COMPAT_FIXTURE_PATH.to_string(),
        receipt_present,
        contract_present,
        schema_supported,
        class_count,
        selector_class_count,
        selector_class_examples: dx_style_browser_compat_selector_examples(selector_class_count),
        tailwind_parity_state_alias_supported_class_count,
        tailwind_parity_supported_state_alias_examples:
            dx_style_tailwind_parity_supported_state_alias_examples(
                tailwind_parity_state_alias_supported_class_count,
            ),
        package_ownership_package_count: None,
        package_ownership_generated_class_count: None,
        package_ownership_unsupported_class_count: None,
        package_ownership_package_ids: Vec::new(),
        package_ownership_unsupported_class_examples: Vec::new(),
        rule_metadata_visual_properties: Vec::new(),
        rule_metadata_source_files: Vec::new(),
        rule_metadata_token_references: Vec::new(),
        rule_metadata_editable_class_count: 0,
        rule_metadata_zed_studio_editable: false,
        full_autoprefixer_parity,
        full_tailwind_postcss_output_parity,
        metrics: DX_STYLE_BROWSER_COMPAT_METRICS
            .iter()
            .map(|name| DxCheckPanelStyleEvidenceMetric {
                name: (*name).to_string(),
                value: receipt_metric(section, name),
            })
            .collect(),
        zed_visibility: "dx-style:browser-compat".to_string(),
        runtime_limitations: vec![
            "SOURCE-ONLY: this row mirrors lower dx-check receipt metrics; it does not execute Tailwind/PostCSS or a browser.".to_string(),
            "PARTIAL-PARITY: the checked-in canary tracks declaration fragments only and must not be read as full Autoprefixer or Tailwind/PostCSS output parity.".to_string(),
            "SELECTOR-CANARY: selector-level examples such as file:p-4 prove targeted generated selector fragments, not full browser selector parity.".to_string(),
            "STATE-ALIAS-PARTIAL: supported state-alias examples are receipt-backed canaries, not full Tailwind arbitrary variant grammar.".to_string(),
        ],
        next_action: browser_compat_panel_next_action(
            receipt_present,
            contract_present,
            schema_supported,
            full_autoprefixer_parity,
            full_tailwind_postcss_output_parity,
        )
        .to_string(),
    }])
}

fn dx_style_panel_rows(
    root: &Path,
    receipt_sections: &[DxCheckReceiptSection],
) -> Vec<DxCheckPanelStyleEvidenceRow> {
    let mut rows = dx_style_browser_compat_panel_rows(receipt_sections);
    if let Some(row) = dx_style_tailwind_equal_output_panel_row(receipt_sections) {
        rows.push(row);
    }
    if let Some(row) = dx_style_package_ownership_panel_row(root, receipt_sections) {
        rows.push(row);
    }
    if let Some(row) = dx_style_rule_metadata_panel_row(root, receipt_sections) {
        rows.push(row);
    }
    rows
}

fn dx_style_tailwind_equal_output_panel_row(
    receipt_sections: &[DxCheckReceiptSection],
) -> Option<DxCheckPanelStyleEvidenceRow> {
    let section = receipt_sections
        .iter()
        .find(|section| section.name == "dx-style")?;
    let receipt_present =
        receipt_metric(section, "dx_style_tailwind_equal_output_receipt_present") > 0;
    let contract_present =
        receipt_metric(section, "dx_style_tailwind_equal_output_contract_present") > 0;
    let schema_supported =
        receipt_metric(section, "dx_style_tailwind_equal_output_schema_supported") > 0;
    let class_count = receipt_metric(section, "dx_style_tailwind_equal_output_class_count");
    let equal_class_count =
        receipt_metric(section, "dx_style_tailwind_equal_output_equal_class_count");
    let unsupported_class_count = receipt_metric(
        section,
        "dx_style_tailwind_equal_output_unsupported_classes",
    );
    let live_tailwind_execution = receipt_metric(
        section,
        "dx_style_tailwind_equal_output_live_tailwind_execution",
    ) > 0;
    let full_tailwind_parity = receipt_metric(
        section,
        "dx_style_tailwind_equal_output_full_tailwind_parity",
    ) > 0;
    let fair_speed_benchmark = receipt_metric(
        section,
        "dx_style_tailwind_equal_output_fair_speed_benchmark",
    ) > 0;

    Some(DxCheckPanelStyleEvidenceRow {
        row_id: DX_STYLE_TAILWIND_EQUAL_OUTPUT_ROW_ID.to_string(),
        title: "dx-style Tailwind equal-output canary".to_string(),
        status: tailwind_equal_output_panel_status(
            receipt_present,
            contract_present,
            schema_supported,
            unsupported_class_count,
            full_tailwind_parity,
            fair_speed_benchmark,
            live_tailwind_execution,
        )
        .to_string(),
        receipt_path: DX_STYLE_CHECK_RECEIPT_PATH_FOR_PANEL.to_string(),
        fixture_path: DX_STYLE_TAILWIND_EQUAL_OUTPUT_FIXTURE_PATH.to_string(),
        receipt_present,
        contract_present,
        schema_supported,
        class_count,
        selector_class_count: 0,
        selector_class_examples: Vec::new(),
        tailwind_parity_state_alias_supported_class_count: 0,
        tailwind_parity_supported_state_alias_examples: Vec::new(),
        package_ownership_package_count: None,
        package_ownership_generated_class_count: None,
        package_ownership_unsupported_class_count: None,
        package_ownership_package_ids: Vec::new(),
        package_ownership_unsupported_class_examples: Vec::new(),
        rule_metadata_visual_properties: Vec::new(),
        rule_metadata_source_files: Vec::new(),
        rule_metadata_token_references: Vec::new(),
        rule_metadata_editable_class_count: 0,
        rule_metadata_zed_studio_editable: false,
        full_autoprefixer_parity: false,
        full_tailwind_postcss_output_parity: full_tailwind_parity,
        metrics: DX_STYLE_TAILWIND_EQUAL_OUTPUT_METRICS
            .iter()
            .map(|name| DxCheckPanelStyleEvidenceMetric {
                name: (*name).to_string(),
                value: receipt_metric(section, name),
            })
            .collect(),
        zed_visibility: "dx-style:tailwind-equal-output".to_string(),
        runtime_limitations: vec![
            "SOURCE-ONLY: this row mirrors a checked-in equal-output canary and does not execute Tailwind live.".to_string(),
            "NOT-A-BENCHMARK: fair speed comparison remains false until both engines generate equal CSS in an approved run.".to_string(),
            "PARTIAL-PARITY: this is a tiny supported-subset fixture, not universal Tailwind utility/config/plugin parity.".to_string(),
        ],
        next_action: tailwind_equal_output_panel_next_action(
            receipt_present,
            contract_present,
            schema_supported,
            class_count,
            equal_class_count,
            unsupported_class_count,
            fair_speed_benchmark,
            live_tailwind_execution,
        )
        .to_string(),
    })
}

fn dx_style_package_ownership_panel_row(
    root: &Path,
    receipt_sections: &[DxCheckReceiptSection],
) -> Option<DxCheckPanelStyleEvidenceRow> {
    let section = receipt_sections
        .iter()
        .find(|section| section.name == "dx-style")?;
    let receipt_present = receipt_metric(section, "dx_style_package_ownership_receipt_present") > 0;
    let package_count = receipt_metric(section, "dx_style_package_ownership_package_count");
    let generated_class_count =
        receipt_metric(section, "dx_style_package_ownership_generated_class_count");
    let unsupported_class_count = receipt_metric(
        section,
        "dx_style_package_ownership_unsupported_class_count",
    );
    let summary = dx_style_package_ownership_summary(root);

    Some(DxCheckPanelStyleEvidenceRow {
        row_id: DX_STYLE_PACKAGE_OWNERSHIP_ROW_ID.to_string(),
        title: "dx-style package ownership".to_string(),
        status: package_ownership_panel_status(receipt_present, package_count, unsupported_class_count)
            .to_string(),
        receipt_path: DX_STYLE_CHECK_RECEIPT_PATH_FOR_PANEL.to_string(),
        fixture_path: DX_STYLE_PACKAGE_OWNERSHIP_FIXTURE_PATH.to_string(),
        receipt_present,
        contract_present: package_count > 0,
        schema_supported: true,
        class_count: generated_class_count,
        selector_class_count: 0,
        selector_class_examples: Vec::new(),
        tailwind_parity_state_alias_supported_class_count: 0,
        tailwind_parity_supported_state_alias_examples: Vec::new(),
        package_ownership_package_count: Some(package_count),
        package_ownership_generated_class_count: Some(generated_class_count),
        package_ownership_unsupported_class_count: Some(unsupported_class_count),
        package_ownership_package_ids: summary.package_ids,
        package_ownership_unsupported_class_examples: summary
            .unsupported_classes
            .into_iter()
            .take(8)
            .map(|finding| format!("{}: {}", finding.package_id, finding.class_name))
            .collect(),
        rule_metadata_visual_properties: Vec::new(),
        rule_metadata_source_files: Vec::new(),
        rule_metadata_token_references: Vec::new(),
        rule_metadata_editable_class_count: 0,
        rule_metadata_zed_studio_editable: false,
        full_autoprefixer_parity: false,
        full_tailwind_postcss_output_parity: false,
        metrics: DX_STYLE_PACKAGE_OWNERSHIP_METRICS
            .iter()
            .map(|name| DxCheckPanelStyleEvidenceMetric {
                name: (*name).to_string(),
                value: receipt_metric(section, name),
            })
            .collect(),
        zed_visibility: "dx-style:package-ownership".to_string(),
        runtime_limitations: vec![
            "SOURCE-ONLY: this row mirrors lower dx-check package ownership metrics and does not execute package runtime code.".to_string(),
            "PACKAGE-SCOPED: unsupported classes are attributed to Forge package ids so launch blockers are not anonymous.".to_string(),
            "PARTIAL-PARITY: package ownership evidence does not claim universal Tailwind config/plugin/theme parity.".to_string(),
        ],
        next_action: package_ownership_panel_next_action(
            receipt_present,
            package_count,
            unsupported_class_count,
        )
        .to_string(),
    })
}

fn dx_style_rule_metadata_panel_row(
    root: &Path,
    receipt_sections: &[DxCheckReceiptSection],
) -> Option<DxCheckPanelStyleEvidenceRow> {
    let section = receipt_sections
        .iter()
        .find(|section| section.name == "dx-style")?;
    let receipt_present = receipt_metric(section, "dx_style_rule_metadata_receipt_present") > 0;
    let class_count = receipt_metric(section, "dx_style_rule_metadata_class_count");
    let editable_class_count =
        receipt_metric(section, "dx_style_rule_metadata_editable_class_count");
    let summary = dx_style_rule_metadata_summary(root);
    let zed_studio_editable = class_count > 0 && editable_class_count == class_count;

    Some(DxCheckPanelStyleEvidenceRow {
        row_id: DX_STYLE_RULE_METADATA_ROW_ID.to_string(),
        title: "dx-style rule metadata".to_string(),
        status: rule_metadata_panel_status(receipt_present, class_count, editable_class_count)
            .to_string(),
        receipt_path: DX_STYLE_CHECK_RECEIPT_PATH_FOR_PANEL.to_string(),
        fixture_path: DX_STYLE_CHECK_RECEIPT_PATH_FOR_PANEL.to_string(),
        receipt_present,
        contract_present: class_count > 0,
        schema_supported: true,
        class_count,
        selector_class_count: 0,
        selector_class_examples: Vec::new(),
        tailwind_parity_state_alias_supported_class_count: 0,
        tailwind_parity_supported_state_alias_examples: Vec::new(),
        package_ownership_package_count: None,
        package_ownership_generated_class_count: None,
        package_ownership_unsupported_class_count: None,
        package_ownership_package_ids: Vec::new(),
        package_ownership_unsupported_class_examples: Vec::new(),
        rule_metadata_visual_properties: summary.visual_properties,
        rule_metadata_source_files: summary.source_files,
        rule_metadata_token_references: summary.token_references,
        rule_metadata_editable_class_count: editable_class_count,
        rule_metadata_zed_studio_editable: zed_studio_editable,
        full_autoprefixer_parity: false,
        full_tailwind_postcss_output_parity: false,
        metrics: DX_STYLE_RULE_METADATA_METRICS
            .iter()
            .map(|name| DxCheckPanelStyleEvidenceMetric {
                name: (*name).to_string(),
                value: receipt_metric(section, name),
            })
            .collect(),
        zed_visibility: "dx-style:rule-metadata".to_string(),
        runtime_limitations: vec![
            "SOURCE-ONLY: this row mirrors generated class-to-rule metadata from `.dx/receipts/style/check.json`.".to_string(),
            "EDITOR-MAPPING: source files, visual properties, and token references are evidence for Zed/DX Studio selection, not live browser inspection.".to_string(),
            "PARTIAL-PARITY: rule metadata proves traceability for generated dx-style CSS; it does not prove universal Tailwind/PostCSS parity.".to_string(),
        ],
        next_action: rule_metadata_panel_next_action(
            receipt_present,
            class_count,
            editable_class_count,
        )
        .to_string(),
    })
}

fn dx_style_browser_compat_selector_examples(count: u64) -> Vec<String> {
    let count = count.min(DX_STYLE_BROWSER_COMPAT_SELECTOR_EXAMPLES.len() as u64) as usize;
    DX_STYLE_BROWSER_COMPAT_SELECTOR_EXAMPLES[..count]
        .iter()
        .map(|class_name| (*class_name).to_string())
        .collect()
}

fn dx_style_tailwind_parity_supported_state_alias_examples(count: u64) -> Vec<String> {
    let count = count.min(DX_STYLE_TAILWIND_PARITY_STATE_ALIAS_EXAMPLES.len() as u64) as usize;
    DX_STYLE_TAILWIND_PARITY_STATE_ALIAS_EXAMPLES[..count]
        .iter()
        .map(|class_name| (*class_name).to_string())
        .collect()
}

fn receipt_metric(section: &DxCheckReceiptSection, name: &str) -> u64 {
    section
        .metrics
        .iter()
        .find(|metric| metric.name == name)
        .map(|metric| metric.value)
        .unwrap_or(0)
}

fn browser_compat_panel_status(
    receipt_present: bool,
    contract_present: bool,
    schema_supported: bool,
    full_autoprefixer_parity: bool,
    full_tailwind_postcss_output_parity: bool,
) -> &'static str {
    if !receipt_present {
        return "missing-receipt";
    }
    if !contract_present {
        return "missing-contract";
    }
    if !schema_supported {
        return "unsupported-schema";
    }
    if full_autoprefixer_parity || full_tailwind_postcss_output_parity {
        return "overclaimed";
    }
    "present"
}

fn tailwind_equal_output_panel_status(
    receipt_present: bool,
    contract_present: bool,
    schema_supported: bool,
    unsupported_class_count: u64,
    full_tailwind_parity: bool,
    fair_speed_benchmark: bool,
    live_tailwind_execution: bool,
) -> &'static str {
    if !receipt_present {
        return "missing-receipt";
    }
    if !contract_present {
        return "missing-contract";
    }
    if !schema_supported {
        return "unsupported-schema";
    }
    if unsupported_class_count > 0 {
        return "blocked";
    }
    if full_tailwind_parity || (fair_speed_benchmark && !live_tailwind_execution) {
        return "overclaimed";
    }
    "present"
}

fn package_ownership_panel_status(
    receipt_present: bool,
    package_count: u64,
    unsupported_class_count: u64,
) -> &'static str {
    if !receipt_present {
        return "missing-receipt";
    }
    if package_count == 0 {
        return "missing-package-ownership";
    }
    if unsupported_class_count > 0 {
        return "blocked";
    }
    "present"
}

fn rule_metadata_panel_status(
    receipt_present: bool,
    class_count: u64,
    editable_class_count: u64,
) -> &'static str {
    if !receipt_present {
        return "missing-receipt";
    }
    if class_count == 0 {
        return "missing-rule-metadata";
    }
    if editable_class_count < class_count {
        return "partial-editor-metadata";
    }
    "present"
}

fn browser_compat_panel_next_action(
    receipt_present: bool,
    contract_present: bool,
    schema_supported: bool,
    full_autoprefixer_parity: bool,
    full_tailwind_postcss_output_parity: bool,
) -> &'static str {
    if !receipt_present {
        return "Run `dx style check` so `.dx/receipts/style/check.json` records browser-compat evidence.";
    }
    if !contract_present {
        return "Regenerate the style receipt with the current dx-style CLI so `browser_compat_receipt_contract` is present.";
    }
    if !schema_supported {
        return "Regenerate the style receipt with the current dx-style schema before trusting browser-compat evidence.";
    }
    if full_autoprefixer_parity || full_tailwind_postcss_output_parity {
        return "Turn full parity flags back to false until an equal-output Tailwind/PostCSS fixture proves them.";
    }
    "Keep expanding `tailwind-postcss-browser-compat.json` declaration fixtures and selector canaries with measured Tailwind/PostCSS output before claiming full browser-prefix parity."
}

#[allow(clippy::too_many_arguments)]
fn tailwind_equal_output_panel_next_action(
    receipt_present: bool,
    contract_present: bool,
    schema_supported: bool,
    class_count: u64,
    equal_class_count: u64,
    unsupported_class_count: u64,
    fair_speed_benchmark: bool,
    live_tailwind_execution: bool,
) -> &'static str {
    if !receipt_present {
        return "Run `dx style check` so the equal-output canary receipt is available.";
    }
    if !contract_present {
        return "Regenerate the style receipt with the current dx-style CLI so `tailwind_equal_output_canary_contract` is present.";
    }
    if !schema_supported {
        return "Regenerate the equal-output canary with the current schema before trusting comparison evidence.";
    }
    if unsupported_class_count > 0 || equal_class_count < class_count {
        return "Keep the equal-output canary limited to classes that generate matching CSS, or implement the missing utility output.";
    }
    if fair_speed_benchmark && !live_tailwind_execution {
        return "Turn fair speed benchmark claims off until an approved equal-output run executes Tailwind and dx-style on the same fixture.";
    }
    "Use `tailwind-equal-output-canary.json` as the tiny fair-comparison seed, then run a governed equal-output Tailwind benchmark before making speed claims."
}

fn package_ownership_panel_next_action(
    receipt_present: bool,
    package_count: u64,
    unsupported_class_count: u64,
) -> &'static str {
    if !receipt_present {
        return "Run `dx style check` so `.dx/receipts/style/check.json` records package-owned style evidence.";
    }
    if package_count == 0 {
        return "Publish `style_package_ownership_rows` in the style check receipt from Forge package style surfaces.";
    }
    if unsupported_class_count > 0 {
        return "Fix or explicitly block package-owned unsupported classes before claiming Forge package style readiness.";
    }
    "Keep package-owned generated class and token evidence in the style receipt so Forge/Zed diagnostics stay attributable."
}

fn rule_metadata_panel_next_action(
    receipt_present: bool,
    class_count: u64,
    editable_class_count: u64,
) -> &'static str {
    if !receipt_present {
        return "Run `dx style check` so generated CSS rule metadata is available to dx-check, Zed, and DX Studio.";
    }
    if class_count == 0 {
        return "Regenerate the style receipt with the current dx-style CLI so `style_rule_metadata` rows are present.";
    }
    if editable_class_count < class_count {
        return "Keep class-to-rule metadata complete enough for Zed/DX Studio to map classes, source files, visual properties, and tokens.";
    }
    "Use `style_rule_metadata` as the shared class-to-rule map for Zed/DX Studio visual editing, then keep widening visual property coverage."
}

fn finding_row(finding: &DxCheckZedFinding) -> DxCheckPanelFindingRow {
    DxCheckPanelFindingRow {
        severity: finding.severity.clone(),
        code: finding.code.clone(),
        message: finding.message.clone(),
        evidence_path: finding.evidence_path.clone(),
        next_action: finding.next_action.clone(),
    }
}

fn quick_fix_row(fix: &DxCheckZedQuickFix) -> DxCheckPanelQuickFixRow {
    let mut fix = fix.clone();
    normalize_quick_fix_metadata(&mut fix);

    DxCheckPanelQuickFixRow {
        id: fix.id.clone(),
        label: fix.label.clone(),
        next_action: fix.next_action.clone(),
        risk_level: fix.risk_level.clone(),
        requires_user_approval: fix.requires_user_approval,
        writes_receipts: fix.writes_receipts,
        command: fix.command.clone(),
    }
}

fn normalize_zed_panel(mut zed: DxCheckZedPanel) -> DxCheckZedPanel {
    for fix in &mut zed.quick_fixes {
        normalize_quick_fix_metadata(fix);
    }
    zed
}

fn normalize_quick_fix_metadata(fix: &mut DxCheckZedQuickFix) {
    let command = fix.command.as_deref();
    if fix.risk_level == default_quick_fix_risk_level() {
        fix.risk_level = quick_fix_risk_level(command).to_string();
    }
    fix.requires_user_approval = fix.requires_user_approval || quick_fix_requires_approval(command);
    fix.writes_receipts = fix.writes_receipts || quick_fix_writes_receipts(command);
}

fn default_quick_fix_risk_level() -> String {
    "unknown".to_string()
}

fn quick_fix_risk_level(command: Option<&str>) -> &'static str {
    let Some(command) = command else {
        return "manual";
    };

    if command.contains("--run ") || command.contains("--run-web") || command.contains("--run-e2e")
    {
        "executes-approved-runner"
    } else if command.contains("--https-probe") {
        "network-probe"
    } else if command.contains("--lighthouse-json")
        || command.contains("--cdp-json")
        || command.contains("--smoke-evidence")
    {
        "evidence-import"
    } else if command.starts_with("dx check") {
        "receipt-write"
    } else {
        "manual"
    }
}

fn quick_fix_requires_approval(command: Option<&str>) -> bool {
    command.is_some_and(|command| {
        command.contains("--run ")
            || command.contains("--run-web")
            || command.contains("--run-e2e")
            || command.contains("--https-probe")
    })
}

fn quick_fix_writes_receipts(command: Option<&str>) -> bool {
    command.is_some_and(|command| command == "dx check --json" || command.starts_with("dx check "))
}
