pub(super) const MOTION_VERSION: &str = "12.38.0-dx.12";

pub(super) fn motion_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        ("js/motion/presets.ts", MOTION_PRESETS_TS),
        ("js/motion/controls.tsx", MOTION_CONTROLS_TSX),
        ("js/motion/frame.tsx", MOTION_FRAME_TSX),
        ("js/motion/layout.tsx", MOTION_LAYOUT_TSX),
        ("js/motion/lazy.tsx", MOTION_LAZY_TSX),
        ("js/motion/motion-values.tsx", MOTION_VALUES_TSX),
        ("js/motion/page-visibility.tsx", MOTION_PAGE_VISIBILITY_TSX),
        ("js/motion/provider.tsx", MOTION_PROVIDER_TSX),
        ("js/motion/presence.tsx", MOTION_PRESENCE_TSX),
        ("js/motion/reorder.tsx", MOTION_REORDER_TSX),
        ("js/motion/reveal.tsx", MOTION_REVEAL_TSX),
        ("js/motion/scoped-animate.tsx", MOTION_SCOPED_ANIMATE_TSX),
        ("js/motion/scroll-progress.tsx", MOTION_SCROLL_PROGRESS_TSX),
        ("js/motion/will-change.tsx", MOTION_WILL_CHANGE_TSX),
        (
            "js/motion/dashboard-workflow.ts",
            MOTION_DASHBOARD_WORKFLOW_TS,
        ),
        ("js/motion/metadata.ts", MOTION_METADATA_TS),
        ("js/motion/README.md", MOTION_README_MD),
    ]
}

const MOTION_PRESETS_TS: &str = r#"import type { Transition, Variants } from "motion/react";

export type DxMotionPresetName = "fade-up" | "scale-in" | "slide-left";

export const dxMotionTransition: Transition = {
  duration: 0.42,
  ease: [0.22, 1, 0.36, 1],
};

export const dxMotionViewport = {
  once: true,
  amount: 0.3,
  margin: "0px 0px -10% 0px",
} as const;

export const dxMotionStagger = {
  delayChildren: 0.05,
  staggerChildren: 0.07,
} as const;

export const dxMotionVariants = {
  "fade-up": {
    hidden: { opacity: 0, y: 18 },
    visible: { opacity: 1, y: 0 },
  },
  "scale-in": {
    hidden: { opacity: 0, scale: 0.96 },
    visible: { opacity: 1, scale: 1 },
  },
  "slide-left": {
    hidden: { opacity: 0, x: 18 },
    visible: { opacity: 1, x: 0 },
  },
} satisfies Record<DxMotionPresetName, Variants>;

export function dxMotionPreset(name: DxMotionPresetName = "fade-up"): Variants {
  return dxMotionVariants[name];
}

export function dxMotionStaggerContainer(): Variants {
  return {
    hidden: {},
    visible: {
      transition: dxMotionStagger,
    },
  };
}
"#;

const MOTION_CONTROLS_TSX: &str = r#""use client";

import * as React from "react";
import {
  animationControls,
  motion,
  useAnimation,
  useAnimationControls,
  useReducedMotion,
  type AnimationDefinition,
  type HTMLMotionProps,
  type LegacyAnimationControls,
  type Transition,
} from "motion/react";

import { dxMotionTransition } from "./presets";

export type DxAnimationControlState = "idle" | "ready" | "active" | "complete";

export const dxAnimationControlTargets = {
  idle: { opacity: 0.72, y: 0, scale: 1 },
  ready: { opacity: 1, y: 0, scale: 1 },
  active: { opacity: 1, y: -2, scale: 1.01 },
  complete: { opacity: 1, y: 0, scale: 1 },
} satisfies Record<DxAnimationControlState, AnimationDefinition>;

export const useDxRawAnimationControls = useAnimationControls;
export const useDxLegacyAnimation = useAnimation;

export type DxAnimationControlsOptions = {
  disabled?: boolean;
  initial?: DxAnimationControlState;
  onComplete?: (state: DxAnimationControlState) => void;
  transition?: Transition;
};

export type DxAnimationControlsState = {
  controls: LegacyAnimationControls;
  prefersReducedMotion: boolean;
  reset: () => void;
  set: (state: DxAnimationControlState) => void;
  start: (state?: DxAnimationControlState) => Promise<void>;
  state: DxAnimationControlState;
  stop: () => void;
};

export function useDxAnimationControls({
  disabled = false,
  initial = "ready",
  onComplete,
  transition = dxMotionTransition,
}: DxAnimationControlsOptions = {}): DxAnimationControlsState {
  const controls = useAnimationControls();
  const prefersReducedMotion = useReducedMotion();
  const [state, setState] = React.useState<DxAnimationControlState>(initial);

  React.useEffect(() => {
    controls.set(dxAnimationControlTargets[initial]);
    setState(initial);
  }, [controls, initial]);

  const set = React.useCallback(
    (nextState: DxAnimationControlState) => {
      setState(nextState);
      controls.set(dxAnimationControlTargets[nextState]);
    },
    [controls],
  );

  const start = React.useCallback(
    async (nextState: DxAnimationControlState = "active") => {
      setState(nextState);
      const target = dxAnimationControlTargets[nextState];

      if (disabled || prefersReducedMotion) {
        controls.set(target);
        onComplete?.(nextState);
        return;
      }

      await controls.start(target, transition);
      onComplete?.(nextState);
    },
    [controls, disabled, onComplete, prefersReducedMotion, transition],
  );

  const reset = React.useCallback(() => {
    set(initial);
  }, [initial, set]);

  const stop = React.useCallback(() => {
    controls.stop();
  }, [controls]);

  return {
    controls,
    prefersReducedMotion: Boolean(prefersReducedMotion),
    reset,
    set,
    start,
    state,
    stop,
  };
}

export type MotionControlledStatusProps = Omit<
  HTMLMotionProps<"div">,
  "animate" | "children" | "initial" | "transition"
> & {
  active?: boolean;
  children: React.ReactNode;
  label: string;
  status?: DxAnimationControlState;
  transition?: Transition;
};

export function MotionControlledStatus({
  active = true,
  children,
  label,
  status,
  transition = dxMotionTransition,
  ...props
}: MotionControlledStatusProps) {
  const targetState = status ?? (active ? "active" : "idle");
  const motionControls = useDxAnimationControls({
    initial: targetState,
    transition,
  });

  React.useEffect(() => {
    void motionControls.start(targetState);
  }, [motionControls.start, targetState]);

  return (
    <motion.div
      animate={motionControls.controls}
      aria-label={label}
      data-dx-motion="controlled-status"
      data-dx-motion-state={motionControls.state}
      role="status"
      transition={transition}
      {...props}
    >
      {children}
    </motion.div>
  );
}

export function createDxAnimationControls(): LegacyAnimationControls {
  return animationControls();
}
"#;

const MOTION_FRAME_TSX: &str = r#""use client";

import * as React from "react";
import {
  motion,
  useAnimationFrame,
  useCycle,
  useReducedMotion,
  useTime,
  useTransform,
  type Cycle,
  type CycleState,
  type HTMLMotionProps,
  type MotionValue,
} from "motion/react";

export type DxMotionFramePhase = "idle" | "warming" | "live";

export const dxMotionFramePhases = [
  "idle",
  "warming",
  "live",
] as const satisfies readonly DxMotionFramePhase[];

export type DxMotionFrameCycle = CycleState<DxMotionFramePhase>;

export type DxFrameClockOptions = {
  disabled?: boolean;
  liveAfterMs?: number;
  sampleEveryMs?: number;
};

export type DxFrameClockState = {
  cyclePhase: Cycle;
  deltaMs: number;
  elapsedMs: number;
  frameCount: number;
  phase: DxMotionFramePhase;
  prefersReducedMotion: boolean;
  pulseOpacity: MotionValue<number>;
  time: MotionValue<number>;
};

export function useDxFrameClock({
  disabled = false,
  liveAfterMs = 900,
  sampleEveryMs = 250,
}: DxFrameClockOptions = {}): DxFrameClockState {
  const time = useTime();
  const prefersReducedMotion = useReducedMotion();
  const [phase, cyclePhase] = useCycle<DxMotionFramePhase>(
    ...dxMotionFramePhases,
  );
  const frameCount = React.useRef(0);
  const lastSampleAt = React.useRef(0);
  const [snapshot, setSnapshot] = React.useState({
    deltaMs: 0,
    elapsedMs: 0,
    frameCount: 0,
  });

  const pulseOpacity = useTransform(time, (latest) => {
    if (disabled || prefersReducedMotion) {
      return 1;
    }

    return 0.82 + Math.sin(latest / 240) * 0.18;
  });

  const updateFrame = React.useCallback(
    (timestamp: number, delta: number) => {
      if (disabled || prefersReducedMotion) {
        return;
      }

      frameCount.current += 1;

      if (timestamp - lastSampleAt.current < sampleEveryMs) {
        return;
      }

      lastSampleAt.current = timestamp;
      setSnapshot({
        deltaMs: Math.round(delta),
        elapsedMs: Math.round(timestamp),
        frameCount: frameCount.current,
      });

      if (timestamp >= liveAfterMs && phase !== "live") {
        cyclePhase(2);
        return;
      }

      if (timestamp >= sampleEveryMs && phase === "idle") {
        cyclePhase(1);
      }
    },
    [cyclePhase, disabled, liveAfterMs, phase, prefersReducedMotion, sampleEveryMs],
  );

  useAnimationFrame(updateFrame);

  return {
    cyclePhase,
    deltaMs: snapshot.deltaMs,
    elapsedMs: snapshot.elapsedMs,
    frameCount: snapshot.frameCount,
    phase,
    prefersReducedMotion: Boolean(prefersReducedMotion),
    pulseOpacity,
    time,
  };
}

