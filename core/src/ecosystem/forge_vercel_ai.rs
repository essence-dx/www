pub(crate) const VERCEL_AI_VERSION: &str = "7.0.0-canary.146-dx.0";

pub(crate) fn vercel_ai_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "js/lib/ai/model.ts",
            r#"import type { LanguageModel } from "ai";

export type DxAIModelConfig = {
  model: LanguageModel;
  system?: string;
};

export type DxAIModelFactory = () => DxAIModelConfig;

export function createDxAIModelConfig(config: DxAIModelConfig): DxAIModelFactory {
  return () => config;
}
"#,
        ),
        (
            "js/lib/ai/model-policy.ts",
            r#"import {
  defaultSettingsMiddleware,
  wrapLanguageModel,
  type LanguageModel,
  type LanguageModelMiddleware,
} from "ai";

export type DxLaunchModelPolicyOptions = {
  maxOutputTokens?: number;
  temperature?: number;
  topP?: number;
  providerOptions?: Record<string, unknown>;
};

export function createDxLaunchModelPolicy({
  maxOutputTokens = 800,
  temperature = 0.2,
  topP,
  providerOptions,
}: DxLaunchModelPolicyOptions = {}): LanguageModelMiddleware {
  return defaultSettingsMiddleware({
    settings: {
      maxOutputTokens,
      temperature,
      topP,
      providerOptions,
    },
  });
}

export function withDxLaunchModelPolicy(
  model: LanguageModel,
  options: DxLaunchModelPolicyOptions = {},
): LanguageModel {
  return wrapLanguageModel({
    model,
    middleware: createDxLaunchModelPolicy(options),
  });
}
"#,
        ),
        (
            "js/lib/ai/tools.ts",
            r#"import { tool } from "ai";
import { z } from "zod";

export type DxLaunchStatus = {
  current: string;
  next: string;
  score: number;
};

export function createDxLaunchTools(readStatus: () => DxLaunchStatus) {
  return {
    launchStatus: tool({
      description: "Read the current DX launch status and next useful action.",
      inputSchema: z.object({
        area: z.string().optional(),
      }),
      execute: async () => readStatus(),
    }),
  };
}
"#,
        ),
        (
            "js/lib/ai/chat-route.ts",
            r#"import { convertToModelMessages, streamText, tool } from "ai";
import type { UIMessage } from "ai";
import { z } from "zod";

import type { DxAIModelFactory } from "./model";
import { createDxLaunchTools, type DxLaunchStatus } from "./tools";

export type DxAIChatRouteOptions = {
  model: DxAIModelFactory;
  readStatus?: () => DxLaunchStatus;
  messagePruner?: ReturnType<
    typeof import("./message-pruning").createDxLaunchMessagePruner
  >;
  toolApproval?: ReturnType<
    typeof import("./tool-approval").createDxLaunchToolApproval
  >;
  telemetry?: ReturnType<
    typeof import("./telemetry").createDxLaunchTelemetryOptions
  >;
};

export function createDxAIChatRoute({
  model,
  readStatus,
  messagePruner,
  toolApproval,
  telemetry,
}: DxAIChatRouteOptions) {
  return async function POST(request: Request): Promise<Response> {
    const body = (await request.json()) as { messages?: UIMessage[] };
    const messages = body.messages ?? [];
    const config = model();
    const modelMessages = convertToModelMessages(messages);

    const result = streamText({
      model: config.model,
      system: config.system,
      messages: messagePruner ? messagePruner(modelMessages) : modelMessages,
      toolApproval,
      telemetry,
      tools: readStatus
        ? createDxLaunchTools(readStatus)
        : {
            launchStatus: tool({
              description: "Explain why launch status is not connected yet.",
              inputSchema: z.object({}),
              execute: async () => ({
                current: "AI route is wired; connect readStatus for live launch evidence.",
                next: "Pass readStatus to createDxAIChatRoute.",
                score: 80,
              }),
            }),
          },
    });

    return result.toUIMessageStreamResponse();
  };
}
"#,
        ),
        (
            "js/lib/ai/message-pruning.ts",
            r#"import { pruneMessages, type ModelMessage } from "ai";

export type DxLaunchMessagePruningOptions = {
  reasoning?: "all" | "before-last-message" | "none";
  toolCalls?:
    | "all"
    | "before-last-message"
    | `before-last-${number}-messages`
    | "none"
    | Array<{
        type: "all" | "before-last-message" | `before-last-${number}-messages`;
        tools?: string[];
      }>;
  emptyMessages?: "keep" | "remove";
};

export function createDxLaunchMessagePruner({
  reasoning = "before-last-message",
  toolCalls = "before-last-2-messages",
  emptyMessages = "remove",
}: DxLaunchMessagePruningOptions = {}) {
  return (messages: ModelMessage[]): ModelMessage[] =>
    pruneMessages({
      messages,
      reasoning,
      toolCalls,
      emptyMessages,
    });
}
"#,
        ),
        (
            "js/lib/ai/tool-approval.ts",
            r#"import type {
  ToolApprovalConfiguration,
  ToolApprovalRequest,
  ToolApprovalResponse,
  ToolApprovalStatus,
  ToolSet,
} from "ai";

export type DxLaunchToolApprovalConfig = {
  autoApproveTools?: string[];
  denyTools?: Record<string, string>;
  requireUserApprovalByDefault?: boolean;
};

export function createDxLaunchToolApproval({
  autoApproveTools = [],
  denyTools = {},
  requireUserApprovalByDefault = true,
}: DxLaunchToolApprovalConfig = {}): ToolApprovalConfiguration<ToolSet, unknown> {
  const approved = new Set(autoApproveTools);

  return ({ toolCall }): ToolApprovalStatus => {
    const deniedReason = denyTools[toolCall.toolName];
    if (deniedReason) {
      return { type: "denied", reason: deniedReason };
    }

    if (approved.has(toolCall.toolName)) {
      return "approved";
    }

    return requireUserApprovalByDefault ? "user-approval" : "not-applicable";
  };
}

export function createDxToolApprovalResponse({
  request,
  approved,
  reason,
}: {
  request: ToolApprovalRequest;
  approved: boolean;
  reason?: string;
}): ToolApprovalResponse {
  return {
    type: "tool-approval-response",
    approvalId: request.approvalId,
    approved,
    reason,
  };
}
"#,
        ),
        (
            "js/lib/ai/image-generation.ts",
            r#"import {
  generateImage,
  type GenerateImageResult,
  type ImageModel,
} from "ai";

export type DxLaunchImageGenerationInput = {
  imageModel: ImageModel;
  prompt: Parameters<typeof generateImage>[0]["prompt"];
  n?: number;
  size?: `${number}x${number}`;
  aspectRatio?: `${number}:${number}`;
  seed?: number;
  providerOptions?: Parameters<typeof generateImage>[0]["providerOptions"];
  maxImagesPerCall?: number;
  maxRetries?: number;
};

export type DxLaunchImageAsset = {
  mediaType: string;
  base64: string;
  imageCount: number;
  warnings: GenerateImageResult["warnings"];
  usage: GenerateImageResult["usage"];
  providerMetadata: GenerateImageResult["providerMetadata"];
};

export async function generateDxLaunchImages({
  imageModel,
  prompt,
  n = 1,
  size,
  aspectRatio,
  seed,
  providerOptions,
  maxImagesPerCall,
  maxRetries,
}: DxLaunchImageGenerationInput): Promise<GenerateImageResult> {
  return generateImage({
    model: imageModel,
    prompt,
    n,
    size,
    aspectRatio,
    seed,
    providerOptions,
    maxImagesPerCall,
    maxRetries,
  });
}

export async function generateDxLaunchImageAsset(
  input: DxLaunchImageGenerationInput,
): Promise<DxLaunchImageAsset> {
  const result = await generateDxLaunchImages(input);

  return {
    mediaType: result.image.mediaType,
    base64: result.image.base64,
    imageCount: result.images.length,
    warnings: result.warnings,
    usage: result.usage,
    providerMetadata: result.providerMetadata,
  };
}
"#,
        ),
        (
            "js/lib/ai/speech-generation.ts",
            r#"import {
  experimental_generateSpeech as generateSpeech,
  type Experimental_SpeechResult,
  type GeneratedAudioFile,
  type SpeechModel,
} from "ai";

export type DxLaunchSpeechGenerationInput = {
  speechModel: SpeechModel;
  text: string;
  voice?: string;
  outputFormat?: "mp3" | "wav" | (string & {});
  instructions?: string;
  speed?: number;
  language?: string;
  providerOptions?: Parameters<typeof generateSpeech>[0]["providerOptions"];
  maxRetries?: number;
};

export type DxLaunchSpeechAudio = {
  mediaType: string;
  format: string;
  base64: string;
  warnings: Experimental_SpeechResult["warnings"];
  providerMetadata: Experimental_SpeechResult["providerMetadata"];
};

export async function generateDxLaunchSpeech({
  speechModel,
  text,
  voice,
  outputFormat,
  instructions,
  speed,
  language,
  providerOptions,
  maxRetries,
}: DxLaunchSpeechGenerationInput): Promise<Experimental_SpeechResult> {
  return generateSpeech({
    model: speechModel,
    text,
    voice,
    outputFormat,
    instructions,
    speed,
    language,
    providerOptions,
    maxRetries,
  });
}

