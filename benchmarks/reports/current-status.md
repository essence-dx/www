# DX-WWW Current Status vs Next.js 16.2.6

Generated: 2026-05-15T11:25:38.627Z

> Historical benchmark snapshot, not the current release-readiness score. The
> current 30-agent worker checkpoint is 84/100 on 2026-05-24 with 62 Rust
> warnings, generated artifact curation, and missing browser/overlay/full-clippy proof still blocking 95+.

## What Passed

- DX-WWW Rust workspace: `cargo check --workspace` passed, with warnings.
- DX-WWW browser WASM runtime: `cargo check -p dx-www-browser --target wasm32-unknown-unknown` passed.
- Next baseline: `npm run build` passed on Next.js 16.2.6 / React 19.2.4.
- Next latest version was verified against the npm registry as `16.2.6`.
- Next audit currently reports a moderate PostCSS advisory through the Next dependency tree.

## Live Payload Comparison

| Target | Resources Counted | Raw decoded bytes | gzip estimate | Brotli estimate |
| --- | ---: | ---: | ---: | ---: |
| DX-WWW demo | 2 | 6,262 B | 2,434 B | 1,980 B |
| Next.js baseline | 11 | 714,759 B | 251,853 B | 225,721 B |

Raw payload ratio: Next is 114.14x larger for this first route.
Compressed ratio estimate: Next is 103.47x larger with gzip and 114x larger with Brotli.

## Local Timing

| Check | Median | p95 | Samples |
| --- | ---: | ---: | ---: |
| DX index.html | 2.563 ms | 4.85 ms | 25 |
| DX wasm | 2.545 ms | 3.358 ms | 25 |
| Next / HTML | 3.594 ms | 5.919 ms | 25 |
| DX HTML + WASM sequential | 5.918 ms | 8.14 ms | 15 |
| Next HTML + static assets parallel | 23.149 ms | 29.842 ms | 10 |

## Honest Verdict

DX-WWW is dramatically smaller than a current minimal Next.js app in this demo. That is real.

It is not yet better than Next.js as a framework. The current demo still uses inline JavaScript for the visible counter, and the WASM currently proves loading/exports more than full app rendering, routing, data, auth, deploy behavior, or ecosystem compatibility.

The strongest validated advantage is payload size. The biggest unvalidated claims are framework completeness, real-world app ergonomics, production routing/data semantics, browser API coverage, update/security workflow, and developer adoption.
