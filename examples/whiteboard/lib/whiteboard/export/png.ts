import type { WhiteboardDocument } from "../persistence/schema";
import { createWhiteboardExportMetadata, type WhiteboardExportMetadata } from "./metadata";
import { exportWhiteboardToSvg, type WhiteboardSvgExportOptions } from "./svg";

export type WhiteboardPngExportOptions = WhiteboardSvgExportOptions & {
  fileName?: string;
};

export type WhiteboardPngExportPlan = {
  mimeType: "image/png";
  fileName: string;
  width: number;
  height: number;
  svg: string;
  metadata: WhiteboardExportMetadata;
};

export type WhiteboardPngRenderer = (plan: WhiteboardPngExportPlan) => Promise<string> | string;

function sizeFromOptions(document: WhiteboardDocument, options: WhiteboardPngExportOptions): { width: number; height: number } {
  void document;
  const width = options.width ?? 1280;
  const height = options.height ?? 720;

  if (!Number.isFinite(width) || width <= 0) {
    throw new Error("PNG export width must be greater than 0");
  }

  if (!Number.isFinite(height) || height <= 0) {
    throw new Error("PNG export height must be greater than 0");
  }

  return { width, height };
}

export function createWhiteboardPngExport(
  document: WhiteboardDocument,
  options: WhiteboardPngExportOptions = {},
): WhiteboardPngExportPlan {
  const size = sizeFromOptions(document, options);
  const svg = exportWhiteboardToSvg(document, {
    ...options,
    width: size.width,
    height: size.height,
  });

  return {
    mimeType: "image/png",
    fileName: options.fileName ?? `${document.id}.png`,
    width: size.width,
    height: size.height,
    svg,
    metadata: createWhiteboardExportMetadata(document, { format: "png-plan" }),
  };
}

export async function exportWhiteboardToPngDataUrl(
  document: WhiteboardDocument,
  options: WhiteboardPngExportOptions = {},
  renderer?: WhiteboardPngRenderer,
): Promise<string> {
  const plan = createWhiteboardPngExport(document, options);

  if (!renderer) {
    throw new Error("PNG rasterization requires a renderer, such as a browser canvas or server image adapter.");
  }

  const dataUrl = await renderer(plan);

  if (!dataUrl.startsWith("data:image/png")) {
    throw new Error("PNG renderer must return a data:image/png URL");
  }

  return dataUrl;
}
