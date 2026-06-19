import {
  DEFAULT_WHITEBOARD_STYLE,
  WHITEBOARD_SCHEMA_VERSION,
  createWhiteboardDocument,
  makeDocumentId,
  makeElementId,
  makeGroupId,
  type WhiteboardDocument,
  type WhiteboardConnectorAnchor,
  type WhiteboardConnectorEndpoint,
  type WhiteboardElement,
  type WhiteboardElementId,
  type WhiteboardElementRole,
  type WhiteboardElementType,
  type WhiteboardGroup,
  type WhiteboardGroupId,
  type WhiteboardMetadata,
  type WhiteboardMetadataValue,
  type WhiteboardPoint,
  type WhiteboardStyle,
  type WhiteboardTool,
} from "../model";
import {
  embeddedImageSourceAt,
  optionalPositiveNumberAt,
} from "./image-fields";

export { WHITEBOARD_SCHEMA_VERSION };
export type {
  WhiteboardDocument,
  WhiteboardElement,
  WhiteboardElementId,
  WhiteboardElementType,
  WhiteboardGroup,
  WhiteboardGroupId,
  WhiteboardPoint,
  WhiteboardStyle,
};

export const WHITEBOARD_DOCUMENT_SCHEMA = "dx.whiteboard.document";

type RecordValue = Record<string, unknown>;

const TOOL_NAMES = new Set<WhiteboardTool>([
  "select",
  "pan",
  "hand",
  "pen",
  "freehand",
  "rectangle",
  "ellipse",
  "diamond",
  "line",
  "arrow",
  "text",
  "eraser",
]);

const ELEMENT_ROLES = new Set<WhiteboardElementRole>([
  "frame",
  "sticky-note",
  "checklist",
  "label",
  "shape",
  "connector",
  "image",
]);

const CONNECTOR_ANCHORS = new Set<WhiteboardConnectorAnchor>([
  "auto",
  "center",
  "top",
  "right",
  "bottom",
  "left",
]);

