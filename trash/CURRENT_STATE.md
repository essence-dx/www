# DX WWW Current State: React And Next Compatibility

Date: 2026-05-27
Workspace: `G:\Dx\www`
Mode: report-only investigation

This document answers one question honestly:

Can DX WWW eventually render normal React and Next.js projects by covering the React/Next API surface in a DX-owned JS/WASM runtime, while keeping the DX no-opaque-node-modules / Forge-first discipline?

## Executive Verdict

It is not impossible.

But it is not a small renderer tweak. To support real React projects and real React packages, DX WWW must become a React-compatible runtime and framework adapter, not only a TSX static renderer.

The current WWW framework is strong as a source-owned App Router-shaped compiler/runtime subset. It can discover routes, parse TSX, lower safe JSX, generate build/dev receipts, serve pages, emit client-island metadata, and run a limited DX-owned state/event binder.

The current WWW framework is not yet a drop-in replacement for:

- `react`
- `react/jsx-runtime`
- `react/jsx-dev-runtime`
- `react-dom`
- `react-dom/client`
- `react-dom/server`
- React Server Components runtime semantics
- full Next.js App Router runtime semantics
- arbitrary React package behavior

The reason the Zen/onboard project exposed the blocker is simple: the original UI is not just JSX markup. It uses React component execution, hooks, effects, refs, canvas, runtime state, DnD providers, motion adapters, context, and browser event lifecycles. WWW currently analyzes and statically previews parts of that world; it does not execute the full React tree.

## Hardness Score

These scores use `100` as "extremely hard", not "quality".

| Goal | Hardness | Current Readiness |
| --- | ---: | ---: |
| Render DX-authored static App Router pages with no local `node_modules` | 25/100 | 80/100 |
| Convert simple Next/React projects that are mostly static JSX and global CSS | 45/100 | 60/100 |
| Convert custom React UI with hooks, effects, canvas, DnD, and motion after replacing dependencies | 75/100 | 30/100 |
| Support most popular React packages through DX-owned compat/adapters | 88/100 | 15/100 |
| Support arbitrary React packages that claim React compatibility | 93/100 | 10/100 |
| Support arbitrary modern Next.js App Router projects | 95/100 | 20/100 |

My direct answer:

- Impossible? No.
- Can DX do it by matching React APIs? Yes, if "matching" means matching behavior, not just function names.
- Why is it not working now? Because current WWW does not execute the React component tree, hook dispatcher, reconciler, hydration, synthetic events, context propagation, effects, portals, Suspense, or full Next/RSC runtime.
- What is the honest full-project difficulty? `93/100` for broad React package compatibility and `95/100` for broad Next.js compatibility.
- What is the sane next target? Build a DX React compatibility ladder that starts with common React app execution, not "all React and all Next" on day one.

## What The Current WWW Framework Is

Current DX WWW is a Rust-first, source-owned framework with Next-familiar authoring.

It has real foundations:

- App Router-like route discovery for `app` and `src/app`.
- Route segment parsing for static, dynamic, catch-all, optional catch-all, and route groups.
- TSX parsing and source graph analysis.
- Static JSX lowering for intrinsic elements and safe literal/static expressions.
- Static lowering for some familiar surfaces like `next/link` and `next/image`.
- Metadata and route handler receipt surfaces.
- Build output receipts and App Router execution contracts.
- Hot reload endpoints and browser dev feedback.
- Client-island artifacts.
- A limited source-owned state/event runtime.
- A no-local-`node_modules` template discipline.
- Forge-owned ecosystem intent.

That is a real product direction. It is not fake.

But the current rendering model is mostly:

1. discover source files,
2. parse TSX,
3. analyze imports/hooks/events/context,
4. render a safe static DOM preview,
5. emit contracts/receipts,
6. attach limited DX runtime markers or a limited binder.

It is not:

1. load arbitrary JS/TS modules,
2. call component functions,
3. build a React element tree,
4. reconcile updates,
5. run hooks and effects,
6. hydrate server HTML,
7. replay events,
8. run client transitions,
9. stream RSC/Flight payloads,
10. behave exactly like React DOM.

## Local Evidence

The current source says this directly.

