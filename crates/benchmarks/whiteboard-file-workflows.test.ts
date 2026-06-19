import assert from "node:assert/strict";
import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, it } from "node:test";

import { exportDxDraw } from "../examples/whiteboard/lib/whiteboard/export/dxdraw.ts";
import { importDxDraw } from "../examples/whiteboard/lib/whiteboard/export/dxdraw.ts";
import { createWhiteboardShareReceipt } from "../examples/whiteboard/lib/whiteboard/export/share-receipt.ts";
import { createImportedImageElement } from "../examples/whiteboard/lib/whiteboard/image-import.ts";
import { validateWhiteboardImport } from "../examples/whiteboard/lib/whiteboard/import/validation.ts";
import {
  DEFAULT_WHITEBOARD_STYLE,
  createWhiteboardDocument,
} from "../examples/whiteboard/lib/whiteboard/model.ts";
import {
  createImageElement,
  createRectangleElement,
} from "../examples/whiteboard/lib/whiteboard/scene.ts";
import { createWhiteboardFileStorageSnapshot } from "../examples/whiteboard/server/whiteboard/filesystem-snapshot.ts";
import { createWhiteboardFileStorageDriver } from "../examples/whiteboard/server/whiteboard/filesystem-storage.ts";
import { createWhiteboardStore } from "../examples/whiteboard/lib/stores/whiteboard-store.ts";

const NOW = "2026-06-03T00:00:00.000Z";
const EMBEDDED_IMAGE_SRC =
  "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 180 120'%3E%3Crect width='180' height='120' fill='%230f172a'/%3E%3Ccircle cx='90' cy='60' r='32' fill='%2338bdf8'/%3E%3C/svg%3E";
const EMBEDDED_PNG_SRC = "data:image/png;base64,iVBORw0KGgo=";

describe("whiteboard import validation", () => {
  it("accepts .dxdraw envelopes and raw canonical documents with summaries", () => {
    const document = fixtureDocument();
    const dxdrawReport = validateWhiteboardImport(exportDxDraw(document, { exportedAt: NOW }));
    const rawReport = validateWhiteboardImport(document);

    assert.equal(dxdrawReport.status, "accepted");
    assert.equal(dxdrawReport.status === "accepted" ? dxdrawReport.format : null, "dxdraw");
    assert.equal(dxdrawReport.status === "accepted" ? dxdrawReport.summary.elementCount : null, 4);
    assert.equal(dxdrawReport.status === "accepted" ? dxdrawReport.summary.hiddenElementCount : null, 1);
    assert.equal(dxdrawReport.status === "accepted" ? dxdrawReport.summary.imageCount : null, 1);
    assert.equal(rawReport.status, "accepted");
    assert.equal(rawReport.status === "accepted" ? rawReport.format : null, "document");
  });

  it("rejects invalid whiteboard input with structured diagnostics", () => {
    const invalidJson = validateWhiteboardImport("{not-json");
    const invalidDocument = validateWhiteboardImport({
      schemaVersion: 1,
      id: "bad-board",
      name: "Bad Board",
      elements: [{ id: "bad", type: "line", points: [{ x: 0, y: 0 }] }],
    });

    assert.equal(invalidJson.status, "rejected");
    assert.match(invalidJson.diagnostics[0]?.message ?? "", /Invalid whiteboard JSON/);
    assert.equal(invalidDocument.status, "rejected");
    assert.match(invalidDocument.diagnostics[0]?.message ?? "", /points/);
  });

  it("reports legacy document migration as an explicit warning", () => {
    const report = validateWhiteboardImport({
      schema: "dx.whiteboard.document",
      version: 1,
      id: "legacy-workflow-board",
      name: "Legacy Workflow Board",
      elements: [],
    });

    assert.equal(report.status, "accepted");
    assert.match(report.diagnostics.map((diagnostic) => diagnostic.message).join("\n"), /migrated/);
  });
});

