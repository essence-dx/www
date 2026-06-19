import fs from "node:fs";
import path from "node:path";
import crypto from "node:crypto";
import os from "node:os";
import { execFileSync } from "node:child_process";
import { performance } from "node:perf_hooks";
import { fileURLToPath } from "node:url";

const args = new Map();
for (let index = 2; index < process.argv.length; index += 1) {
  const arg = process.argv[index];
  if (!arg.startsWith("--")) {
    continue;
  }
  const [key, inlineValue] = arg.slice(2).split("=", 2);
  const nextValue = process.argv[index + 1];
  const hasSeparateValue = inlineValue === undefined && nextValue && !nextValue.startsWith("--");
  const value = inlineValue ?? (hasSeparateValue ? nextValue : "true");
  args.set(key, value);
  if (hasSeparateValue) {
    index += 1;
  }
}

const targetTemplates = [
  { name: "www", url: "http://127.0.0.1:42104/fair-counter" },
  { name: "next", url: "http://127.0.0.1:42101/" },
  { name: "svelte", url: "http://127.0.0.1:42102/" },
  { name: "astro", url: "http://127.0.0.1:42103/" },
];

const defaultTargets = targetTemplates.map((target) => ({
  ...target,
  url: String(args.get(`${target.name}-url`) ?? target.url),
}));

const requestCount = Number(args.get("requests") ?? 240);
const concurrency = Number(args.get("concurrency") ?? 16);
const warmup = Number(args.get("warmup") ?? 20);
const requestTimeoutMs = Number(args.get("request-timeout-ms") ?? 10_000);
const requestedRounds = Number(args.get("rounds") ?? 1);
const rounds = Number.isFinite(requestedRounds) ? Math.max(0, Math.floor(requestedRounds)) : 1;
const outputPath = args.get("out") || "target/framework-comparison-20260531/throughput.json";
const dxWwwBinaryPath = args.get("dx-www-bin") || "target/release/dx-www.exe";
const scriptFilePath = fileURLToPath(import.meta.url);
const scriptPath = path.relative(process.cwd(), scriptFilePath).replace(/\\/g, "/");
const userAgent = "dx-www-throughput-benchmark/1.0";
const dryRun = args.has("dry-run") || args.get("mode") === "dry-run";
const measurementExecuted = !dryRun && rounds > 0;
const preflightHttpRequestsExecuted = !dryRun;
const httpRequestsExecuted = preflightHttpRequestsExecuted || measurementExecuted;

function percentile(values, percentileRank) {
  if (values.length === 0) {
    return null;
  }
  const index = Math.min(
    values.length - 1,
    Math.max(0, Math.ceil((percentileRank / 100) * values.length) - 1),
  );
  return values[index];
}

function rounded(value) {
  return Math.round(value * 100) / 100;
}

function sortedNumbers(values) {
  return values
    .filter((value) => Number.isFinite(value))
    .slice()
    .sort((left, right) => left - right);
}

function sha256Buffer(buffer) {
  return crypto.createHash("sha256").update(buffer).digest("hex");
}

function sha256File(filePath) {
  const resolved = path.resolve(filePath);
  if (!fs.existsSync(resolved)) {
    return null;
  }
  return {
    path: filePath,
    exists: true,
    sha256: sha256Buffer(fs.readFileSync(resolved)),
    bytes: fs.statSync(resolved).size,
    hash_status: "captured",
  };
}

function validateDxWwwBinary(binary) {
  if (dryRun) {
    return;
  }
  if (binary.exists !== true || !binary.sha256 || binary.hash_status !== "captured") {
    throw new Error(
      `same-machine throughput benchmark requires a hashed dx-www binary; run a release build or pass --dx-www-bin. Missing: ${binary.path}`,
    );
  }
}

function readJsonIfPresent(filePath) {
  const resolved = path.resolve(filePath);
  if (!fs.existsSync(resolved)) {
    return null;
  }
  return JSON.parse(fs.readFileSync(resolved, "utf8"));
}

