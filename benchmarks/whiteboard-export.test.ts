import assert from "node:assert/strict";
import { describe, it } from "node:test";

import { exportDxDraw, importDxDraw } from "../examples/whiteboard/lib/whiteboard/export/dxdraw.ts";
import {
  createWhiteboardPngExport,
  exportWhiteboardToPngDataUrl,
} from "../examples/whiteboard/lib/whiteboard/export/png.ts";
import { exportWhiteboardToSvg } from "../examples/whiteboard/lib/whiteboard/export/svg.ts";
import { createImportedImageElement } from "../examples/whiteboard/lib/whiteboard/image-import.ts";
import { createWhiteboardDocument } from "../examples/whiteboard/server/whiteboard/commands.ts";
import { DEFAULT_WHITEBOARD_STYLE } from "../examples/whiteboard/lib/whiteboard/model.ts";
import type { WhiteboardElement } from "../examples/whiteboard/lib/whiteboard/persistence/schema.ts";

const EMBEDDED_IMAGE_SRC =
  "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 160 120'%3E%3Crect width='160' height='120' fill='%23111827'/%3E%3Ctext x='24' y='64' fill='%23f8fafc'%3EDX%3C/text%3E%3C/svg%3E";

function exportFixture() {
  const document = createWhiteboardDocument({
    id: "export-board",
    name: "Export Board",
    now: () => "2026-06-02T09:00:00.000Z",
  });
  const elements: WhiteboardElement[] = [
    {
      id: "rect-export",
      type: "rectangle",
      x: 10,
      y: 20,
      width: 200,
      height: 100,
      rotation: 0,
      style: {
        ...DEFAULT_WHITEBOARD_STYLE,
        fill: "#f8fafc",
        stroke: "#111827",
        strokeWidth: 2,
      },
      locked: false,
      hidden: false,
      createdAt: "2026-06-02T09:00:00.000Z",
      updatedAt: "2026-06-02T09:00:00.000Z",
    },
    {
      id: "text-export",
      type: "text",
      x: 32,
      y: 56,
      width: 160,
      height: 40,
      rotation: 0,
      text: "DX <Whiteboard> & Export",
      textAlign: "left",
      verticalAlign: "top",
      style: {
        ...DEFAULT_WHITEBOARD_STYLE,
        fill: "#111827",
        stroke: "none",
        strokeWidth: 0,
      },
      locked: false,
      hidden: false,
      createdAt: "2026-06-02T09:00:00.000Z",
      updatedAt: "2026-06-02T09:00:00.000Z",
    },
    {
      id: "image-export",
      type: "image",
      role: "image",
      x: 220,
      y: 30,
      width: 80,
      height: 60,
      rotation: 0,
      src: EMBEDDED_IMAGE_SRC,
      alt: "Export image <safe>",
      naturalWidth: 160,
      naturalHeight: 120,
      style: {
        ...DEFAULT_WHITEBOARD_STYLE,
        fill: "#0f172a",
        stroke: "#38bdf8",
        textColor: "#f8fafc",
      },
      locked: false,
      hidden: false,
      createdAt: "2026-06-02T09:00:00.000Z",
      updatedAt: "2026-06-02T09:00:00.000Z",
    },
    {
      id: "hidden-image-export",
      type: "image",
      role: "image",
      x: 40,
      y: 140,
      width: 80,
      height: 60,
      rotation: 0,
      src: EMBEDDED_IMAGE_SRC,
      alt: "Hidden image must not export",
      style: DEFAULT_WHITEBOARD_STYLE,
      locked: false,
      hidden: true,
      createdAt: "2026-06-02T09:00:00.000Z",
      updatedAt: "2026-06-02T09:00:00.000Z",
    },
  ];

  return {
    ...document,
    elements,
    metadata: {
      ...document.metadata,
      revision: 3,
    },
  };
}

