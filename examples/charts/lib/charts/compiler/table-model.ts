import { formatValue, readField, toLabel, toNumber } from "../format";
import type {
  ChartPrimitive,
  ChartSpec,
  Datum,
  MarkSpec,
  ReducerKind,
  TableFieldSpec,
  TableDrillSpec,
  TableFilterSpec,
  TableHierarchyKind,
  TableInteractionSpec,
  TableMeasureSpec,
  TableSheetSpec,
  TableSortSpec,
  TableTotalsSpec,
} from "../spec";
import { sceneId } from "./shared";

export type TableSection = "body" | "row-header" | "column-header" | "row-total" | "column-total" | "grand-total";

export interface SemanticTableHeader {
  key: string;
  labels: string[];
  fields: TableFieldSpec[];
}

export interface SemanticTableColumn extends SemanticTableHeader {
  baseKey: string;
  measure: TableMeasureSpec;
}

export interface SemanticTableCell {
  id: string;
  rowKey: string;
  columnKey: string;
  rowLabels: string[];
  columnLabels: string[];
  valueField: string;
  value: number;
  formattedValue: string;
  section: TableSection;
  sourceRows: Datum[];
}

export interface SemanticTableModel {
  rowFields: TableFieldSpec[];
  columnFields: TableFieldSpec[];
  valueFields: TableMeasureSpec[];
  filteredRows: Datum[];
  rows: SemanticTableHeader[];
  columns: SemanticTableColumn[];
  cells: SemanticTableCell[];
  rowTotals: SemanticTableCell[];
  columnTotals: SemanticTableCell[];
  grandTotal?: SemanticTableCell;
  hierarchyType: TableHierarchyKind;
  interactions: TableInteractionSpec[];
  totals: Required<TableTotalsSpec>;
  sort: TableSortSpec[];
  drillDown: TableDrillSpec[];
  filterRows: TableFilterSpec[];
}

const DEFAULT_TOTALS: Required<TableTotalsSpec> = {
  row: "none",
  column: "none",
  grand: false,
};

export function buildSemanticTable(spec: ChartSpec, mark: MarkSpec, data: Datum[]): SemanticTableModel {
  const sheet = normalizeTableSheet(spec, mark);
  const sort = sheet.sort ?? [];
  const drillDown = sheet.drillDown ?? [];
  const filterRows = sheet.filterRows ?? [];
  const filteredRows = applyTableFilters(data, filterRows);
  const rowSort = sort.find((entry) => entry.target === "row");
  const columnSort = sort.find((entry) => entry.target === "column");
  const rows = applyTableSort(headersFromFields(filteredRows, sheet.rows, "All rows"), filteredRows, rowSort, "All rows", sheet.values);
  const baseColumns = applyTableSort(headersFromFields(filteredRows, sheet.columns, "Value"), filteredRows, columnSort, "Value", sheet.values);
  const columns = expandMeasureColumns(baseColumns, sheet.values);
  const cells = aggregateTableCells(filteredRows, rows, columns, sheet.values);
  const totals = { ...DEFAULT_TOTALS, ...sheet.totals };
  const rowTotals = totals.row === "right" ? aggregateRowTotals(filteredRows, rows, sheet.values) : [];
  const columnTotals = totals.column === "bottom" ? aggregateColumnTotals(filteredRows, columns) : [];
  const grandTotal = totals.grand ? aggregateGrandTotal(filteredRows, sheet.values[0]) : undefined;

  return {
    rowFields: sheet.rows,
    columnFields: sheet.columns,
    valueFields: sheet.values,
    filteredRows,
    rows,
    columns,
    cells,
    rowTotals,
    columnTotals,
    grandTotal,
    hierarchyType: sheet.hierarchyType ?? "grid",
    interactions: enabledInteractions(sheet.interactions, sort, drillDown),
    totals,
    sort,
    drillDown,
    filterRows,
  };
}

