import { createCredentialReadinessForNode } from "./credentials";
import {
  createParameterSchemaForNode,
  getNodeTypeDescription,
  n8nNodeTypeRegistry,
} from "./node-type-registry";
import type { NodeTypeDescription } from "./node-types/types";
import type {
  CredentialReadiness,
  CredentialPickerOption,
  EditorSessionReadiness,
  EditorSessionRequestKind,
  EditorSessionRequestPlan,
  EditorSessionTransportResponse,
  ParameterOption,
  ParameterField,
  ResourceMapperSchema,
  ResourceMapperSchemaField,
  WorkflowDocument,
  WorkflowNode,
} from "./types";

const EDITOR_SESSION_SCHEMA = "dx.n8n-studio.editor-session" as const;
const REDACTION_POLICY = "secret-values-never-included" as const;
const PROVIDER_BOUNDARY_ISSUE =
  "Dynamic editor-session requests require the DX-owned n8n editor-session adapter before provider calls can run.";

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function blockedRequestBase(
  kind: EditorSessionRequestKind,
  node: WorkflowNode,
  issue: string,
): Pick<
  EditorSessionRequestPlan,
  | "kind"
  | "nodeId"
  | "nodeName"
  | "nodeType"
  | "status"
  | "providerBoundary"
  | "liveProviderExecution"
  | "secretsIncluded"
  | "redaction"
  | "issue"
> {
  return {
    kind,
    nodeId: node.id,
    nodeName: node.name,
    nodeType: node.type,
    status: "blocked",
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    redaction: REDACTION_POLICY,
    issue,
  };
}

function flattenParameterFields(fields: ParameterField[]): ParameterField[] {
  return fields.flatMap((field) => [
    field,
    ...(field.childFields ? flattenParameterFields(field.childFields) : []),
    ...(field.collectionItems?.flatMap((item) =>
      flattenParameterFields(item.fields),
    ) ?? []),
  ]);
}

function parameterFieldPath(field: ParameterField) {
  return field.valuePath?.map(String).join(".") || field.name;
}

function createDynamicParameterRequestPlans(
  node: WorkflowNode,
  fields: ParameterField[],
): EditorSessionRequestPlan[] {
  return flattenParameterFields(fields).flatMap((field) => {
    const fieldPath = parameterFieldPath(field);

    return [
      ...(field.resourceLocatorModes
        ?.filter((mode) => mode.searchListMethod)
        .map((mode) => ({
          ...blockedRequestBase(
            "dynamic-node-parameters",
            node,
            `Dynamic options for ${field.label || field.name} require the n8n editor-session adapter.`,
          ),
          fieldName: field.name,
          fieldLabel: field.label,
          loadMethod: mode.searchListMethod,
          dynamicLoadBoundary: `${node.type}.${fieldPath}.${mode.searchListMethod}`,
        })) ?? []),
      ...(field.dynamicOptions
        ? [
            {
              ...blockedRequestBase(
                "dynamic-node-parameters",
                node,
                field.dynamicOptions.issue,
              ),
              fieldName: field.name,
              fieldLabel: field.label,
              loadMethod: field.dynamicOptions.loadMethod,
              dynamicLoadBoundary: `${node.type}.${fieldPath}.${field.dynamicOptions.loadMethod}`,
            },
          ]
        : []),
    ];
  });
}

function createResourceLocatorRequestPlans(
  node: WorkflowNode,
  fields: ParameterField[],
): EditorSessionRequestPlan[] {
  return flattenParameterFields(fields).flatMap((field) => {
    const draft = field.resourceLocatorDraft;
    if (!draft?.searchable || !draft.listSearchMethod) {
      return [];
    }

    return [
      {
        ...blockedRequestBase(
          "resource-locator-search",
          node,
          draft.listBoundary.issue,
        ),
        fieldName: field.name,
        fieldLabel: field.label,
        loadMethod: draft.listSearchMethod,
        query: draft.query,
        selectedValue: draft.selectedValue,
        selectedLabel: draft.selectedLabel,
      },
    ];
  });
}

function createResourceMapperRequestPlans(
  node: WorkflowNode,
  fields: ParameterField[],
): EditorSessionRequestPlan[] {
  return flattenParameterFields(fields).flatMap((field) => {
    const mapper = field.resourceMapper;
    if (!mapper) {
      return [];
    }
    const fieldPath = parameterFieldPath(field);

    return [
      {
        ...blockedRequestBase(
          "resource-mapper-schema",
          node,
          mapper.issue,
        ),
        fieldName: field.name,
        fieldLabel: field.label,
        loadMethod: mapper.resourceMapperMethod,
        dynamicLoadBoundary: `${node.type}.${fieldPath}.${mapper.resourceMapperMethod}`,
      },
    ];
  });
}

