import {
  canRedoWhiteboard,
  canUndoWhiteboard,
  createWhiteboardHistory,
  pushWhiteboardCommandBatch,
  pushWhiteboardCommand,
  redoWhiteboard,
  undoWhiteboard,
  type WhiteboardHistory,
} from "../whiteboard/history";
import { hitTestScene, type WhiteboardHit, type WhiteboardHitTestOptions } from "../whiteboard/hit-test";
import type { WhiteboardAlignment, WhiteboardDistribution } from "../whiteboard/arrange";
import type { WhiteboardLibraryPresetId } from "../whiteboard/library";
import {
  createWhiteboardDocument,
  type WhiteboardDocument,
  type WhiteboardElement,
  type WhiteboardElementId,
  type WhiteboardElementPatch,
  type WhiteboardGroupId,
  type WhiteboardMetadata,
  type WhiteboardPoint,
  type WhiteboardSelectionMode,
  type WhiteboardStyle,
  type WhiteboardTool,
  type WhiteboardViewport,
} from "../whiteboard/model";
import type { WhiteboardImportedImageElement } from "../whiteboard/image-import";
import type { WhiteboardCommand } from "../whiteboard/commands";
import { createDemoWhiteboardDocument } from "../whiteboard/demo-document";
import type { WhiteboardGridSettingsPatch } from "../whiteboard/grid";
import type { WhiteboardReorderIntent } from "../whiteboard/scene";

export type DxStoreDefinition<TStore extends Record<string, unknown>> = TStore;
export type DxStoreAction<TArgs extends readonly unknown[] = readonly [], TResult = void> = (
  ...args: TArgs
) => TResult;

export interface WhiteboardStoreState {
  readonly history: WhiteboardHistory;
  readonly document: WhiteboardDocument;
  readonly canUndo: boolean;
  readonly canRedo: boolean;
}

export type WhiteboardStoreListener = (
  state: WhiteboardStoreState,
  previousState: WhiteboardStoreState,
) => void;

export interface CreateWhiteboardStoreOptions {
  readonly document?: WhiteboardDocument;
  readonly history?: WhiteboardHistory;
  readonly historyLimit?: number;
}

export interface WhiteboardStoreActions {
  readonly setTool: (tool: WhiteboardTool) => WhiteboardDocument;
  readonly setViewport: (viewport: Partial<WhiteboardViewport>) => WhiteboardDocument;
  readonly panViewport: (delta: WhiteboardPoint) => WhiteboardDocument;
  readonly zoomViewport: (zoom: number, anchor?: WhiteboardPoint) => WhiteboardDocument;
  readonly setMetadata: (metadata: WhiteboardMetadata) => WhiteboardDocument;
  readonly setGridSettings: (settings: WhiteboardGridSettingsPatch) => WhiteboardDocument;
  readonly setStyle: (style: Partial<WhiteboardStyle>) => WhiteboardDocument;
  readonly selectElements: (
    ids: readonly WhiteboardElementId[],
    mode?: WhiteboardSelectionMode,
  ) => WhiteboardDocument;
  readonly clearSelection: () => WhiteboardDocument;
  readonly groupSelection: (name?: string) => WhiteboardDocument;
  readonly ungroupSelection: () => WhiteboardDocument;
  readonly addElement: (
    element: WhiteboardElement,
    options?: { readonly index?: number; readonly select?: boolean },
  ) => WhiteboardDocument;
  readonly importImage: (
    element: WhiteboardImportedImageElement,
    options?: { readonly index?: number },
  ) => WhiteboardDocument;
  readonly updateElement: (
    id: WhiteboardElementId,
    patch: WhiteboardElementPatch,
  ) => WhiteboardDocument;
  readonly commitText: (id: WhiteboardElementId, text: string) => WhiteboardDocument;
  readonly removeGroups: (groupIds: readonly WhiteboardGroupId[]) => WhiteboardDocument;
  readonly removeElements: (ids: readonly WhiteboardElementId[]) => WhiteboardDocument;
  readonly removeSelection: () => WhiteboardDocument;
  readonly duplicateSelection: () => WhiteboardDocument;
  readonly setSelectionLocked: (locked: boolean) => WhiteboardDocument;
  readonly setSelectionHidden: (hidden: boolean) => WhiteboardDocument;
  readonly translateElements: (
    ids: readonly WhiteboardElementId[],
    delta: WhiteboardPoint,
  ) => WhiteboardDocument;
  readonly reorderElements: (
    ids: readonly WhiteboardElementId[],
    intent: WhiteboardReorderIntent,
  ) => WhiteboardDocument;
  readonly alignSelection: (alignment: WhiteboardAlignment) => WhiteboardDocument;
  readonly distributeSelection: (distribution: WhiteboardDistribution) => WhiteboardDocument;
  readonly insertLibraryPreset: (
    preset: WhiteboardLibraryPresetId,
    origin?: WhiteboardPoint,
  ) => WhiteboardDocument;
  readonly hitTest: (point: WhiteboardPoint, options?: WhiteboardHitTestOptions) => WhiteboardHit | null;
}