export function toDxLaunchSpeechAudio(
  audio: GeneratedAudioFile,
  result: Pick<Experimental_SpeechResult, "warnings" | "providerMetadata">,
): DxLaunchSpeechAudio {
  return {
    mediaType: audio.mediaType,
    format: audio.format,
    base64: audio.base64,
    warnings: result.warnings,
    providerMetadata: result.providerMetadata,
  };
}

export async function generateDxLaunchSpeechAudio(
  input: DxLaunchSpeechGenerationInput,
): Promise<DxLaunchSpeechAudio> {
  const result = await generateDxLaunchSpeech(input);
  return toDxLaunchSpeechAudio(result.audio, result);
}
"#,
        ),
        (
            "js/lib/ai/transcription.ts",
            r#"import {
  createDownload,
  experimental_transcribe as transcribe,
  type Experimental_TranscriptionResult,
  type TranscriptionModel,
} from "ai";

export type DxLaunchTranscriptionInput = {
  transcriptionModel: TranscriptionModel;
  audio: Parameters<typeof transcribe>[0]["audio"];
  providerOptions?: Parameters<typeof transcribe>[0]["providerOptions"];
  maxRetries?: number;
  maxDownloadBytes?: number;
};

export type DxLaunchTranscript = {
  text: string;
  language: Experimental_TranscriptionResult["language"];
  durationInSeconds: Experimental_TranscriptionResult["durationInSeconds"];
  segments: Experimental_TranscriptionResult["segments"];
  warnings: Experimental_TranscriptionResult["warnings"];
  providerMetadata: Experimental_TranscriptionResult["providerMetadata"];
};

export async function transcribeDxLaunchAudio({
  transcriptionModel,
  audio,
  providerOptions,
  maxRetries,
  maxDownloadBytes = 25 * 1024 * 1024,
}: DxLaunchTranscriptionInput): Promise<DxLaunchTranscript> {
  const result = await transcribe({
    model: transcriptionModel,
    audio,
    providerOptions,
    maxRetries,
    download: createDownload({
      maxBytes: maxDownloadBytes,
    }),
  });

  return {
    text: result.text,
    language: result.language,
    durationInSeconds: result.durationInSeconds,
    segments: result.segments,
    warnings: result.warnings,
    providerMetadata: result.providerMetadata,
  };
}
"#,
        ),
        (
            "js/lib/ai/video-generation.ts",
            r#"import {
  createDownload,
  experimental_generateVideo as generateVideo,
  type GenerateVideoPrompt,
  type GenerateVideoResult,
  type VideoModel,
} from "ai";

export type DxLaunchVideoGenerationInput = {
  videoModel: VideoModel;
  prompt: GenerateVideoPrompt;
  n?: number;
  aspectRatio?: `${number}:${number}`;
  resolution?: `${number}x${number}`;
  duration?: number;
  fps?: number;
  seed?: number;
  providerOptions?: Parameters<typeof generateVideo>[0]["providerOptions"];
  maxVideosPerCall?: number;
  maxRetries?: number;
  maxDownloadBytes?: number;
};

export type DxLaunchVideoAsset = {
  mediaType: string;
  base64: string;
  videoCount: number;
  warnings: GenerateVideoResult["warnings"];
  providerMetadata: GenerateVideoResult["providerMetadata"];
};

export async function generateDxLaunchVideos({
  videoModel,
  prompt,
  n = 1,
  aspectRatio,
  resolution,
  duration,
  fps,
  seed,
  providerOptions,
  maxVideosPerCall,
  maxRetries,
  maxDownloadBytes = 50 * 1024 * 1024,
}: DxLaunchVideoGenerationInput): Promise<GenerateVideoResult> {
  return generateVideo({
    model: videoModel,
    prompt,
    n,
    aspectRatio,
    resolution,
    duration,
    fps,
    seed,
    providerOptions,
    maxVideosPerCall,
    maxRetries,
    download: createDownload({
      maxBytes: maxDownloadBytes,
    }),
  });
}

export async function generateDxLaunchVideoAsset(
  input: DxLaunchVideoGenerationInput,
): Promise<DxLaunchVideoAsset> {
  const result = await generateDxLaunchVideos(input);

  return {
    mediaType: result.video.mediaType,
    base64: result.video.base64,
    videoCount: result.videos.length,
    warnings: result.warnings,
    providerMetadata: result.providerMetadata,
  };
}
"#,
        ),
        (
            "js/lib/ai/object-generation.ts",
            r#"import {
  generateObject,
  streamObject,
  type FlexibleSchema,
  type GenerateObjectResult,
  type InferSchema,
  type LanguageModel,
  type StreamObjectResult,
} from "ai";

export type DxLaunchObjectGenerationInput<
  SCHEMA extends FlexibleSchema<unknown>,
> = {
  model: LanguageModel;
  schema: SCHEMA;
  prompt: string;
  system?: string;
  schemaName?: string;
  schemaDescription?: string;
  providerOptions?: Parameters<typeof generateObject>[0]["providerOptions"];
  maxRetries?: number;
};

export async function generateDxLaunchObject<
  SCHEMA extends FlexibleSchema<unknown>,
>({
  model,
  schema,
  prompt,
  system,
  schemaName,
  schemaDescription,
  providerOptions,
  maxRetries,
}: DxLaunchObjectGenerationInput<SCHEMA>): Promise<
  GenerateObjectResult<InferSchema<SCHEMA>>
> {
  return generateObject({
    model,
    schema,
    prompt,
    system,
    schemaName,
    schemaDescription,
    providerOptions,
    maxRetries,
  });
}

export function streamDxLaunchObject<SCHEMA extends FlexibleSchema<unknown>>({
  model,
  schema,
  prompt,
  system,
  schemaName,
  schemaDescription,
  providerOptions,
  maxRetries,
}: DxLaunchObjectGenerationInput<SCHEMA>): StreamObjectResult<
  Partial<InferSchema<SCHEMA>>,
  InferSchema<SCHEMA>,
  never
> {
  return streamObject({
    model,
    schema,
    prompt,
    system,
    schemaName,
    schemaDescription,
    providerOptions,
    maxRetries,
  });
}
"#,
        ),
        (
            "js/lib/ai/structured-output.ts",
            r#"import { generateText, Output } from "ai";
import { z } from "zod";

import type { DxAIModelFactory } from "./model";

export const dxLaunchStructuredStatusSchema = z.object({
  current: z.string(),
  next: z.string(),
  score: z.number().min(0).max(100),
  risks: z.array(z.string()),
});

export type DxLaunchStructuredStatus = z.infer<typeof dxLaunchStructuredStatusSchema>;

export type DxLaunchStructuredStatusOptions = {
  model: DxAIModelFactory;
  prompt?: string;
  readStatus?: () => {
    current: string;
    next: string;
    score: number;
    risks?: string[];
  };
};

export async function generateDxLaunchStructuredStatus({
  model,
  prompt,
  readStatus,
}: DxLaunchStructuredStatusOptions): Promise<DxLaunchStructuredStatus> {
  const config = model();
  const seed = readStatus?.();
  const result = await generateText({
    model: config.model,
    system: config.system,
    prompt:
      prompt ??
      `Return a DX launch status object from this evidence: ${JSON.stringify({
        current: seed?.current ?? "DX launch AI structured output is wired.",
        next: seed?.next ?? "Connect live receipts before production launch.",
        score: seed?.score ?? 80,
        risks: seed?.risks ?? ["live provider credentials are app-owned"],
      })}`,
    output: Output.object({
      schema: dxLaunchStructuredStatusSchema,
    }),
  });

  return result.output;
}
"#,
        ),
        (
            "js/lib/ai/telemetry.ts",
            r#"import {
  registerTelemetry,
  type Telemetry,
  type TelemetryOptions,
} from "ai";

export type DxLaunchTelemetryEvent = {
  event:
    | "start"
    | "step-finish"
    | "tool-start"
    | "tool-end"
    | "embed-start"
    | "embed-end"
    | "end"
    | "error";
  at: string;
  functionId: string;
  operationId?: string;
  stepNumber?: number;
  toolName?: string;
  tokenCount?: number;
  success?: boolean;
};

export type DxLaunchTelemetrySink = (
  event: DxLaunchTelemetryEvent,
) => void | Promise<void>;

export type DxLaunchTelemetryConfig = {
  functionId?: string;
  sink: DxLaunchTelemetrySink;
  recordInputs?: boolean;
  recordOutputs?: boolean;
  includeRuntimeContext?: TelemetryOptions["includeRuntimeContext"];
};

type ObservedTelemetryEvent = {
  operationId?: string;
  stepNumber?: number;
  usage?: {
    totalTokens?: number;
    tokens?: number;
  };
  toolCall?: {
    toolName?: string;
  };
  toolOutput?: {
    type?: string;
  };
};

function now() {
  return new Date().toISOString();
}

function tokenCount(event: ObservedTelemetryEvent) {
  return event.usage?.totalTokens ?? event.usage?.tokens;
}

function emit(
  sink: DxLaunchTelemetrySink,
  event: DxLaunchTelemetryEvent,
) {
  return sink(event);
}

