import { getElementBounds } from "../../lib/whiteboard/geometry";
import { connectorRouteForElement } from "../../lib/whiteboard/connector-routes";
import { frameIdForElement, isFrameElement } from "../../lib/whiteboard/frames";
import type {
  WhiteboardDocument,
  WhiteboardElement,
  WhiteboardRect,
  WhiteboardViewport,
} from "../../lib/whiteboard/model";
import type { WhiteboardSelection } from "../../lib/whiteboard/render/model";

export type SvgStagePreviewProps = {
  readonly document: WhiteboardDocument;
  readonly selection: WhiteboardSelection;
  readonly viewport: WhiteboardViewport;
  readonly width: number;
  readonly height: number;
  readonly draft?: WhiteboardElement | null;
  readonly selectionArea?: WhiteboardRect | null;
};

export function SvgStagePreview({
  document,
  selection,
  viewport,
  width,
  height,
  draft = null,
  selectionArea = null,
}: SvgStagePreviewProps) {
  const selectedIds = new Set(selection.ids);

  return (
    <svg
      aria-hidden="true"
      className="whiteboard-svg-preview"
      data-grid={Boolean(document.metadata?.gridVisible)}
      data-snap={Boolean(document.metadata?.snapToGrid)}
      data-whiteboard-frame-membership="metadata-backed"
      data-whiteboard-svg-preview="source-owned"
      height={height}
      viewBox={`0 0 ${width} ${height}`}
      width={width}
    >
      <defs>
        <pattern id="wb-preview-grid" width="24" height="24" patternUnits="userSpaceOnUse">
          <path d="M 24 0 L 0 0 0 24" className="wb-preview-grid-line" />
        </pattern>
        <marker
          id="wb-preview-arrowhead"
          markerHeight="10"
          markerWidth="10"
          orient="auto"
          refX="8"
          refY="5"
          viewBox="0 0 10 10"
        >
          <path d="M 0 0 L 10 5 L 0 10 z" className="wb-preview-arrowhead" />
        </marker>
      </defs>
      <rect className="wb-preview-canvas" height={height} width={width} />
      <rect className="wb-preview-grid" height={height} width={width} />
      <g transform={`translate(${viewport.x} ${viewport.y}) scale(${viewport.zoom})`}>
        {document.elements.map((element) =>
          element.hidden ? null : <PreviewElement element={element} key={element.id} />,
        )}
        {draft && !draft.hidden ? (
          <g data-whiteboard-runtime-draft="input-runtime-ephemeral">
            <PreviewElement element={draft} />
          </g>
        ) : null}
        {selectionArea ? <SelectionAreaPreview bounds={selectionArea} /> : null}
        {document.elements.map((element) =>
          !element.hidden && selectedIds.has(element.id) ? (
            <SelectionOutline element={element} key={`selection-${element.id}`} />
          ) : null,
        )}
      </g>
    </svg>
  );
}

function SelectionAreaPreview({ bounds }: { readonly bounds: WhiteboardRect }) {
  return (
    <rect
      className="wb-preview-selection-area"
      data-whiteboard-selection-area-preview="input-runtime-ephemeral"
      height={bounds.height}
      width={bounds.width}
      x={bounds.x}
      y={bounds.y}
    />
  );
}

function PreviewElement({ element }: { readonly element: WhiteboardElement }) {
  const style = elementStyle(element);
  const frameId = frameIdForElement(element) ?? "none";

  return (
    <g
      data-whiteboard-element-id={element.id}
      data-whiteboard-element-role={element.role ?? element.type}
      data-whiteboard-frame-id={frameId}
      data-whiteboard-frame-member={frameId !== "none"}
      data-whiteboard-frame-container={isFrameElement(element)}
      data-whiteboard-role={isFrameElement(element) ? "frame" : element.role ?? "none"}
    >
      <PreviewElementShape element={element} style={style} />
    </g>
  );
}

