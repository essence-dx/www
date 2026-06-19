import { n8nCatalogSummary } from "../../lib/n8n-studio/catalog";
import type { CatalogSummary } from "../../lib/n8n-studio/types";

export function createCatalogResponse(catalog: CatalogSummary = n8nCatalogSummary) {
  return {
    schema: catalog.schema,
    sourceManifestPath: catalog.sourceManifestPath,
    copiedFrom: catalog.copiedFrom,
    counts: {
      nodeFolders: catalog.nodeFolderCount,
      nodeFiles: catalog.nodeFileCount,
      credentialFiles: catalog.credentialFileCount,
    },
    nodes: catalog.catalogNodes,
    generatedMetadata: catalog.generatedMetadata ?? {
      sourceAvailable: false,
      sourcePath: "integrations/n8n-nodes-base/generated/dx-automations-connectors.json",
      sourceRecordCount: 0,
      nodeTypeCount: 0,
      skippedRecordCount: 0,
      credentialTypeCount: 0,
      issue: "generated connector catalog was not loaded",
    },
    providerBoundary: true,
    liveProviderExecution: false,
  };
}
