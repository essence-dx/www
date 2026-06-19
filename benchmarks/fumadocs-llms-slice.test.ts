const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

const forge = read("core/src/ecosystem/forge_fumadocs.rs");
const docsStatus = read("examples/template/docs-status.tsx");
const packageCatalog = read("examples/template/package-catalog.ts");
const cli = read("dx-www/src/cli/mod.rs");
const scorecard = read("core/src/ecosystem/forge_scorecard.rs");
const security = read("core/src/ecosystem/forge_security.rs");
const trustPolicy = read("core/src/ecosystem/forge_trust_policy.rs");

test("fumadocs slice exposes the real llms source API", () => {
  assert.match(forge, /llms from fumadocs-core\/source/);
  assert.match(forge, /includeProcessedMarkdown: true/);
  assert.match(forge, /page\.data\.getText\("processed"\)/);
  assert.match(forge, /getDxFumadocsLLMText/);
  assert.match(forge, /createDxFumadocsLLMsIndex/);
  assert.match(forge, /js\/lib\/fumadocs\/llms\.ts/);
  assert.match(forge, /js\/app\/llms\.txt\/route\.ts/);
  assert.match(forge, /js\/app\/llms-full\.txt\/route\.ts/);
  assert.match(forge, /js\/app\/llms\.mdx\/docs\/\[\[...slug\]\]\/route\.ts/);
});

test("fumadocs llms routes are discoverable and honest", () => {
  assert.match(forge, /llmsIndexRoute: "\/llms\.txt"/);
  assert.match(forge, /llmsFullRoute: "\/llms-full\.txt"/);
  assert.match(forge, /llmsPageMarkdownRoute: "\/llms\.mdx\/docs\/\[\[...slug\]\]"/);
  assert.match(packageCatalog, /AI indexing policy/);
  assert.match(scorecard, /LLMs route materialization/);
  assert.match(security, /llms\(\)/);
  assert.match(trustPolicy, /AI crawler exposure/);
  assert.match(cli, /dxFumadocsLLMsContract/);
});

test("launch template consumes the fumadocs llms contract", () => {
  assert.match(docsStatus, /dxFumadocsLLMsContract/);
  assert.match(docsStatus, /dxFumadocsRouteContract\.llmsIndexRoute/);
  assert.match(docsStatus, /dxFumadocsRouteContract\.llmsFullRoute/);
  assert.match(docsStatus, /data-dx-docs-llms-route/);
  assert.match(docsStatus, /data-dx-docs-llms-full-route/);
});
