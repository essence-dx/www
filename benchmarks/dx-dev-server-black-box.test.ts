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
const defaultDevBaseUrl = "http://127.0.0.1:3000";
const hotReloadPath = "/_dx/hot-reload/version?resource=route%3A%2F";

function endpoint(baseUrl, requestPath) {
  return new URL(requestPath, baseUrl.endsWith("/") ? baseUrl : `${baseUrl}/`).toString();
}

function writeFixtureFile(projectRoot, relativePath, content) {
  const filePath = path.join(projectRoot, relativePath);
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, content);
}

function createTinyDevFixture() {
  const projectRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-dev-black-box-"));
  writeFixtureFile(
    projectRoot,
    "dx",
    `project.name="dx-dev-black-box"
build.output_dir=".dx/build"
tooling.dx_style.generated_css="styles/generated.css"
`,
  );
  writeFixtureFile(
    projectRoot,
    "app/layout.tsx",
    `export default function RootLayout({ children }) {
  return <html><body>{children}</body></html>;
}
`,
  );
  writeFixtureFile(
    projectRoot,
    "app/page.tsx",
    `export default function Page() {
  return <main className="p-4" data-dx-dev-black-box-root="true">DX dev black-box proof</main>;
}
`,
  );
  writeFixtureFile(
    projectRoot,
    "public/favicon.svg",
    `<svg viewBox="0 0 16 16" role="img" aria-label="DX"><rect width="16" height="16"/></svg>
`,
  );
  writeFixtureFile(
    projectRoot,
    "public/robots.txt",
    `User-agent: *
Disallow: /node_modules
`,
  );
  writeFixtureFile(
    projectRoot,
    "styles/generated.css",
    `/* stale fixture CSS; dx dev should regenerate this from app/page.tsx */
`,
  );
  return projectRoot;
}

async function fetchText(url, timeoutMs, init = {}) {
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), timeoutMs);
  try {
    const response = await fetch(url, { ...init, signal: controller.signal });
    const body = await response.text();
    return { response, body };
  } finally {
    clearTimeout(timer);
  }
}

async function fetchJson(url, timeoutMs) {
  const { response, body } = await fetchText(url, timeoutMs);
  let payload;
  try {
    payload = JSON.parse(body);
  } catch (error) {
    assert.fail(`expected ${url} to return JSON, got: ${body.slice(0, 300)}`);
  }
  return { response, payload };
}

function assertHotReloadPollPayload(response, payload) {
  assert.match(response.headers.get("content-type") ?? "", /application\/json/i);
  assert.equal(payload.ok, true);
  assert.equal(payload.protocol, "dx.hot-reload.poll");
  assert.equal(payload.protocol_format, 1);
  assert.equal(payload.transport, "poll");
  assert.equal(payload.resource?.kind, "route");
  assert.equal(payload.resource?.id, "route:/");
  assert.equal(payload.instruction?.type, "restart");
  assert.equal(payload.instruction?.mode, "full-page");
  assert.equal(payload.capabilities?.route_scoped_resources, true);
  assert.equal(payload.capabilities?.partial_module_updates, false);
  assert.equal(payload.boundaries?.runtime, "dx-owned");
  assert.equal(payload.boundaries?.server, "axum");
  assert.equal(payload.boundaries?.node_runtime, "not-required");
  assert.equal(payload.receipt?.schema, "dx.dev.hot_reload.poll_receipt");
  assert.equal(payload.receipt?.source, "dx-www-rust-dev-server");
  assert.equal(payload.receipt?.hot_reload_enabled, true);
  assert.equal(payload.receipt?.transport, "poll");
  assert.equal(payload.receipt?.boundaries?.partial_module_updates, false);
  assert.equal(JSON.stringify(payload).includes("node_modules"), false);
  assert.equal(JSON.stringify(payload).toLowerCase().includes("turbopack"), false);
}

async function isDxDevServer(baseUrl) {
  try {
    const { response, payload } = await fetchJson(endpoint(baseUrl, hotReloadPath), 2500);
    return (
      response.status === 200 &&
      payload.ok === true &&
      payload.protocol === "dx.hot-reload.poll" &&
      payload.receipt?.source === "dx-www-rust-dev-server"
    );
  } catch (_error) {
    return false;
  }
}

