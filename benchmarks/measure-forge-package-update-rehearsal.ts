const fs = require("fs");
const os = require("os");
const path = require("path");
const { runDxWwwCli } = require("./dx-www-cli-paths.ts");

const root = path.resolve(__dirname, "..");
const reportDir = path.join(__dirname, "reports");
const outJsonPath = path.join(reportDir, "forge-package-update-rehearsal.json");
const outMdPath = path.join(reportDir, "forge-package-update-rehearsal.md");

async function buildReport(options = {}) {
  const generatedAt = options.generatedAt || new Date().toISOString();
  const explicitProjectDir = options.projectDir || process.env.DX_FORGE_UPDATE_REHEARSAL_PROJECT || null;
  const projectDir = path.resolve(explicitProjectDir || path.join(root, ".dx", "adoption-update-rehearsal"));
  const prepare = options.prepare ?? process.env.DX_FORGE_UPDATE_REHEARSAL_PREPARE !== "0";
  const runDxCommand = options.runDxCommand || runDxCommandDefault;

  let prepareResult = {
    enabled: false,
    status: "skipped",
    reason: "prepare disabled",
  };
  if (prepare) {
    if ((options.resetProject ?? !explicitProjectDir) && projectDir.startsWith(path.join(root, ".dx"))) {
      fs.rmSync(projectDir, { recursive: true, force: true });
    }
    prepareResult = prepareAdoptionProject(projectDir, runDxCommand);
  }

  const scenarioRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-forge-update-rehearsal-"));
  const findings = [];
  const baseReady = fs.existsSync(buttonPath(projectDir));
  if (!baseReady) {
    findings.push("base adoption project is missing components/ui/button.tsx");
  }

  const scenarios = {
    green_update: baseReady
      ? runGreenUpdate(projectDir, scenarioRoot, runDxCommand)
      : skippedScenario("green update", "base project missing button source"),
    yellow_default_block: baseReady
      ? runYellowDefaultBlock(projectDir, scenarioRoot, runDxCommand)
      : skippedScenario("yellow default block", "base project missing button source"),
    yellow_review_accept: baseReady
      ? runYellowReviewAccept(projectDir, scenarioRoot, runDxCommand)
      : skippedScenario("yellow review accept", "base project missing button source"),
    red_quarantine: baseReady
      ? runRedQuarantine(projectDir, scenarioRoot, runDxCommand)
      : skippedScenario("red quarantine", "base project missing button source"),
    rollback_coverage: baseReady
      ? runRollbackCoverage(projectDir, scenarioRoot, runDxCommand)
      : skippedScenario("rollback coverage", "base project missing button source"),
  };

  const noNodeModules = !projectHasNodeModules(projectDir) &&
    Object.values(scenarios).every((scenario) => !scenario.project_dir || !projectHasNodeModules(scenario.project_dir));
  if (!noNodeModules) {
    findings.push("node_modules exists in the base or scenario project");
  }
  if (prepareResult.enabled && prepareResult.status !== "passed") {
    findings.push("adoption-smoke prepare command failed");
  }
  for (const scenario of Object.values(scenarios)) {
    if (!scenario.passed) {
      findings.push(`${scenario.name} did not pass`);
    }
  }

  const score = scoreReport(scenarios, prepareResult, noNodeModules, findings);
  return {
    generated_at: generatedAt,
    report_id: "forge-package-update-rehearsal-v1",
    source: "benchmarks/measure-forge-package-update-rehearsal.ts",
    project_dir: projectDir,
    scenario_root: scenarioRoot,
    package: "shadcn/ui/button",
    score,
    passed: score >= 90 && findings.length === 0,
    no_node_modules: noNodeModules,
    prepare_result: prepareResult,
    scenarios,
    findings,
    honest_scope: [
      "This is a local package-update rehearsal for the generated Forge adoption app.",
      "It proves Forge update traffic behavior for the curated shadcn/ui/button package, not arbitrary npm ingestion.",
      "The harness never runs package installs and treats node_modules as a release-risk finding.",
      "Rollback coverage is checked through Forge receipts, not by deleting user work or resetting git state.",
    ],
  };
}

