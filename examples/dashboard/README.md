
# Dashboard Example

A dashboard application demonstrating authentication, guards, data fetching, and visualization with dx-www.

## Features

- User authentication with Ed25519 tokens
- Authentication account workflow with session/profile/provider readiness receipts
- Protected routes with guards
- Role-based access control
- Session management
- Data fetching patterns with loading/error states
- Data Fetching & Cache dashboard workflow with cache profile and refresh readiness receipts
- Zod settings validation with safeParse and flattened issue receipts
- Zustand-compatible workspace settings state with selector subscriptions and persisted browser storage
- Documentation System workflow with route, page-tree, OpenAPI, search, and LLM handoff readiness
- AI SDK provider readiness with a safe local prompt receipt
- UI Components dashboard controls with source-owned settings receipt preview
- Motion & Animation workflow with stage controls, reorder preview, reduced-motion policy preview, and a source-owned animation receipt
- WebAssembly Bridge workflow with local WebAssembly execution and generated-module readiness boundaries
- Stripe checkout workflow with plan selection, hosted/embedded mode, and safe missing-config receipt
- Drizzle dashboard workflow readiness with package export and SQLite runtime boundaries
- Type-Safe API workflow with procedure readiness and a safe local receipt
- Realtime App Database dashboard workflow with realtime, presence, auth, storage, stream, Sync Table, and route readiness boundaries
- n8n automation workflow with connector readiness and redacted local receipt drafting
- Simple chart visualization component

## Running

```bash
cd examples/dashboard dx dev ```
Open //localhost:3000 to see the app.


## Test Credentials


- Admin: admin@example.com / admin123
- User: user@example.com / user123


## Building


```bash
dx build ```

## Key Patterns Demonstrated

### Data Fetching

The Dashboard page demonstrates the recommended data fetching pattern:
```tsx
useEffect(() => { let cancelled = false;
async function loadData() { try { setIsLoading(true);
const data = await fetchFromApi();
if (!cancelled) setData(data);
} catch (err) { if (!cancelled) setError('Failed to load');
} finally { if (!cancelled) setIsLoading(false);
}
}
loadData();
return () => { cancelled = true; };
}, []);
```

### Protected Routes

Routes can require authentication and specific roles:
```tsx
<ProtectedRoute path="/admin" component={Admin} requiredRole="admin" /> ```


### Chart Visualization


Simple bar chart component showing how to build visualizations:
```tsx
<BarChart data={chartData} height={180} /> ```

### AI Provider Boundary

The dashboard imports `AiLaunchAssistant` as a real AI SDK workflow surface for
`ai/vercel-ai`. It exposes provider readiness, required environment variables,
AI SDK public API names, a prompt field, and a safe local missing-config receipt.
It does not stream model output until the app owns provider credentials.

### Authentication account workflow

The dashboard imports `BetterAuthAccountWorkflow` as a real workflow surface for
`auth/better-auth`. It consumes the source-owned dashboard helper under
`src/lib/forge/auth/better-auth/dashboard.ts`, shows the local session boundary,
lets the user choose session/profile/provider/session-revocation actions, and
prepares an honest app-owned receipt for Authentication surfaces backed by
upstream better-auth APIs: `useSession`,
`auth.api.getSession({ headers })`, `authClient.updateUser`,
`authClient.changeEmail`, `authClient.signIn.social`,
`authClient.linkSocial`, and `authClient.revokeOtherSessions` APIs. Production
credentials, routes, database adapters, OAuth callbacks, email verification,
account linking, token storage, and session policy remain app-owned. The launch
companion also exposes `data-dx-auth-network-state` so missing config produces a
safe local auth preview instead of sending credentials to an unconfigured route.
The matching `/launch` workflow receipt lives at
`examples/template/.dx/forge/receipts/auth-better-auth.json`.

### UI Components dashboard controls

The dashboard imports `ShadcnDashboardControls` as a real workflow surface for
the official **UI Components** package. The selected surfaces remain
`shadcn/ui/button`, `shadcn/ui/badge`, `shadcn/ui/card`, `shadcn/ui/label`,
`shadcn/ui/field`, `shadcn/ui/item`, `shadcn/ui/input`,
`shadcn/ui/textarea`, and `shadcn/ui/separator` as provenance-backed Forge slice ids. It uses the upstream v4 `data-slot`,
`data-variant`, and `data-size` contracts, exposes DX icon metadata, previews a
local settings receipt, and keeps persistence, final copy, accessibility review,
and full registry sync app-owned.

### Documentation System Workflow

The dashboard imports `FumadocsDocsWorkflow` as a visible workflow surface for
`content/fumadocs-next`. It exposes the Documentation System route contract,
Fumadocs-derived navigation APIs, OpenAPI proxy env boundary, LLM export route,
and a safe local route receipt without creating template-local `node_modules`.

### Motion & Animation workflow

The dashboard imports `MotionDashboardWorkflow` as a visible workflow surface for
`animation/motion` and mirrors the source-owned `js/motion/dashboard-workflow.ts`
contract. It exposes MotionConfig, MotionValue, scroll progress, LayoutGroup,
Reorder, and AnimatePresence readiness, lets the operator switch animation
stages, reverse the preview order, preview reduced-motion policy, and prepare a
local motion policy receipt.
The panel uses `<dx-icon name="pack:motion" />`, theme-token classes, and keeps
route choreography, persistence, gestures, and reduced-motion review app-owned.
The `/launch` template consumes the same package as a dashboard choreography
workflow with `data-dx-component="launch-motion-dashboard-workflow"` and a
runtime summary marker plus `#mission-motion-policy` for Web Preview selection.

