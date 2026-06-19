const { spawn, spawnSync } = require("node:child_process");
const http = require("node:http");

async function startPreview({ baseUrl, binary, options, template }) {
  const stdout = [];
  const stderr = [];
  const child = spawn(
    binary,
    ["www", "dev", "--host", options.host, "--port", String(options.port)],
    {
      cwd: template,
      windowsHide: true,
    },
  );
  child.stdout.on("data", (chunk) => pushLines(stdout, chunk));
  child.stderr.on("data", (chunk) => pushLines(stderr, chunk));

  const ready = await waitForPreview({
    baseUrl,
    child,
    timeoutMs: options.devStartTimeoutMs,
  });

  return {
    child,
    exitCode: child.exitCode,
    ready,
    stderrTail: stderr.slice(-20),
    stdoutTail: stdout.slice(-20),
    summary: {
      exitCode: child.exitCode,
      pid: child.pid,
      ready,
      stderrTail: stderr.slice(-20),
      stdoutTail: stdout.slice(-20),
    },
  };
}

async function checkExistingPreview({ baseUrl, timeoutMs }) {
  const ready = await httpGet(`${baseUrl}/_dx/hot-reload/version`, timeoutMs);
  return {
    mode: "existing-preview",
    ready,
    started: false,
  };
}

function waitForPreview({ baseUrl, child, timeoutMs }) {
  const deadline = Date.now() + timeoutMs;
  return new Promise((resolve) => {
    const poll = () => {
      if (child.exitCode !== null) {
        resolve(false);
        return;
      }
      httpGet(`${baseUrl}/_dx/hot-reload/version`, 2000).then((ok) => {
        if (ok) {
          resolve(true);
        } else if (Date.now() >= deadline) {
          resolve(false);
        } else {
          setTimeout(poll, 500);
        }
      });
    };
    poll();
  });
}

function httpGet(url, timeoutMs) {
  return new Promise((resolve) => {
    const request = http.get(url, (response) => {
      response.resume();
      resolve(response.statusCode >= 200 && response.statusCode < 500);
    });
    request.setTimeout(timeoutMs, () => {
      request.destroy();
      resolve(false);
    });
    request.on("error", () => resolve(false));
  });
}

function stopPreview(child) {
  if (!child || child.exitCode !== null) {
    return Promise.resolve();
  }
  return new Promise((resolve) => {
    child.once("exit", () => resolve());
    child.kill();
    setTimeout(() => {
      if (child.exitCode === null && child.pid) {
        spawnSync("taskkill.exe", ["/PID", String(child.pid), "/T", "/F"], {
          windowsHide: true,
        });
      }
      resolve();
    }, 5000);
  });
}

function pushLines(lines, chunk) {
  lines.push(...String(chunk).split(/\r?\n/).filter(Boolean));
  while (lines.length > 80) {
    lines.shift();
  }
}

module.exports = {
  checkExistingPreview,
  startPreview,
  stopPreview,
};
