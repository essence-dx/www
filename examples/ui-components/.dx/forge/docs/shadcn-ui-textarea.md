# DX Forge Package: `shadcn/ui/textarea`

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

This package is source-owned. The files below are editable project files, not opaque `node_modules` content. Forge tracks their hashes, treats local edits as reviewable yellow traffic, blocks red/security-sensitive traffic, and updates them through `dx update shadcn/ui/textarea`.

## Package Metadata Review

- Provenance note: DX inspected the local shadcn-ui v4 registry entry for the official UI Components Textarea surface; this is curated launch metadata, not SLSA or live upstream provenance.
- Advisory note: Curated DX Forge advisory fixture records no known advisory findings for this shadcn textarea package, but it is not a live advisory feed.
- License review note: License is recorded from the curated package declaration only; no formal DX legal review is claimed.

## Materialized Files

| File | Logical Source | Bytes | Hash |
| --- | --- | ---: | --- |
| `lib/utils.ts` | `js/lib/utils.ts` | `119` | `597629c1fe30ddda8d02ad7f93c4bd8f50e041e2166da29ff03a1b96712c06a1` |
| `components/ui/textarea.tsx` | `js/ui/textarea.tsx` | `465` | `40a9eb1736f22b38624335951673335607185e69549eab929677420d0023e836` |

## Forge Policy

| Traffic | Policy | Decision |
| --- | --- | --- |
| `green` | `green-only-update` | Forge update write only applies green changes and never runs package scripts. |
| `green` | `update-unchanged` | lib/utils.ts already matches the latest package. |
| `green` | `update-update` | components/ui/textarea.tsx would be updated from the latest package. |
| `green` | `explicit-yellow-review` | Reviewer `Codex` accepted yellow local edits for `shadcn/ui/textarea` variant `default`: Reviewed WWW-native no-npm adaptation for the UI Components source slice |

## Update Decisions

| Traffic | Change | Path | Decision |
| --- | --- | --- | --- |
| `green` | `Unchanged` | `lib/utils.ts` | `Accepted` |
| `green` | `Update` | `components/ui/textarea.tsx` | `Accepted` |
