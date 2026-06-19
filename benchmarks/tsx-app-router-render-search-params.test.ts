import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const executionPath = path.join(root, "dx-www", "src", "cli", "app_router_execution.rs");

function read(filePath) {
  assert.ok(fs.existsSync(filePath), `missing ${path.relative(root, filePath)}`);
  return fs.readFileSync(filePath, "utf8");
}

test("App Router execution page_props records optional and awaited searchParams reads", () => {
  const execution = read(executionPath);

  assert.match(execution, /fn search_params_used_by_page/);
  assert.match(execution, /collect_search_param_accesses\(source, "searchParams"/);
  assert.match(execution, /search_params_used_by_page_records_optional_and_awaited_reads/);
  assert.match(execution, /searchParams\?\.preview/);
  assert.match(execution, /\(await searchParams\)\?\.tab/);
  assert.match(execution, /searchParams\?\.\["mode"\]/);
  assert.match(execution, /\(await searchParams\)\["view"\]/);
});
