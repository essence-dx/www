import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import {
  createAirtableNodeTypeFromSource,
  createDiscordNodeTypeFromSource,
  createGmailNodeTypeFromSource,
  createGoogleSheetsNodeTypeFromSource,
  createHttpRequestV3NodeTypeFromSource,
  createNotionV2NodeTypeFromSource,
  createOpenAiNodeTypeFromSource,
  createPostgresNodeTypeFromSource,
  createSlackMessageNodeTypeFromSource,
  extractAirtableSourceParameters,
  extractDiscordSourceParameters,
  extractGmailSourceParameters,
  extractGoogleSheetsSourceParameters,
  extractHttpRequestV3SourceParameters,
  extractNotionV2SourceParameters,
  extractOpenAiSourceParameters,
  extractPostgresSourceParameters,
  extractSlackMessageSourceParameters,
} from "../examples/n8n-studio/lib/n8n-studio/source-parameters";
import { createParameterSchemaForNode } from "../examples/n8n-studio/lib/n8n-studio/node-type-registry";

const repoRoot = path.resolve(import.meta.dirname, "..");
const slackMessageSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "Slack",
  "V2",
  "MessageDescription.ts",
);
const httpRequestDescriptionSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "HttpRequest",
  "V3",
  "Description.ts",
);
const httpRequestOptimizerSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "HttpRequest",
  "shared",
  "optimizeResponse.ts",
);
const openAiChatSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "OpenAi",
  "ChatDescription.ts",
);
const openAiTextSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "OpenAi",
  "TextDescription.ts",
);
const openAiImageSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "OpenAi",
  "ImageDescription.ts",
);
const gmailNodeSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "Google",
  "Gmail",
  "v2",
  "GmailV2.node.ts",
);
const gmailMessageSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "Google",
  "Gmail",
  "v2",
  "MessageDescription.ts",
);
const gmailDraftSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "Google",
  "Gmail",
  "v2",
  "DraftDescription.ts",
);
const gmailLabelSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "Google",
  "Gmail",
  "v2",
  "LabelDescription.ts",
);
const gmailThreadSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "Google",
  "Gmail",
  "v2",
  "ThreadDescription.ts",
);
const n8nDescriptionUtilitiesSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "utils",
  "descriptions.ts",
);
const n8nSendAndWaitUtilitiesSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "utils",
  "sendAndWait",
  "utils.ts",
);
const n8nSendAndWaitDescriptionsSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "utils",
  "sendAndWait",
  "descriptions.ts",
);
const n8nFormNodeSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "Form",
  "Form.node.ts",
);
const n8nFormCommonDescriptionSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "Form",
  "common.descriptions.ts",
);
function discordActionSourcePath(...segments: string[]) {
  return path.join(
    repoRoot,
    "integrations",
    "n8n-nodes-base",
    "nodes",
    "Discord",
    "v2",
    "actions",
    ...segments,
  );
}

const discordVersionDescriptionSourcePath = discordActionSourcePath(
  "versionDescription.ts",
);
const discordCommonDescriptionSourcePath = discordActionSourcePath(
  "common.description.ts",
);
const discordMessageResourceSourcePath = discordActionSourcePath(
  "message",
  "index.ts",
);
const discordChannelResourceSourcePath = discordActionSourcePath(
  "channel",
  "index.ts",
);
const discordMemberResourceSourcePath = discordActionSourcePath(
  "member",
  "index.ts",
);
const discordMessageSendOperationSourcePath = discordActionSourcePath(
  "message",
  "send.operation.ts",
);
const discordMessageSendAndWaitOperationSourcePath = discordActionSourcePath(
  "message",
  "sendAndWait.operation.ts",
);
const discordMessageDeleteOperationSourcePath = discordActionSourcePath(
  "message",
  "deleteMessage.operation.ts",
);
const discordMessageGetOperationSourcePath = discordActionSourcePath(
  "message",
  "get.operation.ts",
);
const discordMessageGetAllOperationSourcePath = discordActionSourcePath(
  "message",
  "getAll.operation.ts",
);
const discordMessageReactOperationSourcePath = discordActionSourcePath(
  "message",
  "react.operation.ts",
);
const discordChannelCreateOperationSourcePath = discordActionSourcePath(
  "channel",
  "create.operation.ts",
);
const discordChannelDeleteOperationSourcePath = discordActionSourcePath(
  "channel",
  "deleteChannel.operation.ts",
);
const discordChannelGetOperationSourcePath = discordActionSourcePath(
  "channel",
  "get.operation.ts",
);
const discordChannelGetAllOperationSourcePath = discordActionSourcePath(
  "channel",
  "getAll.operation.ts",
);
const discordChannelUpdateOperationSourcePath = discordActionSourcePath(
  "channel",
  "update.operation.ts",
);
const discordMemberGetAllOperationSourcePath = discordActionSourcePath(
  "member",
  "getAll.operation.ts",
);
const discordMemberRoleAddOperationSourcePath = discordActionSourcePath(
  "member",
  "roleAdd.operation.ts",
);
const discordMemberRoleRemoveOperationSourcePath = discordActionSourcePath(
  "member",
  "roleRemove.operation.ts",
);
function airtableActionSourcePath(...segments: string[]) {
  return path.join(
    repoRoot,
    "integrations",
    "n8n-nodes-base",
    "nodes",
    "Airtable",
    "v2",
    "actions",
    ...segments,
  );
}

const airtableVersionDescriptionSourcePath = airtableActionSourcePath(
  "versionDescription.ts",
);
const airtableCommonDescriptionSourcePath = airtableActionSourcePath(
  "common.descriptions.ts",
);
const airtableBaseResourceSourcePath = airtableActionSourcePath(
  "base",
  "Base.resource.ts",
);
const airtableRecordResourceSourcePath = airtableActionSourcePath(
  "record",
  "Record.resource.ts",
);
const airtableRecordCreateOperationSourcePath = airtableActionSourcePath(
  "record",
  "create.operation.ts",
);
const airtableRecordDeleteOperationSourcePath = airtableActionSourcePath(
  "record",
  "deleteRecord.operation.ts",
);
const airtableRecordGetOperationSourcePath = airtableActionSourcePath(
  "record",
  "get.operation.ts",
);
const airtableRecordSearchOperationSourcePath = airtableActionSourcePath(
  "record",
  "search.operation.ts",
);
const airtableRecordUpdateOperationSourcePath = airtableActionSourcePath(
  "record",
  "update.operation.ts",
);
const airtableRecordUpsertOperationSourcePath = airtableActionSourcePath(
  "record",
  "upsert.operation.ts",
);
const airtableBaseGetManyOperationSourcePath = airtableActionSourcePath(
  "base",
  "getMany.operation.ts",
);
const airtableBaseGetSchemaOperationSourcePath = airtableActionSourcePath(
  "base",
  "getSchema.operation.ts",
);
function googleSheetsActionSourcePath(...segments: string[]) {
  return path.join(
    repoRoot,
    "integrations",
    "n8n-nodes-base",
    "nodes",
    "Google",
    "Sheet",
    "v2",
    "actions",
    ...segments,
  );
}

function postgresActionSourcePath(...segments: string[]) {
  return path.join(
    repoRoot,
    "integrations",
    "n8n-nodes-base",
    "nodes",
    "Postgres",
    "v2",
    "actions",
    ...segments,
  );
}

const googleSheetsVersionDescriptionSourcePath = googleSheetsActionSourcePath(
  "versionDescription.ts",
);
const googleSheetsSheetResourceSourcePath = googleSheetsActionSourcePath(
  "sheet",
  "Sheet.resource.ts",
);
const googleSheetsSpreadsheetResourceSourcePath = googleSheetsActionSourcePath(
  "spreadsheet",
  "SpreadSheet.resource.ts",
);
const googleSheetsAppendOperationSourcePath = googleSheetsActionSourcePath(
  "sheet",
  "append.operation.ts",
);
const googleSheetsAppendOrUpdateOperationSourcePath = googleSheetsActionSourcePath(
  "sheet",
  "appendOrUpdate.operation.ts",
);
const googleSheetsClearOperationSourcePath = googleSheetsActionSourcePath(
  "sheet",
  "clear.operation.ts",
);
const googleSheetsCreateSheetOperationSourcePath = googleSheetsActionSourcePath(
  "sheet",
  "create.operation.ts",
);
const googleSheetsDeleteOperationSourcePath = googleSheetsActionSourcePath(
  "sheet",
  "delete.operation.ts",
);
const googleSheetsReadOperationSourcePath = googleSheetsActionSourcePath(
  "sheet",
  "read.operation.ts",
);
const googleSheetsCommonDescriptionSourcePath = googleSheetsActionSourcePath(
  "sheet",
  "commonDescription.ts",
);
const googleSheetsUpdateOperationSourcePath = googleSheetsActionSourcePath(
  "sheet",
  "update.operation.ts",
);
const googleSheetsCreateSpreadsheetOperationSourcePath = googleSheetsActionSourcePath(
  "spreadsheet",
  "create.operation.ts",
);
const googleSheetsDeleteSpreadsheetOperationSourcePath = googleSheetsActionSourcePath(
  "spreadsheet",
  "delete.operation.ts",
);
const postgresVersionDescriptionSourcePath = postgresActionSourcePath(
  "versionDescription.ts",
);
const postgresCommonDescriptionSourcePath = postgresActionSourcePath(
  "common.descriptions.ts",
);
const postgresDatabaseResourceSourcePath = postgresActionSourcePath(
  "database",
  "Database.resource.ts",
);
const postgresDeleteTableOperationSourcePath = postgresActionSourcePath(
  "database",
  "deleteTable.operation.ts",
);
const postgresExecuteQueryOperationSourcePath = postgresActionSourcePath(
  "database",
  "executeQuery.operation.ts",
);
const postgresInsertOperationSourcePath = postgresActionSourcePath(
  "database",
  "insert.operation.ts",
);
const postgresSelectOperationSourcePath = postgresActionSourcePath(
  "database",
  "select.operation.ts",
);
const postgresUpdateOperationSourcePath = postgresActionSourcePath(
  "database",
  "update.operation.ts",
);
const postgresUpsertOperationSourcePath = postgresActionSourcePath(
  "database",
  "upsert.operation.ts",
);
function notionSourcePath(...segments: string[]) {
  return path.join(
    repoRoot,
    "integrations",
    "n8n-nodes-base",
    "nodes",
    "Notion",
    ...segments,
  );
}

const notionVersionDescriptionSourcePath = notionSourcePath(
  "v2",
  "VersionDescription.ts",
);
const notionBlockDescriptionSourcePath = notionSourcePath(
  "shared",
  "descriptions",
  "BlockDescription.ts",
);
const notionBlocksDescriptionSourcePath = notionSourcePath(
  "shared",
  "descriptions",
  "Blocks.ts",
);
const notionFiltersDescriptionSourcePath = notionSourcePath(
  "shared",
  "descriptions",
  "Filters.ts",
);
const notionGenericFunctionsSourcePath = notionSourcePath(
  "shared",
  "GenericFunctions.ts",
);
const notionDatabaseDescriptionSourcePath = notionSourcePath(
  "shared",
  "descriptions",
  "DatabaseDescription.ts",
);
const notionDatabasePageDescriptionSourcePath = notionSourcePath(
  "shared",
  "descriptions",
  "DatabasePageDescription.ts",
);
const notionPageDescriptionSourcePath = notionSourcePath(
  "shared",
  "descriptions",
  "PageDescription.ts",
);
const notionUserDescriptionSourcePath = notionSourcePath(
  "shared",
  "descriptions",
  "UserDescription.ts",
);

