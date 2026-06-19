const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  const filePath = path.join(root, relativePath);
  assert.ok(fs.existsSync(filePath), `missing ${relativePath}`);
  return fs.readFileSync(filePath, "utf8");
}

test("Forms package docs and catalog expose official lane metadata", () => {
  const docs = read("docs/packages/forms-react-hook-form.md");
  const catalog = read("examples/template/package-catalog.ts");
  const forge = read("core/src/ecosystem/forge_react_hook_form.rs");
  const registry = read("core/src/ecosystem/forge_registry.rs");
  const cli = read("dx-www/src/cli/mod.rs");
  const routeContract = read("examples/template/template-route-contract.ts");
  const receipt = read(
    "examples/template/.dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json",
  );

  assert.match(docs, /^# Forms/m);
  assert.match(docs, /official_package_name: Forms/);
  assert.match(docs, /package_id: forms\/react-hook-form/);
  assert.match(docs, /upstream_package: react-hook-form/);
  assert.match(docs, /source_mirror: G:\/WWW\/inspirations\/react-hook-form/);
  assert.match(docs, /upstream_version: 7\.75\.0/);
  assert.match(docs, /honesty_label: SOURCE-ONLY/);
  assert.match(docs, /src\/useForm\.ts/);
  assert.match(docs, /src\/controller\.tsx/);
  assert.match(docs, /src\/useFieldArray\.ts/);
  assert.match(docs, /src\/types\/form\.ts/);
  assert.match(docs, /useForm/);
  assert.match(docs, /FormProvider/);
  assert.match(docs, /Controller/);
  assert.match(docs, /useFieldArray/);
  assert.match(docs, /Resolver/);
  assert.match(docs, /data-dx-component="template-lead-form"/);
  assert.match(docs, /data-dx-edit-id="launch\.lead-form"/);
  assert.match(docs, /tracked_files/);
  assert.match(docs, /current_files/);
  assert.match(docs, /stale_files/);
  assert.match(docs, /missing_files/);
  assert.match(docs, /stale_mirror_files/);
  assert.match(docs, /missing_mirror_files/);
  assert.match(docs, /present/);
  assert.match(docs, /stale/);
  assert.match(docs, /missing receipt/);
  assert.match(docs, /blocked/);
  assert.match(docs, /unsupported surface/);

  assert.match(
    catalog,
    /packageId: "forms\/react-hook-form",[\s\S]*?aliases: \["forms", "react-hook-form", "rhf", "forms\/rhf"\]/,
  );
  assert.match(
    catalog,
    /packageId: "forms\/react-hook-form",[\s\S]*?command: "dx add forms --write"/,
  );
  assert.match(
    catalog,
    /packageId: "forms\/react-hook-form",[\s\S]*?sourceMirror: "G:\/WWW\/inspirations\/react-hook-form"/,
  );
  assert.match(
    catalog,
    /packageId: "forms\/react-hook-form",[\s\S]*?"docs\/packages\/forms-react-hook-form\.md"/,
  );
  assert.match(
    catalog,
    /packageId: "forms\/react-hook-form",[\s\S]*?"examples\/template\/\.dx\/forge\/receipts\/2026-05-22-forms-dashboard-workflow\.json"/,
  );
  assert.match(
    catalog,
    /packageId: "forms\/react-hook-form",[\s\S]*?component: "template-lead-form"/,
  );
  assert.match(catalog, /"forms\/react-hook-form": \{\s*name: "Forms"/);
  assert.match(catalog, /"forms\/react-hook-form": \{[\s\S]*?"useForm"/);
  assert.match(catalog, /"forms\/react-hook-form": \{[\s\S]*?"createDxZodResolver"/);

  assert.match(forge, /officialPackageName: "Forms"/);
  assert.match(forge, /dxAdd: "dx add forms --write"/);
  assert.match(forge, /sourceMirror: "G:\/WWW\/inspirations\/react-hook-form"/);
  assert.match(forge, /inspectedSourceFiles: \[/);
  assert.match(forge, /"src\/useForm\.ts"/);
  assert.match(forge, /"src\/controller\.tsx"/);
  assert.match(forge, /surfaces: \[/);
  assert.match(forge, /"template-lead-form"/);
  assert.match(forge, /honestyLabel: "SOURCE-ONLY"/);
  assert.match(registry, /"forms"[\s\S]*?=> \{\s*"forms\/react-hook-form"/);
  assert.match(cli, /"official_name": "Forms"/);
  assert.match(cli, /"command": "dx add forms --write"/);
  assert.match(cli, /NEXT_FAMILIAR_FORMS_DASHBOARD_RECEIPT_JSON/);
  assert.match(
    cli,
    /"\.dx\/forge\/receipts\/2026-05-22-forms-dashboard-workflow\.json"/,
  );
  assert.match(
    cli,
    /path: "\.dx\/forge\/receipts\/2026-05-22-forms-dashboard-workflow\.json"/,
  );
  assert.doesNotMatch(cli, /"cli_add"\] == "dx add react-hook-form --write"/);
  assert.match(
    routeContract,
    /"\.dx\/forge\/receipts\/2026-05-22-forms-dashboard-workflow\.json"/,
  );
  assert.match(routeContract, /formsDashboardWorkflow: \{/);
  assert.match(routeContract, /packageId: "forms\/react-hook-form"/);
  assert.match(routeContract, /sourceGuard: "dx run --test [^"]*forms-react-hook-form-package-doc\.test\.ts"/);
  assert.match(routeContract, /materializedReceiptFile:\s*"\.dx\/forge\/receipts\/2026-05-22-forms-dashboard-workflow\.json"/);

  const parsedReceipt = JSON.parse(receipt);
  assert.equal(parsedReceipt.package_name, "Forms");
  assert.equal(parsedReceipt.package_id, "forms/react-hook-form");
  assert.equal(parsedReceipt.upstream_package, "react-hook-form");
  assert.equal(parsedReceipt.source_mirror, "G:/WWW/inspirations/react-hook-form");
  assert.equal(parsedReceipt.honesty_label, "SOURCE-ONLY");
  assert.ok(parsedReceipt.selected_surfaces.includes("template-lead-form"));
  assert.ok(parsedReceipt.files.includes("examples/template/template-lead-form.tsx"));
  assert.ok(parsedReceipt.files.includes("examples/template/template-route-contract.ts"));
  assert.ok(parsedReceipt.provenance.inspected_source_files.includes("src/useForm.ts"));
  assert.ok(parsedReceipt.dx_check_visibility.includes("missing receipt"));
  assert.equal(parsedReceipt.runtime_execution, false);
});
