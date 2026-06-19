const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes queryOnce and local-id helpers from upstream API", () => {
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
  const upstreamStatusBar = read(
    path.join(mirror, "examples", "solidjs-vite-advanced", "src", "components", "StatusBar.tsx"),
  );
  const upstreamTodoList = read(
    path.join(mirror, "examples", "solidjs-vite-advanced", "src", "components", "TodoList.tsx"),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"));
  const registry = read(path.join(root, "core", "src", "ecosystem", "forge_registry.rs"));
  const launchProof = read(path.join(root, "examples", "template", "instantdb-status.tsx"));

  assert.match(upstreamReactCommon, /useLocalId = \(name: string\)/);
  assert.match(upstreamReactCommon, /queryOnce = </);
  assert.match(upstreamStatusBar, /db\.useLocalId\("device"\)/);
  assert.match(upstreamTodoList, /db\.queryOnce\(\{ todos: \{\} \}\)/);

  assert.match(slice, /"js\/instant\/queries\.ts"/);
  assert.match(slice, /instantLaunchTodosQuery/);
  assert.match(slice, /queryInstantLaunchTodosSnapshot/);
  assert.match(slice, /db\.queryOnce\(instantLaunchTodosQuery\)/);
  assert.match(slice, /useInstantLaunchDeviceId/);
  assert.match(slice, /db\.useLocalId\("dx-launch-device"\)/);
  assert.match(slice, /queryOnce: "queryInstantLaunchTodosSnapshot\(\)"/);
  assert.match(slice, /localId: "useInstantLaunchDeviceId\(\)"/);

  assert.match(registry, /lib\/instant\/queries\.ts/);
  assert.match(registry, /queryInstantLaunchTodosSnapshot/);
  assert.match(registry, /db\.useLocalId/);

  assert.match(launchProof, /queryInstantLaunchTodosSnapshot/);
  assert.match(launchProof, /db\.useLocalId\("dx-launch-device"\)/);
  assert.match(launchProof, /data-dx-instant-local-id/);
  assert.match(launchProof, /data-dx-instant-query-once/);
});
