const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const upstream = path.resolve(root, "..", "..", "WWW/inspirations/stripe-js");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function readUpstream(relativePath) {
  return fs.readFileSync(path.join(upstream, relativePath), "utf8");
}

function escaped(marker) {
  return new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
}

function matchSection(source, pattern, label) {
  const match = source.match(pattern);
  assert.ok(match, `missing ${label}`);
  return match[0];
}

test("Payments package-lane row exposes receipt helper freshness in dx-check panel", () => {
  const upstreamPackage = JSON.parse(readUpstream("package.json"));
  const upstreamPure = readUpstream("src/pure.ts");
  const upstreamStripeTypes = readUpstream("types/stripe-js/stripe.d.ts");
  const upstreamEmbeddedCheckoutTypes = readUpstream(
    "types/stripe-js/embedded-checkout.d.ts",
  );
  const reader = read("core/src/ecosystem/dx_check_receipt.rs");
  const launchPage = read("tools/launch/runtime-template/pages/index.html");
  const studioContract = read(
    "examples/template/dx-studio-edit-contract.ts",
  );
  const materializer = read("tools/launch/materialize-www-template.ts");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const packageDoc = read("docs/packages/payments-stripe-js.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.equal(upstreamPackage.name, "@stripe/stripe-js");
  assert.equal(upstreamPackage.version, "9.6.0");
  assert.match(upstreamPure, /export const loadStripe/);
  assert.match(upstreamPure, /loadStripe\.setLoadParameters/);
  assert.match(upstreamStripeTypes, /confirmPayment/);
  assert.match(upstreamStripeTypes, /retrievePaymentIntent/);
  assert.match(upstreamStripeTypes, /createEmbeddedCheckoutPage/);
  assert.match(upstreamEmbeddedCheckoutTypes, /fetchClientSecret/);

  for (const marker of [
    'PAYMENTS_PACKAGE_ID: &str = "payments/stripe-js"',
    'PAYMENTS_OFFICIAL_NAME: &str = "Payments"',
    'PAYMENTS_UPSTREAM_PACKAGE: &str = "@stripe/stripe-js"',
    'PAYMENTS_UPSTREAM_VERSION: &str = "9.6.0"',
    'PAYMENTS_SOURCE_MIRROR: &str = "G:/WWW/inspirations/stripe-js"',
    'PAYMENTS_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json"',
    'PAYMENTS_PACKAGE_RECEIPT_PATH: &str = "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json"',
    "rows.extend(payments_package_lane_row(root, package_status));",
    "fn payments_package_lane_row(",
    "receipt_hash_refresh: receipt_hash_refresh.clone(),",
    "payments_hash_manifest_present",
    "payments_hash_mismatch",
    "payments_receipt_hash_refresh_current",
    "payments_receipt_hash_refresh_stale",
    "payments_receipt_hash_refresh_missing",
    "payments_dx_style_compatibility_present",
    "payments_dx_style_compatibility_missing",
    "payments:receipt-hash-refresh",
    "dx_check_latest_panel_exposes_payments_package_lane_hash_refresh_row",
    "dx_check_latest_panel_exposes_payments_stale_helper_without_source_hash_drift",
  ]) {
    assert.match(reader, escaped(marker), `missing check-panel marker ${marker}`);
  }
  assert.match(
    reader,
    /payments_next_action\s*\(\s*status,\s*receipt_hash_refresh\.as_ref\(\),\s*dx_style_compatibility_missing,?\s*\)/,
  );

  for (const marker of [
    'data-dx-check-package-lane-template="payments/stripe-js"',
    'data-dx-check-package-lane-row="payments/stripe-js"',
    'data-dx-check-package-lane-name="Payments"',
    'data-dx-check-package-lane-upstream-package="@stripe/stripe-js"',
    'data-dx-check-package-lane-source-mirror="G:/WWW/inspirations/stripe-js"',
    'data-dx-check-package-lane-receipt-path="examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json"',
    'data-dx-check-package-lane-dx-style-status="present"',
    'data-dx-check-package-lane-hash-refresh-status="current"',
    'data-dx-check-package-lane-hash-refresh-helper="examples/template/payments-receipt-hashes.ts"',
    'data-dx-check-package-lane-hash-refresh-json-command="node tools/launch/run-template-receipt-helper.js examples/template/payments-receipt-hashes.ts --check --json"',
    'data-dx-check-package-lane-hash-refresh-zed="payments:receipt-hash-refresh"',
    'data-dx-check-package-lane-hash-refresh-tracked-files="3"',
    'data-dx-check-package-lane-hash-refresh-stale-files="0"',
    'data-dx-check-package-lane-hash-refresh-missing-files="0"',
    'data-dx-style-surface="payments"',
    'data-dx-token-scope="payments/stripe-js"',
  ]) {
    assert.match(launchPage, escaped(marker), `missing static launch marker ${marker}`);
  }

  const studioDxCheckSurface = matchSection(
    studioContract,
    /id: "dx-check-health-panel"[\s\S]*?noNodeModulesRequired: true,/,
    "Payments Studio dx-check edit surface",
  );
  assert.match(studioDxCheckSurface, /"payments\/stripe-js"/);

  const materializerDxCheckSurface = matchSection(
    materializer,
    /"launch-runtime-dx-check-panel"[\s\S]*?"pages\/index\.html"/,
    "Payments materialized dx-check surface",
  );
  assert.match(materializerDxCheckSurface, /"payments\/stripe-js"/);

  const rustStudioDxCheckSurface = matchSection(
    studioManifest,
    /fn studio_dx_check_edit_surface\(\) -> Value \{[\s\S]*?&\["move_reorder_section", "update_text_content"\]/,
    "Payments Rust Studio dx-check edit surface",
  );
  assert.match(rustStudioDxCheckSurface, /"payments\/stripe-js"/);

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /DX Studio\/check-panel Payments package row/);
    assert.match(source, /receipt_hash_refresh/);
    assert.match(source, /payments:receipt-hash-refresh/);
    assert.match(
      source,
      /without claiming live Stripe Checkout or webhook runtime proof/,
    );
  }
});

