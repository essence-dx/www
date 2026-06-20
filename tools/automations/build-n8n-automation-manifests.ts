const fs = require("fs");
const path = require("path");

const root = path.resolve(__dirname, "..", "..");
const integrationRoot = path.join(root, "integrations", "n8n-nodes-base");
const manifestPath = path.join(integrationRoot, "dx-node-source-.dx/build-cache/manifest.json");
const generatedDir = path.join(integrationRoot, "generated");
const templateAutomationDir = path.join(root, "examples", "template", "automations");

const secretNamePattern = /(secret|token|password|api.?key|private.?key|client.?secret|signature)/i;
const preferredConnectorIds = [
  "n8n-nodes-base.slack",
  "n8n-nodes-base.notion",
  "n8n-nodes-base.gmail",
  "n8n-nodes-base.googleSheets",
  "n8n-nodes-base.github",
  "n8n-nodes-base.linear",
  "n8n-nodes-base.airtable",
  "n8n-nodes-base.stripe",
  "n8n-nodes-base.telegram",
  "n8n-nodes-base.webhook",
  "n8n-nodes-base.httpRequest",
  "n8n-nodes-base.cron",
];

function readText(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

function readJson(filePath) {
  return JSON.parse(readText(filePath).replace(/^\uFEFF/, ""));
}

function writeJson(filePath, value) {
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, `${JSON.stringify(value, null, 2)}\n`);
}

function posixPath(value) {
  return value.replace(/\\/g, "/");
}

function firstString(source, pattern) {
  const match = source.match(pattern);
  if (!match) return undefined;
  return String(match[2] ?? match[1]).trim();
}

function titleFromName(value) {
  return String(value)
    .replace(/([a-z0-9])([A-Z])/g, "$1 $2")
    .replace(/[_-]+/g, " ")
    .replace(/\b\w/g, (char) => char.toUpperCase())
    .trim();
}

function normalizeNodeName(value) {
  const clean = String(value || "")
    .replace(/Trigger$/, "Trigger")
    .trim();
  return clean ? clean[0].toLowerCase() + clean.slice(1) : "unknown";
}

function findMatchingBracket(source, startIndex, openChar, closeChar) {
  let depth = 0;
  let quote = "";
  let escaped = false;

  for (let index = startIndex; index < source.length; index += 1) {
    const char = source[index];

    if (quote) {
      if (escaped) {
        escaped = false;
      } else if (char === "\\") {
        escaped = true;
      } else if (char === quote) {
        quote = "";
      }
      continue;
    }

    if (char === "'" || char === '"' || char === "`") {
      quote = char;
      continue;
    }

    if (char === openChar) depth += 1;
    if (char === closeChar) {
      depth -= 1;
      if (depth === 0) return index;
    }
  }

  return -1;
}

function blockAfter(source, token, openChar = "[", closeChar = "]") {
  const tokenIndex = source.indexOf(token);
  if (tokenIndex < 0) return "";
  const start = source.indexOf(openChar, tokenIndex);
  if (start < 0) return "";
  const end = findMatchingBracket(source, start, openChar, closeChar);
  return end < 0 ? "" : source.slice(start + 1, end);
}

function blocksAfter(source, token, openChar = "[", closeChar = "]") {
  const blocks = [];
  let searchIndex = 0;

  while (searchIndex < source.length) {
    const tokenIndex = source.indexOf(token, searchIndex);
    if (tokenIndex < 0) break;
    const start = source.indexOf(openChar, tokenIndex);
    if (start < 0) break;
    const end = findMatchingBracket(source, start, openChar, closeChar);
    if (end < 0) break;
    blocks.push(source.slice(start + 1, end));
    searchIndex = end + 1;
  }

  return blocks;
}

