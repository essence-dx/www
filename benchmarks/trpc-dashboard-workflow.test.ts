const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sourceMirrorRoot = "G:\\WWW\\inspirations\\trpc";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readSource(relativePath) {
  return fs.readFileSync(path.join(sourceMirrorRoot, relativePath), "utf8");
}

test("api/trpc upstream mirror exposes the public APIs the slice claims", () => {
  const initTrpc = readSource(
    "packages/server/src/unstable-core-do-not-import/initTRPC.ts",
  );
  const fetchAdapter = readSource(
    "packages/server/src/adapters/fetch/fetchRequestHandler.ts",
  );
  const client = readSource("packages/client/src/createTRPCClient.ts");
  const batchLink = readSource("packages/client/src/links/httpBatchLink.ts");
  const batchStreamLink = readSource(
    "packages/client/src/links/httpBatchStreamLink.ts",
  );
  const subscriptionLink = readSource(
    "packages/client/src/links/httpSubscriptionLink.ts",
  );
  const splitLink = readSource("packages/client/src/links/splitLink.ts");
  const optionsProxy = readSource(
    "packages/tanstack-react-query/src/internals/createOptionsProxy.ts",
  );

  assert.match(initTrpc, /context<TNewContext extends object \| ContextCallback>\(\)/);
  assert.match(initTrpc, /procedure: createBuilder/);
  assert.match(initTrpc, /router: createRouterFactory/);
  assert.match(initTrpc, /createCallerFactory: createCallerFactory/);
  assert.match(initTrpc, /export const initTRPC = new TRPCBuilder\(\);/);

  assert.match(fetchAdapter, /export async function fetchRequestHandler/);
  assert.match(fetchAdapter, /const createContext/);
  assert.match(fetchAdapter, /const endpoint = trimSlashes/);
  assert.match(fetchAdapter, /return await resolveResponse/);

  assert.match(client, /export type TRPCClient<TRouter extends AnyRouter>/);
  assert.match(client, /export function createTRPCClient/);
  assert.match(client, /createTRPCClientProxy/);
  assert.match(batchLink, /export function httpBatchLink/);
  assert.match(batchStreamLink, /export function httpBatchStreamLink/);
  assert.match(subscriptionLink, /export function httpSubscriptionLink/);
  assert.match(splitLink, /export function splitLink/);

  assert.match(optionsProxy, /export function createTRPCOptionsProxy/);
  assert.match(optionsProxy, /queryOptions: \(\) =>/);
  assert.match(optionsProxy, /infiniteQueryOptions: \(\) =>/);
  assert.match(optionsProxy, /mutationOptions: \(\) =>/);
  assert.match(optionsProxy, /subscriptionOptions: \(\) =>/);
});

test("api/trpc dashboard workflow is a visible starter surface", () => {
  const component = read("examples/dashboard/src/components/TrpcDashboardWorkflow.tsx");
  const dashboard = read("examples/dashboard/src/pages/Dashboard.tsx");

  assert.match(component, /export function TrpcDashboardWorkflow\(\)/);
  assert.match(component, /data-dx-package="api\/trpc"/);
  assert.match(component, /data-dx-component="dashboard-trpc-workflow"/);
  assert.match(component, /data-dx-dashboard-workflow="typed-api-boundary"/);
  assert.match(component, /data-dx-style-surface="theme-token"/);
  assert.match(component, /data-dx-node-modules="forbidden"/);
  assert.match(component, /<dx-icon name="api:trpc"/);
  assert.match(component, /data-dx-trpc-action="select-procedure"/);
  assert.match(component, /data-dx-trpc-action="prepare-local-receipt"/);
  assert.match(component, /data-dx-trpc-receipt-state/);
  assert.match(component, /data-dx-trpc-source-mirror/);
  assert.doesNotMatch(component, /local proof|proof card/i);
  assert.doesNotMatch(
    component,
    /#(?:[0-9a-f]{3}){1,2}|text-sky-|bg-black|border-neutral|text-neutral|bg-neutral/i,
  );

  assert.match(dashboard, /import \{ TrpcDashboardWorkflow \} from '\.\.\/components\/TrpcDashboardWorkflow';/);
  assert.match(dashboard, /<TrpcDashboardWorkflow \/>/);
});

