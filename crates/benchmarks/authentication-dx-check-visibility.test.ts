const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const visibilitySchema = "dx.forge.package.dx_check_visibility";
const statuses = [
  "present",
  "stale",
  "missing-receipt",
  "blocked",
  "unsupported-surface",
];

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

test("Authentication exposes dx-check visibility for receipt-backed surfaces", () => {
  const docs = read("docs/packages/authentication.md");
  const catalog = read("examples/template/package-catalog.ts");
  const registry = read("core/src/ecosystem/forge_registry.rs");
  const cli = read("dx-www/src/cli/mod.rs");
  const dashboardPackage = read(
    "examples/dashboard/src/lib/forge/auth/better-auth/dashboard.ts",
  );
  const receipt = readJson(
    "examples/template/.dx/forge/receipts/auth-better-auth.json",
  );

  assert.equal(receipt.dx_check_visibility.schema, visibilitySchema);
  assert.equal(receipt.dx_check_visibility.package_id, "auth/better-auth");
  assert.equal(receipt.dx_check_visibility.official_package_name, "Authentication");
  assert.equal(receipt.dx_check_visibility.current_status, "present");
  assert.deepEqual(
    receipt.dx_check_visibility.status_legend.map((entry) => entry.status),
    statuses,
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "authentication-account-workflow" &&
        surface.status === "present" &&
        surface.source_file === "examples/template/template-shell.tsx" &&
        surface.materialized_file === "components/template-app/template-shell.tsx",
    ),
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "authentication-session-status" &&
        surface.status === "present" &&
        surface.source_file === "examples/template/auth-session-status.tsx" &&
        surface.materialized_file ===
          "components/template-app/auth-session-status.tsx",
    ),
  );

  assert.match(docs, /## dx-check visibility/);
  assert.match(docs, new RegExp(visibilitySchema));
  for (const status of statuses) {
    assert.match(docs, new RegExp("`" + status + "`"));
    assert.match(catalog, new RegExp(`"${status}"`));
    assert.match(registry, new RegExp(`"${status}"`));
    assert.match(cli, new RegExp(`"${status}"`));
  }

  assert.match(catalog, /dxCheckVisibility: \{/);
  assert.match(
    catalog,
    /receiptPath:\s*"examples\/template\/\.dx\/forge\/receipts\/auth-better-auth\.json"/,
  );
  assert.match(catalog, /"authentication-account-workflow"/);
  assert.match(catalog, /"authentication-session-status"/);

  assert.match(registry, /dxCheckVisibility: \{/);
  assert.match(registry, /schema: "dx\.forge\.package\.dx_check_visibility"/);
  assert.match(registry, /"authentication-account-workflow"/);

  assert.match(dashboardPackage, /dxCheckVisibility: \{/);
  assert.match(dashboardPackage, /schema: 'dx\.forge\.package\.dx_check_visibility'/);
  assert.match(dashboardPackage, /'authentication-session-status'/);

  assert.equal(
    receipt.dx_style_compatibility.schema,
    "dx.forge.package.dx_style_compatibility",
  );
  assert.equal(receipt.dx_style_compatibility.status, "present");
  assert.deepEqual(receipt.dx_style_compatibility.visible_surfaces, [
    "authentication-account-workflow",
    "authentication-session-status",
  ]);
  assert.ok(
    receipt.dx_style_compatibility.source_files.includes(
      "examples/template/template-shell.tsx",
    ),
  );
  assert.ok(
    receipt.dx_style_compatibility.source_files.includes(
      "examples/template/auth-session-status.tsx",
    ),
  );
  assert.ok(
    receipt.dx_style_compatibility.data_dx_markers.includes(
      'data-dx-style-surface="authentication-session-status"',
    ),
  );

  assert.match(cli, /"dx_check_visibility": \{/);
  assert.match(cli, /"schema": "dx\.forge\.package\.dx_check_visibility"/);
  assert.match(cli, /"monitored_surfaces": \[/);
});
