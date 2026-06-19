import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const unsupportedTestExtension = /\.test\.(?:cjs|mjs)$/;

type StatusEntry = {
  status: string;
  filePath: string;
};

function git(args: string[]): string {
  return execFileSync("git", args, {
    cwd: repoRoot,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  });
}

function statusEntries(): StatusEntry[] {
  const fields = git(["status", "--porcelain=v1", "-z", "--untracked-files=all"]).split("\0");
  const entries: StatusEntry[] = [];

  for (let index = 0; index < fields.length; index += 1) {
    const field = fields[index];
    if (!field) {
      continue;
    }

    const status = field.slice(0, 2);
    const filePath = field.slice(3);
    entries.push({ status, filePath });

    if (/[CR]/.test(status)) {
      index += 1;
    }
  }

  return entries;
}

function isIntroduced(status: string): boolean {
  return status === "??" || /[ACR]/.test(status);
}

function isDeleted(status: string): boolean {
  return status.includes("D");
}

function expectedTypeScriptReplacement(filePath: string): string {
  return filePath.replace(/\.(?:cjs|mjs)$/, ".ts");
}

test("final polish adds new tests as TypeScript, not cjs or mjs", () => {
  const unsupportedNewTests = statusEntries()
    .filter(({ status }) => isIntroduced(status))
    .map(({ filePath }) => filePath.replaceAll("\\", "/"))
    .filter((filePath) => unsupportedTestExtension.test(filePath));

  assert.deepEqual(
    unsupportedNewTests,
    [],
    `new final-polish tests must be .ts by default: ${unsupportedNewTests.join(", ")}`,
  );
});

test("final polish legacy test migrations keep a TypeScript replacement", () => {
  const missingReplacements = statusEntries()
    .filter(({ status }) => isDeleted(status))
    .map(({ filePath }) => filePath.replaceAll("\\", "/"))
    .filter((filePath) => unsupportedTestExtension.test(filePath))
    .map((filePath) => expectedTypeScriptReplacement(filePath))
    .filter((replacementPath) => !fs.existsSync(path.join(repoRoot, replacementPath)));

  assert.deepEqual(
    missingReplacements,
    [],
    `deleted legacy test guards need same-name .ts replacements: ${missingReplacements.join(", ")}`,
  );
});
