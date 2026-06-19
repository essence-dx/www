import { extent, readField, uniqueLabels } from "../format";
import { mobileSceneMetadata } from "../mobile-model";
import { createBandScale, createLinearScale, createOrdinalScale, createPointScale } from "../scales";
import type { ChartSpec, FieldEncoding } from "../spec";
import type { ChartScene, SceneElement } from "../scene";
import { applyMarkTransforms } from "../transforms";
import {
  addAreas,
  addBars,
  addBoxplots,
  addBullets,
  addGridAndAxes,
  addHeatmap,
  addLines,
  addPoints,
  addRules,
  type CartesianXScale,
  yValuesForDomain,
} from "./cartesian-marks";
import { coordinateSceneMetadata, emptyScene, makeLegend, plotBounds, withPadding } from "./shared";

const bandedMarks = new Set(["bar", "heatmap", "boxplot", "bullet"]);

export function compileCartesian(spec: ChartSpec): ChartScene {
  const bounds = plotBounds(spec, withPadding(spec.padding));
  const primary = spec.marks[0];
  const x = primary.encoding.x;
  const y = primary.encoding.y ?? primary.encoding.median;
  const colorField = primary.encoding.color ?? primary.encoding.series;

  if (!x || !y) {
    return emptyScene(spec);
  }

  const rowsByMark = new Map(spec.marks.map((mark) => [mark.id, applyMarkTransforms(spec.data, mark)]));
  const primaryRows = rowsByMark.get(primary.id) ?? spec.data;
  const xScale = createXScale(primary.type, x, primaryRows, bounds.left, bounds.right);
  const yDomain = extent(spec.marks.flatMap((mark) => {
    const rows = rowsByMark.get(mark.id) ?? spec.data;
    return rows.flatMap((datum) => yValuesForDomain(mark, datum, y));
  }));
  const yScale = createLinearScale(yDomain, [bounds.bottom, bounds.top]);
  const colorDomain = colorField ? uniqueLabels(primaryRows.map((datum) => readField(datum, colorField.field))) : [];
  const colorScale = createOrdinalScale(colorDomain);
  const elements: SceneElement[] = [];
  const mobileMetadata = mobileSceneMetadata(spec.mobile);

  if (spec.axes?.length !== 0) {
    addGridAndAxes(elements, spec, bounds, x, y, xScale, yScale.ticks(5));
  }

  for (const mark of spec.marks) {
    const rows = rowsByMark.get(mark.id) ?? spec.data;
    if (mark.type === "bar") addBars(elements, rows, mark, x, y, xScale, yScale, colorField, colorScale);
    if (mark.type === "line") addLines(elements, rows, mark, x, y, xScale, yScale, colorField, colorScale);
    if (mark.type === "area") addAreas(elements, rows, mark, x, y, xScale, yScale, colorField, colorScale, bounds.bottom);
    if (mark.type === "point") addPoints(elements, rows, mark, x, y, xScale, yScale, colorField, colorScale);
    if (mark.type === "rule") addRules(elements, rows, mark, mark.encoding.y ?? y, yScale, bounds);
    if (mark.type === "heatmap") addHeatmap(elements, rows, mark, x, y, bounds);
    if (mark.type === "boxplot") addBoxplots(elements, rows, mark, x, y, xScale, yScale);
    if (mark.type === "bullet") addBullets(elements, rows, mark, x, y, xScale, yScale, bounds);
  }

  return {
    id: spec.id,
    title: spec.title,
    description: spec.description,
    width: spec.width,
    height: spec.height,
    elements: elements.map((element) => ({ ...element, ...coordinateSceneMetadata(spec), ...mobileMetadata })),
    legend: makeLegend(colorDomain, colorScale.map),
    summary: `${spec.title}: ${primaryRows.length} rows, ${spec.marks.map((mark) => mark.type).join(", ")} marks.`,
  };
}

function createXScale(kind: string, x: FieldEncoding, rows: ChartSpec["data"], left: number, right: number): CartesianXScale {
  if (bandedMarks.has(kind)) {
    return createBandScale(uniqueLabels(rows.map((datum) => readField(datum, x.field))), [left, right]);
  }

  if (x.type === "quantitative" || x.type === "temporal") {
    return createLinearScale(extent(rows.map((datum) => readField(datum, x.field))), [left, right]);
  }

  return createPointScale(uniqueLabels(rows.map((datum) => readField(datum, x.field))), [left, right]);
}
