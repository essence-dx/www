#!/usr/bin/env node
"use strict";

const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const SCHEMA = "dx.style.liveTailwindOutputComparison";
const BASELINE = "tailwindcss@4.3.0";
const TAILWIND_CLI_PACKAGE = "@tailwindcss/cli@4.3.0";
const FIXTURE_PATH = "related-crates/style/fixtures/tailwind-equal-output-canary.json";
const RUN_POLICY =
  "Normal tests do not execute Tailwind. Pass --live or set DX_STYLE_RUN_LIVE_TAILWIND=1 to run npx --yes @tailwindcss/cli@4.3.0, whose package dependency pins tailwindcss@4.3.0. This does not add Tailwind as a dx-style dependency.";

function repoRoot(explicitRoot) {
  return explicitRoot ? path.resolve(explicitRoot) : path.resolve(__dirname, "..", "..");
}

function readFixture(root) {
  const fixtureAbsolutePath = path.join(root, FIXTURE_PATH);
  const fixture = JSON.parse(fs.readFileSync(fixtureAbsolutePath, "utf8"));

  if (fixture.schema !== "dx.style.tailwindEqualOutputCanary") {
    throw new Error(`Unexpected fixture schema: ${fixture.schema}`);
  }
  if (!Array.isArray(fixture.classes) || fixture.classes.length === 0) {
    throw new Error("Tailwind equal-output fixture has no classes");
  }

  return fixture;
}

function sourceInlineToken(className) {
  return className.replace(/\\/g, "\\\\").replace(/"/g, '\\"');
}

function buildTailwindInput(fixture) {
  const lines = [
    '@import "tailwindcss";',
    "",
    "@theme {",
    "  --color-card: oklch(0.985 0 0);",
    "  --color-border: oklch(0.922 0 0);",
    "  --color-ring: oklch(0.708 0 0);",
    "}",
    "",
  ];

  for (const entry of fixture.classes) {
    lines.push(`@source inline("${sourceInlineToken(entry.className)}");`);
  }

  return `${lines.join("\n")}\n`;
}

function normalizeCss(input) {
  return String(input)
    .replace(/\s+/g, " ")
    .replace(/\s*:\s*/g, ": ")
    .replace(/\s*;\s*/g, "; ")
    .trim();
}

function outputContainsDeclaration(output, declaration) {
  const haystack = normalizeCss(output);
  const needle = normalizeCss(declaration).replace(/;$/, "");
  return haystack.includes(needle);
}

function declarationsMatch(entry) {
  const dx = entry.dxStyleRequiredDeclarations || [];
  const tailwind = entry.tailwindRequiredDeclarations || [];
  return dx.length === tailwind.length && dx.every((declaration, index) => declaration === tailwind[index]);
}

function compareTailwindOutput(fixture, output) {
  const results = fixture.classes.map((entry) => {
    const requiredDeclarations = entry.tailwindRequiredDeclarations || [];
    const missingDeclarations = requiredDeclarations.filter(
      (declaration) => !outputContainsDeclaration(output, declaration),
    );
    const declarationContractMatches = declarationsMatch(entry);

    return {
      className: entry.className,
      area: entry.area,
      requiredDeclarationCount: requiredDeclarations.length,
      missingDeclarations,
      declarationContractMatches,
      passed: missingDeclarations.length === 0 && declarationContractMatches,
    };
  });

  return {
    results,
    passedClassCount: results.filter((result) => result.passed).length,
    failedClassCount: results.filter((result) => !result.passed).length,
    failedClasses: results.filter((result) => !result.passed).map((result) => result.className),
  };
}

function runTailwindCli({ root, inputPath, outputPath }) {
  const npxCommand = process.platform === "win32" ? "npx.cmd" : "npx";
  return spawnSync(npxCommand, ["--yes", TAILWIND_CLI_PACKAGE, "-i", inputPath, "-o", outputPath], {
    cwd: root,
    encoding: "utf8",
    windowsHide: true,
  });
}

function baseReport({ fixture }) {
  return {
    schema: SCHEMA,
    baseline: BASELINE,
    tailwindCliPackage: TAILWIND_CLI_PACKAGE,
    fixturePath: FIXTURE_PATH,
    comparisonScope: fixture.comparisonScope,
    classCount: fixture.classes.length,
    normalTestsRunLiveTailwind: false,
    fullTailwindParity: false,
    fairSpeedBenchmark: false,
    runPolicy: RUN_POLICY,
  };
}

function runLiveComparison({ root, fixture }) {
  const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-style-tailwind-live-"));
  const inputPath = path.join(tempRoot, "input.css");
  const outputPath = path.join(tempRoot, "tailwind.css");

  try {
    fs.writeFileSync(inputPath, buildTailwindInput(fixture), "utf8");

    const run = runTailwindCli({ root, inputPath, outputPath });
    if (run.status !== 0) {
      return {
        ...baseReport({ fixture }),
        status: "failed",
        liveTailwindExecuted: true,
        tailwindExitCode: run.status,
        stdout: run.stdout || "",
        stderr: run.stderr || "",
        passedClassCount: 0,
        failedClassCount: fixture.classes.length,
        failedClasses: fixture.classes.map((entry) => entry.className),
      };
    }

    const output = fs.readFileSync(outputPath, "utf8");
    const comparison = compareTailwindOutput(fixture, output);

    return {
      ...baseReport({ fixture }),
      status: comparison.failedClassCount === 0 ? "passed" : "failed",
      liveTailwindExecuted: true,
      tailwindExitCode: run.status,
      outputBytes: Buffer.byteLength(output),
      passedClassCount: comparison.passedClassCount,
      failedClassCount: comparison.failedClassCount,
      failedClasses: comparison.failedClasses,
      results: comparison.results,
    };
  } finally {
    fs.rmSync(tempRoot, { recursive: true, force: true });
  }
}

function runComparison(options = {}) {
  const root = repoRoot(options.root);
  const fixture = readFixture(root);
  const live = options.live === true || process.env.DX_STYLE_RUN_LIVE_TAILWIND === "1";

  if (!live) {
    return {
      ...baseReport({ fixture }),
      status: "skipped-governed-live-run",
      liveTailwindExecuted: false,
      skipReason: "Set DX_STYLE_RUN_LIVE_TAILWIND=1 or pass --live to execute real Tailwind.",
    };
  }

  return runLiveComparison({ root, fixture });
}

if (require.main === module) {
  const live = process.argv.includes("--live");
  const report = runComparison({ live });
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
  if (report.liveTailwindExecuted && report.status !== "passed") {
    process.exitCode = 1;
  }
}

module.exports = {
  BASELINE,
  FIXTURE_PATH,
  RUN_POLICY,
  SCHEMA,
  TAILWIND_CLI_PACKAGE,
  buildTailwindInput,
  compareTailwindOutput,
  runComparison,
};
