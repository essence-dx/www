import { ChartGallery } from "../../components/charts/chart-gallery";
import { ChartSiteShell } from "../../components/charts/chart-site-shell";

export default function ChartsGalleryPage() {
  return (
    <ChartSiteShell active="gallery">
      <ChartGallery />
    </ChartSiteShell>
  );
}
