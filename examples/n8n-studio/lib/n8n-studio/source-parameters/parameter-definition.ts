import type { NodeParameterDefinition } from "../node-types/types";
import { sourcePropertyReader } from "./property-reader";
import { splitTopLevelEntries } from "./source-blocks";

const supportedSourceTypes: Record<string, NodeParameterDefinition["type"]> = {
  boolean: "boolean",
  collection: "collection",
  color: "color",
  credentials: "credentialsSelect",
  credentialsSelect: "credentialsSelect",
  curlImport: "curlImport",
  fixedCollection: "fixedCollection",
  json: "json",
  multiOptions: "multiOptions",
  notice: "notice",
  number: "number",
  options: "options",
  resourceLocator: "resourceLocator",
  resourceMapper: "resourceMapper",
  string: "string",
};

function createChildDefinition(source: string) {
  return createParameterDefinitionFromSource(source);
}

function childParametersFromCollection(source: string) {
  const optionsBlock = sourcePropertyReader.readPropertyBlock(source, "options", "[");
  if (!optionsBlock) {
    return undefined;
  }

  const childParameters = splitTopLevelEntries(optionsBlock)
    .filter((entry) => entry.startsWith("{"))
    .map(createChildDefinition)
    .filter((parameter): parameter is NodeParameterDefinition => Boolean(parameter));

  return childParameters.length > 0 ? childParameters : undefined;
}

function childParametersFromFixedCollection(source: string) {
  const optionsBlock = sourcePropertyReader.readPropertyBlock(source, "options", "[");
  if (!optionsBlock) {
    return undefined;
  }

  const childParameters = splitTopLevelEntries(optionsBlock).flatMap((entry) => {
    if (!entry.startsWith("{")) {
      return [];
    }

    const valuesBlock = sourcePropertyReader.readPropertyBlock(entry, "values", "[");
    if (!valuesBlock) {
      const child = createChildDefinition(entry);
      return child ? [child] : [];
    }

    return splitTopLevelEntries(valuesBlock)
      .filter((valueEntry) => valueEntry.startsWith("{"))
      .map(createChildDefinition)
      .filter((parameter): parameter is NodeParameterDefinition => Boolean(parameter));
  });

  return childParameters.length > 0 ? childParameters : undefined;
}

function childParametersFromSource(
  source: string,
  type: NodeParameterDefinition["type"],
) {
  if (type === "collection") {
    return childParametersFromCollection(source);
  }
  if (type === "fixedCollection") {
    return childParametersFromFixedCollection(source);
  }
  return undefined;
}

export function createParameterDefinitionFromSource(
  source: string,
): NodeParameterDefinition | undefined {
  const name = sourcePropertyReader.stringProperty(source, "name");
  const label = sourcePropertyReader.stringProperty(source, "displayName");
  const sourceType = sourcePropertyReader.stringProperty(source, "type");

  if (!name || label === undefined || !sourceType) {
    return undefined;
  }

  const type = supportedSourceTypes[sourceType] ?? "string";
  const renderingBoundary =
    type === "collection" || type === "fixedCollection" || type === "resourceMapper"
      ? "complex-source-field"
      : "native";
  const childParameters = childParametersFromSource(source, type);

  return {
    name,
    label,
    type,
    defaultValue: sourcePropertyReader.literalDefault(source),
    required: sourcePropertyReader.booleanProperty(source, "required"),
    noDataExpression: sourcePropertyReader.booleanProperty(source, "noDataExpression"),
    description: sourcePropertyReader.stringProperty(source, "description"),
    placeholder: sourcePropertyReader.stringProperty(source, "placeholder"),
    options: sourcePropertyReader.readOptions(source),
    credentialTypes: sourcePropertyReader.readStringArrayProperty(source, "credentialTypes"),
    displayOptions: sourcePropertyReader.readDisplayOptions(source),
    dynamicOptions: sourcePropertyReader.readDynamicOptions(source),
    resourceMapper: sourcePropertyReader.readResourceMapperBoundary(source),
    resourceLocatorModes: sourcePropertyReader.readResourceLocatorModes(source),
    childParameters,
    renderingBoundary,
  };
}