export function aggregateTableCells(data: Datum[], rows: SemanticTableHeader[], columns: SemanticTableColumn[], values: TableMeasureSpec[]): SemanticTableCell[] {
  const groups = rowsByTableAddress(data, rows, columns);

  return rows.flatMap((row) =>
    columns.map((column) => {
      const sourceRows = groups.get(groupKey(row.key, column.baseKey)) ?? [];
      const measure = column.measure ?? values[0];
      const value = reduceMeasure(sourceRows, measure);
      return makeCell("body", row, column, measure, value, sourceRows);
    }),
  );
}

export function applyTableFilters(data: Datum[], filterRows: TableFilterSpec[]): Datum[] {
  if (filterRows.length === 0) return data;
  return data.filter((datum) => filterRows.every((filter) => matchesTableFilter(datum, filter)));
}

function normalizeTableSheet(spec: ChartSpec, mark: MarkSpec): TableSheetSpec {
  if (spec.table) {
    return {
      ...spec.table,
      values: spec.table.values.length > 0 ? spec.table.values : fallbackMeasures(mark),
    };
  }

  return {
    rows: mark.encoding.y ? [fieldFromEncoding(mark.encoding.y)] : [],
    columns: mark.encoding.x ? [fieldFromEncoding(mark.encoding.x)] : [],
    values: fallbackMeasures(mark),
    hierarchyType: "grid",
    totals: DEFAULT_TOTALS,
    interactions: [{ type: "cell-hover" }],
    filterRows: [],
  };
}

function fallbackMeasures(mark: MarkSpec): TableMeasureSpec[] {
  const value = mark.encoding.color ?? mark.encoding.size ?? mark.encoding.y;
  return value ? [{ field: value.field, label: value.label, format: value.format, reducer: "sum" }] : [{ field: "value", label: "Value", reducer: "count" }];
}

function fieldFromEncoding(field: { field: string; label?: string; type?: TableFieldSpec["type"] }): TableFieldSpec {
  return { field: field.field, label: field.label, type: field.type };
}

function headersFromFields(data: Datum[], fields: TableFieldSpec[], fallbackLabel: string): SemanticTableHeader[] {
  if (fields.length === 0) {
    return [{ key: fallbackLabel, labels: [fallbackLabel], fields: [] }];
  }

  const seen = new Map<string, SemanticTableHeader>();
  data.forEach((datum) => {
    const labels = fields.map((field) => valueLabel(readField(datum, field.field)));
    const key = labels.join(" / ");
    if (!seen.has(key)) {
      seen.set(key, { key, labels, fields });
    }
  });

  return Array.from(seen.values());
}

export function applyTableSort(headers: SemanticTableHeader[], data: Datum[], sort: TableSortSpec | undefined, fallbackLabel: string, measures: TableMeasureSpec[]): SemanticTableHeader[] {
  if (!sort) return headers;
  return [...headers].sort((left, right) => compareHeaders(left, right, data, sort, fallbackLabel, measures));
}

export function compareHeaders(left: SemanticTableHeader, right: SemanticTableHeader, data: Datum[], sort: TableSortSpec, fallbackLabel: string, measures: TableMeasureSpec[]): number {
  const direction = sort.order === "desc" ? -1 : 1;
  const primary = compareValues(
    sortableHeaderValue(left, data, sort, fallbackLabel, measures),
    sortableHeaderValue(right, data, sort, fallbackLabel, measures),
  );

  return primary === 0 ? left.key.localeCompare(right.key) : primary * direction;
}

function expandMeasureColumns(columns: SemanticTableHeader[], values: TableMeasureSpec[]): SemanticTableColumn[] {
  const hasMultipleMeasures = values.length > 1;
  return columns.flatMap((column) =>
    values.map((measure) => ({
      ...column,
      key: hasMultipleMeasures ? `${column.key} / ${measureLabel(measure)}` : column.key,
      labels: hasMultipleMeasures ? [...column.labels, measureLabel(measure)] : column.labels,
      baseKey: column.key,
      measure,
    })),
  );
}