function prepareAdoptionProject(projectDir, runDxCommand) {
  fs.mkdirSync(path.join(projectDir, ".dx", "forge", "adoption-smoke"), { recursive: true });
  const outputPath = path.join(projectDir, ".dx", "forge", "adoption-smoke", "forge-smoke.json");
  const result = timedRun(() =>
    runDxCommand([
      "forge",
      "adoption-smoke",
      "--project",
      projectDir,
      "--format",
      "json",
      "--output",
      outputPath,
      "--fail-under",
      "90",
      "--quiet",
    ])
  );
  return commandSummary("prepare adoption smoke", result, result.status === 0, {
    output: outputPath,
  });
}

function runGreenUpdate(baseProject, scenarioRoot, runDxCommand) {
  const projectDir = copyScenarioProject(baseProject, scenarioRoot, "green");
  const update = timedRun(() => runDxCommand(["update", "ui/button", "--project", projectDir, "--write"]));
  const check = runPackageVerification(projectDir, runDxCommand);
  const passed = update.status === 0 && check.status === 0 && !projectHasNodeModules(projectDir);
  return scenarioSummary("green update", projectDir, passed, {
    update: commandSummary("dx update ui/button --write", update, update.status === 0),
    check: commandSummary("dx forge verify-package shadcn/ui/button", check, check.status === 0),
    traffic: "green",
    receipt: newestReceipt(projectDir),
  });
}

function runYellowDefaultBlock(baseProject, scenarioRoot, runDxCommand) {
  const projectDir = copyScenarioProject(baseProject, scenarioRoot, "yellow-block");
  appendLocalEdit(projectDir);
  const update = timedRun(() => runDxCommand(["update", "ui/button", "--project", projectDir, "--write"]));
  const combined = `${update.stdout}\n${update.stderr}`;
  const passed =
    update.status !== 0 &&
    /yellow|review/i.test(combined) &&
    fs.readFileSync(buttonPath(projectDir), "utf8").includes("local product edit") &&
    !projectHasNodeModules(projectDir);
  return scenarioSummary("yellow default block", projectDir, passed, {
    update: commandSummary("dx update ui/button --write", update, update.status !== 0),
    traffic: "yellow",
    local_edit_preserved: fs.readFileSync(buttonPath(projectDir), "utf8").includes("local product edit"),
  });
}

function runYellowReviewAccept(baseProject, scenarioRoot, runDxCommand) {
  const projectDir = copyScenarioProject(baseProject, scenarioRoot, "yellow-review");
  appendLocalEdit(projectDir);
  const update = timedRun(() =>
    runDxCommand([
      "update",
      "ui/button",
      "--project",
      projectDir,
      "--write",
      "--accept-yellow",
      "--review-note",
      "Reviewed local adoption-app button edit during package update rehearsal",
    ])
  );
  const check = runPackageVerification(projectDir, runDxCommand, "80");
  const editPreserved = fs.readFileSync(buttonPath(projectDir), "utf8").includes("local product edit");
  const passed = update.status === 0 && editPreserved && !projectHasNodeModules(projectDir);
  return scenarioSummary("yellow review accept", projectDir, passed, {
    update: commandSummary("dx update ui/button --write --accept-yellow", update, update.status === 0),
    review_state_check: commandSummary(
      "dx forge verify-package shadcn/ui/button after reviewed local edit",
      check,
      check.status === 0
    ),
    traffic: "yellow",
    local_edit_preserved: editPreserved,
    receipt: newestReceipt(projectDir),
    note: "Reviewed yellow edits are accepted into a receipt; strict launch gates can still require human review before release.",
  });
}

