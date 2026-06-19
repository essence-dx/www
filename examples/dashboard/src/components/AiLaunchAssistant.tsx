import { useState } from 'dx';
import {
    aiProviderReadiness,
    createAiPreviewReceipt,
    type AiProviderId,
    type AiPreviewReceipt,
} from '../lib/aiLaunchAssistant';

export function AiLaunchAssistant() {
    const [provider, setProvider] = useState<AiProviderId>('openai-compatible');
    const [prompt, setPrompt] = useState('Summarize dashboard risks before launch.');
    const [receipt, setReceipt] = useState<AiPreviewReceipt | null>(null);

    const selectedProvider = aiProviderReadiness.find(item => item.id === provider) || aiProviderReadiness[0];

    const previewResponse = () => {
        setReceipt(createAiPreviewReceipt({ provider, prompt }));
    };

    return (
        <section
            class="ai-assistant-panel"
            data-dx-component="dashboard-ai-launch-assistant"
            data-dx-package="ai/vercel-ai"
            data-dx-ai-config-state="missing-config"
            data-dx-ai-dashboard-workflow="launch-risk-review"
            data-dx-ai-provider={provider}
            data-dx-node-modules="forbidden"
        >
            <header class="panel-header">
                <dx-icon name="pack:ai" aria-label="AI" />
                <div>
                    <h2>AI SDK launch assistant</h2>
                    <p>Review dashboard risk through the official AI SDK boundary.</p>
                </div>
            </header>

            <div class="provider-options" data-dx-ai-interaction="provider-picker">
                {aiProviderReadiness.map(item => (
                    <button
                        key={item.id}
                        type="button"
                        class={item.id === provider ? 'active' : ''}
                        data-dx-ai-provider-choice={item.id}
                        data-dx-ai-provider-selected={item.id === provider ? 'true' : 'false'}
                        onClick={() => {
                            setProvider(item.id);
                            setReceipt(null);
                        }}
                    >
                        <dx-icon name="pack:settings" aria-hidden="true" />
                        {item.label}
                    </button>
                ))}
            </div>

            <dl class="readiness-list" data-dx-ai-provider-readiness="app-owned">
                <div>
                    <dt>Required env</dt>
                    <dd data-dx-ai-required-env={selectedProvider.requiredEnv.join(',')}>
                        {selectedProvider.requiredEnv.join(', ')}
                    </dd>
                </div>
                <div>
                    <dt>Public APIs</dt>
                    <dd data-dx-ai-public-api={selectedProvider.publicApi.join(',')}>
                        {selectedProvider.publicApi.join(', ')}
                    </dd>
                </div>
            </dl>

            <label class="prompt-field" data-dx-ai-interaction="prompt-field">
                Launch prompt
                <textarea
                    value={prompt}
                    data-dx-ai-prompt-input="dashboard-launch-risk"
                    onChange={(event) => setPrompt((event.target as HTMLTextAreaElement).value)}
                />
            </label>

            <button
                type="button"
                class="primary-action"
                data-dx-ai-action="safe-local-preview"
                onClick={previewResponse}
            >
                <dx-icon name="pack:play" aria-hidden="true" />
                Preview response boundary
            </button>

            <p
                class="assistant-receipt"
                data-dx-ai-local-response={receipt ? 'missing-config' : 'idle'}
                data-dx-ai-receipt-state={receipt ? receipt.status : 'idle'}
            >
                {receipt
                    ? `${receipt.provider} needs app-owned credentials. Prompt length: ${receipt.promptLength}. ${receipt.nextAction}`
                    : 'No AI request has been previewed yet.'}
            </p>
        </section>
    );
}
