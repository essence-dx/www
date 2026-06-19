import { elementsById, selectionBounds, selectionHandleRects } from "./geometry";
import type {
  WhiteboardDocument,
  WhiteboardElement,
  WhiteboardBounds,
  WhiteboardSelection,
  WhiteboardSize,
  WhiteboardViewport,
} from "./model";

export type WhiteboardCanvasContext = {
  fillStyle: string;
  strokeStyle: string;
  lineWidth: number;
  lineCap: string;
  lineJoin: string;
  font: string;
  textBaseline: string;
  save(): void;
  restore(): void;
  clearRect(x: number, y: number, width: number, height: number): void;
  translate(x: number, y: number): void;
  scale(x: number, y: number): void;
  beginPath(): void;
  closePath(): void;
  moveTo(x: number, y: number): void;
  lineTo(x: number, y: number): void;
  bezierCurveTo(cp1x: number, cp1y: number, cp2x: number, cp2y: number, x: number, y: number): void;
  ellipse(x: number, y: number, radiusX: number, radiusY: number, rotation: number, startAngle: number, endAngle: number): void;
  rect(x: number, y: number, width: number, height: number): void;
  fill(): void;
  stroke(): void;
  fillText(text: string, x: number, y: number): void;
  setLineDash(dash: number[]): void;
};

export type WhiteboardRenderScene = {
  readonly document: WhiteboardDocument;
  readonly viewport?: WhiteboardViewport;
  readonly size: WhiteboardSize;
  readonly selection?: WhiteboardSelection;
  readonly draft?: WhiteboardElement | null;
  readonly selectionArea?: WhiteboardBounds | null;
};

const SELECTION_STROKE = "hsl(212 100% 58%)";
const SELECTION_FILL = "hsl(0 0% 100%)";

export function renderWhiteboardScene(context: WhiteboardCanvasContext, scene: WhiteboardRenderScene): void {
  const viewport = scene.viewport ?? scene.document.viewport;
  const selection = scene.selection ?? { ids: scene.document.selection };

  context.save();
  context.clearRect(0, 0, scene.size.width, scene.size.height);
  context.translate(viewport.x, viewport.y);
  context.scale(viewport.zoom, viewport.zoom);

  for (const element of scene.document.elements) {
    if (!element.hidden) {
      renderWhiteboardElement(context, element);
    }
  }

  if (scene.draft) {
    renderWhiteboardElement(context, scene.draft, { draft: true });
  }

  if (scene.selectionArea) {
    renderSelectionArea(context, scene.selectionArea);
  }

  renderSelection(context, scene.document, selection);
  context.restore();
}

export function renderWhiteboardElement(
  context: WhiteboardCanvasContext,
  element: WhiteboardElement,
  options: { readonly draft?: boolean } = {},
): void {
  const strokeWidth = element.style.strokeWidth;

  context.save();
  context.strokeStyle = options.draft ? SELECTION_STROKE : element.style.stroke;
  context.fillStyle = element.style.fill;
  context.lineWidth = options.draft ? Math.max(1, strokeWidth) : strokeWidth;
  context.lineCap = element.style.lineCap;
  context.lineJoin = "round";
  context.setLineDash(dashForElement(element, options.draft));

  switch (element.type) {
    case "rectangle":
      drawRectangle(context, element.x, element.y, element.width, element.height, element.style.fill);
      break;
    case "ellipse":
      drawEllipse(context, element.x, element.y, element.width, element.height, element.style.fill);
      break;
    case "diamond":
      drawDiamond(context, element.x, element.y, element.width, element.height, element.style.fill);
      break;
    case "line":
      drawPolyline(context, element.points, false);
      break;
    case "arrow":
      drawPolyline(context, element.points, true);
      break;
    case "path":
      drawPolyline(context, element.closed ? [...element.points, element.points[0]].filter(Boolean) : element.points, false);
      break;
    case "freehand":
      drawFreehand(context, element.points);
      break;
    case "text":
      drawText(context, element);
      break;
    case "image":
      drawImagePlaceholder(context, element);
      break;
  }

  context.setLineDash([]);
  context.restore();
}

function drawRectangle(
  context: WhiteboardCanvasContext,
  x: number,
  y: number,
  width: number,
  height: number,
  fill: string,
): void {
  context.beginPath();
  context.rect(x, y, width, height);
  fillIfVisible(context, fill);
  context.stroke();
}

function drawEllipse(
  context: WhiteboardCanvasContext,
  x: number,
  y: number,
  width: number,
  height: number,
  fill: string,
): void {
  context.beginPath();
  context.ellipse(x + width / 2, y + height / 2, Math.abs(width / 2), Math.abs(height / 2), 0, 0, Math.PI * 2);
  fillIfVisible(context, fill);
  context.stroke();
}

function drawDiamond(
  context: WhiteboardCanvasContext,
  x: number,
  y: number,
  width: number,
  height: number,
  fill: string,
): void {
  const centerX = x + width / 2;
  const centerY = y + height / 2;

  context.beginPath();
  context.moveTo(centerX, y);
  context.lineTo(x + width, centerY);
  context.lineTo(centerX, y + height);
  context.lineTo(x, centerY);
  context.closePath();
  fillIfVisible(context, fill);
  context.stroke();
}

