import { ChartSiteShell } from "../../components/charts/chart-site-shell";
import { ThemeOverview } from "../../components/charts/theme-overview";

export default function ChartsThemePage() {
  return (
    <ChartSiteShell active="theme">
      <ThemeOverview />
    </ChartSiteShell>
  );
}
