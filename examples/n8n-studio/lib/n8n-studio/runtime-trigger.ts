import type { ImportSanitationIssue } from "./workflow-import/types";
import { stripSecretValues } from "./workflow-import/secret-redaction";
import type { WorkflowDocument, WorkflowNode } from "./types";

export type RuntimeTriggerMethod =
  | "DELETE"
  | "GET"
  | "HEAD"
  | "PATCH"
  | "POST"
  | "PUT";

export type RuntimeTriggerPlanStatus = "provider-gated" | "blocked";

export type RuntimeTriggerMode = "webhook" | "manual-provider-control";

export type RuntimeTriggerPlan = {
  schema: "dx.n8n-studio.runtime-trigger.plan";
  status: RuntimeTriggerPlanStatus;
  triggerMode: RuntimeTriggerMode;
  workflowId: string;
  workflowName: string;
  triggerNodeId?: string;
  triggerNodeName?: string;
  triggerNodeType?: string;
  httpMethod?: RuntimeTriggerMethod;
  requiredCredentialType?: "n8nWebhookTrigger";
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  sourcePath?: "nodes/Webhook/Webhook.node.ts";
  sourceDescriptionPath?: "nodes/Webhook/description.ts";
  receiptPath: ".dx/receipts/n8n-studio/runtime/trigger-latest.sr";
  redaction: "secret-values-never-included";
  issue: string;
};

export type RuntimeTriggerRequest = {
  method: RuntimeTriggerMethod;
  url: string;
  headers: {
    Accept: "application/json";
    "Content-Type"?: "application/json";
  };
  body?: unknown;
};

export type RuntimeTriggerTransport = (
  request: RuntimeTriggerRequest,
) => Promise<unknown>;

export type RuntimeTriggerResponseSummary = {
  responseReceived: boolean;
  statusCode?: number;
  bodyStored: false;
  secretsIncluded: false;
};

export type RuntimeTriggerReceipt = {
  schema: "dx.n8n-studio.runtime-trigger.receipt";
  status: "submitted";
  workflowId: string;
  workflowName: string;
  triggerNodeId: string;
  triggerNodeName: string;
  triggerMode: "webhook";
  httpMethod: RuntimeTriggerMethod;
  providerBoundary: true;
  workflowExecutionRequested: true;
  liveProviderExecution: true;
  executionReceiptImported: false;
  executionProofRequired: true;
  secretsIncluded: false;
  sideEffectPolicy: "governed-live-webhook-trigger-request";
  targetOrigin: string;
  targetUrlStored: false;
  requestBodyStored: false;
  providerResponse: RuntimeTriggerResponseSummary;
  sourcePath: "nodes/Webhook/Webhook.node.ts";
  sourceDescriptionPath: "nodes/Webhook/description.ts";
  receiptPath: ".dx/receipts/n8n-studio/runtime/trigger-latest.sr";
  redaction: "secret-values-never-included";
  issue: string;
};

const runtimeTriggerReceiptPath =
  ".dx/receipts/n8n-studio/runtime/trigger-latest.sr";

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function numberValue(value: unknown) {
  return typeof value === "number" && Number.isFinite(value) ? value : undefined;
}

function isRuntimeTriggerMethod(value: unknown): value is RuntimeTriggerMethod {
  return (
    value === "DELETE" ||
    value === "GET" ||
    value === "HEAD" ||
    value === "PATCH" ||
    value === "POST" ||
    value === "PUT"
  );
}

function workflowTriggerNode(document: WorkflowDocument) {
  return document.nodes.find((node) =>
    node.type.toLowerCase().includes("trigger"),
  );
}

function webhookTriggerNode(document: WorkflowDocument) {
  return document.nodes.find((node) => node.type === "n8n-nodes-base.webhook");
}

function webhookMethod(node: WorkflowNode): RuntimeTriggerMethod {
  const method = node.parameters.httpMethod;
  if (isRuntimeTriggerMethod(method)) {
    return method;
  }

  if (Array.isArray(method)) {
    const firstSupportedMethod = method.find(isRuntimeTriggerMethod);
    if (firstSupportedMethod) {
      return firstSupportedMethod;
    }
  }

  return "GET";
}

function createBlockedPlan(
  document: WorkflowDocument,
  triggerNode: WorkflowNode | undefined,
): RuntimeTriggerPlan {
  return {
    schema: "dx.n8n-studio.runtime-trigger.plan",
    status: "blocked",
    triggerMode: "manual-provider-control",
    workflowId: document.id,
    workflowName: document.name,
    triggerNodeId: triggerNode?.id,
    triggerNodeName: triggerNode?.name,
    triggerNodeType: triggerNode?.type,
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    receiptPath: runtimeTriggerReceiptPath,
    redaction: "secret-values-never-included",
    issue:
      "This workflow does not expose a governed webhook trigger request. Manual and internal triggers require n8n provider controls, followed by imported execution receipts.",
  };
}