export function createDxLaunchTelemetry({
  functionId = "dx.launch.ai",
  sink,
}: DxLaunchTelemetryConfig): Telemetry {
  return {
    onStart: (event) =>
      emit(sink, {
        event: "start",
        at: now(),
        functionId,
        operationId: (event as ObservedTelemetryEvent).operationId,
      }),
    onStepFinish: (event) =>
      emit(sink, {
        event: "step-finish",
        at: now(),
        functionId,
        operationId: (event as ObservedTelemetryEvent).operationId,
        stepNumber: (event as ObservedTelemetryEvent).stepNumber,
        tokenCount: tokenCount(event as ObservedTelemetryEvent),
      }),
    onToolExecutionStart: (event) =>
      emit(sink, {
        event: "tool-start",
        at: now(),
        functionId,
        toolName: (event as ObservedTelemetryEvent).toolCall?.toolName,
      }),
    onToolExecutionEnd: (event) =>
      emit(sink, {
        event: "tool-end",
        at: now(),
        functionId,
        toolName: (event as ObservedTelemetryEvent).toolCall?.toolName,
        success: (event as ObservedTelemetryEvent).toolOutput?.type === "tool-result",
      }),
    onEmbedStart: (event) =>
      emit(sink, {
        event: "embed-start",
        at: now(),
        functionId,
        operationId: (event as ObservedTelemetryEvent).operationId,
      }),
    onEmbedEnd: (event) =>
      emit(sink, {
        event: "embed-end",
        at: now(),
        functionId,
        operationId: (event as ObservedTelemetryEvent).operationId,
        tokenCount: tokenCount(event as ObservedTelemetryEvent),
      }),
    onEnd: (event) =>
      emit(sink, {
        event: "end",
        at: now(),
        functionId,
        operationId: (event as ObservedTelemetryEvent).operationId,
        tokenCount: tokenCount(event as ObservedTelemetryEvent),
      }),
    onError: () =>
      emit(sink, {
        event: "error",
        at: now(),
        functionId,
      }),
  };
}

export function createDxLaunchTelemetryOptions({
  functionId = "dx.launch.ai",
  sink,
  recordInputs = false,
  recordOutputs = false,
  includeRuntimeContext = {
    launchId: true,
    requestId: true,
  },
}: DxLaunchTelemetryConfig): TelemetryOptions {
  return {
    functionId,
    recordInputs,
    recordOutputs,
    includeRuntimeContext,
    integrations: [createDxLaunchTelemetry({ functionId, sink })],
  };
}

export function registerDxLaunchTelemetry(config: DxLaunchTelemetryConfig) {
  registerTelemetry(createDxLaunchTelemetry(config));
}
"#,
        ),
        (
            "js/lib/ai/embeddings.ts",
            r#"import {
  cosineSimilarity,
  embed,
  embedMany,
  type Embedding,
  type EmbeddingModel,
} from "ai";

export type DxLaunchEmbeddingSearchInput = {
  embeddingModel: EmbeddingModel;
  query: string;
  notes: Array<{
    id: string;
    text: string;
  }>;
  minSimilarity?: number;
};

export type DxLaunchEmbeddingSearchResult = {
  id: string;
  text: string;
  similarity: number;
};

export async function embedDxLaunchQuery(
  embeddingModel: EmbeddingModel,
  query: string,
): Promise<Embedding> {
  const { embedding } = await embed({
    model: embeddingModel,
    value: query,
  });

  return embedding;
}

export async function rankDxLaunchNotesBySimilarity({
  embeddingModel,
  query,
  notes,
  minSimilarity = 0,
}: DxLaunchEmbeddingSearchInput): Promise<DxLaunchEmbeddingSearchResult[]> {
  if (notes.length === 0) {
    return [];
  }

  const [queryEmbedding, notesResult] = await Promise.all([
    embedDxLaunchQuery(embeddingModel, query),
    embedMany({
      model: embeddingModel,
      values: notes.map((note) => note.text),
    }),
  ]);

  return notes
    .map((note, index) => ({
      ...note,
      similarity: cosineSimilarity(queryEmbedding, notesResult.embeddings[index]),
    }))
    .filter((note) => note.similarity >= minSimilarity)
    .sort((a, b) => b.similarity - a.similarity);
}
"#,
        ),
        (
            "js/lib/ai/reranking.ts",
            r#"import { rerank, type RerankingModel } from "ai";

export type DxLaunchEvidenceDocument = {
  id: string;
  title: string;
  text: string;
  source?: string;
};

export type DxLaunchRerankInput = {
  rerankingModel: RerankingModel;
  query: string;
  documents: DxLaunchEvidenceDocument[];
  topN?: number;
};

export type DxLaunchRerankResult = {
  id: string;
  title: string;
  text: string;
  source?: string;
  score: number;
  originalIndex: number;
};

export async function rerankDxLaunchEvidence({
  rerankingModel,
  query,
  documents,
  topN,
}: DxLaunchRerankInput): Promise<DxLaunchRerankResult[]> {
  if (documents.length === 0) {
    return [];
  }

  const { ranking, rerankedDocuments } = await rerank({
    model: rerankingModel,
    documents,
    query,
    topN,
  });

  return rerankedDocuments.map((document, index) => ({
    id: document.id,
    title: document.title,
    text: document.text,
    source: document.source,
    score: ranking[index].score,
    originalIndex: ranking[index].originalIndex,
  }));
}
"#,
        ),
        (
            "js/lib/ai/agent.ts",
            r#"import {
  createAgentUIStreamResponse,
  ToolLoopAgent,
  type InferAgentUIMessage,
  type LanguageModel,
  type ToolSet,
} from "ai";

import { createDxLaunchTools, type DxLaunchStatus } from "./tools";

export type DxLaunchAgentOptions = {
  model: LanguageModel;
  instructions?: string;
  readStatus?: () => DxLaunchStatus;
  toolApproval?: ReturnType<
    typeof import("./tool-approval").createDxLaunchToolApproval
  >;
  tools?: ToolSet;
};

export function createDxLaunchAgent({
  model,
  instructions = "You are the DX launch assistant. Use tools to report launch readiness and next useful work.",
  readStatus,
  toolApproval,
  tools = {},
}: DxLaunchAgentOptions) {
  return new ToolLoopAgent({
    model,
    instructions,
    toolApproval,
    tools: {
      ...(readStatus ? createDxLaunchTools(readStatus) : {}),
      ...tools,
    },
  });
}

export type DxLaunchAgent = ReturnType<typeof createDxLaunchAgent>;
export type DxLaunchAgentUIMessage = InferAgentUIMessage<DxLaunchAgent>;

export function createDxAgentRoute(agent: DxLaunchAgent) {
  return async function POST(request: Request): Promise<Response> {
    const body = (await request.json()) as { messages?: DxLaunchAgentUIMessage[] };

    return createAgentUIStreamResponse({
      agent,
      uiMessages: body.messages ?? [],
    });
  };
}
"#,
        ),
        (
            "js/lib/ai/provider-freedom.ts",
            r#"import {
  createGateway,
  createProviderRegistry,
  customProvider,
  gateway,
  type EmbeddingModel,
  type LanguageModel,
} from "ai";

export type DxProviderFreedomConfig = {
  apiKey?: string;
  languageModels?: Record<string, LanguageModel>;
  embeddingModels?: Record<string, EmbeddingModel>;
};

export function createDxGatewayProvider(config: Pick<DxProviderFreedomConfig, "apiKey"> = {}) {
  return createGateway({
    apiKey: config.apiKey,
  });
}

export function createDxLaunchProvider({
  languageModels = {
    "launch-fast": gateway("openai/gpt-5-mini"),
    "launch-reasoning": gateway("anthropic/claude-sonnet-4.5"),
  },
  embeddingModels = {
    "launch-embedding": gateway.embeddingModel("openai/text-embedding-3-small"),
  },
}: DxProviderFreedomConfig = {}) {
  return customProvider({
    languageModels,
    embeddingModels,
    fallbackProvider: gateway,
  });
}

export function createDxProviderRegistry(
  config: DxProviderFreedomConfig = {},
) {
  return createProviderRegistry({
    gateway,
    dx: createDxLaunchProvider(config),
  });
}
"#,
        ),
        (
            "js/lib/ai/ui-message-stream.ts",
            r#"import {
  createUIMessageStream,
  createUIMessageStreamResponse,
  pipeUIMessageStreamToResponse,
  readUIMessageStream,
  UI_MESSAGE_STREAM_HEADERS,
  type UIMessage,
  type UIMessageChunk,
  type UIMessageStreamWriter,
} from "ai";

export type DxLaunchUIMessageStreamExecute<
  UI_MESSAGE extends UIMessage = UIMessage,
> = (writer: UIMessageStreamWriter<UI_MESSAGE>) => Promise<void> | void;

export type DxLaunchUIMessageStreamOptions<
  UI_MESSAGE extends UIMessage = UIMessage,
> = {
  execute: DxLaunchUIMessageStreamExecute<UI_MESSAGE>;
  originalMessages?: UI_MESSAGE[];
  onError?: (error: unknown) => string;
};

export function createDxLaunchUIMessageStream<
  UI_MESSAGE extends UIMessage = UIMessage,
