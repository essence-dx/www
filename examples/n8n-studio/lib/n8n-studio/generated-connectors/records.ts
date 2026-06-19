import type { GeneratedConnectorCatalog, GeneratedConnectorRecord } from "./types";

const generatedConnectorSourcePath =
  "integrations/n8n-nodes-base/generated/dx-automations-connectors.json";

function isRecord(value: unknown): value is GeneratedConnectorRecord {
  if (!value || typeof value !== "object") {
    return false;
  }

  const record = value as Partial<GeneratedConnectorRecord>;
  return (
    typeof record.id === "string" &&
    typeof record.display_name === "string" &&
    typeof record.node_name === "string" &&
    typeof record.source_file === "string" &&
    Array.isArray(record.resources) &&
    Array.isArray(record.operations) &&
    Array.isArray(record.credential_type_names)
  );
}

export function readGeneratedConnectorRecords(
  catalog: unknown,
): {
  catalog: GeneratedConnectorCatalog;
  records: GeneratedConnectorRecord[];
  skippedRecordCount: number;
  sourceRecordCount: number;
} {
  if (!catalog || typeof catalog !== "object") {
    throw new Error("Generated n8n connector catalog must be an object");
  }

  const source = catalog as Partial<GeneratedConnectorCatalog>;
  if (!Array.isArray(source.connectors)) {
    throw new Error("Generated n8n connector catalog is missing connectors");
  }

  const records = source.connectors.filter(isRecord);
  const sourceRecordCount = source.connectors.length;

  return {
    catalog: {
      schema: typeof source.schema === "string" ? source.schema : "dx.automations.connectors",
      generated_at: source.generated_at,
      connectors: records,
    },
    records,
    skippedRecordCount: sourceRecordCount - records.length,
    sourceRecordCount,
  };
}

export function generatedConnectorCoverage(
  catalog: GeneratedConnectorCatalog,
  sourceRecordCount: number,
  nodeTypeCount: number,
  skippedRecordCount: number,
  credentialTypeCount: number,
) {
  return {
    sourceAvailable: true,
    sourcePath: generatedConnectorSourcePath,
    sourceRecordCount,
    nodeTypeCount,
    skippedRecordCount,
    credentialTypeCount,
    generatedAt: catalog.generated_at,
  };
}