function packageVersionSummary(targetName) {
  const packageJsonPaths = [
    `target/framework-comparison-20260531/${targetName}/package.json`,
    `benchmarks/fair-counter/${targetName}/package.json`,
  ];
  for (const packageJsonPath of packageJsonPaths) {
    const packageJson = readJsonIfPresent(packageJsonPath);
    if (!packageJson) {
      continue;
    }
    return {
      package_json: packageJsonPath,
      package_name: packageJson.name ?? null,
      package_version: packageJson.version ?? null,
      dependencies: packageJson.dependencies ?? {},
      dev_dependencies: packageJson.devDependencies ?? {},
    };
  }
  return {
    package_json: null,
    package_name: targetName,
    package_version: null,
    dependencies: {},
    dev_dependencies: {},
  };
}

function gitOutput(args) {
  try {
    return execFileSync("git", args, {
      cwd: process.cwd(),
      encoding: "utf8",
      stdio: ["ignore", "pipe", "ignore"],
    }).trim();
  } catch (_error) {
    return null;
  }
}

function gitSha() {
  return gitOutput(["rev-parse", "HEAD"]);
}

function gitBranch() {
  return gitOutput(["branch", "--show-current"]);
}

function gitStatusShort() {
  return gitOutput(["status", "--short"]) ?? "";
}

function machineMetadata() {
  const cpus = os.cpus();
  return {
    platform: os.platform(),
    os_platform: os.platform(),
    arch: os.arch(),
    os_arch: os.arch(),
    release: os.release(),
    os_release: os.release(),
    hostname: os.hostname(),
    timezone: Intl.DateTimeFormat().resolvedOptions().timeZone ?? null,
    totalmem_bytes: os.totalmem(),
    total_memory_bytes: os.totalmem(),
    freemem_bytes: os.freemem(),
    cpu_count: cpus.length,
    cpu_logical_cores: cpus.length,
    cpu_model: cpus[0]?.model ?? null,
    cpu_speed_mhz: cpus[0]?.speed ?? null,
  };
}

function targetPort(target) {
  try {
    return Number(new URL(target.url).port);
  } catch (_error) {
    return null;
  }
}

function serverProvenanceFor(target) {
  const pidText = args.get(`${target.name}-pid`);
  const pid = pidText ? Number(pidText) : null;
  return {
    name: target.name,
    url: target.url,
    host: new URL(target.url).hostname,
    port: targetPort(target),
    command: args.get(`${target.name}-command`) ?? null,
    cwd: args.get(`${target.name}-cwd`) ?? null,
    pid: Number.isFinite(pid) ? pid : null,
    command_provided_by_user: args.has(`${target.name}-command`),
    pid_provided_by_user: args.has(`${target.name}-pid`),
    command_source: args.has(`${target.name}-command`) ? "operator_supplied" : "not_captured",
    pid_source: args.has(`${target.name}-pid`) ? "operator_supplied" : "not_captured",
    port_source: "url",
    captured_at: new Date().toISOString(),
  };
}

async function timedRequest(url) {
  const started = performance.now();
  const signal = AbortSignal.timeout(requestTimeoutMs);
  const response = await fetch(url, {
    cache: "no-store",
    signal,
    headers: {
      "cache-control": "no-cache",
      "user-agent": userAgent,
    },
  });
  const body = await response.arrayBuffer();
  const ended = performance.now();
  return {
    status: response.status,
    ok: response.ok,
    bytes: body.byteLength,
    body,
    ms: ended - started,
  };
}

async function hashOutputFixture(target) {
  try {
    const result = await timedRequest(target.url);
    return {
      name: target.name,
      target_name: target.name,
      url: target.url,
      status: result.status,
      ok: result.ok,
      bytes: result.bytes,
      sha256: sha256Buffer(Buffer.from(result.body)),
      hash_algorithm: "sha256",
      captured_from: "preflight",
      captured_at: new Date().toISOString(),
    };
  } catch (error) {
    return {
      name: target.name,
      target_name: target.name,
      url: target.url,
      status: null,
      ok: false,
      bytes: null,
      sha256: null,
      hash_algorithm: "sha256",
      captured_from: "preflight",
      captured_at: new Date().toISOString(),
      error: error instanceof Error ? error.message : String(error),
    };
  }
}

