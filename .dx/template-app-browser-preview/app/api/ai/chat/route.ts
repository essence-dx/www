import { createOpenAI } from "@ai-sdk/openai";

import { createDxAIChatRoute } from "@/lib/ai/chat-route";
import { createDxLaunchMessagePruner } from "@/lib/ai/message-pruning";
import { createDxAIModelConfig } from "@/lib/ai/model";
import { createDxAiMissingProviderResponse } from "@/lib/ai/provider-boundary";
import { createDxLaunchToolApproval } from "@/lib/ai/tool-approval";

const openai = createOpenAI({
  apiKey: process.env.AI_PROVIDER_API_KEY,
});

const postWithConfiguredProvider = createDxAIChatRoute({
  model: createDxAIModelConfig({
    model: openai("gpt-5-mini"),
    system: "You are the DX launch assistant. Answer with concise, sourced launch status.",
  }),
  messagePruner: createDxLaunchMessagePruner({
    reasoning: "before-last-message",
    toolCalls: "before-last-2-messages",
  }),
  toolApproval: createDxLaunchToolApproval({
    autoApproveTools: ["launchStatus"],
  }),
  readStatus: () => ({
    current: "DX launch AI route is wired.",
    next: "Connect this function to live DX receipts before production launch.",
    score: 80,
  }),
});

export async function POST(request: Request): Promise<Response> {
  if (!process.env.AI_PROVIDER_API_KEY) {
    return createDxAiMissingProviderResponse({
      provider: "openai-compatible",
      capability: "chat-stream",
      requiredEnv: "AI_PROVIDER_API_KEY",
      appOwnedBoundary:
        "Set AI_PROVIDER_API_KEY in the app environment to stream model output.",
    });
  }

  return postWithConfiguredProvider(request);
}
