const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("G2 heatmap compiler owns size-channel cell geometry", () => {
  const cartesianMarks = read("examples/charts/lib/charts/compiler/cartesian-marks.ts");

  assert.match(cartesianMarks, /const size = mark\.encoding\.size;/);
  assert.match(cartesianMarks, /const sizeDomain = size \? extent\(data\.map\(\(datum\) => readField\(datum, size\.field\)\)\) : \[0, 1\] as \[number, number\];/);
  assert.match(cartesianMarks, /const sizeRatio = size \? heatmapSizeRatio\(readField\(datum, size\.field\), sizeDomain\) : 1;/);
  assert.match(cartesianMarks, /const drawWidth = cellWidth \* sizeRatio;/);
  assert.match(cartesianMarks, /const drawHeight = cellHeight \* sizeRatio;/);
  assert.match(cartesianMarks, /x: cellX \+ \(cellWidth - drawWidth\) \/ 2,/);
  assert.match(cartesianMarks, /y: cellY \+ \(cellHeight - drawHeight\) \/ 2,/);
  assert.match(cartesianMarks, /function heatmapSizeRatio\(value:/);
  assert.doesNotMatch(cartesianMarks, /width: xScale\.bandwidth\(\),\s*\n\s*height: yScale\.bandwidth\(\),/);
});

test("G2Plot heatmap preset lowers color and size fields into typed G2 channels", () => {
  const g2plotPreset = read("examples/charts/lib/charts/presets/g2plot.ts");

  assert.match(g2plotPreset, /export type G2PlotHeatmapPreset = G2PlotPresetBase & \{/);
  assert.match(g2plotPreset, /export function g2plotHeatmap\(config: G2PlotHeatmapPreset\): ChartSpec \{/);
  assert.match(g2plotPreset, /type: "heatmap",/);
  assert.match(g2plotPreset, /color: config\.colorField \? field\(config\.colorField, "quantitative", undefined, config\.valueFormat\) : undefined,/);
  assert.match(g2plotPreset, /size: config\.sizeField \? field\(config\.sizeField, "quantitative", undefined, config\.valueFormat\) : undefined,/);
  assert.match(g2plotPreset, /legend: config\.colorField \? \{ channel: "color", title: labelFor\(config\.colorField\) \} : undefined,/);
});
