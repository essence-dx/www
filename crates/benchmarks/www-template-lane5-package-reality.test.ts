import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

const lane5PackageIds = [
  "payments/stripe-js",
  "ai/vercel-ai",
  "automations/n8n",
] as const;

const lane5TemplateReadinessReceipts = {
  "payments/stripe-js": {
    file: "examples/template/.dx/forge/template-readiness/payments.json",
    officialName: "Payments",
    route: "/api/payments/stripe-js/readiness",
    routeHandler: "app/api/payments/stripe-js/readiness/route.ts",
    requiredEnv: [
      "NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY",
      "STRIPE_SECRET_KEY",
      "STRIPE_PRICE_ID",
    ],
  },
  "ai/vercel-ai": {
    file: "examples/template/.dx/forge/template-readiness/ai-sdk.json",
    officialName: "AI SDK",
    route: "/api/ai/chat",
    routeHandler: "app/api/ai/chat/route.ts",
    requiredEnv: ["AI_PROVIDER_API_KEY"],
  },
  "automations/n8n": {
    file: "examples/template/.dx/forge/template-readiness/automation-connectors.json",
    officialName: "Automation Connectors",
    route: "/api/automations/n8n/dry-run",
    routeHandler: "app/api/automations/n8n/dry-run/route.ts",
    requiredEnv: [
      "SLACK_BOT_TOKEN",
      "SLACK_CLIENT_ID",
      "SLACK_CLIENT_SECRET",
      "NOTION_API_KEY",
      "NOTION_CLIENT_ID",
      "NOTION_CLIENT_SECRET",
    ],
  },
} as const;

const lane5PackageFiles = {
  "payments/stripe-js": [
    "examples/template/lib/payments/stripe-js/config.ts",
    "examples/template/lib/payments/stripe-js/dashboard-checkout.ts",
    "examples/template/lib/payments/stripe-js/server.ts",
    "examples/template/app/api/checkout/route.ts",
    "examples/template/app/api/payments/stripe-js/readiness/route.ts",
    "examples/template/app/api/stripe/webhook/route.ts",
  ],
  "ai/vercel-ai": [
    "examples/template/lib/ai/chat-route.ts",
    "examples/template/lib/ai/client-chat.tsx",
    "examples/template/lib/ai/dashboard-readiness.ts",
    "examples/template/lib/ai/provider-boundary.ts",
    "examples/template/app/api/ai/chat/route.ts",
  ],
  "automations/n8n": [
    "examples/template/lib/automations/n8n/catalog.ts",
    "examples/template/lib/automations/n8n/readiness.ts",
    "examples/template/lib/automations/n8n/receipt.ts",
    "examples/template/app/api/automations/n8n/dry-run/route.ts",
  ],
} as const;

function read(relativePath: string) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath: string) {
  return JSON.parse(read(relativePath));
}

