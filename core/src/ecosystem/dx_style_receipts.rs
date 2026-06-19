//! Receipt readers for dx-style check output.

use super::json_receipt_machine::read_json_receipt_machine_alias;
use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

const DX_STYLE_CHECK_RECEIPT_PATH: &str = ".dx/receipts/style/check.json";
const DX_STYLE_CHECK_MACHINE_PATH: &str = ".dx/www/style-check-receipt.machine";

/// Schema emitted by dx-style for the Tailwind/PostCSS browser-compat canary.
pub const DX_STYLE_BROWSER_COMPAT_SCHEMA: &str = "dx.style.tailwindPostcssBrowserCompatFixture";

/// Schema emitted by dx-style for the Tailwind equal-output canary.
pub const DX_STYLE_TAILWIND_EQUAL_OUTPUT_SCHEMA: &str = "dx.style.tailwindEqualOutputCanary";

/// Schema emitted by dx-style for the PostCSS compatibility matrix.
pub const DX_STYLE_POSTCSS_COMPAT_SCHEMA: &str = "dx.style.postcssCompatibilityMatrix";

/// Schema emitted by dx-style for class-to-generated-rule metadata.
pub const DX_STYLE_RULE_METADATA_SCHEMA: &str = "dx.style.rule_metadata";

/// Rule emitted by `dx style check` for scanned utility classes with no CSS output.
pub const DX_STYLE_UNSUPPORTED_SCANNED_CLASS_RULE: &str = "dx-style-unsupported-scanned-class";

/// Rule emitted by `dx style check` for unsupported CSS-first directives.
pub const DX_STYLE_UNSUPPORTED_CSS_DIRECTIVE_RULE: &str = "dx-style-unsupported-css-directive";

/// Browser compatibility canary summary from the style check receipt.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DxStyleBrowserCompatSummary {
    /// Whether the receipt file exists.
    pub receipt_file_present: bool,
    /// Whether the browser compatibility contract is present in the receipt.
    pub contract_present: bool,
    /// Whether the browser compatibility contract schema is supported by dx-check.
    pub schema_supported: bool,
    /// Number of classes covered by the browser compatibility canary.
    pub class_count: u64,
    /// Number of selector-level classes covered by the browser compatibility canary.
    pub selector_class_count: u64,
    /// Whether the receipt claims full Autoprefixer parity.
    pub full_autoprefixer_parity: bool,
    /// Whether the receipt claims full Tailwind/PostCSS output parity.
    pub full_tailwind_postcss_output_parity: bool,
    /// Receipt read or parse error, when present.
    pub parse_error: Option<String>,
}

/// Equal-output canary summary from the style check receipt.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DxStyleTailwindEqualOutputSummary {
    /// Whether the receipt file exists.
    pub receipt_file_present: bool,
    /// Whether the equal-output contract is present in the receipt.
    pub contract_present: bool,
    /// Whether the equal-output contract schema is supported by dx-check.
    pub schema_supported: bool,
    /// Number of classes covered by the canary.
    pub class_count: u64,
    /// Number of classes with matching declaration fragments.
    pub equal_output_class_count: u64,
    /// Number of classes unsupported by dx-style.
    pub unsupported_class_count: u64,
    /// Whether Tailwind was executed live to produce this receipt.
    pub live_tailwind_execution: bool,
    /// Whether the receipt claims universal Tailwind parity.
    pub full_tailwind_parity: bool,
    /// Whether the receipt claims fair speed-benchmark evidence.
    pub fair_speed_benchmark: bool,
    /// Receipt read or parse error, when present.
    pub parse_error: Option<String>,
}

