import type { N8nStudioState } from "../../lib/n8n-studio/types";
import { Icon } from "../icons/icon";

export type StudioTopbarProps = {
  state: N8nStudioState;
};

export function StudioTopbar({ state }: StudioTopbarProps) {
  return (
    <header className="n8ns-topbar">
      <div className="n8ns-brand">
        <span className="n8ns-brand-mark" aria-hidden="true">
          <Icon className="n8ns-icon" name="n8n-studio:workflow" />
        </span>
        <div>
          <p className="n8ns-eyebrow">DX WWW</p>
          <h1>n8n Studio</h1>
        </div>
      </div>
      <nav className="n8ns-topbar-tabs" aria-label="Studio views">
        <a aria-current="page" href="/">Editor</a>
        <a href="/connectors">Connectors</a>
        <a href="/credentials">Credentials</a>
        <a href="/executions">Executions</a>
        <a href="/receipts">Receipts</a>
      </nav>
      <div className="n8ns-topbar-status" aria-label="Provider status">
        <span>{state.catalog.nodeFileCount} node files</span>
        <span>{state.catalog.credentialFileCount} credentials</span>
        <strong>Live execution blocked</strong>
      </div>
    </header>
  );
}

