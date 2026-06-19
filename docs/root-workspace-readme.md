# DX-WWW Binary Web Workspace

This workspace stages the current DX-WWW binary-web research into one honest product track: source-owned packages, Forge-governed updates, adaptive runtime delivery, DX-Style CSS-first output, benchmark proof, and a future creator/hosting layer.

The goal is not to claim victory from a single demo number. The goal is to build a repeatable system that can beat bloated WordPress, Webflow/Wix-style sites, and average React/Next deployments on payload size, startup cost, deployment simplicity, and source ownership.

## Authoritative Folders

- `Cargo.toml`: current Cargo-facing DX-WWW workspace root. Treat this as the active code path for compiler/runtime/CLI/security work.
- `flow/`: copied Flow reference snapshot. Use it for Forge, serializer, browser-core, traits, and bridge ideas.
- `integrations/`: focused Flow Forge/serializer/bridge references copied for easier review.
- `related-crates/`: Cargo-facing WWW-local support crates used by the flattened workspace; shared serializer now lives at sibling crate `G:\Dx\serializer`.
- `benchmarks/`: fair-counter and binary-web-lab benchmark artifacts and framework baselines.
- `inspirations/`: third-party reference repos for architecture study only. Do not copy code into DX-WWW without a license review and a clear purpose.

## Current Product Direction

DX-WWW should become:

1. A source-owned package system that materializes editable files instead of opaque dependency trees.
2. A Forge-governed update system with receipts, provenance, hashes, and green/yellow/red safety.
3. A binary/static-first web compiler that chooses the smallest correct delivery mode per page.
4. A CSS-first DX-Style pipeline that can beat Tailwind-class workflows with smaller generated CSS before claiming any binary-style browser win.
5. A benchmarked creator/hosting layer that proves real payload, startup, and operational savings.

## Current Verification Baseline

Run from this folder:

```powershell
cargo metadata --no-deps --format-version 1
cargo fmt -p dx-www -p dx-www-compiler -p dx-security --check
cargo fmt --manifest-path ..\serializer\Cargo.toml --check
cargo test -q -p dx-www-compiler ecosystem::forge_security --lib
cargo test -q -p dx-www-compiler ecosystem::forge_registry --lib
cargo test -q -p dx-www-compiler ecosystem::project_check --lib
cargo test -q -p dx-www --no-default-features --features cli cli::tests --lib
cargo check -q -p dx-www --bin dx-www
```

Use targeted checks first. Avoid repeated full workspace builds unless a change touches shared contracts broadly enough to justify it.

## Repository State

`G:\Dx\www` is the flattened DX-WWW git workspace. The original `G:\WWW` folder remains available as the non-destructive migration backup until this layout has passed repeated smoke checks.

The root `.gitignore` intentionally keeps generated build output, local screenshots/logs, external inspiration clones, and the large duplicated icon JSON corpus out of root history.

## Working Rule

Treat demos and benchmarks as evidence only when they use the actual compiler/runtime path and count all required bytes: HTML, CSS, JavaScript, WASM, binary packets, dictionaries, decoders, metadata, and shims.

## Public Claims

- [Forge launch limitations](docs/forge-launch-limitations.md): the current honest public boundary for source-owned packages, npm replacement claims, registry safety, and launch messaging.
- [DX-WWW developer contract](docs/dx-www-developer-contract.md): the React-familiar, source-owned, no-`node_modules` default project shape for strict DX-WWW apps.
- [DX-WWW route/state standard](docs/dx-www-route-state-standard.md): the Route Unit, State Graph, adaptive runtime, and AI-readable source standard for the App Router compiler path.
