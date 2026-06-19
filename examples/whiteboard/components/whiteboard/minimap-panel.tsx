import { whiteboardActions } from "../../lib/stores/whiteboard-store";
import {
  createWhiteboardMinimapModel,
  minimapPointToViewport,
  WHITEBOARD_MINIMAP_DEFAULT_SIZE,
} from "../../lib/whiteboard/minimap";
import type { WhiteboardDocument, WhiteboardSize } from "../../lib/whiteboard/model";

export type MinimapPanelProps = {
  readonly document: WhiteboardDocument;
  readonly stageSize: WhiteboardSize;
};

export function MinimapPanel({ document, stageSize }: MinimapPanelProps) {
  const minimap = createWhiteboardMinimapModel(document, {
    size: WHITEBOARD_MINIMAP_DEFAULT_SIZE,
    stageSize,
  });

  return (
    <section
      className="wb-panel wb-minimap-panel"
      aria-labelledby="wb-minimap-title"
      data-whiteboard-minimap="model-backed"
      data-whiteboard-minimap-action="pan-to-point"
      data-whiteboard-minimap-bounds={formatRect(minimap.documentBounds)}
      data-whiteboard-minimap-empty={minimap.empty}
      data-whiteboard-minimap-elements={minimap.elements.length}
      data-whiteboard-minimap-scale={String(minimap.scale)}
      data-whiteboard-minimap-stage-size={`${stageSize.width}x${stageSize.height}`}
    >
      <div className="wb-panel-heading">
        <div>
          <p className="wb-eyebrow">Navigate</p>
          <h2 id="wb-minimap-title">Minimap</h2>
        </div>
        <span className="wb-panel-badge">{minimap.elements.length} visible</span>
      </div>

      <button
        aria-label="Pan whiteboard viewport from minimap"
        className="wb-minimap-button"
        data-whiteboard-command="viewport.set"
        onClick={(event) => navigateFromMinimap(event, minimap)}
        type="button"
      >
        <svg
          aria-hidden="true"
          className="wb-minimap-svg"
          data-whiteboard-minimap-surface="svg"
          focusable="false"
          viewBox={`0 0 ${minimap.size.width} ${minimap.size.height}`}
        >
          <rect
            className="wb-minimap-background"
            height={minimap.size.height}
            rx="8"
            width={minimap.size.width}
            x="0"
            y="0"
          />
          {minimap.elements.map((element) => (
            <rect
              className="wb-minimap-element"
              data-selected={element.selected}
              data-whiteboard-element-id={element.id}
              data-whiteboard-element-type={element.type}
              data-whiteboard-role={element.role ?? "none"}
              height={Math.max(1, element.rect.height)}
              key={element.id}
              rx="2"
              width={Math.max(1, element.rect.width)}
              x={element.rect.x}
              y={element.rect.y}
            />
          ))}
          {minimap.viewportRect ? (
            <rect
              className="wb-minimap-viewport"
              data-whiteboard-minimap-viewport="viewport-state"
              height={Math.max(1, minimap.viewportRect.height)}
              rx="4"
              width={Math.max(1, minimap.viewportRect.width)}
              x={minimap.viewportRect.x}
              y={minimap.viewportRect.y}
            />
          ) : null}
        </svg>
      </button>
    </section>
  );
}

function navigateFromMinimap(
  event: {
    readonly currentTarget: HTMLElement;
    readonly clientX: number;
    readonly clientY: number;
  },
  minimap: ReturnType<typeof createWhiteboardMinimapModel>,
) {
  const bounds = event.currentTarget.getBoundingClientRect();
  const nextViewport = minimapPointToViewport(minimap, {
    x: ((event.clientX - bounds.left) / bounds.width) * minimap.size.width,
    y: ((event.clientY - bounds.top) / bounds.height) * minimap.size.height,
  });

  return whiteboardActions.setViewport(nextViewport);
}

function formatRect(rect: {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
}): string {
  return `${Math.round(rect.x)},${Math.round(rect.y)},${Math.round(rect.width)},${Math.round(rect.height)}`;
}