function PreviewElementShape({
  element,
  style,
}: {
  readonly element: WhiteboardElement;
  readonly style: string;
}) {
  switch (element.type) {
    case "rectangle":
      return (
        <g>
          <rect
            className="wb-preview-element"
            height={element.height}
            rx={element.radius ?? 10}
            style={style}
            width={element.width}
            x={element.x}
            y={element.y}
          />
          <BoxLabel element={element} />
        </g>
      );
    case "ellipse":
      return (
        <g>
          <ellipse
            className="wb-preview-element"
            cx={element.x + element.width / 2}
            cy={element.y + element.height / 2}
            rx={Math.abs(element.width / 2)}
            ry={Math.abs(element.height / 2)}
            style={style}
          />
          <BoxLabel element={element} />
        </g>
      );
    case "diamond": {
      const points = [
        `${element.x + element.width / 2},${element.y}`,
        `${element.x + element.width},${element.y + element.height / 2}`,
        `${element.x + element.width / 2},${element.y + element.height}`,
        `${element.x},${element.y + element.height / 2}`,
      ].join(" ");
      return (
        <g>
          <polygon className="wb-preview-element" points={points} style={style} />
          <BoxLabel element={element} />
        </g>
      );
    }
    case "line":
    case "arrow":
      return (
        <polyline
          className="wb-preview-element"
          data-whiteboard-connector-route={connectorRouteForElement(element)}
          data-whiteboard-connector-type={element.type}
          data-whiteboard-element-id={element.id}
          data-whiteboard-end-anchor={element.endBinding?.anchor ?? "auto"}
          data-whiteboard-end-binding={element.endBinding?.elementId ?? "unbound"}
          data-whiteboard-start-anchor={element.startBinding?.anchor ?? "auto"}
          data-whiteboard-start-binding={element.startBinding?.elementId ?? "unbound"}
          fill="none"
          marker-end={element.endArrow === "triangle" ? "url(#wb-preview-arrowhead)" : undefined}
          marker-start={element.startArrow === "triangle" ? "url(#wb-preview-arrowhead)" : undefined}
          points={element.points.map((point) => `${point.x},${point.y}`).join(" ")}
          style={style}
        />
      );
    case "path":
    case "freehand":
      return (
        <polyline
          className="wb-preview-element"
          fill="none"
          points={element.points.map((point) => `${point.x},${point.y}`).join(" ")}
          style={style}
        />
      );
    case "text":
      return <BoxLabel element={element} text={element.text} />;
    case "image":
      return (
        <g>
          <image
            className="wb-preview-image"
            height={element.height}
            href={element.src}
            preserveAspectRatio="xMidYMid meet"
            width={element.width}
            x={element.x}
            y={element.y}
          />
          <rect
            className="wb-preview-element wb-preview-image-frame"
            height={element.height}
            style={style}
            width={element.width}
            x={element.x}
            y={element.y}
          />
          <BoxLabel element={element} text={element.alt} />
        </g>
      );
  }
}

function BoxLabel({
  element,
  text = element.name,
}: {
  readonly element: Extract<WhiteboardElement, { type: "rectangle" | "ellipse" | "diamond" | "text" | "image" }>;
  readonly text?: string;
}) {
  if (!text) return null;

  return (
    <text
      className="wb-preview-label"
      style={`--wb-preview-text:${element.style.textColor};--wb-preview-font:${element.style.fontSize}px;--wb-preview-family:${element.style.fontFamily}`}
      x={element.x + 18}
      y={element.y + Math.min(element.height - 18, Math.max(30, element.height / 2 + 6))}
    >
      {text}
    </text>
  );
}

function SelectionOutline({ element }: { readonly element: WhiteboardElement }) {
  const bounds = getElementBounds(element);

  return (
    <rect
      className="wb-preview-selection"
      height={bounds.height}
      width={bounds.width}
      x={bounds.x}
      y={bounds.y}
    />
  );
}

function elementStyle(element: WhiteboardElement): string {
  const dash =
    element.style.strokeStyle === "dashed"
      ? "10 8"
      : element.style.strokeStyle === "dotted"
        ? "2 6"
        : "none";

  return [
    `--wb-preview-fill:${element.style.fill}`,
    `--wb-preview-stroke:${element.style.stroke}`,
    `--wb-preview-width:${element.style.strokeWidth}`,
    `--wb-preview-opacity:${element.style.opacity}`,
    `--wb-preview-dash:${dash}`,
  ].join(";");
}
