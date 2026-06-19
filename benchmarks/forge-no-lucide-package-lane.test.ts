const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("Lucide React is not registered as a standalone Forge package lane", () => {
  const forbidden = [
    "icons/lucide-react",
    "dx add lucide --write",
    "forge add lucide",
    "forge_lucide_react",
    "lucide_react_templates",
  ];
  const sourceFiles = [
    "examples/template/package-catalog.ts",
    "examples/template/template-surface-registry.ts",
    "examples/template/framework-completeness.ts",
    "core/src/ecosystem/mod.rs",
    "core/src/ecosystem/forge_registry.rs",
    "core/src/ecosystem/forge_scorecard.rs",
    "core/src/ecosystem/forge_security.rs",
    "core/src/ecosystem/forge_trust_policy.rs",
    "dx-www/src/cli/mod.rs",
    "dx-www/src/cli/studio_manifest.rs",
    "dx-www/src/cli/launch_readiness_bundle.rs",
  ];

  for (const relativePath of sourceFiles) {
    const source = read(relativePath);
    for (const marker of forbidden) {
      assert.equal(
        source.includes(marker),
        false,
        `${relativePath} still contains ${marker}`,
      );
    }
  }

  assert.equal(
    fs.existsSync(path.join(root, "core/src/ecosystem/forge_lucide_react.rs")),
    false,
    "the removed standalone Lucide Forge package module should not exist",
  );
});