test("api/trpc dashboard helper exposes real upstream-shaped public APIs", () => {
  const helper = read("examples/dashboard/src/lib/trpcDashboardWorkflow.ts");
  const forge = read("core/src/ecosystem/forge_trpc.rs");

  assert.match(helper, /packageId: 'api\/trpc'/);
  assert.match(helper, /aliases: \[/);
  assert.match(helper, /sourceMirror: 'G:\/WWW\/inspirations\/trpc'/);
  assert.match(helper, /provenance: \{/);
  assert.match(helper, /upstreamRepo: 'trpc\/trpc'/);
  assert.match(helper, /requiredEnv: \[\]/);
  assert.match(helper, /exportedFiles: \[/);
  assert.match(helper, /receiptPaths: \[/);
  assert.match(helper, /\.dx\/forge\/receipts\/2026-05-22-api-trpc-dashboard-workflow\.json/);
  assert.match(helper, /appOwnedBoundaries: \[/);
  assert.match(helper, /initTRPC\.context\(\)\.create\(\)/);
  assert.match(helper, /fetchRequestHandler/);
  assert.match(helper, /packages\/server\/src\/unstable-core-do-not-import\/http\/resolveResponse\.ts/);
  assert.match(helper, /createTRPCClient/);
  assert.match(helper, /httpBatchLink/);
  assert.match(helper, /packages\/client\/src\/links\/httpBatchStreamLink\.ts/);
  assert.match(helper, /packages\/client\/src\/links\/httpSubscriptionLink\.ts/);
  assert.match(helper, /packages\/client\/src\/links\/splitLink\.ts/);
  assert.match(helper, /queryOptions/);
  assert.match(helper, /mutationOptions/);
  assert.match(helper, /infiniteQueryOptions/);
  assert.match(helper, /subscriptionOptions/);
  assert.match(helper, /createTrpcDashboardReceipt/);
  assert.match(helper, /status: 'local-receipt'/);

  assert.match(forge, /"js\/lib\/trpc\/dashboard-workflow\.ts"[\s\S]*TRPC_DASHBOARD_WORKFLOW_TS/);
  assert.match(forge, /"js\/components\/dashboard\/trpc-dashboard-workflow\.tsx"[\s\S]*TRPC_DASHBOARD_WORKFLOW_TSX/);
  assert.match(forge, /export const trpcDashboardPackage = \{/);
  assert.match(forge, /packages\/server\/src\/unstable-core-do-not-import\/http\/resolveResponse\.ts/);
  assert.match(forge, /packages\/client\/src\/links\/httpBatchStreamLink\.ts/);
  assert.match(forge, /packages\/client\/src\/links\/httpSubscriptionLink\.ts/);
  assert.match(forge, /packages\/client\/src\/links\/splitLink\.ts/);
  assert.match(forge, /export default TrpcDashboardWorkflow;/);
  assert.match(forge, /"components\/dashboard\/trpc-dashboard-workflow\.tsx"/);
  assert.match(forge, /dashboardWorkflowApiFile: "lib\/trpc\/dashboard-workflow\.ts"/);
  assert.match(forge, /dashboardWorkflowComponent: "components\/dashboard\/trpc-dashboard-workflow\.tsx"/);
});

test("api/trpc dashboard workflow is documented for handoff", () => {
  const readme = read("examples/dashboard/README.md");
  const packageDoc = read("docs/packages/api-trpc.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");
  const dx = read("DX.md");

  assert.match(readme, /Type-Safe API workflow/);
  assert.match(readme, /TrpcDashboardWorkflow/);
  assert.match(readme, /api\/trpc/);
  assert.match(todo, /tRPC starter dashboard workflow: 100\/100/);
  assert.match(changelog, /Added `TrpcDashboardWorkflow` to the dashboard starter/);
  assert.match(dx, /TrpcDashboardWorkflow/);
  assert.match(dx, /data-dx-dashboard-workflow="typed-api-boundary"/);
  assert.match(packageDoc, /# Type-Safe API/);
  assert.match(packageDoc, /Official DX package: `Type-Safe API`/);
  assert.match(packageDoc, /TrpcDashboardWorkflow/);
  assert.match(packageDoc, /G:\\WWW\\inspirations\\trpc/);
  assert.match(packageDoc, /node_modules/);
  assert.match(packageDoc, /App-owned boundaries/);
  assert.doesNotMatch(
    packageDoc,
    /trpc-runtime-proof|trpc-backend-proof|id="trpc-action"|id="trpc-status"/,
  );
});
