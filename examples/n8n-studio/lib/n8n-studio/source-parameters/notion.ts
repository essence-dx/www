import type { NodeTypeDescription } from "../node-types/types";
import {
  credentialsFromVersionDescription,
  sourceEntriesForArray,
  sourceParametersFromEntries,
  sourceParametersFromVersionDescription,
} from "./source-description-reader";
import { sourceEntriesFromNotionBlockCalls } from "./notion-blocks";
import { sourceEntriesFromNotionSearchFilterCalls } from "./notion-filters";
import type { SourceParameterExtraction } from "./types";

const sourcePath = "nodes/Notion/v2/VersionDescription.ts";

export type NotionV2SourceBundle = {
  versionDescriptionSource: string;
  blockDescriptionSource: string;
  databaseDescriptionSource: string;
  databasePageDescriptionSource: string;
  pageDescriptionSource: string;
  userDescriptionSource: string;
  blocksDescriptionSource: string;
  filtersDescriptionSource: string;
  genericFunctionsSource: string;
};

function sourceParametersForArrays(
  source: string,
  arrayNames: string[],
  localObjectNames: string[] = [],
) {
  return sourceParametersFromEntries(
    arrayNames.flatMap((arrayName) =>
      sourceEntriesForArray({
        source,
        arrayName,
        localObjectNames,
      }),
    ),
  );
}

function sourceParameters(sourceBundle: NotionV2SourceBundle) {
  return [
    ...sourceParametersFromVersionDescription(
      sourceBundle.versionDescriptionSource,
    ),
    ...sourceParametersForArrays(
      sourceBundle.blockDescriptionSource,
      ["blockOperations", "blockFields"],
      ["blockIdRLC"],
    ),
    ...sourceParametersForArrays(sourceBundle.databaseDescriptionSource, [
      "databaseOperations",
      "databaseFields",
    ]),
    ...sourceParametersForArrays(sourceBundle.databasePageDescriptionSource, [
      "databasePageOperations",
      "databasePageFields",
    ]),
    ...sourceParametersForArrays(sourceBundle.pageDescriptionSource, [
      "pageOperations",
      "pageFields",
    ]),
    ...sourceParametersForArrays(sourceBundle.userDescriptionSource, [
      "userOperations",
      "userFields",
    ]),
    ...sourceParametersFromEntries(
      sourceEntriesFromNotionBlockCalls(sourceBundle.blocksDescriptionSource, [
        sourceBundle.blockDescriptionSource,
        sourceBundle.databasePageDescriptionSource,
        sourceBundle.pageDescriptionSource,
      ]),
    ),
    ...sourceParametersFromEntries(
      sourceEntriesFromNotionSearchFilterCalls(
        sourceBundle.genericFunctionsSource,
        sourceBundle.filtersDescriptionSource,
        [
          sourceBundle.databasePageDescriptionSource,
        ],
      ),
    ),
  ];
}

export function extractNotionV2SourceParameters(
  sourceBundle: NotionV2SourceBundle,
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

export function createNotionV2NodeTypeFromSource(
  sourceBundle: NotionV2SourceBundle,
): NodeTypeDescription {
  const extraction = extractNotionV2SourceParameters(sourceBundle);

  return {
    name: "n8n-nodes-base.notionV2",
    displayName: "Notion V2",
    sourcePath,
    version: 2.2,
    credentials: credentialsFromVersionDescription(
      sourceBundle.versionDescriptionSource,
    ),
    properties: extraction.parameters,
    categories: ["Productivity"],
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
