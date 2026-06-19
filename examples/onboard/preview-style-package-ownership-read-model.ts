export type DxStylePackageUnsupportedInput =
  | string
  | {
      readonly className?: string;
      readonly class_name?: string;
      readonly reason?: string;
    };

export type DxStylePackageOwnershipInput = {
  readonly schema?: string;
  readonly packageId?: string;
  readonly package_id?: string;
  readonly packageName?: string;
  readonly package_name?: string;
  readonly officialPackageName?: string;
  readonly official_package_name?: string;
  readonly styleScope?: string;
  readonly style_scope?: string;
  readonly sourceFiles?: readonly string[];
  readonly source_files?: readonly string[];
  readonly requiredTokens?: readonly string[];
  readonly required_tokens?: readonly string[];
  readonly generatedClasses?: readonly string[];
  readonly generated_classes?: readonly string[];
  readonly unsupportedClasses?: readonly DxStylePackageUnsupportedInput[];
  readonly unsupported_classes?: readonly DxStylePackageUnsupportedInput[];
  readonly tokenSource?: string;
  readonly token_source?: string;
  readonly generatedCss?: string;
  readonly generated_css?: string;
  readonly receiptPath?: string;
  readonly receipt_path?: string;
  readonly zedVisibility?: string;
  readonly zed_visibility?: string;
  readonly runtimeProof?: boolean;
  readonly runtime_proof?: boolean;
};

export type DxStylePackageOwnershipManifestRoute = {
  readonly route: string;
  readonly stylePackageOwnershipRows?: readonly DxStylePackageOwnershipInput[];
};

export type DxStylePackageOwnershipManifest = {
  readonly stylePackageOwnershipRows?: readonly DxStylePackageOwnershipInput[];
  readonly routes?: readonly DxStylePackageOwnershipManifestRoute[];
};

export type DxStylePackageOwnershipReadinessReceipt = {
  readonly stylePackageOwnershipRows?: readonly DxStylePackageOwnershipInput[];
  readonly style_package_ownership_rows?: readonly DxStylePackageOwnershipInput[];
};

export type DxStyleUnsupportedClassOwnership = {
  readonly packageId: string;
  readonly className: string;
  readonly reason: string;
};

export type DxStylePackageOwnershipRow = {
  readonly schema: "dx.style.package_ownership";
  readonly packageId: string;
  readonly packageName: string;
  readonly styleScope: string;
  readonly sourceFiles: readonly string[];
  readonly requiredTokens: readonly string[];
  readonly generatedClasses: readonly string[];
  readonly unsupportedClasses: readonly DxStyleUnsupportedClassOwnership[];
  readonly tokenSource: string;
  readonly generatedCss: string;
  readonly receiptPath: string;
  readonly zedVisibility: string;
  readonly runtimeProof: boolean;
};

export type DxStylePackageOwnershipReadModel = {
  readonly schema: "dx.www.template.style_package_ownership_read_model";
  readonly source:
    | `preview_manifest.routes[${string}].stylePackageOwnershipRows`
    | "preview_manifest.stylePackageOwnershipRows"
    | "readiness_receipt.style_package_ownership_rows"
    | "missing";
  readonly route: string;
  readonly packageCount: number;
  readonly classOwnershipCount: number;
  readonly unsupportedClassCount: number;
  readonly packageIds: readonly string[];
  readonly tokenUsage: readonly string[];
  readonly packages: readonly DxStylePackageOwnershipRow[];
  readonly readsHtml: false;
  readonly readsRawStyleReceipt: false;
  readonly readsCheckReceipt: false;
  readonly readsReadinessReceipt: boolean;
  readonly nextAction: string;
};

const STYLE_PACKAGE_OWNERSHIP_SCHEMA =
  "dx.www.template.style_package_ownership_read_model";
const MISSING_NEXT_ACTION =
  "Publish stylePackageOwnershipRows in preview-manifest metadata or style_package_ownership_rows in the launch readiness receipt.";
const PRESENT_NEXT_ACTION =
  "Feed dx-check, Forge, and Zed package panels from this read model so generated and unsupported classes stay package-owned.";
const DEFAULT_UNSUPPORTED_REASON =
  "dx-style did not generate CSS for this package-owned class in the current evidence set";

