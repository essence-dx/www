import type { WorkflowDocument } from "./types";

export function exportWorkflowDocument(document: WorkflowDocument) {
  return {
    name: document.name,
    active: document.active,
    nodes: document.nodes.map((node) => ({
      id: node.id,
      name: node.name,
      type: node.type,
      typeVersion: node.typeVersion,
      position: [node.position.x, node.position.y],
      parameters: node.parameters,
      credentials: node.credentials,
      disabled: node.disabled ?? false,
      notes: node.notes,
    })),
    connections: document.connections.reduce<Record<string, { main: Array<Array<{ node: string; type: string; index: number }>> }>>(
      (connections, connection) => {
        const source = connections[connection.sourceNode] ?? { main: [[]] };
        source.main[0].push({
          node: connection.targetNode,
          type: connection.targetInput,
          index: connection.index,
        });
        connections[connection.sourceNode] = source;
        return connections;
      },
      {},
    ),
    pinData: document.pinData,
    tags: document.tags,
    meta: document.meta,
  };
}