test("Payments package-lane row survives generated starter materialization", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-payments-package-lane-"));
  const materializer = path.join(
    root,
    "tools",
    "launch",
    "materialize-www-template.ts",
  );

  const result = JSON.parse(
    execFileSync(process.execPath, [materializer, dir], {
      cwd: root,
      encoding: "utf8",
    }),
  );
  const launch = fs.readFileSync(path.join(dir, "pages", "index.html"), "utf8");
  const manifest = JSON.parse(
    fs.readFileSync(path.join(dir, "public", "preview-.dx/build-cache/manifest.json"), "utf8"),
  );

  assert.equal(result.ok, true);
  assert.equal(result.noNodeModules, true);

  for (const marker of [
    'data-dx-check-package-lane-template="payments/stripe-js"',
    'data-dx-check-package-lane-row="payments/stripe-js"',
    'data-dx-check-package-lane-name="Payments"',
    'data-dx-check-package-lane-hash-refresh-helper="examples/template/payments-receipt-hashes.ts"',
    'data-dx-check-package-lane-hash-refresh-zed="payments:receipt-hash-refresh"',
    'data-dx-style-surface="payments"',
    'data-dx-package="payments/stripe-js"',
  ]) {
    assert.match(launch, escaped(marker), `missing generated launch marker ${marker}`);
  }

  const launchRoute = manifest.routes.find((route) => route.route === "/");
  assert.ok(launchRoute, "expected materialized /launch route metadata");
  assert.ok(launchRoute.forgePackages.includes("payments/stripe-js"));

  const checkPanel = manifest.editContract.editableSurfaces.find(
    (surface) => surface.id === "launch-runtime-dx-check-panel",
  );
  assert.ok(checkPanel, "expected dx-check panel edit surface");
  assert.equal(checkPanel.sourceFile, "pages/index.html");
  assert.ok(
    checkPanel.packageIds.includes("payments/stripe-js"),
    "generated dx-check panel package scope must include Payments",
  );
  assert.ok(
    checkPanel.stateMarkers.includes("data-dx-check-package-lane-hash-refresh-helper"),
  );
  assert.ok(
    checkPanel.stateMarkers.includes("data-dx-check-package-lane-hash-refresh-zed"),
  );

  assert.ok(
    Array.isArray(manifest.sourceGuardRunbookFixtures),
    "generated preview manifest must expose source-guard runbook fixtures",
  );
  const paymentsRunbookFixture = manifest.sourceGuardRunbookFixtures.find(
    (fixture) => fixture.packageId === "payments/stripe-js",
  );
  assert.ok(
    paymentsRunbookFixture,
    "generated preview manifest must expose the Payments source-guard runbook fixture",
  );
  assert.equal(paymentsRunbookFixture.officialPackageName, "Payments");
  assert.equal(
    paymentsRunbookFixture.fixture,
    "docs/packages/payments.source-guard-runbook.json",
  );
  assert.equal(
    paymentsRunbookFixture.guardId,
    "payments-generated-starter-materialization",
  );
  assert.equal(paymentsRunbookFixture.route, "/");
  assert.equal(paymentsRunbookFixture.honestyLabel, "SOURCE-ONLY");
  assert.equal(paymentsRunbookFixture.runtimeProof, false);
  assert.equal(paymentsRunbookFixture.zedVisibility, "payments:receipt-hash-refresh");

  assert.ok(
    launchRoute.sourceGuardRunbookFixtures.includes(
      "docs/packages/payments.source-guard-runbook.json",
    ),
    "the materialized /launch route must point at the Payments runbook fixture",
  );

  for (const source of [
    read("docs/packages/payments-stripe-js.md"),
    read("DX.md"),
    read("TODO.md"),
    read("CHANGELOG.md"),
  ]) {
    assert.match(source, /generated-starter materialization guard for Payments/);
    assert.match(source, /without claiming live Stripe Checkout or webhook runtime proof/);
  }
});

