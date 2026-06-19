const path = require("node:path");

const { INSTALLED_BINARY_SMOKE_RECEIPT } = require("./constants.ts");
const { httpProbeStep } = require("./proof-http-probe.ts");
const {
  publicStep,
  runProofSteps: runProofStepsWithBundle,
  summarizeSteps,
} = require("./proof-runner.ts");

function createProofBundle(mode, options = {}) {
  const steps = proofSteps(options);
  return {
    schema: "dx.build.readinessGate.proofBundle",
    mode,
    command: readinessGateCommand(options),
    policy: "Windows-friendly low-concurrency readiness proof; probes existing local servers only.",
    steps: steps.map(publicStep),
    summary: summarizeSteps(steps, mode),
  };
}

function runProofBundle(repoRoot, options = {}) {
  return runProofSteps(repoRoot, proofSteps(options), options);
}

function runProofSteps(repoRoot, steps, options = {}) {
  return runProofStepsWithBundle(repoRoot, steps, () => createProofBundle("run", options));
}

function proofSteps(options = {}) {
  return [
    commandStep({
      id: "cargo-check-dx-www-cli",
      label: "Cargo check dx-www CLI",
      command: "cargo check -p dx-www --no-default-features --features cli --bin dx-www -j 1",
      executable: "cargo",
      args: ["check", "-p", "dx-www", "--no-default-features", "--features", "cli", "--bin", "dx-www", "-j", "1"],
      maxConcurrency: 1,
      timeoutMs: 180000,
    }),
    commandStep({
      id: "focused-readiness-node-test",
      label: "Focused release readiness Node test",
      command: "node --test benchmarks/dx-build-readiness-gate.test.ts",
      executable: process.execPath,
      args: ["--test", "benchmarks/dx-build-readiness-gate.test.ts"],
      timeoutMs: 120000,
    }),
    installedSmokeStep(options),
    httpProbeStep("safe-http-root-probe", "HTTP root route proof", "http://127.0.0.1:3000/", null),
    httpProbeStep(
      "safe-http-dashboard-probe",
      "HTTP dashboard route proof",
      "http://127.0.0.1:3000/dashboard",
      null,
    ),
    httpProbeStep(
      "safe-http-login-probe",
      "HTTP login route proof",
      "http://127.0.0.1:3000/login",
      null,
    ),
    httpProbeStep(
      "safe-http-favicon-probe",
      "HTTP favicon proof",
      "http://127.0.0.1:3000/favicon.svg",
      null,
      "image/svg+xml",
    ),
    httpProbeStep(
      "safe-http-hot-reload-probe",
      "HTTP hot reload route proof",
      "http://127.0.0.1:3000/_dx/hot-reload/version?resource=route%3A%2F",
      "dx.hot-reload.poll",
    ),
  ];
}

function commandStep(step) {
  return {
    kind: "command",
    writesReceipts: false,
    ...step,
  };
}

function installedSmokeStep(options) {
  const selectedProject = selectedProjectRoot(options);
  const receipt = selectedProject
    ? path.join(selectedProject, INSTALLED_BINARY_SMOKE_RECEIPT)
    : INSTALLED_BINARY_SMOKE_RECEIPT;
  const args = ["tools/build/dx-build-installed-smoke.ts", "--json", "--require-product"];
  if (selectedProject) {
    args.push("--project", selectedProject);
  }
  args.push("--receipt", receipt);
  return commandStep({
    id: "dx-build-installed-smoke",
    label: "DX build installed smoke",
    command: commandLine(["node", ...args]),
    executable: process.execPath,
    args,
    timeoutMs: 180000,
    writesReceipts: true,
  });
}

function readinessGateCommand(options) {
  const args = ["node", "tools/build/dx-build-readiness-gate.ts"];
  const selectedProject = selectedProjectRoot(options);
  if (selectedProject) {
    args.push("--project", selectedProject);
  }
  args.push("--proof-bundle", "--json");
  return commandLine(args);
}

function selectedProjectRoot(options = {}) {
  if (!options.projectRoot || !options.repoRoot) {
    return null;
  }
  return path.resolve(options.projectRoot) === path.resolve(options.repoRoot)
    ? null
    : options.projectRoot;
}

function commandLine(args) {
  return args.map(commandArg).join(" ");
}

function commandArg(value) {
  const text = String(value);
  return /\s/.test(text) ? `'${text.replace(/'/g, "''")}'` : text;
}

module.exports = {
  createProofBundle,
  runProofBundle,
  runProofSteps,
};