| Evidence | Meaning |
| --- | --- |
| `dx-www/src/cli/app_router_semantics.rs` contains `react_compatibility_claim: "react-familiar-compiler-owned"` and `full_react_runtime_parity: false`. | WWW claims React-familiar authoring, not full React runtime parity. |
| `dx-www/src/cli/app_router_execution/source_render.rs` sets `full_jsx_execution: false`. | The renderer is not executing arbitrary JSX/component code. |
| `source_render.rs` uses `MAX_SOURCE_IMPORT_DEPTH = 2` and `MAX_SOURCE_IMPORT_DOCUMENTS = 24`. | Source import traversal is intentionally bounded. |
| `source_render.rs` sets `full_component_execution: false` and `full_react_execution: false`. | Imported components are statically previewed, not fully executed. |
| `source_render.rs` sets `full_react_hydration: false` and `react_synthetic_events: false`. | Client islands are not React hydration or React synthetic events. |
| `app_router_execution/state_runtime.rs` sets `full_react_hook_runtime: false`. | Hooks are not a full React dispatcher. |
| `state_runtime.rs` lists missing effect callback cleanup/order and `useReducer`/`useContext`/`useTransition` compatibility. | Hook behavior is recognized as a gap. |
| `app_router_semantics.rs` says context providers/consumers are registered but `useContext` is not executed. | Context is analyzed, not propagated through a live React tree. |
| `dx-www/src/cli/dev_hot_reload_client.rs` uses `EventSource`, polling, and `location.reload()`. | Dev feedback is route/style reload, not React Fast Refresh or module-level HMR. |
| `dx-www/src/dev/axum_server.rs` exposes `/_dx/hot-reload/version` and `/_dx/hot-reload/events`. | The dev protocol exists, but it is not React's runtime refresh model. |
| `examples/onboard/.dx/receipts/build/readiness.json` has `product_ready: false`, `runtime_proof: false`, and `full_react_hydration: false`. | The onboard conversion is not yet product-runtime ready. |
| `examples/onboard/.dx/www/output/app/client-islands.json` has `event_count: 0`. | The active onboard route currently has no real event slots wired in the generated island output. |

## Official React Surface That DX Must Match

The current React docs show React `19.2` and the public surface is broad. A React-compatible DX runtime cannot only support `useState`.

Required `react` surface includes:

- component APIs: `Fragment`, `Profiler`, `StrictMode`, `Suspense`
- hooks: `useActionState`, `useCallback`, `useContext`, `useDeferredValue`, `useEffect`, `useEffectEvent`, `useId`, `useImperativeHandle`, `useInsertionEffect`, `useLayoutEffect`, `useMemo`, `useOptimistic`, `useReducer`, `useRef`, `useState`, `useSyncExternalStore`, `useTransition`
- APIs: `cache`, `createContext`, `lazy`, `memo`, `startTransition`, `use`
- legacy but package-relevant APIs: `Children`, `cloneElement`, `Component`, `createElement`, `createRef`, `forwardRef`, `isValidElement`, `PureComponent`

Required `react-dom` surface includes:

- `createPortal`
- `flushSync`
- resource hint APIs such as `preconnect`, `preload`, `preinit`, and module variants
- client APIs: `createRoot`, `hydrateRoot`
- server APIs: `renderToPipeableStream`, `renderToReadableStream`, `renderToStaticMarkup`, `renderToString`, `resume`, `resumeToPipeableStream`
- static APIs: `prerender`, `resumeAndPrerender`, and related variants

Official anchors:

- React reference: https://react.dev/reference/react
- `createRoot`: https://react.dev/reference/react-dom/client/createRoot
- `hydrateRoot`: https://react.dev/reference/react-dom/client/hydrateRoot
- React Server Components: https://react.dev/reference/rsc/server-components

The most important detail: React is responsible for calling components and hooks. Matching the API names without matching that runtime responsibility does not make React packages work.

## Official Next Surface That DX Must Match

Next.js App Router compatibility is also bigger than file names.

Current Next docs include:

- file conventions such as `page`, `layout`, `template`, `loading`, `error`, `not-found`, `forbidden`, `unauthorized`, `default`, `route`, `proxy`, `instrumentation`, metadata files, `src`, `public`, route groups, parallel routes, intercepting routes, and route segment config
- route handlers using Web `Request`/`Response`, `NextRequest`, supported HTTP methods, params, dynamic segments, cookies, headers, streaming, body/form data, CORS, and cache revalidation
- `next/link` client navigation and prefetch behavior
- `next/image` optimization, generated `srcset`, loaders, remote image policy, device sizes, placeholders, and config-driven behavior
- `next.config` options for images, redirects, rewrites, base path, asset prefix, headers, transpile packages, turbopack, server actions, cache, deployment, CSS, and more

