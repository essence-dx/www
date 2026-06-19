import type { ChartSpec, Datum, TableDrillSpec, TableFieldSpec, TableFilterSpec, TableHierarchyKind, TableInteractionSpec, TableMeasureSpec, TableSortSpec, TableTotalsSpec } from "../spec";

export type S2PivotSheetPreset = {
  id: string;
  title: string;
  description: string;
  data: Datum[];
  rows: TableFieldSpec[];
  columns: TableFieldSpec[];
  values: TableMeasureSpec[];
  hierarchyType?: TableHierarchyKind;
  totals?: TableTotalsSpec;
  interactions?: TableInteractionSpec[];
  sort?: TableSortSpec[];
  drillDown?: TableDrillSpec[];
  filterRows?: TableFilterSpec[];
  width?: number;
  height?: number;
};

export function s2PivotSheet(config: S2PivotSheetPreset): ChartSpec {
  const firstRow = config.rows[0];
  const firstColumn = config.columns[0];
  const firstValue = config.values[0];

  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: "table",
    family: "S2",
    width: config.width ?? 640,
    height: config.height ?? 380,
    data: config.data,
    table: {
      rows: config.rows,
      columns: config.columns,
      values: config.values,
      hierarchyType: config.hierarchyType ?? "grid",
      totals: config.totals ?? { row: "right", column: "bottom", grand: true },
      interactions: config.interactions ?? defaultInteractions(config.sort, config.drillDown),
      sort: config.sort ?? [],
      drillDown: config.drillDown ?? [],
      filterRows: config.filterRows ?? [],
    },
    marks: [
      {
        id: `${config.id}-pivot`,
        type: "pivot",
        encoding: {
          x: { field: firstColumn?.field ?? "column", type: firstColumn?.type ?? "ordinal" },
          y: { field: firstRow?.field ?? "row", type: firstRow?.type ?? "nominal" },
          color: { field: firstValue?.field ?? "value", type: "quantitative" },
        },
      },
    ],
  };
}

function defaultInteractions(sort: TableSortSpec[] | undefined, drillDown: TableDrillSpec[] | undefined): TableInteractionSpec[] {
  const interactions: TableInteractionSpec[] = [{ type: "cell-hover" }, { type: "brush-selection" }, { type: "sort-header", sort: sort?.[0] }];
  if (drillDown && drillDown.length > 0) interactions.push({ type: "drill-down", drill: drillDown[0] });
  return interactions;
}