function rowsByTableAddress(data: Datum[], rows: SemanticTableHeader[], columns: SemanticTableColumn[]): Map<string, Datum[]> {
  const groups = new Map<string, Datum[]>();
  data.forEach((datum) => {
    const row = rows.find((header) => header.key === keyForDatum(datum, headerFields(header), "All rows"));
    const column = columns.find((header) => header.baseKey === keyForDatum(datum, headerFields(header), "Value"));
    if (!row || !column) return;
    const key = groupKey(row.key, column.baseKey);
    groups.set(key, [...(groups.get(key) ?? []), datum]);
  });
  return groups;
}

function aggregateRowTotals(data: Datum[], rows: SemanticTableHeader[], values: TableMeasureSpec[]): SemanticTableCell[] {
  return rows.flatMap((row) =>
    values.map((measure) => {
      const sourceRows = data.filter((datum) => keyForDatum(datum, row.fields, "All rows") === row.key);
      const column: SemanticTableColumn = { key: `Total / ${measureLabel(measure)}`, labels: ["Total"], fields: [], baseKey: "Total", measure };
      return makeCell("row-total", row, column, measure, reduceMeasure(sourceRows, measure), sourceRows);
    }),
  );
}

function aggregateColumnTotals(data: Datum[], columns: SemanticTableColumn[]): SemanticTableCell[] {
  const totalRow: SemanticTableHeader = { key: "Total", labels: ["Total"], fields: [] };
  return columns.map((column) => {
    const sourceRows = data.filter((datum) => keyForDatum(datum, headerFields(column), "Value") === column.baseKey);
    return makeCell("column-total", totalRow, column, column.measure, reduceMeasure(sourceRows, column.measure), sourceRows);
  });
}

function aggregateGrandTotal(data: Datum[], measure: TableMeasureSpec): SemanticTableCell {
  const totalRow: SemanticTableHeader = { key: "Total", labels: ["Total"], fields: [] };
  const totalColumn: SemanticTableColumn = { key: "Grand total", labels: ["Grand total"], fields: [], baseKey: "Grand total", measure };
  return makeCell("grand-total", totalRow, totalColumn, measure, reduceMeasure(data, measure), data);
}

function sortableHeaderValue(header: SemanticTableHeader, data: Datum[], sort: TableSortSpec, fallbackLabel: string, measures: TableMeasureSpec[]): ChartPrimitive {
  if (sort.valueField) {
    const measure = measures.find((candidate) => candidate.field === sort.valueField) ?? { field: sort.valueField, reducer: "sum" as const };
    return reduceMeasure(headerSourceRows(data, header, fallbackLabel), measure);
  }

  if (sort.field) {
    const fieldIndex = header.fields.findIndex((field) => field.field === sort.field);
    if (fieldIndex >= 0) return header.labels[fieldIndex] ?? header.key;
  }

  return header.key;
}

function headerSourceRows(data: Datum[], header: SemanticTableHeader, fallbackLabel: string): Datum[] {
  return data.filter((datum) => keyForDatum(datum, header.fields, fallbackLabel) === header.key);
}

function makeCell(section: TableSection, row: SemanticTableHeader, column: SemanticTableColumn, measure: TableMeasureSpec, value: number, sourceRows: Datum[]): SemanticTableCell {
  return {
    id: sceneId("s2", section, row.key, column.key, measure.field),
    rowKey: row.key,
    columnKey: column.key,
    rowLabels: row.labels,
    columnLabels: column.labels,
    valueField: measure.field,
    value,
    formattedValue: formatValue(value, measure.format),
    section,
    sourceRows,
  };
}

