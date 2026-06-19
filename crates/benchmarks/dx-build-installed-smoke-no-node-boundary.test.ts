const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.join(__dirname, "..");

test("installed smoke scans nested node_modules directories as build violations", () => {
  const {
    diffNodeModulesDirs,
    findNodeModulesDirs,
  } = require("../tools/build/installed-smoke/no-node-modules.ts");
  const projectRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-installed-no-node-"));

  fs.mkdirSync(path.join(projectRoot, ".dx", "build", "source-routes", "root", "node_modules", "pkg"), {
    recursive: true,
  });
  fs.mkdirSync(path.join(projectRoot, "app", "node_modules", "trap"), { recursive: true });
  fs.mkdirSync(path.join(projectRoot, "target", "node_modules", "ignored"), { recursive: true });

  assert.deepEqual(findNodeModulesDirs(projectRoot), [
    ".dx/build/source-routes/root/node_modules",
    "app/node_modules",
  ]);
  assert.deepEqual(
    diffNodeModulesDirs(["app/node_modules"], [".dx/build/source-routes/root/node_modules", "app/node_modules"]),
    [".dx/build/source-routes/root/node_modules"],
  );
});

test("installed smoke treats node_modules junctions as build violations", () => {
  const { findNodeModulesDirs } = require("../tools/build/installed-smoke/no-node-modules.ts");
  const projectRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-installed-no-node-link-"));
  const externalNodeModules = path.join(projectRoot, "external-node-modules");
  fs.mkdirSync(externalNodeModules);
  fs.symlinkSync(externalNodeModules, path.join(projectRoot, "node_modules"), "junction");

  assert.deepEqual(findNodeModulesDirs(projectRoot), ["node_modules"]);
});

test("installed smoke report carries nested node_modules path evidence", () => {
  const cliSource = fs.readFileSync(
    path.join(repoRoot, "tools", "build", "installed-smoke", "cli.ts"),
    "utf8",
  );
  const reportSource = fs.readFileSync(
    path.join(repoRoot, "tools", "build", "installed-smoke", "report.ts"),
    "utf8",
  );
  const failuresSource = fs.readFileSync(
    path.join(repoRoot, "tools", "build", "installed-smoke", "build-receipt-failures.ts"),
    "utf8",
  );
  const humanReportSource = fs.readFileSync(
    path.join(repoRoot, "tools", "build", "installed-smoke", "human-report.ts"),
    "utf8",
  );
  const humanNodeModulesSource = fs.readFileSync(
    path.join(repoRoot, "tools", "build", "installed-smoke", "human-node-modules-boundary.ts"),
    "utf8",
  );

  assert.match(cliSource, /findNodeModulesDirs\(projectRoot\)/);
  assert.match(cliSource, /nodeModulesBeforePaths/);
  assert.match(cliSource, /nodeModulesCreatedPaths/);
  assert.match(cliSource, /nodeModulesPaths/);
  assert.match(reportSource, /nodeModulesBeforePaths:/);
  assert.match(reportSource, /nodeModulesCreatedPaths:/);
  assert.match(reportSource, /nodeModulesPaths:/);
  assert.match(failuresSource, /nodeModulesPaths/);
  assert.match(failuresSource, /dx build created node_modules in the fixture:/);
  assert.match(failuresSource, /fixture contains pre-existing node_modules before dx build:/);
  assert.match(humanReportSource, /printNodeModulesBoundary/);
  assert.doesNotMatch(humanReportSource, /function normalizePathList/);
  assert.match(humanNodeModulesSource, /function printNodeModulesBoundary/);
  assert.match(humanNodeModulesSource, /Node modules created by build/);
});

