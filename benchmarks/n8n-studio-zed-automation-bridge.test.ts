import assert from "node:assert/strict";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { pathToFileURL } from "node:url";

import { createStudioBootFromLocalGeneratedSource } from "../examples/n8n-studio/server/n8n-studio/generated-catalog-source";

const repoRoot = path.resolve(import.meta.dirname, "..");

test("n8n Studio exposes a Zed automation-list bridge packet without live execution", async () => {
  const helperPath = path.join(
    repoRoot,
    "examples",
    "n8n-studio",
    "server",
    "n8n-studio",
    "zed-automation-bridge.ts",
  );
  const routePath = path.join(
    repoRoot,
    "examples",
    "n8n-studio",
    "app",
    "api",
    "n8n-studio",
    "zed-automation",
    "route.ts",
  );
  const schedulerActionRoutePath = path.join(
    repoRoot,
    "examples",
    "n8n-studio",
    "app",
    "api",
    "n8n-studio",
    "runtime-execution-proof-retry",
    "route.ts",
  );

  assert.equal(fs.existsSync(helperPath), true);
  assert.equal(fs.existsSync(routePath), true);
  assert.equal(fs.existsSync(schedulerActionRoutePath), true);

  const boot = createStudioBootFromLocalGeneratedSource(repoRoot);
  const helper = await import(pathToFileURL(helperPath).href);
  const route = await import(pathToFileURL(routePath).href);
  const schedulerActionRoute = await import(
    pathToFileURL(schedulerActionRoutePath).href
  );
  const createBridgePacket =
    helper.createZedAutomationBridgePacket as (bootContext: typeof boot) => {
      schema_version: string;
      status: string;
      automation_count: number;
      active_task_count: number;
      automations: Array<{
        id: string;
        name: string;
        source: string;
        status: string;
        runtime_available: boolean;
        enabled: boolean;
        unavailable_reason: string;
        destination: { kind: string; label: string };
        schedule: { kind: string; summary: string };
        receipts: Array<{
          kind: string;
          schema_version: string;
          status: string;
          path: string;
        }>;
        actions: Array<{
          id: string;
          schema: string;
          status: string;
          method: string;
          endpoint: string;
          enabled: boolean;
          providerBoundary: true;
          liveProviderExecution: false;
          secretsIncluded: false;
          credentialHandoff: {
            required: true;
            acceptedCredentialTypes: string[];
            credentialValuesAccepted: false;
          };
          requestBody: {
            storesBody: false;
            requiredFields: string[];
            forbiddenFields: string[];
          };
          receiptPath: string;
        }>;
        next_action: string;
      }>;
      redaction: { exports_secret_values: false };
    };

  assert.equal(typeof createBridgePacket, "function");
  assert.equal(typeof route.GET, "function");
  assert.equal(typeof schedulerActionRoute.GET, "function");

  const packet = createBridgePacket(boot);
  const automation = packet.automations[0];
  const schedulerAction = automation.actions.find(
    (action) => action.id === "n8n-runtime-execution-proof-retry",
  );
  const serialized = JSON.stringify(packet);

  assert.equal(packet.schema_version, "dx.agents.zed.automation_list.v1");
  assert.equal(packet.status, "warning");
  assert.equal(packet.automation_count, 1);
  assert.equal(packet.active_task_count, boot.state.editorSession.requestPlans.length);
  assert.equal(automation.id, "n8n-studio-editor-session");
  assert.equal(automation.name, "n8n Studio editor-session");
  assert.equal(automation.source, "dx-www-n8n-studio");
  assert.equal(automation.status, "blocked");
  assert.equal(automation.runtime_available, false);
  assert.match(automation.unavailable_reason, /runtime handoff/i);
  assert.equal(automation.enabled, false);
  assert.equal(automation.destination.kind, "zed");
  assert.equal(automation.schedule.kind, "manual");
  assert.equal(automation.receipts[0]?.schema_version, "dx.n8n-studio.readiness");
  assert.deepEqual(
    automation.receipts.find(
      (receipt) => receipt.kind === "runtime-execution-proof",
    ),
    {
      kind: "runtime-execution-proof",
      schema_version: "dx.n8n-studio.runtime-execution-proof.receipt",
      status: "pending",
      path: ".dx/receipts/n8n-studio/runtime/execution-proof-latest.sr",
    },
  );
  assert.equal(
    schedulerAction?.schema,
    "dx.n8n-studio.runtime-execution-proof.scheduler-action",
  );
  assert.equal(schedulerAction.status, "credential-handoff-required");
  assert.equal(schedulerAction.method, "POST");
  assert.equal(
    schedulerAction.endpoint,
    "/api/n8n-studio/runtime-execution-proof-retry",
  );
  assert.equal(schedulerAction.enabled, false);
  assert.equal(schedulerAction.providerBoundary, true);
  assert.equal(schedulerAction.liveProviderExecution, false);
  assert.equal(schedulerAction.secretsIncluded, false);
  assert.deepEqual(schedulerAction.credentialHandoff.acceptedCredentialTypes, [
    "n8nApi",
  ]);
  assert.equal(
    schedulerAction.credentialHandoff.credentialValuesAccepted,
    false,
  );
  assert.deepEqual(schedulerAction.requestBody.requiredFields, [
    "apiCredentialId",
    "triggerReceiptPath",
    "maxAttempts",
  ]);
  assert.equal(schedulerAction.requestBody.storesBody, false);
  assert.equal(
    schedulerAction.requestBody.forbiddenFields.includes(
      "credential-secret-value",
    ),
    true,
  );
  assert.equal(
    schedulerAction.receiptPath,
    ".dx/receipts/n8n-studio/runtime/execution-proof-latest.sr",
  );
  assert.match(automation.next_action, /runtime proof receipts/i);
  assert.match(automation.next_action, /retry delayed history imports/i);
  assert.equal(packet.redaction.exports_secret_values, false);
  assert.equal(serialized.includes("apiKey"), false);
  assert.equal(serialized.includes("secretRef"), false);
  assert.equal(serialized.includes("must-not-survive"), false);

  const httpResponse = await route.GET(
    new Request("http://localhost/api/n8n-studio/zed-automation"),
  );
  const httpBody = await httpResponse.json();

  assert.equal(httpBody.schema_version, "dx.agents.zed.automation_list.v1");
  assert.equal(httpBody.automation_count, 1);
  assert.equal(httpBody.automations[0].id, "n8n-studio-editor-session");

  const schedulerActionResponse = await schedulerActionRoute.GET(
    new Request("http://localhost/api/n8n-studio/runtime-execution-proof-retry"),
  );
  const schedulerActionBody = await schedulerActionResponse.json();

  assert.equal(
    schedulerActionBody.schema,
    "dx.n8n-studio.runtime-execution-proof.scheduler-action",
  );
  assert.equal(schedulerActionBody.status, "credential-handoff-required");
  assert.equal(schedulerActionBody.secretsIncluded, false);
});

