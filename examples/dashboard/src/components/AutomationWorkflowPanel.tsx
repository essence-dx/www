import { useState } from 'dx';
import {
    buildN8nWorkflowReadiness,
    createRedactedN8nReceipt,
    n8nAutomationForgeMetadata,
    n8nDashboardConnectors,
    type N8nRedactedReceipt,
} from '../lib/n8nAutomationBridge';

export function AutomationWorkflowPanel() {
    const [connectorId, setConnectorId] = useState(n8nDashboardConnectors[0].id);
    const [intent, setIntent] = useState('Notify the launch channel when a release receipt is ready.');
    const [receipt, setReceipt] = useState<N8nRedactedReceipt | null>(null);
    const readiness = buildN8nWorkflowReadiness(connectorId);

    const prepareReceipt = () => {
        setReceipt(createRedactedN8nReceipt({ connectorId, intent }));
    };

    return (
        <section
            class="automation-workflow-panel"
            data-dx-automation-dashboard-workflow="connector-readiness"
            data-dx-component="dashboard-automation-workflow"
            data-dx-icon-search="automation:workflow"
            data-dx-node-modules="forbidden"
            data-dx-package="automations/n8n"
            data-dx-source-mirror={n8nAutomationForgeMetadata.sourceMirror}
        >
            <header class="panel-header">
                <dx-icon name="pack:workflow" aria-label="Automation workflow" />
                <div>
                    <h2>Automation Connectors</h2>
                    <p>Draft a redacted Automation Connectors receipt from source-owned n8n connector metadata.</p>
                </div>
            </header>

            <div class="provider-options" data-dx-automation-interaction="connector-picker">
                {n8nDashboardConnectors.map((connector) => (
                    <button
                        key={connector.id}
                        type="button"
                        class={connector.id === connectorId ? 'active' : ''}
                        data-dx-automation-connector={connector.id}
                        data-dx-automation-connector-selected={connector.id === connectorId ? 'true' : 'false'}
                        data-dx-automation-connector-status={connector.status}
                        onClick={() => {
                            setConnectorId(connector.id);
                            setReceipt(null);
                        }}
                    >
                        <dx-icon name={connector.trigger ? 'pack:play' : 'pack:workflow'} aria-hidden="true" />
                        {connector.displayName}
                    </button>
                ))}
            </div>

            <dl
                class="readiness-list"
                data-dx-automation-interaction="workflow-readiness"
                data-dx-automation-missing-config={readiness.missingEnv.length > 0 ? 'true' : 'false'}
                data-dx-automation-selected-connector={readiness.connector.id}
                data-dx-automation-workflow-status={readiness.status}
            >
                <div>
                    <dt>Source file</dt>
                    <dd data-dx-automation-source-file={readiness.connector.sourceFile}>
                        {readiness.connector.sourceFile}
                    </dd>
                </div>
                <div>
                    <dt>Credential boundary</dt>
                    <dd data-dx-automation-required-env={readiness.missingEnv.join(',')}>
                        {readiness.boundary}
                    </dd>
                </div>
                <div>
                    <dt>Operations</dt>
                    <dd data-dx-automation-operations={readiness.connector.operations.join(',')}>
                        {readiness.connector.operations.join(', ')}
                    </dd>
                </div>
            </dl>

            <label class="prompt-field" data-dx-automation-interaction="workflow-intent">
                Workflow intent
                <textarea
                    value={intent}
                    data-dx-automation-intent-input="dashboard-workflow"
                    onChange={(event) => setIntent((event.target as HTMLTextAreaElement).value)}
                />
            </label>

            <button
                type="button"
                class="primary-action"
                data-dx-automation-action="prepare-redacted-receipt"
                data-dx-automation-local-receipt="dashboard-receipt"
                onClick={prepareReceipt}
            >
                <dx-icon name="pack:receipt" aria-hidden="true" />
                Prepare redacted receipt
            </button>

            <p
                class="assistant-receipt"
                data-dx-automation-intent-preview={receipt?.workflowIntent || intent}
                data-dx-automation-receipt-state={receipt ? 'created' : 'idle'}
                data-dx-automation-receipt-status={receipt ? receipt.status : 'idle'}
                data-dx-automation-receipt-path={receipt?.receiptPath || n8nAutomationForgeMetadata.receiptPaths[0]}
            >
                {receipt
                    ? `${receipt.connectorName}: ${receipt.nextAction} Receipt intent: ${receipt.workflowIntent || 'No intent entered'}. Receipt stays redacted at ${receipt.receiptPath}.`
                    : 'No automation receipt has been drafted yet.'}
            </p>
        </section>
    );
}
