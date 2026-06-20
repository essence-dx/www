import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import {
  applyParameterMutationToStudioState,
  applyParameterMutationToWorkflowNode,
} from "../examples/n8n-studio/lib/n8n-studio/parameter-mutations";
import { createSlackMessageNodeTypeFromSource } from "../examples/n8n-studio/lib/n8n-studio/source-parameters/slack-message";
import type {
  N8nStudioState,
  ParameterField,
  WorkflowNode,
} from "../examples/n8n-studio/lib/n8n-studio/types";

const repoRoot = path.resolve(import.meta.dirname, "..");
const slackMessageSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "Slack",
  "V2",
  "MessageDescription.ts",
);

function readSlackMessageSource() {
  return fs.readFileSync(slackMessageSourcePath, "utf8");
}

function createSlackNode(parameters: Record<string, unknown>): WorkflowNode {
  return {
    id: "node-slack",
    name: "Post Slack Message",
    type: "n8n-nodes-base.slack",
    typeVersion: 2,
    position: { x: 100, y: 100 },
    parameters: {
      resource: "message",
      operation: "post",
      messageType: "attachment",
      ...parameters,
    },
  };
}

function createState(node: WorkflowNode): N8nStudioState {
  return {
    catalog: {
      schema: "dx.n8n-studio.catalog",
      sourceManifestPath: "integrations/n8n-nodes-base/dx-node-source-.dx/build-cache/manifest.json",
      copiedFrom: "local-dx-source",
      nodeFolderCount: 0,
      nodeFileCount: 0,
      credentialFileCount: 0,
      catalogNodes: [],
    },
    document: {
      schemaVersion: 1,
      id: "workflow",
      projectId: "project",
      name: "Mutation fixture",
      active: false,
      nodes: [node],
      connections: [],
      tags: [],
      pinData: {},
      meta: {
        source: "dx-www-n8n-studio",
        liveProviderExecution: false,
      },
    },
    canvas: {
      viewport: { x: 0, y: 0, zoom: 1 },
      selectedNodeId: node.id,
      edgeMode: "main",
      nodes: [node],
      connections: [],
    },
    parameters: [],
    credentials: [],
    resourceLocator: {
      mode: "list",
      status: "source-only",
      query: "",
      issue: "fixture",
    },
    pinnedData: [],
    execution: {
      status: "blocked",
      providerBoundary: true,
      liveProviderExecution: false,
      availableActions: [],
      blockedReason: "fixture",
    },
    aiTools: {
      status: "source-only",
      focusedNodeIds: [],
      toolLifecycle: [],
    },
    importExport: {
      importSources: ["clipboard"],
      exportFormat: "n8n-workflow-json",
      sanitizedFields: [],
    },
    receipts: {
      schema: "dx.n8n-studio.receipts",
      receiptRoot: ".dx/receipts",
      providerBoundary: true,
      liveProviderExecution: false,
      redaction: "secret-values-never-included",
      surfaces: [],
    },
  };
}

function attachmentItems(node: WorkflowNode) {
  return node.parameters.attachments as Array<{
    fallback?: string;
    fields: {
      item: Array<{
        title: string;
        value: string;
        short: boolean;
      }>;
    };
  }>;
}

function createParameterField(
  type: ParameterField["type"] | "json",
  value: unknown = "",
): ParameterField {
  return {
    name: "fixture",
    label: "Fixture",
    type: type as ParameterField["type"],
    required: false,
    expressionEnabled: true,
    value,
    valuePath: ["fixture"],
  };
}

test("n8n Studio applies nested collection add update and remove mutations", () => {
  const node = createSlackNode({});

  const withAttachment = applyParameterMutationToWorkflowNode(node, {
    kind: "addCollectionItem",
    collectionPath: ["attachments"],
    item: {
      fallback: "Incident fallback",
      fields: {
        item: [
          {
            title: "Severity",
            value: "High",
            short: false,
          },
        ],
      },
    },
  });

  assert.deepEqual(withAttachment.parameters.attachments, [
    {
      fallback: "Incident fallback",
      fields: {
        item: [
          {
            title: "Severity",
            value: "High",
            short: false,
          },
        ],
      },
    },
  ]);

  const withUpdatedNestedValue = applyParameterMutationToWorkflowNode(withAttachment, {
    kind: "updateValue",
    valuePath: ["attachments", 0, "fields", "item", 0, "value"],
    value: "Critical",
  });

  assert.equal(attachmentItems(withUpdatedNestedValue)[0].fields.item[0].value, "Critical");
  assert.equal(attachmentItems(withAttachment)[0].fields.item[0].value, "High");

  const withoutAttachment = applyParameterMutationToWorkflowNode(withUpdatedNestedValue, {
    kind: "removeCollectionItem",
    collectionPath: ["attachments"],
    itemIndex: 0,
  });

  assert.deepEqual(withoutAttachment.parameters.attachments, []);
});

