const { READINESS_GATE_RECEIPT } = require("./constants.ts");

function reportConsumers() {
  return {
    dxCli: {
      command: "dx status",
      primaryReceipt: READINESS_GATE_RECEIPT,
    },
    dxWww: {
      primaryReceipt: READINESS_GATE_RECEIPT,
    },
    friday: {
      primaryReceipt: READINESS_GATE_RECEIPT,
      primaryField: "requiredActions",
    },
    zedPreview: {
      primaryReceipt: READINESS_GATE_RECEIPT,
    },
  };
}

function snapshotConsumers() {
  return {
    dxCli: {
      command: "dx status",
      primaryReceipt: READINESS_GATE_RECEIPT,
    },
    dxWww: {
      primaryReceipt: READINESS_GATE_RECEIPT,
      primaryField: "status",
    },
    friday: {
      primaryReceipt: READINESS_GATE_RECEIPT,
      primaryField: "requiredActions",
    },
    zedPreview: {
      primaryReceipt: READINESS_GATE_RECEIPT,
      primaryField: "blockers",
    },
  };
}

module.exports = {
  reportConsumers,
  snapshotConsumers,
};
