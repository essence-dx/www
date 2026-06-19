import type { ExecutionReadiness } from "../../lib/n8n-studio/types";

export type ExecutionPanelProps = {
  execution: ExecutionReadiness;
};

export function ExecutionPanel({ execution }: ExecutionPanelProps) {
  const selectedAttempt =
    execution.attempts.find((attempt) => attempt.id === execution.selectedAttemptId) ??
    execution.attempts[0];

  return (
    <section
      className="n8ns-panel"
      data-execution-live-provider-execution={String(execution.liveProviderExecution)}
      data-studio-surface="execution-debug"
    >
      <div className="n8ns-panel-header">
        <h2>Execution</h2>
        <span className="n8ns-badge">{execution.status}</span>
      </div>
      <div className="n8ns-segmented" role="group" aria-label="Execution view">
        {execution.debugViews.map((view) => (
          <button key={view} data-active={String(view === execution.activeDebugView)} type="button">
            {view}
          </button>
        ))}
      </div>
      {selectedAttempt ? (
        <article
          className="n8ns-execution-attempt"
          data-execution-attempt-id={selectedAttempt.id}
          data-execution-attempt-status={selectedAttempt.status}
        >
          <div>
            <strong>{selectedAttempt.workflowName}</strong>
            <span>{selectedAttempt.mode}</span>
          </div>
          <div className="n8ns-import-counts">
            <span>{selectedAttempt.inputItemCount} input</span>
            <span>{selectedAttempt.outputItemCount} output</span>
            <span>{selectedAttempt.receiptPath}</span>
          </div>
          <p className="n8ns-muted">{selectedAttempt.issue}</p>
        </article>
      ) : null}
      <div className="n8ns-surface-grid" aria-label="Execution actions">
        {execution.availableActions.map((action) => (
          <span key={action}>{action}</span>
        ))}
      </div>
      <div className="n8ns-execution-log-list" aria-label="Execution node logs">
        {execution.nodeLogs.map((log) => (
          <article
            key={log.id}
            data-execution-node-log={log.nodeId}
            data-execution-node-status={log.status}
            data-execution-provider-error={log.providerErrorMessage ? "true" : "false"}
          >
            <div>
              <strong>{log.nodeName}</strong>
              <span>{log.status}</span>
            </div>
            <p>{log.dataPreviewLabel}</p>
            {log.providerErrorMessage ? <p>{log.providerErrorMessage}</p> : null}
            <p>{log.issue}</p>
          </article>
        ))}
      </div>
      <div
        className="n8ns-execution-receipt"
        data-execution-receipt-boundary={String(
          execution.receiptBoundary.providerBoundary,
        )}
        data-execution-receipt-imported={String(
          execution.receiptBoundary.executionReceiptImported,
        )}
        data-execution-receipt-imported-at={
          execution.receiptBoundary.importedAt ?? "not-imported"
        }
      >
        <strong>{execution.receiptBoundary.receiptRoot}</strong>
        <p className="n8ns-muted">{execution.receiptBoundary.issue}</p>
      </div>
      {execution.receiptIssues.length > 0 ? (
        <div className="n8ns-execution-issue-list" aria-label="Execution receipt issues">
          {execution.receiptIssues.map((issue) => (
            <article
              key={`${issue.code}-${issue.nodeName ?? "workflow"}-${issue.message}`}
              data-execution-receipt-issue={issue.code}
              data-execution-receipt-issue-severity={issue.severity}
            >
              <div>
                <strong>{issue.code}</strong>
                <span>{issue.severity}</span>
              </div>
              <p>{issue.message}</p>
            </article>
          ))}
        </div>
      ) : null}
      <p className="n8ns-muted">{execution.blockedReason}</p>
    </section>
  );
}
