import assert from "node:assert/strict";
import { mkdtemp, readFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, it } from "node:test";

import {
  WHITEBOARD_SCHEMA_VERSION,
  migrateWhiteboardDocument,
  validateWhiteboardDocument,
} from "../examples/whiteboard/lib/whiteboard/persistence/schema.ts";
import { connectorRouteForElement } from "../examples/whiteboard/lib/whiteboard/connector-routes.ts";
import { createImportedImageElement } from "../examples/whiteboard/lib/whiteboard/image-import.ts";
import {
  DEFAULT_WHITEBOARD_STYLE,
  createWhiteboardDocument as createModelWhiteboardDocument,
  makeElementId,
  makeGroupId,
} from "../examples/whiteboard/lib/whiteboard/model.ts";
import {
  createLocalFirstWhiteboardStorageDriver,
  createMemoryWhiteboardKeyValueStore,
} from "../examples/whiteboard/lib/whiteboard/persistence/local-first.ts";
import { createWhiteboardDocument } from "../examples/whiteboard/server/whiteboard/commands.ts";
import { createWhiteboardFileStorageDriver } from "../examples/whiteboard/server/whiteboard/filesystem-storage.ts";

const EMBEDDED_IMAGE_SRC =
  "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 640 400'%3E%3Crect width='640' height='400' fill='%23111827'/%3E%3C/svg%3E";

