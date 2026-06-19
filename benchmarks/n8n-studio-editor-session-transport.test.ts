import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { pathToFileURL } from "node:url";

import { createN8nStudioStore } from "../examples/n8n-studio/lib/stores/n8n-studio-store";
import { createStudioBootFromLocalGeneratedSource } from "../examples/n8n-studio/server/n8n-studio/generated-catalog-source";
import type {
  EditorSessionCredentialListTransportResponse,
  EditorSessionCredentialTestTransportResponse,
  EditorSessionRequestPlan,
} from "../examples/n8n-studio/lib/n8n-studio/types";

const repoRoot = path.resolve(import.meta.dirname, "..");

type CredentialRequestPlan = EditorSessionRequestPlan & {
  credentialType: string;
  credentialKey: string;
};

type CredentialValidationRequestPlan = EditorSessionRequestPlan & {
  credentialType: string;
  credentialKey: string;
};

type DynamicOptionRequestPlan = EditorSessionRequestPlan & {
  fieldName: string;
  loadMethod: string;
};

function credentialRequestPlan(
  plans: EditorSessionRequestPlan[],
): CredentialRequestPlan {
  const credentialPlans = plans.filter(
    (candidate): candidate is CredentialRequestPlan =>
      candidate.kind === "credential-list" &&
      typeof candidate.credentialType === "string" &&
      typeof candidate.credentialKey === "string",
  );
  const plan =
    credentialPlans.find((candidate) =>
      plans.some(
        (validationPlan) =>
          validationPlan.kind === "credential-test" &&
          validationPlan.credentialType === candidate.credentialType &&
          validationPlan.credentialKey === candidate.credentialKey,
      ),
    ) ?? credentialPlans[0];

  assert.ok(plan, "expected selected node to expose a credential-list plan");
  return plan;
}

function credentialValidationRequestPlan(
  plans: EditorSessionRequestPlan[],
  credentialPlan: CredentialRequestPlan,
): CredentialValidationRequestPlan {
  const plan = plans.find(
    (candidate): candidate is CredentialValidationRequestPlan =>
      candidate.kind === "credential-test" &&
      candidate.credentialType === credentialPlan.credentialType &&
      candidate.credentialKey === credentialPlan.credentialKey,
  );

  assert.ok(
    plan,
    "expected selected credential to expose a credential-test plan",
  );
  return plan;
}

function credentialListResponseForPlan(
  plan: CredentialRequestPlan,
): EditorSessionCredentialListTransportResponse {
  return {
    kind: "credential-list",
    nodeId: plan.nodeId,
    nodeType: plan.nodeType,
    credentialType: plan.credentialType,
    credentialKey: plan.credentialKey,
    selectedCredentialId: "credential-from-editor-session",
    selectedCredentialName: "Editor Session Credential",
    credentialOptions: [
      {
        id: "credential-from-editor-session",
        name: "Editor Session Credential",
        credentialType: plan.credentialType,
        source: "editor-session-placeholder",
        redaction: "secret-values-never-included",
      },
    ],
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    redaction: "secret-values-never-included",
  };
}

function credentialTestResponseForPlan(
  plan: CredentialValidationRequestPlan,
): EditorSessionCredentialTestTransportResponse {
  return {
    kind: "credential-test",
    nodeId: plan.nodeId,
    nodeType: plan.nodeType,
    credentialType: plan.credentialType,
    credentialKey: plan.credentialKey,
    selectedCredentialId: plan.selectedCredentialId,
    validationStatus: "valid",
    validatedAt: "2026-06-09T00:00:00.000Z",
    message: "Credential verified by the governed editor-session bridge.",
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    redaction: "secret-values-never-included",
  };
}

function dynamicOptionRequestPlan(
  plans: EditorSessionRequestPlan[],
): DynamicOptionRequestPlan {
  const plan = plans.find(
    (candidate): candidate is DynamicOptionRequestPlan =>
      candidate.kind === "dynamic-node-parameters" &&
      typeof candidate.fieldName === "string" &&
      typeof candidate.loadMethod === "string",
  );

  assert.ok(
    plan,
    "expected selected node to expose a dynamic-node-parameters plan",
  );
  return plan;
}

