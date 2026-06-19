import { exportDxDraw } from "../../lib/whiteboard/export/dxdraw";
import { createWhiteboardExportMetadata } from "../../lib/whiteboard/export/metadata";
import { createWhiteboardPngExport } from "../../lib/whiteboard/export/png";
import { exportWhiteboardToSvg } from "../../lib/whiteboard/export/svg";
import type { WhiteboardDocument } from "../../lib/whiteboard/model";
import { DxIcon } from "./toolbar";

export type ExportPanelProps = {
  document: WhiteboardDocument;
};

export function ExportPanel({ document }: ExportPanelProps) {
  const dxdraw = exportDxDraw(document, {
    exportedAt: "2026-06-02T00:00:00.000Z",
  });
  const svg = exportWhiteboardToSvg(document, {
    width: 1200,
    height: 720,
    background: "white",
  });
  const pngPlan = createWhiteboardPngExport(document, {
    width: 1200,
    height: 720,
    background: "white",
    fileName: `${document.name}.png`,
  });
  const metadata = createWhiteboardExportMetadata(document, {
    format: "dxdraw",
    exportedAt: "2026-06-02T00:00:00.000Z",
  });

  return (
    <section className="wb-panel wb-export-panel" aria-labelledby="wb-export-title">
      <div className="wb-panel-heading">
        <div>
          <p className="wb-eyebrow">Files</p>
          <h2 id="wb-export-title">Export</h2>
        </div>
        <span className="wb-panel-badge">.dxdraw</span>
      </div>

      <div className="wb-export-actions">
        <button
          className="wb-primary-button"
          onClick={() => downloadText(`${document.name}.dxdraw`, dxdraw, "application/json")}
          type="button"
        >
          <DxIcon name="download" />
          <span>DXDraw</span>
        </button>
        <div className="wb-export-format-grid">
          <button
            className="wb-action-button"
            onClick={() => downloadText(`${document.name}.svg`, svg, "image/svg+xml")}
            type="button"
          >
            <DxIcon name="download" />
            <span>SVG</span>
          </button>
          <button
            className="wb-action-button"
            onClick={() => navigator.clipboard?.writeText(pngPlan.svg)}
            type="button"
          >
            <DxIcon name="download" />
            <span>PNG plan</span>
          </button>
        </div>
      </div>

      <p
        className="wb-export-note"
        data-dx-whiteboard-export="dxdraw svg png-plan"
        role="status"
      >
        Export receipt covers {metadata.document.elementCount} elements and revision {metadata.document.revision}.
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