Official anchors:

- Next file-system conventions: https://nextjs.org/docs/app/api-reference/file-conventions
- Next route handlers: https://nextjs.org/docs/app/api-reference/file-conventions/route
- Next Link: https://nextjs.org/docs/app/api-reference/components/link
- Next Image: https://nextjs.org/docs/app/api-reference/components/image

WWW supports some of this as source-owned subsets. It does not yet support full Next runtime behavior.

## Why "Same API" Is Not Enough

The intuition is good: if DX exposes the same API shape as React, React packages can import it.

But React package compatibility depends on behavioral contracts:

- React element shape and identity, including `key`, `ref`, symbols, fragments, and dev metadata.
- Component invocation rules.
- Hook call order and hook slot identity.
- State update batching.
- Scheduler priority and transitions.
- Effect timing and cleanup ordering.
- `useLayoutEffect` vs `useEffect` timing.
- Context provider/consumer propagation.
- Ref attach/detach timing.
- Synthetic event delegation and event object behavior.
- Portals preserving React tree context while rendering elsewhere in the DOM.
- Suspense behavior through thrown thenables, fallback reveal, retry, and nested boundaries.
- Error boundary behavior.
- Hydration matching, mismatch recovery, and event replay.
- Server component serialization and client boundary rules.
- Development behavior such as Strict Mode double invocation and Fast Refresh expectations.

If DX implements `useState` with the same name but not the same render/update semantics, many packages still break. If DX implements `createPortal` as "append DOM somewhere" but does not preserve React-tree context and event behavior, Radix-like components break. If DX parses `useEffect` but does not run the callback and cleanup at the right time, animation/canvas/DnD packages break.

So the real target is not "same API names". The target is "same enough runtime semantics, with receipts for every unsupported difference".

## Why The Zen/Onboard UI Did Not Render As The Original React App

The onboard project is the clearest local proof.

The active route is currently a static facade:

- `examples/onboard/app/page.tsx`
- mostly raw JSX tags and copied visual structure
- no real import of the preserved full client component graph

The preserved original source is more dynamic:

- `examples/onboard/components/onboard-next-source.tsx`
- starts with `"use client"`
- imports `DndContext`, `DragOverlay`, `zen-dnd`, `zen-motion`
- uses `useState`, `useEffect`, refs, browser state, overlays, drag events, and runtime interaction

Specific blockers:

- `examples/onboard/components/PixelCircle.tsx` uses `useRef`, `useEffect`, canvas, `document.createElement("canvas")`, and `requestAnimationFrame`. Static JSX lowering cannot make that animation real.
- `examples/onboard/components/friday.tsx` uses React state, `useEffect`, `window.scrollY`, `window.scrollTo`, and `requestAnimationFrame`. Static rendering cannot execute the Friday glow/scroll behavior.
- `examples/onboard/lib/zen-dnd.tsx` depends on React context, pointer lifecycle, DOM geometry, and runtime state.
- `examples/onboard/lib/zen-motion.tsx` is a hook/effect-based motion shim, not just a CSS class.
- `components/ui/*` wrappers use context patterns similar to Radix/shadcn primitives; real behavior needs portals, refs, focus, keyboard handling, escape/outside click behavior, and provider propagation.

That is why a static JSX route can look like a rough page but not behave like the original. The original needs a runtime.

## Current Compatibility State By Layer

