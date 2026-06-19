const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  const absolutePath = path.join(root, relativePath);
  assert.ok(fs.existsSync(absolutePath), `missing ${relativePath}`);
  return fs.readFileSync(absolutePath, "utf8");
}

function assertIncludesAll(source, values, label) {
  for (const value of values) {
    assert.ok(source.includes(value), `${label} missing ${value}`);
  }
}

test("public docs expose a stable WWW API contract for outside developers", () => {
  const readme = read("README.md");
  const versioning = read("docs/api/versioning.md");
  const developerContract = read("docs/dx-www-developer-contract.md");
  const agents = read("AGENTS.md");
  const combined = `${readme}\n${versioning}\n${developerContract}\n${agents}`;

  assertIncludesAll(
    combined,
    [
      "Public API Stability",
      "Stable Public API",
      "Stable public surfaces include",
      "Compatibility rule",
      "React hook-like APIs are not separate public runtime",
      "Silent no-op compatibility APIs are not part of",
      "silent no-op hooks",
    ],
    "public stability wording",
  );

  assertIncludesAll(
    versioning,
    [
      "Stable Public Surfaces",
      "Compatibility Boundaries",
      "Deprecation Process",
      "Public Proof Requirements",
      "Devtools contract: dev-only injection",
      "production `dx build` output must stay Devtools-free",
    ],
    "versioning policy",
  );

  assertIncludesAll(
    combined,
    [
      "dx new",
      "dx dev",
      "dx build",
      "dx check",
      "dx www agent-context",
      "dx style",
      "dx icons",
      "dx imports",
    ],
    "stable commands",
  );

  assertIncludesAll(
    combined,
    [
      "app/",
      "components/",
      "composables/",
      "utils/",
      "lib/",
      "lib/stores/",
      "server/",
      "styles/",
      "public/",
      ".dx/",
      "extensionless root `dx` file",
    ],
    "stable project folders",
  );

  assertIncludesAll(
    combined,
    [
      "onClick",
      "onInput",
      "clientLoad",
      "clientVisible",
      "clientIdle",
      "clientOnly",
      "state",
      "derived",
      "effect",
      "action",
    ],
    "stable TSX and state syntax",
  );

  assertIncludesAll(
    combined,
    [
      ".dx/receipts/**",
      ".sr",
      ".machine",
      ".dx/www/output",
      "dx www agent-context --json",
    ],
    "stable proof surfaces",
  );
});
