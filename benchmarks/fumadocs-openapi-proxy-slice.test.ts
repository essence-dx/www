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

test("upstream fumadocs exposes real openapi proxy APIs", () => {
  const proxy = read(path.join(mirror, "packages", "openapi", "src", "server", "proxy.ts"));
  const server = read(path.join(mirror, "packages", "openapi", "src", "server", "index.tsx"));
  const serverDocs = read(
    path.join(
      mirror,
      "apps",
      "docs",
      "content",
      "docs",
      "(framework)",
      "integrations",
      "openapi",
      "server.mdx",
    ),
  );
  const fetcher = read(path.join(mirror, "packages", "openapi", "src", "playground", "fetcher.ts"));

  assert.match(proxy, /export interface CreateProxyOptions/);
  assert.match(proxy, /allowedOrigins\?: string\[\]/);
  assert.match(proxy, /filterRequest\?: \(request: Request\) => boolean/);
  assert.match(proxy, /export function createProxy/);
  assert.match(server, /createProxy: typeof createProxy/);
  assert.match(server, /proxyUrl\?: string/);
  assert.match(serverDocs, /openapi\.createProxy/);
  assert.match(serverDocs, /allowedOrigins: \['https:\/\/example\.com'\]/);
  assert.match(serverDocs, /proxyUrl: '\/api\/proxy'/);
  assert.match(fetcher, /proxyUrl/);
});

test("fumadocs slice materializes a safe openapi proxy route", () => {
  assert.match(forge, /js\/app\/api\/openapi\/proxy\/route\.ts/);
  assert.match(forge, /proxyRoute: "\/api\/openapi\/proxy"/);
  assert.match(forge, /allowedOriginsEnv: "DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS"/);
  assert.match(forge, /proxyUrl: dxFumadocsOpenAPIContract\.proxyRoute/);
  assert.match(forge, /readDxFumadocsOpenAPIAllowedOrigins/);
  assert.match(forge, /dxFumadocsOpenAPI\.createProxy/);
  assert.match(forge, /allowedOrigins,/);
  assert.match(forge, /filterRequest\(request\)/);
  assert.match(forge, /request\.url\.startsWith\("https:\/\/"\)/);
  assert.match(forge, /GET, HEAD, PUT, POST, PATCH, DELETE/);
});

test("fumadocs proxy metadata is discoverable and app-owned", () => {
  assert.match(forge, /openApiProxyRoute: "\/api\/openapi\/proxy"/);
  assert.match(forge, /openApiProxyRouteFile: "app\/api\/openapi\/proxy\/route\.ts"/);
  assert.match(forge, /openApiProxyAllowedOriginsEnv: "DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS"/);
  assert.match(packageCatalog, /OpenAPI proxy allowed origins/);
  assert.match(scorecard, /OpenAPI request proxy/);
  assert.match(security, /createProxy/);
  assert.match(trustPolicy, /allowed origins/);
  assert.match(cli, /openApiProxyRoute/);
});

test("launch template consumes the fumadocs proxy contract", () => {
  assert.match(docsStatus, /dxFumadocsOpenAPIContract/);
  assert.match(docsStatus, /data-dx-docs-openapi-proxy/);
  assert.match(docsStatus, /DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS/);
});
