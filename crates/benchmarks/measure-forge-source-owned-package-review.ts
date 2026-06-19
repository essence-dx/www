const fs = require("fs");
const os = require("os");
const path = require("path");
const { runDxWwwCli } = require("./dx-www-cli-paths.ts");

const {
  buildReport: buildUpdateRehearsalReport,
} = require("./measure-forge-package-update-rehearsal.ts");

const root = path.resolve(__dirname, "..");
const reportDir = path.join(__dirname, "reports");
const outJsonPath = path.join(reportDir, "forge-source-owned-package-review.json");
const outMdPath = path.join(reportDir, "forge-source-owned-package-review.md");

async function buildReview(options = {}) {
  const generatedAt = options.generatedAt || new Date().toISOString();
  const explicitProjectDir = options.projectDir || process.env.DX_FORGE_SOURCE_REVIEW_PROJECT || null;
  const projectDir = path.resolve(explicitProjectDir || path.join(root, ".dx", "adoption-package-review"));
  const prepare = options.prepare ?? process.env.DX_FORGE_SOURCE_REVIEW_PREPARE !== "0";
  const runDxCommand = options.runDxCommand || runDxCommandDefault;
  const reviewDir = path.join(projectDir, ".dx", "forge", "source-owned-review");
  const adoptionReportPath = path.join(reviewDir, "forge-adoption-report.json");

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

  fs.mkdirSync(reviewDir, { recursive: true });
  const manifestPath = path.join(projectDir, ".dx", "forge", "source-manifest.json");
  const manifest = readJsonFile(manifestPath);
  const packages = packageReviews(projectDir, manifest.value);
  const adoptionReportCommand = timedRun(() =>
    runDxCommand([
      "forge",
      "adoption-report",
      "--project",
      projectDir,
      "--format",
      "json",
      "--output",
      adoptionReportPath,
      "--fail-under",
      "90",
      "--quiet",
    ])
  );
  const adoptionReport = readJsonFile(adoptionReportPath);
  const verifyAllCommand = timedRun(() =>
    runDxCommand(["forge", "verify-package", "--all", "--project", projectDir, "--format", "json", "--fail-under", "90"])
  );
  const verifyAll = parseJson(verifyAllCommand.stdout);
  const updateRehearsal = await buildUpdateRehearsalReport({
    generatedAt,
    projectDir,
    prepare: false,
    resetProject: false,
    runDxCommand,
  });

  const noNodeModules = !projectHasNodeModules(projectDir) && updateRehearsal.no_node_modules === true;
  const reviewGates = reviewGateSummary({
    packages,
    manifest,
    adoptionReport,
    adoptionReportCommand,
    verifyAll,
    verifyAllCommand,
    updateRehearsal,
    noNodeModules,
    prepareResult,
  });
  const findings = reviewFindings(reviewGates, packages, manifest, adoptionReport, adoptionReportCommand, verifyAllCommand);
  const score = scoreReview(reviewGates, findings);

  return {
    generated_at: generatedAt,
    report_id: "forge-source-owned-package-review-v1",
    source: "benchmarks/measure-forge-source-owned-package-review.ts",
    project_dir: projectDir,
    review_dir: reviewDir,
    score,
    passed: score >= 90 && findings.length === 0,
    no_node_modules: noNodeModules,
    package_count: packages.length,
    packages,
    prepare_result: prepareResult,
    commands: {
      adoption_report: commandSummary("dx forge adoption-report", adoptionReportCommand, adoptionReportCommand.status === 0, {
        output: adoptionReportPath,
        parsed: adoptionReport.ok,
      }),
      verify_all: commandSummary("dx forge verify-package --all", verifyAllCommand, verifyAllCommand.status === 0, {
        parsed: verifyAll.ok,
      }),
    },
    adoption_report: adoptionReport.ok ? adoptionReport.value : null,
    verify_all: verifyAll.ok ? verifyAll.value : null,
    update_rehearsal: compactUpdateRehearsal(updateRehearsal),
    review_gates: reviewGates,
    findings,
    honest_scope: [
      "This joins existing local Forge adoption evidence into one source-owned package review artifact.",
      "It proves curated package docs, receipts, advisory placeholders, update traffic, local-edit yellow review, rollback, and no node_modules for the example app path.",
      "It still does not claim arbitrary npm ingestion, live advisory coverage, or production customer adoption.",
    ],
  };
}