function createCredentialRequestPlans(
  node: WorkflowNode,
  credentials: CredentialReadiness[],
): EditorSessionRequestPlan[] {
  return credentials.map((credential) => ({
    ...blockedRequestBase(
      "credential-list",
      node,
      credential.pickerBoundary.issue,
    ),
    credentialType: credential.credentialType,
    credentialKey: credential.credentialKey,
    required: credential.required,
    selectedCredentialId: credential.selectedCredentialId,
    selectedCredentialName: credential.selectedCredentialName,
    credentialOptionCount: credential.credentialOptions.length,
  }));
}

function createCredentialValidationRequestPlans(
  node: WorkflowNode,
  credentials: CredentialReadiness[],
): EditorSessionRequestPlan[] {
  return credentials.flatMap((credential) => {
    if (!credential.selectedCredentialId) {
      return [];
    }

    return [
      {
        ...blockedRequestBase(
          "credential-test",
          node,
          `Credential validation for ${credential.credentialType} requires the n8n editor-session adapter.`,
        ),
        credentialType: credential.credentialType,
        credentialKey: credential.credentialKey,
        required: credential.required,
        selectedCredentialId: credential.selectedCredentialId,
        selectedCredentialName: credential.selectedCredentialName,
      },
    ];
  });
}

function unavailableReadiness(
  selectedNodeId: string,
  issue: string,
  node?: WorkflowNode,
): EditorSessionReadiness {
  return {
    schema: EDITOR_SESSION_SCHEMA,
    status: "blocked",
    selectedNodeId,
    selectedNodeName: node?.name,
    nodeType: node?.type,
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    redaction: REDACTION_POLICY,
    dynamicParameterLoadCount: 0,
    resourceLocatorSearchCount: 0,
    resourceMapperRequestCount: 0,
    credentialRequestCount: 0,
    credentialValidationRequestCount: 0,
    fulfilledRequestCount: 0,
    hiddenFieldCount: 0,
    requestPlans: [],
    issue,
  };
}

function sanitizeTransportOption(
  option: ParameterOption,
): ParameterOption | undefined {
  const candidate = option as Partial<Record<keyof ParameterOption, unknown>>;
  if (
    !option ||
    typeof candidate.name !== "string" ||
    typeof candidate.value !== "string"
  ) {
    return undefined;
  }

  const sanitized: ParameterOption = {
    name: candidate.name,
    value: candidate.value,
  };

  if (typeof candidate.description === "string") {
    sanitized.description = candidate.description;
  }

  if (typeof candidate.action === "string") {
    sanitized.action = candidate.action;
  }

  return sanitized;
}

function sanitizeCredentialPickerOption(
  option: CredentialPickerOption,
  fallbackCredentialType: string,
): CredentialPickerOption | undefined {
  const candidate = option as Partial<
    Record<keyof CredentialPickerOption, unknown>
  >;
  const id = typeof candidate.id === "string" ? candidate.id : "";
  const name =
    typeof candidate.name === "string" ? candidate.name : fallbackCredentialType;

  if (!id && !name) {
    return undefined;
  }

  return {
    id,
    name,
    credentialType:
      typeof candidate.credentialType === "string"
        ? candidate.credentialType
        : fallbackCredentialType,
    source:
      candidate.source === "workflow-reference"
        ? "workflow-reference"
        : "editor-session-placeholder",
    redaction: REDACTION_POLICY,
  };
}

function sanitizeResourceMapperField(
  field: ResourceMapperSchemaField,
): ResourceMapperSchemaField | undefined {
  const candidate = field as Partial<
    Record<keyof ResourceMapperSchemaField, unknown>
  >;
  if (
    !field ||
    typeof candidate.id !== "string" ||
    typeof candidate.displayName !== "string"
  ) {
    return undefined;
  }

  const sanitized: ResourceMapperSchemaField = {
    id: candidate.id,
    displayName: candidate.displayName,
  };

  if (typeof candidate.required === "boolean") {
    sanitized.required = candidate.required;
  }

  if (typeof candidate.defaultMatch === "boolean") {
    sanitized.defaultMatch = candidate.defaultMatch;
  }

  if (typeof candidate.canBeUsedToMatch === "boolean") {
    sanitized.canBeUsedToMatch = candidate.canBeUsedToMatch;
  }

  if (typeof candidate.type === "string") {
    sanitized.type = candidate.type;
  }

  return sanitized;
}

