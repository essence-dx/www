import type { PinnedDataState } from "../../lib/n8n-studio/types";

export type PinnedDataPanelProps = {
  pinnedData: PinnedDataState[];
};

export function PinnedDataPanel({ pinnedData }: PinnedDataPanelProps) {
  return (
    <section className="n8ns-panel" data-studio-surface="pinned-data">
      <div className="n8ns-panel-header">
        <h2>Pinned data</h2>
        <span className="n8ns-badge">{pinnedData.length} node</span>
      </div>
      {pinnedData.map((pin) => (
        <article className="n8ns-readiness-row" data-status={pin.status} key={pin.nodeName}>
          <div>
            <strong>{pin.nodeName}</strong>
            <span>{pin.itemCount} item</span>
          </div>
          <p>{pin.sizePolicy}</p>
        </article>
      ))}
    </section>
  );
}