export type MotionFrameTickerProps = Omit<HTMLMotionProps<"div">, "children"> &
  DxFrameClockOptions & {
    label?: string;
    render?: (clock: DxFrameClockState) => React.ReactNode;
  };

export function MotionFrameTicker({
  disabled,
  label = "Motion frame timing",
  liveAfterMs,
  render,
  sampleEveryMs,
  style,
  ...props
}: MotionFrameTickerProps) {
  const clock = useDxFrameClock({
    disabled,
    liveAfterMs,
    sampleEveryMs,
  });

  return (
    <motion.div
      aria-label={label}
      data-dx-motion="frame-ticker"
      data-dx-motion-phase={clock.phase}
      role="timer"
      style={{
        opacity: clock.pulseOpacity,
        ...style,
      }}
      {...props}
    >
      {render ? (
        render(clock)
      ) : (
        <span>
          {clock.phase} / {clock.elapsedMs}ms / {clock.frameCount} frames
        </span>
      )}
    </motion.div>
  );
}
"#;

const MOTION_LAYOUT_TSX: &str = r#""use client";

import * as React from "react";
import {
  LayoutGroup,
  motion,
  useInstantLayoutTransition,
  useReducedMotion,
  type HTMLMotionProps,
  type Transition,
} from "motion/react";

import { dxMotionTransition } from "./presets";

export type DxMotionLayoutMode = NonNullable<HTMLMotionProps<"div">["layout"]>;
export type DxMotionLayoutInherit = boolean | "id";

export const dxMotionLayoutDefaults = {
  inherit: true,
  layout: "position",
  transition: dxMotionTransition,
} satisfies {
  inherit: DxMotionLayoutInherit;
  layout: DxMotionLayoutMode;
  transition: Transition;
};

export function dxMotionLayoutId(scope: string, id: string | number) {
  return `${scope}-${String(id).replace(/[^a-zA-Z0-9_-]+/g, "-")}`;
}

export type DxMotionLayoutGroupProps = React.PropsWithChildren<{
  disabled?: boolean;
  id: string;
  inherit?: DxMotionLayoutInherit;
}>;

export function DxMotionLayoutGroup({
  children,
  disabled = false,
  id,
  inherit = dxMotionLayoutDefaults.inherit,
}: DxMotionLayoutGroupProps) {
  if (disabled) {
    return <>{children}</>;
  }

  return (
    <LayoutGroup id={id} inherit={inherit}>
      {children}
    </LayoutGroup>
  );
}

export type MotionLayoutItemProps = Omit<
  HTMLMotionProps<"div">,
  "layout" | "layoutDependency" | "layoutId" | "layoutRoot" | "transition"
> & {
  layout?: DxMotionLayoutMode;
  layoutDependency?: unknown;
  layoutId: string;
  layoutRoot?: boolean;
  reducedMotionLayout?: false | DxMotionLayoutMode;
  transition?: Transition;
};

export function MotionLayoutItem({
  layout = dxMotionLayoutDefaults.layout,
  layoutDependency,
  layoutId,
  layoutRoot,
  reducedMotionLayout = "position",
  transition = dxMotionLayoutDefaults.transition,
  ...props
}: MotionLayoutItemProps) {
  const prefersReducedMotion = useReducedMotion();

  return (
    <motion.div
      data-dx-motion="layout-item"
      layout={prefersReducedMotion ? reducedMotionLayout : layout}
      layoutDependency={layoutDependency}
      layoutId={layoutId}
      layoutRoot={layoutRoot}
      transition={prefersReducedMotion ? { duration: 0 } : transition}
      {...props}
    />
  );
}

export function useDxInstantLayoutTransition() {
  const startInstantLayoutTransition = useInstantLayoutTransition();
  const prefersReducedMotion = useReducedMotion();

  return React.useCallback(
    (callback?: () => void) => {
      if (prefersReducedMotion) {
        callback?.();
        return;
      }

      startInstantLayoutTransition(callback);
    },
    [prefersReducedMotion, startInstantLayoutTransition],
  );
}
"#;

const MOTION_LAZY_TSX: &str = r#""use client";

import * as React from "react";
import {
  LazyMotion,
  domAnimation,
  domMax,
  domMin,
  m,
  type HTMLMotionProps,
  type LazyProps,
} from "motion/react";

export type DxLazyMotionFeaturePreset = "animation" | "max" | "min";
export type DxLazyMotionFeatures =
  | DxLazyMotionFeaturePreset
  | LazyProps["features"];

export const dxLazyMotionFeatures = {
  animation: domAnimation,
  max: domMax,
  min: domMin,
} satisfies Record<DxLazyMotionFeaturePreset, LazyProps["features"]>;

export const dxLazyMotion = m;

export function resolveDxLazyMotionFeatures(
  features: DxLazyMotionFeatures = "animation",
) {
  return typeof features === "string" ? dxLazyMotionFeatures[features] : features;
}

export type DxLazyMotionProviderProps = React.PropsWithChildren<{
  features?: DxLazyMotionFeatures;
  strict?: boolean;
}>;

export function DxLazyMotionProvider({
  children,
  features = "animation",
  strict = false,
}: DxLazyMotionProviderProps) {
  return (
    <LazyMotion features={resolveDxLazyMotionFeatures(features)} strict={strict}>
      {children}
    </LazyMotion>
  );
}

export type MotionLazyBoxProps = HTMLMotionProps<"div"> & {
  featureBoundary?: DxLazyMotionFeaturePreset;
};

export function MotionLazyBox({
  featureBoundary = "animation",
  ...props
}: MotionLazyBoxProps) {
  return (
    <m.div
      data-dx-motion="lazy-motion-box"
      data-dx-motion-features={featureBoundary}
      {...props}
    />
  );
}
"#;

const MOTION_VALUES_TSX: &str = r#""use client";

import * as React from "react";
import {
  motion,
  useMotionTemplate,
  useMotionValue,
  useMotionValueEvent,
  useReducedMotion,
  useSpring,
  useTransform,
  useVelocity,
  type HTMLMotionProps,
  type MotionValue,
  type SpringOptions,
} from "motion/react";

export const dxMotionValueSpring = {
  stiffness: 160,
  damping: 24,
  mass: 0.32,
} satisfies SpringOptions;

export function dxClampMotionValue(value: number, max: number) {
  const safeMax = Math.max(1, max);
  return Math.min(safeMax, Math.max(0, value));
}

export type DxMotionValueMeterOptions = {
  value: number;
  max?: number;
  spring?: SpringOptions;
  onValueChange?: (latest: number) => void;
};

export type DxMotionValueMeterState = {
  boundedValue: number;
  rawValue: MotionValue<number>;
  value: MotionValue<number>;
  progress: MotionValue<number>;
  width: MotionValue<string>;
  indicatorOpacity: MotionValue<number>;
  filter: MotionValue<string>;
  velocity: MotionValue<number>;
  prefersReducedMotion: boolean;
  set: (nextValue: number) => void;
};

export function useDxMotionValueMeter({
  value,
  max = 100,
  spring,
  onValueChange,
}: DxMotionValueMeterOptions): DxMotionValueMeterState {
  const safeMax = Math.max(1, max);
  const boundedValue = dxClampMotionValue(value, safeMax);
  const rawValue = useMotionValue(boundedValue);
  const smoothValue = useSpring(rawValue, {
    ...dxMotionValueSpring,
    ...spring,
  });
  const prefersReducedMotion = useReducedMotion();
  const activeValue = prefersReducedMotion ? rawValue : smoothValue;
  const progress = useTransform(activeValue, [0, safeMax], [0, 1], {
    clamp: true,
  });
  const percent = useTransform(progress, [0, 1], [0, 100]);
  const width = useMotionTemplate`${percent}%`;
  const indicatorOpacity = useTransform(progress, [0, 1], [0.52, 1]);
  const shadowRadius = useTransform(progress, [0, 1], [0, 18]);
  const filter = useMotionTemplate`drop-shadow(0 0 ${shadowRadius}px var(--dx-motion-meter-shadow, currentColor))`;
  const velocity = useVelocity(activeValue);

  React.useEffect(() => {
    rawValue.set(boundedValue);
  }, [boundedValue, rawValue]);

  const handleValueChange = React.useCallback(
    (latest: number) => {
      onValueChange?.(latest);
    },
    [onValueChange],
  );

  useMotionValueEvent(activeValue, "change", handleValueChange);

  const set = React.useCallback(
    (nextValue: number) => {
      rawValue.set(dxClampMotionValue(nextValue, safeMax));
    },
    [rawValue, safeMax],
  );

  return {
    boundedValue,
    rawValue,
    value: activeValue,
    progress,
    width,
    indicatorOpacity,
    filter,
    velocity,
    prefersReducedMotion: Boolean(prefersReducedMotion),
    set,
  };
}

