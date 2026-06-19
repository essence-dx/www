# DX Build Source Engine Completion Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Connect `dx build` to DX-WWW, CLI/TUI, Zed, and graph receipts with a source-owned route-shell build slice, first real runtime-transform boundaries, and local CSS import flattening.

**Architecture:** Keep the Rust source engine split by responsibility: discovery, graph inputs, TSX adapter, CSS adapter, local CSS import flattening, route-shell output, linked source-module discovery, helper/leaf runtime transforms, ecosystem receipt writing, and CLI command integration. The slice emits executable fallback-shell ESM chunks, transformed standalone helper ESM, simple TSX leaf component ESM, flattened local token CSS, and canonical receipt aliases while explicitly leaving imported/nested TSX graph execution, full JSX hydration, Rolldown parity, and CSS tree-shaking/minification as governed follow-ups.

**Tech Stack:** Rust, `dx_compiler::delivery`, serde JSON receipts, BLAKE3 content hashing, targeted Cargo tests.

---

### Task 1: Route-Shell Output Adapter

**Files:**
- Create: `dx-www/src/build/source_engine/route_output.rs`
- Modify: `dx-www/src/build/source_engine/mod.rs`
- Modify: `dx-www/src/build/source_engine/graph.rs`
- Test: `dx-www/tests/source_build_engine.rs`

- [x] **Step 1: Write the failing test**

Add a test that builds `app/page.tsx`, `components/Hero.tsx`, `styles/app.css`, and `public/icons/mark.svg`, then asserts that a route output exists with `index.html`, `index.dxpk`, `page-graph.json`, `route-unit.json`, and an executable `.mjs` shell chunk exposing `mount()`.

- [x] **Step 2: Run test to verify it fails**

Run: `cargo test -q -p dx-www --no-default-features --features cli source_build_engine_emits_ecosystem_receipts_and_route_shell_chunk --test source_build_engine`

Expected: FAIL because `SourceBuildReport` has no `route_outputs` field and no route shell writer exists.

- [x] **Step 3: Implement the route-shell output adapter**

Create `route_output.rs` that converts each discovered route into `DxReactAppRouteInput`, compiles it with `compile_react_app_route`, writes route HTML/packet/graph/unit artifacts, and writes an executable ESM shell chunk that mounts the compiler-owned fallback HTML without claiming full React hydration.

- [x] **Step 4: Wire route outputs into manifest and receipt**

Add `SourceBuildRouteOutput` to the manifest, report, and receipt summary. Add a `dx-source-route-shell-adapter` receipt adapter with status `emits-executable-fallback-shell-chunks`.

- [x] **Step 5: Run test to verify it passes**

Run: `cargo test -q -p dx-www --no-default-features --features cli source_build_engine_emits_ecosystem_receipts_and_route_shell_chunk --test source_build_engine`

Expected: PASS.

### Task 2: Ecosystem Receipt Writer

**Files:**
- Create: `dx-www/src/build/source_engine/ecosystem.rs`
- Modify: `dx-www/src/build/source_engine/mod.rs`
- Test: `dx-www/tests/source_build_engine.rs`

- [x] **Step 1: Extend the failing test**

Assert that `.dx/receipts/build/latest.json`, `.dx/receipts/graph/latest.json`, and `.dx/receipts/build/zed-handoff.json` are written during the source build.

- [x] **Step 2: Run test to verify it fails**

Run: `cargo test -q -p dx-www --no-default-features --features cli source_build_engine_emits_ecosystem_receipts_and_route_shell_chunk --test source_build_engine`

Expected: FAIL because canonical ecosystem receipt aliases do not exist yet.

- [x] **Step 3: Implement ecosystem receipt output**

Create `ecosystem.rs` that writes the canonical build receipt alias, a `dx.build.graph` JSON receipt with `dx.www.moduleGraph` and `dx.forge.sourceGraph` names, and a small Zed handoff receipt pointing at the build manifest, source receipt, graph receipt, and route shell chunks.

- [x] **Step 4: Wire output paths into `SourceBuildReport`**

Expose `canonical_receipt_path`, `graph_receipt_path`, and `zed_handoff_path` so CLI/tests can consume the stable DX ecosystem artifacts.

- [x] **Step 5: Run test to verify it passes**

Run: `cargo test -q -p dx-www --no-default-features --features cli source_build_engine_emits_ecosystem_receipts_and_route_shell_chunk --test source_build_engine`

