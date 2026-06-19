const { spawnSync } = require("node:child_process");

function httpProbeStep(id, label, url, expectBodyIncludes, expectContentTypeIncludes = null) {
  return {
    id,
    kind: "http-probe",
    label,
    url,
    expectStatus: 200,
    expectBodyIncludes,
    expectContentTypeIncludes,
    optionalWhenUnavailable: true,
    safeWhen: "only probes an already-running local server",
    startsServer: false,
  };
}

function runHttpProbe(repoRoot, step) {
  const result = spawnSync(
    process.execPath,
    [
      "-e",
      httpProbeScript(),
      step.url,
      String(step.expectStatus),
      step.expectBodyIncludes || "",
      step.expectContentTypeIncludes || "",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      timeout: 10000,
    },
  );
  const unavailable = result.status === 4 || result.status === 5;

  return {
    ...publicStep(step),
    exitCode: result.status,
    passed: result.status === 0 || unavailable,
    status: result.status === 0 ? "passed" : unavailable ? "skipped" : "failed",
    stderrTail: tail(result.stderr),
    stdoutTail: tail(result.stdout),
  };
}

function httpProbeScript() {
  return [
    "const http = require('node:http');",
    "const [url, expected, needle, contentTypeNeedle] = process.argv.slice(1);",
    "const req = http.get(url, { timeout: 5000 }, (res) => {",
    "let body = '';",
    "res.setEncoding('utf8');",
    "res.on('data', (chunk) => { body = `${body}${chunk}`.slice(-4096); });",
    "res.on('end', () => {",
    "if (res.statusCode !== Number(expected)) process.exit(2);",
    "if (needle && !body.includes(needle)) process.exit(3);",
    "if (contentTypeNeedle && !String(res.headers['content-type'] || '').includes(contentTypeNeedle)) process.exit(6);",
    "process.exit(0);",
    "});",
    "});",
    "req.on('timeout', () => { req.destroy(); process.exit(4); });",
    "req.on('error', () => process.exit(5));",
  ].join("");
}

function publicStep(step) {
  const { args, executable, timeoutMs, ...publicFields } = step;
  return publicFields;
}

function tail(value) {
  return value ? value.slice(-4000) : "";
}

module.exports = {
  httpProbeStep,
  runHttpProbe,
};
