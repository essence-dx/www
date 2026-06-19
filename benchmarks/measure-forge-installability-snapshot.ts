const fs = require("fs");
const crypto = require("crypto");
const path = require("path");

const root = path.resolve(__dirname, "..");
const reportDir = path.join(__dirname, "reports");
const defaultJsonName = "forge-installability-snapshot.json";
const defaultMdName = "forge-installability-snapshot.md";
const defaultProvenanceJsonName = "forge-installability-snapshot.provenance.json";
const defaultProvenanceMdName = "forge-installability-snapshot.provenance.md";
const defaultHistoryDirName = "forge-installability-history";
const defaultHistoryJsonName = "index.json";
const defaultHistoryMdName = "index.md";

const staticBaselines = [
  {
    id: "npm-install-baseline",
    tool: "npm",
    operation: "npm install",
    baseline_kind: "static-reference",
    measurement_kind: "skipped package-install reference",
    time_ms: 15000,
    artifact_bytes: 25_000_000,
    package_install_ran: false,
    node_modules_required: true,
    source: "static npm install reference row",
    evidence:
      "Install is intentionally skipped; this row is only a conservative reference point for reviewing Forge installability.",
  },
  {
    id: "shadcn-add-baseline",
    tool: "shadcn",
    operation: "npx shadcn add button",
    baseline_kind: "static-reference",
    measurement_kind: "skipped package-install reference",
    time_ms: 8000,
    artifact_bytes: 4_000_000,
    package_install_ran: false,
    node_modules_required: true,
    source: "static shadcn add reference row",
    evidence:
      "Command is intentionally skipped; this row is not a live shadcn benchmark and creates no node_modules.",
  },
];

function buildSnapshot(options = {}) {
  const rootDir = path.resolve(options.rootDir || root);
  const generatedAt = options.generatedAt || new Date().toISOString();
  const updateReportPath =
    options.updateReportPath || path.join(rootDir, "benchmarks", "reports", "forge-package-update-rehearsal.json");
  const sourceReviewPath =
    options.sourceReviewPath || path.join(rootDir, "benchmarks", "reports", "forge-source-owned-package-review.json");
  const releaseBundle = findFirstExistingDir(options.releaseBundleDirs || defaultReleaseBundleDirs(rootDir));
  const updateReport = readJsonFile(updateReportPath);
  const sourceReview = readJsonFile(sourceReviewPath);
  const releaseBundleSummary = summarizeDirectory(releaseBundle);
  const updateEvidenceSummary = summarizeFiles([updateReportPath, sourceReviewPath].filter(fs.existsSync));

  const forgeInstall = forgeInstallRow({
    rootDir,
    releaseBundle,
    releaseBundleSummary,
    updateReport,
    updateReportPath,
  });
  const forgeUpgrade = forgeUpgradeRow({
    rootDir,
    updateEvidenceSummary,
    updateReport,
    updateReportPath,
  });
  const rows = [forgeInstall, forgeUpgrade, ...staticBaselines.map((baseline) => ({ ...baseline }))];
  const checks = buildChecks({
    rootDir,
    rows,
    releaseBundle,
    updateReport,
    sourceReview,
  });
  const findings = buildFindings(checks, rows, releaseBundle, updateReport, sourceReview);
  const score = scoreSnapshot(checks, findings);

  return {
    generated_at: generatedAt,
    report_id: "forge-installability-snapshot-v1",
    source: "benchmarks/measure-forge-installability-snapshot.ts",
    root_dir: rootDir,
    score,
    passed: score >= 90 && findings.length === 0,
    scope: {
      no_package_installs_run: true,
      not_live_npm_or_shadcn_benchmark: true,
      competitor_builds_run: false,
      no_node_modules_created: !fs.existsSync(path.join(rootDir, "node_modules")),
      safe_public_claim:
        "Forge beta install and upgrade rows use local evidence artifacts; npm and shadcn rows are static references, not live install measurements.",
    },
    sources: {
      release_bundle: releaseBundle ? displayPath(releaseBundle, rootDir) : null,
      package_update_rehearsal: displayPath(updateReportPath, rootDir),
      source_owned_package_review: displayPath(sourceReviewPath, rootDir),
    },
    release_bundle: releaseBundleSummary,
    update_evidence: updateEvidenceSummary,
    rows,
    comparisons: buildComparisons(rows),
    checks,
    findings,
    honest_scope: [
      "This is an installability snapshot, not a live npm/shadcn install benchmark.",
      "The script never runs npm, npx, shadcn, cargo, or package manager install commands.",
      "Forge timing comes from local beta/adoption evidence reports when present; static baseline rows are review references only.",
      "Artifact size is based on local release-bundle and update-review evidence files, not downloaded remote packages.",
      "Broader framework or ecosystem replacement claims still require separate live benchmark suites.",
    ],
  };
}

