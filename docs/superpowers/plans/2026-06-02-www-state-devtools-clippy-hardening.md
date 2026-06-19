# WWW State, Devtools, And Clippy Hardening Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make DX WWW stricter and more production-ready by clearing the dx-www CLI clippy gate, removing misleading React hook compatibility as a public source of truth, and making Devtools inspection/style editing/source operations functional with preview/apply/undo proof.

**Architecture:** Keep WWW's DX-native state model as the only framework-owned state contract. Treat React hooks as foreign adapter syntax unless an explicit migration or adapter boundary owns them. Keep Devtools split into Rust protocol/source operations and small source-owned browser assets.

**Tech Stack:** Rust (`dx-www` CLI/runtime), DX serializer `.sr` and `.machine` receipts, TypeScript `.ts` tests for benchmark contracts, source-owned Devtools JS/CSS assets served by Rust in dev only.

---

## File Ownership

- Clippy core lane owns `dx-www/src/config.rs`, `dx-www/src/router/*`, `dx-www/src/project.rs`, and tiny local fixes in directly related modules.
- Clippy CLI lane owns `dx-www/src/cli/*` warning fixes outside Devtools and state policy.
- State policy lane owns `dx-www/src/cli/app_router_execution/*`, `dx-www/src/cli/readiness.rs`, `dx-www/src/cli/react_migration_plan.rs`, state-related tests, and examples that currently import React hooks as framework behavior.
- Devtools protocol lane owns `dx-www/src/cli/devtools/*`, `dx-www/src/cli/dev_http.rs`, `dx-www/src/cli/dev_response.rs`, and Devtools endpoint tests.
- Devtools runtime lane owns framework-served Devtools assets and styles under `dx-www/src/cli/devtools/*assets*` plus `dx-devtools` reference-only parity checks when needed.
- Verification lane owns `.ts` benchmark tests, docs/agent instruction updates, and no-new-JavaScript hygiene checks.

## Six Lanes

### Lane 1: Core Clippy Cleanup

- [ ] Fix low-risk core warnings without changing behavior: derivable defaults, `strip_prefix`, `&Path` instead of `&PathBuf`, collapsed conditionals, iterator cleanups, and exact integer comparisons.
- [ ] Keep public APIs stable unless a warning requires a mechanical signature improvement.
- [ ] Run `cargo fmt --check`.

### Lane 2: CLI Clippy Cleanup

- [ ] Fix low-risk warnings in CLI modules outside Devtools/state policy.
- [ ] Avoid broad `allow` attributes unless a warning is a documented false positive with a narrow scope.
- [ ] Preserve command output contracts and receipt field names unless tests are updated intentionally.

### Lane 3: DX-Native State Policy

- [ ] Remove public claims that `useState`, `useEffect`, or React hooks are WWW runtime APIs.
- [ ] Replace framework-owned examples/templates with DX-native `state`, `derived`, `effect`, and `action` authoring where supported, or mark external React hooks as adapter-only.
- [ ] Add precise diagnostics for unsupported React hooks in WWW-owned TSX routes.
- [ ] Keep React package-forge examples as third-party adapter material, not WWW runtime truth.

### Lane 4: Devtools Protocol And Source Operations

- [ ] Ensure dev-only endpoints return real session, route, diagnostics, source-map, preview, apply, and undo data.
- [ ] Keep endpoints unavailable from production build/preview.
- [ ] Make style preview non-mutating and style apply mutate only exact known source targets.
- [ ] Persist receipts through existing serializer `.sr` and generated `.machine` contracts.

### Lane 5: Devtools Runtime Workbench

- [ ] Wire inspector selection to real DOM element data: parent chain, computed CSS, box model, breakpoints, and source target.
- [ ] Make controls editable using source-owned themed controls, including text, number, select, color, gradient, and undo surfaces.
- [ ] Keep panels closable and bottom-sheet/side-panel behavior reliable.
- [ ] Avoid React DOM, npm UI packages, Vite, webpack, and external browser UI dependencies.

### Lane 6: Verification, Documentation, And Hygiene

- [ ] Add focused `.ts` benchmark tests for state policy, Devtools functionality, dev-only production exclusion, and no-new-JavaScript script hygiene.
- [ ] Update README/AGENTS/docs to describe the actual state and Devtools behavior without weak or placeholder wording.
- [ ] Run targeted Node tests first, then `cargo check -j 6 -p dx-www --no-default-features --features cli --bin dx-www`, then strict clippy.
- [ ] Create professional commits after coherent batches and sync `features` to `origin`.

## Completion Gate

- [ ] `node --test` focused `.ts` contracts pass.
- [ ] `cargo fmt --check` passes.
- [ ] `cargo check -j 6 -p dx-www --no-default-features --features cli --bin dx-www` passes.
- [ ] `cargo clippy -j 6 -p dx-www --no-default-features --features cli --bin dx-www -- -D warnings` passes.
- [ ] `git diff --check` passes.
- [ ] Conflict marker scan excluding `target`, `node_modules`, `.dx/www/output`, and known conflict-marker fixture files is clean.
- [ ] Major changes are committed with professional messages and pushed to `origin features`.
