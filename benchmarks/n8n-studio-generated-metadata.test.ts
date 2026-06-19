import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import {
  createGeneratedNodeTypeRegistry,
  createStudioCatalogFromGeneratedConnectors,
} from "../examples/n8n-studio/lib/n8n-studio/generated-connectors/index";
import { createNodeCreatorState } from "../examples/n8n-studio/lib/n8n-studio/node-creator-actions";
import { createParameterSchemaForNode } from "../examples/n8n-studio/lib/n8n-studio/node-type-registry";
import { createCatalogResponse } from "../examples/n8n-studio/server/n8n-studio/catalog-response";
import { createStudioBootFromLocalGeneratedSource } from "../examples/n8n-studio/server/n8n-studio/generated-catalog-source";
import { createReadinessResponseFromLocalGeneratedSource } from "../examples/n8n-studio/server/n8n-studio/readiness-response";

const repoRoot = path.resolve(import.meta.dirname, "..");
const generatedCatalogPath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "generated",
  "dx-automations-connectors.json",
);

function readGeneratedCatalog() {
  return JSON.parse(fs.readFileSync(generatedCatalogPath, "utf8"));
}

test("n8n Studio adapts generated connector metadata into node type descriptions", () => {
  const generated = createGeneratedNodeTypeRegistry(readGeneratedCatalog());

  assert.equal(generated.coverage.sourceRecordCount, 536);
  assert.equal(generated.coverage.nodeTypeCount >= 470, true);
  assert.equal(generated.coverage.sourcePath, "integrations/n8n-nodes-base/generated/dx-automations-connectors.json");

  const gmail = generated.registry["n8n-nodes-base.gmail"];
  assert.equal(gmail?.displayName, "Gmail");
  assert.equal(gmail?.sourcePath, "nodes/Google/Gmail/Gmail.node.ts");
  assert.equal(gmail?.credentials.some((credential) => credential.name === "gmailOAuth2"), true);

  const schema = createParameterSchemaForNode("n8n-nodes-base.gmail", { resource: "message" }, generated.registry);
  const resource = schema.fields.find((field) => field.name === "resource");
  assert.equal(resource?.type, "options");
  assert.equal(resource?.options?.some((option) => option.value === "message"), true);

  const operation = schema.fields.find((field) => field.name === "operation");
  assert.equal(operation?.type, "options");
  assert.equal(operation?.options?.some((option) => option.value === "send"), true);

  const credential = schema.fields.find((field) => field.name === "credential");
  assert.equal(credential?.type, "credentialsSelect");
  assert.deepEqual(credential?.credentialTypes, ["googleApi", "gmailOAuth2"]);
});

test("n8n Studio prefers generated executable actions over operator-like operation metadata", () => {
  const generated = createGeneratedNodeTypeRegistry(readGeneratedCatalog());
  const chargebeeSchema = createParameterSchemaForNode(
    "n8n-nodes-base.chargebee",
    {},
    generated.registry,
  );
  const catalog = createStudioCatalogFromGeneratedConnectors(readGeneratedCatalog());
  const chargebeeCatalogNode = catalog.catalogNodes.find(
    (node) => node.id === "chargebee",
  );
  const operation = chargebeeSchema.fields.find(
    (field) => field.name === "operation",
  );
  const operationValues = operation?.options?.map((option) => option.value) ?? [];

  assert.equal(operation?.type, "options");
  assert.equal(operationValues.includes("cancel"), true);
  assert.equal(operationValues.includes("delete"), true);
  assert.equal(operationValues.includes("is"), false);
  assert.equal(operationValues.includes("gte"), false);
  assert.equal(chargebeeCatalogNode?.operations.includes("cancel"), true);
  assert.equal(chargebeeCatalogNode?.operations.includes("is"), false);
});

