const assert = require("node:assert/strict");
const path = require("node:path");
const test = require("node:test");

const {
  dxWwwCargoRunArgs,
  resolveDxWwwBinaryPath,
  resolveDxWwwCargoManifestPath,
} = require("./dx-www-cli-paths.ts");

const root = path.resolve(__dirname, "..");

test("DX-WWW benchmark helpers resolve the flattened workspace CLI paths", () => {
  assert.equal(resolveDxWwwCargoManifestPath(root), path.join(root, "Cargo.toml"));
  assert.equal(
    resolveDxWwwBinaryPath(root, "win32"),
    path.join(root, "target", "debug", "dx-www.exe"),
  );
  assert.deepEqual(dxWwwCargoRunArgs(root, ["forge", "adoption-smoke"]).slice(0, 4), [
    "run",
    "--manifest-path",
    path.join(root, "Cargo.toml"),
    "-p",
  ]);
  assert.deepEqual(dxWwwCargoRunArgs(root, ["prove"], ["-q"]).slice(0, 4), [
    "run",
    "-q",
    "--manifest-path",
    path.join(root, "Cargo.toml"),
  ]);
});
