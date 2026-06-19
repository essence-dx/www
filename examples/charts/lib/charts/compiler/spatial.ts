import { clamp, round } from "../geometry";
import { extent, formatValue } from "../format";
import { CHART_PALETTE } from "../scales";
import type { ChartSpec } from "../spec";
import type { ChartScene, LegendItem, SceneElement } from "../scene";
import { applyMarkTransforms } from "../transforms";
import {
  buildGeoModel,
  layerLegendMetadata,
  layerStateMetadata,
  layerZoomMetadata,
  projectionMetadata,
  viewportMetadata,
  type GeoFeature,
  type GeoModel,
} from "./geo-model";
import { emptyScene, plotBounds, scene, sceneId, withPadding } from "./shared";
import { buildSemanticTable, tableDrillPath, tableSortState, type SemanticTableCell, type SemanticTableModel } from "./table-model";

export function compileMap(spec: ChartSpec): ChartScene {
  const mark = spec.marks[0];
  const x = mark.encoding.x;
  const y = mark.encoding.y;
  if (!x || !y) return emptyScene(spec);

  const data = applyMarkTransforms(spec.data, mark);
  const bounds = plotBounds(spec, withPadding({ left: 34, right: 34, top: 34, bottom: 34 }));
  const model = buildGeoModel(spec, mark, data, bounds);
  const metadata = mapMetadata(model);
  const valueDomain = extent(model.features.map((feature) => feature.value));
  const legend = mapLegend(model, valueDomain);
  const elements: SceneElement[] = [];

  if (model.basemap.type !== "none") {
    elements.push({
      kind: "path",
      id: "map-base-a",
      d: `M ${bounds.left} ${bounds.top + 40} C ${bounds.left + 90} ${bounds.top - 10}, ${bounds.right - 120} ${bounds.top + 10}, ${bounds.right} ${bounds.top + 58} L ${bounds.right - 36} ${bounds.bottom - 30} C ${bounds.right - 150} ${bounds.bottom + 22}, ${bounds.left + 88} ${bounds.bottom}, ${bounds.left + 20} ${bounds.bottom - 44} Z`,
      fill: "hsl(var(--chart-map-land))",
      stroke: "hsl(var(--chart-stroke))",
      strokeWidth: 1,
      className: "chart-map-land",
      mapLayerId: "basemap",
      ...metadata,
    });
    pushMapGrid(elements, bounds.left, bounds.top, bounds.right, bounds.bottom, metadata);
  }

  model.features.forEach((feature, index) => {
    if (feature.layerType === "region") {
      pushMapRegion(elements, mark.id, feature, index, valueDomain, featureMapMetadata(model, feature));
      return;
    }

    const featureMetadata = featureMapMetadata(model, feature);
    elements.push({
      kind: "circle",
      id: sceneId(mark.id, feature.layerId, feature.id),
      cx: feature.x,
      cy: feature.y,
      r: feature.radius,
      fill: feature.layerStyle.fill ?? CHART_PALETTE[index % CHART_PALETTE.length],
      stroke: feature.layerStyle.stroke ?? (feature.layerType === "heatmap" ? undefined : "hsl(var(--chart-stroke))"),
      opacity: feature.layerStyle.opacity ?? 0.82,
      label: `${feature.label}: ${formatValue(feature.value, "compact")}`,
      className: featureClassName(feature),
      ...featureMetadata,
    });
  });

  return scene(spec, elements, legend);
}

