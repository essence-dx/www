import assert from "node:assert/strict";
import path from "node:path";
import test from "node:test";

import {
  createEditorSessionReadiness,
  createEditorSessionReadinessForNode,
} from "../examples/n8n-studio/lib/n8n-studio/editor-session-adapter";
import * as editorSessionAdapter from "../examples/n8n-studio/lib/n8n-studio/editor-session-adapter";
import { createReadinessResponse } from "../examples/n8n-studio/server/n8n-studio/readiness-response";
import { createStudioBootFromLocalGeneratedSource } from "../examples/n8n-studio/server/n8n-studio/generated-catalog-source";
import { createN8nStudioState } from "../examples/n8n-studio/lib/n8n-studio/studio-state";
import { studioWorkflowDocument } from "../examples/n8n-studio/lib/n8n-studio/workflow-document";
import type {
  EditorSessionCredentialTestTransportResponse,
  WorkflowNode,
} from "../examples/n8n-studio/lib/n8n-studio/types";

const repoRoot = path.resolve(import.meta.dirname, "..");

function slackMessageNode(): WorkflowNode {
  return {
    id: "node-slack-message",
    name: "Post Slack Message",
    type: "n8n-nodes-base.slack",
    typeVersion: 2,
    position: { x: 520, y: 240 },
    parameters: {
      resource: "message",
      operation: "post",
      select: "channel",
      channelId: {
        __rl: true,
        mode: "list",
        value: "C123",
        cachedResultName: "general",
      },
    },
    credentials: {
      slackApi: {
        id: "credential-slack",
        name: "Slack workspace",
        accessToken: "must-not-survive",
      } as unknown as { id: string; name: string },
    },
  };
}

function openAiChatNode(): WorkflowNode {
  return {
    id: "node-openai-chat",
    name: "Summarize Readiness",
    type: "n8n-nodes-base.openAi",
    typeVersion: 1.1,
    position: { x: 760, y: 240 },
    parameters: {
      resource: "chat",
      operation: "complete",
      chatModel: "gpt-4o-mini",
      prompt: {
        messages: [
          {
            role: "system",
            content: "Keep output short.",
          },
          {
            role: "user",
            content: "Summarize the readiness receipt.",
          },
        ],
      },
    },
    credentials: {
      openAiApi: {
        id: "credential-openai",
        name: "OpenAI API",
        apiKey: "must-not-survive",
      } as unknown as { id: string; name: string },
    },
  };
}

function googleSheetsAppendNode(): WorkflowNode {
  return {
    id: "node-google-sheets-append",
    name: "Append Google Sheet",
    type: "n8n-nodes-base.googleSheets",
    typeVersion: 4.7,
    position: { x: 980, y: 240 },
    parameters: {
      authentication: "oAuth2",
      resource: "sheet",
      operation: "append",
      documentId: { mode: "list", value: "spreadsheet-1" },
      sheetName: { mode: "list", value: "Sheet1" },
      columns: {
        mappingMode: "defineBelow",
        value: null,
      },
    },
    credentials: {
      googleSheetsOAuth2Api: {
        id: "credential-google-sheets",
        name: "Google Sheets OAuth",
        accessToken: "must-not-survive",
      } as unknown as { id: string; name: string },
    },
  };
}

function googleSheetsReadNode(): WorkflowNode {
  return {
    id: "node-google-sheets-read",
    name: "Read Google Sheet",
    type: "n8n-nodes-base.googleSheets",
    typeVersion: 4.7,
    position: { x: 980, y: 360 },
    parameters: {
      authentication: "oAuth2",
      resource: "sheet",
      operation: "read",
      documentId: { mode: "list", value: "spreadsheet-1" },
      sheetName: { mode: "list", value: "Sheet1" },
      filtersUI: {
        values: [
          {
            lookupColumn: "Email",
            lookupValue: "team@example.com",
          },
        ],
      },
    },
    credentials: {
      googleSheetsOAuth2Api: {
        id: "credential-google-sheets",
        name: "Google Sheets OAuth",
        accessToken: "must-not-survive",
      } as unknown as { id: string; name: string },
    },
  };
}

