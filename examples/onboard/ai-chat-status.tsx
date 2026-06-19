"use client";

import { useMemo, useState } from "react";
import type { UIMessage } from "ai";

import { DxAIClientChat } from "@/lib/ai/client-chat";
import type { createDxLaunchAgent } from "@/lib/ai/agent";
import type { rankDxLaunchNotesBySimilarity } from "@/lib/ai/embeddings";
import type { uploadDxLaunchFile } from "@/lib/ai/file-upload";
import type { generateDxLaunchImageAsset } from "@/lib/ai/image-generation";
import type { createDxLaunchMessagePruner } from "@/lib/ai/message-pruning";
import type { withDxLaunchModelPolicy } from "@/lib/ai/model-policy";
import type { generateDxLaunchObject } from "@/lib/ai/object-generation";
import type { createDxProviderRegistry } from "@/lib/ai/provider-freedom";
import type { rerankDxLaunchEvidence } from "@/lib/ai/reranking";
import type { generateDxLaunchSpeechAudio } from "@/lib/ai/speech-generation";
import type { generateDxLaunchStructuredStatus } from "@/lib/ai/structured-output";
import type { createDxLaunchTelemetryOptions } from "@/lib/ai/telemetry";
import type { createDxLaunchTextStream } from "@/lib/ai/text-stream";
import type { createDxLaunchToolApproval } from "@/lib/ai/tool-approval";
import type { transcribeDxLaunchAudio } from "@/lib/ai/transcription";
import type { createDxLaunchUIMessageStream } from "@/lib/ai/ui-message-stream";
import type { generateDxLaunchVideoAsset } from "@/lib/ai/video-generation";

type LaunchAgentFactory = typeof createDxLaunchAgent;
type LaunchEmbeddingSearch = typeof rankDxLaunchNotesBySimilarity;
type LaunchFileUpload = typeof uploadDxLaunchFile;
type LaunchImageGeneration = typeof generateDxLaunchImageAsset;
type LaunchMessagePruner = typeof createDxLaunchMessagePruner;
type LaunchModelPolicy = typeof withDxLaunchModelPolicy;
type LaunchObjectGeneration = typeof generateDxLaunchObject;
type LaunchProviderRegistry = typeof createDxProviderRegistry;
type LaunchReranking = typeof rerankDxLaunchEvidence;
type LaunchSpeechGeneration = typeof generateDxLaunchSpeechAudio;
type LaunchStructuredStatusGenerator = typeof generateDxLaunchStructuredStatus;
type LaunchTelemetryOptions = typeof createDxLaunchTelemetryOptions;
type LaunchTextStream = typeof createDxLaunchTextStream;
type LaunchToolApproval = typeof createDxLaunchToolApproval;
type LaunchTranscription = typeof transcribeDxLaunchAudio;
type LaunchUIMessageStream = typeof createDxLaunchUIMessageStream;
type LaunchVideoGeneration = typeof generateDxLaunchVideoAsset;

export type LaunchAiChatStatusProps = {
  agentFactory?: LaunchAgentFactory;
  embeddingSearch?: LaunchEmbeddingSearch;
  fileUpload?: LaunchFileUpload;
  imageGeneration?: LaunchImageGeneration;
  messagePruner?: LaunchMessagePruner;
  modelPolicy?: LaunchModelPolicy;
  objectGeneration?: LaunchObjectGeneration;
  providerRegistry?: LaunchProviderRegistry;
  reranking?: LaunchReranking;
  speechGeneration?: LaunchSpeechGeneration;
  structuredStatus?: LaunchStructuredStatusGenerator;
  telemetryOptions?: LaunchTelemetryOptions;
  textStream?: LaunchTextStream;
  toolApproval?: LaunchToolApproval;
  transcription?: LaunchTranscription;
  uiMessageStream?: LaunchUIMessageStream;
  videoGeneration?: LaunchVideoGeneration;
};

const initialMessages = [
  {
    id: "dx-launch-ai-intro",
    role: "assistant",
    parts: [
      {
        type: "text",
        text: "Ask about launch blockers, package readiness, or final release evidence.",
      },
    ],
  },
] satisfies UIMessage[];

const providerOptions = [
  {
    id: "openai-compatible",
    label: "OpenAI-compatible",
    env: "AI_PROVIDER_API_KEY",
    publicApis: ["streamText", "convertToModelMessages", "tool"],
  },
  {
    id: "gateway",
    label: "AI Gateway",
    env: "AI_GATEWAY_API_KEY",
    publicApis: ["gateway", "createGateway", "createProviderRegistry"],
  },
] as const;

type ProviderId = (typeof providerOptions)[number]["id"];

type PreviewState =
  | {
      status: "idle";
      message: string;
    }
  | {
      status: "pending";
      message: string;
    }
  | {
      status: "missing-config" | "route-ready" | "invalid-prompt" | "error";
      message: string;
      requestId: string;
    };

