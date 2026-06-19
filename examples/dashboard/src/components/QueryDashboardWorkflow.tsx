import { useState } from 'dx';
import {
    createQueryDashboardReceipt,
    getQueryDashboardProfile,
    queryDashboardPackage,
    queryDashboardProfiles,
    type QueryDashboardProfileId,
    type QueryDashboardReceipt,
} from '../lib/queryDashboardWorkflow';

export function QueryDashboardWorkflow() {
    const [profileId, setProfileId] =
        useState<QueryDashboardProfileId>('balanced');
    const [receipt, setReceipt] = useState<QueryDashboardReceipt | null>(null);
    const activeProfile = getQueryDashboardProfile(profileId);
    const dashboardReceiptPath =
        receipt?.receiptPath ?? queryDashboardPackage.receiptPaths[0];

    return (
        <section
            class="query-dashboard-panel"
            data-dx-package="tanstack/query"
            data-dx-component="dashboard-tanstack-query-workflow"
            data-dx-dashboard-workflow="query-cache-refresh"
            data-dx-style-surface="theme-token"
            data-dx-query-profile={activeProfile.id}
            data-dx-query-check-visibility={
                receipt?.dxCheckVisibility.currentStatus ??
                queryDashboardPackage.dxCheckVisibility.currentStatus
            }
            data-dx-query-refresh-state={receipt ? receipt.status : 'idle'}
            data-dx-query-public-api={activeProfile.publicApi}
            data-dx-query-stale-time-ms={activeProfile.staleTimeMs}
            data-dx-query-gc-time-ms={activeProfile.gcTimeMs}
            data-dx-query-retry={String(activeProfile.retry)}
            data-dx-query-dashboard-receipt-path={dashboardReceiptPath}
            data-dx-query-runtime-execution={
                receipt ? String(receipt.runtimeExecution) : 'false'
            }
            data-dx-official-package="Data Fetching & Cache"
            data-dx-upstream-package={queryDashboardPackage.upstreamPackage}
            data-dx-node-modules="forbidden"
        >
            <header class="panel-header">
                <dx-icon name="pack:tanstack-query" aria-label="Data Fetching & Cache" />
                <div>
                    <h2>Data Fetching &amp; Cache workflow</h2>
                    <p>
                        Choose the dashboard cache policy and prepare the refresh
                        receipt before the app owns the real QueryClient runtime.
                    </p>
                </div>
            </header>

            <div class="provider-options" data-dx-query-interaction="profile-picker">
                {queryDashboardProfiles.map((profile) => (
                    <button
                        key={profile.id}
                        type="button"
                        class={profile.id === activeProfile.id ? 'active' : ''}
                        data-dx-query-action="select-cache-profile"
                        data-dx-query-profile-option={profile.id}
                        data-dx-query-selected={
                            profile.id === activeProfile.id ? 'true' : 'false'
                        }
                        onClick={() => {
                            setProfileId(profile.id);
                            setReceipt(null);
                        }}
                    >
                        {profile.label}
                    </button>
                ))}
            </div>

            <dl class="readiness-list" data-dx-query-readiness="app-owned">
                <div>
                    <dt>Source API</dt>
                    <dd data-dx-query-public-api={activeProfile.publicApi}>
                        {activeProfile.publicApi}
                    </dd>
                </div>
                <div>
                    <dt>Freshness</dt>
                    <dd
                        data-dx-query-data-freshness={activeProfile.dataFreshness}
                        data-dx-query-stale-time-ms={activeProfile.staleTimeMs}
                    >
                        {activeProfile.staleTimeLabel}
                    </dd>
                </div>
                <div>
                    <dt>Cache defaults</dt>
                    <dd
                        data-dx-query-cache-defaults="profile"
                        data-dx-query-gc-time-ms={activeProfile.gcTimeMs}
                        data-dx-query-retry={String(activeProfile.retry)}
                    >
                        {activeProfile.gcTimeMs}ms gc, retry {String(activeProfile.retry)}
                    </dd>
                </div>
                <div>
                    <dt>Required env</dt>
                    <dd data-dx-query-required-env="none">
                        None; runtime package install remains app-owned.
                    </dd>
                </div>
                <div>
                    <dt>Boundary</dt>
                    <dd data-dx-query-boundary={activeProfile.appOwnedBoundary}>
                        {activeProfile.appOwnedBoundary}
                    </dd>
                </div>
                <div>
                    <dt>Receipt</dt>
                    <dd data-dx-query-receipt-path={dashboardReceiptPath}>
                        {dashboardReceiptPath}
                    </dd>
                </div>
            </dl>

            <button
                type="button"
                class="primary-action"
                data-dx-query-action="prepare-refresh-receipt"
                onClick={() =>
                    setReceipt(createQueryDashboardReceipt(activeProfile.id))
                }
            >
                <dx-icon name="pack:tanstack-query" aria-hidden="true" />
                Prepare refresh receipt
            </button>

            <p
                class="assistant-receipt"
                data-dx-query-receipt-state={receipt ? receipt.status : 'idle'}
                data-dx-query-source-mirror={queryDashboardPackage.sourceMirror}
                data-dx-query-key={receipt ? receipt.queryKey.join('.') : 'idle'}
                data-dx-query-receipt-path={dashboardReceiptPath}
                data-dx-query-check-visibility={
                    receipt?.dxCheckVisibility.currentStatus ??
                    queryDashboardPackage.dxCheckVisibility.currentStatus
                }
                data-dx-query-receipt-public-api={
                    receipt ? receipt.publicApi : activeProfile.publicApi
                }
                data-dx-query-cache-defaults={
                    receipt
                        ? `${receipt.cacheDefaults.staleTimeMs}:${receipt.cacheDefaults.gcTimeMs}:${receipt.cacheDefaults.retry}`
                        : 'idle'
                }
                data-dx-query-runtime-execution={
                    receipt ? String(receipt.runtimeExecution) : 'false'
                }
            >
                {receipt
                    ? `${receipt.officialName} ${receipt.profileId} prepared for ${receipt.queryKey.join('.')}. API: ${receipt.publicApi}. Upstream: ${receipt.upstreamPackage}. Receipt: ${receipt.receiptPath}. dx-check: ${receipt.dxCheckVisibility.currentStatus}. Defaults: stale ${receipt.cacheDefaults.staleTimeMs}ms, gc ${receipt.cacheDefaults.gcTimeMs}ms, retry ${String(receipt.cacheDefaults.retry)}. ${receipt.cacheAction} ${receipt.nextAction}`
                    : `Source mirror: ${queryDashboardPackage.sourceMirror}. Upstream: ${queryDashboardPackage.upstreamPackage}. dx-check: ${queryDashboardPackage.dxCheckVisibility.currentStatus}. Choose a cache profile and prepare a local refresh receipt.`}
            </p>
        </section>
    );
}
