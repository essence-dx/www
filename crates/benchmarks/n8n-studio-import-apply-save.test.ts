import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import {
  applyImportExportActionToStudioState,
} from "../examples/n8n-studio/lib/n8n-studio/import-export-actions";
import { createGeneratedNodeTypeRegistry } from "../examples/n8n-studio/lib/n8n-studio/generated-connectors/index";
import { createN8nStudioState } from "../examples/n8n-studio/lib/n8n-studio/studio-state";

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

function importedWorkflowForDraft() {
  return {
    name: "Imported production workflow",
    active: true,
    tags: ["imported", "draft"],
    nodes: [
      {
        id: "imported-manual",
        name: "Imported Manual",
        type: "n8n-nodes-base.manualTrigger",
        typeVersion: 1,
        position: [80, 160],
        parameters: {},
      },
      {
        id: "imported-http",
        name: "Imported HTTP",
        type: "n8n-nodes-base.httpRequest",
        typeVersion: 4,
        position: [360, 160],
        parameters: {
          method: "POST",
          url: "https://example.com/webhook",
        },
        credentials: {
          httpBearerAuth: {
            id: "imported-credential-id",
            name: "Imported Bearer",
          },
        },
      },
    ],
    connections: {
      "Imported Manual": {
        main: [[{ node: "Imported HTTP", type: "main", index: 0 }]],
      },
    },
    pinData: {
      "Imported HTTP": [{ ok: true }],
    },
  };
}

function importedWorkflowWithBlocker() {
  return {
    name: "Blocked imported workflow",
    nodes: [
      {
        id: "unknown-node",
        name: "Unknown Node",
        type: "n8n-nodes-base.unknown",
        typeVersion: 1,
        position: [100, 100],
        parameters: {},
      },
    ],
    connections: {},
    pinData: {},
  };
}

function generatedGmailWorkflowForDraft() {
  return {
    name: "Generated Gmail draft",
    nodes: [
      {
        id: "generated-manual",
        name: "Generated Manual",
        type: "n8n-nodes-base.manualTrigger",
        typeVersion: 1,
        position: [80, 160],
        parameters: {},
      },
      {
        id: "generated-gmail",
        name: "Generated Gmail",
        type: "n8n-nodes-base.gmail",
        typeVersion: 2.2,
        position: [360, 160],
        parameters: {
          resource: "message",
          operation: "send",
          sendTo: "team@example.com",
        },
      },
    ],
    connections: {
      "Generated Manual": {
        main: [[{ node: "Generated Gmail", type: "main", index: 0 }]],
      },
    },
    pinData: {
      "Generated Gmail": [{ id: "message-1" }],
    },
  };
}

test("n8n Studio applies a sanitized import preview into the real editor state", () => {
  const initialState = createN8nStudioState();
  const withPreview = applyImportExportActionToStudioState(initialState, {
    kind: "loadImportPreview",
    source: "clipboard",
    importedWorkflow: importedWorkflowForDraft(),
  });
  const applied = applyImportExportActionToStudioState(withPreview, {
    kind: "applyImportPreview",
    appliedAt: "2026-06-09T10:00:00.000Z",
  });

  assert.equal(withPreview.document.name, initialState.document.name);
  assert.equal(withPreview.importExport.importPreview.status, "sanitized");
  assert.equal(withPreview.importExport.draft.status, "ready-to-apply");
  assert.equal(withPreview.importExport.draft.canApplyPreview, true);
  assert.equal(withPreview.importExport.draft.canSaveDraft, false);

  assert.equal(applied.document.name, "Imported production workflow");
  assert.equal(applied.document.active, false);
  assert.equal(applied.document.nodes.length, 2);
  assert.equal(applied.canvas.nodes, applied.document.nodes);
  assert.equal(applied.canvas.connections, applied.document.connections);
  assert.equal(applied.canvas.selectedNodeId, "imported-http");
  assert.equal(applied.parameters.length > 0, true);
  assert.equal(
    applied.credentials.some(
      (credential) => credential.selectedCredentialId === "imported-credential-id",
    ),
    true,
  );
  assert.equal(applied.pinnedData[0]?.nodeName, "Imported HTTP");
  assert.equal(applied.execution.attempts[0]?.workflowName, "Imported production workflow");
  assert.equal(applied.importExport.exportReceipt.workflowName, "Imported production workflow");
  assert.equal(applied.importExport.draft.status, "applied");
  assert.equal(applied.importExport.draft.appliedAt, "2026-06-09T10:00:00.000Z");
  assert.equal(applied.importExport.draft.canApplyPreview, false);
  assert.equal(applied.importExport.draft.canSaveDraft, true);
  assert.equal(JSON.stringify(applied).includes("imported-credential-id"), true);
  assert.equal(JSON.stringify(applied).includes("must-not-survive"), false);
});

