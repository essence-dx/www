import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = join(import.meta.dirname, "..");

function read(relativePath: string): string {
  return readFileSync(join(repoRoot, relativePath), "utf8");
}

test("same-machine throughput orchestrator is TypeScript and side-effect-light by default", () => {
  const source = read("benchmarks/dx-runtime-throughput-orchestrator.ts");

  assert.match(source, /type Mode = "preflight" \| "prepare" \| "run" \| "all" \| "static-server"/);
  assert.match(source, /mode === "preflight"/);
  assert.match(source, /mode === "prepare" \|\| mode === "all"/);
  assert.match(source, /Same-machine raceboard prerequisites are missing/);
  assert.match(source, /target-probe-timeout-ms/);
  assert.match(source, /AbortSignal\.timeout\(targetProbeTimeoutMs\)/);
  assert.match(source, /runStaticServerMode/);
  assert.match(source, /--static-dir/);
  assert.match(source, /shouldUseShell/);
  assert.match(source, /taskkill/);
  assert.match(source, /createPreflight/);
  assert.match(source, /runPrepare/);
  assert.match(source, /startServers/);
  assert.match(source, /runBenchmark/);
  assert.match(source, /if \(!preflight\.ready\)/);
  assert.match(source, /const servers = await startServers\(\)/);

  assert.match(source, /http:\/\/127\.0\.0\.1:42104\/fair-counter/);
  assert.match(source, /http:\/\/127\.0\.0\.1:42101\//);
  assert.match(source, /http:\/\/127\.0\.0\.1:42102\//);
  assert.match(source, /http:\/\/127\.0\.0\.1:42103\//);

  assert.match(source, /--dx-www-bin/);
  assert.match(source, /--demo-bin/);
  assert.match(source, /--\$\{target\.name\}-url/);
  assert.match(source, /--\$\{target\.name\}-pid/);
  assert.match(source, /--\$\{target\.name\}-command/);

  assert.match(source, /npmCommand\(\["ci"\]/);
  assert.match(source, /npmCommand\(\["run", "build"\]/);
  assert.match(source, /cargo", \["build", "--release", "-j", jobs, "-p", "dx-www"/);
  assert.match(source, /cargo", \["build", "--release", "-j", jobs, "-p", "dx-www-demo"/);
});

test("same-machine throughput orchestrator preflight prints JSON without running measurement", () => {
  const output = execFileSync(
    process.execPath,
    [
      join(repoRoot, "benchmarks", "dx-runtime-throughput-orchestrator.ts"),
      "--mode",
      "preflight",
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      stdio: ["ignore", "pipe", "pipe"],
    },
  );
  const report = JSON.parse(output);

  assert.equal(report.schema, "dx.www.same_machine_performance_orchestrator_preflight");
  assert.equal(report.mode, "preflight");
  assert.equal(report.benchmark_script, "benchmarks/dx-runtime-throughput-benchmark.ts");
  assert.equal(report.dx_www_binary.path, "target/release/dx-www.exe");
  assert.equal(report.targets.length, 4);
  assert.deepEqual(
    report.targets.map((target: { name: string; url: string }) => [target.name, target.url]),
    [
      ["www", "http://127.0.0.1:42104/fair-counter"],
      ["next", "http://127.0.0.1:42101/"],
      ["svelte", "http://127.0.0.1:42102/"],
      ["astro", "http://127.0.0.1:42103/"],
    ],
  );
  assert.match(report.prepare_command, /--mode prepare --jobs 6/);
  assert.match(report.run_command, /--mode run --rounds 3 --requests 240 --concurrency 16/);
  assert.match(report.run_command, /--dx-www-bin target\/release\/dx-www\.exe/);
  assert.match(report.run_command, /--demo-bin target\/release\/demo-server\.exe/);
  assert.match(report.run_command, /--www-url http:\/\/127\.0\.0\.1:42104\/fair-counter/);
  assert.match(report.run_command, /--next-url http:\/\/127\.0\.0\.1:42101\//);
  assert.match(report.run_command, /--svelte-url http:\/\/127\.0\.0\.1:42102\//);
  assert.match(report.run_command, /--astro-url http:\/\/127\.0\.0\.1:42103\//);
  assert.match(report.all_command, /--mode all --jobs 6 --rounds 3 --requests 240 --concurrency 16/);
  assert.match(report.claim_boundary, /Preflight is not a performance receipt/);
});
