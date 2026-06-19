import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const test = require("node:test");

const {
  printHumanReport,
  smokeCommandPassed,
} = require("../tools/build/installed-smoke/human-report.ts");

test("installed smoke human report marks require-product candidate smoke as command failure", () => {
  const report = {
    passed: true,
    productProofRequired: true,
    productProofPassed: false,
    binaryRole: "candidate-override",
    binaryDefault: "G:\\Dx\\bin\\dx-www.exe",
    binaryOverride: true,
    binaryIdentity: {
      path: "G:\\Dx\\www\\target\\debug\\dx-www.exe",
      present: true,
      kind: "file",
      byteLength: 123456,
      sha256: "abc123",
    },
    receiptPath: "G:\\Temp\\fixture\\.dx\\receipts\\build\\installed-binary-smoke-latest.json",
    proof: {
      scope: "candidate-override",
      productEligible: false,
      nextAction: "Promote this candidate through a governed install step, then rerun without --binary.",
    },
    failures: [],
    projectRoot: "G:\\Temp\\fixture",
    build: {
      command: {
        command: "node",
        args: ["G:\\Dx\\www\\target\\debug\\dx-www.exe", "build"],
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

  assert.equal(smokeCommandPassed(report), false);
  const output = captureStdout(() => printHumanReport(report));

  assert.match(output, /Command result: failed \(product evidence required\)/);
  assert.match(output, /DX build installed-binary smoke: passed/);
  assert.match(output, /Product proof: not eligible \(candidate-override\)/);
  assert.match(output, /Product next action: Promote this candidate through a governed install step/);
  assert.match(output, /Binary role: candidate-override/);
  assert.match(output, /Default binary: G:\\Dx\\bin\\dx-www\.exe/);
  assert.match(output, /Binary path: G:\\Dx\\www\\target\\debug\\dx-www\.exe/);
  assert.match(output, /Binary status: file \(123456 bytes\)/);
  assert.match(output, /Binary sha256: abc123/);
  assert.match(output, /Receipt: G:\\Temp\\fixture\\.dx\\receipts\\build\\installed-binary-smoke-latest\.json/);
  assert.match(output, /Fixture: G:\\Temp\\fixture/);
});

test("installed smoke human report explains missing installed binary identity", () => {
  const report = {
    passed: false,
    productProofRequired: true,
    productProofPassed: false,
    binaryRole: "installed-default",
    binaryDefault: "G:\\Dx\\bin\\dx-www.exe",
    binaryIdentity: {
      path: "G:\\Dx\\bin\\dx-www.exe",
      present: false,
      kind: null,
      byteLength: null,
      sha256: null,
      error: "ENOENT",
    },
    receiptPath: "G:\\Temp\\fixture\\.dx\\receipts\\build\\installed-binary-smoke-latest.json",
    proof: {
      scope: "installed-default",
      productEligible: false,
      nextAction: "Fix any listed failures, then rerun the installed-default smoke.",
    },
    failures: ["dx build binary is missing or not executable"],
    projectRoot: "G:\\Temp\\fixture",
    build: {
      command: {
        command: "G:\\Dx\\bin\\dx-www.exe",
        args: ["build"],
        exitCode: 1,
        stdoutTail: "",
        stderrTail: "spawn G:\\Dx\\bin\\dx-www.exe ENOENT",
      },
    },
    help: {
      command: {
        exitCode: 1,
      },
    },
  };

  const output = captureStdout(() => printHumanReport(report));

  assert.match(output, /Command result: failed/);
  assert.match(output, /Binary role: installed-default/);
  assert.match(output, /Binary path: G:\\Dx\\bin\\dx-www\.exe/);
  assert.match(output, /Binary status: missing \(ENOENT\)/);
  assert.match(output, /- dx build binary is missing or not executable/);
  assert.match(output, /Build stderr tail:[\s\S]*spawn G:\\Dx\\bin\\dx-www\.exe ENOENT/);
});

test("installed smoke human report prints help diagnostics for stale help contract", () => {
  const report = {
    passed: false,
    productProofRequired: true,
    productProofPassed: false,
    binaryRole: "installed-default",
    binaryDefault: "G:\\Dx\\bin\\dx-www.exe",
    binaryIdentity: {
      path: "G:\\Dx\\bin\\dx-www.exe",
      present: true,
      kind: "file",
      byteLength: 123456,
      sha256: "abc123",
    },
    receiptPath: "G:\\Temp\\fixture\\.dx\\receipts\\build\\installed-binary-smoke-latest.json",
    proof: {
      scope: "installed-default",
      productEligible: false,
      nextAction: "Fix any listed failures, then rerun the installed-default smoke.",
    },
    failures: [
      "dx www build --help wrote project output",
      "dx www build --help did not describe the source-owned no-node_modules contract",
      "dx www build --help did not describe the Android build target",
    ],
    projectRoot: "G:\\Temp\\fixture",
    build: {
      command: {
        command: "G:\\Dx\\bin\\dx-www.exe",
        args: ["build"],
        exitCode: 0,
        stdoutTail: "",
        stderrTail: "Build complete",
      },
    },
    help: {
      readOnly: false,
      sourceOwnedContractVisible: false,
      androidTargetVisible: false,
      command: {
        command: "G:\\Dx\\bin\\dx-www.exe",
        args: ["www", "build", "--help"],
        exitCode: 0,
        stdoutTail: "",
        stderrTail: "Building for production...\n? Build complete",
      },
    },
  };

  const output = captureStdout(() => printHumanReport(report));

  assert.match(output, /- dx www build --help wrote project output/);
  assert.match(output, /- dx www build --help did not describe the source-owned no-node_modules contract/);
  assert.match(output, /- dx www build --help did not describe the Android build target/);
  assert.match(output, /Help command: G:\\Dx\\bin\\dx-www\.exe www build --help/);
  assert.match(output, /Help exit code: 0/);
  assert.match(output, /Help stderr tail:[\s\S]*Building for production/);
});

test("installed smoke human report surfaces CSS and asset proof eligibility", () => {
  const report = {
    passed: true,
    productProofRequired: true,
    productProofPassed: false,
    binaryRole: "installed-default",
    binaryDefault: "G:\\Dx\\bin\\dx-www.exe",
    binaryIdentity: {
      path: "G:\\Dx\\bin\\dx-www.exe",
      present: true,
      kind: "file",
      byteLength: 123456,
      sha256: "abc123",
    },
    receiptPath: "G:\\Temp\\fixture\\.dx\\receipts\\build\\installed-binary-smoke-latest.json",
    proof: {
      scope: "installed-default",
      productEligible: false,
      nextAction: "Fix emitted stylesheet source-map and public asset hash evidence, then rerun the installed-default smoke.",
      cssAssetOutputProof: {
        required: true,
        styleOutput: {
          eligible: false,
          path: ".dx/build/styles/app.css",
          sourceMapPath: ".dx/build/styles/app.css.map",
          hasHash: true,
          sourceMapPresent: true,
          sourceMapLinked: true,
          sourceMapLinkedInCss: false,
          hasSourceMapHash: true,
          sourceMapHash: "map123",
          sourceMapJsonValid: true,
          sourceMapSourceCount: 0,
          sourceMapArtifactSourceCount: 1,
          declaresNoNodeModules: true,
          lifecycleScriptsExecuted: false,
          sourceOwnedContract: true,
          externalRuntimeRequired: false,
          externalRuntimeExecuted: false,
        },
        publicAssetOutput: {
          eligible: true,
          path: ".dx/build/public/icons/mark-abc123.svg",
          hasHash: true,
          outputFileNameContainsHash: true,
          sourcePathIsPublicAsset: true,
          outputPathIsPublicAsset: true,
          outputPathMatchesSourceOwnedAssetPath: true,
          declaresNoNodeModules: true,
          lifecycleScriptsExecuted: false,
          sourceOwnedContract: true,
          externalRuntimeRequired: false,
          externalRuntimeExecuted: false,
          sizeMatchesOutput: true,
          size: 7,
          outputByteLength: 7,
        },
      },
    },
    failures: [],
    projectRoot: "G:\\Temp\\fixture",
    build: {
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

  assert.match(output, /Command result: failed \(product evidence required\)/);
  assert.match(
    output,
    /CSS output proof: not eligible \(\.dx\/build\/styles\/app\.css; hash: yes; source map: present; linked: yes; linked in CSS: no; source-map hash: map123; source-map JSON: yes; sources: 0; artifact sources: 1; no node_modules: yes; lifecycle scripts executed: no; source-owned: yes; external runtime required: no; external runtime executed: no\)/,
  );
  assert.match(
    output,
    /Public asset proof: eligible \(\.dx\/build\/public\/icons\/mark-abc123\.svg; hash: yes; hashed filename: yes; public source: yes; public output: yes; source-derived: yes; no node_modules: yes; lifecycle scripts executed: no; source-owned: yes; external runtime required: no; external runtime executed: no; size: 7\/7 bytes; size match: yes\)/,
  );
});

test("installed smoke human report surfaces route-handler readiness", () => {
  const report = {
    passed: true,
    productProofRequired: true,
    productProofPassed: true,
    binaryRole: "installed-default",
    binaryDefault: "G:\\Dx\\bin\\dx-www.exe",
    binaryIdentity: {
      path: "G:\\Dx\\bin\\dx-www.exe",
      present: true,
      kind: "file",
      byteLength: 123456,
      sha256: "abc123",
    },
    receiptPath: "G:\\Temp\\fixture\\.dx\\receipts\\build\\installed-binary-smoke-latest.json",
    proof: {
      scope: "installed-default",
      productEligible: true,
    },
    failures: [],
    projectRoot: "G:\\Temp\\fixture",
    build: {
      sourceBuild: {
        routeHandlerReceipt: {
          routeHandlerReadiness: [
            {
              sourcePath: "app/api/health/route.ts",
              route: "/api/health",
              method: "GET",
              buildStatus: "executed",
              expectedStatus: 200,
              responseStatus: 200,
              skipReason: null,
              duplicateCount: 0,
              sourceOwnedRuntimeBoundary: true,
              externalRuntimeRequired: false,
              externalRuntimeExecuted: false,
              declaresNoNodeModules: true,
              lifecycleScriptsExecuted: false,
            },
            {
              sourcePath: "app/api/checkout/route.ts",
              route: "/api/checkout",
              method: "POST",
              buildStatus: "skipped",
              expectedStatus: 202,
              responseStatus: null,
              skipReason: "unsafe-build-request-body",
              duplicateCount: 0,
              sourceOwnedRuntimeBoundary: null,
              externalRuntimeRequired: null,
              externalRuntimeExecuted: null,
              declaresNoNodeModules: null,
              lifecycleScriptsExecuted: null,
            },
          ],
        },
      },
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

  assert.match(output, /Route handlers:/);
  assert.match(
    output,
    /- GET \/api\/health \(app\/api\/health\/route\.ts\): executed; status 200\/200; source-owned: yes; no node_modules: yes/,
  );
  assert.match(
    output,
    /- POST \/api\/checkout \(app\/api\/checkout\/route\.ts\): skipped \(unsafe-build-request-body\); expected status 202/,
  );
});

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
