import { useContext, useState } from 'dx';
import { AuthContext } from './AuthProvider';
import {
    betterAuthDashboardPackage,
    createDxBetterAuthDashboardActionReceipt,
    createDxBetterAuthDashboardProfileRequest,
    createDxBetterAuthDashboardSessionSnapshot,
    dxBetterAuthDashboardActions,
    type BetterAuthDashboardActionId,
    type BetterAuthDashboardActionReceipt,
    type BetterAuthDashboardProvider,
} from '../lib/forge/auth/better-auth/dashboard';

export function BetterAuthAccountWorkflow() {
    const auth = useContext(AuthContext);
    const sessionSnapshot = createDxBetterAuthDashboardSessionSnapshot(auth?.user);
    const [actionId, setActionId] = useState<BetterAuthDashboardActionId>('read-session');
    const [provider, setProvider] = useState<BetterAuthDashboardProvider>('google');
    const [name, setName] = useState(auth?.user?.name || 'DX Launch Operator');
    const [email, setEmail] = useState(auth?.user?.email || 'operator@example.com');
    const [receipt, setReceipt] = useState<BetterAuthDashboardActionReceipt | null>(null);
    const [error, setError] = useState('');

    const selectedAction =
        dxBetterAuthDashboardActions.find((action) => action.id === actionId) ||
        dxBetterAuthDashboardActions[0];

    const prepareReceipt = () => {
        setError('');

        try {
            if (actionId === 'update-profile') {
                createDxBetterAuthDashboardProfileRequest({
                    name,
                    email,
                    provider,
                });
            }

            setReceipt(
                createDxBetterAuthDashboardActionReceipt({
                    actionId,
                    draft: {
                        name,
                        email,
                        provider,
                    },
                }),
            );
        } catch (workflowError) {
            setReceipt(null);
            setError(
                workflowError instanceof Error
                    ? workflowError.message
                    : 'Authentication action could not be prepared.',
            );
        }
    };

    return (
        <section
            class="settings-section"
            data-dx-package="auth/better-auth"
            data-dx-component="dashboard-better-auth-account-workflow"
            data-dx-auth-dashboard-workflow="session-profile-account"
            data-dx-auth-session-state={sessionSnapshot.status}
            data-dx-auth-selected-action={selectedAction.id}
            data-dx-auth-dashboard-receipt={receipt ? receipt.receiptId : 'none'}
            data-dx-icon-search="auth:account"
            data-dx-node-modules="forbidden"
            data-dx-source-mirror={betterAuthDashboardPackage.sourceMirror}
            data-dx-style-surface="theme-token-card"
        >
            <header class="panel-header">
                <dx-icon name="pack:auth" aria-label="Authentication" />
                <div>
                    <h2>Authentication account workflow</h2>
                    <p>
                        Inspect the local dashboard session and prepare real Authentication
                        actions backed by upstream better-auth APIs behind explicit app-owned boundaries.
                    </p>
                </div>
            </header>

            <dl class="readiness-list" data-dx-auth-readiness="source-owned">
                <div>
                    <dt>Package</dt>
                    <dd data-dx-auth-package-id={betterAuthDashboardPackage.packageId}>
                        {betterAuthDashboardPackage.packageId}
                    </dd>
                </div>
                <div>
                    <dt>Required env</dt>
                    <dd data-dx-auth-required-env={betterAuthDashboardPackage.requiredEnv.join(',')}>
                        {betterAuthDashboardPackage.requiredEnv.join(', ')}
                    </dd>
                </div>
                <div>
                    <dt>Session source</dt>
                    <dd data-dx-auth-session-source={sessionSnapshot.publicApi.join(',')}>
                        {sessionSnapshot.userLabel} · {sessionSnapshot.role}
                    </dd>
                </div>
            </dl>

            <div class="provider-options" data-dx-auth-interaction="action-picker">
                {dxBetterAuthDashboardActions.map((action) => (
                    <button
                        key={action.id}
                        type="button"
                        class={action.id === selectedAction.id ? 'active' : ''}
                        data-dx-auth-action="select-dashboard-action"
                        data-dx-auth-action-id={action.id}
                        data-dx-auth-action-selected={
                            action.id === selectedAction.id ? 'true' : 'false'
                        }
                        onClick={() => {
                            setActionId(action.id);
                            setReceipt(null);
                            setError('');
                        }}
                    >
                        {action.label}
                    </button>
                ))}
            </div>

            <div class="settings-grid" data-dx-auth-interaction="profile-draft">
                <label data-dx-auth-profile-field="name">
                    Profile name
                    <input
                        value={name}
                        data-dx-auth-profile-input="name"
                        onChange={(event) => {
                            setName((event.target as HTMLInputElement).value);
                            setReceipt(null);
                        }}
                    />
                </label>
                <label data-dx-auth-profile-field="email">
                    Account email
                    <input
                        value={email}
                        data-dx-auth-profile-input="email"
                        onChange={(event) => {
                            setEmail((event.target as HTMLInputElement).value);
                            setReceipt(null);
                        }}
                    />
                </label>
            </div>

            <div class="provider-options" data-dx-auth-interaction="provider-picker">
                {(['google'] as BetterAuthDashboardProvider[]).map((item) => (
                    <button
                        key={item}
                        type="button"
                        class={provider === item ? 'active' : ''}
                        data-dx-auth-provider-option={item}
                        data-dx-auth-provider-selected={provider === item ? 'true' : 'false'}
                        onClick={() => {
                            setProvider(item);
                            setReceipt(null);
                        }}
                    >
                        <dx-icon name="pack:account" aria-hidden="true" />
                        {item}
                    </button>
                ))}
            </div>

            <div class="query-box" data-dx-auth-public-api={selectedAction.publicApi.join(',')}>
                <strong>{selectedAction.publicApi.join(', ')}</strong>
                <span>{selectedAction.appOwnedBoundary}</span>
            </div>

            <button
                type="button"
                class="primary-action"
                data-dx-auth-action="prepare-dashboard-auth-receipt"
                data-dx-auth-action-state={receipt ? receipt.status : 'idle'}
                onClick={prepareReceipt}
            >
                <dx-icon name="pack:play" aria-hidden="true" />
                Prepare auth receipt
            </button>

            <p
                class="assistant-receipt"
                data-dx-auth-dashboard-receipt={receipt ? receipt.receiptId : 'none'}
                data-dx-auth-dashboard-status={receipt ? receipt.status : error ? 'error' : 'idle'}
                role={error ? 'alert' : undefined}
            >
                {receipt
                    ? `${receipt.receiptId}: ${receipt.message}`
                    : error || 'No Authentication dashboard receipt has been prepared yet.'}
            </p>
        </section>
    );
}
