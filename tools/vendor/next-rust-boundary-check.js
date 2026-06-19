const fs = require("node:fs");
const path = require("node:path");

require.extensions[".ts"] ??= require.extensions[".js"];

const {
  CANONICAL_RECEIPT_PATH,
  CONSUMER_RECEIPT_PATH,
  EXPECTED_EXCLUDED_CORE_FOUNDATIONS,
  EXPECTED_IMPORTED_GROUPS,
  EXPECTED_PROTECTED_BOUNDARIES,
  FORBIDDEN_WORKSPACE_FOUNDATIONS,
  SCHEMAS,
} = require("./next-rust-boundary/constants.js");
const { buildAdapterBoundary } = require("./next-rust-boundary/adapter-boundary.js");
const { buildClaimPolicy } = require("./next-rust-boundary/claim-policy.js");
const {
  buildNextRustVendorBoundaryConsumerSnapshotReport,
} = require("./next-rust-boundary/consumer-snapshot.ts");
const { buildConsumerSurfacesReport } = require("./next-rust-boundary/consumer-surfaces.js");
const { runNextRustVendorBoundaryCli } = require("./next-rust-boundary/cli.js");
const {
  defaultConsumerReceiptPath,
  defaultReceiptPath,
  readText,
  receiptDisplayPath,
  relativeFromRepo,
  sha256,
  unique,
} = require("./next-rust-boundary/paths.js");
const { collectPublicDependencyClaims } = require("./next-rust-boundary/public-dependencies.js");
const { collectPublicClaimText } = require("./next-rust-boundary/public-claim-text.js");
const { collectPublicSourceExposure } = require("./next-rust-boundary/public-source-exposure.js");
const {
  comparableReceipt,
  findMismatches,
} = require("./next-rust-boundary/receipt-compare.js");
const {
  cargoGroupHasMetadata,
  collectCargoGroups,
  findMissingNeedles,
  parseReadmeImportedGroups,
  parseRustUpstreamGroups,
  readWorkspaceCargoText,
} = require("./next-rust-boundary/report-helpers.js");
const { buildNextRustVendorBoundaryStatusSurfaceReport } = require("./next-rust-boundary/status-surface.js");
const { collectVendoredCargoDependencyClaims } = require("./next-rust-boundary/vendored-cargo-dependencies.js");
const { collectVendorSourceInclusion } = require("./next-rust-boundary/vendor-source-inclusion.js");

