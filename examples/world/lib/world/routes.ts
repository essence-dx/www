import type { WorldIntegration } from "./contracts";
import type { WorldRouteHandlerGapId } from "./live-probes";
import { worldIntegrations } from "./registry";
import { worldRouteHandlerRuntimeGapIds } from "./live-probes";

export type WorldRouteContract = {
  categoryId: string;
  providerId: string;
  packageId: string;
  method: "GET" | "POST";
  path: string;
  mode: "readiness" | "preview-action" | "live-action";
  redaction: "required";
  liveValidationRequiresEnv: boolean;
  providerReceiptImportRequired: true;
  routeHandlerRuntimeGaps: readonly WorldRouteHandlerGapId[];
  receiptSchema: string;
};

export function routeContractsFor(provider: WorldIntegration): readonly WorldRouteContract[] {
  return provider.routeHandlers.map((handler) => {
    const [method, path] = handler.split(" ");

    return {
      categoryId: provider.categoryId,
      providerId: provider.id,
      packageId: provider.packageId,
      method: method === "POST" ? "POST" : "GET",
      path,
      mode: method === "POST" ? "preview-action" : "readiness",
      redaction: "required",
      liveValidationRequiresEnv: provider.env.some((item) => item.required),
      providerReceiptImportRequired: true,
      routeHandlerRuntimeGaps: worldRouteHandlerRuntimeGapIds,
      receiptSchema: provider.receipts[0] ?? "dx.forge.world.preview-only",
    };
  });
}

export const worldRouteContracts: readonly WorldRouteContract[] = worldIntegrations.flatMap(routeContractsFor);
