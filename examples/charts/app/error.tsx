import { ChartRouteBoundary } from "../components/charts/route-boundary";
import { ChartSiteShell } from "../components/charts/chart-site-shell";

export default function ChartsErrorBoundary() {
  return (
    <ChartSiteShell active="overview">
      <ChartRouteBoundary kind="error" />
    </ChartSiteShell>
  );
}