/// PostCSS compatibility summary from the style check receipt.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DxStylePostcssCompatSummary {
    /// Whether the receipt file exists.
    pub receipt_file_present: bool,
    /// Whether the PostCSS compatibility contract is present.
    pub contract_present: bool,
    /// Whether the contract schema is supported by dx-check.
    pub schema_supported: bool,
    /// Supported feature count from the matrix.
    pub supported_count: u64,
    /// Partial feature count from the matrix.
    pub partial_count: u64,
    /// Unsupported feature count from the matrix.
    pub unsupported_count: u64,
    /// Official DX starter replacement score for this governed compatibility surface.
    pub dx_starter_replacement_score: u64,
    /// Whether the contract claims full arbitrary PostCSS plugin parity.
    pub full_postcss_plugin_parity: bool,
    /// Broad plugin-ecosystem parity status.
    pub postcss_plugin_parity_status: String,
    /// Autoprefixer parity state.
    pub autoprefixer_parity_status: String,
    /// Whether a PostCSS runtime/build dependency is required.
    pub postcss_runtime_dependency_required: bool,
    /// Whether a local PostCSS config is required.
    pub local_postcss_config_required: bool,
    /// Unsupported transform warning strings.
    pub unsupported_transform_warnings: Vec<String>,
    /// Receipt read or parse error, when present.
    pub parse_error: Option<String>,
}

/// Unsupported scanned utility finding from the style check receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxStyleUnsupportedScannedClassFinding {
    /// Class token scanned from source.
    pub class_name: String,
    /// Why dx-style could not generate CSS for the token.
    pub reason: String,
}

/// Summary of unsupported scanned utility classes in `.dx/receipts/style/check.json`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DxStyleUnsupportedScannedClassSummary {
    /// Whether the receipt file exists.
    pub receipt_file_present: bool,
    /// Unsupported scanned Tailwind-like utility class count.
    pub unsupported_class_count: u64,
    /// Representative unsupported class findings.
    pub findings: Vec<DxStyleUnsupportedScannedClassFinding>,
    /// Receipt read or parse error, when present.
    pub parse_error: Option<String>,
}

/// Unsupported CSS-first directive finding from the style check receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxStyleUnsupportedCssDirectiveFinding {
    /// Directive name such as `@plugin` or `@config`.
    pub directive: String,
    /// Why the directive is not supported by dx-style.
    pub reason: String,
    /// Source file reported by the style receipt.
    pub file: String,
    /// One-based source line when available.
    pub line: Option<u64>,
}

/// Summary of unsupported CSS-first directives in `.dx/receipts/style/check.json`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DxStyleUnsupportedCssDirectiveSummary {
    /// Whether the receipt file exists.
    pub receipt_file_present: bool,
    /// Unsupported directive count.
    pub unsupported_directive_count: u64,
    /// Representative unsupported directive findings.
    pub findings: Vec<DxStyleUnsupportedCssDirectiveFinding>,
    /// Receipt read or parse error, when present.
    pub parse_error: Option<String>,
}

/// Package-owned unsupported class finding from `style_package_ownership_rows`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxStylePackageUnsupportedClassFinding {
    /// Forge package that introduced the class.
    pub package_id: String,
    /// Class token owned by the package.
    pub class_name: String,
    /// Why dx-style could not prove/generated CSS for the token.
    pub reason: String,
}

/// Package-owned style surface row from the style check receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxStylePackageOwnershipRow {
    /// Forge package id.
    pub package_id: String,
    /// Human-readable package name.
    pub package_name: String,
    /// Package style scope.
    pub style_scope: String,
    /// Source files that introduced the style surface.
    pub source_files: Vec<String>,
    /// Theme tokens required by this package.
    pub required_tokens: Vec<String>,
    /// Generated classes owned by this package.
    pub generated_classes: Vec<String>,
    /// Unsupported package-owned classes.
    pub unsupported_classes: Vec<DxStylePackageUnsupportedClassFinding>,
    /// Package receipt path, when known.
    pub receipt_path: String,
}

