import {
  type DxStylePreviewManifest,
  dxStyleBrowserCompatEvidenceFromPreviewManifest,
} from "./preview-style-evidence-read-model.ts";

export type DxStylePreviewPackagePanel = {
  readonly schema: "dx.www.template.preview_style_package_panel_read_model";
  readonly panelId: "dx-style-browser-compat-package-panel";
  readonly packageId: "dx-style";
  readonly title: string;
  readonly status: string;
  readonly source: string;
  readonly route: string;
  readonly zedVisibility: string;
  readonly receiptPath: string;
  readonly fixturePath: string;
  readonly canaryClassCount: number;
  readonly selectorCanaryClassCount: number;
  readonly selectorClassExamples: readonly string[];
  readonly stateAliasSupportedClassCount: number;
  readonly stateAliasSupportedExamples: readonly string[];
  readonly fullAutoprefixerParity: boolean;
  readonly fullTailwindPostcssOutputParity: boolean;
  readonly readsHtml: false;
  readonly readsRawStyleReceipt: false;
  readonly readsCheckReceipt: false;
  readonly nextAction: string;
};

export type DxStylePreviewPackagePanelWithDrift = Omit<
  DxStylePreviewPackagePanel,
  "schema" | "nextAction"
> & {
  readonly schema: "dx.www.template.preview_style_package_panel_with_drift_read_model";
  readonly driftExerciseState: DxStyleDriftFixtureReadModel["exerciseState"];
  readonly driftExercised: boolean;
  readonly driftStatus: string;
  readonly driftLoaderFile: string;
  readonly driftMarkerHelperFile: string;
  readonly driftStates: readonly string[];
  readonly driftMismatchFields: readonly string[];
  readonly readsReadinessReceipt: boolean;
  readonly nextAction: string;
};

export type DxStylePackagePanelMarkerState = {
  readonly schema: "dx.www.template.preview_style_package_panel_marker_state";
  readonly panelId: DxStylePreviewPackagePanelWithDrift["panelId"];
  readonly readModel: DxStylePreviewPackagePanelWithDrift["schema"];
  readonly driftState: DxStylePreviewPackagePanelWithDrift["driftExerciseState"];
  readonly driftStatus: string;
  readonly driftMismatchFields: readonly string[];
  readonly readinessReceipt: "read" | "missing";
};

export type DxStyleDriftFixtureInput = {
  readonly rowId?: string;
  readonly row_id?: string;
  readonly route?: string;
  readonly status?: string;
  readonly loaderFile?: string;
  readonly loader_file?: string;
  readonly markerHelperFile?: string;
  readonly marker_helper_file?: string;
  readonly fixturePath?: string;
  readonly fixture_path?: string;
  readonly states?: readonly string[];
  readonly fullAutoprefixerParity?: boolean;
  readonly full_autoprefixer_parity?: boolean;
  readonly fullTailwindPostcssOutputParity?: boolean;
  readonly full_tailwind_postcss_output_parity?: boolean;
};

export type DxStyleDriftFixtureManifestRoute = {
  readonly route: string;
  readonly styleEvidenceDriftFixtures?: readonly DxStyleDriftFixtureInput[];
};

export type DxStyleDriftFixtureManifest = DxStylePreviewManifest & {
  readonly styleEvidenceDriftFixtures?: readonly DxStyleDriftFixtureInput[];
  readonly routes?: readonly DxStyleDriftFixtureManifestRoute[];
};

export type DxStyleDriftFixtureReadinessReceipt = {
  readonly styleEvidenceDriftFixture?: DxStyleDriftFixtureInput;
  readonly style_evidence_drift_fixture?: DxStyleDriftFixtureInput;
};

export type DxStyleDriftFixtureReadModel = {
  readonly schema: "dx.www.template.style_evidence_drift_fixture_read_model";
  readonly route: string;
  readonly rowId: string;
  readonly status: string;
  readonly exerciseState:
    | "missing"
    | "preview-only"
    | "readiness-only"
    | "source-guarded"
    | "mismatch";
  readonly exercised: boolean;
  readonly loaderFile: string;
  readonly markerHelperFile: string;
  readonly fixturePath: string;
  readonly states: readonly string[];
  readonly fullAutoprefixerParity: boolean;
  readonly fullTailwindPostcssOutputParity: boolean;
  readonly mismatchFields: readonly string[];
  readonly readsHtml: false;
  readonly readsRawStyleReceipt: false;
  readonly readsReadinessReceipt: boolean;
  readonly nextAction: string;
};

