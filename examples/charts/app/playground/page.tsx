import { ChartSiteShell } from "../../components/charts/chart-site-shell";
import { PlaygroundPreview } from "../../components/charts/playground-preview";

export default function ChartsPlaygroundPage() {
  return (
    <ChartSiteShell active="playground">
      <PlaygroundPreview />
    </ChartSiteShell>
  );
}
