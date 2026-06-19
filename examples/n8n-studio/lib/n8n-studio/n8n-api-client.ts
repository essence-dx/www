import type {
  ExecutionReadiness,
  SanitizedImportPreview,
  WorkflowDocument,
} from "./types";
import {
  createExecutionHistoryApiRequestPlan,
  importExecutionHistoryApiResponse,
} from "./execution-history-api";
import { createImportPreviewState } from "./import-export";
import {
  createWorkflowPublishBody,
  createWorkflowPublishReceipt,
  workflowIdFromPublishResponse,
  workflowPublishOperation,
  type WorkflowPublishOptions,
  type WorkflowPublishReceipt,
} from "./workflow-publish";
import {
  createRuntimeHandoffReceipt,
  type RuntimeHandoffOptions,
  type RuntimeHandoffReceipt,
} from "./runtime-handoff";

type N8nApiMethod = "GET" | "POST" | "PUT";

export type N8nApiCredentials = {
  credentialId: string;
  displayName: string;
  baseUrl: string;
  apiKey: string;
};

export type N8nApiClientRequest = {
  method: N8nApiMethod;
  path:
    | "/workflows"
    | "/workflows/{id}"
    | "/workflows/{id}/activate"
    | "/executions"
    | "/executions/{id}";
  url: string;
  query: Record<string, string | number | boolean>;
  body?: unknown;
  headers: {
    Accept: "application/json";
    "Content-Type"?: "application/json";
    "X-N8N-API-KEY": string;
  };
};

export type N8nApiTransport = (
  request: N8nApiClientRequest,
) => Promise<unknown>;

export type N8nApiClientReadiness = {
  schema: "dx.n8n-studio.n8n-api-client";
  status: "configured" | "blocked";
  credentialType: "n8nApi";
  credentialId?: string;
  credentialName?: string;
  baseUrlOrigin?: string;
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  workflowImportAvailable: boolean;
  executionHistoryImportAvailable: boolean;
  workflowPublishAvailable: boolean;
  workflowActivationAvailable: boolean;
  runtimeHandoffAvailable: boolean;
  redaction: "secret-values-never-included";
  issue: string;
};

export type N8nApiClient = {
  readiness: N8nApiClientReadiness;
  importWorkflow: (workflowId: string) => Promise<SanitizedImportPreview>;
  importExecutionHistory: (
    document: WorkflowDocument,
  ) => Promise<ExecutionReadiness>;
  publishWorkflow: (
    document: WorkflowDocument,
    options?: WorkflowPublishOptions,
  ) => Promise<WorkflowPublishReceipt>;
  submitRuntimeHandoff: (
    document: WorkflowDocument,
    options?: RuntimeHandoffOptions,
  ) => Promise<RuntimeHandoffReceipt>;
};

function normalizeBaseUrl(baseUrl: string) {
  const parsedUrl = new URL(baseUrl);
  if (parsedUrl.protocol !== "https:" && parsedUrl.protocol !== "http:") {
    throw new Error("n8n API base URL must use http or https.");
  }

  parsedUrl.hash = "";
  parsedUrl.search = "";
  return parsedUrl.toString().replace(/\/$/, "");
}

function baseUrlOrigin(baseUrl: string) {
  try {
    return new URL(normalizeBaseUrl(baseUrl)).origin;
  } catch {
    return undefined;
  }
}

function baseUrlIsValid(baseUrl: string | undefined) {
  if (!baseUrl) {
    return false;
  }

  try {
    normalizeBaseUrl(baseUrl);
    return true;
  } catch {
    return false;
  }
}

function encodedPathValue(value: string) {
  return encodeURIComponent(value);
}

function resolvePath(
  path: N8nApiClientRequest["path"],
  params: Record<string, string> = {},
) {
  if (
    path === "/workflows/{id}" ||
    path === "/workflows/{id}/activate" ||
    path === "/executions/{id}"
  ) {
    return path.replace("{id}", encodedPathValue(params.id ?? ""));
  }

  return path;
}

function requestUrl(
  baseUrl: string,
  path: N8nApiClientRequest["path"],
  params?: Record<string, string>,
) {
  return `${normalizeBaseUrl(baseUrl)}${resolvePath(path, params)}`;
}

