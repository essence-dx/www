const { summarizeCommand } = require("./command-summary.ts");

const HELP_COMMAND_LABEL = "dx www build --help";

function summarizeHelp(help, helpText, helpReadOnly) {
  return {
    exitCode: help.status,
    command: summarizeCommand(help),
    readOnly: helpReadOnly,
    sourceOwnedContractVisible:
      helpText.includes("source-owned build engine") &&
      helpText.includes("does not install node_modules"),
    androidTargetVisible:
      helpText.includes("--target <target>") &&
      helpText.includes("--target android"),
  };
}

function helpSummaryFailures(help) {
  const failures = [];
  pushIf(failures, help.exitCode !== 0, `${HELP_COMMAND_LABEL} exited non-zero`);
  pushIf(failures, !help.readOnly, `${HELP_COMMAND_LABEL} wrote project output`);
  pushIf(
    failures,
    !help.sourceOwnedContractVisible,
    `${HELP_COMMAND_LABEL} did not describe the source-owned no-node_modules contract`,
  );
  pushIf(
    failures,
    !help.androidTargetVisible,
    `${HELP_COMMAND_LABEL} did not describe the Android build target`,
  );
  return failures;
}

function pushIf(failures, condition, message) {
  if (condition) {
    failures.push(message);
  }
}

module.exports = {
  helpSummaryFailures,
  summarizeHelp,
};
