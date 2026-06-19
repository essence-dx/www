import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import {
  createN8nApiClient,
  createN8nApiClientReadiness,
  type N8nApiClientRequest,
} from "../examples/n8n-studio/lib/n8n-studio/n8n-api-client";
import { createReadinessResponse } from "../examples/n8n-studio/server/n8n-studio/readiness-response";
import { studioWorkflowDocument } from "../examples/n8n-studio/lib/n8n-studio/workflow-document";

const repoRoot = path.resolve(import.meta.dirname, "..");
const workflowSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "N8n",
  "WorkflowDescription.ts",
);
const executionSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "N8n",
  "ExecutionDescription.ts",
);
const coveragePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "N8n",
  "n8n-api-coverage.json",
);

const credentials = {
  credentialId: "credential-n8n-local",
  displayName: "Local n8n API",
  baseUrl: "https://n8n.local/api/v1/",
  apiKey: "must-not-survive",
};

function workflowPayload() {
  return {
    id: studioWorkflowDocument.id,
    name: "Imported from n8n API",
    active: true,
    nodes: [
      {
        id: "api-http",
        name: "API HTTP",
        type: "n8n-nodes-base.httpRequest",
        typeVersion: 4,
        position: [120, 240],
        parameters: {
          url: "https://example.com",
          accessToken: "must-not-survive",
        },
        credentials: {
          httpBearerAuth: {
            id: "credential-id-only",
            name: "Bearer token",
            data: {
              apiKey: "must-not-survive",
            },
          },
        },
      },
    ],
    connections: {},
    pinData: {
      "API HTTP": [{ ok: true }],
    },
  };
}