test("n8n Studio blocks applying previews that still contain blocker issues", () => {
  const initialState = createN8nStudioState();
  const withPreview = applyImportExportActionToStudioState(initialState, {
    kind: "loadImportPreview",
    source: "file",
    importedWorkflow: importedWorkflowWithBlocker(),
  });
  const blocked = applyImportExportActionToStudioState(withPreview, {
    kind: "applyImportPreview",
    appliedAt: "2026-06-09T10:05:00.000Z",
  });

  assert.equal(withPreview.importExport.importPreview.status, "sanitized-with-issues");
  assert.equal(withPreview.importExport.draft.status, "blocked");
  assert.equal(withPreview.importExport.draft.canApplyPreview, false);
  assert.equal(blocked.document.name, initialState.document.name);
  assert.equal(blocked.importExport.draft.status, "blocked");
  assert.match(blocked.importExport.draft.issue, /blocker sanitation/i);
});

test("n8n Studio applies generated-registry import previews with selected surfaces", () => {
  const generated = createGeneratedNodeTypeRegistry(readGeneratedCatalog());
  const withPreview = applyImportExportActionToStudioState(
    createN8nStudioState(),
    {
      kind: "loadImportPreview",
      source: "clipboard",
      importedWorkflow: generatedGmailWorkflowForDraft(),
    },
    generated.registry,
  );
  const applied = applyImportExportActionToStudioState(
    withPreview,
    {
      kind: "applyImportPreview",
      appliedAt: "2026-06-09T10:07:00.000Z",
    },
    generated.registry,
  );

  assert.equal(withPreview.importExport.importPreview.status, "sanitized");
  assert.equal(withPreview.importExport.importPreview.keptNodeCount, 2);
  assert.equal(applied.document.name, "Generated Gmail draft");
  assert.equal(applied.canvas.selectedNodeId, "generated-gmail");
  assert.equal(applied.parameters.some((field) => field.name === "resource"), true);
  assert.equal(applied.parameters.some((field) => field.name === "operation"), true);
  assert.equal(
    applied.credentials.some((credential) => credential.credentialType === "gmailOAuth2"),
    true,
  );
  assert.equal(applied.editorSession.nodeType, "n8n-nodes-base.gmail");
  assert.equal(applied.pinnedData[0]?.nodeName, "Generated Gmail");
});

test("n8n Studio saves an applied imported draft through a source-only receipt boundary", () => {
  const withPreview = applyImportExportActionToStudioState(createN8nStudioState(), {
    kind: "loadImportPreview",
    source: "url",
    importedWorkflow: importedWorkflowForDraft(),
  });
  const notReady = applyImportExportActionToStudioState(withPreview, {
    kind: "saveImportedDraft",
    savedAt: "2026-06-09T10:09:00.000Z",
  });
  const applied = applyImportExportActionToStudioState(withPreview, {
    kind: "applyImportPreview",
    appliedAt: "2026-06-09T10:10:00.000Z",
  });
  const saved = applyImportExportActionToStudioState(applied, {
    kind: "saveImportedDraft",
    savedAt: "2026-06-09T10:11:00.000Z",
  });

  assert.equal(notReady.importExport.draft.status, "blocked");
  assert.match(notReady.importExport.draft.issue, /Apply a sanitized import preview/);
  assert.equal(saved.importExport.draft.status, "saved");
  assert.equal(saved.importExport.draft.savedAt, "2026-06-09T10:11:00.000Z");
  assert.equal(saved.importExport.draft.canSaveDraft, true);
  assert.equal(
    saved.importExport.draft.saveReceiptPath,
    ".dx/receipts/n8n-studio/import/latest.sr",
  );
  assert.equal(saved.importExport.draft.lastSavedWorkflowId, saved.document.id);
  assert.equal(saved.importExport.draft.liveProviderExecution, false);
  assert.equal(saved.importExport.draft.persistedToDisk, false);
});

test("n8n Studio shell and store wire real import apply/save handlers", () => {
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
  const storeSource = fs.readFileSync(
    path.join(
      repoRoot,
      "examples",
      "n8n-studio",
      "lib",
      "stores",
      "n8n-studio-store.ts",
    ),
    "utf8",
  );

  assert.match(appSource, /applyImportExportActionToStudioState/);
  assert.match(appSource, /onApplyImportPreview=/);
  assert.match(appSource, /onSaveImportedDraft=/);
  assert.match(panelSource, /data-import-action="apply-preview"/);
  assert.match(panelSource, /data-import-action="save-draft"/);
  assert.match(panelSource, /data-import-draft-status=/);
  assert.match(storeSource, /applyImportExportAction/);
});
