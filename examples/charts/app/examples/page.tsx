import { ExamplesOverview } from "../../components/charts/examples-overview";
import { ChartSiteShell } from "../../components/charts/chart-site-shell";

export default function ChartsExamplesPage() {
  return (
    <ChartSiteShell active="examples">
      <ExamplesOverview />
    </ChartSiteShell>
  );
}
