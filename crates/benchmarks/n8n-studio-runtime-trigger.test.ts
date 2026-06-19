import assert from "node:assert/strict";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { pathToFileURL } from "node:url";

import {
  createCredentialVaultBridge,
  createCredentialVaultReadiness,
  type CredentialSecretRequest,
} from "../examples/n8n-studio/lib/n8n-studio/credential-vault";
import {
  createExecutionReadiness,
  importExecutionHistoryApiResponse,
} from "../examples/n8n-studio/lib/n8n-studio/executions";
import { createRuntimeExecutionProofReceipt } from "../examples/n8n-studio/lib/n8n-studio/runtime-execution-proof";
import {
  createRuntimeTriggerPlan,
  type RuntimeTriggerRequest,
} from "../examples/n8n-studio/lib/n8n-studio/runtime-trigger";
import { createReadinessResponse } from "../examples/n8n-studio/server/n8n-studio/readiness-response";
import { studioWorkflowDocument } from "../examples/n8n-studio/lib/n8n-studio/workflow-document";
import type { WorkflowDocument } from "../examples/n8n-studio/lib/n8n-studio/types";

const repoRoot = path.resolve(import.meta.dirname, "..");
const webhookSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "Webhook",
  "Webhook.node.ts",
);
const webhookDescriptionPath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "Webhook",
  "description.ts",
);
const manualTriggerPath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "ManualTrigger",
  "ManualTrigger.node.ts",
);

const triggerRecords = [
  {
    credentialId: "credential-n8n-webhook",
    displayName: "Local n8n webhook trigger",
    credentialType: "n8nWebhookTrigger" as const,
    secretRef: "vault://runtime/n8n/webhook/url",
    runtimeTriggerMethod: "POST" as const,
  },
];

const proofRecords = [
  ...triggerRecords,
  {
    credentialId: "credential-n8n-local",
    displayName: "Local n8n API",
    credentialType: "n8nApi" as const,
    baseUrl: "https://n8n.local/api/v1/",
    secretRef: "vault://runtime/n8n/api/key",
  },
];

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

test("n8n Studio plans webhook runtime triggers without enabling manual trigger execution", () => {
  const webhookPlan = createRuntimeTriggerPlan(webhookWorkflowDocument());
  const manualPlan = createRuntimeTriggerPlan(studioWorkflowDocument);
  const serialized = JSON.stringify({ webhookPlan, manualPlan });

  assert.equal(webhookPlan.schema, "dx.n8n-studio.runtime-trigger.plan");
  assert.equal(webhookPlan.status, "provider-gated");
  assert.equal(webhookPlan.triggerMode, "webhook");
  assert.equal(webhookPlan.triggerNodeId, "node-webhook");
  assert.equal(webhookPlan.triggerNodeName, "Inbound Webhook");
  assert.equal(webhookPlan.httpMethod, "POST");
  assert.equal(webhookPlan.requiredCredentialType, "n8nWebhookTrigger");
  assert.equal(webhookPlan.providerBoundary, true);
  assert.equal(webhookPlan.liveProviderExecution, false);
  assert.equal(webhookPlan.secretsIncluded, false);
  assert.equal(webhookPlan.sourcePath, "nodes/Webhook/Webhook.node.ts");
  assert.equal(webhookPlan.sourceDescriptionPath, "nodes/Webhook/description.ts");
  assert.equal(manualPlan.status, "blocked");
  assert.equal(manualPlan.triggerMode, "manual-provider-control");
  assert.equal(manualPlan.requiredCredentialType, undefined);
  assert.match(manualPlan.issue, /provider controls/i);
  assert.equal(serialized.includes("secretRef"), false);
  assert.equal(serialized.includes("apiKey"), false);
});

