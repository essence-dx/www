import assert from "node:assert/strict";
import test from "node:test";

import {
  createParameterSchemaForNode,
  getNodeTypeDescription,
} from "../examples/n8n-studio/lib/n8n-studio/node-type-registry";
import { sanitizeImportedWorkflow } from "../examples/n8n-studio/lib/n8n-studio/import-export";

test("n8n Studio builds HTTP Request parameters from semantic node metadata", () => {
  const description = getNodeTypeDescription("n8n-nodes-base.httpRequest");
  assert.equal(description.sourcePath, "nodes/HttpRequest/HttpRequest.node.ts");
  assert.equal(description.credentials.some((credential) => credential.name === "HttpBearerAuth"), true);

  const schema = createParameterSchemaForNode("n8n-nodes-base.httpRequest", {
    authentication: "genericCredentialType",
  });

  const method = schema.fields.find((field) => field.name === "method");
  assert.equal(method?.type, "options");
  assert.deepEqual(method?.options?.map((option) => option.value), [
    "DELETE",
    "GET",
    "HEAD",
    "OPTIONS",
    "PATCH",
    "POST",
    "PUT",
  ]);

  const url = schema.fields.find((field) => field.name === "url");
  assert.equal(url?.required, true);
  assert.equal(url?.expressionEnabled, true);

  const genericAuthType = schema.fields.find((field) => field.name === "genericAuthType");
  assert.equal(genericAuthType?.type, "credentialsSelect");
  assert.deepEqual(genericAuthType?.credentialTypes, ["has:genericAuth"]);

  const predefinedCredential = schema.fields.find((field) => field.name === "nodeCredentialType");
  assert.equal(predefinedCredential, undefined);
});

test("n8n Studio preserves resource locator modes and dynamic load boundaries", () => {
  const schema = createParameterSchemaForNode("n8n-nodes-base.slack", {
    resource: "message",
    operation: "post",
    select: "channel",
  });

  const channel = schema.fields.find((field) => field.name === "channelId");
  assert.equal(channel?.type, "resourceLocator");
  assert.deepEqual(channel?.resourceLocatorModes?.map((mode) => mode.name), [
    "list",
    "id",
    "name",
    "url",
  ]);
  assert.equal(channel?.resourceLocatorModes?.[0]?.searchListMethod, "getChannels");
  assert.equal(schema.dynamicLoadBoundaries.length > 0, true);
});

test("n8n Studio import sanitation strips secrets, drops unknown nodes, and records issues", () => {
  const result = sanitizeImportedWorkflow({
    name: "Imported workflow",
    active: true,
    nodes: [
      {
        id: "known-http",
        name: "Known HTTP",
        type: "n8n-nodes-base.httpRequest",
        typeVersion: 4,
        position: [120, 220],
        parameters: {
          url: "https://example.com",
          apiKey: "must-not-survive",
        },
        credentials: {
          httpBearerAuth: {
            id: "cred-1",
            name: "Bearer token",
            data: {
              token: "must-not-survive",
            },
          },
        },
      },
      {
        id: "unknown-node",
        name: "Unknown",
        type: "n8n-nodes-base.unknown",
        typeVersion: 1,
        position: [320, 220],
        parameters: {},
      },
    ],
    connections: {
      "Known HTTP": {
        main: [[{ node: "Unknown", type: "main", index: 0 }]],
      },
    },
    pinData: {
      "Known HTTP": [{ ok: true }],
      Unknown: [{ leak: true }],
    },
    meta: {
      webhookId: "external-webhook-id",
    },
  });

  assert.equal(result.document.nodes.length, 1);
  assert.equal(result.document.nodes[0]?.name, "Known HTTP");
  assert.equal(result.document.active, false);
  assert.deepEqual(result.document.connections, []);
  assert.deepEqual(result.document.pinData.Unknown, undefined);
  assert.equal(result.issues.some((issue) => issue.code === "unknown-node-type"), true);
  assert.equal(result.issues.some((issue) => issue.code === "credential-secret-stripped"), true);
  assert.equal(result.issues.some((issue) => issue.code === "connection-target-missing"), true);
  assert.equal(JSON.stringify(result).includes("must-not-survive"), false);
  assert.equal(result.regeneratedWebhookIds.length, 1);
});