async function findFreePort() {
  return await new Promise((resolve, reject) => {
    const server = net.createServer();
    server.unref();
    server.on("error", reject);
    server.listen(0, "127.0.0.1", () => {
      const address = server.address();
      server.close(() => {
        if (!address || typeof address === "string") {
          reject(new Error("failed to allocate a localhost TCP port"));
          return;
        }
        resolve(address.port);
      });
    });
  });
}

async function reserveNonDxDevPort() {
  return await new Promise((resolve, reject) => {
    let probeHits = 0;
    const sockets = new Set();
    const server = net.createServer((socket) => {
      probeHits += 1;
      sockets.add(socket);
      socket.on("close", () => {
        sockets.delete(socket);
      });
      socket.end(
        "HTTP/1.1 503 Service Unavailable\r\nContent-Type: text/plain\r\nContent-Length: 6\r\nConnection: close\r\n\r\nbusy\r\n",
      );
    });
    server.on("error", reject);
    server.listen(0, "127.0.0.1", () => {
      const address = server.address();
      if (!address || typeof address === "string") {
        server.close(() => reject(new Error("failed to reserve a localhost TCP port")));
        return;
      }
      resolve({
        port: address.port,
        probeHits: () => probeHits,
        close: () => {
          for (const socket of sockets) {
            socket.destroy();
          }
          return new Promise((closeResolve, closeReject) => {
            server.close((error) => (error ? closeReject(error) : closeResolve()));
          });
        },
      });
    });
  });
}

function trimProcessLog(log) {
  return log.length > 5000 ? `${log.slice(-5000)}\n[trimmed]` : log;
}

function assertBusyPortDiagnostic(output, busyPort) {
  const binaryMtime = fs.existsSync(dxWwwBinary)
    ? fs.statSync(dxWwwBinary).mtime.toISOString()
    : "missing";
  const staleBinaryHint =
    `expected the runnable dx-www binary to use the source-owned busy-port diagnostic from dx-www/src/cli/dev_options.rs; rebuild target/debug/dx-www.exe if this still reports a generic bind error (binary=${dxWwwBinary}, mtime=${binaryMtime})`;
  assert.match(output, new RegExp(`127\\.0\\.0\\.1:${busyPort}`), staleBinaryHint);
  assert.match(output, /did not answer as a DX dev server/, `${staleBinaryHint}\n${output}`);
  assert.match(output, /No duplicate dev server was started/, `${staleBinaryHint}\n${output}`);
  assert.match(output, /choose a different --port/, `${staleBinaryHint}\n${output}`);
}

async function waitForDxDevReady(baseUrl, child, getLog) {
  const startedAt = Date.now();
  const deadline = startedAt + 45000;
  let lastError = "server did not respond";

  while (Date.now() < deadline) {
    if (child.exitCode !== null || child.signalCode !== null) {
      throw new Error(
        `dx dev exited before readiness (code=${child.exitCode}, signal=${child.signalCode})\n${getLog()}`,
      );
    }

    try {
      if (await isDxDevServer(baseUrl)) {
        return;
      }
    } catch (error) {
      lastError = error.message;
    }

    await new Promise((resolve) => setTimeout(resolve, 300));
  }

  throw new Error(`dx dev did not become ready at ${baseUrl}: ${lastError}\n${getLog()}`);
}

async function stopChild(child) {
  if (child.exitCode !== null || child.signalCode !== null) {
    return;
  }

  await new Promise((resolve) => {
    let resolved = false;
    const done = () => {
      if (resolved) {
        return;
      }
      resolved = true;
      clearTimeout(forceKillTimer);
      clearTimeout(giveUpTimer);
      resolve();
    };
    const forceKillTimer = setTimeout(() => {
      child.kill("SIGKILL");
    }, 3000);
    const giveUpTimer = setTimeout(done, 6000);
    child.once("exit", done);
    child.kill();
  });
}

async function startDxDevServer() {
  if (!fs.existsSync(dxWwwBinary)) {
    return null;
  }

  const projectRoot = createTinyDevFixture();
  const port = await findFreePort();
  const baseUrl = `http://127.0.0.1:${port}`;
  let stdout = "";
  let stderr = "";
  const child = spawn(dxWwwBinary, ["dev", "--host", "127.0.0.1", "--port", String(port)], {
    cwd: projectRoot,
    stdio: ["ignore", "pipe", "pipe"],
    windowsHide: true,
  });
  child.stdout.on("data", (chunk) => {
    stdout += chunk.toString();
  });
  child.stderr.on("data", (chunk) => {
    stderr += chunk.toString();
  });
  const getLog = () => trimProcessLog(`stdout:\n${stdout}\nstderr:\n${stderr}`);

  try {
    await waitForDxDevReady(baseUrl, child, getLog);
    return {
      baseUrl,
      projectRoot,
      stop: async () => {
        await stopChild(child);
        fs.rmSync(projectRoot, { recursive: true, force: true });
      },
    };
  } catch (error) {
    await stopChild(child);
    fs.rmSync(projectRoot, { recursive: true, force: true });
    throw error;
  }
}

