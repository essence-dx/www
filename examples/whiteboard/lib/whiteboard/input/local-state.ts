import type { WhiteboardCommand } from "../commands";
import type { WhiteboardSelection, WhiteboardTool, WhiteboardViewport } from "../render/model";

export type WhiteboardInputLocalState = {
  readonly selection: WhiteboardSelection;
  readonly viewport: WhiteboardViewport;
  readonly tool: WhiteboardTool;
};

export function applyInputLocalCommands(
  current: WhiteboardInputLocalState,
  commands: readonly WhiteboardCommand[],
): WhiteboardInputLocalState {
  let selection = current.selection;
  let viewport = current.viewport;
  let tool = current.tool;

  for (const command of commands) {
    switch (command.type) {
      case "selection.set":
        selection = { ids: [...command.ids] };
        break;
      case "selection.clear":
        selection = { ids: [] };
        break;
      case "viewport.set":
        viewport = { ...viewport, ...command.viewport };
        break;
      case "tool.set":
        tool = command.tool;
        break;
      case "element.remove":
      case "element.hide": {
        const removed = new Set(command.ids);
        selection = { ids: selection.ids.filter((id) => !removed.has(id)) };
        break;
      }
    }
  }

  return { selection, viewport, tool };
}