/// Summary of package-owned style evidence in `.dx/receipts/style/check.json`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DxStylePackageOwnershipSummary {
    /// Whether the receipt file exists.
    pub receipt_file_present: bool,
    /// Package-owned style row count.
    pub package_count: u64,
    /// Package-owned generated class count.
    pub generated_class_count: u64,
    /// Package-owned unsupported class count.
    pub unsupported_class_count: u64,
    /// Package ids represented by the receipt.
    pub package_ids: Vec<String>,
    /// Package-owned unsupported class findings.
    pub unsupported_classes: Vec<DxStylePackageUnsupportedClassFinding>,
    /// Receipt read or parse error, when present.
    pub parse_error: Option<String>,
}

/// Class-to-generated-rule metadata row from the style check receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxStyleRuleMetadataRow {
    /// Class token that generated the CSS rule.
    pub class_name: String,
    /// Generated CSS selector.
    pub selector: String,
    /// Visual property buckets represented by the declarations.
    pub visual_properties: Vec<String>,
    /// Source files where the class was scanned.
    pub source_files: Vec<String>,
    /// Origin classification for the generated rule.
    pub source_origin: String,
    /// Theme token references in the generated CSS.
    pub token_references: Vec<String>,
    /// Number of declaration objects captured for the rule.
    pub declaration_count: u64,
    /// Whether Zed/DX Studio can expose this class as editable evidence.
    pub zed_studio_editable: bool,
}

/// Summary of class-to-generated-rule metadata in `.dx/receipts/style/check.json`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DxStyleRuleMetadataSummary {
    /// Whether the receipt file exists.
    pub receipt_file_present: bool,
    /// Supported metadata row count.
    pub metadata_count: u64,
    /// Metadata rows that can be exposed to Zed/DX Studio.
    pub editable_class_count: u64,
    /// Distinct visual property bucket count.
    pub visual_property_count: u64,
    /// Distinct token reference count.
    pub token_reference_count: u64,
    /// Distinct source file count.
    pub source_file_count: u64,
    /// Distinct visual property buckets.
    pub visual_properties: Vec<String>,
    /// Distinct source files.
    pub source_files: Vec<String>,
    /// Distinct theme token references.
    pub token_references: Vec<String>,
    /// Supported metadata rows.
    pub rows: Vec<DxStyleRuleMetadataRow>,
    /// Receipt read or parse error, when present.
    pub parse_error: Option<String>,
}

fn read_dx_style_check_receipt(root: &Path) -> Result<Option<Value>, String> {
    let receipt_path = root.join(DX_STYLE_CHECK_RECEIPT_PATH);
    if !receipt_path.exists() {
        return Ok(None);
    }

    if let Some(receipt) = read_json_receipt_machine_alias(
        root,
        DX_STYLE_CHECK_RECEIPT_PATH,
        DX_STYLE_CHECK_MACHINE_PATH,
    ) {
        return Ok(Some(receipt));
    }

    let content = fs::read_to_string(&receipt_path).map_err(|error| error.to_string())?;
    serde_json::from_str(&content)
        .map(Some)
        .map_err(|error| error.to_string())
}

/// Read the browser-compatibility canary from the latest dx-style check receipt.
pub fn dx_style_browser_compat_summary(root: &Path) -> DxStyleBrowserCompatSummary {
    let mut summary = DxStyleBrowserCompatSummary {
        receipt_file_present: true,
        ..DxStyleBrowserCompatSummary::default()
    };
    let receipt = match read_dx_style_check_receipt(root) {
        Ok(Some(receipt)) => receipt,
        Ok(None) => return DxStyleBrowserCompatSummary::default(),
        Err(error) => {
            summary.parse_error = Some(error);
            return summary;
        }
    };

    let Some(contract) = receipt.get("browser_compat_receipt_contract") else {
        return summary;
    };

    summary.contract_present = true;
    summary.schema_supported =
        contract.get("schema").and_then(Value::as_str) == Some(DX_STYLE_BROWSER_COMPAT_SCHEMA);
    summary.class_count = contract
        .get("class_count")
        .and_then(Value::as_u64)
        .or_else(|| {
            contract
                .get("classes")
                .and_then(Value::as_array)
                .map(|classes| classes.len() as u64)
        })
        .unwrap_or(0);
    summary.selector_class_count = contract
        .get("selector_class_count")
        .and_then(Value::as_u64)
        .or_else(|| {
            contract
                .get("selector_classes")
                .and_then(Value::as_array)
                .map(|classes| classes.len() as u64)
        })
        .unwrap_or(0);
    summary.full_autoprefixer_parity = contract
        .get("full_autoprefixer_parity")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    summary.full_tailwind_postcss_output_parity = contract
        .get("full_tailwind_postcss_output_parity")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    summary
}

