import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { execFileSync } = require("node:child_process");
const test = require("node:test");
const {
  buildNextRustVendorBoundaryStatusSurface,
  buildNextRustVendorBoundaryConsumerSnapshot,
  buildNextRustVendorBoundaryConsumerSurfaces,
  buildNextRustVendorBoundaryReport,
  verifyNextRustVendorBoundaryConsumerReceipt,
  verifyNextRustVendorBoundaryReceipt,
  writeNextRustVendorBoundaryConsumerReceipt,
  writeNextRustVendorBoundaryReceipt,
} = require("../tools/vendor/next-rust-boundary-check.js");

const repoRoot = path.resolve(__dirname, "..");
const vendorRoot = path.join(repoRoot, "vendor", "next-rust");
const rustSource = fs.readFileSync(
  path.join(repoRoot, "dx-www", "src", "next_rust.rs"),
  "utf8",
);
const readme = fs.readFileSync(path.join(vendorRoot, "README.md"), "utf8");
const license = fs.readFileSync(path.join(vendorRoot, "license.nextjs.md"), "utf8");

const importedRustGroups = [
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

const protectedRuntimeCrates = [
  "dx-www-browser-micro",
  "dx-www-browser",
  "dx-www-packet",
  "dx-www-binary",
  "dx-www-morph",
  "dx-serializer",
  "dx-style",
  "related-crates/style",
  "dx-www-server",
];

const protectedSourceBoundaries = [
  "Forge/source receipts",
  "dx-check",
  "Zed surfaces",
  "Studio surfaces",
];

const excludedCoreFoundations = [
  "next-core",
  "next-napi-bindings",
  "turbopack-nodejs",
  "React/RSC",
  "Node/NAPI",
  "node_modules",
];

function writeText(filePath, text) {
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, text, "utf8");
}

function materializeVendorBoundaryFixture(extraCargoGroups = []) {
  const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-next-rust-boundary-"));
  writeText(path.join(tempRoot, "Cargo.toml"), "");
  writeText(path.join(tempRoot, "dx-www", "Cargo.toml"), "");
  writeText(path.join(tempRoot, "vendor", "next-rust", "README.md"), readme);
  writeText(path.join(tempRoot, "vendor", "next-rust", "license.nextjs.md"), license);
  writeText(path.join(tempRoot, "dx-www", "src", "next_rust.rs"), rustSource);

  for (const importedGroup of [...importedRustGroups, ...extraCargoGroups]) {
    writeText(
      path.join(tempRoot, "vendor", "next-rust", ...importedGroup.split("/"), "Cargo.toml"),
      "[package]\nname = \"fixture\"\nversion = \"0.0.0\"\n",
    );
  }

  return tempRoot;
}

function collectStringValues(value, values = []) {
  if (typeof value === "string") {
    values.push(value);
    return values;
  }
  if (Array.isArray(value)) {
    for (const item of value) {
      collectStringValues(item, values);
    }
    return values;
  }
  if (value && typeof value === "object") {
    for (const item of Object.values(value)) {
      collectStringValues(item, values);
    }
  }
  return values;
}

test("Next/Turbopack Rust vendor boundary lists exact imported Rust groups", () => {
  assert.ok(fs.existsSync(path.join(vendorRoot, "license.nextjs.md")));

  for (const importedGroup of importedRustGroups) {
    assert.ok(
      fs.existsSync(path.join(vendorRoot, importedGroup, "Cargo.toml")),
      `missing vendored Cargo metadata for ${importedGroup}`,
    );
    assert.match(
      rustSource,
      new RegExp(`upstream: "${importedGroup.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}"`),
      `Rust snapshot metadata must list exact group ${importedGroup}`,
    );
    assert.ok(
      readme.includes(`- \`${importedGroup}\``),
      `vendor README must list exact group ${importedGroup}`,
    );
  }
});

test("Next/Turbopack Rust vendor boundary protects DX-owned runtime and source surfaces", () => {
  for (const crateName of protectedRuntimeCrates) {
    assert.ok(
      rustSource.includes(`"${crateName}"`),
      `Rust snapshot metadata must protect ${crateName}`,
    );
    assert.ok(readme.includes(crateName), `vendor README must protect ${crateName}`);
  }

  for (const boundary of protectedSourceBoundaries) {
    assert.ok(
      rustSource.includes(boundary),
      `Rust snapshot metadata must protect ${boundary}`,
    );
    assert.ok(readme.includes(boundary), `vendor README must protect ${boundary}`);
  }
});

test("Next/Turbopack Rust vendor boundary records excluded foundations", () => {
  assert.ok(readme.includes("- License file: `license.nextjs.md`"));
  assert.ok(readme.includes("- Commit: `f3f56ecec2f3f8cefa0f0a1323ea406740251d5c`"));

  for (const excluded of excludedCoreFoundations) {
    assert.ok(
      rustSource.includes(excluded),
      `Rust snapshot metadata must keep ${excluded} out of the DX core foundation`,
    );
    assert.ok(
      readme.includes(excluded),
      `vendor README must keep ${excluded} out of the DX core foundation`,
    );
  }
});

