const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "vercel-ai");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("Vercel AI slice exposes real telemetry helpers from upstream API", () => {
  const upstreamDocs = read(
    path.join(mirror, "content", "docs", "03-ai-sdk-core", "60-telemetry.mdx"),
  );
  const upstreamEvents = read(
    path.join(mirror, "content", "docs", "03-ai-sdk-core", "65-event-listeners.mdx"),
  );
  const upstreamTelemetryIndex = read(
    path.join(mirror, "packages", "ai", "src", "telemetry", "index.ts"),
  );
  const upstreamTelemetry = read(
    path.join(mirror, "packages", "ai", "src", "telemetry", "telemetry.ts"),
  );
  const upstreamTelemetryOptions = read(
    path.join(mirror, "packages", "ai", "src", "telemetry", "telemetry-options.ts"),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_vercel_ai.rs"));
  const launchProof = read(path.join(root, "examples", "template", "ai-chat-status.tsx"));
  const security = read(path.join(root, "core", "src", "ecosystem", "forge_security.rs"));

  assert.match(upstreamDocs, /registerTelemetry/);
  assert.match(upstreamDocs, /telemetry: \{/);
  assert.match(upstreamDocs, /includeRuntimeContext/);
  assert.match(upstreamEvents, /onStepFinish/);
  assert.match(upstreamEvents, /experimental_onEnd/);
  assert.match(upstreamTelemetryIndex, /export type \{ TelemetryOptions \}/);
  assert.match(upstreamTelemetryIndex, /export \{ registerTelemetry \}/);
  assert.match(upstreamTelemetry, /onEmbedEnd/);
  assert.match(upstreamTelemetry, /onStepFinish/);
  assert.match(upstreamTelemetryOptions, /functionId\?: string/);
  assert.match(upstreamTelemetryOptions, /recordInputs\?: boolean/);
  assert.match(upstreamTelemetryOptions, /includeRuntimeContext/);

  assert.match(slice, /"js\/lib\/ai\/telemetry\.ts"/);
  assert.match(slice, /registerTelemetry/);
  assert.match(slice, /type Telemetry/);
  assert.match(slice, /type TelemetryOptions/);
  assert.match(slice, /createDxLaunchTelemetry/);
  assert.match(slice, /createDxLaunchTelemetryOptions/);
  assert.match(slice, /registerDxLaunchTelemetry/);
  assert.match(slice, /telemetry: "createDxLaunchTelemetryOptions/);

  assert.match(launchProof, /createDxLaunchTelemetryOptions/);
  assert.match(launchProof, /data-dx-ai-telemetry/);
  assert.match(security, /telemetry/);
});
