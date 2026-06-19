import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { execFileSync, spawnSync } = require("node:child_process");
const test = require("node:test");
const {
  buildNextRustVendorBoundaryActiveScope,
  buildNextRustVendorBoundaryConsumerSurfaces,
  buildNextRustVendorBoundaryStatusSurface,
  writeNextRustVendorBoundaryConsumerReceipt,
  writeNextRustVendorBoundaryReceipt,
} = require("../tools/vendor/next-rust-boundary-check.js");

const repoRoot = path.resolve(__dirname, "..");
const checkerPath = path.join(repoRoot, "tools", "vendor", "next-rust-boundary-check.js");
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

function writeText(filePath, text) {
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, text, "utf8");
}

function materializeVendorBoundaryFixture() {
  const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-next-rust-active-scope-"));
  writeText(path.join(tempRoot, "Cargo.toml"), "");
  writeText(path.join(tempRoot, "dx-www", "Cargo.toml"), "");
  writeText(path.join(tempRoot, "vendor", "next-rust", "README.md"), readme);
  writeText(path.join(tempRoot, "vendor", "next-rust", "license.nextjs.md"), license);
  writeText(path.join(tempRoot, "dx-www", "src", "next_rust.rs"), rustSource);

  for (const importedGroup of importedRustGroups) {
    writeText(
      path.join(tempRoot, "vendor", "next-rust", ...importedGroup.split("/"), "Cargo.toml"),
      "[package]\nname = \"fixture\"\nversion = \"0.0.0\"\n",
    );
  }

  return tempRoot;
}

test("active-scope CLI projects the lane-10 scope without the full status surface", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();
  writeNextRustVendorBoundaryReceipt(fixtureRoot);
  writeNextRustVendorBoundaryConsumerReceipt(fixtureRoot);

  const directScope = buildNextRustVendorBoundaryActiveScope(fixtureRoot);
  const cliScope = JSON.parse(
    execFileSync(process.execPath, [checkerPath, "--repo-root", fixtureRoot, "--active-scope"], {
      encoding: "utf8",
    }),
  );

  assert.deepEqual(cliScope, directScope);
  assert.equal(cliScope.schema, "dx.nextRust.vendorBoundary.activeScope");
  assert.equal(cliScope.status, "ok");
  assert.equal(cliScope.referenceOnlyNextRust, true);
  assert.equal(cliScope.runtimeBuildAdoption, false);
  assert.equal(cliScope.turbopackPublicArchitecture, false);
  assert.equal(cliScope.devFeedbackEndpoint, "/_dx/feedback");
  assert.deepEqual(cliScope.removedTargets, [
    "Next DevTools clone",
    "DX-WWW DevTools",
    "/_dx/devtools",
    "external DevTools runtime",
  ]);
  assert.deepEqual(cliScope.excludedRuntimeTargets, [
    "Turbopack runtime/build adoption",
    "external bundler execution proof",
    "Turbopack powers dx build/dev",
  ]);
  assert.equal(cliScope.dxCheck, undefined);
  assert.equal(cliScope.editorSurfaces, undefined);
  assert.equal(cliScope.evidence, undefined);
  assert.equal(cliScope.markers, undefined);
});

test("active-scope CLI blocks stale receipt state without reviving removed targets", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();
  writeNextRustVendorBoundaryReceipt(fixtureRoot);
  const receipt = writeNextRustVendorBoundaryConsumerReceipt(fixtureRoot);
  const staleReceipt = JSON.parse(fs.readFileSync(receipt.receiptPath, "utf8"));
  staleReceipt.snapshot.boundary.publicSourceExposureBlocked = false;
  fs.writeFileSync(receipt.receiptPath, `${JSON.stringify(staleReceipt, null, 2)}\n`, "utf8");

  const result = spawnSync(
    process.execPath,
    [checkerPath, "--repo-root", fixtureRoot, "--active-scope"],
    { encoding: "utf8" },
  );
  const scope = JSON.parse(result.stdout);

  assert.equal(result.status, 1);
  assert.equal(scope.schema, "dx.nextRust.vendorBoundary.activeScope");
  assert.equal(scope.status, "blocked");
  assert.equal(scope.removedTargetsBlocked, true);
  assert.equal(scope.excludedRuntimeTargetsBlocked, true);
  assert.equal(scope.runtimeBuildAdoption, false);
  assert.equal(scope.turbopackPublicArchitecture, false);
});

