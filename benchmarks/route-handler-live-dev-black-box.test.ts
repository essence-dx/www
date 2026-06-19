const assert = require("node:assert/strict");
const { spawn } = require("node:child_process");
const fs = require("node:fs");
const net = require("node:net");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const dxWwwBinary = path.join(
  repoRoot,
  "target",
  "debug",
  process.platform === "win32" ? "dx-www.exe" : "dx-www",
);
const routeHandlerRuntimeSourceFiles = [
  path.join(repoRoot, "core", "src", "delivery", "server_contract.rs"),
  path.join(repoRoot, "core", "src", "delivery", "route_handler_body_boundary.rs"),
  path.join(repoRoot, "core", "src", "delivery", "route_handler_headers.rs"),
  path.join(repoRoot, "core", "src", "delivery", "route_handler_request.rs"),
  path.join(repoRoot, "core", "src", "delivery", "route_handler_response.rs"),
  path.join(repoRoot, "dx-www", "src", "cli", "dev_http.rs"),
  path.join(repoRoot, "dx-www", "src", "cli", "route_handler_runtime_env.rs"),
  path.join(repoRoot, "dx-www", "src", "cli", "app_route_handler_receipt.rs"),
];

function writeFile(root, relativePath, content) {
  const filePath = path.join(root, relativePath);
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, content);
}

async function freePort() {
  const server = net.createServer();
  await new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(0, "127.0.0.1", resolve);
  });
  const { port } = server.address();
  await new Promise((resolve, reject) => server.close((error) => (error ? reject(error) : resolve())));
  return port;
}

function createRouteHandlerFixture(port) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-route-handler-live-"));
  writeFile(
    root,
    "dx",
    `project.name="route-handler-live-black-box"
dev.host="127.0.0.1"
dev.port=${port}
dev.hot_reload=false
`,
  );
  writeFile(
    root,
    "app/page.tsx",
    `export default function Page() {
  return <main>route handler live fixture</main>;
}
`,
  );
  writeFile(
    root,
    "app/api/probe/[id]/route.ts",
    `export function GET(request: Request, { params }: { params: { id: string } }) {
  const url = new URL(request.url);

  return Response.json(
    {
      method: request.method,
      id: params.id,
      ref: url.searchParams.get("ref") ?? "missing",
      probe: request.headers.get("x-dx-probe") ?? "missing",
      session: request.cookies.get("session")?.value ?? "missing",
    },
    {
      status: 203,
      headers: {
        "cache-control": "no-store",
        "x-dx-probe-response": "get-ok",
      },
    },
  );
}

export async function POST(request: Request, { params }: { params: { id: string } }) {
  const body = await request.json();
  const raw = await request.clone().text();
  const url = new URL(request.url);

  return Response.json(
    {
      method: request.method,
      id: params.id,
      ref: url.searchParams.get("ref") ?? "missing",
      echo: body.message,
      count: body.count,
      raw,
      probe: request.headers.get("x-dx-probe") ?? "missing",
      session: request.cookies.get("session")?.value ?? "missing",
    },
    {
      status: 202,
      headers: {
        "cache-control": "no-store",
        "x-dx-probe-response": "post-ok",
      },
    },
  );
}

export function HEAD(request: Request, { params }: { params: { id: string } }) {
  return Response.json(
    {
      method: request.method,
      id: params.id,
    },
    {
      status: 204,
      headers: {
        "x-dx-probe-response": "head-ok",
      },
    },
  );
}
`,
  );
  writeFile(
    root,
    "app/api/unsupported/route.ts",
    `export async function POST(request: Request) {
  const bytes = await request.arrayBuffer();

  return Response.json({
    byteLength: bytes.byteLength,
  });
}
`,
  );
  return root;
}

