import { ChartRouteBoundary } from "../components/charts/route-boundary";
import { ChartSiteShell } from "../components/charts/chart-site-shell";

export default function ChartsLoadingBoundary() {
  return (
    <ChartSiteShell active="overview">
      <ChartRouteBoundary kind="loading" />
    </ChartSiteShell>
  );
}