describe("whiteboard local image import", () => {
  it("creates local image import elements from supported embedded file data URLs", () => {
    const imported = createImportedImageElement({
      id: "imported-logo",
      fileName: "team-logo.png",
      fileType: "image/png",
      fileSizeBytes: 2048,
      naturalWidth: 1200,
      naturalHeight: 900,
      origin: { x: 24, y: 48 },
      source: EMBEDDED_PNG_SRC,
      createdAt: NOW,
      updatedAt: NOW,
    });

    assert.equal(imported.status, "accepted");
    assert.equal(imported.status === "accepted" ? imported.mimeType : null, "image/png");
    assert.equal(imported.status === "accepted" ? imported.element.type : null, "image");
    assert.equal(imported.status === "accepted" ? imported.element.src : null, EMBEDDED_PNG_SRC);
    assert.equal(imported.status === "accepted" ? imported.element.alt : null, "team logo");
    assert.equal(imported.status === "accepted" ? imported.element.name : null, "Image: team logo");
    assert.equal(imported.status === "accepted" ? imported.element.x : null, 24);
    assert.equal(imported.status === "accepted" ? imported.element.y : null, 48);
    assert.equal(imported.status === "accepted" ? imported.element.width : null, 560);
    assert.equal(imported.status === "accepted" ? imported.element.height : null, 420);
    assert.equal(imported.status === "accepted" ? imported.element.naturalWidth : null, 1200);
    assert.equal(imported.status === "accepted" ? imported.element.naturalHeight : null, 900);
    assert.equal(imported.status === "accepted" ? imported.element.metadata?.importSource : null, "local-file");
    assert.equal(imported.status === "accepted" ? imported.element.metadata?.imagePolicy : null, "embedded-data-url-only");
    assert.equal(imported.status === "accepted" ? imported.element.metadata?.mimeType : null, "image/png");
    assert.equal(imported.status === "accepted" ? imported.element.metadata?.originalName : null, "team-logo.png");
    assert.equal(imported.status === "accepted" ? imported.element.metadata?.sourceOwned : null, true);
    assert.equal(imported.status === "accepted" ? imported.element.metadata?.fileSizeBytes : null, 2048);
  });

  it("rejects invalid local image files with explicit diagnostics", () => {
    const cases = [
      ["", "image.empty_source"],
      ["data:text/plain;base64,SGVsbG8=", "image.unsupported_source"],
      ["https://example.com/logo.png", "image.unsupported_source"],
      ["data:image/png;base64,", "image.empty_payload"],
      ["data:image/png;base64,iVBORw0KGgo=", "image.empty_file", { fileSizeBytes: 0 }],
      ["data:image/png;base64,iVBORw0KGgo=", "image.too_large", { fileSizeBytes: 2_097_153 }],
      ["data:image/png;base64,iVBORw0KGgo=", "image.invalid_dimensions", { naturalWidth: 0, naturalHeight: 80 }],
      ["data:image/png;base64,iVBORw0KGgo=", "image.mime_mismatch", { fileType: "image/jpeg" }],
    ] as const;

    for (const [source, code, options] of cases) {
      const imported = createImportedImageElement({
        id: `invalid-${code}`,
        fileName: "invalid.png",
        fileSizeBytes: 1024,
        source,
        ...options,
      });

      assert.equal(imported.status, "rejected", code);
      assert.equal(
        imported.status === "rejected" ? imported.diagnostics.some((diagnostic) => diagnostic.code === code) : false,
        true,
        code,
      );
      assert.equal("element" in imported, false, code);
    }
  });

  it("enforces embedded data URL policy for image import sources", () => {
    for (const source of [
      "data:image/png;base64,aW1hZ2U=",
      "data:image/jpeg;base64,aW1hZ2U=",
      "data:image/webp;base64,aW1hZ2U=",
      "data:image/svg+xml,%3Csvg%3E%3C/svg%3E",
    ]) {
      assert.equal(
        createImportedImageElement({
          id: `accepted-${source.slice(11, 16)}`,
          fileName: "accepted.image",
          source,
        }).status,
        "accepted",
      );
    }

    for (const source of [
      "data:image/gif;base64,aW1hZ2U=",
      "blob:https://example.com/image",
      "file:///tmp/image.png",
      "https://example.com/image.png",
      "data:text/plain;base64,aW1hZ2U=",
    ]) {
      assert.equal(
        createImportedImageElement({
          id: `rejected-${source.slice(0, 4)}`,
          fileName: "rejected.image",
          source,
        }).status,
        "rejected",
        source,
      );
    }
  });

  it("inserts imported images through undoable store commands with selection", () => {
    const imported = createImportedImageElement({
      id: "store-imported-image",
      fileName: "store-image.svg",
      source: "data:image/svg+xml,%3Csvg%3E%3C/svg%3E",
      createdAt: NOW,
      updatedAt: NOW,
    });
    assert.equal(imported.status, "accepted");

    const store = createWhiteboardStore({
      document: createWhiteboardDocument({
        id: "image-store-board",
        name: "Image Store Board",
        createdAt: NOW,
        updatedAt: NOW,
      }),
    });

    if (imported.status === "accepted") {
      store.actions.importImage(imported.element);
    }

    assert.equal(store.getDocument().elements[0]?.type, "image");
    assert.deepEqual(store.getDocument().selection, ["store-imported-image"]);
    assert.equal(store.getState().canUndo, true);

    store.undo();
    assert.equal(store.getDocument().elements.length, 0);
    assert.equal(store.getState().canRedo, true);

    store.redo();
    assert.equal(store.getDocument().elements[0]?.id, "store-imported-image");
  });
});