async function runDxDevToExit(args, cwd, timeoutMs = 12000) {
  let stdout = "";
  let stderr = "";
  const child = spawn(dxWwwBinary, args, {
    cwd,
    stdio: ["ignore", "pipe", "pipe"],
    windowsHide: true,
  });
  child.stdout.on("data", (chunk) => {
    stdout += chunk.toString();
  });
  child.stderr.on("data", (chunk) => {
    stderr += chunk.toString();
  });

  return await new Promise((resolve, reject) => {
    const timer = setTimeout(() => {
      child.kill("SIGKILL");
      reject(
        new Error(
          `dx-www ${args.join(" ")} did not exit within ${timeoutMs}ms\n${trimProcessLog(
            `stdout:\n${stdout}\nstderr:\n${stderr}`,
          )}`,
        ),
      );
    }, timeoutMs);
    child.once("error", (error) => {
      clearTimeout(timer);
      reject(error);
    });
    child.once("exit", (code, signal) => {
      clearTimeout(timer);
      resolve({ code, signal, stdout, stderr });
    });
  });
}

async function assertBlackBoxDevServer(baseUrl, expectedRootMarker) {
  const root = await fetchText(endpoint(baseUrl, "/"), 30000);
  assert.equal(root.response.status, 200, root.body.slice(0, 500));
  assert.match(root.response.headers.get("content-type") ?? "", /text\/html/i);
  assert.match(root.body, /<html[\s>]/i);
  assert.ok(
    /data-dx-route="\//.test(root.body) || /data-dx-route-source="app\/page\.tsx"/.test(root.body),
    "root response should expose a source-owned route marker",
  );
  assert.match(root.body, /data-dx-hot-reload\b/);
  assert.match(root.body, /\/_dx\/hot-reload\/version/);
  assert.match(root.body, /\/_dx\/hot-reload\/events/);
  assert.match(root.body, /data-dx-hot-reload-target/);
  assert.match(root.body, /route:\$\{location\.pathname \|\| "\/"\}/);
  assert.doesNotMatch(root.body, /<title>\s*404|Page not found|404 Not Found/i);
  if (expectedRootMarker) {
    assert.match(root.body, new RegExp(expectedRootMarker));
  }

  const hotReload = await fetchJson(endpoint(baseUrl, hotReloadPath), 10000);
  assert.equal(hotReload.response.status, 200);
  assertHotReloadPollPayload(hotReload.response, hotReload.payload);

  const favicon = await fetchText(endpoint(baseUrl, "/favicon.svg"), 10000);
  assert.equal(favicon.response.status, 200, favicon.body.slice(0, 300));
  assert.match(favicon.response.headers.get("content-type") ?? "", /image\/svg\+xml/i);
  assert.match(favicon.body, /<svg[\s>]/i);
  assert.match(favicon.body, /aria-label="DX"/);

  return {
    routeBytes: root.body.length,
    hotReloadVersion: hotReload.payload.version,
    faviconBytes: favicon.body.length,
  };
}

async function assertStaticAsset(baseUrl, requestPath, contentTypePattern, bodyPattern) {
  const assetUrl = endpoint(baseUrl, requestPath);
  const asset = await fetchText(assetUrl, 10000);
  assert.equal(asset.response.status, 200, asset.body.slice(0, 300));
  assert.match(asset.response.headers.get("content-type") ?? "", contentTypePattern);
  assert.match(asset.response.headers.get("cache-control") ?? "", /no-store/);
  assert.match(asset.body, bodyPattern);

  const expectedLength = Buffer.byteLength(asset.body);
  assert.equal(Number(asset.response.headers.get("content-length")), expectedLength);

  const head = await fetchText(assetUrl, 10000, { method: "HEAD" });
  assert.equal(head.response.status, 200);
  assert.match(head.response.headers.get("content-type") ?? "", contentTypePattern);
  assert.equal(Number(head.response.headers.get("content-length")), expectedLength);
  assert.equal(head.body, "");

  return expectedLength;
}

