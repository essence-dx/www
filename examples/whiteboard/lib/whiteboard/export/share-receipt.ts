import {
  createWhiteboardExportMetadata,
  type WhiteboardExportMetadata,
} from "./metadata";
import type { WhiteboardDocument } from "../model";

export type WhiteboardShareTarget = "local-copy" | "local-preview" | "external-adapter";

export type WhiteboardShareReceipt = {
  readonly schema_version: "dx.whiteboard.share_receipt.v1";
  readonly status: "preview-only" | "adapter-ready";
  readonly target: WhiteboardShareTarget;
  readonly createdAt: string;
  readonly metadata: WhiteboardExportMetadata;
  readonly shareUrl: string | null;
  readonly mutableRemoteState: false;
  readonly notes: readonly string[];
};

export function createWhiteboardShareReceipt(
  document: WhiteboardDocument,
  options: {
    readonly createdAt?: string;
    readonly target?: WhiteboardShareTarget;
    readonly shareUrl?: string | null;
  } = {},
): WhiteboardShareReceipt {
  const createdAt = options.createdAt ?? new Date().toISOString();
  const target = options.target ?? "local-preview";

  return {
    schema_version: "dx.whiteboard.share_receipt.v1",
    status: target === "external-adapter" && options.shareUrl ? "adapter-ready" : "preview-only",
    target,
    createdAt,
    metadata: createWhiteboardExportMetadata(document, {
      format: "share-receipt",
      exportedAt: createdAt,
    }),
    shareUrl: options.shareUrl ?? null,
    mutableRemoteState: false,
    notes: [
      "This receipt proves the board is exportable and share-preparable without claiming live collaboration.",
      "External sharing adapters must attach an exact URL before status can become adapter-ready.",
    ],
  };
}

export function exportWhiteboardShareReceipt(
  document: WhiteboardDocument,
  options: Parameters<typeof createWhiteboardShareReceipt>[1] = {},
): string {
  return JSON.stringify(createWhiteboardShareReceipt(document, options), null, 2);
}
