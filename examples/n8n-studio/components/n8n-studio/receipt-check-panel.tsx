import type { ReceiptSummary } from "../../lib/n8n-studio/types";

export type ReceiptCheckPanelProps = {
  receipts: ReceiptSummary;
};

export function ReceiptCheckPanel({ receipts }: ReceiptCheckPanelProps) {
  return (
    <section className="n8ns-panel" data-studio-surface="receipts">
      <div className="n8ns-panel-header">
        <h2>Receipts</h2>
        <span className="n8ns-badge">Boundary visible</span>
      </div>
      <p className="n8ns-muted">{receipts.receiptRoot}</p>
      <div className="n8ns-surface-grid">
        {receipts.surfaces.map((surface) => (
          <span key={surface}>{surface}</span>
        ))}
      </div>
    </section>
  );
}