test("n8n Studio editor-session readiness models dynamic parameter and credential requests without live calls", () => {
  const readiness = createEditorSessionReadinessForNode(slackMessageNode());
  const serialized = JSON.stringify(readiness);

  assert.equal(readiness.schema, "dx.n8n-studio.editor-session");
  assert.equal(readiness.status, "blocked");
  assert.equal(readiness.providerBoundary, true);
  assert.equal(readiness.liveProviderExecution, false);
  assert.equal(readiness.secretsIncluded, false);
  assert.equal(readiness.selectedNodeId, "node-slack-message");
  assert.equal(readiness.nodeType, "n8n-nodes-base.slack");
  assert.equal(readiness.dynamicParameterLoadCount > 0, true);
  assert.equal(readiness.resourceLocatorSearchCount, 1);
  assert.equal(readiness.credentialRequestCount, 2);
  assert.equal(
    readiness.requestPlans.some(
      (plan) =>
        plan.kind === "resource-locator-search" &&
        plan.fieldName === "channelId" &&
        plan.loadMethod === "getChannels" &&
        plan.query === "general",
    ),
    true,
  );
  assert.equal(
    readiness.requestPlans.some(
      (plan) =>
        plan.kind === "credential-list" &&
        plan.credentialType === "SlackApi" &&
        plan.selectedCredentialId === "credential-slack",
    ),
    true,
  );
  assert.equal(
    readiness.requestPlans.every((plan) => plan.liveProviderExecution === false),
    true,
  );
  assert.equal(serialized.includes("must-not-survive"), false);
  assert.equal(serialized.includes("accessToken"), false);
  assert.equal(serialized.includes("apiKey"), false);
});

test("n8n Studio editor-session readiness models OpenAI dynamic model options without live calls", () => {
  const readiness = createEditorSessionReadinessForNode(openAiChatNode());
  const serialized = JSON.stringify(readiness);

  assert.equal(readiness.status, "blocked");
  assert.equal(readiness.nodeType, "n8n-nodes-base.openAi");
  assert.equal(readiness.providerBoundary, true);
  assert.equal(readiness.liveProviderExecution, false);
  assert.equal(readiness.secretsIncluded, false);
  assert.equal(readiness.dynamicParameterLoadCount, 1);
  assert.equal(readiness.credentialRequestCount, 1);
  assert.equal(readiness.credentialValidationRequestCount, 1);
  assert.equal(
    readiness.requestPlans.some(
      (plan) =>
        plan.kind === "dynamic-node-parameters" &&
        plan.fieldName === "chatModel" &&
        plan.loadMethod === "GET:/v1/models" &&
        plan.dynamicLoadBoundary ===
          "n8n-nodes-base.openAi.chatModel.GET:/v1/models",
    ),
    true,
  );
  assert.equal(
    readiness.requestPlans.some(
      (plan) =>
        plan.kind === "credential-list" &&
        plan.credentialType === "openAiApi" &&
        plan.selectedCredentialId === "credential-openai",
    ),
    true,
  );
  assert.equal(
    readiness.requestPlans.some(
      (plan) =>
        plan.kind === "credential-test" &&
        plan.credentialType === "openAiApi" &&
        plan.credentialKey === "openAiApi" &&
        plan.selectedCredentialId === "credential-openai" &&
        plan.providerBoundary === true &&
        plan.liveProviderExecution === false &&
        plan.secretsIncluded === false,
    ),
    true,
  );
  assert.equal(serialized.includes("must-not-survive"), false);
  assert.equal(serialized.includes("apiKey"), false);
});