test("Next/Turbopack Rust vendor boundary checker emits a clean quarantine report", () => {
  const report = buildNextRustVendorBoundaryReport(repoRoot);

  assert.equal(report.status, "ok");
  assert.equal(report.vendorRoot, "vendor/next-rust");
  assert.equal(report.license.file, "vendor/next-rust/license.nextjs.md");
  assert.equal(report.workspaceQuarantine.vendorInWorkspace, false);
  assert.deepEqual(report.workspaceQuarantine.forbiddenWorkspaceMentions, []);
  assert.deepEqual(report.importedGroups.missingFromReadme, []);
  assert.deepEqual(report.importedGroups.missingFromRustMetadata, []);
  assert.deepEqual(report.importedGroups.missingCargoMetadata, []);
  assert.deepEqual(report.importedGroups.expected, importedRustGroups);
  assert.deepEqual(report.importedGroups.missingExpectedFilesystem, []);
  assert.deepEqual(report.importedGroups.missingExpectedReadme, []);
  assert.deepEqual(report.importedGroups.missingExpectedRustMetadata, []);
  assert.deepEqual(report.importedGroups.unexpectedFilesystem, []);
  assert.deepEqual(report.importedGroups.unexpectedReadme, []);
  assert.deepEqual(report.importedGroups.unexpectedRustMetadata, []);
  assert.deepEqual(report.protectedBoundaries.missingFromReadme, []);
  assert.deepEqual(report.protectedBoundaries.missingFromRustMetadata, []);
  assert.deepEqual(report.excludedCoreFoundations.missingFromReadme, []);
  assert.deepEqual(report.excludedCoreFoundations.missingFromRustMetadata, []);
  assert.deepEqual(report.publicDependencyClaims.checkedManifests, ["Cargo.toml", "dx-www/Cargo.toml"]);
  assert.deepEqual(report.publicDependencyClaims.forbiddenDependencies, []);
  assert.deepEqual(report.publicClaimText.checkedFiles, [
    "vendor/next-rust/README.md",
    "docs/next-rust-merge-checkpoint.md",
    "DX.md",
    "TODO.md",
    "CHANGELOG.md",
  ]);
  assert.deepEqual(report.publicClaimText.forbiddenClaims, []);
  assert.deepEqual(
    report.vendoredCargoDependencyClaims.checkedManifests,
    importedRustGroups.map((group) => `vendor/next-rust/${group}/Cargo.toml`),
  );
  assert.deepEqual(report.vendoredCargoDependencyClaims.forbiddenDependencies, []);
  assert.deepEqual(report.vendorSourceInclusion.checkedRoots, ["core/src", "dx-www/src"]);
  assert.deepEqual(report.vendorSourceInclusion.forbiddenInclusions, []);
  assert.deepEqual(report.publicSourceExposure.checkedFiles, ["dx-www/src/lib.rs", "dx-www/src/main.rs"]);
  assert.deepEqual(report.publicSourceExposure.forbiddenExposures, []);
  assert.equal(report.runtimeTakeover.reactRscRequired, false);
  assert.equal(report.runtimeTakeover.nodeNapiRequired, false);
  assert.equal(report.runtimeTakeover.nodeModulesDefault, false);
});

test("Next/Turbopack Rust vendor boundary checker rejects unexpected imported groups", () => {
  const fixtureRoot = materializeVendorBoundaryFixture(["turbopack/crates/next-core"]);

  const report = buildNextRustVendorBoundaryReport(fixtureRoot);

  assert.equal(report.status, "fail");
  assert.deepEqual(report.importedGroups.unexpectedFilesystem, ["turbopack/crates/next-core"]);
  assert.deepEqual(report.importedGroups.missingFromReadme, ["turbopack/crates/next-core"]);
  assert.deepEqual(report.importedGroups.missingFromRustMetadata, ["turbopack/crates/next-core"]);
  assert.deepEqual(report.importedGroups.missingCargoMetadata, []);
});

test("Next/Turbopack Rust vendor boundary checker rejects public Next/React/Turbo dependency claims", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();
  writeText(
    path.join(fixtureRoot, "package.json"),
    `${JSON.stringify(
      {
        private: true,
        dependencies: {
          next: "16.2.6",
          react: "19.2.6",
        },
        devDependencies: {
          turbo: "3.0.0",
        },
      },
      null,
      2,
    )}\n`,
  );

  const report = buildNextRustVendorBoundaryReport(fixtureRoot);

  assert.equal(report.status, "fail");
  assert.deepEqual(report.publicDependencyClaims.forbiddenDependencies, [
    {
      manifest: "package.json",
      scope: "dependencies",
      dependency: "next",
      reason: "Next.js must not become a public DX-WWW dependency foundation",
    },
    {
      manifest: "package.json",
      scope: "dependencies",
      dependency: "react",
      reason: "React/RSC must not become a required DX-WWW core dependency",
    },
    {
      manifest: "package.json",
      scope: "devDependencies",
      dependency: "turbo",
      reason: "Turborepo/npm tooling must not become the default DX-WWW foundation",
    },
  ]);
});

test("Next/Turbopack Rust vendor boundary checker rejects public overclaim text", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();
  writeText(
    path.join(fixtureRoot, "DX.md"),
    [
      "# DX",
      "DX-WWW has full Next.js parity and Turbopack is the public architecture.",
      "React/RSC is the required core app model.",
      "node_modules is the default resolver.",
    ].join("\n"),
  );

  const report = buildNextRustVendorBoundaryReport(fixtureRoot);

  assert.equal(report.status, "fail");
  assert.deepEqual(
    report.publicClaimText.forbiddenClaims.map((claim) => ({
      file: claim.file,
      line: claim.line,
      claim: claim.claim,
    })),
    [
      { file: "DX.md", line: 2, claim: "full-next-parity" },
      { file: "DX.md", line: 2, claim: "turbopack-public-architecture" },
      { file: "DX.md", line: 3, claim: "react-rsc-core" },
      { file: "DX.md", line: 4, claim: "node-modules-default" },
    ],
  );
});

test("Next/Turbopack Rust vendor boundary checker rejects removed DevTools and Turbopack targets", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();
  writeText(
    path.join(fixtureRoot, "TODO.md"),
    [
      "# TODO",
      "Ship Next DevTools clone parity as the DX-WWW debugging target.",
      "Turbopack powers dx build and dx dev in the production path.",
      "Real Turbopack runtime/build adoption is in scope for this merge.",
    ].join("\n"),
  );

  const report = buildNextRustVendorBoundaryReport(fixtureRoot);

  assert.equal(report.status, "fail");
  assert.deepEqual(
    report.publicClaimText.forbiddenClaims.map((claim) => ({
      file: claim.file,
      line: claim.line,
      claim: claim.claim,
    })),
    [
      { file: "TODO.md", line: 2, claim: "next-devtools-clone-target" },
      { file: "TODO.md", line: 3, claim: "turbopack-runtime-build-adoption" },
      { file: "TODO.md", line: 4, claim: "turbopack-runtime-build-adoption" },
    ],
  );
});

test("Next/Turbopack Rust vendor boundary checker rejects stale DX DevTools routes and bundler proof targets", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();
  writeText(
    path.join(fixtureRoot, "TODO.md"),
    [
      "# TODO",
      "Restore /_dx/devtools as the DX-WWW DevTools surface.",
      "Keep source-safe DevTools code frames and external DevTools runtime parity.",
      "External bundler execution proof remains unclaimed for this merge.",
    ].join("\n"),
  );

  const report = buildNextRustVendorBoundaryReport(fixtureRoot);

  assert.equal(report.status, "fail");
  assert.deepEqual(
    report.publicClaimText.forbiddenClaims.map((claim) => ({
      file: claim.file,
      line: claim.line,
      claim: claim.claim,
    })),
    [
      { file: "TODO.md", line: 2, claim: "dx-devtools-removed-target" },
      { file: "TODO.md", line: 3, claim: "dx-devtools-removed-target" },
      { file: "TODO.md", line: 4, claim: "external-bundler-execution-proof-target" },
    ],
  );
});

