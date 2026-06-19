import { readdir, readFile } from "node:fs/promises";
import { basename, join, resolve } from "node:path";

import { summarizeWhiteboardDocument, type WhiteboardDocumentSummary } from "../../lib/whiteboard/document-summary";
import { importDxDraw } from "../../lib/whiteboard/export/dxdraw";

export type WhiteboardFileSnapshotDiagnostic = {
  readonly file: string;
  readonly message: string;
};

export type WhiteboardFileStorageSnapshot = {
  readonly schema_version: "dx.whiteboard.filesystem_snapshot.v1";
  readonly generatedAt: string;
  readonly storageRoot: {
    readonly name: string;
    readonly redacted: true;
  };
  readonly totalFiles: number;
  readonly validCount: number;
  readonly invalidCount: number;
  readonly boards: readonly WhiteboardDocumentSummary[];
  readonly diagnostics: readonly WhiteboardFileSnapshotDiagnostic[];
};

export async function createWhiteboardFileStorageSnapshot(options: {
  readonly rootDir: string;
  readonly generatedAt?: string;
}): Promise<WhiteboardFileStorageSnapshot> {
  const generatedAt = options.generatedAt ?? new Date().toISOString();
  const files = (await readdir(options.rootDir).catch(() => [])).filter((file) => file.endsWith(".dxdraw"));
  const boards: WhiteboardDocumentSummary[] = [];
  const diagnostics: WhiteboardFileSnapshotDiagnostic[] = [];

  for (const file of files) {
    try {
      const document = importDxDraw(await readFile(join(options.rootDir, file), "utf8"));
      boards.push(summarizeWhiteboardDocument(document));
    } catch (error) {
      diagnostics.push({
        file: basename(file),
        message: error instanceof Error ? error.message : "Unable to read whiteboard file.",
      });
    }
  }

  boards.sort((left, right) => right.updatedAt.localeCompare(left.updatedAt) || left.name.localeCompare(right.name));

  return {
    schema_version: "dx.whiteboard.filesystem_snapshot.v1",
    generatedAt,
    storageRoot: {
      name: basename(resolve(options.rootDir)) || "whiteboard-storage",
      redacted: true,
    },
    totalFiles: files.length,
    validCount: boards.length,
    invalidCount: diagnostics.length,
    boards,
    diagnostics,
  };
}
