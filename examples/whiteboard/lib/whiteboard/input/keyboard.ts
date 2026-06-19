import { roundCoordinate } from "../render/geometry";
import {
  DEFAULT_WHITEBOARD_VIEWPORT,
  type WhiteboardElement,
  type WhiteboardSelection,
  type WhiteboardViewport,
} from "../render/model";
import type { WhiteboardCommand } from "../commands";
import { clampZoom, toolForShortcut } from "./helpers";
import type { WhiteboardKeyboardInput } from "./types";

export type WhiteboardKeyboardCommandResult = {
  readonly commands: WhiteboardCommand[];
  readonly clearInteraction?: boolean;
};

export function commandsForKeyboardInput(
  input: WhiteboardKeyboardInput,
  context: {
    readonly elements: readonly WhiteboardElement[];
    readonly selection: WhiteboardSelection;
    readonly viewport: WhiteboardViewport;
  },
): WhiteboardKeyboardCommandResult | null {
  const key = input.key.toLowerCase();
  const commandModifier = input.ctrlKey || input.metaKey;

  if (commandModifier && key === "a") {
    return {
      commands: [
        {
          type: "selection.set",
          ids: context.elements.filter((element) => !element.hidden).map((element) => element.id),
        },
      ],
    };
  }

  if (commandModifier && key === "d" && context.selection.ids.length > 0) {
    return {
      commands: [{ type: "element.duplicate", ids: [...context.selection.ids] }],
    };
  }
  if (commandModifier && key === "d") {
    return { commands: [] };
  }

  if (commandModifier && input.shiftKey && key === "g" && context.selection.ids.length > 0) {
    return {
      commands: [{ type: "group.remove", elementIds: [...context.selection.ids] }],
    };
  }
  if (commandModifier && input.shiftKey && key === "g") {
    return { commands: [] };
  }

  if (commandModifier && key === "g" && context.selection.ids.length >= 2) {
    return {
      commands: [{ type: "group.create", ids: [...context.selection.ids] }],
    };
  }
  if (commandModifier && key === "g") {
    return { commands: [] };
  }

  if (commandModifier && input.shiftKey && key === "l" && context.selection.ids.length > 0) {
    return {
      commands: [{ type: "element.unlock", ids: [...context.selection.ids] }],
    };
  }
  if (commandModifier && input.shiftKey && key === "l") {
    return { commands: [] };
  }

  if (commandModifier && key === "l" && context.selection.ids.length > 0) {
    return {
      commands: [{ type: "element.lock", ids: [...context.selection.ids] }],
    };
  }
  if (commandModifier && key === "l") {
    return { commands: [] };
  }

  if (commandModifier && input.shiftKey && key === "h" && context.selection.ids.length > 0) {
    return {
      commands: [{ type: "element.show", ids: [...context.selection.ids] }],
    };
  }
  if (commandModifier && input.shiftKey && key === "h") {
    return { commands: [] };
  }

  if (commandModifier && key === "h" && context.selection.ids.length > 0) {
    return {
      commands: [{ type: "element.hide", ids: [...context.selection.ids] }],
    };
  }
  if (commandModifier && key === "h") {
    return { commands: [] };
  }

  if (commandModifier && key === "0") {
    return { commands: [{ type: "viewport.set", viewport: DEFAULT_WHITEBOARD_VIEWPORT }] };
  }

  if (commandModifier && (key === "=" || key === "+")) {
    return { commands: [{ type: "viewport.set", viewport: zoomViewport(context.viewport, 1.1) }] };
  }

  if (commandModifier && key === "-") {
    return { commands: [{ type: "viewport.set", viewport: zoomViewport(context.viewport, 1 / 1.1) }] };
  }

  if (input.key === "Delete" || input.key === "Backspace") {
    return context.selection.ids.length > 0
      ? { commands: [{ type: "element.remove", ids: [...context.selection.ids] }] }
      : { commands: [] };
  }

  const nudge = nudgeDelta(input.key, input.shiftKey ? 10 : 1);
  if (nudge && context.selection.ids.length > 0) {
    return {
      commands: [{ type: "element.translate", ids: [...context.selection.ids], delta: nudge }],
    };
  }
  if (nudge) {
    return { commands: [] };
  }

  if (input.key === "Escape") {
    return { commands: [{ type: "selection.clear" }], clearInteraction: true };
  }

  const shortcutTool = toolForShortcut(key);
  return shortcutTool ? { commands: [{ type: "tool.set", tool: shortcutTool }] } : null;
}

function nudgeDelta(key: string, amount: number) {
  switch (key) {
    case "ArrowUp":
      return { x: 0, y: -amount };
    case "ArrowRight":
      return { x: amount, y: 0 };
    case "ArrowDown":
      return { x: 0, y: amount };
    case "ArrowLeft":
      return { x: -amount, y: 0 };
    default:
      return null;
  }
}

function zoomViewport(viewport: WhiteboardViewport, factor: number): WhiteboardViewport {
  return {
    ...viewport,
    zoom: roundCoordinate(clampZoom(viewport.zoom * factor)),
  };
}
