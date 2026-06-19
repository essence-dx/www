import {
  isUndoableWhiteboardCommand,
  whiteboardCommandReducer,
  type WhiteboardCommand,
} from "./commands";
import { createWhiteboardDocument, type WhiteboardDocument } from "./model";

export interface WhiteboardHistory {
  readonly past: readonly WhiteboardDocument[];
  readonly present: WhiteboardDocument;
  readonly future: readonly WhiteboardDocument[];
  readonly limit: number;
}

export interface CreateWhiteboardHistoryOptions {
  readonly limit?: number;
}

export function createWhiteboardHistory(
  document: WhiteboardDocument = createWhiteboardDocument(),
  options: CreateWhiteboardHistoryOptions = {},
): WhiteboardHistory {
  return {
    past: [],
    present: document,
    future: [],
    limit: normalizeLimit(options.limit),
  };
}

export function pushWhiteboardCommand(
  history: WhiteboardHistory,
  command: WhiteboardCommand,
): WhiteboardHistory {
  const nextPresent = whiteboardCommandReducer(history.present, command);
  if (nextPresent === history.present) return history;

  if (!isUndoableWhiteboardCommand(command)) {
    return {
      ...history,
      present: nextPresent,
    };
  }

  return {
    past: [...history.past, history.present].slice(-history.limit),
    present: nextPresent,
    future: [],
    limit: history.limit,
  };
}

export function pushWhiteboardCommandBatch(
  history: WhiteboardHistory,
  commands: readonly WhiteboardCommand[],
): WhiteboardHistory {
  if (commands.length === 0) return history;

  let nextPresent = history.present;
  let undoable = false;

  for (const command of commands) {
    const changed = whiteboardCommandReducer(nextPresent, command);
    if (changed !== nextPresent && isUndoableWhiteboardCommand(command)) {
      undoable = true;
    }
    nextPresent = changed;
  }

  if (nextPresent === history.present) return history;

  if (!undoable) {
    return {
      ...history,
      present: nextPresent,
    };
  }

  return {
    past: [...history.past, history.present].slice(-history.limit),
    present: nextPresent,
    future: [],
    limit: history.limit,
  };
}

export function undoWhiteboard(history: WhiteboardHistory): WhiteboardHistory {
  const previous = history.past[history.past.length - 1];
  if (!previous) return history;

  return {
    past: history.past.slice(0, -1),
    present: previous,
    future: [history.present, ...history.future],
    limit: history.limit,
  };
}

export function redoWhiteboard(history: WhiteboardHistory): WhiteboardHistory {
  const next = history.future[0];
  if (!next) return history;

  return {
    past: [...history.past, history.present].slice(-history.limit),
    present: next,
    future: history.future.slice(1),
    limit: history.limit,
  };
}

export function replaceWhiteboardHistoryPresent(
  history: WhiteboardHistory,
  present: WhiteboardDocument,
): WhiteboardHistory {
  return {
    ...history,
    present,
  };
}

export function canUndoWhiteboard(history: WhiteboardHistory): boolean {
  return history.past.length > 0;
}

export function canRedoWhiteboard(history: WhiteboardHistory): boolean {
  return history.future.length > 0;
}

function normalizeLimit(limit: number | undefined): number {
  return Math.max(1, Math.trunc(limit ?? 100));
}
