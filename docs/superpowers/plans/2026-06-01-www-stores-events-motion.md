# WWW Stores, Event Classes, And Motion Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make WWW treat app-global stores, native-event interaction classes, and dx-style-backed motion as framework-level capabilities while preserving static/no-JS routes by default.

**Architecture:** Keep static routes unchanged unless source uses stores, event classes, or motion. Add compiler-owned metadata in `core/src/delivery`, bind event-string class interactions through the existing generated DOM action binder, and materialize template examples under `examples/template/lib/stores` without hardcoding app-specific state in Rust runtime behavior.

**Tech Stack:** Rust compiler/runtime contracts, source-owned TypeScript template files, MDN `browser-compat-data`, dx-style grouped/animation utilities, Node `.ts` focused tests.

---

### Task 1: Global Store Contract

**Files:**
- Create: `core/src/delivery/global_store.rs`
- Modify: `core/src/delivery/mod.rs`
- Modify: `core/src/delivery/contract.rs`
- Modify: `core/src/delivery/route_unit.rs`
- Test: `benchmarks/www-framework-stores-events-motion.test.ts`

- [ ] Add a parser for `store({ ... })`, `state(...)`, `derived(...)`, `effect(...)`, and `action(...)`.
- [ ] Mark parsed store slots as `DxStateScope::Global`.
- [ ] Keep `useState` lowering as compatibility-only local state.
- [ ] Expose a state-graph read model showing global store counts and source paths.

### Task 2: Template Store Convention

**Files:**
- Create: `examples/template/lib/stores/counter.ts`
- Modify: `examples/template/components/state-runtime-probe.tsx`
- Modify: `dx-www/src/cli/default_template_sources.rs`
- Test: `benchmarks/www-framework-stores-events-motion.test.ts`

- [ ] Add a small source-owned app store under `lib/stores`.
- [ ] Use it in the state runtime probe without requiring Zustand or `node_modules`.
- [ ] Include it in the default template materializer.
- [ ] Keep the route meaningful without JavaScript until interactive store usage is present.

### Task 3: MDN Event Class Interactions

**Files:**
- Modify: `core/src/delivery/route_unit.rs`
- Modify: `dx-www/src/cli/app_router_execution/source_render_parts/static_expression.rs`
- Modify: `dx-www/src/cli/app_router_execution/source_render_parts/client_component.rs`
- Test: `benchmarks/www-framework-stores-events-motion.test.ts`
- Test: `benchmarks/dx-www-native-dom-event-binder-replay.test.ts`

- [ ] Treat `onClick={handler}` and other expression handlers as JS/TS logic.
- [ ] Treat `onClick="scale-up bg-accent"` and other literal string event values as interaction class names.
- [ ] Validate event names through the compiler-owned MDN/native event catalog.
- [ ] Render safe metadata attributes instead of shipping raw `onClick`.
- [ ] Make the generated binder apply class tokens on the event without executing React synthetic events.

### Task 4: dx-style Motion Contract

**Files:**
- Create: `core/src/delivery/motion.rs`
- Modify: `core/src/delivery/mod.rs`
- Modify: `dx-www/src/cli/app_router_execution/source_render_parts/static_expression.rs`
- Test: `benchmarks/www-framework-stores-events-motion.test.ts`

- [ ] Recognize `motion="alias(...)"`, `motion="animate:..."`, and `motion="groupName()"` as source-owned motion declarations.
- [ ] Preserve dx-style grouped class names rather than hardcoding fade/slide presets.
- [ ] Lower motion declarations to `data-dx-motion-*` metadata and class tokens.
- [ ] Keep runtime optional: no motion script unless source asks for motion behavior that needs JS.

### Task 5: MDN Freshness Receipt

**Files:**
- Local data: `target/mdn-browser-compat-data`
- Existing receipt path: `.dx/receipts/readiness/native-events-latest.json`
- Test: `benchmarks/dx-www-readiness-foundation.test.ts`

- [ ] Ensure the local shallow checkout of `mdn/browser-compat-data` exists.
- [ ] Use existing readiness freshness comparison instead of inventing a new JSON format.
- [ ] Report drift honestly if compiler events differ from latest local MDN data.

### Task 6: Proof And Performance Guard

**Files:**
- Test: `benchmarks/www-framework-stores-events-motion.test.ts`
- Existing tests: `benchmarks/dx-www-native-dom-event-binder-replay.test.ts`, `benchmarks/dx-www-readiness-foundation.test.ts`

- [ ] Prove no-JS routes stay no-JS when no state/events/motion are used.
- [ ] Prove event-class interactions only add micro JS on routes that use them.
- [ ] Prove the generated binder uses the MDN catalog and applies classes.
- [ ] Run focused tests, `cargo fmt --check`, `cargo check -j6 -p dx-www --no-default-features --features cli --bin dx-www`, and `git diff --check` before completion.
