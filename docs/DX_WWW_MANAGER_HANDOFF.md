# DX-WWW Manager Handoff

This is the current handoff for Manager AI, Friday, and future DX launch workers.

## Current 30-Agent Worker Checkpoint

- Date: 2026-05-24.
- Lane status at that checkpoint: overall DX-WWW score was 84/100, not release-ready.
- Green proof at that checkpoint: `git diff --check` with LF/CRLF warnings only,
  conflict-marker scan, `cargo check -p dx-www --no-default-features --features
  cli --bin dx-www -j 1`, `cargo build -p dx-www --no-default-features
  --features cli --bin dx-www -j 1`, real `dx build` in
  `examples/template`, `dx dev --host 127.0.0.1 --port 3000`, HTTP `/`,
  HTTP `/_dx/hot-reload/version?resource=route%3A%2F`, HTTP `/favicon.svg`,
  and 245/245 focused checks.
- Hard caps still open at that checkpoint: 568 dirty entries, 62 Rust warnings, source-guard-heavy
  evidence, missing browser screenshot and overlay recovery proof, generated
  artifact curation, and no post-curation full `cargo test` / `cargo clippy`.
- Final-polish workers have changed the worktree after that checkpoint. Treat
  this section as historical evidence, not a current release score; rerun the
  final integration proof before promoting DX-WWW to 100/100.

## Recent TSX Runtime Work

- TSX App Router dev responses now preserve dynamic route params and search params after request/query normalization.
- Generic route responses expose `data-dx-route-params`, `data-dx-search-params`, and `__DX_APP_ROUTER_EXECUTION__`.
- `app/api/**/route.ts` lookup supports dynamic App Router API segments such as `app/api/trpc/[trpc]/route.ts`.
- Exported const handlers such as `export const POST = async (request) => ...` are detected and interpreted by the safe route runtime.
- Factory/helper route exports now return a precise `501 route-handler-boundary` response instead of pretending the method is missing.
- App Router API and page-route matching are split out of the giant CLI into `app_api_routes.rs` and `app_page_routes.rs`.

Commit: `30d8778b Improve TSX app router route runtime`.

## Remaining Framework Unlock

The biggest framework unlock is still full generic TSX/App Router rendering: layout/page composition, imports, props, common hooks, client islands, and route handlers as first-class behavior. Until that is real, DX-WWW is promising but not production-complete as a DX-owned WWW framework.

## High-Leverage Ideas

1. `dx explain /route`: explain delivery mode, route files, packages, assets, and blockers.
2. AI-readable route contracts: write `.dx/routes/*.json` so Zed and AI can map route UI back to source.
3. Precise Forge maturity labels: every package reports `full`, `slice`, `adapter-boundary`, or `planned`.
4. Dependency-surface view: `dx check packages` reports source-owned files, receipts, env gates, and `node_modules` risk.
5. One-command launch doctor: `dx doctor` checks TSX routes, dx-style, imports, packages, static output, web-perf receipts, and Studio manifest readiness.
6. Visible auto-imports: keep generated import maps readable in `components/auto-imports.ts` and `.dx/imports/import-map.json`.
7. Route-level edit maps: every route contract lists stable `data-dx-*` markers and safe edit operations for Zed Web Preview.
8. dx-style quick ergonomics: make common UI classes feel immediate while keeping generated CSS and token governance.
9. Production-shaped starter: default app should be dashboard/auth/settings/billing/docs with precise missing-env states.
10. Static export speed flex: `dx export --analyze` reports route count, JS bytes, asset bytes, packages, and deploy output readiness.

## Implemented In This Pass

- `dx explain /route [--json] [--all]` writes `.dx/routes/*.json`.
- `dx doctor [--json]` writes `.dx/receipts/doctor/report.json`.
- `dx check packages [--json]` writes `.dx/receipts/check/packages.json`.
- `dx export --analyze [--json]` writes `.dx/receipts/export/analyze.json`.

These surfaces are source-only and do not start a server, install packages, deploy, or claim runtime proof.

## New Risk Guard