/// Read the Tailwind equal-output canary from the latest dx-style check receipt.
pub fn dx_style_tailwind_equal_output_summary(root: &Path) -> DxStyleTailwindEqualOutputSummary {
    let mut summary = DxStyleTailwindEqualOutputSummary {
        receipt_file_present: true,
        ..DxStyleTailwindEqualOutputSummary::default()
    };
    let receipt = match read_dx_style_check_receipt(root) {
        Ok(Some(receipt)) => receipt,
        Ok(None) => return DxStyleTailwindEqualOutputSummary::default(),
        Err(error) => {
            summary.parse_error = Some(error);
            return summary;
        }
    };

    let Some(contract) = receipt.get("tailwind_equal_output_canary_contract") else {
        return summary;
    };

    summary.contract_present = true;
    summary.schema_supported = contract.get("schema").and_then(Value::as_str)
        == Some(DX_STYLE_TAILWIND_EQUAL_OUTPUT_SCHEMA);
    summary.class_count = contract
        .get("class_count")
        .and_then(Value::as_u64)
        .or_else(|| {
            contract
                .get("classes")
                .and_then(Value::as_array)
                .map(|classes| classes.len() as u64)
        })
        .unwrap_or(0);
    summary.equal_output_class_count = contract
        .get("equal_output_class_count")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    summary.unsupported_class_count = contract
        .get("unsupported_class_count")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    summary.live_tailwind_execution = contract
        .get("live_tailwind_execution")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    summary.full_tailwind_parity = contract
        .get("full_tailwind_parity")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    summary.fair_speed_benchmark = contract
        .get("fair_speed_benchmark")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    summary
}

/// Read the PostCSS compatibility contract from the latest dx-style check receipt.
pub fn dx_style_postcss_compat_summary(root: &Path) -> DxStylePostcssCompatSummary {
    let mut summary = DxStylePostcssCompatSummary {
        receipt_file_present: true,
        ..DxStylePostcssCompatSummary::default()
    };
    let receipt = match read_dx_style_check_receipt(root) {
        Ok(Some(receipt)) => receipt,
        Ok(None) => return DxStylePostcssCompatSummary::default(),
        Err(error) => {
            summary.parse_error = Some(error);
            return summary;
        }
    };

    let Some(contract) = receipt.get("postcss_compatibility_contract") else {
        return summary;
    };

    summary.contract_present = true;
    summary.schema_supported =
        contract.get("schema").and_then(Value::as_str) == Some(DX_STYLE_POSTCSS_COMPAT_SCHEMA);
    summary.supported_count = contract
        .get("supported_count")
        .and_then(Value::as_u64)
        .or_else(|| {
            receipt
                .get("postcss_compat_supported_count")
                .and_then(Value::as_u64)
        })
        .unwrap_or(0);
    summary.partial_count = contract
        .get("partial_count")
        .and_then(Value::as_u64)
        .or_else(|| {
            receipt
                .get("postcss_compat_partial_count")
                .and_then(Value::as_u64)
        })
        .unwrap_or(0);
    summary.unsupported_count = contract
        .get("unsupported_count")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    summary.dx_starter_replacement_score = contract
        .get("dx_starter_replacement_score")
        .and_then(Value::as_u64)
        .or_else(|| {
            receipt
                .get("dx_starter_replacement_score")
                .and_then(Value::as_u64)
        })
        .unwrap_or(0);
    summary.full_postcss_plugin_parity = contract
        .get("full_postcss_plugin_parity")
        .and_then(Value::as_bool)
        .or_else(|| {
            receipt
                .get("full_postcss_plugin_parity")
                .and_then(Value::as_bool)
        })
        .unwrap_or(false);
    summary.postcss_plugin_parity_status = contract
        .get("postcss_plugin_parity_status")
        .and_then(Value::as_str)
        .or_else(|| {
            receipt
                .get("postcss_plugin_parity_status")
                .and_then(Value::as_str)
        })
        .unwrap_or("unknown")
        .to_string();
    summary.autoprefixer_parity_status = contract
        .get("autoprefixer_parity_status")
        .and_then(Value::as_str)
        .or_else(|| {
            receipt
                .get("autoprefixer_parity_status")
                .and_then(Value::as_str)
        })
        .unwrap_or("unknown")
        .to_string();
    summary.postcss_runtime_dependency_required = contract
        .get("postcss_runtime_dependency_required")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    summary.local_postcss_config_required = contract
        .get("local_postcss_config_required")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    summary.unsupported_transform_warnings = string_array(
        contract,
        &[
            "unsupported_transform_warnings",
            "unsupportedTransformWarnings",
        ],
    )
    .into_iter()
    .chain(string_array(
        &receipt,
        &[
            "unsupported_transform_warnings",
            "unsupportedTransformWarnings",
        ],
    ))
    .collect::<BTreeSet<_>>()
    .into_iter()
    .collect();
    summary
}