function buildNextRustVendorBoundaryReport(repoRoot = path.resolve(__dirname, "..", "..")) {
  const absoluteRepoRoot = path.resolve(repoRoot);
  const vendorRoot = path.join(absoluteRepoRoot, "vendor", "next-rust");
  const readmePath = path.join(vendorRoot, "README.md");
  const licensePath = path.join(vendorRoot, "license.nextjs.md");
  const rustMetadataPath = path.join(absoluteRepoRoot, "dx-www", "src", "next_rust.rs");
  const workspaceCargoText = readWorkspaceCargoText(absoluteRepoRoot);

  const readme = readText(readmePath);
  const rustSource = readText(rustMetadataPath);
  const license = readText(licensePath);
  const filesystemGroups = collectCargoGroups(vendorRoot);
  const readmeGroups = parseReadmeImportedGroups(readme);
  const rustMetadataGroups = parseRustUpstreamGroups(rustSource);
  const documentedGroups = unique([...readmeGroups, ...rustMetadataGroups]);
  const forbiddenWorkspaceMentions = FORBIDDEN_WORKSPACE_FOUNDATIONS.filter((foundation) =>
    workspaceCargoText.includes(foundation),
  );

  const missingExpectedFilesystem = EXPECTED_IMPORTED_GROUPS.filter(
    (group) => !filesystemGroups.includes(group),
  );
  const missingExpectedReadme = EXPECTED_IMPORTED_GROUPS.filter((group) => !readmeGroups.includes(group));
  const missingExpectedRustMetadata = EXPECTED_IMPORTED_GROUPS.filter(
    (group) => !rustMetadataGroups.includes(group),
  );
  const unexpectedFilesystem = filesystemGroups.filter((group) => !EXPECTED_IMPORTED_GROUPS.includes(group));
  const unexpectedReadme = readmeGroups.filter((group) => !EXPECTED_IMPORTED_GROUPS.includes(group));
  const unexpectedRustMetadata = rustMetadataGroups.filter(
    (group) => !EXPECTED_IMPORTED_GROUPS.includes(group),
  );
  const missingFromReadme = filesystemGroups.filter((group) => !readmeGroups.includes(group));
  const missingFromRustMetadata = filesystemGroups.filter((group) => !rustMetadataGroups.includes(group));
  const missingCargoMetadata = documentedGroups.filter((group) => !cargoGroupHasMetadata(absoluteRepoRoot, group));
  const protectedMissingFromReadme = findMissingNeedles(readme, EXPECTED_PROTECTED_BOUNDARIES);
  const protectedMissingFromRustMetadata = findMissingNeedles(rustSource, EXPECTED_PROTECTED_BOUNDARIES);
  const excludedMissingFromReadme = findMissingNeedles(readme, EXPECTED_EXCLUDED_CORE_FOUNDATIONS);
  const excludedMissingFromRustMetadata = findMissingNeedles(rustSource, EXPECTED_EXCLUDED_CORE_FOUNDATIONS);
  const publicDependencyClaims = collectPublicDependencyClaims(absoluteRepoRoot);
  const publicClaimText = collectPublicClaimText(absoluteRepoRoot);
  const vendoredCargoDependencyClaims = collectVendoredCargoDependencyClaims(absoluteRepoRoot);
  const vendorSourceInclusion = collectVendorSourceInclusion(absoluteRepoRoot);
  const publicSourceExposure = collectPublicSourceExposure(absoluteRepoRoot);
  const runtimeTakeover = {
    reactRscRequired: /\breact\b|react-server-dom|React\/RSC/.test(workspaceCargoText),
    nodeNapiRequired: /\bnapi\b|node-api|Node\/NAPI|turbopack-nodejs/.test(workspaceCargoText),
    nodeModulesDefault: workspaceCargoText.includes("node_modules"),
  };

  const failures = [
    ...missingExpectedFilesystem,
    ...missingExpectedReadme,
    ...missingExpectedRustMetadata,
    ...unexpectedFilesystem,
    ...unexpectedReadme,
    ...unexpectedRustMetadata,
    ...missingFromReadme,
    ...missingFromRustMetadata,
    ...missingCargoMetadata,
    ...protectedMissingFromReadme,
    ...protectedMissingFromRustMetadata,
    ...excludedMissingFromReadme,
    ...excludedMissingFromRustMetadata,
    ...forbiddenWorkspaceMentions,
    ...publicDependencyClaims.forbiddenDependencies.map(
      (dependency) => `${dependency.manifest}:${dependency.scope}:${dependency.dependency}`,
    ),
    ...publicClaimText.forbiddenClaims.map((claim) => `${claim.file}:${claim.line}:${claim.claim}`),
    ...vendoredCargoDependencyClaims.forbiddenDependencies.map(
      (dependency) => `${dependency.manifest}:${dependency.dependency}`,
    ),
    ...vendorSourceInclusion.forbiddenInclusions.map(
      (inclusion) => `${inclusion.file}:${inclusion.pattern}:${inclusion.target}`,
    ),
    ...publicSourceExposure.forbiddenExposures.map(
      (exposure) => `${exposure.file}:${exposure.exposure}`,
    ),
  ];
  if (runtimeTakeover.reactRscRequired) failures.push("React/RSC required by workspace");
  if (runtimeTakeover.nodeNapiRequired) failures.push("Node/NAPI required by workspace");
  if (runtimeTakeover.nodeModulesDefault) failures.push("node_modules default in workspace");

  return {
    schema: SCHEMAS.vendorBoundary,
    status: failures.length === 0 ? "ok" : "fail",
    vendorRoot: "vendor/next-rust",
    upstream: {
      repository: "vercel/next.js",
      commit: "f3f56ecec2f3f8cefa0f0a1323ea406740251d5c",
      branch: "canary",
      importedOn: "2026-05-23",
    },
    license: {
      file: relativeFromRepo(absoluteRepoRoot, licensePath),
      sha256: sha256(license),
      mitNoticePresent: license.includes("The MIT License"),
    },
    importedGroups: {
      expected: EXPECTED_IMPORTED_GROUPS,
      filesystem: filesystemGroups,
      readme: readmeGroups,
      rustMetadata: rustMetadataGroups,
      missingExpectedFilesystem,
      missingExpectedReadme,
      missingExpectedRustMetadata,
      unexpectedFilesystem,
      unexpectedReadme,
      unexpectedRustMetadata,
      missingFromReadme,
      missingFromRustMetadata,
      missingCargoMetadata,
    },
    protectedBoundaries: {
      expected: EXPECTED_PROTECTED_BOUNDARIES,
      missingFromReadme: protectedMissingFromReadme,
      missingFromRustMetadata: protectedMissingFromRustMetadata,
    },
    excludedCoreFoundations: {
      expected: EXPECTED_EXCLUDED_CORE_FOUNDATIONS,
      missingFromReadme: excludedMissingFromReadme,
      missingFromRustMetadata: excludedMissingFromRustMetadata,
    },
    workspaceQuarantine: {
      checkedFiles: ["Cargo.toml", "dx-www/Cargo.toml"],
      vendorInWorkspace: workspaceCargoText.includes("vendor/next-rust"),
      forbiddenWorkspaceMentions,
    },
    publicDependencyClaims,
    publicClaimText,
    vendoredCargoDependencyClaims,
    vendorSourceInclusion,
    publicSourceExposure,
    runtimeTakeover,
  };
}

