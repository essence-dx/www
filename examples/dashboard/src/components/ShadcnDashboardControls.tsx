import { useState } from 'dx';
import {
    createShadcnDashboardReceipt,
    shadcnDashboardAccentOptions,
    shadcnDashboardDensityOptions,
    shadcnDashboardPackageMetadata as metadata,
    shadcnDashboardPackages,
    type ShadcnDashboardAccent,
    type ShadcnDashboardDensity,
    type ShadcnDashboardReceipt,
} from '../lib/shadcnDashboardControls';

export const shadcnDashboardPackageMetadata = metadata;

export function ShadcnDashboardControls() {
    const [density, setDensity] = useState<ShadcnDashboardDensity>('comfortable');
    const [accent, setAccent] = useState<ShadcnDashboardAccent>('launch');
    const [notifications, setNotifications] = useState(true);
    const [filter, setFilter] = useState('billing settings');
    const [note, setNote] = useState('Keep the launch dashboard readable for Friday review.');
    const [receipt, setReceipt] = useState<ShadcnDashboardReceipt | null>(null);

    const activeDensity =
        shadcnDashboardDensityOptions.find((option) => option.id === density) ||
        shadcnDashboardDensityOptions[0];
    const activeAccent =
        shadcnDashboardAccentOptions.find((option) => option.id === accent) ||
        shadcnDashboardAccentOptions[0];
    const activePackage =
        shadcnDashboardPackages.find((item) => item.id === activeAccent.packageId) ||
        shadcnDashboardPackages[0];

    const previewReceipt = () => {
        setReceipt(
            createShadcnDashboardReceipt({
                density,
                accent,
                filter,
                notifications,
            }),
        );
    };

    return (
        <section
            class="shadcn-dashboard-controls"
            data-dx-package="shadcn/ui/button"
            data-dx-component="dashboard-shadcn-controls"
            data-shadcn-dashboard-workflow="settings-review"
            data-shadcn-dashboard-density={density}
            data-shadcn-dashboard-accent={accent}
            data-dx-source-mirror={metadata.sourceMirror}
            data-dx-node-modules="forbidden"
            data-dx-style-surface="theme-token-card"
        >
            <header class="panel-header" data-slot="card-header">
                <dx-icon name="pack:settings" aria-label="Settings controls" />
                <div>
                    <h2 data-slot="card-title">UI Components dashboard controls</h2>
                    <p data-slot="card-description">
                        Compose real source-owned primitives into a settings workflow with DX-owned
                        receipts and upstream provenance.
                    </p>
                </div>
            </header>

            <div
                class="package-chips"
                data-slot="card"
                data-shadcn-dashboard-packages={shadcnDashboardPackages
                    .map((item) => item.id)
                    .join(',')}
            >
                {shadcnDashboardPackages.map((item) => (
                    <span
                        key={item.id}
                        class="package-chip"
                        data-slot="badge"
                        data-variant={item.id === activePackage.id ? 'default' : 'secondary'}
                        data-dx-package={item.id}
                        data-shadcn-dashboard-package={item.id}
                        title={`${item.publicApi}: ${item.role}`}
                    >
                        {item.label}
                    </span>
                ))}
            </div>

            <div class="provider-options" data-slot="card-content">
                {shadcnDashboardDensityOptions.map((option) => (
                    <button
                        key={option.id}
                        type="button"
                        class={option.id === density ? 'active' : ''}
                        data-slot="button"
                        data-variant={option.id === density ? 'default' : 'outline'}
                        data-size="sm"
                        data-shadcn-dashboard-action="select-density"
                        data-shadcn-dashboard-density-option={option.id}
                        data-shadcn-dashboard-selected={
                            option.id === density ? 'true' : 'false'
                        }
                        onClick={() => {
                            setDensity(option.id);
                            setReceipt(null);
                        }}
                    >
                        {option.label}
                    </button>
                ))}
                <button
                    type="button"
                    class={notifications ? 'active' : ''}
                    data-slot="button"
                    data-variant={notifications ? 'secondary' : 'outline'}
                    data-size="sm"
                    data-shadcn-dashboard-action="toggle-notification"
                    data-shadcn-dashboard-notifications={notifications ? 'on' : 'off'}
                    onClick={() => {
                        setNotifications(!notifications);
                        setReceipt(null);
                    }}
                >
                    {notifications ? 'Alerts on' : 'Alerts off'}
                </button>
            </div>

            <div class="settings-grid" data-slot="field-group">
                <label
                    class="prompt-field"
                    data-slot="field"
                    data-orientation="vertical"
                    data-shadcn-dashboard-field="filter"
                >
                    <span data-slot="field-label">
                        <dx-icon name="pack:search" aria-hidden="true" />
                        Control filter
                    </span>
                    <input
                        value={filter}
                        data-slot="input"
                        data-shadcn-dashboard-input="filter"
                        onChange={(event) => {
                            setFilter((event.target as HTMLInputElement).value);
                            setReceipt(null);
                        }}
                    />
                </label>

                <label
                    class="prompt-field"
                    data-slot="field"
                    data-orientation="vertical"
                    data-shadcn-dashboard-field="note"
                >
                    <span data-slot="field-label">Review note</span>
                    <textarea
                        value={note}
                        data-slot="textarea"
                        data-shadcn-dashboard-input="note"
                        onChange={(event) => setNote((event.target as HTMLTextAreaElement).value)}
                    />
                </label>
            </div>

            <div
                class="provider-options"
                data-slot="item-group"
                data-shadcn-dashboard-action="select-accent"
            >
                {shadcnDashboardAccentOptions.map((option) => (
                    <button
                        key={option.id}
                        type="button"
                        class={option.id === accent ? 'active' : ''}
                        data-slot="item"
                        data-variant={option.id === accent ? 'default' : 'outline'}
                        data-size="sm"
                        data-shadcn-dashboard-accent-option={option.id}
                        data-shadcn-dashboard-selected={
                            option.id === accent ? 'true' : 'false'
                        }
                        onClick={() => {
                            setAccent(option.id);
                            setReceipt(null);
                        }}
                    >
                        <span data-slot="item-content">{option.label}</span>
                    </button>
                ))}
            </div>

            <div
                class="readiness-list"
                data-slot="separator"
                data-orientation="horizontal"
                role="separator"
            />

            <dl class="readiness-list" data-slot="card-content">
                <div data-slot="item" data-shadcn-dashboard-summary="density">
                    <dt>Density</dt>
                    <dd>{activeDensity.description}</dd>
                </div>
                <div data-slot="item" data-shadcn-dashboard-summary="api">
                    <dt>Active API</dt>
                    <dd data-shadcn-dashboard-public-api={activePackage.publicApi}>
                        {activePackage.publicApi}
                    </dd>
                </div>
                <div data-slot="item" data-shadcn-dashboard-summary="boundary">
                    <dt>App-owned boundary</dt>
                    <dd>{metadata.appOwnedBoundaries[0]}</dd>
                </div>
            </dl>

            <button
                type="button"
                class="primary-action"
                data-slot="button"
                data-variant="default"
                data-size="default"
                data-shadcn-dashboard-action="preview-save-receipt"
                onClick={previewReceipt}
            >
                <dx-icon name="pack:check" aria-hidden="true" />
                Preview settings receipt
            </button>

            <p
                class="assistant-receipt"
                data-shadcn-dashboard-receipt={receipt ? receipt.receiptId : 'idle'}
                data-shadcn-dashboard-status={receipt ? receipt.status : 'idle'}
                data-shadcn-dashboard-note-length={String(note.length)}
            >
                {receipt
                    ? `${receipt.receiptId}: ${receipt.filter} is ready for an app-owned save action. ${receipt.nextAction}`
                    : `Source mirror: ${metadata.sourceMirror}. Choose controls and preview a local receipt.`}
            </p>
        </section>
    );
}
