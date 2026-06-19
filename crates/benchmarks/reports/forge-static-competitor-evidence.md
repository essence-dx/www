# Forge Static Competitor Evidence

Generated: 2026-05-18T08:46:13.165Z
Source: `benchmarks/reports/forge-public-route-comparison.json`

This is not a full framework benchmark and does not prove broad framework replacement.
It compares measured DX-WWW public Forge static routes against generous static HTML floors for Astro, Svelte, HTMX, and Next.js.

## Scope

- Community/adoption excluded: `true`
- Competitor builds run: `false`
- Package installs run: `false`
- Created node_modules: `false`
- Content parity: Competitor rows are summary-level static floors for the same public route roles, not byte-identical recreations of the full DX-WWW evidence pages.
- Safe public claim: This fixture checks whether DX-WWW public Forge routes stay reasonably small against generous static HTML floors. It does not prove broad framework replacement.

## Totals

| Framework | Baseline | Kind | Routes | Decoded | Brotli | Note |
| --- | --- | --- | ---: | ---: | ---: | --- |
| DX-WWW | measured-forge-public-routes | measured-static-routes | 7 | 37196 B | 7284 B | Measured public Forge compiler output from forge-public-route-comparison.json. It includes real launch evidence, claims, proof links, and route-specific content. |
| Astro | astro-static-html-floor | static-floor | 7 | 6459 B | 2711 B | A generous Astro-style static HTML floor. This does not run astro build and should not be presented as an Astro framework benchmark. |
| Svelte | svelte-prerendered-static-floor | static-floor | 7 | 6473 B | 2729 B | A generous Svelte prerender/static floor. This excludes the normal Vite/Svelte client bundle and is not a SvelteKit or CSR measurement. |
| HTMX | htmx-static-html-floor | static-floor | 7 | 6445 B | 2725 B | A generous HTMX-style static HTML floor. This excludes htmx.js and server behavior, so it is not a production HTMX app benchmark. |
| Next.js | next-static-export-floor | static-floor | 7 | 6487 B | 2727 B | A generous Next.js static-export floor. This excludes next start, React, RSC, image/font/runtime assets, and is not a production Next.js app benchmark. |

## Route Comparison

| Route | Role | DX-WWW Brotli | Astro floor | Svelte floor | HTMX floor | Next.js floor | Winner |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| /forge | Launch evidence | 1240 B | 383 B | 384 B | 385 B | 383 B | Astro (383 B) |
| /forge/scorecard | Package scorecard | 1040 B | 386 B | 390 B | 389 B | 387 B | Astro (386 B) |
| /forge/ci | CI evidence | 862 B | 385 B | 388 B | 387 B | 386 B | Astro (385 B) |
| /forge/evidence | Evidence index | 1106 B | 391 B | 393 B | 393 B | 391 B | Astro (391 B) |
| /forge/releases | Release history | 855 B | 385 B | 389 B | 388 B | 394 B | Astro (385 B) |
| /forge/changelog | Launch changelog | 911 B | 388 B | 389 B | 390 B | 392 B | Astro (388 B) |
| /forge/adoption | Adoption evidence | 1270 B | 393 B | 396 B | 393 B | 394 B | Astro (393 B) |

## Adoption Route Browser Benchmark

Route: `/forge/adoption`
No package installs: `true`
Competitor builds run: `false`

| Framework | Evidence | Decoded | Brotli | HTTP median | Browser load |
| --- | --- | ---: | ---: | ---: | ---: |
| DX-WWW | measured /forge/adoption | 8465 B | 1270 B | 2.815 ms | 21.1 ms |
| Astro | astro-static-html-floor | 930 B | 393 B | n/a | n/a |
| Svelte | svelte-prerendered-static-floor | 932 B | 396 B | n/a | n/a |
| HTMX | htmx-static-html-floor | 928 B | 393 B | n/a | n/a |
| Next.js | next-static-export-floor | 934 B | 394 B | n/a | n/a |

Static floors can be smaller because they omit adoption report copy, claims manifests, DXPK proof artifacts, source-owned package evidence, and reviewer context.

## Honest Findings

- Minimal static HTML floors can be smaller than DX-WWW when they omit Forge evidence, claims, receipts, proof metadata, and generated public review copy.
- Astro, Svelte, HTMX, and Next.js rows in this report are deliberately generous static floors; they are not live framework builds, dev-server timings, or production app measurements.
- DX-WWW should only claim this public surface is static, measured, source-owned, and no-runtime; broader framework wins require separate real app suites.
- Use real-route and framework scorecard reports for broader comparisons, and keep this fixture scoped to public Forge launch evidence routes.

DX-WWW public Forge routes are verified static/no-runtime compiler outputs. This fixture adds an intentionally conservative competitor floor so launch copy stays honest when a plain static page could be smaller.
