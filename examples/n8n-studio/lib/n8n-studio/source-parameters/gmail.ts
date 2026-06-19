import type { NodeTypeDescription } from "../node-types/types";
import {
  credentialsFromVersionDescription,
  sourceEntriesForArray,
  sourceParametersFromEntries,
  sourceParametersFromVersionDescription,
} from "./source-description-reader";
import type { SourceParameterExtraction } from "./types";

const sourcePath = "nodes/Google/Gmail/v2/GmailV2.node.ts";

export type GmailSourceBundle = {
  gmailNodeSource: string;
  messageSource: string;
  draftSource: string;
  labelSource: string;
  threadSource: string;
};

function sourceParametersForArrays(source: string, arrayNames: string[]) {
  return sourceParametersFromEntries(
    arrayNames.flatMap((arrayName) =>
      sourceEntriesForArray({
        source,
        arrayName,
        localArraySpreadNames: [
          "messageOperations",
          "messageFields",
          "labelOperations",
          "labelFields",
          "draftOperations",
          "draftFields",
          "threadOperations",
          "threadFields",
        ],
      }),
    ),
  );
}

function sourceParameters(sourceBundle: GmailSourceBundle) {
  return [
    ...sourceParametersFromVersionDescription(sourceBundle.gmailNodeSource),
    ...sourceParametersForArrays(sourceBundle.messageSource, [
      "messageOperations",
      "messageFields",
    ]),
    ...sourceParametersForArrays(sourceBundle.labelSource, [
      "labelOperations",
      "labelFields",
    ]),
    ...sourceParametersForArrays(sourceBundle.draftSource, [
      "draftOperations",
      "draftFields",
    ]),
    ...sourceParametersForArrays(sourceBundle.threadSource, [
      "threadOperations",
      "threadFields",
    ]),
  ];
}

export function extractGmailSourceParameters(
  sourceBundle: GmailSourceBundle,
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

export function createGmailNodeTypeFromSource(
  sourceBundle: GmailSourceBundle,
): NodeTypeDescription {
  const extraction = extractGmailSourceParameters(sourceBundle);

  return {
    name: "n8n-nodes-base.gmail",
    displayName: "Gmail",
    sourcePath,
    version: 2.2,
    credentials: credentialsFromVersionDescription(sourceBundle.gmailNodeSource),
    properties: extraction.parameters,
    categories: ["Productivity"],
    authKinds: ["oAuth2", "serviceAccount"],
    sourceProvenance: "n8n-nodes-base",
    workflowNode: {
      ready: true,
      trigger: false,
      usable_as_tool: true,
      run_mode: "regular",
    },
  };
}
