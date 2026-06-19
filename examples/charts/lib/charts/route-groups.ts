import { chartRoutes } from "./routes";
import type { ChartRouteId } from "./routes";

export type ChartSourceProofRouteId = "islands" | "state-runtime";
export type ChartGroupedRouteId = ChartRouteId | ChartSourceProofRouteId;
export type ChartRouteGroupId = "product-catalog" | "product-guides" | "source-proof";

export type ChartRouteGroup = {
  id: ChartRouteGroupId;
  label: string;
  routeIds: readonly ChartGroupedRouteId[];
  sourceOwned: true;
};

export const CHART_ROUTE_GROUPS: readonly ChartRouteGroup[] = [
  {
    id: "product-catalog",
    label: "Product catalog",
    routeIds: ["overview", "gallery", "examples", "playground"],
    sourceOwned: true,
  },
  {
    id: "product-guides",
    label: "Product guides",
    routeIds: ["docs", "theme", "ecosystem"],
    sourceOwned: true,
  },
  {
    id: "source-proof",
    label: "Source proof",
    routeIds: ["islands", "state-runtime"],
    sourceOwned: true,
  },
];

export function routeGroupForRoute(routeId: ChartGroupedRouteId): ChartRouteGroup | undefined {
  return CHART_ROUTE_GROUPS.find((group) => group.routeIds.includes(routeId));
}

export function assertChartRouteGroupCoverage(): true {
  const groupedRouteIds = new Set(CHART_ROUTE_GROUPS.flatMap((group) => group.routeIds));
  for (const route of chartRoutes) {
    if (!groupedRouteIds.has(route.id)) {
      throw new Error(`Chart route ${route.id} is not assigned to a source-owned group.`);
    }
  }
  return true;
}
