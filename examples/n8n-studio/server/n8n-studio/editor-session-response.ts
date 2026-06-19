import { applyEditorSessionActionToStudioState } from "../../lib/n8n-studio/editor-session-actions";
import type {
  CredentialPickerOption,
  CredentialValidationStatus,
  EditorSessionTransportResponse,
  ParameterOption,
  ResourceMapperSchema,
  ResourceMapperSchemaField,
} from "../../lib/n8n-studio/types";
import {
  createStudioBootFromLocalGeneratedSource,
  type GeneratedStudioBoot,
} from "./generated-catalog-source";
import { createReadinessResponse } from "./readiness-response";

const REDACTION_POLICY = "secret-values-never-included" as const;

type ParsedResponseBatch = {
  responses: EditorSessionTransportResponse[];
  rejectedResponseCount: number;
};

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function optionalString(value: unknown) {
  return typeof value === "string" ? value : undefined;
}

function governedResponseRecord(
  value: unknown,
): Record<string, unknown> | undefined {
  if (!isRecord(value)) {
    return undefined;
  }

  if (
    typeof value.nodeId !== "string" ||
    typeof value.nodeType !== "string" ||
    value.providerBoundary !== true ||
    value.liveProviderExecution !== false ||
    value.secretsIncluded !== false ||
    value.redaction !== REDACTION_POLICY
  ) {
    return undefined;
  }

  return value;
}

function readParameterOption(value: unknown): ParameterOption | undefined {
  if (
    !isRecord(value) ||
    typeof value.name !== "string" ||
    typeof value.value !== "string"
  ) {
    return undefined;
  }

  return {
    name: value.name,
    value: value.value,
    ...(typeof value.description === "string"
      ? { description: value.description }
      : {}),
    ...(typeof value.action === "string" ? { action: value.action } : {}),
  };
}

function readParameterOptions(value: unknown): ParameterOption[] {
  return Array.isArray(value)
    ? value.flatMap((option) => {
        const parsed = readParameterOption(option);
        return parsed ? [parsed] : [];
      })
    : [];
}

function readCredentialOption(
  value: unknown,
  credentialType: string,
): CredentialPickerOption | undefined {
  if (!isRecord(value)) {
    return undefined;
  }

  const id = optionalString(value.id) ?? "";
  const name = optionalString(value.name) ?? credentialType;
  if (!id && !name) {
    return undefined;
  }

  return {
    id,
    name,
    credentialType: optionalString(value.credentialType) ?? credentialType,
    source:
      value.source === "workflow-reference"
        ? "workflow-reference"
        : "editor-session-placeholder",
    redaction: REDACTION_POLICY,
  };
}

function readCredentialOptions(
  value: unknown,
  credentialType: string,
): CredentialPickerOption[] {
  return Array.isArray(value)
    ? value.flatMap((option) => {
        const parsed = readCredentialOption(option, credentialType);
        return parsed ? [parsed] : [];
      })
    : [];
}

function readCredentialValidationStatus(
  value: unknown,
): CredentialValidationStatus | undefined {
  if (value === "valid" || value === "invalid" || value === "unknown") {
    return value;
  }

  return undefined;
}

function readMapperField(value: unknown): ResourceMapperSchemaField | undefined {
  if (
    !isRecord(value) ||
    typeof value.id !== "string" ||
    typeof value.displayName !== "string"
  ) {
    return undefined;
  }

  return {
    id: value.id,
    displayName: value.displayName,
    ...(typeof value.required === "boolean" ? { required: value.required } : {}),
    ...(typeof value.defaultMatch === "boolean"
      ? { defaultMatch: value.defaultMatch }
      : {}),
    ...(typeof value.canBeUsedToMatch === "boolean"
      ? { canBeUsedToMatch: value.canBeUsedToMatch }
      : {}),
    ...(typeof value.type === "string" ? { type: value.type } : {}),
  };
}

function readResourceMapperSchema(
  value: unknown,
): ResourceMapperSchema | undefined {
  if (!isRecord(value) || !Array.isArray(value.fields)) {
    return undefined;
  }

  const schema: ResourceMapperSchema = {
    fields: value.fields.flatMap((field) => {
      const parsed = readMapperField(field);
      return parsed ? [parsed] : [];
    }),
  };

  if (isRecord(value.fieldWords)) {
    const singular = optionalString(value.fieldWords.singular);
    const plural = optionalString(value.fieldWords.plural);
    if (singular || plural) {
      schema.fieldWords = {
        ...(singular ? { singular } : {}),
        ...(plural ? { plural } : {}),
      };
    }
  }

  const mode = optionalString(value.mode);
  if (mode) {
    schema.mode = mode;
  }

  return schema;
}

