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

const sourcePath = "nodes/Postgres/v2/actions/versionDescription.ts";

export type PostgresSourceBundle = {
  versionDescriptionSource: string;
  commonDescriptionSource: string;
  databaseResourceSource: string;
  deleteTableOperationSource: string;
  executeQueryOperationSource: string;
  insertOperationSource: string;
  selectOperationSource: string;
  updateOperationSource: string;
  upsertOperationSource: string;
};

const sharedObjectNames = [
  "schemaRLC",
  "tableRLC",
  "optionsCollection",
  "whereFixedCollection",
  "sortFixedCollection",
  "combineConditionsCollection",
];
const sharedArrayPropertyNames = ["operatorOptions"];

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
      sharedArrayPropertyNames,
    }),
  );
}

function sourceParametersFromOperation(
  operationSource: string,
  sharedSource: string,
) {
  const displayOptionsBlock = readConstAssignmentBlock(
    operationSource,
    "displayOptions",
    "{",
  );
  const entries = sourceEntriesForArray({
    source: operationSource,
    arrayName: "properties",
    sharedSource,
    sharedObjectNames,
    sharedArrayPropertyNames,
  }).map((entry) => applyN8nDisplayOptions(entry, displayOptionsBlock));

  return sourceParametersFromEntries(entries);
}

function sourceParameters(sourceBundle: PostgresSourceBundle) {
  const operationSources = [
    sourceBundle.deleteTableOperationSource,
    sourceBundle.executeQueryOperationSource,
    sourceBundle.insertOperationSource,
    sourceBundle.selectOperationSource,
    sourceBundle.updateOperationSource,
    sourceBundle.upsertOperationSource,
  ];

  return [
    ...sourceParametersFromVersionDescription(sourceBundle.versionDescriptionSource),
    ...sourceParametersFromResource(
      sourceBundle.databaseResourceSource,
      sourceBundle.commonDescriptionSource,
    ),
    ...operationSources.flatMap((source) =>
      sourceParametersFromOperation(source, sourceBundle.commonDescriptionSource),
    ),
  ];
}

export function extractPostgresSourceParameters(
  sourceBundle: PostgresSourceBundle,
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

export function createPostgresNodeTypeFromSource(
  sourceBundle: PostgresSourceBundle,
): NodeTypeDescription {
  const extraction = extractPostgresSourceParameters(sourceBundle);

  return {
    name: "n8n-nodes-base.postgres",
    displayName: "Postgres",
    sourcePath,
    version: 2.6,
    credentials: credentialsFromVersionDescription(
      sourceBundle.versionDescriptionSource,
    ),
    properties: extraction.parameters,
    categories: ["Data & Storage"],
    authKinds: ["database"],
    sourceProvenance: "n8n-nodes-base",
    workflowNode: {
      ready: true,
      trigger: false,
      usable_as_tool: true,
      run_mode: "regular",
    },
  };
}