function pushMapRegion(
  elements: SceneElement[],
  markId: string,
  feature: GeoFeature,
  index: number,
  valueDomain: [number, number],
  metadata: ReturnType<typeof mapMetadata>,
) {
  const intensity = clamp((feature.value - valueDomain[0]) / (valueDomain[1] - valueDomain[0] || 1), 0, 1);
  const colorIndex = Math.max(0, Math.min(CHART_PALETTE.length - 1, Math.round(intensity * (CHART_PALETTE.length - 1))));
  elements.push({
    kind: "path",
    id: sceneId(markId, feature.layerId, feature.id, index),
    d: regionPath(feature),
    fill: feature.layerStyle.fill ?? CHART_PALETTE[colorIndex],
    stroke: feature.layerStyle.stroke ?? "hsl(var(--chart-stroke))",
    strokeWidth: feature.layerStyle.strokeWidth ?? 1,
    opacity: feature.layerStyle.opacity ?? 0.84,
    label: `${feature.label}: ${formatValue(feature.value, "compact")}`,
    className: featureClassName(feature),
    ...metadata,
  });
}

function regionPath(feature: GeoFeature): string {
  if (feature.points && feature.points.length >= 3) {
    return `${feature.points.map((point, index) => `${index === 0 ? "M" : "L"} ${round(point.x)} ${round(point.y)}`).join(" ")} Z`;
  }

  const radius = Math.max(16, feature.radius);
  const points = [
    [feature.x - radius * 1.35, feature.y - radius * 0.5],
    [feature.x - radius * 0.38, feature.y - radius * 1.18],
    [feature.x + radius * 1.26, feature.y - radius * 0.72],
    [feature.x + radius * 1.08, feature.y + radius * 0.72],
    [feature.x - radius * 0.28, feature.y + radius * 1.12],
    [feature.x - radius * 1.28, feature.y + radius * 0.42],
  ];
  return `${points.map(([x, y], index) => `${index === 0 ? "M" : "L"} ${round(x)} ${round(y)}`).join(" ")} Z`;
}

function pushMapGrid(elements: SceneElement[], left: number, top: number, right: number, bottom: number, metadata: ReturnType<typeof mapMetadata>) {
  const columns = 4;
  const rows = 3;
  for (let index = 1; index < columns; index += 1) {
    const x = left + ((right - left) * index) / columns;
    elements.push({ kind: "line", id: sceneId("map-grid-col", index), x1: x, y1: top, x2: x, y2: bottom, stroke: "hsl(var(--chart-muted-line))", strokeWidth: 1, className: "chart-map-grid", mapLayerId: "graticule", ...metadata });
  }

  for (let index = 1; index < rows; index += 1) {
    const y = top + ((bottom - top) * index) / rows;
    elements.push({ kind: "line", id: sceneId("map-grid-row", index), x1: left, y1: y, x2: right, y2: y, stroke: "hsl(var(--chart-muted-line))", strokeWidth: 1, className: "chart-map-grid", mapLayerId: "graticule", ...metadata });
  }
}

function mapMetadata(model: GeoModel) {
  return {
    mapProjection: projectionMetadata(model.projection),
    mapViewport: viewportMetadata(model.viewport),
    mapInteractions: model.interactions.map((interaction) => interaction.type).join(","),
  };
}

function featureMapMetadata(model: GeoModel, feature: GeoFeature) {
  return {
    mapLayerId: feature.layerId,
    mapLayerType: feature.layerType,
    mapFeatureId: feature.id,
    mapProjection: projectionMetadata(model.projection),
    mapViewport: viewportMetadata(model.viewport),
    mapInteractions: feature.layerInteractions.map((interaction) => interaction.type).join(","),
    mapLayerZoom: layerZoomMetadata(feature),
    mapLayerAutoFit: String(feature.autoFit),
    mapLayerBlend: feature.blend,
    mapLayerState: layerStateMetadata(feature.layerState),
    mapLegend: layerLegendMetadata(feature.legend),
  };
}

function featureClassName(feature: GeoFeature): string {
  const base = feature.layerType === "region" ? "chart-mark chart-map-region" : "chart-mark chart-map-point";
  return [base, feature.layerStyle.className, `chart-map-layer-${feature.layerType}`].filter(Boolean).join(" ");
}