export type MotionValueMeterProps = Omit<
  HTMLMotionProps<"div">,
  "children"
> &
  DxMotionValueMeterOptions & {
    indicatorClassName?: string;
    label: string;
    trackClassName?: string;
  };

export function MotionValueMeter({
  indicatorClassName = "dx-motion-meter-indicator",
  label,
  max = 100,
  trackClassName = "dx-motion-meter-track",
  value,
  spring,
  onValueChange,
  ...props
}: MotionValueMeterProps) {
  const meter = useDxMotionValueMeter({
    max,
    onValueChange,
    spring,
    value,
  });
  const safeMax = Math.max(1, max);

  return (
    <motion.div
      aria-label={label}
      aria-valuemax={safeMax}
      aria-valuemin={0}
      aria-valuenow={meter.boundedValue}
      data-dx-motion="motion-value-meter"
      role="meter"
      {...props}
    >
      <div className={trackClassName} data-dx-motion="motion-value-meter-track">
        <motion.div
          className={indicatorClassName}
          data-dx-motion="motion-value-meter-indicator"
          style={{
            filter: meter.filter,
            opacity: meter.indicatorOpacity,
            width: meter.width,
          }}
        />
      </div>
    </motion.div>
  );
}
"#;

const MOTION_PAGE_VISIBILITY_TSX: &str = r#""use client";

import * as React from "react";
import {
  motion,
  usePageInView,
  type HTMLMotionProps,
} from "motion/react";

export type DxPageVisibilityStatus = "visible" | "hidden";

export type DxPageVisibilityOptions = {
  hiddenLabel?: string;
  visibleLabel?: string;
};

export type DxPageVisibilityState = {
  isPageInView: boolean;
  label: string;
  status: DxPageVisibilityStatus;
};

export function useDxPageVisibility({
  hiddenLabel = "Page hidden",
  visibleLabel = "Page visible",
}: DxPageVisibilityOptions = {}): DxPageVisibilityState {
  const isPageInView = usePageInView();
  const status: DxPageVisibilityStatus = isPageInView ? "visible" : "hidden";

  return {
    isPageInView,
    label: isPageInView ? visibleLabel : hiddenLabel,
    status,
  };
}

export type MotionPageVisibilityBadgeProps = Omit<
  HTMLMotionProps<"div">,
  "children"
> &
  DxPageVisibilityOptions & {
    children?: (state: DxPageVisibilityState) => React.ReactNode;
  };

export function MotionPageVisibilityBadge({
  children,
  hiddenLabel,
  visibleLabel,
  ...props
}: MotionPageVisibilityBadgeProps) {
  const state = useDxPageVisibility({ hiddenLabel, visibleLabel });

  return (
    <motion.div
      animate={{ opacity: state.isPageInView ? 1 : 0.68 }}
      aria-live="polite"
      data-dx-motion="page-visibility-badge"
      data-dx-page-in-view={state.isPageInView}
      data-dx-page-visibility={state.status}
      role="status"
      transition={{ duration: 0.16 }}
      {...props}
    >
      {children ? children(state) : state.label}
    </motion.div>
  );
}
"#;

const MOTION_PROVIDER_TSX: &str = r#""use client";

import * as React from "react";
import { MotionConfig, type MotionConfigProps } from "motion/react";

import { dxMotionTransition } from "./presets";

export const dxMotionConfigDefaults = {
  reducedMotion: "user",
  transition: dxMotionTransition,
  skipAnimations: false,
} satisfies Pick<
  MotionConfigProps,
  "reducedMotion" | "skipAnimations" | "transition"
>;

export type DxMotionProviderProps = MotionConfigProps;

export function DxMotionProvider({
  children,
  reducedMotion = dxMotionConfigDefaults.reducedMotion,
  skipAnimations = false,
  transition = dxMotionConfigDefaults.transition,
  ...props
}: DxMotionProviderProps) {
  return (
    <MotionConfig
      reducedMotion={reducedMotion}
      skipAnimations={skipAnimations}
      transition={transition}
      {...props}
    >
      {children}
    </MotionConfig>
  );
}
"#;

const MOTION_PRESENCE_TSX: &str = r#""use client";

import * as React from "react";
import {
  AnimatePresence,
  LayoutGroup,
  motion,
  useIsPresent,
  usePresence,
  useReducedMotion,
  type AnimatePresenceProps,
  type HTMLMotionProps,
  type Variants,
} from "motion/react";

import { dxMotionTransition } from "./presets";

export const dxMotionPresenceDefaults = {
  initial: false,
  mode: "popLayout",
  presenceAffectsLayout: true,
} satisfies Pick<
  AnimatePresenceProps,
  "initial" | "mode" | "presenceAffectsLayout"
>;

export type DxMotionPresenceProps = React.PropsWithChildren<
  AnimatePresenceProps & {
    layoutGroupId?: string;
    inheritLayout?: boolean | "id";
  }
>;

export function DxMotionPresence({
  children,
  initial = dxMotionPresenceDefaults.initial,
  mode = dxMotionPresenceDefaults.mode,
  presenceAffectsLayout = dxMotionPresenceDefaults.presenceAffectsLayout,
  layoutGroupId,
  inheritLayout = "id",
  ...props
}: DxMotionPresenceProps) {
  const presence = (
    <AnimatePresence
      initial={initial}
      mode={mode}
      presenceAffectsLayout={presenceAffectsLayout}
      {...props}
    >
      {children}
    </AnimatePresence>
  );

  if (!layoutGroupId) {
    return presence;
  }

  return (
    <LayoutGroup id={layoutGroupId} inherit={inheritLayout}>
      {presence}
    </LayoutGroup>
  );
}

export type DxPresenceState = {
  isPresent: boolean;
  safeToRemove: (() => void) | null;
};

export function useDxPresence(subscribe = true): DxPresenceState {
  const [isPresent, safeToRemove] = usePresence(subscribe);
  const isPresentInTree = useIsPresent();

  return {
    isPresent: Boolean(isPresent && isPresentInTree),
    safeToRemove: safeToRemove ?? null,
  };
}

export const dxMotionPresenceVariants = {
  hidden: { opacity: 0, y: 8, scale: 0.98 },
  visible: { opacity: 1, y: 0, scale: 1 },
  exit: {
    opacity: 0,
    y: -6,
    scale: 0.98,
    transition: { duration: 0.16 },
  },
} satisfies Variants;

export const dxReducedMotionPresenceVariants = {
  hidden: { opacity: 0 },
  visible: { opacity: 1 },
  exit: {
    opacity: 0,
    transition: { duration: 0.08 },
  },
} satisfies Variants;

export type MotionPresenceItemProps = Omit<
  HTMLMotionProps<"div">,
  "animate" | "exit" | "initial" | "transition" | "variants"
> & {
  variants?: Variants;
};

export function MotionPresenceItem({
  variants,
  layout = "position",
  ...props
}: MotionPresenceItemProps) {
  const prefersReducedMotion = useReducedMotion();

  return (
    <motion.div
      data-dx-motion="presence-item"
      initial="hidden"
      animate="visible"
      exit="exit"
      layout={layout}
      variants={
        variants ??
        (prefersReducedMotion
          ? dxReducedMotionPresenceVariants
          : dxMotionPresenceVariants)
      }
      transition={dxMotionTransition}
      {...props}
    />
  );
}
"#;

const MOTION_REORDER_TSX: &str = r#""use client";

import * as React from "react";
import {
  Reorder,
  useDragControls,
  useReducedMotion,
  type DragControls,
  type HTMLMotionProps,
} from "motion/react";

export type DxReorderAxis = "x" | "y";
export type DxReorderValue = string | number;

export const dxMotionReorderDefaults = {
  axis: "y",
  dragListener: false,
  layout: "position",
} as const;

export type DxReorderGroupProps<Value extends DxReorderValue> = Omit<
  HTMLMotionProps<"ul">,
  "onReorder"
> & {
  as?: "ul" | "ol" | "div";
  axis?: DxReorderAxis;
  values: readonly Value[];
  onReorder: (values: Value[]) => void;
};