function readSlackMessageSource() {
  return fs.readFileSync(slackMessageSourcePath, "utf8");
}

function readHttpRequestSources() {
  return {
    descriptionSource: fs.readFileSync(httpRequestDescriptionSourcePath, "utf8"),
    optimizerSource: fs.readFileSync(httpRequestOptimizerSourcePath, "utf8"),
  };
}

function readOpenAiSources() {
  return {
    chatSource: fs.readFileSync(openAiChatSourcePath, "utf8"),
    textSource: fs.readFileSync(openAiTextSourcePath, "utf8"),
    imageSource: fs.readFileSync(openAiImageSourcePath, "utf8"),
  };
}

function readGmailSources() {
  return {
    gmailNodeSource: fs.readFileSync(gmailNodeSourcePath, "utf8"),
    messageSource: fs.readFileSync(gmailMessageSourcePath, "utf8"),
    draftSource: fs.readFileSync(gmailDraftSourcePath, "utf8"),
    labelSource: fs.readFileSync(gmailLabelSourcePath, "utf8"),
    threadSource: fs.readFileSync(gmailThreadSourcePath, "utf8"),
  };
}

function readDiscordSources() {
  return {
    versionDescriptionSource: fs.readFileSync(
      discordVersionDescriptionSourcePath,
      "utf8",
    ),
    commonDescriptionSource: fs.readFileSync(
      discordCommonDescriptionSourcePath,
      "utf8",
    ),
    messageResourceSource: fs.readFileSync(
      discordMessageResourceSourcePath,
      "utf8",
    ),
    channelResourceSource: fs.readFileSync(
      discordChannelResourceSourcePath,
      "utf8",
    ),
    memberResourceSource: fs.readFileSync(
      discordMemberResourceSourcePath,
      "utf8",
    ),
    descriptionUtilitiesSource: fs.readFileSync(
      n8nDescriptionUtilitiesSourcePath,
      "utf8",
    ),
    sendAndWaitUtilitiesSource: fs.readFileSync(
      n8nSendAndWaitUtilitiesSourcePath,
      "utf8",
    ),
    sendAndWaitDescriptionsSource: fs.readFileSync(
      n8nSendAndWaitDescriptionsSourcePath,
      "utf8",
    ),
    formNodeSource: fs.readFileSync(n8nFormNodeSourcePath, "utf8"),
    formCommonDescriptionSource: fs.readFileSync(
      n8nFormCommonDescriptionSourcePath,
      "utf8",
    ),
    messageSendOperationSource: fs.readFileSync(
      discordMessageSendOperationSourcePath,
      "utf8",
    ),
    messageSendAndWaitOperationSource: fs.readFileSync(
      discordMessageSendAndWaitOperationSourcePath,
      "utf8",
    ),
    messageDeleteOperationSource: fs.readFileSync(
      discordMessageDeleteOperationSourcePath,
      "utf8",
    ),
    messageGetOperationSource: fs.readFileSync(
      discordMessageGetOperationSourcePath,
      "utf8",
    ),
    messageGetAllOperationSource: fs.readFileSync(
      discordMessageGetAllOperationSourcePath,
      "utf8",
    ),
    messageReactOperationSource: fs.readFileSync(
      discordMessageReactOperationSourcePath,
      "utf8",
    ),
    channelCreateOperationSource: fs.readFileSync(
      discordChannelCreateOperationSourcePath,
      "utf8",
    ),
    channelDeleteOperationSource: fs.readFileSync(
      discordChannelDeleteOperationSourcePath,
      "utf8",
    ),
    channelGetOperationSource: fs.readFileSync(
      discordChannelGetOperationSourcePath,
      "utf8",
    ),
    channelGetAllOperationSource: fs.readFileSync(
      discordChannelGetAllOperationSourcePath,
      "utf8",
    ),
    channelUpdateOperationSource: fs.readFileSync(
      discordChannelUpdateOperationSourcePath,
      "utf8",
    ),
    memberGetAllOperationSource: fs.readFileSync(
      discordMemberGetAllOperationSourcePath,
      "utf8",
    ),
    memberRoleAddOperationSource: fs.readFileSync(
      discordMemberRoleAddOperationSourcePath,
      "utf8",
    ),
    memberRoleRemoveOperationSource: fs.readFileSync(
      discordMemberRoleRemoveOperationSourcePath,
      "utf8",
    ),
  };
}

function readAirtableSources() {
  return {
    versionDescriptionSource: fs.readFileSync(
      airtableVersionDescriptionSourcePath,
      "utf8",
    ),
    commonDescriptionSource: fs.readFileSync(
      airtableCommonDescriptionSourcePath,
      "utf8",
    ),
    baseResourceSource: fs.readFileSync(
      airtableBaseResourceSourcePath,
      "utf8",
    ),
    recordResourceSource: fs.readFileSync(
      airtableRecordResourceSourcePath,
      "utf8",
    ),
    recordCreateOperationSource: fs.readFileSync(
      airtableRecordCreateOperationSourcePath,
      "utf8",
    ),
    recordDeleteOperationSource: fs.readFileSync(
      airtableRecordDeleteOperationSourcePath,
      "utf8",
    ),
    recordGetOperationSource: fs.readFileSync(
      airtableRecordGetOperationSourcePath,
      "utf8",
    ),
    recordSearchOperationSource: fs.readFileSync(
      airtableRecordSearchOperationSourcePath,
      "utf8",
    ),
    recordUpdateOperationSource: fs.readFileSync(
      airtableRecordUpdateOperationSourcePath,
      "utf8",
    ),
    recordUpsertOperationSource: fs.readFileSync(
      airtableRecordUpsertOperationSourcePath,
      "utf8",
    ),
    baseGetManyOperationSource: fs.readFileSync(
      airtableBaseGetManyOperationSourcePath,
      "utf8",
    ),
    baseGetSchemaOperationSource: fs.readFileSync(
      airtableBaseGetSchemaOperationSourcePath,
      "utf8",
    ),
  };
}

function readGoogleSheetsSources() {
  return {
    versionDescriptionSource: fs.readFileSync(
      googleSheetsVersionDescriptionSourcePath,
      "utf8",
    ),
    sheetResourceSource: fs.readFileSync(
      googleSheetsSheetResourceSourcePath,
      "utf8",
    ),
    spreadsheetResourceSource: fs.readFileSync(
      googleSheetsSpreadsheetResourceSourcePath,
      "utf8",
    ),
    appendOperationSource: fs.readFileSync(
      googleSheetsAppendOperationSourcePath,
      "utf8",
    ),
    appendOrUpdateOperationSource: fs.readFileSync(
      googleSheetsAppendOrUpdateOperationSourcePath,
      "utf8",
    ),
    clearOperationSource: fs.readFileSync(
      googleSheetsClearOperationSourcePath,
      "utf8",
    ),
    createSheetOperationSource: fs.readFileSync(
      googleSheetsCreateSheetOperationSourcePath,
      "utf8",
    ),
    deleteOperationSource: fs.readFileSync(
      googleSheetsDeleteOperationSourcePath,
      "utf8",
    ),
    readOperationSource: fs.readFileSync(
      googleSheetsReadOperationSourcePath,
      "utf8",
    ),
    commonDescriptionSource: fs.readFileSync(
      googleSheetsCommonDescriptionSourcePath,
      "utf8",
    ),
    updateOperationSource: fs.readFileSync(
      googleSheetsUpdateOperationSourcePath,
      "utf8",
    ),
    createSpreadsheetOperationSource: fs.readFileSync(
      googleSheetsCreateSpreadsheetOperationSourcePath,
      "utf8",
    ),
    deleteSpreadsheetOperationSource: fs.readFileSync(
      googleSheetsDeleteSpreadsheetOperationSourcePath,
      "utf8",
    ),
  };
}

function readNotionV2Sources() {
  return {
    versionDescriptionSource: fs.readFileSync(
      notionVersionDescriptionSourcePath,
      "utf8",
    ),
    blockDescriptionSource: fs.readFileSync(
      notionBlockDescriptionSourcePath,
      "utf8",
    ),
    blocksDescriptionSource: fs.readFileSync(
      notionBlocksDescriptionSourcePath,
      "utf8",
    ),
    filtersDescriptionSource: fs.readFileSync(
      notionFiltersDescriptionSourcePath,
      "utf8",
    ),
    genericFunctionsSource: fs.readFileSync(
      notionGenericFunctionsSourcePath,
      "utf8",
    ),
    databaseDescriptionSource: fs.readFileSync(
      notionDatabaseDescriptionSourcePath,
      "utf8",
    ),
    databasePageDescriptionSource: fs.readFileSync(
      notionDatabasePageDescriptionSourcePath,
      "utf8",
    ),
    pageDescriptionSource: fs.readFileSync(
      notionPageDescriptionSourcePath,
      "utf8",
    ),
    userDescriptionSource: fs.readFileSync(
      notionUserDescriptionSourcePath,
      "utf8",
    ),
  };
}

function readPostgresSources() {
  return {
    versionDescriptionSource: fs.readFileSync(
      postgresVersionDescriptionSourcePath,
      "utf8",
    ),
    commonDescriptionSource: fs.readFileSync(
      postgresCommonDescriptionSourcePath,
      "utf8",
    ),
    databaseResourceSource: fs.readFileSync(
      postgresDatabaseResourceSourcePath,
      "utf8",
    ),
    deleteTableOperationSource: fs.readFileSync(
      postgresDeleteTableOperationSourcePath,
      "utf8",
    ),
    executeQueryOperationSource: fs.readFileSync(
      postgresExecuteQueryOperationSourcePath,
      "utf8",
    ),
    insertOperationSource: fs.readFileSync(
      postgresInsertOperationSourcePath,
      "utf8",
    ),
    selectOperationSource: fs.readFileSync(
      postgresSelectOperationSourcePath,
      "utf8",
    ),
    updateOperationSource: fs.readFileSync(
      postgresUpdateOperationSourcePath,
      "utf8",
    ),
    upsertOperationSource: fs.readFileSync(
      postgresUpsertOperationSourcePath,
      "utf8",
    ),
  };
}

