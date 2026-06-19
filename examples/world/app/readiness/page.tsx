import { buildWorldStatus } from "../../lib/world/status";

export const metadata = {
  title: "World Readiness",
  description: "Redacted readiness status for DX WWW world integrations.",
} as const;

const status = buildWorldStatus();

export default function ReadinessPage() {
  return (
    <main className="dx-template dx-shell dx-page-shell dx-stack world-shell" data-dx-route="/readiness">
      <header className="world-page-heading">
        <p className="world-kicker">Readiness</p>
        <h1>Env, receipt, and live validation status.</h1>
        <p className="dx-muted world-copy">
          Values are redacted by design. Missing env keys keep a provider in preview mode until the read-only
          TypeScript connection runner proves the provider and writes a replayable receipt.
        </p>
        <nav className="world-actions" aria-label="World route links">
          <a className="dx-secondary-action world-button world-button-secondary" href="/">Home</a>
          <a className="dx-secondary-action world-button world-button-secondary" href="/integrations">Integrations</a>
        </nav>
      </header>

      <section className="dx-grid world-metrics" aria-label="Readiness metrics">
        <article className="dx-card world-metric">
          <span>{status.totals.liveReady}</span>
          <p>live-ready</p>
        </article>
        <article className="dx-card world-metric">
          <span>{status.totals.envReady}</span>
          <p>env-ready</p>
        </article>
        <article className="dx-card world-metric">
          <span>{status.totals.previewOnly}</span>
          <p>preview-only</p>
        </article>
        <article className="dx-card world-metric">
          <span>{status.totals.missingEnv}</span>
          <p>missing env keys</p>
        </article>
        <article className="dx-card world-metric">
          <span>{status.connections.probeCount}</span>
          <p>live probes</p>
        </article>
      </section>

      <section className="dx-card world-live-runner" aria-label="Live connection runner">
        <div>
          <p className="world-kicker">Live runner</p>
          <h2>Read-only provider probes are wired.</h2>
          <p className="dx-muted world-copy">
            Run the TypeScript benchmark to connect available providers, write the redacted receipt, and keep
            generated `.dx` proof out of source control.
          </p>
        </div>
        <code>{status.connections.runner}</code>
        <code>{status.connections.receiptPath}</code>
      </section>

      <section className="world-status-table" aria-label="Provider readiness table">
        {status.providers.map((provider) => (
          <article className="dx-card world-status-row" key={provider.id}>
            <div>
              <h2>{provider.name}</h2>
              <p>{provider.packageId}</p>
            </div>
            <span>{provider.state}</span>
            <small>{provider.missingEnv.length ? provider.missingEnv.join(", ") : "env contract satisfied"}</small>
          </article>
        ))}
      </section>
    </main>
  );
}
