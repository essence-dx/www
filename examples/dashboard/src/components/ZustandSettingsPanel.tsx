import { useEffect, useState } from 'dx';
import {
    clearSavedDashboardSettings,
    dashboardSettingsSnapshot,
    dashboardSettingsStore,
    rehydrateDashboardSettings,
    subscribeToDashboardSettings,
    type DashboardSettingsSnapshot,
} from '../lib/dashboardSettingsStore';

function formatSavedAt(value: string) {
    if (value === 'not-saved') return 'Not saved yet';
    return new Date(value).toLocaleTimeString();
}

type DashboardPersistEvent = 'ready' | 'hydrating' | 'hydrated' | 'cleared';

export function ZustandSettingsPanel() {
    const [settings, setSettings] = useState<DashboardSettingsSnapshot>(
        dashboardSettingsSnapshot(),
    );
    const [persistStatus, setPersistStatus] = useState<{
        event: DashboardPersistEvent;
        count: number;
        hydrated: boolean;
    }>({
        event: dashboardSettingsStore.persist.hasHydrated()
            ? 'hydrated'
            : 'ready',
        count: 0,
        hydrated: dashboardSettingsStore.persist.hasHydrated(),
    });

    useEffect(() => {
        return subscribeToDashboardSettings(snapshot => {
            setSettings(snapshot);
        });
    }, []);

    useEffect(() => {
        const unsubscribeHydrate = dashboardSettingsStore.persist.onHydrate(() => {
            setPersistStatus(status => ({
                ...status,
                event: 'hydrating',
                hydrated: false,
            }));
        });
        const unsubscribeFinish =
            dashboardSettingsStore.persist.onFinishHydration(() => {
                setPersistStatus(status => ({
                    event: 'hydrated',
                    count: status.count + 1,
                    hydrated: true,
                }));
            });

        return () => {
            unsubscribeHydrate();
            unsubscribeFinish();
        };
    }, []);

    const store = dashboardSettingsStore.getState();

    return (
        <section
            class="settings-section"
            data-dx-package="state/zustand"
            data-dx-component="dashboard-zustand-settings-workflow"
            data-dx-zustand-store="dashboard-settings"
            data-dx-zustand-persist-key="dx-dashboard-settings"
            data-dx-zustand-hydrated={String(persistStatus.hydrated)}
            data-dx-zustand-hydration-event={persistStatus.event}
            data-dx-zustand-hydration-count={String(persistStatus.count)}
            data-dx-zustand-density={settings.density}
            data-dx-zustand-sidebar-pinned={String(settings.sidebarPinned)}
            data-dx-zustand-command-hints={String(settings.commandHints)}
        >
            <div class="section-title">
                <dx-icon name="pack:state" aria-hidden="true" />
                <h2>Workspace State</h2>
            </div>
            <p>
                Dashboard settings use a source-owned Zustand-compatible store
                with selector subscriptions, shallow snapshot equality, and
                persisted browser storage.
            </p>

            <div class="settings-actions">
                <button
                    type="button"
                    data-dx-zustand-action="toggle-density"
                    onClick={() => store.toggleDensity()}
                >
                    Density: {settings.density}
                </button>
                <button
                    type="button"
                    data-dx-zustand-action="toggle-sidebar"
                    onClick={() => store.toggleSidebar()}
                >
                    Sidebar {settings.sidebarPinned ? 'pinned' : 'unpinned'}
                </button>
                <button
                    type="button"
                    data-dx-zustand-action="toggle-command-hints"
                    onClick={() => store.toggleCommandHints()}
                >
                    Hints {settings.commandHints ? 'on' : 'off'}
                </button>
                <button
                    type="button"
                    data-dx-zustand-action="rehydrate-settings"
                    onClick={() => rehydrateDashboardSettings()}
                >
                    Rehydrate
                </button>
                <button
                    type="button"
                    data-dx-zustand-action="clear-saved-settings"
                    onClick={() => {
                        clearSavedDashboardSettings();
                        setPersistStatus(status => ({
                            ...status,
                            event: 'cleared',
                        }));
                    }}
                >
                    Clear saved
                </button>
                <button
                    type="button"
                    class="save-btn"
                    data-dx-zustand-action="save-settings"
                    onClick={() => store.saveSettings()}
                >
                    Save state
                </button>
            </div>

            <p
                class="settings-status"
                data-dx-zustand-saved-at={settings.lastSavedAt}
            >
                Persisted as dx-dashboard-settings. Hydration:{' '}
                {persistStatus.event}.{' '}
                {formatSavedAt(settings.lastSavedAt)}
            </p>
        </section>
    );
}