/// Read unsupported scanned utility class findings from the latest dx-style check receipt.
pub fn dx_style_unsupported_scanned_classes_summary(
    root: &Path,
) -> DxStyleUnsupportedScannedClassSummary {
    let mut summary = DxStyleUnsupportedScannedClassSummary {
        receipt_file_present: true,
        ..DxStyleUnsupportedScannedClassSummary::default()
    };
    let receipt = match read_dx_style_check_receipt(root) {
        Ok(Some(receipt)) => receipt,
        Ok(None) => return DxStyleUnsupportedScannedClassSummary::default(),
        Err(error) => {
            summary.parse_error = Some(error);
            return summary;
        }
    };

    summary.findings = receipt
        .get("unsupported_scanned_class_findings")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(unsupported_scanned_class_finding)
                .collect()
        })
        .unwrap_or_default();
    summary.unsupported_class_count = receipt
        .get("unsupported_scanned_class_count")
        .and_then(Value::as_u64)
        .unwrap_or(summary.findings.len() as u64);
    summary
}

/// Read unsupported CSS-first directive findings from the latest dx-style check receipt.
pub fn dx_style_unsupported_css_directives_summary(
    root: &Path,
) -> DxStyleUnsupportedCssDirectiveSummary {
    let mut summary = DxStyleUnsupportedCssDirectiveSummary {
        receipt_file_present: true,
        ..DxStyleUnsupportedCssDirectiveSummary::default()
    };
    let receipt = match read_dx_style_check_receipt(root) {
        Ok(Some(receipt)) => receipt,
        Ok(None) => return DxStyleUnsupportedCssDirectiveSummary::default(),
        Err(error) => {
            summary.parse_error = Some(error);
            return summary;
        }
    };

    summary.findings = receipt
        .get("unsupported_css_directive_findings")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(unsupported_css_directive_finding)
                .collect()
        })
        .unwrap_or_default();
    summary.unsupported_directive_count = receipt
        .get("unsupported_css_directive_count")
        .and_then(Value::as_u64)
        .unwrap_or(summary.findings.len() as u64);
    summary
}

