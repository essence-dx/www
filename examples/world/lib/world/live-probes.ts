import type { WorldIntegration, WorldSupportMode } from "./contracts";

export type WorldRouteHandlerGapId =
  | "imported-helper-execution"
  | "server-only-env-injection"
  | "route-handler-fetch"
  | "provider-receipt-import";

export type WorldRouteHandlerRuntimeGap = {
  id: WorldRouteHandlerGapId;
  frameworkArea: string;
  currentLimitation: string;
  planAction: string;
};

export type WorldLiveProbeStatus =
  | "preview-only"
  | "configured-readiness"
  | "live-validated";

export type WorldLiveProbeReadiness = {
  providerId: string;
  packageId: string;
  status: WorldLiveProbeStatus;
  supportMode: WorldSupportMode;
  requiredEnv: readonly string[];
  presentEnv: readonly string[];
  missingEnv: readonly string[];
  envSatisfied: boolean;
  providerReceiptImported: boolean;
  liveProviderExecution: false;
  liveProviderProof: boolean;
  generatedDxReceiptPolicy: "never-commit-generated-receipts";
  receiptImportRequired: true;
  routeHandlerGaps: readonly WorldRouteHandlerGapId[];
};

export const worldRouteHandlerRuntimeGaps: readonly WorldRouteHandlerRuntimeGap[] = [
  {
    id: "imported-helper-execution",
    frameworkArea: "app/api route handlers",
    currentLimitation:
      "World API routes keep inline preview payloads because current route-handler execution cannot yet prove imported TypeScript helper execution for provider probes.",
    planAction:
      "Execute imported server helpers in route handlers with source-owned module resolution, deterministic errors, and receipt-visible helper provenance.",
  },
  {
    id: "server-only-env-injection",
    frameworkArea: "DX Env Firewall and route handlers",
    currentLimitation:
      "World provider cards can declare server env names, but route handlers do not yet receive a redacted server-only env capability map from the framework.",
    planAction:
      "Inject server-only env presence into route handlers as redacted capability data while keeping values sealed and browser-public keys explicit.",
  },
  {
    id: "route-handler-fetch",
    frameworkArea: "route handler provider probes",
    currentLimitation:
      "Live provider checks require HTTP fetch support with timeout, method, header, body, and redaction rules; the world example does not claim that runtime today.",
    planAction:
      "Add route-handler fetch support for read-only probes with timeout limits, no secret echoing, and response receipts that separate network success from product readiness.",
  },
  {
    id: "provider-receipt-import",
    frameworkArea: "world receipts",
    currentLimitation:
      "A provider cannot become live-validated from env presence alone; WWW needs a provider receipt import path that reads evidence without committing generated .dx receipts.",
    planAction:
      "Add provider receipt import for redacted live evidence and keep generated .dx receipt artifacts out of source control unless an operator explicitly archives them.",
  },
] as const;

export const worldRouteHandlerRuntimeGapIds = worldRouteHandlerRuntimeGaps.map(
  (gap) => gap.id,
);

export const worldProviderReceiptImportContract = {
  schema: "dx.examples.world.provider_receipt_import_contract",
  importCommand:
    "dx world receipts import <provider-receipt.json> --provider <provider-id> --json",
  receiptPathPattern: ".dx/receipts/world/<provider-id>/<receipt-id>.sr",
  configuredReadinessStatus: "configured-readiness",
  liveValidatedStatus: "live-validated",
  generatedDxReceiptPolicy: "never-commit-generated-receipts",
  valuePolicy: "secret-values-never-included",
  liveValidationRule:
    "Env presence can only promote a provider to configured-readiness; live-validated requires an imported provider receipt.",
} as const;

export const worldLiveProbeStatusExamples = [
  {
    status: "configured-readiness",
    providerReceiptImported: false,
    liveProviderExecution: false,
  },
  {
    status: "live-validated",
    providerReceiptImported: true,
    liveProviderExecution: false,
  },
] as const;

export function evaluateWorldLiveProbeReadiness(
  integration: WorldIntegration,
  presentEnv: readonly string[],
  providerReceiptImported = false,
): WorldLiveProbeReadiness {
  const requiredEnv = integration.env
    .filter((item) => item.required)
    .map((item) => item.name);
  const missingEnv = requiredEnv.filter((name) => !presentEnv.includes(name));
  const envSatisfied = missingEnv.length === 0;
  const liveProviderProof = envSatisfied && providerReceiptImported;
  const status: WorldLiveProbeStatus = liveProviderProof
    ? "live-validated"
    : envSatisfied
      ? "configured-readiness"
      : "preview-only";

  return {
    providerId: integration.id,
    packageId: integration.packageId,
    status,
    supportMode:
      status === "preview-only" && requiredEnv.length > 0
        ? "missing-config"
        : integration.supportMode,
    requiredEnv,
    presentEnv: presentEnv.filter((name) => requiredEnv.includes(name)),
    missingEnv,
    envSatisfied,
    providerReceiptImported,
    liveProviderExecution: false,
    liveProviderProof,
    generatedDxReceiptPolicy:
      worldProviderReceiptImportContract.generatedDxReceiptPolicy,
    receiptImportRequired: true,
    routeHandlerGaps: worldRouteHandlerRuntimeGapIds,
  };
}
