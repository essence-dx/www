import fs from "node:fs";
import path from "node:path";

import {
  createStudioBootFromLocalGeneratedSource,
  type GeneratedStudioBoot,
} from "./generated-catalog-source";
import { createReadinessResponse } from "./readiness-response";
import { createRuntimeExecutionProofSchedulerAction } from "./runtime-execution-proof-scheduler";

const REDACTION_DETAIL =
  "The n8n Studio bridge exports source-owned counts, labels, readiness states, and recovery hints only. Provider credentials, secret references, workflow execution payloads, and node secret values are not serialized.";
const automationReceiptPath = ".dx/receipts/agents/automate-list-latest.json";
const readinessReceiptPath = ".dx/receipts/n8n-studio/readiness-latest.json";
const runtimeExecutionProofReceiptPath =
  ".dx/receipts/n8n-studio/runtime/execution-proof-latest.sr";

function editorSessionNextAction(pendingRequestCount: number) {
  if (pendingRequestCount > 0) {
    return "Resolve governed editor-session requests, then configure runtime handoff or webhook trigger credentials, write runtime proof receipts, and retry delayed history imports before enabling live automation.";
  }

  return "Configure runtime handoff or webhook trigger credentials, write runtime proof receipts, and retry delayed history imports before enabling live automation.";
}

export function createZedAutomationBridgePacket(
  boot: GeneratedStudioBoot = createStudioBootFromLocalGeneratedSource(),
) {
  const editorSession = boot.state.editorSession;
  const pendingRequestCount = editorSession.requestPlans.filter(
    (request) => request.status === "blocked",
  ).length;

  return {
    schema_version: "dx.agents.zed.automation_list.v1",
    command: "dx-www n8n-studio zed-automation --json",
    generated_at: new Date().toISOString(),
    status: "warning",
    automation_count: 1,
    active_task_count: editorSession.requestPlans.length,
    automations: [
      {
        id: "n8n-studio-editor-session",
        name: "n8n Studio editor-session",
        prompt:
          "Use DX-native n8n metadata, workflow JSON, connections, credentials, and editor-session requests through governed adapters.",
        source: "dx-www-n8n-studio",
        status: "blocked",
        enabled: false,
        runtime_available: false,
        unavailable_reason:
          "Live n8n automation is not enabled by default. Governed runtime handoff can publish or activate workflows, and webhook trigger requests require configured trigger credentials plus imported execution receipts.",
        schedule: {
          kind: "manual",
          summary: "Manual from DX/Zed automation surface",
          rrule: "",
          timezone: "",
        },
        destination: {
          kind: "zed",
          label: "Zed Plugins and DX automation screen",
          target: "dx-zed-automation-panel",
        },
        last_run: "never",
        next_run: "pending live trigger credential and receipt",
        receipts: [
          {
            kind: "readiness",
            schema_version: "dx.n8n-studio.readiness",
            status: boot.state.execution.status,
            path: readinessReceiptPath,
          },
          {
            kind: "runtime-execution-proof",
            schema_version: "dx.n8n-studio.runtime-execution-proof.receipt",
            status: "pending",
            path: runtimeExecutionProofReceiptPath,
          },
        ],
        history: [],
        actions: [createRuntimeExecutionProofSchedulerAction()],
        next_action: editorSessionNextAction(pendingRequestCount),
      },
    ],
    last_error: null,
    next_action:
      "Use runtime handoff and webhook trigger receipts for provider requests, write runtime proof receipts, and retry delayed execution-history imports before enabling live automation.",
    receipt_path: automationReceiptPath,
    redaction: {
      exports_secret_values: false,
      exports_account_targets: false,
      exports_automation_bodies: false,
      exports_tool_payloads: false,
      exports_task_payloads: false,
      exports_transcripts: false,
      exports_provider_credentials: false,
      detail: REDACTION_DETAIL,
    },
  };
}

export function createZedAutomationBridgePacketFromLocalGeneratedSource(
  startDirectory = process.cwd(),
) {
  return createZedAutomationBridgePacket(
    createStudioBootFromLocalGeneratedSource(startDirectory),
  );
}

export type ZedAutomationBridgeReceiptExport = {
  schema: "dx.n8n-studio.zed-receipt-export";
  receiptRoot: string;
  automationReceiptPath: string;
  readinessReceiptPath: string;
  automationCount: number;
  readinessStatus: string;
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  redaction: {
    exports_secret_values: false;
    exports_provider_credentials: false;
  };
  issue: string;
};

export type ZedAutomationBridgeReceiptOptions = {
  boot?: GeneratedStudioBoot;
  receiptRoot?: string;
};

function writeJsonReceipt(filePath: string, value: unknown) {
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, `${JSON.stringify(value, null, 2)}\n`, "utf8");
}

function defaultReceiptRoot() {
  return path.join(process.cwd(), ".dx", "receipts");
}

export function writeZedAutomationBridgeReceipts(
  options: ZedAutomationBridgeReceiptOptions = {},
): ZedAutomationBridgeReceiptExport {
  const boot = options.boot ?? createStudioBootFromLocalGeneratedSource();
  const receiptRoot = path.resolve(options.receiptRoot ?? defaultReceiptRoot());
  const automationPacket = createZedAutomationBridgePacket(boot);
  const readiness = createReadinessResponse(boot.state);
  const automationPath = path.join(
    receiptRoot,
    "agents",
    "automate-list-latest.json",
  );
  const readinessPath = path.join(
    receiptRoot,
    "n8n-studio",
    "readiness-latest.json",
  );

  writeJsonReceipt(automationPath, automationPacket);
  writeJsonReceipt(readinessPath, readiness);

  return {
    schema: "dx.n8n-studio.zed-receipt-export",
    receiptRoot,
    automationReceiptPath: automationPath,
    readinessReceiptPath: readinessPath,
    automationCount: automationPacket.automation_count,
    readinessStatus: readiness.status,
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    redaction: {
      exports_secret_values: false,
      exports_provider_credentials: false,
    },
    issue:
      "Receipts are written for Zed and DX automation discovery only. Runtime handoff and webhook trigger request bridges can submit provider requests through configured credentials, while execution proof remains blocked until governed execution receipts are written from imported history.",
  };
}

export function writeZedAutomationBridgeReceiptsFromLocalGeneratedSource(
  startDirectory = process.cwd(),
  receiptRoot?: string,
) {
  return writeZedAutomationBridgeReceipts({
    boot: createStudioBootFromLocalGeneratedSource(startDirectory),
    receiptRoot,
  });
}
