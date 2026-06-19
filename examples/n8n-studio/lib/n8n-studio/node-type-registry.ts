import { httpRequestNodeType } from "./node-types/http-request";
import { manualTriggerNodeType } from "./node-types/manual-trigger";
import { openAiNodeType } from "./node-types/openai";
import { slackNodeType } from "./node-types/slack";
import type { NodeParameterSchema, NodeTypeDescription } from "./node-types/types";
import { createParameterField, fieldVisible } from "./node-types/visibility";

export type { NodeParameterDefinition, NodeParameterSchema, NodeTypeDescription } from "./node-types/types";

export const n8nNodeTypeRegistry: Record<string, NodeTypeDescription> = {
  [manualTriggerNodeType.name]: manualTriggerNodeType,
  [httpRequestNodeType.name]: httpRequestNodeType,
  [slackNodeType.name]: slackNodeType,
  [openAiNodeType.name]: openAiNodeType,
};

export function mergeNodeTypeRegistries(
  ...registries: Array<Record<string, NodeTypeDescription>>
): Record<string, NodeTypeDescription> {
  return Object.assign({}, ...registries);
}

export function getNodeTypeDescription(
  name: string,
  registry: Record<string, NodeTypeDescription> = n8nNodeTypeRegistry,
): NodeTypeDescription {
  const description = registry[name];
  if (!description) {
    throw new Error(`Unknown n8n node type: ${name}`);
  }

  return description;
}

function boundaryFieldPath(field: ReturnType<typeof createParameterField>) {
  return field.valuePath?.map(String).join(".") || field.name;
}

function dynamicLoadBoundariesForField(
  nodeType: string,
  field: ReturnType<typeof createParameterField>,
): string[] {
  const fieldPath = boundaryFieldPath(field);

  return [
    ...(field.resourceLocatorModes
      ?.filter((mode) => mode.searchListMethod)
      .map((mode) => `${nodeType}.${fieldPath}.${mode.searchListMethod}`) ?? []),
    ...(field.dynamicOptions
      ? [`${nodeType}.${fieldPath}.${field.dynamicOptions.loadMethod}`]
      : []),
    ...(field.resourceMapper
      ? [`${nodeType}.${fieldPath}.${field.resourceMapper.resourceMapperMethod}`]
      : []),
    ...(field.childFields?.flatMap((childField) =>
      dynamicLoadBoundariesForField(nodeType, childField),
    ) ?? []),
    ...(field.collectionItems?.flatMap((item) =>
      item.fields.flatMap((itemField) =>
        dynamicLoadBoundariesForField(nodeType, itemField),
      ),
    ) ?? []),
  ];
}

export function createParameterSchemaForNode(
  nodeType: string,
  values: Record<string, unknown> = {},
  registry: Record<string, NodeTypeDescription> = n8nNodeTypeRegistry,
): NodeParameterSchema {
  const description = getNodeTypeDescription(nodeType, registry);
  const visibleDefinitions = description.properties.filter((definition) =>
    fieldVisible(definition, values),
  );
  const fields = visibleDefinitions.map((definition) =>
    createParameterField(definition, values),
  );
  const hiddenFields = description.properties
    .filter((definition) => !fieldVisible(definition, values))
    .map((definition) => definition.name);
  const dynamicLoadBoundaries = [
    ...new Set(fields.flatMap((field) => dynamicLoadBoundariesForField(nodeType, field))),
  ];

  return {
    nodeType,
    fields,
    dynamicLoadBoundaries,
    hiddenFields,
  };
}
