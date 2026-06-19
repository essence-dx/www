import assert from "node:assert/strict";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { pathToFileURL } from "node:url";

import {
  createCredentialVaultBridge,
  type CredentialSecretRequest,
} from "../examples/n8n-studio/lib/n8n-studio/credential-vault";
import type { RuntimeTriggerReceipt } from "../examples/n8n-studio/lib/n8n-studio/runtime-trigger";
import type { WorkflowDocument } from "../examples/n8n-studio/lib/n8n-studio/types";
import { studioWorkflowDocument } from "../examples/n8n-studio/lib/n8n-studio/workflow-document";

const repoRoot = path.resolve(import.meta.dirname, "..");
const schedulerActionRoutePath = path.join(
  repoRoot,
  "examples",
  "n8n-studio",
  "app",
  "api",
  "n8n-studio",
  "runtime-execution-proof-retry",
  "route.ts",
);
const schedulerHelperPath = path.join(
  repoRoot,
  "examples",
  "n8n-studio",
  "server",
  "n8n-studio",
  "runtime-execution-proof-scheduler.ts",
);
const runtimeExecutionProofReceiptPath =
  ".dx/receipts/n8n-studio/runtime/execution-proof-latest.sr";

function webhookWorkflowDocument(): WorkflowDocument {
  return {
    ...studioWorkflowDocument,
    id: "webhook-runtime-workflow",
    name: "Webhook Runtime Workflow",
    nodes: [
      {
        id: "node-webhook",
        name: "Inbound Webhook",
        type: "n8n-nodes-base.webhook",
        typeVersion: 2.1,
        position: { x: 80, y: 200 },
        parameters: {
          httpMethod: "POST",
          path: "dx-runtime-trigger",
          authentication: "headerAuth",
          responseMode: "onReceived",
        },
        credentials: {
          httpHeaderAuth: {
            id: "credential-id-only",
            name: "Webhook header",
          },
        },
      },
      studioWorkflowDocument.nodes[1],
    ],
    connections: [
      {
        id: "edge-webhook-to-http",
        sourceNode: "Inbound Webhook",
        targetNode: "Read Connector Manifest",
        sourceOutput: "main",
        targetInput: "main",
        index: 0,
      },
    ],
    pinData: {},
  };
}

function runtimeTriggerReceipt(document: WorkflowDocument): RuntimeTriggerReceipt {
  return {
    schema: "dx.n8n-studio.runtime-trigger.receipt",
    status: "submitted",
    workflowId: document.id,
    workflowName: document.name,
    triggerNodeId: "node-webhook",
    triggerNodeName: "Inbound Webhook",
    triggerMode: "webhook",
    httpMethod: "POST",
    providerBoundary: true,
    workflowExecutionRequested: true,
    liveProviderExecution: true,
    executionReceiptImported: false,
    executionProofRequired: true,
    secretsIncluded: false,
    sideEffectPolicy: "governed-live-webhook-trigger-request",
    targetOrigin: "https://n8n.local",
    targetUrlStored: false,
    requestBodyStored: false,
    providerResponse: {
      responseReceived: true,
      statusCode: 202,
      bodyStored: false,
      secretsIncluded: false,
    },
    sourcePath: "nodes/Webhook/Webhook.node.ts",
    sourceDescriptionPath: "nodes/Webhook/description.ts",
    receiptPath: ".dx/receipts/n8n-studio/runtime/trigger-latest.sr",
    redaction: "secret-values-never-included",
    issue:
      "Webhook trigger request was submitted through the governed runtime trigger bridge.",
  };
}

function executionHistoryPayload(workflowId: string) {
  return {
    data: [
      {
        id: "exec-webhook-proof-1",
        workflowId,
        mode: "webhook",
        status: "success",
        startedAt: "2026-06-09T12:00:00.000Z",
        stoppedAt: "2026-06-09T12:00:01.000Z",
        data: {
          resultData: {
            runData: {
              "Inbound Webhook": [
                {
                  executionTime: 5,
                  data: {
                    main: [[{ json: { ok: true } }]],
                  },
                },
              ],
              "Read Connector Manifest": [
                {
                  executionTime: 7,
                  data: {
                    main: [[{ json: { received: true } }]],
                  },
                },
              ],
            },
          },
        },
      },
    ],
  };
}