export function DxReorderGroup<Value extends DxReorderValue>({
  as = "ul",
  axis = dxMotionReorderDefaults.axis,
  values,
  onReorder,
  style,
  children,
  ...props
}: DxReorderGroupProps<Value>) {
  return (
    <Reorder.Group
      as={as}
      axis={axis}
      data-dx-motion="reorder-group"
      onReorder={onReorder}
      style={{ overflowAnchor: "none", ...style }}
      values={[...values]}
      {...props}
    >
      {children}
    </Reorder.Group>
  );
}

export type DxReorderItemProps<Value extends DxReorderValue> = Omit<
  HTMLMotionProps<"li">,
  "layout" | "value"
> & {
  as?: "li" | "div" | "article";
  value: Value;
  dragControls?: DragControls;
  dragListener?: boolean;
  layout?: true | "position";
  reducedMotionLayout?: true | "position";
};

export function DxReorderItem<Value extends DxReorderValue>({
  as = "li",
  dragListener = dxMotionReorderDefaults.dragListener,
  layout = dxMotionReorderDefaults.layout,
  reducedMotionLayout = "position",
  whileDrag = { scale: 1.01 },
  ...props
}: DxReorderItemProps<Value>) {
  const prefersReducedMotion = useReducedMotion();

  return (
    <Reorder.Item
      as={as}
      data-dx-motion="reorder-item"
      dragListener={dragListener}
      layout={prefersReducedMotion ? reducedMotionLayout : layout}
      whileDrag={prefersReducedMotion ? undefined : whileDrag}
      {...props}
    />
  );
}

export type DxReorderControlsOptions = {
  ariaLabel?: string;
  disabled?: boolean;
  snapToCursor?: boolean;
};

export function useDxReorderControls({
  ariaLabel = "Drag to reorder",
  disabled = false,
  snapToCursor = true,
}: DxReorderControlsOptions = {}) {
  const dragControls = useDragControls();

  const startDrag = React.useCallback(
    (event: React.PointerEvent<HTMLElement>) => {
      if (disabled) {
        return;
      }

      event.preventDefault();
      dragControls.start(event, { snapToCursor });
    },
    [disabled, dragControls, snapToCursor],
  );

  const dragHandleProps = {
    "aria-disabled": disabled || undefined,
    "aria-label": ariaLabel,
    "data-dx-motion": "reorder-handle",
    onPointerDown: startDrag,
    role: "button",
    tabIndex: disabled ? -1 : 0,
  } satisfies React.HTMLAttributes<HTMLElement>;

  return {
    disabled,
    dragControls,
    dragHandleProps,
    dragListener: false,
    startDrag,
  } as const;
}
"#;

const MOTION_REVEAL_TSX: &str = r#""use client";

import * as React from "react";
import {
  motion,
  useInView,
  useReducedMotion,
  type HTMLMotionProps,
  type UseInViewOptions,
} from "motion/react";

import {
  dxMotionPreset,
  dxMotionTransition,
  dxMotionViewport,
  type DxMotionPresetName,
} from "./presets";

export type MotionRevealProps = Omit<
  HTMLMotionProps<"div">,
  "animate" | "initial" | "transition" | "variants"
> & {
  preset?: DxMotionPresetName;
  once?: boolean;
  amount?: UseInViewOptions["amount"];
  margin?: UseInViewOptions["margin"];
  delay?: number;
  disabled?: boolean;
};

export function MotionReveal({
  preset = "fade-up",
  once = dxMotionViewport.once,
  amount = dxMotionViewport.amount,
  margin = dxMotionViewport.margin,
  delay = 0,
  disabled = false,
  className,
  children,
  ...props
}: MotionRevealProps) {
  const ref = React.useRef<HTMLDivElement>(null);
  const inView = useInView(ref, { once, amount, margin });
  const prefersReducedMotion = useReducedMotion();

  if (disabled || prefersReducedMotion) {
    return (
      <div ref={ref} className={className} {...props}>
        {children}
      </div>
    );
  }

  return (
    <motion.div
      ref={ref}
      className={className}
      initial="hidden"
      animate={inView ? "visible" : "hidden"}
      variants={dxMotionPreset(preset)}
      transition={{ ...dxMotionTransition, delay }}
      {...props}
    >
      {children}
    </motion.div>
  );
}
"#;

const MOTION_SCOPED_ANIMATE_TSX: &str = r#""use client";

import * as React from "react";
import {
  useAnimate,
  useReducedMotion,
  type AnimationOptions,
  type AnimationPlaybackControlsWithThen,
  type DOMKeyframesDefinition,
  type ElementOrSelector,
} from "motion/react";

import { dxMotionTransition } from "./presets";

export type DxScopedAnimateResult = AnimationPlaybackControlsWithThen | null;

export type DxScopedAnimate = (
  target: ElementOrSelector,
  keyframes: DOMKeyframesDefinition,
  options?: AnimationOptions,
) => DxScopedAnimateResult;

export function useDxScopedAnimate<T extends Element = HTMLElement>() {
  const [scope, animate] = useAnimate<T>();
  const prefersReducedMotion = useReducedMotion();

  const animateTarget = React.useCallback<DxScopedAnimate>(
    (target, keyframes, options = {}) => {
      if (prefersReducedMotion) {
        return null;
      }

      return animate(target, keyframes, {
        duration: dxMotionTransition.duration,
        ease: dxMotionTransition.ease,
        ...options,
      });
    },
    [animate, prefersReducedMotion],
  );

  return {
    scope,
    animate: animateTarget,
    prefersReducedMotion: Boolean(prefersReducedMotion),
  } as const;
}

export type DxMotionPressFeedbackOptions = {
  scale?: number;
  y?: number;
  duration?: number;
  disabled?: boolean;
};

export function useDxMotionPressFeedback({
  scale = 1.015,
  y = -2,
  duration = 0.18,
  disabled = false,
}: DxMotionPressFeedbackOptions = {}) {
  const { animate, prefersReducedMotion, scope } = useDxScopedAnimate<HTMLElement>();

  const press = React.useCallback(
    (target: Element | null): DxScopedAnimateResult => {
      if (disabled || !target) {
        return null;
      }

      return animate(
        target,
        {
          scale: [1, scale, 1],
          y: [0, y, 0],
        },
        { duration },
      );
    },
    [animate, disabled, duration, scale, y],
  );

  return {
    scope,
    press,
    prefersReducedMotion,
  } as const;
}
"#;

const MOTION_SCROLL_PROGRESS_TSX: &str = r#""use client";

import * as React from "react";
import {
  motion,
  useReducedMotion,
  useScroll,
  useSpring,
  type HTMLMotionProps,
  type MotionValue,
  type SpringOptions,
  type UseScrollOptions,
} from "motion/react";

export type DxScrollProgressOptions = UseScrollOptions & {
  disabled?: boolean;
  smooth?: boolean;
  spring?: SpringOptions;
};

export type DxScrollProgressState = {
  progress: MotionValue<number>;
  rawProgress: MotionValue<number>;
  prefersReducedMotion: boolean;
};

export function useDxScrollProgress({
  disabled = false,
  smooth = true,
  spring,
  ...scrollOptions
}: DxScrollProgressOptions = {}): DxScrollProgressState {
  const { scrollYProgress } = useScroll(scrollOptions);
  const prefersReducedMotion = useReducedMotion();
  const springProgress = useSpring(scrollYProgress, {
    stiffness: 120,
    damping: 26,
    mass: 0.25,
    ...spring,
  });

  return {
    progress:
      disabled || prefersReducedMotion || !smooth
        ? scrollYProgress
        : springProgress,
    rawProgress: scrollYProgress,
    prefersReducedMotion: Boolean(prefersReducedMotion),
  };
}

export type MotionScrollProgressProps = Omit<
  HTMLMotionProps<"div">,
  "style"
> & {
  label?: string;
  scroll?: DxScrollProgressOptions;
  style?: HTMLMotionProps<"div">["style"];
};

export function MotionScrollProgress({
  label = "Page scroll progress",
  scroll,
  style,
  ...props
}: MotionScrollProgressProps) {
  const { progress } = useDxScrollProgress(scroll);

  return (
    <motion.div
      aria-label={label}
      data-dx-motion="scroll-progress"
      role="progressbar"
      style={{
        ...style,
        scaleX: progress,
        transformOrigin: "0% 50%",
      }}
      {...props}
    />
  );
}
"#;

const MOTION_WILL_CHANGE_TSX: &str = r#""use client";

import * as React from "react";
import {
  motion,
  useReducedMotion,
  useWillChange,
  WillChangeMotionValue,
  type HTMLMotionProps,
} from "motion/react";

export type DxWillChangeProperty =
  | "opacity"
  | "transform"
  | "x"
  | "y"
  | "scale"
  | "filter";

export const dxWillChangeProperties = [
  "transform",
  "opacity",
] as const satisfies readonly DxWillChangeProperty[];

