import { StateRuntimeProbe } from "../../components/state-runtime-probe";

export const metadata = {
  title: "State Runtime Probe",
  description: "Browser proof route for DX-native state runtime receipts.",
} as const;

export default function StateRuntimeProbePage() {
  return (
    <main
      className="starter-shell"
      data-dx-route="/state-runtime"
      data-dx-proof-route="state-runtime"
    >
      <StateRuntimeProbe />
      <nav className="starter-route-links" aria-label="Proof route links">
        <a className="starter-link starter-link-secondary" href="/">
          Home
        </a>
        <a className="starter-link starter-link-secondary" href="/islands">
          Island proof
        </a>
      </nav>
    </main>
  );
}
