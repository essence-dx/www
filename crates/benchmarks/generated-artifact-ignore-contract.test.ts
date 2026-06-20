import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");
const gitignorePath = path.join(repoRoot, ".gitignore");

const GENERATED_LOCAL_ARTIFACTS = [
  {
    path: ".tmp/agent25-pass3-installed-smoke.json",
    reason: "worker scratch output",
    expectedPattern: ".tmp/",
  },
  {
    path: ".cache/dx-www-local-cache/index.json",
    reason: "tool-local cache output",
    expectedPattern: ".cache/",
  },
  {
    path: ".dx/launch/session.json",
    reason: "local dev-server process log",
    expectedPattern: ".dx/launch/",
  },
  {
    path: ".dx/codex-localhost-proof/dx-dev-3000.out.log",
    reason: "local HTTP proof server log",
    expectedPattern: "*.out.log",
  },
  {
    path: ".dx/codex-localhost-proof/dx-dev-3000.err.log",
    reason: "local HTTP proof server error log",
    expectedPattern: "*.err.log",
  },
  {
    path: ".dx/receipts/build/installed-binary-smoke-latest.json",
    reason: "machine-local latest pointer",
    expectedPattern: ".dx/receipts/build/installed-binary-smoke-latest.json",
  },
  {
    path: ".dx/www/forge-package-status.machine",
    reason: "validated machine cache for Forge package-status JSON",
    expectedPattern: ".dx/www/*.machine",
  },
  {
    path: ".dx/www/forge-package-status.machine.meta.json",
    reason: "typed machine cache metadata for Forge package-status JSON",
    expectedPattern: ".dx/www/*.machine.meta.json",
  },
  {
    path: ".dx/performance/json-machine-cache-receipts/forge-package-status.json",
    reason: "local performance receipt for JSON-to-machine cache comparison",
    expectedPattern: ".dx/performance/",
  },
  {
    path: ".dx/performance/json-machine-cache-receipts/media-icon-raw-index.json",
    reason: "local performance receipt for media-icon raw index cache comparison",
    expectedPattern: ".dx/performance/",
  },
  {
    path: ".dx/performance/json-machine-cache-receipts/media-icon-engine-startup.json",
    reason: "local performance receipt for media-icon engine startup timing",
    expectedPattern: ".dx/performance/",
  },
  {
    path: ".dx/performance/json-machine-cache-receipts/media-icon-existing-cache-readiness.json",
    reason: "local readiness receipt for media-icon existing machine-cache proof",
    expectedPattern: ".dx/performance/",
  },
  {
    path: ".dx/icon/machine/v1/manifest.machine",
    reason: "typed machine cache for media-icon source manifest",
    expectedPattern: ".dx/icon/",
  },
  {
    path: ".dx/icon/machine/v1/manifest.machine.meta.json",
    reason: "typed machine cache metadata for media-icon source manifest",
    expectedPattern: ".dx/icon/",
  },
  {
    path: ".dx/icon/machine/v1/catalog.machine",
    reason: "typed machine cache for media-icon catalog entries",
    expectedPattern: ".dx/icon/",
  },
  {
    path: ".dx/icon/machine/v1/catalog.machine.meta.json",
    reason: "typed machine cache metadata for media-icon catalog entries",
    expectedPattern: ".dx/icon/",
  },
  {
    path: ".dx/icon/machine/v1/prefix.machine",
    reason: "typed machine cache for media-icon prefix candidates",
    expectedPattern: ".dx/icon/",
  },
  {
    path: ".dx/icon/machine/v1/prefix.machine.meta.json",
    reason: "typed machine cache metadata for media-icon prefix candidates",
    expectedPattern: ".dx/icon/",
  },
  {
    path: ".dx/icon/machine/v1/perfect-hash.machine",
    reason: "typed machine cache for media-icon perfect hash slots",
    expectedPattern: ".dx/icon/",
  },
  {
    path: ".dx/icon/machine/v1/perfect-hash.machine.meta.json",
    reason: "typed machine cache metadata for media-icon perfect hash slots",
    expectedPattern: ".dx/icon/",
  },
  {
    path: ".dx/icon/machine/v1/bloom.machine",
    reason: "typed machine cache for media-icon bloom filters",
    expectedPattern: ".dx/icon/",
  },
  {
    path: ".dx/icon/machine/v1/bloom.machine.meta.json",
    reason: "typed machine cache metadata for media-icon bloom filters",
    expectedPattern: ".dx/icon/",
  },
  {
    path: ".dx/icon/machine/v1/lowercase-cache.machine",
    reason: "typed machine cache for media-icon lowercase names",
    expectedPattern: ".dx/icon/",
  },
  {
    path: ".dx/icon/machine/v1/lowercase-cache.machine.meta.json",
    reason: "typed machine cache metadata for media-icon lowercase names",
    expectedPattern: ".dx/icon/",
  },
  {
    path: ".dx/icon/machine/v1/pack-body.machine",
    reason: "typed machine cache for media-icon pack SVG bodies",
    expectedPattern: ".dx/icon/",
  },
  {
    path: ".dx/icon/machine/v1/pack-body.machine.meta.json",
    reason: "typed machine cache metadata for media-icon pack SVG bodies",
    expectedPattern: ".dx/icon/",
  },
  {
    path: ".dx/template-app-browser-preview/server.pid",
    reason: "local preview server process id",
    expectedPattern: "*.pid",
  },
  {
    path: "NUL",
    reason: "Windows reserved device-name status noise",
    expectedPattern: "/[Nn][Uu][Ll]",
  },
  {
    path: "worker-lanes/state/www-worker.json",
    reason: "local worker lane bookkeeping",
    expectedPattern: "worker-lanes/state/",
  },
  {
    path: "target-codex-readiness-gate/tmp/proof.log",
    reason: "local readiness-gate target output",
    expectedPattern: "target-*/",
  },
  {
    path: "examples/template/.dx/build/.dx/build-cache/manifest.json",
    reason: "rebuildable www-template output",
    expectedPattern: "examples/template/.dx/build/",
  },
  {
    path: "examples/template/.dx/receipts/build/installed-binary-smoke-latest.json",
    reason: "www-template local build receipt",
    expectedPattern: "examples/template/.dx/receipts/build/",
  },
  {
    path: "examples/template/.dx/receipts/graph/latest.json",
    reason: "www-template local graph receipt",
    expectedPattern: "examples/template/.dx/receipts/graph/",
  },
  {
    path: "examples/charts/.vercel/project.json",
    reason: "charts local Vercel deployment link",
    expectedPattern: "/.vercel/",
  },
  {
    path: "examples/charts/.dx/www/output/.dx/build-cache/manifest.json",
    reason: "charts rebuildable www output",
    expectedPattern: "/.dx/www/output/",
  },
  {
    path: "examples/charts/.dx/receipts/build/latest.json",
    reason: "charts local build receipt",
    expectedPattern: "/.dx/receipts/build/",
  },
];