test("Next/Turbopack Rust vendor boundary checker allows honest unproven claim text", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();
  writeText(
    path.join(fixtureRoot, "DX.md"),
    [
      "# DX",
      "This is not full Next.js parity; async RSC and runtime takeover remain unproven.",
      "React/RSC, Node/NAPI, node_modules, and public Turbopack architecture stay visibly false.",
      "DX-WWW keeps hot reload and a basic DX-owned overlay; no Next DevTools clone parity is in scope.",
      "",
      "Real Turbopack runtime/build adoption is out of scope; Turbopack stays reference/provenance only.",
    ].join("\n"),
  );

  const report = buildNextRustVendorBoundaryReport(fixtureRoot);

  assert.equal(report.status, "ok");
  assert.deepEqual(report.publicClaimText.forbiddenClaims, []);
});

test("Next/Turbopack Rust vendor boundary checker rejects hidden runtime dependencies in vendored Cargo metadata", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();
  writeText(
    path.join(fixtureRoot, "vendor", "next-rust", "turbopack", "crates", "turbo-tasks", "Cargo.toml"),
    [
      "[package]",
      "name = \"turbo-tasks\"",
      "version = \"0.0.0\"",
      "",
      "[dependencies]",
      "next-core = { path = \"../../next-core\" }",
      "",
      "[build-dependencies.turbopack-nodejs]",
      "path = \"../turbopack-nodejs\"",
      "",
    ].join("\n"),
  );

  const report = buildNextRustVendorBoundaryReport(fixtureRoot);

  assert.equal(report.status, "fail");
  assert.deepEqual(report.vendoredCargoDependencyClaims.forbiddenDependencies, [
    {
      manifest: "vendor/next-rust/turbopack/crates/turbo-tasks/Cargo.toml",
      dependency: "next-core",
      reason: "Vendored build-layer groups must not depend on Next.js runtime core",
    },
    {
      manifest: "vendor/next-rust/turbopack/crates/turbo-tasks/Cargo.toml",
      dependency: "turbopack-nodejs",
      reason: "Vendored build-layer groups must not depend on Node-backed Turbopack runtime",
    },
  ]);
});

test("Next/Turbopack Rust vendor boundary checker rejects direct vendored Rust source inclusion", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();
  writeText(
    path.join(fixtureRoot, "core", "src", "runtime.rs"),
    "#[path = \"../../vendor/next-rust/turbopack/crates/turbopack-core/src/lib.rs\"]\nmod turbopack_core;\n",
  );
  writeText(
    path.join(fixtureRoot, "dx-www", "src", "dev", "vendor_takeover.rs"),
    "include!(\"../../../vendor/next-rust/crates/next-code-frame/src/lib.rs\");\n",
  );

  const report = buildNextRustVendorBoundaryReport(fixtureRoot);

  assert.equal(report.status, "fail");
  assert.deepEqual(report.vendorSourceInclusion.forbiddenInclusions, [
    {
      file: "core/src/runtime.rs",
      pattern: "#[path]",
      target: "../../vendor/next-rust/turbopack/crates/turbopack-core/src/lib.rs",
      reason: "Vendored Next/Turbopack Rust source must be wrapped, not compiled directly into DX runtime",
    },
    {
      file: "dx-www/src/dev/vendor_takeover.rs",
      pattern: "include!",
      target: "../../../vendor/next-rust/crates/next-code-frame/src/lib.rs",
      reason: "Vendored Next/Turbopack Rust source must be wrapped, not compiled directly into DX runtime",
    },
  ]);
});

test("Next/Turbopack Rust vendor boundary checker rejects public source exposure of Next/Turbopack internals", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();
  writeText(path.join(fixtureRoot, "dx-www", "src", "lib.rs"), "pub use next_core::server::NextServer;\n");
  writeText(path.join(fixtureRoot, "dx-www", "src", "main.rs"), "use turbopack_core::asset::Asset;\nfn main() {}\n");
  writeText(path.join(fixtureRoot, "dx-www", "src", "public_api.ts"), "import { NextResponse } from 'next/server';\n");

  const report = buildNextRustVendorBoundaryReport(fixtureRoot);

  assert.equal(report.status, "fail");
  assert.deepEqual(report.publicSourceExposure.checkedFiles, [
    "dx-www/src/lib.rs",
    "dx-www/src/main.rs",
    "dx-www/src/public_api.ts",
  ]);
  assert.deepEqual(report.publicSourceExposure.forbiddenExposures, [
    {
      file: "dx-www/src/lib.rs",
      exposure: "next_core",
      reason: "Next.js runtime internals must not be re-exported from DX-WWW public source",
    },
    {
      file: "dx-www/src/main.rs",
      exposure: "turbopack_core",
      reason: "Turbopack internals require a DX-owned adapter before public source exposure",
    },
    {
      file: "dx-www/src/public_api.ts",
      exposure: "next/server",
      reason: "Next.js runtime modules must not become DX-WWW public API imports",
    },
  ]);
});

test("Next/Turbopack Rust vendor boundary checker writes a durable receipt", () => {
  const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-next-rust-boundary-"));
  const receiptPath = path.join(tempRoot, ".dx", "receipts", "next-rust", "vendor-boundary.json");

  const receipt = writeNextRustVendorBoundaryReceipt(repoRoot, receiptPath);
  const written = JSON.parse(fs.readFileSync(receiptPath, "utf8"));

  assert.equal(receipt.receiptPath, receiptPath);
  assert.equal(written.schema, "dx.nextRust.vendorBoundary");
  assert.equal(written.status, "ok");
  assert.equal(written.receipt.kind, "dx.nextRust.vendorBoundary");
  assert.equal(written.receipt.path, ".dx/receipts/next-rust/vendor-boundary.json");
  assert.equal(written.workspaceQuarantine.vendorInWorkspace, false);
  assert.equal(written.runtimeTakeover.reactRscRequired, false);
  assert.equal(written.runtimeTakeover.nodeModulesDefault, false);
});

