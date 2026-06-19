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

function writeFile(root, relativePath, content) {
  const filePath = path.join(root, relativePath);
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, content);
}

function createHotReloadFixture(port) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-hot-reload-live-"));
  writeFile(
    root,
    "dx",
    `project.name="hot-reload-live-edit"
dev.host="127.0.0.1"
dev.port=${port}
dev.hot_reload=true
`,
  );
  writeFile(
    root,
    "app/page.tsx",
    `export default function Page() {
  return <main data-hot-reload-live="initial">Hot reload live proof</main>;
}
`,
  );
  writeFile(
    root,
    "app/api/ping/route.ts",
    `export function GET() {
  return Response.json({ ok: true, version: "initial" });
}
`,
  );
  writeFile(root, "styles/app.css", ".hot-reload-live { color: red; }\n");
  writeFile(root, "public/logo.svg", "<svg><title>initial</title></svg>\n");
  return root;
}

async function freePort() {
  const server = net.createServer();
  await new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(0, "127.0.0.1", resolve);
  });
  const { port } = server.address();
  await new Promise((resolve, reject) =>
    server.close((error) => (error ? reject(error) : resolve())),
  );
  return port;
}

async function fetchJson(url, timeoutMs = 2500) {
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), timeoutMs);
  try {
    const response = await fetch(url, { signal: controller.signal });
    const text = await response.text();
    return { response, payload: JSON.parse(text) };
  } finally {
    clearTimeout(timer);
  }
}

async function waitForDxDevReady(baseUrl, child, log) {
  const deadline = Date.now() + 45000;
  let lastError = "server did not respond";
  while (Date.now() < deadline) {
    if (child.exitCode !== null || child.signalCode !== null) {
      throw new Error(`dx dev exited before ready\n${log()}`);
    }
    try {
      const { response, payload } = await fetchJson(
        `${baseUrl}/_dx/hot-reload/version?resource=route%3A%2F`,
      );
      if (
        response.status === 200 &&
        payload.ok === true &&
        payload.protocol === "dx.hot-reload.poll" &&
        payload.receipt?.source === "dx-www-rust-dev-server"
      ) {
        return;
      }
    } catch (error) {
      lastError = error.message;
    }
    await new Promise((resolve) => setTimeout(resolve, 250));
  }
  throw new Error(`dx dev did not become ready at ${baseUrl}: ${lastError}\n${log()}`);
}

async function stopDevServer(child) {
  if (child.exitCode !== null || child.signalCode !== null) {
    return;
  }
  await new Promise((resolve) => {
    const force = setTimeout(() => child.kill("SIGKILL"), 3000);
    child.once("exit", () => {
      clearTimeout(force);
      resolve();
    });
    child.kill();
  });
}

function parseSseFrames(buffer) {
  const frames = [];
  let rest = buffer;
  for (;;) {
    const index = rest.indexOf("\n\n");
    if (index === -1) {
      return { frames, rest };
    }
    const raw = rest.slice(0, index);
    rest = rest.slice(index + 2);
    const data = raw
      .split(/\r?\n/)
      .filter((line) => line.startsWith("data: "))
      .map((line) => line.slice("data: ".length))
      .join("\n");
    if (data) {
      frames.push(JSON.parse(data));
    }
  }
}