test("consumer surfaces expose compact active-scope mismatch summary", () => {
  const cleanRoot = materializeVendorBoundaryFixture();
  writeNextRustVendorBoundaryReceipt(cleanRoot);
  writeNextRustVendorBoundaryConsumerReceipt(cleanRoot);

  const cleanSurfaces = buildNextRustVendorBoundaryConsumerSurfaces(cleanRoot);
  assert.deepEqual(cleanSurfaces.activeScopeSummary, {
    schema: "dx.nextRust.vendorBoundary.activeScopeSummary",
    status: "ok",
    scopeStatus: "ok",
    mismatchCount: 0,
    mismatches: [],
  });

  const cases = [
    {
      text: "Restore /_dx/devtools as the DX-WWW DevTools surface.",
      expectedMismatch: {
        id: "removed-devtools-targets",
        label: "DevTools targets removed",
        publicClaimChecks: ["next-devtools-clone-target", "dx-devtools-removed-target"],
      },
    },
    {
      text: "Turbopack powers dx build and dx dev in the production path.",
      expectedMismatch: {
        id: "turbopack-runtime-adoption",
        label: "Turbopack adoption blocked",
        publicClaimChecks: [
          "turbopack-runtime-build-adoption",
          "external-bundler-execution-proof-target",
        ],
      },
    },
  ];

  for (const testCase of cases) {
    const fixtureRoot = materializeVendorBoundaryFixture();
    writeText(path.join(fixtureRoot, "TODO.md"), `# TODO\n${testCase.text}\n`);
    writeNextRustVendorBoundaryReceipt(fixtureRoot);
    writeNextRustVendorBoundaryConsumerReceipt(fixtureRoot);

    const surfaces = buildNextRustVendorBoundaryConsumerSurfaces(fixtureRoot);
    assert.equal(surfaces.status, "blocked");
    assert.deepEqual(surfaces.activeScopeSummary, {
      schema: "dx.nextRust.vendorBoundary.activeScopeSummary",
      status: "fail",
      scopeStatus: "blocked",
      mismatchCount: 1,
      mismatches: [testCase.expectedMismatch],
    });
  }
});

test("dx-check active-scope evidence exposes the compact mismatch summary", () => {
  const cleanRoot = materializeVendorBoundaryFixture();
  writeNextRustVendorBoundaryReceipt(cleanRoot);
  writeNextRustVendorBoundaryConsumerReceipt(cleanRoot);

  const cleanSurface = buildNextRustVendorBoundaryStatusSurface(cleanRoot);
  const cleanActiveScopeCheck = cleanSurface.dxCheck.checks.find(
    (check) => check.id === "next-rust.vendor-boundary.active-scope",
  );
  assert.deepEqual(cleanActiveScopeCheck.evidence.activeScopeSummary, {
    schema: "dx.nextRust.vendorBoundary.activeScopeSummary",
    status: "ok",
    scopeStatus: "ok",
    mismatchCount: 0,
    mismatches: [],
  });

  const cases = [
    {
      text: "Restore /_dx/devtools as the DX-WWW DevTools surface.",
      expectedMismatch: {
        id: "removed-devtools-targets",
        label: "DevTools targets removed",
        publicClaimChecks: ["next-devtools-clone-target", "dx-devtools-removed-target"],
      },
    },
    {
      text: "Turbopack powers dx build and dx dev in the production path.",
      expectedMismatch: {
        id: "turbopack-runtime-adoption",
        label: "Turbopack adoption blocked",
        publicClaimChecks: [
          "turbopack-runtime-build-adoption",
          "external-bundler-execution-proof-target",
        ],
      },
    },
  ];

  for (const testCase of cases) {
    const fixtureRoot = materializeVendorBoundaryFixture();
    writeText(path.join(fixtureRoot, "TODO.md"), `# TODO\n${testCase.text}\n`);
    writeNextRustVendorBoundaryReceipt(fixtureRoot);
    writeNextRustVendorBoundaryConsumerReceipt(fixtureRoot);

    const surface = buildNextRustVendorBoundaryStatusSurface(fixtureRoot);
    const activeScopeCheck = surface.dxCheck.checks.find(
      (check) => check.id === "next-rust.vendor-boundary.active-scope",
    );

    assert.equal(surface.status, "blocked");
    assert.deepEqual(activeScopeCheck.evidence.activeScopeSummary, {
      schema: "dx.nextRust.vendorBoundary.activeScopeSummary",
      status: "fail",
      scopeStatus: "blocked",
      mismatchCount: 1,
      mismatches: [testCase.expectedMismatch],
    });
  }
});