function validatePreflightOutputFixtures(outputFixtures) {
  if (dryRun) {
    return;
  }
  const failures = outputFixtures.filter((fixture) => {
    const status = Number(fixture.status);
    return (
      fixture.ok !== true ||
      !Number.isFinite(status) ||
      status < 200 ||
      status >= 400 ||
      !fixture.sha256 ||
      typeof fixture.sha256 !== "string"
    );
  });
  if (failures.length > 0) {
    const details = failures
      .map((fixture) => `${fixture.name} ${fixture.url} status=${fixture.status} ok=${fixture.ok} error=${fixture.error ?? "none"}`)
      .join("; ");
    throw new Error(`same-machine throughput preflight failed: ${details}`);
  }
}

function dryRunOutputFixture(target) {
  return {
    name: target.name,
    target_name: target.name,
    url: target.url,
    status: null,
    ok: null,
    bytes: null,
    sha256: null,
    hash_algorithm: "sha256",
    captured_from: "dry-run",
    captured_at: new Date().toISOString(),
  };
}

async function runBatch(target) {
  for (let index = 0; index < warmup; index += 1) {
    await timedRequest(target.url);
  }

  const latencies = [];
  let bytes = 0;
  let errors = 0;
  let issued = 0;
  const started = performance.now();

  async function worker() {
    while (issued < requestCount) {
      issued += 1;
      try {
        const result = await timedRequest(target.url);
        latencies.push(result.ms);
        bytes += result.bytes;
        if (!result.ok) {
          errors += 1;
        }
      } catch (_error) {
        errors += 1;
      }
    }
  }

  await Promise.all(Array.from({ length: concurrency }, worker));
  const ended = performance.now();
  latencies.sort((left, right) => left - right);
  const durationSeconds = (ended - started) / 1000;

  return {
    name: target.name,
    url: target.url,
    requests: requestCount,
    concurrency,
    warmup,
    request_timeout_ms: requestTimeoutMs,
    errors,
    bytes,
    duration_ms: rounded(ended - started),
    requests_per_second: rounded(requestCount / durationSeconds),
    latency_ms: {
      min: rounded(latencies[0] ?? 0),
      p50: rounded(percentile(latencies, 50) ?? 0),
      p95: rounded(percentile(latencies, 95) ?? 0),
      p99: rounded(percentile(latencies, 99) ?? 0),
      max: rounded(latencies[latencies.length - 1] ?? 0),
    },
  };
}

async function runRound(target, roundIndex) {
  const result = await runBatch(target);
  return {
    ...result,
    round_index: roundIndex,
  };
}

function aggregateMetric(values) {
  const sorted = sortedNumbers(values);
  if (sorted.length === 0) {
    return {
      min: null,
      median: null,
      p95: null,
      p99: null,
      max: null,
    };
  }
  return {
    min: rounded(sorted[0]),
    median: rounded(percentile(sorted, 50)),
    p95: rounded(percentile(sorted, 95)),
    p99: rounded(percentile(sorted, 99)),
    max: rounded(sorted[sorted.length - 1]),
  };
}