| Layer | Current State | Honest Gap |
| --- | --- | --- |
| File discovery | Strong App Router-inspired subset. | Not full Next file convention parity. |
| TSX parsing | Strong and improving. | Parsing is not execution. |
| JSX lowering | Good for safe intrinsic/static JSX. | No arbitrary component function execution. |
| Component imports | Bounded source-owned static preview. | No general module graph runtime. |
| Hooks | Detected; simple `useState` slots partially lowered. | No full hook dispatcher. |
| Effects | Detected/scheduled as records. | Callback body, cleanup, and timing not executed. |
| Context | Providers/consumers detected. | No real provider tree propagation or `useContext` execution. |
| Events | Limited DOM binder for safe event slots. | No React synthetic event system or arbitrary handler execution. |
| Refs | Not a real React ref runtime. | Need object refs, callback refs, `forwardRef`, attach/detach timing. |
| Hydration | Explicitly false for full React hydration. | Need `hydrateRoot`-style DOM takeover and mismatch handling. |
| Suspense | Boundary data exists. | No thrown-promise suspension, retry, streaming reveal, transitions. |
| Client islands | Artifacts exist. | Not React client islands yet. |
| Hot reload | SSE/poll/reload works. | No React Fast Refresh or partial component update semantics. |
| `next/link` | Static anchor lowering. | No full client router/prefetch/navigation semantics. |
| `next/image` | Static image lowering. | No full optimizer, loaders, config, remote policy, generated `srcset`. |
| Route handlers | Safe subset and receipts. | No full Next request/middleware/stream/cache behavior. |
| RSC | Boundaries detected. | No full RSC/Flight runtime. |
| CSS | Global CSS/public assets are strong. | CSS Modules, runtime style injection, and framework parity need proof. |

## What DX Must Build To Support Real React Packages

### 1. DX React ABI

Create Forge-owned virtual packages:

- `react`
- `react/jsx-runtime`
- `react/jsx-dev-runtime`
- `react-dom`
- `react-dom/client`
- `react-dom/server`
- `scheduler`

These must be resolved by WWW/Forge, not installed in project-local `node_modules`.

Minimum first milestone:

- React element object shape
- `jsx`, `jsxs`, `jsxDEV`
- `Fragment`
- `createElement`
- `isValidElement`
- `Children`
- `cloneElement`
- `memo`
- `forwardRef`
- `createContext`
- `createRef`

### 2. DX Reconciler

DX needs a runtime that can:

- call function components,
- preserve component identity,
- diff keyed children,
- update DOM nodes,
- mount/unmount cleanly,
- handle refs,
- handle errors,
- schedule updates,
- batch state updates,
- commit effects in the correct phase.

This can be JS-first, WASM-assisted, or WASM-first. The public API boundary must stay JS-compatible because React packages import JS modules.

### 3. Hook Dispatcher

Minimum hook compatibility:

- `useState`
- `useReducer`
- `useRef`
- `useMemo`
- `useCallback`
- `useEffect`
- `useLayoutEffect`
- `useInsertionEffect`
- `useContext`
- `useId`
- `useSyncExternalStore`
- `useTransition`
- `useDeferredValue`
- `useOptimistic`
- `useActionState`
- `use`

This must include hook order validation and custom hook behavior, because libraries are built around those invariants.

### 4. Event System

DX needs either:

- a React-compatible synthetic event layer, or
- a clearly documented DX event layer plus adapter guarantees for supported packages.

For broad React package compatibility, synthetic-like behavior is needed:

- root event delegation,
- capture and bubble phases,
- normalized event objects,
- `preventDefault`,
- `stopPropagation`,
- form/input edge cases,
- pointer/mouse/keyboard/focus behavior,
- event ordering around updates.

### 5. Hydration

DX must implement a browser takeover path comparable to `hydrateRoot`:

- match existing server HTML,
- attach listeners,
- preserve IDs,
- recover from mismatches,
- support Suspense boundaries,
- support partial/selective hydration or a deliberate DX alternative,
- report hydration errors clearly.

### 6. Suspense And Transitions

Modern React packages increasingly depend on:

- `Suspense`
- `lazy`
- thrown-promise suspension
- nested boundary retry
- fallback reveal
- `startTransition`
- `useTransition`
- `useDeferredValue`
- scheduler priorities

DX can stage this, but broad compatibility eventually needs it.

### 7. RSC And Next App Router Adapter

For Next-style projects, React compatibility is not enough. DX also needs:

- server/client module graph,
- `"use client"` boundary enforcement,
- `"use server"` / server actions,
- Flight-like serialization or a DX-owned equivalent,
- async Server Component execution,
- metadata resolution,
- route segment config,
- streaming behavior,
- navigation/cache/revalidation semantics,
- `next/navigation`,
- `next/headers`,
- `next/server`,
- `next/font`,
- `next/image`,
- `next/link`,
- middleware/proxy behavior or an honest unsupported marker.

React Server Components are especially sensitive because the official docs warn that framework/bundler implementation APIs do not follow normal semver and should be pinned to a React version. DX should pin a compatibility target rather than chasing "latest" blindly.

### 8. Package Certification Receipts

Do not claim "React packages supported" generically.

Create receipts per package family:

