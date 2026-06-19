import type { ChartPrimitive } from "./spec";
import { toLabel, toNumber } from "./format";

export interface ContinuousScale {
  kind: "linear";
  map(value: ChartPrimitive): number;
  ticks(count?: number): number[];
}

export interface BandScale {
  kind: "band";
  map(value: ChartPrimitive): number;
  ticks(): string[];
  bandwidth(): number;
}

export interface PointScale {
  kind: "point";
  map(value: ChartPrimitive): number;
  ticks(): string[];
}

export interface OrdinalScale {
  kind: "ordinal";
  map(value: ChartPrimitive): string;
  ticks(): string[];
}

export type ChartScale = ContinuousScale | BandScale | PointScale | OrdinalScale;

export const CHART_PALETTE = [
  "hsl(var(--chart-blue))",
  "hsl(var(--chart-green))",
  "hsl(var(--chart-amber))",
  "hsl(var(--chart-rose))",
  "hsl(var(--chart-violet))",
  "hsl(var(--chart-cyan))",
  "hsl(var(--chart-lime))",
  "hsl(var(--chart-orange))",
] as const;

export function createLinearScale(domain: [number, number], range: [number, number]): ContinuousScale {
  const [domainMin, domainMax] = domain;
  const [rangeMin, rangeMax] = range;
  const span = domainMax - domainMin || 1;

  return {
    kind: "linear",
    map(value) {
      const ratio = (toNumber(value) - domainMin) / span;
      return rangeMin + ratio * (rangeMax - rangeMin);
    },
    ticks(count = 5) {
      const steps = Math.max(1, count - 1);
      return Array.from({ length: count }, (_, index) => domainMin + ((domainMax - domainMin) * index) / steps);
    },
  };
}

export function createBandScale(domain: string[], range: [number, number], padding = 0.18): BandScale {
  const [rangeMin, rangeMax] = range;
  const count = Math.max(1, domain.length);
  const outer = (rangeMax - rangeMin) * padding;
  const step = (rangeMax - rangeMin - outer) / count;
  const band = step * (1 - padding);
  const offset = outer / 2 + (step - band) / 2;

  return {
    kind: "band",
    map(value) {
      const index = Math.max(0, domain.indexOf(toLabel(value)));
      return rangeMin + offset + index * step;
    },
    ticks() {
      return domain;
    },
    bandwidth() {
      return Math.max(1, band);
    },
  };
}

export function createPointScale(domain: string[], range: [number, number]): PointScale {
  const [rangeMin, rangeMax] = range;
  const count = Math.max(1, domain.length - 1);

  return {
    kind: "point",
    map(value) {
      const index = Math.max(0, domain.indexOf(toLabel(value)));
      return domain.length <= 1 ? (rangeMin + rangeMax) / 2 : rangeMin + ((rangeMax - rangeMin) * index) / count;
    },
    ticks() {
      return domain;
    },
  };
}

export function createOrdinalScale(domain: string[]): OrdinalScale {
  return {
    kind: "ordinal",
    map(value) {
      const index = Math.max(0, domain.indexOf(toLabel(value)));
      return CHART_PALETTE[index % CHART_PALETTE.length];
    },
    ticks() {
      return domain;
    },
  };
}