test("status surface exposes top-level active-scope mismatch summary", () => {
  const cleanRoot = materializeVendorBoundaryFixture();
  writeNextRustVendorBoundaryReceipt(cleanRoot);
  writeNextRustVendorBoundaryConsumerReceipt(cleanRoot);

  const cleanSurface = buildNextRustVendorBoundaryStatusSurface(cleanRoot);
  assert.deepEqual(cleanSurface.activeScopeSummary, {
    schema: "dx.nextRust.vendorBoundary.activeScopeSummary",
    status: "ok",
    scopeStatus: "ok",
    mismatchCount: 0,
    mismatches: [],
  });

  const cases = [
    {
      text: "Restore /_dx/devtools as the DX-WWW DevTools surface.",
      expectedMismatch: {
        id: "removed-devtools-targets",
        label: "DevTools targets removed",
        publicClaimChecks: ["next-devtools-clone-target", "dx-devtools-removed-target"],
      },
    },
    {
      text: "Turbopack powers dx build and dx dev in the production path.",
      expectedMismatch: {
        id: "turbopack-runtime-adoption",
        label: "Turbopack adoption blocked",
        publicClaimChecks: [
          "turbopack-runtime-build-adoption",
          "external-bundler-execution-proof-target",
        ],
      },
    },
  ];

  for (const testCase of cases) {
    const fixtureRoot = materializeVendorBoundaryFixture();
    writeText(path.join(fixtureRoot, "TODO.md"), `# TODO\n${testCase.text}\n`);
    writeNextRustVendorBoundaryReceipt(fixtureRoot);
    writeNextRustVendorBoundaryConsumerReceipt(fixtureRoot);

    const surface = buildNextRustVendorBoundaryStatusSurface(fixtureRoot);
    assert.deepEqual(surface.activeScopeSummary, {
      schema: "dx.nextRust.vendorBoundary.activeScopeSummary",
      status: "fail",
      scopeStatus: "blocked",
      mismatchCount: 1,
      mismatches: [testCase.expectedMismatch],
    });
  }
});

test("editor surfaces expose active-scope badges for removed targets", () => {
  const fixtureRoot = materializeVendorBoundaryFixture();
  writeNextRustVendorBoundaryReceipt(fixtureRoot);
  writeNextRustVendorBoundaryConsumerReceipt(fixtureRoot);

  const surface = buildNextRustVendorBoundaryStatusSurface(fixtureRoot);

  for (const editorSurface of surface.editorSurfaces.surfaces) {
    assert.deepEqual(
      editorSurface.badges.slice(-2),
      [
        { label: "DevTools targets removed", status: "ok" },
        { label: "Turbopack adoption blocked", status: "ok" },
      ],
    );
    assert.equal(editorSurface.activeScope.schema, "dx.nextRust.vendorBoundary.activeScope");
    assert.equal(editorSurface.activeScope.removedTargetsBlocked, true);
    assert.equal(editorSurface.activeScope.excludedRuntimeTargetsBlocked, true);
    assert.equal(editorSurface.activeScope.runtimeBuildAdoption, false);
    assert.equal(editorSurface.activeScope.turbopackPublicArchitecture, false);
  }
});

test("editor active-scope badges fail for reintroduced public target claims", () => {
  const cases = [
    {
      text: "Restore /_dx/devtools as the DX-WWW DevTools surface.",
      expectedBadges: [
        { label: "DevTools targets removed", status: "fail" },
        { label: "Turbopack adoption blocked", status: "ok" },
      ],
      expectedRemovedTargetsBlocked: false,
      expectedExcludedRuntimeTargetsBlocked: true,
    },
    {
      text: "Turbopack powers dx build and dx dev in the production path.",
      expectedBadges: [
        { label: "DevTools targets removed", status: "ok" },
        { label: "Turbopack adoption blocked", status: "fail" },
      ],
      expectedRemovedTargetsBlocked: true,
      expectedExcludedRuntimeTargetsBlocked: false,
    },
  ];

  for (const testCase of cases) {
    const fixtureRoot = materializeVendorBoundaryFixture();
    writeText(path.join(fixtureRoot, "TODO.md"), `# TODO\n${testCase.text}\n`);
    writeNextRustVendorBoundaryReceipt(fixtureRoot);
    writeNextRustVendorBoundaryConsumerReceipt(fixtureRoot);

    const surface = buildNextRustVendorBoundaryStatusSurface(fixtureRoot);
    const zedSurface = surface.editorSurfaces.surfaces[0];

    assert.equal(surface.status, "blocked");
    assert.deepEqual(zedSurface.badges.slice(-2), testCase.expectedBadges);
    assert.equal(
      zedSurface.activeScope.removedTargetsBlocked,
      testCase.expectedRemovedTargetsBlocked,
    );
    assert.equal(
      zedSurface.activeScope.excludedRuntimeTargetsBlocked,
      testCase.expectedExcludedRuntimeTargetsBlocked,
    );
  }
});
