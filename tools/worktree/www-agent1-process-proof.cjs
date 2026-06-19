"use strict";

const fs = require("node:fs");
const path = require("node:path");
const { execFileSync } = require("node:child_process");

const REPORT_SCHEMA = "dx.www.agent1ProcessProof";
const DEFAULT_PORT = 3000;
const DEFAULT_STALE_AGE_MINUTES = 45;
const HEAVY_PROCESS_NAMES = new Set([
  "cargo.exe",
  "cargo-clippy.exe",
  "rustc.exe",
  "dx-www.exe",
  "node.exe",
]);

function summarizeProcessProof(snapshot = {}, options = {}) {
  const staleAgeMinutes = numberOrDefault(options.staleAgeMinutes, DEFAULT_STALE_AGE_MINUTES);
  const processes = normalizeProcesses(snapshot.processes || []);
  const port = normalizePort(snapshot.port || snapshot.port3000 || {}, numberOrDefault(options.port, DEFAULT_PORT));
  const disk = normalizeDisk(snapshot.disk || []);
  const binary = normalizeBinary(snapshot.binary || {}, snapshot.latestSource || {});
  const childrenByParent = buildChildrenByParent(processes);
  const classified = processes.map((process) => ({
    ...process,
    classification: classifyProcess(process),
  }));
  const dxCargoLike = classified.filter((process) =>
    ["cargo-build", "cargo-check", "cargo-clippy", "cargo-test", "rustc-dx-www"].includes(
      process.classification,
    ),
  );
  const activeRustc = classified.filter((process) => process.classification === "rustc-dx-www");
  const activeBuilds = classified.filter((process) => process.classification === "cargo-build");
  const staleCandidates = classified.filter((process) =>
    isStaleCandidate(process, classified, childrenByParent, staleAgeMinutes),
  );
  const portOwner = port.listener
    ? classified.find((process) => process.processId === port.listener.owningProcess) || null
    : null;
  const warnings = buildWarnings({
    activeBuilds,
    activeRustc,
    binary,
    port,
    portOwner,
    staleCandidates,
  });
  const recommendedAction = chooseRecommendedAction({
    activeBuilds,
    activeRustc,
    binary,
    dxCargoLike,
    port,
    portOwner,
    staleCandidates,
  });
  const nextAction = describeNextAction(recommendedAction, {
    activeBuilds,
    activeRustc,
    binary,
    dxCargoLike,
    port,
    portOwner,
    staleCandidates,
  });

  return {
    schema: REPORT_SCHEMA,
    format: 1,
    generatedAt: snapshot.generatedAt || new Date().toISOString(),
    recommendedAction,
    nextAction,
    processCounts: countByClassification(classified),
    activeDxWwwProofProcessCount: dxCargoLike.length,
    port,
    binary,
    disk,
    staleCandidates: staleCandidates.map(summarizeProcess),
    activeProofProcesses: dxCargoLike.map(summarizeProcess),
    warnings,
  };
}

function classifyProcess(process) {
  const name = String(process.name || "").toLowerCase();
  const commandLine = String(process.commandLine || "");
  const lowerCommand = commandLine.toLowerCase();
  const isDxPackage = /(?:^|\s)-p\s+dx-www(?:\s|$)/i.test(commandLine);

  if (name === "rustc.exe" && /(?:--crate-name\s+dx_www|\bdx-www\\src\b|\bdx-www\/src\b)/i.test(commandLine)) {
    return "rustc-dx-www";
  }
  if (name === "dx-www.exe") {
    return "dx-www-server";
  }
  if (!isDxPackage) {
    return name === "node.exe" ? "node-tool" : "other";
  }
  if (/\bbuild\s+-p\s+dx-www\b/i.test(lowerCommand)) {
    return "cargo-build";
  }
  if (/\bcheck\s+-p\s+dx-www\b/i.test(lowerCommand)) {
    return "cargo-check";
  }
  if (/\bclippy\s+-p\s+dx-www\b/i.test(lowerCommand) || name === "cargo-clippy.exe") {
    return "cargo-clippy";
  }
  if (/\btest\s+-p\s+dx-www\b/i.test(lowerCommand)) {
    return "cargo-test";
  }
  return "cargo-dx-www";
}