const SOURCE_OWNED_TRACKED_ARTIFACTS = [
  ".dx/receipts/build/latest.json",
  ".dx/receipts/deploy/deploy-plan-latest.json",
  ".dx/receipts/graph/latest.json",
  ".dx/receipts/style/build.json",
  ".dx/template-app-browser-preview/server.cjs",
  "benchmarks/reports/latest.json",
  "cache/Cargo.toml",
  "examples/template/.dx/forge/package-status.json",
  "examples/charts/.dx/receipts/check/check-latest.json",
  "examples/charts/.dx/receipts/icons/check.json",
  "examples/charts/.dx/receipts/imports/check.json",
  "examples/charts/.dx/receipts/style/check.json",
  "related-crates/style/src/cache/mod.rs",
  "tools/build/installed-smoke/proof.ts",
];

const SOURCE_OWNED_VISIBLE_HELPERS = [
  "start-www-worker.ps1",
  "worker-lanes/README.md",
  "worker-lanes/WWW_30_AGENT_AUTO_LANE_PROMPT.md",
  "worker-lanes/WWW_30_AGENT_FINAL_POLISH_PROMPT.md",
  "worker-lanes/claim-www-lane.ps1",
  "benchmarks/test-format-standard.test.ts",
  "dx-www/src/build/source_engine/route_paths.rs",
  "dx-www/src/build/source_engine/server_data.rs",
  "dx-www/src/cli/build_command.rs",
  "dx-www/src/cli/build_options.rs",
  "tools/build/installed-smoke/artifact-hash.ts",
  "tools/build/installed-smoke/human-report.ts",
  "tools/build/installed-smoke/manifest-server-data-route-manifest.ts",
  "tools/build/installed-smoke/no-node-modules.ts",
  "tools/build/installed-smoke/receipt-writer.ts",
  "tools/build/installed-smoke/source-freshness.ts",
  "tools/build/readiness-gate/proof-bundle.ts",
];