- `lucide-react`: icon-only, easy source-owned replacement.
- shadcn primitives: mostly wrappers, but depend on refs, context, portals, event/focus semantics.
- Radix-like primitives: hard because behavior is the product.
- `@dnd-kit`: hard because pointer lifecycle, DOM geometry, drag overlay, collision, sensors.
- Motion/Framer-like APIs: hard because layout animation, transitions, spring timing, presence, refs.
- TanStack Query: medium-hard because external store/subscriptions/cache lifecycle.
- React Hook Form: medium-hard because refs, uncontrolled inputs, subscriptions, event semantics.
- Zustand/Jotai-like stores: medium if `useSyncExternalStore` is correct.
- R3F/Three: hard because reconciler and canvas host behavior.

Each package should get:

- import scan,
- API coverage map,
- runtime fixture,
- browser proof,
- failure receipts,
- "supported / adapter / unsupported" status.

## Recommended Architecture

The best path is a hybrid ladder, not one giant rewrite.

### Stage 0: Keep Current WWW

Keep the current source-owned App Router compiler and static output. It is valuable for:

- fast static pages,
- no-node projects,
- receipts,
- dev preview,
- DX visual editing,
- build provenance,
- simple templates.

Do not throw it away.

### Stage 1: Add A React Compatibility Mode

Add a mode that can be enabled per route/project:

- `dx-native`: current static/source-owned path.
- `react-compat`: DX runtime executes React-shaped components.
- `react-island`: embed/execute only selected interactive islands.
- `adapter-boundary`: supported through Forge adapter.
- `unsupported`: honest blocker with machine-readable details.

### Stage 2: Create Forge Virtual Packages

Forge should materialize the DX-owned React compatibility packages:

- no local app `node_modules`,
- source-owned package code,
- versioned receipts,
- checksum/provenance,
- no hidden vendoring.

### Stage 3: Execute Simple Component Trees

First real runtime milestone:

- component invocation,
- props,
- children,
- fragments,
- keyed lists,
- conditional rendering,
- `useState`,
- `useReducer`,
- `useRef`,
- `useEffect`,
- simple event handlers,
- re-render into DOM.

This would already fix many custom React UIs that currently need static facades.

### Stage 4: Hydrate DX-Owned Client Islands

Replace "static facade plus marker" with real runtime attachment:

- generated client module graph,
- island root,
- state slots,
- handler execution,
- effect execution,
- context propagation,
- browser proof.

### Stage 5: Package Compatibility Harness

Create a compatibility suite with real examples:

- Zen/onboard original UI as a fixture.
- shadcn/Radix primitive fixture.
- DnD fixture.
- Motion fixture.
- Form fixture.
- Data fetching/store fixture.
- Next App Router fixture.

Every fixture should produce `.dx` receipts and browser proof.

### Stage 6: Next Adapter Layer

Add Next-specific virtual modules and runtime behavior:

- `next/link`
- `next/image`
- `next/navigation`
- `next/headers`
- `next/server`
- `next/font`
- route handlers
- metadata
- redirects/rewrites
- middleware/proxy
- cache/revalidate APIs
- server actions
- RSC/Flight or DX-owned equivalent

This should be separate from React compatibility so the React layer remains useful outside Next-style projects.

### Stage 7: WASM Acceleration

Use WASM where it gives measurable wins:

- parser/lowering,
- DOM diff planning,
- style/class compilation,
- source maps,
- route graph,
- serialization,
- static analysis.

Do not use WASM as a reason to change React semantics. Packages do not care if internals are Rust/WASM/JS. They care that imports and behavior work.

## Strategic Options

### Option A: Full DX React Runtime

DX implements the React public API and a compatible DOM runtime.

Pros:

- strongest DX ownership,
- best long-term no-node story,
- can be optimized for DX visual editing and serializer receipts.

Cons:

- hardest path,
- very high compatibility burden,
- React ecosystem edge cases will take time.

Score: best vision, hardest engineering.

### Option B: Preact-Style Compatibility

DX builds a smaller React-compatible layer inspired by the idea of `preact/compat`.

Pros:

- faster useful compatibility,
- many packages may work sooner,
- smaller runtime than full React.

Cons:

- behavior differences become product bugs,
- exact parity is still hard,
- advanced packages still fail.

Score: best middle path for early useful wins.

### Option C: Use Real React Internals Through A Host Config

