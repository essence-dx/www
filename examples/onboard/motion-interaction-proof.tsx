"use client";

import * as React from "react";

import { Badge } from "@/components/ui/badge";
import { MotionControlledStatus } from "@/motion/controls";
import {
  DxMotionLayoutGroup,
  MotionLayoutItem,
  dxMotionLayoutId,
  useDxInstantLayoutTransition,
} from "@/motion/layout";
import { MotionLazyBox } from "@/motion/lazy";
import { MotionValueMeter } from "@/motion/motion-values";
import { MotionReveal } from "@/motion/reveal";
import { useDxMotionPressFeedback } from "@/motion/scoped-animate";

const motionStages = [
  {
    id: "source-owned",
    label: "Source-owned adapters",
    description:
      "The launch route imports Forge-owned Motion helpers instead of editing package internals.",
    value: 34,
  },
  {
    id: "interactive",
    label: "Local interaction",
    description:
      "Controls update React state, animate the active panel, and reorder layout items in-place.",
    value: 67,
  },
  {
    id: "preview-ready",
    label: "Web Preview markers",
    description:
      "Stable data-dx markers let Zed target the visible Motion surface from source.",
    value: 100,
  },
] as const;

const initialStageIds = motionStages.map((stage) => stage.id);
const motionPreferenceStorageKey = "dx.launch.motion.dashboard";

type MotionStageMoveDirection = "previous" | "next";
type MotionStagePlacement = MotionStageMoveDirection | "first" | "last";
type MotionPreference = {
  motionOrder?: readonly string[];
  motionReduced?: boolean;
};

function normalizeMotionOrder(stageIds: unknown) {
  if (!Array.isArray(stageIds)) return null;

  const nextStageIds = stageIds.filter(
    (stageId): stageId is string =>
      typeof stageId === "string" &&
      motionStages.some((stage) => stage.id === stageId),
  );
  const uniqueStageIds = [...new Set(nextStageIds)];

  if (uniqueStageIds.length !== motionStages.length) return null;
  return uniqueStageIds;
}

function readMotionPreference(): Required<MotionPreference> | null {
  if (typeof window === "undefined") return null;

  try {
    const rawPreference = window.localStorage.getItem(motionPreferenceStorageKey);
    if (!rawPreference) return null;

    const preference = JSON.parse(rawPreference) as MotionPreference;
    const motionOrder = normalizeMotionOrder(preference.motionOrder);
    if (!motionOrder) return null;

    return {
      motionOrder,
      motionReduced: Boolean(preference.motionReduced),
    };
  } catch {
    return null;
  }
}

function writeMotionPreference(
  motionOrder: readonly string[],
  motionReduced: boolean,
) {
  if (typeof window === "undefined") return;

  try {
    window.localStorage.setItem(
      motionPreferenceStorageKey,
      JSON.stringify({
        motionOrder: [...motionOrder],
        motionReduced,
      }),
    );
  } catch {
    // Embedded previews can block storage; the visible Motion & Animation proof still works.
  }
}

function orderedMotionStages(stageIds: readonly string[]) {
  return stageIds
    .map((stageId) => motionStages.find((stage) => stage.id === stageId))
    .filter((stage): stage is (typeof motionStages)[number] => Boolean(stage));
}

function moveMotionStageId(
  stageIds: readonly string[],
  stageId: string,
  direction: MotionStageMoveDirection,
) {
  return placeMotionStageId(stageIds, stageId, direction);
}

function placeMotionStageId(
  stageIds: readonly string[],
  stageId: string,
  target: MotionStagePlacement,
) {
  const currentIndex = stageIds.indexOf(stageId);
  if (currentIndex < 0) return [...stageIds];

  const nextIndex = {
    first: 0,
    last: stageIds.length - 1,
    previous: Math.max(0, currentIndex - 1),
    next: Math.min(stageIds.length - 1, currentIndex + 1),
  }[target];

  if (nextIndex === currentIndex) return [...stageIds];

  const nextStageIds = [...stageIds];
  const [stage] = nextStageIds.splice(currentIndex, 1);
  nextStageIds.splice(nextIndex, 0, stage);
  return nextStageIds;
}

