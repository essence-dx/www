import { createCanvasInteractionState } from "./canvas-interaction-state";
import type { CanvasProjection, WorkflowDocument } from "./types";

export const studioWorkflowDocument: WorkflowDocument = {
  schemaVersion: 1,
  id: "dx-n8n-studio-workflow",
  projectId: "local-dx-workspace",
  name: "DX Catalog Readiness",
  active: false,
  tags: ["dx-www", "n8n", "source-owned"],
  nodes: [
    {
      id: "node-manual-trigger",
      name: "Manual Trigger",
      type: "n8n-nodes-base.manualTrigger",
      typeVersion: 1,
      position: { x: 84, y: 208 },
      parameters: {},
      notes: "Source-owned trigger node discovered from the local n8n manifest.",
    },
    {
      id: "node-http-request",
      name: "Read Connector Manifest",
      type: "n8n-nodes-base.httpRequest",
      typeVersion: 4,
      position: { x: 408, y: 208 },
      parameters: {
        method: "GET",
        url: "={{ $json.manifestUrl }}",
        authentication: "genericCredentialType",
      },
      credentials: {
        httpBearerAuth: {
          id: "credential-id-only",
          name: "DX Bearer Token",
        },
      },
      notes: "Credential reference stores id and display name only; no secret value is mirrored.",
    },
    {
      id: "node-openai",
      name: "Summarize Readiness",
      type: "n8n-nodes-base.openAi",
      typeVersion: 1,
      position: { x: 760, y: 208 },
      parameters: {
        resource: "chat",
        operation: "complete",
        chatModel: "gpt-3.5-turbo",
        prompt: {
          messages: [
            {
              role: "system",
              content: "Summarize DX n8n connector readiness without exposing secrets.",
            },
            {
              role: "user",
              content: "={{ $json.connectorSummary }}",
            },
          ],
        },
        simplifyOutput: true,
      },
      credentials: {
        openAiApi: {
          id: "credential-id-only",
          name: "OpenAI API",
        },
      },
      notes: "AI node is modeled as an editor state and remains blocked from live execution.",
    },
  ],
  connections: [
    {
      id: "edge-manual-to-http",
      sourceNode: "Manual Trigger",
      targetNode: "Read Connector Manifest",
      sourceOutput: "main",
      targetInput: "main",
      index: 0,
    },
    {
      id: "edge-http-to-openai",
      sourceNode: "Read Connector Manifest",
      targetNode: "Summarize Readiness",
      sourceOutput: "main",
      targetInput: "main",
      index: 0,
    },
  ],
  pinData: {
    "Read Connector Manifest": [
      {
        manifestUrl: "file://integrations/n8n-nodes-base/dx-node-source-.dx/build-cache/manifest.json",
        connectorSummary: "536 node files and 396 credential files available for source-owned readiness.",
      },
    ],
  },
  meta: {
    source: "dx-www-n8n-studio",
    liveProviderExecution: false,
  },
};

function defaultSelectedNodeId(document: WorkflowDocument) {
  return (
    document.nodes.find(
      (node) =>
        !node.type.toLowerCase().includes("trigger") &&
        (Object.keys(node.parameters).length > 0 ||
          Object.keys(node.credentials ?? {}).length > 0),
    )?.id ??
    document.nodes[0]?.id ??
    ""
  );
}

export function createCanvasProjection(
  document: WorkflowDocument,
  selectedNodeId: string = defaultSelectedNodeId(document),
): CanvasProjection {

  return {
    viewport: { x: 0, y: 0, zoom: 0.9 },
    selectedNodeId,
    edgeMode: "main",
    nodes: document.nodes,
    connections: document.connections,
    interaction: createCanvasInteractionState(document, selectedNodeId),
  };
}