const FORBIDDEN_BROAD_ARTIFACT_IGNORES = [
  ".dx/",
  ".dx/**",
  ".dx/receipts/",
  ".dx/receipts/**",
  ".dx/receipts/deploy/",
  ".dx/receipts/deploy/**",
  ".dx/receipts/graph/",
  ".dx/receipts/graph/**",
  ".dx/receipts/style/",
  ".dx/receipts/style/**",
  ".dx/template-app-browser-preview/",
  ".dx/template-app-browser-preview/**",
  "benchmarks/reports/",
  "benchmarks/reports/**",
  "cache/",
  "cache/**",
  "examples/template/.dx/forge/",
  "examples/template/.dx/forge/**",
  "**/cache/",
  "**/cache/**",
  "dx-www/src/build/",
  "dx-www/src/build/**",
  "tools/build/",
  "tools/build/**",
  "tools/build/installed-smoke/",
  "tools/build/installed-smoke/**",
  "tools/build/readiness-gate/",
  "tools/build/readiness-gate/**",
  "worker-lanes/",
  "worker-lanes/**",
];

const ARTIFACT_INVENTORY_DECISIONS = [
  ...GENERATED_LOCAL_ARTIFACTS.map((artifact) => ({
    path: artifact.path,
    decision: "ignore-local-generated",
    reason: artifact.reason,
  })),
  ...SOURCE_OWNED_TRACKED_ARTIFACTS.map((sourceOwnedPath) => ({
    path: sourceOwnedPath,
    decision: "keep-source-owned-tracked",
    reason: "tracked proof or source-owned artifact",
  })),
  ...SOURCE_OWNED_VISIBLE_HELPERS.map((sourceOwnedPath) => ({
    path: sourceOwnedPath,
    decision: "keep-source-owned-visible",
    reason: "source-owned helper or in-flight source work",
  })),
];

const VALID_ARTIFACT_DECISIONS = new Set([
  "ignore-local-generated",
  "keep-source-owned-tracked",
  "keep-source-owned-visible",
]);

const GENERATED_LOCAL_ARTIFACT_PATHS = new Set(
  GENERATED_LOCAL_ARTIFACTS.map((artifact) => artifact.path),
);

const ignoredCache = new Map();
const ignorePatternCache = new Map();
const trackedCache = new Map();

test("local generated artifact buckets are ignored", () => {
  for (const artifact of GENERATED_LOCAL_ARTIFACTS) {
    assert.equal(
      isIgnored(artifact.path),
      true,
      `${artifact.path} should be ignored as ${artifact.reason}`,
    );
  }
});

test("local generated artifacts are ignored by narrow policy rules", () => {
  for (const artifact of GENERATED_LOCAL_ARTIFACTS) {
    assert.equal(
      ignorePattern(artifact.path),
      artifact.expectedPattern,
      `${artifact.path} should be covered by a narrow generated-artifact rule`,
    );
  }
});

