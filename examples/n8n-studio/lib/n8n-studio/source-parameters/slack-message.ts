import type { NodeParameterDefinition, NodeTypeDescription } from "../node-types/types";
import { createParameterDefinitionFromSource } from "./parameter-definition";
import { readExportedAssignmentBlock, splitTopLevelEntries } from "./source-blocks";
import type { SourceParameterExtraction } from "./types";

const sourcePath = "nodes/Slack/V2/MessageDescription.ts";

function sourceParameterMap(source: string) {
  const map = new Map<string, string>();
  for (const name of ["sendToSelector", "channelRLC", "userRLC"]) {
    map.set(name, readExportedAssignmentBlock(source, name, "{"));
  }
  return map;
}

function mergeSpreadParameter(entry: string, sourceObjects: Map<string, string>) {
  const spreadMatch = entry.match(/\.\.\.(\w+)/);
  if (!spreadMatch) {
    return entry;
  }

  const base = sourceObjects.get(spreadMatch[1]);
  if (!base) {
    return entry;
  }

  const override = entry.replace(/\.\.\.\w+\s*,?/, "").replace(/^\{/, "").replace(/\}$/, "");
  return `${base.slice(0, -1)},${override}}`;
}

function resolveSourceEntry(entry: string, sourceObjects: Map<string, string>) {
  const identifierMatch = entry.match(/\b(sendToSelector|channelRLC|userRLC)\b/);
  if (identifierMatch && !entry.includes("...")) {
    return sourceObjects.get(identifierMatch[1]) ?? entry;
  }

  return mergeSpreadParameter(entry, sourceObjects);
}

function operationParameter(source: string): NodeParameterDefinition {
  const operationsBlock = readExportedAssignmentBlock(source, "messageOperations", "[");
  const operationSource = splitTopLevelEntries(operationsBlock).find((entry) =>
    entry.includes("name: 'operation'"),
  );
  const operation = operationSource
    ? createParameterDefinitionFromSource(operationSource)
    : undefined;

  return {
    name: "operation",
    label: "Operation",
    type: "options",
    defaultValue: "post",
    noDataExpression: true,
    options: operation?.options,
    displayOptions: {
      show: {
        resource: ["message"],
      },
    },
  };
}

function sourceParameters(source: string) {
  const sourceObjects = sourceParameterMap(source);
  const fieldsBlock = readExportedAssignmentBlock(source, "messageFields", "[");
  const parameters = splitTopLevelEntries(fieldsBlock)
    .map((entry) => resolveSourceEntry(entry, sourceObjects))
    .map(createParameterDefinitionFromSource)
    .filter((parameter): parameter is NodeParameterDefinition => Boolean(parameter))
    .filter((parameter) => {
      const show = parameter.displayOptions?.show;
      return show?.resource?.includes("message") && show?.operation?.includes("post");
    });

  return [
    {
      name: "resource",
      label: "Resource",
      type: "options",
      defaultValue: "message",
      noDataExpression: true,
      options: [{ name: "Message", value: "message" }],
    } satisfies NodeParameterDefinition,
    operationParameter(source),
    ...parameters,
  ];
}

export function extractSlackMessageSourceParameters(
  source: string,
): SourceParameterExtraction {
  const parameters = sourceParameters(source);

  return {
    sourcePath,
    generatedFrom: "source-parameter-description",
    parameters,
    unsupportedParameterNames: parameters
      .filter((parameter) => parameter.renderingBoundary === "complex-source-field")
      .map((parameter) => parameter.name),
  };
}

export function createSlackMessageNodeTypeFromSource(source: string): NodeTypeDescription {
  const extraction = extractSlackMessageSourceParameters(source);

  return {
    name: "n8n-nodes-base.slack",
    displayName: "Slack",
    sourcePath,
    version: 2,
    credentials: [
      { name: "slackApi", required: true },
      { name: "slackOAuth2Api", required: true },
    ],
    properties: extraction.parameters,
    sourceProvenance: "n8n-nodes-base",
  };
}
