import {
  DX_STYLE_BROWSER_COMPAT_FIXTURE_PATH,
  DX_STYLE_BROWSER_COMPAT_ROW_ID,
  DX_STYLE_BROWSER_COMPAT_ZED_VISIBILITY,
  DX_STYLE_CHECK_RECEIPT_PATH,
} from "./dx-check-style-evidence-read-model.ts";

export type DxStylePreviewManifestEvidenceRow = {
  readonly schema?: string;
  readonly rowId: string;
  readonly title?: string;
  readonly status: string;
  readonly receiptPath?: string;
  readonly fixturePath?: string;
  readonly zedVisibility?: string;
  readonly canaryClassCount?: number;
  readonly selectorCanaryClassCount?: number;
  readonly selectorClassExamples?: readonly string[];
  readonly tailwindParityStateAliasSupportedClassCount?: number;
  readonly tailwindParitySupportedStateAliasExamples?: readonly string[];
  readonly fullAutoprefixerParity?: boolean;
  readonly fullTailwindPostcssOutputParity?: boolean;
  readonly runtimeProof?: boolean;
  readonly nextAction?: string;
};

export type DxStylePreviewManifestRoute = {
  readonly route: string;
  readonly styleEvidenceRows?: readonly DxStylePreviewManifestEvidenceRow[];
};

export type DxStylePreviewManifest = {
  readonly styleEvidenceRows?: readonly DxStylePreviewManifestEvidenceRow[];
  readonly routes?: readonly DxStylePreviewManifestRoute[];
};

export type DxStylePreviewManifestEvidence = {
  readonly schema: "dx.www.template.preview_style_evidence_read_model";
  readonly source: string;
  readonly route: string;
  readonly rowId: string;
  readonly title: string;
  readonly status: string;
  readonly receiptPath: string;
  readonly fixturePath: string;
  readonly zedVisibility: string;
  readonly canaryClassCount: number;
  readonly selectorCanaryClassCount: number;
  readonly selectorClassExamples: readonly string[];
  readonly tailwindParityStateAliasSupportedClassCount: number;
  readonly tailwindParitySupportedStateAliasExamples: readonly string[];
  readonly fullAutoprefixerParity: boolean;
  readonly fullTailwindPostcssOutputParity: boolean;
  readonly runtimeProof: boolean;
  readonly readsHtml: false;
  readonly readsRawStyleReceipt: false;
  readonly nextAction: string;
};

const ROOT_STYLE_EVIDENCE_SOURCE = "preview_manifest.styleEvidenceRows";
const MISSING_PREVIEW_EVIDENCE_NEXT_ACTION =
  "Run the launch materializer so public/preview-.dx/build-cache/manifest.json carries styleEvidenceRows.";

export function dxStyleBrowserCompatEvidenceFromPreviewManifest(
  manifest: DxStylePreviewManifest,
  route = "/",
): DxStylePreviewManifestEvidence {
  const routeRow = manifest.routes
    ?.find((candidate) => candidate.route === route)
    ?.styleEvidenceRows?.find(isBrowserCompatRow);
  const rootRow = manifest.styleEvidenceRows?.find(isBrowserCompatRow);
  const row = routeRow ?? rootRow;
  const source = routeRow
    ? `preview_manifest.routes[${route}].styleEvidenceRows`
    : ROOT_STYLE_EVIDENCE_SOURCE;

  if (!row) {
    return {
      schema: "dx.www.template.preview_style_evidence_read_model",
      source,
      route,
      rowId: DX_STYLE_BROWSER_COMPAT_ROW_ID,
      title: "dx-style browser compatibility",
      status: "missing",
      receiptPath: DX_STYLE_CHECK_RECEIPT_PATH,
      fixturePath: DX_STYLE_BROWSER_COMPAT_FIXTURE_PATH,
      zedVisibility: DX_STYLE_BROWSER_COMPAT_ZED_VISIBILITY,
      canaryClassCount: 0,
      selectorCanaryClassCount: 0,
      selectorClassExamples: [],
      tailwindParityStateAliasSupportedClassCount: 0,
      tailwindParitySupportedStateAliasExamples: [],
      fullAutoprefixerParity: false,
      fullTailwindPostcssOutputParity: false,
      runtimeProof: false,
      readsHtml: false,
      readsRawStyleReceipt: false,
      nextAction: MISSING_PREVIEW_EVIDENCE_NEXT_ACTION,
    };
  }

  return {
    schema: "dx.www.template.preview_style_evidence_read_model",
    source,
    route,
    rowId: row.rowId,
    title: row.title ?? "dx-style browser compatibility",
    status: row.status,
    receiptPath: row.receiptPath ?? DX_STYLE_CHECK_RECEIPT_PATH,
    fixturePath: row.fixturePath ?? DX_STYLE_BROWSER_COMPAT_FIXTURE_PATH,
    zedVisibility: row.zedVisibility ?? DX_STYLE_BROWSER_COMPAT_ZED_VISIBILITY,
    canaryClassCount: row.canaryClassCount ?? 0,
    selectorCanaryClassCount: row.selectorCanaryClassCount ?? 0,
    selectorClassExamples: row.selectorClassExamples ?? [],
    tailwindParityStateAliasSupportedClassCount:
      row.tailwindParityStateAliasSupportedClassCount ?? 0,
    tailwindParitySupportedStateAliasExamples:
      row.tailwindParitySupportedStateAliasExamples ?? [],
    fullAutoprefixerParity: row.fullAutoprefixerParity ?? false,
    fullTailwindPostcssOutputParity:
      row.fullTailwindPostcssOutputParity ?? false,
    runtimeProof: row.runtimeProof ?? false,
    readsHtml: false,
    readsRawStyleReceipt: false,
    nextAction: row.nextAction ?? MISSING_PREVIEW_EVIDENCE_NEXT_ACTION,
  };
}

function isBrowserCompatRow(row: DxStylePreviewManifestEvidenceRow): boolean {
  return row.rowId === DX_STYLE_BROWSER_COMPAT_ROW_ID;
}