DX uses React/reconciler concepts and targets DX host operations.

Pros:

- real hook/reconciler semantics sooner,
- less reimplementation risk.

Cons:

- fights the no-node/source-owned story unless vendored through Forge,
- React internals can be unstable,
- still needs Next/RSC integration.

Score: practical but politically/architecturally sensitive.

### Option D: Embed React For Advanced Islands

Keep DX native for most pages, but allow opt-in embedded React runtime for complex islands.

Pros:

- fastest route to real package compatibility,
- best for DnD, motion, charts, editors, complex forms.

Cons:

- weaker "we replaced React DOM" claim,
- must be carefully Forge-owned to avoid hidden node dependency,
- two runtime modes.

Score: best emergency bridge.

### Recommended Choice

Use a hybrid:

1. Keep current DX-native static compiler.
2. Add Forge-owned React ABI virtual packages.
3. Build a DX React compatibility runtime for common apps.
4. Allow `react-island` escape hatches for complex packages until DX native compatibility catches up.
5. Certify packages with receipts instead of broad claims.

This preserves the DX vision and avoids repeating the onboard mistake of replacing a real React UI with a static copy.

## What Not To Do Next

Do not:

- keep making static facades for dynamic React UIs,
- claim full Next parity because route files are discovered,
- claim React package support because imports can be scanned,
- hard-code one UI conversion instead of fixing runtime categories,
- hide unsupported behavior behind "looks close",
- rewrite good UI into simpler DX-looking components,
- install normal `node_modules` into example projects as the default answer,
- treat server docs, receipts, or static snapshots as runtime proof.

## Brutal Root Cause

The current WWW framework was designed around a source-owned compiler and safe static preview path.

That is why it is fast, controlled, no-node, and good for DX-native pages.

But arbitrary React/Next projects expect a live runtime:

- module execution,
- component execution,
- hook execution,
- event execution,
- effect execution,
- context propagation,
- reconciliation,
- hydration,
- router/runtime APIs,
- package ABI compatibility.

Current WWW has the language shape but not the full runtime shape.

That does not make the project wrong. It means the next frontier is clear.

## The Real 100/100 Definition

WWW should not define `100/100` as "we can hand-convert a Next screen until it visually resembles the source".

For this compatibility lane, `100/100` should mean:

1. A real preserved React source file is the active route.
2. The route imports DX/Forge-owned `react` and `react-dom` virtual packages.
3. The component function tree executes.
4. Hooks execute and re-render.
5. Effects execute and clean up.
6. Context providers/consumers work.
7. Refs attach correctly.
8. Event handlers execute.
9. Hydration or DX resumability is proven in a browser.
10. At least one hard fixture passes: Zen/onboard PixelCircle, Friday, DnD shell, and motion/presence behavior.
11. `.dx` receipts say exactly which React/Next surfaces were used and which were native, adapted, or unsupported.
12. No app-local `node_modules` are required.

Until that is true, the honest claim is:

DX WWW supports a growing source-owned React-familiar subset. It does not yet support arbitrary React/Next projects as drop-in runtime-compatible apps.

## Immediate Next Work If We Choose This Lane

The next implementation plan should be:

1. Create `dx-react-compat` as a Forge-owned virtual package set.
2. Implement `react/jsx-runtime` element objects and `Fragment`.
3. Implement a minimal component invocation loop with keyed child reconciliation.
4. Implement real `useState`, `useReducer`, `useRef`, `useEffect`, `useLayoutEffect`, and `useContext`.
5. Execute event handler bodies for intrinsic DOM events.
6. Add a client island root that can hydrate/attach to server HTML.
7. Make Zen/onboard use the real preserved source as the route again.
8. Add receipt gates that fail if a dynamic React UI is replaced by a static facade.
9. Add package certification fixtures for shadcn/Radix, DnD, motion, and forms.
10. Only then widen to Next-specific APIs like `next/navigation`, `next/image`, `next/font`, server actions, and RSC.

## Final Call

DX can absolutely grow into this.

The current project is not blocked by theory. It is blocked by runtime scope.

The right mental model is:

Current WWW is a source-owned App Router compiler and static/runtime subset.

Next WWW needs a DX-owned React compatibility runtime and Next adapter layer.

That is a hard but believable path. It is not impossible, and it is exactly the kind of infrastructure that would make DX meaningfully different from "just another Next wrapper" once it works.
