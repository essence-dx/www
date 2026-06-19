import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");

const forbiddenRootFiles = [
  "NONE",
  "NUL",
  "index.html",
  ".dx-devtools-server.err.log",
  ".dx-devtools-server.out.log",
  "cargo-operator-index.log",
  "cargo-operator-index-cmd.log",
  "cargo-release-checklist.log",
  "cargo-share-manifest.log",
];

const forbiddenDirectories = [
  ".codex-tmp",
  ".tmp",
  ".dx/codex-run",
  ".dx/run",
  ".dx/build",
  ".dx/style",
  ".dx/serializer",
  ".dx/launch",
  ".dx/adoption-package-review",
  ".dx/adoption-update-rehearsal",
  "artifacts",
  "target-codex-readiness-gate",
  "trash",
  "examples/deprecated-1",
  "examples/deprecated-2",
  "examples/depricated-3",
  "examples/onboard-deprecated-recovery-copy",
];

export const TWELVE_WORST_HYGIENE_FLAWS = [
  {
    id: "cli-mod-large",
    category: "large-source-file",
    path: "dx-www/src/cli/mod.rs",
    lines: 5_000,
    owner: "CLI dispatch",
    severity: "critical",
  },
  {
    id: "public-framework-tools-large",
    category: "large-source-file",
    path: "dx-www/src/cli/public_framework_tools.rs",
    lines: 4_000,
    owner: "public framework tools",
    severity: "high",
  },
  {
    id: "source-render-large",
    category: "large-source-file",
    path: "dx-www/src/cli/app_router_execution/source_render.rs",
    lines: 4_000,
    owner: "App Router renderer",
    severity: "high",
  },
  {
    id: "dx-check-receipt-large",
    category: "large-source-file",
    path: "core/src/ecosystem/dx_check_receipt.rs",
    lines: 4_000,
    owner: "ecosystem receipt model",
    severity: "critical",
  },
  {
    id: "forge-registry-large",
    category: "large-source-file",
    path: "core/src/ecosystem/forge_registry.rs",
    lines: 4_000,
    owner: "Forge registry",
    severity: "critical",
  },
  {
    id: "project-check-large",
    category: "large-source-file",
    path: "core/src/ecosystem/project_check.rs",
    lines: 4_000,
    owner: "project checker",
    severity: "high",
  },
  {
    id: "devtools-runtime-large",
    category: "large-source-file",
    path: "dx-www/src/cli/devtools/assets/runtime.ts",
    lines: 1_200,
    owner: "Devtools browser runtime",
    severity: "high",
  },
  {
    id: "devtools-style-ops-large",
    category: "large-source-file",
    path: "dx-www/src/cli/devtools/style_ops.rs",
    lines: 1_200,
    owner: "Devtools style operations",
    severity: "medium",
  },
  {
    id: "devtools-css-large",
    category: "large-source-file",
    path: "dx-devtools/styles/devtools.css",
    lines: 2_000,
    owner: "standalone Devtools CSS",
    severity: "high",
  },
  {
    id: "source-visible-fixtures",
    category: "tracked-generated-surface",
    owner: "source-visible fixture contract",
    severity: "medium",
  },
  {
    id: "legacy-script-extensions",
    category: "legacy-script-extension",
    owner: "TypeScript benchmark migration",
    severity: "medium",
  },
  {
    id: "readiness-overclaim-risk",
    category: "readiness-overclaim-risk",
    owner: "repo-wide 100/100 claims",
    severity: "critical",
  },
];

const largeSourceBudgets = TWELVE_WORST_HYGIENE_FLAWS.filter(
  (flaw) => flaw.category === "large-source-file",
);

const sourceFileExtensions = new Set([".rs", ".ts", ".tsx", ".css"]);
const largeSourceChildRoots = new Map([
  ["dx-www/src/cli/mod.rs", ["dx-www/src/cli/mod_parts"]],
  [
    "dx-www/src/cli/public_framework_tools.rs",
    ["dx-www/src/cli/public_framework_tools"],
  ],
  [
    "dx-www/src/cli/app_router_execution/source_render.rs",
    ["dx-www/src/cli/app_router_execution/source_render_parts"],
  ],
  [
    "dx-www/src/cli/devtools/assets/runtime.ts",
    ["dx-www/src/cli/devtools/assets/runtime"],
  ],
  ["core/src/ecosystem/dx_check_receipt.rs", ["core/src/ecosystem/dx_check_receipt"]],
  ["core/src/ecosystem/forge_registry.rs", ["core/src/ecosystem/forge_registry_parts"]],
  ["core/src/ecosystem/project_check.rs", ["core/src/ecosystem/project_check"]],
  ["dx-devtools/styles/devtools.css", ["dx-devtools/styles/devtools"]],
]);