function runRedQuarantine(baseProject, scenarioRoot, runDxCommand) {
  const projectDir = copyScenarioProject(baseProject, scenarioRoot, "red");
  fs.rmSync(buttonPath(projectDir), { force: true });
  const update = timedRun(() => runDxCommand(["update", "ui/button", "--project", projectDir, "--write"]));
  const combined = `${update.stdout}\n${update.stderr}`;
  const passed =
    update.status !== 0 &&
    /red|blocked|green updates/i.test(combined) &&
    !fs.existsSync(buttonPath(projectDir)) &&
    !projectHasNodeModules(projectDir);
  return scenarioSummary("red quarantine", projectDir, passed, {
    update: commandSummary("dx update ui/button --write", update, update.status !== 0),
    traffic: "red",
    source_still_quarantined: !fs.existsSync(buttonPath(projectDir)),
  });
}

function runRollbackCoverage(baseProject, scenarioRoot, runDxCommand) {
  const projectDir = copyScenarioProject(baseProject, scenarioRoot, "rollback");
  const update = timedRun(() => runDxCommand(["update", "ui/button", "--project", projectDir, "--write"]));
  const receipt = newestReceipt(projectDir);
  appendLocalEdit(projectDir);
  const rollback = receipt
    ? timedRun(() => runDxCommand(["forge", "rollback", receipt, "--project", projectDir, "--write"]))
    : { status: 1, stdout: "", stderr: "missing receipt", duration_ms: 0 };
  const check = runPackageVerification(projectDir, runDxCommand);
  const restored = fs.existsSync(buttonPath(projectDir)) &&
    !fs.readFileSync(buttonPath(projectDir), "utf8").includes("local product edit");
  const passed =
    update.status === 0 &&
    Boolean(receipt) &&
    rollback.status === 0 &&
    check.status === 0 &&
    restored &&
    !projectHasNodeModules(projectDir);
  return scenarioSummary("rollback coverage", projectDir, passed, {
    update: commandSummary("dx update ui/button --write", update, update.status === 0),
    rollback: commandSummary("dx forge rollback --write", rollback, rollback.status === 0),
    check: commandSummary("dx forge verify-package shadcn/ui/button", check, check.status === 0),
    receipt,
    restored,
  });
}

function runPackageVerification(projectDir, runDxCommand, failUnder = "90") {
  return timedRun(() =>
    runDxCommand([
      "forge",
      "verify-package",
      "shadcn/ui/button",
      "--project",
      projectDir,
      "--format",
      "json",
      "--fail-under",
      failUnder,
    ])
  );
}

function runDxCommandDefault(args) {
  return runDxWwwCli(root, args);
}

function timedRun(run) {
  const started = Date.now();
  const result = run();
  return {
    ...result,
    duration_ms: Date.now() - started,
  };
}

function copyScenarioProject(baseProject, scenarioRoot, scenarioName) {
  const target = path.join(scenarioRoot, scenarioName);
  fs.rmSync(target, { recursive: true, force: true });
  fs.cpSync(baseProject, target, {
    recursive: true,
    filter(source) {
      return !source.includes(`${path.sep}node_modules${path.sep}`) && !source.endsWith(`${path.sep}node_modules`);
    },
  });
  return target;
}

function appendLocalEdit(projectDir) {
  fs.appendFileSync(buttonPath(projectDir), "\n// local product edit kept for Forge review\n");
}

function newestReceipt(projectDir) {
  const receiptsDir = path.join(projectDir, ".dx", "forge", "receipts");
  if (!fs.existsSync(receiptsDir)) return null;
  const receipts = fs
    .readdirSync(receiptsDir)
    .filter((name) => name.includes("shadcn-ui-button") && name.endsWith(".json"))
    .map((name) => path.join(receiptsDir, name))
    .sort((left, right) => fs.statSync(right).mtimeMs - fs.statSync(left).mtimeMs);
  return receipts[0] || null;
}

function buttonPath(projectDir) {
  return path.join(projectDir, "components", "ui", "button.tsx");
}

function projectHasNodeModules(projectDir) {
  return fs.existsSync(path.join(projectDir, "node_modules"));
}

