import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const { readFileSync } = require("node:fs");
const { resolve } = require("node:path");
const test = require("node:test");

const repoRoot = resolve(__dirname, "..");

function readRepoFile(relativePath) {
  return readFileSync(resolve(repoRoot, relativePath), "utf8");
}

test("Fumadocs llms text routes execute through a DX-owned source-safe interpreter", () => {
  const fumadocs = readRepoFile("core/src/delivery/route_handler_fumadocs.rs");
  const serverContract = readRepoFile("core/src/delivery/server_contract.rs");

  assert.match(fumadocs, /pub\(super\) fn fumadocs_llms_route_handler_response/);
  assert.match(fumadocs, /createDxFumadocsLLMsIndex\(\)\.index\(\)/);
  assert.match(fumadocs, /pages\.join\("\\n\\n"\)/);
  assert.match(fumadocs, /"\/llms\.txt"/);
  assert.match(fumadocs, /"\/llms-full\.txt"/);
  assert.match(fumadocs, /"text\/plain; charset=utf-8"/);
  assert.match(fumadocs, /source-owned-fumadocs-llms-interpreter/);
  assert.match(fumadocs, /nodeModulesRequired: false/);
  assert.match(fumadocs, /networkCalls: false/);
  assert.match(fumadocs, /fn fumadocs_llms_index_route_returns_source_owned_text\(\)/);
  assert.match(fumadocs, /fn fumadocs_llms_full_route_returns_source_owned_text\(\)/);

  assert.match(serverContract, /fumadocs_llms_route_handler_response/);
  assert.match(
    serverContract,
    /fumadocs_llms_route_handler_response\(&source\.source, body, &request\)/,
  );
  assert.doesNotMatch(fumadocs, /Turbopack powers|full Next\.js parity/);
});
