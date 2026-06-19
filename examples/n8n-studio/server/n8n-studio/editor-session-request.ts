import type {
  EditorSessionRequestKind,
  EditorSessionRequestPlan,
} from "../../lib/n8n-studio/types";
import {
  createStudioBootFromLocalGeneratedSource,
  type GeneratedStudioBoot,
} from "./generated-catalog-source";

const REDACTION_POLICY = "secret-values-never-included" as const;

type RequestKindCounts = Record<EditorSessionRequestKind, number>;

function emptyRequestKindCounts(): RequestKindCounts {
  return {
    "dynamic-node-parameters": 0,
    "resource-locator-search": 0,
    "resource-mapper-schema": 0,
    "credential-list": 0,
    "credential-test": 0,
  };
}

function countRequestKinds(
  requests: EditorSessionRequestPlan[],
): RequestKindCounts {
  const counts = emptyRequestKindCounts();
  for (const request of requests) {
    counts[request.kind] += 1;
  }
  return counts;
}

export function createEditorSessionRequestBatchResponse(
  boot: GeneratedStudioBoot = createStudioBootFromLocalGeneratedSource(),
) {
  const requestPlans = boot.state.editorSession.requestPlans;
  const requests = requestPlans.filter((request) => request.status === "blocked");
  const configuredRequestCount = requestPlans.length - requests.length;

  return {
    schema: "dx.n8n-studio.editor-session.request-batch",
    ok: true,
    status: boot.state.editorSession.status,
    selectedNodeId: boot.state.editorSession.selectedNodeId,
    selectedNodeName: boot.state.editorSession.selectedNodeName,
    nodeType: boot.state.editorSession.nodeType,
    requestCount: requestPlans.length,
    pendingRequestCount: requests.length,
    configuredRequestCount,
    requestKindCounts: countRequestKinds(requests),
    dynamicParameterLoadCount:
      boot.state.editorSession.dynamicParameterLoadCount,
    resourceLocatorSearchCount:
      boot.state.editorSession.resourceLocatorSearchCount,
    resourceMapperRequestCount:
      boot.state.editorSession.resourceMapperRequestCount,
    credentialRequestCount: boot.state.editorSession.credentialRequestCount,
    credentialValidationRequestCount:
      boot.state.editorSession.credentialValidationRequestCount,
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    redaction: REDACTION_POLICY,
    issue: boot.state.editorSession.issue,
    requests,
  };
}

export function createEditorSessionRequestBatchResponseFromLocalGeneratedSource(
  startDirectory = process.cwd(),
) {
  return createEditorSessionRequestBatchResponse(
    createStudioBootFromLocalGeneratedSource(startDirectory),
  );
}