function createRequest(
  credentials: N8nApiCredentials,
  path: N8nApiClientRequest["path"],
  options: {
    method?: N8nApiMethod;
    query?: Record<string, string | number | boolean>;
    params?: Record<string, string>;
    body?: unknown;
  } = {},
): N8nApiClientRequest {
  const method = options.method ?? "GET";

  return {
    method,
    path,
    url: requestUrl(credentials.baseUrl, path, options.params),
    query: options.query ?? {},
    ...(options.body === undefined ? {} : { body: options.body }),
    headers: {
      Accept: "application/json",
      ...(method === "GET" ? {} : { "Content-Type": "application/json" as const }),
      "X-N8N-API-KEY": credentials.apiKey,
    },
  };
}

export function createN8nApiClientReadiness(
  credentials?: Partial<N8nApiCredentials>,
): N8nApiClientReadiness {
  const hasRequiredFields = Boolean(
    credentials?.credentialId &&
      credentials.displayName &&
      credentials.baseUrl &&
      credentials.apiKey,
  );
  const validBaseUrl = baseUrlIsValid(credentials?.baseUrl);
  const configured = Boolean(hasRequiredFields && validBaseUrl);

  return {
    schema: "dx.n8n-studio.n8n-api-client",
    status: configured ? "configured" : "blocked",
    credentialType: "n8nApi",
    credentialId: credentials?.credentialId,
    credentialName: credentials?.displayName,
    baseUrlOrigin: credentials?.baseUrl
      ? baseUrlOrigin(credentials.baseUrl)
      : undefined,
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    workflowImportAvailable: configured,
    executionHistoryImportAvailable: configured,
    workflowPublishAvailable: configured,
    workflowActivationAvailable: configured,
    runtimeHandoffAvailable: configured,
    redaction: "secret-values-never-included",
    issue:
      hasRequiredFields && !validBaseUrl
        ? "n8n API base URL must use http or https."
        : configured
          ? "n8n API client is configured for governed workflow import, publish, activation, runtime handoff, and execution history import."
          : "n8n API client requires a credential id, display name, base URL, and secret value from the credential boundary.",
  };
}

export function createN8nApiClient(
  credentials: N8nApiCredentials,
  transport: N8nApiTransport,
): N8nApiClient {
  const importExecutionHistory = async (document: WorkflowDocument) => {
    const plan = createExecutionHistoryApiRequestPlan(document);
    const historyRequest = plan.requests.find(
      (request) => request.path === "/executions",
    );
    const payload = await transport(
      createRequest(
        credentials,
        "/executions",
        {
          query: historyRequest?.query ?? {
            workflowId: document.id,
            includeData: true,
            limit: 100,
          },
        },
      ),
    );

    return importExecutionHistoryApiResponse(document, payload);
  };

  const publishWorkflow = async (
    document: WorkflowDocument,
    options: WorkflowPublishOptions = {},
  ) => {
    const operation = workflowPublishOperation(options);
    const publishBody = createWorkflowPublishBody(document);
    const publishResponse = await transport(
      createRequest(
        credentials,
        operation === "create" ? "/workflows" : "/workflows/{id}",
        {
          method: operation === "create" ? "POST" : "PUT",
          params:
            operation === "create"
              ? undefined
              : { id: options.targetWorkflowId ?? "" },
          body: publishBody,
        },
      ),
    );
    const workflowId = workflowIdFromPublishResponse(
      publishResponse,
      options.targetWorkflowId ?? document.id,
    );
    const activate = options.activate === true;

    if (activate) {
      await transport(
        createRequest(credentials, "/workflows/{id}/activate", {
          method: "POST",
          params: { id: workflowId },
        }),
      );
    }

    return createWorkflowPublishReceipt({
      document,
      operation,
      workflowId,
      requestCount: activate ? 2 : 1,
      activationRequested: activate,
    });
  };

  return {
    readiness: createN8nApiClientReadiness(credentials),
    async importWorkflow(workflowId: string) {
      const payload = await transport(
        createRequest(credentials, "/workflows/{id}", {
          params: { id: workflowId },
        }),
      );

      return createImportPreviewState(payload, "url");
    },
    importExecutionHistory,
    publishWorkflow,
    async submitRuntimeHandoff(document, options = {}) {
      const publish = await publishWorkflow(document, options);
      const historyDocument = {
        ...document,
        id: publish.workflowId,
      };
      const execution = options.importExecutionHistory
        ? await importExecutionHistory(historyDocument)
        : undefined;

      return createRuntimeHandoffReceipt({
        document: historyDocument,
        publish,
        execution,
        executionHistoryImportRequested:
          options.importExecutionHistory === true,
      });
    },
  };
}
