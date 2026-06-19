import {
  DEFAULT_WHITEBOARD_STYLE,
  makeElementId,
  type WhiteboardElement,
  type WhiteboardElementId,
  type WhiteboardElementRole,
  type WhiteboardGroupId,
  type WhiteboardMetadata,
  type WhiteboardStyle,
} from "./model";
import { assertEmbeddedImageSource } from "./image-source";

export interface CreateElementBaseOptions {
  readonly id: string | WhiteboardElementId;
  readonly role?: WhiteboardElementRole;
  readonly name?: string;
  readonly groupId?: WhiteboardGroupId;
  readonly locked?: boolean;
  readonly hidden?: boolean;
  readonly style?: Partial<WhiteboardStyle>;
  readonly createdAt?: string;
  readonly updatedAt?: string;
  readonly metadata?: WhiteboardMetadata;
}

export interface CreateBoxElementOptions extends CreateElementBaseOptions {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
  readonly rotation?: number;
}

export interface CreateTextElementOptions extends CreateBoxElementOptions {
  readonly text: string;
}

export interface CreateImageElementOptions extends CreateBoxElementOptions {
  readonly src: string;
  readonly alt: string;
  readonly naturalWidth?: number;
  readonly naturalHeight?: number;
}

type CreatedBaseElement<TType extends WhiteboardElement["type"]> = {
  readonly id: WhiteboardElementId;
  readonly type: TType;
  readonly role?: WhiteboardElementRole;
  readonly name?: string;
  readonly groupId?: WhiteboardGroupId;
  readonly locked: boolean;
  readonly hidden: boolean;
  readonly style: WhiteboardStyle;
  readonly createdAt: string;
  readonly updatedAt: string;
  readonly metadata?: WhiteboardMetadata;
};

export function createRectangleElement(
  options: CreateBoxElementOptions,
): Extract<WhiteboardElement, { type: "rectangle" }> {
  return {
    ...createBaseElement("rectangle", options),
    x: options.x,
    y: options.y,
    width: options.width,
    height: options.height,
    rotation: options.rotation ?? 0,
  };
}

export function createEllipseElement(
  options: CreateBoxElementOptions,
): Extract<WhiteboardElement, { type: "ellipse" }> {
  return {
    ...createBaseElement("ellipse", options),
    x: options.x,
    y: options.y,
    width: options.width,
    height: options.height,
    rotation: options.rotation ?? 0,
  };
}

export function createDiamondElement(
  options: CreateBoxElementOptions,
): Extract<WhiteboardElement, { type: "diamond" }> {
  return {
    ...createBaseElement("diamond", options),
    x: options.x,
    y: options.y,
    width: options.width,
    height: options.height,
    rotation: options.rotation ?? 0,
  };
}

export function createTextElement(
  options: CreateTextElementOptions,
): Extract<WhiteboardElement, { type: "text" }> {
  return {
    ...createBaseElement("text", options),
    x: options.x,
    y: options.y,
    width: options.width,
    height: options.height,
    rotation: options.rotation ?? 0,
    text: options.text,
    textAlign: "left",
    verticalAlign: "top",
  };
}

export function createImageElement(
  options: CreateImageElementOptions,
): Extract<WhiteboardElement, { type: "image" }> {
  assertEmbeddedImageSource(options.src);
  assertPositiveOptionalNumber(options.naturalWidth, "image.naturalWidth");
  assertPositiveOptionalNumber(options.naturalHeight, "image.naturalHeight");

  return {
    ...createBaseElement("image", options),
    role: options.role ?? "image",
    x: options.x,
    y: options.y,
    width: options.width,
    height: options.height,
    rotation: options.rotation ?? 0,
    src: options.src,
    alt: options.alt,
    naturalWidth: options.naturalWidth,
    naturalHeight: options.naturalHeight,
  };
}

function assertPositiveOptionalNumber(value: number | undefined, path: string): void {
  if (value !== undefined && (!Number.isFinite(value) || value <= 0)) {
    throw new Error(`${path} must be greater than 0`);
  }
}

function createBaseElement<TType extends WhiteboardElement["type"]>(
  type: TType,
  options: CreateElementBaseOptions,
): CreatedBaseElement<TType> {
  const createdAt = options.createdAt ?? new Date().toISOString();

  return {
    id: makeElementId(options.id),
    type,
    role: options.role,
    name: options.name,
    groupId: options.groupId,
    locked: options.locked ?? false,
    hidden: options.hidden ?? false,
    style: { ...DEFAULT_WHITEBOARD_STYLE, ...options.style },
    createdAt,
    updatedAt: options.updatedAt ?? createdAt,
    metadata: options.metadata,
  };
}