function forgeInstallRow(input) {
  const timing = installTiming(input.updateReport, input.updateReportPath, input.rootDir);
  return {
    id: "dx-forge-beta-install",
    tool: "dx forge",
    operation: "beta-install",
    baseline_kind: "local-evidence",
    measurement_kind: timing.kind,
    time_ms: timing.time_ms,
    artifact_bytes: input.releaseBundleSummary.total_bytes,
    artifact_files: input.releaseBundleSummary.file_count,
    package_install_ran: false,
    node_modules_required: false,
    source: input.releaseBundle ? displayPath(input.releaseBundle, input.rootDir) : "missing local release bundle",
    evidence: timing.evidence,
  };
}

function forgeUpgradeRow(input) {
  const timing = upgradeTiming(input.updateReport, input.updateReportPath, input.rootDir);
  return {
    id: "dx-forge-beta-upgrade",
    tool: "dx forge",
    operation: "beta-upgrade",
    baseline_kind: "local-evidence",
    measurement_kind: timing.kind,
    time_ms: timing.time_ms,
    artifact_bytes: input.updateEvidenceSummary.total_bytes,
    artifact_files: input.updateEvidenceSummary.file_count,
    package_install_ran: false,
    node_modules_required: false,
    source: displayPath(input.updateReportPath, input.rootDir),
    evidence: timing.evidence,
  };
}

function installTiming(updateReport, reportPath, rootDir) {
  const duration = numberOrNull(updateReport.value?.prepare_result?.duration_ms);
  if (duration !== null && updateReport.value?.prepare_result?.passed !== false) {
    return {
      kind: "derived from local Forge adoption prepare evidence",
      time_ms: duration,
      evidence: `${displayPath(reportPath, rootDir)} prepare_result.duration_ms`,
    };
  }
  return {
    kind: "missing local Forge install timing evidence",
    time_ms: null,
    evidence: "missing prepare_result.duration_ms in package update rehearsal report",
  };
}

function upgradeTiming(updateReport, reportPath, rootDir) {
  const scenarios = updateReport.value?.scenarios || {};
  const timings = [
    scenarios.green_update?.update?.duration_ms,
    scenarios.yellow_review_accept?.update?.duration_ms,
  ]
    .map(numberOrNull)
    .filter((duration) => duration !== null);
  if (timings.length > 0) {
    return {
      kind: "local Forge package update rehearsal",
      time_ms: Math.max(...timings),
      evidence: `${displayPath(reportPath, rootDir)} green/yellow update.duration_ms`,
    };
  }
  return {
    kind: "missing local Forge upgrade timing evidence",
    time_ms: null,
    evidence: "missing green/yellow update.duration_ms in package update rehearsal report",
  };
}

function buildChecks(input) {
  const baselineRows = input.rows.filter((row) => row.baseline_kind === "static-reference");
  const forgeRows = input.rows.filter((row) => row.baseline_kind === "local-evidence");
  const noPackageInstalls =
    input.rows.every((row) => row.package_install_ran === false) &&
    !fs.existsSync(path.join(input.rootDir, "node_modules"));
  return {
    no_package_installs: gate("no package installs", noPackageInstalls, noPackageInstalls ? 1 : 0, 1),
    forge_installability: gate(
      "Forge installability evidence",
      forgeRows.length >= 2 && forgeRows.every((row) => Number.isFinite(row.time_ms) && row.time_ms > 0),
      forgeRows.filter((row) => Number.isFinite(row.time_ms) && row.time_ms > 0).length,
      2
    ),
    artifact_size: gate(
      "artifact size evidence",
      forgeRows.every((row) => Number.isFinite(row.artifact_bytes) && row.artifact_bytes > 0),
      forgeRows.filter((row) => Number.isFinite(row.artifact_bytes) && row.artifact_bytes > 0).length,
      2
    ),
    baseline_scope: gate(
      "static baseline scope",
      baselineRows.length >= 2 && baselineRows.every((row) => row.package_install_ran === false),
      baselineRows.filter((row) => row.package_install_ran === false).length,
      2
    ),
    local_reports: gate(
      "local reports readable",
      input.updateReport.ok && input.sourceReview.ok,
      [input.updateReport.ok, input.sourceReview.ok].filter(Boolean).length,
      2
    ),
    release_bundle: gate(
      "release bundle present",
      Boolean(input.releaseBundle),
      input.releaseBundle ? 1 : 0,
      1
    ),
  };
}

