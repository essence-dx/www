function printCommandDiagnostics(label, command) {
  if (!command || typeof command !== "object") {
    return;
  }
  process.stdout.write(`${label} command: ${formatCommand(command)}\n`);
  process.stdout.write(`${label} exit code: ${command.exitCode ?? "unknown"}\n`);
  printOutputTail(`${label} stdout tail`, command.stdoutTail, {
    truncated: command.stdoutTruncated,
    length: command.stdoutLength,
  });
  printOutputTail(`${label} stderr tail`, command.stderrTail, {
    truncated: command.stderrTruncated,
    length: command.stderrLength,
  });
}

function formatCommand(command) {
  return [command.command, ...(Array.isArray(command.args) ? command.args : [])]
    .filter(Boolean)
    .map(formatCommandPart)
    .join(" ");
}

function formatCommandPart(part) {
  const text = String(part);
  return /\s/.test(text) ? JSON.stringify(text) : text;
}

function printOutputTail(label, value, metadata = {}) {
  if (typeof value !== "string" || value.length === 0) {
    return;
  }
  const suffix = metadata.truncated === true && Number.isInteger(metadata.length)
    ? ` (truncated to last ${value.length} of ${metadata.length} chars)`
    : "";
  process.stdout.write(`${label}${suffix}:\n${value.trimEnd()}\n`);
}

module.exports = { printCommandDiagnostics };
