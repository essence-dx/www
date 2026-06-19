import type { ImportExportState } from "../../lib/n8n-studio/types";

export type ImportExportPanelProps = {
  importExport: ImportExportState;
  onApplyImportPreview?: () => void;
  onExportCurrentWorkflow?: () => void;
  onSaveImportedDraft?: () => void;
};

export function ImportExportPanel({
  importExport,
  onApplyImportPreview,
  onExportCurrentWorkflow,
  onSaveImportedDraft,
}: ImportExportPanelProps) {
  const importPreview = importExport.importPreview;
  const exportReceipt = importExport.exportReceipt;
  const currentExport = importExport.currentExport;
  const draft = importExport.draft;

  return (
    <section className="n8ns-panel" data-studio-surface="import-export">
      <div className="n8ns-panel-header">
        <h2>Import / export</h2>
        <span className="n8ns-badge">{importExport.exportFormat}</span>
      </div>
      <div className="n8ns-action-row">
        {importExport.importSources.map((source) => (
          <button
            key={source.source}
            aria-disabled="true"
            data-import-source={source.source}
            data-import-source-status={source.status}
            data-live-provider-execution={String(source.liveProviderExecution)}
            disabled
            title={source.issue}
            type="button"
          >
            {source.label}
          </button>
        ))}
      </div>
      <div
        className="n8ns-import-summary"
        data-import-draft-status={draft.status}
        data-import-preview-status={importPreview.status}
        data-import-source={importPreview.source ?? "none"}
        data-live-provider-execution={String(importPreview.boundary.liveProviderExecution)}
        data-secrets-included={String(importPreview.boundary.secretsIncluded)}
      >
        <div>
          <strong>{importPreview.workflowName ?? "No imported workflow loaded"}</strong>
          <span className="n8ns-badge">{importPreview.status}</span>
        </div>
        <div className="n8ns-import-counts">
          <span>{importPreview.keptNodeCount} node</span>
          <span>{importPreview.connectionCount} connection</span>
          <span>{importPreview.pinDataNodeCount} pinned set</span>
          <span>{importPreview.regeneratedWebhookCount} webhook id</span>
        </div>
        <p className="n8ns-muted">{importPreview.boundary.issue}</p>
      </div>
      <div className="n8ns-action-row" aria-label="Import draft controls">
        <button
          data-import-action="apply-preview"
          data-import-action-ready={String(draft.canApplyPreview)}
          disabled={!draft.canApplyPreview || !onApplyImportPreview}
          onClick={onApplyImportPreview}
          type="button"
        >
          Apply preview
        </button>
        <button
          data-import-action="save-draft"
          data-import-action-ready={String(draft.canSaveDraft)}
          disabled={!draft.canSaveDraft || !onSaveImportedDraft}
          onClick={onSaveImportedDraft}
          type="button"
        >
          Save draft
        </button>
        <span
          className="n8ns-badge"
          data-import-draft-saved={String(draft.status === "saved")}
          data-import-save-receipt={draft.saveReceiptPath}
        >
          {draft.status}
        </span>
      </div>
      <p className="n8ns-muted">{draft.issue}</p>
      <div className="n8ns-action-row" aria-label="Current workflow export controls">
        <button
          data-current-export-status={currentExport.status}
          data-export-action="current-workflow"
          disabled={currentExport.status === "exporting" || !onExportCurrentWorkflow}
          onClick={onExportCurrentWorkflow}
          type="button"
        >
          Export current workflow
        </button>
        <span
          className="n8ns-badge"
          data-current-export-status={currentExport.status}
          data-export-route={currentExport.routePath}
        >
          {currentExport.status}
        </span>
      </div>
      <div
        className="n8ns-import-summary"
        data-current-export-live-provider-execution={String(
          currentExport.liveProviderExecution,
        )}
        data-current-export-secrets-included={String(currentExport.secretsIncluded)}
        data-current-export-status={currentExport.status}
      >
        <div>
          <strong>{currentExport.downloadName ?? exportReceipt.downloadName}</strong>
          <span className="n8ns-badge">{currentExport.routePath}</span>
        </div>
        <div className="n8ns-import-counts">
          <span>{currentExport.nodeCount ?? exportReceipt.nodeCount} node</span>
          <span>
            {currentExport.connectionCount ?? exportReceipt.connectionCount} connection
          </span>
          <span>
            {currentExport.credentialReferenceCount ??
              exportReceipt.credentialReferenceCount} credential ref
          </span>
        </div>
        <p className="n8ns-muted">{currentExport.errorMessage ?? currentExport.issue}</p>
      </div>
      <div className="n8ns-surface-grid" aria-label="Sanitized import fields">
        {importExport.sanitizedFields.map((field) => (
          <span key={field} data-import-sanitized-field={field}>
            {field}
          </span>
        ))}
      </div>
      <div className="n8ns-issue-list" aria-label="Import sanitation issues">
        {importPreview.issues.length === 0 ? (
          <p className="n8ns-muted">No sanitation issues for the current import preview.</p>
        ) : (
          importPreview.issues.map((issue) => (
            <article
              key={`${issue.code}-${issue.nodeName ?? "workflow"}-${issue.message}`}
              data-import-issue-code={issue.code}
              data-import-issue-severity={issue.severity}
            >
              <div>
                <strong>{issue.code}</strong>
                <span>{issue.severity}</span>
              </div>
              <p>{issue.message}</p>
              <p>{issue.action}</p>
            </article>
          ))
        )}
      </div>
      <div
        className="n8ns-export-receipt"
        data-export-live-provider-execution={String(exportReceipt.liveProviderExecution)}
        data-export-receipt-schema={exportReceipt.schema}
        data-export-secrets-included={String(exportReceipt.secretsIncluded)}
      >
        <div>
          <strong>{exportReceipt.downloadName}</strong>
          <span className="n8ns-badge">{exportReceipt.status}</span>
        </div>
        <div className="n8ns-import-counts">
          <span>{exportReceipt.nodeCount} node</span>
          <span>{exportReceipt.connectionCount} connection</span>
          <span>{exportReceipt.credentialReferenceCount} credential ref</span>
        </div>
        <p className="n8ns-muted">{exportReceipt.issue}</p>
      </div>
    </section>
  );
}
