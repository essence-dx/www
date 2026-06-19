import {
  forgeSafetyArchiveContract,
  type ForgeSafetyArchiveContract,
} from "./forge-safety-archive-contract";
import {
  forgeSafetyArchiveRunbookReadModel,
  type ForgeSafetyArchiveRunbookReadModel,
} from "./forge-safety-archive-runbook";

type ForgeSafetyArchivePanelProps = {
  readonly contract?: ForgeSafetyArchiveContract;
  readonly runbook?: ForgeSafetyArchiveRunbookReadModel;
};

function safetyArchiveTone(state: string) {
  if (state === "covered") {
    return "border-border bg-card text-card-foreground";
  }
  if (state === "partial") {
    return "border-border bg-muted text-foreground";
  }
  return "border-border bg-background text-muted-foreground";
}

export function ForgeSafetyArchivePanel({
  contract,
  runbook,
}: ForgeSafetyArchivePanelProps) {
  const panelContract = contract ?? forgeSafetyArchiveContract;
  const panelRunbook = runbook ?? forgeSafetyArchiveRunbookReadModel;
  const tone = safetyArchiveTone(panelContract.state);

  return (
    <article
      className={`grid gap-3 rounded-md border p-3 text-sm ${tone}`}
      data-dx-component={panelContract.component}
      data-dx-forge-status-surface={panelContract.statusSurface}
      data-dx-zed-surface={panelContract.zedSurface}
      data-dx-safety-archive-contract={panelContract.schema}
      data-dx-safety-archive-state={panelContract.state}
      data-dx-safety-archive-safe-delete={
        panelContract.safeForDestructivePackageOperations
      }
      data-dx-safety-archive-package-count={panelContract.packageCount}
      data-dx-safety-archive-rollback-coverage={
        panelContract.rollbackCoveragePercent
      }
      data-dx-safety-archive-receipt-count={
        panelContract.archiveReceiptCount
      }
      data-dx-safety-archive-runbook-source={
        panelRunbook.previewManifestSource
      }
      data-dx-safety-archive-runbook-fixture={panelRunbook.fixturePath}
      data-dx-safety-archive-runbook-guard={panelRunbook.guardId}
      data-dx-safety-archive-runbook-command={panelRunbook.command}
      data-dx-safety-archive-runbook-policy={
        panelRunbook.sourceOnly ? "source-only" : "unknown"
      }
    >
      <div className="flex items-start justify-between gap-3">
        <div className="grid gap-1">
          <p className="text-xs font-medium uppercase tracking-normal">
            Archive safety
          </p>
          <h3 className="text-sm font-semibold text-current">
            {panelContract.rollbackCoveragePercent}% rollback coverage
          </h3>
        </div>
        <span className="rounded-md border border-current/20 px-2 py-1 text-xs">
          {panelContract.state}
        </span>
      </div>

      <dl className="grid grid-cols-2 gap-2 text-xs">
        <div>
          <dt className="text-current/70">Packages</dt>
          <dd className="font-medium">{panelContract.packageCount}</dd>
        </div>
        <div>
          <dt className="text-current/70">Archive receipts</dt>
          <dd className="font-medium">{panelContract.archiveReceiptCount}</dd>
        </div>
        <div>
          <dt className="text-current/70">Covered</dt>
          <dd className="font-medium">
            {panelContract.rollbackCoveredPackageCount}
          </dd>
        </div>
        <div>
          <dt className="text-current/70">Missing</dt>
          <dd className="font-medium">
            {panelContract.rollbackMissingPackageCount}
          </dd>
        </div>
      </dl>

      <p className="text-xs leading-5 text-current/80">
        {panelContract.nextAction}
      </p>
      <dl className="grid gap-1 text-xs">
        <dt className="text-current/70">Rollback proof</dt>
        <dd className="break-all font-mono text-current/80">
          {panelRunbook.command}
        </dd>
      </dl>
      <p className="break-all text-xs text-current/60">
        {panelContract.archiveDirectory}
      </p>
    </article>
  );
}
