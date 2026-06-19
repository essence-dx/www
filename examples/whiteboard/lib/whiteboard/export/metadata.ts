import {
  summarizeWhiteboardDocument,
  type WhiteboardDocumentSummary,
} from "../document-summary";
import { WHITEBOARD_IMAGE_SOURCE_POLICY, type WhiteboardImageSourcePolicy } from "../image-source";
import type { WhiteboardDocument } from "../model";

export type WhiteboardExportFormat = "dxdraw" | "svg" | "png-plan" | "share-receipt";

export type WhiteboardExportMetadata = {
  readonly schema_version: "dx.whiteboard.export_metadata.v1";
  readonly format: WhiteboardExportFormat;
  readonly exportedAt: string;
  readonly document: WhiteboardDocumentSummary;
  readonly sourceOwned: true;
  readonly imagePolicy: WhiteboardImageSourcePolicy;
  readonly includesSecretValues: false;
  readonly runtimeDependency: "dx-www-whiteboard";
};

export function createWhiteboardExportMetadata(
  document: WhiteboardDocument,
  options: {
    readonly format: WhiteboardExportFormat;
    readonly exportedAt?: string;
  },
): WhiteboardExportMetadata {
  return {
    schema_version: "dx.whiteboard.export_metadata.v1",
    format: options.format,
    exportedAt: options.exportedAt ?? new Date().toISOString(),
    document: summarizeWhiteboardDocument(document),
    sourceOwned: true,
    imagePolicy: WHITEBOARD_IMAGE_SOURCE_POLICY,
    includesSecretValues: false,
    runtimeDependency: "dx-www-whiteboard",
  };
}