test("Next/Turbopack Rust vendor boundary receipts avoid public version-suffix schema noise", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();

  const sourceReceipt = writeNextRustVendorBoundaryReceipt(fixtureRoot);
  const consumerReceipt = writeNextRustVendorBoundaryConsumerReceipt(fixtureRoot);
  const sourceWritten = JSON.parse(fs.readFileSync(sourceReceipt.receiptPath, "utf8"));
  const consumerWritten = JSON.parse(fs.readFileSync(consumerReceipt.receiptPath, "utf8"));
  const versionSuffixSchemas = collectStringValues({ sourceWritten, consumerWritten }).filter((value) =>
    /^dx\.[A-Za-z0-9.]+\.v\d+$/.test(value),
  );

  assert.deepEqual(versionSuffixSchemas, []);
  assert.equal(sourceWritten.schema, "dx.nextRust.vendorBoundary");
  assert.equal(consumerWritten.schema, "dx.nextRust.vendorBoundary.consumerReceipt");
  assert.equal(consumerWritten.snapshot.schema, "dx.nextRust.vendorBoundary.consumerSnapshot");
  assert.equal(consumerWritten.snapshot.claimPolicy.schema, "dx.nextRust.vendorBoundary.claimPolicy");
  assert.equal(consumerWritten.snapshot.adapterBoundary.schema, "dx.nextRust.vendorBoundary.adapterBoundary");
});

test("Next/Turbopack Rust vendor boundary checker detects stale receipts", () => {
  const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-next-rust-boundary-"));
  const receiptPath = path.join(tempRoot, ".dx", "receipts", "next-rust", "vendor-boundary.json");

  writeNextRustVendorBoundaryReceipt(repoRoot, receiptPath);
  const fresh = verifyNextRustVendorBoundaryReceipt(repoRoot, receiptPath);
  assert.equal(fresh.status, "ok");
  assert.equal(fresh.stale, false);
  assert.deepEqual(fresh.mismatches, []);

  const staleReceipt = JSON.parse(fs.readFileSync(receiptPath, "utf8"));
  staleReceipt.runtimeTakeover.reactRscRequired = true;
  fs.writeFileSync(receiptPath, `${JSON.stringify(staleReceipt, null, 2)}\n`, "utf8");

  const stale = verifyNextRustVendorBoundaryReceipt(repoRoot, receiptPath);
  assert.equal(stale.status, "stale");
  assert.equal(stale.stale, true);
  assert.deepEqual(stale.mismatches, ["runtimeTakeover.reactRscRequired"]);
});

test("Next/Turbopack Rust vendor boundary receipt ignores clean source scan-list churn", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();

  writeNextRustVendorBoundaryReceipt(fixtureRoot);
  writeText(path.join(fixtureRoot, "core", "src", "new_safe_module.rs"), "pub fn source_owned() {}\n");

  const fresh = verifyNextRustVendorBoundaryReceipt(fixtureRoot);

  assert.equal(fresh.status, "ok");
  assert.equal(fresh.stale, false);
  assert.deepEqual(fresh.mismatches, []);
});

test("Next/Turbopack Rust vendor boundary receipt still rejects new forbidden source inclusions", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();

  writeNextRustVendorBoundaryReceipt(fixtureRoot);
  writeText(
    path.join(fixtureRoot, "core", "src", "vendor_takeover.rs"),
    "include!(\"../../vendor/next-rust/crates/next-code-frame/src/lib.rs\");\n",
  );

  const stale = verifyNextRustVendorBoundaryReceipt(fixtureRoot);

  assert.equal(stale.status, "stale");
  assert.equal(stale.stale, true);
  assert.deepEqual(stale.mismatches, ["status", "vendorSourceInclusion.forbiddenInclusions"]);
});

test("Next/Turbopack Rust vendor boundary exposes a compact consumer snapshot", () => {
  const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-next-rust-boundary-"));
  const receiptPath = path.join(tempRoot, ".dx", "receipts", "next-rust", "vendor-boundary.json");

  writeNextRustVendorBoundaryReceipt(repoRoot, receiptPath);
  const snapshot = buildNextRustVendorBoundaryConsumerSnapshot(repoRoot, receiptPath);

  assert.equal(snapshot.schema, "dx.nextRust.vendorBoundary.consumerSnapshot");
  assert.equal(snapshot.status, "ok");
  assert.equal(snapshot.vendor.root, "vendor/next-rust");
  assert.equal(snapshot.vendor.role, "quarantined build infrastructure reference");
  assert.equal(snapshot.vendor.publicArchitecture, "DX-WWW runtime/security/source model");
  assert.equal(snapshot.upstream.commit, "f3f56ecec2f3f8cefa0f0a1323ea406740251d5c");
  assert.equal(snapshot.counts.importedGroups, importedRustGroups.length);
  assert.equal(snapshot.counts.protectedBoundaries, 12);
  assert.equal(snapshot.counts.excludedCoreFoundations, 8);
  assert.equal(snapshot.sourceReceipt.fresh, true);
  assert.equal(snapshot.sourceReceipt.mismatchCount, 0);
  assert.deepEqual(snapshot.sourceReceipt.mismatches, []);
  assert.equal(snapshot.boundary.workspaceQuarantined, true);
  assert.equal(snapshot.boundary.runtimeTakeoverBlocked, true);
  assert.equal(snapshot.boundary.publicDependencyClaimsBlocked, true);
  assert.equal(snapshot.boundary.vendoredCargoDependencyClaimsBlocked, true);
  assert.equal(snapshot.boundary.vendorSourceInclusionBlocked, true);
  assert.equal(snapshot.boundary.publicSourceExposureBlocked, true);
  assert.equal(snapshot.boundary.nextRuntimeRequired, false);
  assert.equal(snapshot.boundary.reactRscRequired, false);
  assert.equal(snapshot.boundary.nodeNapiRequired, false);
  assert.equal(snapshot.boundary.nodeModulesDefault, false);
  assert.equal(snapshot.boundary.turbopackPublicArchitecture, false);
  assert.equal(snapshot.importedGroups, undefined);
  assert.equal(snapshot.protectedBoundaries, undefined);
});

