# Legacy Script Extension Ownership

This file is an audit contract for source-visible `.js`, `.cjs`, and `.mjs`
files. A remaining legacy extension is acceptable only when it is explicitly
owned as one of these reasons: `runtime`, `vendor`, `fixture`, or
`generated-proof`.

The audit treats a trailing slash as a prefix contract and an entry without a
trailing slash as an exact file contract. New `.js`, `.cjs`, or `.mjs` files
outside these rows reopen `legacy-script-extensions`.

Required row format:

```text
- `path-or-prefix`: owner=team or area; reason=runtime|vendor|fixture|generated-proof; migration_gate=condition for converting, deleting, or moving it
```

Owned legacy script surfaces:

- `.dx/template-app-browser-preview/`: owner=browser preview fixture; reason=generated-proof; migration_gate=template app proof no longer needs checked-in generated server/runtime files.
- `benchmarks/fair-counter/`: owner=fair-counter benchmark fixture; reason=fixture; migration_gate=fair-counter fixture moves native config/runtime files behind a generated temp fixture harness.
- `benchmarks/report-snapshot-status.js`: owner=benchmark status reporter; reason=runtime; migration_gate=status reporter is converted to a typed entrypoint or folded into the benchmark runner.
- `browser/js/`: owner=browser crate runtime; reason=runtime; migration_gate=browser runtime owns a TypeScript source and build step for these host scripts.
- `demo/site/site.js`: owner=demo site runtime; reason=runtime; migration_gate=demo site moves to a typed source or generated asset pipeline.
- `dx-devtools/.dx/`: owner=devtools generated-proof contract; reason=generated-proof; migration_gate=devtools proof tests stop reading checked-in `.dx` output.
- `dx-devtools/next.config.mjs`: owner=devtools Next runtime config; reason=runtime; migration_gate=standalone Devtools config moves to a typed or generated config entrypoint.
- `examples/conversion-proof/.dx/vercel-landing/`: owner=conversion proof fixture; reason=generated-proof; migration_gate=conversion proof no longer needs checked-in generated `.dx` launch output.
- `examples/conversion-proof/public/launch-runtime.js`: owner=conversion proof fixture; reason=generated-proof; migration_gate=conversion proof no longer needs checked-in generated public launch runtime.
- `examples/onboard/.dx/`: owner=onboard generated fixture; reason=generated-proof; migration_gate=onboard output proof moves to ignored generated artifacts or golden data.
- `tools/agent-browser.mjs`: owner=agent browser tool; reason=runtime; migration_gate=agent browser command is converted to a typed tool entrypoint.
- `tools/build-graph/index.js`: owner=build graph tool; reason=runtime; migration_gate=build graph command is converted to a typed tool entrypoint.
- `tools/build/`: owner=build readiness tools; reason=runtime; migration_gate=build readiness command surfaces are converted to typed entrypoints.
- `tools/dx-style/`: owner=style diagnostics tools; reason=runtime; migration_gate=style diagnostics command surfaces are converted to typed entrypoints.
- `tools/launch/`: owner=launch runtime tools; reason=runtime; migration_gate=launch command surfaces are converted to typed entrypoints.
- `tools/launch-stabilize/`: owner=launch stabilization tools; reason=runtime; migration_gate=launch stabilization command surfaces are converted to typed entrypoints.
- `tools/next-rust-merge/`: owner=Next/Rust merge tools; reason=runtime; migration_gate=merge command surfaces are converted to typed entrypoints.
- `tools/style/`: owner=style comparison tools; reason=runtime; migration_gate=style comparison command surfaces are converted to typed entrypoints.
- `tools/vendor/`: owner=vendored boundary tools; reason=vendor; migration_gate=vendored boundary scripts are either removed or regenerated from their upstream source.
- `tools/worktree/`: owner=worktree orchestration tools; reason=runtime; migration_gate=worktree command surfaces are converted to typed entrypoints.
- `vendor/`: owner=vendored Next Rust fixtures; reason=vendor; migration_gate=vendored upstream fixtures are removed, regenerated, or moved outside source-visible repo ownership.