function drawPolyline(context: WhiteboardCanvasContext, points: readonly { readonly x: number; readonly y: number }[], arrow: boolean): void {
  if (points.length < 2) {
    return;
  }

  context.beginPath();
  context.moveTo(points[0].x, points[0].y);
  for (const point of points.slice(1)) {
    context.lineTo(point.x, point.y);
  }
  context.stroke();

  if (arrow) {
    drawArrowHead(context, points[points.length - 2], points[points.length - 1]);
  }
}

function drawFreehand(context: WhiteboardCanvasContext, points: readonly { readonly x: number; readonly y: number }[]): void {
  if (points.length < 2) {
    return;
  }

  context.beginPath();
  context.moveTo(points[0].x, points[0].y);

  for (let index = 1; index < points.length - 1; index += 1) {
    const current = points[index];
    const next = points[index + 1];
    const midX = (current.x + next.x) / 2;
    const midY = (current.y + next.y) / 2;
    context.bezierCurveTo(current.x, current.y, current.x, current.y, midX, midY);
  }

  const last = points[points.length - 1];
  context.lineTo(last.x, last.y);
  context.stroke();
}

function drawArrowHead(
  context: WhiteboardCanvasContext,
  start: { readonly x: number; readonly y: number },
  end: { readonly x: number; readonly y: number },
): void {
  const angle = Math.atan2(end.y - start.y, end.x - start.x);
  const size = 14;
  const left = {
    x: end.x - Math.cos(angle - Math.PI / 6) * size,
    y: end.y - Math.sin(angle - Math.PI / 6) * size,
  };
  const right = {
    x: end.x - Math.cos(angle + Math.PI / 6) * size,
    y: end.y - Math.sin(angle + Math.PI / 6) * size,
  };

  context.beginPath();
  context.moveTo(end.x, end.y);
  context.lineTo(left.x, left.y);
  context.moveTo(end.x, end.y);
  context.lineTo(right.x, right.y);
  context.stroke();
}

function drawText(context: WhiteboardCanvasContext, element: Extract<WhiteboardElement, { type: "text" }>): void {
  const fontSize = element.style.fontSize;
  const lineHeight = fontSize * 1.3;
  const lines = element.text.length > 0 ? element.text.split("\n") : [""];

  context.fillStyle = element.style.textColor;
  context.font = `${fontSize}px ${element.style.fontFamily}`;
  context.textBaseline = "top";

  for (let index = 0; index < lines.length; index += 1) {
    context.fillText(lines[index], element.x, element.y + index * lineHeight);
  }
}

function drawImagePlaceholder(context: WhiteboardCanvasContext, element: Extract<WhiteboardElement, { type: "image" }>): void {
  drawRectangle(context, element.x, element.y, element.width, element.height, element.style.fill);

  context.fillStyle = element.style.textColor;
  context.font = `${Math.max(12, Math.min(18, element.style.fontSize))}px ${element.style.fontFamily}`;
  context.textBaseline = "top";
  context.fillText(element.alt || "Image", element.x + 12, element.y + 12);
  context.fillText("Image", element.x + 12, element.y + 36);
}

function renderSelection(
  context: WhiteboardCanvasContext,
  document: WhiteboardDocument,
  selection: WhiteboardSelection,
): void {
  const selectedElements = elementsById(document.elements, selection.ids).filter((element) => !element.hidden);
  const bounds = selectionBounds(selectedElements);

  if (!bounds) {
    return;
  }

  context.save();
  context.strokeStyle = SELECTION_STROKE;
  context.fillStyle = SELECTION_FILL;
  context.lineWidth = 1;
  context.setLineDash([4, 4]);
  context.beginPath();
  context.rect(bounds.x, bounds.y, bounds.width, bounds.height);
  context.stroke();
  context.setLineDash([]);

  for (const handle of selectionHandleRects(bounds)) {
    context.beginPath();
    context.rect(handle.x, handle.y, handle.width, handle.height);
    context.fill();
    context.stroke();
  }

  context.restore();
}

function renderSelectionArea(context: WhiteboardCanvasContext, bounds: WhiteboardBounds): void {
  context.save();
  context.strokeStyle = SELECTION_STROKE;
  context.fillStyle = "hsla(212, 100%, 58%, 0.1)";
  context.lineWidth = 1;
  context.setLineDash([6, 6]);
  context.beginPath();
  context.rect(bounds.x, bounds.y, bounds.width, bounds.height);
  context.fill();
  context.stroke();
  context.restore();
}

function fillIfVisible(context: WhiteboardCanvasContext, fill: string): void {
  const normalized = fill.trim().toLowerCase();
  if (normalized !== "transparent" && normalized !== "none") {
    context.fill();
  }
}

function dashForElement(element: WhiteboardElement, draft: boolean | undefined): number[] {
  if (draft) {
    return [6, 6];
  }

  if (element.style.strokeStyle === "dashed") {
    return [10, 8];
  }

  if (element.style.strokeStyle === "dotted") {
    return [2, 6];
  }

  return [];
}
