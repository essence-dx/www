# Fair Counter Comparison

Generated: 2026-05-15T19:40:35.759Z

## Method

Equivalent minimal counter pages were built for DX-WWW, Next.js, Svelte, Astro, and HTMX. The measurement counts first-route HTML plus same-origin JS/CSS/runtime assets. DX-WWW is served by its real Rust demo route, Next.js uses `next start`, Svelte/Astro use static local servers, and HTMX uses a small local fragment server. DX-WWW uses the micro-JS/no-WASM path for this tiny interaction. Local timings are localhost HTTP sanity checks, not browser Lighthouse or production CDN results.

## Versions

- Next.js 16.2.6 / React 19.2.6
- Svelte 5.55.7 / Vite 8.0.13
- Astro 6.3.3
- HTMX 2.0.10

## Payload And Timing

| Target | Model | Requests | Raw decoded | gzip estimate | Brotli estimate | Full route median |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| DX-WWW | Rust route + static HTML + micro JS | 1 | 1,867 B | 919 B | 657 B | 0.63 ms |
| Next.js | App Router + React client island | 9 | 636,239 B | 187,903 B | 162,325 B | 20.361 ms |
| Svelte | Svelte compiled client bundle | 3 | 47,774 B | 18,437 B | 16,439 B | 3.491 ms |
| Astro | Static HTML + inline script | 1 | 2,722 B | 1,007 B | 729 B | 1.465 ms |
| HTMX | HTML + htmx runtime + server fragment | 2 | 53,905 B | 17,559 B | 15,719 B | 3.923 ms |

## Ratio Against DX-WWW

| Target | Raw | gzip | Brotli | Full route median |
| --- | ---: | ---: | ---: | ---: |
| DX-WWW | 1x | 1x | 1x | 1x |
| Next.js | 340.78x | 204.46x | 247.07x | 32.32x |
| Svelte | 25.59x | 20.06x | 25.02x | 5.54x |
| Astro | 1.46x | 1.1x | 1.11x | 2.33x |
| HTMX | 28.87x | 19.11x | 23.93x | 6.23x |

## Rankings

- Smallest raw payload: DX-WWW -> Astro -> Svelte -> HTMX -> Next.js
- Smallest gzip estimate: DX-WWW -> Astro -> HTMX -> Svelte -> Next.js
- Fastest localhost full-route median: DX-WWW -> Astro -> Svelte -> HTMX -> Next.js

## Notes

- Astro uses static HTML plus a tiny vanilla script, which is idiomatic for this kind of page and is a very strong baseline.
- Svelte compiles the counter into a small client bundle, but it still ships more client JavaScript than the Astro static version.
- HTMX ships the htmx runtime and moves state mutation to the server endpoint, so it is not the same client-state model as DX-WWW, Svelte, or React.
- Next.js ships much more runtime JavaScript for this tiny client island, even after removing Tailwind, fonts, images, and extra demo content.