test("n8n Studio editor-session readiness applies governed dynamic option transport responses", () => {
  assert.equal(
    typeof editorSessionAdapter.applyEditorSessionTransportResponses,
    "function",
  );

  const readiness = createEditorSessionReadinessForNode(openAiChatNode());
  const applied = editorSessionAdapter.applyEditorSessionTransportResponses(
    readiness,
    [
      {
        kind: "dynamic-node-parameters",
        nodeId: "node-openai-chat",
        nodeType: "n8n-nodes-base.openAi",
        fieldName: "chatModel",
        loadMethod: "GET:/v1/models",
        dynamicLoadBoundary: "n8n-nodes-base.openAi.chatModel.GET:/v1/models",
        options: [
          {
            name: "GPT-4o Mini",
            value: "gpt-4o-mini",
          },
          {
            name: "Secret Candidate",
            value: "must-not-survive",
            apiKey: "must-not-survive",
          } as unknown as { name: string; value: string },
        ],
        providerBoundary: true,
        liveProviderExecution: false,
        secretsIncluded: false,
        redaction: "secret-values-never-included",
      },
    ],
  );
  const dynamicPlan = applied.requestPlans.find(
    (plan) =>
      plan.kind === "dynamic-node-parameters" &&
      plan.dynamicLoadBoundary ===
        "n8n-nodes-base.openAi.chatModel.GET:/v1/models",
  );
  const serialized = JSON.stringify(applied);

  assert.equal(applied.status, "blocked");
  assert.equal(applied.fulfilledRequestCount, 1);
  assert.equal(dynamicPlan?.status, "configured");
  assert.equal(dynamicPlan?.resolvedOptionCount, 2);
  assert.equal(dynamicPlan?.resolvedOptions?.[0]?.value, "gpt-4o-mini");
  assert.equal(dynamicPlan?.liveProviderExecution, false);
  assert.equal(dynamicPlan?.secretsIncluded, false);
  assert.equal(serialized.includes("apiKey"), false);
});

test("n8n Studio editor-session readiness applies governed resource locator search responses", () => {
  const readiness = createEditorSessionReadinessForNode(slackMessageNode());
  const applied = editorSessionAdapter.applyEditorSessionTransportResponses(
    readiness,
    [
      {
        kind: "resource-locator-search",
        nodeId: "node-slack-message",
        nodeType: "n8n-nodes-base.slack",
        fieldName: "channelId",
        loadMethod: "getChannels",
        query: "general",
        nextPageToken: "cursor-page-2",
        options: [
          {
            name: "#general",
            value: "C123",
            description: "General workspace channel",
          },
          {
            name: "#private",
            value: "C999",
            secret: "must-not-survive",
          } as unknown as { name: string; value: string },
        ],
        providerBoundary: true,
        liveProviderExecution: false,
        secretsIncluded: false,
        redaction: "secret-values-never-included",
      },
    ],
  );
  const locatorPlan = applied.requestPlans.find(
    (plan) =>
      plan.kind === "resource-locator-search" &&
      plan.fieldName === "channelId",
  );
  const serialized = JSON.stringify(applied);

  assert.equal(applied.status, "blocked");
  assert.equal(applied.fulfilledRequestCount, 1);
  assert.equal(locatorPlan?.status, "configured");
  assert.equal(locatorPlan?.resolvedQuery, "general");
  assert.equal(locatorPlan?.nextPageToken, "cursor-page-2");
  assert.equal(locatorPlan?.resolvedOptionCount, 2);
  assert.equal(
    locatorPlan?.resolvedOptions?.[0]?.description,
    "General workspace channel",
  );
  assert.equal(serialized.includes("\"secret\""), false);
  assert.equal(serialized.includes("must-not-survive"), false);
});

test("n8n Studio editor-session readiness models resource mapper schema requests without live calls", () => {
  const boot = createStudioBootFromLocalGeneratedSource(repoRoot);
  const readiness = createEditorSessionReadinessForNode(
    googleSheetsAppendNode(),
    boot.nodeTypeRegistry,
  );
  const response = createReadinessResponse({
    ...boot.state,
    editorSession: readiness,
  });
  const serialized = JSON.stringify(readiness);

  assert.equal(readiness.status, "blocked");
  assert.equal(readiness.nodeType, "n8n-nodes-base.googleSheets");
  assert.equal(readiness.providerBoundary, true);
  assert.equal(readiness.liveProviderExecution, false);
  assert.equal(readiness.secretsIncluded, false);
  assert.equal(readiness.resourceMapperRequestCount, 1);
  assert.equal(
    readiness.requestPlans.some(
      (plan) =>
        plan.kind === "resource-mapper-schema" &&
        plan.fieldName === "columns" &&
        plan.loadMethod === "getMappingColumns" &&
        plan.dynamicLoadBoundary ===
          "n8n-nodes-base.googleSheets.columns.getMappingColumns",
    ),
    true,
  );
  assert.equal(response.editorSession.resourceMapperRequestCount, 1);
  assert.equal(serialized.includes("must-not-survive"), false);
  assert.equal(serialized.includes("accessToken"), false);
});

