"use client";

import { action, derived, effect, state, store } from "@/lib/stores/counter";

type IslandRuntimeProbeProps = {
  label?: string;
  clientLoad?: boolean;
  clientVisible?: boolean | { rootMargin?: string };
  clientIdle?: boolean | { timeout?: number };
  clientOnly?: string;
};

type IslandRuntimeProbeStoreDraft = {
  count: number;
  message: string;
};

const islandRuntimeProbeStore = store({
  count: state(0),
  message: state("Ready"),
  nextCount: derived((draft: IslandRuntimeProbeStoreDraft) => draft.count + 1),
  increment: action((draft: IslandRuntimeProbeStoreDraft) => {
    draft.count += 1;
  }),
  activatePointer: action((draft: IslandRuntimeProbeStoreDraft) => {
    draft.message = "Pointer active";
  }),
  markEdited: action((draft: IslandRuntimeProbeStoreDraft) => {
    draft.message = "Edited";
  }),
  announce: effect((draft: IslandRuntimeProbeStoreDraft) => draft.message),
});

export function IslandRuntimeProbe({
  label = "Island Runtime",
}: IslandRuntimeProbeProps) {
  const count = islandRuntimeProbeStore.count;
  const message = islandRuntimeProbeStore.message;

  return (
    <section
      className="starter-card"
      data-dx-component="island-runtime-probe"
      data-dx-source-owned-probe="island-runtime"
      data-dx-client-directives="clientLoad clientVisible clientIdle clientOnly"
      aria-labelledby="island-runtime-title"
    >
      <p className="starter-kicker">Island runtime</p>
      <h1 id="island-runtime-title">{label}</h1>
      <p className="starter-copy" data-dx-state-read="message">
        {message}
      </p>
      <p data-dx-state-read="count">{count}</p>
      <div className="starter-actions" aria-label="Island runtime actions">
        <button
          className="starter-action-button"
          type="button"
          onClick={() => islandRuntimeProbeStore.increment(islandRuntimeProbeStore)}
        >
          Increment
        </button>
        <button
          className="starter-action-button"
          type="button"
          onPointerMove={() => islandRuntimeProbeStore.activatePointer(islandRuntimeProbeStore)}
        >
          Pointer
        </button>
      </div>
      <input
        className="starter-input"
        aria-label="Island message"
        value={message}
        onInput={() => islandRuntimeProbeStore.markEdited(islandRuntimeProbeStore)}
      />
    </section>
  );
}