test("n8n Studio credential vault submits webhook runtime trigger without leaking trigger secrets", async () => {
  const secretRequests: CredentialSecretRequest[] = [];
  const triggerRequests: RuntimeTriggerRequest[] = [];
  const bridge = createCredentialVaultBridge({
    records: triggerRecords,
    loadSecret: async (request) => {
      secretRequests.push(request);
      return "https://n8n.local/webhook/dx-secret-path?token=must-not-survive";
    },
  });
  const document = webhookWorkflowDocument();

  const receipt = await bridge.submitRuntimeTrigger(
    "credential-n8n-webhook",
    async (request) => {
      triggerRequests.push(request);
      return {
        statusCode: 202,
        body: {
          ok: true,
          token: "must-not-survive",
        },
      };
    },
    document,
    {
      apiKey: "must-not-survive",
      safeValue: "kept-for-provider",
    },
  );
  const returnedState = JSON.stringify({
    readiness: createCredentialVaultReadiness(triggerRecords),
    receipt,
  });

  assert.deepEqual(secretRequests, [
    {
      credentialId: "credential-n8n-webhook",
      credentialType: "n8nWebhookTrigger",
      secretRef: "vault://runtime/n8n/webhook/url",
    },
  ]);
  assert.equal(triggerRequests.length, 1);
  assert.equal(triggerRequests[0].method, "POST");
  assert.equal(
    triggerRequests[0].url,
    "https://n8n.local/webhook/dx-secret-path?token=must-not-survive",
  );
  assert.equal(JSON.stringify(triggerRequests[0].body).includes("safeValue"), true);
  assert.equal(
    JSON.stringify(triggerRequests[0].body).includes("must-not-survive"),
    false,
  );
  assert.equal(receipt.schema, "dx.n8n-studio.runtime-trigger.receipt");
  assert.equal(receipt.status, "submitted");
  assert.equal(receipt.workflowExecutionRequested, true);
  assert.equal(receipt.liveProviderExecution, true);
  assert.equal(receipt.executionReceiptImported, false);
  assert.equal(receipt.executionProofRequired, true);
  assert.equal(receipt.targetOrigin, "https://n8n.local");
  assert.equal(receipt.targetUrlStored, false);
  assert.equal(receipt.providerResponse.bodyStored, false);
  assert.equal(receipt.providerResponse.statusCode, 202);
  assert.equal(receipt.secretsIncluded, false);
  assert.equal(returnedState.includes("must-not-survive"), false);
  assert.equal(returnedState.includes("dx-secret-path"), false);
  assert.equal(returnedState.includes("vault://"), false);
  assert.equal(returnedState.includes("apiKey"), false);
});

test("n8n Studio credential vault submits webhook trigger and imports execution proof", async () => {
  const secretRequests: CredentialSecretRequest[] = [];
  const triggerRequests: RuntimeTriggerRequest[] = [];
  const apiRequests: Array<{
    method: string;
    path: string;
    query: Record<string, string | number | boolean>;
    apiKey?: string;
  }> = [];
  const bridge = createCredentialVaultBridge({
    records: proofRecords,
    loadSecret: async (request) => {
      secretRequests.push(request);
      if (request.credentialType === "n8nWebhookTrigger") {
        return "https://n8n.local/webhook/dx-secret-path?token=must-not-survive";
      }

      return "must-not-survive";
    },
  });
  const document = webhookWorkflowDocument();

  const proof = await bridge.submitRuntimeTriggerWithExecutionProof(
    "credential-n8n-webhook",
    "credential-n8n-local",
    async (request) => {
      triggerRequests.push(request);
      return {
        statusCode: 202,
        body: {
          ok: true,
          token: "must-not-survive",
        },
      };
    },
    async (request) => {
      apiRequests.push({
        method: request.method,
        path: request.path,
        query: request.query,
        apiKey: request.headers["X-N8N-API-KEY"],
      });
      return executionHistoryPayload(document.id);
    },
    document,
    {
      apiKey: "must-not-survive",
      safeValue: "kept-for-provider",
    },
  );
  const returnedState = JSON.stringify({
    readiness: bridge.readiness,
    proof,
  });

  assert.deepEqual(secretRequests, [
    {
      credentialId: "credential-n8n-webhook",
      credentialType: "n8nWebhookTrigger",
      secretRef: "vault://runtime/n8n/webhook/url",
    },
    {
      credentialId: "credential-n8n-local",
      credentialType: "n8nApi",
      secretRef: "vault://runtime/n8n/api/key",
    },
  ]);
  assert.equal(triggerRequests.length, 1);
  assert.equal(triggerRequests[0].method, "POST");
  assert.equal(JSON.stringify(triggerRequests[0].body).includes("must-not-survive"), false);
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
  assert.equal(proof.schema, "dx.n8n-studio.runtime-execution-proof.receipt");
  assert.equal(proof.status, "proved");
  assert.equal(proof.workflowId, document.id);
  assert.equal(proof.workflowExecutionRequested, true);
  assert.equal(proof.liveProviderExecution, true);
  assert.equal(proof.executionReceiptImported, true);
  assert.equal(proof.execution.selectedAttemptId, "exec-webhook-proof-1");
  assert.equal(proof.execution.nodeLogCount, 2);
  assert.equal(proof.trigger.targetOrigin, "https://n8n.local");
  assert.equal(proof.trigger.targetUrlStored, false);
  assert.equal(proof.secretsIncluded, false);
  assert.equal(returnedState.includes("must-not-survive"), false);
  assert.equal(returnedState.includes("dx-secret-path"), false);
  assert.equal(returnedState.includes("vault://"), false);
  assert.equal(returnedState.includes("apiKey"), false);
});