describe(".dxdraw import and export", () => {
  it("round-trips validated documents through a versioned .dxdraw envelope", () => {
    const document = exportFixture();
    const json = exportDxDraw(document, {
      exportedAt: "2026-06-02T09:30:00.000Z",
    });
    const parsed = JSON.parse(json);
    const imported = importDxDraw(json);

    assert.equal(parsed.format, "dx.whiteboard.dxdraw");
    assert.equal(parsed.version, 1);
    assert.equal(parsed.exportedAt, "2026-06-02T09:30:00.000Z");
    assert.equal(imported.id, "export-board");
    assert.equal(imported.elements.length, 4);
    assert.equal(imported.elements[2]?.type, "image");
    assert.equal(parsed.metadata.document.imageCount, 2);
    assert.equal(parsed.metadata.imagePolicy, "embedded-data-url-only");
  });

  it("round-trips connector route metadata through .dxdraw import and export", () => {
    const document = createWhiteboardDocument({
      id: "route-roundtrip-board",
      name: "Route Roundtrip Board",
      now: () => "2026-06-02T09:00:00.000Z",
    });
    const json = exportDxDraw({
      ...document,
      elements: [
        {
          id: "orthogonal-roundtrip",
          type: "arrow",
          points: [
            { x: 20, y: 40 },
            { x: 80, y: 40 },
            { x: 80, y: 120 },
            { x: 160, y: 120 },
          ],
          style: DEFAULT_WHITEBOARD_STYLE,
          locked: false,
          hidden: false,
          startArrow: "none",
          endArrow: "triangle",
          createdAt: "2026-06-02T09:00:00.000Z",
          updatedAt: "2026-06-02T09:00:00.000Z",
          metadata: { connectorRoute: "orthogonal" },
        },
      ],
    });
    const parsed = JSON.parse(json);
    const imported = importDxDraw(json);

    assert.equal(parsed.document.elements[0]?.metadata?.connectorRoute, "orthogonal");
    assert.equal(imported.elements[0]?.metadata?.connectorRoute, "orthogonal");
  });

  it("round-trips locally imported image metadata through .dxdraw import and export", () => {
    const importedImage = createImportedImageElement({
      id: "local-import-export",
      fileName: "local-import.png",
      fileType: "image/png",
      fileSizeBytes: 1024,
      naturalWidth: 512,
      naturalHeight: 256,
      source: "data:image/png;base64,aW1hZ2U=",
      createdAt: "2026-06-02T09:00:00.000Z",
      updatedAt: "2026-06-02T09:00:00.000Z",
    });
    assert.equal(importedImage.status, "accepted");
    const document = createWhiteboardDocument({
      id: "local-import-export-board",
      name: "Local Import Export Board",
      now: () => "2026-06-02T09:00:00.000Z",
    });
    const json = exportDxDraw({
      ...document,
      elements: importedImage.status === "accepted" ? [importedImage.element] : [],
    });
    const imported = importDxDraw(json);
    const element = imported.elements[0];

    assert.equal(JSON.parse(json).metadata.document.imageCount, 1);
    assert.equal(element?.type, "image");
    assert.equal(element?.type === "image" ? element.src : "", "data:image/png;base64,aW1hZ2U=");
    assert.equal(element?.type === "image" ? element.alt : "", "local import");
    assert.equal(element?.type === "image" ? element.metadata?.importSource : "", "local-file");
    assert.equal(element?.type === "image" ? element.metadata?.imagePolicy : "", "embedded-data-url-only");
    assert.equal(element?.type === "image" ? element.metadata?.fileSizeBytes : 0, 1024);
  });

  it("rejects non-whiteboard envelopes instead of silently accepting arbitrary JSON", () => {
    assert.throws(
      () => importDxDraw(JSON.stringify({ format: "not.dxdraw", version: 1, document: {} })),
      /dxdraw format/,
    );
  });
});