test("n8n Studio extracts Airtable v2 create parameters from local n8n source", () => {
  const extraction = extractAirtableSourceParameters(readAirtableSources());

  assert.equal(
    extraction.sourcePath,
    "nodes/Airtable/v2/actions/versionDescription.ts",
  );
  assert.equal(extraction.generatedFrom, "source-parameter-description");
  assert.equal(extraction.parameters.length >= 7, true);

  const fieldNames = extraction.parameters.map((parameter) => parameter.name);
  assert.deepEqual(
    [
      "authentication",
      "resource",
      "operation",
      "base",
      "table",
      "columns",
      "options",
    ].every((name) => fieldNames.includes(name)),
    true,
  );

  const resource = extraction.parameters.find((parameter) => parameter.name === "resource");
  assert.deepEqual(resource?.options?.map((option) => option.value), [
    "base",
    "record",
  ]);

  const base = extraction.parameters.find((parameter) => parameter.name === "base");
  const table = extraction.parameters.find((parameter) => parameter.name === "table");
  assert.equal(base?.type, "resourceLocator");
  assert.equal(
    base?.resourceLocatorModes?.some((mode) => mode.searchListMethod === "baseSearch"),
    true,
  );
  assert.equal(table?.type, "resourceLocator");
  assert.equal(
    table?.resourceLocatorModes?.some((mode) => mode.searchListMethod === "tableSearch"),
    true,
  );

  const columns = extraction.parameters.find((parameter) => parameter.name === "columns");
  assert.equal(columns?.type, "resourceMapper");
  assert.equal(columns?.resourceMapper?.resourceMapperMethod, "getColumns");
  assert.equal(columns?.resourceMapper?.mode, "add");
  assert.deepEqual(columns?.resourceMapper?.loadOptionsDependsOn, [
    "table.value",
    "base.value",
  ]);

  const options = extraction.parameters.find(
    (parameter) =>
      parameter.name === "options" &&
      parameter.displayOptions?.show?.operation?.includes("create"),
  );
  assert.equal(options?.type, "collection");
  assert.equal(options?.childParameters?.some((field) => field.name === "typecast"), true);
  assert.equal(
    options?.childParameters?.some(
      (field) =>
        field.name === "ignoreFields" &&
        field.displayOptions?.show?.["/columns.mappingMode"]?.includes("autoMapInputData"),
    ),
    true,
  );
});

test("n8n Studio builds Airtable source-backed record create schemas", () => {
  const sourceNodeType = createAirtableNodeTypeFromSource(readAirtableSources());
  const registry = { [sourceNodeType.name]: sourceNodeType };
  const schema = createParameterSchemaForNode(
    "n8n-nodes-base.airtable",
    {
      authentication: "airtableTokenApi",
      resource: "record",
      operation: "create",
      base: { mode: "list", value: "app123" },
      table: { mode: "list", value: "tbl123" },
      columns: {
        mappingMode: "defineBelow",
        value: null,
      },
    },
    registry,
  );
  const autoMapSchema = createParameterSchemaForNode(
    "n8n-nodes-base.airtable",
    {
      authentication: "airtableTokenApi",
      resource: "record",
      operation: "create",
      base: { mode: "list", value: "app123" },
      table: { mode: "list", value: "tbl123" },
      columns: {
        mappingMode: "autoMapInputData",
        value: null,
      },
      options: {
        ignoreFields: "internalNotes",
      },
    },
    registry,
  );

  assert.deepEqual(sourceNodeType.credentials.map((credential) => credential.name), [
    "airtableTokenApi",
    "airtableOAuth2Api",
  ]);

  const base = schema.fields.find((field) => field.name === "base");
  const table = schema.fields.find((field) => field.name === "table");
  const columns = schema.fields.find((field) => field.name === "columns");
  const options = schema.fields.find((field) => field.name === "options");
  const autoMapOptions = autoMapSchema.fields.find((field) => field.name === "options");

  assert.equal(base?.type, "resourceLocator");
  assert.equal(table?.type, "resourceLocator");
  assert.equal(columns?.type, "resourceMapper");
  assert.equal(columns?.resourceMapper?.resourceMapperMethod, "getColumns");
  assert.equal(options?.childFields?.some((field) => field.name === "typecast"), true);
  assert.equal(options?.childFields?.some((field) => field.name === "ignoreFields"), false);
  assert.equal(
    autoMapOptions?.childFields?.some((field) => field.name === "ignoreFields"),
    true,
  );
  assert.equal(
    schema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.airtable.base.baseSearch",
    ),
    true,
  );
  assert.equal(
    schema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.airtable.table.tableSearch",
    ),
    true,
  );
  assert.equal(
    schema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.airtable.columns.getColumns",
    ),
    true,
  );
});

test("n8n Studio preserves Airtable source-backed operation-specific fields", () => {
  const sourceNodeType = createAirtableNodeTypeFromSource(readAirtableSources());
  const registry = { [sourceNodeType.name]: sourceNodeType };
  const updateSchema = createParameterSchemaForNode(
    "n8n-nodes-base.airtable",
    {
      authentication: "airtableTokenApi",
      resource: "record",
      operation: "update",
      base: { mode: "list", value: "app123" },
      table: { mode: "list", value: "tbl123" },
      columns: {
        mappingMode: "autoMapInputData",
        value: null,
      },
      options: {
        updateAllMatches: true,
        ignoreFields: "internalNotes",
      },
    },
    registry,
  );
  const searchSchema = createParameterSchemaForNode(
    "n8n-nodes-base.airtable",
    {
      authentication: "airtableTokenApi",
      resource: "record",
      operation: "search",
      base: { mode: "list", value: "app123" },
      table: { mode: "list", value: "tbl123" },
      returnAll: false,
      options: {
        view: { mode: "list", value: "viw123" },
      },
    },
    registry,
  );
  const baseSchema = createParameterSchemaForNode(
    "n8n-nodes-base.airtable",
    {
      authentication: "airtableTokenApi",
      resource: "base",
      operation: "getSchema",
      base: { mode: "list", value: "app123" },
    },
    registry,
  );

  const updateColumns = updateSchema.fields.find((field) => field.name === "columns");
  const updateOptions = updateSchema.fields.find((field) => field.name === "options");
  const searchOptions = searchSchema.fields.find((field) => field.name === "options");
  const sort = searchSchema.fields.find((field) => field.name === "sort");
  const schemaBase = baseSchema.fields.find((field) => field.name === "base");

  assert.equal(updateColumns?.resourceMapper?.resourceMapperMethod, "getColumnsWithRecordId");
  assert.equal(updateColumns?.resourceMapper?.mode, "update");
  assert.equal(
    updateOptions?.childFields?.some((field) => field.name === "updateAllMatches"),
    true,
  );
  assert.equal(
    updateOptions?.childFields?.some((field) => field.name === "ignoreFields"),
    true,
  );
  assert.equal(searchSchema.fields.some((field) => field.name === "filterByFormula"), true);
  assert.equal(searchSchema.fields.some((field) => field.name === "limit"), true);
  assert.equal(
    searchOptions?.childFields?.some(
      (field) =>
        field.name === "view" &&
        field.resourceLocatorModes?.some(
          (mode) => mode.searchListMethod === "viewSearch",
        ),
    ),
    true,
  );
  assert.equal(sort?.type, "fixedCollection");
  assert.equal(schemaBase?.type, "resourceLocator");
  assert.equal(
    baseSchema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.airtable.base.baseSearch",
    ),
    true,
  );
});

test("n8n Studio extracts Postgres v2 parameters from local n8n source", () => {
  const extraction = extractPostgresSourceParameters(readPostgresSources());

  assert.equal(
    extraction.sourcePath,
    "nodes/Postgres/v2/actions/versionDescription.ts",
  );
  assert.equal(extraction.generatedFrom, "source-parameter-description");

  const fieldNames = extraction.parameters.map((parameter) => parameter.name);
  assert.deepEqual(
    [
      "resource",
      "operation",
      "schema",
      "table",
      "columns",
      "query",
      "options",
      "where",
      "sort",
      "combineConditions",
    ].every((name) => fieldNames.includes(name)),
    true,
  );

  const sourceNodeType = createPostgresNodeTypeFromSource(readPostgresSources());
  assert.deepEqual(sourceNodeType.credentials.map((credential) => credential.name), [
    "postgres",
  ]);
});

test("n8n Studio builds Postgres source-backed database schemas", () => {
  const sourceNodeType = createPostgresNodeTypeFromSource(readPostgresSources());
  const registry = { [sourceNodeType.name]: sourceNodeType };
  const insertSchema = createParameterSchemaForNode(
    "n8n-nodes-base.postgres",
    {
      resource: "database",
      operation: "insert",
      schema: { mode: "list", value: "public" },
      table: { mode: "list", value: "customers" },
      columns: {
        mappingMode: "defineBelow",
        value: null,
      },
    },
    registry,
  );
  const updateSchema = createParameterSchemaForNode(
    "n8n-nodes-base.postgres",
    {
      resource: "database",
      operation: "update",
      schema: { mode: "list", value: "public" },
      table: { mode: "list", value: "customers" },
      columns: {
        mappingMode: "autoMapInputData",
        value: null,
      },
      options: {
        outputColumns: ["id"],
      },
    },
    registry,
  );
  const selectSchema = createParameterSchemaForNode(
    "n8n-nodes-base.postgres",
    {
      resource: "database",
      operation: "select",
      schema: { mode: "list", value: "public" },
      table: { mode: "list", value: "customers" },
      returnAll: false,
      where: {
        values: [
          {
            column: "status",
            condition: "equal",
            value: "active",
          },
        ],
      },
    },
    registry,
  );
  const querySchema = createParameterSchemaForNode(
    "n8n-nodes-base.postgres",
    {
      resource: "database",
      operation: "executeQuery",
      query: "SELECT * FROM customers WHERE id = $1",
      options: {
        queryReplacement: "1",
      },
    },
    registry,
  );

  const schema = insertSchema.fields.find((field) => field.name === "schema");
  const table = insertSchema.fields.find((field) => field.name === "table");
  const insertColumns = insertSchema.fields.find((field) => field.name === "columns");
  const updateColumns = updateSchema.fields.find((field) => field.name === "columns");
  const updateOptions = updateSchema.fields.find((field) => field.name === "options");
  const where = selectSchema.fields.find((field) => field.name === "where");
  const sort = selectSchema.fields.find((field) => field.name === "sort");
  const query = querySchema.fields.find((field) => field.name === "query");
  const queryOptions = querySchema.fields.find((field) => field.name === "options");

  assert.equal(schema?.type, "resourceLocator");
  assert.equal(table?.type, "resourceLocator");
  assert.equal(
    schema?.resourceLocatorModes?.some((mode) => mode.searchListMethod === "schemaSearch"),
    true,
  );
  assert.equal(
    table?.resourceLocatorModes?.some((mode) => mode.searchListMethod === "tableSearch"),
    true,
  );
  assert.equal(insertColumns?.type, "resourceMapper");
  assert.equal(insertColumns?.resourceMapper?.resourceMapperMethod, "getMappingColumns");
  assert.equal(insertColumns?.resourceMapper?.mode, "add");
  assert.equal(updateColumns?.resourceMapper?.mode, "update");
  assert.equal(
    updateOptions?.childFields?.some((field) => field.name === "outputColumns"),
    true,
  );
  assert.equal(where?.type, "fixedCollection");
  assert.equal(sort?.type, "fixedCollection");
  assert.equal(query?.name, "query");
  assert.equal(querySchema.hiddenFields.includes("schema"), true);
  assert.equal(querySchema.hiddenFields.includes("table"), true);
  assert.equal(
    queryOptions?.childFields?.some((field) => field.name === "queryReplacement"),
    true,
  );
  assert.equal(
    insertSchema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.postgres.schema.schemaSearch",
    ),
    true,
  );
  assert.equal(
    insertSchema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.postgres.table.tableSearch",
    ),
    true,
  );
  assert.equal(
    insertSchema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.postgres.columns.getMappingColumns",
    ),
    true,
  );
});

