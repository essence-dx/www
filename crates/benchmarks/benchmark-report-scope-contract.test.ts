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

test("benchmark reports redact external Next bundler runtime chunk URLs", () => {
  const generatorSources = [
    "benchmarks/measure-current-status.ts",
    "benchmarks/measure-real-routes.ts",
  ].map(read).join("\n");
  const reportSources = [
    "benchmarks/reports/current-status.json",
    "benchmarks/reports/latest.json",
    "benchmarks/reports/fair-counter-comparison.json",
    "benchmarks/reports/real-route-comparison.json",
  ].map(read).join("\n");

  assert.match(generatorSources, /redactExternalNextBundlerResource/);
  assert.match(generatorSources, /_next\\\/static\\\/chunks\\\/\[\^\/\]\+\\\.js/);
  assert.doesNotMatch(reportSources, /turbopack-[^"]+\.js/);
  assert.match(reportSources, /external-next-bundler-runtime-chunk/);
  assert.match(reportSources, /"url_redacted": true/);
  assert.match(reportSources, /"url_redaction_reason": "external-framework-baseline-runtime-asset"/);
});

test("benchmark scorecards do not keep DevTools clone or Turbopack adoption goals alive", () => {
  const scorecardReports = [
    "benchmarks/reports/framework-scorecard.json",
    "benchmarks/reports/framework-scorecard.md",
  ].map(read).join("\n");

  assert.doesNotMatch(
    scorecardReports,
    /Next DevTools|DevTools clone|devtools surface|nextjs_devtools_clone_target/i,
  );
  assert.doesNotMatch(
    scorecardReports,
    /Turbopack runtime\/build adoption|Turbopack powers|powers the build|turbopack_runtime_build_adoption/i,
  );
  assert.match(scorecardReports, /DX-owned dev feedback and diagnostics surface/);
});