test("n8n Studio scheduler POST accepts id-only retry requests without live execution", async () => {
  const route = await import(pathToFileURL(schedulerActionRoutePath).href);

  assert.equal(typeof route.POST, "function");

  const response = await route.POST(
    new Request("http://localhost/api/n8n-studio/runtime-execution-proof-retry", {
      method: "POST",
      body: JSON.stringify({
        apiCredentialId: "credential-n8n-local",
        triggerReceiptPath: runtimeExecutionProofReceiptPath,
        maxAttempts: 3,
      }),
    }),
  );
  const body = await response.json();
  const serialized = JSON.stringify(body);

  assert.equal(response.status, 409);
  assert.equal(
    body.schema,
    "dx.n8n-studio.runtime-execution-proof.scheduler-response",
  );
  assert.equal(body.status, "credential-handoff-required");
  assert.equal(body.accepted, true);
  assert.deepEqual(body.request, {
    apiCredentialId: "credential-n8n-local",
    triggerReceiptPath: runtimeExecutionProofReceiptPath,
    maxAttempts: 3,
    credentialIdsOnly: true,
    credentialValuesAccepted: false,
    bodyStored: false,
  });
  assert.deepEqual(body.execution, {
    attempted: false,
    reason: "credential-handoff-adapter-required",
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
  });
  assert.equal(body.action.status, "credential-handoff-required");
  assert.equal(body.action.secretsIncluded, false);
  assert.equal(body.nextAction, "connect-dx-zed-credential-handoff-adapter");
  assert.equal(serialized.includes("apiKey"), false);
  assert.equal(serialized.includes("secretRef"), false);
  assert.equal(serialized.includes("must-not-survive"), false);
});

test("n8n Studio scheduler POST rejects secret-bearing retry requests without echoing secrets", async () => {
  const route = await import(pathToFileURL(schedulerActionRoutePath).href);

  assert.equal(typeof route.POST, "function");

  const response = await route.POST(
    new Request("http://localhost/api/n8n-studio/runtime-execution-proof-retry", {
      method: "POST",
      body: JSON.stringify({
        apiCredentialId: "credential-n8n-local",
        triggerReceiptPath: runtimeExecutionProofReceiptPath,
        maxAttempts: 2,
        apiKey: "must-not-survive",
        nested: {
          secretRef: "vault://runtime/n8n/api/key",
        },
      }),
    }),
  );
  const body = await response.json();
  const serialized = JSON.stringify(body);

  assert.equal(response.status, 400);
  assert.equal(
    body.schema,
    "dx.n8n-studio.runtime-execution-proof.scheduler-response",
  );
  assert.equal(body.status, "rejected-secret-bearing-request");
  assert.equal(body.accepted, false);
  assert.equal(body.providerBoundary, true);
  assert.equal(body.liveProviderExecution, false);
  assert.equal(body.secretsIncluded, false);
  assert.equal(body.redaction, "secret-values-never-included");
  assert.equal(serialized.includes("apiKey"), false);
  assert.equal(serialized.includes("secretRef"), false);
  assert.equal(serialized.includes("must-not-survive"), false);
  assert.equal(serialized.includes("vault://"), false);
});

test("n8n Studio scheduler route factory runs retry through host runtime context", async () => {
  const scheduler = await import(pathToFileURL(schedulerHelperPath).href);
  const receiptRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-n8n-runtime-scheduler-route-"),
  );
  const document = webhookWorkflowDocument();
  const trigger = runtimeTriggerReceipt(document);
  const secretRequests: CredentialSecretRequest[] = [];
  const apiRequests: Array<{
    method: string;
    path: string;
    query: Record<string, string | number | boolean>;
    apiKey?: string;
  }> = [];
  const credentialVault = createCredentialVaultBridge({
    records: [
      {
        credentialId: "credential-n8n-local",
        displayName: "Local n8n API",
        credentialType: "n8nApi",
        baseUrl: "https://n8n.local/api/v1/",
        secretRef: "vault://runtime/n8n/api/key",
      },
    ],
    loadSecret: async (request) => {
      secretRequests.push(request);
      return "must-not-survive";
    },
  });

  assert.equal(
    typeof scheduler.createRuntimeExecutionProofSchedulerRoute,
    "function",
  );

  const route = scheduler.createRuntimeExecutionProofSchedulerRoute({
    resolveRuntimeContext: () => ({
      credentialVault,
      document,
      trigger,
      receiptRoot,
      apiTransport: async (request) => {
        apiRequests.push({
          method: request.method,
          path: request.path,
          query: request.query,
          apiKey: request.headers["X-N8N-API-KEY"],
        });
        return executionHistoryPayload(document.id);
      },
    }),
  });

  const response = await route.POST(
    new Request("http://localhost/api/n8n-studio/runtime-execution-proof-retry", {
      method: "POST",
      body: JSON.stringify({
        apiCredentialId: "credential-n8n-local",
        triggerReceiptPath: runtimeExecutionProofReceiptPath,
        maxAttempts: 2,
      }),
    }),
  );
  const body = await response.json();
  const serialized = JSON.stringify(body);
  const writtenProof = JSON.parse(
    fs.readFileSync(body.receiptExport.receiptPath, "utf8"),
  );

  assert.equal(response.status, 200);
  assert.equal(
    body.schema,
    "dx.n8n-studio.runtime-execution-proof.retry-result",
  );
  assert.equal(body.status, "proved");
  assert.equal(body.attemptCount, 1);
  assert.equal(body.providerBoundary, true);
  assert.equal(body.liveProviderExecution, false);
  assert.equal(body.secretsIncluded, false);
  assert.equal(writtenProof.status, "proved");
  assert.deepEqual(secretRequests, [
    {
      credentialId: "credential-n8n-local",
      credentialType: "n8nApi",
      secretRef: "vault://runtime/n8n/api/key",
    },
  ]);
  assert.deepEqual(apiRequests, [
    {
      method: "GET",
      path: "/executions",
      query: {
        workflowId: document.id,
        includeData: true,
        limit: 100,
      },
      apiKey: "must-not-survive",
    },
  ]);
  assert.equal(serialized.includes("must-not-survive"), false);
  assert.equal(serialized.includes("vault://"), false);
  assert.equal(serialized.includes("apiKey"), false);
});

