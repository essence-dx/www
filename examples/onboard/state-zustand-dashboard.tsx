"use client";

import * as React from "react";

import { Button } from "@/components/ui/button";
import {
  createWithEqualityFn,
  persist,
  shallow,
} from "@/lib/forge/state/zustand";

type LaunchDashboardDensity = "comfortable" | "compact";
type LaunchDashboardFocus = "session" | "payment" | "data" | "studio";
type LaunchDashboardRehydrateState = "idle" | "loading";

type LaunchDashboardSettingsState = {
  commandHints: boolean;
  density: LaunchDashboardDensity;
  focus: LaunchDashboardFocus;
  rehydrateState: LaunchDashboardRehydrateState;
  savedAt: string;
  reset: () => void;
  save: () => void;
  setDensity: (density: LaunchDashboardDensity) => void;
  setFocus: (focus: LaunchDashboardFocus) => void;
  setRehydrateState: (state: LaunchDashboardRehydrateState) => void;
  toggleCommandHints: () => void;
};

const defaultSettings = {
  commandHints: true,
  density: "comfortable" as LaunchDashboardDensity,
  focus: "payment" as LaunchDashboardFocus,
  rehydrateState: "idle" as LaunchDashboardRehydrateState,
  savedAt: "not-saved",
};

export const useLaunchDashboardSettings =
  createWithEqualityFn<LaunchDashboardSettingsState>()(
    persist(
      (set) => ({
        ...defaultSettings,
        reset: () => set(defaultSettings),
        save: () => set({ savedAt: new Date().toISOString() }),
        setDensity: (density) => set({ density }),
        setFocus: (focus) => set({ focus }),
        setRehydrateState: (rehydrateState) => set({ rehydrateState }),
        toggleCommandHints: () =>
          set((state) => ({ commandHints: !state.commandHints })),
      }),
      {
        name: "dx-template-dashboard-settings",
        partialize: ({ commandHints, density, focus, savedAt }) => ({
          commandHints,
          density,
          focus,
          savedAt,
        }),
      },
    ),
    shallow,
  );

export function LaunchDashboardStateControl() {
  const [hydrated, setHydrated] = React.useState(() =>
    useLaunchDashboardSettings.persist.hasHydrated(),
  );
  const {
    commandHints,
    density,
    focus,
    rehydrateState,
    reset,
    save,
    savedAt,
    setDensity,
    setFocus,
    setRehydrateState,
    toggleCommandHints,
  } = useLaunchDashboardSettings(
    (state) => ({
      commandHints: state.commandHints,
      density: state.density,
      focus: state.focus,
      rehydrateState: state.rehydrateState,
      reset: state.reset,
      save: state.save,
      savedAt: state.savedAt,
      setDensity: state.setDensity,
      setFocus: state.setFocus,
      setRehydrateState: state.setRehydrateState,
      toggleCommandHints: state.toggleCommandHints,
    }),
    shallow,
  );

  React.useEffect(() => {
    const unsubscribeHydrate = useLaunchDashboardSettings.persist.onHydrate(
      () => setHydrated(false),
    );
    const unsubscribeFinish =
      useLaunchDashboardSettings.persist.onFinishHydration(() =>
        setHydrated(true),
      );

    return () => {
      unsubscribeHydrate();
      unsubscribeFinish();
    };
  }, []);

  function rehydrateDashboardSettings() {
    if (rehydrateState === "loading") return;

    setHydrated(false);
    setRehydrateState("loading");
    void useLaunchDashboardSettings.persist.rehydrate().finally(() => {
      setHydrated(useLaunchDashboardSettings.persist.hasHydrated());
      setRehydrateState("idle");
    });
  }

  return (
    <section
      className="grid gap-4 rounded-md border border-border bg-card p-4 text-card-foreground"
      data-dx-package="state/zustand"
      data-dx-component="launch-dashboard-state-workflow"
      data-dx-dashboard-workflow="ui-state-persistence"
      data-dx-style-surface="state-management"
      data-dx-zustand-store="launch-dashboard-settings"
      data-dx-zustand-persist-key="dx-template-dashboard-settings"
      data-dx-zustand-dashboard-density={density}
      data-dx-zustand-dashboard-focus={focus}
      data-dx-zustand-command-hints={commandHints ? "enabled" : "disabled"}
      data-dx-zustand-hydration={hydrated ? "ready" : "loading"}
      data-dx-zustand-hydration-event={hydrated ? "onFinishHydration" : "onHydrate"}
      data-dx-zustand-rehydrate-state={rehydrateState}
      data-dx-node-modules="forbidden"
      data-dx-icon-search="state:zustand"
    >
      <div className="grid gap-1">
        <p className="text-sm font-medium">
          <dx-icon name="pack:state" aria-hidden="true" /> Dashboard view state
        </p>
        <p className="text-xs leading-5 text-muted-foreground">
          Persist density, focused workflow, and command hints for the launch
          dashboard without a template-local node_modules path.
        </p>
      </div>
      <div className="grid gap-2 text-sm text-muted-foreground sm:grid-cols-3">
        <span>Density: {density}</span>
        <span>Focus: {focus}</span>
        <span>Hints: {commandHints ? "on" : "off"}</span>
      </div>
      <div className="flex flex-wrap gap-2">
        <Button
          type="button"
          size="sm"
          variant={density === "comfortable" ? "secondary" : "outline"}
          data-dx-zustand-action="set-dashboard-density"
          data-dx-zustand-dashboard-density-option="comfortable"
          onClick={() => setDensity("comfortable")}
        >
          Comfortable
        </Button>
        <Button
          type="button"
          size="sm"
          variant={density === "compact" ? "secondary" : "outline"}
          data-dx-zustand-action="set-dashboard-density"
          data-dx-zustand-dashboard-density-option="compact"
          onClick={() => setDensity("compact")}
        >
          Compact
        </Button>
        {(["session", "payment", "data", "studio"] as const).map((option) => (
          <Button
            key={option}
            type="button"
            size="sm"
            variant={focus === option ? "secondary" : "outline"}
            data-dx-zustand-action="select-dashboard-focus"
            data-dx-zustand-dashboard-focus-option={option}
            onClick={() => setFocus(option)}
          >
            {option}
          </Button>
        ))}
        <Button
          type="button"
          size="sm"
          variant="outline"
          data-dx-zustand-action="toggle-command-hints"
          onClick={toggleCommandHints}
        >
          Toggle hints
        </Button>
        <Button
          type="button"
          size="sm"
          data-dx-zustand-action="save-dashboard-settings"
          onClick={save}
        >
          Save view
        </Button>
        <Button
          type="button"
          size="sm"
          variant="outline"
          data-dx-zustand-action="reset-dashboard-settings"
          onClick={reset}
        >
          Reset
        </Button>
        <Button
          type="button"
          size="sm"
          variant="outline"
          data-dx-zustand-action="rehydrate-dashboard-settings"
          data-dx-zustand-rehydrate-state={rehydrateState}
          disabled={rehydrateState === "loading"}
          onClick={rehydrateDashboardSettings}
        >
          {rehydrateState === "loading" ? "Rehydrating" : "Rehydrate"}
        </Button>
      </div>
      <p className="text-xs text-muted-foreground">
        Persisted: {savedAt}; hydration {hydrated ? "ready" : "loading"}
      </p>
    </section>
  );
}
