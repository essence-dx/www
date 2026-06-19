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

test("launch template tracks the requested DX-WWW framework completeness lanes", () => {
  const framework = read("examples/template/framework-completeness.ts");
  const registry = read("examples/template/template-surface-registry.ts");
  const shell = read("examples/template/template-shell.tsx");
  const routeContract = read("examples/template/template-route-contract.ts");
  const cli = [
    read("dx-www/src/cli/mod_parts/next_familiar_template.rs"),
    read("dx-www/src/cli/new_command.rs"),
  ].join("\n");
  const projectCheck = read("core/src/ecosystem/project_check/readiness.rs");

  for (const lane of [
    "routing-parity",
    "server-client-model",
    "dev-experience",
    "production-template",
    "package-ecosystem",
  ]) {
    assert.match(framework, new RegExp(`"${lane}"`));
  }

  for (const feature of [
    "nested-layouts",
    "loading-error-not-found-boundaries",
    "route-groups",
    "dynamic-params",
    "metadata-seo",
    "route-handlers",
    "server-actions-equivalent",
    "form-actions",
    "cookies-headers-session-helpers",
    "streaming-response-boundary",
    "cache-revalidate-story",
    "reliable-hot-reload",
    "tsx-first-templates",
    "auto-imports",
    "dx-style-css-generation",
    "dx-check-receipts",
    "obvious-cli-path",
    "real-dashboard-starter",
    "auth-page",
    "settings-validation-form",
    "payment-plan-page",
    "database-backed-table-boundary",
    "docs-content-route",
    "ai-chat-route",
    "visual-studio-markers",
  ]) {
    assert.match(framework, new RegExp(`"${feature}"`));
  }

  for (const packageId of [
    "auth/better-auth",
    "validation/zod",
    "forms/react-hook-form",
    "state/zustand",
    "tanstack/query",
    "db/drizzle-sqlite",
    "supabase/client",
    "payments/stripe-js",
    "i18n/next-intl",
    "content/react-markdown",
    "content/fumadocs-next",
    "animation/motion",
    "dx/icon/search",
  ]) {
    assert.match(framework, new RegExp(packageId.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(framework, /publicAuthoring: "tsx-app-router"/);
  assert.match(framework, /packagePolicy: "forge-source-owned-visible-files"/);
  assert.match(framework, /stylePolicy: "dx-style-generated-css"/);
  assert.match(framework, /checkPolicy: "dx-check-receipts"/);
  assert.match(framework, /dxWwwFrameworkCompletenessSummary/);
  assert.match(framework, /dxWwwFrameworkCompletenessScore/);

  assert.match(registry, /from "\.\/framework-completeness"/);
  assert.match(registry, /frameworkCompletenessScore/);
  assert.match(registry, /frameworkCompletenessSchema/);

  assert.match(shell, /data-dx-component="dx-www-framework-completeness"/);
  assert.match(shell, /data-dx-framework-completeness-score/);

  assert.match(routeContract, /components\/template-app\/framework-completeness\.ts/);
  assert.match(routeContract, /dx\.www\.framework_completeness/);

  assert.match(cli, /NEXT_FAMILIAR_FRAMEWORK_COMPLETENESS_TS/);
  assert.match(cli, /components\/template-app\/framework-completeness\.ts/);
  assert.match(cli, /examples\/onboard\/framework-completeness\.ts/);

  assert.match(projectCheck, /framework_completeness_metrics_and_findings/);
  assert.match(projectCheck, /framework_completeness_contract_present/);
  assert.match(projectCheck, /framework_completeness_features_present/);
});
