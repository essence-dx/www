import {
    createStore,
    persist,
    shallow,
    subscribeWithSelector,
    type PersistStoreApi,
    type StoreApi,
} from './forge/state/zustand';

export type DashboardDensity = 'compact' | 'comfortable';

export interface DashboardSettingsState {
    density: DashboardDensity;
    sidebarPinned: boolean;
    commandHints: boolean;
    lastSavedAt: string;
    toggleDensity: () => void;
    toggleSidebar: () => void;
    toggleCommandHints: () => void;
    saveSettings: () => void;
}

export type DashboardSettingsSnapshot = Pick<
    DashboardSettingsState,
    'density' | 'sidebarPinned' | 'commandHints' | 'lastSavedAt'
>;

const DEFAULT_DASHBOARD_SETTINGS: DashboardSettingsSnapshot = {
    density: 'comfortable',
    sidebarPinned: true,
    commandHints: true,
    lastSavedAt: 'not-saved',
};

export const dashboardSettingsStore = createStore<DashboardSettingsState>()(
    persist(
        subscribeWithSelector(set => ({
            ...DEFAULT_DASHBOARD_SETTINGS,
            toggleDensity: () =>
                set(state => ({
                    density:
                        state.density === 'comfortable'
                            ? 'compact'
                            : 'comfortable',
                })),
            toggleSidebar: () =>
                set(state => ({ sidebarPinned: !state.sidebarPinned })),
            toggleCommandHints: () =>
                set(state => ({ commandHints: !state.commandHints })),
            saveSettings: () =>
                set({
                    lastSavedAt: new Date().toISOString(),
                }),
        })),
        {
            name: 'dx-dashboard-settings',
            version: 1,
            partialize: pickDashboardSettings,
        },
    ),
) as PersistStoreApi<DashboardSettingsState>;

function pickDashboardSettings(
    state: DashboardSettingsState,
): DashboardSettingsSnapshot {
    return {
        density: state.density,
        sidebarPinned: state.sidebarPinned,
        commandHints: state.commandHints,
        lastSavedAt: state.lastSavedAt,
    };
}

export function dashboardSettingsSnapshot(
    store: StoreApi<DashboardSettingsState> = dashboardSettingsStore,
): DashboardSettingsSnapshot {
    return pickDashboardSettings(store.getState());
}

export function subscribeToDashboardSettings(
    listener: (
        snapshot: DashboardSettingsSnapshot,
        previousSnapshot: DashboardSettingsSnapshot,
    ) => void,
) {
    return dashboardSettingsStore.subscribe(
        pickDashboardSettings,
        listener,
        { equalityFn: shallow, fireImmediately: true },
    );
}

export function rehydrateDashboardSettings() {
    dashboardSettingsStore.persist.rehydrate();
}

export function clearSavedDashboardSettings() {
    dashboardSettingsStore.persist.clearStorage();
}
