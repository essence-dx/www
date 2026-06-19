import type { WhiteboardTool } from "../render/model";

export function createDefaultIdFactory(): () => string {
  let counter = 0;
  return () => {
    counter += 1;
    return `whiteboard-${counter}`;
  };
}

export function clampZoom(value: number): number {
  return Math.max(0.1, Math.min(8, value));
}

export function toolForShortcut(key: string): WhiteboardTool | null {
  const shortcuts: Record<string, WhiteboardTool> = {
    v: "select",
    h: "pan",
    p: "pan",
    r: "rectangle",
    e: "ellipse",
    d: "diamond",
    l: "line",
    a: "arrow",
    f: "freehand",
    b: "freehand",
    t: "text",
    x: "eraser",
  };

  return shortcuts[key] ?? null;
}

