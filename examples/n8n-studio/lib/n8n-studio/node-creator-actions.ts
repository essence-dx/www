import { createCanvasInteractionState } from "./canvas-interactions";
import { createCredentialReadinessForNode } from "./credentials";
import { createEditorSessionReadiness } from "./editor-session-adapter";
import {
  applyExpressionStateToFields,
  createExpressionEditorState,
} from "./expression-editor";
import {
  getNodeTypeDescription,
  n8nNodeTypeRegistry,
} from "./node-type-registry";
import { fieldVisible } from "./node-types/visibility";
import type { NodeParameterDefinition, NodeTypeDescription } from "./node-types/types";
import { createSelectedNodeParameters } from "./parameter-schema";
import type {
  CatalogSummary,
  N8nSourceNode,
  N8nStudioState,
  WorkflowNode,
} from "./types";

export type NodeCreatorResult = {
  node: N8nSourceNode;
  nodeType: string;
  addable: boolean;
  reason?: string;
};

export type NodeCreatorState = {
  query: string;
  totalCount: number;
  addableCount: number;
  results: NodeCreatorResult[];
};

export type NodeCreatorAction = {
  kind: "addCatalogNode";
  catalogNodeId: string;
};

function lowerFirst(value: string) {
  return value ? `${value[0]?.toLowerCase()}${value.slice(1)}` : value;
}

function nodeTypeForCatalogNode(node: N8nSourceNode) {
  return `n8n-nodes-base.${lowerFirst(node.name)}`;
}

function normalizedQuery(query: string) {
  return query.trim().toLowerCase();
}

function searchableCatalogText(node: N8nSourceNode) {
  return [
    node.displayName,
    node.name,
    node.category,
    node.role,
    node.description,
    node.sourcePath,
    ...node.credentialTypes,
    ...node.operations,
  ].join(" ").toLowerCase();
}

function matchesQuery(node: N8nSourceNode, query: string) {
  return !query || searchableCatalogText(node).includes(query);
}

function creatorResultForNode(
  node: N8nSourceNode,
  registry: Record<string, NodeTypeDescription>,
): NodeCreatorResult {
  const nodeType = nodeTypeForCatalogNode(node);
  const addable = Boolean(registry[nodeType]);

  return {
    node,
    nodeType,
    addable,
    reason: addable
      ? undefined
      : "This catalog entry is visible from source but is not ready in the semantic registry.",
  };
}

export function createNodeCreatorState(
  catalog: CatalogSummary,
  query = "",
  registry: Record<string, NodeTypeDescription> = n8nNodeTypeRegistry,
): NodeCreatorState {
  const normalized = normalizedQuery(query);
  const results = catalog.catalogNodes
    .filter((node) => matchesQuery(node, normalized))
    .map((node) => creatorResultForNode(node, registry));

  return {
    query,
    totalCount: catalog.catalogNodes.length,
    addableCount: results.filter((result) => result.addable).length,
    results,
  };
}

function cloneDefaultValue(value: unknown): unknown {
  if (Array.isArray(value)) {
    return value.map(cloneDefaultValue);
  }
  if (value && typeof value === "object") {
    return Object.fromEntries(
      Object.entries(value).map(([key, nestedValue]) => [
        key,
        cloneDefaultValue(nestedValue),
      ]),
    );
  }
  return value;
}

function applyVisibleDefaults(
  parameters: Record<string, unknown>,
  definitions: NodeParameterDefinition[],
) {
  let nextParameters = { ...parameters };

  for (let pass = 0; pass < 4; pass += 1) {
    for (const definition of definitions) {
      if (
        nextParameters[definition.name] === undefined &&
        definition.defaultValue !== undefined &&
        fieldVisible(definition, nextParameters)
      ) {
        nextParameters = {
          ...nextParameters,
          [definition.name]: cloneDefaultValue(definition.defaultValue),
        };
      }
    }
  }

  return nextParameters;
}

function defaultParametersForNodeType(description: NodeTypeDescription) {
  return applyVisibleDefaults({}, description.properties);
}

function uniqueNodeId(nodes: WorkflowNode[], catalogNodeId: string) {
  const baseId = `node-${catalogNodeId}`;
  const existingIds = new Set(nodes.map((node) => node.id));
  if (!existingIds.has(baseId)) {
    return baseId;
  }

  for (let index = 2; ; index += 1) {
    const candidate = `${baseId}-${index}`;
    if (!existingIds.has(candidate)) {
      return candidate;
    }
  }
}

function uniqueNodeName(nodes: WorkflowNode[], displayName: string) {
  const existingNames = new Set(nodes.map((node) => node.name));
  if (!existingNames.has(displayName)) {
    return displayName;
  }

  for (let index = 2; ; index += 1) {
    const candidate = `${displayName} ${index}`;
    if (!existingNames.has(candidate)) {
      return candidate;
    }
  }
}

function nextNodePosition(state: N8nStudioState) {
  const selectedNode = state.document.nodes.find(
    (node) => node.id === state.canvas.selectedNodeId,
  );
  const maxX = state.document.nodes.reduce(
    (currentMax, node) => Math.max(currentMax, node.position.x),
    84,
  );

  return {
    x: maxX + 300,
    y: selectedNode?.position.y ?? 208,
  };
}

function withNodeCreatorIssue(state: N8nStudioState, issue: string) {
  return {
    ...state,
    canvas: {
      ...state.canvas,
      interaction: {
        ...state.canvas.interaction,
        issue,
      },
    },
  };
}

function createWorkflowNodeFromCatalogResult(
  state: N8nStudioState,
  result: NodeCreatorResult,
  registry: Record<string, NodeTypeDescription>,
): WorkflowNode {
  const description = getNodeTypeDescription(result.nodeType, registry);

  return {
    id: uniqueNodeId(state.document.nodes, result.node.id),
    name: uniqueNodeName(state.document.nodes, result.node.displayName),
    type: result.nodeType,
    typeVersion: description.version,
    position: nextNodePosition(state),
    parameters: defaultParametersForNodeType(description),
    notes: `Added from ${result.node.sourcePath}.`,
  };
}

export function applyNodeCreatorActionToStudioState(
  state: N8nStudioState,
  action: NodeCreatorAction,
  registry: Record<string, NodeTypeDescription> = n8nNodeTypeRegistry,
): N8nStudioState {
  const creatorState = createNodeCreatorState(state.catalog, "", registry);
  const result = creatorState.results.find(
    (candidate) => candidate.node.id === action.catalogNodeId,
  );
  if (!result) {
    return withNodeCreatorIssue(
      state,
      `Catalog entry ${action.catalogNodeId} is not available.`,
    );
  }
  if (!result.addable) {
    return withNodeCreatorIssue(state, result.reason ?? "Catalog entry is not ready.");
  }

  const node = createWorkflowNodeFromCatalogResult(state, result, registry);
  const document = {
    ...state.document,
    nodes: [...state.document.nodes, node],
  };
  const parameters = createSelectedNodeParameters(node, registry);
  const expressionEditor = createExpressionEditorState({
    document,
    parameters,
    selectedNode: node,
  });
  const description = getNodeTypeDescription(node.type, registry);

  return {
    ...state,
    document,
    canvas: {
      ...state.canvas,
      selectedNodeId: node.id,
      nodes: document.nodes,
      connections: document.connections,
      interaction: createCanvasInteractionState(document, node.id, [node.id]),
    },
    parameters: applyExpressionStateToFields(parameters, expressionEditor.fields),
    expressionEditor,
    credentials: createCredentialReadinessForNode(node, description),
    editorSession: createEditorSessionReadiness(document, node.id, registry),
  };
}
