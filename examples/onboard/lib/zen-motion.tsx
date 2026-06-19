"use client";

import * as React from "react";

type NumericStyle = number | string;
type MotionStyleValue = NumericStyle | MotionValue<NumericStyle>;
type MotionStyle = React.CSSProperties & {
  x?: MotionStyleValue;
  y?: MotionStyleValue;
  scale?: MotionStyleValue;
  rotate?: MotionStyleValue;
};

type VariantStyle = MotionStyle | undefined;

export interface MotionValue<T = number> {
  get: () => T;
  set: (next: T) => void;
  onChange: (listener: (value: T) => void) => () => void;
}

class ZenMotionValue<T> implements MotionValue<T> {
  private value: T;
  private listeners = new Set<(value: T) => void>();

  constructor(initial: T) {
    this.value = initial;
  }

  get() {
    return this.value;
  }

  set(next: T) {
    this.value = next;
    this.listeners.forEach((listener) => listener(next));
  }

  onChange(listener: (value: T) => void) {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }
}

function isMotionValue(value: unknown): value is MotionValue {
  return Boolean(value && typeof value === "object" && "get" in value && "set" in value);
}

function resolveStyleValue(value: MotionStyleValue | undefined): NumericStyle | undefined {
  return isMotionValue(value) ? value.get() : value;
}

function toCssLength(value: NumericStyle | undefined): string | undefined {
  if (value === undefined) return undefined;
  return typeof value === "number" ? `${value}px` : value;
}

function transformFrom(style: MotionStyle): string | undefined {
  const transforms: string[] = [];
  const x = resolveStyleValue(style.x);
  const y = resolveStyleValue(style.y);
  const scale = resolveStyleValue(style.scale);
  const rotate = resolveStyleValue(style.rotate);
  if (x !== undefined) transforms.push(`translateX(${toCssLength(x)})`);
  if (y !== undefined) transforms.push(`translateY(${toCssLength(y)})`);
  if (scale !== undefined) transforms.push(`scale(${scale})`);
  if (rotate !== undefined) transforms.push(`rotate(${typeof rotate === "number" ? `${rotate}deg` : rotate})`);
  return transforms.length ? transforms.join(" ") : undefined;
}

function transitionFrom(transition: any): string {
  if (!transition) return "transform 180ms ease, opacity 180ms ease";
  const seconds =
    typeof transition.duration === "number"
      ? transition.duration
      : transition.type === "spring"
        ? 0.26
        : 0.18;
  return `transform ${seconds}s ease, opacity ${seconds}s ease, width ${seconds}s ease, height ${seconds}s ease, margin ${seconds}s ease, box-shadow ${seconds}s ease`;
}

function mergeMotionStyle(
  base: MotionStyle | undefined,
  variant: VariantStyle,
  transition: any,
): React.CSSProperties {
  const merged: MotionStyle = { ...(base ?? {}), ...(variant ?? {}) };
  const transform = [base?.transform, transformFrom(merged)].filter(Boolean).join(" ");
  const style: React.CSSProperties = { ...merged };
  delete (style as MotionStyle).x;
  delete (style as MotionStyle).y;
  delete (style as MotionStyle).scale;
  delete (style as MotionStyle).rotate;
  if (transform) style.transform = transform;
  style.transition = [style.transition, transitionFrom(transition)].filter(Boolean).join(", ");
  return style;
}

type MotionElementProps<T extends keyof JSX.IntrinsicElements> =
  React.ComponentPropsWithoutRef<T> & {
    initial?: VariantStyle | false;
    animate?: VariantStyle;
    exit?: VariantStyle;
    transition?: any;
    whileHover?: VariantStyle;
    whileTap?: VariantStyle;
    layout?: boolean;
    drag?: boolean | "x" | "y";
    dragConstraints?: unknown;
    dragElastic?: number;
    onDragStart?: (event: React.PointerEvent, info: { point: { x: number; y: number } }) => void;
    onDragEnd?: (
      event: React.PointerEvent,
      info: { offset: { x: number; y: number }; velocity: { x: number; y: number } },
    ) => void;
    style?: MotionStyle;
  };

