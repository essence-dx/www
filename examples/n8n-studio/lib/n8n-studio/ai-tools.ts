import type { AiToolState } from "./types";

export const aiToolState: AiToolState = {
  status: "source-only",
  focusedNodeIds: ["node-http-request", "node-openai"],
  toolLifecycle: ["pending", "running", "suspended", "done", "cancelled", "error"],
};