test("n8n Studio renders generated trigger nodes with source-only trigger configuration boundaries", () => {
  const generated = createGeneratedNodeTypeRegistry(readGeneratedCatalog());
  const schema = createParameterSchemaForNode(
    "n8n-nodes-base.acuitySchedulingTrigger",
    {},
    generated.registry,
  );
  const triggerBoundary = schema.fields.find(
    (field) => field.name === "triggerConfiguration",
  );
  const credential = schema.fields.find((field) => field.name === "credential");

  assert.equal(triggerBoundary?.type, "notice");
  assert.equal(triggerBoundary?.expressionEnabled, false);
  assert.match(String(triggerBoundary?.value), /source-backed trigger adapter/);
  assert.match(String(triggerBoundary?.description), /Live trigger execution remains disabled/);
  assert.equal(credential?.type, "credentialsSelect");
  assert.deepEqual(credential?.credentialTypes, [
    "acuitySchedulingApi",
    "acuitySchedulingOAuth2Api",
  ]);
});

test("n8n Studio renders generated-only node summaries as semantic parameter notices", () => {
  const generated = createGeneratedNodeTypeRegistry(readGeneratedCatalog());
  const schema = createParameterSchemaForNode(
    "n8n-nodes-base.actionNetwork",
    {
      resource: "event",
      operation: "create",
    },
    generated.registry,
  );
  const summary = schema.fields.find(
    (field) => field.name === "generatedNodeSummary",
  );
  const operation = schema.fields.find((field) => field.name === "operation");
  const createOperation = operation?.options?.find(
    (option) => option.value === "create",
  );
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

  assert.equal(summary?.type, "notice");
  assert.equal(summary?.expressionEnabled, false);
  assert.match(String(summary?.value), /Consume the Action Network API/);
  assert.match(String(summary?.description), /nodes\/ActionNetwork/);
  assert.equal(createOperation?.description, "Create an attendance");
  assert.match(controlSource, /data-parameter-notice-field/);
  assert.match(controlSource, /n8ns-field-description/);
});

test("n8n Studio catalog can be built from generated connector metadata", () => {
  const catalog = createStudioCatalogFromGeneratedConnectors(readGeneratedCatalog());

  assert.equal(catalog.generatedMetadata?.sourceRecordCount, 536);
  assert.equal(catalog.generatedMetadata?.nodeTypeCount >= 470, true);
  assert.equal(catalog.catalogNodes.length >= 470, true);

  const gmail = catalog.catalogNodes.find((node) => node.name === "Gmail");
  assert.equal(gmail?.displayName, "Gmail");
  assert.equal(gmail?.category, "Communication");
  assert.equal(gmail?.credentialTypes.includes("gmailOAuth2"), true);
  assert.equal(gmail?.operations.includes("send"), true);
  assert.equal(gmail?.sourcePath, "nodes/Google/Gmail/Gmail.node.ts");
});

test("n8n Studio catalog response accepts generated connector catalog data", () => {
  const catalog = createStudioCatalogFromGeneratedConnectors(readGeneratedCatalog());
  const response = createCatalogResponse(catalog);

  assert.equal(response.nodes.length >= 470, true);
  assert.equal(response.generatedMetadata?.sourceAvailable, true);
  assert.equal(response.generatedMetadata?.nodeTypeCount, catalog.generatedMetadata?.nodeTypeCount);
  assert.equal(response.liveProviderExecution, false);
});

test("n8n Studio default page boot can use generated catalog and registry data", () => {
  const boot = createStudioBootFromLocalGeneratedSource(repoRoot);
  const gmail = createNodeCreatorState(
    boot.state.catalog,
    "gmail",
    boot.nodeTypeRegistry,
  ).results.find((result) => result.node.id === "gmail");
  const pageSource = fs.readFileSync(
    path.join(repoRoot, "examples", "n8n-studio", "app", "page.tsx"),
    "utf8",
  );

  assert.equal(boot.state.catalog.generatedMetadata?.sourceAvailable, true);
  assert.equal(boot.state.catalog.catalogNodes.length >= 470, true);
  assert.equal(Boolean(boot.nodeTypeRegistry["n8n-nodes-base.gmail"]), true);
  assert.equal(gmail?.addable, true);
  assert.equal(
    boot.state.parameters.some((field) => field.name === "url"),
    true,
    "default selected HTTP node should keep source-backed HTTP parameters",
  );
  assert.match(pageSource, /createStudioBootFromLocalGeneratedSource/);
  assert.match(pageSource, /nodeTypeRegistry=/);
});

