const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

function resolveDxWwwCargoManifestPath(rootDir) {
  const flattened = path.join(rootDir, "Cargo.toml");
  if (fs.existsSync(flattened)) {
    return flattened;
  }
  const legacyNested = path.join(rootDir, "www", "Cargo.toml");
  if (fs.existsSync(legacyNested)) {
    return legacyNested;
  }
  return flattened;
}

function resolveDxWwwBinaryPath(rootDir, platform = process.platform) {
  const binaryName = platform === "win32" ? "dx-www.exe" : "dx-www";
  const flattened = path.join(rootDir, "target", "debug", binaryName);
  if (fs.existsSync(flattened)) {
    return flattened;
  }
  const legacyNested = path.join(rootDir, "www", "target", "debug", binaryName);
  if (fs.existsSync(legacyNested)) {
    return legacyNested;
  }
  return flattened;
}

function dxWwwCargoRunArgs(rootDir, args, cargoArgs = []) {
  return [
    "run",
    ...cargoArgs,
    "--manifest-path",
    resolveDxWwwCargoManifestPath(rootDir),
    "-p",
    "dx-www",
    "--bin",
    "dx-www",
    "--",
    ...args,
  ];
}

function runDxWwwCli(rootDir, args, options = {}) {
  const spawn = options.spawnSync || spawnSync;
  const cwd = options.cwd || rootDir;
  const explicit = process.env.DX_FORGE_CLI;
  const exe = explicit || resolveDxWwwBinaryPath(rootDir);
  const commandOptions = {
    cwd,
    encoding: "utf8",
    timeout: options.timeout || 180000,
    windowsHide: true,
  };
  if (fs.existsSync(exe)) {
    return spawnResult(spawn(exe, args, commandOptions));
  }
  return spawnResult(spawn("cargo", dxWwwCargoRunArgs(rootDir, args), commandOptions));
}

function spawnResult(result) {
  return {
    status: typeof result.status === "number" ? result.status : 1,
    stdout: result.stdout || "",
    stderr: result.stderr || result.error?.message || "",
  };
}

module.exports = {
  dxWwwCargoRunArgs,
  resolveDxWwwBinaryPath,
  resolveDxWwwCargoManifestPath,
  runDxWwwCli,
};