function aggregateTarget(target, roundResults, outputFixtures) {
  const targetResults = roundResults.filter((result) => result.name === target.name);
  const preflightOutputSha256Values = Array.from(
    new Set(targetResults.map((result) => result.output_sha256).filter(Boolean)),
  ).sort();
  const outputFixture = outputFixtures.find((fixture) => fixture.name === target.name);
  return {
    name: target.name,
    target_name: target.name,
    url: target.url,
    rounds: targetResults.length,
    round_count: targetResults.length,
    successful_round_count: targetResults.filter((result) => result.errors === 0).length,
    requests_per_round: requestCount,
    total_requests: targetResults.length * requestCount,
    concurrency,
    warmup,
    request_timeout_ms: requestTimeoutMs,
    errors: targetResults.reduce((sum, result) => sum + result.errors, 0),
    errors_total: targetResults.reduce((sum, result) => sum + result.errors, 0),
    bytes: targetResults.reduce((sum, result) => sum + result.bytes, 0),
    output_hash_stable: null,
    output_hash_stability_measured: false,
    output_hash_stability_scope: "not_measured_in_rounds",
    preflight_output_hash_stable:
      preflightOutputSha256Values.length > 0 ? preflightOutputSha256Values.length <= 1 : null,
    preflight_output_sha256: outputFixture?.sha256 ?? null,
    preflight_output_sha256_values: preflightOutputSha256Values,
    output_sha256_values: preflightOutputSha256Values,
    aggregate: {
      requests_per_second: aggregateMetric(targetResults.map((result) => result.requests_per_second)),
      duration_ms: aggregateMetric(targetResults.map((result) => result.duration_ms)),
      latency_ms: {
        p50: aggregateMetric(targetResults.map((result) => result.latency_ms.p50)),
        p95: aggregateMetric(targetResults.map((result) => result.latency_ms.p95)),
        p99: aggregateMetric(targetResults.map((result) => result.latency_ms.p99)),
      },
    },
    requests_per_second: aggregateMetric(targetResults.map((result) => result.requests_per_second)),
    duration_ms: aggregateMetric(targetResults.map((result) => result.duration_ms)),
    latency_ms: {
      p50: aggregateMetric(targetResults.map((result) => result.latency_ms.p50)),
      p95: aggregateMetric(targetResults.map((result) => result.latency_ms.p95)),
      p99: aggregateMetric(targetResults.map((result) => result.latency_ms.p99)),
    },
  };
}

function validateMeasuredTargetSummaries(targetSummaries) {
  if (!measurementExecuted) {
    return;
  }
  const failures = targetSummaries.filter((summary) => {
    const rounds = Number(summary.round_count);
    const successfulRounds = Number(summary.successful_round_count);
    const errors = Number(summary.errors_total ?? summary.errors);
    const medianRps = Number(summary.requests_per_second?.median);
    return (
      !Number.isFinite(rounds) ||
      rounds <= 0 ||
      !Number.isFinite(successfulRounds) ||
      successfulRounds !== rounds ||
      !Number.isFinite(errors) ||
      errors !== 0 ||
      !Number.isFinite(medianRps) ||
      medianRps <= 0
    );
  });
  if (failures.length > 0) {
    const details = failures
      .map(
        (summary) =>
          `${summary.name} rounds=${summary.round_count} successful=${summary.successful_round_count} errors=${summary.errors_total ?? summary.errors} medianRps=${summary.requests_per_second?.median}`,
      )
      .join("; ");
    throw new Error(`same-machine throughput measurement failed: ${details}`);
  }
}

