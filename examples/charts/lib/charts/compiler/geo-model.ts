import { readField, toLabel, toNumber } from "../format";
import type {
  ChartSpec,
  Datum,
  FieldEncoding,
  GeoBasemapSpec,
  GeoInteractionSpec,
  GeoLayerBlendMode,
  GeoLayerSpec,
  GeoLayerStateSpec,
  GeoLayerStyleSpec,
  GeoLegendSpec,
  GeoProjectionKind,
  GeoRegionSpec,
  GeoViewportSpec,
  MarkSpec,
} from "../spec";
import type { PlotBounds } from "./shared";
import { sceneId } from "./shared";

export interface NormalizedGeoLayer {
  id: string;
  type: GeoLayerSpec["type"];
  encoding: Required<Pick<Partial<Record<"x" | "y", FieldEncoding>>, "x" | "y">> & Partial<GeoLayerSpec["encoding"]>;
  regionField?: string;
  layerStyle: GeoLayerStyleSpec;
  layerInteractions: GeoInteractionSpec[];
  minZoom?: number;
  maxZoom?: number;
  autoFit: boolean;
  blend: GeoLayerBlendMode;
  layerState: GeoLayerStateSpec;
  legend?: GeoLegendSpec;
  visibleAtZoom: boolean;
  zIndex: number;
}

export interface GeoFeature {
  id: string;
  layerId: string;
  layerType: GeoLayerSpec["type"];
  x: number;
  y: number;
  radius: number;
  value: number;
  label: string;
  datum: Datum;
  layerStyle: GeoLayerStyleSpec;
  layerInteractions: GeoInteractionSpec[];
  minZoom?: number;
  maxZoom?: number;
  autoFit: boolean;
  blend: GeoLayerBlendMode;
  layerState: GeoLayerStateSpec;
  legend?: GeoLegendSpec;
  visibleAtZoom: boolean;
  points?: Array<{ x: number; y: number }>;
}

export interface GeoModel {
  projection: GeoProjectionKind;
  viewport: GeoViewportSpec;
  basemap: GeoBasemapSpec;
  interactions: GeoInteractionSpec[];
  layers: NormalizedGeoLayer[];
  legends: GeoLegendSpec[];
  features: GeoFeature[];
}

const DEFAULT_BASEMAP: GeoBasemapSpec = { type: "token-land", labels: true };
const DEFAULT_INTERACTIONS: GeoInteractionSpec[] = [{ type: "tooltip" }];
const DEFAULT_LAYER_STYLES: Record<GeoLayerSpec["type"], GeoLayerStyleSpec> = {
  point: { fill: "hsl(var(--chart-success))", stroke: "hsl(var(--chart-stroke))", opacity: 0.9, radiusScale: 1, className: "chart-map-point" },
  bubble: { fill: "hsl(var(--chart-info))", stroke: "hsl(var(--chart-stroke))", opacity: 0.82, radiusScale: 1, className: "chart-map-bubble" },
  heatmap: { fill: "hsl(var(--chart-info))", opacity: 0.36, radiusScale: 1.28, className: "chart-map-heat" },
  region: { fill: "hsl(var(--chart-warning))", stroke: "hsl(var(--chart-stroke))", opacity: 0.84, strokeWidth: 1, radiusScale: 1, className: "chart-map-choropleth" },
};

export function buildGeoModel(spec: ChartSpec, mark: MarkSpec, data: Datum[], bounds: PlotBounds): GeoModel {
  const projection = spec.map?.projection ?? "equirectangular";
  const interactions = (spec.map?.interactions ?? DEFAULT_INTERACTIONS).filter((interaction) => interaction.enabled !== false);
  const allLayers = normalizeGeoLayers(spec, mark, interactions);
  const viewport = normalizeViewport(spec.map?.viewport ?? fitViewport(data, allLayers[0]?.encoding.x, allLayers[0]?.encoding.y));
  const layers = filterGeoLayersByViewport(allLayers, viewport);
  const basemap = spec.map?.basemap ?? DEFAULT_BASEMAP;
  const features = layers.flatMap((layer) => featuresForLayer(layer, data, bounds, viewport, projection, spec.map?.regions ?? []));
  const legends = normalizeGeoLegends(spec.map?.legends ?? []);

  return { projection, viewport, basemap, interactions, layers, legends, features };
}

