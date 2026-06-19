import type { ResourceLocatorState } from "../../lib/n8n-studio/types";

export type ResourceLocatorPanelProps = {
  resourceLocator: ResourceLocatorState;
};

export function ResourceLocatorPanel({ resourceLocator }: ResourceLocatorPanelProps) {
  return (
    <section className="n8ns-panel" data-studio-surface="resource-locator">
      <div className="n8ns-panel-header">
        <h2>Resource locator</h2>
        <span className="n8ns-badge">{resourceLocator.mode}</span>
      </div>
      <label className="n8ns-field">
        <span>Search</span>
        <input aria-label="Resource search" readOnly value={resourceLocator.query} />
      </label>
      <p className="n8ns-muted">{resourceLocator.issue}</p>
    </section>
  );
}