export type DxWillChangeOptions = {
  disabled?: boolean;
  properties?: readonly DxWillChangeProperty[];
};

export type DxWillChangeState = {
  disabled: boolean;
  prefersReducedMotion: boolean;
  properties: readonly DxWillChangeProperty[];
  willChange: ReturnType<typeof useWillChange>;
};

export function createDxWillChangeValue(
  properties: readonly DxWillChangeProperty[] = dxWillChangeProperties,
) {
  const willChange = new WillChangeMotionValue("auto");

  for (const property of properties) {
    willChange.add(property);
  }

  return willChange;
}

export function useDxWillChange({
  disabled = false,
  properties = dxWillChangeProperties,
}: DxWillChangeOptions = {}): DxWillChangeState {
  const willChange = useWillChange();
  const prefersReducedMotion = useReducedMotion();
  const propertyKey = properties.join(",");

  React.useEffect(() => {
    if (disabled || prefersReducedMotion) {
      return;
    }

    for (const property of properties) {
      willChange.add(property);
    }
  }, [disabled, prefersReducedMotion, properties, propertyKey, willChange]);

  return {
    disabled,
    prefersReducedMotion: Boolean(prefersReducedMotion),
    properties,
    willChange,
  };
}

export type MotionWillChangeBoxProps = Omit<
  HTMLMotionProps<"div">,
  "style"
> &
  DxWillChangeOptions & {
    style?: HTMLMotionProps<"div">["style"];
  };

export function MotionWillChangeBox({
  disabled,
  properties,
  style,
  ...props
}: MotionWillChangeBoxProps) {
  const state = useDxWillChange({ disabled, properties });

  return (
    <motion.div
      data-dx-motion="will-change-box"
      data-dx-motion-disabled={state.disabled || state.prefersReducedMotion}
      data-dx-motion-properties={state.properties.join(",")}
      style={{
        willChange: state.willChange,
        ...style,
      }}
      {...props}
    />
  );
}
"#;

const MOTION_DASHBOARD_WORKFLOW_TS: &str = r#"export type MotionDashboardStageId = "reveal" | "measure" | "reorder";
export type MotionDashboardReducedMotionPolicy = "system" | "preview";
export type MotionDashboardOrderDirection = "previous" | "next";
export type MotionDashboardOrderTarget = MotionDashboardOrderDirection | "first" | "last";
export type MotionDashboardReceiptStatus = "local-preview-ready";
export type MotionDashboardSelectedSurfaceId =
  | "provider-policy"
  | "layout-reorder"
  | "dashboard-workflow";
export type MotionDashboardDxCheckStatus =
  | "present"
  | "stale"
  | "missing-receipt"
  | "blocked"
  | "unsupported-surface";

export interface MotionDashboardStage {
  id: MotionDashboardStageId;
  label: string;
  publicApi: string;
  packageExport: string;
  progress: number;
  appBoundary: string;
}

export interface MotionDashboardReceipt {
  receiptId: string;
  status: MotionDashboardReceiptStatus;
  packageId: "animation/motion";
  officialPackageName: "Motion & Animation";
  cliCommand: "dx add motion-animation --write";
  stageId: MotionDashboardStageId;
  progress: number;
  orderedSurfaceIds: readonly MotionDashboardStageId[];
  selectedSurfaceIds: readonly MotionDashboardSelectedSurfaceId[];
  reducedMotionPolicy: MotionDashboardReducedMotionPolicy;
  sourceMirror: string;
  dxCheckVisibility: MotionDashboardDxCheckVisibility;
  nextAction: string;
}

export interface MotionDashboardPreference {
  orderedSurfaceIds: readonly MotionDashboardStageId[];
  reducedMotionPolicy: MotionDashboardReducedMotionPolicy;
}

export interface MotionDashboardSelectedSurface {
  id: MotionDashboardSelectedSurfaceId;
  files: readonly string[];
  upstreamPublicApis: readonly string[];
  appOwnedBoundary: string;
}

export interface MotionDashboardDxCheckLegendEntry {
  status: MotionDashboardDxCheckStatus;
  meaning: string;
}

export interface MotionDashboardDxCheckSurface {
  id: string;
  status: MotionDashboardDxCheckStatus;
  sourceFile: string;
  materializedFile: string;
  receiptPath: string;
  nextAction: string;
}

export interface MotionDashboardDxCheckVisibility {
  schema: "dx.forge.package.dx_check_visibility";
  packageId: "animation/motion";
  officialPackageName: "Motion & Animation";
  currentStatus: MotionDashboardDxCheckStatus;
  statusLegend: readonly MotionDashboardDxCheckLegendEntry[];
  monitoredSurfaces: readonly MotionDashboardDxCheckSurface[];
}

export const motionDashboardPreferenceStorageKey = "dx.launch.motion.dashboard";

export const motionDashboardOfficialPackageName = "Motion & Animation";
export const motionDashboardCliCommand = "dx add motion-animation --write";

export const motionDashboardInspectedSourceFiles = [
  "packages/motion/src/react.ts",
  "packages/framer-motion/src/index.ts",
  "packages/framer-motion/src/components/AnimatePresence/index.tsx",
  "packages/framer-motion/src/components/Reorder/Group.tsx",
  "packages/framer-motion/src/value/use-scroll.ts",
] as const;

export const motionDashboardSelectedSurfaces = [
  {
    id: "provider-policy",
    files: ["js/motion/provider.tsx", "js/motion/reveal.tsx"],
    upstreamPublicApis: ["MotionConfig", "motion", "useInView", "useReducedMotion"],
    appOwnedBoundary: "app-wide reduced-motion and route reveal policy",
  },
  {
    id: "layout-reorder",
    files: ["js/motion/layout.tsx", "js/motion/reorder.tsx", "js/motion/presence.tsx"],
    upstreamPublicApis: ["LayoutGroup", "Reorder", "AnimatePresence", "useDragControls"],
    appOwnedBoundary: "persistent dashboard ordering and route-level choreography",
  },
  {
    id: "dashboard-workflow",
    files: [
      "js/motion/dashboard-workflow.ts",
      "components/launch/motion-interaction-proof.tsx",
    ],
    upstreamPublicApis: ["useMotionValue", "useScroll", "useSpring", "useAnimate"],
    appOwnedBoundary: "governed runtime/browser proof and animation performance budget",
  },
] as const satisfies readonly MotionDashboardSelectedSurface[];

export const motionDashboardDxCheckVisibility = {
  schema: "dx.forge.package.dx_check_visibility",
  packageId: "animation/motion",
  officialPackageName: motionDashboardOfficialPackageName,
  currentStatus: "present",
  statusLegend: [
    {
      status: "present",
      meaning: "selected Motion & Animation surfaces, receipt, and source markers are present",
    },
    {
      status: "stale",
      meaning: "materialized Motion & Animation files or hashes no longer match the Forge receipt",
    },
    {
      status: "missing-receipt",
      meaning: "selected Motion & Animation surfaces exist without the dashboard workflow receipt",
    },
    {
      status: "blocked",
      meaning: "runtime proof or app-owned policy approval is required before claiming more",
    },
    {
      status: "unsupported-surface",
      meaning: "a requested Motion & Animation surface is outside the selected upstream-backed set",
    },
  ],
  monitoredSurfaces: [
    {
      id: "motion-dashboard-workflow",
      status: "present",
      sourceFile: "examples/template/template-shell.tsx",
      materializedFile: "components/template-app/template-shell.tsx",
      receiptPath:
        "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
      nextAction: "Run the Motion source guards after editing the dashboard workflow surface.",
    },
    {
      id: "motion-interaction-proof",
      status: "present",
      sourceFile: "examples/template/motion-interaction-proof.tsx",
      materializedFile: "components/launch/motion-interaction-proof.tsx",
      receiptPath:
        "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
      nextAction: "Keep Zed/DX Studio selectors aligned with the visible interaction proof.",
    },
  ],
} as const satisfies MotionDashboardDxCheckVisibility;