test("Next/Turbopack Rust vendor boundary consumer snapshot carries an explicit no-overclaim policy", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();

  writeNextRustVendorBoundaryReceipt(fixtureRoot);
  const snapshot = buildNextRustVendorBoundaryConsumerSnapshot(fixtureRoot);

  assert.equal(snapshot.claimPolicy.schema, "dx.nextRust.vendorBoundary.claimPolicy");
  assert.equal(snapshot.claimPolicy.fullNextParityClaimed, false);
  assert.equal(snapshot.claimPolicy.nextRuntimeTakeoverClaimed, false);
  assert.equal(snapshot.claimPolicy.reactRscCoreDependencyClaimed, false);
  assert.equal(snapshot.claimPolicy.nodeNapiFoundationClaimed, false);
  assert.equal(snapshot.claimPolicy.nodeModulesDefaultClaimed, false);
  assert.equal(snapshot.claimPolicy.turbopackPublicArchitectureClaimed, false);
  assert.deepEqual(snapshot.claimPolicy.allowedClaimScope, [
    "quarantined provenance",
    "license evidence",
    "selected Rust build infrastructure reference",
    "DX-owned receipt handoff",
  ]);
  assert.deepEqual(snapshot.claimPolicy.unprovenClaims, [
    "full Next.js parity",
    "Next.js runtime takeover",
    "React/RSC core app model",
    "Node/NAPI default foundation",
    "node_modules default resolver",
    "Turbopack public architecture",
  ]);
});

test("Next/Turbopack Rust vendor boundary consumer receipt marks adapter-only integration gaps", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();

  writeNextRustVendorBoundaryReceipt(fixtureRoot);
  const receipt = writeNextRustVendorBoundaryConsumerReceipt(fixtureRoot);
  const written = JSON.parse(fs.readFileSync(receipt.receiptPath, "utf8"));

  assert.equal(written.snapshot.adapterBoundary.schema, "dx.nextRust.vendorBoundary.adapterBoundary");
  assert.equal(written.snapshot.adapterBoundary.sourceOnlyReceipt, true);
  assert.equal(written.snapshot.adapterBoundary.nativeDxCheckWired, false);
  assert.equal(written.snapshot.adapterBoundary.nativeZedSurfaceWired, false);
  assert.equal(written.snapshot.adapterBoundary.nativeStudioSurfaceWired, false);
  assert.equal(written.snapshot.adapterBoundary.cargoWorkspaceWired, false);
  assert.equal(written.snapshot.adapterBoundary.publicTurbopackApi, false);
  assert.deepEqual(written.snapshot.adapterBoundary.allowedConsumers, [
    "Forge/source receipts",
    "dx-check receipt adapters",
    "Zed/Studio status surfaces",
  ]);
  assert.deepEqual(written.snapshot.adapterBoundary.requiredFreshnessChecks, [
    ".dx/receipts/next-rust/vendor-boundary.json",
    ".dx/receipts/next-rust/vendor-boundary-consumer.json",
  ]);
  assert.deepEqual(written.snapshot.adapterBoundary.blockedUntilProven, [
    "native dx-check rendering",
    "native Zed surface rendering",
    "native Studio surface rendering",
    "Cargo workspace dependency wiring",
    "runtime use of vendored Next/Turbopack crates",
  ]);
});

test("Next/Turbopack Rust vendor boundary writes a DX-owned consumer receipt", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();

  writeNextRustVendorBoundaryReceipt(fixtureRoot);
  const receipt = writeNextRustVendorBoundaryConsumerReceipt(fixtureRoot);
  const written = JSON.parse(fs.readFileSync(receipt.receiptPath, "utf8"));

  assert.equal(
    receipt.receiptPath,
    path.join(fixtureRoot, ".dx", "receipts", "next-rust", "vendor-boundary-consumer.json"),
  );
  assert.equal(written.schema, "dx.nextRust.vendorBoundary.consumerReceipt");
  assert.equal(written.status, "ok");
  assert.equal(written.receipt.kind, "dx.nextRust.vendorBoundary.consumer");
  assert.equal(written.receipt.path, ".dx/receipts/next-rust/vendor-boundary-consumer.json");
  assert.equal(written.snapshot.schema, "dx.nextRust.vendorBoundary.consumerSnapshot");
  assert.equal(written.snapshot.sourceReceipt.path, ".dx/receipts/next-rust/vendor-boundary.json");
  assert.equal(written.snapshot.sourceReceipt.fresh, true);
  assert.equal(written.snapshot.vendor.publicArchitecture, "DX-WWW runtime/security/source model");
  assert.equal(written.snapshot.boundary.runtimeTakeoverBlocked, true);
  assert.equal(written.snapshot.boundary.publicDependencyClaimsBlocked, true);
  assert.equal(written.snapshot.boundary.vendoredCargoDependencyClaimsBlocked, true);
  assert.equal(written.snapshot.boundary.vendorSourceInclusionBlocked, true);
  assert.equal(written.snapshot.boundary.publicSourceExposureBlocked, true);
  assert.equal(written.snapshot.boundary.turbopackPublicArchitecture, false);
  assert.equal(written.snapshot.importedGroups, undefined);
  assert.equal(written.snapshot.protectedBoundaries, undefined);
});