>({
  execute,
  originalMessages,
  onError,
}: DxLaunchUIMessageStreamOptions<UI_MESSAGE>): ReadableStream<UIMessageChunk> {
  return createUIMessageStream<UI_MESSAGE>({
    originalMessages,
    onError,
    execute: ({ writer }) => execute(writer),
  });
}

export function createDxLaunchUIMessageStreamResponse({
  stream,
  status = 200,
  statusText,
  headers,
  consumeSseStream,
}: {
  stream: ReadableStream<UIMessageChunk>;
  status?: number;
  statusText?: string;
  headers?: HeadersInit;
  consumeSseStream?: Parameters<typeof createUIMessageStreamResponse>[0]["consumeSseStream"];
}): Response {
  return createUIMessageStreamResponse({
    stream,
    status,
    statusText,
    headers,
    consumeSseStream,
  });
}

export function pipeDxLaunchUIMessageStreamToResponse(
  options: Parameters<typeof pipeUIMessageStreamToResponse>[0],
): void {
  pipeUIMessageStreamToResponse(options);
}

export function readDxLaunchUIMessageStream<
  UI_MESSAGE extends UIMessage = UIMessage,
>(options: Parameters<typeof readUIMessageStream<UI_MESSAGE>>[0]) {
  return readUIMessageStream<UI_MESSAGE>(options);
}

export { UI_MESSAGE_STREAM_HEADERS };
"#,
        ),
        (
            "js/lib/ai/text-stream.ts",
            r#"import {
  createTextStreamResponse,
  pipeTextStreamToResponse,
} from "ai";

export type DxLaunchTextStreamSource = Iterable<string> | AsyncIterable<string>;

export function createDxLaunchTextStream({
  chunks,
}: {
  chunks: DxLaunchTextStreamSource;
}): ReadableStream<string> {
  const iterator =
    Symbol.asyncIterator in chunks
      ? chunks[Symbol.asyncIterator]()
      : (async function* () {
          yield* chunks;
        })();

  return new ReadableStream<string>({
    async pull(controller) {
      const next = await iterator.next();

      if (next.done) {
        controller.close();
        return;
      }

      controller.enqueue(next.value);
    },
  });
}

export function createDxLaunchTextStreamResponse({
  textStream,
  status = 200,
  statusText,
  headers,
}: {
  textStream: ReadableStream<string>;
  status?: number;
  statusText?: string;
  headers?: HeadersInit;
}): Response {
  return createTextStreamResponse({
    textStream,
    status,
    statusText,
    headers,
  });
}

export function pipeDxLaunchTextStreamToResponse(
  options: Parameters<typeof pipeTextStreamToResponse>[0],
): void {
  pipeTextStreamToResponse(options);
}
"#,
        ),
        (
            "js/lib/ai/file-upload.ts",
            r#"import { uploadFile, type UploadFileResult } from "ai";

export type DxLaunchFileUploadInput = {
  api: Parameters<typeof uploadFile>[0]["api"];
  data: Parameters<typeof uploadFile>[0]["data"];
  filename?: string;
  mediaType?: string;
  providerOptions?: Parameters<typeof uploadFile>[0]["providerOptions"];
  maxBytes?: number;
};

export async function uploadDxLaunchFile({
  api,
  data,
  filename,
  mediaType,
  providerOptions,
  maxBytes,
}: DxLaunchFileUploadInput): Promise<UploadFileResult> {
  if (maxBytes !== undefined && estimateUploadBytes(data) > maxBytes) {
    throw new Error("DX file upload exceeds the configured maxBytes limit.");
  }

  return uploadFile({
    api,
    data,
    filename,
    mediaType,
    providerOptions,
  });
}

function estimateUploadBytes(data: DxLaunchFileUploadInput["data"]): number {
  if (typeof data === "string") {
    return data.length;
  }

  if (data instanceof Uint8Array) {
    return data.byteLength;
  }

  const payload = data.data;

  if (typeof payload === "string") {
    return payload.length;
  }

  return payload.byteLength;
}
"#,
        ),
        (
            "js/lib/ai/client-chat.tsx",
            r#""use client";

import * as React from "react";
import { DefaultChatTransport, type UIMessage } from "ai";

export type DxAIClientChatProps = {
  api?: string;
  initialMessages?: UIMessage[];
  placeholder?: string;
};

export function DxAIClientChat({
  api = "/api/ai/chat",
  initialMessages = [],
  placeholder = "Ask DX",
}: DxAIClientChatProps) {
  const [messages, setMessages] = React.useState<UIMessage[]>(initialMessages);
  const [input, setInput] = React.useState("");
  const [status, setStatus] = React.useState<"ready" | "streaming" | "error">("ready");
  const transport = React.useMemo(() => new DefaultChatTransport({ api }), [api]);

  async function submit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const text = input.trim();
    if (!text || status === "streaming") {
      return;
    }

    const nextMessages = [
      ...messages,
      {
        id: crypto.randomUUID(),
        role: "user" as const,
        parts: [{ type: "text" as const, text }],
      },
    ];
    setMessages(nextMessages);
    setInput("");
    setStatus("streaming");

    try {
      await transport.sendMessages({
        messages: nextMessages,
        abortSignal: undefined,
      });
      setStatus("ready");
    } catch {
      setStatus("error");
    }
  }

  return (
    <section className="grid gap-3 rounded-lg border border-neutral-200 p-4">
      <div className="grid gap-2">
        {messages.map((message) => (
          <div key={message.id} className="text-sm">
            <span className="font-medium">{message.role}</span>{" "}
            {message.parts
              .filter((part) => part.type === "text")
              .map((part) => part.text)
              .join("")}
          </div>
        ))}
      </div>
      <form className="flex gap-2" onSubmit={submit}>
        <input
          className="min-w-0 flex-1 rounded-md border border-neutral-300 px-3 py-2 text-sm"
          value={input}
          onChange={(event) => setInput(event.currentTarget.value)}
          placeholder={placeholder}
        />
        <button
          className="rounded-md bg-neutral-950 px-3 py-2 text-sm font-medium text-white disabled:opacity-50"
          disabled={status === "streaming"}
          type="submit"
        >
          Send
        </button>
      </form>
    </section>
  );
}
"#,
        ),
        (
            "js/lib/ai/dashboard-readiness.ts",
            r#"export type DxAiDashboardProviderId = "openai-compatible" | "gateway";

export type DxAiDashboardProviderReadiness = {
  id: DxAiDashboardProviderId;
  label: string;
  requiredEnv: string[];
  publicApi: string[];
};

export type DxAiDashboardReceipt = {
  provider: DxAiDashboardProviderId;
  status: "missing-config" | "ready";
  promptLength: number;
  nextAction: string;
};

export const dxAiDashboardProviders: DxAiDashboardProviderReadiness[] = [
  {
    id: "openai-compatible",
    label: "OpenAI-compatible",
    requiredEnv: ["AI_PROVIDER_API_KEY"],
    publicApi: ["streamText", "convertToModelMessages", "tool"],
  },
  {
    id: "gateway",
    label: "AI Gateway",
    requiredEnv: ["AI_GATEWAY_API_KEY"],
    publicApi: ["gateway", "createGateway", "createProviderRegistry"],
  },
];

export function createDxAiDashboardReceipt({
  provider,
  prompt,
}: {
  provider: DxAiDashboardProviderId;
  prompt: string;
}): DxAiDashboardReceipt {
  const readiness = dxAiDashboardProviders.find((item) => item.id === provider);

  return {
    provider,
    status: "missing-config",
    promptLength: prompt.trim().length,
    nextAction: `Configure ${readiness?.requiredEnv.join(", ") || "AI provider env"} before streaming model output.`,
  };
}
"#,
        ),
        (
            "js/components/ai/ai-launch-assistant.tsx",
            r#""use client";

import * as React from "react";

import {
  createDxAiDashboardReceipt,
  dxAiDashboardProviders,
  type DxAiDashboardProviderId,
  type DxAiDashboardReceipt,
} from "@/lib/ai/dashboard-readiness";

export function AiLaunchAssistant() {
  const [provider, setProvider] = React.useState<DxAiDashboardProviderId>("openai-compatible");
  const [prompt, setPrompt] = React.useState("Summarize dashboard risks before launch.");
  const [receipt, setReceipt] = React.useState<DxAiDashboardReceipt | null>(null);
  const selectedProvider =
    dxAiDashboardProviders.find((item) => item.id === provider) ?? dxAiDashboardProviders[0];

  return (
    <section
      className="dx-ai-assistant-panel"
      data-dx-ai-config-state="missing-config"
      data-dx-ai-dashboard-workflow="launch-risk-review"
      data-dx-ai-provider={provider}
      data-dx-component="dashboard-ai-launch-assistant"
      data-dx-style-surface="ai-sdk"
      data-dx-token-scope="ai/vercel-ai"
      data-dx-node-modules="forbidden"
      data-dx-package="ai/vercel-ai"
    >
      <header className="dx-panel-header">
        <dx-icon aria-label="AI" name="pack:ai" />
        <div>
          <h2>AI SDK launch assistant</h2>
          <p>Review dashboard risk through the official AI SDK boundary.</p>
        </div>
      </header>

      <div className="dx-provider-options" data-dx-ai-interaction="provider-picker">
        {dxAiDashboardProviders.map((item) => (
          <button
            className={item.id === provider ? "is-active" : undefined}
            data-dx-ai-provider-choice={item.id}
            data-dx-ai-provider-selected={item.id === provider ? "true" : "false"}
            key={item.id}
            onClick={() => {
              setProvider(item.id);
              setReceipt(null);
            }}
            type="button"
          >
            <dx-icon aria-hidden="true" name="pack:settings" />
            {item.label}
          </button>
        ))}
      </div>

      <dl className="dx-readiness-list" data-dx-ai-provider-readiness="app-owned">
        <div>
          <dt>Required env</dt>
          <dd data-dx-ai-required-env={selectedProvider.requiredEnv.join(",")}>
            {selectedProvider.requiredEnv.join(", ")}
          </dd>
        </div>
        <div>
          <dt>Public APIs</dt>
          <dd data-dx-ai-public-api={selectedProvider.publicApi.join(",")}>
            {selectedProvider.publicApi.join(", ")}
          </dd>
        </div>
      </dl>

      <label className="dx-prompt-field" data-dx-ai-interaction="prompt-field">
        Launch prompt
        <textarea
          data-dx-ai-prompt-input="dashboard-launch-risk"
          onChange={(event) => setPrompt(event.currentTarget.value)}
          value={prompt}
        />
      </label>

      <button
        className="dx-primary-action"
        data-dx-ai-action="safe-local-preview"
        onClick={() => setReceipt(createDxAiDashboardReceipt({ provider, prompt }))}
        type="button"
      >
        <dx-icon aria-hidden="true" name="pack:play" />
        Preview response boundary
      </button>

      <p
        className="dx-assistant-receipt"
        data-dx-ai-local-response={receipt ? "missing-config" : "idle"}
        data-dx-ai-receipt-state={receipt ? receipt.status : "idle"}
      >
        {receipt
          ? `${receipt.provider} needs app-owned credentials. Prompt length: ${receipt.promptLength}. ${receipt.nextAction}`
          : "No AI request has been previewed yet."}
      </p>
    </section>
  );
}
"#,
        ),
        (
            "js/lib/ai/provider-boundary.ts",
            r#"export type DxAiProviderBoundaryOptions = {
  provider: "openai-compatible" | "gateway";
  capability: string;
  requiredEnv: string;
  appOwnedBoundary: string;
};