function sanitizeResourceMapperSchema(
  schema: ResourceMapperSchema,
): ResourceMapperSchema {
  const candidate = schema as Partial<
    Record<keyof ResourceMapperSchema, unknown>
  >;
  const fields = Array.isArray(candidate.fields)
    ? candidate.fields.flatMap((field) => {
        const sanitized = sanitizeResourceMapperField(
          field as ResourceMapperSchemaField,
        );
        return sanitized ? [sanitized] : [];
      })
    : [];
  const sanitized: ResourceMapperSchema = { fields };

  if (isRecord(candidate.fieldWords)) {
    const fieldWords: ResourceMapperSchema["fieldWords"] = {};
    if (typeof candidate.fieldWords.singular === "string") {
      fieldWords.singular = candidate.fieldWords.singular;
    }
    if (typeof candidate.fieldWords.plural === "string") {
      fieldWords.plural = candidate.fieldWords.plural;
    }
    if (fieldWords.singular || fieldWords.plural) {
      sanitized.fieldWords = fieldWords;
    }
  }

  if (typeof candidate.mode === "string") {
    sanitized.mode = candidate.mode;
  }

  return sanitized;
}

function responseCanEnterReadiness(
  response: EditorSessionTransportResponse,
): boolean {
  return (
    response.providerBoundary === true &&
    response.liveProviderExecution === false &&
    response.secretsIncluded === false &&
    response.redaction === REDACTION_POLICY
  );
}

function responseMatchesRequestPlan(
  plan: EditorSessionRequestPlan,
  response: EditorSessionTransportResponse,
): boolean {
  if (
    !responseCanEnterReadiness(response) ||
    plan.status === "configured" ||
    plan.kind !== response.kind ||
    plan.nodeId !== response.nodeId ||
    plan.nodeType !== response.nodeType
  ) {
    return false;
  }

  if (response.kind === "credential-list") {
    return (
      plan.credentialType === response.credentialType &&
      (response.credentialKey === undefined ||
        plan.credentialKey === response.credentialKey)
    );
  }

  if (response.kind === "credential-test") {
    return (
      plan.credentialType === response.credentialType &&
      (response.credentialKey === undefined ||
        plan.credentialKey === response.credentialKey) &&
      (response.selectedCredentialId === undefined ||
        plan.selectedCredentialId === response.selectedCredentialId)
    );
  }

  if (response.dynamicLoadBoundary) {
    return plan.dynamicLoadBoundary === response.dynamicLoadBoundary;
  }

  return (
    response.fieldName !== undefined &&
    plan.fieldName === response.fieldName &&
    (response.loadMethod === undefined || plan.loadMethod === response.loadMethod)
  );
}

function issueAfterTransportResponses(
  readiness: EditorSessionReadiness,
  appliedResponseCount: number,
  blockedRequestCount: number,
): string {
  if (appliedResponseCount === 0) {
    return readiness.issue;
  }

  if (blockedRequestCount > 0) {
    return `${appliedResponseCount} governed editor-session response(s) applied; unresolved editor-session requests remain blocked.`;
  }

  return "Governed editor-session responses applied without live provider execution.";
}

function configuredRequestPlanFromResponse(
  plan: EditorSessionRequestPlan,
  response: EditorSessionTransportResponse,
): EditorSessionRequestPlan {
  if (response.kind === "credential-list") {
    const resolvedCredentialOptions = response.credentialOptions.flatMap(
      (option) => {
        const sanitized = sanitizeCredentialPickerOption(
          option,
          response.credentialType,
        );
        return sanitized ? [sanitized] : [];
      },
    );
    const selectedCredentialId =
      typeof response.selectedCredentialId === "string"
        ? response.selectedCredentialId
        : plan.selectedCredentialId;
    const selectedCredentialName =
      typeof response.selectedCredentialName === "string"
        ? response.selectedCredentialName
        : resolvedCredentialOptions.find(
            (option) => option.id === selectedCredentialId,
          )?.name ?? plan.selectedCredentialName;

    return {
      ...plan,
      status: "configured",
      providerBoundary: true,
      liveProviderExecution: false,
      secretsIncluded: false,
      redaction: REDACTION_POLICY,
      selectedCredentialId,
      selectedCredentialName,
      credentialOptionCount: resolvedCredentialOptions.length,
      resolvedCredentialOptionCount: resolvedCredentialOptions.length,
      resolvedCredentialOptions,
      issue:
        response.issue ??
        "Governed editor-session response applied without live provider execution.",
    };
  }

  if (response.kind === "credential-test") {
    return {
      ...plan,
      status: "configured",
      providerBoundary: true,
      liveProviderExecution: false,
      secretsIncluded: false,
      redaction: REDACTION_POLICY,
      credentialValidationStatus: response.validationStatus,
      credentialValidatedAt: response.validatedAt,
      credentialValidationMessage: response.message,
      issue:
        response.issue ??
        "Governed credential validation response applied without live provider execution.",
    };
  }

  if (response.kind === "resource-mapper-schema") {
    const resolvedSchema = sanitizeResourceMapperSchema(response.schema);

    return {
      ...plan,
      status: "configured",
      providerBoundary: true,
      liveProviderExecution: false,
      secretsIncluded: false,
      redaction: REDACTION_POLICY,
      resolvedFieldCount: resolvedSchema.fields.length,
      resolvedSchema,
      issue:
        response.issue ??
        "Governed editor-session response applied without live provider execution.",
    };
  }

  const resolvedOptions = response.options.flatMap((option) => {
    const sanitized = sanitizeTransportOption(option);
    return sanitized ? [sanitized] : [];
  });

  return {
    ...plan,
    status: "configured",
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    redaction: REDACTION_POLICY,
    resolvedQuery:
      typeof response.query === "string" ? response.query : plan.query,
    nextPageToken:
      typeof response.nextPageToken === "string"
        ? response.nextPageToken
        : undefined,
    resolvedOptionCount: resolvedOptions.length,
    resolvedOptions,
    issue:
      response.issue ??
      "Governed editor-session response applied without live provider execution.",
  };
}