function chooseRecommendedAction(evidence) {
  if (evidence.port.listener && evidence.portOwner?.classification !== "dx-www-server") {
    return "do-not-reuse-port";
  }
  if (evidence.activeRustc.length > 0) {
    return "wait-for-active-rustc";
  }
  if (evidence.activeBuilds.length > 0 && evidence.binary.fresh === false) {
    return "wait-for-active-binary-build";
  }
  if (evidence.staleCandidates.length > 0) {
    return "review-stale-heavy-processes";
  }
  if (evidence.dxCargoLike.length > 0) {
    return "wait-for-active-cargo-proof";
  }
  if (evidence.binary.fresh === false) {
    return "run-one-fresh-cargo-build";
  }
  if (!evidence.port.listener) {
    return "start-dev-server-from-fresh-binary";
  }
  return "safe-for-focused-proof";
}

function describeNextAction(recommendedAction, evidence) {
  const actions = {
    "do-not-reuse-port": {
      safeToStartBuild: false,
      safeToStartDevServer: false,
      requiresHumanReview: true,
      command: null,
      reason: `port ${evidence.port.port} is owned by PID ${evidence.port.listener?.owningProcess}`,
    },
    "wait-for-active-rustc": {
      safeToStartBuild: false,
      safeToStartDevServer: false,
      requiresHumanReview: false,
      command: "node tools\\worktree\\www-agent1-process-proof.cjs --sample-seconds 1",
      reason: "active dx-www rustc work is still compiling",
    },
    "wait-for-active-binary-build": {
      safeToStartBuild: false,
      safeToStartDevServer: false,
      requiresHumanReview: false,
      command: "node tools\\worktree\\www-agent1-process-proof.cjs --sample-seconds 1",
      reason: "an active dx-www cargo build may still produce the fresh binary",
    },
    "review-stale-heavy-processes": {
      safeToStartBuild: false,
      safeToStartDevServer: false,
      requiresHumanReview: true,
      command: null,
      reason: "idle cargo processes crossed the stale threshold; review exact PIDs before killing anything",
    },
    "wait-for-active-cargo-proof": {
      safeToStartBuild: false,
      safeToStartDevServer: false,
      requiresHumanReview: false,
      command: "node tools\\worktree\\www-agent1-process-proof.cjs --sample-seconds 1",
      reason: "dx-www cargo proof commands are still running",
    },
    "run-one-fresh-cargo-build": {
      safeToStartBuild: true,
      safeToStartDevServer: false,
      requiresHumanReview: false,
      command: "cargo build -p dx-www --no-default-features --features cli --bin dx-www -j 1",
      reason: "no active dx-www proof process is blocking, but the binary is stale or missing",
    },
    "start-dev-server-from-fresh-binary": {
      safeToStartBuild: false,
      safeToStartDevServer: true,
      requiresHumanReview: false,
      command: "target\\debug\\dx-www.exe dev --host 127.0.0.1 --port 3000",
      reason: "binary is fresh and port 3000 is free",
    },
    "safe-for-focused-proof": {
      safeToStartBuild: false,
      safeToStartDevServer: false,
      requiresHumanReview: false,
      command: null,
      reason: "no process, port, or binary freshness blocker was detected",
    },
  };

  return actions[recommendedAction] || {
    safeToStartBuild: false,
    safeToStartDevServer: false,
    requiresHumanReview: true,
    command: null,
    reason: `unrecognized recommendation: ${recommendedAction}`,
  };
}

function buildWarnings(evidence) {
  const warnings = [];
  if (evidence.port.listener && evidence.portOwner?.classification !== "dx-www-server") {
    warnings.push({
      id: "port-owned-by-non-dx-www",
      message: `port ${evidence.port.port} is already owned by PID ${evidence.port.listener.owningProcess}`,
    });
  }
  if (!evidence.port.listener) {
    warnings.push({
      id: "port-not-listening",
      message: `port ${evidence.port.port} has no listener`,
    });
  }
  if (evidence.binary.fresh === false) {
    warnings.push({
      id: "binary-stale",
      message: "target/debug/dx-www.exe is older than the newest dx-www Rust source",
    });
  }
  if (evidence.activeRustc.length > 0) {
    warnings.push({
      id: "active-rustc",
      message: "a dx-www rustc process is active; do not start another heavy proof yet",
    });
  }
  if (evidence.staleCandidates.length > 0) {
    warnings.push({
      id: "stale-candidate-review-required",
      message: "idle cargo processes crossed the stale threshold and need human review before killing",
    });
  }
  return warnings;
}