export type DxAiExtendedRouteBoundaryOptions = DxAiProviderBoundaryOptions & {
  route: string;
  enableEnv?: string;
  credentialsConfigured?: boolean;
};

export const DX_AI_EXTENDED_ROUTES_ENABLE_ENV = "DX_ENABLE_EXTENDED_AI_ROUTES";

export function isDxAiExtendedRouteEnabled(
  env: Record<string, string | undefined> = process.env,
): boolean {
  return env[DX_AI_EXTENDED_ROUTES_ENABLE_ENV] === "true";
}

export function createDxAiMissingProviderResponse({
  provider,
  capability,
  requiredEnv,
  appOwnedBoundary,
}: DxAiProviderBoundaryOptions): Response {
  return Response.json(
    {
      ok: false,
      status: "missing-config",
      httpStatus: 501,
      provider,
      capability,
      requiredEnv: [requiredEnv],
      credentialsConfigured: false,
      adapterBoundary: "provider-credential-boundary",
      runtimeExecution: false,
      modelStreaming: false,
      providerRuntime: false,
      secretValues: [],
      appOwnedBoundary,
    },
    { status: 501 },
  );
}

export function createDxAiExtendedRouteDisabledResponse({
  provider,
  capability,
  requiredEnv,
  appOwnedBoundary,
  route,
  enableEnv = DX_AI_EXTENDED_ROUTES_ENABLE_ENV,
  credentialsConfigured = false,
}: DxAiExtendedRouteBoundaryOptions): Response {
  return Response.json(
    {
      ok: false,
      status: "extended-route-disabled",
      httpStatus: 501,
      route,
      provider,
      capability,
      requiredEnv: [enableEnv, requiredEnv],
      credentialsConfigured,
      adapterBoundary: "extended-provider-route-boundary",
      proofSurface: "outside-default-ai-surface",
      defaultAiSurface: false,
      runtimeExecution: false,
      modelStreaming: false,
      providerRuntime: false,
      secretValues: [],
      appOwnedBoundary,
    },
    { status: 501 },
  );
}
"#,
        ),
        (
            "js/lib/ai/metadata.ts",
            r#"export const dxVercelAIForgePackage = {
  packageId: "ai/vercel-ai",
  officialName: "AI SDK",
  officialPackageName: "AI SDK",
  aliases: ["vercel-ai", "ai-sdk", "@vercel/ai"],
  upstreamPackage: "ai",
  upstreamVersion: "7.0.0-canary.146",
  sourceMirror: "G:/WWW/inspirations/vercel-ai",
  basedOn: "Vercel AI SDK",
  provenance: {
    repository: "https://github.com/vercel/ai",
    docs: "https://ai-sdk.dev/docs",
    license: "Apache-2.0",
  },
  inspectedSourceFiles: [
    "packages/ai/package.json",
    "packages/ai/src/index.ts",
    "packages/ai/src/generate-text/index.ts",
    "packages/ai/src/ui/index.ts",
    "packages/ai/src/registry/index.ts",
  ],
  selectedSurfaces: [
    "chat-route",
    "provider-readiness",
    "dashboard-workflow",
    "route-contract-preview",
  ],
  honestyLabel: "ADAPTER-BOUNDARY",
  dxCheckVisibility: {
    schema: "dx.forge.package.dx_check_visibility",
    currentStatus: "present",
    statuses: ["present", "stale", "missing-receipt", "blocked", "unsupported-surface"],
    receiptPath: "examples/template/.dx/forge/receipts/2026-05-22-ai-vercel-ai-launch-assistant.json",
    monitoredSurfaces: [
      "lib/ai/chat-route.ts",
      "lib/ai/dashboard-readiness.ts",
      "components/ai/ai-launch-assistant.tsx",
      "components/launch/ai-chat-status.tsx",
    ],
  },
  dxStyleCompatibility: {
    schema: "dx.forge.package.dx_style_compatibility",
    status: "present",
    tokenSource: "styles/theme.css",
    generatedCss: "styles/generated.css",
    visibleSurfaces: [
      "dashboard-ai-launch-assistant",
      "launch-ai-assistant-dashboard-workflow",
    ],
    sourceFiles: [
      "core/src/ecosystem/forge_vercel_ai.rs",
      "examples/template/ai-chat-status.tsx",
      "styles/theme.css",
      "styles/generated.css",
    ],
    runtimeProof: false,
    runtimeLimitations: [
      "SOURCE-ONLY: dx-style compatibility is source and receipt evidence; no live model streaming or browser visual proof is claimed.",
      "ADAPTER-BOUNDARY: provider-specific theme review, accessibility QA, and credential-backed runtime validation remain app-owned.",
    ],
  },
  requiredEnv: ["AI_PROVIDER_API_KEY"],
  optionalEnv: ["AI_GATEWAY_API_KEY"],
  appOwnedBoundaries: [
    "Provider API keys",
    "Model safety policy",
    "Prompt moderation",
    "Persistence and resume policy",
    "Rate limits and billing controls",
  ],
  receiptPaths: [
    ".dx/forge/receipts/*-ai-vercel-ai.json",
    ".dx/forge/docs/ai-vercel-ai.md",
  ],
  documentation: {
    packageDoc: "docs/packages/ai-vercel-ai.md",
    dashboardWorkflow: "examples/dashboard/src/components/AiLaunchAssistant.tsx",
    launchProof: "tools/launch/runtime-template/pages/index.html",
  },
  sourceSurface: [
    "streamText",
    "pruneMessages",
    "ModelMessage",
    "generateImage",
    "ImageModel",
    "GenerateImageResult",
    "GeneratedFile",
    "experimental_generateSpeech",
    "SpeechModel",
    "Experimental_SpeechResult",
    "GeneratedAudioFile",
    "experimental_transcribe",
    "TranscriptionModel",
    "Experimental_TranscriptionResult",
    "createDownload",
    "experimental_generateVideo",
    "GenerateVideoPrompt",
    "GenerateVideoResult",
    "VideoModel",
    "generateObject",
    "streamObject",
        "GenerateObjectResult",
        "StreamObjectResult",
        "uploadFile",
        "UploadFileResult",
        "createTextStreamResponse",
        "pipeTextStreamToResponse",
        "createUIMessageStream",
        "createUIMessageStreamResponse",
        "pipeUIMessageStreamToResponse",
        "readUIMessageStream",
        "UI_MESSAGE_STREAM_HEADERS",
        "generateText",
        "Output.object",
    "embed",
    "embedMany",
    "cosineSimilarity",
    "EmbeddingModel",
    "RerankingModel",
    "rerank",
    "LanguageModelMiddleware",
    "wrapLanguageModel",
    "defaultSettingsMiddleware",
    "Telemetry",
    "TelemetryOptions",
    "registerTelemetry",
    "ToolLoopAgent",
    "InferAgentUIMessage",
    "createAgentUIStreamResponse",
    "ToolApprovalConfiguration",
    "ToolApprovalStatus",
    "ToolApprovalRequest",
    "ToolApprovalResponse",
    "gateway",
    "createGateway",
    "customProvider",
    "createProviderRegistry",
    "convertToModelMessages",
    "tool",
    "zod-backed inputSchema",
    "DefaultChatTransport",
    "UIMessage",
  ],
  dependencies: [
    {
      name: "ai",
      version: "^7.0.0-canary.146",
      required: true,
    },
    {
      name: "zod",
      version: "^3.25.76 || ^4.1.8",
      required: true,
    },
  ],
  exportedFiles: [
    "lib/ai/model.ts",
    "lib/ai/chat-route.ts",
    "lib/ai/client-chat.tsx",
    "lib/ai/provider-freedom.ts",
    "lib/ai/provider-boundary.ts",
    "lib/ai/dashboard-readiness.ts",
    "lib/ai/text-stream.ts",
    "lib/ai/ui-message-stream.ts",
    "components/ai/ai-launch-assistant.tsx",
    "lib/ai/metadata.ts",
  ],
  materializedFiles: [
    "lib/ai/model.ts",
    "lib/ai/model-policy.ts",
    "lib/ai/tools.ts",
    "lib/ai/chat-route.ts",
    "lib/ai/message-pruning.ts",
    "lib/ai/tool-approval.ts",
    "lib/ai/image-generation.ts",
    "lib/ai/speech-generation.ts",
    "lib/ai/transcription.ts",
    "lib/ai/video-generation.ts",
    "lib/ai/object-generation.ts",
    "lib/ai/structured-output.ts",
    "lib/ai/telemetry.ts",
    "lib/ai/embeddings.ts",
    "lib/ai/reranking.ts",
        "lib/ai/agent.ts",
        "lib/ai/provider-freedom.ts",
        "lib/ai/provider-boundary.ts",
        "lib/ai/file-upload.ts",
        "lib/ai/text-stream.ts",
        "lib/ai/ui-message-stream.ts",
        "lib/ai/client-chat.tsx",
        "lib/ai/dashboard-readiness.ts",
        "components/ai/ai-launch-assistant.tsx",
    "lib/ai/metadata.ts",
    "app/api/ai/chat/route.ts",
    "app/api/ai/agent/route.ts",
    "app/api/ai/image/route.ts",
    "app/api/ai/speech/route.ts",
    "app/api/ai/transcribe/route.ts",
    "app/api/ai/video/route.ts",
    "app/api/ai/object/route.ts",
    "app/api/ai/upload-file/route.ts",
    "app/api/ai/text-stream/route.ts",
    "app/api/ai/ui-stream/route.ts",
  ],
  outsideDefaultSurfaceRoutes: [
    "app/api/ai/agent/route.ts",
    "app/api/ai/image/route.ts",
    "app/api/ai/object/route.ts",
    "app/api/ai/speech/route.ts",
    "app/api/ai/text-stream/route.ts",
    "app/api/ai/transcribe/route.ts",
    "app/api/ai/ui-stream/route.ts",
    "app/api/ai/upload-file/route.ts",
    "app/api/ai/video/route.ts",
  ],
  discovery: {
    dxAdd: "dx add ai-sdk --write",
    dashboardWorkflow: "components/ai/ai-launch-assistant.tsx",
    dashboardExample: "examples/dashboard/src/components/AiLaunchAssistant.tsx",
    agentFactory: "createDxLaunchAgent({ model, readStatus })",
    agentRoute: "createDxAgentRoute(agent)",
    messagePruning: "createDxLaunchMessagePruner({ reasoning, toolCalls })",
    toolApproval: "createDxLaunchToolApproval({ autoApproveTools, denyTools })",
    imageGeneration: "generateDxLaunchImageAsset({ imageModel, prompt })",
    speechGeneration: "generateDxLaunchSpeechAudio({ speechModel, text })",
    transcription: "transcribeDxLaunchAudio({ transcriptionModel, audio })",
    videoGeneration: "generateDxLaunchVideoAsset({ videoModel, prompt })",
    objectGeneration: "generateDxLaunchObject({ model, schema, prompt })",
    uploadFile: "uploadDxLaunchFile({ api, data, filename })",
    textStream: "createDxLaunchTextStream({ chunks })",
    uiMessageStream: "createDxLaunchUIMessageStream({ execute })",
    modelPolicy: "withDxLaunchModelPolicy(model, { maxOutputTokens, temperature })",
    routeFactory: "createDxAIChatRoute({ model, readStatus })",
    structuredOutput: "generateDxLaunchStructuredStatus({ model, readStatus })",
    telemetry: "createDxLaunchTelemetryOptions({ functionId, sink })",
    embeddingSearch: "rankDxLaunchNotesBySimilarity({ embeddingModel, query, notes })",
    reranking: "rerankDxLaunchEvidence({ rerankingModel, query, documents })",
    providerFreedom: "createDxProviderRegistry({ languageModels, embeddingModels })",
    clientComponent: "DxAIClientChat",
  },
} as const;