/// Read package-owned style evidence from the latest dx-style check receipt.
pub fn dx_style_package_ownership_summary(root: &Path) -> DxStylePackageOwnershipSummary {
    let mut summary = DxStylePackageOwnershipSummary {
        receipt_file_present: true,
        ..DxStylePackageOwnershipSummary::default()
    };
    let receipt = match read_dx_style_check_receipt(root) {
        Ok(Some(receipt)) => receipt,
        Ok(None) => return DxStylePackageOwnershipSummary::default(),
        Err(error) => {
            summary.parse_error = Some(error);
            return summary;
        }
    };

    let rows = package_ownership_rows(&receipt);
    let mut package_ids = BTreeSet::new();
    let mut unsupported_classes = Vec::new();
    let mut generated_class_count = 0u64;

    for row in rows {
        package_ids.insert(row.package_id);
        generated_class_count += row.generated_classes.len() as u64;
        unsupported_classes.extend(row.unsupported_classes);
    }

    summary.package_count = package_ids.len() as u64;
    summary.generated_class_count = generated_class_count;
    summary.unsupported_class_count = unsupported_classes.len() as u64;
    summary.package_ids = package_ids.into_iter().collect();
    summary.unsupported_classes = unsupported_classes;
    summary
}

/// Read class-to-generated-rule metadata from the latest dx-style check receipt.
pub fn dx_style_rule_metadata_summary(root: &Path) -> DxStyleRuleMetadataSummary {
    let mut summary = DxStyleRuleMetadataSummary {
        receipt_file_present: true,
        ..DxStyleRuleMetadataSummary::default()
    };
    let receipt = match read_dx_style_check_receipt(root) {
        Ok(Some(receipt)) => receipt,
        Ok(None) => return DxStyleRuleMetadataSummary::default(),
        Err(error) => {
            summary.parse_error = Some(error);
            return summary;
        }
    };

    let rows = style_rule_metadata_rows(&receipt);
    let mut visual_properties = BTreeSet::new();
    let mut source_files = BTreeSet::new();
    let mut token_references = BTreeSet::new();
    let mut editable_class_count = 0u64;

    for row in &rows {
        if row.zed_studio_editable {
            editable_class_count += 1;
        }
        visual_properties.extend(row.visual_properties.iter().cloned());
        source_files.extend(row.source_files.iter().cloned());
        token_references.extend(row.token_references.iter().cloned());
    }

    summary.metadata_count = rows.len() as u64;
    summary.editable_class_count = editable_class_count;
    summary.visual_properties = visual_properties.into_iter().collect();
    summary.source_files = source_files.into_iter().collect();
    summary.token_references = token_references.into_iter().collect();
    summary.visual_property_count = summary.visual_properties.len() as u64;
    summary.source_file_count = summary.source_files.len() as u64;
    summary.token_reference_count = summary.token_references.len() as u64;
    summary.rows = rows;
    summary
}

fn unsupported_scanned_class_finding(
    value: &Value,
) -> Option<DxStyleUnsupportedScannedClassFinding> {
    if value.get("rule").and_then(Value::as_str) != Some(DX_STYLE_UNSUPPORTED_SCANNED_CLASS_RULE) {
        return None;
    }

    Some(DxStyleUnsupportedScannedClassFinding {
        class_name: value.get("class_name")?.as_str()?.to_string(),
        reason: value
            .get("reason")
            .and_then(Value::as_str)
            .unwrap_or("dx-style did not generate CSS for this scanned utility class")
            .to_string(),
    })
}

fn unsupported_css_directive_finding(
    value: &Value,
) -> Option<DxStyleUnsupportedCssDirectiveFinding> {
    if value.get("rule").and_then(Value::as_str) != Some(DX_STYLE_UNSUPPORTED_CSS_DIRECTIVE_RULE) {
        return None;
    }

    Some(DxStyleUnsupportedCssDirectiveFinding {
        directive: value.get("directive")?.as_str()?.to_string(),
        reason: value
            .get("reason")
            .and_then(Value::as_str)
            .unwrap_or("dx-style does not support this CSS-first directive")
            .to_string(),
        file: value
            .get("file")
            .and_then(Value::as_str)
            .unwrap_or(".dx/receipts/style/check.json")
            .to_string(),
        line: value.get("line").and_then(Value::as_u64),
    })
}

fn style_rule_metadata_rows(receipt: &Value) -> Vec<DxStyleRuleMetadataRow> {
    array_value(receipt, &["style_rule_metadata", "styleRuleMetadata"])
        .into_iter()
        .filter_map(style_rule_metadata_row)
        .collect()
}