### WebAssembly Bridge workflow

The dashboard imports `WasmBindgenWorkflow` as a visible workflow surface for
`wasm/bindgen`. It exposes the official package name `WebAssembly Bridge`,
upstream package provenance `wasm-bindgen`, source mirror, exported Forge files,
receipt paths, app-owned generated glue boundary, and a safe local
`WebAssembly.instantiate()` add action. The panel uses
`<dx-icon name="pack:wasm-bindgen" />`, stable `data-dx-wasm-*` markers, and
keeps Rust crate ownership, wasm-bindgen CLI output, CSP/MIME policy, and live
generated module imports app-owned. The matching `/launch` dashboard workflow
receipt lives at
`examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json`.

### Stripe checkout workflow

The dashboard imports `StripePlanCheckout` as a real workflow surface for
`payments/stripe-js`. It exposes required env readiness, upstream Stripe.js API
names, a plan picker, hosted/embedded Checkout mode selection, validated contact
inputs, and `data-dx-component="dashboard-stripe-plan-checkout"` for dx-check.
Its helper exports the same `createDxStripeDashboardCheckoutRequest({ planId,
checkoutMode, contact })` and `createDxStripeDashboardMissingConfigReceipt`
boundary names used by the Forge slice, including an app-owned
`source: "dx-www-dashboard"` request body.
Without Stripe credentials and a plan-specific or fallback Price ID it writes
only a safe local missing-config receipt; it never collects card fields or
claims a live payment.

### Drizzle dashboard workflow

The dashboard imports `DrizzleDashboardWorkflow` as a real workflow surface for
`db/drizzle-sqlite`. It exposes the source-owned `readDrizzleDashboardOverview`,
`readDrizzleDashboardQueryPlan`, and `readDrizzleDashboardQueryPlanById` entry
points, query helpers, safe `.toSQL()` SQL preview, source mirror, exported files,
receipt paths, and a safe local
action that prepares the SQLite runtime receipt without claiming a database read
before `drizzle-orm`, `better-sqlite3`, migrations, and the app's database path
are configured.

### Data Fetching & Cache dashboard workflow

The dashboard imports `QueryDashboardWorkflow` as a real workflow surface for
`tanstack/query`. It exposes QueryClient cache profiles, the
`setQueryDefaults`, `getQueryDefaults`, and `invalidateQueries` API boundary,
DX icon markers, and a safe local refresh receipt while keeping query keys,
fetchers, runtime package installation, and production cache policy app-owned.

### Type-Safe API workflow

The dashboard imports `TrpcDashboardWorkflow` as a real workflow surface for
the official `Type-Safe API` package (`api/trpc`). It exposes upstream tRPC 11
public APIs including
`initTRPC.context().create()`, `fetchRequestHandler`, `createTRPCClient`,
`httpBatchLink`, `queryOptions`, `mutationOptions`, `infiniteQueryOptions`, and
`subscriptionOptions`, lets the operator choose a dashboard procedure, and
prepares a local receipt without claiming a live route call. Applications still
own domain routers, authorization, session context, runtime dependency
installation, transport policy, cache policy, stream fan-out, and production
observability.

### Realtime App Database dashboard workflow

The dashboard imports `InstantDbDashboardWorkflow` as a real workflow surface for
Realtime App Database. It records `instantdb/react` as package provenance and
exposes upstream-shaped `init`, `i.schema`, `db.useQuery`, `db.transact`,
`db.tx`, room presence, typing indicators, `db.auth`, `db.storage`,
`db.streams`, `SyncTableCallbackEventType`, `db.core._syncTableExperimental`,
and `createInstantRouteHandler` readiness. Without
`NEXT_PUBLIC_INSTANT_APP_ID` it only prepares a safe local receipt and keeps the
Instant app, rules, auth policy, storage rules, stream lifecycle, topic payloads,
experimental Sync Table subscriptions, local store retention, and route mounting
app-owned.

### n8n automation workflow

The dashboard imports `AutomationWorkflowPanel` as a real workflow surface for
`automations/n8n`. It uses the upstream-shaped connector and credential metadata
from `G:\\WWW\\inspirations\\n8n\\packages\\nodes-base`, lets the user choose a
connector, shows the source file and credential boundary, and prepares a
redacted local receipt. Live execution remains blocked until the app owns
credentials, OAuth callbacks, operator approval, and receipt retention.

### Zod settings validation

The settings route imports `ZodSettingsValidator` as the visible workflow for
`validation/zod`. It consumes the source-owned dashboard settings API under
`src/lib/forge/validation/zod/dashboard-settings.ts`, imports real Zod v4, uses
the upstream `safeParse`, `strictObject`, metadata, and `flattenError` result
contracts, previews local validation issues and receipts, and keeps account
persistence, authorization, and full arbitrary schema composition app-owned. The
visible panel also exposes the `z.flattenError` field-error map and parsed
settings JSON so the package behavior is inspectable in the browser.

### Zustand settings state

The settings route imports `ZustandSettingsPanel` as the visible workflow for
`state/zustand`. It uses the source-owned `createStore`,
`subscribeWithSelector`, `createJSONStorage`, `persist`, and `shallow` API slice
under `src/lib/forge/state/zustand.ts` to manage dashboard density, sidebar
pinning, command hints, safe local save timestamps, and the
`dx-dashboard-settings` persistence key without creating `node_modules`. The
visible panel also subscribes to `onHydrate` and `onFinishHydration` so DX
Studio can inspect hydration event/count markers from the starter dashboard.

## Project Structure

@tree:dashboard[]
