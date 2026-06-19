# DX Forge Package: `shadcn/ui/input`

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

This package is source-owned. The files below are editable project files, not opaque `node_modules` content. Forge tracks their hashes, treats local edits as reviewable yellow traffic, blocks red/security-sensitive traffic, and updates them through `dx update shadcn/ui/input`.

## Package Metadata Review

- Provenance note: DX inspected the local shadcn-ui v4 registry entry for the official UI Components Input surface; this is curated launch metadata, not SLSA or live upstream provenance.
- Advisory note: Curated DX Forge advisory fixture records no known advisory findings for this shadcn input package, but it is not a live advisory feed.
- License review note: License is recorded from the curated package declaration only; no formal DX legal review is claimed.

## Materialized Files

| File | Logical Source | Bytes | Hash |
| --- | --- | ---: | --- |
| `lib/utils.ts` | `js/lib/utils.ts` | `119` | `597629c1fe30ddda8d02ad7f93c4bd8f50e041e2166da29ff03a1b96712c06a1` |
| `components/ui/input.tsx` | `js/ui/input.tsx` | `545` | `4e2777a9c0c7f17fa617c426b874feca17f83104de1b9223fcdeffe6f8090c83` |

## Forge Policy

| Traffic | Policy | Decision |
| --- | --- | --- |
| `green` | `green-only-update` | Forge update write only applies green changes and never runs package scripts. |
| `green` | `update-unchanged` | lib/utils.ts already matches the latest package. |
| `green` | `update-update` | components/ui/input.tsx would be updated from the latest package. |
| `green` | `explicit-yellow-review` | Reviewer `Codex` accepted yellow local edits for `shadcn/ui/input` variant `default`: Reviewed WWW-native no-npm adaptation for the UI Components source slice |

## Update Decisions

| Traffic | Change | Path | Decision |
| --- | --- | --- | --- |
| `green` | `Unchanged` | `lib/utils.ts` | `Accepted` |
| `green` | `Update` | `components/ui/input.tsx` | `Accepted` |