describe("whiteboard schema validation and migration", () => {
  it("migrates legacy v1 documents into the current validated schema", () => {
    const legacy = {
      schema: "dx.whiteboard.document",
      version: 1,
      id: "legacy-board",
      name: "Legacy Sketch",
      createdAt: "2026-06-01T00:00:00.000Z",
      updatedAt: "2026-06-01T00:01:00.000Z",
      elements: [
        {
          id: "legacy-note",
          kind: "text",
          x: 10,
          y: 20,
          width: 160,
          height: 48,
          text: "old but important",
        },
      ],
    };

    const migrated = migrateWhiteboardDocument(legacy);

    assert.equal(migrated.schemaVersion, WHITEBOARD_SCHEMA_VERSION);
    assert.equal(migrated.name, "Legacy Sketch");
    assert.equal(migrated.elements[0]?.type, "text");
    assert.equal(migrated.elements[0]?.text, "old but important");
    assert.doesNotThrow(() => validateWhiteboardDocument(migrated));
  });

  it("reports precise validation errors for invalid element shapes", () => {
    const document = createWhiteboardDocument({
      id: "invalid-board",
      name: "Invalid Board",
      now: () => "2026-06-02T00:00:00.000Z",
    });

    assert.throws(
      () =>
        validateWhiteboardDocument({
          ...document,
          elements: [
            {
              id: "bad-rect",
              type: "rectangle",
              x: "not-a-number",
              y: 0,
              width: 100,
              height: 100,
              rotation: 0,
              style: {
                fill: "#ffffff",
                stroke: "#111827",
                strokeWidth: 1,
                opacity: 1,
              },
            },
          ],
        }),
      /elements\[0\]\.x/,
    );
  });

  it("drops stale selected ids during schema validation", () => {
    const document = createModelWhiteboardDocument({
      id: "selection-cleanup-board",
      name: "Selection Cleanup",
      createdAt: "2026-06-03T00:00:00.000Z",
      updatedAt: "2026-06-03T00:00:00.000Z",
      selection: ["visible-note", "missing-note"],
      elements: [
        {
          id: makeElementId("visible-note"),
          type: "text",
          x: 10,
          y: 20,
          width: 180,
          height: 64,
          rotation: 0,
          text: "Keep this selection",
          textAlign: "left",
          verticalAlign: "top",
          style: DEFAULT_WHITEBOARD_STYLE,
          locked: false,
          hidden: false,
          createdAt: "2026-06-03T00:00:00.000Z",
          updatedAt: "2026-06-03T00:00:00.000Z",
        },
      ],
    });

    const validated = validateWhiteboardDocument(JSON.parse(JSON.stringify(document)));

    assert.deepEqual(validated.selection, [makeElementId("visible-note")]);
  });

  it("preserves source-owned image elements through schema validation", () => {
    const document = createModelWhiteboardDocument({
      id: "image-board",
      name: "Image Board",
      createdAt: "2026-06-03T00:00:00.000Z",
      updatedAt: "2026-06-03T00:00:00.000Z",
      elements: [
        {
          id: makeElementId("hero-image"),
          type: "image",
          role: "image",
          name: "Hero image",
          x: 10,
          y: 20,
          width: 320,
          height: 200,
          rotation: 0,
          src: EMBEDDED_IMAGE_SRC,
          alt: "DX hero image",
          naturalWidth: 640,
          naturalHeight: 400,
          style: DEFAULT_WHITEBOARD_STYLE,
          locked: false,
          hidden: false,
          createdAt: "2026-06-03T00:00:00.000Z",
          updatedAt: "2026-06-03T00:00:00.000Z",
        },
      ],
    });

    const validated = validateWhiteboardDocument(JSON.parse(JSON.stringify(document)));
    const image = validated.elements[0];

    assert.equal(image?.type, "image");
    assert.equal(image?.type === "image" ? image.src : "", EMBEDDED_IMAGE_SRC);
    assert.equal(image?.type === "image" ? image.alt : "", "DX hero image");
    assert.equal(image?.type === "image" ? image.naturalWidth : null, 640);
    assert.equal(image?.type === "image" ? image.naturalHeight : null, 400);
  });

  it("rejects image elements without a source", () => {
    const document = createModelWhiteboardDocument({
      id: "invalid-image-board",
      elements: [
        {
          id: makeElementId("missing-source"),
          type: "image",
          x: 0,
          y: 0,
          width: 100,
          height: 100,
          rotation: 0,
          src: "",
          alt: "Missing",
          style: DEFAULT_WHITEBOARD_STYLE,
          locked: false,
          hidden: false,
          createdAt: "2026-06-03T00:00:00.000Z",
          updatedAt: "2026-06-03T00:00:00.000Z",
        },
      ],
    });

    assert.throws(() => validateWhiteboardDocument(JSON.parse(JSON.stringify(document))), /elements\[0\]\.src/);
  });

  it("rejects non-embedded image sources and invalid intrinsic image sizes", () => {
    const baseImage = {
      id: makeElementId("unsafe-image"),
      type: "image",
      x: 0,
      y: 0,
      width: 100,
      height: 100,
      rotation: 0,
      src: EMBEDDED_IMAGE_SRC,
      alt: "Unsafe",
      style: DEFAULT_WHITEBOARD_STYLE,
      locked: false,
      hidden: false,
      createdAt: "2026-06-03T00:00:00.000Z",
      updatedAt: "2026-06-03T00:00:00.000Z",
    };
    const document = createModelWhiteboardDocument({
      id: "invalid-image-policy-board",
      elements: [baseImage],
    });

    for (const src of [
      "https://example.com/hero.svg",
      "blob:https://example.com/hero",
      "file:///C:/hero.svg",
      "data:text/plain,not-an-image",
    ]) {
      assert.throws(
        () =>
          validateWhiteboardDocument({
            ...document,
            elements: [{ ...baseImage, src }],
          }),
        /data:image/,
      );
    }

    assert.throws(
      () =>
        validateWhiteboardDocument({
          ...document,
          elements: [{ ...baseImage, naturalWidth: 0 }],
        }),
      /naturalWidth/,
    );
    assert.throws(
      () =>
        validateWhiteboardDocument({
          ...document,
          elements: [{ ...baseImage, alt: "" }],
        }),
      /alt/,
    );
  });

  it("preserves semantic roles, groups, connector bindings, and text layout fields", () => {
    const document = createModelWhiteboardDocument({
      id: "semantic-board",
      name: "Semantic Board",
      createdAt: "2026-06-03T00:00:00.000Z",
      updatedAt: "2026-06-03T00:01:00.000Z",
      groups: [
        {
          id: makeGroupId("group-alpha"),
          name: "Frame group",
          elementIds: [makeElementId("frame"), makeElementId("label")],
          createdAt: "2026-06-03T00:00:00.000Z",
          updatedAt: "2026-06-03T00:01:00.000Z",
          metadata: { owner: "design" },
        },
      ],
      elements: [
        {
          id: makeElementId("frame"),
          type: "rectangle",
          role: "frame",
          groupId: makeGroupId("group-alpha"),
          name: "Frame",
          x: 0,
          y: 0,
          width: 300,
          height: 180,
          rotation: 0,
          style: {
            ...DEFAULT_WHITEBOARD_STYLE,
            strokeStyle: "dashed",
            lineCap: "square",
          },
          locked: false,
          hidden: false,
          createdAt: "2026-06-03T00:00:00.000Z",
          updatedAt: "2026-06-03T00:01:00.000Z",
          metadata: { preset: "frame", role: "frame" },
        },
        {
          id: makeElementId("label"),
          type: "text",
          role: "label",
          groupId: makeGroupId("group-alpha"),
          x: 24,
          y: 24,
          width: 160,
          height: 64,
          rotation: 0,
          text: "Frame title",
          textAlign: "center",
          verticalAlign: "middle",
          style: DEFAULT_WHITEBOARD_STYLE,
          locked: false,
          hidden: false,
          createdAt: "2026-06-03T00:00:00.000Z",
          updatedAt: "2026-06-03T00:01:00.000Z",
          metadata: { frameId: "frame", purpose: "title" },
        },
        {
          id: makeElementId("arrow"),
          type: "arrow",
          role: "connector",
          points: [
            { x: 12, y: 12 },
            { x: 120, y: 80 },
          ],
          startBinding: { elementId: makeElementId("frame"), anchor: "auto" },
          endBinding: { elementId: makeElementId("label"), anchor: "center" },
          startArrow: "none",
          endArrow: "triangle",
          style: DEFAULT_WHITEBOARD_STYLE,
          locked: false,
          hidden: false,
          createdAt: "2026-06-03T00:00:00.000Z",
          updatedAt: "2026-06-03T00:01:00.000Z",
        },
      ],
    });

    const validated = validateWhiteboardDocument(JSON.parse(JSON.stringify(document)));

    assert.equal(validated.groups?.[0]?.id, makeGroupId("group-alpha"));
    assert.equal(validated.groups?.[0]?.metadata?.owner, "design");
    assert.equal(validated.elements[0]?.role, "frame");
    assert.equal(validated.elements[0]?.groupId, makeGroupId("group-alpha"));
    assert.equal(validated.elements[0]?.style.strokeStyle, "dashed");
    assert.equal(validated.elements[0]?.style.lineCap, "square");
    assert.equal(validated.elements[1]?.type === "text" ? validated.elements[1].textAlign : "", "center");
    assert.equal(validated.elements[1]?.type === "text" ? validated.elements[1].verticalAlign : "", "middle");
    assert.equal(validated.elements[1]?.metadata?.frameId, "frame");
    assert.equal(validated.elements[1]?.metadata?.purpose, "title");
    assert.equal(validated.elements[1]?.groupId, makeGroupId("group-alpha"));
    assert.equal(validated.elements[2]?.type === "arrow" ? validated.elements[2].startBinding?.elementId : "", "frame");
    assert.equal(validated.elements[2]?.type === "arrow" ? validated.elements[2].endBinding?.anchor : "", "center");
  });

  it("preserves connector route metadata through schema validation", () => {
    const document = createModelWhiteboardDocument({
      id: "connector-route-board",
      name: "Connector Route Board",
      createdAt: "2026-06-03T00:00:00.000Z",
      updatedAt: "2026-06-03T00:01:00.000Z",
      elements: [
        {
          id: makeElementId("route-arrow"),
          type: "arrow",
          role: "connector",
          points: [
            { x: 20, y: 40 },
            { x: 100, y: 40 },
            { x: 100, y: 120 },
            { x: 180, y: 120 },
          ],
          startArrow: "none",
          endArrow: "triangle",
          style: DEFAULT_WHITEBOARD_STYLE,
          locked: false,
          hidden: false,
          createdAt: "2026-06-03T00:00:00.000Z",
          updatedAt: "2026-06-03T00:01:00.000Z",
          metadata: { connectorRoute: "orthogonal", owner: "diagram" },
        },
      ],
    });

    const validated = validateWhiteboardDocument(JSON.parse(JSON.stringify(document)));
    const connector = validated.elements[0];

    assert.equal(connector?.type, "arrow");
    assert.equal(connector?.metadata?.connectorRoute, "orthogonal");
    assert.equal(connector?.metadata?.owner, "diagram");
    assert.equal(connector?.type === "arrow" ? connectorRouteForElement(connector) : "", "orthogonal");
  });
});

