import {
  filterDxN8nConnectors,
  normalizeDxN8nConnector,
  type DxN8nConnectorFilter,
  type DxN8nLaunchConnector,
} from "@/lib/automations/n8n/catalog";
import { buildDxN8nCredentialReadiness } from "@/lib/automations/n8n/readiness";
import {
  createDxN8nRunReceipt,
  type DxN8nRunMode,
} from "@/lib/automations/n8n/receipt";

export const runtime = "nodejs";
export const dynamic = "force-dynamic";

const launchConnectors = [
  {
    id: "manual-trigger",
    displayName: "Manual Trigger",
    status: "ready",
    authKinds: [],
    credentials: [],
    sourceFile: "nodes/ManualTrigger/ManualTrigger.node.ts",
    resources: [{ name: "Manual trigger", value: "manualTrigger" }],
    operations: [
      {
        name: "Start workflow",
        value: "start",
        action: "manual.trigger",
      },
    ],
    workflowNode: {
      ready: true,
      trigger: true,
      usableAsTool: false,
      runMode: "metadata-ready",
    },
  },
  {
    id: "slack-release-note",
    displayName: "Slack release note",
    status: "needs_credential",
    authKinds: ["token", "oauth2"],
    credentials: ["slackApi", "slackOAuth2Api"],
    sourceFile: "nodes/Slack/V2/SlackV2.node.ts",
    resources: [{ name: "Message", value: "message" }],
    operations: [
      {
        name: "Post message",
        value: "post",
        action: "slack.message.post",
      },
    ],
    workflowNode: {
      ready: true,
      trigger: false,
      usableAsTool: true,
      runMode: "credential-gated",
    },
  },
  {
    id: "notion-launch-log",
    displayName: "Notion launch log",
    status: "needs_credential",
    authKinds: ["api-key", "oauth2"],
    credentials: ["notionApi", "notionOAuth2Api"],
    sourceFile: "nodes/Notion/Notion.node.ts",
    resources: [{ name: "Page", value: "page" }],
    operations: [
      {
        name: "Append page",
        value: "append",
        action: "notion.page.append",
      },
    ],
    workflowNode: {
      ready: true,
      trigger: false,
      usableAsTool: true,
      runMode: "credential-gated",
    },
  },
] as const satisfies readonly DxN8nLaunchConnector[];

export async function GET(request: Request) {
  const url = new URL(request.url);
  const filter = readConnectorFilter(url.searchParams.get("filter"));
  const connectors = filterDxN8nConnectors(
    launchConnectors.map((connector) => normalizeDxN8nConnector(connector)),
    filter,
  );

  return Response.json({
    ok: true,
    packageId: "automations/n8n",
    status: "provider-boundary",
    runtimeExecution: false,
    filter,
    connectors: connectors.map((connector) => ({
      ...connector,
      readiness: buildDxN8nCredentialReadiness(connector, process.env),
    })),
    boundary:
      "This route exposes local connector readiness only. Provider credentials, webhook registration, and live n8n execution remain app-owned.",
  });
}

export async function POST(request: Request) {
  try {
    const body = await readDryRunBody(request);
    const connector = findConnector(readString(body.connectorId) ?? "slack-release-note");
    const mode = readRunMode(body.mode);
    const intent =
      readString(body.intent) ??
      "Draft a template handoff without contacting provider APIs.";
    const workflowId = readString(body.workflowId) ?? `${connector.id}-launch-dry-run`;
    const readiness = buildDxN8nCredentialReadiness(connector, process.env);
    const receipt = createDxN8nRunReceipt({
      connector,
      intent,
      mode,
      workflowId,
      env: process.env,
    });

    return Response.json(
      {
        ok: receipt.status !== "blocked-missing-config",
        packageId: "automations/n8n",
        status:
          receipt.status === "blocked-missing-config"
            ? "missing-config"
            : "local-dry-run",
        httpStatus: receipt.status === "blocked-missing-config" ? 501 : 202,
        providerBoundary: receipt.status === "blocked-missing-config",
        connector: normalizeDxN8nConnector(connector),
        readiness,
        receipt,
        runtimeExecution: false,
        secretValues: [],
        boundary:
          "The response is a local DX/Zed handoff. It does not run n8n, read secrets, register webhooks, or call provider APIs.",
      },
      { status: receipt.status === "blocked-missing-config" ? 501 : 202 },
    );
  } catch (error) {
    return Response.json(
      {
        ok: false,
        packageId: "automations/n8n",
        status: "bad-request",
        message:
          error instanceof Error
            ? error.message
            : "Automation dry-run request failed.",
        runtimeExecution: false,
        secretValues: [],
      },
      { status: 400 },
    );
  }
}

async function readDryRunBody(request: Request) {
  try {
    const value = await request.json();
    return isRecord(value) ? value : {};
  } catch {
    return {};
  }
}

function findConnector(connectorId: string) {
  const connector = launchConnectors.find((item) => item.id === connectorId);

  if (!connector) {
    throw new Error("Automation connector must be one of the template automation connectors.");
  }

  return connector;
}

function readConnectorFilter(value: string | null): DxN8nConnectorFilter {
  if (
    value === "ready" ||
    value === "missing-config" ||
    value === "tool-ready"
  ) {
    return value;
  }

  return "all";
}

function readRunMode(value: unknown): DxN8nRunMode {
  if (value === "draft" || value === "run") {
    return value;
  }

  return "dry-run";
}

function readString(value: unknown) {
  return typeof value === "string" && value.trim() ? value.trim() : undefined;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}
