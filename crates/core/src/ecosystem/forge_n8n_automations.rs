pub const N8N_AUTOMATIONS_VERSION: &str = "1.0.0-dx.3";

pub fn n8n_automations_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "js/lib/automations/n8n/metadata.ts",
            N8N_AUTOMATIONS_METADATA_TS,
        ),
        (
            "js/lib/automations/n8n/catalog.ts",
            N8N_AUTOMATIONS_CATALOG_TS,
        ),
        (
            "js/lib/automations/n8n/readiness.ts",
            N8N_AUTOMATIONS_READINESS_TS,
        ),
        (
            "js/lib/automations/n8n/receipt.ts",
            N8N_AUTOMATIONS_RECEIPT_TS,
        ),
        (
            "js/lib/automations/n8n/bridge.ts",
            N8N_AUTOMATIONS_BRIDGE_TS,
        ),
        (
            "js/lib/automations/n8n/README.md",
            N8N_AUTOMATIONS_README_MD,
        ),
    ]
}

const N8N_AUTOMATIONS_METADATA_TS: &str = r#"export type DxN8nConnectorStatus = "ready" | "needs_credential";

export type DxN8nConnectorMetadata = {
  id: string;
  displayName: string;
  status: DxN8nConnectorStatus;
  authKinds: readonly string[];
  credentials: readonly string[];
  sourceFile: string;
  resources: readonly { name: string; value: string }[];
  operations: readonly { name: string; value: string; action?: string }[];
  workflowNode: {
    ready: boolean;
    trigger: boolean;
    usableAsTool: boolean;
    runMode: "metadata-ready" | "credential-gated";
  };
};

export type DxN8nAutomationBridgeMetadata = {
  packageId: "automations/n8n";
  officialName: "Automation Connectors";
  officialPackageName: "Automation Connectors";
  aliases: readonly ["n8n", "n8n-nodes-base", "workflows/n8n"];
  upstreamPackage: "n8n-nodes-base";
  upstreamVersion: "2.22.0";
  sourceMirror: "G:/WWW/inspirations/n8n";
  nodeSourceMirror: "G:\\WWW\\inspirations\\n8n\\packages\\nodes-base";
  sourceManifest: "integrations/n8n-nodes-base/dx-node-source-manifest.json";
  generatedManifestRoot: "integrations/n8n-nodes-base/generated";
  provenance: "n8n-nodes-base";
  inspectedSourceFiles: readonly string[];
  surfaces: readonly string[];
  selectedSurfaces: readonly string[];
  dxCheckVisibility: {
    schema: "dx.forge.package.dx_check_visibility";
    currentStatus: "present";
    statuses: readonly ["present", "stale", "missing-receipt", "blocked", "unsupported-surface"];
    receiptPath: "G:/Dx/.dx/receipts/automations/launch-release-notification.json";
    monitoredSurfaces: readonly string[];
  };
  honestyLabel: "ADAPTER-BOUNDARY";
  dashboardUsage: {
    route: "/";
    component: "AutomationWorkflowPanel";
    sourceMirror: "G:\\WWW\\inspirations\\n8n\\packages\\nodes-base";
    sourceFile: "examples/dashboard/src/components/AutomationWorkflowPanel.tsx";
    modelFile: "examples/dashboard/src/lib/n8nAutomationBridge.ts";
    dxIcon: "pack:workflow";
  };
  launchDashboardUsage: {
    route: "/launch";
    component: "launch-automation-dashboard-workflow";
    connectorWorkflowComponent: "launch-automation-connector-workflow";
    catalogSummaryComponent: "launch-automation-catalog-summary";
    missionSummaryComponent: "launch-automation-mission-summary";
    sourceFile: "examples/template/automations-status.tsx";
    missionSummarySourceFile: "examples/template/automation-mission-summary.tsx";
    runReceiptPath: "G:/Dx/.dx/receipts/automations/run-latest.json";
    zedActionMarker: "data-dx-automation-safe-action=\"prepare-zed-run-handoff\"";
    dashboardStateMarker: "data-dx-automation-dashboard-state";
    connectorWorkflowMarker: "data-dx-component=\"launch-automation-connector-workflow\"";
    catalogSummaryMarker: "data-dx-component=\"launch-automation-catalog-summary\"";
    connectorReadinessMarker: "data-dx-automation-workflow=\"connector-readiness\"";
    missionSummaryMarker: "data-dx-component=\"launch-automation-mission-summary\"";
    credentialSchemaMarker: "data-dx-automation-credential-schema";
    workflowNodeMarker: "data-dx-automation-workflow-node-readiness";
    requiredEnvMarker: "data-dx-automation-required-env";
    receiptIntentMarker: "data-dx-automation-receipt-intent";
    runReceiptIntentMarker: "data-dx-automation-run-receipt-intent";
  };
  commands: {
    connectors: "dx automations connectors --json";
    credentials: "dx automations credentials --json";
    run: "dx automations run --json";
  };
  exportedFiles: readonly string[];
  publicApis: readonly string[];
  requiredEnv: readonly [
    "SLACK_BOT_TOKEN",
    "SLACK_SIGNING_SECRET",
    "NOTION_API_KEY",
    "DX_AUTOMATIONS_OPERATOR_APPROVAL",
  ];
  requiredEnvByCredentialType: {
    slackApi: readonly ["SLACK_BOT_TOKEN", "SLACK_SIGNING_SECRET"];
    slackOAuth2Api: readonly ["SLACK_BOT_TOKEN", "SLACK_SIGNING_SECRET"];
    notionApi: readonly ["NOTION_API_KEY"];
    notionOAuth2Api: readonly ["NOTION_API_KEY"];
  };
  appOwnedBoundaries: readonly string[];
  receiptPaths: {
    run: ".dx/forge/receipts/automations/run-latest.json";
    readiness: ".dx/forge/receipts/automations/readiness.json";
    launchSeed: "G:/Dx/.dx/receipts/automations/launch-release-notification.json";
    zedRun: "G:/Dx/.dx/receipts/automations/run-latest.json";
  };
  credentialPolicy: "app-owned-redacted-boundary";
  noNodeModulesRequired: true;
};

