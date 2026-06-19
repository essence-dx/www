# Forge Medium Route Comparison

Generated: 2026-05-16T22:04:13.346Z
Fixture route: `/forge/medium`

This medium-route fixture is not a full framework benchmark.
It compares deterministic static route payloads for the same repeated cards, route links, and form fields.

## Fixture Shape

- Repeated cards: `12`
- Form fields: `6`
- Route links: `8`
- Static evidence: `true`
- Content parity: Every framework row renders the same medium-page model: repeated cards, route links, stats, and review form fields.

## Scope

- Competitor builds run: `false`
- Package installs run: `false`
- Created node_modules: `false`
- Browser timings measured: `false`
- Safe public claim: This medium fixture compares deterministic static route payloads for the same content shape. It does not prove broad framework replacement.

## Framework Rows

| Framework | Baseline | Kind | Decoded | Brotli | Proof artifact | Runtime claim |
| --- | --- | --- | ---: | ---: | ---: | --- |
| DX-WWW | dx-www-medium-static-route | generated-static-evidence | 5057 B | 1454 B | 187 B | static first route with DXPK proof artifact, no route runtime |
| Astro | astro-medium-static-floor | static-floor | 4986 B | 1438 B | 0 B | no island hydration included in this fixture |
| Svelte | svelte-medium-prerender-floor | static-floor | 4985 B | 1441 B | 0 B | no Svelte client bundle included in this fixture |
| HTMX | htmx-medium-static-floor | static-floor-with-htmx-attributes | 5660 B | 1511 B | 0 B | HTML carries htmx-style attributes; external htmx runtime bytes are not fetched |
| Next.js | next-medium-static-export-floor | static-floor | 4993 B | 1436 B | 0 B | no React, RSC, router, font, image, or prefetch runtime included |

## Brotli Ranking

1. Next.js: 1436 B
2. Astro: 1438 B
3. Svelte: 1441 B
4. DX-WWW: 1454 B
5. HTMX: 1511 B

## Honest Findings

- This is a medium route fixture, not a real Astro/Svelte/HTMX/Next production build.
- The competitor rows are intentionally generous static floors and exclude common framework runtime, router, devtool, image, font, and hydration overhead.
- DX-WWW should use this evidence to find payload direction, not to claim it beats every framework in every scenario.
- A future browser suite must add real framework builds, real route navigation, hydration or interaction tests, and production CDN transfer measurements.

DX-WWW now has a reproducible medium-route evidence fixture beyond tiny public pages. The next proof should run real framework builds for the same page model.