function buildNextRustVendorBoundaryConsumerSnapshot(
  repoRoot = path.resolve(__dirname, "..", ".."),
  receiptPath = defaultReceiptPath(path.resolve(repoRoot)),
) {
  const absoluteRepoRoot = path.resolve(repoRoot);
  const absoluteReceiptPath = path.resolve(receiptPath);
  const report = buildNextRustVendorBoundaryReport(absoluteRepoRoot);
  const receiptCheck = verifyNextRustVendorBoundaryReceipt(absoluteRepoRoot, absoluteReceiptPath);

  return buildNextRustVendorBoundaryConsumerSnapshotReport(
    absoluteRepoRoot,
    absoluteReceiptPath,
    report,
    receiptCheck,
  );
}

function buildConsumerReceiptReport(
  repoRoot,
  consumerReceiptPath = defaultConsumerReceiptPath(path.resolve(repoRoot)),
  sourceReceiptPath = defaultReceiptPath(path.resolve(repoRoot)),
) {
  const absoluteRepoRoot = path.resolve(repoRoot);
  const absoluteConsumerReceiptPath = path.resolve(consumerReceiptPath);
  const snapshot = buildNextRustVendorBoundaryConsumerSnapshot(absoluteRepoRoot, sourceReceiptPath);

  return {
    schema: SCHEMAS.consumerReceipt,
    status: snapshot.status,
    receipt: {
      kind: "dx.nextRust.vendorBoundary.consumer",
      path: receiptDisplayPath(absoluteRepoRoot, absoluteConsumerReceiptPath),
      generatedAt: new Date().toISOString(),
      producer: "tools/vendor/next-rust-boundary-check.js",
    },
    snapshot,
  };
}

function buildNextRustVendorBoundaryStatusSurface(
  repoRoot = path.resolve(__dirname, "..", ".."),
  consumerReceiptPath = defaultConsumerReceiptPath(path.resolve(repoRoot)),
  sourceReceiptPath = defaultReceiptPath(path.resolve(repoRoot)),
) {
  const absoluteRepoRoot = path.resolve(repoRoot);
  const absoluteConsumerReceiptPath = path.resolve(consumerReceiptPath);
  const absoluteSourceReceiptPath = path.resolve(sourceReceiptPath);
  const snapshot = buildNextRustVendorBoundaryConsumerSnapshot(absoluteRepoRoot, absoluteSourceReceiptPath);
  const sourceReceiptCheck = verifyNextRustVendorBoundaryReceipt(absoluteRepoRoot, absoluteSourceReceiptPath);
  const consumerReceiptCheck = verifyNextRustVendorBoundaryConsumerReceipt(absoluteRepoRoot, absoluteConsumerReceiptPath, absoluteSourceReceiptPath);

  return buildNextRustVendorBoundaryStatusSurfaceReport(absoluteRepoRoot, snapshot, sourceReceiptCheck, consumerReceiptCheck);
}

function buildNextRustVendorBoundaryActiveScope(
  repoRoot = path.resolve(__dirname, "..", ".."),
  consumerReceiptPath = defaultConsumerReceiptPath(path.resolve(repoRoot)),
  sourceReceiptPath = defaultReceiptPath(path.resolve(repoRoot)),
) {
  return buildNextRustVendorBoundaryStatusSurface(
    repoRoot,
    consumerReceiptPath,
    sourceReceiptPath,
  ).activeScope;
}

function buildNextRustVendorBoundaryConsumerSurfaces(repoRoot = path.resolve(__dirname, "..", ".."), consumerReceiptPath = defaultConsumerReceiptPath(path.resolve(repoRoot)), sourceReceiptPath = defaultReceiptPath(path.resolve(repoRoot))) {
  return buildConsumerSurfacesReport(buildNextRustVendorBoundaryStatusSurface(repoRoot, consumerReceiptPath, sourceReceiptPath));
}

function buildReceiptReport(repoRoot) {
  const report = buildNextRustVendorBoundaryReport(repoRoot);
  return {
    ...report,
    receipt: {
      kind: "dx.nextRust.vendorBoundary",
      path: CANONICAL_RECEIPT_PATH,
      generatedAt: new Date().toISOString(),
      producer: "tools/vendor/next-rust-boundary-check.js",
    },
  };
}