export interface WhiteboardStoreApi {
  readonly actions: WhiteboardStoreActions;
  readonly getState: () => WhiteboardStoreState;
  readonly getDocument: () => WhiteboardDocument;
  readonly dispatch: (command: WhiteboardCommand) => WhiteboardDocument;
  readonly dispatchBatch: (commands: readonly WhiteboardCommand[]) => WhiteboardDocument;
  readonly undo: () => WhiteboardDocument;
  readonly redo: () => WhiteboardDocument;
  readonly reset: (document?: WhiteboardDocument) => WhiteboardDocument;
  readonly subscribe: (listener: WhiteboardStoreListener) => () => void;
}

export function store<TStore extends Record<string, unknown>>(
  definition: TStore,
): DxStoreDefinition<TStore> {
  return definition;
}

export function action<TArgs extends readonly unknown[], TResult>(
  handler: (...args: TArgs) => TResult,
): DxStoreAction<TArgs, TResult> {
  return handler;
}

export function createWhiteboardStore(
  options: CreateWhiteboardStoreOptions = {},
): WhiteboardStoreApi {
  const listeners = new Set<WhiteboardStoreListener>();
  const historyLimit = options.historyLimit ?? options.history?.limit ?? 100;
  let state = createStoreState(
    options.history ??
      createWhiteboardHistory(options.document ?? createWhiteboardDocument(), {
        limit: historyLimit,
      }),
  );

  let api: WhiteboardStoreApi;

  const setHistory = (history: WhiteboardHistory): WhiteboardDocument => {
    if (history === state.history) return state.document;

    const previousState = state;
    state = createStoreState(history);
    listeners.forEach((listener) => listener(state, previousState));
    return state.document;
  };

  const dispatch = (command: WhiteboardCommand) =>
    setHistory(pushWhiteboardCommand(state.history, command));
  const dispatchBatch = (commands: readonly WhiteboardCommand[]) =>
    setHistory(pushWhiteboardCommandBatch(state.history, commands));

  const actions: WhiteboardStoreActions = {
    setTool: (tool) => api.dispatch({ type: "tool.set", tool }),
    setViewport: (viewport) => api.dispatch({ type: "viewport.set", viewport }),
    panViewport: (delta) => api.dispatch({ type: "viewport.pan", delta }),
    zoomViewport: (zoom, anchor) => api.dispatch({ type: "viewport.zoom", zoom, anchor }),
    setMetadata: (metadata) => api.dispatch({ type: "document.metadata.set", metadata }),
    setGridSettings: (settings) => api.dispatch({ type: "grid.set", settings }),
    setStyle: (style) => api.dispatch({ type: "style.set", style }),
    selectElements: (ids, mode) => api.dispatch({ type: "selection.set", ids, mode }),
    clearSelection: () => api.dispatch({ type: "selection.clear" }),
    groupSelection: (name) =>
      api.dispatch({ type: "group.create", ids: state.document.selection, name }),
    ungroupSelection: () =>
      api.dispatch({ type: "group.remove", elementIds: state.document.selection }),
    addElement: (element, addOptions = {}) =>
      api.dispatch({
        type: "element.add",
        element,
        index: addOptions.index,
        select: addOptions.select,
      }),
    importImage: (element, addOptions = {}) =>
      api.dispatch({
        type: "element.add",
        element,
        index: addOptions.index,
        select: true,
      }),
    updateElement: (id, patch) => api.dispatch({ type: "element.update", id, patch }),
    commitText: (id, text) => api.dispatch({ type: "text.commit", id, text }),
    removeGroups: (groupIds) => api.dispatch({ type: "group.remove", groupIds }),
    removeElements: (ids) => api.dispatch({ type: "element.remove", ids }),
    removeSelection: () => api.dispatch({ type: "element.remove", ids: state.document.selection }),
    duplicateSelection: () => api.dispatch({ type: "element.duplicate", ids: state.document.selection }),
    setSelectionLocked: (locked) =>
      api.dispatch({
        type: locked ? "element.lock" : "element.unlock",
        ids: state.document.selection,
      }),
    setSelectionHidden: (hidden) =>
      api.dispatch({
        type: hidden ? "element.hide" : "element.show",
        ids: state.document.selection,
      }),
    translateElements: (ids, delta) => api.dispatch({ type: "element.translate", ids, delta }),
    reorderElements: (ids, intent) => api.dispatch({ type: "element.reorder", ids, intent }),
    alignSelection: (alignment) =>
      api.dispatch({ type: "element.align", ids: state.document.selection, alignment }),
    distributeSelection: (distribution) =>
      api.dispatch({ type: "element.distribute", ids: state.document.selection, distribution }),
    insertLibraryPreset: (preset, origin) =>
      api.dispatch({ type: "library.insert", preset, origin }),
    hitTest: (point, hitOptions) => hitTestScene(state.document, point, hitOptions),
  };

  api = {
    actions,
    getState: () => state,
    getDocument: () => state.document,
    dispatch,
    dispatchBatch,
    undo: () => setHistory(undoWhiteboard(state.history)),
    redo: () => setHistory(redoWhiteboard(state.history)),
    reset: (document = createWhiteboardDocument()) =>
      setHistory(createWhiteboardHistory(document, { limit: historyLimit })),
    subscribe: (listener) => {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
  };

  return api;
}

export function createWhiteboardActionStore(api: WhiteboardStoreApi = createWhiteboardStore()) {
  return store({
    snapshot: action(() => api.getState()),
    dispatch: action((command: WhiteboardCommand) => api.dispatch(command)),
    dispatchBatch: action((commands: readonly WhiteboardCommand[]) =>
      api.dispatchBatch(commands),
    ),
    undo: action(() => api.undo()),
    redo: action(() => api.redo()),
    reset: action((document?: WhiteboardDocument) => api.reset(document)),
    setTool: action((tool: WhiteboardTool) => api.actions.setTool(tool)),
    setViewport: action((viewport: Partial<WhiteboardViewport>) =>
      api.actions.setViewport(viewport),
    ),
    zoomViewport: action((zoom: number, anchor?: WhiteboardPoint) =>
      api.actions.zoomViewport(zoom, anchor),
    ),
    setMetadata: action((metadata: WhiteboardMetadata) => api.actions.setMetadata(metadata)),
    setGridSettings: action((settings: WhiteboardGridSettingsPatch) =>
      api.actions.setGridSettings(settings),
    ),
    setStyle: action((style: Partial<WhiteboardStyle>) => api.actions.setStyle(style)),
    addElement: action((element: WhiteboardElement) =>
      api.actions.addElement(element, { select: true }),
    ),
    importImage: action((element: WhiteboardImportedImageElement) =>
      api.actions.importImage(element),
    ),
    groupSelection: action((name?: string) => api.actions.groupSelection(name)),
    ungroupSelection: action(() => api.actions.ungroupSelection()),
    duplicateSelection: action(() => api.actions.duplicateSelection()),
    commitText: action((id: WhiteboardElementId, text: string) =>
      api.actions.commitText(id, text),
    ),
    setSelectionLocked: action((locked: boolean) => api.actions.setSelectionLocked(locked)),
    setSelectionHidden: action((hidden: boolean) => api.actions.setSelectionHidden(hidden)),
    alignSelection: action((alignment: WhiteboardAlignment) => api.actions.alignSelection(alignment)),
    distributeSelection: action((distribution: WhiteboardDistribution) =>
      api.actions.distributeSelection(distribution),
    ),
    insertLibraryPreset: action((preset: WhiteboardLibraryPresetId, origin?: WhiteboardPoint) =>
      api.actions.insertLibraryPreset(preset, origin),
    ),
    hitTest: action((point: WhiteboardPoint, options?: WhiteboardHitTestOptions) =>
      api.actions.hitTest(point, options),
    ),
  });
}

export const whiteboardStore = createWhiteboardStore({
  document: createDemoWhiteboardDocument(),
});
export const whiteboardActions = createWhiteboardActionStore(whiteboardStore);

function createStoreState(history: WhiteboardHistory): WhiteboardStoreState {
  return {
    history,
    document: history.present,
    canUndo: canUndoWhiteboard(history),
    canRedo: canRedoWhiteboard(history),
  };
}