test("n8n Studio store applies governed editor-session response batches", () => {
  const boot = createStudioBootFromLocalGeneratedSource(repoRoot);
  const studioStore = createN8nStudioStore({
    initialState: boot.state,
    nodeTypeRegistry: boot.nodeTypeRegistry,
  });
  const plan = credentialRequestPlan(studioStore.state.editorSession.requestPlans);

  assert.equal(typeof studioStore.applyEditorSessionAction, "function");

  studioStore.applyEditorSessionAction(studioStore, {
    kind: "applyTransportResponses",
    responses: [credentialListResponseForPlan(plan)],
  });

  const configuredPlan = studioStore.state.editorSession.requestPlans.find(
    (candidate) =>
      candidate.kind === "credential-list" &&
      candidate.credentialType === plan.credentialType,
  );
  const serialized = JSON.stringify(studioStore.state.editorSession);

  assert.equal(studioStore.state.editorSession.fulfilledRequestCount, 1);
  assert.equal(configuredPlan?.status, "configured");
  assert.equal(
    configuredPlan?.selectedCredentialId,
    "credential-from-editor-session",
  );
  assert.equal(configuredPlan?.resolvedCredentialOptionCount, 1);
  assert.equal(serialized.includes("must-not-survive"), false);
});

test("n8n Studio store overlays governed dynamic options into visible parameter state", () => {
  const boot = createStudioBootFromLocalGeneratedSource(repoRoot);
  const studioStore = createN8nStudioStore({
    initialState: boot.state,
    nodeTypeRegistry: boot.nodeTypeRegistry,
  });
  const dynamicNode = studioStore.state.document.nodes.find((node) =>
    node.type.includes("openAi"),
  );

  assert.ok(dynamicNode, "expected boot workflow to include a dynamic OpenAI node");
  studioStore.applyCanvasInteraction(studioStore, {
    kind: "selectNode",
    nodeId: dynamicNode.id,
  });

  const plan = dynamicOptionRequestPlan(
    studioStore.state.editorSession.requestPlans,
  );

  studioStore.applyEditorSessionAction(studioStore, {
    kind: "applyTransportResponses",
    responses: [
      {
        kind: "dynamic-node-parameters",
        nodeId: plan.nodeId,
        nodeType: plan.nodeType,
        fieldName: plan.fieldName,
        loadMethod: plan.loadMethod,
        dynamicLoadBoundary: plan.dynamicLoadBoundary,
        options: [
          {
            name: "Governed option",
            value: "governed-option",
          },
        ],
        providerBoundary: true,
        liveProviderExecution: false,
        secretsIncluded: false,
        redaction: "secret-values-never-included",
      },
    ],
  });

  const field = studioStore.state.parameters.find(
    (candidate) => candidate.name === plan.fieldName,
  );
  const configuredPlan = studioStore.state.editorSession.requestPlans.find(
    (candidate) =>
      candidate.kind === "dynamic-node-parameters" &&
      candidate.fieldName === plan.fieldName,
  );

  assert.equal(configuredPlan?.status, "configured");
  assert.equal(field?.options?.some((option) => option.value === "governed-option"), true);
  assert.equal(JSON.stringify(studioStore.state.parameters).includes("must-not-survive"), false);
});

