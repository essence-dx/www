import { DocsOverview } from "../../components/charts/docs-overview";
import { ChartSiteShell } from "../../components/charts/chart-site-shell";

export default function ChartsDocsPage() {
  return (
    <ChartSiteShell active="docs">
      <DocsOverview />
    </ChartSiteShell>
  );
}
