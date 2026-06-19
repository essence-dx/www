const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "vercel-ai");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("Vercel AI slice exposes real transcription helpers from upstream API", () => {
  const upstreamDocs = read(
    path.join(
      mirror,
      "content",
      "docs",
      "07-reference",
      "01-ai-sdk-core",
      "11-transcribe.mdx",
    ),
  );
  const upstreamIndex = read(path.join(mirror, "packages", "ai", "src", "transcribe", "index.ts"));
  const upstreamSource = read(
    path.join(mirror, "packages", "ai", "src", "transcribe", "transcribe.ts"),
  );
  const upstreamResult = read(
    path.join(mirror, "packages", "ai", "src", "transcribe", "transcribe-result.ts"),
  );
  const upstreamExample = read(
    path.join(
      mirror,
      "examples",
      "ai-functions",
      "src",
      "transcribe",
      "openai",
      "basic.ts",
    ),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_vercel_ai.rs"));
  const launchProof = read(path.join(root, "examples", "template", "ai-chat-status.tsx"));
  const security = read(path.join(root, "core", "src", "ecosystem", "forge_security.rs"));

  assert.match(upstreamDocs, /experimental_transcribe as transcribe/);
  assert.match(upstreamDocs, /TranscriptionModelV4/);
  assert.match(upstreamDocs, /durationInSeconds/);
  assert.match(upstreamIndex, /experimental_transcribe/);
  assert.match(upstreamIndex, /Experimental_TranscriptionResult/);
  assert.match(upstreamSource, /export async function transcribe/);
  assert.match(upstreamSource, /createDownload/);
  assert.match(upstreamResult, /interface TranscriptionResult/);
  assert.match(upstreamExample, /openai\.transcription/);

  assert.match(slice, /"js\/lib\/ai\/transcription\.ts"/);
  assert.match(slice, /"js\/app\/api\/ai\/transcribe\/route\.ts"/);
  assert.match(slice, /experimental_transcribe as transcribe/);
  assert.match(slice, /Experimental_TranscriptionResult/);
  assert.match(slice, /TranscriptionModel/);
  assert.match(slice, /createDownload/);
  assert.match(slice, /transcribeDxLaunchAudio/);
  assert.match(slice, /transcription: "transcribeDxLaunchAudio/);

  assert.match(launchProof, /transcribeDxLaunchAudio/);
  assert.match(launchProof, /data-dx-ai-transcription/);
  assert.match(security, /transcription/);
});