describe("local-first whiteboard storage driver", () => {
  it("saves, lists, loads, migrates, and removes boards from a local key-value store", async () => {
    const storage = createMemoryWhiteboardKeyValueStore();
    const driver = createLocalFirstWhiteboardStorageDriver({
      storage,
      namespace: "test.whiteboard",
      now: () => "2026-06-02T12:00:00.000Z",
    });
    const document = createWhiteboardDocument({
      id: "board-local-first",
      name: "Local First",
      now: () => "2026-06-02T11:59:00.000Z",
    });

    await driver.save(document);
    const listed = await driver.list();
    const loaded = await driver.load("board-local-first");

    assert.equal(listed.length, 1);
    assert.equal(listed[0]?.id, "board-local-first");
    assert.equal(listed[0]?.name, "Local First");
    assert.equal(loaded?.id, "board-local-first");
    assert.equal(loaded?.updatedAt, "2026-06-02T12:00:00.000Z");

    storage.setItem(
      "test.whiteboard:boards:legacy-local",
      JSON.stringify({
        schema: "dx.whiteboard.document",
        version: 1,
        id: "legacy-local",
        name: "Legacy Local",
        elements: [],
      }),
    );
    storage.setItem(
      "test.whiteboard:index",
      JSON.stringify([{ id: "legacy-local", title: "Legacy Local", updatedAt: "2026-06-01T00:00:00.000Z" }]),
    );

    assert.equal((await driver.load("legacy-local"))?.schemaVersion, WHITEBOARD_SCHEMA_VERSION);

    await driver.remove("board-local-first");
    assert.equal(await driver.load("board-local-first"), null);
  });

  it("persists locally imported images through local-first storage with metadata intact", async () => {
    const imported = createImportedImageElement({
      id: "local-first-imported-image",
      fileName: "local-first-image.jpeg",
      fileType: "image/jpeg",
      fileSizeBytes: 4096,
      naturalWidth: 640,
      naturalHeight: 360,
      source: "data:image/jpeg;base64,aW1hZ2U=",
      createdAt: "2026-06-02T11:59:00.000Z",
      updatedAt: "2026-06-02T11:59:00.000Z",
    });
    assert.equal(imported.status, "accepted");
    const storage = createMemoryWhiteboardKeyValueStore();
    const driver = createLocalFirstWhiteboardStorageDriver({
      storage,
      namespace: "test.whiteboard.imported-image",
      now: () => "2026-06-02T12:00:00.000Z",
    });
    const document = createModelWhiteboardDocument({
      id: "local-first-imported-image-board",
      name: "Local First Imported Image",
      createdAt: "2026-06-02T11:59:00.000Z",
      updatedAt: "2026-06-02T11:59:00.000Z",
      selection: ["local-first-imported-image"],
      elements: imported.status === "accepted" ? [imported.element] : [],
    });

    await driver.save(document);
    const loaded = await driver.load("local-first-imported-image-board");
    const image = loaded?.elements[0];

    assert.equal(loaded?.selection[0], "local-first-imported-image");
    assert.equal(image?.type, "image");
    assert.equal(image?.type === "image" ? image.src : "", "data:image/jpeg;base64,aW1hZ2U=");
    assert.equal(image?.type === "image" ? image.alt : "", "local first image");
    assert.equal(image?.type === "image" ? image.naturalWidth : 0, 640);
    assert.equal(image?.type === "image" ? image.metadata?.importSource : "", "local-file");
    assert.equal(image?.type === "image" ? image.metadata?.imagePolicy : "", "embedded-data-url-only");
    assert.equal(image?.type === "image" ? image.metadata?.originalName : "", "local-first-image.jpeg");
  });
});

