import {
  createWhiteboardShareReceipt,
  exportWhiteboardShareReceipt,
} from "../../lib/whiteboard/export/share-receipt";
import type { WhiteboardDocument } from "../../lib/whiteboard/model";
import { DxIcon } from "./toolbar";

export type SharePanelProps = {
  readonly document: WhiteboardDocument;
};

export function SharePanel({ document }: SharePanelProps) {
  const shareReceipt = createWhiteboardShareReceipt(document, {
    createdAt: "2026-06-03T00:00:00.000Z",
    target: "local-preview",
  });
  const receipt = exportWhiteboardShareReceipt(document, {
    createdAt: "2026-06-03T00:00:00.000Z",
    target: "local-preview",
  });

  return (
    <section className="wb-panel wb-share-panel" aria-labelledby="wb-share-title">
      <div className="wb-panel-heading">
        <div>
          <p className="wb-eyebrow">Share</p>
          <h2 id="wb-share-title">Receipt</h2>
        </div>
        <span className="wb-panel-badge">{shareReceipt.status}</span>
      </div>

      <div className="wb-share-proof" data-dx-whiteboard-share="receipt-backed">
        <span>{shareReceipt.metadata.document.elementCount} elements</span>
        <span>Revision {shareReceipt.metadata.document.revision}</span>
      </div>

      <div className="wb-export-format-grid">
        <button
          className="wb-action-button"
          onClick={() => navigator.clipboard?.writeText(receipt)}
          type="button"
        >
          <DxIcon name="copy" />
          <span>Copy</span>
        </button>
        <button
          className="wb-action-button"
          onClick={() => downloadText(`${document.name}.share-receipt.json`, receipt, "application/json")}
          type="button"
        >
          <DxIcon name="download" />
          <span>Receipt</span>
        </button>
      </div>

      <p className="wb-export-note">
        Local receipt only. Live links need an explicit adapter before WWW claims remote sharing.
      </p>
    </section>
  );
}

function downloadText(filename: string, content: string, type: string) {
  const blob = new Blob([content], { type });
  const url = URL.createObjectURL(blob);
  const link = window.document.createElement("a");
  link.href = url;
  link.download = filename;
  link.click();
  URL.revokeObjectURL(url);
}
