import { getChartBySlug } from "../../lib/charts";
import { ChartFrame } from "./chart-frame";

export function PlaygroundPreview() {
  const item = getChartBySlug("histogram-quality");

  return (
    <div className="charts-page-stack" data-dx-route="playground">
      <header className="charts-page-heading">
        <p className="charts-kicker">Playground</p>
        <h1>Read-only spec preview for the first production slice.</h1>
        <p className="charts-lead">
          The live editor is intentionally not claimed yet. This page shows the current spec, data shape, and compiled
          output so the route is useful without pretending to execute arbitrary user code.
        </p>
      </header>
      <section className="charts-playground-layout">
        <div className="charts-code-panel">
          <p className="charts-kicker">Spec source</p>
          <pre>
            <code>{JSON.stringify(item.spec, null, 2)}</code>
          </pre>
        </div>
        <ChartFrame item={item} />
      </section>
    </div>
  );
}