Expected: PASS.

### Task 3: CLI and Status Wiring

**Files:**
- Modify: `dx-www/src/cli/mod.rs`
- Modify: `DX.md`
- Modify: `TODO.md`
- Modify: `CHANGELOG.md`
- Modify: `G:/Dx/DX.md`
- Modify: `G:/Dx/TODO.md`

- [x] **Step 1: Add CLI manifest fields**

Record source route output count, graph receipt status, canonical build receipt status, and Zed handoff status in the existing `dx build` manifest JSON.

- [x] **Step 2: Update status docs**

Document the exact implemented state, next action, and current score without overstating full bundler/runtime parity.

- [x] **Step 3: Run targeted verification**

Run:
`cargo test -q -p dx-www --no-default-features --features cli source_build_engine --test source_build_engine`
`rustfmt --edition 2024 --check dx-www/src/build/source_engine/*.rs dx-www/src/build/mod.rs dx-www/src/cli/mod.rs dx-www/tests/source_build_engine.rs`
`git diff --check -- dx-www/src/build/mod.rs dx-www/src/build/source_engine dx-www/src/cli/mod.rs dx-www/tests/source_build_engine.rs DX.md TODO.md CHANGELOG.md`

Expected: targeted test and formatting pass; diff check has no whitespace errors beyond pre-existing line-ending warnings.

### Task 4: Linked Source-Module Graph

**Files:**
- Create: `dx-www/src/build/source_engine/module_linker.rs`
- Create: `dx-www/src/build/source_engine/module_linker_paths.rs`
- Create: `dx-www/src/build/source_engine/module_linker_writer.rs`
- Modify: `dx-www/src/build/source_engine/route_output.rs`
- Modify: `dx-www/src/build/source_engine/graph.rs`
- Modify: `dx-www/src/build/source_engine/ecosystem_graph.rs`
- Modify: `dx-www/src/build/source_engine/ecosystem_handoff.rs`
- Modify: `dx-www/src/build/source_engine/ecosystem.rs`
- Modify: `dx-www/src/cli/mod.rs`
- Test: `dx-www/tests/source_build_engine.rs`

- [x] **Step 1: Extend the failing fixture**

Add nested local imports (`Hero` -> `Badge` plus a local TS helper) and assert that the route shell imports `dxRouteEntryModule`, the manifest records four linked source-module chunks, the graph receipt contains `source-module-chunk` nodes plus `imports-source-module` edges, the compact consumer snapshot is written, and the Zed handoff reports source module chunk count.

- [x] **Step 2: Run test to verify it fails**

Run: `cargo test -q -p dx-www --no-default-features --features cli source_build_engine_emits_ecosystem_receipts_and_route_shell_chunk --test source_build_engine`

Expected: FAIL because the existing shell chunk has no linked route entry module and no source-module graph.

- [x] **Step 3: Implement the source-module linker**

Create `module_linker.rs` to recursively resolve local TSX/JSX/TS/JS imports inside the project root, emit browser-executable metadata `.mjs` chunks, preserve source ownership without `node_modules`, and keep `source_transformed: false` until a real TSX-to-runtime-JS transform lands.

- [x] **Step 4: Connect graph, snapshot, handoff, and CLI fields**

Record source module chunks in route outputs and manifests, add graph nodes/edges, write `.dx/receipts/graph/consumer-snapshot.json`, point the Zed handoff at that snapshot, and expose `source_build_module_chunks` plus graph snapshot emission in `cmd_build`.

- [x] **Step 5: Run test to verify it passes**

Run: `cargo test -q -p dx-www --no-default-features --features cli source_build_engine_emits_ecosystem_receipts_and_route_shell_chunk --test source_build_engine`

Expected: PASS.

### Task 5: Standalone Helper Runtime Transform And TSX Leaf Boundary

**Files:**
- Create: `dx-www/src/build/source_engine/module_runtime_transform.rs`
- Use existing worker file: `dx-www/src/build/source_engine/module_tsx_runtime.rs`
- Modify: `dx-www/src/build/source_engine/module_linker.rs`
- Modify: `dx-www/src/build/source_engine/module_linker_writer.rs`
- Modify: `dx-www/src/build/source_engine/graph.rs`
- Modify: `dx-www/src/build/source_engine/ecosystem_graph.rs`
- Test: `dx-www/tests/source_build_engine.rs`

- [x] **Step 1: Extend the failing fixture**

