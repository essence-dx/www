import type { ChartScene, SceneElement } from "../scene";
import type { ChartSpec, DiagramPortSpec, MarkSpec } from "../spec";
import { layoutDiagram, routeDiagramEdge } from "./diagram-layout";
import { buildDiagramModel, diagramSceneMetadata } from "./diagram-model";
import { emptyScene, scene, sceneId } from "./shared";

export function compileDiagram(spec: ChartSpec, mark: MarkSpec): ChartScene {
  if (!spec.diagram) return emptyScene(spec);

  const model = buildDiagramModel(spec.diagram);
  const layout = layoutDiagram(model, spec.width, spec.height);
  const metadata = diagramSceneMetadata(model);
  const elements: SceneElement[] = [];

  model.edges.forEach((edge) => {
    elements.push({
      kind: "path",
      id: sceneId(mark.id, "edge", edge.id),
      d: routeDiagramEdge(edge, layout),
      fill: "none",
      stroke: "hsl(var(--chart-muted-line))",
      strokeWidth: 2.25,
      label: edge.label ?? `${edge.source.cell} to ${edge.target.cell}`,
      className: `chart-graph-edge chart-diagram-router-${edge.router.name} chart-diagram-connector-${edge.connector.name}`,
      diagramEdgeId: edge.id,
      diagramRouter: edge.router.name,
      diagramConnector: edge.connector.name,
      ...metadata,
    });
  });

  Array.from(layout.nodes.values()).forEach((node) => {
    elements.push({
      kind: "rect",
      id: sceneId(mark.id, "node", node.id),
      x: node.x - node.width / 2,
      y: node.y - node.height / 2,
      width: node.width,
      height: node.height,
      fill: "hsl(var(--card))",
      stroke: "hsl(var(--chart-stroke))",
      radius: node.shape === "rect" ? 2 : 8,
      label: node.label,
      className: "chart-mark chart-graph-node",
      diagramNodeId: node.id,
      ...metadata,
    });
    elements.push({
      kind: "text",
      id: sceneId(mark.id, "node-label", node.id),
      x: node.x,
      y: node.y + 5,
      text: node.label,
      anchor: "middle",
      className: "chart-axis-label",
      diagramNodeId: node.id,
      ...metadata,
    });

    node.ports.forEach((port) => {
      elements.push({
        kind: "circle",
        id: sceneId(mark.id, "port", node.id, port.id),
        cx: port.x,
        cy: port.y,
        r: 5,
        fill: portFill(port),
        stroke: "hsl(var(--card))",
        label: `${node.label} ${port.label ?? port.id}`,
        className: `chart-mark-node diagram-port-state-${port.state} diagram-port-group-${port.group} diagram-port-position-${port.position}`,
        diagramNodeId: node.id,
        diagramPortId: port.id,
        diagramPortState: port.state,
        diagramPortGroup: port.group,
        diagramPortPosition: port.position,
        ...metadata,
      });
    });
  });

  return scene(spec, elements, []);
}

function portFill(port: DiagramPortSpec): string {
  if (port.state === "disabled") return "hsl(var(--chart-muted-line))";
  if (port.state === "active") return "hsl(var(--chart-warning))";
  if (port.state === "connected") return "hsl(var(--chart-success))";
  if (port.group === "input") return "hsl(var(--chart-info))";
  if (port.group === "control") return "hsl(var(--chart-violet))";
  return "hsl(var(--chart-success))";
}
