import type { NodeTypeDescription } from "./types";

export const manualTriggerNodeType: NodeTypeDescription = {
  name: "n8n-nodes-base.manualTrigger",
  displayName: "Manual Trigger",
  sourcePath: "nodes/ManualTrigger/ManualTrigger.node.ts",
  version: 1,
  credentials: [],
  properties: [],
  categories: ["trigger"],
  aliases: ["manual", "run manually"],
  sourceProvenance: "local-n8n-source-manifest",
  workflowNode: {
    ready: true,
    trigger: true,
    usable_as_tool: false,
    run_mode: "manual",
  },
};
