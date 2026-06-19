import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import {
  applyCanvasInteractionToStudioState,
  createCanvasInteractionState,
} from "../examples/n8n-studio/lib/n8n-studio/canvas-interactions";
import { createN8nStudioState } from "../examples/n8n-studio/lib/n8n-studio/studio-state";

const repoRoot = path.resolve(import.meta.dirname, "..");

test("n8n Studio canvas selection refreshes selected-node dependent editor surfaces", () => {
  const state = createN8nStudioState();
  const nextState = applyCanvasInteractionToStudioState(state, {
    kind: "selectNode",
    nodeId: "node-openai",
  });

  assert.equal(nextState.canvas.selectedNodeId, "node-openai");
  assert.deepEqual(nextState.canvas.interaction.selectedNodeIds, ["node-openai"]);
  assert.equal(nextState.parameters.some((field) => field.name === "chatModel"), true);
  assert.equal(nextState.expressionEditor.selectedNodeId, "node-openai");
  assert.equal(nextState.expressionEditor.expressionFieldCount, 1);
  assert.equal(nextState.credentials[0]?.credentialType, "openAiApi");
  assert.equal(nextState.editorSession.selectedNodeId, "node-openai");
  assert.equal(state.canvas.selectedNodeId, "node-http-request");
});

test("n8n Studio canvas interactions move nodes, pan and zoom without mutating the previous state", () => {
  const state = createN8nStudioState();
  const dragging = applyCanvasInteractionToStudioState(state, {
    kind: "beginNodeDrag",
    nodeId: "node-http-request",
    pointerId: 7,
    canvasPoint: { x: 0, y: 0 },
    snapToGrid: false,
  });
  const moved = applyCanvasInteractionToStudioState(dragging, {
    kind: "movePointer",
    pointerId: 7,
    canvasPoint: { x: 32, y: 48 },
  });
  const ended = applyCanvasInteractionToStudioState(moved, {
    kind: "endPointer",
    pointerId: 7,
  });
  const transformed = applyCanvasInteractionToStudioState(ended, {
    kind: "panViewport",
    delta: { x: -120, y: 80 },
  });
  const zoomed = applyCanvasInteractionToStudioState(transformed, {
    kind: "zoomViewport",
    delta: 0.4,
  });

  const movedNode = zoomed.document.nodes.find(
    (node) => node.id === "node-http-request",
  );

  assert.equal(movedNode?.position.x, 440);
  assert.equal(movedNode?.position.y, 256);
  assert.equal(zoomed.canvas.nodes[1]?.position.x, 440);
  assert.equal(zoomed.canvas.viewport.x, -120);
  assert.equal(zoomed.canvas.viewport.y, 80);
  assert.equal(zoomed.canvas.viewport.zoom, 1.3);
  assert.equal(zoomed.canvas.interaction.mode, "idle");
  assert.equal(state.document.nodes[1]?.position.x, 408);
});

test("n8n Studio canvas supports fit, tidy, and guarded delete workflows", () => {
  const state = createN8nStudioState();
  const fitted = applyCanvasInteractionToStudioState(state, {
    kind: "fitWorkflow",
    viewportSize: { width: 760, height: 420 },
  });
  const tidied = applyCanvasInteractionToStudioState(fitted, {
    kind: "tidyWorkflow",
  });
  const selected = applyCanvasInteractionToStudioState(tidied, {
    kind: "selectNode",
    nodeId: "node-openai",
  });
  const deleted = applyCanvasInteractionToStudioState(selected, {
    kind: "deleteSelection",
  });
  const allSelected = applyCanvasInteractionToStudioState(deleted, {
    kind: "selectAll",
  });
  const blocked = applyCanvasInteractionToStudioState(allSelected, {
    kind: "deleteSelection",
  });

  assert.equal(fitted.canvas.viewport.zoom <= 1.4, true);
  assert.equal(tidied.document.nodes[0]?.position.x < tidied.document.nodes[1]?.position.x, true);
  assert.equal(deleted.document.nodes.some((node) => node.id === "node-openai"), false);
  assert.equal(deleted.document.connections.some((edge) => edge.targetNode === "Summarize Readiness"), false);
  assert.equal(deleted.canvas.selectedNodeId, "node-http-request");
  assert.equal(blocked.document.nodes.length, deleted.document.nodes.length);
  assert.match(blocked.canvas.interaction.issue ?? "", /Cannot delete every node/);
});

test("n8n Studio canvas interaction state records keyboard shortcuts and bounds", () => {
  const state = createN8nStudioState();
  const interaction = createCanvasInteractionState(
    state.document,
    state.canvas.selectedNodeId,
  );

  assert.equal(interaction.mode, "idle");
  assert.equal(interaction.keyboardShortcuts.some((shortcut) => shortcut.key === "Delete"), true);
  assert.equal(interaction.keyboardShortcuts.some((shortcut) => shortcut.key === "f"), true);
  assert.equal(interaction.bounds.minX, 84);
  assert.equal(interaction.bounds.maxX, 940);
  assert.equal(interaction.canDeleteSelection, true);
});

test("n8n Studio canvas components wire real pointer and keyboard actions", () => {
  const canvasSource = fs.readFileSync(
    path.join(
      repoRoot,
      "examples",
      "n8n-studio",
      "components",
      "n8n-studio",
      "workflow-canvas.tsx",
    ),
    "utf8",
  );
  const nodeSource = fs.readFileSync(
    path.join(
      repoRoot,
      "examples",
      "n8n-studio",
      "components",
      "n8n-studio",
      "workflow-node-card.tsx",
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

  assert.match(canvasSource, /onCanvasAction/);
  assert.match(canvasSource, /onPointerMove/);
  assert.match(canvasSource, /onKeyDown/);
  assert.match(canvasSource, /clientPointToCanvasPoint/);
  assert.match(canvasSource, /canvas\.viewport\.zoom/);
  assert.match(canvasSource, /data-canvas-interaction-mode/);
  assert.match(nodeSource, /onPointerDown/);
  assert.match(nodeSource, /tabIndex={0}/);
  assert.match(nodeSource, /aria-selected/);
  assert.match(appSource, /applyCanvasInteractionToStudioState/);
  assert.match(appSource, /onCanvasAction=/);
});