function prepareAdoptionProject(projectDir, runDxCommand) {
  fs.mkdirSync(path.join(projectDir, ".dx", "forge", "source-owned-review"), { recursive: true });
  const outputPath = path.join(projectDir, ".dx", "forge", "source-owned-review", "forge-adoption-smoke.json");
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
    enabled: true,
    status: result.status === 0 ? "passed" : "failed",
    output: outputPath,
  });
}

function packageReviews(projectDir, manifest) {
  const packages = Array.isArray(manifest?.packages) ? manifest.packages : [];
  return packages.map((pkg) => {
    const slug = packageSlug(pkg.package_id || "unknown");
    const docsPath = path.join(projectDir, ".dx", "forge", "docs", docsName(pkg.package_id || "unknown", pkg.variant || "default"));
    const rollbackReceiptPath = pkg.rollback_receipt
      ? path.join(projectDir, ".dx", "forge", "receipts", pkg.rollback_receipt)
      : null;
    const advisory = pkg.advisory_review || {};
    return {
      package_id: pkg.package_id || "unknown",
      variant: pkg.variant || "default",
      version: pkg.version || "unknown",
      source_kind: pkg.source_kind || "unknown",
      file_count: Array.isArray(pkg.files) ? pkg.files.length : 0,
      docs: {
        path: docsPath,
        exists: fs.existsSync(docsPath),
      },
      receipts: receiptFilesForPackage(projectDir, slug),
      rollback: {
        receipt: pkg.rollback_receipt || null,
        exists: rollbackReceiptPath ? fs.existsSync(rollbackReceiptPath) : false,
      },
      advisory: {
        coverage_kind: advisory.coverage_kind || "missing",
        provider: advisory.provider || "missing",
        live_coverage: advisory.live_coverage === true,
        finding_count: Number.isFinite(advisory.finding_count) ? advisory.finding_count : 0,
        reviewed_at: advisory.reviewed_at || null,
        placeholder:
          advisory.coverage_kind === "curated-fixture" &&
          advisory.provider === "dx-forge-curated-advisory-fixture" &&
          advisory.live_coverage === false &&
          Boolean(advisory.reviewed_at),
      },
      license: pkg.license || "unknown",
    };
  });
}

function receiptFilesForPackage(projectDir, slug) {
  const receiptsDir = path.join(projectDir, ".dx", "forge", "receipts");
  if (!fs.existsSync(receiptsDir)) return [];
  return fs
    .readdirSync(receiptsDir)
    .filter((name) => name.includes(slug) && name.endsWith(".json"))
    .sort()
    .map((name) => path.join(receiptsDir, name));
}