function isStaleCandidate(process, processes, childrenByParent, staleAgeMinutes) {
  if (!["cargo-build", "cargo-check", "cargo-clippy", "cargo-test", "cargo-dx-www"].includes(process.classification)) {
    return false;
  }
  if (process.ageMinutes < staleAgeMinutes) {
    return false;
  }
  if (typeof process.cpuDeltaSeconds !== "number" || process.cpuDeltaSeconds > 0.01) {
    return false;
  }
  return !hasDescendantRustc(process.processId, processes, childrenByParent);
}

function hasDescendantRustc(processId, processes, childrenByParent, seen = new Set()) {
  if (seen.has(processId)) {
    return false;
  }
  seen.add(processId);
  for (const child of childrenByParent.get(processId) || []) {
    if (child.classification === "rustc-dx-www" || classifyProcess(child) === "rustc-dx-www") {
      return true;
    }
    if (hasDescendantRustc(child.processId, processes, childrenByParent, seen)) {
      return true;
    }
  }
  return false;
}

function collectLiveSnapshot(options = {}) {
  const repoRoot = path.resolve(options.repoRoot || process.cwd());
  const port = numberOrDefault(options.port, DEFAULT_PORT);
  const sampleSeconds = Math.max(0, numberOrDefault(options.sampleSeconds, 0));
  const firstProcesses = collectWindowsProcesses();
  let processes = firstProcesses;

  if (sampleSeconds > 0) {
    sleep(sampleSeconds * 1000);
    const secondByPid = new Map(collectWindowsProcesses().map((process) => [process.processId, process]));
    processes = firstProcesses.map((process) => {
      const second = secondByPid.get(process.processId);
      return {
        ...process,
        state: second ? "running" : "exited-during-sample",
        cpuDeltaSeconds:
          second && typeof process.cpu === "number" && typeof second.cpu === "number"
            ? round(second.cpu - process.cpu, 2)
            : null,
      };
    });
  }

  return {
    generatedAt: new Date().toISOString(),
    processes,
    port: collectWindowsPort(port),
    disk: collectWindowsDisk(["C", "F", "G"]),
    binary: fileEvidence(path.join(repoRoot, "target", "debug", "dx-www.exe")),
    latestSource: latestRustSource(path.join(repoRoot, "dx-www", "src")),
  };
}

function collectWindowsProcesses() {
  if (process.platform !== "win32") {
    return [];
  }
  const script = `
$now = Get-Date
$names = @("cargo.exe","cargo-clippy.exe","rustc.exe","dx-www.exe","node.exe")
$rows = Get-CimInstance Win32_Process | Where-Object { $names -contains $_.Name } | ForEach-Object {
  $proc = Get-Process -Id $_.ProcessId -ErrorAction SilentlyContinue
  [pscustomobject]@{
    processId = $_.ProcessId
    parentProcessId = $_.ParentProcessId
    name = $_.Name
    ageMinutes = [math]::Round(($now - $_.CreationDate).TotalMinutes, 2)
    cpu = if ($proc) { [double]$proc.CPU } else { $null }
    commandLine = $_.CommandLine
  }
}
@($rows) | ConvertTo-Json -Depth 5 -Compress
`;
  return parseJsonArray(execPowerShell(script));
}

function collectWindowsPort(port) {
  if (process.platform !== "win32") {
    return { port, listeners: [], listener: null };
  }
  const script = `
$rows = Get-NetTCPConnection -LocalPort ${port} -State Listen -ErrorAction SilentlyContinue | ForEach-Object {
  [pscustomobject]@{
    state = $_.State.ToString()
    localAddress = $_.LocalAddress
    localPort = $_.LocalPort
    owningProcess = $_.OwningProcess
  }
}
@($rows) | ConvertTo-Json -Depth 5 -Compress
`;
  return normalizePort({ port, listeners: parseJsonArray(execPowerShell(script)) }, port);
}

function collectWindowsDisk(names) {
  if (process.platform !== "win32") {
    return [];
  }
  const nameList = names.map((name) => `"${name}"`).join(",");
  const script = `
$rows = Get-PSDrive -Name ${nameList} -ErrorAction SilentlyContinue | ForEach-Object {
  [pscustomobject]@{
    name = $_.Name
    freeGb = [math]::Round($_.Free / 1GB, 2)
    usedGb = [math]::Round($_.Used / 1GB, 2)
  }
}
@($rows) | ConvertTo-Json -Depth 5 -Compress
`;
  return normalizeDisk(parseJsonArray(execPowerShell(script)));
}

