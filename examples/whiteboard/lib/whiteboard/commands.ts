import {
  alignElements,
  createElementGroup,
  distributeElements,
  duplicateElements,
  removeElementGroups,
  setElementGroupMetadata,
  setDocumentMetadata,
  setElementsHidden,
  setElementsLocked,
  type WhiteboardAlignment,
  type WhiteboardDistribution,
} from "./arrange";
import {
  createLibraryPresetElements,
  type WhiteboardLibraryPresetId,
} from "./library";
import { gridMetadataFromSettings, type WhiteboardGridSettingsPatch } from "./grid";
import {
  clearSelection,
  insertElements,
  removeElements,
  renameDocument,
  reorderElements,
  selectElements,
  setActiveTool,
  setCurrentStyle,
  setViewport,
  translateElements,
  updateElement,
  type WhiteboardReorderIntent,
} from "./scene";
import { assignFrameToElements, clearElementFrames } from "./frames";
import type {
  WhiteboardDocument,
  WhiteboardElement,
  WhiteboardElementId,
  WhiteboardElementPatch,
  WhiteboardGroupId,
  WhiteboardMetadata,
  WhiteboardPoint,
  WhiteboardSelectionMode,
  WhiteboardStyle,
  WhiteboardTool,
  WhiteboardViewport,
} from "./model";

export type WhiteboardCommand =
  | { readonly type: "document.rename"; readonly name: string; readonly now?: string }
  | { readonly type: "document.metadata.set"; readonly metadata: WhiteboardMetadata; readonly now?: string }
  | { readonly type: "grid.set"; readonly settings: WhiteboardGridSettingsPatch; readonly now?: string }
  | { readonly type: "tool.set"; readonly tool: WhiteboardTool; readonly now?: string }
  | { readonly type: "viewport.set"; readonly viewport: Partial<WhiteboardViewport>; readonly now?: string }
  | { readonly type: "viewport.pan"; readonly delta: WhiteboardPoint; readonly now?: string }
  | {
      readonly type: "viewport.zoom";
      readonly zoom: number;
      readonly anchor?: WhiteboardPoint;
      readonly now?: string;
    }
  | { readonly type: "style.set"; readonly style: Partial<WhiteboardStyle>; readonly now?: string }
  | {
      readonly type: "selection.set";
      readonly ids: readonly WhiteboardElementId[];
      readonly mode?: WhiteboardSelectionMode;
      readonly now?: string;
    }
  | { readonly type: "selection.clear"; readonly now?: string }
  | {
      readonly type: "group.create";
      readonly ids: readonly WhiteboardElementId[];
      readonly id?: string | WhiteboardGroupId;
      readonly name?: string;
      readonly metadata?: WhiteboardMetadata;
      readonly now?: string;
    }
  | {
      readonly type: "group.metadata.set";
      readonly id: string | WhiteboardGroupId;
      readonly metadata: WhiteboardMetadata;
      readonly now?: string;
    }
  | {
      readonly type: "group.remove";
      readonly groupIds?: readonly (string | WhiteboardGroupId)[];
      readonly elementIds?: readonly (string | WhiteboardElementId)[];
      readonly now?: string;
    }
  | {
      readonly type: "frame.assign";
      readonly frameId: string | WhiteboardElementId;
      readonly ids: readonly (string | WhiteboardElementId)[];
      readonly now?: string;
    }
  | {
      readonly type: "frame.clear";
      readonly ids: readonly (string | WhiteboardElementId)[];
      readonly now?: string;
    }
  | {
      readonly type: "element.add";
      readonly element: WhiteboardElement;
      readonly index?: number;
      readonly select?: boolean;
      readonly now?: string;
    }
  | {
      readonly type: "element.addMany";
      readonly elements: readonly WhiteboardElement[];
      readonly index?: number;
      readonly select?: boolean;
      readonly now?: string;
    }
  | {
      readonly type: "element.update";
      readonly id: WhiteboardElementId;
      readonly patch: WhiteboardElementPatch;
      readonly now?: string;
    }
  | {
      readonly type: "text.commit";
      readonly id: WhiteboardElementId;
      readonly text: string;
      readonly now?: string;
    }
  | {
      readonly type: "connector.bind";
      readonly id: WhiteboardElementId;
      readonly startBinding?: WhiteboardElementPatch["startBinding"];
      readonly endBinding?: WhiteboardElementPatch["endBinding"];
      readonly now?: string;
    }
  | {
      readonly type: "element.remove";
      readonly ids: readonly WhiteboardElementId[];
      readonly now?: string;
    }
  | {
      readonly type: "element.duplicate";
      readonly ids: readonly WhiteboardElementId[];
      readonly offset?: WhiteboardPoint;
      readonly now?: string;
    }
  | {
      readonly type: "element.lock";
      readonly ids: readonly WhiteboardElementId[];
      readonly now?: string;
    }
  | {
      readonly type: "element.unlock";
      readonly ids: readonly WhiteboardElementId[];
      readonly now?: string;
    }
  | {
      readonly type: "element.hide";
      readonly ids: readonly WhiteboardElementId[];
      readonly now?: string;
    }
  | {
      readonly type: "element.show";
      readonly ids: readonly WhiteboardElementId[];
      readonly now?: string;
    }
  | {
      readonly type: "element.translate";
      readonly ids: readonly WhiteboardElementId[];
      readonly delta: WhiteboardPoint;
      readonly now?: string;
    }
  | {
      readonly type: "element.reorder";
      readonly ids: readonly WhiteboardElementId[];
      readonly intent: WhiteboardReorderIntent;
      readonly now?: string;
    }
  | {
      readonly type: "element.align";
      readonly ids: readonly WhiteboardElementId[];
      readonly alignment: WhiteboardAlignment;
      readonly now?: string;
    }
  | {
      readonly type: "element.distribute";
      readonly ids: readonly WhiteboardElementId[];
      readonly distribution: WhiteboardDistribution;
      readonly now?: string;
    }
  | {
      readonly type: "library.insert";
      readonly preset: WhiteboardLibraryPresetId;
      readonly origin?: WhiteboardPoint;
      readonly idPrefix?: string;
      readonly now?: string;
    };