export function dxStylePackageOwnershipFromPreviewAndReadiness(
  manifest: DxStylePackageOwnershipManifest = {},
  readinessReceipt: DxStylePackageOwnershipReadinessReceipt = {},
  route = "/",
): DxStylePackageOwnershipReadModel {
  const routeRows = manifest.routes?.find(
    (candidate) => candidate.route === route,
  )?.stylePackageOwnershipRows;
  const rootRows = manifest.stylePackageOwnershipRows;
  const readinessRows =
    readinessReceipt.stylePackageOwnershipRows ??
    readinessReceipt.style_package_ownership_rows;

  const inputRows = routeRows ?? rootRows ?? readinessRows ?? [];
  const source = routeRows
    ? `preview_manifest.routes[${route}].stylePackageOwnershipRows`
    : rootRows
      ? "preview_manifest.stylePackageOwnershipRows"
      : readinessRows
        ? "readiness_receipt.style_package_ownership_rows"
        : "missing";
  const packages = inputRows.map(normalizePackageOwnership).filter(isPresent);

  return {
    schema: STYLE_PACKAGE_OWNERSHIP_SCHEMA,
    source,
    route,
    packageCount: packages.length,
    classOwnershipCount: packages.reduce(
      (count, row) => count + row.generatedClasses.length,
      0,
    ),
    unsupportedClassCount: packages.reduce(
      (count, row) => count + row.unsupportedClasses.length,
      0,
    ),
    packageIds: sortedUnique(packages.map((row) => row.packageId)),
    tokenUsage: sortedUnique(packages.flatMap((row) => row.requiredTokens)),
    packages,
    readsHtml: false,
    readsRawStyleReceipt: false,
    readsCheckReceipt: false,
    readsReadinessReceipt: source === "readiness_receipt.style_package_ownership_rows",
    nextAction: packages.length > 0 ? PRESENT_NEXT_ACTION : MISSING_NEXT_ACTION,
  };
}

function normalizePackageOwnership(
  input: DxStylePackageOwnershipInput,
): DxStylePackageOwnershipRow | undefined {
  const packageId = stringValue(input.packageId ?? input.package_id);
  if (!packageId) {
    return undefined;
  }

  return {
    schema: "dx.style.package_ownership",
    packageId,
    packageName: stringValue(
      input.packageName ??
        input.package_name ??
        input.officialPackageName ??
        input.official_package_name,
      packageId,
    ),
    styleScope: stringValue(input.styleScope ?? input.style_scope, packageId),
    sourceFiles: stringArray(input.sourceFiles ?? input.source_files),
    requiredTokens: stringArray(input.requiredTokens ?? input.required_tokens),
    generatedClasses: stringArray(
      input.generatedClasses ?? input.generated_classes,
    ),
    unsupportedClasses: unsupportedClasses(
      packageId,
      input.unsupportedClasses ?? input.unsupported_classes,
    ),
    tokenSource: stringValue(input.tokenSource ?? input.token_source),
    generatedCss: stringValue(input.generatedCss ?? input.generated_css),
    receiptPath: stringValue(input.receiptPath ?? input.receipt_path),
    zedVisibility: stringValue(input.zedVisibility ?? input.zed_visibility),
    runtimeProof: booleanValue(input.runtimeProof ?? input.runtime_proof),
  };
}

function unsupportedClasses(
  packageId: string,
  inputs: readonly DxStylePackageUnsupportedInput[] | undefined,
): readonly DxStyleUnsupportedClassOwnership[] {
  if (!Array.isArray(inputs)) {
    return [];
  }

  return inputs
    .map((input) => unsupportedClass(packageId, input))
    .filter(isPresent);
}

function unsupportedClass(
  packageId: string,
  input: DxStylePackageUnsupportedInput,
): DxStyleUnsupportedClassOwnership | undefined {
  if (typeof input === "string") {
    return {
      packageId,
      className: input,
      reason: DEFAULT_UNSUPPORTED_REASON,
    };
  }

  const className = stringValue(input.className ?? input.class_name);
  if (!className) {
    return undefined;
  }

  return {
    packageId,
    className,
    reason: stringValue(input.reason, DEFAULT_UNSUPPORTED_REASON),
  };
}

function stringValue(value: string | undefined, fallback = ""): string {
  return typeof value === "string" ? value : fallback;
}

function stringArray(value: readonly string[] | undefined): readonly string[] {
  return Array.isArray(value)
    ? value.filter((item): item is string => typeof item === "string")
    : [];
}

function booleanValue(value: boolean | undefined): boolean {
  return typeof value === "boolean" ? value : false;
}

function sortedUnique(values: readonly string[]): readonly string[] {
  return [...new Set(values)].sort((a, b) => a.localeCompare(b));
}

function isPresent<T>(value: T | undefined): value is T {
  return value !== undefined;
}
