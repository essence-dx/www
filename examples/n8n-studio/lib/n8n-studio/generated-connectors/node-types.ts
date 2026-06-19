import type { NodeParameterDefinition, NodeTypeDescription } from "../node-types/types";
import type { GeneratedConnectorOption, GeneratedConnectorRecord } from "./types";
import { generatedExecutableOperationOptions } from "./operation-options";
import { generatedTriggerConfigurationBoundary } from "./trigger-boundary";

function optionFromGenerated(option: GeneratedConnectorOption) {
  return {
    name: option.name,
    value: option.value,
    action: option.action,
    description: option.action,
  };
}

function createOptionsField(
  name: string,
  label: string,
  options: GeneratedConnectorOption[],
): NodeParameterDefinition | undefined {
  if (options.length === 0) {
    return undefined;
  }

  return {
    name,
    label,
    type: "options",
    defaultValue: options[0]?.value,
    options: options.map(optionFromGenerated),
  };
}

function createCredentialField(record: GeneratedConnectorRecord): NodeParameterDefinition | undefined {
  if (record.credential_type_names.length === 0) {
    return undefined;
  }

  return {
    name: "credential",
    label: "Credential",
    type: "credentialsSelect",
    defaultValue: record.credential_type_names[0],
    credentialTypes: record.credential_type_names,
    description: "Credential type required before live execution can run.",
  };
}

function createGeneratedNodeSummaryField(
  record: GeneratedConnectorRecord,
): NodeParameterDefinition {
  const authKinds = record.auth_kinds.length
    ? record.auth_kinds.join(", ")
    : "none";
  const description =
    record.description ||
    `DX Automations connector metadata for ${record.display_name}.`;
  const summary = `${description} Source: ${record.source_file}. Runtime: ${record.workflow_node.run_mode}. Auth: ${authKinds}.`;
  const boundaryDescription = `Generated from ${record.source_file} in ${record.source_provenance}. Full node-specific parameter execution stays behind DX credential, dynamic option, resource locator, and runtime adapters.`;

  return {
    name: "generatedNodeSummary",
    label: "Generated Node Metadata",
    type: "notice",
    defaultValue: summary,
    noDataExpression: true,
    description: boundaryDescription,
  };
}

export function generatedRecordToNodeType(record: GeneratedConnectorRecord): NodeTypeDescription {
  const properties = [
    createGeneratedNodeSummaryField(record),
    createOptionsField("resource", "Resource", record.resources),
    createOptionsField(
      "operation",
      "Operation",
      generatedExecutableOperationOptions(record),
    ),
    generatedTriggerConfigurationBoundary(record),
    createCredentialField(record),
  ].filter((field): field is NodeParameterDefinition => Boolean(field));

  return {
    name: record.id,
    displayName: record.display_name,
    sourcePath: record.source_file,
    version: 1,
    credentials: record.credential_type_names.map((name) => ({
      name,
      required: record.status === "needs_credential",
    })),
    properties,
    categories: record.categories,
    aliases: record.aliases,
    authKinds: record.auth_kinds,
    sourceProvenance: record.source_provenance,
    workflowNode: record.workflow_node,
  };
}