test("installed smoke human report names pre-existing and build-created node_modules paths", () => {
  const { printHumanReport } = require("../tools/build/installed-smoke/human-report.ts");
  const report = {
    passed: false,
    productProofRequired: true,
    productProofPassed: false,
    receiptPath: "G:\\Temp\\fixture\\.dx\\receipts\\build\\installed-binary-smoke-latest.json",
    proof: {
      scope: "installed-default",
      productEligible: false,
    },
    failures: [
      "fixture contains pre-existing node_modules before dx build: app/node_modules",
      "dx build created node_modules in the fixture: .dx/build/source-routes/root/node_modules",
    ],
    projectRoot: "G:\\Temp\\fixture",
    build: {
      nodeModulesPresent: true,
      nodeModulesCreated: true,
      nodeModulesBeforePaths: ["app/node_modules"],
      nodeModulesCreatedPaths: [".dx/build/source-routes/root/node_modules"],
      nodeModulesPaths: ["app/node_modules", ".dx/build/source-routes/root/node_modules"],
      command: {
        command: "G:\\Dx\\bin\\dx-www.exe",
        args: ["build"],
        exitCode: 0,
        stdoutTail: "",
        stderrTail: "",
      },
    },
    help: {
      command: {
        exitCode: 0,
      },
    },
  };

  const output = captureStdout(() => printHumanReport(report));

  assert.match(output, /Node modules: present/);
  assert.match(output, /Node modules before build: app\/node_modules/);
  assert.match(output, /Node modules created by build: \.dx\/build\/source-routes\/root\/node_modules/);
  assert.match(
    output,
    /Node modules after build: app\/node_modules, \.dx\/build\/source-routes\/root\/node_modules/,
  );
});

test("installed smoke rejects node_modules segments in manifest output paths", () => {
  const {
    isSafeManifestPath,
    publicAssetOutputPathMatchesSourceOwnedAssetPath,
  } = require("../tools/build/installed-smoke/manifest-output-paths.ts");

  assert.equal(isSafeManifestPath(".dx/build/public/icons/mark.svg"), true);
  assert.equal(isSafeManifestPath(".dx/build/node_modules/pkg/chunk.js"), false);
  assert.equal(isSafeManifestPath(".dx/build/Node_Modules/pkg/chunk.js"), false);
  assert.equal(isSafeManifestPath("public/node_modules/pkg/asset.svg"), false);
  assert.equal(
    publicAssetOutputPathMatchesSourceOwnedAssetPath(
      "public/node_modules/pkg/asset.svg",
      ".dx/build/public/node_modules/pkg/asset-abc123.svg",
      "abc123",
    ),
    false,
  );
});

test("installed smoke reports node_modules manifest path reasons", () => {
  const { manifestPathSafety } = require("../tools/build/installed-smoke/manifest-output-paths.ts");
  const { outputProofFailures } = require("../tools/build/installed-smoke/output-proof-failures.ts");
  const {
    summarizePublicAssetOutputSummary,
  } = require("../tools/build/installed-smoke/proof-output-public-assets.ts");
  const { summarizeStyleOutputSummary } = require("../tools/build/installed-smoke/proof-output-styles.ts");

  assert.deepEqual(manifestPathSafety(".dx/build/public/icons/mark.svg"), {
    normalizedPath: ".dx/build/public/icons/mark.svg",
    safe: true,
    unsafeReason: null,
  });
  assert.deepEqual(manifestPathSafety(".dx/build/node_modules/pkg/chunk.js"), {
    normalizedPath: null,
    safe: false,
    unsafeReason: "node-modules-segment",
  });
  assert.equal(manifestPathSafety("C:\\Temp\\asset.css").unsafeReason, "absolute-path");
  assert.equal(manifestPathSafety(".dx/../asset.css").unsafeReason, "parent-directory-segment");

  const styleSummary = summarizeStyleOutputSummary(validStyleOutput({
    outputPathSafe: false,
    outputPathUnsafeReason: "node-modules-segment",
  }));
  assert.ok(styleSummary.missingChecks.includes("style-output-path-safe"));
  assert.ok(styleSummary.missingChecks.includes("style-output-path-no-node-modules"));

  const publicAssetSummary = summarizePublicAssetOutputSummary(validPublicAssetOutput({
    outputPathSafe: false,
    outputPathUnsafeReason: "node-modules-segment",
  }));
  assert.ok(publicAssetSummary.missingChecks.includes("public-asset-output-path-safe"));
  assert.ok(publicAssetSummary.missingChecks.includes("public-asset-output-path-no-node-modules"));

  assert.deepEqual(outputProofFailures({
    outputProofSummary: {
      missingChecks: [
        "style-output-path-no-node-modules",
        "public-asset-output-path-no-node-modules",
      ],
    },
  }), [
    "source-build output proof failed: style-output-path-no-node-modules (stylesheet output path contains a node_modules segment)",
    "source-build output proof failed: public-asset-output-path-no-node-modules (public asset output path contains a node_modules segment)",
  ]);
});

