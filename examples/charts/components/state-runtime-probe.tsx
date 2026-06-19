"use client";

import {
  action,
  counterStore,
  derived,
  effect,
  state,
  store,
} from "@/lib/stores/counter";

type StateRuntimeProbeStoreDraft = {
  count: number;
  enabled: boolean;
  label: string;
};

const stateRuntimeProbeStore = store({
  count: state(1),
  enabled: state(false),
  label: state("State runtime"),
  doubled: derived((draft: StateRuntimeProbeStoreDraft) => draft.count * 2),
  nextCount: derived((draft: StateRuntimeProbeStoreDraft) => draft.count + 1),
  increment: action((draft: StateRuntimeProbeStoreDraft) => {
    draft.count += 1;
  }),
  toggle: action((draft: StateRuntimeProbeStoreDraft) => {
    draft.enabled = !draft.enabled;
  }),
  markPointer: action((draft: StateRuntimeProbeStoreDraft) => {
    draft.count += 1;
  }),
  markReady: action((draft: StateRuntimeProbeStoreDraft) => {
    draft.label = "State runtime ready";
  }),
  announce: effect((draft: StateRuntimeProbeStoreDraft) => draft.label),
});

export function StateRuntimeProbe() {
  const count = stateRuntimeProbeStore.count;
  const enabled = stateRuntimeProbeStore.enabled;
  const label = stateRuntimeProbeStore.label;
  const doubled = stateRuntimeProbeStore.doubled;
  const nextCount = stateRuntimeProbeStore.nextCount;

  return (
    <section
      className="starter-card"
      data-dx-component="state-runtime-probe"
      data-dx-source-owned-probe="state-runtime"
      data-dx-state-runtime-count={String(count)}
      data-dx-state-runtime-enabled={enabled ? "true" : "false"}
      aria-labelledby="state-runtime-title"
    >
      <p className="starter-kicker">Runtime proof</p>
      <h1 id="state-runtime-title">State Runtime Probe</h1>
      <p className="starter-copy" data-dx-state-read="label">
        {label}
      </p>
      <p data-dx-state-read="count">{count}</p>
      <p data-dx-state-read="enabled">{enabled ? "enabled" : "disabled"}</p>
      <p data-dx-state-read="doubled">{doubled}</p>
      <p data-dx-state-read="nextCount">{nextCount}</p>
      <p data-dx-state-read="counterStore.count">{counterStore.count}</p>
      <p data-dx-state-read="counterStore.label">{counterStore.label}</p>
      <div className="starter-actions" aria-label="State runtime actions">
        <button
          className="starter-action-button"
          type="button"
          onClick={() => stateRuntimeProbeStore.increment(stateRuntimeProbeStore)}
          motion="button(transition-transform duration-200) animation-pop(opacity:0..1,scale:0.96..1)"
        >
          Increment
        </button>
        <button
          className="starter-action-button"
          type="button"
          onClick={() => stateRuntimeProbeStore.toggle(stateRuntimeProbeStore)}
        >
          Toggle
        </button>
        <button
          className="starter-action-button"
          type="button"
          onPointerMove={() => stateRuntimeProbeStore.markPointer(stateRuntimeProbeStore)}
        >
          Track pointer
        </button>
        <button className="starter-action-button" type="button" onClick="bg-accent">
          Save
        </button>
        <button
          className="starter-action-button"
          type="button"
          onClick={() => counterStore.increment(counterStore)}
        >
          Global increment
        </button>
      </div>
      <input
        className="starter-input"
        aria-label="Probe label"
        value={label}
        onInput={() => stateRuntimeProbeStore.markReady(stateRuntimeProbeStore)}
      />
    </section>
  );
}
