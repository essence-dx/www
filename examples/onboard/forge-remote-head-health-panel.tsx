import {
  createForgeRemoteHeadHealthPanelContract,
  forgeRemoteHeadHealthPanelContract,
  type ForgeRemoteHeadHealthPanelContract,
} from "./forge-remote-head-health-contract";
import type { LaunchForgeRemoteObjectHeadHealthStatus } from "./forge-package-status-read-model";

type ForgeRemoteHeadHealthPanelProps = {
  readonly contract?: ForgeRemoteHeadHealthPanelContract;
  readonly rows?: readonly LaunchForgeRemoteObjectHeadHealthStatus[];
};

function remoteHeadTone(state: string) {
  if (state === "safe") {
    return "border-border bg-card text-card-foreground";
  }
  if (state === "blocked") {
    return "border-border bg-muted text-foreground";
  }
  return "border-border bg-background text-muted-foreground";
}

export function ForgeRemoteHeadHealthPanel({
  contract,
  rows,
}: ForgeRemoteHeadHealthPanelProps) {
  const panelContract =
    contract ??
    (rows
      ? createForgeRemoteHeadHealthPanelContract(rows)
      : forgeRemoteHeadHealthPanelContract);
  const tone = remoteHeadTone(panelContract.state);

  return (
    <article
      className={`grid gap-3 rounded-md border p-3 text-sm ${tone}`}
      data-dx-component={panelContract.component}
      data-dx-forge-status-surface={panelContract.statusSurface}
      data-dx-zed-surface={panelContract.zedSurface}
      data-dx-remote-head-contract={panelContract.schema}
      data-dx-remote-head-state={panelContract.state}
      data-dx-remote-head-row-count={panelContract.rowCount}
      data-dx-remote-head-blocking-count={panelContract.blockingCheckCount}
      data-dx-remote-head-receipt-path={panelContract.sourceReceiptPath}
    >
      <div className="flex items-start justify-between gap-3">
        <div className="grid gap-1">
          <p className="text-xs font-medium uppercase tracking-normal">
            R2/S3 HEAD health
          </p>
          <h3 className="text-sm font-semibold text-current">
            {panelContract.packageId ?? "Remote package receipt missing"}
          </h3>
        </div>
        <span className="rounded-md border border-current/20 px-2 py-1 text-xs">
          {panelContract.state}
        </span>
      </div>

      {panelContract.state !== "missing" ? (
        <dl className="grid grid-cols-2 gap-2 text-xs">
          <div>
            <dt className="text-current/70">Provider</dt>
            <dd className="font-medium">{panelContract.providerKind}</dd>
          </div>
          <div>
            <dt className="text-current/70">Checks</dt>
            <dd className="font-medium">
              {panelContract.checkCount} total,{" "}
              {panelContract.blockingCheckCount} blocking
            </dd>
          </div>
          <div>
            <dt className="text-current/70">Missing required</dt>
            <dd className="font-medium">
              {panelContract.missingRequiredCount}
            </dd>
          </div>
          <div>
            <dt className="text-current/70">Byte mismatches</dt>
            <dd className="font-medium">{panelContract.byteMismatchCount}</dd>
          </div>
        </dl>
      ) : null}

      <p className="text-xs leading-5 text-current/80">
        {panelContract.nextAction}
      </p>
      <p className="break-all text-xs text-current/60">
        {panelContract.sourceReceiptPath}
      </p>
    </article>
  );
}
