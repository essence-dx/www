# DX Forge Package: `shadcn/ui/button`

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

This package is source-owned. The files below are editable project files, not opaque `node_modules` content. Forge tracks their hashes, treats local edits as reviewable yellow traffic, blocks red/security-sensitive traffic, and updates them through `dx update shadcn/ui/button`.

## Package Metadata Review

- Provenance note: DX inspected the local shadcn-ui v4 registry entry and Radix Slot primitive for the official UI Components Button surface; this is curated launch metadata, not SLSA or live upstream provenance.
- Advisory note: Curated DX Forge advisory fixture records no known advisory findings for this shadcn-style button package, but it is not a live advisory feed.
- License review note: License is recorded from the curated package declaration only; no formal DX legal review is claimed.

## Materialized Files

| File | Logical Source | Bytes | Hash |
| --- | --- | ---: | --- |
| `lib/utils.ts` | `js/lib/utils.ts` | `119` | `597629c1fe30ddda8d02ad7f93c4bd8f50e041e2166da29ff03a1b96712c06a1` |
| `components/ui/slot.tsx` | `js/ui/slot.tsx` | `642` | `ba2a601a0e6ff61980dca81e9e81cf87c8be19d05cd09af3adefbbc7658786ff` |
| `components/ui/button.tsx` | `js/ui/button.tsx` | `1976` | `dd3775a5f32b9a2e3f05cd9955042b68686b514ef88c814c64e47f79200f9afe` |
| `components/ui/README.md` | `js/ui/README.md` | `183` | `6a5633e2163aff242dff4dfbe3f2feb8dd906c4ac848464662de7e22a13e02ba` |

## Forge Policy

| Traffic | Policy | Decision |
| --- | --- | --- |
| `green` | `green-only-update` | Forge update write only applies green changes and never runs package scripts. |
| `green` | `update-update` | lib/utils.ts would be updated from the latest package. |
| `green` | `update-update` | components/ui/slot.tsx would be updated from the latest package. |
| `green` | `update-update` | components/ui/button.tsx would be updated from the latest package. |
| `green` | `update-unchanged` | components/ui/README.md already matches the latest package. |
| `green` | `explicit-yellow-review` | Reviewer `Codex` accepted yellow local edits for `shadcn/ui/button` variant `default`: Reviewed WWW-native no-npm adaptation for the UI Components source slice |

## Update Decisions

| Traffic | Change | Path | Decision |
| --- | --- | --- | --- |
| `green` | `Update` | `lib/utils.ts` | `Accepted` |
| `green` | `Update` | `components/ui/slot.tsx` | `Accepted` |
| `green` | `Update` | `components/ui/button.tsx` | `Accepted` |
| `green` | `Unchanged` | `components/ui/README.md` | `Accepted` |