test("dx dev serves route HTML, hot reload poll receipts, and favicon over real HTTP", async (t) => {
  const envBaseUrl = process.env.DX_WWW_DEV_BASE_URL;
  let server = null;
  let baseUrl = envBaseUrl;
  let mode = "DX_WWW_DEV_BASE_URL";
  let expectedRootMarker = null;

  if (!baseUrl) {
    if (await isDxDevServer(defaultDevBaseUrl)) {
      baseUrl = defaultDevBaseUrl;
      mode = "existing";
    } else {
      server = await startDxDevServer();
      if (!server) {
        t.skip("target/debug/dx-www executable or an existing DX dev server is required");
        return;
      }
      baseUrl = server.baseUrl;
      mode = "spawned";
      expectedRootMarker = "DX dev black-box proof";
      assert.equal(fs.existsSync(path.join(server.projectRoot, "node_modules")), false);
    }
  }

  try {
    const proof = await assertBlackBoxDevServer(baseUrl, expectedRootMarker);
    t.diagnostic(
      `dx dev black-box proof (${mode}) ${baseUrl}: routeBytes=${proof.routeBytes}, hotReloadVersion=${proof.hotReloadVersion}, faviconBytes=${proof.faviconBytes}`,
    );
  } finally {
    if (server) {
      await server.stop();
    }
  }
});

test("dx dev starts on an explicit free port and serves source-owned static assets", async (t) => {
  let server = await startDxDevServer();
  if (!server) {
    t.skip("target/debug/dx-www executable is required for spawned dev-server proof");
    return;
  }

  try {
    const spawnedBaseUrl = server.baseUrl;
    const spawnedProjectRoot = server.projectRoot;
    assert.equal(fs.existsSync(path.join(server.projectRoot, "node_modules")), false);
    const proof = await assertBlackBoxDevServer(server.baseUrl, "DX dev black-box proof");
    const cssBytes = await assertStaticAsset(
      server.baseUrl,
      "/styles/generated.css?dev=1",
      /text\/css/i,
      /\.p-4\s*\{/,
    );
    const publicRobotsBytes = await assertStaticAsset(
      server.baseUrl,
      "/public/robots.txt?dev=1",
      /text\/plain/i,
      /Disallow: \/node_modules/,
    );

    assert.equal(fs.existsSync(path.join(server.projectRoot, "node_modules")), false);
    await server.stop();
    server = null;

    assert.equal(fs.existsSync(spawnedProjectRoot), false);
    assert.equal(await isDxDevServer(spawnedBaseUrl), false);
    t.diagnostic(
      `dx dev spawned static proof ${spawnedBaseUrl}: routeBytes=${proof.routeBytes}, cssBytes=${cssBytes}, publicRobotsBytes=${publicRobotsBytes}, cleaned=true`,
    );
  } finally {
    if (server) {
      await server.stop();
    }
  }
});

test("dx dev rejects an explicit busy non-DX port without starting a duplicate server", async (t) => {
  if (!fs.existsSync(dxWwwBinary)) {
    t.skip("target/debug/dx-www executable is required for busy-port dev-server proof");
    return;
  }

  const projectRoot = createTinyDevFixture();
  const busyPort = await reserveNonDxDevPort();

  try {
    const result = await runDxDevToExit(
      ["dev", "--host", "127.0.0.1", "--port", String(busyPort.port)],
      projectRoot,
    );
    const output = `${result.stdout}\n${result.stderr}`;

    assert.notEqual(
      result.code,
      0,
      `dx dev unexpectedly succeeded on a non-DX busy port\n${output}`,
    );
    assert.equal(result.signal, null, output);
    assert.ok(
      busyPort.probeHits() >= 1,
      "dx dev should probe the occupied port before rejecting it as non-DX",
    );
    assertBusyPortDiagnostic(output, busyPort.port);
    assert.doesNotMatch(output, /selected available port/);
    assert.doesNotMatch(output, /Development server running at/);
    assert.equal(await isDxDevServer(`http://127.0.0.1:${busyPort.port}`), false);
    assert.equal(fs.existsSync(path.join(projectRoot, "node_modules")), false);

    t.diagnostic(
      `dx dev busy-port proof port=${busyPort.port}: exitCode=${result.code}, probeHits=${busyPort.probeHits()}`,
    );
  } finally {
    await busyPort.close();
    fs.rmSync(projectRoot, { recursive: true, force: true });
  }
});