export function normalizeGeoLayers(spec: ChartSpec, mark: MarkSpec, mapInteractions: GeoInteractionSpec[] = DEFAULT_INTERACTIONS): NormalizedGeoLayer[] {
  const declaredLayers = spec.map?.layers?.length ? spec.map.layers : [{ id: mark.id, type: "bubble" as const, encoding: mark.encoding }];
  return declaredLayers
    .filter((layer) => layer.visible !== false)
    .map((layer, index) => {
      const encoding = { ...mark.encoding, ...layer.encoding };
      if (!encoding.x || !encoding.y) {
        throw new Error(`Geo layer ${layer.id} requires x and y encodings.`);
      }

      return {
        id: layer.id,
        type: layer.type,
        regionField: layer.regionField,
        layerStyle: normalizeGeoLayerStyle(layer),
        layerInteractions: normalizeLayerInteractions(layer, mapInteractions),
        minZoom: layer.minZoom,
        maxZoom: layer.maxZoom,
        autoFit: layer.autoFit === true,
        blend: layer.blend ?? "normal",
        layerState: normalizeLayerState(layer),
        legend: normalizeGeoLegend(layer.legend),
        visibleAtZoom: true,
        encoding: {
          ...encoding,
          x: encoding.x,
          y: encoding.y,
        },
        zIndex: layer.zIndex ?? index,
      };
    })
    .sort((left, right) => left.zIndex - right.zIndex);
}

export function filterGeoLayersByViewport(layers: NormalizedGeoLayer[], viewport: GeoViewportSpec): NormalizedGeoLayer[] {
  return layers
    .map((layer) => ({ ...layer, visibleAtZoom: layerVisibleAtZoom(layer, viewport.zoom) }))
    .filter((layer) => layer.visibleAtZoom);
}

export function normalizeGeoLayerStyle(layer: GeoLayerSpec): GeoLayerStyleSpec {
  return {
    ...DEFAULT_LAYER_STYLES[layer.type],
    ...(layer.style ?? {}),
  };
}

export function normalizeLayerInteractions(layer: GeoLayerSpec, mapInteractions: GeoInteractionSpec[]): GeoInteractionSpec[] {
  const source = layer.interactions ?? mapInteractions;
  return source.filter((interaction) => interaction.enabled !== false);
}

export function normalizeViewport(viewport: GeoViewportSpec): GeoViewportSpec {
  const minZoom = viewport.minZoom ?? 0;
  const maxZoom = viewport.maxZoom ?? 24;
  return {
    ...viewport,
    zoom: clamp(viewport.zoom, minZoom, maxZoom),
  };
}

export function normalizeGeoLegends(legends: GeoLegendSpec[]): GeoLegendSpec[] {
  return legends.map(normalizeGeoLegend).filter((legend): legend is GeoLegendSpec => Boolean(legend));
}

export function layerZoomMetadata(layer: Pick<NormalizedGeoLayer, "minZoom" | "maxZoom" | "visibleAtZoom">): string {
  return [
    `min:${formatZoom(layer.minZoom)}`,
    `max:${formatZoom(layer.maxZoom)}`,
    `visible:${String(layer.visibleAtZoom)}`,
  ].join(",");
}

export function layerStateMetadata(state: GeoLayerStateSpec): string | undefined {
  const entries = [
    state.active === undefined ? undefined : `active:${String(state.active)}`,
    state.selected === undefined ? undefined : `selected:${String(state.selected)}`,
  ].filter(Boolean);
  return entries.length > 0 ? entries.join(",") : undefined;
}

export function layerLegendMetadata(legend: GeoLegendSpec | undefined): string | undefined {
  if (!legend || legend.enabled === false) return undefined;
  return [
    `channel:${legend.channel}`,
    legend.title ? `title:${legend.title}` : undefined,
    legend.position ? `position:${legend.position}` : undefined,
  ].filter(Boolean).join(",");
}

export function fitViewport(data: Datum[], longitude: FieldEncoding | undefined, latitude: FieldEncoding | undefined): GeoViewportSpec {
  if (!longitude || !latitude) {
    return { center: [0, 0], zoom: 1 };
  }

  const longitudes = data.map((datum) => toNumber(readField(datum, longitude.field))).filter(Number.isFinite);
  const latitudes = data.map((datum) => toNumber(readField(datum, latitude.field))).filter(Number.isFinite);
  if (longitudes.length === 0 || latitudes.length === 0) {
    return { center: [0, 0], zoom: 1 };
  }

  const minLon = Math.min(...longitudes);
  const maxLon = Math.max(...longitudes);
  const minLat = Math.min(...latitudes);
  const maxLat = Math.max(...latitudes);
  const lonSpan = Math.max(1, maxLon - minLon);
  const latSpan = Math.max(1, maxLat - minLat);
  const zoom = clamp(Math.log2(300 / Math.max(lonSpan, latSpan * 1.6)), 0.75, 4);

  return {
    center: [(minLon + maxLon) / 2, (minLat + maxLat) / 2],
    zoom,
  };
}

export function projectGeoPoint(longitude: number, latitude: number, bounds: PlotBounds, viewport: GeoViewportSpec, projection: GeoProjectionKind): { x: number; y: number } {
  const scale = 2 ** viewport.zoom;
  const lonSpan = 360 / scale;
  const latSpan = 170 / scale;
  const [centerLon, centerLat] = viewport.center;
  const leftLon = centerLon - lonSpan / 2;
  const topLat = clamp(centerLat + latSpan / 2, -84, 84);
  const bottomLat = clamp(centerLat - latSpan / 2, -84, 84);
  const xRatio = (longitude - leftLon) / lonSpan;
  const yRatio = projection === "mercator-lite"
    ? mercatorRatio(latitude, topLat, bottomLat)
    : (topLat - latitude) / (topLat - bottomLat || 1);

  return {
    x: bounds.left + clamp(xRatio, 0, 1) * bounds.width,
    y: bounds.top + clamp(yRatio, 0, 1) * bounds.height,
  };
}

