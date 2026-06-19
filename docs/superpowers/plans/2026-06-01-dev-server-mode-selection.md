# DX Dev Server Mode Selection Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Keep the existing Axum dev server, add an automatic tiny dev server path for projects that do not need the Axum stack, and provide `template-axum` plus `template-may-minihttp` examples that prove the selection.

**Architecture:** `dx dev` parses a source-owned `dev.server_mode` setting from the extensionless `dx` file and CLI flags, then resolves `auto` to either Axum or a tiny may-minihttp-style TCP responder. Axum remains the full-feature path for hot reload, devtools, API routes, route handlers, server actions, and middleware-like behavior; the tiny path reuses existing DX request/response conversion and cache helpers so user-owned app sources still render through the normal WWW framework.

**Tech Stack:** Rust CLI/dev server, existing extensionless `dx` parser, focused Node `node:test` source guards, Rust unit tests, `cargo fmt`, and `cargo check -j 6`.

---

### Task 1: Configuration And CLI Contract

**Files:**
- Modify: `G:/Dx/www/dx-www/src/config.rs`
- Modify: `G:/Dx/www/dx-www/src/config_source.rs`
- Modify: `G:/Dx/www/dx-www/src/cli/dev_options.rs`
- Test: `G:/Dx/www/dx-www/src/cli/tests/part_02.rs`

- [ ] Add `DxDevServerMode` with `auto`, `axum`, and `may-minihttp`.
- [ ] Add `DevConfig.server_mode` defaulting to `auto`.
- [ ] Parse `dev.server_mode` and `dev.server` from extensionless `dx`.
- [ ] Parse `--server-mode <auto|axum|may-minihttp>` and `--server <auto|axum|may-minihttp>` for `dx dev`.
- [ ] Reject invalid values with field-specific diagnostics.

### Task 2: Server Mode Resolver

**Files:**
- Create: `G:/Dx/www/dx-www/src/cli/dev_server_mode.rs`
- Modify: `G:/Dx/www/dx-www/src/cli/mod.rs`
- Modify: `G:/Dx/www/dx-www/src/cli/dev_command.rs`

- [ ] Resolve `auto` to Axum whenever hot reload or devtools is enabled.
- [ ] Resolve `auto` to Axum when the project contains API route handlers, route files, server actions, or a server source directory.
- [ ] Resolve `auto` to the tiny path when none of those signals are present.
- [ ] Reject forced `may-minihttp` when hot reload/devtools are enabled, because those features intentionally use the full Axum dev endpoints.
- [ ] Print the selected runtime in `dx dev` startup output.

### Task 3: Tiny Dev Server Path

**Files:**
- Modify: `G:/Dx/www/dx-www/src/cli/dev_wire.rs`
- Create: `G:/Dx/www/dx-www/src/cli/dev_tiny_server.rs`
- Modify: `G:/Dx/www/dx-www/src/cli/dev_command.rs`

- [ ] Make the existing bounded wire reader/cache/serializer helpers available in normal `dev-server` builds.
- [ ] Add a small `TcpListener` loop that uses `read_http_wire_request`, `handle_http_wire_response_cached`, and `dev_wire_response_bytes`.
- [ ] Reuse the same `DxDevParsedResponder` as Axum so template/app rendering stays framework-owned and user source stays user-owned.
- [ ] Keep tiny mode intentionally scoped to simple projects without devtools/hot reload.

### Task 4: Axum Path Overhead Cut

**Files:**
- Modify: `G:/Dx/www/dx-www/src/dev/axum_server.rs`

- [ ] Skip `axum::body::to_bytes` for GET and HEAD requests after built-in static/hot-reload/dev-feedback handling.
- [ ] Preserve full body parsing for POST, PUT, PATCH, DELETE, and route-handler flows.
- [ ] Keep response conversion behavior unchanged.

### Task 5: Example Templates

**Files:**
- Create: `G:/Dx/www/examples/template-axum/**`
- Create: `G:/Dx/www/examples/template-may-minihttp/**`
- Modify: `G:/Dx/www/examples/template/dx`

- [ ] Add `dev(server_mode=auto ...)` to the existing template.
- [ ] Create `template-axum` with `devtools=true`, `hot_reload=true`, and an app/API shape that resolves to Axum.
- [ ] Create `template-may-minihttp` with `devtools=false`, `hot_reload=false`, and static TSX/CSS sources that resolve to the tiny path.
- [ ] Keep template-side source files in `.ts`/`.tsx` for scripts/components; do not add `.js`, `.cjs`, or `.mjs`.
- [ ] Do not add new Rust hardcoded source-text arrays for these templates.

### Task 6: Focused Proof

**Files:**
- Create: `G:/Dx/www/benchmarks/dx-dev-server-mode-selection.test.ts`
- Create: `G:/Dx/www/benchmarks/dx-template-server-modes.test.ts`
- Modify: existing Rust unit tests as needed.

- [ ] Prove source contains a mode resolver independent of template names.
- [ ] Prove `template-axum` and `template-may-minihttp` config to different modes.
- [ ] Prove new template folders have no `.js`, `.cjs`, or `.mjs` files.
- [ ] Prove Rust default-template hardcoded arrays were not expanded with the new template names.
- [ ] Run `node --test` for the new tests.
- [ ] Run `cargo fmt --check`.
- [ ] Run `cargo check -j 6 -p dx-www --no-default-features --features cli --bin dx-www`.

### Completion

- [ ] Review `git diff --check`.
- [ ] Commit with a professional message.
- [ ] Push branch `features`.
