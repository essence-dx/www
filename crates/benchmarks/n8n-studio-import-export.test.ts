import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { pathToFileURL } from "node:url";

import {
  createExportReceiptDetail,
  createImportExportState,
  createImportPreviewState,
  sanitizeImportedWorkflow,
} from "../examples/n8n-studio/lib/n8n-studio/import-export";
import { createGeneratedNodeTypeRegistry } from "../examples/n8n-studio/lib/n8n-studio/generated-connectors/index";
import {
  createExportResponse,
  createExportResponseFromLocalGeneratedSource,
  createExportResponseFromPayload,
} from "../examples/n8n-studio/server/n8n-studio/export-response";
import { createN8nStudioState } from "../examples/n8n-studio/lib/n8n-studio/studio-state";
import { studioWorkflowDocument } from "../examples/n8n-studio/lib/n8n-studio/workflow-document";
import type { WorkflowDocument } from "../examples/n8n-studio/lib/n8n-studio/types";

const repoRoot = path.resolve(import.meta.dirname, "..");
const generatedCatalogPath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "generated",
  "dx-automations-connectors.json",
);

function readGeneratedCatalog() {
  return JSON.parse(fs.readFileSync(generatedCatalogPath, "utf8"));
}

function importedWorkflowWithUnsafeData() {
  return {
    name: "Imported unsafe workflow",
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
      Unknown: {
        main: [[{ node: "Known HTTP", type: "main", index: 0 }]],
      },
    },
    pinData: {
      "Known HTTP": [{ ok: true }],
      Unknown: [{ leak: true }],
    },
    meta: {
      webhookId: "external-webhook-id",
    },
  };
}

test("n8n Studio creates sanitized import preview state without secrets", () => {
  const preview = createImportPreviewState(importedWorkflowWithUnsafeData(), "clipboard");

  assert.equal(preview.status, "sanitized-with-issues");
  assert.equal(preview.source, "clipboard");
  assert.equal(preview.workflowName, "Imported unsafe workflow");
  assert.equal(preview.keptNodeCount, 1);
  assert.equal(preview.connectionCount, 0);
  assert.equal(preview.pinDataNodeCount, 1);
  assert.equal(preview.droppedIssueCount >= 2, true);
  assert.equal(preview.strippedSecretCount >= 2, true);
  assert.equal(preview.regeneratedWebhookCount, 1);
  assert.equal(preview.boundary.providerBoundary, true);
  assert.equal(preview.boundary.liveProviderExecution, false);
  assert.equal(preview.boundary.secretsIncluded, false);
  assert.equal(preview.boundary.executableAfterImport, false);
  assert.equal(preview.issues.every((issue) => issue.action.length > 0), true);
  assert.equal(JSON.stringify(preview).includes("must-not-survive"), false);
});

test("n8n Studio import/export session keeps empty import honest and export receipt detailed", () => {
  const state = createImportExportState(studioWorkflowDocument);

  assert.equal(state.importPreview.status, "awaiting-input");
  assert.equal(state.importPreview.boundary.liveProviderExecution, false);
  assert.equal(state.importPreview.sanitizedDocument, undefined);
  assert.equal(state.exportReceipt.schema, "dx.n8n-studio.export.receipt");
  assert.equal(state.exportReceipt.workflowName, studioWorkflowDocument.name);
  assert.equal(state.exportReceipt.nodeCount, studioWorkflowDocument.nodes.length);
  assert.equal(state.exportReceipt.connectionCount, studioWorkflowDocument.connections.length);
  assert.equal(state.exportReceipt.credentialReferenceCount, 2);
  assert.equal(state.exportReceipt.routePath, "/api/n8n-studio/export");
  assert.equal(state.exportReceipt.secretsIncluded, false);
  assert.equal(state.currentExport.status, "idle");
  assert.equal(state.currentExport.routePath, "/api/n8n-studio/export");
  assert.equal(state.currentExport.secretsIncluded, false);
});

test("n8n Studio can attach a sanitized import preview to the session model", () => {
  const preview = createImportPreviewState(importedWorkflowWithUnsafeData(), "file");
  const state = createImportExportState(studioWorkflowDocument, preview);

  assert.equal(state.importPreview.source, "file");
  assert.equal(state.importPreview.sanitizedDocument?.nodes.length, 1);
  assert.equal(state.importPreview.issues.some((issue) => issue.code === "unknown-node-type"), true);
  assert.equal(state.importPreview.issues.some((issue) => issue.severity === "blocker"), true);
});

test("n8n Studio import sanitizer can keep generated-registry nodes", () => {
  const generated = createGeneratedNodeTypeRegistry(readGeneratedCatalog());
  const result = sanitizeImportedWorkflow(
    {
      name: "Generated Gmail workflow",
      nodes: [
        {
          id: "manual",
          name: "Manual Trigger",
          type: "n8n-nodes-base.manualTrigger",
          position: [80, 200],
          parameters: {},
        },
        {
          id: "gmail",
          name: "Send Gmail",
          type: "n8n-nodes-base.gmail",
          typeVersion: 2.2,
          position: [380, 200],
          parameters: {
            resource: "message",
            operation: "send",
            sendTo: "team@example.com",
            apiKey: "must-not-survive",
          },
        },
      ],
      connections: {
        "Manual Trigger": {
          main: [[{ node: "Send Gmail", type: "main", index: 0 }]],
        },
      },
      pinData: {
        "Send Gmail": [{ id: "message-1" }],
      },
    },
    generated.registry,
  );

  assert.equal(result.document.nodes.length, 2);
  assert.equal(result.document.nodes.some((node) => node.type === "n8n-nodes-base.gmail"), true);
  assert.equal(result.document.connections.length, 1);
  assert.equal(result.document.pinData["Send Gmail"]?.length, 1);
  assert.equal(result.issues.some((issue) => issue.code === "unknown-node-type"), false);
  assert.equal(result.issues.some((issue) => issue.code === "parameter-secret-stripped"), true);
  assert.equal(JSON.stringify(result.document).includes("must-not-survive"), false);
});

