const assert = require("assert");
const crypto = require("crypto");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function sha256(relativePath) {
  return crypto.createHash("sha256").update(read(relativePath)).digest("hex");
}

test("Database ORM receipt exposes dx-check visibility states and official naming", () => {
  const receipt = readJson(receiptPath);
  const forge = read("core/src/ecosystem/forge_drizzle.rs");
  const packageCatalog = read("examples/template/package-catalog.ts");
  const packageDoc = read("docs/packages/db-drizzle-sqlite.md");
  const launchWorkflow = read("examples/template/drizzle-query-proof.tsx");

  assert.equal(receipt.package_id, "db/drizzle-sqlite");
  assert.equal(receipt.package_name, "Database ORM");
  assert.equal(receipt.official_package_name, "Database ORM");
  assert.equal(receipt.upstream_package, "drizzle-orm");
  assert.equal(receipt.upstream_version, "0.45.3");
  assert.equal(receipt.honesty_label, "SOURCE-ONLY");
  assert.equal(receipt.hash_algorithm, "sha256");
  assert.equal(
    receipt.file_hashes["core/src/ecosystem/forge_drizzle.rs"],
    sha256("core/src/ecosystem/forge_drizzle.rs"),
  );
  assert.equal(
    receipt.file_hashes["examples/template/drizzle-query-proof.tsx"],
    sha256("examples/template/drizzle-query-proof.tsx"),
  );
  assert.ok(receipt.materialized_files.includes("db/drizzle/replicas.ts"));
  assert.ok(receipt.upstream_public_apis.includes("withReplicas"));
  assert.ok(receipt.upstream_public_apis.includes("selectDistinct"));
  assert.ok(receipt.upstream_public_apis.includes("$count"));
  assert.ok(
    receipt.guards.includes(
      "node --test .\\benchmarks\\drizzle-sqlite-replica-routing-slice.test.ts",
    ),
  );
  assert.ok(
    receipt.guards.includes(
      "node --test .\\benchmarks\\drizzle-receipt-hash-refresh.test.ts",
    ),
  );

  assert.equal(
    receipt.dx_check_visibility.schema,
    "dx.forge.package.dx_check_visibility",
  );
  assert.equal(receipt.dx_check_visibility.current_status, "present");
  assert.deepEqual(
    receipt.dx_check_visibility.status_legend.map((entry) => entry.status),
    ["present", "stale", "missing-receipt", "blocked", "unsupported-surface"],
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "drizzle-replica-routing" &&
        surface.status === "present" &&
        surface.source_file === "core/src/ecosystem/forge_drizzle.rs" &&
        surface.materialized_file === "db/drizzle/replicas.ts" &&
        surface.hash_algorithm === "sha256" &&
        surface.file_hashes["core/src/ecosystem/forge_drizzle.rs"],
    ),
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "drizzle-launch-dashboard-workflow" &&
        surface.status === "present" &&
        surface.materialized_file === "components/template-app/drizzle-query-proof.tsx" &&
        surface.hash_algorithm === "sha256" &&
        surface.file_hashes["tools/launch/runtime-template/pages/index.html"],
    ),
  );
  assert.equal(
    receipt.dx_style_compatibility.schema,
    "dx.forge.package.dx_style_compatibility",
  );
  assert.equal(receipt.dx_style_compatibility.status, "present");
  assert.equal(
    receipt.dx_style_compatibility.token_source,
    "examples/template/styles/globals.css",
  );
  assert.equal(
    receipt.dx_style_compatibility.generated_css,
    "examples/template/styles/globals.css",
  );
  assert.equal(receipt.dx_style_compatibility.runtime_proof, false);
  assert.ok(
    receipt.dx_style_compatibility.visible_surfaces.includes(
      "launch-drizzle-data-workflow",
    ),
  );
  assert.match(
    receipt.dx_style_compatibility.runtime_limitations.join(" "),
    /source-only dx-style token compatibility/i,
  );

  assert.match(forge, /dxCheckVisibility: \{/);
  assert.match(forge, /dxStyleCompatibility: \{/);
  assert.match(forge, /schema: "dx\.forge\.package\.dx_check_visibility"/);
  assert.match(
    forge,
    /schema: "dx\.forge\.package\.dx_style_compatibility"/,
  );
  assert.match(forge, /currentStatus: "present"/);
  assert.match(forge, /"missing-receipt"/);
  assert.match(forge, /"unsupported-surface"/);

  assert.match(packageCatalog, /dxCheckVisibility: \{/);
  assert.match(packageCatalog, /dxStyleCompatibility: \{/);
  assert.match(packageCatalog, /receiptPath:\s*"examples\/template\/\.dx\/forge\/receipts\/2026-05-22-db-drizzle-sqlite-dashboard-workflow\.json"/);
  assert.match(packageCatalog, /docs\/packages\/database-orm\.mirror-drift\.fixture\.json/);
  assert.match(packageCatalog, /monitoredSurfaces:\s*\[/);
  assert.match(packageCatalog, /db\/drizzle\/replicas\.ts/);

  assert.match(packageDoc, /dx-check visibility/i);
  assert.match(packageDoc, /## DX-Style Compatibility/);
  assert.match(packageDoc, /present, stale, missing-receipt, blocked, and unsupported-surface/);
  assert.match(launchWorkflow, /data-dx-style-surface="database-orm"/);
});
