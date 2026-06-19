# DX-WWW Conversion Proof Notes

Status: 99 / 100

## Purpose

This example proves that DX-WWW can absorb large modern website products into a lighter Forge-owned route framework while keeping upstream source provenance auditable.

## Canonical Routes

- `/ui`: shadcn-ui registry, gallery, docs, style, and dashboard block structure converted to `pages/ui.html`.
- `/`: correct DX launch landing recreated from `G:\Dx\website` into `pages/index.html` with platform downloads, token/speed/performance sections, Forge, Traffic Security, Check, media, tools, pricing, benchmarks, testimonials, comparison, and waitlist.
- `/database`: Supabase Studio database, auth, storage, and admin surfaces converted to `pages/database.html`.
- `/backend`: Convex dashboard functions, realtime, data, logs, files, schedules, and runtime topology converted to `pages/backend.html`.

## Route State

- Real: source mirrors, source file lists, license notices, copied assets, Forge receipts, DX landing thumbnails, and DX-WWW `.html` routes exist.
- Partial: interactive React application behavior is represented as route structure, tables, navigation, and operational surfaces rather than executed as React runtime code.
- Blocked: hosted credentials, database mutations, realtime sockets, deployment writes, package-manager installs, cargo checks, npm builds, and local servers were not run by design.

## Source Boundary

Upstream mirrors under `G:\WWW\inspirations` remain read-only. The converted launch proof lives in this directory and can be promoted into the main DX-WWW route set after a heavier verification pass is explicitly allowed.

## Forge-Owned Package Layer

- `forge/primitives` owns the launch-useful UI pieces that would otherwise require Radix, shadcn runtime helpers, next-themes, clsx, or tailwind-merge.
- `components` owns the DX-WWW `.tsx` route building blocks shared by `/ui`, `/database`, and `/backend`.
- `forge/source-surfaces` records the UI, layout, interaction, docs, dashboard, and brand source surfaces for every converted route.
- `forge/visual-audits` records the route sections, visual assets, responsive constraints, accessibility notes, and real/partial/blocked launch state for every converted route.
- `forge/route-discovery/conversion-routes.json` is the canonical source-owned index for DX-WWW, DX CLI, Zed route discovery, and Studio preview selector handoff.
- `forge/acceptance/no-runtime-route-acceptance.json` is the no-runtime acceptance checklist for rendered-proof promotion. It names required snapshots, asset checks, responsive review, accessibility review, interaction-boundary review, and provenance review without claiming any runtime evidence.
- `forge/acceptance/rendered-proof-evidence.schema.json` defines the rendered-proof evidence receipt runtime lanes can write under `.dx/forge/runtime-evidence` after approval, without mutating route source proof.
- `forge/acceptance/validate-rendered-proof-evidence.ts` is the source-only rendered-proof validator. It exits successfully with `missing_runtime_evidence` while receipts are absent, so source readiness does not pretend runtime proof exists.
- `forge/acceptance/fixtures/blocked-rendered-proof.sample.json` is the blocked rendered-proof sample for runtime lanes. It keeps approval status, missing artifacts, checks, and provenance explicit without adding fake screenshots or hashes.
- `forge/acceptance/rendered-proof-import-plan.json` and `forge/acceptance/prepare-rendered-proof-import.ts` define the rendered-proof import handoff. The reporter is dry-run by default, requires an approval reference before evaluating external receipts, and does not write runtime evidence from this lane.
- `forge/acceptance/review-rendered-proof-completeness.ts` is the rendered-proof completeness reviewer. It keeps the source proof honest at 99 / 100 until approved runtime receipts exist.
- `forge/acceptance/rendered-proof-runtime-approval-request.json` and `forge/acceptance/request-rendered-proof-runtime-approval.ts` publish the runtime approval request. They keep rendered-proof capture blocked until an operator approves the exact runtime scope and still write no evidence from this source lane.
- `forge/acceptance/rendered-proof-evidence-authoring-guide.json` and `forge/acceptance/summarize-rendered-proof-evidence-requirements.ts` publish the evidence authoring guide. They map the schema to route-specific capture fields, keep capture values null, and explicitly reject fake screenshots, placeholder hashes, or source-lane runtime receipts.
- `forge/shims` records the heavier runtime gaps as missing-runtime, shim, or blocked states. These adapters intentionally return skipped non-success states until real DX-WWW runtime code exists.
