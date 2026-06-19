import {
  normalizeDxN8nConnector,
  type DxN8nLaunchConnector,
  type DxN8nNormalizedConnector,
} from "./catalog";
import { buildDxN8nCredentialReadiness } from "./readiness";

export type DxN8nRunMode = "draft" | "dry-run" | "run";

export type DxN8nRunReceiptStatus =
  | "draft-created"
  | "local-dry-run"
  | "zed-handoff-created"
  | "blocked-missing-config";

export type DxN8nRunReceipt = {
  readonly schema: "dx.automation.n8n.run_receipt";
  readonly packageId: "automations/n8n";
  readonly upstreamPackage: "n8n-nodes-base";
  readonly connectorId: string;
  readonly connectorName: string;
  readonly workflowId: string;
  readonly workflowIntent: string;
  readonly mode: DxN8nRunMode;
  readonly status: DxN8nRunReceiptStatus;
  readonly requiredEnv: readonly string[];
  readonly missingCredentials: readonly string[];
  readonly credentialsConfigured: boolean;
  readonly runtimeExecution: false;
  readonly secretValues: readonly [];
  readonly commands: {
    readonly dryRun: string;
    readonly run: string;
  };
  readonly boundary: string;
  readonly generatedAt: string;
};

export function createDxN8nRunReceipt({
  connector,
  intent,
  mode,
  workflowId,
  env = {},
}: {
  readonly connector: DxN8nLaunchConnector | DxN8nNormalizedConnector;
  readonly intent: string;
  readonly mode: DxN8nRunMode;
  readonly workflowId: string;
  readonly env?: Record<string, string | undefined>;
}): DxN8nRunReceipt {
  const normalized = normalizeDxN8nConnector(connector);
  const readiness = buildDxN8nCredentialReadiness(normalized, env);
  const missingConfig = readiness.missingCredentials.length > 0 && mode === "run";
  const quotedConnector = quoteCliValue(normalized.id);
  const quotedWorkflow = quoteCliValue(workflowId);

  return {
    schema: "dx.automation.n8n.run_receipt",
    packageId: "automations/n8n",
    upstreamPackage: "n8n-nodes-base",
    connectorId: normalized.id,
    connectorName: normalized.displayName,
    workflowId,
    workflowIntent: intent.trim(),
    mode,
    status: missingConfig
      ? "blocked-missing-config"
      : mode === "run"
        ? "zed-handoff-created"
        : mode === "dry-run"
          ? "local-dry-run"
          : "draft-created",
    requiredEnv: readiness.requiredEnv,
    missingCredentials: readiness.missingCredentials,
    credentialsConfigured: readiness.credentialsConfigured,
    runtimeExecution: false,
    secretValues: [],
    commands: {
      dryRun: `dx automations run --json --dry-run --connector ${quotedConnector} --workflow ${quotedWorkflow}`,
      run: `dx automations run --json --connector ${quotedConnector} --workflow ${quotedWorkflow} --mode ${mode}`,
    },
    boundary:
      "This receipt is a local DX/Zed handoff. n8n credentials, webhook registration, and live workflow execution remain app-owned.",
    generatedAt: new Date().toISOString(),
  };
}

function quoteCliValue(value: string) {
  return JSON.stringify(value);
}
