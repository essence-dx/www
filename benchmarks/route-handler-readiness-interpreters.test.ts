import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function maybeRead(relativePath) {
  const absolutePath = path.join(root, relativePath);
  return fs.existsSync(absolutePath) ? fs.readFileSync(absolutePath, "utf8") : "";
}

test("DX route-handler runtime owns launch readiness helper interpreters", () => {
  const databaseRoute = read("examples/template/app/api/database-api/readiness/route.ts");
  const databaseHelper = read("examples/template/server/database-api/readiness.ts");
  const databaseSourceContract = read("examples/template/lib/database-api/source-contract.ts");
  const databaseOrmRoute = read("examples/template/app/api/database-orm/readiness/route.ts");
  const databaseOrmHelper = read("examples/template/server/database-orm/readiness.ts");
  const supabaseRoute = read("examples/template/app/api/supabase/readiness/route.ts");
  const supabaseHelper = read("examples/template/server/supabase/readiness.ts");
  const supabaseEnv = read("examples/template/lib/supabase/env.ts");
  const queryRoute = read("examples/template/app/api/query-cache/readiness/route.ts");
  const queryHelper = read("examples/template/server/query-cache/readiness.ts");
  const instantRoute = read("examples/template/app/api/instant/route.ts");
  const instantReadinessRoute = read("examples/template/app/api/instant/readiness/route.ts");
  const instantReadinessHelper = read("examples/template/server/instant/readiness.ts");
  const instantHelper = read("examples/template/lib/instant/route.ts");
  const instantEnv = read("examples/template/lib/instant/env.ts");
  const instantSchema = read("examples/template/lib/instant/schema.ts");
  const authRoute = read("examples/template/app/api/auth/readiness/route.ts");
  const authSessionRoute = read("examples/template/app/api/auth/session/route.ts");
  const authCatchAllRoute = read("examples/template/app/api/auth/[...all]/route.ts");
  const authServerBoundary = read("examples/template/server/auth/better-auth.ts");
  const authServer = read("examples/template/auth/better-auth/server.ts");
  const n8nRoute = read("examples/template/app/api/automations/n8n/dry-run/route.ts");
  const n8nReceipt = read("examples/template/lib/automations/n8n/receipt.ts");
  const n8nReadiness = read("examples/template/lib/automations/n8n/readiness.ts");
  const stripeRoute = read("examples/template/app/api/payments/stripe-js/readiness/route.ts");
  const stripeCheckoutRoute = read("examples/template/app/api/checkout/route.ts");
  const stripeWebhookRoute = read("examples/template/app/api/stripe/webhook/route.ts");
  const stripeServer = read("examples/template/lib/payments/stripe-js/server.ts");
  const stripeCheckout = read("examples/template/lib/payments/stripe-js/dashboard-checkout.ts");
  const aiRoute = read("examples/template/app/api/ai/chat/route.ts");
  const aiTextStreamRoute = read("examples/template/app/api/ai/text-stream/route.ts");
  const aiTextStreamHelper = read("examples/template/lib/ai/text-stream.ts");
  const aiUiStreamRoute = read("examples/template/app/api/ai/ui-stream/route.ts");
  const aiUiStreamHelper = read("examples/template/lib/ai/ui-message-stream.ts");
  const aiProviderBoundary = read("examples/template/lib/ai/provider-boundary.ts");
  const fumadocsSearchRoute = read("examples/template/app/api/search/route.ts");
  const fumadocsStaticSearchRoute = read("examples/template/app/api/search-static/route.ts");
  const fumadocsOpenApiProxyRoute = read("examples/template/app/api/openapi/proxy/route.ts");
  const fumadocsSearchHelper = read("examples/template/lib/fumadocs/search.ts");
  const fumadocsOpenApiHelper = read("examples/template/lib/fumadocs/openapi.ts");
  const fumadocsRouteContract = read("examples/template/lib/fumadocs/route-contract.ts");
  const serverContract = read("core/src/delivery/server_contract.rs");
  const routeHandlerCompat = read("core/src/delivery/route_handler_compat.rs");
  const routeHandlerDatabaseOrm = read("core/src/delivery/route_handler_database_orm.rs");
  const routeHandlerFumadocs = maybeRead("core/src/delivery/route_handler_fumadocs.rs");
  const routeHandlerInstantReadiness = maybeRead("core/src/delivery/route_handler_instant_readiness.rs");
  const routeHandlerSupabase = maybeRead("core/src/delivery/route_handler_supabase.rs");
  const routeHandlerPayments = read("core/src/delivery/route_handler_payments.rs");
  const routeHandlerAi = read("core/src/delivery/route_handler_ai.rs");
  const routeHandlerAutomations = read("core/src/delivery/route_handler_automations.rs");
  const appRouteHandlerBuildOutput = read("dx-www/src/cli/app_route_handler_build_output.rs");
  const cli = read("dx-www/src/cli/mod.rs");
  const routeHandlerRuntimeEnv = maybeRead("dx-www/src/cli/route_handler_runtime_env.rs");
  const launchMaterializer = read("tools/launch/materialize-www-template.ts");

  assert.match(serverContract, /impl DxReactRouteHandlerRequest/);
  assert.match(serverContract, /pub\(crate\) fn path_for_match\(&self\) -> &str/);
  assert.match(routeHandlerCompat, /request\.path_for_match\(\)\.trim_end_matches\('\/'\)/);
  assert.doesNotMatch(routeHandlerCompat, /request\.path\.trim_end_matches\('\/'\)/);
  for (const [name, source] of [
    ["database ORM", routeHandlerDatabaseOrm],
    ["supabase", routeHandlerSupabase],
    ["instant readiness", routeHandlerInstantReadiness],
    ["payments", routeHandlerPayments],
    ["AI", routeHandlerAi],
    ["automations", routeHandlerAutomations],
    ["Fumadocs", routeHandlerFumadocs],
  ]) {
    assert.match(
      source,
      /request\.path_for_match\(\)\.trim_end_matches\('\/'\)/,
      `${name} route handler should share normalized request URL matching`,
    );
    assert.doesNotMatch(
      source,
      /request\.path\.trim_end_matches\('\/'\)/,
      `${name} route handler should not match against the raw request URL`,
    );
  }

  assert.match(databaseRoute, /createDatabaseApiReadinessResponse/);
  assert.match(databaseHelper, /readDatabaseApiRouteReadiness/);
  assert.match(databaseSourceContract, /databaseApiSourceContract/);
  assert.match(serverContract, /database_api_readiness_route_handler_response/);
  assert.match(routeHandlerCompat, /source-owned-database-api-readiness-interpreter/);
  assert.match(routeHandlerCompat, /dx\.www\.template\.database_api_readiness/);
  assert.match(routeHandlerCompat, /dx\.www\.template\.database_api_source_contract/);
  assert.match(routeHandlerCompat, /createDatabaseApiReadinessResponse\(\)/);

  assert.match(databaseOrmRoute, /createDatabaseOrmReadinessResponse/);
  assert.match(databaseOrmHelper, /readDatabaseOrmReadiness/);
  assert.match(databaseOrmHelper, /DX_DATABASE_URL or DX_SQLITE_DATABASE_PATH/);
  assert.match(serverContract, /database_orm_readiness_route_handler_response/);
  assert.match(routeHandlerDatabaseOrm, /source-owned-database-orm-readiness-interpreter/);
  assert.match(routeHandlerDatabaseOrm, /dx\.www\.template\.database_orm_readiness/);
  assert.match(routeHandlerDatabaseOrm, /createDatabaseOrmReadinessResponse\(\)/);
  assert.match(routeHandlerDatabaseOrm, /x-dx-database-orm-readiness/);
  assert.match(routeHandlerDatabaseOrm, /DX_DATABASE_MIGRATIONS_REVIEWED/);
  assert.match(routeHandlerDatabaseOrm, /tenant authorization policy/);
  assert.match(routeHandlerDatabaseOrm, /runtime_env/);
  assert.match(routeHandlerDatabaseOrm, /database_orm_missing_config/);
  assert.match(routeHandlerDatabaseOrm, /configured-source-owned-adapter-boundary/);
  assert.match(routeHandlerDatabaseOrm, /Database ORM runtime inputs are locally acknowledged/);
  assert.doesNotMatch(
    routeHandlerDatabaseOrm,
    /Some\(DxReactRouteHandlerResponse \{[\s\S]{0,240}status: 501,/,
  );

  assert.match(supabaseRoute, /createSupabaseReadinessResponse/);
  assert.match(supabaseHelper, /readSupabaseReadiness/);
  assert.match(supabaseHelper, /NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY/);
  assert.match(supabaseEnv, /readSupabasePublicConfig/);
  if (routeHandlerSupabase) {
    assert.match(serverContract, /supabase_readiness_route_handler_response/);
    assert.match(routeHandlerSupabase, /source-owned-supabase-readiness-interpreter/);
    assert.match(routeHandlerSupabase, /dx\.www\.template\.supabase_readiness/);
    assert.match(routeHandlerSupabase, /createSupabaseReadinessResponse\(\)/);
    assert.match(routeHandlerSupabase, /x-dx-supabase-readiness/);
    assert.match(routeHandlerSupabase, /NEXT_PUBLIC_SUPABASE_URL/);
    assert.match(routeHandlerSupabase, /hosted read\/write\/realtime proof/);
    assert.match(routeHandlerSupabase, /runtime_env/);
    assert.match(routeHandlerSupabase, /supabase_missing_env/);
    assert.match(routeHandlerSupabase, /supabase_validation/);
    assert.match(routeHandlerSupabase, /configured-source-owned-adapter-boundary/);
    assert.match(routeHandlerSupabase, /Supabase public configuration validates locally/);
    assert.doesNotMatch(
      routeHandlerSupabase,
      /Some\(DxReactRouteHandlerResponse \{[\s\S]{0,240}status: 501,/,
    );
  }

  assert.match(queryRoute, /createDataFetchingCacheReadinessResponse/);
  assert.match(queryHelper, /readDataFetchingCacheReadiness/);
  assert.match(queryHelper, /createDashboardQueryCacheStatus/);
  assert.match(serverContract, /data_fetching_cache_readiness_route_handler_response/);
  assert.match(routeHandlerCompat, /source-owned-data-fetching-cache-readiness-interpreter/);
  assert.match(routeHandlerCompat, /source-owned-data-fetching-cache-action-interpreter/);
  assert.match(serverContract, /data_fetching_cache_action_route_handler_response/);
  assert.match(routeHandlerCompat, /dx\.www\.template\.data_fetching_cache_readiness/);
  assert.match(routeHandlerCompat, /dx\.www\.template\.data_fetching_cache_action_receipt/);
  assert.match(routeHandlerCompat, /createDataFetchingCacheReadinessResponse\(request\)/);
  assert.match(routeHandlerCompat, /createDataFetchingCacheActionResponse\(request\)/);
  assert.match(routeHandlerCompat, /queryclient-adapter-required/);
  assert.match(routeHandlerCompat, /queryClientExecution/);
  assert.match(routeHandlerCompat, /"cache-control"\.to_string\(\),\s*"no-store"\.to_string\(\)/);

  assert.match(instantRoute, /export const \{ GET, POST \} = createDxInstantRouteHandlers\(\)/);
  assert.match(instantHelper, /createInstantRouteHandler\(\{ appId \}\)/);
  assert.match(instantHelper, /createInstantMissingConfigResponse/);
  assert.match(instantHelper, /status: 501/);
  assert.match(instantEnv, /requiredEnv\(env, "NEXT_PUBLIC_INSTANT_APP_ID"\)/);
  assert.match(instantSchema, /rooms:\s*\{\s*launch:/);
  assert.match(serverContract, /instant_route_handler_compat_response/);
  assert.match(routeHandlerCompat, /source-owned-instantdb-route-handler-interpreter/);
  assert.match(routeHandlerCompat, /dx\.www\.template\.instantdb_route_handler/);
  assert.match(routeHandlerCompat, /NEXT_PUBLIC_INSTANT_APP_ID/);
  assert.match(routeHandlerCompat, /x-dx-instant-route-handler/);
  assert.match(routeHandlerCompat, /createDxInstantRouteHandlers/);
  assert.match(routeHandlerCompat, /instant_route_handler_post_body/);
  assert.match(routeHandlerCompat, /configured-source-owned-dry-run/);
  assert.match(routeHandlerCompat, /InstantDB app ID is configured locally; POST returns a local route receipt/);

  assert.match(instantReadinessRoute, /createInstantReadinessResponse/);
  assert.match(instantReadinessHelper, /readInstantReadiness/);
  assert.match(instantReadinessHelper, /NEXT_PUBLIC_INSTANT_APP_ID/);
  if (routeHandlerInstantReadiness) {
    assert.match(serverContract, /instant_readiness_route_handler_response/);
    assert.match(routeHandlerInstantReadiness, /source-owned-instant-readiness-interpreter/);
    assert.match(routeHandlerInstantReadiness, /dx\.www\.template\.instant_readiness/);
    assert.match(routeHandlerInstantReadiness, /createInstantReadinessResponse\(\)/);
    assert.match(routeHandlerInstantReadiness, /x-dx-instant-readiness/);
    assert.match(routeHandlerInstantReadiness, /NEXT_PUBLIC_INSTANT_APP_ID/);
    assert.match(routeHandlerInstantReadiness, /storage and stream runtime proof/);
    assert.match(routeHandlerInstantReadiness, /runtime_env/);
    assert.match(routeHandlerInstantReadiness, /instant_missing_env/);
    assert.match(routeHandlerInstantReadiness, /instant_validation_error/);
    assert.match(routeHandlerInstantReadiness, /configured-source-owned-adapter-boundary/);
    assert.match(routeHandlerInstantReadiness, /InstantDB public configuration validates locally/);
    assert.doesNotMatch(
      routeHandlerInstantReadiness,
      /Some\(DxReactRouteHandlerResponse \{[\s\S]{0,240}status: 501,/,
    );
  }

  assert.match(serverContract, /pub runtime_env: BTreeMap<String, String>/);
  assert.match(appRouteHandlerBuildOutput, /runtime_env: BTreeMap::new\(\)/);
  assert.match(cli, /mod route_handler_runtime_env;/);
  assert.match(cli, /route_handler_runtime_env::route_handler_runtime_env\(\)/);
  assert.match(routeHandlerRuntimeEnv, /DX_DATABASE_URL/);
  assert.match(routeHandlerRuntimeEnv, /DX_SQLITE_DATABASE_PATH/);
  assert.match(routeHandlerRuntimeEnv, /BETTER_AUTH_SECRET/);
  assert.match(routeHandlerRuntimeEnv, /BETTER_AUTH_URL/);
  assert.match(routeHandlerRuntimeEnv, /NEXT_PUBLIC_BETTER_AUTH_URL/);
  assert.match(routeHandlerRuntimeEnv, /GOOGLE_CLIENT_ID/);
  assert.match(routeHandlerRuntimeEnv, /GOOGLE_CLIENT_SECRET/);
  assert.match(routeHandlerRuntimeEnv, /NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY/);
  assert.match(routeHandlerRuntimeEnv, /STRIPE_SECRET_KEY/);
  assert.match(routeHandlerRuntimeEnv, /STRIPE_PRICE_ID_TEAM/);
  assert.match(routeHandlerRuntimeEnv, /SLACK_BOT_TOKEN/);
  assert.match(routeHandlerRuntimeEnv, /SLACK_CLIENT_ID/);
  assert.match(routeHandlerRuntimeEnv, /SLACK_CLIENT_SECRET/);
  assert.match(routeHandlerRuntimeEnv, /NOTION_API_KEY/);
  assert.match(routeHandlerRuntimeEnv, /NOTION_CLIENT_ID/);
  assert.match(routeHandlerRuntimeEnv, /NOTION_CLIENT_SECRET/);
  assert.match(routeHandlerRuntimeEnv, /AI_PROVIDER_API_KEY/);
  assert.match(routeHandlerRuntimeEnv, /AI_GATEWAY_API_KEY/);
  assert.match(routeHandlerRuntimeEnv, /DX_ENABLE_EXTENDED_AI_ROUTES/);
  assert.match(
    routeHandlerRuntimeEnv,
    /const SENSITIVE_PRESENCE_ENV:[\s\S]*"BETTER_AUTH_SECRET"[\s\S]*"GOOGLE_CLIENT_SECRET"/,
  );
  assert.doesNotMatch(
    routeHandlerRuntimeEnv,
    /const PUBLIC_VALUE_ENV:[\s\S]*"BETTER_AUTH_SECRET"/,
  );
  assert.doesNotMatch(
    routeHandlerRuntimeEnv,
    /const PUBLIC_VALUE_ENV:[\s\S]*"GOOGLE_CLIENT_SECRET"/,
  );
  assert.doesNotMatch(
    routeHandlerRuntimeEnv,
    /const PUBLIC_VALUE_ENV:[\s\S]*"STRIPE_SECRET_KEY"/,
  );
  assert.doesNotMatch(
    routeHandlerRuntimeEnv,
    /const PUBLIC_VALUE_ENV:[\s\S]*"STRIPE_PRICE_ID_TEAM"/,
  );
  assert.doesNotMatch(
    routeHandlerRuntimeEnv,
    /const PUBLIC_VALUE_ENV:[\s\S]*"SLACK_BOT_TOKEN"/,
  );
  assert.doesNotMatch(
    routeHandlerRuntimeEnv,
    /const PUBLIC_VALUE_ENV:[\s\S]*"NOTION_API_KEY"/,
  );
  assert.doesNotMatch(
    routeHandlerRuntimeEnv,
    /const PUBLIC_VALUE_ENV:[\s\S]*"AI_PROVIDER_API_KEY"/,
  );
  assert.doesNotMatch(
    routeHandlerRuntimeEnv,
    /const PUBLIC_VALUE_ENV:[\s\S]*"AI_GATEWAY_API_KEY"/,
  );
  assert.match(routeHandlerRuntimeEnv, /NEXT_PUBLIC_SUPABASE_URL/);
  assert.match(routeHandlerRuntimeEnv, /NEXT_PUBLIC_INSTANT_APP_ID/);
  assert.match(routeHandlerRuntimeEnv, /redacted-present/);

  assert.match(authRoute, /createTemplateBetterAuthReadiness/);
  assert.match(authRoute, /\{ status: 200 \}/);
  assert.match(
    authRoute,
    /\.\.\.readiness,\s*packageReadinessStatus: readiness\.status,\s*status: readiness\.canRunRouteHandlers/,
  );
  assert.doesNotMatch(
    authRoute,
    /status: readiness\.canRunRouteHandlers[\s\S]*\.\.\.readiness/,
  );
  assert.match(authRoute, /liveRouteHandlersHttpStatus: readiness\.canRunRouteHandlers \? 200 : 501/);
  assert.match(authServerBoundary, /dxTemplateBetterAuthDatabaseBoundary/);
  assert.match(authServerBoundary, /migrationsRequired: !readiness\.databaseAdapterConfigured/);
  assert.match(authServerBoundary, /\{ status: 501 \}/);
  assert.match(authServerBoundary, /liveSessionExecution: false/);
  assert.match(authServer, /canRunRouteHandlers: config\.configured && databaseAdapterConfigured/);
  assert.match(serverContract, /template_better_auth_readiness_route_handler_response/);
  assert.match(routeHandlerCompat, /source-owned-template-better-auth-readiness-interpreter/);
  assert.match(routeHandlerCompat, /dx\.template\.authentication\.database_boundary/);
  assert.match(routeHandlerCompat, /createTemplateBetterAuthReadiness\(\)/);
  assert.match(routeHandlerCompat, /BETTER_AUTH_SECRET/);
  assert.match(routeHandlerCompat, /template_better_auth_readiness_body\(&request\.runtime_env\)/);
  assert.match(
    routeHandlerCompat,
    /fn template_better_auth_missing_config\(\s*runtime_env: &BTreeMap<String, String>,\s*\)/,
  );
  assert.match(routeHandlerCompat, /let credentials_configured = missing_config\.is_empty\(\);/);
  assert.match(routeHandlerCompat, /template_better_auth_package_readiness_status\(credentials_configured\)/);
  assert.match(routeHandlerCompat, /"credentialsConfigured": credentials_configured/);
  assert.match(routeHandlerCompat, /"canRunRouteHandlers": false/);
  assert.match(routeHandlerCompat, /template_better_auth_base_url\(runtime_env\)/);

  assert.match(authSessionRoute, /createTemplateBetterAuthSessionReceipt/);
  assert.match(authSessionRoute, /status:\s*200/);
  assert.match(authSessionRoute, /"cache-control": "no-store"/);
  assert.doesNotMatch(authSessionRoute, /\{ status: 501 \}/);
  assert.match(launchMaterializer, /app", "api", "auth", "session", "route\.ts"/);
  assert.match(launchMaterializer, /createTemplateBetterAuthSessionReceipt/);
  assert.match(launchMaterializer, /headers: \{ "cache-control": "no-store" \}/);
  assert.match(serverContract, /template_better_auth_session_route_handler_response/);
  assert.match(routeHandlerCompat, /source-owned-template-better-auth-session-interpreter/);
  assert.match(routeHandlerCompat, /template_better_auth_session_body\(&request\.runtime_env\)/);
  assert.match(routeHandlerCompat, /dx\.template\.authentication\.session_receipt/);
  assert.match(routeHandlerCompat, /"authenticated": false/);
  assert.match(routeHandlerCompat, /"liveSessionExecution": false/);
  assert.match(routeHandlerCompat, /x-dx-auth-session/);

  assert.match(authCatchAllRoute, /export \{ GET, POST \} from "\@\/server\/auth\/better-auth"/);
  assert.match(serverContract, /template_better_auth_route_handler_response/);
  assert.match(routeHandlerCompat, /source-owned-template-better-auth-route-handler-interpreter/);
  assert.match(routeHandlerCompat, /template_better_auth_route_handler_body/);
  assert.match(routeHandlerCompat, /template_better_auth_route_handler_body\(&request\.method,\s*&request\.runtime_env\)/);
  assert.match(routeHandlerCompat, /x-dx-auth-route-handler/);
  assert.match(
    routeHandlerCompat,
    /Configure Authentication credentials and pass an app-owned Better Auth database adapter/,
  );

  assert.match(n8nRoute, /createDxN8nRunReceipt/);
  assert.match(n8nRoute, /runtimeExecution: false/);
  assert.match(n8nReceipt, /dx\.automation\.n8n\.run_receipt/);
  assert.match(n8nRoute, /process\.env/);
  assert.match(n8nReceipt, /missingCredentials/);
  assert.match(n8nReadiness, /credentialsConfigured/);
  assert.match(n8nReadiness, /envValuePresent/);
  assert.match(n8nReadiness, /buildDxN8nCredentialReadiness/);
  assert.match(serverContract, /automation_n8n_dry_run_route_handler_response/);
  assert.match(serverContract, /route_handler_automations::automation_n8n_dry_run_route_handler_response/);
  assert.match(routeHandlerAutomations, /source-owned-automation-n8n-dry-run-interpreter/);
  assert.match(routeHandlerAutomations, /dx\.automation\.n8n\.run_receipt/);
  assert.match(routeHandlerAutomations, /automations\/n8n/);
  assert.match(routeHandlerAutomations, /x-dx-automation-n8n-dry-run/);
  assert.match(routeHandlerAutomations, /createDxN8nRunReceipt/);
  assert.match(routeHandlerAutomations, /n8n_missing_env/);
  assert.match(routeHandlerAutomations, /zed-handoff-created/);
  assert.match(routeHandlerAutomations, /credentialsConfigured/);
  assert.doesNotMatch(routeHandlerCompat, /source-owned-automation-n8n-dry-run-interpreter/);
  assert.match(stripeRoute, /createDxStripeDashboardCheckoutRequest/);
  assert.match(stripeRoute, /createDxStripeDashboardMissingConfigReceipt/);
  assert.match(stripeRoute, /dx\.payments\.stripe_js\.readiness/);
  assert.match(stripeRoute, /stripeLiveExecution: false/);
  assert.match(stripeCheckoutRoute, /createDxStripeCheckoutContactPayload/);
  assert.match(stripeCheckoutRoute, /stripeLiveExecution: false/);
  assert.match(stripeCheckoutRoute, /kind: status === 501 \? "provider-boundary" : "contact"/);
  assert.match(stripeWebhookRoute, /verifyDxStripeWebhookRequest/);
  assert.match(stripeWebhookRoute, /routeDxStripeWebhookEvent/);
  assert.match(stripeWebhookRoute, /fulfillmentStatus: "app-owned"/);
  assert.match(stripeWebhookRoute, /createDxStripeWebhookProviderBoundaryResponse/);
  assert.match(stripeWebhookRoute, /isDxStripeWebhookProviderBoundaryError/);
  assert.match(stripeWebhookRoute, /dx\.payments\.stripe_js\.webhook_boundary/);
  assert.match(stripeWebhookRoute, /providerBoundary: true/);
  assert.match(stripeWebhookRoute, /\{ status: 501 \}/);
  assert.match(stripeWebhookRoute, /\{ status: 400 \}/);
  assert.match(stripeServer, /STRIPE_WEBHOOK_SECRET is required to verify Stripe webhook events/);
  assert.match(stripeCheckout, /STRIPE_PRICE_ID_STARTER/);
  assert.match(serverContract, /payments_stripe_readiness_route_handler_response/);
  assert.match(serverContract, /payments_stripe_checkout_route_handler_response/);
  assert.match(serverContract, /payments_stripe_webhook_route_handler_response/);
  assert.match(routeHandlerPayments, /source-owned-payments-stripe-readiness-interpreter/);
  assert.match(routeHandlerPayments, /source-owned-payments-stripe-checkout-boundary-interpreter/);
  assert.match(routeHandlerPayments, /source-owned-payments-stripe-webhook-boundary-interpreter/);
  assert.match(routeHandlerPayments, /dx\.payments\.stripe_js\.readiness/);
  assert.match(routeHandlerPayments, /dx\.payments\.stripe_js\.checkout_boundary/);
  assert.match(routeHandlerPayments, /dx\.payments\.stripe_js\.webhook_boundary/);
  assert.match(routeHandlerPayments, /payments\/stripe-js/);
  assert.match(routeHandlerPayments, /x-dx-payments-stripe-readiness/);
  assert.match(routeHandlerPayments, /x-dx-payments-stripe-checkout/);
  assert.match(routeHandlerPayments, /x-dx-payments-stripe-webhook/);
  assert.match(routeHandlerPayments, /createDxStripeDashboardCheckoutRequest/);
  assert.match(routeHandlerPayments, /createDxStripeCheckoutContactPayload/);
  assert.match(routeHandlerPayments, /verifyDxStripeWebhookRequest/);
  assert.match(routeHandlerPayments, /stripe_checkout_boundary_body\(&request\.body,\s*&request\.runtime_env\)/);
  assert.match(routeHandlerPayments, /stripe_post_readiness_body\(&request\.body,\s*&request\.runtime_env\)/);
  assert.match(routeHandlerPayments, /stripe_get_readiness_body\(&request\.runtime_env\)/);
  assert.match(routeHandlerPayments, /stripe_webhook_boundary_body\(request,\s*&request\.runtime_env\)/);
  assert.match(routeHandlerPayments, /configured-source-owned-webhook-receipt/);
  assert.match(routeHandlerPayments, /webhookVerificationBoundary/);
  assert.match(routeHandlerPayments, /stripe_webhook_event_action/);
  assert.match(routeHandlerPayments, /stripe_missing_required_env/);
  assert.match(routeHandlerPayments, /provider-configured-dry-run-only/);
  assert.match(routeHandlerPayments, /dry-run-ready/);
  assert.match(routeHandlerPayments, /STRIPE_WEBHOOK_SECRET/);
  assert.match(routeHandlerPayments, /webhook delivery and fulfillment stay app-owned/);
  assert.match(routeHandlerPayments, /Payments readiness dry-run requires contact details/);
  assert.match(routeHandlerPayments, /Enter a valid checkout email/);
  assert.match(routeHandlerPayments, /Enter the checkout contact name/);

  assert.match(aiRoute, /createDxAiMissingProviderResponse/);
  assert.match(aiProviderBoundary, /runtimeExecution: false/);
  assert.match(aiProviderBoundary, /modelStreaming: false/);
  assert.match(aiTextStreamRoute, /createDxLaunchTextStreamResponse/);
  assert.match(aiTextStreamHelper, /createTextStreamResponse/);
  assert.match(aiUiStreamRoute, /createDxLaunchUIMessageStreamResponse/);
  assert.match(aiUiStreamHelper, /createUIMessageStreamResponse/);
  assert.match(serverContract, /ai_provider_boundary_route_handler_response/);
  assert.match(serverContract, /ai_local_stream_route_handler_response/);
  assert.match(routeHandlerAi, /source-owned-ai-provider-boundary-interpreter/);
  assert.match(routeHandlerAi, /source-owned-ai-local-stream-interpreter/);
  assert.match(routeHandlerAi, /dx\.ai\.provider_boundary/);
  assert.match(routeHandlerAi, /dx\.ai\.local_stream_receipt/);
  assert.match(routeHandlerAi, /ai\/vercel-ai/);
  assert.match(routeHandlerAi, /x-dx-ai-provider-boundary/);
  assert.match(routeHandlerAi, /x-dx-ai-local-stream/);
  assert.match(routeHandlerAi, /createDxAiMissingProviderResponse/);
  assert.match(routeHandlerAi, /ai_provider_boundary_response\(\s*boundary,\s*&request\.body,\s*&request\.runtime_env,\s*\)/);
  assert.match(routeHandlerAi, /provider-configured-readiness-only/);
  assert.match(routeHandlerAi, /liveProviderProof/);
  assert.match(routeHandlerAi, /AI_GATEWAY_API_KEY/);
  assert.match(routeHandlerAi, /DX_ENABLE_EXTENDED_AI_ROUTES/);
  assert.match(routeHandlerAi, /ai_provider_env_configured/);
  assert.match(routeHandlerAi, /ai_extended_route_enabled/);
  assert.match(routeHandlerAi, /extended-route-disabled/);
  assert.match(routeHandlerAi, /extended-provider-route-boundary/);
  assert.match(routeHandlerAi, /createDxLaunchTextStreamResponse/);
  assert.match(routeHandlerAi, /createDxLaunchUIMessageStreamResponse/);
  assert.match(routeHandlerAi, /ai-sdk-stream-adapter-boundary/);

  assert.match(fumadocsSearchRoute, /createDxFumadocsSearchApi/);
  assert.match(fumadocsSearchRoute, /export const GET = searchApi\.GET/);
  assert.match(fumadocsStaticSearchRoute, /export const GET = searchApi\.staticGET/);
  assert.match(fumadocsSearchHelper, /dxFumadocsSearchContract/);
  assert.match(fumadocsSearchHelper, /createFromSource\(source, dxFumadocsSearchConfig\)/);
  assert.match(fumadocsOpenApiProxyRoute, /dxFumadocsOpenAPI\.createProxy/);
  assert.match(fumadocsOpenApiProxyRoute, /readDxFumadocsOpenAPIAllowedOrigins/);
  assert.match(fumadocsOpenApiHelper, /DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS/);
  assert.match(fumadocsRouteContract, /searchRoute: "\/api\/search"/);
  assert.match(fumadocsRouteContract, /staticSearchRoute: "\/api\/search-static"/);
  assert.match(serverContract, /fumadocs_search_route_handler_response/);
  assert.match(serverContract, /fumadocs_openapi_proxy_route_handler_response/);
  assert.match(routeHandlerFumadocs, /source-owned-fumadocs-search-interpreter/);
  assert.match(routeHandlerFumadocs, /source-owned-fumadocs-openapi-proxy-boundary-interpreter/);
  assert.match(routeHandlerFumadocs, /"policyStatus": policy\.status_label/);
  assert.match(routeHandlerFumadocs, /dx\.fumadocs\.search_receipt/);
  assert.match(routeHandlerFumadocs, /dx\.fumadocs\.openapi_proxy_boundary/);
  assert.match(routeHandlerFumadocs, /x-dx-fumadocs-search/);
  assert.match(routeHandlerFumadocs, /x-dx-fumadocs-openapi-proxy/);
  assert.match(routeHandlerFumadocs, /createDxFumadocsSearchApi/);
  assert.match(routeHandlerFumadocs, /dxFumadocsOpenAPI\.createProxy/);
  assert.match(routeHandlerFumadocs, /content\/fumadocs-next/);
  assert.match(routeHandlerFumadocs, /createFromSource-adapter-boundary/);
  assert.match(routeHandlerFumadocs, /DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS/);
  assert.match(routeHandlerFumadocs, /openapi_proxy_status\(/);
  assert.match(routeHandlerFumadocs, /status: 202/);
  assert.match(routeHandlerFumadocs, /proxy-policy-accepted/);
  assert.match(routeHandlerFumadocs, /requestedOrigin/);
  assert.match(routeHandlerRuntimeEnv, /DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS/);
  for (const [routePath, capability, requiredEnv] of [
    ["/api/ai/chat", "chat-stream", "AI_PROVIDER_API_KEY"],
    ["/api/ai/agent", "agent-loop", "AI_PROVIDER_API_KEY"],
    ["/api/ai/image", "image-generation", "AI_PROVIDER_API_KEY"],
    ["/api/ai/object", "object-generation", "AI_PROVIDER_API_KEY"],
    ["/api/ai/speech", "speech-generation", "AI_PROVIDER_API_KEY"],
    ["/api/ai/transcribe", "audio-transcription", "AI_PROVIDER_API_KEY"],
    ["/api/ai/upload-file", "provider-file-upload", "AI_PROVIDER_API_KEY"],
    ["/api/ai/video", "video-generation", "AI_GATEWAY_API_KEY"],
  ]) {
    assert.match(routeHandlerAi, new RegExp(routePath.replaceAll("/", "\\/")));
    assert.match(routeHandlerAi, new RegExp(capability));
    assert.match(routeHandlerAi, new RegExp(requiredEnv));
  }
  assert.doesNotMatch(routeHandlerPayments, /\.v1/);
  assert.doesNotMatch(routeHandlerDatabaseOrm, /\.v1/);
  assert.doesNotMatch(routeHandlerFumadocs, /\.v1/);
  if (routeHandlerInstantReadiness) {
    assert.doesNotMatch(routeHandlerInstantReadiness, /\.v1/);
  }
  if (routeHandlerSupabase) {
    assert.doesNotMatch(routeHandlerSupabase, /\.v1/);
  }
  assert.doesNotMatch(routeHandlerAi, /\.v1/);
  assert.doesNotMatch(routeHandlerAutomations, /\.v1/);
  assert.doesNotMatch(routeHandlerCompat, /\.v1/);
});
