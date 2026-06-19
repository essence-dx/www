import type { GeneratedConnectorRecord } from "./types";

function pathDepth(sourcePath: string) {
  return sourcePath.split("/").length;
}

function recordScore(record: GeneratedConnectorRecord) {
  const versionedPath = /\/v?\d+(?:[./]|$)/i.test(record.source_file);
  const optionCount = record.resources.length + record.operations.length;
  const sourcePreference = versionedPath ? 0 : 200;
  const toolPreference = record.workflow_node.usable_as_tool ? 10 : 0;
  const triggerPreference = record.workflow_node.trigger ? 5 : 0;

  return sourcePreference + optionCount + toolPreference + triggerPreference - pathDepth(record.source_file);
}

export function selectGeneratedConnectorRecords(records: GeneratedConnectorRecord[]) {
  const selected = new Map<string, GeneratedConnectorRecord>();

  for (const record of records) {
    const existing = selected.get(record.id);
    if (!existing || recordScore(record) > recordScore(existing)) {
      selected.set(record.id, record);
    }
  }

  return Array.from(selected.values()).sort((left, right) =>
    left.id.localeCompare(right.id),
  );
}