export type DxVercelAIForgePackage = typeof dxVercelAIForgePackage;
"#,
        ),
        (
            "js/app/api/ai/chat/route.ts",
            r#"import { createOpenAI } from "@ai-sdk/openai";

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
"#,
        ),
        (
            "js/app/api/ai/agent/route.ts",
            r#"import { createOpenAI } from "@ai-sdk/openai";

import { createDxAgentRoute, createDxLaunchAgent } from "@/lib/ai/agent";
import {
  createDxAiExtendedRouteDisabledResponse,
  createDxAiMissingProviderResponse,
  isDxAiExtendedRouteEnabled,
} from "@/lib/ai/provider-boundary";
import { createDxLaunchToolApproval } from "@/lib/ai/tool-approval";

const openai = createOpenAI({
  apiKey: process.env.AI_PROVIDER_API_KEY,
});

const postWithConfiguredProvider = createDxAgentRoute(
  createDxLaunchAgent({
    model: openai("gpt-5-mini"),
    toolApproval: createDxLaunchToolApproval({
      autoApproveTools: ["launchStatus"],
    }),
    readStatus: () => ({
      current: "DX launch agent route is wired.",
      next: "Connect this agent to live receipts and app-approved tools before launch.",
      score: 80,
    }),
  }),
);

export async function POST(request: Request): Promise<Response> {
  if (!isDxAiExtendedRouteEnabled()) {
    return createDxAiExtendedRouteDisabledResponse({
      route: "/api/ai/agent",
      provider: "openai-compatible",
      capability: "agent-loop",
      requiredEnv: "AI_PROVIDER_API_KEY",
      credentialsConfigured: Boolean(process.env.AI_PROVIDER_API_KEY),
      appOwnedBoundary:
        "This extended AI route is outside the default launch AI proof surface. Set DX_ENABLE_EXTENDED_AI_ROUTES=true after app-owned agent policy, rate limits, telemetry, and browser proof are ready.",
    });
  }

  if (!process.env.AI_PROVIDER_API_KEY) {
    return createDxAiMissingProviderResponse({
      provider: "openai-compatible",
      capability: "agent-loop",
      requiredEnv: "AI_PROVIDER_API_KEY",
      appOwnedBoundary:
        "Set AI_PROVIDER_API_KEY and review app-approved tools before running the agent loop.",
    });
  }

  return postWithConfiguredProvider(request);
}
"#,
        ),
        (
            "js/app/api/ai/image/route.ts",
            r#"import { createOpenAI } from "@ai-sdk/openai";

import { generateDxLaunchImageAsset } from "@/lib/ai/image-generation";
import {
  createDxAiExtendedRouteDisabledResponse,
  createDxAiMissingProviderResponse,
  isDxAiExtendedRouteEnabled,
} from "@/lib/ai/provider-boundary";

const openai = createOpenAI({
  apiKey: process.env.AI_PROVIDER_API_KEY,
});

export async function POST(request: Request): Promise<Response> {
  if (!isDxAiExtendedRouteEnabled()) {
    return createDxAiExtendedRouteDisabledResponse({
      route: "/api/ai/image",
      provider: "openai-compatible",
      capability: "image-generation",
      requiredEnv: "AI_PROVIDER_API_KEY",
      credentialsConfigured: Boolean(process.env.AI_PROVIDER_API_KEY),
      appOwnedBoundary:
        "This extended AI route is outside the default launch AI proof surface. Set DX_ENABLE_EXTENDED_AI_ROUTES=true after app-owned image policy, rate limits, telemetry, storage, and browser proof are ready.",
    });
  }

  if (!process.env.AI_PROVIDER_API_KEY) {
    return createDxAiMissingProviderResponse({
      provider: "openai-compatible",
      capability: "image-generation",
      requiredEnv: "AI_PROVIDER_API_KEY",
      appOwnedBoundary:
        "Set AI_PROVIDER_API_KEY before generating www image assets.",
    });
  }

  const body = (await request.json()) as { prompt?: string };

  const asset = await generateDxLaunchImageAsset({
    imageModel: openai.image("gpt-image-1.5"),
    prompt:
      body.prompt ??
      "A polished DX launch dashboard hero asset with native editor, web preview, and source-owned packages.",
    size: "1024x1024",
    providerOptions: {
      openai: {
        quality: "high",
      },
    },
  });

  return Response.json(asset);
}
"#,
        ),
        (
            "js/app/api/ai/speech/route.ts",
            r#"import { createOpenAI } from "@ai-sdk/openai";

import {
  createDxAiExtendedRouteDisabledResponse,
  createDxAiMissingProviderResponse,
  isDxAiExtendedRouteEnabled,
} from "@/lib/ai/provider-boundary";
import { generateDxLaunchSpeechAudio } from "@/lib/ai/speech-generation";

const openai = createOpenAI({
  apiKey: process.env.AI_PROVIDER_API_KEY,
});

export async function POST(request: Request): Promise<Response> {
  if (!isDxAiExtendedRouteEnabled()) {
    return createDxAiExtendedRouteDisabledResponse({
      route: "/api/ai/speech",
      provider: "openai-compatible",
      capability: "speech-generation",
      requiredEnv: "AI_PROVIDER_API_KEY",
      credentialsConfigured: Boolean(process.env.AI_PROVIDER_API_KEY),
      appOwnedBoundary:
        "This extended AI route is outside the default launch AI proof surface. Set DX_ENABLE_EXTENDED_AI_ROUTES=true after app-owned audio consent, storage, rate limits, telemetry, and browser proof are ready.",
    });
  }

  if (!process.env.AI_PROVIDER_API_KEY) {
    return createDxAiMissingProviderResponse({
      provider: "openai-compatible",
      capability: "speech-generation",
      requiredEnv: "AI_PROVIDER_API_KEY",
      appOwnedBoundary:
        "Set AI_PROVIDER_API_KEY before generating www speech audio.",
    });
  }

  const body = (await request.json()) as { text?: string; voice?: string };

  const audio = await generateDxLaunchSpeechAudio({
    speechModel: openai.speech("tts-1"),
    text:
      body.text ??
      "DX launch voice brief is wired. Connect production copy, consent, and storage before release.",
    voice: body.voice ?? "alloy",
    outputFormat: "mp3",
  });

  return Response.json(audio);
}
"#,
        ),
        (
            "js/app/api/ai/transcribe/route.ts",
            r#"import { createOpenAI } from "@ai-sdk/openai";

import {
  createDxAiExtendedRouteDisabledResponse,
  createDxAiMissingProviderResponse,
  isDxAiExtendedRouteEnabled,
} from "@/lib/ai/provider-boundary";
import { transcribeDxLaunchAudio } from "@/lib/ai/transcription";

const openai = createOpenAI({
  apiKey: process.env.AI_PROVIDER_API_KEY,
});

export async function POST(request: Request): Promise<Response> {
  if (!isDxAiExtendedRouteEnabled()) {
    return createDxAiExtendedRouteDisabledResponse({
      route: "/api/ai/transcribe",
      provider: "openai-compatible",
      capability: "audio-transcription",
      requiredEnv: "AI_PROVIDER_API_KEY",
      credentialsConfigured: Boolean(process.env.AI_PROVIDER_API_KEY),
      appOwnedBoundary:
        "This extended AI route is outside the default launch AI proof surface. Set DX_ENABLE_EXTENDED_AI_ROUTES=true after app-owned audio consent, storage, rate limits, telemetry, and browser proof are ready.",
    });
  }

  if (!process.env.AI_PROVIDER_API_KEY) {
    return createDxAiMissingProviderResponse({
      provider: "openai-compatible",
      capability: "audio-transcription",
      requiredEnv: "AI_PROVIDER_API_KEY",
      appOwnedBoundary:
        "Set AI_PROVIDER_API_KEY before transcribing www audio.",
    });
  }

  const formData = await request.formData();
  const audio = formData.get("audio");
  const audioUrl = formData.get("audioUrl");

  const input =
    audio instanceof File
      ? new Uint8Array(await audio.arrayBuffer())
      : typeof audioUrl === "string" && audioUrl.length > 0
        ? new URL(audioUrl)
        : undefined;

  if (!input) {
    return Response.json(
      { error: "Upload an audio file or provide audioUrl." },
      { status: 400 },
    );
  }

  const transcript = await transcribeDxLaunchAudio({
    transcriptionModel: openai.transcription("whisper-1"),
    audio: input,
  });

  return Response.json(transcript);
}
"#,
        ),
        (
            "js/app/api/ai/video/route.ts",
            r#"import { gateway } from "ai";

import {
  createDxAiExtendedRouteDisabledResponse,
  createDxAiMissingProviderResponse,
  isDxAiExtendedRouteEnabled,
} from "@/lib/ai/provider-boundary";
import { generateDxLaunchVideoAsset } from "@/lib/ai/video-generation";

export async function POST(request: Request): Promise<Response> {
  if (!isDxAiExtendedRouteEnabled()) {
    return createDxAiExtendedRouteDisabledResponse({
      route: "/api/ai/video",
      provider: "gateway",
      capability: "video-generation",
      requiredEnv: "AI_GATEWAY_API_KEY",
      credentialsConfigured: Boolean(process.env.AI_GATEWAY_API_KEY),
      appOwnedBoundary:
        "This extended AI route is outside the default launch AI proof surface. Set DX_ENABLE_EXTENDED_AI_ROUTES=true after app-owned video policy, cost limits, storage, telemetry, and browser proof are ready.",
    });
  }

  if (!process.env.AI_GATEWAY_API_KEY) {
    return createDxAiMissingProviderResponse({
      provider: "gateway",
      capability: "video-generation",
      requiredEnv: "AI_GATEWAY_API_KEY",
      appOwnedBoundary:
        "Set AI_GATEWAY_API_KEY before generating www video assets.",
    });
  }

  const body = (await request.json()) as { prompt?: string };

  const video = await generateDxLaunchVideoAsset({
    videoModel: gateway.videoModel("fal:luma-dream-machine/ray-2"),
    prompt:
      body.prompt ??
      "A short DX launch walkthrough showing a native editor, web preview, agents, and source-owned Forge packages.",
    aspectRatio: "16:9",
    duration: 5,
    maxDownloadBytes: 50 * 1024 * 1024,
  });

  return Response.json(video);
}
"#,
        ),
        (
            "js/app/api/ai/object/route.ts",
            r#"import { createOpenAI } from "@ai-sdk/openai";
import { z } from "zod";

import { generateDxLaunchObject } from "@/lib/ai/object-generation";
import {
  createDxAiExtendedRouteDisabledResponse,
  createDxAiMissingProviderResponse,
  isDxAiExtendedRouteEnabled,
} from "@/lib/ai/provider-boundary";

const openai = createOpenAI({
  apiKey: process.env.AI_PROVIDER_API_KEY,
});

const launchPlanSchema = z.object({
  current: z.string(),
  next: z.string(),
  risks: z.array(z.string()),
});

export async function POST(request: Request): Promise<Response> {
  if (!isDxAiExtendedRouteEnabled()) {
    return createDxAiExtendedRouteDisabledResponse({
      route: "/api/ai/object",
      provider: "openai-compatible",
      capability: "object-generation",
      requiredEnv: "AI_PROVIDER_API_KEY",
      credentialsConfigured: Boolean(process.env.AI_PROVIDER_API_KEY),
      appOwnedBoundary:
        "This extended AI route is outside the default launch AI proof surface. Set DX_ENABLE_EXTENDED_AI_ROUTES=true after app-owned schema policy, rate limits, telemetry, and browser proof are ready.",
    });
  }

  if (!process.env.AI_PROVIDER_API_KEY) {
    return createDxAiMissingProviderResponse({
      provider: "openai-compatible",
      capability: "object-generation",
      requiredEnv: "AI_PROVIDER_API_KEY",
      appOwnedBoundary:
        "Set AI_PROVIDER_API_KEY before generating structured www objects.",
    });
  }

  const body = (await request.json()) as { prompt?: string };

  const result = await generateDxLaunchObject({
    model: openai("gpt-5-mini"),
    schema: launchPlanSchema,
    schemaName: "DxLaunchPlan",
    schemaDescription: "A compact DX launch plan object.",
    prompt:
      body.prompt ??
      "Return the current DX launch plan, next action, and risks from the latest package evidence.",
  });

  return result.toJsonResponse();
}
"#,
        ),
        (
            "js/app/api/ai/ui-stream/route.ts",
            r#"import {
  createDxLaunchUIMessageStream,
  createDxLaunchUIMessageStreamResponse,
} from "@/lib/ai/ui-message-stream";
import {
  createDxAiExtendedRouteDisabledResponse,
  isDxAiExtendedRouteEnabled,
} from "@/lib/ai/provider-boundary";