export function LaunchMotionInteractionProof() {
  const [stageIndex, setStageIndex] = React.useState(0);
  const [orderedStageIds, setOrderedStageIds] =
    React.useState<readonly string[]>(
      () => readMotionPreference()?.motionOrder ?? initialStageIds,
    );
  const [reducedMotionPreview, setReducedMotionPreview] = React.useState(
    () => readMotionPreference()?.motionReduced ?? false,
  );
  const startInstantLayoutTransition = useDxInstantLayoutTransition();
  const motionPress = useDxMotionPressFeedback({ scale: 1.02, y: -1 });

  const activeStage = motionStages[stageIndex] ?? motionStages[0];
  const progress = activeStage.value;
  const reducedMotionState = reducedMotionPreview ? "preview" : "system";
  const motionPolicyLabel = reducedMotionPreview
    ? "Reduced motion preview"
    : "System motion setting";
  const orderedStages = React.useMemo(
    () => orderedMotionStages(orderedStageIds),
    [orderedStageIds],
  );
  const activeOrderIndex = orderedStageIds.indexOf(activeStage.id);
  const canMovePrevious = activeOrderIndex > 0;
  const canMoveNext = activeOrderIndex >= 0 && activeOrderIndex < orderedStageIds.length - 1;

  React.useEffect(() => {
    writeMotionPreference(orderedStageIds, reducedMotionPreview);
  }, [orderedStageIds, reducedMotionPreview]);

  const advanceStage = React.useCallback(() => {
    setStageIndex((current) => (current + 1) % motionStages.length);
  }, []);

  const reverseOrder = React.useCallback(() => {
    const reorder = () => {
      setOrderedStageIds((current) => [...current].reverse());
    };

    if (reducedMotionPreview) {
      reorder();
      return;
    }

    startInstantLayoutTransition(reorder);
  }, [reducedMotionPreview, startInstantLayoutTransition]);

  const moveStage = React.useCallback(
    (direction: MotionStageMoveDirection) => {
      const reorder = () => {
        setOrderedStageIds((current) =>
          moveMotionStageId(current, activeStage.id, direction),
        );
      };

      if (reducedMotionPreview) {
        reorder();
        return;
      }

      startInstantLayoutTransition(reorder);
    },
    [activeStage.id, reducedMotionPreview, startInstantLayoutTransition],
  );

  const placeStage = React.useCallback(
    (target: MotionStagePlacement) => {
      const reorder = () => {
        setOrderedStageIds((current) =>
          placeMotionStageId(current, activeStage.id, target),
        );
      };

      if (reducedMotionPreview) {
        reorder();
        return;
      }

      startInstantLayoutTransition(reorder);
    },
    [activeStage.id, reducedMotionPreview, startInstantLayoutTransition],
  );

  const handleKeyboardReorder = React.useCallback(
    (event: React.KeyboardEvent<HTMLDivElement>) => {
      const target = {
        ArrowLeft: "previous",
        ArrowUp: "previous",
        ArrowRight: "next",
        ArrowDown: "next",
        Home: "first",
        End: "last",
      }[event.key] as MotionStagePlacement | undefined;

      if (!target) return;
      event.preventDefault();
      placeStage(target);
    },
    [placeStage],
  );

  const resetProof = React.useCallback(() => {
    const reset = () => {
      setStageIndex(0);
      setOrderedStageIds(initialStageIds);
      setReducedMotionPreview(false);
    };

    if (reducedMotionPreview) {
      reset();
      return;
    }

    startInstantLayoutTransition(reset);
  }, [reducedMotionPreview, startInstantLayoutTransition]);

  return (
    <section
      ref={motionPress.scope}
      className="grid gap-4"
      data-dx-package="animation/motion"
      data-dx-component="motion-interaction-proof"
      data-dx-motion-interaction="local-state-animation"
      data-dx-motion-policy="app-owned-reduced-motion-preview"
      data-dx-motion-reduced={reducedMotionPreview ? "preview" : "system"}
      data-dx-motion-state={activeStage.id}
      data-dx-motion-progress={progress}
      data-dx-motion-order={orderedStageIds.join(",")}
      data-dx-motion-preference-storage="local-storage"
      data-dx-motion-storage-key={motionPreferenceStorageKey}
      data-dx-node-modules="forbidden"
    >
      <MotionControlledStatus
        active
        className="grid gap-3 rounded-md border border-border bg-muted/30 p-4"
        data-dx-motion="motion-proof-controlled-status"
        label="Motion & Animation launch proof state"
        status={progress >= 100 ? "complete" : "active"}
      >
        <MotionLazyBox
          key={activeStage.id}
          animate={{ opacity: 1, y: 0 }}
          className="grid gap-2"
          data-dx-motion="motion-proof-reveal-panel"
          featureBoundary="animation"
          initial={reducedMotionPreview ? { opacity: 1, y: 0 } : { opacity: 0, y: 8 }}
          transition={reducedMotionPreview ? { duration: 0 } : { duration: 0.22 }}
        >
          <div className="flex flex-wrap items-center justify-between gap-2">
            <div>
              <p className="text-xs font-medium uppercase tracking-normal text-muted-foreground">
                Motion & Animation interaction proof
              </p>
              <h3 className="text-base font-semibold text-foreground">
                {activeStage.label}
              </h3>
            </div>
            <Badge variant="outline">{progress}%</Badge>
          </div>
          <p className="text-sm leading-6 text-muted-foreground">
            {activeStage.description}
          </p>
          <p
            className="text-xs leading-5 text-muted-foreground"
            data-dx-motion-policy-status={reducedMotionState}
          >
            {motionPolicyLabel}: dashboard choreography remains selectable while
            app-owned route animation policy stays explicit.
          </p>
          <MotionValueMeter
            data-dx-motion="motion-proof-progress"
            indicatorClassName="h-full rounded-full bg-primary"
            label="Motion & Animation proof progress"
            max={100}
            trackClassName="h-1.5 overflow-hidden rounded-full bg-muted"
            value={progress}
          />
        </MotionLazyBox>
      </MotionControlledStatus>

      <div
        className="flex flex-wrap gap-2"
        data-dx-motion-interaction="control-row"
      >
        <button
          type="button"
          className="rounded-md border border-border px-3 py-2 text-sm text-foreground hover:border-primary hover:bg-accent hover:text-primary"
          data-dx-motion-interaction="advance-stage"
          onClick={advanceStage}
          onMouseEnter={(event) => motionPress.press(event.currentTarget)}
        >
          Advance stage
        </button>
        <button
          type="button"
          className="rounded-md border border-border px-3 py-2 text-sm text-foreground hover:border-primary hover:bg-accent hover:text-primary"
          data-dx-motion-interaction="reverse-order"
          onClick={reverseOrder}
          onMouseEnter={(event) => motionPress.press(event.currentTarget)}
        >
          Reverse order
        </button>
        <button
          type="button"
          className="rounded-md border border-border px-3 py-2 text-sm text-foreground hover:border-primary hover:bg-accent hover:text-primary disabled:cursor-not-allowed disabled:opacity-60"
          data-dx-motion-interaction="move-stage-previous"
          data-dx-motion-order-available={canMovePrevious ? "true" : "false"}
          data-dx-motion-order-direction="previous"
          disabled={!canMovePrevious}
          onClick={() => moveStage("previous")}
          onMouseEnter={(event) => motionPress.press(event.currentTarget)}
        >
          Move earlier
        </button>
        <button
          type="button"
          className="rounded-md border border-border px-3 py-2 text-sm text-foreground hover:border-primary hover:bg-accent hover:text-primary disabled:cursor-not-allowed disabled:opacity-60"
          data-dx-motion-interaction="move-stage-next"
          data-dx-motion-order-available={canMoveNext ? "true" : "false"}
          data-dx-motion-order-direction="next"
          disabled={!canMoveNext}
          onClick={() => moveStage("next")}
          onMouseEnter={(event) => motionPress.press(event.currentTarget)}
        >
          Move later
        </button>
        <button
          type="button"
          className="rounded-md border border-border px-3 py-2 text-sm text-foreground hover:border-primary hover:bg-accent hover:text-primary"
          data-dx-motion-interaction="reset-proof"
          onClick={resetProof}
          onMouseEnter={(event) => motionPress.press(event.currentTarget)}
        >
          Reset
        </button>
        <button
          type="button"
          aria-pressed={reducedMotionPreview}
          className="rounded-md border border-border px-3 py-2 text-sm text-foreground hover:border-primary hover:bg-accent hover:text-primary"
          data-dx-motion-interaction="toggle-reduced-motion"
          data-dx-motion-reduced={reducedMotionState}
          onClick={() => setReducedMotionPreview((current) => !current)}
          onMouseEnter={(event) => motionPress.press(event.currentTarget)}
        >
          {reducedMotionPreview ? "Use system motion" : "Preview reduced motion"}
        </button>
      </div>

      <DxMotionLayoutGroup id="launch-motion-proof-order">
        <div
          className="grid gap-2 sm:grid-cols-3"
          aria-activedescendant={`motion-stage-${activeStage.id}`}
          aria-label="Motion & Animation launch stage order"
          data-dx-motion-interaction="layout-reorder-list"
          data-dx-motion-keyboard-reorder="arrow-home-end"
          data-dx-motion-keyboard-state={activeStage.id}
          onKeyDown={handleKeyboardReorder}
          role="listbox"
          tabIndex={0}
        >
          {orderedStages.map((stage) => (
            <MotionLayoutItem
              key={stage.id}
              id={`motion-stage-${stage.id}`}
              aria-selected={stage.id === activeStage.id ? "true" : "false"}
              className="rounded-md border border-border bg-muted/30 p-3"
              data-dx-motion="motion-proof-layout-item"
              data-dx-motion-stage={stage.id}
              layoutId={dxMotionLayoutId("launch-motion-proof", stage.id)}
              role="option"
            >
              <MotionReveal preset="scale-in">
                <p className="text-sm font-medium text-foreground">
                  {stage.label}
                </p>
                <p className="mt-1 text-xs leading-5 text-muted-foreground">
                  {stage.value}% of launch path
                </p>
              </MotionReveal>
            </MotionLayoutItem>
          ))}
        </div>
      </DxMotionLayoutGroup>
    </section>
  );
}
