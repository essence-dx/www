# DX Devtools Framework Integration Plan

## Goal

Connect the existing DX Devtools surface into DX WWW at the framework `dx dev` layer so every project receives the dev-only runtime automatically, while `dx build` remains free of Devtools assets, routes, and production abstractions.

## Safety

- Verified the workspace is on `dev` with existing unrelated dirty files before editing.
- Keep changes scoped to `dx-www` dev/runtime code, the new `cli/devtools` module, and a focused benchmark.
- Do not restore, remove, or rewrite unrelated dirty files.

## Agent Scopes

1. Architecture: map current dev HTML injection, hot reload, diagnostics, route markers, and `dx-devtools` source identity.
2. Runtime Injection: add config/CLI toggles and inject Devtools through the dev framework.
3. Devtools Asset: serve source-owned JS/CSS under `/_dx/devtools/*` without React, npm, Next, Vite, or webpack.
4. Protocol + Data: expose dev-only session, route, diagnostics, source-map, style-preview, and style-apply endpoints.
5. Visual Edit: wire selection, parent chain, computed CSS, box model, breakpoints, preview, and safe exact-range apply.
6. Verification: add focused tests proving dev injection, build cleanliness, dev-only endpoints, hot reload/error overlay preservation, read-only preview, and exact source writes.

## Verification

- Run `node --test benchmarks/dx-devtools-framework-integration.test.ts`.
- Run `cargo fmt --check`.
- Attempt `cargo check -j 1 -p dx-www --no-default-features --features cli --bin dx-www`.
- If Cargo is blocked by unrelated workspace state, report the exact blocker without fabricating missing source files.
