import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { join } from "node:path";
import { describe, it } from "node:test";

const root = process.cwd();
const whiteboardRoot = join(root, "examples", "whiteboard");

async function source(path: string): Promise<string> {
  return readFile(join(whiteboardRoot, path), "utf8");
}

async function repoSource(...path: string[]): Promise<string> {
  return readFile(join(root, ...path), "utf8");
}

function escapeRegex(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

describe("whiteboard WWW shell contract", () => {
  it("uses DX-native TSX instead of importing the React runtime in the app shell", async () => {
    const app = await source(join("components", "whiteboard", "whiteboard-app.tsx"));
    const componentSources = await Promise.all([
      source(join("components", "whiteboard", "toolbar.tsx")),
      source(join("components", "whiteboard", "inspector.tsx")),
      source(join("components", "whiteboard", "document-panel.tsx")),
      source(join("components", "whiteboard", "export-panel.tsx")),
      source(join("components", "whiteboard", "import-panel.tsx")),
      source(join("components", "whiteboard", "share-panel.tsx")),
      source(join("components", "whiteboard", "text-controls.tsx")),
      source(join("components", "whiteboard", "connector-info.tsx")),
      source(join("components", "whiteboard", "status-bar.tsx")),
      source(join("lib", "stores", "whiteboard-input-controller.ts")),
    ]);

    assert.doesNotMatch(app, /from\s+["']react["']/);
    assert.doesNotMatch(componentSources.join("\n"), /from\s+["']react["']/);
    assert.match(app, /whiteboardActions/);
    assert.match(app, /whiteboardActions\.snapshot\(\)/);
    assert.doesNotMatch(app, /const\s+document\s*=\s*createDemoWhiteboardDocument\(\)/);
  });

  it("renders the source-owned canvas stage and points at the canonical input/render modules", async () => {
    const app = await source(join("components", "whiteboard", "whiteboard-app.tsx"));
    const canvasStage = await source(join("components", "whiteboard", "canvas-stage.tsx"));
    const statusBar = await source(join("components", "whiteboard", "status-bar.tsx"));
    const toolbar = await source(join("components", "whiteboard", "toolbar.tsx"));
    const keyboard = await source(join("lib", "whiteboard", "input", "keyboard.ts"));
    const library = await source(join("lib", "whiteboard", "library.ts"));
    const imageSource = await source(join("lib", "whiteboard", "image-source.ts"));
    const inputController = await source(join("lib", "stores", "whiteboard-input-controller.ts"));
    const inspector = await source(join("components", "whiteboard", "inspector.tsx"));
    const documentPanel = await source(join("components", "whiteboard", "document-panel.tsx"));
    const svgPreview = await source(join("components", "whiteboard", "svg-stage-preview.tsx"));
    const readme = await source("README.md");

    assert.match(app, /<CanvasStage/);
    assert.match(app, /whiteboardInputController\.keyDown/);
    assert.match(app, /fitViewportToBounds/);
    assert.match(app, /getDocumentSelectionBounds/);
    assert.match(app, /getDocumentContentBounds/);
    assert.doesNotMatch(app, /onZoomFit=\{\(\) => whiteboardActions\.setViewport\(\{ x: 0, y: 0, zoom: 1 \}\)\}/);
    assert.match(canvasStage, /data-whiteboard-renderer="dx\.whiteboard\.canvas-renderer"/);
    assert.match(canvasStage, /data-whiteboard-input-runtime="dx\.whiteboard\.input-runtime"/);
    assert.match(canvasStage, /data-whiteboard-selection-area="runtime-ephemeral"/);
    assert.match(canvasStage, /data-whiteboard-marquee-selection="runtime-ephemeral"/);
    assert.match(canvasStage, /data-whiteboard-keyboard-workflows="command-backed"/);
    assert.match(canvasStage, /data-whiteboard-connector-bindings="pointer-runtime"/);
    assert.match(canvasStage, /data-whiteboard-connector-rerouting="scene-reducer-live"/);
    assert.match(canvasStage, /data-whiteboard-connector-anchor-policy="auto-edge"/);
    assert.match(canvasStage, /data-whiteboard-pointer-workflows="controller-backed"/);
    assert.match(canvasStage, /data-whiteboard-pointer-controller="store-backed"/);
    assert.match(canvasStage, /onKeyDown/);
    assert.match(canvasStage, /onPointerDown/);
    assert.match(canvasStage, /onPointerMove/);
    assert.match(canvasStage, /onPointerUp/);
    assert.match(canvasStage, /onPointerCancel/);
    assert.match(canvasStage, /onWheel/);
    assert.match(canvasStage, /canvasLocalPoint\(event\)/);
    assert.match(canvasStage, /getBoundingClientRect\(\)/);
    assert.match(canvasStage, /setPointerCapture\?\.\(event\.pointerId\)/);
    assert.match(canvasStage, /releasePointerCapture\?\.\(event\.pointerId\)/);
    assert.match(canvasStage, /data-whiteboard-grid-size/);
    assert.match(app, /whiteboardInputController/);
    assert.match(app, /whiteboardKeyboardInputRecognized/);
    assert.match(app, /whiteboardInputResultHandled/);
    assert.match(app, /whiteboardInputController\.snapshot\(\)/);
    assert.match(app, /whiteboardInputController\.pointerDown\(input\)/);
    assert.match(app, /whiteboardInputController\.pointerMove\(input\)/);
    assert.match(app, /whiteboardInputController\.pointerUp\(input\)/);
    assert.match(app, /whiteboardInputController\.wheel\(input\)/);
    assert.doesNotMatch(app, /commandsForKeyboardInput/);
    assert.match(inputController, /createWhiteboardInputRuntime/);
    assert.match(inputController, /whiteboardKeyboardInputRecognized/);
    assert.match(inputController, /storeApi\.dispatchBatch\(result\.commands\)/);
    assert.match(inputController, /storeApi\.subscribe/);
    assert.match(canvasStage, /data-whiteboard-frame-membership="metadata-backed"/);
    assert.match(canvasStage, /data-whiteboard-frame-commands="frame\.assign frame\.clear"/);
    assert.match(canvasStage, /data-whiteboard-frame-count=\{frameCount\}/);
    assert.match(canvasStage, /data-whiteboard-framed-element-count=\{framedElementCount\}/);
    assert.match(statusBar, /data-whiteboard-keyboard-status="nudge-duplicate-group"/);
    assert.match(statusBar, /data-whiteboard-fit-target/);
    assert.match(statusBar, /data-whiteboard-selection-count/);
    assert.doesNotMatch(statusBar, /Keys on/);
    assert.match(toolbar, /data-whiteboard-tool-shortcut/);
    assert.doesNotMatch(toolbar, /aria-keyshortcuts/);
    assert.match(keyboard, /"element\.translate"/);
    assert.match(keyboard, /"element\.duplicate"/);
    assert.match(keyboard, /"group\.create"/);
    assert.match(keyboard, /"group\.remove"/);
    assert.match(library, /id: "image"/);
    assert.match(library, /data:image\/svg\+xml,/);
    assert.match(imageSource, /embedded-data-url-only/);
    assert.match(inspector, /data-whiteboard-image-controls="source-owned"/);
    assert.match(inspector, /data-whiteboard-frame-controls="command-backed"/);
    assert.match(inspector, /data-whiteboard-command="frame\.assign"/);
    assert.match(inspector, /data-whiteboard-command="frame\.clear"/);
    assert.match(inspector, /Image source/);
    assert.match(inspector, /Image alt text/);
    assert.match(documentPanel, /frameIdForElement/);
    assert.match(documentPanel, /data-whiteboard-frame-controls="command-backed"/);
    assert.match(documentPanel, /data-whiteboard-frame-storage="metadata\.frameId"/);
    assert.match(documentPanel, /data-whiteboard-command="frame\.assign"/);
    assert.match(documentPanel, /data-whiteboard-command="frame\.clear"/);
    assert.match(documentPanel, /data-whiteboard-frame-id=\{frameId \?\? "none"\}/);
    assert.match(documentPanel, /data-whiteboard-frame-member=\{Boolean\(frameId\)\}/);
    assert.match(svgPreview, /<image/);
    assert.match(svgPreview, /wb-preview-image/);
    assert.match(svgPreview, /data-whiteboard-frame-membership="metadata-backed"/);
    assert.match(svgPreview, /data-whiteboard-frame-id=\{frameId\}/);
    assert.match(svgPreview, /data-whiteboard-role=\{isFrameElement\(element\)/);
    assert.match(svgPreview, /data-whiteboard-connector-route=\{connectorRouteForElement\(element\)\}/);
    assert.match(svgPreview, /data-whiteboard-runtime-draft="input-runtime-ephemeral"/);
    assert.match(svgPreview, /data-whiteboard-selection-area-preview="input-runtime-ephemeral"/);
    assert.match(svgPreview, /data-whiteboard-element-id=\{element\.id\}/);
    assert.match(svgPreview, /data-whiteboard-connector-type=\{element\.type\}/);
    assert.match(svgPreview, /data-whiteboard-start-binding=\{element\.startBinding\?\.elementId \?\? "unbound"\}/);
    assert.match(svgPreview, /data-whiteboard-end-binding=\{element\.endBinding\?\.elementId \?\? "unbound"\}/);
    assert.match(readme, /browser-level E2E receipts are not claimed/);
  });

  it("routes grid controls through a source-owned grid command", async () => {
    const documentPanel = await source(join("components", "whiteboard", "document-panel.tsx"));
    const commands = await source(join("lib", "whiteboard", "commands.ts"));

    assert.match(documentPanel, /data-whiteboard-command="grid\.set"/);
    assert.match(documentPanel, /setGridSettings/);
    assert.match(commands, /readonly type: "grid\.set"/);
  });

  it("exposes source-owned measurements for selection and document bounds", async () => {
    const app = await source(join("components", "whiteboard", "whiteboard-app.tsx"));
    const measurementPanel = await source(join("components", "whiteboard", "measurement-panel.tsx"));
    const measurementModel = await source(join("lib", "whiteboard", "measurements.ts"));
    const styles = await source(join("styles", "whiteboard.css"));

    assert.match(app, /<MeasurementPanel/);
    assert.match(app, /document=\{document\}/);
    assert.match(app, /stageSize=\{stageSize\}/);
    assert.match(measurementPanel, /createWhiteboardMeasurementModel/);
    assert.match(measurementPanel, /measurementCommandsForFocus/);
    assert.match(measurementPanel, /whiteboardActions\.dispatchBatch/);
    assert.match(measurementPanel, /data-whiteboard-measurements="source-owned"/);
    assert.match(measurementPanel, /data-whiteboard-measurement-subject=\{measurements\.subject\}/);
    assert.match(measurementPanel, /data-whiteboard-measurement-item-count=\{measurements\.counts\.items\}/);
    assert.match(measurementPanel, /data-whiteboard-measurement-can-focus=\{measurements\.canFocus\}/);
    assert.match(measurementPanel, /data-whiteboard-measurement-count=\{field\.key\}/);
    assert.match(measurementPanel, /data-whiteboard-command="viewport\.set"/);
    assert.match(measurementPanel, /disabled=\{!measurements\.canFocus\}/);
    assert.match(measurementModel, /getDocumentSelectionBounds/);
    assert.match(measurementModel, /getDocumentContentBounds/);
    assert.match(measurementModel, /fitViewportToBounds\(bounds, stageSize, \{ padding: 84 \}\)/);
    assert.match(measurementModel, /type: "viewport\.set"/);
    assert.match(styles, /\.wb-measurement-panel/);
    assert.match(styles, /\.wb-measurement-grid/);
    assert.doesNotMatch(`${measurementPanel}\n${measurementModel}`, /from\s+["']react["']|from\s+["']react\/|useState|useEffect|^["']use client["']/m);
  });

  it("exposes a model-backed minimap with command-backed viewport navigation", async () => {
    const app = await source(join("components", "whiteboard", "whiteboard-app.tsx"));
    const minimapPanel = await source(join("components", "whiteboard", "minimap-panel.tsx"));
    const minimapModel = await source(join("lib", "whiteboard", "minimap.ts"));
    const styles = await source(join("styles", "whiteboard.css"));

    assert.match(app, /<MinimapPanel/);
    assert.match(app, /stageSize=\{stageSize\}/);
    assert.match(minimapPanel, /createWhiteboardMinimapModel/);
    assert.match(minimapPanel, /minimapPointToViewport/);
    assert.match(minimapPanel, /whiteboardActions\.setViewport/);
    assert.match(minimapPanel, /data-whiteboard-minimap="model-backed"/);
    assert.match(minimapPanel, /data-whiteboard-minimap-action="pan-to-point"/);
    assert.match(minimapPanel, /data-whiteboard-minimap-stage-size=\{`\$\{stageSize\.width\}x\$\{stageSize\.height\}`\}/);
    assert.match(minimapPanel, /data-whiteboard-minimap-scale=\{String\(minimap\.scale\)\}/);
    assert.match(minimapPanel, /data-whiteboard-minimap-bounds=\{formatRect\(minimap\.documentBounds\)\}/);
    assert.match(minimapPanel, /data-whiteboard-minimap-viewport="viewport-state"/);
    assert.match(minimapPanel, /data-whiteboard-command="viewport\.set"/);
    assert.match(minimapPanel, /onClick/);
    assert.match(minimapPanel, /getBoundingClientRect\(\)/);
    assert.match(minimapPanel, /data-whiteboard-element-id=\{element\.id\}/);
    assert.match(minimapPanel, /aria-label="Pan whiteboard viewport from minimap"/);
    assert.doesNotMatch(`${minimapPanel}\n${minimapModel}`, /from\s+["']react["']|from\s+["']react\/|useState|useEffect|^["']use client["']/m);
    assert.match(styles, /\.wb-minimap-panel/);
    assert.match(styles, /\.wb-minimap-viewport/);
  });

  it("exposes a source-owned presentation navigator for frame slides", async () => {
    const app = await source(join("components", "whiteboard", "whiteboard-app.tsx"));
    const presentationPanel = await source(join("components", "whiteboard", "presentation-panel.tsx"));
    const presentationModel = await source(join("lib", "whiteboard", "presentation.ts"));
    const styles = await source(join("styles", "whiteboard.css"));

    assert.match(app, /<PresentationPanel/);
    assert.match(app, /stageSize=\{stageSize\}/);
    assert.match(presentationPanel, /createWhiteboardPresentationModel/);
    assert.match(presentationPanel, /presentationCommandsForSlide/);
    assert.match(presentationPanel, /whiteboardActions\.dispatchBatch/);
    assert.match(presentationPanel, /data-whiteboard-presentation="frame-navigator"/);
    assert.match(presentationPanel, /data-whiteboard-presentation-slide-count=\{presentation\.slides\.length\}/);
    assert.match(presentationPanel, /data-whiteboard-presentation-current-index=\{presentation\.currentIndex\}/);
    assert.match(presentationPanel, /data-whiteboard-presentation-slide-id=\{slide\.id\}/);
    assert.match(presentationPanel, /data-whiteboard-command="selection\.set viewport\.set"/);
    assert.match(presentationModel, /frameIdForElement/);
    assert.match(presentationModel, /fitViewportToBounds/);
    assert.match(presentationModel, /type: "selection\.set"/);
    assert.match(presentationModel, /type: "viewport\.set"/);
    assert.match(styles, /\.wb-presentation-panel/);
    assert.match(styles, /\.wb-presentation-slide/);
    assert.doesNotMatch(`${presentationPanel}\n${presentationModel}`, /from\s+["']react["']|from\s+["']react\/|useState|useEffect|^["']use client["']/m);
  });

  it("exposes a source-owned outline navigator with grouped focus markers", async () => {
    const app = await source(join("components", "whiteboard", "whiteboard-app.tsx"));
    const outlinePanel = await source(join("components", "whiteboard", "outline-panel.tsx"));
    const outlineModel = await source(join("lib", "whiteboard", "outline.ts"));
    const styles = await source(join("styles", "whiteboard.css"));

    assert.match(app, /<OutlinePanel/);
    assert.match(app, /document=\{document\}/);
    assert.match(app, /stageSize=\{stageSize\}/);
    assert.match(outlinePanel, /createWhiteboardOutlineModel/);
    assert.match(outlinePanel, /outlineCommandsForItem/);
    assert.match(outlinePanel, /whiteboardActions\.dispatchBatch/);
    assert.match(outlinePanel, /data-whiteboard-outline="frame-grouped"/);
    assert.match(outlinePanel, /data-whiteboard-outline-section-count=\{outline\.sections\.length\}/);
    assert.match(outlinePanel, /data-whiteboard-outline-selected-count=\{outline\.selectedItemCount\}/);
    assert.match(outlinePanel, /data-whiteboard-outline-section=\{section\.id\}/);
    assert.match(outlinePanel, /data-whiteboard-outline-frame-id=\{section\.frameId \?\? "none"\}/);
    assert.match(outlinePanel, /data-whiteboard-outline-item-id=\{item\.id\}/);
    assert.match(outlinePanel, /data-whiteboard-outline-item-role=\{item\.role\}/);
    assert.match(outlinePanel, /data-whiteboard-outline-item-type=\{item\.type\}/);
    assert.match(outlinePanel, /data-whiteboard-command="selection\.set viewport\.set"/);
    assert.match(outlinePanel, /aria-selected=\{item\.selected\}/);
    assert.match(outlinePanel, /data-active=\{item\.selected\}/);
    assert.match(outlinePanel, /data-hidden=\{item\.hidden\}/);
    assert.match(outlinePanel, /data-locked=\{item\.locked\}/);
    assert.match(outlineModel, /frameIdForElement/);
    assert.match(outlineModel, /fitViewportToBounds\(bounds, stageSize, \{ padding: 96 \}\)/);
    assert.match(outlineModel, /type: "selection\.set"/);
    assert.match(outlineModel, /type: "viewport\.set"/);
    assert.match(styles, /\.wb-outline-panel/);
    assert.match(styles, /\.wb-outline-item:focus-visible/);
    assert.doesNotMatch(`${outlinePanel}\n${outlineModel}`, /from\s+["']react["']|from\s+["']react\/|useState|useEffect|^["']use client["']/m);
  });

  it("exposes schema-backed file workflow panels without React runtime packages", async () => {
    const app = await source(join("components", "whiteboard", "whiteboard-app.tsx"));
    const importPanel = await source(join("components", "whiteboard", "import-panel.tsx"));
    const imageImport = await source(join("lib", "whiteboard", "image-import.ts"));
    const store = await source(join("lib", "stores", "whiteboard-store.ts"));
    const sharePanel = await source(join("components", "whiteboard", "share-panel.tsx"));

    assert.match(app, /<ImportPanel/);
    assert.match(app, /<SharePanel/);
    assert.match(importPanel, /validateWhiteboardImport/);
    assert.match(importPanel, /Open Files/);
    assert.match(importPanel, /Add Image/);
    assert.match(importPanel, /data-whiteboard-file-validation="schema-validated"/);
    assert.match(importPanel, /data-whiteboard-image-policy="embedded-data-url-only"/);
    assert.match(importPanel, /accept="image\/png,image\/jpeg,image\/webp,image\/svg\+xml"/);
    assert.match(importPanel, /FileReader/);
    assert.match(importPanel, /readAsDataURL/);
    assert.match(importPanel, /createImportedImageElement/);
    assert.match(importPanel, /whiteboardActions\.importImage/);
    assert.match(importPanel, /aria-describedby="wb-image-import-status"/);
    assert.match(importPanel, /data-whiteboard-file-status="visible"/);
    assert.match(importPanel, /data-whiteboard-image-status="visible"/);
    assert.doesNotMatch(importPanel, /URL\.createObjectURL|blob:|file:\/\//);
    assert.match(imageImport, /WHITEBOARD_IMAGE_SOURCE_POLICY/);
    assert.match(imageImport, /image\.unsupported_source/);
    assert.match(imageImport, /image\.too_large/);
    assert.match(store, /importImage/);
    assert.match(store, /type: "element\.add"/);
    assert.match(store, /select: true/);
    assert.match(importPanel, /aria-live="polite"/);
    assert.match(importPanel, /role="status"/);
    assert.match(sharePanel, /createWhiteboardShareReceipt/);
    assert.match(sharePanel, /data-dx-whiteboard-share="receipt-backed"/);
    assert.doesNotMatch(`${importPanel}\n${sharePanel}`, /from\s+["']react["']/);
  });

  it("keeps file input focus visible through the styled import button", async () => {
    const styles = await source(join("styles", "whiteboard.css"));

    assert.match(styles, /\.wb-file-button:focus-within/);
    assert.match(styles, /\.wb-import-status/);
  });

  it("keeps the direct workbench bottom bar media and font controls source-owned", async () => {
    const page = await source(join("app", "page.tsx"));
    const runtime = await source(join("public", "whiteboard-runtime.ts"));
    const styles = await source(join("styles", "whiteboard.css"));
    const toolbar = await source(join("components", "whiteboard", "toolbar.tsx"));
    const rustIcons = await repoSource("related-crates", "media-icon", "src", "lib.rs");
    const toolbarIconNames = Array.from(toolbar.matchAll(/\|\s+"([^"]+)"/g), (match) => match[1]);
    const directIconNames = Array.from(page.matchAll(/(?:name|data-whiteboard-icon-insert)="whiteboard:([^"]+)"/g), (match) => match[1]);
    const coveredIconNames = Array.from(new Set([...toolbarIconNames, ...directIconNames])).sort();

    assert.match(page, /data-whiteboard-route="direct-renderable-workbench"/);
    assert.doesNotMatch(page, /wb-topbar/);
    assert.match(page, /wb-toolbar-actions/);
    assert.match(page, /data-whiteboard-icon-menu-trigger/);
    assert.match(page, /data-whiteboard-icon-insert="whiteboard:select"/);
    assert.match(page, /data-whiteboard-media-pick="image"/);
    assert.match(page, /data-whiteboard-media-input="video"/);
    assert.match(page, /data-whiteboard-font-input/);

    assert.match(runtime, /MAX_INLINE_MEDIA_BYTES/);
    assert.match(runtime, /document\.head\.querySelector/);
    assert.match(runtime, /function addIconObject/);
    assert.match(runtime, /function addMediaObject/);
    assert.match(runtime, /target\.closest\?\.\("audio, video"\)/);
    assert.match(runtime, /closeTransientUi/);
    assert.match(runtime, /nodes\("\[data-dx-icon\], dx-icon\[name\]"\)/);
    assert.ok(toolbarIconNames.length > 20);
    assert.ok(directIconNames.length > 20);
    for (const expected of ["icons", "audio", "video", "keyboard", "panel", "table", "math"]) {
      assert.ok(coveredIconNames.includes(expected), `direct page should cover whiteboard:${expected}`);
    }
    for (const name of coveredIconNames) {
      const runtimeKey = new RegExp(`(?:^|\\n)\\s*(?:"${escapeRegex(name)}"|${escapeRegex(name)}):\\s*'`, "m");
      const rustKey = new RegExp(`\\("whiteboard", "${escapeRegex(name)}"\\)`);
      assert.match(runtime, runtimeKey, `runtime renders whiteboard:${name}`);
      assert.match(rustIcons, rustKey, `Rust icon adapter resolves whiteboard:${name}`);
    }

    assert.match(styles, /\.wb-toolbar/);
    assert.match(styles, /bottom: 14px/);
    assert.match(styles, /\.wb-hidden-file-input/);
    assert.match(styles, /\[data-whiteboard-media-input\]/);
    assert.match(styles, /\.wb-file-button \.wb-file-input/);
    assert.match(styles, /\.wb-foreign-object/);
    assert.match(styles, /\.wb-workbench\[data-whiteboard-toolbar-popover-open="true"\] \.wb-toolbar/);
  });

  it("exposes command-backed semantic whiteboard controls", async () => {
    const arrangePanel = await source(join("components", "whiteboard", "arrange-panel.tsx"));
    const libraryPanel = await source(join("components", "whiteboard", "library-panel.tsx"));
    const library = await source(join("lib", "whiteboard", "library.ts"));
    const libraryTemplates = await source(join("lib", "whiteboard", "library-templates.ts"));
    const connectorInfo = await source(join("components", "whiteboard", "connector-info.tsx"));
    const textControls = await source(join("components", "whiteboard", "text-controls.tsx"));
    const textOverlay = await source(join("components", "whiteboard", "text-editor-overlay.tsx"));
    const canvasStage = await source(join("components", "whiteboard", "canvas-stage.tsx"));
    const app = await source(join("components", "whiteboard", "whiteboard-app.tsx"));
    const styles = await source(join("styles", "whiteboard.css"));

    assert.match(arrangePanel, /data-whiteboard-group-controls="command-backed"/);
    assert.match(arrangePanel, /data-whiteboard-command="group\.create"/);
    assert.match(arrangePanel, /data-whiteboard-command="group\.remove"/);
    assert.match(libraryPanel, /data-whiteboard-library="command-backed"/);
    assert.match(libraryPanel, /data-whiteboard-library-shell="grouped-presets"/);
    assert.match(libraryPanel, /data-whiteboard-library-template-count=\{WHITEBOARD_LIBRARY_TEMPLATE_PRESETS\.length\}/);
    assert.match(libraryPanel, /data-whiteboard-library-group=\{group\.id\}/);
    assert.match(libraryPanel, /data-whiteboard-library-group-count=\{group\.presets\.length\}/);
    assert.match(libraryPanel, /data-whiteboard-preset-category=\{preset\.category\}/);
    assert.match(libraryPanel, /data-whiteboard-template=\{preset\.category === "template" \? "source-owned" : undefined\}/);
    assert.match(libraryPanel, /data-whiteboard-element-count=\{preset\.elementCount\}/);
    assert.match(libraryPanel, /data-whiteboard-preset=\{preset\.id\}/);
    assert.match(libraryPanel, /whiteboardActions\.insertLibraryPreset/);
    assert.match(library, /WHITEBOARD_LIBRARY_TEMPLATE_PRESETS/);
    assert.match(library, /id: "flowchart-basic"/);
    assert.match(library, /id: "kanban-board"/);
    assert.match(library, /id: "retrospective-board"/);
    assert.match(library, /id: "system-map"/);
    assert.match(library, /createTemplatePresetElements/);
    assert.match(libraryTemplates, /sourceOwned: true/);
    assert.match(libraryTemplates, /connectorRoute: "orthogonal"/);
    assert.match(styles, /\.wb-library-groups/);
    assert.match(styles, /\.wb-library-card\[data-whiteboard-preset-category="template"\]/);
    assert.doesNotMatch(`${libraryPanel}\n${library}\n${libraryTemplates}`, /from\s+["']react["']|from\s+["']react\/|useState|useEffect|^["']use client["']/m);
    assert.match(connectorInfo, /data-whiteboard-connector-info="model-backed"/);
    assert.match(connectorInfo, /data-whiteboard-connector-rerouting="scene-reducer-live"/);
    assert.match(connectorInfo, /connectorRouteForElement/);
    assert.match(connectorInfo, /connectorRouteMetadata/);
    assert.match(connectorInfo, /data-whiteboard-connector-route-controls="command-backed"/);
    assert.match(connectorInfo, /data-whiteboard-connector-route-storage="metadata\.connectorRoute"/);
    assert.match(connectorInfo, /data-whiteboard-command="element\.update"/);
    assert.match(connectorInfo, /data-whiteboard-connector-route=\{mode\}/);
    assert.match(connectorInfo, /data-whiteboard-connector-route-option=\{mode\}/);
    assert.match(connectorInfo, /connectorRouteMetadata\(route\)/);
    assert.match(connectorInfo, /data-whiteboard-start-binding/);
    assert.match(connectorInfo, /data-whiteboard-end-binding/);
    assert.match(connectorInfo, /data-whiteboard-start-anchor/);
    assert.match(connectorInfo, /data-whiteboard-end-anchor/);
    assert.match(connectorInfo, /data-whiteboard-start-bound/);
    assert.match(connectorInfo, /data-whiteboard-end-bound/);
    assert.match(textControls, /data-whiteboard-command="text\.commit"/);
    assert.match(textControls, /whiteboardActions\.commitText/);
    assert.match(textOverlay, /data-whiteboard-command="text\.commit"/);
    assert.match(textOverlay, /onBlur/);
    assert.match(textOverlay, /onKeyDown/);
    assert.doesNotMatch(textOverlay, /whiteboard\.commitText/);
    assert.match(canvasStage, /onCommitText/);
    assert.match(app, /whiteboardActions\.commitText/);
    assert.match(styles, /\.wb-grouping-controls/);
    assert.match(styles, /\.wb-connector-info/);
    assert.match(styles, /\.whiteboard-text-editor:focus-visible/);
  });
});
