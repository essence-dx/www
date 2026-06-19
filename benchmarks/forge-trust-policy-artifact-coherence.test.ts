import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");

function read(relativePath) {
  return readFileSync(join(repoRoot, relativePath), "utf8");
}

function trustDecision({ policyPresent, receiptFindings, statusReferences }) {
  if (!policyPresent) {
    return "manual review";
  }
  if (statusReferences.some((reference) => !reference.present)) {
    return "blocked";
  }
  if (receiptFindings.some((finding) => finding === "red")) {
    return "blocked";
  }
  if (receiptFindings.some((finding) => finding === "manual-review")) {
    return "manual review";
  }
  return "allowed";
}

test("Forge trust policy docs require authority agreement before materialization", () => {
  const model = read("docs/forge-import-security-model.md");
  const plan = read("docs/superpowers/plans/2026-06-07-forge-source-package-imports.md");

  for (const phrase of [
    "source manifest",
    "package lock",
    "receipts",
    "trust policy",
    "remote status",
    "current bytes",
  ]) {
    assert.match(model, new RegExp(phrase));
  }

  assert.match(model, /A trust policy allows materialization/);
  assert.match(model, /unresolved red or\s+manual-review findings/);
  assert.match(plan, /Evidence Coherence And Security Docs/);
});

test("trust decisions block missing policy references and red findings", () => {
  assert.equal(
    trustDecision({
      policyPresent: false,
      receiptFindings: [],
      statusReferences: [],
    }),
    "manual review",
  );
  assert.equal(
    trustDecision({
      policyPresent: true,
      receiptFindings: [],
      statusReferences: [{ path: ".dx/forge/package-lock.json", present: false }],
    }),
    "blocked",
  );
  assert.equal(
    trustDecision({
      policyPresent: true,
      receiptFindings: ["red"],
      statusReferences: [{ path: ".dx/forge/package-lock.json", present: true }],
    }),
    "blocked",
  );
  assert.equal(
    trustDecision({
      policyPresent: true,
      receiptFindings: ["manual-review"],
      statusReferences: [{ path: ".dx/forge/package-lock.json", present: true }],
    }),
    "manual review",
  );
  assert.equal(
    trustDecision({
      policyPresent: true,
      receiptFindings: [],
      statusReferences: [{ path: ".dx/forge/package-lock.json", present: true }],
    }),
    "allowed",
  );
});
