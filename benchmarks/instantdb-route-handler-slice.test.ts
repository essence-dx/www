const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes the real route handler API", () => {
  const upstreamCore = read(
    path.join(mirror, "client", "packages", "core", "src", "createRouteHandler.ts"),
  );
  const upstreamReact = read(
    path.join(mirror, "client", "packages", "react", "src", "index.ts"),
  );
  const upstreamNextExample = read(
    path.join(mirror, "client", "sandbox", "react-nextjs", "app", "api", "instant", "route.ts"),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"));
  const registry = read(path.join(root, "core", "src", "ecosystem", "forge_registry.rs"));
  const launchProof = read(path.join(root, "examples", "template", "instantdb-status.tsx"));

  assert.match(upstreamCore, /export const createInstantRouteHandler/);
  assert.match(upstreamCore, /case 'sync-user'/);
  assert.match(upstreamCore, /instant_user_\$\{config\.appId\}/);
  assert.match(upstreamReact, /createInstantRouteHandler,/);
  assert.match(upstreamNextExample, /createInstantRouteHandler/);

  assert.match(slice, /"js\/instant\/route\.ts"/);
  assert.match(slice, /"js\/app\/api\/instant\/route\.ts"/);
  assert.match(slice, /import \{ createInstantRouteHandler \} from "@instantdb\/react"/);
  assert.match(slice, /createDxInstantRouteHandlers/);
  assert.match(slice, /return createInstantRouteHandler\(\{ appId \}\)/);
  assert.match(slice, /export const \{ POST \} = createDxInstantRouteHandlers\(\);/);
  assert.match(slice, /routeHandler: "createDxInstantRouteHandlers\(\)\.POST\(request\)"/);
  assert.match(slice, /firstPartyRoute: "app\/api\/instant\/route\.ts"/);
  assert.match(slice, /DX-WWW-owned route tree/);
  assert.match(slice, /NEXT_PUBLIC_INSTANT_FIRST_PARTY_PATH=\/api\/instant/);

  assert.match(registry, /lib\/instant\/route\.ts/);
  assert.match(registry, /app\/api\/instant\/route\.ts/);
  assert.match(registry, /createDxInstantRouteHandlers/);
  assert.match(registry, /assert_eq!\(paths\.len\(\), 26\)/);

  assert.match(launchProof, /createDxInstantRouteHandlers/);
  assert.match(launchProof, /data-dx-instant-route/);
  assert.match(launchProof, /route POST handler wired/);
});