test("n8n Studio editor-session request endpoint exposes governed pending request batches", async () => {
  const helperPath = path.join(
    repoRoot,
    "examples",
    "n8n-studio",
    "server",
    "n8n-studio",
    "editor-session-request.ts",
  );
  const routePath = path.join(
    repoRoot,
    "examples",
    "n8n-studio",
    "app",
    "api",
    "n8n-studio",
    "editor-session",
    "route.ts",
  );

  assert.equal(fs.existsSync(helperPath), true);
  assert.equal(fs.existsSync(routePath), true);

  const boot = createStudioBootFromLocalGeneratedSource(repoRoot);
  const helper = await import(pathToFileURL(helperPath).href);
  const route = await import(pathToFileURL(routePath).href);
  const createRequestBatch =
    helper.createEditorSessionRequestBatchResponse as (
      bootContext: typeof boot,
    ) => {
      schema: string;
      ok: boolean;
      requestCount: number;
      pendingRequestCount: number;
      configuredRequestCount: number;
      providerBoundary: true;
      liveProviderExecution: false;
      secretsIncluded: false;
      redaction: "secret-values-never-included";
      requests: EditorSessionRequestPlan[];
    };

  assert.equal(typeof createRequestBatch, "function");
  assert.equal(typeof route.GET, "function");

  const response = createRequestBatch(boot);
  const serialized = JSON.stringify(response);

  assert.equal(response.schema, "dx.n8n-studio.editor-session.request-batch");
  assert.equal(response.ok, true);
  assert.equal(response.requestCount, boot.state.editorSession.requestPlans.length);
  assert.equal(response.pendingRequestCount > 0, true);
  assert.equal(response.configuredRequestCount, 0);
  assert.equal(response.requests.length, response.pendingRequestCount);
  assert.equal(response.providerBoundary, true);
  assert.equal(response.liveProviderExecution, false);
  assert.equal(response.secretsIncluded, false);
  assert.equal(response.redaction, "secret-values-never-included");
  assert.equal(
    response.requests.every(
      (request) =>
        request.status === "blocked" &&
        request.providerBoundary === true &&
        request.liveProviderExecution === false &&
        request.secretsIncluded === false &&
        request.redaction === "secret-values-never-included",
    ),
    true,
  );
  assert.equal(
    response.requests.some((request) => request.kind === "credential-test"),
    true,
  );
  assert.equal(serialized.includes("apiKey"), false);
  assert.equal(serialized.includes("must-not-survive"), false);

  const httpResponse = await route.GET(
    new Request("http://localhost/api/n8n-studio/editor-session"),
  );
  const httpBody = await httpResponse.json();

  assert.equal(httpBody.schema, "dx.n8n-studio.editor-session.request-batch");
  assert.equal(httpBody.pendingRequestCount, response.pendingRequestCount);
  assert.equal(httpBody.requests.length, response.requests.length);
});

test("n8n Studio editor-session response endpoint applies only governed batches", async () => {
  const helperPath = path.join(
    repoRoot,
    "examples",
    "n8n-studio",
    "server",
    "n8n-studio",
    "editor-session-response.ts",
  );
  const routePath = path.join(
    repoRoot,
    "examples",
    "n8n-studio",
    "app",
    "api",
    "n8n-studio",
    "editor-session",
    "route.ts",
  );

  assert.equal(fs.existsSync(helperPath), true);
  assert.equal(fs.existsSync(routePath), true);

  const boot = createStudioBootFromLocalGeneratedSource(repoRoot);
  const plan = credentialRequestPlan(boot.state.editorSession.requestPlans);
  const validationPlan = credentialValidationRequestPlan(
    boot.state.editorSession.requestPlans,
    plan,
  );
  const helper = await import(pathToFileURL(helperPath).href);
  const route = await import(pathToFileURL(routePath).href);
  const createResponse =
    helper.createEditorSessionResponseBatchResponse as (
      payload: unknown,
      bootContext: typeof boot,
    ) => {
      schema: string;
      appliedResponseCount: number;
      rejectedResponseCount: number;
      providerBoundary: true;
      liveProviderExecution: false;
      secretsIncluded: false;
      readiness: { editorSession: { fulfilledRequestCount: number } };
      editorSession: { requestPlans: EditorSessionRequestPlan[] };
    };

  assert.equal(typeof createResponse, "function");
  assert.equal(typeof route.POST, "function");

  const response = createResponse(
    {
      responses: [
        credentialListResponseForPlan(plan),
        credentialTestResponseForPlan(validationPlan),
        {
          ...credentialListResponseForPlan(plan),
          selectedCredentialId: "unsafe-credential",
          secretsIncluded: true,
          apiKey: "must-not-survive",
        },
        {
          ...credentialTestResponseForPlan(validationPlan),
          secretsIncluded: true,
          apiKey: "must-not-survive",
        },
      ],
    },
    boot,
  );
  const routeSource = fs.readFileSync(routePath, "utf8");
  const configuredPlan = response.editorSession.requestPlans.find(
    (candidate) =>
      candidate.kind === "credential-list" &&
      candidate.credentialType === plan.credentialType,
  );
  const configuredValidationPlan = response.editorSession.requestPlans.find(
    (candidate) =>
      candidate.kind === "credential-test" &&
      candidate.credentialType === plan.credentialType,
  );
  const serialized = JSON.stringify(response);

  assert.equal(response.schema, "dx.n8n-studio.editor-session.response-batch");
  assert.equal(response.appliedResponseCount, 2);
  assert.equal(response.rejectedResponseCount, 2);
  assert.equal(response.providerBoundary, true);
  assert.equal(response.liveProviderExecution, false);
  assert.equal(response.secretsIncluded, false);
  assert.equal(response.readiness.editorSession.fulfilledRequestCount, 2);
  assert.equal(configuredPlan?.status, "configured");
  assert.equal(configuredValidationPlan?.status, "configured");
  assert.equal(
    configuredValidationPlan?.credentialValidationStatus,
    "valid",
  );
  assert.equal(serialized.includes("unsafe-credential"), false);
  assert.equal(serialized.includes("apiKey"), false);
  assert.equal(serialized.includes("must-not-survive"), false);

  const httpResponse = await route.POST(
    new Request("http://localhost/api/n8n-studio/editor-session", {
      method: "POST",
      body: JSON.stringify({
        responses: [
          credentialListResponseForPlan(plan),
          credentialTestResponseForPlan(validationPlan),
        ],
      }),
    }),
  );
  const httpBody = await httpResponse.json();

  assert.equal(httpBody.appliedResponseCount, 2);
  assert.equal(httpBody.readiness.editorSession.fulfilledRequestCount, 2);
  assert.match(routeSource, /export async function POST/);
  assert.match(
    routeSource,
    /createEditorSessionResponseBatchResponseFromLocalGeneratedSource/,
  );
});

