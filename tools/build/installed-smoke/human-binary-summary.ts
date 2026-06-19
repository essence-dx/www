function printBinarySummary(report) {
  if (report.binaryRole) {
    process.stdout.write(`Binary role: ${report.binaryRole}\n`);
  }
  if (report.binaryDefault) {
    process.stdout.write(`Default binary: ${report.binaryDefault}\n`);
  }
  const identity = report.binaryIdentity;
  if (identity?.path) {
    process.stdout.write(`Binary path: ${identity.path}\n`);
  }
  const status = formatBinaryStatus(identity);
  if (status) {
    process.stdout.write(`Binary status: ${status}\n`);
  }
  if (identity?.sha256) {
    process.stdout.write(`Binary sha256: ${identity.sha256}\n`);
  }
  const freshness = formatBinaryFreshness(report.binarySourceFreshness);
  if (freshness) {
    process.stdout.write(`Binary freshness: ${freshness}\n`);
  }
}

function formatBinaryStatus(identity) {
  if (!identity || typeof identity !== "object") {
    return null;
  }
  if (identity.present !== true) {
    return `missing (${identity.error || "unreadable"})`;
  }
  if (identity.kind === "file") {
    return `file (${formatByteLength(identity.byteLength)})`;
  }
  return identity.kind || "present";
}

function formatByteLength(byteLength) {
  return Number.isFinite(byteLength) ? `${byteLength} bytes` : "unknown size";
}

function formatBinaryFreshness(freshness) {
  if (!freshness || typeof freshness !== "object") {
    return null;
  }
  if (freshness.checked !== true) {
    return freshness.reason || "not checked";
  }
  if (freshness.fresh === true) {
    return "fresh";
  }
  if (freshness.fresh === false) {
    const source = freshness.newestSourcePath || "tracked source";
    return `stale (newer source: ${source})`;
  }
  return freshness.reason || "unknown";
}

module.exports = { printBinarySummary };
