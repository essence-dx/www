const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes optional runtime endpoint config from upstream InstantConfig", () => {
  const upstreamCore = read(
    path.join(mirror, "client", "packages", "core", "src", "index.ts"),
  );
  const upstreamNextConfig = read(
    path.join(mirror, "client", "sandbox", "react-nextjs", "config.ts"),
  );
  const upstreamSsrDb = read(
    path.join(
      mirror,
      "client",
      "sandbox",
      "react-nextjs",
      "app",
      "play",
      "ssr",
      "db.ts",
    ),
  );
  const slice = read(
    path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"),
  );
  const registry = read(
    path.join(root, "core", "src", "ecosystem", "forge_registry.rs"),
  );
  const launchProof = read(
    path.join(root, "examples", "template", "instantdb-status.tsx"),
  );

  assert.match(upstreamCore, /apiURI\?: string/);
  assert.match(upstreamCore, /devtool\?: boolean \| DevtoolConfig/);
  assert.match(upstreamCore, /disableValidation\?: boolean/);
  assert.match(upstreamCore, /firstPartyPath\?: string/);
  assert.match(upstreamCore, /queryCacheLimit\?: number/);
  assert.match(upstreamCore, /websocketURI\?: string/);
  assert.match(upstreamCore, /verbose\?: boolean/);
  assert.match(upstreamNextConfig, /devtool: true/);
  assert.match(upstreamNextConfig, /apiURI:/);
  assert.match(upstreamNextConfig, /websocketURI:/);
  assert.match(upstreamSsrDb, /firstPartyPath: '\/api\/instant'/);
  assert.match(upstreamSsrDb, /apiURI: 'http:\/\/localhost:8888'/);
  assert.match(upstreamSsrDb, /websocketURI: 'ws:\/\/localhost:8888\/runtime\/session'/);

  assert.match(slice, /NEXT_PUBLIC_INSTANT_API_URI/);
  assert.match(slice, /NEXT_PUBLIC_INSTANT_DEVTOOL/);
  assert.match(slice, /NEXT_PUBLIC_INSTANT_DISABLE_VALIDATION/);
  assert.match(slice, /NEXT_PUBLIC_INSTANT_FIRST_PARTY_PATH/);
  assert.match(slice, /NEXT_PUBLIC_INSTANT_QUERY_CACHE_LIMIT/);
  assert.match(slice, /NEXT_PUBLIC_INSTANT_WEBSOCKET_URI/);
  assert.match(slice, /NEXT_PUBLIC_INSTANT_VERBOSE/);
  assert.match(slice, /optionalBooleanEnv/);
  assert.match(slice, /optionalNumberEnv/);
  assert.match(slice, /InstantConfig\.apiURI/);
  assert.match(slice, /InstantConfig\.devtool/);
  assert.match(slice, /InstantConfig\.disableValidation/);
  assert.match(slice, /InstantConfig\.firstPartyPath/);
  assert.match(slice, /InstantConfig\.queryCacheLimit/);
  assert.match(slice, /InstantConfig\.websocketURI/);
  assert.match(slice, /InstantConfig\.verbose/);
  assert.match(slice, /config: "readInstantConfig\(env\)"/);

  assert.match(registry, /env\.contains\("NEXT_PUBLIC_INSTANT_API_URI"\)/);
  assert.match(registry, /env\.contains\("NEXT_PUBLIC_INSTANT_DEVTOOL"\)/);
  assert.match(registry, /env\.contains\("NEXT_PUBLIC_INSTANT_DISABLE_VALIDATION"\)/);
  assert.match(registry, /env\.contains\("NEXT_PUBLIC_INSTANT_FIRST_PARTY_PATH"\)/);
  assert.match(registry, /env\.contains\("NEXT_PUBLIC_INSTANT_QUERY_CACHE_LIMIT"\)/);
  assert.match(registry, /env\.contains\("NEXT_PUBLIC_INSTANT_WEBSOCKET_URI"\)/);
  assert.match(registry, /env\.contains\("NEXT_PUBLIC_INSTANT_VERBOSE"\)/);

  assert.match(launchProof, /readInstantConfig/);
  assert.match(launchProof, /data-dx-instant-config/);
  assert.match(launchProof, /config env boundary wired/);
});
