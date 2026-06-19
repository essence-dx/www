import fs from "node:fs";
import path from "node:path";

import { n8nCatalogSummary } from "../../lib/n8n-studio/catalog";
import {
  createGeneratedNodeTypeRegistry,
  createStudioCatalogFromGeneratedConnectors,
} from "../../lib/n8n-studio/generated-connectors/index";
import {
  mergeNodeTypeRegistries,
  n8nNodeTypeRegistry,
} from "../../lib/n8n-studio/node-type-registry";
import type { NodeTypeDescription } from "../../lib/n8n-studio/node-types/types";
import {
  createAirtableNodeTypeFromSource,
  createDiscordNodeTypeFromSource,
  createGoogleSheetsNodeTypeFromSource,
  createNotionV2NodeTypeFromSource,
  createPostgresNodeTypeFromSource,
} from "../../lib/n8n-studio/source-parameters";
import {
  createN8nStudioState,
  type N8nStudioStateOptions,
} from "../../lib/n8n-studio/studio-state";
import type { CatalogSummary, N8nStudioState } from "../../lib/n8n-studio/types";
import { createCatalogResponse } from "./catalog-response";

const generatedConnectorRelativePath = path.join(
  "integrations",
  "n8n-nodes-base",
  "generated",
  "dx-automations-connectors.json",
);

