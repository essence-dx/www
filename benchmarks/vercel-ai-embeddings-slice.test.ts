const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "vercel-ai");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("Vercel AI slice exposes real embedding helpers from upstream API", () => {
  const upstreamDocs = read(
    path.join(mirror, "content", "docs", "03-ai-sdk-core", "30-embeddings.mdx"),
  );
  const upstreamRouter = read(
    path.join(
      mirror,
      "examples",
      "ai-functions",
      "src",
      "complex",
      "semantic-router",
      "semantic-router.ts",
    ),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_vercel_ai.rs"));
  const launchProof = read(path.join(root, "examples", "template", "ai-chat-status.tsx"));
  const security = read(path.join(root, "core", "src", "ecosystem", "forge_security.rs"));

  assert.match(upstreamDocs, /import \{ embedMany \} from 'ai';/);
  assert.match(upstreamDocs, /import \{ cosineSimilarity, embedMany \} from 'ai';/);
  assert.match(upstreamRouter, /type EmbeddingModel/);
  assert.match(upstreamRouter, /await embedMany\(/);
  assert.match(upstreamRouter, /await embed\(/);
  assert.match(upstreamRouter, /cosineSimilarity/);

  assert.match(slice, /"js\/lib\/ai\/embeddings\.ts"/);
  assert.match(slice, /type EmbeddingModel/);
  assert.match(slice, /embedMany/);
  assert.match(slice, /embed\(/);
  assert.match(slice, /cosineSimilarity/);
  assert.match(slice, /rankDxLaunchNotesBySimilarity/);
  assert.match(slice, /embeddingSearch: "rankDxLaunchNotesBySimilarity/);

  assert.match(launchProof, /rankDxLaunchNotesBySimilarity/);
  assert.match(launchProof, /data-dx-ai-embeddings/);
  assert.match(security, /embedding model/);
});
