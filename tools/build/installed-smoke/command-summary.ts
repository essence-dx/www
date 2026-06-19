const COMMAND_OUTPUT_TAIL_MAX_LENGTH = 1200;

function summarizeCommand(result) {
  const stdout = summarizeCommandOutput(result.stdout);
  const stderr = summarizeCommandOutput(result.stderr);

  return {
    command: result.command || null,
    args: Array.isArray(result.args) ? result.args : [],
    dxArgs: Array.isArray(result.dxArgs) ? result.dxArgs : [],
    cwd: result.cwd || null,
    exitCode: result.status,
    stdoutTail: stdout.tail,
    stdoutLength: stdout.length,
    stdoutTruncated: stdout.truncated,
    stderrTail: stderr.tail,
    stderrLength: stderr.length,
    stderrTruncated: stderr.truncated,
    skipped: result.skipped === true,
    skipReason: result.skipReason || null,
  };
}

function outputTail(value) {
  return summarizeCommandOutput(value).tail;
}

function summarizeCommandOutput(value) {
  const text = typeof value === "string" ? value : "";
  return {
    tail: tail(text),
    length: text.length,
    truncated: text.length > COMMAND_OUTPUT_TAIL_MAX_LENGTH,
  };
}

function tail(text) {
  return text.length > COMMAND_OUTPUT_TAIL_MAX_LENGTH
    ? text.slice(text.length - COMMAND_OUTPUT_TAIL_MAX_LENGTH)
    : text;
}

module.exports = {
  COMMAND_OUTPUT_TAIL_MAX_LENGTH,
  outputTail,
  summarizeCommand,
};