test("n8n Studio scheduler handoff runs retry through injected credential and API adapters", async () => {
  const scheduler = await import(pathToFileURL(schedulerHelperPath).href);
  const receiptRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-n8n-runtime-scheduler-"),
  );
  const document = webhookWorkflowDocument();
  const trigger = runtimeTriggerReceipt(document);
  const secretRequests: CredentialSecretRequest[] = [];
  const apiRequests: Array<{
    method: string;
    path: string;
    query: Record<string, string | number | boolean>;
    apiKey?: string;
  }> = [];
  const credentialVault = createCredentialVaultBridge({
    records: [
      {
        credentialId: "credential-n8n-local",
        displayName: "Local n8n API",
        credentialType: "n8nApi",
        baseUrl: "https://n8n.local/api/v1/",
        secretRef: "vault://runtime/n8n/api/key",
      },
    ],
    loadSecret: async (request) => {
      secretRequests.push(request);
      return "must-not-survive";
    },
  });
  const acceptedRequest = scheduler.createRuntimeExecutionProofSchedulerResponse({
    apiCredentialId: "credential-n8n-local",
    triggerReceiptPath: runtimeExecutionProofReceiptPath,
    maxAttempts: 2,
  }).body.request;

  assert.equal(
    typeof scheduler.runRuntimeExecutionProofSchedulerHandoff,
    "function",
  );

  const result = await scheduler.runRuntimeExecutionProofSchedulerHandoff({
    request: acceptedRequest,
    credentialVault,
    document,
    trigger,
    receiptRoot,
    apiTransport: async (request) => {
      apiRequests.push({
        method: request.method,
        path: request.path,
        query: request.query,
        apiKey: request.headers["X-N8N-API-KEY"],
      });
      return executionHistoryPayload(document.id);
    },
  });
  const serialized = JSON.stringify(result);
  const writtenProof = JSON.parse(
    fs.readFileSync(result.receiptExport.receiptPath, "utf8"),
  );

  assert.deepEqual(secretRequests, [
    {
      credentialId: "credential-n8n-local",
      credentialType: "n8nApi",
      secretRef: "vault://runtime/n8n/api/key",
    },
  ]);
  assert.deepEqual(apiRequests, [
    {
      method: "GET",
      path: "/executions",
      query: {
        workflowId: document.id,
        includeData: true,
        limit: 100,
      },
      apiKey: "must-not-survive",
    },
  ]);
  assert.equal(
    result.schema,
    "dx.n8n-studio.runtime-execution-proof.retry-result",
  );
  assert.equal(result.status, "proved");
  assert.equal(result.attemptCount, 1);
  assert.equal(result.providerBoundary, true);
  assert.equal(result.liveProviderExecution, false);
  assert.equal(result.secretsIncluded, false);
  assert.equal(result.receiptExport.status, "proved");
  assert.equal(writtenProof.status, "proved");
  assert.equal(serialized.includes("must-not-survive"), false);
  assert.equal(serialized.includes("vault://"), false);
  assert.equal(serialized.includes("apiKey"), false);
});