function commandSummary(name, result, passed, extra = {}) {
  return {
    name,
    passed,
    exit_code: result.status,
    duration_ms: result.duration_ms,
    stdout_tail: tail(result.stdout),
    stderr_tail: tail(result.stderr),
    ...extra,
  };
}

function scenarioSummary(name, projectDir, passed, details) {
  return {
    name,
    project_dir: projectDir,
    passed,
    no_node_modules: !projectHasNodeModules(projectDir),
    ...details,
  };
}

function skippedScenario(name, reason) {
  return {
    name,
    project_dir: null,
    passed: false,
    skipped: true,
    reason,
  };
}

function scoreReport(scenarios, prepareResult, noNodeModules, findings) {
  let score = 100;
  if (prepareResult.enabled && prepareResult.status !== "passed") score -= 20;
  for (const scenario of Object.values(scenarios)) {
    if (!scenario.passed) score -= 15;
  }
  if (!noNodeModules) score -= 20;
  score -= Math.max(0, findings.length - 1) * 4;
  return Math.max(0, Math.min(100, score));
}

function renderMarkdown(report) {
  const rows = [
    ["Green update", report.scenarios.green_update],
    ["Yellow default block", report.scenarios.yellow_default_block],
    ["Yellow review accept", report.scenarios.yellow_review_accept],
    ["Red quarantine", report.scenarios.red_quarantine],
    ["Rollback coverage", report.scenarios.rollback_coverage],
  ];
  const lines = [
    "# Forge Package Update Rehearsal",
    "",
    `Generated: ${report.generated_at}`,
    `Project: \`${report.project_dir}\``,
    `Package: \`${report.package}\``,
    `Score: \`${report.score}\` / \`100\``,
    `Passed: \`${report.passed}\``,
    `No node_modules: \`${report.no_node_modules}\``,
    "",
    "## Scenarios",
    "",
    "| Scenario | Passed | Traffic | No node_modules | Evidence |",
    "| --- | --- | --- | --- | --- |",
    ...rows.map(([label, scenario]) =>
      [
        label,
        scenario.passed,
        scenario.traffic || "n/a",
        scenario.no_node_modules ?? "n/a",
        scenarioEvidence(scenario),
      ].join(" | ")
    ).map((row) => `| ${row} |`),
    "",
    "## Findings",
    "",
    ...(report.findings.length ? report.findings.map((finding) => `- ${finding}`) : ["- none"]),
    "",
    "## Honest Scope",
    "",
    ...report.honest_scope.map((item) => `- ${item}`),
    "",
  ];
  return lines.join("\n");
}

function scenarioEvidence(scenario) {
  if (scenario.skipped) return scenario.reason;
  if (scenario.receipt) return `receipt ${path.basename(scenario.receipt)}`;
  if (scenario.rollback?.passed) return "rollback command passed";
  if (scenario.update?.stderr_tail) return scenario.update.stderr_tail.replace(/\r?\n/g, " ").slice(0, 80);
  if (scenario.update?.stdout_tail) return scenario.update.stdout_tail.replace(/\r?\n/g, " ").slice(0, 80);
  return "command evidence recorded";
}

function writeReport(report) {
  fs.mkdirSync(reportDir, { recursive: true });
  fs.writeFileSync(outJsonPath, `${JSON.stringify(report, null, 2)}\n`);
  fs.writeFileSync(outMdPath, renderMarkdown(report));
}

function tail(value, maxLength = 1200) {
  const text = String(value || "").trim();
  return text.length <= maxLength ? text : text.slice(text.length - maxLength);
}

async function main() {
  const report = await buildReport();
  writeReport(report);
  console.log(
    JSON.stringify(
      {
        report: [path.relative(root, outJsonPath), path.relative(root, outMdPath)],
        score: report.score,
        passed: report.passed,
        no_node_modules: report.no_node_modules,
      },
      null,
      2
    )
  );
  if (!report.passed) process.exitCode = 1;
}

if (require.main === module) {
  main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
  });
}

module.exports = {
  buildReport,
  renderMarkdown,
};