export const dxN8nAutomationBridgeMetadata = {
  packageId: "automations/n8n",
  officialName: "Automation Connectors",
  officialPackageName: "Automation Connectors",
  aliases: ["n8n", "n8n-nodes-base", "workflows/n8n"],
  upstreamPackage: "n8n-nodes-base",
  upstreamVersion: "2.22.0",
  sourceMirror: "G:/WWW/inspirations/n8n",
  nodeSourceMirror: "G:\\WWW\\inspirations\\n8n\\packages\\nodes-base",
  sourceManifest: "integrations/n8n-nodes-base/dx-node-source-manifest.json",
  generatedManifestRoot: "integrations/n8n-nodes-base/generated",
  provenance: "n8n-nodes-base",
  inspectedSourceFiles: [
    "packages/nodes-base/package.json",
    "packages/nodes-base/nodes/ManualTrigger/ManualTrigger.node.ts",
    "packages/nodes-base/nodes/Slack/Slack.node.ts",
    "packages/nodes-base/nodes/Slack/V2/SlackV2.node.ts",
    "packages/nodes-base/nodes/Webhook/Webhook.node.ts",
    "packages/nodes-base/nodes/Notion/Notion.node.ts",
    "packages/nodes-base/credentials/SlackApi.credentials.ts",
    "packages/nodes-base/credentials/SlackOAuth2Api.credentials.ts",
    "packages/nodes-base/credentials/NotionApi.credentials.ts",
  ],
  surfaces: [
    "connector catalog",
    "credential readiness",
    "redacted run receipt",
    "launch dashboard workflow",
    "starter dashboard workflow",
    "Rust/Zed run handoff",
  ],
  selectedSurfaces: [
    "connector-catalog",
    "credential-readiness",
    "redacted-run-receipt",
    "launch-dashboard-workflow",
    "starter-dashboard-workflow",
    "zed-run-handoff",
  ],
  dxCheckVisibility: {
    schema: "dx.forge.package.dx_check_visibility",
    currentStatus: "present",
    statuses: ["present", "stale", "missing-receipt", "blocked", "unsupported-surface"],
    receiptPath: "G:/Dx/.dx/receipts/automations/launch-release-notification.json",
    monitoredSurfaces: [
      "launch-automation-dashboard-workflow",
      "launch-automation-connector-workflow",
      "launch-automation-mission-summary",
      "dashboard-automation-workflow",
      "zed-run-receipt",
    ],
  },
  honestyLabel: "ADAPTER-BOUNDARY",
  dashboardUsage: {
    route: "/",
    component: "AutomationWorkflowPanel",
    sourceMirror: "G:\\WWW\\inspirations\\n8n\\packages\\nodes-base",
    sourceFile: "examples/dashboard/src/components/AutomationWorkflowPanel.tsx",
    modelFile: "examples/dashboard/src/lib/n8nAutomationBridge.ts",
    dxIcon: "pack:workflow",
  },
  launchDashboardUsage: {
    route: "/launch",
    component: "launch-automation-dashboard-workflow",
    connectorWorkflowComponent: "launch-automation-connector-workflow",
    catalogSummaryComponent: "launch-automation-catalog-summary",
    missionSummaryComponent: "launch-automation-mission-summary",
    sourceFile: "examples/template/automations-status.tsx",
    missionSummarySourceFile: "examples/template/automation-mission-summary.tsx",
    runReceiptPath: "G:/Dx/.dx/receipts/automations/run-latest.json",
    zedActionMarker: 'data-dx-automation-safe-action="prepare-zed-run-handoff"',
    dashboardStateMarker: "data-dx-automation-dashboard-state",
    connectorWorkflowMarker: 'data-dx-component="launch-automation-connector-workflow"',
    catalogSummaryMarker: 'data-dx-component="launch-automation-catalog-summary"',
    connectorReadinessMarker: 'data-dx-automation-workflow="connector-readiness"',
    missionSummaryMarker: 'data-dx-component="launch-automation-mission-summary"',
    credentialSchemaMarker: "data-dx-automation-credential-schema",
    workflowNodeMarker: "data-dx-automation-workflow-node-readiness",
    requiredEnvMarker: "data-dx-automation-required-env",
    receiptIntentMarker: "data-dx-automation-receipt-intent",
    runReceiptIntentMarker: "data-dx-automation-run-receipt-intent",
  },
  commands: {
    connectors: "dx automations connectors --json",
    credentials: "dx automations credentials --json",
    run: "dx automations run --json",
  },
  exportedFiles: [
    "js/lib/automations/n8n/metadata.ts",
    "js/lib/automations/n8n/catalog.ts",
    "js/lib/automations/n8n/readiness.ts",
    "js/lib/automations/n8n/receipt.ts",
    "js/lib/automations/n8n/bridge.ts",
    "examples/template/template-shell.tsx",
    "examples/template/automations-status.tsx",
    "examples/template/automations/automations-metadata.ts",
    "examples/template/automation-mission-summary.tsx",
    "tools/launch/runtime-template/pages/index.html",
    "tools/launch/runtime-template/assets/launch-runtime.ts",
    "examples/template/package-catalog.ts",
    "examples/dashboard/src/lib/n8nAutomationBridge.ts",
    "examples/dashboard/src/components/AutomationWorkflowPanel.tsx",
  ],
  publicApis: [
    "dxN8nAutomationBridgeMetadata",
    "automationSummary",
    "connectorMetadata",
    "normalizeDxN8nConnector",
    "filterDxN8nConnectors",
    "summarizeDxN8nConnectorCatalog",
    "buildDxN8nCredentialReadiness",
    "requiredEnvForDxN8nConnector",
    "buildDxN8nWorkflowDraft",
    "DX_N8N_RUN_RECEIPT_SCHEMA",
    "createDxN8nRunReceipt",
    "n8nAutomationForgeMetadata",
    "createN8nDashboardWorkflow",
    "selectN8nConnector",
    "buildN8nWorkflowReadiness",
    "createRedactedN8nReceipt",
    "formatN8nCredentialBoundary",
  ],
  requiredEnv: [
    "SLACK_BOT_TOKEN",
    "SLACK_SIGNING_SECRET",
    "NOTION_API_KEY",
    "DX_AUTOMATIONS_OPERATOR_APPROVAL",
  ],
  requiredEnvByCredentialType: {
    slackApi: ["SLACK_BOT_TOKEN", "SLACK_SIGNING_SECRET"],
    slackOAuth2Api: ["SLACK_BOT_TOKEN", "SLACK_SIGNING_SECRET"],
    notionApi: ["NOTION_API_KEY"],
    notionOAuth2Api: ["NOTION_API_KEY"],
  },
  appOwnedBoundaries: [
    "Connector selection",
    "Credential approval",
    "Workflow execution",
    "Receipt retention",
    "Secret handoff",
  ],
  receiptPaths: {
    run: ".dx/forge/receipts/automations/run-latest.json",
    readiness: ".dx/forge/receipts/automations/readiness.json",
    launchSeed: "G:/Dx/.dx/receipts/automations/launch-release-notification.json",
    zedRun: "G:/Dx/.dx/receipts/automations/run-latest.json",
  },
  credentialPolicy: "app-owned-redacted-boundary",
  noNodeModulesRequired: true,
} as const satisfies DxN8nAutomationBridgeMetadata;