function normalizeProcesses(processes) {
  return processes
    .map((process) => ({
      processId: Number(process.processId ?? process.ProcessId),
      parentProcessId: Number(process.parentProcessId ?? process.ParentProcessId ?? 0),
      name: String(process.name ?? process.Name ?? ""),
      ageMinutes: numberOrDefault(process.ageMinutes ?? process.AgeMinutes, 0),
      cpu: maybeNumber(process.cpu ?? process.CPU),
      cpuDeltaSeconds: maybeNumber(process.cpuDeltaSeconds ?? process.CpuDeltaSeconds),
      commandLine: String(process.commandLine ?? process.CommandLine ?? ""),
      state: process.state || process.State || "running",
    }))
    .filter((process) => Number.isFinite(process.processId) && HEAVY_PROCESS_NAMES.has(process.name.toLowerCase()));
}

function normalizePort(portEvidence, fallbackPort) {
  const port = numberOrDefault(portEvidence.port ?? portEvidence.localPort, fallbackPort);
  const rawListeners = portEvidence.listeners || (portEvidence.listener ? [portEvidence.listener] : []);
  const listeners = rawListeners
    .map((listener) => ({
      state: String(listener.state ?? listener.State ?? ""),
      localAddress: String(listener.localAddress ?? listener.LocalAddress ?? ""),
      localPort: Number(listener.localPort ?? listener.LocalPort ?? port),
      owningProcess: Number(listener.owningProcess ?? listener.OwningProcess),
    }))
    .filter((listener) => Number.isFinite(listener.owningProcess));
  return {
    port,
    listeners,
    listener: listeners[0] || null,
  };
}

function normalizeBinary(binary, latestSource) {
  const binaryTime = parseTime(binary.lastWriteTime ?? binary.LastWriteTime);
  const sourceTime = parseTime(latestSource.lastWriteTime ?? latestSource.LastWriteTime);
  const exists = Boolean(binary.exists ?? binary.Exists ?? binary.path ?? binary.fullName ?? binary.FullName);
  return {
    exists,
    path: binary.path || binary.fullName || binary.FullName || null,
    lastWriteTime: binaryTime ? new Date(binaryTime).toISOString() : null,
    size: maybeNumber(binary.size ?? binary.Length),
    latestSourcePath: latestSource.path || latestSource.fullName || latestSource.FullName || null,
    latestSourceWriteTime: sourceTime ? new Date(sourceTime).toISOString() : null,
    fresh: exists && binaryTime && sourceTime ? binaryTime >= sourceTime : false,
  };
}

function normalizeDisk(diskRows) {
  return diskRows.map((row) => ({
    name: String(row.name ?? row.Name ?? ""),
    freeGb: numberOrDefault(row.freeGb ?? row.FreeGB, 0),
    usedGb: numberOrDefault(row.usedGb ?? row.UsedGB, 0),
  }));
}

function buildChildrenByParent(processes) {
  const map = new Map();
  for (const process of processes) {
    const children = map.get(process.parentProcessId) || [];
    children.push(process);
    map.set(process.parentProcessId, children);
  }
  return map;
}

function countByClassification(processes) {
  const counts = {};
  for (const process of processes) {
    counts[process.classification] = (counts[process.classification] || 0) + 1;
  }
  return counts;
}

function summarizeProcess(process) {
  return {
    processId: process.processId,
    parentProcessId: process.parentProcessId,
    name: process.name,
    ageMinutes: process.ageMinutes,
    cpu: process.cpu,
    cpuDeltaSeconds: process.cpuDeltaSeconds,
    classification: process.classification,
    commandLine: process.commandLine,
  };
}

function fileEvidence(filePath) {
  try {
    const stat = fs.statSync(filePath);
    return {
      exists: true,
      path: filePath,
      lastWriteTime: stat.mtime.toISOString(),
      size: stat.size,
    };
  } catch {
    return { exists: false, path: filePath };
  }
}

function latestRustSource(sourceRoot) {
  let latest = null;
  walkFiles(sourceRoot, (filePath) => {
    if (!filePath.endsWith(".rs")) {
      return;
    }
    const stat = fs.statSync(filePath);
    if (!latest || stat.mtimeMs > latest.mtimeMs) {
      latest = {
        path: filePath,
        lastWriteTime: stat.mtime.toISOString(),
        mtimeMs: stat.mtimeMs,
      };
    }
  });
  return latest || {};
}

