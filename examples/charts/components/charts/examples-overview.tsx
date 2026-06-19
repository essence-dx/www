import { getChartBySlug } from "../../lib/charts";
import { ChartFrame } from "./chart-frame";

const categories = [
  { label: "Basic marks", slugs: ["bar-builds", "line-receipts", "scatter-runtime"] },
  { label: "Grammar transforms", slugs: ["stacked-release-work", "normalized-release-share", "dodged-package-modes"] },
  { label: "View composition", slugs: ["faceted-runtime-proof", "heatmap-lanes", "radar-capabilities"] },
  { label: "Components", slugs: ["pivot-readiness", "bullet-readiness", "gauge-source-readiness"] },
  { label: "Ecosystem scenes", slugs: ["graph-ecosystem", "map-usage", "sankey-pipeline"] },
] as const;

export function ExamplesOverview() {
  return (
    <div className="charts-page-stack" data-dx-route="examples">
      <header className="charts-page-heading">
        <p className="charts-kicker">Examples</p>
        <h1>Source previews grouped by capability.</h1>
        <p className="charts-lead">
          These examples mirror the G2 site’s docs-first posture: basic charts, components, relationships, maps, and
          storytelling surfaces are all represented by real data and local specs.
        </p>
      </header>
      {categories.map((category) => (
        <section className="charts-section" key={category.label}>
          <div className="charts-section-heading">
            <p className="charts-kicker">Capability</p>
            <h2>{category.label}</h2>
          </div>
          <div className="charts-preview-grid">
            {category.slugs.map((slug) => {
              const item = getChartBySlug(slug);
              return <ChartFrame item={item} compact key={slug} />;
            })}
          </div>
        </section>
      ))}
    </div>
  );
}