async function openHotReloadStream(baseUrl) {
  const response = await fetch(`${baseUrl}/_dx/hot-reload/events?resource=route%3A%2F`, {
    headers: { Accept: "text/event-stream" },
  });
  if (response.status === 404) {
    await response.body?.cancel();
    return null;
  }
  assert.equal(response.status, 200);
  assert.match(response.headers.get("content-type") ?? "", /text\/event-stream/i);
  assert.equal(response.headers.get("x-dx-hot-reload"), "sse");

  const reader = response.body.getReader();
  const decoder = new TextDecoder();
  let buffer = "";

  async function nextPayload(predicate, label) {
    const deadline = Date.now() + 8000;
    while (Date.now() < deadline) {
      const remaining = deadline - Date.now();
      const read = await Promise.race([
        reader.read(),
        new Promise((_, reject) =>
          setTimeout(() => reject(new Error(`timed out waiting for ${label}`)), remaining),
        ),
      ]);
      assert.equal(read.done, false, `SSE stream ended before ${label}`);
      buffer += decoder.decode(read.value, { stream: true }).replace(/\r\n/g, "\n");
      const parsed = parseSseFrames(buffer);
      buffer = parsed.rest;
      for (const payload of parsed.frames) {
        if (predicate(payload)) {
          return payload;
        }
      }
    }
    throw new Error(`timed out waiting for ${label}`);
  }

  return {
    nextPayload,
    close: () => reader.cancel().catch(() => {}),
  };
}

async function writeUntilEvent(root, relativePath, makeContent, stream, predicate, label) {
  for (let attempt = 0; attempt < 5; attempt += 1) {
    writeFile(root, relativePath, makeContent(attempt));
    await new Promise((resolve) => setTimeout(resolve, 150));
  }
  return await stream.nextPayload(predicate, label);
}

async function pollHotReloadResource(baseUrl, resource) {
  const encoded = encodeURIComponent(resource);
  const { response, payload } = await fetchJson(
    `${baseUrl}/_dx/hot-reload/version?resource=${encoded}`,
    5000,
  );
  assert.equal(response.status, 200);
  assert.equal(payload.ok, true);
  assert.equal(payload.protocol, "dx.hot-reload.poll");
  assert.equal(payload.transport, "poll");
  assert.equal(payload.receipt?.source, "dx-www-rust-dev-server");
  assert.equal(JSON.stringify(payload).toLowerCase().includes("turbopack"), false);
  return payload;
}

async function writeUntilPollTokenChanges(root, baseUrl, resource, relativePath, makeContent) {
  const before = await pollHotReloadResource(baseUrl, resource);
  for (let attempt = 0; attempt < 5; attempt += 1) {
    writeFile(root, relativePath, makeContent(attempt));
    await new Promise((resolve) => setTimeout(resolve, 175));
  }

  const deadline = Date.now() + 8000;
  while (Date.now() < deadline) {
    const after = await pollHotReloadResource(baseUrl, resource);
    if (after.token !== before.token) {
      return after;
    }
    await new Promise((resolve) => setTimeout(resolve, 250));
  }
  throw new Error(`hot reload polling token did not change for ${resource}`);
}

async function assertPollingFallbackEdits(projectRoot, baseUrl) {
  const style = await writeUntilPollTokenChanges(
    projectRoot,
    baseUrl,
    "style:styles/app.css",
    "styles/app.css",
    (attempt) => `.hot-reload-live { color: rgb(${attempt}, 2, 3); }\n`,
  );
  assert.equal(style.resource?.id, "style:styles/app.css");
  assert.equal(style.instruction?.type, "refresh-style");
  assert.equal(style.instruction?.mode, "stylesheet-link");

  const asset = await writeUntilPollTokenChanges(
    projectRoot,
    baseUrl,
    "asset:logo.svg",
    "public/logo.svg",
    (attempt) => `<svg><title>asset-${attempt}</title></svg>\n`,
  );
  assert.equal(asset.resource?.id, "asset:logo.svg");
  assert.equal(asset.instruction?.type, "refresh-asset");
  assert.equal(asset.instruction?.mode, "dom-asset-url");

  const page = await writeUntilPollTokenChanges(
    projectRoot,
    baseUrl,
    "route:/",
    "app/page.tsx",
    (attempt) => `export default function Page() {
  return <main data-hot-reload-live="page-${attempt}">Hot reload live proof</main>;
}
`,
  );
  assert.equal(page.resource?.id, "route:/");
  assert.equal(page.instruction?.type, "restart");
  assert.equal(page.receipt?.boundaries?.partial_module_updates, false);

  const routeHandler = await writeUntilPollTokenChanges(
    projectRoot,
    baseUrl,
    "route:/api/ping",
    "app/api/ping/route.ts",
    (attempt) => `export function GET() {
  return Response.json({ ok: true, version: "route-${attempt}" });
}
`,
  );
  assert.equal(routeHandler.resource?.id, "route:/api/ping");
  assert.equal(routeHandler.instruction?.type, "restart");
  assert.equal(routeHandler.receipt?.boundaries?.partial_module_updates, false);
}

