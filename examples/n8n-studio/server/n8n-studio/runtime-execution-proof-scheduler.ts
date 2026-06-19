import type { CredentialVaultBridge } from "../../lib/n8n-studio/credential-vault";
import type { N8nApiTransport } from "../../lib/n8n-studio/n8n-api-client";
import type { RuntimeTriggerReceipt } from "../../lib/n8n-studio/runtime-trigger";
import type { WorkflowDocument } from "../../lib/n8n-studio/types";
import {
  runRuntimeExecutionProofRetry,
  type RuntimeExecutionProofRetryResult,
} from "./runtime-execution-proof-receipts";

const runtimeExecutionProofRetryEndpoint =
  "/api/n8n-studio/runtime-execution-proof-retry";
const runtimeExecutionProofReceiptPath =
  ".dx/receipts/n8n-studio/runtime/execution-proof-latest.sr";
const maxSchedulerAttempts = 10;
const secretBearingFieldNames = new Set([
  "apikey",
  "api-key",
  "api_key",
  "authorization",
  "credential-secret-value",
  "password",
  "password-value",
  "provider-token",
  "provider-token-value",
  "providertoken",
  "secret",
  "secret-ref",
  "secretref",
  "secret_ref",
  "token",
  "webhook-url",
  "webhook-url-value",
  "webhookurl",
  "webhook_url",
]);

export type RuntimeExecutionProofSchedulerAction = {
  schema: "dx.n8n-studio.runtime-execution-proof.scheduler-action";
  id: "n8n-runtime-execution-proof-retry";
  label: "Retry runtime execution proof import";
  status: "credential-handoff-required";
  method: "POST";
  endpoint: typeof runtimeExecutionProofRetryEndpoint;
  enabled: false;
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  sideEffectPolicy: "read-only-history-import";
  credentialHandoff: {
    required: true;
    acceptedCredentialTypes: ["n8nApi"];
    credentialValuesAccepted: false;
    credentialIdsOnly: true;
  };
  requestBody: {
    storesBody: false;
    requiredFields: ["apiCredentialId", "triggerReceiptPath", "maxAttempts"];
    forbiddenFields: [
      "credential-secret-value",
      "provider-token-value",
      "password-value",
      "webhook-url-value",
    ];
  };
  receiptPath: typeof runtimeExecutionProofReceiptPath;
  resultSchema: "dx.n8n-studio.runtime-execution-proof.retry-result";
  redaction: "secret-values-never-included";
  issue: string;
};

export type RuntimeExecutionProofSchedulerRequest = {
  apiCredentialId: string;
  triggerReceiptPath: typeof runtimeExecutionProofReceiptPath;
  maxAttempts: number;
  credentialIdsOnly: true;
  credentialValuesAccepted: false;
  bodyStored: false;
};

export type RuntimeExecutionProofSchedulerResponse = {
  schema: "dx.n8n-studio.runtime-execution-proof.scheduler-response";
  status:
    | "credential-handoff-required"
    | "invalid-request"
    | "rejected-secret-bearing-request"
    | "runtime-context-unavailable";
  accepted: boolean;
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  redaction: "secret-values-never-included";
  action?: RuntimeExecutionProofSchedulerAction;
  request?: RuntimeExecutionProofSchedulerRequest;
  execution?: {
    attempted: false;
    reason: "credential-handoff-adapter-required";
    providerBoundary: true;
    liveProviderExecution: false;
    secretsIncluded: false;
  };
  nextAction:
    | "connect-dx-zed-credential-handoff-adapter"
    | "submit-id-only-runtime-proof-retry-request";
  issue: string;
};

export type RuntimeExecutionProofSchedulerHttpResponse = {
  statusCode: 400 | 409;
  body: RuntimeExecutionProofSchedulerResponse;
};

export type RuntimeExecutionProofSchedulerHandoffOptions = {
  request: RuntimeExecutionProofSchedulerRequest;
  credentialVault: CredentialVaultBridge;
  apiTransport: N8nApiTransport;
  document: WorkflowDocument;
  trigger: RuntimeTriggerReceipt;
  receiptRoot?: string;
};

export type RuntimeExecutionProofSchedulerRuntimeContext = Omit<
  RuntimeExecutionProofSchedulerHandoffOptions,
  "request"
>;