export const motionDashboardPackage = {
  packageId: "animation/motion",
  officialPackageName: motionDashboardOfficialPackageName,
  cliCommand: motionDashboardCliCommand,
  aliases: ["motion", "framer-motion", "motion/react", "animation/motion"],
  upstreamPackage: "motion",
  upstreamVersion: "12.38.0",
  sourceMirror: "G:/WWW/inspirations/motion",
  inspectedSourceFiles: motionDashboardInspectedSourceFiles,
  selectedSurfaces: motionDashboardSelectedSurfaces,
  dxCheckVisibility: motionDashboardDxCheckVisibility,
  provenance: {
    upstreamPackage: "motion",
    upstreamVersion: "12.38.0",
    sourcePackages: [
      "packages/motion",
      "packages/framer-motion",
      "packages/motion-dom",
      "packages/motion-utils",
    ],
    publicApi: [
      "motion",
      "m",
      "MotionConfig",
      "LazyMotion",
      "AnimatePresence",
      "LayoutGroup",
      "Reorder",
      "domAnimation",
      "domMax",
      "domMin",
      "useAnimationControls",
      "useAnimation",
      "animationControls",
      "useAnimationFrame",
      "useTime",
      "useCycle",
      "useWillChange",
      "WillChangeMotionValue",
      "usePageInView",
      "useInstantLayoutTransition",
      "useInView",
      "useMotionValue",
      "useTransform",
      "useMotionTemplate",
      "useMotionValueEvent",
      "useVelocity",
      "useScroll",
      "useSpring",
      "useReducedMotion",
      "useAnimate",
      "usePresence",
      "useIsPresent",
      "useDragControls",
      "AnimationPlaybackControlsWithThen",
      "MotionValue",
      "Transition",
      "Variants",
    ],
  },
  exportedFiles: [
    "js/motion/provider.tsx",
    "js/motion/controls.tsx",
    "js/motion/lazy.tsx",
    "js/motion/layout.tsx",
    "js/motion/motion-values.tsx",
    "js/motion/presence.tsx",
    "js/motion/reorder.tsx",
    "js/motion/scroll-progress.tsx",
    "js/motion/dashboard-workflow.ts",
    "js/motion/metadata.ts",
  ],
  requiredEnv: [],
  appOwnedBoundaries: [
    "global motion policy and reduced-motion review",
    "route transition choreography",
    "production preference sync beyond local storage",
    "performance budget for dashboard animation density",
  ],
  receiptPaths: [
    ".dx/forge/receipts/*-animation-motion.json",
    "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
    ".dx/forge/docs/animation-motion.md",
    "examples/dashboard/README.md#motion-dashboard-workflow",
  ],
} as const;

export const motionDashboardStages = [
  {
    id: "reveal",
    label: "Reveal",
    publicApi: "MotionConfig + MotionReveal",
    packageExport: "motion/provider.tsx + motion/reveal.tsx",
    progress: 34,
    appBoundary: "choose launch-page reveal timing and reduced-motion policy",
  },
  {
    id: "measure",
    label: "Measure",
    publicApi: "useMotionValue + useScroll + useSpring",
    packageExport: "motion/motion-values.tsx + motion/scroll-progress.tsx",
    progress: 67,
    appBoundary: "decide which dashboard metrics deserve animated feedback",
  },
  {
    id: "reorder",
    label: "Reorder",
    publicApi: "LayoutGroup + Reorder + AnimatePresence",
    packageExport: "motion/layout.tsx + motion/reorder.tsx + motion/presence.tsx",
    progress: 100,
    appBoundary: "persist user ordering and route-level choreography policy",
  },
] as const satisfies readonly MotionDashboardStage[];

function isMotionDashboardStageId(value: string): value is MotionDashboardStageId {
  return motionDashboardStages.some((stage) => stage.id === value);
}

function normalizeMotionDashboardOrder(
  value: unknown,
): readonly MotionDashboardStageId[] | null {
  if (!Array.isArray(value)) return null;

  const stageIds = value.filter(
    (stageId): stageId is MotionDashboardStageId =>
      typeof stageId === "string" && isMotionDashboardStageId(stageId),
  );
  const uniqueStageIds = [...new Set(stageIds)];

  if (uniqueStageIds.length !== motionDashboardStages.length) return null;
  return uniqueStageIds;
}

export function readMotionDashboardPreference(
  storage: Pick<Storage, "getItem"> | null | undefined,
): MotionDashboardPreference | null {
  if (!storage) return null;

  try {
    const rawPreference = storage.getItem(motionDashboardPreferenceStorageKey);
    if (!rawPreference) return null;

    const preference = JSON.parse(rawPreference) as Partial<MotionDashboardPreference>;
    const orderedSurfaceIds = normalizeMotionDashboardOrder(
      preference.orderedSurfaceIds,
    );
    const reducedMotionPolicy =
      preference.reducedMotionPolicy === "preview" ? "preview" : "system";

    if (!orderedSurfaceIds) return null;

    return {
      orderedSurfaceIds,
      reducedMotionPolicy,
    };
  } catch {
    return null;
  }
}

export function writeMotionDashboardPreference(
  storage: Pick<Storage, "setItem"> | null | undefined,
  preference: MotionDashboardPreference,
) {
  if (!storage) return;

  try {
    storage.setItem(
      motionDashboardPreferenceStorageKey,
      JSON.stringify({
        orderedSurfaceIds: [...preference.orderedSurfaceIds],
        reducedMotionPolicy: preference.reducedMotionPolicy,
      }),
    );
  } catch {
    // Storage can be unavailable in embedded previews; the visible state remains source-owned.
  }
}

export function getMotionDashboardStage(stageId: MotionDashboardStageId) {
  return (
    motionDashboardStages.find((stage) => stage.id === stageId) ??
    motionDashboardStages[0]
  );
}

export function moveMotionDashboardStage(
  orderedStageIds: readonly MotionDashboardStageId[],
  stageId: MotionDashboardStageId,
  direction: MotionDashboardOrderDirection,
) {
  return placeMotionDashboardStage(orderedStageIds, stageId, direction);
}

export function placeMotionDashboardStage(
  orderedStageIds: readonly MotionDashboardStageId[],
  stageId: MotionDashboardStageId,
  target: MotionDashboardOrderTarget,
) {
  const currentIndex = orderedStageIds.indexOf(stageId);
  if (currentIndex < 0) return [...orderedStageIds];

  const nextIndex = {
    first: 0,
    last: orderedStageIds.length - 1,
    previous: Math.max(0, currentIndex - 1),
    next: Math.min(orderedStageIds.length - 1, currentIndex + 1),
  }[target];

  if (nextIndex === currentIndex) return [...orderedStageIds];

  const nextOrder = [...orderedStageIds];
  const [stage] = nextOrder.splice(currentIndex, 1);
  nextOrder.splice(nextIndex, 0, stage);
  return nextOrder;
}

export function createMotionDashboardReceipt(input: {
  stageId: MotionDashboardStageId;
  orderedSurfaceIds: readonly MotionDashboardStageId[];
  reducedMotionPolicy?: MotionDashboardReducedMotionPolicy;
}): MotionDashboardReceipt {
  const stage = getMotionDashboardStage(input.stageId);
  const reducedMotionPolicy = input.reducedMotionPolicy ?? "system";

  return {
    receiptId: `dx-motion-dashboard-${stage.id}-${stage.progress}-${reducedMotionPolicy}`,
    status: "local-preview-ready",
    packageId: motionDashboardPackage.packageId,
    officialPackageName: motionDashboardOfficialPackageName,
    cliCommand: motionDashboardCliCommand,
    stageId: stage.id,
    progress: stage.progress,
    orderedSurfaceIds: [...input.orderedSurfaceIds],
    selectedSurfaceIds: motionDashboardSelectedSurfaces.map((surface) => surface.id),
    reducedMotionPolicy,
    sourceMirror: motionDashboardPackage.sourceMirror,
    dxCheckVisibility: motionDashboardDxCheckVisibility,
    nextAction:
      reducedMotionPolicy === "preview"
        ? "Keep reduced-motion preview enabled while app-owned route transitions are reviewed."
        : "Wire this receipt to the app-owned route transition and dashboard preference policy.",
  };
}
"#;

