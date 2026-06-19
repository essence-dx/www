const path = require("node:path");

function parseArgs(args) {
  const options = {
    binary: null,
    runner: null,
    project: null,
    receipt: null,
    json: false,
    requireProduct: false,
    help: false,
  };

  for (let index = 0; index < args.length; ) {
    const arg = args[index];
    if (arg === "--binary") {
      options.binary = path.resolve(requireValue(args, index, arg));
      index += 2;
      continue;
    }
    if (arg === "--runner") {
      options.runner = path.resolve(requireValue(args, index, arg));
      index += 2;
      continue;
    }
    if (arg === "--project") {
      options.project = path.resolve(requireValue(args, index, arg));
      index += 2;
      continue;
    }
    if (arg === "--receipt") {
      options.receipt = path.resolve(requireValue(args, index, arg));
      index += 2;
      continue;
    }
    if (arg === "--json") {
      options.json = true;
      index += 1;
      continue;
    }
    if (arg === "--require-product") {
      options.requireProduct = true;
      index += 1;
      continue;
    }
    if (arg === "--help" || arg === "-h") {
      options.help = true;
      index += 1;
      continue;
    }
    throw new Error(`Unknown option: ${arg}`);
  }

  return options;
}

function requireValue(args, index, flag) {
  const value = args[index + 1];
  if (!value) {
    throw new Error(`${flag} requires a value`);
  }
  return value;
}

function printUsage(output = process.stdout) {
  output.write(
    [
      "Usage: node tools/build/dx-build-installed-smoke.ts [--binary <path>] [--runner <path>] [--project <path>] [--receipt <path>] [--json] [--require-product]",
      "",
      "Runs the installed dx-www build command against a tiny source-owned TSX/CSS/assets fixture.",
      "--require-product fails the command unless the smoke proves the installed default binary.",
      "Does not install packages and does not start a server.",
      "",
    ].join("\n"),
  );
}

module.exports = {
  parseArgs,
  printUsage,
};