export type RuntimeExecutionProofSchedulerRuntimeContextResolver = (input: {
  request: RuntimeExecutionProofSchedulerRequest;
  httpRequest: Request;
  acceptedResponse: RuntimeExecutionProofSchedulerResponse;
}) =>
  | RuntimeExecutionProofSchedulerRuntimeContext
  | undefined
  | Promise<RuntimeExecutionProofSchedulerRuntimeContext | undefined>;

export type RuntimeExecutionProofSchedulerRouteOptions = {
  resolveRuntimeContext?: RuntimeExecutionProofSchedulerRuntimeContextResolver;
};

export type RuntimeExecutionProofSchedulerRoute = {
  GET: () => Response;
  POST: (request: Request) => Promise<Response>;
};

export function createRuntimeExecutionProofSchedulerAction(): RuntimeExecutionProofSchedulerAction {
  return {
    schema: "dx.n8n-studio.runtime-execution-proof.scheduler-action",
    id: "n8n-runtime-execution-proof-retry",
    label: "Retry runtime execution proof import",
    status: "credential-handoff-required",
    method: "POST",
    endpoint: runtimeExecutionProofRetryEndpoint,
    enabled: false,
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    sideEffectPolicy: "read-only-history-import",
    credentialHandoff: {
      required: true,
      acceptedCredentialTypes: ["n8nApi"],
      credentialValuesAccepted: false,
      credentialIdsOnly: true,
    },
    requestBody: {
      storesBody: false,
      requiredFields: ["apiCredentialId", "triggerReceiptPath", "maxAttempts"],
      forbiddenFields: [
        "credential-secret-value",
        "provider-token-value",
        "password-value",
        "webhook-url-value",
      ],
    },
    receiptPath: runtimeExecutionProofReceiptPath,
    resultSchema: "dx.n8n-studio.runtime-execution-proof.retry-result",
    redaction: "secret-values-never-included",
    issue:
      "DX/Zed schedulers can use this action contract to call the bounded runtime execution-proof retry executor after handing off an n8n API credential id and a prior trigger receipt path. The action descriptor never accepts or stores credential values, webhook URLs, request bodies, or provider response payloads.",
  };
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function normalizedFieldName(value: string) {
  return value.replace(/[^A-Za-z0-9_-]/g, "").toLowerCase();
}

function hasSecretBearingField(value: unknown): boolean {
  if (Array.isArray(value)) {
    return value.some((entry) => hasSecretBearingField(entry));
  }

  if (!isRecord(value)) {
    return false;
  }

  return Object.entries(value).some(([fieldName, fieldValue]) => {
    if (secretBearingFieldNames.has(normalizedFieldName(fieldName))) {
      return true;
    }

    return hasSecretBearingField(fieldValue);
  });
}

function parseMaxAttempts(value: unknown): number | undefined {
  if (typeof value !== "number" || !Number.isFinite(value)) {
    return undefined;
  }

  const attempts = Math.floor(value);
  if (attempts < 1) {
    return undefined;
  }

  return Math.min(attempts, maxSchedulerAttempts);
}

function readSchedulerRequest(
  value: unknown,
): RuntimeExecutionProofSchedulerRequest | undefined {
  if (!isRecord(value)) {
    return undefined;
  }

  const apiCredentialId =
    typeof value.apiCredentialId === "string"
      ? value.apiCredentialId.trim()
      : "";
  const triggerReceiptPath =
    typeof value.triggerReceiptPath === "string"
      ? value.triggerReceiptPath.trim()
      : "";
  const maxAttempts = parseMaxAttempts(value.maxAttempts);

  if (
    !apiCredentialId ||
    triggerReceiptPath !== runtimeExecutionProofReceiptPath ||
    maxAttempts === undefined
  ) {
    return undefined;
  }

  return {
    apiCredentialId,
    triggerReceiptPath: runtimeExecutionProofReceiptPath,
    maxAttempts,
    credentialIdsOnly: true,
    credentialValuesAccepted: false,
    bodyStored: false,
  };
}

function rejectedSchedulerResponse(
  status: RuntimeExecutionProofSchedulerResponse["status"],
  issue: string,
): RuntimeExecutionProofSchedulerHttpResponse {
  return {
    statusCode: 400,
    body: {
      schema: "dx.n8n-studio.runtime-execution-proof.scheduler-response",
      status,
      accepted: false,
      providerBoundary: true,
      liveProviderExecution: false,
      secretsIncluded: false,
      redaction: "secret-values-never-included",
      nextAction: "submit-id-only-runtime-proof-retry-request",
      issue,
    },
  };
}

export function createRuntimeExecutionProofSchedulerResponse(
  payload: unknown,
): RuntimeExecutionProofSchedulerHttpResponse {
  if (hasSecretBearingField(payload)) {
    return rejectedSchedulerResponse(
      "rejected-secret-bearing-request",
      "Scheduler retry requests must use credential ids only. Credential values, provider tokens, webhook URLs, request bodies, and provider response payloads are never accepted by this route.",
    );
  }

  const request = readSchedulerRequest(payload);
  if (!request) {
    return rejectedSchedulerResponse(
      "invalid-request",
      "Scheduler retry requests require apiCredentialId, the runtime execution-proof receipt path, and a positive maxAttempts value.",
    );
  }

  return {
    statusCode: 409,
    body: {
      schema: "dx.n8n-studio.runtime-execution-proof.scheduler-response",
      status: "credential-handoff-required",
      accepted: true,
      providerBoundary: true,
      liveProviderExecution: false,
      secretsIncluded: false,
      redaction: "secret-values-never-included",
      action: createRuntimeExecutionProofSchedulerAction(),
      request,
      execution: {
        attempted: false,
        reason: "credential-handoff-adapter-required",
        providerBoundary: true,
        liveProviderExecution: false,
        secretsIncluded: false,
      },
      nextAction: "connect-dx-zed-credential-handoff-adapter",
      issue:
        "The scheduler request is valid and redacted, but live retry execution requires a DX/Zed credential-handoff adapter to inject the n8n API transport without exposing credential values to this route.",
    },
  };
}

function runtimeContextUnavailableResponse(): RuntimeExecutionProofSchedulerHttpResponse {
  return {
    statusCode: 409,
    body: {
      schema: "dx.n8n-studio.runtime-execution-proof.scheduler-response",
      status: "runtime-context-unavailable",
      accepted: false,
      providerBoundary: true,
      liveProviderExecution: false,
      secretsIncluded: false,
      redaction: "secret-values-never-included",
      action: createRuntimeExecutionProofSchedulerAction(),
      execution: {
        attempted: false,
        reason: "credential-handoff-adapter-required",
        providerBoundary: true,
        liveProviderExecution: false,
        secretsIncluded: false,
      },
      nextAction: "connect-dx-zed-credential-handoff-adapter",
      issue:
        "The scheduler request is valid, but the DX/Zed host did not provide the credential vault, n8n API transport, workflow document, trigger receipt, and receipt root required for runtime proof retry execution.",
    },
  };
}

function retryResultStatusCode(result: RuntimeExecutionProofRetryResult) {
  return result.status === "proved" ? 200 : 202;
}

export function createRuntimeExecutionProofSchedulerRoute(
  options: RuntimeExecutionProofSchedulerRouteOptions = {},
): RuntimeExecutionProofSchedulerRoute {
  return {
    GET() {
      return Response.json(createRuntimeExecutionProofSchedulerAction());
    },

    async POST(httpRequest: Request) {
      let payload: unknown;

      try {
        payload = await httpRequest.json();
      } catch {
        payload = undefined;
      }

      const response = createRuntimeExecutionProofSchedulerResponse(payload);
      const acceptedRequest = response.body.request;

      if (
        response.statusCode !== 409 ||
        !acceptedRequest ||
        !options.resolveRuntimeContext
      ) {
        return Response.json(response.body, { status: response.statusCode });
      }

      const runtimeContext = await options.resolveRuntimeContext({
        request: acceptedRequest,
        httpRequest,
        acceptedResponse: response.body,
      });

      if (!runtimeContext) {
        const unavailable = runtimeContextUnavailableResponse();
        return Response.json(unavailable.body, {
          status: unavailable.statusCode,
        });
      }

      const result = await runRuntimeExecutionProofSchedulerHandoff({
        request: acceptedRequest,
        ...runtimeContext,
      });

      return Response.json(result, { status: retryResultStatusCode(result) });
    },
  };
}

export async function runRuntimeExecutionProofSchedulerHandoff(
  options: RuntimeExecutionProofSchedulerHandoffOptions,
): Promise<RuntimeExecutionProofRetryResult> {
  const client = await options.credentialVault.createN8nApiClient(
    options.request.apiCredentialId,
    options.apiTransport,
  );

  return runRuntimeExecutionProofRetry({
    document: options.document,
    trigger: options.trigger,
    receiptRoot: options.receiptRoot,
    maxAttempts: options.request.maxAttempts,
    importExecutionHistory: () => client.importExecutionHistory(options.document),
  });
}
