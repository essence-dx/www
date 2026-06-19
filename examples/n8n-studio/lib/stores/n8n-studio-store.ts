import { createN8nStudioState } from "../n8n-studio/studio-state";
import {
  applyCanvasInteractionToStudioState,
  type CanvasInteractionAction,
} from "../n8n-studio/canvas-interactions";
import {
  applyParameterMutationToStudioState,
  type ParameterMutation,
} from "../n8n-studio/parameter-mutations";
import {
  applyImportExportActionToStudioState,
  type ImportExportAction,
} from "../n8n-studio/import-export-actions";
import {
  applyEditorSessionActionToStudioState,
  type EditorSessionAction,
} from "../n8n-studio/editor-session-actions";
import {
  applyNodeCreatorActionToStudioState,
  type NodeCreatorAction,
} from "../n8n-studio/node-creator-actions";
import { n8nNodeTypeRegistry } from "../n8n-studio/node-type-registry";
import type { NodeTypeDescription } from "../n8n-studio/node-types/types";
import type { N8nStudioState } from "../n8n-studio/types";

export type DxStoreDefinition<TStore extends Record<string, unknown>> = TStore;
export type DxStoreAction<
  TStore,
  TArgs extends unknown[] = [],
  TResult = void,
> = (store: TStore, ...args: TArgs) => TResult;
export type DxStoreDerived<TStore, TResult> = (store: TStore) => TResult;

export function store<TStore extends Record<string, unknown>>(
  definition: TStore,
): DxStoreDefinition<TStore> {
  return definition;
}

export function state<TValue>(initial: TValue): TValue {
  return initial;
}

export function derived<TStore, TResult>(
  compute: DxStoreDerived<TStore, TResult>,
): TResult {
  return compute as unknown as TResult;
}

export function action<TStore, TArgs extends unknown[] = [], TResult = void>(
  handler: DxStoreAction<TStore, TArgs, TResult>,
): DxStoreAction<TStore, TArgs, TResult> {
  return handler;
}

type N8nStudioStoreDraft = {
  state: N8nStudioState;
  nodeTypeRegistry: Record<string, NodeTypeDescription>;
};

export type N8nStudioStoreOptions = {
  initialState?: N8nStudioState;
  nodeTypeRegistry?: Record<string, NodeTypeDescription>;
};

export function createN8nStudioStore(options: N8nStudioStoreOptions = {}) {
  const nodeTypeRegistry = options.nodeTypeRegistry ?? n8nNodeTypeRegistry;

  return store({
    state: state(
      options.initialState ??
        createN8nStudioState({
          nodeTypeRegistry,
        }),
    ),
    nodeTypeRegistry,
    selectedNode: derived((store: N8nStudioStoreDraft) =>
      store.state.document.nodes.find(
        (node) => node.id === store.state.canvas.selectedNodeId,
      ) ?? null,
    ),
    providerConfigured: derived((store: N8nStudioStoreDraft) =>
      store.state.credentials.some((credential) => credential.status === "configured"),
    ),
    markLiveExecutionBlocked: action((store: N8nStudioStoreDraft) => {
      store.state.execution.status = "blocked";
    }),
    applyParameterMutation: action(
      (store: N8nStudioStoreDraft, mutation: ParameterMutation) => {
        store.state = applyParameterMutationToStudioState(
          store.state,
          store.state.canvas.selectedNodeId,
          mutation,
          store.nodeTypeRegistry,
        );
      },
    ),
    applyCanvasInteraction: action(
      (store: N8nStudioStoreDraft, canvasAction: CanvasInteractionAction) => {
        store.state = applyCanvasInteractionToStudioState(store.state, canvasAction);
      },
    ),
    applyNodeCreatorAction: action(
      (store: N8nStudioStoreDraft, nodeCreatorAction: NodeCreatorAction) => {
        store.state = applyNodeCreatorActionToStudioState(
          store.state,
          nodeCreatorAction,
          store.nodeTypeRegistry,
        );
      },
    ),
    applyImportExportAction: action(
      (store: N8nStudioStoreDraft, importExportAction: ImportExportAction) => {
        store.state = applyImportExportActionToStudioState(
          store.state,
          importExportAction,
          store.nodeTypeRegistry,
        );
      },
    ),
    applyEditorSessionAction: action(
      (store: N8nStudioStoreDraft, editorSessionAction: EditorSessionAction) => {
        store.state = applyEditorSessionActionToStudioState(
          store.state,
          editorSessionAction,
        );
      },
    ),
  });
}

export const n8nStudioStore = createN8nStudioStore();
