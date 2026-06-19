import { isRecord } from "./records";
import type { ImportSanitationIssue } from "./types";

function secretLikeKey(key: string) {
  return /(api[-_]?key|token|password|secret|private[-_]?key|client[-_]?secret)/i.test(key);
}

export function stripSecretValues(
  value: unknown,
  issues: ImportSanitationIssue[],
  nodeName: string | undefined,
  issueCode: ImportSanitationIssue["code"],
): unknown {
  if (Array.isArray(value)) {
    return value.map((item) => stripSecretValues(item, issues, nodeName, issueCode));
  }

  if (!isRecord(value)) {
    return value;
  }

  const sanitized: Record<string, unknown> = {};
  for (const [key, child] of Object.entries(value)) {
    if (secretLikeKey(key)) {
      issues.push({
        code: issueCode,
        message: "Removed a secret-like field from imported workflow data.",
        nodeName,
      });
      continue;
    }

    sanitized[key] = stripSecretValues(child, issues, nodeName, issueCode);
  }

  return sanitized;
}
