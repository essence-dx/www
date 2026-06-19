import type { CredentialReadiness } from "../../lib/n8n-studio/types";

export type CredentialReadinessPanelProps = {
  credentials: CredentialReadiness[];
};

export function CredentialReadinessPanel({ credentials }: CredentialReadinessPanelProps) {
  return (
    <section className="n8ns-panel" data-studio-surface="credentials">
      <div className="n8ns-panel-header">
        <h2>Credentials</h2>
        <span className="n8ns-badge">Redacted</span>
      </div>
      <div className="n8ns-credential-list">
        {credentials.map((credential) => (
          <article
            className="n8ns-readiness-row"
            data-status={credential.status}
            key={`${credential.nodeId}-${credential.credentialType}`}
          >
            <div>
              <strong>{credential.credentialType}</strong>
              <span>{credential.nodeName}</span>
            </div>
            <select
              aria-label={`${credential.credentialType} credential`}
              data-credential-picker={credential.credentialType}
              disabled
              value={credential.selectedCredentialId ?? ""}
            >
              <option value="">No credential selected</option>
              {credential.credentialOptions.map((option) => (
                <option key={option.id} value={option.id}>
                  {option.name}
                </option>
              ))}
            </select>
            <small
              data-credential-provider-boundary={credential.credentialType}
              data-live-provider-execution={String(
                credential.pickerBoundary.liveProviderExecution,
              )}
              data-secrets-included={String(credential.pickerBoundary.secretsIncluded)}
            >
              secret-values-never-included
            </small>
            <p>{credential.issue}</p>
          </article>
        ))}
      </div>
    </section>
  );
}