export function whiteboardCommandReducer(
  document: WhiteboardDocument,
  command: WhiteboardCommand,
): WhiteboardDocument {
  switch (command.type) {
    case "document.rename":
      return renameDocument(document, command.name, { now: command.now });
    case "document.metadata.set":
      return setDocumentMetadata(document, command.metadata, command.now);
    case "grid.set":
      return setDocumentMetadata(document, gridMetadataFromSettings(command.settings), command.now);
    case "tool.set":
      return setActiveTool(document, command.tool, { now: command.now });
    case "viewport.set":
      return setViewport(document, command.viewport, { now: command.now });
    case "viewport.pan":
      return setViewport(
        document,
        {
          x: document.viewport.x + command.delta.x,
          y: document.viewport.y + command.delta.y,
        },
        { now: command.now },
      );
    case "viewport.zoom":
      return zoomViewport(document, command.zoom, command.anchor, command.now);
    case "style.set":
      return setCurrentStyle(document, command.style, { now: command.now });
    case "selection.set":
      return selectElements(document, command.ids, {
        mode: command.mode,
        now: command.now,
      });
    case "selection.clear":
      return clearSelection(document, { now: command.now });
    case "group.create":
      return createElementGroup(document, command.ids, {
        id: command.id,
        name: command.name,
        metadata: command.metadata,
        now: command.now,
      });
    case "group.metadata.set":
      return setElementGroupMetadata(document, {
        id: command.id,
        metadata: command.metadata,
        now: command.now,
      });
    case "group.remove":
      return removeElementGroups(document, {
        groupIds: command.groupIds,
        elementIds: command.elementIds,
        now: command.now,
      });
    case "frame.assign":
      return assignFrameToElements(document, command.frameId, command.ids, command.now);
    case "frame.clear":
      return clearElementFrames(document, command.ids, command.now);
    case "element.add":
      return insertElements(document, [command.element], {
        index: command.index,
        select: command.select,
        now: command.now,
      });
    case "element.addMany":
      return insertElements(document, command.elements, {
        index: command.index,
        select: command.select,
        now: command.now,
      });
    case "element.update":
      return updateElement(document, command.id, command.patch, { now: command.now });
    case "text.commit":
      return commitText(document, command.id, command.text, command.now);
    case "connector.bind":
      return updateElement(
        document,
        command.id,
        {
          startBinding: command.startBinding,
          endBinding: command.endBinding,
        },
        { now: command.now },
      );
    case "element.remove":
      return removeElements(document, command.ids, { now: command.now });
    case "element.duplicate":
      return duplicateElements(document, command.ids, {
        offset: command.offset,
        now: command.now,
      });
    case "element.lock":
      return setElementsLocked(document, command.ids, true, command.now);
    case "element.unlock":
      return setElementsLocked(document, command.ids, false, command.now);
    case "element.hide":
      return setElementsHidden(document, command.ids, true, command.now);
    case "element.show":
      return setElementsHidden(document, command.ids, false, command.now);
    case "element.translate":
      return translateElements(document, command.ids, command.delta, { now: command.now });
    case "element.reorder":
      return reorderElements(document, command.ids, command.intent, { now: command.now });
    case "element.align":
      return alignElements(document, command.ids, command.alignment, command.now);
    case "element.distribute":
      return distributeElements(document, command.ids, command.distribution, command.now);
    case "library.insert":
      return insertElements(
        document,
        createLibraryPresetElements({
          preset: command.preset,
          origin: command.origin,
          idPrefix: command.idPrefix,
          now: command.now,
          style: document.currentStyle,
        }),
        { select: true, now: command.now },
      );
  }
}