test("n8n Studio writes runtime execution proof receipts and plans delayed history retries", async () => {
  const helperPath = path.join(
    repoRoot,
    "examples",
    "n8n-studio",
    "server",
    "n8n-studio",
    "runtime-execution-proof-receipts.ts",
  );
  const receiptRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-n8n-runtime-proof-receipts-"),
  );
  const triggerRequests: RuntimeTriggerRequest[] = [];
  const bridge = createCredentialVaultBridge({
    records: triggerRecords,
    loadSecret: async () =>
      "https://n8n.local/webhook/dx-secret-path?token=must-not-survive",
  });
  const document = webhookWorkflowDocument();
  const trigger = await bridge.submitRuntimeTrigger(
    "credential-n8n-webhook",
    async (request) => {
      triggerRequests.push(request);
      return { statusCode: 202, body: { token: "must-not-survive" } };
    },
    document,
    { apiKey: "must-not-survive", ok: true },
  );
  const proof = createRuntimeExecutionProofReceipt({
    document,
    trigger,
    execution: createExecutionReadiness(document),
  });

  assert.equal(fs.existsSync(helperPath), true);
  const helper = await import(pathToFileURL(helperPath).href);
  const writeProofReceipt = helper.writeRuntimeExecutionProofReceipt as (options: {
    receiptRoot: string;
    proof: typeof proof;
  }) => {
    schema: string;
    receiptRoot: string;
    receiptPath: string;
    relativeReceiptPath: string;
    status: string;
    providerBoundary: true;
    liveProviderExecution: false;
    secretsIncluded: false;
    runtimeProof: { liveProviderExecution: true; executionReceiptImported: false };
  };
  const createRetryPlan = helper.createRuntimeExecutionProofRetryPlan as (options: {
    proof: typeof proof;
    attempt: number;
    maxAttempts: number;
  }) => {
    schema: string;
    status: string;
    workflowId: string;
    nextAction: string;
    attempt: number;
    maxAttempts: number;
    nextAttempt: { attempt: number; delayMs: number };
    providerBoundary: true;
    liveProviderExecution: false;
    secretsIncluded: false;
  };

  assert.equal(typeof writeProofReceipt, "function");
  assert.equal(typeof createRetryPlan, "function");

  const receipt = writeProofReceipt({ receiptRoot, proof });
  const writtenProof = JSON.parse(fs.readFileSync(receipt.receiptPath, "utf8"));
  const retryPlan = createRetryPlan({ proof, attempt: 1, maxAttempts: 4 });
  const serialized = JSON.stringify({ receipt, writtenProof, retryPlan });

  assert.equal(receipt.schema, "dx.n8n-studio.runtime-execution-proof.export");
  assert.equal(receipt.receiptRoot, receiptRoot);
  assert.equal(
    receipt.receiptPath,
    path.join(receiptRoot, "n8n-studio", "runtime", "execution-proof-latest.sr"),
  );
  assert.equal(
    receipt.relativeReceiptPath,
    "n8n-studio/runtime/execution-proof-latest.sr",
  );
  assert.equal(receipt.status, "proof-blocked");
  assert.equal(receipt.providerBoundary, true);
  assert.equal(receipt.liveProviderExecution, false);
  assert.equal(receipt.secretsIncluded, false);
  assert.deepEqual(receipt.runtimeProof, {
    liveProviderExecution: true,
    executionReceiptImported: false,
  });
  assert.equal(writtenProof.schema, "dx.n8n-studio.runtime-execution-proof.receipt");
  assert.equal(writtenProof.status, "proof-blocked");
  assert.equal(writtenProof.trigger.targetOrigin, "https://n8n.local");
  assert.equal(writtenProof.trigger.targetUrlStored, false);
  assert.equal(writtenProof.secretsIncluded, false);
  assert.equal(retryPlan.schema, "dx.n8n-studio.runtime-execution-proof.retry-plan");
  assert.equal(retryPlan.status, "scheduled");
  assert.equal(retryPlan.workflowId, document.id);
  assert.equal(retryPlan.nextAction, "import-execution-history");
  assert.equal(retryPlan.attempt, 1);
  assert.equal(retryPlan.maxAttempts, 4);
  assert.deepEqual(retryPlan.nextAttempt, {
    attempt: 2,
    delayMs: 2000,
  });
  assert.equal(retryPlan.providerBoundary, true);
  assert.equal(retryPlan.liveProviderExecution, false);
  assert.equal(retryPlan.secretsIncluded, false);
  assert.equal(triggerRequests[0].url.includes("must-not-survive"), true);
  assert.equal(serialized.includes("must-not-survive"), false);
  assert.equal(serialized.includes("dx-secret-path"), false);
  assert.equal(serialized.includes("vault://"), false);
  assert.equal(serialized.includes("apiKey"), false);
});