test("n8n Studio extracts HTTP Request V3 parameters from local n8n source", () => {
  const extraction = extractHttpRequestV3SourceParameters(readHttpRequestSources());

  assert.equal(extraction.sourcePath, "nodes/HttpRequest/V3/Description.ts");
  assert.equal(extraction.generatedFrom, "source-parameter-description");
  assert.equal(extraction.parameters.length >= 40, true);

  const fieldNames = extraction.parameters.map((parameter) => parameter.name);
  assert.deepEqual(
    [
      "curlImport",
      "method",
      "url",
      "authentication",
      "nodeCredentialType",
      "genericAuthType",
      "sendQuery",
      "queryParameters",
      "sendHeaders",
      "headerParameters",
      "sendBody",
      "contentType",
      "bodyParameters",
      "options",
      "optimizeResponse",
      "responseType",
      "fields",
      "maxLength",
    ].every((name) => fieldNames.includes(name)),
    true,
  );

  const queryParameters = extraction.parameters.find((parameter) => parameter.name === "queryParameters");
  assert.equal(queryParameters?.type, "fixedCollection");
  assert.equal(queryParameters?.displayOptions?.show?.sendQuery?.includes(true), true);
  assert.equal(queryParameters?.childParameters?.some((child) => child.name === "name"), true);
  assert.equal(queryParameters?.childParameters?.some((child) => child.name === "value"), true);

  const nodeCredentialType = extraction.parameters.find((parameter) => parameter.name === "nodeCredentialType");
  assert.equal(nodeCredentialType?.type, "credentialsSelect");
  assert.deepEqual(nodeCredentialType?.credentialTypes, [
    "extends:oAuth2Api",
    "extends:oAuth1Api",
    "has:authenticate",
  ]);

  const fields = extraction.parameters.find((parameter) => parameter.name === "fields");
  assert.equal(fields?.displayOptions?.show?.["@tool"]?.includes(true), true);
  assert.equal(fields?.displayOptions?.hide?.fieldsToInclude?.includes("all"), true);
});

test("n8n Studio builds HTTP Request source-backed query and tool schemas", () => {
  const sourceNodeType = createHttpRequestV3NodeTypeFromSource(readHttpRequestSources());
  const registry = { [sourceNodeType.name]: sourceNodeType };
  const querySchema = createParameterSchemaForNode(
    "n8n-nodes-base.httpRequest",
    {
      sendQuery: true,
      specifyQuery: "keypair",
      queryParameters: {
        parameters: [
          {
            name: "limit",
            value: "10",
          },
        ],
      },
    },
    registry,
  );

  const queryParameters = querySchema.fields.find((field) => field.name === "queryParameters");
  assert.equal(queryParameters?.type, "fixedCollection");
  assert.equal(queryParameters?.collectionItems?.[0]?.label, "Parameters 1");
  assert.equal(queryParameters?.collectionItems?.[0]?.fields.find((field) => field.name === "name")?.value, "limit");
  assert.equal(queryParameters?.collectionItems?.[0]?.fields.find((field) => field.name === "value")?.value, "10");
  assert.equal(querySchema.fields.some((field) => field.name === "jsonQuery"), false);

  const toolSchema = createParameterSchemaForNode(
    "n8n-nodes-base.httpRequest",
    {
      "@tool": true,
      optimizeResponse: true,
      responseType: "json",
      fieldsToInclude: "selected",
    },
    registry,
  );

  assert.equal(toolSchema.fields.some((field) => field.name === "responseType"), true);
  assert.equal(toolSchema.fields.some((field) => field.name === "dataField"), true);
  assert.equal(toolSchema.fields.some((field) => field.name === "fields"), true);
  assert.equal(toolSchema.fields.some((field) => field.name === "cssSelector"), false);
});

test("n8n Studio extracts OpenAI chat, text, and image parameters from local n8n source", () => {
  const extraction = extractOpenAiSourceParameters(readOpenAiSources());

  assert.equal(extraction.sourcePath, "nodes/OpenAi/OpenAi.node.ts");
  assert.equal(extraction.generatedFrom, "source-parameter-description");
  assert.equal(extraction.parameters.length >= 20, true);

  const fieldNames = extraction.parameters.map((parameter) => parameter.name);
  assert.deepEqual(
    [
      "resource",
      "operation",
      "chatModel",
      "prompt",
      "simplifyOutput",
      "options",
      "model",
      "input",
      "instruction",
      "imageModel",
      "responseFormat",
    ].every((name) => fieldNames.includes(name)),
    true,
  );

  const resource = extraction.parameters.find((parameter) => parameter.name === "resource");
  assert.deepEqual(resource?.options?.map((option) => option.value), [
    "chat",
    "image",
    "text",
  ]);

  const chatModel = extraction.parameters.find((parameter) => parameter.name === "chatModel");
  assert.equal(chatModel?.type, "options");
  assert.equal(chatModel?.dynamicOptions?.request.method, "GET");
  assert.equal(chatModel?.dynamicOptions?.request.url, "/v1/models");
  assert.match(chatModel?.dynamicOptions?.responseFilter ?? "", /gpt-/);

  const imageModel = extraction.parameters.find((parameter) => parameter.name === "imageModel");
  assert.equal(imageModel?.dynamicOptions?.request.url, "/v1/models");
  assert.match(imageModel?.dynamicOptions?.responseFilter ?? "", /dall-/);

  const prompt = extraction.parameters.find(
    (parameter) =>
      parameter.name === "prompt" &&
      parameter.displayOptions?.show?.resource?.includes("chat"),
  );
  assert.equal(prompt?.type, "fixedCollection");
  assert.equal(prompt?.childParameters?.some((child) => child.name === "role"), true);
  assert.equal(prompt?.childParameters?.some((child) => child.name === "content"), true);

  const options = extraction.parameters.find(
    (parameter) =>
      parameter.name === "options" &&
      parameter.displayOptions?.show?.resource?.includes("chat"),
  );
  assert.equal(options?.type, "collection");
  assert.equal(options?.childParameters?.some((child) => child.name === "temperature"), true);
  assert.equal(options?.childParameters?.some((child) => child.name === "topP"), true);
});

test("n8n Studio builds OpenAI source-backed parameter schemas with dynamic model boundaries", () => {
  const sourceNodeType = createOpenAiNodeTypeFromSource(readOpenAiSources());
  const registry = { [sourceNodeType.name]: sourceNodeType };
  const chatSchema = createParameterSchemaForNode(
    "n8n-nodes-base.openAi",
    {
      resource: "chat",
      operation: "complete",
      chatModel: "gpt-4o-mini",
      prompt: {
        messages: [
          {
            role: "system",
            content: "Keep replies concise.",
          },
          {
            role: "user",
            content: "Summarize the workflow.",
          },
        ],
      },
      options: {
        temperature: 0.2,
      },
    },
    registry,
  );

  const names = chatSchema.fields.map((field) => field.name);
  assert.equal(names.includes("chatModel"), true);
  assert.equal(names.includes("prompt"), true);
  assert.equal(names.includes("simplifyOutput"), true);
  assert.equal(names.includes("responseFormat"), false);
  assert.equal(
    chatSchema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.openAi.chatModel.GET:/v1/models",
    ),
    true,
  );

  const prompt = chatSchema.fields.find((field) => field.name === "prompt");
  assert.equal(prompt?.collectionItems?.[0]?.label, "Messages 1");
  assert.equal(
    prompt?.collectionItems?.[0]?.fields.find((field) => field.name === "role")?.value,
    "system",
  );
  assert.equal(
    prompt?.collectionItems?.[1]?.fields.find((field) => field.name === "content")?.value,
    "Summarize the workflow.",
  );

  const textSchema = createParameterSchemaForNode(
    "n8n-nodes-base.openAi",
    {
      resource: "text",
      operation: "edit",
      input: "What day of the wek is it?",
      instruction: "Fix spelling.",
    },
    registry,
  );

  assert.equal(textSchema.fields.some((field) => field.name === "input"), true);
  assert.equal(textSchema.fields.some((field) => field.name === "instruction"), true);
  assert.equal(textSchema.fields.some((field) => field.name === "chatModel"), false);
});

test("n8n Studio extracts Gmail v2 parameters from local n8n source", () => {
  const extraction = extractGmailSourceParameters(readGmailSources());

  assert.equal(extraction.sourcePath, "nodes/Google/Gmail/v2/GmailV2.node.ts");
  assert.equal(extraction.generatedFrom, "source-parameter-description");
  assert.equal(extraction.parameters.length >= 45, true);

  const fieldNames = extraction.parameters.map((parameter) => parameter.name);
  assert.deepEqual(
    [
      "authentication",
      "resource",
      "operation",
      "sendTo",
      "subject",
      "emailType",
      "message",
      "options",
      "messageId",
      "returnAll",
      "limit",
      "labelIds",
      "threadId",
    ].every((name) => fieldNames.includes(name)),
    true,
  );

  const resource = extraction.parameters.find((parameter) => parameter.name === "resource");
  assert.deepEqual(resource?.options?.map((option) => option.value), [
    "message",
    "label",
    "draft",
    "thread",
  ]);

  const messageOperation = extraction.parameters.find(
    (parameter) =>
      parameter.name === "operation" &&
      parameter.displayOptions?.show?.resource?.includes("message"),
  );
  assert.deepEqual(
    ["send", "reply", "getAll", "addLabels"].every((operation) =>
      messageOperation?.options?.some((option) => option.value === operation),
    ),
    true,
  );

  const labelIds = extraction.parameters.find(
    (parameter) =>
      parameter.name === "labelIds" &&
      parameter.dynamicOptions?.loadMethod === "getLabels",
  );
  assert.equal(labelIds?.type, "multiOptions");
  assert.equal(labelIds?.dynamicOptions?.source, "n8n-type-options-load-method");
  assert.equal(labelIds?.dynamicOptions?.request.method, "LOAD_OPTIONS");
  assert.equal(labelIds?.dynamicOptions?.request.url, "n8n://load-options/getLabels");

  const draftOptions = extraction.parameters.find(
    (parameter) =>
      parameter.name === "options" &&
      parameter.displayOptions?.show?.resource?.includes("draft") &&
      parameter.displayOptions.show.operation?.includes("create"),
  );
  const fromAlias = draftOptions?.childParameters?.find((child) => child.name === "fromAlias");
  const attachmentsUi = draftOptions?.childParameters?.find((child) => child.name === "attachmentsUi");
  assert.equal(fromAlias?.dynamicOptions?.loadMethod, "getGmailAliases");
  assert.equal(attachmentsUi?.type, "fixedCollection");
  assert.equal(attachmentsUi?.childParameters?.some((child) => child.name === "property"), true);
});

