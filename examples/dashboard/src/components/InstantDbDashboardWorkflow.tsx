import { useState } from 'dx';
import {
    createInstantDashboardReceipt,
    getInstantDashboardSurface,
    instantDashboardPackage,
    instantDashboardSurfaces,
    type InstantDashboardReceipt,
    type InstantDashboardSurfaceId,
} from '../lib/instantdbDashboard';

export function InstantDbDashboardWorkflow() {
    const [surfaceId, setSurfaceId] =
        useState<InstantDashboardSurfaceId>('realtime-todos');
    const [receipt, setReceipt] = useState<InstantDashboardReceipt | null>(
        null,
    );
    const activeSurface = getInstantDashboardSurface(surfaceId);

    return (
        <section
            class="instantdb-dashboard-panel"
            data-dx-package="instantdb/react"
            data-dx-component="dashboard-instantdb-workflow"
            data-dx-instant-dashboard-workflow="realtime-boundary"
            data-dx-instant-dashboard-surface={activeSurface.id}
            data-dx-instant-dashboard-status={
                receipt ? receipt.status : 'missing-config'
            }
            data-dx-instant-dashboard-dx-check-schema={
                instantDashboardPackage.dxCheckVisibility.schema
            }
            data-dx-instant-dashboard-dx-check-status={
                instantDashboardPackage.dxCheckVisibility.currentStatus
            }
            data-dx-node-modules="forbidden"
            data-dx-style-surface="realtime-app-database"
        >
            <header class="panel-header">
                <dx-icon name="pack:database" aria-label="Realtime App Database" />
                <div>
                    <h2>Realtime App Database workflow</h2>
                    <p>
                        Prepare realtime todos, room presence, auth, storage,
                        streams, and first-party routes behind app-owned Instant
                        credentials.
                    </p>
                </div>
            </header>

            <div class="provider-options" data-dx-instant-dashboard-interaction="surface-picker">
                {instantDashboardSurfaces.map((surface) => (
                    <button
                        key={surface.id}
                        type="button"
                        class={surface.id === activeSurface.id ? 'active' : ''}
                        data-dx-instant-dashboard-action="select-surface"
                        data-dx-instant-dashboard-option={surface.id}
                        data-dx-instant-dashboard-selected={
                            surface.id === activeSurface.id ? 'true' : 'false'
                        }
                        onClick={() => {
                            setSurfaceId(surface.id);
                            setReceipt(null);
                        }}
                    >
                        {surface.label}
                    </button>
                ))}
            </div>

            <dl class="readiness-list" data-dx-instant-dashboard-readiness="source-owned">
                <div>
                    <dt>Package</dt>
                    <dd data-dx-instant-dashboard-package-id={instantDashboardPackage.packageId}>
                        {instantDashboardPackage.packageId}
                    </dd>
                </div>
                <div>
                    <dt>Public API</dt>
                    <dd data-dx-instant-dashboard-public-api={activeSurface.publicApi}>
                        {activeSurface.publicApi}
                    </dd>
                </div>
                <div>
                    <dt>Required env</dt>
                    <dd data-dx-instant-dashboard-required-env="NEXT_PUBLIC_INSTANT_APP_ID">
                        {instantDashboardPackage.requiredEnv.join(', ')}
                    </dd>
                </div>
                <div>
                    <dt>App-owned boundary</dt>
                    <dd data-dx-instant-dashboard-boundary={activeSurface.appBoundary}>
                        {activeSurface.appBoundary}
                    </dd>
                </div>
                <div>
                    <dt>Receipts</dt>
                    <dd
                        data-dx-instant-dashboard-receipt-paths={instantDashboardPackage.receiptPaths.join(
                            '|',
                        )}
                    >
                        {instantDashboardPackage.receiptPaths.length} receipt paths
                    </dd>
                </div>
                <div>
                    <dt>dx-check</dt>
                    <dd
                        data-dx-instant-dashboard-dx-check-receipt={
                            instantDashboardPackage.dxCheckVisibility.receiptPath
                        }
                    >
                        {instantDashboardPackage.dxCheckVisibility.currentStatus}
                    </dd>
                </div>
                <div>
                    <dt>Provenance</dt>
                    <dd data-dx-instant-dashboard-provenance={instantDashboardPackage.provenance}>
                        local source mirror inspected
                    </dd>
                </div>
            </dl>

            <p
                class="assistant-receipt"
                data-dx-instant-dashboard-source-mirror={instantDashboardPackage.sourceMirror}
            >
                {activeSurface.dashboardUse}
            </p>

            <button
                type="button"
                class="primary-action"
                data-dx-instant-dashboard-action="prepare-local-receipt"
                onClick={() =>
                    setReceipt(createInstantDashboardReceipt(activeSurface.id))
                }
            >
                <dx-icon name="pack:database" aria-hidden="true" />
                Prepare Realtime App Database receipt
            </button>

            <output
                class="assistant-receipt"
                data-dx-instant-dashboard-receipt={receipt ? receipt.receiptId : 'none'}
                data-dx-instant-dashboard-receipt-status={
                    receipt ? receipt.status : 'idle'
                }
                data-dx-instant-dashboard-exported-files={instantDashboardPackage.exportedFiles.length}
            >
                {receipt
                    ? `${receipt.receiptId}: ${receipt.nextAction}`
                    : `Source mirror: ${instantDashboardPackage.sourceMirror}. No live Realtime App Database call runs until the app id and rules are configured.`}
            </output>
        </section>
    );
}