describe("server whiteboard filesystem storage", () => {
  it("persists boards as .dxdraw files with the same validated schema boundary", async () => {
    const rootDir = await mkdtemp(join(tmpdir(), "dx-whiteboard-"));
    const driver = createWhiteboardFileStorageDriver({
      rootDir,
      now: () => "2026-06-02T13:00:00.000Z",
    });
    const document = createWhiteboardDocument({
      id: "server-board",
      name: "Server Board",
      now: () => "2026-06-02T12:45:00.000Z",
    });

    try {
      await driver.save(document);
      const stored = await readFile(join(rootDir, "server-board.dxdraw"), "utf8");
      const listed = await driver.list();
      const loaded = await driver.load("server-board");

      assert.match(stored, /"format": "dx.whiteboard.dxdraw"/);
      assert.equal(listed[0]?.id, "server-board");
      assert.equal(loaded?.updatedAt, "2026-06-02T13:00:00.000Z");

      await driver.remove("server-board");
      assert.equal(await driver.load("server-board"), null);
    } finally {
      await rm(rootDir, { recursive: true, force: true });
    }
  });

  it("persists locally imported images through filesystem .dxdraw storage without leaking paths", async () => {
    const imported = createImportedImageElement({
      id: "filesystem-imported-image",
      fileName: "filesystem-image.webp",
      fileType: "image/webp",
      fileSizeBytes: 2048,
      naturalWidth: 512,
      naturalHeight: 256,
      source: "data:image/webp;base64,aW1hZ2U=",
      createdAt: "2026-06-02T12:45:00.000Z",
      updatedAt: "2026-06-02T12:45:00.000Z",
    });
    assert.equal(imported.status, "accepted");
    const rootDir = await mkdtemp(join(tmpdir(), "dx-whiteboard-imported-image-"));
    const driver = createWhiteboardFileStorageDriver({
      rootDir,
      now: () => "2026-06-02T13:00:00.000Z",
    });
    const document = createModelWhiteboardDocument({
      id: "filesystem-imported-image-board",
      name: "Filesystem Imported Image",
      createdAt: "2026-06-02T12:45:00.000Z",
      updatedAt: "2026-06-02T12:45:00.000Z",
      elements: imported.status === "accepted" ? [imported.element] : [],
    });

    try {
      await driver.save(document);
      const stored = await readFile(join(rootDir, "filesystem-imported-image-board.dxdraw"), "utf8");
      const loaded = await driver.load("filesystem-imported-image-board");
      const image = loaded?.elements[0];

      assert.match(stored, /"format": "dx.whiteboard.dxdraw"/);
      assert.doesNotMatch(stored, new RegExp(rootDir.replace(/[\\^$.*+?()[\]{}|]/g, "\\$&")));
      assert.equal(image?.type, "image");
      assert.equal(image?.type === "image" ? image.src : "", "data:image/webp;base64,aW1hZ2U=");
      assert.equal(image?.type === "image" ? image.metadata?.importSource : "", "local-file");
      assert.equal(image?.type === "image" ? image.metadata?.imagePolicy : "", "embedded-data-url-only");
    } finally {
      await rm(rootDir, { recursive: true, force: true });
    }
  });
});
