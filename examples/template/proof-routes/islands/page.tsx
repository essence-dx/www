import { IslandRuntimeProbe } from "../../components/island-runtime-probe";

export const metadata = {
  title: "Island Runtime",
  description: "Source-owned island runtime route for DX WWW.",
} as const;

export default function IslandsPage() {
  return (
    <main
      className="starter-shell"
      data-dx-route="/islands"
      data-dx-proof-route="islands"
    >
      <IslandRuntimeProbe
        clientLoad
        clientVisible
        clientIdle
        clientOnly="dx"
        label="Island Runtime"
      />
      <nav className="starter-route-links" aria-label="Proof route links">
        <a className="starter-link starter-link-secondary" href="/">
          Home
        </a>
        <a className="starter-link starter-link-secondary" href="/state-runtime">
          State runtime proof
        </a>
      </nav>
    </main>
  );
}
