# DX WWW Current Details - 2026-05-30

## Framework Status

DX WWW is now at `99 / 100` local receipt-backed release readiness for the WWW scope. The remaining work is post-release hardening for broader hosted/provider benchmarks, cross-browser breadth, and public benchmark publication.

This file records the current source-owned state and scope. Hosted/provider benchmark publication is tracked separately from the controlled local benchmark set.

## What Is Currently Source-Owned

- Rust-owned CLI, App Router execution, builder, dev server, preview server, route handlers, server-action runtime, DX Style, DX Icons, Forge, and receipt surfaces.
- React/Next-familiar TSX authoring without adopting React DOM as the framework runtime.
- Source-owned runtime behavior for supported state slots, derived reads, effects, actions, DOM events, and client islands. There is no hidden React runtime claim in the current starter.
- Materializable runtime evidence fixtures under `examples/template/proof-routes/`, covering DX-native state/event replay surfaces and camelCase island directives without making `/state-runtime` or `/islands` default starter pages.
- DX-native readiness proof surfaces behind `dx www readiness`.
- Agent handoff through `dx www agent-context --json --full`.
- Readiness receipts and serializer surfaces under `.dx/receipts/readiness/*`, `.dx/receipts/devtools/*`, and `.dx/serializer/*`.
- Auto-import support for `components`, `composables`, `utils`, generated `.d.ts`, `#imports`, `#components`, and stale-map check/build gates.
- Devtools source modules and dev-only protocol work, with visual edit workbench proof still gated by current receipts.

## Readiness Proof Gates In Source

- `tiny-static` and no-JS output contracts are tracked separately from richer route output.
- Public runtime output and `.dx` evidence artifacts are treated as different bundles.
- `/state-runtime` and `/islands` are route-level proof targets only after the proof fixtures are materialized; their presence does not clear browser/provider proof gates.
- Native event catalog and browser binder receipts exist as explicit readiness contracts.
- State runtime browser replay has a receipt contract, freshness status, and stale reasons.
- Server-action replay ledger has a release-readiness receipt contract, serializer receipt, machine path, and local-preview-only hosted-provider boundary.
- Primitive proof covers Image, Font, Script, Wasm, and plain HTML surfaces as source-owned work, not hosted browser proof.
- Proof graph and agent-context output expose local release readiness plus remaining hardening items instead of hiding provider/global caveats.

## Scope Boundaries

- The current `99 / 100` score is scoped to local receipt-backed WWW release readiness; hosted/provider publication remains a separate benchmark gate.
- Local server-action replay is separate from distributed idempotency storage.
- Local preview is separate from provider-hosted validation.
- Source-owned browser replay receipts are separate from live Browser or Chrome proof.
- Production HTTP local replay is a source-owned receipt contract/writer; treat the latest JSON/SR/machine receipt files as current only after `dx www readiness --write-receipts` regenerates them. Browser, TCP preview server, CDN, and hosted-provider proof remain separate gates.
- Browser-receipt import bridge status: source-owned import is now guarded. The TypeScript harness can convert a real page snapshot into import candidates, and `dx www readiness` validates imported real browser JSON before canonical JSON/SR/machine writes. The current import path is explicit: `dx www readiness --import-state-runtime-browser-receipt <browser-receipt.json> --json --full`, `dx www readiness --import-native-event-browser-binder-receipt <browser-receipt.json> --json --full`, `dx www readiness --import-visual-edit-browser-receipt <browser-receipt.json> --json --full`, and `dx www readiness --import-no-js-browser-receipt <browser-receipt.json> --json --full`. The harness and import bridge are covered by `dx-www-readiness-browser-receipt-import.test.ts`, `dx-www-readiness-browser-receipt-harness.test.ts`, and `dx-www-readiness-browser-receipt-harness.ts`; they do not themselves run Browser/Chrome or convert local receipts into hosted or release proof.
- Astro tiny-static parity is proven for the current controlled local `/fair-counter` benchmark; broader route shapes still need separate receipts.
- Static/no-JS proof remains bounded to source/output evidence until JS-disabled browser proof is imported. `tiny-static`, `data-dx-js="none"`, and no-JS artifact receipts are separate from hosted/provider parity proof.
- React compatibility remains bounded. `useState` and related React-like APIs must lower to DX-native semantics or fail with precise diagnostics.

## Latest Focused Verification

The current focused readiness source proof set passed:

```powershell
node --test --test-concurrency=1 .\benchmarks\dx-www-agent-context-command.test.ts .\benchmarks\dx-www-islands-abi-camelcase.test.ts .\benchmarks\dx-www-tiny-static-public-partition-proof.test.ts .\benchmarks\dx-www-native-dom-event-binder-replay.test.ts .\benchmarks\tsx-app-router-state-runtime-operations.test.ts .\benchmarks\server-action-replay-ledger-honesty.test.ts
node --test --test-concurrency=1 .\benchmarks\dx-www-readiness-foundation.test.ts .\benchmarks\dx-runtime-throughput-receipt-contract.test.ts .\benchmarks\dx-www-lighthouse-runtime-guard.test.ts
cargo check -j 1 -p dx-www --no-default-features --features cli --bin dx-www --message-format=short
```

That proof is intentionally scoped. Browser, Lighthouse, release build, hosted provider, and broader runtime benchmark proof remain post-release hardening tracks.

## Next Gates

- Refresh Readiness receipts from a current build once the source changes settle.
- Run JS-disabled and JS-enabled browser proof against the canonical starter.
- Run Lighthouse and throughput across tiny static, island, route-handler, server-action, image-heavy, and dashboard route shapes.
- Prove production output separates public bytes from evidence bundles.
- Prove provider-hosted route handlers and server-action replay behavior.
- Run real Browser/Chrome replays, convert the real page snapshot with the readiness browser receipt harness, and import current browser receipts through the guarded source bridge.
- Keep `dx check` and `dx www agent-context --json --full` aligned with stale reasons and replay commands.
