import { TextEditorOverlay } from "./text-editor-overlay";
import { SvgStagePreview } from "./svg-stage-preview";
import { frameIdForElement, isFrameElement } from "../../lib/whiteboard/frames";
import {
  DEFAULT_WHITEBOARD_SELECTION,
  DEFAULT_WHITEBOARD_VIEWPORT,
  type WhiteboardDocument,
  type WhiteboardElement,
  type WhiteboardElementId,
  type WhiteboardSelection,
  type WhiteboardSize,
  type WhiteboardTextElement,
  type WhiteboardTool,
  type WhiteboardViewport,
} from "../../lib/whiteboard/render/model";
import type {
  WhiteboardInputState,
  WhiteboardKeyboardInput,
  WhiteboardPointerInput,
  WhiteboardWheelInput,
} from "../../lib/whiteboard/input/types";

type WhiteboardPointerMoveInput = Omit<WhiteboardPointerInput, "button">;

type CanvasStageProps = {
  document: WhiteboardDocument;
  size?: WhiteboardSize;
  viewport?: WhiteboardViewport;
  selection?: WhiteboardSelection;
  tool?: WhiteboardTool;
  activeTextId?: WhiteboardElementId | null;
  inputState?: WhiteboardInputState;
  onCommitText?: (id: WhiteboardElementId, text: string) => void;
  onKeyboardInput?: (input: WhiteboardKeyboardInput) => boolean;
  onPointerDownInput?: (input: WhiteboardPointerInput) => boolean;
  onPointerMoveInput?: (input: WhiteboardPointerMoveInput) => boolean;
  onPointerUpInput?: (input: WhiteboardPointerMoveInput) => boolean;
  onPointerCancelInput?: (input: WhiteboardPointerMoveInput) => boolean;
  onWheelInput?: (input: WhiteboardWheelInput) => boolean;
  className?: string;
  canvasClassName?: string;
  label?: string;
};

type SerializedCanvasStage = {
  schema: "dx.whiteboard.canvas-stage";
  renderer: "dx.whiteboard.canvas-renderer";
  input: "dx.whiteboard.input-runtime";
  tool: WhiteboardTool;
  size: WhiteboardSize;
  viewport: WhiteboardViewport;
  selection: WhiteboardSelection;
  document: WhiteboardDocument;
};

const DEFAULT_STAGE_SIZE: WhiteboardSize = {
  width: 1280,
  height: 720,
};