export const automationSummary = {
  schema: "dx.automations.template_metadata",
  officialName: dxN8nAutomationBridgeMetadata.officialName,
  upstreamPackage: dxN8nAutomationBridgeMetadata.upstreamPackage,
  upstreamVersion: dxN8nAutomationBridgeMetadata.upstreamVersion,
  connectorCount: 536,
  credentialCount: 396,
  readyConnectorCount: 112,
  credentialGatedConnectorCount: 424,
  triggerCount: 110,
  toolReadyCount: 360,
  receiptDir: "G:/Dx/.dx/receipts/automations",
  receiptSeedPath: dxN8nAutomationBridgeMetadata.receiptPaths.launchSeed,
  runReceiptPath: dxN8nAutomationBridgeMetadata.receiptPaths.zedRun,
  commands: dxN8nAutomationBridgeMetadata.commands,
  sourceProvenance: dxN8nAutomationBridgeMetadata.provenance,
} as const;

export const connectorMetadata = {
  officialName: dxN8nAutomationBridgeMetadata.officialName,
  sourceManifest: dxN8nAutomationBridgeMetadata.sourceManifest,
  generatedManifestRoot: dxN8nAutomationBridgeMetadata.generatedManifestRoot,
  launchMetadataSourceFile: "examples/template/automations/automations-metadata.ts",
  connectorWorkflowMarker:
    dxN8nAutomationBridgeMetadata.launchDashboardUsage.connectorWorkflowMarker,
  catalogSummaryMarker:
    dxN8nAutomationBridgeMetadata.launchDashboardUsage.catalogSummaryMarker,
  workflowNodeMarker:
    dxN8nAutomationBridgeMetadata.launchDashboardUsage.workflowNodeMarker,
  credentialSchemaMarker:
    dxN8nAutomationBridgeMetadata.launchDashboardUsage.credentialSchemaMarker,
  requiredEnvMarker:
    dxN8nAutomationBridgeMetadata.launchDashboardUsage.requiredEnvMarker,
  receiptIntentMarker:
    dxN8nAutomationBridgeMetadata.launchDashboardUsage.receiptIntentMarker,
  runReceiptIntentMarker:
    dxN8nAutomationBridgeMetadata.launchDashboardUsage.runReceiptIntentMarker,
} as const;
"#;

