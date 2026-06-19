"use client";

import * as React from "react";

import { Button } from "@/components/ui/button";
import {
  combine,
  createStore,
  createWithEqualityFn,
  devtools,
  persist,
  redux,
  shallow,
  subscribeWithSelector,
  unstable_ssrSafe,
  useStore,
  type Mutate,
  type ReduxState,
  type StoreApi,
} from "@/lib/forge/state/zustand";
import { immer } from "@/lib/forge/state/zustand/immer";

type LaunchCounterModel = {
  count: number;
  reviewMode: boolean;
};

type LaunchCounterAction =
  | { type: "increment" }
  | { type: "reset" }
  | { type: "toggleReviewMode" };
type LaunchCounterStore = LaunchCounterModel & ReduxState<LaunchCounterAction>;
type LaunchCounterAuditAction = LaunchCounterAction["type"] | "hydrated";
type LaunchCounterAudit = {
  lastAction: LaunchCounterAuditAction;
  updates: number;
  noteAction: (action: LaunchCounterAuditAction) => void;
};
type LaunchCounterAuditApi = Mutate<
  StoreApi<LaunchCounterAudit>,
  [["zustand/subscribeWithSelector", never], ["zustand/immer", never]]
>;

function launchCounterReducer(
  state: LaunchCounterModel,
  action: LaunchCounterAction,
) {
  switch (action.type) {
    case "increment":
      return { ...state, count: state.count + 1 };
    case "reset":
      return { count: 0, reviewMode: false };
    case "toggleReviewMode":
      return { ...state, reviewMode: !state.reviewMode };
  }
}

export const useLaunchCounter = createWithEqualityFn<LaunchCounterStore>()(
  devtools(
    unstable_ssrSafe(
      persist(
        redux(launchCounterReducer, {
          count: 0,
          reviewMode: false,
        }),
        {
          name: "dx-launch-counter",
          partialize: (state) => ({
            count: state.count,
            reviewMode: state.reviewMode,
          }),
        },
      ),
    ),
    { name: "DX Launch Counter" },
  ),
  shallow,
);

const launchCounterAuditStore = createStore<LaunchCounterAudit>()(
  subscribeWithSelector(
    immer(
      combine(
        {
          lastAction: "hydrated" as LaunchCounterAuditAction,
          updates: 0,
        },
        (set) => ({
          noteAction: (action) =>
            set((state) => {
              state.lastAction = action;
              state.updates += 1;
            }),
        }),
      ),
    ),
  ),
);
const launchCounterAudit = launchCounterAuditStore as LaunchCounterAuditApi;

export function LaunchCounterControl() {
  const [hasHydrated, setHasHydrated] = React.useState(() =>
    useLaunchCounter.persist.hasHydrated(),
  );
  const audit = useStore(launchCounterAudit, (state) => ({
    lastAction: state.lastAction,
    updates: state.updates,
  }));
  const { count, reviewMode, dispatch } = useLaunchCounter(
    (state) => ({
      count: state.count,
      reviewMode: state.reviewMode,
      dispatch: state.dispatch,
    }),
    shallow,
  );

  React.useEffect(() => {
    const unsubscribeHydrate = useLaunchCounter.persist.onHydrate(() => {
      setHasHydrated(false);
    });
    const unsubscribeFinish = useLaunchCounter.persist.onFinishHydration(
      () => {
        setHasHydrated(true);
        launchCounterAudit.getState().noteAction("hydrated");
      },
    );
    const unsubscribeAuditAction = launchCounterAudit.subscribe(
      (state) => state.lastAction,
      () => {},
    );

    return () => {
      unsubscribeHydrate();
      unsubscribeFinish();
      unsubscribeAuditAction();
    };
  }, []);

  return (
    <div
      className="grid gap-3"
      data-dx-package="state/zustand"
      data-dx-component="zustand-state-card"
      data-dx-zustand-store="launch-counter"
      data-dx-zustand-count={String(count)}
      data-dx-zustand-toggle-state={reviewMode ? "enabled" : "disabled"}
      data-dx-zustand-persist-key="dx-launch-counter"
      data-dx-zustand-hydration={hasHydrated ? "ready" : "loading"}
      data-dx-node-modules="forbidden"
    >
      <div className="grid gap-2 text-sm text-muted-foreground sm:grid-cols-3">
        <span>Launch state: {count}</span>
        <span>Review mode: {reviewMode ? "on" : "off"}</span>
        <span
          className="text-xs"
          data-dx-zustand-vanilla-store={`${audit.lastAction}:${audit.updates}`}
        >
          Last action: {audit.lastAction}
        </span>
      </div>
      <div className="flex flex-wrap gap-2">
        <Button
          type="button"
          size="sm"
          data-dx-zustand-action="increment"
          onClick={() => {
            dispatch({ type: "increment" });
            launchCounterAudit.getState().noteAction("increment");
          }}
        >
          Increment
        </Button>
        <Button
          type="button"
          variant="outline"
          size="sm"
          data-dx-zustand-action="toggle-review-mode"
          onClick={() => {
            dispatch({ type: "toggleReviewMode" });
            launchCounterAudit.getState().noteAction("toggleReviewMode");
          }}
        >
          Toggle mode
        </Button>
        <Button
          type="button"
          variant="outline"
          size="sm"
          data-dx-zustand-action="reset"
          onClick={() => {
            dispatch({ type: "reset" });
            launchCounterAudit.getState().noteAction("reset");
          }}
        >
          Reset
        </Button>
        <Button
          type="button"
          variant="outline"
          size="sm"
          data-dx-zustand-action="rehydrate"
          onClick={() => {
            void useLaunchCounter.persist.rehydrate();
          }}
        >
          Sync
        </Button>
      </div>
      <p className="text-xs text-muted-foreground">
        Persisted: {hasHydrated ? "ready" : "loading"}
      </p>
    </div>
  );
}