function reviewGateSummary(input) {
  const packageCount = input.packages.length;
  const advisoryScopePackages = input.packages.filter((pkg) => pkg.source_kind !== "local");
  const verifyAllExpected = advisoryScopePackages.length || packageCount;
  const docsPresent = input.packages.filter((pkg) => pkg.docs.exists).length;
  const packagesWithReceipts = input.packages.filter((pkg) => pkg.receipts.length > 0).length;
  const advisoryPlaceholders = advisoryScopePackages.filter((pkg) => pkg.advisory.placeholder).length;
  const yellowBlock = input.updateRehearsal.scenarios?.yellow_default_block;
  const yellowAccept = input.updateRehearsal.scenarios?.yellow_review_accept;
  const rollback = input.updateRehearsal.scenarios?.rollback_coverage;

  return {
    docs: gate("docs", packageCount > 0 && docsPresent === packageCount, docsPresent, packageCount),
    receipts: gate("receipts", packageCount > 0 && packagesWithReceipts === packageCount, packagesWithReceipts, packageCount),
    rollback: gate("rollback", Boolean(rollback?.passed), rollback?.passed ? 1 : 0, 1, {
      scenario: rollback?.name || "rollback coverage",
    }),
    advisory_placeholders: gate(
      "advisory placeholders",
      advisoryScopePackages.length > 0 && advisoryPlaceholders === advisoryScopePackages.length,
      advisoryPlaceholders,
      advisoryScopePackages.length
    ),
    local_edit_yellow: gate(
      "local edit yellow review",
      Boolean(yellowBlock?.passed && yellowAccept?.passed),
      [yellowBlock, yellowAccept].filter((scenario) => scenario?.passed).length,
      2,
      {
        scenarios: [yellowBlock?.name || "yellow default block", yellowAccept?.name || "yellow review accept"],
      }
    ),
    verify_all: gate(
      "verify all current Forge review packages",
      input.verifyAllCommand.status === 0 && input.verifyAll.ok && input.verifyAll.value?.passed === true,
      input.verifyAll.ok && Array.isArray(input.verifyAll.value?.packages) ? input.verifyAll.value.packages.length : 0,
      verifyAllExpected
    ),
    adoption_report: gate(
      "adoption report",
      input.adoptionReportCommand.status === 0 && input.adoptionReport.ok && input.adoptionReport.value?.passed === true,
      input.adoptionReport.ok ? 1 : 0,
      1
    ),
    no_node_modules: gate("no node_modules", input.noNodeModules, input.noNodeModules ? 1 : 0, 1),
    prepare: gate(
      "prepare adoption app",
      !input.prepareResult.enabled || input.prepareResult.status === "passed",
      !input.prepareResult.enabled || input.prepareResult.status === "passed" ? 1 : 0,
      1
    ),
  };
}

function gate(name, passed, present, expected, extra = {}) {
  return {
    name,
    passed,
    present,
    expected,
    score: expected === 0 ? 0 : Math.min(100, Math.floor((Number(present) * 100) / expected)),
    ...extra,
  };
}

function reviewFindings(gates, packages, manifest, adoptionReport, adoptionReportCommand, verifyAllCommand) {
  const findings = [];
  if (!manifest.ok) findings.push(`source manifest could not be read: ${manifest.error}`);
  if (!adoptionReport.ok) findings.push(`adoption report could not be read: ${adoptionReport.error}`);
  if (adoptionReportCommand.status !== 0) findings.push("dx forge adoption-report failed");
  if (verifyAllCommand.status !== 0) findings.push("dx forge verify-package --all failed");
  for (const [name, gateValue] of Object.entries(gates)) {
    if (!gateValue.passed) {
      findings.push(`${name.replaceAll("_", " ")} gate did not pass (${gateValue.present}/${gateValue.expected})`);
    }
  }
  for (const pkg of packages) {
    if (!pkg.docs.exists) findings.push(`${pkg.package_id} is missing package docs`);
    if (pkg.receipts.length === 0) findings.push(`${pkg.package_id} has no package receipt`);
    if (pkg.source_kind !== "local" && !pkg.advisory.placeholder) {
      findings.push(`${pkg.package_id} is missing curated advisory placeholder metadata`);
    }
  }
  return [...new Set(findings)];
}

function scoreReview(gates, findings) {
  let score = 100;
  for (const gateValue of Object.values(gates)) {
    if (!gateValue.passed) score -= 12;
  }
  score -= Math.max(0, findings.length - 1) * 2;
  return Math.max(0, Math.min(100, score));
}

function compactUpdateRehearsal(report) {
  const scenarioNames = [
    "green_update",
    "yellow_default_block",
    "yellow_review_accept",
    "red_quarantine",
    "rollback_coverage",
  ];
  return {
    score: report.score,
    passed: report.passed,
    no_node_modules: report.no_node_modules,
    scenarios: Object.fromEntries(
      scenarioNames.map((name) => [
        name,
        {
          passed: report.scenarios?.[name]?.passed === true,
          traffic: report.scenarios?.[name]?.traffic || "n/a",
          evidence: scenarioEvidence(report.scenarios?.[name]),
        },
      ])
    ),
  };
}

