import { round } from "../geometry";
import { readField, toLabel, toNumber, uniqueLabels } from "../format";
import { CHART_PALETTE, createPointScale } from "../scales";
import type { ChartSpec, Datum } from "../spec";
import type { ChartScene, SceneElement } from "../scene";
import { applyMarkTransforms } from "../transforms";
import { compileDiagram } from "./diagram-scene";
import { layoutGraph, routeGraphEdge } from "./graph-layout";
import { describeGraphBehavior, describeGraphPlugin, edgeKey, graphInteractionState, graphModelFromSpec } from "./graph-model";
import { emptyScene, plotBounds, scene, sceneId, withPadding } from "./shared";

export function compileSankey(spec: ChartSpec): ChartScene {
  const mark = spec.marks[0];
  const source = mark.encoding.source;
  const target = mark.encoding.target;
  const value = mark.encoding.size ?? mark.encoding.y;
  if (!source || !target) return emptyScene(spec);

  const data = applyMarkTransforms(spec.data, mark);
  const bounds = plotBounds(spec, withPadding({ left: 48, right: 48, top: 36, bottom: 36 }));
  const sources = uniqueLabels(data.map((datum) => readField(datum, source.field)));
  const targets = uniqueLabels(data.map((datum) => readField(datum, target.field)));
  const leftScale = createPointScale(sources, [bounds.top, bounds.bottom]);
  const rightScale = createPointScale(targets, [bounds.top, bounds.bottom]);
  const elements: SceneElement[] = [];

  data.forEach((datum, index) => {
    const sx = bounds.left + 34;
    const tx = bounds.right - 34;
    const sy = leftScale.map(readField(datum, source.field));
    const ty = rightScale.map(readField(datum, target.field));
    const width = value ? Math.max(2, Math.min(14, toNumber(readField(datum, value.field)) / 8)) : 5;
    elements.push({
      kind: "path",
      id: sceneId(mark.id, "flow", index),
      d: `M ${round(sx)} ${round(sy)} C ${round((sx + tx) / 2)} ${round(sy)}, ${round((sx + tx) / 2)} ${round(ty)}, ${round(tx)} ${round(ty)}`,
      fill: "none",
      stroke: CHART_PALETTE[index % CHART_PALETTE.length],
      strokeWidth: width,
      opacity: 0.62,
      label: `${toLabel(readField(datum, source.field))} to ${toLabel(readField(datum, target.field))}`,
      className: "chart-mark chart-mark-sankey",
      ...dataSetMetadata(datum),
    });
  });

  sources.forEach((name, index) => {
    elements.push({ kind: "rect", id: sceneId("sankey-source", name, index), x: bounds.left, y: leftScale.map(name) - 16, width: 68, height: 32, fill: CHART_PALETTE[index % CHART_PALETTE.length], radius: 6, className: "chart-mark-node" });
    elements.push({ kind: "text", id: sceneId("sankey-source-label", name, index), x: bounds.left + 34, y: leftScale.map(name) + 4, text: name, anchor: "middle", className: "chart-inside-label" });
  });
  targets.forEach((name, index) => {
    elements.push({ kind: "rect", id: sceneId("sankey-target", name, index), x: bounds.right - 68, y: rightScale.map(name) - 16, width: 68, height: 32, fill: CHART_PALETTE[(index + 3) % CHART_PALETTE.length], radius: 6, className: "chart-mark-node" });
    elements.push({ kind: "text", id: sceneId("sankey-target-label", name, index), x: bounds.right - 34, y: rightScale.map(name) + 4, text: name, anchor: "middle", className: "chart-inside-label" });
  });

  return scene(spec, elements, []);
}

function dataSetMetadata(datum: Datum) {
  const stageId = toLabel(readField(datum, "stageId"));
  if (!stageId) {
    return {};
  }

  return {
    dataSetStageId: stageId,
    dataSetStageName: toLabel(readField(datum, "stageName")),
    dataSetTransform: toLabel(readField(datum, "transform")),
    dataSetRowCount: toLabel(readField(datum, "rowCount")),
  };
}

