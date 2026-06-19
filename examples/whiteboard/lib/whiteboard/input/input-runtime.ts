import {
  elementsById,
  hitTestDocument,
  patchElementToBounds,
  resizeBoundsFromHandle,
  roundCoordinate,
  screenToWorld,
  translateElementPatch,
} from "../render/geometry";
import {
  DEFAULT_WHITEBOARD_VIEWPORT,
  makeElementId,
  type WhiteboardDocument,
  type WhiteboardElement,
  type WhiteboardElementId,
  type WhiteboardPoint,
  type WhiteboardSelection,
  type WhiteboardTool,
  type WhiteboardViewport,
} from "../render/model";
import type { WhiteboardCommand } from "../commands";
import { getGridSettings, snapElementPatch, snapPoint } from "../grid";
import { hitTestConnectorTarget } from "../hit-test";
import { draftFromDrag, elementFromDrawDrag, freehandElement, textElement } from "./elements";
import { clampZoom, createDefaultIdFactory } from "./helpers";
import { commandsForKeyboardInput } from "./keyboard";
import { applyInputLocalCommands } from "./local-state";
import { beginSelectionDrag, finishSelectionAreaDrag, selectionAreaBounds } from "./selection";
import type {
  ActiveDrag,
  WhiteboardInputResult,
  WhiteboardInputRuntimeOptions,
  WhiteboardInputState,
  WhiteboardKeyboardInput,
  WhiteboardPointerInput,
  WhiteboardWheelInput,
} from "./types";

export type {
  ActiveDrag,
  WhiteboardDrawDrag,
  WhiteboardInputResult,
  WhiteboardInputRuntimeOptions,
  WhiteboardInputState,
  WhiteboardKeyboardInput,
  WhiteboardPointerInput,
  WhiteboardWheelInput,
} from "./types";

export type WhiteboardInputRuntime = ReturnType<typeof createWhiteboardInputRuntime>;