const MOTION_METADATA_TS: &str = r#"export const dxMotionForgePackage = {
  packageId: "animation/motion",
  officialPackageName: "Motion & Animation",
  upstreamPackage: "motion",
  upstreamVersion: "12.38.0",
  forgeVersion: "12.38.0-dx.12",
  importPath: "motion/react",
  cliCommand: "dx add motion-animation --write",
  inspectedSourceFiles: [
    "packages/motion/src/react.ts",
    "packages/framer-motion/src/index.ts",
    "packages/framer-motion/src/components/AnimatePresence/index.tsx",
    "packages/framer-motion/src/components/Reorder/Group.tsx",
    "packages/framer-motion/src/value/use-scroll.ts",
  ],
  selectedSurfaces: [
    "provider-policy",
    "layout-reorder",
    "dashboard-workflow",
  ],
  dxCheckVisibility: {
    schema: "dx.forge.package.dx_check_visibility",
    currentStatus: "present",
    statuses: [
      "present",
      "stale",
      "missing-receipt",
      "blocked",
      "unsupported-surface",
    ],
    receiptPath: "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
    monitoredSurfaces: ["motion-dashboard-workflow", "motion-interaction-proof"],
  },
  sourceSurface: [
    "motion component factory from motion/react",
    "useAnimationControls, useAnimation, animationControls, and LegacyAnimationControls",
    "useAnimationFrame, useTime, and useCycle frame timing",
    "useWillChange and WillChangeMotionValue performance hints",
    "MotionConfig provider defaults",
    "LayoutGroup, layoutId, layoutDependency, and useInstantLayoutTransition",
    "LazyMotion, domAnimation, domMax, domMin, and m feature bundles",
    "useMotionValue, useTransform, useMotionTemplate, useMotionValueEvent, and useVelocity",
    "usePageInView document visibility hook",
    "AnimatePresence, LayoutGroup, and presence hooks",
    "Reorder and useDragControls drag sorting",
    "useInView viewport observation hook",
    "useReducedMotion accessibility hook",
    "useAnimate scoped animation hook",
    "useScroll scroll progress hook",
    "useSpring MotionValue smoothing hook",
    "AnimationPlaybackControlsWithThen control contract",
    "MotionValue scroll progress contract",
    "Transition and Variants type contracts",
  ],
  dependencies: [
    {
      name: "motion",
      version: "^12.38.0",
      required: true,
    },
    {
      name: "react",
      version: "^18 || ^19",
      required: true,
    },
  ],
  materializedFiles: [
    "motion/presets.ts",
    "motion/controls.tsx",
    "motion/frame.tsx",
    "motion/layout.tsx",
    "motion/lazy.tsx",
    "motion/motion-values.tsx",
    "motion/page-visibility.tsx",
    "motion/provider.tsx",
    "motion/presence.tsx",
    "motion/reorder.tsx",
    "motion/reveal.tsx",
    "motion/scoped-animate.tsx",
    "motion/scroll-progress.tsx",
    "motion/will-change.tsx",
    "motion/dashboard-workflow.ts",
    "motion/metadata.ts",
    "motion/README.md",
  ],
  discovery: {
    dxAdd: "dx add motion-animation --write",
    controls: "MotionControlledStatus",
    controlsHook: "useDxAnimationControls",
    rawAnimationControlsHook: "useDxRawAnimationControls",
    legacyAnimationHook: "useDxLegacyAnimation",
    animationControlsFactory: "createDxAnimationControls",
    frameTicker: "MotionFrameTicker",
    frameClockHook: "useDxFrameClock",
    layoutGroup: "DxMotionLayoutGroup",
    layoutItem: "MotionLayoutItem",
    layoutIdHelper: "dxMotionLayoutId",
    instantLayoutTransitionHook: "useDxInstantLayoutTransition",
    lazyProvider: "DxLazyMotionProvider",
    lazyBox: "MotionLazyBox",
    lazyMotionProxy: "dxLazyMotion",
    motionValueMeter: "MotionValueMeter",
    motionValueMeterHook: "useDxMotionValueMeter",
    pageVisibilityBadge: "MotionPageVisibilityBadge",
    pageVisibilityHook: "useDxPageVisibility",
    provider: "DxMotionProvider",
    presence: "DxMotionPresence",
    presenceItem: "MotionPresenceItem",
    presenceHook: "useDxPresence",
    reorderGroup: "DxReorderGroup",
    reorderItem: "DxReorderItem",
    reorderControlsHook: "useDxReorderControls",
    component: "MotionReveal",
    presetHelper: "dxMotionPreset",
    staggerHelper: "dxMotionStaggerContainer",
    scopedAnimateHook: "useDxScopedAnimate",
    pressFeedbackHook: "useDxMotionPressFeedback",
    scrollProgressComponent: "MotionScrollProgress",
    scrollProgressHook: "useDxScrollProgress",
    willChangeBox: "MotionWillChangeBox",
    willChangeHook: "useDxWillChange",
    willChangeFactory: "createDxWillChangeValue",
  },
  dashboardWorkflow: {
    sourceRoute: "tools/launch/runtime-template/pages/index.html",
    runtimeScript: "tools/launch/runtime-template/assets/launch-runtime.ts",
    runtimeStyles: "tools/launch/runtime-template/assets/launch-runtime.css",
    packageMarker: "data-dx-package=\"animation/motion\"",
    componentMarker: "data-dx-component=\"motion-animation-card\"",
    dashboardSummaryMarker: "data-dx-component=\"launch-motion-dashboard-summary\"",
    workflowMarker: "data-dx-dashboard-workflow=\"motion-panel-orchestration\"",
    reducedMotionMarker: "data-dx-motion-reduced",
    productSurface: "launch-dashboard",
    receiptPath: "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
    interactions: ["advance-stage", "reverse-order", "move-stage-previous", "move-stage-next", "keyboard-reorder", "persist-preference", "reset-proof", "toggle-reduced-motion"],
    styleTokens: [
      "--dx-motion-active-border",
      "--dx-motion-active-bg",
      "--dx-motion-progress-track",
    ],
    materializedGuard: "dx run --test .\\benchmarks\\motion-launch-materialized.test.ts",
    interactionGuard: "dx run --test .\\benchmarks\\motion-runtime-interaction.test.ts",
    studioSurface: "motion-interaction-proof",
    studioSurfaces: ["motion-dashboard-workflow", "motion-interaction-proof"],
    studioManifestMarkers: [
      "data-dx-motion-interaction",
      "data-dx-motion-reduced",
      "data-dx-motion-state",
      "data-dx-motion-progress",
      "data-dx-motion-order",
      "data-dx-motion-order-available",
      "data-dx-motion-keyboard-reorder",
      "data-dx-motion-keyboard-state",
      "data-dx-motion-preference-storage",
      "data-dx-motion-storage-key",
    ],
  },
  dashboardUsage: {
    starterComponent: "examples/dashboard/src/components/MotionDashboardWorkflow.tsx",
    helperModel: "examples/dashboard/src/lib/motionDashboardWorkflow.ts",
    launchComponent: "examples/template/template-shell.tsx",
    proofComponent: "examples/template/motion-interaction-proof.tsx",
    launchWorkflow: "motion-panel-orchestration",
    workflow: "animated-readiness",
    dxIcon: "pack:motion",
    actions: ["select-stage", "reverse-order", "move-stage-previous", "move-stage-next", "keyboard-reorder", "persist-preference", "toggle-reduced-motion", "prepare-motion-receipt"],
    sourceGuard: "dx run --test .\\benchmarks\\motion-dashboard-workflow.test.ts",
  },
} as const;

export type DxMotionForgePackageMetadata = typeof dxMotionForgePackage;
"#;

const MOTION_README_MD: &str = r#"# DX Forge Motion & Animation Slice

This package materializes a small source-owned adapter around the real `motion` 12 React API. It does not reimplement Motion, fake animation state, or run package-manager lifecycle scripts.

Official DX package name: Motion & Animation. Package id: `animation/motion`. Official CLI: `dx add motion-animation --write`. Upstream package metadata stays recorded as `motion` from `G:\WWW\inspirations\motion`.

## Owned Files

- `motion/presets.ts` defines reusable launch transitions, viewport defaults, variants, and stagger helpers.
- `motion/controls.tsx` exposes `useDxAnimationControls`, `MotionControlledStatus`, raw hook aliases, and `createDxAnimationControls` over Motion's real `useAnimationControls`, `useAnimation`, `animationControls`, and `LegacyAnimationControls` APIs.
- `motion/frame.tsx` exposes `useDxFrameClock` and `MotionFrameTicker` over Motion's real `useAnimationFrame`, `useTime`, `useCycle`, and `MotionValue` APIs for lightweight launch heartbeat displays.
- `motion/layout.tsx` exposes `DxMotionLayoutGroup`, `MotionLayoutItem`, `dxMotionLayoutId`, and `useDxInstantLayoutTransition` over Motion's real `LayoutGroup`, `layoutId`, `layoutDependency`, `layoutRoot`, and `useInstantLayoutTransition` APIs.
- `motion/lazy.tsx` exposes `DxLazyMotionProvider`, `MotionLazyBox`, and `dxLazyMotion` over Motion's real `LazyMotion`, `domAnimation`, `domMax`, `domMin`, and `m` feature-bundle APIs.
- `motion/motion-values.tsx` exposes `useDxMotionValueMeter` and `MotionValueMeter` over Motion's real `useMotionValue`, `useSpring`, `useTransform`, `useMotionTemplate`, `useMotionValueEvent`, and `useVelocity` APIs.
- `motion/page-visibility.tsx` exposes `useDxPageVisibility` and `MotionPageVisibilityBadge` over Motion's real `usePageInView` document-visibility API.
- `motion/provider.tsx` exposes `DxMotionProvider`, a small policy boundary over Motion's real `MotionConfig` provider for default transitions, `reducedMotion`, CSP `nonce`, and `skipAnimations`.
- `motion/presence.tsx` exposes `DxMotionPresence`, `MotionPresenceItem`, and `useDxPresence` over Motion's real `AnimatePresence`, `LayoutGroup`, `usePresence`, and `useIsPresent` APIs.
- `motion/reorder.tsx` exposes `DxReorderGroup`, `DxReorderItem`, and `useDxReorderControls` over Motion's real `Reorder` namespace and `useDragControls` hook.
- `motion/reveal.tsx` provides a reduced-motion-safe reveal component backed by `motion`, `useInView`, and `useReducedMotion` from `motion/react`.
- `motion/scoped-animate.tsx` exposes reduced-motion-aware helpers around the real `useAnimate` scoped animation API and its playback controls.
- `motion/scroll-progress.tsx` exposes a top-level progress component and hook backed by Motion's real `useScroll`, `useSpring`, and `MotionValue` contracts.
- `motion/will-change.tsx` exposes `useDxWillChange`, `MotionWillChangeBox`, and `createDxWillChangeValue` over Motion's real `useWillChange` and `WillChangeMotionValue` performance-hint APIs.
- `motion/dashboard-workflow.ts` exposes `motionDashboardPackage`, `motionDashboardStages`, `getMotionDashboardStage`, `moveMotionDashboardStage`, and `createMotionDashboardReceipt` for starter dashboards that need source-owned Motion policy receipts and button-based order previews without claiming app-owned choreography.
- `motion/metadata.ts` gives DX CLI, Zed, and launch templates a stable discovery record.

