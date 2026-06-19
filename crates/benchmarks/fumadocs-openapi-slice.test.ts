const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "fumadocs");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

const forge = read(path.join(root, "core", "src", "ecosystem", "forge_fumadocs.rs"));
const docsStatus = read(path.join(root, "examples", "template", "docs-status.tsx"));
const packageCatalog = read(path.join(root, "examples", "template", "package-catalog.ts"));
const cli = read(path.join(root, "dx-www", "src", "cli", "mod.rs"));
const registry = read(path.join(root, "core", "src", "ecosystem", "forge_registry.rs"));
const scorecard = read(path.join(root, "core", "src", "ecosystem", "forge_scorecard.rs"));
const security = read(path.join(root, "core", "src", "ecosystem", "forge_security.rs"));
const trustPolicy = read(path.join(root, "core", "src", "ecosystem", "forge_trust_policy.rs"));

test("upstream fumadocs exposes the real openapi server and ui APIs", () => {
  const openapiPackage = read(path.join(mirror, "packages", "openapi", "package.json"));
  const openapiServer = read(path.join(mirror, "packages", "openapi", "src", "server", "index.tsx"));
  const openapiGenerate = read(path.join(mirror, "packages", "openapi", "src", "generate-file.ts"));
  const exampleSource = read(path.join(mirror, "examples", "openapi", "lib", "source.ts"));
  const exampleApiPage = read(path.join(mirror, "examples", "openapi", "components", "api-page.tsx"));

  assert.match(openapiPackage, /"name": "fumadocs-openapi"/);
  assert.match(openapiPackage, /"\.\/server": "\.\/dist\/server\/index\.js"/);
  assert.match(openapiPackage, /"\.\/ui": "\.\/dist\/ui\/index\.js"/);
  assert.match(openapiServer, /export function createOpenAPI/);
  assert.match(openapiServer, /staticSource/);
  assert.match(openapiServer, /loaderPlugin/);
  assert.match(openapiServer, /getAPIPageProps/);
  assert.match(openapiServer, /getSchema/);
  assert.match(openapiGenerate, /export async function generateFilesOnly/);
  assert.match(exampleSource, /openapi\.staticSource/);
  assert.match(exampleSource, /plugins: \[openapi\.loaderPlugin\(\)\]/);
  assert.match(exampleApiPage, /createAPIPage\(openapi\)/);
});

test("fumadocs slice materializes openapi virtual docs from real upstream APIs", () => {
  assert.match(forge, /js\/lib\/fumadocs\/openapi\.ts/);
  assert.match(forge, /js\/components\/api-page\.tsx/);
  assert.match(forge, /js\/openapi\/dx-launch\.yaml/);
  assert.match(forge, /createOpenAPI from fumadocs-openapi\/server/);
  assert.match(forge, /createAPIPage from fumadocs-openapi\/ui/);
  assert.match(forge, /dxFumadocsOpenAPI\.staticSource/);
  assert.match(forge, /plugins: \[dxFumadocsOpenAPI\.loaderPlugin\(\)\]/);
  assert.match(forge, /page\.type === "openapi"/);
  assert.match(forge, /page\.data\.getAPIPageProps\(\)/);
  assert.match(forge, /page\.data\.getSchema\(\)\.bundled/);
  assert.match(forge, /fumadocs-openapi\/css\/preset\.css/);
});

test("fumadocs openapi route contracts are discoverable and honest", () => {
  assert.match(forge, /openApiDocsRoute: "\/docs\/api"/);
  assert.match(forge, /openApiSchemaFile: "openapi\/dx-launch\.yaml"/);
  assert.match(forge, /openApiConfigFile: "lib\/fumadocs\/openapi\.ts"/);
  assert.match(registry, /fumadocs-openapi@10\.8\.6/);
  assert.match(packageCatalog, /OpenAPI schema governance/);
  assert.match(scorecard, /OpenAPI virtual docs/);
  assert.match(security, /createOpenAPI\(\)/);
  assert.match(trustPolicy, /OpenAPI proxy/);
  assert.match(cli, /dxFumadocsOpenAPIContract/);
});

test("launch template consumes the fumadocs openapi contract", () => {
  assert.match(docsStatus, /dxFumadocsOpenAPIContract/);
  assert.match(docsStatus, /dxFumadocsRouteContract\.openApiDocsRoute/);
  assert.match(docsStatus, /data-dx-docs-openapi-route/);
});
