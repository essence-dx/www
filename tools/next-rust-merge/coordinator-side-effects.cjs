const READ_ONLY_SIDE_EFFECTS = sideEffects({
  workspaceWrites: false,
  tempFixtureWrites: false,
  writesReceipts: false,
  receiptPaths: [],
  note: "source guard is read-only",
});

function readOnlySideEffects(note = "source guard is read-only") {
  return sideEffects({
    workspaceWrites: false,
    tempFixtureWrites: false,
    writesReceipts: false,
    receiptPaths: [],
    note,
  });
}

function tempReceiptSideEffects({ receiptPaths, note }) {
  return sideEffects({
    workspaceWrites: false,
    tempFixtureWrites: true,
    writesReceipts: true,
    receiptPaths,
    note,
  });
}

function sideEffects({
  workspaceWrites,
  tempFixtureWrites,
  writesReceipts,
  receiptPaths,
  note,
}) {
  return Object.freeze({
    workspaceWrites,
    tempFixtureWrites,
    writesReceipts,
    receiptPaths: Object.freeze([...receiptPaths]),
    note,
  });
}

function coordinatorSideEffectSummary(checks) {
  const workspaceWriteChecks = checks.filter((entry) => entry.sideEffects.workspaceWrites);
  const tempFixtureWriteChecks = checks.filter((entry) => entry.sideEffects.tempFixtureWrites);
  const receiptWritingChecks = checks.filter((entry) => entry.sideEffects.writesReceipts);

  return {
    workspaceWriteCheckCount: workspaceWriteChecks.length,
    tempFixtureWriteCheckCount: tempFixtureWriteChecks.length,
    readOnlyCheckCount: checks.length - receiptWritingChecks.length,
    receiptWritingCheckIds: receiptWritingChecks.map((entry) => entry.id),
  };
}

module.exports = {
  READ_ONLY_SIDE_EFFECTS,
  coordinatorSideEffectSummary,
  readOnlySideEffects,
  tempReceiptSideEffects,
};
