import { useState } from 'dx';
import {
    createDrizzleDashboardReceipt,
    drizzleDashboardPackage,
    drizzleDashboardQueries,
    drizzleDashboardWorkflowReceiptPath,
    getDrizzleDashboardQuery,
    readDrizzleDashboardRuntimeReadiness,
    type DrizzleDashboardQueryId,
    type DrizzleDashboardReceipt,
} from '../lib/drizzleDashboard';

export function DrizzleDashboardWorkflow() {
    const [queryId, setQueryId] =
        useState<DrizzleDashboardQueryId>('overview');
    const [receipt, setReceipt] = useState<DrizzleDashboardReceipt | null>(
        null,
    );
    const activeQuery = getDrizzleDashboardQuery(queryId);
    const runtimeReadiness = readDrizzleDashboardRuntimeReadiness();

    return (
        <section
            class="drizzle-dashboard-panel"
            data-dx-package="db/drizzle-sqlite"
            data-dx-component="dashboard-drizzle-workflow"
            data-dx-drizzle-dashboard-workflow="content-readiness"
            data-dx-drizzle-query={activeQuery.id}
            data-dx-drizzle-runtime={runtimeReadiness.status}
            data-dx-drizzle-runtime-dependencies={
                runtimeReadiness.runtimeDependencies.join(',')
            }
            data-dx-drizzle-receipt-path={drizzleDashboardWorkflowReceiptPath}
            data-dx-node-modules="forbidden"
        >
            <header class="panel-header">
                <dx-icon name="pack:database" aria-label="Database" />
                <div>
                    <h2>Drizzle dashboard workflow</h2>
                    <p>
                        Prepare the starter dashboard to read real Drizzle
                        SQLite content once the app owns the runtime.
                    </p>
                </div>
            </header>

            <div class="provider-options" data-dx-drizzle-interaction="query-picker">
                {drizzleDashboardQueries.map((query) => (
                    <button
                        key={query.id}
                        type="button"
                        class={query.id === activeQuery.id ? 'active' : ''}
                        data-dx-drizzle-action="select-dashboard-query"
                        data-dx-drizzle-query-option={query.id}
                        data-dx-drizzle-selected={
                            query.id === activeQuery.id ? 'true' : 'false'
                        }
                        onClick={() => {
                            setQueryId(query.id);
                            setReceipt(null);
                        }}
                    >
                        {query.label}
                    </button>
                ))}
            </div>

            <dl class="readiness-list" data-dx-drizzle-readiness="app-owned">
                <div>
                    <dt>Package export</dt>
                    <dd data-dx-drizzle-export={activeQuery.packageExport}>
                        {activeQuery.packageExport}
                    </dd>
                </div>
                <div>
                    <dt>Public API</dt>
                    <dd data-dx-drizzle-public-api={activeQuery.publicApi}>
                        {activeQuery.publicApi}
                    </dd>
                </div>
                <div>
                    <dt>Result shape</dt>
                    <dd data-dx-drizzle-result-shape={activeQuery.resultShape}>
                        {activeQuery.resultShape}
                    </dd>
                </div>
                <div>
                    <dt>App-owned boundary</dt>
                    <dd
                        data-dx-drizzle-required-env="none"
                        data-dx-drizzle-runtime-dependencies={
                            runtimeReadiness.runtimeDependencies.join(',')
                        }
                    >
                        {drizzleDashboardPackage.appOwnedBoundaries.join(', ')}
                    </dd>
                </div>
            </dl>

            <div
                class="query-preview"
                data-dx-drizzle-sql-preview={activeQuery.id}
                data-dx-drizzle-action="preview-dashboard-query-plan"
                data-dx-drizzle-query-plan-export={activeQuery.queryPlanExport}
            >
                <span>Safe query plan preview</span>
                <code>{activeQuery.sqlPreview}</code>
            </div>

            <button
                type="button"
                class="primary-action"
                data-dx-drizzle-action="prepare-dashboard-query"
                onClick={() =>
                    setReceipt(createDrizzleDashboardReceipt(activeQuery.id))
                }
            >
                <dx-icon name="pack:database" aria-hidden="true" />
                Prepare DB receipt
            </button>

            <p
                class="assistant-receipt"
                data-dx-drizzle-receipt-state={receipt ? receipt.status : 'idle'}
                data-dx-drizzle-receipt-path={
                    receipt?.receiptPath ?? drizzleDashboardWorkflowReceiptPath
                }
                data-dx-drizzle-source-mirror={drizzleDashboardPackage.sourceMirror}
            >
                {receipt
                    ? `${receipt.packageId} ${receipt.queryId} is ready for app-owned SQLite runtime setup. ${receipt.nextAction}`
                    : `Source mirror: ${drizzleDashboardPackage.sourceMirror}. No dashboard DB read has been prepared yet.`}
            </p>
        </section>
    );
}
