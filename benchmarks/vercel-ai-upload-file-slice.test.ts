const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "vercel-ai");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("Vercel AI slice exposes real provider file upload helpers", () => {
  const upstreamIndex = read(
    path.join(mirror, "packages", "ai", "src", "upload-file", "index.ts"),
  );
  const upstreamUpload = read(
    path.join(
      mirror,
      "packages",
      "ai",
      "src",
      "upload-file",
      "upload-file.ts",
    ),
  );
  const upstreamResult = read(
    path.join(
      mirror,
      "packages",
      "ai",
      "src",
      "upload-file",
      "upload-file-result.ts",
    ),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_vercel_ai.rs"));
  const launchProof = read(path.join(root, "examples", "onboard", "ai-chat-status.tsx"));
  const security = read(path.join(root, "core", "src", "ecosystem", "forge_security.rs"));

  assert.match(upstreamIndex, /export \{ uploadFile \}/);
  assert.match(upstreamIndex, /UploadFileResult/);
  assert.match(upstreamUpload, /export async function uploadFile/);
  assert.match(upstreamUpload, /FilesV4 \| ProviderV4/);
  assert.match(upstreamUpload, /The provider does not support file uploads/);
  assert.match(upstreamResult, /interface UploadFileResult/);
  assert.match(upstreamResult, /providerReference/);

  assert.match(slice, /"js\/lib\/ai\/file-upload\.ts"/);
  assert.match(slice, /"js\/app\/api\/ai\/upload-file\/route\.ts"/);
  assert.match(slice, /uploadFile/);
  assert.match(slice, /UploadFileResult/);
  assert.match(slice, /uploadDxLaunchFile/);
  assert.match(slice, /uploadFile: "uploadDxLaunchFile/);

  assert.match(launchProof, /uploadDxLaunchFile/);
  assert.match(launchProof, /data-dx-ai-file-upload/);
  assert.match(security, /file upload/);
});
