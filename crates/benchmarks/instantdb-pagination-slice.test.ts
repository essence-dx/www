const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes infinite pagination helpers from upstream API", () => {
  const upstreamReactCommon = read(
    path.join(
      mirror,
      "client",
      "packages",
      "react-common",
      "src",
      "InstantReactAbstractDatabase.tsx",
    ),
  );
  const upstreamCore = read(
    path.join(mirror, "client", "packages", "core", "src", "index.ts"),
  );
  const upstreamInfinite = read(
    path.join(
      mirror,
      "client",
      "sandbox",
      "react-nextjs",
      "pages",
      "play",
      "infinite-scroll.tsx",
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

  assert.match(upstreamReactCommon, /useInfiniteQuery = </);
  assert.match(upstreamReactCommon, /useInfiniteQuerySubscription/);
  assert.match(upstreamCore, /subscribeInfiniteQuery<Q extends ValidQuery<Q, Schema>>/);
  assert.match(upstreamInfinite, /db\.useInfiniteQuery/);
  assert.match(upstreamInfinite, /loadNextPage\(\)/);

  assert.match(slice, /"js\/instant\/pagination\.ts"/);
  assert.match(slice, /instantLaunchTodosPageQuery/);
  assert.match(slice, /useInstantLaunchTodosInfinite/);
  assert.match(slice, /db\.useInfiniteQuery\(instantLaunchTodosPageQuery\(pageSize\)\)/);
  assert.match(slice, /subscribeInstantLaunchTodosInfinite/);
  assert.match(slice, /db\.core\.subscribeInfiniteQuery/);
  assert.match(slice, /pagination: "useInstantLaunchTodosInfinite\(pageSize\)"/);

  assert.match(registry, /lib\/instant\/pagination\.ts/);
  assert.match(registry, /useInstantLaunchTodosInfinite/);
  assert.match(registry, /db\.core\.subscribeInfiniteQuery/);

  assert.match(launchProof, /useInstantLaunchTodosInfinite/);
  assert.match(launchProof, /data-dx-instant-pagination/);
  assert.match(launchProof, /pagination helpers wired/);
});
