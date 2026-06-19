import type {
  WhiteboardDocument,
  WhiteboardElement,
  WhiteboardTool,
} from "../../lib/whiteboard/model";
import { getDocumentContentBounds, getDocumentSelectionBounds } from "../../lib/whiteboard/scene";
import { DxIcon } from "./toolbar";

export type StatusBarProps = {
  activeTool: WhiteboardTool;
  document: WhiteboardDocument;
  selectedElement: WhiteboardElement | null;
};

export function StatusBar({ activeTool, document, selectedElement }: StatusBarProps) {
  const selectionBounds = getDocumentSelectionBounds(document);
  const contentBounds = getDocumentContentBounds(document);
  const fitTarget = selectionBounds ? "selection" : contentBounds ? "content" : "empty";

  return (
    <footer
      className="wb-status-bar"
      aria-label="Whiteboard status"
      aria-live="polite"
      data-whiteboard-selection-count={String(document.selection.length)}
      data-whiteboard-fit-target={fitTarget}
      data-whiteboard-content-bounds={formatBoundsSize(contentBounds)}
    >
      <div className="wb-status-cluster">
        <span className="wb-status-item">
          <DxIcon name="select" />
          {activeTool}
        </span>
        <span className="wb-status-item">{document.elements.length} elements</span>
        <span className="wb-status-item">
          {selectedElement ? selectedElement.type : "Nothing selected"}
        </span>
        <span className="wb-status-item">
          {document.metadata?.gridVisible ? "Grid" : "No grid"}
        </span>
        <span className="wb-status-item">
          {document.metadata?.snapToGrid ? "Snap" : "Free"}
        </span>
        <span className="wb-status-item" data-whiteboard-keyboard-status="nudge-duplicate-group">
          Key commands
        </span>
      </div>

      <div className="wb-zoom-control" data-dx-whiteboard-zoom={String(document.viewport.zoom)}>
        <DxIcon name="zoom-out" />
        <span className="wb-zoom-meter">
          {Math.round(document.viewport.zoom * 100)}%
        </span>
        <DxIcon name="zoom-in" />
      </div>

      <div className="wb-status-cluster wb-status-cluster-right">
        {selectedElement ? (
          <span className="wb-status-item">
            {selectedElement.locked ? "Locked" : selectedElement.hidden ? "Hidden" : "Editable"}
          </span>
        ) : null}
        <span className="wb-status-item">Rev {String(document.metadata?.revision ?? 0)}</span>
        <span className="wb-status-item">
          Updated {new Date(document.updatedAt).toLocaleTimeString([], {
            hour: "2-digit",
            minute: "2-digit",
          })}
        </span>
      </div>
    </footer>
  );
}

function formatBoundsSize(bounds: ReturnType<typeof getDocumentContentBounds>): string {
  return bounds ? `${Math.round(bounds.width)}x${Math.round(bounds.height)}` : "empty";
}