test("n8n Studio export route returns export receipt metadata with the workflow", () => {
  const receipt = createExportReceiptDetail(studioWorkflowDocument);
  const response = createExportResponse();

  assert.equal(response.receipt.schema, receipt.schema);
  assert.equal(response.receipt.nodeCount, receipt.nodeCount);
  assert.equal(response.receipt.providerBoundary, true);
  assert.equal(response.receipt.liveProviderExecution, false);
  assert.equal(response.receipt.secretsIncluded, false);
  assert.equal(JSON.stringify(response).includes("secret-values-never-included"), true);
});

test("n8n Studio export route can use generated catalog boot data", () => {
  const response = createExportResponseFromLocalGeneratedSource(repoRoot);
  const routeSource = fs.readFileSync(
    path.join(repoRoot, "examples", "n8n-studio", "app", "api", "n8n-studio", "export", "route.ts"),
    "utf8",
  );

  assert.equal(response.generatedMetadata?.sourceAvailable, true);
  assert.equal(response.catalogNodeCount >= 470, true);
  assert.equal(response.receipt.liveProviderExecution, false);
  assert.equal(response.workflow.nodes.length, studioWorkflowDocument.nodes.length);
  assert.match(routeSource, /createExportResponseFromLocalGeneratedSource/);
});

test("n8n Studio export route can export the current edited workflow document", async () => {
  const editedDocument: WorkflowDocument = {
    ...studioWorkflowDocument,
    nodes: [
      ...studioWorkflowDocument.nodes,
      {
        id: "node-edited-webhook",
        name: "Edited Webhook",
        type: "n8n-nodes-base.webhook",
        typeVersion: 2,
        position: { x: 1080, y: 260 },
        parameters: {
          path: "dx-edited-webhook",
          apiKey: "must-not-survive",
        },
      },
    ],
    connections: [
      ...studioWorkflowDocument.connections,
      {
        id: "edge-openai-to-edited-webhook",
        sourceNode: "Summarize Readiness",
        targetNode: "Edited Webhook",
        sourceOutput: "main",
        targetInput: "main",
        index: 0,
      },
    ],
  };
  const helperResponse = createExportResponseFromPayload({
    document: editedDocument,
  });
  const routePath = path.join(
    repoRoot,
    "examples",
    "n8n-studio",
    "app",
    "api",
    "n8n-studio",
    "export",
    "route.ts",
  );
  const route = await import(pathToFileURL(routePath).href);

  assert.equal(typeof route.POST, "function");
  assert.equal(helperResponse.workflow.nodes.length, editedDocument.nodes.length);
  assert.equal(
    helperResponse.workflow.nodes.some((node) => node.name === "Edited Webhook"),
    true,
  );
  assert.equal(
    helperResponse.workflow.connections["Summarize Readiness"].main[0].some(
      (connection) => connection.node === "Edited Webhook",
    ),
    true,
  );
  assert.equal(helperResponse.receipt.nodeCount, editedDocument.nodes.length);
  assert.equal(helperResponse.liveProviderExecution, false);
  assert.equal(JSON.stringify(helperResponse).includes("must-not-survive"), false);

  const httpResponse = await route.POST(
    new Request("http://localhost/api/n8n-studio/export", {
      method: "POST",
      body: JSON.stringify({ document: editedDocument }),
    }),
  );
  const httpBody = await httpResponse.json();

  assert.equal(httpBody.workflow.nodes.length, editedDocument.nodes.length);
  assert.equal(
    httpBody.workflow.nodes.some((node: { name: string }) => node.name === "Edited Webhook"),
    true,
  );
  assert.equal(JSON.stringify(httpBody).includes("must-not-survive"), false);
});

test("n8n Studio import/export panel renders preview and receipt contract markers", () => {
  const panelSource = fs.readFileSync(
    path.join(
      repoRoot,
      "examples",
      "n8n-studio",
      "components",
      "n8n-studio",
      "import-export-panel.tsx",
    ),
    "utf8",
  );
  const state = createN8nStudioState();

  assert.equal(state.importExport.importPreview.status, "awaiting-input");
  assert.match(panelSource, /data-import-preview-status=/);
  assert.match(panelSource, /data-import-sanitized-field=/);
  assert.match(panelSource, /data-import-issue-code=/);
  assert.match(panelSource, /data-export-receipt-schema=/);
  assert.match(panelSource, /data-export-live-provider-execution=/);
});

test("n8n Studio app wires current workflow export to the POST route", () => {
  const panelSource = fs.readFileSync(
    path.join(
      repoRoot,
      "examples",
      "n8n-studio",
      "components",
      "n8n-studio",
      "import-export-panel.tsx",
    ),
    "utf8",
  );
  const appSource = fs.readFileSync(
    path.join(
      repoRoot,
      "examples",
      "n8n-studio",
      "components",
      "n8n-studio",
      "n8n-studio-app.tsx",
    ),
    "utf8",
  );

  assert.match(panelSource, /onExportCurrentWorkflow/);
  assert.match(panelSource, /data-export-action="current-workflow"/);
  assert.match(panelSource, /data-current-export-status=/);
  assert.match(appSource, /handleExportCurrentWorkflow/);
  assert.match(appSource, /fetch\("\/api\/n8n-studio\/export"/);
  assert.match(appSource, /JSON\.stringify\(\{\s*document: exportDocument\s*\}\)/s);
  assert.match(appSource, /applyCurrentWorkflowExportResponseToStudioState/);
});
