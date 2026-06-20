import fs from "node:fs";
import path from "node:path";

type ReplayOptions = {
  baseUrl: string;
  matrixPath: string;
  outPath?: string;
  hostedProvider: boolean;
  providerId: string;
};

type ReplayCase = {
  route_path: string;
  source_path: string;
  method: string;
  expected_status: number;
  expected_status_text: string;
  expected_allow_methods: string[];
  actual_status: number | null;
  actual_allow: string | null;
  passed: boolean;
  error: string | null;
};

const schema = "dx.www.readiness.route_handler_provider_replay_receipt_contract";
const matrixSchema = "dx.www.deploy.route_handler_conformance_matrix";

async function main(): Promise<void> {
  const options = parseArgs(process.argv.slice(2));
  const base = parseHttpBaseUrl(options.baseUrl);
  const localBaseUrl = isLocalOrPrivateBaseUrl(base);
  if (options.hostedProvider && localBaseUrl) {
    throw new Error(
      "--hosted-provider requires a non-local, non-private http(s) base URL so hosted provider proof cannot be forged from localhost",
    );
  }

  const matrix = JSON.parse(fs.readFileSync(options.matrixPath, "utf8"));
  const cases = collectReplayCases(matrix);
  const results: ReplayCase[] = [];
  for (const replayCase of cases) {
    results.push(await executeReplayCase(base, replayCase));
  }

  const failed = results.filter((item) => !item.passed);
  const hostedProviderProof = options.hostedProvider && !localBaseUrl && failed.length === 0 && results.length > 0;
  const receipt = {
    schema,
    schema_revision: 1,
    id: "route-handler-provider-replay",
    collector: "dx-www-route-handler-provider-replay",
    provider_id: options.providerId,
    provider_replay_executed: true,
    hosted_provider_requested: options.hostedProvider,
    hosted_provider_proof: hostedProviderProof,
    local_base_url: localBaseUrl,
    base_url: base.toString(),
    matrix_path: slashPath(options.matrixPath),
    matrix_schema: matrix.schema ?? null,
    matrix_status: matrix.matrix_status ?? null,
    manifest_hash: matrix.manifest_hash ?? null,
    passed: failed.length === 0 && results.length > 0,
    status:
      hostedProviderProof
        ? "hosted-route-handler-provider-replay-current"
        : failed.length === 0 && results.length > 0
          ? "local-route-handler-provider-replay-current-not-hosted-proof"
          : "route-handler-provider-replay-failed",
    release_ready: false,
    fastest_world_claim: false,
    case_count: results.length,
    passed_case_count: results.length - failed.length,
    failed_case_count: failed.length,
    route_count: Array.isArray(matrix.routes) ? matrix.routes.length : 0,
    replay_cases: results,
    rule:
      "Hosted provider proof is true only when --hosted-provider is used with a non-local http(s) URL and every concrete GET/HEAD/OPTIONS/405 route-handler replay case passes.",
  };

  const rendered = `${JSON.stringify(receipt, null, 2)}\n`;
  if (options.outPath) {
    fs.mkdirSync(path.dirname(options.outPath), { recursive: true });
    fs.writeFileSync(options.outPath, rendered);
  } else {
    process.stdout.write(rendered);
  }
}

function parseArgs(argv: string[]): ReplayOptions {
  let baseUrl = "";
  let matrixPath = "";
  let outPath: string | undefined;
  let hostedProvider = false;
  let providerId = "provider";

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === "--base-url") {
      baseUrl = requiredValue(argv, ++index, arg);
    } else if (arg === "--matrix") {
      matrixPath = requiredValue(argv, ++index, arg);
    } else if (arg === "--out") {
      outPath = requiredValue(argv, ++index, arg);
    } else if (arg === "--hosted-provider") {
      hostedProvider = true;
    } else if (arg === "--provider-id") {
      providerId = requiredValue(argv, ++index, arg);
    } else if (arg === "--help" || arg === "-h") {
      printUsage();
      process.exit(0);
    } else {
      throw new Error(`Unknown option: ${arg}`);
    }
  }

  if (!baseUrl || !matrixPath) {
    printUsage();
    throw new Error("--base-url and --matrix are required");
  }

  return {
    baseUrl,
    matrixPath,
    outPath,
    hostedProvider,
    providerId,
  };
}

function requiredValue(argv: string[], index: number, option: string): string {
  const value = argv[index];
  if (!value) {
    throw new Error(`${option} requires a value`);
  }
  return value;
}

function printUsage(): void {
  process.stderr.write(
    [
      "Usage: node benchmarks/dx-www-route-handler-provider-replay.ts --base-url <url> --matrix <.dx/build-cache/route-handler-conformance-matrix.json> [--out <receipt.json>] [--hosted-provider] [--provider-id <id>]",
      "",
    ].join("\n"),
  );
}