`dx doctor` now includes `framework_risks` with schema `dx.framework.riskRegister`. This prevents the tool from reporting complete readiness while known framework risks remain, including TSX runtime gaps, the oversized CLI module, Forge package overclaim risk, static export proof, missing web-performance receipts, Studio manifest gaps, public-story crowding, and source-copying/legal risk.

The point is not to talk the framework down. The point is to make DX-WWW more trustworthy than current frameworks by making the sharp edges visible, actionable, and machine-readable.

## TSX Semantics Step

Generic App Router responses now include a source-owned `tsx_semantics` payload with schema `dx.tsx.appRouterSemantics`. It records layout/template/page composition, default exports, children slots, imports, source-owned versus external import boundaries, detected hooks, event handlers, client boundaries, server boundaries, and explicit warnings for dynamic imports, React context, and implicit client boundaries.

This is not a full React renderer yet. It is the compiler ABI that makes the next renderer pass possible without guessing.

## State Graph ABI Step

The TSX App Router execution contract now embeds the compiler-owned `dx.tsx.stateGraph` state graph from the route unit. This exposes DX-native state slots, derived slots, event slots, effect slots, and server-action edges through `__DX_APP_ROUTER_EXECUTION__`, plus DOM counts through `data-dx-state-slots` and `data-dx-event-slots`.

React hooks are adapter-only authoring syntax. Exact compatibility patterns may lower into DX state slots, but unsupported hooks must emit diagnostics or remain behind explicit client islands. The next renderer task is to lower the DX-native state/event graph into generated JS client islands without claiming hidden React hook execution.

## State Runtime Step

Generic App Router responses now also emit `dx.tsx.stateRuntime` when a route has state or event slots. The generated script is source-owned, requires no `node_modules`, exposes `window.__DX_STATE_GRAPH_RUNTIME__`, provides `getSnapshot`, `setSlot`, and `dispatch`, and announces readiness through `dx:state-runtime-ready`.

This is still not full React execution. It is the first dev-runtime bridge from route-unit state meaning into generated JS client-island behavior. Arbitrary component imports, DOM binding for real JSX nodes, full effect ordering, and advanced hooks remain renderer follow-up work.

## TSX Render Plan Step

Generic App Router responses now include `dx.tsx.renderPlan`. The plan records the component graph, source-owned versus Forge-owned nodes, render/runtime readiness, and claim policy in the same response as `tsx_semantics` and `state_runtime`.

This deliberately blocks overclaiming. The public sentence remains: React-familiar apps with source-owned packages and no hidden dependency surface. The render plan marks drop-in React replacement, full Next.js App Router parity, global faster-than-Next claims, and full Forge replacement claims as blocked until implementation or measured receipts prove them.

## TSX Source Render Surface Step

Generic App Router responses now also include `dx.tsx.sourceRenderSurface`. This source-owned surface lowers the route page and its segment files into visible renderable elements, component references, prop bindings, and form surfaces, then exposes them through `__DX_APP_ROUTER_EXECUTION__`, `__DX_TSX_SOURCE_RENDER__`, and stable DOM markers such as `data-dx-tsx-renderable-elements`, `data-dx-tsx-component-refs`, and `data-dx-tsx-prop-bindings`.

The surface now follows bounded source-owned local TSX/JSX imports from route and segment files, including relative imports and `@/` aliases. It records scanned and skipped imports so Studio/check consumers can see which local component files informed the render surface.

This is another compatibility step. It makes normal `.tsx` files more inspectable for Studio, checks, and future runtime execution while making the next React/App Router compatibility blockers explicit: imported component execution, expression evaluation, client-island DOM binding, effect ordering, and advanced hook lowering.

## TSX Static DOM Snapshot Step

Generic App Router responses now include a bounded `dx.tsx.staticDomSnapshot` payload inside the source render surface. The snapshot serializes safe intrinsic JSX elements from the route, segment files, and scanned source-owned imports into a common-subset HTML fragment, then exposes status/count markers through `data-dx-tsx-static-snapshot` and `data-dx-tsx-static-snapshot-elements`.

This is still not full React execution. It does not run component functions, hooks, arbitrary JavaScript expressions, or event handlers. It is the next practical bridge: Studio/check consumers can now see a concrete static DOM skeleton for ordinary `.tsx` routes while the renderer continues toward literal prop evaluation, child composition, and event/state slot attachment.

