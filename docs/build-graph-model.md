# DX Build Graph Model

Status: first source-owned slice implemented for receipt generation, dependency invalidation, and compact consumer snapshots. This is a DX graph contract, not a Turbopack dependency.

## Position

DX should treat Turbopack as a research reference for graph-shaped incremental work, not as the center of the ecosystem. The useful ideas are:

- demand-driven module graph thinking for route/component surfaces,
- explicit dependency edges instead of hidden rebuild heuristics,
- cache and invalidation receipts that explain why work is stale,
- asset/module/reference separation,
- CLI-readable graph output for tools and future editor previews.

The public DX center remains Forge, dx-style, dx-check, DX WWW, source-owned files, and receipts.

## Upstream Provenance

Sources inspected:

- https://nextjs.org/docs/app/api-reference/turbopack
- https://github.com/vercel/next.js/tree/canary/turbopack
- https://github.com/vercel/next.js/blob/canary/turbopack/crates/turbopack-core/src/module_graph/mod.rs
- https://github.com/vercel/next.js/blob/canary/turbopack/crates/turbopack-core/src/module.rs
- https://github.com/vercel/next.js/blob/canary/turbopack/crates/turbopack-core/src/reference/mod.rs
- https://github.com/vercel/next.js/blob/canary/license.md

License: MIT, Vercel, Inc. No upstream code was copied into DX. The current implementation is a small source-owned scanner/resolver/receipt writer shaped by the concepts above.

## Contract Names

- `dx.build.graph`: top-level receipt for nodes, edges, invalidation, provenance, and consumer hints.
- `dx.www.moduleGraph`: route/component module nodes that DX WWW, DX CLI, and future Zed preview can inspect.
- `dx.forge.sourceGraph`: Forge package/source-surface nodes that own app files and receipts.

No `.v1` suffix is used in the public names. The receipt carries a numeric `format` field for future schema evolution.

## Node Kinds

- `tsx-route`: app/pages route files such as `app/page.tsx`.
- `tsx-component`: source-owned component files such as `components/LaunchPanel.tsx`.
- `dx-style-css`: generated or stable dx-style CSS, including `styles/*.generated.css`.
- `forge-surface`: package surfaces from `.dx/forge/source-manifest.json`.
- `dx-check-receipt`: check receipts under `.dx/receipts/check`.
- `public-asset`: static assets under `public`.
- `deploy-output`: deploy/build output receipts under `.dx/deploy`, `.dx/build`, or `out`.

## Edge Kinds

- `imports`: route/component source imports another source, CSS, or receipt-resolved file.
- `owns-source`: Forge surface owns one source-owned file.
- `expects-receipt`: Forge surface expects a receipt node.
- `checks`: dx-check receipt covers source nodes.
- `emitted-from`: deploy output depends on source or asset nodes.

Edges point from dependent to dependency. Invalidation walks the reverse direction from changed nodes.

## Receipt Shape

The first slice writes JSON with:

- `schema: "dx.build.graph"`,
- `names` for `dx.build.graph`, `dx.www.moduleGraph`, and `dx.forge.sourceGraph`,
- `positioning.turbopackPublicDependency: false`,
- sorted `graph.nodes` and `graph.edges`,
- `invalidation.changedNodeIds`,
- `invalidation.affectedNodeIds` for every dependent receipt/source/deploy node,
- `invalidation.rebuildNodeIds` for source rebuild work only,
- upstream `provenance`,
- consumer hints for DX CLI, DX WWW, and future Zed preview.

## Implementation Slice

Implemented source files:

- `tools/build-graph/types.ts`
- `tools/build-graph/scanner.ts`
- `tools/build-graph/resolver.ts`
- `tools/build-graph/receipt.ts`
- `tools/build-graph/reader.ts`
- `tools/build-graph/index.js`
- `tools/build-graph/dx-graph.ts`

Fixture and proof:

- `benchmarks/fixtures/build-graph/minimal-app`
- `benchmarks/dx-build-graph-receipt.test.ts`

The fixture proves that changing `styles/generated.css` invalidates the CSS node, the importing component, the route, and the dependent Forge/check/deploy receipt nodes without running a full build.

`reader.ts` adds the consumer-facing read path. It validates `dx.build.graph` receipts and emits a compact `dx.build.graph.consumerSnapshot` summary with graph counts, node-kind counts, invalidation counts, contract names, provenance, and stable consumer pointers for DX CLI, DX WWW, and future Zed preview.

DX-WWW's Rust source build path now emits the same consumer concept at `.dx/receipts/graph/consumer-snapshot.json` during `dx build`. That source-build graph includes route-shell chunks, linked source-module chunks, and `imports-source-module` edges, then points `dx.build.zedHandoff` at the compact snapshot so Zed can render graph status without reparsing the full receipt.

