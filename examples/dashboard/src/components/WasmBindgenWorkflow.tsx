import { useState } from 'dx';
import {
    createWasmBindgenDashboardReceipt,
    getWasmBindgenDashboardScenario,
    wasmBindgenDashboardPackage,
    wasmBindgenDashboardScenarios,
    type WasmBindgenDashboardReceipt,
    type WasmBindgenDashboardScenarioId,
} from '../lib/wasmBindgenDashboard';

export function WasmBindgenWorkflow() {
    const [scenarioId, setScenarioId] =
        useState<WasmBindgenDashboardScenarioId>('local-add');
    const [receipt, setReceipt] =
        useState<WasmBindgenDashboardReceipt | null>(null);
    const [busy, setBusy] = useState(false);
    const activeScenario = getWasmBindgenDashboardScenario(scenarioId);

    const runWorkflow = async () => {
        setBusy(true);
        try {
            setReceipt(await createWasmBindgenDashboardReceipt(scenarioId));
        } finally {
            setBusy(false);
        }
    };

    return (
        <section
            class="wasm-bindgen-workflow-panel"
            data-dx-package="wasm/bindgen"
            data-dx-component="dashboard-wasm-bindgen-workflow"
            data-dx-dashboard-workflow="wasm-interop"
            data-dx-wasm-dashboard-scenario={activeScenario.id}
            data-dx-wasm-dashboard-status={receipt ? receipt.status : 'idle'}
            data-dx-source-mirror={wasmBindgenDashboardPackage.sourceMirror}
            data-dx-style-surface="theme-token-card"
            data-dx-icon-search="wasm:bindgen"
            data-dx-node-modules="forbidden"
        >
            <header class="panel-header">
                <dx-icon name="pack:wasm-bindgen" aria-label="WebAssembly" />
                <div>
                    <h2>WebAssembly Bridge workflow</h2>
                    <p>
                        Run a browser WebAssembly proof while keeping generated
                        wasm-bindgen glue app-owned.
                    </p>
                </div>
            </header>

            <div class="provider-options" data-dx-wasm-interaction="scenario-picker">
                {wasmBindgenDashboardScenarios.map((scenario) => (
                    <button
                        key={scenario.id}
                        type="button"
                        class={scenario.id === activeScenario.id ? 'active' : ''}
                        data-dx-wasm-dashboard-action="select-scenario"
                        data-dx-wasm-dashboard-option={scenario.id}
                        data-dx-wasm-dashboard-selected={
                            scenario.id === activeScenario.id ? 'true' : 'false'
                        }
                        onClick={() => {
                            setScenarioId(scenario.id);
                            setReceipt(null);
                        }}
                    >
                        <dx-icon name="pack:wasm-bindgen" aria-hidden="true" />
                        {scenario.label}
                    </button>
                ))}
            </div>

            <dl class="readiness-list" data-dx-wasm-readiness="app-owned">
                <div>
                    <dt>Public API</dt>
                    <dd data-dx-wasm-public-api={activeScenario.publicApi}>
                        {activeScenario.publicApi}
                    </dd>
                </div>
                <div>
                    <dt>Source mirror</dt>
                    <dd data-dx-wasm-source-mirror={wasmBindgenDashboardPackage.sourceMirror}>
                        {wasmBindgenDashboardPackage.sourceMirror}
                    </dd>
                </div>
                <div>
                    <dt>Boundary</dt>
                    <dd data-dx-wasm-app-boundary={activeScenario.boundary}>
                        {activeScenario.boundary}
                    </dd>
                </div>
            </dl>

            <button
                type="button"
                class="primary-action"
                data-dx-wasm-dashboard-action="run-local-add"
                disabled={busy}
                onClick={runWorkflow}
            >
                <dx-icon name="pack:play" aria-hidden="true" />
                {busy ? 'Running WebAssembly...' : 'Run WebAssembly workflow'}
            </button>

            <p
                class="assistant-receipt"
                data-dx-wasm-dashboard-receipt={receipt ? receipt.status : 'idle'}
                data-dx-wasm-add-result={receipt?.result ?? 'idle'}
            >
                {receipt
                    ? `${receipt.packageId} ${receipt.status}; result ${receipt.result ?? 'none'}. ${receipt.nextAction}`
                    : 'No WebAssembly workflow has been run yet.'}
            </p>
        </section>
    );
}