export function createWhiteboardInputRuntime(options: WhiteboardInputRuntimeOptions) {
  let document = options.document;
  const idFactory = options.idFactory ?? createDefaultIdFactory();
  let tool = options.tool ?? document.activeTool;
  let viewport = options.viewport ?? document.viewport ?? DEFAULT_WHITEBOARD_VIEWPORT;
  let selection = options.selection ?? { ids: document.selection };
  let draft: WhiteboardElement | null = null;
  let activeTextId: WhiteboardElementId | null = null;
  let activeDrag: ActiveDrag | null = null;
  const erasedIds = new Set<WhiteboardElementId>();

  function snapshot(): WhiteboardInputState {
    return {
      tool,
      viewport,
      selection: { ids: [...selection.ids] },
      draft,
      selectionArea: activeDrag?.kind === "select-area" ? selectionAreaBounds(activeDrag) : null,
      activeTextId,
    };
  }

  function result(commands: WhiteboardCommand[]): WhiteboardInputResult {
    const nextLocalState = applyInputLocalCommands({ selection, viewport, tool }, commands);
    selection = nextLocalState.selection;
    viewport = nextLocalState.viewport;
    tool = nextLocalState.tool;

    return {
      commands,
      state: snapshot(),
    };
  }

  function pointerDown(input: WhiteboardPointerInput): WhiteboardInputResult {
    if (input.button !== undefined && input.button !== 0) {
      return result([]);
    }

    const screenPoint = { x: input.clientX, y: input.clientY };
    const rawWorldPoint = screenToWorld(screenPoint, viewport);
    draft = null;

    if (tool === "pan" || tool === "hand") {
      activeDrag = {
        kind: "pan",
        pointerId: input.pointerId,
        startClient: screenPoint,
        startViewport: viewport,
      };
      return result([]);
    }

    if (tool === "select") {
      const selectionDrag = beginSelectionDrag({
        document,
        selection,
        pointerId: input.pointerId,
        worldPoint: rawWorldPoint,
        extend: input.shiftKey,
      });
      activeDrag = selectionDrag.drag;
      return result(selectionDrag.commands);
    }

    if (tool === "eraser") {
      return result(eraseAt(rawWorldPoint));
    }

    if (tool === "text") {
      const id = makeElementId(idFactory());
      const element = textElement(id, snapWorldPoint(rawWorldPoint));
      activeTextId = id;
      return result([
        { type: "element.add", element, select: true },
        { type: "tool.set", tool: "select" },
      ]);
    }

    if (tool === "freehand" || tool === "pen") {
      activeDrag = {
        kind: "freehand",
        pointerId: input.pointerId,
        points: [rawWorldPoint],
      };
      draft = freehandElement(makeElementId("__draft_freehand__"), [rawWorldPoint]);
      return result([]);
    }

    activeDrag = {
      kind: "draw",
      pointerId: input.pointerId,
      tool,
      startWorld: snapWorldPoint(rawWorldPoint),
      startBinding: connectorBindingAt(tool, rawWorldPoint),
      currentWorld: snapWorldPoint(rawWorldPoint),
    };
    draft = draftFromDrag(activeDrag);
    return result([]);
  }

  function pointerMove(input: Omit<WhiteboardPointerInput, "button">): WhiteboardInputResult {
    if (!activeDrag || activeDrag.pointerId !== input.pointerId) {
      return result([]);
    }

    const screenPoint = { x: input.clientX, y: input.clientY };
    const rawWorldPoint = screenToWorld(screenPoint, viewport);
    const worldPoint = activeDragUsesSnap(activeDrag) ? snapWorldPoint(rawWorldPoint) : rawWorldPoint;

    if (activeDrag.kind === "pan") {
      return result([
        {
          type: "viewport.set",
          viewport: {
            zoom: activeDrag.startViewport.zoom,
            x: roundCoordinate(activeDrag.startViewport.x + screenPoint.x - activeDrag.startClient.x),
            y: roundCoordinate(activeDrag.startViewport.y + screenPoint.y - activeDrag.startClient.y),
          },
        },
      ]);
    }

    if (activeDrag.kind === "select-area") {
      activeDrag.currentWorld = rawWorldPoint;
      return result([]);
    }

    if (activeDrag.kind === "move") {
      const movableElements = activeDrag.elements.filter((element) => !element.locked && !element.hidden);
      if (movableElements.length === 0) {
        return result([]);
      }

      const delta = {
        x: roundCoordinate(worldPoint.x - activeDrag.startWorld.x),
        y: roundCoordinate(worldPoint.y - activeDrag.startWorld.y),
      };
      return result(
        movableElements.map((element) => ({
          type: "element.update",
          id: element.id,
          patch: snapElementPatch(translateElementPatch(element, delta), getGridSettings(document)),
        })),
      );
    }

    if (activeDrag.kind === "resize") {
      if (activeDrag.element.locked || activeDrag.element.hidden) {
        return result([]);
      }

      const bounds = resizeBoundsFromHandle(activeDrag.initialBounds, activeDrag.handle, worldPoint);
      return result([
        {
          type: "element.update",
          id: activeDrag.element.id,
          patch: snapElementPatch(patchElementToBounds(activeDrag.element, bounds), getGridSettings(document)),
        },
      ]);
    }

    if (activeDrag.kind === "draw") {
      activeDrag.currentWorld = worldPoint;
      draft = draftFromDrag(activeDrag);
      return result([]);
    }

    activeDrag.points.push(rawWorldPoint);
    draft = freehandElement(makeElementId("__draft_freehand__"), activeDrag.points);
    return result([]);
  }

  function pointerUp(input: Omit<WhiteboardPointerInput, "button">): WhiteboardInputResult {
    if (!activeDrag || activeDrag.pointerId !== input.pointerId) {
      return result([]);
    }

    const drag = activeDrag;
    activeDrag = null;

    if (drag.kind === "draw") {
      const rawWorldPoint = screenToWorld({ x: input.clientX, y: input.clientY }, viewport);
      drag.currentWorld = snapWorldPoint(rawWorldPoint);
      drag.endBinding = connectorBindingAt(drag.tool, rawWorldPoint);
      const element = elementFromDrawDrag(makeElementId(idFactory()), drag);
      draft = null;

      return element
        ? result([
            { type: "element.add", element, select: true },
            { type: "tool.set", tool: "select" },
          ])
        : result([]);
    }

    if (drag.kind === "freehand") {
      drag.points.push(screenToWorld({ x: input.clientX, y: input.clientY }, viewport));
      const element = freehandElement(makeElementId(idFactory()), drag.points);
      draft = null;

      return result([
        { type: "element.add", element, select: true },
        { type: "tool.set", tool: "select" },
      ]);
    }

    if (drag.kind === "select-area") {
      drag.currentWorld = screenToWorld({ x: input.clientX, y: input.clientY }, viewport);
      return result(
        finishSelectionAreaDrag({
          document,
          selection,
          drag,
        }),
      );
    }

    draft = null;
    return result([]);
  }

  function wheel(input: WhiteboardWheelInput): WhiteboardInputResult {
    if (input.ctrlKey || input.metaKey) {
      const factor = input.deltaY < 0 ? 1.1 : 1 / 1.1;
      const nextZoom = clampZoom(viewport.zoom * factor);
      const worldPoint = screenToWorld({ x: input.clientX, y: input.clientY }, viewport);

      return result([
        {
          type: "viewport.set",
          viewport: {
            zoom: roundCoordinate(nextZoom),
            x: roundCoordinate(input.clientX - worldPoint.x * nextZoom),
            y: roundCoordinate(input.clientY - worldPoint.y * nextZoom),
          },
        },
      ]);
    }

    return result([
      {
        type: "viewport.set",
        viewport: {
          zoom: viewport.zoom,
          x: roundCoordinate(viewport.x - input.deltaX),
          y: roundCoordinate(viewport.y - input.deltaY),
        },
      },
    ]);
  }

  function keyDown(input: WhiteboardKeyboardInput): WhiteboardInputResult {
    const keyboardResult = commandsForKeyboardInput(input, {
      elements: document.elements,
      selection,
      viewport,
    });

    if (!keyboardResult) {
      return result([]);
    }

    if (keyboardResult.clearInteraction) {
      activeDrag = null;
      draft = null;
      activeTextId = null;
    }

    return result(keyboardResult.commands);
  }

  function eraseAt(worldPoint: WhiteboardPoint): WhiteboardCommand[] {
    const hit = hitTestDocument(
      document.elements.filter((element) => !erasedIds.has(element.id) && !element.locked && !element.hidden),
      worldPoint,
      8 / viewport.zoom,
    );

    if (!hit) {
      return [];
    }

    erasedIds.add(hit.id);
    return [{ type: "element.remove", ids: [hit.id] }];
  }

  function setTool(nextTool: WhiteboardTool): void {
    tool = nextTool;
    draft = null;
    activeDrag = null;
  }

  function setDocumentState(next: {
    readonly document?: WhiteboardDocument;
    readonly viewport?: WhiteboardViewport;
    readonly selection?: WhiteboardSelection;
    readonly tool?: WhiteboardTool;
  }): void {
    if (next.document) {
      document = next.document;
    }
    if (next.viewport) {
      viewport = next.viewport;
    }
    if (next.selection) {
      selection = next.selection;
    }
    if (next.tool) {
      tool = next.tool;
    }
  }

  function snapWorldPoint(point: WhiteboardPoint): WhiteboardPoint {
    return snapPoint(point, getGridSettings(document));
  }

  function connectorBindingAt(tool: WhiteboardTool, point: WhiteboardPoint) {
    if (tool !== "line" && tool !== "arrow") return undefined;
    const hit = hitTestConnectorTarget(document, point, {
      tolerance: 8 / viewport.zoom,
      includeLocked: true,
    });

    return hit ? { elementId: hit.elementId, anchor: "auto" as const } : undefined;
  }

  return {
    pointerDown,
    pointerMove,
    pointerUp,
    wheel,
    keyDown,
    setTool,
    setDocumentState,
    state: snapshot,
  };
}

function activeDragUsesSnap(activeDrag: ActiveDrag): boolean {
  return activeDrag.kind === "draw" || activeDrag.kind === "move" || activeDrag.kind === "resize";
}