type NormalizedDriftFixture = {
  readonly route: string;
  readonly rowId: string;
  readonly status: string;
  readonly loaderFile: string;
  readonly markerHelperFile: string;
  readonly fixturePath: string;
  readonly states: readonly string[];
  readonly fullAutoprefixerParity: boolean;
  readonly fullTailwindPostcssOutputParity: boolean;
};

const STYLE_EVIDENCE_DRIFT_SCHEMA =
  "dx.www.template.style_evidence_drift_fixture_read_model";
const STYLE_EVIDENCE_DRIFT_NEXT_ACTION =
  "Wire this source-guarded drift fixture into TemplateShell/Studio marker consumers without scraping HTML or raw style receipts.";
const MISSING_STYLE_EVIDENCE_DRIFT_NEXT_ACTION =
  "Publish styleEvidenceDriftFixtures in preview-manifest metadata and style_evidence_drift_fixture in the launch readiness receipt.";

export function dxStylePackagePanelFromPreviewManifest(
  manifest: DxStylePreviewManifest,
  route = "/",
): DxStylePreviewPackagePanel {
  const evidence = dxStyleBrowserCompatEvidenceFromPreviewManifest(
    manifest,
    route,
  );

  return {
    schema: "dx.www.template.preview_style_package_panel_read_model",
    panelId: "dx-style-browser-compat-package-panel",
    packageId: "dx-style",
    title: evidence.title,
    status: evidence.status,
    source: evidence.source,
    route: evidence.route,
    zedVisibility: evidence.zedVisibility,
    receiptPath: evidence.receiptPath,
    fixturePath: evidence.fixturePath,
    canaryClassCount: evidence.canaryClassCount,
    selectorCanaryClassCount: evidence.selectorCanaryClassCount,
    selectorClassExamples: evidence.selectorClassExamples,
    stateAliasSupportedClassCount:
      evidence.tailwindParityStateAliasSupportedClassCount,
    stateAliasSupportedExamples:
      evidence.tailwindParitySupportedStateAliasExamples,
    fullAutoprefixerParity: evidence.fullAutoprefixerParity,
    fullTailwindPostcssOutputParity:
      evidence.fullTailwindPostcssOutputParity,
    readsHtml: false,
    readsRawStyleReceipt: false,
    readsCheckReceipt: false,
    nextAction: evidence.nextAction,
  };
}

export function dxStylePackagePanelFromPreviewAndReadiness(
  manifest: DxStyleDriftFixtureManifest = {},
  readinessReceipt: DxStyleDriftFixtureReadinessReceipt = {},
  route = "/",
): DxStylePreviewPackagePanelWithDrift {
  const panel = dxStylePackagePanelFromPreviewManifest(manifest, route);
  const drift = dxStyleDriftFixtureFromPreviewAndReadiness(
    manifest,
    readinessReceipt,
    route,
  );

  return {
    ...panel,
    schema: "dx.www.template.preview_style_package_panel_with_drift_read_model",
    driftExerciseState: drift.exerciseState,
    driftExercised: drift.exercised,
    driftStatus: drift.status,
    driftLoaderFile: drift.loaderFile,
    driftMarkerHelperFile: drift.markerHelperFile,
    driftStates: drift.states,
    driftMismatchFields: drift.mismatchFields,
    readsReadinessReceipt: drift.readsReadinessReceipt,
    nextAction: drift.exercised ? panel.nextAction : drift.nextAction,
  };
}

export function dxStylePackagePanelMarkersFromPreviewAndReadiness(
  manifest: DxStyleDriftFixtureManifest = {},
  readinessReceipt: DxStyleDriftFixtureReadinessReceipt = {},
  route = "/",
): DxStylePackagePanelMarkerState {
  const panel = dxStylePackagePanelFromPreviewAndReadiness(
    manifest,
    readinessReceipt,
    route,
  );

  return {
    schema: "dx.www.template.preview_style_package_panel_marker_state",
    panelId: panel.panelId,
    readModel: panel.schema,
    driftState: panel.driftExerciseState,
    driftStatus: panel.driftStatus,
    driftMismatchFields: panel.driftMismatchFields,
    readinessReceipt: panel.readsReadinessReceipt ? "read" : "missing",
  };
}

