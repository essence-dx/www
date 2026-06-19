import { spawn, spawnSync, type ChildProcess } from "node:child_process";
import crypto from "node:crypto";
import fs from "node:fs";
import http, { type Server } from "node:http";
import path from "node:path";
import { fileURLToPath } from "node:url";

type Mode = "preflight" | "prepare" | "run" | "all" | "static-server";

type CommandSpec = {
  command: string;
  args: string[];
  cwd: string;
  env?: NodeJS.ProcessEnv;
};

type TargetSpec = {
  name: "www" | "next" | "svelte" | "astro";
  url: string;
  port: number;
  cwd: string;
  install?: CommandSpec;
  build?: CommandSpec;
  outputChecks: string[];
};

type ManagedServer = {
  name: TargetSpec["name"];
  command: string;
  pid: number | null;
  close: () => Promise<void>;
};

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const suiteRoot = path.join(root, "benchmarks", "fair-counter");
const benchmarkScript = path.join(root, "benchmarks", "dx-runtime-throughput-benchmark.ts");
const orchestratorScript = fileURLToPath(import.meta.url);

const args = parseArgs(process.argv.slice(2));
const mode = stringArg("mode", "preflight") as Mode;
const jobs = stringArg("jobs", "6");
const rounds = stringArg("rounds", "3");
const requests = stringArg("requests", "240");
const concurrency = stringArg("concurrency", "16");
const targetProbeTimeoutMs = Number(stringArg("target-probe-timeout-ms", "2000"));
const staticDir = stringArg("static-dir", "");
const staticPort = Number(stringArg("port", "0"));
const out = stringArg("out", "target/framework-comparison-20260531/throughput.json");
const dxWwwBinary = stringArg("dx-www-bin", "target/release/dx-www.exe");
const demoBinary = stringArg("demo-bin", "target/release/demo-server.exe");
const jsonOutput = args.has("json") || mode === "preflight";

const targets: TargetSpec[] = [
  {
    name: "www",
    port: 42104,
    url: stringArg("www-url", "http://127.0.0.1:42104/fair-counter"),
    cwd: root,
    build: command("cargo", ["build", "--release", "-j", jobs, "-p", "dx-www-demo", "--bin", "demo-server"], root),
    outputChecks: [demoBinary],
  },
  {
    name: "next",
    port: 42101,
    url: stringArg("next-url", "http://127.0.0.1:42101/"),
    cwd: path.join(suiteRoot, "next"),
    install: npmCommand(["ci"], path.join(suiteRoot, "next")),
    build: npmCommand(["run", "build"], path.join(suiteRoot, "next")),
    outputChecks: ["benchmarks/fair-counter/next/node_modules", "benchmarks/fair-counter/next/.next"],
  },
  {
    name: "svelte",
    port: 42102,
    url: stringArg("svelte-url", "http://127.0.0.1:42102/"),
    cwd: path.join(suiteRoot, "svelte"),
    install: npmCommand(["ci"], path.join(suiteRoot, "svelte")),
    build: npmCommand(["run", "build"], path.join(suiteRoot, "svelte")),
    outputChecks: [
      "benchmarks/fair-counter/svelte/node_modules",
      "benchmarks/fair-counter/svelte/dist/index.html",
    ],
  },
  {
    name: "astro",
    port: 42103,
    url: stringArg("astro-url", "http://127.0.0.1:42103/"),
    cwd: path.join(suiteRoot, "astro"),
    install: npmCommand(["ci"], path.join(suiteRoot, "astro")),
    build: npmCommand(["run", "build"], path.join(suiteRoot, "astro")),
    outputChecks: [
      "benchmarks/fair-counter/astro/node_modules",
      "benchmarks/fair-counter/astro/dist/index.html",
    ],
  },
];