test("local generated artifact buckets stay out of normal status", () => {
  const status = git([
    "status",
    "--short",
    "--",
    ".tmp",
    ".cache",
    ".dx/launch",
    ".dx/codex-localhost-proof",
    ".dx/receipts/build/installed-binary-smoke-latest.json",
    ".dx/template-app-browser-preview/server.pid",
    "NUL",
    "worker-lanes/state",
    "target-codex-readiness-gate",
    "examples/template/.dx/build",
    "examples/template/.dx/receipts/build",
    "examples/template/.dx/receipts/graph",
  ]);
  const unexpectedStatus = status.stdout
    .split(/\r?\n/)
    .filter(Boolean)
    .filter((line) => !isPendingGeneratedArtifactDeletion(line))
    .join("\n");
  assert.equal(unexpectedStatus, "", unexpectedStatus);
});

test("ignored local generated artifacts are not tracked and present", () => {
  for (const artifact of GENERATED_LOCAL_ARTIFACTS) {
    assert.equal(
      isTrackedAndPresent(artifact.path),
      false,
      `${artifact.path} is local generated output and must not be tracked as a present file`,
    );
  }
});

test("artifact inventory has explicit cleanup decisions", () => {
  const seenPaths = new Set();
  const decisionCounts = new Map();

  for (const artifact of ARTIFACT_INVENTORY_DECISIONS) {
    assert.equal(seenPaths.has(artifact.path), false, `${artifact.path} has duplicate decisions`);
    assert.equal(
      VALID_ARTIFACT_DECISIONS.has(artifact.decision),
      true,
      `${artifact.path} has unknown decision ${artifact.decision}`,
    );
    assert.notEqual(artifact.reason.trim(), "", `${artifact.path} needs a cleanup reason`);

    seenPaths.add(artifact.path);
    decisionCounts.set(artifact.decision, (decisionCounts.get(artifact.decision) ?? 0) + 1);

    if (artifact.decision === "ignore-local-generated") {
      assert.equal(isIgnored(artifact.path), true, `${artifact.path} should stay ignored`);
      continue;
    }

    assert.equal(isIgnored(artifact.path), false, `${artifact.path} should stay visible`);
    if (artifact.decision === "keep-source-owned-tracked") {
      assert.equal(isTracked(artifact.path), true, `${artifact.path} should stay tracked`);
    }
  }

  for (const decision of VALID_ARTIFACT_DECISIONS) {
    assert.ok(decisionCounts.get(decision) > 0, `${decision} needs inventory coverage`);
  }
});

test("artifact ignore policy does not hide source-owned evidence families", () => {
  const activePatterns = fs
    .readFileSync(gitignorePath, "utf8")
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter((line) => line && !line.startsWith("#"));

  for (const forbiddenPattern of FORBIDDEN_BROAD_ARTIFACT_IGNORES) {
    assert.equal(
      activePatterns.includes(forbiddenPattern),
      false,
      `${forbiddenPattern} would hide source-owned evidence`,
    );
  }
});

test("source-owned tracked proof and Forge artifacts remain visible", () => {
  for (const sourceOwnedPath of SOURCE_OWNED_TRACKED_ARTIFACTS) {
    assert.equal(isIgnored(sourceOwnedPath), false, `${sourceOwnedPath} should remain visible`);
    assert.equal(isTracked(sourceOwnedPath), true, `${sourceOwnedPath} should stay tracked`);
  }
});

test("source-owned build and proof helper work remains visible for release control", () => {
  for (const sourceOwnedPath of SOURCE_OWNED_VISIBLE_HELPERS) {
    assert.equal(isIgnored(sourceOwnedPath), false, `${sourceOwnedPath} should remain visible`);
  }
});