test("dx dev hot reload reports page, style, asset, and route-handler edits over SSE or polling fallback", async (t) => {
  if (!fs.existsSync(dxWwwBinary)) {
    t.skip("target/debug/dx-www executable is required for live hot reload proof");
    return;
  }

  const port = await freePort();
  const projectRoot = createHotReloadFixture(port);
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
  const log = () => `stdout:\n${stdout.slice(-4000)}\nstderr:\n${stderr.slice(-4000)}`;

  t.after(async () => {
    await stopDevServer(child);
    fs.rmSync(projectRoot, { recursive: true, force: true });
  });

  await waitForDxDevReady(baseUrl, child, log);
  assert.equal(fs.existsSync(path.join(projectRoot, "node_modules")), false);

  const stream = await openHotReloadStream(baseUrl);
  if (!stream) {
    await assertPollingFallbackEdits(projectRoot, baseUrl);
    t.diagnostic("SSE endpoint unavailable in this binary; verified dx.hot-reload.poll fallback");
    return;
  }

  t.after(() => stream.close());
  const initial = await stream.nextPayload(
    (payload) => payload.event_stream?.initial === true,
    "initial hot reload frame",
  );
  assert.equal(initial.resource?.id, "route:/");
  assert.equal(initial.transport, "sse");
  assert.equal(JSON.stringify(initial).toLowerCase().includes("turbopack"), false);

  const style = await writeUntilEvent(
    projectRoot,
    "styles/app.css",
    (attempt) => `.hot-reload-live { color: rgb(${attempt}, 2, 3); }\n`,
    stream,
    (payload) => payload.resource?.id === "style:styles/app.css",
    "style edit event",
  );
  assert.equal(style.instruction?.type, "refresh-style");
  assert.equal(style.instruction?.mode, "stylesheet-link");
  assert.equal(style.event_stream?.initial, false);

  const asset = await writeUntilEvent(
    projectRoot,
    "public/logo.svg",
    (attempt) => `<svg><title>asset-${attempt}</title></svg>\n`,
    stream,
    (payload) => payload.resource?.id === "asset:logo.svg",
    "asset edit event",
  );
  assert.equal(asset.instruction?.type, "refresh-asset");
  assert.equal(asset.instruction?.mode, "dom-asset-url");
  assert.equal(asset.event_stream?.initial, false);

  const page = await writeUntilEvent(
    projectRoot,
    "app/page.tsx",
    (attempt) => `export default function Page() {
  return <main data-hot-reload-live="page-${attempt}">Hot reload live proof</main>;
}
`,
    stream,
    (payload) => payload.resource?.id === "route:/",
    "page edit event",
  );
  assert.equal(page.instruction?.type, "restart");
  assert.equal(page.receipt?.boundaries?.partial_module_updates, false);
  assert.equal(page.event_stream?.initial, false);

  const routeHandler = await writeUntilEvent(
    projectRoot,
    "app/api/ping/route.ts",
    (attempt) => `export function GET() {
  return Response.json({ ok: true, version: "route-${attempt}" });
}
`,
    stream,
    (payload) => payload.resource?.id === "route:/api/ping",
    "route-handler edit event",
  );
  assert.equal(routeHandler.instruction?.type, "restart");
  assert.equal(routeHandler.receipt?.source, "dx-www-rust-dev-server");
  assert.equal(routeHandler.event_stream?.initial, false);
  assert.equal(JSON.stringify(routeHandler).toLowerCase().includes("turbopack"), false);
});
