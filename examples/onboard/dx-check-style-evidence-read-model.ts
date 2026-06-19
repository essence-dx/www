export const DX_STYLE_BROWSER_COMPAT_ROW_ID = "dx-style-browser-compat";
export const DX_STYLE_BROWSER_COMPAT_ZED_VISIBILITY = "dx-style:browser-compat";
export const DX_STYLE_BROWSER_COMPAT_FIXTURE_PATH =
  "related-crates/style/fixtures/tailwind-postcss-browser-compat.json";
export const DX_STYLE_CHECK_RECEIPT_PATH = ".dx/receipts/style/check.json";

export type DxCheckPanelStyleEvidenceMetric = {
  readonly name: string;
  readonly value: number;
};

export type DxCheckPanelStyleEvidenceRow = {
  readonly row_id: string;
  readonly title: string;
  readonly status: string;
  readonly receipt_path: string;
  readonly fixture_path: string;
  readonly receipt_present: boolean;
  readonly contract_present: boolean;
  readonly schema_supported: boolean;
  readonly class_count: number;
  readonly selector_class_count?: number;
  readonly selector_class_examples?: readonly string[];
  readonly tailwind_parity_state_alias_supported_class_count?: number;
  readonly tailwind_parity_supported_state_alias_examples?: readonly string[];
  readonly package_ownership_package_count?: number;
  readonly package_ownership_generated_class_count?: number;
  readonly package_ownership_unsupported_class_count?: number;
  readonly package_ownership_package_ids?: readonly string[];
  readonly package_ownership_unsupported_class_examples?: readonly string[];
  readonly full_autoprefixer_parity: boolean;
  readonly full_tailwind_postcss_output_parity: boolean;
  readonly metrics: readonly DxCheckPanelStyleEvidenceMetric[];
  readonly zed_visibility: string;
  readonly runtime_limitations: readonly string[];
  readonly next_action: string;
};

export type DxCheckPanelStyleEvidenceViewModel = {
  readonly style_evidence_rows?: readonly DxCheckPanelStyleEvidenceRow[];
};

export type DxStyleBrowserCompatMetricSummary = {
  readonly dxStyleBrowserCompatReceiptPresent: number;
  readonly dxStyleBrowserCompatContractPresent: number;
  readonly dxStyleBrowserCompatSchemaSupported: number;
  readonly dxStyleBrowserCompatClassCount: number;
  readonly dxStyleBrowserCompatSelectorClassCount: number;
  readonly dxStyleBrowserCompatFullAutoprefixerParity: number;
  readonly dxStyleBrowserCompatFullTailwindPostcssOutputParity: number;
  readonly dxStyleTailwindParityStateAliasSupportedClasses: number;
};

export type DxStyleBrowserCompatEvidence = {
  readonly schema: "dx.www.template.dx_check_style_evidence_read_model";
  readonly source: "check_panel.view_model.style_evidence_rows";
  readonly rowId: string;
  readonly title: string;
  readonly status: string;
  readonly receiptPath: string;
  readonly fixturePath: string;
  readonly receiptPresent: boolean;
  readonly contractPresent: boolean;
  readonly schemaSupported: boolean;
  readonly canaryClassCount: number;
  readonly selectorCanaryClassCount: number;
  readonly selectorClassExamples: readonly string[];
  readonly tailwindParityStateAliasSupportedClassCount: number;
  readonly tailwindParitySupportedStateAliasExamples: readonly string[];
  readonly fullAutoprefixerParity: boolean;
  readonly fullTailwindPostcssOutputParity: boolean;
  readonly metrics: DxStyleBrowserCompatMetricSummary;
  readonly zedVisibility: string;
  readonly readsRawStyleReceipt: false;
  readonly runtimeLimitations: readonly string[];
  readonly nextAction: string;
};

const MISSING_LIMITATIONS = [
  "SOURCE-ONLY: this read model consumes check_panel.view_model.style_evidence_rows only.",
  "NO-RAW-STYLE-READ: Zed and Studio should not parse .dx/receipts/style/check.json directly for this panel.",
] as const;