Assert that the local helper module `lib/formatLabel.ts` records `source_transformed: true`, `transform_kind: "typescript-helper-runtime"`, a `runtime_exports` entry for `formatLabel`, and an emitted chunk containing executable `export function formatLabel(value)` without the TypeScript annotation. Preserve the concurrent worker's red expectation that a simple `Badge.tsx` leaf component records `transform_kind: "tsx-leaf-runtime"` and emits a `dxCreateElement("p", ...)` runtime.

- [x] **Step 2: Run test to verify it fails**

Run: `cargo test -q -p dx-www --no-default-features --features cli source_build_engine_emits_ecosystem_receipts_and_route_shell_chunk --test source_build_engine`

Expected: FAIL because source-module chunks are metadata-only and have no runtime transform metadata.

- [x] **Step 3: Implement the helper transform adapter**

Create `module_runtime_transform.rs` for conservative standalone TS/JS exported helper functions and simple TSX/JSX leaf components. It refuses modules with runtime imports, strips TypeScript parameter/return annotations only from exported function signatures, delegates simple leaf JSX to `module_tsx_runtime.rs`, emits transformed ESM source, and leaves imported/nested TSX graphs `metadata-only` until compiler-owned import rewriting and component-call lowering are implemented.

- [x] **Step 4: Wire runtime metadata into chunks and graph nodes**

Add `transform_kind` and `runtime_exports` to `SourceBuildModuleChunk` with serde defaults, write `dxRuntimeModule` and `dxRuntimeExports` in module chunks, and copy transform metadata into `source-module-chunk` graph nodes for DX CLI/TUI/Zed consumers.

- [x] **Step 5: Run targeted verification**

Run:
`cargo test -q -p dx-www --no-default-features --features cli source_build_engine_emits_ecosystem_receipts_and_route_shell_chunk --test source_build_engine`
`cargo test -q -p dx-www --no-default-features --features cli source_build_engine --test source_build_engine`
`cargo test -q -p dx-www --no-default-features --features cli module_runtime_transform --lib`
`rustfmt --edition 2024 --check <touched Rust/test files>`

Expected: PASS with only the existing unrelated `DX_STYLE_PUBLIC_GLOBALS_FILE` dead-code warning.

### Task 6: Local CSS Import Flattening

**Files:**
- Create: `dx-www/src/build/source_engine/css_imports.rs`
- Modify: `dx-www/src/build/source_engine/css.rs`
- Modify: `dx-www/src/build/source_engine/mod.rs`
- Test: `dx-www/tests/source_build_engine.rs`

- [x] **Step 1: Extend the failing fixture**

Add `styles/app.css` importing `../tokens/theme.css` and assert that generated CSS contains `--dx-accent`, removes the local `@import`, and records `import_count: 0` after flattening.

- [x] **Step 2: Run test to verify it fails**

Run: `cargo test -q -p dx-www --no-default-features --features cli source_build_engine --test source_build_engine`

Expected: FAIL because local CSS imports are retained instead of flattened.

- [x] **Step 3: Implement local import flattening**

Create `css_imports.rs` to flatten only quoted local `@import "<path>";` CSS files that resolve inside the project root. External, absolute, or non-simple imports remain untouched for later professional handling.

- [x] **Step 4: Run targeted verification**

Run:
`cargo test -q -p dx-www --no-default-features --features cli source_build_engine --test source_build_engine`
`rustfmt --edition 2024 --check <touched Rust/test files>`

Expected: PASS with only the existing unrelated `DX_STYLE_PUBLIC_GLOBALS_FILE` dead-code warning.

## Self-Review

Spec coverage: the plan keeps files small, uses a real DX compiler output path, writes stable DX ecosystem receipts and a compact graph consumer snapshot, preserves source-owned/no-node_modules posture, transforms standalone helper exports and simple TSX leaf components into real ESM, flattens local CSS imports, and updates status docs. It does not claim full Rolldown, production minification, imported/nested TSX graph execution, or React hydration parity.

Placeholder scan: no implementation step relies on TBD behavior; the remaining boundary is explicitly stated as a production follow-up.

Type consistency: `SourceBuildRouteOutput`, `SourceBuildModuleChunk`, `transform_kind`, `runtime_exports`, `canonical_receipt_path`, `graph_receipt_path`, `graph_snapshot_path`, and `zed_handoff_path` are introduced in tests first, then implemented in the source engine and surfaced in CLI/graph manifest output.