function mapLegend(model: GeoModel, valueDomain: [number, number]): LegendItem[] {
  const layerItems = model.layers
    .filter((layer) => layer.legend?.enabled !== false && layer.legend?.channel === "layer")
    .map((layer) => ({
      label: layer.legend?.title ?? readableLayerName(layer.id),
      color: layer.layerStyle.fill ?? colorForLayerType(layer.type),
    }));

  const mapItems = model.legends.flatMap((legend) => {
    if (legend.channel === "layer") {
      return model.layers.map((layer) => ({
        label: legend.title ? `${legend.title}: ${readableLayerName(layer.id)}` : readableLayerName(layer.id),
        color: layer.layerStyle.fill ?? colorForLayerType(layer.type),
      }));
    }

    if (legend.channel === "size") {
      return [
        { label: `${legend.title ?? "Size"} ${formatValue(valueDomain[0], "compact")}`, color: "hsl(var(--chart-muted-line))" },
        { label: `${legend.title ?? "Size"} ${formatValue(valueDomain[1], "compact")}`, color: "hsl(var(--chart-info))" },
      ];
    }

    return [
      { label: `${legend.title ?? "Value"} low`, color: CHART_PALETTE[0] },
      { label: `${legend.title ?? "Value"} high`, color: CHART_PALETTE[CHART_PALETTE.length - 1] },
    ];
  });

  return dedupeLegend([...layerItems, ...mapItems]);
}

function dedupeLegend(items: LegendItem[]): LegendItem[] {
  const seen = new Set<string>();
  return items.filter((item) => {
    const key = `${item.label}:${item.color}`;
    if (seen.has(key)) return false;
    seen.add(key);
    return true;
  });
}

function colorForLayerType(type: GeoFeature["layerType"]): string {
  if (type === "region") return "hsl(var(--chart-warning))";
  if (type === "heatmap") return "hsl(var(--chart-info))";
  if (type === "point") return "hsl(var(--chart-success))";
  return "hsl(var(--chart-accent))";
}

function readableLayerName(id: string): string {
  return id
    .replace(/[-_]+/g, " ")
    .replace(/\b\w/g, (letter) => letter.toUpperCase());
}

