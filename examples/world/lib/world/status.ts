import type { WorldStatus, WorldValidationState } from "./contracts";
import { worldConnectionProbes } from "./connections/providers";
import { worldConnectionReceiptPath } from "./connections/runner";
import { missingEnvNames, presentEnvNames, readWorldEnv, requiredEnvNames, type WorldEnvSnapshot } from "./env";
import { worldCategories, worldIntegrations } from "./registry";

function providerState(base: WorldValidationState, missingEnv: readonly string[]): WorldValidationState {
  if (base === "preview-only") {
    return "preview-only";
  }

  if (missingEnv.length > 0) {
    return "preview-only";
  }

  return base;
}

export function buildWorldStatus(snapshot: WorldEnvSnapshot = readWorldEnv()): WorldStatus {
  const providers = worldIntegrations.map((integration) => {
    const missingEnv = missingEnvNames(integration.env, snapshot);
    const state = providerState(integration.validation, missingEnv);

    return {
      id: integration.id,
      packageId: integration.packageId,
      name: integration.name,
      category: integration.category,
      state,
      supportMode: missingEnv.length > 0 ? "missing-config" : integration.supportMode,
      requiredEnv: requiredEnvNames(integration),
      presentEnv: presentEnvNames(integration.env, snapshot),
      missingEnv,
      routeHandlers: integration.routeHandlers,
      receipts: integration.receipts,
      nextAction: missingEnv.length > 0 ? "Provide missing Env Firewall keys before live validation." : integration.nextAction,
    };
  });

  return {
    generatedBy: "examples/world",
    redaction: "secret-values-never-included",
    totals: {
      categories: worldCategories.length,
      providers: providers.length,
      liveReady: providers.filter((provider) => provider.state === "live-validated").length,
      envReady: providers.filter((provider) => provider.missingEnv.length === 0).length,
      previewOnly: providers.filter((provider) => provider.state === "preview-only").length,
      missingEnv: providers.reduce((total, provider) => total + provider.missingEnv.length, 0),
    },
    connections: {
      runner: "examples/world/lib/world/connections/runner.ts",
      receiptPath: worldConnectionReceiptPath,
      probeCount: worldConnectionProbes.length,
      liveProbeMode: "read-only-when-env-present",
    },
    providers,
  };
}
