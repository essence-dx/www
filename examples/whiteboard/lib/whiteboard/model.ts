declare const whiteboardDocumentIdBrand: unique symbol;
declare const whiteboardElementIdBrand: unique symbol;
declare const whiteboardGroupIdBrand: unique symbol;

export type WhiteboardDocumentId = string & {
  readonly [whiteboardDocumentIdBrand]: true;
};

export type WhiteboardElementId = string & {
  readonly [whiteboardElementIdBrand]: true;
};

export type WhiteboardGroupId = string & {
  readonly [whiteboardGroupIdBrand]: true;
};

export type WhiteboardIsoDate = string;

export type WhiteboardTool =
  | "select"
  | "pan"
  | "hand"
  | "pen"
  | "freehand"
  | "rectangle"
  | "ellipse"
  | "diamond"
  | "line"
  | "arrow"
  | "text"
  | "eraser";

export type WhiteboardSelectionMode = "replace" | "extend" | "toggle";
export type WhiteboardStrokeStyle = "solid" | "dashed" | "dotted";
export type WhiteboardLineCap = "round" | "square" | "butt";
export type WhiteboardTextAlign = "left" | "center" | "right";
export type WhiteboardVerticalAlign = "top" | "middle" | "bottom";
export type WhiteboardMetadataValue = string | number | boolean | null;
export type WhiteboardMetadata = Readonly<Record<string, WhiteboardMetadataValue>>;
export type WhiteboardElementRole =
  | "frame"
  | "sticky-note"
  | "checklist"
  | "label"
  | "shape"
  | "connector"
  | "image";
export type WhiteboardConnectorAnchor = "auto" | "center" | "top" | "right" | "bottom" | "left";

export interface WhiteboardPoint {
  readonly x: number;
  readonly y: number;
}

export interface WhiteboardSize {
  readonly width: number;
  readonly height: number;
}

export interface WhiteboardRect extends WhiteboardPoint, WhiteboardSize {}

export interface WhiteboardConnectorEndpoint {
  readonly elementId: WhiteboardElementId;
  readonly anchor?: WhiteboardConnectorAnchor;
}

export interface WhiteboardViewport {
  readonly x: number;
  readonly y: number;
  readonly zoom: number;
}

export interface WhiteboardStyle {
  readonly fill: string;
  readonly stroke: string;
  readonly strokeWidth: number;
  readonly strokeStyle: WhiteboardStrokeStyle;
  readonly opacity: number;
  readonly lineCap: WhiteboardLineCap;
  readonly fontFamily: string;
  readonly fontSize: number;
  readonly textColor: string;
}

export interface WhiteboardElementBase {
  readonly id: WhiteboardElementId;
  readonly type: WhiteboardElementType;
  readonly role?: WhiteboardElementRole;
  readonly name?: string;
  readonly groupId?: WhiteboardGroupId;
  readonly locked: boolean;
  readonly hidden: boolean;
  readonly style: WhiteboardStyle;
  readonly createdAt: WhiteboardIsoDate;
  readonly updatedAt: WhiteboardIsoDate;
  readonly metadata?: WhiteboardMetadata;
}

export interface WhiteboardBoxElementBase extends WhiteboardElementBase {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
  readonly rotation: number;
}

export interface WhiteboardRectangleElement extends WhiteboardBoxElementBase {
  readonly type: "rectangle";
  readonly radius?: number;
}

export interface WhiteboardEllipseElement extends WhiteboardBoxElementBase {
  readonly type: "ellipse";
}

export interface WhiteboardDiamondElement extends WhiteboardBoxElementBase {
  readonly type: "diamond";
}

export interface WhiteboardTextElement extends WhiteboardBoxElementBase {
  readonly type: "text";
  readonly text: string;
  readonly textAlign: WhiteboardTextAlign;
  readonly verticalAlign: WhiteboardVerticalAlign;
}

export interface WhiteboardImageElement extends WhiteboardBoxElementBase {
  readonly type: "image";
  readonly src: string;
  readonly alt: string;
  readonly naturalWidth?: number;
  readonly naturalHeight?: number;
}

export interface WhiteboardPathElement extends WhiteboardElementBase {
  readonly type: "path";
  readonly points: readonly WhiteboardPoint[];
  readonly closed: boolean;
}

export interface WhiteboardFreehandElement extends WhiteboardElementBase {
  readonly type: "freehand";
  readonly points: readonly WhiteboardPoint[];
}

export interface WhiteboardConnectorElement extends WhiteboardElementBase {
  readonly type: "line" | "arrow";
  readonly points: readonly [WhiteboardPoint, WhiteboardPoint, ...WhiteboardPoint[]];
  readonly startBinding?: WhiteboardConnectorEndpoint;
  readonly endBinding?: WhiteboardConnectorEndpoint;
  readonly startArrow?: "none" | "triangle";
  readonly endArrow?: "none" | "triangle";
}

export type WhiteboardBoxElement =
  | WhiteboardRectangleElement
  | WhiteboardEllipseElement
  | WhiteboardDiamondElement
  | WhiteboardTextElement
  | WhiteboardImageElement;

export type WhiteboardElement =
  | WhiteboardBoxElement
  | WhiteboardPathElement
  | WhiteboardFreehandElement
  | WhiteboardConnectorElement;

