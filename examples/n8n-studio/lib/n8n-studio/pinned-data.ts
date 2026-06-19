import type { PinnedDataState, WorkflowDocument } from "./types";

export function createPinnedDataState(document: WorkflowDocument): PinnedDataState[] {
  return Object.entries(document.pinData).map(([nodeName, items]) => ({
    nodeName,
    itemCount: items.length,
    status: "configured",
    sizePolicy: "validated-before-save",
  }));
}

export const pinnedDataState: PinnedDataState[] = [
  {
    nodeName: "Read Connector Manifest",
    itemCount: 1,
    status: "configured",
    sizePolicy: "validated-before-save",
  },
];
