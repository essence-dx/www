# Forge Source-Owned Package Fixture Review

Generated: 2026-05-24T13:25:37.359Z
Project: `G:\Dx\www\.dx\adoption-package-review`
Score: `100` / `100`
Passed: `true`
No node_modules: `true`

## Review Gates

| Gate | Passed | Present | Expected |
| --- | --- | ---: | ---: |
| docs | `true` | 30 | 30 |
| receipts | `true` | 30 | 30 |
| rollback | `true` | 1 | 1 |
| advisory placeholders | `true` | 29 | 29 |
| local edit yellow review | `true` | 2 | 2 |
| verify all current Forge review packages | `true` | 29 | 29 |
| adoption report | `true` | 1 | 1 |
| no node_modules | `true` | 1 | 1 |
| prepare adoption app | `true` | 1 | 1 |

## Packages

| Package | Source | Docs | Receipts | Advisory | Live Feed | Rollback Receipt |
| --- | --- | --- | ---: | --- | --- | --- |
| `shadcn/ui/button` | `curated-registry` | `true` | 2 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `shadcn/ui/badge` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `shadcn/ui/card` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `shadcn/ui/label` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `shadcn/ui/separator` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `shadcn/ui/field` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `shadcn/ui/item` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `shadcn/ui/input` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `shadcn/ui/textarea` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `dx/icon/search` | `curated-registry` | `true` | 2 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `auth/better-auth` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `animation/motion` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `i18n/next-intl` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `tanstack/query` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `validation/zod` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `forms/react-hook-form` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `payments/stripe-js` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `automations/n8n` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `state/zustand` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `ai/vercel-ai` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `api/trpc` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `content/fumadocs-next` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `content/react-markdown` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `supabase/client` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `db/drizzle-sqlite` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `instantdb/react` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `wasm/bindgen` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `3d/launch-scene` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `migration/static-site` | `curated-registry` | `true` | 1 | `curated-fixture` / `dx-forge-curated-advisory-fixture` | `false` | `n/a` |
| `dx-www/vertical/forge` | `local` | `true` | 1 | `missing` / `none` | `false` | `n/a` |

## Local-Edit And Rollback Rehearsal

- Score: `100` / `100`
- Passed: `true`
- No node_modules: `true`

| Scenario | Passed | Traffic | Evidence |
| --- | --- | --- | --- |
| green update | `true` | `green` | receipt 20260524T132604637098300Z-shadcn-ui-button.json |
| yellow default block | `true` | `yellow` | DX-WWW error: Internal DX-WWW error Forge update for `shadcn/ui/button` variant `default`  |
| yellow review accept | `true` | `yellow` | receipt 20260524T132608725706000Z-shadcn-ui-button.json |
| red quarantine | `true` | `red` | DX-WWW error: Internal DX-WWW error Forge update for `shadcn/ui/button` variant `default`  |
| rollback coverage | `true` | `n/a` | receipt 20260524T132610836627100Z-shadcn-ui-button.json |

## Findings

- none

## Honest Scope

- This joins existing local Forge adoption evidence into one source-owned package review artifact.
- It proves curated package docs, receipts, advisory placeholders, update traffic, local-edit yellow review, rollback, and no node_modules for the example app path.
- It still does not claim arbitrary npm ingestion, live advisory coverage, or production customer adoption.