## TSX Literal Prop Evaluation Step

The static DOM snapshot now evaluates simple literal TSX expressions for safe attributes and direct children: quoted strings, static template literals without interpolation, numbers, booleans, and nullish values. The generic route DOM exposes `data-dx-tsx-literal-expressions`, and the source surface records `static_dom_snapshot_literal_expressions`.

This remains deliberately narrow. Expressions such as identifiers, function calls, object spreads, arrays, ternaries, maps, and runtime data reads still stay out of the static snapshot until the real common-subset evaluator exists.

## TSX Static Template Prop Interpolation Step

Source-owned component previews now resolve bounded template literals with safe caller prop bindings. Common patterns such as `` `card ${className}`.trim() `` and `` `Hello ${props.name}` `` lower into the static return preview when every interpolation is a literal or a resolved prop alias.

This improves normal `.tsx` className/text rendering without pretending to run arbitrary JavaScript. Function calls other than the final `.trim()`, object spreads, computed members, hooks, effects, runtime data reads, and full React expression semantics remain out of scope for this bounded renderer.

## TSX Static Conditional Prop Branch Step

Source-owned component previews now resolve bounded ternary branches when the condition and selected output can be resolved from literals, safe template literals, or caller prop bindings. Common patterns such as `active ? "enabled" : "disabled"` and `` variant === "primary" ? `button ${variant}` : "button" `` lower into the static return preview.

This is a practical React-familiar compatibility step for className/text branching. It still rejects function calls, computed members, object/array spreads, hooks, runtime data reads, and arbitrary JavaScript evaluation.

## TSX Static Class List And Class Call Prop Step

Source-owned component previews now resolve bounded class-list expressions when the list is a static array followed by `.filter(Boolean).join(" ")`. Common patterns such as `["button", active && "is-active", variant === "primary" && "button-primary"].filter(Boolean).join(" ")` lower into static `className` output when every item is a literal, caller prop binding, safe template binding, or static condition.

Source-owned previews now also resolve bounded `cn(...)` and `clsx(...)` class calls for the same safe argument subset, including `cn("button", active && "is-active")` and `clsx("button", variant === "primary" ? "is-primary" : "is-secondary")`. It still rejects object/array `clsx` payloads, function calls, spreads, hooks, runtime data reads, and arbitrary JavaScript so the renderer stays truthful while covering a very common React/shadcn authoring pattern.

## TSX App Router Static Shell Composition Step

The composed static DOM surface now emits `dx.tsx.appRouterStaticShell`, composes page HTML through layout/template `{children}` placeholders, and generic TSX dev responses inject that shell with `data-dx-tsx-app-router-shell`. Full `<html><body>` layout output is normalized into a browser-safe body fragment before injection, so the dev response does not nest a whole document inside its fallback body. This makes the bounded TSX renderer behave more like the App Router for the common `app/layout.tsx` + `app/page.tsx` shape instead of treating layout and page as unrelated HTML fragments.

This is still a static TSX surface, not full React execution or a Next.js clone. It does not execute Server Components, Client Components, async data, context providers, effects, or React reconciliation. The next runtime step is live browser proof for shell composition and then removing the legacy static preview duplicate after QA approves the served response shape.

## TSX Client Island Manifest Step

Generic App Router responses now publish `dx.tsx.clientIslandManifest` through the source render surface and inject `__DX_TSX_CLIENT_ISLANDS__` into served HTML. The response also exposes `data-dx-client-islands`, and `dx.tsx.renderPlan` marks `client_island_manifest` plus `generated_dom_action_binder` as runtime readiness surfaces.

This is the next bridge from static route understanding into browser behavior: safe intrinsic button/input/form action targets are mapped to event slot IDs, state slots, `__DX_DOM_ACTION_BINDER__`, and `__DX_STATE_GRAPH_RUNTIME__` without requiring `node_modules`. It is not arbitrary Client Component execution, React synthetic events, effect ordering, context semantics, or full hydration. The next proof should be live browser QA showing a generated island dispatching through the state runtime.

## TSX Effect And Context Boundary Step