test("Payments generated-starter guard is discoverable from the Studio source runbook", () => {
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const frameworkStructure = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const packageDoc = read("docs/packages/payments-stripe-js.md");

  assert.match(
    studioManifest,
    /studio_source_guard\(\s*"payments-generated-starter-materialization"/,
    "Rust Studio manifest must publish the Payments generated-starter source guard",
  );
  assert.match(
    studioManifest,
    /"payments-generated-starter-materialization"[\s\S]*"benchmarks\/payments-dx-check-package-lane-panel\.test\.ts"/,
    "Payments source guard must point at the focused lane benchmark",
  );
  assert.match(
    studioManifest,
    /"payments-generated-starter-materialization"[\s\S]*"dx run --test \.\\\\benchmarks\\\\payments-dx-check-package-lane-panel\.test\.ts"/,
    "Payments source guard must expose the exact lightweight command",
  );
  assert.match(
    studioManifest,
    /"payments-generated-starter-materialization"[\s\S]*"Payments generated-starter materialization guard"/,
  );
  assert.match(
    studioManifest,
    /"payments-generated-starter-materialization"[\s\S]*"data-dx-check-package-lane-row=\\\"payments\/stripe-js\\\""/,
  );
  assert.match(
    studioManifest,
    /"payments-generated-starter-materialization"[\s\S]*"data-dx-token-scope=\\\"payments\/stripe-js\\\""/,
  );
  assert.match(
    studioManifest,
    /"payments-generated-starter-materialization"[\s\S]*"payments:receipt-hash-refresh"/,
  );
  assert.match(
    studioManifest,
    /"\/" => guards\.extend\(\[[\s\S]*"payments-generated-starter-materialization"/,
    "the /launch source guard runbook must list the Payments generated-starter guard",
  );
  assert.match(
    studioManifest,
    /"payments-generated-starter-materialization"[\s\S]*"The generated starter preserves the Payments package-lane row, helper freshness markers, and package-scoped dx-check panel without live Stripe Checkout or webhook runtime proof\."/,
  );
  assert.match(
    studioManifest,
    /"dx run --test \.\\\\benchmarks\\\\payments-dx-check-package-lane-panel\.test\.ts"[\s\S]*"Validate the source-only Payments package-lane row, generated starter materialization, helper freshness markers, and dx-check panel package scope without live Stripe Checkout or webhook runtime proof\."/,
  );
  assert.match(
    studioManifest,
    /studio_source_guard_with_fixture\(\s*"payments-lower-dx-check-helper-freshness"[\s\S]*"core\/src\/ecosystem\/project_check\/payments_dx_check\.rs"[\s\S]*"cargo test -q -p dx-www-compiler payments_hash_refresh_stale_helper_keeps_source_hash_clean --lib"/,
    "Rust Studio manifest must publish the Payments lower dx-check helper freshness fixture",
  );
  assert.match(
    studioManifest,
    /studio_source_guard_with_fixture\(\s*"payments-check-panel-helper-freshness"[\s\S]*"core\/src\/ecosystem\/dx_check_receipt\.rs"[\s\S]*"cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_payments_stale_helper_without_source_hash_drift --lib"/,
    "Rust Studio manifest must publish the Payments check-panel helper freshness fixture",
  );
  assert.match(
    studioManifest,
    /"\/" => guards\.extend\(\[[\s\S]*"payments-lower-dx-check-helper-freshness"[\s\S]*"payments-check-panel-helper-freshness"/,
    "the /launch source guard runbook must list both Payments helper-freshness guards",
  );
  assert.match(
    studioManifest,
    /"source_guard_id": "payments-lower-dx-check-helper-freshness"[\s\S]*"fixture_path": "docs\/packages\/payments\.source-guard-runbook\.json"/,
    "Payments lower helper guard must expose a structured runbook fixture path",
  );
  assert.match(
    studioManifest,
    /"source_guard_id": "payments-check-panel-helper-freshness"[\s\S]*"fixture_path": "docs\/packages\/payments\.source-guard-runbook\.json"/,
    "Payments check-panel helper guard must expose a structured runbook fixture path",
  );

  for (const source of [
    frameworkStructure,
    packageDoc,
    read("DX.md"),
    read("TODO.md"),
    read("CHANGELOG.md"),
  ]) {
    assert.match(source, /Payments Studio source-guard\/runbook entry/);
    assert.match(source, /payments-generated-starter-materialization/);
    assert.match(source, /without claiming live Stripe Checkout or webhook runtime proof/);
  }

  assert.doesNotMatch(studioManifest, /dx add stripe-js --write/);
});

test("Payments source-guard runbook fixture mirrors the Studio manifest", () => {
  const fixture = readJson("docs/packages/payments.source-guard-runbook.json");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const packageDoc = read("docs/packages/payments-stripe-js.md");
  const frameworkStructure = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.equal(
    fixture.schema,
    "dx.forge.package.source_guard_runbook_fixture",
  );
  assert.equal(fixture.route, "/");
  assert.equal(fixture.package.official_package_name, "Payments");
  assert.equal(fixture.package.package_id, "payments/stripe-js");
  assert.equal(fixture.package.upstream_package, "@stripe/stripe-js");
  assert.equal(fixture.package.upstream_version, "9.6.0");
  assert.deepEqual(fixture.package.source_mirrors, [
    "G:/WWW/inspirations/stripe-js",
  ]);
  assert.equal(
    fixture.package.based_on,
    "Stripe.js browser loader plus server-owned Checkout and webhook boundaries",
  );

  assert.equal(
    fixture.guard.id,
    "payments-generated-starter-materialization",
  );
  assert.equal(
    fixture.guard.guard_file,
    "benchmarks/payments-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(
    fixture.guard.command,
    "dx run --test .\\benchmarks\\payments-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(fixture.guard.execution_policy, "source-only");
  assert.equal(fixture.guard.writes_files, false);
  assert.equal(fixture.guard.starts_server, false);
  assert.equal(fixture.guard.runs_package_install, false);
  assert.equal(fixture.guard.runs_full_build, false);
  assert.equal(fixture.guard.node_modules_required, false);

  for (const proof of [
    "Payments generated-starter materialization guard",
    'data-dx-check-package-lane-row="payments/stripe-js"',
    'data-dx-token-scope="payments/stripe-js"',
    "payments:receipt-hash-refresh",
    "docs/packages/payments.source-guard-runbook.json",
    "without claiming live Stripe Checkout or webhook runtime proof",
    "payments/stripe-js source-only Studio discovery",
  ]) {
    assert.ok(fixture.guard.proves.includes(proof), `missing proof ${proof}`);
    assert.ok(
      studioManifest.includes(proof) ||
        studioManifest.includes(proof.replaceAll('"', '\\"')),
      `Studio manifest missing ${proof}`,
    );
  }

  assert.equal(
    fixture.runbook.contract.evidence_field,
    "benchmarks/payments-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(
    fixture.runbook.command.purpose,
    "Validate the source-only Payments package-lane row, generated starter materialization, helper freshness markers, readiness route, and dx-check panel package scope without live Stripe Checkout or webhook runtime proof.",
  );

  assert.deepEqual(fixture.inspected_upstream_files, [
    "package.json",
    "src/pure.ts",
    "src/shared.ts",
    "types/shared.d.ts",
    "types/stripe-js/stripe.d.ts",
    "types/stripe-js/embedded-checkout.d.ts",
    "types/stripe-js/checkout.d.ts",
  ]);
  assert.deepEqual(fixture.upstream_public_apis, [
    "loadStripe",
    "loadStripe.setLoadParameters",
    "stripe.confirmPayment",
    "stripe.retrievePaymentIntent",
    "stripe.createEmbeddedCheckoutPage",
    "StripeEmbeddedCheckoutOptions.fetchClientSecret",
  ]);

  assert.ok(
    Array.isArray(fixture.helper_freshness_guards),
    "Payments runbook fixture must expose helper-freshness guards",
  );
  assert.deepEqual(
    fixture.helper_freshness_guards.map((guard) => guard.id),
    [
      "payments-lower-dx-check-helper-freshness",
      "payments-check-panel-helper-freshness",
    ],
  );
  for (const helperGuard of fixture.helper_freshness_guards) {
    assert.equal(helperGuard.execution_policy, "source-only");
    assert.equal(helperGuard.writes_files, false);
    assert.equal(helperGuard.starts_server, false);
    assert.equal(helperGuard.runs_package_install, false);
    assert.equal(helperGuard.runs_full_build, false);
    assert.equal(helperGuard.node_modules_required, false);
    assert.ok(helperGuard.proves.includes("payments_receipt_hash_refresh_stale"));
    assert.ok(
      helperGuard.proves.includes("payments_hash_mismatch stays byte-derived"),
    );
    assert.ok(
      helperGuard.proves.includes(
        "without claiming live Stripe Checkout or webhook runtime proof",
      ),
    );
    assert.ok(
      studioManifest.includes(helperGuard.id),
      `Studio manifest missing ${helperGuard.id}`,
    );
    assert.ok(
      studioManifest.includes(helperGuard.command),
      `Studio manifest missing ${helperGuard.command}`,
    );
  }
  assert.deepEqual(fixture.dx_check_metrics, [
    "payments_receipt_hash_refresh_current",
    "payments_receipt_hash_refresh_stale",
    "payments_receipt_hash_refresh_missing",
    "payments_hash_mismatch",
    "payments-readiness-route",
    "stripeLiveExecution: false",
  ]);
  assert.deepEqual(fixture.source_guard_fixture_paths, [
    {
      source_guard_id: "payments-lower-dx-check-helper-freshness",
      package_id: "payments/stripe-js",
      fixture_path: "docs/packages/payments.source-guard-runbook.json",
      schema: "dx.forge.package.source_guard_runbook_fixture",
    },
    {
      source_guard_id: "payments-check-panel-helper-freshness",
      package_id: "payments/stripe-js",
      fixture_path: "docs/packages/payments.source-guard-runbook.json",
      schema: "dx.forge.package.source_guard_runbook_fixture",
    },
  ]);
  assert.ok(
    fixture.preview_manifest,
    "Payments runbook fixture must describe its preview-manifest exposure",
  );
  assert.equal(fixture.preview_manifest.generated_file, "public/preview-.dx/build-cache/manifest.json");
  assert.equal(
    fixture.preview_manifest.materializer,
    "tools/launch/materialize-www-template.ts",
  );
  assert.equal(
    fixture.preview_manifest.field,
    "sourceGuardRunbookFixtures",
  );
  assert.equal(
    fixture.preview_manifest.route_field,
    "routes[].sourceGuardRunbookFixtures",
  );
  assert.equal(
    fixture.preview_manifest.fixture_path,
    "docs/packages/payments.source-guard-runbook.json",
  );
  assert.equal(fixture.preview_manifest.runtime_proof, false);
  assert.equal(
    fixture.preview_manifest.zed_visibility,
    "payments:receipt-hash-refresh",
  );

  for (const marker of [
    "data-dx-check-package-lane-row",
    "data-dx-check-package-lane-hash-refresh-helper",
    "data-dx-check-package-lane-hash-refresh-json-command",
    "data-dx-check-package-lane-hash-refresh-zed",
    "data-dx-style-surface",
    "data-dx-token-scope",
    "data-dx-package",
  ]) {
    assert.ok(
      fixture.zed_dx_studio_markers.includes(marker),
      `fixture must expose ${marker}`,
    );
  }

  assert.equal(fixture.receipt.zed_visibility, "payments:receipt-hash-refresh");
  assert.equal(
    fixture.receipt.hash_helper,
    "examples/template/payments-receipt-hashes.ts",
  );
  assert.equal(fixture.honesty_label, "SOURCE-ONLY");
  assert.equal(fixture.runtime_proof, false);
  assert.match(fixture.runtime_limitations.join("\n"), /live Stripe Checkout/);

  for (const source of [
    studioManifest,
    packageDoc,
    frameworkStructure,
    dx,
    todo,
    changelog,
  ]) {
    assert.match(source, /payments\.source-guard-runbook\.json/);
    assert.match(source, /payments-generated-starter-materialization/);
    assert.match(source, /without claiming live Stripe Checkout or webhook runtime proof/);
  }
});

test("Payments user-facing CLI copy uses official package naming", () => {
  const activeSources = [
    read("examples/template/package-catalog.ts"),
    read("tools/launch/runtime-template/pages/index.html"),
    read("core/src/ecosystem/forge_stripe_js.rs"),
    read("dx-www/src/cli/mod.rs"),
  ];
  const packageDoc = read("docs/packages/payments-stripe-js.md");
  const frameworkStructure = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const forgeRegistry = read("core/src/ecosystem/forge_registry.rs");
  const dxPackageRegistryEntry = matchSection(
    read("DX.md"),
    /- `payments\/stripe-js`:[\s\S]*?Applications still own [^\n]+/,
    "Payments DX package registry entry",
  );
  const registryDispatchFixture = matchSection(
    forgeRegistry,
    /fn stripe_js_alias_materializes_launch_payment_slice\(\)[\s\S]*?fn vercel_ai_alias_materializes_launch_ai_slice/,
    "Payments Forge registry dispatch fixture",
  );

  for (const source of activeSources) {
    assert.match(source, /dx add payments --write/);
    assert.doesNotMatch(source, /dx add stripe-js --write/);
  }

  assert.match(dxPackageRegistryEntry, /Add with `dx add payments --write`/);
  assert.doesNotMatch(dxPackageRegistryEntry, /dx add stripe-js --write/);
  assert.match(packageDoc, /Recommended command: `dx add payments --write`/);
  assert.doesNotMatch(packageDoc, /dx add stripe-js --write/);
  assert.match(
    frameworkStructure,
    /Lane 12 uses the official front-facing package name `Payments`/,
  );
  assert.match(
    frameworkStructure,
    /dx run --test \.\\benchmarks\\payments-dx-check-package-lane-panel\.test\.ts/,
  );
  assert.match(
    read("examples/template/package-catalog.ts"),
    /aliases:\s*\[\s*"payments",\s*"stripe-js"/,
  );
  assert.match(
    read("core/src/ecosystem/forge_stripe_js.rs"),
    /aliases:\s*\[\s*"payments",\s*"stripe-js"/,
  );
  assert.match(forgeRegistry, /"payments"\s*\|\s*"@stripe\/stripe-js"/);
  assert.match(forgeRegistry, /"payments"\.to_string\(\)/);
  assert.match(registryDispatchFixture, /canonical_package_id\("payments"\)/);
  assert.match(registryDispatchFixture, /source_package_for_project\("payments"/);
  assert.match(
    registryDispatchFixture,
    /metadata\.contains\(r#"officialPackageName: "Payments""#\)/,
  );

  for (const source of [read("DX.md"), read("TODO.md"), read("CHANGELOG.md")]) {
    assert.match(source, /Payments official CLI command alias/);
    assert.match(source, /`dx add payments --write`/);
  }
});