test("n8n Studio generated boot overlays source-backed Google Sheets parameters", () => {
  const boot = createStudioBootFromLocalGeneratedSource(repoRoot);
  const appendSchema = createParameterSchemaForNode(
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
    boot.nodeTypeRegistry,
  );
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
    },
    boot.nodeTypeRegistry,
  );

  const columns = appendSchema.fields.find((field) => field.name === "columns");
  const filters = readSchema.fields.find((field) => field.name === "filtersUI");
  const readOptions = readSchema.fields.find((field) => field.name === "options");
  assert.equal(columns?.type, "resourceMapper");
  assert.equal(columns?.resourceMapper?.resourceMapperMethod, "getMappingColumns");
  assert.equal(
    appendSchema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.googleSheets.columns.getMappingColumns",
    ),
    true,
  );
  assert.equal(filters?.type, "fixedCollection");
  assert.equal(
    filters?.collectionItems?.[0]?.fields.find((field) => field.name === "lookupColumn")?.value,
    "Email",
  );
  assert.equal(
    readOptions?.childFields?.some((field) => field.name === "dataLocationOnSheet"),
    true,
  );
  assert.equal(
    readOptions?.childFields?.some((field) => field.name === "outputFormatting"),
    true,
  );
});

test("n8n Studio generated boot overlays source-backed Airtable parameters", () => {
  const boot = createStudioBootFromLocalGeneratedSource(repoRoot);
  const schema = createParameterSchemaForNode(
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
    boot.nodeTypeRegistry,
  );

  const airtable = boot.nodeTypeRegistry["n8n-nodes-base.airtable"];
  const base = schema.fields.find((field) => field.name === "base");
  const table = schema.fields.find((field) => field.name === "table");
  const columns = schema.fields.find((field) => field.name === "columns");
  const options = schema.fields.find((field) => field.name === "options");

  assert.equal(
    airtable?.sourcePath,
    "nodes/Airtable/v2/actions/versionDescription.ts",
  );
  assert.equal(base?.type, "resourceLocator");
  assert.equal(table?.type, "resourceLocator");
  assert.equal(columns?.type, "resourceMapper");
  assert.equal(columns?.resourceMapper?.resourceMapperMethod, "getColumns");
  assert.equal(
    options?.childFields?.some((field) => field.name === "ignoreFields"),
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

test("n8n Studio generated boot overlays source-backed Postgres parameters", () => {
  const boot = createStudioBootFromLocalGeneratedSource(repoRoot);
  const schema = createParameterSchemaForNode(
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
    boot.nodeTypeRegistry,
  );

  const postgres = boot.nodeTypeRegistry["n8n-nodes-base.postgres"];
  const table = schema.fields.find((field) => field.name === "table");
  const columns = schema.fields.find((field) => field.name === "columns");

  assert.equal(
    postgres?.sourcePath,
    "nodes/Postgres/v2/actions/versionDescription.ts",
  );
  assert.equal(table?.type, "resourceLocator");
  assert.equal(columns?.type, "resourceMapper");
  assert.equal(columns?.resourceMapper?.resourceMapperMethod, "getMappingColumns");
  assert.equal(
    schema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.postgres.columns.getMappingColumns",
    ),
    true,
  );
});

test("n8n Studio generated boot overlays source-backed Notion v2 parameters", () => {
  const boot = createStudioBootFromLocalGeneratedSource(repoRoot);
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
    boot.nodeTypeRegistry,
  );
  const getAllSchema = createParameterSchemaForNode(
    "n8n-nodes-base.notionV2",
    {
      authentication: "apiKey",
      resource: "databasePage",
      operation: "getAll",
      databaseId: { mode: "list", value: "database-1" },
      filterType: "manual",
      filters: {
        conditions: [
          {
            key: "Name|rich_text",
            type: "rich_text",
            condition: "contains",
            richTextValue: "Launch",
          },
        ],
      },
    },
    boot.nodeTypeRegistry,
  );

  const notion = boot.nodeTypeRegistry["n8n-nodes-base.notionV2"];
  const databaseId = schema.fields.find((field) => field.name === "databaseId");
  const propertiesUi = schema.fields.find((field) => field.name === "propertiesUi");
  const blockUi = schema.fields.find((field) => field.name === "blockUi");
  const blockItem = blockUi?.collectionItems?.[0];
  const filterType = getAllSchema.fields.find((field) => field.name === "filterType");
  const filters = getAllSchema.fields.find((field) => field.name === "filters");
  const filterItem = filters?.collectionItems?.[0];

  assert.equal(notion?.sourcePath, "nodes/Notion/v2/VersionDescription.ts");
  assert.equal(databaseId?.type, "resourceLocator");
  assert.equal(propertiesUi?.type, "fixedCollection");
  assert.equal(blockUi?.type, "fixedCollection");
  assert.equal(
    blockItem?.fields.find((field) => field.name === "textContent")?.value,
    "Launch notes",
  );
  assert.equal(filterType?.type, "options");
  assert.equal(filters?.type, "fixedCollection");
  assert.equal(
    filterItem?.fields.find((field) => field.name === "richTextValue")?.value,
    "Launch",
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
  assert.equal(
    getAllSchema.dynamicLoadBoundaries.some((boundary) =>
      boundary.endsWith(".getFilterProperties"),
    ),
    true,
  );
});

test("n8n Studio generated boot overlays source-backed Discord v2 message parameters", () => {
  const boot = createStudioBootFromLocalGeneratedSource(repoRoot);
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
    },
    boot.nodeTypeRegistry,
  );

  const discord = boot.nodeTypeRegistry["n8n-nodes-base.discord"];
  const guildId = schema.fields.find((field) => field.name === "guildId");
  const channelId = schema.fields.find((field) => field.name === "channelId");
  const embeds = schema.fields.find((field) => field.name === "embeds");
  const embedItem = embeds?.collectionItems?.[0];

  assert.equal(
    discord?.sourcePath,
    "nodes/Discord/v2/actions/versionDescription.ts",
  );
  assert.equal(guildId?.type, "resourceLocator");
  assert.equal(channelId?.resourceLocatorModes?.[0]?.searchListMethod, "textChannelSearch");
  assert.equal(embeds?.type, "fixedCollection");
  assert.equal(
    embedItem?.fields.find((field) => field.name === "json")?.value,
    "{\"title\":\"Launch\"}",
  );
  assert.equal(
    schema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.discord.guildId.guildSearch",
    ),
    true,
  );
});

