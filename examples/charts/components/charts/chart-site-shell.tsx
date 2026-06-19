import { Icon } from "../icons/icon";
import { chartHomeHref, chartRoutes } from "../../lib/charts";
import type { ChartRouteId } from "../../lib/charts";

export type ChartSiteShellProps = {
  active: ChartRouteId;
  children: any;
};

type NavIconName = (typeof chartRoutes)[number]["icon"];

export function ChartSiteShell({ active, children }: ChartSiteShellProps) {
  return (
    <main className="charts-site-shell min-h-screen p-6 md:(p-8)" data-dx-surface="charts-framework">
      <header className="charts-topbar">
        <a className="charts-brand" href={chartHomeHref} aria-label="DX Charts overview">
          <span className="charts-brand-mark" aria-hidden="true">
            <Icon name="charts:sparkline" />
          </span>
          <span>
            <strong>DX Charts</strong>
            <small>source-owned visualization grammar</small>
          </span>
        </a>
        <nav className="charts-nav" aria-label="Charts routes">
          {chartRoutes.map((item) => (
            <a className="charts-nav-link" data-active={active === item.id ? "true" : "false"} href={item.href} key={item.id}>
              <NavIcon name={item.icon} />
              <span>{item.label}</span>
            </a>
          ))}
        </nav>
      </header>
      {children}
      <div className="chart-tooltip" data-dx-chart-tooltip="true" role="status" aria-live="polite" />
    </main>
  );
}

function NavIcon({ name }: { name: NavIconName }) {
  if (name === "activity") return <Icon name="charts:activity" />;
  if (name === "bar") return <Icon name="charts:bar" />;
  if (name === "layers") return <Icon name="charts:layers" />;
  if (name === "book") return <Icon name="charts:book" />;
  if (name === "palette") return <Icon name="charts:palette" />;
  if (name === "network") return <Icon name="charts:network" />;
  return <Icon name="charts:terminal" />;
}
