const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const routePath = path.join(root, "examples", "template", "app", "page.tsx");
const runtimePagePath = path.join(root, "tools", "launch", "runtime-template", "pages", "index.html");
const runtimeScriptPath = path.join(root, "tools", "launch", "runtime-template", "assets", "launch-runtime.ts");
const shellPath = path.join(root, "examples", "template", "template-shell.tsx");
const catalogPath = path.join(root, "examples", "template", "package-catalog.ts");
const editContractPath = path.join(
  root,
  "examples",
  "www-template",
  "dx-studio-edit-contract.ts",
);
const paymentsStatusPath = path.join(
  root,
  "examples",
  "www-template",
  "payments-status.tsx",
);
const stripeSourcePath = path.join(root, "core", "src", "ecosystem", "forge_stripe_js.rs");
const cliPath = path.join(root, "dx-www", "src", "cli", "mod.rs");
const defaultTemplateSourcesPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "default_template_sources.rs",
);
const studioManifestPath = path.join(root, "dx-www", "src", "cli", "studio_manifest.rs");
const materializerPath = path.join(root, "tools", "launch", "materialize-www-template.ts");
const stripeReceiptPath = path.join(
  root,
  "examples",
  "www-template",
  ".dx",
  "forge",
  "receipts",
  "2026-05-22-payments-stripe-js-billing-workflow.json",
);
const stripeReceiptRelativePath =
  "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json";

function read(filePath) {
  assert.ok(fs.existsSync(filePath), `expected ${path.relative(root, filePath)} to exist`);
  return fs.readFileSync(filePath, "utf8");
}