const N8N_AUTOMATIONS_CATALOG_TS: &str = r#"import {
  dxN8nAutomationBridgeMetadata,
  type DxN8nConnectorMetadata,
} from "./metadata";

export type DxN8nUpstreamConnector = {
  id: string;
  display_name: string;
  status: "ready" | "needs_credential";
  source_file: string;
  credential_type_names?: readonly string[];
  auth_kinds?: readonly string[];
  resources?: readonly { name?: string; displayName?: string; value: string }[];
  operations?: readonly {
    name?: string;
    displayName?: string;
    value: string;
    action?: string;
  }[];
  workflow_node?: {
    ready?: boolean;
    trigger?: boolean;
    usable_as_tool?: boolean;
    run_mode?: "metadata-ready" | "credential-gated";
  };
};

export type DxN8nLaunchConnector = Omit<
  DxN8nUpstreamConnector,
  "display_name" | "source_file" | "credential_type_names" | "auth_kinds" | "workflow_node"
> & {
  displayName: string;
  sourceFile: string;
  credentials: readonly string[];
  authKinds: readonly string[];
  workflowNode?: {
    ready?: boolean;
    trigger?: boolean;
    usableAsTool?: boolean;
    usable_as_tool?: boolean;
    runMode?: "metadata-ready" | "credential-gated";
    run_mode?: "metadata-ready" | "credential-gated";
  };
};

