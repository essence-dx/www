import type { NodeTypeDescription } from "../node-types/types";
import { generatedRecordToNodeType } from "./node-types";
import { generatedConnectorCoverage, readGeneratedConnectorRecords } from "./records";
import { selectGeneratedConnectorRecords } from "./select-record";
import type { GeneratedConnectorRecord, GeneratedNodeTypeRegistry } from "./types";

export type GeneratedNodeTypeRegistryResult = GeneratedNodeTypeRegistry & {
  selectedRecords: GeneratedConnectorRecord[];
};

function credentialTypeCount(records: GeneratedConnectorRecord[]) {
  const credentialTypes = new Set<string>();
  for (const record of records) {
    for (const credentialType of record.credential_type_names) {
      credentialTypes.add(credentialType);
    }
  }
  return credentialTypes.size;
}

export function createGeneratedNodeTypeRegistry(
  generatedCatalog: unknown,
): GeneratedNodeTypeRegistryResult {
  const { catalog, records, skippedRecordCount, sourceRecordCount } =
    readGeneratedConnectorRecords(generatedCatalog);
  const selectedRecords = selectGeneratedConnectorRecords(records);
  const registry: Record<string, NodeTypeDescription> = {};

  for (const record of selectedRecords) {
    registry[record.id] = generatedRecordToNodeType(record);
  }

  return {
    registry,
    selectedRecords,
    coverage: generatedConnectorCoverage(
      catalog,
      sourceRecordCount,
      selectedRecords.length,
      skippedRecordCount,
      credentialTypeCount(selectedRecords),
    ),
  };
}