export function dxStyleBrowserCompatEvidenceFromCheckPanel(
  viewModel: DxCheckPanelStyleEvidenceViewModel,
): DxStyleBrowserCompatEvidence {
  const row = viewModel.style_evidence_rows?.find(
    (candidate) => candidate.row_id === DX_STYLE_BROWSER_COMPAT_ROW_ID,
  );

  if (!row) {
    return {
      schema: "dx.www.template.dx_check_style_evidence_read_model",
      source: "check_panel.view_model.style_evidence_rows",
      rowId: DX_STYLE_BROWSER_COMPAT_ROW_ID,
      title: "dx-style browser compatibility",
      status: "missing",
      receiptPath: DX_STYLE_CHECK_RECEIPT_PATH,
      fixturePath: DX_STYLE_BROWSER_COMPAT_FIXTURE_PATH,
      receiptPresent: false,
      contractPresent: false,
      schemaSupported: false,
      canaryClassCount: 0,
      selectorCanaryClassCount: 0,
      selectorClassExamples: [],
      tailwindParityStateAliasSupportedClassCount: 0,
      tailwindParitySupportedStateAliasExamples: [],
      fullAutoprefixerParity: false,
      fullTailwindPostcssOutputParity: false,
      metrics: metricSummary([]),
      zedVisibility: DX_STYLE_BROWSER_COMPAT_ZED_VISIBILITY,
      readsRawStyleReceipt: false,
      runtimeLimitations: MISSING_LIMITATIONS,
      nextAction:
        "Run `dx check --json` after `dx style check` so the check panel exposes style_evidence_rows.",
    };
  }

  const metrics = metricSummary(row.metrics);
  const stateAliasCount =
    row.tailwind_parity_state_alias_supported_class_count ??
    metrics.dxStyleTailwindParityStateAliasSupportedClasses;

  return {
    schema: "dx.www.template.dx_check_style_evidence_read_model",
    source: "check_panel.view_model.style_evidence_rows",
    rowId: row.row_id,
    title: row.title,
    status: row.status,
    receiptPath: row.receipt_path,
    fixturePath: row.fixture_path,
    receiptPresent: row.receipt_present,
    contractPresent: row.contract_present,
    schemaSupported: row.schema_supported,
    canaryClassCount: row.class_count,
    selectorCanaryClassCount:
      row.selector_class_count ??
      metrics.dxStyleBrowserCompatSelectorClassCount,
    selectorClassExamples: row.selector_class_examples ?? [],
    tailwindParityStateAliasSupportedClassCount: stateAliasCount,
    tailwindParitySupportedStateAliasExamples:
      row.tailwind_parity_supported_state_alias_examples ?? [],
    fullAutoprefixerParity: row.full_autoprefixer_parity,
    fullTailwindPostcssOutputParity:
      row.full_tailwind_postcss_output_parity,
    metrics,
    zedVisibility: row.zed_visibility,
    readsRawStyleReceipt: false,
    runtimeLimitations: row.runtime_limitations,
    nextAction: row.next_action,
  };
}

function metricSummary(
  metrics: readonly DxCheckPanelStyleEvidenceMetric[],
): DxStyleBrowserCompatMetricSummary {
  return {
    dxStyleBrowserCompatReceiptPresent: metricValue(
      metrics,
      "dx_style_browser_compat_receipt_present",
    ),
    dxStyleBrowserCompatContractPresent: metricValue(
      metrics,
      "dx_style_browser_compat_contract_present",
    ),
    dxStyleBrowserCompatSchemaSupported: metricValue(
      metrics,
      "dx_style_browser_compat_schema_supported",
    ),
    dxStyleBrowserCompatClassCount: metricValue(
      metrics,
      "dx_style_browser_compat_class_count",
    ),
    dxStyleBrowserCompatSelectorClassCount: metricValue(
      metrics,
      "dx_style_browser_compat_selector_class_count",
    ),
    dxStyleBrowserCompatFullAutoprefixerParity: metricValue(
      metrics,
      "dx_style_browser_compat_full_autoprefixer_parity",
    ),
    dxStyleBrowserCompatFullTailwindPostcssOutputParity: metricValue(
      metrics,
      "dx_style_browser_compat_full_tailwind_postcss_output_parity",
    ),
    dxStyleTailwindParityStateAliasSupportedClasses: metricValue(
      metrics,
      "dx_style_tailwind_parity_state_alias_supported_classes",
    ),
  };
}

function metricValue(
  metrics: readonly DxCheckPanelStyleEvidenceMetric[],
  name: string,
): number {
  return metrics.find((metric) => metric.name === name)?.value ?? 0;
}