test("n8n Studio builds Gmail source-backed schemas with nested dynamic option boundaries", () => {
  const sourceNodeType = createGmailNodeTypeFromSource(readGmailSources());
  const registry = { [sourceNodeType.name]: sourceNodeType };

  assert.deepEqual(sourceNodeType.credentials.map((credential) => credential.name), [
    "googleApi",
    "gmailOAuth2",
  ]);

  const messageSchema = createParameterSchemaForNode(
    "n8n-nodes-base.gmail",
    {
      authentication: "oAuth2",
      resource: "message",
      operation: "send",
      sendTo: "team@example.com",
      subject: "DX update",
      emailType: "html",
      message: "={{ $json.emailBody }}",
    },
    registry,
  );

  const messageNames = messageSchema.fields.map((field) => field.name);
  assert.equal(messageNames.includes("sendTo"), true);
  assert.equal(messageNames.includes("subject"), true);
  assert.equal(messageNames.includes("message"), true);
  assert.equal(messageNames.includes("messageId"), false);

  const draftSchema = createParameterSchemaForNode(
    "n8n-nodes-base.gmail",
    {
      authentication: "oAuth2",
      resource: "draft",
      operation: "create",
      sendTo: "team@example.com",
      subject: "Draft update",
      emailType: "text",
      message: "The draft body",
      options: {
        fromAlias: "primary@example.com",
        attachmentsUi: {
          attachmentsBinary: [
            {
              property: "invoicePdf",
            },
          ],
        },
      },
    },
    registry,
  );

  const options = draftSchema.fields.find((field) => field.name === "options");
  const fromAlias = options?.childFields?.find((field) => field.name === "fromAlias");
  assert.equal(options?.type, "collection");
  assert.equal(fromAlias?.dynamicOptions?.loadMethod, "getGmailAliases");
  assert.equal(
    draftSchema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.gmail.options.fromAlias.getGmailAliases",
    ),
    true,
  );

  const optionsItem = options?.collectionItems?.[0];
  assert.equal(
    optionsItem?.fields.find((field) => field.name === "fromAlias")?.value,
    "primary@example.com",
  );

  const attachmentsUi = optionsItem?.fields.find((field) => field.name === "attachmentsUi");
  const attachmentItem = attachmentsUi?.collectionItems?.[0];
  assert.equal(
    attachmentItem?.fields.find((field) => field.name === "property")?.value,
    "invoicePdf",
  );
});

test("n8n Studio extracts Discord v2 message send parameters from local n8n source", () => {
  const extraction = extractDiscordSourceParameters(readDiscordSources());

  assert.equal(
    extraction.sourcePath,
    "nodes/Discord/v2/actions/versionDescription.ts",
  );
  assert.equal(extraction.generatedFrom, "source-parameter-description");

  const fieldNames = extraction.parameters.map((parameter) => parameter.name);
  assert.deepEqual(
    [
      "authentication",
      "resource",
      "operation",
      "guildId",
      "sendTo",
      "userId",
      "channelId",
      "content",
      "options",
      "embeds",
      "files",
    ].every((name) => fieldNames.includes(name)),
    true,
  );

  const resource = extraction.parameters.find(
    (parameter) => parameter.name === "resource",
  );
  const channelId = extraction.parameters.find(
    (parameter) =>
      parameter.name === "channelId" &&
      parameter.displayOptions?.show?.sendTo?.includes("channel"),
  );
  const userId = extraction.parameters.find(
    (parameter) =>
      parameter.name === "userId" &&
      parameter.displayOptions?.show?.sendTo?.includes("user"),
  );
  const embeds = extraction.parameters.find(
    (parameter) => parameter.name === "embeds",
  );

  assert.deepEqual(resource?.options?.map((option) => option.value), [
    "channel",
    "message",
    "member",
  ]);
  assert.equal(channelId?.type, "resourceLocator");
  assert.equal(
    channelId?.resourceLocatorModes?.[0]?.searchListMethod,
    "textChannelSearch",
  );
  assert.equal(userId?.resourceLocatorModes?.[0]?.searchListMethod, "userSearch");
  assert.equal(embeds?.type, "fixedCollection");
  assert.equal(
    embeds?.childParameters?.some((parameter) => parameter.name === "inputMethod"),
    true,
  );
});

test("n8n Studio builds Discord v2 source-backed message send schemas", () => {
  const sourceNodeType = createDiscordNodeTypeFromSource(readDiscordSources());
  const registry = { [sourceNodeType.name]: sourceNodeType };
  const schema = createParameterSchemaForNode(
    "n8n-nodes-base.discord",
    {
      authentication: "botToken",
      resource: "message",
      operation: "send",
      guildId: { mode: "list", value: "guild-1" },
      sendTo: "channel",
      channelId: { mode: "list", value: "channel-1" },
      content: "Ship it",
      embeds: {
        values: [
          {
            inputMethod: "json",
            json: "{\"title\":\"Launch\"}",
          },
        ],
      },
      files: {
        values: [
          {
            inputFieldName: "data",
          },
        ],
      },
    },
    registry,
  );

  const guildId = schema.fields.find((field) => field.name === "guildId");
  const channelId = schema.fields.find((field) => field.name === "channelId");
  const content = schema.fields.find((field) => field.name === "content");
  const embeds = schema.fields.find((field) => field.name === "embeds");
  const files = schema.fields.find((field) => field.name === "files");
  const embedItem = embeds?.collectionItems?.[0];
  const fileItem = files?.collectionItems?.[0];

  assert.deepEqual(sourceNodeType.credentials.map((credential) => credential.name), [
    "discordBotApi",
    "discordOAuth2Api",
    "discordWebhookApi",
  ]);
  assert.equal(guildId?.type, "resourceLocator");
  assert.equal(channelId?.type, "resourceLocator");
  assert.equal(content?.value, "Ship it");
  assert.equal(embeds?.type, "fixedCollection");
  assert.equal(files?.type, "fixedCollection");
  assert.equal(
    embedItem?.fields.find((field) => field.name === "json")?.value,
    "{\"title\":\"Launch\"}",
  );
  assert.equal(
    fileItem?.fields.find((field) => field.name === "inputFieldName")?.value,
    "data",
  );
  assert.equal(
    schema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.discord.guildId.guildSearch",
    ),
    true,
  );
  assert.equal(
    schema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.discord.channelId.textChannelSearch",
    ),
    true,
  );
});

test("n8n Studio builds Discord v2 source-backed send-and-wait schemas", () => {
  const sourceNodeType = createDiscordNodeTypeFromSource(readDiscordSources());
  const registry = { [sourceNodeType.name]: sourceNodeType };
  const approvalSchema = createParameterSchemaForNode(
    "n8n-nodes-base.discord",
    {
      authentication: "botToken",
      resource: "message",
      operation: "sendAndWait",
      guildId: { mode: "list", value: "guild-1" },
      sendTo: "channel",
      channelId: { mode: "list", value: "channel-1" },
      message: "Approve deployment",
      responseType: "approval",
      approvalOptions: {
        values: {
          approvalType: "double",
          approveLabel: "Approve",
          disapproveLabel: "Decline",
        },
      },
      options: {
        appendAttribution: false,
        limitWaitTime: {
          values: {
            limitType: "afterTimeInterval",
            resumeAmount: 30,
            resumeUnit: "minutes",
          },
        },
      },
    },
    registry,
  );
  const freeTextSchema = createParameterSchemaForNode(
    "n8n-nodes-base.discord",
    {
      authentication: "botToken",
      resource: "message",
      operation: "sendAndWait",
      guildId: { mode: "list", value: "guild-1" },
      sendTo: "user",
      userId: { mode: "list", value: "user-1" },
      message: "Share a status note",
      responseType: "freeText",
      options: {
        messageButtonLabel: "Respond",
        responseFormTitle: "Status response",
      },
    },
    registry,
  );
  const customFormSchema = createParameterSchemaForNode(
    "n8n-nodes-base.discord",
    {
      authentication: "botToken",
      resource: "message",
      operation: "sendAndWait",
      guildId: { mode: "list", value: "guild-1" },
      sendTo: "channel",
      channelId: { mode: "list", value: "channel-1" },
      message: "Choose a release lane",
      responseType: "customForm",
      defineForm: "fields",
      formFields: {
        values: [
          {
            fieldLabel: "Release lane",
            fieldType: "dropdown",
            fieldOptions: {
              values: [{ option: "Production" }],
            },
            requiredField: true,
          },
        ],
      },
    },
    registry,
  );

  const messageOperation = sourceNodeType.properties.find(
    (parameter) =>
      parameter.name === "operation" &&
      parameter.displayOptions?.show?.resource?.includes("message"),
  );
  const subject = approvalSchema.fields.find((field) => field.name === "subject");
  const channelId = approvalSchema.fields.find((field) => field.name === "channelId");
  const message = approvalSchema.fields.find((field) => field.name === "message");
  const responseType = approvalSchema.fields.find(
    (field) => field.name === "responseType",
  );
  const approvalOptions = approvalSchema.fields.find(
    (field) => field.name === "approvalOptions",
  );
  const approvalItem = approvalOptions?.collectionItems?.[0];
  const approvalOptionsField = approvalSchema.fields.find(
    (field) => field.name === "options",
  );
  const approvalOptionsItem = approvalOptionsField?.collectionItems?.[0];
  const limitWaitTime = approvalOptionsItem?.fields.find(
    (field) => field.name === "limitWaitTime",
  );
  const waitItem = limitWaitTime?.collectionItems?.[0];
  const freeTextOptions = freeTextSchema.fields.find(
    (field) => field.name === "options",
  );
  const freeTextOptionsItem = freeTextOptions?.collectionItems?.[0];
  const freeTextApprovalOptions = freeTextSchema.fields.find(
    (field) => field.name === "approvalOptions",
  );
  const freeTextUserId = freeTextSchema.fields.find(
    (field) => field.name === "userId",
  );
  const freeTextDefineForm = freeTextSchema.fields.find(
    (field) => field.name === "defineForm",
  );
  const defineForm = customFormSchema.fields.find(
    (field) => field.name === "defineForm",
  );
  const jsonOutput = customFormSchema.fields.find(
    (field) => field.name === "jsonOutput",
  );
  const formFields = customFormSchema.fields.find(
    (field) => field.name === "formFields",
  );
  const formFieldItem = formFields?.collectionItems?.[0];
  const fieldType = formFieldItem?.fields.find(
    (field) => field.name === "fieldType",
  );
  const fieldOptions = formFieldItem?.fields.find(
    (field) => field.name === "fieldOptions",
  );
  const fieldOptionItem = fieldOptions?.collectionItems?.[0];

  assert.equal(
    messageOperation?.options?.some((option) => option.value === "sendAndWait"),
    true,
  );
  assert.equal(subject, undefined);
  assert.equal(channelId?.type, "resourceLocator");
  assert.equal(
    channelId?.resourceLocatorModes?.[0]?.searchListMethod,
    "textChannelSearch",
  );
  assert.equal(message?.value, "Approve deployment");
  assert.deepEqual(responseType?.options?.map((option) => option.value), [
    "approval",
    "freeText",
    "customForm",
  ]);
  assert.equal(approvalOptions?.type, "fixedCollection");
  assert.equal(
    approvalItem?.fields.find((field) => field.name === "approvalType")?.value,
    "double",
  );
  assert.equal(
    approvalItem?.fields.find((field) => field.name === "approveLabel")?.value,
    "Approve",
  );
  assert.equal(
    approvalItem?.fields.some((field) => field.name === "buttonApprovalStyle"),
    false,
  );
  assert.equal(limitWaitTime?.type, "fixedCollection");
  assert.equal(
    waitItem?.fields.find((field) => field.name === "resumeAmount")?.value,
    30,
  );
  assert.equal(
    approvalOptionsItem?.fields.find((field) => field.name === "appendAttribution")?.value,
    false,
  );
  assert.equal(freeTextApprovalOptions, undefined);
  assert.equal(freeTextDefineForm, undefined);
  assert.equal(freeTextUserId?.type, "resourceLocator");
  assert.equal(
    freeTextUserId?.resourceLocatorModes?.[0]?.searchListMethod,
    "userSearch",
  );
  assert.equal(
    freeTextOptionsItem?.fields.find((field) => field.name === "responseFormTitle")?.value,
    "Status response",
  );
  assert.equal(defineForm?.type, "options");
  assert.equal(defineForm?.value, "fields");
  assert.equal(jsonOutput, undefined);
  assert.equal(formFields?.type, "fixedCollection");
  assert.equal(
    fieldType?.options?.some((option) => option.value === "dropdown"),
    true,
  );
  assert.equal(fieldOptions?.type, "fixedCollection");
  assert.equal(
    fieldOptionItem?.fields.find((field) => field.name === "option")?.value,
    "Production",
  );
  assert.equal(
    formFieldItem?.fields.find((field) => field.name === "requiredField")?.value,
    true,
  );
  assert.equal(
    formFieldItem?.fields.some((field) => field.name === "multipleFiles"),
    false,
  );
  assert.equal(
    approvalSchema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.discord.channelId.textChannelSearch",
    ),
    true,
  );
});

