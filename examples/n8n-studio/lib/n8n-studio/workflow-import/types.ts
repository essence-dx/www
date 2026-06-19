import type { WorkflowDocument } from "../types";

export type ImportSanitationIssue = {
  code:
    | "unknown-node-type"
    | "credential-secret-stripped"
    | "parameter-secret-stripped"
    | "connection-source-missing"
    | "connection-target-missing"
    | "webhook-id-regenerated";
  message: string;
  nodeName?: string;
};

export type ImportSanitationResult = {
  document: WorkflowDocument;
  issues: ImportSanitationIssue[];
  regeneratedWebhookIds: string[];
};
