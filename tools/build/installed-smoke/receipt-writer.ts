const fs = require("node:fs");
const path = require("node:path");

const { summarizeOutputProofSummary, summarizeProof } = require("./proof.ts");

function writeReportReceipt(report, receiptPath) {
  const baseSummary = {
    attempted: true,
    path: receiptPath,
    written: true,
    byteLength: 0,
    jsonParseable: true,
    matchesReport: true,
    error: null,
  };

  try {
    report.receiptWrite = baseSummary;
    refreshReportStatus(report);
    const json = serializeReportWithStableByteLength(report);
    fs.mkdirSync(path.dirname(receiptPath), { recursive: true });
    fs.writeFileSync(receiptPath, json);
    report.receiptWrite.byteLength = Buffer.byteLength(json);
    return report.receiptWrite;
  } catch (error) {
    report.receiptWrite = {
      attempted: true,
      path: receiptPath,
      written: false,
      byteLength: null,
      jsonParseable: false,
      matchesReport: false,
      error: summarizeWriteError(error),
    };
    report.failures.push(`installed smoke receipt could not be written: ${report.receiptWrite.error.message}`);
    refreshReportStatus(report);
    return report.receiptWrite;
  }
}

function serializeReportWithStableByteLength(report) {
  for (let attempt = 0; attempt < 5; attempt += 1) {
    const json = `${JSON.stringify(report, null, 2)}\n`;
    const byteLength = Buffer.byteLength(json);
    if (report.receiptWrite.byteLength === byteLength) {
      return json;
    }
    report.receiptWrite.byteLength = byteLength;
  }
  return `${JSON.stringify(report, null, 2)}\n`;
}

function refreshReportStatus(report) {
  report.passed = report.failures.length === 0;
  report.proof = summarizeProof(report);
  report.outputProofSummary = summarizeOutputProofSummary(report.proof);
  report.productProofPassed = report.passed && report.proof.productEligible === true;
}

function summarizeWriteError(error) {
  return {
    name: error && error.name ? error.name : "Error",
    code: error && typeof error.code === "string" ? error.code : null,
    message: error && error.message ? error.message : "unknown receipt write failure",
  };
}

module.exports = {
  writeReportReceipt,
};
