import { whiteboardActions } from "../../lib/stores/whiteboard-store";
import { exportDxDraw } from "../../lib/whiteboard/export/dxdraw";
import {
  createImportedImageElement,
  WHITEBOARD_IMAGE_IMPORT_MAX_BYTES,
} from "../../lib/whiteboard/image-import";
import { validateWhiteboardImport } from "../../lib/whiteboard/import";
import type { WhiteboardDocument } from "../../lib/whiteboard/model";
import { DxIcon } from "./toolbar";

export type ImportPanelProps = {
  readonly document: WhiteboardDocument;
};

export function ImportPanel({ document }: ImportPanelProps) {
  const selfCheck = validateWhiteboardImport(
    exportDxDraw(document, { exportedAt: "2026-06-03T00:00:00.000Z" }),
  );

  return (
    <section className="wb-panel wb-import-panel" aria-labelledby="wb-import-title">
      <div className="wb-panel-heading">
        <div>
          <p className="wb-eyebrow">Files</p>
          <h2 id="wb-import-title">Open Files</h2>
        </div>
        <span className="wb-panel-badge">{selfCheck.status}</span>
      </div>

      <div className="wb-import-actions">
        <label className="wb-primary-button wb-file-button">
          <DxIcon name="download" />
          <span>Open DXDraw</span>
          <input
            accept=".dxdraw,application/json"
            aria-label="Open a whiteboard file"
            aria-describedby="wb-import-status"
            className="wb-file-input"
            onInput={(event) => importWhiteboardFile(event.currentTarget.files?.[0], "wb-import-status")}
            type="file"
          />
        </label>
        <label className="wb-secondary-button wb-file-button" data-whiteboard-image-policy="embedded-data-url-only">
          <DxIcon name="image" />
          <span>Add Image</span>
          <input
            accept="image/png,image/jpeg,image/webp,image/svg+xml"
            aria-label="Add a local image"
            aria-describedby="wb-image-import-status"
            className="wb-file-input"
            onInput={(event) => {
              importImageFile(event.currentTarget.files?.[0], "wb-image-import-status");
              event.currentTarget.value = "";
            }}
            type="file"
          />
        </label>
      </div>

      <div className="wb-import-summary" data-whiteboard-file-validation="schema-validated">
        <span>{selfCheck.status === "accepted" ? `${selfCheck.summary.elementCount} elements` : "Unavailable"}</span>
        <span>{selfCheck.format}</span>
      </div>

      <p className="wb-export-note">
        Files accept source-owned `.dxdraw` envelopes and canonical whiteboard document JSON.
      </p>
      <p
        aria-live="polite"
        className="wb-import-status"
        data-whiteboard-file-status="visible"
        id="wb-import-status"
        role="status"
      >
        Current board validates as a DXDraw source.
      </p>
      <p
        aria-live="polite"
        className="wb-import-status"
        data-whiteboard-image-status="visible"
        id="wb-image-import-status"
        role="status"
      >
        Local image loading embeds PNG, JPEG, WebP, or SVG data URLs up to {formatBytes(WHITEBOARD_IMAGE_IMPORT_MAX_BYTES)}.
      </p>
    </section>
  );
}

async function importWhiteboardFile(file: File | undefined, statusId: string) {
  if (!file) return;

  const report = validateWhiteboardImport(await file.text());
  if (report.status === "accepted") {
    writeImportStatus(statusId, `Loaded ${report.summary.elementCount} elements from ${file.name}.`);
    whiteboardActions.reset(report.document);
    return;
  }

  writeImportStatus(statusId, report.diagnostics.map((diagnostic) => diagnostic.message).join(" "));
  console.warn("Whiteboard file load rejected", report.diagnostics);
}

async function importImageFile(file: File | undefined, statusId: string) {
  if (!file) return;

  try {
    const source = await readFileAsDataUrl(file);
    const naturalSize = await readImageNaturalSize(source);
    const imported = createImportedImageElement({
      id: importedImageId(file.name),
      fileName: file.name,
      fileType: file.type,
      fileSizeBytes: file.size,
      naturalWidth: naturalSize?.width,
      naturalHeight: naturalSize?.height,
      source,
    });

    if (imported.status === "accepted") {
      whiteboardActions.importImage(imported.element);
      writeImportStatus(
        statusId,
        `Embedded ${file.name} as a local ${imported.mimeType} image.`,
      );
      return;
    }

    writeImportStatus(statusId, imported.diagnostics.map((diagnostic) => diagnostic.message).join(" "));
  } catch (error) {
    writeImportStatus(statusId, error instanceof Error ? error.message : "Image loading failed.");
  }
}

function readFileAsDataUrl(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.addEventListener("load", () => {
      if (typeof reader.result === "string") {
        resolve(reader.result);
        return;
      }

      reject(new Error("Image loading did not produce a data URL."));
    });
    reader.addEventListener("error", () => reject(new Error("Image loading could not read the local file.")));
    reader.readAsDataURL(file);
  });
}

function readImageNaturalSize(source: string): Promise<{ readonly width: number; readonly height: number } | null> {
  return new Promise((resolve) => {
    const image = new Image();
    image.addEventListener("load", () => {
      if (image.naturalWidth <= 0 || image.naturalHeight <= 0) {
        resolve(null);
        return;
      }

      resolve({
        width: image.naturalWidth,
        height: image.naturalHeight,
      });
    });
    image.addEventListener("error", () => resolve(null));
    image.src = source;
  });
}

let importedImageCounter = 0;

function importedImageId(fileName: string): string {
  importedImageCounter += 1;
  const stem = fileName
    .replace(/\.[^.]+$/, "")
    .replace(/[^a-z0-9]+/gi, "-")
    .replace(/^-+|-+$/g, "")
    .toLowerCase();
  return `imported-image-${stem || "local"}-${importedImageCounter}`;
}

function formatBytes(value: number): string {
  return `${Math.round(value / 1024 / 1024)} MB`;
}

function writeImportStatus(statusId: string, message: string) {
  const target = window.document.getElementById(statusId);
  if (target) {
    target.textContent = message;
  }
}