export function compileGraph(spec: ChartSpec): ChartScene {
  const mark = spec.marks[0];
  if (spec.diagram) return compileDiagram(spec, mark);

  const bounds = plotBounds(spec, withPadding({ left: 38, right: 38, top: 38, bottom: 38 }));
  const model = graphModelFromSpec(spec, mark);
  if (model.nodes.length === 0 || model.edges.length === 0) return emptyScene(spec);

  const layout = layoutGraph(model, bounds, spec.width, spec.height);
  const interactionState = graphInteractionState(model);
  const graphMetadata = {
    graphLayout: model.layout.type,
    graphBehaviors: model.behaviors.map(describeGraphBehavior).join(","),
    graphPlugins: model.plugins.map(describeGraphPlugin).join(","),
    graphFocusNodeId: interactionState.focusNodeId ?? "none",
    graphRelationState: interactionState.relationState,
  };
  const elements: SceneElement[] = [];

  layout.combos.forEach((combo) => {
    elements.push({
      kind: "circle",
      id: sceneId(mark.id, "combo", combo.id),
      cx: combo.x,
      cy: combo.y,
      r: combo.r,
      fill: "hsl(var(--muted))",
      stroke: "hsl(var(--chart-muted-line))",
      opacity: 0.26,
      label: combo.label ?? combo.id,
      className: "chart-graph-combo",
      graphComboId: combo.id,
      ...graphMetadata,
    });
    elements.push({
      kind: "text",
      id: sceneId(mark.id, "combo-label", combo.id),
      x: combo.x,
      y: combo.y - combo.r - 8,
      text: combo.label ?? combo.id,
      anchor: "middle",
      className: "chart-axis-label",
      graphComboId: combo.id,
      ...graphMetadata,
    });
  });

  model.edges.forEach((edge, index) => {
    const edgeId = edgeKey(edge);
    const graphEdgeRoute = layout.edges.get(edgeId) ?? routeGraphEdge(edge, layout);
    if (!graphEdgeRoute) return;
    const active = interactionState.activatedEdgeIds.has(edgeId);
    const muted = interactionState.activatedEdgeIds.size > 0 && !active;
    elements.push({
      kind: "path",
      id: sceneId(mark.id, "edge", edgeId),
      d: graphEdgeRoute.d,
      fill: "none",
      stroke: active ? CHART_PALETTE[(index + 2) % CHART_PALETTE.length] : "hsl(var(--chart-muted-line))",
      strokeWidth: active ? Math.max(3, Math.min(8, (edge.weight ?? 2) + 1.5)) : Math.max(1.5, Math.min(6, edge.weight ?? 2)),
      opacity: muted ? 0.22 : active ? 0.9 : 0.68,
      label: edge.label ?? edge.relation ?? `${edge.source} to ${edge.target}`,
      className: `chart-graph-edge${active ? " chart-graph-edge-active" : ""}${muted ? " chart-graph-edge-muted" : ""}`,
      graphEdgeId: edgeId,
      ...graphMetadata,
    });
  });

  Array.from(layout.nodes.values()).forEach((node, index) => {
    const radius = Math.max(14, Math.min(24, 14 + (node.value ?? 4)));
    const focused = interactionState.focusedNodeIds.has(node.id);
    const dimmed = interactionState.focusedNodeIds.size > 0 && !focused;
    elements.push({
      kind: "circle",
      id: sceneId(mark.id, "node", node.id, index),
      cx: node.x,
      cy: node.y,
      r: radius,
      fill: node.color,
      stroke: "hsl(var(--chart-stroke))",
      opacity: dimmed ? 0.46 : 1,
      label: node.label ?? node.id,
      className: `chart-mark chart-graph-node${focused ? " chart-graph-node-focused" : ""}${dimmed ? " chart-graph-node-dimmed" : ""}`,
      graphNodeId: node.id,
      graphComboId: node.combo,
      ...graphMetadata,
    });
    elements.push({
      kind: "text",
      id: sceneId(mark.id, "node-label", node.id, index),
      x: node.x,
      y: node.y + radius + 16,
      text: node.label ?? node.id,
      anchor: "middle",
      className: "chart-axis-label",
      graphNodeId: node.id,
      graphComboId: node.combo,
      ...graphMetadata,
    });
  });

  return scene(spec, elements, []);
}
