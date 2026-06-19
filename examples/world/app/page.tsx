import { getWorldSummary, worldCategories } from "../lib/world/registry";

export const metadata = {
  title: "DX WWW World Connections",
  description: "Provider connection lab for the DX WWW ecosystem.",
} as const;

const summary = getWorldSummary();

export default function WorldHomePage() {
  return (
    <main className="dx-template dx-shell dx-page-shell dx-stack world-shell" data-dx-route="/" data-world-example="home">
      <header className="world-header">
        <p className="world-kicker">DX WWW world lab</p>
        <h1>Connect the production web without hardcoding the framework.</h1>
        <p className="dx-muted world-copy">
          This example tracks the first provider targets from every WORLD.md category using typed contracts,
          redacted env status, and receipt expectations.
        </p>
        <nav className="world-actions" aria-label="World example routes">
          <a className="dx-button dx-action world-button" href="/integrations">Review integrations</a>
          <a className="dx-secondary-action world-button world-button-secondary" href="/readiness">Readiness status</a>
          <a className="dx-secondary-action world-button world-button-secondary" href="/api/world/status">Status API</a>
        </nav>
      </header>

      <section className="dx-grid world-metrics" aria-label="World integration metrics">
        <article className="dx-card world-metric">
          <span>{summary.categoryCount}</span>
          <p>categories</p>
        </article>
        <article className="dx-card world-metric">
          <span>{summary.providerCount}</span>
          <p>top provider targets</p>
        </article>
        <article className="dx-card world-metric">
          <span>{summary.requiredEnvCount}</span>
          <p>declared env keys</p>
        </article>
        <article className="dx-card world-metric">
          <span>{summary.receiptCount}</span>
          <p>receipt lanes</p>
        </article>
      </section>

      <section className="dx-grid world-grid" aria-label="World categories">
        {worldCategories.map((category) => (
          <article className="dx-card world-card" key={category.id}>
            <p className="world-card-index">{category.providers.length} targets</p>
            <h2>{category.title}</h2>
            <p>{category.purpose}</p>
            <ul className="world-provider-list">
              {category.providers.map((provider) => (
                <li key={provider.id}>
                  <span>{provider.name}</span>
                  <small>{provider.validation}</small>
                </li>
              ))}
            </ul>
          </article>
        ))}
      </section>
    </main>
  );
}
