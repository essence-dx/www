const fs = require("node:fs");
const path = require("node:path");

const CONFLICT_MARKER_SCHEMA = "dx.nextRustMerge.conflictMarkers";

const DEFAULT_CONFLICT_MARKER_TARGETS = Object.freeze([
  "docs/next-rust-merge-checkpoint.md",
  "vendor/next-rust/README.md",
  "dx-www/src/next_rust.rs",
  "tools/next-rust-merge",
  "benchmarks/next-rust-merge-coordinator.test.ts",
  "benchmarks/next-rust-merge-audit-comparison.test.ts",
  "benchmarks/next-rust-vendor-boundary.test.ts",
  "benchmarks/next-rust-schema-status-noise.test.ts",
  "benchmarks/next-rust-giant-cli-mod.test.ts",
]);

const TEXT_EXTENSIONS = new Set([
  ".cjs",
  ".css",
  ".html",
  ".js",
  ".json",
  ".md",
  ".mjs",
  ".rs",
  ".toml",
  ".ts",
  ".tsx",
]);

function scanConflictMarkers({
  cwd = process.cwd(),
  targets = DEFAULT_CONFLICT_MARKER_TARGETS,
} = {}) {
  const resolved = collectTargetFiles(cwd, targets);
  const markers = [];

  for (const filePath of resolved.files) {
    markers.push(...scanFileForConflictMarkers(filePath, cwd));
  }

  return {
    schema: CONFLICT_MARKER_SCHEMA,
    status: markers.length === 0 && resolved.missingTargets.length === 0 ? "passed" : "failed",
    targetCount: targets.length,
    scannedFileCount: resolved.files.length,
    missingTargets: resolved.missingTargets,
    markers,
  };
}

function collectTargetFiles(cwd, targets) {
  const files = [];
  const missingTargets = [];

  for (const target of targets) {
    const absoluteTarget = path.resolve(cwd, target);
    if (!fs.existsSync(absoluteTarget)) {
      missingTargets.push(normalizePath(target));
      continue;
    }

    const stat = fs.statSync(absoluteTarget);
    if (stat.isDirectory()) {
      collectDirectoryFiles(absoluteTarget, files);
    } else if (stat.isFile() && isScannableTextFile(absoluteTarget)) {
      files.push(absoluteTarget);
    }
  }

  return {
    files: [...new Set(files)].sort(),
    missingTargets,
  };
}

function collectDirectoryFiles(directory, files) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    if (entry.name === ".git" || entry.name === "node_modules" || entry.name === "target") {
      continue;
    }

    const absolute = path.join(directory, entry.name);
    if (entry.isDirectory()) {
      collectDirectoryFiles(absolute, files);
    } else if (entry.isFile() && isScannableTextFile(absolute)) {
      files.push(absolute);
    }
  }
}

function isScannableTextFile(filePath) {
  return TEXT_EXTENSIONS.has(path.extname(filePath).toLowerCase());
}

function scanFileForConflictMarkers(filePath, cwd = process.cwd()) {
  const text = fs.readFileSync(filePath, "utf8");
  const lines = text.split(/\r?\n/);
  const markers = [];
  let block = null;

  lines.forEach((line, index) => {
    const lineNumber = index + 1;

    if (/^<<<<<<<(?:\s|$)/.test(line)) {
      block = {
        opener: marker(filePath, cwd, lineNumber, line),
        separators: [],
      };
      return;
    }

    if (block && /^=======$/.test(line)) {
      block.separators.push(marker(filePath, cwd, lineNumber, line));
      return;
    }

    if (/^>>>>>>>(?:\s|$)/.test(line)) {
      if (block) {
        markers.push(block.opener, ...block.separators, marker(filePath, cwd, lineNumber, line));
        block = null;
      } else {
        markers.push(marker(filePath, cwd, lineNumber, line));
      }
    }
  });

  if (block) {
    markers.push(block.opener, ...block.separators);
  }

  return markers;
}

function marker(filePath, cwd, line, text) {
  return {
    file: normalizePath(path.relative(cwd, filePath)),
    line,
    marker: text,
  };
}

function normalizePath(value) {
  return value.replace(/\\/g, "/");
}

if (require.main === module) {
  const report = scanConflictMarkers();
  if (process.argv.includes("--json")) {
    process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
  } else if (report.status === "passed") {
    process.stdout.write(`No merge conflict markers in ${report.scannedFileCount} files.\n`);
  } else {
    for (const entry of report.markers) {
      process.stdout.write(`${entry.file}:${entry.line}: ${entry.marker}\n`);
    }
    for (const target of report.missingTargets) {
      process.stdout.write(`missing target: ${target}\n`);
    }
  }
  process.exitCode = report.status === "passed" ? 0 : 1;
}

module.exports = {
  CONFLICT_MARKER_SCHEMA,
  DEFAULT_CONFLICT_MARKER_TARGETS,
  scanConflictMarkers,
  scanFileForConflictMarkers,
};
