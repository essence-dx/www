import type { WorldCategory, WorldIntegration } from "./contracts";
import { aiRealtimeWorkflowCategories } from "./lanes/ai-realtime-workflows";
import { commerceMediaSearchCategories } from "./lanes/commerce-media-search";
import { contentDeployCategories } from "./lanes/content-deploy";
import { dataIdentityCategories } from "./lanes/data-identity";
import { operationsCategories } from "./lanes/operations";
import { trustProductivityCategories } from "./lanes/trust-productivity";

export const worldCategories: readonly WorldCategory[] = [
  ...dataIdentityCategories,
  ...commerceMediaSearchCategories,
  ...aiRealtimeWorkflowCategories,
  ...operationsCategories,
  ...contentDeployCategories,
  ...trustProductivityCategories,
];

export const worldIntegrations: readonly WorldIntegration[] = worldCategories.flatMap(
  (category) => category.providers,
);

export function getWorldSummary() {
  const envNames = new Set<string>();
  const receipts = new Set<string>();

  for (const integration of worldIntegrations) {
    for (const item of integration.env) {
      envNames.add(item.name);
    }

    for (const receipt of integration.receipts) {
      receipts.add(receipt);
    }
  }

  return {
    categoryCount: worldCategories.length,
    providerCount: worldIntegrations.length,
    requiredEnvCount: envNames.size,
    receiptCount: receipts.size,
  } as const;
}