function waitForDevServer(child) {
  return new Promise((resolve, reject) => {
    let output = "";
    const timeout = setTimeout(() => {
      reject(new Error(`timed out waiting for dx dev startup\n${output}`));
    }, 15000);
    const onData = (chunk) => {
      output += chunk.toString();
      const match = output.match(/Development server running at (http:\/\/[^\s]+)/);
      if (!match) {
        return;
      }
      clearTimeout(timeout);
      resolve(match[1]);
    };
    child.stdout.on("data", onData);
    child.stderr.on("data", onData);
    child.once("exit", (code, signal) => {
      clearTimeout(timeout);
      reject(new Error(`dx dev exited before startup: code=${code} signal=${signal}\n${output}`));
    });
  });
}

async function stopDevServer(child) {
  if (child.exitCode !== null) {
    return;
  }
  child.kill();
  await new Promise((resolve) => child.once("exit", resolve));
}

async function readJson(response) {
  const text = await response.text();
  return JSON.parse(text);
}

async function fetchWithStartupRetry(url, init) {
  const deadline = Date.now() + 2000;
  let lastError;
  while (Date.now() < deadline) {
    try {
      return await fetch(url, init);
    } catch (error) {
      lastError = error;
      await new Promise((resolve) => setTimeout(resolve, 50));
    }
  }
  throw lastError;
}

function freshRouteHandlerBinaryAvailable(t) {
  if (!fs.existsSync(dxWwwBinary)) {
    t.skip("target/debug/dx-www executable is required for live route-handler proof");
    return false;
  }

  const binaryMtime = fs.statSync(dxWwwBinary).mtimeMs;
  const staleSourceFile = routeHandlerRuntimeSourceFiles.find(
    (sourceFile) => fs.existsSync(sourceFile) && fs.statSync(sourceFile).mtimeMs > binaryMtime,
  );
  if (staleSourceFile) {
    const relativeSource = path.relative(repoRoot, staleSourceFile).replace(/\\/g, "/");
    t.skip(`target/debug/dx-www is older than ${relativeSource}; rebuild before live route-handler proof`);
    return false;
  }
  return true;
}

