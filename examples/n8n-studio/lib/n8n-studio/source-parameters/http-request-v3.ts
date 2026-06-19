import type { NodeParameterDefinition, NodeTypeDescription } from "../node-types/types";
import { createParameterDefinitionFromSource } from "./parameter-definition";
import { readExportedAssignmentBlock, splitTopLevelEntries } from "./source-blocks";
import type { SourceParameterExtraction } from "./types";

const sourcePath = "nodes/HttpRequest/V3/Description.ts";

export type HttpRequestV3SourceBundle = {
  descriptionSource: string;
  optimizerSource?: string;
};

function parseParameterArray(block: string) {
  return splitTopLevelEntries(block)
    .filter((entry) => entry.startsWith("{"))
    .map(createParameterDefinitionFromSource)
    .filter((parameter): parameter is NodeParameterDefinition => Boolean(parameter));
}

function addToolDisplayOption(parameter: NodeParameterDefinition): NodeParameterDefinition {
  return {
    ...parameter,
    displayOptions: {
      ...parameter.displayOptions,
      show: {
        ...parameter.displayOptions?.show,
        "@tool": [true],
      },
    },
  };
}

function optimizerParameters(source: string | undefined) {
  if (!source) {
    return [];
  }

  const block = readExportedAssignmentBlock(source, "optimizeResponseProperties", "[");
  return parseParameterArray(block).map(addToolDisplayOption);
}

function mainParameters(source: string) {
  const block = readExportedAssignmentBlock(source, "mainProperties", "[");
  return parseParameterArray(block);
}

function sourceParameters(sourceBundle: HttpRequestV3SourceBundle) {
  return [
    ...mainParameters(sourceBundle.descriptionSource),
    ...optimizerParameters(sourceBundle.optimizerSource),
  ];
}

export function extractHttpRequestV3SourceParameters(
  sourceBundle: HttpRequestV3SourceBundle,
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

export function createHttpRequestV3NodeTypeFromSource(
  sourceBundle: HttpRequestV3SourceBundle,
): NodeTypeDescription {
  const extraction = extractHttpRequestV3SourceParameters(sourceBundle);

  return {
    name: "n8n-nodes-base.httpRequest",
    displayName: "HTTP Request",
    sourcePath,
    version: 4,
    credentials: [
      { name: "httpBasicAuth", required: false },
      { name: "httpBearerAuth", required: false },
      { name: "httpDigestAuth", required: false },
      { name: "httpHeaderAuth", required: false },
      { name: "httpQueryAuth", required: false },
      { name: "httpCustomAuth", required: false },
      { name: "httpSslAuth", required: false },
      { name: "oAuth1Api", required: false },
      { name: "oAuth2Api", required: false },
    ],
    properties: extraction.parameters,
    sourceProvenance: "n8n-nodes-base",
  };
}
