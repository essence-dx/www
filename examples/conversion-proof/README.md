# DX-WWW Website Conversion Proof

Status: 99 / 100

This package is a source-backed DX-WWW conversion proof. It uses local inspiration mirrors from `G:\WWW\inspirations` as source material, preserves provenance and license notices, and rebuilds the launch routes as DX-WWW `.html` pages instead of importing Next.js, TanStack Router, or React Router routing.

## Routes

| Route | Source | State | Notes |
| --- | --- | --- | --- |
| `/` | DX landing | Real source recreation, runtime pending | Recreates the correct DX website launch landing as a DX-WWW-owned `.html` route with hero, OS/platform downloads, token/speed/performance claims, Forge, Traffic Security, Check, media, tools, pricing, benchmarks, testimonials, comparison, and waitlist sections. |
| `/ui` | shadcn-ui | Real source conversion, partial runtime | Converts registry, gallery, docs, style, and dashboard block structure into a DX-WWW-owned UI route. Live React previews, package installs, and registry generation stay unsupported in this proof. |
| `/database` | Supabase | Real source conversion, partial runtime | Converts Studio database editor, auth users, storage files, and query model surfaces into a DX-WWW-owned database route. Hosted project credentials, Postgres mutations, auth writes, and storage API calls stay unsupported. |
| `/backend` | Convex backend | Real source conversion, partial and blocked runtime | Converts dashboard functions, realtime, data, logs, files, schedule, and runtime topology into a DX-WWW-owned backend route. Runtime execution, deployment writes, realtime sockets, and FSL-sensitive server reuse stay blocked. |

## Provenance

- shadcn-ui source: `https://github.com/shadcn-ui/ui.git` at `36139f6200d9c2684ef7695fce5f3d9787378e26`, MIT.
- Supabase source: `https://github.com/supabase/supabase.git` at `fdeaaec098e6151a199ff84b55b030547afe7eee`, Apache-2.0.
- Convex backend source: `https://github.com/get-convex/convex-backend.git` at `e5026b0d466058eea21084f9561d7ad974e925fc`, FSL-1.1-Apache-2.0.
- DX landing source: project-local `G:\Dx\website` startpage and thumbnail assets.

The full manifests live in `forge/conversion-manifests`. Forge receipts live in `.dx/forge/receipts`. Copied upstream license notices live in `notices`.

## Forge Primitives And Shims

- Source-owned Forge primitives live in `forge/primitives` and cover cn/class merge, slot/as-child, theme provider, button, input, badge, card, table, tabs, dialog, dropdown, and sidebar recipes.
- The reviewed package-conversion proof is covered by `forge_import_www_conversion_packages_materialize_reviewed_adapters`: reviewed slices for `three`, `xlsx`, `pptxgenjs`, `jszip`, `fabric`, `konva`, `fflate`, and `hyperformula` materialize through `dx forge import` as accepted Forge import aliases and source paths with no `node_modules`; `@ffmpeg/ffmpeg` is intentionally recorded as a bridge/native-runtime refusal instead of unsafe source materialization.
- Source-owned DX-WWW components live in `components` and keep the shared route header, source surface table, and runtime boundary panel reusable across `/ui`, `/database`, and `/backend`.
- The source-owned DX landing lives in the converted route artifact with route styling in `styles/dx-landing.css` and copied launch thumbnails under `public/thumbnails`.
- Source surface maps live in `forge/source-surfaces` and record the UI, layout, interaction, docs, dashboard, and brand surfaces converted for each route.
- Visual audits live in `forge/visual-audits` and record route sections, responsive constraints, accessibility notes, assets, and honest launch state for `/ui`, `/database`, and `/backend`.
- Route discovery lives in `forge/route-discovery/conversion-routes.json` and gives DX-WWW, DX CLI, and Zed one canonical index for pages, manifests, receipts, source maps, visual audits, assets, notices, runtime state, and Studio preview selectors.
- The no-runtime acceptance checklist lives in `forge/acceptance/no-runtime-route-acceptance.json` and records the evidence needed before `/ui`, `/database`, or `/backend` can move from source proof to rendered proof.
- The rendered-proof evidence schema lives in `forge/acceptance/rendered-proof-evidence.schema.json` and defines the future `.dx/forge/runtime-evidence` receipt shape without claiming any runtime capture now.
- The rendered-proof validator lives in `forge/acceptance/validate-rendered-proof-evidence.ts` and reports missing runtime evidence without failing source readiness or writing files.
- The blocked rendered-proof sample lives in `forge/acceptance/fixtures/blocked-rendered-proof.sample.json` and shows the not-approved receipt shape without inventing screenshots, accessibility scans, or asset hashes.
- The rendered-proof import plan lives in `forge/acceptance/rendered-proof-import-plan.json`, with `forge/acceptance/prepare-rendered-proof-import.ts` providing a dry-run, approval-gated import report for future real receipts.
- The rendered-proof completeness reviewer lives in `forge/acceptance/review-rendered-proof-completeness.ts` and keeps the route proof at 99 / 100 until approved runtime receipts exist.
- The rendered-proof runtime approval request lives in `forge/acceptance/rendered-proof-runtime-approval-request.json`, with `forge/acceptance/request-rendered-proof-runtime-approval.ts` reporting exactly what runtime capture would need approval before any server, build, credentialed backend, or evidence write can happen.
- The rendered-proof evidence authoring guide lives in `forge/acceptance/rendered-proof-evidence-authoring-guide.json`, with `forge/acceptance/summarize-rendered-proof-evidence-requirements.ts` summarizing the real artifact fields to collect while keeping every capture value null until approval.
- Heavy runtime dependencies live as honest launch shims in `forge/shims`: Next routing, React hydration, TanStack Query, Supabase auth/storage, Convex realtime/functions, and Radix state machines.
- The shims keep the UI running as source proof and mark missing-runtime boundaries clearly. They do not report fake backend, auth, realtime, or focus-management success.

## Verification

Allowed verification for this slice:

- `dx run --test .\benchmarks\dx-www-conversion-proof.test.ts`
- `node .\examples\conversion-proof\forge\acceptance\validate-rendered-proof-evidence.ts --json`
- `node .\examples\conversion-proof\forge\acceptance\prepare-rendered-proof-import.ts --json`
- `node .\examples\conversion-proof\forge\acceptance\review-rendered-proof-completeness.ts --json`
- `node .\examples\conversion-proof\forge\acceptance\request-rendered-proof-runtime-approval.ts --json`
- `node .\examples\conversion-proof\forge\acceptance\summarize-rendered-proof-evidence-requirements.ts --json`
- `git diff --check -- .\benchmarks\dx-www-conversion-proof.test.ts .\examples\conversion-proof`
- `rg -n "^(<<<<<<<|=======|>>>>>>>)" .\benchmarks\dx-www-conversion-proof.test.ts .\examples\conversion-proof`

Heavy builds, npm installs, cargo checks, and local servers are intentionally out of scope for this worker.