test("dx dev executes route handlers over HTTP with request maps and no node_modules", async (t) => {
  if (!freshRouteHandlerBinaryAvailable(t)) {
    return;
  }

  const port = await freePort();
  const projectRoot = createRouteHandlerFixture(port);
  const child = spawn(dxWwwBinary, ["dev", "--host", "127.0.0.1", "--port", String(port), "--no-hot-reload"], {
    cwd: projectRoot,
    stdio: ["ignore", "pipe", "pipe"],
    windowsHide: true,
  });

  t.after(async () => {
    await stopDevServer(child);
    fs.rmSync(projectRoot, { recursive: true, force: true });
  });

  const baseUrl = await waitForDevServer(child);

  const getResponse = await fetchWithStartupRetry(`${baseUrl}/api/probe/launch%20team?ref=qa+pass`, {
    headers: {
      cookie: "session=abc123; theme=dark",
      "x-dx-probe": "get-header",
    },
  });
  const getBody = await readJson(getResponse);

  assert.equal(getResponse.status, 203);
  assert.equal(getResponse.headers.get("x-dx-route-handler-receipt"), "dx.next.appRouteHandlerReceipt");
  assert.equal(getResponse.headers.get("x-dx-route-handler-request-maps"), "params=1;searchParams=1");
  assert.equal(getResponse.headers.get("x-dx-node-modules-required"), "false");
  assert.equal(getResponse.headers.get("x-dx-external-runtime-executed"), "false");
  assert.equal(getResponse.headers.get("x-dx-probe-response"), "get-ok");
  assert.deepEqual(getBody, {
    method: "GET",
    id: "launch team",
    ref: "qa pass",
    probe: "get-header",
    session: "abc123",
  });

  const postPayload = { message: "ship it", count: 2 };
  const postResponse = await fetchWithStartupRetry(`${baseUrl}/api/probe/launch%20team?ref=body+pass`, {
    method: "POST",
    headers: {
      "content-type": "application/json",
      cookie: "session=post456",
      "x-dx-probe": "post-header",
    },
    body: JSON.stringify(postPayload),
  });
  const postBody = await readJson(postResponse);

  assert.equal(postResponse.status, 202);
  assert.equal(postResponse.headers.get("x-dx-route-handler-receipt"), "dx.next.appRouteHandlerReceipt");
  assert.equal(postResponse.headers.get("x-dx-route-handler-request-maps"), "params=1;searchParams=1");
  assert.equal(postResponse.headers.get("x-dx-probe-response"), "post-ok");
  assert.equal(postBody.method, "POST");
  assert.equal(postBody.id, "launch team");
  assert.equal(postBody.ref, "body pass");
  assert.equal(postBody.echo, "ship it");
  assert.equal(postBody.count, 2);
  assert.equal(postBody.probe, "post-header");
  assert.equal(postBody.session, "post456");
  assert.deepEqual(JSON.parse(postBody.raw), postPayload);

  const headResponse = await fetchWithStartupRetry(`${baseUrl}/api/probe/launch%20team?ref=head+pass`, {
    method: "HEAD",
    headers: {
      cookie: "session=head789",
      "x-dx-probe": "head-header",
    },
  });

  assert.equal(headResponse.status, 204);
  assert.equal(headResponse.headers.get("x-dx-route-handler-receipt"), "dx.next.appRouteHandlerReceipt");
  assert.equal(headResponse.headers.get("x-dx-route-handler-request-maps"), "params=1;searchParams=1");
  assert.equal(headResponse.headers.get("x-dx-probe-response"), "head-ok");
  assert.equal(await headResponse.text(), "");

  const unsupportedResponse = await fetchWithStartupRetry(`${baseUrl}/api/unsupported?ref=binary+reader`, {
    method: "POST",
    headers: {
      "content-type": "application/octet-stream",
      "x-dx-probe": "binary-reader",
    },
    body: "not-executed",
  });
  const unsupportedBody = await readJson(unsupportedResponse);

  assert.equal(unsupportedResponse.status, 501);
  assert.equal(unsupportedResponse.headers.get("x-dx-route-handler-receipt"), "dx.next.appRouteHandlerReceipt");
  assert.equal(unsupportedResponse.headers.get("x-dx-node-modules-required"), "false");
  assert.equal(unsupportedResponse.headers.get("x-dx-route-handler-source-owned"), "true");
  assert.equal(unsupportedResponse.headers.get("x-dx-external-runtime-executed"), "false");
  assert.equal(unsupportedBody.ok, false);
  assert.equal(unsupportedBody.status, "route-handler-boundary");
  assert.equal(unsupportedBody.method, "POST");
  assert.equal(unsupportedBody.path, "/api/unsupported?ref=binary+reader");
  assert.equal(unsupportedBody.lifecycleScriptsExecuted, false);
  assert.match(unsupportedBody.sourcePath, /app\/api\/unsupported\/route\.ts$/);
  assert.match(unsupportedBody.message, /unsupported route handler request body reader `request\.arrayBuffer\(\)`/);
  assert.match(unsupportedBody.message, /source-owned-safe-interpreter supports request\.json\(\), request\.text\(\), request\.formData\(\), and request\.body/);
  assert.match(unsupportedBody.message, /node_modules_required=false/);
  assert.match(unsupportedBody.message, /external_runtime_executed=false/);

  assert.deepEqual(findNodeModulesDirs(projectRoot), []);
});

function findNodeModulesDirs(root) {
  const found = [];
  const stack = [root];
  while (stack.length > 0) {
    const current = stack.pop();
    for (const entry of fs.readdirSync(current, { withFileTypes: true })) {
      const fullPath = path.join(current, entry.name);
      if (!entry.isDirectory()) {
        continue;
      }
      if (entry.name === "node_modules") {
        found.push(path.relative(root, fullPath).replace(/\\/g, "/"));
        continue;
      }
      stack.push(fullPath);
    }
  }
  return found.sort();
}