export async function POST(request: Request): Promise<Response> {
  if (!isDxAiExtendedRouteEnabled()) {
    return createDxAiExtendedRouteDisabledResponse({
      route: "/api/ai/ui-stream",
      provider: "openai-compatible",
      capability: "ui-message-stream",
      requiredEnv: "DX_ENABLE_EXTENDED_AI_ROUTES",
      credentialsConfigured: false,
      appOwnedBoundary:
        "This extended AI route is outside the default launch AI proof surface. Set DX_ENABLE_EXTENDED_AI_ROUTES=true after app-owned stream policy, rate limits, telemetry, and browser proof are ready.",
    });
  }

  const body = (await request.json()) as { text?: string };
  const text =
    body.text ??
    "DX UI message stream bridge is wired. Connect it to real route work before launch.";

  const stream = createDxLaunchUIMessageStream({
    execute: (writer) => {
      writer.write({ type: "text-start", id: "dx-launch-ui-stream" });
      writer.write({
        type: "text-delta",
        id: "dx-launch-ui-stream",
        delta: text,
      });
      writer.write({ type: "text-end", id: "dx-launch-ui-stream" });
    },
  });

  return createDxLaunchUIMessageStreamResponse({ stream });
}
"#,
        ),
        (
            "js/app/api/ai/text-stream/route.ts",
            r#"import {
  createDxLaunchTextStream,
  createDxLaunchTextStreamResponse,
} from "@/lib/ai/text-stream";
import {
  createDxAiExtendedRouteDisabledResponse,
  isDxAiExtendedRouteEnabled,
} from "@/lib/ai/provider-boundary";

