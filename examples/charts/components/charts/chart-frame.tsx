import { compileChart } from "../../lib/charts";
import type { ChartCatalogItem } from "../../lib/charts";
import type { SceneElement } from "../../lib/charts/scene";

export type ChartFrameProps = {
  item: ChartCatalogItem;
  compact?: boolean;
};

export function ChartFrame({ item, compact = false }: ChartFrameProps) {
  const scene = compileChart(item);

  return (
    <figure
      className={compact ? "chart-frame chart-frame-compact" : "chart-frame"}
      data-dx-chart-id={scene.id}
      data-dx-chart-family={item.family}
      data-dx-chart-task={item.task}
    >
      <svg
        className="chart-svg"
        role="img"
        aria-labelledby={`${scene.id}-title ${scene.id}-desc`}
        viewBox={`0 0 ${scene.width} ${scene.height}`}
        preserveAspectRatio="xMidYMid meet"
      >
        <title id={`${scene.id}-title`}>{scene.title}</title>
        <desc id={`${scene.id}-desc`}>{scene.description}</desc>
        {scene.elements.map((element) => renderSceneElement(element))}
      </svg>
      <figcaption className="chart-caption">
        <strong>{item.title}</strong>
        <span>{item.summary}</span>
      </figcaption>
      {scene.legend.length > 0 ? (
        <ul className="chart-legend" aria-label={`${item.title} legend`}>
          {scene.legend.map((entry, index) => (
            <li key={entry.label}>
              <span
                className="chart-legend-swatch"
                data-series-index={String(index % 8)}
                data-series-color={entry.color}
                aria-hidden="true"
              />
              {entry.label}
            </li>
          ))}
        </ul>
      ) : null}
    </figure>
  );
}

function renderSceneElement(element: SceneElement) {
  const common = {
    key: element.id,
    className: element.className,
    transform: element.transform,
    role: "label" in element && element.label ? "button" : undefined,
    "aria-label": "label" in element ? element.label : undefined,
    "aria-pressed": "label" in element && element.label ? "false" : undefined,
    "data-dx-chart-hit": "label" in element ? element.label : undefined,
    "data-dx-chart-mark-id": element.id,
    "data-dx-chart-label": "label" in element ? element.label : undefined,
    "data-dx-g2-coordinate": element.coordinateType,
    "data-dx-g2-coordinate-options": element.coordinateOptions,
    "data-dx-g2-view-id": element.viewId,
    "data-dx-g2-view-title": element.viewTitle,
    "data-dx-g2-view-region": element.viewRegion,
    "data-dx-graph-node-id": element.graphNodeId,
    "data-dx-graph-edge-id": element.graphEdgeId,
    "data-dx-graph-combo-id": element.graphComboId,
    "data-dx-graph-layout": element.graphLayout,
    "data-dx-graph-behaviors": element.graphBehaviors,
    "data-dx-graph-plugins": element.graphPlugins,
    "data-dx-graph-focus-node-id": element.graphFocusNodeId,
    "data-dx-graph-relation-state": element.graphRelationState,
    "data-dx-g2-chord-source": element.chordSource,
    "data-dx-g2-chord-target": element.chordTarget,
    "data-dx-g2-chord-weight": element.chordWeight,
    "data-dx-g2-chord-node-id": element.chordNodeId,
    "data-dx-s2-cell-id": element.tableCellId,
    "data-dx-s2-row-key": element.tableRowKey,
    "data-dx-s2-column-key": element.tableColumnKey,
    "data-dx-s2-value-field": element.tableValueField,
    "data-dx-s2-section": element.tableSection,
    "data-dx-s2-hierarchy": element.tableHierarchy,
    "data-dx-s2-interactions": element.tableInteractions,
    "data-dx-s2-totals": element.tableTotals,
    "data-dx-s2-sort-state": element.tableSortState,
    "data-dx-s2-drill-path": element.tableDrillPath,
    "data-dx-l7-layer-id": element.mapLayerId,
    "data-dx-l7-layer-type": element.mapLayerType,
    "data-dx-l7-feature-id": element.mapFeatureId,
    "data-dx-l7-projection": element.mapProjection,
    "data-dx-l7-viewport": element.mapViewport,
    "data-dx-l7-interactions": element.mapInteractions,
    "data-dx-l7-layer-zoom": element.mapLayerZoom,
    "data-dx-l7-auto-fit": element.mapLayerAutoFit,
    "data-dx-l7-blend": element.mapLayerBlend,
    "data-dx-l7-state": element.mapLayerState,
    "data-dx-l7-legend": element.mapLegend,
    "data-dx-ava-rule-id": element.adviceRuleId,
    "data-dx-ava-reason": element.adviceReason,
    "data-dx-ava-confidence": element.adviceConfidence,
    "data-dx-dataset-stage-id": element.dataSetStageId,
    "data-dx-dataset-stage-name": element.dataSetStageName,
    "data-dx-dataset-transform": element.dataSetTransform,
    "data-dx-dataset-row-count": element.dataSetRowCount,
    "data-dx-f2-viewport": element.mobileViewport,
    "data-dx-f2-gestures": element.mobileGestures,
    "data-dx-f2-pixel-ratio": element.mobilePixelRatio,
    "data-dx-f2-safe-area": element.mobileSafeArea,
    "data-dx-f2-word": element.wordCloudTerm,
    "data-dx-f2-word-weight": element.wordCloudWeight,
    "data-dx-x6-node-id": element.diagramNodeId,
    "data-dx-x6-port-id": element.diagramPortId,
    "data-dx-x6-edge-id": element.diagramEdgeId,
    "data-dx-x6-router": element.diagramRouter,
    "data-dx-x6-connector": element.diagramConnector,
    "data-dx-x6-interactions": element.diagramInteractions,
    "data-dx-x6-port-state": element.diagramPortState,
    "data-dx-x6-port-group": element.diagramPortGroup,
    "data-dx-x6-port-position": element.diagramPortPosition,
    tabIndex: "label" in element && element.label ? 0 : undefined,
  };

  if (element.kind === "rect") {
    return (
      <rect
        {...common}
        x={element.x}
        y={element.y}
        width={element.width}
        height={element.height}
        rx={element.radius ?? 0}
        fill={element.fill}
        stroke={element.stroke}
        opacity={element.opacity}
      />
    );
  }

  if (element.kind === "line") {
    return (
      <line
        {...common}
        x1={element.x1}
        y1={element.y1}
        x2={element.x2}
        y2={element.y2}
        stroke={element.stroke}
        strokeWidth={element.strokeWidth}
        opacity={element.opacity}
      />
    );
  }

  if (element.kind === "path") {
    return (
      <path
        {...common}
        d={element.d}
        fill={element.fill}
        stroke={element.stroke}
        strokeWidth={element.strokeWidth}
        opacity={element.opacity}
      />
    );
  }

  if (element.kind === "circle") {
    return (
      <circle
        {...common}
        cx={element.cx}
        cy={element.cy}
        r={element.r}
        fill={element.fill}
        stroke={element.stroke}
        opacity={element.opacity}
      />
    );
  }

  if (element.kind === "polygon") {
    return (
      <polygon
        {...common}
        points={element.points}
        fill={element.fill}
        stroke={element.stroke}
        opacity={element.opacity}
      />
    );
  }

  return (
    <text
      {...common}
      x={element.x}
      y={element.y}
      textAnchor={element.anchor ?? "start"}
      fill={element.fill}
      fontSize={element.fontSize}
      fontWeight={element.fontWeight}
    >
      {element.text}
    </text>
  );
}
