import type {
  EditorSessionRequestKind,
  EditorSessionRequestPlan,
} from "../../lib/n8n-studio/types";
import {
  createStudioBootFromLocalGeneratedSource,
  type GeneratedStudioBoot,
} from "./generated-catalog-source";
import { createEditorSessionRequestBatchResponse } from "./editor-session-request";
import { createEditorSessionResponseBatchResponse } from "./editor-session-response";

const REDACTION_POLICY = "secret-values-never-included" as const;

export type EditorSessionRequestExecutor = (
  request: EditorSessionRequestPlan,
) => Promise<unknown>;

export type EditorSessionExecutionOptions = {
  boot?: GeneratedStudioBoot;
  executeRequest: EditorSessionRequestExecutor;
  maxRequests?: number;
  requestKinds?: EditorSessionRequestKind[];
};

function normalizedMaxRequests(value: number | undefined, fallback: number) {
  if (value === undefined || !Number.isFinite(value)) {
    return fallback;
  }

  return Math.max(0, Math.floor(value));
}

function requestKindAllowed(
  request: EditorSessionRequestPlan,
  requestKinds: EditorSessionRequestKind[] | undefined,
) {
  return !requestKinds?.length || requestKinds.includes(request.kind);
}

export async function runEditorSessionRequestBatch(
  options: EditorSessionExecutionOptions,
) {
  const boot = options.boot ?? createStudioBootFromLocalGeneratedSource();
  const requestBatch = createEditorSessionRequestBatchResponse(boot);
  const executableRequests = requestBatch.requests
    .filter((request) => requestKindAllowed(request, options.requestKinds))
    .slice(
      0,
      normalizedMaxRequests(options.maxRequests, requestBatch.requests.length),
    );
  const responses: unknown[] = [];

  for (const request of executableRequests) {
    const response = await options.executeRequest(request);
    if (response) {
      responses.push(response);
    }
  }

  const responseBatch = createEditorSessionResponseBatchResponse(
    { responses },
    boot,
  );

  return {
    schema: "dx.n8n-studio.editor-session.execution-batch",
    ok: true,
    status: responseBatch.status,
    selectedNodeId: requestBatch.selectedNodeId,
    selectedNodeName: requestBatch.selectedNodeName,
    nodeType: requestBatch.nodeType,
    requestCount: requestBatch.requestCount,
    pendingRequestCount: requestBatch.pendingRequestCount,
    executedRequestCount: executableRequests.length,
    acceptedResponseCount: responseBatch.acceptedResponseCount,
    appliedResponseCount: responseBatch.appliedResponseCount,
    rejectedResponseCount: responseBatch.rejectedResponseCount,
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    redaction: REDACTION_POLICY,
    editorSession: responseBatch.editorSession,
    readiness: responseBatch.readiness,
    issue:
      "Editor-session requests were executed through a host-owned adapter and applied through the governed no-secret response pipeline.",
  };
}