test("n8n Studio builds Discord v2 source-backed message lookup and reaction schemas", () => {
  const sourceNodeType = createDiscordNodeTypeFromSource(readDiscordSources());
  const registry = { [sourceNodeType.name]: sourceNodeType };
  const getSchema = createParameterSchemaForNode(
    "n8n-nodes-base.discord",
    {
      authentication: "botToken",
      resource: "message",
      operation: "get",
      guildId: { mode: "list", value: "guild-1" },
      channelId: { mode: "list", value: "channel-1" },
      messageId: "message-1",
      options: {
        simplify: false,
      },
    },
    registry,
  );
  const getAllSchema = createParameterSchemaForNode(
    "n8n-nodes-base.discord",
    {
      authentication: "botToken",
      resource: "message",
      operation: "getAll",
      guildId: { mode: "list", value: "guild-1" },
      channelId: { mode: "list", value: "channel-1" },
      returnAll: false,
      limit: 12,
      options: {
        simplify: true,
      },
    },
    registry,
  );
  const reactSchema = createParameterSchemaForNode(
    "n8n-nodes-base.discord",
    {
      authentication: "botToken",
      resource: "message",
      operation: "react",
      guildId: { mode: "list", value: "guild-1" },
      channelId: { mode: "list", value: "channel-1" },
      messageId: "message-1",
      emoji: ":rocket:",
    },
    registry,
  );
  const deleteSchema = createParameterSchemaForNode(
    "n8n-nodes-base.discord",
    {
      authentication: "botToken",
      resource: "message",
      operation: "deleteMessage",
      guildId: { mode: "list", value: "guild-1" },
      channelId: { mode: "list", value: "channel-1" },
      messageId: "message-1",
    },
    registry,
  );

  const getChannelId = getSchema.fields.find((field) => field.name === "channelId");
  const getMessageId = getSchema.fields.find((field) => field.name === "messageId");
  const getOptions = getSchema.fields.find((field) => field.name === "options");
  const getOptionsItem = getOptions?.collectionItems?.[0];
  const returnAll = getAllSchema.fields.find((field) => field.name === "returnAll");
  const limit = getAllSchema.fields.find((field) => field.name === "limit");
  const reactEmoji = reactSchema.fields.find((field) => field.name === "emoji");
  const deleteMessageId = deleteSchema.fields.find(
    (field) => field.name === "messageId",
  );

  assert.equal(getChannelId?.type, "resourceLocator");
  assert.equal(
    getChannelId?.resourceLocatorModes?.[0]?.searchListMethod,
    "channelSearch",
  );
  assert.equal(getMessageId?.value, "message-1");
  assert.equal(
    getOptionsItem?.fields.find((field) => field.name === "simplify")?.value,
    false,
  );
  assert.equal(returnAll?.value, false);
  assert.equal(limit?.value, 12);
  assert.equal(reactEmoji?.value, ":rocket:");
  assert.equal(deleteMessageId?.value, "message-1");
  assert.equal(
    getSchema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.discord.channelId.channelSearch",
    ),
    true,
  );
});

test("n8n Studio builds Discord v2 source-backed channel and member schemas", () => {
  const sourceNodeType = createDiscordNodeTypeFromSource(readDiscordSources());
  const registry = { [sourceNodeType.name]: sourceNodeType };
  const channelSchema = createParameterSchemaForNode(
    "n8n-nodes-base.discord",
    {
      authentication: "botToken",
      resource: "channel",
      operation: "create",
      guildId: { mode: "list", value: "guild-1" },
      name: "launch-room",
      type: "0",
      options: {
        categoryId: { mode: "list", value: "category-1" },
        rate_limit_per_user: 5,
      },
    },
    registry,
  );
  const memberSchema = createParameterSchemaForNode(
    "n8n-nodes-base.discord",
    {
      authentication: "botToken",
      resource: "member",
      operation: "roleAdd",
      guildId: { mode: "list", value: "guild-1" },
      userId: { mode: "list", value: "user-1" },
      role: ["role-1"],
    },
    registry,
  );
  const memberGetAllSchema = createParameterSchemaForNode(
    "n8n-nodes-base.discord",
    {
      authentication: "botToken",
      resource: "member",
      operation: "getAll",
      guildId: { mode: "list", value: "guild-1" },
      returnAll: false,
      limit: 25,
      options: {
        simplify: false,
      },
    },
    registry,
  );

  const channelOperation = channelSchema.fields.find(
    (field) => field.name === "operation",
  );
  const channelGuildId = channelSchema.fields.find(
    (field) => field.name === "guildId",
  );
  const channelName = channelSchema.fields.find((field) => field.name === "name");
  const channelOptions = channelSchema.fields.find(
    (field) => field.name === "options",
  );
  const channelOptionItem = channelOptions?.collectionItems?.[0];
  const categoryId = channelOptionItem?.fields.find(
    (field) => field.name === "categoryId",
  );
  const memberUserId = memberSchema.fields.find(
    (field) => field.name === "userId",
  );
  const memberRole = memberSchema.fields.find((field) => field.name === "role");
  const memberLimit = memberGetAllSchema.fields.find(
    (field) => field.name === "limit",
  );
  const memberOptions = memberGetAllSchema.fields.find(
    (field) => field.name === "options",
  );
  const memberOptionsItem = memberOptions?.collectionItems?.[0];

  assert.deepEqual(
    channelOperation?.options?.map((option) => option.value),
    ["create", "deleteChannel", "get", "getAll", "update"],
  );
  assert.equal(channelGuildId?.type, "resourceLocator");
  assert.equal(
    channelGuildId?.resourceLocatorModes?.[0]?.searchListMethod,
    "guildSearch",
  );
  assert.equal(channelName?.value, "launch-room");
  assert.equal(categoryId?.type, "resourceLocator");
  assert.equal(categoryId?.resourceLocatorModes?.[0]?.searchListMethod, "categorySearch");
  assert.equal(
    channelOptionItem?.fields.find((field) => field.name === "rate_limit_per_user")?.value,
    5,
  );
  assert.equal(memberUserId?.type, "resourceLocator");
  assert.equal(
    memberUserId?.resourceLocatorModes?.[0]?.searchListMethod,
    "userSearch",
  );
  assert.equal(memberRole?.type, "multiOptions");
  assert.equal(memberRole?.dynamicOptions?.loadMethod, "getRoles");
  assert.deepEqual(memberRole?.value, ["role-1"]);
  assert.equal(memberLimit?.value, 25);
  assert.equal(
    memberOptionsItem?.fields.find((field) => field.name === "simplify")?.value,
    false,
  );
  assert.equal(
    channelSchema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.discord.options.categoryId.categorySearch",
    ),
    true,
  );
  assert.equal(
    memberSchema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.discord.userId.userSearch",
    ),
    true,
  );
  assert.equal(
    memberSchema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.discord.role.getRoles",
    ),
    true,
  );
});

test("n8n Studio extracts Google Sheets v2 parameters from local n8n source", () => {
  const extraction = extractGoogleSheetsSourceParameters(readGoogleSheetsSources());

  assert.equal(
    extraction.sourcePath,
    "nodes/Google/Sheet/v2/actions/versionDescription.ts",
  );
  assert.equal(extraction.generatedFrom, "source-parameter-description");
  assert.equal(extraction.parameters.length >= 8, true);

  const fieldNames = extraction.parameters.map((parameter) => parameter.name);
  assert.deepEqual(
    [
      "authentication",
      "resource",
      "operation",
      "documentId",
      "sheetName",
      "dataMode",
      "columns",
      "options",
    ].every((name) => fieldNames.includes(name)),
    true,
  );

  const resource = extraction.parameters.find((parameter) => parameter.name === "resource");
  assert.deepEqual(resource?.options?.map((option) => option.value), [
    "spreadsheet",
    "sheet",
  ]);

  const sheetOperation = extraction.parameters.find(
    (parameter) =>
      parameter.name === "operation" &&
      parameter.displayOptions?.show?.resource?.includes("sheet"),
  );
  assert.equal(
    ["append", "read", "update"].every((operation) =>
      sheetOperation?.options?.some((option) => option.value === operation),
    ),
    true,
  );

  const documentId = extraction.parameters.find(
    (parameter) => parameter.name === "documentId",
  );
  assert.equal(documentId?.type, "resourceLocator");
  assert.equal(
    documentId?.resourceLocatorModes?.some(
      (mode) => mode.searchListMethod === "spreadSheetsSearch",
    ),
    true,
  );

  const sheetName = extraction.parameters.find((parameter) => parameter.name === "sheetName");
  assert.equal(sheetName?.resourceLocatorModes?.some((mode) => mode.name === "name"), true);
  assert.equal(
    sheetName?.resourceLocatorModes?.some(
      (mode) => mode.searchListMethod === "sheetsSearch",
    ),
    true,
  );

  const columns = extraction.parameters.find((parameter) => parameter.name === "columns");
  assert.equal(columns?.type, "resourceMapper");
  assert.equal(columns?.renderingBoundary, "complex-source-field");
  assert.equal(columns?.resourceMapper?.resourceMapperMethod, "getMappingColumns");
  assert.deepEqual(columns?.resourceMapper?.loadOptionsDependsOn, ["sheetName.value"]);
  assert.equal(columns?.resourceMapper?.liveProviderExecution, false);
});