export type DxN8nConnectorCatalog = {
  schema: "dx.automations.connectors";
  summary: {
    connector_count: number;
    ready_count?: number;
    ready_connector_count?: number;
    needs_credential_count?: number;
    credential_gated_connector_count?: number;
    trigger_count: number;
    tool_ready_count: number;
  };
  connectors: readonly DxN8nUpstreamConnector[];
};

export type DxN8nConnectorFilter =
  | "all"
  | "ready"
  | "missing-config"
  | "tool-ready";

export function normalizeDxN8nConnector(
  connector:
    | DxN8nUpstreamConnector
    | DxN8nLaunchConnector
    | DxN8nConnectorMetadata,
): DxN8nConnectorMetadata {
  const workflowNode =
    "workflowNode" in connector ? connector.workflowNode : connector.workflow_node;
  const displayName =
    "displayName" in connector ? connector.displayName : connector.display_name;
  const sourceFile =
    "sourceFile" in connector ? connector.sourceFile : connector.source_file;
  const credentials =
    "credentials" in connector
      ? connector.credentials
      : connector.credential_type_names ?? [];
  const authKinds =
    "authKinds" in connector ? connector.authKinds : connector.auth_kinds ?? [];

  return {
    id: connector.id,
    displayName,
    status: connector.status,
    authKinds,
    credentials,
    sourceFile,
    resources: (connector.resources ?? []).map((resource) => ({
      name: resource.name ?? resource.displayName ?? resource.value,
      value: resource.value,
    })),
    operations: (connector.operations ?? []).map((operation) => ({
      name: operation.name ?? operation.displayName ?? operation.value,
      value: operation.value,
      action: operation.action,
    })),
    workflowNode: {
      ready: workflowNode?.ready ?? connector.status === "ready",
      trigger: workflowNode?.trigger ?? false,
      usableAsTool:
        workflowNode?.usableAsTool ?? workflowNode?.usable_as_tool ?? false,
      runMode:
        workflowNode?.runMode ??
        workflowNode?.run_mode ??
        (connector.status === "needs_credential"
          ? "credential-gated"
          : "metadata-ready"),
    },
  };
}

export function connectorNeedsCredential(
  connector:
    | DxN8nUpstreamConnector
    | DxN8nLaunchConnector
    | DxN8nConnectorMetadata,
) {
  const normalized = normalizeDxN8nConnector(connector);

  return (
    normalized.status === "needs_credential" ||
    normalized.credentials.length > 0 ||
    normalized.workflowNode.runMode === "credential-gated"
  );
}

