import {
  createParameterSchemaForNode,
  n8nNodeTypeRegistry,
} from "./node-type-registry";
import type { NodeTypeDescription } from "./node-types/types";
import type { ParameterField, WorkflowNode } from "./types";

const n8nTypeAliases: Record<string, string> = {
  "n8n-nodes-base.manualTrigger": "n8n-nodes-base.manualTrigger",
  "n8n-nodes-base.httpRequest": "n8n-nodes-base.httpRequest",
  "n8n-nodes-base.slack": "n8n-nodes-base.slack",
  "n8n-nodes-base.openAi": "n8n-nodes-base.openAi",
};

export function createSelectedNodeParameters(
  node: WorkflowNode,
  registry: Record<string, NodeTypeDescription> = n8nNodeTypeRegistry,
): ParameterField[] {
  const nodeType =
    n8nTypeAliases[node.type] ?? (registry[node.type] ? node.type : undefined);
  if (!nodeType) {
    return [];
  }

  return createParameterSchemaForNode(nodeType, node.parameters, registry).fields;
}
