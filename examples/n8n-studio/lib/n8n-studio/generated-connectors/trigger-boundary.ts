import type { NodeParameterDefinition } from "../node-types/types";
import type { GeneratedConnectorRecord } from "./types";

export function generatedTriggerConfigurationBoundary(
  record: GeneratedConnectorRecord,
): NodeParameterDefinition | undefined {
  if (!record.workflow_node.trigger) {
    return undefined;
  }

  return {
    name: "triggerConfiguration",
    label: "Trigger Configuration",
    type: "notice",
    defaultValue:
      "Generated trigger metadata is available, but detailed trigger event fields require a source-backed trigger adapter.",
    description:
      "Live trigger execution remains disabled until DX owns the source-backed trigger configuration and webhook/session adapter for this node.",
    noDataExpression: true,
  };
}
