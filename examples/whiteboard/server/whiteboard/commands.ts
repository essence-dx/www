import {
  whiteboardCommandReducer,
  type WhiteboardCommand as WhiteboardModelCommand,
} from "../../lib/whiteboard/commands";
import {
  createWhiteboardDocument as createCanonicalWhiteboardDocument,
  makeElementId,
  type WhiteboardDocument,
  type WhiteboardElement,
  type WhiteboardElementPatch,
} from "../../lib/whiteboard/model";
import {
  normalizeWhiteboardElement,
  validateWhiteboardDocument,
} from "../../lib/whiteboard/persistence/schema";

export type CreateWhiteboardDocumentOptions = {
  id: string;
  name?: string;
  title?: string;
  now?: () => string;
};

export type WhiteboardServerCommand =
  | {
      readonly type: "element.add";
      readonly element: WhiteboardElement;
      readonly select?: boolean;
      readonly now?: () => string;
    }
  | {
      readonly type: "element.update";
      readonly id: string;
      readonly patch: WhiteboardElementPatch;
      readonly now?: () => string;
    }
  | {
      readonly type: "element.remove";
      readonly ids: readonly string[];
      readonly now?: () => string;
    }
  | {
      readonly type: "element.reorder";
      readonly ids: readonly string[];
      readonly intent: "front" | "back" | "forward" | "backward";
      readonly now?: () => string;
    }
  | {
      readonly type: "document.rename";
      readonly name: string;
      readonly now?: () => string;
    };

export function createWhiteboardDocument(
  options: CreateWhiteboardDocumentOptions,
): WhiteboardDocument {
  const createdAt = timestamp(options.now);

  return createCanonicalWhiteboardDocument({
    id: options.id,
    name: options.name ?? options.title ?? "Untitled whiteboard",
    createdAt,
    updatedAt: createdAt,
    metadata: {
      revision: 0,
      storage: "server",
    },
  });
}

export function applyWhiteboardCommand(
  document: WhiteboardDocument,
  command: WhiteboardServerCommand,
): WhiteboardDocument {
  const current = validateWhiteboardDocument(document);
  const now = timestamp(command.now);
  const nextCommand = toModelCommand(command, now);
  const next = whiteboardCommandReducer(current, nextCommand);

  return validateWhiteboardDocument(withRevision(next, now));
}

function toModelCommand(
  command: WhiteboardServerCommand,
  now: string,
): WhiteboardModelCommand {
  switch (command.type) {
    case "element.add":
      return {
        type: "element.add",
        element: normalizeWhiteboardElement(command.element),
        select: command.select,
        now,
      };
    case "element.update":
      return {
        type: "element.update",
        id: makeElementId(command.id),
        patch: command.patch,
        now,
      };
    case "element.remove":
      return {
        type: "element.remove",
        ids: command.ids.map((id) => makeElementId(id)),
        now,
      };
    case "element.reorder":
      return {
        type: "element.reorder",
        ids: command.ids.map((id) => makeElementId(id)),
        intent: command.intent,
        now,
      };
    case "document.rename":
      return {
        type: "document.rename",
        name: command.name,
        now,
      };
  }
}

function withRevision(document: WhiteboardDocument, updatedAt: string): WhiteboardDocument {
  const currentRevision = document.metadata?.revision;
  const revision =
    typeof currentRevision === "number" && Number.isFinite(currentRevision)
      ? currentRevision + 1
      : 1;

  return {
    ...document,
    updatedAt,
    metadata: {
      ...document.metadata,
      revision,
    },
  };
}

function timestamp(now?: () => string): string {
  return now?.() ?? new Date().toISOString();
}