export function compilePivot(spec: ChartSpec): ChartScene {
  const mark = spec.marks[0];
  const x = mark.encoding.x;
  const y = mark.encoding.y;
  if (!spec.table && (!x || !y)) return emptyScene(spec);

  const data = applyMarkTransforms(spec.data, mark);
  const model = buildSemanticTable(spec, mark, data);
  const rowHeaderWidth = Math.max(86, Math.min(154, 58 + model.rowFields.length * 42));
  const columnHeaderHeight = Math.max(42, 28 + model.columnFields.length * 18 + (model.valueFields.length > 1 ? 16 : 0));
  const bounds = plotBounds(spec, withPadding({ left: rowHeaderWidth, right: 24, top: columnHeaderHeight, bottom: 28 }));
  const rowTotalColumns = model.rowTotals.length > 0 ? model.valueFields : [];
  const totalColumnCount = Math.max(1, model.columns.length + rowTotalColumns.length);
  const totalRowCount = Math.max(1, model.rows.length + (model.columnTotals.length > 0 ? 1 : 0));
  const colWidth = bounds.width / totalColumnCount;
  const rowHeight = bounds.height / totalRowCount;
  const valueDomain = extent(model.cells.map((cell) => cell.value));
  const metadata = tableMetadata(model);
  const elements: SceneElement[] = [];

  elements.push({
    kind: "rect",
    id: sceneId(mark.id, "corner"),
    x: 12,
    y: bounds.top - columnHeaderHeight + 8,
    width: bounds.left - 18,
    height: columnHeaderHeight - 14,
    fill: "hsl(var(--surface-raised))",
    stroke: "hsl(var(--border))",
    radius: 5,
    className: "chart-s2-header",
    tableSection: "column-header",
    ...metadata,
  });

  model.columns.forEach((column, columnIndex) => {
    const xPosition = bounds.left + colWidth * columnIndex;
    elements.push({
      kind: "rect",
      id: sceneId(mark.id, "column-header", column.key, columnIndex),
      x: xPosition,
      y: bounds.top - columnHeaderHeight + 8,
      width: colWidth - 6,
      height: columnHeaderHeight - 14,
      fill: "hsl(var(--surface-raised))",
      stroke: "hsl(var(--border))",
      radius: 5,
      label: column.labels.join(" / "),
      className: "chart-mark chart-s2-header",
      tableColumnKey: column.key,
      tableValueField: column.measure.field,
      tableSection: "column-header",
      ...metadata,
    });
    elements.push({
      kind: "text",
      id: sceneId(mark.id, "column-label", column.key, columnIndex),
      x: xPosition + colWidth / 2,
      y: bounds.top - 14,
      text: compactLabel(column.labels.join(" / "), 14),
      anchor: "middle",
      className: "chart-axis-label",
      tableColumnKey: column.key,
      tableValueField: column.measure.field,
      tableSection: "column-header",
      ...metadata,
    });
  });

  rowTotalColumns.forEach((measure, measureIndex) => {
    const xPosition = bounds.left + colWidth * (model.columns.length + measureIndex);
    elements.push({
      kind: "rect",
      id: sceneId(mark.id, "row-total-header", measure.field),
      x: xPosition,
      y: bounds.top - columnHeaderHeight + 8,
      width: colWidth - 6,
      height: columnHeaderHeight - 14,
      fill: "hsl(var(--muted))",
      stroke: "hsl(var(--border))",
      radius: 5,
      label: `Total ${measure.label ?? measure.field}`,
      className: "chart-mark chart-s2-header chart-s2-total-header",
      tableColumnKey: "Total",
      tableValueField: measure.field,
      tableSection: "row-total",
      ...metadata,
    });
    elements.push({
      kind: "text",
      id: sceneId(mark.id, "row-total-label", measure.field),
      x: xPosition + colWidth / 2,
      y: bounds.top - 14,
      text: "Total",
      anchor: "middle",
      className: "chart-axis-label",
      tableColumnKey: "Total",
      tableValueField: measure.field,
      tableSection: "row-total",
      ...metadata,
    });
  });

  model.rows.forEach((row, rowIndex) => {
    const yPosition = bounds.top + rowHeight * rowIndex;
    elements.push({
      kind: "rect",
      id: sceneId(mark.id, "row-header", row.key, rowIndex),
      x: 12,
      y: yPosition,
      width: bounds.left - 18,
      height: rowHeight - 6,
      fill: "hsl(var(--surface-raised))",
      stroke: "hsl(var(--border))",
      radius: 5,
      label: row.labels.join(" / "),
      className: "chart-mark chart-s2-header",
      tableRowKey: row.key,
      tableSection: "row-header",
      ...metadata,
    });
    elements.push({
      kind: "text",
      id: sceneId(mark.id, "row-label", row.key, rowIndex),
      x: bounds.left - 24,
      y: yPosition + rowHeight / 2 + 4,
      text: compactLabel(row.labels.join(" / "), 16),
      anchor: "end",
      className: "chart-axis-label",
      tableRowKey: row.key,
      tableSection: "row-header",
      ...metadata,
    });

    model.columns.forEach((column, columnIndex) => {
      const cell = model.cells.find((candidate) => candidate.rowKey === row.key && candidate.columnKey === column.key && candidate.valueField === column.measure.field);
      if (!cell) return;
      pushTableCell(elements, cell, bounds.left + colWidth * columnIndex, yPosition, colWidth, rowHeight, valueDomain, metadata, "chart-mark chart-pivot-cell");
    });

    rowTotalColumns.forEach((measure, measureIndex) => {
      const cell = model.rowTotals.find((candidate) => candidate.rowKey === row.key && candidate.valueField === measure.field);
      if (!cell) return;
      pushTableCell(elements, cell, bounds.left + colWidth * (model.columns.length + measureIndex), yPosition, colWidth, rowHeight, valueDomain, metadata, "chart-mark chart-pivot-cell chart-s2-total-cell");
    });
  });

  if (model.columnTotals.length > 0) {
    const yPosition = bounds.top + rowHeight * model.rows.length;
    elements.push({
      kind: "rect",
      id: sceneId(mark.id, "column-total-row-header"),
      x: 12,
      y: yPosition,
      width: bounds.left - 18,
      height: rowHeight - 6,
      fill: "hsl(var(--muted))",
      stroke: "hsl(var(--border))",
      radius: 5,
      label: "Column totals",
      className: "chart-mark chart-s2-header chart-s2-total-header",
      tableRowKey: "Total",
      tableSection: "column-total",
      ...metadata,
    });
    elements.push({
      kind: "text",
      id: sceneId(mark.id, "column-total-row-label"),
      x: bounds.left - 24,
      y: yPosition + rowHeight / 2 + 4,
      text: "Total",
      anchor: "end",
      className: "chart-axis-label",
      tableRowKey: "Total",
      tableSection: "column-total",
      ...metadata,
    });

    model.columns.forEach((column, columnIndex) => {
      const cell = model.columnTotals.find((candidate) => candidate.columnKey === column.key && candidate.valueField === column.measure.field);
      if (!cell) return;
      pushTableCell(elements, cell, bounds.left + colWidth * columnIndex, yPosition, colWidth, rowHeight, valueDomain, metadata, "chart-mark chart-pivot-cell chart-s2-total-cell");
    });

    if (model.grandTotal && rowTotalColumns.length > 0) {
      pushTableCell(elements, model.grandTotal, bounds.left + colWidth * model.columns.length, yPosition, colWidth, rowHeight, valueDomain, metadata, "chart-mark chart-pivot-cell chart-s2-total-cell chart-s2-grand-total");
    }
  }

  return scene(spec, elements, []);
}