function buildFindings(checks, rows, releaseBundle, updateReport, sourceReview) {
  const findings = [];
  for (const gateValue of Object.values(checks)) {
    if (!gateValue.passed) {
      findings.push(`${gateValue.name} did not pass (${gateValue.present}/${gateValue.expected})`);
    }
  }
  if (!releaseBundle) findings.push("no local Forge release bundle directory was found");
  if (!updateReport.ok) findings.push(`package update rehearsal report missing or invalid: ${updateReport.error}`);
  if (!sourceReview.ok) findings.push(`source-owned package review report missing or invalid: ${sourceReview.error}`);
  for (const row of rows) {
    if (row.package_install_ran) findings.push(`${row.id} unexpectedly ran a package install`);
    if (row.baseline_kind === "local-evidence" && (!Number.isFinite(row.time_ms) || row.time_ms <= 0)) {
      findings.push(`${row.id} has no positive local timing evidence`);
    }
    if (row.baseline_kind === "local-evidence" && (!Number.isFinite(row.artifact_bytes) || row.artifact_bytes <= 0)) {
      findings.push(`${row.id} has no positive artifact-size evidence`);
    }
  }
  return [...new Set(findings)];
}

function buildComparisons(rows) {
  const forgeInstall = rows.find((row) => row.id === "dx-forge-beta-install");
  const forgeUpgrade = rows.find((row) => row.id === "dx-forge-beta-upgrade");
  return rows
    .filter((row) => row.baseline_kind === "static-reference")
    .map((baseline) => ({
      baseline: baseline.id,
      install_time_delta_ms: diff(baseline.time_ms, forgeInstall?.time_ms),
      install_artifact_delta_bytes: diff(baseline.artifact_bytes, forgeInstall?.artifact_bytes),
      upgrade_time_delta_ms: diff(baseline.time_ms, forgeUpgrade?.time_ms),
      upgrade_artifact_delta_bytes: diff(baseline.artifact_bytes, forgeUpgrade?.artifact_bytes),
      caveat: "Positive deltas favor Forge only inside this static-reference snapshot.",
    }));
}

