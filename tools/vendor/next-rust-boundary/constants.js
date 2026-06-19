const EXPECTED_IMPORTED_GROUPS = [
  "crates/next-code-frame",
  "crates/next-custom-transforms",
  "turbopack/crates/turbo-persistence",
  "turbopack/crates/turbo-tasks",
  "turbopack/crates/turbo-tasks-auto-hash-map",
  "turbopack/crates/turbo-tasks-backend",
  "turbopack/crates/turbo-tasks-bytes",
  "turbopack/crates/turbo-tasks-env",
  "turbopack/crates/turbo-tasks-fetch",
  "turbopack/crates/turbo-tasks-fs",
  "turbopack/crates/turbo-tasks-fuzz",
  "turbopack/crates/turbo-tasks-hash",
  "turbopack/crates/turbo-tasks-macros",
  "turbopack/crates/turbo-tasks-macros-tests",
  "turbopack/crates/turbo-tasks-malloc",
  "turbopack/crates/turbo-tasks-testing",
  "turbopack/crates/turbopack-core",
  "turbopack/crates/turbopack-css",
  "turbopack/crates/turbopack-dev-server",
  "turbopack/crates/turbopack-ecmascript",
  "turbopack/crates/turbopack-ecmascript-hmr-protocol",
  "turbopack/crates/turbopack-image",
  "turbopack/crates/turbopack-mdx",
  "turbopack/crates/turbopack-resolve",
];

const EXPECTED_PROTECTED_BOUNDARIES = [
  "browser-micro",
  "browser",
  "packet",
  "binary",
  "morph",
  "serializer",
  "dx-style / related-crates/style",
  "Rust server",
  "Forge/source receipts",
  "dx-check",
  "Zed surfaces",
  "Studio surfaces",
];

const EXPECTED_EXCLUDED_CORE_FOUNDATIONS = [
  "next-core",
  "next-napi-bindings",
  "turbopack-nodejs",
  "React/RSC",
  "Node/NAPI",
  "Turborepo",
  "node_modules",
  "Node resolver defaults",
];

const FORBIDDEN_WORKSPACE_FOUNDATIONS = [
  "vendor/next-rust",
  "next-core",
  "next-napi-bindings",
  "turbopack-nodejs",
  "turbo-tasks",
  "turbopack-core",
  "turbopack-css",
  "turbopack-ecmascript",
  "turbopack-resolve",
];

const CANONICAL_RECEIPT_PATH = ".dx/receipts/next-rust/vendor-boundary.json";
const CONSUMER_RECEIPT_PATH = ".dx/receipts/next-rust/vendor-boundary-consumer.json";

const SCHEMAS = {
  activeScope: "dx.nextRust.vendorBoundary.activeScope",
  activeScopeSummary: "dx.nextRust.vendorBoundary.activeScopeSummary",
  adapterBoundary: "dx.nextRust.vendorBoundary.adapterBoundary",
  claimPolicy: "dx.nextRust.vendorBoundary.claimPolicy",
  consumerReceipt: "dx.nextRust.vendorBoundary.consumerReceipt",
  consumerSurfaces: "dx.nextRust.vendorBoundary.consumerSurfaces",
  consumerSnapshot: "dx.nextRust.vendorBoundary.consumerSnapshot",
  dxCheckProjection: "dx.nextRust.vendorBoundary.dxCheckProjection",
  editorSurfaceProjection: "dx.nextRust.vendorBoundary.editorSurfaceProjection",
  statusSurface: "dx.nextRust.vendorBoundary.statusSurface",
  vendorBoundary: "dx.nextRust.vendorBoundary",
};

module.exports = {
  CANONICAL_RECEIPT_PATH,
  CONSUMER_RECEIPT_PATH,
  EXPECTED_EXCLUDED_CORE_FOUNDATIONS,
  EXPECTED_IMPORTED_GROUPS,
  EXPECTED_PROTECTED_BOUNDARIES,
  FORBIDDEN_WORKSPACE_FOUNDATIONS,
  SCHEMAS,
};