export function dxStyleDriftFixtureFromPreviewAndReadiness(
  manifest: DxStyleDriftFixtureManifest = {},
  readinessReceipt: DxStyleDriftFixtureReadinessReceipt = {},
  route = "/",
): DxStyleDriftFixtureReadModel {
  const preview = normalizeDriftFixture(
    manifest.routes
      ?.find((candidate) => candidate.route === route)
      ?.styleEvidenceDriftFixtures?.find(isDxStyleDriftFixture) ??
      manifest.styleEvidenceDriftFixtures?.find(isDxStyleDriftFixture),
    route,
  );
  const readiness = normalizeDriftFixture(
    readinessReceipt.styleEvidenceDriftFixture ??
      readinessReceipt.style_evidence_drift_fixture,
    route,
  );
  const fixture = preview ?? readiness;

  if (!fixture) {
    return {
      schema: STYLE_EVIDENCE_DRIFT_SCHEMA,
      route,
      rowId: "dx-style-browser-compat",
      status: "missing",
      exerciseState: "missing",
      exercised: false,
      loaderFile: "",
      markerHelperFile: "",
      fixturePath: "",
      states: [],
      fullAutoprefixerParity: false,
      fullTailwindPostcssOutputParity: false,
      mismatchFields: [],
      readsHtml: false,
      readsRawStyleReceipt: false,
      readsReadinessReceipt: false,
      nextAction: MISSING_STYLE_EVIDENCE_DRIFT_NEXT_ACTION,
    };
  }

  const mismatchFields =
    preview && readiness ? driftFixtureMismatchFields(preview, readiness) : [];
  const exerciseState = preview
    ? readiness
      ? mismatchFields.length === 0
        ? "source-guarded"
        : "mismatch"
      : "preview-only"
    : "readiness-only";

  return {
    schema: STYLE_EVIDENCE_DRIFT_SCHEMA,
    route: fixture.route,
    rowId: fixture.rowId,
    status: fixture.status,
    exerciseState,
    exercised: exerciseState === "source-guarded",
    loaderFile: fixture.loaderFile,
    markerHelperFile: fixture.markerHelperFile,
    fixturePath: fixture.fixturePath,
    states: fixture.states,
    fullAutoprefixerParity: fixture.fullAutoprefixerParity,
    fullTailwindPostcssOutputParity:
      fixture.fullTailwindPostcssOutputParity,
    mismatchFields,
    readsHtml: false,
    readsRawStyleReceipt: false,
    readsReadinessReceipt: Boolean(readiness),
    nextAction:
      exerciseState === "source-guarded"
        ? STYLE_EVIDENCE_DRIFT_NEXT_ACTION
        : MISSING_STYLE_EVIDENCE_DRIFT_NEXT_ACTION,
  };
}

function normalizeDriftFixture(
  input: DxStyleDriftFixtureInput | undefined,
  route: string,
): NormalizedDriftFixture | undefined {
  if (!input) {
    return undefined;
  }

  return {
    route: stringValue(input.route, route),
    rowId: stringValue(input.rowId ?? input.row_id, "dx-style-browser-compat"),
    status: stringValue(input.status, "missing"),
    loaderFile: stringValue(input.loaderFile ?? input.loader_file, ""),
    markerHelperFile: stringValue(
      input.markerHelperFile ?? input.marker_helper_file,
      "",
    ),
    fixturePath: stringValue(input.fixturePath ?? input.fixture_path, ""),
    states: arrayValue(input.states),
    fullAutoprefixerParity: booleanValue(
      input.fullAutoprefixerParity ?? input.full_autoprefixer_parity,
    ),
    fullTailwindPostcssOutputParity: booleanValue(
      input.fullTailwindPostcssOutputParity ??
        input.full_tailwind_postcss_output_parity,
    ),
  };
}

function isDxStyleDriftFixture(input: DxStyleDriftFixtureInput): boolean {
  return (input.rowId ?? input.row_id) === "dx-style-browser-compat";
}

function driftFixtureMismatchFields(
  preview: NormalizedDriftFixture,
  readiness: NormalizedDriftFixture,
): readonly string[] {
  const fields: string[] = [];

  for (const field of [
    "route",
    "rowId",
    "status",
    "loaderFile",
    "markerHelperFile",
    "fixturePath",
    "fullAutoprefixerParity",
    "fullTailwindPostcssOutputParity",
  ] as const) {
    if (preview[field] !== readiness[field]) {
      fields.push(field);
    }
  }

  if (preview.states.join("\0") !== readiness.states.join("\0")) {
    fields.push("states");
  }

  return fields;
}

function stringValue(value: string | undefined, fallback: string): string {
  return typeof value === "string" ? value : fallback;
}

function booleanValue(value: boolean | undefined): boolean {
  return typeof value === "boolean" ? value : false;
}

function arrayValue(value: readonly string[] | undefined): readonly string[] {
  return Array.isArray(value)
    ? value.filter((item): item is string => typeof item === "string")
    : [];
}
