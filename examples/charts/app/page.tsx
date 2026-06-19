import { ChartHome } from "../components/charts/chart-home";
import { ChartSiteShell } from "../components/charts/chart-site-shell";
import { chartRuntimeManifest } from "../server/charts/manifest";

export default function ChartsHomePage() {
  return (
    <ChartSiteShell active="overview">
      <ChartHome manifest={chartRuntimeManifest} />
    </ChartSiteShell>
  );
}