The compact snapshot also carries `graph.styleOptimization`, an aggregate of `dx-style-css` node metadata. DX CLI, DX-WWW, and Zed can show style node count, original/retained/pruned rule totals, and minified style count without loading every graph node or treating Turbopack as public infrastructure.

The Zed handoff mirrors the same source truth as `style_optimization` in `.dx/receipts/build/zed-handoff.json`. That keeps future editor preview code on a small, stable receipt surface while the full `dx.build.graph` remains available for deeper diagnostics.

## Lane 2 Turbo Tasks Adapter

Lane 2 adds `turboTasksAdapter` to the `dx.build.graph` receipt and compacts it into `dx.build.graph.consumerSnapshot`. This is an adapter plan and receipt surface, not a public architecture switch to `turbo-tasks`, Node, NAPI, Turborepo, or `node_modules`.

Local vendor files studied:

- `vendor/next-rust/turbopack/crates/turbo-tasks/src/task/task_input.rs`
- `vendor/next-rust/turbopack/crates/turbo-tasks/src/lib.rs`
- `vendor/next-rust/turbopack/crates/turbo-tasks-backend/src/backend/operation/update_cell.rs`
- `vendor/next-rust/turbopack/crates/turbo-tasks-backend/src/backend/operation/invalidate.rs`
- `vendor/next-rust/turbopack/crates/turbo-tasks-fs/src/invalidation.rs`
- `vendor/next-rust/turbopack/crates/turbo-persistence/src/lib.rs`

Mapped concepts:

- `TaskInput` becomes a source-owned `tasks[].inputKey` receipt field.
- Cell comparison invalidation becomes `invalidation.strategy: "reverse-dependency-dirty-propagation"` over DX graph edges.
- The parallel scheduler idea becomes `parallelism.executionLevels`, stable topological levels that a future DX scheduler can execute without exposing Turbopack.
- Persistent task cache ideas become `persistence.mode: "source-receipt-plan"` with `.dx/cache/build-graph/tasks` as the planned DX namespace. Each planned task records a SHA-256 `inputFingerprint` derived from the source node, content hash, and dependency node ids, and the compact snapshot records an aggregate adapter fingerprint. No Turbo Persistence database is opened by this slice.

The adapter only plans rebuild work for `invalidation.rebuildNodeIds`; it does not claim full task runtime parity, full persistent cache parity, or upstream scheduler parity. Public consumers should read the compact summary from `dx.build.graph.consumerSnapshot.turboTasksAdapter` unless they need the full per-task receipt.

`diffTurboTasksAdapterPlans(previous, current)` compares two adapter receipts and emits `dx.build.graph.turboTasksAdapterDiff`, including changed task input fingerprints, added/removed tasks, dependent stale task ids, and `summary` counts with `recommendedAction`. This is a small source-owned staleness report for future DX cache decisions; it does not open Turbo Persistence, execute a scheduler, or make `turbo-tasks` public architecture.

The lower JS graph CLI exposes the same adapter-boundary report with `--diff-against <receipt>`, comparing a saved `dx.build.graph` receipt to the current source scan and returning the diff JSON. `--write-diff <path>` persists that same `dx.build.graph.turboTasksAdapterDiff` payload for future DX/Zed consumers, and `--diff-summary <path>` reads a persisted diff receipt back as the compact consumer summary. This is receipt inspection only; it does not write a cache database or run a task scheduler.

`readTurboTasksAdapterDiffReceipt(path)` validates persisted diff receipts, and `createTurboTasksAdapterDiffConsumerSummary(path)` emits `dx.build.graph.turboTasksAdapterDiff.consumerSummary` with status, task counts, stale task ids, recommended action, and boundary flags. Malformed JSON is reported as an invalid Turbo Tasks adapter diff receipt with the receipt path, not as an anonymous parser failure. The `--diff-summary` CLI path rejects scan, write, diff, and snapshot flags so consumers cannot accidentally mix a read-only summary with a new graph calculation. This gives DX CLI, DX-WWW, and Zed a small summary surface without parsing every task change or adopting Turbo Persistence.

The previous Turbo Tasks executor, execution handoff, and Zed execution panel artifacts are removed from the active graph CLI. DX-WWW is not adopting real Turbopack or Turbo Tasks runtime/build execution; those upstream pieces are reference/provenance only. The allowed Lane 2 surface is now adapter/diff/status evidence that can inform a future DX-owned cache runner without executing upstream scheduler semantics, opening Turbo Persistence, or making Turbopack part of `dx build` or `dx dev`.

`createTurboTasksAdapterStatusFromReceipt(receipt)` emits `dx.build.graph.turboTasksAdapterStatus`, a Lane 2 source health contract with `sourceScore: 100` and an honest lane `score: 95` for reference-only adapter/diff/status evidence. The consumer snapshot exposes this as `turboTasksAdapterStatus`, and the lower CLI can emit it directly with `--turbo-tasks-status --json`.

## Lane 3 Turbopack Core Module Graph Map

