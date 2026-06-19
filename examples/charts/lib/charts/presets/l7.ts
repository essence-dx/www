import type { ChartSpec, Datum, GeoBasemapSpec, GeoInteractionSpec, GeoLayerRuntimeSpec, GeoLayerStyleSpec, GeoLegendSpec, GeoProjectionKind, GeoViewportSpec } from "../spec";

export type L7CompositeLayerPreset = {
  id: string;
  title: string;
  description: string;
  data: Datum[];
  xField: string;
  yField: string;
  valueField: string;
  labelField?: string;
  projection?: GeoProjectionKind;
  viewport?: GeoViewportSpec;
  basemap?: GeoBasemapSpec;
  interactions?: GeoInteractionSpec[];
  legends?: GeoLegendSpec[];
  heatmapLayer?: GeoLayerRuntimeSpec;
  bubbleLayer?: GeoLayerRuntimeSpec;
  heatmapStyle?: GeoLayerStyleSpec;
  bubbleStyle?: GeoLayerStyleSpec;
  width?: number;
  height?: number;
};

export function l7CompositeLayers(config: L7CompositeLayerPreset): ChartSpec {
  const encoding = {
    x: { field: config.xField, type: "quantitative" as const },
    y: { field: config.yField, type: "quantitative" as const },
    size: { field: config.valueField, type: "quantitative" as const },
    ...(config.labelField ? { label: { field: config.labelField, type: "nominal" as const } } : {}),
  };

  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: "map",
    family: "L7",
    width: config.width ?? 640,
    height: config.height ?? 380,
    data: config.data,
    map: {
      projection: config.projection ?? "mercator-lite",
      viewport: config.viewport ?? { center: [90, 24], zoom: 1.15 },
      basemap: config.basemap ?? { type: "token-land", labels: true },
      legends: config.legends ?? [{ channel: "size", title: "Usage", position: "bottom-right" }],
      layers: [
        {
          id: `${config.id}-heat`,
          type: "heatmap",
          encoding,
          minZoom: 0,
          maxZoom: 2.5,
          autoFit: true,
          blend: "max",
          state: { active: true },
          legend: { channel: "layer", title: "Heat density", position: "bottom-left" },
          ...(config.heatmapLayer ?? {}),
          style: { fill: "hsl(var(--chart-info))", opacity: 0.28, radiusScale: 1.35, className: "chart-map-heat", ...(config.heatmapStyle ?? {}) },
          interactions: [{ type: "tooltip" }, { type: "select-feature" }],
          zIndex: 0,
        },
        {
          id: `${config.id}-bubble`,
          type: "bubble",
          encoding,
          minZoom: 1,
          maxZoom: 8,
          autoFit: false,
          blend: "normal",
          state: { active: true, selected: true },
          legend: { channel: "layer", title: "Usage bubbles", position: "bottom-left" },
          ...(config.bubbleLayer ?? {}),
          style: { fill: "hsl(var(--chart-success))", stroke: "hsl(var(--chart-stroke))", opacity: 0.86, radiusScale: 1.08, className: "chart-map-bubble", ...(config.bubbleStyle ?? {}) },
          interactions: [{ type: "tooltip" }, { type: "select-feature" }],
          zIndex: 1,
        },
      ],
      interactions: config.interactions ?? [{ type: "pan" }, { type: "zoom" }, { type: "tooltip" }, { type: "select-feature" }],
    },
    marks: [{ id: `${config.id}-map`, type: "map", encoding }],
  };
}
