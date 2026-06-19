import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import fs from "node:fs";
import http from "node:http";
import os from "node:os";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");
const script = path.join(root, "benchmarks/dx-www-hosted-bundle-replay.ts");

test("readiness exposes guarded hosted bundle replay import", () => {
  const readiness = fs.readFileSync(path.join(root, "dx-www/src/cli/readiness.rs"), "utf8");
  assert.match(readiness, /READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_CONTRACT/);
  assert.match(readiness, /READINESS_BUNDLE_PROVIDER_REPLAY_COLLECT_COMMAND/);
  assert.match(readiness, /--import-bundle-provider-replay-receipt/);
  assert.match(readiness, /import_readiness_bundle_provider_replay_receipt/);
  assert.match(readiness, /hosted-public-evidence-bundle-replay-current/);
  assert.match(readiness, /bundle_provider_replay_current/);
});

test("hosted bundle replay writes local functional receipt without hosted proof", async () => {
  await withServer(false, async (baseUrl) => {
    const temp = fs.mkdtempSync(path.join(os.tmpdir(), "dx-bundle-provider-"));
    const { deployAdapter, providerAdapter } = writeAdapters(temp);
    const outPath = path.join(temp, "receipt.json");
    const result = await runScript([
      script,
      "--base-url",
      baseUrl,
      "--deploy-adapter",
      deployAdapter,
      "--provider-adapter",
      providerAdapter,
      "--out",
      outPath,
      "--provider-id",
      "local-preview",
    ]);

    assert.equal(result.status, 0, result.stderr);
    const receipt = JSON.parse(fs.readFileSync(outPath, "utf8"));
    assert.equal(receipt.schema, "dx.www.readiness.bundle_provider_replay_receipt_contract");
    assert.equal(receipt.provider_replay_executed, true);
    assert.equal(receipt.hosted_provider_proof, false);
    assert.equal(receipt.local_base_url, true);
    assert.equal(receipt.passed, true);
    assert.equal(receipt.status, "local-public-evidence-bundle-replay-current-not-hosted-proof");
    assert.equal(receipt.public_runtime_artifact_count, 1);
    assert.equal(receipt.evidence_artifact_count, 2);
    assert.equal(receipt.public_failure_count, 0);
    assert.equal(receipt.evidence_public_leak_count, 0);
  });
});

test("hosted bundle replay rejects localhost hosted-provider overclaims", async () => {
  await withServer(false, async (baseUrl) => {
    const temp = fs.mkdtempSync(path.join(os.tmpdir(), "dx-bundle-provider-"));
    const { deployAdapter, providerAdapter } = writeAdapters(temp);
    const result = await runScript([
      script,
      "--base-url",
      baseUrl,
      "--deploy-adapter",
      deployAdapter,
      "--provider-adapter",
      providerAdapter,
      "--hosted-provider",
    ]);

    assert.notEqual(result.status, 0);
    assert.match(result.stderr, /requires a non-local, non-private http\(s\) base URL/);
  });
});

test("hosted bundle replay fails when evidence artifacts are public", async () => {
  await withServer(true, async (baseUrl) => {
    const temp = fs.mkdtempSync(path.join(os.tmpdir(), "dx-bundle-provider-"));
    const { deployAdapter, providerAdapter } = writeAdapters(temp);
    const outPath = path.join(temp, "receipt.json");
    const result = await runScript([
      script,
      "--base-url",
      baseUrl,
      "--deploy-adapter",
      deployAdapter,
      "--provider-adapter",
      providerAdapter,
      "--out",
      outPath,
    ]);

    assert.equal(result.status, 0, result.stderr);
    const receipt = JSON.parse(fs.readFileSync(outPath, "utf8"));
    assert.equal(receipt.passed, false);
    assert.equal(receipt.hosted_provider_proof, false);
    assert.equal(receipt.status, "public-evidence-bundle-replay-failed");
    assert.equal(receipt.evidence_public_leak_count, 1);
    assert.equal(
      receipt.artifact_probes.some((probe: any) => probe.path === ".dx/receipts/readiness/proof-graph.sr"),
      true,
    );
  });
});

function writeAdapters(dir: string): { deployAdapter: string; providerAdapter: string } {
  const deployAdapter = path.join(dir, "deploy-adapter.json");
  const providerAdapter = path.join(dir, "provider-adapter.dx-cloud.json");
  fs.writeFileSync(
    deployAdapter,
    `${JSON.stringify(
      {
        schema: "dx.www.deploy.adapter",
        bundle_partition: {
          schema: "dx.www.readiness.bundle_partition",
          public_runtime_bundle: { deployable: true },
          evidence_bundle: { deployable_public_bytes: false },
        },
      },
      null,
      2,
    )}\n`,
  );
  fs.writeFileSync(
    providerAdapter,
    `${JSON.stringify(
      {
        schema: "dx.www.deploy.provider_adapter",
        bundle_partition: {
          schema: "dx.www.readiness.bundle_partition",
          separation_enforced: true,
        },
        upload_plan: [
          { path: "app/index.html", bundle: "public-runtime", cache_control: "public, max-age=31536000, immutable" },
          { path: ".dx/receipts/readiness/proof-graph.sr", bundle: "evidence", cache_control: "no-store" },
          { path: "deploy-adapter.json.br", bundle: "evidence", cache_control: "no-store" },
        ],
      },
      null,
      2,
    )}\n`,
  );
  return { deployAdapter, providerAdapter };
}

type RunResult = {
  status: number | null;
  stdout: string;
  stderr: string;
};

function runScript(args: string[]): Promise<RunResult> {
  return new Promise((resolve, reject) => {
    const child = spawn(process.execPath, args, {
      cwd: root,
      stdio: ["ignore", "pipe", "pipe"],
    });
    let stdout = "";
    let stderr = "";
    const timer = setTimeout(() => {
      child.kill();
      reject(new Error(`hosted bundle replay script timed out: ${args.join(" ")}`));
    }, 30_000);
    child.stdout.setEncoding("utf8");
    child.stderr.setEncoding("utf8");
    child.stdout.on("data", (chunk) => {
      stdout += chunk;
    });
    child.stderr.on("data", (chunk) => {
      stderr += chunk;
    });
    child.on("error", (error) => {
      clearTimeout(timer);
      reject(error);
    });
    child.on("close", (status) => {
      clearTimeout(timer);
      resolve({ status, stdout, stderr });
    });
  });
}

async function withServer(leakEvidence: boolean, callback: (baseUrl: string) => Promise<void>): Promise<void> {
  const server = http.createServer((request, response) => {
    if (request.url === "/app/index.html") {
      response.writeHead(200, {
        "cache-control": "public, max-age=31536000, immutable",
        "content-type": "text/html",
      });
      response.end(request.method === "HEAD" ? undefined : "<main>ok</main>");
      return;
    }

    if (request.url === "/.dx/receipts/readiness/proof-graph.sr" && leakEvidence) {
      response.writeHead(200, {
        "cache-control": "public, max-age=31536000, immutable",
        "content-type": "text/plain",
      });
      response.end(request.method === "HEAD" ? undefined : "leaked");
      return;
    }

    response.writeHead(404, { "cache-control": "no-store" });
    response.end("not found");
  });

  await new Promise<void>((resolve) => server.listen(0, "127.0.0.1", resolve));
  try {
    const address = server.address();
    assert.equal(typeof address, "object");
    assert.ok(address);
    await callback(`http://127.0.0.1:${address.port}`);
  } finally {
    server.closeAllConnections();
    await new Promise<void>((resolve, reject) => {
      server.close((error) => (error ? reject(error) : resolve()));
    });
  }
}