App Router semantics now emit `dx.tsx.effectContextBoundaryManifest`. It records effect slots, dependency names, `effect_scheduler_status`, `createContext` usage, provider counts, and `useContext` consumer counts so effects and context are explicit runtime boundaries rather than hidden TODOs.

This is not React effect scheduling or context propagation yet. It keeps `full_react_effect_ordering` and `full_react_context_runtime` false until www implements deterministic effect cleanup/order and source-owned provider lookup for the supported TSX subset.

## TSX Effect And Context DOM Visibility Step

Generic App Router responses now expose the same effect/context boundary contract directly in served HTML through `data-dx-effect-boundaries`, `data-dx-context-boundaries`, and `__DX_TSX_EFFECT_CONTEXT_BOUNDARIES__`. This gives DX Studio and browser/runtime probes a stable page-level surface for effect scheduler and context propagation work without parsing the whole execution payload.

This is still a boundary manifest, not effect execution. The next production slice is deterministic source-owned effect scheduling and provider lookup for the supported TSX subset, followed by live browser proof for cleanup/order and `useContext` reads.

## TSX Effect Scheduler Runtime Step

The generated state runtime now includes `dx.tsx.effectScheduler`. It emits a runtime script even when a route has effects but no state/event slots, schedules effect records on mount and after compiler-owned dependency state changes, exposes `scheduleEffectsForState`, and dispatches `dx:effect-scheduled` plus `dx:effect-scheduler-flush`. Generic App Router DOM exposes `data-dx-effect-scheduler`, and the render plan marks `effect_scheduler` as ready for routes with effect slots.

This is dependency scheduling only. DX-WWW still does not execute effect callback bodies, run cleanup functions, model subscriptions/timers, emulate Strict Mode double invocation, or claim full React effect ordering. The next proof should be live browser QA that observes the scheduling events, followed by a deliberately tiny safe callback subset only if it stays source-owned and deterministic.

## TSX Source-Owned Component Composition Step

The source render surface now links component JSX usages to bounded source-owned local imports. When a route uses an imported local component, DX-WWW records a `component_compositions` entry with the importer, source file, import kind, imported symbol, and that component file's safe static snapshot. Generic App Router DOM now exposes `data-dx-tsx-component-compositions`.

This is a useful bridge toward real component execution, but it is still not a React runtime replacement. It matches local source files and serializes their intrinsic JSX skeletons; it does not execute component functions, prop logic, hooks, effects, context, or runtime data reads.

## TSX Component Invocation Inputs Step

Component composition entries now also carry `dx.tsx.componentInvocationInputs`. DX-WWW records literal props, static expression props, event-handler prop boundaries, spread prop boundaries, direct text children, direct literal child expressions, and skipped child expressions for each source-owned local component usage.

This moves the common-subset renderer closer to real component execution, but it is still a bounded source contract. It does not run the component function, bind `children` into returned JSX, evaluate identifiers, attach events, run hooks, or claim React compatibility beyond static invocation inputs.

## TSX Children Insertion Preview Step

Source-owned component composition entries now include `dx.tsx.componentReturnPreview`. The preview serializes the imported component file's safe intrinsic JSX and substitutes direct `{children}` / `{props.children}` placeholders with the caller's direct text and literal expression children when that can be done without running JavaScript.

This is intentionally not full component execution. It now preserves ordered JSX child nodes and resolves simple prop/template bindings, but it still does not execute component functions, attach events, run hooks, evaluate arbitrary expressions, or provide full React semantics. It gives Studio/check a truthful preview of the smallest executable local-component subset.

## TSX Component Runtime Binding Plan Step

The component return preview now also carries `dx.tsx.componentRuntimeBindingPlan`. For each source-owned local component, DX-WWW records matching state slots, matching event slots, and intrinsic form-lowering metadata from the same source file so Studio/check can see which previewed component output is ready for future client-island attachment.

This is still not hydration or React event execution. The contract reports `full_dom_binding: false`, keeps native form metadata separate from React submit handler execution, and labels event listener attachment as planned from the state graph rather than silently pretending browser listeners are wired.

## TSX DOM Action Descriptor Step

