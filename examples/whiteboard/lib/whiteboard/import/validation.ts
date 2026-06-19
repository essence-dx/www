import { DXDRAW_FORMAT, importDxDraw } from "../export/dxdraw";
import {
  summarizeWhiteboardDocument,
  type WhiteboardDocumentSummary,
} from "../document-summary";
import type { WhiteboardDocument } from "../model";
import { migrateWhiteboardDocument } from "../persistence/schema";

export type WhiteboardImportFormat = "dxdraw" | "document" | "unknown";

export type WhiteboardImportDiagnostic = {
  readonly severity: "error" | "warning";
  readonly message: string;
};

export type WhiteboardImportValidationReport =
  | {
      readonly status: "accepted";
      readonly format: Exclude<WhiteboardImportFormat, "unknown">;
      readonly document: WhiteboardDocument;
      readonly summary: WhiteboardDocumentSummary;
      readonly diagnostics: readonly WhiteboardImportDiagnostic[];
    }
  | {
      readonly status: "rejected";
      readonly format: WhiteboardImportFormat;
      readonly diagnostics: readonly WhiteboardImportDiagnostic[];
    };

export function validateWhiteboardImport(value: string | unknown): WhiteboardImportValidationReport {
  const parsed = parseImportSource(value);
  if (parsed.error) {
    return rejected("unknown", parsed.error);
  }

  const source = parsed.value;
  const format = detectImportFormat(source);

  try {
    const document = format === "dxdraw" ? importDxDraw(source) : migrateWhiteboardDocument(source);
    const summary = summarizeWhiteboardDocument(document);
    const diagnostics = [
      ...legacyDocumentWarnings(source),
      ...emptyDocumentWarning(summary),
    ];

    return {
      status: "accepted",
      format: format === "dxdraw" ? "dxdraw" : "document",
      document,
      summary,
      diagnostics,
    };
  } catch (error) {
    return rejected(format, error instanceof Error ? error.message : "Whiteboard import failed");
  }
}

function parseImportSource(value: string | unknown): { readonly value: unknown; readonly error?: undefined } | { readonly error: string } {
  if (typeof value !== "string") {
    return { value };
  }

  try {
    return { value: JSON.parse(value) };
  } catch (error) {
    return {
      error: `Invalid whiteboard JSON: ${error instanceof Error ? error.message : "parse failed"}`,
    };
  }
}

function detectImportFormat(value: unknown): WhiteboardImportFormat {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return "unknown";
  }

  const source = value as Record<string, unknown>;
  if (source.format === DXDRAW_FORMAT) {
    return "dxdraw";
  }

  if (source.schemaVersion !== undefined || source.schema !== undefined || source.elements !== undefined) {
    return "document";
  }

  return "unknown";
}

function emptyDocumentWarning(summary: WhiteboardDocumentSummary): readonly WhiteboardImportDiagnostic[] {
  return summary.elementCount === 0
    ? [{ severity: "warning", message: "Imported whiteboard has no elements." }]
    : [];
}

function legacyDocumentWarnings(value: unknown): readonly WhiteboardImportDiagnostic[] {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return [];
  }

  const source = value as Record<string, unknown>;
  return source.schema === "dx.whiteboard.document" && source.schemaVersion === undefined
    ? [{ severity: "warning", message: "Legacy whiteboard document was migrated to the current schema." }]
    : [];
}

function rejected(format: WhiteboardImportFormat, message: string): WhiteboardImportValidationReport {
  return {
    status: "rejected",
    format,
    diagnostics: [{ severity: "error", message }],
  };
}
