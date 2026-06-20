import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import fs from "node:fs";
import http from "node:http";
import os from "node:os";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");
const script = path.join(root, "benchmarks/dx-www-route-handler-provider-replay.ts");

test("readiness exposes guarded route-handler provider receipt import", () => {
  const readiness = fs.readFileSync(path.join(root, "dx-www/src/cli/readiness.rs"), "utf8");
  assert.match(readiness, /READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT_CONTRACT/);
  assert.match(readiness, /READINESS_ROUTE_HANDLER_PROVIDER_COLLECT_COMMAND/);
  assert.match(readiness, /--import-route-handler-provider-receipt/);
  assert.match(readiness, /import_readiness_route_handler_provider_receipt/);
  assert.match(readiness, /readiness_route_handler_provider_base_url_is_hosted/);
  assert.match(readiness, /route-handler-provider-replay-not-hosted-proof/);
  assert.match(readiness, /hosted-route-handler-provider-replay-current/);
  assert.match(readiness, /route_handler_provider_replay_current/);
});

test("route-handler provider replay writes local functional receipt without hosted proof", async () => {
  await withServer(true, async (baseUrl) => {
    const temp = fs.mkdtempSync(path.join(os.tmpdir(), "dx-route-provider-"));
    const matrixPath = writeMatrix(temp);
    const outPath = path.join(temp, "receipt.json");
    const result = await runScript(
      [script, "--base-url", baseUrl, "--matrix", matrixPath, "--out", outPath, "--provider-id", "local-preview"],
    );

    assert.equal(result.status, 0, result.stderr);
    const receipt = JSON.parse(fs.readFileSync(outPath, "utf8"));
    assert.equal(receipt.schema, "dx.www.readiness.route_handler_provider_replay_receipt_contract");
    assert.equal(receipt.provider_replay_executed, true);
    assert.equal(receipt.hosted_provider_proof, false);
    assert.equal(receipt.local_base_url, true);
    assert.equal(receipt.passed, true);
    assert.equal(receipt.status, "local-route-handler-provider-replay-current-not-hosted-proof");
    assert.equal(receipt.case_count, 4);
    assert.equal(receipt.failed_case_count, 0);
  });
});

test("route-handler provider replay rejects localhost hosted-provider overclaims", async () => {
  await withServer(true, async (baseUrl) => {
    const temp = fs.mkdtempSync(path.join(os.tmpdir(), "dx-route-provider-"));
    const result = await runScript([script, "--base-url", baseUrl, "--matrix", writeMatrix(temp), "--hosted-provider"]);

    assert.notEqual(result.status, 0);
    assert.match(result.stderr, /requires a non-local, non-private http\(s\) base URL/);
  });
});

test("route-handler provider replay fails when provider status mismatches matrix", async () => {
  await withServer(false, async (baseUrl) => {
    const temp = fs.mkdtempSync(path.join(os.tmpdir(), "dx-route-provider-"));
    const outPath = path.join(temp, "receipt.json");
    const result = await runScript([script, "--base-url", baseUrl, "--matrix", writeMatrix(temp), "--out", outPath]);

    assert.equal(result.status, 0, result.stderr);
    const receipt = JSON.parse(fs.readFileSync(outPath, "utf8"));
    assert.equal(receipt.passed, false);
    assert.equal(receipt.hosted_provider_proof, false);
    assert.equal(receipt.failed_case_count, 1);
    assert.equal(receipt.status, "route-handler-provider-replay-failed");
    assert.equal(
      receipt.replay_cases.some((item: any) => item.method === "OPTIONS" && item.actual_status === 200),
      true,
    );
  });
});

function writeMatrix(dir: string): string {
  const matrixPath = path.join(dir, ".dx/build-cache/route-handler-conformance-matrix.json");
  fs.writeFileSync(
    matrixPath,
    `${JSON.stringify(
      {
        schema: "dx.www.deploy.route_handler_conformance_matrix",
        schema_revision: 1,
        manifest_hash: "fixture",
        matrix_status: "local-route-handler-conformance-foundation",
        routes: [
          {
            path: "/api/health",
            source_path: "app/api/health/route.ts",
            allowed_methods: ["GET", "HEAD", "OPTIONS"],
            local_replay_cases: [
              { method: "GET", expected_status: "200 OK" },
              { method: "HEAD", expected_status: "200 OK" },
              { method: "OPTIONS", expected_status: "204 No Content" },
            ],
            method_not_allowed_case: {
              method: "DELETE",
              expected_status: "405 Method Not Allowed",
            },
          },
        ],
      },
      null,
      2,
    )}\n`,
  );
  return matrixPath;
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
      reject(new Error(`route-handler provider replay script timed out: ${args.join(" ")}`));
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

async function withServer(optionsPass: boolean, callback: (baseUrl: string) => Promise<void>): Promise<void> {
  const server = http.createServer((request, response) => {
    if (request.url !== "/api/health") {
      response.writeHead(404);
      response.end("not found");
      return;
    }

    const allow = "GET, HEAD, OPTIONS";
    if (request.method === "GET" || request.method === "HEAD") {
      response.writeHead(200, { allow });
      response.end(request.method === "HEAD" ? undefined : "ok");
    } else if (request.method === "OPTIONS") {
      response.writeHead(optionsPass ? 204 : 200, { allow });
      response.end();
    } else {
      response.writeHead(405, { allow });
      response.end("method not allowed");
    }
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
