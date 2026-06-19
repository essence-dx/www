import { isRecord } from "./records";
import type { ImportSanitationIssue } from "./types";

export function regenerateWebhookIds(
  input: Record<string, unknown>,
  issues: ImportSanitationIssue[],
) {
  const meta = isRecord(input.meta) ? input.meta : {};
  if (typeof meta.webhookId !== "string") {
    return [];
  }

  const regenerated = `dx-${meta.webhookId}`;
  issues.push({
    code: "webhook-id-regenerated",
    message: "Regenerated imported webhook identifier for local draft safety.",
  });
  return [regenerated];
}