function googleSheetsActionPath(...segments: string[]) {
  return path.join(
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

function airtableActionPath(...segments: string[]) {
  return path.join(
    "integrations",
    "n8n-nodes-base",
    "nodes",
    "Airtable",
    "v2",
    "actions",
    ...segments,
  );
}

function postgresActionPath(...segments: string[]) {
  return path.join(
    "integrations",
    "n8n-nodes-base",
    "nodes",
    "Postgres",
    "v2",
    "actions",
    ...segments,
  );
}

function notionPath(...segments: string[]) {
  return path.join(
    "integrations",
    "n8n-nodes-base",
    "nodes",
    "Notion",
    ...segments,
  );
}

function discordActionPath(...segments: string[]) {
  return path.join(
    "integrations",
    "n8n-nodes-base",
    "nodes",
    "Discord",
    "v2",
    "actions",
    ...segments,
  );
}

const n8nDescriptionUtilitiesPath = path.join(
  "integrations",
  "n8n-nodes-base",
  "utils",
  "descriptions.ts",
);
const n8nSendAndWaitUtilitiesPath = path.join(
  "integrations",
  "n8n-nodes-base",
  "utils",
  "sendAndWait",
  "utils.ts",
);
const n8nSendAndWaitDescriptionsPath = path.join(
  "integrations",
  "n8n-nodes-base",
  "utils",
  "sendAndWait",
  "descriptions.ts",
);
const n8nFormNodePath = path.join(
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "Form",
  "Form.node.ts",
);
const n8nFormCommonDescriptionPath = path.join(
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "Form",
  "common.descriptions.ts",
);

const googleSheetsSourcePaths = {
  versionDescriptionSource: googleSheetsActionPath("versionDescription.ts"),
  sheetResourceSource: googleSheetsActionPath("sheet", "Sheet.resource.ts"),
  spreadsheetResourceSource: googleSheetsActionPath(
    "spreadsheet",
    "SpreadSheet.resource.ts",
  ),
  commonDescriptionSource: googleSheetsActionPath(
    "sheet",
    "commonDescription.ts",
  ),
  appendOperationSource: googleSheetsActionPath("sheet", "append.operation.ts"),
  appendOrUpdateOperationSource: googleSheetsActionPath(
    "sheet",
    "appendOrUpdate.operation.ts",
  ),
  clearOperationSource: googleSheetsActionPath("sheet", "clear.operation.ts"),
  createSheetOperationSource: googleSheetsActionPath(
    "sheet",
    "create.operation.ts",
  ),
  deleteOperationSource: googleSheetsActionPath("sheet", "delete.operation.ts"),
  readOperationSource: googleSheetsActionPath("sheet", "read.operation.ts"),
  updateOperationSource: googleSheetsActionPath("sheet", "update.operation.ts"),
  createSpreadsheetOperationSource: googleSheetsActionPath(
    "spreadsheet",
    "create.operation.ts",
  ),
  deleteSpreadsheetOperationSource: googleSheetsActionPath(
    "spreadsheet",
    "delete.operation.ts",
  ),
};

const airtableSourcePaths = {
  versionDescriptionSource: airtableActionPath("versionDescription.ts"),
  commonDescriptionSource: airtableActionPath("common.descriptions.ts"),
  baseResourceSource: airtableActionPath("base", "Base.resource.ts"),
  recordResourceSource: airtableActionPath("record", "Record.resource.ts"),
  recordCreateOperationSource: airtableActionPath(
    "record",
    "create.operation.ts",
  ),
  recordDeleteOperationSource: airtableActionPath(
    "record",
    "deleteRecord.operation.ts",
  ),
  recordGetOperationSource: airtableActionPath("record", "get.operation.ts"),
  recordSearchOperationSource: airtableActionPath(
    "record",
    "search.operation.ts",
  ),
  recordUpdateOperationSource: airtableActionPath(
    "record",
    "update.operation.ts",
  ),
  recordUpsertOperationSource: airtableActionPath(
    "record",
    "upsert.operation.ts",
  ),
  baseGetManyOperationSource: airtableActionPath(
    "base",
    "getMany.operation.ts",
  ),
  baseGetSchemaOperationSource: airtableActionPath(
    "base",
    "getSchema.operation.ts",
  ),
};

const postgresSourcePaths = {
  versionDescriptionSource: postgresActionPath("versionDescription.ts"),
  commonDescriptionSource: postgresActionPath("common.descriptions.ts"),
  databaseResourceSource: postgresActionPath("database", "Database.resource.ts"),
  deleteTableOperationSource: postgresActionPath(
    "database",
    "deleteTable.operation.ts",
  ),
  executeQueryOperationSource: postgresActionPath(
    "database",
    "executeQuery.operation.ts",
  ),
  insertOperationSource: postgresActionPath("database", "insert.operation.ts"),
  selectOperationSource: postgresActionPath("database", "select.operation.ts"),
  updateOperationSource: postgresActionPath("database", "update.operation.ts"),
  upsertOperationSource: postgresActionPath("database", "upsert.operation.ts"),
};

const discordSourcePaths = {
  versionDescriptionSource: discordActionPath("versionDescription.ts"),
  commonDescriptionSource: discordActionPath("common.description.ts"),
  descriptionUtilitiesSource: n8nDescriptionUtilitiesPath,
  sendAndWaitUtilitiesSource: n8nSendAndWaitUtilitiesPath,
  sendAndWaitDescriptionsSource: n8nSendAndWaitDescriptionsPath,
  formNodeSource: n8nFormNodePath,
  formCommonDescriptionSource: n8nFormCommonDescriptionPath,
  channelResourceSource: discordActionPath("channel", "index.ts"),
  messageResourceSource: discordActionPath("message", "index.ts"),
  memberResourceSource: discordActionPath("member", "index.ts"),
  channelCreateOperationSource: discordActionPath(
    "channel",
    "create.operation.ts",
  ),
  channelDeleteOperationSource: discordActionPath(
    "channel",
    "deleteChannel.operation.ts",
  ),
  channelGetOperationSource: discordActionPath("channel", "get.operation.ts"),
  channelGetAllOperationSource: discordActionPath(
    "channel",
    "getAll.operation.ts",
  ),
  channelUpdateOperationSource: discordActionPath(
    "channel",
    "update.operation.ts",
  ),
  messageSendOperationSource: discordActionPath(
    "message",
    "send.operation.ts",
  ),
  messageSendAndWaitOperationSource: discordActionPath(
    "message",
    "sendAndWait.operation.ts",
  ),
  messageDeleteOperationSource: discordActionPath(
    "message",
    "deleteMessage.operation.ts",
  ),
  messageGetOperationSource: discordActionPath("message", "get.operation.ts"),
  messageGetAllOperationSource: discordActionPath(
    "message",
    "getAll.operation.ts",
  ),
  messageReactOperationSource: discordActionPath(
    "message",
    "react.operation.ts",
  ),
  memberGetAllOperationSource: discordActionPath(
    "member",
    "getAll.operation.ts",
  ),
  memberRoleAddOperationSource: discordActionPath(
    "member",
    "roleAdd.operation.ts",
  ),
  memberRoleRemoveOperationSource: discordActionPath(
    "member",
    "roleRemove.operation.ts",
  ),
};

const notionSourcePaths = {
  versionDescriptionSource: notionPath("v2", "VersionDescription.ts"),
  blockDescriptionSource: notionPath(
    "shared",
    "descriptions",
    "BlockDescription.ts",
  ),
  blocksDescriptionSource: notionPath(
    "shared",
    "descriptions",
    "Blocks.ts",
  ),
  filtersDescriptionSource: notionPath(
    "shared",
    "descriptions",
    "Filters.ts",
  ),
  genericFunctionsSource: notionPath(
    "shared",
    "GenericFunctions.ts",
  ),
  databaseDescriptionSource: notionPath(
    "shared",
    "descriptions",
    "DatabaseDescription.ts",
  ),
  databasePageDescriptionSource: notionPath(
    "shared",
    "descriptions",
    "DatabasePageDescription.ts",
  ),
  pageDescriptionSource: notionPath(
    "shared",
    "descriptions",
    "PageDescription.ts",
  ),
  userDescriptionSource: notionPath(
    "shared",
    "descriptions",
    "UserDescription.ts",
  ),
};

function findRepoRoot(startDirectory: string) {
  let current = startDirectory;

  while (true) {
    if (fs.existsSync(path.join(current, generatedConnectorRelativePath))) {
      return current;
    }

    const parent = path.dirname(current);
    if (parent === current) {
      return startDirectory;
    }
    current = parent;
  }
}

function unavailableCatalog(issue: string): CatalogSummary {
  return {
    ...n8nCatalogSummary,
    generatedMetadata: {
      sourceAvailable: false,
      sourcePath: generatedConnectorRelativePath.replaceAll("\\", "/"),
      sourceRecordCount: 0,
      nodeTypeCount: 0,
      skippedRecordCount: 0,
      credentialTypeCount: 0,
      issue,
    },
  };
}

function readRequiredSourceFiles<TPaths extends Record<string, string>>(
  repoRoot: string,
  sourcePaths: TPaths,
): { [Key in keyof TPaths]: string } | undefined {
  const entries = Object.entries(sourcePaths);
  if (entries.some(([, sourcePath]) => !fs.existsSync(path.join(repoRoot, sourcePath)))) {
    return undefined;
  }

  return Object.fromEntries(
    entries.map(([key, sourcePath]) => [
      key,
      fs.readFileSync(path.join(repoRoot, sourcePath), "utf8"),
    ]),
  ) as { [Key in keyof TPaths]: string };
}

function createLocalSourceNodeTypeRegistry(repoRoot: string) {
  const registry: Record<string, NodeTypeDescription> = {};
  const airtableSources = readRequiredSourceFiles(
    repoRoot,
    airtableSourcePaths,
  );
  const googleSheetsSources = readRequiredSourceFiles(
    repoRoot,
    googleSheetsSourcePaths,
  );
  const postgresSources = readRequiredSourceFiles(
    repoRoot,
    postgresSourcePaths,
  );
  const discordSources = readRequiredSourceFiles(
    repoRoot,
    discordSourcePaths,
  );
  const notionSources = readRequiredSourceFiles(repoRoot, notionSourcePaths);

  if (airtableSources) {
    const airtable = createAirtableNodeTypeFromSource(airtableSources);
    registry[airtable.name] = airtable;
  }

  if (googleSheetsSources) {
    const googleSheets = createGoogleSheetsNodeTypeFromSource(googleSheetsSources);
    registry[googleSheets.name] = googleSheets;
  }

  if (postgresSources) {
    const postgres = createPostgresNodeTypeFromSource(postgresSources);
    registry[postgres.name] = postgres;
  }

  if (discordSources) {
    const discord = createDiscordNodeTypeFromSource(discordSources);
    registry[discord.name] = discord;
  }

  if (notionSources) {
    const notion = createNotionV2NodeTypeFromSource(notionSources);
    registry[notion.name] = notion;
  }

  return registry;
}

function readLocalGeneratedCatalog(startDirectory: string) {
  const repoRoot = findRepoRoot(startDirectory);
  const generatedConnectorPath = path.join(repoRoot, generatedConnectorRelativePath);
  const sourceBackedRegistry = createLocalSourceNodeTypeRegistry(repoRoot);

  if (!fs.existsSync(generatedConnectorPath)) {
    return {
      catalog: unavailableCatalog("generated connector catalog file is missing"),
      nodeTypeRegistry: mergeNodeTypeRegistries(
        sourceBackedRegistry,
        n8nNodeTypeRegistry,
      ),
    };
  }

  const generatedCatalog = JSON.parse(fs.readFileSync(generatedConnectorPath, "utf8"));
  const generated = createGeneratedNodeTypeRegistry(generatedCatalog);

  return {
    catalog: createStudioCatalogFromGeneratedConnectors(generatedCatalog),
    nodeTypeRegistry: mergeNodeTypeRegistries(
      generated.registry,
      sourceBackedRegistry,
      n8nNodeTypeRegistry,
    ),
  };
}

export function createCatalogResponseFromLocalGeneratedSource() {
  const { catalog } = readLocalGeneratedCatalog(process.cwd());

  return createCatalogResponse(catalog);
}

export type GeneratedStudioBoot = {
  state: N8nStudioState;
  nodeTypeRegistry: Record<string, NodeTypeDescription>;
};

export function createStudioBootFromLocalGeneratedSource(
  startDirectory = process.cwd(),
): GeneratedStudioBoot {
  const { catalog, nodeTypeRegistry } = readLocalGeneratedCatalog(startDirectory);
  const stateOptions: N8nStudioStateOptions = {
    catalog,
    nodeTypeRegistry,
  };

  return {
    state: createN8nStudioState(stateOptions),
    nodeTypeRegistry,
  };
}