export function applyWhiteboardCommands(
  document: WhiteboardDocument,
  commands: readonly WhiteboardCommand[],
): WhiteboardDocument {
  return commands.reduce(whiteboardCommandReducer, document);
}

export function isUndoableWhiteboardCommand(command: WhiteboardCommand): boolean {
  return (
    command.type === "document.rename" ||
    command.type === "document.metadata.set" ||
    command.type === "grid.set" ||
    command.type === "group.create" ||
    command.type === "group.metadata.set" ||
    command.type === "group.remove" ||
    command.type === "frame.assign" ||
    command.type === "frame.clear" ||
    command.type === "element.add" ||
    command.type === "element.addMany" ||
    command.type === "element.duplicate" ||
    command.type === "element.update" ||
    command.type === "text.commit" ||
    command.type === "connector.bind" ||
    command.type === "element.remove" ||
    command.type === "element.lock" ||
    command.type === "element.unlock" ||
    command.type === "element.hide" ||
    command.type === "element.show" ||
    command.type === "element.translate" ||
    command.type === "element.reorder" ||
    command.type === "element.align" ||
    command.type === "element.distribute" ||
    command.type === "library.insert"
  );
}

function commitText(
  document: WhiteboardDocument,
  id: WhiteboardElementId,
  text: string,
  now: string | undefined,
): WhiteboardDocument {
  const element = document.elements.find((item) => item.id === id);
  if (!element || element.type !== "text" || element.text === text) return document;
  return updateElement(document, id, { text }, { now });
}

function zoomViewport(
  document: WhiteboardDocument,
  requestedZoom: number,
  anchor: WhiteboardPoint | undefined,
  now: string | undefined,
): WhiteboardDocument {
  const zoom = Math.max(0.05, Math.min(8, requestedZoom));
  if (!anchor) return setViewport(document, { zoom }, { now });

  const scale = document.viewport.zoom / zoom;
  return setViewport(
    document,
    {
      x: anchor.x - (anchor.x - document.viewport.x) * scale,
      y: anchor.y - (anchor.y - document.viewport.y) * scale,
      zoom,
    },
    { now },
  );
}