Lane 3 adds `coreConceptMap` to `dx.build.graph` and summarizes it into `dx.build.graph.consumerSnapshot.coreConceptMap`. This maps useful `turbopack-core` concepts onto DX-owned graph surfaces without turning Turbopack into the public runtime, resolver, package model, or app model.

Local vendor files studied:

- `vendor/next-rust/turbopack/crates/turbopack-core/src/module_graph/mod.rs`
- `vendor/next-rust/turbopack/crates/turbopack-core/src/module.rs`
- `vendor/next-rust/turbopack/crates/turbopack-core/src/reference/mod.rs`
- `vendor/next-rust/turbopack/crates/turbopack-core/src/asset.rs`
- `vendor/next-rust/turbopack/crates/turbopack-core/src/output.rs`
- `vendor/next-rust/turbopack/crates/turbopack-core/src/chunk/mod.rs`

Mapped concepts:

- `ModuleGraph` informs `dx.build.graph` / `dx.www.moduleGraph` nodes and edges.
- `Module` maps to `tsx-route`, `tsx-component`, and `source-module-chunk` nodes.
- `ModuleReference` maps to `imports`, `imports-source-module`, and `links-entry-module` edges.
- `Asset` maps to `dx-style-css` and `public-asset` nodes while dx-style stays authoritative.
- `OutputAsset` and `ChunkingContext` map to `route-shell-chunk`, `source-module-chunk`, and deploy-output receipt surfaces.
- `SourceOwnedInvalidation` keeps invalidation in `changedNodeIds`, `affectedNodeIds`, and `rebuildNodeIds`.
- `ForgeSourceSurface` keeps Forge package ownership under `dx.forge.sourceGraph` via `forge-surface`, `owns-source`, and `expects-receipt`.

Every mapping carries `nodeModulesRequired: false` and an adapter-boundary statement. This is source-only receipt evidence, not full Turbopack parity, not Next runtime adoption, and not a claim that React/RSC or Node/NAPI are required by DX-WWW.

The concept-map validator blocks broad boundary overclaims, including full Next/Turbopack parity, Turbopack as the public graph model, React/RSC as a required core app model, or Node/NAPI as the default foundation.

`validateTurbopackCoreConceptMap(projectRoot)` is exported from `tools/build-graph`, and the consumer snapshot now exposes `coreConceptMapValidation` beside the compact summary. `createTurbopackCoreConceptMapStatus(projectRoot)` wraps the same validation as `dx.build.graph.turbopackCoreConceptMapStatus`, giving DX CLI, Zed, Studio, and the final coordinator one shared Lane 3 health contract for vendor coverage, explicit source-only adapter-boundary status, and no-`node_modules` proof without treating Turbopack as the public graph model. The Next-Rust merge coordinator registry points its Lane 3 `turbopack-core-map` check at that status contract through `healthContract` metadata, so scorecards can display the intended health surface without scraping benchmark output.

The status-only CLI mode is intentionally not a graph scan path; it rejects changed-path, receipt, diff, and snapshot flags.

## CLI Contract

Root DX CLI source command:

```powershell
dx graph --json --changed styles/generated.css --write .dx/receipts/graph/latest.json
dx graph --consumer-snapshot --json
dx graph --turbo-tasks-status --json
dx graph --diff-summary .dx/receipts/graph/turbo-tasks-diff.json --json
```

Root `dx graph` remains an adapter-boundary forwarding surface when available; graph ownership stays in the lower source-owned DX-WWW implementation below. Lane 2 now covers reference-only graph adapter, diff, status, and snapshot behavior. Turbopack/Turbo Tasks runtime execution is not a DX-WWW goal.

Lower source-owned implementation:

```powershell
node tools/build-graph/dx-graph.ts --project <project> --changed <path> --json
node tools/build-graph/dx-graph.ts --project <project> --changed <path> --consumer-snapshot --json
node tools/build-graph/dx-graph.ts --project <project> --changed <path> --write .dx/receipts/graph/latest.json
node tools/build-graph/dx-graph.ts --core-map-status --json
node tools/build-graph/dx-graph.ts --project <project> --changed <path> --diff-against .dx/receipts/graph/previous.json --json
node tools/build-graph/dx-graph.ts --project <project> --changed <path> --diff-against .dx/receipts/graph/previous.json --write-diff .dx/receipts/graph/turbo-tasks-diff.json --json
node tools/build-graph/dx-graph.ts --diff-summary .dx/receipts/graph/turbo-tasks-diff.json --json
node tools/build-graph/dx-graph.ts --project <project> --changed <path> --turbo-tasks-status --json
```

Current Lane 2 source verification covers the lower JS CLI and direct module APIs only: adapter receipts, adapter diffs, adapter status, and consumer snapshots. Execution receipts, execution handoffs, and Zed execution panels were removed from the active target set. Next action: wire the reference-only adapter/diff/status data into a DX-owned build cache design without turning Turbopack or Turborepo into DX's package/workspace model.