test("n8n Studio retries delayed execution history import until proof is available", async () => {
  const helperPath = path.join(
    repoRoot,
    "examples",
    "n8n-studio",
    "server",
    "n8n-studio",
    "runtime-execution-proof-receipts.ts",
  );
  const receiptRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-n8n-runtime-proof-retry-"),
  );
  const bridge = createCredentialVaultBridge({
    records: triggerRecords,
    loadSecret: async () =>
      "https://n8n.local/webhook/dx-secret-path?token=must-not-survive",
  });
  const document = webhookWorkflowDocument();
  const trigger = await bridge.submitRuntimeTrigger(
    "credential-n8n-webhook",
    async () => ({ statusCode: 202 }),
    document,
  );
  const helper = await import(pathToFileURL(helperPath).href);
  const runRetry = helper.runRuntimeExecutionProofRetry as (options: {
    document: typeof document;
    trigger: typeof trigger;
    receiptRoot: string;
    maxAttempts: number;
    importExecutionHistory: (request: { attempt: number }) => Promise<unknown>;
  }) => Promise<{
    schema: string;
    status: string;
    attemptCount: number;
    retryPlans: Array<{
      status: string;
      nextAttempt?: { attempt: number; delayMs: number };
    }>;
    proof: { status: string; executionReceiptImported: boolean };
    receiptExport: { receiptPath: string; status: string };
    providerBoundary: true;
    liveProviderExecution: false;
    secretsIncluded: false;
  }>;
  const importAttempts: number[] = [];

  assert.equal(typeof runRetry, "function");

  const result = await runRetry({
    document,
    trigger,
    receiptRoot,
    maxAttempts: 3,
    async importExecutionHistory(request) {
      importAttempts.push(request.attempt);
      if (request.attempt === 1) {
        return createExecutionReadiness(document);
      }

      return importExecutionHistoryApiResponse(
        document,
        executionHistoryPayload(document.id),
      );
    },
  });
  const writtenProof = JSON.parse(
    fs.readFileSync(result.receiptExport.receiptPath, "utf8"),
  );
  const serialized = JSON.stringify({ result, writtenProof });

  assert.deepEqual(importAttempts, [1, 2]);
  assert.equal(result.schema, "dx.n8n-studio.runtime-execution-proof.retry-result");
  assert.equal(result.status, "proved");
  assert.equal(result.attemptCount, 2);
  assert.equal(result.retryPlans.length, 1);
  assert.equal(result.retryPlans[0].status, "scheduled");
  assert.deepEqual(result.retryPlans[0].nextAttempt, {
    attempt: 2,
    delayMs: 2000,
  });
  assert.equal(result.proof.status, "proved");
  assert.equal(result.proof.executionReceiptImported, true);
  assert.equal(result.receiptExport.status, "proved");
  assert.equal(writtenProof.status, "proved");
  assert.equal(result.providerBoundary, true);
  assert.equal(result.liveProviderExecution, false);
  assert.equal(result.secretsIncluded, false);
  assert.equal(serialized.includes("must-not-survive"), false);
  assert.equal(serialized.includes("dx-secret-path"), false);
  assert.equal(serialized.includes("vault://"), false);
  assert.equal(serialized.includes("apiKey"), false);
});

test("n8n Studio readiness exposes runtime trigger planning without live execution", () => {
  const response = createReadinessResponse();
  const serialized = JSON.stringify(response);

  assert.equal(response.runtimeTrigger.schema, "dx.n8n-studio.runtime-trigger.plan");
  assert.equal(response.runtimeTrigger.status, "blocked");
  assert.equal(response.runtimeTrigger.triggerMode, "manual-provider-control");
  assert.equal(response.runtimeTrigger.providerBoundary, true);
  assert.equal(response.runtimeTrigger.liveProviderExecution, false);
  assert.equal(response.runtimeTrigger.secretsIncluded, false);
  assert.equal(response.credentialVault.n8nRuntimeTriggerCredentialCount, 0);
  assert.deepEqual(response.credentialVault.n8nRuntimeTriggerCredentialIds, []);
  assert.equal(serialized.includes("secretRef"), false);
  assert.equal(serialized.includes("apiKey"), false);
});

test("n8n Studio runtime trigger bridge remains tied to local Webhook source", () => {
  const webhookSource = fs.readFileSync(webhookSourcePath, "utf8");
  const webhookDescription = fs.readFileSync(webhookDescriptionPath, "utf8");
  const manualTrigger = fs.readFileSync(manualTriggerPath, "utf8");

  assert.match(webhookSource, /Starts the workflow when a webhook is called/);
  assert.match(webhookSource, /activationMessage/);
  assert.match(webhookSource, /production webhook URL/);
  assert.match(webhookDescription, /defaultWebhookDescription/);
  assert.match(webhookDescription, /httpMethod/);
  assert.match(manualTrigger, /Runs the flow on clicking a button in n8n/);
});