function optionItems(optionsBlock) {
  const items = [];
  const optionPattern = /\{\s*name:\s*(['"`])([^'"`]+)\1,\s*value:\s*([^,\n}]+)([\s\S]*?)\}/g;
  let match;

  while ((match = optionPattern.exec(optionsBlock))) {
    const rawValue = match[3].trim();
    const valueMatch = rawValue.match(/^(['"`])([^'"`]+)\1$/);
    const action = firstString(match[4], /\baction:\s*(['"`])([^'"`]+)\1/);
    items.push({
      name: match[2].trim(),
      value: valueMatch ? valueMatch[2].trim() : rawValue.replace(/[{},]/g, "").trim(),
      ...(action ? { action } : {}),
    });
  }

  return items;
}

function uniqueBy(items, keyFn) {
  const seen = new Set();
  const output = [];

  for (const item of items) {
    const key = keyFn(item);
    if (seen.has(key)) continue;
    seen.add(key);
    output.push(item);
  }

  return output;
}

function extractOptionGroup(source, propertyName) {
  const groups = [];
  const namePattern = new RegExp(`\\bname:\\s*['"\`]${propertyName}['"\`]`, "g");
  let match;

  while ((match = namePattern.exec(source))) {
    const nameIndex = match.index;
    const optionsIndex = source.indexOf("options:", nameIndex);
    if (optionsIndex < 0) {
      continue;
    }
    const start = source.indexOf("[", optionsIndex);
    if (start < 0) {
      continue;
    }
    const end = findMatchingBracket(source, start, "[", "]");
    if (end < 0) {
      continue;
    }
    groups.push(...optionItems(source.slice(start + 1, end)));
    namePattern.lastIndex = end + 1;
  }

  return uniqueBy(groups, (item) => `${item.value}:${item.name}`);
}

function extractCredentials(source) {
  return uniqueBy(
    blocksAfter(source, "credentials:")
      .flatMap((block) => Array.from(block.matchAll(/\bname:\s*(['"`])([^'"`]+)\1/g), (match) => match[2])),
    (value) => value,
  );
}

function extractPropertyFields(source) {
  const propertiesIndex = source.indexOf("properties:");
  const equalsIndex = propertiesIndex >= 0 ? source.indexOf("=", propertiesIndex) : -1;
  const start = equalsIndex >= 0 ? source.indexOf("[", equalsIndex) : -1;
  const end = start >= 0 ? findMatchingBracket(source, start, "[", "]") : -1;
  const propertiesBlock = end >= 0 ? source.slice(start + 1, end) : "";
  const fields = [];
  const fieldPattern =
    /\{\s*displayName:\s*(['"`])([^'"`]+)\1,\s*name:\s*(['"`])([^'"`]+)\3,\s*type:\s*(['"`])([^'"`]+)\5([\s\S]*?)\n\s*\}/g;
  let match;

  while ((match = fieldPattern.exec(propertiesBlock))) {
    const name = match[4].trim();
    const trailing = match[7] || "";
    const secret = secretNamePattern.test(name) || /password:\s*true/.test(trailing);
    fields.push({
      display_name: match[2].trim(),
      name,
      type: match[6].trim(),
      required: /required:\s*true/.test(trailing),
      secret,
      default_provided: /\bdefault:/.test(trailing),
    });
  }

  return uniqueBy(fields, (field) => field.name);
}

function inferCredentialAuthKind(name, source, extensions, fields) {
  const lowerName = name.toLowerCase();
  const lowerSource = source.toLowerCase();
  const lowerExtensions = extensions.map((item) => item.toLowerCase());

  if (lowerName.includes("oauth2") || lowerExtensions.some((item) => item.includes("oauth2"))) {
    return "oauth2";
  }
  if (lowerName.includes("oauth1") || lowerExtensions.some((item) => item.includes("oauth1"))) {
    return "oauth1";
  }
  if (
    lowerName.includes("basic") ||
    lowerExtensions.some((item) => item.includes("basic")) ||
    lowerSource.includes("basicauth")
  ) {
    return "basic_auth";
  }
  if (
    lowerSource.includes("authorization: '=bearer") ||
    fields.some((field) => /access.?token|bearer/i.test(field.name))
  ) {
    return "bearer_token";
  }
  if (fields.length === 0) return "none";
  if (fields.some((field) => /api.?key|token|secret/i.test(field.name))) return "api_key";
  return "custom";
}

function credentialExtensions(source) {
  const block = blockAfter(source, "extends");
  return block ? Array.from(block.matchAll(/['"`]([^'"`]+)['"`]/g), (match) => match[1]) : [];
}

function credentialRecord(entry) {
  const sourceFile = path.join(integrationRoot, entry.path);
  const source = readText(sourceFile);
  const id = firstString(source, /\bname\s*=\s*(['"`])([^'"`]+)\1/) || entry.name;
  const displayName =
    firstString(source, /\bdisplayName\s*=\s*(['"`])([^'"`]+)\1/) || titleFromName(id);
  const extensions = credentialExtensions(source);
  const fields = extractPropertyFields(source).map((field) => ({
    display_name: field.display_name,
    name: field.name,
    type: field.type,
    required: field.required,
    secret: field.secret,
    default_provided: field.default_provided,
  }));
  const authKind = inferCredentialAuthKind(id, source, extensions, fields);

  return {
    id,
    display_name: displayName,
    source_file: posixPath(entry.path),
    auth_kind: authKind,
    extends: extensions,
    documentation_url: firstString(source, /\bdocumentationUrl\s*=\s*(['"`])([^'"`]+)\1/) || null,
    fields,
    secret_fields: fields.filter((field) => field.secret).map((field) => field.name),
    storage_policy: "dx-keychain-or-zed-credential-provider",
    redaction: {
      secrets_included: false,
      defaults_include_secret_templates: false,
    },
    source_provenance: "n8n-nodes-base",
  };
}

function listTypeScriptFiles(dir) {
  if (!fs.existsSync(dir)) return [];
  const output = [];

  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    if (entry.name === "test" || entry.name === "__schema__" || entry.name === "__tests__") {
      continue;
    }
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      output.push(...listTypeScriptFiles(fullPath));
    } else if (
      entry.name.endsWith(".node.ts") ||
      entry.name.endsWith("Description.ts") ||
      entry.name.endsWith(".resource.ts") ||
      entry.name.endsWith(".operation.ts") ||
      entry.name === "GenericFunctions.ts"
    ) {
      output.push(fullPath);
    }
  }

  return output;
}

const nodeCorpusCache = new Map();
function nodeCorpus(entry) {
  if (nodeCorpusCache.has(entry.folder)) return nodeCorpusCache.get(entry.folder);
  const folderPath = path.join(integrationRoot, "nodes", entry.folder);
  const files = listTypeScriptFiles(folderPath);
  const source = files
    .map((filePath) => readText(filePath))
    .join("\n\n");
  nodeCorpusCache.set(entry.folder, source);
  return source;
}

function sidecarMetadata(entry) {
  const sidecarPath = path
    .join(integrationRoot, entry.path)
    .replace(/\.node\.ts$/, ".node.json");
  if (!fs.existsSync(sidecarPath)) return {};
  try {
    return readJson(sidecarPath);
  } catch {
    return {};
  }
}

function connectorRecord(entry, credentialMap) {
  const sourceFile = path.join(integrationRoot, entry.path);
  const source = readText(sourceFile);
  const corpus = `${source}\n\n${nodeCorpus(entry)}`;
  const sidecar = sidecarMetadata(entry);
  const nodeName = firstString(source, /\bname:\s*(['"`])([^'"`]+)\1/) || normalizeNodeName(entry.name);
  const credentialTypeNames = uniqueBy(extractCredentials(corpus), (value) => value).filter(
    (value) => credentialMap.has(value),
  );
  const resources = extractOptionGroup(corpus, "resource");
  const operations = extractOptionGroup(corpus, "operation");
  const authKinds = uniqueBy(
    credentialTypeNames.map((name) => credentialMap.get(name)?.auth_kind || "custom"),
    (value) => value,
  );
  const isTrigger = /Trigger/.test(entry.name) || /Trigger\.node\.ts$/.test(entry.path);
  const usableAsTool = /\busableAsTool:\s*true/.test(corpus);
  const status = credentialTypeNames.length > 0 ? "needs_credential" : "ready";

  return {
    id: `n8n-nodes-base.${nodeName}`,
    display_name:
      firstString(source, /\bdisplayName:\s*(['"`])([^'"`]+)\1/) ||
      sidecar.displayName ||
      titleFromName(entry.name),
    node_name: nodeName,
    source_file: posixPath(entry.path),
    folder: posixPath(entry.folder),
    categories: Array.isArray(sidecar.categories) ? sidecar.categories : [],
    aliases: Array.isArray(sidecar.alias) ? sidecar.alias : [],
    description:
      firstString(source, /\bdescription:\s*(['"`])([^'"`]+)\1/) ||
      `DX Automations connector metadata for ${titleFromName(entry.name)}.`,
    credential_type_names: credentialTypeNames,
    auth_kinds: authKinds.length > 0 ? authKinds : ["none"],
    resources,
    operations,
    actions: uniqueBy(
      operations
        .filter((operation) => operation.action)
        .map((operation) => ({
          name: operation.name,
          value: operation.value,
          action: operation.action,
        })),
      (item) => `${item.value}:${item.action}`,
    ),
    status,
    workflow_node: {
      ready: true,
      trigger: isTrigger,
      usable_as_tool: usableAsTool,
      run_mode: credentialTypeNames.length > 0 ? "credential-gated" : "metadata-ready",
    },
    risk: credentialTypeNames.length > 0 ? "needs_credential" : "ready",
    source_provenance: "n8n-nodes-base",
  };
}

function manifestSource(manifest) {
  return {
    upstream: manifest.source.upstream,
    local_clone: manifest.source.local_clone,
    copied_to: manifest.source.copied_to,
    package: manifest.source.package,
    license_files: manifest.source.license_files,
    manifest: "integrations/n8n-nodes-base/dx-node-source-.dx/build-cache/manifest.json",
    provenance: "n8n-nodes-base",
  };
}

function commandBridge() {
  return {
    commands: {
      connectors: "dx automations connectors --json",
      credentials: "dx automations credentials --json",
      run: "dx automations run --json",
    },
    receipts_dir: "G:/Dx/.dx/receipts/automations",
    zed_contract: "metadata-json-first",
    runtime_execution: "blocked-until-explicit-workflow-and-credential-approval",
  };
}

function buildTemplateMetadata(connectorsManifest, credentialsManifest, readinessManifest) {
  const byId = new Map(connectorsManifest.connectors.map((connector) => [connector.id, connector]));
  const highlightedConnectors = [];

  for (const id of preferredConnectorIds) {
    const connector = byId.get(id);
    if (connector) highlightedConnectors.push(connector);
  }

  for (const connector of connectorsManifest.connectors) {
    if (highlightedConnectors.length >= 12) break;
    if (!highlightedConnectors.some((item) => item.id === connector.id)) {
      highlightedConnectors.push(connector);
    }
  }

  const highlightedCredentials = credentialsManifest.credentials
    .filter((credential) => ["oauth2", "bearer_token", "api_key", "basic_auth"].includes(credential.auth_kind))
    .slice(0, 10);

  const connectorRows = highlightedConnectors.map((connector) => ({
    id: connector.id,
    displayName: connector.display_name,
    status: connector.status,
    authKinds: connector.auth_kinds,
    credentials: connector.credential_type_names,
    sourceFile: connector.source_file,
    resources: connector.resources.slice(0, 5),
    operations: connector.operations.slice(0, 6),
    workflowNode: connector.workflow_node,
  }));

  const credentialRows = highlightedCredentials.map((credential) => ({
    id: credential.id,
    displayName: credential.display_name,
    authKind: credential.auth_kind,
    fieldCount: credential.fields.length,
    secretFields: credential.secret_fields,
    sourceFile: credential.source_file,
  }));

  return `export const automationSummary = ${JSON.stringify(
    {
      schema: "dx.automations.template_metadata",
      connectorCount: connectorsManifest.summary.connector_count,
      credentialCount: credentialsManifest.summary.credential_count,
      readyConnectorCount: connectorsManifest.summary.ready_count,
      credentialGatedConnectorCount: connectorsManifest.summary.needs_credential_count,
      triggerCount: connectorsManifest.summary.trigger_count,
      toolReadyCount: connectorsManifest.summary.tool_ready_count,
      receiptDir: readinessManifest.bridge.receipts_dir,
      commands: readinessManifest.bridge.commands,
      sourceProvenance: "n8n-nodes-base",
    },
    null,
    2,
  )} as const;

export const automationConnectorHighlights = ${JSON.stringify(connectorRows, null, 2)} as const;

export const automationCredentialHighlights = ${JSON.stringify(credentialRows, null, 2)} as const;

export const automationRoutes = [
  { href: "/automations", label: "Overview" },
  { href: "/automations/connectors", label: "Connectors" },
  { href: "/automations/credentials", label: "Credentials" },
  { href: "/automations/workflows", label: "Workflows" },
] as const;
`;
}

function main() {
  const generatedAt = new Date().toISOString();
  const manifest = readJson(manifestPath);
  const credentials = manifest.credentials.map(credentialRecord).sort((a, b) => a.id.localeCompare(b.id));
  const credentialMap = new Map(credentials.map((credential) => [credential.id, credential]));
  const connectors = manifest.nodes.map((entry) => connectorRecord(entry, credentialMap)).sort((a, b) =>
    a.display_name.localeCompare(b.display_name),
  );
  const readyCount = connectors.filter((connector) => connector.status === "ready").length;
  const needsCredentialCount = connectors.filter((connector) => connector.status === "needs_credential").length;
  const triggerCount = connectors.filter((connector) => connector.workflow_node.trigger).length;
  const toolReadyCount = connectors.filter((connector) => connector.workflow_node.usable_as_tool).length;

  const connectorsManifest = {
    schema: "dx.automations.connectors",
    generated_at: generatedAt,
    source: manifestSource(manifest),
    summary: {
      connector_count: connectors.length,
      ready_count: readyCount,
      needs_credential_count: needsCredentialCount,
      credential_backed_count: needsCredentialCount,
      trigger_count: triggerCount,
      tool_ready_count: toolReadyCount,
    },
    connectors,
  };

  const credentialsManifest = {
    schema: "dx.automations.credentials",
    generated_at: generatedAt,
    source: manifestSource(manifest),
    summary: {
      credential_count: credentials.length,
      oauth2_count: credentials.filter((credential) => credential.auth_kind === "oauth2").length,
      bearer_token_count: credentials.filter((credential) => credential.auth_kind === "bearer_token").length,
      api_key_count: credentials.filter((credential) => credential.auth_kind === "api_key").length,
      basic_auth_count: credentials.filter((credential) => credential.auth_kind === "basic_auth").length,
      secret_field_count: credentials.reduce((count, credential) => count + credential.secret_fields.length, 0),
    },
    credentials,
  };

  const readinessManifest = {
    schema: "dx.automations.workflow_readiness",
    generated_at: generatedAt,
    source: manifestSource(manifest),
    bridge: commandBridge(),
    workflow_node_readiness: {
      ready_count: connectors.length,
      credential_gated_count: needsCredentialCount,
      trigger_count: triggerCount,
      tool_ready_count: toolReadyCount,
      unsupported_count: 0,
      execution_policy: "metadata-first; no external connector execution without explicit approval",
    },
    routes: [
      "/automations",
      "/automations/connectors",
      "/automations/credentials",
      "/automations/workflows",
    ],
    receipt_contract: {
      schema: "dx.automations.zed.run_receipt",
      path: "G:/Dx/.dx/receipts/automations/run-latest.json",
      redacted: true,
      secrets_included: false,
    },
    connectors: connectors.map((connector) => ({
      id: connector.id,
      status: connector.status,
      source_file: connector.source_file,
      credential_type_names: connector.credential_type_names,
      workflow_node: connector.workflow_node,
    })),
  };

  writeJson(path.join(generatedDir, "dx-automations-connectors.json"), connectorsManifest);
  writeJson(path.join(generatedDir, "dx-automations-credentials.json"), credentialsManifest);
  writeJson(path.join(generatedDir, "dx-automations-readiness.json"), readinessManifest);

  fs.mkdirSync(templateAutomationDir, { recursive: true });
  fs.writeFileSync(
    path.join(templateAutomationDir, "automations-metadata.ts"),
    buildTemplateMetadata(connectorsManifest, credentialsManifest, readinessManifest),
  );

  console.log(
    JSON.stringify(
      {
        generated_at: generatedAt,
        connectors: connectors.length,
        credentials: credentials.length,
        output: posixPath(generatedDir),
      },
      null,
      2,
    ),
  );
}

main();
