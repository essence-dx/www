# Changelog

## 2026-05-21

- Added a source-owned `favicon.svg`, linked it in the static Vercel landing export, regenerated `.dx/vercel-landing`, and deployed the updated landing-only static bundle to the existing `vercel-landing` Vercel project.
- Polished the DX-WWW landing for public launch: removed the browser default white border through a page-level reset, restored natural scrolling, added Vercel-like light/dark/system theme controls, animated capability icons, a theme proof section, and a Midday-style footer with "Enhanced Development Experience" and "I use Dx BTW".
- Moved the theme switcher to the footer, switched the public landing to JetBrains Mono, replaced the hero mascot with a responsive inline SVG circular DX face, and tightened root/body scroll behavior after in-app browser review.
- Added `dx-icon` platform icons to the download cards, enlarged the DX face eyes with a smaller smile, and made the DX landing itself the scroll container so localhost/Web Preview scrolling does not depend on page-level body scrolling.
- Made dark the default landing theme, added a visible custom DX scroll rail tied to the landing scroll container, moved face SVG colors to theme variables, enlarged the mascot circle, fixed CTA contrast with dedicated button tokens, added above-the-fold hero imagery, and wired scroll-reveal plus hover animation states.
- Fixed local landing image rendering by using DX-WWW's `/public` static asset route, kept the Vercel export bridge rewriting those URLs for static output, and raised CTA button color specificity so footer CTA text remains readable.
- Added a DX Icons-labeled icon treatment to the primary download CTA, rebuilt the custom scrollbar as a top-layer draggable/clickable control with theme-token translucent styling, and removed colorful editor mockup glows in favor of the Vercel-like DX theme.
- Hid the default generated DX-WWW wrapper header on the landing page, simplified the DX face to one Vercel-token mascot circle with a smaller smile curve, and widened the custom scrollbar thumb while keeping its rail/drag math theme-token based.
- Replaced the top download-card placeholders with DX-owned `<dx-icon name="pack:name" />` source syntax, removed the temporary pricing section, changed benchmark/proof bars to Vercel-theme token gradients, and refocused the visible landing copy on DX-WWW, dx-style, dx-forge, dx-check, agents, browser automation, and local AI instead of presenting the code editor as the current product story.
- Reworked the landing header and hero: the header is now fixed and compact with logo-adjacent links, the long header text/button and face mascot are removed for now, the hero starts with a large 8K YouTube nature video carousel, reveal motion uses a softer spring curve, and the footer has a more professional clipped mega wordmark.
- Tuned the landing for Lighthouse-style performance by replacing the above-the-fold YouTube iframes with local preview cards, removing the blocking Google Fonts import, adding stable lazy/async thumbnail dimensions, and keeping lower sections paint-cheap with content visibility.
- Expanded the DX-WWW launch landing with the latest DX product story: Zed/GPUI editor platform, full-width AI workspace, screen dock, editor-connected Web Preview, DX-WWW Forge Studio, source-owned launch packages, local model runtime, provider onboarding, RLM/serializer token receipts, dx-agents, n8n automation, browser visual checks, and Forge safety workflows.
- Added `tools/export-vercel-landing.ts` so the source-owned DX-WWW `.html` landing can be exported into a static `.dx/vercel-landing` bundle for Vercel without adding a template-local `node_modules` workflow.
- Recreated the correct DX launch landing as a DX-WWW source-owned route at `pages/index.html`, with dedicated CSS in `styles/dx-landing.css`, copied thumbnail assets, and preserved sections for hero, platform downloads, token/speed/performance proof, Forge, Traffic Security, Check, media, built-in tools, pricing, benchmarks, testimonials, comparison, and waitlist.
- Added DX landing conversion manifest, Forge receipt, source-surface map, visual audit, and route-discovery entry for `/`.
- Added 99 / 100 DX-WWW conversion proof package for shadcn-ui, Supabase, and Convex backend.
- Added real DX-WWW route targets: `/ui`, `/database`, and `/backend`.
- Added shared DX-WWW `.tsx` route components for the conversion proof shell, source tables, and runtime boundary panels.
- Added source surface maps for each converted route so UI, layout, interaction, docs, dashboard, and brand provenance is explicit.
- Added visual audits for `/ui`, `/database`, and `/backend` so Web Preview route quality, assets, responsive constraints, and launch state are discoverable.
- Added route discovery for `/ui`, `/database`, and `/backend` so DX-WWW, DX CLI, and Zed can consume one canonical source-owned route index.
- Added Studio preview selector metadata for `/ui`, `/database`, and `/backend` so DOM selections map back to source-owned pages, surface maps, visual audits, manifests, and receipts.
- Added a no-runtime acceptance checklist for `/ui`, `/database`, and `/backend` so Studio/Zed can track the exact rendered-proof evidence still needed without pretending screenshots or live runtime checks exist.
- Added a rendered-proof evidence receipt schema so approved runtime-verification lanes can write future `.dx/forge/runtime-evidence` receipts without mutating source proof files.
- Added a source-only rendered-proof validator that reports `missing_runtime_evidence` while keeping source readiness green.
- Added a blocked rendered-proof sample receipt that records the not-approved runtime shape without fake screenshots, scans, or artifact hashes.
- Added a rendered-proof import plan and dry-run reporter that require explicit approval before external receipts are evaluated and never write fake runtime evidence from this lane.
- Added a rendered-proof completeness reviewer that reports the final 99 / 100 runtime blockers without writing receipts or claiming screenshots exist.
- Added a rendered-proof runtime approval request packet and source-only reporter that keep the source proof at 99 / 100 until approved runtime capture and real receipts exist.
- Added a rendered-proof evidence authoring guide and source-only reporter that name the real artifact fields to collect while forbidding fake screenshots, placeholder hashes, and source-lane runtime receipts.
- Added conversion manifests, Forge receipts, copied asset references, and notice copies for upstream provenance.
- Added source-owned Forge primitives and honest launch shim adapters so the proof does not depend on node_modules or fake full runtime behavior.
- Added lightweight verification coverage for real, partial, and blocked route state.
