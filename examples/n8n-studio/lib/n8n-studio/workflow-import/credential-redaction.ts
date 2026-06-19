import { isRecord } from "./records";
import { stripSecretValues } from "./secret-redaction";
import type { ImportSanitationIssue } from "./types";

export function sanitizeCredentials(
  credentials: unknown,
  issues: ImportSanitationIssue[],
  nodeName: string,
) {
  if (!isRecord(credentials)) {
    return undefined;
  }

  const sanitized: Record<string, { id: string; name: string }> = {};
  for (const [credentialName, credential] of Object.entries(credentials)) {
    if (!isRecord(credential)) {
      continue;
    }

    const id = typeof credential.id === "string" ? credential.id : "";
    const name = typeof credential.name === "string" ? credential.name : credentialName;
    sanitized[credentialName] = { id, name };

    const stripped = stripSecretValues(
      credential,
      issues,
      nodeName,
      "credential-secret-stripped",
    );
    if (JSON.stringify(stripped) !== JSON.stringify({ id, name })) {
      issues.push({
        code: "credential-secret-stripped",
        message: `Kept only id and display name for credential "${credentialName}".`,
        nodeName,
      });
    }
  }

  return Object.keys(sanitized).length > 0 ? sanitized : undefined;
}