## dx-check visibility

The dashboard workflow receipt carries `dx.forge.package.dx_check_visibility` with the current `present` source state plus the status legend `present`, `stale`, `missing-receipt`, `blocked`, and `unsupported-surface`. DX-WWW, dx-check, and Zed should use that receipt field to show whether the selected Motion & Animation surfaces and receipts are present, stale, blocked by governed runtime proof, or outside the selected upstream-backed surface set.

## Required App Dependency

Install or provide `motion` and React in the app runtime. Forge owns these adapter files and receipts; it does not vendor the Motion engine.

## Launch Runtime Proof

- Runtime-safe source route: `tools/launch/runtime-template/pages/index.html`.
- Source-owned receipt: `examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json`.
- Visible markers: `data-dx-package="animation/motion"`, `data-dx-component="launch-motion-dashboard-summary"`, `data-dx-component="motion-animation-card"`, `data-dx-dashboard-workflow="motion-panel-orchestration"`, `data-dx-motion-preference-storage="local-storage"`, `data-dx-motion-storage-key="dx.launch.motion.dashboard"`, and `data-dx-motion-reduced`.
- Safe local interactions: `advance-stage`, `reverse-order`, `move-stage-previous`, `move-stage-next`, Arrow/Home/End keyboard reorder, local preference persistence, `reset-proof`, and `toggle-reduced-motion` mutate visible state, progress, order markers, actionable order-availability markers, app-owned motion policy markers, and the runtime dashboard's Motion summary.
- Studio mapping: `motion-dashboard-workflow` and `motion-interaction-proof` map the visible Motion controls to `template-shell.tsx` and `motion-interaction-proof.tsx` through `data-dx-motion-interaction`, `data-dx-motion-reduced`, and the dashboard receipt path.
- Guards: `dx run --test .\benchmarks\motion-launch-materialized.test.ts` proves the generated no-`node_modules` `/launch` source keeps the Motion section, and `dx run --test .\benchmarks\motion-runtime-interaction.test.ts` executes the launch runtime script against a tiny DOM to check visible state changes.
- Styling uses DX theme tokens such as `--dx-motion-active-border`, `--dx-motion-active-bg`, and `--dx-motion-progress-track`; the app owns final density, choreography, and motion-budget policy.
- Component defaults use app-styleable classes such as `dx-motion-meter-track`, `dx-motion-meter-indicator`, and `dx-motion-scroll-progress` instead of hardcoded brand colors.

## Dashboard Starter Workflow

- Visible starter surface: `examples/dashboard/src/components/MotionDashboardWorkflow.tsx`.
- Source-owned model: `motion/dashboard-workflow.ts`, mirrored by `examples/dashboard/src/lib/motionDashboardWorkflow.ts` for the starter.
- Stable markers: `data-dx-package="animation/motion"`, `data-dx-component="dashboard-motion-workflow"`, `data-dx-motion-dashboard-workflow="animated-readiness"`, `data-dx-motion-preference-storage`, and `data-dx-motion-storage-key`.
- Interactions: `select-stage`, `reverse-order`, `move-stage-previous`, `move-stage-next`, keyboard reorder, local preference persistence, `toggle-reduced-motion`, and `prepare-motion-receipt` preview Motion policy and expose actionable order availability without claiming app-owned route choreography.
- Icon contract: `<dx-icon name="pack:motion" />`.
- Guard: `dx run --test .\benchmarks\motion-dashboard-workflow.test.ts` proves the dashboard imports and renders the workflow.

## Template Usage

```tsx
import { MotionControlledStatus } from "@/motion/controls";
import { MotionFrameTicker } from "@/motion/frame";
import { MotionWillChangeBox } from "@/motion/will-change";
import { MotionReveal } from "@/motion/reveal";
import {
  DxMotionLayoutGroup,
  MotionLayoutItem,
  dxMotionLayoutId,
  useDxInstantLayoutTransition,
} from "@/motion/layout";
import { DxLazyMotionProvider, MotionLazyBox } from "@/motion/lazy";
import { MotionValueMeter } from "@/motion/motion-values";
import { MotionPageVisibilityBadge } from "@/motion/page-visibility";
import { DxMotionProvider } from "@/motion/provider";
import { DxMotionPresence, MotionPresenceItem } from "@/motion/presence";
import { DxReorderGroup, DxReorderItem, useDxReorderControls } from "@/motion/reorder";
import { MotionScrollProgress } from "@/motion/scroll-progress";
import { useDxMotionPressFeedback } from "@/motion/scoped-animate";

export function LaunchMetric({ value, label }: { value: string; label: string }) {
  const instantLayoutTransition = useDxInstantLayoutTransition();
  const motionPress = useDxMotionPressFeedback();

  return (
    <DxMotionProvider reducedMotion="user">
      <DxLazyMotionProvider features="animation">
      <MotionControlledStatus
        active={true}
        data-dx-motion="launch-controlled-status"
        label="Launch package motion readiness"
      >
        <MotionLazyBox
          ref={motionPress.scope}
          data-dx-motion="launch-lazy-capability-row"
        >
        <MotionScrollProgress className="dx-motion-scroll-progress" />
        <DxMotionLayoutGroup id="launch-metric-layout">
          <DxMotionPresence layoutGroupId="launch-metric">
            <DxReorderGroup
              values={["metric"]}
              onReorder={() => instantLayoutTransition()}
            >
              <DxReorderItem value="metric" dragListener={false}>
                <MotionPresenceItem>
                  <MotionLayoutItem
                    layoutId={dxMotionLayoutId("launch-metric", label)}
                    data-dx-motion="launch-package-layout"
                  >
                        <MotionReveal
                          className="grid gap-1 rounded-md border p-4"
                          data-dx-motion="pressable"
                      onMouseEnter={(event) => motionPress.press(event.currentTarget)}
                      preset="scale-in"
                    >
                      <strong className="text-2xl">{value}</strong>
                          <span className="text-sm text-neutral-500">{label}</span>
                                  <MotionWillChangeBox data-dx-motion="launch-will-change-metric">
                                    <MotionValueMeter
                                      data-dx-motion="launch-package-meter"
                                      label={`${label} coverage`}
                                      max={100}
                                      value={75}
                                    />
                                        <MotionFrameTicker
                                          data-dx-motion="launch-frame-ticker"
                                          sampleEveryMs={500}
                                        />
                                        <MotionPageVisibilityBadge
                                          data-dx-motion="launch-page-visibility"
                                        />
                                      </MotionWillChangeBox>
                            </MotionReveal>
                  </MotionLayoutItem>
                </MotionPresenceItem>
              </DxReorderItem>
            </DxReorderGroup>
          </DxMotionPresence>
        </DxMotionLayoutGroup>
        </MotionLazyBox>
      </MotionControlledStatus>
      </DxLazyMotionProvider>
    </DxMotionProvider>
  );
}

export function LaunchReorderHandle() {
  const reorder = useDxReorderControls({ ariaLabel: "Reorder launch package" });

  return (
    <button
      type="button"
      {...reorder.dragHandleProps}
      data-dx-motion="launch-package-drag-handle"
    >
      Reorder
    </button>
  );
}
```

Keep route-transition strategy, persistence, cross-list drag targets, governed keyboard accessibility QA, strict LazyMotion migration, imperative animation sequencing, frame sampling policy, page visibility policy, will-change performance-hint policy, and scroll-linked information architecture in application code. Use this slice as a launch-safe baseline for tasteful default motion, MotionConfig provider defaults, LazyMotion feature bundles, LayoutGroup, layoutId, layoutDependency, layoutRoot, useInstantLayoutTransition, MotionValue meters, page visibility badges, AnimatePresence, presence hooks, Reorder and useDragControls drag sorting, useAnimationControls, useAnimation, animationControls, LegacyAnimationControls, useAnimationFrame, useTime, useCycle, useWillChange, WillChangeMotionValue, useAnimate, scoped selector animation, useScroll, useSpring, and MotionValue scroll progress.
"#;