test("Next/Turbopack Rust vendor boundary exposes an executable dx-check status surface", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();

  writeNextRustVendorBoundaryReceipt(fixtureRoot);
  writeNextRustVendorBoundaryConsumerReceipt(fixtureRoot);
  const surface = buildNextRustVendorBoundaryStatusSurface(fixtureRoot);

  assert.equal(surface.schema, "dx.nextRust.vendorBoundary.statusSurface");
  assert.equal(surface.status, "ok");
  assert.equal(surface.surface.kind, "dx-check.statusSurface");
  assert.equal(surface.surface.id, "next-rust-vendor-boundary");
  assert.equal(surface.surface.owner, "DX-WWW");
  assert.equal(surface.surface.adapterBoundary, "executable receipt adapter, not native runtime integration");
  assert.deepEqual(surface.evidence.sourceReceipt, {
    path: ".dx/receipts/next-rust/vendor-boundary.json",
    status: "ok",
    stale: false,
    mismatches: [],
  });
  assert.deepEqual(surface.evidence.consumerReceipt, {
    path: ".dx/receipts/next-rust/vendor-boundary-consumer.json",
    status: "ok",
    stale: false,
    mismatches: [],
  });
  assert.deepEqual(surface.dxCheck, {
    schema: "dx.nextRust.vendorBoundary.dxCheckProjection",
    status: "ok",
    adapterBoundary: "source-owned JSON projection for dx-check receipt consumers; native renderer unimplemented",
    checks: [
      {
        id: "next-rust.vendor-boundary.receipts",
        title: "Next/Turbopack Rust vendor boundary receipts are fresh",
        status: "ok",
        severity: "blocking",
        messages: [],
        evidencePaths: [
          ".dx/receipts/next-rust/vendor-boundary.json",
          ".dx/receipts/next-rust/vendor-boundary-consumer.json",
        ],
      },
      {
        id: "next-rust.vendor-boundary.runtime-takeover",
        title: "Vendored Next/Turbopack Rust does not replace DX runtime foundations",
        status: "ok",
        severity: "blocking",
        messages: [],
        evidence: {
          runtimeTakeoverBlocked: true,
          nextRuntimeRequired: false,
          reactRscRequired: false,
          nodeNapiRequired: false,
          nodeModulesDefault: false,
          turbopackPublicArchitecture: false,
        },
      },
      {
        id: "next-rust.vendor-boundary.public-claims",
        title: "Public dependency and source claims stay DX-owned",
        status: "ok",
        severity: "blocking",
        messages: [],
        evidence: {
          publicDependencyClaimsBlocked: true,
          vendoredCargoDependencyClaimsBlocked: true,
          vendorSourceInclusionBlocked: true,
          publicSourceExposureBlocked: true,
        },
      },
      {
        id: "next-rust.vendor-boundary.active-scope",
        title: "DX-WWW active scope excludes DevTools clone and Turbopack adoption targets",
        status: "ok",
        severity: "blocking",
        messages: [],
        evidence: {
          schema: "dx.nextRust.vendorBoundary.activeScope",
          status: "ok",
          referenceOnlyNextRust: true,
          runtimeBuildAdoption: false,
          turbopackPublicArchitecture: false,
          devFeedbackEndpoint: "/_dx/feedback",
          removedTargetsBlocked: true,
          excludedRuntimeTargetsBlocked: true,
          publicClaimChecks: [
            "next-devtools-clone-target",
            "dx-devtools-removed-target",
            "turbopack-runtime-build-adoption",
            "external-bundler-execution-proof-target",
          ],
          activeScopeSummary: {
            schema: "dx.nextRust.vendorBoundary.activeScopeSummary",
            status: "ok",
            scopeStatus: "ok",
            mismatchCount: 0,
            mismatches: [],
          },
        },
      },
    ],
  });
  assert.deepEqual(surface.activeScopeSummary, {
    schema: "dx.nextRust.vendorBoundary.activeScopeSummary",
    status: "ok",
    scopeStatus: "ok",
    mismatchCount: 0,
    mismatches: [],
  });
  const expectedEditorActiveScope = {
    schema: "dx.nextRust.vendorBoundary.activeScope",
    status: "ok",
    referenceOnlyNextRust: true,
    runtimeBuildAdoption: false,
    turbopackPublicArchitecture: false,
    devFeedbackEndpoint: "/_dx/feedback",
    removedTargetsBlocked: true,
    removedTargets: [
      "Next DevTools clone",
      "DX-WWW DevTools",
      "/_dx/devtools",
      "external DevTools runtime",
    ],
    excludedRuntimeTargetsBlocked: true,
    excludedRuntimeTargets: [
      "Turbopack runtime/build adoption",
      "external bundler execution proof",
      "Turbopack powers dx build/dev",
    ],
    publicClaimChecks: [
      "next-devtools-clone-target",
      "dx-devtools-removed-target",
      "turbopack-runtime-build-adoption",
      "external-bundler-execution-proof-target",
    ],
  };
  assert.deepEqual(surface.editorSurfaces, {
    schema: "dx.nextRust.vendorBoundary.editorSurfaceProjection",
    status: "ok",
    adapterBoundary: "source-owned JSON projection for Zed/Studio receipt consumers; native renderers unimplemented",
    surfaces: [
      {
        id: "next-rust.vendor-boundary.zed",
        kind: "zed.statusSurface",
        title: "Next/Turbopack Rust vendor boundary",
        status: "ok",
        severity: "blocking",
        messages: [],
        badges: [
          { label: "receipts fresh", status: "ok" },
          { label: "DX runtime protected", status: "ok" },
          { label: "public claims blocked", status: "ok" },
          { label: "DevTools targets removed", status: "ok" },
          { label: "Turbopack adoption blocked", status: "ok" },
        ],
        receiptPaths: [
          ".dx/receipts/next-rust/vendor-boundary.json",
          ".dx/receipts/next-rust/vendor-boundary-consumer.json",
        ],
        blockedUntilProven: ["native Zed surface rendering"],
        activeScope: expectedEditorActiveScope,
      },
      {
        id: "next-rust.vendor-boundary.studio",
        kind: "studio.statusSurface",
        title: "Next/Turbopack Rust vendor boundary",
        status: "ok",
        severity: "blocking",
        messages: [],
        badges: [
          { label: "receipts fresh", status: "ok" },
          { label: "DX runtime protected", status: "ok" },
          { label: "public claims blocked", status: "ok" },
          { label: "DevTools targets removed", status: "ok" },
          { label: "Turbopack adoption blocked", status: "ok" },
        ],
        receiptPaths: [
          ".dx/receipts/next-rust/vendor-boundary.json",
          ".dx/receipts/next-rust/vendor-boundary-consumer.json",
        ],
        blockedUntilProven: ["native Studio surface rendering"],
        activeScope: expectedEditorActiveScope,
      },
    ],
  });
  assert.deepEqual(surface.markers, [
    {
      name: "data-dx-next-rust-vendor-boundary-status",
      value: "ok",
    },
    {
      name: "data-dx-next-rust-vendor-boundary-upstream",
      value: "f3f56ecec2f3f8cefa0f0a1323ea406740251d5c",
    },
    {
      name: "data-dx-next-rust-vendor-boundary-public-architecture",
      value: "DX-WWW runtime/security/source model",
    },
    {
      name: "data-dx-next-rust-vendor-boundary-claim-scope",
      value: "quarantined provenance|license evidence|selected Rust build infrastructure reference|DX-owned receipt handoff",
    },
  ]);
  assert.deepEqual(surface.unproven, [
    "native dx-check rendering",
    "native Zed surface rendering",
    "native Studio surface rendering",
    "Cargo workspace dependency wiring",
    "runtime use of vendored Next/Turbopack crates",
  ]);

  const checkerPath = path.join(repoRoot, "tools", "vendor", "next-rust-boundary-check.js");
  const cliSurface = JSON.parse(
    execFileSync(process.execPath, [checkerPath, "--repo-root", fixtureRoot, "--dx-check-surface"], {
      encoding: "utf8",
    }),
  );
  assert.equal(cliSurface.status, "ok");
  assert.equal(cliSurface.surface.id, "next-rust-vendor-boundary");
  assert.equal(cliSurface.dxCheck.status, "ok");
  assert.equal(cliSurface.dxCheck.checks.length, 4);
  assert.equal(cliSurface.dxCheck.checks[3].id, "next-rust.vendor-boundary.active-scope");
  assert.equal(cliSurface.editorSurfaces.status, "ok");
  assert.equal(cliSurface.editorSurfaces.surfaces.length, 2);
});

