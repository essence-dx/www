import { worldCategories } from "../../lib/world/registry";

export const metadata = {
  title: "World Integrations",
  description: "Top provider targets grouped by DX WWW integration category.",
} as const;

export default function IntegrationsPage() {
  return (
    <main className="dx-template dx-shell dx-page-shell dx-stack world-shell" data-dx-route="/integrations">
      <header className="world-page-heading">
        <p className="world-kicker">Provider targets</p>
        <h1>Top three integrations for every production category.</h1>
        <nav className="world-actions" aria-label="World route links">
          <a className="dx-secondary-action world-button world-button-secondary" href="/">Home</a>
          <a className="dx-secondary-action world-button world-button-secondary" href="/readiness">Readiness</a>
        </nav>
      </header>

      <section className="world-category-stack" aria-label="Integration catalog">
        {worldCategories.map((category) => (
          <section className="world-band" key={category.id} aria-labelledby={category.id}>
            <div className="world-band-heading">
              <p>{category.providers.length} provider targets</p>
              <h2 id={category.id}>{category.title}</h2>
              <span>{category.purpose}</span>
            </div>
            <div className="dx-grid world-provider-grid">
              {category.providers.map((provider) => (
                <article className="dx-card world-provider-card" key={provider.id}>
                  <div className="world-provider-title">
                    <h3>{provider.name}</h3>
                    <span>{provider.validation}</span>
                  </div>
                  <p>{provider.reason}</p>
                  <dl>
                    <div>
                      <dt>Package</dt>
                      <dd>{provider.packageId}</dd>
                    </div>
                    <div>
                      <dt>Runtime</dt>
                      <dd>{provider.runtime}</dd>
                    </div>
                    <div>
                      <dt>Surface</dt>
                      <dd>{provider.surface}</dd>
                    </div>
                    <div>
                      <dt>Adapter</dt>
                      <dd>{provider.adapter}</dd>
                    </div>
                    <div>
                      <dt>Env</dt>
                      <dd>{provider.env.length ? provider.env.map((item) => item.name).join(", ") : "none"}</dd>
                    </div>
                    <div>
                      <dt>Route</dt>
                      <dd>{provider.routeHandlers.join(", ")}</dd>
                    </div>
                  </dl>
                </article>
              ))}
            </div>
          </section>
        ))}
      </section>
    </main>
  );
}
