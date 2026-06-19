import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

function sourceUnderTest(text) {
  return text
    .split(/\r?\n/)
    .filter((line) => !/assert\.doesNotMatch\(/.test(line))
    .join("\n");
}

function section(relativePath, heading) {
  const lines = read(relativePath).split(/\r?\n/);
  const start = lines.findIndex((line) => line === `## ${heading}`);
  assert.notEqual(start, -1, `expected ${relativePath} to contain ## ${heading}`);

  let end = lines.length;
  let sawContent = false;
  for (let index = start + 1; index < lines.length; index += 1) {
    if (sawContent && lines[index].trim() === "") {
      end = index;
      break;
    }

    if (/^#{1,2} /.test(lines[index])) {
      end = index;
      break;
    }

    if (lines[index].trim() !== "") {
      sawContent = true;
    }
  }

  return lines.slice(start, end).join("\n");
}

test("active launch notes remove Next DevTools and Turbopack runtime adoption targets", () => {
  const activeNotes = [
    section("DX.md", "Architecture Scope Reset"),
    section("TODO.md", "Architecture Scope Reset"),
    section("CHANGELOG.md", "Architecture Scope Reset"),
  ]
    .join("\n")
    .replace(/\s+/g, " ");

  assert.doesNotMatch(activeNotes, /DX-WWW DevTools|Next DevTools|next-devtools/);
  assert.doesNotMatch(activeNotes, /DevTools clone targets?/);
  assert.doesNotMatch(activeNotes, /\/_dx\/devtools/);
  assert.doesNotMatch(activeNotes, /dx-devtools-overlay-contract/);
  assert.doesNotMatch(activeNotes, /Turbopack execution (?:remain|was|proof|claimed|unclaimed)/);
  assert.doesNotMatch(activeNotes, /Turbopack execution is (?!not a DX-WWW goal)/);
  assert.doesNotMatch(activeNotes, /real Turbopack execution|Turbopack runtime proof|Turbopack-class HMR/);
  assert.doesNotMatch(activeNotes, /full Next\.js parity/);
  assert.match(activeNotes, /DX-owned WWW framework with Next-familiar authoring/);
  assert.match(activeNotes, /External framework debugging tool clone targets are out of scope/);
  assert.match(activeNotes, /Turbopack runtime\/build adoption is out of scope/);
  assert.match(activeNotes, /DX dev feedback/);
});

test("next-rust adapters stay reference-only and are not runtime adoption tests", () => {
  const taskTest = read("benchmarks/next-rust-task-input-adapter.test.ts");
  const sourceMapTest = read("benchmarks/next-rust-source-map-adapter.test.ts");
  const sourceMapAdapter = read("dx-www/src/next_rust_source_map_adapter.rs");

  for (const source of [sourceUnderTest(taskTest), sourceUnderTest(sourceMapTest), sourceMapAdapter]) {
    assert.doesNotMatch(source, /powers the build|runtime adoption target|Turbopack runtime proof/);
    assert.doesNotMatch(source, /real Turbopack runtime|real Turbopack build|Turbopack-class/);
  }

  assert.match(taskTest, /reference-only/);
  assert.match(sourceMapTest, /reference-only/);
  assert.match(sourceMapAdapter, /reference material only/);
  assert.match(sourceMapAdapter, /public_architecture: false/);
  assert.match(sourceMapAdapter, /turbopack_runtime_executed: false/);
  assert.match(sourceMapAdapter, /node_modules_required: false/);
});