test("n8n Studio editor-session readiness applies governed resource mapper schema responses", () => {
  const boot = createStudioBootFromLocalGeneratedSource(repoRoot);
  const readiness = createEditorSessionReadinessForNode(
    googleSheetsAppendNode(),
    boot.nodeTypeRegistry,
  );
  const applied = editorSessionAdapter.applyEditorSessionTransportResponses(
    readiness,
    [
      {
        kind: "resource-mapper-schema",
        nodeId: "node-google-sheets-append",
        nodeType: "n8n-nodes-base.googleSheets",
        fieldName: "columns",
        loadMethod: "getMappingColumns",
        dynamicLoadBoundary:
          "n8n-nodes-base.googleSheets.columns.getMappingColumns",
        schema: {
          fields: [
            {
              id: "Name",
              displayName: "Name",
              required: true,
              canBeUsedToMatch: true,
              type: "string",
            },
            {
              id: "Secret",
              displayName: "Secret",
              credentialValue: "must-not-survive",
            } as unknown as { id: string; displayName: string },
          ],
          fieldWords: {
            singular: "column",
            plural: "columns",
          },
        },
        providerBoundary: true,
        liveProviderExecution: false,
        secretsIncluded: false,
        redaction: "secret-values-never-included",
      },
    ],
  );
  const mapperPlan = applied.requestPlans.find(
    (plan) =>
      plan.kind === "resource-mapper-schema" &&
      plan.dynamicLoadBoundary ===
        "n8n-nodes-base.googleSheets.columns.getMappingColumns",
  );
  const serialized = JSON.stringify(applied);

  assert.equal(applied.status, "blocked");
  assert.equal(applied.fulfilledRequestCount, 1);
  assert.equal(mapperPlan?.status, "configured");
  assert.equal(mapperPlan?.resolvedFieldCount, 2);
  assert.equal(mapperPlan?.resolvedSchema?.fields[0]?.id, "Name");
  assert.equal(mapperPlan?.resolvedSchema?.fields[0]?.required, true);
  assert.equal(mapperPlan?.resolvedSchema?.fieldWords?.plural, "columns");
  assert.equal(serialized.includes("credentialValue"), false);
  assert.equal(serialized.includes("must-not-survive"), false);
});

test("n8n Studio editor-session readiness applies governed credential list responses", () => {
  const readiness = createEditorSessionReadinessForNode(openAiChatNode());
  const applied = editorSessionAdapter.applyEditorSessionTransportResponses(
    readiness,
    [
      {
        kind: "credential-list",
        nodeId: "node-openai-chat",
        nodeType: "n8n-nodes-base.openAi",
        credentialType: "openAiApi",
        credentialKey: "openAiApi",
        selectedCredentialId: "credential-openai-secondary",
        selectedCredentialName: "Backup OpenAI",
        credentialOptions: [
          {
            id: "credential-openai",
            name: "OpenAI API",
            credentialType: "openAiApi",
            source: "workflow-reference",
            redaction: "secret-values-never-included",
            apiKey: "must-not-survive",
          } as unknown as {
            id: string;
            name: string;
            credentialType: string;
            source: "workflow-reference";
            redaction: "secret-values-never-included";
          },
          {
            id: "credential-openai-secondary",
            name: "Backup OpenAI",
            credentialType: "openAiApi",
            source: "editor-session-placeholder",
            redaction: "secret-values-never-included",
          },
        ],
        providerBoundary: true,
        liveProviderExecution: false,
        secretsIncluded: false,
        redaction: "secret-values-never-included",
      },
    ],
  );
  const credentialPlan = applied.requestPlans.find(
    (plan) =>
      plan.kind === "credential-list" &&
      plan.credentialType === "openAiApi",
  );
  const serialized = JSON.stringify(applied);

  assert.equal(applied.status, "blocked");
  assert.equal(applied.fulfilledRequestCount, 1);
  assert.equal(credentialPlan?.status, "configured");
  assert.equal(
    credentialPlan?.selectedCredentialId,
    "credential-openai-secondary",
  );
  assert.equal(credentialPlan?.selectedCredentialName, "Backup OpenAI");
  assert.equal(credentialPlan?.resolvedCredentialOptionCount, 2);
  assert.equal(
    credentialPlan?.resolvedCredentialOptions?.[0]?.name,
    "OpenAI API",
  );
  assert.equal(
    credentialPlan?.resolvedCredentialOptions?.[0]?.redaction,
    "secret-values-never-included",
  );
  assert.equal(serialized.includes("apiKey"), false);
  assert.equal(serialized.includes("must-not-survive"), false);
});

