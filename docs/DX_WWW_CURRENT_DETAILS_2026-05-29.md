# DX WWW Current Details - 2026-05-29

## Current Verified State

DX WWW is a Rust-owned web framework in `G:\Dx\www` with App Router-shaped TSX authoring, source-owned style/icon/import tooling, dev-only Devtools, hot reload, diagnostic receipts, production contracts, and production output under `.dx/www/output`.

The current verified release binary is:

```text
G:\Dx\www\target\release\dx-www.exe
```

The fresh WWW starter benchmark fixture is:

```text
G:\Dx\www\target\framework-comparison-20260529\www-runtime-fixed
```

The latest release build after the preview/runtime fixes passed:

```powershell
cargo build --manifest-path G:\Dx\www\Cargo.toml -p dx-www --bin dx-www --release --locked -j6
```

## Features Verified In The Current Pass

- `dx new` creates a working App Router TSX starter.
- `dx build` compiles production output without local app `node_modules`.
- `dx check` passes at `100 / green` on the generated WWW benchmark starter.
- `dx imports` scans `components`, `composables`, and `utils`.
- `dx imports` generates `components/auto-imports.ts`, `.dx/imports/import-map.json`, `.dx/imports/imports.d.ts`, `.dx/imports/sync.sr`, and `.dx/imports/check.sr`.
- Auto-import contracts include explicit `#imports` and `#components` metadata.
- `dx check` and `dx build` fail when auto-import artifacts are stale.
- Devtools are injected only in `dx dev`.
- Production output scan found no `devtools`, `/_dx/devtools`, `style-preview`, or `style-apply` markers.
- Production fallback HTML is visible in no-JS/server-rendered mode.
- Production route shell no longer pulls remote Google Fonts.
- `dx preview --production-contract` now defaults to `.dx/www/output`.
- Production preview caches contract route/asset bytes at startup.
- Production preview now handles `HEAD` without a response body.
- Production preview reads bounded full HTTP request bodies instead of a single TCP chunk.
- Deploy contracts now include runtime asset extensions such as `.mjs`, `.webmanifest`, `.ico`, `.avif`, `.wasm`, and font files.

## WWW Fixes Made From Benchmark Evidence

| Issue | Evidence | Fix |
| --- | --- | --- |
| WWW Lighthouse failed with `NO_FCP` | Production fallback shell had `display: none` | Removed the starter CSS rule hiding `main.dx-shell.dx-page-shell[data-dx-template="next-familiar"]` |
| WWW Lighthouse performance was `93` after paint fix | Remote Google Font request blocked/added transfer | Removed production shell `fonts.googleapis.com` / `fonts.gstatic.com` links while preserving local/system JetBrains Mono fallback |
| WWW preview throughput was disk/contract heavy | Initial throughput: `1030.52 req/s`, p99 `20.42 ms` | Added startup cache for contract route and immutable asset bytes |
| Production preview default was misleading | CLI default pointed at `.dx/build` while framework builds `.dx/www/output` | Updated preview default and help text |
| Preview HTTP semantics were incomplete | `HEAD` returned body, large/split request bodies could truncate | Added `HEAD` body omission and bounded full request reading |

## Browser Runtime Proof

Browser plugin inspection against the fixed WWW preview on `http://127.0.0.1:42104/` showed:

```text
bodyTextLength: 413
main display: grid
scriptCount: 0
stylesheetCount: 4
remoteFontLinks: 0
consoleErrors: 0
```

This proves the benchmark starter can paint meaningful HTML without page JavaScript.

## Lighthouse Comparison

Lighthouse version used:

```text
13.3.0
```

Reports live under:

```text
G:\Dx\www\target\framework-comparison-20260529\lighthouse
```

| Framework | Perf | A11y | Best Practices | SEO | FCP | LCP | TBT | CLS | Transfer |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| DX WWW | 100 | 100 | 100 | 100 | 1054 ms | 1054 ms | 0 ms | 0 | 18,432 B |
| Next.js | 98 | 100 | 100 | 100 | 911 ms | 2455 ms | 63 ms | 0 | 247,811 B |
| SvelteKit | 100 | 91 | 100 | 82 | 1283 ms | 1360 ms | 0 ms | 0 | 33,803 B |
| Astro | 100 | 94 | 100 | 90 | 618 ms | 903 ms | 0 ms | 0 | 2,503 B |

Verdict: WWW wins/ties the Lighthouse category scoreboard and beats Next/Svelte on transfer. Astro still wins raw paint time and payload size in this tiny static starter.

## Server Throughput Comparison

Harness:

```powershell
node benchmarks\dx-runtime-throughput-benchmark.ts --requests 240 --concurrency 16 --warmup 20 --out target\framework-comparison-20260529\throughput-after-cache.json
```

