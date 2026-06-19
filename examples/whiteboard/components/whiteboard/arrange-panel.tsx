import { whiteboardActions } from "../../lib/stores/whiteboard-store";
import type { WhiteboardAlignment, WhiteboardDistribution } from "../../lib/whiteboard/arrange";
import type { WhiteboardDocument } from "../../lib/whiteboard/model";
import { DxIcon } from "./toolbar";

export type ArrangePanelProps = {
  readonly document: WhiteboardDocument;
};

const ALIGNMENTS: readonly { readonly id: WhiteboardAlignment; readonly label: string }[] = [
  { id: "left", label: "Left" },
  { id: "center", label: "Center" },
  { id: "right", label: "Right" },
  { id: "top", label: "Top" },
  { id: "middle", label: "Middle" },
  { id: "bottom", label: "Bottom" },
];

const DISTRIBUTIONS: readonly { readonly id: WhiteboardDistribution; readonly label: string }[] = [
  { id: "horizontal", label: "Horizontal" },
  { id: "vertical", label: "Vertical" },
];

export function ArrangePanel({ document }: ArrangePanelProps) {
  const selectedCount = document.selection.length;
  const selectedGroupId = selectedGroupIdFor(document);

  return (
    <section className="wb-panel wb-arrange-panel" aria-labelledby="wb-arrange-title">
      <div className="wb-panel-heading">
        <div>
          <p className="wb-eyebrow">Layout</p>
          <h2 id="wb-arrange-title">Arrange</h2>
        </div>
        <span className="wb-panel-badge">{selectedCount} selected</span>
      </div>

      <div className="wb-control-grid" role="toolbar" aria-label="Alignment controls">
        {ALIGNMENTS.map((alignment) => (
          <button
            className="wb-action-button"
            data-whiteboard-command="element.align"
            data-whiteboard-alignment={alignment.id}
            disabled={selectedCount < 2}
            key={alignment.id}
            onClick={() => whiteboardActions.alignSelection(alignment.id)}
            type="button"
          >
            <DxIcon name="align" />
            <span>{alignment.label}</span>
          </button>
        ))}
      </div>

      <div className="wb-control-grid" role="toolbar" aria-label="Distribution controls">
        {DISTRIBUTIONS.map((distribution) => (
          <button
            className="wb-action-button"
            data-whiteboard-command="element.distribute"
            data-whiteboard-distribution={distribution.id}
            disabled={selectedCount < 3}
            key={distribution.id}
            onClick={() => whiteboardActions.distributeSelection(distribution.id)}
            type="button"
          >
            <DxIcon name="align" />
            <span>{distribution.label}</span>
          </button>
        ))}
      </div>

      <div
        className="wb-control-grid wb-grouping-controls"
        data-whiteboard-group-controls="command-backed"
        role="toolbar"
        aria-label="Grouping controls"
      >
        <button
          className="wb-action-button"
          data-whiteboard-command="group.create"
          data-whiteboard-selection-size={selectedCount}
          disabled={selectedCount < 2}
          onClick={() => whiteboardActions.groupSelection()}
          type="button"
        >
          <DxIcon name="align" />
          <span>Group</span>
        </button>
        <button
          className="wb-action-button"
          data-whiteboard-command="group.remove"
          data-whiteboard-group-id={selectedGroupId ?? ""}
          disabled={!selectedGroupId}
          onClick={() => whiteboardActions.ungroupSelection()}
          type="button"
        >
          <DxIcon name="align" />
          <span>Ungroup</span>
        </button>
      </div>
    </section>
  );
}

function selectedGroupIdFor(document: WhiteboardDocument): string | null {
  const selectedIds = new Set(document.selection);
  const selectedGroups = new Set(
    document.elements
      .filter((element) => selectedIds.has(element.id) && element.groupId)
      .map((element) => element.groupId),
  );

  return selectedGroups.size === 1 ? [...selectedGroups][0] ?? null : null;
}