export function LaunchAiChatStatus({
  agentFactory,
  embeddingSearch,
  fileUpload,
  imageGeneration,
  messagePruner,
  modelPolicy,
  objectGeneration,
  providerRegistry,
  reranking,
  speechGeneration,
  structuredStatus,
  telemetryOptions,
  textStream,
  toolApproval,
  transcription,
  uiMessageStream,
  videoGeneration,
}: LaunchAiChatStatusProps) {
  const agentLabel = agentFactory ? "agent helper wired" : "agent policy app-owned";
  const embeddingLabel = embeddingSearch
    ? "embedding search helper wired"
    : "embedding search app-owned";
  const imageGenerationLabel = imageGeneration
    ? "image generation helper wired"
    : "image generation app-owned";
  const fileUploadLabel = fileUpload
    ? "file upload helper wired"
    : "file upload app-owned";
  const messagePruningLabel = messagePruner
    ? "message pruning helper wired"
    : "message pruning app-owned";
  const providerLabel = providerRegistry
    ? "provider registry helper wired"
    : "provider registry app-owned";
  const structuredLabel = structuredStatus
    ? "structured output helper wired"
    : "structured output app-owned";
  const telemetryLabel = telemetryOptions
    ? "telemetry helper wired"
    : "telemetry sink app-owned";
  const textStreamLabel = textStream
    ? "text stream helper wired"
    : "text stream app-owned";
  const toolApprovalLabel = toolApproval
    ? "tool approval helper wired"
    : "tool approval app-owned";
  const modelPolicyLabel = modelPolicy
    ? "model policy helper wired"
    : "model policy app-owned";
  const objectGenerationLabel = objectGeneration
    ? "object generation helper wired"
    : "object generation compatibility app-owned";
  const rerankingLabel = reranking
    ? "reranking helper wired"
    : "reranking model app-owned";
  const speechGenerationLabel = speechGeneration
    ? "speech generation helper wired"
    : "speech generation app-owned";
  const transcriptionLabel = transcription
    ? "transcription helper wired"
    : "transcription app-owned";
  const uiMessageStreamLabel = uiMessageStream
    ? "UI message stream helper wired"
    : "UI message stream app-owned";
  const videoGenerationLabel = videoGeneration
    ? "video generation helper wired"
    : "video generation app-owned";
  const [provider, setProvider] = useState<ProviderId>("openai-compatible");
  const [prompt, setPrompt] = useState("Summarize launch blockers and missing credentials.");
  const [preview, setPreview] = useState<PreviewState>({
    status: "idle",
    message: "AI route idle. Provider credentials are app-owned.",
  });
  const activeProvider = useMemo(
    () => providerOptions.find((option) => option.id === provider) ?? providerOptions[0],
    [provider],
  );
  const trimmedPrompt = prompt.trim();
  const promptState = trimmedPrompt.length > 0 ? "ready" : "empty";

  async function previewRouteContract() {
    const requestId = `dx-ai-${Date.now().toString(36)}`;
    if (!trimmedPrompt) {
      setPreview({
        status: "invalid-prompt",
        requestId,
        message: "Enter a launch prompt before previewing the AI route contract.",
      });
      return;
    }

    setPreview({
      status: "pending",
      message: "Checking the stream-ready route contract...",
    });

    try {
      const response = await fetch("/api/ai/chat", {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({
          message: trimmedPrompt,
          provider,
          requestId,
        }),
      });
      const payload = await response.json().catch(() => ({}));
      setPreview({
        status: response.ok ? "route-ready" : "missing-config",
        requestId,
        message: response.ok
          ? `${payload.provider ?? provider} route accepted ${requestId}. Streaming remains provider-owned until credentials are configured.`
          : `${activeProvider.env} is missing. Route contract is present and ready for app-owned credentials.`,
      });
    } catch {
      setPreview({
        status: "missing-config",
        requestId,
        message:
          "Local server route is not executing in this source view. The dashboard still records the app-owned credential boundary.",
      });
    }
  }

  return (
    <section
      className="grid gap-3"
      data-dx-package="ai/vercel-ai"
      data-dx-component="launch-ai-dashboard-workflow"
      data-dx-dashboard-workflow="launch-ai-assistant"
      data-dx-style-surface="ai-sdk"
      data-dx-token-scope="ai/vercel-ai"
      data-dx-ai-route-contract="/api/ai/chat"
      data-dx-ai-provider={provider}
      data-dx-ai-config-state="missing-config"
      data-dx-ai-preview-state={preview.status}
      data-dx-node-modules="forbidden"
      data-dx-ai-agent={agentLabel}
      data-dx-ai-embeddings={embeddingLabel}
      data-dx-ai-file-upload={fileUploadLabel}
      data-dx-ai-image-generation={imageGenerationLabel}
      data-dx-ai-message-pruning={messagePruningLabel}
      data-dx-ai-model-policy={modelPolicyLabel}
      data-dx-ai-object-generation={objectGenerationLabel}
      data-dx-ai-provider-freedom={providerLabel}
      data-dx-ai-reranking={rerankingLabel}
      data-dx-ai-speech-generation={speechGenerationLabel}
      data-dx-ai-structured-output={structuredLabel}
      data-dx-ai-telemetry={telemetryLabel}
      data-dx-ai-text-stream={textStreamLabel}
      data-dx-ai-tool-approval={toolApprovalLabel}
      data-dx-ai-transcription={transcriptionLabel}
      data-dx-ai-ui-message-stream={uiMessageStreamLabel}
      data-dx-ai-video-generation={videoGenerationLabel}
    >
      <div className="grid gap-3 rounded-lg border border-border bg-card p-3">
        <div className="flex flex-wrap items-start justify-between gap-3">
          <div className="space-y-1">
            <p className="text-sm font-medium">
              <dx-icon name="pack:ai" aria-hidden="true" /> AI SDK launch assistant
            </p>
            <p className="text-xs text-muted-foreground">
              Prompt review is wired to the stream-ready route contract while
              provider credentials stay app-owned.
            </p>
          </div>
          <span
            className="rounded-md border border-border px-2 py-1 text-xs text-muted-foreground"
            data-dx-ai-required-env={activeProvider.env}
          >
            {activeProvider.env}
          </span>
        </div>

        <div
          className="grid gap-2 sm:grid-cols-2"
          data-dx-ai-interaction="provider-picker"
          data-dx-ai-provider-readiness="app-owned"
        >
          {providerOptions.map((option) => (
            <button
              key={option.id}
              type="button"
              className="rounded-lg border border-border px-3 py-2 text-left text-sm transition-colors hover:bg-muted"
              data-dx-ai-provider-choice={option.id}
              data-dx-ai-provider-selected={provider === option.id}
              data-dx-ai-provider-env={option.env}
              onClick={() => setProvider(option.id)}
            >
              <span className="font-medium">{option.label}</span>
              <span className="mt-1 block text-xs text-muted-foreground">
                {option.publicApis.join(", ")}
              </span>
            </button>
          ))}
        </div>

        <label className="grid gap-1 text-sm" data-dx-ai-interaction="prompt-field">
          Launch prompt
          <textarea
            className="min-h-24 rounded-lg border border-input bg-background px-3 py-2 text-sm"
            data-dx-ai-prompt-input="launch-assistant"
            data-dx-ai-prompt-state={promptState}
            value={prompt}
            onChange={(event) => setPrompt(event.currentTarget.value)}
          />
        </label>

        <div className="flex flex-wrap items-center gap-2">
          <button
            type="button"
            className="inline-flex items-center gap-2 rounded-lg bg-primary px-3 py-2 text-sm font-medium text-primary-foreground"
            data-dx-ai-action="safe-stream-contract-preview"
            data-dx-ai-action-state={
              preview.status === "pending"
                ? "pending"
                : promptState === "empty"
                  ? "needs-prompt"
                  : "ready"
            }
            onClick={previewRouteContract}
            disabled={preview.status === "pending"}
          >
            <dx-icon name="pack:play" aria-hidden="true" />
            {preview.status === "pending" ? "Checking route" : "Preview route"}
          </button>
          <p
            className="text-xs text-muted-foreground"
            data-dx-ai-public-api={activeProvider.publicApis.join(",")}
          >
            Uses {activeProvider.publicApis.join(", ")}
          </p>
        </div>

        <p
          className="rounded-lg border border-border bg-muted px-3 py-2 text-sm text-muted-foreground"
          data-dx-ai-local-response={preview.status}
          data-dx-ai-response-state={preview.status}
          data-dx-ai-request-id={"requestId" in preview ? preview.requestId : "idle"}
        >
          {preview.message}
        </p>
      </div>

      <p className="text-xs text-muted-foreground">
        {agentLabel}; {structuredLabel}; {modelPolicyLabel}; {embeddingLabel};{" "}
        {fileUploadLabel}; {imageGenerationLabel}; {messagePruningLabel};{" "}
        {objectGenerationLabel}; {providerLabel}; {rerankingLabel}; {speechGenerationLabel};{" "}
        {textStreamLabel}; {transcriptionLabel}; {uiMessageStreamLabel};{" "}
        {videoGenerationLabel}; {toolApprovalLabel}; {telemetryLabel}
      </p>
      <DxAIClientChat
        api="/api/ai/chat"
        initialMessages={initialMessages}
        placeholder="Ask DX about this launch"
      />
    </section>
  );
}