async function main(): Promise<void> {
  if (!["preflight", "prepare", "run", "all", "static-server"].includes(mode)) {
    throw new Error(`Unsupported mode "${mode}". Use preflight, prepare, run, all, or static-server.`);
  }

  if (mode === "static-server") {
    await runStaticServerMode();
    return;
  }

  if (mode === "prepare" || mode === "all") {
    runPrepare();
  }

  const preflight = createPreflight();
  if (mode === "preflight" || mode === "prepare") {
    printJson(preflight);
    return;
  }

  if (!preflight.ready) {
    printJson(preflight);
    throw new Error("Same-machine raceboard prerequisites are missing; run --mode prepare or inspect the preflight report.");
  }

  const servers = await startServers();
  try {
    progress("waiting for benchmark targets");
    await waitForTargets(targets);
    progress("running benchmark receipt writer");
    runBenchmark(servers);
  } finally {
    progress("stopping benchmark targets");
    await Promise.allSettled(servers.map((server) => server.close()));
  }
}

function parseArgs(input: string[]): Map<string, string> {
  const parsed = new Map<string, string>();
  for (let index = 0; index < input.length; index += 1) {
    const arg = input[index];
    if (!arg.startsWith("--")) {
      continue;
    }
    const [key, inlineValue] = arg.slice(2).split("=", 2);
    const nextValue = input[index + 1];
    const hasSeparateValue = inlineValue === undefined && nextValue && !nextValue.startsWith("--");
    parsed.set(key, inlineValue ?? (hasSeparateValue ? nextValue : "true"));
    if (hasSeparateValue) {
      index += 1;
    }
  }
  return parsed;
}

function stringArg(name: string, fallback: string): string {
  return args.get(name) ?? fallback;
}

function command(commandName: string, commandArgs: string[], cwd: string, env?: NodeJS.ProcessEnv): CommandSpec {
  return { command: commandName, args: commandArgs, cwd, env };
}

function npmCommand(commandArgs: string[], cwd: string): CommandSpec {
  return command("npm", commandArgs, cwd, {
    ...process.env,
    npm_config_fund: "false",
    npm_config_audit: "false",
    ASTRO_TELEMETRY_DISABLED: "1",
    NEXT_TELEMETRY_DISABLED: "1",
  });
}

function createPreflight() {
  const missing = [
    fileStatus(dxWwwBinary),
    ...targets.flatMap((target) => target.outputChecks.map(fileStatus)),
  ].filter((entry) => !entry.exists);

  const report = {
    schema: "dx.www.same_machine_performance_orchestrator_preflight",
    schema_revision: 1,
    mode,
    ready: missing.length === 0,
    generated_at: new Date().toISOString(),
    root,
    benchmark_script: relativePath(benchmarkScript),
    dx_www_binary: fileStatus(dxWwwBinary),
    targets: targets.map((target) => ({
      name: target.name,
      url: target.url,
      port: target.port,
      cwd: relativePath(target.cwd),
      output_checks: target.outputChecks.map(fileStatus),
      install_command: target.install ? commandToString(target.install) : null,
      build_command: target.build ? commandToString(target.build) : null,
    })),
    missing,
    prepare_command: orchestratorCommand("prepare", { json: true }),
    run_command: orchestratorCommand("run"),
    all_command: orchestratorCommand("all"),
    claim_boundary:
      "Preflight is not a performance receipt. Only dx-runtime-throughput-benchmark.ts can write target/framework-comparison-20260531/throughput.json after real HTTP measurement.",
  };
  return report;
}

function fileStatus(relativeOrAbsolutePath: string) {
  const absolute = path.resolve(root, relativeOrAbsolutePath);
  const exists = fs.existsSync(absolute);
  const stat = exists ? fs.statSync(absolute) : null;
  return {
    path: relativePath(absolute),
    exists,
    kind: stat?.isDirectory() ? "directory" : stat?.isFile() ? "file" : null,
    bytes: stat?.isFile() ? stat.size : null,
    sha256: stat?.isFile() ? sha256File(absolute) : null,
  };
}

function sha256File(filePath: string): string {
  return crypto.createHash("sha256").update(fs.readFileSync(filePath)).digest("hex");
}

function runPrepare(): void {
  runCommand(command("cargo", ["build", "--release", "-j", jobs, "-p", "dx-www", "--bin", "dx-www"], root));
  for (const target of targets) {
    if (target.install) {
      runCommand(target.install);
    }
    if (target.build) {
      runCommand(target.build);
    }
  }
}

