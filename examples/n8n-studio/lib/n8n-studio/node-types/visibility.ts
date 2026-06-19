import type {
  ParameterCollectionItem,
  ParameterField,
  ParameterValuePath,
} from "../types";
import { createResourceLocatorDraft } from "../resource-locator-draft";
import type { NodeParameterDefinition } from "./types";

function valueMatchesDisplayRule(
  values: Record<string, unknown>,
  rule: Record<string, unknown[]> | undefined,
) {
  if (!rule) {
    return true;
  }

  return Object.entries(rule).every(([fieldName, acceptedValues]) =>
    acceptedValues.includes(displayRuleValue(values, fieldName)),
  );
}

function valueAtPath(values: Record<string, unknown>, path: string) {
  return path.split(".").reduce<unknown>((value, segment) => {
    if (!isRecord(value)) {
      return undefined;
    }
    return value[segment];
  }, values);
}

function displayRuleValue(values: Record<string, unknown>, fieldName: string) {
  const path = fieldName.startsWith("/") ? fieldName.slice(1) : fieldName;
  return path.includes(".") ? valueAtPath(values, path) : values[path];
}

export function fieldVisible(
  definition: NodeParameterDefinition,
  values: Record<string, unknown>,
) {
  const showMatches = valueMatchesDisplayRule(values, definition.displayOptions?.show);
  const hideMatches = definition.displayOptions?.hide
    ? valueMatchesDisplayRule(values, definition.displayOptions.hide)
    : false;

  return showMatches && !hideMatches;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function isRecordWithValues(value: unknown): value is Record<string, unknown> {
  return isRecord(value) && Object.keys(value).length > 0;
}

function titleFromKey(key: string) {
  return key
    .replace(/[_-]+/g, " ")
    .replace(/([a-z0-9])([A-Z])/g, "$1 $2")
    .replace(/\b\w/g, (letter) => letter.toUpperCase());
}

function collectionValueScopes(
  type: ParameterField["type"],
  value: unknown,
  valuePath: ParameterValuePath,
): Array<{
  key: string;
  label: string;
  itemIndex: number;
  collectionPath: ParameterValuePath;
  valuePath: ParameterValuePath;
  values: Record<string, unknown>;
}> {
  if (type === "collection") {
    if (Array.isArray(value)) {
      return value
        .filter(isRecordWithValues)
        .map((item, index) => ({
          key: `item-${index + 1}`,
          label: `Item ${index + 1}`,
          itemIndex: index,
          collectionPath: valuePath,
          valuePath: [...valuePath, index],
          values: item,
        }));
    }

    return isRecordWithValues(value)
      ? [
          {
            key: "item-1",
            label: "Item 1",
            itemIndex: 0,
            collectionPath: valuePath,
            valuePath,
            values: value,
          },
        ]
      : [];
  }

  if (type !== "fixedCollection" || !isRecord(value)) {
    return [];
  }

  return Object.entries(value).flatMap(([groupName, groupValue]) => {
    if (Array.isArray(groupValue)) {
      return groupValue
        .filter(isRecordWithValues)
        .map((item, index) => ({
          key: `${groupName}-${index + 1}`,
          label: `${titleFromKey(groupName)} ${index + 1}`,
          itemIndex: index,
          collectionPath: [...valuePath, groupName],
          valuePath: [...valuePath, groupName, index],
          values: item,
        }));
    }

    return isRecordWithValues(groupValue)
      ? [
          {
            key: `${groupName}-1`,
            label: `${titleFromKey(groupName)} 1`,
            itemIndex: 0,
            collectionPath: [...valuePath, groupName],
            valuePath: [...valuePath, groupName],
            values: groupValue,
          },
        ]
      : [];
  });
}

function childValueScope(
  type: ParameterField["type"],
  value: unknown,
  valuePath: ParameterValuePath,
): Record<string, unknown> {
  return collectionValueScopes(type, value, valuePath)[0]?.values ?? {};
}

function childValuePath(
  type: ParameterField["type"],
  value: unknown,
  valuePath: ParameterValuePath,
) {
  return collectionValueScopes(type, value, valuePath)[0]?.valuePath ?? valuePath;
}

function collectionItemsFromDefinition(
  definition: NodeParameterDefinition,
  value: unknown,
  valuePath: ParameterValuePath,
  rootValues: Record<string, unknown>,
): ParameterCollectionItem[] | undefined {
  if (!definition.childParameters?.length) {
    return undefined;
  }

  const collectionItems = collectionValueScopes(definition.type, value, valuePath).map((item) => ({
    key: item.key,
    label: item.label,
    itemIndex: item.itemIndex,
    collectionPath: item.collectionPath,
    valuePath: item.valuePath,
    fields: visibleChildDefinitions(
      definition.childParameters ?? [],
      item.values,
      rootValues,
    ).map((childDefinition) =>
      createParameterField(
        childDefinition,
        item.values,
        [
          ...item.valuePath,
          childDefinition.name,
        ],
        rootValues,
      ),
    ),
  }));

  return collectionItems.length > 0 ? collectionItems : undefined;
}

export function createParameterField(
  definition: NodeParameterDefinition,
  values: Record<string, unknown>,
  valuePath: ParameterValuePath = [definition.name],
  rootValues: Record<string, unknown> = values,
): ParameterField {
  const value = values[definition.name] ?? definition.defaultValue;
  const nestedValues = childValueScope(definition.type, value, valuePath);
  const nestedValuePath = childValuePath(definition.type, value, valuePath);
  const childDefinitions = visibleChildDefinitions(
    definition.childParameters ?? [],
    nestedValues,
    rootValues,
  );

  return {
    name: definition.name,
    label: definition.label,
    type: definition.type,
    required: definition.required ?? false,
    expressionEnabled:
      definition.noDataExpression === true ||
      definition.type === "notice" ||
      definition.type === "curlImport"
        ? false
        : true,
    noDataExpression: definition.noDataExpression,
    value,
    valuePath,
    defaultValue: definition.defaultValue,
    description: definition.description,
    placeholder: definition.placeholder,
    options: definition.options,
    credentialTypes: definition.credentialTypes,
    displayOptions: definition.displayOptions,
    dynamicOptions: definition.dynamicOptions,
    resourceMapper: definition.resourceMapper,
    resourceLocatorModes: definition.resourceLocatorModes,
    resourceLocatorDraft:
      definition.type === "resourceLocator"
        ? createResourceLocatorDraft(
            definition.name,
            value,
            definition.resourceLocatorModes,
          )
        : undefined,
    childFields: childDefinitions.map((childDefinition) =>
      createParameterField(
        childDefinition,
        nestedValues,
        [
          ...nestedValuePath,
          childDefinition.name,
        ],
        rootValues,
      ),
    ),
    collectionItems: collectionItemsFromDefinition(
      definition,
      value,
      valuePath,
      rootValues,
    ),
    renderingBoundary: definition.renderingBoundary,
  };
}

function visibleChildDefinitions(
  childDefinitions: NodeParameterDefinition[],
  values: Record<string, unknown>,
  rootValues: Record<string, unknown>,
) {
  const visibilityValues = {
    ...rootValues,
    ...values,
  };

  return childDefinitions.filter((childDefinition) =>
    fieldVisible(childDefinition, visibilityValues),
  );
}