function writeNextRustVendorBoundaryReceipt(
  repoRoot = path.resolve(__dirname, "..", ".."),
  receiptPath = defaultReceiptPath(path.resolve(repoRoot)),
) {
  const absoluteRepoRoot = path.resolve(repoRoot);
  const absoluteReceiptPath = path.resolve(receiptPath);
  const report = buildReceiptReport(absoluteRepoRoot);

  fs.mkdirSync(path.dirname(absoluteReceiptPath), { recursive: true });
  fs.writeFileSync(absoluteReceiptPath, `${JSON.stringify(report, null, 2)}\n`, "utf8");

  return {
    receiptPath: absoluteReceiptPath,
    report,
  };
}

function writeNextRustVendorBoundaryConsumerReceipt(
  repoRoot = path.resolve(__dirname, "..", ".."),
  consumerReceiptPath = defaultConsumerReceiptPath(path.resolve(repoRoot)),
  sourceReceiptPath = defaultReceiptPath(path.resolve(repoRoot)),
) {
  const absoluteRepoRoot = path.resolve(repoRoot);
  const absoluteConsumerReceiptPath = path.resolve(consumerReceiptPath);
  const report = buildConsumerReceiptReport(
    absoluteRepoRoot,
    absoluteConsumerReceiptPath,
    sourceReceiptPath,
  );

  fs.mkdirSync(path.dirname(absoluteConsumerReceiptPath), { recursive: true });
  fs.writeFileSync(absoluteConsumerReceiptPath, `${JSON.stringify(report, null, 2)}\n`, "utf8");

  return {
    receiptPath: absoluteConsumerReceiptPath,
    report,
  };
}

function verifyNextRustVendorBoundaryReceipt(
  repoRoot = path.resolve(__dirname, "..", ".."),
  receiptPath = defaultReceiptPath(path.resolve(repoRoot)),
) {
  const absoluteRepoRoot = path.resolve(repoRoot);
  const absoluteReceiptPath = path.resolve(receiptPath);
  if (!fs.existsSync(absoluteReceiptPath)) {
    return {
      status: "missing",
      stale: true,
      receiptPath: absoluteReceiptPath,
      mismatches: ["receipt"],
    };
  }

  const current = comparableReceipt(buildReceiptReport(absoluteRepoRoot));
  const written = comparableReceipt(JSON.parse(readText(absoluteReceiptPath)));
  const mismatches = findMismatches(written, current);

  return {
    status: mismatches.length === 0 ? "ok" : "stale",
    stale: mismatches.length !== 0,
    receiptPath: absoluteReceiptPath,
    mismatches,
  };
}

function verifyNextRustVendorBoundaryConsumerReceipt(
  repoRoot = path.resolve(__dirname, "..", ".."),
  consumerReceiptPath = defaultConsumerReceiptPath(path.resolve(repoRoot)),
  sourceReceiptPath = defaultReceiptPath(path.resolve(repoRoot)),
) {
  const absoluteRepoRoot = path.resolve(repoRoot);
  const absoluteConsumerReceiptPath = path.resolve(consumerReceiptPath);
  if (!fs.existsSync(absoluteConsumerReceiptPath)) {
    return {
      status: "missing",
      stale: true,
      receiptPath: absoluteConsumerReceiptPath,
      mismatches: ["receipt"],
    };
  }

  const current = comparableReceipt(
    buildConsumerReceiptReport(absoluteRepoRoot, absoluteConsumerReceiptPath, sourceReceiptPath),
  );
  const written = comparableReceipt(JSON.parse(readText(absoluteConsumerReceiptPath)));
  const mismatches = findMismatches(written, current);

  return {
    status: mismatches.length === 0 ? "ok" : "stale",
    stale: mismatches.length !== 0,
    receiptPath: absoluteConsumerReceiptPath,
    sourceReceiptPath: path.resolve(sourceReceiptPath),
    mismatches,
  };
}

function main(argv = process.argv.slice(2)) {
  runNextRustVendorBoundaryCli(argv, module.exports);
}

module.exports = {
  buildAdapterBoundary,
  buildClaimPolicy,
  buildNextRustVendorBoundaryActiveScope,
  buildNextRustVendorBoundaryConsumerSnapshot,
  buildNextRustVendorBoundaryConsumerSurfaces,
  buildNextRustVendorBoundaryReport,
  buildNextRustVendorBoundaryStatusSurface,
  collectCargoGroups,
  CONSUMER_RECEIPT_PATH,
  defaultConsumerReceiptPath,
  defaultReceiptPath,
  EXPECTED_IMPORTED_GROUPS,
  parseReadmeImportedGroups,
  parseRustUpstreamGroups,
  verifyNextRustVendorBoundaryConsumerReceipt,
  verifyNextRustVendorBoundaryReceipt,
  writeNextRustVendorBoundaryConsumerReceipt,
  writeNextRustVendorBoundaryReceipt,
};

if (require.main === module) {
  main();
}
