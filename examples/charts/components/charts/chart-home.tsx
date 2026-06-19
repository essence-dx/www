import { Icon } from "../icons/icon";
import { chartDocsHref, chartGalleryHref, chartTasks, featuredChartSlugs, getChartBySlug, taskHref } from "../../lib/charts";
import { ChartFrame } from "./chart-frame";

type ChartHomeProps = {
  manifest: {
    chartSpecCount: number;
    ecosystemFamilyCount: number;
    taskFilterCount: number;
    packageLaneCount: number;
  };
};

export function ChartHome({ manifest }: ChartHomeProps) {
  const featured = featuredChartSlugs.map((slug) => getChartBySlug(slug));
  const metrics = [
    { label: "Chart specs", value: manifest.chartSpecCount },
    { label: "AntV families", value: manifest.ecosystemFamilyCount },
    { label: "Task filters", value: manifest.taskFilterCount },
    { label: "Package lanes", value: manifest.packageLaneCount },
  ] as const;

  return (
    <div className="charts-page-stack" data-dx-route="overview">
      <section className="charts-hero">
        <div className="charts-hero-copy">
          <p className="charts-kicker">DX WWW chart framework</p>
          <h1>AntV-class visualization ideas, rebuilt as DX-owned source.</h1>
          <p className="charts-lead">
            A grammar-first chart framework for dashboards, data exploration, storytelling, maps, graphs, pivots, and
            recommendation workflows.
          </p>
          <div className="charts-actions">
            <a className="charts-primary-action" href={chartGalleryHref}>
              <Icon name="charts:bar" />
              Open gallery
            </a>
            <a className="charts-secondary-action" href={chartDocsHref}>
              <Icon name="charts:book" />
              Read grammar
            </a>
          </div>
        </div>
        <ChartFrame item={getChartBySlug("line-receipts")} compact />
      </section>

      <section className="charts-metric-grid" aria-label="DX Charts metrics">
        {metrics.map((metric) => (
          <article className="charts-metric" key={metric.label}>
            <strong>{metric.value}</strong>
            <span>{metric.label}</span>
          </article>
        ))}
      </section>

      <section className="charts-section" aria-labelledby="featured-charts">
        <div className="charts-section-heading">
          <p className="charts-kicker">Featured charts</p>
          <h2 id="featured-charts">Real specs compiled by the local chart kernel.</h2>
        </div>
        <div className="charts-preview-grid">
          {featured.map((item) => (
            <ChartFrame item={item} compact key={item.slug} />
          ))}
        </div>
      </section>

      <section className="charts-section" aria-labelledby="task-finder">
        <div className="charts-section-heading">
          <p className="charts-kicker">Choose by task</p>
          <h2 id="task-finder">The gallery is organized like practitioner docs, not marketing cards.</h2>
        </div>
        <div className="charts-task-grid">
          {chartTasks.map((task) => (
            <a className="charts-task-card" href={taskHref(task.id)} key={task.id}>
              <Icon name="charts:target" />
              <strong>{task.label}</strong>
              <span>{task.description}</span>
            </a>
          ))}
        </div>
      </section>
    </div>
  );
}
