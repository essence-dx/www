import { StateRuntimeProbe } from "../../components/state-runtime-probe";

export const metadata = {
  title: "State Runtime Probe",
  description: "Browser proof route for DX-native state runtime receipts.",
} as const;

export default function StateRuntimeProbePage() {
  return (
    <main
      className="starter-shell"
      data-dx-route="state-runtime"
      data-dx-proof-route="state-runtime"
    >
      <StateRuntimeProbe />
    </main>
  );
}
