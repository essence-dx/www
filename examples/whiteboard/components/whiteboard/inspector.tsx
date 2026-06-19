import { whiteboardActions } from "../../lib/stores/whiteboard-store";
import { frameChildren, frameIdForElement, isFrameElement } from "../../lib/whiteboard/frames";
import { isEmbeddedImageSource } from "../../lib/whiteboard/image-source";
import type {
  WhiteboardDocument,
  WhiteboardElement,
  WhiteboardElementPatch,
  WhiteboardStyle,
} from "../../lib/whiteboard/model";
import { ConnectorInfo } from "./connector-info";
import { FILL_SWATCHES, STROKE_SWATCHES } from "./inspector-options";
import { NumberField } from "./number-field";
import { TextControls } from "./text-controls";
import { DxIcon } from "./toolbar";

export type InspectorProps = {
  document: WhiteboardDocument;
  selectedElement: WhiteboardElement | null;
  style: WhiteboardStyle;
};

export function Inspector({ document, selectedElement, style }: InspectorProps) {
  const selectedIsFrame = Boolean(selectedElement && isFrameElement(selectedElement));
  const selectedFrameId = selectedElement ? frameIdForElement(selectedElement) : null;
  const selectedFrame = selectedFrameId
    ? document.elements.find((element) => element.id === selectedFrameId)
    : null;
  const selectedFrameChildCount = selectedElement && selectedIsFrame
    ? frameChildren(document, selectedElement.id).length
    : 0;
  const assignableSelectionIds = selectedElement && selectedIsFrame
    ? document.selection.filter((id) => {
        const element = document.elements.find((item) => item.id === id);
        return Boolean(element && element.id !== selectedElement.id && !isFrameElement(element));
      })
    : [];

  return (
    <section className="wb-panel wb-inspector" aria-labelledby="wb-inspector-title">
      <div className="wb-panel-heading">
        <div>
          <p className="wb-eyebrow">Style</p>
          <h2 id="wb-inspector-title">Inspector</h2>
        </div>
        <span className="wb-panel-badge">
          {selectedElement ? selectedElement.type : "Defaults"}
        </span>
      </div>

      <fieldset className="wb-fieldset">
        <legend>Stroke</legend>
        <div className="wb-swatch-grid">
          {STROKE_SWATCHES.map((swatch) => (
            <button
              aria-label={`Stroke ${swatch.label}`}
              aria-pressed={style.stroke === swatch.value}
              className="wb-swatch"
              data-active={style.stroke === swatch.value}
              key={swatch.value}
              onClick={() => applyStyle(selectedElement, { stroke: swatch.value })}
              style={`--swatch-color:${swatch.value}`}
              title={swatch.label}
              type="button"
            />
          ))}
        </div>
      </fieldset>

      <fieldset className="wb-fieldset">
        <legend>Fill</legend>
        <div className="wb-swatch-grid">
          {FILL_SWATCHES.map((swatch) => (
            <button
              aria-label={`Fill ${swatch.label}`}
              aria-pressed={style.fill === swatch.value}
              className="wb-swatch"
              data-active={style.fill === swatch.value}
              data-swatch={swatch.value === "transparent" ? "transparent" : undefined}
              key={swatch.value}
              onClick={() => applyStyle(selectedElement, { fill: swatch.value })}
              style={swatch.value === "transparent" ? undefined : `--swatch-color:${swatch.value}`}
              title={swatch.label}
              type="button"
            />
          ))}
        </div>
      </fieldset>

      <label className="wb-control-row">
        <span>Width</span>
        <input
          max="16"
          min="1"
          onInput={(event) => applyStyle(selectedElement, { strokeWidth: Number(event.currentTarget.value) })}
          step="1"
          type="range"
          value={style.strokeWidth}
        />
        <output>{style.strokeWidth}px</output>
      </label>

      <label className="wb-control-row">
        <span>Opacity</span>
        <input
          max="1"
          min="0.1"
          onInput={(event) => applyStyle(selectedElement, { opacity: Number(event.currentTarget.value) })}
          step="0.05"
          type="range"
          value={style.opacity}
        />
        <output>{Math.round(style.opacity * 100)}%</output>
      </label>

      <label className="wb-control-row">
        <span>Font</span>
        <input
          max="48"
          min="12"
          onInput={(event) => applyStyle(selectedElement, { fontSize: Number(event.currentTarget.value) })}
          step="1"
          type="range"
          value={style.fontSize}
        />
        <output>{style.fontSize}px</output>
      </label>

      <label className="wb-select-row">
        <span>Stroke pattern</span>
        <select
          onInput={(event) =>
            applyStyle(selectedElement, {
              strokeStyle: event.currentTarget.value as WhiteboardStyle["strokeStyle"],
            })
          }
          value={style.strokeStyle}
        >
          <option value="solid">Solid</option>
          <option value="dashed">Dashed</option>
          <option value="dotted">Dotted</option>
        </select>
      </label>

      <label className="wb-select-row">
        <span>Line cap</span>
        <select
          onInput={(event) =>
            applyStyle(selectedElement, {
              lineCap: event.currentTarget.value as WhiteboardStyle["lineCap"],
            })
          }
          value={style.lineCap}
        >
          <option value="round">Round</option>
          <option value="square">Square</option>
          <option value="butt">Butt</option>
        </select>
      </label>

      <label className="wb-color-field">
        <span>Text color</span>
        <input
          aria-label="Text color"
          onInput={(event) => applyStyle(selectedElement, { textColor: event.currentTarget.value })}
          type="color"
          value={normalizeColorInput(style.textColor)}
        />
        <output>{style.textColor}</output>
      </label>

      <fieldset className="wb-fieldset" disabled={!selectedElement}>
        <legend>Element</legend>
        <label className="wb-input-row">
          <span>Name</span>
          <input
            onInput={(event) => updateSelectedElement(selectedElement, { name: event.currentTarget.value })}
            value={selectedElement?.name ?? ""}
          />
        </label>

        {selectedElement && "x" in selectedElement ? (
          <div className="wb-control-grid wb-control-grid-compact">
            <NumberField
              label="X"
              onInput={(value) => updateSelectedElement(selectedElement, { x: value })}
              value={selectedElement.x}
            />
            <NumberField
              label="Y"
              onInput={(value) => updateSelectedElement(selectedElement, { y: value })}
              value={selectedElement.y}
            />
            <NumberField
              label="W"
              min={1}
              onInput={(value) => updateSelectedElement(selectedElement, { width: value })}
              value={selectedElement.width}
            />
            <NumberField
              label="H"
              min={1}
              onInput={(value) => updateSelectedElement(selectedElement, { height: value })}
              value={selectedElement.height}
            />
            <NumberField
              label="Rotate"
              onInput={(value) => updateSelectedElement(selectedElement, { rotation: value })}
              value={selectedElement.rotation}
            />
          </div>
        ) : null}

        {selectedElement?.type === "text" ? <TextControls element={selectedElement} /> : null}

        <fieldset
          className="wb-fieldset wb-frame-controls"
          data-whiteboard-frame-controls="command-backed"
          disabled={!selectedElement}
        >
          <legend>Frame</legend>
          <p className="wb-frame-note">
            {selectedIsFrame
              ? `${selectedFrameChildCount} framed item${selectedFrameChildCount === 1 ? "" : "s"}`
              : selectedFrame
                ? `Inside ${selectedFrame.name ?? selectedFrame.id}`
                : "No frame membership"}
          </p>
          <div className="wb-layer-actions">
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
        </fieldset>

        {selectedElement?.type === "image" ? (
          <fieldset className="wb-fieldset wb-image-controls" data-whiteboard-image-controls="source-owned">
            <legend>Image</legend>
            <label className="wb-input-row">
              <span>Source</span>
              <input
                aria-label="Image source"
                data-whiteboard-image-policy="embedded-data-url-only"
                onInput={(event) => updateImageSource(selectedElement, event.currentTarget.value)}
                value={selectedElement.src}
              />
            </label>
            <label className="wb-input-row">
              <span>Alt text</span>
              <input
                aria-label="Image alt text"
                onInput={(event) => updateSelectedElement(selectedElement, { alt: event.currentTarget.value })}
                value={selectedElement.alt}
              />
            </label>
          </fieldset>
        ) : null}

        {selectedElement?.type === "line" || selectedElement?.type === "arrow" ? (
          <ConnectorInfo element={selectedElement} />
        ) : null}
      </fieldset>

      <div className="wb-inspector-preview" aria-label="Current style preview">
        <DxIcon name="pen" />
        <svg viewBox="0 0 220 84" aria-hidden="true">
          <rect
            className="wb-preview-shape"
            data-dash={style.strokeStyle}
            height="48"
            rx="8"
            style={`--wb-preview-fill:${style.fill};--wb-preview-stroke:${style.stroke};--wb-preview-opacity:${style.opacity};--wb-preview-width:${style.strokeWidth}`}
            width="148"
            x="34"
            y="18"
          />
          <text className="wb-preview-text" x="48" y="49">
            Aa
          </text>
        </svg>
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

function applyStyle(
  selectedElement: WhiteboardElement | null,
  style: Partial<WhiteboardStyle>,
) {
  if (selectedElement) {
    return whiteboardActions.dispatch({
      type: "element.update",
      id: selectedElement.id,
      patch: { style },
    });
  }

  return whiteboardActions.setStyle(style);
}

function updateSelectedElement(
  selectedElement: WhiteboardElement | null,
  patch: WhiteboardElementPatch,
) {
  if (!selectedElement) return undefined;
  return whiteboardActions.dispatch({
    type: "element.update",
    id: selectedElement.id,
    patch,
  });
}

function updateImageSource(
  selectedElement: Extract<WhiteboardElement, { type: "image" }>,
  source: string,
) {
  if (!isEmbeddedImageSource(source)) {
    return undefined;
  }

  return updateSelectedElement(selectedElement, { src: source });
}

function normalizeColorInput(value: string): string {
  return /^#[0-9a-f]{6}$/i.test(value) ? value : "hsl(var(--wb-swatch-ink))";
}