function renderMarkdown(report) {
  const lines = [
    "# Forge Source-Owned Package Fixture Review",
    "",
    `Generated: ${report.generated_at}`,
    `Project: \`${report.project_dir}\``,
    `Score: \`${report.score}\` / \`100\``,
    `Passed: \`${report.passed}\``,
    `No node_modules: \`${report.no_node_modules}\``,
    "",
    "## Review Gates",
    "",
    "| Gate | Passed | Present | Expected |",
    "| --- | --- | ---: | ---: |",
    ...Object.values(report.review_gates).map((gateValue) =>
      `| ${gateValue.name} | \`${gateValue.passed}\` | ${gateValue.present} | ${gateValue.expected} |`
    ),
    "",
    "## Packages",
    "",
    "| Package | Source | Docs | Receipts | Advisory | Live Feed | Rollback Receipt |",
    "| --- | --- | --- | ---: | --- | --- | --- |",
    ...report.packages.map((pkg) =>
      [
        `\`${pkg.package_id}\``,
        `\`${pkg.source_kind}\``,
        `\`${pkg.docs.exists}\``,
        pkg.receipts.length,
        `\`${pkg.advisory.coverage_kind}\` / \`${pkg.advisory.provider}\``,
        `\`${pkg.advisory.live_coverage}\``,
        pkg.rollback.receipt ? `\`${pkg.rollback.receipt}\`` : "`n/a`",
      ].join(" | ")
    ).map((row) => `| ${row} |`),
    "",
    "## Local-Edit And Rollback Rehearsal",
    "",
    `- Score: \`${report.update_rehearsal.score}\` / \`100\``,
    `- Passed: \`${report.update_rehearsal.passed}\``,
    `- No node_modules: \`${report.update_rehearsal.no_node_modules}\``,
    "",
    "| Scenario | Passed | Traffic | Evidence |",
    "| --- | --- | --- | --- |",
    ...Object.entries(report.update_rehearsal.scenarios).map(([name, scenario]) =>
      `| ${name.replaceAll("_", " ")} | \`${scenario.passed}\` | \`${scenario.traffic}\` | ${markdownTableCell(scenario.evidence)} |`
    ),
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

function writeReport(report) {
  fs.mkdirSync(reportDir, { recursive: true });
  fs.writeFileSync(outJsonPath, `${JSON.stringify(report, null, 2)}\n`);
  fs.writeFileSync(outMdPath, renderMarkdown(report));
}

function readJsonFile(filePath) {
  if (!fs.existsSync(filePath)) {
    return { ok: false, value: null, error: `${filePath} does not exist` };
  }
  return parseJson(fs.readFileSync(filePath, "utf8"), filePath);
}

function parseJson(text, label = "json") {
  try {
    return { ok: true, value: JSON.parse(text), error: null };
  } catch (error) {
    return { ok: false, value: null, error: `${label}: ${error.message}` };
  }
}

function docsName(packageId, variant) {
  const slug = packageSlug(packageId);
  return variant === "default" ? `${slug}.md` : `${slug}--variant-${variant.replaceAll(".", "-")}.md`;
}

function packageSlug(packageId) {
  return packageId.replaceAll("/", "-");
}

function scenarioEvidence(scenario) {
  if (!scenario) return "missing scenario";
  if (scenario.receipt) return `receipt ${path.basename(scenario.receipt)}`;
  if (scenario.rollback?.passed) return "rollback command passed";
  if (scenario.update?.stderr_tail) return scenario.update.stderr_tail.replace(/\r?\n/g, " ").slice(0, 90);
  if (scenario.update?.stdout_tail) return scenario.update.stdout_tail.replace(/\r?\n/g, " ").slice(0, 90);
  return "command evidence recorded";
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

function timedRun(run) {
  const started = Date.now();
  const result = run();
  return {
    ...result,
    duration_ms: Date.now() - started,
  };
}

function runDxCommandDefault(args) {
  return runDxWwwCli(root, args);
}

function projectHasNodeModules(projectDir) {
  return fs.existsSync(path.join(projectDir, "node_modules"));
}

function markdownTableCell(value) {
  return String(value || "-").replace(/\|/g, "\\|").replace(/\r?\n/g, " ");
}

function tail(value, maxLength = 1200) {
  const text = String(value || "").trim();
  return text.length <= maxLength ? text : text.slice(text.length - maxLength);
}

async function main() {
  const report = await buildReview();
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
  buildReview,
  renderMarkdown,
};
