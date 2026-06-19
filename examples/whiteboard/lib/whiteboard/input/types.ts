import type { WhiteboardCommand } from "../commands";
import type { WhiteboardSelectionHandle } from "../render/geometry";
import type {
  WhiteboardBounds,
  WhiteboardConnectorEndpoint,
  WhiteboardDocument,
  WhiteboardElement,
  WhiteboardElementId,
  WhiteboardPoint,
  WhiteboardSelection,
  WhiteboardTool,
  WhiteboardViewport,
} from "../render/model";

export type WhiteboardPointerInput = {
  readonly pointerId: number;
  readonly clientX: number;
  readonly clientY: number;
  readonly button?: number;
  readonly shiftKey?: boolean;
};

export type WhiteboardWheelInput = {
  readonly clientX: number;
  readonly clientY: number;
  readonly deltaX: number;
  readonly deltaY: number;
  readonly ctrlKey?: boolean;
  readonly metaKey?: boolean;
};

export type WhiteboardKeyboardInput = {
  readonly key: string;
  readonly ctrlKey?: boolean;
  readonly metaKey?: boolean;
  readonly shiftKey?: boolean;
};

export type WhiteboardInputState = {
  readonly tool: WhiteboardTool;
  readonly viewport: WhiteboardViewport;
  readonly selection: WhiteboardSelection;
  readonly draft: WhiteboardElement | null;
  readonly selectionArea: WhiteboardBounds | null;
  readonly activeTextId: WhiteboardElementId | null;
};

export type WhiteboardInputResult = {
  readonly commands: WhiteboardCommand[];
  readonly state: WhiteboardInputState;
};

export type WhiteboardInputRuntimeOptions = {
  readonly document: WhiteboardDocument;
  readonly tool?: WhiteboardTool;
  readonly viewport?: WhiteboardViewport;
  readonly selection?: WhiteboardSelection;
  readonly idFactory?: () => string;
};

export type WhiteboardDrawDrag = {
  kind: "draw";
  readonly pointerId: number;
  readonly tool: Extract<WhiteboardTool, "rectangle" | "ellipse" | "diamond" | "line" | "arrow">;
  readonly startWorld: WhiteboardPoint;
  readonly startBinding?: WhiteboardConnectorEndpoint;
  currentWorld: WhiteboardPoint;
  endBinding?: WhiteboardConnectorEndpoint;
};

export type WhiteboardSelectionAreaDrag = {
  readonly kind: "select-area";
  readonly pointerId: number;
  readonly startWorld: WhiteboardPoint;
  currentWorld: WhiteboardPoint;
  readonly extend?: boolean;
};

export type ActiveDrag =
  | WhiteboardSelectionAreaDrag
  | {
      readonly kind: "pan";
      readonly pointerId: number;
      readonly startClient: WhiteboardPoint;
      readonly startViewport: WhiteboardViewport;
    }
  | {
      readonly kind: "move";
      readonly pointerId: number;
      readonly startWorld: WhiteboardPoint;
      readonly elements: readonly WhiteboardElement[];
    }
  | {
      readonly kind: "resize";
      readonly pointerId: number;
      readonly handle: WhiteboardSelectionHandle;
      readonly element: WhiteboardElement;
      readonly initialBounds: WhiteboardBounds;
    }
  | WhiteboardDrawDrag
  | {
      readonly kind: "freehand";
      readonly pointerId: number;
      readonly points: WhiteboardPoint[];
    };
