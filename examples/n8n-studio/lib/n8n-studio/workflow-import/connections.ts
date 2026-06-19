import type { WorkflowConnection } from "../types";
import { isRecord } from "./records";
import type { ImportSanitationIssue } from "./types";

export function sanitizeConnections(
  importedConnections: unknown,
  keptNodeNames: Set<string>,
  issues: ImportSanitationIssue[],
) {
  if (!isRecord(importedConnections)) {
    return [];
  }

  const connections: WorkflowConnection[] = [];
  for (const [sourceName, sourceConnections] of Object.entries(importedConnections)) {
    if (!keptNodeNames.has(sourceName)) {
      issues.push({
        code: "connection-source-missing",
        message: `Dropped connection from missing source node "${sourceName}".`,
        nodeName: sourceName,
      });
      continue;
    }

    if (!isRecord(sourceConnections) || !Array.isArray(sourceConnections.main)) {
      continue;
    }

    for (const [outputIndex, connectionGroup] of sourceConnections.main.entries()) {
      if (!Array.isArray(connectionGroup)) {
        continue;
      }

      for (const connection of connectionGroup) {
        if (!isRecord(connection)) {
          continue;
        }

        const targetNode = typeof connection.node === "string" ? connection.node : "";
        if (!keptNodeNames.has(targetNode)) {
          issues.push({
            code: "connection-target-missing",
            message: `Dropped connection to missing target node "${targetNode}".`,
            nodeName: sourceName,
          });
          continue;
        }

        connections.push({
          id: `edge-${sourceName}-${targetNode}-${outputIndex}-${connections.length}`,
          sourceNode: sourceName,
          targetNode,
          sourceOutput: "main",
          targetInput: connection.type === "ai_tool" ? "ai_tool" : "main",
          index: typeof connection.index === "number" ? connection.index : 0,
        });
      }
    }
  }

  return connections;
}
