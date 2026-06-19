import { CHART_ROUTE_GROUPS, chartCatalog, chartRoutes, chartTasks, ecosystemFamilies } from "../../lib/charts";

export type ChartCatalogCachePolicy = {
  schema: "dx.charts.cache_policy";
  mode: "deterministic-local-source";
  revalidateSeconds: 0;
  refresh: "manual-refresh";
  tags: readonly string[];
  noRemoteCache: true;
};

export type ChartManifestStreamFrame = {
  id: string;
  streamBoundary: "chart-manifest";
  label: string;
  itemCount: number;
};

export const chartCatalogCachePolicy: ChartCatalogCachePolicy = {
  schema: "dx.charts.cache_policy",
  mode: "deterministic-local-source",
  revalidateSeconds: 0,
  refresh: "manual-refresh",
  tags: ["charts", "catalog", "ecosystem", "routes"],
  noRemoteCache: true,
};

export const chartManifestStreamFrames: readonly ChartManifestStreamFrame[] = [
  { id: "routes", streamBoundary: "chart-manifest", label: "Route catalog", itemCount: chartRoutes.length },
  { id: "route-groups", streamBoundary: "chart-manifest", label: "Route groups", itemCount: CHART_ROUTE_GROUPS.length },
  { id: "charts", streamBoundary: "chart-manifest", label: "Chart specs", itemCount: chartCatalog.length },
  { id: "tasks", streamBoundary: "chart-manifest", label: "Task filters", itemCount: chartTasks.length },
  { id: "packages", streamBoundary: "chart-manifest", label: "Package lanes", itemCount: ecosystemFamilies.length },
];

export function chartManifestStreamBoundary() {
  return {
    schema: "dx.charts.stream_boundary",
    streamBoundary: "chart-manifest",
    frameCount: chartManifestStreamFrames.length,
    frameIds: chartManifestStreamFrames.map((frame) => frame.id),
  } as const;
}