function pushTableCell(
  elements: SceneElement[],
  cell: SemanticTableCell,
  x: number,
  y: number,
  width: number,
  height: number,
  domain: [number, number],
  metadata: ReturnType<typeof tableMetadata>,
  className: string,
) {
  const intensity = (cell.value - domain[0]) / (domain[1] - domain[0] || 1);
  const fill = cell.section === "body" ? CHART_PALETTE[Math.round(clamp(intensity, 0, 1) * (CHART_PALETTE.length - 1))] : "hsl(var(--muted))";
  const cellMetadata = {
    tableCellId: cell.id,
    tableRowKey: cell.rowKey,
    tableColumnKey: cell.columnKey,
    tableValueField: cell.valueField,
    tableSection: cell.section,
    ...metadata,
  };

  elements.push({
    kind: "rect",
    id: sceneId(cell.id, "rect"),
    x,
    y,
    width: width - 6,
    height: height - 6,
    fill,
    stroke: cell.section === "body" ? undefined : "hsl(var(--border))",
    opacity: cell.section === "body" ? 0.74 : 0.9,
    radius: 5,
    label: `${cell.rowLabels.join(" / ")} ${cell.columnLabels.join(" / ")}: ${cell.formattedValue}`,
    className,
    ...cellMetadata,
  });
  elements.push({
    kind: "text",
    id: sceneId(cell.id, "value"),
    x: x + width / 2 - 3,
    y: y + height / 2 + 4,
    text: cell.formattedValue,
    anchor: "middle",
    className: "chart-s2-value-label",
    ...cellMetadata,
  });
}

function tableMetadata(model: SemanticTableModel) {
  return {
    tableHierarchy: model.hierarchyType,
    tableInteractions: model.interactions.map((interaction) => interaction.type).join(","),
    tableTotals: `row:${model.totals.row},column:${model.totals.column},grand:${String(model.totals.grand)}`,
    tableSortState: tableSortState(model),
    tableDrillPath: tableDrillPath(model),
  };
}

function compactLabel(label: string, maxLength: number): string {
  return label.length <= maxLength ? label : `${label.slice(0, Math.max(1, maxLength - 3))}...`;
}
