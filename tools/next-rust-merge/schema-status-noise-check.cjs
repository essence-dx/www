const fs = require("node:fs");
const path = require("node:path");

const SCHEMA_STATUS_NOISE_SCHEMA = "dx.nextRustMerge.schemaStatusNoiseCheck";

const DEFAULT_SCHEMA_STATUS_NOISE_FILES = [
  "tools/next-rust-merge/audit-gap-check-map.json",
  "tools/next-rust-merge/readiness-audit-baseline.json",
  "tools/next-rust-merge/coordinator-audit-comparison.cjs",
  "tools/next-rust-merge/coordinator-readiness-audit.cjs",
  "tools/next-rust-merge/coordinator-checks.cjs",
  "tools/next-rust-merge/coordinator-preflight.cjs",
  "tools/next-rust-merge/coordinator-report-contract.cjs",
  "tools/next-rust-merge/coordinator-runner.cjs",
  "tools/next-rust-merge/coordinator-side-effects.cjs",
  "tools/next-rust-merge/giant-cli-mod-check.cjs",
  "tools/next-rust-merge/schema-status-noise-check.cjs",
  "dx-www/src/cli/app_route_handler_receipt.rs",
  "dx-www/src/cli/app_router_build_command.rs",
  "core/src/delivery/server_contract.rs",
  "docs/NEXTJS_COMPATIBILITY_MAP.md",
  "examples/template/package.json",
  "integrations/n8n-nodes-base/dx-node-source-manifest.json",
];

const SCHEMA_STATUS_NOISE_RULES = [
  {
    id: "public-schema-version-suffix",
    pattern: /\bdx(?:\.[A-Za-z][A-Za-z0-9_-]*)+\.v1\b/g,
    message: "Public DX schema names should not regain a .v1 suffix.",
  },
  {
    id: "full-next-parity-overclaim",
    pattern: /\bfull\s+Next(?:\.js)?\s+parity\b/gi,
    message: "Coordinator status must not claim complete Next compatibility.",
  },
  {
    id: "production-merge-ready-overclaim",
    pattern: /\bproduction\s+merge\s+ready\s*[:=]\s*true\b/gi,
    message: "Coordinator status must not mark the audited merge production-ready.",
  },
];

function runSchemaStatusNoiseCheck({
  cwd = process.cwd(),
  files = DEFAULT_SCHEMA_STATUS_NOISE_FILES,
} = {}) {
  const scannedFiles = files.map((file) => path.resolve(cwd, file));
  const violations = scannedFiles.flatMap((filePath) => scanFile(filePath));

  return {
    schema: SCHEMA_STATUS_NOISE_SCHEMA,
    lane: 14,
    laneName: "Final Coordinator",
    featureImplementation: false,
    proofLevel: "read-only-source-noise-scan",
    status: violations.length === 0 ? "passing" : "failed",
    scannedFiles,
    rules: SCHEMA_STATUS_NOISE_RULES.map(({ id, message }) => ({ id, message })),
    violations,
    sideEffects: [],
  };
}

function scanFile(filePath) {
  if (!fs.existsSync(filePath)) {
    return [
      {
        id: "missing-source-file",
        file: filePath,
        line: 0,
        column: 0,
        match: "",
        message: "Expected coordinator source file is missing.",
      },
    ];
  }

  const text = fs.readFileSync(filePath, "utf8");
  return SCHEMA_STATUS_NOISE_RULES.flatMap((rule) =>
    findRuleViolations(filePath, text, rule),
  );
}

function findRuleViolations(filePath, text, rule) {
  const violations = [];
  const lines = text.split(/\r?\n/);

  lines.forEach((line, index) => {
    const pattern = new RegExp(rule.pattern.source, rule.pattern.flags);
    let match = pattern.exec(line);

    while (match !== null) {
      violations.push({
        id: rule.id,
        file: filePath,
        line: index + 1,
        column: match.index + 1,
        match: match[0],
        message: rule.message,
      });
      match = pattern.exec(line);
    }
  });

  return violations;
}

if (require.main === module) {
  const report = runSchemaStatusNoiseCheck();

  if (process.argv.includes("--json")) {
    process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
  } else if (report.status === "passing") {
    process.stdout.write("schema/status noise check passed\n");
  } else {
    for (const violation of report.violations) {
      process.stdout.write(
        `${violation.file}:${violation.line}:${violation.column} ${violation.id}: ${violation.message}\n`,
      );
    }
  }

  process.exitCode = report.status === "passing" ? 0 : 1;
}

module.exports = {
  DEFAULT_SCHEMA_STATUS_NOISE_FILES,
  SCHEMA_STATUS_NOISE_RULES,
  SCHEMA_STATUS_NOISE_SCHEMA,
  runSchemaStatusNoiseCheck,
};
