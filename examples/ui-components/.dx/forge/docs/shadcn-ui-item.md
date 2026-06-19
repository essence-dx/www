# DX Forge Package: `shadcn/ui/item`

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

This package is source-owned. The files below are editable project files, not opaque `node_modules` content. Forge tracks their hashes, treats local edits as reviewable yellow traffic, blocks red/security-sensitive traffic, and updates them through `dx update shadcn/ui/item`.

## Package Metadata Review

- Provenance note: DX inspected the local shadcn-ui v4 registry entry plus Radix Slot and Separator primitives for the official UI Components Item surface; this is curated launch metadata, not SLSA or live upstream provenance.
- Advisory note: Curated DX Forge advisory fixture records no known advisory findings for this shadcn item launch slice, but it is not a live advisory feed and does not audit application list semantics, actions, or row-level authorization.
- License review note: License is recorded from the curated package declaration only; no formal DX legal review is claimed.

## Materialized Files

| File | Logical Source | Bytes | Hash |
| --- | --- | ---: | --- |
| `lib/utils.ts` | `js/lib/utils.ts` | `119` | `597629c1fe30ddda8d02ad7f93c4bd8f50e041e2166da29ff03a1b96712c06a1` |
| `components/ui/slot.tsx` | `js/ui/slot.tsx` | `642` | `ba2a601a0e6ff61980dca81e9e81cf87c8be19d05cd09af3adefbbc7658786ff` |
| `components/ui/separator.tsx` | `js/ui/separator.tsx` | `809` | `fce33cd494a500bdbbbd36398c0be4623f3c84900b41728c7f6c3a6ac9f8e849` |
| `components/ui/item.tsx` | `js/ui/item.tsx` | `4591` | `55fc3287b32031458aa88913316156390cd0d3b83e22bb1d7b6196752aa3bc8b` |
| `components/ui/README.md` | `js/ui/README.md` | `748` | `67f31042a73433879648ba5d9c5165c7f357fb9c803388e34c0d08716654a8fd` |

## Forge Policy

| Traffic | Policy | Decision |
| --- | --- | --- |
| `green` | `green-only-update` | Forge update write only applies green changes and never runs package scripts. |
| `green` | `update-unchanged` | lib/utils.ts already matches the latest package. |
| `green` | `update-unchanged` | components/ui/slot.tsx already matches the latest package. |
| `green` | `update-unchanged` | components/ui/separator.tsx already matches the latest package. |
| `green` | `update-update` | components/ui/item.tsx would be updated from the latest package. |
| `green` | `update-update` | components/ui/README.md would be updated from the latest package. |
| `green` | `explicit-yellow-review` | Reviewer `Codex` accepted yellow local edits for `shadcn/ui/item` variant `default`: Reviewed WWW-native no-npm adaptation for the UI Components source slice |

## Update Decisions

| Traffic | Change | Path | Decision |
| --- | --- | --- | --- |
| `green` | `Unchanged` | `lib/utils.ts` | `Accepted` |
| `green` | `Unchanged` | `components/ui/slot.tsx` | `Accepted` |
| `green` | `Unchanged` | `components/ui/separator.tsx` | `Accepted` |
| `green` | `Update` | `components/ui/item.tsx` | `Accepted` |
| `green` | `Update` | `components/ui/README.md` | `Accepted` |
