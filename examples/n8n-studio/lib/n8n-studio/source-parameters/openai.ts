import type { NodeParameterDefinition, NodeTypeDescription } from "../node-types/types";
import {
  sourceEntriesForArray,
  sourceParametersFromEntries,
} from "./source-description-reader";
import type { SourceParameterExtraction } from "./types";

const sourcePath = "nodes/OpenAi/OpenAi.node.ts";

export type OpenAiSourceBundle = {
  chatSource: string;
  textSource: string;
  imageSource: string;
};

function resourceParameter(): NodeParameterDefinition {
  return {
    name: "resource",
    label: "Resource",
    type: "options",
    defaultValue: "text",
    noDataExpression: true,
    options: [
      { name: "Chat", value: "chat" },
      { name: "Image", value: "image" },
      { name: "Text", value: "text" },
    ],
  };
}

function sourceParametersForArrays(source: string, arrayNames: string[]) {
  return sourceParametersFromEntries(
    arrayNames.flatMap((arrayName) =>
      sourceEntriesForArray({
        source,
        arrayName,
        localArraySpreadNames: [
          "completeOperations",
          "sharedOperations",
          "createOperations",
          "editOperations",
          "moderateOperations",
        ],
      }),
    ),
  );
}

function sourceParameters(sourceBundle: OpenAiSourceBundle) {
  return [
    resourceParameter(),
    ...sourceParametersForArrays(sourceBundle.chatSource, [
      "chatOperations",
      "chatFields",
    ]),
    ...sourceParametersForArrays(sourceBundle.imageSource, [
      "imageOperations",
      "imageFields",
    ]),
    ...sourceParametersForArrays(sourceBundle.textSource, [
      "textOperations",
      "textFields",
    ]),
  ];
}

export function extractOpenAiSourceParameters(
  sourceBundle: OpenAiSourceBundle,
): SourceParameterExtraction {
  const parameters = sourceParameters(sourceBundle);

  return {
    sourcePath,
    generatedFrom: "source-parameter-description",
    parameters,
    unsupportedParameterNames: parameters
      .filter((parameter) => parameter.renderingBoundary === "complex-source-field")
      .map((parameter) => parameter.name),
  };
}

export function createOpenAiNodeTypeFromSource(
  sourceBundle: OpenAiSourceBundle,
): NodeTypeDescription {
  const extraction = extractOpenAiSourceParameters(sourceBundle);

  return {
    name: "n8n-nodes-base.openAi",
    displayName: "OpenAI",
    sourcePath,
    version: 1.1,
    credentials: [{ name: "openAiApi", required: true }],
    properties: extraction.parameters,
    categories: ["AI"],
    authKinds: ["apiKey"],
    sourceProvenance: "n8n-nodes-base",
    workflowNode: {
      ready: true,
      trigger: false,
      usable_as_tool: true,
      run_mode: "regular",
    },
  };
}