fn style_rule_metadata_row(value: &Value) -> Option<DxStyleRuleMetadataRow> {
    if string_value(value, &["schema"]).as_deref() != Some(DX_STYLE_RULE_METADATA_SCHEMA) {
        return None;
    }

    Some(DxStyleRuleMetadataRow {
        class_name: string_value(value, &["class_name", "className"])?,
        selector: string_value(value, &["selector"]).unwrap_or_default(),
        visual_properties: string_array(value, &["visual_properties", "visualProperties"]),
        source_files: string_array(value, &["source_files", "sourceFiles"]),
        source_origin: string_value(value, &["source_origin", "sourceOrigin"])
            .unwrap_or_else(|| "dx-style".to_string()),
        token_references: string_array(value, &["token_references", "tokenReferences"]),
        declaration_count: array_value(value, &["declarations"]).len() as u64,
        zed_studio_editable: bool_value(value, &["zed_studio_editable", "zedStudioEditable"])
            .unwrap_or(false),
    })
}

fn package_ownership_rows(receipt: &Value) -> Vec<DxStylePackageOwnershipRow> {
    array_value(
        receipt,
        &["style_package_ownership_rows", "stylePackageOwnershipRows"],
    )
    .into_iter()
    .filter_map(package_ownership_row)
    .collect()
}

fn package_ownership_row(value: &Value) -> Option<DxStylePackageOwnershipRow> {
    let package_id = string_value(value, &["package_id", "packageId"])?;
    let unsupported_classes = array_value(value, &["unsupported_classes", "unsupportedClasses"])
        .into_iter()
        .filter_map(|item| package_unsupported_class(&package_id, item))
        .collect();

    Some(DxStylePackageOwnershipRow {
        package_name: string_value(value, &["package_name", "packageName"])
            .unwrap_or_else(|| package_id.clone()),
        style_scope: string_value(value, &["style_scope", "styleScope"])
            .unwrap_or_else(|| package_id.clone()),
        source_files: string_array(value, &["source_files", "sourceFiles"]),
        required_tokens: string_array(value, &["required_tokens", "requiredTokens"]),
        generated_classes: string_array(value, &["generated_classes", "generatedClasses"]),
        unsupported_classes,
        receipt_path: string_value(value, &["receipt_path", "receiptPath"]).unwrap_or_default(),
        package_id,
    })
}

fn package_unsupported_class(
    package_id: &str,
    value: &Value,
) -> Option<DxStylePackageUnsupportedClassFinding> {
    if let Some(class_name) = value.as_str().map(str::to_string) {
        return Some(DxStylePackageUnsupportedClassFinding {
            package_id: package_id.to_string(),
            class_name,
            reason: "dx-style did not generate CSS for this package-owned class".to_string(),
        });
    }

    Some(DxStylePackageUnsupportedClassFinding {
        package_id: package_id.to_string(),
        class_name: string_value(value, &["class_name", "className"])?,
        reason: string_value(value, &["reason"]).unwrap_or_else(|| {
            "dx-style did not generate CSS for this package-owned class".to_string()
        }),
    })
}

fn array_value<'a>(value: &'a Value, keys: &[&str]) -> Vec<&'a Value> {
    keys.iter()
        .find_map(|key| value.get(*key).and_then(Value::as_array))
        .map(|items| items.iter().collect())
        .unwrap_or_default()
}

fn string_value(value: &Value, keys: &[&str]) -> Option<String> {
    keys.iter()
        .find_map(|key| value.get(*key).and_then(Value::as_str))
        .map(str::to_string)
}

fn string_array(value: &Value, keys: &[&str]) -> Vec<String> {
    array_value(value, keys)
        .into_iter()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}

fn bool_value(value: &Value, keys: &[&str]) -> Option<bool> {
    keys.iter()
        .find_map(|key| value.get(*key).and_then(Value::as_bool))
}
