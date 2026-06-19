export type WhiteboardImageSourcePolicy = "embedded-data-url-only";
export type WhiteboardImageMimeType = "image/png" | "image/jpeg" | "image/webp" | "image/svg+xml";

export const WHITEBOARD_IMAGE_SOURCE_POLICY: WhiteboardImageSourcePolicy = "embedded-data-url-only";

const SUPPORTED_IMAGE_MIME_TYPES = new Set<WhiteboardImageMimeType>([
  "image/png",
  "image/jpeg",
  "image/webp",
  "image/svg+xml",
]);

const DATA_IMAGE_PREFIX = /^data:([^,;]+)(?:;[^,]*)?,/i;

export function imageMimeTypeFromSource(source: string): WhiteboardImageMimeType | null {
  const match = DATA_IMAGE_PREFIX.exec(source.trim());
  if (!match) {
    return null;
  }

  const mimeType = match[1].toLowerCase() as WhiteboardImageMimeType;
  return SUPPORTED_IMAGE_MIME_TYPES.has(mimeType) ? mimeType : null;
}

export function assertEmbeddedImageSource(source: string, path = "image.src"): void {
  if (!imageMimeTypeFromSource(source)) {
    throw new Error(`${path} must be an embedded data:image/png, image/jpeg, image/webp, or image/svg+xml URL`);
  }
}

export function isEmbeddedImageSource(source: string): boolean {
  return imageMimeTypeFromSource(source) !== null;
}
