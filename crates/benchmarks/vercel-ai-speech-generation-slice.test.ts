const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "vercel-ai");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("Vercel AI slice exposes real speech generation helpers from upstream API", () => {
  const upstreamDocs = read(
    path.join(
      mirror,
      "content",
      "docs",
      "07-reference",
      "01-ai-sdk-core",
      "12-generate-speech.mdx",
    ),
  );
  const upstreamIndex = read(
    path.join(mirror, "packages", "ai", "src", "generate-speech", "index.ts"),
  );
  const upstreamSource = read(
    path.join(
      mirror,
      "packages",
      "ai",
      "src",
      "generate-speech",
      "generate-speech.ts",
    ),
  );
  const upstreamResult = read(
    path.join(
      mirror,
      "packages",
      "ai",
      "src",
      "generate-speech",
      "generate-speech-result.ts",
    ),
  );
  const upstreamAudio = read(
    path.join(
      mirror,
      "packages",
      "ai",
      "src",
      "generate-speech",
      "generated-audio-file.ts",
    ),
  );
  const upstreamExample = read(
    path.join(
      mirror,
      "examples",
      "ai-functions",
      "src",
      "generate-speech",
      "openai",
      "basic.ts",
    ),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_vercel_ai.rs"));
  const launchProof = read(path.join(root, "examples", "template", "ai-chat-status.tsx"));
  const security = read(path.join(root, "core", "src", "ecosystem", "forge_security.rs"));

  assert.match(upstreamDocs, /experimental_generateSpeech as generateSpeech/);
  assert.match(upstreamDocs, /SpeechModelV4/);
  assert.match(upstreamDocs, /GeneratedAudioFile/);
  assert.match(upstreamIndex, /experimental_generateSpeech/);
  assert.match(upstreamIndex, /Experimental_SpeechResult/);
  assert.match(upstreamSource, /export async function generateSpeech/);
  assert.match(upstreamResult, /interface SpeechResult/);
  assert.match(upstreamAudio, /interface GeneratedAudioFile/);
  assert.match(upstreamExample, /openai\.speech/);

  assert.match(slice, /"js\/lib\/ai\/speech-generation\.ts"/);
  assert.match(slice, /"js\/app\/api\/ai\/speech\/route\.ts"/);
  assert.match(slice, /experimental_generateSpeech as generateSpeech/);
  assert.match(slice, /Experimental_SpeechResult/);
  assert.match(slice, /GeneratedAudioFile/);
  assert.match(slice, /SpeechModel/);
  assert.match(slice, /generateDxLaunchSpeechAudio/);
  assert.match(slice, /speechGeneration: "generateDxLaunchSpeechAudio/);

  assert.match(launchProof, /generateDxLaunchSpeechAudio/);
  assert.match(launchProof, /data-dx-ai-speech-generation/);
  assert.match(security, /speech generation/);
});