export async function GET(): Promise<Response> {
  if (!isDxAiExtendedRouteEnabled()) {
    return createDxAiExtendedRouteDisabledResponse({
      route: "/api/ai/text-stream",
      provider: "openai-compatible",
      capability: "plain-text-stream",
      requiredEnv: "DX_ENABLE_EXTENDED_AI_ROUTES",
      credentialsConfigured: false,
      appOwnedBoundary:
        "This extended AI route is outside the default launch AI proof surface. Set DX_ENABLE_EXTENDED_AI_ROUTES=true after app-owned stream policy, rate limits, telemetry, and browser proof are ready.",
    });
  }

  const textStream = createDxLaunchTextStream({
    chunks: [
      "DX text stream bridge is wired.\n",
      "Connect it to live launch receipts before production.\n",
    ],
  });

  return createDxLaunchTextStreamResponse({ textStream });
}
"#,
        ),
        (
            "js/app/api/ai/upload-file/route.ts",
            r#"import { createOpenAI } from "@ai-sdk/openai";

import { uploadDxLaunchFile } from "@/lib/ai/file-upload";
import {
  createDxAiExtendedRouteDisabledResponse,
  createDxAiMissingProviderResponse,
  isDxAiExtendedRouteEnabled,
} from "@/lib/ai/provider-boundary";

const openai = createOpenAI({
  apiKey: process.env.AI_PROVIDER_API_KEY,
});

export async function POST(request: Request): Promise<Response> {
  if (!isDxAiExtendedRouteEnabled()) {
    return createDxAiExtendedRouteDisabledResponse({
      route: "/api/ai/upload-file",
      provider: "openai-compatible",
      capability: "file-upload",
      requiredEnv: "AI_PROVIDER_API_KEY",
      credentialsConfigured: Boolean(process.env.AI_PROVIDER_API_KEY),
      appOwnedBoundary:
        "This extended AI route is outside the default launch AI proof surface. Set DX_ENABLE_EXTENDED_AI_ROUTES=true after app-owned file policy, size limits, storage, telemetry, and browser proof are ready.",
    });
  }

  if (!process.env.AI_PROVIDER_API_KEY) {
    return createDxAiMissingProviderResponse({
      provider: "openai-compatible",
      capability: "file-upload",
      requiredEnv: "AI_PROVIDER_API_KEY",
      appOwnedBoundary:
        "Set AI_PROVIDER_API_KEY before uploading www files to a provider.",
    });
  }

  const formData = await request.formData();
  const file = formData.get("file");

  if (!(file instanceof File)) {
    return Response.json({ error: "Upload a file field." }, { status: 400 });
  }

  const result = await uploadDxLaunchFile({
    api: openai,
    data: new Uint8Array(await file.arrayBuffer()),
    filename: file.name,
    mediaType: file.type || undefined,
    maxBytes: 10 * 1024 * 1024,
  });

  return Response.json({
    providerReference: result.providerReference,
    filename: result.filename,
    mediaType: result.mediaType,
    warnings: result.warnings,
  });
}
"#,
        ),
        (
            "js/lib/ai/README.md",
            r#"# DX Forge AI SDK Slice

This package materializes a source-owned AI SDK launch slice around the upstream `ai` public API, with Vercel AI SDK provenance recorded in metadata.

## Owned Surface

- `model.ts` keeps the application-owned model provider explicit.
- `model-policy.ts` applies launch-safe default settings with `wrapLanguageModel()` and `defaultSettingsMiddleware()`.
- `tools.ts` shows a typed tool using `tool()` and Zod input schema.
- `chat-route.ts` builds a streaming route with `streamText()`, `convertToModelMessages()`, and `toUIMessageStreamResponse()`.
- `message-pruning.ts` wraps `pruneMessages()` so long chat, reasoning, tool, and approval history can be trimmed before model calls.
- `tool-approval.ts` creates app-owned tool approval policies and typed `ToolApprovalResponse` values.
- `image-generation.ts` creates launch image assets with `generateImage()`, `ImageModel`, and `GenerateImageResult`.
- `speech-generation.ts` creates launch voice briefs with `experimental_generateSpeech()`, `SpeechModel`, `Experimental_SpeechResult`, and `GeneratedAudioFile`.
- `transcription.ts` transcribes launch audio with `experimental_transcribe()`, `TranscriptionModel`, `Experimental_TranscriptionResult`, and `createDownload()`.
- `video-generation.ts` creates launch video assets with `experimental_generateVideo()`, `GenerateVideoPrompt`, `GenerateVideoResult`, `VideoModel`, and `createDownload()`.
- `object-generation.ts` keeps a small compatibility wrapper for deprecated-but-exported `generateObject()` and `streamObject()` APIs.
- `structured-output.ts` builds typed launch status objects with `generateText()` and `Output.object()`.
- `telemetry.ts` exposes privacy-first `TelemetryOptions`, `Telemetry`, and `registerTelemetry()` helpers for launch observability.
- `embeddings.ts` ranks launch notes with `embed()`, `embedMany()`, `EmbeddingModel`, and `cosineSimilarity()`.
- `reranking.ts` reranks launch evidence documents with `rerank()` and `RerankingModel`.
- `agent.ts` builds a source-owned `ToolLoopAgent` and `createAgentUIStreamResponse()` route helper with typed `InferAgentUIMessage` messages.
- `provider-freedom.ts` creates gateway-backed custom providers and a provider registry with `gateway`, `createGateway`, `customProvider`, and `createProviderRegistry`.
- `file-upload.ts` uploads provider-owned file references with `uploadFile()` and `UploadFileResult`.
- `text-stream.ts` creates plain text stream responses with `createTextStreamResponse()` and `pipeTextStreamToResponse()`.
- `ui-message-stream.ts` creates and reads UI message streams with `createUIMessageStream()`, `createUIMessageStreamResponse()`, `pipeUIMessageStreamToResponse()`, `readUIMessageStream()`, and `UI_MESSAGE_STREAM_HEADERS`.
- `client-chat.tsx` gives templates a small client chat surface using `DefaultChatTransport`.
- `dashboard-readiness.ts` gives dashboards a typed provider-readiness and missing-config receipt helper.
- `components/ai/ai-launch-assistant.tsx` is the public dashboard workflow component for provider choice, prompt preview, and receipt state.
- `metadata.ts` gives DX CLI, Zed, and launch templates stable discovery metadata.
- `app/api/ai/chat/route.ts` is a tiny www-template mount point.
- `app/api/ai/agent/route.ts` is a tiny www-template agent route mount point.
- `app/api/ai/image/route.ts` is a tiny www-template image generation route mount point.
- `app/api/ai/speech/route.ts` is a tiny www-template speech generation route mount point.
- `app/api/ai/transcribe/route.ts` is a tiny www-template transcription route mount point.
- `app/api/ai/video/route.ts` is a tiny www-template video generation route mount point.
- `app/api/ai/object/route.ts` is a tiny www-template object generation compatibility route mount point.
- `app/api/ai/upload-file/route.ts` is a tiny www-template provider file upload route mount point.
- `app/api/ai/text-stream/route.ts` is a tiny www-template plain text stream route mount point.
- `app/api/ai/ui-stream/route.ts` is a tiny www-template UI message stream route mount point.
- `examples/dashboard/src/components/AiLaunchAssistant.tsx` consumes the same workflow shape in the starter dashboard.

## Boundary

Forge owns the route and helper shape. The app owns the provider package, API keys, language model choice, image model choice, speech model/voice choice, transcription model choice, video model choice, schema design, object generation compatibility policy, provider file API support, upload limits, retention, scanning, storage policy, text stream content/pacing/Node response policy, UI message stream persistence/resume/error policy, audio/video upload and download limits, consent, PII handling, generated asset storage/CDN, prompt and speech copy moderation, voice consent, video rights review, asset licensing, embedding model choice, reranking model choice, provider registry policy, model safety review, vector storage, telemetry sink/exporter, agent instructions, message pruning policy, tool approval policy, runtime context, dashboard escalation rules, rate limits, persistence, moderation, structured output policy, and production observability.

## Default Launch Proof

The default AI proof surface is the provider-gated chat route plus dashboard readiness metadata. It does not claim live model execution, provider billing coverage, or browser streaming proof.

## Extended Route Boundaries

The agent, image, object, speech, text-stream, transcription, UI-stream, upload-file, and video routes are outside the proven default AI surface. They stay disabled unless `DX_ENABLE_EXTENDED_AI_ROUTES=true` and app-owned provider credentials are configured.
"#,
        ),
    ]
}