test("n8n Studio generated boot overlays source-backed Discord v2 channel and member parameters", () => {
  const boot = createStudioBootFromLocalGeneratedSource(repoRoot);
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
    boot.nodeTypeRegistry,
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
    boot.nodeTypeRegistry,
  );

  const channelOperation = channelSchema.fields.find(
    (field) => field.name === "operation",
  );
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

  assert.equal(
    channelOperation?.options?.some((option) => option.value === "deleteChannel"),
    true,
  );
  assert.equal(categoryId?.type, "resourceLocator");
  assert.equal(categoryId?.resourceLocatorModes?.[0]?.searchListMethod, "categorySearch");
  assert.equal(memberUserId?.type, "resourceLocator");
  assert.equal(
    memberUserId?.resourceLocatorModes?.[0]?.searchListMethod,
    "userSearch",
  );
  assert.equal(memberRole?.type, "multiOptions");
  assert.equal(memberRole?.dynamicOptions?.loadMethod, "getRoles");
  assert.equal(
    channelSchema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.discord.options.categoryId.categorySearch",
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

test("n8n Studio generated boot overlays source-backed Discord v2 message lookup and reaction parameters", () => {
  const boot = createStudioBootFromLocalGeneratedSource(repoRoot);
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
    boot.nodeTypeRegistry,
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
    boot.nodeTypeRegistry,
  );

  const getAllChannelId = getAllSchema.fields.find(
    (field) => field.name === "channelId",
  );
  const getAllLimit = getAllSchema.fields.find((field) => field.name === "limit");
  const reactMessageId = reactSchema.fields.find(
    (field) => field.name === "messageId",
  );
  const reactEmoji = reactSchema.fields.find((field) => field.name === "emoji");

  assert.equal(getAllChannelId?.type, "resourceLocator");
  assert.equal(
    getAllChannelId?.resourceLocatorModes?.[0]?.searchListMethod,
    "channelSearch",
  );
  assert.equal(getAllLimit?.value, 12);
  assert.equal(reactMessageId?.value, "message-1");
  assert.equal(reactEmoji?.value, ":rocket:");
  assert.equal(
    getAllSchema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.discord.channelId.channelSearch",
    ),
    true,
  );
});

