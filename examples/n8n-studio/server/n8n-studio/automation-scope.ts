type AutomationCapabilityStatus =
  | "available"
  | "partial"
  | "planned"
  | "provider-gated";

type AutomationCapability = {
  id:
    | "node-metadata-catalog"
    | "workflow-json-authoring"
    | "node-connection-authoring"
    | "n8n-runtime-execution"
    | "credential-bridge"
    | "dynamic-options-resource-locators";
  status: AutomationCapabilityStatus;
  summary: string;
};

const capabilities: AutomationCapability[] = [
  {
    id: "node-metadata-catalog",
    status: "available",
    summary:
      "Local generated and source-backed n8n metadata feed the DX catalog and node creator.",
  },
  {
    id: "workflow-json-authoring",
    status: "available",
    summary:
      "DX-owned workflow documents can be created, imported, sanitized, and exported as n8n-shaped JSON.",
  },
  {
    id: "node-connection-authoring",
    status: "available",
    summary:
      "DX-owned canvas actions create and validate node connections without provider execution.",
  },
  {
    id: "n8n-runtime-execution",
    status: "partial",
    summary:
      "Governed n8n runtime handoff can publish, activate, submit webhook trigger requests, write redacted execution-proof receipts, and retry delayed history imports; manual/internal triggers still require provider controls and imported receipts.",
  },
  {
    id: "credential-bridge",
    status: "partial",
    summary:
      "Credential references, vault readiness, n8n API handoff, webhook trigger URLs, picker responses, and provider credential validation receipts are redacted; secret loading remains adapter-owned.",
  },
  {
    id: "dynamic-options-resource-locators",
    status: "partial",
    summary:
      "Editor-session request and response contracts cover dynamic options, resource locators, mapper schemas, credential metadata, and host-executed request batches.",
  },
];

export function createAutomationScope() {
  return {
    schema: "dx.n8n-studio.automation-scope",
    goal: "n8n-runtime-backed-dx-automation",
    uiOwnership: "dx-www-and-zed-native",
    editorPortTarget: false,
    runtimeOwnership: "n8n-backend-through-governed-dx-bridge",
    nonGoals: ["n8n-vue-editor-port"],
    capabilities,
  };
}
