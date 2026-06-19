import type { NodeTypeDescription } from "./node-types/types";
import type {
  CredentialPickerOption,
  CredentialReadiness,
  StudioReadinessStatus,
  WorkflowNode,
} from "./types";

function normalizeCredentialName(value: string) {
  return value.replace(/[^a-z0-9]/gi, "").toLowerCase();
}

function workflowCredentialEntry(
  node: WorkflowNode,
  credentialType: string,
) {
  const normalizedType = normalizeCredentialName(credentialType);
  return Object.entries(node.credentials ?? {}).find(
    ([credentialKey]) => normalizeCredentialName(credentialKey) === normalizedType,
  );
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

export function sanitizeCredentialReference(
  credentialType: string,
  reference: unknown,
): CredentialPickerOption | undefined {
  if (!isRecord(reference)) {
    return undefined;
  }

  const id = typeof reference.id === "string" ? reference.id : "";
  const name = typeof reference.name === "string" ? reference.name : credentialType;
  if (!id && !name) {
    return undefined;
  }

  return {
    id,
    name,
    credentialType,
    source: "workflow-reference",
    redaction: "secret-values-never-included",
  };
}

function credentialStatus(
  required: boolean,
  selectedCredentialId: string | undefined,
): StudioReadinessStatus {
  if (selectedCredentialId) {
    return "configured";
  }
  return required ? "blocked" : "source-only";
}

function readinessIssue(
  credentialType: string,
  required: boolean,
  selectedCredentialId: string | undefined,
) {
  if (selectedCredentialId) {
    return "Credential id and display name are available; live provider execution remains blocked.";
  }
  if (required) {
    return `A required credential reference for ${credentialType} has not been selected.`;
  }
  return `Optional credential ${credentialType} can be selected after the editor-session adapter is available.`;
}

export function createCredentialReadinessForNode(
  node: WorkflowNode,
  description: NodeTypeDescription,
): CredentialReadiness[] {
  return description.credentials.map((credential) => {
    const workflowEntry = workflowCredentialEntry(node, credential.name);
    const credentialKey = workflowEntry?.[0] ?? credential.name;
    const selectedOption = sanitizeCredentialReference(
      credential.name,
      workflowEntry?.[1],
    );
    const credentialOptions = selectedOption ? [selectedOption] : [];
    const selectedCredentialId = selectedOption?.id || undefined;
    const status = credentialStatus(credential.required, selectedCredentialId);

    return {
      nodeId: node.id,
      nodeName: node.name,
      credentialType: credential.name,
      credentialKey,
      required: credential.required,
      selectedCredentialId,
      selectedCredentialName: selectedOption?.name,
      credentialOptions,
      pickerBoundary: {
        status: "source-only",
        providerBoundary: true,
        liveProviderExecution: false,
        secretsIncluded: false,
        issue: "Credential list loading and validation require the n8n editor-session adapter.",
      },
      status,
      redaction: "secret-values-never-included",
      issue: readinessIssue(credential.name, credential.required, selectedCredentialId),
    };
  });
}

export const credentialReadiness: CredentialReadiness[] = [];
