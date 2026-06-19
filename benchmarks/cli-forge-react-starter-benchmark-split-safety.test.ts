import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.join(__dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const starterBenchmarkPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_react_starter_benchmark.rs",
);

test("Forge react starter benchmark command wrapper lives outside cli mod.rs", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  const starterBenchmark = fs.readFileSync(starterBenchmarkPath, "utf8");
  const commandStart = cliMod.indexOf("fn cmd_forge_react_starter_benchmark");
  const nextCommandStart = cliMod.indexOf("fn cmd_forge_migration_guide", commandStart);
  assert.notEqual(commandStart, -1, "expected react starter benchmark command in cli module");
  assert.notEqual(nextCommandStart, -1, "expected next forge command after react starter benchmark");
  const commandBlock = cliMod.slice(commandStart, nextCommandStart);

  assert.match(cliMod, /^mod forge_react_starter_benchmark;$/m);
  assert.match(
    cliMod,
    /use forge_react_starter_benchmark::cmd_forge_react_starter_benchmark as run_forge_react_starter_benchmark_command;/,
  );
  assert.match(
    commandBlock,
    /run_forge_react_starter_benchmark_command\(&self\.cwd, args\)/,
  );

  assert.doesNotMatch(commandBlock, /let mut project: Option<PathBuf> = None/);
  assert.doesNotMatch(commandBlock, /Unknown forge react-starter-benchmark option/);
  assert.doesNotMatch(commandBlock, /Unexpected forge react-starter-benchmark path/);
  assert.doesNotMatch(commandBlock, /std::fs::write\(&output, &rendered\)/);
  assert.doesNotMatch(commandBlock, /DX Forge react-starter-benchmark score/);
  assert.doesNotMatch(commandBlock, /forge_react_starter_benchmark_failure_summary\(&report\)/);

  assert.match(
    starterBenchmark,
    /pub\(super\) fn cmd_forge_react_starter_benchmark\(/,
  );
  assert.match(starterBenchmark, /parse_score_threshold\(value\)\?/);
  assert.match(starterBenchmark, /Unknown forge react-starter-benchmark option/);
  assert.match(starterBenchmark, /Unexpected forge react-starter-benchmark path/);
  assert.match(starterBenchmark, /DX Forge react-starter-benchmark score/);
  assert.match(starterBenchmark, /forge_react_starter_benchmark_failure_summary\(&report\)/);
});