function parseHttpBaseUrl(value: string): URL {
  const url = new URL(value);
  if (url.protocol !== "http:" && url.protocol !== "https:") {
    throw new Error(`Route-handler provider replay requires http(s), got ${url.protocol}`);
  }
  if (!url.pathname.endsWith("/")) {
    url.pathname = `${url.pathname}/`;
  }
  return url;
}

function isLocalOrPrivateBaseUrl(url: URL): boolean {
  const host = url.hostname.toLowerCase().replace(/^\[(.*)\]$/, "$1");
  if (
    host === "localhost" ||
    host.endsWith(".localhost") ||
    host === "0.0.0.0" ||
    host === "::1" ||
    host === "127.0.0.1" ||
    host.startsWith("127.") ||
    host.startsWith("10.") ||
    host.startsWith("192.168.")
  ) {
    return true;
  }
  const match172 = host.match(/^172\.(\d{1,3})\./);
  if (match172) {
    const second = Number(match172[1]);
    return second >= 16 && second <= 31;
  }
  return false;
}

function collectReplayCases(matrix: any): ReplayCase[] {
  if (matrix.schema !== matrixSchema) {
    throw new Error(`Route-handler matrix must use schema ${matrixSchema}`);
  }
  const routes = Array.isArray(matrix.routes) ? matrix.routes : [];
  const cases: ReplayCase[] = [];
  for (const route of routes) {
    const routePath = typeof route.path === "string" ? route.path : "/";
    const sourcePath = typeof route.source_path === "string" ? route.source_path : "unknown";
    const allowedMethods = stringArray(route.allowed_methods);
    for (const item of Array.isArray(route.local_replay_cases) ? route.local_replay_cases : []) {
      const status = expectedStatusCode(item.expected_status);
      if (status === null || typeof item.method !== "string") {
        continue;
      }
      cases.push(emptyReplayCase(routePath, sourcePath, item.method, item.expected_status, status, allowMethodsFor(item.method, allowedMethods)));
    }
    const methodGuard = route.method_not_allowed_case;
    const methodGuardStatus = expectedStatusCode(methodGuard?.expected_status);
    if (methodGuardStatus !== null && typeof methodGuard?.method === "string") {
      cases.push(
        emptyReplayCase(
          routePath,
          sourcePath,
          methodGuard.method,
          methodGuard.expected_status,
          methodGuardStatus,
          allowMethodsFor(methodGuard.method, allowedMethods),
        ),
      );
    }
  }
  return cases;
}

function emptyReplayCase(
  routePath: string,
  sourcePath: string,
  method: string,
  expectedStatusText: unknown,
  expectedStatus: number,
  expectedAllowMethods: string[],
): ReplayCase {
  return {
    route_path: routePath,
    source_path: sourcePath,
    method: method.toUpperCase(),
    expected_status: expectedStatus,
    expected_status_text: String(expectedStatusText),
    expected_allow_methods: expectedAllowMethods,
    actual_status: null,
    actual_allow: null,
    passed: false,
    error: null,
  };
}

async function executeReplayCase(base: URL, replayCase: ReplayCase): Promise<ReplayCase> {
  const url = new URL(replayCase.route_path.replace(/^\//, ""), base);
  try {
    const response = await fetch(url, {
      method: replayCase.method,
      redirect: "manual",
    });
    const actualAllow = response.headers.get("allow");
    await response.arrayBuffer().catch(() => new ArrayBuffer(0));
    const statusPassed = response.status === replayCase.expected_status;
    const allowPassed =
      replayCase.expected_allow_methods.length === 0 ||
      replayCase.expected_allow_methods.every((method) => headerHasToken(actualAllow, method));
    return {
      ...replayCase,
      actual_status: response.status,
      actual_allow: actualAllow,
      passed: statusPassed && allowPassed,
    };
  } catch (error) {
    return {
      ...replayCase,
      error: error instanceof Error ? error.message : String(error),
    };
  }
}

function expectedStatusCode(value: unknown): number | null {
  const match = String(value ?? "").match(/^(\d{3})(?:\s|$)/);
  return match ? Number(match[1]) : null;
}

function allowMethodsFor(method: string, allowedMethods: string[]): string[] {
  const normalized = method.toUpperCase();
  if (normalized !== "OPTIONS" && normalized !== "DELETE" && normalized !== "PATCH" && normalized !== "PUT" && normalized !== "POST") {
    return [];
  }
  return Array.from(new Set([...allowedMethods, "OPTIONS"])).sort();
}

function stringArray(value: unknown): string[] {
  return Array.isArray(value)
    ? value.filter((item): item is string => typeof item === "string").map((item) => item.toUpperCase())
    : [];
}

function headerHasToken(header: string | null, token: string): boolean {
  if (!header) {
    return false;
  }
  return header
    .split(",")
    .map((item) => item.trim().toUpperCase())
    .includes(token.toUpperCase());
}

function slashPath(value: string): string {
  return value.replaceAll("\\", "/");
}

main().catch((error) => {
  process.stderr.write(`${error instanceof Error ? error.message : String(error)}\n`);
  process.exitCode = 1;
});
