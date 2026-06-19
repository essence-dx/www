import { Icon } from "../icons/icon";

export type ChartRouteBoundaryKind = "loading" | "error" | "not-found";

type ChartRouteBoundaryProps = {
  kind: ChartRouteBoundaryKind;
};

const boundaryCopy: Record<
  ChartRouteBoundaryKind,
  {
    kicker: string;
    title: string;
    text: string;
    icon: "charts:activity" | "charts:target" | "charts:terminal";
    href: string;
    action: string;
  }
> = {
  loading: {
    kicker: "Preparing chart surface",
    title: "Loading DX Charts",
    text: "The source-owned catalog is preparing its route data, theme tokens, and chart runtime manifest.",
    icon: "charts:activity",
    href: "/charts",
    action: "Open gallery",
  },
  error: {
    kicker: "Chart route boundary",
    title: "This chart route needs a clean reload",
    text: "The route shell is still available while the chart surface recovers.",
    icon: "charts:terminal",
    href: "/",
    action: "Return home",
  },
  "not-found": {
    kicker: "Unknown chart route",
    title: "That chart route is not in the DX catalog",
    text: "The product routes and source-proof routes are intentionally separated in the local route map.",
    icon: "charts:target",
    href: "/charts",
    action: "Browse catalog",
  },
};

export function ChartRouteBoundary({ kind }: ChartRouteBoundaryProps) {
  const copy = boundaryCopy[kind];

  return (
    <section className="charts-page-stack" data-dx-route-boundary={kind} aria-live={kind === "loading" ? "polite" : "assertive"}>
      <div className="charts-page-heading">
        <p className="charts-kicker">{copy.kicker}</p>
        <h1>{copy.title}</h1>
        <p className="charts-lead">{copy.text}</p>
        <div className="charts-actions">
          <a className="charts-secondary-action" href={copy.href}>
            <Icon name={copy.icon} />
            <span>{copy.action}</span>
          </a>
        </div>
      </div>
    </section>
  );
}
