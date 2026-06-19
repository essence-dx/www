# DX WWW Production Readiness Analysis

DX WWW is a source-owned web framework built around a Rust-owned server/runtime,
React/Next-familiar TSX authoring, tiny/no-JS output, DX Style, DX Icons, Forge
package governance, devtools, and command-written receipts.

This demo analysis records what the project is proving and how to present it
professionally.

## Performance Position

The current controlled local benchmark set places DX WWW first for the measured
tiny route:

| Framework | Median RPS | Best RPS | p50 Latency | Total Route Bytes |
| --- | ---: | ---: | ---: | ---: |
| **DX WWW** | **`2398.95`** | **`2515.26`** | **`6.231 ms`** | **`474`** |
| Svelte / SvelteKit | `1142.21` | `1247.13` | `13.797 ms` | `47787` |
| Astro | `804.84` | `1155.89` | `17.175 ms` | `2722` |
| Next.js | `696.88` | `912.87` | `22.218 ms` | `636239` |

The latest local development run also reported `6000+` RPS for WWW. Treat that
as a strong local performance signal until a release-facing benchmark receipt is
published.

## What WWW Proves

- A Rust-owned web framework can keep the public route payload extremely small.
- React/Next-shaped TSX authoring can lower into source-owned runtime behavior
  without hiding React DOM behind the starter.
- Static/no-JS routes can stay static.
- Interactive routes can receive a micro runtime, client island, or explicit
  adapter boundary only when needed.
- `.dx/*` receipts make framework checks, generated CSS, icons, packages,
  runtime proof, and benchmark evidence inspectable.

## Where WWW Leads

- **Payload discipline:** the captured benchmark route is `474` bytes.
- **Throughput:** WWW leads the controlled local comparison against Astro,
  Svelte, and Next.js.
- **Runtime ownership:** the framework owns the server/runtime path instead of
  depending on an opaque template-local runtime bundle.
- **Developer surface:** TSX, App Router-shaped routes, React-style events,
  dx-style, icons, stores, islands, and devtools sit under one DX contract.
- **Proof culture:** checks and receipts are command-owned, not hand-written
  marketing claims.

## Scope Boundaries

WWW does not need to claim full React or Next.js internals to win its category.
The product direction is stronger:

- React/Next-familiar authoring.
- DX-native state, events, effects, actions, stores, motion metadata, and
  islands.
- Explicit adapter boundaries for React, Svelte, or other framework runtimes
  when a project opts into them.
- Clear diagnostics when unsupported React/Next runtime behavior appears.

## Public Wording

Use this style in README, docs, and launch copy:

```text
DX WWW leads the controlled local benchmark set with the smallest captured
payload and highest local throughput. It combines React/Next-familiar authoring
with a Rust-owned framework runtime, tiny/no-JS output, source-owned styling,
icons, packages, devtools, and receipts.
```

Avoid casual status labels, insult-style section titles, and unverifiable
superlatives without the benchmark source beside them.

## Next Proof Targets

- Refresh the release-facing benchmark receipt.
- Publish hosted/provider benchmark runs.
- Add JS-disabled and JS-enabled browser receipts for the starter.
- Expand route shapes beyond the tiny counter route: static content, islands,
  forms/actions, image-heavy pages, dashboard pages, and route handlers.
- Keep agent instructions aligned with the public framework contract.
