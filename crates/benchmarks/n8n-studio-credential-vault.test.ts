import assert from "node:assert/strict";
import test from "node:test";

import {
  createCredentialVaultBridge,
  createCredentialVaultReadiness,
  type CredentialSecretRequest,
} from "../examples/n8n-studio/lib/n8n-studio/credential-vault";
import type { N8nApiClientRequest } from "../examples/n8n-studio/lib/n8n-studio/n8n-api-client";
import { createReadinessResponse } from "../examples/n8n-studio/server/n8n-studio/readiness-response";
import { studioWorkflowDocument } from "../examples/n8n-studio/lib/n8n-studio/workflow-document";

const vaultRecords = [
  {
    credentialId: "credential-n8n-local",
    displayName: "Local n8n API",
    credentialType: "n8nApi" as const,
    baseUrl: "https://n8n.local/api/v1/",
    secretRef: "vault://credentials/n8n/local/api-key",
  },
  {
    credentialId: "credential-openai",
    displayName: "OpenAI API",
    credentialType: "OpenAiApi" as const,
    secretRef: "vault://credentials/openai/api-key",
  },
];

function workflowPayload() {
  return {
    id: studioWorkflowDocument.id,
    name: "Vault imported workflow",
    nodes: [],
    connections: {},
  };
}