export function workflowStatusForConnector(
  connector:
    | DxN8nUpstreamConnector
    | DxN8nLaunchConnector
    | DxN8nConnectorMetadata,
) {
  return connectorNeedsCredential(connector)
    ? "missing-config"
    : "metadata-ready";
}

export function filterDxN8nConnectors(
  connectors: readonly (
    | DxN8nUpstreamConnector
    | DxN8nLaunchConnector
    | DxN8nConnectorMetadata
  )[],
  filter: DxN8nConnectorFilter,
) {
  return connectors
    .map(normalizeDxN8nConnector)
    .filter((connector) => {
      if (filter === "ready") {
        return !connectorNeedsCredential(connector);
      }

      if (filter === "missing-config") {
        return connectorNeedsCredential(connector);
      }

      if (filter === "tool-ready") {
        return connector.workflowNode.usableAsTool;
      }

      return true;
    });
}

export function summarizeDxN8nConnectorCatalog(catalog: DxN8nConnectorCatalog) {
  const readyConnectorCount =
    catalog.summary.ready_connector_count ?? catalog.summary.ready_count ?? 0;
  const credentialGatedConnectorCount =
    catalog.summary.credential_gated_connector_count ??
    catalog.summary.needs_credential_count ??
    0;

  return {
    packageId: dxN8nAutomationBridgeMetadata.packageId,
    connectorCount: catalog.summary.connector_count,
    readyConnectorCount,
    credentialGatedConnectorCount,
    triggerCount: catalog.summary.trigger_count,
    toolReadyCount: catalog.summary.tool_ready_count,
    noNodeModulesRequired:
      dxN8nAutomationBridgeMetadata.noNodeModulesRequired,
  };
}
"#;

const N8N_AUTOMATIONS_READINESS_TS: &str = r#"import {
  connectorNeedsCredential,
  normalizeDxN8nConnector,
  workflowStatusForConnector,
} from "./catalog";
import { dxN8nAutomationBridgeMetadata, type DxN8nConnectorMetadata } from "./metadata";

export type DxN8nCredentialReadiness = {
  packageId: "automations/n8n";
  connectorId: string;
  connectorName: string;
  status: "metadata-ready" | "missing-config";
  missingCredentials: readonly string[];
  authKinds: readonly string[];
  safeUserAction: "prepare-dry-run-receipt" | "configure-credentials";
  credentialPolicy: "app-owned-redacted-boundary";
  requiredEnv: readonly string[];
  noNodeModulesRequired: true;
};

export function requiredEnvForDxN8nConnector(
  connector: DxN8nConnectorMetadata,
): readonly string[] {
  const normalized = normalizeDxN8nConnector(connector);
  const env = new Set<string>();

  for (const credentialType of normalized.credentials) {
    for (const envName of dxN8nAutomationBridgeMetadata.requiredEnvByCredentialType[
      credentialType as keyof typeof dxN8nAutomationBridgeMetadata.requiredEnvByCredentialType
    ] ?? []) {
      env.add(envName);
    }
  }

  env.add("DX_AUTOMATIONS_OPERATOR_APPROVAL");

  return [...env];
}

export function buildDxN8nCredentialReadiness(
  connector: DxN8nConnectorMetadata,
): DxN8nCredentialReadiness {
  const normalized = normalizeDxN8nConnector(connector);
  const needsCredential = connectorNeedsCredential(normalized);

  return {
    packageId: dxN8nAutomationBridgeMetadata.packageId,
    connectorId: normalized.id,
    connectorName: normalized.displayName,
    status: workflowStatusForConnector(normalized),
    missingCredentials: needsCredential ? normalized.credentials : [],
    authKinds: normalized.authKinds,
    safeUserAction: needsCredential
      ? "configure-credentials"
      : "prepare-dry-run-receipt",
    credentialPolicy: dxN8nAutomationBridgeMetadata.credentialPolicy,
    requiredEnv: requiredEnvForDxN8nConnector(normalized),
    noNodeModulesRequired: dxN8nAutomationBridgeMetadata.noNodeModulesRequired,
  };
}
"#;