test("n8n Studio writes Zed-readable automation and readiness receipts", async () => {
  const helperPath = path.join(
    repoRoot,
    "examples",
    "n8n-studio",
    "server",
    "n8n-studio",
    "zed-automation-bridge.ts",
  );
  const receiptRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-n8n-studio-zed-receipts-"),
  );
  const helper = await import(pathToFileURL(helperPath).href);
  const writeReceipts = helper.writeZedAutomationBridgeReceipts as (options: {
    receiptRoot: string;
  }) => {
    schema: string;
    receiptRoot: string;
    automationReceiptPath: string;
    readinessReceiptPath: string;
    redaction: { exports_secret_values: false };
  };

  assert.equal(typeof writeReceipts, "function");

  const receipt = writeReceipts({ receiptRoot });
  const automationPacket = JSON.parse(
    fs.readFileSync(receipt.automationReceiptPath, "utf8"),
  );
  const readiness = JSON.parse(fs.readFileSync(receipt.readinessReceiptPath, "utf8"));
  const serialized = JSON.stringify({ receipt, automationPacket, readiness });

  assert.equal(receipt.schema, "dx.n8n-studio.zed-receipt-export");
  assert.equal(receipt.receiptRoot, receiptRoot);
  assert.equal(
    receipt.automationReceiptPath,
    path.join(receiptRoot, "agents", "automate-list-latest.json"),
  );
  assert.equal(
    receipt.readinessReceiptPath,
    path.join(receiptRoot, "n8n-studio", "readiness-latest.json"),
  );
  assert.equal(automationPacket.schema_version, "dx.agents.zed.automation_list.v1");
  assert.equal(automationPacket.receipt_path, ".dx/receipts/agents/automate-list-latest.json");
  assert.equal(automationPacket.automations[0].id, "n8n-studio-editor-session");
  assert.equal(
    automationPacket.automations[0].receipts[0].path,
    ".dx/receipts/n8n-studio/readiness-latest.json",
  );
  assert.equal(readiness.schema, "dx.n8n-studio.readiness");
  assert.equal(readiness.automationScope.editorPortTarget, false);
  assert.equal(receipt.redaction.exports_secret_values, false);
  assert.equal(serialized.includes("apiKey"), false);
  assert.equal(serialized.includes("secretRef"), false);
  assert.equal(serialized.includes("must-not-survive"), false);
});
