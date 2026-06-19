const { spawnSync } = require("node:child_process");

function runDx(binary, runner, args, cwd) {
  const command = runner || binary;
  const commandArgs = runner ? [binary, ...args] : args;
  const result = spawnSync(command, commandArgs, {
    cwd,
    encoding: "utf8",
    windowsHide: true,
  });

  return {
    status: result.status ?? 1,
    stdout: result.stdout || "",
    stderr: result.stderr || result.error?.message || "",
    command,
    args: commandArgs,
    dxArgs: args,
    cwd,
  };
}

function skippedDx(binary, runner, args, cwd, reason) {
  const command = runner || binary;
  const commandArgs = runner ? [binary, ...args] : args;
  return {
    status: null,
    stdout: "",
    stderr: reason ? `skipped: ${reason}` : "skipped",
    command,
    args: commandArgs,
    dxArgs: args,
    cwd,
    skipped: true,
    skipReason: reason || "skipped",
  };
}

module.exports = {
  runDx,
  skippedDx,
};