const N8N_AUTOMATIONS_RECEIPT_TS: &str = r#"import { workflowStatusForConnector } from "./catalog";
import { dxN8nAutomationBridgeMetadata, type DxN8nConnectorMetadata } from "./metadata";
import { requiredEnvForDxN8nConnector } from "./readiness";

export const DX_N8N_RUN_RECEIPT_SCHEMA =
  "dx.automations.zed.run_receipt" as const;

export type DxN8nRunReceiptInput = {
  workflowId: string;
  connector: DxN8nConnectorMetadata;
  mode: "draft" | "dry-run" | "run";
  operation?: string;
  intent?: string;
};

export type DxN8nRunReceipt = {
  schema: typeof DX_N8N_RUN_RECEIPT_SCHEMA;
  packageId: "automations/n8n";
  workflowId: string;
  workflowIntent: string;
  connectorId: string;
  connectorName: string;
  mode: "draft" | "dry-run" | "run";
  operation?: string;
  status: "metadata-ready" | "missing-config" | "credential-gated";
  credentialPolicy: "app-owned-redacted-boundary";
  requiredEnv: readonly string[];
  commands: typeof dxN8nAutomationBridgeMetadata.commands;
  receiptPath: ".dx/forge/receipts/automations/run-latest.json";
  secretsImported: false;
  noNodeModulesRequired: true;
};

export function buildDxN8nWorkflowDraft(input: DxN8nRunReceiptInput) {
  return {
    workflowId: input.workflowId,
    workflowIntent: input.intent?.trim() ?? "",
    connectorId: input.connector.id,
    connectorName: input.connector.displayName,
    operation: input.operation ?? input.connector.operations[0]?.value,
    requiredEnv: requiredEnvForDxN8nConnector(input.connector),
    credentialPolicy: dxN8nAutomationBridgeMetadata.credentialPolicy,
    noNodeModulesRequired: dxN8nAutomationBridgeMetadata.noNodeModulesRequired,
  };
}

export function createDxN8nRunReceipt(
  input: DxN8nRunReceiptInput,
): DxN8nRunReceipt {
  const status =
    input.mode === "run"
      ? "credential-gated"
      : workflowStatusForConnector(input.connector);
  const workflowIntent = input.intent?.trim() ?? "";
  const requiredEnv = requiredEnvForDxN8nConnector(input.connector);

  return {
    schema: DX_N8N_RUN_RECEIPT_SCHEMA,
    packageId: dxN8nAutomationBridgeMetadata.packageId,
    workflowId: input.workflowId,
    workflowIntent,
    connectorId: input.connector.id,
    connectorName: input.connector.displayName,
    mode: input.mode,
    operation: input.operation ?? input.connector.operations[0]?.value,
    status,
    credentialPolicy: dxN8nAutomationBridgeMetadata.credentialPolicy,
    requiredEnv,
    commands: dxN8nAutomationBridgeMetadata.commands,
    receiptPath: dxN8nAutomationBridgeMetadata.receiptPaths.run,
    secretsImported: false,
    noNodeModulesRequired: dxN8nAutomationBridgeMetadata.noNodeModulesRequired,
  };
}
"#;

