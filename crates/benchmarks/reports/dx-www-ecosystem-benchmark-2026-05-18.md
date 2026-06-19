# DX-WWW Ecosystem Benchmark

Generated: 2026-05-18
Workspace root: `G:\WWW`

## Verdict

DX-WWW is not the best all-around web framework today. Next.js plus the npm ecosystem is still the best current shipping ecosystem for breadth, hiring, hosting, docs, integrations, AI tooling, auth, payments, CMS, observability, examples, and production battle testing.

DX-WWW is best in this repo's narrow evidence lane: tiny first-route payloads, adaptive repeated-content packets, no-runtime static Forge routes, and source-owned package safety for curated packages. DX Forge is a serious security wedge, but it is not yet a universal npm replacement.

## Evidence Used

- `benchmarks/reports/framework-scorecard.md`: local framework scorecard. DX-WWW scored `87.7/100` in the benchmark formula, while the report still marks Next.js as today's product-readiness winner.
- `benchmarks/reports/all-size-framework-comparison.md`: DX-WWW measured `657 B` Brotli for the tiny counter route versus `729 B` Astro, `16.05 KB` Svelte, and `158.52 KB` Next.js.
- `benchmarks/reports/forge-static-competitor-evidence.md`: minimal static floors can beat DX-WWW Forge public routes when they omit evidence, claims, receipts, proof metadata, and reviewer context.
- `docs/forge-launch-limitations.md`: Forge v1 is a source-owned package firewall, not a complete npm replacement.
- Official docs checked: Next.js App Router, React Compiler, Astro Islands, SvelteKit, Qwik resumability, Vue, Nuxt, Angular hydration/signals, npm scripts/audit, and Bun lifecycle-script trust.

Current package versions checked with `npm view`:

| Package | Version |
| --- | ---: |
| `next` | 16.2.6 |
| `react` / `react-dom` | 19.2.6 |
| `astro` | 6.3.3 |
| `svelte` | 5.55.7 |
| `@sveltejs/kit` | 2.60.1 |
| `@builder.io/qwik` | 1.19.2 |
| `@qwik.dev/core` | 2.0.0-beta.35 |
| `vue` | 3.5.34 |
| `nuxt` | 4.4.5 |
| `@angular/core` | 21.2.13 |
| `solid-js` / `@solidjs/start` | 1.9.13 / 1.3.2 |
| `react-router` / `@remix-run/react` | 7.15.1 / 2.17.4 |
| `vite` | 8.0.13 |
| `htmx.org` | 2.0.10 |

## Score Meaning

- Shipping score: can a strong team build and operate many real production apps today?
- Security-control score: supply-chain blast-radius reduction, install/update safety, source reviewability, and default script posture.
- Payload/perf score: current evidence for small payloads, startup cost, and rendering model. DX-WWW's score here is based on local repo evidence, not market-wide proof.

## Current Ecosystem Scorecard

| Ecosystem | Shipping score | Security-control | Payload/perf | Demanding judgment |
| --- | ---: | ---: | ---: | --- |
| Next.js + React + npm | 94 | 65 | 68 | Best overall production ecosystem. Heavy default payloads and npm supply-chain exposure are the weak points. |
| React + Vite + npm | 90 | 64 | 74 | Huge ecosystem and flexible tooling. Less full-stack opinion than Next, same npm blast-radius problem. |
| Nuxt + Vue + npm | 88 | 64 | 82 | Excellent full-stack Vue path with Nitro/hybrid rendering. Smaller market than React/Next. |
| SvelteKit + Svelte + npm | 86 | 64 | 88 | Strong compiler story and good app framework. Ecosystem smaller than React/Next. |
| Astro + npm | 86 | 67 | 92 | Best content/static/islands story. Not the universal app/dashboard answer. |
| Angular + npm | 86 | 67 | 76 | Enterprise mature and highly integrated. Heavier, more opinionated, less startup-light. |
| React Router/Remix + npm | 85 | 64 | 80 | Very strong web fundamentals and routing/data model. Less dominant than Next as a total platform. |
| Bun full-stack/package-manager ecosystem | 82 | 80 | 82 | Better default lifecycle-script posture than npm. Still depends on npm package universe for breadth. |
| Qwik/Qwik City | 80 | 70 | 91 | Resumability is philosophically close to DX-WWW's startup thesis. Ecosystem and adoption are the constraint. |
| SolidStart + Solid | 78 | 64 | 88 | Excellent fine-grained performance. Smaller production ecosystem and less platform gravity. |
| HTMX | 76 | 72 | 86 | Tiny client surface and simple mental model. Server architecture carries more responsibility. |
| DX-WWW + DX Forge current | 64 | 88 | 95 | Most interesting security/performance wedge here, but not yet a complete framework ecosystem. |
| DX-WWW + DX Forge target state | 92 | 93 | 96 | Could beat the field only after real app parity, registry breadth, hosting, docs, adapters, devtools, and migration paths exist. |

## Direct DX Forge vs Next.js + npm

| Category | Next.js + npm | DX-WWW + DX Forge current |
| --- | ---: | ---: |
| App capability breadth | 100 | 32 |
| Package ecosystem breadth | 100 | 18 |
| Source ownership and update reviewability | 45 | 95 |
| Install-time script blast-radius control | 55 | 92 |
| First-route payload proof in this repo | 33 | 88 |
| Production maturity | 96 | 48 |
| Overall, security-weighted | 86 | 72 |

## Demanding Read

DX-WWW should not claim "we beat Next.js" as a whole-product statement yet. That would be dishonest.

DX-WWW can claim this:

- The compiler/payload thesis is real enough to keep building.
- Forge's source-owned package model attacks a real npm weakness: opaque dependency trees and install/update blast radius.
- Local evidence shows DX-WWW can beat Next.js, Svelte, Astro, and HTMX on the specific tiny/adaptive payload lanes measured in this repo.
- Public Forge routes are static/no-runtime and evidence-backed, but plain static floors can be smaller when they do not carry proof and review metadata.

DX-WWW must still earn this before it can be called best overall:

- arbitrary npm/package ingestion or a credible curated registry expansion path
- production router/data/auth/form/image/error/loading story
- real app suites beyond synthetic and Forge evidence routes
- deploy adapters, hosting story, logs, previews, cache controls, and rollback
- stable docs, templates, migration paths, and examples
- browser runtime selection with DOM-apply benchmarks
- long-running CI evidence and third-party adoption

If "quickjs" meant QuickJS, it is a JavaScript engine, not a web framework. If it meant Qwik, Qwik is the closest mainstream framework to DX-WWW's no-eager-hydration thesis.