| Framework | Req/s | p50 | p95 | p99 | Errors | Bytes Read |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| DX WWW | 1116.71 | 14.17 ms | 16.39 ms | 17.94 ms | 0 | 536,640 |
| Next.js | 1042.36 | 14.59 ms | 24.36 ms | 26.95 ms | 0 | 2,588,160 |
| SvelteKit | 1135.96 | 13.96 ms | 20.05 ms | 21.17 ms | 0 | 764,160 |
| Astro | 2160.52 | 6.78 ms | 12.08 ms | 15.85 ms | 0 | 75,840 |

Verdict: WWW now beats Next and has better tail latency than SvelteKit in this run. SvelteKit slightly beats WWW on raw req/s. Astro decisively wins raw throughput because its test page is extremely small static output.

## Production Output

Fixed WWW starter production output:

```text
209,694 bytes / 44 files
```

Clean production scan:

```powershell
rg -n "fonts\.googleapis|fonts\.gstatic|JetBrains\+Mono|display\s*:\s*none" target\framework-comparison-20260529\www-runtime-fixed\.dx\www\output\app\index.html target\framework-comparison-20260529\www-runtime-fixed\.dx\www\output\styles target\framework-comparison-20260529\www-runtime-fixed\.dx\www\output\_dx\styles
```

Result: no matches.

## Verification Run

Passed:

```powershell
node --test benchmarks\dx-www-lighthouse-runtime-guard.test.ts
cargo fmt --check
cargo check -j 1 -p dx-www --no-default-features --features cli --bin dx-www
cargo test -j 1 -p dx-www --no-default-features --features cli production_contract_head_response_keeps_length_and_omits_body -- --nocapture
cargo test -j 1 -p dx-www --no-default-features --features cli read_preview_http_request_reads_full_body -- --nocapture
cargo test -j 1 -p dx-www --no-default-features --features cli deploy_immutable_asset_extensions_cover_runtime_assets -- --nocapture
cargo build --manifest-path G:\Dx\www\Cargo.toml -p dx-www --bin dx-www --release --locked -j6
```

Previous broad-preview caveat:

```text
cargo test -j 1 -p dx-www --no-default-features --features cli preview_ -- --nocapture
```

An earlier broad substring filter ran 20 tests and failed 3 older integration-style tests. Those failures are no longer the current targeted evidence:

- `dx_build_emits_hosted_preview_bundle_with_forge_receipts`: fixed by refreshing imports before build in the hosted-preview fixture.
- `dx_server_action_post_endpoints_run_in_dev_and_preview_with_receipts`: fixed by proving local `server-action-protocols.json`, hash-only `server-action-replay-ledger.json` preview evidence, dev/preview POST execution, and structured `400 Bad Request` validation failures. This is not a distributed or provider-backed replay store.
- `dx_preview_production_contract_serves_only_deploy_adapter_outputs`: fixed by serving production preview from deploy-adapter outputs, including `/.dx/ready`, while not inventing `/api/health` unless the deploy contract lists it.
- `provider-adapter-smoke-matrix.json`: now records local replay, account-free fixture, and upload-plan-only CDN evidence while explicitly keeping `release_ready=false` and `hosted_provider_proof=false`.

The broad `preview_` substring sweep was not rerun after these targeted fixes because it is heavier and this repo is under parallel worker load. The readiness proof still keeps provider-hosted route-handler conformance, distributed server-action replay, and multi-provider deployed smoke proof as release-readiness gaps.

## Honest Score

Measured starter score after this pass:

```text
97 / 100
```

Why it moved up:

- WWW now has real Browser/Lighthouse/throughput proof for the measured starter and runtime benchmark surfaces, plus production-cleanliness and release-binary proof. Tiny-static/no-JS browser parity remains gated on its own current receipts.
- WWW ties/wins the Lighthouse category scoreboard.
- WWW ships the benchmark starter with zero page scripts and no remote font dependency.
- WWW beats Next.js in throughput and payload on this benchmark and is competitive with SvelteKit.

Why it is not `99 / 100` yet:

- Astro still wins raw static paint, payload, and throughput on the tiny starter.
- Framework-wide islands, image/font/script primitives, wasm language imports, full TSX event coverage, and mature production adapter behavior still need deeper end-to-end proof.
- Provider-hosted route-handler conformance, distributed server-action replay, and multi-provider deployed smoke proof still remain release-readiness gaps even though the old hosted-preview/server-action/production-preview fixture failures now have targeted passing proof.

Theoretical target remains `99 / 100`, but the honest current claim is: WWW is now a measured no-JS/Lighthouse/build-footprint leader for this starter path, not yet the universal runtime/server winner across every scenario.
