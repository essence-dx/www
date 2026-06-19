import { chartCatalog, chartRoutes, chartTasks, ecosystemFamilies } from "../../lib/charts";
import { chartCatalogCachePolicy, chartManifestStreamBoundary, chartManifestStreamFrames } from "./delivery";

export type ChartRuntimeManifest = {
  schema: "dx.charts.runtime_manifest";
  chartSpecCount: number;
  ecosystemFamilyCount: number;
  taskFilterCount: number;
  routeCount: number;
  packageLaneCount: number;
  cachePolicy: typeof chartCatalogCachePolicy;
  streamFrameCount: number;
  streamBoundary: ReturnType<typeof chartManifestStreamBoundary>;
  noNpmRuntime: true;
};

export const chartRuntimeManifest: ChartRuntimeManifest = {
  schema: "dx.charts.runtime_manifest",
  chartSpecCount: chartCatalog.length,
  ecosystemFamilyCount: ecosystemFamilies.length,
  taskFilterCount: chartTasks.length,
  routeCount: chartRoutes.length,
  packageLaneCount: ecosystemFamilies.length,
  cachePolicy: chartCatalogCachePolicy,
  streamFrameCount: chartManifestStreamFrames.length,
  streamBoundary: chartManifestStreamBoundary(),
  noNpmRuntime: true,
};

export function chartRuntimeRouteSlugs() {
  return chartRoutes.map((route) => route.id);
}
