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
const bodyParserSource = path.join(repoRoot, "dx-www", "src", "cli", "dev_http.rs");

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

function createFormDataFixture(port) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-route-handler-formdata-"));
  writeFile(
    root,
    "dx",
    `project.name="route-handler-formdata-live-black-box"
dev.host="127.0.0.1"
dev.port=${port}
dev.hot_reload=false
`,
  );
  writeFile(
    root,
    "app/page.tsx",
    `export default function Page() {
  return <main>form data live fixture</main>;
}
`,
  );
  writeFile(
    root,
    "app/api/signup/route.ts",
    `export async function POST(request: Request) {
  const form = await request.formData();
  const cloned = request.clone();
  const url = new URL(request.url);

  return Response.json(
    {
      email: form.get("email") ?? "missing",
      plan: form.get("plan") ?? "free",
      directEmail: (await request.formData()).get("email") ?? "missing",
      cloneEmail: (await request.clone().formData()).get("email") ?? "missing",
      clonedEmail: (await cloned.formData()).get("email") ?? "missing",
      source: url.searchParams.get("source") ?? "missing",
      probe: request.headers.get("x-dx-probe") ?? "missing",
      session: request.cookies.get("session")?.value ?? "missing",
    },
    {
      status: 202,
      headers: {
        "cache-control": "no-store",
        "x-dx-formdata": "ok",
      },
    },
  );
}
`,
  );
  writeFile(
    root,
    "app/api/custom-json/route.ts",
    `export async function POST(request: Request) {
  const body = await request.json();
  const raw = await request.clone().text();

  return Response.json(
    {
      title: body.title ?? "missing",
      count: body.count ?? 0,
      raw,
      probe: request.headers.get("x-dx-probe") ?? "missing",
    },
    {
      status: 207,
      headers: {
        "x-dx-custom-json": "ok",
      },
    },
  );
}
`,
  );
  writeFile(
    root,
    "app/api/text/route.ts",
    `export async function POST(request: Request) {
  const text = await request.text();
  const cloneText = await request.clone().text();

  return Response.json(
    {
      text,
      cloneText,
      probe: request.headers.get("x-dx-probe") ?? "missing",
    },
    {
      status: 206,
      headers: {
        "x-dx-text-body": "ok",
      },
    },
  );
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
      if (match) {
        clearTimeout(timeout);
        resolve(match[1]);
      }
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

test("dx dev projects urlencoded request.formData() through route handlers", async (t) => {
  if (!fs.existsSync(dxWwwBinary)) {
    t.skip("target/debug/dx-www executable is required for live route-handler proof");
    return;
  }
  if (fs.existsSync(bodyParserSource) && fs.statSync(bodyParserSource).mtimeMs > fs.statSync(dxWwwBinary).mtimeMs) {
    t.skip("target/debug/dx-www is older than dx-www/src/cli/dev_http.rs; rebuild before live formData proof");
    return;
  }

  const port = await freePort();
  const projectRoot = createFormDataFixture(port);
  const child = spawn(
    dxWwwBinary,
    ["dev", "--host", "127.0.0.1", "--port", String(port), "--no-hot-reload"],
    {
      cwd: projectRoot,
      stdio: ["ignore", "pipe", "pipe"],
      windowsHide: true,
    },
  );

  t.after(async () => {
    await stopDevServer(child);
    fs.rmSync(projectRoot, { recursive: true, force: true });
  });

  const baseUrl = await waitForDevServer(child);
  const form = new URLSearchParams([
    ["email", "ada+launch@example.com"],
    ["plan", "team space"],
    ["plan", "enterprise"],
  ]);
  const response = await fetch(`${baseUrl}/api/signup?source=launch+template`, {
    method: "POST",
    headers: {
      "content-type": "application/x-www-form-urlencoded;charset=UTF-8",
      cookie: "session=form123",
      "x-dx-probe": "formdata-live",
    },
    body: form.toString(),
  });
  const body = await readJson(response);

  assert.equal(response.status, 202);
  assert.equal(response.headers.get("x-dx-route-handler-receipt"), "dx.next.appRouteHandlerReceipt");
  assert.equal(response.headers.get("x-dx-node-modules-required"), "false");
  assert.equal(response.headers.get("x-dx-external-runtime-executed"), "false");
  assert.equal(response.headers.get("x-dx-formdata"), "ok");
  assert.deepEqual(body, {
    email: "ada+launch@example.com",
    plan: "team space",
    directEmail: "ada+launch@example.com",
    cloneEmail: "ada+launch@example.com",
    clonedEmail: "ada+launch@example.com",
    source: "launch template",
    probe: "formdata-live",
    session: "form123",
  });
  assert.deepEqual(findNodeModulesDirs(projectRoot), []);

  const customJsonPayload = { title: "Launch body", count: 3 };
  const customJsonResponse = await fetch(`${baseUrl}/api/custom-json`, {
    method: "POST",
    headers: {
      "content-type": "application/vnd.dx.route+json; charset=UTF-8",
      "x-dx-probe": "custom-json-live",
    },
    body: JSON.stringify(customJsonPayload),
  });
  const customJsonBody = await readJson(customJsonResponse);

  assert.equal(customJsonResponse.status, 207);
  assert.equal(customJsonResponse.headers.get("x-dx-route-handler-receipt"), "dx.next.appRouteHandlerReceipt");
  assert.equal(customJsonResponse.headers.get("x-dx-custom-json"), "ok");
  assert.equal(customJsonBody.title, "Launch body");
  assert.equal(customJsonBody.count, 3);
  assert.equal(customJsonBody.probe, "custom-json-live");
  assert.deepEqual(JSON.parse(customJsonBody.raw), customJsonPayload);

  const textPayload = "DX route handler text body";
  const textResponse = await fetch(`${baseUrl}/api/text`, {
    method: "POST",
    headers: {
      "content-type": "text/plain; charset=UTF-8",
      "x-dx-probe": "text-live",
    },
    body: textPayload,
  });
  const textBody = await readJson(textResponse);

  assert.equal(textResponse.status, 206);
  assert.equal(textResponse.headers.get("x-dx-route-handler-receipt"), "dx.next.appRouteHandlerReceipt");
  assert.equal(textResponse.headers.get("x-dx-text-body"), "ok");
  assert.deepEqual(textBody, {
    text: textPayload,
    cloneText: textPayload,
    probe: "text-live",
  });
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
