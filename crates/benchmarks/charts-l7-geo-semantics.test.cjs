const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const projectRoot = path.join(repoRoot, "examples", "charts");

function read(relativePath) {
  return fs.readFileSync(path.join(projectRoot, relativePath), "utf8");
}

test("charts L7 and L7Plot slices own layer style and interaction semantics", () => {
  const spec = read("lib/charts/spec.ts");
  const geoModel = read("lib/charts/compiler/geo-model.ts");
  const spatial = read("lib/charts/compiler/spatial.ts");
  const l7 = read("lib/charts/presets/l7.ts");
  const l7plot = read("lib/charts/presets/l7plot.ts");
  const gallery = read("lib/charts/gallery.ts");

  for (const marker of ["GeoLayerStyleSpec", "GeoLayerRuntimeSpec", "GeoLegendSpec", "style?: GeoLayerStyleSpec", "interactions?: GeoInteractionSpec[]", "legends?: GeoLegendSpec[]"]) {
    assert.match(spec, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of ["layerStyle: GeoLayerStyleSpec", "layerInteractions: GeoInteractionSpec[]", "normalizeGeoLayerStyle", "normalizeLayerInteractions", "filterGeoLayersByViewport", "normalizeGeoLegends", "layerZoomMetadata", "layerLegendMetadata", "viewportMetadata", "projectionMetadata"]) {
    assert.match(geoModel, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of ["featureMapMetadata", "featureClassName", "feature.layerStyle.fill", "feature.layerInteractions.map", "mapViewport: viewportMetadata(model.viewport)", "mapProjection: projectionMetadata(model.projection)", "mapLayerZoom: layerZoomMetadata(feature)", "mapLegend: layerLegendMetadata(feature.legend)", "return scene(spec, elements, legend)"]) {
    assert.match(spatial, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of ["style: { fill:", "style: { fill: \"hsl(var(--chart-info))\"", "interactions: [{ type: \"tooltip\" }, { type: \"select-feature\" }]", "heatmapLayer?: GeoLayerRuntimeSpec", "bubbleLayer?: GeoLayerRuntimeSpec", "legends: config.legends ?? [{ channel: \"size\""]) {
    assert.match(l7, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of ["style: config.style ?? { fill:", "style: config.style ?? { fill: \"hsl(var(--chart-success))\"", "style: config.style ?? { fill: \"hsl(var(--chart-warning))\"", "const layerInteractions = config.interactions ?? [{ type: \"tooltip\" }, { type: \"select-feature\" }]", "layerOptions?: GeoLayerRuntimeSpec", "legends: config.legends ??"]) {
    assert.match(l7plot, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(gallery, /viewport: \{ center: \[90, 24\], zoom: 1\.25, minZoom: 0, maxZoom: 8, pitch: 8, bearing: -6 \}/);
  assert.match(gallery, /heatmapLayer: \{ minZoom: 0, maxZoom: 2\.4/);
  assert.match(gallery, /layerOptions: \{ minZoom: 0, maxZoom: 7/);
  assert.match(gallery, /basemap: \{ type: "token-land", labels: true \}/);
});
