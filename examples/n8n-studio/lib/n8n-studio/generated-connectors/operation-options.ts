import type { GeneratedConnectorOption, GeneratedConnectorRecord } from "./types";

export function generatedExecutableOperationOptions(
  record: GeneratedConnectorRecord,
): GeneratedConnectorOption[] {
  return record.actions.length > 0 ? record.actions : record.operations;
}
