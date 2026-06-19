# Framework Scorecard

Generated: 2026-05-18T08:46:13.115Z

> Historical benchmark snapshot, not the current release-readiness score. The
> current 30-agent worker checkpoint is 84/100 on 2026-05-24 with 62 Rust
> warnings, generated artifact curation, and missing browser/overlay/full-clippy proof still blocking 95+.

Community/adoption is deliberately excluded.

## Method

- Small: Actual local route benchmark. DX-WWW uses its Rust demo runtime; Astro/Svelte/HTMX use static local servers; Next.js uses next start.
- Medium: Scale payload/access model over docs and marketing-card page graphs from binary-web-lab. This is compiler evidence, not a full browser Lighthouse run.
- Big: Scale payload/access model over a 1200-row dashboard. DX-WWW uses adaptive viewport delivery for initial load and records full-data packet numbers separately.
- Score: Overall = 25% small actual route + 20% medium scale + 20% big scale + 20% current product readiness + 15% developer experience. Community/adoption is not included.

## Small Actual Route

| Framework | Brotli | Median |
| --- | ---: | ---: |
| DX-WWW | 657 B | 0.63 ms |
| Astro | 729 B | 1.465 ms |
| Svelte | 16.05 KB | 3.491 ms |
| HTMX | 15.35 KB | 3.923 ms |
| Next.js | 158.52 KB | 20.361 ms |

## Medium And Big Scale Models

| Framework | Medium model | Medium Brotli | Big model | Big Brotli |
| --- | --- | ---: | --- | ---: |
| DX-WWW | DX template/data packets for repeated docs and marketing cards | 412 B | adaptive initial viewport packet; full data can stream separately | 199 B |
| Astro | static HTML baseline, Astro-like output shape | 495 B | full static dashboard HTML baseline | 3.82 KB |
| Svelte | static HTML payload plus measured Svelte runtime floor | 16.39 KB | full dashboard payload plus measured Svelte runtime floor | 19.73 KB |
| HTMX | static HTML payload plus measured HTMX runtime floor | 15.69 KB | full dashboard payload plus measured HTMX runtime floor | 19.03 KB |
| Next.js | static HTML payload plus measured Next/React runtime floor | 158.86 KB | full dashboard payload plus measured Next/React runtime floor | 162.20 KB |

## Scores

| Framework | Small | Medium | Big | Readiness | DX | Overall | Stars |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| DX-WWW | 100 | 100 | 100 | 58 | 74 | 87.7 | ★★★★☆ |
| Astro | 66.5 | 44.9 | 3 | 88 | 82 | 56.1 | ★★★☆☆ |
| Svelte | 11 | 4.5 | 1 | 84 | 86 | 33.6 | ★★☆☆☆ |
| Next.js | 2 | 3.8 | 1 | 91 | 89 | 33 | ★★☆☆☆ |
| HTMX | 10.2 | 4.6 | 1 | 76 | 70 | 29.4 | ★☆☆☆☆ |

## Demanding Verdict

- Performance winner: DX-WWW
- Product-readiness winner today: Next.js
- DX-WWW is now winning the measured small route and the adaptive medium/big payload model, but its product-readiness score is intentionally lower because App Router coverage, dx build proof, and the DX-owned dev feedback and diagnostics surface still need production proof.