function executionHistoryPayload(workflowId = studioWorkflowDocument.id) {
  return {
    data: [
      {
        id: "exec-api-client-1",
        workflowId,
        mode: "manual",
        status: "success",
        startedAt: "2026-06-08T12:00:00.000Z",
        stoppedAt: "2026-06-08T12:00:01.000Z",
        data: {
          resultData: {
            runData: {
              "Manual Trigger": [
                {
                  executionTime: 3,
                  data: {
                    main: [[{ json: { ok: true } }]],
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

test("n8n Studio exposes redacted n8n API client readiness", () => {
  const readiness = createN8nApiClientReadiness(credentials);

  assert.equal(readiness.status, "configured");
  assert.equal(readiness.credentialType, "n8nApi");
  assert.equal(readiness.credentialId, "credential-n8n-local");
  assert.equal(readiness.providerBoundary, true);
  assert.equal(readiness.secretsIncluded, false);
  assert.equal(readiness.workflowImportAvailable, true);
  assert.equal(readiness.executionHistoryImportAvailable, true);
  assert.equal(readiness.workflowPublishAvailable, true);
  assert.equal(readiness.workflowActivationAvailable, true);
  assert.equal(readiness.runtimeHandoffAvailable, true);
  assert.equal(readiness.baseUrlOrigin, "https://n8n.local");
  assert.equal(JSON.stringify(readiness).includes("must-not-survive"), false);
  assert.equal(JSON.stringify(readiness).includes("apiKey"), false);
});

test("n8n Studio n8n API client readiness blocks invalid base URLs", () => {
  const readiness = createN8nApiClientReadiness({
    ...credentials,
    baseUrl: "file:///tmp/n8n",
  });

  assert.equal(readiness.status, "blocked");
  assert.equal(readiness.workflowImportAvailable, false);
  assert.equal(readiness.executionHistoryImportAvailable, false);
  assert.equal(readiness.workflowPublishAvailable, false);
  assert.equal(readiness.workflowActivationAvailable, false);
  assert.equal(readiness.runtimeHandoffAvailable, false);
  assert.equal(readiness.baseUrlOrigin, undefined);
  assert.match(readiness.issue, /http or https/);
});

test("n8n Studio n8n API client imports workflows and execution history without leaking secrets", async () => {
  const requests: N8nApiClientRequest[] = [];
  const client = createN8nApiClient(credentials, async (request) => {
    requests.push(request);

    if (request.path === "/workflows/{id}") {
      return workflowPayload();
    }

    if (request.path === "/executions") {
      return executionHistoryPayload();
    }

    throw new Error(`Unexpected request ${request.path}`);
  });

  const workflowPreview = await client.importWorkflow(studioWorkflowDocument.id);
  const execution = await client.importExecutionHistory(studioWorkflowDocument);
  const returnedJson = JSON.stringify({ workflowPreview, execution });

  assert.equal(requests.length, 2);
  assert.equal(
    requests[0].url,
    "https://n8n.local/api/v1/workflows/dx-n8n-studio-workflow",
  );
  assert.equal(requests[0].headers["X-N8N-API-KEY"], "must-not-survive");
  assert.equal(requests[1].url, "https://n8n.local/api/v1/executions");
  assert.deepEqual(requests[1].query, {
    workflowId: studioWorkflowDocument.id,
    includeData: true,
    limit: 100,
  });
  assert.equal(workflowPreview.status, "sanitized-with-issues");
  assert.equal(workflowPreview.source, "url");
  assert.equal(workflowPreview.boundary.secretsIncluded, false);
  assert.equal(execution.status, "configured");
  assert.equal(execution.selectedAttemptId, "exec-api-client-1");
  assert.equal(execution.receiptBoundary.executionReceiptImported, true);
  assert.equal(returnedJson.includes("must-not-survive"), false);
  assert.equal(returnedJson.includes("apiKey"), false);
  assert.equal(returnedJson.includes("accessToken"), false);
});

test("n8n Studio n8n API client publishes workflow JSON through governed write requests", async () => {
  const requests: N8nApiClientRequest[] = [];
  const document = {
    ...studioWorkflowDocument,
    id: "published-workflow",
    active: true,
    nodes: [
      {
        ...studioWorkflowDocument.nodes[0],
        parameters: {
          apiKey: "must-not-survive",
          nested: {
            accessToken: "must-not-survive",
            safeLabel: "kept",
          },
        },
      },
    ],
    connections: [],
  };
  const client = createN8nApiClient(credentials, async (request) => {
    requests.push(request);

    if (request.path === "/workflows/{id}/activate") {
      return {
        id: "created-workflow",
        name: document.name,
        active: true,
      };
    }

    if (request.path === "/workflows") {
      return {
        id: "created-workflow",
        name: document.name,
        active: false,
      };
    }

    throw new Error(`Unexpected request ${request.path}`);
  });

  const receipt = await client.publishWorkflow(document, { activate: true });
  const publishBody = requests[0].body as Record<string, unknown>;
  const returnedJson = JSON.stringify({ receipt, publishBody });

  assert.equal(requests.length, 2);
  assert.equal(requests[0].method, "POST");
  assert.equal(requests[0].path, "/workflows");
  assert.equal(requests[0].url, "https://n8n.local/api/v1/workflows");
  assert.deepEqual(Object.keys(publishBody).sort(), [
    "connections",
    "name",
    "nodes",
    "settings",
    "staticData",
  ]);
  assert.equal(JSON.stringify(publishBody).includes("safeLabel"), true);
  assert.equal(JSON.stringify(publishBody).includes("must-not-survive"), false);
  assert.equal(requests[1].method, "POST");
  assert.equal(requests[1].path, "/workflows/{id}/activate");
  assert.equal(
    requests[1].url,
    "https://n8n.local/api/v1/workflows/created-workflow/activate",
  );
  assert.equal(receipt.schema, "dx.n8n-studio.workflow-publish.receipt");
  assert.equal(receipt.status, "submitted");
  assert.equal(receipt.operation, "create");
  assert.equal(receipt.workflowId, "created-workflow");
  assert.equal(receipt.providerWrite, true);
  assert.equal(receipt.liveProviderExecution, false);
  assert.equal(receipt.secretsIncluded, false);
  assert.equal(receipt.activation.status, "submitted");
  assert.equal(returnedJson.includes("must-not-survive"), false);
  assert.equal(returnedJson.includes("apiKey"), false);
  assert.equal(returnedJson.includes("accessToken"), false);
});

test("n8n Studio n8n API client updates an existing workflow without activating it", async () => {
  const requests: N8nApiClientRequest[] = [];
  const client = createN8nApiClient(credentials, async (request) => {
    requests.push(request);
    return {
      id: "existing-workflow",
      name: studioWorkflowDocument.name,
      active: false,
    };
  });

  const receipt = await client.publishWorkflow(studioWorkflowDocument, {
    targetWorkflowId: "existing-workflow",
    activate: false,
  });

  assert.equal(requests.length, 1);
  assert.equal(requests[0].method, "PUT");
  assert.equal(requests[0].path, "/workflows/{id}");
  assert.equal(
    requests[0].url,
    "https://n8n.local/api/v1/workflows/existing-workflow",
  );
  assert.equal(receipt.operation, "update");
  assert.equal(receipt.workflowId, "existing-workflow");
  assert.equal(receipt.activation.status, "not-requested");
});

test("n8n Studio n8n API client submits runtime handoff without pretending to run workflows", async () => {
  const requests: N8nApiClientRequest[] = [];
  const document = {
    ...studioWorkflowDocument,
    id: "runtime-handoff-workflow",
    active: true,
    nodes: [
      {
        ...studioWorkflowDocument.nodes[0],
        parameters: {
          apiKey: "must-not-survive",
          safeLabel: "Runtime handoff",
        },
      },
    ],
  };
  const client = createN8nApiClient(credentials, async (request) => {
    requests.push(request);

    if (request.path === "/workflows") {
      return {
        id: "runtime-provider-workflow",
        name: document.name,
        active: false,
      };
    }

    if (request.path === "/workflows/{id}/activate") {
      return {
        id: "runtime-provider-workflow",
        active: true,
      };
    }

    if (request.path === "/executions") {
      return executionHistoryPayload("runtime-provider-workflow");
    }

    throw new Error(`Unexpected request ${request.path}`);
  });

  const receipt = await client.submitRuntimeHandoff(document, {
    activate: true,
    importExecutionHistory: true,
  });
  const returnedJson = JSON.stringify(receipt);

  assert.deepEqual(
    requests.map((request) => [request.method, request.path]),
    [
      ["POST", "/workflows"],
      ["POST", "/workflows/{id}/activate"],
      ["GET", "/executions"],
    ],
  );
  assert.deepEqual(requests[2].query, {
    workflowId: "runtime-provider-workflow",
    includeData: true,
    limit: 100,
  });
  assert.equal(
    requests.some((request) =>
      request.path === "/executions/{id}" ||
      /\/(?:run|retry|stop)(?:\/|$)/.test(new URL(request.url).pathname)
    ),
    false,
  );
  assert.equal(
    JSON.stringify(requests[0].body).includes("must-not-survive"),
    false,
  );
  assert.equal(receipt.schema, "dx.n8n-studio.runtime-handoff.receipt");
  assert.equal(receipt.status, "submitted-with-history");
  assert.equal(receipt.workflowId, "runtime-provider-workflow");
  assert.equal(receipt.providerBoundary, true);
  assert.equal(receipt.providerWrite, true);
  assert.equal(receipt.workflowExecutionRequested, false);
  assert.equal(receipt.liveProviderExecution, false);
  assert.equal(receipt.executionHistoryImportRequested, true);
  assert.equal(receipt.executionReceiptImported, true);
  assert.equal(receipt.secretsIncluded, false);
  assert.equal(receipt.publish.workflowId, "runtime-provider-workflow");
  assert.equal(receipt.execution?.selectedAttemptId, "exec-api-client-1");
  assert.equal(returnedJson.includes("must-not-survive"), false);
  assert.equal(returnedJson.includes("apiKey"), false);
  assert.equal(returnedJson.includes("accessToken"), false);
});

test("n8n Studio n8n API client remains tied to locally mirrored n8n API source", () => {
  const workflowSource = fs.readFileSync(workflowSourcePath, "utf8");
  const executionSource = fs.readFileSync(executionSourcePath, "utf8");
  const coverage = JSON.parse(fs.readFileSync(coveragePath, "utf8"));

  assert.match(workflowSource, /method:\s*'POST',\s*\n\s*url:\s*'\/workflows'/);
  assert.match(workflowSource, /method:\s*'PUT',\s*\n\s*url:\s*'=\/workflows\/{{ \$value }}'/);
  assert.match(workflowSource, /method:\s*'POST',\s*\n\s*url:\s*'=\/workflows\/{{ \$value }}\/activate'/);
  assert.match(workflowSource, /url:\s*'\/workflows'/);
  assert.match(workflowSource, /url:\s*'=\/workflows\/{{ \$value }}'/);
  assert.match(executionSource, /url:\s*'\/executions'/);
  assert.equal(coverage.endpoints["POST /workflows"].status, "covered");
  assert.equal(coverage.endpoints["PUT /workflows/{id}"].status, "covered");
  assert.equal(coverage.endpoints["POST /workflows/{id}/activate"].status, "covered");
  assert.equal(coverage.endpoints["GET /workflows"].status, "covered");
  assert.equal(coverage.endpoints["GET /workflows/{id}"].status, "covered");
  assert.equal(coverage.endpoints["GET /executions"].status, "covered");
});

test("n8n Studio readiness response exposes the n8n API client boundary", () => {
  const response = createReadinessResponse();

  assert.equal(response.n8nApiClient.status, "blocked");
  assert.equal(response.n8nApiClient.credentialType, "n8nApi");
  assert.equal(response.n8nApiClient.workflowImportAvailable, false);
  assert.equal(response.n8nApiClient.executionHistoryImportAvailable, false);
  assert.equal(response.n8nApiClient.workflowPublishAvailable, false);
  assert.equal(response.n8nApiClient.workflowActivationAvailable, false);
  assert.equal(response.n8nApiClient.runtimeHandoffAvailable, false);
  assert.equal(response.n8nApiClient.secretsIncluded, false);
  assert.equal(JSON.stringify(response.n8nApiClient).includes("apiKey"), false);
});
