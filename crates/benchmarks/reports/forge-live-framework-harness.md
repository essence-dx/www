# Forge Live Framework Harness

Generated: 2026-05-18T08:46:13.199Z

This report separates deterministic static-floor evidence from opt-in live framework builds.

## Static Floor

- Source: `benchmarks/reports/forge-public-route-comparison.json`
- Routes: `7`
- DX-WWW Brotli: `7284` B
- Competitor builds run for static floor: `false`
- Package installs run for static floor: `false`

## Live Builds

- Enabled: `false`
- Builds run: `false`
- Package installs run: `false`

| Framework | Status | Project | Build command | Reason | Outputs |
| --- | --- | --- | --- | --- | --- |
| Astro | skipped | `G:\WWW\benchmarks\fair-counter\astro` | `npm run build` | Set DX_FORGE_LIVE_FRAMEWORKS=1 to run installed local framework builds. | dist:292686 |
| Svelte | skipped | `G:\WWW\benchmarks\fair-counter\svelte` | `npm run build` | Set DX_FORGE_LIVE_FRAMEWORKS=1 to run installed local framework builds. | dist:47774, build:missing |
| HTMX | skipped | `G:\WWW\benchmarks\fair-counter\htmx` | `none` | Set DX_FORGE_LIVE_FRAMEWORKS=1 to run installed local framework builds. | public:2667 |
| Next.js | skipped | `G:\WWW\benchmarks\fair-counter\next` | `npm run build` | Set DX_FORGE_LIVE_FRAMEWORKS=1 to run installed local framework builds. | .next:4821265, out:missing |

## Separation Contract

- Static-floor rows are deterministic local fixtures and are always generated.
- Live framework rows are opt-in and run only when DX_FORGE_LIVE_FRAMEWORKS=1.
- The live harness never runs npm install, pnpm install, yarn install, bun install, or package manager audit commands.
- Missing node_modules or package scripts produce skipped rows, not fake benchmark numbers.
- Do not merge static-floor and live-build rows into one winner without labeling the evidence type.

## Honest Scope

- This harness proves the comparison workflow, not broad framework superiority.
- Live rows measure only already-installed local baseline projects.
- Static floors remain useful for conservative payload direction when live toolchains are unavailable.
- A production benchmark claim still needs route parity, browser timing, CDN transfer, and interaction evidence.

## Static Competitor Detail

# Forge Static Competitor Evidence

Generated: 2026-05-18T08:46:13.199Z
Source: `benchmarks/reports/forge-public-route-comparison.json`

This is not a full framework benchmark and does not prove broad framework replacement.
It compares measured DX-WWW public Forge static routes against generous static HTML floors for Astro, Svelte, HTMX, and Next.js.

## Scope

- Community/adoption excluded: `true`
- Competitor builds run: `false`