test("n8n Studio editor-session execution helper applies host-executed request responses", async () => {
  const helperPath = path.join(
    repoRoot,
    "examples",
    "n8n-studio",
    "server",
    "n8n-studio",
    "editor-session-execution.ts",
  );
  const baseBoot = createStudioBootFromLocalGeneratedSource(repoRoot);
  const studioStore = createN8nStudioStore({
    initialState: baseBoot.state,
    nodeTypeRegistry: baseBoot.nodeTypeRegistry,
  });
  const dynamicNode = studioStore.state.document.nodes.find((node) =>
    node.type.includes("openAi"),
  );

  assert.ok(dynamicNode, "expected boot workflow to include a dynamic OpenAI node");
  studioStore.applyCanvasInteraction(studioStore, {
    kind: "selectNode",
    nodeId: dynamicNode.id,
  });

  const boot = {
    ...baseBoot,
    state: studioStore.state,
  };
  const plan = dynamicOptionRequestPlan(boot.state.editorSession.requestPlans);
  const executedRequests: EditorSessionRequestPlan[] = [];

  assert.equal(fs.existsSync(helperPath), true);

  const helper = await import(pathToFileURL(helperPath).href);

  assert.equal(typeof helper.runEditorSessionRequestBatch, "function");

  const response = await helper.runEditorSessionRequestBatch({
    boot,
    maxRequests: 1,
    executeRequest: async (request: EditorSessionRequestPlan) => {
      executedRequests.push(request);

      return {
        kind: "dynamic-node-parameters",
        nodeId: request.nodeId,
        nodeType: request.nodeType,
        fieldName: request.fieldName,
        loadMethod: request.loadMethod,
        dynamicLoadBoundary: request.dynamicLoadBoundary,
        options: [
          {
            name: "Executor option",
            value: "executor-option",
            description: "Returned by the governed host executor.",
          },
          {
            name: "Unsafe extra",
            value: "unsafe-extra",
            apiKey: "must-not-survive",
          },
        ],
        providerBoundary: true,
        liveProviderExecution: false,
        secretsIncluded: false,
        redaction: "secret-values-never-included",
      };
    },
  });
  const configuredPlan = response.editorSession.requestPlans.find(
    (candidate: EditorSessionRequestPlan) =>
      candidate.kind === "dynamic-node-parameters" &&
      candidate.dynamicLoadBoundary === plan.dynamicLoadBoundary,
  );
  const serialized = JSON.stringify(response);

  assert.equal(response.schema, "dx.n8n-studio.editor-session.execution-batch");
  assert.equal(response.executedRequestCount, 1);
  assert.equal(response.acceptedResponseCount, 1);
  assert.equal(response.appliedResponseCount, 1);
  assert.equal(response.rejectedResponseCount, 0);
  assert.equal(configuredPlan?.status, "configured");
  assert.equal(configuredPlan?.resolvedOptionCount, 2);
  assert.equal(configuredPlan?.resolvedOptions?.[0]?.value, "executor-option");
  assert.equal(executedRequests.length, 1);
  assert.equal(JSON.stringify(executedRequests).includes("must-not-survive"), false);
  assert.equal(serialized.includes("apiKey"), false);
  assert.equal(serialized.includes("must-not-survive"), false);
});
