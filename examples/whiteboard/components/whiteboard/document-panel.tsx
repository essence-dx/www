import { whiteboardActions } from "../../lib/stores/whiteboard-store";
import { frameChildren, frameIdForElement, isFrameElement } from "../../lib/whiteboard/frames";
import type {
  WhiteboardDocument,
  WhiteboardElement,
} from "../../lib/whiteboard/model";
import { DxIcon } from "./toolbar";

export type DocumentPanelProps = {
  document: WhiteboardDocument;
  selectedElement: WhiteboardElement | null;
};

export function DocumentPanel({ document, selectedElement }: DocumentPanelProps) {
  const hasSelection = document.selection.length > 0;
  const gridVisible = Boolean(document.metadata?.gridVisible);
  const snapToGrid = Boolean(document.metadata?.snapToGrid);
  const gridSize = typeof document.metadata?.gridSize === "number" ? document.metadata.gridSize : 24;
  const selectedLocked = Boolean(selectedElement?.locked);
  const selectedHidden = Boolean(selectedElement?.hidden);
  const selectedIsFrame = Boolean(selectedElement && isFrameElement(selectedElement));
  const selectedFrameId = selectedElement ? frameIdForElement(selectedElement) : null;
  const selectedFrame = selectedFrameId
    ? document.elements.find((element) => element.id === selectedFrameId)
    : null;
  const assignableSelectionIds = selectedElement && selectedIsFrame
    ? document.selection.filter((id) => {
        const element = document.elements.find((item) => item.id === id);
        return Boolean(element && element.id !== selectedElement.id && !isFrameElement(element));
      })
    : [];

  return (
    <section className="wb-panel wb-document-panel" aria-labelledby="wb-document-title">
      <div className="wb-panel-heading">
        <div>
          <p className="wb-eyebrow">Board</p>
          <h2 id="wb-document-title">Document</h2>
        </div>
        <span className="wb-panel-badge">{document.elements.length} items</span>
      </div>

      <label className="wb-input-row">
        <span>Name</span>
        <input
          onInput={(event) => whiteboardActions.dispatch({
            type: "document.rename",
            name: event.currentTarget.value,
          })}
          value={document.name}
        />
      </label>

      <div className="wb-toggle-grid">
        <button
          aria-pressed={gridVisible}
          className="wb-toggle-button"
          data-active={gridVisible}
          data-whiteboard-command="grid.set"
          onClick={() => whiteboardActions.setGridSettings({ gridVisible: !gridVisible })}
          type="button"
        >
          <DxIcon name="grid" />
          <span>Grid</span>
        </button>
        <button
          aria-pressed={snapToGrid}
          className="wb-toggle-button"
          data-active={snapToGrid}
          data-whiteboard-command="grid.set"
          onClick={() => whiteboardActions.setGridSettings({ snapToGrid: !snapToGrid })}
          type="button"
        >
          <DxIcon name="snap" />
          <span>Snap</span>
        </button>
      </div>

      <label className="wb-number-field">
        <span>Grid</span>
        <input
          min="4"
          onInput={(event) => whiteboardActions.setGridSettings({ gridSize: Number(event.currentTarget.value) })}
          step="4"
          type="number"
          value={gridSize}
        />
      </label>

      <div className="wb-layer-actions" role="toolbar" aria-label="Layer actions">
        <button
          className="wb-action-button"
          data-whiteboard-command="element.duplicate"
          disabled={!hasSelection}
          onClick={() => whiteboardActions.duplicateSelection()}
          type="button"
        >
          <DxIcon name="copy" />
          <span>Duplicate</span>
        </button>
        <button
          className="wb-action-button"
          data-whiteboard-command={selectedLocked ? "element.unlock" : "element.lock"}
          disabled={!hasSelection}
          onClick={() => whiteboardActions.setSelectionLocked(!selectedLocked)}
          type="button"
        >
          <DxIcon name={selectedLocked ? "unlock" : "lock"} />
          <span>{selectedLocked ? "Unlock" : "Lock"}</span>
        </button>
        <button
          className="wb-action-button"
          data-whiteboard-command={selectedHidden ? "element.show" : "element.hide"}
          disabled={!hasSelection}
          onClick={() => whiteboardActions.setSelectionHidden(!selectedHidden)}
          type="button"
        >
          <DxIcon name={selectedHidden ? "eye" : "eye-off"} />
          <span>{selectedHidden ? "Show" : "Hide"}</span>
        </button>
        <button
          className="wb-action-button"
          data-whiteboard-command="element.reorder"
          disabled={!hasSelection}
          onClick={() => whiteboardActions.dispatch({
            type: "element.reorder",
            ids: document.selection,
            intent: "front",
          })}
          type="button"
        >
          <DxIcon name="arrow" />
          <span>Front</span>
        </button>
        <button
          className="wb-action-button"
          data-whiteboard-command="element.reorder"
          disabled={!hasSelection}
          onClick={() => whiteboardActions.dispatch({
            type: "element.reorder",
            ids: document.selection,
            intent: "back",
          })}
          type="button"
        >
          <DxIcon name="line" />
          <span>Back</span>
        </button>
        <button
          className="wb-action-button wb-danger-button"
          data-whiteboard-command="element.remove"
          disabled={!hasSelection}
          onClick={() => whiteboardActions.dispatch({
            type: "element.remove",
            ids: document.selection,
          })}
          type="button"
        >
          <DxIcon name="eraser" />
          <span>Delete</span>
        </button>
      </div>

      <div
        className="wb-frame-controls"
        data-whiteboard-frame-controls="command-backed"
        data-whiteboard-frame-id={selectedFrameId ?? ""}
        data-whiteboard-frame-picker="selection"
        data-whiteboard-frame-storage="metadata.frameId"
        data-whiteboard-selection-size={document.selection.length}
      >
        <p className="wb-frame-note">
          {selectedIsFrame
            ? `${frameChildren(document, selectedElement?.id ?? "").length} framed items`
            : selectedFrame
              ? `Inside ${selectedFrame.name ?? selectedFrame.id}`
              : "Select a frame and elements to assign membership"}
        </p>
        <div className="wb-layer-actions" role="toolbar" aria-label="Frame actions">
          <button
            className="wb-action-button"
            data-whiteboard-command="frame.assign"
            disabled={!selectedElement || !selectedIsFrame || assignableSelectionIds.length === 0}
            onClick={() => assignSelectedElementsToFrame(selectedElement, assignableSelectionIds)}
            type="button"
          >
            <DxIcon name="rectangle" />
            <span>Assign</span>
          </button>
          <button
            className="wb-action-button"
            data-whiteboard-command="frame.clear"
            disabled={!selectedElement || !selectedFrameId}
            onClick={() => clearSelectedFrameMembership(document.selection, selectedElement)}
            type="button"
          >
            <DxIcon name="eraser" />
            <span>Clear</span>
          </button>
        </div>
      </div>

      <div className="wb-layer-list" role="listbox" aria-label="Board layers">
        {document.elements.map((element, index) => {
          const frameId = frameIdForElement(element);
          const frameChildCount = isFrameElement(element) ? frameChildren(document, element.id).length : 0;

          return (
            <button
              aria-selected={selectedElement?.id === element.id}
              className="wb-layer-row"
              data-active={selectedElement?.id === element.id}
              data-hidden={element.hidden}
              data-locked={element.locked}
              data-whiteboard-element-id={element.id}
              data-whiteboard-element-role={element.role ?? element.type}
              data-whiteboard-frame-child-count={frameChildCount}
              data-whiteboard-frame-container={isFrameElement(element)}
              data-whiteboard-frame-id={frameId ?? "none"}
              data-whiteboard-frame-member={Boolean(frameId)}
              data-whiteboard-role={element.role ?? "none"}
              key={element.id}
              onClick={() => whiteboardActions.selectElements([element.id])}
              role="option"
              type="button"
            >
              <span className="wb-layer-mark" data-element-type={element.type} />
              <span>{element.name ?? elementLabel(element, index)}</span>
              <small>
                {layerStatus(element, frameId)}
              </small>
            </button>
          );
        })}
      </div>
    </section>
  );
}

function assignSelectedElementsToFrame(
  selectedElement: WhiteboardElement | null,
  ids: readonly WhiteboardElement["id"][],
) {
  if (!selectedElement || !isFrameElement(selectedElement) || ids.length === 0) return undefined;
  return whiteboardActions.dispatch({
    type: "frame.assign",
    frameId: selectedElement.id,
    ids,
  });
}

function clearSelectedFrameMembership(
  selectedIds: readonly WhiteboardElement["id"][],
  selectedElement: WhiteboardElement | null,
) {
  if (!selectedElement) return undefined;
  return whiteboardActions.dispatch({
    type: "frame.clear",
    ids: selectedIds.length > 0 ? selectedIds : [selectedElement.id],
  });
}

function layerStatus(element: WhiteboardElement, frameId: ReturnType<typeof frameIdForElement>) {
  if (element.locked) return "locked";
  if (element.hidden) return "hidden";
  if (isFrameElement(element)) return "frame";
  return frameId ? "framed" : element.type;
}

function elementLabel(element: WhiteboardElement, index: number) {
  if (element.type === "text" && element.text) {
    return element.text;
  }

  return `${index + 1}. ${element.type}`;
}
