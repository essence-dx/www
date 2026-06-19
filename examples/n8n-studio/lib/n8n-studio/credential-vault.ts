import {
  createN8nApiClient,
  type N8nApiClient,
  type N8nApiTransport,
} from "./n8n-api-client";
import type {
  RuntimeHandoffOptions,
  RuntimeHandoffReceipt,
} from "./runtime-handoff";
import {
  createRuntimeExecutionProofReceipt,
  type RuntimeExecutionProofReceipt,
} from "./runtime-execution-proof";
import {
  submitRuntimeTrigger,
  type RuntimeTriggerMethod,
  type RuntimeTriggerReceipt,
  type RuntimeTriggerTransport,
} from "./runtime-trigger";
import type { CredentialValidationStatus, WorkflowDocument } from "./types";

export type CredentialVaultRecord = {
  credentialId: string;
  displayName: string;
  credentialType: string;
  baseUrl?: string;
  secretRef?: string;
  runtimeTriggerMethod?: RuntimeTriggerMethod;
};

export type CredentialSecretRequest = {
  credentialId: string;
  credentialType: string;
  secretRef: string;
};

export type CredentialSecretLoader = (
  request: CredentialSecretRequest,
) => Promise<string | undefined>;

export type CredentialVaultReadiness = {
  schema: "dx.n8n-studio.credential-vault";
  status: "configured" | "blocked";
  providerBoundary: true;
  liveProviderExecution: false;
  secretsIncluded: false;
  credentialCount: number;
  n8nApiCredentialCount: number;
  n8nApiCredentialIds: string[];
  n8nRuntimeTriggerCredentialCount: number;
  n8nRuntimeTriggerCredentialIds: string[];
  redaction: "secret-values-never-included";
  issue: string;
};

export type CredentialValidationTransportRequest = {
  credentialId: string;
  credentialType: string;
  displayName: string;
  baseUrl?: string;
  secretValue: string;
};

export type CredentialValidationTransportResult = {
  status: CredentialValidationStatus;
  statusCode?: number;
  message?: string;
  validatedAt?: string;
};

export type CredentialValidationTransport = (
  request: CredentialValidationTransportRequest,
) => Promise<CredentialValidationTransportResult>;

export type CredentialValidationReceipt = {
  schema: "dx.n8n-studio.credential-validation.receipt";
  status: CredentialValidationStatus;
  credentialId: string;
  credentialType: string;
  displayName: string;
  providerBoundary: true;
  liveProviderExecution: true;
  secretsIncluded: false;
  credentialSecretLoaded: true;
  providerResponse: {
    responseReceived: boolean;
    statusCode?: number;
    bodyStored: false;
    secretsIncluded: false;
  };
  validatedAt?: string;
  redaction: "secret-values-never-included";
  issue: string;
};

export type CredentialVaultBridge = {
  readiness: CredentialVaultReadiness;
  validateCredential: (
    credentialId: string,
    transport: CredentialValidationTransport,
  ) => Promise<CredentialValidationReceipt>;
  createN8nApiClient: (
    credentialId: string,
    transport: N8nApiTransport,
  ) => Promise<N8nApiClient>;
  submitRuntimeHandoff: (
    credentialId: string,
    transport: N8nApiTransport,
    document: WorkflowDocument,
    options?: RuntimeHandoffOptions,
  ) => Promise<RuntimeHandoffReceipt>;
  submitRuntimeTrigger: (
    credentialId: string,
    transport: RuntimeTriggerTransport,
    document: WorkflowDocument,
    payload?: unknown,
  ) => Promise<RuntimeTriggerReceipt>;
  submitRuntimeTriggerWithExecutionProof: (
    triggerCredentialId: string,
    apiCredentialId: string,
    triggerTransport: RuntimeTriggerTransport,
    apiTransport: N8nApiTransport,
    document: WorkflowDocument,
    payload?: unknown,
  ) => Promise<RuntimeExecutionProofReceipt>;
};

export type CredentialVaultBridgeOptions = {
  records: CredentialVaultRecord[];
  loadSecret: CredentialSecretLoader;
};

function isN8nApiRecord(record: CredentialVaultRecord) {
  return record.credentialType === "n8nApi";
}

