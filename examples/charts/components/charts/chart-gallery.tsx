import { chartCatalog, chartTasks, taskAnchor } from "../../lib/charts";
import { ChartFrame } from "./chart-frame";

export function ChartGallery() {
  return (
    <div className="charts-page-stack" data-dx-route="charts">
      <header className="charts-page-heading">
        <p className="charts-kicker">Chart gallery</p>
        <h1>Specs grouped by analytical task.</h1>
        <p className="charts-lead">
          Every card is backed by local data and a source-owned chart spec. The broader AntV ecosystem is represented as
          DX-native families with a consistent visual language.
        </p>
      </header>

      <nav className="charts-filter-row" aria-label="Chart task filters">
        {chartTasks.map((task) => (
          <a href={taskAnchor(task.id)} key={task.id}>
            {task.label}
          </a>
        ))}
      </nav>

      {chartTasks.map((task) => {
        const charts = chartCatalog.filter((item) => item.task === task.id);
        if (charts.length === 0) return null;

        return (
          <section className="charts-section" id={`task-${task.id}`} key={task.id} aria-labelledby={`${task.id}-title`}>
            <div className="charts-section-heading">
              <p className="charts-kicker">{task.description}</p>
              <h2 id={`${task.id}-title`}>{task.label}</h2>
            </div>
            <div className="charts-gallery-grid">
              {charts.map((item) => (
                <article className="charts-gallery-card" key={item.slug}>
                  <ChartFrame item={item} compact />
                  <div className="charts-card-copy">
                    <span>{item.family}</span>
                    <h3>{item.title}</h3>
                    <p>{item.whenToUse}</p>
                    <dl>
                      <div>
                        <dt>Data</dt>
                        <dd>{item.dataShape}</dd>
                      </div>
                      <div>
                        <dt>Avoid</dt>
                        <dd>{item.avoidWhen}</dd>
                      </div>
                    </dl>
                  </div>
                </article>
              ))}
            </div>
          </section>
        );
      })}
    </div>
  );
}
