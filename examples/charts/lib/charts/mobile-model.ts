import type { ChartSpec, Datum, FieldEncoding, MobileChartSpec, MobileGestureSpec, MobileViewportSpec } from "./spec";

export const DEFAULT_F2_GESTURES: MobileGestureSpec[] = [
  { type: "tap-tooltip" },
  { type: "pan-x" },
  { type: "pinch-zoom" },
];

export type F2CompactChartConfig = {
  id: string;
  title: string;
  description: string;
  data: Datum[];
  xField: string;
  yField: string;
  xLabel?: string;
  yLabel?: string;
  gestures?: MobileGestureSpec[];
  viewport?: Partial<MobileViewportSpec>;
};

type MobileViewportInput = Partial<Omit<MobileChartSpec, "viewport">> & {
  viewport?: Partial<MobileViewportSpec>;
};

export function f2CompactChart(config: F2CompactChartConfig): ChartSpec {
  const mobile = normalizeMobileViewport({
    viewport: config.viewport,
    gestures: config.gestures,
    snap: "nearest",
  });
  const x: FieldEncoding = { field: config.xField, type: "ordinal", label: config.xLabel };
  const y: FieldEncoding = { field: config.yField, type: "quantitative", label: config.yLabel };

  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: "mobile",
    family: "F2",
    width: mobile.viewport.width,
    height: mobile.viewport.height,
    data: config.data,
    padding: { left: 34, right: 16, top: 18, bottom: 28 },
    mobile,
    marks: [
      { id: `${config.id}-area`, type: "area", encoding: { x, y }, style: { opacity: 0.18 } },
      { id: `${config.id}-line`, type: "line", encoding: { x, y }, style: { strokeWidth: 2 } },
      { id: `${config.id}-point`, type: "point", encoding: { x, y }, style: { radius: 4 } },
    ],
  };
}

export function normalizeMobileViewport(config: MobileViewportInput = {}): MobileChartSpec {
  const viewport = config.viewport ?? {};
  return {
    viewport: {
      width: viewport.width ?? 360,
      height: viewport.height ?? 220,
      pixelRatio: viewport.pixelRatio ?? 2,
      safeArea: viewport.safeArea ?? "compact",
    },
    gestures: config.gestures ?? DEFAULT_F2_GESTURES,
    snap: config.snap ?? "nearest",
  };
}

export function mobileSceneMetadata(mobile?: MobileChartSpec) {
  if (!mobile) {
    return {};
  }

  const viewport = normalizeMobileViewport(mobile).viewport;
  const gestures = (mobile.gestures ?? DEFAULT_F2_GESTURES)
    .filter((gesture) => gesture.enabled !== false)
    .map((gesture) => gesture.type);

  return {
    mobileViewport: `${viewport.width}x${viewport.height}`,
    mobileGestures: gestures.join(","),
    mobilePixelRatio: String(viewport.pixelRatio ?? 1),
    mobileSafeArea: viewport.safeArea ?? "none",
  };
}
