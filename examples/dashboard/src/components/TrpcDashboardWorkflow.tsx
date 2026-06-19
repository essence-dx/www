import { useState } from 'dx';
import {
    createTrpcDashboardReceipt,
    getTrpcDashboardProcedure,
    trpcDashboardPackage,
    trpcDashboardProcedures,
    type TrpcDashboardProcedureId,
    type TrpcDashboardReceipt,
} from '../lib/trpcDashboardWorkflow';

export function TrpcDashboardWorkflow() {
    const [procedureId, setProcedureId] =
        useState<TrpcDashboardProcedureId>('health.query');
    const [sequence, setSequence] = useState(1);
    const [receipt, setReceipt] = useState<TrpcDashboardReceipt | null>(null);
    const activeProcedure = getTrpcDashboardProcedure(procedureId);

    return (
        <section
            class="trpc-dashboard-panel"
            data-dx-package="api/trpc"
            data-dx-component="dashboard-trpc-workflow"
            data-dx-dashboard-workflow="typed-api-boundary"
            data-dx-style-surface="theme-token"
            data-dx-trpc-procedure={activeProcedure.id}
            data-dx-trpc-runtime={receipt ? receipt.status : 'ready-for-runtime'}
            data-dx-node-modules="forbidden"
        >
            <header class="panel-header">
                <dx-icon name="api:trpc" aria-label="tRPC" />
                <div>
                    <h2>Type-Safe API workflow</h2>
                    <p>
                        Prepare dashboard procedures against the source-owned
                        upstream tRPC router, client, and TanStack Query boundary.
                    </p>
                </div>
            </header>

            <div class="provider-options" data-dx-trpc-interaction="procedure-picker">
                {trpcDashboardProcedures.map((procedure) => (
                    <button
                        key={procedure.id}
                        type="button"
                        class={procedure.id === activeProcedure.id ? 'active' : ''}
                        data-dx-trpc-action="select-procedure"
                        data-dx-trpc-procedure-option={procedure.id}
                        data-dx-trpc-selected={
                            procedure.id === activeProcedure.id ? 'true' : 'false'
                        }
                        onClick={() => {
                            setProcedureId(procedure.id);
                            setReceipt(null);
                        }}
                    >
                        {procedure.label}
                    </button>
                ))}
            </div>

            <dl class="readiness-list" data-dx-trpc-readiness="app-owned">
                <div>
                    <dt>Source API</dt>
                    <dd data-dx-trpc-source-api={activeProcedure.sourceApi}>
                        {activeProcedure.sourceApi}
                    </dd>
                </div>
                <div>
                    <dt>Dashboard use</dt>
                    <dd data-dx-trpc-dashboard-use={activeProcedure.id}>
                        {activeProcedure.dashboardUse}
                    </dd>
                </div>
                <div>
                    <dt>Required env</dt>
                    <dd data-dx-trpc-required-env="none">
                        None for local receipt; runtime packages are app-owned.
                    </dd>
                </div>
                <div>
                    <dt>Boundary</dt>
                    <dd data-dx-trpc-boundary={activeProcedure.appOwnedBoundary}>
                        {activeProcedure.appOwnedBoundary}
                    </dd>
                </div>
            </dl>

            <button
                type="button"
                class="primary-action"
                data-dx-trpc-action="prepare-local-receipt"
                onClick={() => {
                    setReceipt(createTrpcDashboardReceipt(activeProcedure.id, sequence));
                    setSequence(sequence + 1);
                }}
            >
                <dx-icon name="api:trpc" aria-hidden="true" />
                Prepare API receipt
            </button>

            <p
                class="assistant-receipt"
                data-dx-trpc-receipt-state={receipt ? receipt.status : 'idle'}
                data-dx-trpc-request-id={receipt?.requestId || 'idle'}
                data-dx-trpc-source-mirror={trpcDashboardPackage.sourceMirror}
            >
                {receipt
                    ? `${receipt.packageId} ${receipt.procedure} prepared ${receipt.requestId}. ${receipt.cacheAction}. ${receipt.nextAction}`
                    : `Source mirror: ${trpcDashboardPackage.sourceMirror}. Choose a procedure and prepare a local receipt before runtime wiring.`}
            </p>
        </section>
    );
}
