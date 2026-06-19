import { ChartSiteShell } from "../../components/charts/chart-site-shell";
import { EcosystemOverview } from "../../components/charts/ecosystem-overview";

export default function ChartsEcosystemPage() {
  return (
    <ChartSiteShell active="ecosystem">
      <EcosystemOverview />
    </ChartSiteShell>
  );
}