export function applyEditorSessionTransportResponses(
  readiness: EditorSessionReadiness,
  responses: EditorSessionTransportResponse[],
): EditorSessionReadiness {
  let appliedResponseCount = 0;
  const requestPlans = readiness.requestPlans.map((plan) => {
    const response = responses.find((candidate) =>
      responseMatchesRequestPlan(plan, candidate),
    );

    if (!response) {
      return plan;
    }

    appliedResponseCount += 1;

    return configuredRequestPlanFromResponse(plan, response);
  });
  const fulfilledRequestCount = requestPlans.filter(
    (plan) => plan.status === "configured",
  ).length;
  const blockedRequestCount = requestPlans.length - fulfilledRequestCount;

  return {
    ...readiness,
    status:
      requestPlans.length > 0 && blockedRequestCount === 0
        ? "configured"
        : readiness.status,
    fulfilledRequestCount,
    requestPlans,
    issue: issueAfterTransportResponses(
      readiness,
      appliedResponseCount,
      blockedRequestCount,
    ),
  };
}

export function createEditorSessionReadinessForNode(
  node: WorkflowNode,
  registry: Record<string, NodeTypeDescription> = n8nNodeTypeRegistry,
): EditorSessionReadiness {
  let parameterSchema;
  let description;

  try {
    parameterSchema = createParameterSchemaForNode(node.type, node.parameters, registry);
    description = getNodeTypeDescription(node.type, registry);
  } catch {
    return unavailableReadiness(
      node.id,
      `Node type ${node.type} is not available in the source registry for editor-session planning.`,
      node,
    );
  }

  const dynamicParameterPlans = createDynamicParameterRequestPlans(
    node,
    parameterSchema.fields,
  );
  const resourceLocatorPlans = createResourceLocatorRequestPlans(
    node,
    parameterSchema.fields,
  );
  const resourceMapperPlans = createResourceMapperRequestPlans(
    node,
    parameterSchema.fields,
  );
  const credentialReadiness = createCredentialReadinessForNode(
    node,
    description,
  );
  const credentialPlans = createCredentialRequestPlans(node, credentialReadiness);
  const credentialValidationPlans = createCredentialValidationRequestPlans(
    node,
    credentialReadiness,
  );
  const requestPlans = [
    ...dynamicParameterPlans,
    ...resourceLocatorPlans,
    ...resourceMapperPlans,
    ...credentialPlans,
    ...credentialValidationPlans,
  ];

  return {
    schema: EDITOR_SESSION_SCHEMA,
    status: requestPlans.length > 0 ? "blocked" : "source-only",
    selectedNodeId: node.id,
    selectedNodeName: node.name,
    nodeType: node.type,
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    redaction: REDACTION_POLICY,
    dynamicParameterLoadCount: dynamicParameterPlans.length,
    resourceLocatorSearchCount: resourceLocatorPlans.length,
    resourceMapperRequestCount: resourceMapperPlans.length,
    credentialRequestCount: credentialPlans.length,
    credentialValidationRequestCount: credentialValidationPlans.length,
    fulfilledRequestCount: 0,
    hiddenFieldCount: parameterSchema.hiddenFields.length,
    requestPlans,
    issue:
      requestPlans.length > 0
        ? PROVIDER_BOUNDARY_ISSUE
        : "No editor-session bridge requests are required for the selected node.",
  };
}

export function createEditorSessionReadiness(
  document: WorkflowDocument,
  selectedNodeId: string,
  registry: Record<string, NodeTypeDescription> = n8nNodeTypeRegistry,
): EditorSessionReadiness {
  const selectedNode = document.nodes.find((node) => node.id === selectedNodeId);
  if (!selectedNode) {
    return unavailableReadiness(
      selectedNodeId,
      "Selected node is not available in the workflow document.",
    );
  }

  return createEditorSessionReadinessForNode(selectedNode, registry);
}
