import { whiteboardActions } from "../../lib/stores/whiteboard-store";
import {
  whiteboardInputController,
  whiteboardKeyboardInputRecognized,
  whiteboardInputResultHandled,
} from "../../lib/stores/whiteboard-input-controller";
import { fitViewportToBounds } from "../../lib/whiteboard/render/geometry";
import { getDocumentContentBounds, getDocumentSelectionBounds, getSelectedElements } from "../../lib/whiteboard/scene";
import { ArrangePanel } from "./arrange-panel";
import { CanvasStage } from "./canvas-stage";
import { DocumentPanel } from "./document-panel";
import { ExportPanel } from "./export-panel";
import { ImportPanel } from "./import-panel";
import { Inspector } from "./inspector";
import { LibraryPanel } from "./library-panel";
import { MeasurementPanel } from "./measurement-panel";
import { MinimapPanel } from "./minimap-panel";
import { OutlinePanel } from "./outline-panel";
import { PresentationPanel } from "./presentation-panel";
import { SharePanel } from "./share-panel";
import { StatusBar } from "./status-bar";
import { Toolbar } from "./toolbar";

export function WhiteboardApp() {
  const state = whiteboardActions.snapshot();
  const document = state.document;
  const selectedElement = getSelectedElements(document)[0] ?? null;
  const stageSize = { width: 1200, height: 720 };
  const inputState = whiteboardInputController.snapshot();
  const handleKeyboardInput = (input: Parameters<typeof whiteboardInputController.keyDown>[0]) => {
    const result = whiteboardInputController.keyDown(input);
    return whiteboardInputResultHandled(result) || whiteboardKeyboardInputRecognized(input);
  };
  const handleZoomFit = () => {
    const bounds = getDocumentSelectionBounds(document) ?? getDocumentContentBounds(document);
    whiteboardActions.setViewport(bounds ? fitViewportToBounds(bounds, stageSize) : { x: 0, y: 0, zoom: 1 });
  };

  return (
    <main
      className="wb-workbench"
      data-dx-surface="whiteboard"
      data-dx-state-runtime="source-owned"
      data-dx-renderer="canvas"
    >
      <header className="wb-topbar">
        <div className="wb-brand">
          <span className="wb-brand-mark" aria-hidden="true" />
          <div>
            <p className="wb-eyebrow">DX WWW</p>
            <h1>{document.name}</h1>
          </div>
        </div>
        <div className="wb-topbar-meta" aria-label="Board summary">
          <span>{document.elements.length} elements</span>
          <span>{document.metadata?.gridVisible ? "Grid on" : "Grid off"}</span>
          <span>{document.metadata?.snapToGrid ? "Snap on" : "Snap off"}</span>
        </div>
      </header>

      <Toolbar
        activeTool={document.activeTool}
        canRedo={state.canRedo}
        canUndo={state.canUndo}
        onRedo={() => whiteboardActions.redo()}
        onToolChange={(tool) => whiteboardActions.setTool(tool)}
        onUndo={() => whiteboardActions.undo()}
        onZoomFit={handleZoomFit}
        onZoomIn={() => whiteboardActions.zoomViewport(document.viewport.zoom * 1.1)}
        onZoomOut={() => whiteboardActions.zoomViewport(document.viewport.zoom / 1.1)}
        zoom={Math.round(document.viewport.zoom * 100)}
      />

      <section className="wb-canvas-panel" aria-label="Whiteboard canvas workbench">
        <div className="wb-canvas-scroll">
          <CanvasStage
            activeTextId={inputState.activeTextId}
            document={document}
            inputState={inputState}
            onKeyboardInput={handleKeyboardInput}
            onCommitText={(id, text) => whiteboardActions.commitText(id, text)}
            onPointerDownInput={(input) =>
              whiteboardInputResultHandled(whiteboardInputController.pointerDown(input))}
            onPointerMoveInput={(input) =>
              whiteboardInputResultHandled(whiteboardInputController.pointerMove(input))}
            onPointerUpInput={(input) =>
              whiteboardInputResultHandled(whiteboardInputController.pointerUp(input))}
            onPointerCancelInput={(input) =>
              whiteboardInputResultHandled(whiteboardInputController.pointerUp(input))}
            onWheelInput={(input) =>
              whiteboardInputResultHandled(whiteboardInputController.wheel(input))}
            selection={{ ids: document.selection }}
            size={stageSize}
            tool={document.activeTool}
            viewport={document.viewport}
          />
          <p className="wb-canvas-proof" data-dx-whiteboard-runtime-proof="canvas-input-runtime">
            Canvas renderer and pointer input are source-owned under lib/whiteboard/render and
            lib/whiteboard/input.
          </p>
        </div>
        <MinimapPanel document={document} stageSize={stageSize} />
      </section>

      <aside className="wb-side-panel" aria-label="Whiteboard properties">
        <Inspector
          document={document}
          selectedElement={selectedElement}
          style={selectedElement?.style ?? document.currentStyle}
        />
        <MeasurementPanel document={document} stageSize={stageSize} />
        <ArrangePanel document={document} />
        <LibraryPanel />
        <PresentationPanel document={document} stageSize={stageSize} />
        <OutlinePanel document={document} stageSize={stageSize} />
        <DocumentPanel document={document} selectedElement={selectedElement} />
        <ImportPanel document={document} />
        <ExportPanel document={document} />
        <SharePanel document={document} />
      </aside>

      <StatusBar
        activeTool={document.activeTool}
        document={document}
        selectedElement={selectedElement}
      />
    </main>
  );
}
