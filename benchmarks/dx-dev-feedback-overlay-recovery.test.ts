const assert = require("node:assert/strict");
const { spawn } = require("node:child_process");
const fs = require("node:fs");
const http = require("node:http");
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

function writeFixtureFile(projectRoot, relativePath, content) {
  const filePath = path.join(projectRoot, relativePath);
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, content);
}

function delay(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function reservePort() {
  return new Promise((resolve, reject) => {
    const server = net.createServer();
    server.on("error", reject);
    server.listen(0, "127.0.0.1", () => {
      const address = server.address();
      server.close(() => resolve(address.port));
    });
  });
}

function getJson(url) {
  return new Promise((resolve, reject) => {
    const request = http.get(url, (response) => {
      let body = "";
      response.setEncoding("utf8");
      response.on("data", (chunk) => {
        body += chunk;
      });
      response.on("end", () => {
        try {
          resolve({
            status: response.statusCode,
            headers: response.headers,
            body: JSON.parse(body),
          });
        } catch (error) {
          reject(
            new Error(
              `Expected JSON from ${url}, got ${response.statusCode}: ${body.slice(0, 240)}`,
            ),
          );
        }
      });
    });
    request.on("error", reject);
    request.setTimeout(1500, () => {
      request.destroy(new Error(`Timed out requesting ${url}`));
    });
  });
}

function getText(url) {
  return new Promise((resolve, reject) => {
    const request = http.get(url, (response) => {
      let body = "";
      response.setEncoding("utf8");
      response.on("data", (chunk) => {
        body += chunk;
      });
      response.on("end", () => {
        resolve({
          status: response.statusCode,
          headers: response.headers,
          body,
        });
      });
    });
    request.on("error", reject);
    request.setTimeout(1500, () => {
      request.destroy(new Error(`Timed out requesting ${url}`));
    });
  });
}

function parseDevFeedbackEvent(body) {
  assert.match(body, /^event: dx-dev-feedback$/m);
  const dataLine = body
    .split(/\r?\n/)
    .find((line) => line.startsWith("data: "));
  assert.ok(dataLine, `expected dx-dev-feedback data line in:\n${body}`);
  return JSON.parse(dataLine.slice("data: ".length));
}

async function waitForJson(url, logs) {
  const deadline = Date.now() + 15_000;
  let lastError = "not attempted";
  while (Date.now() < deadline) {
    try {
      const response = await getJson(url);
      if (response.status === 200) {
        return response;
      }
      lastError = `HTTP ${response.status}`;
    } catch (error) {
      lastError = error.message;
    }
    await delay(200);
  }
  throw new Error(`Timed out waiting for ${url}: ${lastError}\n${logs()}`);
}

function waitForChildClose(child) {
  return new Promise((resolve) => {
    if (child.exitCode !== null || child.signalCode) {
      resolve();
      return;
    }
    child.once("close", resolve);
  });
}

async function stopChild(child) {
  if (!child || child.exitCode !== null || child.signalCode) {
    return;
  }
  child.kill();
  await Promise.race([waitForChildClose(child), delay(3000)]);
}

test("dx dev feedback errors endpoint proves overlay issue and recovery through the CLI server", async (t) => {
  if (!fs.existsSync(dxWwwBinary)) {
    t.skip("target/debug/dx-www executable is required for this black-box dev feedback proof");
    return;
  }

  const projectRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-overlay-recovery-"));
  const port = await reservePort();
  let child;
  let stdout = "";
  let stderr = "";

  try {
    writeFixtureFile(
      projectRoot,
      "dx",
      `project.name="overlay-recovery"
dev.host="127.0.0.1"
dev.port=3000
`,
    );
    writeFixtureFile(
      projectRoot,
      "app/page.tsx",
      `export default function Page() {
  return <main>
}
`,
    );
    await delay(30);
    writeFixtureFile(
      projectRoot,
      ".dx/diagnostics/latest.json",
      JSON.stringify({
        issues: [
          {
            severity: "error",
            diagnostic_code: "dx.source.parse_error",
            title: "Compile failed",
            message: "Unclosed JSX element",
            file: "app/page.tsx",
            line: 2,
            column: 10,
            endLine: 2,
            endColumn: 16,
          },
        ],
      }),
    );

    child = spawn(dxWwwBinary, ["dev", "--host", "127.0.0.1", "--port", String(port)], {
      cwd: projectRoot,
      stdio: ["ignore", "pipe", "pipe"],
      windowsHide: true,
    });
    child.stdout.on("data", (chunk) => {
      stdout += chunk;
    });
    child.stderr.on("data", (chunk) => {
      stderr += chunk;
    });

    const errorsUrl = `http://127.0.0.1:${port}/_dx/feedback/errors`;
    const first = await waitForJson(errorsUrl, () => `${stdout}\n${stderr}`);
    assert.equal(first.headers["x-dx-dev-feedback"], "json");
    assert.equal(first.body.schema, "dx.dev_feedback.errors");
    assert.equal(first.body.issue_count, 1, JSON.stringify(first.body, null, 2));
    assert.equal(first.body.highest_severity, "error");
    assert.equal(first.body.recovery.status, "active-error");
    assert.equal(first.body.recovery.overlay_action, "show-overlay");
    assert.equal(first.body.recovery.clears_overlay, false);
    assert.equal(first.body.recovery.requires_full_reload, false);
    assert.equal(first.body.recovery.source_owned_contract, true);
    assert.equal(first.body.node_modules_required, false);
    assert.equal(first.body.next_runtime, false);
    assert.equal(first.body.turbopack_hmr, false);
    assert.equal(first.body.issues[0].diagnostic_code, "dx.source.parse_error");
    assert.equal(first.body.issues[0].code_frame_source, "source-file");
    assert.equal(first.body.issues[0].code_frame_adapter_boundary, false);
    assert.match(first.body.issues[0].code_frame, /> 2 \|   return <main>/);
    assert.match(first.body.issues[0].code_frame, /\^\^\^\^\^\^/);

    writeFixtureFile(
      projectRoot,
      "app/page.tsx",
      `export default function Page() {
  return <main>Recovered</main>;
}
`,
    );
    await delay(30);
    writeFixtureFile(projectRoot, ".dx/diagnostics/latest.json", JSON.stringify({ issues: [] }));

    const recovered = await getJson(errorsUrl);
    assert.equal(recovered.status, 200);
    assert.equal(recovered.body.schema, "dx.dev_feedback.errors");
    assert.equal(recovered.body.issue_count, 0, JSON.stringify(recovered.body, null, 2));
    assert.equal(recovered.body.highest_severity, null);
    assert.equal(recovered.body.next_action.type, "clear-overlay");
    assert.equal(recovered.body.recovery.status, "recovered");
    assert.equal(recovered.body.recovery.overlay_action, "clear-overlay");
    assert.equal(recovered.body.recovery.clears_overlay, true);
    assert.equal(recovered.body.recovery.requires_full_reload, false);
    assert.equal(recovered.body.recovery.diagnostics_artifact_status, "current");
    assert.equal(recovered.body.recovery.source_owned_contract, true);
    assert.equal(recovered.body.recovery.node_modules_required, false);
    assert.equal(recovered.body.recovery.next_runtime, false);
    assert.equal(recovered.body.recovery.turbopack_hmr, false);
    assert.equal(recovered.body.diagnostics_artifact.status, "current");
    assert.deepEqual(recovered.body.issues, []);
    assert.equal(recovered.body.node_modules_required, false);
    assert.equal(recovered.body.next_runtime, false);
    assert.equal(recovered.body.turbopack_hmr, false);

    const eventResponse = await getText(`http://127.0.0.1:${port}/_dx/feedback/events`);
    assert.equal(eventResponse.status, 200);
    assert.match(eventResponse.headers["content-type"], /^text\/event-stream/);
    const eventPayload = parseDevFeedbackEvent(eventResponse.body);
    assert.equal(eventPayload.schema, "dx.dev_feedback.events");
    assert.equal(eventPayload.errors.issue_count, 0);
    assert.equal(eventPayload.errors.next_action.type, "clear-overlay");
    assert.equal(eventPayload.errors.recovery.status, "recovered");
    assert.equal(eventPayload.errors.recovery.clears_overlay, true);
    assert.equal(eventPayload.errors.recovery.diagnostics_artifact_status, "current");
    assert.equal(eventPayload.node_modules_required, false);
    assert.equal(eventPayload.next_runtime, false);
  } finally {
    await stopChild(child);
    fs.rmSync(projectRoot, { recursive: true, force: true });
  }
});