(async () => {
  const dxWwwBinary = sha256File(dxWwwBinaryPath) ?? {
    path: dxWwwBinaryPath,
    exists: false,
    sha256: null,
    bytes: null,
    hash_status: "missing",
  };
  validateDxWwwBinary(dxWwwBinary);

  const outputFixtures = [];
  for (const target of defaultTargets) {
    outputFixtures.push(dryRun ? dryRunOutputFixture(target) : await hashOutputFixture(target));
  }
  validatePreflightOutputFixtures(outputFixtures);

  const roundRecords = [];
  if (measurementExecuted) {
    for (let roundIndex = 1; roundIndex <= rounds; roundIndex += 1) {
      const targets = [];
      const startedAt = new Date().toISOString();
      for (const target of defaultTargets) {
        const fixture = outputFixtures.find((outputFixture) => outputFixture.name === target.name);
        targets.push({
          ...(await runRound(target, roundIndex)),
          output_sha256: fixture?.sha256 ?? null,
          output_sha256_source: "preflight",
          measurement_body_hash_captured: false,
        });
      }
      roundRecords.push({
        round_index: roundIndex,
        started_at: startedAt,
        ended_at: new Date().toISOString(),
        targets,
      });
    }
  }
  const roundResults = roundRecords.flatMap((roundRecord) => roundRecord.targets);
  const statusShort = gitStatusShort();
  const aggregateTargets = defaultTargets.map((target) =>
    aggregateTarget(target, roundResults, outputFixtures),
  );
  validateMeasuredTargetSummaries(aggregateTargets);

  const report = {
    schema: "dx.www.same_machine_performance_receipt",
    legacy_schema: "dx.www.runtime_throughput_benchmark",
    previous_schema: "dx.www.runtime_throughput_benchmark",
    generated_at: new Date().toISOString(),
    receipt_id: `dx-www-same-machine-${Date.now()}`,
    benchmark: {
      script_path: scriptPath,
      script_sha256: sha256File(scriptPath)?.sha256 ?? null,
      args: Object.fromEntries(args),
      request_count: requestCount,
      concurrency,
      warmup,
      request_timeout_ms: requestTimeoutMs,
      round_count: rounds,
      requested_round_count: requestedRounds,
      dry_run: dryRun,
      measurement_executed: measurementExecuted,
      http_requests_executed: httpRequestsExecuted,
      preflight_http_requests_executed: preflightHttpRequestsExecuted,
      measurement_http_requests_executed: measurementExecuted,
      target_order: defaultTargets.map((target) => target.name),
      user_agent: userAgent,
    },
    runtime: {
      node_version: process.version,
      node_exec_path: process.execPath,
      node_platform: process.platform,
      node_arch: process.arch,
    },
    node: process.version,
    node_exec_path: process.execPath,
    request_count: requestCount,
    concurrency,
    warmup,
    request_timeout_ms: requestTimeoutMs,
    round_count: rounds,
    requested_round_count: requestedRounds,
    dry_run: dryRun,
    measurement_executed: measurementExecuted,
    http_requests_executed: httpRequestsExecuted,
    preflight_http_requests_executed: preflightHttpRequestsExecuted,
    measurement_http_requests_executed: measurementExecuted,
    measurement_skipped_reason: dryRun
      ? "dry-run requested; no HTTP requests were issued"
      : rounds === 0
        ? "round_count was 0; no measurement rounds were executed"
        : null,
    machine: machineMetadata(),
    git: {
      repo_path: process.cwd(),
      head_sha: gitSha(),
      sha: gitSha(),
      branch: gitBranch(),
      status_short: statusShort,
      status_porcelain: statusShort,
      status_captured_at: new Date().toISOString(),
      is_dirty: statusShort.length > 0,
      dirty: statusShort.length > 0,
    },
    dx_www_binary: dxWwwBinary,
    binary: dxWwwBinary,
    framework_package_versions: Object.fromEntries(
      defaultTargets.map((target) => [target.name, packageVersionSummary(target.name)]),
    ),
    frameworks: Object.fromEntries(
      defaultTargets.map((target) => [target.name, packageVersionSummary(target.name)]),
    ),
    server_provenance: defaultTargets.map(serverProvenanceFor),
    servers: defaultTargets.map(serverProvenanceFor),
    output_fixtures: outputFixtures,
    rounds: roundRecords,
    round_results: roundResults,
    targets: roundResults,
    legacy_targets: roundResults,
    target_summaries: aggregateTargets,
    aggregate: {
      round_count: rounds,
      measured_round_count: roundRecords.length,
      request_count: requestCount,
      concurrency,
      warmup,
      targets: aggregateTargets,
    },
    aggregates: aggregateTargets,
    upstream_baseline_measured: false,
    faster_than_upstream_claimed: false,
    same_machine_replay_required_for_speed_claim: true,
    claims: {
      upstream_baseline_measured: false,
      faster_than_upstream_claimed: false,
      same_machine_replay_required_for_speed_claim: true,
    },
    no_claims: {
      no_claim_cross_machine_comparison: true,
      no_claim_production_latency: true,
      no_claim_browser_render_performance: true,
      no_claim_cold_start_performance: true,
      no_claim_memory_or_cpu_efficiency: true,
      no_claim_build_time_or_bundle_size: true,
      no_claim_statistical_significance_beyond_recorded_rounds: true,
      no_claim_framework_absolute_superiority: true,
    },
    claim_boundary:
      "This receipt records one local configured benchmark run. It is not a faster-than-upstream claim without a replayed upstream/fork baseline on the same machine.",
  };

  fs.mkdirSync(path.dirname(outputPath), { recursive: true });
  fs.writeFileSync(outputPath, `${JSON.stringify(report, null, 2)}\n`);
  console.log(JSON.stringify(report, null, 2));
})().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