const trackedGeneratedDebtRoots = [
  ".dx/template-app-browser-preview",
  ".dx/receipts",
  "components",
  "lib",
  "pages",
  "public",
  "dx-devtools/.dx",
];

const sourceVisibleFixtureContractPath = "docs/hygiene/source-visible-fixtures.md";
const legacyScriptExtensions = new Set([".js", ".cjs", ".mjs"]);
const legacyScriptContractPath = "docs/hygiene/legacy-script-extensions.md";
const legacyScriptOwnershipReasons = new Set([
  "runtime",
  "vendor",
  "fixture",
  "generated-proof",
]);

function normalizeContractPath(contractPath) {
  return contractPath.replaceAll("\\", "/").replace(/^\.\//, "");
}

function readOwnershipContract(root, relativePath) {
  const fullPath = path.join(root, relativePath);
  if (!fs.existsSync(fullPath)) {
    return [];
  }

  const entries = [];
  const lines = fs.readFileSync(fullPath, "utf8").split(/\r?\n/);
  let insideFence = false;
  for (const [index, line] of lines.entries()) {
    if (line.trimStart().startsWith("```")) {
      insideFence = !insideFence;
      continue;
    }
    if (insideFence) {
      continue;
    }

    const match = line.match(/^\s*-\s+`([^`]+)`:\s*(.+)$/);
    if (!match) {
      continue;
    }

    const fields = {};
    for (const part of match[2].split(";")) {
      const trimmed = part.trim();
      if (!trimmed) {
        continue;
      }
      const separator = trimmed.indexOf("=");
      if (separator === -1) {
        continue;
      }
      const key = trimmed.slice(0, separator).trim();
      const value = trimmed.slice(separator + 1).trim();
      if (key && value) {
        fields[key] = value;
      }
    }

    entries.push({
      path: normalizeContractPath(match[1]),
      fields,
      line: index + 1,
    });
  }

  return entries;
}

function contractEntryMatches(entry, relativePath) {
  const normalized = normalizeContractPath(relativePath);
  if (entry.path.endsWith("/")) {
    const rootPath = entry.path.slice(0, -1);
    return normalized === rootPath || normalized.startsWith(entry.path);
  }
  return normalized === entry.path;
}

function contractEntryForPath(entries, relativePath) {
  return entries.find((entry) => contractEntryMatches(entry, relativePath));
}

function hasContractFields(entry, fields) {
  return Boolean(
    entry &&
      fields.every(
        (field) =>
          typeof entry.fields[field] === "string" &&
          entry.fields[field].trim().length > 0,
      ),
  );
}

function isOwnedSourceVisibleFixture(entry) {
  return hasContractFields(entry, ["owner", "source", "removal_gate"]);
}

function isOwnedLegacyScript(entry) {
  return (
    hasContractFields(entry, ["owner", "reason", "migration_gate"]) &&
    legacyScriptOwnershipReasons.has(entry.fields.reason)
  );
}

export function auditRepoHygiene(root = repoRoot) {
  const blockers = [];
  const debt = [];
  const repoFiles = walkFiles(root);
  const sourceVisibleFixtureContract = readOwnershipContract(
    root,
    sourceVisibleFixtureContractPath,
  );
  const legacyScriptContract = readOwnershipContract(root, legacyScriptContractPath);

  for (const relativePath of forbiddenRootFiles) {
    const fullPath = path.join(root, relativePath);
    if (fs.existsSync(fullPath)) {
      blockers.push({
        category: "root-junk",
        path: relativePath,
        message: `${relativePath} must not exist at the repository root`,
      });
    }
  }

  for (const relativePath of forbiddenDirectories) {
    const fullPath = path.join(root, relativePath);
    if (fs.existsSync(fullPath)) {
      blockers.push({
        category: "forbidden-directory",
        path: relativePath,
        message: `${relativePath} is generated, archived, or deprecated output and must not live in the active tree`,
      });
    }
  }

  for (const budget of largeSourceBudgets) {
    for (const relativePath of largeSourceBudgetPaths(repoFiles, budget)) {
      const fullPath = path.join(root, relativePath);
      if (!fs.existsSync(fullPath)) {
        continue;
      }
      const lines = countLines(fullPath);
      if (lines <= budget.lines) {
        continue;
      }
      debt.push({
        flawId: budget.id,
        category: "large-source-file",
        path: relativePath,
        owner: budget.owner,
        lines,
        budget: budget.lines,
        message: `${relativePath} has ${lines} lines; split ${budget.owner} before calling the codebase 100/100`,
      });
    }
  }

  const trackedGeneratedDebt = trackedGeneratedDebtRoots
    .filter((relativePath) => fs.existsSync(path.join(root, relativePath)))
    .filter((relativePath) => {
      const entry = contractEntryForPath(sourceVisibleFixtureContract, relativePath);
      return !isOwnedSourceVisibleFixture(entry);
    })
    .map((relativePath) => ({
      flawId: "source-visible-fixtures",
      category: "tracked-generated-surface",
      path: relativePath,
      message: `${relativePath} is source-visible generated or fixture state without docs/hygiene/source-visible-fixtures.md ownership`,
    }));
  debt.push(...trackedGeneratedDebt);

  const legacyScripts = repoFiles
    .filter((relativePath) => legacyScriptExtensions.has(path.extname(relativePath)))
  const unownedLegacyScripts = legacyScripts.filter((relativePath) => {
    const entry = contractEntryForPath(legacyScriptContract, relativePath);
    return !isOwnedLegacyScript(entry);
  });
  if (unownedLegacyScripts.length > 0) {
    debt.push({
      flawId: "legacy-script-extensions",
      category: "legacy-script-extension",
      path: "repo",
      count: unownedLegacyScripts.length,
      sample: unownedLegacyScripts.slice(0, 25),
      total: legacyScripts.length,
      message: "Legacy .js/.cjs/.mjs scripts remain without runtime/vendor/fixture/generated-proof ownership",
    });
  }

  const scorecard = buildScorecard({ blockers, debt, legacyScripts });
  const readiness = buildReadiness({ blockers, scorecard });

  return {
    ok: blockers.length === 0 && debt.length === 0,
    blockers,
    debt,
    scorecard,
    readiness,
    metrics: {
      blockers: blockers.length,
      debt: debt.length,
      activeFlaws: scorecard.filter((flaw) => flaw.status !== "passing").length,
      legacyScripts: legacyScripts.length,
      legacyScriptsOwned: legacyScripts.length - unownedLegacyScripts.length,
      legacyScriptsUnowned: unownedLegacyScripts.length,
      sourceVisibleFixturesOwned: trackedGeneratedDebtRoots
        .filter((relativePath) => fs.existsSync(path.join(root, relativePath)))
        .filter((relativePath) =>
          isOwnedSourceVisibleFixture(contractEntryForPath(sourceVisibleFixtureContract, relativePath)),
        ).length,
      hygieneScore: readiness.score,
    },
  };
}

function largeSourceBudgetPaths(repoFiles, budget) {
  const paths = new Set([budget.path]);
  for (const rootPath of largeSourceChildRoots.get(budget.path) ?? []) {
    const normalizedRoot = normalizeContractPath(rootPath);
    for (const relativePath of repoFiles) {
      if (
        relativePath.startsWith(`${normalizedRoot}/`) &&
        sourceFileExtensions.has(path.extname(relativePath))
      ) {
        paths.add(relativePath);
      }
    }
  }
  return [...paths].sort();
}

function buildScorecard({ blockers, debt, legacyScripts }) {
  const debtByFlawId = new Map();
  for (const item of debt) {
    if (!item.flawId) {
      continue;
    }
    const items = debtByFlawId.get(item.flawId) ?? [];
    items.push(item);
    debtByFlawId.set(item.flawId, items);
  }

  return TWELVE_WORST_HYGIENE_FLAWS.map((flaw) => {
    if (flaw.id === "readiness-overclaim-risk") {
      const active = blockers.length > 0 || debt.length > 0;
      return {
        id: flaw.id,
        category: flaw.category,
        severity: flaw.severity,
        owner: flaw.owner,
        status: active ? "active" : "passing",
        itemCount: blockers.length + debt.length,
        message: active
          ? "Repo hygiene 100/100 claims are blocked until blockers and hygiene debt are zero."
          : "The hygiene scorecard is closed; codebase, product, browser, provider, and launch readiness still need separate proof.",
      };
    }

    const items = debtByFlawId.get(flaw.id) ?? [];
    const pathValue = flaw.path ?? flaw.id;
    return {
      id: flaw.id,
      category: flaw.category,
      severity: flaw.severity,
      owner: flaw.owner,
      path: pathValue,
      status: items.length > 0 ? "active" : "passing",
      itemCount: items.length,
      legacyScriptCount: flaw.id === "legacy-script-extensions" ? legacyScripts.length : undefined,
      message: items.length > 0
        ? items[0].message
        : `${pathValue} is inside the current hygiene budget.`,
    };
  });
}

function buildReadiness({ blockers, scorecard }) {
  const activeFlaws = scorecard.filter((flaw) => flaw.status !== "passing");
  const score = Math.max(
    0,
    100
      - blockers.length * 15
      - activeFlaws.reduce((penalty, flaw) => penalty + severityPenalty(flaw.severity), 0),
  );

  return {
    status: blockers.length > 0 ? "blocked" : activeFlaws.length > 0 ? "debt" : "ready",
    claimScope: "repo-hygiene-scorecard",
    readyFor100: blockers.length === 0 && activeFlaws.length === 0,
    blockerFree: blockers.length === 0,
    debtFree: activeFlaws.length === 0,
    codebaseReady: false,
    productReady: false,
    browserReady: false,
    providerReady: false,
    launchReady: false,
    score,
    blockerCount: blockers.length,
    activeFlawCount: activeFlaws.length,
    scorecardTotal: scorecard.length,
    scorecardOpen: activeFlaws.length,
    activeFlawIds: activeFlaws.map((flaw) => flaw.id),
    message: activeFlaws.length === 0 && blockers.length === 0
      ? "Repository hygiene scorecard is closed. This does not prove codebase, product, browser, provider, or launch readiness."
      : "Repository hygiene still blocks a hygiene 100/100 claim; inspect activeFlawIds before reporting readiness.",
  };
}

function severityPenalty(severity) {
  if (severity === "critical") {
    return 10;
  }
  if (severity === "high") {
    return 7;
  }
  return 4;
}

function countLines(fullPath) {
  const source = fs.readFileSync(fullPath, "utf8");
  if (source.length === 0) {
    return 0;
  }
  return source.split(/\r\n|\r|\n/).length;
}

function walkFiles(root) {
  const ignoredDirectories = new Set([
    ".git",
    "target",
    "target-codex-readiness-gate",
    "node_modules",
  ]);
  const files = [];
  const stack = [root];

  while (stack.length > 0) {
    const current = stack.pop();
    for (const entry of fs.readdirSync(current, { withFileTypes: true })) {
      if (entry.isDirectory() && ignoredDirectories.has(entry.name)) {
        continue;
      }
      const fullPath = path.join(current, entry.name);
      if (entry.isDirectory()) {
        stack.push(fullPath);
        continue;
      }
      if (entry.isFile()) {
        files.push(path.relative(root, fullPath).replaceAll(path.sep, "/"));
      }
    }
  }

  return files.sort();
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const result = auditRepoHygiene();
  const wantsJson = process.argv.includes("--json");

  if (wantsJson) {
    process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
  } else {
    const status = result.ok ? "PASS" : "FAIL";
    process.stdout.write(`repo hygiene: ${status}\n`);
    process.stdout.write(`blockers: ${result.blockers.length}\n`);
    process.stdout.write(`debt: ${result.debt.length}\n`);
    process.stdout.write(`readiness.readyFor100: ${result.readiness.readyFor100}\n`);
    process.stdout.write(`readiness.status: ${result.readiness.status}\n`);
    process.stdout.write(`readiness.claimScope: ${result.readiness.claimScope}\n`);
    const broaderReadiness = [
      `codebase=${result.readiness.codebaseReady}`,
      `product=${result.readiness.productReady}`,
      `browser=${result.readiness.browserReady}`,
      `provider=${result.readiness.providerReady}`,
      `launch=${result.readiness.launchReady}`,
    ].join(" ");
    process.stdout.write(
      `readiness.broaderReadiness: ${broaderReadiness}\n`,
    );
    process.stdout.write(`scorecard: ${result.readiness.scorecardOpen}/${result.readiness.scorecardTotal} active\n`);
    for (const blocker of result.blockers) {
      process.stdout.write(`BLOCKER ${blocker.category}: ${blocker.path} - ${blocker.message}\n`);
    }
    for (const item of result.debt.slice(0, 30)) {
      process.stdout.write(`DEBT ${item.category}: ${item.path} - ${item.message}\n`);
    }
    for (const flaw of result.scorecard) {
      process.stdout.write(`SCORECARD ${flaw.id}: ${flaw.status} - ${flaw.message}\n`);
    }
  }

  if (!result.ok) {
    process.exitCode = 1;
  }
}