test("lane 5 packages are lock-backed and materialized in the launch template", () => {
  const lock = readJson("examples/template/.dx/forge/package-lock.json");
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const sourceManifest = readJson("examples/template/.dx/forge/source-manifest.json");
  const realitySource = read("examples/template/components/template-app/package-reality.ts");
  const realityPanelSource = read(
    "examples/template/components/template-app/package-reality-panel.tsx",
  );
  const materializedDashboard = read(".dx/template-app-browser-preview/pages/dashboard.html");
  const materializerSource = read("tools/launch/materialize-www-template.ts");
  const dashboardStateSource = read(
    "examples/template/components/template-app/dashboard-state.ts",
  );

  for (const packageId of lane5PackageIds) {
    assert.ok(
      lock.packages.some((entry: { name: string }) => entry.name === packageId),
      `${packageId} should be promoted into the Forge package lock`,
    );
    assert.ok(
      status.locked_package_names.includes(packageId),
      `${packageId} should be visible as a lock-backed package`,
    );
    assert.ok(
      sourceManifest.packages.some(
        (entry: { package_id: string }) => entry.package_id === packageId,
      ),
      `${packageId} should be backed by the source manifest`,
    );
  }

  for (const [packageId, files] of Object.entries(lane5PackageFiles)) {
    const lockedPackage = lock.packages.find(
      (entry: { name: string }) => entry.name === packageId,
    );
    assert.ok(lockedPackage, `${packageId} missing package-lock entry`);

    for (const file of files) {
      assert.ok(fs.existsSync(path.join(root, file)), `${packageId} missing ${file}`);
      assert.ok(
        lockedPackage.files.some(
          (entry: { path: string }) =>
            entry.path === file.replace("examples/template/", ""),
        ),
        `${packageId} should lock ${file}`,
      );
    }
  }

  for (const [packageId, expected] of Object.entries(lane5TemplateReadinessReceipts)) {
    assert.ok(fs.existsSync(path.join(root, expected.file)), `${packageId} missing readiness receipt`);
    assert.ok(
      fs.existsSync(
        path.join(
          root,
          expected.file.replace(
            "examples/template/",
            ".dx/template-app-browser-preview/",
          ),
        ),
      ),
      `${packageId} missing materialized preview readiness receipt`,
    );
    const receipt = readJson(expected.file);

    assert.equal(receipt.schema, "dx.forge.template_readiness.package");
    assert.equal(receipt.package_id, packageId);
    assert.equal(receipt.official_package_name, expected.officialName);
    assert.equal(receipt.classification, "provider-gated");
    assert.equal(receipt.runtime_proof, false);
    assert.equal(receipt.live_provider_execution, false);
    assert.equal(receipt.secret_access, false);
    assert.equal(receipt.readiness_route, expected.route);
    assert.equal(receipt.app_route_handler, expected.routeHandler);
    assert.deepEqual(receipt.required_env, expected.requiredEnv);
    assert.ok(
      receipt.source_owned_surfaces.includes(expected.routeHandler),
      `${packageId} receipt should name its route handler`,
    );
    assert.ok(
      receipt.blocked_until_configured.length > 0,
      `${packageId} receipt should name missing provider prerequisites`,
    );
    assert.ok(
      receipt.honesty_notes.some((note: string) => note.includes("not live")),
      `${packageId} receipt should explicitly avoid live-provider claims`,
    );

    const templateRelativeReceipt = expected.file.replace("examples/template/", "");
    assert.match(realitySource, new RegExp(templateRelativeReceipt.replaceAll("/", "\\/")));
    assert.match(materializedDashboard, new RegExp(templateRelativeReceipt.replaceAll("/", "\\/")));
  }

  const automationsDryRunRoute = read(
    "examples/template/app/api/automations/n8n/dry-run/route.ts",
  );
  assert.match(automationsDryRunRoute, /createDxN8nRunReceipt/);
  assert.match(automationsDryRunRoute, /buildDxN8nCredentialReadiness/);
  assert.match(automationsDryRunRoute, /runtimeExecution: false/);
  assert.match(automationsDryRunRoute, /"missing-config"/);
  assert.match(automationsDryRunRoute, /\{ status: receipt\.status === "blocked-missing-config" \? 501 : 202 \}/);

  const paymentsReadinessRoute = read(
    "examples/template/app/api/payments/stripe-js/readiness/route.ts",
  );
  assert.match(paymentsReadinessRoute, /createDxStripeDashboardMissingConfigReceipt/);
  assert.match(paymentsReadinessRoute, /dxStripeDashboardCheckoutReadiness/);
  assert.match(paymentsReadinessRoute, /runtimeExecution: false/);
  assert.match(paymentsReadinessRoute, /stripeLiveExecution: false/);
  assert.match(paymentsReadinessRoute, /secretValues: \[\]/);
  assert.match(paymentsReadinessRoute, /"missing-config"/);
  assert.match(paymentsReadinessRoute, /\{ status: readiness\.status === "missing-config" \? 501 : 202 \}/);

  const checkoutRoute = read("examples/template/app/api/checkout/route.ts");
  assert.match(checkoutRoute, /kind: status === 501 \? "provider-boundary" : "contact"/);
  assert.match(checkoutRoute, /return 501/);

  const webhookRoute = read("examples/template/app/api/stripe/webhook/route.ts");
  const materializedWebhookRoute = read(
    ".dx/template-app-browser-preview/app/api/stripe/webhook/route.ts",
  );
  const materializedStripeServer = read(
    ".dx/template-app-browser-preview/lib/payments/stripe-js/server.ts",
  );
  assert.match(webhookRoute, /verifyDxStripeWebhookRequest/);
  assert.match(webhookRoute, /routeDxStripeWebhookEvent/);
  assert.match(webhookRoute, /fulfillmentStatus: "app-owned"/);
  assert.match(webhookRoute, /createDxStripeWebhookProviderBoundaryResponse/);
  assert.match(webhookRoute, /isDxStripeWebhookProviderBoundaryError/);
  assert.match(webhookRoute, /dx\.payments\.stripe_js\.webhook_boundary/);
  assert.match(webhookRoute, /providerBoundary: true/);
  assert.match(webhookRoute, /webhookVerified: false/);
  assert.match(webhookRoute, /stripeLiveExecution: false/);
  assert.match(webhookRoute, /secretValues: \[\]/);
  assert.match(webhookRoute, /\{ status: 501 \}/);
  assert.match(webhookRoute, /\{ status: 400 \}/);
  assert.match(materializedWebhookRoute, /createDxStripeWebhookProviderBoundaryResponse/);
  assert.match(materializedWebhookRoute, /dx\.payments\.stripe_js\.webhook_boundary/);
  assert.match(materializedStripeServer, /STRIPE_WEBHOOK_SECRET/);
  assert.match(materializerSource, /app\/api\/stripe\/webhook\/route\.ts/);
  assert.match(materializerSource, /lib\/payments\/stripe-js\/server\.ts/);

  const aiRoute = read("examples/template/app/api/ai/chat/route.ts");
  const aiProviderBoundary = read("examples/template/lib/ai/provider-boundary.ts");
  assert.match(aiRoute, /createDxAiMissingProviderResponse/);
  assert.match(aiProviderBoundary, /httpStatus: 501/);
  assert.match(aiProviderBoundary, /\{ status: 501 \}/);

  assert.match(realitySource, /"payments\/stripe-js": \{/);
  assert.match(realitySource, /"ai\/vercel-ai": \{/);
  assert.match(realitySource, /"automations\/n8n": \{/);
  assert.match(realitySource, /Stripe checkout boundary/);
  assert.match(realitySource, /\/api\/stripe\/webhook/);
  assert.match(realitySource, /webhook boundary route/);
  assert.match(realitySource, /webhook verification/);
  assert.match(realitySource, /AI route boundary/);
  assert.match(realitySource, /n8n connector boundary/);
  assert.match(realitySource, /lane5ProviderReadinessRows/);
  assert.match(realitySource, /\/api\/payments\/stripe-js\/readiness/);
  assert.match(realitySource, /\/api\/ai\/chat/);
  assert.match(realitySource, /\/api\/automations\/n8n\/dry-run/);
  assert.match(realitySource, /runtimeExecution: false/);
  assert.match(realitySource, /secretValues: \[\]/);
  assert.match(realitySource, /readinessReceipt/);
  assert.match(realityPanelSource, /data-dx-lane5-provider-readiness/);
  assert.match(realityPanelSource, /data-dx-provider-endpoint/);
  assert.match(realityPanelSource, /data-dx-provider-boundary-endpoints/);
  assert.match(realityPanelSource, /data-dx-provider-boundary/);
  assert.match(realityPanelSource, /data-dx-template-readiness-receipt/);
  assert.match(realityPanelSource, /Provider readiness/);
  assert.match(materializedDashboard, /data-dx-template-readiness-receipt/);
  assert.match(dashboardStateSource, /stripe-checkout-boundary/);
  assert.match(dashboardStateSource, /Payments setup reviewed/);
  assert.match(dashboardStateSource, /AI setup reviewed/);
  assert.match(dashboardStateSource, /Automation receipt drafted/);
});

test("AI provider routes return honest missing-config boundaries before live execution", () => {
  const providerRoutes = [
    "examples/template/app/api/ai/agent/route.ts",
    "examples/template/app/api/ai/chat/route.ts",
    "examples/template/app/api/ai/image/route.ts",
    "examples/template/app/api/ai/object/route.ts",
    "examples/template/app/api/ai/speech/route.ts",
    "examples/template/app/api/ai/transcribe/route.ts",
    "examples/template/app/api/ai/upload-file/route.ts",
  ];

  for (const routePath of providerRoutes) {
    const source = read(routePath);
    assert.match(source, /createDxAiMissingProviderResponse/);
    assert.match(source, /process\.env\.AI_PROVIDER_API_KEY/);
    assert.doesNotMatch(source, /process\.env\.OPENAI_API_KEY/);
  }

  const videoRoute = read("examples/template/app/api/ai/video/route.ts");
  assert.match(videoRoute, /createDxAiMissingProviderResponse/);
  assert.match(videoRoute, /process\.env\.AI_GATEWAY_API_KEY/);

  const providerBoundary = read("examples/template/lib/ai/provider-boundary.ts");
  assert.match(providerBoundary, /\{ status: 501 \}/);
  assert.match(providerBoundary, /secretValues: \[\]/);
  assert.match(providerBoundary, /runtimeExecution: false/);
  assert.match(providerBoundary, /modelStreaming: false/);
  assert.match(providerBoundary, /providerRuntime: false/);
});