function runCommand(spec: CommandSpec): void {
  const result = spawnSync(spec.command, spec.args, {
    cwd: spec.cwd,
    env: spec.env ?? process.env,
    stdio: "inherit",
    shell: shouldUseShell(spec.command),
  });
  if (result.status !== 0) {
    throw new Error(`Command failed (${result.status}): ${commandToString(spec)} in ${relativePath(spec.cwd)}`);
  }
}

async function startServers(): Promise<ManagedServer[]> {
  progress("starting www benchmark target");
  const www = startChildServer("www", command(path.resolve(root, demoBinary), [], root, { ...process.env, PORT: "42104" }));
  progress("starting next benchmark target");
  const next = startChildServer(
    "next",
    command("npm", ["run", "start", "--", "-H", "127.0.0.1", "-p", "42101"], path.join(suiteRoot, "next"), {
      ...process.env,
      NEXT_TELEMETRY_DISABLED: "1",
      PORT: "42101",
    }),
  );
  progress("starting svelte static benchmark target");
  const svelte = startStaticServer("svelte", path.join(suiteRoot, "svelte", "dist"), 42102);
  progress("starting astro static benchmark target");
  const astro = startStaticServer("astro", path.join(suiteRoot, "astro", "dist"), 42103);
  return [www, next, svelte, astro];
}

function startChildServer(name: TargetSpec["name"], spec: CommandSpec): ManagedServer {
  const child = spawn(spec.command, spec.args, {
    cwd: spec.cwd,
    env: spec.env ?? process.env,
    stdio: ["ignore", "pipe", "pipe"],
    shell: shouldUseShell(spec.command),
  });
  child.stdout?.on("data", (chunk) => process.stderr.write(`[${name}] ${chunk}`));
  child.stderr?.on("data", (chunk) => process.stderr.write(`[${name}] ${chunk}`));
  return {
    name,
    command: commandToString(spec),
    pid: child.pid ?? null,
    close: () => stopChild(child),
  };
}

function startStaticServer(name: TargetSpec["name"], directory: string, port: number): ManagedServer {
  return startChildServer(
    name,
    command(
      process.execPath,
      [
        orchestratorScript,
        "--mode",
        "static-server",
        "--static-dir",
        directory,
        "--port",
        String(port),
      ],
      root,
    ),
  );
}

async function runStaticServerMode(): Promise<void> {
  if (!staticDir || !Number.isFinite(staticPort) || staticPort <= 0) {
    throw new Error("--mode static-server requires --static-dir and --port");
  }
  const server = createStaticFileServer(path.resolve(root, staticDir));
  await listen(server, staticPort);
  progress(`static server listening ${staticDir} on ${staticPort}`);
  await new Promise(() => {});
}

function createStaticFileServer(directory: string): Server {
  return http.createServer((request, response) => {
    const url = new URL(request.url ?? "/", "http://127.0.0.1");
    const cleanPath = url.pathname === "/" ? "/index.html" : url.pathname;
    const absolute = path.resolve(directory, `.${decodeURIComponent(cleanPath)}`);
    const base = path.resolve(directory);
    if (!absolute.startsWith(base) || !fs.existsSync(absolute) || !fs.statSync(absolute).isFile()) {
      response.writeHead(404, { "content-type": "text/plain; charset=utf-8" });
      response.end("Not found");
      return;
    }
    const body = fs.readFileSync(absolute);
    response.writeHead(200, {
      "content-type": contentType(absolute),
      "content-length": String(body.length),
    });
    response.end(body);
  });
}

function listen(server: Server, port: number): Promise<void> {
  return new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(port, "127.0.0.1", () => resolve());
  });
}

function contentType(filePath: string): string {
  switch (path.extname(filePath).toLowerCase()) {
    case ".html":
      return "text/html; charset=utf-8";
    case ".css":
      return "text/css; charset=utf-8";
    case ".js":
    case ".mjs":
      return "application/javascript; charset=utf-8";
    case ".json":
      return "application/json; charset=utf-8";
    case ".svg":
      return "image/svg+xml";
    default:
      return "application/octet-stream";
  }
}

