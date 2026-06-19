import { applyEditorSessionTransportResponses } from "./editor-session-adapter";
import { applyConfiguredEditorSessionPlansToParameters } from "./editor-session-parameter-overlay";
import type { EditorSessionTransportResponse, N8nStudioState } from "./types";

export type EditorSessionAction = {
  kind: "applyTransportResponses";
  responses: EditorSessionTransportResponse[];
};

export function applyEditorSessionActionToStudioState(
  state: N8nStudioState,
  action: EditorSessionAction,
): N8nStudioState {
  switch (action.kind) {
    case "applyTransportResponses": {
      const editorSession = applyEditorSessionTransportResponses(
        state.editorSession,
        action.responses,
      );

      return {
        ...state,
        editorSession,
        parameters: applyConfiguredEditorSessionPlansToParameters(
          state.parameters,
          editorSession.requestPlans,
          editorSession.nodeType,
        ),
      };
    }
  }
}
