import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import test from "node:test";

const repoRoot = join(import.meta.dirname, "..");

function read(path: string): string {
  return readFileSync(join(repoRoot, path), "utf8");
}

test("dx runtime throughput benchmark writes replayable same-machine receipts", () => {
  const source = read("benchmarks/dx-runtime-throughput-benchmark.ts");

  assert.match(source, /dx\.www\.same_machine_performance_receipt/);
  assert.match(source, /receipt_id:/);
  assert.match(source, /benchmark:/);
  assert.match(source, /script_sha256/);
  assert.match(source, /script_path/);
  assert.match(source, /args\.get\("rounds"\)/);
  assert.match(source, /round_count:\s*rounds/);
  assert.match(source, /args\.get\("request-timeout-ms"\)/);
  assert.match(source, /AbortSignal\.timeout\(requestTimeoutMs\)/);
  assert.match(source, /request_timeout_ms:\s*requestTimeoutMs/);
  assert.match(source, /runRound/);
  assert.match(source, /http:\/\/127\.0\.0\.1:42104\/fair-counter/);
  assert.match(source, /args\.get\(`\$\{target\.name\}-url`\)/);

  assert.match(source, /machine:/);
  assert.match(source, /os\.platform\(\)/);
  assert.match(source, /os\.arch\(\)/);
  assert.match(source, /os\.release\(\)/);
  assert.match(source, /os\.cpus\(\)/);
  assert.match(source, /os\.totalmem\(\)/);

  assert.match(source, /git:/);
  assert.match(source, /gitSha/);
  assert.match(source, /gitStatusShort/);
  assert.match(source, /gitBranch/);
  assert.match(source, /dirty/);

  assert.match(source, /runtime:/);
  assert.match(source, /node_version/);
  assert.match(source, /node_exec_path/);

  assert.match(source, /dx_www_binary:/);
  assert.match(source, /sha256File/);
  assert.match(source, /args\.get\("dx-www-bin"\)/);
  assert.match(source, /validateDxWwwBinary/);
  assert.match(source, /same-machine throughput benchmark requires a hashed dx-www binary/);

  assert.match(source, /server_provenance:/);
  assert.match(source, /command:/);
  assert.match(source, /pid:/);
  assert.match(source, /port:/);
  assert.match(source, /command_source:/);

  assert.match(source, /output_fixtures:/);
  assert.match(source, /hashOutputFixture/);
  assert.match(source, /sha256/);
  assert.match(source, /validatePreflightOutputFixtures/);
  assert.match(source, /same-machine throughput preflight failed/);
  assert.match(source, /error=.*none/);

  assert.match(source, /aggregate:/);
  assert.match(source, /target_summaries:/);
  assert.match(source, /legacy_targets:/);
  assert.match(source, /median/);
  assert.match(source, /p95/);
  assert.match(source, /p99/);
  assert.match(source, /validateMeasuredTargetSummaries/);
  assert.match(source, /same-machine throughput measurement failed/);
  assert.match(source, /output_hash_stability_measured:\s*false/);
  assert.match(source, /output_hash_stability_scope:\s*"not_measured_in_rounds"/);

  assert.match(source, /upstream_baseline_measured:\s*false/);
  assert.match(source, /faster_than_upstream_claimed:\s*false/);
  assert.match(source, /same_machine_replay_required_for_speed_claim:\s*true/);
  assert.match(source, /no_claims:/);
  assert.match(source, /no_claim_framework_absolute_superiority:\s*true/);
  assert.match(source, /no_claim_browser_render_performance:\s*true/);
  assert.doesNotMatch(source, /require\(["'](?:node:)?(?:net|tls|undici)["']\)/);
  assert.doesNotMatch(source, /from ["'](?:node:)?(?:net|tls|undici)["']/);
});

test("dx runtime throughput benchmark dry-run writes receipt without HTTP measurement", () => {
  const outputDir = mkdtempSync(join(tmpdir(), "dx-www-throughput-dry-run-"));
  const outputPath = join(outputDir, "receipt.json");
  const networkAttemptPath = join(outputDir, "network-attempt.txt");
  const noHttpPreloadPath = join(outputDir, "no-http-preload.ts");

  writeFileSync(
    noHttpPreloadPath,
    `
import fs from "node:fs";
function failNetwork(kind) {
  fs.writeFileSync(process.env.DX_NO_HTTP_SENTINEL, kind);
  throw new Error("dry-run attempted HTTP: " + kind);
}
globalThis.fetch = () => failNetwork("fetch");
`,
  );

  try {
    execFileSync(
      process.execPath,
      [
        "--import",
        pathToFileURL(noHttpPreloadPath).href,
        join(repoRoot, "benchmarks", "dx-runtime-throughput-benchmark.ts"),
        "--dry-run",
        "--rounds",
        "2",
        "--out",
        outputPath,
      ],
      {
        cwd: repoRoot,
        encoding: "utf8",
        stdio: ["ignore", "pipe", "pipe"],
        env: {
          ...process.env,
          DX_NO_HTTP_SENTINEL: networkAttemptPath,
        },
      },
    );

    assert.equal(existsSync(networkAttemptPath), false);
    assert.equal(existsSync(outputPath), true);
    const receipt = JSON.parse(readFileSync(outputPath, "utf8"));
    assert.equal(receipt.schema, "dx.www.same_machine_performance_receipt");
    assert.equal(receipt.dry_run, true);
    assert.equal(receipt.measurement_executed, false);
    assert.equal(receipt.http_requests_executed, false);
    assert.equal(receipt.preflight_http_requests_executed, false);
    assert.equal(receipt.measurement_http_requests_executed, false);
    assert.equal(receipt.benchmark.dry_run, true);
    assert.equal(receipt.benchmark.http_requests_executed, false);
    assert.equal(receipt.benchmark.measurement_executed, false);
    assert.equal(receipt.benchmark.preflight_http_requests_executed, false);
    assert.equal(receipt.benchmark.measurement_http_requests_executed, false);
    assert.equal(receipt.benchmark.round_count, 2);
    assert.equal(receipt.benchmark.request_timeout_ms, 10000);
    assert.equal(receipt.request_timeout_ms, 10000);
    assert.match(receipt.measurement_skipped_reason, /no HTTP requests were issued/);
    assert.equal(receipt.round_count, 2);
    assert.deepEqual(receipt.rounds, []);
    assert.deepEqual(receipt.round_results, []);
    assert.deepEqual(receipt.targets, []);
    assert.deepEqual(receipt.legacy_targets, []);
    assert.equal(receipt.target_summaries.length, 4);
    assert.equal(receipt.target_summaries[0].url, "http://127.0.0.1:42104/fair-counter");
    for (const targetSummary of receipt.target_summaries) {
      assert.equal(targetSummary.round_count, 0);
      assert.equal(targetSummary.output_hash_stable, null);
      assert.equal(targetSummary.output_hash_stability_measured, false);
      assert.equal(targetSummary.requests_per_second.median, null);
      assert.equal(targetSummary.duration_ms.median, null);
      assert.equal(targetSummary.latency_ms.p50.median, null);
      assert.equal(targetSummary.latency_ms.p95.median, null);
      assert.equal(targetSummary.latency_ms.p99.median, null);
      assert.equal(targetSummary.preflight_output_sha256, null);
      assert.deepEqual(targetSummary.preflight_output_sha256_values, []);
    }
    assert.equal(receipt.aggregate.round_count, 2);
    assert.equal(receipt.aggregate.measured_round_count, 0);
    assert.deepEqual(receipt.aggregate.targets, receipt.target_summaries);
    assert.deepEqual(receipt.aggregates, receipt.target_summaries);
    assert.equal(receipt.output_fixtures.length, 4);
    for (const outputFixture of receipt.output_fixtures) {
      assert.equal(outputFixture.captured_from, "dry-run");
      assert.equal(outputFixture.status, null);
      assert.equal(outputFixture.ok, null);
      assert.equal(outputFixture.bytes, null);
      assert.equal(outputFixture.sha256, null);
    }
    assert.equal(receipt.no_claims.no_claim_framework_absolute_superiority, true);
    assert.equal(receipt.upstream_baseline_measured, false);
    assert.equal(receipt.faster_than_upstream_claimed, false);
  } finally {
    rmSync(outputDir, { recursive: true, force: true });
  }
});

test("dx runtime throughput benchmark requires a hashed dx-www binary before HTTP measurement", () => {
  const outputDir = mkdtempSync(join(tmpdir(), "dx-www-throughput-missing-binary-"));
  const outputPath = join(outputDir, "receipt.json");
  const networkAttemptPath = join(outputDir, "network-attempt.txt");
  const noHttpPreloadPath = join(outputDir, "no-http-preload.ts");
  const missingBinaryPath = join(outputDir, "missing-dx-www.exe");

  writeFileSync(
    noHttpPreloadPath,
    `
import fs from "node:fs";
function failNetwork(kind) {
  fs.writeFileSync(process.env.DX_NO_HTTP_SENTINEL, kind);
  throw new Error("missing-binary test attempted HTTP: " + kind);
}
globalThis.fetch = () => failNetwork("fetch");
`,
  );

  try {
    assert.throws(
      () =>
        execFileSync(
          process.execPath,
          [
            "--import",
            pathToFileURL(noHttpPreloadPath).href,
            join(repoRoot, "benchmarks", "dx-runtime-throughput-benchmark.ts"),
            "--rounds",
            "1",
            "--requests",
            "1",
            "--concurrency",
            "1",
            "--dx-www-bin",
            missingBinaryPath,
            "--out",
            outputPath,
          ],
          {
            cwd: repoRoot,
            encoding: "utf8",
            stdio: ["ignore", "pipe", "pipe"],
            env: {
              ...process.env,
              DX_NO_HTTP_SENTINEL: networkAttemptPath,
            },
          },
        ),
      /same-machine throughput benchmark requires a hashed dx-www binary/,
    );

    assert.equal(existsSync(networkAttemptPath), false);
    assert.equal(existsSync(outputPath), false);
  } finally {
    rmSync(outputDir, { recursive: true, force: true });
  }
});

test("readiness exposes actionable stale reasons for same-machine performance receipts", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");

  assert.match(readiness, /same_machine_performance_stale_reason/);
  assert.match(readiness, /same-machine-performance-receipt-missing/);
  assert.match(readiness, /same-machine-performance-measurement-not-executed/);
  assert.match(readiness, /same-machine-performance-target-coverage-incomplete/);
  assert.match(readiness, /same-machine-performance-binary-hash-missing/);
  assert.match(readiness, /same-machine-performance-preflight-failed/);
  assert.match(readiness, /same-machine-performance-target-errors/);
  assert.match(readiness, /same-machine-performance-paint-and-hosted-proof-missing/);
  assert.match(readiness, /same_machine_performance_missing_targets/);
  assert.match(readiness, /readiness_same_machine_performance_raceboard/);
  assert.match(readiness, /same_machine_performance_raceboard/);
  assert.match(readiness, /www_vs_next_median_rps_ratio/);
  assert.match(readiness, /smallest_public_bytes_target/);
  assert.match(readiness, /first_response_bytes/);
  assert.match(readiness, /same_machine_first_response_bytes/);
  assert.match(readiness, /same_machine_performance_stale_reason_from_receipt/);
  assert.match(readiness, /same_machine_performance_stale_reason": same_machine_performance_stale_reason/);
  assert.match(readiness, /"stale_reason": same_machine_performance_stale_reason/);
  assert.match(readiness, /dx-runtime-throughput-orchestrator\.ts --mode all --jobs 6/);
  assert.match(readiness, /READINESS_SAME_MACHINE_PERFORMANCE_RAW_REPLAY_COMMAND/);
  assert.match(readiness, /same-machine throughput raceboard proof is missing/i);
  assert.match(readiness, /Lighthouse paint receipts, JS-disabled browser proof, Astro tiny-static payload parity/);
});
