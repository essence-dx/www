import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import {
  applyNodeCreatorActionToStudioState,
  createNodeCreatorState,
} from "../examples/n8n-studio/lib/n8n-studio/node-creator-actions";
import { createN8nStudioStore } from "../examples/n8n-studio/lib/stores/n8n-studio-store";
import {
  createGeneratedNodeTypeRegistry,
  createStudioCatalogFromGeneratedConnectors,
} from "../examples/n8n-studio/lib/n8n-studio/generated-connectors/index";
import { createN8nStudioState } from "../examples/n8n-studio/lib/n8n-studio/studio-state";
import { createStudioBootFromLocalGeneratedSource } from "../examples/n8n-studio/server/n8n-studio/generated-catalog-source";

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

test("n8n Studio node creator filters the catalog by real query state", () => {
  const state = createN8nStudioState();
  const creator = createNodeCreatorState(state.catalog, "open");

  assert.equal(creator.query, "open");
  assert.equal(creator.totalCount, state.catalog.catalogNodes.length);
  assert.equal(creator.results.length, 1);
  assert.equal(creator.results[0]?.node.displayName, "OpenAI");
  assert.equal(creator.results[0]?.nodeType, "n8n-nodes-base.openAi");
  assert.equal(creator.results[0]?.addable, true);

  const gmail = createNodeCreatorState(state.catalog, "gmail").results[0];
  assert.equal(gmail?.node.displayName, "Gmail");
  assert.equal(gmail?.nodeType, "n8n-nodes-base.gmail");
  assert.equal(gmail?.addable, false);
  assert.match(gmail?.reason ?? "", /semantic registry/);
});

test("n8n Studio node creator adds supported catalog nodes into the real workflow", () => {
  const state = createN8nStudioState();
  const nextState = applyNodeCreatorActionToStudioState(state, {
    kind: "addCatalogNode",
    catalogNodeId: "openai",
  });

  const addedNode = nextState.document.nodes.at(-1);
  assert.equal(nextState.document.nodes.length, state.document.nodes.length + 1);
  assert.equal(addedNode?.type, "n8n-nodes-base.openAi");
  assert.equal(addedNode?.name, "OpenAI");
  assert.equal(addedNode?.position.x, 1060);
  assert.equal(addedNode?.position.y, 208);
  assert.equal(nextState.canvas.selectedNodeId, addedNode?.id);
  assert.deepEqual(nextState.canvas.interaction.selectedNodeIds, [addedNode?.id]);
  assert.equal(nextState.parameters.some((field) => field.name === "resource"), true);
  assert.equal(nextState.credentials[0]?.credentialType, "openAiApi");
  assert.equal(nextState.editorSession.selectedNodeId, addedNode?.id);
  assert.equal(state.document.nodes.some((node) => node.id === addedNode?.id), false);
});

test("n8n Studio node creator can add generated-registry catalog nodes", () => {
  const generatedCatalog = readGeneratedCatalog();
  const generated = createGeneratedNodeTypeRegistry(generatedCatalog);
  const catalog = createStudioCatalogFromGeneratedConnectors(generatedCatalog);
  const state = {
    ...createN8nStudioState(),
    catalog,
  };
  const creator = createNodeCreatorState(catalog, "gmail", generated.registry);
  const gmail = creator.results.find((result) => result.node.id === "gmail");

  assert.equal(gmail?.addable, true);
  assert.equal(gmail?.nodeType, "n8n-nodes-base.gmail");

  const nextState = applyNodeCreatorActionToStudioState(
    state,
    {
      kind: "addCatalogNode",
      catalogNodeId: "gmail",
    },
    generated.registry,
  );
  const addedNode = nextState.document.nodes.at(-1);

  assert.equal(addedNode?.type, "n8n-nodes-base.gmail");
  assert.equal(nextState.canvas.selectedNodeId, addedNode?.id);
  assert.equal(nextState.parameters.some((field) => field.name === "resource"), true);
  assert.equal(nextState.parameters.some((field) => field.name === "operation"), true);
  assert.equal(
    nextState.parameters
      .find((field) => field.name === "operation")
      ?.options?.some((option) => option.value === "send"),
    true,
  );
  assert.equal(
    nextState.credentials.some((credential) => credential.credentialType === "gmailOAuth2"),
    true,
  );
  assert.equal(nextState.editorSession.nodeType, "n8n-nodes-base.gmail");
});

test("n8n Studio store can boot generated catalog state and add generated nodes", () => {
  const boot = createStudioBootFromLocalGeneratedSource(repoRoot);
  const studioStore = createN8nStudioStore({
    initialState: boot.state,
    nodeTypeRegistry: boot.nodeTypeRegistry,
  });

  assert.equal(studioStore.state.catalog.generatedMetadata?.sourceAvailable, true);

  studioStore.applyNodeCreatorAction(studioStore, {
    kind: "addCatalogNode",
    catalogNodeId: "gmail",
  });

  const addedNode = studioStore.state.document.nodes.at(-1);
  assert.equal(addedNode?.type, "n8n-nodes-base.gmail");
  assert.equal(studioStore.state.canvas.selectedNodeId, addedNode?.id);
  assert.equal(studioStore.state.editorSession.nodeType, "n8n-nodes-base.gmail");
});

test("n8n Studio node creator blocks unsupported catalog entries without fake nodes", () => {
  const state = createN8nStudioState();
  const nextState = applyNodeCreatorActionToStudioState(state, {
    kind: "addCatalogNode",
    catalogNodeId: "gmail",
  });

  assert.equal(nextState.document.nodes.length, state.document.nodes.length);
  assert.equal(nextState.canvas.selectedNodeId, state.canvas.selectedNodeId);
  assert.match(nextState.canvas.interaction.issue ?? "", /semantic registry/);
});

test("n8n Studio catalog panel wires search and add actions instead of hardcoded UI", () => {
  const panelSource = fs.readFileSync(
    path.join(
      repoRoot,
      "examples",
      "n8n-studio",
      "components",
      "n8n-studio",
      "node-catalog-panel.tsx",
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

  assert.doesNotMatch(panelSource, /value="http, ai, slack"/);
  assert.match(panelSource, /useState\(""\)/);
  assert.match(panelSource, /createNodeCreatorState/);
  assert.match(panelSource, /onAddCatalogNode/);
  assert.match(panelSource, /data-node-creator-add/);
  assert.match(panelSource, /data-node-addable/);
  assert.match(appSource, /applyNodeCreatorActionToStudioState/);
  assert.match(appSource, /onAddCatalogNode=/);
});