test("Next/Turbopack Rust vendor boundary dx-check status surface blocks stale receipts", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();

  writeNextRustVendorBoundaryReceipt(fixtureRoot);
  const receipt = writeNextRustVendorBoundaryConsumerReceipt(fixtureRoot);
  const staleReceipt = JSON.parse(fs.readFileSync(receipt.receiptPath, "utf8"));
  staleReceipt.snapshot.boundary.publicSourceExposureBlocked = false;
  fs.writeFileSync(receipt.receiptPath, `${JSON.stringify(staleReceipt, null, 2)}\n`, "utf8");

  const surface = buildNextRustVendorBoundaryStatusSurface(fixtureRoot);

  assert.equal(surface.status, "blocked");
  assert.equal(surface.evidence.consumerReceipt.status, "stale");
  assert.deepEqual(surface.evidence.consumerReceipt.mismatches, [
    "snapshot.boundary.publicSourceExposureBlocked",
  ]);
  assert.deepEqual(surface.blockers, [
    "consumer receipt stale:snapshot.boundary.publicSourceExposureBlocked",
  ]);
  assert.equal(surface.dxCheck.status, "fail");
  assert.deepEqual(surface.dxCheck.checks[0], {
    id: "next-rust.vendor-boundary.receipts",
    title: "Next/Turbopack Rust vendor boundary receipts are fresh",
    status: "fail",
    severity: "blocking",
    messages: ["consumer receipt stale:snapshot.boundary.publicSourceExposureBlocked"],
    evidencePaths: [
      ".dx/receipts/next-rust/vendor-boundary.json",
      ".dx/receipts/next-rust/vendor-boundary-consumer.json",
    ],
  });
  assert.deepEqual(surface.dxCheck.checks[3], {
    id: "next-rust.vendor-boundary.active-scope",
    title: "DX-WWW active scope excludes DevTools clone and Turbopack adoption targets",
    status: "fail",
    severity: "blocking",
    messages: ["activeScope.status:blocked"],
    evidence: {
      schema: "dx.nextRust.vendorBoundary.activeScope",
      status: "blocked",
      referenceOnlyNextRust: true,
      runtimeBuildAdoption: false,
      turbopackPublicArchitecture: false,
      devFeedbackEndpoint: "/_dx/feedback",
      removedTargetsBlocked: true,
      excludedRuntimeTargetsBlocked: true,
      publicClaimChecks: [
        "next-devtools-clone-target",
        "dx-devtools-removed-target",
        "turbopack-runtime-build-adoption",
        "external-bundler-execution-proof-target",
      ],
      activeScopeSummary: {
        schema: "dx.nextRust.vendorBoundary.activeScopeSummary",
        status: "ok",
        scopeStatus: "blocked",
        mismatchCount: 0,
        mismatches: [],
      },
    },
  });
  assert.deepEqual(surface.activeScopeSummary, {
    schema: "dx.nextRust.vendorBoundary.activeScopeSummary",
    status: "ok",
    scopeStatus: "blocked",
    mismatchCount: 0,
    mismatches: [],
  });
  assert.equal(surface.editorSurfaces.status, "fail");
  assert.equal(surface.editorSurfaces.surfaces[0].status, "fail");
  assert.deepEqual(surface.editorSurfaces.surfaces[0].messages, [
    "consumer receipt stale:snapshot.boundary.publicSourceExposureBlocked",
  ]);
});

test("Next/Turbopack Rust vendor boundary exposes compact consumer surfaces", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();

  writeNextRustVendorBoundaryReceipt(fixtureRoot);
  writeNextRustVendorBoundaryConsumerReceipt(fixtureRoot);
  const surfaces = buildNextRustVendorBoundaryConsumerSurfaces(fixtureRoot);

  assert.equal(surfaces.schema, "dx.nextRust.vendorBoundary.consumerSurfaces");
  assert.equal(surfaces.status, "ok");
  assert.deepEqual(surfaces.source, {
    statusSurface: "next-rust-vendor-boundary",
    owner: "DX-WWW",
    adapterBoundary: "executable receipt adapter, not native runtime integration",
    receiptPaths: [
      ".dx/receipts/next-rust/vendor-boundary.json",
      ".dx/receipts/next-rust/vendor-boundary-consumer.json",
    ],
  });
  assert.equal(surfaces.dxCheck.status, "ok");
  assert.equal(surfaces.dxCheck.checks.length, 4);
  assert.equal(surfaces.dxCheck.checks[3].id, "next-rust.vendor-boundary.active-scope");
  assert.equal(surfaces.editorSurfaces.status, "ok");
  assert.deepEqual(
    surfaces.editorSurfaces.surfaces.map((surface) => surface.kind),
    ["zed.statusSurface", "studio.statusSurface"],
  );
  assert.deepEqual(surfaces.blockers, []);
  assert.deepEqual(surfaces.unproven, [
    "native dx-check rendering",
    "native Zed surface rendering",
    "native Studio surface rendering",
    "Cargo workspace dependency wiring",
    "runtime use of vendored Next/Turbopack crates",
  ]);
  assert.deepEqual(surfaces.activeScope, {
    schema: "dx.nextRust.vendorBoundary.activeScope",
    status: "ok",
    referenceOnlyNextRust: true,
    runtimeBuildAdoption: false,
    turbopackPublicArchitecture: false,
    devFeedbackEndpoint: "/_dx/feedback",
    removedTargetsBlocked: true,
    removedTargets: [
      "Next DevTools clone",
      "DX-WWW DevTools",
      "/_dx/devtools",
      "external DevTools runtime",
    ],
    excludedRuntimeTargetsBlocked: true,
    excludedRuntimeTargets: [
      "Turbopack runtime/build adoption",
      "external bundler execution proof",
      "Turbopack powers dx build/dev",
    ],
    publicClaimChecks: [
      "next-devtools-clone-target",
      "dx-devtools-removed-target",
      "turbopack-runtime-build-adoption",
      "external-bundler-execution-proof-target",
    ],
  });
  assert.equal(surfaces.evidence, undefined);
  assert.equal(surfaces.markers, undefined);

  const checkerPath = path.join(repoRoot, "tools", "vendor", "next-rust-boundary-check.js");
  const cliSurfaces = JSON.parse(
    execFileSync(process.execPath, [checkerPath, "--repo-root", fixtureRoot, "--consumer-surfaces"], {
      encoding: "utf8",
    }),
  );
  assert.equal(cliSurfaces.status, "ok");
  assert.equal(cliSurfaces.schema, "dx.nextRust.vendorBoundary.consumerSurfaces");
  assert.equal(cliSurfaces.dxCheck.status, "ok");
  assert.equal(cliSurfaces.dxCheck.checks[3].status, "ok");
  assert.equal(cliSurfaces.editorSurfaces.surfaces.length, 2);
  assert.equal(cliSurfaces.activeScope.schema, "dx.nextRust.vendorBoundary.activeScope");
  assert.equal(cliSurfaces.activeScope.removedTargetsBlocked, true);
  assert.equal(cliSurfaces.activeScope.excludedRuntimeTargetsBlocked, true);
  assert.equal(cliSurfaces.evidence, undefined);
  assert.equal(cliSurfaces.markers, undefined);
});

