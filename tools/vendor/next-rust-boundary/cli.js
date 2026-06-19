const path = require("node:path");

function optionValue(argv, optionName, fallback) {
  const optionIndex = argv.indexOf(optionName);
  return optionIndex === -1 ? fallback : argv[optionIndex + 1];
}

function runNextRustVendorBoundaryCli(argv, commands) {
  const repoRoot = optionValue(argv, "--repo-root", path.resolve(__dirname, "..", "..", ".."));
  const resolvedRoot = path.resolve(repoRoot);
  const receiptPath = optionValue(argv, "--receipt", commands.defaultReceiptPath(resolvedRoot));
  const consumerReceiptPath = optionValue(
    argv,
    "--consumer-receipt",
    commands.defaultConsumerReceiptPath(resolvedRoot),
  );
  let report;

  if (argv.includes("--active-scope")) {
    report = commands.buildNextRustVendorBoundaryActiveScope(
      resolvedRoot,
      consumerReceiptPath,
      receiptPath,
    );
  } else if (argv.includes("--consumer-surfaces")) {
    report = commands.buildNextRustVendorBoundaryConsumerSurfaces(
      resolvedRoot,
      consumerReceiptPath,
      receiptPath,
    );
  } else if (argv.includes("--dx-check-surface")) {
    report = commands.buildNextRustVendorBoundaryStatusSurface(
      resolvedRoot,
      consumerReceiptPath,
      receiptPath,
    );
  } else if (argv.includes("--write-consumer-receipt")) {
    if (argv.includes("--write")) {
      commands.writeNextRustVendorBoundaryReceipt(resolvedRoot, receiptPath);
    }
    report = commands.writeNextRustVendorBoundaryConsumerReceipt(
      resolvedRoot,
      consumerReceiptPath,
      receiptPath,
    ).report;
  } else if (argv.includes("--check-consumer-receipt")) {
    report = commands.verifyNextRustVendorBoundaryConsumerReceipt(
      resolvedRoot,
      consumerReceiptPath,
      receiptPath,
    );
  } else if (argv.includes("--consumer-snapshot")) {
    if (argv.includes("--write")) {
      commands.writeNextRustVendorBoundaryReceipt(resolvedRoot, receiptPath);
    }
    report = commands.buildNextRustVendorBoundaryConsumerSnapshot(resolvedRoot, receiptPath);
  } else if (argv.includes("--check-receipt")) {
    report = commands.verifyNextRustVendorBoundaryReceipt(resolvedRoot, receiptPath);
  } else if (argv.includes("--write")) {
    report = commands.writeNextRustVendorBoundaryReceipt(resolvedRoot, receiptPath).report;
  } else {
    report = commands.buildNextRustVendorBoundaryReport(resolvedRoot);
  }

  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
  if (report.status !== "ok") {
    process.exitCode = 1;
  }
}

module.exports = {
  runNextRustVendorBoundaryCli,
};