export function CanvasStage({
  document,
  size = DEFAULT_STAGE_SIZE,
  viewport = DEFAULT_WHITEBOARD_VIEWPORT,
  selection = DEFAULT_WHITEBOARD_SELECTION,
  tool = "select",
  activeTextId = null,
  inputState,
  onCommitText,
  onKeyboardInput,
  onPointerDownInput,
  onPointerMoveInput,
  onPointerUpInput,
  onPointerCancelInput,
  onWheelInput,
  className = "whiteboard-stage",
  canvasClassName = "whiteboard-canvas",
  label = "Whiteboard canvas",
}: CanvasStageProps) {
  const activeTextElement = textElementById(document.elements, activeTextId);
  const frameCount = document.elements.filter(isFrameElement).length;
  const framedElementCount = document.elements.filter((element) => Boolean(frameIdForElement(element))).length;
  const serializedStage = serializeCanvasStage({
    schema: "dx.whiteboard.canvas-stage",
    renderer: "dx.whiteboard.canvas-renderer",
    input: "dx.whiteboard.input-runtime",
    tool,
    size,
    viewport,
    selection,
    document,
  });

  return (
    <section
      className={className}
      data-whiteboard-stage="source-owned-canvas"
      data-whiteboard-renderer="dx.whiteboard.canvas-renderer"
      data-whiteboard-input-runtime="dx.whiteboard.input-runtime"
      data-whiteboard-selection-area="runtime-ephemeral"
      data-whiteboard-marquee-selection="runtime-ephemeral"
      data-whiteboard-keyboard-workflows="command-backed"
      data-whiteboard-connector-bindings="pointer-runtime"
      data-whiteboard-connector-rerouting="scene-reducer-live"
      data-whiteboard-connector-anchor-policy="auto-edge"
      data-whiteboard-pointer-workflows="controller-backed"
      data-whiteboard-frame-membership="metadata-backed"
      data-whiteboard-frame-commands="frame.assign frame.clear"
      data-whiteboard-frame-count={frameCount}
      data-whiteboard-framed-element-count={framedElementCount}
      data-whiteboard-tool={tool}
      data-whiteboard-selection={selection.ids.join(" ")}
      data-whiteboard-grid={Boolean(document.metadata?.gridVisible)}
      data-whiteboard-grid-size={document.metadata?.gridSize ?? 24}
      data-whiteboard-snap={Boolean(document.metadata?.snapToGrid)}
      data-whiteboard-revision={String(document.metadata?.revision ?? 0)}
    >
      <SvgStagePreview
        document={document}
        draft={inputState?.draft ?? null}
        height={size.height}
        selection={selection}
        selectionArea={inputState?.selectionArea ?? null}
        viewport={viewport}
        width={size.width}
      />
      <canvas
        aria-label={label}
        className={canvasClassName}
        data-whiteboard-canvas="source-owned"
        data-whiteboard-pointer-controller="store-backed"
        data-whiteboard-stage-state={serializedStage}
        data-whiteboard-viewport={serializeViewport(viewport)}
        height={size.height}
        onKeyDown={(event) => {
          const handled = onKeyboardInput?.({
            key: event.key,
            ctrlKey: event.ctrlKey,
            metaKey: event.metaKey,
            shiftKey: event.shiftKey,
          });

          if (handled) {
            event.preventDefault();
          }
        }}
        onPointerDown={(event) => {
          const point = canvasLocalPoint(event);
          const handled = onPointerDownInput?.({
            button: event.button,
            clientX: point.x,
            clientY: point.y,
            pointerId: event.pointerId,
            shiftKey: event.shiftKey,
          });

          if (handled || event.button === 0) {
            event.currentTarget.setPointerCapture?.(event.pointerId);
            event.preventDefault();
          }
        }}
        onPointerMove={(event) => {
          const point = canvasLocalPoint(event);
          const handled = onPointerMoveInput?.({
            clientX: point.x,
            clientY: point.y,
            pointerId: event.pointerId,
            shiftKey: event.shiftKey,
          });

          if (handled) {
            event.preventDefault();
          }
        }}
        onPointerUp={(event) => {
          const point = canvasLocalPoint(event);
          const handled = onPointerUpInput?.({
            clientX: point.x,
            clientY: point.y,
            pointerId: event.pointerId,
            shiftKey: event.shiftKey,
          });

          event.currentTarget.releasePointerCapture?.(event.pointerId);
          if (handled) {
            event.preventDefault();
          }
        }}
        onPointerCancel={(event) => {
          const point = canvasLocalPoint(event);
          const handled = onPointerCancelInput?.({
            clientX: point.x,
            clientY: point.y,
            pointerId: event.pointerId,
            shiftKey: event.shiftKey,
          });

          event.currentTarget.releasePointerCapture?.(event.pointerId);
          if (handled) {
            event.preventDefault();
          }
        }}
        onWheel={(event) => {
          const point = canvasLocalPoint(event);
          const handled = onWheelInput?.({
            clientX: point.x,
            clientY: point.y,
            ctrlKey: event.ctrlKey,
            deltaX: event.deltaX,
            deltaY: event.deltaY,
            metaKey: event.metaKey,
          });

          if (handled) {
            event.preventDefault();
          }
        }}
        tabIndex={0}
        width={size.width}
      />
      {activeTextElement ? (
        <TextEditorOverlay
          element={activeTextElement}
          key={activeTextElement.id}
          onCommit={onCommitText}
          viewport={viewport}
        />
      ) : null}
    </section>
  );
}

export function serializeCanvasStage(stage: SerializedCanvasStage): string {
  return JSON.stringify(stage);
}

function serializeViewport(viewport: WhiteboardViewport): string {
  return JSON.stringify(viewport);
}

function textElementById(elements: readonly WhiteboardElement[], id: WhiteboardElementId | null): WhiteboardTextElement | null {
  if (!id) {
    return null;
  }

  const element = elements.find((item) => item.id === id);
  return element?.type === "text" ? element : null;
}

function canvasLocalPoint(event: {
  readonly clientX: number;
  readonly clientY: number;
  readonly currentTarget: HTMLCanvasElement;
}): { readonly x: number; readonly y: number } {
  const rect = event.currentTarget.getBoundingClientRect();
  const scaleX = rect.width === 0 ? 1 : event.currentTarget.width / rect.width;
  const scaleY = rect.height === 0 ? 1 : event.currentTarget.height / rect.height;

  return {
    x: (event.clientX - rect.left) * scaleX,
    y: (event.clientY - rect.top) * scaleY,
  };
}
