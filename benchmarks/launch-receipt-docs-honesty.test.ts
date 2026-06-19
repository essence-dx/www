import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const launchStatusDocs = ["README.md", "DX.md", "TODO.md", "CHANGELOG.md"];

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("launch receipt docs state current runner-backed helper truth", () => {
  const staleReceiptCopy =
    /Several package receipts still report honest stale hashes|stale hashes from active source edits|not a blanket receipt freshness claim/i;

  for (const relativePath of launchStatusDocs) {
    assert.doesNotMatch(
      read(relativePath),
      staleReceiptCopy,
      `${relativePath} must not preserve stale receipt-helper launch copy`,
    );
  }

  const docs = launchStatusDocs.map(read).join("\n");
  assert.match(docs, /tools\/launch\/run-template-receipt-helper\.js/);
  assert.match(docs, /20\s+package\s+`\*-receipt-hash-refresh\.test\.ts`\s+files/i);
  assert.match(docs, /no browser proof|browser proof.*remain/i);
  assert.match(docs, /live provider|provider execution|provider.*remain/i);
});
