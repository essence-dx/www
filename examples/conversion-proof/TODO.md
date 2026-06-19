# DX-WWW Conversion Proof TODO

Status: 99 / 100

## Done

- Added source-backed DX-WWW pages for `/ui`, `/database`, and `/backend`.
- Expanded the `/` launch landing with the current DX-WWW launch feature set: dx-style, dx-forge, dx-check, source-owned package slices, local model runtime, provider onboarding, RLM/serializer receipts, dx-agents, n8n automations, and browser/check visual QA.
- Added the Vercel static export script for the DX-WWW landing so the launch page can be deployed from `.dx/vercel-landing` while the native `.dx/build` format continues to mature.
- Added and deployed the landing favicon through the Vercel static export path so `/favicon.svg` resolves from production.
- Added `pages/index.html` as the source-owned DX-WWW recreation of the correct DX launch landing from `G:\Dx\website`, preserving hero, platform downloads, token/speed/performance, Forge, Traffic Security, Check, media, tools, pricing, benchmarks, testimonials, comparison, and waitlist sections.
- Added `styles/dx-landing.css`, copied thumbnail assets, and registered DX landing manifests, source surfaces, visual audit, receipt, and route discovery metadata.
- Added the launch polish pass for the DX landing: scroll-safe page reset, Vercel-like light/dark/system theme controls, animated icon rail, theme proof section, and Midday-style footer with the required "Enhanced Development Experience" and "I use Dx BTW" launch identity.
- Corrected the launch UX follow-up: moved theme controls from the top nav to the footer, switched the landing typography to JetBrains Mono, replaced the mascot block with a responsive inline SVG circular face, and hardened root/body scroll behavior for the in-app browser.
- Replaced generic download-card placeholders with DX-owned `<dx-icon name="pack:name" />` source syntax, enlarged the circular mascot eyes, reduced the smile, and moved the landing into an explicit full-height scroll container for localhost reliability.
- Added local-only landing polish: dark is now the default theme, the DX custom scrollbar/progress rail is visible, face SVG colors are token-driven, the mascot circle is larger, CTA contrast uses dedicated theme tokens, hero imagery appears above the fold, and scroll-reveal/hover motion is wired for cards and sections.
- Fixed the local Web Preview asset path and CTA link specificity so thumbnail imagery loads from DX-WWW's `/public` static route and the bottom primary action keeps readable dark-on-light contrast.
- Added the DX Icons-labeled download CTA treatment, made the custom scrollbar top-layer/clickable/draggable with token-only transparent thumb styling, and reframed the landing around DX-WWW, dx-style, dx-forge, and dx-check while keeping editor integration out of the current page story.
- Removed the temporary pricing section and changed the benchmark/proof bars to Vercel-theme token gradients instead of random accent colors.
- Added a compact fixed header with logo-adjacent links, removed the hero face mascot for now, added the top 8K YouTube nature video carousel, improved spring-style reveal motion, and cropped the huge footer wordmark at the bottom edge.
- Removed the default DX-WWW wrapper header from the landing route, simplified the DX face to a single token-colored mascot circle with a smaller smile curve, and widened the custom scrollbar thumb for better local Web Preview use.
- Tuned the landing for Lighthouse-style performance by removing the blocking Google Fonts import, replacing hero YouTube iframes with local preview cards that open video only on click, adding stable lazy/async image dimensions, and keeping sections paint-cheap with content visibility.
- Added shared DX-WWW `.tsx` components for the converted route header, source surface table, and runtime boundary panel.
- Added source surface maps for UI, layout, interaction, docs, dashboard, and brand coverage across `/ui`, `/database`, and `/backend`.
- Added visual audits for `/ui`, `/database`, and `/backend` so route sections, assets, responsive behavior, accessibility notes, and honest launch state are source-guarded.
- Added route discovery for `/ui`, `/database`, and `/backend` so DX-WWW, DX CLI, and Zed can index the conversion proof from one source-owned manifest.
- Added Studio preview selector markers for `/ui`, `/database`, and `/backend` so route DOM selections can open the matching source-owned page, surface map, visual audit, manifest, and receipt.
- Added a no-runtime acceptance checklist for `/ui`, `/database`, and `/backend` so Studio/Zed can see the exact evidence still required before rendered-proof promotion.
- Added the rendered-proof evidence receipt schema so approved runtime lanes can write renderer, asset, responsive, accessibility, interaction-boundary, and provenance evidence without changing source proof files.
- Added the source-only rendered-proof validator so missing `.dx/forge/runtime-evidence` receipts are reported honestly without failing source readiness.
- Added the blocked rendered-proof sample receipt so future runtime lanes can copy the not-approved shape without fake renderer snapshots or artifact hashes.
- Added the rendered-proof import plan and dry-run reporter so approved runtime lanes can map external receipt files into governed targets without this lane writing fake evidence.
- Added the rendered-proof completeness reviewer so operators can see the 99 / 100 blocker list without pretending source metadata is final rendered proof.
- Added the rendered-proof runtime approval request packet and source-only reporter so the route proof names the exact approved capture scope before any runtime server, build, credentialed backend, or receipt write.
- Added the rendered-proof evidence authoring guide and source-only reporter so runtime lanes know which real fields to collect without fake screenshots, placeholder hashes, or source-lane receipt writes.
- Added conversion manifests for shadcn-ui, Supabase, and Convex backend.
- Added Forge receipts and copied upstream notice files.
- Added source-owned Forge primitives for cn, slot/as-child, theme provider, button, input, badge, card, table, tabs, dialog, dropdown, and sidebar recipes.
- Added honest launch shims for heavier runtime dependencies instead of pretending backend, auth, realtime, hydration, or Radix state-machine behavior is implemented.
- Added a lightweight guard that checks routes, provenance, copied assets, unsupported runtime notes, and status documentation.

## Partial

- Replace static route structure with DX-WWW runtime widgets once the launch framework has the final component hydration path.
- Promote selected styles into the main DX-WWW visual system after screenshot verification is allowed.
- Capture final browser screenshots for the expanded `/` DX landing route and attach them to the rendered-proof evidence packet before public launch.
- Collect the route-level renderer snapshots named by the acceptance checklist, run the runtime approval request and evidence authoring reporters with an approval reference, run the rendered-proof import reporter with that approval reference, pass the rendered-proof completeness reviewer, and feed the imported receipts through the rendered-proof validator once runtime preview is approved.

## Blocked

- No npm install, npm build, cargo check, cargo clippy, cargo test, or long local server was allowed for this worker.
- Supabase hosted project operations require credentials and an explicit runtime integration step.
- Convex runtime execution and deployment mutations require a license-sensitive review and a heavier integration plan.
