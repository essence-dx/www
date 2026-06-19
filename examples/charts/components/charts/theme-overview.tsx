import { getChartBySlug } from "../../lib/charts";
import { ChartFrame } from "./chart-frame";

const swatches = [
  ["Blue", "chart-blue"],
  ["Green", "chart-green"],
  ["Amber", "chart-amber"],
  ["Rose", "chart-rose"],
  ["Violet", "chart-violet"],
  ["Cyan", "chart-cyan"],
  ["Lime", "chart-lime"],
  ["Orange", "chart-orange"],
] as const;

export function ThemeOverview() {
  return (
    <div className="charts-page-stack" data-dx-route="theme">
      <header className="charts-page-heading">
        <p className="charts-kicker">Theme</p>
        <h1>Chart style lives in DX tokens first.</h1>
        <p className="charts-lead">
          Marks refer to theme variables and DX Style-owned classes. The chart kernel does not smuggle random hex colors
          into renderer code.
        </p>
      </header>
      <section className="charts-theme-layout">
        <div className="charts-swatch-panel" aria-label="Chart palette">
          {swatches.map(([label, token]) => (
            <div className="charts-swatch-row" key={token}>
              <span className="charts-swatch" data-token={token} aria-hidden="true" />
              <strong>{label}</strong>
              <code>--{token}</code>
            </div>
          ))}
        </div>
        <ChartFrame item={getChartBySlug("gauge-source-readiness")} />
      </section>
    </div>
  );
}