function parseTransportResponse(
  value: unknown,
): EditorSessionTransportResponse | undefined {
  const record = governedResponseRecord(value);
  if (!record) {
    return undefined;
  }

  const base = {
    nodeId: record.nodeId,
    nodeType: record.nodeType,
    fieldName: optionalString(record.fieldName),
    loadMethod: optionalString(record.loadMethod),
    dynamicLoadBoundary: optionalString(record.dynamicLoadBoundary),
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    redaction: REDACTION_POLICY,
    issue: optionalString(record.issue),
  } as const;

  if (
    record.kind === "dynamic-node-parameters" ||
    record.kind === "resource-locator-search"
  ) {
    return {
      ...base,
      kind: record.kind,
      query: optionalString(record.query),
      nextPageToken: optionalString(record.nextPageToken),
      options: readParameterOptions(record.options),
    };
  }

  if (record.kind === "resource-mapper-schema") {
    const schema = readResourceMapperSchema(record.schema);
    return schema
      ? {
          ...base,
          kind: "resource-mapper-schema",
          schema,
        }
      : undefined;
  }

  if (record.kind === "credential-list") {
    const credentialType = optionalString(record.credentialType);
    if (!credentialType) {
      return undefined;
    }

    return {
      ...base,
      kind: "credential-list",
      credentialType,
      credentialKey: optionalString(record.credentialKey),
      selectedCredentialId: optionalString(record.selectedCredentialId),
      selectedCredentialName: optionalString(record.selectedCredentialName),
      credentialOptions: readCredentialOptions(
        record.credentialOptions,
        credentialType,
      ),
    };
  }

  if (record.kind === "credential-test") {
    const credentialType = optionalString(record.credentialType);
    const validationStatus = readCredentialValidationStatus(
      record.validationStatus,
    );
    if (!credentialType || !validationStatus) {
      return undefined;
    }

    return {
      ...base,
      kind: "credential-test",
      credentialType,
      credentialKey: optionalString(record.credentialKey),
      selectedCredentialId: optionalString(record.selectedCredentialId),
      validationStatus,
      validatedAt: optionalString(record.validatedAt),
      message: optionalString(record.message),
    };
  }

  return undefined;
}

export function parseEditorSessionTransportBatch(
  payload: unknown,
): ParsedResponseBatch {
  const rawResponses =
    isRecord(payload) && Array.isArray(payload.responses)
      ? payload.responses
      : [];
  const responses: EditorSessionTransportResponse[] = [];
  let rejectedResponseCount = 0;

  for (const rawResponse of rawResponses) {
    const response = parseTransportResponse(rawResponse);
    if (response) {
      responses.push(response);
    } else {
      rejectedResponseCount += 1;
    }
  }

  return { responses, rejectedResponseCount };
}

export function createEditorSessionResponseBatchResponse(
  payload: unknown,
  boot: GeneratedStudioBoot = createStudioBootFromLocalGeneratedSource(),
) {
  const parsed = parseEditorSessionTransportBatch(payload);
  const previousFulfilledRequestCount =
    boot.state.editorSession.fulfilledRequestCount;
  const state = applyEditorSessionActionToStudioState(boot.state, {
    kind: "applyTransportResponses",
    responses: parsed.responses,
  });
  const appliedResponseCount = Math.max(
    0,
    state.editorSession.fulfilledRequestCount - previousFulfilledRequestCount,
  );

  return {
    schema: "dx.n8n-studio.editor-session.response-batch",
    ok: true,
    status: state.editorSession.status,
    acceptedResponseCount: parsed.responses.length,
    appliedResponseCount,
    rejectedResponseCount: parsed.rejectedResponseCount,
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    redaction: REDACTION_POLICY,
    editorSession: state.editorSession,
    readiness: createReadinessResponse(state),
  };
}

export function createEditorSessionResponseBatchResponseFromLocalGeneratedSource(
  payload: unknown,
  startDirectory = process.cwd(),
) {
  return createEditorSessionResponseBatchResponse(
    payload,
    createStudioBootFromLocalGeneratedSource(startDirectory),
  );
}