test("n8n Studio builds Google Sheets source-backed append schemas", () => {
  const sourceNodeType = createGoogleSheetsNodeTypeFromSource(readGoogleSheetsSources());
  const controlSource = fs.readFileSync(
    path.join(
      repoRoot,
      "examples",
      "n8n-studio",
      "components",
      "n8n-studio",
      "parameter-field-control.tsx",
    ),
    "utf8",
  );
  const schema = createParameterSchemaForNode(
    "n8n-nodes-base.googleSheets",
    {
      authentication: "oAuth2",
      resource: "sheet",
      operation: "append",
      documentId: { mode: "list", value: "spreadsheet-1" },
      sheetName: { mode: "list", value: "Sheet1" },
      columns: {
        mappingMode: "defineBelow",
        value: null,
      },
    },
    { [sourceNodeType.name]: sourceNodeType },
  );
  const autoMapSchema = createParameterSchemaForNode(
    "n8n-nodes-base.googleSheets",
    {
      authentication: "oAuth2",
      resource: "sheet",
      operation: "append",
      documentId: { mode: "list", value: "spreadsheet-1" },
      sheetName: { mode: "list", value: "Sheet1" },
      columns: {
        mappingMode: "autoMapInputData",
        value: null,
      },
    },
    { [sourceNodeType.name]: sourceNodeType },
  );

  assert.deepEqual(sourceNodeType.credentials.map((credential) => credential.name), [
    "googleApi",
    "googleSheetsOAuth2Api",
  ]);

  const names = schema.fields.map((field) => field.name);
  assert.equal(names.includes("documentId"), true);
  assert.equal(names.includes("sheetName"), true);
  assert.equal(names.includes("columns"), true);

  const columns = schema.fields.find((field) => field.name === "columns");
  const options = schema.fields.find((field) => field.name === "options");
  const handlingExtraDataFields = options?.childFields?.filter(
    (field) => field.name === "handlingExtraData",
  ) ?? [];
  const autoMapOptions = autoMapSchema.fields.find((field) => field.name === "options");
  const autoMapHandlingExtraData = autoMapOptions?.childFields?.filter(
    (field) => field.name === "handlingExtraData",
  ) ?? [];
  assert.equal(columns?.type, "resourceMapper");
  assert.equal(columns?.resourceMapper?.resourceMapperMethod, "getMappingColumns");
  assert.equal(options?.type, "collection");
  assert.equal(
    options?.childFields?.some((field) => field.name === "cellFormat"),
    true,
  );
  assert.equal(
    options?.childFields?.some((field) => field.name === "locationDefine"),
    true,
  );
  assert.equal(
    options?.childFields?.some((field) => field.name === "useAppend"),
    true,
  );
  assert.equal(handlingExtraDataFields.length, 0);
  assert.equal(
    autoMapHandlingExtraData.some((field) =>
      field.displayOptions?.show?.["/columns.mappingMode"]?.includes("autoMapInputData"),
    ),
    true,
  );
  assert.equal(
    schema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.googleSheets.documentId.spreadSheetsSearch",
    ),
    true,
  );
  assert.equal(
    schema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.googleSheets.sheetName.sheetsSearch",
    ),
    true,
  );
  assert.equal(
    schema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.googleSheets.columns.getMappingColumns",
    ),
    true,
  );
  assert.match(controlSource, /data-resource-mapper-field/);
  assert.match(controlSource, /data-resource-mapper-method/);
  assert.match(controlSource, /data-resource-mapper-mode/);
  assert.match(controlSource, /data-resource-mapper-value/);
});

test("n8n Studio builds Google Sheets source-backed update and spreadsheet create schemas", () => {
  const sourceNodeType = createGoogleSheetsNodeTypeFromSource(readGoogleSheetsSources());
  const registry = { [sourceNodeType.name]: sourceNodeType };
  const updateSchema = createParameterSchemaForNode(
    "n8n-nodes-base.googleSheets",
    {
      authentication: "oAuth2",
      resource: "sheet",
      operation: "update",
      documentId: { mode: "list", value: "spreadsheet-1" },
      sheetName: { mode: "list", value: "Sheet1" },
      columns: {
        mappingMode: "defineBelow",
        value: null,
      },
    },
    registry,
  );

  const updateColumns = updateSchema.fields.find((field) => field.name === "columns");
  assert.equal(updateSchema.fields.some((field) => field.name === "options"), true);
  assert.equal(updateColumns?.type, "resourceMapper");
  assert.equal(updateColumns?.resourceMapper?.mode, "update");
  assert.equal(updateColumns?.resourceMapper?.resourceMapperMethod, "getMappingColumns");

  const spreadsheetCreateSchema = createParameterSchemaForNode(
    "n8n-nodes-base.googleSheets",
    {
      authentication: "oAuth2",
      resource: "spreadsheet",
      operation: "create",
      title: "DX launch tracking",
      sheetsUi: {
        sheetValues: [
          {
            title: "Runs",
            hidden: false,
          },
        ],
      },
    },
    registry,
  );

  const names = spreadsheetCreateSchema.fields.map((field) => field.name);
  assert.equal(names.includes("title"), true);
  assert.equal(names.includes("sheetsUi"), true);
  assert.equal(names.includes("options"), true);

  const sheetsUi = spreadsheetCreateSchema.fields.find((field) => field.name === "sheetsUi");
  assert.equal(sheetsUi?.type, "fixedCollection");
  assert.equal(
    sheetsUi?.collectionItems?.[0]?.fields.find((field) => field.name === "title")?.value,
    "Runs",
  );
});

test("n8n Studio builds Google Sheets source-backed read filter schemas", () => {
  const sourceNodeType = createGoogleSheetsNodeTypeFromSource(readGoogleSheetsSources());
  const registry = { [sourceNodeType.name]: sourceNodeType };
  const extraction = extractGoogleSheetsSourceParameters(readGoogleSheetsSources());
  const readSchema = createParameterSchemaForNode(
    "n8n-nodes-base.googleSheets",
    {
      authentication: "oAuth2",
      resource: "sheet",
      operation: "read",
      documentId: { mode: "list", value: "spreadsheet-1" },
      sheetName: { mode: "list", value: "Sheet1" },
      filtersUI: {
        values: [
          {
            lookupColumn: "Email",
            lookupValue: "team@example.com",
          },
        ],
      },
      combineFilters: "AND",
    },
    registry,
  );

  const extractedFilters = extraction.parameters.find((field) => field.name === "filtersUI");
  const extractedFilterColumn = extractedFilters?.childParameters?.find(
    (field) => field.name === "lookupColumn",
  );
  assert.equal(extractedFilters?.type, "fixedCollection");
  assert.equal(
    extractedFilterColumn?.dynamicOptions?.loadMethod,
    "getSheetHeaderRowWithGeneratedColumnNames",
  );

  const filters = readSchema.fields.find((field) => field.name === "filtersUI");
  const filterColumn = filters?.collectionItems?.[0]?.fields.find(
    (field) => field.name === "lookupColumn",
  );
  const combineFilters = readSchema.fields.find((field) => field.name === "combineFilters");
  const readOptions = readSchema.fields.find((field) => field.name === "options");
  const dataLocation = readOptions?.childFields?.find(
    (field) => field.name === "dataLocationOnSheet",
  );
  const outputFormatting = readOptions?.childFields?.find(
    (field) => field.name === "outputFormatting",
  );
  assert.equal(filters?.type, "fixedCollection");
  assert.equal(filterColumn?.value, "Email");
  assert.equal(combineFilters?.type, "options");
  assert.deepEqual(combineFilters?.options?.map((option) => option.value), [
    "AND",
    "OR",
  ]);
  assert.equal(readOptions?.type, "collection");
  assert.equal(dataLocation?.type, "fixedCollection");
  assert.equal(
    dataLocation?.childFields?.some((field) => field.name === "rangeDefinition"),
    true,
  );
  assert.equal(outputFormatting?.type, "fixedCollection");
  assert.equal(
    outputFormatting?.childFields?.some((field) => field.name === "general"),
    true,
  );
  assert.equal(
    readSchema.dynamicLoadBoundaries.some(
      (boundary) =>
        boundary.includes("n8n-nodes-base.googleSheets.filtersUI") &&
        boundary.endsWith(".getSheetHeaderRowWithGeneratedColumnNames"),
    ),
    true,
  );
});

test("n8n Studio extracts Notion v2 parameters from local n8n source", () => {
  const extraction = extractNotionV2SourceParameters(readNotionV2Sources());

  assert.equal(extraction.sourcePath, "nodes/Notion/v2/VersionDescription.ts");
  assert.equal(extraction.generatedFrom, "source-parameter-description");
  assert.equal(extraction.parameters.length >= 30, true);

  const fieldNames = extraction.parameters.map((parameter) => parameter.name);
  assert.deepEqual(
    [
      "authentication",
      "notionNotice",
      "Credentials",
      "resource",
      "operation",
      "databaseId",
      "pageId",
      "blockId",
      "blockUi",
      "filterType",
      "filters",
      "propertiesUi",
      "userId",
    ].every((name) => fieldNames.includes(name)),
    true,
  );

  const resource = extraction.parameters.find((parameter) => parameter.name === "resource");
  assert.deepEqual(resource?.options?.map((option) => option.value), [
    "block",
    "database",
    "databasePage",
    "page",
    "user",
  ]);

  const databaseId = extraction.parameters.find(
    (parameter) =>
      parameter.name === "databaseId" &&
      parameter.displayOptions?.show?.resource?.includes("databasePage"),
  );
  assert.equal(databaseId?.type, "resourceLocator");
  assert.equal(
    databaseId?.resourceLocatorModes?.some(
      (mode) => mode.searchListMethod === "getDatabases",
    ),
    true,
  );

  const pageId = extraction.parameters.find(
    (parameter) =>
      parameter.name === "pageId" &&
      parameter.displayOptions?.show?.resource?.includes("page"),
  );
  assert.equal(pageId?.type, "resourceLocator");
  assert.deepEqual(pageId?.resourceLocatorModes?.map((mode) => mode.name), [
    "url",
    "id",
  ]);

  const propertiesUi = extraction.parameters.find(
    (parameter) => parameter.name === "propertiesUi",
  );
  const propertyKey = propertiesUi?.childParameters?.find(
    (parameter) => parameter.name === "key",
  );
  const blockUi = extraction.parameters.find(
    (parameter) =>
      parameter.name === "blockUi" &&
      parameter.displayOptions?.show?.resource?.includes("databasePage"),
  );
  const blockType = blockUi?.childParameters?.find(
    (parameter) => parameter.name === "type",
  );
  assert.equal(propertiesUi?.type, "fixedCollection");
  assert.equal(propertyKey?.dynamicOptions?.loadMethod, "getDatabaseProperties");
  assert.equal(blockUi?.type, "fixedCollection");
  assert.equal(blockType?.dynamicOptions?.loadMethod, "getBlockTypes");

  const filters = extraction.parameters.find(
    (parameter) =>
      parameter.name === "filters" &&
      parameter.displayOptions?.show?.resource?.includes("databasePage"),
  );
  const filterKey = filters?.childParameters?.find(
    (parameter) => parameter.name === "key",
  );
  const filterCondition = filters?.childParameters?.find(
    (parameter) =>
      parameter.name === "condition" &&
      parameter.displayOptions?.show?.type?.includes("rich_text"),
  );
  assert.equal(filters?.type, "fixedCollection");
  assert.equal(filterKey?.dynamicOptions?.loadMethod, "getFilterProperties");
  assert.deepEqual(
    filterCondition?.options?.map((option) => option.value).slice(0, 3),
    ["equals", "does_not_equal", "contains"],
  );
});

