#!/usr/bin/env node

const {
  DEFAULT_COMPILE_TIMEOUT_MS,
  DEFAULT_DEV_START_TIMEOUT_MS,
  DEFAULT_HOST,
  DEFAULT_PORT,
  DEFAULT_ROUTE_RETRY_COUNT,
  DEFAULT_ROUTE_RETRY_DELAY_MS,
  DEFAULT_ROUTE_TIMEOUT_MS,
  DEFAULT_RUST_IDLE_WAIT_MS,
  parseArgs,
} = require("./readiness-gate/args.js");
const {
  DEFAULT_ROUTE_COUNT,
  buildCompileGateArgs,
  buildRouteSmokeArgs,
  routeSmokeProcessTimeoutMs,
} = require("./readiness-gate/commands.js");
const {
  EXPECTED_BROWSER_ROUTES,
  readBrowserProof,
} = require("./readiness-gate/browser-proof.js");
const { FORMAT, SCHEMA, writeReport } = require("./readiness-gate/report.js");
const {
  routeSmokeBlocker,
  runReadinessGate,
} = require("./readiness-gate/runner.js");

async function main(argv) {
  const options = parseArgs(argv);
  const report = await runReadinessGate(options);
  writeReport(options, report);
  process.exitCode = report.status === "passed" ? 0 : 1;
}

if (require.main === module) {
  main(process.argv.slice(2)).catch((error) => {
    process.stderr.write(`${error.message}\n`);
    process.exitCode = 2;
  });
}

module.exports = {
  DEFAULT_COMPILE_TIMEOUT_MS,
  DEFAULT_DEV_START_TIMEOUT_MS,
  DEFAULT_HOST,
  DEFAULT_PORT,
  DEFAULT_ROUTE_COUNT,
  DEFAULT_ROUTE_RETRY_COUNT,
  DEFAULT_ROUTE_RETRY_DELAY_MS,
  DEFAULT_ROUTE_TIMEOUT_MS,
  DEFAULT_RUST_IDLE_WAIT_MS,
  FORMAT,
  SCHEMA,
  buildCompileGateArgs,
  buildRouteSmokeArgs,
  EXPECTED_BROWSER_ROUTES,
  parseArgs,
  readBrowserProof,
  routeSmokeBlocker,
  routeSmokeProcessTimeoutMs,
  runReadinessGate,
};
