# Real Route Framework Comparison

Generated: 2026-05-15T19:40:10.201Z

Community/adoption is deliberately excluded.

Chrome headless metrics: enabled.

Samples: 12 HTTP route timings, 7 Chrome page loads per framework/route.

## Overall

| Framework | Real-route score | Stars |
| --- | ---: | --- |
| DX-WWW | 100 | ★★★★★ |
| Astro | 76.7 | ★★★★☆ |
| Svelte | 33.8 | ★★☆☆☆ |
| HTMX | 33.4 | ★★☆☆☆ |
| Next.js | 13.9 | ★☆☆☆☆ |

## Route Scores

| Route | DX-WWW | Astro | Svelte | HTMX | Next.js |
| --- | ---: | ---: | ---: | ---: | ---: |
| Small counter | 100 | 90 | 23.4 | 28.6 | 8.3 |
| Medium docs, 160 sections | 100 | 77.3 | 25.9 | 16.4 | 5 |
| Medium cards, 180 cards + filter | 100 | 62.5 | 32.8 | 30.5 | 12.1 |
| Big dashboard, 1,200 rows + filter | 99.8 | 77 | 53.2 | 58.1 | 30.4 |

## Payload And Timing

### Small counter

| Framework | Brotli | Raw decoded | Resources | HTTP full median | Chrome load | DOM nodes |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| DX-WWW | 657 B | 1.82 KB | 1 | 1.273 ms | 12.8 ms | 23 |
| Astro | 729 B | 2.66 KB | 1 | 1.471 ms | 13.7 ms | 23 |
| Svelte | 16.05 KB | 46.65 KB | 3 | 5.093 ms | 31 ms | 24 |
| HTMX | 15.35 KB | 52.64 KB | 2 | 2.72 ms | 36.8 ms | 24 |
| Next.js | 158.52 KB | 621.33 KB | 9 | 20.185 ms | 72.5 ms | 37 |

### Medium docs, 160 sections

| Framework | Brotli | Raw decoded | Resources | HTTP full median | Chrome load | DOM nodes |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| DX-WWW | 1.27 KB | 39.65 KB | 1 | 0.602 ms | 11.4 ms | 661 |
| Astro | 1.32 KB | 40.96 KB | 1 | 1.136 ms | 13.7 ms | 661 |
| Svelte | 16.05 KB | 46.65 KB | 3 | 2.444 ms | 25.3 ms | 663 |
| HTMX | 15.90 KB | 89.59 KB | 2 | 2.459 ms | 67.9 ms | 662 |
| Next.js | 159.95 KB | 721.54 KB | 8 | 21.808 ms | 101.2 ms | 689 |

### Medium cards, 180 cards + filter

| Framework | Brotli | Raw decoded | Resources | HTTP full median | Chrome load | DOM nodes |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| DX-WWW | 1.62 KB | 40.06 KB | 1 | 0.631 ms | 35.9 ms | 745 |
| Astro | 1.67 KB | 42.78 KB | 1 | 3.257 ms | 50.8 ms | 744 |
| Svelte | 16.05 KB | 46.65 KB | 3 | 3.178 ms | 52.4 ms | 745 |
| HTMX | 16.21 KB | 89.92 KB | 2 | 2.706 ms | 61.6 ms | 745 |
| Next.js | 159.50 KB | 653.77 KB | 9 | 21.892 ms | 110.7 ms | 759 |

### Big dashboard, 1,200 rows + filter

| Framework | Brotli | Raw decoded | Resources | HTTP full median | Chrome load | DOM nodes |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| DX-WWW | 8.37 KB | 190.57 KB | 1 | 1.65 ms | 113.3 ms | 8434 |
| Astro | 8.54 KB | 199.43 KB | 1 | 2.895 ms | 148.5 ms | 8434 |
| Svelte | 16.05 KB | 46.65 KB | 3 | 5.926 ms | 141.4 ms | 8435 |
| HTMX | 23.07 KB | 240.50 KB | 2 | 4.347 ms | 112.6 ms | 8435 |
| Next.js | 164.17 KB | 767.53 KB | 9 | 18.725 ms | 145.7 ms | 8449 |

## Route-Specific Gaps

- Static HTML routes leave less room for DX-WWW to beat Astro by huge margins because both ultimately ship HTML.
- Svelte here is a Vite CSR app, not SvelteKit SSR; payload may look small on content-heavy routes while browser render work moves client-side.
- The big-dashboard full DOM route is intentionally demanding. DX-WWW needs its adaptive/binary route wired into production output to turn the lab win into a real route win.