async function waitForTargets(targetSpecs: TargetSpec[]): Promise<void> {
  await Promise.all(targetSpecs.map((target) => waitForUrl(target.url)));
}

async function waitForUrl(url: string): Promise<void> {
  const deadline = Date.now() + 45_000;
  let lastError: unknown = null;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(url, {
        headers: { "cache-control": "no-cache" },
        signal: AbortSignal.timeout(targetProbeTimeoutMs),
      });
      if (response.ok) {
        await response.arrayBuffer();
        progress(`target ready ${url}`);
        return;
      }
      lastError = new Error(`${url} returned ${response.status}`);
    } catch (error) {
      lastError = error;
    }
    await new Promise((resolve) => setTimeout(resolve, 500));
  }
  throw new Error(`Timed out waiting for ${url}: ${lastError}`);
}

function runBenchmark(servers: ManagedServer[]): void {
  const serverByName = new Map(servers.map((server) => [server.name, server]));
  const benchmarkArgs = [
    benchmarkScript,
    "--rounds",
    rounds,
    "--requests",
    requests,
    "--concurrency",
    concurrency,
    "--out",
    out,
    "--dx-www-bin",
    dxWwwBinary,
  ];
  for (const target of targets) {
    const server = serverByName.get(target.name);
    benchmarkArgs.push(`--${target.name}-url`, target.url);
    benchmarkArgs.push(`--${target.name}-command`, server?.command ?? "not-captured");
    if (server?.pid) {
      benchmarkArgs.push(`--${target.name}-pid`, String(server.pid));
    }
  }
  runCommand(command(process.execPath, benchmarkArgs, root));
}

function orchestratorCommand(commandMode: Mode, options: { json?: boolean } = {}): string {
  const commandArgs = ["benchmarks/dx-runtime-throughput-orchestrator.ts", "--mode", commandMode];
  if (commandMode === "prepare" || commandMode === "all") {
    commandArgs.push("--jobs", jobs);
  }
  if (commandMode === "run" || commandMode === "all") {
    commandArgs.push("--rounds", rounds);
    commandArgs.push("--requests", requests);
    commandArgs.push("--concurrency", concurrency);
    commandArgs.push("--out", out);
    commandArgs.push("--dx-www-bin", dxWwwBinary);
    commandArgs.push("--demo-bin", demoBinary);
    commandArgs.push(...targetUrlArgs());
  }
  if (options.json) {
    commandArgs.push("--json");
  }
  return shellCommandString("node", commandArgs);
}

function targetUrlArgs(): string[] {
  const commandArgs: string[] = [];
  for (const target of targets) {
    commandArgs.push(`--${target.name}-url`, target.url);
  }
  return commandArgs;
}

function stopChild(child: ChildProcess): Promise<void> {
  return new Promise((resolve) => {
    if (!child.pid || child.killed) {
      resolve();
      return;
    }
    if (process.platform === "win32") {
      spawnSync("taskkill", ["/PID", String(child.pid), "/T", "/F"], { stdio: "ignore" });
      resolve();
      return;
    }
    child.once("exit", () => resolve());
    child.kill("SIGTERM");
    setTimeout(() => {
      if (!child.killed) {
        child.kill("SIGKILL");
      }
      resolve();
    }, 2_000).unref();
  });
}

function commandToString(spec: CommandSpec): string {
  return shellCommandString(spec.command, spec.args);
}

function shouldUseShell(commandName: string): boolean {
  return process.platform === "win32" && !path.isAbsolute(commandName);
}

function shellCommandString(commandName: string, commandArgs: string[]): string {
  return [commandName, ...commandArgs].map(shellQuote).join(" ");
}

function shellQuote(value: string): string {
  return /^[A-Za-z0-9_./:\\-]+$/.test(value) ? value : JSON.stringify(value);
}

function relativePath(filePath: string): string {
  return path.relative(root, path.resolve(root, filePath)).replace(/\\/g, "/") || ".";
}

function printJson(value: unknown): void {
  if (jsonOutput) {
    console.log(JSON.stringify(value, null, 2));
  }
}

function progress(message: string): void {
  if (mode !== "preflight") {
    process.stderr.write(`[dx-throughput] ${message}\n`);
  }
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
