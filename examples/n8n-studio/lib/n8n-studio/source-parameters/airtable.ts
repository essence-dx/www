import type { NodeTypeDescription } from "../node-types/types";
import {
  applyN8nDisplayOptions,
  credentialsFromVersionDescription,
  sourceEntriesForArray,
  sourceParametersFromEntries,
  sourceParametersFromVersionDescription,
} from "./source-description-reader";
import { readConstAssignmentBlock } from "./source-blocks";
import type { SourceParameterExtraction } from "./types";

const sourcePath = "nodes/Airtable/v2/actions/versionDescription.ts";

export type AirtableSourceBundle = {
  versionDescriptionSource: string;
  commonDescriptionSource: string;
  baseResourceSource: string;
  recordResourceSource: string;
  recordCreateOperationSource: string;
  recordDeleteOperationSource: string;
  recordGetOperationSource: string;
  recordSearchOperationSource: string;
  recordUpdateOperationSource: string;
  recordUpsertOperationSource: string;
  baseGetManyOperationSource: string;
  baseGetSchemaOperationSource: string;
};

const sharedObjectNames = ["baseRLC", "tableRLC", "viewRLC"];
const sharedArraySpreadNames = ["insertUpdateOptions"];

function sourceParametersFromResource(
  resourceSource: string,
  sharedSource: string,
) {
  return sourceParametersFromEntries(
    sourceEntriesForArray({
      source: resourceSource,
      arrayName: "description",
      sharedSource,
      sharedObjectNames,
    }),
  );
}

function sourceParametersFromOperation(
  operationSource: string,
  sharedSource: string,
) {
  const propertiesBlock = readConstAssignmentBlock(operationSource, "properties", "[");
  const displayOptionsBlock = readConstAssignmentBlock(
    operationSource,
    "displayOptions",
    "{",
  );
  const entries = sourceEntriesForArray({
    source: `const properties = ${propertiesBlock};`,
    arrayName: "properties",
    sharedSource,
    sharedObjectNames,
    sharedArraySpreadNames,
  }).map((entry) => applyN8nDisplayOptions(entry, displayOptionsBlock));

  return sourceParametersFromEntries(entries);
}

function sourceParameters(sourceBundle: AirtableSourceBundle) {
  const operationSources = [
    sourceBundle.recordCreateOperationSource,
    sourceBundle.recordDeleteOperationSource,
    sourceBundle.recordGetOperationSource,
    sourceBundle.recordSearchOperationSource,
    sourceBundle.recordUpdateOperationSource,
    sourceBundle.recordUpsertOperationSource,
    sourceBundle.baseGetManyOperationSource,
    sourceBundle.baseGetSchemaOperationSource,
  ];

  return [
    ...sourceParametersFromVersionDescription(sourceBundle.versionDescriptionSource),
    ...sourceParametersFromResource(
      sourceBundle.baseResourceSource,
      sourceBundle.commonDescriptionSource,
    ),
    ...sourceParametersFromResource(
      sourceBundle.recordResourceSource,
      sourceBundle.commonDescriptionSource,
    ),
    ...operationSources.flatMap((source) =>
      sourceParametersFromOperation(source, sourceBundle.commonDescriptionSource),
    ),
  ];
}

export function extractAirtableSourceParameters(
  sourceBundle: AirtableSourceBundle,
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

export function createAirtableNodeTypeFromSource(
  sourceBundle: AirtableSourceBundle,
): NodeTypeDescription {
  const extraction = extractAirtableSourceParameters(sourceBundle);

  return {
    name: "n8n-nodes-base.airtable",
    displayName: "Airtable",
    sourcePath,
    version: 2.2,
    credentials: credentialsFromVersionDescription(
      sourceBundle.versionDescriptionSource,
    ),
    properties: extraction.parameters,
    categories: ["Data & Storage"],
    authKinds: ["apiKey", "oAuth2"],
    sourceProvenance: "n8n-nodes-base",
    workflowNode: {
      ready: true,
      trigger: false,
      usable_as_tool: true,
      run_mode: "regular",
    },
  };
}