function isN8nWebhookTriggerRecord(record: CredentialVaultRecord) {
  return record.credentialType === "n8nWebhookTrigger";
}

function hasHttpBaseUrl(baseUrl: string | undefined) {
  if (!baseUrl) {
    return false;
  }

  try {
    const parsedUrl = new URL(baseUrl);
    return parsedUrl.protocol === "http:" || parsedUrl.protocol === "https:";
  } catch {
    return false;
  }
}

function n8nApiRecords(records: CredentialVaultRecord[]) {
  return records.filter(isN8nApiRecord);
}

function n8nWebhookTriggerRecords(records: CredentialVaultRecord[]) {
  return records.filter(isN8nWebhookTriggerRecord);
}

function recordCanCreateN8nClient(record: CredentialVaultRecord) {
  return Boolean(hasHttpBaseUrl(record.baseUrl) && record.secretRef);
}

function recordCanCreateRuntimeTrigger(record: CredentialVaultRecord) {
  return Boolean(record.secretRef);
}

function n8nApiCredentialIds(records: CredentialVaultRecord[]) {
  return n8nApiRecords(records)
    .filter(recordCanCreateN8nClient)
    .map((record) => record.credentialId)
    .sort();
}

function n8nRuntimeTriggerCredentialIds(records: CredentialVaultRecord[]) {
  return n8nWebhookTriggerRecords(records)
    .filter(recordCanCreateRuntimeTrigger)
    .map((record) => record.credentialId)
    .sort();
}

export function createCredentialVaultReadiness(
  records: CredentialVaultRecord[] = [],
): CredentialVaultReadiness {
  const availableN8nCredentialIds = n8nApiCredentialIds(records);
  const availableRuntimeTriggerCredentialIds =
    n8nRuntimeTriggerCredentialIds(records);
  const configured =
    availableN8nCredentialIds.length > 0 ||
    availableRuntimeTriggerCredentialIds.length > 0;

  return {
    schema: "dx.n8n-studio.credential-vault",
    status: configured ? "configured" : "blocked",
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    credentialCount: records.length,
    n8nApiCredentialCount: availableN8nCredentialIds.length,
    n8nApiCredentialIds: availableN8nCredentialIds,
    n8nRuntimeTriggerCredentialCount:
      availableRuntimeTriggerCredentialIds.length,
    n8nRuntimeTriggerCredentialIds: availableRuntimeTriggerCredentialIds,
    redaction: "secret-values-never-included",
    issue: configured
      ? "Credential vault can resolve n8n API and webhook trigger credentials by id for governed runtime handoff without exposing secret values to Studio state."
      : "Credential vault requires an n8n API or webhook trigger credential id with the needed secret reference before provider runtime requests can run.",
  };
}

function findN8nApiRecord(
  records: CredentialVaultRecord[],
  credentialId: string,
) {
  return records.find(
    (record) => record.credentialId === credentialId && isN8nApiRecord(record),
  );
}

function findN8nWebhookTriggerRecord(
  records: CredentialVaultRecord[],
  credentialId: string,
) {
  return records.find(
    (record) =>
      record.credentialId === credentialId && isN8nWebhookTriggerRecord(record),
  );
}

function findCredentialRecord(
  records: CredentialVaultRecord[],
  credentialId: string,
) {
  return records.find((record) => record.credentialId === credentialId);
}

function ensureCredentialRecord(
  records: CredentialVaultRecord[],
  credentialId: string,
) {
  const record = findCredentialRecord(records, credentialId);
  if (!record?.secretRef) {
    throw new Error("Credential is not available.");
  }
  if (record.baseUrl && !hasHttpBaseUrl(record.baseUrl)) {
    throw new Error("Credential base URL must use http or https.");
  }

  return record;
}

function ensureN8nApiRecord(
  records: CredentialVaultRecord[],
  credentialId: string,
) {
  const record = findN8nApiRecord(records, credentialId);
  if (!record || !record.baseUrl || !record.secretRef) {
    throw new Error("n8n API credential is not available.");
  }
  if (!hasHttpBaseUrl(record.baseUrl)) {
    throw new Error("n8n API base URL must use http or https.");
  }

  return record;
}

