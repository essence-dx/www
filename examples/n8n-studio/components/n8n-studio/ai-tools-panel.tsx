import type { AiToolState } from "../../lib/n8n-studio/types";

export type AiToolsPanelProps = {
  aiTools: AiToolState;
};

export function AiToolsPanel({ aiTools }: AiToolsPanelProps) {
  return (
    <section className="n8ns-panel" data-studio-surface="ai-tools">
      <div className="n8ns-panel-header">
        <h2>AI tools</h2>
        <span className="n8ns-badge">{aiTools.status}</span>
      </div>
      <div className="n8ns-tool-lifecycle">
        {aiTools.toolLifecycle.map((state) => (
          <span key={state}>{state}</span>
        ))}
      </div>
      <p className="n8ns-muted">{aiTools.focusedNodeIds.length} focused workflow nodes</p>
    </section>
  );
}

