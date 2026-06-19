import { startCase } from "lodash";

export const metadata = {
  title: "Forge package proof",
  description: "A source-owned npm lodash source slice rendered through DX WWW.",
} as const;

export default function ForgeProofPage() {
  return (
    <main
      className="starter-shell forge-backed-shell"
      data-dx-route="/forge-proof"
      data-forge-package="npm/lodash"
    >
      <section
        className="starter-card source-owned-forge-package"
        aria-labelledby="forge-proof-title"
      >
        <p className="starter-kicker">Forge proof</p>
        <h1 id="forge-proof-title">lodash rendered from Forge source</h1>
        <p className="starter-copy">
          This route imports startCase from a source-owned npm package snapshot
          through Forge and renders it without node_modules.
        </p>
      </section>
    </main>
  );
}
