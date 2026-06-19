import { assertEmbeddedImageSource } from "./image-source";
import type { WhiteboardElementPatch } from "./model";

type ImagePatchInput = Pick<WhiteboardElementPatch, "src" | "alt" | "naturalWidth" | "naturalHeight">;

export function normalizedImagePatch(patch: ImagePatchInput): WhiteboardElementPatch {
  const next: WhiteboardElementPatch = {};

  if (patch.src !== undefined) {
    assertEmbeddedImageSource(patch.src);
    next.src = patch.src;
  }

  if (patch.alt !== undefined) {
    if (patch.alt.trim().length === 0) {
      throw new Error("image.alt must be a non-empty string");
    }
    next.alt = patch.alt;
  }

  if (hasPatchKey(patch, "naturalWidth")) {
    assertPositiveOptionalNumber(patch.naturalWidth, "image.naturalWidth");
    next.naturalWidth = patch.naturalWidth;
  }

  if (hasPatchKey(patch, "naturalHeight")) {
    assertPositiveOptionalNumber(patch.naturalHeight, "image.naturalHeight");
    next.naturalHeight = patch.naturalHeight;
  }

  return next;
}

function assertPositiveOptionalNumber(value: number | undefined, path: string): void {
  if (value !== undefined && (!Number.isFinite(value) || value <= 0)) {
    throw new Error(`${path} must be greater than 0`);
  }
}

function hasPatchKey(patch: ImagePatchInput, key: keyof ImagePatchInput): boolean {
  return Object.prototype.hasOwnProperty.call(patch, key);
}