test("installed smoke refuses explicit node_modules_required true output claims", () => {
  const { outputProofFailures } = require("../tools/build/installed-smoke/output-proof-failures.ts");
  const {
    summarizePublicAssetOutputSummary,
  } = require("../tools/build/installed-smoke/proof-output-public-assets.ts");
  const { summarizeStyleOutputSummary } = require("../tools/build/installed-smoke/proof-output-styles.ts");
  const { summarizeStyleOutputProof } = require("../tools/build/installed-smoke/proof-css-style-output.ts");
  const {
    summarizePublicAssetOutputProof,
  } = require("../tools/build/installed-smoke/proof-public-assets.ts");

  const styleProof = summarizeStyleOutputProof(validStyleOutput({
    declaresNoNodeModules: false,
    nodeModulesRequired: true,
  }));
  const publicAssetProof = summarizePublicAssetOutputProof(validPublicAssetOutput({
    declaresNoNodeModules: false,
    nodeModulesRequired: true,
  }));

  assert.equal(styleProof.nodeModulesRequired, true);
  assert.equal(publicAssetProof.nodeModulesRequired, true);

  const styleSummary = summarizeStyleOutputSummary(styleProof);
  const publicAssetSummary = summarizePublicAssetOutputSummary(publicAssetProof);
  assert.ok(styleSummary.missingChecks.includes("style-no-node-modules"));
  assert.ok(styleSummary.missingChecks.includes("style-node-modules-not-required"));
  assert.ok(publicAssetSummary.missingChecks.includes("public-asset-no-node-modules"));
  assert.ok(publicAssetSummary.missingChecks.includes("public-asset-node-modules-not-required"));

  assert.deepEqual(outputProofFailures({
    outputProofSummary: {
      missingChecks: [
        "style-node-modules-not-required",
        "public-asset-node-modules-not-required",
      ],
    },
  }), [
    "source-build output proof failed: style-node-modules-not-required (stylesheet output declares node_modules_required=true)",
    "source-build output proof failed: public-asset-node-modules-not-required (public asset output declares node_modules_required=true)",
  ]);
});

test("installed smoke fails source modules with explicit node_modules adapter boundaries", () => {
  const {
    sourceModuleResolverFailures,
    summarizeSourceModuleResolver,
  } = require("../tools/build/installed-smoke/source-module-resolver.ts");

  const summary = summarizeSourceModuleResolver({
    route_outputs: [
      {
        source_module_chunks: [
          {
            source_path: "app/page.tsx",
            diagnostics: 0,
            node_modules_required: false,
            dependencies: [
              {
                specifier: "../node_modules/trap",
                resolved_path: null,
                chunk_output: null,
                kind: "local-node-modules-adapter-boundary",
                resolver_source: "local-node-modules-boundary",
                resolver_detail: "local-import-node-modules-boundary",
                node_modules_required: false,
              },
              {
                specifier: "@/components/Hero",
                resolved_path: "components/Hero.tsx",
                chunk_output: ".dx/build/app/modules/components-hero.mjs",
                kind: "tsx",
                resolver_source: "project-root-alias",
                node_modules_required: false,
              },
            ],
          },
        ],
      },
    ],
  });

  assert.equal(summary.nodeModuleDependencyCount, 0);
  assert.equal(summary.nodeModuleBoundaryDependencyCount, 1);
  assert.deepEqual(summary.nodeModuleBoundaryDependencies, [
    {
      sourcePath: "app/page.tsx",
      specifier: "../node_modules/trap",
      resolverSource: "local-node-modules-boundary",
      resolverDetail: "local-import-node-modules-boundary",
    },
  ]);
  assert.deepEqual(sourceModuleResolverFailures(summary), [
    "source-build source module app/page.tsx dependency ../node_modules/trap crosses node_modules adapter boundary (local-import-node-modules-boundary)",
  ]);
});