test("n8n Studio generated boot overlays source-backed Discord v2 send-and-wait parameters", () => {
  const boot = createStudioBootFromLocalGeneratedSource(repoRoot);
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
    boot.nodeTypeRegistry,
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
    boot.nodeTypeRegistry,
  );

  const channelId = approvalSchema.fields.find((field) => field.name === "channelId");
  const responseType = approvalSchema.fields.find(
    (field) => field.name === "responseType",
  );
  const approvalOptions = approvalSchema.fields.find(
    (field) => field.name === "approvalOptions",
  );
  const approvalItem = approvalOptions?.collectionItems?.[0];
  const options = approvalSchema.fields.find((field) => field.name === "options");
  const optionsItem = options?.collectionItems?.[0];
  const limitWaitTime = optionsItem?.fields.find(
    (field) => field.name === "limitWaitTime",
  );
  const waitItem = limitWaitTime?.collectionItems?.[0];
  const defineForm = customFormSchema.fields.find(
    (field) => field.name === "defineForm",
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

  assert.equal(channelId?.type, "resourceLocator");
  assert.equal(
    channelId?.resourceLocatorModes?.[0]?.searchListMethod,
    "textChannelSearch",
  );
  assert.equal(
    responseType?.options?.some((option) => option.value === "customForm"),
    true,
  );
  assert.equal(approvalOptions?.type, "fixedCollection");
  assert.equal(
    approvalItem?.fields.find((field) => field.name === "approvalType")?.value,
    "double",
  );
  assert.equal(limitWaitTime?.type, "fixedCollection");
  assert.equal(
    waitItem?.fields.find((field) => field.name === "resumeAmount")?.value,
    30,
  );
  assert.equal(
    optionsItem?.fields.find((field) => field.name === "appendAttribution")?.value,
    false,
  );
  assert.equal(defineForm?.type, "options");
  assert.equal(formFields?.type, "fixedCollection");
  assert.equal(
    fieldType?.options?.some((option) => option.value === "dropdown"),
    true,
  );
  assert.equal(
    fieldOptionItem?.fields.find((field) => field.name === "option")?.value,
    "Production",
  );
  assert.equal(
    approvalSchema.dynamicLoadBoundaries.includes(
      "n8n-nodes-base.discord.channelId.textChannelSearch",
    ),
    true,
  );
});

test("n8n Studio readiness response can use generated catalog boot data", () => {
  const response = createReadinessResponseFromLocalGeneratedSource(repoRoot);
  const routeSource = fs.readFileSync(
    path.join(repoRoot, "examples", "n8n-studio", "app", "api", "n8n-studio", "readiness", "route.ts"),
    "utf8",
  );

  assert.equal(response.generatedMetadata?.sourceAvailable, true);
  assert.equal(response.catalogNodeCount >= 470, true);
  assert.equal(response.catalogNodeFiles, 536);
  assert.equal(response.editorSession.nodeType, "n8n-nodes-base.httpRequest");
  assert.equal(response.liveProviderExecution, false);
  assert.match(routeSource, /createReadinessResponseFromLocalGeneratedSource/);
});
