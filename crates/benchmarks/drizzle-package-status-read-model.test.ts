const assert = require("assert");
const crypto = require("crypto");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function sha256(relativePath) {
  return crypto.createHash("sha256").update(read(relativePath)).digest("hex");
}

test("Database ORM visibility is wired into the shared package-status read model", () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const receipt = readJson(
    "examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json",
  );
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const statusSource = read("examples/template/forge-package-status.ts");
  const packageDoc = read("docs/packages/db-drizzle-sqlite.md");
  const rustDxCheck = read(
    "core/src/ecosystem/project_check/database_orm_dx_check.rs",
  );

  const statusVocabulary = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
  ];
  const expectedHashManifest = {
    "core/src/ecosystem/forge_drizzle.rs":
      sha256("core/src/ecosystem/forge_drizzle.rs"),
    "examples/template/drizzle-query-proof.tsx":
      sha256("examples/template/drizzle-query-proof.tsx"),
    "examples/template/data-status.tsx":
      sha256("examples/template/data-status.tsx"),
    "tools/launch/runtime-template/pages/index.html":
      sha256("tools/launch/runtime-template/pages/index.html"),
    "docs/packages/database-orm.source-guard-runbook.json":
      sha256("docs/packages/database-orm.source-guard-runbook.json"),
    "tools/launch/materialize-www-template.ts":
      sha256("tools/launch/materialize-www-template.ts"),
    "tools/launch/runtime-template/assets/launch-runtime.ts":
      sha256("tools/launch/runtime-template/assets/launch-runtime.ts"),
    "examples/template/db/drizzle/schema.ts":
      sha256("examples/template/db/drizzle/schema.ts"),
    "examples/template/db/drizzle/metadata.ts":
      sha256("examples/template/db/drizzle/metadata.ts"),
    "examples/template/db/drizzle/README.md":
      sha256("examples/template/db/drizzle/README.md"),
    "examples/template/server/database-orm/readiness.ts":
      sha256("examples/template/server/database-orm/readiness.ts"),
    "examples/template/app/api/database-orm/readiness/route.ts":
      sha256("examples/template/app/api/database-orm/readiness/route.ts"),
  };
  const expectedTrackedFiles = Object.keys(expectedHashManifest);
  const expectedReceiptHashRefresh = {
    schema: "dx.forge.package.receipt_hash_refresh",
    status: "current",
    helper_path: "examples/template/database-orm-receipt-hashes.ts",
    check_command:
      "node tools/launch/run-template-receipt-helper.js examples/template/database-orm-receipt-hashes.ts --check",
    write_command:
      "node tools/launch/run-template-receipt-helper.js examples/template/database-orm-receipt-hashes.ts --write",
    json_check_command:
      "node tools/launch/run-template-receipt-helper.js examples/template/database-orm-receipt-hashes.ts --check --json",
    source_guard_runbook_fixture:
      "docs/packages/database-orm.source-guard-runbook.json",
    preview_manifest_materializer:
      "tools/launch/materialize-www-template.ts",
    receipt_path:
      "examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json",
    hash_algorithm: "sha256",
    tracked_file_count: expectedTrackedFiles.length,
    stale_file_count: 0,
    missing_file_count: 0,
    tracked_files: expectedTrackedFiles,
    current_files: expectedTrackedFiles,
    stale_files: [],
    missing_files: [],
    stale_mirror_files: [],
    missing_mirror_files: [],
    mirror_problem_count: 0,
    runtime_execution: false,
    secret_access: false,
    zed_visibility: "database-orm:receipt-hash-refresh",
    runtime_limitations: [
      "SOURCE-ONLY: this helper checks local Database ORM receipt hash freshness only.",
      "ADAPTER-BOUNDARY: SQLite files, better-sqlite3 runtime installation, migration rollout, authorization, and replica health stay app-owned.",
    ],
  };
  const expectedDxStyleCompatibility = {
    schema: "dx.forge.package.dx_style_compatibility",
    status: "present",
    token_source: "examples/template/styles/globals.css",
    generated_css: "examples/template/styles/globals.css",
    visible_surfaces: ["launch-drizzle-data-workflow"],
    source_files: [
      "examples/template/drizzle-query-proof.tsx",
      "examples/template/styles/globals.css",
    ],
    receipt_path:
      "examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json",
    runtime_proof: false,
    runtime_limitations: [
      "source-only dx-style token compatibility; no browser visual proof is claimed",
      "live SQLite data rendering remains app-owned",
      "theme token review remains app-owned",
    ],
  };

  const databaseVisibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "db/drizzle-sqlite",
  );

  assert.ok(databaseVisibility, "Database ORM visibility row is missing");
  assert.strictEqual(databaseVisibility.official_package_name, "Database ORM");
  assert.strictEqual(databaseVisibility.upstream_package, "drizzle-orm");
  assert.strictEqual(databaseVisibility.upstream_version, "0.45.3");
  assert.strictEqual(databaseVisibility.status, "present");
  assert.strictEqual(databaseVisibility.receipt_status, "present");
  assert.deepStrictEqual(databaseVisibility.status_vocabulary, statusVocabulary);
  assert.strictEqual(
    databaseVisibility.package_receipt_path,
    "examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json",
  );
  assert.ok(
    databaseVisibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "drizzle-replica-routing" &&
        surface.files.includes("db/drizzle/replicas.ts") &&
        surface.source_markers.includes("data-dx-package=\"db/drizzle-sqlite\"") &&
        surface.hash_algorithm === "sha256" &&
        surface.file_hashes["core/src/ecosystem/forge_drizzle.rs"] ===
          expectedHashManifest["core/src/ecosystem/forge_drizzle.rs"],
    ),
    "Database ORM replica-routing visibility surface is missing",
  );
  assert.ok(
    databaseVisibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "drizzle-launch-dashboard-workflow" &&
        surface.files.includes("components/template-app/drizzle-query-proof.tsx") &&
        surface.source_markers.includes(
          "data-dx-component=\"launch-drizzle-data-workflow\"",
        ) &&
        surface.source_markers.includes(
          'data-dx-style-surface="database-orm"',
        ) &&
        surface.hash_algorithm === "sha256" &&
        surface.file_hashes["examples/template/drizzle-query-proof.tsx"] ===
          expectedHashManifest[
            "examples/template/drizzle-query-proof.tsx"
          ] &&
        surface.file_hashes["tools/launch/runtime-template/assets/launch-runtime.ts"] ===
          expectedHashManifest[
            "tools/launch/runtime-template/assets/launch-runtime.ts"
          ],
    ),
    "Database ORM launch dashboard workflow visibility surface is missing",
  );
  assert.ok(
    databaseVisibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "database-orm-source-guard-runbook" &&
        surface.files.includes(
          "docs/packages/database-orm.source-guard-runbook.json",
        ) &&
        surface.source_markers.includes("source_guard_runbook_index") &&
        surface.source_markers.includes(
          "database-orm-lower-dx-check-helper-freshness",
        ) &&
        surface.source_markers.includes(
          "database-orm:receipt-hash-refresh",
        ) &&
        surface.hash_algorithm === "sha256" &&
        surface.file_hashes[
          "docs/packages/database-orm.source-guard-runbook.json"
        ] ===
          expectedHashManifest[
            "docs/packages/database-orm.source-guard-runbook.json"
          ],
    ),
    "Database ORM source-guard runbook visibility surface is missing",
  );
  assert.ok(
    databaseVisibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "database-orm-preview-manifest-materializer" &&
        surface.files.includes(
          "tools/launch/materialize-www-template.ts",
        ) &&
        surface.source_markers.includes(
          "DATABASE_ORM_SOURCE_GUARD_RUNBOOK_FIXTURE",
        ) &&
        surface.source_markers.includes("sourceGuardRunbookFixtures") &&
        surface.source_markers.includes(
          "docs/packages/database-orm.source-guard-runbook.json",
        ) &&
        surface.source_markers.includes(
          "database-orm:receipt-hash-refresh",
        ) &&
        surface.hash_algorithm === "sha256" &&
        surface.file_hashes[
          "tools/launch/materialize-www-template.ts"
        ] ===
          expectedHashManifest[
            "tools/launch/materialize-www-template.ts"
          ],
    ),
    "Database ORM preview-manifest materializer visibility surface is missing",
  );
  assert.ok(
    databaseVisibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "database-orm-lock-backed-source" &&
        surface.files.includes("db/drizzle/schema.ts") &&
        surface.files.includes("db/drizzle/metadata.ts") &&
        surface.files.includes("db/drizzle/README.md") &&
        surface.files.includes("server/database-orm/readiness.ts") &&
        surface.files.includes("app/api/database-orm/readiness/route.ts") &&
        surface.source_markers.includes("dx.www.template.database_orm_readiness") &&
        surface.source_markers.includes("runtimeProof: false") &&
        surface.source_markers.includes("networkCalls: false") &&
        surface.source_markers.includes("hostedCredentials: false") &&
        surface.hash_algorithm === "sha256" &&
        surface.file_hashes["examples/template/db/drizzle/schema.ts"] ===
          expectedHashManifest["examples/template/db/drizzle/schema.ts"] &&
        surface.file_hashes["examples/template/server/database-orm/readiness.ts"] ===
          expectedHashManifest[
            "examples/template/server/database-orm/readiness.ts"
          ] &&
        surface.file_hashes[
          "examples/template/app/api/database-orm/readiness/route.ts"
        ] ===
          expectedHashManifest[
            "examples/template/app/api/database-orm/readiness/route.ts"
          ],
    ),
    "Database ORM lock-backed source visibility surface is missing",
  );

  for (const metric of [
    "database_orm_receipt_present",
    "database_orm_receipt_stale",
    "database_orm_missing_receipt",
    "database_orm_blocked_surface",
    "database_orm_unsupported_surface",
    "database_orm_hash_manifest_present",
    "database_orm_hash_mismatch",
    "database_orm_receipt_hash_refresh_current",
    "database_orm_receipt_hash_refresh_stale",
    "database_orm_receipt_hash_refresh_missing",
    "database_orm_dx_style_compatibility_present",
    "database_orm_dx_style_compatibility_missing",
  ]) {
    assert.ok(
      databaseVisibility.dx_check_metrics.includes(metric),
      `${metric} missing from Database ORM visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
  }

  assert.ok(
    status.zed_receipt_surfaces.includes("database-orm:drizzle-replica-routing"),
    "Database ORM replica routing surface is missing from Zed receipt surfaces",
  );
  assert.ok(
    status.zed_receipt_surfaces.includes(
      "database-orm:drizzle-launch-dashboard-workflow",
    ),
    "Database ORM dashboard workflow surface is missing from Zed receipt surfaces",
  );
  assert.ok(
    status.zed_receipt_surfaces.includes("database-orm:receipt-hash-refresh"),
    "Database ORM receipt hash refresh surface is missing from Zed receipt surfaces",
  );

  assert.match(readModel, /export const databaseOrmPackageVisibility/);
  assert.deepStrictEqual(
    databaseVisibility.receipt_hash_refresh,
    expectedReceiptHashRefresh,
  );
  assert.match(readModel, /receiptHashRefresh: \{/);
  assert.match(
    readModel,
    /sourceGuardRunbookFixture: "docs\/packages\/database-orm\.source-guard-runbook\.json"/,
  );
  assert.match(
    readModel,
    /previewManifestMaterializer: "tools\/launch\/materialize-www-template\.ts"/,
  );
  assert.match(readModel, /currentFiles: \[/);
  assert.match(readModel, /staleMirrorFiles: \[/);
  assert.match(readModel, /mirrorProblemCount: 0/);
  assert.match(readModel, /database-orm:receipt-hash-refresh/);
  assert.match(readModel, /database-orm-source-guard-runbook/);
  assert.match(readModel, /database-orm-preview-manifest-materializer/);
  assert.match(readModel, /database-orm-lock-backed-source/);
  assert.match(readModel, /app\/api\/database-orm\/readiness\/route\.ts/);
  assert.match(readModel, /server\/database-orm\/readiness\.ts/);
  assert.match(readModel, /db\/drizzle\/metadata\.ts/);
  assert.match(readModel, /db\/drizzle\/README\.md/);
  assert.match(readModel, /dxStyleCompatibility: \{/);
  assert.match(
    readModel,
    /packageLaneVisibility:\s*\[[\s\S]*databaseOrmPackageVisibility[\s\S]*\]/,
  );
  assert.match(statusSource, /databaseOrmPackageVisibility/);
  assert.match(statusSource, /databaseOrmVisibility: databaseOrmPackageVisibility/);
  assert.match(packageDoc, /shared dx-check\/Zed package-status read model/i);
  assert.match(packageDoc, /database-orm-receipt-hashes\.ts/);
  assert.match(packageDoc, /receipt_hash_refresh/);
  assert.match(packageDoc, /## DX-Style Compatibility/);
  assert.match(rustDxCheck, /use sha2::\{Digest, Sha256\}/);
  assert.match(rustDxCheck, /database_orm_hash_manifest_present/);
  assert.match(rustDxCheck, /database_orm_hash_mismatch/);
  assert.match(rustDxCheck, /database-orm-hash-mismatch/);
  assert.match(rustDxCheck, /database_orm_receipt_hash_refresh_current/);
  assert.match(rustDxCheck, /database_orm_receipt_hash_refresh_stale/);
  assert.match(rustDxCheck, /database_orm_receipt_hash_refresh_missing/);
  assert.match(rustDxCheck, /fn receipt_hash_refresh_counts/);
  assert.match(
    rustDxCheck,
    /fn database_orm_hash_refresh_stale_helper_keeps_source_hash_clean/,
  );
  assert.match(rustDxCheck, /database_orm_dx_style_compatibility_present/);
  assert.match(rustDxCheck, /database_orm_dx_style_compatibility_missing/);
  assert.match(rustDxCheck, /database-orm-missing-dx-style-compatibility/);
  assert.match(rustDxCheck, /fn dx_style_compatibility_is_present/);
  assert.match(rustDxCheck, /fn database_orm_dx_style_compatibility_missing_is_reported/);
  assert.match(rustDxCheck, /fn sha256_project_file/);

  assert.strictEqual(receipt.official_package_name, "Database ORM");
  assert.strictEqual(receipt.hash_algorithm, "sha256");
  assert.deepStrictEqual(receipt.file_hashes, expectedHashManifest);
  assert.deepStrictEqual(
    databaseVisibility.dx_style_compatibility,
    expectedDxStyleCompatibility,
  );
  assert.deepStrictEqual(
    receipt.dx_style_compatibility,
    expectedDxStyleCompatibility,
  );
  assert.strictEqual(
    receipt.dx_check_visibility.schema,
    "dx.forge.package.dx_check_visibility",
  );
  assert.deepStrictEqual(
    receipt.dx_check_visibility.status_legend.map((entry) => entry.status),
    statusVocabulary,
  );
});
