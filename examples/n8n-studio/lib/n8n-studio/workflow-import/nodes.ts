import { n8nNodeTypeRegistry } from "../node-type-registry";
import type { NodeTypeDescription } from "../node-types/types";
import type { WorkflowNode } from "../types";
import { sanitizeCredentials } from "./credential-redaction";
import { isRecord } from "./records";
import { stripSecretValues } from "./secret-redaction";
import type { ImportSanitationIssue } from "./types";

function normalizePosition(position: unknown) {
  if (Array.isArray(position)) {
    return {
      x: typeof position[0] === "number" ? position[0] : 0,
      y: typeof position[1] === "number" ? position[1] : 0,
    };
  }

  if (isRecord(position)) {
    return {
      x: typeof position.x === "number" ? position.x : 0,
      y: typeof position.y === "number" ? position.y : 0,
    };
  }

  return { x: 0, y: 0 };
}

export function sanitizeNode(
  node: unknown,
  issues: ImportSanitationIssue[],
  registry: Record<string, NodeTypeDescription> = n8nNodeTypeRegistry,
): WorkflowNode | null {
  if (!isRecord(node)) {
    return null;
  }

  const type = typeof node.type === "string" ? node.type : "";
  const name = typeof node.name === "string" ? node.name : type;
  if (!registry[type]) {
    issues.push({
      code: "unknown-node-type",
      message: `Dropped unsupported node type "${type}".`,
      nodeName: name,
    });
    return null;
  }

  const parameters = isRecord(node.parameters)
    ? (stripSecretValues(
        node.parameters,
        issues,
        name,
        "parameter-secret-stripped",
      ) as Record<string, unknown>)
    : {};

  return {
    id: typeof node.id === "string" ? node.id : `node-${name}`,
    name,
    type,
    typeVersion: typeof node.typeVersion === "number" ? node.typeVersion : 1,
    position: normalizePosition(node.position),
    parameters,
    credentials: sanitizeCredentials(node.credentials, issues, name),
    disabled: node.disabled === true,
    notes: typeof node.notes === "string" ? node.notes : undefined,
  };
}