test("n8n Studio editor-session readiness applies governed credential validation responses", () => {
  const readiness = createEditorSessionReadinessForNode(openAiChatNode());
  const validationResponse: EditorSessionCredentialTestTransportResponse & {
    apiKey: string;
  } = {
    kind: "credential-test",
    nodeId: "node-openai-chat",
    nodeType: "n8n-nodes-base.openAi",
    credentialType: "openAiApi",
    credentialKey: "openAiApi",
    selectedCredentialId: "credential-openai",
    validationStatus: "valid",
    validatedAt: "2026-06-09T00:00:00.000Z",
    message: "Credential verified by the governed editor-session bridge.",
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    redaction: "secret-values-never-included",
    apiKey: "must-not-survive",
  };
  const applied = editorSessionAdapter.applyEditorSessionTransportResponses(
    readiness,
    [validationResponse],
  );
  const validationPlan = applied.requestPlans.find(
    (plan) =>
      plan.kind === "credential-test" &&
      plan.credentialType === "openAiApi",
  );
  const serialized = JSON.stringify(applied);

  assert.equal(applied.status, "blocked");
  assert.equal(applied.fulfilledRequestCount, 1);
  assert.equal(validationPlan?.status, "configured");
  assert.equal(validationPlan?.credentialValidationStatus, "valid");
  assert.equal(
    validationPlan?.credentialValidatedAt,
    "2026-06-09T00:00:00.000Z",
  );
  assert.equal(
    validationPlan?.credentialValidationMessage,
    "Credential verified by the governed editor-session bridge.",
  );
  assert.equal(validationPlan?.liveProviderExecution, false);
  assert.equal(validationPlan?.secretsIncluded, false);
  assert.equal(serialized.includes("apiKey"), false);
  assert.equal(serialized.includes("must-not-survive"), false);
});

test("n8n Studio editor-session readiness preserves nested dynamic parameter paths", () => {
  const boot = createStudioBootFromLocalGeneratedSource(repoRoot);
  const readiness = createEditorSessionReadinessForNode(
    googleSheetsReadNode(),
    boot.nodeTypeRegistry,
  );
  const serialized = JSON.stringify(readiness);

  assert.equal(readiness.status, "blocked");
  assert.equal(readiness.nodeType, "n8n-nodes-base.googleSheets");
  assert.equal(readiness.dynamicParameterLoadCount, 4);
  assert.equal(
    readiness.requestPlans.some(
      (plan) =>
        plan.kind === "dynamic-node-parameters" &&
        plan.fieldName === "lookupColumn" &&
        plan.loadMethod === "getSheetHeaderRowWithGeneratedColumnNames" &&
        plan.dynamicLoadBoundary ===
          "n8n-nodes-base.googleSheets.filtersUI.values.0.lookupColumn.getSheetHeaderRowWithGeneratedColumnNames",
    ),
    true,
  );
  assert.equal(serialized.includes("must-not-survive"), false);
  assert.equal(serialized.includes("accessToken"), false);
});

test("n8n Studio editor-session readiness is projected onto the Studio state and bridge response", () => {
  const state = createN8nStudioState();
  const response = createReadinessResponse();

  assert.equal(state.editorSession.schema, "dx.n8n-studio.editor-session");
  assert.equal(state.editorSession.selectedNodeId, state.canvas.selectedNodeId);
  assert.equal(state.editorSession.liveProviderExecution, false);
  assert.equal(response.editorSession.status, state.editorSession.status);
  assert.equal(response.editorSession.selectedNodeId, state.editorSession.selectedNodeId);
  assert.equal(response.editorSession.secretsIncluded, false);
});

test("n8n Studio editor-session readiness blocks missing selected nodes explicitly", () => {
  const readiness = createEditorSessionReadiness(
    studioWorkflowDocument,
    "missing-node",
  );

  assert.equal(readiness.status, "blocked");
  assert.equal(readiness.selectedNodeId, "missing-node");
  assert.equal(readiness.requestPlans.length, 0);
  assert.match(readiness.issue, /Selected node is not available/);
});
