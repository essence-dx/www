import {
  createParameterSchemaForNode,
  n8nNodeTypeRegistry,
} from "./node-type-registry";
import {
  applyExpressionStateToFields,
  createExpressionEditorState,
} from "./expression-editor";
import type { NodeTypeDescription } from "./node-types/types";
import type {
  N8nStudioState,
  ParameterValuePath,
  WorkflowNode,
} from "./types";

export type ParameterMutation =
  | {
      kind: "addCollectionItem";
      collectionPath: ParameterValuePath;
      item?: Record<string, unknown>;
    }
  | {
      kind: "removeCollectionItem";
      collectionPath: ParameterValuePath;
      itemIndex: number;
    }
  | {
      kind: "updateValue";
      valuePath: ParameterValuePath;
      value: unknown;
    };

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function cloneRecord(value: Record<string, unknown>) {
  return Object.fromEntries(
    Object.entries(value).map(([key, nestedValue]) => [key, cloneValue(nestedValue)]),
  );
}

function cloneValue(value: unknown): unknown {
  if (Array.isArray(value)) {
    return value.map(cloneValue);
  }
  if (isRecord(value)) {
    return cloneRecord(value);
  }
  return value;
}

function readValueAtPath(root: unknown, path: ParameterValuePath): unknown {
  return path.reduce((currentValue, segment) => {
    if (typeof segment === "number") {
      return Array.isArray(currentValue) ? currentValue[segment] : undefined;
    }
    return isRecord(currentValue) ? currentValue[segment] : undefined;
  }, root);
}

function setNestedValue(
  currentValue: unknown,
  path: ParameterValuePath,
  value: unknown,
): unknown {
  if (path.length === 0) {
    return cloneValue(value);
  }

  const [segment, ...remainingPath] = path;
  if (typeof segment === "number") {
    const nextArray = Array.isArray(currentValue) ? [...currentValue] : [];
    nextArray[segment] = setNestedValue(nextArray[segment], remainingPath, value);
    return nextArray;
  }

  const nextRecord = isRecord(currentValue) ? { ...currentValue } : {};
  nextRecord[segment] = setNestedValue(nextRecord[segment], remainingPath, value);
  return nextRecord;
}

function setValueAtPath(
  parameters: Record<string, unknown>,
  path: ParameterValuePath,
  value: unknown,
) {
  return setNestedValue(parameters, path, value) as Record<string, unknown>;
}

export function addParameterCollectionItem(
  parameters: Record<string, unknown>,
  collectionPath: ParameterValuePath,
  item: Record<string, unknown> = {},
) {
  const currentValue = readValueAtPath(parameters, collectionPath);
  const currentItems = Array.isArray(currentValue)
    ? currentValue.map(cloneValue)
    : [];

  return setValueAtPath(parameters, collectionPath, [
    ...currentItems,
    cloneRecord(item),
  ]);
}

export function removeParameterCollectionItem(
  parameters: Record<string, unknown>,
  collectionPath: ParameterValuePath,
  itemIndex: number,
) {
  const currentValue = readValueAtPath(parameters, collectionPath);
  const currentItems = Array.isArray(currentValue) ? currentValue.map(cloneValue) : [];

  return setValueAtPath(
    parameters,
    collectionPath,
    currentItems.filter((_, index) => index !== itemIndex),
  );
}

export function updateParameterValue(
  parameters: Record<string, unknown>,
  valuePath: ParameterValuePath,
  value: unknown,
) {
  return setValueAtPath(parameters, valuePath, value);
}

export function applyParameterMutationToWorkflowNode(
  node: WorkflowNode,
  mutation: ParameterMutation,
): WorkflowNode {
  const parameters =
    mutation.kind === "addCollectionItem"
      ? addParameterCollectionItem(node.parameters, mutation.collectionPath, mutation.item)
      : mutation.kind === "removeCollectionItem"
        ? removeParameterCollectionItem(
            node.parameters,
            mutation.collectionPath,
            mutation.itemIndex,
          )
        : updateParameterValue(node.parameters, mutation.valuePath, mutation.value);

  return {
    ...node,
    parameters,
  };
}

function schemaFieldsForNode(
  node: WorkflowNode,
  registry: Record<string, NodeTypeDescription>,
) {
  try {
    return createParameterSchemaForNode(node.type, node.parameters, registry).fields;
  } catch {
    return undefined;
  }
}

export function applyParameterMutationToStudioState(
  state: N8nStudioState,
  nodeId: string,
  mutation: ParameterMutation,
  registry: Record<string, NodeTypeDescription> = n8nNodeTypeRegistry,
): N8nStudioState {
  let updatedNode: WorkflowNode | undefined;
  const nodes = state.document.nodes.map((node) => {
    if (node.id !== nodeId) {
      return node;
    }
    updatedNode = applyParameterMutationToWorkflowNode(node, mutation);
    return updatedNode;
  });

  if (!updatedNode) {
    return state;
  }
  const document = {
    ...state.document,
    nodes,
  };
  const selectedNodeWasUpdated = state.canvas.selectedNodeId === nodeId;
  const selectedParameters = selectedNodeWasUpdated
    ? (schemaFieldsForNode(updatedNode, registry) ?? state.parameters)
    : state.parameters;
  const expressionEditor = selectedNodeWasUpdated
    ? createExpressionEditorState({
        document,
        parameters: selectedParameters,
        selectedNode: updatedNode,
      })
    : state.expressionEditor;

  return {
    ...state,
    document,
    canvas: {
      ...state.canvas,
      nodes,
    },
    parameters: selectedNodeWasUpdated
      ? applyExpressionStateToFields(selectedParameters, expressionEditor.fields)
      : selectedParameters,
    expressionEditor,
  };
}
