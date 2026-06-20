import type { CatalogSummary, N8nSourceNode } from "./types";

export const n8nSourceManifestPath =
  "integrations/n8n-nodes-base/dx-node-source-.dx/build-cache/manifest.json";

export const n8nCatalogNodes: N8nSourceNode[] = [
  {
    id: "manual-trigger",
    name: "ManualTrigger",
    displayName: "Manual Trigger",
    category: "Core",
    role: "trigger",
    description: "Starts a workflow from an explicit editor action.",
    sourcePath: "nodes/ManualTrigger/ManualTrigger.node.ts",
    credentialTypes: [],
    operations: ["trigger"],
    trustStatus: "source-mirrored",
  },
  {
    id: "webhook",
    name: "Webhook",
    displayName: "Webhook",
    category: "Core",
    role: "trigger",
    description: "Receives HTTP requests and starts a workflow from a webhook URL.",
    sourcePath: "nodes/Webhook/Webhook.node.ts",
    credentialTypes: [],
    operations: ["GET", "POST", "PUT", "PATCH", "DELETE"],
    trustStatus: "source-mirrored",
  },
  {
    id: "http-request",
    name: "HttpRequest",
    displayName: "HTTP Request",
    category: "Core",
    role: "action",
    description: "Calls an HTTP endpoint with method, URL, authentication, headers, and body settings.",
    sourcePath: "nodes/HttpRequest/HttpRequest.node.ts",
    credentialTypes: [
      "HttpBasicAuth",
      "HttpBearerAuth",
      "HttpDigestAuth",
      "HttpCustomAuth",
    ],
    operations: ["request"],
    trustStatus: "source-mirrored",
  },
  {
    id: "slack",
    name: "Slack",
    displayName: "Slack",
    category: "Communication",
    role: "action",
    description: "Posts, searches, updates, and manages Slack messages and resources.",
    sourcePath: "nodes/Slack/Slack.node.ts",
    credentialTypes: ["SlackApi", "SlackOAuth2Api"],
    operations: ["message.post", "message.search", "channel.getAll", "file.upload"],
    trustStatus: "source-mirrored",
  },
  {
    id: "gmail",
    name: "Gmail",
    displayName: "Gmail",
    category: "Productivity",
    role: "action",
    description: "Reads, sends, labels, replies to, and manages Gmail messages.",
    sourcePath: "nodes/Google/Gmail/Gmail.node.ts",
    credentialTypes: ["GmailOAuth2Api"],
    operations: ["message.send", "message.reply", "draft.create", "label.getAll"],
    trustStatus: "source-mirrored",
  },
  {
    id: "openai",
    name: "OpenAi",
    displayName: "OpenAI",
    category: "AI",
    role: "ai-tool",
    description: "Runs chat, text, and image operations through the n8n OpenAI connector.",
    sourcePath: "nodes/OpenAi/OpenAi.node.ts",
    credentialTypes: ["OpenAiApi"],
    operations: ["chat.complete", "text.complete", "image.create"],
    trustStatus: "source-mirrored",
  },
  {
    id: "message-an-agent",
    name: "MessageAnAgent",
    displayName: "Message an Agent",
    category: "AI",
    role: "ai-tool",
    description: "Bridges workflow execution into an agent messaging step.",
    sourcePath: "nodes/MessageAnAgent/MessageAnAgent.node.ts",
    credentialTypes: [],
    operations: ["message"],
    trustStatus: "source-mirrored",
  },
];

export const n8nCatalogSummary: CatalogSummary = {
  schema: "dx.n8n-studio.catalog",
  sourceManifestPath: n8nSourceManifestPath,
  copiedFrom: "packages/nodes-base",
  nodeFolderCount: 307,
  nodeFileCount: 536,
  credentialFileCount: 396,
  catalogNodes: n8nCatalogNodes,
};

export function findCatalogNode(name: string) {
  return n8nCatalogNodes.find((node) => node.name === name);
}

