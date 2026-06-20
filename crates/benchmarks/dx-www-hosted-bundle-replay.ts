import fs from "node:fs";
import path from "node:path";

type ReplayOptions = {
  baseUrl: string;
  deployAdapterPath: string;
  providerAdapterPath: string;
  outPath?: string;
  hostedProvider: boolean;
  providerId: string;
};

type UploadArtifact = {
  path: string;
  bundle: "public-runtime" | "evidence" | string;
  cache_control: string | null;
};

type ArtifactProbe = {
  path: string;
  bundle: string;
  expected_public: boolean;
  method: string;
  status: number | null;
  cache_control: string | null;
  content_type: string | null;
  public_accessible: boolean;
  passed: boolean;
  error: string | null;
};

const schema = "dx.www.readiness.bundle_provider_replay_receipt_contract";
const deployPartitionSchema = "dx.www.readiness.bundle_partition";

async function main(): Promise<void> {
  const options = parseArgs(process.argv.slice(2));
  const base = parseHttpBaseUrl(options.baseUrl);
  const localBaseUrl = isLocalOrPrivateBaseUrl(base);
  if (options.hostedProvider && localBaseUrl) {
    throw new Error(
      "--hosted-provider requires a non-local, non-private http(s) base URL so hosted bundle proof cannot be forged from localhost",
    );
  }

  const deployAdapter = readJson(options.deployAdapterPath);
  const providerAdapter = readJson(options.providerAdapterPath);
  const uploadPlan = collectUploadPlan(providerAdapter);
  const publicArtifacts = uploadPlan.filter((artifact) => artifact.bundle === "public-runtime");
  const evidenceArtifacts = uploadPlan.filter((artifact) => artifact.bundle === "evidence");
  const probeArtifacts = [...publicArtifacts, ...evidenceArtifacts];
  const probes: ArtifactProbe[] = [];
  for (const artifact of probeArtifacts) {
    probes.push(await probeArtifact(base, artifact));
  }

  const publicFailures = probes.filter((probe) => probe.expected_public && !probe.public_accessible);
  const evidenceLeaks = probes.filter((probe) => !probe.expected_public && probe.public_accessible);
  const passed =
    publicArtifacts.length > 0 &&
    evidenceArtifacts.length > 0 &&
    publicFailures.length === 0 &&
    evidenceLeaks.length === 0 &&
    deployPartitionPresent(deployAdapter, providerAdapter);
  const hostedProviderProof = passed && options.hostedProvider && !localBaseUrl;

  const receipt = {
    schema,
    schema_revision: 1,
    id: "bundle-provider-replay",
    collector: "dx-www-hosted-bundle-replay",
    provider_id: options.providerId,
    provider_replay_executed: true,
    hosted_provider_requested: options.hostedProvider,
    hosted_provider_proof: hostedProviderProof,
    local_base_url: localBaseUrl,
    base_url: base.toString(),
    deploy_adapter_path: slashPath(options.deployAdapterPath),
    provider_adapter_path: slashPath(options.providerAdapterPath),
    deploy_partition_schema: deployAdapter?.bundle_partition?.schema ?? null,
    provider_partition_schema: providerAdapter?.bundle_partition?.schema ?? null,
    public_runtime_artifact_count: publicArtifacts.length,
    evidence_artifact_count: evidenceArtifacts.length,
    checked_artifact_count: probes.length,
    public_checked_count: publicArtifacts.length,
    evidence_checked_count: evidenceArtifacts.length,
    public_accessible_count: probes.filter((probe) => probe.expected_public && probe.public_accessible).length,
    public_failure_count: publicFailures.length,
    evidence_public_leak_count: evidenceLeaks.length,
    passed,
    status: hostedProviderProof
      ? "hosted-public-evidence-bundle-replay-current"
      : passed
        ? "local-public-evidence-bundle-replay-current-not-hosted-proof"
        : "public-evidence-bundle-replay-failed",
    release_ready: false,
    fastest_world_claim: false,
    artifact_probes: probes,
    rule:
      "Hosted bundle proof is true only when --hosted-provider is used with a non-local http(s) URL, every public-runtime upload-plan artifact is publicly reachable, and every evidence artifact is not publicly reachable.",
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
  let deployAdapterPath = "";
  let providerAdapterPath = "";
  let outPath: string | undefined;
  let hostedProvider = false;
  let providerId = "provider";

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === "--base-url") {
      baseUrl = requiredValue(argv, ++index, arg);
    } else if (arg === "--deploy-adapter") {
      deployAdapterPath = requiredValue(argv, ++index, arg);
    } else if (arg === "--provider-adapter") {
      providerAdapterPath = requiredValue(argv, ++index, arg);
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

  if (!baseUrl || !deployAdapterPath || !providerAdapterPath) {
    printUsage();
    throw new Error("--base-url, --deploy-adapter, and --provider-adapter are required");
  }

  return {
    baseUrl,
    deployAdapterPath,
    providerAdapterPath,
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
      "Usage: node benchmarks/dx-www-hosted-bundle-replay.ts --base-url <url> --deploy-adapter <.dx/build-cache/deploy-adapter.json> --provider-adapter <.dx/build-cache/provider-adapter.dx-cloud.json> [--out <receipt.json>] [--hosted-provider] [--provider-id <id>]",
      "",
    ].join("\n"),
  );
}

