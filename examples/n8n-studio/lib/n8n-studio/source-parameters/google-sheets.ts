import type { NodeTypeDescription } from "../node-types/types";
import {
  credentialsFromVersionDescription,
  sourceEntriesForArray,
  sourceParametersFromEntries,
  sourceParametersFromVersionDescription,
} from "./source-description-reader";
import type { SourceParameterExtraction } from "./types";

const sourcePath = "nodes/Google/Sheet/v2/actions/versionDescription.ts";

export type GoogleSheetsSourceBundle = {
  versionDescriptionSource: string;
  sheetResourceSource: string;
  spreadsheetResourceSource: string;
  commonDescriptionSource: string;
  appendOperationSource: string;
  appendOrUpdateOperationSource: string;
  clearOperationSource: string;
  createSheetOperationSource: string;
  deleteOperationSource: string;
  readOperationSource: string;
  updateOperationSource: string;
  createSpreadsheetOperationSource: string;
  deleteSpreadsheetOperationSource: string;
};

const sharedParameterNames = [
  "dataLocationOnSheet",
  "outputFormatting",
  "cellFormat",
  "handlingExtraData",
  "useAppendOption",
  "locationDefine",
];

const localObjectNames = ["readFilter", "combineFiltersOptions"];

function sourceParametersForArray(
  source: string,
  arrayName: string,
  sharedSource: string,
) {
  return sourceParametersFromEntries(
    sourceEntriesForArray({
      source,
      arrayName,
      sharedSource,
      sharedObjectNames: sharedParameterNames,
      localObjectNames,
    }),
  );
}

function sourceParameters(sourceBundle: GoogleSheetsSourceBundle) {
  const operationSources = [
    sourceBundle.appendOperationSource,
    sourceBundle.appendOrUpdateOperationSource,
    sourceBundle.clearOperationSource,
    sourceBundle.createSheetOperationSource,
    sourceBundle.deleteOperationSource,
    sourceBundle.readOperationSource,
    sourceBundle.updateOperationSource,
    sourceBundle.createSpreadsheetOperationSource,
    sourceBundle.deleteSpreadsheetOperationSource,
  ];

  return [
    ...sourceParametersFromVersionDescription(sourceBundle.versionDescriptionSource, {
      leadingConstObjectNames: ["authentication"],
    }),
    ...sourceParametersForArray(
      sourceBundle.sheetResourceSource,
      "descriptions",
      sourceBundle.commonDescriptionSource,
    ),
    ...sourceParametersForArray(
      sourceBundle.spreadsheetResourceSource,
      "descriptions",
      sourceBundle.commonDescriptionSource,
    ),
    ...operationSources.flatMap((source) =>
      sourceParametersForArray(
        source,
        "description",
        sourceBundle.commonDescriptionSource,
      ),
    ),
  ];
}

export function extractGoogleSheetsSourceParameters(
  sourceBundle: GoogleSheetsSourceBundle,
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

export function createGoogleSheetsNodeTypeFromSource(
  sourceBundle: GoogleSheetsSourceBundle,
): NodeTypeDescription {
  const extraction = extractGoogleSheetsSourceParameters(sourceBundle);

  return {
    name: "n8n-nodes-base.googleSheets",
    displayName: "Google Sheets",
    sourcePath,
    version: 4.7,
    credentials: credentialsFromVersionDescription(
      sourceBundle.versionDescriptionSource,
    ),
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
