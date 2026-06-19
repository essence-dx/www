import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("DX API router detects HEAD and OPTIONS route exports", () => {
  const api = read("dx-www/src/api/mod.rs");

  assert.match(api, /content\.contains\("pub async fn head"\)/);
  assert.match(api, /content\.contains\("export async function HEAD"\)/);
  assert.match(api, /content\.contains\("async def head"\)/);
  assert.match(api, /content\.contains\("func Head"\)/);
  assert.match(api, /methods\.push\(HttpMethod::Head\)/);

  assert.match(api, /content\.contains\("pub async fn options"\)/);
  assert.match(api, /content\.contains\("export async function OPTIONS"\)/);
  assert.match(api, /content\.contains\("async def options"\)/);
  assert.match(api, /content\.contains\("func Options"\)/);
  assert.match(api, /methods\.push\(HttpMethod::Options\)/);

  assert.match(api, /detect_methods_includes_head_and_options_exports/);
  assert.doesNotMatch(api, /node_modules|_next|turbopack/i);
});
