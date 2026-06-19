import {
  imageMimeTypeFromSource,
  WHITEBOARD_IMAGE_SOURCE_POLICY,
  type WhiteboardImageMimeType,
} from "./image-source";
import {
  createImageElement,
  type CreateImageElementOptions,
} from "./element-factory";
import type {
  WhiteboardElement,
  WhiteboardMetadata,
  WhiteboardPoint,
  WhiteboardSize,
} from "./model";

export const WHITEBOARD_IMAGE_IMPORT_MAX_BYTES = 2 * 1024 * 1024;
export const WHITEBOARD_IMAGE_IMPORT_DEFAULT_SIZE = { width: 320, height: 200 } as const;
export const WHITEBOARD_IMAGE_IMPORT_MAX_SIZE = { width: 640, height: 420 } as const;

export type WhiteboardImportedImageElement = Extract<WhiteboardElement, { type: "image" }>;

export type CreateImportedImageElementOptions = {
  readonly id: string;
  readonly fileName: string;
  readonly fileType?: string;
  readonly source: string;
  readonly fileSizeBytes?: number;
  readonly naturalWidth?: number;
  readonly naturalHeight?: number;
  readonly origin?: WhiteboardPoint;
  readonly createdAt?: string;
  readonly updatedAt?: string;
};

export type WhiteboardImageImportDiagnostic = {
  readonly code:
    | "image.empty_source"
    | "image.unsupported_source"
    | "image.mime_mismatch"
    | "image.empty_payload"
    | "image.empty_file"
    | "image.too_large"
    | "image.invalid_dimensions";
  readonly message: string;
};

export type WhiteboardImageImportResult =
  | {
      readonly status: "accepted";
      readonly element: WhiteboardImportedImageElement;
      readonly mimeType: WhiteboardImageMimeType;
    }
  | {
      readonly status: "rejected";
      readonly diagnostics: readonly WhiteboardImageImportDiagnostic[];
    };

export function createImportedImageElement(
  options: CreateImportedImageElementOptions,
): WhiteboardImageImportResult {
  const diagnostics = validateImportedImage(options);
  if (diagnostics.length > 0) {
    return { status: "rejected", diagnostics };
  }

  const source = options.source.trim();
  const mimeType = imageMimeTypeFromSource(source);
  if (!mimeType) {
    return {
      status: "rejected",
      diagnostics: [unsupportedSourceDiagnostic()],
    };
  }

  const size = fitImportedImageSize({
    width: options.naturalWidth,
    height: options.naturalHeight,
  });
  const createdAt = options.createdAt ?? new Date().toISOString();
  const metadata = importedImageMetadata({
    fileName: options.fileName,
    fileSizeBytes: options.fileSizeBytes,
    mimeType,
  });
  const elementOptions: CreateImageElementOptions = {
    id: options.id,
    role: "image",
    name: importedImageName(options.fileName),
    x: options.origin?.x ?? 96,
    y: options.origin?.y ?? 96,
    width: size.width,
    height: size.height,
    src: source,
    alt: importedImageAlt(options.fileName),
    naturalWidth: options.naturalWidth,
    naturalHeight: options.naturalHeight,
    createdAt,
    updatedAt: options.updatedAt ?? createdAt,
    metadata,
  };

  return {
    status: "accepted",
    element: createImageElement(elementOptions),
    mimeType,
  };
}

export function fitImportedImageSize(size: {
  readonly width?: number;
  readonly height?: number;
}): WhiteboardSize {
  if (!hasPositiveDimensions(size)) {
    return WHITEBOARD_IMAGE_IMPORT_DEFAULT_SIZE;
  }

  const scale = Math.min(
    1,
    WHITEBOARD_IMAGE_IMPORT_MAX_SIZE.width / size.width,
    WHITEBOARD_IMAGE_IMPORT_MAX_SIZE.height / size.height,
  );

  return {
    width: roundDimension(size.width * scale),
    height: roundDimension(size.height * scale),
  };
}

export function importedImageAlt(fileName: string): string {
  const cleaned = fileBaseName(fileName)
    .replace(/[-_]+/g, " ")
    .replace(/\s+/g, " ")
    .trim();

  return cleaned || "Imported image";
}

export function importedImageName(fileName: string): string {
  const alt = importedImageAlt(fileName);
  return alt === "Imported image" ? alt : `Image: ${alt}`;
}

function validateImportedImage(
  options: CreateImportedImageElementOptions,
): readonly WhiteboardImageImportDiagnostic[] {
  const diagnostics: WhiteboardImageImportDiagnostic[] = [];
  const source = options.source.trim();

  if (!source) {
    diagnostics.push({
      code: "image.empty_source",
      message: "Image import needs a non-empty embedded image source.",
    });
  } else if (!imageMimeTypeFromSource(source)) {
    diagnostics.push(unsupportedSourceDiagnostic());
  } else if (!hasImagePayload(source)) {
    diagnostics.push({
      code: "image.empty_payload",
      message: "Image import needs a non-empty embedded image payload.",
    });
  } else if (options.fileType && options.fileType !== imageMimeTypeFromSource(source)) {
    diagnostics.push({
      code: "image.mime_mismatch",
      message: "Image import file type must match the embedded data URL MIME type.",
    });
  }

  if (options.fileSizeBytes !== undefined) {
    if (!Number.isFinite(options.fileSizeBytes) || options.fileSizeBytes <= 0) {
      diagnostics.push({
        code: "image.empty_file",
        message: "Image import needs a non-empty local file.",
      });
    } else if (options.fileSizeBytes > WHITEBOARD_IMAGE_IMPORT_MAX_BYTES) {
      diagnostics.push({
        code: "image.too_large",
        message: "Image import is limited to 2 MB embedded files.",
      });
    }
  }

  if (
    (options.naturalWidth !== undefined || options.naturalHeight !== undefined) &&
    !hasPositiveDimensions({ width: options.naturalWidth, height: options.naturalHeight })
  ) {
    diagnostics.push({
      code: "image.invalid_dimensions",
      message: "Image import dimensions must be positive numbers when provided.",
    });
  }

  return diagnostics;
}

function unsupportedSourceDiagnostic(): WhiteboardImageImportDiagnostic {
  return {
    code: "image.unsupported_source",
    message: "Image import accepts embedded PNG, JPEG, WebP, or SVG data URLs.",
  };
}

function importedImageMetadata(options: {
  readonly fileName: string;
  readonly fileSizeBytes?: number;
  readonly mimeType: WhiteboardImageMimeType;
}): WhiteboardMetadata {
  return {
    importSource: "local-file",
    imagePolicy: WHITEBOARD_IMAGE_SOURCE_POLICY,
    mimeType: options.mimeType,
    originalName: options.fileName || "image",
    sourceOwned: true,
    ...(options.fileSizeBytes === undefined ? {} : { fileSizeBytes: options.fileSizeBytes }),
  };
}

function fileBaseName(fileName: string): string {
  return fileName.replace(/\.[^.]+$/, "");
}

function hasImagePayload(source: string): boolean {
  const comma = source.indexOf(",");
  return comma >= 0 && source.slice(comma + 1).trim().length > 0;
}

function hasPositiveDimensions(size: {
  readonly width?: number;
  readonly height?: number;
}): size is { readonly width: number; readonly height: number } {
  return (
    size.width !== undefined &&
    size.height !== undefined &&
    Number.isFinite(size.width) &&
    Number.isFinite(size.height) &&
    size.width > 0 &&
    size.height > 0
  );
}

function roundDimension(value: number): number {
  return Math.max(1, Number(value.toFixed(2)));
}