describe("SVG export", () => {
  it("renders stable SVG with escaped text and board metadata", () => {
    const svg = exportWhiteboardToSvg(exportFixture(), {
      width: 320,
      height: 200,
      background: "#ffffff",
    });

    assert.match(svg, /^<svg /);
    assert.match(svg, /data-dx-whiteboard-id="export-board"/);
    assert.match(svg, /<rect x="10" y="20" width="200" height="100"/);
    assert.match(svg, /<image x="220" y="30" width="80" height="60"/);
    assert.match(svg, /href="data:image\/svg\+xml,/);
    assert.match(svg, /data-dx-whiteboard-image-policy="embedded-data-url-only"/);
    assert.match(svg, /data-dx-whiteboard-alt="Export image &lt;safe&gt;"/);
    assert.doesNotMatch(svg, /Hidden image must not export/);
    assert.match(svg, /DX &lt;Whiteboard&gt; &amp; Export/);
    assert.match(svg, /<\/svg>$/);
  });

  it("rejects unsupported image sources before producing SVG", () => {
    const [first, second, image] = exportFixture().elements;

    assert.throws(
      () =>
        exportWhiteboardToSvg({
          ...exportFixture(),
          elements: [
            first as WhiteboardElement,
            second as WhiteboardElement,
            {
              ...(image as Extract<WhiteboardElement, { type: "image" }>),
              src: "https://example.com/remote.svg",
            },
          ],
        }),
      /data:image/,
    );
  });

  it("marks connector routes in SVG without tagging freehand strokes", () => {
    const document = createWhiteboardDocument({
      id: "connector-export-board",
      name: "Connector Export Board",
      now: () => "2026-06-02T09:00:00.000Z",
    });
    const svg = exportWhiteboardToSvg({
      ...document,
      elements: [
        {
          id: "orthogonal-export",
          type: "arrow",
          points: [
            { x: 20, y: 40 },
            { x: 80, y: 40 },
            { x: 80, y: 120 },
            { x: 160, y: 120 },
          ],
          style: DEFAULT_WHITEBOARD_STYLE,
          locked: false,
          hidden: false,
          startArrow: "none",
          endArrow: "triangle",
          createdAt: "2026-06-02T09:00:00.000Z",
          updatedAt: "2026-06-02T09:00:00.000Z",
          metadata: { connectorRoute: "orthogonal" },
        },
        {
          id: "freehand-export",
          type: "freehand",
          points: [
            { x: 20, y: 150 },
            { x: 80, y: 168 },
          ],
          style: DEFAULT_WHITEBOARD_STYLE,
          locked: false,
          hidden: false,
          createdAt: "2026-06-02T09:00:00.000Z",
          updatedAt: "2026-06-02T09:00:00.000Z",
        },
      ],
    });

    assert.match(
      svg,
      /d="M 20 40 L 80 40 L 80 120 L 160 120"[^>]*data-whiteboard-connector-route="orthogonal"/,
    );
    assert.equal(svg.split("data-whiteboard-connector-route=").length - 1, 1);
  });

  it("exports locally imported images to SVG without weakening image policy", () => {
    const importedImage = createImportedImageElement({
      id: "local-import-svg",
      fileName: "svg-image.png",
      fileType: "image/png",
      fileSizeBytes: 1024,
      naturalWidth: 320,
      naturalHeight: 180,
      source: "data:image/png;base64,aW1hZ2U=",
      createdAt: "2026-06-02T09:00:00.000Z",
      updatedAt: "2026-06-02T09:00:00.000Z",
    });
    assert.equal(importedImage.status, "accepted");
    const document = createWhiteboardDocument({
      id: "local-import-svg-board",
      name: "Local Import SVG Board",
      now: () => "2026-06-02T09:00:00.000Z",
    });
    const board = {
      ...document,
      elements: importedImage.status === "accepted" ? [importedImage.element] : [],
    };
    const svg = exportWhiteboardToSvg(board);
    const pngPlan = createWhiteboardPngExport(board);

    assert.match(svg, /<image /);
    assert.match(svg, /href="data:image\/png;base64,aW1hZ2U="/);
    assert.match(svg, /data-dx-whiteboard-alt="svg image"/);
    assert.match(svg, /data-dx-whiteboard-image-policy="embedded-data-url-only"/);
    assert.equal(pngPlan.metadata.document.imageCount, 1);
    assert.equal(pngPlan.metadata.imagePolicy, "embedded-data-url-only");
  });
});

describe("PNG export helper API", () => {
  it("creates a rasterization plan without pretending Node has a canvas", async () => {
    const plan = createWhiteboardPngExport(exportFixture(), {
      width: 320,
      height: 200,
      background: "#ffffff",
      fileName: "export-board.png",
    });

    assert.equal(plan.mimeType, "image/png");
    assert.equal(plan.fileName, "export-board.png");
    assert.equal(plan.width, 320);
    assert.equal(plan.height, 200);
    assert.equal(plan.metadata.imagePolicy, "embedded-data-url-only");
    assert.equal(plan.metadata.document.imageCount, 2);
    assert.match(plan.svg, /<svg /);
    assert.match(plan.svg, /href="data:image\/svg\+xml,/);
    assert.doesNotMatch(plan.svg, /Hidden image must not export/);
    await assert.rejects(
      () => exportWhiteboardToPngDataUrl(exportFixture(), { width: 320, height: 200 }),
      /PNG rasterization requires a renderer/,
    );
  });

  it("delegates PNG rasterization to an injected renderer", async () => {
    const dataUrl = await exportWhiteboardToPngDataUrl(
      exportFixture(),
      { width: 320, height: 200 },
      async (plan) => {
        assert.equal(plan.mimeType, "image/png");
        assert.match(plan.svg, /data-dx-whiteboard-id="export-board"/);
        assert.equal(plan.metadata.imagePolicy, "embedded-data-url-only");
        return "data:image/png;base64,ZmFrZS1wbmc=";
      },
    );

    assert.equal(dataUrl, "data:image/png;base64,ZmFrZS1wbmc=");
  });
});
