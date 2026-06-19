import { mkdir, readdir, readFile, rm, writeFile } from "node:fs/promises";
import { basename, join, resolve } from "node:path";

import { exportDxDraw, importDxDraw } from "../../lib/whiteboard/export/dxdraw";
import type {
  WhiteboardStorageDriver,
  WhiteboardStorageSummary,
} from "../../lib/whiteboard/persistence/local-first";
import type { WhiteboardDocument } from "../../lib/whiteboard/persistence/schema";

export type WhiteboardFileStorageOptions = {
  rootDir: string;
  now?: () => string;
};

function assertSafeId(id: string): void {
  if (!/^[A-Za-z0-9._:-]+$/.test(id)) {
    throw new Error("whiteboard id must be safe for local file storage");
  }
}

function pathFor(rootDir: string, id: string): string {
  assertSafeId(id);

  const root = resolve(rootDir);
  const file = resolve(root, `${id}.dxdraw`);

  if (!file.startsWith(root)) {
    throw new Error("whiteboard file path escaped the storage root");
  }

  return file;
}

function summary(document: WhiteboardDocument): WhiteboardStorageSummary {
  return {
    id: document.id,
    name: document.name,
    updatedAt: document.updatedAt,
    revision: revisionFromDocument(document),
  };
}

export function createWhiteboardFileStorageDriver(options: WhiteboardFileStorageOptions): WhiteboardStorageDriver {
  const rootDir = options.rootDir;
  const now = options.now ?? (() => new Date().toISOString());

  return {
    async save(document) {
      await mkdir(rootDir, { recursive: true });

      const updated = {
        ...document,
        updatedAt: now(),
      };

      await writeFile(pathFor(rootDir, updated.id), exportDxDraw(updated), "utf8");

      return updated;
    },

    async load(id) {
      try {
        return importDxDraw(await readFile(pathFor(rootDir, id), "utf8"));
      } catch (error) {
        if ((error as { code?: string }).code === "ENOENT") {
          return null;
        }

        throw error;
      }
    },

    async list() {
      await mkdir(rootDir, { recursive: true });

      const files = (await readdir(rootDir)).filter((file) => file.endsWith(".dxdraw"));
      const entries = await Promise.all(
        files.map(async (file) => {
          const id = basename(file, ".dxdraw");
          const document = await this.load(id);

          return document ? summary(document) : null;
        }),
      );

      return entries
        .filter((entry): entry is WhiteboardStorageSummary => entry !== null)
        .sort((a, b) => b.updatedAt.localeCompare(a.updatedAt) || a.name.localeCompare(b.name));
    },

    async remove(id) {
      await rm(pathFor(rootDir, id), { force: true });
    },

    async clear() {
      await rm(rootDir, { recursive: true, force: true });
      await mkdir(rootDir, { recursive: true });
    },
  };
}

function revisionFromDocument(document: WhiteboardDocument): number {
  const revision = document.metadata?.revision;
  return typeof revision === "number" && Number.isFinite(revision) ? revision : document.elements.length;
}