function isRecord(value: unknown): value is RecordValue {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function recordAt(value: unknown, path: string): RecordValue {
  if (!isRecord(value)) {
    throw new Error(`${path} must be an object`);
  }

  return value;
}

function stringAt(value: unknown, path: string): string {
  if (typeof value !== "string" || value.trim().length === 0) {
    throw new Error(`${path} must be a non-empty string`);
  }

  return value;
}

function numberAt(value: unknown, path: string, fallback?: number): number {
  if (value === undefined && fallback !== undefined) {
    return fallback;
  }

  if (typeof value !== "number" || !Number.isFinite(value)) {
    throw new Error(`${path} must be a finite number`);
  }

  return value;
}

function timestampAt(value: unknown, path: string, fallback?: string): string {
  const timestamp = value === undefined && fallback ? fallback : stringAt(value, path);

  if (Number.isNaN(Date.parse(timestamp))) {
    throw new Error(`${path} must be an ISO timestamp`);
  }

  return timestamp;
}

function metadataFrom(value: unknown): WhiteboardMetadata | undefined {
  if (!isRecord(value)) {
    return undefined;
  }

  const metadata: Record<string, WhiteboardMetadataValue> = {};
  for (const [key, entry] of Object.entries(value)) {
    if (entry === null || typeof entry === "string" || typeof entry === "number" || typeof entry === "boolean") {
      metadata[key] = entry;
    }
  }

  return metadata;
}

function styleFrom(value: unknown): WhiteboardStyle {
  const source = isRecord(value) ? value : {};
  return {
    ...DEFAULT_WHITEBOARD_STYLE,
    fill: typeof source.fill === "string" ? source.fill : DEFAULT_WHITEBOARD_STYLE.fill,
    stroke: typeof source.stroke === "string" ? source.stroke : DEFAULT_WHITEBOARD_STYLE.stroke,
    strokeWidth: numberAt(source.strokeWidth, "element.style.strokeWidth", DEFAULT_WHITEBOARD_STYLE.strokeWidth),
    strokeStyle:
      source.strokeStyle === "solid" || source.strokeStyle === "dashed" || source.strokeStyle === "dotted"
        ? source.strokeStyle
        : DEFAULT_WHITEBOARD_STYLE.strokeStyle,
    opacity: clamp(numberAt(source.opacity, "element.style.opacity", DEFAULT_WHITEBOARD_STYLE.opacity), 0, 1),
    lineCap:
      source.lineCap === "round" || source.lineCap === "square" || source.lineCap === "butt"
        ? source.lineCap
        : DEFAULT_WHITEBOARD_STYLE.lineCap,
    fontFamily: typeof source.fontFamily === "string" ? source.fontFamily : DEFAULT_WHITEBOARD_STYLE.fontFamily,
    fontSize: numberAt(source.fontSize, "element.style.fontSize", DEFAULT_WHITEBOARD_STYLE.fontSize),
    textColor: typeof source.textColor === "string" ? source.textColor : DEFAULT_WHITEBOARD_STYLE.textColor,
  };
}

function roleFrom(value: unknown): WhiteboardElementRole | undefined {
  return typeof value === "string" && ELEMENT_ROLES.has(value as WhiteboardElementRole)
    ? (value as WhiteboardElementRole)
    : undefined;
}

function connectorEndpointFrom(value: unknown, path: string): WhiteboardConnectorEndpoint | undefined {
  if (value === undefined || value === null) return undefined;
  const source = recordAt(value, path);
  const anchor = typeof source.anchor === "string" && CONNECTOR_ANCHORS.has(source.anchor as WhiteboardConnectorAnchor)
    ? (source.anchor as WhiteboardConnectorAnchor)
    : undefined;

  return {
    elementId: makeElementId(stringAt(source.elementId, `${path}.elementId`)),
    anchor,
  };
}

function pointsFrom(value: unknown, path: string): readonly WhiteboardPoint[] {
  if (!Array.isArray(value)) {
    return [];
  }

  return value.map((point, index) => {
    const source = recordAt(point, `${path}[${index}]`);
    return {
      x: numberAt(source.x, `${path}[${index}].x`),
      y: numberAt(source.y, `${path}[${index}].y`),
    };
  });
}

function normalizeElement(value: unknown, path: string): WhiteboardElement {
  const source = recordAt(value, path);
  const metadata = metadataFrom(source.metadata);
  const metadataRole = metadata?.role;
  const rawType = stringAt(source.type ?? source.kind, `${path}.type`);
  const type = rawType === "stroke" ? "freehand" : rawType;
  const createdAt = timestampAt(source.createdAt, `${path}.createdAt`, "1970-01-01T00:00:00.000Z");
  const updatedAt = timestampAt(source.updatedAt, `${path}.updatedAt`, createdAt);
  const base = {
    id: makeElementId(stringAt(source.id, `${path}.id`)),
    role: roleFrom(source.role ?? metadataRole),
    name: typeof source.name === "string" ? source.name : undefined,
    groupId: typeof source.groupId === "string" ? makeGroupId(source.groupId) : undefined,
    locked: Boolean(source.locked),
    hidden: Boolean(source.hidden),
    style: styleFrom(source.style ?? source),
    createdAt,
    updatedAt,
    metadata,
  };

  if (type === "rectangle" || type === "ellipse" || type === "diamond") {
    return {
      ...base,
      type,
      x: numberAt(source.x, `${path}.x`, 0),
      y: numberAt(source.y, `${path}.y`, 0),
      width: numberAt(source.width, `${path}.width`, 0),
      height: numberAt(source.height, `${path}.height`, 0),
      rotation: numberAt(source.rotation, `${path}.rotation`, 0),
      ...(source.radius !== undefined ? { radius: numberAt(source.radius, `${path}.radius`, 0) } : {}),
    };
  }

  if (type === "text") {
    return {
      ...base,
      type,
      x: numberAt(source.x, `${path}.x`, 0),
      y: numberAt(source.y, `${path}.y`, 0),
      width: numberAt(source.width, `${path}.width`, 180),
      height: numberAt(source.height, `${path}.height`, 48),
      rotation: numberAt(source.rotation, `${path}.rotation`, 0),
      text: typeof source.text === "string" ? source.text : "",
      textAlign:
        source.textAlign === "center" || source.textAlign === "right" ? source.textAlign : "left",
      verticalAlign:
        source.verticalAlign === "middle" || source.verticalAlign === "bottom" ? source.verticalAlign : "top",
    };
  }

  if (type === "image") {
    return {
      ...base,
      type,
      role: base.role ?? "image",
      x: numberAt(source.x, `${path}.x`, 0),
      y: numberAt(source.y, `${path}.y`, 0),
      width: numberAt(source.width, `${path}.width`, 240),
      height: numberAt(source.height, `${path}.height`, 160),
      rotation: numberAt(source.rotation, `${path}.rotation`, 0),
      src: embeddedImageSourceAt(source.src, `${path}.src`),
      alt: stringAt(source.alt, `${path}.alt`),
      naturalWidth: optionalPositiveNumberAt(source.naturalWidth, `${path}.naturalWidth`),
      naturalHeight: optionalPositiveNumberAt(source.naturalHeight, `${path}.naturalHeight`),
    };
  }

  if (type === "line" || type === "arrow") {
    const points = pointsFrom(source.points, `${path}.points`);
    if (points.length < 2) {
      throw new Error(`${path}.points must include at least two points`);
    }

    const [first, second, ...rest] = points;
    return {
      ...base,
      type,
      points: [first, second, ...rest],
      startBinding: connectorEndpointFrom(source.startBinding, `${path}.startBinding`),
      endBinding: connectorEndpointFrom(source.endBinding, `${path}.endBinding`),
      startArrow: source.startArrow === "triangle" ? "triangle" : "none",
      endArrow: source.endArrow === "none" || source.endArrow === "triangle"
        ? source.endArrow
        : type === "arrow"
          ? "triangle"
          : "none",
    };
  }

  if (type === "freehand" || type === "path") {
    return {
      ...base,
      type,
      points: pointsFrom(source.points, `${path}.points`),
      ...(type === "path" ? { closed: Boolean(source.closed) } : {}),
    } as WhiteboardElement;
  }

  throw new Error(`${path}.type must be a supported whiteboard element type`);
}

function normalizeGroup(value: unknown, path: string): WhiteboardGroup {
  const source = recordAt(value, path);
  const createdAt = timestampAt(source.createdAt, `${path}.createdAt`, "1970-01-01T00:00:00.000Z");
  const updatedAt = timestampAt(source.updatedAt, `${path}.updatedAt`, createdAt);
  const elementIds = Array.isArray(source.elementIds)
    ? source.elementIds.map((id, index) => makeElementId(stringAt(id, `${path}.elementIds[${index}]`)))
    : [];

  return {
    id: makeGroupId(stringAt(source.id, `${path}.id`)),
    name: typeof source.name === "string" ? source.name : undefined,
    elementIds,
    createdAt,
    updatedAt,
    metadata: metadataFrom(source.metadata),
  };
}

export function normalizeWhiteboardElement(value: unknown, path = "element"): WhiteboardElement {
  return normalizeElement(value, path);
}

export function validateWhiteboardDocument(value: unknown): WhiteboardDocument {
  const source = recordAt(value, "document");
  const schemaVersion = numberAt(source.schemaVersion ?? source.version, "document.schemaVersion", WHITEBOARD_SCHEMA_VERSION);

  if (schemaVersion !== WHITEBOARD_SCHEMA_VERSION) {
    throw new Error(`document.schemaVersion must be ${WHITEBOARD_SCHEMA_VERSION}`);
  }

  const normalizedElements = Array.isArray(source.elements)
    ? source.elements.map((element, index) => normalizeElement(element, `document.elements[${index}]`))
    : [];
  const existingElementIds = new Set(normalizedElements.map((element) => element.id));
  const groups = Array.isArray(source.groups)
    ? source.groups
        .map((group, index) => normalizeGroup(group, `document.groups[${index}]`))
        .map((group) => ({
          ...group,
          elementIds: group.elementIds.filter((id) => existingElementIds.has(id)),
        }))
        .filter((group) => group.elementIds.length > 1)
    : undefined;
  const groupByElementId = new Map<WhiteboardElementId, WhiteboardGroupId>();

  for (const group of groups ?? []) {
    for (const elementId of group.elementIds) {
      if (!groupByElementId.has(elementId)) {
        groupByElementId.set(elementId, group.id);
      }
    }
  }

  const elements = normalizedElements.map((element) => {
    const groupId = groupByElementId.get(element.id);
    return element.groupId === groupId ? element : { ...element, groupId };
  });
  const elementIds = new Set(elements.map((element) => element.id));
  const selection = Array.isArray(source.selection)
    ? source.selection
        .map((id) => makeElementId(String(id)))
        .filter((id) => elementIds.has(id))
    : [];
  const activeTool = typeof source.activeTool === "string" && TOOL_NAMES.has(source.activeTool as WhiteboardTool)
    ? (source.activeTool as WhiteboardTool)
    : "select";
  const viewport = recordAt(source.viewport ?? {}, "document.viewport");

  return createWhiteboardDocument({
    id: makeDocumentId(stringAt(source.id, "document.id")),
    name: typeof source.name === "string" ? source.name : stringAt(source.title, "document.title"),
    elements,
    groups,
    selection,
    activeTool,
    viewport: {
      x: numberAt(viewport.x, "document.viewport.x", 0),
      y: numberAt(viewport.y, "document.viewport.y", 0),
      zoom: numberAt(viewport.zoom, "document.viewport.zoom", 1),
    },
    currentStyle: styleFrom(source.currentStyle),
    createdAt: timestampAt(source.createdAt, "document.createdAt", "1970-01-01T00:00:00.000Z"),
    updatedAt: timestampAt(source.updatedAt, "document.updatedAt", "1970-01-01T00:00:00.000Z"),
    metadata: metadataFrom(source.metadata),
  });
}

export function migrateWhiteboardDocument(value: unknown): WhiteboardDocument {
  const source = recordAt(value, "document");

  if (source.schema === WHITEBOARD_DOCUMENT_SCHEMA) {
    return validateWhiteboardDocument({
      ...source,
      schemaVersion: WHITEBOARD_SCHEMA_VERSION,
      name: typeof source.name === "string" ? source.name : source.title,
      viewport: source.viewport,
    });
  }

  return validateWhiteboardDocument(source);
}

function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, value));
}