function createMotionComponent<T extends keyof JSX.IntrinsicElements>(tag: T) {
  return React.forwardRef<Element, MotionElementProps<T>>(function MotionComponent(
    {
      initial,
      animate,
      transition,
      whileHover,
      whileTap,
      style,
      drag,
      onDragStart,
      onDragEnd,
      onPointerDown,
      onPointerUp,
      onPointerLeave,
      ...props
    },
    ref,
  ) {
    const [mounted, setMounted] = React.useState(initial === false);
    const [hovered, setHovered] = React.useState(false);
    const [pressed, setPressed] = React.useState(false);
    const [, forceUpdate] = React.useReducer((value) => value + 1, 0);
    const dragStartRef = React.useRef<{ x: number; y: number } | null>(null);

    React.useEffect(() => {
      setMounted(true);
    }, []);

    React.useEffect(() => {
      const unsubscribers: Array<() => void> = [];
      for (const value of Object.values(style ?? {})) {
        if (isMotionValue(value)) unsubscribers.push(value.onChange(forceUpdate));
      }
      return () => unsubscribers.forEach((unsubscribe) => unsubscribe());
    }, [style]);

    const variant = pressed && whileTap ? whileTap : hovered && whileHover ? whileHover : mounted ? animate : initial || undefined;
    const nextStyle = mergeMotionStyle(style, variant as VariantStyle, transition);
    const Comp = tag as any;

    return (
      <Comp
        ref={ref}
        style={nextStyle}
        onMouseEnter={(event: React.MouseEvent) => {
          setHovered(true);
          (props as any).onMouseEnter?.(event);
        }}
        onMouseLeave={(event: React.MouseEvent) => {
          setHovered(false);
          (props as any).onMouseLeave?.(event);
        }}
        onPointerDown={(event: React.PointerEvent) => {
          setPressed(true);
          if (drag) {
            dragStartRef.current = { x: event.clientX, y: event.clientY };
            onDragStart?.(event, { point: { x: event.clientX, y: event.clientY } });
          }
          onPointerDown?.(event);
        }}
        onPointerUp={(event: React.PointerEvent) => {
          setPressed(false);
          if (drag && dragStartRef.current) {
            onDragEnd?.(event, {
              offset: {
                x: event.clientX - dragStartRef.current.x,
                y: event.clientY - dragStartRef.current.y,
              },
              velocity: { x: 0, y: 0 },
            });
            dragStartRef.current = null;
          }
          onPointerUp?.(event);
        }}
        onPointerLeave={(event: React.PointerEvent) => {
          setPressed(false);
          onPointerLeave?.(event);
        }}
        {...props}
      />
    );
  });
}

export const motion = {
  div: createMotionComponent("div"),
  span: createMotionComponent("span"),
  button: createMotionComponent("button"),
  form: createMotionComponent("form"),
  section: createMotionComponent("section"),
  article: createMotionComponent("article"),
  header: createMotionComponent("header"),
  footer: createMotionComponent("footer"),
};

export function AnimatePresence({ children }: { children: React.ReactNode }) {
  return <>{children}</>;
}

export function useMotionValue<T extends NumericStyle>(initial: T): MotionValue<T> {
  const ref = React.useRef<MotionValue<T> | null>(null);
  if (!ref.current) ref.current = new ZenMotionValue(initial);
  return ref.current;
}

export function animate<T extends NumericStyle>(
  value: MotionValue<T>,
  target: T,
  _options?: unknown,
) {
  value.set(target);
  return { stop() {} };
}

export function useTransform(
  value: MotionValue<number>,
  input: number[] | ((value: number) => number),
  output?: number[],
): MotionValue<number> {
  const transformed = useMotionValue(
    typeof input === "function" ? input(value.get()) : interpolate(value.get(), input, output ?? input),
  );

  React.useEffect(
    () =>
      value.onChange((next) => {
        transformed.set(
          typeof input === "function" ? input(next) : interpolate(next, input, output ?? input),
        );
      }),
    [value, input, output, transformed],
  );

  return transformed;
}

export function useSpring(value: MotionValue<number> | number): MotionValue<number> {
  return typeof value === "number" ? useMotionValue(value) : value;
}

function interpolate(value: number, input: number[], output: number[]) {
  if (input.length < 2 || output.length < 2) return output[0] ?? value;
  const start = input[0];
  const end = input[input.length - 1];
  const outStart = output[0];
  const outEnd = output[output.length - 1];
  if (end === start) return outStart;
  const progress = Math.max(0, Math.min(1, (value - start) / (end - start)));
  return outStart + (outEnd - outStart) * progress;
}

export type MotionProps = Record<string, unknown>;
