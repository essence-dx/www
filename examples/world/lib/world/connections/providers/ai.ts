import type { WorldConnectionProbe } from "../contracts";
import { httpProbe } from "./shared";

export const aiConnectionProbes: readonly WorldConnectionProbe[] = [
  httpProbe(
    {
      id: "openai-models",
      providerId: "openai",
      packageId: "ai/openai",
      name: "OpenAI models",
      category: "AI",
      kind: "http",
      endpoint: "https://api.openai.com/v1/models",
      documentationUrl: "https://platform.openai.com/docs/api-reference/models/list",
      requiredEnv: ["OPENAI_API_KEY"],
      optionalEnv: [],
    },
    (env) => ({
      endpoint: "https://api.openai.com/v1/models",
      headers: {
        Authorization: `Bearer ${env.OPENAI_API_KEY ?? ""}`,
      },
      expectedStatuses: [200],
      evidence: "model-list-readable",
    }),
  ),
  httpProbe(
    {
      id: "anthropic-models",
      providerId: "anthropic",
      packageId: "ai/anthropic",
      name: "Anthropic models",
      category: "AI",
      kind: "http",
      endpoint: "https://api.anthropic.com/v1/models",
      documentationUrl: "https://docs.anthropic.com/en/api/models",
      requiredEnv: ["ANTHROPIC_API_KEY"],
      optionalEnv: [],
    },
    (env) => ({
      endpoint: "https://api.anthropic.com/v1/models",
      headers: {
        "x-api-key": env.ANTHROPIC_API_KEY ?? "",
        "anthropic-version": "2023-06-01",
      },
      expectedStatuses: [200],
      evidence: "model-list-readable",
    }),
  ),
  httpProbe(
    {
      id: "google-gemini-models",
      providerId: "google-gemini",
      packageId: "ai/google-gemini",
      name: "Google Gemini models",
      category: "AI",
      kind: "http",
      endpoint: "https://generativelanguage.googleapis.com/v1beta/models",
      documentationUrl: "https://ai.google.dev/api/models",
      requiredEnv: ["GOOGLE_GENERATIVE_AI_API_KEY"],
      optionalEnv: [],
    },
    (env) => ({
      endpoint: `https://generativelanguage.googleapis.com/v1beta/models?key=${encodeURIComponent(
        env.GOOGLE_GENERATIVE_AI_API_KEY ?? "",
      )}`,
      expectedStatuses: [200],
      evidence: "model-list-readable",
    }),
  ),
];
