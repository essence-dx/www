const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "vercel-ai");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("Vercel AI slice exposes real reranking helpers from upstream API", () => {
  const upstreamDocs = read(
    path.join(mirror, "content", "docs", "03-ai-sdk-core", "31-reranking.mdx"),
  );
  const upstreamReference = read(
    path.join(mirror, "content", "docs", "07-reference", "01-ai-sdk-core", "06-rerank.mdx"),
  );
  const upstreamRerank = read(path.join(mirror, "packages", "ai", "src", "rerank", "rerank.ts"));
  const upstreamTypes = read(
    path.join(mirror, "packages", "ai", "src", "types", "reranking-model.ts"),
  );
  const upstreamIndex = read(path.join(mirror, "packages", "ai", "src", "index.ts"));
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_vercel_ai.rs"));
  const launchProof = read(path.join(root, "examples", "template", "ai-chat-status.tsx"));
  const security = read(path.join(root, "core", "src", "ecosystem", "forge_security.rs"));

  assert.match(upstreamDocs, /import \{ rerank \} from 'ai';/);
  assert.match(upstreamDocs, /rerankedDocuments/);
  assert.match(upstreamReference, /RerankingModel/);
  assert.match(upstreamRerank, /export async function rerank/);
  assert.match(upstreamRerank, /rerankedDocuments/);
  assert.match(upstreamTypes, /export type RerankingModel/);
  assert.match(upstreamIndex, /export \* from '\.\/rerank'/);

  assert.match(slice, /"js\/lib\/ai\/reranking\.ts"/);
  assert.match(slice, /rerank/);
  assert.match(slice, /type RerankingModel/);
  assert.match(slice, /rerankedDocuments/);
  assert.match(slice, /rerankDxLaunchEvidence/);
  assert.match(slice, /reranking: "rerankDxLaunchEvidence/);

  assert.match(launchProof, /rerankDxLaunchEvidence/);
  assert.match(launchProof, /data-dx-ai-reranking/);
  assert.match(security, /reranking model/);
});
