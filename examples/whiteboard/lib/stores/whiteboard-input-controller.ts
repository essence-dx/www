import {
  createWhiteboardInputRuntime,
  type WhiteboardInputResult,
  type WhiteboardInputRuntime,
  type WhiteboardInputState,
  type WhiteboardKeyboardInput,
  type WhiteboardPointerInput,
  type WhiteboardWheelInput,
} from "../whiteboard/input/input-runtime";
import { toolForShortcut } from "../whiteboard/input/helpers";
import { whiteboardStore, type WhiteboardStoreApi } from "./whiteboard-store";

type WhiteboardPointerMoveInput = Omit<WhiteboardPointerInput, "button">;

export interface CreateWhiteboardInputControllerOptions {
  readonly idFactory?: () => string;
}

export interface WhiteboardInputController {
  readonly pointerDown: (input: WhiteboardPointerInput) => WhiteboardInputResult;
  readonly pointerMove: (input: WhiteboardPointerMoveInput) => WhiteboardInputResult;
  readonly pointerUp: (input: WhiteboardPointerMoveInput) => WhiteboardInputResult;
  readonly wheel: (input: WhiteboardWheelInput) => WhiteboardInputResult;
  readonly keyDown: (input: WhiteboardKeyboardInput) => WhiteboardInputResult;
  readonly snapshot: () => WhiteboardInputState;
  readonly sync: () => void;
  readonly destroy: () => void;
}

export function createWhiteboardInputController(
  storeApi: WhiteboardStoreApi,
  options: CreateWhiteboardInputControllerOptions = {},
): WhiteboardInputController {
  const runtime = createRuntime(storeApi, options);
  let dispatching = false;

  const unsubscribe = storeApi.subscribe((state) => {
    if (!dispatching) {
      syncRuntime(runtime, state.document);
    }
  });

  const run = (result: WhiteboardInputResult): WhiteboardInputResult => {
    if (result.commands.length > 0) {
      dispatching = true;
      try {
        storeApi.dispatchBatch(result.commands);
      } finally {
        dispatching = false;
      }
    }

    syncRuntime(runtime, storeApi.getDocument());
    return result;
  };

  return {
    pointerDown: (input) => run(runtime.pointerDown(input)),
    pointerMove: (input) => run(runtime.pointerMove(input)),
    pointerUp: (input) => run(runtime.pointerUp(input)),
    wheel: (input) => run(runtime.wheel(input)),
    keyDown: (input) => run(runtime.keyDown(input)),
    snapshot: () => runtime.state(),
    sync: () => {
      syncRuntime(runtime, storeApi.getDocument());
    },
    destroy: unsubscribe,
  };
}

export function whiteboardInputResultHandled(result: WhiteboardInputResult): boolean {
  return (
    result.commands.length > 0 ||
    result.state.draft !== null ||
    result.state.selectionArea !== null ||
    result.state.activeTextId !== null
  );
}

export function whiteboardKeyboardInputRecognized(input: WhiteboardKeyboardInput): boolean {
  const key = input.key.toLowerCase();
  const commandModifier = input.ctrlKey || input.metaKey;

  return Boolean(
    (commandModifier && ["a", "d", "g", "l", "h", "0", "=", "+", "-"].includes(key)) ||
      input.key === "Delete" ||
      input.key === "Backspace" ||
      input.key === "Escape" ||
      key.startsWith("arrow") ||
      (!commandModifier && toolForShortcut(key)),
  );
}

export const whiteboardInputController = createWhiteboardInputController(whiteboardStore);

function createRuntime(
  storeApi: WhiteboardStoreApi,
  options: CreateWhiteboardInputControllerOptions,
): WhiteboardInputRuntime {
  const document = storeApi.getDocument();
  return createWhiteboardInputRuntime({
    document,
    idFactory: options.idFactory,
    selection: { ids: document.selection },
    tool: document.activeTool,
    viewport: document.viewport,
  });
}

function syncRuntime(runtime: WhiteboardInputRuntime, document = whiteboardStore.getDocument()): void {
  runtime.setDocumentState({
    document,
    selection: { ids: document.selection },
    tool: document.activeTool,
    viewport: document.viewport,
  });
}
