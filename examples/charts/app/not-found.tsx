import { ChartRouteBoundary } from "../components/charts/route-boundary";
import { ChartSiteShell } from "../components/charts/chart-site-shell";

export default function ChartsNotFoundBoundary() {
  return (
    <ChartSiteShell active="gallery">
      <ChartRouteBoundary kind="not-found" />
    </ChartSiteShell>
  );
}
