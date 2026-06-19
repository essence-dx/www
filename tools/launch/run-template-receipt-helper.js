#!/usr/bin/env node

const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");

const repoRoot = path.resolve(__dirname, "..", "..");

function usage() {
  return [
    "Usage: node tools/launch/run-template-receipt-helper.js <helper.ts> [...args]",
    "",
    "Runs a template receipt hash helper from the repository root.",
    "Use this entrypoint from DX/Zed/check surfaces so module-shaped template",
    "projects can keep source-owned receipt commands stable.",
  ].join("\n");
}

function fail(message) {
  process.stderr.write(`${message}\n`);
  process.stderr.write(`${usage()}\n`);
  process.exit(2);
}

function resolveInsideRepo(relativePath) {
  const normalized = String(relativePath || "").replace(/\\/g, "/");
  if (!normalized || path.isAbsolute(normalized) || normalized.includes("..")) {
    fail(`Receipt helper path must stay inside this repository: ${relativePath}`);
  }

  const absolutePath = path.resolve(repoRoot, normalized);
  const relativeToRoot = path.relative(repoRoot, absolutePath);
  if (relativeToRoot.startsWith("..") || path.isAbsolute(relativeToRoot)) {
    fail(`Receipt helper path escapes this repository: ${relativePath}`);
  }
  return absolutePath;
}

const [rawHelperArg, ...helperArgs] = process.argv.slice(2);
const helperArg = rawHelperArg ? rawHelperArg.replace(/\\/g, "/") : rawHelperArg;

if (helperArg === "--help" || helperArg === "-h") {
  process.stdout.write(`${usage()}\n`);
  process.exit(0);
}

if (!helperArg) {
  fail("Missing receipt helper path.");
}

if (!helperArg.startsWith("examples/template/")) {
  fail("Receipt helper must live under examples/template/.");
}

if (!helperArg.endsWith("-receipt-hashes.ts")) {
  fail("Receipt helper must be a *-receipt-hashes.ts file.");
}

const helperPath = resolveInsideRepo(helperArg);
if (!fs.existsSync(helperPath)) {
  fail(`Receipt helper not found: ${helperArg}`);
}

const result = spawnSync(process.execPath, [helperPath, ...helperArgs], {
  cwd: repoRoot,
  encoding: "utf8",
  env: process.env,
});

if (result.stdout) {
  process.stdout.write(result.stdout);
}
if (result.stderr) {
  process.stderr.write(result.stderr);
}
if (result.error) {
  process.stderr.write(`${result.error.message}\n`);
  process.exit(1);
}

process.exit(result.status ?? (result.signal ? 1 : 0));
