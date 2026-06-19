# DX WWW Benchmarks

DX WWW is designed for a simple performance target: ship the smallest public
surface a route can use, then serve it through a Rust-owned framework runtime.
Static routes should remain static. Interactive routes should receive only the
micro runtime, island chunk, or adapter boundary they actually need.

## Benchmark Results

The controlled local benchmark set below compares the same tiny route shape
across DX WWW, Astro, Svelte, and Next.js. It is the current source of the
published README performance table.

| Rank | Framework | Runtime / Server Class | Median RPS | Best RPS | p50 Latency | Total Route Bytes |
| ---: | --- | --- | ---: | ---: | ---: | ---: |
| `1` | **DX WWW** | Rust-owned web framework and server runtime | **`2398.95`** | **`2515.26`** | **`6.231 ms`** | **`474`** |
| `2` | Svelte / SvelteKit | JavaScript compiler framework | `1142.21` | `1247.13` | `13.797 ms` | `47787` |
| `3` | Astro | JavaScript static/islands framework | `804.84` | `1155.89` | `17.175 ms` | `2722` |
| `4` | Next.js | JavaScript/React full-stack framework | `696.88` | `912.87` | `22.218 ms` | `636239` |

DX WWW leads this measured set with the smallest captured route payload and the
highest local throughput. The benchmark report was written to
`target/framework-comparison-live.json` during the comparison run. Refresh the
report before publishing exact numbers externally.

## Public Context

External benchmark numbers belong in a separate context column or report. A
published TanStack Start SSR number, a js-framework-benchmark browser result,
and a local DX WWW route measurement are not the same workload. They are useful
industry signals, but they should not be merged into one direct RPS ranking
unless every framework is run on the same hardware, route shape, OS, server
mode, concurrency, and load tool.

Recommended public wording:

```text
DX WWW leads the controlled local benchmark set with the smallest captured
payload and the highest local throughput. Additional hosted/provider benchmark
publication is tracked separately.
```

## Methodology

The local framework comparison measures:

- document response bytes
- linked public asset bytes
- total route payload
- median RPS
- best RPS
- p50 latency

Recommended benchmark discipline:

- Build or serve each framework in its production-style mode.
- Use the same route shape and same visible page behavior.
- Warm each server before measurement.
- Run repeated request batches with fixed concurrency.
- Stop local servers after the run.
- Store the report under `target/` unless the report is an intentional release
  artifact.

## Commands

Focused local comparison:

```powershell
node --test benchmarks\measure-framework-scorecard.ts
```

Runtime throughput checks:

```powershell
node --test benchmarks\dx-runtime-throughput-benchmark.ts
node --test benchmarks\dx-runtime-throughput-orchestrator.ts
```

Release readiness and score evidence:

```powershell
dx check examples/template --json
dx www readiness --json --full
dx www agent-context --json --full
```

Docs and hygiene checks:

```powershell
node --test benchmarks\dx-www-current-status-docs.test.ts
git diff --check
```

## Benchmark Governance

Use precise labels:

- `controlled local benchmark` for same-machine local comparison.
- `published benchmark` for third-party or project-published measurements.
- `browser benchmark` for js-framework-benchmark-style DOM/runtime results.
- `real-world LCP sample` for aggregate field data.
- `hosted/provider benchmark` for deployment-provider comparisons.

Do not collapse those categories into a single universal ranking. DX WWW can
and should lead the performance story with strong evidence, but the evidence
source must travel with the claim.