The runtime binding plan now includes `dx.tsx.domActionDescriptors` for safe intrinsic controls. DX-WWW emits source-owned descriptors for `button`, `input`/`textarea`/`select`, and `form` surfaces from local component files, carrying event attributes, matching state event slots, field metadata, and native submit boundaries.

This is not browser listener attachment or React event execution. The descriptors keep `browser_listeners_attached: false` and `full_react_event_execution: false`; the next pass is to turn these descriptors into generated JS handlers for the common subset.

## TSX DOM Action Binder Step

Generic App Router responses now aggregate those descriptors into `dx.tsx.domActionBinder` and inject a no-`node_modules` script with id `__DX_DOM_ACTION_BINDER__`. The binder marks matched intrinsic controls, listens for safe `click`, `input`/`change`, and native `submit` preview events, dispatches `dx:dom-action-preview`, and announces `dx:dom-action-binder-ready`.

This is still not React event execution. The script does not call component handler bodies, does not use React synthetic events, and keeps full hook parity unclaimed.

## TSX DOM Action State Bridge Step

The DOM action binder now carries `dx.tsx.domActionStateBridge`. Matched safe previews dispatch into `window.__DX_STATE_GRAPH_RUNTIME__.dispatch`, emit `dx:state-runtime-dispatch`, and mark controls with `data-dx-state-runtime-dispatch` so Studio/check consumers can see whether a safe action reached the compiler-owned state runtime.

The generated state runtime now applies lowerable DX state operations: `set-from-input`, `toggle`, `add`, `subtract`, and `set-literal`. This updates compiler-owned state slots without running React handler bodies or claiming React synthetic events, effect ordering, reducer/context support, or full component execution.

## TSX State DOM Reflection Step

Generic App Router responses now carry `dx.tsx.stateDomReflection` plus a hidden `data-dx-tsx-static-dom-preview="safe-common-subset"` snapshot before runtime scripts. The static snapshot includes compiler-owned `data-dx-state-*` markers for direct state reads, and the generated state runtime exposes `state_dom_reflection`, `reflectStateSlotToDom`, startup reflection, and `dx:state-dom-reflection` events.

This is a real response-DOM bridge for Studio/runtime probes, not full React reconciliation. The next engine task is to lower simple caller prop identifiers into source-owned component return previews while keeping arbitrary JavaScript, hooks, effects, and full DOM reconciliation out of scope until they are genuinely implemented.

## TSX Context Runtime Step

Generic App Router responses now carry `data-dx-context-runtime` and a source-owned `__DX_CONTEXT_RUNTIME__` module script derived from `dx.tsx.contextRuntime`. The semantics pass detects context names from `createContext`, `.Provider`, and `useContext(...)`; the runtime exposes `getContext`, `resolveContextValue`, `setContext`, and `attachContextRuntime`, reflects values to `data-dx-context-read` markers, and emits `dx:context-runtime-ready` plus `dx:context-value`.

The context runtime now seeds named values from safe literal `createContext(...)` defaults and safe literal `<Context.Provider value=...>` attributes. Those seeds are emitted through `context_runtime.initial_values`, counted as `context_initial_values`, and surfaced on App Router responses through `data-dx-context-initial-values`.

This is a provider-value map for DX Studio/runtime probes, not full React context propagation. It does not execute dynamic provider expressions, execute `useContext`, reconcile providers through a component tree, run concurrent semantics, or execute arbitrary component imports. The next engine step is browser QA for seeded values and `setContext`, then source-owned provider-tree lowering only where the TSX renderer already composes the component tree deterministically.

## TSX Simple Prop Identifier Step

Source-owned component return previews now include `dx.tsx.componentPropIdentifierBindings`. DX-WWW collects literal caller props from a local component invocation, resolves `{propName}` and `{props.propName}` reads inside safe intrinsic preview HTML, and reports skipped dynamic, spread, and event props separately.

This is still not full component execution. It does not run component functions, evaluate spreads, execute hooks, or claim arbitrary JavaScript support.

## TSX Destructured Prop Alias Step

Source-owned component return previews now include `dx.tsx.componentDestructuredPropAliases`. DX-WWW scans source-owned component files for simple object-pattern parameters such as `function Card({ title })` and `const Card = ({ title }) => ...`, records the supported signature pattern, and uses those aliases when resolving literal caller props inside safe intrinsic preview HTML.

