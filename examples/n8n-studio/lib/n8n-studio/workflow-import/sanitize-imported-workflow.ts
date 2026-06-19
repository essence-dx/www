import type { WorkflowNode } from "../types";
import { n8nNodeTypeRegistry } from "../node-type-registry";
import type { NodeTypeDescription } from "../node-types/types";
import { sanitizeConnections } from "./connections";
import { sanitizeNode } from "./nodes";
import { sanitizePinData } from "./pin-data";
import { isRecord } from "./records";
import type { ImportSanitationIssue, ImportSanitationResult } from "./types";
import { regenerateWebhookIds } from "./webhooks";

export function sanitizeImportedWorkflow(
  importedWorkflow: unknown,
  registry: Record<string, NodeTypeDescription> = n8nNodeTypeRegistry,
): ImportSanitationResult {
  const input = isRecord(importedWorkflow) ? importedWorkflow : {};
  const issues: ImportSanitationIssue[] = [];
  const nodes = Array.isArray(input.nodes)
    ? input.nodes
        .map((node) => sanitizeNode(node, issues, registry))
        .filter((node): node is WorkflowNode => node !== null)
    : [];
  const keptNodeNames = new Set(nodes.map((node) => node.name));

  return {
    document: {
      schemaVersion: 1,
      id: "imported-n8n-workflow",
      projectId: "local-dx-workspace",
      name: typeof input.name === "string" ? input.name : "Imported workflow",
      active: false,
      nodes,
      connections: sanitizeConnections(input.connections, keptNodeNames, issues),
      tags: Array.isArray(input.tags)
        ? input.tags.filter((tag): tag is string => typeof tag === "string")
        : [],
      pinData: sanitizePinData(input.pinData, keptNodeNames),
      meta: {
        source: "dx-www-n8n-studio",
        liveProviderExecution: false,
      },
    },
    issues,
    regeneratedWebhookIds: regenerateWebhookIds(input, issues),
  };
}
