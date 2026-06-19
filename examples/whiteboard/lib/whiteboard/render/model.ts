export type {
  WhiteboardDocument,
  WhiteboardConnectorAnchor,
  WhiteboardConnectorEndpoint,
  WhiteboardElement,
  WhiteboardElementId,
  WhiteboardElementPatch,
  WhiteboardElementRole,
  WhiteboardGroup,
  WhiteboardGroupId,
  WhiteboardImageElement,
  WhiteboardPoint,
  WhiteboardRect as WhiteboardBounds,
  WhiteboardSize,
  WhiteboardTextElement,
  WhiteboardTool,
  WhiteboardViewport,
} from "../model";
export {
  DEFAULT_WHITEBOARD_STYLE,
  DEFAULT_WHITEBOARD_VIEWPORT,
  makeElementId,
} from "../model";

import { DEFAULT_WHITEBOARD_STYLE, type WhiteboardElementId } from "../model";

export type WhiteboardSelection = {
  readonly ids: readonly WhiteboardElementId[];
};

export const DEFAULT_WHITEBOARD_SELECTION: WhiteboardSelection = {
  ids: [],
};

export const DEFAULT_STROKE = DEFAULT_WHITEBOARD_STYLE.stroke;
export const DEFAULT_FILL = DEFAULT_WHITEBOARD_STYLE.fill;
