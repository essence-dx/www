import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import {
  analyzeN8nExpression,
  createExpressionEditorState,
} from "../examples/n8n-studio/lib/n8n-studio/expression-editor";
import { createN8nStudioState } from "../examples/n8n-studio/lib/n8n-studio/studio-state";
import { createSelectedNodeParameters } from "../examples/n8n-studio/lib/n8n-studio/parameter-schema";
import { studioWorkflowDocument } from "../examples/n8n-studio/lib/n8n-studio/workflow-document";

const repoRoot = path.resolve(import.meta.dirname, "..");

test("n8n Studio analyzes selected-node expressions without evaluating provider data", () => {
  const state = createN8nStudioState();

  assert.equal(state.expressionEditor.schema, "dx.n8n-studio.expression-editor");
  assert.equal(state.expressionEditor.selectedNodeId, "node-http-request");
  assert.equal(state.expressionEditor.liveProviderExecution, false);
  assert.equal(state.expressionEditor.secretsIncluded, false);
  assert.equal(state.expressionEditor.expressionFieldCount, 1);

  const urlExpression = state.expressionEditor.fields.find(
    (field) => field.fieldName === "url",
  );
  assert.equal(urlExpression?.fieldLabel, "URL");
  assert.equal(urlExpression?.expression, "={{ $json.manifestUrl }}");
  assert.equal(urlExpression?.expressionBody, "$json.manifestUrl");
  assert.deepEqual(urlExpression?.valuePath, ["url"]);
  assert.equal(urlExpression?.references[0]?.kind, "json");
  assert.equal(urlExpression?.references[0]?.path, "manifestUrl");
  assert.equal(urlExpression?.previewBoundary.liveProviderExecution, false);
  assert.match(urlExpression?.previewBoundary.issue ?? "", /source-owned expression preview/i);
});

test("n8n Studio carries nested OpenAI prompt expressions into expression editor state", () => {
  const openAiNode = studioWorkflowDocument.nodes.find(
    (node) => node.id === "node-openai",
  );
  assert.ok(openAiNode);

  const parameters = createSelectedNodeParameters(openAiNode);
  const expressionEditor = createExpressionEditorState({
    document: studioWorkflowDocument,
    parameters,
    selectedNode: openAiNode,
  });

  const promptContent = expressionEditor.fields.find(
    (field) =>
      field.fieldName === "content" &&
      field.valuePath.join(".") === "prompt.messages.1.content",
  );

  assert.equal(expressionEditor.selectedNodeId, "node-openai");
  assert.equal(expressionEditor.expressionFieldCount, 1);
  assert.equal(promptContent?.expression, "={{ $json.connectorSummary }}");
  assert.equal(promptContent?.references[0]?.kind, "json");
  assert.equal(promptContent?.references[0]?.path, "connectorSummary");
  assert.equal(promptContent?.fieldLabel, "Content");
});

test("n8n Studio flags credential and env expression references without preserving secret values", () => {
  const analysis = analyzeN8nExpression(
    '={{ $credentials.openAiApi.apiKey + " " + $env.OPENAI_API_KEY }}',
  );
  const serialized = JSON.stringify(analysis);

  assert.equal(analysis.mode, "full-expression");
  assert.deepEqual(
    analysis.references.map((reference) => reference.kind),
    ["credentials", "env"],
  );
  assert.deepEqual(
    analysis.diagnostics.map((diagnostic) => diagnostic.code),
    ["secret-reference-blocked", "secret-reference-blocked"],
  );
  assert.equal(serialized.includes("must-not-survive"), false);
  assert.equal(serialized.includes("apiKey"), true);
});

test("n8n Studio expression editor UI is driven by expression state instead of fixed demo text", () => {
  const inspectorSource = fs.readFileSync(
    path.join(
      repoRoot,
      "examples",
      "n8n-studio",
      "components",
      "n8n-studio",
      "parameter-inspector.tsx",
    ),
    "utf8",
  );
  const fieldControlSource = fs.readFileSync(
    path.join(
      repoRoot,
      "examples",
      "n8n-studio",
      "components",
      "n8n-studio",
      "parameter-field-control.tsx",
    ),
    "utf8",
  );

  assert.match(inspectorSource, /ExpressionEditorPanel/);
  assert.doesNotMatch(inspectorSource, /\$json\.manifestUrl/);
  assert.match(fieldControlSource, /data-expression-field/);
  assert.match(fieldControlSource, /expressionEditor/);
});
