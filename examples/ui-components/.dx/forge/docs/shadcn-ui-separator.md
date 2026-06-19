# DX Forge Package: `shadcn/ui/separator`

- Variant: `default`
- Version: `0.1.0`
- Upstream: `shadcn-ui`
- Generator: `dx-forge/shadcn-ui`
- License: `MIT OR Apache-2.0`
- Provenance: `dx-forge-curated-registry` (verified: `no`)
- Advisory coverage: `curated-fixture` via `dx-forge-curated-advisory-fixture` (live: `no`, findings: `0`)
- License review: declared `MIT OR Apache-2.0` (reviewed: `no`)
- Last action: `UpdateWrite`
- Risk score: `100`

This package is source-owned. The files below are editable project files, not opaque `node_modules` content. Forge tracks their hashes, treats local edits as reviewable yellow traffic, blocks red/security-sensitive traffic, and updates them through `dx update shadcn/ui/separator`.

## Package Metadata Review

- Provenance note: DX inspected the local shadcn-ui v4 registry entry and Radix Separator primitive for the official UI Components Separator surface; this is curated launch metadata, not SLSA or live upstream provenance.
- Advisory note: Curated DX Forge advisory fixture records no known advisory findings for this shadcn separator launch slice, but it is not a live advisory feed and does not audit application information hierarchy or decorative-versus-semantic divider policy.
- License review note: License is recorded from the curated package declaration only; no formal DX legal review is claimed.

## Materialized Files

| File | Logical Source | Bytes | Hash |
| --- | --- | ---: | --- |
| `lib/utils.ts` | `js/lib/utils.ts` | `119` | `597629c1fe30ddda8d02ad7f93c4bd8f50e041e2166da29ff03a1b96712c06a1` |
| `components/ui/separator.tsx` | `js/ui/separator.tsx` | `809` | `fce33cd494a500bdbbbd36398c0be4623f3c84900b41728c7f6c3a6ac9f8e849` |
| `components/ui/README.md` | `js/ui/README.md` | `481` | `d817dd403cb0e4016814ea81dc44a8ce41803692ca58f8bdb7c5151082c34a15` |

## Forge Policy

| Traffic | Policy | Decision |
| --- | --- | --- |
| `green` | `green-only-update` | Forge update write only applies green changes and never runs package scripts. |
| `green` | `update-unchanged` | lib/utils.ts already matches the latest package. |
| `green` | `update-update` | components/ui/separator.tsx would be updated from the latest package. |
| `green` | `update-update` | components/ui/README.md would be updated from the latest package. |
| `green` | `explicit-yellow-review` | Reviewer `Codex` accepted yellow local edits for `shadcn/ui/separator` variant `default`: Reviewed WWW-native no-npm adaptation for the UI Components source slice |

## Update Decisions

| Traffic | Change | Path | Decision |
| --- | --- | --- | --- |
| `green` | `Unchanged` | `lib/utils.ts` | `Accepted` |
| `green` | `Update` | `components/ui/separator.tsx` | `Accepted` |
| `green` | `Update` | `components/ui/README.md` | `Accepted` |