test("media-icon required machine caches and readiness receipt are local generated artifacts", () => {
  const readinessSource = fs.readFileSync(
    path.join(repoRoot, "related-crates/media-icon/src/machine_readiness.rs"),
    "utf8",
  );
  const requiredCacheNames = readinessSource.match(
    /pub const REQUIRED_ICON_MACHINE_CACHE_NAMES: \[&str; 7\] = \[([\s\S]*?)\];/,
  );
  assert.ok(requiredCacheNames, "required media-icon machine cache names should be declared");
  const cacheNames = Array.from(
    requiredCacheNames[1].matchAll(/"([^"]+)"/g),
    (match) => match[1],
  );

  assert.deepEqual(cacheNames, [
    "manifest",
    "catalog",
    "prefix",
    "perfect-hash",
    "bloom",
    "lowercase-cache",
    "pack-body",
  ]);

  for (const cacheName of cacheNames) {
    for (const suffix of [".machine", ".machine.meta.json"]) {
      const artifactPath = `.dx/icon/machine/v1/${cacheName}${suffix}`;
      assert.equal(
        GENERATED_LOCAL_ARTIFACTS.some(
          (artifact) =>
            artifact.path === artifactPath && artifact.expectedPattern === ".dx/icon/",
        ),
        true,
        `${artifactPath} should be inventoried as local generated media-icon cache output`,
      );
    }
  }

  const readinessReceipt = readinessSource.match(
    /pub const ICON_MACHINE_CACHE_READINESS_RECEIPT_PATH: &str =\s*"([^"]+)"/,
  );
  assert.ok(readinessReceipt, "readiness receipt path constant should stay visible to hygiene");
  assert.equal(
    GENERATED_LOCAL_ARTIFACTS.some(
      (artifact) =>
        artifact.path === readinessReceipt[1] && artifact.expectedPattern === ".dx/performance/",
    ),
    true,
    `${readinessReceipt[1]} should be inventoried as local generated performance output`,
  );
});

function isIgnored(relativePath) {
  if (ignoredCache.has(relativePath)) {
    return ignoredCache.get(relativePath);
  }

  const result = git(["check-ignore", "--no-index", "-q", "--", relativePath], {
    allowStatus: [0, 1],
  });
  if (result.status === 0) {
    ignoredCache.set(relativePath, true);
    return true;
  }
  if (result.status === 1) {
    ignoredCache.set(relativePath, false);
    return false;
  }
  throw new Error(result.stderr || `git check-ignore exited ${result.status}`);
}

function ignorePattern(relativePath) {
  if (ignorePatternCache.has(relativePath)) {
    return ignorePatternCache.get(relativePath);
  }

  const result = git(["check-ignore", "--no-index", "-v", "--", relativePath]);
  const match = result.stdout.match(/^[^:]+:\d+:(.*?)\t/);
  if (!match) {
    throw new Error(`Could not parse git check-ignore output for ${relativePath}: ${result.stdout}`);
  }
  ignorePatternCache.set(relativePath, match[1]);
  return match[1];
}

function isTracked(relativePath) {
  if (trackedCache.has(relativePath)) {
    return trackedCache.get(relativePath);
  }

  const result = git(["ls-files", "--error-unmatch", "--", relativePath], {
    allowStatus: [0, 1],
  });
  const tracked = result.status === 0;
  trackedCache.set(relativePath, tracked);
  return tracked;
}

function isTrackedAndPresent(relativePath) {
  return isTracked(relativePath) && fs.existsSync(path.join(repoRoot, relativePath));
}

function isPendingGeneratedArtifactDeletion(statusLine) {
  const statusCode = statusLine.slice(0, 2);
  const statusPath = statusLine.slice(3);
  return statusCode.includes("D") && GENERATED_LOCAL_ARTIFACT_PATHS.has(statusPath);
}

function git(args, options = {}) {
  const result = spawnSync("git", args, {
    cwd: repoRoot,
    encoding: "utf8",
    windowsHide: true,
  });
  if (options.allowStatus?.includes(result.status)) {
    return result;
  }
  if (result.status !== 0) {
    throw new Error(result.stderr || `git ${args.join(" ")} exited ${result.status}`);
  }
  return result;
}