test("Next/Turbopack Rust vendor boundary compact consumer surfaces block stale receipts", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();

  writeNextRustVendorBoundaryReceipt(fixtureRoot);
  const receipt = writeNextRustVendorBoundaryConsumerReceipt(fixtureRoot);
  const staleReceipt = JSON.parse(fs.readFileSync(receipt.receiptPath, "utf8"));
  staleReceipt.snapshot.boundary.publicSourceExposureBlocked = false;
  fs.writeFileSync(receipt.receiptPath, `${JSON.stringify(staleReceipt, null, 2)}\n`, "utf8");

  const surfaces = buildNextRustVendorBoundaryConsumerSurfaces(fixtureRoot);

  assert.equal(surfaces.status, "blocked");
  assert.equal(surfaces.dxCheck.status, "fail");
  assert.equal(surfaces.editorSurfaces.status, "fail");
  assert.equal(surfaces.activeScope.status, "blocked");
  assert.deepEqual(surfaces.blockers, [
    "consumer receipt stale:snapshot.boundary.publicSourceExposureBlocked",
  ]);
});

test("Next/Turbopack Rust vendor boundary verifier rejects missing and stale consumer receipts", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();

  const missing = verifyNextRustVendorBoundaryConsumerReceipt(fixtureRoot);
  assert.equal(missing.status, "missing");
  assert.equal(missing.stale, true);
  assert.deepEqual(missing.mismatches, ["receipt"]);

  writeNextRustVendorBoundaryReceipt(fixtureRoot);
  const receipt = writeNextRustVendorBoundaryConsumerReceipt(fixtureRoot);
  const fresh = verifyNextRustVendorBoundaryConsumerReceipt(fixtureRoot);
  assert.equal(fresh.status, "ok");
  assert.equal(fresh.stale, false);
  assert.deepEqual(fresh.mismatches, []);

  const staleReceipt = JSON.parse(fs.readFileSync(receipt.receiptPath, "utf8"));
  staleReceipt.snapshot.boundary.runtimeTakeoverBlocked = false;
  fs.writeFileSync(receipt.receiptPath, `${JSON.stringify(staleReceipt, null, 2)}\n`, "utf8");

  const stale = verifyNextRustVendorBoundaryConsumerReceipt(fixtureRoot);
  assert.equal(stale.status, "stale");
  assert.equal(stale.stale, true);
  assert.deepEqual(stale.mismatches, ["snapshot.boundary.runtimeTakeoverBlocked"]);
});

test("Next/Turbopack Rust vendor boundary checker stays split into small modules", () => {
  const checkerPath = path.join(repoRoot, "tools", "vendor", "next-rust-boundary-check.js");
  const checkerLines = fs.readFileSync(checkerPath, "utf8").split(/\r?\n/).length;
  const modulePaths = [
    path.join(repoRoot, "tools", "vendor", "next-rust-boundary", "adapter-boundary.js"),
    path.join(repoRoot, "tools", "vendor", "next-rust-boundary", "active-scope.js"),
    path.join(repoRoot, "tools", "vendor", "next-rust-boundary", "claim-policy.js"),
    path.join(repoRoot, "tools", "vendor", "next-rust-boundary", "cli.js"),
    path.join(repoRoot, "tools", "vendor", "next-rust-boundary", "constants.js"),
    path.join(repoRoot, "tools", "vendor", "next-rust-boundary", "consumer-surfaces.js"),
    path.join(repoRoot, "tools", "vendor", "next-rust-boundary", "dx-check-projection.js"),
    path.join(repoRoot, "tools", "vendor", "next-rust-boundary", "editor-surface-projection.js"),
    path.join(repoRoot, "tools", "vendor", "next-rust-boundary", "paths.js"),
    path.join(repoRoot, "tools", "vendor", "next-rust-boundary", "public-claim-text.js"),
    path.join(repoRoot, "tools", "vendor", "next-rust-boundary", "public-dependencies.js"),
    path.join(repoRoot, "tools", "vendor", "next-rust-boundary", "public-source-exposure.js"),
    path.join(repoRoot, "tools", "vendor", "next-rust-boundary", "report-helpers.js"),
    path.join(repoRoot, "tools", "vendor", "next-rust-boundary", "receipt-compare.js"),
    path.join(repoRoot, "tools", "vendor", "next-rust-boundary", "status-surface.js"),
    path.join(repoRoot, "tools", "vendor", "next-rust-boundary", "vendored-cargo-dependencies.js"),
    path.join(repoRoot, "tools", "vendor", "next-rust-boundary", "vendor-source-inclusion.js"),
  ];

  assert.ok(checkerLines <= 430, `CLI boundary wrapper should stay small, found ${checkerLines} lines`);
  for (const modulePath of modulePaths) {
    assert.ok(fs.existsSync(modulePath), `missing split boundary module ${modulePath}`);
  }
});
