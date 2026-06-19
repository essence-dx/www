import {
  normalizeDxN8nConnector,
  type DxN8nLaunchConnector,
  type DxN8nNormalizedConnector,
  type DxN8nWorkflowStatus,
} from "./catalog";

export type DxN8nCredentialReadiness = {
  readonly status: DxN8nWorkflowStatus;
  readonly requiredEnv: readonly string[];
  readonly missingCredentials: readonly string[];
  readonly credentialsConfigured: boolean;
  readonly credentialTypes: readonly string[];
  readonly message: string;
};

const credentialEnvNames: Record<string, readonly string[]> = {
  gmailOAuth2: ["GOOGLE_CLIENT_ID", "GOOGLE_CLIENT_SECRET"],
  googleApi: ["GOOGLE_API_KEY"],
  notionApi: ["NOTION_API_KEY"],
  notionOAuth2Api: ["NOTION_CLIENT_ID", "NOTION_CLIENT_SECRET"],
  slackApi: ["SLACK_BOT_TOKEN"],
  slackOAuth2Api: ["SLACK_CLIENT_ID", "SLACK_CLIENT_SECRET"],
};

export function buildDxN8nCredentialReadiness(
  connector: DxN8nLaunchConnector | DxN8nNormalizedConnector,
  env: Record<string, string | undefined> = {},
): DxN8nCredentialReadiness {
  const normalized = normalizeDxN8nConnector(connector);
  const requiredEnv =
    normalized.status === "ready"
      ? []
      : unique(
          normalized.credentials.flatMap((credential) =>
            credentialEnvNames[credential] ?? [fallbackEnvName(credential)],
          ),
        );
  const missingCredentials = requiredEnv.filter(
    (name) => !envValuePresent(env, name),
  );
  const credentialsConfigured = missingCredentials.length === 0;
  const status =
    requiredEnv.length === 0
      ? "metadata-ready"
      : normalized.workflowNode.runMode === "credential-gated"
        ? "credential-gated"
        : "missing-config";

  return {
    status,
    requiredEnv,
    missingCredentials,
    credentialsConfigured,
    credentialTypes: normalized.credentials,
    message:
      requiredEnv.length === 0
        ? `${normalized.displayName} can draft a local workflow receipt without secrets.`
        : credentialsConfigured
          ? `${normalized.displayName} has local credential presence for a Zed handoff; live n8n execution still remains app-owned.`
        : `${normalized.displayName} needs app-owned credentials before live execution.`,
  };
}

function envValuePresent(env: Record<string, string | undefined>, name: string) {
  const value = env[name];

  return typeof value === "string" && value.trim().length > 0;
}

function fallbackEnvName(credential: string) {
  return `${credential
    .replace(/([a-z0-9])([A-Z])/g, "$1_$2")
    .replace(/[^a-zA-Z0-9]+/g, "_")
    .toUpperCase()}_SECRET`;
}

function unique(values: readonly string[]) {
  return [...new Set(values)];
}
