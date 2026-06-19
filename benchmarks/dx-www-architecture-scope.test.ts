import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  const fullPath = path.join(repoRoot, relativePath);
  assert.ok(fs.existsSync(fullPath), `expected ${relativePath} to exist`);
  return fs.readFileSync(fullPath, "utf8");
}

test("active launch docs keep DX-owned WWW scope instead of Next DevTools or Turbopack adoption", () => {
  const docs = {
    "DX.md": read("DX.md"),
    "TODO.md": read("TODO.md"),
    "CHANGELOG.md": read("CHANGELOG.md"),
  };

  assert.match(
    docs["DX.md"],
    /DX-WWW is a DX-owned WWW framework with Next-familiar\s+authoring\./,
  );
  assert.match(
    docs["TODO.md"],
    /External\s+framework debugging tool clone targets are out of scope\./,
  );
  assert.match(
    docs["CHANGELOG.md"],
    /Architecture Scope Reset/,
  );

  for (const [relativePath, contents] of Object.entries(docs)) {
    assert.doesNotMatch(
      contents,
      /^##\s+DX-WWW DevTools\b/m,
      `${relativePath} must not keep active DX-WWW DevTools target headings`,
    );
    assert.doesNotMatch(
      contents,
      /Next\s+DevTools\s+(runtime|clone|parity target)|Next\.js\s+DevTools\s+(runtime|clone target|parity target)|External\s+framework\s+DevTools\s+clone targets?|next-devtools|dev-overlay/,
      `${relativePath} must not advertise Next DevTools clone/parity work`,
    );
    assert.doesNotMatch(
      contents,
      /\/_dx\/devtools|dx\.devtools|DX_DEVTOOLS|dev::devtools/,
      `${relativePath} must point at DX feedback names instead of old devtools paths`,
    );
    assert.doesNotMatch(
      contents,
      /Turbopack powers|powers the build|Turbopack (powers|powering) (dx build|dx dev|the build)/i,
      `${relativePath} must not claim Turbopack powers DX build/dev`,
    );
  }

  const activeDocs = `${docs["DX.md"]}\n${docs["TODO.md"]}\n${docs["CHANGELOG.md"]}`;

  assert.match(activeDocs, /\/_dx\/feedback/);
  assert.match(activeDocs, /DX dev feedback/);
  assert.match(activeDocs, /hot reload/i);
  assert.match(activeDocs, /code frames/i);
  assert.match(activeDocs, /Runtime\/build adoption is not a\s+DX-WWW goal/);
  assert.match(activeDocs, /Turbopack does not power\s+`dx build` or `dx dev`/);
  assert.match(activeDocs, /Turbopack\s+reference\/provenance/);
});