export type WhiteboardElementType = WhiteboardElement["type"];

export type WhiteboardElementPatch = Partial<{
  readonly role: WhiteboardElementRole;
  readonly name: string;
  readonly groupId: WhiteboardGroupId | undefined;
  readonly locked: boolean;
  readonly hidden: boolean;
  readonly style: Partial<WhiteboardStyle>;
  readonly metadata: WhiteboardMetadata;
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
  readonly rotation: number;
  readonly radius: number;
  readonly text: string;
  readonly textAlign: WhiteboardTextAlign;
  readonly verticalAlign: WhiteboardVerticalAlign;
  readonly src: string;
  readonly alt: string;
  readonly naturalWidth: number | undefined;
  readonly naturalHeight: number | undefined;
  readonly points: readonly WhiteboardPoint[];
  readonly startBinding: WhiteboardConnectorEndpoint | undefined;
  readonly endBinding: WhiteboardConnectorEndpoint | undefined;
  readonly closed: boolean;
  readonly startArrow: "none" | "triangle";
  readonly endArrow: "none" | "triangle";
}>;

export interface WhiteboardGroup {
  readonly id: WhiteboardGroupId;
  readonly name?: string;
  readonly elementIds: readonly WhiteboardElementId[];
  readonly createdAt: WhiteboardIsoDate;
  readonly updatedAt: WhiteboardIsoDate;
  readonly metadata?: WhiteboardMetadata;
}

export type WhiteboardGroupPatch = Partial<{
  readonly name: string;
  readonly metadata: WhiteboardMetadata;
}>;

export interface WhiteboardDocument {
  readonly schemaVersion: 1;
  readonly id: WhiteboardDocumentId;
  readonly name: string;
  readonly elements: readonly WhiteboardElement[];
  readonly groups?: readonly WhiteboardGroup[];
  readonly selection: readonly WhiteboardElementId[];
  readonly activeTool: WhiteboardTool;
  readonly viewport: WhiteboardViewport;
  readonly currentStyle: WhiteboardStyle;
  readonly createdAt: WhiteboardIsoDate;
  readonly updatedAt: WhiteboardIsoDate;
  readonly metadata?: WhiteboardMetadata;
}

export interface CreateWhiteboardDocumentOptions {
  readonly id?: string | WhiteboardDocumentId;
  readonly name?: string;
  readonly elements?: readonly WhiteboardElement[];
  readonly groups?: readonly WhiteboardGroup[];
  readonly selection?: readonly (string | WhiteboardElementId)[];
  readonly activeTool?: WhiteboardTool;
  readonly viewport?: Partial<WhiteboardViewport>;
  readonly currentStyle?: Partial<WhiteboardStyle>;
  readonly createdAt?: WhiteboardIsoDate;
  readonly updatedAt?: WhiteboardIsoDate;
  readonly metadata?: WhiteboardMetadata;
}

export const WHITEBOARD_SCHEMA_VERSION = 1;

export const DEFAULT_WHITEBOARD_VIEWPORT: WhiteboardViewport = {
  x: 0,
  y: 0,
  zoom: 1,
};

export const DEFAULT_WHITEBOARD_STYLE: WhiteboardStyle = {
  fill: "transparent",
  stroke: "#1f2937",
  strokeWidth: 2,
  strokeStyle: "solid",
  opacity: 1,
  lineCap: "round",
  fontFamily: "Inter, ui-sans-serif, system-ui, sans-serif",
  fontSize: 16,
  textColor: "#111827",
};

export function makeDocumentId(value: string): WhiteboardDocumentId {
  return stableId(value, "whiteboard document") as WhiteboardDocumentId;
}

export function makeElementId(value: string): WhiteboardElementId {
  return stableId(value, "whiteboard element") as WhiteboardElementId;
}

export function makeGroupId(value: string): WhiteboardGroupId {
  return stableId(value, "whiteboard group") as WhiteboardGroupId;
}

export function createWhiteboardDocument(
  options: CreateWhiteboardDocumentOptions = {},
): WhiteboardDocument {
  const createdAt = options.createdAt ?? new Date().toISOString();
  const updatedAt = options.updatedAt ?? createdAt;

  return {
    schemaVersion: WHITEBOARD_SCHEMA_VERSION,
    id: makeDocumentId(options.id ?? createRandomId("whiteboard-doc")),
    name: options.name?.trim() || "Untitled whiteboard",
    elements: options.elements ?? [],
    groups: options.groups,
    selection: options.selection?.map((id) => makeElementId(id)) ?? [],
    activeTool: options.activeTool ?? "select",
    viewport: {
      ...DEFAULT_WHITEBOARD_VIEWPORT,
      ...options.viewport,
    },
    currentStyle: {
      ...DEFAULT_WHITEBOARD_STYLE,
      ...options.currentStyle,
    },
    createdAt,
    updatedAt,
    metadata: options.metadata,
  };
}

function stableId(value: string, label: string): string {
  const id = value.trim();
  if (!id) {
    throw new Error(`A ${label} id cannot be empty.`);
  }
  return id;
}

function createRandomId(prefix: string): string {
  const cryptoApi = globalThis.crypto as
    | { readonly randomUUID?: () => string }
    | undefined;
  const token =
    cryptoApi?.randomUUID?.() ??
    `${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}`;

  return `${prefix}-${token}`;
}
