const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "vercel-ai");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("Vercel AI slice exposes real structured output helpers from upstream API", () => {
  const upstreamExample = read(
    path.join(mirror, "examples", "ai-functions", "src", "generate-text", "gateway", "output-object.ts"),
  );
  const upstreamMigration = read(
    path.join(mirror, "content", "docs", "08-migration-guides", "23-migration-guide-7-0.mdx"),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_vercel_ai.rs"));
  const launchProof = read(path.join(root, "examples", "template", "ai-chat-status.tsx"));
  const packageGuard = read(path.join(root, "benchmarks", "launch-package-slices.test.ts"));

  assert.match(upstreamExample, /import \{ generateText, Output \} from 'ai';/);
  assert.match(upstreamExample, /output: Output\.object\(/);
  assert.match(upstreamMigration, /Replace all remaining usages with `output`/);

  assert.match(slice, /"js\/lib\/ai\/structured-output\.ts"/);
  assert.match(slice, /generateText/);
  assert.match(slice, /Output\.object/);
  assert.match(slice, /dxLaunchStructuredStatusSchema/);
  assert.match(slice, /generateDxLaunchStructuredStatus/);
  assert.match(slice, /structuredOutput: "generateDxLaunchStructuredStatus/);
  assert.match(slice, /structured output policy/);

  assert.match(launchProof, /generateDxLaunchStructuredStatus/);
  assert.match(launchProof, /data-dx-ai-structured-output/);
  assert.match(packageGuard, /Output\.object/);
});
