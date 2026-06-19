import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import * as credentialModel from "../examples/n8n-studio/lib/n8n-studio/credentials";
import { getNodeTypeDescription } from "../examples/n8n-studio/lib/n8n-studio/node-type-registry";
import { createN8nStudioState } from "../examples/n8n-studio/lib/n8n-studio/studio-state";
import type { WorkflowNode } from "../examples/n8n-studio/lib/n8n-studio/types";

const repoRoot = path.resolve(import.meta.dirname, "..");

function createOpenAiNode(credentials: WorkflowNode["credentials"]): WorkflowNode {
  return {
    id: "node-openai",
    name: "Summarize Readiness",
    type: "n8n-nodes-base.openAi",
    typeVersion: 1.1,
    position: { x: 100, y: 100 },
    parameters: {
      resource: "chat",
      operation: "complete",
    },
    credentials,
  };
}

test("n8n Studio builds selected-node credential picker state without secrets", () => {
  const readiness = createCredentialReadinessForNode(
    createOpenAiNode({
      openAiApi: {
        id: "cred-openai",
        name: "OpenAI production",
        data: {
          apiKey: "must-not-survive",
        },
      } as unknown as { id: string; name: string },
    }),
    getNodeTypeDescription("n8n-nodes-base.openAi"),
  );

  assert.equal(readiness.length, 1);
  assert.equal(readiness[0]?.nodeId, "node-openai");
  assert.equal(readiness[0]?.nodeName, "Summarize Readiness");
  assert.equal(readiness[0]?.credentialType, "openAiApi");
  assert.equal(readiness[0]?.credentialKey, "openAiApi");
  assert.equal(readiness[0]?.required, true);
  assert.equal(readiness[0]?.status, "configured");
  assert.equal(readiness[0]?.selectedCredentialId, "cred-openai");
  assert.equal(readiness[0]?.selectedCredentialName, "OpenAI production");
  assert.deepEqual(readiness[0]?.credentialOptions, [
    {
      id: "cred-openai",
      name: "OpenAI production",
      credentialType: "openAiApi",
      source: "workflow-reference",
      redaction: "secret-values-never-included",
    },
  ]);
  assert.equal(readiness[0]?.pickerBoundary.liveProviderExecution, false);
  assert.equal(readiness[0]?.pickerBoundary.secretsIncluded, false);
  assert.equal(JSON.stringify(readiness).includes("must-not-survive"), false);
  assert.equal(JSON.stringify(readiness).includes("apiKey"), false);
});

test("n8n Studio blocks required credentials that have no selected reference", () => {
  const readiness = createCredentialReadinessForNode(
    createOpenAiNode(undefined),
    getNodeTypeDescription("n8n-nodes-base.openAi"),
  );

  assert.equal(readiness[0]?.credentialType, "openAiApi");
  assert.equal(readiness[0]?.required, true);
  assert.equal(readiness[0]?.status, "blocked");
  assert.match(readiness[0]?.issue ?? "", /required credential/i);
  assert.equal(readiness[0]?.credentialOptions.length, 0);
});

test("n8n Studio state scopes credential readiness to the selected node", () => {
  const state = createN8nStudioState();

  assert.equal(state.canvas.selectedNodeId, "node-http-request");
  assert.equal(state.credentials.length > 0, true);
  assert.equal(
    state.credentials.every((credential) => credential.nodeId === state.canvas.selectedNodeId),
    true,
  );
  assert.equal(JSON.stringify(state.credentials).includes("accessToken"), false);
  assert.equal(JSON.stringify(state.credentials).includes("apiKey"), false);
});

test("n8n Studio credential panel renders picker data and redaction boundaries", () => {
  const panelSource = fs.readFileSync(
    path.join(
      repoRoot,
      "examples",
      "n8n-studio",
      "components",
      "n8n-studio",
      "credential-readiness-panel.tsx",
    ),
    "utf8",
  );

  assert.match(panelSource, /credentialOptions/);
  assert.match(panelSource, /data-credential-picker=/);
  assert.match(panelSource, /data-credential-provider-boundary=/);
  assert.match(panelSource, /secret-values-never-included/);
});

test("n8n Studio credential reference sanitizer keeps id and name only", () => {
  assert.deepEqual(
    sanitizeCredentialReference("SlackApi", {
      id: "cred-slack",
      name: "Slack",
      accessToken: "must-not-survive",
      oauthTokenData: {
        refreshToken: "must-not-survive",
      },
    }),
    {
      id: "cred-slack",
      name: "Slack",
      credentialType: "SlackApi",
      source: "workflow-reference",
      redaction: "secret-values-never-included",
    },
  );
});

const createCredentialReadinessForNode =
  credentialModel.createCredentialReadinessForNode as (
    node: WorkflowNode,
    description: ReturnType<typeof getNodeTypeDescription>,
  ) => import("../examples/n8n-studio/lib/n8n-studio/types").CredentialReadiness[];

const sanitizeCredentialReference = credentialModel.sanitizeCredentialReference as (
  credentialType: string,
  reference: unknown,
) => NonNullable<
  import("../examples/n8n-studio/lib/n8n-studio/types").CredentialReadiness["credentialOptions"]
>[number];