function walkFiles(root, visit) {
  if (!fs.existsSync(root)) {
    return;
  }
  for (const entry of fs.readdirSync(root, { withFileTypes: true })) {
    const fullPath = path.join(root, entry.name);
    if (entry.isDirectory()) {
      walkFiles(fullPath, visit);
    } else if (entry.isFile()) {
      visit(fullPath);
    }
  }
}

function execPowerShell(script) {
  return execFileSync("powershell", ["-NoProfile", "-Command", script], {
    encoding: "utf8",
    maxBuffer: 1024 * 1024 * 8,
    windowsHide: true,
  }).trim();
}

function parseJsonArray(stdout) {
  if (!stdout) {
    return [];
  }
  const parsed = JSON.parse(stdout);
  return Array.isArray(parsed) ? parsed : [parsed];
}

function parseTime(value) {
  if (!value) {
    return null;
  }
  const parsed = Date.parse(value);
  return Number.isFinite(parsed) ? parsed : null;
}

function maybeNumber(value) {
  const number = Number(value);
  return Number.isFinite(number) ? number : null;
}

function numberOrDefault(value, fallback) {
  const number = Number(value);
  return Number.isFinite(number) ? number : fallback;
}

function round(value, places) {
  const scale = 10 ** places;
  return Math.round(value * scale) / scale;
}

function sleep(milliseconds) {
  Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, milliseconds);
}

function parseArgs(argv) {
  const options = {
    json: false,
    port: DEFAULT_PORT,
    repoRoot: process.cwd(),
    sampleSeconds: 0,
    staleAgeMinutes: DEFAULT_STALE_AGE_MINUTES,
  };

  for (let index = 0; index < argv.length; ) {
    const arg = argv[index];
    if (arg === "--json") {
      options.json = true;
      index += 1;
      continue;
    }
    if (arg === "--port") {
      options.port = Number(requireValue(argv, index, arg));
      index += 2;
      continue;
    }
    if (arg === "--repo-root") {
      options.repoRoot = path.resolve(requireValue(argv, index, arg));
      index += 2;
      continue;
    }
    if (arg === "--sample-seconds") {
      options.sampleSeconds = Number(requireValue(argv, index, arg));
      index += 2;
      continue;
    }
    if (arg === "--stale-minutes") {
      options.staleAgeMinutes = Number(requireValue(argv, index, arg));
      index += 2;
      continue;
    }
    if (arg === "--help" || arg === "-h") {
      printUsage();
      process.exit(0);
    }
    throw new Error(`Unknown option: ${arg}`);
  }

  return options;
}

function requireValue(argv, index, flag) {
  const value = argv[index + 1];
  if (!value) {
    throw new Error(`${flag} requires a value`);
  }
  return value;
}

function printHuman(report) {
  process.stdout.write(`DX-WWW Agent 1 process proof: ${report.recommendedAction}\n`);
  process.stdout.write(`Active dx-www proof processes: ${report.activeDxWwwProofProcessCount}\n`);
  process.stdout.write(
    `Port ${report.port.port}: ${report.port.listener ? `owned by PID ${report.port.listener.owningProcess}` : "not listening"}\n`,
  );
  process.stdout.write(
    `Binary: ${report.binary.fresh ? "fresh" : "stale or missing"}${
      report.binary.lastWriteTime ? ` (${report.binary.lastWriteTime})` : ""
    }\n`,
  );
  if (report.nextAction.command) {
    process.stdout.write(`Next command: ${report.nextAction.command}\n`);
  }
  process.stdout.write(`Reason: ${report.nextAction.reason}\n`);
  for (const warning of report.warnings) {
    process.stdout.write(`- ${warning.id}: ${warning.message}\n`);
  }
}

function printUsage() {
  process.stdout.write(
    [
      "Usage: node tools/worktree/www-agent1-process-proof.cjs [--json] [--sample-seconds N]",
      "",
      "Collects process, port, disk, and binary freshness evidence for DX-WWW proof coordination.",
      "It only reports evidence and never starts, kills, or edits anything.",
      "",
    ].join("\n"),
  );
}

function main(argv = process.argv.slice(2)) {
  try {
    const options = parseArgs(argv);
    const snapshot = collectLiveSnapshot(options);
    const report = summarizeProcessProof(snapshot, options);
    if (options.json) {
      process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
    } else {
      printHuman(report);
    }
  } catch (error) {
    process.stderr.write(`${error.message}\n`);
    process.exitCode = 2;
  }
}

if (require.main === module) {
  main();
}

module.exports = {
  classifyProcess,
  collectLiveSnapshot,
  parseArgs,
  summarizeProcessProof,
  describeNextAction,
};
