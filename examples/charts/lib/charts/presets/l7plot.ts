import type { ChartSpec, Datum, GeoBasemapSpec, GeoInteractionSpec, GeoLayerRuntimeSpec, GeoLayerStyleSpec, GeoLegendSpec, GeoProjectionKind, GeoRegionSpec, GeoViewportSpec } from "../spec";

type L7PlotBasePreset = {
  id: string;
  title: string;
  description: string;
  data: Datum[];
  projection?: GeoProjectionKind;
  viewport?: GeoViewportSpec;
  basemap?: GeoBasemapSpec;
  interactions?: GeoInteractionSpec[];
  legends?: GeoLegendSpec[];
  layerOptions?: GeoLayerRuntimeSpec;
  style?: GeoLayerStyleSpec;
  width?: number;
  height?: number;
};

export type L7PlotMapPreset = L7PlotBasePreset & {
  xField: string;
  yField: string;
  valueField?: string;
  labelField?: string;
  mapType?: "point" | "bubble";
};

export type L7PlotChoroplethPreset = L7PlotBasePreset & {
  xField: string;
  yField: string;
  regionField: string;
  valueField: string;
  regions?: GeoRegionSpec[];
};

export function l7plotMap(config: L7PlotMapPreset): ChartSpec {
  const size = { width: config.width ?? 640, height: config.height ?? 380 };
  const layerType = config.mapType ?? (config.valueField ? "bubble" : "point");
  const layerInteractions = config.interactions ?? [{ type: "tooltip" }, { type: "select-feature" }];
  const encoding = {
    x: { field: config.xField, type: "quantitative" as const },
    y: { field: config.yField, type: "quantitative" as const },
    ...(config.valueField ? { size: { field: config.valueField, type: "quantitative" as const } } : {}),
    ...(config.labelField ? { label: { field: config.labelField, type: "nominal" as const } } : {}),
  };

  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: "map",
    family: "L7Plot",
    ...size,
    data: config.data,
    map: {
      projection: config.projection ?? "mercator-lite",
      viewport: config.viewport ?? { center: [90, 24], zoom: 1.15 },
      basemap: config.basemap ?? { type: "graticule", labels: false },
      legends: config.legends ?? (config.valueField ? [{ channel: "size", title: labelFor(config.valueField), position: "bottom-right" }] : []),
      layers: [{
        id: `${config.id}-layer`,
        type: layerType,
        encoding,
        minZoom: 0,
        maxZoom: 8,
        autoFit: true,
        blend: "normal",
        state: { active: true },
        legend: { channel: "layer", title: layerType === "point" ? "Point layer" : "Bubble layer", position: "bottom-left" },
        ...(config.layerOptions ?? {}),
        style: config.style ?? { fill: "hsl(var(--chart-success))", stroke: "hsl(var(--chart-stroke))", opacity: layerType === "point" ? 0.92 : 0.78, radiusScale: layerType === "point" ? 1 : 1.16, className: layerType === "point" ? "chart-map-point" : "chart-map-bubble" },
        interactions: layerInteractions,
      }],
      interactions: config.interactions ?? [{ type: "pan" }, { type: "zoom" }, { type: "tooltip" }, { type: "select-feature" }],
    },
    marks: [{ id: `${config.id}-mark`, type: "map", encoding }],
  };
}

export function l7plotChoropleth(config: L7PlotChoroplethPreset): ChartSpec {
  const size = { width: config.width ?? 640, height: config.height ?? 380 };
  const layerInteractions = config.interactions ?? [{ type: "tooltip" }, { type: "select-feature" }];
  const encoding = {
    x: { field: config.xField, type: "quantitative" as const },
    y: { field: config.yField, type: "quantitative" as const },
    label: { field: config.regionField, type: "nominal" as const },
    size: { field: config.valueField, type: "quantitative" as const },
    color: { field: config.valueField, type: "quantitative" as const },
  };

  return {
    id: config.id,
    title: config.title,
    description: config.description,
    task: "map",
    family: "L7Plot",
    ...size,
    data: config.data,
    map: {
      projection: config.projection ?? "mercator-lite",
      viewport: config.viewport ?? { center: [88, 22], zoom: 1.2 },
      basemap: config.basemap ?? { type: "token-land", labels: true },
      regions: config.regions,
      legends: config.legends ?? [{ channel: "color", title: labelFor(config.valueField), position: "bottom-right" }],
      layers: [{
        id: `${config.id}-region`,
        type: "region",
        regionField: config.regionField,
        encoding,
        minZoom: 0,
        maxZoom: 7,
        autoFit: true,
        blend: "normal",
        state: { active: true, selected: true },
        legend: { channel: "layer", title: "Region polygons", position: "bottom-left" },
        ...(config.layerOptions ?? {}),
        style: config.style ?? { fill: "hsl(var(--chart-warning))", stroke: "hsl(var(--chart-stroke))", opacity: 0.78, strokeWidth: 1.2, className: "chart-map-choropleth" },
        interactions: layerInteractions,
      }],
      interactions: config.interactions ?? [{ type: "pan" }, { type: "zoom" }, { type: "tooltip" }, { type: "select-feature" }],
    },
    marks: [{ id: `${config.id}-mark`, type: "map", encoding }],
  };
}

function labelFor(field: string): string {
  return field
    .replace(/[-_]+/g, " ")
    .replace(/\b\w/g, (letter) => letter.toUpperCase());
}