function renderMarkdown(report) {
  const lines = [
    "# DX Forge Installability Snapshot",
    "",
    `Generated: ${report.generated_at}`,
    `Score: \`${report.score}\` / \`100\``,
    `Passed: \`${report.passed}\``,
    "",
    "This is not a live npm/shadcn install benchmark; npm and shadcn rows are static references and no package installs are run.",
    "",
    "## Scope",
    "",
    `- Package installs run: \`${!report.scope.no_package_installs_run}\``,
    `- Live npm/shadcn benchmark: \`${!report.scope.not_live_npm_or_shadcn_benchmark}\``,
    `- Competitor builds run: \`${report.scope.competitor_builds_run}\``,
    `- Created node_modules: \`${!report.scope.no_node_modules_created}\``,
    `- Safe public claim: ${report.scope.safe_public_claim}`,
    "",
    "## Checks",
    "",
    "| Check | Passed | Present | Expected |",
    "| --- | --- | ---: | ---: |",
    ...Object.values(report.checks).map(
      (check) => `| ${check.name} | \`${check.passed}\` | ${check.present} | ${check.expected} |`
    ),
    "",
    "## Rows",
    "",
    "| ID | Operation | Kind | Time | Artifact Size | Package Install Ran | node_modules Required | Evidence |",
    "| --- | --- | --- | ---: | ---: | --- | --- | --- |",
    ...report.rows.map((row) =>
      [
        `\`${row.id}\``,
        row.operation,
        row.baseline_kind,
        formatMs(row.time_ms),
        formatBytes(row.artifact_bytes),
        `\`${row.package_install_ran}\``,
        `\`${row.node_modules_required}\``,
        markdownTableCell(row.evidence),
      ].join(" | ")
    ).map((row) => `| ${row} |`),
    "",
    "## Comparisons",
    "",
    "| Baseline | Install Time Delta | Install Artifact Delta | Upgrade Time Delta | Upgrade Artifact Delta |",
    "| --- | ---: | ---: | ---: | ---: |",
    ...report.comparisons.map((comparison) =>
      [
        `\`${comparison.baseline}\``,
        formatDeltaMs(comparison.install_time_delta_ms),
        formatDeltaBytes(comparison.install_artifact_delta_bytes),
        formatDeltaMs(comparison.upgrade_time_delta_ms),
        formatDeltaBytes(comparison.upgrade_artifact_delta_bytes),
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

function writeSnapshot(report, options = {}) {
  const outDir = path.resolve(options.outDir || reportDir);
  const jsonPath = path.join(outDir, defaultJsonName);
  const mdPath = path.join(outDir, defaultMdName);
  fs.mkdirSync(outDir, { recursive: true });
  fs.writeFileSync(jsonPath, `${JSON.stringify(report, null, 2)}\n`);
  fs.writeFileSync(mdPath, renderMarkdown(report));
  const provenance = buildSnapshotProvenance({
    outDir,
    files: [jsonPath, mdPath],
    generatedAt: options.provenanceGeneratedAt || report.generated_at,
    privateKeyPem: options.provenancePrivateKeyPem,
    signer: options.provenanceSigner || "dx-forge-local-publisher",
  });
  const provenanceJsonPath = path.join(outDir, defaultProvenanceJsonName);
  const provenanceMdPath = path.join(outDir, defaultProvenanceMdName);
  fs.writeFileSync(provenanceJsonPath, `${JSON.stringify(provenance, null, 2)}\n`);
  fs.writeFileSync(provenanceMdPath, renderSnapshotProvenanceMarkdown(provenance));
  const history = writeInstallabilityHistory(report, {
    outDir,
    historyDir: options.historyDir,
  });
  return {
    jsonPath,
    mdPath,
    provenanceJsonPath,
    provenanceMdPath,
    historyJsonPath: history.historyJsonPath,
    historyMdPath: history.historyMdPath,
    historySnapshotPath: history.snapshotPath,
  };
}

function writeInstallabilityHistory(report, options = {}) {
  const outDir = path.resolve(options.outDir || reportDir);
  const historyDir = path.resolve(options.historyDir || path.join(outDir, defaultHistoryDirName));
  const snapshotsDir = path.join(historyDir, "snapshots");
  fs.mkdirSync(snapshotsDir, { recursive: true });

  const snapshotPath = path.join(snapshotsDir, `${historySnapshotId(report.generated_at)}.json`);
  fs.writeFileSync(snapshotPath, `${JSON.stringify(report, null, 2)}\n`);

  const snapshotReports = fs
    .readdirSync(snapshotsDir, { withFileTypes: true })
    .filter((entry) => entry.isFile() && entry.name.endsWith(".json"))
    .map((entry) => {
      const filePath = path.join(snapshotsDir, entry.name);
      return {
        path: filePath,
        report: JSON.parse(fs.readFileSync(filePath, "utf8")),
      };
    })
    .sort((left, right) => String(left.report.generated_at).localeCompare(String(right.report.generated_at)));

  const snapshots = snapshotReports.map((entry, index) => {
    const previous = index > 0 ? snapshotReports[index - 1] : null;
    return buildInstallabilityHistoryRecord(entry.report, {
      snapshotPath: entry.path,
      historyDir,
      previous: previous?.report || null,
    });
  });

  const history = {
    version: 1,
    generated_at: report.generated_at,
    report_id: "forge-installability-trend-history-v1",
    source: "benchmarks/measure-forge-installability-snapshot.ts",
    snapshot_count: snapshots.length,
    latest: snapshots.at(-1) || null,
    previous: snapshots.length > 1 ? snapshots.at(-2) : null,
    snapshots,
    checks: buildInstallabilityHistoryChecks(snapshots),
    honest_scope: [
      "Trend history compares local Forge installability snapshots only.",
      "Negative time and byte deltas mean the newer snapshot is smaller or faster than the previous snapshot.",
      "npm and shadcn rows remain static references; this history does not run package installs.",
      "Each history snapshot is copied into the history folder so release-to-release movement can be reviewed later.",
    ],
  };

  const historyJsonPath = path.join(historyDir, defaultHistoryJsonName);
  const historyMdPath = path.join(historyDir, defaultHistoryMdName);
  fs.writeFileSync(historyJsonPath, `${JSON.stringify(history, null, 2)}\n`);
  fs.writeFileSync(historyMdPath, renderInstallabilityHistoryMarkdown(history));
  return { historyJsonPath, historyMdPath, snapshotPath };
}

function buildInstallabilityHistoryRecord(report, options) {
  const rowsById = Object.fromEntries((report.rows || []).map((row) => [row.id, row]));
  const previousRowsById = options.previous
    ? Object.fromEntries((options.previous.rows || []).map((row) => [row.id, row]))
    : {};
  const installRow = rowsById["dx-forge-beta-install"] || {};
  const upgradeRow = rowsById["dx-forge-beta-upgrade"] || {};
  const previousInstallRow = previousRowsById["dx-forge-beta-install"] || {};
  const previousUpgradeRow = previousRowsById["dx-forge-beta-upgrade"] || {};
  const delta = options.previous
    ? {
        score: diff(report.score, options.previous.score),
        install_time_ms: diff(installRow.time_ms, previousInstallRow.time_ms),
        upgrade_time_ms: diff(upgradeRow.time_ms, previousUpgradeRow.time_ms),
        install_artifact_bytes: diff(installRow.artifact_bytes, previousInstallRow.artifact_bytes),
        upgrade_artifact_bytes: diff(upgradeRow.artifact_bytes, previousUpgradeRow.artifact_bytes),
      }
    : {
        score: null,
        install_time_ms: null,
        upgrade_time_ms: null,
        install_artifact_bytes: null,
        upgrade_artifact_bytes: null,
      };

  return {
    id: historySnapshotId(report.generated_at),
    generated_at: report.generated_at,
    snapshot_path: displayPath(options.snapshotPath, options.historyDir).replace(/\\/g, "/"),
    passed: report.passed,
    score: report.score,
    beta_install_time_ms: installRow.time_ms ?? null,
    beta_upgrade_time_ms: upgradeRow.time_ms ?? null,
    beta_install_artifact_bytes: installRow.artifact_bytes ?? null,
    beta_upgrade_artifact_bytes: upgradeRow.artifact_bytes ?? null,
    no_package_installs_run: report.scope?.no_package_installs_run === true,
    no_node_modules_created: report.scope?.no_node_modules_created === true,
    release_bundle: report.sources?.release_bundle || null,
    finding_count: Array.isArray(report.findings) ? report.findings.length : 0,
    delta,
    trend: installabilityTrend(delta),
  };
}

function buildInstallabilityHistoryChecks(snapshots) {
  const latest = snapshots.at(-1) || null;
  return {
    has_history: gate("installability history", snapshots.length > 0, snapshots.length, 1),
    latest_passed: gate("latest snapshot passed", latest?.passed === true, latest?.passed === true ? 1 : 0, 1),
    no_package_installs: gate(
      "no package installs across history",
      snapshots.every((snapshot) => snapshot.no_package_installs_run && snapshot.no_node_modules_created),
      snapshots.filter((snapshot) => snapshot.no_package_installs_run && snapshot.no_node_modules_created).length,
      snapshots.length
    ),
  };
}

function renderInstallabilityHistoryMarkdown(history) {
  const lines = [
    "# DX Forge Installability Trend History",
    "",
    `Generated: ${history.generated_at}`,
    `Snapshots: \`${history.snapshot_count}\``,
    `Latest trend: \`${history.latest?.trend || "n/a"}\``,
    "",
    "Negative deltas mean the newer snapshot is faster or smaller than the previous snapshot.",
    "",
    "## Checks",
    "",
    "| Check | Passed | Present | Expected |",
    "| --- | --- | ---: | ---: |",
    ...Object.values(history.checks).map(
      (check) => `| ${check.name} | \`${check.passed}\` | ${check.present} | ${check.expected} |`
    ),
    "",
    "## Snapshots",
    "",
    "| Generated | Passed | Score | Install | Delta | Upgrade | Delta | Trend | Snapshot |",
    "| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- |",
    ...history.snapshots.map((snapshot) =>
      [
        snapshot.generated_at,
        `\`${snapshot.passed}\``,
        snapshot.score,
        formatMs(snapshot.beta_install_time_ms),
        formatDeltaMs(snapshot.delta.install_time_ms),
        formatMs(snapshot.beta_upgrade_time_ms),
        formatDeltaMs(snapshot.delta.upgrade_time_ms),
        `\`${snapshot.trend}\``,
        `\`${snapshot.snapshot_path}\``,
      ].join(" | ")
    ).map((row) => `| ${row} |`),
    "",
    "## Honest Scope",
    "",
    ...history.honest_scope.map((item) => `- ${item}`),
    "",
  ];
  return lines.join("\n");
}

function installabilityTrend(delta) {
  if (!Number.isFinite(delta.install_time_ms) && !Number.isFinite(delta.upgrade_time_ms)) {
    return "baseline";
  }
  if (
    [delta.score, delta.install_time_ms, delta.upgrade_time_ms, delta.install_artifact_bytes, delta.upgrade_artifact_bytes]
      .every((value) => value === null || value === 0)
  ) {
    return "stable";
  }
  if (
    (Number.isFinite(delta.score) && delta.score < 0) ||
    (Number.isFinite(delta.install_time_ms) && delta.install_time_ms > 0) ||
    (Number.isFinite(delta.upgrade_time_ms) && delta.upgrade_time_ms > 0) ||
    (Number.isFinite(delta.install_artifact_bytes) && delta.install_artifact_bytes > 0) ||
    (Number.isFinite(delta.upgrade_artifact_bytes) && delta.upgrade_artifact_bytes > 0)
  ) {
    return "regressed";
  }
  return "improved";
}

function historySnapshotId(generatedAt) {
  return String(generatedAt || new Date().toISOString())
    .replace(/\.\d+Z$/, (match) => match.replace(".", "-").replace("Z", "Z"))
    .replace(/[:]/g, "-")
    .replace(/[^A-Za-z0-9TZ-]/g, "-")
    .replace(/-+/g, "-")
    .replace(/-Z$/, "Z");
}

function buildSnapshotProvenance(options) {
  const outDir = path.resolve(options.outDir);
  const generatedAt = options.generatedAt || new Date().toISOString();
  const artifacts = options.files.map((filePath) => {
    const absolutePath = path.resolve(filePath);
    const raw = fs.readFileSync(absolutePath);
    return {
      path: displayPath(absolutePath, outDir).replace(/\\/g, "/"),
      artifact_type: path.extname(absolutePath) === ".md" ? "benchmark-snapshot-markdown" : "benchmark-snapshot-json",
      bytes: raw.byteLength,
      sha256: crypto.createHash("sha256").update(raw).digest("hex"),
    };
  }).sort((left, right) => left.path.localeCompare(right.path));
  const manifestDigest = crypto
    .createHash("sha256")
    .update(JSON.stringify(artifacts))
    .digest("hex");
  const provenance = {
    version: 1,
    generated_at: generatedAt,
    report_id: "forge-installability-snapshot-provenance-v1",
    source: "benchmarks/measure-forge-installability-snapshot.ts",
    hash_algorithm: "sha256",
    artifact_count: artifacts.length,
    artifacts,
    manifest_digest: manifestDigest,
    signed: false,
    signer: null,
    key_id: null,
    algorithm: null,
    public_key: null,
    signature: null,
    signature_payload: null,
    signature_verified: false,
    message:
      "Unsigned snapshot provenance: SHA-256 artifact hashes are verified locally; pass --provenance-private-key to sign.",
  };

  if (options.privateKeyPem) {
    const privateKey = crypto.createPrivateKey(options.privateKeyPem);
    const publicKey = crypto.createPublicKey(privateKey);
    const publicKeyPem = publicKey.export({ type: "spki", format: "pem" });
    const keyId = `ed25519-sha256:${crypto.createHash("sha256").update(publicKeyPem).digest("hex")}`;
    const signer = String(options.signer || "dx-forge-local-publisher").trim() || "dx-forge-local-publisher";
    const signaturePayload = snapshotProvenanceSigningPayload({
      version: provenance.version,
      manifestDigest,
      artifactCount: artifacts.length,
      signer,
      keyId,
      publicKey: publicKeyPem,
      generatedAt,
    });
    const signature = crypto.sign(null, Buffer.from(signaturePayload), privateKey).toString("base64");
    provenance.signed = true;
    provenance.signer = signer;
    provenance.key_id = keyId;
    provenance.algorithm = "ed25519";
    provenance.public_key = publicKeyPem;
    provenance.signature = `ed25519:${signature}`;
    provenance.signature_payload = signaturePayload;
    provenance.signature_verified = verifySnapshotProvenanceSignature(provenance);
    provenance.message =
      "Signed snapshot provenance: Ed25519 publisher identity covers the installability snapshot artifact digest.";
  }

  return provenance;
}

function verifySnapshotProvenance(provenancePath) {
  const rootDir = path.dirname(path.resolve(provenancePath));
  const findings = [];
  let provenance;
  try {
    provenance = JSON.parse(fs.readFileSync(provenancePath, "utf8"));
  } catch (error) {
    return { passed: false, findings: [`provenance unreadable: ${error.message}`] };
  }

  const artifacts = Array.isArray(provenance.artifacts) ? provenance.artifacts : [];
  const normalizedArtifacts = [];
  for (const artifact of artifacts) {
    const relative = String(artifact.path || "");
    if (!relative || path.isAbsolute(relative) || relative.split(/[\\/]/).includes("..")) {
      findings.push(`unsafe artifact path: ${relative || "<empty>"}`);
      continue;
    }
    const artifactPath = path.join(rootDir, relative);
    let raw;
    try {
      raw = fs.readFileSync(artifactPath);
    } catch (error) {
      findings.push(`${relative} unreadable: ${error.message}`);
      continue;
    }
    const sha256 = crypto.createHash("sha256").update(raw).digest("hex");
    if (sha256 !== artifact.sha256) {
      findings.push(`${relative} hash mismatch`);
    }
    if (raw.byteLength !== artifact.bytes) {
      findings.push(`${relative} byte count mismatch`);
    }
    normalizedArtifacts.push({
      path: relative.replace(/\\/g, "/"),
      artifact_type: artifact.artifact_type,
      bytes: artifact.bytes,
      sha256: artifact.sha256,
    });
  }

  normalizedArtifacts.sort((left, right) => left.path.localeCompare(right.path));
  const manifestDigest = crypto
    .createHash("sha256")
    .update(JSON.stringify(normalizedArtifacts))
    .digest("hex");
  if (manifestDigest !== provenance.manifest_digest) {
    findings.push("manifest digest mismatch");
  }
  if (provenance.signed && !verifySnapshotProvenanceSignature(provenance)) {
    findings.push("signature verification failed");
  }

  return {
    passed: findings.length === 0,
    signed: provenance.signed === true,
    signature_verified: provenance.signed === true ? verifySnapshotProvenanceSignature(provenance) : false,
    artifact_count: artifacts.length,
    findings,
  };
}

function verifySnapshotProvenanceSignature(provenance) {
  if (
    provenance.signed !== true ||
    provenance.algorithm !== "ed25519" ||
    typeof provenance.public_key !== "string" ||
    typeof provenance.signature !== "string" ||
    typeof provenance.signature_payload !== "string"
  ) {
    return false;
  }
  const signature = provenance.signature.startsWith("ed25519:")
    ? provenance.signature.slice("ed25519:".length)
    : provenance.signature;
  try {
    return crypto.verify(
      null,
      Buffer.from(provenance.signature_payload),
      crypto.createPublicKey(provenance.public_key),
      Buffer.from(signature, "base64")
    );
  } catch {
    return false;
  }
}

function snapshotProvenanceSigningPayload(input) {
  return [
    "dx-forge-report-provenance-v1",
    `version=${input.version}`,
    "hash_algorithm=sha256",
    `manifest_digest=${input.manifestDigest}`,
    `artifact_count=${input.artifactCount}`,
    `signer=${input.signer}`,
    `key_id=${input.keyId}`,
    `public_key=${input.publicKey}`,
    `generated_at=${input.generatedAt}`,
    "",
  ].join("\n");
}

function renderSnapshotProvenanceMarkdown(provenance) {
  const lines = [
    "# DX Forge Installability Snapshot Provenance",
    "",
    `Generated: ${provenance.generated_at}`,
    `Signed: \`${provenance.signed}\``,
    `Signature verified: \`${provenance.signature_verified}\``,
    `Signer: \`${provenance.signer || "not attached"}\``,
    `Key id: \`${provenance.key_id || "not attached"}\``,
    `Manifest digest: \`${provenance.manifest_digest}\``,
    "",
    "## Artifacts",
    "",
    "| Path | Type | Bytes | SHA-256 |",
    "| --- | --- | ---: | --- |",
    ...provenance.artifacts.map(
      (artifact) =>
        `| \`${artifact.path}\` | \`${artifact.artifact_type}\` | ${artifact.bytes} | \`${artifact.sha256}\` |`
    ),
    "",
    "## Message",
    "",
    `- ${provenance.message}`,
    "",
  ];
  return lines.join("\n");
}

function defaultReleaseBundleDirs(rootDir) {
  return [
    path.join(rootDir, ".dx", "ci", "forge-release-bundle-adoption"),
    path.join(rootDir, ".dx", "forge-release-bundle-adoption"),
    path.join(rootDir, ".dx", "adoption-update-rehearsal", ".dx", "forge", "adoption-smoke", "release-bundle"),
    path.join(rootDir, ".dx", "adoption-package-review", ".dx", "forge", "adoption-smoke", "release-bundle"),
    path.join(rootDir, ".dx", "adoption-browser-smoke", ".dx", "forge", "adoption-smoke", "release-bundle"),
  ];
}

function findFirstExistingDir(candidates) {
  return candidates.map((candidate) => path.resolve(candidate)).find((candidate) => {
    try {
      return fs.statSync(candidate).isDirectory();
    } catch {
      return false;
    }
  }) || null;
}

function summarizeDirectory(dir) {
  if (!dir) {
    return {
      path: null,
      total_bytes: 0,
      file_count: 0,
    };
  }
  const files = listFiles(dir);
  return {
    path: dir,
    total_bytes: files.reduce((total, file) => total + fs.statSync(file).size, 0),
    file_count: files.length,
  };
}

function summarizeFiles(files) {
  return {
    files,
    total_bytes: files.reduce((total, file) => total + fs.statSync(file).size, 0),
    file_count: files.length,
  };
}

function listFiles(dir) {
  const found = [];
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      if (entry.name !== "node_modules") found.push(...listFiles(fullPath));
    } else if (entry.isFile()) {
      found.push(fullPath);
    }
  }
  return found;
}

function readJsonFile(filePath) {
  if (!fs.existsSync(filePath)) {
    return { ok: false, value: null, error: `${filePath} does not exist` };
  }
  try {
    return { ok: true, value: JSON.parse(fs.readFileSync(filePath, "utf8")), error: null };
  } catch (error) {
    return { ok: false, value: null, error: `${filePath}: ${error.message}` };
  }
}

function gate(name, passed, present, expected) {
  return {
    name,
    passed,
    present,
    expected,
    score: expected === 0 ? 0 : Math.min(100, Math.floor((Number(present) * 100) / expected)),
  };
}

function scoreSnapshot(checks, findings) {
  let score = 100;
  for (const check of Object.values(checks)) {
    if (!check.passed) score -= 15;
  }
  score -= Math.max(0, findings.length - 1) * 2;
  return Math.max(0, Math.min(100, score));
}

function numberOrNull(value) {
  const number = Number(value);
  return Number.isFinite(number) && number >= 0 ? number : null;
}

function diff(left, right) {
  if (!Number.isFinite(left) || !Number.isFinite(right)) return null;
  return left - right;
}

function displayPath(filePath, rootDir) {
  if (!filePath) return null;
  const relative = path.relative(rootDir, filePath);
  return relative && !relative.startsWith("..") ? relative : filePath;
}

function formatMs(value) {
  return Number.isFinite(value) ? `${value} ms` : "n/a";
}

function formatBytes(value) {
  return Number.isFinite(value) ? `${value} B` : "n/a";
}

function formatDeltaMs(value) {
  return Number.isFinite(value) ? `${value} ms` : "n/a";
}

function formatDeltaBytes(value) {
  return Number.isFinite(value) ? `${value} B` : "n/a";
}

function markdownTableCell(value) {
  return String(value || "-").replace(/\|/g, "\\|").replace(/\r?\n/g, " ");
}

function parseArgs(argv) {
  const options = {};
  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === "--out") {
      options.outDir = argv[index + 1];
      index += 1;
    } else if (arg === "--root") {
      options.rootDir = argv[index + 1];
      index += 1;
    } else if (arg === "--history-dir") {
      options.historyDir = argv[index + 1];
      index += 1;
    } else if (arg === "--provenance-private-key") {
      options.provenancePrivateKeyPem = fs.readFileSync(argv[index + 1], "utf8");
      index += 1;
    } else if (arg === "--provenance-signer") {
      options.provenanceSigner = argv[index + 1];
      index += 1;
    }
  }
  return options;
}

