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
const scorecard = read(path.join(root, "core", "src", "ecosystem", "forge_scorecard.rs"));
const security = read(path.join(root, "core", "src", "ecosystem", "forge_security.rs"));
const trustPolicy = read(path.join(root, "core", "src", "ecosystem", "forge_trust_policy.rs"));

test("upstream fumadocs exposes real openapi code usage APIs", () => {
  const apiPageDocs = read(
    path.join(
      mirror,
      "apps",
      "docs",
      "content",
      "docs",
      "(framework)",
      "integrations",
      "openapi",
      "api-page.mdx",
    ),
  );
  const generators = read(
    path.join(mirror, "packages", "openapi", "src", "requests", "generators", "index.ts"),
  );
  const defaultGenerators = read(
    path.join(mirror, "packages", "openapi", "src", "requests", "generators", "all.ts"),
  );
  const clientConfig = read(
    path.join(mirror, "packages", "openapi", "src", "ui", "client", "index.tsx"),
  );

  assert.match(apiPageDocs, /createCodeUsageGeneratorRegistry/);
  assert.match(apiPageDocs, /registerDefault\(codeUsages\)/);
  assert.match(apiPageDocs, /defineClientConfig/);
  assert.match(apiPageDocs, /codeUsages,/);
  assert.match(generators, /export interface CodeUsageGeneratorRegistry/);
  assert.match(generators, /addInline/);
  assert.match(generators, /export type CodeUsageGeneratorFn/);
  assert.match(defaultGenerators, /registry\.add\('curl', curl\)/);
  assert.match(defaultGenerators, /registry\.add\('js', javascript\)/);
  assert.match(clientConfig, /codeUsages\?: CodeUsageGeneratorRegistry/);
  assert.match(clientConfig, /export function defineClientConfig/);
});

test("fumadocs slice materializes code usage registry and client config", () => {
  assert.match(forge, /js\/lib\/fumadocs\/openapi-code-usage\.ts/);
  assert.match(forge, /js\/components\/api-page\.client\.tsx/);
  assert.match(forge, /createCodeUsageGeneratorRegistry/);
  assert.match(forge, /registerDefault\(dxFumadocsOpenAPICodeUsages\)/);
  assert.match(forge, /dxFumadocsOpenAPICodeUsages\.add\("dx-launch-fetch"/);
  assert.match(forge, /defineClientConfig/);
  assert.match(forge, /client: dxFumadocsOpenAPIClientConfig/);
  assert.match(forge, /codeUsages: dxFumadocsOpenAPICodeUsages/);
});

test("fumadocs code usage metadata is discoverable and honest", () => {
  assert.match(forge, /openApiCodeUsageFile: "lib\/fumadocs\/openapi-code-usage\.ts"/);
  assert.match(forge, /openApiClientConfigFile: "components\/api-page\.client\.tsx"/);
  assert.match(packageCatalog, /request code sample policy/);
  assert.match(scorecard, /OpenAPI request code usage/);
  assert.match(security, /createCodeUsageGeneratorRegistry/);
  assert.match(trustPolicy, /request code sample policy/);
  assert.match(cli, /dxFumadocsOpenAPICodeUsageContract/);
});

test("launch template consumes the fumadocs code usage contract", () => {
  assert.match(docsStatus, /dxFumadocsOpenAPICodeUsageContract/);
  assert.match(docsStatus, /data-dx-docs-openapi-code-usage/);
});
