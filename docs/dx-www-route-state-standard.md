# DX-WWW Route, Page, And State Compilation Standard

DX-WWW keeps authoring TSX-shaped and familiar, but the compiler owns route meaning, state meaning, package ownership, and delivery cost. A developer can write `app/page.tsx`, local components, server actions, and styles in visible source files while DX-WWW lowers each route into a reviewable Route Unit.

## Route Unit

Every compiled App Router route should produce a Route Unit with these sections:

- `shell`: crawlable fallback HTML, generated CSS hrefs, streaming strategy, and whether a browser runtime is required.
- `graph`: typed component, style, and source graph for the route.
- `state`: local state slots, derived slots, event slots, effects, and server-action edges.
- `packet`: DXPK packet metadata when a browser packet is useful.
- `receipt`: Forge/source ownership evidence, source paths, package ids, and no-`node_modules` runtime proof.
- `runtime_report`: selected delivery mode, rejected alternatives, candidate byte estimates, and benchmark caveats.

## State Graph

State is local by default. DX-native `state()`, `derived()`, `effect()`, and `action()` are the runtime source of truth. React hook-shaped syntax is adapter-only authoring inventory unless an exact compiler proof lowers it into DX state slots; unsupported hooks must produce diagnostics or stay behind an explicit client island adapter.

The state graph records:

- mutable slots from `state()`
- derived slots from `derived()` expressions that depend on state
- event slots from JSX event attributes
- effect slots from `effect()` boundaries
- action slots from `action()` handlers
- server-action edges from imports under `server/`

Page/global state should become explicit in source and diagnostics before DX-WWW treats it as shared state.

## Adaptive Delivery

Each route must explain why it picked one of the supported delivery modes:

- `static`: HTML/CSS only.
- `micro-js`: tiny generated JavaScript for simple local interactions.
- `server-fragment`: server-rendered fragments when safer or smaller.
- `wasm-core`: DXPK plus shared WASM runtime when the byte/behavior tradeoff wins.
- `wasm-split`: larger app islands with a split boot/runtime path.

Reports must count shell bytes, generated CSS, runtime bytes, DXPK bytes, dictionaries, shims, and metadata together before public performance claims.

## AI/LLM-Friendly Source Standard

Strict DX-WWW apps should keep one obvious source of truth:

- root `dx`
- `app/`
- `components/`
- `server/`
- `styles/`
- `.dx/forge/`

`dx check --project-contract` should flag large source files, hidden barrel-file imports outside package boundaries, dynamic imports without an explicit reviewed boundary, client components importing server-owned files directly, unmanaged `node_modules`, separate config sprawl, and Forge provenance gaps.
