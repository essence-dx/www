import { Icon } from "../icons/icon";
import { chartCatalog, ecosystemFamilies, taskHref } from "../../lib/charts";

function packageAnchorId(family: string): string {
  return `chart-package-${family.toLowerCase().replace(/[^a-z0-9]+/g, "-")}`;
}

export function EcosystemOverview() {
  const familyCoverage = ecosystemFamilies.map((item) => {
    const specs = chartCatalog.filter((chart) => chart.family === item.family);
    return {
      ...item,
      implementedCount: specs.length,
      packageId: packageAnchorId(item.family),
      parityStatus: specs.length > 0 ? "Source-backed" : "Reference-mapped",
      representativeHref: specs[0] ? taskHref(specs[0].task) : "/docs",
    };
  });
  const implementedPackages = familyCoverage.filter((item) => item.coverageStatus === "Implemented").length;
  const proofBackedPackages = familyCoverage.filter((item) => item.sourceProof && item.interactionProof).length;

  return (
    <div className="charts-page-stack" data-dx-route="ecosystem">
      <header className="charts-page-heading">
        <p className="charts-kicker">Ecosystem map</p>
        <h1>AntV package ideas translated into DX packages.</h1>
        <p className="charts-lead">
          The cloned repositories are provenance. The running example stays DX-native: one package boundary at a time,
          clear source ownership, and practical chart-family coverage.
        </p>
      </header>
      <section className="charts-metric-grid" aria-label="Ecosystem coverage summary">
        <article className="charts-metric">
          <strong>{chartCatalog.length}</strong>
          <span>Implemented specs</span>
        </article>
        <article className="charts-metric">
          <strong>{ecosystemFamilies.length}</strong>
          <span>Mapped packages</span>
        </article>
        <article className="charts-metric">
          <strong>{implementedPackages}</strong>
          <span>Implemented slices</span>
        </article>
        <article className="charts-metric">
          <strong>{proofBackedPackages}</strong>
          <span>Proof-backed cards</span>
        </article>
      </section>
      <section className="charts-ecosystem-grid" aria-label="AntV to DX package map">
        {familyCoverage.map((item) => (
          <article
            aria-describedby={`${item.packageId}-status ${item.packageId}-proof ${item.packageId}-interaction`}
            aria-labelledby={`${item.packageId}-title`}
            className="charts-ecosystem-card"
            data-dx-chart-package-status={item.coverageStatus}
            data-dx-chart-parity-status={item.parityStatus}
            data-dx-chart-source-proof={item.sourceProof}
            id={item.packageId}
            key={item.family}
            tabIndex={0}
          >
            <Icon name="charts:package" />
            <div>
              <p>{item.packageName}</p>
              <h2 id={`${item.packageId}-title`}>{item.family}</h2>
              <span id={`${item.packageId}-status`}>
                {item.coverageStatus}: {item.implementedCount} specs, {item.parityStatus}
              </span>
            </div>
            <dl>
              <div>
                <dt>Reference</dt>
                <dd>{item.source}</dd>
              </div>
              <div>
                <dt>DX boundary</dt>
                <dd>{item.dxBoundary}</dd>
              </div>
              <div id={`${item.packageId}-proof`}>
                <dt>Source proof</dt>
                <dd>{item.sourceProof}</dd>
              </div>
              <div id={`${item.packageId}-interaction`}>
                <dt>Interaction proof</dt>
                <dd>{item.interactionProof}</dd>
              </div>
            </dl>
            <a className="charts-secondary-action" href={item.representativeHref}>
              <Icon name="charts:bar" />
              View coverage
            </a>
          </article>
        ))}
      </section>
    </div>
  );
}
