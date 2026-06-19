const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes connection status and async local-id helpers", () => {
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
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"));
  const registry = read(path.join(root, "core", "src", "ecosystem", "forge_registry.rs"));
  const launchProof = read(path.join(root, "examples", "template", "instantdb-status.tsx"));

  assert.match(upstreamReactCommon, /getLocalId = \(name: string\): Promise<string>/);
  assert.match(upstreamReactCommon, /useConnectionStatus = \(\): ConnectionStatus/);
  assert.match(upstreamStatusBar, /db\.useConnectionStatus\(\)/);
  assert.match(upstreamStatusBar, /db\.getLocalId\("device"\)/);

  assert.match(slice, /"js\/instant\/status\.ts"/);
  assert.match(slice, /useInstantLaunchConnectionStatus/);
  assert.match(slice, /db\.useConnectionStatus\(\)/);
  assert.match(slice, /getInstantLaunchDeviceId/);
  assert.match(slice, /db\.getLocalId\("dx-launch-device"\)/);
  assert.match(slice, /connectionStatus: "useInstantLaunchConnectionStatus\(\)"/);
  assert.match(slice, /deviceId: "getInstantLaunchDeviceId\(\)"/);

  assert.match(registry, /lib\/instant\/status\.ts/);
  assert.match(registry, /useInstantLaunchConnectionStatus/);
  assert.match(registry, /db\.getLocalId/);

  assert.match(launchProof, /useInstantLaunchConnectionStatus/);
  assert.match(launchProof, /db\.useConnectionStatus\(\)/);
  assert.match(launchProof, /data-dx-instant-connection/);
  assert.match(launchProof, /data-dx-instant-status-helpers/);
});