function readJson(filePath: string): any {
  return JSON.parse(fs.readFileSync(filePath, "utf8"));
}

function collectUploadPlan(providerAdapter: any): UploadArtifact[] {
  const uploadPlan = Array.isArray(providerAdapter?.upload_plan) ? providerAdapter.upload_plan : [];
  return uploadPlan
    .map((artifact: any) => ({
      path: typeof artifact?.path === "string" ? artifact.path : "",
      bundle: typeof artifact?.bundle === "string" ? artifact.bundle : "unknown",
      cache_control: typeof artifact?.cache_control === "string" ? artifact.cache_control : null,
    }))
    .filter((artifact: UploadArtifact) => artifact.path.length > 0);
}

function deployPartitionPresent(deployAdapter: any, providerAdapter: any): boolean {
  const deployPartition = deployAdapter?.bundle_partition;
  const providerPartition = providerAdapter?.bundle_partition;
  const deploySchemaCurrent =
    deployPartition?.schema === deployPartitionSchema ||
    (deployPartition?.schema === "dx.www.deploy.bundle_partition" && deployPartition?.separation_enforced === true);
  return deploySchemaCurrent && providerPartition?.schema === deployPartitionSchema;
}

async function probeArtifact(base: URL, artifact: UploadArtifact): Promise<ArtifactProbe> {
  const expectedPublic = artifact.bundle === "public-runtime";
  const url = new URL(artifact.path.replace(/^\/+/, ""), base);
  try {
    let response = await fetch(url, { method: "HEAD", redirect: "manual" });
    let method = "HEAD";
    if (response.status === 405 || response.status === 501) {
      response = await fetch(url, {
        method: "GET",
        headers: { range: "bytes=0-0" },
        redirect: "manual",
      });
      method = "GET";
      await response.arrayBuffer().catch(() => new ArrayBuffer(0));
    }
    const publicAccessible = response.status >= 200 && response.status < 400;
    return {
      path: slashPath(artifact.path),
      bundle: artifact.bundle,
      expected_public: expectedPublic,
      method,
      status: response.status,
      cache_control: response.headers.get("cache-control"),
      content_type: response.headers.get("content-type"),
      public_accessible: publicAccessible,
      passed: expectedPublic ? publicAccessible : !publicAccessible,
      error: null,
    };
  } catch (error) {
    return {
      path: slashPath(artifact.path),
      bundle: artifact.bundle,
      expected_public: expectedPublic,
      method: "HEAD",
      status: null,
      cache_control: null,
      content_type: null,
      public_accessible: false,
      passed: !expectedPublic,
      error: error instanceof Error ? error.message : String(error),
    };
  }
}

function parseHttpBaseUrl(value: string): URL {
  const url = new URL(value);
  if (url.protocol !== "http:" && url.protocol !== "https:") {
    throw new Error(`Bundle provider replay requires http(s), got ${url.protocol}`);
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

function slashPath(value: string): string {
  return value.replaceAll("\\", "/");
}

main().catch((error) => {
  process.stderr.write(`${error instanceof Error ? error.message : String(error)}\n`);
  process.exitCode = 1;
});