test("n8n Studio refreshes selected node parameter schema after collection mutations", () => {
  const sourceNodeType = createSlackMessageNodeTypeFromSource(readSlackMessageSource());
  const initialNode = createSlackNode({});
  const state = createState(initialNode);

  const nextState = applyParameterMutationToStudioState(
    state,
    initialNode.id,
    {
      kind: "addCollectionItem",
      collectionPath: ["attachments"],
      item: {
        fallback: "Schema fallback",
        color: "#439fe0",
      },
    },
    { [sourceNodeType.name]: sourceNodeType },
  );

  const attachments = nextState.parameters.find((field) => field.name === "attachments");
  assert.equal(
    attachments?.collectionItems?.[0]?.fields.find((field) => field.name === "fallback")?.value,
    "Schema fallback",
  );
  assert.equal(
    nextState.document.nodes[0].parameters.attachments,
    nextState.canvas.nodes[0].parameters.attachments,
  );
});

test("n8n Studio collection controls are callback-gated instead of dummy buttons", () => {
  const controlSource = fs.readFileSync(
    path.join(repoRoot, "examples", "n8n-studio", "components", "n8n-studio", "parameter-field-control.tsx"),
    "utf8",
  );

  assert.match(controlSource, /onAddCollectionItem/);
  assert.match(controlSource, /onRemoveCollectionItem/);
  assert.match(controlSource, /data-collection-action="add-item"/);
  assert.match(controlSource, /data-collection-action="remove-item"/);
});

test("n8n Studio shell wires selected-node parameter mutations into live client state", () => {
  const appSource = fs.readFileSync(
    path.join(repoRoot, "examples", "n8n-studio", "components", "n8n-studio", "n8n-studio-app.tsx"),
    "utf8",
  );
  const inspectorSource = fs.readFileSync(
    path.join(repoRoot, "examples", "n8n-studio", "components", "n8n-studio", "parameter-inspector.tsx"),
    "utf8",
  );

  assert.match(appSource, /"use client"/);
  assert.match(appSource, /useState/);
  assert.match(appSource, /applyParameterMutationToStudioState/);
  assert.match(appSource, /kind:\s*"addCollectionItem"/);
  assert.match(appSource, /kind:\s*"removeCollectionItem"/);
  assert.match(appSource, /kind:\s*"updateValue"/);
  assert.match(appSource, /onAddCollectionItem=/);
  assert.match(appSource, /onRemoveCollectionItem=/);
  assert.match(appSource, /onUpdateParameterValue=/);
  assert.match(inspectorSource, /onAddCollectionItem/);
  assert.match(inspectorSource, /onRemoveCollectionItem/);
  assert.match(inspectorSource, /onUpdateParameterValue/);
  assert.match(inspectorSource, /<ParameterFieldControl[\s\S]*onAddCollectionItem=/);
  assert.match(inspectorSource, /<ParameterFieldControl[\s\S]*onRemoveCollectionItem=/);
  assert.match(inspectorSource, /<ParameterFieldControl[\s\S]*onUpdateParameterValue=/);
});

test("n8n Studio coerces parameter input values by field type before mutation", async () => {
  const coercionModulePath = path.join(
    repoRoot,
    "examples",
    "n8n-studio",
    "lib",
    "n8n-studio",
    "parameter-value-coercion.ts",
  );
  const controlSource = fs.readFileSync(
    path.join(repoRoot, "examples", "n8n-studio", "components", "n8n-studio", "parameter-field-control.tsx"),
    "utf8",
  );

  assert.equal(fs.existsSync(coercionModulePath), true);

  const { coerceParameterInputValue } = await import(
    "../examples/n8n-studio/lib/n8n-studio/parameter-value-coercion"
  );

  assert.equal(coerceParameterInputValue(createParameterField("boolean"), true), true);
  assert.equal(coerceParameterInputValue(createParameterField("boolean"), "false"), false);
  assert.equal(coerceParameterInputValue(createParameterField("number"), "42.5"), 42.5);
  assert.equal(coerceParameterInputValue(createParameterField("number"), ""), undefined);
  assert.deepEqual(coerceParameterInputValue(createParameterField("json"), "{\"enabled\":true}"), {
    enabled: true,
  });
  assert.deepEqual(
    coerceParameterInputValue(
      createParameterField("resourceLocator"),
      "{\"mode\":\"list\",\"value\":\"C123\"}",
    ),
    {
      mode: "list",
      value: "C123",
    },
  );
  assert.match(controlSource, /coerceParameterInputValue\(field,/);
  assert.match(controlSource, /type="checkbox"/);
  assert.match(controlSource, /checked={field\.value === true}/);
  assert.match(controlSource, /inputMode={field\.type === "number" \? "decimal" : undefined}/);
});
