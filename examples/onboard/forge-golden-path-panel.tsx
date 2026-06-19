import {
  forgeGoldenPathContract,
  type ForgeGoldenPathContract,
  type ForgeGoldenPathStepState,
} from "./forge-golden-path-contract";

type ForgeGoldenPathPanelProps = {
  readonly contract?: ForgeGoldenPathContract;
};

function stateTone(state: ForgeGoldenPathStepState) {
  if (state === "real") {
    return "border-border bg-card text-card-foreground";
  }
  if (state === "partial") {
    return "border-border bg-muted text-foreground";
  }
  if (state === "blocked") {
    return "border-border bg-muted text-foreground";
  }
  return "border-border bg-background text-muted-foreground";
}

export function ForgeGoldenPathPanel({ contract }: ForgeGoldenPathPanelProps) {
  const panelContract = contract ?? forgeGoldenPathContract;
  const tone = stateTone(panelContract.state);

  return (
    <article
      className={`grid gap-3 rounded-md border p-3 text-sm ${tone}`}
      data-dx-component={panelContract.component}
      data-dx-forge-status-surface={panelContract.statusSurface}
      data-dx-zed-surface={panelContract.zedSurface}
      data-dx-forge-golden-path={panelContract.schema}
      data-dx-forge-golden-path-state={panelContract.state}
      data-dx-forge-golden-path-package={panelContract.packageId}
      data-dx-forge-golden-path-export={panelContract.selectedExport}
      data-dx-forge-golden-path-real-steps={panelContract.realStepCount}
      data-dx-forge-golden-path-partial-steps={panelContract.partialStepCount}
      data-dx-forge-golden-path-blocked-steps={panelContract.blockedStepCount}
      data-dx-forge-golden-path-total-steps={panelContract.totalStepCount}
      data-dx-forge-golden-path-dx-check-score={panelContract.dxCheckScore}
      data-dx-forge-golden-path-dx-check-traffic={
        panelContract.dxCheckTraffic
      }
    >
      <div className="flex flex-wrap items-start justify-between gap-3">
        <div className="grid gap-1">
          <p className="text-xs font-medium uppercase tracking-normal">
            Forge golden path
          </p>
          <h3 className="text-sm font-semibold text-current">
            {panelContract.realStepCount}/{panelContract.totalStepCount} steps
            real
          </h3>
        </div>
        <span className="rounded-md border border-current/20 px-2 py-1 text-xs">
          dx-check {panelContract.dxCheckScore}/100{" "}
          {panelContract.dxCheckTraffic}
        </span>
      </div>

      <div className="grid gap-2">
        {panelContract.steps.map((step) => (
          <div
            key={step.id}
            className="grid gap-1 rounded-md border border-border bg-muted/40 p-2"
            data-dx-forge-golden-path-step={step.id}
            data-dx-forge-golden-path-step-state={step.state}
            data-dx-forge-golden-path-step-evidence={step.evidencePath}
          >
            <div className="flex flex-wrap items-center justify-between gap-2">
              <span className="font-medium">{step.label}</span>
              <span className="text-xs text-current/70">{step.state}</span>
            </div>
            <p className="text-xs leading-5 text-current/80">{step.summary}</p>
          </div>
        ))}
      </div>

      <p className="text-xs leading-5 text-current/80">
        {panelContract.nextAction}
      </p>
      <p className="break-all text-xs text-current/60">
        {panelContract.statusReceiptPath}
      </p>
    </article>
  );
}
