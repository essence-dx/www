import {
  migrateWhiteboardDocument,
  validateWhiteboardDocument,
  type WhiteboardDocument,
} from "../persistence/schema";
import {
  createWhiteboardExportMetadata,
  type WhiteboardExportMetadata,
} from "./metadata";

export const DXDRAW_FORMAT = "dx.whiteboard.dxdraw";
export const DXDRAW_VERSION = 1;

export type DxDrawEnvelope = {
  format: typeof DXDRAW_FORMAT;
  version: typeof DXDRAW_VERSION;
  exportedAt: string;
  metadata: WhiteboardExportMetadata;
  document: WhiteboardDocument;
};

export type DxDrawExportOptions = {
  exportedAt?: string;
};

function parseJson(value: string): unknown {
  try {
    return JSON.parse(value);
  } catch (error) {
    throw new Error(`Invalid .dxdraw JSON: ${error instanceof Error ? error.message : "parse failed"}`);
  }
}

function envelope(value: unknown): Record<string, unknown> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw new Error(".dxdraw file must contain a JSON object");
  }

  return value as Record<string, unknown>;
}

export function exportDxDraw(document: WhiteboardDocument, options: DxDrawExportOptions = {}): string {
  const validated = validateWhiteboardDocument(document);
  const exportedAt = options.exportedAt ?? new Date().toISOString();
  const payload: DxDrawEnvelope = {
    format: DXDRAW_FORMAT,
    version: DXDRAW_VERSION,
    exportedAt,
    metadata: createWhiteboardExportMetadata(validated, {
      format: "dxdraw",
      exportedAt,
    }),
    document: validated,
  };

  return JSON.stringify(payload, null, 2);
}

export function importDxDraw(value: string | unknown): WhiteboardDocument {
  const source = envelope(typeof value === "string" ? parseJson(value) : value);

  if (source.format !== DXDRAW_FORMAT) {
    throw new Error("Unsupported .dxdraw format");
  }

  if (source.version !== DXDRAW_VERSION) {
    throw new Error("Unsupported .dxdraw version");
  }

  return migrateWhiteboardDocument(source.document);
}