function ensureN8nWebhookTriggerRecord(
  records: CredentialVaultRecord[],
  credentialId: string,
) {
  const record = findN8nWebhookTriggerRecord(records, credentialId);
  if (!record?.secretRef) {
    throw new Error("n8n webhook trigger credential is not available.");
  }

  return record;
}

function normalizedValidationStatus(
  status: CredentialValidationTransportResult["status"],
): CredentialValidationStatus {
  return status === "valid" || status === "invalid" ? status : "unknown";
}

function createCredentialValidationReceipt(
  record: CredentialVaultRecord,
  result: CredentialValidationTransportResult,
): CredentialValidationReceipt {
  const status = normalizedValidationStatus(result.status);

  return {
    schema: "dx.n8n-studio.credential-validation.receipt",
    status,
    credentialId: record.credentialId,
    credentialType: record.credentialType,
    displayName: record.displayName,
    providerBoundary: true,
    liveProviderExecution: true,
    secretsIncluded: false,
    credentialSecretLoaded: true,
    providerResponse: {
      responseReceived: true,
      ...(typeof result.statusCode === "number"
        ? { statusCode: result.statusCode }
        : {}),
      bodyStored: false,
      secretsIncluded: false,
    },
    ...(typeof result.validatedAt === "string"
      ? { validatedAt: result.validatedAt }
      : {}),
    redaction: "secret-values-never-included",
    issue:
      result.message ??
      "Credential validation ran through an injected provider adapter. Secret values, secret refs, and provider response bodies are not stored in the returned receipt.",
  };
}

export function createCredentialVaultBridge(
  options: CredentialVaultBridgeOptions,
): CredentialVaultBridge {
  const loadSecret = async (record: CredentialVaultRecord) => {
    if (!record.secretRef) {
      throw new Error("Credential secret is unavailable.");
    }

    const secret = await options.loadSecret({
      credentialId: record.credentialId,
      credentialType: record.credentialType,
      secretRef: record.secretRef,
    });

    if (!secret) {
      throw new Error("Credential secret is unavailable.");
    }

    return secret;
  };
  const createClient = async (
    credentialId: string,
    transport: N8nApiTransport,
  ) => {
    const record = ensureN8nApiRecord(options.records, credentialId);
    const apiKey = await loadSecret(record);

    return createN8nApiClient(
      {
        credentialId: record.credentialId,
        displayName: record.displayName,
        baseUrl: record.baseUrl,
        apiKey,
      },
      transport,
    );
  };
  const submitTrigger = async (
    credentialId: string,
    transport: RuntimeTriggerTransport,
    document: WorkflowDocument,
    payload?: unknown,
  ) => {
    const record = ensureN8nWebhookTriggerRecord(
      options.records,
      credentialId,
    );
    const triggerUrl = await loadSecret(record);

    return submitRuntimeTrigger({
      document,
      triggerUrl,
      method: record.runtimeTriggerMethod,
      payload,
      transport,
    });
  };

  return {
    readiness: createCredentialVaultReadiness(options.records),
    async validateCredential(credentialId, transport) {
      const record = ensureCredentialRecord(options.records, credentialId);
      const secretValue = await loadSecret(record);
      const result = await transport({
        credentialId: record.credentialId,
        credentialType: record.credentialType,
        displayName: record.displayName,
        baseUrl: record.baseUrl,
        secretValue,
      });

      return createCredentialValidationReceipt(record, result);
    },
    createN8nApiClient: createClient,
    async submitRuntimeHandoff(credentialId, transport, document, options) {
      const client = await createClient(credentialId, transport);
      return client.submitRuntimeHandoff(document, options);
    },
    submitRuntimeTrigger: submitTrigger,
    async submitRuntimeTriggerWithExecutionProof(
      triggerCredentialId,
      apiCredentialId,
      triggerTransport,
      apiTransport,
      document,
      payload,
    ) {
      const trigger = await submitTrigger(
        triggerCredentialId,
        triggerTransport,
        document,
        payload,
      );
      const client = await createClient(apiCredentialId, apiTransport);
      const execution = await client.importExecutionHistory(document);

      return createRuntimeExecutionProofReceipt({
        document,
        trigger,
        execution,
      });
    },
  };
}