export function createRuntimeTriggerPlan(
  document: WorkflowDocument,
): RuntimeTriggerPlan {
  const webhookNode = webhookTriggerNode(document);
  if (!webhookNode) {
    return createBlockedPlan(document, workflowTriggerNode(document));
  }

  return {
    schema: "dx.n8n-studio.runtime-trigger.plan",
    status: "provider-gated",
    triggerMode: "webhook",
    workflowId: document.id,
    workflowName: document.name,
    triggerNodeId: webhookNode.id,
    triggerNodeName: webhookNode.name,
    triggerNodeType: webhookNode.type,
    httpMethod: webhookMethod(webhookNode),
    requiredCredentialType: "n8nWebhookTrigger",
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    sourcePath: "nodes/Webhook/Webhook.node.ts",
    sourceDescriptionPath: "nodes/Webhook/description.ts",
    receiptPath: runtimeTriggerReceiptPath,
    redaction: "secret-values-never-included",
    issue:
      "Webhook runtime trigger is available behind a credential-owned webhook URL. Submitting the trigger requests live provider execution and still requires imported execution receipts for proof.",
  };
}

function normalizeTriggerUrl(triggerUrl: string) {
  const parsedUrl = new URL(triggerUrl);
  if (parsedUrl.protocol !== "https:" && parsedUrl.protocol !== "http:") {
    throw new Error("Runtime trigger URL must use http or https.");
  }

  return parsedUrl.toString();
}

function targetOrigin(triggerUrl: string) {
  return new URL(normalizeTriggerUrl(triggerUrl)).origin;
}

function sanitizedPayload(payload: unknown) {
  const issues: ImportSanitationIssue[] = [];
  return stripSecretValues(
    payload ?? {},
    issues,
    undefined,
    "parameter-secret-stripped",
  );
}

function requestBody(method: RuntimeTriggerMethod, payload: unknown) {
  if (method === "GET" || method === "HEAD") {
    return undefined;
  }

  return sanitizedPayload(payload);
}

function createRuntimeTriggerRequest(options: {
  method: RuntimeTriggerMethod;
  triggerUrl: string;
  payload?: unknown;
}): RuntimeTriggerRequest {
  const body = requestBody(options.method, options.payload);

  return {
    method: options.method,
    url: normalizeTriggerUrl(options.triggerUrl),
    headers: {
      Accept: "application/json",
      ...(body === undefined ? {} : { "Content-Type": "application/json" as const }),
    },
    ...(body === undefined ? {} : { body }),
  };
}

function responseStatusCode(response: unknown) {
  const responseRecord = isRecord(response) ? response : {};
  return (
    numberValue(responseRecord.statusCode) ??
    numberValue(responseRecord.status) ??
    numberValue(responseRecord.code)
  );
}

function createRuntimeTriggerReceipt(options: {
  document: WorkflowDocument;
  plan: RuntimeTriggerPlan;
  request: RuntimeTriggerRequest;
  response: unknown;
}): RuntimeTriggerReceipt {
  if (
    options.plan.status !== "provider-gated" ||
    options.plan.triggerMode !== "webhook" ||
    !options.plan.triggerNodeId ||
    !options.plan.triggerNodeName ||
    !options.plan.httpMethod
  ) {
    throw new Error("Workflow does not expose a governed webhook trigger.");
  }

  return {
    schema: "dx.n8n-studio.runtime-trigger.receipt",
    status: "submitted",
    workflowId: options.document.id,
    workflowName: options.document.name,
    triggerNodeId: options.plan.triggerNodeId,
    triggerNodeName: options.plan.triggerNodeName,
    triggerMode: "webhook",
    httpMethod: options.request.method,
    providerBoundary: true,
    workflowExecutionRequested: true,
    liveProviderExecution: true,
    executionReceiptImported: false,
    executionProofRequired: true,
    secretsIncluded: false,
    sideEffectPolicy: "governed-live-webhook-trigger-request",
    targetOrigin: targetOrigin(options.request.url),
    targetUrlStored: false,
    requestBodyStored: false,
    providerResponse: {
      responseReceived: true,
      statusCode: responseStatusCode(options.response),
      bodyStored: false,
      secretsIncluded: false,
    },
    sourcePath: "nodes/Webhook/Webhook.node.ts",
    sourceDescriptionPath: "nodes/Webhook/description.ts",
    receiptPath: runtimeTriggerReceiptPath,
    redaction: "secret-values-never-included",
    issue:
      "Webhook trigger request was submitted through the governed runtime trigger bridge. This requests live provider execution, but execution proof still requires a separately imported n8n execution receipt.",
  };
}

export async function submitRuntimeTrigger(options: {
  document: WorkflowDocument;
  triggerUrl: string;
  method?: RuntimeTriggerMethod;
  payload?: unknown;
  transport: RuntimeTriggerTransport;
}): Promise<RuntimeTriggerReceipt> {
  const plan = createRuntimeTriggerPlan(options.document);
  if (plan.status !== "provider-gated") {
    throw new Error("Workflow does not expose a governed webhook trigger.");
  }

  const request = createRuntimeTriggerRequest({
    method: options.method ?? plan.httpMethod ?? "GET",
    triggerUrl: options.triggerUrl,
    payload: options.payload,
  });
  const response = await options.transport(request);

  return createRuntimeTriggerReceipt({
    document: options.document,
    plan,
    request,
    response,
  });
}