function reduceMeasure(rows: Datum[], measure: TableMeasureSpec): number {
  const reducer: ReducerKind = measure.reducer ?? "sum";
  if (reducer === "count") return rows.length;

  const values = rows.map((row) => toNumber(readField(row, measure.field))).filter(Number.isFinite);
  if (values.length === 0) return 0;
  if (reducer === "mean") return values.reduce((sum, value) => sum + value, 0) / values.length;
  if (reducer === "min") return Math.min(...values);
  if (reducer === "max") return Math.max(...values);
  return values.reduce((sum, value) => sum + value, 0);
}

function enabledInteractions(interactions: TableInteractionSpec[] | undefined, sort: TableSortSpec[], drillDown: TableDrillSpec[]): TableInteractionSpec[] {
  const base = interactions ?? [{ type: "cell-hover" }];
  const withState = base.map((interaction) => {
    if (interaction.type === "sort-header" && sort.length > 0 && !interaction.sort) return { ...interaction, sort: sort[0] };
    if (interaction.type === "drill-down" && drillDown.length > 0 && !interaction.drill) return { ...interaction, drill: drillDown[0] };
    return interaction;
  });

  return ensureTableDrillInteraction(ensureTableSortInteraction(withState, sort), drillDown)
    .filter((interaction) => interaction.enabled !== false);
}

function ensureTableSortInteraction(interactions: TableInteractionSpec[], sort: TableSortSpec[]): TableInteractionSpec[] {
  if (sort.length === 0 || interactions.some((interaction) => interaction.type === "sort-header")) return interactions;
  return [...interactions, { type: "sort-header", sort: sort[0] }];
}

function ensureTableDrillInteraction(interactions: TableInteractionSpec[], drillDown: TableDrillSpec[]): TableInteractionSpec[] {
  if (drillDown.length === 0 || interactions.some((interaction) => interaction.type === "drill-down")) return interactions;
  return [...interactions, { type: "drill-down", drill: drillDown[0] }];
}

export function tableSortState(model: SemanticTableModel): string {
  return model.sort.map((sort) => `${sort.target}:${sort.field ?? sort.valueField ?? "value"}:${sort.order ?? "asc"}`).join("|") || "none";
}

export function tableDrillPath(model: SemanticTableModel): string {
  return model.drillDown.map((drill) => `${drill.field}=${toLabel(drill.value)}>${drill.path.map(toLabel).join("/")}`).join("|") || "none";
}

export function tableFilterState(model: SemanticTableModel): string {
  return model.filterRows.map(formatTableFilter).join("|") || "none";
}

function formatTableFilter(filter: TableFilterSpec): string {
  const parts = [filter.field];
  if (filter.equals !== undefined) parts.push(`eq=${toLabel(filter.equals)}`);
  if (filter.min !== undefined) parts.push(`min=${filter.min}`);
  if (filter.max !== undefined) parts.push(`max=${filter.max}`);
  return parts.join(":");
}

function matchesTableFilter(datum: Datum, filter: TableFilterSpec): boolean {
  const value = readField(datum, filter.field);
  if (filter.equals !== undefined && toLabel(value) !== toLabel(filter.equals)) return false;
  if (filter.min !== undefined && toNumber(value) < filter.min) return false;
  if (filter.max !== undefined && toNumber(value) > filter.max) return false;
  return true;
}

function compareValues(left: ChartPrimitive, right: ChartPrimitive): number {
  if (typeof left === "number" && typeof right === "number") return left - right;
  return toLabel(left).localeCompare(toLabel(right));
}

function keyForDatum(datum: Datum, fields: TableFieldSpec[], fallbackLabel: string): string {
  if (fields.length === 0) return fallbackLabel;
  return fields.map((field) => valueLabel(readField(datum, field.field))).join(" / ");
}

function headerFields(header: SemanticTableHeader): TableFieldSpec[] {
  return header.fields;
}

function groupKey(rowKey: string, columnKey: string): string {
  return `${rowKey}\u001f${columnKey}`;
}

function valueLabel(value: ChartPrimitive): string {
  return toLabel(value) || "(blank)";
}

function measureLabel(measure: TableMeasureSpec): string {
  return measure.label ?? measure.field;
}
