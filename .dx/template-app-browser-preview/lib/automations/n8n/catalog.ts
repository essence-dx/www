import {
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
