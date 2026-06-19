import type { WhiteboardDocument, WhiteboardElement, WhiteboardPoint } from "../persistence/schema";
import { validateWhiteboardDocument } from "../persistence/schema";
import { connectorRouteForElement } from "../connector-routes";
import { WHITEBOARD_IMAGE_SOURCE_POLICY } from "../image-source";

export type WhiteboardSvgExportOptions = {
  width?: number;
  height?: number;
  background?: string;
  padding?: number;
};

function escapeXml(value: string): string {
  return value
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

function number(value: number): string {
  return Number(value.toFixed(4)).toString();
}

function dimensions(document: WhiteboardDocument, options: WhiteboardSvgExportOptions): { width: number; height: number } {
  const width = options.width ?? 1280;
  const height = options.height ?? 720;

  if (!Number.isFinite(width) || width <= 0) {
    throw new Error("SVG export width must be greater than 0");
  }

  if (!Number.isFinite(height) || height <= 0) {
    throw new Error("SVG export height must be greater than 0");
  }

  return { width, height };
}

function styleAttributes(element: WhiteboardElement): string {
  const attrs = [
    `fill="${escapeXml(element.style.fill)}"`,
    `stroke="${escapeXml(element.style.stroke)}"`,
    `stroke-width="${number(element.style.strokeWidth)}"`,
    `opacity="${number(element.style.opacity)}"`,
  ];

  const dash = strokeDashArray(element.style.strokeStyle, element.style.strokeWidth);
  if (dash.length > 0) {
    attrs.push(`stroke-dasharray="${dash.map(number).join(" ")}"`);
  }

  return attrs.join(" ");
}

function strokeDashArray(strokeStyle: WhiteboardElement["style"]["strokeStyle"], strokeWidth: number): number[] {
  if (strokeStyle === "dashed") {
    return [Math.max(8, strokeWidth * 4), Math.max(6, strokeWidth * 3)];
  }

  if (strokeStyle === "dotted") {
    return [Math.max(1, strokeWidth), Math.max(5, strokeWidth * 3)];
  }

  return [];
}

function pathFromPoints(points: readonly WhiteboardPoint[] | undefined): string {
  if (!points || points.length === 0) {
    return "";
  }

  const [first, ...rest] = points;

  return [`M ${number(first.x)} ${number(first.y)}`, ...rest.map((point) => `L ${number(point.x)} ${number(point.y)}`)].join(" ");
}

function rotation(element: WhiteboardElement): string {
  if (element.rotation === 0) {
    return "";
  }

  const centerX = element.x + element.width / 2;
  const centerY = element.y + element.height / 2;

  return ` transform="rotate(${number(element.rotation)} ${number(centerX)} ${number(centerY)})"`;
}

function renderElement(element: WhiteboardElement): string {
  switch (element.type) {
    case "rectangle":
      return `<rect x="${number(element.x)}" y="${number(element.y)}" width="${number(element.width)}" height="${number(element.height)}"${rotation(element)} ${styleAttributes(element)} />`;
    case "ellipse":
      return `<ellipse cx="${number(element.x + element.width / 2)}" cy="${number(element.y + element.height / 2)}" rx="${number(element.width / 2)}" ry="${number(element.height / 2)}"${rotation(element)} ${styleAttributes(element)} />`;
    case "diamond": {
      const x = element.x;
      const y = element.y;
      const midX = x + element.width / 2;
      const midY = y + element.height / 2;
      const points = `${number(midX)},${number(y)} ${number(x + element.width)},${number(midY)} ${number(midX)},${number(y + element.height)} ${number(x)},${number(midY)}`;

      return `<polygon points="${points}"${rotation(element)} ${styleAttributes(element)} />`;
    }
    case "line":
    case "arrow": {
      const marker = element.type === "arrow" ? ' marker-end="url(#dx-whiteboard-arrowhead)"' : "";

      return `<path d="${escapeXml(pathFromPoints(element.points))}" ${styleAttributes(element)} fill="none" stroke-linecap="round" stroke-linejoin="round" data-whiteboard-connector-route="${connectorRouteForElement(element)}"${marker} />`;
    }
    case "freehand":
      return `<path d="${escapeXml(pathFromPoints(element.points))}" ${styleAttributes(element)} fill="none" stroke-linecap="round" stroke-linejoin="round" />`;
    case "text": {
      const fontSize = element.style.fontSize ?? 16;
      const fontFamily = escapeXml(element.style.fontFamily ?? "Inter, Arial, sans-serif");
      const text = escapeXml(element.text);

      return `<text x="${number(element.x)}" y="${number(element.y + fontSize)}"${rotation(element)} fill="${escapeXml(element.style.textColor)}" opacity="${number(element.style.opacity)}" font-family="${fontFamily}" font-size="${number(fontSize)}">${text}</text>`;
    }
    case "image":
      return `<image x="${number(element.x)}" y="${number(element.y)}" width="${number(element.width)}" height="${number(element.height)}"${rotation(element)} href="${escapeXml(element.src)}" opacity="${number(element.style.opacity)}" aria-label="${escapeXml(element.alt)}" data-dx-whiteboard-alt="${escapeXml(element.alt)}" data-dx-whiteboard-image-policy="${WHITEBOARD_IMAGE_SOURCE_POLICY}" preserveAspectRatio="xMidYMid meet" />`;
    case "path":
      return `<path d="${escapeXml(pathFromPoints(element.points))}" ${styleAttributes(element)} fill="${element.closed ? escapeXml(element.style.fill) : "none"}" stroke-linecap="round" stroke-linejoin="round" />`;
  }
}

export function exportWhiteboardToSvg(document: WhiteboardDocument, options: WhiteboardSvgExportOptions = {}): string {
  const validated = validateWhiteboardDocument(document);
  const size = dimensions(validated, options);
  const background = options.background
    ? `<rect x="0" y="0" width="${number(size.width)}" height="${number(size.height)}" fill="${escapeXml(options.background)}" />`
    : "";
  const elements = validated.elements.filter((element) => !element.hidden).map(renderElement).join("");

  return `<svg xmlns="http://www.w3.org/2000/svg" width="${number(size.width)}" height="${number(size.height)}" viewBox="0 0 ${number(size.width)} ${number(size.height)}" role="img" data-dx-whiteboard-id="${escapeXml(validated.id)}"><title>${escapeXml(validated.name)}</title><defs><marker id="dx-whiteboard-arrowhead" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse"><path d="M 0 0 L 10 5 L 0 10 z" fill="context-stroke" /></marker></defs>${background}${elements}</svg>`;
}
