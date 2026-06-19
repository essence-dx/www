const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const vm = require("node:vm");

const repoRoot = path.resolve(__dirname, "..");
const projectRoot = path.join(repoRoot, "examples", "charts");

function read(relativePath) {
  return fs.readFileSync(path.join(projectRoot, relativePath), "utf8");
}

function extractFunction(source, name) {
  const exportStart = source.indexOf(`export function ${name}`);
  const start = exportStart >= 0 ? exportStart : source.indexOf(`function ${name}`);
  assert.notEqual(start, -1, `${name} function is missing`);

  const bodyStart = source.indexOf("{", start);
  let depth = 0;
  for (let index = bodyStart; index < source.length; index += 1) {
    const char = source[index];
    if (char === "{") depth += 1;
    if (char === "}") depth -= 1;
    if (depth === 0) return source.slice(start, index + 1);
  }

  throw new Error(`${name} function body is incomplete`);
}

function transpileHelper(source) {
  return source
    .replace(/^export\s+/, "")
    .replace(
      /function waterfallStages\(rows: Datum\[\], x: FieldEncoding, y: FieldEncoding, totalField\?: string\): WaterfallStage\[\]/,
      "function waterfallStages(rows, x, y, totalField)",
    )
    .replace(/function isWaterfallTotalValue\(value: ChartPrimitive\): boolean/, "function isWaterfallTotalValue(value)")
    .replace(/function isTotalStageLabel\(datum: Datum, labelField: string\): boolean/, "function isTotalStageLabel(datum, labelField)");
}

function loadWaterfallHelpers() {
  const source = read("lib/charts/compiler/waterfall.ts");
  const moduleSource = `
    function readField(datum, field) {
      return Object.prototype.hasOwnProperty.call(datum, field) ? datum[field] : null;
    }
    function toLabel(value) {
      if (value === null) return "";
      if (value instanceof Date) return value.toISOString();
      return String(value);
    }
    function toNumber(value) {
      if (typeof value === "number" && Number.isFinite(value)) return value;
      const parsed = Number(value);
      return Number.isFinite(parsed) ? parsed : 0;
    }
    ${transpileHelper(extractFunction(source, "isWaterfallTotalValue"))}
    ${transpileHelper(extractFunction(source, "isTotalStageLabel"))}
    ${transpileHelper(extractFunction(source, "waterfallStages"))}
    module.exports = { isWaterfallTotalValue, waterfallStages };
  `;
  const sandbox = { module: { exports: {} }, Date, Number, Object, String };
  vm.runInNewContext(moduleSource, sandbox, { filename: "waterfall-source-test.js" });
  return sandbox.module.exports;
}

test("waterfall total flags parse explicit false values without treating the last bar as total", () => {
  const { isWaterfallTotalValue, waterfallStages } = loadWaterfallHelpers();

  for (const value of [false, 0, "false", "0", "no", "n", "none", "null", "", null]) {
    assert.equal(isWaterfallTotalValue(value), false, `${String(value)} should not be a total flag`);
  }

  for (const value of [true, 1, "true", "1", "yes", "y", "total", "grand total"]) {
    assert.equal(isWaterfallTotalValue(value), true, `${String(value)} should be a total flag`);
  }

  const stages = waterfallStages(
    [
      { stage: "Initial scope", delta: 10, total: "false" },
      { stage: "Risk removed", delta: -3, total: "false" },
      { stage: "Follow-up work", delta: 2, total: "false" },
    ],
    { field: "stage", type: "ordinal" },
    { field: "delta", type: "quantitative" },
    "total",
  );

  assert.deepEqual(stages.map((stage) => stage.total), [false, false, false]);
  assert.deepEqual(stages.map((stage) => [stage.start, stage.end]), [[0, 10], [10, 7], [7, 9]]);
});

test("waterfall total semantics live in typed mark options instead of tooltip encoding", () => {
  const spec = read("lib/charts/spec.ts");
  const preset = read("lib/charts/presets/g2plot-waterfall.ts");
  const compiler = read("lib/charts/compiler/waterfall.ts");

  assert.match(spec, /export interface WaterfallMarkOptions/);
  assert.match(spec, /waterfall\?: WaterfallMarkOptions/);
  assert.match(preset, /waterfall: config\.totalField \? \{ totalField: config\.totalField \} : undefined/);
  assert.doesNotMatch(preset, /tooltip: config\.totalField/);
  assert.match(compiler, /mark\.waterfall\?\.totalField/);
  assert.doesNotMatch(compiler, /mark\.encoding\.tooltip/);
  assert.doesNotMatch(compiler, /stage\.total \|\| index === stages\.length - 1/);
});