This is a real React-shaped compatibility step, not full prop evaluation. Rest props, nested patterns, computed keys, spreads, hooks, effects, context, and arbitrary JavaScript are still skipped.

## TSX Renamed And Default Prop Alias Step

The same destructured-prop contract now handles simple renamed aliases and static literal defaults. DX-WWW records `renamed-alias-object-pattern` for signatures like `{ title: heading }`, records `default-value-object-pattern` for signatures like `{ title = "Untitled" }`, stores alias-to-prop metadata, and resolves those reads inside safe intrinsic previews when the caller provided a literal prop or the alias has a static literal default.

This still does not run component functions or evaluate arbitrary parameter logic.

## TSX Oxc Parser And JSX Surface Step

DX-WWW now has an active Oxc-backed parser path instead of merely declaring Oxc as an unused optional crate. `core/src/delivery/tsx_ast.rs` emits `dx.tsx.parserBackend`, calls `oxc_parser::Parser` under the `oxc` feature, derives import declarations and `export const metadata` route metadata from the Oxc `Program` AST, carries backend status through `DxTsxModuleAst` and `DxReactJsxDocument`, and the generic App Router source-render JSON reports `parser_backends` plus `oxc_validated_documents`. `dx-www --features cli` now enables `dx-www-compiler/oxc`, so the public CLI path validates TSX and reads imports/metadata with the Rust parser backend.

The JSX lowering path also reports `dx.tsx.oxcJsxSurface`. It derives JSX elements, attributes, text, expression containers, event/key hints, common conditional/list expression surfaces, call/new arguments, arrays, objects, TypeScript wrappers, and statement branches from the Oxc AST, with `jsx_backends` and `oxc_jsx_documents` visible in the source-render JSON.

Important limit: this is parser-backed source understanding, not full React execution. DX-WWW still does not execute arbitrary component functions, hooks, effects, context, dynamic prop logic, or React reconciliation. The next production task is safe source-owned component execution for the common App Router subset: layout/page composition, literal props, children order, and client-island state/event binding.

## TSX Composed Static DOM Snapshot Step

The source-render surface now includes `dx.tsx.composedStaticDomSnapshot` with `schema_revision: 1`. It walks route, layout, template, and page documents, keeps safe intrinsic JSX snapshots, and inlines local source-owned component preview HTML from the component return preview contract when a route imports and renders that component.

The contract reports `composed_static_dom_snapshot_elements`, `component_preview_insertions`, `component_prop_identifier_bindings`, skipped component references, `component_preview_html`, and `source-owned-component-preview` markers while keeping `full_component_execution: false` and `full_react_execution: false`.

This is a stronger common-subset bridge. It makes local component usage visible and partially materialized for DX Studio/check without running arbitrary JavaScript. Remaining TSX engine work is ordered child graphs, safe component function execution for a narrow subset, visible response-DOM attachment, client island lowering, effects/context/reducer boundaries, and browser runtime proof.

## TSX Ordered Child Tree

The current TSX pass now preserves ordered JSX child structure with `parent_index` and `child_nodes` in the compiler JSX surface. The source renderer consumes that tree for nested intrinsic DOM and nested source-owned component preview insertion, so safe static TSX output no longer degrades into flat sibling-only HTML for common nested component/page layouts.

This is still bounded TSX execution, not a full React runtime replacement. The next engine work is safe expression evaluation for common ternary/map/className patterns, then client-island lowering and visible browser proof.

## Contract Naming Cleanup

The TSX/App Router contracts now use professional schema names with separate revision fields instead of public generated version suffixes. Examples: `dx.tsx.appRouterSemantics`, `dx.tsx.stateGraph`, `dx.tsx.stateRuntime`, `dx.tsx.renderPlan`, `dx.tsx.sourceRenderSurface`, `dx.tsx.componentReturnPreview`, `dx.tsx.domActionBinder`, and `dx.framework.riskRegister`.

This cleanup is scoped to the TSX/framework runtime lane. Forge and launch evidence receipts still have older suffix-style names and need a coordinated migration because several workers and generated receipts consume them.