describe("whiteboard export metadata and share receipt", () => {
  it("embeds exact export metadata in .dxdraw output", () => {
    const parsed = JSON.parse(exportDxDraw(fixtureDocument(), { exportedAt: NOW }));

    assert.equal(parsed.metadata.schema_version, "dx.whiteboard.export_metadata.v1");
    assert.equal(parsed.metadata.exportedAt, NOW);
    assert.equal(parsed.metadata.document.elementCount, 4);
    assert.equal(parsed.metadata.document.imageCount, 1);
    assert.equal(parsed.metadata.document.lockedElementCount, 1);
    assert.equal(parsed.metadata.imagePolicy, "embedded-data-url-only");
    assert.equal(parsed.metadata.sourceOwned, true);
    assert.equal(parsed.metadata.includesSecretValues, false);
  });

  it("round-trips locally imported images through .dxdraw export and import", () => {
    const imported = createImportedImageElement({
      id: "imported-roundtrip",
      fileName: "roundtrip.webp",
      fileType: "image/webp",
      fileSizeBytes: 512,
      naturalWidth: 480,
      naturalHeight: 320,
      source: "data:image/webp;base64,aW1hZ2U=",
      createdAt: NOW,
      updatedAt: NOW,
    });
    assert.equal(imported.status, "accepted");

    const document = createWhiteboardDocument({
      id: "imported-roundtrip-board",
      name: "Imported Roundtrip Board",
      createdAt: NOW,
      updatedAt: NOW,
      elements: imported.status === "accepted" ? [imported.element] : [],
    });
    const parsed = JSON.parse(exportDxDraw(document, { exportedAt: NOW }));
    const restored = importDxDraw(JSON.stringify(parsed));
    const restoredImage = restored.elements[0];

    assert.equal(parsed.metadata.document.imageCount, 1);
    assert.equal(parsed.metadata.imagePolicy, "embedded-data-url-only");
    assert.equal(restoredImage?.type, "image");
    assert.equal(restoredImage?.type === "image" ? restoredImage.src : "", "data:image/webp;base64,aW1hZ2U=");
    assert.equal(restoredImage?.type === "image" ? restoredImage.alt : "", "roundtrip");
    assert.equal(restoredImage?.type === "image" ? restoredImage.naturalWidth : 0, 480);
    assert.equal(restoredImage?.type === "image" ? restoredImage.metadata?.importSource : "", "local-file");
    assert.equal(restoredImage?.type === "image" ? restoredImage.metadata?.imagePolicy : "", "embedded-data-url-only");
  });

  it("embeds frame membership summary counts in .dxdraw metadata", () => {
    const parsed = JSON.parse(exportDxDraw(framedFixtureDocument(), { exportedAt: NOW }));

    assert.equal(parsed.metadata.document.frameCount, 1);
    assert.equal(parsed.metadata.document.framedElementCount, 1);
  });

  it("creates preview-only share receipts without pretending live collaboration exists", () => {
    const receipt = createWhiteboardShareReceipt(fixtureDocument(), {
      createdAt: NOW,
      target: "local-preview",
    });

    assert.equal(receipt.schema_version, "dx.whiteboard.share_receipt.v1");
    assert.equal(receipt.status, "preview-only");
    assert.equal(receipt.shareUrl, null);
    assert.equal(receipt.mutableRemoteState, false);
    assert.equal(receipt.metadata.document.revision, 7);
  });
});

