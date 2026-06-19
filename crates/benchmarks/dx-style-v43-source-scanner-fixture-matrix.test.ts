import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const matrixPath = path.join(
  root,
  "related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json",
);
const publicToolsPath = path.join(root, "dx-www/src/cli/public_framework_tools.rs");

function readJson(filePath) {
  assert.ok(fs.existsSync(filePath), `expected ${path.relative(root, filePath)} to exist`);
  return JSON.parse(fs.readFileSync(filePath, "utf8"));
}

test("dx-style v4.3 source scanner tracks Tailwind plain-text detection canaries", () => {
  const matrix = readJson(matrixPath);

  assert.ok(
    matrix.sources.some(
      (source) => source.url === "https://tailwindcss.com/docs/detecting-classes-in-source-files",
    ),
    "matrix should cite Tailwind's official class detection docs",
  );

  assert.equal(matrix.sourceScannerCanaries?.fullTailwindSourceDetectionParity, false);
  assert.match(matrix.sourceScannerCanaries?.scope ?? "", /plain-text/i);

  const canaries = new Map(
    (matrix.sourceScannerCanaries?.canaries ?? []).map((entry) => [entry.id, entry]),
  );
  for (const [id, requiredTokens] of [
    ["tsx-static-object-map", ["bg-blue-600", "hover:bg-blue-500", "text-white"]],
    [
      "arbitrary-value-static-string",
      ["grid-cols-[repeat(auto-fit,minmax(12rem,1fr))]", "[--card-size:theme(spacing.4)]"],
    ],
    ["template-interpolation-rejection", ["bg-${color}-600"]],
    ["css-source-inline-bridge", ["@source inline(\"{hover:,focus:,}underline\")"]],
  ]) {
    const entry = canaries.get(id);
    assert.ok(entry, `source scanner canaries should include ${id}`);
    for (const token of requiredTokens) {
      assert.ok(
        entry.tokens?.includes(token) || entry.rejectedTokens?.includes(token),
        `${id} should document ${token}`,
      );
    }
  }
});

test("dx-style scanner implementation has source-owned plain-text token extraction", () => {
  const source = fs.readFileSync(publicToolsPath, "utf8");

  for (const marker of [
    "extract_tailwind_plain_text_class_tokens",
    "is_tailwind_plain_text_class_candidate",
    "TAILWIND_PLAIN_TEXT_SINGLE_WORD_UTILITIES",
    "extract_class_tokens_detects_tailwind_plain_text_static_strings",
  ]) {
    assert.match(source, new RegExp(marker));
  }
});
