import {
  migrateWhiteboardDocument,
  validateWhiteboardDocument,
  type WhiteboardDocument,
} from "./schema";

export type WhiteboardStorageSummary = {
  id: string;
  name: string;
  updatedAt: string;
  revision: number;
};

export type WhiteboardKeyValueStore = {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
  removeItem(key: string): void;
  keys?: () => string[];
};

export type WhiteboardStorageDriver = {
  save(document: WhiteboardDocument): Promise<WhiteboardDocument>;
  load(id: string): Promise<WhiteboardDocument | null>;
  list(): Promise<WhiteboardStorageSummary[]>;
  remove(id: string): Promise<void>;
  clear(): Promise<void>;
};

export type LocalFirstWhiteboardStorageOptions = {
  storage: WhiteboardKeyValueStore;
  namespace?: string;
  now?: () => string;
};

const DEFAULT_NAMESPACE = "dx.whiteboard";

function assertLocalId(id: string): void {
  if (!/^[A-Za-z0-9._:-]+$/.test(id)) {
    throw new Error("whiteboard id must be local-storage safe");
  }
}

function indexKey(namespace: string): string {
  return `${namespace}:index`;
}

function boardKey(namespace: string, id: string): string {
  assertLocalId(id);
  return `${namespace}:boards:${id}`;
}

function summaryFromDocument(document: WhiteboardDocument): WhiteboardStorageSummary {
  return {
    id: document.id,
    name: document.name,
    updatedAt: document.updatedAt,
    revision: revisionFromDocument(document),
  };
}

function parseIndex(value: string | null): WhiteboardStorageSummary[] {
  if (!value) {
    return [];
  }

  const parsed = JSON.parse(value) as unknown;

  if (!Array.isArray(parsed)) {
    return [];
  }

  return parsed
    .filter((entry): entry is Record<string, unknown> => typeof entry === "object" && entry !== null)
    .filter((entry) => typeof entry.id === "string" && (typeof entry.name === "string" || typeof entry.title === "string") && typeof entry.updatedAt === "string")
    .map((entry) => ({
      id: entry.id as string,
      name: (entry.name ?? entry.title) as string,
      updatedAt: entry.updatedAt as string,
      revision: typeof entry.revision === "number" && Number.isFinite(entry.revision) ? entry.revision : 0,
    }));
}

function sortIndex(index: readonly WhiteboardStorageSummary[]): WhiteboardStorageSummary[] {
  return [...index].sort((a, b) => b.updatedAt.localeCompare(a.updatedAt) || a.name.localeCompare(b.name));
}

function upsertIndex(
  index: readonly WhiteboardStorageSummary[],
  summary: WhiteboardStorageSummary,
): WhiteboardStorageSummary[] {
  const entries = index.filter((entry) => entry.id !== summary.id);
  entries.push(summary);

  return sortIndex(entries);
}

export function createMemoryWhiteboardKeyValueStore(initial?: Record<string, string>): WhiteboardKeyValueStore {
  const values = new Map(Object.entries(initial ?? {}));

  return {
    getItem(key) {
      return values.get(key) ?? null;
    },
    setItem(key, value) {
      values.set(key, value);
    },
    removeItem(key) {
      values.delete(key);
    },
    keys() {
      return [...values.keys()];
    },
  };
}

export function createLocalFirstWhiteboardStorageDriver(
  options: LocalFirstWhiteboardStorageOptions,
): WhiteboardStorageDriver {
  const namespace = options.namespace ?? DEFAULT_NAMESPACE;
  const now = options.now ?? (() => new Date().toISOString());
  const storage = options.storage;

  function readIndex(): WhiteboardStorageSummary[] {
    return parseIndex(storage.getItem(indexKey(namespace)));
  }

  function writeIndex(index: readonly WhiteboardStorageSummary[]): void {
    storage.setItem(indexKey(namespace), JSON.stringify(sortIndex(index)));
  }

  return {
    async save(document) {
      const migrated = validateWhiteboardDocument({
        ...migrateWhiteboardDocument(document),
        updatedAt: now(),
      });

      storage.setItem(boardKey(namespace, migrated.id), JSON.stringify(migrated));
      writeIndex(upsertIndex(readIndex(), summaryFromDocument(migrated)));

      return migrated;
    },

    async load(id) {
      const raw = storage.getItem(boardKey(namespace, id));

      if (!raw) {
        return null;
      }

      const migrated = migrateWhiteboardDocument(JSON.parse(raw));

      if (raw !== JSON.stringify(migrated)) {
        storage.setItem(boardKey(namespace, migrated.id), JSON.stringify(migrated));
        writeIndex(upsertIndex(readIndex(), summaryFromDocument(migrated)));
      }

      return migrated;
    },

    async list() {
      return readIndex();
    },

    async remove(id) {
      storage.removeItem(boardKey(namespace, id));
      writeIndex(readIndex().filter((entry) => entry.id !== id));
    },

    async clear() {
      const prefix = `${namespace}:boards:`;

      for (const key of storage.keys?.() ?? []) {
        if (key.startsWith(prefix)) {
          storage.removeItem(key);
        }
      }

      storage.removeItem(indexKey(namespace));
    },
  };
}

function revisionFromDocument(document: WhiteboardDocument): number {
  const revision = document.metadata?.revision;
  return typeof revision === "number" && Number.isFinite(revision) ? revision : document.elements.length;
}
