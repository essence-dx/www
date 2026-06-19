import type { NodeTypeDescription } from "../node-types/types";
import {
  applyN8nDisplayOptions,
  credentialsFromVersionDescription,
  sourceEntriesForArray,
  sourceParametersFromEntries,
  sourceParametersFromVersionDescription,
} from "./source-description-reader";
import { readConstAssignmentBlock } from "./source-blocks";
import {
  normalizeDiscordSendAndWaitOperationConstant,
  sourceParametersFromDiscordSendAndWaitOperation,
} from "./discord-send-and-wait";
import type { SourceParameterExtraction } from "./types";

const sourcePath = "nodes/Discord/v2/actions/versionDescription.ts";

export type DiscordSourceBundle = {
  versionDescriptionSource: string;
  commonDescriptionSource: string;
  descriptionUtilitiesSource: string;
  sendAndWaitUtilitiesSource: string;
  sendAndWaitDescriptionsSource: string;
  formNodeSource: string;
  formCommonDescriptionSource: string;
  channelResourceSource: string;
  messageResourceSource: string;
  memberResourceSource: string;
  channelCreateOperationSource: string;
  channelDeleteOperationSource: string;
  channelGetOperationSource: string;
  channelGetAllOperationSource: string;
  channelUpdateOperationSource: string;
  messageDeleteOperationSource: string;
  messageGetOperationSource: string;
  messageGetAllOperationSource: string;
  messageReactOperationSource: string;
  messageSendOperationSource: string;
  messageSendAndWaitOperationSource: string;
  memberGetAllOperationSource: string;
  memberRoleAddOperationSource: string;
  memberRoleRemoveOperationSource: string;
};

const sharedObjectNames = [
  "categoryRLC",
  "channelRLC",
  "guildRLC",
  "messageIdString",
  "roleMultiOptions",
  "simplifyBoolean",
  "textChannelRLC",
  "userRLC",
  "embedsFixedCollection",
  "filesFixedCollection",
];
const sharedArraySpreadNames = ["sendToProperties"];
const localArraySpreadNames = ["returnAllOrLimit"];

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
  descriptionUtilitiesSource: string,
) {
  const propertiesBlock = readConstAssignmentBlock(operationSource, "properties", "[");
  const displayOptionsBlock = readConstAssignmentBlock(
    operationSource,
    "displayOptions",
    "{",
  );
  const entries = sourceEntriesForArray({
    source: `const properties = ${propertiesBlock};\n${descriptionUtilitiesSource}`,
    arrayName: "properties",
    sharedSource,
    sharedObjectNames,
    sharedArraySpreadNames,
    localArraySpreadNames,
  }).map((entry) => applyN8nDisplayOptions(entry, displayOptionsBlock));

  return sourceParametersFromEntries(entries);
}

function sourceParametersFromOperations(sourceBundle: DiscordSourceBundle) {
  return [
    sourceBundle.channelCreateOperationSource,
    sourceBundle.channelDeleteOperationSource,
    sourceBundle.channelGetOperationSource,
    sourceBundle.channelGetAllOperationSource,
    sourceBundle.channelUpdateOperationSource,
    sourceBundle.messageDeleteOperationSource,
    sourceBundle.messageGetOperationSource,
    sourceBundle.messageGetAllOperationSource,
    sourceBundle.messageReactOperationSource,
    sourceBundle.messageSendOperationSource,
    sourceBundle.memberGetAllOperationSource,
    sourceBundle.memberRoleAddOperationSource,
    sourceBundle.memberRoleRemoveOperationSource,
  ].flatMap((operationSource) =>
    sourceParametersFromOperation(
      operationSource,
      sourceBundle.commonDescriptionSource,
      sourceBundle.descriptionUtilitiesSource,
    ),
  );
}

function sourceParameters(sourceBundle: DiscordSourceBundle) {
  return [
    ...sourceParametersFromVersionDescription(
      sourceBundle.versionDescriptionSource,
    ),
    ...sourceParametersFromResource(
      sourceBundle.channelResourceSource,
      sourceBundle.commonDescriptionSource,
    ),
    ...sourceParametersFromResource(
      normalizeDiscordSendAndWaitOperationConstant(
        sourceBundle.messageResourceSource,
      ),
      sourceBundle.commonDescriptionSource,
    ),
    ...sourceParametersFromResource(
      sourceBundle.memberResourceSource,
      sourceBundle.commonDescriptionSource,
    ),
    ...sourceParametersFromOperations(sourceBundle),
    ...sourceParametersFromDiscordSendAndWaitOperation({
      sourceBundle,
      sharedObjectNames,
    }),
  ];
}

export function extractDiscordSourceParameters(
  sourceBundle: DiscordSourceBundle,
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

export function createDiscordNodeTypeFromSource(
  sourceBundle: DiscordSourceBundle,
): NodeTypeDescription {
  const extraction = extractDiscordSourceParameters(sourceBundle);

  return {
    name: "n8n-nodes-base.discord",
    displayName: "Discord",
    sourcePath,
    version: 2,
    credentials: credentialsFromVersionDescription(
      sourceBundle.versionDescriptionSource,
    ),
    properties: extraction.parameters,
    categories: ["Communication", "HITL"],
    aliases: ["human", "form", "wait", "hitl", "approval"],
    authKinds: ["apiKey", "oAuth2", "custom"],
    sourceProvenance: "n8n-nodes-base",
    workflowNode: {
      ready: true,
      trigger: false,
      usable_as_tool: true,
      run_mode: "credential-gated",
    },
  };
}