const N8N_AUTOMATIONS_BRIDGE_TS: &str = r#"export {
  connectorNeedsCredential,
  filterDxN8nConnectors,
  normalizeDxN8nConnector,
  summarizeDxN8nConnectorCatalog,
  workflowStatusForConnector,
  type DxN8nConnectorCatalog,
  type DxN8nConnectorFilter,
  type DxN8nLaunchConnector,
  type DxN8nUpstreamConnector,
} from "./catalog";
export {
  buildDxN8nCredentialReadiness,
  requiredEnvForDxN8nConnector,
  type DxN8nCredentialReadiness,
} from "./readiness";
export {
  DX_N8N_RUN_RECEIPT_SCHEMA,
  buildDxN8nWorkflowDraft,
  createDxN8nRunReceipt,
  type DxN8nRunReceipt,
  type DxN8nRunReceiptInput,
} from "./receipt";
export {
  automationSummary,
  connectorMetadata,
  dxN8nAutomationBridgeMetadata,
  type DxN8nAutomationBridgeMetadata,
  type DxN8nConnectorMetadata,
  type DxN8nConnectorStatus,
} from "./metadata";
"#;

const N8N_AUTOMATIONS_README_MD: &str = r#"# DX Forge Automation Connectors

Source-owned Automation Connectors APIs for DX launch templates, based on upstream `n8n-nodes-base` `2.22.0`.

What is real:

- `metadata.ts` records the official DX package name `Automation Connectors`, package id `automations/n8n`, upstream package/version provenance, source mirror, inspected upstream files, selected surfaces, dx-check visibility states, receipt paths, and app-owned boundaries.
- `catalog.ts` maps the generated `n8n-nodes-base` connector manifest shape, including `display_name`, `credential_type_names`, resources, operations, and `workflow_node.usable_as_tool`.
- `readiness.ts` turns connector metadata into an app-owned credential readiness boundary without importing secrets and exposes `requiredEnvForDxN8nConnector` for connector-specific env gates.
- `receipt.ts` exposes `createDxN8nRunReceipt` for DX/Zed run receipts in draft, dry-run, and run handoff workflows, including the normalized operator workflow intent and required env boundary.
- `examples/template/automations-status.tsx` powers the `/launch` release notification workflow with connector selection, visible credential schema/auth-kind/workflow-node readiness, intent editing, missing-config state, redacted draft receipt handoff, and a visible Rust/Zed run handoff to `G:\Dx\.dx\receipts\automations\run-latest.json`.
- `examples/template/automation-mission-summary.tsx` consumes `LaunchAutomationDashboardState` from that workflow so the launch mission dashboard updates `data-dx-component="launch-automation-mission-summary"` and `data-dx-automation-dashboard-state` from the real connector interaction while the shell stays focused on layout composition.
- `examples/dashboard/src/lib/n8nAutomationBridge.ts` and `examples/dashboard/src/components/AutomationWorkflowPanel.tsx` consume the same n8n-shaped connector and credential concepts in the starter dashboard.

Dashboard usage:

- The starter dashboard workflow exposes `data-dx-package="automations/n8n"` and `data-dx-component="dashboard-automation-workflow"`.
- Operators can select Manual Trigger, Slack, or Notion metadata, inspect the selected connector's credential schema, auth kinds, credential type names, required env, trigger/tool readiness, draft a workflow intent, and prepare a redacted local receipt.
- The generated `/launch` dashboard also exposes `data-dx-component="launch-automation-connector-workflow"`, `data-dx-component="launch-automation-catalog-summary"`, `data-dx-component="launch-automation-mission-summary"`, `data-dx-automation-workflow="connector-readiness"`, `data-dx-automation-dashboard-state`, `data-dx-automation-required-env`, `data-dx-automation-receipt-intent`, `data-dx-automation-run-receipt-intent`, and `data-dx-automation-safe-action="prepare-zed-run-handoff"` for the `dx automations run --json` receipt path.
- Slack and Notion show honest missing-config state until the app owns credential storage and provider environment wiring.

What is app-owned:

- Connector selection, credential approval, live execution, receipt retention, and secret handoff.
- Account-specific credential values are never imported by this package.

What is intentionally deferred:

- Runtime n8n canvas parity.
- Live workflow execution.
- Provider account onboarding.

This package is intentionally node_modules-free for DX/Forge materialization.
"#;
