import { worldIntegrations } from "./registry";
import { worldProviderReceiptImportContract } from "./live-probes";

export function worldReceiptPaths() {
  return worldIntegrations.map((integration) => ({
    integration: integration.id,
    receipts: integration.receipts.map((receipt) => `.dx/receipts/world/${integration.id}/${receipt}.sr`),
    importCommand: worldProviderReceiptImportContract.importCommand.replace(
      "<provider-id>",
      integration.id,
    ),
    generatedDxReceiptPolicy:
      worldProviderReceiptImportContract.generatedDxReceiptPolicy,
  }));
}