function main() {
  const options = parseArgs(process.argv.slice(2));
  const report = buildSnapshot({ rootDir: options.rootDir });
  const written = writeSnapshot(report, {
    outDir: options.outDir,
    historyDir: options.historyDir,
    provenancePrivateKeyPem: options.provenancePrivateKeyPem,
    provenanceSigner: options.provenanceSigner,
  });
  console.log(
    JSON.stringify(
      {
        report: [path.relative(root, written.jsonPath), path.relative(root, written.mdPath)],
        provenance: [
          path.relative(root, written.provenanceJsonPath),
          path.relative(root, written.provenanceMdPath),
        ],
        history: [
          path.relative(root, written.historyJsonPath),
          path.relative(root, written.historyMdPath),
          path.relative(root, written.historySnapshotPath),
        ],
        score: report.score,
        passed: report.passed,
        no_package_installs_run: report.scope.no_package_installs_run,
        not_live_npm_or_shadcn_benchmark: report.scope.not_live_npm_or_shadcn_benchmark,
      },
      null,
      2
    )
  );
  if (!report.passed) process.exitCode = 1;
}

if (require.main === module) {
  main();
}

module.exports = {
  buildSnapshot,
  buildSnapshotProvenance,
  buildInstallabilityHistoryRecord,
  renderMarkdown,
  renderInstallabilityHistoryMarkdown,
  renderSnapshotProvenanceMarkdown,
  verifySnapshotProvenance,
  writeInstallabilityHistory,
  writeSnapshot,
};