test("installed smoke fails source-module adapter boundaries even with chunk outputs", () => {
  const {
    sourceModuleResolverFailures,
    summarizeSourceModuleResolver,
  } = require("../tools/build/installed-smoke/source-module-resolver.ts");

  const summary = summarizeSourceModuleResolver({
    route_outputs: [
      {
        source_module_chunks: [
          {
            source_path: "app/page.tsx",
            dependencies: [
              {
                specifier: "@vendor/widget",
                resolved_path: null,
                chunk_output: ".dx/build/app/modules/vendor-widget-boundary.mjs",
                kind: "package-node-modules-adapter-boundary",
                resolver_source: "package-node-modules-boundary",
                resolver_detail: "package-import-node-modules-boundary",
                node_modules_required: false,
              },
            ],
            diagnostics: 0,
            node_modules_required: false,
          },
        ],
      },
    ],
  });

  assert.equal(summary.nodeModuleDependencyCount, 0);
  assert.equal(summary.nodeModuleBoundaryDependencyCount, 1);
  assert.deepEqual(summary.nodeModuleBoundaryDependencies, [
    {
      sourcePath: "app/page.tsx",
      specifier: "@vendor/widget",
      resolverSource: "package-node-modules-boundary",
      resolverDetail: "package-import-node-modules-boundary",
    },
  ]);
  assert.deepEqual(sourceModuleResolverFailures(summary), [
    "source-build source module app/page.tsx dependency @vendor/widget crosses node_modules adapter boundary (package-import-node-modules-boundary)",
  ]);
});

test("installed smoke fails source modules with node_modules source or output paths", () => {
  const {
    sourceModuleResolverFailures,
    summarizeSourceModuleResolver,
  } = require("../tools/build/installed-smoke/source-module-resolver.ts");

  const summary = summarizeSourceModuleResolver({
    route_outputs: [
      {
        source_module_chunks: [
          {
            source_path: "node_modules/pkg/index.ts",
            chunk_output: ".dx/build/source-routes/root/modules/pkg-index.mjs",
            diagnostics: 0,
            dependencies: [],
            node_modules_required: false,
          },
          {
            source_path: "app/page.tsx",
            chunk_output: ".dx/build/node_modules/pkg/page.mjs",
            diagnostics: 0,
            dependencies: [],
            node_modules_required: false,
          },
        ],
      },
    ],
  });

  assert.equal(summary.nodeModuleModuleCount, 0);
  assert.equal(summary.nodeModulePathModuleCount, 2);
  assert.deepEqual(summary.nodeModulePathModules, [
    {
      sourcePath: "node_modules/pkg/index.ts",
      pathKind: "source_path",
      path: "node_modules/pkg/index.ts",
    },
    {
      sourcePath: "app/page.tsx",
      pathKind: "chunk_output",
      path: ".dx/build/node_modules/pkg/page.mjs",
    },
  ]);
  assert.deepEqual(sourceModuleResolverFailures(summary), [
    "source-build source module node_modules/pkg/index.ts has node_modules source_path node_modules/pkg/index.ts",
    "source-build source module app/page.tsx has node_modules chunk_output .dx/build/node_modules/pkg/page.mjs",
  ]);
});

test("installed smoke does not flag source names that merely mention node modules", () => {
  const {
    sourceModuleResolverFailures,
    summarizeSourceModuleResolver,
  } = require("../tools/build/installed-smoke/source-module-resolver.ts");

  const summary = summarizeSourceModuleResolver({
    route_outputs: [
      {
        source_module_chunks: [
          {
            source_path: "components/node-modules-guide.tsx",
            chunk_output: ".dx/build/app/modules/components-node-modules-guide.mjs",
            diagnostics: 0,
            dependencies: [
              {
                specifier: "./node-modules-helper",
                resolved_path: "components/node-modules-helper.ts",
                chunk_output: ".dx/build/app/modules/components-node-modules-helper.mjs",
                kind: "ts",
                resolver_source: "relative",
                node_modules_required: false,
              },
            ],
            node_modules_required: false,
          },
        ],
      },
    ],
  });

  assert.equal(summary.nodeModulePathModuleCount, 0);
  assert.equal(summary.nodeModuleDependencyCount, 0);
  assert.equal(summary.nodeModuleBoundaryDependencyCount, 0);
  assert.deepEqual(sourceModuleResolverFailures(summary), []);
});

