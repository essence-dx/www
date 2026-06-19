import type { ReceiptSummary, StudioSurface } from "./types";

export const requiredStudioSurfaces: StudioSurface[] = [
  "node-creator",
  "workflow-canvas",
  "node-parameters",
  "expression-editor",
  "credentials",
  "resource-locator",
  "pinned-data",
  "execution-debug",
  "ai-tools",
  "import-export",
  "receipts",
];

export const receiptSummary: ReceiptSummary = {
  schema: "dx.n8n-studio.receipts",
  receiptRoot: ".dx/receipts/n8n-studio",
  providerBoundary: true,
  liveProviderExecution: false,
  redaction: "secret-values-never-included",
  surfaces: requiredStudioSurfaces,
};

