import { n8nCatalogSummary } from "../catalog";
import type { CatalogSummary, N8nSourceNode } from "../types";
import type { GeneratedConnectorRecord } from "./types";
import { createGeneratedNodeTypeRegistry } from "./registry";
import { generatedExecutableOperationOptions } from "./operation-options";

function sourceFileName(sourceFile: string) {
  const fileName = sourceFile.split("/").at(-1) ?? sourceFile;
  return fileName.replace(/\.node\.ts$/, "");
}

function roleForRecord(record: GeneratedConnectorRecord): N8nSourceNode["role"] {
  if (record.workflow_node.trigger) {
    return "trigger";
  }

  if (record.workflow_node.usable_as_tool) {
    return "ai-tool";
  }

  return record.operations.length > 0 || record.resources.length > 0 ? "action" : "utility";
}

function catalogNodeFromRecord(record: GeneratedConnectorRecord): N8nSourceNode {
  return {
    id: record.id.replace(/^n8n-nodes-base\./, ""),
    name: sourceFileName(record.source_file),
    displayName: record.display_name,
    category: record.categories[0] ?? "Uncategorized",
    role: roleForRecord(record),
    description: record.description,
    sourcePath: record.source_file,
    credentialTypes: record.credential_type_names,
    operations: generatedExecutableOperationOptions(record).map(
      (operation) => operation.value,
    ),
    trustStatus: "source-generated",
  };
}

export function createStudioCatalogFromGeneratedConnectors(
  generatedCatalog: unknown,
  baseCatalog: CatalogSummary = n8nCatalogSummary,
): CatalogSummary {
  const generated = createGeneratedNodeTypeRegistry(generatedCatalog);
  const recordsById = new Map(generated.selectedRecords.map((record) => [record.id, record]));
  const catalogNodes = Array.from(recordsById.values())
    .map(catalogNodeFromRecord)
    .sort((left, right) => left.displayName.localeCompare(right.displayName));

  return {
    ...baseCatalog,
    catalogNodes,
    generatedMetadata: generated.coverage,
  };
}
