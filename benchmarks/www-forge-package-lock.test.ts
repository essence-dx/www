const assert = require("assert");
const { execFileSync } = require("child_process");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const templateRoot = path.join(root, "examples", "template");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

test("launch template exposes real Forge package locks and package-add receipts", () => {
  execFileSync(
    process.execPath,
    ["examples/template/forge-package-lock.ts", "--check"],
    { cwd: root, stdio: "pipe" },
  );

  const readModelSource = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const generatedStatus = readJson(
    "examples/template/.dx/forge/package-status.json",
  );
  assert.match(readModelSource, /state: "generated-from-source-manifest"/);
  assert.match(
    readModelSource,
    new RegExp(`lockedPackageCount: ${generatedStatus.locked_package_count}`),
  );
  assert.match(readModelSource, /sourceReceipt: "\.dx\/forge\/package-status\.json"/);

  const statusSource = read("examples/template/forge-package-status.ts");
  assert.match(statusSource, /readLaunchForgePackageStatus/);
  assert.match(statusSource, /packageRows: launchForgePackageRows\(\)/);

  const sourceManifest = readJson(
    "examples/template/.dx/forge/source-manifest.json",
  );
  const sourcePackageIds = new Set(
    sourceManifest.packages.map((packageEntry) => packageEntry.package_id),
  );
  assert.ok(
    sourceManifest.packages.length >= 3,
    "source manifest should expose the locked package entries",
  );
  assert.ok(
    sourcePackageIds.has("shadcn/ui/button") &&
      sourcePackageIds.has("state/zustand") &&
      sourcePackageIds.has("tanstack/query") &&
      sourcePackageIds.has("payments/stripe-js") &&
      sourcePackageIds.has("ai/vercel-ai") &&
      sourcePackageIds.has("automations/n8n") &&
      sourcePackageIds.has("animation/motion") &&
      sourcePackageIds.has("3d/launch-scene"),
    "source manifest should expose the launch package set",
  );
  for (const packageEntry of sourceManifest.packages) {
    assert.ok(
      packageEntry.rollback_receipt,
      `${packageEntry.package_id} should expose a rollback receipt anchor`,
    );
    assert.ok(
      fs.existsSync(
        path.join(
          templateRoot,
          ".dx",
          "forge",
          "receipts",
          packageEntry.rollback_receipt,
        ),
      ),
      `${packageEntry.package_id} rollback receipt should exist`,
    );
  }

  const lock = readJson("examples/template/.dx/forge/package-lock.json");
  assert.equal(lock.schema, "forge.package_lock");
  assert.ok(lock.packages.length >= 2, "expected at least two locked packages");
  const packages = new Map(lock.packages.map((item) => [item.name, item]));
  const button = packages.get("shadcn/ui/button");
  const zustand = packages.get("state/zustand");
  const query = packages.get("tanstack/query");
  const validation = packages.get("validation/zod");
  const forms = packages.get("forms/react-hook-form");
  const payments = packages.get("payments/stripe-js");
  const ai = packages.get("ai/vercel-ai");
  const automations = packages.get("automations/n8n");
  const motion = packages.get("animation/motion");
  const launchScene = packages.get("3d/launch-scene");
  assert.ok(button, "missing shadcn/ui/button lock entry");
  assert.ok(zustand, "missing state/zustand lock entry");
  assert.ok(query, "missing tanstack/query lock entry");
  assert.ok(validation, "missing validation/zod lock entry");
  assert.ok(forms, "missing forms/react-hook-form lock entry");
  assert.ok(payments, "missing payments/stripe-js lock entry");
  assert.ok(ai, "missing ai/vercel-ai lock entry");
  assert.ok(automations, "missing automations/n8n lock entry");
  assert.ok(motion, "missing animation/motion lock entry");
  assert.ok(launchScene, "missing 3d/launch-scene lock entry");
  assert.equal(button.version, "0.1.0");
  assert.equal(button.source_kind, "local-slice");
  assert.equal(button.integrity_state, "valid");
  assert.ok(button.rollback_receipt_path, "button should expose rollback receipt path");
  assert.ok(
    button.safety_archive_receipt_path,
    "button should expose safety archive receipt path",
  );
  assert.ok(/^[a-f0-9]{64}$/.test(button.integrity_hash));
  assert.equal(zustand.version, "5.0.13-dx.10");
  assert.equal(zustand.source_kind, "local-slice");
  assert.equal(zustand.source_locator, "lib/forge/state/zustand/index.ts");
  assert.equal(zustand.integrity_state, "valid");
  assert.ok(zustand.rollback_receipt_path, "zustand should expose rollback receipt path");
  assert.ok(
    zustand.safety_archive_receipt_path,
    "zustand should expose safety archive receipt path",
  );
  assert.equal(zustand.files.length, 13);
  assert.ok(/^[a-f0-9]{64}$/.test(zustand.integrity_hash));
  assert.equal(query.version, "5.100.10-dx.0");
  assert.equal(query.source_kind, "local-slice");
  assert.equal(query.source_locator, "lib/query/client.ts");
  assert.equal(query.integrity_state, "valid");
  assert.ok(query.rollback_receipt_path, "query should expose rollback receipt path");
  assert.ok(
    query.safety_archive_receipt_path,
    "query should expose safety archive receipt path",
  );
  assert.ok(query.files.length >= 30);
  assert.ok(/^[a-f0-9]{64}$/.test(query.integrity_hash));
  const queryReactDependency = query.dependency_constraints.find(
    (dependency) => dependency.name === "@tanstack/react-query",
  );
  assert.ok(
    queryReactDependency,
    "tanstack/query should record its real React Query peer boundary",
  );
  assert.match(queryReactDependency.constraint, /peer runtime/);
  assert.ok(
    query.dependency_constraints.some(
      (dependency) => dependency.name === "@tanstack/query-core",
    ),
    "tanstack/query should record the query-core boundary",
  );
  assert.equal(validation.version, "4.4.3-dx.13");
  assert.equal(validation.source_kind, "local-slice");
  assert.equal(validation.source_locator, "lib/validation/zod/dashboard-settings.ts");
  assert.equal(validation.integrity_state, "valid");
  assert.equal(validation.files.length, 20);
  assert.ok(validation.rollback_receipt_path, "validation/zod should expose rollback receipt path");
  assert.ok(
    validation.safety_archive_receipt_path,
    "validation/zod should expose safety archive receipt path",
  );
  assert.ok(/^[a-f0-9]{64}$/.test(validation.integrity_hash));
  assert.equal(forms.version, "7.75.0-dx.0");
  assert.equal(forms.source_kind, "local-slice");
  assert.equal(forms.source_locator, "lib/forms/react-hook-form/form.tsx");
  assert.equal(forms.integrity_state, "valid");
  assert.equal(forms.files.length, 7);
  assert.ok(forms.rollback_receipt_path, "forms/react-hook-form should expose rollback receipt path");
  assert.ok(
    forms.safety_archive_receipt_path,
    "forms/react-hook-form should expose safety archive receipt path",
  );
  assert.ok(/^[a-f0-9]{64}$/.test(forms.integrity_hash));
  assert.equal(payments.version, "9.6.0-dx.1");
  assert.equal(payments.source_kind, "local-slice");
  assert.equal(payments.source_locator, "lib/payments/stripe-js/dashboard-checkout.ts");
  assert.equal(payments.integrity_state, "valid");
  assert.equal(payments.files.length, 11);
  assert.ok(
    payments.files.some(
      (file) => file.path === "app/api/payments/stripe-js/readiness/route.ts",
    ),
    "payments/stripe-js should lock the App Router readiness route",
  );
  assert.ok(
    payments.dependency_constraints.some(
      (dependency) => dependency.name === "@stripe/stripe-js",
    ),
    "payments/stripe-js should record its real Stripe.js boundary",
  );
  assert.ok(
    payments.safety_archive_receipt_path,
    "payments/stripe-js should expose safety archive receipt path",
  );
  assert.ok(/^[a-f0-9]{64}$/.test(payments.integrity_hash));
  assert.equal(ai.version, "7.0.0-canary.146-dx.0");
  assert.equal(ai.source_kind, "local-slice");
  assert.equal(ai.source_locator, "lib/ai/chat-route.ts");
  assert.equal(ai.integrity_state, "valid");
  assert.equal(ai.files.length, 36);
  assert.ok(
    ai.files.some((file) => file.path === "lib/ai/provider-boundary.ts"),
    "ai/vercel-ai should lock the provider-boundary source file",
  );
  assert.ok(
    ai.dependency_constraints.some((dependency) => dependency.name === "ai"),
    "ai/vercel-ai should record its real AI SDK boundary",
  );
  assert.ok(ai.safety_archive_receipt_path, "ai/vercel-ai should expose safety archive receipt path");
  assert.ok(/^[a-f0-9]{64}$/.test(ai.integrity_hash));
  assert.equal(automations.version, "1.0.0-dx.3");
  assert.equal(automations.source_kind, "local-slice");
  assert.equal(automations.source_locator, "lib/automations/n8n/catalog.ts");
  assert.equal(automations.integrity_state, "valid");
  assert.equal(automations.files.length, 7);
  assert.ok(
    automations.files.some(
      (file) => file.path === "app/api/automations/n8n/dry-run/route.ts",
    ),
    "automations/n8n should lock the App Router dry-run route",
  );
  assert.ok(
    automations.dependency_constraints.some(
      (dependency) => dependency.name === "n8n-nodes-base",
    ),
    "automations/n8n should record its real n8n package boundary",
  );
  assert.ok(
    automations.safety_archive_receipt_path,
    "automations/n8n should expose safety archive receipt path",
  );
  assert.ok(/^[a-f0-9]{64}$/.test(automations.integrity_hash));
  assert.equal(motion.version, "12.38.0-dx.12");
  assert.equal(motion.source_kind, "local-slice");
  assert.equal(motion.source_locator, "motion/provider.tsx");
  assert.equal(motion.integrity_state, "valid");
  assert.equal(motion.files.length, 34);
  assert.ok(
    motion.files.some((file) => file.path === "motion/dashboard-workflow.ts"),
    "animation/motion should lock the dashboard workflow helper",
  );
  assert.ok(
    motion.dependency_constraints.some(
      (dependency) => dependency.name === "motion",
    ),
    "animation/motion should record its real Motion package boundary",
  );
  assert.ok(
    motion.safety_archive_receipt_path,
    "animation/motion should expose safety archive receipt path",
  );
  assert.equal(motion.runtime_proof, false);
  assert.ok(/^[a-f0-9]{64}$/.test(motion.integrity_hash));
  assert.equal(launchScene.version, "0.184.0-r3f10-dx.0");
  assert.equal(launchScene.source_kind, "local-slice");
  assert.equal(launchScene.source_locator, "components/scene/launch-scene.tsx");
  assert.equal(launchScene.integrity_state, "valid");
  assert.equal(launchScene.files.length, 38);
  assert.ok(
    launchScene.files.some(
      (file) => file.path === "lib/scene/renderer-handoff.ts",
    ),
    "3d/launch-scene should lock the renderer handoff helper",
  );
  assert.ok(
    launchScene.dependency_constraints.some(
      (dependency) => dependency.name === "three",
    ),
    "3d/launch-scene should record its real Three package boundary",
  );
  assert.ok(
    launchScene.safety_archive_receipt_path,
    "3d/launch-scene should expose safety archive receipt path",
  );
  assert.equal(launchScene.runtime_proof, false);
  assert.equal(launchScene.webgl_runtime_proof, false);
  assert.equal(launchScene.browser_screenshot_proof, false);
  assert.ok(/^[a-f0-9]{64}$/.test(launchScene.integrity_hash));
  assert.equal(lock.remotes.length, 5);
  assert.equal(
    lock.remotes.filter((remote) => remote.executable_now).length,
    1,
  );
  assert.ok(lock.remotes.every((remote) => remote.secrets_safe === true));
  assert.equal(lock.media.length, 1);
  assert.equal(lock.media[0].path, "tools/launch/runtime-template/assets/favicon.svg");
  assert.equal(lock.media[0].chunk_count, 1);
  assert.ok(/^[a-f0-9]{64}$/.test(lock.media[0].content_hash));

  for (const packageEntry of lock.packages) {
    for (const file of packageEntry.files) {
      assert.ok(
        fs.existsSync(path.join(templateRoot, file.path)),
        `missing locked package file ${file.path}`,
      );
      assert.ok(/^[a-f0-9]{64}$/.test(file.content_hash));
    }
  }

  for (const expectedFile of [
    "lib/forge/state/zustand/index.ts",
    "lib/forge/state/zustand/persist.ts",
    "lib/forge/state/zustand/README.md",
  ]) {
    assert.ok(
      zustand.files.some((file) => file.path === expectedFile),
      `missing state/zustand locked file ${expectedFile}`,
    );
  }

  for (const expectedFile of [
    "lib/query/client.ts",
    "lib/query/provider.tsx",
    "lib/query/prefetch.tsx",
    "lib/query/hydration.ts",
    "lib/query/metadata.ts",
    "lib/query/README.md",
  ]) {
    assert.ok(
      query.files.some((file) => file.path === expectedFile),
      `missing tanstack/query locked file ${expectedFile}`,
    );
  }

  const receiptPath = button.receipt_paths.find((candidate) =>
    candidate.includes("receipts/packages/shadcn-ui-button.json"),
  );
  assert.ok(receiptPath, "lock should reference the package-add receipt");

  const receipt = readJson(`examples/template/${receiptPath}`);
  assert.equal(receipt.schema, "forge.package_add_receipt");
  assert.equal(receipt.package.name, "shadcn/ui/button");
  assert.equal(
    receipt.boundary,
    "forge-owned source slice; no node_modules install performed",
  );
  assert.equal(receipt.cache.cached_files.length, button.files.length);
  assert.equal(receipt.rollback.state, "covered");
  assert.equal(receipt.rollback.receipt_path, button.rollback_receipt_path);
  assert.equal(
    receipt.rollback.safety_archive_receipt_path,
    button.safety_archive_receipt_path,
  );

  const stateReceiptPath = zustand.receipt_paths.find((candidate) =>
    candidate.includes("receipts/packages/state-zustand.json"),
  );
  assert.ok(stateReceiptPath, "state/zustand should reference package-add receipt");
  const stateReceipt = readJson(`examples/template/${stateReceiptPath}`);
  assert.equal(stateReceipt.schema, "forge.package_add_receipt");
  assert.equal(stateReceipt.package.name, "state/zustand");
  assert.equal(
    stateReceipt.boundary,
    "forge-owned source slice; no node_modules install performed",
  );
  assert.equal(stateReceipt.cache.cached_files.length, zustand.files.length);
  assert.equal(stateReceipt.rollback.state, "covered");
  assert.equal(stateReceipt.rollback.receipt_path, zustand.rollback_receipt_path);
  assert.equal(
    stateReceipt.rollback.safety_archive_receipt_path,
    zustand.safety_archive_receipt_path,
  );
  assert.ok(
    stateReceipt.cache.cached_files.every((file) =>
      file.cache_path.includes(".dx/forge/cache/state-zustand/5.0.13-dx.10/"),
    ),
  );

  const queryReceiptPath = query.receipt_paths.find((candidate) =>
    candidate.includes("receipts/packages/tanstack-query.json"),
  );
  assert.ok(queryReceiptPath, "tanstack/query should reference package-add receipt");
  const queryReceipt = readJson(`examples/template/${queryReceiptPath}`);
  assert.equal(queryReceipt.schema, "forge.package_add_receipt");
  assert.equal(queryReceipt.package.name, "tanstack/query");
  assert.equal(
    queryReceipt.boundary,
    "forge-owned source slice; no node_modules install performed",
  );
  assert.equal(queryReceipt.cache.cached_files.length, query.files.length);
  assert.equal(queryReceipt.rollback.state, "covered");
  assert.equal(queryReceipt.rollback.receipt_path, query.rollback_receipt_path);
  assert.equal(
    queryReceipt.rollback.safety_archive_receipt_path,
    query.safety_archive_receipt_path,
  );
  assert.ok(
    queryReceipt.cache.cached_files.every((file) =>
      file.cache_path.includes(".dx/forge/cache/tanstack-query/5.100.10-dx.0/"),
    ),
  );

  for (const packageEntry of [
    button,
    zustand,
    query,
    validation,
    forms,
    payments,
    ai,
    automations,
    motion,
    launchScene,
  ]) {
    const archiveReceipt = readJson(
      `examples/template/${packageEntry.safety_archive_receipt_path}`,
    );
    assert.equal(archiveReceipt.schema, "forge.package_safety_archive_receipt");
    assert.equal(archiveReceipt.package.name, packageEntry.name);
    assert.equal(archiveReceipt.rollback.state, "covered");
    assert.equal(archiveReceipt.rollback.receipt_path, packageEntry.rollback_receipt_path);
    assert.equal(archiveReceipt.archive.file_count, packageEntry.files.length);
    assert.equal(archiveReceipt.archive.cache_file_count, packageEntry.files.length);
    assert.ok(
      archiveReceipt.archive.files.every((file) =>
        fs.existsSync(path.join(templateRoot, file.cache_path)),
      ),
      `${packageEntry.name} archive should reference existing cache files`,
    );
  }

  const status = readJson("examples/template/.dx/forge/package-status.json");
  assert.equal(status.status, "lock-backed");
  assert.equal(status.package_lock.state, "generated-from-source-manifest");
  assert.equal(status.package_lock.package_count, lock.packages.length);
  assert.equal(status.package_lock.integrity_valid, true);
  assert.equal(status.package_lock.remote_count, lock.remotes.length);
  assert.equal(status.package_lock.media_asset_count, lock.media.length);
  assert.equal(
    status.package_lock.cache_file_count,
    lock.packages.reduce((count, packageEntry) => count + packageEntry.files.length, 0),
  );
  assert.ok(
    status.locked_package_names.includes("tanstack/query"),
    "package status should expose tanstack/query for DX-WWW and dx-check",
  );
  assert.ok(
    status.locked_package_names.includes("validation/zod"),
    "package status should expose validation/zod after lock-backed promotion",
  );
  assert.ok(
    status.locked_package_names.includes("forms/react-hook-form"),
    "package status should expose forms/react-hook-form after lock-backed promotion",
  );
  assert.ok(
    status.locked_package_names.includes("payments/stripe-js"),
    "package status should expose payments/stripe-js after lock-backed promotion",
  );
  assert.ok(
    status.locked_package_names.includes("ai/vercel-ai"),
    "package status should expose ai/vercel-ai after lock-backed promotion",
  );
  assert.ok(
    status.locked_package_names.includes("automations/n8n"),
    "package status should expose automations/n8n after lock-backed promotion",
  );
  assert.ok(
    status.locked_package_names.includes("animation/motion"),
    "package status should expose animation/motion after lock-backed promotion",
  );
  assert.ok(
    status.locked_package_names.includes("3d/launch-scene"),
    "package status should expose 3d/launch-scene after lock-backed promotion",
  );
  assert.equal(status.safety_archive.status, "covered");
  assert.equal(status.safety_archive.rollback_covered_package_count, lock.packages.length);
  assert.equal(status.safety_archive.rollback_missing_package_count, 0);
  assert.equal(status.safety_archive.archive_receipt_count, lock.packages.length);
});
