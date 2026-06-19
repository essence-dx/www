const { SCHEMAS } = require("./constants.js");

function buildAdapterBoundary() {
  return {
    schema: SCHEMAS.adapterBoundary,
    sourceOnlyReceipt: true,
    nativeDxCheckWired: false,
    nativeZedSurfaceWired: false,
    nativeStudioSurfaceWired: false,
    cargoWorkspaceWired: false,
    publicTurbopackApi: false,
    allowedConsumers: [
      "Forge/source receipts",
      "dx-check receipt adapters",
      "Zed/Studio status surfaces",
    ],
    requiredFreshnessChecks: [
      ".dx/receipts/next-rust/vendor-boundary.json",
      ".dx/receipts/next-rust/vendor-boundary-consumer.json",
    ],
    blockedUntilProven: [
      "native dx-check rendering",
      "native Zed surface rendering",
      "native Studio surface rendering",
      "Cargo workspace dependency wiring",
      "runtime use of vendored Next/Turbopack crates",
    ],
  };
}

module.exports = {
  buildAdapterBoundary,
};
