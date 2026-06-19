const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

const forgeTrpc = read("core/src/ecosystem/forge_trpc.rs");
const forgeRegistry = read("core/src/ecosystem/forge_registry.rs");
const packageGuard = read("benchmarks/launch-package-slices.test.ts");
const shellGuard = read("benchmarks/template-shell.test.ts");
const launchShell = read("examples/template/template-shell.tsx");
const packageCatalog = read("examples/template/package-catalog.ts");
const templateRouteContract = read("examples/template/template-route-contract.ts");
const launchContract = read("examples/template/trpc-launch-contract.ts");
const launchExample = read("examples/template/trpc-launch-health.tsx");
const launchServerExample = read("examples/template/trpc-server-readiness.ts");
const packageDoc = read("docs/packages/api-trpc.md");
const dashboardHelper = read("examples/dashboard/src/lib/trpcDashboardWorkflow.ts");
const dashboardComponent = read("examples/dashboard/src/components/TrpcDashboardWorkflow.tsx");
const dashboardReceipt = read(
  "examples/template/.dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json",
);
const cli = read("dx-www/src/cli/mod.rs");
const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
const dxNotes = read("DX.md");
const todo = read("TODO.md");
const changelog = read("CHANGELOG.md");

test("api/trpc slice exposes mutation and inference APIs from the upstream tRPC shape", () => {
  assert.match(forgeTrpc, /TRPC_NEXT_VERSION: &str = "11\.17\.0-dx\.10"/);
  assert.match(forgeTrpc, /inferRouterInputs/);
  assert.match(forgeTrpc, /inferRouterOutputs/);
  assert.match(forgeTrpc, /export type AppRouterInputs = inferRouterInputs<AppRouter>;/);
  assert.match(forgeTrpc, /export type AppRouterOutputs = inferRouterOutputs<AppRouter>;/);
  assert.match(forgeTrpc, /launchEvent: publicProcedure/);
  assert.match(forgeTrpc, /\.mutation\(\(\{ ctx, input \}\) =>/);
  assert.match(forgeTrpc, /TRPCClientError/);
  assert.match(forgeTrpc, /useQueryClient/);
  assert.match(forgeTrpc, /trpc\.launchEvent\.mutationOptions/);
  assert.match(forgeTrpc, /trpc\.health\.queryFilter\(\)/);
  assert.match(forgeTrpc, /"mutationOptions from @trpc\/tanstack-react-query"/);
  assert.match(forgeTrpc, /"inferRouterInputs and inferRouterOutputs from @trpc\/server"/);
  assert.match(forgeTrpc, /data-trpc-interaction="local-launch-event-mutation"/);
});

test("api/trpc launch template visibly mounts a runtime-safe workflow", () => {
  assert.match(launchShell, /import \{ TrpcLaunchHealth \} from "\.\/trpc-launch-health";/);
  assert.match(launchShell, /<TrpcLaunchHealth \/>/);
  assert.match(launchContract, /export const trpcLaunchContract = /);
  assert.match(launchContract, /export type LaunchEventResult = /);
  assert.match(launchContract, /export type HealthCheckResult = /);
  assert.match(launchContract, /export type TrpcLaunchWorkflowResult = HealthCheckResult \| LaunchEventResult;/);
  assert.match(launchContract, /export function createLocalHealthCheck/);
  assert.match(launchContract, /export function createLocalLaunchEvent/);
  assert.match(launchContract, /packageId: "api\/trpc"/);
  assert.match(launchContract, /trpc\.health\.queryOptions\(\)/);
  assert.match(launchContract, /trpc\.launchEvent\.mutationOptions\(\)/);
  assert.match(launchContract, /trpc\.health\.queryFilter\(\)/);
  assert.match(launchExample, /"use client";/);
  assert.match(launchExample, /import \* as React from "react";/);
  assert.match(
    launchExample,
    /import \{\s*createLocalHealthCheck,\s*createLocalLaunchEvent,\s*trpcLaunchContract,\s*type TrpcLaunchWorkflowResult,\s*\} from "\.\/trpc-launch-contract";/,
  );
  assert.match(launchExample, /data-dx-package="api\/trpc"/);
  assert.match(launchExample, /data-dx-component="trpc-launch-health-workflow"/);
  assert.match(launchExample, /data-dx-dashboard-card="typed-api-health"/);
  assert.match(launchExample, /data-dx-dashboard-workflow="typed-api-health"/);
  assert.match(launchExample, /data-dx-dashboard-flow="typed-api-readiness"/);
  assert.match(launchExample, /data-dx-trpc-workflow="launch-api-readiness"/);
  assert.match(launchExample, /data-dx-style-surface="theme-token"/);
  assert.match(launchExample, /<dx-icon name="api:trpc" aria-hidden="true" \/>/);
  assert.match(launchExample, /bg-card/);
  assert.match(launchExample, /text-card-foreground/);
  assert.match(launchExample, /text-muted-foreground/);
  assert.match(launchExample, /data-trpc-workflow="template-visible"/);
  assert.match(launchExample, /data-trpc-runtime-boundary="source-owned-runtime-pending"/);
  assert.match(launchExample, /data-dx-trpc-action="check-health"/);
  assert.match(launchExample, /data-trpc-interaction="health-query"/);
  assert.match(launchExample, /data-dx-trpc-action="prepare-launch-event"/);
  assert.match(launchExample, /data-trpc-interaction="local-launch-event-mutation"/);
  assert.match(launchExample, /data-trpc-node-modules="not-required-for-workflow"/);
  assert.match(launchExample, /React\.useState<TrpcLaunchWorkflowResult \| null>/);
  assert.match(launchExample, /setResult\(createLocalHealthCheck\(nextSequence\)\)/);
  assert.match(launchExample, /setResult\(createLocalLaunchEvent\(nextSequence\)\)/);
  assert.match(launchExample, /trpcLaunchContract\.sourceApis\[3\]/);
  assert.match(launchExample, /trpcLaunchContract\.sourceApis\[4\]/);
  assert.match(launchExample, /data-api-mutation=\{result\?\.status \?\? "idle"\}/);
  assert.match(launchExample, /data-dx-trpc-receipt-state=\{result\?\.status \?\? "idle"\}/);
  assert.match(launchExample, /data-dx-trpc-request-id=\{result\?\.requestId \?\? "not-run"\}/);
  assert.match(launchExample, /data-trpc-query-state=/);
  assert.match(launchExample, /data-trpc-procedure-result=\{result\?\.procedure \?\? "idle"\}/);
  assert.match(launchExample, /data-trpc-client-ready="local-contract"/);
  assert.doesNotMatch(launchExample, /const trpcLaunchContract =/);
  assert.doesNotMatch(launchExample, /type LaunchEventResult =/);
  assert.doesNotMatch(launchExample, /data-trpc-proof=/);
  assert.doesNotMatch(launchExample, /trpc-launch-health-proof/);
  assert.doesNotMatch(launchExample, /not-required-for-proof/);
  assert.doesNotMatch(launchExample, /text-sky-|bg-black|border-neutral|text-neutral|bg-neutral/);
  assert.doesNotMatch(launchExample, /from "@trpc\/client"/);
  assert.doesNotMatch(launchExample, /from "@tanstack\/react-query"/);
  assert.doesNotMatch(launchExample, /from "@\/lib\/trpc\/provider"/);
});

test("api/trpc discovery docs and guards mention the real added surface", () => {
  assert.match(forgeRegistry, /package\.version, "11\.17\.0-dx\.10"/);
  assert.match(forgeRegistry, /launch_example\.contains\(r#"data-dx-component="trpc-launch-health-workflow""#\)/);
  assert.match(forgeTrpc, /aliases: \[/);
  assert.match(forgeTrpc, /sourceMirror: "G:\/WWW\/inspirations\/trpc"/);
  assert.match(forgeTrpc, /upstreamRepo: "trpc\/trpc"/);
  assert.match(forgeTrpc, /requiredEnv: \[\]/);
  assert.match(forgeTrpc, /exportedFiles: \[/);
  assert.match(forgeTrpc, /"lib\/trpc\/dashboard-workflow\.ts"/);
  assert.match(forgeTrpc, /"components\/dashboard\/trpc-dashboard-workflow\.tsx"/);
  assert.match(forgeTrpc, /receiptPaths: \[/);
  assert.match(forgeTrpc, /dashboardWorkflowApiFile: "lib\/trpc\/dashboard-workflow\.ts"/);
  assert.match(forgeTrpc, /dashboardWorkflowComponent: "components\/dashboard\/trpc-dashboard-workflow\.tsx"/);
  assert.match(forgeTrpc, /dashboardWorkflow: \{/);
  assert.match(forgeTrpc, /launchDashboard: \{/);
  assert.match(forgeTrpc, /component: "launch-trpc-api-dashboard-workflow"/);
  assert.match(forgeTrpc, /healthAction: 'data-dx-trpc-action="check-health"'/);
  assert.match(forgeTrpc, /launchEventAction: 'data-dx-trpc-action="prepare-launch-event"'/);
  assert.match(
    forgeTrpc,
    /"js\/examples\/template\/trpc-launch-contract\.ts",\s*TRPC_LAUNCH_CONTRACT_TS/,
  );
  assert.match(forgeTrpc, /const TRPC_LAUNCH_CONTRACT_TS: &str =/);
  assert.ok(forgeTrpc.includes('icon: \'<dx-icon name="api:trpc" />\''));
  assert.match(packageGuard, /"mutationOptions"/);
  assert.match(packageGuard, /"inferRouterInputs"/);
  assert.match(cli, /"api\/trpc" => vec!\[/);
  assert.match(cli, /"createDxTrpcRouteHandler"/);
  assert.match(cli, /"createDxTrpcClient"/);
  assert.match(cli, /"createDxTrpcServerCaller"/);
  assert.match(cli, /"createLocalHealthCheck"/);
  assert.match(cli, /"createLocalLaunchEvent"/);
  assert.doesNotMatch(cli, /"createTRPCRouter", "useTRPC"/);
  assert.match(shellGuard, /data-dx-component="trpc-launch-health-workflow"/);
  assert.match(shellGuard, /data-dx-component="launch-trpc-api-dashboard-workflow"/);
  assert.match(shellGuard, /data-dx-dashboard-card="typed-api"/);
  assert.match(shellGuard, /data-dx-trpc-workflow="launch-api-readiness"/);
  assert.match(shellGuard, /data-trpc-interaction="local-launch-event-mutation"/);
  assert.match(shellGuard, /data-dx-dashboard-workflow="typed-api-health"/);
  assert.match(shellGuard, /<dx-icon name="api:trpc"/);
  assert.match(dxNotes, /mutationOptions/);
  assert.match(dxNotes, /inferRouterInputs/);
  assert.match(dxNotes, /persistence, audit logging, rate limiting, authorization/);
  assert.match(dxNotes, /data-dx-dashboard-workflow="typed-api-health"/);
  assert.match(dxNotes, /trpc-launch-contract\.ts/);
  assert.match(todo, /tRPC mutation\/inference launch upgrade: 100\/100/);
  assert.match(todo, /tRPC dashboard professionalization: 100\/100/);
  assert.match(todo, /tRPC launch workflow contract cleanup: 100\/100/);
  assert.match(changelog, /Extended `api\/trpc` with a source-owned mutation and inference slice/);
  assert.match(changelog, /Professionalized `api\/trpc` for the dashboard starter/);
  assert.match(changelog, /Professionalized the `api\/trpc` launch workflow contract/);
  assert.match(packageDoc, /TrpcDashboardWorkflow/);
  assert.match(packageDoc, /trpc-launch-contract\.ts/);
  assert.match(packageDoc, /G:\\WWW\\inspirations\\trpc/);
});

test("api/trpc dashboard receipt pins the visible launch workflow", () => {
  assert.match(dashboardReceipt, /"schema": "dx\.forge\.package_dashboard_workflow_receipt"/);
  assert.match(dashboardReceipt, /"package_id": "api\/trpc"/);
  assert.match(dashboardReceipt, /"source_mirror": "G:\/WWW\/inspirations\/trpc"/);
  assert.match(dashboardReceipt, /"component": "launch-trpc-api-dashboard-workflow"/);
  assert.match(dashboardReceipt, /"workflow": "typed-api-readiness"/);
  assert.match(dashboardReceipt, /tools\/launch\/runtime-template\/pages\/index\.html/);
  assert.match(dashboardReceipt, /data-dx-package=\\"api\/trpc\\"/);
  assert.match(dashboardReceipt, /data-dx-component=\\"trpc-backend-workflow\\"/);
  assert.match(dashboardReceipt, /data-dx-dashboard-card=\\"typed-api-health\\"/);
  assert.match(dashboardReceipt, /data-dx-trpc-action=\\"check-health\\"/);
  assert.match(dashboardReceipt, /data-dx-trpc-action=\\"prepare-launch-event\\"/);
  assert.match(dashboardReceipt, /data-dx-trpc-action=\\"open-launch-workflow\\"/);
  assert.match(dashboardReceipt, /trpc\.health\.queryOptions\(\)/);
  assert.match(dashboardReceipt, /trpc\.launchEvent\.mutationOptions\(\)/);
  assert.match(dashboardReceipt, /"local_contract_exports": \[/);
  assert.match(dashboardReceipt, /"HealthCheckResult"/);
  assert.match(dashboardReceipt, /"TrpcLaunchWorkflowResult"/);
  assert.match(dashboardReceipt, /"createLocalHealthCheck"/);
  assert.match(dashboardReceipt, /"createLocalLaunchEvent"/);
  assert.match(dashboardReceipt, /fetchRequestHandler/);
  assert.match(dashboardReceipt, /"upstream_source_guard": \{/);
  assert.match(dashboardReceipt, /packages\/server\/src\/unstable-core-do-not-import\/initTRPC\.ts/);
  assert.match(dashboardReceipt, /packages\/client\/src\/links\/httpBatchStreamLink\.ts/);
  assert.match(dashboardReceipt, /packages\/tanstack-react-query\/src\/internals\/createOptionsProxy\.ts/);
  assert.match(dashboardReceipt, /"createTRPCOptionsProxy"/);
  assert.match(dashboardReceipt, /"infiniteQueryOptions"/);
  assert.match(dashboardReceipt, /"subscriptionOptions"/);
  assert.match(dashboardReceipt, /"node_modules_required": false/);
  assert.match(dashboardReceipt, /dx run --test \.\\\\benchmarks\\\\trpc-dashboard-workflow\.test\.ts/);
  assert.match(dashboardReceipt, /dx run --test \.\\\\benchmarks\\\\trpc-launch-runtime-proof\.test\.ts/);
  assert.match(
    templateRouteContract,
    /\.dx\/forge\/receipts\/2026-05-22-api-trpc-dashboard-workflow\.json/,
  );
  assert.match(templateRouteContract, /trpcTypedApiDashboard/);
  assert.match(templateRouteContract, /packageId: "api\/trpc"/);
  assert.match(templateRouteContract, /component: "launch-trpc-api-dashboard-workflow"/);
  assert.match(templateRouteContract, /dashboardWorkflow: "typed-api-readiness"/);
  assert.match(templateRouteContract, /routeContract: "\/api\/trpc\/health"/);
  assert.match(templateRouteContract, /sourceGuard: "dx run --test \.\\\\benchmarks\\\\trpc-forge-slice\.test\.ts"/);
});

test("api/trpc front-facing surfaces use the official Type-Safe API package name", () => {
  assert.match(forgeTrpc, /officialDxPackageName: "Type-Safe API"/);
  assert.match(forgeTrpc, /packageDisplayName: "Type-Safe API"/);
  assert.match(forgeTrpc, /upstreamPackageName: "@trpc\/server"/);
  assert.match(dashboardHelper, /officialDxPackageName: 'Type-Safe API'/);
  assert.match(dashboardHelper, /packageDisplayName: 'Type-Safe API'/);
  assert.match(dashboardHelper, /upstreamPackageName: '@trpc\/server'/);
  assert.match(dashboardComponent, /<h2>Type-Safe API workflow<\/h2>/);
  assert.match(dashboardComponent, /upstream tRPC router, client, and TanStack Query boundary/);
  assert.doesNotMatch(dashboardComponent, />tRPC typed API workflow</);
  assert.match(packageCatalog, /"api\/trpc": \{\s+name: "Type-Safe API"/);
  assert.doesNotMatch(packageCatalog, /name: "tRPC Typed API Workflow"/);
  assert.match(launchContract, /officialName: "Type-Safe API"/);
  assert.match(launchContract, /upstreamPackage: "@trpc\/server"/);
  assert.match(launchExample, /<span>Type-Safe API launch health<\/span>/);
  assert.match(launchExample, /local Type-Safe API workflow action/);
  assert.match(dashboardReceipt, /"official_dx_package_name": "Type-Safe API"/);
  assert.match(dashboardReceipt, /"package_name": "Type-Safe API"/);
  assert.match(dashboardReceipt, /"upstream_package": "@trpc\/server"/);
  assert.doesNotMatch(dashboardReceipt, /"package_name": "tRPC Typed API Workflow"/);
  assert.match(packageDoc, /# Type-Safe API/);
  assert.match(packageDoc, /Official DX package: `Type-Safe API`/);
  assert.match(packageDoc, /Upstream provenance: `@trpc\/server`, `@trpc\/client`, and\s+`@trpc\/tanstack-react-query`/);
  assert.doesNotMatch(packageDoc, /^# api\/trpc/m);
  assert.match(cli, /"official_dx_package_name": "Type-Safe API"/);
  assert.match(cli, /components\/template-app\/trpc-launch-health\.tsx/);
  assert.match(studioManifest, /"front_facing_name": "Type-Safe API Dashboard"/);
  assert.doesNotMatch(studioManifest, /"front_facing_name": "tRPC Typed API Dashboard"/);
});

test("api/trpc slice exposes a server caller helper for App Router and RSC use", () => {
  assert.match(forgeTrpc, /TRPC_NEXT_VERSION: &str = "11\.17\.0-dx\.10"/);
  assert.match(forgeTrpc, /"js\/lib\/trpc\/server-caller\.ts", TRPC_SERVER_CALLER_TS/);
  assert.match(forgeTrpc, /"js\/examples\/template\/trpc-server-readiness\.ts"/);
  assert.match(forgeTrpc, /export async function createDxTrpcServerCaller/);
  assert.match(forgeTrpc, /return createCaller\(await createDxTrpcContext\(\{/);
  assert.match(forgeTrpc, /export async function readDxTrpcLaunchReadiness/);
  assert.match(forgeTrpc, /type LaunchReadinessOutput = AppRouterOutputs\["launchReadiness"\];/);
  assert.match(forgeTrpc, /await caller\.launchReadiness\(\{/);
  assert.match(forgeTrpc, /"createCallerFactory server callers for RSC\/server actions"/);
  assert.match(forgeTrpc, /"lib\/trpc\/server-caller\.ts"/);
  assert.match(forgeTrpc, /"examples\/template\/trpc-server-readiness\.ts"/);
  assert.match(forgeTrpc, /serverCaller: "createDxTrpcServerCaller\(\)"/);
  assert.match(forgeTrpc, /serverSnapshot: "readDxTrpcLaunchReadiness\(\)"/);
  assert.match(launchServerExample, /readDxTrpcLaunchReadiness/);
  assert.match(launchServerExample, /next-familiar-www-template/);
  assert.match(launchServerExample, /packageId: "api\/trpc" as const/);
  assert.match(forgeRegistry, /paths\.contains\(&"lib\/trpc\/server-caller\.ts"\)/);
  assert.match(forgeRegistry, /paths\.contains\(&"examples\/template\/trpc-server-readiness\.ts"\)/);
  assert.match(forgeRegistry, /assert_eq!\(paths\.len\(\), 28\)/);
  assert.match(packageGuard, /"createDxTrpcServerCaller"/);
  assert.match(packageGuard, /"readDxTrpcLaunchReadiness"/);
  assert.match(dxNotes, /createDxTrpcServerCaller/);
  assert.match(todo, /tRPC server-caller launch upgrade: 100\/100/);
  assert.match(changelog, /Extended `api\/trpc` with a source-owned server caller slice/);
});

test("api/trpc slice exposes the real HTTP subscription and TanStack subscription options path", () => {
  assert.match(forgeTrpc, /TRPC_NEXT_VERSION: &str = "11\.17\.0-dx\.10"/);
  assert.match(forgeTrpc, /"js\/lib\/trpc\/subscriptions\.ts", TRPC_SUBSCRIPTIONS_TS/);
  assert.match(forgeTrpc, /"js\/examples\/template\/trpc-subscription-status\.tsx"/);
  assert.match(forgeTrpc, /import \{ tracked \} from "@trpc\/server";/);
  assert.match(forgeTrpc, /launchFeed: publicProcedure/);
  assert.match(forgeTrpc, /\.subscription\(async function\* \(\{ input, signal \}\)/);
  assert.match(forgeTrpc, /yield tracked\(String\(count\),/);
  assert.match(forgeTrpc, /httpSubscriptionLink/);
  assert.match(forgeTrpc, /splitLink/);
  assert.match(forgeTrpc, /createDxTrpcSubscriptionClient/);
  assert.match(forgeTrpc, /useSubscription/);
  assert.match(forgeTrpc, /trpc\.launchFeed\.subscriptionOptions/);
  assert.match(forgeTrpc, /sourceSurface: \[/);
  assert.match(forgeTrpc, /"subscriptionOptions and useSubscription from @trpc\/tanstack-react-query"/);
  assert.match(forgeTrpc, /"httpSubscriptionLink and splitLink from @trpc\/client"/);
  assert.match(forgeTrpc, /subscriptionClient: "createDxTrpcSubscriptionClient\(\)"/);
  assert.match(forgeTrpc, /subscriptionHook: "launchFeed.subscriptionOptions"/);

  const launchSubscriptionExample = read("examples/template/trpc-subscription-status.tsx");
  assert.match(launchSubscriptionExample, /useSubscription/);
  assert.match(launchSubscriptionExample, /trpc\.launchFeed\.subscriptionOptions/);
  assert.match(launchSubscriptionExample, /data-trpc-subscription="pending"/);
  assert.match(forgeRegistry, /paths\.contains\(&"lib\/trpc\/subscriptions\.ts"\)/);
  assert.match(forgeRegistry, /paths\.contains\(&"examples\/template\/trpc-subscription-status\.tsx"\)/);
  assert.match(forgeRegistry, /assert_eq!\(paths\.len\(\), 28\)/);
  assert.match(packageGuard, /"httpSubscriptionLink"/);
  assert.match(packageGuard, /"subscriptionOptions"/);
  assert.match(packageGuard, /"createDxTrpcSubscriptionClient"/);
  assert.match(dxNotes, /httpSubscriptionLink/);
  assert.match(todo, /tRPC subscription launch upgrade: 100\/100/);
  assert.match(changelog, /Extended `api\/trpc` with a source-owned subscription slice/);
});

test("api/trpc slice exposes typed error formatting and launch error status", () => {
  const launchErrorExample = read("examples/template/trpc-error-status.tsx");

  assert.match(forgeTrpc, /TRPC_NEXT_VERSION: &str = "11\.17\.0-dx\.10"/);
  assert.match(forgeTrpc, /"js\/lib\/trpc\/errors\.ts", TRPC_ERRORS_TS/);
  assert.match(forgeTrpc, /"js\/examples\/template\/trpc-error-status\.tsx"/);
  assert.match(forgeTrpc, /type TRPCErrorFormatter/);
  assert.match(forgeTrpc, /type TRPCErrorShape/);
  assert.match(forgeTrpc, /type TRPCDefaultErrorData/);
  assert.match(forgeTrpc, /getHTTPStatusCodeFromError/);
  assert.match(forgeTrpc, /export const formatDxTrpcError: TRPCErrorFormatter/);
  assert.match(forgeTrpc, /errorFormatter: formatDxTrpcError/);
  assert.match(forgeTrpc, /export type AppRouterError = inferRouterError<AppRouter>;/);
  assert.match(forgeTrpc, /export function createDxTrpcError/);
  assert.match(forgeTrpc, /publicMessage: dxTrpcPublicErrorMessage\(error\)/);
  assert.match(forgeTrpc, /"errorFormatter from initTRPC.create\(\)"/);
  assert.match(forgeTrpc, /"getHTTPStatusCodeFromError from @trpc\/server\/http"/);
  assert.match(forgeTrpc, /errorFormatter: "formatDxTrpcError"/);
  assert.match(forgeTrpc, /errorShape: "AppRouterError"/);
  assert.match(launchErrorExample, /TRPCClientError/);
  assert.match(launchErrorExample, /type AppRouterError/);
  assert.match(launchErrorExample, /data-trpc-error-code/);
  assert.match(forgeRegistry, /paths\.contains\(&"lib\/trpc\/errors\.ts"\)/);
  assert.match(forgeRegistry, /paths\.contains\(&"examples\/template\/trpc-error-status\.tsx"\)/);
  assert.match(packageGuard, /"formatDxTrpcError"/);
  assert.match(packageGuard, /"getHTTPStatusCodeFromError"/);
  assert.match(dxNotes, /formatDxTrpcError/);
  assert.match(todo, /tRPC error-formatting launch upgrade: 100\/100/);
  assert.match(changelog, /Extended `api\/trpc` with a source-owned error-formatting slice/);
});

test("api/trpc slice exposes streaming transport and opt-in diagnostics links", () => {
  const launchStreamingExample = read("examples/template/trpc-streaming-client-status.tsx");

  assert.match(forgeTrpc, /TRPC_NEXT_VERSION: &str = "11\.17\.0-dx\.10"/);
  assert.match(forgeTrpc, /"js\/lib\/trpc\/streaming-client\.ts", TRPC_STREAMING_CLIENT_TS/);
  assert.match(forgeTrpc, /"js\/examples\/template\/trpc-streaming-client-status\.tsx"/);
  assert.match(forgeTrpc, /httpBatchStreamLink/);
  assert.match(forgeTrpc, /loggerLink/);
  assert.match(forgeTrpc, /export function createDxTrpcStreamingClient/);
  assert.match(forgeTrpc, /export function createDxTrpcLoggerLink/);
  assert.match(forgeTrpc, /streamHeader: "trpc-accept"/);
  assert.match(forgeTrpc, /transport\?: "batch" \| "stream" \| "subscription"/);
  assert.match(forgeTrpc, /transport === "stream"/);
  assert.match(forgeTrpc, /enableLogger/);
  assert.match(forgeTrpc, /"httpBatchStreamLink and loggerLink from @trpc\/client"/);
  assert.match(forgeTrpc, /streamingClient: "createDxTrpcStreamingClient\(\)"/);
  assert.match(forgeTrpc, /diagnosticsLink: "createDxTrpcLoggerLink\(\)"/);
  assert.match(launchStreamingExample, /DxTrpcProvider/);
  assert.match(launchStreamingExample, /transport="stream"/);
  assert.match(launchStreamingExample, /enableLogger/);
  assert.match(launchStreamingExample, /data-trpc-transport="httpBatchStreamLink"/);
  assert.match(forgeRegistry, /paths\.contains\(&"lib\/trpc\/streaming-client\.ts"\)/);
  assert.match(forgeRegistry, /paths\.contains\(&"examples\/template\/trpc-streaming-client-status\.tsx"\)/);
  assert.match(forgeRegistry, /assert_eq!\(paths\.len\(\), 28\)/);
  assert.match(packageGuard, /"httpBatchStreamLink"/);
  assert.match(packageGuard, /"loggerLink"/);
  assert.match(dxNotes, /createDxTrpcStreamingClient/);
  assert.match(todo, /tRPC streaming-client launch upgrade: 100\/100/);
  assert.match(changelog, /Extended `api\/trpc` with a source-owned streaming client slice/);
});

test("api/trpc slice exposes response metadata and cache policy boundaries", () => {
  const launchResponseMetaExample = read("examples/template/trpc-response-meta.ts");

  assert.match(forgeTrpc, /TRPC_NEXT_VERSION: &str = "11\.17\.0-dx\.10"/);
  assert.match(forgeTrpc, /"js\/lib\/trpc\/response-meta\.ts", TRPC_RESPONSE_META_TS/);
  assert.match(forgeTrpc, /"js\/examples\/template\/trpc-response-meta\.ts"/);
  assert.match(forgeTrpc, /ResponseMeta,/);
  assert.match(forgeTrpc, /ResponseMetaFn,/);
  assert.match(forgeTrpc, /export function createDxTrpcResponseMeta/);
  assert.match(forgeTrpc, /export function dxTrpcPublicCacheResponseMeta/);
  assert.match(forgeTrpc, /export function dxTrpcNoStoreResponseMeta/);
  assert.match(forgeTrpc, /responseMeta: options\.responseMeta \?\? createDxTrpcResponseMeta\(\)/);
  assert.match(forgeTrpc, /Next\.js App Router may override Cache-Control/);
  assert.match(forgeTrpc, /"responseMeta, ResponseMeta, and ResponseMetaFn from @trpc\/server\/http"/);
  assert.match(forgeTrpc, /responseMeta: "createDxTrpcResponseMeta\(\)"/);
  assert.match(launchResponseMetaExample, /createDxTrpcResponseMeta/);
  assert.match(launchResponseMetaExample, /createDxTrpcRouteHandler/);
  assert.match(launchResponseMetaExample, /publicPathPrefix: "health"/);
  assert.match(launchResponseMetaExample, /trpcResponseMetaReadiness/);
  assert.match(forgeRegistry, /paths\.contains\(&"lib\/trpc\/response-meta\.ts"\)/);
  assert.match(forgeRegistry, /paths\.contains\(&"examples\/template\/trpc-response-meta\.ts"\)/);
  assert.match(forgeRegistry, /assert_eq!\(paths\.len\(\), 28\)/);
  assert.match(packageGuard, /"createDxTrpcResponseMeta"/);
  assert.match(packageGuard, /"ResponseMetaFn"/);
  assert.match(dxNotes, /createDxTrpcResponseMeta/);
  assert.match(todo, /tRPC response-meta launch upgrade: 100\/100/);
  assert.match(changelog, /Extended `api\/trpc` with a source-owned response metadata slice/);
});

test("api/trpc slice exposes infinite query pagination for launch feeds", () => {
  const launchInfiniteFeedExample = read("examples/template/trpc-infinite-feed.tsx");

  assert.match(forgeTrpc, /TRPC_NEXT_VERSION: &str = "11\.17\.0-dx\.10"/);
  assert.match(forgeTrpc, /"js\/examples\/template\/trpc-infinite-feed\.tsx"/);
  assert.match(forgeTrpc, /launchEvents: publicProcedure/);
  assert.match(forgeTrpc, /cursor: z\.number\(\)\.int\(\)\.min\(0\)\.default\(0\)/);
  assert.match(forgeTrpc, /nextCursor/);
  assert.match(forgeTrpc, /export type LaunchEventsInput = AppRouterInputs\["launchEvents"\];/);
  assert.match(forgeTrpc, /export type LaunchEventsOutput = AppRouterOutputs\["launchEvents"\];/);
  assert.match(forgeTrpc, /"infiniteQueryOptions, infiniteQueryKey, and infiniteQueryFilter from @trpc\/tanstack-react-query"/);
  assert.match(forgeTrpc, /infiniteQuery: "launchEvents.infiniteQueryOptions"/);
  assert.match(launchInfiniteFeedExample, /useInfiniteQuery/);
  assert.match(launchInfiniteFeedExample, /trpc\.launchEvents\.infiniteQueryOptions/);
  assert.match(launchInfiniteFeedExample, /getNextPageParam/);
  assert.match(launchInfiniteFeedExample, /trpc\.launchEvents\.infiniteQueryKey/);
  assert.match(launchInfiniteFeedExample, /trpc\.launchEvents\.infiniteQueryFilter/);
  assert.match(launchInfiniteFeedExample, /data-trpc-infinite-feed="ready"/);
  assert.match(forgeRegistry, /paths\.contains\(&"examples\/template\/trpc-infinite-feed\.tsx"\)/);
  assert.match(forgeRegistry, /assert_eq!\(paths\.len\(\), 28\)/);
  assert.match(packageGuard, /"infiniteQueryOptions"/);
  assert.match(packageGuard, /"infiniteQueryKey"/);
  assert.match(dxNotes, /infiniteQueryOptions/);
  assert.match(todo, /tRPC infinite-query launch upgrade: 100\/100/);
  assert.match(changelog, /Extended `api\/trpc` with a source-owned infinite query pagination slice/);
});

test("api/trpc slice exposes a source-owned transformer boundary", () => {
  const launchTransformerExample = read("examples/template/trpc-transformer-status.ts");

  assert.match(forgeTrpc, /TRPC_NEXT_VERSION: &str = "11\.17\.0-dx\.10"/);
  assert.match(forgeTrpc, /"js\/lib\/trpc\/transformer\.ts", TRPC_TRANSFORMER_TS/);
  assert.match(forgeTrpc, /"js\/examples\/template\/trpc-transformer-status\.ts"/);
  assert.match(forgeTrpc, /TRPCCombinedDataTransformer/);
  assert.match(forgeTrpc, /TRPCDataTransformer/);
  assert.match(forgeTrpc, /export function createDxTrpcTransformer/);
  assert.match(forgeTrpc, /export const dxTrpcIdentityDataTransformer/);
  assert.match(forgeTrpc, /export const dxTrpcTransformer = createDxTrpcTransformer\(\)/);
  assert.match(forgeTrpc, /transformer: dxTrpcTransformer/);
  assert.match(forgeTrpc, /transformer: options\.transformer \?\? dxTrpcTransformer/);
  assert.match(forgeTrpc, /transformer: options\.transformer \?\? dxTrpcTransformer,\n\s+}\),/);
  assert.match(forgeTrpc, /"TRPCCombinedDataTransformer and TRPCDataTransformer from @trpc\/server"/);
  assert.match(forgeTrpc, /transformer: "dxTrpcTransformer"/);
  assert.match(launchTransformerExample, /createDxTrpcTransformer/);
  assert.match(launchTransformerExample, /dxTrpcIdentityDataTransformer/);
  assert.match(launchTransformerExample, /trpcTransformerReadiness/);
  assert.match(launchTransformerExample, /appOwned/);
  assert.match(forgeRegistry, /paths\.contains\(&"lib\/trpc\/transformer\.ts"\)/);
  assert.match(forgeRegistry, /paths\.contains\(&"examples\/template\/trpc-transformer-status\.ts"\)/);
  assert.match(forgeRegistry, /assert_eq!\(paths\.len\(\), 28\)/);
  assert.match(packageGuard, /"dxTrpcTransformer"/);
  assert.match(packageGuard, /"TRPCCombinedDataTransformer"/);
  assert.match(dxNotes, /dxTrpcTransformer/);
  assert.match(todo, /tRPC transformer launch upgrade: 100\/100/);
  assert.match(changelog, /Extended `api\/trpc` with a source-owned transformer boundary/);
});

test("api/trpc slice exposes request headers and batch policy link options", () => {
  const launchRequestPolicyExample = read("examples/template/trpc-request-policy.ts");

  assert.match(forgeTrpc, /TRPC_NEXT_VERSION: &str = "11\.17\.0-dx\.10"/);
  assert.match(forgeTrpc, /"js\/lib\/trpc\/http\.ts", TRPC_HTTP_TS/);
  assert.match(forgeTrpc, /"js\/examples\/template\/trpc-request-policy\.ts"/);
  assert.match(forgeTrpc, /HTTPBatchLinkOptions/);
  assert.match(forgeTrpc, /HTTPHeaders/);
  assert.match(forgeTrpc, /TRPCFetch/);
  assert.match(forgeTrpc, /export function createDxTrpcRequestHeaders/);
  assert.match(forgeTrpc, /export function createDxTrpcHttpLinkOptions/);
  assert.match(forgeTrpc, /maxItems: options\.maxItems \?\? dxTrpcHttpBatchPolicy\.maxItems/);
  assert.match(forgeTrpc, /maxURLLength: options\.maxURLLength \?\? dxTrpcHttpBatchPolicy\.maxURLLength/);
  assert.match(forgeTrpc, /methodOverride: options\.methodOverride/);
  assert.match(forgeTrpc, /\.\.\.createDxTrpcHttpLinkOptions\(options\)/);
  assert.match(forgeTrpc, /"HTTPBatchLinkOptions, HTTPHeaders, TRPCFetch, maxItems, maxURLLength, methodOverride, and headers from @trpc\/client"/);
  assert.match(forgeTrpc, /requestPolicy: "createDxTrpcHttpLinkOptions\(\)"/);
  assert.match(launchRequestPolicyExample, /createDxTrpcRequestHeaders/);
  assert.match(launchRequestPolicyExample, /createDxTrpcHttpLinkOptions/);
  assert.match(launchRequestPolicyExample, /x-dx-template-id/);
  assert.match(launchRequestPolicyExample, /trpcRequestPolicyReadiness/);
  assert.match(forgeRegistry, /paths\.contains\(&"lib\/trpc\/http\.ts"\)/);
  assert.match(forgeRegistry, /paths\.contains\(&"examples\/template\/trpc-request-policy\.ts"\)/);
  assert.match(forgeRegistry, /assert_eq!\(paths\.len\(\), 28\)/);
  assert.match(packageGuard, /"createDxTrpcHttpLinkOptions"/);
  assert.match(packageGuard, /"HTTPBatchLinkOptions"/);
  assert.match(dxNotes, /createDxTrpcHttpLinkOptions/);
  assert.match(todo, /tRPC request-policy launch upgrade: 100\/100/);
  assert.match(changelog, /Extended `api\/trpc` with a source-owned request policy slice/);
});
