import { whiteboardActions } from "../../lib/stores/whiteboard-store";
import type { WhiteboardDocument, WhiteboardRect, WhiteboardSize } from "../../lib/whiteboard/model";
import {
  createWhiteboardMeasurementModel,
  measurementCommandsForFocus,
  type WhiteboardMeasurementCounts,
  type WhiteboardMeasurementModel,
} from "../../lib/whiteboard/measurements";
import { DxIcon } from "./toolbar";

export type MeasurementPanelProps = {
  readonly document: WhiteboardDocument;
  readonly stageSize: WhiteboardSize;
};

export function MeasurementPanel({ document, stageSize }: MeasurementPanelProps) {
  const measurements = createWhiteboardMeasurementModel(document, stageSize);

  return (
    <section
      className="wb-panel wb-measurement-panel"
      aria-labelledby="wb-measurement-title"
      data-whiteboard-measurements="source-owned"
      data-whiteboard-measurement-can-focus={measurements.canFocus}
      data-whiteboard-measurement-item-count={measurements.counts.items}
      data-whiteboard-measurement-subject={measurements.subject}
    >
      <div className="wb-panel-heading">
        <div>
          <p className="wb-eyebrow">Measure</p>
          <h2 id="wb-measurement-title">{measurements.label}</h2>
        </div>
        <span className="wb-panel-badge">
          {measurements.counts.items} items
        </span>
      </div>

      {measurements.bounds ? (
        <div className="wb-measurement-grid" aria-label="Measured bounds">
          {measurementFields(measurements.bounds).map((field) => (
            <span className="wb-measurement-field" key={field.label}>
              <small>{field.label}</small>
              <strong>{field.value}</strong>
            </span>
          ))}
        </div>
      ) : (
        <p className="wb-empty-note">Add an element to measure board bounds.</p>
      )}

      <div className="wb-measurement-chips" aria-label="Measured element counts">
        {countFields(measurements.counts).map((field) => (
          <span
            className="wb-measurement-chip"
            data-whiteboard-measurement-count={field.key}
            key={field.key}
          >
            <strong>{field.value}</strong>
            <small>{field.label}</small>
          </span>
        ))}
      </div>

      <button
        className="wb-action-button"
        data-whiteboard-command="viewport.set"
        disabled={!measurements.canFocus}
        onClick={() => focusMeasurement(measurements)}
        type="button"
      >
        <DxIcon name="fit" />
        <span>Focus measured area</span>
      </button>
    </section>
  );
}

function focusMeasurement(model: WhiteboardMeasurementModel) {
  const commands = measurementCommandsForFocus(model);
  return commands.length > 0 ? whiteboardActions.dispatchBatch(commands) : undefined;
}

function measurementFields(bounds: WhiteboardRect) {
  return [
    { label: "X", value: formatNumber(bounds.x) },
    { label: "Y", value: formatNumber(bounds.y) },
    { label: "W", value: formatNumber(bounds.width) },
    { label: "H", value: formatNumber(bounds.height) },
  ];
}

function countFields(counts: WhiteboardMeasurementCounts) {
  return [
    { key: "frames", label: "frames", value: counts.frames },
    { key: "framed", label: "framed", value: counts.framed },
    { key: "connectors", label: "connectors", value: counts.connectors },
    { key: "text", label: "text", value: counts.text },
    { key: "images", label: "images", value: counts.images },
    { key: "locked", label: "locked", value: counts.locked },
    { key: "hidden", label: "hidden", value: counts.hidden },
  ];
}

function formatNumber(value: number): string {
  return new Intl.NumberFormat("en-US", {
    maximumFractionDigits: 1,
    minimumFractionDigits: 0,
  }).format(value);
}