function executionHistoryPayload(workflowId: string) {
  return {
    data: [
      {
        id: "exec-vault-handoff-1",
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

test("n8n Studio credential vault readiness keeps only redacted credential metadata", () => {
  const readiness = createCredentialVaultReadiness(vaultRecords);
  const serialized = JSON.stringify(readiness);

  assert.equal(readiness.schema, "dx.n8n-studio.credential-vault");
  assert.equal(readiness.status, "configured");
  assert.equal(readiness.providerBoundary, true);
  assert.equal(readiness.liveProviderExecution, false);
  assert.equal(readiness.secretsIncluded, false);
  assert.equal(readiness.credentialCount, 2);
  assert.equal(readiness.n8nApiCredentialCount, 1);
  assert.deepEqual(readiness.n8nApiCredentialIds, ["credential-n8n-local"]);
  assert.equal(serialized.includes("vault://"), false);
  assert.equal(serialized.includes("api-key"), false);
  assert.equal(serialized.includes("apiKey"), false);
});

test("n8n Studio credential vault creates an n8n API client without persisting secrets in returned state", async () => {
  const secretRequests: CredentialSecretRequest[] = [];
  const transportRequests: N8nApiClientRequest[] = [];
  const bridge = createCredentialVaultBridge({
    records: vaultRecords,
    loadSecret: async (request) => {
      secretRequests.push(request);
      return "must-not-survive";
    },
  });

  const client = await bridge.createN8nApiClient(
    "credential-n8n-local",
    async (request) => {
      transportRequests.push(request);
      return workflowPayload();
    },
  );
  const preview = await client.importWorkflow(studioWorkflowDocument.id);
  const returnedState = JSON.stringify({
    vault: bridge.readiness,
    client: client.readiness,
    preview,
  });

  assert.deepEqual(secretRequests, [
    {
      credentialId: "credential-n8n-local",
      credentialType: "n8nApi",
      secretRef: "vault://credentials/n8n/local/api-key",
    },
  ]);
  assert.equal(transportRequests.length, 1);
  assert.equal(transportRequests[0].headers["X-N8N-API-KEY"], "must-not-survive");
  assert.equal(client.readiness.status, "configured");
  assert.equal(client.readiness.credentialId, "credential-n8n-local");
  assert.equal(preview.boundary.secretsIncluded, false);
  assert.equal(returnedState.includes("must-not-survive"), false);
  assert.equal(returnedState.includes("vault://"), false);
  assert.equal(returnedState.includes("api-key"), false);
  assert.equal(returnedState.includes("apiKey"), false);
});

test("n8n Studio credential vault submits runtime handoff with adapter-owned secret loading", async () => {
  const secretRequests: CredentialSecretRequest[] = [];
  const transportRequests: N8nApiClientRequest[] = [];
  const bridge = createCredentialVaultBridge({
    records: vaultRecords,
    loadSecret: async (request) => {
      secretRequests.push(request);
      return "must-not-survive";
    },
  });
  const document = {
    ...studioWorkflowDocument,
    id: "vault-runtime-handoff",
    nodes: [
      {
        ...studioWorkflowDocument.nodes[0],
        parameters: {
          apiKey: "must-not-survive",
          label: "Vault handoff",
        },
      },
    ],
  };

  const receipt = await bridge.submitRuntimeHandoff(
    "credential-n8n-local",
    async (request) => {
      transportRequests.push(request);

      if (request.path === "/workflows") {
        return {
          id: "vault-provider-workflow",
          name: document.name,
        };
      }

      if (request.path === "/workflows/{id}/activate") {
        return {
          id: "vault-provider-workflow",
          active: true,
        };
      }

      if (request.path === "/executions") {
        return executionHistoryPayload("vault-provider-workflow");
      }

      throw new Error(`Unexpected request ${request.path}`);
    },
    document,
    {
      activate: true,
      importExecutionHistory: true,
    },
  );
  const returnedState = JSON.stringify({
    vault: bridge.readiness,
    receipt,
  });

  assert.deepEqual(secretRequests, [
    {
      credentialId: "credential-n8n-local",
      credentialType: "n8nApi",
      secretRef: "vault://credentials/n8n/local/api-key",
    },
  ]);
  assert.deepEqual(
    transportRequests.map((request) => [request.method, request.path]),
    [
      ["POST", "/workflows"],
      ["POST", "/workflows/{id}/activate"],
      ["GET", "/executions"],
    ],
  );
  assert.equal(
    transportRequests.every(
      (request) => request.headers["X-N8N-API-KEY"] === "must-not-survive",
    ),
    true,
  );
  assert.equal(
    JSON.stringify(transportRequests[0].body).includes("must-not-survive"),
    false,
  );
  assert.equal(receipt.schema, "dx.n8n-studio.runtime-handoff.receipt");
  assert.equal(receipt.status, "submitted-with-history");
  assert.equal(receipt.workflowId, "vault-provider-workflow");
  assert.equal(receipt.execution?.selectedAttemptId, "exec-vault-handoff-1");
  assert.equal(receipt.providerBoundary, true);
  assert.equal(receipt.liveProviderExecution, false);
  assert.equal(receipt.workflowExecutionRequested, false);
  assert.equal(receipt.secretsIncluded, false);
  assert.equal(returnedState.includes("must-not-survive"), false);
  assert.equal(returnedState.includes("vault://"), false);
  assert.equal(returnedState.includes("api-key"), false);
  assert.equal(returnedState.includes("apiKey"), false);
});

test("n8n Studio credential vault validates provider credentials through adapter-owned secret loading", async () => {
  const secretRequests: CredentialSecretRequest[] = [];
  const validationRequests: Array<{
    credentialId: string;
    credentialType: string;
    displayName: string;
    secretValue: string;
  }> = [];
  const bridge = createCredentialVaultBridge({
    records: vaultRecords,
    loadSecret: async (request) => {
      secretRequests.push(request);
      return "must-not-survive";
    },
  });

  assert.equal(typeof bridge.validateCredential, "function");

  const receipt = await bridge.validateCredential(
    "credential-openai",
    async (request) => {
      validationRequests.push({
        credentialId: request.credentialId,
        credentialType: request.credentialType,
        displayName: request.displayName,
        secretValue: request.secretValue,
      });

      return {
        status: "valid",
        statusCode: 200,
        message: "Credential validated by provider adapter.",
      };
    },
  );
  const returnedState = JSON.stringify({
    vault: bridge.readiness,
    receipt,
  });

  assert.deepEqual(secretRequests, [
    {
      credentialId: "credential-openai",
      credentialType: "OpenAiApi",
      secretRef: "vault://credentials/openai/api-key",
    },
  ]);
  assert.deepEqual(validationRequests, [
    {
      credentialId: "credential-openai",
      credentialType: "OpenAiApi",
      displayName: "OpenAI API",
      secretValue: "must-not-survive",
    },
  ]);
  assert.equal(receipt.schema, "dx.n8n-studio.credential-validation.receipt");
  assert.equal(receipt.status, "valid");
  assert.equal(receipt.credentialId, "credential-openai");
  assert.equal(receipt.credentialType, "OpenAiApi");
  assert.equal(receipt.providerBoundary, true);
  assert.equal(receipt.liveProviderExecution, true);
  assert.equal(receipt.secretsIncluded, false);
  assert.equal(receipt.providerResponse.bodyStored, false);
  assert.equal(receipt.providerResponse.secretsIncluded, false);
  assert.equal(returnedState.includes("must-not-survive"), false);
  assert.equal(returnedState.includes("vault://"), false);
  assert.equal(returnedState.includes("api-key"), false);
  assert.equal(returnedState.includes("apiKey"), false);
});

test("n8n Studio credential vault blocks unavailable n8n API secrets without naming secret refs", async () => {
  const bridge = createCredentialVaultBridge({
    records: vaultRecords,
    loadSecret: async () => undefined,
  });

  await assert.rejects(
    () => bridge.createN8nApiClient("credential-n8n-local", async () => workflowPayload()),
    /Credential secret is unavailable/,
  );
  await assert.rejects(
    () => bridge.createN8nApiClient("missing-credential", async () => workflowPayload()),
    /n8n API credential is not available/,
  );
});

test("n8n Studio credential vault blocks invalid n8n API base URLs", async () => {
  const records = [
    {
      ...vaultRecords[0],
      baseUrl: "file:///tmp/n8n",
    },
  ];
  const readiness = createCredentialVaultReadiness(records);
  const bridge = createCredentialVaultBridge({
    records,
    loadSecret: async () => "must-not-survive",
  });

  assert.equal(readiness.status, "blocked");
  assert.equal(readiness.n8nApiCredentialCount, 0);
  await assert.rejects(
    () => bridge.createN8nApiClient("credential-n8n-local", async () => workflowPayload()),
    /http or https/,
  );
});

test("n8n Studio readiness response exposes the credential vault bridge boundary", () => {
  const response = createReadinessResponse();

  assert.equal(response.credentialVault.status, "blocked");
  assert.equal(response.credentialVault.providerBoundary, true);
  assert.equal(response.credentialVault.liveProviderExecution, false);
  assert.equal(response.credentialVault.secretsIncluded, false);
  assert.equal(response.credentialVault.n8nApiCredentialCount, 0);
  assert.equal(JSON.stringify(response.credentialVault).includes("apiKey"), false);
});