describe("whiteboard filesystem snapshots", () => {
  it("summarizes valid .dxdraw files and reports invalid storage entries", async () => {
    const rootDir = await mkdtemp(join(tmpdir(), "dx-whiteboard-snapshot-"));
    const storage = createWhiteboardFileStorageDriver({
      rootDir,
      now: () => NOW,
    });

    try {
      await storage.save(fixtureDocument());
      await writeFile(join(rootDir, "broken.dxdraw"), "{broken", "utf8");

      const snapshot = await createWhiteboardFileStorageSnapshot({
        rootDir,
        generatedAt: NOW,
      });

      assert.equal(snapshot.schema_version, "dx.whiteboard.filesystem_snapshot.v1");
      assert.equal(snapshot.storageRoot.redacted, true);
      assert.equal(snapshot.storageRoot.name.includes(rootDir), false);
      assert.equal(snapshot.totalFiles, 2);
      assert.equal(snapshot.validCount, 1);
      assert.equal(snapshot.invalidCount, 1);
      assert.equal(snapshot.boards[0]?.id, "workflow-board");
      assert.equal(snapshot.diagnostics[0]?.file, "broken.dxdraw");
    } finally {
      await rm(rootDir, { recursive: true, force: true });
    }
  });
});

function fixtureDocument() {
  return createWhiteboardDocument({
    id: "workflow-board",
    name: "Workflow Board",
    createdAt: NOW,
    updatedAt: NOW,
    selection: ["visible-card"],
    metadata: { revision: 7 },
    elements: [
      createRectangleElement({
        id: "visible-card",
        x: 10,
        y: 20,
        width: 120,
        height: 80,
        style: { ...DEFAULT_WHITEBOARD_STYLE, fill: "#f8fafc" },
        createdAt: NOW,
        updatedAt: NOW,
      }),
      createRectangleElement({
        id: "locked-card",
        x: 180,
        y: 20,
        width: 120,
        height: 80,
        locked: true,
        createdAt: NOW,
        updatedAt: NOW,
      }),
      createRectangleElement({
        id: "hidden-card",
        x: 340,
        y: 20,
        width: 120,
        height: 80,
        hidden: true,
        createdAt: NOW,
        updatedAt: NOW,
      }),
      createImageElement({
        id: "embedded-image",
        role: "image",
        name: "Embedded image",
        x: 500,
        y: 20,
        width: 180,
        height: 120,
        src: EMBEDDED_IMAGE_SRC,
        alt: "Embedded workflow image",
        naturalWidth: 180,
        naturalHeight: 120,
        style: {
          fill: "#0f172a",
          stroke: "#38bdf8",
          textColor: "#f8fafc",
        },
        createdAt: NOW,
        updatedAt: NOW,
      }),
    ],
  });
}

function framedFixtureDocument() {
  return createWhiteboardDocument({
    id: "framed-workflow-board",
    name: "Framed Workflow Board",
    createdAt: NOW,
    updatedAt: NOW,
    elements: [
      createRectangleElement({
        id: "frame",
        role: "frame",
        x: 0,
        y: 0,
        width: 360,
        height: 220,
        createdAt: NOW,
        updatedAt: NOW,
      }),
      createRectangleElement({
        id: "framed-card",
        x: 40,
        y: 48,
        width: 120,
        height: 80,
        metadata: { frameId: "frame" },
        createdAt: NOW,
        updatedAt: NOW,
      }),
    ],
  });
}
