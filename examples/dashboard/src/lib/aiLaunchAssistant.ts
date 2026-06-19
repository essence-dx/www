export type AiProviderId = 'openai-compatible' | 'gateway';

export type AiProviderReadiness = {
    id: AiProviderId;
    label: string;
    requiredEnv: string[];
    publicApi: string[];
};

export type AiPreviewReceipt = {
    provider: AiProviderId;
    status: 'missing-config' | 'ready';
    promptLength: number;
    nextAction: string;
};

export const aiProviderReadiness: AiProviderReadiness[] = [
    {
        id: 'openai-compatible',
        label: 'OpenAI-compatible',
        requiredEnv: ['AI_PROVIDER_API_KEY'],
        publicApi: ['streamText', 'convertToModelMessages', 'tool'],
    },
    {
        id: 'gateway',
        label: 'AI Gateway',
        requiredEnv: ['AI_GATEWAY_API_KEY'],
        publicApi: ['gateway', 'createGateway', 'createProviderRegistry'],
    },
];

export function createAiPreviewReceipt({
    provider,
    prompt,
}: {
    provider: AiProviderId;
    prompt: string;
}): AiPreviewReceipt {
    const readiness = aiProviderReadiness.find((item) => item.id === provider);

    return {
        provider,
        status: 'missing-config',
        promptLength: prompt.trim().length,
        nextAction: `Configure ${readiness?.requiredEnv.join(', ') || 'AI provider env'} before streaming model output.`,
    };
}