test("payments/stripe-js is a visible launch proof with a safe local receipt path", () => {
  const route = read(routePath);
  const runtimePage = read(runtimePagePath);
  const runtimeScript = read(runtimeScriptPath);
  const shell = read(shellPath);
  const catalog = read(catalogPath);
  const editContract = read(editContractPath);
  const paymentsStatus = read(paymentsStatusPath);
  const stripeSource = read(stripeSourcePath);
  const cli = read(cliPath);
  const newCommand = read(path.join(root, "dx-www", "src", "cli", "new_command.rs"));
  const defaultTemplateSources = read(defaultTemplateSourcesPath);
  const studioManifest = read(studioManifestPath);
  const materializer = read(materializerPath);
  const stripeReceipt = JSON.parse(read(stripeReceiptPath));

  assert.match(route, /import \{ TemplateShell \} from "@\/components\/template-app\/template-shell";/);
  assert.match(route, /<DxIntlProvider locale=\{dxDefaultLocale\} messages=\{messages\}>/);
  assert.match(route, /<TemplateShell \/>/);
  assert.doesNotMatch(route, /stripe-checkout-contact-proof|stripe-payment-card/);

  assert.match(runtimePage, /data-dx-package="payments\/stripe-js"/);
  assert.match(runtimePage, /data-dx-component="launch-billing-checkout-workflow"/);
  assert.match(runtimePage, /data-dx-dashboard-flow="billing-checkout"/);
  assert.match(runtimePage, /data-dx-stripe-dashboard-workflow="plan-checkout"/);
  assert.match(runtimePage, /data-dx-stripe-action="select-plan"/);
  assert.match(runtimePage, /id="stripe-checkout-form"/);
  assert.match(runtimePage, /data-dx-stripe-interaction="checkout-contact-form"/);
  assert.match(runtimePage, /data-dx-stripe-action="request-checkout-intent"/);
  assert.match(runtimePage, /data-dx-stripe-post-result="idle"/);
  assert.match(runtimePage, /data-dx-stripe-config-state="missing-config"/);
  assert.match(runtimePage, /data-dx-stripe-receipt-path="examples\/template\/\.dx\/forge\/receipts\/2026-05-22-payments-stripe-js-billing-workflow\.json"/);
  assert.match(runtimePage, /data-dx-stripe-safe-readiness-post="true"/);
  assert.doesNotMatch(runtimePage, /data-dx-stripe-safe-placeholder-post/);
  assert.match(runtimePage, /<dx-icon name="pack:payments"/);
  assert.doesNotMatch(runtimePage, /stripe-checkout-contact-proof|stripe-payment-card/);

  assert.match(runtimeScript, /function bindPayment\(\)/);
  assert.match(runtimeScript, /launch-billing-checkout-workflow/);
  assert.match(runtimeScript, /data-dx-stripe-action="select-plan"/);
  assert.match(runtimeScript, /new FormData\(form\)/);
  assert.match(runtimeScript, /fetch\("\/api\/checkout"/);
  assert.match(runtimeScript, /dxStripePostResult/);
  assert.match(runtimeScript, /stripe-local-/);
  assert.match(runtimeScript, /safe readiness POST/i);
  assert.doesNotMatch(runtimeScript, /safe placeholder POST|local placeholder receipt/i);
  assert.doesNotMatch(runtimeScript, /stripe-checkout-contact-proof|stripe-payment-card/);

  assert.match(shell, /import \{ LaunchPaymentStatus \} from "\.\/payments-status";/);
  assert.match(shell, /<LaunchPaymentStatus workflowContext="launch-dashboard" \/>/);
  assert.match(shell, /data-dx-package="payments\/stripe-js"/);
  assert.match(shell, /data-dx-section="billing-workflow"/);
  assert.match(shell, /data-dx-component="launch-billing-dashboard-workflow"/);
  assert.match(shell, /data-dx-dashboard-flow="billing-checkout"/);

  assert.match(catalog, /sourceMirror: "G:\/WWW\/inspirations\/stripe-js"/);
  assert.match(catalog, /dashboard-checkout\.ts/);
  assert.match(catalog, /createDxStripeDashboardCheckoutRequest/);
  assert.match(catalog, /dashboard-stripe-plan-checkout/);
  assert.match(catalog, /tools\/launch\/materialize-www-template\.ts/);
  assert.match(catalog, /examples\/conversion-proof\/public\/preview-manifest\.json/);
  assert.match(catalog, /2026-05-22-payments-stripe-js-billing-workflow\.json/);
  assert.match(catalog, /data-dx-stripe-receipt-path="examples\/template\/\.dx\/forge\/receipts\/2026-05-22-payments-stripe-js-billing-workflow\.json"/);

  assert.match(editContract, /id: "launch-billing-checkout-workflow"/);
  assert.match(editContract, /selector: '\[data-dx-component="launch-billing-checkout-workflow"\]'/);
  assert.match(editContract, /materializedFile: "components\/template-app\/payments-status\.tsx"/);

  assert.match(studioManifest, /"front_facing_name": "Payments Billing Workflow"/);
  assert.match(studioManifest, /"dashboard_workflow": "billing-checkout"/);
  assert.match(studioManifest, /"source_mirror": "G:\/WWW\/inspirations\/stripe-js"/);
  assert.match(studioManifest, /"createDxStripeDashboardCheckoutRequest"/);
  assert.match(studioManifest, /"createDxStripeDashboardMissingConfigReceipt"/);
  assert.match(studioManifest, /"dxStripeDashboardPlans"/);
  assert.match(studioManifest, /"data-dx-dashboard-flow"/);
  assert.match(studioManifest, /"data-dx-stripe-dashboard-workflow"/);
  assert.match(studioManifest, /"data-dx-stripe-action"/);
  assert.match(studioManifest, /"data-dx-stripe-receipt-path"/);
  assert.match(studioManifest, /studio_edit_surface\(\s+"launch-billing-checkout-workflow"/);
  assert.match(studioManifest, /\[data-dx-component=\\?"launch-billing-checkout-workflow\\?"\]/);
  assert.doesNotMatch(studioManifest, /launch-stripe-contract-proof|stripe-contract-proof/);

  assert.match(materializer, /"launch-runtime-billing-checkout"/);
  assert.match(materializer, /\[data-dx-component="launch-billing-checkout-workflow"\]/);
  assert.match(materializer, /"data-dx-stripe-dashboard-workflow"/);
  assert.match(materializer, /"data-dx-stripe-action"/);
  assert.match(materializer, /"data-dx-stripe-receipt-path"/);
  assert.match(materializer, /materializeForgeReceipts/);

  assert.equal(stripeReceipt.package_id, "payments/stripe-js");
  assert.equal(stripeReceipt.dashboard_workflow, "billing-checkout");
  assert.equal(stripeReceipt.component_marker, 'data-dx-component="launch-billing-checkout-workflow"');
  assert.equal(stripeReceipt.receipt_path_marker, `data-dx-stripe-receipt-path="${stripeReceiptRelativePath}"`);
  assert.equal(stripeReceipt.runtime_execution, false);
  assert.equal(stripeReceipt.secret_values.length, 0);
  assert.ok(stripeReceipt.required_env.includes("STRIPE_SECRET_KEY"));
  assert.ok(stripeReceipt.required_env.includes("STRIPE_PRICE_ID"));
  assert.ok(stripeReceipt.required_env.includes("STRIPE_PRICE_ID_STARTER"));
  assert.equal(stripeReceipt.plan_price_env_contract.team, "STRIPE_PRICE_ID_TEAM");
  assert.match(
    stripeReceipt.plan_price_env_contract.trust_boundary,
    /accepts only the app-owned starter\/team\/scale plan ids/,
  );
  assert.ok(stripeReceipt.source_files.includes("examples/template/payments-status.tsx"));
  assert.ok(stripeReceipt.source_files.includes("tools/launch/runtime-template/pages/index.html"));
  assert.ok(stripeReceipt.stable_markers.includes('data-dx-stripe-action="request-checkout-intent"'));
  assert.ok(stripeReceipt.stable_markers.includes(`data-dx-stripe-receipt-path="${stripeReceiptRelativePath}"`));
  assert.match(JSON.stringify(stripeReceipt), /no-template-local-node_modules/);

  assert.match(paymentsStatus, /data-dx-package="payments\/stripe-js"/);
  assert.match(paymentsStatus, /from "@\/lib\/payments\/stripe-js\/dashboard-checkout"/);
  assert.match(paymentsStatus, /dxStripeDashboardPlans/);
  assert.match(paymentsStatus, /createDxStripeDashboardCheckoutRequest/);
  assert.match(paymentsStatus, /createDxStripeDashboardMissingConfigReceipt/);
  assert.match(paymentsStatus, /data-dx-component="launch-billing-checkout-workflow"/);
  assert.match(paymentsStatus, /data-dx-stripe-dashboard-workflow="plan-checkout"/);
  assert.match(paymentsStatus, /data-dx-stripe-config-state=\{status\.kind\}/);
  assert.match(paymentsStatus, /data-dx-stripe-checkout-mode=\{checkoutMode\}/);
  assert.match(paymentsStatus, /data-dx-stripe-plan-id=\{selectedPlan\.id\}/);
  assert.match(paymentsStatus, /data-dx-stripe-price-env=\{selectedPlan\.priceEnv\}/);
  assert.match(paymentsStatus, /data-dx-stripe-action="refresh-config"/);
  assert.match(paymentsStatus, /data-dx-stripe-action="select-plan"/);
  assert.match(paymentsStatus, /data-dx-stripe-action="request-checkout-intent"/);
  assert.match(paymentsStatus, /data-dx-stripe-submit-state=\{formSubmitState\.kind\}/);
  assert.match(paymentsStatus, /data-dx-stripe-local-receipt=\{localReceipt\?\.id \?\? "none"\}/);
  assert.match(paymentsStatus, /data-dx-stripe-receipt-status=\{localReceipt\?\.status \?\? "idle"\}/);
  assert.match(paymentsStatus, /data-dx-stripe-receipt-path=\{stripeBillingWorkflowReceiptPath\}/);
  assert.match(paymentsStatus, /<dx-icon name="pack:payments"/);
  assert.match(paymentsStatus, /createLaunchStripePreviewReceipt/);
  assert.match(paymentsStatus, /status\.kind === "missing-config"/);
  assert.match(paymentsStatus, /Local preview receipt/);
  assert.doesNotMatch(paymentsStatus, /cardNumber|4242|fake card/i);

  assert.match(stripeSource, /launchUsage: \{/);
  assert.match(stripeSource, /componentMarker: 'data-dx-component="launch-billing-checkout-workflow"'/);
  assert.match(stripeSource, /dashboardFlowMarker: 'data-dx-dashboard-flow="billing-checkout"'/);
  assert.match(stripeSource, /sourceFile: "examples\/template\/payments-status\.tsx"/);
  assert.match(stripeSource, /previewManifestSurface: "launch-runtime-billing-checkout"/);
  assert.match(stripeSource, /materializerFile: "tools\/launch\/materialize-www-template\.ts"/);
  assert.match(stripeSource, /receiptPathMarker: 'data-dx-stripe-receipt-path="examples\/template\/\.dx\/forge\/receipts\/2026-05-22-payments-stripe-js-billing-workflow\.json"'/);
  assert.match(stripeSource, /2026-05-22-payments-stripe-js-billing-workflow\.json/);

  assert.match(cli, /include_str!\("\.\.\/\.\.\/\.\.\/examples\/template\/payments-status\.tsx"\)/);
  assert.match(
    cli,
    /const NEXT_FAMILIAR_STRIPE_BILLING_WORKFLOW_RECEIPT_JSON: &str =\s*include_str!\(\s*"\.\.\/\.\.\/\.\.\/examples\/template\/\.dx\/forge\/receipts\/2026-05-22-payments-stripe-js-billing-workflow\.json"\s*\);/,
  );
  assert.match(cli, /"components\/template-app\/payments-status\.tsx"/);
  assert.match(
    newCommand,
    /"\.dx\/forge\/receipts\/2026-05-22-payments-stripe-js-billing-workflow\.json"/,
  );
  assert.match(
    newCommand,
    /path:\s*"\.dx\/forge\/receipts\/2026-05-22-payments-stripe-js-billing-workflow\.json"\s*\.to_string\(\),\s*content: NEXT_FAMILIAR_STRIPE_BILLING_WORKFLOW_RECEIPT_JSON\.to_string\(\),/,
  );
  assert.match(newCommand, /project_dir\.join\("app\/page\.tsx"\)/);
  assert.match(newCommand, /"components\/template-app\/template-shell\.tsx"/);
  assert.match(defaultTemplateSources, /source_file: "examples\/template\/app\/page\.tsx"/);
  assert.match(defaultTemplateSources, /materialized_file: "app\/page\.tsx"/);
  assert.match(
    defaultTemplateSources,
    /include_str!\("\.\.\/\.\.\/\.\.\/examples\/template\/app\/page\.tsx"\)/,
  );
  assert.match(cli, /"components\/template-app\/payments-status\.tsx"/);
});
