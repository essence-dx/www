import { IslandRuntimeProbe } from "../../components/island-runtime-probe";

export const metadata = {
  title: "Island Runtime",
  description: "Source-owned island runtime route for DX WWW.",
} as const;

export default function IslandsPage() {
  return (
    <main
      className="starter-shell"
      data-dx-route="islands"
      data-dx-proof-route="islands"
    >
      <IslandRuntimeProbe
        clientLoad
        clientVisible
        clientIdle
        clientOnly="dx"
        label="Island Runtime"
      />
    </main>
  );
}