test("installed smoke fails source-module dependencies with node_modules chunk outputs", () => {
  const {
    sourceModuleResolverFailures,
    summarizeSourceModuleResolver,
  } = require("../tools/build/installed-smoke/source-module-resolver.ts");

  const summary = summarizeSourceModuleResolver({
    route_outputs: [
      {
        source_module_chunks: [
          {
            source_path: "app/page.tsx",
            dependencies: [
              {
                specifier: "@/components/Hero",
                resolved_path: "components/Hero.tsx",
                chunk_output: ".dx/build/node_modules/pkg/hero.mjs",
                kind: "tsx",
                resolver_source: "project-root-alias",
                node_modules_required: false,
              },
            ],
            diagnostics: 0,
            node_modules_required: false,
          },
        ],
      },
    ],
  });

  assert.equal(summary.nodeModuleDependencyCount, 1);
  assert.deepEqual(summary.nodeModuleDependencies, [
    {
      sourcePath: "app/page.tsx",
      specifier: "@/components/Hero",
      resolvedPath: ".dx/build/node_modules/pkg/hero.mjs",
    },
  ]);
  assert.deepEqual(sourceModuleResolverFailures(summary), [
    "source-build source module app/page.tsx dependency @/components/Hero requires node_modules",
  ]);
});

function validStyleOutput(overrides = {}) {
  return {
    present: true,
    sourcePath: "app/global.css",
    sourceOutputPath: ".dx/build/app/global.css",
    outputPath: ".dx/build/app/global.css",
    outputPathSafe: true,
    sourceOutputPathMatchesOutput: true,
    hasHash: true,
    hashMatchesOutput: true,
    sourceMapPresent: true,
    sourceMapOutputPathSafe: true,
    sourceMapLinked: true,
    sourceMapLinkedInCss: true,
    hasSourceMapHash: true,
    sourceMapHashMatchesArtifact: true,
    sourceMapSourceCount: 1,
    sourceMapJsonValid: true,
    sourceMapHasSources: true,
    declaresNoNodeModules: true,
    nodeModulesRequired: false,
    lifecycleScriptsExecuted: false,
    sourceOwnedContract: true,
    externalRuntimeRequired: false,
    externalRuntimeExecuted: false,
    ...overrides,
  };
}

function validPublicAssetOutput(overrides = {}) {
  return {
    present: true,
    sourcePath: "public/icon.svg",
    sourceOutputPath: ".dx/build/public/icon.svg",
    outputPath: ".dx/build/public/icon-abc123.svg",
    outputPathSafe: true,
    sourcePathIsPublicAsset: true,
    outputPathIsPublicAsset: true,
    outputPathMatchesSourceOwnedAssetPath: true,
    hasHash: true,
    hashMatchesOutput: true,
    outputFileNameContainsHash: true,
    declaresNoNodeModules: true,
    nodeModulesRequired: false,
    lifecycleScriptsExecuted: false,
    sourceOwnedContract: true,
    externalRuntimeRequired: false,
    externalRuntimeExecuted: false,
    sizeMatchesOutput: true,
    ...overrides,
  };
}

function captureStdout(callback) {
  let output = "";
  const originalWrite = process.stdout.write;
  process.stdout.write = (chunk, encoding, callbackArg) => {
    output += String(chunk);
    if (typeof callbackArg === "function") {
      callbackArg();
    } else if (typeof encoding === "function") {
      encoding();
    }
    return true;
  };

  try {
    callback();
  } finally {
    process.stdout.write = originalWrite;
  }

  return output;
}
