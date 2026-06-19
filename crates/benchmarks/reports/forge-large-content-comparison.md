# Forge Large Content Comparison

Generated: 2026-05-16T22:10:56.789Z
Fixture route: `/forge/large-content`

This large-content fixture is not a full framework benchmark.
It stresses repeated sections, source-owned package metadata, and first-route payload budget pressure.

## Fixture Shape

- Sections: `8`
- Rows per section: `12`
- Repeated items: `96`
- Source-owned package metadata entries: `3`
- Package files represented: `8`
- Content parity: Every framework row renders the same repeated release sections and the same source-owned package metadata.

## First-Route Payload Budget

- Passed: `true`
- DX-WWW decoded: `28008` B / max `64000` B
- DX-WWW Brotli: `1534` B / max `9000` B
- Package metadata: `505` B / max `3000` B
- Caveat: Budget applies only to this deterministic large static route fixture, not to arbitrary application pages.

## Scope

- Competitor builds run: `false`
- Package installs run: `false`
- Created node_modules: `false`
- Browser timings measured: `false`
- Safe public claim: This large-content fixture checks static payload direction and first-route budget pressure. It does not prove broad framework replacement.

## Framework Rows

| Framework | Baseline | Kind | Decoded | Brotli | Metadata | Runtime claim |
| --- | --- | --- | ---: | ---: | ---: | --- |
| DX-WWW | dx-www-large-static-route | generated-static-evidence | 28008 B | 1534 B | 505 B | static first route with package metadata and no route runtime |
| Astro | astro-large-static-floor | static-floor | 25275 B | 1483 B | 505 B | no Astro island hydration included |
| Svelte | svelte-large-prerender-floor | static-floor | 25278 B | 1486 B | 505 B | no Svelte client runtime included |
| HTMX | htmx-large-static-floor | static-floor-with-htmx-attributes | 28382 B | 1542 B | 505 B | htmx-shaped attributes included; external runtime bytes not fetched |
| Next.js | next-large-static-export-floor | static-floor | 25275 B | 1466 B | 505 B | no React, RSC, router, font, image, or prefetch runtime included |

## Brotli Ranking

1. Next.js: 1466 B
2. Astro: 1483 B
3. Svelte: 1486 B
4. DX-WWW: 1534 B
5. HTMX: 1542 B

## Honest Findings

- Large repeated static content compresses well in every framework row, so small differences here are not a universal framework verdict.
- Competitor rows remain generous static floors and exclude normal framework runtime, hydration, router, image, font, and data-loading overhead.
- DX-WWW should treat this as a first-route budget stress test for package metadata and repeated content, not as proof of market victory.
- The next credible benchmark step is a real build suite that renders this same model in each framework and measures browser navigation.

DX-WWW now has a large-content stress fixture with repeated sections and source-owned package metadata. The budget result is useful only inside this scoped static fixture.
