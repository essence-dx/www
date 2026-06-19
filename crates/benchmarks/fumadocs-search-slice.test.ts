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
const scorecard = read("core/src/ecosystem/forge_scorecard.rs");
const security = read("core/src/ecosystem/forge_security.rs");
const trustPolicy = read("core/src/ecosystem/forge_trust_policy.rs");
const cli = read("dx-www/src/cli/mod.rs");

test("fumadocs slice exposes the real upstream search route API", () => {
  assert.match(forge, /fumadocs-core\/search\/server/);
  assert.match(forge, /createFromSource/);
  assert.match(forge, /createDxFumadocsSearchApi/);
  assert.match(forge, /staticGET/);
  assert.match(forge, /useDocsSearch from fumadocs-core\/search\/client/);
  assert.match(forge, /js\/lib\/fumadocs\/search\.ts/);
  assert.match(forge, /js\/lib\/fumadocs\/search-client\.ts/);
  assert.match(forge, /js\/app\/api\/search\/route\.ts/);
  assert.match(forge, /js\/app\/api\/search-static\/route\.ts/);
  assert.match(forge, /searchRoute: "\/api\/search"/);
  assert.match(forge, /staticSearchRoute: "\/api\/search-static"/);
  assert.match(forge, /searchConfigFile: "lib\/fumadocs\/search\.ts"/);
  assert.match(forge, /searchClientFile: "lib\/fumadocs\/search-client\.ts"/);
  assert.match(forge, /searchRouteFile: "app\/api\/search\/route\.ts"/);
  assert.match(forge, /staticSearchRouteFile: "app\/api\/search-static\/route\.ts"/);
  assert.match(forge, /language: "english"/);
  assert.match(forge, /queryParam: "query"/);
  assert.match(forge, /optionalParams: \["locale", "tag", "limit", "mode"\]/);
  assert.match(forge, /dxFumadocsFetchSearchClient/);
  assert.match(forge, /dxFumadocsStaticSearchClient/);
});

test("fumadocs search stays honest about app-owned production boundaries", () => {
  assert.match(forge, /Search UI/);
  assert.match(forge, /multilingual\/vector policy/);
  assert.match(packageCatalog, /search UI/);
  assert.match(scorecard, /search route materialization/);
  assert.match(security, /createFromSource\(\)/);
  assert.match(trustPolicy, /search UI/);
  assert.doesNotMatch(packageCatalog, /hosted search setup/);
  assert.doesNotMatch(cli, /hosted search setup/);
  assert.doesNotMatch(cli, /dxFumadocsRouteContract\.route/);
});

test("launch template consumes the fumadocs search route contract", () => {
  assert.match(docsStatus, /dxFumadocsRouteContract\.docsRoute/);
  assert.match(docsStatus, /dxFumadocsRouteContract\.searchRoute/);
  assert.match(docsStatus, /dxFumadocsRouteContract\.staticSearchRoute/);
  assert.match(docsStatus, /dxFumadocsSearchClientContract/);
  assert.match(docsStatus, /data-dx-docs-search-route/);
  assert.match(docsStatus, /data-dx-docs-static-search-route/);
  assert.doesNotMatch(docsStatus, /dxFumadocsRouteContract\.route/);
});