test("n8n Studio builds Notion v2 source-backed database page schemas", () => {
  const sourceNodeType = createNotionV2NodeTypeFromSource(readNotionV2Sources());
  const registry = { [sourceNodeType.name]: sourceNodeType };
  const schema = createParameterSchemaForNode(
    "n8n-nodes-base.notionV2",
    {
      authentication: "apiKey",
      resource: "databasePage",
      operation: "create",
      databaseId: { mode: "list", value: "database-1" },
      propertiesUi: {
        propertyValues: [
          {
            key: "Status",
            type: "select",
            selectValue: "In Progress",
          },
        ],
      },
      blockUi: {
        blockValues: [
          {
            type: "paragraph",
            richText: false,
            textContent: "Launch notes",
          },
        ],
      },
    },
    registry,
  );

  assert.deepEqual(sourceNodeType.credentials.map((credential) => credential.name), [
    "notionApi",
    "notionOAuth2Api",
  ]);

  const names = schema.fields.map((field) => field.name);
  assert.equal(names.includes("databaseId"), true);
  assert.equal(names.includes("title"), true);
  assert.equal(names.includes("propertiesUi"), true);
  assert.equal(names.includes("blockUi"), true);

  const databaseId = schema.fields.find((field) => field.name === "databaseId");
  const propertiesUi = schema.fields.find((field) => field.name === "propertiesUi");
  const blockUi = schema.fields.find((field) => field.name === "blockUi");
  const propertyItem = propertiesUi?.collectionItems?.[0];
  const blockItem = blockUi?.collectionItems?.[0];
  assert.equal(databaseId?.type, "resourceLocator");
  assert.equal(propertiesUi?.type, "fixedCollection");
  assert.equal(blockUi?.type, "fixedCollection");
  assert.equal(
    propertyItem?.fields.find((field) => field.name === "key")?.value,
    "Status",
  );
  assert.equal(
    blockItem?.fields.find((field) => field.name === "type")?.value,
    "paragraph",
  );
  assert.equal(
    blockItem?.fields.find((field) => field.name === "textContent")?.value,
    "Launch notes",
  );
  assert.equal(
    schema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.notionV2.databaseId.getDatabases",
    ),
    true,
  );
  assert.equal(
    schema.dynamicLoadBoundaries.some((boundary) =>
      boundary.endsWith(".getDatabaseProperties"),
    ),
    true,
  );
  assert.equal(
    schema.dynamicLoadBoundaries.some((boundary) =>
      boundary.endsWith(".getBlockTypes"),
    ),
    true,
  );
});

test("n8n Studio builds Notion v2 source-backed database page filter schemas", () => {
  const sourceNodeType = createNotionV2NodeTypeFromSource(readNotionV2Sources());
  const registry = { [sourceNodeType.name]: sourceNodeType };
  const schema = createParameterSchemaForNode(
    "n8n-nodes-base.notionV2",
    {
      authentication: "apiKey",
      resource: "databasePage",
      operation: "getAll",
      databaseId: { mode: "list", value: "database-1" },
      filterType: "manual",
      matchType: "allFilters",
      filters: {
        conditions: [
          {
            key: "Name|rich_text",
            type: "rich_text",
            condition: "contains",
            richTextValue: "Friday",
          },
        ],
      },
    },
    registry,
  );

  const filterType = schema.fields.find((field) => field.name === "filterType");
  const matchType = schema.fields.find((field) => field.name === "matchType");
  const filters = schema.fields.find((field) => field.name === "filters");
  const filterItem = filters?.collectionItems?.[0];

  assert.equal(filterType?.type, "options");
  assert.deepEqual(filterType?.options?.map((option) => option.value), [
    "none",
    "manual",
    "json",
  ]);
  assert.deepEqual(matchType?.options?.map((option) => option.value), [
    "anyFilter",
    "allFilters",
  ]);
  assert.equal(filters?.type, "fixedCollection");
  assert.equal(
    filterItem?.fields.find((field) => field.name === "key")?.value,
    "Name|rich_text",
  );
  assert.equal(
    filterItem?.fields.find((field) => field.name === "richTextValue")?.value,
    "Friday",
  );
  assert.equal(
    filterItem?.fields
      .find((field) => field.name === "condition")
      ?.options?.some((option) => option.value === "contains"),
    true,
  );
  assert.equal(
    schema.dynamicLoadBoundaries.some((boundary) =>
      boundary.endsWith(".getFilterProperties"),
    ),
    true,
  );
});

test("n8n Studio extracts Slack message input parameters from local n8n source", () => {
  const extraction = extractSlackMessageSourceParameters(readSlackMessageSource());

  assert.equal(extraction.sourcePath, "nodes/Slack/V2/MessageDescription.ts");
  assert.equal(extraction.generatedFrom, "source-parameter-description");
  assert.equal(extraction.parameters.length >= 10, true);
  assert.equal(extraction.unsupportedParameterNames.includes("attachments"), true);

  const fieldNames = extraction.parameters.map((parameter) => parameter.name);
  assert.deepEqual(
    ["select", "channelId", "user", "messageType", "text", "blocksUi", "noticeAttachments", "attachments", "otherOptions"].every(
      (name) => fieldNames.includes(name),
    ),
    true,
  );

  const channel = extraction.parameters.find((parameter) => parameter.name === "channelId");
  assert.equal(channel?.type, "resourceLocator");
  assert.deepEqual(channel?.resourceLocatorModes?.map((mode) => mode.name), [
    "list",
    "id",
    "name",
    "url",
  ]);
  assert.equal(channel?.resourceLocatorModes?.[0]?.searchListMethod, "getChannels");

  const attachments = extraction.parameters.find((parameter) => parameter.name === "attachments");
  assert.equal(attachments?.type, "collection");
  assert.equal(attachments?.childParameters?.some((child) => child.name === "fallback"), true);
  assert.equal(attachments?.childParameters?.some((child) => child.name === "color"), true);
  assert.equal(attachments?.childParameters?.some((child) => child.name === "fields"), true);

  const attachmentFields = attachments?.childParameters?.find((child) => child.name === "fields");
  assert.equal(attachmentFields?.type, "fixedCollection");
  assert.equal(attachmentFields?.childParameters?.some((child) => child.name === "title"), true);
  assert.equal(attachmentFields?.childParameters?.some((child) => child.name === "short"), true);

  const options = extraction.parameters.find((parameter) => parameter.name === "otherOptions");
  assert.equal(options?.type, "collection");
  assert.equal(options?.childParameters?.some((child) => child.name === "includeLinkToWorkflow"), true);
  assert.equal(options?.childParameters?.some((child) => child.name === "botProfile"), true);
});

test("n8n Studio builds Slack post parameters from source-backed definitions", () => {
  const sourceNodeType = createSlackMessageNodeTypeFromSource(readSlackMessageSource());
  const schema = createParameterSchemaForNode(
    "n8n-nodes-base.slack",
    {
      resource: "message",
      operation: "post",
      select: "channel",
      messageType: "text",
    },
    { [sourceNodeType.name]: sourceNodeType },
  );

  const names = schema.fields.map((field) => field.name);
  assert.equal(names.includes("messageType"), true);
  assert.equal(names.includes("text"), true);
  assert.equal(names.includes("blocksUi"), false);
  assert.equal(names.includes("channelId"), true);
  assert.equal(schema.dynamicLoadBoundaries.includes("n8n-nodes-base.slack.channelId.getChannels"), true);

  const text = schema.fields.find((field) => field.name === "text");
  assert.equal(text?.label, "Message Text");
  assert.equal(text?.required, true);
  assert.equal(text?.type, "string");

  const messageType = schema.fields.find((field) => field.name === "messageType");
  assert.deepEqual(messageType?.options?.map((option) => option.value), [
    "text",
    "block",
    "attachment",
  ]);
});

test("n8n Studio carries nested source children into rendered parameter fields", () => {
  const sourceNodeType = createSlackMessageNodeTypeFromSource(readSlackMessageSource());
  const schema = createParameterSchemaForNode(
    "n8n-nodes-base.slack",
    {
      resource: "message",
      operation: "post",
      messageType: "attachment",
    },
    { [sourceNodeType.name]: sourceNodeType },
  );

  const attachments = schema.fields.find((field) => field.name === "attachments");
  assert.equal(attachments?.renderingBoundary, "complex-source-field");
  assert.equal(attachments?.childFields?.some((child) => child.name === "fallback"), true);
  assert.equal(attachments?.childFields?.some((child) => child.name === "fields"), true);

  const nestedFields = attachments?.childFields?.find((child) => child.name === "fields");
  assert.equal(nestedFields?.childFields?.some((child) => child.name === "value"), true);
});

test("n8n Studio preserves nested collection values for editor controls", () => {
  const sourceNodeType = createSlackMessageNodeTypeFromSource(readSlackMessageSource());
  const schema = createParameterSchemaForNode(
    "n8n-nodes-base.slack",
    {
      resource: "message",
      operation: "post",
      messageType: "attachment",
      attachments: [
        {
          fallback: "Incident fallback",
          color: "#36a64f",
          fields: {
            item: [
              {
                title: "Severity",
                value: "High",
                short: false,
              },
            ],
          },
        },
      ],
    },
    { [sourceNodeType.name]: sourceNodeType },
  );

  const attachments = schema.fields.find((field) => field.name === "attachments");
  assert.equal(attachments?.collectionItems?.length, 1);

  const attachmentItem = attachments?.collectionItems?.[0];
  assert.equal(attachmentItem?.fields.find((field) => field.name === "fallback")?.value, "Incident fallback");
  assert.equal(attachmentItem?.fields.find((field) => field.name === "color")?.value, "#36a64f");

  const fields = attachmentItem?.fields.find((field) => field.name === "fields");
  const nestedFieldItem = fields?.collectionItems?.[0];
  assert.equal(nestedFieldItem?.label, "Item 1");
  assert.equal(nestedFieldItem?.fields.find((field) => field.name === "title")?.value, "Severity");
  assert.equal(nestedFieldItem?.fields.find((field) => field.name === "value")?.value, "High");
  assert.equal(nestedFieldItem?.fields.find((field) => field.name === "short")?.value, false);

  const controlSource = fs.readFileSync(
    path.join(repoRoot, "examples", "n8n-studio", "components", "n8n-studio", "parameter-field-control.tsx"),
    "utf8",
  );
  assert.match(controlSource, /data-collection-field/);
  assert.match(controlSource, /data-collection-item/);
  assert.match(controlSource, /collectionItems/);
});