export function projectionMetadata(projection: GeoProjectionKind): string {
  return projection;
}

export function viewportMetadata(viewport: GeoViewportSpec): string {
  return [
    `center:${formatCoordinate(viewport.center[0])},${formatCoordinate(viewport.center[1])}`,
    `z:${formatCoordinate(viewport.zoom)}`,
    viewport.minZoom === undefined ? undefined : `min:${formatCoordinate(viewport.minZoom)}`,
    viewport.maxZoom === undefined ? undefined : `max:${formatCoordinate(viewport.maxZoom)}`,
    `pitch:${formatCoordinate(viewport.pitch ?? 0)}`,
    `bearing:${formatCoordinate(viewport.bearing ?? 0)}`,
  ].filter(Boolean).join(",");
}

function featuresForLayer(layer: NormalizedGeoLayer, data: Datum[], bounds: PlotBounds, viewport: GeoViewportSpec, projection: GeoProjectionKind, regions: GeoRegionSpec[]): GeoFeature[] {
  const regionById = new Map(regions.map((region) => [region.id, region]));

  return data.map((datum, index) => {
    const regionId = layer.regionField ? toLabel(readField(datum, layer.regionField)) : "";
    const region = regionId ? regionById.get(regionId) : undefined;
    const longitude = toNumber(readField(datum, layer.encoding.x.field));
    const latitude = toNumber(readField(datum, layer.encoding.y.field));
    const size = layer.encoding.size ? toNumber(readField(datum, layer.encoding.size.field)) : 8;
    const label = region?.label ?? (layer.encoding.label ? toLabel(readField(datum, layer.encoding.label.field)) : `${layer.id} ${index + 1}`);
    const projected = projectGeoPoint(longitude, latitude, bounds, viewport, projection);
    const points = region?.points.map(([pointLongitude, pointLatitude]) => projectGeoPoint(pointLongitude, pointLatitude, bounds, viewport, projection));

    return {
      id: sceneId("l7", layer.id, label || index),
      layerId: layer.id,
      layerType: layer.type,
      x: projected.x,
      y: projected.y,
      radius: radiusForLayer(layer.type, size) * (layer.layerStyle.radiusScale ?? 1),
      value: size,
      label,
      datum,
      layerStyle: layer.layerStyle,
      layerInteractions: layer.layerInteractions,
      minZoom: layer.minZoom,
      maxZoom: layer.maxZoom,
      autoFit: layer.autoFit,
      blend: layer.blend,
      layerState: layer.layerState,
      legend: layer.legend,
      visibleAtZoom: layer.visibleAtZoom,
      points,
    };
  });
}

function normalizeGeoLegend(legend: GeoLegendSpec | undefined): GeoLegendSpec | undefined {
  if (!legend || legend.enabled === false) return undefined;
  return {
    ...legend,
    position: legend.position ?? "bottom-left",
  };
}

function normalizeLayerState(layer: GeoLayerSpec): GeoLayerStateSpec {
  return {
    ...(layer.state ?? {}),
  };
}

function layerVisibleAtZoom(layer: Pick<NormalizedGeoLayer, "minZoom" | "maxZoom">, zoom: number): boolean {
  if (layer.minZoom !== undefined && zoom < layer.minZoom) return false;
  if (layer.maxZoom !== undefined && zoom > layer.maxZoom) return false;
  return true;
}

function radiusForLayer(type: GeoLayerSpec["type"], value: number): number {
  if (type === "heatmap") return clamp(Math.sqrt(value) * 2.8, 14, 34);
  if (type === "point") return 7;
  if (type === "region") return clamp(Math.sqrt(value) * 2.1, 18, 42);
  return clamp(Math.sqrt(value) * 1.6, 6, 24);
}

function mercatorRatio(latitude: number, topLatitude: number, bottomLatitude: number): number {
  const top = mercatorY(topLatitude);
  const bottom = mercatorY(bottomLatitude);
  return (top - mercatorY(latitude)) / (top - bottom || 1);
}

function mercatorY(latitude: number): number {
  const radians = (clamp(latitude, -84, 84) * Math.PI) / 180;
  return Math.log(Math.tan(Math.PI / 4 + radians / 2));
}

function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, value));
}

function formatCoordinate(value: number): string {
  return Number.isInteger(value) ? String(value) : value.toFixed(2).replace(/0+$/, "").replace(/\.$/, "");
}

function formatZoom(value: number | undefined): string {
  return value === undefined ? "default" : formatCoordinate(value);
}
