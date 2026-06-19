import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import { applyCanvasInteractionToStudioState } from "../examples/n8n-studio/lib/n8n-studio/canvas-interactions";
import { createN8nStudioState } from "../examples/n8n-studio/lib/n8n-studio/studio-state";

const repoRoot = path.resolve(import.meta.dirname, "..");

test("n8n Studio canvas creates validated main connections from source and target handles", () => {
  const state = createN8nStudioState();
  const withoutHttpToOpenAi = {
    ...state,
    document: {
      ...state.document,
      connections: state.document.connections.filter(
        (connection) => connection.id !== "edge-http-to-openai",
      ),
    },
    canvas: {
      ...state.canvas,
      connections: state.canvas.connections.filter(
        (connection) => connection.id !== "edge-http-to-openai",
      ),
    },
  };

  const drafting = applyCanvasInteractionToStudioState(withoutHttpToOpenAi, {
    kind: "beginConnectionDrag",
    pointerId: 22,
    sourceNodeId: "node-http-request",
    sourceOutput: "main",
    canvasPoint: { x: 596, y: 252 },
  });
  const connected = applyCanvasInteractionToStudioState(drafting, {
    kind: "completeConnection",
    targetNodeId: "node-openai",
    targetInput: "main",
  });

  const connection = connected.document.connections.find(
    (edge) =>
      edge.sourceNode === "Read Connector Manifest" &&
      edge.targetNode === "Summarize Readiness",
  );

  assert.equal(drafting.canvas.interaction.mode, "edge-drag");
  assert.equal(drafting.canvas.interaction.edgeDraft?.sourceNodeId, "node-http-request");
  assert.equal(drafting.canvas.interaction.edgeDraft?.validEndpointNodeIds.includes("node-openai"), true);
  assert.equal(connection?.id, "edge-node-http-request-to-node-openai-main-main");
  assert.equal(connection?.sourceOutput, "main");
  assert.equal(connection?.targetInput, "main");
  assert.equal(connected.canvas.connections, connected.document.connections);
  assert.equal(connected.canvas.interaction.selectedConnectionId, connection?.id);
});

test("n8n Studio canvas rejects duplicate and self-loop connections without mutating connections", () => {
  const state = createN8nStudioState();
  const duplicateDraft = applyCanvasInteractionToStudioState(state, {
    kind: "beginConnectionDrag",
    pointerId: 23,
    sourceNodeId: "node-http-request",
    sourceOutput: "main",
    canvasPoint: { x: 596, y: 252 },
  });
  const duplicateRejected = applyCanvasInteractionToStudioState(duplicateDraft, {
    kind: "completeConnection",
    targetNodeId: "node-openai",
    targetInput: "main",
  });
  const selfDraft = applyCanvasInteractionToStudioState(state, {
    kind: "beginConnectionDrag",
    pointerId: 24,
    sourceNodeId: "node-http-request",
    sourceOutput: "main",
    canvasPoint: { x: 596, y: 252 },
  });
  const selfRejected = applyCanvasInteractionToStudioState(selfDraft, {
    kind: "completeConnection",
    targetNodeId: "node-http-request",
    targetInput: "main",
  });

  assert.equal(duplicateRejected.document.connections.length, state.document.connections.length);
  assert.match(duplicateRejected.canvas.interaction.issue ?? "", /already connected/);
  assert.equal(selfRejected.document.connections.length, state.document.connections.length);
  assert.match(selfRejected.canvas.interaction.issue ?? "", /cannot connect to itself/);
});

test("n8n Studio canvas reconnects existing edge targets with validation", () => {
  const state = createN8nStudioState();
  const reconnecting = applyCanvasInteractionToStudioState(state, {
    kind: "beginReconnectEdge",
    edgeId: "edge-manual-to-http",
    endpoint: "target",
    pointerId: 31,
    canvasPoint: { x: 420, y: 252 },
  });
  const reconnected = applyCanvasInteractionToStudioState(reconnecting, {
    kind: "completeConnection",
    targetNodeId: "node-openai",
    targetInput: "main",
  });
  const edge = reconnected.document.connections.find(
    (connection) => connection.id === "edge-manual-to-http",
  );
  const invalidReconnect = applyCanvasInteractionToStudioState(reconnecting, {
    kind: "completeConnection",
    targetNodeId: "node-manual-trigger",
    targetInput: "main",
  });

  assert.equal(reconnecting.canvas.interaction.mode, "edge-reconnect");
  assert.equal(reconnecting.canvas.interaction.edgeDraft?.edgeId, "edge-manual-to-http");
  assert.equal(edge?.sourceNode, "Manual Trigger");
  assert.equal(edge?.targetNode, "Summarize Readiness");
  assert.equal(reconnected.document.connections.length, state.document.connections.length);
  assert.match(invalidReconnect.canvas.interaction.issue ?? "", /cannot connect to itself/);
});

test("n8n Studio canvas reconnects existing edge sources with validation", () => {
  const state = createN8nStudioState();
  const reconnecting = applyCanvasInteractionToStudioState(state, {
    kind: "beginReconnectEdge",
    edgeId: "edge-http-to-openai",
    endpoint: "source",
    pointerId: 32,
    canvasPoint: { x: 576, y: 252 },
  });
  const reconnected = applyCanvasInteractionToStudioState(reconnecting, {
    kind: "completeConnection",
    nodeId: "node-manual-trigger",
    endpoint: "source",
    inputOrOutput: "main",
  });
  const edge = reconnected.document.connections.find(
    (connection) => connection.id === "edge-http-to-openai",
  );
  const invalidHandle = applyCanvasInteractionToStudioState(reconnecting, {
    kind: "completeConnection",
    nodeId: "node-manual-trigger",
    endpoint: "target",
    inputOrOutput: "main",
  });

  assert.equal(reconnecting.canvas.interaction.mode, "edge-reconnect");
  assert.equal(reconnecting.canvas.interaction.edgeDraft?.reconnectEndpoint, "source");
  assert.equal(edge?.sourceNode, "Manual Trigger");
  assert.equal(edge?.targetNode, "Summarize Readiness");
  assert.equal(reconnected.document.connections.length, state.document.connections.length);
  assert.match(invalidHandle.canvas.interaction.issue ?? "", /source handle/);
});

test("n8n Studio canvas components expose real connection and reconnect handles", () => {
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
  const edgeSource = fs.readFileSync(
    path.join(
      repoRoot,
      "examples",
      "n8n-studio",
      "components",
      "n8n-studio",
      "workflow-edge-layer.tsx",
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

  assert.match(canvasSource, /onBeginReconnectEdge/);
  assert.match(canvasSource, /WorkflowEdgeLayer/);
  assert.match(canvasSource, /kind:\s*"completeConnection"/);
  assert.match(edgeSource, /data-connection-id/);
  assert.match(edgeSource, /data-edge-reconnect-endpoint/);
  assert.match(nodeSource, /onBeginConnectionDrag/);
  assert.match(nodeSource, /onCompleteConnection/);
  assert.match(nodeSource, /data-edge-handle="source"/);
  assert.match(nodeSource, /data-edge-handle="target"/);
});
